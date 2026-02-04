# Tableauxx Project Directory Structure

This document describes the organization of the Tableauxx OWL2 Reasoner project.

## Root Directory Layout

```
.
├── .github/            # GitHub configurations (workflows, templates)
├── .clinerules/        # Cline IDE rules
├── .kilocode/          # Kilocode IDE configuration
├── .traycer/           # Traycer IDE configuration
├── archive/            # Archived/old files (kept for reference)
├── assets/             # Images, diagrams, visual assets
├── backup_tableauxx/   # Legacy backup (kept for safety)
├── benches/            # Rust benchmark suites
├── docs/               # Documentation
│   ├── reports/        # Project reports (Thai and English)
│   ├── research/       # Research papers and findings
│   └── *.md            # Main documentation files
├── results/            # Benchmark results (JSON files)
├── scripts/            # Python scripts for testing/demo
├── src/                # Main Rust source code
│   ├── bin/            # Binary executables
│   ├── core/           # Core types (IRI, entities, ontology)
│   ├── logic/          # Logic definitions (axioms, datatypes)
│   ├── parser/         # Input parsers
│   ├── reasoner/       # Reasoning engines (tableaux, SPACL)
│   ├── strategy/       # Strategy selection (meta, evolutionary)
│   ├── util/           # Utilities (cache, memory, validation)
│   └── app/            # Application-specific code
├── target/             # Cargo build output
├── tests/              # Test files
│   └── data/           # Test ontologies (OWL files)
├── Cargo.toml          # Rust project configuration
├── Cargo.lock          # Dependency lock file
└── .gitignore          # Git ignore rules
```

## Directory Descriptions

### `src/` - Source Code
Main Rust source code organized by module:
- **core/**: Fundamental types (IRI, entities, ontology, errors)
- **logic/**: OWL logic (axioms, class expressions, datatypes)
- **parser/**: Input format parsers (Turtle, RDF/XML, etc.)
- **reasoner/**: Reasoning engines including the novel SPACL algorithm
- **strategy/**: Strategy selection and optimization
- **util/**: Utility modules (caching, memory, validation)
- **app/**: Application-specific implementations (EPCIS)

### `docs/` - Documentation
- **README.md**: Main project documentation
- **PROJECT_STRUCTURE.md**: Source code structure guide
- **SPACL_ALGORITHM.md**: Novel algorithm documentation
- **IMPLEMENTATION_PLAN.md**: Development roadmap
- **reports/**: Project reports in Thai and English
- **research/**: Research papers and algorithm findings

### `benches/` - Benchmarks
Rust benchmark suites using Criterion:
- `spacl_vs_sequential.rs`: Compares SPACL vs traditional tableaux
- Other performance benchmarks

### `scripts/` - Python Scripts
- `tableau_reasoner.py`: Python tableau implementation
- `test_tableau_reasoner.py`: Python tests
- `simple_demo.py`: Simple demonstration
- `benchmark_tableau.py`: Benchmarking script
- `run_real_benchmarks.py`: Real-world benchmark runner

### `assets/` - Visual Assets
PNG images from benchmarks:
- `benchmark_results.png`
- `detailed_ontology_performance.png`
- `phase_comparison.png`
- `standard_ontology_algorithm_comparison.png`

### `results/` - Benchmark Results
JSON files with benchmark data:
- `enhanced_reasoner_benchmark.json`
- `real_benchmark_results.json`
- `standard_ontology_test_results.json`

### `tests/data/` - Test Data
OWL ontology files for testing:
- `univ-bench.owl`: LUBM benchmark ontology

### `archive/` - Archived Files
Old source files kept for reference:
- Old Rust source files before reorganization
- Previous versions of lib.rs

## Key Files

| File | Purpose |
|------|---------|
| `Cargo.toml` | Rust project configuration |
| `docs/README.md` | Main project documentation |
| `src/lib.rs` | Library root with public exports |
| `src/reasoner/speculative.rs` | SPACL novel algorithm |
| `benches/spacl_vs_sequential.rs` | Performance benchmark |

## Quick Navigation

- **Documentation**: `docs/`
- **Source Code**: `src/`
- **Benchmarks**: `benches/`
- **Test Data**: `tests/data/`
- **Results**: `results/`
- **Python Scripts**: `scripts/`
- **Images**: `assets/`

## Notes

- The `archive/` directory contains old files kept for reference only
- The `backup_tableauxx/` directory is a legacy backup
- Build artifacts are in `target/` (managed by Cargo)
