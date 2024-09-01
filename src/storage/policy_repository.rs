use async_trait::async_trait;
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::RwLock;
use semver::Version;
use crate::models::Policy;

#[async_trait]
pub trait PolicyRepository: Send + Sync {
    async fn add_policy(&self, policy: Policy) -> Result<(), Box<dyn Error + Send + Sync>>;
    async fn get_policy(&self, purl: &str, version: Option<&str>) -> Result<Arc<Policy>, Box<dyn Error + Send + Sync>>;
    async fn list_policies(&self, purl: &str) -> Result<Vec<Arc<Policy>>, Box<dyn Error + Send + Sync>>;
    async fn delete_policy(&self, purl: &str, version: &str) -> Result<(), Box<dyn Error + Send + Sync>>;
}

#[derive(Debug, Clone)]
struct VersionedPolicy {
    policy: Arc<Policy>,
    version: Version,
}

pub struct InMemoryPolicyRepository {
    policies: RwLock<HashMap<String, Vec<VersionedPolicy>>>,
}

impl InMemoryPolicyRepository {
    pub fn new() -> Self {
        Self {
            policies: RwLock::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl PolicyRepository for InMemoryPolicyRepository {
    async fn add_policy(&self, policy: Policy) -> Result<(), Box<dyn Error + Send + Sync>> {
        let version = Version::parse(&policy.version)?;
        let versioned_policy = VersionedPolicy {
            policy: Arc::new(policy.clone()),
            version,
        };

        let mut policies = self.policies.write().await;
        policies
            .entry(policy.purl.clone())
            .or_insert_with(Vec::new)
            .push(versioned_policy);

        Ok(())
    }

    async fn get_policy(&self, purl: &str, version: Option<&str>) -> Result<Arc<Policy>, Box<dyn Error + Send + Sync>> {
        let policies = self.policies.read().await;
        let policy_versions = policies.get(purl).ok_or("Policy not found")?;

        match version {
            Some(v) => {
                let version = Version::parse(v)?;
                policy_versions
                    .iter()
                    .find(|p| p.version == version)
                    .map(|p| p.policy.clone())
                    .ok_or_else(|| "Specific version not found".into())
            }
            None => {
                policy_versions
                    .iter()
                    .max_by(|a, b| a.version.cmp(&b.version))
                    .map(|p| p.policy.clone())
                    .ok_or_else(|| "No versions available".into())
            }
        }
    }

    async fn list_policies(&self, purl: &str) -> Result<Vec<Arc<Policy>>, Box<dyn Error + Send + Sync>> {
        let policies = self.policies.read().await;
        Ok(policies
            .get(purl)
            .map(|versions| versions.iter().map(|v| v.policy.clone()).collect())
            .unwrap_or_default())
    }

    async fn delete_policy(&self, purl: &str, version: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut policies = self.policies.write().await;
        let version = Version::parse(version)?;

        if let Some(versions) = policies.get_mut(purl) {
            let initial_len = versions.len();
            versions.retain(|p| p.version != version);
            
            if versions.len() == initial_len {
                return Err("Specific version not found".into());
            }
            
            if versions.is_empty() {
                policies.remove(purl);
            }
        } else {
            return Err("Policy not found".into());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::PolicyRules;

    #[tokio::test]
    async fn test_in_memory_policy_repository() {
        let repo = InMemoryPolicyRepository::new();

        // Create test policies
        let policy1 = Policy {
            purl: "pkg:policy/test".to_string(),
            version: "1.0.0".to_string(),
            rules: PolicyRules {
                allowed_issuers: vec!["issuer1".to_string()].into_iter().collect(),
                max_age_days: 7,
                max_critical_vulnerabilities: 0,
                max_high_medium_vulnerabilities: 5,
            },
        };

        let policy2 = Policy {
            purl: "pkg:policy/test".to_string(),
            version: "1.1.0".to_string(),
            rules: PolicyRules {
                allowed_issuers: vec!["issuer1".to_string(), "issuer2".to_string()].into_iter().collect(),
                max_age_days: 14,
                max_critical_vulnerabilities: 0,
                max_high_medium_vulnerabilities: 3,
            },
        };

        // Test adding policies
        repo.add_policy(policy1.clone()).await.unwrap();
        repo.add_policy(policy2.clone()).await.unwrap();

        // Test getting specific version
        let retrieved1 = repo.get_policy("pkg:policy/test", Some("1.0.0")).await.unwrap();
        assert_eq!(retrieved1.version, "1.0.0");

        // Test getting latest version
        let latest = repo.get_policy("pkg:policy/test", None).await.unwrap();
        assert_eq!(latest.version, "1.1.0");

        // Test listing policies
        let all_policies = repo.list_policies("pkg:policy/test").await.unwrap();
        assert_eq!(all_policies.len(), 2);

        // Test deleting a policy
        repo.delete_policy("pkg:policy/test", "1.0.0").await.unwrap();
        assert!(repo.get_policy("pkg:policy/test", Some("1.0.0")).await.is_err());

        // Test error handling
        assert!(repo.get_policy("non_existent", None).await.is_err());
        assert!(repo.delete_policy("pkg:policy/test", "2.0.0").await.is_err());
    }
}