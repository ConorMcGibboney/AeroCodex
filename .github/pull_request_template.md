## Summary

Describe the change and the affected Codex IDs.

## Checklist

- [ ] Code is Rust-only and introduces no forbidden native dependency path.
- [ ] Public unit-bearing APIs use typed quantities or strong dimensionless types.
- [ ] Branches and validity ranges are explicit.
- [ ] Evidence cards are added or updated.
- [ ] Tests are linked to evidence cards.
- [ ] Reference data has provenance and redistribution review.
- [ ] Rustdoc examples compile.
- [ ] Certification caveat remains intact.

## Checks run

```text
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo run -p xtask -- verify --all
cargo run -p xtask -- dependency-policy
cargo doc --workspace --all-features --no-deps
```
