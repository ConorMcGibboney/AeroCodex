# A37 external M07 J2 perturbation / numerical propagation policy Wave 1

A37 records metadata-only terminal dispositions for the first bounded J2 perturbation and numerical-propagation policy slice.

## Scope

- Selected source-row range: `PORT_STATUS_RELEASE_GATE.csv:row_0057` through `PORT_STATUS_RELEASE_GATE.csv:row_0851`.
- Selected rows: 40.
- Source group: `10B_J2_perturbation_and_numerical_policy`.
- Formula family: `perturbation_or_J2`.
- Risk tier: `high_risk_requires_numerical_policy`.
- Disposition: `blocked_until_J2_perturbation_model_and_numerical_validation_policy`.

## Source-file distribution

- `ast_ch1_two_body.sci`: 1
- `ast_ch3_j2_rates.sci`: 6
- `ast_ch9_core.sci`: 24
- `ast_ch9_cowell.sci`: 3
- `ast_ch9_encke.sci`: 4
- `ast_ch9_integration.sci`: 2

## Boundary

No runtime source, Scilab source, certification, operational readiness, or external parity claim is added. The rows remain blocked until J2 perturbation force-model scope, numerical integration policy, frame/time conventions, source registry, and independent validation oracles are explicitly approved.
