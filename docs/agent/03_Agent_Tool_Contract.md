# Agent Tool Contract

## Purpose

An agent tool is a structured interface between an external planner and an AeroCodex Rust calculation. It must expose a strict schema, validate input, convert JSON into typed Rust values, call the verified implementation, and return structured output or an instructional error.

## Tool spec

```rust
pub struct AgentToolSpec {
    pub name: AgentToolName,
    pub codex_id: CodexId,
    pub crate_name: CrateName,
    pub version: SemVer,
    pub title: String,
    pub description: String,
    pub mode_support: AgentModeSupport,
    pub input_schema: JsonSchemaDocument,
    pub output_schema: JsonSchemaDocument,
    pub error_schema: JsonSchemaDocument,
    pub assumptions: Vec<Assumption>,
    pub limitations: Vec<Limitation>,
    pub validity: ValidityDomain,
    pub branch_rules: Vec<BranchRule>,
    pub evidence: EvidenceSummary,
    pub examples: Vec<AgentToolExample>,
    pub safety: AgentToolSafety,
}
```

## Tool runtime trait

```rust
pub trait AgentTool {
    fn spec(&self) -> &'static AgentToolSpec;

    fn invoke_json(
        &self,
        input: serde_json::Value,
        context: &AgentInvocationContext,
    ) -> Result<serde_json::Value, AgentError>;
}
```

## Invocation context

```rust
pub struct AgentInvocationContext {
    pub mode: AgentUseMode,
    pub trace_policy: TracePolicy,
    pub evidence_minimum: Option<EvidenceStatus>,
    pub extrapolation_policy: ExtrapolationPolicy,
    pub units_policy: UnitsPolicy,
    pub warnings_policy: WarningsPolicy,
}
```

## Input policy

Agent-facing tools shall use structured objects, not positional `f64` arguments.

```rust
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct NormalShockPressureRatioInput {
    pub mach1: MachInput,
    pub gamma: RatioOfSpecificHeatsInput,
}
```

Unknown fields are rejected by default.

## Output policy

```rust
pub struct AgentEngineeringOutput<T> {
    pub value: T,
    pub quantity: String,
    pub unit: String,
    pub validity: ValidityStatus,
    pub assumptions: Vec<String>,
    pub warnings: Vec<AgentWarning>,
    pub evidence: EvidenceSummary,
    pub uncertainty: Option<UncertaintySummary>,
    pub trace: Option<TraceSummary>,
}
```

## Error policy

Errors must be recoverable when possible.

```rust
pub struct AgentError {
    pub code: AgentErrorCode,
    pub message: String,
    pub codex_id: Option<CodexId>,
    pub tool_name: Option<AgentToolName>,
    pub parameter: Option<String>,
    pub received_value: Option<serde_json::Value>,
    pub expected_condition: Option<String>,
    pub recoverability: Recoverability,
    pub suggested_tools: Vec<AgentToolName>,
    pub suggested_questions: Vec<String>,
    pub evidence: Option<EvidenceSummary>,
}
```

Example:

```json
{
  "code": "RequiresSupersonic",
  "message": "Normal shock relations require upstream Mach number greater than 1.",
  "codex_id": "gasdyn.normal_shock.p2_over_p1",
  "tool_name": "gasdyn.normal_shock.p2_over_p1",
  "parameter": "mach1",
  "received_value": 0.82,
  "expected_condition": "mach1 > 1",
  "recoverability": "correct_input_or_select_different_model",
  "suggested_tools": ["gasdyn.isentropic.static_to_total_pressure_ratio"],
  "suggested_questions": ["Is the upstream flow actually supersonic?", "Do you want an isentropic subsonic relation instead?"]
}
```

## Macro direction

```rust
#[codex_tool(
    id = "gasdyn.normal_shock.p2_over_p1",
    name = "gasdyn.normal_shock.p2_over_p1",
    evidence = "validation/evidence-cards/gasdynamics/normal_shock_p2_over_p1.yaml",
    modes = ["discover", "explain", "compute", "verify"]
)]
pub fn normal_shock_pressure_ratio(
    input: NormalShockPressureRatioInput,
) -> Result<EngineeringResult<PressureRatio>, AeroError> {
    todo!()
}
```

The macro should generate registry hooks, schema refs, evidence links, and documentation badges.
