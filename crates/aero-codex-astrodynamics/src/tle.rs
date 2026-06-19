//! Contract-only TLE source-review policy for the astrodynamics foundation.
//!
//! This module deliberately does not parse text, compute checksums, expand
//! epochs, interpret orbital fields, propagate an orbit, implement SGP4, or
//! transform TEME states. It records the review topics and fixture restrictions
//! that a later, separately approved parser-design chunk must satisfy.
//!
//! No real, recognizable, or externally sourced TLE lines are embedded here.
//! Source-locator records identify review obligations only; they are not proof
//! that a source is authoritative, licensed for fixture reuse, or sufficient to
//! authorize parser implementation.

use core::fmt;

/// Line-level TLE format topics requiring exact source review before parsing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TleLinePolicy {
    /// Review the record and line-pair structure without encoding it here.
    RecordLinePairStructure,
    /// Review record width, character set, and fixed-column policy.
    RecordWidthAndCharacterSet,
    /// Review line-identifier fields and accepted values.
    LineIdentifierFields,
    /// Review object-identifier consistency across the record.
    ObjectIdentifierConsistency,
    /// Review object-identifier encoding and accepted forms.
    ObjectIdentifierEncoding,
    /// Confirm whether line endings or trailing bytes are accepted or rejected.
    TrailingLineEndingPolicy,
}

impl TleLinePolicy {
    /// Deterministic list of all line-level review topics.
    #[allow(non_upper_case_globals)]
    pub const line_topics: [Self; 6] = [
        Self::RecordLinePairStructure,
        Self::RecordWidthAndCharacterSet,
        Self::LineIdentifierFields,
        Self::ObjectIdentifierConsistency,
        Self::ObjectIdentifierEncoding,
        Self::TrailingLineEndingPolicy,
    ];

    const fn code(self) -> &'static str {
        match self {
            Self::RecordLinePairStructure => "record_line_pair_structure",
            Self::RecordWidthAndCharacterSet => "record_width_and_character_set",
            Self::LineIdentifierFields => "line_identifier_fields",
            Self::ObjectIdentifierConsistency => "object_identifier_consistency",
            Self::ObjectIdentifierEncoding => "object_identifier_encoding",
            Self::TrailingLineEndingPolicy => "trailing_line_ending_policy",
        }
    }
}

impl fmt::Display for TleLinePolicy {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.code())
    }
}

/// Field- and downstream-interpretation topics requiring exact source review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TleFieldPolicy {
    /// Checksum coverage, character values, and expected checksum digit.
    ChecksumRule,
    /// Classification-character syntax and acceptance policy.
    ClassificationCharacterPolicy,
    /// International-designator syntax and blank-field policy.
    InternationalDesignatorPolicy,
    /// Epoch-year encoding and year-resolution policy.
    EpochYearEncodingPolicy,
    /// Fractional day-of-year lower/upper bounds and precision policy.
    EpochDayOfYearRange,
    /// Leap-year and civil-calendar validation policy for epoch day values.
    EpochLeapYearPolicy,
    /// First mean-motion derivative syntax, scaling, units, and range.
    MeanMotionFirstDerivativePolicy,
    /// Second mean-motion derivative implied-decimal/exponent policy.
    MeanMotionSecondDerivativePolicy,
    /// Drag-term implied-decimal/exponent syntax, sign, and units policy.
    DragTermExponentPolicy,
    /// Ephemeris-type field syntax and supported-value policy.
    EphemerisTypePolicy,
    /// Element-set-number syntax and range policy.
    ElementSetNumberPolicy,
    /// Inclination units, lexical form, and accepted numeric range.
    InclinationUnitsAndRange,
    /// Right ascension of ascending node units and accepted range.
    RaanUnitsAndRange,
    /// Eccentricity encoding, lexical form, and accepted range.
    EccentricityEncodingAndRange,
    /// Argument of perigee units and accepted numeric range.
    ArgumentOfPerigeeUnitsAndRange,
    /// Mean-anomaly units and accepted numeric range.
    MeanAnomalyUnitsAndRange,
    /// Mean-motion units, sign, precision, and accepted range.
    MeanMotionUnitsAndRange,
    /// Revolution-number-at-epoch syntax and range policy.
    RevolutionNumberAtEpochPolicy,
    /// General-perturbations model coupling and limits on decoded-value interpretation.
    GeneralPerturbationsModelContext,
    /// TEME frame label, epoch coupling, and blocked transform semantics.
    TemeFrameAndEpochSemantics,
}

impl TleFieldPolicy {
    /// Deterministic list of all field-level review topics.
    #[allow(non_upper_case_globals)]
    pub const field_topics: [Self; 20] = [
        Self::ChecksumRule,
        Self::ClassificationCharacterPolicy,
        Self::InternationalDesignatorPolicy,
        Self::EpochYearEncodingPolicy,
        Self::EpochDayOfYearRange,
        Self::EpochLeapYearPolicy,
        Self::MeanMotionFirstDerivativePolicy,
        Self::MeanMotionSecondDerivativePolicy,
        Self::DragTermExponentPolicy,
        Self::EphemerisTypePolicy,
        Self::ElementSetNumberPolicy,
        Self::InclinationUnitsAndRange,
        Self::RaanUnitsAndRange,
        Self::EccentricityEncodingAndRange,
        Self::ArgumentOfPerigeeUnitsAndRange,
        Self::MeanAnomalyUnitsAndRange,
        Self::MeanMotionUnitsAndRange,
        Self::RevolutionNumberAtEpochPolicy,
        Self::GeneralPerturbationsModelContext,
        Self::TemeFrameAndEpochSemantics,
    ];

    const fn code(self) -> &'static str {
        match self {
            Self::ChecksumRule => "checksum_rule",
            Self::ClassificationCharacterPolicy => "classification_character_policy",
            Self::InternationalDesignatorPolicy => "international_designator_policy",
            Self::EpochYearEncodingPolicy => "epoch_year_encoding_policy",
            Self::EpochDayOfYearRange => "epoch_day_of_year_range",
            Self::EpochLeapYearPolicy => "epoch_leap_year_policy",
            Self::MeanMotionFirstDerivativePolicy => "mean_motion_first_derivative_policy",
            Self::MeanMotionSecondDerivativePolicy => "mean_motion_second_derivative_policy",
            Self::DragTermExponentPolicy => "drag_term_exponent_policy",
            Self::EphemerisTypePolicy => "ephemeris_type_policy",
            Self::ElementSetNumberPolicy => "element_set_number_policy",
            Self::InclinationUnitsAndRange => "inclination_units_and_range",
            Self::RaanUnitsAndRange => "raan_units_and_range",
            Self::EccentricityEncodingAndRange => "eccentricity_encoding_and_range",
            Self::ArgumentOfPerigeeUnitsAndRange => "argument_of_perigee_units_and_range",
            Self::MeanAnomalyUnitsAndRange => "mean_anomaly_units_and_range",
            Self::MeanMotionUnitsAndRange => "mean_motion_units_and_range",
            Self::RevolutionNumberAtEpochPolicy => "revolution_number_at_epoch_policy",
            Self::GeneralPerturbationsModelContext => "general_perturbations_model_context",
            Self::TemeFrameAndEpochSemantics => "teme_frame_and_epoch_semantics",
        }
    }
}

impl fmt::Display for TleFieldPolicy {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.code())
    }
}

/// One line- or field-level source-review obligation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TleSourceReviewTopic {
    /// Line-format review topic.
    Line(TleLinePolicy),
    /// Field or downstream-interpretation review topic.
    Field(TleFieldPolicy),
}

impl TleSourceReviewTopic {
    /// Deterministic list of all required source-review topics.
    #[allow(non_upper_case_globals)]
    pub const review_topics: [Self; 26] = [
        Self::Line(TleLinePolicy::RecordLinePairStructure),
        Self::Line(TleLinePolicy::RecordWidthAndCharacterSet),
        Self::Line(TleLinePolicy::LineIdentifierFields),
        Self::Line(TleLinePolicy::ObjectIdentifierConsistency),
        Self::Line(TleLinePolicy::ObjectIdentifierEncoding),
        Self::Line(TleLinePolicy::TrailingLineEndingPolicy),
        Self::Field(TleFieldPolicy::ChecksumRule),
        Self::Field(TleFieldPolicy::ClassificationCharacterPolicy),
        Self::Field(TleFieldPolicy::InternationalDesignatorPolicy),
        Self::Field(TleFieldPolicy::EpochYearEncodingPolicy),
        Self::Field(TleFieldPolicy::EpochDayOfYearRange),
        Self::Field(TleFieldPolicy::EpochLeapYearPolicy),
        Self::Field(TleFieldPolicy::MeanMotionFirstDerivativePolicy),
        Self::Field(TleFieldPolicy::MeanMotionSecondDerivativePolicy),
        Self::Field(TleFieldPolicy::DragTermExponentPolicy),
        Self::Field(TleFieldPolicy::EphemerisTypePolicy),
        Self::Field(TleFieldPolicy::ElementSetNumberPolicy),
        Self::Field(TleFieldPolicy::InclinationUnitsAndRange),
        Self::Field(TleFieldPolicy::RaanUnitsAndRange),
        Self::Field(TleFieldPolicy::EccentricityEncodingAndRange),
        Self::Field(TleFieldPolicy::ArgumentOfPerigeeUnitsAndRange),
        Self::Field(TleFieldPolicy::MeanAnomalyUnitsAndRange),
        Self::Field(TleFieldPolicy::MeanMotionUnitsAndRange),
        Self::Field(TleFieldPolicy::RevolutionNumberAtEpochPolicy),
        Self::Field(TleFieldPolicy::GeneralPerturbationsModelContext),
        Self::Field(TleFieldPolicy::TemeFrameAndEpochSemantics),
    ];
}

impl fmt::Display for TleSourceReviewTopic {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Line(policy) => write!(formatter, "line.{policy}"),
            Self::Field(policy) => write!(formatter, "field.{policy}"),
        }
    }
}

/// Exact source locator proposed for one TLE review topic.
///
/// This is planning metadata only. A nonempty source ID and locator do not prove
/// authority, licensing, review quality, or implementation approval.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TleSourceLocator<'a> {
    /// Topic covered by this locator.
    pub topic: TleSourceReviewTopic,
    /// Proposed AeroCodex source-registry ID.
    pub source_id: &'a str,
    /// Exact section, table, paragraph, or equivalent locator within the source.
    pub exact_locator: &'a str,
}

/// Source-locator set supplied for contract review.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TleSourcePolicy<'a> {
    /// Proposed locators. Exactly one valid locator is required per topic.
    pub locators: &'a [TleSourceLocator<'a>],
}

/// Fixture restriction for this contract-only layer.
///
/// Neither variant permits live, recognizable, or unreviewed external TLE data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TleFixturePolicy {
    /// Do not store or exercise any TLE-shaped fixture text.
    NoFixtureData,
    /// Permit only artificial structure labels in future design tests, with no
    /// catalog object, real line, or externally sourced numeric pattern.
    ArtificialStructureOnly,
}

impl TleFixturePolicy {
    /// Deterministic list of the only fixture policies admitted by this layer.
    #[allow(non_upper_case_globals)]
    pub const fixture_policies: [Self; 2] = [Self::NoFixtureData, Self::ArtificialStructureOnly];

    const fn code(self) -> &'static str {
        match self {
            Self::NoFixtureData => "no_fixture_data",
            Self::ArtificialStructureOnly => "artificial_structure_only",
        }
    }

    /// Returns whether this contract layer permits external fixture data.
    ///
    /// The result is false for every admitted policy by construction.
    #[must_use]
    pub const fn permits_external_fixture_data(self) -> bool {
        match self {
            Self::NoFixtureData | Self::ArtificialStructureOnly => false,
        }
    }
}

impl fmt::Display for TleFixturePolicy {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.code())
    }
}

/// Contract input for reviewing TLE parser prerequisites.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TleContract<'a> {
    /// Exact source-locator coverage proposed for review.
    pub source_policy: TleSourcePolicy<'a>,
    /// Fixture restriction retained by the contract-only layer.
    pub fixture_policy: TleFixturePolicy,
}

/// Deterministic result of reviewing TLE contract prerequisites.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TleContractStatus {
    /// At least one required topic has no source locator.
    BlockedMissingSourceLocator,
    /// A required locator is empty, whitespace-only, or contains a control character.
    BlockedInvalidSourceLocator,
    /// More than one locator was supplied for a topic, making precedence unclear.
    BlockedDuplicateSourceLocator,
    /// A locator still contains an explicit placeholder marker.
    BlockedPlaceholderSourceLocator,
    /// Locator coverage is structurally complete but still requires human review.
    StructurallyCompleteHumanReviewRequired,
}

impl TleContractStatus {
    /// Deterministic list of all contract statuses.
    #[allow(non_upper_case_globals)]
    pub const contract_statuses: [Self; 5] = [
        Self::BlockedMissingSourceLocator,
        Self::BlockedInvalidSourceLocator,
        Self::BlockedDuplicateSourceLocator,
        Self::BlockedPlaceholderSourceLocator,
        Self::StructurallyCompleteHumanReviewRequired,
    ];

    const fn code(self) -> &'static str {
        match self {
            Self::BlockedMissingSourceLocator => "blocked_missing_source_locator",
            Self::BlockedInvalidSourceLocator => "blocked_invalid_source_locator",
            Self::BlockedDuplicateSourceLocator => "blocked_duplicate_source_locator",
            Self::BlockedPlaceholderSourceLocator => "blocked_placeholder_source_locator",
            Self::StructurallyCompleteHumanReviewRequired => {
                "structurally_complete_human_review_required"
            }
        }
    }

    /// Returns true only when locator coverage is structurally complete enough
    /// to begin a separate human source/design review.
    ///
    /// A true result still does not authorize parser implementation.
    #[must_use]
    pub const fn is_structurally_complete_for_human_review(self) -> bool {
        matches!(self, Self::StructurallyCompleteHumanReviewRequired)
    }
}

impl fmt::Display for TleContractStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.code())
    }
}

/// Review report for a contract-only TLE prerequisite check.
///
/// Fields are private so callers cannot forge an authoritative-looking
/// structurally complete review without using [`evaluate_tle_contract`]. Use the
/// read-only accessors to inspect the deterministic, non-authorizing result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TleContractReview {
    /// Overall prerequisite status.
    status: TleContractStatus,
    /// First blocking topic in deterministic policy order, if any.
    first_blocking_topic: Option<TleSourceReviewTopic>,
    /// Fixture restriction applied during review.
    fixture_policy: TleFixturePolicy,
}

impl TleContractReview {
    /// Overall prerequisite status from the fail-closed evaluator.
    #[must_use]
    pub const fn status(&self) -> TleContractStatus {
        self.status
    }

    /// First blocking topic in deterministic policy order, if any.
    #[must_use]
    pub const fn first_blocking_topic(&self) -> Option<TleSourceReviewTopic> {
        self.first_blocking_topic
    }

    /// Fixture restriction applied during review.
    #[must_use]
    pub const fn fixture_policy(&self) -> TleFixturePolicy {
        self.fixture_policy
    }
}

impl fmt::Display for TleContractReview {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "tle_contract status={} first_blocking_topic=",
            self.status
        )?;
        match self.first_blocking_topic {
            Some(topic) => write!(formatter, "{topic}")?,
            None => formatter.write_str("none")?,
        }
        write!(
            formatter,
            " fixture_policy={} parser_implementation_authorized=false \
             caveat=contract-only-no-parser-no-sgp4-no-teme-transform-no-operational-tracking",
            self.fixture_policy
        )
    }
}

/// Reviews source-locator coverage for the contract-only TLE policy.
///
/// Every line and field topic must have exactly one locator whose `source_id`
/// and `exact_locator` are nonempty after trimming and contain no control
/// characters. Structural completeness
/// returns `StructurallyCompleteHumanReviewRequired`; it never authorizes parser
/// implementation or fixture import.
#[must_use]
pub fn evaluate_tle_contract(contract: &TleContract<'_>) -> TleContractReview {
    for topic in TleSourceReviewTopic::review_topics {
        let mut matching_count = 0_usize;
        let mut has_invalid_locator = false;
        let mut has_placeholder_locator = false;

        for locator in contract.source_policy.locators {
            if locator.topic == topic {
                matching_count = matching_count.saturating_add(1);
                if !is_valid_locator_component(locator.source_id)
                    || !is_valid_locator_component(locator.exact_locator)
                {
                    has_invalid_locator = true;
                }
                if contains_placeholder_marker(locator.source_id)
                    || contains_placeholder_marker(locator.exact_locator)
                {
                    has_placeholder_locator = true;
                }
            }
        }

        let status = if matching_count == 0 {
            Some(TleContractStatus::BlockedMissingSourceLocator)
        } else if has_invalid_locator {
            Some(TleContractStatus::BlockedInvalidSourceLocator)
        } else if matching_count > 1 {
            Some(TleContractStatus::BlockedDuplicateSourceLocator)
        } else if has_placeholder_locator {
            Some(TleContractStatus::BlockedPlaceholderSourceLocator)
        } else {
            None
        };

        if let Some(status) = status {
            return TleContractReview {
                status,
                first_blocking_topic: Some(topic),
                fixture_policy: contract.fixture_policy,
            };
        }
    }

    TleContractReview {
        status: TleContractStatus::StructurallyCompleteHumanReviewRequired,
        first_blocking_topic: None,
        fixture_policy: contract.fixture_policy,
    }
}

fn is_valid_locator_component(value: &str) -> bool {
    let trimmed = value.trim();
    !trimmed.is_empty() && !trimmed.chars().any(char::is_control)
}

fn contains_placeholder_marker(value: &str) -> bool {
    value
        .split(|character: char| !character.is_ascii_alphanumeric())
        .any(|token| {
            token.eq_ignore_ascii_case("pending")
                || token.eq_ignore_ascii_case("todo")
                || token.eq_ignore_ascii_case("tbd")
                || token.eq_ignore_ascii_case("unknown")
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn complete_locators() -> Vec<TleSourceLocator<'static>> {
        TleSourceReviewTopic::review_topics
            .iter()
            .copied()
            .map(|topic| TleSourceLocator {
                topic,
                source_id: "source.synthetic.contract.review",
                exact_locator: "synthetic section locator for structural unit test",
            })
            .collect()
    }

    #[test]
    fn line_policy_labels_are_stable() {
        let labels: Vec<String> = TleLinePolicy::line_topics
            .iter()
            .map(|value| value.to_string())
            .collect();
        assert_eq!(
            labels,
            [
                "record_line_pair_structure",
                "record_width_and_character_set",
                "line_identifier_fields",
                "object_identifier_consistency",
                "object_identifier_encoding",
                "trailing_line_ending_policy",
            ]
        );
    }

    #[test]
    fn field_policy_labels_are_stable() {
        let labels: Vec<String> = TleFieldPolicy::field_topics
            .iter()
            .map(|value| value.to_string())
            .collect();
        assert_eq!(
            labels,
            [
                "checksum_rule",
                "classification_character_policy",
                "international_designator_policy",
                "epoch_year_encoding_policy",
                "epoch_day_of_year_range",
                "epoch_leap_year_policy",
                "mean_motion_first_derivative_policy",
                "mean_motion_second_derivative_policy",
                "drag_term_exponent_policy",
                "ephemeris_type_policy",
                "element_set_number_policy",
                "inclination_units_and_range",
                "raan_units_and_range",
                "eccentricity_encoding_and_range",
                "argument_of_perigee_units_and_range",
                "mean_anomaly_units_and_range",
                "mean_motion_units_and_range",
                "revolution_number_at_epoch_policy",
                "general_perturbations_model_context",
                "teme_frame_and_epoch_semantics",
            ]
        );
    }

    #[test]
    fn source_review_topic_order_is_deterministic() {
        assert_eq!(TleSourceReviewTopic::review_topics.len(), 26);
        assert_eq!(
            TleSourceReviewTopic::review_topics[0].to_string(),
            "line.record_line_pair_structure"
        );
        assert_eq!(
            TleSourceReviewTopic::review_topics[25].to_string(),
            "field.teme_frame_and_epoch_semantics"
        );
    }

    #[test]
    fn fixture_policies_reject_external_data_by_construction() {
        for policy in TleFixturePolicy::fixture_policies {
            assert!(!policy.permits_external_fixture_data());
        }
        assert_eq!(
            TleFixturePolicy::NoFixtureData.to_string(),
            "no_fixture_data"
        );
        assert_eq!(
            TleFixturePolicy::ArtificialStructureOnly.to_string(),
            "artificial_structure_only"
        );
    }

    #[test]
    fn contract_status_labels_are_stable() {
        let labels: Vec<String> = TleContractStatus::contract_statuses
            .iter()
            .map(|value| value.to_string())
            .collect();
        assert_eq!(
            labels,
            [
                "blocked_missing_source_locator",
                "blocked_invalid_source_locator",
                "blocked_duplicate_source_locator",
                "blocked_placeholder_source_locator",
                "structurally_complete_human_review_required",
            ]
        );
        for status in &TleContractStatus::contract_statuses[..4] {
            assert!(!status.is_structurally_complete_for_human_review());
        }
    }

    #[test]
    fn empty_source_policy_is_blocked_at_first_topic() {
        let contract = TleContract {
            source_policy: TleSourcePolicy { locators: &[] },
            fixture_policy: TleFixturePolicy::NoFixtureData,
        };
        let review = evaluate_tle_contract(&contract);
        assert_eq!(
            review.status(),
            TleContractStatus::BlockedMissingSourceLocator
        );
        assert_eq!(
            review.first_blocking_topic(),
            Some(TleSourceReviewTopic::Line(
                TleLinePolicy::RecordLinePairStructure
            ))
        );
    }

    #[test]
    fn whitespace_source_id_is_invalid() {
        let locator = TleSourceLocator {
            topic: TleSourceReviewTopic::Line(TleLinePolicy::RecordLinePairStructure),
            source_id: "   ",
            exact_locator: "table pending review",
        };
        let contract = TleContract {
            source_policy: TleSourcePolicy {
                locators: &[locator],
            },
            fixture_policy: TleFixturePolicy::NoFixtureData,
        };
        let review = evaluate_tle_contract(&contract);
        assert_eq!(
            review.status(),
            TleContractStatus::BlockedInvalidSourceLocator
        );
        assert_eq!(review.first_blocking_topic(), Some(locator.topic));
    }

    #[test]
    fn whitespace_exact_locator_is_invalid() {
        let locator = TleSourceLocator {
            topic: TleSourceReviewTopic::Line(TleLinePolicy::RecordLinePairStructure),
            source_id: "source.synthetic.contract.review",
            exact_locator: "\t",
        };
        let contract = TleContract {
            source_policy: TleSourcePolicy {
                locators: &[locator],
            },
            fixture_policy: TleFixturePolicy::NoFixtureData,
        };
        let review = evaluate_tle_contract(&contract);
        assert_eq!(
            review.status(),
            TleContractStatus::BlockedInvalidSourceLocator
        );
        assert_eq!(review.first_blocking_topic(), Some(locator.topic));
    }

    #[test]
    fn embedded_control_character_is_invalid() {
        let locator = TleSourceLocator {
            topic: TleSourceReviewTopic::Line(TleLinePolicy::RecordLinePairStructure),
            source_id: "source.synthetic.contract.review",
            exact_locator: "section one\nsection two",
        };
        let contract = TleContract {
            source_policy: TleSourcePolicy {
                locators: &[locator],
            },
            fixture_policy: TleFixturePolicy::NoFixtureData,
        };
        let review = evaluate_tle_contract(&contract);
        assert_eq!(
            review.status(),
            TleContractStatus::BlockedInvalidSourceLocator
        );
        assert_eq!(review.first_blocking_topic(), Some(locator.topic));
    }

    #[test]
    fn duplicate_locator_for_one_topic_is_blocked() {
        let topic = TleSourceReviewTopic::Line(TleLinePolicy::RecordLinePairStructure);
        let locators = [
            TleSourceLocator {
                topic,
                source_id: "source.synthetic.contract.review",
                exact_locator: "first exact locator",
            },
            TleSourceLocator {
                topic,
                source_id: "source.synthetic.contract.review",
                exact_locator: "second exact locator",
            },
        ];
        let contract = TleContract {
            source_policy: TleSourcePolicy {
                locators: &locators,
            },
            fixture_policy: TleFixturePolicy::NoFixtureData,
        };
        let review = evaluate_tle_contract(&contract);
        assert_eq!(
            review.status(),
            TleContractStatus::BlockedDuplicateSourceLocator
        );
        assert_eq!(review.first_blocking_topic(), Some(topic));
    }

    #[test]
    fn placeholder_locator_is_blocked() {
        let locator = TleSourceLocator {
            topic: TleSourceReviewTopic::Line(TleLinePolicy::RecordLinePairStructure),
            source_id: "source.synthetic.pending",
            exact_locator: "section 1",
        };
        let contract = TleContract {
            source_policy: TleSourcePolicy {
                locators: &[locator],
            },
            fixture_policy: TleFixturePolicy::NoFixtureData,
        };
        let review = evaluate_tle_contract(&contract);
        assert_eq!(
            review.status(),
            TleContractStatus::BlockedPlaceholderSourceLocator
        );
        assert_eq!(review.first_blocking_topic(), Some(locator.topic));
    }

    #[test]
    fn complete_locator_coverage_requires_human_review() {
        let locators = complete_locators();
        let contract = TleContract {
            source_policy: TleSourcePolicy {
                locators: &locators,
            },
            fixture_policy: TleFixturePolicy::ArtificialStructureOnly,
        };
        let review = evaluate_tle_contract(&contract);
        assert_eq!(
            review.status(),
            TleContractStatus::StructurallyCompleteHumanReviewRequired
        );
        assert!(review.status().is_structurally_complete_for_human_review());
        assert!(review.first_blocking_topic().is_none());
    }

    #[test]
    fn locator_input_order_does_not_change_review() {
        let locators = complete_locators();
        let mut reversed_locators = locators.clone();
        reversed_locators.reverse();

        let forward_contract = TleContract {
            source_policy: TleSourcePolicy {
                locators: &locators,
            },
            fixture_policy: TleFixturePolicy::ArtificialStructureOnly,
        };
        let reversed_contract = TleContract {
            source_policy: TleSourcePolicy {
                locators: &reversed_locators,
            },
            fixture_policy: TleFixturePolicy::ArtificialStructureOnly,
        };

        assert_eq!(
            evaluate_tle_contract(&forward_contract),
            evaluate_tle_contract(&reversed_contract)
        );
    }

    #[test]
    fn review_summary_is_deterministic_and_non_claiming() {
        let contract = TleContract {
            source_policy: TleSourcePolicy { locators: &[] },
            fixture_policy: TleFixturePolicy::NoFixtureData,
        };
        let first = evaluate_tle_contract(&contract).to_string();
        let second = evaluate_tle_contract(&contract).to_string();
        let expected = concat!(
            "tle_contract status=blocked_missing_source_locator ",
            "first_blocking_topic=line.record_line_pair_structure ",
            "fixture_policy=no_fixture_data ",
            "parser_implementation_authorized=false ",
            "caveat=contract-only-no-parser-no-sgp4-no-teme-transform-no-operational-tracking",
        );
        assert_eq!(first, expected);
        assert_eq!(first, second);
        assert!(first.contains("parser_implementation_authorized=false"));
        assert!(first.contains("contract-only-no-parser"));
        assert!(first.contains("no-operational-tracking"));
        assert!(!first.contains("parser_implementation_authorized=true"));
        assert!(!first.contains("certified=true"));
    }

    #[test]
    fn topic_lists_have_exact_variant_coverage() {
        use std::collections::BTreeSet;

        let line_topics: Vec<_> = TleLinePolicy::line_topics
            .iter()
            .copied()
            .map(TleSourceReviewTopic::Line)
            .collect();
        let field_topics: Vec<_> = TleFieldPolicy::field_topics
            .iter()
            .copied()
            .map(TleSourceReviewTopic::Field)
            .collect();
        let combined: Vec<_> = line_topics
            .iter()
            .chain(field_topics.iter())
            .copied()
            .collect();
        assert_eq!(TleLinePolicy::line_topics.len(), 6);
        assert_eq!(TleFieldPolicy::field_topics.len(), 20);
        assert_eq!(combined.len(), 26);
        assert_eq!(combined, TleSourceReviewTopic::review_topics);
        assert_eq!(
            TleLinePolicy::line_topics
                .iter()
                .copied()
                .collect::<BTreeSet<_>>()
                .len(),
            TleLinePolicy::line_topics.len()
        );
        assert_eq!(
            TleFieldPolicy::field_topics
                .iter()
                .copied()
                .collect::<BTreeSet<_>>()
                .len(),
            TleFieldPolicy::field_topics.len()
        );
        assert_eq!(
            TleSourceReviewTopic::review_topics
                .iter()
                .copied()
                .collect::<BTreeSet<_>>()
                .len(),
            TleSourceReviewTopic::review_topics.len()
        );
    }

    #[test]
    fn placeholder_matching_is_case_insensitive_whole_token() {
        for marker in [
            "pending".to_ascii_uppercase(),
            "ToDo".to_string(),
            "tbD".to_string(),
            "Unknown".to_string(),
        ] {
            let locator = TleSourceLocator {
                topic: TleSourceReviewTopic::Line(TleLinePolicy::RecordLinePairStructure),
                source_id: "source.synthetic.contract.review",
                exact_locator: marker.as_str(),
            };
            let contract = TleContract {
                source_policy: TleSourcePolicy {
                    locators: &[locator],
                },
                fixture_policy: TleFixturePolicy::NoFixtureData,
            };
            let review = evaluate_tle_contract(&contract);
            assert_eq!(
                review.status(),
                TleContractStatus::BlockedPlaceholderSourceLocator
            );
            assert_eq!(review.first_blocking_topic(), Some(locator.topic));
        }

        let locator = TleSourceLocator {
            topic: TleSourceReviewTopic::Line(TleLinePolicy::RecordLinePairStructure),
            source_id: "source.synthetic.contract.review",
            exact_locator: "section-pending-review",
        };
        let contract = TleContract {
            source_policy: TleSourcePolicy {
                locators: &[locator],
            },
            fixture_policy: TleFixturePolicy::NoFixtureData,
        };
        let review = evaluate_tle_contract(&contract);
        assert_eq!(
            review.status(),
            TleContractStatus::BlockedPlaceholderSourceLocator
        );

        let mut complete = complete_locators();
        complete[0] = TleSourceLocator {
            topic: TleSourceReviewTopic::Line(TleLinePolicy::RecordLinePairStructure),
            source_id: "source.independent.pendingly.review",
            exact_locator: "methodology unknowns todolist tbdish",
        };
        let contract = TleContract {
            source_policy: TleSourcePolicy {
                locators: &complete,
            },
            fixture_policy: TleFixturePolicy::NoFixtureData,
        };
        assert_eq!(
            evaluate_tle_contract(&contract).status(),
            TleContractStatus::StructurallyCompleteHumanReviewRequired
        );
    }

    #[test]
    fn locator_components_reject_empty_and_control_characters() {
        for (source_id, exact_locator) in [
            ("", "section one"),
            ("source.synthetic.contract.review", ""),
            ("source.synthetic.contract.review", "\n"),
            ("source.synthetic.contract.review", "\r"),
            ("source.synthetic.contract.review", "\t"),
            ("source.synthetic.contract.review", "\0"),
            ("source.synthetic.contract.review", "\u{7f}"),
            ("source.synthetic.contract.review", "section\u{1f}one"),
        ] {
            let locator = TleSourceLocator {
                topic: TleSourceReviewTopic::Line(TleLinePolicy::RecordLinePairStructure),
                source_id,
                exact_locator,
            };
            let contract = TleContract {
                source_policy: TleSourcePolicy {
                    locators: &[locator],
                },
                fixture_policy: TleFixturePolicy::NoFixtureData,
            };
            let review = evaluate_tle_contract(&contract);
            assert_eq!(
                review.status(),
                TleContractStatus::BlockedInvalidSourceLocator
            );
            assert_eq!(review.first_blocking_topic(), Some(locator.topic));
        }
    }

    #[test]
    fn locator_components_allow_leading_and_trailing_whitespace_when_nonempty() {
        let mut complete = complete_locators();
        complete[0] = TleSourceLocator {
            topic: TleSourceReviewTopic::Line(TleLinePolicy::RecordLinePairStructure),
            source_id: " source.synthetic.contract.review ",
            exact_locator: " section one ",
        };
        let contract = TleContract {
            source_policy: TleSourcePolicy {
                locators: &complete,
            },
            fixture_policy: TleFixturePolicy::ArtificialStructureOnly,
        };
        assert_eq!(
            evaluate_tle_contract(&contract).status(),
            TleContractStatus::StructurallyCompleteHumanReviewRequired
        );
    }

    #[test]
    fn duplicate_defect_precedence_is_deterministic() {
        let topic = TleSourceReviewTopic::Line(TleLinePolicy::RecordLinePairStructure);
        let invalid_duplicate = [
            TleSourceLocator {
                topic,
                source_id: "source.synthetic.contract.review",
                exact_locator: "valid locator",
            },
            TleSourceLocator {
                topic,
                source_id: "source.synthetic.contract.review",
                exact_locator: "\n",
            },
        ];
        let invalid_contract = TleContract {
            source_policy: TleSourcePolicy {
                locators: &invalid_duplicate,
            },
            fixture_policy: TleFixturePolicy::NoFixtureData,
        };
        assert_eq!(
            evaluate_tle_contract(&invalid_contract).status(),
            TleContractStatus::BlockedInvalidSourceLocator
        );

        let placeholder_duplicate = [
            TleSourceLocator {
                topic,
                source_id: "source.synthetic.contract.review",
                exact_locator: "valid locator",
            },
            TleSourceLocator {
                topic,
                source_id: "source.synthetic.contract.review",
                exact_locator: "pending",
            },
        ];
        let placeholder_contract = TleContract {
            source_policy: TleSourcePolicy {
                locators: &placeholder_duplicate,
            },
            fixture_policy: TleFixturePolicy::NoFixtureData,
        };
        assert_eq!(
            evaluate_tle_contract(&placeholder_contract).status(),
            TleContractStatus::BlockedDuplicateSourceLocator
        );
    }

    #[test]
    fn earliest_topic_precedence_is_independent_of_locator_input_order() {
        let first_topic = TleSourceReviewTopic::Line(TleLinePolicy::RecordLinePairStructure);
        let later_topic = TleSourceReviewTopic::Field(TleFieldPolicy::ChecksumRule);
        let locators = [
            TleSourceLocator {
                topic: later_topic,
                source_id: "source.synthetic.contract.review",
                exact_locator: "\n",
            },
            TleSourceLocator {
                topic: first_topic,
                source_id: "source.synthetic.contract.review",
                exact_locator: "pending",
            },
        ];
        let reversed = [locators[1], locators[0]];
        for ordered in [&locators[..], &reversed[..]] {
            let contract = TleContract {
                source_policy: TleSourcePolicy { locators: ordered },
                fixture_policy: TleFixturePolicy::NoFixtureData,
            };
            let review = evaluate_tle_contract(&contract);
            assert_eq!(
                review.status(),
                TleContractStatus::BlockedPlaceholderSourceLocator
            );
            assert_eq!(review.first_blocking_topic(), Some(first_topic));
        }
    }

    #[test]
    fn review_accessors_are_read_only_and_non_authorizing() {
        let contract = TleContract {
            source_policy: TleSourcePolicy { locators: &[] },
            fixture_policy: TleFixturePolicy::ArtificialStructureOnly,
        };
        let review = evaluate_tle_contract(&contract);
        assert_eq!(
            review.status(),
            TleContractStatus::BlockedMissingSourceLocator
        );
        assert_eq!(
            review.first_blocking_topic(),
            Some(TleSourceReviewTopic::Line(
                TleLinePolicy::RecordLinePairStructure
            ))
        );
        assert_eq!(
            review.fixture_policy(),
            TleFixturePolicy::ArtificialStructureOnly
        );
        assert!(!review.status().is_structurally_complete_for_human_review());
        assert!(!review
            .to_string()
            .contains("parser_implementation_authorized=true"));
    }
}
