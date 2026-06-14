# Prompt for a Deployment Agent

You are deploying the founder baseline for the private GitHub repository `ConorMcGibboney/AeroCodex`.

Your task is to initialize the repository with the provided AeroCodex foundation files exactly as a clean first commit. AeroCodex is a 100% Rust project for verified, source-traceable aerospace engineering mathematics. Preserve the dual `MIT OR Apache-2.0` license, the certification caveat, the no-wrapper dependency policy, and the validation evidence-card structure.

## Inputs

- Zip file: `AeroCodex_repository_foundation_v0_0_1.zip`
- Target repository: `git@github.com:ConorMcGibboney/AeroCodex.git` or the equivalent HTTPS remote
- Default branch: `main`

## Procedure

1. Clone the empty repository locally.
2. Unzip `AeroCodex_repository_foundation_v0_0_1.zip` into a temporary directory.
3. Copy the contents of the top-level `AeroCodex_repository_foundation_v0_0_1/` directory into the repository root. Do not nest the files under an extra folder.
4. Inspect `git status` and verify that the root contains `README.md`, `Cargo.toml`, `LICENSE`, `LICENSE-MIT`, `LICENSE-APACHE`, `NOTICE`, `.github/`, `crates/`, `docs/`, `validation/`, and `xtask/`.
5. Run the verification commands:

   ```bash
   cargo fmt --all -- --check
   cargo clippy --workspace --all-targets --all-features -- -D warnings
   cargo test --workspace --all-features
   cargo run -p xtask -- verify --all
   cargo run -p xtask -- dependency-policy
   cargo doc --workspace --all-features --no-deps
   ```

6. If the Rust toolchain is missing, install stable Rust with `rustup` and rerun the checks. Do not change code merely to bypass a failed check; diagnose and fix the underlying issue.
7. Confirm there are no forbidden native dependencies or wrapper-tool dependencies.
8. Commit all files with:

   ```bash
   git add .
   git commit -m "chore: bootstrap AeroCodex foundation"
   git push -u origin main
   ```

9. After pushing, confirm that the GitHub Actions workflow starts.
10. Open a short deployment report containing:
    - commit hash;
    - commands run;
    - pass/fail status;
    - any deviations from this prompt;
    - next recommended issue: `M0: convert founder baseline into tracked tasks`.

## Guardrails

- Do not replace the dual license with a single license.
- Do not remove the certification caveat.
- Do not add generated binaries, target directories, or local IDE files.
- Do not add foreign-language dependencies or wrappers.
- Do not claim AeroCodex is certified, flight-ready, mission-ready, validated for all aerospace use, or suitable for regulated use without project-specific assurance.
