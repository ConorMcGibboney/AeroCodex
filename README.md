# AeroCodex

**Verified Aerospace Engineering Mathematics in Pure Rust**

AeroCodex is a 100% Rust workspace for source-traceable aerospace engineering mathematics. It is designed as a living engineering codex where each stable calculation is typed, source-traceable, range-checked, branch-explicit, tested, and honest about assumptions, limitations, numerical behavior, and verification status.

## Current status: Project Phase 0.001

This repository is at **AeroCodex Project Phase 0.001**.

The Cargo-compatible package version is **0.0.1**. The human planning/version label remains **0.001** until AeroCodex has basic Rust equations, evidence cards, tests, and source-verification plans across:

1. foundational aerospace math categories discussed in the founder plan;
2. bio-regenerative life support systems;
3. celestial mechanics / astrodynamics.

No production engineering, flight-readiness, mission-readiness, regulatory, or certification claim is made by this baseline.

## North Star

Give the engineer the answer. Tell them where it came from. Tell them when not to trust it. Tell them how it was checked. Never silently extrapolate. Never hide units, frames, assumptions, branches, or tolerances.

## Repository shape

```text
crates/
  aero-codex/                  # public umbrella crate
  aero-codex-core/             # results, evidence IDs, errors, assumptions, warnings
  aero-codex-units/            # typed engineering quantities and dimensionless types
  aero-codex-constants/        # sourced physical constants
  aero-codex-numerics/         # solver traits and verified numerical kernels
  aero-codex-validation/       # evidence-card and release-report primitives
  aero-codex-data/             # dataset provenance records
  aero-codex-atmosphere/       # atmosphere and environment starter equations
  aero-codex-thermo/           # thermodynamics starter equations
  aero-codex-gasdynamics/      # compressible-flow starter equations
  aero-codex-aerodynamics/     # aerodynamics starter equations
  aero-codex-propulsion/       # propulsion starter equations
  aero-codex-heat-transfer/    # heat-transfer starter equations
  aero-codex-structures/       # structures and loads starter equations
  aero-codex-flight-dynamics/  # flight mechanics starter equations
  aero-codex-astrodynamics/    # celestial mechanics / astrodynamics starter equations
  aero-codex-bioregen/         # bio-regenerative life support starter equations
  aero-codex-agent-*/          # agentic discovery, schema, retrieval, trace, and tool scaffolds
validation/
  evidence-cards/              # machine-readable model evidence cards
  schemas/                     # JSON/YAML schemas, including agent-facing schemas
  source-registry/             # source-verification backlog and seed source records
  traces/                      # agent trace fixtures
planning/                      # milestone, versioning, and agentic work-package plans
docs/                          # policies, architecture, roadmap, theory, and agent specs
xtask/                         # repository automation and policy checks
```

## Quick start

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo run -p xtask -- verify --all
cargo run -p xtask -- dependency-policy
cargo run -p aero-codex-agent-cli -- build-index --check
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

## Versioning rule

AeroCodex uses two related but distinct version labels:

```text
Project phase label: 0.001  # human planning label, kept until the first multi-domain starter baseline is verified
Cargo SemVer:        0.0.1  # machine-compatible Rust package version
```

The project should not advance beyond **0.001** until the milestone gates in `docs/VERSIONING_PLAN.md` and `planning/version_train_v0_001.yaml` are satisfied.

## License

AeroCodex is licensed under either of:

- Apache License, Version 2.0
- MIT License

at your option.

Unless explicitly stated otherwise, contributions intentionally submitted for inclusion in AeroCodex are licensed under the same dual license.

## Certification caveat

AeroCodex is an engineering mathematics library with source traceability, tests, and validation evidence where stated. It is not by itself certified for safety-critical, airborne, launch, spacecraft, life-support, or mission-critical use. Regulated use requires project-specific requirements, verification, validation, configuration management, independence, assurance, and approval. Users are responsible for determining suitability for their application.
