#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]

use serde::{Deserialize, Serialize};

#[doc = r" Error types."]
pub mod error {
    #[doc = r" Error from a TryFrom or FromStr implementation."]
    pub struct ConversionError(std::borrow::Cow<'static, str>);
    impl std::error::Error for ConversionError {}
    impl std::fmt::Display for ConversionError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl std::fmt::Debug for ConversionError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
#[doc = "ResourceDescriptor"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"required\": ["]
#[doc = "        \"uri\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"required\": ["]
#[doc = "        \"digest\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"required\": ["]
#[doc = "        \"content\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"annotations\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": true"]
#[doc = "    },"]
#[doc = "    \"content\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"digest\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"sha256\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"sha256\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"downloadLocation\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"mediaType\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"uri\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged, deny_unknown_fields)]
pub enum ResourceDescriptor {
    Variant0 {
        #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
        annotations: serde_json::Map<String, serde_json::Value>,
        content: Option<String>,
        digest: Option<ResourceDescriptorVariant1Digest>,
        #[serde(
            rename = "downloadLocation",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        download_location: Option<String>,
        #[serde(rename = "mediaType", default, skip_serializing_if = "Option::is_none")]
        media_type: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        name: Option<String>,
        uri: String,
    },
    Variant1 {
        #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
        annotations: serde_json::Map<String, serde_json::Value>,
        content: Option<String>,
        digest: ResourceDescriptorVariant1Digest,
        #[serde(
            rename = "downloadLocation",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        download_location: Option<String>,
        #[serde(rename = "mediaType", default, skip_serializing_if = "Option::is_none")]
        media_type: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        name: Option<String>,
        uri: Option<String>,
    },
    Variant2 {
        #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
        annotations: serde_json::Map<String, serde_json::Value>,
        content: String,
        digest: Option<ResourceDescriptorVariant1Digest>,
        #[serde(
            rename = "downloadLocation",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        download_location: Option<String>,
        #[serde(rename = "mediaType", default, skip_serializing_if = "Option::is_none")]
        media_type: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        name: Option<String>,
        uri: Option<String>,
    },
}
impl From<&ResourceDescriptor> for ResourceDescriptor {
    fn from(value: &ResourceDescriptor) -> Self {
        value.clone()
    }
}
#[doc = "ResourceDescriptorVariant1Digest"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"sha256\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"sha256\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ResourceDescriptorVariant1Digest {
    pub sha256: String,
}
impl From<&ResourceDescriptorVariant1Digest> for ResourceDescriptorVariant1Digest {
    fn from(value: &ResourceDescriptorVariant1Digest) -> Self {
        value.clone()
    }
}
impl ResourceDescriptorVariant1Digest {
    pub fn builder() -> builder::ResourceDescriptorVariant1Digest {
        Default::default()
    }
}
#[doc = "SummaryScai"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"SummarySCAI\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"_type\","]
#[doc = "    \"predicate\","]
#[doc = "    \"predicateType\","]
#[doc = "    \"subject\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"_type\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"predicate\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"attributes\","]
#[doc = "        \"producer\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"attributes\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"type\": \"object\","]
#[doc = "            \"required\": ["]
#[doc = "              \"attribute\","]
#[doc = "              \"evidence\""]
#[doc = "            ],"]
#[doc = "            \"properties\": {"]
#[doc = "              \"attribute\": {"]
#[doc = "                \"type\": \"string\","]
#[doc = "                \"enum\": ["]
#[doc = "                  \"PASSED_DEVELOPMENT_ENVIRONMENT\","]
#[doc = "                  \"PASSED_SOURCE\","]
#[doc = "                  \"PASSED_BUILD\","]
#[doc = "                  \"PASSED_PACKAGE\","]
#[doc = "                  \"PASSED_DEPLOY\""]
#[doc = "                ]"]
#[doc = "              },"]
#[doc = "              \"conditions\": {"]
#[doc = "                \"type\": \"object\","]
#[doc = "                \"properties\": {"]
#[doc = "                  \"policy\": {"]
#[doc = "                    \"type\": \"string\""]
#[doc = "                  }"]
#[doc = "                }"]
#[doc = "              },"]
#[doc = "              \"evidence\": {"]
#[doc = "                \"$ref\": \"#/$defs/ResourceDescriptor\""]
#[doc = "              }"]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"producer\": {"]
#[doc = "          \"$ref\": \"#/$defs/ResourceDescriptor\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"predicateType\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"subject\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/ResourceDescriptor\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SummaryScai {
    pub predicate: SummaryScaiPredicate,
    #[serde(rename = "predicateType")]
    pub predicate_type: String,
    pub subject: Vec<ResourceDescriptor>,
    #[serde(rename = "_type")]
    pub type_: String,
}
impl From<&SummaryScai> for SummaryScai {
    fn from(value: &SummaryScai) -> Self {
        value.clone()
    }
}
impl SummaryScai {
    pub fn builder() -> builder::SummaryScai {
        Default::default()
    }
}
#[doc = "SummaryScaiPredicate"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"attributes\","]
#[doc = "    \"producer\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"attributes\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"attribute\","]
#[doc = "          \"evidence\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"attribute\": {"]
#[doc = "            \"type\": \"string\","]
#[doc = "            \"enum\": ["]
#[doc = "              \"PASSED_DEVELOPMENT_ENVIRONMENT\","]
#[doc = "              \"PASSED_SOURCE\","]
#[doc = "              \"PASSED_BUILD\","]
#[doc = "              \"PASSED_PACKAGE\","]
#[doc = "              \"PASSED_DEPLOY\""]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"conditions\": {"]
#[doc = "            \"type\": \"object\","]
#[doc = "            \"properties\": {"]
#[doc = "              \"policy\": {"]
#[doc = "                \"type\": \"string\""]
#[doc = "              }"]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"evidence\": {"]
#[doc = "            \"$ref\": \"#/$defs/ResourceDescriptor\""]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"producer\": {"]
#[doc = "      \"$ref\": \"#/$defs/ResourceDescriptor\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SummaryScaiPredicate {
    pub attributes: Vec<SummaryScaiPredicateAttributesItem>,
    pub producer: ResourceDescriptor,
}
impl From<&SummaryScaiPredicate> for SummaryScaiPredicate {
    fn from(value: &SummaryScaiPredicate) -> Self {
        value.clone()
    }
}
impl SummaryScaiPredicate {
    pub fn builder() -> builder::SummaryScaiPredicate {
        Default::default()
    }
}
#[doc = "SummaryScaiPredicateAttributesItem"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"attribute\","]
#[doc = "    \"evidence\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"attribute\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"PASSED_DEVELOPMENT_ENVIRONMENT\","]
#[doc = "        \"PASSED_SOURCE\","]
#[doc = "        \"PASSED_BUILD\","]
#[doc = "        \"PASSED_PACKAGE\","]
#[doc = "        \"PASSED_DEPLOY\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"conditions\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"policy\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"evidence\": {"]
#[doc = "      \"$ref\": \"#/$defs/ResourceDescriptor\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SummaryScaiPredicateAttributesItem {
    pub attribute: SummaryScaiPredicateAttributesItemAttribute,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<SummaryScaiPredicateAttributesItemConditions>,
    pub evidence: ResourceDescriptor,
}
impl From<&SummaryScaiPredicateAttributesItem> for SummaryScaiPredicateAttributesItem {
    fn from(value: &SummaryScaiPredicateAttributesItem) -> Self {
        value.clone()
    }
}
impl SummaryScaiPredicateAttributesItem {
    pub fn builder() -> builder::SummaryScaiPredicateAttributesItem {
        Default::default()
    }
}
#[doc = "SummaryScaiPredicateAttributesItemAttribute"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"PASSED_DEVELOPMENT_ENVIRONMENT\","]
#[doc = "    \"PASSED_SOURCE\","]
#[doc = "    \"PASSED_BUILD\","]
#[doc = "    \"PASSED_PACKAGE\","]
#[doc = "    \"PASSED_DEPLOY\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum SummaryScaiPredicateAttributesItemAttribute {
    #[serde(rename = "PASSED_DEVELOPMENT_ENVIRONMENT")]
    PassedDevelopmentEnvironment,
    #[serde(rename = "PASSED_SOURCE")]
    PassedSource,
    #[serde(rename = "PASSED_BUILD")]
    PassedBuild,
    #[serde(rename = "PASSED_PACKAGE")]
    PassedPackage,
    #[serde(rename = "PASSED_DEPLOY")]
    PassedDeploy,
}
impl From<&SummaryScaiPredicateAttributesItemAttribute>
    for SummaryScaiPredicateAttributesItemAttribute
{
    fn from(value: &SummaryScaiPredicateAttributesItemAttribute) -> Self {
        value.clone()
    }
}
impl ToString for SummaryScaiPredicateAttributesItemAttribute {
    fn to_string(&self) -> String {
        match *self {
            Self::PassedDevelopmentEnvironment => "PASSED_DEVELOPMENT_ENVIRONMENT".to_string(),
            Self::PassedSource => "PASSED_SOURCE".to_string(),
            Self::PassedBuild => "PASSED_BUILD".to_string(),
            Self::PassedPackage => "PASSED_PACKAGE".to_string(),
            Self::PassedDeploy => "PASSED_DEPLOY".to_string(),
        }
    }
}
impl std::str::FromStr for SummaryScaiPredicateAttributesItemAttribute {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "PASSED_DEVELOPMENT_ENVIRONMENT" => Ok(Self::PassedDevelopmentEnvironment),
            "PASSED_SOURCE" => Ok(Self::PassedSource),
            "PASSED_BUILD" => Ok(Self::PassedBuild),
            "PASSED_PACKAGE" => Ok(Self::PassedPackage),
            "PASSED_DEPLOY" => Ok(Self::PassedDeploy),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for SummaryScaiPredicateAttributesItemAttribute {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for SummaryScaiPredicateAttributesItemAttribute {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for SummaryScaiPredicateAttributesItemAttribute {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "SummaryScaiPredicateAttributesItemConditions"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"policy\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SummaryScaiPredicateAttributesItemConditions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
}
impl From<&SummaryScaiPredicateAttributesItemConditions>
    for SummaryScaiPredicateAttributesItemConditions
{
    fn from(value: &SummaryScaiPredicateAttributesItemConditions) -> Self {
        value.clone()
    }
}
impl SummaryScaiPredicateAttributesItemConditions {
    pub fn builder() -> builder::SummaryScaiPredicateAttributesItemConditions {
        Default::default()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct ResourceDescriptorVariant1Digest {
        sha256: Result<String, String>,
    }
    impl Default for ResourceDescriptorVariant1Digest {
        fn default() -> Self {
            Self {
                sha256: Err("no value supplied for sha256".to_string()),
            }
        }
    }
    impl ResourceDescriptorVariant1Digest {
        pub fn sha256<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.sha256 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sha256: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<ResourceDescriptorVariant1Digest>
        for super::ResourceDescriptorVariant1Digest
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ResourceDescriptorVariant1Digest,
        ) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                sha256: value.sha256?,
            })
        }
    }
    impl From<super::ResourceDescriptorVariant1Digest> for ResourceDescriptorVariant1Digest {
        fn from(value: super::ResourceDescriptorVariant1Digest) -> Self {
            Self {
                sha256: Ok(value.sha256),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SummaryScai {
        predicate: Result<super::SummaryScaiPredicate, String>,
        predicate_type: Result<String, String>,
        subject: Result<Vec<super::ResourceDescriptor>, String>,
        type_: Result<String, String>,
    }
    impl Default for SummaryScai {
        fn default() -> Self {
            Self {
                predicate: Err("no value supplied for predicate".to_string()),
                predicate_type: Err("no value supplied for predicate_type".to_string()),
                subject: Err("no value supplied for subject".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl SummaryScai {
        pub fn predicate<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::SummaryScaiPredicate>,
            T::Error: std::fmt::Display,
        {
            self.predicate = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for predicate: {}", e));
            self
        }
        pub fn predicate_type<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.predicate_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for predicate_type: {}", e));
            self
        }
        pub fn subject<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::ResourceDescriptor>>,
            T::Error: std::fmt::Display,
        {
            self.subject = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subject: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<SummaryScai> for super::SummaryScai {
        type Error = super::error::ConversionError;
        fn try_from(value: SummaryScai) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                predicate: value.predicate?,
                predicate_type: value.predicate_type?,
                subject: value.subject?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::SummaryScai> for SummaryScai {
        fn from(value: super::SummaryScai) -> Self {
            Self {
                predicate: Ok(value.predicate),
                predicate_type: Ok(value.predicate_type),
                subject: Ok(value.subject),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SummaryScaiPredicate {
        attributes: Result<Vec<super::SummaryScaiPredicateAttributesItem>, String>,
        producer: Result<super::ResourceDescriptor, String>,
    }
    impl Default for SummaryScaiPredicate {
        fn default() -> Self {
            Self {
                attributes: Err("no value supplied for attributes".to_string()),
                producer: Err("no value supplied for producer".to_string()),
            }
        }
    }
    impl SummaryScaiPredicate {
        pub fn attributes<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::SummaryScaiPredicateAttributesItem>>,
            T::Error: std::fmt::Display,
        {
            self.attributes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for attributes: {}", e));
            self
        }
        pub fn producer<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ResourceDescriptor>,
            T::Error: std::fmt::Display,
        {
            self.producer = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for producer: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<SummaryScaiPredicate> for super::SummaryScaiPredicate {
        type Error = super::error::ConversionError;
        fn try_from(value: SummaryScaiPredicate) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                attributes: value.attributes?,
                producer: value.producer?,
            })
        }
    }
    impl From<super::SummaryScaiPredicate> for SummaryScaiPredicate {
        fn from(value: super::SummaryScaiPredicate) -> Self {
            Self {
                attributes: Ok(value.attributes),
                producer: Ok(value.producer),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SummaryScaiPredicateAttributesItem {
        attribute: Result<super::SummaryScaiPredicateAttributesItemAttribute, String>,
        conditions: Result<Option<super::SummaryScaiPredicateAttributesItemConditions>, String>,
        evidence: Result<super::ResourceDescriptor, String>,
    }
    impl Default for SummaryScaiPredicateAttributesItem {
        fn default() -> Self {
            Self {
                attribute: Err("no value supplied for attribute".to_string()),
                conditions: Ok(Default::default()),
                evidence: Err("no value supplied for evidence".to_string()),
            }
        }
    }
    impl SummaryScaiPredicateAttributesItem {
        pub fn attribute<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::SummaryScaiPredicateAttributesItemAttribute>,
            T::Error: std::fmt::Display,
        {
            self.attribute = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for attribute: {}", e));
            self
        }
        pub fn conditions<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SummaryScaiPredicateAttributesItemConditions>>,
            T::Error: std::fmt::Display,
        {
            self.conditions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for conditions: {}", e));
            self
        }
        pub fn evidence<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ResourceDescriptor>,
            T::Error: std::fmt::Display,
        {
            self.evidence = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for evidence: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<SummaryScaiPredicateAttributesItem>
        for super::SummaryScaiPredicateAttributesItem
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SummaryScaiPredicateAttributesItem,
        ) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                attribute: value.attribute?,
                conditions: value.conditions?,
                evidence: value.evidence?,
            })
        }
    }
    impl From<super::SummaryScaiPredicateAttributesItem> for SummaryScaiPredicateAttributesItem {
        fn from(value: super::SummaryScaiPredicateAttributesItem) -> Self {
            Self {
                attribute: Ok(value.attribute),
                conditions: Ok(value.conditions),
                evidence: Ok(value.evidence),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SummaryScaiPredicateAttributesItemConditions {
        policy: Result<Option<String>, String>,
    }
    impl Default for SummaryScaiPredicateAttributesItemConditions {
        fn default() -> Self {
            Self {
                policy: Ok(Default::default()),
            }
        }
    }
    impl SummaryScaiPredicateAttributesItemConditions {
        pub fn policy<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.policy = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for policy: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<SummaryScaiPredicateAttributesItemConditions>
        for super::SummaryScaiPredicateAttributesItemConditions
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SummaryScaiPredicateAttributesItemConditions,
        ) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                policy: value.policy?,
            })
        }
    }
    impl From<super::SummaryScaiPredicateAttributesItemConditions>
        for SummaryScaiPredicateAttributesItemConditions
    {
        fn from(value: super::SummaryScaiPredicateAttributesItemConditions) -> Self {
            Self {
                policy: Ok(value.policy),
            }
        }
    }
}
