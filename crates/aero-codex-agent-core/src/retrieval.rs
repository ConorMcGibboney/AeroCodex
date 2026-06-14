use crate::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RetrievalRequest {
    pub query: String,
    pub domains: Vec<Domain>,
    pub evidence_minimum: Option<EvidenceStatus>,
    pub callable_only: bool,
    pub required_inputs: Vec<String>,
    pub desired_outputs: Vec<String>,
    pub mode: AgentUseMode,
    pub max_results: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub enum RetrievalReason {
    ExactCodexIdMatch,
    SymbolMatch { symbol: String },
    AliasMatch { alias: String },
    SemanticMatch { similarity: f64 },
    DomainMatch { domain: Domain },
    WorkflowDependency,
    EvidenceLevelSatisfied,
    OutputQuantityMatch { quantity: String },
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RetrievalResult {
    pub codex_id: CodexId,
    pub tool_name: Option<AgentToolName>,
    pub title: String,
    pub score: f64,
    pub reasons: Vec<RetrievalReason>,
    pub evidence_status: EvidenceStatus,
    pub validity_summary: String,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub enum CapabilityNodeKind {
    Calculation,
    Model,
    Solver,
    Dataset,
    Workflow,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CapabilityNode {
    pub node_id: CapabilityNodeId,
    pub codex_id: CodexId,
    pub title: String,
    pub domain: Domain,
    pub node_kind: CapabilityNodeKind,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
    pub evidence_status: EvidenceStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub enum CapabilityEdgeKind {
    RequiresOutputFrom,
    Refines,
    AlternativeTo,
    InverseOf,
    ValidatesAgainst,
    UsesDataset,
    UsesSolver,
    ProducesInputFor,
    SamePhysicalModelAs,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CapabilityEdge {
    pub from: CapabilityNodeId,
    pub to: CapabilityNodeId,
    pub edge_kind: CapabilityEdgeKind,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CapabilityGraph {
    pub nodes: Vec<CapabilityNode>,
    pub edges: Vec<CapabilityEdge>,
}
