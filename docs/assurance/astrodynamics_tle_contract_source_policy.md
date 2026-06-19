# Astrodynamics TLE Contract-Only Source Policy

Status: `research_required`; parser implementation blocked

This policy defines the evidence required before a later AeroCodex TLE parser proposal may be considered. The current Rust layer is metadata-only: it records review topics, fixture restrictions, source locators, and a fail-closed review status.

## Current blocked capabilities

The current foundation does not parse TLE text, calculate checksums, decode epochs or compact numeric fields, interpret orbital elements, construct TEME states, run SGP4, or perform frame transformations. It embeds no TLE-like line pair.

## Required exact-source review

A later parser proposal must provide one reviewed locator for every topic represented by `TleSourceReviewTopic::review_topics`, including:

- record/line-pair structure, character set, width, identifiers, and line-ending policy;
- checksum coverage and character contribution rules;
- classification and international-designator fields;
- epoch-year, fractional-day, leap-year, calendar, and time-scale policy;
- first/second mean-motion derivatives and compact mantissa/exponent fields;
- drag-term syntax and units;
- ephemeris type, element-set number, and revolution number;
- inclination, right ascension of ascending node, eccentricity, argument of perigee, mean anomaly, and mean-motion units/ranges;
- general-perturbations model context;
- TEME frame and epoch semantics.

Each locator must identify the source, edition or revision, publication/revision date, exact section/table/paragraph, stable access identifier, license/access review, supported rule, reviewer, and review date. A homepage, implementation source file, test body, fixture, or generic claim that the format is standard is insufficient.

## Structural review algorithm

`evaluate_tle_contract` checks metadata only. It requires exactly one nonempty, non-placeholder locator per topic; rejects duplicates, control characters, and placeholder tokens; and returns `StructurallyCompleteHumanReviewRequired` only after all topics pass. Structural completeness never authorizes parser implementation.

The deterministic review summary always records `parser_implementation_authorized=false`.

## Fixture policy

`NoFixtureData` and `ArtificialStructureOnly` both prohibit live, historical, recognizable, or unreviewed external TLE data. `ArtificialStructureOnly` permits source-free metadata labels for contract tests; it does not permit a complete line, line pair, catalog object, checksum vector, epoch, or orbital-value set.

Any later synthetic parser fixture requires separate approval, documented value selection, one-to-one mapping to reviewed rules, malformed cases for every rejection policy, and a fixture manifest. External fixtures additionally require provenance, license/access, retrieval date, exact content hash, transformations, and expected-value generation procedure.

## Independent implementation rule

A future parser may implement paraphrased, reviewed public-format rules in AeroCodex-native Rust. It must not copy or translate Orekit or another library's parser source, comments, tests, fixtures, helper decomposition, errors, or permissive/strict behavior.

## Entry gate for a later parser chunk

Parser work remains blocked until the proposal includes exact locator evidence for all topics, a source-registry update, byte/character and malformed-input policy, complete epoch/calendar/time policy, field units/ranges, model/TEME boundary, synthetic-fixture provenance plan, function-by-function API audit, and equation/helper inventory classification. SGP4 and numerical TEME transforms require separate authorization.

This contract is not a parser, orbit propagator, tracking system, conjunction-assessment tool, navigation product, certified implementation, or operational service.
