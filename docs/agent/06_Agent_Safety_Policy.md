# Agent Safety Policy

## Purpose

The agent layer must reduce misuse by forcing evidence, validity, branch selection, unit clarity, and explicit warnings into every automated workflow.

## Safety classes

```rust
pub enum AgentMisuseRisk {
    Low,
    Medium,
    High,
    Restricted,
}
```

Examples:

- Low: unit conversion, standard atmosphere lookup within range.
- Medium: normal shock relation, isentropic flow relation, thin airfoil approximation.
- High: hypersonic heating estimates, empirical structural buckling correlations, preliminary flight dynamics models.
- Restricted: anything that can be misread as certified flight guidance or operational control guidance.

## Safety metadata

```rust
pub struct AgentToolSafety {
    pub misuse_risk: AgentMisuseRisk,
    pub safety_notes: Vec<String>,
    pub prohibited_claims: Vec<String>,
    pub requires_user_acknowledgement: bool,
}
```

## Policy rules

1. No agent-visible tool without a safety classification.
2. No high-risk model without explicit `should_not_use_when` metadata.
3. No branchy equation with hidden branch selection.
4. No empirical model with silent extrapolation.
5. No tool may claim certification.
6. The local server starts as localhost-only and read-only except for configured trace/report directories.
7. Tool invocation is limited to registered Codex tools.
8. No arbitrary code execution.
9. No hidden network access.
10. Input size and timeout limits are required.

## Certification caveat

AeroCodex is an engineering mathematics library with source traceability, tests, and validation evidence where stated. It is not by itself certified for safety-critical, airborne, launch, spacecraft, or mission-critical use. Regulated use requires project-specific requirements, verification, validation, configuration management, independence, assurance, and approval. Users are responsible for determining suitability for their application.
