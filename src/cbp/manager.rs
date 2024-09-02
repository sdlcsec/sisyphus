use std::error::Error;
use std::sync::Arc;
use tokio::sync::mpsc;
use serde_json::{json, Value};
use crate::models::{Attestation, Policy, CDEvent, CDEventType, EventSubject, SubjectType};
use crate::storage::{PolicyRepository, AttestationStorage, InMemoryPolicyRepository, InMemoryAttestationStorage};
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
        }
    }

    pub async fn run(&mut self) {
        while let Some(event) = self.event_receiver.recv().await {
            if let Err(e) = self.handle_event(event).await {
                eprintln!("Error handling event: {}", e);
            }
        }
    }

    async fn handle_event(&self, event: CDEvent) -> Result<(), Box<dyn Error + Send + Sync>> {
        match event.event_type {
            CDEventType::AttestationCreated { attestation_id, attestation_uri } => {
                self.handle_attestation_created(attestation_id, attestation_uri).await?;
            }
            _ => {
                // Handle other event types as needed
            }
        }
        Ok(())
    }

    async fn handle_attestation_created(&self, attestation_id: String, attestation_uri: String) -> Result<(), Box<dyn Error + Send + Sync>> {
        // 1. Fetch the attestation using the URI
        let attestation = self.attestation_storage.get_attestation(&attestation_uri).await?;

        // 2. Retrieve the relevant policy
        let policy = self.policy_repo.get_policy(&attestation.content["purl"].as_str().unwrap_or(""), None).await?;

        // 3. Verify the attestation against the policy
        let is_valid = self.policy_verifier.verify_attestation(&attestation, &policy).await?;

        // 4. Generate a summary attestation
        let summary_attestation = self.generate_summary_attestation(&attestation, is_valid).await?;

        // 5. Store the summary attestation
        let summary_uri = format!("summary:{}", summary_attestation.id);
        self.attestation_storage.store_attestation(Arc::new(summary_attestation.clone())).await?;

        // 6. Create and emit a new CDEvent for the summary attestation
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

        // 7. Emit the summary event
        // TODO: Implement this

        Ok(())
    }

    async fn generate_summary_attestation(&self, original_attestation: &Attestation, is_valid: bool) -> Result<Attestation, Box<dyn Error + Send + Sync>> {
        let attribute = self.determine_attribute(original_attestation)?;
        let evidence = self.create_evidence(original_attestation)?;

        let summary_content = json!({
            "_type": "https://in-toto.io/Statement/v1",
            "subject": [
                {
                    "name": original_attestation.content["name"].as_str().unwrap_or("example-software-artifact"),
                    "digest": original_attestation.content["digest"].clone(),
                }
            ],
            "predicateType": "https://in-toto.io/attestation/scai/attribute-report/v0.2",
            "predicate": {
                "attributes": [
                    {
                        "attribute": attribute,
                        "evidence": evidence,
                    }
                ],
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

        Ok(summary_attestation)
    }

    fn determine_attribute(&self, attestation: &Attestation) -> Result<String, Box<dyn Error + Send + Sync>> {
        // This is a placeholder. In a real implementation, you would analyze the attestation
        // to determine the appropriate attribute (e.g., "SLSA_L3", "VALID_SBOM", etc.)
        Ok("SLSA_L3".to_string())
    }

    fn create_evidence(&self, attestation: &Attestation) -> Result<Value, Box<dyn Error + Send + Sync>> {
        let evidence = json!({
            "name": format!("{}.slsa.jsonl", attestation.id),
            "uri": format!("https://example.com/scai/{}.slsa.jsonl", attestation.id),
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
    use crate::verification::SimplePolicyVerifier;
    use crate::models::PolicyRules;

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

        // Create a test attestation
        let test_attestation = Attestation {
            id: "test-attestation-id".to_string(),
            issuer: "test-issuer".to_string(),
            timestamp: chrono::Utc::now(),
            content: json!({
                "name": "test-artifact",
                "digest": {"sha256": "test-digest"},
                "purl": "test-purl"
            }),
        };

        // Store the test attestation
        let attestation_uri = format!("test:{}", test_attestation.id);
        attestation_storage.store_attestation(Arc::new(test_attestation.clone())).await.unwrap();

        // Send an AttestationCreated event
        let event = CDEvent::new(
            CDEventType::AttestationCreated {
                attestation_id: test_attestation.id.clone(),
                attestation_uri: attestation_uri.clone(),
            },
            EventSubject {
                id: test_attestation.id.clone(),
                subject_type: SubjectType::Attestation,
            },
        );

        tx.send(event).await.unwrap();

        // Run the manager
        tokio::spawn(async move {
            manager.run().await;
        });

        // Allow some time for processing
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        // Verify that the original attestation is still stored
        let stored_attestation = attestation_storage.get_attestation(&attestation_uri).await.unwrap();
        assert_eq!(stored_attestation.id, test_attestation.id);

        // Verify that a summary attestation was created and stored
        let all_attestations = attestation_storage.list_attestations().await.unwrap();
        let summary_attestations: Vec<_> = all_attestations.into_iter()
            .filter(|att| att.id.starts_with("summary:"))
            .collect();

        assert_eq!(summary_attestations.len(), 1);
        let summary_attestation = &summary_attestations[0];

        // Verify the content of the summary attestation
        let content = &summary_attestation.content;
        assert_eq!(content["_type"], "https://in-toto.io/Statement/v1");
        assert_eq!(content["predicateType"], "https://in-toto.io/attestation/scai/attribute-report/v0.2");
        assert_eq!(content["subject"][0]["name"], "test-artifact");
        assert_eq!(content["subject"][0]["digest"]["sha256"], "test-digest");
        assert_eq!(content["predicate"]["attributes"][0]["attribute"], "SLSA_L3");
        assert_eq!(content["predicate"]["producer"]["uri"], "https://example.com/cbp/build");
        assert_eq!(content["predicate"]["producer"]["name"], "CBP Build Attestor");
    }
}