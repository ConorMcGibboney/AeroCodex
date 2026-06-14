#![forbid(unsafe_code)]
//! Atmosphere and environment starter calculations.

use aero_codex_core::{AeroError, AeroResult, EngineeringResult, EvidenceLevel};
use aero_codex_units::{Density, Pressure, Temperature};

pub const IDEAL_GAS_DENSITY_ID: &str = "atmosphere.ideal_gas.density";

/// Computes gas density from absolute pressure, temperature, and a specific gas constant.
///
/// This is the ideal-gas relation `rho = p / (R T)`. The source and domain record are
/// provisional in Phase 0.001 and must be reviewed before stable release.
pub fn ideal_gas_density(
    pressure: Pressure,
    temperature: Temperature,
    specific_gas_constant: f64,
) -> AeroResult<EngineeringResult<Density>> {
    let r = specific_gas_constant;
    if !r.is_finite() || r <= 0.0 {
        return Err(AeroError::RequiresPositive {
            parameter: "specific_gas_constant",
            value: r,
        });
    }
    let t = temperature.as_kelvin();
    if t <= 0.0 {
        return Err(AeroError::RequiresPositive {
            parameter: "temperature",
            value: t,
        });
    }

    let density = Density::kilograms_per_cubic_meter(pressure.as_pascals() / (r * t))?;
    Ok(EngineeringResult::new(
        density,
        IDEAL_GAS_DENSITY_ID,
        EvidenceLevel::ImplementationVerified,
    )
    .with_assumption("ideal_gas", "gas obeys rho = p / (R T)")
    .with_warning(
        "phase_0_001_source_review",
        "source and validation data are provisional until the Phase 0.001 source review is complete",
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sea_level_air_density_starter_case() {
        let density = ideal_gas_density(
            Pressure::pascals(101_325.0).unwrap(),
            Temperature::kelvin(288.15).unwrap(),
            287.052_87,
        )
        .unwrap();
        assert!((density.value.as_kilograms_per_cubic_meter() - 1.225).abs() < 1.0e-3);
    }
}
