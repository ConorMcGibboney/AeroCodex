# AeroCodex Agentic Addendum — Executive Summary

## Purpose

This addendum defines the Rust implementation layer required to make AeroCodex safe and useful for agentic workflows. The agent layer does not replace the verified aerospace mathematics layer. It exposes that layer to automated systems through typed schemas, evidence metadata, retrieval records, structured errors, and replayable traces.

## Agentic north star

AeroCodex should not ask an agent to calculate aerospace engineering math. AeroCodex should make an agent ask Rust for verified math.

The system must answer these machine-facing questions:

- What calculation should be used?
- What are its assumptions and limitations?
- What inputs are required?
- Which units, frames, dimensionless types, and branches are valid?
- What evidence supports the result?
- Can the calculation be invoked with structured input?
- Can the result be traced and replayed?
- Can invalid input be rejected with recoverable guidance?

## Primary deliverables

1. Agent registry of Codex Entries.
2. JSON schema generation from Rust types.
3. Companion Vector Tokens generated from evidence metadata.
4. Hybrid retrieval over symbols, aliases, metadata, graph relationships, and optional vectors.
5. Agent tool invocation with typed conversion and instructional errors.
6. Explain and verify modes.
7. Trace and replay system.
8. Local-only tool server and CLI.
9. CI gates proving no tool is exposed without evidence, schema, and trace tests.

## Implementation doctrine

The LLM may plan, route, and narrate. Rust owns all authoritative operations:

```text
Rust types
Rust validation
Rust schemas
Rust errors
Rust traces
Rust registry
Rust retrieval
Rust tool invocation
Rust replay
```

Companion Vector Tokens are retrieval sidecars, not the source of truth. Evidence cards, Rust code, reference data, tests, and validation reports remain the source of truth.
