# Milestones

## M0 — Deployable Founder Repository

Status target: Phase 0.001.

Deliverables:

- deployable repository zip;
- dual license files;
- CI and dependency policy;
- evidence-card schema;
- starter workspace;
- agentic scaffold integrated;
- founder docs and roadmap.

Exit gate: repository deploys cleanly and the deployment agent opens a report.

## M1 — Phase 0.001 Starter Equations

Deliverables:

- at least one Rust calculation in atmosphere, thermo, gas dynamics, aerodynamics, propulsion, heat transfer, structures, flight dynamics, astrodynamics, and bioregen;
- tests for each starter calculation;
- Codex IDs for each calculation;
- evidence cards or source-verification backlog records.

Exit gate: `cargo test --workspace --all-features` passes locally.

## M2 — Source Registry and Review Workflow

Deliverables:

- source registry for NACA/NASA/NOAA/NIST/JPL/NAIF and other authoritative references;
- source-review checklist;
- reference-data licensing notes;
- evidence-card source status field.

Exit gate: every starter Codex Entry is tied to a source-review task.

## M3 — Multi-Domain Validation Harness

Deliverables:

- validation runner loads cards;
- generated release report;
- reference-table fixture structure;
- property/limit/inverse test taxonomy.

Exit gate: at least three domains produce validation report sections.

## M4 — Agentic Trust Loop

Deliverables:

- generated registry;
- JSON schema export;
- tool invocation for one calculation;
- trace write/read/replay;
- retrieval fixture and CVT fixture.

Exit gate: one calculation can be selected and replayed by Rust without LLM math.

## M5 — v0.01 Research Preview

Deliverables:

- example notebooks/scripts avoided unless they call Rust only;
- CLI examples;
- docs for categories;
- clear caveats.

Exit gate: a private user can reproduce all examples from clean checkout.

## M6 — v0.1 Engineering Alpha

Deliverables:

- expanded equations in each major category;
- source-reviewed evidence cards;
- numerics hardening;
- initial API consistency review.

Exit gate: no major architecture rewrites needed for v1.0 plan.

## M7 — v0.5 Beta Candidate

Deliverables:

- stable candidate modules identified;
- deprecation policy active;
- cross-domain examples;
- data provenance reviewed.

Exit gate: v1.0 stable surface is proposed.

## M8 — v0.9 Release Candidate

Deliverables:

- API freeze candidate;
- release evidence report;
- docs complete for stable entries;
- changelog and migration notes.

Exit gate: no known blocking defects for v1.0.

## M9 — v1.0 First Stable Verified Core

Deliverables:

- bounded stable API;
- release report;
- no C0 stable entries;
- dependency and license audit;
- explicit non-certification statement.

Exit gate: tagged v1.0.0 release.

## M10+ — Beyond v1.0

Continue through post-1.0 domains: advanced gas dynamics, real-gas thermo, propulsion cycles, orbit propagation, structures, controls, bioregenerative loops, agentic tool server, UQ, and high-fidelity adapters where pure Rust or data-only policies permit.
