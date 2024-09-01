use async_trait::async_trait;
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::models::Attestation;

#[async_trait]
pub trait AttestationStorage: Send + Sync {
    async fn store_attestation(&self, attestation: Arc<Attestation>) -> Result<(), Box<dyn Error + Send + Sync>>;
    async fn get_attestation(&self, id: &str) -> Result<Arc<Attestation>, Box<dyn Error + Send + Sync>>;
    async fn delete_attestation(&self, id: &str) -> Result<(), Box<dyn Error + Send + Sync>>;
    async fn list_attestations(&self) -> Result<Vec<Arc<Attestation>>, Box<dyn Error + Send + Sync>>;
}

pub struct InMemoryAttestationStorage {
    attestations: RwLock<HashMap<String, Arc<Attestation>>>,
}

impl InMemoryAttestationStorage {
    pub fn new() -> Self {
        Self {
            attestations: RwLock::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl AttestationStorage for InMemoryAttestationStorage {
    async fn store_attestation(&self, attestation: Arc<Attestation>) -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut attestations = self.attestations.write().await;
        attestations.insert(attestation.id.clone(), attestation);
        Ok(())
    }

    async fn get_attestation(&self, id: &str) -> Result<Arc<Attestation>, Box<dyn Error + Send + Sync>> {
        let attestations = self.attestations.read().await;
        attestations
            .get(id)
            .cloned()
            .ok_or_else(|| Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "Attestation not found")) as Box<dyn Error + Send + Sync>)
    }

    async fn delete_attestation(&self, id: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut attestations = self.attestations.write().await;
        attestations.remove(id).ok_or_else(|| Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "Attestation not found")) as Box<dyn Error + Send + Sync>)?;
        Ok(())
    }

    async fn list_attestations(&self) -> Result<Vec<Arc<Attestation>>, Box<dyn Error + Send + Sync>> {
        let attestations = self.attestations.read().await;
        Ok(attestations.values().cloned().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[tokio::test]
    async fn test_in_memory_attestation_storage() {
        let storage = InMemoryAttestationStorage::new();

        let attestation1 = Arc::new(Attestation {
            id: "att1".to_string(),
            issuer: "issuer1".to_string(),
            timestamp: Utc::now(),
            content: serde_json::json!({"key": "value1"}),
        });

        let attestation2 = Arc::new(Attestation {
            id: "att2".to_string(),
            issuer: "issuer2".to_string(),
            timestamp: Utc::now(),
            content: serde_json::json!({"key": "value2"}),
        });

        // Test storing attestations
        storage.store_attestation(attestation1.clone()).await.unwrap();
        storage.store_attestation(attestation2.clone()).await.unwrap();

        // Test retrieving attestations
        let retrieved1 = storage.get_attestation("att1").await.unwrap();
        assert_eq!(retrieved1.id, "att1");
        assert_eq!(retrieved1.issuer, "issuer1");

        // Test listing attestations
        let all_attestations = storage.list_attestations().await.unwrap();
        assert_eq!(all_attestations.len(), 2);

        // Test deleting an attestation
        storage.delete_attestation("att1").await.unwrap();
        assert!(storage.get_attestation("att1").await.is_err());

        // Test error handling for non-existent attestation
        assert!(storage.get_attestation("non_existent").await.is_err());
        assert!(storage.delete_attestation("non_existent").await.is_err());
    }
}