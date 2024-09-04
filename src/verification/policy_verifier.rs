use async_trait::async_trait;
use chrono::{Duration, Utc};
use std::error::Error;

use crate::models::{attestation::Attestation, policy::Policy};

#[async_trait]
pub trait PolicyVerifier: Send + Sync {
    async fn verify_attestation(&self, attestation: &Attestation, policy: &Policy) -> Result<bool, Box<dyn Error + Send + Sync>>;
}

pub struct SimplePolicyVerifier;

#[async_trait]
impl PolicyVerifier for SimplePolicyVerifier {
    async fn verify_attestation(&self, attestation: &Attestation, policy: &Policy) -> Result<bool, Box<dyn Error + Send + Sync>> {
        // 1. Verify the identity
        if !policy.rules.allowed_issuers.contains(&attestation.issuer) {
            return Ok(false);
        }

        // 2. Ensure the attestation's timestamp is within the policy time frame
        let age = Utc::now() - attestation.timestamp;
        if age > Duration::days(policy.rules.max_age_days as i64) {
            return Ok(false);
        }

        // 3. Verify the values in the JSON of the attestation
        let vulnerabilities = attestation.content.get("vulnerabilities")
            .and_then(|v| v.as_object())
            .ok_or("Missing or invalid vulnerabilities data")?;

        let critical_vulns = vulnerabilities.get("critical")
            .and_then(|v| v.as_u64())
            .unwrap_or(0) as u32;

        let high_vulns = vulnerabilities.get("high")
            .and_then(|v| v.as_u64())
            .unwrap_or(0) as u32;

        let medium_vulns = vulnerabilities.get("medium")
            .and_then(|v| v.as_u64())
            .unwrap_or(0) as u32;

        if critical_vulns > policy.rules.max_critical_vulnerabilities {
            return Ok(false);
        }

        if high_vulns + medium_vulns > policy.rules.max_high_medium_vulnerabilities {
            return Ok(false);
        }

        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::policy::PolicyRules;
    use serde_json::json;

    #[tokio::test]
    async fn test_simple_policy_verifier() {
        let verifier = SimplePolicyVerifier;

        let policy = Policy {
            purl: "pkg:policy/test".to_string(),
            version: "1.0.0".to_string(),
            rules: PolicyRules {
                allowed_issuers: vec!["trusted_issuer".to_string()].into_iter().collect(),
                max_age_days: 7,
                max_critical_vulnerabilities: 0,
                max_high_medium_vulnerabilities: 5,
            },
        };

        let valid_attestation = Attestation {
            id: "test1".to_string(),
            issuer: "trusted_issuer".to_string(),
            timestamp: Utc::now(),
            content: json!({
                "vulnerabilities": {
                    "critical": 0,
                    "high": 2,
                    "medium": 2,
                    "low": 10
                }
            }),
        };

        let invalid_attestation = Attestation {
            id: "test2".to_string(),
            issuer: "untrusted_issuer".to_string(),
            timestamp: Utc::now() - Duration::days(10),
            content: json!({
                "vulnerabilities": {
                    "critical": 1,
                    "high": 3,
                    "medium": 3,
                    "low": 10
                }
            }),
        };

        assert!(verifier.verify_attestation(&valid_attestation, &policy).await.unwrap());
        assert!(!verifier.verify_attestation(&invalid_attestation, &policy).await.unwrap());
    }
}