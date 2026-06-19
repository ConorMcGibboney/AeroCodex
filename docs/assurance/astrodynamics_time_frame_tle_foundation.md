# Astrodynamics Time, Frame, State, Element, Kepler, Oracle, and TLE-Contract Foundation

Status: `research_required`

This note describes a bounded, independently written Rust foundation for `aero-codex-astrodynamics`. It provides explicit labels and fail-closed contracts before any high-fidelity time conversion, frame transformation, external-oracle execution, TLE parsing, or SGP4 work is attempted.

AeroCodex remains research and preliminary-design software. This foundation is not certified, flight-ready, mission-ready, navigation-ready, operational-tracking capable, or approved for regulated use.

## Module boundary

| Module | Responsibility | Explicit non-scope |
|---|---|---|
| `time.rs` | Finite scalar durations and epochs carrying TAI, UTC, TT, TDB-placeholder, or GPS-placeholder labels | Calendars, leap-second tables, civil UTC arithmetic, relativistic conversion, cross-scale conversion |
| `frames.rs` | Frame labels and fail-closed transform contracts | Frame tree, Earth orientation, precession/nutation, TEME-to-ITRF, GCRF realization, local-basis construction |
| `state.rs` | Finite SI Cartesian containers, mandatory frame/epoch labels, norms, and context guards | Propagation, estimation, covariance, perturbations, frame/time conversion |
| `elements.rs` | Standard vector-form classical elements for bounded, non-singular elliptic states | Circular/equatorial conventions, parabolic/hyperbolic extraction, modified elements, orbit determination |
| `kepler.rs` | Bounded elliptic mean motion, anomaly advance, safeguarded Kepler solve, and scalar radius/anomaly helpers | Parabolic/hyperbolic solvers, perturbations, universal variables, Cartesian propagation |
| `oracle.rs` | Context-complete comparison records and deterministic tolerance checks | Running Orekit, Java interop, fixture import, evidence generation, hash calculation |
| `tle.rs` | Contract-only TLE source-review and fixture-policy records | Parser, checksum implementation, field decoding, TLE fixtures, SGP4, TEME transforms |

Crate-root changes are module declarations only; no broad `pub use` block is introduced.

## Time contract

`AstroEpoch` stores a time-scale label and finite scalar seconds from a caller-defined reference epoch. It intentionally does not implement `PartialOrd`. Same-scale duration and ordering operations first require equal labels and reject cross-scale requests.

For UTC, same-label offsets are label-preserving scalar arithmetic only. They are not civil UTC arithmetic and do not account for leap seconds. TDB and GPS remain placeholder labels without conversion models.

## Frame contract

Frame values identify context; they do not realize coordinate transforms. Same-frame identity is permitted for every label, including label-only frames, because no relabeling or numerical conversion occurs. Every non-identity request is blocked as label-only, blocked as dynamically defined, or reported unsupported.

No frame tree or external-library-shaped class hierarchy is introduced.

## Cartesian-state contract

Position is in metres, velocity is in metres per second, and gravitational parameter is in cubic metres per square second. Finite-container validation is separate from orbital-use validation. A finite zero-position vector can exist as a synthetic record, while orbital-use validation rejects zero radius. Norm helpers use scaled arithmetic and fail closed if a finite vector still produces an unrepresentable result.

`StateWithFrameAndEpoch` contains mandatory frame and epoch values. Combining states requires an explicit same-frame and same-time-scale guard; this layer does not transform either context.

## Classical-element and Kepler contracts

Complete classical-element extraction is bounded to non-singular elliptic inputs expressed in one consistent inertial-like Cartesian basis at one epoch. Callers provide positive finite exclusion thresholds for angular momentum, node magnitude, eccentricity, and near-parabolic energy. Circular, equatorial, zero-angular-momentum, near-parabolic, parabolic, and hyperbolic cases return explicit errors.

The Kepler layer accepts eccentricity in `[0, 1)` only. Solver callers provide a positive finite residual tolerance and nonzero iteration cap. The implementation uses a safeguarded Newton step within a monotonic elliptic bracket and reports a recomputed final residual. It does not provide a Cartesian propagation service.

## Oracle-record contract

Oracle comparison cases require units, frame label and frame-context text, declared time scale, a matching labeled epoch, epoch-reference text, source/oracle label, input summary, expected-output summary, tolerance, and evidence state. Public comparison entry points operate only on validated records. Pending evidence yields a blocked status without comparison metrics.

The module does not run Orekit, execute Java, import external fixtures, retrieve evidence, or compute hashes.

## TLE contract-only boundary

The TLE layer names the line, field, epoch, unit, range, model-context, and fixture-provenance decisions that must receive exact source review before parser work begins. It contains no parser, checksum algorithm, epoch decoder, compact-exponent decoder, TLE-like line pair, SGP4 model, or TEME transform.

Even a structurally complete locator record requires human review and does not authorize parser implementation.

## Deterministic smoke example

`astrodynamics_foundation_smoke` uses one hand-written synthetic SI state and exercises only O2a/O2b behavior: TT label-preserving scalar time arithmetic, same-frame identity, Cartesian validation, norms, specific energy, angular momentum, non-singular elliptic element extraction, and a bounded Kepler solve. It performs no TLE or oracle execution.

The report has fixed ordering and numeric formatting, no timestamp, no random identifier, and no environment-dependent path. Its output includes an explicit non-claim caveat.

## Promotion blockers

For O2d closeout, the deployment agent must reconcile this final Orekit v3 subpatch against live `main`; run formatting, Clippy, tests, examples, docs, workspace gates, nomenclature gates, and source-boundary scans; reconcile the public-function inventory from rustfmt-stable live line numbers; recompute governed hashes from the tracked live tree; preserve `research_required`; and verify exact-head GitHub Actions after deployment.

A passing smoke example is not source validation, reference parity, accuracy evidence, operational readiness, or certification.
