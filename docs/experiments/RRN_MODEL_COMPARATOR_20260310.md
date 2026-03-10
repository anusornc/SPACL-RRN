# RRN Policy Model Comparator (2026-03-10)

Branch: `exp/hybrid-rrn-paper`  
Goal: compare branch-ordering policies under the same synthetic policy-isolation protocol:

- `heuristic` (deterministic baseline)
- `hybrid_rrn` + linear pairwise model (`rrn_linear_model_v3_pairwise.json`)
- `hybrid_rrn` + GBDT-stump models (`rrn_gbdt_stump_model_v1.json`, `rrn_gbdt_stump_model_v2.json`)

## Protocol lock

- Workloads: `mixed_operands_8,mixed_operands_16`
- Modes: `adaptive`
- Repeats: `5` (with `1` warmup)
- Gate overrides: `SPACL_SYNTH_PARALLEL_THRESHOLD=4`, `SPACL_SYNTH_COST_THRESHOLD_US=1`
- Runner: `cargo run --bin run_spacl_synthetic_ablation`

All values below are read from raw CSV artifacts under
`benchmarks/competitors/results/history/` (no hand-edited metric rows).

## Model training artifacts

- Linear pairwise model (existing):
  - `benchmarks/models/rrn_linear_model_v3_pairwise.json`
- New GBDT-stump models:
  - v1 (higher capacity): `benchmarks/models/rrn_gbdt_stump_model_v1.json`
  - v2 (regularized): `benchmarks/models/rrn_gbdt_stump_model_v2.json`

Training data source for GBDT:

- `benchmarks/models/rrn_train_subset_r2_100k.jsonl`

## Evaluation run IDs

- Heuristic baseline:
  - `spacl_synthetic_ablation_20260310_094758`
- Hybrid + linear v3:
  - `spacl_synthetic_ablation_20260310_095023`
- Hybrid + GBDT v1:
  - `spacl_synthetic_ablation_20260310_095209`
- Hybrid + GBDT v2:
  - `spacl_synthetic_ablation_20260310_095728`

## Median / P95 summary (ms)

| Policy | mixed_operands_8 | mixed_operands_16 |
|---|---:|---:|
| heuristic | 4064.497 / 28538.353 | 6262.586 / 30039.719 |
| hybrid + linear v3 | **2331.282 / 4461.202** | **4012.926 / 28945.686** |
| hybrid + GBDT v1 | 25479.835 / 30484.816 | 18472.111 / 30035.976 |
| hybrid + GBDT v2 | 10576.917 / 27794.777 | 7139.843 / 7573.800 |

## Repeat-matched pair outcomes (10 pairs = 2 workloads x 5 repeats)

- Linear v3 vs heuristic: `5 / 10` wins each (direction mixed), but lower medians for linear on both workloads in this pass.
- GBDT v1 vs heuristic: `4 / 10` wins (worse overall).
- GBDT v2 vs heuristic: `5 / 10` wins (tie on wins, but weaker medians).
- GBDT v2 vs linear v3: `4 / 10` wins for GBDT v2 (`6 / 10` for linear v3).

## Operational telemetry checks

In both hybrid+linear and hybrid+GBDT runs:

- `hybrid_model_calls > 0`
- `policy_fallbacks = 0`

This confirms model-backed ordering was active (no fallback contamination).

## Decision for manuscript track

1. Keep linear pairwise (`v3`) as primary learned-policy result.
2. Report GBDT results as exploratory negative/neutral comparator.
3. Keep claims conservative: policy gains remain workload-sensitive and variance-sensitive.
