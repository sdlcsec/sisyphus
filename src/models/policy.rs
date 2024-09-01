use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use chrono::Duration;
use semver::Version;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Policy {
    pub purl: String,
    pub version: String,
    pub rules: PolicyRules,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyRules {
    pub allowed_issuers: HashSet<String>,
    pub max_age_days: u32,
    pub max_critical_vulnerabilities: u32,
    pub max_high_medium_vulnerabilities: u32,
}

impl Policy {
    pub fn new(purl: String, version: String, rules: PolicyRules) -> Result<Self, String> {
        // Validate the version string
        Version::parse(&version).map_err(|e| format!("Invalid version string: {}", e))?;

        Ok(Self {
            purl,
            version,
            rules,
        })
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.purl.is_empty() {
            return Err("PURL cannot be empty".to_string());
        }

        Version::parse(&self.version).map_err(|e| format!("Invalid version string: {}", e))?;

        self.rules.validate()?;

        Ok(())
    }
}

impl PolicyRules {
    pub fn new(
        allowed_issuers: HashSet<String>,
        max_age_days: u32,
        max_critical_vulnerabilities: u32,
        max_high_medium_vulnerabilities: u32,
    ) -> Self {
        Self {
            allowed_issuers,
            max_age_days,
            max_critical_vulnerabilities,
            max_high_medium_vulnerabilities,
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.allowed_issuers.is_empty() {
            return Err("At least one allowed issuer must be specified".to_string());
        }

        if self.max_age_days == 0 {
            return Err("Max age must be greater than 0 days".to_string());
        }

        Ok(())
    }

    pub fn is_issuer_allowed(&self, issuer: &str) -> bool {
        self.allowed_issuers.contains(issuer)
    }

    pub fn max_age(&self) -> Duration {
        Duration::days(self.max_age_days as i64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_policy_creation_and_validation() {
        let rules = PolicyRules::new(
            vec!["trusted_issuer".to_string()].into_iter().collect(),
            7,
            0,
            5,
        );

        let policy = Policy::new(
            "pkg:policy/test".to_string(),
            "1.0.0".to_string(),
            rules,
        ).unwrap();

        assert!(policy.validate().is_ok());
    }

    #[test]
    fn test_policy_invalid_version() {
        let rules = PolicyRules::new(
            vec!["trusted_issuer".to_string()].into_iter().collect(),
            7,
            0,
            5,
        );

        let result = Policy::new(
            "pkg:policy/test".to_string(),
            "invalid_version".to_string(),
            rules,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_policy_rules_validation() {
        let valid_rules = PolicyRules::new(
            vec!["trusted_issuer".to_string()].into_iter().collect(),
            7,
            0,
            5,
        );
        assert!(valid_rules.validate().is_ok());

        let invalid_rules = PolicyRules::new(
            HashSet::new(),
            0,
            0,
            5,
        );
        assert!(invalid_rules.validate().is_err());
    }
}