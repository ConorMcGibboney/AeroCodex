use crate::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub enum CvtKind {
    Calculation,
    Model,
    Solver,
    Dataset,
    TheoryNote,
    Workflow,
    ErrorExplanation,
    BranchRule,
    ValidityRule,
    UnitDefinition,
    FrameDefinition,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct EmbeddingRef {
    pub model_id: EmbeddingModelId,
    pub vector_path: PathBuf,
    pub dimensions: usize,
    pub hash: Sha256Hash,
    pub generated_at: Timestamp,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CvtProvenance {
    pub generated_from: Vec<String>,
    pub generator: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub enum ValidationStatus {
    Unknown,
    ReferenceModelOnly,
    ExperimentalComparison,
    Deprecated,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CompanionVectorToken {
    pub token_id: CvtId,
    pub codex_id: CodexId,
    pub version: SemVer,
    pub kind: CvtKind,
    pub domain: Domain,
    pub crate_name: CrateName,
    pub title: String,
    pub summary: String,
    pub retrieval_text: String,
    pub aliases: Vec<String>,
    pub symbols: Vec<String>,
    pub equations: Vec<EquationRef>,
    pub source_refs: Vec<SourceRef>,
    pub inputs: Vec<FieldRef>,
    pub outputs: Vec<FieldRef>,
    pub assumptions: Vec<String>,
    pub limitations: Vec<String>,
    pub validity: ValidityDomain,
    pub forbidden_when: Vec<String>,
    pub evidence_status: EvidenceStatus,
    pub validation_status: ValidationStatus,
    pub callable: bool,
    pub tool_name: Option<AgentToolName>,
    pub explainable: bool,
    pub verifiable: bool,
    pub embedding: Option<EmbeddingRef>,
    pub provenance: CvtProvenance,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct EmbeddingVector {
    pub model_id: EmbeddingModelId,
    pub dimensions: usize,
    pub values: Vec<f32>,
    pub normalized: bool,
}

pub trait EmbeddingProvider {
    fn model_id(&self) -> EmbeddingModelId;
    fn embed(&self, texts: &[String]) -> Result<Vec<EmbeddingVector>, String>;
}
