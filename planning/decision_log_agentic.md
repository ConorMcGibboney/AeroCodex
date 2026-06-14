# Agentic Decision Log

## ADR-A001 — CVTs are retrieval sidecars

Status: proposed

Decision: Companion Vector Tokens are not source-of-truth records. They are generated from evidence cards, metadata, docs, and validated registry records.

Reason: Retrieval text and embeddings can drift. The authoritative engineering record must remain the Rust implementation, evidence card, tests, and release report.

## ADR-A002 — Rust owns all calculations

Status: proposed

Decision: The agent layer must never delegate engineering math to an LLM. LLMs may plan, summarize, and select tools, but tool execution is typed Rust.

## ADR-A003 — Local server first

Status: proposed

Decision: The first agent tool server shall bind to localhost by default and expose only registered tools. No arbitrary code execution and no hidden network access.

## ADR-A004 — Unknown input fields denied

Status: proposed

Decision: Agent-facing JSON input structs should use `#[serde(deny_unknown_fields)]` by default.

## ADR-A005 — Trace replay must not require an LLM

Status: proposed

Decision: Traces must be standalone structured records that can be replayed by Rust tooling.
