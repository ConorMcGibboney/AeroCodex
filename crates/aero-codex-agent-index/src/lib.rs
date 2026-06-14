#![forbid(unsafe_code)]
//! Companion Vector Token generator.

use aero_codex_agent_core::*;

pub struct CvtGenerator;

impl CvtGenerator {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    #[must_use]
    pub fn generate_for_tool(&self, spec: &AgentToolSpec) -> CompanionVectorToken {
        CompanionVectorToken {
            token_id: CvtId(format!("cvt:{}:{}", spec.codex_id.0, spec.version.0)),
            codex_id: spec.codex_id.clone(),
            version: spec.version.clone(),
            kind: CvtKind::Calculation,
            domain: Domain("unknown".to_owned()),
            crate_name: spec.crate_name.clone(),
            title: spec.title.clone(),
            summary: spec.description.clone(),
            retrieval_text: spec.description.clone(),
            aliases: Vec::new(),
            symbols: Vec::new(),
            equations: Vec::new(),
            source_refs: Vec::new(),
            inputs: Vec::new(),
            outputs: Vec::new(),
            assumptions: spec.assumptions.clone(),
            limitations: spec.limitations.clone(),
            validity: spec.validity.clone(),
            forbidden_when: Vec::new(),
            evidence_status: spec.evidence.status.clone(),
            validation_status: ValidationStatus::Unknown,
            callable: spec.mode_support.compute,
            tool_name: Some(spec.name.clone()),
            explainable: spec.mode_support.explain,
            verifiable: spec.mode_support.verify,
            embedding: None,
            provenance: CvtProvenance {
                generated_from: vec!["AgentToolSpec".to_owned()],
                generator: "aero-codex-agent-index".to_owned(),
            },
        }
    }
}

impl Default for CvtGenerator {
    fn default() -> Self {
        Self::new()
    }
}
