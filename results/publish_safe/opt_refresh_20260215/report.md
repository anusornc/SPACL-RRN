# Optimized Parser Refresh (2026-02-15)

## Large Suite (No ChEBI)

Sources:
- baseline: `ab_large_text_base_20260214`
- previous experimental: `ab_large_text_exp_fix_20260214`
- optimized experimental: `opt_large_no_chebi_20260215`

| Ontology | Baseline (ms) | Prev Exp (ms) | Optimized Exp (ms) | Base/Opt | Prev/Opt |
|---|---:|---:|---:|---:|---:|
| doid.owl | 56560 | 49171 | 31969 | 1.769x | 1.538x |
| go-basic.owl | 272080 | 269065 | 162263 | 1.677x | 1.658x |
| pato.owl | 41770 | 44634 | 29943 | 1.395x | 1.491x |
| uberon.owl | 260139 | 254569 | 146535 | 1.775x | 1.737x |

- Total baseline: `630549 ms`
- Total previous experimental: `617439 ms`
- Total optimized experimental: `370710 ms`
- Overall Base/Opt: `1.701x`
- Overall Prev/Opt: `1.666x`

## ChEBI Only

Sources:
- baseline: `ab_chebi_text_base_20260214`
- previous experimental: `ab_chebi_text_exp_fix_20260214`
- optimized experimental: `opt_chebi_only_20260215`

| Ontology | Baseline | Baseline (ms) | Prev Exp | Prev Exp (ms) | Opt Exp | Opt Exp (ms) | Base/Opt | Prev/Opt |
|---|---|---:|---|---:|---|---:|---:|---:|
| chebi.owl | timeout | 1802495 | success | 1518932 | success | 880640 | 2.047x | 1.725x |

- Baseline note: `lower_bound_due_to_timeout`

## Files

- `results/publish_safe/opt_refresh_20260215/large_no_chebi_optimized_comparison.csv`
- `results/publish_safe/opt_refresh_20260215/chebi_optimized_comparison.csv`
