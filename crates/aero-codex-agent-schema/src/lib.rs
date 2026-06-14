#![forbid(unsafe_code)]
//! JSON schema generation helpers for agent tools.

use aero_codex_agent_core::JsonSchemaDocument;
use schemars::{schema_for, JsonSchema};
use serde_json::Value;

pub struct SchemaGenerator;

impl SchemaGenerator {
    pub fn schema_for<T: JsonSchema>() -> JsonSchemaDocument {
        let schema = schema_for!(T);
        JsonSchemaDocument {
            value: serde_json::to_value(schema).unwrap_or(Value::Null),
        }
    }
}
