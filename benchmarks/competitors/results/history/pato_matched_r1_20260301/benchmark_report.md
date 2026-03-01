# OWL Reasoner Head-to-Head Report

- Run ID: pato_matched_r1_20260301
- Generated: 2026-03-01T10:51:03+07:00
- Suite: large
- Operation: consistency
- Timeout per run: 900s
- Reasoners: tableauxx hermit openllet elk jfact pellet
- Primary metric: wall time (ms)

## Summary by Reasoner

| Reasoner | Success | Non-Comparable | Failed | Timeout | Not Available | Killed | Orphan | Avg Wall Time (ms) |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| tableauxx | 1 | 0 | 0 | 0 | 4 | 0 | 0 | 3137 |
| hermit | 1 | 0 | 0 | 0 | 4 | 0 | 0 | 14457 |
| openllet | 0 | 0 | 0 | 1 | 4 | 0 | 0 | N/A |
| elk | 1 | 0 | 0 | 0 | 4 | 0 | 0 | 7042 |
| jfact | 0 | 0 | 0 | 1 | 4 | 0 | 0 | N/A |
| pellet | 1 | 0 | 0 | 0 | 4 | 0 | 0 | 752048 |

## Detailed Results

| Reasoner | Ontology | Wall (ms) | Reported (ms) | Status | Reasoning Result |
|---|---|---:|---:|---|---|
| elk | chebi.owl | -1 | -1 | not_available | unknown |
| elk | doid.owl | -1 | -1 | not_available | unknown |
| elk | go-basic.owl | -1 | -1 | not_available | unknown |
| elk | pato.owl | 7042 | 477 | success | consistent |
| elk | uberon.owl | -1 | -1 | not_available | unknown |
| hermit | chebi.owl | -1 | -1 | not_available | unknown |
| hermit | doid.owl | -1 | -1 | not_available | unknown |
| hermit | go-basic.owl | -1 | -1 | not_available | unknown |
| hermit | pato.owl | 14457 | 11933 | success | consistent |
| hermit | uberon.owl | -1 | -1 | not_available | unknown |
| jfact | chebi.owl | -1 | -1 | not_available | unknown |
| jfact | doid.owl | -1 | -1 | not_available | unknown |
| jfact | go-basic.owl | -1 | -1 | not_available | unknown |
| jfact | pato.owl | 903739 | 903739 | timeout | unknown |
| jfact | uberon.owl | -1 | -1 | not_available | unknown |
| openllet | chebi.owl | -1 | -1 | not_available | unknown |
| openllet | doid.owl | -1 | -1 | not_available | unknown |
| openllet | go-basic.owl | -1 | -1 | not_available | unknown |
| openllet | pato.owl | 902504 | 902504 | timeout | unknown |
| openllet | uberon.owl | -1 | -1 | not_available | unknown |
| pellet | chebi.owl | -1 | -1 | not_available | unknown |
| pellet | doid.owl | -1 | -1 | not_available | unknown |
| pellet | go-basic.owl | -1 | -1 | not_available | unknown |
| pellet | pato.owl | 752048 | 732954 | success | consistent |
| pellet | uberon.owl | -1 | -1 | not_available | unknown |
| tableauxx | chebi.owl | -1 | -1 | not_available | unknown |
| tableauxx | doid.owl | -1 | -1 | not_available | unknown |
| tableauxx | go-basic.owl | -1 | -1 | not_available | unknown |
| tableauxx | pato.owl | 3137 | 915 | success | consistent |
| tableauxx | uberon.owl | -1 | -1 | not_available | unknown |

## Notes

- Wall time is the primary metric for cross-engine comparison.
- OWLAPI/Konclude internal reported times are kept as secondary diagnostics.
- `pellet` may appear as `not_available` when legacy artifact repositories are unavailable.
