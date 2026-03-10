# RRN Hybrid Track Tasklist

Status: active  
Branch: `exp/hybrid-rrn-paper`  
Last updated: 2026-03-09 (evening)

## Goal

Build and evaluate a neural-symbolic extension where an RRN policy guides SPACL branch priority, while final consistency decisions remain symbolic (`SimpleReasoner`/SPACL) for correctness.

## Scope guardrails

- RRN is guidance-only (ranking/prioritization), not final entailment authority.
- Keep existing SPACL path unchanged when hybrid mode is disabled.
- All claims must be backed by reproducible scripts and committed artifacts.

## Deliverables

1. Hybrid runtime mode (`SPACL_BRANCH_POLICY=hybrid_rrn`) with telemetry.
2. Training/inference data pipeline for branch-level policy signals.
3. Benchmark matrix with fixed seeds and repeated runs.
4. Separate manuscript track for the hybrid study (`paper/submission_rrn`).

## Execution checklist

### A. Branching and planning

- [x] Create dedicated branch: `exp/hybrid-rrn-paper`
- [x] Create this tasklist document
- [x] Freeze baseline commit hash for pre-hybrid comparison

### B. Runtime integration (non-invasive first)

- [x] Add policy mode enum: `baseline` (default), `heuristic`, `hybrid_rrn`
- [x] Add env flag parsing in CLI (`SPACL_BRANCH_POLICY`)
- [x] Add telemetry fields for policy decision counts and fallback counts
- [x] Keep deterministic behavior when policy is not enabled

### C. Data and model pipeline

- [x] Define branch-decision training schema (`jsonl` branch snapshots)
- [x] Export branch snapshots from benchmark runs
- [x] Add offline trainer utility (`train_rrn_linear_model`)
- [x] Train first RRN baseline (offline)
- [x] Add model loading/inference wrapper with fail-safe fallback

### D. Experiments and validation

- [x] Run SPACL baseline (same hardware, same timeout policy)
- [x] Run SPACL+RRN hybrid with identical protocol
- [x] Report median, P95, variance, and failure-aware metrics
- [x] Add ablation: `SPACL + heuristic-ranked` vs `SPACL + RRN`

### E. Paper split and writing

- [x] Initialize separate paper directory (`paper/submission_rrn`)
- [x] Set title/abstract for neural-symbolic positioning
- [ ] Add methodology for guidance-only safety boundary
- [ ] Add dedicated limitations section for model dependence
- [ ] Add reproducibility appendix mapped to public repo paths

## Risks and mitigations

- Risk: RRN adds overhead and increases variance.
  - Mitigation: keep strict fallback and measure per-stage overhead.
- Risk: reviewer questions symbolic correctness.
  - Mitigation: enforce symbolic final decision; RRN only ranks branches.
- Risk: benchmark drift between branches.
  - Mitigation: lock harness scripts and timeout policy before runs.

## Immediate next actions

1. Build a stronger policy model (pairwise/ranking loss or richer model than linear regression) from the mixed-operand snapshot set.
2. Re-run the same policy matrix and check whether hybrid beats heuristic on at least 2/3 workloads.
3. Add median/P95/variance/failure-aware aggregation script and commit publication-ready tables.

## Run archive (2026-03-09)

- `rrn_snapshot_seed_20260309_r1`:
  - default `run_rrn_policy_protocol.sh` sanity check
  - no branch activity (`used_parallel=false`, `branches_created=0`)
- `spacl_synthetic_ablation_20260309_162326` + `rrn_synth_snapshot_20260309_r1/branch_snapshots.jsonl`:
  - first branch-active snapshot export (`328743` rows)
- `spacl_synthetic_ablation_20260309_163120` + `rrn_synth_snapshot_20260309_r2/branch_snapshots.jsonl`:
  - mixed-operand branch-active snapshot export (`277661` rows)
  - `policy_reordered_splits=75267`
- Models:
  - `benchmarks/models/rrn_linear_model_v1.json`
  - `benchmarks/models/rrn_linear_model_v2.json`
- Policy matrix runs:
  - baseline: `spacl_synthetic_ablation_20260309_163617`
  - heuristic: `spacl_synthetic_ablation_20260309_164013`
  - hybrid + model: `spacl_synthetic_ablation_20260309_164224`
  - hybrid fallback: `spacl_synthetic_ablation_20260309_164507`

## Current status summary

- Runtime and telemetry wiring are complete and validated.
- Branch-policy dataset pipeline is active with large JSONL exports.
- Hybrid mode correctly switches between model and fallback paths.
- Current linear model (`v2`) does not yet outperform heuristic policy on the latest mixed-operand matrix.
- Strict 2026-03-10 matrix and summary were generated from raw CSV artifacts only (no hardcoded/fake metric rows), using `benchmarks/competitors/scripts/summarize_policy_matrix.sh`.
- Pairwise-trained `v3` model shows improved direction on mixed workloads (repeat-matched win count `9/10` against heuristic on `mixed_operands_8,16`), but variance is still high and requires another confirmation pass.
- Second confirmation pass on a different CPU set (`taskset -c 12-23`) kept the same `9/10` repeat-matched win direction for hybrid-v3 vs heuristic on `mixed_operands_8,16`.
