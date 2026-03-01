# Matched PATO Aggregate 2026-03-01

This directory aggregates three clean matched runs executed under the same large-suite policy on `pato.owl` only.

Source runs:
- `benchmarks/competitors/results/history/pato_matched_r1_20260301`
- `benchmarks/competitors/results/history/pato_matched_r2_20260301`
- `benchmarks/competitors/results/history/pato_matched_r3_20260301`

Timeout policy: 900s per reasoner/ontology pair.
Primary metric: container-level wall time (startup + parsing + reasoning).
Median successful wall times are computed across successful runs only.
