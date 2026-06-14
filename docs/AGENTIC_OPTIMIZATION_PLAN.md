# Agentic Optimization Plan

## Purpose

AeroCodex should eventually support agent-assisted design-space exploration and optimization, but Rust remains the authority for calculations, constraints, units, evidence, and traces.

## Doctrine

```text
The agent may propose.
Rust must dispose.
```

Agents may:

- search the Codex registry;
- select candidate models;
- assemble workflows;
- explain assumptions;
- propose parameter sweeps;
- summarize results.

Agents must not:

- invent aerospace math;
- bypass typed Rust calculations;
- silently extrapolate;
- modify evidence status;
- claim certification or mission suitability;
- execute arbitrary code through the tool server.

## Optimization roadmap

### AO-0 — Planning only

Current Phase 0.001 status. Agentic crates define types, traces, registry, schemas, retrieval, and local tool-serving scaffolds.

### AO-1 — Deterministic search and invocation

One tool can be discovered, schema-inspected, invoked, and traced.

### AO-2 — Multi-tool workflow graph

The agent can assemble typed workflows, such as atmosphere -> speed of sound -> Mach -> dynamic pressure -> load.

### AO-3 — Parameter sweeps

Rust executes bounded sweeps with explicit constraints and returns structured tables/reports.

### AO-4 — Native optimization

Rust optimization crates or internal algorithms perform constrained optimization with full trace records.

### AO-5 — Design-space exploration with evidence filters

The agent may search design alternatives but every model must meet a declared minimum evidence level.

### AO-6 — Replayable studies

Every agent-initiated study can be replayed without the agent from trace records and input manifests.

## Safety gates

- read-only local server by default;
- explicit input schemas;
- no arbitrary code execution;
- no hidden network calls;
- trace required for compute mode once mature;
- evidence minimum respected;
- extrapolation policy explicit;
- all warnings returned in machine-readable form.
