# Dependency Policy

AeroCodex dependencies must preserve the Rust-only trust model.

## Deny by default

The core workspace must not depend on:

- `cc`
- `cmake`
- `bindgen`
- `pkg-config`
- `vcpkg`
- `*_sys` crates
- BLAS, LAPACK, OpenBLAS, Netlib, SuiteSparse
- NASA CEA, REFPROP, CoolProp, Cantera, OpenFOAM, SU2, SPICE wrappers
- proprietary binary libraries

## Admission checklist

Before adding a dependency, record:

- crate name and version;
- license;
- whether default features are acceptable;
- whether build scripts exist;
- whether native code can be compiled or linked;
- whether unsafe code exists;
- whether the dependency is required in core, optional, dev-only, or tool-only;
- why a project-local implementation is not preferable.

## Automated check

Run:

```bash
cargo run -p xtask -- dependency-policy
```
