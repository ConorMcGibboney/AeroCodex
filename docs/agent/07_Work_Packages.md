# Agentic Rust Work Packages

## WP-A01 — Agent core types

Crate: `aero-codex-agent-core`

Implement IDs, tool specs, errors, retrieval structs, CVT structs, safety metadata, and trace summary types.

Exit criteria:

- Types serialize and deserialize.
- JSON snapshots are tested.
- No domain crate dependency.
- No hidden native dependency.

## WP-A02 — Evidence card agent extension

Crate: `aero-codex-validation`

Add `agent:` metadata to evidence cards and validate it.

Exit criteria:

- `exposed = true` requires `tool_name`.
- Every callable tool has input and output schema references.
- Aliases are linted for collisions.

## WP-A03 — Tool schema generator

Crate: `aero-codex-agent-schema`

Generate JSON schemas from Rust input/output/error types.

Exit criteria:

- Normal shock pressure-ratio input type generates schema.
- Example JSON validates.
- Unknown fields are rejected.

## WP-A04 — Registration macros

Crate: `aero-codex-macros`

Implement `#[codex_tool]`, `#[codex_entry]`, `#[codex_dataset]`, `#[codex_workflow]` macro interfaces.

Exit criteria:

- One gas dynamics function is registered without duplicated metadata.

## WP-A05 — Registry builder

Crate: `aero-codex-agent-registry`

Collect tool specs, evidence metadata, schema refs, and CVT refs into a deterministic registry.

Exit criteria:

- Build fails if exposed tool lacks evidence, schema, or Codex ID.

## WP-A06 — CVT generator

Crate: `aero-codex-agent-index`

Generate Companion Vector Tokens from evidence cards, rustdoc summaries, theory notes, datasets, errors, and workflow definitions.

Exit criteria:

- Every exposed Codex Entry generates at least one CVT.

## WP-A07 — Hybrid search

Crate: `aero-codex-agent-retrieval`

Implement exact ID search, symbol search, alias search, metadata filtering, optional vector search, and graph expansion.

Exit criteria:

- Query `pressure rise across normal shock` retrieves the normal shock pressure-ratio tool with reasons.

## WP-A08 — Capability graph

Crate: `aero-codex-agent-registry` or `aero-codex-agent-index`

Implement graph nodes, edges, workflow definitions, and simple planning expansion.

Exit criteria:

- Atmosphere output can be linked to flow-state construction and then shock evaluation.

## WP-A09 — Tool invocation engine

Crate: `aero-codex-agent-tools`

Accept JSON, validate schemas, convert into typed Rust domain inputs, call verified functions, return structured output or instructional errors.

Exit criteria:

- One tool can be invoked safely from JSON.

## WP-A10 — Explain mode

Crate: `aero-codex-agent-tools` or `aero-codex-agent-registry`

Build explanations from evidence cards, theory notes, Rustdoc, and validation metadata.

Exit criteria:

- Agent can request assumptions, equations, sources, validity, limitations, and examples.

## WP-A11 — Verify mode

Crate: `aero-codex-agent-tools` or `aero-codex-validation`

Return evidence status, test summary, data provenance, release report status, and blocking issues.

Exit criteria:

- Agent can determine whether a Codex Entry is implementation-verified, reference-validated, deprecated, or blocked.

## WP-A12 — Trace system

Crate: `aero-codex-agent-trace`

Write, read, replay, diff, and summarize traces.

Exit criteria:

- Every agent invocation can write a trace and replay it without an LLM.

## WP-A13 — Local tool server

Crate: `aero-codex-agent-server`

Expose list, schema, search, explain, verify, invoke, and trace endpoints through a local structured interface.

Exit criteria:

- Local process can list tools, search registry, invoke tool, and write trace.

## WP-A14 — CLI

Crate: `aero-codex-agent-cli` or subcommands in `aero-codex-cli`

Commands:

```text
aero-codex agent build-index
aero-codex agent list-tools
aero-codex agent schema <tool-name>
aero-codex agent search "<query>"
aero-codex agent explain <codex-id>
aero-codex agent verify <codex-id>
aero-codex agent invoke <tool-name> --json input.json
aero-codex agent trace replay trace.json
aero-codex agent graph export
aero-codex agent doctor
```

## WP-A15 — CI and release gates

Add CI checks for index generation, schema validation, retrieval tests, trace replay, server tests, and forbidden dependencies.
