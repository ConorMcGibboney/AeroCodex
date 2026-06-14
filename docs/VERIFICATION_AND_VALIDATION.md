# Verification and Validation Standard

AeroCodex separates equation traceability, implementation verification, numerical verification, model validation, and certification.

## Evidence levels

| Level | Name | Meaning |
| --- | --- | --- |
| C0 | Proposed | Candidate entry exists only as a design candidate or experimental API. |
| C1 | Equation-traceable | Source, assumptions, units, and domain are recorded. |
| C2 | Implementation-verified | Tests show the implementation computes the stated model correctly. |
| C3 | Reference-validated | Outputs match accepted tables, reference software output, or published benchmark cases within stated tolerances. |
| C4 | Experiment-validated | Model has documented comparison to experimental or flight/wind-tunnel/bench data within an intended-use envelope. |
| CX | Deprecated/withdrawn | Entry remains indexed for traceability but is not recommended for new use. |

## Stable public calculation rule

A stable public calculation must have:

- a Codex ID;
- an evidence card;
- tests linked to that card;
- typed units or strong dimensionless types;
- validity range;
- branch behavior;
- explicit failure modes;
- documentation;
- release-report inclusion.

## No silent extrapolation

Empirical and approximate models must not silently extrapolate beyond their declared valid range. The caller must choose an explicit extrapolation policy when such behavior is permitted.
