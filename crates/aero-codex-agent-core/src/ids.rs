use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt;

macro_rules! string_id {
    ($name:ident) => {
        #[derive(
            Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, JsonSchema,
        )]
        pub struct $name(pub String);

        impl From<&str> for $name {
            fn from(value: &str) -> Self {
                Self(value.to_owned())
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(f)
            }
        }
    };
}

string_id!(CodexId);
string_id!(CvtId);
string_id!(AgentToolName);
string_id!(CrateName);
string_id!(Domain);
string_id!(CapabilityNodeId);
string_id!(TraceId);
string_id!(SchemaRef);
string_id!(SemVer);
string_id!(SourceRef);
string_id!(EquationRef);
string_id!(Sha256Hash);
string_id!(EmbeddingModelId);
string_id!(Timestamp);

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FieldRef {
    pub name: String,
    pub type_name: String,
    pub required: bool,
    pub unit: Option<String>,
}
