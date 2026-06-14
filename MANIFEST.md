# AeroCodex Repository Foundation Manifest

This zip contains a deployable founder baseline for the AeroCodex GitHub repository.

## Included

- Dual license files: `LICENSE`, `LICENSE-MIT`, `LICENSE-APACHE`, `NOTICE`.
- Repository governance: `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`, `GOVERNANCE.md`, `SECURITY.md`, `SUPPORT.md`.
- Agent guidance: `AGENTS.md`, `DEPLOY_AGENT_PROMPT.md`.
- Rust workspace: `Cargo.toml`, `rust-toolchain.toml`, `rustfmt.toml`, `.cargo/config.toml`, `crates/`, `xtask/`.
- CI: `.github/workflows/ci.yml`, issue templates, PR template, Dependabot config, CODEOWNERS.
- Validation foundation: `validation/codex_card.schema.yaml`, `validation/evidence_levels.yaml`, example evidence card.
- Policy docs: certification caveat, Rust-only policy, dependency policy, V&V standard, architecture, data governance, roadmap, ADRs.

## Deployment

Use the prompt in `DEPLOY_AGENT_PROMPT.md`.
