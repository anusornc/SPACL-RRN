# Parser A/B (Large Suite, No ChEBI)

- Baseline run: `ab_large_text_base_20260214`
- Experimental run: `ab_large_text_exp_fix_20260214`
- Configuration: Tableauxx only, forced text parse, no .owlbin preference, timeout 900s, strict experimental mode enabled.
- Date: 2026-02-14
- Primary metric: wall time ms (benchmark harness).
- Data hygiene: early run `ab_large_text_exp_20260214` was discarded due a detected parser race causing invalid empty ontology statistics.

| Ontology | Baseline (ms) | Experimental (ms) | Speedup (base/exp) | Baseline status | Experimental status |
|---|---:|---:|---:|---|---|
| doid.owl | 56560 | 49171 | 1.150x | success | success |
| go-basic.owl | 272080 | 269065 | 1.011x | success | success |
| pato.owl | 41770 | 44634 | 0.936x | success | success |
| uberon.owl | 260139 | 254569 | 1.022x | success | success |

## Aggregate

- Total baseline wall time: `630549 ms`
- Total experimental wall time: `617439 ms`
- Overall speedup (base/exp): `1.021x`
- Mean per-ontology speedup (unweighted): `1.030x`

## Provenance

- Source CSV: `benchmarks/competitors/results/history/ab_large_text_base_20260214/results.csv`
- Source CSV: `benchmarks/competitors/results/history/ab_large_text_exp_fix_20260214/results.csv`
- Derived CSV: `results/publish_safe/ab_large_text_20260214/parser_ab_large_no_chebi.csv`
