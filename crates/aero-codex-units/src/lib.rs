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
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mach(f64);

impl Mach {
    pub fn new(value: f64) -> AeroResult<Self> {
        let value = finite("mach", value)?;
        if value < 0.0 {
            return Err(AeroError::InvalidInput {
                parameter: "mach",
                value,
                reason: "Mach number cannot be negative",
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
pub struct Pressure {
    pascals: f64,
}

impl Pressure {
    pub fn pascals(value: f64) -> AeroResult<Self> {
        let value = finite("pressure", value)?;
        if value < 0.0 {
            return Err(AeroError::InvalidInput {
                parameter: "pressure",
                value,
                reason: "absolute pressure cannot be negative",
            });
        }
        Ok(Self { pascals: value })
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
        let value = finite("temperature", value)?;
        if value < 0.0 {
            return Err(AeroError::InvalidInput {
                parameter: "temperature",
                value,
                reason: "absolute temperature cannot be negative",
            });
        }
        Ok(Self { kelvin: value })
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
        let value = finite("density", value)?;
        if value < 0.0 {
            return Err(AeroError::InvalidInput {
                parameter: "density",
                value,
                reason: "density cannot be negative",
            });
        }
        Ok(Self {
            kilograms_per_cubic_meter: value,
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

pub mod prelude {
    pub use crate::{
        Angle, Density, Length, Mach, Pressure, Ratio, RatioOfSpecificHeats, Temperature, Velocity,
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
}
