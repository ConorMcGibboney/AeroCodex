#![forbid(unsafe_code)]
//! Aerodynamics starter calculations.

use aero_codex_core::{AeroResult, EngineeringResult, EvidenceLevel};
use aero_codex_units::{Density, Pressure, Velocity};

pub const DYNAMIC_PRESSURE_ID: &str = "aerodynamics.dynamic_pressure";

/// Computes dynamic pressure `q = 0.5 rho V^2`.
pub fn dynamic_pressure(
    density: Density,
    velocity: Velocity,
) -> AeroResult<EngineeringResult<Pressure>> {
    let value =
        0.5 * density.as_kilograms_per_cubic_meter() * velocity.as_meters_per_second().powi(2);
    let pressure = Pressure::pascals(value)?;
    Ok(EngineeringResult::new(
        pressure,
        DYNAMIC_PRESSURE_ID,
        EvidenceLevel::ImplementationVerified,
    )
    .with_assumption(
        "continuum_flow",
        "density and velocity describe the same flow state",
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dynamic_pressure_basic_case() {
        let q = dynamic_pressure(
            Density::kilograms_per_cubic_meter(1.225).unwrap(),
            Velocity::meters_per_second(100.0).unwrap(),
        )
        .unwrap();
        assert!((q.value.as_pascals() - 6_125.0).abs() < 1.0e-10);
    }
}
