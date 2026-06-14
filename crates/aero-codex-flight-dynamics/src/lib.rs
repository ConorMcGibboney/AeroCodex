#![forbid(unsafe_code)]
//! Flight mechanics starter calculations.

use aero_codex_core::{AeroError, AeroResult, EngineeringResult, EvidenceLevel};
use aero_codex_units::{Acceleration, Angle, Length, Velocity};

pub const COORDINATED_TURN_RADIUS_ID: &str = "flight_dynamics.coordinated_turn.radius";

/// Computes level coordinated-turn radius `R = V^2 / (g tan(phi))`.
pub fn coordinated_turn_radius(
    velocity: Velocity,
    bank_angle: Angle,
    gravity: Acceleration,
) -> AeroResult<EngineeringResult<Length>> {
    let tan_phi = bank_angle.tan();
    if tan_phi.abs() <= f64::EPSILON {
        return Err(AeroError::InvalidInput {
            parameter: "bank_angle",
            value: bank_angle.as_degrees(),
            reason: "bank angle must produce a nonzero tangent",
        });
    }
    let g = gravity.as_meters_per_second_squared();
    if g <= 0.0 {
        return Err(AeroError::RequiresPositive {
            parameter: "gravity",
            value: g,
        });
    }
    let radius = velocity.as_meters_per_second().powi(2) / (g * tan_phi.abs());
    let length = Length::meters(radius)?;
    Ok(EngineeringResult::new(
        length,
        COORDINATED_TURN_RADIUS_ID,
        EvidenceLevel::ImplementationVerified,
    )
    .with_assumption(
        "level_coordinated_turn",
        "constant-altitude coordinated turn with no sideslip",
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coordinated_turn_radius_basic_case() {
        let radius = coordinated_turn_radius(
            Velocity::meters_per_second(100.0).unwrap(),
            Angle::degrees(45.0).unwrap(),
            Acceleration::meters_per_second_squared(9.806_65).unwrap(),
        )
        .unwrap();
        assert!((radius.value.as_meters() - 1_019.716).abs() < 0.01);
    }
}
