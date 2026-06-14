# Category Admission Plan

AeroCodex admits a domain category only when it can express the category through typed Rust APIs, explicit assumptions, evidence cards, tests, and source-review tasks.

## Phase 0.001 categories

| Category | Starter purpose | First equations |
|---|---|---|
| Core | Evidence, errors, assumptions, warnings | `EngineeringResult`, `AeroError` |
| Units | Prevent unit/angle/frame mistakes | typed quantities |
| Constants | Store sourced constants with provenance | `g0`, `R_air`, Stefan-Boltzmann, Earth `mu` |
| Numerics | Reusable solver backbone | bisection root solver |
| Atmosphere | Environment starter | ideal-gas density |
| Thermo | Gas property starter | perfect-gas speed of sound |
| Gas dynamics | Compressible-flow starter | normal shock pressure ratio, isentropic pressure ratio |
| Aerodynamics | Flow-load starter | dynamic pressure |
| Propulsion | Rocket starter | ideal rocket delta-v |
| Heat transfer | Thermal starter | blackbody emissive power |
| Structures | Loads/stress starter | axial stress |
| Flight dynamics | Flight mechanics starter | coordinated turn radius |
| Astrodynamics | Celestial mechanics starter | circular orbit speed, vis-viva speed |
| Bio-regenerative life support | Life-support loop starter | closed-loop fraction, production area, buffer residence time |
| Agentic layer | Safe automation surface | registry/schema/retrieval/trace/tool scaffold |

## Category gates

Before a category can have stable public entries:

1. It has a category owner or reviewer.
2. It has a source map.
3. It has at least one evidence card.
4. It has tests and known limitations.
5. Its public API exposes typed quantities and explicit branches.
6. It has no hidden native/wrapper dependency.
7. It has no certification or mission-readiness claim.

## Deferred categories beyond 1.0

```text
real-gas and equilibrium thermo
advanced inlet/nozzle methods
method of characteristics
low-order aircraft design workflows
propulsion cycles and engine maps
upper-atmosphere and space environment models
orbit determination and estimation
relative motion and rendezvous
control systems and GNC
composite structures and aeroelastic hooks
bio-regenerative crop/culture models
waste processing and nutrient loops
closed habitat reliability and resilience models
uncertainty quantification and sensitivity analysis
agentic local server and WASM workbench
```
