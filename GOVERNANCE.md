# Governance

AeroCodex is maintained as an evidence-first engineering mathematics project.

## Roles

- **Project lead:** scope, roadmap, release approval, final governance decisions.
- **Core maintainer:** core types, units, errors, dependency policy, API stability.
- **Numerics maintainer:** solvers, interpolation, convergence policy, tolerance policy.
- **Validation lead:** evidence-card schema, test links, reports, validation gates.
- **Domain maintainer:** technical correctness of a domain crate.
- **Data steward:** dataset provenance, licensing, checksums, extraction records.
- **Security/supply-chain reviewer:** dependency review, advisories, native-dependency gate, unsafe review.
- **External reviewer:** independent review of high-impact equations and validation methods.

## Review gates for stable public models

1. Source review.
2. Equation and unit review.
3. Domain-of-validity review.
4. Implementation review.
5. Test and tolerance review.
6. Documentation review.
7. Dependency/supply-chain review if dependencies are added.
8. Release report inclusion.

## Decision records

Architecture and policy changes should be recorded under `docs/adr/`.
