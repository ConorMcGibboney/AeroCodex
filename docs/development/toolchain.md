# Toolchain baseline

RR-003 records the professional open-source tooling baseline for AeroCodex as research/preliminary-design software. AeroCodex is not certified for flight, mission operations, habitat safety, medical/life-support decisions, or regulatory approval.

## Rust and Cargo

- Rust stable only. AeroCodex does not require nightly Rust for the first research-readiness milestone.
- The repository is cargo-first: use Cargo commands from the workspace root for formatting, linting, tests, documentation, and local governance checks.
- Optional `just` usage, if added later, is optional only. A contributor must be able to run the documented Cargo commands without installing `just`.
- The current workspace policy keeps the root `Cargo.lock` absent during this phase.

## Platform posture

- Linux and macOS first for contributor setup and continuous-integration expectations.
- Windows should not be intentionally broken. Windows-specific fixes are welcome when they preserve the same Cargo-first workflow and do not add phase-inappropriate dependencies.

## Dependencies not required for this phase

The first research-readiness milestone does not require:

- Python, Jupyter, web services, or API servers;
- nightly Rust;
- external native math libraries such as BLAS, LAPACK, CEA, REFPROP, CoolProp, or Cantera;
- compiled native binary blobs or generated binaries.

Existing source-intake, planning, or evidence materials may mention outside tools as references, but they are not required to build, test, document, or verify the Rust workspace in this phase.

## Generated registry posture

Generated registry artifacts should be checked in only when their generation is deterministic and governed by documented inputs, stable ordering, reviewable hashes, and local verification gates. RR-003 documents the tooling baseline only; it does not create or execute registry generation.
