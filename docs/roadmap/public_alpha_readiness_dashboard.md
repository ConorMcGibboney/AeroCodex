# Public alpha readiness dashboard

This dashboard is a tester-facing summary for the public friend-test package. It is not a release approval, certification artifact, safety case, mission-readiness case, habitat-safety case, medical-use case, or regulated-use approval.

AeroCodex is research/preliminary-design software. Passing local checks does not prove physical validity, safety, certification, or mission readiness.

## Dashboard scope

The public alpha friend-test package is intended to answer four narrow questions:

1. Can a tester clone the repository and run the configured local Rust and governance checks?
2. Does the equation inventory explain what is executable research code, metadata-only candidate material, validation-card-only metadata, and helper code?
3. Do testers see the project’s research-only and blocked status before using any formula?
4. Can failures be reported in a reproducible way without importing external source material or generated artifacts?

## Live counts for Session G deployment

The live Session G branch reports the following governed counts from `cargo run -p xtask -- verify equation-inventory`. These counts must be recomputed if another handoff lands before this branch merges.

| Inventory class | Live count after Session G | Session G delta | Meaning |
|---|---:|---:|---|
| Executable research equations | 138 | +0 | Public Rust research/preliminary-design equation kernels. |
| Metadata-only formula-vault candidates | 27 | +0 | Candidate metadata records; not implementations by themselves. |
| External M07 backlog rows | 1,323 | +0 | Quarantined represented rows not selected into formula-vault metadata. |
| Validation cards | 42 | +1 | Conservative validation/governance records. |
| Source-registry seeds | 40 | +1 | Source/governance traceability records. |
| Validation-card-only records | 42 | +1 | Metadata records, not formula implementations. |
| Helper algorithms | 138 | +0 | Support routines not counted as executable research equations. |

Session G does not add Rust equations, formula-vault candidates, external source imports, crates, dependencies, or public application programming interfaces. It adds friend-test docs, simple local scripts, one validation card, one source-registry seed, and one equation-inventory validation-card-only row.

## Public alpha lanes

| Lane | User-visible artifact | Current readiness label | Blocked from promotion because |
|---|---|---|---|
| Local clone checks | `docs/testing/friend_test_quickstart.md`, `scripts/friend_test_local.sh`, `scripts/friend_test_local.ps1` | research_required | Local checks are software gates only and do not prove physical validity or safety. |
| Expected-output guide | `docs/testing/friend_test_expected_output.md` | research_required | Output varies by toolchain and is not validation evidence. |
| Safety caveats | `docs/testing/research_safety_caveats_for_testers.md` | research_required | Caveats must stay visible for public testers. |
| Equation inventory | `validation/equation_inventory.tsv` and `cargo run -p xtask -- verify equation-inventory` | research_required | Inventory rows classify items; they do not validate formulas. |
| Formula-vault metadata | `formula-vault/` and `cargo run -p xtask -- verify formula-vault` | research_required | Candidate records remain metadata-only until a later authorized implementation chunk. |
| Dependency policy | `cargo run -p xtask -- dependency-policy` | research_required | Dependency hygiene is not physical validation or safety approval. |
| Documentation build | `cargo doc --workspace --all-features --no-deps` | research_required | Documentation generation does not create validation evidence. |

## Known blocked public-facing items

- `wrap2pi` remains blocked for a dedicated endpoint-behavior policy and test-vector chunk.
- `app_resolve_coplanar` remains blocked for least-squares, rank, singularity, and tolerance policy.
- BioSim-style clean-room primitives remain research-only and are not a habitat-control system.
- Orekit-related material remains reference-oracle planning only and is not copied into AeroCodex.
- M07 material remains quarantined source material and is not bulk-imported.

## Minimum friend-test acceptance criteria

A public friend-test report is useful if it includes:

- platform and shell;
- Rust and Cargo versions;
- commit hash if available;
- whether the Bash script, PowerShell script, or manual command list was used;
- the first failing command and nearby terminal output, or a note that all checks completed;
- whether a root `Cargo.lock` appeared after the run.

A clean report is still not a safety or certification claim.

## Deployment-agent refresh rule

Before merging this dashboard, the deployment agent should run the live inventory verifier and refresh any absolute counts affected by earlier serial handoffs. If the live baseline differs again before merge, keep the Session G deltas and update only the absolute values that the verifier proves.
