# OWL Reasoner Head-to-Head Report

- Run ID: pato_matched_r3_20260301
- Generated: 2026-03-01T22:49:25+07:00
- Suite: large
- Operation: consistency
- Timeout per run: 900s
- Reasoners: tableauxx hermit openllet elk jfact pellet
- Primary metric: wall time (ms)

## Summary by Reasoner

| Reasoner | Success | Non-Comparable | Failed | Timeout | Not Available | Killed | Orphan | Avg Wall Time (ms) |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| tableauxx | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 3238 |
| hermit | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 12362 |
| openllet | 0 | 0 | 0 | 1 | 0 | 0 | 0 | N/A |
| elk | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 6664 |
| jfact | 0 | 0 | 0 | 1 | 0 | 0 | 0 | N/A |
| pellet | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 745159 |

## Detailed Results

| Reasoner | Ontology | Wall (ms) | Reported (ms) | Status | Reasoning Result |
|---|---|---:|---:|---|---|
| elk | pato.owl | 6664 | 350 | success | consistent |
| hermit | pato.owl | 12362 | 10013 | success | consistent |
| jfact | pato.owl | 902516 | 902516 | timeout | unknown |
| openllet | pato.owl | 902608 | 902608 | timeout | unknown |
| pellet | pato.owl | 745159 | 726936 | success | consistent |
| tableauxx | pato.owl | 3238 | 916 | success | consistent |

## Notes

- Wall time is the primary metric for cross-engine comparison.
- OWLAPI/Konclude internal reported times are kept as secondary diagnostics.
- `pellet` may appear as `not_available` when legacy artifact repositories are unavailable.
