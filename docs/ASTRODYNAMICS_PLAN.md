# Celestial Mechanics / Astrodynamics Plan

## Intent

AeroCodex will include celestial mechanics and astrodynamics as a first-class category, beginning with analytically traceable two-body equations and expanding toward propagation, frames, time systems, perturbations, and mission-analysis primitives.

## Phase 0.001 starter scope

```text
astrodynamics.two_body.circular_orbit_speed
astrodynamics.two_body.vis_viva_speed
```

These establish the first two-body calculation API and typed `GravitationalParameter` handling.

## Near-term expansion

```text
specific orbital energy
orbital period
escape speed
semi-major axis from energy
state vector <-> classical orbital elements
Kepler equation solvers
Hohmann transfer delta-v
plane-change delta-v
J2 nodal precession
relative motion starter equations
ECI/ECEF and local orbital frame types
```

## Source and validation targets

Primary source targets:

- classical astrodynamics texts and open NASA training material;
- JPL Solar System Dynamics astrodynamic parameter tables;
- NAIF/SPICE documentation for frames and kernels, used as a reference target and not as a core dependency;
- NASA/NTRS orbital mechanics presentations and reports;
- published examples for Bate, Mueller, and White style two-body calculations.

## Rules

1. Core astrodynamics remains 100% Rust.
2. SPICE, GMAT, Orekit, or other external tools can be comparison targets, not hidden dependencies in the verified core.
3. Every frame transform must state frame, epoch, convention, and source.
4. Every propagated result must state force model assumptions.
5. Every gravitational parameter must state source, epoch/version, units, and uncertainty if available.

## Post-1.0 expansion

Post-1.0 can add Lambert solvers, patched conics, interplanetary transfers, higher-order gravity, third-body perturbations, drag, solar radiation pressure, event detection, orbit determination, covariance, and agent-assisted mission-analysis workflows.
