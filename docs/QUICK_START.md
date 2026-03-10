# Quick Start

## 0. Prerequisites

Required tools:

- Rust toolchain (stable; tested with Rust `1.84+`)
- Docker Engine (`docker` CLI usable by your account)
- `jq`
- GNU `timeout` (from `coreutils`)

Quick check:

```bash
cargo --version
docker --version
jq --version
timeout --version | head -n 1
```

## 1. Build and test

```bash
cargo build --release
cargo test
```

## 2. Run the main reasoner CLI

```bash
# Consistency check
cargo run --bin owl2-reasoner -- check tests/data/univ-bench.owl

# Auto profile-aware reasoner selection
cargo run --bin owl2-reasoner -- check-auto tests/data/univ-bench.owl

# Stats
cargo run --bin owl2-reasoner -- stats tests/data/univ-bench.owl

# Convert to binary format
cargo run --bin owl2-reasoner -- convert tests/data/univ-bench.owl /tmp/univ-bench.owlbin
```

## 3. Alternative CLIs

```bash
cargo run --bin owl2_validation -- help
cargo run --bin epcis-reasoner -- help
```

## 4. Large ontology loading options

Environment controls used by shared loader:

- `OWL2_REASONER_LARGE_PARSE=1` - enable large parse mode
- `OWL2_REASONER_AUTO_CACHE=1` - write `.owlbin` cache after text parse
- `OWL2_REASONER_FORCE_TEXT=1` - force text parsing even if `.owlbin` exists
- `OWL2_REASONER_BIN_ONLY=1` - require `.owlbin`
- `OWL2_REASONER_MAX_FILE_SIZE=<bytes>` - override file-size threshold
- `OWL2_REASONER_STAGE_TIMING=1` - emit parse/reason stage timing in benchmark runs

Example:

```bash
OWL2_REASONER_LARGE_PARSE=1 OWL2_REASONER_AUTO_CACHE=1 \
cargo run --bin owl2-reasoner -- check benchmarks/ontologies/other/go-basic.owl
```

SPACL policy controls (for ablations/hybrid track):

- `SPACL_SCHED_MODE=adaptive|sequential|always_parallel`
- `SPACL_BRANCH_POLICY=baseline|heuristic|hybrid_rrn`
- `SPACL_RRN_MODEL_PATH=/path/to/model.json` (optional)
- `SPACL_BRANCH_SNAPSHOT_FILE=/path/to/branch_snapshots.jsonl` (optional training export)

## 5. Run benchmark harness

### 5.1 Minimal smoke benchmark (recommended first)

```bash
RUN_ID=smoke_$(date +%Y%m%d_%H%M%S) \
ONTOLOGY_SUITE=standard \
ONTOLOGY_REGEX='^disjunctive_simple\.owl$' \
REASONERS_OVERRIDE=tableauxx \
TIMEOUT_SECONDS=60 \
SKIP_BUILD=0 \
benchmarks/competitors/scripts/run_benchmarks.sh all
```

Verify smoke result:

```bash
run_dir="$(readlink -f benchmarks/competitors/results/latest)"
jq -r '.status' "$run_dir"/*.json | sort | uniq -c
```

Expected: at least one `success` row.

### 5.2 Core competitor harness

```bash
# Small suite example
RUN_ID=small_workload_suite_real_YYYYMMDD \
ONTOLOGY_SUITE=standard \
REASONERS_OVERRIDE=tableauxx,hermit,konclude,openllet,elk,jfact,pellet \
TIMEOUT_SECONDS=180 \
SKIP_BUILD=0 \
benchmarks/competitors/scripts/run_benchmarks.sh all
```

### 5.3 Locked RRN policy protocol

```bash
RUN_ID=rrn_policy_r1 \
REPEATS=3 \
TIMEOUT_SECONDS=900 \
SKIP_BUILD=0 \
benchmarks/competitors/scripts/run_rrn_policy_protocol.sh
```

If this run produces `branch_snapshots_written=0`, switch to branch-active synthetic
workloads for training export:

```bash
SPACL_BRANCH_POLICY=heuristic \
SPACL_BRANCH_SNAPSHOT_FILE=benchmarks/competitors/results/history/rrn_synth_snapshot_r1/branch_snapshots.jsonl \
SPACL_SYNTH_ABLATION_WORKLOADS='mixed_operands_16,mixed_operands_32,reused_conflict_12' \
SPACL_SYNTH_ABLATION_MODES='adaptive' \
SPACL_SYNTH_ABLATION_REPEATS=3 \
SPACL_SYNTH_ABLATION_WARMUPS=1 \
SPACL_SYNTH_PARALLEL_THRESHOLD=4 \
SPACL_SYNTH_COST_THRESHOLD_US=1 \
cargo run --quiet --bin run_spacl_synthetic_ablation
```

Train a first linear RRN policy file from exported snapshots:

```bash
cargo run --bin train_rrn_linear_model -- \
  benchmarks/competitors/results/history/<RUN_ID>/branch_snapshots.jsonl \
  benchmarks/models/rrn_linear_model.json \
  heuristic
```

Train with pairwise ranking objective (recommended for branch ordering):

```bash
RRN_TRAIN_OBJECTIVE=pairwise \
RRN_TRAIN_EPOCHS=30 \
RRN_PAIRWISE_MAX_PAIRS_PER_RECORD=128 \
cargo run --bin train_rrn_linear_model -- \
  benchmarks/models/rrn_train_subset_r2_100k.jsonl \
  benchmarks/models/rrn_linear_model_v3_pairwise.json \
  heuristic
```

### OWL2Bench wrapper

```bash
OWL2BENCH_SOURCE_DIR=/tmp/owl2bench/OWL2Bench \
benchmarks/external/owl2bench/prepare.sh

RUN_ID=owl2bench_univ_core_example \
TIMEOUT_SECONDS=300 \
SKIP_BUILD=1 \
REASONERS_OVERRIDE=tableauxx,hermit,konclude,openllet,elk,jfact,pellet \
benchmarks/external/owl2bench/run.sh all
```

For authoritative benchmark commands and run IDs, use:

- `docs/benchmarking/BENCHMARK_RUNBOOK.md`

## 6. Troubleshooting (common)

Docker permission error:

```text
permission denied while trying to connect to the docker API socket
```

Fix:

```bash
sudo usermod -aG docker "$USER"
# log out/login (or reboot), then retry
```

## 7. Build paper PDF

```bash
cd paper/submission
./compile.sh
```

## 8. Export paper DOCX

```bash
cd paper/submission
./export_docx.sh
```

Optional custom output file:

```bash
cd paper/submission
./export_docx.sh manuscript_for_review.docx
```

Note:
- DOCX export uses Pandoc (Docker image `pandoc/core` if Docker is available).
- Review equations/algorithm formatting in Word after export, because some LaTeX math macros can remain as TeX.

## 9. Build hybrid RRN paper track

```bash
cd paper/submission_rrn
./compile.sh
```
