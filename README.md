# SPACL (Tableauxx Repository)

This repository contains the SPACL OWL reasoner implementation and benchmarking toolkit in Rust.
Repository/package names still use `tableauxx` in some paths and scripts for historical compatibility.
It supports multi-format ontology loading, profile-aware reasoning, and reproducible head-to-head benchmarking.

## Prerequisites

- Rust toolchain (stable; tested with Rust `1.84+`)
- Docker Engine (`docker` CLI must be usable by your user)
- `jq`
- GNU `timeout` (usually from `coreutils`)

Quick check:

```bash
cargo --version
docker --version
jq --version
timeout --version | head -n 1
```

## Current benchmark position (latest)

From the latest clean OWL2Bench core 3-run aggregate in this repository:

- Run set: `owl2bench_univ_core_clean_20260223`, `_r2`, `_r3`
- Aggregate: `benchmarks/competitors/results/history/owl2bench_univ_core_clean_aggregate_20260223`
- SPACL/Tableauxx: `12/12` success, median wall time `2107.5 ms`, P95 `2707 ms`
- In this harness, SPACL/Tableauxx is median winner on all 4 core profiles (`DL`, `EL`, `QL`, `RL`)

See details in `docs/benchmarking/BENCHMARK_RUNBOOK.md`.

## Quick start

```bash
# Build
cargo build --release

# Run tests
cargo test

# Check consistency
cargo run --bin owl2-reasoner -- check tests/data/univ-bench.owl

# Auto-select reasoner by ontology profile
cargo run --bin owl2-reasoner -- check-auto tests/data/univ-bench.owl

# Convert ontology to binary cache format
cargo run --bin owl2-reasoner -- convert tests/data/univ-bench.owl /tmp/univ-bench.owlbin
```

## Benchmark smoke test (recommended first)

Before full competitor runs, verify your environment with a one-ontology smoke run:

```bash
RUN_ID=smoke_$(date +%Y%m%d_%H%M%S) \
ONTOLOGY_SUITE=standard \
ONTOLOGY_REGEX='^disjunctive_simple\.owl$' \
REASONERS_OVERRIDE=tableauxx \
TIMEOUT_SECONDS=60 \
SKIP_BUILD=0 \
benchmarks/competitors/scripts/run_benchmarks.sh all
```

Verify smoke status:

```bash
run_dir="$(readlink -f benchmarks/competitors/results/latest)"
jq -r '.status' "$run_dir"/*.json | sort | uniq -c
```

Expected: at least one `success` row for `tableauxx`.

If Docker permission fails (e.g., `permission denied while trying to connect to the docker API socket`), add your user to Docker group and re-login:

```bash
sudo usermod -aG docker "$USER"
# then log out/login (or reboot) before retrying
```

## Main CLIs

- `owl2-reasoner`: primary CLI for loading, checking, profiling, and converting ontologies
- `owl2_validation`: lightweight validation/check/stats CLI
- `epcis-reasoner`: EPCIS/traceability-oriented demo CLI
- `train_rrn_linear_model`: offline utility to fit a first linear hybrid policy from snapshot JSONL
- `train_rrn_gbdt_model`: offline utility to fit a GBDT-stump hybrid policy from snapshot JSONL

Run help:

```bash
cargo run --bin owl2-reasoner -- help
cargo run --bin owl2_validation -- help
cargo run --bin epcis-reasoner -- help
cargo run --bin train_rrn_linear_model -- --help
cargo run --bin train_rrn_gbdt_model -- --help
```

## Hybrid RRN Model Training & Usage

The `exp/hybrid-rrn-paper` branch includes a complete training pipeline for learned branch-priority policies.

### Quick Start: Use Pre-trained Models

Pre-trained models are ready to use out of the box (no training required):

```bash
# Linear model (trained on 294K samples, 65.6% pairwise accuracy)
SPACL_BRANCH_POLICY=hybrid_rrn \
SPACL_RRN_MODEL_PATH=benchmarks/models/rrn_linear_model_v3_pairwise.json \
cargo run --bin owl2-reasoner -- check tests/data/univ-bench.owl

# GBDT stump model (alternative, non-neural comparator)
SPACL_BRANCH_POLICY=hybrid_rrn \
SPACL_RRN_MODEL_PATH=benchmarks/models/rrn_gbdt_stump_model_v2.json \
cargo run --bin owl2-reasoner -- check tests/data/univ-bench.owl
```

### Full Training Pipeline (3 Steps)

**Step 1: Generate Training Data**

Generate snapshots from synthetic workloads:

```bash
SPACL_BRANCH_POLICY=heuristic \
SPACL_BRANCH_SNAPSHOT_FILE=/tmp/snapshots.jsonl \
SPACL_SYNTH_ABLATION_WORKLOADS='mixed_operands_16,mixed_operands_32' \
SPACL_SYNTH_ABLATION_MODES='adaptive' \
SPACL_SYNTH_ABLATION_REPEATS=5 \
cargo run --bin run_spacl_synthetic_ablation
```

Or from real ontologies:

```bash
SPACL_BRANCH_SNAPSHOT_DIR=/tmp/snapshots/ \
SPACL_BRANCH_POLICY=heuristic \
cargo run --bin owl2-reasoner -- \
  --input my_ontology.owl \
  --classify \
  --export-snapshots
```

**Step 2: Train Custom Model**

Train a linear model:

```bash
cargo run --bin train_rrn_linear_model -- \
  /tmp/snapshots.jsonl \
  benchmarks/models/my_custom_model.json \
  heuristic
```

Or train a GBDT stump model (more powerful):

```bash
cargo run --bin train_rrn_gbdt_model -- \
  /tmp/snapshots.jsonl \
  benchmarks/models/my_custom_gbdt_model.json \
  heuristic
```

**Step 3: Use Trained Model**

```bash
SPACL_BRANCH_POLICY=hybrid_rrn \
SPACL_RRN_MODEL_PATH=benchmarks/models/my_custom_model.json \
cargo run --bin owl2-reasoner -- check my_ontology.owl
```

### Model Options Summary

| Option | Training Required | Best For |
|--------|-------------------|----------|
| Pre-trained linear model | No | Quick start, general use |
| Pre-trained GBDT model | No | Alternative heuristic |
| Custom trained model | Yes | Domain-specific optimization |
| No model (fallback) | No | Safe default, uses heuristic |

### Additional Resources

- Training protocol: `docs/experiments/RRN_PROTOCOL_LOCK_20260309.md`
- Model comparison: `docs/experiments/RRN_MODEL_COMPARATOR_20260310.md`
- Branch architecture: `docs/MODULE_ARCHITECTURE_EXP_HYBRID_RRN.md`
- Branch comparison: `docs/BRANCH_COMPARISON.md`

## Benchmarking

Primary benchmark harness:

- `benchmarks/competitors/scripts/run_benchmarks.sh`
- `benchmarks/competitors/scripts/run_spacl_ablation.sh`
- `benchmarks/competitors/scripts/run_rrn_policy_protocol.sh` (locked RRN/branch-policy matrix)

External OWL2Bench wrapper:

- `benchmarks/external/owl2bench/prepare.sh`
- `benchmarks/external/owl2bench/run.sh`
- `benchmarks/external/owl2bench/report.sh`

Operational benchmark guide:

- `docs/benchmarking/BENCHMARK_RUNBOOK.md`
- `docs/experiments/RRN_PROTOCOL_LOCK_20260309.md`

## Documentation map

Start here:

- `docs/README.md`
- `docs/QUICK_START.md`
- `docs/BRANCH_WORKFLOW.md`
- `docs/PROJECT_STRUCTURE.md`
- `docs/DIRECTORY_STRUCTURE.md`

Architecture documentation:

- `docs/MODULE_ARCHITECTURE_MAIN.md` - Main branch architecture
- `docs/MODULE_ARCHITECTURE_EXP_HYBRID_RRN.md` - Experimental branch (with RRN)
- `docs/BRANCH_COMPARISON.md` - Side-by-side branch comparison

Domain and deployment docs:

- `docs/BLOCKCHAIN_TRANSACTION_PROFILE_GUIDE.md`

RRN Hybrid documentation:

- `docs/experiments/RRN_PROTOCOL_LOCK_20260309.md` - Training protocol
- `docs/experiments/RRN_MODEL_COMPARATOR_20260310.md` - Model comparison results
- `docs/experiments/RRN_HYBRID_TASKLIST.md` - Implementation tasklist

## Paper

Submission workspace:

- `paper/submission/manuscript.tex`
- `paper/submission/compile.sh`
- `paper/submission/export_docx.sh`
- `paper/submission_rrn/manuscript.tex` (separate hybrid RRN paper track)
- `paper/submission_rrn/compile.sh`
- `paper/submission_rrn/export_docx.sh`

Build PDF:

```bash
cd paper/submission
./compile.sh
```

Export DOCX:

```bash
cd paper/submission
./export_docx.sh
```

Optional output name:

```bash
cd paper/submission
./export_docx.sh manuscript_for_review.docx
```

## License

- Code: MIT (`LICENSE`)
