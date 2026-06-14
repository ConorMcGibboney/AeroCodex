# ADR 0001: Use dual MIT OR Apache-2.0 licensing

## Status

Accepted

## Decision

AeroCodex source code is licensed as `MIT OR Apache-2.0`.

## Rationale

This is the standard licensing pattern for many Rust ecosystem crates. It maximizes reuse while preserving an explicit Apache-2.0 patent grant path.

## Consequences

- Each crate uses `license = "MIT OR Apache-2.0"`.
- Root files include `LICENSE`, `LICENSE-MIT`, `LICENSE-APACHE`, and `NOTICE`.
- Contributions are accepted under the same dual license unless stated otherwise.
- Reference datasets still require separate provenance and redistribution review.
