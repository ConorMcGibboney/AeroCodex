# 100% Rust Core Product Policy

## Mandatory Rust-only rule

The stable core of AeroCodex shall be implemented in Rust and shall not compile, link, execute, or require foreign-language source or binary dependencies.

Rust may consume checked-in reference data with documented provenance. External tools may be cited as validation references, but they shall not be runtime dependencies of the core library.

## Forbidden in the core

- C, C++, Fortran, Python, MATLAB, Julia, Modelica, or generated foreign source.
- BLAS, LAPACK, OpenBLAS, Netlib, SuiteSparse, system libraries, proprietary binary libraries.
- NASA CEA wrappers, REFPROP wrappers, CoolProp wrappers, Cantera wrappers, OpenFOAM wrappers, SU2 wrappers, SPICE wrappers.
- Proprietary datasets or redistribution-restricted tables.
- Feature flags that silently enable foreign native dependencies.

## Allowed in the core

- Rust source, Rust macros, and Rust build scripts that do not compile foreign code.
- Rust crates compiled by Cargo after license, feature, native-build, and adequacy review.
- Checked-in public/reference data with provenance and redistribution review.
