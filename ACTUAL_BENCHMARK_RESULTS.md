# Actual Benchmark Results - Hierarchical Classification

## Synthetic Hierarchy Benchmark (Measured)

| Classes | Hierarchical | Simple Reasoner | Speedup |
|---------|-------------|-----------------|---------|
| 100 | 962 µs | 36 µs | **0.04× (Simple is 26× faster)** |
| 1,000 | 111 ms | 607 µs | **0.005× (Simple is 183× faster)** |
| 5,000 | 2.67 s | 4.15 ms | **0.002× (Simple is 644× faster)** |
| 10,000 | 13.3 s | (skipped) | - |
| 50,000 | (timeout) | (skipped) | - |

## What We Actually Found

### 1. The Hierarchical Engine is MUCH Slower
- For 100 classes: Simple reasoner is **26× faster**
- For 1,000 classes: Simple reasoner is **183× faster**
- For 5,000 classes: Simple reasoner is **644× faster**

### 2. Complexity Issues
The hierarchical engine shows worse-than-linear scaling:
- 100 → 1,000 classes: 100× size increase → 115× time increase (roughly O(n))
- 1,000 → 5,000 classes: 5× size increase → 24× time increase (worse than O(n))
- 5,000 → 10,000 classes: 2× size increase → 5× time increase (worse than O(n))

### 3. Real GO_Basic Test
- Loading GO_Basic (51,897 classes) takes **5+ minutes** just to parse
- Could not complete classification benchmark due to timeouts

## Root Cause Analysis

The hierarchical engine has significant overhead:
1. **Ontology cloning**: Each benchmark iteration clones the entire ontology
2. **Graph construction**: Building the hierarchy graph is expensive
3. **Topological sort overhead**: The petgraph topological sort may not be optimized

## Honest Conclusion

**We CANNOT claim 95× speedup for GO_Basic.** The actual measurements show:

1. For small ontologies (<100 classes), SimpleReasoner is faster
2. For medium ontologies (1K-5K classes), SimpleReasoner is 100-600× faster
3. The hierarchical engine may only become beneficial at very large scales with specific structures
4. The parser is the real bottleneck - taking minutes to load large ontologies

## Recommendation

1. **Revert paper claims** about hierarchical engine speedup
2. **Focus on parser optimization** - that's where the real bottleneck is
3. **Profile the hierarchical engine** to understand why it's slow
4. **Consider that SimpleReasoner might already be optimal** for most cases
