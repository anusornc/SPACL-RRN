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

## First offline training command (linear baseline)

After collecting snapshots (prefer `SPACL_BRANCH_POLICY=heuristic` runs for distillation targets):

```bash
cargo run --bin train_rrn_linear_model -- \
  benchmarks/competitors/results/history/<RUN_ID>/branch_snapshots.jsonl \
  benchmarks/models/rrn_linear_model.json \
  heuristic
```

Then evaluate with:

```bash
RUN_ID=rrn_model_eval_r1 \
RRN_MODEL_PATH=benchmarks/models/rrn_linear_model.json \
MODE_MATRIX='adaptive|1|baseline,adaptive|1|hybrid_rrn' \
benchmarks/competitors/scripts/run_rrn_policy_protocol.sh
```

## Fairness guardrails

- Same workloads, timeout, and repeats across policy modes.
- Same executable path and host resources for all modes.
- Final consistency decision remains symbolic; policy only reorders branch expansion.

## 2026-03-09 execution notes

### Important caveat (current default `.owl` protocol)

Run `rrn_snapshot_seed_20260309_r1` on the default `run_rrn_policy_protocol.sh` workloads
(`disjunctive_10k`, `disjunctive_30k`) produced:

- `used_parallel=false`
- `branches_created=0`
- `branch_snapshots_written=0`

So this path is valid for end-to-end harness checks, but not sufficient for branch-policy training data.

### Branch-active snapshot protocol used

To force branch-policy activity, use the synthetic ablation runner with low gate thresholds:

```bash
SPACL_BRANCH_POLICY=heuristic \
SPACL_BRANCH_SNAPSHOT_FILE=benchmarks/competitors/results/history/rrn_synth_snapshot_20260309_r2/branch_snapshots.jsonl \
SPACL_SYNTH_ABLATION_WORKLOADS='mixed_operands_16,mixed_operands_32,reused_conflict_12' \
SPACL_SYNTH_ABLATION_MODES='adaptive' \
SPACL_SYNTH_ABLATION_REPEATS=3 \
SPACL_SYNTH_ABLATION_WARMUPS=1 \
SPACL_SYNTH_PARALLEL_THRESHOLD=4 \
SPACL_SYNTH_COST_THRESHOLD_US=1 \
cargo run --quiet --bin run_spacl_synthetic_ablation
```

Artifacts:

- Snapshot set 1: `benchmarks/competitors/results/history/rrn_synth_snapshot_20260309_r1/branch_snapshots.jsonl` (`328743` rows)
- Snapshot set 2: `benchmarks/competitors/results/history/rrn_synth_snapshot_20260309_r2/branch_snapshots.jsonl` (`277661` rows)
- Branch-active run: `benchmarks/competitors/results/history/spacl_synthetic_ablation_20260309_163120/results.csv`
  - `policy_reordered_splits=75267`
  - `branch_snapshots_written=188130`

### Models trained

- `benchmarks/models/rrn_linear_model_v1.json` (from `rrn_train_subset_50k.jsonl`, low-information baseline)
- `benchmarks/models/rrn_linear_model_v2.json` (from `rrn_train_subset_r2_100k.jsonl`, branch-active mixed operands)

### Policy matrix runs (same synthetic protocol)

- Baseline: `benchmarks/competitors/results/history/spacl_synthetic_ablation_20260309_163617/results.csv`
- Heuristic: `benchmarks/competitors/results/history/spacl_synthetic_ablation_20260309_164013/results.csv`
- Hybrid + model (`v2`): `benchmarks/competitors/results/history/spacl_synthetic_ablation_20260309_164224/results.csv`
- Hybrid fallback (no model): `benchmarks/competitors/results/history/spacl_synthetic_ablation_20260309_164507/results.csv`

Telemetry sanity checks:

- Hybrid + model: `hybrid_model_calls=151558`, `policy_fallbacks=0`
- Hybrid fallback: `hybrid_model_calls=0`, `policy_fallbacks=148545`
- Heuristic: `policy_reordered_splits=74548`

Policy-matrix aggregate averages (wall-clock ms, `REPEATS=2`, synthetic branch-active set):

- Baseline: `16935.150`
- Heuristic: `9989.586`
- Hybrid + model (`v2`): `16396.951`
- Hybrid fallback: `15269.925`

Per-workload averages (ms):

- `mixed_operands_16`: baseline `9337.001`, heuristic `7073.468`, hybrid-model `15732.460`, fallback `15088.069`
- `mixed_operands_32`: baseline `35128.711`, heuristic `22296.522`, hybrid-model `32708.943`, fallback `30042.280`
- `reused_conflict_12`: baseline `6339.738`, heuristic `598.769`, hybrid-model `749.452`, fallback `679.427`

Current preliminary outcome on this matrix: heuristic branch ranking is stronger than `rrn_linear_model_v2`; keep claims conservative and treat hybrid mode as in-progress.

## 2026-03-10 strict evidence pass (no hardcoded/fake numbers)

Matrix settings (identical across policies):

- `SPACL_SYNTH_ABLATION_WORKLOADS='mixed_operands_16,mixed_operands_32,reused_conflict_12'`
- `SPACL_SYNTH_ABLATION_MODES='adaptive'`
- `SPACL_SYNTH_ABLATION_REPEATS=5`
- `SPACL_SYNTH_ABLATION_WARMUPS=1`
- `SPACL_SYNTH_PARALLEL_THRESHOLD=4`
- `SPACL_SYNTH_COST_THRESHOLD_US=1`
- `taskset -c 0-11` for all runs

Run IDs:

- baseline: `spacl_synthetic_ablation_20260310_001159`
- heuristic: `spacl_synthetic_ablation_20260310_001810`
- hybrid + model (`v2`): `spacl_synthetic_ablation_20260310_002215`
- hybrid fallback (no model): `spacl_synthetic_ablation_20260310_002807`

Raw result files:

- `benchmarks/competitors/results/history/spacl_synthetic_ablation_20260310_001159/results.csv`
- `benchmarks/competitors/results/history/spacl_synthetic_ablation_20260310_001810/results.csv`
- `benchmarks/competitors/results/history/spacl_synthetic_ablation_20260310_002215/results.csv`
- `benchmarks/competitors/results/history/spacl_synthetic_ablation_20260310_002807/results.csv`

Summary was generated directly from these CSVs (no manual editing of metrics) by:

```bash
benchmarks/competitors/scripts/summarize_policy_matrix.sh \
  benchmarks/competitors/results/history/rrn_policy_matrix_summary_20260310_final.csv \
  benchmarks/competitors/results/history/spacl_synthetic_ablation_20260310_001159/results.csv \
  benchmarks/competitors/results/history/spacl_synthetic_ablation_20260310_001810/results.csv \
  benchmarks/competitors/results/history/spacl_synthetic_ablation_20260310_002215/results.csv \
  benchmarks/competitors/results/history/spacl_synthetic_ablation_20260310_002807/results.csv
```

Generated summary artifact:

- `benchmarks/competitors/results/history/rrn_policy_matrix_summary_20260310_final.csv`

High-level reading from this strict pass:

- On branch-reordered mixed workloads, heuristic remains strongest in this pass.
- Hybrid model path is active (`mean_hybrid_model_calls > 0`) but does not yet beat heuristic.
- Keep paper claims conservative and report this as negative/neutral hybrid result at current model quality.

## 2026-03-10 pairwise-model refinement pass

We then switched the trainer objective from score regression to pairwise ranking:

- `RRN_TRAIN_OBJECTIVE=pairwise`
- output model: `benchmarks/models/rrn_linear_model_v3_pairwise.json`
- trainer telemetry: `pair_acc=1.0000` on the training subset

Evaluation runs (same gate/worker controls, mixed workloads only):

- heuristic: `benchmarks/competitors/results/history/spacl_synthetic_ablation_20260310_063022/results.csv`
- hybrid + `v3_pairwise`: `benchmarks/competitors/results/history/spacl_synthetic_ablation_20260310_063405/results.csv`

Matrix controls:

- `SPACL_SYNTH_ABLATION_WORKLOADS='mixed_operands_8,mixed_operands_16'`
- `SPACL_SYNTH_ABLATION_REPEATS=5`
- `taskset -c 0-11`

Observed direction from raw run pairs:

- Hybrid won `9/10` repeat-matched comparisons across these two mixed workloads.
- `mixed_operands_8` median improved from `16467.947` (heuristic) to `6968.183` (hybrid v3).
- `mixed_operands_16` medians were close (`15700.232` heuristic vs `15813.491` hybrid), but hybrid had lower mean due several faster repeats.

Caveat: runtime variance remains high; keep claims bounded and report both median and spread.

## 2026-03-10 confirmation pass (different CPU set)

To check reproducibility direction, we repeated heuristic vs hybrid-v3 on a different pinned CPU set:

- `taskset -c 12-23`
- workloads: `mixed_operands_8,mixed_operands_16`
- `SPACL_SYNTH_ABLATION_REPEATS=5`
- same gate controls (`SPACL_SYNTH_PARALLEL_THRESHOLD=4`, `SPACL_SYNTH_COST_THRESHOLD_US=1`)

Run IDs:

- heuristic: `benchmarks/competitors/results/history/spacl_synthetic_ablation_20260310_064158/results.csv`
- hybrid + `v3_pairwise`: `benchmarks/competitors/results/history/spacl_synthetic_ablation_20260310_064702/results.csv`

Result summary from raw CSV:

- `mixed_operands_8` median: heuristic `30102.014` vs hybrid `3791.560`
- `mixed_operands_16` median: heuristic `19175.384` vs hybrid `13027.817`
- repeat-matched wins: hybrid `9/10`

This confirms the improved direction of the pairwise-trained model on branch-reordered mixed workloads under a second CPU pinning setup.

## 2026-03-10 non-neural comparator pass (GBDT-stump)

To test whether a stronger non-neural ranker outperforms linear-v3, we added a
GBDT-stump trainer and evaluated two models from the same snapshot source:

- training source: `benchmarks/models/rrn_train_subset_r2_100k.jsonl`
- model v1: `benchmarks/models/rrn_gbdt_stump_model_v1.json` (64 trees)
- model v2: `benchmarks/models/rrn_gbdt_stump_model_v2.json` (16 trees, larger leaf)

Evaluation protocol (same as pairwise pass):

- workloads: `mixed_operands_8,mixed_operands_16`
- repeats: `5`
- mode: `adaptive`
- gate overrides: `SPACL_SYNTH_PARALLEL_THRESHOLD=4`, `SPACL_SYNTH_COST_THRESHOLD_US=1`

Run IDs:

- heuristic: `spacl_synthetic_ablation_20260310_094758`
- hybrid + linear v3: `spacl_synthetic_ablation_20260310_095023`
- hybrid + GBDT v1: `spacl_synthetic_ablation_20260310_095209`
- hybrid + GBDT v2: `spacl_synthetic_ablation_20260310_095728`

Outcome from raw CSV artifacts:

- linear-v3 remains strongest overall in this pass (lower medians on both mixed workloads).
- GBDT v1 shows unstable high-variance behavior and is worse than linear-v3.
- GBDT v2 is more stable than v1 but still does not beat linear-v3 on median outcome.

Decision: keep linear-v3 as primary hybrid model in the manuscript; include
GBDT as exploratory comparator with conservative interpretation.
