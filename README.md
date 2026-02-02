# Tableauxx - Hybrid OWL2 Reasoner

A novel hybrid and evolutionary approach to ontology reasoning, combining tableaux-based algorithms with machine learning-driven strategy selection and evolutionary optimization.

[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![Python](https://img.shields.io/badge/Python-3.8+-blue.svg)](https://www.python.org)
[![License](https://img.shields.io/badge/License-MIT%2FApache--2.0-blue.svg)](LICENSE)

## 🌟 Overview

Tableauxx implements a hybrid reasoning engine that dynamically selects between multiple reasoning strategies based on ontology characteristics:

- **Tableaux Reasoning**: For expressive SROIQ(D) ontologies with nominals and cardinality restrictions
- **Saturation**: Optimized for EL profile ontologies (polynomial time)
- **Transformation**: Lightweight approach for simple ontologies
- **Hybrid**: Combines strategies with intelligent fallback

### Key Innovations

1. **Meta-Reasoner**: ML-based decision tree for automatic strategy selection
2. **Evolutionary Optimization**: Genetic algorithms for tuning reasoning parameters
3. **Multi-Profile Support**: Specialized handling for EL, QL, RL, and SROIQ fragments
4. **ALC Tableau Implementation**: Complete Python implementation with blocking strategies

## 📁 Project Structure

```
tableauxx/
├── Rust Implementation (Core Engine)
│   ├── lib.rs                    # Library entry point (simulation framework)
│   ├── main.rs                   # CLI executable
│   ├── reasoning.rs              # Core reasoning engine trait
│   ├── meta_reasoner.rs          # ML-based strategy selector
│   ├── evolutionary.rs           # Genetic algorithm optimizer
│   ├── benchmarking.rs           # Performance evaluation framework
│   ├── tableaux.rs               # Tableaux algorithm implementation (stub)
│   ├── transformation.rs         # EL++ transformation rules (stub)
│   ├── saturation.rs             # Rule-based saturation engine (stub)
│   ├── simple_benchmark.rs       # Basic benchmark suite
│   ├── benchmark_enhanced_reasoner.rs  # Enhanced reasoner benchmarks
│   ├── enhanced_reasoning_bench.rs     # Reasoning benchmarks
│   └── mod.rs                    # Module declarations
│
├── Python Implementation (Prototyping)
│   ├── tableau_reasoner.py       # Full ALC tableau with expansion rules
│   ├── simple_demo.py            # **SIMULATED** proof-of-concept demo
│   ├── enhanced_reasoner_standard_test.py  # Standard ontology tests
│   ├── test_tableau_reasoner.py  # Unit tests
│   └── benchmark_tableau.py      # Python benchmarking
│
├── Documentation
│   ├── A Novel Hybrid and Evolutionary Approach to Ontology Reasoning.md
│   ├── Tableau Algorithm Research Findings.md
│   ├── novel_algorithm_design.md
│   ├── research_findings.md
│   ├── codebase_analysis.md
│   └── รายงาน*.md               # Thai research reports
│
├── Test Data & Results
│   ├── univ-bench.owl            # Standard test ontology (small sample)
│   ├── standard_ontology_test_results.json
│   ├── enhanced_reasoner_benchmark.json
│   └── *.png                     # Benchmark visualizations
│
└── Cargo.toml                    # Rust project configuration
```

## ⚠️ Important Disclaimer

**This is a research prototype, not a production-ready reasoner.**

- The Python `simple_demo.py` uses **simulated reasoning** (`time.sleep()` and random success rates) to model expected performance
- The Rust `lib.rs` enhanced reasoner uses **simulated reasoning components** (`simulate_*` functions with `thread::sleep()`)
- **Real implementation status**: 
  - ✅ Working ALC tableau in Python
  - ✅ Meta-reasoner framework (rule-based strategy selection)
  - ✅ Evolutionary optimizer structure
  - 🚧 Full OWL2/SROIQ implementation is planned
- Benchmark results shown in documentation are from **simulations**, not real reasoning performance

## 🚀 Quick Start

### Prerequisites

- Rust 1.70+ (install via [rustup](https://rustup.rs))
- Python 3.8+ (for Python components)

### Build & Run

```bash
# Build the Rust project
cargo build --release

# Run the main executable (runs simulations)
cargo run --release

# Run tests
cargo test

# Run Python tableau reasoner (REAL implementation)
python tableau_reasoner.py

# Run Python demo (SIMULATED - for demonstration only)
python simple_demo.py
```

## 🧪 Usage Examples

### Rust - Enhanced Reasoner Framework

```rust
use enhanced_owl_reasoner::{EnhancedOwlReasoner, SimpleOntology};

fn main() -> anyhow::Result<()> {
    // Create ontology
    let mut ontology = SimpleOntology::new();
    ontology.classes = vec!["Person".to_string(), "Student".to_string()];
    ontology.axioms = vec!["Student ⊑ Person".to_string()];
    
    // Create reasoner with meta-reasoner (NOTE: uses simulated reasoning)
    let mut reasoner = EnhancedOwlReasoner::new(ontology)?;
    
    // Check consistency (SIMULATED - returns true with artificial delay)
    let is_consistent = reasoner.is_consistent()?;
    println!("Ontology is consistent: {}", is_consistent);
    
    Ok(())
}
```

### Python - ALC Tableau (REAL Implementation)

```python
from tableau_reasoner import *

# Create concepts
A = atomic_concept("A")
B = atomic_concept("B")

# Test satisfiability of A ⊓ B
conj = conjunction(A, B)
reasoner = TableauReasoner()
satisfiable, model = reasoner.is_satisfiable(conj)

print(f"A ⊓ B is {'satisfiable' if satisfiable else 'unsatisfiable'}")
print(f"Statistics: {reasoner.get_statistics()}")
```

## 📊 Current Implementation Status

| Feature | Status | Notes |
|---------|--------|-------|
| ALC Tableau (Python) | ✅ Working | Full implementation with expansion rules |
| Meta-Reasoner | ✅ Framework | Decision tree for strategy selection |
| Evolutionary Optimizer | ✅ Structure | Basic GA implementation |
| OWL2/SROIQ | 🚧 Planned | Stubs exist, not fully implemented |
| OWL Parsing | 🚧 Planned | Parser infrastructure in Cargo.toml |
| Real Benchmarks | 🚧 Planned | Currently using simulation |

**Code Size**: ~5,500 lines mixed Rust/Python (not 30,000+ as previously claimed)

## 🏗️ Architecture

### Meta-Reasoner Decision Tree

```
Ontology Features
       ↓
  ┌────┴────┐
  ↓         ↓
EL Profile? → Saturation
  └────┬────┘
       ↓ No
  ┌────┴────┐
  ↓         ↓
Low Complexity? → Transformation
  └────┬────┘
       ↓ No
  ┌────┴────┐
  ↓         ↓
Medium Complexity? → Hybrid
  └────┬────┘
       ↓ No
  ┌────┴────┐
  ↓         ↓
High/Nominals? → Tableaux
  └────┬────┘
       ↓
    Default → Hybrid
```

### Evolutionary Optimization

- **Population Size**: 20 strategies
- **Mutation Rate**: 10%
- **Crossover Rate**: 70%
- **Evolves**: Feature weights, selection thresholds, cache config, timeouts

## 🧬 Core Components

| Module | Description |
|--------|-------------|
| `meta_reasoner.rs` | Decision tree + performance history for strategy selection |
| `evolutionary.rs` | Genetic algorithm for parameter optimization |
| `tableau_reasoner.py` | **Working** classical tableau with blocking for termination |
| `lib.rs` | Enhanced reasoner **framework** (simulated reasoning) |

## 🧪 Testing

```bash
# Rust tests
cargo test
cargo test --release

# Python tests (real ALC tableau)
python test_tableau_reasoner.py

# Run demos (simulated performance)
python simple_demo.py
```

## 📈 Benchmarking

**Current Status**: Benchmarks use simulated reasoning to demonstrate the framework.

Real benchmarking against established reasoners (HermiT, Pellet, ELK) is planned for future work.

## 📚 Documentation

Key research documents:

- **A Novel Hybrid and Evolutionary Approach to Ontology Reasoning.md**: Research proposal with simulation results
- **Tableau Algorithm Research Findings.md**: Literature review (accurate)
- **novel_algorithm_design.md**: Algorithm design details
- **รายงานการพัฒนา Tableau Reasoner...**: Thai development report

## 🔬 Research Contributions

1. **Hybrid Strategy Selection Framework**: Architecture for combining multiple reasoning paradigms
2. **Evolutionary Parameter Tuning**: GA structure for optimizing reasoning parameters
3. **Profile-Aware Processing**: Design for specialized algorithms for different OWL2 profiles
4. **Educational Implementation**: Complete Python ALC tableau for learning

## 🛠️ Development

```bash
# Format code
cargo fmt

# Run clippy
cargo clippy -- -D warnings

# Check compilation
cargo check

# Build docs
cargo doc --no-deps
```

## 📦 Dependencies

### Rust
- `rio_api`, `rio_turtle`, `rio_xml` - RDF parsing (declared, not fully used)
- `petgraph`, `indexmap`, `hashbrown` - Data structures
- `serde`, `serde_json` - Serialization
- `rayon`, `dashmap`, `bumpalo` - Performance (declared, not fully used)

### Python
- Standard library only (no external dependencies)

## 📝 License

This project is licensed under either:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

## 👤 Author

**Anusorn Chaikaew** - Research on hybrid ontology reasoning approaches

---

*Research prototype for hybrid ontology reasoning. The ALC tableau implementation is functional; other components are framework/stub implementations.*
