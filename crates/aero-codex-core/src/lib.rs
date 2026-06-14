#![forbid(unsafe_code)]
//! Core result, evidence, validity, and error types for AeroCodex.

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VerificationId(&'static str);

impl VerificationId {
    #[must_use]
    pub const fn new(value: &'static str) -> Self {
        Self(value)
    }

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

impl fmt::Display for VerificationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvidenceLevel {
    Proposed,
    EquationTraceable,
    ImplementationVerified,
    ReferenceValidated,
    ExperimentValidated,
    DeprecatedOrWithdrawn,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidityStatus {
    NotAssessed,
    WithinDeclaredDomain,
    ReferenceValidated,
    ExperimentValidated,
    OutsideValidityRange,
    InvalidInput,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Assumption {
    pub id: &'static str,
    pub text: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelWarning {
    pub id: &'static str,
    pub text: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Uncertainty {
    pub standard_uncertainty: f64,
    pub coverage_factor: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerificationRecord {
    pub id: VerificationId,
    pub evidence_level: EvidenceLevel,
    pub report_id: &'static str,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EngineeringResult<T> {
    pub value: T,
    pub assumptions: Vec<Assumption>,
    pub warnings: Vec<ModelWarning>,
    pub validity: ValidityStatus,
    pub verification: VerificationRecord,
    pub uncertainty: Option<Uncertainty>,
}

impl<T> EngineeringResult<T> {
    #[must_use]
    pub fn new(value: T, verification_id: &'static str, evidence_level: EvidenceLevel) -> Self {
        Self {
            value,
            assumptions: Vec::new(),
            warnings: Vec::new(),
            validity: ValidityStatus::WithinDeclaredDomain,
            verification: VerificationRecord {
                id: VerificationId::new(verification_id),
                evidence_level,
                report_id: "founder-baseline-v0.0.0",
            },
            uncertainty: None,
        }
    }

    #[must_use]
    pub fn with_assumption(mut self, id: &'static str, text: &'static str) -> Self {
        self.assumptions.push(Assumption { id, text });
        self
    }

    #[must_use]
    pub fn with_warning(mut self, id: &'static str, text: &'static str) -> Self {
        self.warnings.push(ModelWarning { id, text });
        self
    }

    #[must_use]
    pub fn with_validity(mut self, validity: ValidityStatus) -> Self {
        self.validity = validity;
        self
    }

    #[must_use]
    pub fn with_uncertainty(mut self, uncertainty: Uncertainty) -> Self {
        self.uncertainty = Some(uncertainty);
        self
    }
}

pub type AeroResult<T> = Result<T, AeroError>;

#[derive(Debug, Clone, PartialEq)]
pub enum AeroError {
    InvalidInput {
        parameter: &'static str,
        value: f64,
        reason: &'static str,
    },
    RequiresSupersonic {
        parameter: &'static str,
        value: f64,
    },
    RequiresSubsonic {
        parameter: &'static str,
        value: f64,
    },
    OutsideValidityRange {
        model: &'static str,
        variable: &'static str,
        value: f64,
    },
    AmbiguousBranch {
        model: &'static str,
        valid_branches: &'static [&'static str],
    },
    NoConvergence {
        solver: &'static str,
        iterations: usize,
        residual: f64,
    },
    NonPhysicalState {
        reason: &'static str,
    },
    MissingEvidenceCard {
        id: VerificationId,
    },
}

impl fmt::Display for AeroError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidInput {
                parameter,
                value,
                reason,
            } => write!(f, "invalid input {parameter}={value}: {reason}"),
            Self::RequiresSupersonic { parameter, value } => {
                write!(f, "{parameter}={value} must be supersonic")
            }
            Self::RequiresSubsonic { parameter, value } => {
                write!(f, "{parameter}={value} must be subsonic")
            }
            Self::OutsideValidityRange {
                model,
                variable,
                value,
            } => write!(
                f,
                "{model} is outside validity range for {variable}={value}"
            ),
            Self::AmbiguousBranch {
                model,
                valid_branches,
            } => write!(
                f,
                "{model} requires an explicit branch from {valid_branches:?}"
            ),
            Self::NoConvergence {
                solver,
                iterations,
                residual,
            } => write!(
                f,
                "{solver} did not converge after {iterations} iterations; residual={residual}"
            ),
            Self::NonPhysicalState { reason } => write!(f, "non-physical state: {reason}"),
            Self::MissingEvidenceCard { id } => write!(f, "missing evidence card for {id}"),
        }
    }
}

impl std::error::Error for AeroError {}

pub mod prelude {
    pub use crate::{
        AeroError, AeroResult, Assumption, EngineeringResult, EvidenceLevel, ModelWarning,
        Uncertainty, ValidityStatus, VerificationId, VerificationRecord,
    };
}
