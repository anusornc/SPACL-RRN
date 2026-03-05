# SPACL Nogood-Activation Ablation Summary

This summary captures the first controlled synthetic rerun where branch-level nogood pruning activates in adaptive mode.

## Source artifact
- `benchmarks/competitors/results/history/spacl_synthetic_ablation_20260303_230552/results.csv`

## Workload and overrides
- Workload: `reused_conflict_12`
- Repeats: `3`
- Warmups: `1`
- Overrides used to expose speculative execution:
  - `SPACL_SYNTH_PARALLEL_THRESHOLD=8`
  - `SPACL_SYNTH_COST_THRESHOLD_US=1`

## Observed medians
- `sequential`: `0.452 ms`
- `adaptive`: `1037.632 ms`
- `adaptive_no_nogood`: `6821.705 ms`
- `always_parallel`: `14424.697 ms`

## Nogood activity
- `adaptive`: non-zero pruning in all three repeats (`branches_pruned` / `nogood_hits` = `418`, `4159`, `13`)
- `adaptive_no_nogood`: `0` pruning/hits in all repeats
- `always_parallel`: `0` pruning/hits in all repeats

## Interpretation
- This artifact is a controlled stress run intended to verify mechanism activation, not to claim stable production-level speedups.
- Runtime spread remains high across repeats, so use medians and pruning counters descriptively.
