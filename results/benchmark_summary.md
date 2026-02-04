# Benchmark Results Summary

**Date**: February 2, 2026

## Overview

Benchmarks were run comparing the sequential tableaux reasoner against the novel SPACL (Speculative Parallel Tableaux with Adaptive Conflict Learning) algorithm.

## Key Findings

### 1. Simple Reasoner (Family Ontology)
- **Median Time**: 3.74 µs
- **Use Case**: Small family ontology with basic class hierarchies
- **Status**: ✅ Very fast for simple cases

### 2. Hierarchy Scaling (Sequential Only)

| Classes | Median Time | Throughput |
|---------|-------------|------------|
| 10 | 11.2 µs | 890K elem/s |
| 50 | 60.8 µs | 822K elem/s |
| 100 | 174 µs | 575K elem/s |
| 500 | 2.09 ms | 239K elem/s |

**Observation**: Performance scales roughly linearly with ontology size for simple hierarchies.

### 3. SPACL vs Sequential (Size 4 Branches)

| Algorithm | Median Time | vs Sequential |
|-----------|-------------|---------------|
| Sequential | 11.7 µs | baseline |
| SPACL Single Worker | 48.4 µs | 4.1x slower |
| SPACL Default | 189.9 µs | 16.2x slower |
| SPACL No Learning | 192.2 µs | 16.4x slower |

## Analysis

### Performance Characteristics

1. **Sequential Reasoner**: Extremely fast for small ontologies (< 100 classes)
2. **SPACL Overhead**: Currently shows significant overhead compared to sequential
   - Thread synchronization costs
   - Work queue management
   - Nogood database lookups

### Why SPACL is Slower on Small Ontologies

1. **Parallelization Threshold**: The current implementation spawns threads even for small problems
2. **Synchronization Cost**: Mutex locks and Arc cloning add overhead
3. **Speculative Work**: Processing branches that may not be needed wastes cycles
4. **Small Problem Size**: Benefits of parallelism only appear at larger scales

## Recommendations

### For Current Use
- **Small ontologies (< 1000 classes)**: Use sequential reasoner
- **Large ontologies with complex branches**: SPACL may show benefits at scale

### For SPACL Optimization
1. **Adaptive Threshold**: Only use parallelism when branch count exceeds threshold
2. **Work Stealing Optimization**: Reduce synchronization overhead
3. **Nogood Caching**: Improve hit rates for learned conflicts
4. **Batch Processing**: Process multiple work items per thread acquisition

## Next Steps

1. Run benchmarks with larger ontologies (10K+ classes)
2. Profile SPACL to identify bottlenecks
3. Implement adaptive threshold based on ontology complexity
4. Add more sophisticated work distribution strategies

---

*Benchmarks run on: Criterion.rs with 100 samples, 3s warm-up*
