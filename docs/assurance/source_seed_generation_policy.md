# Source-seed generation policy

Status: Stage 5 Session D design policy. This policy defines a conservative way to generate source-registry seed skeletons for formula-family review. The checked-in template is non-operative governance metadata; this chunk implements no generator, asserts no source traceability, validates no formula, and authorizes no source import.

## Purpose

Source-registry seeds capture what source evidence a future formula-family review must collect. A seed is a request and traceability scaffold, not evidence by itself. A future `xtask` generator may create consistent source-seed skeletons so reviewers can keep source-intake boundaries explicit, but no generator or write mode is implemented or authorized here.

## Current checked-in template stance

The current `xtask` verifier recursively scans every `.yaml` file below `validation/source_registry/`. Therefore the checked-in template at `validation/source_registry/templates/template_formula_family_source_seed.yaml` is a verifier-safe non-operative template record. It is counted as a source-registry seed by the current scaffold and must remain conservative.

A future generator may render slice-specific seeds from this template and policy. Generated seeds must replace template IDs and generic text with bounded slice-specific source-request details before deployment.

## Required generated source-seed fields

Every generated formula-family source seed must include the following fields:

| Field | Policy |
| --- | --- |
| `id` | Stable dotted source ID beginning with `source.`; normally `source.formula_vault.<slice_id>.research_required`. |
| `title` | Human-readable source-seed title for the formula family. |
| `category` | Canonical source-registry category accepted by `xtask`; formula-vault governance seeds normally use `validation` until a domain-specific category is reviewed. |
| `status` | Must default to `research_required`. A generator must reject higher initial status. |
| `intended_use` | Nonempty list describing the bounded review purpose and the fact that the seed is not validation evidence. |
| `requested_details` | Nonempty list of source locators, variables, units, domain, branch, singularity, tolerance, and evidence details needed from reviewers. |
| `implementation_notes` | Nonempty list documenting independent implementation expectations and source-boundary constraints. |
| `limits` | Nonempty list of no-claim, no-bulk-import, and no-external-parity limits. |
| `notes` | Conservative prose summary preserving `research_required` status. |

## Generator input contract

A future generator should accept a reviewed source-seed request with:

- `slice_id`: lowercase ASCII letters, digits, and underscores only;
- `source_seed_id`: exact dotted source-registry ID;
- `title`: source-seed title;
- `category`: canonical source-registry category;
- `family_scope`: bounded scope;
- `source_locator_request`: concise locators to collect without source-code excerpts;
- `variables_units_frames_time`: required convention review details;
- `validation_evidence_request`: analytical, reference-oracle, or test-vector evidence needed before status upgrade;
- `implementation_boundary`: independent implementation and no-import constraints;
- `limits`: explicit no-claim and no-readiness limits.

The generator must reject missing source locator requirements. It must not infer source evidence from file names alone.

## Output and overwrite policy

Default command behavior in any future generator design must be dry-run only. A dry run must show the generated seed and target path without writing repo files.

Recommended generated path:

```text
validation/source_registry/source_<slice_id>.yaml
```

A future write mode would require separate review and authorization, and must refuse to overwrite existing seeds unless an explicit reviewer-approved overwrite flag is supplied. The generator must prevent absolute paths, path traversal, hidden paths, and path separators inside `slice_id`.

## Source-boundary policy

Generated source seeds must request locators and evidence; they must not embed raw external source text. In particular, they must not copy raw M07 source, M07 comments, M07 control flow, generated Rust, Scilab outputs, GPL BioSim Java text, Orekit Java text, external archives, binaries, or fixture data.

Acceptable seed content includes concise references such as source registry IDs, function aliases, row identifiers, equation/table/page identifiers, and repo-relative metadata paths. Such references must not be represented as validation evidence until reviewed.

## Status and non-claim policy

Generated seeds start at `research_required`. A seed must not say or imply that a source has been validated, that implementation is verified, that external parity exists, or that any operational/certification/habitat/medical/regulated-use readiness has been achieved.

A source seed may list required future evidence. It must not claim the evidence exists unless the deployment patch includes reviewed artifacts and the validation status is updated under the normal status-upgrade policy.

## Review checklist for generated source seeds

Before deployment, the reviewer must confirm:

1. the seed ID matches any validation card that references it;
2. the seed status remains `research_required` unless separate evidence justifies an upgrade;
3. requested details are bounded and do not import source text;
4. implementation notes preserve independent implementation and no bulk import;
5. limits include no certification, no operational readiness, no habitat safety, no medical use, no regulated-use approval, and no external parity without evidence;
6. `cargo run -p xtask -- verify source-registry` and `cargo run -p xtask -- verify cards` pass after the patch is applied.
