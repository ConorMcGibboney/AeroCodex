#![forbid(unsafe_code)]
//! Umbrella crate for AeroCodex foundation crates.

pub use aero_codex_aerodynamics as aerodynamics;
pub use aero_codex_astrodynamics as astrodynamics;
pub use aero_codex_atmosphere as atmosphere;
pub use aero_codex_bioregen as bioregen;
pub use aero_codex_constants as constants;
pub use aero_codex_core as core;
pub use aero_codex_data as data;
pub use aero_codex_flight_dynamics as flight_dynamics;
pub use aero_codex_gasdynamics as gasdynamics;
pub use aero_codex_heat_transfer as heat_transfer;
pub use aero_codex_numerics as numerics;
pub use aero_codex_propulsion as propulsion;
pub use aero_codex_structures as structures;
pub use aero_codex_thermo as thermo;
pub use aero_codex_units as units;
pub use aero_codex_validation as validation;

pub mod prelude {
    pub use aero_codex_core::prelude::*;
    pub use aero_codex_units::prelude::*;
}
