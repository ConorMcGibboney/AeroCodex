# Versioning Plan

## Principle

AeroCodex uses two version labels during early development:

```text
Project phase label: 0.001
Cargo SemVer:        0.0.1
```

The project phase label is the founder planning label. The Cargo version is the SemVer-compatible Rust package version.

The project shall remain at **0.001** until basic Rust equations exist in the agreed domains:

1. foundation/core categories discussed so far;
2. bio-regenerative life support systems;
3. celestial mechanics / astrodynamics.

## Why not put `0.001` directly in every Cargo package?

Cargo packages should use SemVer-compatible version strings. The project therefore records the human phase as metadata while using `0.0.1` in `Cargo.toml`.

```toml
[workspace.package]
version = "0.0.1"

[workspace.metadata.aerocodex]
project_phase_version = "0.001"
```

## Evidence levels

```text
C0 Proposed
C1 Equation traceable
C2 Implementation verified
C3 Reference validated
C4 Experiment validated
Cx Deprecated or withdrawn
```

Early starter equations may be C2 for implementation tests but still blocked from stable release if the source card or reference dataset is incomplete.

## Release bands

| Band | Meaning | Public promise |
|---|---|---|
| 0.001 | Founder research baseline | Planning, repository, starter equations, no engineering claim |
| 0.002-0.009 | Source review and trust-loop formation | Increasing traceability, still unstable |
| 0.01-0.09 | Research previews | Useful examples, unstable APIs |
| 0.1-0.4 | Engineering alpha | Broader source-reviewed implementation |
| 0.5-0.8 | Pre-beta and beta hardening | API shape stabilizes |
| 0.9.x | v1.0 release candidates | API freeze candidates |
| 1.0.0 | First stable verified core | Stable bounded API with release evidence |
| 1.x | Stable expansion | Additive capabilities under stable guarantees |
| 2.x+ | Major architecture changes | Breaking changes only when justified |

## Version advance rules

### 0.001 -> 0.002

Required:

- all required Phase 0.001 crates compile locally;
- each starter equation has a Codex ID;
- each starter equation has at least one unit test;
- each starter equation has an evidence card or explicit source-verification backlog item;
- deployment agent reports all commands run and any failures.

### 0.002 -> 0.003

Required:

- source registry seeded with authoritative source candidates;
- source-review checklist exists;
- evidence cards carry source review state;
- no equation is marked stable;
- reference-data licensing notes exist.

### 0.003 -> 0.01

Required:

- multi-domain validation report generated;
- at least one full trust-loop example outside gas dynamics;
- agentic index scaffold passes;
- CI gates pass in clean environment.

### 0.01 -> 0.1

Required:

- public docs describe assumptions and caveats;
- first reference-table reproduction exists;
- numerical solver failure modes are tested;
- dependency policy is automated.

### 0.9 -> 1.0

Required:

- stable API review complete;
- release evidence report generated;
- no C0 entries in stable API;
- all stable entries are at least C2 and selected flagship entries are C3 or better;
- certification caveat appears in README and docs;
- no wrapper or native dependency violations;
- governance and contribution review active;
- changelog and migration notes complete.

## Post-1.0 version themes

| Version family | Theme |
|---|---|
| 1.1 | More gas dynamics and atmosphere reference validation |
| 1.2 | Aerodynamics low-order methods |
| 1.3 | Propulsion ideal cycles and rocket-nozzle workflows |
| 1.4 | Astrodynamics propagation and frame utilities |
| 1.5 | Structures/loads analytical methods |
| 1.6 | Bio-regenerative life support mass/energy/nutrient loops |
| 1.7 | Agentic trace/replay and local tool server maturity |
| 1.8 | WASM/documentation workbench |
| 1.9 | Uncertainty propagation and sensitivity analysis |
| 2.0 | Major API cleanup after real-world use |

## Non-negotiable caveat

No version number means AeroCodex is certified, flight-ready, life-support-ready, mission-ready, or regulator-approved. AeroCodex can only report the verification and validation evidence that exists for a specific model, range, and implementation.
