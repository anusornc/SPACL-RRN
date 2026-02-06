# Tableauxx - OWL2 DL Reasoner

A high-performance, feature-complete OWL2 reasoning engine implemented in Rust.

## 🚀 Quick Start

```bash
# Build the library
cargo build --lib

# Run tests
cargo test --lib

# Run benchmarks
cargo bench
```

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

### Citation

If you use this work in your research, please cite:

```bibtex
@article{spacl2026,
  title={SPACL: Speculative Parallelism and Conflict Learning for Scalable OWL Ontology Reasoning},
  author={Chaikaew, Anusorn and Chouvatut, Varin and Boonchieng, Ekkarat},
  journal={Journal of Web Semantics},
  year={2026},
  publisher={Elsevier}
}
```

## 🤝 Contributing

[Add contribution guidelines]

---

**Note**: This project has been reorganized for better maintainability. See `DIRECTORY_STRUCTURE.md` for the complete organization guide.
