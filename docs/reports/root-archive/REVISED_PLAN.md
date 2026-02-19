# Revised Plan: Solving Large Hierarchy Scaling

## Key Finding: Compact BitSet is SLOWER

Benchmark results:
- 100 classes: 1.5x slower
- 1,000 classes: 2.9x slower  
- 5,000 classes: 3.8x slower

**Why?** BitSets use O(n^2) memory for ancestor matrix!
- Chain of 10K: 12.5 MB vs 80 KB for HashSet

## Real Solution: Better Algorithm (Not Data Structure)

### Phase 1: Linear-Time Topological Sort

Current: O(n * depth) - essentially O(n^2) for chains
Target: O(n + e) - single pass!

```rust
// One-pass ancestor computation
fn compute_ancestors_linear(&self) {
    // 1. Topological sort
    let order = topological_sort(&self.graph);
    
    // 2. Single pass: ancestors[v] = union of ancestors[parents]
    for v in order {
        for parent in &direct_parents[v] {
            ancestors[v].insert(parent);
            ancestors[v].extend(&ancestors[parent]);
        }
    }
}
```

**Expected: 10-30x speedup**

### Phase 2: Lazy Evaluation

Don't compute all ancestors upfront:
```rust
struct LazyHierarchy {
    direct_parents: HashMap<IRI, Vec<IRI>>,
    cache: LruCache<IRI, HashSet<IRI>>,
}
```

### Phase 3: Hybrid Storage
```rust
enum ParentSet {
    Small([IRI; 2]),    // 0-2 parents: inline
    Medium(Vec<IRI>),   // 3-100 parents
    Large(BitSet),      // 100+ parents
}
```

## Expected Results

| Ontology | Current | Optimized | vs HermiT |
|----------|---------|-----------|-----------|
| hierarchy_10K | 6,000ms | 200ms | 20x faster |
| GO_Basic (51K) | timeout | 5s | 4x faster |

## Implementation Order

1. **Week 1**: Linear algorithm (biggest impact)
2. **Week 2**: Lazy evaluation
3. **Week 3**: Hybrid storage
4. **Week 4**: Parallel subtrees

## Target

Beat HermiT on 10K+ class hierarchies within 4 weeks.
