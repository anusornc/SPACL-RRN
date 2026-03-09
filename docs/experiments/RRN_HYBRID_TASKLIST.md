# RRN Hybrid Track Tasklist

Status: active  
Branch: `exp/hybrid-rrn-paper`  
Last updated: 2026-03-09

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
- [ ] Train first RRN baseline (offline)
- [x] Add model loading/inference wrapper with fail-safe fallback

### D. Experiments and validation

- [ ] Run SPACL baseline (same hardware, same timeout policy)
- [ ] Run SPACL+RRN hybrid with identical protocol
- [ ] Report median, P95, variance, and failure-aware metrics
- [ ] Add ablation: `SPACL + heuristic-ranked` vs `SPACL + RRN`

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

1. Run first locked protocol set and archive run IDs in this tasklist.
2. Train and validate first RRN weight file on exported snapshots.
3. Wire benchmark report aggregation for policy-level statistics (median/P95/failure-aware).
