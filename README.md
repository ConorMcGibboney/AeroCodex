# AeroCodex

**Verified Aerospace Engineering Mathematics in Pure Rust**

AeroCodex is a Rust workspace for source-traceable aerospace engineering mathematics. The goal is not to be a black-box calculator or a wrapper around legacy tools. The goal is a living engineering codex where each stable calculation is typed, source-traceable, range-checked, branch-explicit, tested, and honest about assumptions, limitations, numerical behavior, and verification status.

## North Star

Give the engineer the answer. Tell them where it came from. Tell them when not to trust it. Tell them how it was checked. Never silently extrapolate. Never hide units, frames, assumptions, branches, or tolerances.

## Current status

This repository is at **Founder Baseline v0.0.0**. It contains the project foundation, governance files, licensing, CI skeleton, validation-card schema, dependency policy, and a minimal Rust workspace that demonstrates the trust loop with one gas-dynamics example.

No production engineering claim is made by this baseline.

## Repository shape

```text
crates/
  aero-codex/              # public umbrella crate
  aero-codex-core/         # results, evidence IDs, errors, assumptions, warnings
  aero-codex-units/        # typed engineering quantities and strong dimensionless types
  aero-codex-constants/    # sourced physical constants
  aero-codex-numerics/     # solver traits and verified numerical kernels
  aero-codex-validation/   # evidence-card and release-report primitives
  aero-codex-data/         # dataset provenance records
  aero-codex-gasdynamics/  # first demonstration domain slice
validation/
  evidence-cards/          # machine-readable model evidence cards
docs/                      # policies, architecture, verification, roadmap
xtask/                     # repository automation and policy checks
```

## Quick start

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo run -p xtask -- verify --all
cargo run -p xtask -- dependency-policy
cargo doc --workspace --all-features --no-deps
```

## Example

```rust
use aero_codex_gasdynamics::normal_shock_p2_over_p1;
use aero_codex_units::{Mach, RatioOfSpecificHeats};

let result = normal_shock_p2_over_p1(
    Mach::new(2.0)?,
    RatioOfSpecificHeats::new(1.4)?,
)?;

assert!((result.value.value() - 4.5).abs() < 1.0e-12);
println!("evidence = {}", result.verification.id.as_str());
# Ok::<(), aero_codex_core::AeroError>(())
```

## License

AeroCodex is licensed under either of:

- Apache License, Version 2.0
- MIT License

at your option.

Unless explicitly stated otherwise, contributions intentionally submitted for inclusion in AeroCodex are licensed under the same dual license.

## Certification caveat

AeroCodex is an engineering mathematics library with source traceability, tests, and validation evidence where stated. It is not by itself certified for safety-critical, airborne, launch, spacecraft, or mission-critical use. Regulated use requires project-specific requirements, verification, validation, configuration management, independence, assurance, and approval. Users are responsible for determining suitability for their application.
