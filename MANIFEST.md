# AeroCodex Repository Foundation Manifest — Phase 0.001

This zip contains a deployable Phase 0.001 baseline for the AeroCodex GitHub repository.

## Status

- Human project phase: `0.001`
- Cargo package version: `0.0.1`
- Release status: founder research / starter equation baseline
- Certification status: not certified, not flight-ready, not life-support-ready, not mission-ready

## Included

### Repository governance

- `LICENSE`, `LICENSE-MIT`, `LICENSE-APACHE`, `NOTICE`
- `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`, `GOVERNANCE.md`, `SECURITY.md`, `SUPPORT.md`
- `AGENTS.md`, `DEPLOY_AGENT_PROMPT.md`

### Rust workspace

- `Cargo.toml`, `rust-toolchain.toml`, `rustfmt.toml`, `.cargo/config.toml`
- `crates/aero-codex-core/`
- `crates/aero-codex-units/`
- `crates/aero-codex-constants/`
- `crates/aero-codex-numerics/`
- `crates/aero-codex-validation/`
- `crates/aero-codex-data/`
- `crates/aero-codex-atmosphere/`
- `crates/aero-codex-thermo/`
- `crates/aero-codex-gasdynamics/`
- `crates/aero-codex-aerodynamics/`
- `crates/aero-codex-propulsion/`
- `crates/aero-codex-heat-transfer/`
- `crates/aero-codex-structures/`
- `crates/aero-codex-flight-dynamics/`
- `crates/aero-codex-astrodynamics/`
- `crates/aero-codex-bioregen/`
- `crates/aero-codex-agent-*/`
- `crates/aero-codex-macros/`
- `xtask/`

### Starter equation categories

Phase 0.001 contains starter Rust equations in:

```text
atmosphere
thermodynamics
gas dynamics
aerodynamics
propulsion
heat transfer
structures
flight dynamics
celestial mechanics / astrodynamics
bio-regenerative life support
```

### Validation and research

- `validation/codex_card.schema.yaml`
- `validation/evidence_levels.yaml`
- `validation/evidence-cards/*.yaml`
- `validation/source-registry/seed_sources.yaml`
- `validation/schemas/`
- `validation/traces/`
- `validation/agent-index/`

### Planning and docs

- `docs/VERSIONING_PLAN.md`
- `docs/MILESTONES.md`
- `docs/CATEGORY_ADMISSION_PLAN.md`
- `docs/BIOREGENERATIVE_LIFE_SUPPORT_PLAN.md`
- `docs/ASTRODYNAMICS_PLAN.md`
- `docs/RESEARCH_AND_SOURCE_VERIFICATION_PLAN.md`
- `docs/AGENTIC_OPTIMIZATION_PLAN.md`
- `docs/POST_1_0_LONG_RANGE_PLAN.md`
- `docs/SESSION_DECISIONS_0_001.md`
- `docs/agent/` integrated agentic specs
- `planning/version_train_v0_001.yaml`
- `planning/milestones_v0_001_to_v1_0.csv`
- `planning/milestones_post_v1_0.csv`
- `planning/founder_backlog_v0_001.md`

## Deployment

Use the prompt in `DEPLOY_AGENT_PROMPT.md` or the top-level prompt shipped beside the zip.

## Known limitation of this artifact

This artifact was assembled without running `cargo` in the generation environment because the Rust toolchain was unavailable there. The deployment agent must run the full command list in `DEPLOY_AGENT_PROMPT.md` and fix any compile, format, lint, or test failures without weakening the project guardrails.
