# Agent Instructions for AeroCodex

Agents working in this repository must preserve the AeroCodex trust model.

## Non-negotiables

- Do not add C, C++, Fortran, Python, MATLAB, Julia, BLAS, LAPACK, REFPROP, CEA, Cantera, OpenFOAM, SU2, SPICE, or proprietary binary dependencies to the core workspace.
- Do not introduce `cc`, `cmake`, `bindgen`, `pkg-config`, `vcpkg`, or `*_sys` crates without an explicit architecture decision record and maintainer approval.
- Do not make certification, flightworthiness, launchworthiness, spacecraft, safety-critical, or mission-critical claims.
- Do not silently extrapolate beyond declared model validity.
- Do not accept raw `f64` for public unit-bearing APIs when a typed quantity or strong dimensionless type is available.
- Do not add a stable public calculation without an evidence card and tests.
- Keep the public project phase at `0.001` until the Phase 0.001 exit gates in `docs/VERSIONING_PLAN.md` are satisfied.
- Agents may plan, retrieve, explain, and route; Rust owns calculations, validation, units, traces, and reproducibility.

## Required checks before proposing a change

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo run -p xtask -- verify --all
cargo run -p xtask -- dependency-policy
cargo run -p aero-codex-agent-cli -- build-index --check
cargo run -p xtask -- agent-index
cargo doc --workspace --all-features --no-deps
```
