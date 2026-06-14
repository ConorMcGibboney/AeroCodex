# Agentic Crate Map

## aero-codex-agent-core

Defines shared agent-layer types with no domain dependencies.

Files:

- `ids.rs`
- `tool.rs`
- `error.rs`
- `cvt.rs`
- `retrieval.rs`
- `trace.rs`
- `safety.rs`

## aero-codex-agent-schema

Generates and validates JSON schemas for inputs, outputs, and errors.

## aero-codex-agent-registry

Builds a registry of Codex Entries, tool specs, schema refs, evidence refs, and capability graph nodes.

## aero-codex-agent-index

Generates CVTs, alias index, symbol index, error index, workflow index, and optional embedding refs.

## aero-codex-agent-retrieval

Searches registry and CVTs with lexical, symbol, alias, metadata, graph, and optional vector strategies.

## aero-codex-agent-tools

Invokes registered tools safely from JSON.

## aero-codex-agent-trace

Writes, reads, replays, diffs, and summarizes traces.

## aero-codex-agent-server

Provides a local server interface for agents.

## aero-codex-agent-cli

Provides developer and automation commands.

## aero-codex-macros

Provides registration macros that reduce duplicated metadata.
