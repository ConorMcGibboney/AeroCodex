# Research safety caveats for testers

AeroCodex is research/preliminary-design software. Treat every local test result, validation card, source-registry seed, formula-vault record, equation inventory row, and generated report as conservative engineering-development evidence only.

AeroCodex is not certified, not operational, not flight-ready, not habitat-safe, not medical-use software, and not approved for regulated use. Passing local checks does not prove physical validity, safety, certification, or mission readiness.

## What local checks can show

Local checks can show that, for a specific checkout and toolchain:

- Rust formatting, linting, tests, and documentation generation completed;
- validation metadata files conform to the current repository schemas and vocabulary;
- the equation inventory agrees with the current repository files;
- formula-vault metadata satisfies the current quarantine gate;
- the dependency policy did not find blocked dependency tokens.

These are software-maintenance and governance signals. They are not a substitute for source review, independent derivation checks, numerical validation, uncertainty analysis, system safety review, test-article evidence, qualification, certification, or approval by any authority.

## What validation cards mean

Validation cards are conservative planning and traceability records. A validation card can identify intended evidence, assumptions, tests, failure modes, and limits. It does not automatically mean the referenced formula is implemented, physically validated, safe, certified, or approved for regulated use.

## What formula-vault records mean

Formula-vault records are candidate metadata and quarantine gates. They do not automatically create public Rust functions, source-equivalence evidence, reference-oracle evidence, physical validation, certification, flight readiness, mission readiness, habitat safety, medical suitability, or regulated-use approval.

## What equation inventory rows mean

Equation inventory rows classify repository items. Executable research equations are still research/preliminary-design kernels. Metadata-only candidates are not implementations. Validation-card-only records are not formula implementations. Helper algorithms are support routines and are not counted as executable research equations.

## Tester responsibilities

When reporting friend-test results:

- preserve the exact command that failed or succeeded;
- include platform and Rust toolchain information;
- avoid editing source files to make a check pass unless you are explicitly preparing a separate patch;
- do not treat a clean local run as evidence for safety-critical, operational, crewed, habitat, medical, regulated, or mission use;
- do not attach external source archives, generated binaries, local evidence logs, credentials, `.env` files, `target/`, or root `Cargo.lock` as part of a routine friend-test report.
