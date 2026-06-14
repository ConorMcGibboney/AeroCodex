#![forbid(unsafe_code)]
//! Agent registry builder.

use aero_codex_agent_core::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub enum CodexEntryKind {
    Calculation,
    Model,
    Solver,
    Dataset,
    TheoryNote,
    Workflow,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AgentRegistryEntry {
    pub codex_id: CodexId,
    pub tool_name: Option<AgentToolName>,
    pub title: String,
    pub domain: Domain,
    pub crate_name: CrateName,
    pub version: SemVer,
    pub entry_kind: CodexEntryKind,
    pub evidence_status: EvidenceStatus,
    pub callable: bool,
    pub explainable: bool,
    pub verifiable: bool,
    pub input_schema_ref: Option<PathBuf>,
    pub output_schema_ref: Option<PathBuf>,
    pub cvt_refs: Vec<CvtId>,
    pub capability_node: CapabilityNodeId,
    pub deprecated: bool,
    pub replacement: Option<CodexId>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AgentRegistry {
    pub entries: Vec<AgentRegistryEntry>,
}

pub struct RegistryBuilder;

impl RegistryBuilder {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    #[must_use]
    pub fn build(&self) -> AgentRegistry {
        // TODO: collect registered Codex tools, evidence cards, schema refs, and CVTs.
        AgentRegistry {
            entries: Vec::new(),
        }
    }
}

impl Default for RegistryBuilder {
    fn default() -> Self {
        Self::new()
    }
}
