#![forbid(unsafe_code)]
//! Agent-facing core types for AeroCodex.
//!
//! This crate should stay small, serializable, and free of domain crate dependencies.

pub mod cvt;
pub mod error;
pub mod ids;
pub mod retrieval;
pub mod safety;
pub mod tool;
pub mod trace;

pub use cvt::*;
pub use error::*;
pub use ids::*;
pub use retrieval::*;
pub use safety::*;
pub use tool::*;
pub use trace::*;
