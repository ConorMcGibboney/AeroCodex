#![forbid(unsafe_code)]
//! Propulsion starter calculations.

use aero_codex_core::{AeroError, AeroResult, EngineeringResult, EvidenceLevel};
use aero_codex_units::{Acceleration, Mass, Time, Velocity};

pub const IDEAL_ROCKET_DELTA_V_ID: &str = "propulsion.rocket.ideal_delta_v";

/// Computes ideal rocket delta-v `Delta v = Isp g0 ln(m0 / mf)`.
pub fn ideal_rocket_delta_v(
    specific_impulse: Time,
    standard_gravity: Acceleration,
    initial_mass: Mass,
    final_mass: Mass,
) -> AeroResult<EngineeringResult<Velocity>> {
    let isp = specific_impulse.as_seconds();
    let g0 = standard_gravity.as_meters_per_second_squared();
    let m0 = initial_mass.as_kilograms();
    let mf = final_mass.as_kilograms();
    if isp <= 0.0 {
        return Err(AeroError::RequiresPositive {
            parameter: "specific_impulse",
            value: isp,
        });
    }
    if g0 <= 0.0 {
        return Err(AeroError::RequiresPositive {
            parameter: "standard_gravity",
            value: g0,
        });
    }
    if mf <= 0.0 || m0 <= mf {
        return Err(AeroError::InvalidInput {
            parameter: "mass_ratio",
            value: if mf > 0.0 { m0 / mf } else { f64::NAN },
            reason: "initial mass must be greater than final mass and final mass must be positive",
        });
    }

    let velocity = Velocity::meters_per_second(isp * g0 * (m0 / mf).ln())?;
    Ok(EngineeringResult::new(
        velocity,
        IDEAL_ROCKET_DELTA_V_ID,
        EvidenceLevel::ImplementationVerified,
    )
    .with_assumption(
        "ideal_rocket",
        "impulsive ideal rocket equation with constant specific impulse",
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ideal_rocket_delta_v_basic_case() {
        let dv = ideal_rocket_delta_v(
            Time::seconds(300.0).unwrap(),
            Acceleration::meters_per_second_squared(9.806_65).unwrap(),
            Mass::kilograms(10_000.0).unwrap(),
            Mass::kilograms(5_000.0).unwrap(),
        )
        .unwrap();
        assert!((dv.value.as_meters_per_second() - 2_039.155).abs() < 1.0);
    }
}
