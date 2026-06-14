# Security Policy

AeroCodex is not a network service by default, but supply-chain security matters because engineering users may embed the library in larger tools.

## Supported versions

Founder baseline versions are experimental. Security fixes should target the default branch until a stable release line exists.

## Reporting a vulnerability

Open a private security advisory in GitHub when available, or contact the maintainers privately. Include:

- affected commit, tag, or version;
- reproduction steps;
- impact;
- whether the issue affects code execution, data integrity, numerical correctness, provenance, or supply-chain controls.

## Security expectations

- No hidden native dependencies in the core product.
- No arbitrary script execution from datasets.
- No hidden network access during tests.
- Unsafe Rust is forbidden in this baseline and must be explicitly reviewed before any future use.
- Dependency and license policy checks are release gates.
