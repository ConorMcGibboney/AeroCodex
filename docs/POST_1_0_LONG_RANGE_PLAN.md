# Post-1.0 Long-Range Plan

This file intentionally preserves the parts of the founder vision that are beyond v1.0.

## 1.x expansion themes

```text
1.1 Reference validation expansion
1.2 Low-order aerodynamics
1.3 Propulsion cycles and rocket workflows
1.4 Astrodynamics propagation and frames
1.5 Structures, loads, and mass properties
1.6 Bio-regenerative life support loops
1.7 Agentic registry, trace, and local server maturity
1.8 WASM and documentation workbench
1.9 Uncertainty, sensitivity, and design sweeps
```

## 2.x themes

```text
real-gas thermodynamics
equilibrium chemistry in pure Rust
higher-fidelity atmosphere and space environment models
advanced gas dynamics and inlet/nozzle design
multi-body astrodynamics
orbit determination and covariance
low-order CFD-like methods where appropriate
composites and aeroelasticity hooks
control/GNC workflows
robust native optimization
```

## 3.x+ themes

```text
integrated aircraft/spacecraft preliminary design
habitat and closed-loop life-support system studies
mission-level workflow graphs
validated surrogate model framework
formal methods for selected numerical kernels
multi-language bindings that call Rust only
long-term ecosystem governance
```

## Permanent constraints

The 100% Rust verified core remains non-negotiable unless the project explicitly creates a separate, non-core adapter layer. External legacy tools may be used as comparison targets, never hidden sources of calculation authority in the core.
