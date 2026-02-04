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
├── Rust Implementation (Framework)
│   ├── lib.rs                    # Framework entry point
│   ├── main.rs                   # CLI executable
│   ├── reasoning.rs              # Reasoning engine trait definitions
│   ├── meta_reasoner.rs          # Decision tree for strategy selection
│   ├── evolutionary.rs           # Genetic algorithm optimizer
│   ├── benchmarking.rs           # Benchmark framework
│   ├── tableaux.rs               # Tableaux algorithm (stub)
│   ├── transformation.rs         # EL++ transformation (stub)
│   ├── saturation.rs             # Saturation engine (stub)
│   └── ...
│
├── Python Implementation (Working)
│   ├── tableau_reasoner.py       # ✅ FULL ALC tableau implementation
│   ├── simple_demo.py            # Real ALC tableau benchmarks
│   ├── run_real_benchmarks.py    # Comprehensive benchmark suite
│   ├── test_tableau_reasoner.py  # Unit tests (37 tests)
│   ├── benchmark_tableau.py      # Performance benchmarks
│   └── ...
│
└── ...
```

## 🚀 Quick Start

### Prerequisites

- Rust 1.70+ (install via [rustup](https://rustup.rs))
- Python 3.8+ (for working components)

### Build & Run

```bash
# Build the Rust framework
cargo build --release

# Run the Python ALC tableau (REAL implementation)
python tableau_reasoner.py

# Run real benchmarks
python run_real_benchmarks.py
python benchmark_tableau.py

# Run comprehensive tests
python test_tableau_reasoner.py
```

## 📊 Real Benchmark Results

The ALC tableau implementation has been benchmarked with **real** test cases:

### Performance Summary (Real Results)

| Metric | Value |
|--------|-------|
| Total Tests | 37 |
| Pass Rate | 100% |
| Avg Execution Time | 0.72ms |
| Total Nodes Created | 122 |
| Total Rules Applied | 83 |

### Sample Test Results

| Concept | Time | Result |
|---------|------|--------|
| A (Atomic) | 0.02ms | ✓ Satisfiable |
| A ⊓ B | 0.14ms | ✓ Satisfiable |
| A ⊓ ¬A | 0.09ms | ✗ Unsatisfiable |
| ∃R.A | 0.07ms | ✓ Satisfiable |
| ∃R.(A ⊓ B) | 0.42ms | ✓ Satisfiable |
| (∃R.A ⊓ ∀R.¬A) | 0.40ms | ✗ Unsatisfiable |

### Comparison with Baseline

| Metric | ALC Tableau | Simple Baseline |
|--------|-------------|-----------------|
| Coverage | 100% (13/13) | 38.5% (5/13) |
| Complex Concepts | ✅ Handles all | ❌ Limited |
| Avg Speedup | 2.2x | - |

See `real_benchmark_results.json` for detailed results.

## 🧪 Usage Examples

### Python - ALC Tableau (REAL Implementation)

```python
from tableau_reasoner import *

# Create reasoner
reasoner = TableauReasoner()

# Test satisfiability of ∃R.(A ⊓ B)
A = atomic_concept("A")
B = atomic_concept("B")
concept = existential_restriction("R", conjunction(A, B))

satisfiable, model = reasoner.is_satisfiable(concept)
print(f"Satisfiable: {satisfiable}")  # Real result!
print(f"Statistics: {reasoner.get_statistics()}")
```

### Rust - Enhanced Reasoner Framework

```rust
use enhanced_owl_reasoner::{EnhancedOwlReasoner, SimpleOntology};

fn main() -> anyhow::Result<()> {
    // Create ontology
    let mut ontology = SimpleOntology::new();
    ontology.classes = vec!["Person".to_string(), "Student".to_string()];
    
    // Create reasoner framework
    let mut reasoner = EnhancedOwlReasoner::new(ontology)?;
    
    // Strategy selection works
    let is_consistent = reasoner.is_consistent()?;
    println!("Result: {}", is_consistent);
    
    Ok(())
}
```

## 🏗️ Implementation Status

| Feature | Status | Notes |
|---------|--------|-------|
| **ALC Tableau (Python)** | ✅ **Working** | Full implementation with expansion rules |
| **Meta-Reasoner** | ✅ **Implemented** | Decision tree for strategy selection |
| **Evolutionary Optimizer** | ✅ **Structure** | Basic GA implementation |
| **SROIQ(D) Tableaux** | 🚧 **Planned** | Framework stub exists |
| **Saturation Engine** | 🚧 **Planned** | Framework stub exists |
| **OWL Parsing** | 🚧 **Planned** | Dependencies declared |

**Code Size**: ~5,500 lines mixed Rust/Python

## 🧬 Core Components

### ✅ Working (Python)

| Module | Description |
|--------|-------------|
| `tableau_reasoner.py` | Full ALC tableau with expansion rules (⊓, ⊔, ∃, ∀), clash detection, blocking |
| `test_tableau_reasoner.py` | 37 comprehensive tests |
| `benchmark_tableau.py` | Performance comparison with baseline |

### 🚧 Framework (Rust)

| Module | Description |
|--------|-------------|
| `meta_reasoner.rs` | Decision tree for strategy selection |
| `evolutionary.rs` | Genetic algorithm structure |
| `lib.rs` | Enhanced reasoner framework |

## 🧪 Testing

```bash
# Real ALC tableau tests (37 tests)
python test_tableau_reasoner.py

# Real benchmarks
python run_real_benchmarks.py
python benchmark_tableau.py

# Rust framework tests
cargo test
```

## 📈 Benchmarking

### Real Benchmarks Available

1. **`run_real_benchmarks.py`**: Comprehensive ALC tableau benchmarks
2. **`benchmark_tableau.py`**: Comparison with baseline reasoner
3. **`test_tableau_reasoner.py`**: 37 test cases with performance metrics

### Results Location

- `real_benchmark_results.json` - Detailed benchmark results
- Console output with timing and statistics

## 📚 Documentation

Key documents:

- **A Novel Hybrid and Evolutionary Approach to Ontology Reasoning.md**: Research proposal
- **Tableau Algorithm Research Findings.md**: Literature review
- **codebase_analysis.md**: Codebase analysis
- **รายงาน*.md**: Thai research reports

## 🔬 Research Contributions

1. **Hybrid Strategy Selection Framework**: Architecture for combining reasoning paradigms
2. **Evolutionary Parameter Tuning**: GA structure for optimization
3. **Working ALC Tableau**: Complete implementation for educational/research use
4. **Real Benchmarks**: Actual performance measurements

## 🛠️ Development

```bash
# Format code
cargo fmt

# Run clippy
cargo clippy -- -D warnings

# Run tests
cargo test
python test_tableau_reasoner.py
```

## 📦 Dependencies

### Rust
- `serde`, `serde_json` - Serialization
- `petgraph`, `indexmap` - Data structures
- Parser libraries declared for future use

### Python
- Standard library only

## 📝 License

This project is licensed under either:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

## 👤 Author

**Anusorn Chaikaew** - Research on hybrid ontology reasoning approaches

---

*Research prototype with working ALC tableau implementation and framework for enhanced reasoning.*
