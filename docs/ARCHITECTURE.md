# Architecture

AeroCodex is a Rust workspace with a small verified core, typed quantity layer, validation/evidence system, domain crates, and an optional agentic surface.

## Dependency direction

```text
Agent CLI / Local Tool Server / WASM Surface
        |
Agent registry / schema / retrieval / tools / trace
        |
Domain crates
        |
Validation / Data / Numerics
        |
Core / Units / Constants
```

Rules:

1. Core, units, constants, validation, data, and numerics must not depend on domain crates.
2. Domain crates may depend on core, units, constants, numerics, validation, and data.
3. Agent crates may depend on the verified math layer.
4. The verified math layer must not depend on the agent layer.
5. No core crate may hide C, C++, Fortran, Python, MATLAB, Julia, BLAS/LAPACK, REFPROP, CEA, Cantera, OpenFOAM, SU2, SPICE, or proprietary binary dependencies.

## Phase 0.001 crate families

```text
foundation:
  aero-codex-core
  aero-codex-units
  aero-codex-constants
  aero-codex-numerics
  aero-codex-validation
  aero-codex-data

domain starters:
  aero-codex-atmosphere
  aero-codex-thermo
  aero-codex-gasdynamics
  aero-codex-aerodynamics
  aero-codex-propulsion
  aero-codex-heat-transfer
  aero-codex-structures
  aero-codex-flight-dynamics
  aero-codex-astrodynamics
  aero-codex-bioregen

agentic scaffold:
  aero-codex-agent-core
  aero-codex-agent-schema
  aero-codex-agent-registry
  aero-codex-agent-index
  aero-codex-agent-retrieval
  aero-codex-agent-tools
  aero-codex-agent-trace
  aero-codex-agent-server
  aero-codex-agent-cli
  aero-codex-macros
```

## Public API rule

Stable public functions should prefer typed inputs and return `AeroResult<EngineeringResult<T>>` or a structured equivalent.

```rust
fn calculation(input: TypedInput) -> AeroResult<EngineeringResult<TypedOutput>>
```

The result should expose assumptions, warnings, validity status, verification record, and uncertainty when available.

## Evidence model

Each stable calculation must have a Codex ID and evidence card. Phase 0.001 starter equations can remain experimental while source review and reference validation are pending.

## Agentic model

Agents may discover, explain, route, and summarize. Rust owns calculations, units, branch validation, evidence status, errors, traces, and replay.
