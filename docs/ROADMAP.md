# AeroCodex Roadmap

AeroCodex remains at **Project Phase 0.001** until it has basic Rust equations, evidence cards, tests, and source-verification plans across the founder aerospace categories, bio-regenerative life support systems, and celestial mechanics / astrodynamics.

The Cargo package version for this phase is `0.0.1`; the project phase label is `0.001`.

## Phase 0.001 — Research and Starter Equation Baseline

Purpose: establish the repository, policies, evidence structure, agentic scaffold, and a first set of Rust equations across multiple domains.

Required domains before leaving Phase 0.001:

```text
core / units / constants / numerics / validation
aerodynamics
atmosphere / environment
thermodynamics
gas dynamics
propulsion
heat transfer
structures / loads
flight dynamics
celestial mechanics / astrodynamics
bio-regenerative life support systems
agentic discovery / schema / trace scaffold
```

Exit gate: each required domain has at least one Rust calculation, one test, one Codex ID, and one evidence card or explicit evidence-card backlog record.

## Phase 0.002 — Source-Verification Alpha

Purpose: turn starter calculations into sourced, reviewed Codex Entries.

Exit gate: every Phase 0.001 equation has a primary source record, source-review status, test strategy, and known-validity note.

## Phase 0.003 — Multi-Domain Trust Loop Alpha

Purpose: prove the trust loop across several domains, not just gas dynamics.

Exit gate: at least one calculation from atmosphere, gas dynamics, aerodynamics, propulsion, astrodynamics, and bioregen can be discovered, explained, tested, and reported.

## Phase 0.004 — Numerics and Units Hardening

Purpose: harden root solvers, interpolation, quantity types, angles, frames, branches, error taxonomy, and unit conversions.

Exit gate: no public API accepts ambiguous units/branches where typed alternatives exist.

## Phase 0.005 — Agentic Vertical Slice

Purpose: one agent-visible Codex Entry can be discovered, schema-inspected, invoked, traced, verified, and replayed without an LLM performing the calculation.

Exit gate: agent trace replay works for at least one tool.

## Phase 0.01 — Research Preview

Purpose: a usable private preview with cross-domain examples and generated verification reports.

Exit gate: validation report generated from repository evidence, tests pass in CI, dependency policy passes.

## Phase 0.1 — Engineering Alpha

Purpose: broaden first-principles and textbook aerospace coverage with source-reviewed APIs.

Exit gate: major categories have stable internal APIs and non-stable public API markings.

## Phase 0.5 — Pre-Beta

Purpose: multi-fidelity workflows, examples, source audit, and major refactors complete.

Exit gate: no known architecture blockers to v1.0.

## Phase 0.9 — Release Candidate Series

Purpose: freeze APIs targeted for v1.0 and complete release evidence.

Exit gate: v1.0 release checklist can be satisfied with only bug fixes and documentation polish.

## Version 1.0 — First Stable Verified Core

Purpose: stable public API for a carefully bounded set of source-traceable aerospace math.

Minimum v1.0 scope is intentionally narrower than the long-range vision. Stable means stable within declared assumptions and validity ranges, not certified for regulated use.

## Beyond 1.0

Post-1.0 expands breadth and fidelity: real-gas thermo, equilibrium chemistry, upper-atmosphere models, advanced gas dynamics, low-order aero, propulsion cycle analysis, astrodynamics propagation, structures, controls, bioregenerative life support modeling, agentic workflows, WASM, Python bindings calling Rust, and validated data pipelines.
