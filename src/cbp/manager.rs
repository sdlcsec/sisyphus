use std::error::Error;
use std::sync::Arc;
use tokio::sync::mpsc;
use serde_json::{json, Value};
use std::collections::HashMap;
use crate::models::events::{CDEvent, CDEventType, EventSubject, SubjectType};
use crate::models::policy::Policy;
use crate::models::attestation::Attestation;
use crate::storage::attestation_storage::AttestationStorage;
use crate::storage::policy_repository::PolicyRepository;
use crate::verification::PolicyVerifier;

pub struct CBPManager<P, A>
where
    P: PolicyRepository + 'static,
    A: AttestationStorage + 'static,
{
    policy_verifier: Arc<dyn PolicyVerifier>,
    policy_repo: Arc<P>,
    attestation_storage: Arc<A>,
    event_receiver: mpsc::Receiver<CDEvent>,
    pending_attestations: HashMap<String, Vec<String>>, // subject -> Vec<attestation_uri>
}

impl<P, A> CBPManager<P, A>
where
    P: PolicyRepository + 'static,
    A: AttestationStorage + 'static,
{
    pub fn new(
        policy_verifier: Arc<dyn PolicyVerifier>,
        policy_repo: Arc<P>,
        attestation_storage: Arc<A>,
        event_receiver: mpsc::Receiver<CDEvent>,
    ) -> Self {
        Self {
            policy_verifier,
            policy_repo,
            attestation_storage,
            event_receiver,
            pending_attestations: HashMap::new(),
        }
    }

    pub async fn run(&mut self) {
        while let Some(event) = self.event_receiver.recv().await {
            if let Err(e) = self.handle_event(event).await {
                eprintln!("Error handling event: {}", e);
            }
        }
    }

    async fn handle_event(&mut self, event: CDEvent) -> Result<(), Box<dyn Error + Send + Sync>> {
        match event.event_type {
            CDEventType::AttestationCreated { attestation_id, attestation_uri } => {
                self.handle_attestation_created(attestation_id, attestation_uri).await?;
            }
            _ => {
                // TODO: Handle other event types
            }
        }
        Ok(())
    }

    async fn handle_attestation_created(&mut self, attestation_id: String, attestation_uri: String) -> Result<(), Box<dyn Error + Send + Sync>> {
        let attestation = self.attestation_storage.get_attestation(&attestation_uri).await?;
        let subject = self.get_subject_from_attestation(&attestation)?;

        self.pending_attestations
            .entry(subject.clone())
            .or_default()
            .push(attestation_uri.clone());

        // Check if we have all required attestations for this subject
        if self.is_subject_complete(&subject).await? {
            self.generate_summary_attestation(&subject).await?;
            self.pending_attestations.remove(&subject);
        }

        Ok(())
    }

    async fn is_subject_complete(&self, subject: &str) -> Result<bool, Box<dyn Error + Send + Sync>> {
        // This method should check if all required attestations for the subject are present
        // For now, we'll assume that if we have at least one attestation, it's complete
        Ok(self.pending_attestations.get(subject).map_or(false, |atts| !atts.is_empty()))
    }

    async fn generate_summary_attestation(&self, subject: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
        let attestation_uris = self.pending_attestations.get(subject).ok_or("No pending attestations found")?;
        let mut attributes = Vec::new();

        for uri in attestation_uris {
            let attestation = self.attestation_storage.get_attestation(uri).await?;
            let policies = self.get_relevant_policies(&attestation).await?;

            for policy in policies {
                let is_valid = self.policy_verifier.verify_attestation(&attestation, &policy).await?;
                let attribute = self.determine_attribute(&attestation, &policy, is_valid)?;
                let evidence = self.create_evidence(&attestation)?;

                attributes.push(json!({
                    "attribute": attribute,
                    "evidence": evidence,
                }));
            }
        }

        let summary_content = json!({
            "_type": "https://in-toto.io/Statement/v1",
            "subject": [
                {
                    "name": subject,
                    "digest": { "sha256": self.calculate_digest(subject) },
                }
            ],
            "predicateType": "https://in-toto.io/attestation/scai/attribute-report/v0.2",
            "predicate": {
                "attributes": attributes,
                "producer": {
                    "uri": "https://example.com/cbp/build",
                    "name": "CBP Build Attestor",
                    "digest": {
                        "sha256": self.calculate_digest("CBP Build Attestor"),
                    }
                }
            }
        });

        let summary_attestation = Attestation {
            id: uuid::Uuid::new_v4().to_string(),
            issuer: "CBPManager".to_string(),
            timestamp: chrono::Utc::now(),
            content: summary_content,
        };

        let summary_uri = self.attestation_storage.store_attestation(Arc::new(summary_attestation.clone())).await?;

        // Create and emit a new CDEvent for the summary attestation
        let summary_event = CDEvent::new(
            CDEventType::AttestationCreated {
                attestation_id: summary_attestation.id.clone(),
                attestation_uri: summary_uri,
            },
            EventSubject {
                id: summary_attestation.id.clone(),
                subject_type: SubjectType::Attestation,
            },
        );

        // Here you would emit the summary_event to your event system
        // For example: self.event_sender.send(summary_event).await?;

        Ok(())
    }

    fn get_subject_from_attestation(&self, attestation: &Attestation) -> Result<String, Box<dyn Error + Send + Sync>> {
        // Extract the subject from the attestation
        // This is a placeholder implementation; adjust according to your attestation structure
        attestation.content["subject"][0]["name"]
            .as_str()
            .map(String::from)
            .ok_or_else(|| "Unable to extract subject from attestation".into())
    }

    async fn get_relevant_policies(&self, attestation: &Attestation) -> Result<Vec<Arc<Policy>>, Box<dyn Error + Send + Sync>> {
        // This method should return all policies that apply to the given attestation
        // For now, we'll just return a single policy based on the PURL
        let purl = attestation.content["purl"].as_str().unwrap_or("");
        let policy = self.policy_repo.get_policy(purl, None).await?;
        Ok(vec![policy])
    }

    fn determine_attribute(&self, attestation: &Attestation, policy: &Policy, is_valid: bool) -> Result<String, Box<dyn Error + Send + Sync>> {
        // This method should determine the appropriate attribute based on the attestation, policy, and validation result
        // This is a placeholder implementation
        if is_valid {
            Ok(format!("VALID_{}", policy.purl.to_uppercase()))
        } else {
            Ok(format!("INVALID_{}", policy.purl.to_uppercase()))
        }
    }

    fn create_evidence(&self, attestation: &Attestation) -> Result<Value, Box<dyn Error + Send + Sync>> {
        let evidence = json!({
            "name": format!("{}.jsonl", attestation.id),
            "uri": format!("https://example.com/scai/{}.jsonl", attestation.id),
            "digest": {
                "sha256": self.calculate_digest(&attestation.id),
            },
            "mediaType": "application/x.dsse+json"
        });

        Ok(evidence)
    }

    fn calculate_digest(&self, input: &str) -> String {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::attestation_storage::InMemoryAttestationStorage;
    use crate::storage::policy_repository::InMemoryPolicyRepository;
    use crate::models::policy::PolicyRules;

    struct MockPolicyVerifier;

    #[async_trait::async_trait]
    impl PolicyVerifier for MockPolicyVerifier {
        async fn verify_attestation(&self, _attestation: &Attestation, _policy: &Policy) -> Result<bool, Box<dyn Error + Send + Sync>> {
            Ok(true)
        }
    }

    #[tokio::test]
    async fn test_handle_attestation_created() {
        // Setup
        let (tx, rx) = mpsc::channel(100);
        let policy_verifier = Arc::new(MockPolicyVerifier);
        let policy_repo = Arc::new(InMemoryPolicyRepository::new());
        let attestation_storage = Arc::new(InMemoryAttestationStorage::new());

        let mut manager = CBPManager::new(
            policy_verifier,
            policy_repo.clone(),
            attestation_storage.clone(),
            rx,
        );

        // Create a test policy
        let test_policy = Policy {
            purl: "test-purl".to_string(),
            version: "1.0.0".to_string(),
            rules: PolicyRules {
                allowed_issuers: vec!["test-issuer".to_string()].into_iter().collect(),
                max_age_days: 7,
                max_critical_vulnerabilities: 0,
                max_high_medium_vulnerabilities: 5,
            },
        };
        policy_repo.add_policy(test_policy).await.unwrap();

        // Create a test attestation with the correct structure
        let test_attestation = Attestation {
            id: "test-attestation-id".to_string(),
            issuer: "test-issuer".to_string(),
            timestamp: chrono::Utc::now(),
            content: json!({
                "subject": [
                    {
                        "name": "test-artifact",
                        "digest": {"sha256": "test-digest"}
                    }
                ],
                "purl": "test-purl"
            }),
        };

        // Store the test attestation
        let uri = attestation_storage.store_attestation(Arc::new(test_attestation.clone())).await.unwrap();

        // Send an AttestationCreated event
        let event = CDEvent::new(
            CDEventType::AttestationCreated {
                attestation_id: test_attestation.id.clone(),
                attestation_uri: uri.clone(),
            },
            EventSubject {
                id: test_attestation.id.clone(),
                subject_type: SubjectType::Attestation,
            },
        );

        tx.send(event).await.unwrap();

        // Run the manager in a separate task
        /*let manager_handle = tokio::spawn(async move {
            if let Err(e) = manager.run().await {
                eprintln!("Manager run error: {:?}", e);
            }
        });*/
        let manager_handle = tokio::spawn(async move {
            manager.run().await;
        });

        // Allow some time for processing
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        // Stop the manager
        drop(tx);
        manager_handle.await.unwrap();

        // Verify that the original attestation is still stored
        let stored_attestation = attestation_storage.get_attestation(&uri).await.unwrap();
        assert_eq!(stored_attestation.id, test_attestation.id);

        // Verify that a summary attestation was created and stored
        let all_attestations = attestation_storage.list_attestations().await.unwrap();
        let summary_attestations: Vec<_> = all_attestations.into_iter()
            .filter(|att| att.content["predicateType"] == "https://in-toto.io/attestation/scai/attribute-report/v0.2")
            .collect();

        assert_eq!(summary_attestations.len(), 1, "Expected 1 summary attestation, found {}", summary_attestations.len());
        let summary_attestation = &summary_attestations[0];

        // Verify the content of the summary attestation
        let content = &summary_attestation.content;
        assert_eq!(content["_type"], "https://in-toto.io/Statement/v1");
        assert_eq!(content["predicateType"], "https://in-toto.io/attestation/scai/attribute-report/v0.2");
        assert_eq!(content["subject"][0]["name"], "test-artifact");
        
        let attributes = content["predicate"]["attributes"].as_array().unwrap();
        assert_eq!(attributes.len(), 1, "Expected 1 attribute, found {}", attributes.len());
        assert_eq!(attributes[0]["attribute"], "VALID_TEST-PURL");
        
        assert_eq!(content["predicate"]["producer"]["uri"], "https://example.com/cbp/build");
        assert_eq!(content["predicate"]["producer"]["name"], "CBP Build Attestor");

        // Print the actual content for debugging
        println!("Summary attestation content: {}", serde_json::to_string_pretty(&content).unwrap());
    }
}