# Tableauxx

Tableauxx is an OWL2 reasoner library and CLI toolkit implemented in Rust.
It supports multi-format ontology loading, profile-aware reasoning, and reproducible head-to-head benchmarking.

## Current benchmark position (latest)

From the latest clean OWL2Bench core 3-run aggregate in this repository:

- Run set: `owl2bench_univ_core_clean_20260223`, `_r2`, `_r3`
- Aggregate: `benchmarks/competitors/results/history/owl2bench_univ_core_clean_aggregate_20260223`
- Tableauxx: `12/12` success, median wall time `2107.5 ms`, P95 `2707 ms`
- In this harness, Tableauxx is median winner on all 4 core profiles (`DL`, `EL`, `QL`, `RL`)

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

## Main CLIs

- `owl2-reasoner`: primary CLI for loading, checking, profiling, and converting ontologies
- `owl2_validation`: lightweight validation/check/stats CLI
- `epcis-reasoner`: EPCIS/traceability-oriented demo CLI

Run help:

```bash
cargo run --bin owl2-reasoner -- help
cargo run --bin owl2_validation -- help
cargo run --bin epcis-reasoner -- help
```

## Benchmarking

Primary benchmark harness:

- `benchmarks/competitors/scripts/run_benchmarks.sh`

External OWL2Bench wrapper:

- `benchmarks/external/owl2bench/prepare.sh`
- `benchmarks/external/owl2bench/run.sh`
- `benchmarks/external/owl2bench/report.sh`

Operational benchmark guide:

- `docs/benchmarking/BENCHMARK_RUNBOOK.md`

## Documentation map

Start here:

- `docs/README.md`
- `docs/QUICK_START.md`
- `docs/PROJECT_STRUCTURE.md`
- `docs/DIRECTORY_STRUCTURE.md`

Domain and deployment docs:

- `docs/BLOCKCHAIN_TRANSACTION_PROFILE_GUIDE.md`

## Paper

Submission workspace:

- `paper/submission/manuscript.tex`
- `paper/submission/compile.sh`

Build PDF:

```bash
cd paper/submission
./compile.sh
```

## License

- Code: MIT (`LICENSE`)
