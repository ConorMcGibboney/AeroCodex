#![forbid(unsafe_code)]
//! Celestial mechanics and astrodynamics starter calculations.

use aero_codex_core::{AeroError, AeroResult, EngineeringResult, EvidenceLevel};
use aero_codex_units::{GravitationalParameter, Length, Velocity};

pub const CIRCULAR_ORBIT_SPEED_ID: &str = "astrodynamics.two_body.circular_orbit_speed";
pub const VIS_VIVA_SPEED_ID: &str = "astrodynamics.two_body.vis_viva_speed";

/// Computes circular two-body orbital speed `v = sqrt(mu / r)`.
pub fn circular_orbit_speed(
    mu: GravitationalParameter,
    radius: Length,
) -> AeroResult<EngineeringResult<Velocity>> {
    let r = radius.as_meters();
    if r <= 0.0 {
        return Err(AeroError::RequiresPositive {
            parameter: "radius",
            value: r,
        });
    }
    let velocity =
        Velocity::meters_per_second((mu.as_cubic_meters_per_square_second() / r).sqrt())?;
    Ok(EngineeringResult::new(
        velocity,
        CIRCULAR_ORBIT_SPEED_ID,
        EvidenceLevel::ImplementationVerified,
    )
    .with_assumption(
        "two_body",
        "point-mass central-body gravity with no perturbations",
    ))
}

/// Computes two-body speed from the vis-viva equation `v = sqrt(mu * (2/r - 1/a))`.
pub fn vis_viva_speed(
    mu: GravitationalParameter,
    radius: Length,
    semi_major_axis: Length,
) -> AeroResult<EngineeringResult<Velocity>> {
    let r = radius.as_meters();
    let a = semi_major_axis.as_meters();
    if r <= 0.0 {
        return Err(AeroError::RequiresPositive {
            parameter: "radius",
            value: r,
        });
    }
    if a <= 0.0 {
        return Err(AeroError::RequiresPositive {
            parameter: "semi_major_axis",
            value: a,
        });
    }
    let term = mu.as_cubic_meters_per_square_second() * (2.0 / r - 1.0 / a);
    if term < 0.0 {
        return Err(AeroError::NonPhysicalState {
            reason: "vis-viva radicand is negative for the supplied radius and semi-major axis",
        });
    }
    let velocity = Velocity::meters_per_second(term.sqrt())?;
    Ok(EngineeringResult::new(
        velocity,
        VIS_VIVA_SPEED_ID,
        EvidenceLevel::ImplementationVerified,
    )
    .with_assumption(
        "two_body",
        "Keplerian two-body motion with no perturbations",
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn circular_low_earth_orbit_speed_basic_case() {
        let v = circular_orbit_speed(
            GravitationalParameter::cubic_meters_per_square_second(3.986_004_36e14).unwrap(),
            Length::meters(6_778_000.0).unwrap(),
        )
        .unwrap();
        assert!((v.value.as_meters_per_second() - 7_668.64).abs() < 2.0);
    }

    #[test]
    fn vis_viva_equals_circular_when_a_equals_r() {
        let mu = GravitationalParameter::cubic_meters_per_square_second(3.986_004_36e14).unwrap();
        let r = Length::meters(7_000_000.0).unwrap();
        let v1 = circular_orbit_speed(mu, r)
            .unwrap()
            .value
            .as_meters_per_second();
        let v2 = vis_viva_speed(mu, r, r)
            .unwrap()
            .value
            .as_meters_per_second();
        assert!((v1 - v2).abs() < 1.0e-10);
    }
}
