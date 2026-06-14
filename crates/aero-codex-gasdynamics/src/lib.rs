#![forbid(unsafe_code)]
//! Gas-dynamics starter calculations.

use aero_codex_core::{AeroError, AeroResult, EngineeringResult, EvidenceLevel};
use aero_codex_units::{Mach, Ratio, RatioOfSpecificHeats};

pub const NORMAL_SHOCK_P2_P1_ID: &str = "gasdyn.normal_shock.p2_over_p1";

/// Computes the normal-shock static pressure ratio `p2 / p1` for a calorically perfect gas.
///
/// The upstream Mach number must be supersonic. This founder-baseline implementation is
/// implementation-verified against a simple exact example and is not yet reference-table validated.
pub fn normal_shock_p2_over_p1(
    mach1: Mach,
    gamma: RatioOfSpecificHeats,
) -> AeroResult<EngineeringResult<Ratio>> {
    let m1 = mach1.value();
    let g = gamma.value();
    if m1 <= 1.0 {
        return Err(AeroError::RequiresSupersonic {
            parameter: "mach1",
            value: m1,
        });
    }

    let value = 1.0 + (2.0 * g / (g + 1.0)) * (m1 * m1 - 1.0);
    let ratio = Ratio::new(value)?;

    Ok(EngineeringResult::new(
        ratio,
        NORMAL_SHOCK_P2_P1_ID,
        EvidenceLevel::ImplementationVerified,
    )
    .with_assumption(
        "perfect_gas",
        "calorically perfect gas with constant ratio of specific heats",
    )
    .with_assumption("normal_shock", "steady one-dimensional normal shock"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_shock_pressure_ratio_mach_two_air() {
        let result = normal_shock_p2_over_p1(
            Mach::new(2.0).unwrap(),
            RatioOfSpecificHeats::new(1.4).unwrap(),
        )
        .unwrap();
        assert!((result.value.value() - 4.5).abs() < 1.0e-12);
        assert_eq!(result.verification.id.as_str(), NORMAL_SHOCK_P2_P1_ID);
    }

    #[test]
    fn subsonic_upstream_is_error() {
        let err = normal_shock_p2_over_p1(
            Mach::new(0.8).unwrap(),
            RatioOfSpecificHeats::new(1.4).unwrap(),
        )
        .unwrap_err();
        assert!(matches!(err, AeroError::RequiresSupersonic { .. }));
    }
}
