#![forbid(unsafe_code)]
//! Sourced constants used by AeroCodex foundation crates.

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SourcedConstant {
    pub value: f64,
    pub unit: &'static str,
    pub source: &'static str,
    pub codex_id: &'static str,
}

pub const STANDARD_GRAVITY: SourcedConstant = SourcedConstant {
    value: 9.806_65,
    unit: "m/s^2",
    source: "standard gravity; source record pending final review",
    codex_id: "constants.standard_gravity",
};

pub const UNIVERSAL_GAS_CONSTANT: SourcedConstant = SourcedConstant {
    value: 8.314_462_618_153_24,
    unit: "J/(mol*K)",
    source: "CODATA-style SI value; source record to be finalized before stable release",
    codex_id: "constants.universal_gas_constant",
};

pub const AIR_SPECIFIC_GAS_CONSTANT: SourcedConstant = SourcedConstant {
    value: 287.052_87,
    unit: "J/(kg*K)",
    source: "U.S. Standard Atmosphere 1976 convention; source record to be finalized",
    codex_id: "constants.air_specific_gas_constant",
};

pub const SEA_LEVEL_STANDARD_PRESSURE: SourcedConstant = SourcedConstant {
    value: 101_325.0,
    unit: "Pa",
    source: "standard atmosphere sea-level pressure; source record pending final review",
    codex_id: "constants.sea_level_standard_pressure",
};

pub const SEA_LEVEL_STANDARD_TEMPERATURE: SourcedConstant = SourcedConstant {
    value: 288.15,
    unit: "K",
    source: "standard atmosphere sea-level temperature; source record pending final review",
    codex_id: "constants.sea_level_standard_temperature",
};

pub const STEFAN_BOLTZMANN_CONSTANT: SourcedConstant = SourcedConstant {
    value: 5.670_374_419e-8,
    unit: "W/(m^2*K^4)",
    source: "SI exact constant; source record pending final review",
    codex_id: "constants.stefan_boltzmann",
};

pub const EARTH_STANDARD_GRAVITATIONAL_PARAMETER: SourcedConstant = SourcedConstant {
    value: 3.986_004_36e14,
    unit: "m^3/s^2",
    source: "JPL/NASA astrodynamic parameter source review pending",
    codex_id: "constants.earth_standard_gravitational_parameter",
};
