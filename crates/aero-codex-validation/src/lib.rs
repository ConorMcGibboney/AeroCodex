#![forbid(unsafe_code)]
//! Validation-card primitives for AeroCodex.

use aero_codex_core::EvidenceLevel;

pub const CODEX_CARD_SCHEMA_VERSION: &str = "0.1.0";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReleaseStatus {
    Experimental,
    Beta,
    Stable,
    Deprecated,
    Withdrawn,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodexCardSummary {
    pub id: &'static str,
    pub name: &'static str,
    pub crate_name: &'static str,
    pub evidence_level: EvidenceLevel,
    pub release_status: ReleaseStatus,
}

#[must_use]
pub fn founder_cards() -> &'static [CodexCardSummary] {
    &[CodexCardSummary {
        id: "gasdyn.normal_shock.p2_over_p1",
        name: "Normal shock static pressure ratio",
        crate_name: "aero-codex-gasdynamics",
        evidence_level: EvidenceLevel::ImplementationVerified,
        release_status: ReleaseStatus::Experimental,
    }]
}
