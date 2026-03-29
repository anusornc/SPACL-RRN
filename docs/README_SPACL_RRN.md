# SPACL-RRN: Hybrid RRN Policy for OWL2 Reasoning

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

This repository contains the **Hybrid RRN (Neural Related Reasoning)** extension for the SPACL OWL reasoner. It implements learned branch-priority policies using machine learning models.

> **Research Track:** Advanced policy learning for speculative parallel reasoning

---

## Quick Start

### Option 1: Use Pre-trained Models (Recommended)

No training required! Pre-trained models are included and ready to use.

```bash
# Clone the repository
git clone https://github.com/anusornc/SPACL-RRN.git
cd SPACL-RRN

# Build
cargo build --release

# Run with pre-trained GBDT model
SPACL_BRANCH_POLICY=hybrid_rrn \
SPACL_RRN_MODEL_PATH=benchmarks/models/rrn_gbdt_stump_model_v2.json \
cargo run --bin owl2-reasoner -- check tests/data/univ-bench.owl
```

### Option 2: Train Custom Model

See [Training Guide](docs/TRAINING_GUIDE.md) for detailed instructions.

---

## What is RRN?

**Hybrid RRN** (Neural Related Reasoning) is a learned branch-priority policy system that improves SPACL's parallel reasoning performance by intelligently ordering branch expansions using machine learning models.

### Key Features

- **Pre-trained Models**: 5 models trained on 294K+ samples, ready to use
- **Training Pipeline**: Complete tools to train custom models on your ontologies
- **Model Types**: Linear regression and GBDT-stump models
- **Fallback Safety**: Gracefully falls back to heuristic if no model available
- **Reproducible**: Locked benchmark protocols for research reproducibility

### Performance

From our evaluation (median times, 5 repeats):

| Policy | mixed_operands_8 | mixed_operands_16 |
|--------|------------------|-------------------|
| Baseline (ontology order) | 4064 ms | 6262 ms |
| **Hybrid RRN (Linear v3)** | **2331 ms** | **4012 ms** |
| Hybrid RRN (GBDT v2) | 10576 ms | 7139 ms |

**Speedup:** Up to **1.7x** faster than baseline with linear pairwise model.

---

## Installation

### Prerequisites

- Rust toolchain (stable, 1.84+)
- Docker Engine (for benchmarking)
- `jq`
- GNU `timeout`

### Build

```bash
cargo build --release
```

### Test

```bash
cargo test
```

---

## Usage

### Basic Reasoning

```bash
# Consistency check
cargo run --bin owl2-reasoner -- check ontology.owl

# Classification
cargo run --bin owl2-reasoner -- classify ontology.owl

# With RRN model
SPACL_BRANCH_POLICY=hybrid_rrn \
SPACL_RRN_MODEL_PATH=benchmarks/models/rrn_gbdt_stump_model_v2.json \
cargo run --bin owl2-reasoner -- classify ontology.owl
```

### Model Options

| Model | Training Samples | Accuracy | Best For |
|-------|------------------|----------|----------|
| `rrn_linear_model_v3_pairwise.json` | 294,495 | 65.6% | General use (recommended) |
| `rrn_gbdt_stump_model_v2.json` | 294,495 | 65.6% | Alternative comparator |
| Custom trained | Your data | Varies | Domain-specific optimization |

---

## Training Custom Models

### Step 1: Generate Training Data

```bash
# From synthetic workloads
SPACL_BRANCH_POLICY=heuristic \
SPACL_BRANCH_SNAPSHOT_FILE=/tmp/snapshots.jsonl \
SPACL_SYNTH_ABLATION_WORKLOADS='mixed_operands_16,mixed_operands_32' \
SPACL_SYNTH_ABLATION_REPEATS=5 \
cargo run --bin run_spacl_synthetic_ablation

# Or from real ontologies
SPACL_BRANCH_SNAPSHOT_DIR=/tmp/snapshots/ \
SPACL_BRANCH_POLICY=heuristic \
cargo run --bin owl2-reasoner -- \
  --input my_ontology.owl \
  --classify \
  --export-snapshots
```

### Step 2: Train Model

```bash
# Linear model
cargo run --bin train_rrn_linear_model -- \
  /tmp/snapshots.jsonl \
  my_model.json \
  heuristic

# GBDT stump model (recommended)
cargo run --bin train_rrn_gbdt_model -- \
  /tmp/snapshots.jsonl \
  my_model.json \
  heuristic
```

### Step 3: Use Trained Model

```bash
SPACL_BRANCH_POLICY=hybrid_rrn \
SPACL_RRN_MODEL_PATH=my_model.json \
cargo run --bin owl2-reasoner -- classify ontology.owl
```

---

## Documentation

- [Training Guide](docs/TRAINING_GUIDE.md) - Complete training pipeline
- [Protocol Lock](docs/RRN_PROTOCOL_LOCK.md) - Reproducible benchmark protocol
- [Model Comparison](docs/RRN_MODEL_COMPARISON.md) - Evaluation results
- [Architecture](docs/MODULE_ARCHITECTURE.md) - System architecture

---

## Benchmarking

### Smoke Test

```bash
RUN_ID=smoke_$(date +%Y%m%d_%H%M%S) \
ONTOLOGY_SUITE=standard \
ONTOLOGY_REGEX='^disjunctive_simple\.owl$' \
REASONERS_OVERRIDE=spacl \
TIMEOUT_SECONDS=60 \
./benchmarks/competitors/scripts/run_benchmarks.sh all
```

### Full RRN Policy Evaluation

```bash
RUN_ID=rrn_eval_$(date +%Y%m%d_%H%M%S) \
RRN_MODEL_PATH=benchmarks/models/rrn_linear_model_v3_pairwise.json \
MODE_MATRIX='adaptive|1|baseline,adaptive|1|heuristic,adaptive|1|hybrid_rrn' \
WORKLOADS='mixed_operands_16,mixed_operands_32' \
REPEATS=5 \
./benchmarks/competitors/scripts/run_rrn_policy_protocol.sh
```

See [Benchmark Runbook](docs/benchmarking/BENCHMARK_RUNBOOK.md) for details.

---

## Repository Structure

```
SPACL-RRN/
├── src/
│   ├── reasoner/
│   │   └── speculative.rs      # SPACL + RRN policy integration
│   └── bin/
│       └── owl2-reasoner.rs    # Main CLI with RRN support
├── scripts/
│   ├── train_rrn_linear_model.rs
│   ├── train_rrn_gbdt_model.rs
│   └── run_spacl_synthetic_ablation.rs
├── benchmarks/
│   └── models/                 # Pre-trained models
├── docs/                       # Documentation
└── paper/                      # Paper submission artifacts
```

---

## Citation

If you use this software in your research, please cite:

```bibtex
@software{spacl_rrn,
  author = {Author Name},
  title = {SPACL-RRN: Hybrid Neural Related Reasoning for OWL2},
  year = {2026},
  url = {https://github.com/anusornc/SPACL-RRN},
  version = {1.0.0}
}
```

---

## License

MIT License - See [LICENSE](LICENSE) file.

---

## Related Repositories

- **SPACL (Main Algorithm)**: [github.com/anusornc/SPACL](https://github.com/anusornc/SPACL)

---

## Support

- Issues: [GitHub Issues](https://github.com/anusornc/SPACL-RRN/issues)
- Documentation: [docs/](docs/)
