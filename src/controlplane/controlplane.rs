use std::collections::HashMap;
use chrono::{Duration, Utc};
use serde_json::json;

use crate::models::attestation::Attestation;
use crate::models::policy::{Policy, PolicyRules};
use crate::storage::policy_repository::{InMemoryPolicyRepository, PolicyRepository};
use crate::storage::attestation_storage::{AttestationStorage, InMemoryAttestationStorage};
use crate::verification::policy_verifier::{PolicyVerifier, SimplePolicyVerifier};
use std::sync::Arc;

pub struct Component {
    pub name: String,
    pub version: String,
    pub policy: Arc<Policy>,
}

pub struct SDLCProject {
    pub name: String,
    pub components: Vec<Component>,
}

pub struct ControlPlane<P: PolicyRepository, A: AttestationStorage, V: PolicyVerifier> {
    projects: HashMap<String, SDLCProject>,
    policy_repo: Arc<P>,
    attestation_storage: Arc<A>,
    policy_verifier: Arc<V>,
}

impl<P: PolicyRepository, A: AttestationStorage, V: PolicyVerifier> ControlPlane<P, A, V> {
    pub fn new(policy_repo: Arc<P>, attestation_storage: Arc<A>, policy_verifier: Arc<V>) -> Self {
        Self {
            projects: HashMap::new(),
            policy_repo,
            attestation_storage,
            policy_verifier,
        }
    }

    pub async fn add_project(&mut self, project: SDLCProject) {
        self.projects.insert(project.name.clone(), project);
    }

    pub async fn verify_project(&self, project_name: &str) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        let project = self.projects.get(project_name).ok_or("Project not found")?;
        
        for component in &project.components {
            println!("Verifying component: {}", component.name);
            let attestations = self.attestation_storage.list_attestations().await?;
            println!("Total attestations: {}", attestations.len());
            
            let matching_attestation = attestations.iter().find(|att| {
                let matches = att.content["subject"].as_array()
                    .and_then(|subjects| subjects.first())
                    .and_then(|subject| {
                        let name_match = subject["name"].as_str() == Some(&component.name);
                        let version_match = subject["version"].as_str() == Some(&component.version);
                        if name_match && version_match {
                            Some(())
                        } else {
                            None
                        }
                    })
                    .is_some();
                println!("Attestation {} matches component: {}", att.id, matches);
                matches
            });

            if let Some(attestation) = matching_attestation {
                println!("Found matching attestation: {}", attestation.id);
                let is_valid = self.policy_verifier.verify_attestation(attestation, &component.policy).await?;
                println!("Attestation verification result: {}", is_valid);
                if !is_valid {
                    println!("Component {} failed verification", component.name);
                    return Ok(false);
                }
            } else {
                println!("No matching attestation found for component {}", component.name);
                return Ok(false);
            }
        }

        println!("All components verified successfully");
        Ok(true)
    }
}

#[tokio::test]
async fn test_acme_app_x_project() {
    // Initialize repositories and verifier
    let policy_repo = Arc::new(InMemoryPolicyRepository::new());
    let attestation_storage = Arc::new(InMemoryAttestationStorage::new());
    let policy_verifier = Arc::new(SimplePolicyVerifier);

    // Create a control plane
    let mut control_plane = ControlPlane::new(
        policy_repo.clone(),
        attestation_storage.clone(),
        policy_verifier,
    );

    // Create policies for components
    let frontend_policy = Policy {
        purl: "pkg:github/acme/frontend".to_string(),
        version: "1.0.0".to_string(),
        rules: PolicyRules {
            allowed_issuers: vec!["trusted_issuer".to_string()].into_iter().collect(),
            max_age_days: 30,
            max_critical_vulnerabilities: 0,
            max_high_medium_vulnerabilities: 5,
        },
    };
    let backend_policy = Policy {
        purl: "pkg:github/acme/backend".to_string(),
        version: "1.0.0".to_string(),
        rules: PolicyRules {
            allowed_issuers: vec!["trusted_issuer".to_string()].into_iter().collect(),
            max_age_days: 30,
            max_critical_vulnerabilities: 0,
            max_high_medium_vulnerabilities: 3,
        },
    };

    // Add policies to the repository
    policy_repo.add_policy(frontend_policy.clone()).await.unwrap();
    policy_repo.add_policy(backend_policy.clone()).await.unwrap();

    // Create ACMEAppX project
    let acme_app_x = SDLCProject {
        name: "ACMEAppX".to_string(),
        components: vec![
            Component {
                name: "frontend".to_string(),
                version: "1.2.3".to_string(),
                policy: Arc::new(frontend_policy),
            },
            Component {
                name: "backend".to_string(),
                version: "2.3.4".to_string(),
                policy: Arc::new(backend_policy),
            },
        ],
    };

    // Add project to the control plane
    control_plane.add_project(acme_app_x).await;

    // Create valid attestations for components
    let frontend_attestation = Attestation {
        id: "frontend-att".to_string(),
        issuer: "trusted_issuer".to_string(),
        timestamp: Utc::now(),
        content: json!({
            "subject": [
                {
                    "name": "frontend",
                    "version": "1.2.3"
                }
            ],
            "vulnerabilities": {
                "critical": 0,
                "high": 2,
                "medium": 2,
                "low": 10
            }
        }),
    };

    let backend_attestation = Attestation {
        id: "backend-att".to_string(),
        issuer: "trusted_issuer".to_string(),
        timestamp: Utc::now(),
        content: json!({
            "subject": [
                {
                    "name": "backend",
                    "version": "2.3.4"
                }
            ],
            "vulnerabilities": {
                "critical": 0,
                "high": 1,
                "medium": 1,
                "low": 5
            }
        }),
    };

    // Store attestations and keep the URIs
    let frontend_uri = attestation_storage.store_attestation(Arc::new(frontend_attestation)).await.unwrap();
    let backend_uri = attestation_storage.store_attestation(Arc::new(backend_attestation)).await.unwrap();

    println!("Frontend attestation URI: {}", frontend_uri);
    println!("Backend attestation URI: {}", backend_uri);

    // Verify the project
    let is_valid = control_plane.verify_project("ACMEAppX").await.unwrap();
    assert!(is_valid, "ACMEAppX should be valid");

    // Test with an invalid attestation
    let invalid_backend_attestation = Attestation {
        id: "invalid-backend-att".to_string(),
        issuer: "trusted_issuer".to_string(),
        timestamp: Utc::now(),
        content: json!({
            "subject": [
                {
                    "name": "backend",
                    "version": "2.3.4"
                }
            ],
            "vulnerabilities": {
                "critical": 1,
                "high": 3,
                "medium": 2,
                "low": 5
            }
        }),
    };

    // Replace the valid backend attestation with the invalid one
    attestation_storage.delete_attestation(&backend_uri).await.unwrap();
    let new_backend_uri = attestation_storage.store_attestation(Arc::new(invalid_backend_attestation)).await.unwrap();

    println!("New backend attestation URI: {}", new_backend_uri);

    // List all attestations for debugging
    let all_attestations = attestation_storage.list_attestations().await.unwrap();
    println!("All attestations after replacement:");
    for att in all_attestations {
        println!("ID: {}, Subject: {:?}", att.id, att.content["subject"]);
    }

    // Verify the project again
    let is_valid = control_plane.verify_project("ACMEAppX").await.unwrap();
    assert!(!is_valid, "ACMEAppX should be invalid due to the backend component");
}

#[tokio::test]
async fn test_project_with_failing_policy() {
    // Initialize repositories and verifier
    let policy_repo = Arc::new(InMemoryPolicyRepository::new());
    let attestation_storage = Arc::new(InMemoryAttestationStorage::new());
    let policy_verifier = Arc::new(SimplePolicyVerifier);

    // Create a control plane
    let mut control_plane = ControlPlane::new(
        policy_repo.clone(),
        attestation_storage.clone(),
        policy_verifier,
    );

    // Create a strict policy for the component
    let strict_policy = Policy {
        purl: "pkg:github/acme/strict-component".to_string(),
        version: "1.0.0".to_string(),
        rules: PolicyRules {
            allowed_issuers: vec!["trusted_issuer".to_string()].into_iter().collect(),
            max_age_days: 7, // Strict: Only 7 days old attestations allowed
            max_critical_vulnerabilities: 0,
            max_high_medium_vulnerabilities: 2, // Strict: Only 2 high/medium vulnerabilities allowed
        },
    };

    // Add policy to the repository
    policy_repo.add_policy(strict_policy.clone()).await.unwrap();

    // Create project with the strict component
    let strict_project = SDLCProject {
        name: "StrictProject".to_string(),
        components: vec![
            Component {
                name: "strict-component".to_string(),
                version: "1.0.0".to_string(),
                policy: Arc::new(strict_policy),
            },
        ],
    };

    // Add project to the control plane
    control_plane.add_project(strict_project).await;

    // Create an attestation that violates the policy
    let violating_attestation = Attestation {
        id: "violating-att".to_string(),
        issuer: "trusted_issuer".to_string(),
        timestamp: Utc::now() - Duration::days(10), // Older than allowed
        content: json!({
            "subject": [
                {
                    "name": "strict-component",
                    "version": "1.0.0"
                }
            ],
            "vulnerabilities": {
                "critical": 0,
                "high": 2,
                "medium": 1, // Total high+medium is 3, which exceeds the limit
                "low": 5
            }
        }),
    };

    // Store the violating attestation
    let violating_uri = attestation_storage.store_attestation(Arc::new(violating_attestation)).await.unwrap();
    println!("Stored violating attestation with URI: {}", violating_uri);

    // Verify the project
    let is_valid = control_plane.verify_project("StrictProject").await.unwrap();
    assert!(!is_valid, "StrictProject should be invalid due to policy violations");

    // Create a valid attestation
    let valid_attestation = Attestation {
        id: "valid-att".to_string(),
        issuer: "trusted_issuer".to_string(),
        timestamp: Utc::now(), // Current timestamp
        content: json!({
            "subject": [
                {
                    "name": "strict-component",
                    "version": "1.0.0"
                }
            ],
            "vulnerabilities": {
                "critical": 0,
                "high": 1,
                "medium": 1, // Total high+medium is 2, which meets the limit
                "low": 5
            }
        }),
    };

    // Replace the violating attestation with the valid one
    attestation_storage.delete_attestation(&violating_uri).await.unwrap();
    let valid_uri = attestation_storage.store_attestation(Arc::new(valid_attestation)).await.unwrap();
    println!("Stored valid attestation with URI: {}", valid_uri);

    // Print all stored attestations
    let all_attestations = attestation_storage.list_attestations().await.unwrap();
    println!("All stored attestations:");
    for att in all_attestations {
        println!("ID: {}, Subject: {:?}", att.id, att.content["subject"]);
    }

    // Verify the project again
    let is_valid = control_plane.verify_project("StrictProject").await.unwrap();
    assert!(is_valid, "StrictProject should be valid after replacing with a compliant attestation");
}