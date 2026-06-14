# AeroCodex Agentic Rust Implementation Plan

## 1. Objective

Build an optional but first-class Rust-coded agent layer that lets AI agents and other automated systems discover, understand, invoke, verify, and audit AeroCodex calculations without bypassing typed units, branch rules, validity domains, evidence cards, or validation reports.

## 2. New crates

```text
crates/
  aero-codex-agent-core/
  aero-codex-agent-schema/
  aero-codex-agent-registry/
  aero-codex-agent-index/
  aero-codex-agent-retrieval/
  aero-codex-agent-tools/
  aero-codex-agent-trace/
  aero-codex-agent-server/
  aero-codex-agent-cli/
  aero-codex-macros/
```

## 3. Layering

```text
Agent CLI / Local Tool Server / WASM Agent Surface
        |
Agent Tools / Tool Invocation / Trace / Explain / Verify
        |
Agent Registry / Schemas / Companion Vector Tokens / Capability Graph
        |
Validation / Evidence Cards / Data / Numerics
        |
Core / Units / Constants / Errors / Frames / EngineeringResult
        |
Domain Crates: Atmosphere, Gas Dynamics, Thermo, Aero, Propulsion, etc.
```

Rules:

1. The agent layer may depend on the verified math layer.
2. The verified math layer shall not depend on the agent layer.
3. Embedding generation is optional and isolated behind a provider trait.
4. The core product must remain usable without an LLM or network calls.

## 4. Agent requirements

### AGT-DISC — discovery

```text
AGT-DISC-001 Every stable public Codex Entry shall be discoverable through a Rust-generated agent registry.
AGT-DISC-002 Every agent-visible entry shall expose a Codex ID, title, domain, crate, capability type, evidence status, schemas, assumptions, limitations, validity rules, and failure modes.
AGT-DISC-003 Every agent-visible entry shall provide aliases and retrieval text.
AGT-DISC-004 Every agent-visible entry shall declare whether it is callable, explainable, verifiable, deprecated, experimental, or hidden.
AGT-DISC-005 No agent-visible tool shall exist without a corresponding evidence card.
```

### AGT-SAFE — invocation safety

```text
AGT-SAFE-001 Agent tools shall accept structured inputs only.
AGT-SAFE-002 Agent tools shall return EngineeringResult metadata or a structured equivalent.
AGT-SAFE-003 Agent tools shall never silently select a physical or mathematical branch unless a documented report-level default explicitly allows it.
AGT-SAFE-004 Agent tools shall return instructional errors with violated conditions and recovery suggestions where possible.
AGT-SAFE-005 Agent tools shall reject unknown fields by default.
```

### AGT-RET — retrieval

```text
AGT-RET-001 The system shall generate Companion Vector Tokens from evidence cards, rustdoc metadata, theory notes, and registered Codex Entry metadata.
AGT-RET-002 Companion Vector Tokens shall not be the source of truth.
AGT-RET-003 Retrieval shall support lexical search, metadata filtering, graph expansion, and vector similarity.
AGT-RET-004 Retrieval shall return reasons for ranked results.
AGT-RET-005 Embedding generation shall be behind a provider interface.
```

### AGT-TRACE — audit

```text
AGT-TRACE-001 Every agent tool invocation shall be serializable as a replayable trace.
AGT-TRACE-002 A trace shall record tool name, Codex ID, input payload, normalized typed values, output payload, warnings, errors, evidence status, crate version, data checksums, and timestamp.
AGT-TRACE-003 Trace replay shall recompute prior calls and report whether outputs match within documented tolerances.
AGT-TRACE-004 Trace replay shall not require an LLM.
```

## 5. Agent modes

```rust
pub enum AgentUseMode {
    Discover,
    Explain,
    Compute,
    Verify,
    Replay,
}
```

## 6. Required generated artifacts

```text
validation/agent-index/
  manifest.json
  codex_agent_registry.json
  codex_agent_registry.jsonl
  codex_tools.json
  codex_companion_vector_tokens.jsonl
  codex_capability_graph.json
  codex_aliases.json
  codex_symbols.json
  codex_errors.json
  codex_workflows.json
  schemas/
    inputs/
    outputs/
    errors/
  embeddings/
```

## 7. First vertical slice

The first slice should implement `gasdyn.normal_shock.p2_over_p1` all the way through:

1. Evidence card with agent metadata.
2. Typed Rust function.
3. Structured input and output types.
4. Generated JSON schema.
5. Agent tool spec.
6. Companion Vector Token.
7. Registry entry.
8. Retrieval result.
9. JSON invocation.
10. Instructional `RequiresSupersonic` error.
11. Trace write.
12. Trace replay.
13. Explain response.
14. Verify response.

## 8. Release policy

An agent-layer release candidate must fail if any exposed tool lacks:

- Codex ID.
- Evidence card.
- Input schema.
- Output schema.
- Error schema.
- At least one Companion Vector Token.
- Invocation test.
- Trace replay test.
- Safety metadata.
