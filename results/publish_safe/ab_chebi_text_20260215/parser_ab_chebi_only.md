# Parser A/B (ChEBI Only, Forced Text)

- Baseline run: `ab_chebi_text_base_20260214`
- Experimental run: `ab_chebi_text_exp_fix_20260214`
- Configuration: Tableauxx only, `chebi.owl` only, forced text parse, no .owlbin preference, timeout 1800s, strict experimental mode enabled.
- Date: 2026-02-15

## Result

| Ontology | Baseline status | Baseline wall (ms) | Experimental status | Experimental wall (ms) | Speedup (base/exp) |
|---|---|---:|---|---:|---:|
| chebi.owl | timeout | 1802495 | success | 1518932 | 1.187x |

## Interpretation

- Baseline outcome: `timeout` (reasoning result: `unknown`)
- Experimental outcome: `success` (reasoning result: `consistent`)
- Because baseline timed out at the configured limit, the speedup is a **lower bound**.
- Lower-bound speedup: `1.187x`

## Validity checks

- Experimental parser log includes: `experimental_compact_ready triples=8806955 skipped=0`
- Experimental run produced non-empty ontology stats (classes/axioms present) and a final consistency result.

## Provenance

- Source CSV: `benchmarks/competitors/results/history/ab_chebi_text_base_20260214/results.csv`
- Source CSV: `benchmarks/competitors/results/history/ab_chebi_text_exp_fix_20260214/results.csv`
- Derived CSV: `results/publish_safe/ab_chebi_text_20260215/parser_ab_chebi_only.csv`
