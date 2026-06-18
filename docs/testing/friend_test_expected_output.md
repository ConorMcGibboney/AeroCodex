# Friend-test expected output

This page describes the expected shape of the friend-test run. Exact Cargo output varies by Rust version, platform, and cache state, so testers should compare command success/failure rather than matching every line exactly.

AeroCodex is research/preliminary-design software. These outputs are local software-gate signals only. They are not physical-validation evidence, safety evidence, certification evidence, mission-readiness evidence, habitat-safety evidence, medical-use evidence, or regulated-use approval.

## Script output shape

Both scripts print a short banner, the repository root, Rust tool versions when available, and numbered check steps. A typical clean run has this structure:

```text
[friend-test] AeroCodex local friend-test package
[friend-test] repository root: <path-to-clone>
[friend-test] rustc: <version>
[friend-test] cargo: <version>
[friend-test] step 1/8: cargo fmt --all -- --check
[friend-test] step 2/8: cargo clippy --workspace --all-targets --all-features -- -D warnings
[friend-test] step 3/8: cargo test --workspace --all-features
[friend-test] step 4/8: cargo run -p xtask -- verify --all
[friend-test] step 5/8: cargo run -p xtask -- verify equation-inventory
[friend-test] step 6/8: cargo run -p xtask -- verify formula-vault
[friend-test] step 7/8: cargo run -p xtask -- dependency-policy
[friend-test] step 8/8: cargo doc --workspace --all-features --no-deps
[friend-test] completed all requested local checks
```

If a step fails, the scripts stop at that step and return a non-zero exit code. The first failing command is usually the most useful item to report.

## Expected command outcomes

| Step | Expected local outcome | What it means |
|---|---|---|
| `cargo fmt --all -- --check` | exits successfully | Rust formatting matches the checked-in style. |
| `cargo clippy --workspace --all-targets --all-features -- -D warnings` | exits successfully | Lints configured as warnings did not fire for the workspace under the selected toolchain. |
| `cargo test --workspace --all-features` | exits successfully | Workspace tests passed under the selected toolchain and platform. |
| `cargo run -p xtask -- verify --all` | exits successfully | The configured governance verifiers completed as a group. |
| `cargo run -p xtask -- verify equation-inventory` | exits successfully | Inventory rows and counts agree with the current repo state. |
| `cargo run -p xtask -- verify formula-vault` | exits successfully | Formula-vault metadata records pass the current metadata gate. |
| `cargo run -p xtask -- dependency-policy` | exits successfully | The workspace did not add dependency tokens blocked by the current policy. |
| `cargo doc --workspace --all-features --no-deps` | exits successfully | Workspace documentation builds without pulling dependency docs. |

## Current governed inventory snapshot

This Session G branch reports these live counts from `cargo run -p xtask -- verify equation-inventory` after applying the friend-test documentation package:

| Inventory class | Live count after Session G |
|---|---:|
| Executable research equations | 138 |
| Metadata-only formula-vault candidates | 27 |
| External M07 backlog rows | 1,323 |
| Validation cards | 42 |
| Source-registry seeds | 40 |
| Validation-card-only records | 42 |
| Helper algorithms | 138 |

If another handoff lands before this branch merges, deployment should recompute the live counts. The Session G deltas are `+0` executable research equations, `+0` formula-vault candidates, `+0` external M07 backlog rows, `+1` validation card, `+1` source-registry seed, `+1` validation-card-only record, and `+0` helper algorithms.

## Expected blocked states

A clean friend-test run still leaves blocked and research-only items blocked. In particular:

- `wrap2pi` remains blocked pending endpoint-behavior policy;
- `app_resolve_coplanar` remains blocked pending least-squares, rank, and tolerance policy;
- formula-vault candidates remain metadata-only unless a later chunk explicitly implements and validates them;
- validation cards remain conservative records and do not prove implementation or validation by themselves.

## Failure-report template

```text
OS and version:
Shell:
rustc --version:
cargo --version:
Repository commit:
Friend-test path: Bash script / PowerShell script / manual
First failing command:
Exit code:
Terminal excerpt:
Did Cargo generate a root Cargo.lock?: yes/no/unknown
Additional notes:
```
