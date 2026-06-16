# Equation inventory and readiness dashboard

Stage 4 Chunk 7E adds a machine-readable equation inventory at `validation/equation_inventory.tsv` and a dependency-free verifier:

```text
cargo run -p xtask -- verify equation-inventory
```

The verifier is also wired into:

```text
cargo run -p xtask -- verify --all
```

## Scope

This chunk is inventory/governance only.

It does not implement formulas, import M07 code, generate formula code, create a new runtime, create public application programming interfaces, execute Scilab jobs, bundle M07 archive contents, or promote any flight, mission, operational, certification, or regulated-use claim.

## Inventory classes

The inventory distinguishes:

- `executable_research_equation` — currently public Rust research/preliminary-design equation kernels in repository crates. These are executable but remain blocked from readiness promotion.
- `metadata_only_formula_vault_candidate` — formula-vault formula identifiers selected as metadata-only candidates, currently the M00 angle/unit slice.
- `external_m07_backlog_row` — aggregate external M07 backlog rows not represented as metadata-only formula-vault candidates.
- `validation_card_only_record` — validation-card metadata rows. These records are not formula implementations.
- `helper_algorithm` — public support/helper routines, validation helpers, type constructors, provenance helpers, BioSim governance primitives, and other support algorithms not counted as executable research equations.

## Final Stage 4 Chunk 7E counts

The verifier is expected to report:

- executable research equations: 112
- metadata-only formula-vault candidates: 3
- external M07 backlog rows: 1347
- validation cards: 36
- source-registry seeds: 34
- validation-card-only records: 36
- helper algorithms: 89

The external M07 count is derived from the existing registered M07 represented function-row count of 1350 minus the three metadata-only formula-vault candidate identifiers already selected for M00 angle/unit review.

## Readiness rule

Every inventory row is explicitly blocked. The block reason is row-local and machine-readable. No row may claim certification, flight readiness, mission readiness, operational approval, or regulated-use approval. The dashboard answers readiness by showing which class an item belongs to and why it is blocked before any future implementation chunk can be considered.
