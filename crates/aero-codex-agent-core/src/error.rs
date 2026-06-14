use crate::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub enum Recoverability {
    RetryWithCorrectedInput,
    SelectDifferentTool,
    CorrectInputOrSelectDifferentModel,
    NotRecoverable,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub enum AgentErrorCode {
    InvalidJson,
    SchemaViolation,
    UnknownTool,
    ToolNotCallable,
    MissingRequiredInput,
    InvalidUnit,
    InvalidFrame,
    InvalidAngle,
    InvalidBranch,
    AmbiguousBranch,
    OutsideValidityRange,
    RequiresSupersonic,
    RequiresSubsonic,
    NoConvergence,
    MissingEvidenceCard,
    DeprecatedTool,
    InternalError,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Error)]
#[error("{code:?}: {message}")]
pub struct AgentError {
    pub code: AgentErrorCode,
    pub message: String,
    pub codex_id: Option<CodexId>,
    pub tool_name: Option<AgentToolName>,
    pub parameter: Option<String>,
    pub received_value: Option<Value>,
    pub expected_condition: Option<String>,
    pub recoverability: Recoverability,
    pub suggested_tools: Vec<AgentToolName>,
    pub suggested_questions: Vec<String>,
    pub evidence: Option<EvidenceSummary>,
}

pub type AgentResult<T> = Result<T, Box<AgentError>>;

impl AgentError {
    #[must_use]
    pub fn requires_supersonic(
        tool_name: AgentToolName,
        codex_id: CodexId,
        parameter: &str,
        value: f64,
    ) -> Self {
        Self {
            code: AgentErrorCode::RequiresSupersonic,
            message: "Normal shock relations require upstream Mach number greater than 1."
                .to_owned(),
            codex_id: Some(codex_id),
            tool_name: Some(tool_name),
            parameter: Some(parameter.to_owned()),
            received_value: Some(Value::from(value)),
            expected_condition: Some(format!("{parameter} > 1")),
            recoverability: Recoverability::CorrectInputOrSelectDifferentModel,
            suggested_tools: vec![AgentToolName(
                "gasdyn.isentropic.static_to_total_pressure_ratio".to_owned(),
            )],
            suggested_questions: vec![
                "Is the upstream flow actually supersonic?".to_owned(),
                "Do you want an isentropic subsonic relation instead?".to_owned(),
            ],
            evidence: None,
        }
    }
}
