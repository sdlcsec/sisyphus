use async_trait::async_trait;
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::RwLock;
use url::form_urlencoded;
use sha2::{Sha256, Digest};
use crate::models::Attestation;

#[async_trait]
pub trait AttestationStorage: Send + Sync {
    async fn store_attestation(&self, attestation: Arc<Attestation>) -> Result<String, Box<dyn Error + Send + Sync>>;
    async fn get_attestation(&self, uri: &str) -> Result<Arc<Attestation>, Box<dyn Error + Send + Sync>>;
    async fn delete_attestation(&self, uri: &str) -> Result<(), Box<dyn Error + Send + Sync>>;
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

    fn generate_uri(attestation: &Attestation) -> String {
        let predicate_type = attestation.content["predicateType"].as_str().unwrap_or("unknown");
        let encoded_predicate_type = form_urlencoded::byte_serialize(predicate_type.as_bytes()).collect::<String>();

        let attestation_json = serde_json::to_string(attestation).unwrap();
        let hash = Sha256::digest(attestation_json.as_bytes());
        let hash_str = hex::encode(&hash[..6]);  // Use first 6 bytes of the hash

        format!("https://example.com/{}/{}.jsonl", encoded_predicate_type, hash_str)
    }
}

#[async_trait]
impl AttestationStorage for InMemoryAttestationStorage {
    async fn store_attestation(&self, attestation: Arc<Attestation>) -> Result<String, Box<dyn Error + Send + Sync>> {
        let uri = Self::generate_uri(&attestation);
        let mut attestations = self.attestations.write().await;
        attestations.insert(uri.clone(), attestation);
        Ok(uri)
    }

    async fn get_attestation(&self, uri: &str) -> Result<Arc<Attestation>, Box<dyn Error + Send + Sync>> {
        let attestations = self.attestations.read().await;
        attestations
            .get(uri)
            .cloned()
            .ok_or_else(|| Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "Attestation not found")) as Box<dyn Error + Send + Sync>)
    }

    async fn delete_attestation(&self, uri: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut attestations = self.attestations.write().await;
        attestations.remove(uri).ok_or_else(|| Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "Attestation not found")) as Box<dyn Error + Send + Sync>)?;
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
    use serde_json::json;
    use chrono::Utc;

    #[tokio::test]
    async fn test_in_memory_attestation_storage() {
        let storage = InMemoryAttestationStorage::new();

        let attestation1 = Arc::new(Attestation {
            id: "att1".to_string(),
            issuer: "issuer1".to_string(),
            timestamp: Utc::now(),
            content: json!({
                "predicateType": "https://in-toto.io/attestation/scai/attribute-report/v0.2",
                "key": "value1"
            }),
        });

        let attestation2 = Arc::new(Attestation {
            id: "att2".to_string(),
            issuer: "issuer2".to_string(),
            timestamp: Utc::now(),
            content: json!({
                "predicateType": "https://example.com/custom-attestation/v1",
                "key": "value2"
            }),
        });

        // Test storing attestations
        let uri1 = storage.store_attestation(attestation1.clone()).await.unwrap();
        let uri2 = storage.store_attestation(attestation2.clone()).await.unwrap();

        // Check that URIs are correctly formatted
        assert!(uri1.starts_with("https://example.com/https%3A%2F%2Fin-toto.io%2Fattestation%2Fscai%2Fattribute-report%2Fv0.2/"));
        assert!(uri1.ends_with(".jsonl"));
        assert!(uri2.starts_with("https://example.com/https%3A%2F%2Fexample.com%2Fcustom-attestation%2Fv1/"));
        assert!(uri2.ends_with(".jsonl"));

        // Test retrieving attestations
        let retrieved1 = storage.get_attestation(&uri1).await.unwrap();
        assert_eq!(retrieved1.id, "att1");
        assert_eq!(retrieved1.issuer, "issuer1");

        // Test listing attestations
        let all_attestations = storage.list_attestations().await.unwrap();
        assert_eq!(all_attestations.len(), 2);

        // Test deleting an attestation
        storage.delete_attestation(&uri1).await.unwrap();
        assert!(storage.get_attestation(&uri1).await.is_err());

        // Test error handling for non-existent attestation
        assert!(storage.get_attestation("non_existent").await.is_err());
        assert!(storage.delete_attestation("non_existent").await.is_err());
    }
}