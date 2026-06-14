# Architecture

AeroCodex is a Rust workspace, not a monolithic crate.

## Layers

```text
CLI, WASM, examples, reports
Domain crates: atmosphere, gas dynamics, aerodynamics, propulsion, structures, flight dynamics, astrodynamics, controls
Numerics, validation, data, uncertainty
Core types, units, constants, errors, frames, results
```

## Dependency rules

1. `aero-codex-core`, `aero-codex-units`, and `aero-codex-constants` shall not depend on domain crates.
2. Domain crates may depend on core, units, constants, numerics, validation, and data when necessary.
3. The umbrella crate may re-export stable foundation crates.
4. No crate shall create hidden global mutable state for model selection, units, frames, or tolerances.
5. The core product shall not use foreign-language runtime dependencies.

## Current baseline crates

- `aero-codex-core`
- `aero-codex-units`
- `aero-codex-constants`
- `aero-codex-numerics`
- `aero-codex-validation`
- `aero-codex-data`
- `aero-codex-gasdynamics`
- `aero-codex`
