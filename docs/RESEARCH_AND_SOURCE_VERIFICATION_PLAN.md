# Research and Source Verification Plan

## Purpose

AeroCodex must be built from checked sources, not memory or unreviewed formula dumps. Each equation, constant, dataset, and correlation needs a review trail.

## Source tiers

| Tier | Examples | Use |
|---|---|---|
| A | NASA/NTRS, NACA, NOAA, NIST, JPL, NAIF, standards bodies | Preferred source or validation target |
| B | Peer-reviewed journal articles, university textbooks, classic references | Acceptable with citation and review |
| C | Open-source implementations, calculators, lecture notes | Comparison only unless independently traced |
| D | Blogs, forums, uncited web pages | Never source of truth |

## Source review states

```text
unreviewed
candidate_identified
source_obtained
license_checked
equation_extracted
independent_review_complete
reference_data_extracted
validation_case_built
accepted
rejected
```

## Required fields for every source record

```text
source_id
title
author_or_organization
year
url_or_archive_location
source_tier
license_or_public_status
redistribution_notes
used_for
equation_or_table_refs
review_status
reviewer
checksum_if_downloaded
known_limitations
```

## Verification workflow

1. Identify candidate source.
2. Confirm authority and version.
3. Check license and redistribution restrictions.
4. Extract equation/table/reference case.
5. Implement independent tests.
6. Add evidence card.
7. Add release-report entry.
8. Assign evidence level.
9. Prevent stable exposure until review gate passes.

## Initial source registry

The seed registry is in `validation/source-registry/seed_sources.yaml`. It includes initial targets for compressible flow, standard atmosphere, NASA CEA comparison, bio-regenerative life support BVAD research, and JPL/NAIF astrodynamics records.
