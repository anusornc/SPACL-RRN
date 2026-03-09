# Repository Directory Structure

This document describes the current top-level organization of the Tableauxx repository.

## Top-level layout

```text
.
├── src/                # Rust library and internal modules
├── tests/              # Rust tests and ontology test data
├── benches/            # Criterion benchmarks
├── benchmarks/         # Head-to-head benchmark harness and datasets
├── scripts/            # Utility scripts (benchmarking, validation, profiling)
├── docs/               # Active technical documentation
├── paper/              # Submission manuscript and paper assets
├── specs/              # Interface/schema specs (including blockchain-related)
├── results/            # Historical benchmark/result artifacts
├── admin/              # Project management notes and revision logs
├── assets/             # Static diagrams/images
└── archive/            # Archived non-active materials
```

## Key directories

### `src/`

Main Rust codebase:

- `src/core/`
- `src/logic/`
- `src/parser/`
- `src/reasoner/`
- `src/strategy/`
- `src/serializer/`
- `src/storage/`
- `src/util/`
- `src/bin/` (CLI binaries)

### `benchmarks/`

Benchmark infrastructure:

- `benchmarks/competitors/` - competitor harness (`run_benchmarks.sh`) and run history
- `benchmarks/competitors/scripts/run_spacl_ablation.sh` - SPACL scheduler/policy ablations
- `benchmarks/competitors/scripts/run_rrn_policy_protocol.sh` - locked RRN policy protocol
- `benchmarks/external/owl2bench/` - external OWL2Bench wrapper
- `benchmarks/ontologies/` - benchmark ontology sets

### `docs/`

Current active documentation and benchmark runbook.
Legacy planning/status docs are moved under `docs/archive/`.

### `paper/`

Submission files and build script:

- `paper/submission/manuscript.tex`
- `paper/submission/compile.sh`
- `paper/submission/export_docx.sh`
- `paper/submission_rrn/manuscript.tex`
- `paper/submission_rrn/compile.sh`
- `paper/submission_rrn/export_docx.sh`

## Generated artifacts

Not part of source architecture:

- `target/` - Cargo build output
- benchmark result directories under `benchmarks/competitors/results/history/`

## Maintenance rule

- keep active docs in `docs/`
- move stale planning/status snapshots into `docs/archive/`
- keep reproducibility artifacts in benchmark history directories, not in root docs
