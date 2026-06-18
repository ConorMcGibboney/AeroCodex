//! Frame-label and transform-contract primitives for the astrodynamics foundation.
//!
//! This module stores frame identities and explicit transform contracts. It
//! does not implement a frame tree, Earth orientation, precession/nutation,
//! TEME, GCRF, ITRF, local-orbital dynamics, interpolation, or any numerical
//! frame transform.
//!
//! Same-frame identity is allowed for every label, including label-only frames,
//! because it is only a statement that no relabeling or coordinate conversion is
//! being requested. Any non-identity transform involving a label-only frame is
//! blocked until a future source-reviewed frame-policy chunk exists.

use core::fmt;

/// Frame labels used by early astrodynamics records.
///
/// These variants are identity labels, not implemented transform models. The
/// `*LabelOnly` variants intentionally advertise that the crate currently has no
/// reviewed transform implementation for those frames.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AstroFrame {
    /// Generic inertial Earth-centered mean-equator label for synthetic tests.
    InertialEciMeanEquator,
    /// GCRF label only; no GCRF transform model is implemented.
    GcrfLabelOnly,
    /// TEME label only; no TEME transform or SGP4 coupling is implemented.
    TemeLabelOnly,
    /// ECEF/ITRF label only; no Earth-orientation transform is implemented.
    EcefItrfLabelOnly,
    /// Perifocal orbital-plane label; no element-derived transform is implemented here.
    Perifocal,
    /// Local orbital RTN label; no state-derived transform is implemented here.
    LocalOrbitalRtn,
}

impl AstroFrame {
    /// Stable list of all frame labels for validation and deterministic tests.
    #[must_use]
    pub const fn all() -> [Self; 6] {
        [
            Self::InertialEciMeanEquator,
            Self::GcrfLabelOnly,
            Self::TemeLabelOnly,
            Self::EcefItrfLabelOnly,
            Self::Perifocal,
            Self::LocalOrbitalRtn,
        ]
    }

    /// Stable display label for reports, tests, and future comparison records.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::InertialEciMeanEquator => "ECI-mean-equator(label)",
            Self::GcrfLabelOnly => "GCRF(label-only)",
            Self::TemeLabelOnly => "TEME(label-only)",
            Self::EcefItrfLabelOnly => "ECEF/ITRF(label-only)",
            Self::Perifocal => "perifocal(label)",
            Self::LocalOrbitalRtn => "local-orbital-RTN(label)",
        }
    }

    /// Returns true when no reviewed non-identity transform model exists yet.
    ///
    /// Same-frame identity is still allowed for label-only frames. This predicate
    /// blocks only non-identity transform requests.
    #[must_use]
    const fn is_label_only(self) -> bool {
        matches!(
            self,
            Self::GcrfLabelOnly | Self::TemeLabelOnly | Self::EcefItrfLabelOnly
        )
    }

    /// Returns true for frames whose non-identity transforms require a state,
    /// orbit geometry, or local basis definition not supplied by this module.
    #[must_use]
    const fn requires_dynamic_definition(self) -> bool {
        matches!(self, Self::LocalOrbitalRtn)
    }
}

impl fmt::Display for AstroFrame {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.label())
    }
}

/// Directed request to transform from one frame label to another.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FrameTransformDirection {
    from: AstroFrame,
    to: AstroFrame,
}

impl FrameTransformDirection {
    /// Creates a directed frame-transform request.
    #[must_use]
    const fn new(from: AstroFrame, to: AstroFrame) -> Self {
        Self { from, to }
    }

    /// Source frame label.
    #[must_use]
    pub const fn from(self) -> AstroFrame {
        self.from
    }

    /// Destination frame label.
    #[must_use]
    pub const fn to(self) -> AstroFrame {
        self.to
    }

    /// Returns true when this request is a same-frame identity request.
    #[must_use]
    fn is_same_frame(self) -> bool {
        self.from == self.to
    }
}

/// Status assigned to a frame-transform contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FrameTransformStatus {
    /// No coordinate transform is needed because source and destination labels match.
    IdentitySameFrame,
    /// A non-identity transform was blocked because a label-only frame is involved.
    BlockedLabelOnlyFrame,
    /// A non-identity transform was blocked because a state-defined local frame is involved.
    BlockedDynamicLocalFrame,
    /// The requested non-identity transform is not implemented in this foundation layer.
    UnsupportedTransform,
}

impl FrameTransformStatus {
    /// Stable list of all status values for validation and deterministic tests.
    #[must_use]
    pub const fn all() -> [Self; 4] {
        [
            Self::IdentitySameFrame,
            Self::BlockedLabelOnlyFrame,
            Self::BlockedDynamicLocalFrame,
            Self::UnsupportedTransform,
        ]
    }

    /// Stable snake-case code for reports and tests.
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::IdentitySameFrame => "identity_same_frame",
            Self::BlockedLabelOnlyFrame => "blocked_label_only_frame",
            Self::BlockedDynamicLocalFrame => "blocked_dynamic_local_frame",
            Self::UnsupportedTransform => "unsupported_transform",
        }
    }

    /// Returns true for the only currently supported transform contract.
    #[must_use]
    pub const fn is_identity(self) -> bool {
        matches!(self, Self::IdentitySameFrame)
    }

    /// Returns true when the request is intentionally blocked or unsupported.
    #[must_use]
    pub const fn is_blocked(self) -> bool {
        !self.is_identity()
    }
}

impl fmt::Display for FrameTransformStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.code())
    }
}

/// Reviewed contract result for a frame-transform request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FrameTransformContract {
    direction: FrameTransformDirection,
    status: FrameTransformStatus,
}

impl FrameTransformContract {
    /// Creates a transform contract from a direction and reviewed status.
    #[must_use]
    const fn new(direction: FrameTransformDirection, status: FrameTransformStatus) -> Self {
        Self { direction, status }
    }

    /// Returns the directed transform request.
    #[must_use]
    pub const fn direction(self) -> FrameTransformDirection {
        self.direction
    }

    /// Source frame label.
    #[must_use]
    pub const fn from(self) -> AstroFrame {
        self.direction.from()
    }

    /// Destination frame label.
    #[must_use]
    pub const fn to(self) -> AstroFrame {
        self.direction.to()
    }

    /// Transform-contract status.
    #[must_use]
    pub const fn status(self) -> FrameTransformStatus {
        self.status
    }

    /// Returns true for same-frame identity contracts.
    #[must_use]
    pub const fn is_identity(self) -> bool {
        self.status.is_identity()
    }

    /// Returns true for blocked or unsupported non-identity contracts.
    #[must_use]
    pub const fn is_blocked(self) -> bool {
        self.status.is_blocked()
    }

    /// Stable single-line report string for friend tests and future comparison records.
    #[must_use]
    pub fn summary_line(self) -> String {
        format!(
            "frame_transform from={} to={} status={}",
            self.from().label(),
            self.to().label(),
            self.status().code()
        )
    }
}

/// Error returned when a caller requires a supported non-blocked transform.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameTransformError {
    /// A non-identity request involved a label-only frame.
    LabelOnlyFrame { from: AstroFrame, to: AstroFrame },
    /// A non-identity request involved a state-defined local frame.
    DynamicLocalFrame { from: AstroFrame, to: AstroFrame },
    /// A non-identity request has no implementation in this foundation layer.
    UnsupportedTransform { from: AstroFrame, to: AstroFrame },
}

impl FrameTransformError {
    /// Stable snake-case error code for reports and tests.
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::LabelOnlyFrame { .. } => "blocked_label_only_frame",
            Self::DynamicLocalFrame { .. } => "blocked_dynamic_local_frame",
            Self::UnsupportedTransform { .. } => "unsupported_transform",
        }
    }

    /// Source frame label for the rejected request.
    #[must_use]
    const fn from(self) -> AstroFrame {
        match self {
            Self::LabelOnlyFrame { from, .. }
            | Self::DynamicLocalFrame { from, .. }
            | Self::UnsupportedTransform { from, .. } => from,
        }
    }

    /// Destination frame label for the rejected request.
    #[must_use]
    const fn to(self) -> AstroFrame {
        match self {
            Self::LabelOnlyFrame { to, .. }
            | Self::DynamicLocalFrame { to, .. }
            | Self::UnsupportedTransform { to, .. } => to,
        }
    }
}

impl fmt::Display for FrameTransformError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "frame transform from {} to {} is blocked: {}",
            self.from().label(),
            self.to().label(),
            self.code()
        )
    }
}

impl std::error::Error for FrameTransformError {}

/// Result alias for frame-transform contract checks.
pub type FrameTransformResult<T> = Result<T, FrameTransformError>;

/// Creates an explicit same-frame identity contract.
///
/// This is valid for every `AstroFrame` variant, including label-only frames,
/// because no coordinate conversion is requested.
#[must_use]
pub fn same_frame_identity_contract(frame: AstroFrame) -> FrameTransformContract {
    FrameTransformContract::new(
        FrameTransformDirection::new(frame, frame),
        FrameTransformStatus::IdentitySameFrame,
    )
}

/// Classifies a frame-transform request without performing a coordinate transform.
///
/// Non-identity requests involving `GcrfLabelOnly`, `TemeLabelOnly`, or
/// `EcefItrfLabelOnly` are blocked as label-only. Non-identity requests
/// involving `LocalOrbitalRtn` are blocked because this module has no
/// state-derived local basis. Other non-identity requests are reported as
/// unsupported until a later reviewed transform implementation exists.
#[must_use]
pub fn frame_transform_contract(from: AstroFrame, to: AstroFrame) -> FrameTransformContract {
    let direction = FrameTransformDirection::new(from, to);
    let status = if direction.is_same_frame() {
        FrameTransformStatus::IdentitySameFrame
    } else if from.is_label_only() || to.is_label_only() {
        FrameTransformStatus::BlockedLabelOnlyFrame
    } else if from.requires_dynamic_definition() || to.requires_dynamic_definition() {
        FrameTransformStatus::BlockedDynamicLocalFrame
    } else {
        FrameTransformStatus::UnsupportedTransform
    };
    FrameTransformContract::new(direction, status)
}

/// Requires a currently supported transform contract.
///
/// At this stage, only same-frame identity is supported. This function is useful
/// for call sites that should fail closed instead of silently accepting an
/// unimplemented frame conversion.
pub fn require_supported_frame_transform(
    from: AstroFrame,
    to: AstroFrame,
) -> FrameTransformResult<FrameTransformContract> {
    let contract = frame_transform_contract(from, to);
    match contract.status() {
        FrameTransformStatus::IdentitySameFrame => Ok(contract),
        FrameTransformStatus::BlockedLabelOnlyFrame => {
            Err(FrameTransformError::LabelOnlyFrame { from, to })
        }
        FrameTransformStatus::BlockedDynamicLocalFrame => {
            Err(FrameTransformError::DynamicLocalFrame { from, to })
        }
        FrameTransformStatus::UnsupportedTransform => {
            Err(FrameTransformError::UnsupportedTransform { from, to })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_unique_labels(labels: &[&str]) {
        for (left_index, left) in labels.iter().enumerate() {
            for right in labels.iter().skip(left_index + 1) {
                assert_ne!(left, right);
            }
        }
    }

    #[test]
    fn all_frame_labels_are_stable_and_unique() {
        let labels: Vec<_> = AstroFrame::all()
            .iter()
            .map(|frame| frame.label())
            .collect();
        assert_eq!(labels.len(), 6);
        assert!(labels.contains(&"ECI-mean-equator(label)"));
        assert!(labels.contains(&"GCRF(label-only)"));
        assert!(labels.contains(&"TEME(label-only)"));
        assert!(labels.contains(&"ECEF/ITRF(label-only)"));
        assert!(labels.contains(&"perifocal(label)"));
        assert!(labels.contains(&"local-orbital-RTN(label)"));
        assert_unique_labels(&labels);
    }

    #[test]
    fn label_only_and_dynamic_classification_covers_all_labels() {
        for frame in AstroFrame::all() {
            match frame {
                AstroFrame::GcrfLabelOnly
                | AstroFrame::TemeLabelOnly
                | AstroFrame::EcefItrfLabelOnly => assert!(frame.is_label_only()),
                AstroFrame::LocalOrbitalRtn => assert!(frame.requires_dynamic_definition()),
                AstroFrame::InertialEciMeanEquator | AstroFrame::Perifocal => {
                    assert!(!frame.is_label_only());
                    assert!(!frame.requires_dynamic_definition());
                }
            }
        }
    }

    #[test]
    fn frame_equality_and_ordering_are_deterministic() {
        let left = AstroFrame::TemeLabelOnly;
        let right = AstroFrame::TemeLabelOnly;
        assert_eq!(left, right);

        let mut unordered = [
            AstroFrame::LocalOrbitalRtn,
            AstroFrame::GcrfLabelOnly,
            AstroFrame::InertialEciMeanEquator,
        ];
        unordered.sort();
        assert_eq!(unordered[0], AstroFrame::InertialEciMeanEquator);
    }

    #[test]
    fn all_transform_status_codes_are_stable_and_unique() {
        let codes: Vec<_> = FrameTransformStatus::all()
            .iter()
            .map(|status| status.code())
            .collect();
        assert_eq!(codes.len(), 4);
        assert!(codes.contains(&"identity_same_frame"));
        assert!(codes.contains(&"blocked_label_only_frame"));
        assert!(codes.contains(&"blocked_dynamic_local_frame"));
        assert!(codes.contains(&"unsupported_transform"));
        assert_unique_labels(&codes);
    }

    #[test]
    fn same_frame_contract_is_identity_for_all_labels() {
        for frame in AstroFrame::all() {
            let contract = frame_transform_contract(frame, frame);
            assert_eq!(contract.status(), FrameTransformStatus::IdentitySameFrame);
            assert!(contract.is_identity());
            assert!(!contract.is_blocked());

            let required = require_supported_frame_transform(frame, frame).unwrap();
            assert_eq!(required, same_frame_identity_contract(frame));
        }
    }

    #[test]
    fn teme_to_ecef_transform_is_blocked() {
        let contract =
            frame_transform_contract(AstroFrame::TemeLabelOnly, AstroFrame::EcefItrfLabelOnly);
        assert_eq!(
            contract.status(),
            FrameTransformStatus::BlockedLabelOnlyFrame
        );
        assert!(contract.is_blocked());

        let error = require_supported_frame_transform(
            AstroFrame::TemeLabelOnly,
            AstroFrame::EcefItrfLabelOnly,
        )
        .unwrap_err();
        assert_eq!(error.code(), "blocked_label_only_frame");
    }

    #[test]
    fn gcrf_label_only_transform_is_blocked_unless_same_frame() {
        let same = frame_transform_contract(AstroFrame::GcrfLabelOnly, AstroFrame::GcrfLabelOnly);
        assert_eq!(same.status(), FrameTransformStatus::IdentitySameFrame);

        let non_identity = frame_transform_contract(
            AstroFrame::GcrfLabelOnly,
            AstroFrame::InertialEciMeanEquator,
        );
        assert_eq!(
            non_identity.status(),
            FrameTransformStatus::BlockedLabelOnlyFrame
        );
    }

    #[test]
    fn local_orbital_transform_requires_future_dynamic_definition() {
        let contract = frame_transform_contract(
            AstroFrame::LocalOrbitalRtn,
            AstroFrame::InertialEciMeanEquator,
        );
        assert_eq!(
            contract.status(),
            FrameTransformStatus::BlockedDynamicLocalFrame
        );

        let error = require_supported_frame_transform(
            AstroFrame::LocalOrbitalRtn,
            AstroFrame::InertialEciMeanEquator,
        )
        .unwrap_err();
        assert_eq!(error.from(), AstroFrame::LocalOrbitalRtn);
        assert_eq!(error.to(), AstroFrame::InertialEciMeanEquator);
    }

    #[test]
    fn inertial_to_perifocal_is_unsupported_without_reviewed_transform() {
        let contract =
            frame_transform_contract(AstroFrame::InertialEciMeanEquator, AstroFrame::Perifocal);
        assert_eq!(
            contract.status(),
            FrameTransformStatus::UnsupportedTransform
        );
        assert!(contract.is_blocked());

        let error = require_supported_frame_transform(
            AstroFrame::InertialEciMeanEquator,
            AstroFrame::Perifocal,
        )
        .unwrap_err();
        assert_eq!(error.code(), "unsupported_transform");
    }

    #[test]
    fn report_text_includes_blocked_status() {
        let contract =
            frame_transform_contract(AstroFrame::TemeLabelOnly, AstroFrame::EcefItrfLabelOnly);
        let line = contract.summary_line();
        assert!(line.contains("from=TEME(label-only)"));
        assert!(line.contains("to=ECEF/ITRF(label-only)"));
        assert!(line.contains("status=blocked_label_only_frame"));
    }
}
