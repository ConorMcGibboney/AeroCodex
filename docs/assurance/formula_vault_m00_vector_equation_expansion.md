# Formula-vault M00 vector-equation expansion

This Stage 4 handoff expands the formula-vault beyond the first three angle/unit candidates by adding a bounded M00 vector-algebra slice and a matching independent Rust research-kernel implementation plan.

AeroCodex remains research/preliminary-design software. This expansion is not certification evidence, flight-readiness evidence, mission-readiness evidence, operational approval, habitat-safety approval, medical-use approval, or regulated-use approval.

## What is added

The machine-readable vector candidate record is:

```text
formula-vault/candidates/m00_vector_algebra.yaml
```

The independent contract record is:

```text
formula-vault/contracts/m00_vector_algebra_contract.yaml
```

The candidate slice covers fourteen M00 vector helpers:

- `formula_vault.m00.vector.dot`
- `formula_vault.m00.vector.norm`
- `formula_vault.m00.vector.cross`
- `formula_vault.m00.vector.unit`
- `formula_vault.m00.vector.angle`
- `formula_vault.m00.vector.projection`
- `formula_vault.m00.vector.scalar_triple`
- `formula_vault.m00.vector.vector_triple`
- `formula_vault.m00.vector.vector_triple_bac_cab`
- `formula_vault.m00.vector.is_collinear`
- `formula_vault.m00.vector.is_coplanar`
- `formula_vault.m00.vector.tangent_from_dr_ds`
- `formula_vault.m00.vector.velocity_from_arc_rate`
- `formula_vault.m00.vector.distance`

The implementation handoff also includes the already-contracted `deg2rad` and `rad2deg` formulas from `formula-vault/contracts/m00_angle_unit_conversions_contract.yaml`. `wrap2pi` remains intentionally blocked unless a future chunk separately authorizes endpoint-sensitive implementation.

## Why this is a good expansion slice

The chosen vector helpers are low-dependency algebraic identities. They avoid time standards, frames, ephemerides, Kepler solvers, singular orbital elements, SGP4, and external fixture interpretation. They can be tested against independent analytical identities before any Scilab-equivalence work is attempted.

The excluded M00 helper is `app_resolve_coplanar`, because it uses a least-squares solve. That needs a separate rank, tolerance, and singularity policy before implementation.

## Source boundary

The M07 source artifact remains external and quarantined as `stage4.m07_rust_port_v14.2026_06_15`. The candidate and contract files record row/function/line-span locators and independent mathematical summaries only. They do not commit raw M07 source code, copied comments, copied control flow, generated Rust, Scilab output, external fixtures, archives, or binaries.

The M07 facts remain unchanged:

- 1,350 represented function rows;
- 188 Scilab equivalence jobs;
- release-candidate / not certified status.

## Implementation boundary

The code handoff adds bounded public Rust research functions in `crates/aero-codex-astrodynamics/src/lib.rs`. These are executable research kernels, not application-interface promotion and not a readiness claim.

The functions must reject non-finite inputs. Normalization-like operations must reject zero vectors. Predicate helpers must use explicit nonnegative tolerance inputs. Future Scilab-equivalence and external oracle evidence remain pending.

## Expected inventory effect

If the patch lands as drafted, the intended governed count changes are:

```text
executable_research_equations: 112 -> 128
metadata_only_formula_vault_candidates: 3 -> 17
external_m07_backlog_rows: 1347 -> 1333
validation_cards: 37 -> 38
source_registry_seeds: 35 -> 36
validation_card_only_records: 37 -> 38
```

The agent must rerun the equation-inventory verifier and recompute data-registry aggregate hashes on live `main` before merging.
