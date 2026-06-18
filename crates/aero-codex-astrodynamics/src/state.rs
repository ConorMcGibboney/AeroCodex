//! Cartesian state containers and validation for the astrodynamics foundation.
//!
//! Units are explicit and SI-based:
//! - position components: meters (`m`);
//! - velocity components: meters per second (`m/s`);
//! - gravitational parameter: cubic meters per square second (`m^3/s^2`);
//!
//! This module stores labels from `time.rs` and `frames.rs`; it does not perform
//! time-scale conversion, frame conversion, Earth orientation, propagation,
//! perturbation modeling, satellite-tracking propagation, or mission-grade orbit
//! determination.
//!
//! Finite Cartesian-state validation is intentionally separate from
//! orbital-use validation. A finite zero-position vector can be represented as
//! a synthetic state record, but it is rejected by functions that require a
//! nonzero orbital radius.

use crate::frames::AstroFrame;
use crate::time::{AstroEpoch, AstroTimeScale};
use core::fmt;

fn position_component_names() -> [&'static str; 3] {
    ["position_m[0]", "position_m[1]", "position_m[2]"]
}

fn velocity_component_names() -> [&'static str; 3] {
    [
        "velocity_m_per_s[0]",
        "velocity_m_per_s[1]",
        "velocity_m_per_s[2]",
    ]
}

/// Result alias for Cartesian-state validation and state-derived helpers.
pub type AstroStateResult<T> = Result<T, AstroStateValidationError>;

/// Position and velocity vectors in SI units.
///
/// This is a finite data container only. It carries no frame, epoch, gravity
/// model, orbit-classification, or propagation semantics.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PositionVelocity {
    position_m: [f64; 3],
    velocity_m_per_s: [f64; 3],
}

impl PositionVelocity {
    /// Creates a finite position/velocity pair.
    ///
    /// Units: `position_m` in meters; `velocity_m_per_s` in meters per second.
    /// Invalid input: any NaN or infinite component.
    pub fn new(position_m: [f64; 3], velocity_m_per_s: [f64; 3]) -> AstroStateResult<Self> {
        validate_position_velocity_components(position_m, velocity_m_per_s)?;
        Ok(Self {
            position_m,
            velocity_m_per_s,
        })
    }

    /// Returns position components in meters.
    #[must_use]
    pub const fn position_m(self) -> [f64; 3] {
        self.position_m
    }

    /// Returns velocity components in meters per second.
    #[must_use]
    pub const fn velocity_m_per_s(self) -> [f64; 3] {
        self.velocity_m_per_s
    }
}

/// Bare Cartesian state without a frame or epoch label.
///
/// Prefer `StateWithFrameAndEpoch` at API boundaries where silent frame or
/// time-scale mixing could occur.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CartesianState {
    position_velocity: PositionVelocity,
}

impl CartesianState {
    /// Creates a finite Cartesian state from SI position and velocity vectors.
    pub fn new(position_m: [f64; 3], velocity_m_per_s: [f64; 3]) -> AstroStateResult<Self> {
        Ok(Self {
            position_velocity: PositionVelocity::new(position_m, velocity_m_per_s)?,
        })
    }

    /// Wraps an already finite position/velocity pair as a Cartesian state.
    #[must_use]
    pub const fn from_position_velocity(position_velocity: PositionVelocity) -> Self {
        Self { position_velocity }
    }

    /// Returns the finite position/velocity pair.
    #[must_use]
    const fn position_velocity(self) -> PositionVelocity {
        self.position_velocity
    }
}

/// Cartesian state with explicit frame and epoch labels.
///
/// The frame and epoch are mandatory type fields, not optional metadata. This
/// module never transforms between frames or time scales; consumers that combine
/// two labeled states should call `require_same_frame_and_time_scale` or an
/// equivalent future policy guard before doing vector arithmetic.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StateWithFrameAndEpoch {
    state: CartesianState,
    frame: AstroFrame,
    epoch: AstroEpoch,
}

impl StateWithFrameAndEpoch {
    /// Attaches mandatory frame and epoch labels to a finite Cartesian state.
    #[must_use]
    pub const fn new(state: CartesianState, frame: AstroFrame, epoch: AstroEpoch) -> Self {
        Self {
            state,
            frame,
            epoch,
        }
    }

    /// Returns the frame label.
    #[must_use]
    pub const fn frame(self) -> AstroFrame {
        self.frame
    }

    /// Returns the epoch label and scalar offset.
    #[must_use]
    pub const fn epoch(self) -> AstroEpoch {
        self.epoch
    }

    /// Returns the position/velocity vectors.
    #[must_use]
    pub const fn position_velocity(self) -> PositionVelocity {
        self.state.position_velocity()
    }
}

/// Positive gravitational parameter, `mu`, in `m^3/s^2`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GravitationalParameter {
    m3_per_s2: f64,
}

impl GravitationalParameter {
    /// Creates a positive finite gravitational parameter.
    pub fn new(m3_per_s2: f64) -> AstroStateResult<Self> {
        if !m3_per_s2.is_finite() {
            return Err(AstroStateValidationError::NonfiniteComponent {
                parameter: "mu_m3_per_s2",
                value: m3_per_s2,
            });
        }
        if m3_per_s2 <= 0.0 {
            return Err(
                AstroStateValidationError::NonPositiveGravitationalParameter { value: m3_per_s2 },
            );
        }
        Ok(Self { m3_per_s2 })
    }

    /// Returns `mu` in `m^3/s^2`.
    #[must_use]
    pub const fn as_m3_per_s2(self) -> f64 {
        self.m3_per_s2
    }
}

/// Validation errors for Cartesian-state containers and state-derived helpers.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AstroStateValidationError {
    /// A position, velocity, epoch, or `mu` scalar was NaN or infinite.
    NonfiniteComponent { parameter: &'static str, value: f64 },
    /// A Euclidean norm computation produced NaN or infinity after finite input checks.
    NonfiniteNorm { quantity: &'static str },
    /// A positive finite gravitational parameter was required.
    NonPositiveGravitationalParameter { value: f64 },
    /// An orbital-use helper required nonzero position radius.
    ZeroPositionRadius,
    /// Two labeled states had different frame labels.
    FrameMismatch { left: AstroFrame, right: AstroFrame },
    /// Two labeled states had different time-scale labels.
    TimeScaleMismatch {
        left: AstroTimeScale,
        right: AstroTimeScale,
    },
}

impl AstroStateValidationError {
    /// Stable snake-case code for tests, validation records, and reports.
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::NonfiniteComponent { .. } => "nonfinite_component",
            Self::NonfiniteNorm { .. } => "nonfinite_norm",
            Self::NonPositiveGravitationalParameter { .. } => {
                "non_positive_gravitational_parameter"
            }
            Self::ZeroPositionRadius => "zero_position_radius",
            Self::FrameMismatch { .. } => "frame_mismatch",
            Self::TimeScaleMismatch { .. } => "time_scale_mismatch",
        }
    }
}

impl fmt::Display for AstroStateValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonfiniteComponent { parameter, value } => {
                write!(
                    formatter,
                    "state component `{parameter}` must be finite, got {value}"
                )
            }
            Self::NonfiniteNorm { quantity } => {
                write!(formatter, "{quantity} norm produced a nonfinite value")
            }
            Self::NonPositiveGravitationalParameter { value } => {
                write!(
                    formatter,
                    "gravitational parameter mu must be positive and finite, got {value}"
                )
            }
            Self::ZeroPositionRadius => {
                formatter.write_str("orbital-use helper requires nonzero position radius")
            }
            Self::FrameMismatch { left, right } => {
                write!(
                    formatter,
                    "state frame mismatch: left={} right={}",
                    left.label(),
                    right.label()
                )
            }
            Self::TimeScaleMismatch { left, right } => {
                write!(
                    formatter,
                    "state time-scale mismatch: left={} right={}",
                    left.label(),
                    right.label()
                )
            }
        }
    }
}

impl std::error::Error for AstroStateValidationError {}

/// Validates finite position and velocity components only.
///
/// This function does not require a nonzero radius and does not require `mu`.
/// Use `validate_cartesian_state_for_orbital_use` for orbital equations that
/// need a central-body gravitational parameter and a nonzero radius.
pub fn validate_position_velocity_finite(
    position_velocity: &PositionVelocity,
) -> AstroStateResult<()> {
    validate_position_velocity_components(
        position_velocity.position_m(),
        position_velocity.velocity_m_per_s(),
    )
}

/// Validates a labeled Cartesian state for finite storage.
///
/// Frame and epoch labels are mandatory by type construction. This guard also
/// re-checks the epoch scalar offset for defensive validation. It does not
/// perform frame transforms or time-scale conversion.
pub fn validate_cartesian_state(state: &StateWithFrameAndEpoch) -> AstroStateResult<()> {
    validate_position_velocity_finite(&state.position_velocity())?;
    let epoch_seconds = state.epoch().seconds_from_reference_epoch();
    if !epoch_seconds.is_finite() {
        return Err(AstroStateValidationError::NonfiniteComponent {
            parameter: "epoch.seconds_from_reference_epoch",
            value: epoch_seconds,
        });
    }
    Ok(())
}

/// Validates a labeled Cartesian state for two-body orbital-use helpers.
///
/// This extends finite storage validation with a positive `mu` contract and a
/// nonzero position radius. It intentionally does not classify circular,
/// equatorial, parabolic, or hyperbolic singularities; those belong in later
/// element/Kepler modules.
pub fn validate_cartesian_state_for_orbital_use(
    state: &StateWithFrameAndEpoch,
    mu: GravitationalParameter,
) -> AstroStateResult<()> {
    validate_cartesian_state(state)?;
    if !mu.as_m3_per_s2().is_finite() || mu.as_m3_per_s2() <= 0.0 {
        return Err(
            AstroStateValidationError::NonPositiveGravitationalParameter {
                value: mu.as_m3_per_s2(),
            },
        );
    }
    let radius_m = position_norm(&state.position_velocity())?;
    if radius_m == 0.0 {
        return Err(AstroStateValidationError::ZeroPositionRadius);
    }
    Ok(())
}

/// Computes the Euclidean norm of the position vector in meters.
///
/// Domain: finite components. A zero vector is valid for finite storage but may
/// be rejected by orbital-use helpers. If finite components produce a nonfinite
/// norm because the mathematical magnitude exceeds `f64`, this function fails
/// closed instead of returning infinity.
pub fn position_norm(position_velocity: &PositionVelocity) -> AstroStateResult<f64> {
    finite_scaled_vector_norm(
        "position",
        position_component_names(),
        position_velocity.position_m(),
    )
}

/// Computes the Euclidean norm of the velocity vector in meters per second.
///
/// Domain: finite components. If finite components produce a nonfinite norm
/// because the mathematical magnitude exceeds `f64`, this function fails closed
/// instead of returning infinity.
pub fn velocity_norm(position_velocity: &PositionVelocity) -> AstroStateResult<f64> {
    finite_scaled_vector_norm(
        "velocity",
        velocity_component_names(),
        position_velocity.velocity_m_per_s(),
    )
}

/// Requires two labeled states to share the same frame and time-scale labels.
///
/// This is a guard against silent frame/time-scale mixing. It is not a frame
/// transform, not an epoch-equality test, and not a time-scale conversion.
pub fn require_same_frame_and_time_scale(
    left: &StateWithFrameAndEpoch,
    right: &StateWithFrameAndEpoch,
) -> AstroStateResult<()> {
    if left.frame() != right.frame() {
        return Err(AstroStateValidationError::FrameMismatch {
            left: left.frame(),
            right: right.frame(),
        });
    }

    let left_scale = left.epoch().time_scale();
    let right_scale = right.epoch().time_scale();
    if left_scale != right_scale {
        return Err(AstroStateValidationError::TimeScaleMismatch {
            left: left_scale,
            right: right_scale,
        });
    }

    Ok(())
}

fn validate_position_velocity_components(
    position_m: [f64; 3],
    velocity_m_per_s: [f64; 3],
) -> AstroStateResult<()> {
    for (index, value) in position_m.iter().copied().enumerate() {
        validate_finite_component(position_component_names()[index], value)?;
    }
    for (index, value) in velocity_m_per_s.iter().copied().enumerate() {
        validate_finite_component(velocity_component_names()[index], value)?;
    }
    Ok(())
}

fn validate_finite_component(parameter: &'static str, value: f64) -> AstroStateResult<()> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(AstroStateValidationError::NonfiniteComponent { parameter, value })
    }
}

fn finite_scaled_vector_norm(
    quantity: &'static str,
    component_names: [&'static str; 3],
    components: [f64; 3],
) -> AstroStateResult<f64> {
    for (index, value) in components.iter().copied().enumerate() {
        validate_finite_component(component_names[index], value)?;
    }

    let mut scale = 0.0_f64;
    for value in components.iter().copied() {
        scale = scale.max(value.abs());
    }
    if scale == 0.0 {
        return Ok(0.0);
    }

    let mut scaled_sum = 0.0_f64;
    for value in components.iter().copied() {
        let ratio = value / scale;
        scaled_sum += ratio * ratio;
    }

    let norm = scale * scaled_sum.sqrt();
    if norm.is_finite() {
        Ok(norm)
    } else {
        Err(AstroStateValidationError::NonfiniteNorm { quantity })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::time::AstroTimeScale;

    const EARTH_MU_M3_PER_S2: f64 = 3.986_004_418e14;

    fn tai_epoch(seconds: f64) -> AstroEpoch {
        AstroEpoch::new(AstroTimeScale::Tai, seconds).expect("finite TAI epoch")
    }

    fn utc_epoch(seconds: f64) -> AstroEpoch {
        AstroEpoch::new(AstroTimeScale::Utc, seconds).expect("finite UTC epoch")
    }

    fn circular_state(radius_m: f64) -> PositionVelocity {
        let speed_m_per_s = (EARTH_MU_M3_PER_S2 / radius_m).sqrt();
        PositionVelocity::new([radius_m, 0.0, 0.0], [0.0, speed_m_per_s, 0.0])
            .expect("finite circular state")
    }

    fn labeled_state(frame: AstroFrame, epoch: AstroEpoch) -> StateWithFrameAndEpoch {
        StateWithFrameAndEpoch::new(
            CartesianState::from_position_velocity(circular_state(7_000_000.0)),
            frame,
            epoch,
        )
    }

    #[test]
    fn finite_labeled_state_passes_validation_and_preserves_labels() {
        let state = labeled_state(AstroFrame::InertialEciMeanEquator, tai_epoch(10.0));
        validate_cartesian_state(&state).expect("finite state should validate");
        assert_eq!(state.frame(), AstroFrame::InertialEciMeanEquator);
        assert_eq!(state.epoch().time_scale(), AstroTimeScale::Tai);
        assert_eq!(state.epoch().seconds_from_reference_epoch(), 10.0);
    }

    #[test]
    fn nonfinite_position_component_is_rejected() {
        let error = PositionVelocity::new([f64::NAN, 0.0, 0.0], [0.0, 1.0, 0.0])
            .expect_err("NaN position should fail");
        assert_eq!(error.code(), "nonfinite_component");
    }

    #[test]
    fn infinite_velocity_component_is_rejected() {
        let error = PositionVelocity::new([1.0, 0.0, 0.0], [0.0, f64::INFINITY, 0.0])
            .expect_err("infinite velocity should fail");
        assert_eq!(error.code(), "nonfinite_component");
    }

    #[test]
    fn finite_zero_radius_state_is_allowed_for_storage_but_not_orbital_use() {
        let position_velocity =
            PositionVelocity::new([0.0, 0.0, 0.0], [0.0, 7_500.0, 0.0]).unwrap();
        let state = StateWithFrameAndEpoch::new(
            CartesianState::from_position_velocity(position_velocity),
            AstroFrame::InertialEciMeanEquator,
            tai_epoch(0.0),
        );
        let mu = GravitationalParameter::new(EARTH_MU_M3_PER_S2).unwrap();

        validate_cartesian_state(&state).expect("finite zero-radius storage state is allowed");
        assert_eq!(
            validate_cartesian_state_for_orbital_use(&state, mu),
            Err(AstroStateValidationError::ZeroPositionRadius)
        );
    }

    #[test]
    fn nonpositive_gravitational_parameter_is_rejected() {
        assert_eq!(
            GravitationalParameter::new(-1.0),
            Err(AstroStateValidationError::NonPositiveGravitationalParameter { value: -1.0 })
        );
        assert_eq!(
            GravitationalParameter::new(0.0),
            Err(AstroStateValidationError::NonPositiveGravitationalParameter { value: 0.0 })
        );
    }

    #[test]
    fn nonfinite_gravitational_parameter_is_rejected() {
        let error =
            GravitationalParameter::new(f64::INFINITY).expect_err("infinite mu should fail");
        assert_eq!(error.code(), "nonfinite_component");
    }

    #[test]
    fn direct_position_and_velocity_norm_tests_use_si_units() {
        let position_velocity = PositionVelocity::new([3.0, 4.0, 12.0], [2.0, 3.0, 6.0]).unwrap();
        assert_eq!(position_norm(&position_velocity).unwrap(), 13.0);
        assert_eq!(velocity_norm(&position_velocity).unwrap(), 7.0);
    }

    #[test]
    fn vector_norm_overflow_is_blocked_after_finite_component_validation() {
        let position_velocity =
            PositionVelocity::new([f64::MAX, f64::MAX, 0.0], [0.0, 0.0, 0.0]).unwrap();
        assert_eq!(
            position_norm(&position_velocity),
            Err(AstroStateValidationError::NonfiniteNorm {
                quantity: "position"
            })
        );

        let position_velocity =
            PositionVelocity::new([1.0, 0.0, 0.0], [f64::MAX, f64::MAX, 0.0]).unwrap();
        assert_eq!(
            velocity_norm(&position_velocity),
            Err(AstroStateValidationError::NonfiniteNorm {
                quantity: "velocity"
            })
        );
    }

    #[test]
    fn same_frame_and_time_scale_guard_accepts_matching_labels() {
        let left = labeled_state(AstroFrame::InertialEciMeanEquator, tai_epoch(0.0));
        let right = labeled_state(AstroFrame::InertialEciMeanEquator, tai_epoch(60.0));
        require_same_frame_and_time_scale(&left, &right).expect("labels match");
    }

    #[test]
    fn same_frame_and_time_scale_guard_rejects_frame_mismatch() {
        let left = labeled_state(AstroFrame::InertialEciMeanEquator, tai_epoch(0.0));
        let right = labeled_state(AstroFrame::Perifocal, tai_epoch(0.0));
        let error = require_same_frame_and_time_scale(&left, &right).unwrap_err();
        assert_eq!(error.code(), "frame_mismatch");
    }

    #[test]
    fn same_frame_and_time_scale_guard_rejects_time_scale_mismatch() {
        let left = labeled_state(AstroFrame::InertialEciMeanEquator, tai_epoch(0.0));
        let right = labeled_state(AstroFrame::InertialEciMeanEquator, utc_epoch(0.0));
        let error = require_same_frame_and_time_scale(&left, &right).unwrap_err();
        assert_eq!(error.code(), "time_scale_mismatch");
    }

    #[test]
    fn orbital_use_validation_accepts_simple_finite_circular_state() {
        let state = labeled_state(AstroFrame::InertialEciMeanEquator, tai_epoch(0.0));
        let mu = GravitationalParameter::new(EARTH_MU_M3_PER_S2).unwrap();
        validate_cartesian_state_for_orbital_use(&state, mu)
            .expect("finite nonzero-radius state should pass orbital-use validation");
    }
}
