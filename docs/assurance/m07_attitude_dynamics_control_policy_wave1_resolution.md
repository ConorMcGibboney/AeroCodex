# A36 external M07 attitude dynamics/control policy Wave 1

A36 records metadata-only terminal dispositions for the 38 rows in `10A_attitude_dynamics_and_control_policy`.

## Scope

- Selected source-row range: `PORT_STATUS_RELEASE_GATE.csv:row_1052` through `PORT_STATUS_RELEASE_GATE.csv:row_1215`.
- Selected rows: 38.
- Source group: `10A_attitude_dynamics_and_control_policy`.
- Formula family: `coordinate_transform_sensitive`.
- Risk tier: `high_risk_requires_numerical_policy`.
- Disposition: `blocked_until_attitude_dynamics_control_and_integration_policy`.

## Source-file distribution

- `ac_utils.sci`: 1
- `coning_maneuver.sci`: 3
- `dual_spin.sci`: 6
- `gravity_gradient.sci`: 5
- `nutation_damper.sci`: 2
- `rb_dynamics.sci`: 12
- `rb_kinematics.sci`: 3
- `rb_quaternions.sci`: 2
- `rb_rotations.sci`: 2
- `torque_free.sci`: 2

## Boundary

No runtime source, Scilab source, certification, operational readiness, or external parity claim is added. The rows remain blocked until attitude dynamics, control law, torque/inertia, frame-orientation, source-registry, integration policy, and independent validation-oracle policy are approved.
