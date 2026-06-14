# ADR 0002: Adopt a 100% Rust core policy

## Status

Accepted

## Decision

The stable core of AeroCodex must not compile, link, execute, or require foreign-language source or binary dependencies.

## Consequences

- External aerospace tools may be used as source references or benchmark generators, not runtime dependencies.
- CI includes a dependency-policy gate.
- Native wrapper crates are denied by default.
