# Scheduler-Centered Ablation Checklist

Goal: isolate the contribution of speculative scheduling and nogood reuse while keeping parser, ontology input, and core branch semantics fixed.

## Modes

- `SPACL_SCHED_MODE=sequential SPACL_NOGOOD=0`
  - Sequential baseline without nogood reuse.
- `SPACL_SCHED_MODE=adaptive SPACL_NOGOOD=0`
  - Adaptive scheduling only.
- `SPACL_SCHED_MODE=adaptive SPACL_NOGOOD=1`
  - Full adaptive SPACL.
- `SPACL_SCHED_MODE=always_parallel SPACL_NOGOOD=0`
  - Force speculative scheduling to expose overhead on poor-fit workloads.
- `SPACL_SCHED_MODE=always_parallel SPACL_NOGOOD=1`
  - Optional: force speculative scheduling with nogood reuse enabled.

## Telemetry

Set `SPACL_EMIT_STATS=1` for ablation runs. The CLI emits a single `[spacl]` line with:

- `mode`
- `used_parallel`
- `branches_created`
- `work_items_expanded`
- `branches_pruned`
- `nogood_hits`
- `local_cache_hits`
- `global_cache_hits`
- `steal_attempts`
- `steal_successes`

## Tier 1 Workloads

These should be run first because they answer the scheduler question most directly.

- `benchmarks/ontologies/disjunctive/disjunctive_simple.owl`
- `benchmarks/ontologies/disjunctive/disjunctive_test.owl`
- `benchmarks/ontologies/disjunctive/disjunctive_5k.ofn`
- Existing inconsistent disjunctive workload(s) used for nogood evaluation

## Tier 2 Workloads

Use these to show that adaptive scheduling stays out of the way on branch-light ontologies.

- `tests/data/hierarchy_10000.owl`
- `tests/data/hierarchy_100000.owl`
- `tests/data/univ-bench.owl`

## Run Policy

- Use `3` clean runs per `(workload, mode)` at minimum.
- Use `5` clean runs for Tier 1 if runtime remains practical.
- Keep parser path fixed across all modes.
- Prefer `reason_time_ms` plus emitted scheduler telemetry for interpretation.

## Expected Pattern

- Branch-light workloads:
  - `sequential ~= adaptive`
  - `always_parallel` should regress
- Branch-heavy workloads:
  - `adaptive` should beat `sequential`
  - `adaptive` should beat `always_parallel`
- Inconsistent branch-heavy workloads:
  - `adaptive + nogood` should beat `adaptive without nogood`

## Acceptance Criteria

- A main ablation table can report wall or reason time for:
  - `sequential`
  - `adaptive`
  - `always_parallel`
  - `adaptive_no_nogood`
- A telemetry table can report:
  - `used_parallel`
  - `work_items_expanded`
  - `steal_successes`
  - `branches_pruned`
  - `nogood_hits`

## Immediate Follow-Up

1. Add a lightweight runner script that loops over the mode matrix.
2. Capture CSV + raw `[spacl]` lines into an artifact directory.
3. Patch the manuscript with one `Speculative Scheduling Ablation` subsection if the expected pattern holds.
