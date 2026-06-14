# Agent Registry and Retrieval Plan

## Registry goal

The agent registry is a machine-readable index of agent-visible AeroCodex capabilities. It exists so agents can discover the correct Rust tool before attempting a calculation.

## Registry record

```rust
pub struct AgentRegistryEntry {
    pub codex_id: CodexId,
    pub tool_name: Option<AgentToolName>,
    pub title: String,
    pub domain: Domain,
    pub crate_name: CrateName,
    pub version: SemVer,
    pub entry_kind: CodexEntryKind,
    pub evidence_status: EvidenceStatus,
    pub callable: bool,
    pub explainable: bool,
    pub verifiable: bool,
    pub input_schema_ref: Option<PathBuf>,
    pub output_schema_ref: Option<PathBuf>,
    pub cvt_refs: Vec<CvtId>,
    pub capability_node: CapabilityNodeId,
    pub deprecated: bool,
    pub replacement: Option<CodexId>,
}
```

## Generated files

```text
validation/agent-index/
  codex_agent_registry.json
  codex_agent_registry.jsonl
  codex_tools.json
  codex_companion_vector_tokens.jsonl
  codex_capability_graph.json
  codex_aliases.json
  codex_symbols.json
  codex_errors.json
  codex_workflows.json
  manifest.json
```

## Retrieval pipeline

```text
User or agent query
        |
        v
Normalize query
        |
        v
Exact Codex ID match
        |
        v
Symbol and alias search
        |
        v
Metadata filters
        |
        v
Vector similarity over CVTs
        |
        v
Capability graph expansion
        |
        v
Validity and evidence filtering
        |
        v
Ranked candidates with reasons
```

## Retrieval request

```rust
pub struct RetrievalRequest {
    pub query: String,
    pub domains: Vec<Domain>,
    pub evidence_minimum: Option<EvidenceStatus>,
    pub callable_only: bool,
    pub required_inputs: Vec<String>,
    pub desired_outputs: Vec<String>,
    pub mode: AgentUseMode,
    pub max_results: usize,
}
```

## Retrieval result

```rust
pub struct RetrievalResult {
    pub codex_id: CodexId,
    pub tool_name: Option<AgentToolName>,
    pub title: String,
    pub score: f64,
    pub reasons: Vec<RetrievalReason>,
    pub evidence_status: EvidenceStatus,
    pub validity_summary: String,
    pub warnings: Vec<String>,
}
```

## Retrieval reasons

```rust
pub enum RetrievalReason {
    ExactCodexIdMatch,
    SymbolMatch { symbol: String },
    AliasMatch { alias: String },
    SemanticMatch { similarity: f64 },
    DomainMatch { domain: Domain },
    WorkflowDependency,
    EvidenceLevelSatisfied,
    OutputQuantityMatch { quantity: String },
}
```

## Capability graph

```rust
pub struct CapabilityGraph {
    pub nodes: Vec<CapabilityNode>,
    pub edges: Vec<CapabilityEdge>,
}
```

Edge kinds:

```rust
pub enum CapabilityEdgeKind {
    RequiresOutputFrom,
    Refines,
    AlternativeTo,
    InverseOf,
    ValidatesAgainst,
    UsesDataset,
    UsesSolver,
    ProducesInputFor,
    SamePhysicalModelAs,
}
```

Example workflow:

```text
atmosphere.standard_1976.at_altitude
        |
        v
gasdyn.flow_state.from_mach_static_conditions
        |
        v
gasdyn.normal_shock.evaluate
        |
        v
propulsion.nozzle.off_design_report
```
