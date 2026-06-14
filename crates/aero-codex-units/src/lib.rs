#![forbid(unsafe_code)]
//! Typed quantities and strong dimensionless wrappers for AeroCodex public APIs.

use aero_codex_core::{AeroError, AeroResult};

fn finite(parameter: &'static str, value: f64) -> AeroResult<f64> {
    if value.is_finite() {
        Ok(value)
    } else {
        Err(AeroError::InvalidInput {
            parameter,
            value,
            reason: "value must be finite",
        })
    }
}

fn non_negative(parameter: &'static str, value: f64) -> AeroResult<f64> {
    let value = finite(parameter, value)?;
    if value < 0.0 {
        Err(AeroError::InvalidInput {
            parameter,
            value,
            reason: "value cannot be negative",
        })
    } else {
        Ok(value)
    }
}

fn positive(parameter: &'static str, value: f64) -> AeroResult<f64> {
    let value = finite(parameter, value)?;
    if value <= 0.0 {
        Err(AeroError::RequiresPositive { parameter, value })
    } else {
        Ok(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Angle {
    radians: f64,
}

impl Angle {
    pub fn radians(value: f64) -> AeroResult<Self> {
        Ok(Self {
            radians: finite("angle", value)?,
        })
    }

    pub fn degrees(value: f64) -> AeroResult<Self> {
        Ok(Self {
            radians: finite("angle", value)?.to_radians(),
        })
    }

    #[must_use]
    pub fn as_radians(self) -> f64 {
        self.radians
    }

    #[must_use]
    pub fn as_degrees(self) -> f64 {
        self.radians.to_degrees()
    }

    #[must_use]
    pub fn tan(self) -> f64 {
        self.radians.tan()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mach(f64);

impl Mach {
    pub fn new(value: f64) -> AeroResult<Self> {
        Ok(Self(non_negative("mach", value)?))
    }

    #[must_use]
    pub fn value(self) -> f64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RatioOfSpecificHeats(f64);

impl RatioOfSpecificHeats {
    pub fn new(value: f64) -> AeroResult<Self> {
        let value = finite("gamma", value)?;
        if value <= 1.0 {
            return Err(AeroError::InvalidInput {
                parameter: "gamma",
                value,
                reason: "ratio of specific heats must be greater than one",
            });
        }
        Ok(Self(value))
    }

    #[must_use]
    pub fn value(self) -> f64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ratio(f64);

impl Ratio {
    pub fn new(value: f64) -> AeroResult<Self> {
        Ok(Self(finite("ratio", value)?))
    }

    pub fn fraction(value: f64) -> AeroResult<Self> {
        let value = finite("ratio", value)?;
        if !(0.0..=1.0).contains(&value) {
            return Err(AeroError::InvalidInput {
                parameter: "ratio",
                value,
                reason: "fractional ratio must be between zero and one, inclusive",
            });
        }
        Ok(Self(value))
    }

    #[must_use]
    pub fn value(self) -> f64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Length {
    meters: f64,
}

impl Length {
    pub fn meters(value: f64) -> AeroResult<Self> {
        Ok(Self {
            meters: finite("length", value)?,
        })
    }

    pub fn positive_meters(value: f64) -> AeroResult<Self> {
        Ok(Self {
            meters: positive("length", value)?,
        })
    }

    pub fn kilometers(value: f64) -> AeroResult<Self> {
        Self::meters(finite("length", value)? * 1_000.0)
    }

    pub fn feet(value: f64) -> AeroResult<Self> {
        Self::meters(finite("length", value)? * 0.3048)
    }

    #[must_use]
    pub fn as_meters(self) -> f64 {
        self.meters
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Area {
    square_meters: f64,
}

impl Area {
    pub fn square_meters(value: f64) -> AeroResult<Self> {
        Ok(Self {
            square_meters: non_negative("area", value)?,
        })
    }

    pub fn positive_square_meters(value: f64) -> AeroResult<Self> {
        Ok(Self {
            square_meters: positive("area", value)?,
        })
    }

    pub fn square_feet(value: f64) -> AeroResult<Self> {
        Self::square_meters(finite("area", value)? * 0.092_903_04)
    }

    #[must_use]
    pub fn as_square_meters(self) -> f64 {
        self.square_meters
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pressure {
    pascals: f64,
}

impl Pressure {
    pub fn pascals(value: f64) -> AeroResult<Self> {
        Ok(Self {
            pascals: non_negative("pressure", value)?,
        })
    }

    pub fn kilopascals(value: f64) -> AeroResult<Self> {
        Self::pascals(finite("pressure", value)? * 1_000.0)
    }

    #[must_use]
    pub fn as_pascals(self) -> f64 {
        self.pascals
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Temperature {
    kelvin: f64,
}

impl Temperature {
    pub fn kelvin(value: f64) -> AeroResult<Self> {
        Ok(Self {
            kelvin: non_negative("temperature", value)?,
        })
    }

    pub fn positive_kelvin(value: f64) -> AeroResult<Self> {
        Ok(Self {
            kelvin: positive("temperature", value)?,
        })
    }

    #[must_use]
    pub fn as_kelvin(self) -> f64 {
        self.kelvin
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Density {
    kilograms_per_cubic_meter: f64,
}

impl Density {
    pub fn kilograms_per_cubic_meter(value: f64) -> AeroResult<Self> {
        Ok(Self {
            kilograms_per_cubic_meter: non_negative("density", value)?,
        })
    }

    #[must_use]
    pub fn as_kilograms_per_cubic_meter(self) -> f64 {
        self.kilograms_per_cubic_meter
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Velocity {
    meters_per_second: f64,
}

impl Velocity {
    pub fn meters_per_second(value: f64) -> AeroResult<Self> {
        Ok(Self {
            meters_per_second: finite("velocity", value)?,
        })
    }

    pub fn knots(value: f64) -> AeroResult<Self> {
        Self::meters_per_second(finite("velocity", value)? * 0.514_444_444_444_444_5)
    }

    #[must_use]
    pub fn as_meters_per_second(self) -> f64 {
        self.meters_per_second
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Acceleration {
    meters_per_second_squared: f64,
}

impl Acceleration {
    pub fn meters_per_second_squared(value: f64) -> AeroResult<Self> {
        Ok(Self {
            meters_per_second_squared: finite("acceleration", value)?,
        })
    }

    pub fn positive_meters_per_second_squared(value: f64) -> AeroResult<Self> {
        Ok(Self {
            meters_per_second_squared: positive("acceleration", value)?,
        })
    }

    #[must_use]
    pub fn as_meters_per_second_squared(self) -> f64 {
        self.meters_per_second_squared
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mass {
    kilograms: f64,
}

impl Mass {
    pub fn kilograms(value: f64) -> AeroResult<Self> {
        Ok(Self {
            kilograms: non_negative("mass", value)?,
        })
    }

    pub fn positive_kilograms(value: f64) -> AeroResult<Self> {
        Ok(Self {
            kilograms: positive("mass", value)?,
        })
    }

    #[must_use]
    pub fn as_kilograms(self) -> f64 {
        self.kilograms
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Time {
    seconds: f64,
}

impl Time {
    pub fn seconds(value: f64) -> AeroResult<Self> {
        Ok(Self {
            seconds: non_negative("time", value)?,
        })
    }

    pub fn positive_seconds(value: f64) -> AeroResult<Self> {
        Ok(Self {
            seconds: positive("time", value)?,
        })
    }

    pub fn days(value: f64) -> AeroResult<Self> {
        Self::seconds(finite("time", value)? * 86_400.0)
    }

    #[must_use]
    pub fn as_seconds(self) -> f64 {
        self.seconds
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MassFlowRate {
    kilograms_per_second: f64,
}

impl MassFlowRate {
    pub fn kilograms_per_second(value: f64) -> AeroResult<Self> {
        Ok(Self {
            kilograms_per_second: non_negative("mass_flow_rate", value)?,
        })
    }

    pub fn positive_kilograms_per_second(value: f64) -> AeroResult<Self> {
        Ok(Self {
            kilograms_per_second: positive("mass_flow_rate", value)?,
        })
    }

    pub fn kilograms_per_day(value: f64) -> AeroResult<Self> {
        Self::kilograms_per_second(finite("mass_flow_rate", value)? / 86_400.0)
    }

    #[must_use]
    pub fn as_kilograms_per_second(self) -> f64 {
        self.kilograms_per_second
    }

    #[must_use]
    pub fn as_kilograms_per_day(self) -> f64 {
        self.kilograms_per_second * 86_400.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ArealMassRate {
    kilograms_per_square_meter_second: f64,
}

impl ArealMassRate {
    pub fn kilograms_per_square_meter_second(value: f64) -> AeroResult<Self> {
        Ok(Self {
            kilograms_per_square_meter_second: non_negative("areal_mass_rate", value)?,
        })
    }

    pub fn positive_kilograms_per_square_meter_second(value: f64) -> AeroResult<Self> {
        Ok(Self {
            kilograms_per_square_meter_second: positive("areal_mass_rate", value)?,
        })
    }

    pub fn kilograms_per_square_meter_day(value: f64) -> AeroResult<Self> {
        Self::kilograms_per_square_meter_second(finite("areal_mass_rate", value)? / 86_400.0)
    }

    #[must_use]
    pub fn as_kilograms_per_square_meter_second(self) -> f64 {
        self.kilograms_per_square_meter_second
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Force {
    newtons: f64,
}

impl Force {
    pub fn newtons(value: f64) -> AeroResult<Self> {
        Ok(Self {
            newtons: finite("force", value)?,
        })
    }

    #[must_use]
    pub fn as_newtons(self) -> f64 {
        self.newtons
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HeatFlux {
    watts_per_square_meter: f64,
}

impl HeatFlux {
    pub fn watts_per_square_meter(value: f64) -> AeroResult<Self> {
        Ok(Self {
            watts_per_square_meter: finite("heat_flux", value)?,
        })
    }

    #[must_use]
    pub fn as_watts_per_square_meter(self) -> f64 {
        self.watts_per_square_meter
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GravitationalParameter {
    cubic_meters_per_square_second: f64,
}

impl GravitationalParameter {
    pub fn cubic_meters_per_square_second(value: f64) -> AeroResult<Self> {
        Ok(Self {
            cubic_meters_per_square_second: positive("gravitational_parameter", value)?,
        })
    }

    pub fn cubic_kilometers_per_square_second(value: f64) -> AeroResult<Self> {
        Self::cubic_meters_per_square_second(finite("gravitational_parameter", value)? * 1.0e9)
    }

    #[must_use]
    pub fn as_cubic_meters_per_square_second(self) -> f64 {
        self.cubic_meters_per_square_second
    }
}

pub mod prelude {
    pub use crate::{
        Acceleration, Angle, Area, ArealMassRate, Density, Force, GravitationalParameter, HeatFlux,
        Length, Mach, Mass, MassFlowRate, Pressure, Ratio, RatioOfSpecificHeats, Temperature, Time,
        Velocity,
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn angle_converts_degrees_to_radians() {
        let angle = Angle::degrees(180.0).unwrap();
        assert!((angle.as_radians() - std::f64::consts::PI).abs() < 1.0e-12);
    }

    #[test]
    fn rejects_negative_mach() {
        assert!(Mach::new(-0.1).is_err());
    }

    #[test]
    fn mass_flow_converts_days_to_seconds() {
        let rate = MassFlowRate::kilograms_per_day(86_400.0).unwrap();
        assert!((rate.as_kilograms_per_second() - 1.0).abs() < 1.0e-12);
    }
}
