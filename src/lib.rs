/**
 * These are a set of abstractions that can be used to model the state of a software project.
 * The basic flow is:
 * 1. Create a new project with a name
 * 2. Initialize the project which should create a git repository
 *
 * The flow of actions for changes to an initialized project should be:
 * 1. Developer writes code
 * 2. Developer commits code
 * 3. Git commit hook generates in-toto attestation for commit, signs it and pushes it to the attestation repository
 * 4. Developer pushes code
 * 5. Proxy server intercepts the push and verifies that the commit has an attestation associated with it
 * 6. Proxy server streams the code to the git repository
 * 7. Build triggers on the git repository
 * 8. Build server verifies that the commit has an attestation associated with it
 * 9. Build server pulls the code
 * 10. Build server builds the code
 * 11. Build server generates a SLSA in-toto attestation for the build, signs it and pushes it to the attestation repository
 * 12. Build server pushes the build artifacts
 * 13. Proxy server intercepts the push and verifies that the build has an attestation associated with it
 * 14. Proxy server streams the build artifacts to the artifact repository
 * 15. Deploy triggers on the artifact repository
 * 16. Deploy server verifies that the build has an attestation associated with it
 * 17. Deploy server pulls the build artifacts
 * 18. Deploy server deploys the build artifacts
 */
mod models;
mod cbp;
mod storage;
mod verification;

use std::fmt::{self, Display};

use thiserror::Error;
struct Uninitialized;

struct Unverified;
struct DevelopmentEnvironmentVerified;
struct SourceVerified;
struct BuildVerified;
struct PackageVerified;
struct DeployVerified;
struct FullyVerified {
    pub digest: String,
    pub version: String,
}

trait VerifiedState {}

impl VerifiedState for Unverified {}
impl VerifiedState for DevelopmentEnvironmentVerified {}
impl VerifiedState for SourceVerified {}
impl VerifiedState for BuildVerified {}
impl VerifiedState for PackageVerified {}
impl VerifiedState for DeployVerified {}
impl VerifiedState for FullyVerified {}

struct SDLCRelease<VerifiedState> {
    name: String,
    state: VerifiedState,
}

/*impl SDLCRelease<Unverified> {
    pub fn new(name: String) -> Self {
        SDLCRelease {
            name,
            state: Unverified,
        }
    }

    pub fn verify(self, summary_scai: SummaryScai) -> Result<SDLCRelease<FullyVerified>, VerificationError> {
        let verified_enum: [&SummaryScaiPredicateAttributesItemAttribute; 5] = [
            &SummaryScaiPredicateAttributesItemAttribute::PassedDevelopmentEnvironment,
            &SummaryScaiPredicateAttributesItemAttribute::PassedSource,
            &SummaryScaiPredicateAttributesItemAttribute::PassedBuild,
            &SummaryScaiPredicateAttributesItemAttribute::PassedPackage,
            &SummaryScaiPredicateAttributesItemAttribute::PassedDeploy,
        ];
        let verified = verified_enum.iter().all(|&x| summary_scai.predicate.attributes.iter().any(|y| y.attribute == *x));
        let resource_descriptor = summary_scai.subject.get(0).unwrap();
        let variant = match resource_descriptor {
            models::summary_scai::ResourceDescriptor::Variant0 { annotations, content, digest, download_location, media_type, name, uri } => todo!(),
            models::summary_scai::ResourceDescriptor::Variant1 { annotations, content, digest, download_location, media_type, name, uri } => todo!(),
            models::summary_scai::ResourceDescriptor::Variant2 { annotations, content, digest, download_location, media_type, name, uri } => todo!(),
        }

        if verified {
            Ok(SDLCRelease {
                name: self.name,
                state: FullyVerified {
                    digest: summary_scai.subject.get(0).unwrap().digest.sha256.clone(),
                    version: "1".into(),
                },
            })
        } else {
            Err(VerificationError::MissingPassed {
                attributes: verified_enum.iter().map(|x| x.to_string()).collect(),
            })
        }
    }
}*/

#[derive(Error, Debug)]
enum VerificationError {
    MissingPassed {
        attributes: Vec<String>,
    },
}

impl Display for VerificationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VerificationError::MissingPassed { attributes } => {
                write!(f, "Missing passed attributes: {:?}", attributes)
            }
        }
    }
}
/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_summary_scai() {
        let json = include_str!("../examples/summary_scai.json");
        let summary_scai: Result<SummaryScai, Box<Error>> = serde_json::from_str(json).map_err(|e| Box::new(e) as Box<dyn Error>);
        if let Err(err) = &summary_scai {
            eprintln!("Error: {}", err);
        }
        assert!(summary_scai.is_ok());
    }
}
    */