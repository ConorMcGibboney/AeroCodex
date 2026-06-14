use crate::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DatasetChecksum {
    pub dataset_id: String,
    pub sha256: Sha256Hash,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TraceEnvironment {
    pub os: String,
    pub arch: String,
    pub rustc: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AgentInvocationRecord {
    pub tool_name: AgentToolName,
    pub codex_id: CodexId,
    pub raw_input_json: Value,
    pub normalized_input_json: Value,
    pub context: AgentInvocationContext,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AgentResultRecord {
    pub success: bool,
    pub raw_output_json: Option<Value>,
    pub error: Option<AgentError>,
    pub warnings: Vec<AgentWarning>,
    pub validity: Option<ValidityStatus>,
    pub duration_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AgentTrace {
    pub trace_id: TraceId,
    pub created_at: Timestamp,
    pub aero_codex_version: SemVer,
    pub workspace_git_sha: Option<String>,
    pub invocation: AgentInvocationRecord,
    pub result: AgentResultRecord,
    pub evidence: EvidenceSummary,
    pub data_checksums: Vec<DatasetChecksum>,
    pub environment: TraceEnvironment,
}
