# Formula vault

This directory is a Stage 4 skeleton for future formula-vault metadata. It intentionally contains no imported M07 source code, no external archives, no generated binaries, and no public AeroCodex implementation application programming interfaces.

AeroCodex remains research and preliminary-design software. This skeleton does not imply certification, flight readiness, mission readiness, habitat-safety approval, medical-use approval, operational approval, or regulated-use approval.

## Current state

Stage 4 Chunk 3 defines the vault shape in documentation only:

- `docs/assurance/formula_vault_staging.md`
- `docs/source_intake/m07_formula_vault_intake.md`

Stage 4 Chunk 7A adds the first candidate-gate metadata package without selecting or implementing a formula:

- `docs/assurance/formula_vault_candidate_gate.md`
- `formula-vault/templates/implementation_candidate_slice.yaml`
- validation card `validation.formula_vault.candidate_gate`
- source seed `source.validation.formula_vault_candidate_gate.research_required`

Stage 4 Chunk 7B adds the first bounded metadata-only candidate slice without implementing formulas:

- `formula-vault/candidates/m00_angle_unit_conversions.yaml`
- `docs/assurance/formula_vault_m00_angle_unit_candidate.md`
- validation card `validation.formula_vault.m00_angle_unit_conversions`
- source seed `source.formula_vault.m00_angle_unit_conversions.research_required`

The Chunk 7B slice is limited to three M00 release-gate rows: `app_deg2rad`, `app_rad2deg`, and `app_wrap2pi`. It does not promote exact expressions, wrap endpoint behavior, tolerances, executable code, or public application programming interfaces.

Stage 4 Chunk 8A handoff expands the formula-vault with a bounded M00 vector-algebra slice and implementation-ready research-kernel patch:

- `formula-vault/candidates/m00_vector_algebra.yaml`
- `formula-vault/contracts/m00_vector_algebra_contract.yaml`
- `docs/assurance/formula_vault_m00_vector_equation_expansion.md`
- validation card `validation.formula_vault.m00_vector_algebra`
- source seed `source.formula_vault.m00_vector_algebra.research_required`

The handoff covers fourteen finite 3-vector helpers, plus implementation of the already-contracted `deg2rad` and `rad2deg` helpers. Post-Stage-5 adds only the bounded `m00_wrap2pi` Rust runtime for `formula_vault.m00.angle.wrap2pi`; `app_resolve_coplanar` remains blocked for a separate least-squares/rank/tolerance policy chunk.

Stage 4 Chunk 7C adds the first dependency-free candidate metadata verifier without implementing formulas:

- command `cargo run -p xtask -- verify formula-vault`
- assurance note `docs/assurance/formula_vault_candidate_verifier.md`
- validation card `validation.formula_vault.candidate_verifier`
- source seed `source.validation.formula_vault_candidate_verifier.research_required`

The Chunk 7C verifier checks required metadata fields, cross-links, duplicate slice/formula identifiers, blocked promotion state, non-claim booleans, and absence of local evidence paths. It is included in `cargo run -p xtask -- verify --all`.

Stage 4 Chunk 7D adds the first per-candidate manifest/reference-link package without implementing formulas:

- `formula-vault/manifests/m00_angle_unit_conversions_manifest.yaml`
- `docs/assurance/formula_vault_m00_reference_manifest.md`
- validation card `validation.formula_vault.m00_reference_manifest`
- source seed `source.formula_vault.m00_reference_manifest.research_required`

The Chunk 7D manifest links each selected formula identifier to row/function/source-file aliases, pending source-expression review status, and assurance/validation/source/intake records. It does not copy source expressions, import M07 source, execute Scilab, promote fixtures, implement formulas, or create public application programming interfaces.

The M07 source artifact remains registered externally as `stage4.m07_rust_port_v14.2026_06_15` in `data-governance/DATA_REGISTRY.yaml`.

## Future allowed contents

Future chunks may add reviewed metadata such as:

- formula inventory records;
- equation contract drafts;
- source artifact identifiers;
- source equation/table/page/function-row references;
- variables, units, coordinate/time assumptions, domains, exclusions, and singularities;
- tolerance and reference-oracle plans;
- promotion-gate checklists.

Future chunks must not add raw M07 source code or public application-programming-interface implementation files here unless the active prompt explicitly authorizes that bounded scope and the required gates pass.


Stage 4 Chunk 7F adds a metadata-only source-expression and test-vector contract for the existing M00 angle/unit candidate:

- `formula-vault/contracts/m00_angle_unit_conversions_contract.yaml`
- `docs/assurance/formula_vault_m00_source_expression_test_vectors.md`
- validation card `validation.formula_vault.m00_source_expression_test_vectors`
- source seed `source.formula_vault.m00_source_expression_test_vectors.research_required`

The contract records independent mathematical summaries, finite-input domains, tolerance metadata, and endpoint-sensitive `wrap2pi` expectations. Post-Stage-5 deploys the single public `m00_wrap2pi` Rust API with research_required status, finite-input validation, `rem_euclid(std::f64::consts::TAU)`, [0, TAU) output, canonical positive zero, nonfinite rejection, no epsilon/ordinary-value clamping, and no M07/Scilab parity claim. It does not import M07 source, generate Rust from source material, import Scilab outputs or fixtures, or promote alternate public aliases.
