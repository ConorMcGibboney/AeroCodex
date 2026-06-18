# Formula-vault M00 canonical unit conversion expansion

This Stage 4 Chunk 8C handoff proposes a bounded second low-risk M00 scalar/math slice: canonical unit construction and scalar conversion helpers. The slice is designed to add ten executable research kernels without introducing frame, epoch, orbit-propagator-frame, solver, rank, branch-cut, or external-table dependencies.

AeroCodex remains research/preliminary-design software. This expansion is not certification evidence, flight-readiness evidence, mission-readiness evidence, operational approval, habitat-safety approval, medical-use approval, or regulated-use approval.

## Selected formula family

The selected candidate record is:

```text
formula-vault/candidates/m00_canonical_unit_conversions.yaml
```

The independent contract record is:

```text
formula-vault/contracts/m00_canonical_unit_conversions_contract.yaml
```

The slice covers ten formulas:

- `formula_vault.m00.canonical.time_unit_from_mu_du`
- `formula_vault.m00.canonical.speed_unit_from_du_tu`
- `formula_vault.m00.canonical.speed_unit_from_mu_du`
- `formula_vault.m00.canonical.mu_from_units`
- `formula_vault.m00.canonical.distance_to_canonical`
- `formula_vault.m00.canonical.distance_from_canonical`
- `formula_vault.m00.canonical.time_to_canonical`
- `formula_vault.m00.canonical.time_from_canonical`
- `formula_vault.m00.canonical.speed_to_canonical`
- `formula_vault.m00.canonical.speed_from_canonical`

## Why this is low-risk

These helpers are finite scalar algebra over caller-supplied positive unit scales. They do not depend on hidden global state, external tables, coordinate frames, time scales, Earth orientation, ephemerides, iterative solvers, rank-sensitive least-squares logic, or angle endpoint policies.

The implementation draft intentionally avoids string-dispatch unit conversion APIs and avoids importing the M07 constants table. Instead, callers provide the distance unit, time unit, and gravitational parameter explicitly in consistent physical units.

## Exclusions

The following M00 rows remain outside this slice:

- `app_wrap2pi`, because endpoint behavior is isolated to the wrap2pi policy chunk.
- `app_resolve_coplanar`, because least-squares rank, tolerance, and singularity policy are unresolved.
- `app_ut_day_fraction`, because compact civil-time input encoding needs a dedicated contract.
- `app_constants`, because the constants table is not copied or promoted into AeroCodex code in this chunk.

## Count-accounting note

The repository verifier counts formula-vault metadata by formula ID, not by exact external source row alias. This handoff adds ten formula-vault formula IDs and ten public research kernels while using seven M07 release-gate rows as locators. The deployment agent must use live verifier output as the source of truth and must not hard-code absolute counts.

## Source boundary

The M07 artifact remains external and quarantined. The handoff records row/function locators and independent mathematical summaries only. It does not import raw M07 source text, comments, control flow, generated Rust, Scilab outputs, archives, binaries, fixtures, evidence logs, or local absolute paths.

## Required deployment checks

The deployment agent should run the full repository gates after applying the patch, including formatting, linting, tests, formula-vault verification, equation-inventory verification, dependency policy, and documentation generation. If any count or line-number verifier differs on live `main`, regenerate the inventory from live sources before merge.
