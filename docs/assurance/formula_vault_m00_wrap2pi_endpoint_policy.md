# Formula-vault M00 wrap2pi endpoint policy

Stage 5 Session A isolates the endpoint behavior for `formula_vault.m00.angle.wrap2pi` / `app_wrap2pi` before any Rust implementation is considered.

## Decision

The endpoint interval is resolved as:

```text
reviewed_interval_0_to_2pi_exclusive_upper
[0, 2*pi)
```

The current formula-vault contract and source locator are sufficient to resolve the intended interval convention for metadata/test-contract deployment. The existing candidate remains blocked from public API promotion and executable implementation until a later chunk explicitly scopes code and tests.

## Source-boundary result

The review used existing AeroCodex metadata plus the registered M07 source locator for `app_wrap2pi`. No raw M07 source text, comments, source control flow, generated Rust, Scilab output, external fixture, archive contents, or local evidence log is copied here.

Relevant in-repo metadata paths are:

- `formula-vault/candidates/m00_angle_unit_conversions.yaml`;
- `formula-vault/contracts/m00_angle_unit_conversions_contract.yaml`;
- `formula-vault/manifests/m00_angle_unit_conversions_manifest.yaml`;
- `docs/assurance/formula_vault_m00_source_expression_test_vectors.md`.

The dedicated endpoint contract is:

```text
formula-vault/contracts/m00_wrap2pi_endpoint_contract.yaml
```

The dedicated CSV test-vector table is:

```text
formula-vault/contracts/m00_wrap2pi_test_vectors.csv
```

## Endpoint behavior

The output interval is lower-inclusive and upper-exclusive. Exact period multiples map to the lower endpoint. `pi`, `-pi`, `3*pi`, and `-3*pi` all map to `pi` under the `[0, 2*pi)` convention. Very small positive finite angles remain near the lower endpoint; very small negative finite angles map just below `2*pi`.

Non-finite inputs are outside the contract. A future Rust implementation should reject `NaN`, positive infinity, and negative infinity before attempting to wrap.

## Signed zero and f64 policy

The source locator is not used as a sign-bit oracle. For deterministic AeroCodex Rust tests, the contract chooses positive-zero canonicalization for every zero output, including input `+0`, input `-0`, and exact positive or negative period multiples. If the deployment agent rejects this policy, stop the chunk and classify it as `needs_review`; do not implement `wrap2pi`.

Large-magnitude behavior is limited to bounded f64 regression vectors. This contract does not claim exact behavior for arbitrary huge inputs where finite-precision remainder operations can lose low-order information.

## Required future tests before implementation

A later implementation chunk must cover every row in `formula-vault/contracts/m00_wrap2pi_test_vectors.csv`, including:

- zero, `+0`, and `-0`;
- positive and negative `2*pi` multiples;
- `pi`, `-pi`, `3*pi`, and `-3*pi`;
- very small positive and negative finite inputs;
- bounded large positive and negative multiples of `2*pi`;
- finite non-multiple values across quadrants;
- rejection of `NaN` and infinities.

## Non-claims

This policy does not implement `wrap2pi`, create a public API, run Scilab, import fixtures, certify source parity, or promote validation status beyond `research_required`. AeroCodex remains research/preliminary-design software only, not certified, not flight-ready, not mission-ready, not habitat-safe, not medical-use software, not operationally approved, and not approved for regulated use.
