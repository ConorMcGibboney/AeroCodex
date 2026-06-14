#![forbid(unsafe_code)]
//! Bio-regenerative life support starter calculations.
//!
//! Phase 0.001 intentionally starts with general mass-balance equations. Biological,
//! crop-specific, mission-specific, and human-rating assumptions require source review,
//! test data, and validation before any design claim.

use aero_codex_core::{AeroError, AeroResult, EngineeringResult, EvidenceLevel};
use aero_codex_units::{Area, ArealMassRate, Mass, MassFlowRate, Ratio, Time};

pub const CLOSED_LOOP_FRACTION_ID: &str = "bioregen.mass_balance.closed_loop_fraction";
pub const REQUIRED_PRODUCTION_AREA_ID: &str = "bioregen.production.required_area";
pub const BUFFER_RESIDENCE_TIME_ID: &str = "bioregen.buffer.residence_time";

/// Computes closed-loop fraction `recovered / demand`.
pub fn closed_loop_fraction(
    recovered: MassFlowRate,
    demand: MassFlowRate,
) -> AeroResult<EngineeringResult<Ratio>> {
    let demand_value = demand.as_kilograms_per_second();
    if demand_value <= 0.0 {
        return Err(AeroError::RequiresPositive {
            parameter: "demand",
            value: demand_value,
        });
    }
    let ratio = Ratio::new(recovered.as_kilograms_per_second() / demand_value)?;
    Ok(EngineeringResult::new(
        ratio,
        CLOSED_LOOP_FRACTION_ID,
        EvidenceLevel::ImplementationVerified,
    )
    .with_assumption(
        "steady_mass_balance",
        "steady-state recovered and demand mass-flow rates",
    ))
}

/// Computes required production area from demand and areal productivity.
pub fn required_production_area(
    demand: MassFlowRate,
    areal_productivity: ArealMassRate,
) -> AeroResult<EngineeringResult<Area>> {
    let productivity = areal_productivity.as_kilograms_per_square_meter_second();
    if productivity <= 0.0 {
        return Err(AeroError::RequiresPositive {
            parameter: "areal_productivity",
            value: productivity,
        });
    }
    let area = Area::square_meters(demand.as_kilograms_per_second() / productivity)?;
    Ok(EngineeringResult::new(
        area,
        REQUIRED_PRODUCTION_AREA_ID,
        EvidenceLevel::ImplementationVerified,
    )
    .with_assumption("steady_production", "constant areal productivity and constant demand")
    .with_warning(
        "not_bioreactor_design",
        "this starter equation does not validate crop species, lighting, nutrient cycling, gas exchange, reliability, or crew health constraints",
    ))
}

/// Computes buffer residence time from inventory and consumption/production imbalance.
pub fn buffer_residence_time(
    inventory: Mass,
    net_drawdown: MassFlowRate,
) -> AeroResult<EngineeringResult<Time>> {
    let drawdown = net_drawdown.as_kilograms_per_second();
    if drawdown <= 0.0 {
        return Err(AeroError::RequiresPositive {
            parameter: "net_drawdown",
            value: drawdown,
        });
    }
    let time = Time::seconds(inventory.as_kilograms() / drawdown)?;
    Ok(EngineeringResult::new(
        time,
        BUFFER_RESIDENCE_TIME_ID,
        EvidenceLevel::ImplementationVerified,
    )
    .with_assumption(
        "well_mixed_buffer",
        "inventory is usable and net drawdown is constant",
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn closed_loop_fraction_basic_case() {
        let fraction = closed_loop_fraction(
            MassFlowRate::kilograms_per_day(0.75).unwrap(),
            MassFlowRate::kilograms_per_day(1.0).unwrap(),
        )
        .unwrap();
        assert!((fraction.value.value() - 0.75).abs() < 1.0e-12);
    }

    #[test]
    fn required_production_area_basic_case() {
        let area = required_production_area(
            MassFlowRate::kilograms_per_day(1.0).unwrap(),
            ArealMassRate::kilograms_per_square_meter_day(0.1).unwrap(),
        )
        .unwrap();
        assert!((area.value.as_square_meters() - 10.0).abs() < 1.0e-12);
    }

    #[test]
    fn buffer_residence_time_basic_case() {
        let time = buffer_residence_time(
            Mass::kilograms(2.0).unwrap(),
            MassFlowRate::kilograms_per_day(1.0).unwrap(),
        )
        .unwrap();
        assert!((time.value.as_seconds() - 172_800.0).abs() < 1.0e-9);
    }
}
