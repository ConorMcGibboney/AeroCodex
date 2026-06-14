use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub enum AgentMisuseRisk {
    Low,
    Medium,
    High,
    Restricted,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AgentToolSafety {
    pub misuse_risk: AgentMisuseRisk,
    pub safety_notes: Vec<String>,
    pub prohibited_claims: Vec<String>,
    pub requires_user_acknowledgement: bool,
}
