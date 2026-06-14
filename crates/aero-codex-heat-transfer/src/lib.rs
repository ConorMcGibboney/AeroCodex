#![forbid(unsafe_code)]
//! Heat-transfer starter calculations.

use aero_codex_constants::STEFAN_BOLTZMANN_CONSTANT;
use aero_codex_core::{AeroResult, EngineeringResult, EvidenceLevel};
use aero_codex_units::{HeatFlux, Temperature};

pub const BLACKBODY_EMISSIVE_POWER_ID: &str = "heat_transfer.radiation.blackbody_emissive_power";

/// Computes blackbody emissive power `E_b = sigma T^4`.
pub fn blackbody_emissive_power(
    temperature: Temperature,
) -> AeroResult<EngineeringResult<HeatFlux>> {
    let flux = HeatFlux::watts_per_square_meter(
        STEFAN_BOLTZMANN_CONSTANT.value * temperature.as_kelvin().powi(4),
    )?;
    Ok(EngineeringResult::new(
        flux,
        BLACKBODY_EMISSIVE_POWER_ID,
        EvidenceLevel::ImplementationVerified,
    )
    .with_assumption(
        "blackbody",
        "surface is an ideal blackbody with emissivity equal to one",
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blackbody_at_300_k_basic_case() {
        let flux = blackbody_emissive_power(Temperature::kelvin(300.0).unwrap()).unwrap();
        assert!((flux.value.as_watts_per_square_meter() - 459.300_327_939).abs() < 1.0e-9);
    }
}
