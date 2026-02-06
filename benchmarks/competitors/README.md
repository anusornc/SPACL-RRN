# OWL2 Reasoner Competitor Benchmarks

This directory contains Docker-based benchmarking infrastructure for comparing **Tableauxx** with major OWL2 DL reasoners.

## Competitors

| Reasoner | Type | Language | Approach |
|----------|------|----------|----------|
| **HermiT** | Tableau | Java | Hypertableau calculus |
| **Konclude** | Saturation | C++ | Optimized saturation + indexing |
| **Pellet** | Tableau | Java | Standard tableau + SWRL |
| **FaCT++** | Tableau | C++ | Optimized tableau |
| **Tableauxx** | Speculative Parallel | Rust | SPACL: Speculative Parallelism + Conflict Learning |

## Quick Start

```bash
# Run complete benchmark suite
cd benchmarks/competitors
./scripts/run_benchmarks.sh

# Or step by step:
./scripts/run_benchmarks.sh prepare   # Copy test ontologies
./scripts/run_benchmarks.sh build     # Build Docker images
./scripts/run_benchmarks.sh run       # Run benchmarks
./scripts/run_benchmarks.sh report    # Generate report
```

## Individual Reasoner Testing

```bash
# Build a specific reasoner
docker build -f docker/Dockerfile.hermit -t owl-reasoner-hermit ../..

# Run benchmark
docker run --rm \
  -v ./ontologies:/ontologies:ro \
  -v ./results:/results \
  owl-reasoner-hermit \
  /ontologies/univ-bench.owl consistency
```

## Directory Structure

```
benchmarks/competitors/
├── docker/                  # Dockerfiles for each reasoner
│   ├── Dockerfile.hermit
│   ├── Dockerfile.konclude
│   ├── Dockerfile.pellet
│   ├── Dockerfile.factpp
│   └── Dockerfile.tableauxx
├── scripts/
│   └── run_benchmarks.sh   # Main benchmark orchestration
├── ontologies/             # Shared volume for test ontologies
├── results/                # Benchmark output
└── docker-compose.yml      # Compose setup (optional)
```

## Test Ontologies

The benchmark uses ontologies from `tests/data/`:

- `univ-bench.owl` - Small university benchmark
- `disjunctive_test.owl` - Tests disjunctive reasoning
- `hierarchy_*.owl` - Scalable hierarchy tests (100-100K classes)

## Metrics

Each reasoner is tested on:

1. **Consistency Checking** - Determine if ontology is consistent
2. **Classification** - Compute class hierarchy (when supported)

Measured:
- Wall-clock time (ms)
- Success/failure status
- Error messages (if any)

## Notes

- **FaCT++**: Limited standalone CLI; primarily designed for OWL API integration
- **Timeouts**: Each test has a 5-minute timeout
- **Hardware**: Results depend on Docker resource limits

## Expected Results

Based on literature:

| Ontology Type | Expected Leader |
|--------------|-----------------|
| EL profile (SNOMED CT-like) | ELK > Konclude > HermiT |
| Disjunctive (unions) | Konclude ≈ HermiT > Pellet |
| Very large hierarchies | Konclude > FaCT++ > HermiT |
| **With disjunctions + SPACL** | **Tableauxx (with parallelism)** |

## References

- [HermiT](http://www.hermit-reasoner.com/) - Oxford University
- [Konclude](https://www.derivo.de/fileadmin/externe_websites/ext_derivo/Konclude/) - University of Ulm
- [Pellet](https://github.com/stardog-union/pellet) - Stardog
- [FaCT++](https://bitbucket.org/dtsarkov/factplusplus/) - Manchester University
