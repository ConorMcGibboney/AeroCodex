//! Classical orbital-element helpers for bounded, non-singular elliptic states.
//!
//! This module is independently written from standard two-body vector relations.
//! It does not copy or translate Orekit source, tests, fixtures, comments, class
//! hierarchy, control flow, or API architecture.
//!
//! Units and conventions:
//! - position: metres (`m`);
//! - velocity: metres per second (`m/s`);
//! - gravitational parameter: cubic metres per square second (`m^3/s^2`);
//! - specific angular momentum and node vectors: square metres per second (`m^2/s`);
//! - specific orbital energy: square metres per square second (`m^2/s^2`);
//! - eccentricity: dimensionless;
//! - angles: radians.
//!
//! `PositionVelocity` carries no frame or epoch. Callers must provide a state in
//! one consistent inertial-like Cartesian basis at one epoch. No frame transform,
//! time-scale conversion, propagation, perturbation model, or operational orbit
//! determination is performed here.
//!
//! Complete extraction is elliptic-only. Circular/near-circular,
//! equatorial/near-equatorial, zero-angular-momentum, near-parabolic,
//! parabolic, and hyperbolic cases are explicit errors. Longitude-like angles
//! are normalized to `[0, 2*pi)`; inclination is in `[0, pi]`.

use crate::state::{
    position_norm, validate_position_velocity_finite, velocity_norm, AstroStateValidationError,
    GravitationalParameter, PositionVelocity,
};
use core::fmt;
use std::f64::consts::PI;

const TWO_PI: f64 = 2.0 * PI;

/// Classical orbital elements for a non-singular elliptic two-body state.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ClassicalOrbitalElements {
    /// Positive semi-major axis in metres.
    pub semi_major_axis_m: f64,
    /// Eccentricity magnitude in `(0, 1)`.
    pub eccentricity: f64,
    /// Inclination in radians, in `[0, pi]`.
    pub inclination_rad: f64,
    /// Right ascension of the ascending node in radians, in `[0, 2*pi)`.
    pub raan_rad: f64,
    /// Argument of periapsis in radians, in `[0, 2*pi)`.
    pub argument_of_periapsis_rad: f64,
    /// True anomaly in radians, in `[0, 2*pi)`.
    pub true_anomaly_rad: f64,
}

/// Positive finite numerical thresholds for classical-element singularities.
///
/// Angular-momentum and node thresholds are absolute and scale-sensitive.
/// There is intentionally no `Default` implementation: callers must select and
/// document thresholds for their problem scale. These are numerical guards, not
/// validated accuracy requirements.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OrbitalElementTolerances {
    angular_momentum_norm_min_m2_per_s: f64,
    node_norm_min_m2_per_s: f64,
    eccentricity_min: f64,
    specific_energy_abs_min_m2_per_s2: f64,
}

impl OrbitalElementTolerances {
    /// Creates positive finite singularity thresholds.
    ///
    /// Zero is rejected so every extraction path retains a nonzero exclusion
    /// band around each singular boundary.
    pub fn new(
        angular_momentum_norm_min_m2_per_s: f64,
        node_norm_min_m2_per_s: f64,
        eccentricity_min: f64,
        specific_energy_abs_min_m2_per_s2: f64,
    ) -> Result<Self, OrbitalElementError> {
        let value = Self {
            angular_momentum_norm_min_m2_per_s,
            node_norm_min_m2_per_s,
            eccentricity_min,
            specific_energy_abs_min_m2_per_s2,
        };
        value.validate()?;
        Ok(value)
    }

    /// Minimum accepted specific-angular-momentum norm in `m^2/s`.
    #[must_use]
    const fn angular_momentum_norm_min_m2_per_s(self) -> f64 {
        self.angular_momentum_norm_min_m2_per_s
    }

    /// Minimum accepted node-vector norm in `m^2/s`.
    #[must_use]
    const fn node_norm_min_m2_per_s(self) -> f64 {
        self.node_norm_min_m2_per_s
    }

    /// Minimum accepted eccentricity magnitude.
    #[must_use]
    const fn eccentricity_min(self) -> f64 {
        self.eccentricity_min
    }

    /// Minimum accepted absolute specific energy in `m^2/s^2`.
    #[must_use]
    const fn specific_energy_abs_min_m2_per_s2(self) -> f64 {
        self.specific_energy_abs_min_m2_per_s2
    }

    fn validate(self) -> Result<(), OrbitalElementError> {
        validate_positive_tolerance(
            "angular_momentum_norm_min_m2_per_s",
            self.angular_momentum_norm_min_m2_per_s,
        )?;
        validate_positive_tolerance("node_norm_min_m2_per_s", self.node_norm_min_m2_per_s)?;
        validate_positive_tolerance("eccentricity_min", self.eccentricity_min)?;
        validate_positive_tolerance(
            "specific_energy_abs_min_m2_per_s2",
            self.specific_energy_abs_min_m2_per_s2,
        )
    }
}

/// Errors returned by classical orbital-element helpers.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OrbitalElementError {
    /// Shared Cartesian-state validation failed.
    StateValidation(AstroStateValidationError),
    /// A raw vector argument contained NaN or infinity.
    NonfiniteVectorComponent {
        parameter: &'static str,
        component_index: usize,
        value: f64,
    },
    /// A threshold was zero, negative, NaN, or infinite.
    InvalidTolerance { parameter: &'static str, value: f64 },
    /// An intermediate or final scalar/vector result was nonfinite.
    NonfiniteResult { function: &'static str },
    /// Specific angular momentum was inside the singularity band.
    ZeroAngularMomentum {
        norm_m2_per_s: f64,
        minimum_m2_per_s: f64,
    },
    /// The node vector was inside the equatorial singularity band.
    EquatorialSingularity {
        node_norm_m2_per_s: f64,
        minimum_m2_per_s: f64,
    },
    /// Eccentricity was inside the circular singularity band.
    CircularSingularity {
        eccentricity: f64,
        minimum_eccentricity: f64,
    },
    /// Specific energy was too close to zero for bounded extraction.
    NearParabolicBoundary {
        specific_energy_m2_per_s2: f64,
        minimum_abs_specific_energy_m2_per_s2: f64,
    },
    /// Specific energy was nonnegative; elliptic extraction is required.
    NonEllipticOrbit { specific_energy_m2_per_s2: f64 },
    /// Eccentricity did not lie in the supported open interval `(0, 1)`.
    EccentricityOutsideEllipticDomain { eccentricity: f64 },
}

impl OrbitalElementError {
    /// Stable snake-case code for tests and later evidence records.
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::StateValidation(_) => "state_validation",
            Self::NonfiniteVectorComponent { .. } => "nonfinite_vector_component",
            Self::InvalidTolerance { .. } => "invalid_tolerance",
            Self::NonfiniteResult { .. } => "nonfinite_result",
            Self::ZeroAngularMomentum { .. } => "zero_angular_momentum",
            Self::EquatorialSingularity { .. } => "equatorial_singularity",
            Self::CircularSingularity { .. } => "circular_singularity",
            Self::NearParabolicBoundary { .. } => "near_parabolic_boundary",
            Self::NonEllipticOrbit { .. } => "non_elliptic_orbit",
            Self::EccentricityOutsideEllipticDomain { .. } => {
                "eccentricity_outside_elliptic_domain"
            }
        }
    }
}

impl From<AstroStateValidationError> for OrbitalElementError {
    fn from(value: AstroStateValidationError) -> Self {
        Self::StateValidation(value)
    }
}

impl fmt::Display for OrbitalElementError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::StateValidation(error) => write!(formatter, "state validation failed: {error}"),
            Self::NonfiniteVectorComponent {
                parameter,
                component_index,
                value,
            } => write!(
                formatter,
                "vector `{parameter}` component {component_index} must be finite, got {value}"
            ),
            Self::InvalidTolerance { parameter, value } => write!(
                formatter,
                "tolerance `{parameter}` must be positive and finite, got {value}"
            ),
            Self::NonfiniteResult { function } => {
                write!(formatter, "{function} produced a nonfinite result")
            }
            Self::ZeroAngularMomentum {
                norm_m2_per_s,
                minimum_m2_per_s,
            } => write!(
                formatter,
                "specific angular-momentum norm {norm_m2_per_s} m^2/s is not above {minimum_m2_per_s} m^2/s"
            ),
            Self::EquatorialSingularity {
                node_norm_m2_per_s,
                minimum_m2_per_s,
            } => write!(
                formatter,
                "node-vector norm {node_norm_m2_per_s} m^2/s is not above {minimum_m2_per_s} m^2/s"
            ),
            Self::CircularSingularity {
                eccentricity,
                minimum_eccentricity,
            } => write!(
                formatter,
                "eccentricity {eccentricity} is not above {minimum_eccentricity}"
            ),
            Self::NearParabolicBoundary {
                specific_energy_m2_per_s2,
                minimum_abs_specific_energy_m2_per_s2,
            } => write!(
                formatter,
                "specific energy {specific_energy_m2_per_s2} m^2/s^2 lies inside the near-parabolic band +/-{minimum_abs_specific_energy_m2_per_s2} m^2/s^2"
            ),
            Self::NonEllipticOrbit {
                specific_energy_m2_per_s2,
            } => write!(
                formatter,
                "specific energy {specific_energy_m2_per_s2} m^2/s^2 is not negative"
            ),
            Self::EccentricityOutsideEllipticDomain { eccentricity } => write!(
                formatter,
                "eccentricity {eccentricity} is outside the supported interval (0, 1)"
            ),
        }
    }
}

impl std::error::Error for OrbitalElementError {}

/// Computes specific angular momentum, `h = r x v`, in `m^2/s`.
///
/// A radial state may validly return the zero vector; complete extraction blocks
/// zero/near-zero angular momentum separately.
pub fn compute_specific_angular_momentum(
    state: &PositionVelocity,
) -> Result<[f64; 3], OrbitalElementError> {
    validate_position_velocity_finite(state)?;
    checked_cross_product(
        "compute_specific_angular_momentum",
        state.position_m(),
        state.velocity_m_per_s(),
    )
}

/// Computes `n = k x h = [-h_y, h_x, 0]` in `m^2/s`.
///
/// A zero node vector is a valid intermediate; angle extractors reject it.
pub fn compute_node_vector(h_vector_m2_per_s: [f64; 3]) -> Result<[f64; 3], OrbitalElementError> {
    validate_vector3("h_vector_m2_per_s", h_vector_m2_per_s)?;
    ensure_finite_vector3(
        "compute_node_vector",
        [-h_vector_m2_per_s[1], h_vector_m2_per_s[0], 0.0],
    )
}

/// Computes the dimensionless eccentricity vector,
/// `e = (v x h)/mu - r/|r|`.
///
/// This formula supports any finite two-body conic with positive `mu` and
/// nonzero radius. Only complete extraction imposes the elliptic domain.
pub fn compute_eccentricity_vector(
    state: &PositionVelocity,
    mu: GravitationalParameter,
) -> Result<[f64; 3], OrbitalElementError> {
    validate_position_velocity_finite(state)?;
    let radius_m = position_norm(state)?;
    if radius_m == 0.0 {
        return Err(AstroStateValidationError::ZeroPositionRadius.into());
    }

    let h_vector = compute_specific_angular_momentum(state)?;
    let velocity_cross_h = checked_cross_product(
        "compute_eccentricity_vector.velocity_cross_h",
        state.velocity_m_per_s(),
        h_vector,
    )?;
    let position_m = state.position_m();
    let mu_m3_per_s2 = mu.as_m3_per_s2();
    let mut eccentricity_vector = [0.0; 3];
    for (component_index, component) in eccentricity_vector.iter_mut().enumerate() {
        *component = velocity_cross_h[component_index] / mu_m3_per_s2
            - position_m[component_index] / radius_m;
    }
    ensure_finite_vector3("compute_eccentricity_vector", eccentricity_vector)
}

/// Computes two-body specific orbital energy from a Cartesian state.
///
/// Formula and units: `epsilon = v^2 / 2 - mu / r`, where `epsilon` is in
/// `m^2/s^2`, `v` is in `m/s`, `r` is in metres, and `mu` is in `m^3/s^2`.
///
/// Domain: finite position/velocity components, positive finite `mu`, nonzero
/// radius, and finite intermediate arithmetic. This performs no frame transform,
/// time-scale conversion, perturbation modeling, or propagation.
pub fn compute_specific_orbital_energy_from_state(
    state: &PositionVelocity,
    mu: GravitationalParameter,
) -> Result<f64, OrbitalElementError> {
    validate_position_velocity_finite(state)?;
    let radius_m = position_norm(state)?;
    if radius_m == 0.0 {
        return Err(AstroStateValidationError::ZeroPositionRadius.into());
    }

    let speed_m_per_s = velocity_norm(state)?;
    let speed_squared_m2_per_s2 = speed_m_per_s * speed_m_per_s;
    if !speed_squared_m2_per_s2.is_finite() {
        return Err(OrbitalElementError::NonfiniteResult {
            function: "compute_specific_orbital_energy_from_state",
        });
    }

    let energy_m2_per_s2 = 0.5 * speed_squared_m2_per_s2 - mu.as_m3_per_s2() / radius_m;
    if energy_m2_per_s2.is_finite() {
        Ok(energy_m2_per_s2)
    } else {
        Err(OrbitalElementError::NonfiniteResult {
            function: "compute_specific_orbital_energy_from_state",
        })
    }
}

/// Computes positive elliptic semi-major axis from Cartesian specific energy.
///
/// Formula: `a = -mu/(2*epsilon)`. Near-parabolic, parabolic, and hyperbolic
/// states are rejected.
pub fn compute_semimajor_axis_from_state(
    state: &PositionVelocity,
    mu: GravitationalParameter,
    tolerances: OrbitalElementTolerances,
) -> Result<f64, OrbitalElementError> {
    tolerances.validate()?;
    let specific_energy_m2_per_s2 = compute_specific_orbital_energy_from_state(state, mu)?;
    classify_elliptic_specific_energy(specific_energy_m2_per_s2, tolerances)?;

    let denominator = 2.0 * specific_energy_m2_per_s2;
    if !denominator.is_finite() {
        return Err(OrbitalElementError::NonfiniteResult {
            function: "compute_semimajor_axis_from_state",
        });
    }
    let semi_major_axis_m = -mu.as_m3_per_s2() / denominator;
    if semi_major_axis_m.is_finite() && semi_major_axis_m > 0.0 {
        Ok(semi_major_axis_m)
    } else {
        Err(OrbitalElementError::NonfiniteResult {
            function: "compute_semimajor_axis_from_state",
        })
    }
}

/// Computes inclination from a specific-angular-momentum vector.
///
/// The result is `atan2(sqrt(h_x^2 + h_y^2), h_z)` in `[0, pi]`.
pub fn compute_inclination(
    h_vector_m2_per_s: [f64; 3],
    tolerances: OrbitalElementTolerances,
) -> Result<f64, OrbitalElementError> {
    tolerances.validate()?;
    let h_norm_m2_per_s = checked_vector_norm(
        "compute_inclination.h_vector",
        "h_vector_m2_per_s",
        h_vector_m2_per_s,
    )?;
    require_angular_momentum_norm(h_norm_m2_per_s, tolerances)?;

    let horizontal_norm_m2_per_s = checked_vector_norm(
        "compute_inclination.horizontal",
        "h_horizontal_m2_per_s",
        [h_vector_m2_per_s[0], h_vector_m2_per_s[1], 0.0],
    )?;
    ensure_finite_scalar(
        "compute_inclination",
        horizontal_norm_m2_per_s.atan2(h_vector_m2_per_s[2]),
    )
}

/// Computes right ascension of the ascending node.
///
/// The result is `atan2(n_y, n_x)` normalized to `[0, 2*pi)`. Zero or
/// near-zero node vectors are rejected.
pub fn compute_raan(
    node_vector_m2_per_s: [f64; 3],
    tolerances: OrbitalElementTolerances,
) -> Result<f64, OrbitalElementError> {
    tolerances.validate()?;
    validate_vector3("node_vector_m2_per_s", node_vector_m2_per_s)?;
    let node_in_reference_plane = [node_vector_m2_per_s[0], node_vector_m2_per_s[1], 0.0];
    let node_norm_m2_per_s = checked_vector_norm(
        "compute_raan.node_vector",
        "node_vector_m2_per_s",
        node_in_reference_plane,
    )?;
    require_node_norm(node_norm_m2_per_s, tolerances)?;
    normalize_angle_0_to_two_pi(
        "compute_raan",
        node_vector_m2_per_s[1].atan2(node_vector_m2_per_s[0]),
    )
}

/// Computes argument of periapsis from node, eccentricity, and angular-momentum vectors.
///
/// The oriented angle is
/// `atan2(h_hat dot (n_hat x e_hat), n_hat dot e_hat)` and is normalized to
/// `[0, 2*pi)`. All vectors must use the same inertial Cartesian basis.
pub fn compute_argument_of_periapsis(
    node_vector_m2_per_s: [f64; 3],
    eccentricity_vector: [f64; 3],
    h_vector_m2_per_s: [f64; 3],
    tolerances: OrbitalElementTolerances,
) -> Result<f64, OrbitalElementError> {
    tolerances.validate()?;

    validate_vector3("node_vector_m2_per_s", node_vector_m2_per_s)?;
    let node_in_reference_plane = [node_vector_m2_per_s[0], node_vector_m2_per_s[1], 0.0];
    let node_norm_m2_per_s = checked_vector_norm(
        "compute_argument_of_periapsis.node_vector",
        "node_vector_m2_per_s",
        node_in_reference_plane,
    )?;
    require_node_norm(node_norm_m2_per_s, tolerances)?;

    let eccentricity = checked_vector_norm(
        "compute_argument_of_periapsis.eccentricity_vector",
        "eccentricity_vector",
        eccentricity_vector,
    )?;
    require_eccentricity(eccentricity, tolerances)?;

    let h_norm_m2_per_s = checked_vector_norm(
        "compute_argument_of_periapsis.h_vector",
        "h_vector_m2_per_s",
        h_vector_m2_per_s,
    )?;
    require_angular_momentum_norm(h_norm_m2_per_s, tolerances)?;

    let node_unit = checked_unit_vector(
        "compute_argument_of_periapsis.node_unit",
        node_in_reference_plane,
        node_norm_m2_per_s,
    )?;
    let eccentricity_unit = checked_unit_vector(
        "compute_argument_of_periapsis.eccentricity_unit",
        eccentricity_vector,
        eccentricity,
    )?;
    let h_unit = checked_unit_vector(
        "compute_argument_of_periapsis.h_unit",
        h_vector_m2_per_s,
        h_norm_m2_per_s,
    )?;

    oriented_angle_0_to_two_pi(
        "compute_argument_of_periapsis",
        node_unit,
        eccentricity_unit,
        h_unit,
    )
}

/// Computes true anomaly from position, eccentricity, and angular-momentum direction.
///
/// The oriented angle is
/// `atan2(h_hat dot (e_hat x r_hat), e_hat dot r_hat)` and is normalized to
/// `[0, 2*pi)`. Circular and zero-angular-momentum states are rejected.
pub fn compute_true_anomaly(
    state: &PositionVelocity,
    eccentricity_vector: [f64; 3],
    tolerances: OrbitalElementTolerances,
) -> Result<f64, OrbitalElementError> {
    tolerances.validate()?;
    validate_position_velocity_finite(state)?;

    let radius_m = position_norm(state)?;
    if radius_m == 0.0 {
        return Err(AstroStateValidationError::ZeroPositionRadius.into());
    }

    let eccentricity = checked_vector_norm(
        "compute_true_anomaly.eccentricity_vector",
        "eccentricity_vector",
        eccentricity_vector,
    )?;
    require_eccentricity(eccentricity, tolerances)?;

    let h_vector_m2_per_s = compute_specific_angular_momentum(state)?;
    let h_norm_m2_per_s = checked_vector_norm(
        "compute_true_anomaly.h_vector",
        "h_vector_m2_per_s",
        h_vector_m2_per_s,
    )?;
    require_angular_momentum_norm(h_norm_m2_per_s, tolerances)?;

    let eccentricity_unit = checked_unit_vector(
        "compute_true_anomaly.eccentricity_unit",
        eccentricity_vector,
        eccentricity,
    )?;
    let position_unit = checked_unit_vector(
        "compute_true_anomaly.position_unit",
        state.position_m(),
        radius_m,
    )?;
    let h_unit = checked_unit_vector(
        "compute_true_anomaly.h_unit",
        h_vector_m2_per_s,
        h_norm_m2_per_s,
    )?;

    oriented_angle_0_to_two_pi(
        "compute_true_anomaly",
        eccentricity_unit,
        position_unit,
        h_unit,
    )
}

/// Extracts complete classical elements for a non-singular elliptic Cartesian state.
///
/// Domain:
/// - finite position and velocity;
/// - positive finite `mu`;
/// - nonzero radius;
/// - negative specific energy outside the near-parabolic exclusion band;
/// - eccentricity in `(eccentricity_min, 1)`;
/// - angular-momentum and node norms above configured thresholds.
///
/// Callers remain responsible for a consistent inertial-like frame and epoch.
pub fn compute_orbital_elements_basic(
    state: &PositionVelocity,
    mu: GravitationalParameter,
    tolerances: OrbitalElementTolerances,
) -> Result<ClassicalOrbitalElements, OrbitalElementError> {
    tolerances.validate()?;
    validate_position_velocity_finite(state)?;

    let semi_major_axis_m = compute_semimajor_axis_from_state(state, mu, tolerances)?;

    let h_vector_m2_per_s = compute_specific_angular_momentum(state)?;
    let h_norm_m2_per_s = checked_vector_norm(
        "compute_orbital_elements_basic.h_vector",
        "h_vector_m2_per_s",
        h_vector_m2_per_s,
    )?;
    require_angular_momentum_norm(h_norm_m2_per_s, tolerances)?;

    let eccentricity_vector = compute_eccentricity_vector(state, mu)?;
    let eccentricity = checked_vector_norm(
        "compute_orbital_elements_basic.eccentricity_vector",
        "eccentricity_vector",
        eccentricity_vector,
    )?;
    require_eccentricity(eccentricity, tolerances)?;
    if eccentricity >= 1.0 {
        return Err(OrbitalElementError::EccentricityOutsideEllipticDomain { eccentricity });
    }

    let node_vector_m2_per_s = compute_node_vector(h_vector_m2_per_s)?;

    Ok(ClassicalOrbitalElements {
        semi_major_axis_m,
        eccentricity,
        inclination_rad: compute_inclination(h_vector_m2_per_s, tolerances)?,
        raan_rad: compute_raan(node_vector_m2_per_s, tolerances)?,
        argument_of_periapsis_rad: compute_argument_of_periapsis(
            node_vector_m2_per_s,
            eccentricity_vector,
            h_vector_m2_per_s,
            tolerances,
        )?,
        true_anomaly_rad: compute_true_anomaly(state, eccentricity_vector, tolerances)?,
    })
}

fn validate_positive_tolerance(
    parameter: &'static str,
    value: f64,
) -> Result<(), OrbitalElementError> {
    if value.is_finite() && value > 0.0 {
        Ok(())
    } else {
        Err(OrbitalElementError::InvalidTolerance { parameter, value })
    }
}

fn classify_elliptic_specific_energy(
    specific_energy_m2_per_s2: f64,
    tolerances: OrbitalElementTolerances,
) -> Result<(), OrbitalElementError> {
    if !specific_energy_m2_per_s2.is_finite() {
        return Err(OrbitalElementError::NonfiniteResult {
            function: "classify_elliptic_specific_energy",
        });
    }
    if specific_energy_m2_per_s2.abs() <= tolerances.specific_energy_abs_min_m2_per_s2() {
        return Err(OrbitalElementError::NearParabolicBoundary {
            specific_energy_m2_per_s2,
            minimum_abs_specific_energy_m2_per_s2: tolerances.specific_energy_abs_min_m2_per_s2(),
        });
    }
    if specific_energy_m2_per_s2 >= 0.0 {
        return Err(OrbitalElementError::NonEllipticOrbit {
            specific_energy_m2_per_s2,
        });
    }
    Ok(())
}

fn require_angular_momentum_norm(
    norm_m2_per_s: f64,
    tolerances: OrbitalElementTolerances,
) -> Result<(), OrbitalElementError> {
    if norm_m2_per_s <= tolerances.angular_momentum_norm_min_m2_per_s() {
        Err(OrbitalElementError::ZeroAngularMomentum {
            norm_m2_per_s,
            minimum_m2_per_s: tolerances.angular_momentum_norm_min_m2_per_s(),
        })
    } else {
        Ok(())
    }
}

fn require_node_norm(
    node_norm_m2_per_s: f64,
    tolerances: OrbitalElementTolerances,
) -> Result<(), OrbitalElementError> {
    if node_norm_m2_per_s <= tolerances.node_norm_min_m2_per_s() {
        Err(OrbitalElementError::EquatorialSingularity {
            node_norm_m2_per_s,
            minimum_m2_per_s: tolerances.node_norm_min_m2_per_s(),
        })
    } else {
        Ok(())
    }
}

fn require_eccentricity(
    eccentricity: f64,
    tolerances: OrbitalElementTolerances,
) -> Result<(), OrbitalElementError> {
    if eccentricity <= tolerances.eccentricity_min() {
        Err(OrbitalElementError::CircularSingularity {
            eccentricity,
            minimum_eccentricity: tolerances.eccentricity_min(),
        })
    } else {
        Ok(())
    }
}

fn validate_vector3(parameter: &'static str, value: [f64; 3]) -> Result<(), OrbitalElementError> {
    for (component_index, component) in value.iter().copied().enumerate() {
        if !component.is_finite() {
            return Err(OrbitalElementError::NonfiniteVectorComponent {
                parameter,
                component_index,
                value: component,
            });
        }
    }
    Ok(())
}

fn ensure_finite_scalar(function: &'static str, value: f64) -> Result<f64, OrbitalElementError> {
    if value.is_finite() {
        Ok(value)
    } else {
        Err(OrbitalElementError::NonfiniteResult { function })
    }
}

fn ensure_finite_vector3(
    function: &'static str,
    value: [f64; 3],
) -> Result<[f64; 3], OrbitalElementError> {
    if value.iter().all(|component| component.is_finite()) {
        Ok(value)
    } else {
        Err(OrbitalElementError::NonfiniteResult { function })
    }
}

fn checked_cross_product(
    function: &'static str,
    left: [f64; 3],
    right: [f64; 3],
) -> Result<[f64; 3], OrbitalElementError> {
    validate_vector3("left_vector", left)?;
    validate_vector3("right_vector", right)?;
    ensure_finite_vector3(
        function,
        [
            left[1] * right[2] - left[2] * right[1],
            left[2] * right[0] - left[0] * right[2],
            left[0] * right[1] - left[1] * right[0],
        ],
    )
}

fn checked_dot_product(
    function: &'static str,
    left: [f64; 3],
    right: [f64; 3],
) -> Result<f64, OrbitalElementError> {
    validate_vector3("left_vector", left)?;
    validate_vector3("right_vector", right)?;
    ensure_finite_scalar(
        function,
        left[0] * right[0] + left[1] * right[1] + left[2] * right[2],
    )
}

fn checked_vector_norm(
    function: &'static str,
    parameter: &'static str,
    value: [f64; 3],
) -> Result<f64, OrbitalElementError> {
    validate_vector3(parameter, value)?;

    let mut scale = 0.0_f64;
    for component in value.iter().copied() {
        scale = scale.max(component.abs());
    }
    if scale == 0.0 {
        return Ok(0.0);
    }

    let mut scaled_sum = 0.0_f64;
    for component in value.iter().copied() {
        let ratio = component / scale;
        scaled_sum += ratio * ratio;
    }
    ensure_finite_scalar(function, scale * scaled_sum.sqrt())
}

fn checked_unit_vector(
    function: &'static str,
    value: [f64; 3],
    norm: f64,
) -> Result<[f64; 3], OrbitalElementError> {
    if !norm.is_finite() || norm <= 0.0 {
        return Err(OrbitalElementError::NonfiniteResult { function });
    }
    ensure_finite_vector3(
        function,
        [value[0] / norm, value[1] / norm, value[2] / norm],
    )
}

fn oriented_angle_0_to_two_pi(
    function: &'static str,
    from_unit: [f64; 3],
    to_unit: [f64; 3],
    positive_normal_unit: [f64; 3],
) -> Result<f64, OrbitalElementError> {
    let cross = checked_cross_product(function, from_unit, to_unit)?;
    let sine = checked_dot_product(function, cross, positive_normal_unit)?;
    let cosine = checked_dot_product(function, from_unit, to_unit)?.clamp(-1.0, 1.0);
    normalize_angle_0_to_two_pi(function, sine.atan2(cosine))
}

fn normalize_angle_0_to_two_pi(
    function: &'static str,
    angle_rad: f64,
) -> Result<f64, OrbitalElementError> {
    if !angle_rad.is_finite() {
        return Err(OrbitalElementError::NonfiniteResult { function });
    }
    let mut normalized = angle_rad % TWO_PI;
    if normalized < 0.0 {
        normalized += TWO_PI;
    }
    ensure_finite_scalar(function, normalized)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn earth_mu_m3_per_s2() -> f64 {
        3.986_004_418e14
    }

    fn scalar_tolerance() -> f64 {
        1.0e-12
    }

    fn angle_tolerance_rad() -> f64 {
        5.0e-12
    }

    fn test_tolerances() -> OrbitalElementTolerances {
        OrbitalElementTolerances::new(1.0e-12, 1.0e-12, 1.0e-12, 1.0e-12)
            .expect("test tolerances should be positive and finite")
    }

    fn assert_close(actual: f64, expected: f64, tolerance: f64) {
        assert!(
            (actual - expected).abs() <= tolerance,
            "actual={actual}, expected={expected}, tolerance={tolerance}"
        );
    }

    fn assert_relative_close(actual: f64, expected: f64, relative_tolerance: f64) {
        assert_close(
            actual,
            expected,
            relative_tolerance * expected.abs().max(1.0),
        );
    }

    fn assert_vector_close(actual: [f64; 3], expected: [f64; 3], tolerance: f64) {
        for (actual_component, expected_component) in actual.into_iter().zip(expected) {
            assert_close(actual_component, expected_component, tolerance);
        }
    }

    fn angular_distance(left: f64, right: f64) -> f64 {
        let wrapped = (left - right).rem_euclid(TWO_PI);
        wrapped.min(TWO_PI - wrapped)
    }

    fn assert_angle_close(actual: f64, expected: f64, tolerance: f64) {
        assert!(
            angular_distance(actual, expected) <= tolerance,
            "actual={actual}, expected={expected}, tolerance={tolerance}"
        );
    }

    fn earth_mu() -> GravitationalParameter {
        GravitationalParameter::new(earth_mu_m3_per_s2()).unwrap()
    }

    fn circular_state(radius_m: f64) -> PositionVelocity {
        let speed_m_per_s = (earth_mu_m3_per_s2() / radius_m).sqrt();
        PositionVelocity::new([radius_m, 0.0, 0.0], [0.0, speed_m_per_s, 0.0]).unwrap()
    }

    fn synthetic_state_from_classical(
        semi_major_axis_m: f64,
        eccentricity: f64,
        inclination_rad: f64,
        raan_rad: f64,
        argument_of_periapsis_rad: f64,
        true_anomaly_rad: f64,
        mu_m3_per_s2: f64,
    ) -> PositionVelocity {
        let parameter_m = semi_major_axis_m * (1.0 - eccentricity * eccentricity);
        let radius_m = parameter_m / (1.0 + eccentricity * true_anomaly_rad.cos());
        let position_perifocal_m = [
            radius_m * true_anomaly_rad.cos(),
            radius_m * true_anomaly_rad.sin(),
            0.0,
        ];
        let speed_scale_m_per_s = (mu_m3_per_s2 / parameter_m).sqrt();
        let velocity_perifocal_m_per_s = [
            -speed_scale_m_per_s * true_anomaly_rad.sin(),
            speed_scale_m_per_s * (eccentricity + true_anomaly_rad.cos()),
            0.0,
        ];

        let (sin_raan, cos_raan) = raan_rad.sin_cos();
        let (sin_inclination, cos_inclination) = inclination_rad.sin_cos();
        let (sin_argument, cos_argument) = argument_of_periapsis_rad.sin_cos();
        let rotation = [
            [
                cos_raan * cos_argument - sin_raan * sin_argument * cos_inclination,
                -cos_raan * sin_argument - sin_raan * cos_argument * cos_inclination,
                sin_raan * sin_inclination,
            ],
            [
                sin_raan * cos_argument + cos_raan * sin_argument * cos_inclination,
                -sin_raan * sin_argument + cos_raan * cos_argument * cos_inclination,
                -cos_raan * sin_inclination,
            ],
            [
                sin_argument * sin_inclination,
                cos_argument * sin_inclination,
                cos_inclination,
            ],
        ];

        let rotate = |vector: [f64; 3]| -> [f64; 3] {
            [
                rotation[0][0] * vector[0]
                    + rotation[0][1] * vector[1]
                    + rotation[0][2] * vector[2],
                rotation[1][0] * vector[0]
                    + rotation[1][1] * vector[1]
                    + rotation[1][2] * vector[2],
                rotation[2][0] * vector[0]
                    + rotation[2][1] * vector[1]
                    + rotation[2][2] * vector[2],
            ]
        };

        PositionVelocity::new(
            rotate(position_perifocal_m),
            rotate(velocity_perifocal_m_per_s),
        )
        .expect("synthetic classical state should be finite")
    }

    fn unchecked_tolerances(
        angular_momentum_norm_min_m2_per_s: f64,
        node_norm_min_m2_per_s: f64,
        eccentricity_min: f64,
        specific_energy_abs_min_m2_per_s2: f64,
    ) -> OrbitalElementTolerances {
        OrbitalElementTolerances {
            angular_momentum_norm_min_m2_per_s,
            node_norm_min_m2_per_s,
            eccentricity_min,
            specific_energy_abs_min_m2_per_s2,
        }
    }

    #[test]
    fn specific_angular_momentum_vector_for_simple_state() {
        let state = PositionVelocity::new([1.0, 0.0, 0.0], [0.0, 2.0, 0.0]).unwrap();
        assert_vector_close(
            compute_specific_angular_momentum(&state).unwrap(),
            [0.0, 0.0, 2.0],
            0.0,
        );
    }

    #[test]
    fn node_vector_has_direct_expected_value() {
        assert_vector_close(
            compute_node_vector([2.0, -3.0, 4.0]).unwrap(),
            [3.0, 2.0, 0.0],
            0.0,
        );
    }

    #[test]
    fn eccentricity_vector_for_simple_periapsis_state() {
        let mu = GravitationalParameter::new(1.0).unwrap();
        let semi_major_axis_m = 10.0;
        let eccentricity = 0.2;
        let radius_m = semi_major_axis_m * (1.0 - eccentricity);
        let speed_m_per_s = ((1.0_f64 + eccentricity) / radius_m).sqrt();
        let state = PositionVelocity::new([radius_m, 0.0, 0.0], [0.0, speed_m_per_s, 0.0]).unwrap();
        assert_vector_close(
            compute_eccentricity_vector(&state, mu).unwrap(),
            [eccentricity, 0.0, 0.0],
            1.0e-14,
        );
    }

    #[test]
    fn specific_orbital_energy_matches_circular_orbit_expectation() {
        let radius_m = 7_000_000.0;
        let state = circular_state(radius_m);
        let mu = earth_mu();
        let energy = compute_specific_orbital_energy_from_state(&state, mu).unwrap();
        let expected = -earth_mu_m3_per_s2() / (2.0 * radius_m);
        assert!((energy - expected).abs() < 1.0e-6 * expected.abs());
    }

    #[test]
    fn specific_orbital_energy_blocks_zero_radius_and_speed_square_overflow() {
        let mu = earth_mu();
        let zero_radius = PositionVelocity::new([0.0, 0.0, 0.0], [0.0, 1.0, 0.0]).unwrap();
        assert!(matches!(
            compute_specific_orbital_energy_from_state(&zero_radius, mu),
            Err(OrbitalElementError::StateValidation(
                AstroStateValidationError::ZeroPositionRadius
            ))
        ));

        let overflow_speed =
            PositionVelocity::new([7_000_000.0, 0.0, 0.0], [f64::MAX, 0.0, 0.0]).unwrap();
        assert!(matches!(
            compute_specific_orbital_energy_from_state(&overflow_speed, mu),
            Err(OrbitalElementError::NonfiniteResult {
                function: "compute_specific_orbital_energy_from_state"
            })
        ));
    }

    #[test]
    fn semimajor_axis_from_energy_for_simple_bound_orbit() {
        let mu = GravitationalParameter::new(1.0).unwrap();
        let state = PositionVelocity::new([8.0, 0.0, 0.0], [0.0, 0.15_f64.sqrt(), 0.0]).unwrap();
        assert_close(
            compute_semimajor_axis_from_state(&state, mu, test_tolerances()).unwrap(),
            10.0,
            scalar_tolerance(),
        );
    }

    #[test]
    fn semimajor_axis_helper_blocks_near_parabolic_and_hyperbolic_states() {
        let mu = GravitationalParameter::new(1.0).unwrap();
        let near_parabolic =
            PositionVelocity::new([1.0, 0.0, 0.0], [0.0, 2.0_f64.sqrt(), 0.0]).unwrap();
        let hyperbolic = PositionVelocity::new([1.0, 0.0, 0.0], [0.0, 2.0, 0.0]).unwrap();

        assert_eq!(
            compute_semimajor_axis_from_state(&near_parabolic, mu, test_tolerances())
                .unwrap_err()
                .code(),
            "near_parabolic_boundary"
        );
        assert_eq!(
            compute_semimajor_axis_from_state(&hyperbolic, mu, test_tolerances())
                .unwrap_err()
                .code(),
            "non_elliptic_orbit"
        );
    }

    #[test]
    fn inclination_covers_prograde_polar_and_retrograde_geometry() {
        let tolerances = test_tolerances();
        assert_angle_close(
            compute_inclination([0.0, 0.0, 2.0], tolerances).unwrap(),
            0.0,
            angle_tolerance_rad(),
        );
        assert_angle_close(
            compute_inclination([0.0, 2.0, 0.0], tolerances).unwrap(),
            PI / 2.0,
            angle_tolerance_rad(),
        );
        assert_angle_close(
            compute_inclination([0.0, 3.0_f64.sqrt() / 2.0, -0.5], tolerances).unwrap(),
            2.0 * PI / 3.0,
            angle_tolerance_rad(),
        );
    }

    #[test]
    fn raan_quadrants_are_normalized_to_zero_to_two_pi() {
        let tolerances = test_tolerances();
        let cases = [
            ([1.0, 1.0, 0.0], PI / 4.0),
            ([-1.0, 1.0, 0.0], 3.0 * PI / 4.0),
            ([-1.0, -1.0, 0.0], 5.0 * PI / 4.0),
            ([1.0, -1.0, 0.0], 7.0 * PI / 4.0),
        ];
        for (node_vector, expected) in cases {
            let actual = compute_raan(node_vector, tolerances).unwrap();
            assert!((0.0..TWO_PI).contains(&actual));
            assert_angle_close(actual, expected, angle_tolerance_rad());
        }
    }

    #[test]
    fn raan_rejects_node_vector_with_only_out_of_plane_component() {
        let error = compute_raan([0.0, 0.0, 1.0], test_tolerances()).unwrap_err();
        assert_eq!(error.code(), "equatorial_singularity");
    }

    #[test]
    fn argument_of_periapsis_quadrants_use_oriented_angle() {
        let tolerances = test_tolerances();
        let node_vector = [1.0, 0.0, 0.0];
        let h_vector = [0.0, 0.0, 2.0];
        let angles = [PI / 4.0, 3.0 * PI / 4.0, 5.0 * PI / 4.0, 7.0 * PI / 4.0];
        for expected in angles {
            let eccentricity_vector = [0.2 * expected.cos(), 0.2 * expected.sin(), 0.0];
            let actual = compute_argument_of_periapsis(
                node_vector,
                eccentricity_vector,
                h_vector,
                tolerances,
            )
            .unwrap();
            assert!((0.0..TWO_PI).contains(&actual));
            assert_angle_close(actual, expected, angle_tolerance_rad());
        }
    }

    #[test]
    fn argument_of_periapsis_direct_singularity_paths_are_explicit() {
        let tolerances = test_tolerances();
        assert_eq!(
            compute_argument_of_periapsis(
                [0.0, 0.0, 0.0],
                [0.2, 0.0, 0.0],
                [0.0, 0.0, 1.0],
                tolerances,
            )
            .unwrap_err()
            .code(),
            "equatorial_singularity"
        );
        assert_eq!(
            compute_argument_of_periapsis(
                [1.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 1.0],
                tolerances,
            )
            .unwrap_err()
            .code(),
            "circular_singularity"
        );
        assert_eq!(
            compute_argument_of_periapsis(
                [1.0, 0.0, 0.0],
                [0.2, 0.0, 0.0],
                [0.0, 0.0, 0.0],
                tolerances,
            )
            .unwrap_err()
            .code(),
            "zero_angular_momentum"
        );
    }

    #[test]
    fn true_anomaly_quadrants_use_oriented_angle() {
        let tolerances = test_tolerances();
        let eccentricity_vector = [0.2, 0.0, 0.0];
        let angles = [PI / 4.0, 3.0 * PI / 4.0, 5.0 * PI / 4.0, 7.0 * PI / 4.0];
        for expected in angles {
            let position = [8.0 * expected.cos(), 8.0 * expected.sin(), 0.0];
            let velocity = [-expected.sin(), expected.cos(), 0.0];
            let state = PositionVelocity::new(position, velocity).unwrap();
            let actual = compute_true_anomaly(&state, eccentricity_vector, tolerances).unwrap();
            assert!((0.0..TWO_PI).contains(&actual));
            assert_angle_close(actual, expected, angle_tolerance_rad());
        }
    }

    #[test]
    fn complete_elements_match_synthetic_non_singular_elliptic_state() {
        let expected_semi_major_axis_m = 12_000_000.0;
        let expected_eccentricity = 0.3;
        let expected_inclination_rad = 50.0_f64.to_radians();
        let expected_raan_rad = 120.0_f64.to_radians();
        let expected_argument_rad = 250.0_f64.to_radians();
        let expected_true_anomaly_rad = 300.0_f64.to_radians();
        let state = synthetic_state_from_classical(
            expected_semi_major_axis_m,
            expected_eccentricity,
            expected_inclination_rad,
            expected_raan_rad,
            expected_argument_rad,
            expected_true_anomaly_rad,
            earth_mu_m3_per_s2(),
        );
        let mu = GravitationalParameter::new(earth_mu_m3_per_s2()).unwrap();
        let elements = compute_orbital_elements_basic(&state, mu, test_tolerances()).unwrap();

        assert_relative_close(
            elements.semi_major_axis_m,
            expected_semi_major_axis_m,
            5.0e-12,
        );
        assert_close(elements.eccentricity, expected_eccentricity, 5.0e-13);
        assert_angle_close(
            elements.inclination_rad,
            expected_inclination_rad,
            angle_tolerance_rad(),
        );
        assert_angle_close(elements.raan_rad, expected_raan_rad, angle_tolerance_rad());
        assert_angle_close(
            elements.argument_of_periapsis_rad,
            expected_argument_rad,
            angle_tolerance_rad(),
        );
        assert_angle_close(
            elements.true_anomaly_rad,
            expected_true_anomaly_rad,
            angle_tolerance_rad(),
        );
    }

    #[test]
    fn zero_angular_momentum_is_an_explicit_singularity() {
        let state = PositionVelocity::new([1.0, 0.0, 0.0], [0.1, 0.0, 0.0]).unwrap();
        let mu = GravitationalParameter::new(1.0).unwrap();
        let error = compute_orbital_elements_basic(&state, mu, test_tolerances()).unwrap_err();
        assert_eq!(error.code(), "zero_angular_momentum");
    }

    #[test]
    fn near_parabolic_energy_boundary_is_blocked_before_angle_extraction() {
        let state = PositionVelocity::new([1.0, 0.0, 0.0], [0.0, 2.0_f64.sqrt(), 0.0]).unwrap();
        let mu = GravitationalParameter::new(1.0).unwrap();
        let error = compute_orbital_elements_basic(&state, mu, test_tolerances()).unwrap_err();
        assert_eq!(error.code(), "near_parabolic_boundary");
    }

    #[test]
    fn hyperbolic_state_is_rejected_explicitly() {
        let state = PositionVelocity::new([1.0, 0.0, 0.0], [0.0, 2.0, 0.5]).unwrap();
        let mu = GravitationalParameter::new(1.0).unwrap();
        let error = compute_orbital_elements_basic(&state, mu, test_tolerances()).unwrap_err();
        assert_eq!(error.code(), "non_elliptic_orbit");
    }

    #[test]
    fn near_equatorial_non_circular_state_blocks_raan() {
        let state = synthetic_state_from_classical(10.0, 0.2, 1.0e-14, 0.7, 0.4, 0.2, 1.0);
        let mu = GravitationalParameter::new(1.0).unwrap();
        let error = compute_orbital_elements_basic(&state, mu, test_tolerances()).unwrap_err();
        assert_eq!(error.code(), "equatorial_singularity");
    }

    #[test]
    fn near_circular_inclined_state_blocks_apsidal_angles() {
        let state = synthetic_state_from_classical(10.0, 1.0e-14, 0.5, 0.7, 0.4, 0.2, 1.0);
        let mu = GravitationalParameter::new(1.0).unwrap();
        let error = compute_orbital_elements_basic(&state, mu, test_tolerances()).unwrap_err();
        assert_eq!(error.code(), "circular_singularity");
    }

    #[test]
    fn exact_equatorial_non_circular_state_blocks_raan() {
        let state = synthetic_state_from_classical(10.0, 0.2, 0.0, 0.7, 0.4, 0.2, 1.0);
        let mu = GravitationalParameter::new(1.0).unwrap();
        let error = compute_orbital_elements_basic(&state, mu, test_tolerances()).unwrap_err();
        assert_eq!(error.code(), "equatorial_singularity");
    }

    #[test]
    fn exact_circular_inclined_state_blocks_apsidal_angles() {
        let state = synthetic_state_from_classical(10.0, 0.0, 0.5, 0.7, 0.4, 0.2, 1.0);
        let mu = GravitationalParameter::new(1.0).unwrap();
        let error = compute_orbital_elements_basic(&state, mu, test_tolerances()).unwrap_err();
        assert_eq!(error.code(), "circular_singularity");
    }

    #[test]
    fn tolerance_constructor_rejects_zero_negative_nan_and_infinity_for_every_field() {
        let invalid_values = [0.0, -1.0, f64::NAN, f64::INFINITY];
        for invalid_value in invalid_values {
            let cases = [
                [invalid_value, 1.0e-12, 1.0e-12, 1.0e-12],
                [1.0e-12, invalid_value, 1.0e-12, 1.0e-12],
                [1.0e-12, 1.0e-12, invalid_value, 1.0e-12],
                [1.0e-12, 1.0e-12, 1.0e-12, invalid_value],
            ];
            for values in cases {
                let error =
                    OrbitalElementTolerances::new(values[0], values[1], values[2], values[3])
                        .unwrap_err();
                assert_eq!(error.code(), "invalid_tolerance");
            }
        }
    }

    #[test]
    fn every_threshold_accepting_public_helper_revalidates_tolerances() {
        let state = PositionVelocity::new([8.0, 0.0, 0.0], [0.0, 0.15_f64.sqrt(), 0.1]).unwrap();
        let mu = GravitationalParameter::new(1.0).unwrap();
        let h_vector = compute_specific_angular_momentum(&state).unwrap();
        let node_vector = compute_node_vector(h_vector).unwrap();
        let eccentricity_vector = compute_eccentricity_vector(&state, mu).unwrap();

        let invalid_energy = unchecked_tolerances(1.0e-12, 1.0e-12, 1.0e-12, -1.0);
        assert_eq!(
            compute_semimajor_axis_from_state(&state, mu, invalid_energy)
                .unwrap_err()
                .code(),
            "invalid_tolerance"
        );

        let invalid_h = unchecked_tolerances(f64::NAN, 1.0e-12, 1.0e-12, 1.0e-12);
        assert_eq!(
            compute_inclination(h_vector, invalid_h).unwrap_err().code(),
            "invalid_tolerance"
        );

        let invalid_node = unchecked_tolerances(1.0e-12, f64::INFINITY, 1.0e-12, 1.0e-12);
        assert_eq!(
            compute_raan(node_vector, invalid_node).unwrap_err().code(),
            "invalid_tolerance"
        );

        let invalid_eccentricity = unchecked_tolerances(1.0e-12, 1.0e-12, 0.0, 1.0e-12);
        assert_eq!(
            compute_argument_of_periapsis(
                node_vector,
                eccentricity_vector,
                h_vector,
                invalid_eccentricity,
            )
            .unwrap_err()
            .code(),
            "invalid_tolerance"
        );
        assert_eq!(
            compute_true_anomaly(&state, eccentricity_vector, invalid_eccentricity)
                .unwrap_err()
                .code(),
            "invalid_tolerance"
        );

        let invalid_full = unchecked_tolerances(1.0e-12, -1.0, 1.0e-12, 1.0e-12);
        assert_eq!(
            compute_orbital_elements_basic(&state, mu, invalid_full)
                .unwrap_err()
                .code(),
            "invalid_tolerance"
        );
    }

    #[test]
    fn nonfinite_raw_vector_inputs_are_rejected() {
        let error = compute_node_vector([0.0, f64::NAN, 1.0]).unwrap_err();
        assert_eq!(error.code(), "nonfinite_vector_component");
    }

    #[test]
    fn finite_input_overflow_returns_error_instead_of_nan_output() {
        let state =
            PositionVelocity::new([f64::MAX, f64::MAX, 0.0], [f64::MAX, -f64::MAX, 0.0]).unwrap();
        let error = compute_specific_angular_momentum(&state).unwrap_err();
        assert_eq!(error.code(), "nonfinite_result");
    }

    #[test]
    fn zero_position_radius_is_rejected_without_nan_output() {
        let state = PositionVelocity::new([0.0, 0.0, 0.0], [0.0, 1.0, 0.0]).unwrap();
        let mu = GravitationalParameter::new(1.0).unwrap();
        let error = compute_eccentricity_vector(&state, mu).unwrap_err();
        assert_eq!(error.code(), "state_validation");
    }

    #[test]
    fn explicit_tolerances_preserve_units_and_positive_values() {
        let tolerances = test_tolerances();
        assert!(
            tolerances.angular_momentum_norm_min_m2_per_s().is_finite()
                && tolerances.angular_momentum_norm_min_m2_per_s() > 0.0
        );
        assert!(
            tolerances.node_norm_min_m2_per_s().is_finite()
                && tolerances.node_norm_min_m2_per_s() > 0.0
        );
        assert!(tolerances.eccentricity_min().is_finite());
        assert!(tolerances.eccentricity_min() > 0.0);
        assert!(
            tolerances.specific_energy_abs_min_m2_per_s2().is_finite()
                && tolerances.specific_energy_abs_min_m2_per_s2() > 0.0
        );
    }
}
