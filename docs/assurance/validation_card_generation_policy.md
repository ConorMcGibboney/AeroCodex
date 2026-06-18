# Validation-card generation policy

Status: Stage 5 Session D design policy. This policy defines a conservative way to generate formula-family validation-card skeletons. The checked-in template is non-operative governance metadata; this chunk implements no generator, validates no formula, and upgrades no validation status.

## Purpose

Formula-family validation cards are repetitive governance artifacts. A future `xtask` generator may create bounded skeleton cards so reviewers can focus on source boundaries, assumptions, tests, failure modes, and non-claims instead of hand-copying boilerplate.

Any future generator design is permitted to create only research-planning skeletons, and no generator is implemented here. It must never create implementation code, Rust public APIs, equation kernels, external fixtures, certification evidence, or operational-readiness claims.

## Current checked-in template stance

The current `xtask` verifier recursively scans every `.yaml` file below `validation/cards/`. Therefore the checked-in template at `validation/cards/templates/template_formula_family_validation_card.yaml` is a verifier-safe non-operative template record rather than a literal placeholder file with invalid IDs.

A future generator may render slice-specific output from this record and the policy below, but generated files must replace the template IDs, names, paths, scope text, tests, failure modes, and notes with slice-specific review content before deployment.

## Required generated card fields

Every generated formula-family validation card must include the following fields:

| Field | Policy |
| --- | --- |
| `id` | Stable dotted ID, normally `validation.formula_vault.<slice_id>` or another reviewer-approved family namespace. |
| `name` | Human-readable formula-family card name. |
| `category` | One of the canonical validation-card categories already accepted by `xtask`; formula-vault governance cards normally use `validation`. |
| `status` | Must default to `research_required`. A generator must reject any request to start at a higher status. |
| `source.id` | Must match a source-registry seed that exists or is generated in the same reviewed patch. |
| `source.status` | Must default to `research_required` and must not exceed the card status. |
| `assumptions` | Nonempty review assumptions, including bounded family scope and source-boundary assumptions. |
| `domain` | Summary plus bounds/exclusions for variables, units, frames, time-scales, branches, singularities, and excluded cases where relevant. |
| `inputs` | Nonempty list of source metadata, candidate manifests, contract files, independent tests, or other review inputs. |
| `outputs` | Nonempty list of planned outputs; skeletons must not imply an implementation exists unless it does. |
| `tests` | Check commands or planned tests. Passing tests must not be phrased as validation evidence unless evidence is actually present. |
| `failure_modes` | Nonempty list of ways the family could be mis-scoped, mis-generated, or over-claimed. |
| `non_claims` | Explicit list preserving no certification, no flight/mission/operational readiness, no medical/habitat safety, no regulated-use approval, no external parity without evidence, and no bulk source import. |
| `notes` | One paragraph with conservative status and applicability limits. |

## Generator input contract

A future generator should accept a small reviewed input contract rather than infer intent from arbitrary files. The minimum contract should contain:

- `slice_id`: lowercase identifier using only ASCII lowercase letters, digits, and underscores;
- `family_title`: reviewer-facing title;
- `category`: canonical card category, defaulting to `validation` for formula-vault governance;
- `source_seed_id`: exact dotted source-registry seed ID;
- `family_scope`: bounded description of the formula family;
- `excluded_cases`: branch, singularity, frame/time, solver, external-data, or source-boundary exclusions;
- `input_artifacts`: repo-relative metadata, contract, or test-plan paths;
- `planned_outputs`: metadata and tests that the chunk proposes;
- `required_checks`: exact commands the deployment agent must run;
- `failure_modes`: family-specific risks;
- `non_claims`: explicit no-claim list.

The generator must reject missing required fields. It must not fill unknown scientific details from defaults.

## Output and overwrite policy

Default command behavior in any future generator design must be dry-run only. Dry-run output may be printed to stdout or written under an ignored scratch path, but it must not alter repo files.

A future write mode would require separate review and authorization, must require an explicit flag, and must refuse to overwrite any existing validation card unless an explicit reviewer-approved overwrite flag is supplied. The generator must display the target path before writing.

Recommended generated path:

```text
validation/cards/validation_<slice_id>.yaml
```

The generator must prevent path traversal, absolute paths, hidden files, and path separators inside `slice_id`.

## Status and evidence policy

Generated cards start at `research_required`. They may describe required evidence, but they are not evidence themselves. A status upgrade requires a separate reviewed patch that documents source edition, equation/table/page/function locators, independent tests, reference values, tolerance policy, and any external-oracle comparisons needed for that family.

The generator must reject status values outside the canonical vocabulary. It must also reject requests to create fields or prose that imply certification, flight readiness, mission readiness, operational approval, habitat safety, medical use, or regulated-use approval.

## Source-boundary policy

Generated cards must not include raw M07 source text, generated code, Scilab outputs, external archives, binaries, GPL BioSim Java text, Orekit Java text, or copied comments/control flow from any external source. Source locators are permitted only as concise repo-relative or external-governance identifiers without source-code excerpts.

## Inventory policy

Adding a generated card under `validation/cards/` changes the scaffold counts because the current verifier counts all YAML files recursively. A deployment chunk that writes a new card must also update `validation/equation_inventory.tsv` with one `validation_card_only_record` row for the generated card unless the verifier is changed to exclude templates or generated drafts.

This policy patch includes one inventory row for the checked-in template card because the current verifier counts it.

## Review checklist for generated validation cards

Before deployment, the reviewer must confirm:

1. the card source ID exists in `validation/source_registry/`;
2. the status remains `research_required` unless separate evidence justifies an upgrade;
3. the card has nonempty assumptions, inputs, outputs, tests, failure modes, non-claims, and notes;
4. the target family scope is bounded and excludes ambiguous formulas;
5. no source text, generated code, external archive, binary, fixture, or public API has been imported;
6. equation-inventory count deltas match the actual file additions;
7. `cargo run -p xtask -- verify --all` passes after the patch is applied.
