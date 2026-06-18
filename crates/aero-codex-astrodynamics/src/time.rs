//! Label-preserving astrodynamics time primitives.
//!
//! This module stores a time-scale label and a scalar second offset from a
//! caller-defined reference epoch. It deliberately does not contain a calendar,
//! a leap-second table, relativistic time-scale conversion, or GPS and TDB
//! conversion models.
//!
//! Same-label UTC offsets are **label-preserving scalar arithmetic only**. They
//! are not civil UTC arithmetic, do not account for leap seconds, and must not be
//! used to answer leap-second-sensitive questions.

use aero_codex_core::{validation, AeroError, AeroResult};
use core::cmp::Ordering;

/// Supported astrodynamics time-scale labels.
///
/// The labels are identifiers attached to epochs and states. They are not a
/// conversion engine. `TdbLabelOnly` and `GpsLabelOnly` are placeholders for
/// future reviewed policies and must not be treated as implemented time scales.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AstroTimeScale {
    /// International Atomic Time label.
    Tai,
    /// Coordinated Universal Time label; no calendar or leap-second model is present.
    Utc,
    /// Terrestrial Time label.
    Tt,
    /// Barycentric Dynamical Time placeholder label.
    TdbLabelOnly,
    /// Global Positioning System time placeholder label.
    GpsLabelOnly,
}

impl AstroTimeScale {
    /// Stable display label for reports, tests, and future oracle records.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Tai => "TAI",
            Self::Utc => "UTC",
            Self::Tt => "TT",
            Self::TdbLabelOnly => "TDB(label-only)",
            Self::GpsLabelOnly => "GPS(label-only)",
        }
    }

    /// Returns true when the label currently has no reviewed conversion model.
    #[must_use]
    pub const fn is_label_only(self) -> bool {
        matches!(self, Self::TdbLabelOnly | Self::GpsLabelOnly)
    }
}

/// Finite scalar duration in SI seconds.
///
/// Negative values are allowed so callers can represent backward offsets. This
/// type does not carry a time scale and does not imply calendar arithmetic.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AstroDuration {
    seconds: f64,
}

impl AstroDuration {
    /// Creates a duration from finite SI seconds.
    pub fn seconds(seconds: f64) -> AeroResult<Self> {
        validation::ensure_finite("seconds", seconds)?;
        Ok(Self { seconds })
    }

    /// Returns the stored duration in seconds.
    #[must_use]
    pub const fn as_seconds(self) -> f64 {
        self.seconds
    }
}

/// Alias used when an API wants to emphasize offset semantics.
pub type AstroTimeOffset = AstroDuration;

/// Policy label for requests that would require time-scale conversion.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeScaleConversionPolicy {
    /// Only same-label scalar operations are allowed; cross-label conversion is blocked.
    SameScaleOnly,
    /// Conversion is blocked until UTC leap-second and placeholder-scale rules are reviewed.
    LabelOnlyNoLeapSecondConversion,
}

/// Same-scale epoch comparison result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimeScaleComparison {
    /// Shared time-scale label that made comparison valid.
    pub time_scale: AstroTimeScale,
    /// Ordering of scalar seconds after the same-scale check.
    pub ordering: Ordering,
}

/// Epoch represented as a time-scale label plus seconds from an external reference epoch.
///
/// The reference epoch is a caller contract; this type does not parse dates,
/// normalize UTC, apply leap seconds, or perform time-scale conversion.
///
/// `AstroEpoch` intentionally does **not** implement `PartialOrd`. Use
/// [`compare_same_time_scale`] so cross-scale ordering requests fail explicitly.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AstroEpoch {
    time_scale: AstroTimeScale,
    seconds_from_reference_epoch: f64,
}

impl AstroEpoch {
    /// Creates an epoch with a time-scale label and finite seconds from the caller reference epoch.
    pub fn new(time_scale: AstroTimeScale, seconds_from_reference_epoch: f64) -> AeroResult<Self> {
        validation::ensure_finite("seconds_from_reference_epoch", seconds_from_reference_epoch)?;
        Ok(Self {
            time_scale,
            seconds_from_reference_epoch,
        })
    }

    /// Returns the stored time-scale label.
    #[must_use]
    pub const fn time_scale(self) -> AstroTimeScale {
        self.time_scale
    }

    /// Returns seconds from the caller-defined reference epoch.
    #[must_use]
    pub const fn seconds_from_reference_epoch(self) -> f64 {
        self.seconds_from_reference_epoch
    }

    /// Offsets the epoch by a finite scalar duration without changing its label.
    ///
    /// For `AstroTimeScale::Utc`, this is label-preserving scalar arithmetic
    /// only. It does not model civil UTC, leap seconds, time zones, or calendar
    /// boundaries.
    pub fn offset_by_duration(self, duration: AstroDuration) -> AeroResult<Self> {
        let seconds_from_reference_epoch = self.seconds_from_reference_epoch + duration.seconds;
        if seconds_from_reference_epoch.is_finite() {
            Ok(Self {
                time_scale: self.time_scale,
                seconds_from_reference_epoch,
            })
        } else {
            Err(AeroError::NumericalFailure {
                solver: "astrodynamics.time.offset_by_duration",
                reason: "epoch offset produced a nonfinite seconds value",
            })
        }
    }

    /// Computes a same-scale scalar duration from `earlier` to `self`.
    ///
    /// Cross-scale duration requests are rejected. Same-scale UTC results are
    /// scalar label-preserving offsets only, not leap-second-aware civil UTC
    /// intervals.
    pub fn duration_since_same_scale(self, earlier: Self) -> AeroResult<AstroDuration> {
        require_same_time_scale(self, earlier)?;
        AstroDuration::seconds(
            self.seconds_from_reference_epoch - earlier.seconds_from_reference_epoch,
        )
    }
}

/// Requires two epochs to carry the same time-scale label.
///
/// This is a label equality check only; it is not a conversion policy and does
/// not validate that a placeholder scale has a physical realization.
pub fn require_same_time_scale(left: AstroEpoch, right: AstroEpoch) -> AeroResult<AstroTimeScale> {
    if left.time_scale == right.time_scale {
        Ok(left.time_scale)
    } else {
        Err(AeroError::OutOfDomain {
            parameter: "time_scale_pair",
            value: 0.0,
            expected: "same time-scale label; cross-scale conversion requires a reviewed policy",
        })
    }
}

/// Compares two epochs only after proving they have the same time-scale label.
pub fn compare_same_time_scale(
    left: AstroEpoch,
    right: AstroEpoch,
) -> AeroResult<TimeScaleComparison> {
    let time_scale = require_same_time_scale(left, right)?;
    let ordering = left
        .seconds_from_reference_epoch
        .partial_cmp(&right.seconds_from_reference_epoch)
        .ok_or(AeroError::NumericalFailure {
            solver: "astrodynamics.time.compare_same_time_scale",
            reason: "finite epoch offsets could not be ordered",
        })?;
    Ok(TimeScaleComparison {
        time_scale,
        ordering,
    })
}

/// Blocks UTC leap-sensitive or placeholder time-scale conversion requests.
///
/// A same-label request for `TAI`, `UTC`, or `TT` is treated as no conversion.
/// That does not make UTC arithmetic leap-second-aware; it only confirms that no
/// cross-scale conversion was requested. Placeholder labels remain blocked here
/// because no reviewed conversion/no-op policy has been accepted for them.
pub fn block_utc_leap_sensitive_conversion(
    from: AstroTimeScale,
    to: AstroTimeScale,
    policy: TimeScaleConversionPolicy,
) -> AeroResult<()> {
    if from == to && !from.is_label_only() {
        return Ok(());
    }

    let expected = match policy {
        TimeScaleConversionPolicy::SameScaleOnly => "same non-placeholder time-scale label",
        TimeScaleConversionPolicy::LabelOnlyNoLeapSecondConversion => {
            "reviewed UTC leap-second and placeholder time-scale conversion policy"
        }
    };

    Err(AeroError::OutOfDomain {
        parameter: "time_scale_conversion",
        value: 0.0,
        expected,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_time_scale_labels_are_stable() {
        let cases = [
            (AstroTimeScale::Tai, "TAI", false),
            (AstroTimeScale::Utc, "UTC", false),
            (AstroTimeScale::Tt, "TT", false),
            (AstroTimeScale::TdbLabelOnly, "TDB(label-only)", true),
            (AstroTimeScale::GpsLabelOnly, "GPS(label-only)", true),
        ];

        for (scale, label, label_only) in cases {
            assert_eq!(scale.label(), label);
            assert_eq!(scale.is_label_only(), label_only);
        }
    }

    #[test]
    fn duration_constructor_rejects_nonfinite_seconds() {
        assert!(AstroDuration::seconds(f64::NAN).is_err());
        assert!(AstroDuration::seconds(f64::INFINITY).is_err());
        assert!(AstroDuration::seconds(-f64::INFINITY).is_err());
    }

    #[test]
    fn epoch_constructor_rejects_nonfinite_seconds() {
        assert!(AstroEpoch::new(AstroTimeScale::Tai, f64::NAN).is_err());
        assert!(AstroEpoch::new(AstroTimeScale::Tai, f64::INFINITY).is_err());
        assert!(AstroEpoch::new(AstroTimeScale::Tai, -f64::INFINITY).is_err());
    }

    #[test]
    fn duration_arithmetic_is_deterministic_and_label_preserving() {
        let epoch = AstroEpoch::new(AstroTimeScale::Tai, 10.0).unwrap();
        let shifted = epoch
            .offset_by_duration(AstroDuration::seconds(2.5).unwrap())
            .unwrap();
        assert_eq!(shifted.seconds_from_reference_epoch(), 12.5);
        assert_eq!(shifted.time_scale(), AstroTimeScale::Tai);

        let shifted_back = shifted
            .offset_by_duration(AstroDuration::seconds(-2.5).unwrap())
            .unwrap();
        assert_eq!(shifted_back, epoch);
    }

    #[test]
    fn utc_offset_is_scalar_label_preserving_only() {
        let epoch = AstroEpoch::new(AstroTimeScale::Utc, 1_000.0).unwrap();
        let shifted = epoch
            .offset_by_duration(AstroDuration::seconds(60.0).unwrap())
            .unwrap();
        assert_eq!(shifted.time_scale(), AstroTimeScale::Utc);
        assert_eq!(shifted.seconds_from_reference_epoch(), 1_060.0);
    }

    #[test]
    fn offset_overflow_is_rejected() {
        let epoch = AstroEpoch::new(AstroTimeScale::Tai, f64::MAX).unwrap();
        let duration = AstroDuration::seconds(f64::MAX).unwrap();
        assert!(epoch.offset_by_duration(duration).is_err());
    }

    #[test]
    fn same_scale_duration_works_for_all_labels() {
        let labels = [
            AstroTimeScale::Tai,
            AstroTimeScale::Utc,
            AstroTimeScale::Tt,
            AstroTimeScale::TdbLabelOnly,
            AstroTimeScale::GpsLabelOnly,
        ];

        for label in labels {
            let start = AstroEpoch::new(label, 100.0).unwrap();
            let stop = AstroEpoch::new(label, 125.25).unwrap();
            let duration = stop.duration_since_same_scale(start).unwrap();
            assert_eq!(duration.as_seconds(), 25.25);
        }
    }

    #[test]
    fn same_scale_duration_overflow_is_rejected() {
        let early = AstroEpoch::new(AstroTimeScale::Tai, -f64::MAX).unwrap();
        let late = AstroEpoch::new(AstroTimeScale::Tai, f64::MAX).unwrap();
        assert!(late.duration_since_same_scale(early).is_err());
    }

    #[test]
    fn cross_scale_duration_without_policy_fails() {
        let tai = AstroEpoch::new(AstroTimeScale::Tai, 1.0).unwrap();
        let utc = AstroEpoch::new(AstroTimeScale::Utc, 1.0).unwrap();
        assert!(tai.duration_since_same_scale(utc).is_err());
    }

    #[test]
    fn require_same_time_scale_returns_shared_label() {
        let left = AstroEpoch::new(AstroTimeScale::Tt, -5.0).unwrap();
        let right = AstroEpoch::new(AstroTimeScale::Tt, 5.0).unwrap();
        assert_eq!(
            require_same_time_scale(left, right).unwrap(),
            AstroTimeScale::Tt
        );
    }

    #[test]
    fn compare_same_time_scale_covers_less_equal_and_greater() {
        let a = AstroEpoch::new(AstroTimeScale::Tai, 1.0).unwrap();
        let b = AstroEpoch::new(AstroTimeScale::Tai, 2.0).unwrap();
        let c = AstroEpoch::new(AstroTimeScale::Tai, 2.0).unwrap();

        assert_eq!(
            compare_same_time_scale(a, b).unwrap().ordering,
            Ordering::Less
        );
        assert_eq!(
            compare_same_time_scale(b, a).unwrap().ordering,
            Ordering::Greater
        );
        assert_eq!(
            compare_same_time_scale(b, c).unwrap().ordering,
            Ordering::Equal
        );
    }

    #[test]
    fn compare_same_time_scale_rejects_cross_scale_epochs() {
        let tai = AstroEpoch::new(AstroTimeScale::Tai, 1.0).unwrap();
        let utc = AstroEpoch::new(AstroTimeScale::Utc, 1.0).unwrap();
        assert!(compare_same_time_scale(tai, utc).is_err());
    }

    #[test]
    fn utc_leap_sensitive_conversion_is_blocked() {
        assert!(block_utc_leap_sensitive_conversion(
            AstroTimeScale::Utc,
            AstroTimeScale::Tai,
            TimeScaleConversionPolicy::LabelOnlyNoLeapSecondConversion,
        )
        .is_err());
    }

    #[test]
    fn same_non_placeholder_scale_is_not_a_conversion() {
        for label in [AstroTimeScale::Tai, AstroTimeScale::Utc, AstroTimeScale::Tt] {
            assert!(block_utc_leap_sensitive_conversion(
                label,
                label,
                TimeScaleConversionPolicy::SameScaleOnly,
            )
            .is_ok());
        }
    }

    #[test]
    fn placeholder_time_scale_conversion_is_blocked_even_when_labels_match() {
        for label in [AstroTimeScale::TdbLabelOnly, AstroTimeScale::GpsLabelOnly] {
            assert!(block_utc_leap_sensitive_conversion(
                label,
                label,
                TimeScaleConversionPolicy::LabelOnlyNoLeapSecondConversion,
            )
            .is_err());
        }
    }
}
