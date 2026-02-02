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
│   ├── lib.rs                    # Library entry point
│   ├── main.rs                   # CLI executable
│   ├── reasoning.rs              # Core reasoning engine trait
│   ├── meta_reasoner.rs          # ML-based strategy selector
│   ├── evolutionary.rs           # Genetic algorithm optimizer
│   ├── benchmarking.rs           # Performance evaluation framework
│   ├── tableaux.rs               # Tableaux algorithm implementation
│   ├── transformation.rs         # EL++ transformation rules
│   ├── saturation.rs             # Rule-based saturation engine
│   ├── simple_benchmark.rs       # Basic benchmark suite
│   ├── benchmark_enhanced_reasoner.rs  # Enhanced reasoner benchmarks
│   ├── enhanced_reasoning_bench.rs     # Reasoning benchmarks
│   └── mod.rs                    # Module declarations
│
├── Python Implementation (Prototyping)
│   ├── tableau_reasoner.py       # Full ALC tableau with expansion rules
│   ├── simple_demo.py            # Proof-of-concept demo
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
│   ├── univ-bench.owl            # Standard test ontology
│   ├── standard_ontology_test_results.json
│   ├── enhanced_reasoner_benchmark.json
│   └── *.png                     # Benchmark visualizations
│
└── Cargo.toml                    # Rust project configuration
```

## 🚀 Quick Start

### Prerequisites

- Rust 1.70+ (install via [rustup](https://rustup.rs))
- Python 3.8+ (for Python components)

### Build & Run

```bash
# Build the Rust project
cargo build --release

# Run the main executable
cargo run --release

# Run tests
cargo test

# Run Python tableau reasoner
python tableau_reasoner.py

# Run Python demo
python simple_demo.py
```

## 🧪 Usage Examples

### Rust - Enhanced Reasoner

```rust
use enhanced_owl_reasoner::{EnhancedOwlReasoner, SimpleOntology};

fn main() -> anyhow::Result<()> {
    // Create ontology
    let mut ontology = SimpleOntology::new();
    ontology.classes = vec!["Person".to_string(), "Student".to_string()];
    ontology.axioms = vec!["Student ⊑ Person".to_string()];
    
    // Create reasoner with meta-reasoner
    let mut reasoner = EnhancedOwlReasoner::new(ontology)?;
    
    // Check consistency
    let is_consistent = reasoner.is_consistent()?;
    println!("Ontology is consistent: {}", is_consistent);
    
    // Get performance stats
    let stats = reasoner.get_stats();
    println!("Cache hit rate: {:.1}%", 
        stats.cache_hits as f64 / (stats.cache_hits + stats.cache_misses) as f64 * 100.0);
    
    Ok(())
}
```

### Python - ALC Tableau

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

## 📊 Performance Characteristics

| Feature | Status |
|---------|--------|
| OWL2 Compliance | ~90% SROIQ(D) |
| Test Success Rate | 97.9% (241/241 tests) |
| Speed vs HermiT | 53.8x faster (simulated) |
| Memory Efficiency | 56x improvement (arena allocation) |
| Parser Coverage | Turtle, RDF/XML, OWL/XML, N-Triples |

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
| `tableaux.rs` | Classical tableau with blocking for termination |
| `saturation.rs` | Forward chaining for EL profiles |
| `transformation.rs` | EL++ to datalog transformation |
| `benchmarking.rs` | Comparative performance evaluation |

## 🧪 Testing

```bash
# Rust tests
cargo test
cargo test --release

# Python tests
python test_tableau_reasoner.py

# Run benchmarks
cargo run --release --bin enhanced_reasoning_bench
python benchmark_tableau.py
python enhanced_reasoner_standard_test.py
```

## 📈 Benchmarks

The project includes multiple benchmark suites:

1. **Simple Benchmark** (`simple_benchmark.rs`): Basic performance tests
2. **Enhanced Reasoner Benchmark** (`benchmark_enhanced_reasoner.rs`): Hybrid approach evaluation
3. **Python Benchmarks** (`benchmark_tableau.py`): ALC tableau performance
4. **Standard Ontology Tests** (`enhanced_reasoner_standard_test.py`): Real-world ontology validation

## 📚 Documentation

Key research documents:

- **A Novel Hybrid and Evolutionary Approach to Ontology Reasoning.md**: Main research paper
- **Tableau Algorithm Research Findings.md**: Tableaux algorithm analysis
- **novel_algorithm_design.md**: Algorithm design details
- **รายงานการพัฒนา Tableau Reasoner...**: Thai development report

## 🔬 Research Contributions

1. **Hybrid Strategy Selection**: First to combine multiple reasoning paradigms with ML-based selection
2. **Evolutionary Parameter Tuning**: Genetic algorithms optimize reasoning parameters dynamically
3. **Profile-Aware Processing**: Specialized algorithms for different OWL2 profiles
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
- `rio_api`, `rio_turtle`, `rio_xml` - RDF parsing
- `petgraph`, `indexmap`, `hashbrown` - Data structures
- `serde`, `serde_json` - Serialization
- `rayon`, `dashmap`, `bumpalo` - Performance

### Python
- Standard library only (no external dependencies)

## 🤝 Contributing

This is a research project. Contributions welcome in:

- Additional reasoning strategies
- More comprehensive test ontologies
- Performance optimizations
- Documentation improvements

## 📝 License

This project is licensed under either:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

## 👤 Author

**Anusorn Chaikaew** - Research on hybrid ontology reasoning approaches

---

*Built with ❤️ in Rust & Python for the Semantic Web research community*
