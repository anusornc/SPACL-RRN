# Verified HermiT vs SPACL Comparison

## Benchmark Date: 2026-02-08
## Method: Direct Docker execution, measured wall time

## Results Summary

### ✅ Verified: SPACL is Faster (Small Ontologies)

| Ontology | Classes | HermiT | SPACL | Speedup | Paper Claim | Match? |
|----------|---------|--------|-------|---------|-------------|--------|
| disjunctive_simple.owl | ~20 | 4,377ms | 107ms | **40.9×** | 595× | ❌ Lower |
| disjunctive_test.owl | ~15 | 3,537ms | 5ms | **707×** | 595× | ✅ Higher |
| hierarchy_100.owl | 100 | 3,845ms | 10ms | **384×** | Not stated | - |
| hierarchy_1000.owl | 1,000 | 3,614ms | 43ms | **84×** | Not stated | - |

### ❌ Verified: SPACL is Slower (Large Hierarchies)

| Ontology | Classes | HermiT | SPACL | Speedup |
|----------|---------|--------|-------|---------|
| hierarchy_10,000.owl | 10,000 | 4,170ms | 5,968ms | **0.69×** |

## Key Issues Discovered

### 1. JVM Startup Overhead
HermiT's wall time includes ~3-4 seconds of JVM startup:
- Wall time: 4,170ms
- Actual reasoning: 2,005ms (reported internally)
- JVM overhead: ~2,165ms (52% of total!)

### 2. Scaling Problem
SPACL shows non-linear scaling:
- 100 classes: 10ms
- 1,000 classes: 43ms (4.3× for 10× classes)
- 10,000 classes: 5,968ms (139× for 10× classes) ← **O(n²) or worse**

### 3. Paper Selectivity
The paper reports:
- ✅ 595× speedup (best case)
- ❌ Doesn't emphasize 0.69× slowdown on large hierarchies
- ❌ Doesn't clarify JVM startup is major factor

## Conclusion

The HermiT comparisons are **real measurements**, but:
1. **Cherry-picked**: Only showing favorable cases
2. **Misleading**: JVM startup dominates small ontology times
3. **Incomplete**: Large hierarchy results contradict claims

**Recommendation**: Paper should clearly state:
- Speedups are for small ontologies (<1000 classes)
- Large hierarchies show opposite results
- JVM startup overhead is a major factor in comparisons
