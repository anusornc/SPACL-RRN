# Tableauxx Benchmark Results

**Date**: February 2, 2026  
**Test Environment**: Local development machine

## Executive Summary

Benchmarks demonstrate that the **sequential tableaux reasoner** is extremely fast for small to medium ontologies, while the novel **SPACL algorithm** shows expected overhead for small problems due to thread synchronization costs.

## Sequential Tableaux Performance

### Hierarchy Scaling Test

Testing class hierarchy reasoning with different ontology sizes:

| Classes | Median Time | Throughput |
|---------|-------------|------------|
| 10 | 3.98 µs | ~2.5M ops/s |
| 50 | 13.42 µs | ~3.7M ops/s |
| 100 | 29.11 µs | ~3.4M ops/s |

**Observation**: Linear scaling with excellent performance. 100 classes processed in under 30 microseconds.

### Simple Reasoner

| Test | Median Time |
|------|-------------|
| Family Ontology (4 classes) | 2.62 µs |

## SPACL Algorithm Analysis

### SPACL vs Sequential (4 branches)

| Configuration | Median Time | vs Sequential |
|---------------|-------------|---------------|
| Sequential | 11.7 µs | 1.0x (baseline) |
| SPACL Single Worker | 48.4 µs | 4.1x slower |
| SPACL Default | 189.9 µs | 16.2x slower |
| SPACL No Learning | 192.2 µs | 16.4x slower |

### Key Findings

1. **SPACL Overhead**: Currently shows 4-16x overhead vs sequential for small ontologies
2. **Thread Synchronization**: Mutex locks and Arc cloning add significant overhead
3. **Speculative Work**: Processing branches speculatively wastes cycles when problems are small

## Why SPACL is Slower on Small Ontologies

1. **Fixed Costs**: Thread pool creation, work queue management
2. **Synchronization**: Mutex contention on shared nogood database
3. **Small Problem Size**: Parallelism benefits only appear at larger scales (>1000 branches)
4. **Nogood Overhead**: Database lookups add cost even when few nogoods exist

## Recommendations

### For Current Use
- **Small ontologies (< 1000 classes)**: Use **sequential reasoner**
- **Large ontologies with complex branching**: SPACL may show benefits at scale (needs testing)

### For SPACL Optimization
1. **Adaptive Threshold**: Only use parallelism when branch count exceeds threshold
2. **Work Stealing Optimization**: Reduce synchronization overhead
3. **Nogood Caching**: Improve hit rates for learned conflicts
4. **Lazy Thread Pool**: Create threads only when needed

## Benchmark Methodology

- **Framework**: Criterion.rs
- **Samples**: 50 per test
- **Warm-up**: 1-3 seconds
- **Measurement**: Median time with 95% confidence intervals

## Running Benchmarks

```bash
# Quick benchmark (~30 seconds)
cargo bench --bench quick_benchmark

# Full SPACL comparison (~5-10 minutes)
cargo bench --bench spacl_vs_sequential

# View HTML report
open target/criterion/report/index.html
```

## Conclusion

The sequential tableaux reasoner demonstrates excellent performance for small to medium ontologies, processing 100-class hierarchies in under 30 microseconds. The SPACL algorithm, while innovative, requires further optimization and larger test cases to demonstrate its parallel processing advantages. The current implementation serves as a solid foundation for future optimizations.

---

*Generated: February 2, 2026*
