# Orekit Non-Copying Boundary for the Rust Astrodynamics Foundation

Status: `research_required`

Orekit may be used later as an external reference oracle or architecture-comparison source. It is not implementation source for this Rust foundation.

## Prohibited material

The foundation must not copy, translate, mechanically restate, or structurally mirror Orekit Java source, comments, tests, fixtures, class hierarchy, method decomposition, control flow, exception text, package layout, or internal data model. It must not bundle an Orekit archive, Java runtime, generated output, or external reference fixture.

## Permitted inputs

Permitted inputs are independently selected public mathematical relations, exact public-format specifications reviewed through AeroCodex source governance, hand-written synthetic values, and AeroCodex-native metadata contracts. Generic terms such as epoch, frame, Cartesian state, classical elements, Kepler equation, TLE, and tolerance are domain vocabulary rather than copied architecture.

## Independent module rationale

The modules are organized around bounded AeroCodex responsibilities:

- `time`: label-preserving scalar records and explicit conversion blockers;
- `frames`: labels and fail-closed transform contracts, not a frame tree;
- `state`: SI Cartesian containers and validation;
- `elements`: standard vector equations with explicit singularity bands;
- `kepler`: bounded scalar elliptic helpers;
- `oracle`: evidence metadata and tolerance comparison, not an external runtime adapter;
- `tle`: review obligations only, not parser behavior.

This organization is intentionally small, dependency-free, and unlike a Java toolkit object hierarchy.

## Test-data boundary

Rust tests and the smoke example use source-free synthetic values selected to exercise mathematical domains and failure paths. No live or historical satellite record, recognizable TLE pair, Orekit fixture, or copied expected-output table is included. The TLE module contains no complete line string at all.

Future external evidence must be stored outside the core implementation until provenance, license/access, retrieval date, transformation history, expected-value procedure, units, frame, time scale, epoch, and hash have been reviewed.

## Oracle boundary

The `oracle` module stores context-complete records and compares caller-supplied values. It does not invoke Orekit, Java, subprocesses, network services, or fixture files. A source label containing the word Orekit would identify an external evidence producer only; it would not authorize source import or equivalence claims.

## Review evidence for this handoff

The static review checks source paths and text for Java files, Orekit package/class markers, recognizable TLE line prefixes, external fixture paths, archives, unsafe Rust, local absolute paths, credentials, and readiness claims. These scans are necessary but not proof of independent implementation. Live reviewer inspection and repository gates remain required.

## Non-claim

This boundary does not establish mathematical correctness, source traceability, parity with Orekit, operational tracking capability, flight readiness, mission readiness, certification, or regulated-use approval.
