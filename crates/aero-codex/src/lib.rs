#![forbid(unsafe_code)]
//! Umbrella crate for AeroCodex foundation crates.

pub use aero_codex_constants as constants;
pub use aero_codex_core as core;
pub use aero_codex_data as data;
pub use aero_codex_gasdynamics as gasdynamics;
pub use aero_codex_numerics as numerics;
pub use aero_codex_units as units;
pub use aero_codex_validation as validation;

pub mod prelude {
    pub use aero_codex_core::prelude::*;
    pub use aero_codex_units::prelude::*;
}
