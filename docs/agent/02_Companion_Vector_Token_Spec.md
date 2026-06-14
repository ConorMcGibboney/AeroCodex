# Companion Vector Token Specification

## Definition

A Companion Vector Token, or CVT, is a structured retrieval sidecar attached to a Codex Entry, model, solver, evidence card, dataset, theory note, workflow, error explanation, branch rule, or validity rule.

It is not a tokenizer token. It is a machine-readable retrieval unit with optional embedding metadata.

## Source of truth rule

The CVT is never authoritative. The authoritative sources are:

1. Rust implementation.
2. Evidence card.
3. Validation tests.
4. Reference data.
5. Release report.
6. Theory note.

## Rust type

```rust
pub struct CompanionVectorToken {
    pub token_id: CvtId,
    pub codex_id: CodexId,
    pub version: SemVer,
    pub kind: CvtKind,
    pub domain: Domain,
    pub crate_name: CrateName,
    pub title: String,
    pub summary: String,
    pub retrieval_text: String,
    pub aliases: Vec<String>,
    pub symbols: Vec<String>,
    pub equations: Vec<EquationRef>,
    pub source_refs: Vec<SourceRef>,
    pub inputs: Vec<AgentFieldRef>,
    pub outputs: Vec<AgentFieldRef>,
    pub assumptions: Vec<Assumption>,
    pub limitations: Vec<Limitation>,
    pub validity: ValidityDomain,
    pub forbidden_when: Vec<ForbiddenCondition>,
    pub evidence_status: EvidenceStatus,
    pub validation_status: ValidationStatus,
    pub callable: bool,
    pub tool_name: Option<AgentToolName>,
    pub explainable: bool,
    pub verifiable: bool,
    pub embedding: Option<EmbeddingRef>,
    pub provenance: CvtProvenance,
}
```

## CVT kinds

```rust
pub enum CvtKind {
    Calculation,
    Model,
    Solver,
    Dataset,
    TheoryNote,
    Workflow,
    ErrorExplanation,
    BranchRule,
    ValidityRule,
    UnitDefinition,
    FrameDefinition,
}
```

## Required fields

Every CVT must have:

- `token_id`
- `codex_id`
- `kind`
- `title`
- `retrieval_text`
- `evidence_status`
- `provenance`

Callable CVTs must also have:

- `tool_name`
- `inputs`
- `outputs`
- `validity`
- `forbidden_when`

## Example

```json
{
  "token_id": "cvt:gasdyn.normal_shock.p2_over_p1:v0.4.0",
  "codex_id": "gasdyn.normal_shock.p2_over_p1",
  "version": "0.4.0",
  "kind": "calculation",
  "domain": "gas_dynamics",
  "crate_name": "aero-codex-gasdynamics",
  "title": "Normal shock static pressure ratio",
  "summary": "Computes downstream-to-upstream static pressure ratio across a normal shock.",
  "retrieval_text": "Use this when the user asks for pressure rise, pressure jump, downstream static pressure ratio, or p2/p1 across a normal shock in a perfect gas.",
  "aliases": ["normal shock pressure ratio", "pressure rise across shock", "shock pressure jump", "p2 over p1"],
  "symbols": ["p2/p1", "p_2/p_1"],
  "inputs": [{"name": "mach1", "type": "Mach", "required": true}, {"name": "gamma", "type": "RatioOfSpecificHeats", "required": true}],
  "outputs": [{"name": "pressure_ratio", "type": "PressureRatio"}],
  "assumptions": ["steady flow", "inviscid discontinuity", "one-dimensional control volume", "calorically perfect gas"],
  "limitations": ["no real-gas correction", "no viscous shock structure", "no reacting flow"],
  "validity": {"mach1": {"exclusive_min": 1.0}, "gamma": {"exclusive_min": 1.0}},
  "forbidden_when": ["mach1 <= 1", "real gas effects dominate", "reacting flow required"],
  "evidence_status": "C3_reference_validated",
  "callable": true,
  "tool_name": "gasdyn.normal_shock.p2_over_p1",
  "explainable": true,
  "verifiable": true
}
```

## Embedding policy

Embedding generation is optional. Rust shall support precomputed embedding storage and similarity search, but the verified math crates must not depend on any remote embedding provider.

Allowed modes:

```rust
pub enum EmbeddingProviderMode {
    None,
    Precomputed,
    LocalRustModel,
    ExternalBuildTimeProvider,
}
```

When embeddings exist, the CVT must record:

- model ID;
- vector dimensions;
- normalized flag;
- vector file path;
- SHA-256 hash;
- generation timestamp.
