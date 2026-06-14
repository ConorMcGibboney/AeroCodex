#![forbid(unsafe_code)]
//! Thermodynamics starter calculations.

use aero_codex_core::{AeroError, AeroResult, EngineeringResult, EvidenceLevel};
use aero_codex_units::{RatioOfSpecificHeats, Temperature, Velocity};

pub const PERFECT_GAS_SPEED_OF_SOUND_ID: &str = "thermo.perfect_gas.speed_of_sound";

/// Computes perfect-gas speed of sound `a = sqrt(gamma R T)`.
pub fn perfect_gas_speed_of_sound(
    temperature: Temperature,
    gamma: RatioOfSpecificHeats,
    specific_gas_constant: f64,
) -> AeroResult<EngineeringResult<Velocity>> {
    let r = specific_gas_constant;
    if !r.is_finite() || r <= 0.0 {
        return Err(AeroError::RequiresPositive {
            parameter: "specific_gas_constant",
            value: r,
        });
    }
    let value = (gamma.value() * r * temperature.as_kelvin()).sqrt();
    let velocity = Velocity::meters_per_second(value)?;
    Ok(EngineeringResult::new(
        velocity,
        PERFECT_GAS_SPEED_OF_SOUND_ID,
        EvidenceLevel::ImplementationVerified,
    )
    .with_assumption(
        "perfect_gas",
        "gas is calorically perfect with constant gamma and R",
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sea_level_air_speed_of_sound_starter_case() {
        let a = perfect_gas_speed_of_sound(
            Temperature::kelvin(288.15).unwrap(),
            RatioOfSpecificHeats::new(1.4).unwrap(),
            287.052_87,
        )
        .unwrap();
        assert!((a.value.as_meters_per_second() - 340.294).abs() < 1.0e-3);
    }
}
