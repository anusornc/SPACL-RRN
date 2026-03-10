# RRN Reviewer Diff (2026-03-10)

This note summarizes what changed for reviewer-facing hybrid-policy updates and where to verify each claim.

## Scope

- Branch: `exp/hybrid-rrn-paper`
- Focus: hybrid policy evidence, reproducibility, and conservative claim boundaries.

## Manuscript Changes

- Updated hybrid results narrative and added confirmation-pass evidence.
  - File: `paper/submission_rrn/manuscript.tex`
  - Added subsection: `Hybrid Branch-Prioritization Results` (`\label{sec:hybrid-results}`)
  - Added table: `tab:hybrid-confirmation` (median/P95 from raw runs)
- Expanded limitations with explicit hybrid-study boundaries.
  - File: `paper/submission_rrn/manuscript.tex`
  - Section: `\subsection{Limitations}`
- Updated conclusion to reflect bounded positive hybrid direction.
  - File: `paper/submission_rrn/manuscript.tex`

## Runtime / Tooling Changes

- Added mixed workload generator for policy-signal runs:
  - `mixed_operands_8` in `scripts/run_spacl_synthetic_ablation.rs`
- Extended trainer with pairwise ranking objective:
  - File: `scripts/train_rrn_linear_model.rs`
  - New envs:
    - `RRN_TRAIN_OBJECTIVE=regression|pairwise`
    - `RRN_PAIRWISE_MAX_PAIRS_PER_RECORD=<n>`
- Added deterministic matrix summarizer:
  - File: `benchmarks/competitors/scripts/summarize_policy_matrix.sh`
  - Computes mean/median/P95/Q1/Q3/IQR directly from raw CSVs.

## Evidence Artifacts (Raw Runs)

- Pass A (CPU set `0-11`)
  - Heuristic: `benchmarks/competitors/results/history/spacl_synthetic_ablation_20260310_063022/results.csv`
  - Hybrid v3: `benchmarks/competitors/results/history/spacl_synthetic_ablation_20260310_063405/results.csv`
- Pass B (CPU set `12-23`)
  - Heuristic: `benchmarks/competitors/results/history/spacl_synthetic_ablation_20260310_064158/results.csv`
  - Hybrid v3: `benchmarks/competitors/results/history/spacl_synthetic_ablation_20260310_064702/results.csv`

## Model Artifact

- Pairwise-trained model used in manuscript updates:
  - `benchmarks/models/rrn_linear_model_v3_pairwise.json`

## Integrity Notes

- No fake rows, no manual timing injection, no hardcoded benchmark outputs.
- All reported metrics are traceable to the run artifacts above.
- Train subsets (`benchmarks/models/*.jsonl`) are local-only and ignored from git.
