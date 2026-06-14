# Prompt for a Deployment Agent — AeroCodex Phase 0.001

You are deploying the Phase 0.001 baseline for the private GitHub repository `ConorMcGibboney/AeroCodex`.

Your task is to initialize or update the repository with the provided AeroCodex Phase 0.001 foundation files as a clean deployment commit. AeroCodex is a 100% Rust project for verified, source-traceable aerospace engineering mathematics. Preserve the dual `MIT OR Apache-2.0` license, the certification caveat, the no-wrapper dependency policy, the validation evidence-card structure, the Phase `0.001` project label, and the Cargo-compatible `0.0.1` package version.

## Inputs

- Repository zip file: `AeroCodex_repository_foundation_v0_001.zip`
- Target repository: `git@github.com:ConorMcGibboney/AeroCodex.git` or the equivalent HTTPS remote
- Default branch: `main`
- Human project phase: `0.001`
- Cargo package version: `0.0.1`

## Deployment intent

This package supersedes the earlier `AeroCodex_repository_foundation_v0_0_1.zip` prompt. The older `v0_0_1` material and agentic pack are preserved in the founders bundle for traceability, but this zip is the one to deploy.

Phase 0.001 must remain in place until AeroCodex has basic Rust equations, source-verification records, tests, and evidence cards across:

1. foundational aerospace categories discussed in the founder plan;
2. bio-regenerative life support systems;
3. celestial mechanics / astrodynamics.

## Procedure

1. Clone the repository locally.

   ```bash
   git clone git@github.com:ConorMcGibboney/AeroCodex.git
   cd AeroCodex
   ```

2. If the repository is empty, continue. If it already has commits, create a branch first:

   ```bash
   git checkout -b chore/bootstrap-phase-0-001
   ```

3. Unzip `AeroCodex_repository_foundation_v0_001.zip` into a temporary directory.

4. Copy the contents of the top-level `AeroCodex_repository_foundation_v0_001/` directory into the repository root. Do not nest the files under an extra folder.

5. Inspect `git status` and verify that the root contains at minimum:

   ```text
   README.md
   Cargo.toml
   LICENSE
   LICENSE-MIT
   LICENSE-APACHE
   NOTICE
   AGENTS.md
   DEPLOY_AGENT_PROMPT.md
   .github/
   crates/
   docs/
   planning/
   validation/
   xtask/
   ```

6. Verify that the workspace includes starter crates for:

   ```text
   aero-codex-atmosphere
   aero-codex-thermo
   aero-codex-gasdynamics
   aero-codex-aerodynamics
   aero-codex-propulsion
   aero-codex-heat-transfer
   aero-codex-structures
   aero-codex-flight-dynamics
   aero-codex-astrodynamics
   aero-codex-bioregen
   aero-codex-agent-*
   ```

7. Run the verification commands:

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

8. If the Rust toolchain is missing, install stable Rust with `rustup` and rerun the checks. Do not change code merely to bypass a failed check; diagnose and fix the underlying issue.

9. Confirm there are no forbidden native dependencies or wrapper-tool dependencies. The core must not hide C, C++, Fortran, Python, MATLAB, Julia, BLAS/LAPACK, REFPROP, CEA, Cantera, OpenFOAM, SU2, SPICE, or proprietary binary dependencies.

10. Confirm that `Cargo.toml` keeps:

    ```toml
    [workspace.package]
    version = "0.0.1"

    [workspace.metadata.aerocodex]
    project_phase_version = "0.001"
    ```

11. Commit all files.

    For an empty repository:

    ```bash
    git add .
    git commit -m "chore: bootstrap AeroCodex phase 0.001"
    git push -u origin main
    ```

    For an existing repository branch:

    ```bash
    git add .
    git commit -m "chore: integrate AeroCodex phase 0.001 baseline"
    git push -u origin chore/bootstrap-phase-0-001
    ```

12. After pushing, confirm that GitHub Actions starts.

13. Open a deployment report containing:

    - commit hash;
    - branch name;
    - commands run;
    - pass/fail status for every command;
    - any diagnostics and fixes;
    - any deviations from this prompt;
    - whether Cargo version is `0.0.1` and project phase is `0.001`;
    - next recommended issue: `M0: convert Phase 0.001 founder baseline into tracked tasks`.

## Guardrails

- Do not replace the dual license with a single license.
- Do not remove the certification caveat.
- Do not remove the Phase `0.001` project label.
- Do not claim AeroCodex is certified, flight-ready, life-support-ready, mission-ready, validated for all aerospace use, or suitable for regulated use without project-specific assurance.
- Do not add generated binaries, target directories, local IDE files, or downloaded reference PDFs to the repository unless license/provenance review explicitly permits it.
- Do not add foreign-language dependencies or wrappers.
- Do not bypass failed checks by weakening lints, deleting tests, removing evidence cards, or relaxing dependency policy.
- Do not expose agent tools without evidence records, schemas, structured errors, and trace strategy.
