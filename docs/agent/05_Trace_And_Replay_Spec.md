# Trace and Replay Specification

## Purpose

Agentic engineering calculations must be auditable. A trace records exactly which tool was called, what input was supplied, how it was normalized into typed values, which evidence supported the call, and what output or error was returned.

A trace must be replayable without an LLM.

## Trace record

```rust
pub struct AgentTrace {
    pub trace_id: TraceId,
    pub created_at: Timestamp,
    pub aero_codex_version: SemVer,
    pub workspace_git_sha: Option<String>,
    pub invocation: AgentInvocationRecord,
    pub result: AgentResultRecord,
    pub evidence: EvidenceSummary,
    pub data_checksums: Vec<DatasetChecksum>,
    pub environment: TraceEnvironment,
}
```

## Invocation record

```rust
pub struct AgentInvocationRecord {
    pub tool_name: AgentToolName,
    pub codex_id: CodexId,
    pub raw_input_json: serde_json::Value,
    pub normalized_input_json: serde_json::Value,
    pub context: AgentInvocationContext,
}
```

## Result record

```rust
pub struct AgentResultRecord {
    pub success: bool,
    pub raw_output_json: Option<serde_json::Value>,
    pub error: Option<AgentError>,
    pub warnings: Vec<AgentWarning>,
    pub validity: Option<ValidityStatus>,
    pub duration_ms: u64,
}
```

## CLI commands

```text
aero-codex agent trace show <trace-file>
aero-codex agent trace replay <trace-file>
aero-codex agent trace diff <trace-a> <trace-b>
aero-codex agent trace summarize <trace-dir>
```

## Replay behavior

Replay should:

1. Load trace.
2. Locate tool by exact name and Codex ID.
3. Validate that the tool still exists.
4. Validate version compatibility.
5. Re-run with normalized input.
6. Compare output against original output using tool-defined tolerance.
7. Report identical, within tolerance, changed within documented tolerance, or failed.

## Trace privacy

Trace files should avoid unnecessary user text. They should include structured inputs and outputs, not raw conversation logs, unless explicitly configured for a private local audit mode.
