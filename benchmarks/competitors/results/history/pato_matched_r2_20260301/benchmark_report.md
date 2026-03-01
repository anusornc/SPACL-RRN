# OWL Reasoner Head-to-Head Report

- Run ID: pato_matched_r2_20260301
- Generated: 2026-03-01T19:16:23+07:00
- Suite: large
- Operation: consistency
- Timeout per run: 900s
- Reasoners: tableauxx hermit openllet elk jfact pellet
- Primary metric: wall time (ms)

## Summary by Reasoner

| Reasoner | Success | Non-Comparable | Failed | Timeout | Not Available | Killed | Orphan | Avg Wall Time (ms) |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| tableauxx | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 3326 |
| hermit | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 11901 |
| openllet | 0 | 0 | 0 | 1 | 0 | 0 | 0 | N/A |
| elk | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 6524 |
| jfact | 0 | 0 | 0 | 1 | 0 | 0 | 0 | N/A |
| pellet | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 724357 |

## Detailed Results

| Reasoner | Ontology | Wall (ms) | Reported (ms) | Status | Reasoning Result |
|---|---|---:|---:|---|---|
| elk | pato.owl | 6524 | 379 | success | consistent |
| hermit | pato.owl | 11901 | 9499 | success | consistent |
| jfact | pato.owl | 902701 | 902701 | timeout | unknown |
| openllet | pato.owl | 902572 | 902572 | timeout | unknown |
| pellet | pato.owl | 724357 | 706609 | success | consistent |
| tableauxx | pato.owl | 3326 | 916 | success | consistent |

## Notes

- Wall time is the primary metric for cross-engine comparison.
- OWLAPI/Konclude internal reported times are kept as secondary diagnostics.
- `pellet` may appear as `not_available` when legacy artifact repositories are unavailable.
