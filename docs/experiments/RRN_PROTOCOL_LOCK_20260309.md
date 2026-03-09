# RRN Hybrid Baseline and Protocol Lock (2026-03-09)

Branch: `exp/hybrid-rrn-paper`  
Scope: lock reproducible baseline and benchmark knobs for policy-level comparison.

## Baseline freeze

- Baseline merge-base with `origin/main`: `ea9bb99a68d944ce33b984ea05d19befde5d59e5`
- Baseline summary: `ea9bb99` (2026-03-07) `Split synthetic scalability visuals into three figures`

This baseline is the symbolic reference point before model-backed `hybrid_rrn` behavior.

## Locked benchmark knobs

Use script:

```bash
benchmarks/competitors/scripts/run_rrn_policy_protocol.sh
```

Default locked settings in that script:

- `REPEATS=3`
- `TIMEOUT_SECONDS=900`
- `MODE_MATRIX=adaptive|1|baseline,adaptive|1|heuristic,adaptive|1|hybrid_rrn`
- `WORKLOADS=disjunctive_10k,disjunctive_30k,univ-bench`
- `SPACL_EMIT_STATS=1` (set by `run_spacl_ablation.sh`)
- `SPACL_BRANCH_SNAPSHOT_FILE=<run_dir>/branch_snapshots.jsonl`

## Optional model path

To run hybrid with an explicit model file:

```bash
RUN_ID=rrn_model_r1 \
RRN_MODEL_PATH=/absolute/path/to/rrn_model.json \
benchmarks/competitors/scripts/run_rrn_policy_protocol.sh
```

Without `RRN_MODEL_PATH`, `hybrid_rrn` runs in strict deterministic fallback mode and is still comparable.

## Output artifacts

Per run ID:

- `benchmarks/competitors/results/history/<RUN_ID>/results.csv`
- `benchmarks/competitors/results/history/<RUN_ID>/logs/*.log`
- `benchmarks/competitors/results/history/<RUN_ID>/branch_snapshots.jsonl` (training export)

## Fairness guardrails

- Same workloads, timeout, and repeats across policy modes.
- Same executable path and host resources for all modes.
- Final consistency decision remains symbolic; policy only reorders branch expansion.
