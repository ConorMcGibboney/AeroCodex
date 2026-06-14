#![forbid(unsafe_code)]
//! Structures and loads starter calculations.

use aero_codex_core::{AeroError, AeroResult, EngineeringResult, EvidenceLevel};
use aero_codex_units::{Area, Force, Pressure};

pub const AXIAL_STRESS_ID: &str = "structures.stress.axial_force_over_area";

/// Computes average axial stress `sigma = F / A`.
pub fn axial_stress(force: Force, area: Area) -> AeroResult<EngineeringResult<Pressure>> {
    let area_value = area.as_square_meters();
    if area_value <= 0.0 {
        return Err(AeroError::RequiresPositive {
            parameter: "area",
            value: area_value,
        });
    }
    let stress = Pressure::pascals((force.as_newtons() / area_value).abs())?;
    Ok(EngineeringResult::new(
        stress,
        AXIAL_STRESS_ID,
        EvidenceLevel::ImplementationVerified,
    )
    .with_assumption(
        "uniform_stress",
        "average axial stress over a uniform cross-sectional area",
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn axial_stress_basic_case() {
        let stress = axial_stress(
            Force::newtons(1_000.0).unwrap(),
            Area::square_meters(0.01).unwrap(),
        )
        .unwrap();
        assert!((stress.value.as_pascals() - 100_000.0).abs() < 1.0e-10);
    }
}
