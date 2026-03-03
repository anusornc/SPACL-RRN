# SPACL Scheduler Policy Ablation Summary

This summary consolidates the controlled synthetic scheduler-policy ablation used in the manuscript revision.

## Source artifact
- `benchmarks/competitors/results/history/spacl_synthetic_ablation_20260303_221641/results.csv`

## Workload and overrides
- Workload: `union_4x10`
- Repeats: `3`
- Warmups: `1`
- Override thresholds used only for policy exposure:
  - `SPACL_SYNTH_PARALLEL_THRESHOLD=4`
  - `SPACL_SYNTH_COST_THRESHOLD_US=1`

## Median wall-clock results
- `sequential`: `0.146 ms`
- `adaptive`: `53.149 ms`
- `adaptive_no_nogood`: `84.560 ms`
- `always_parallel`: `108.012 ms`

## Interpretation
- The run is a controlled scheduler-policy exposure experiment, not a default-setting benchmark.
- It shows that once speculative execution is deliberately activated on the same branch-heavy synthetic workload, the adaptive policy is materially better than forcing parallel execution everywhere.
- `nogood_hits` remained `0` in this artifact, so the table supports scheduling-policy claims only and should not be interpreted as positive nogood-learning evidence.

## Variance note
- For later scheduler-ablation runs, pinned-core execution is recommended (for example via `TASKSET_CPUS=0-3` in `benchmarks/competitors/scripts/run_spacl_ablation.sh`).
- Core pinning reduces machine-noise variance, but it does not remove scheduler/search-order nondeterminism inside speculative runs.
