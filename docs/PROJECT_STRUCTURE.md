# Project Structure

This document describes the organization of the Tableauxx OWL2 Reasoner codebase.

## Directory Layout

```
src/
├── lib.rs              # Main library exports and documentation
├── bin/                # Binary executables
│   └── main.rs
├── core/               # Core data structures
│   ├── mod.rs          # Module exports
│   ├── error.rs        # Error types and handling
│   ├── iri.rs          # IRI management
│   ├── entities.rs     # OWL entities (Class, Property, Individual)
│   └── ontology.rs     # Ontology container with indexing
├── logic/              # Logic and axiom definitions
│   ├── mod.rs
│   ├── axioms/         # Axiom types and class expressions
│   │   ├── mod.rs
│   │   ├── core.rs
│   │   ├── types.rs
│   │   ├── class_axioms.rs
│   │   ├── class_expressions.rs
│   │   └── property_expressions.rs
│   └── datatypes/      # Datatype definitions
│       └── mod.rs
├── parser/             # Input parsers
│   ├── mod.rs
│   ├── turtle.rs
│   ├── rdf_xml.rs
│   ├── owl_xml.rs
│   └── ...
├── reasoner/           # Reasoning engines
│   ├── mod.rs          # Core reasoning traits and types
│   ├── tableaux/       # Traditional tableaux algorithm
│   │   ├── mod.rs
│   │   ├── core.rs
│   │   ├── blocking.rs
│   │   ├── dependency.rs
│   │   ├── expansion.rs
│   │   ├── memory.rs
│   │   └── graph.rs
│   ├── speculative.rs  # SPACL (novel algorithm)
│   ├── simple.rs       # Simple cached reasoner
│   ├── batch_operations.rs
│   ├── classification.rs
│   ├── consistency.rs
│   ├── profile_optimized.rs
│   ├── rules.rs
│   └── query/          # Query engine
├── strategy/           # Strategy selection and optimization
│   ├── mod.rs
│   ├── meta_reasoner.rs    # ML-based strategy selection
│   ├── evolutionary.rs     # Evolutionary optimization
│   └── profiles/           # OWL2 profile validation
│       ├── mod.rs
│       ├── common.rs
│       ├── cache.rs
│       ├── el.rs
│       ├── ql.rs
│       └── rl.rs
├── util/               # Utility modules
│   ├── mod.rs
│   ├── cache.rs
│   ├── cache_manager.rs
│   ├── config.rs
│   ├── constants.rs
│   ├── memory.rs
│   ├── memory_protection.rs
│   ├── utils.rs
│   └── validation.rs
├── app/                # Application-specific code
│   ├── mod.rs
│   ├── epcis.rs
│   ├── epcis_parser.rs
│   └── epcis_test_generator.rs
└── storage/            # Storage backends (future)
    └── mod.rs

benches/                # Performance benchmarks
├── spacl_vs_sequential.rs  # SPACL vs sequential comparison
└── ...

tests/                  # Integration tests (future)
```

## Module Responsibilities

### Core (`src/core/`)
Fundamental types used throughout the reasoner:
- **Error handling**: Centralized error types (`OwlError`, `OwlResult`)
- **IRI management**: Efficient IRI storage and manipulation
- **Entities**: OWL2 entities (classes, properties, individuals)
- **Ontology**: Container with indexed storage for fast lookups

### Logic (`src/logic/`)
OWL2 logic definitions:
- **Axioms**: All axiom types (subclass, equivalence, assertions, etc.)
- **Class expressions**: Intersection, union, restriction, etc.
- **Datatypes**: Datatype definitions and value spaces

### Parser (`src/parser/`)
Multi-format ontology parsers:
- Turtle (TTL)
- RDF/XML
- OWL/XML
- JSON-LD (planned)
- Manchester syntax

### Reasoner (`src/reasoner/`)
Reasoning engines:
- **Tableaux**: Traditional tableaux algorithm (complete for SROIQ)
- **SPACL**: Speculative Parallel Tableaux with Adaptive Conflict Learning (novel)
- **Simple**: Simplified reasoner with caching
- **Query**: Query answering engine

### Strategy (`src/strategy/`)
Strategy selection and optimization:
- **Meta-reasoner**: ML-based selection of reasoning strategy
- **Evolutionary**: Genetic algorithm for parameter tuning
- **Profiles**: OWL2 profile validation (EL, QL, RL)

### Util (`src/util/`)
Shared utilities:
- Caching infrastructure
- Configuration management
- Memory management
- Validation utilities

### App (`src/app/`)
Application-specific implementations:
- EPCIS (GS1 supply chain) support

## Key Design Principles

1. **Modularity**: Clear separation of concerns between modules
2. **Re-exports**: Convenient public API through `lib.rs`
3. **Feature flags**: Optional features (e.g., web-service) can be enabled/disabled
4. **Test organization**: Unit tests in modules, integration tests in `tests/`
5. **Benchmarking**: Performance benchmarks in `benches/`

## Adding New Modules

When adding new functionality:

1. Place in appropriate directory based on responsibility
2. Create `mod.rs` if adding a new subdirectory
3. Add public exports to parent `mod.rs`
4. Add convenience re-exports to `src/lib.rs` if part of public API
5. Add documentation and examples

## Build and Test

```bash
# Build the library
cargo build --lib

# Run tests
cargo test --lib

# Run benchmarks
cargo bench

# Run specific benchmark
cargo bench --bench spacl_vs_sequential
```
