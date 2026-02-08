# Tableauxx - OWL2 DL Reasoner

A high-performance, feature-complete OWL2 reasoning engine implemented in Rust.

[![GitHub](https://img.shields.io/badge/GitHub-anusornc%2Ftableauxx-blue)](https://github.com/anusornc/tableauxx)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

> **Paper Status**: Ready for final verification before Journal of Web Semantics submission.  
> **Repository**: https://github.com/anusornc/tableauxx

---

## 📁 Project Structure

```
tableauxx/
├── src/           # Rust source code
├── tests/         # Test suite
├── paper/         # Journal submission
│   ├── submission/    # Manuscript files
│   ├── guides/        # Reference guides
│   └── downloads/     # Downloaded PDFs
├── admin/         # Project administration
├── docs/          # Developer documentation
└── benchmarks/    # Benchmark data
```

**📖 [Full Project Structure](PROJECT_STRUCTURE.md)**

## 🚀 Quick Start

```bash
# Build the library
cargo build --lib

# Run tests
cargo test --lib

# Run benchmarks
cargo bench
```

## 🖥️ Demo Applications

### 1. OWL2 Reasoner CLI

General-purpose ontology reasoning tool:

```bash
# Check ontology consistency
cargo run --bin owl2-reasoner -- check tests/data/univ-bench.owl

# Show ontology statistics
cargo run --bin owl2-reasoner -- stats tests/data/univ-bench.owl

# Compare Sequential vs SPACL performance
cargo run --bin owl2-reasoner -- compare tests/data/univ-bench.owl
```

### 2. EPCIS Supply Chain Reasoner

Demo for GS1 EPCIS supply chain tracking:

```bash
# Generate and verify a demo supply chain
cargo run --bin epcis-reasoner -- generate-demo

# Check EPCIS ontology consistency
cargo run --bin epcis-reasoner -- check-consistency

# Show EPCIS statistics
cargo run --bin epcis-reasoner -- stats
```

The EPCIS demo tracks a pharmaceutical product through the supply chain:
- **Manufacturing** at Factory A
- **Shipping** to Warehouse B  
- **Receiving** at Hospital C

Verifies logical consistency of the trace using OWL2 reasoning.

## 📁 Project Structure

This project has been organized into a clean, modular structure:

```
├── docs/          # Documentation (see docs/README.md)
├── src/           # Source code (see PROJECT_STRUCTURE.md)
├── benches/       # Benchmarks
├── scripts/       # Python scripts
├── assets/        # Images and diagrams
├── results/       # Benchmark results
└── tests/data/    # Test ontologies
```

**📖 Full Documentation**: See [`docs/README.md`](docs/README.md)

**🏗️ Source Structure**: See [`DIRECTORY_STRUCTURE.md`](DIRECTORY_STRUCTURE.md)

## ✨ Key Features

- **SPACL Algorithm**: Novel Speculative Parallel Tableaux with Adaptive Conflict Learning
- **Complete OWL2 DL**: Full SROIQ(D) description logic support
- **Multi-format Parsing**: Turtle, RDF/XML, OWL/XML, JSON-LD
- **Profile Optimization**: Automatic EL/QL/RL optimizations
- **Meta-reasoning**: ML-based strategy selection
- **Evolutionary Optimization**: Self-tuning parameters

## 🧪 Testing

```bash
# Run all tests
cargo test --lib

# Run specific benchmark
cargo bench --bench spacl_vs_sequential
```

All 71 tests passing! ✅

## 📊 Benchmarks

Compare SPACL vs sequential tableaux:

```bash
cargo bench --bench spacl_vs_sequential
```

Benchmark results are saved to `results/` directory.

## 📚 Documentation

| Document | Description |
|----------|-------------|
| [docs/README.md](docs/README.md) | Full project documentation |
| [docs/SPACL_ALGORITHM.md](docs/SPACL_ALGORITHM.md) | Novel algorithm details |
| [DIRECTORY_STRUCTURE.md](DIRECTORY_STRUCTURE.md) | This directory organization |
| [docs/IMPLEMENTATION_PLAN.md](docs/IMPLEMENTATION_PLAN.md) | Development roadmap |

## 🏗️ Architecture

The codebase is organized into modules:

- **core/**: IRI, entities, ontology, errors
- **logic/**: Axioms, class expressions, datatypes  
- **parser/**: Input format parsers
- **reasoner/**: Tableaux, SPACL, simple reasoner
- **strategy/**: Meta-reasoner, evolutionary, profiles
- **util/**: Cache, memory, validation

## 🔬 Research

This project includes novel contributions:
- **SPACL**: Speculative Parallel Tableaux algorithm
- **Adaptive Conflict Learning**: Nogood-based pruning
- **Evolutionary Parameter Tuning**: Self-optimizing reasoner

See `docs/research/` for research papers and findings.

## 📝 License

This project is dual-licensed:

- **Code** (Rust implementation): [MIT License](LICENSE) - See `LICENSE` file
- **Paper** (LaTeX manuscript): [CC BY 4.0](paper/LICENSE) - See `paper/LICENSE` file

## 🤝 Contributing

[Add contribution guidelines]

---

**Note**: This project has been reorganized for better maintainability. See `DIRECTORY_STRUCTURE.md` for the complete organization guide.
