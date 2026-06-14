use crate::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub enum AgentUseMode {
    Discover,
    Explain,
    Compute,
    Verify,
    Replay,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct AgentModeSupport {
    pub discover: bool,
    pub explain: bool,
    pub compute: bool,
    pub verify: bool,
    pub replay: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct JsonSchemaDocument {
    pub value: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct EvidenceSummary {
    pub codex_id: CodexId,
    pub status: EvidenceStatus,
    pub source: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub enum EvidenceStatus {
    C0Proposed,
    C1EquationTraceable,
    C2ImplementationVerified,
    C3ReferenceValidated,
    C4ExperimentValidated,
    CxDeprecatedWithdrawn,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub enum ValidityStatus {
    Valid,
    Warning,
    OutsideValidityRange,
    Invalid,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AgentWarning {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct UncertaintySummary {
    pub description: String,
    pub value: Option<f64>,
    pub unit: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TraceSummary {
    pub trace_id: TraceId,
    pub trace_ref: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AgentEngineeringOutput<T> {
    pub value: T,
    pub quantity: String,
    pub unit: String,
    pub validity: ValidityStatus,
    pub assumptions: Vec<String>,
    pub warnings: Vec<AgentWarning>,
    pub evidence: EvidenceSummary,
    pub uncertainty: Option<UncertaintySummary>,
    pub trace: Option<TraceSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AgentToolExample {
    pub title: String,
    pub input: Value,
    pub output: Option<Value>,
    pub error: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ValidityDomain {
    pub rules: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BranchRule {
    pub parameter: String,
    pub branches: Vec<String>,
    pub default_allowed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AgentToolSpec {
    pub name: AgentToolName,
    pub codex_id: CodexId,
    pub crate_name: CrateName,
    pub version: SemVer,
    pub title: String,
    pub description: String,
    pub mode_support: AgentModeSupport,
    pub input_schema: JsonSchemaDocument,
    pub output_schema: JsonSchemaDocument,
    pub error_schema: JsonSchemaDocument,
    pub assumptions: Vec<String>,
    pub limitations: Vec<String>,
    pub validity: ValidityDomain,
    pub branch_rules: Vec<BranchRule>,
    pub evidence: EvidenceSummary,
    pub examples: Vec<AgentToolExample>,
    pub safety: AgentToolSafety,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub enum TracePolicy {
    None,
    Write,
    Require,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub enum ExtrapolationPolicy {
    Error,
    Warn,
    AllowWithFlag,
    Clamp,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub enum UnitsPolicy {
    Strict,
    ConvertKnownUnits,
    AssumeSiWithWarning,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub enum WarningsPolicy {
    Include,
    PromoteToError,
    SuppressNonSafety,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AgentInvocationContext {
    pub mode: AgentUseMode,
    pub trace_policy: TracePolicy,
    pub evidence_minimum: Option<EvidenceStatus>,
    pub extrapolation_policy: ExtrapolationPolicy,
    pub units_policy: UnitsPolicy,
    pub warnings_policy: WarningsPolicy,
}

pub trait AgentTool: Send + Sync {
    fn spec(&self) -> &'static AgentToolSpec;

    fn invoke_json(&self, input: Value, context: &AgentInvocationContext) -> AgentResult<Value>;
}
