# Contributing to AeroCodex

AeroCodex accepts contributions that improve source-traceable, unit-safe, verified aerospace engineering mathematics in Rust.

## Contribution rules

Every stable public calculation must have:

- a stable Codex ID;
- a machine-readable evidence card;
- source, assumptions, units, validity range, branch behavior, and failure behavior;
- tests linked to the evidence card;
- documentation with at least one executable Rust example;
- no silent extrapolation;
- no hidden foreign-language runtime dependency.

## Checklist for new models

- [ ] Function/model has an evidence card.
- [ ] Source and equation references are complete.
- [ ] Units and dimensionless quantities are typed or strongly named.
- [ ] Domains and branches are explicit.
- [ ] Invalid inputs return specific errors.
- [ ] Unit tests are added.
- [ ] Property, limit, inverse, or reference-data tests are added where applicable.
- [ ] Reference data is reviewed and licensed.
- [ ] Rustdoc example compiles.
- [ ] Theory note is updated.
- [ ] Release report recognizes the new evidence card.
- [ ] No forbidden dependency or feature is introduced.

## Contribution license

Unless explicitly stated otherwise, any contribution intentionally submitted for inclusion in AeroCodex is licensed under `MIT OR Apache-2.0`, without additional terms or conditions.
