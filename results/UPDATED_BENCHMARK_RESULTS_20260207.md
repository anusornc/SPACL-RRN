# Updated Benchmark Results - February 7, 2026

## Fresh Benchmarks (2026-02-07)

Hardware: AMD Ryzen 9 5900X, 32GB RAM, Rust 1.84.0

### Table 1: Head-to-Head Performance Comparison

| Test | HermiT (ms) | Pellet (ms) | SPACL (ms) | Speedup |
|------|-------------|-------------|------------|---------|
| Disjunctive (6 axioms) | 3,569 | 2,233* | **6** | **595×** |
| Hierarchy 10K | 4,269 | 2,367* | 2,705† | **1.6×** |
| Hierarchy 100K | 8,343 | 2,913* | 86,847† | 0.10× |
| LUBM/univ-bench | 3,432 | 2,242* | **5** | **686×** |

*Pellet with OWL API overhead
†SPACL with binary format

### Key Findings

1. **Disjunctive Ontologies**: SPACL achieves 595× speedup vs HermiT
   - HermiT: 3,569ms (OWL API parsing + reasoning)
   - SPACL: 6ms (binary format + optimized reasoning)

2. **10K Hierarchies**: SPACL is 1.6× faster than HermiT
   - HermiT: 4,269ms
   - SPACL: 2,705ms (binary format)
   - Speedup comes from binary parsing (2.7s load vs 5.9s XML)

3. **100K Hierarchies**: HermiT is 10× faster than SPACL
   - HermiT: 8,343ms
   - SPACL: 86,847ms
   - SPACL is optimized for disjunctive reasoning, not pure hierarchies

4. **LUBM/univ-bench**: SPACL achieves 686× speedup vs HermiT
   - HermiT: 3,432ms
   - SPACL: 5ms

### Limitations Acknowledged

- Real-world biomedical ontologies (PATO, DOID, UBERON, GO) show 0.4-0.5× speedup
  because they are primarily taxonomic hierarchies with sparse disjunctions (<0.1%)
- SPACL is optimized for disjunctive reasoning, not pure taxonomic hierarchies

---

*Results verified on 2026-02-07*
