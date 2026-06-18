//! Bounded elliptic Kepler and two-body scalar helpers.
//!
//! This module is independently written from standard two-body relations. It
//! does not copy or translate external astrodynamics source code, tests,
//! fixtures, comments, class hierarchies, control flow, or API architecture.
//!
//! Units and conventions:
//! - gravitational parameter: cubic metres per square second (`m^3/s^2`);
//! - semi-major axis and radius: metres (`m`);
//! - elapsed duration: SI seconds (`s`);
//! - mean motion: radians per second (`rad/s`);
//! - anomalies and residuals: radians (`rad`);
//! - eccentricity: dimensionless.
//!
//! All returned anomaly angles are normalized to `[0, 2*pi)`. The normalization
//! is local to this module and does not define a generic endpoint-sensitive
//! angle-wrapping API.
//!
//! These functions are scalar, elliptic, unperturbed two-body helpers. They do
//! not propagate Cartesian states, attach or transform frames, convert time
//! scales, model leap seconds, evaluate perturbations, implement SGP4, or make
//! operational orbit-prediction claims. `AstroDuration` is used only as a finite
//! scale-free duration container; negative duration is allowed for backward
//! scalar anomaly advance.

use crate::state::GravitationalParameter;
use crate::time::AstroDuration;
use core::fmt;
use std::f64::consts::PI;

const TWO_PI: f64 = 2.0 * PI;

/// Result alias for bounded elliptic Kepler helpers.
pub type KeplerResult<T> = Result<T, KeplerError>;

/// Explicit numerical controls for the elliptic Kepler solver.
///
/// `residual_tolerance_rad` is applied to the absolute residual of
/// `E - e*sin(E) - M`. There is intentionally no `Default` implementation:
/// callers must choose and document a positive finite tolerance and iteration
/// cap for their use case.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KeplerSolveOptions {
    residual_tolerance_rad: f64,
    max_iterations: u32,
}

impl KeplerSolveOptions {
    /// Creates solver options with a positive finite residual tolerance and a
    /// nonzero iteration cap.
    pub fn new(residual_tolerance_rad: f64, max_iterations: u32) -> KeplerResult<Self> {
        let options = Self {
            residual_tolerance_rad,
            max_iterations,
        };
        options.validate()?;
        Ok(options)
    }

    fn validate(self) -> KeplerResult<()> {
        if !self.residual_tolerance_rad.is_finite() || self.residual_tolerance_rad <= 0.0 {
            return Err(KeplerError::InvalidResidualTolerance {
                value: self.residual_tolerance_rad,
            });
        }
        if self.max_iterations == 0 {
            return Err(KeplerError::InvalidIterationLimit {
                value: self.max_iterations,
            });
        }
        Ok(())
    }
}

/// Successful output from the bounded elliptic Kepler solve.
///
/// Fields are public because this is an immutable value report, not an
/// invariant-bearing input type. Avoiding accessor methods also keeps the
/// governed helper-function surface bounded.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KeplerSolveReport {
    /// Mean anomaly after normalization to `[0, 2*pi)`.
    pub normalized_mean_anomaly_rad: f64,
    /// Solved eccentric anomaly in `[0, 2*pi)`.
    pub eccentric_anomaly_rad: f64,
    /// Number of safeguarded Newton updates performed.
    pub iterations: u32,
    /// Recomputed absolute final residual, `|E - e*sin(E) - M|`, in radians.
    pub absolute_residual_rad: f64,
}

/// Errors returned by bounded elliptic Kepler helpers.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KeplerError {
    /// A scalar input was NaN or infinite.
    NonfiniteInput { parameter: &'static str, value: f64 },
    /// A strictly positive scalar input was zero or negative.
    NonPositiveInput { parameter: &'static str, value: f64 },
    /// Eccentricity was not finite or was outside the elliptic interval `[0, 1)`.
    EccentricityOutsideEllipticDomain { value: f64 },
    /// Solver residual tolerance was not positive and finite.
    InvalidResidualTolerance { value: f64 },
    /// Solver iteration cap was zero.
    InvalidIterationLimit { value: u32 },
    /// A finite input combination produced an unrepresentable or invalid result.
    NumericalFailure {
        function: &'static str,
        reason: &'static str,
    },
    /// The solver exhausted its iteration cap before satisfying the residual tolerance.
    NonConvergence {
        iterations: u32,
        eccentric_anomaly_rad: f64,
        absolute_residual_rad: f64,
    },
}

impl KeplerError {
    /// Stable snake-case code for tests and later evidence records.
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::NonfiniteInput { .. } => "nonfinite_input",
            Self::NonPositiveInput { .. } => "non_positive_input",
            Self::EccentricityOutsideEllipticDomain { .. } => {
                "eccentricity_outside_elliptic_domain"
            }
            Self::InvalidResidualTolerance { .. } => "invalid_residual_tolerance",
            Self::InvalidIterationLimit { .. } => "invalid_iteration_limit",
            Self::NumericalFailure { .. } => "numerical_failure",
            Self::NonConvergence { .. } => "non_convergence",
        }
    }
}

impl fmt::Display for KeplerError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonfiniteInput { parameter, value } => {
                write!(formatter, "`{parameter}` must be finite, got {value}")
            }
            Self::NonPositiveInput { parameter, value } => {
                write!(formatter, "`{parameter}` must be positive, got {value}")
            }
            Self::EccentricityOutsideEllipticDomain { value } => write!(
                formatter,
                "`eccentricity` must be finite and in [0, 1), got {value}"
            ),
            Self::InvalidResidualTolerance { value } => write!(
                formatter,
                "`residual_tolerance_rad` must be positive and finite, got {value}"
            ),
            Self::InvalidIterationLimit { value } => {
                write!(formatter, "`max_iterations` must be nonzero, got {value}")
            }
            Self::NumericalFailure { function, reason } => {
                write!(formatter, "`{function}` failed numerically: {reason}")
            }
            Self::NonConvergence {
                iterations,
                eccentric_anomaly_rad,
                absolute_residual_rad,
            } => write!(
                formatter,
                "elliptic Kepler solve did not converge after {iterations} updates; \
                 final E={eccentric_anomaly_rad} rad, absolute residual={absolute_residual_rad} rad"
            ),
        }
    }
}

impl std::error::Error for KeplerError {}

/// Computes elliptic two-body mean motion, `n = sqrt(mu / a^3)`.
///
/// Input units are `mu` in `m^3/s^2` and positive semi-major axis `a` in
/// metres. The returned scalar is in `rad/s`. The implementation uses an
/// equivalent logarithmic form to avoid directly overflowing `a^3`; a true
/// result outside the positive finite `f64` range is rejected.
pub fn mean_motion(mu: GravitationalParameter, semi_major_axis_m: f64) -> KeplerResult<f64> {
    validate_positive_finite("semi_major_axis_m", semi_major_axis_m)?;

    let log_mean_motion = 0.5 * mu.as_m3_per_s2().ln() - 1.5 * semi_major_axis_m.ln();
    if !log_mean_motion.is_finite() {
        return Err(KeplerError::NumericalFailure {
            function: "mean_motion",
            reason: "logarithmic mean-motion form produced a nonfinite exponent",
        });
    }

    let mean_motion_rad_per_s = log_mean_motion.exp();
    if !mean_motion_rad_per_s.is_finite() || mean_motion_rad_per_s <= 0.0 {
        return Err(KeplerError::NumericalFailure {
            function: "mean_motion",
            reason: "mean motion is outside the positive finite f64 range",
        });
    }

    Ok(mean_motion_rad_per_s)
}

/// Advances mean anomaly by `M = M0 + n*delta_t` and normalizes to `[0, 2*pi)`.
///
/// `mean_motion_rad_per_s` is a positive scalar magnitude. `elapsed_duration`
/// may be negative for a backward scalar advance. This function does not attach
/// an epoch or time scale and does not perform leap-second-aware arithmetic.
pub fn mean_anomaly_advance(
    initial_mean_anomaly_rad: f64,
    mean_motion_rad_per_s: f64,
    elapsed_duration: AstroDuration,
) -> KeplerResult<f64> {
    validate_finite("initial_mean_anomaly_rad", initial_mean_anomaly_rad)?;
    validate_positive_finite("mean_motion_rad_per_s", mean_motion_rad_per_s)?;
    let elapsed_seconds = elapsed_duration.as_seconds();
    validate_finite("elapsed_duration_s", elapsed_seconds)?;

    let phase_advance_rad = mean_motion_rad_per_s * elapsed_seconds;
    if !phase_advance_rad.is_finite() {
        return Err(KeplerError::NumericalFailure {
            function: "mean_anomaly_advance",
            reason: "mean-motion and duration product was nonfinite",
        });
    }

    let initial_normalized = normalize_angle_0_to_two_pi(initial_mean_anomaly_rad)?;
    let advance_normalized = normalize_angle_0_to_two_pi(phase_advance_rad)?;
    normalize_angle_0_to_two_pi(initial_normalized + advance_normalized)
}

/// Solves the elliptic Kepler equation `E - e*sin(E) = M`.
///
/// The input mean anomaly is normalized to `[0, 2*pi)`. Eccentricity must be in
/// `[0, 1)`. A Newton update is accepted only while it remains inside the
/// monotonic elliptic root bracket; otherwise a bisection step is used. This
/// safeguarded policy preserves a finite bracket while retaining Newton's local
/// convergence. Parabolic and hyperbolic cases are blocked.
///
/// On success, the report contains a normalized eccentric anomaly and a
/// recomputed final absolute residual. On iteration exhaustion, the error also
/// contains the final candidate and its recomputed residual after the last
/// update; it never reports a stale pre-update residual.
pub fn solve_kepler_elliptic_newton(
    mean_anomaly_rad: f64,
    eccentricity: f64,
    options: KeplerSolveOptions,
) -> KeplerResult<KeplerSolveReport> {
    options.validate()?;
    validate_finite("mean_anomaly_rad", mean_anomaly_rad)?;
    validate_elliptic_eccentricity(eccentricity)?;

    let normalized_mean_anomaly_rad = normalize_angle_0_to_two_pi(mean_anomaly_rad)?;

    if eccentricity == 0.0 || normalized_mean_anomaly_rad == 0.0 {
        return Ok(KeplerSolveReport {
            normalized_mean_anomaly_rad,
            eccentric_anomaly_rad: normalized_mean_anomaly_rad,
            iterations: 0,
            absolute_residual_rad: 0.0,
        });
    }

    let mut lower_bound_rad = 0.0;
    let mut upper_bound_rad = TWO_PI;
    let mut eccentric_anomaly_rad = if eccentricity < 0.8 {
        normalized_mean_anomaly_rad
    } else {
        PI
    };
    let mut residual_rad = kepler_equation_residual(
        eccentric_anomaly_rad,
        eccentricity,
        normalized_mean_anomaly_rad,
    )?;

    if residual_rad.abs() <= options.residual_tolerance_rad {
        return build_solve_report(
            normalized_mean_anomaly_rad,
            eccentric_anomaly_rad,
            0,
            residual_rad,
        );
    }

    for iteration in 1..=options.max_iterations {
        if residual_rad < 0.0 {
            lower_bound_rad = eccentric_anomaly_rad;
        } else {
            upper_bound_rad = eccentric_anomaly_rad;
        }

        let derivative = 1.0 - eccentricity * eccentric_anomaly_rad.cos();
        if !derivative.is_finite() || derivative <= 0.0 {
            return Err(KeplerError::NumericalFailure {
                function: "solve_kepler_elliptic_newton",
                reason: "Kepler-equation derivative was not positive and finite",
            });
        }

        eccentric_anomaly_rad = safeguarded_newton_candidate(
            eccentric_anomaly_rad,
            residual_rad,
            derivative,
            lower_bound_rad,
            upper_bound_rad,
        )?;
        residual_rad = kepler_equation_residual(
            eccentric_anomaly_rad,
            eccentricity,
            normalized_mean_anomaly_rad,
        )?;

        if residual_rad.abs() <= options.residual_tolerance_rad {
            return build_solve_report(
                normalized_mean_anomaly_rad,
                eccentric_anomaly_rad,
                iteration,
                residual_rad,
            );
        }
    }

    let final_eccentric_anomaly_rad = normalize_angle_0_to_two_pi(eccentric_anomaly_rad)?;
    let final_residual_rad = kepler_equation_residual(
        eccentric_anomaly_rad,
        eccentricity,
        normalized_mean_anomaly_rad,
    )?
    .abs();

    Err(KeplerError::NonConvergence {
        iterations: options.max_iterations,
        eccentric_anomaly_rad: final_eccentric_anomaly_rad,
        absolute_residual_rad: final_residual_rad,
    })
}

/// Converts elliptic eccentric anomaly to true anomaly.
///
/// Input and output angles are radians. The output is normalized to
/// `[0, 2*pi)`. Eccentricity must be in `[0, 1)`.
pub fn true_anomaly_from_eccentric_anomaly(
    eccentric_anomaly_rad: f64,
    eccentricity: f64,
) -> KeplerResult<f64> {
    validate_finite("eccentric_anomaly_rad", eccentric_anomaly_rad)?;
    validate_elliptic_eccentricity(eccentricity)?;

    let eccentric_anomaly_rad = normalize_angle_0_to_two_pi(eccentric_anomaly_rad)?;
    let elliptic_factor = ((1.0 - eccentricity) * (1.0 + eccentricity)).sqrt();
    if !elliptic_factor.is_finite() || elliptic_factor <= 0.0 {
        return Err(KeplerError::NumericalFailure {
            function: "true_anomaly_from_eccentric_anomaly",
            reason: "elliptic square-root factor was not positive and finite",
        });
    }

    let sine_component = elliptic_factor * eccentric_anomaly_rad.sin();
    let cosine_component = eccentric_anomaly_rad.cos() - eccentricity;
    if !sine_component.is_finite() || !cosine_component.is_finite() {
        return Err(KeplerError::NumericalFailure {
            function: "true_anomaly_from_eccentric_anomaly",
            reason: "true-anomaly atan2 components were nonfinite",
        });
    }
    if sine_component == 0.0 && cosine_component == 0.0 {
        return Err(KeplerError::NumericalFailure {
            function: "true_anomaly_from_eccentric_anomaly",
            reason: "true-anomaly direction was indeterminate",
        });
    }

    normalize_angle_0_to_two_pi(sine_component.atan2(cosine_component))
}

/// Computes elliptic orbital radius, `r = a*(1 - e*cos(E))`.
///
/// `semi_major_axis_m` must be positive and finite, eccentricity must be in
/// `[0, 1)`, and eccentric anomaly is in radians. The returned radius is in
/// metres and must remain positive and finite.
pub fn radius_from_semimajor_eccentric_anomaly(
    semi_major_axis_m: f64,
    eccentricity: f64,
    eccentric_anomaly_rad: f64,
) -> KeplerResult<f64> {
    validate_positive_finite("semi_major_axis_m", semi_major_axis_m)?;
    validate_elliptic_eccentricity(eccentricity)?;
    validate_finite("eccentric_anomaly_rad", eccentric_anomaly_rad)?;

    let eccentric_anomaly_rad = normalize_angle_0_to_two_pi(eccentric_anomaly_rad)?;
    let radial_factor = 1.0 - eccentricity * eccentric_anomaly_rad.cos();
    if !radial_factor.is_finite() || radial_factor <= 0.0 {
        return Err(KeplerError::NumericalFailure {
            function: "radius_from_semimajor_eccentric_anomaly",
            reason: "elliptic radial factor was not positive and finite",
        });
    }

    let radius_m = semi_major_axis_m * radial_factor;
    if !radius_m.is_finite() || radius_m <= 0.0 {
        return Err(KeplerError::NumericalFailure {
            function: "radius_from_semimajor_eccentric_anomaly",
            reason: "orbital radius was outside the positive finite f64 range",
        });
    }

    Ok(radius_m)
}

fn safeguarded_newton_candidate(
    current_rad: f64,
    residual_rad: f64,
    derivative: f64,
    lower_bound_rad: f64,
    upper_bound_rad: f64,
) -> KeplerResult<f64> {
    let newton_candidate = current_rad - residual_rad / derivative;
    let candidate_rad = if newton_candidate.is_finite()
        && newton_candidate > lower_bound_rad
        && newton_candidate < upper_bound_rad
    {
        newton_candidate
    } else {
        lower_bound_rad + 0.5 * (upper_bound_rad - lower_bound_rad)
    };

    if candidate_rad.is_finite() {
        Ok(candidate_rad)
    } else {
        Err(KeplerError::NumericalFailure {
            function: "solve_kepler_elliptic_newton",
            reason: "safeguarded Newton candidate was nonfinite",
        })
    }
}

fn build_solve_report(
    normalized_mean_anomaly_rad: f64,
    eccentric_anomaly_rad: f64,
    iterations: u32,
    residual_rad: f64,
) -> KeplerResult<KeplerSolveReport> {
    let eccentric_anomaly_rad = normalize_angle_0_to_two_pi(eccentric_anomaly_rad)?;
    let absolute_residual_rad = residual_rad.abs();
    if !absolute_residual_rad.is_finite() {
        return Err(KeplerError::NumericalFailure {
            function: "solve_kepler_elliptic_newton",
            reason: "final absolute residual was nonfinite",
        });
    }

    Ok(KeplerSolveReport {
        normalized_mean_anomaly_rad,
        eccentric_anomaly_rad,
        iterations,
        absolute_residual_rad,
    })
}

fn kepler_equation_residual(
    eccentric_anomaly_rad: f64,
    eccentricity: f64,
    normalized_mean_anomaly_rad: f64,
) -> KeplerResult<f64> {
    let residual_rad = eccentric_anomaly_rad
        - eccentricity * eccentric_anomaly_rad.sin()
        - normalized_mean_anomaly_rad;
    if residual_rad.is_finite() {
        Ok(residual_rad)
    } else {
        Err(KeplerError::NumericalFailure {
            function: "solve_kepler_elliptic_newton",
            reason: "Kepler-equation residual was nonfinite",
        })
    }
}

fn normalize_angle_0_to_two_pi(angle_rad: f64) -> KeplerResult<f64> {
    validate_finite("angle_rad", angle_rad)?;
    let normalized = angle_rad.rem_euclid(TWO_PI);
    if normalized.is_finite() && (0.0..TWO_PI).contains(&normalized) {
        Ok(normalized)
    } else {
        Err(KeplerError::NumericalFailure {
            function: "normalize_angle_0_to_two_pi",
            reason: "angle normalization did not produce a value in [0, 2*pi)",
        })
    }
}

fn validate_finite(parameter: &'static str, value: f64) -> KeplerResult<()> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(KeplerError::NonfiniteInput { parameter, value })
    }
}

fn validate_positive_finite(parameter: &'static str, value: f64) -> KeplerResult<()> {
    validate_finite(parameter, value)?;
    if value > 0.0 {
        Ok(())
    } else {
        Err(KeplerError::NonPositiveInput { parameter, value })
    }
}

fn validate_elliptic_eccentricity(eccentricity: f64) -> KeplerResult<()> {
    if eccentricity.is_finite() && (0.0..1.0).contains(&eccentricity) {
        Ok(())
    } else {
        Err(KeplerError::EccentricityOutsideEllipticDomain {
            value: eccentricity,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn earth_mu_m3_per_s2() -> f64 {
        3.986_004_418e14
    }

    fn assert_close(actual: f64, expected: f64, tolerance: f64) {
        let error = (actual - expected).abs();
        assert!(
            error <= tolerance,
            "actual={actual}, expected={expected}, error={error}, tolerance={tolerance}"
        );
    }

    fn assert_relative_close(actual: f64, expected: f64, relative_tolerance: f64) {
        let scale = expected.abs().max(f64::MIN_POSITIVE);
        assert_close(actual, expected, relative_tolerance * scale);
    }

    fn assert_normalized(angle_rad: f64) {
        assert!(
            (0.0..TWO_PI).contains(&angle_rad),
            "angle was not normalized: {angle_rad}"
        );
    }

    fn earth_mu() -> GravitationalParameter {
        GravitationalParameter::new(earth_mu_m3_per_s2()).unwrap()
    }

    #[test]
    fn options_constructor_and_accessors_are_explicit() {
        let options = KeplerSolveOptions::new(1.0e-12, 64).unwrap();
        assert_eq!(options.residual_tolerance_rad, 1.0e-12);
        assert_eq!(options.max_iterations, 64);
    }

    #[test]
    fn invalid_residual_tolerance_matrix_is_rejected() {
        for value in [0.0, -1.0, f64::NAN, f64::INFINITY, -f64::INFINITY] {
            assert!(matches!(
                KeplerSolveOptions::new(value, 32),
                Err(KeplerError::InvalidResidualTolerance { .. })
            ));
        }
    }

    #[test]
    fn zero_iteration_limit_is_rejected() {
        assert!(matches!(
            KeplerSolveOptions::new(1.0e-12, 0),
            Err(KeplerError::InvalidIterationLimit { value: 0 })
        ));
    }

    #[test]
    fn solver_revalidates_options_invariant() {
        let invalid_tolerance = KeplerSolveOptions {
            residual_tolerance_rad: f64::NAN,
            max_iterations: 10,
        };
        assert!(matches!(
            solve_kepler_elliptic_newton(1.0, 0.1, invalid_tolerance),
            Err(KeplerError::InvalidResidualTolerance { .. })
        ));

        let invalid_limit = KeplerSolveOptions {
            residual_tolerance_rad: 1.0e-12,
            max_iterations: 0,
        };
        assert!(matches!(
            solve_kepler_elliptic_newton(1.0, 0.1, invalid_limit),
            Err(KeplerError::InvalidIterationLimit { value: 0 })
        ));
    }

    #[test]
    fn safeguarded_newton_candidate_falls_back_to_bracket_midpoint() {
        let candidate = safeguarded_newton_candidate(1.0, 3.0, 1.0, 0.0, 4.0).unwrap();
        assert_eq!(candidate, 2.0);
    }

    #[test]
    fn mean_motion_matches_standard_two_body_relation() {
        let semi_major_axis_m = 7_000_000.0;
        let actual = mean_motion(earth_mu(), semi_major_axis_m).unwrap();
        let expected = (earth_mu_m3_per_s2() / semi_major_axis_m.powi(3)).sqrt();
        assert_relative_close(actual, expected, 2.0e-14);
        assert!(actual > 0.0);
    }

    #[test]
    fn mean_motion_rejects_nonpositive_and_nonfinite_axis() {
        for value in [0.0, -1.0] {
            assert!(matches!(
                mean_motion(earth_mu(), value),
                Err(KeplerError::NonPositiveInput {
                    parameter: "semi_major_axis_m",
                    ..
                })
            ));
        }
        for value in [f64::NAN, f64::INFINITY, -f64::INFINITY] {
            assert!(matches!(
                mean_motion(earth_mu(), value),
                Err(KeplerError::NonfiniteInput {
                    parameter: "semi_major_axis_m",
                    ..
                })
            ));
        }
    }

    #[test]
    fn mean_motion_blocks_unrepresentable_extremes() {
        let largest_mu = GravitationalParameter::new(f64::MAX).unwrap();
        assert!(matches!(
            mean_motion(largest_mu, f64::MIN_POSITIVE),
            Err(KeplerError::NumericalFailure {
                function: "mean_motion",
                ..
            })
        ));

        let smallest_mu = GravitationalParameter::new(f64::MIN_POSITIVE).unwrap();
        assert!(matches!(
            mean_motion(smallest_mu, f64::MAX),
            Err(KeplerError::NumericalFailure {
                function: "mean_motion",
                ..
            })
        ));
    }

    #[test]
    fn mean_anomaly_advance_is_finite_deterministic_and_normalized() {
        let duration = AstroDuration::seconds(120.0).unwrap();
        let first = mean_anomaly_advance(0.25, 0.01, duration).unwrap();
        let second = mean_anomaly_advance(0.25, 0.01, duration).unwrap();
        assert_eq!(first, second);
        assert_close(first, 1.45, 1.0e-15);
        assert_normalized(first);
    }

    #[test]
    fn negative_duration_performs_backward_scalar_advance() {
        let duration = AstroDuration::seconds(-50.0).unwrap();
        let actual = mean_anomaly_advance(0.25, 0.01, duration).unwrap();
        assert_close(actual, TWO_PI - 0.25, 1.0e-15);
        assert_normalized(actual);
    }

    #[test]
    fn mean_anomaly_advance_rejects_nonpositive_mean_motion() {
        let duration = AstroDuration::seconds(1.0).unwrap();
        for value in [0.0, -1.0] {
            assert!(matches!(
                mean_anomaly_advance(0.0, value, duration),
                Err(KeplerError::NonPositiveInput {
                    parameter: "mean_motion_rad_per_s",
                    ..
                })
            ));
        }
    }

    #[test]
    fn mean_anomaly_advance_rejects_nonfinite_inputs_and_product_overflow() {
        let one_second = AstroDuration::seconds(1.0).unwrap();
        assert!(matches!(
            mean_anomaly_advance(f64::NAN, 1.0, one_second),
            Err(KeplerError::NonfiniteInput {
                parameter: "initial_mean_anomaly_rad",
                ..
            })
        ));
        assert!(matches!(
            mean_anomaly_advance(0.0, f64::INFINITY, one_second),
            Err(KeplerError::NonfiniteInput {
                parameter: "mean_motion_rad_per_s",
                ..
            })
        ));

        let two_seconds = AstroDuration::seconds(2.0).unwrap();
        assert!(matches!(
            mean_anomaly_advance(0.0, f64::MAX, two_seconds),
            Err(KeplerError::NumericalFailure {
                function: "mean_anomaly_advance",
                ..
            })
        ));
    }

    #[test]
    fn very_large_finite_initial_mean_anomaly_is_normalized() {
        let zero_duration = AstroDuration::seconds(0.0).unwrap();
        let large_angle = 1_000_000.0 * TWO_PI + 1.25;
        let actual = mean_anomaly_advance(large_angle, 1.0, zero_duration).unwrap();
        assert_close(actual, large_angle.rem_euclid(TWO_PI), 0.0);
        assert_normalized(actual);
    }

    #[test]
    fn circular_kepler_solve_normalizes_mean_anomaly() {
        let options = KeplerSolveOptions::new(1.0e-14, 16).unwrap();
        let report = solve_kepler_elliptic_newton(-0.25, 0.0, options).unwrap();
        assert_close(report.normalized_mean_anomaly_rad, TWO_PI - 0.25, 1.0e-15);
        assert_eq!(
            report.eccentric_anomaly_rad,
            report.normalized_mean_anomaly_rad
        );
        assert_eq!(report.iterations, 0);
        assert_eq!(report.absolute_residual_rad, 0.0);
    }

    #[test]
    fn low_eccentricity_case_converges_with_small_residual() {
        let options = KeplerSolveOptions::new(1.0e-13, 32).unwrap();
        let report = solve_kepler_elliptic_newton(1.0, 0.1, options).unwrap();
        assert_normalized(report.eccentric_anomaly_rad);
        assert!(report.iterations <= options.max_iterations);
        assert!(report.absolute_residual_rad <= options.residual_tolerance_rad);
        let reconstructed = report.eccentric_anomaly_rad - 0.1 * report.eccentric_anomaly_rad.sin();
        assert_close(reconstructed, report.normalized_mean_anomaly_rad, 1.0e-13);
    }

    #[test]
    fn large_mean_anomaly_is_normalized_before_solving() {
        let options = KeplerSolveOptions::new(1.0e-13, 64).unwrap();
        let large_mean_anomaly = 1_000_000.0 * TWO_PI + 1.25;
        let report = solve_kepler_elliptic_newton(large_mean_anomaly, 0.2, options).unwrap();
        assert_close(
            report.normalized_mean_anomaly_rad,
            large_mean_anomaly.rem_euclid(TWO_PI),
            0.0,
        );
        assert!(report.absolute_residual_rad <= options.residual_tolerance_rad);
    }

    #[test]
    fn eccentricity_near_one_converges_with_safeguard() {
        let options = KeplerSolveOptions::new(1.0e-12, 128).unwrap();
        let eccentricity = 1.0 - 1.0e-12;
        let report = solve_kepler_elliptic_newton(0.1, eccentricity, options).unwrap();
        assert_normalized(report.eccentric_anomaly_rad);
        assert!(report.absolute_residual_rad <= options.residual_tolerance_rad);
    }

    #[test]
    fn invalid_eccentricity_matrix_is_rejected_by_solver() {
        let options = KeplerSolveOptions::new(1.0e-12, 32).unwrap();
        for eccentricity in [
            -f64::EPSILON,
            1.0,
            1.5,
            f64::NAN,
            f64::INFINITY,
            -f64::INFINITY,
        ] {
            assert!(matches!(
                solve_kepler_elliptic_newton(0.5, eccentricity, options),
                Err(KeplerError::EccentricityOutsideEllipticDomain { .. })
            ));
        }
    }

    #[test]
    fn iteration_cap_error_reports_recomputed_final_residual() {
        let eccentricity = 1.0 - 1.0e-12;
        let mean_anomaly_rad = 0.1;
        let options = KeplerSolveOptions::new(1.0e-30, 1).unwrap();
        let error =
            solve_kepler_elliptic_newton(mean_anomaly_rad, eccentricity, options).unwrap_err();

        match error {
            KeplerError::NonConvergence {
                iterations,
                eccentric_anomaly_rad,
                absolute_residual_rad,
            } => {
                assert_eq!(iterations, 1);
                let normalized_mean_anomaly = mean_anomaly_rad.rem_euclid(TWO_PI);
                let expected_residual = (eccentric_anomaly_rad
                    - eccentricity * eccentric_anomaly_rad.sin()
                    - normalized_mean_anomaly)
                    .abs();
                assert_close(absolute_residual_rad, expected_residual, 1.0e-15);
                assert!(absolute_residual_rad > options.residual_tolerance_rad);
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn nonfinite_mean_anomaly_is_rejected_by_solver() {
        let options = KeplerSolveOptions::new(1.0e-12, 32).unwrap();
        for value in [f64::NAN, f64::INFINITY, -f64::INFINITY] {
            assert!(matches!(
                solve_kepler_elliptic_newton(value, 0.1, options),
                Err(KeplerError::NonfiniteInput {
                    parameter: "mean_anomaly_rad",
                    ..
                })
            ));
        }
    }

    #[test]
    fn true_anomaly_matches_known_quadrant_case() {
        let actual = true_anomaly_from_eccentric_anomaly(PI / 2.0, 0.5).unwrap();
        let expected = 2.0 * PI / 3.0;
        assert_close(actual, expected, 1.0e-15);
        assert_normalized(actual);
    }

    #[test]
    fn zero_eccentricity_true_anomaly_uses_normalized_eccentric_anomaly() {
        for eccentric_anomaly_rad in [-0.25, 0.25, PI, TWO_PI + 0.25] {
            let actual = true_anomaly_from_eccentric_anomaly(eccentric_anomaly_rad, 0.0).unwrap();
            assert_close(actual, eccentric_anomaly_rad.rem_euclid(TWO_PI), 1.0e-15);
        }
    }

    #[test]
    fn true_anomaly_rejects_invalid_inputs() {
        assert!(matches!(
            true_anomaly_from_eccentric_anomaly(f64::NAN, 0.1),
            Err(KeplerError::NonfiniteInput {
                parameter: "eccentric_anomaly_rad",
                ..
            })
        ));
        for eccentricity in [-0.1, 1.0, f64::NAN, f64::INFINITY] {
            assert!(matches!(
                true_anomaly_from_eccentric_anomaly(0.1, eccentricity),
                Err(KeplerError::EccentricityOutsideEllipticDomain { .. })
            ));
        }
    }

    #[test]
    fn radius_matches_periapsis_and_apoapsis_relations() {
        let semi_major_axis_m = 10_000_000.0;
        let eccentricity = 0.25;
        let periapsis =
            radius_from_semimajor_eccentric_anomaly(semi_major_axis_m, eccentricity, 0.0).unwrap();
        let apoapsis =
            radius_from_semimajor_eccentric_anomaly(semi_major_axis_m, eccentricity, PI).unwrap();
        assert_close(periapsis, semi_major_axis_m * (1.0 - eccentricity), 1.0e-9);
        assert_close(apoapsis, semi_major_axis_m * (1.0 + eccentricity), 1.0e-9);
    }

    #[test]
    fn radius_rejects_invalid_axis_eccentricity_and_anomaly() {
        for semi_major_axis_m in [0.0, -1.0] {
            assert!(matches!(
                radius_from_semimajor_eccentric_anomaly(semi_major_axis_m, 0.1, 0.0,),
                Err(KeplerError::NonPositiveInput {
                    parameter: "semi_major_axis_m",
                    ..
                })
            ));
        }
        assert!(matches!(
            radius_from_semimajor_eccentric_anomaly(f64::NAN, 0.1, 0.0),
            Err(KeplerError::NonfiniteInput {
                parameter: "semi_major_axis_m",
                ..
            })
        ));
        assert!(matches!(
            radius_from_semimajor_eccentric_anomaly(1.0, 1.0, 0.0),
            Err(KeplerError::EccentricityOutsideEllipticDomain { .. })
        ));
        assert!(matches!(
            radius_from_semimajor_eccentric_anomaly(1.0, 0.1, f64::INFINITY),
            Err(KeplerError::NonfiniteInput {
                parameter: "eccentric_anomaly_rad",
                ..
            })
        ));
    }

    #[test]
    fn radius_blocks_overflow_and_underflow_to_zero() {
        assert!(matches!(
            radius_from_semimajor_eccentric_anomaly(f64::MAX, 0.5, PI),
            Err(KeplerError::NumericalFailure {
                function: "radius_from_semimajor_eccentric_anomaly",
                ..
            })
        ));

        let eccentricity = 1.0 - f64::EPSILON / 2.0;
        assert!(matches!(
            radius_from_semimajor_eccentric_anomaly(f64::MIN_POSITIVE, eccentricity, 0.0,),
            Err(KeplerError::NumericalFailure {
                function: "radius_from_semimajor_eccentric_anomaly",
                ..
            })
        ));
    }

    #[test]
    fn error_codes_and_parameters_are_stable() {
        let error = KeplerError::EccentricityOutsideEllipticDomain { value: 1.0 };
        assert_eq!(error.code(), "eccentricity_outside_elliptic_domain");

        let error = KeplerError::NonConvergence {
            iterations: 1,
            eccentric_anomaly_rad: 0.0,
            absolute_residual_rad: 1.0,
        };
        assert_eq!(error.code(), "non_convergence");
    }
}
