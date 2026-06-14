#![forbid(unsafe_code)]
//! Validation-card primitives for AeroCodex.

use aero_codex_core::EvidenceLevel;

pub const CODEX_CARD_SCHEMA_VERSION: &str = "0.1.0";
pub const PROJECT_PHASE_VERSION: &str = "0.001";

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
pub fn phase_0_001_cards() -> &'static [CodexCardSummary] {
    &[
        CodexCardSummary {
            id: "atmosphere.ideal_gas.density",
            name: "Atmosphere ideal gas density",
            crate_name: "aero-codex-atmosphere",
            evidence_level: EvidenceLevel::ImplementationVerified,
            release_status: ReleaseStatus::Experimental,
        },
        CodexCardSummary {
            id: "thermo.perfect_gas.speed_of_sound",
            name: "Perfect gas speed of sound",
            crate_name: "aero-codex-thermo",
            evidence_level: EvidenceLevel::ImplementationVerified,
            release_status: ReleaseStatus::Experimental,
        },
        CodexCardSummary {
            id: "gasdyn.normal_shock.p2_over_p1",
            name: "Normal shock static pressure ratio",
            crate_name: "aero-codex-gasdynamics",
            evidence_level: EvidenceLevel::ImplementationVerified,
            release_status: ReleaseStatus::Experimental,
        },
        CodexCardSummary {
            id: "gasdyn.isentropic.p0_over_p",
            name: "Isentropic total-to-static pressure ratio",
            crate_name: "aero-codex-gasdynamics",
            evidence_level: EvidenceLevel::ImplementationVerified,
            release_status: ReleaseStatus::Experimental,
        },
        CodexCardSummary {
            id: "aerodynamics.dynamic_pressure",
            name: "Dynamic pressure",
            crate_name: "aero-codex-aerodynamics",
            evidence_level: EvidenceLevel::ImplementationVerified,
            release_status: ReleaseStatus::Experimental,
        },
        CodexCardSummary {
            id: "propulsion.rocket.ideal_delta_v",
            name: "Ideal rocket delta-v",
            crate_name: "aero-codex-propulsion",
            evidence_level: EvidenceLevel::ImplementationVerified,
            release_status: ReleaseStatus::Experimental,
        },
        CodexCardSummary {
            id: "heat_transfer.radiation.blackbody_emissive_power",
            name: "Blackbody emissive power",
            crate_name: "aero-codex-heat-transfer",
            evidence_level: EvidenceLevel::ImplementationVerified,
            release_status: ReleaseStatus::Experimental,
        },
        CodexCardSummary {
            id: "structures.stress.axial_force_over_area",
            name: "Average axial stress",
            crate_name: "aero-codex-structures",
            evidence_level: EvidenceLevel::ImplementationVerified,
            release_status: ReleaseStatus::Experimental,
        },
        CodexCardSummary {
            id: "flight_dynamics.coordinated_turn.radius",
            name: "Coordinated turn radius",
            crate_name: "aero-codex-flight-dynamics",
            evidence_level: EvidenceLevel::ImplementationVerified,
            release_status: ReleaseStatus::Experimental,
        },
        CodexCardSummary {
            id: "astrodynamics.two_body.circular_orbit_speed",
            name: "Circular orbit speed",
            crate_name: "aero-codex-astrodynamics",
            evidence_level: EvidenceLevel::ImplementationVerified,
            release_status: ReleaseStatus::Experimental,
        },
        CodexCardSummary {
            id: "astrodynamics.two_body.vis_viva_speed",
            name: "Vis-viva orbital speed",
            crate_name: "aero-codex-astrodynamics",
            evidence_level: EvidenceLevel::ImplementationVerified,
            release_status: ReleaseStatus::Experimental,
        },
        CodexCardSummary {
            id: "bioregen.mass_balance.closed_loop_fraction",
            name: "Closed-loop mass fraction",
            crate_name: "aero-codex-bioregen",
            evidence_level: EvidenceLevel::ImplementationVerified,
            release_status: ReleaseStatus::Experimental,
        },
        CodexCardSummary {
            id: "bioregen.production.required_area",
            name: "Required bio-regenerative production area",
            crate_name: "aero-codex-bioregen",
            evidence_level: EvidenceLevel::ImplementationVerified,
            release_status: ReleaseStatus::Experimental,
        },
        CodexCardSummary {
            id: "bioregen.buffer.residence_time",
            name: "Life-support buffer residence time",
            crate_name: "aero-codex-bioregen",
            evidence_level: EvidenceLevel::ImplementationVerified,
            release_status: ReleaseStatus::Experimental,
        },
    ]
}

#[must_use]
pub fn founder_cards() -> &'static [CodexCardSummary] {
    phase_0_001_cards()
}
