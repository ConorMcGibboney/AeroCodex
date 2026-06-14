# MVP Vertical Slice: Normal Shock Pressure Ratio

## Target Codex Entry

```text
gasdyn.normal_shock.p2_over_p1
```

## What must exist

1. Rust function for pressure ratio.
2. Strong input types: `Mach`, `RatioOfSpecificHeats`.
3. Strong output type: `PressureRatio`.
4. Evidence card.
5. Agent metadata in the evidence card.
6. Generated input schema.
7. Generated output schema.
8. Generated error schema.
9. Agent tool spec.
10. Companion Vector Token.
11. Agent registry entry.
12. Retrieval test.
13. JSON invocation test.
14. Invalid input test returning `RequiresSupersonic`.
15. Trace file.
16. Trace replay test.
17. Explain response.
18. Verify response.

## Formula

For a calorically perfect gas normal shock:

```text
p2/p1 = 1 + 2*gamma/(gamma + 1) * (M1^2 - 1)
```

Validity:

```text
M1 > 1
gamma > 1
```

## Agent search expectation

Query:

```text
pressure rise across normal shock
```

Expected top result:

```text
gasdyn.normal_shock.p2_over_p1
```

Expected reasons:

- Alias match: `pressure rise across shock`.
- Semantic match: `normal shock pressure ratio`.
- Output quantity match: `pressure_ratio`.
- Evidence level satisfied.

## Agent invocation example

Input:

```json
{
  "mach1": {"value": 2.5},
  "gamma": {"value": 1.4}
}
```

Expected output:

```json
{
  "value": 7.125,
  "quantity": "pressure_ratio",
  "unit": "dimensionless",
  "validity": "valid"
}
```

## Agent error example

Input:

```json
{
  "mach1": {"value": 0.82},
  "gamma": {"value": 1.4}
}
```

Expected error:

```json
{
  "code": "RequiresSupersonic",
  "parameter": "mach1",
  "expected_condition": "mach1 > 1"
}
```
