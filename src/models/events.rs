use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CDEvent {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub event_type: CDEventType,
    pub subject: EventSubject,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum CDEventType {
    AttestationCreated {
        attestation_id: String,
        attestation_uri: String,
    },
    AttestationVerified {
        attestation_id: String,
        is_valid: bool,
    },
    BuildStarted {
        build_id: String,
    },
    BuildCompleted {
        build_id: String,
        status: BuildStatus,
    },
    ArtifactPublished {
        artifact_id: String,
        artifact_type: String,
    },
    DeploymentStarted {
        deployment_id: String,
        environment: String,
    },
    DeploymentCompleted {
        deployment_id: String,
        environment: String,
        status: DeploymentStatus,
    },
    PolicyUpdated {
        policy_id: String,
        version: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventSubject {
    pub id: String,
    pub subject_type: SubjectType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SubjectType {
    Attestation,
    Build,
    Artifact,
    Deployment,
    Policy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BuildStatus {
    Success,
    Failure,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeploymentStatus {
    Success,
    Failure,
    Rollback,
}

impl CDEvent {
    pub fn new(event_type: CDEventType, subject: EventSubject) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            event_type,
            subject,
            metadata: serde_json::Value::Null,
        }
    }

    pub fn with_metadata(mut self, metadata: serde_json::Value) -> Self {
        self.metadata = metadata;
        self
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::models::Attestation;

    use super::*;

    #[test]
    fn test_create_attestation_created_event() {
        let attestation = Arc::new(Attestation {
            id: "att123".to_string(),
            issuer: "trusted_issuer".to_string(),
            timestamp: Utc::now(),
            content: serde_json::json!({}),
        });

        let event = CDEvent::new(
            CDEventType::AttestationCreated {
                attestation_id: attestation.id.clone(),
                attestation_uri: "http://example.com/attestations/att123".to_string(),
            },
            EventSubject {
                id: attestation.id.clone(),
                subject_type: SubjectType::Attestation,
            },
        );

        assert_eq!(event.subject.id, "att123");
        match event.event_type {
            CDEventType::AttestationCreated { attestation_id, .. } => {
                assert_eq!(attestation_id, "att123");
            }
            _ => panic!("Unexpected event type"),
        }
    }

    #[test]
    fn test_create_build_completed_event() {
        let build_id = "build456".to_string();
        let event = CDEvent::new(
            CDEventType::BuildCompleted {
                build_id: build_id.clone(),
                status: BuildStatus::Success,
            },
            EventSubject {
                id: build_id.clone(),
                subject_type: SubjectType::Build,
            },
        ).with_metadata(serde_json::json!({
            "duration_seconds": 120,
            "artifact_count": 3,
        }));

        assert_eq!(event.subject.id, "build456");
        match event.event_type {
            CDEventType::BuildCompleted { build_id, status } => {
                assert_eq!(build_id, "build456");
                assert!(matches!(status, BuildStatus::Success));
            }
            _ => panic!("Unexpected event type"),
        }
        assert_eq!(event.metadata["duration_seconds"], 120);
    }
}