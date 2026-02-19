# Plan: Solving Poor Scaling on Large Hierarchies

## Current Problem Analysis

### Why is the Hierarchical Engine Slow?

From code analysis and benchmarks:

1. **Ontology Cloning** (Major bottleneck)
   - Each benchmark iteration: `ontology.clone()` 
   - For 50K classes: clones massive data structures
   - Time: ~100ms+ per clone

2. **HashSet/HashMap Overhead** (Major bottleneck)
   - `parents: HashMap<IRI, HashSet<IRI>>` 
   - `children: HashMap<IRI, HashSet<IRI>>`
   - For 50K classes: millions of small allocations
   - Cache-unfriendly random access pattern

3. **Transitive Closure Algorithm** (Moderate bottleneck)
   - BFS with HashSet operations at each step
   - `O(n * avg_depth)` HashSet insertions
   - For deep hierarchies: ~50K × 100 = 5M insertions

4. **IRI Cloning** (Moderate bottleneck)
   - `let class_iri = (**class.iri()).clone()` 
   - Repeated cloning of same IRIs

---

## Research: What Other Reasoners Do

### ELK (Consequence-Based)
- Uses **saturation** instead of tableaux
- Keeps consequences in memory-efficient format
- **Rule indexing** - direct lookup, no search
- Parallelizes rule applications

### Konclude (Tableaux + Saturation)
- **Binary decision diagrams (BDDs)** for representing sets
- **Compressed bitmaps** for class hierarchies
- Incremental classification

### HermiT (HyperTableaux)
- **Lazy unfolding** - don't materialize full hierarchy
- **Model caching** between consistency checks
- Optimized C++ with minimal allocations

### RDFox (Datalog-based)
- **Columnar storage** (not row-based)
- **Vectorized operations** (SIMD)
- **Incremental maintenance** of materialization

---

## Proposed Solutions (Phased Approach)

### Phase 1: Memory Layout Optimization (High Impact)

#### 1.1 Replace HashMap with Vec + Index
```rust
// Instead of: HashMap<IRI, HashSet<IRI>>
// Use: Vec<BitSet> with class index mapping

struct CompactHierarchy {
    // Map IRI -> compact index (0..n-1)
    iri_to_idx: HashMap<IRI, u32>,  // Only this is HashMap
    
    // Dense storage: parents[i] = bitset of parent indices
    parents: Vec<BitSet>,  // Fixed-size, cache-friendly
    children: Vec<BitSet>,
}
```

**Benefits:**
- Cache-friendly sequential access
- BitSet operations use SIMD (automatic in Rust)
- 64× smaller memory for parent sets (1 bit vs 1 pointer)
- Predictable memory layout

#### 1.2 Pool Allocator for IRIs
```rust
struct IriPool {
    // Intern all IRIs once
    interned: Arc<Vec<IRI>>,
    // Use indices instead of Arc<IRI> everywhere
}
```

**Benefits:**
- No IRI cloning during classification
- Compare indices instead of string comparison

### Phase 2: Algorithm Improvements (High Impact)

#### 2.1 Incremental Transitive Closure
```rust
// Current: Compute everything from scratch
// New: Incremental updates with worklist

fn compute_incremental(&mut self, changes: &[Axiom]) {
    let mut worklist: VecDeque<ClassIdx> = changes.iter().map(...).collect();
    
    while let Some(class) = worklist.pop_front() {
        // Only recompute affected classes
        let new_ancestors = self.compute_ancestors(class);
        let old_ancestors = self.ancestors[class].clone();
        
        // Propagate changes to children
        for descendant in self.get_descendants(class) {
            if new_ancestors != old_ancestors {
                worklist.push_back(descendant);
            }
        }
    }
}
```

**Benefits:**
- For dynamic ontologies: O(changes) not O(n)
- Reuse previous computation

#### 2.2 Parallel Transitive Closure
```rust
// Use Rayon for parallel frontier expansion
use rayon::prelude::*;

fn compute_parallel(&mut self) {
    // Process independent subtrees in parallel
    let roots = self.find_roots();
    
    roots.par_iter().for_each(|root| {
        self.compute_subtree(root);
    });
}
```

**Benefits:**
- Independent subtrees processed in parallel
- Linear speedup with cores (for bushy hierarchies)

### Phase 3: Lazy Evaluation (Medium Impact)

#### 3.1 On-Demand Ancestor Computation
```rust
// Instead of: precompute all ancestors
// Do: compute when queried, with caching

struct LazyHierarchy {
    direct_parents: Vec<Vec<ClassIdx>>,
    ancestor_cache: HashMap<ClassIdx, Arc<BitSet>>,
}

impl LazyHierarchy {
    fn get_ancestors(&mut self, class: ClassIdx) -> Arc<BitSet> {
        if let Some(cached) = self.ancestor_cache.get(&class) {
            return cached.clone();
        }
        
        // Compute on demand
        let ancestors = self.compute_ancestors(class);
        let arc = Arc::new(ancestors);
        self.ancestor_cache.insert(class, arc.clone());
        arc
    }
}
```

**Benefits:**
- Only compute what's needed
- For sparse queries: massive speedup

#### 3.2 Skip Unchanged Classes
```rust
// If subclass axioms haven't changed, skip recomputation
fn classify(&mut self, ontology: &Ontology, changed: Option<&[Axiom]>) {
    match changed {
        None => self.full_classification(),
        Some(changes) => self.incremental_classification(changes),
    }
}
```

### Phase 4: Low-Level Optimizations (Medium Impact)

#### 4.1 SIMD BitSet Operations
```rust
// Use bitvec crate or custom SIMD bitset
use bitvec::prelude::*;

// Union of ancestor sets: 64 classes at a time with SIMD
fn union_ancestors(dst: &mut BitSet, src: &BitSet) {
    // Automatic SIMD via LLVM
    for (d, s) in dst.chunks_mut(64).zip(src.chunks(64)) {
        d.store(d.load::<u64>() | s.load::<u64>());
    }
}
```

#### 4.2 Memory Pool for BitSets
```rust
struct BitSetPool {
    // Reuse allocated bitsets instead of allocating new ones
    available: Vec<BitSet>,
}
```

### Phase 5: Hybrid Approach (Research Direction)

#### 5.1 EL-Style Saturation for Hierarchies
For purely EL (Existential Logic) ontologies:
```rust
// Instead of tableaux or naive hierarchy building
// Use consequence-based saturation

fn saturate(&mut self) {
    // Apply rules until fixpoint
    // SubClassOf(A, B) + SubClassOf(B, C) => SubClassOf(A, C)
    // This is O(n) for tree structures
}
```

#### 5.2 Graph Compression
```rust
// Compress long chains
// A -> B -> C -> D becomes A -> D (transitive reduction)
fn compress_chains(&mut self) {
    // Remove intermediate nodes with single parent/child
}
```

---

## Expected Performance Improvements

### Hierarchy_10,000 (Current: 0.69× HermiT)

| Optimization | Expected Time | Speedup vs Current | vs HermiT |
|--------------|---------------|-------------------|-----------|
| **Baseline** | 5,968ms | 1.0× | 0.69× |
| **Phase 1** (Compact layout) | 2,000ms | 3.0× | 2.1× |
| **Phase 2** (Parallel) | 500ms | 12× | 8.3× |
| **Phase 3** (Lazy eval) | 300ms | 20× | 13.9× |
| **Combined** | 200ms | 30× | 20× |

### Hierarchy_50,000 (Estimated)

| Approach | Expected Time | vs HermiT (est. ~8s) |
|----------|---------------|---------------------|
| **Current** | ~180s (estimated) | 0.04× |
| **Optimized** | ~2s | 4× |

---

## Implementation Roadmap

### Week 1-2: Phase 1 - Memory Layout
- [ ] Implement CompactHierarchy with Vec<BitSet>
- [ ] IRI interning/pooling
- [ ] Benchmark vs current

### Week 3-4: Phase 2 - Parallelization
- [ ] Parallel subtree processing
- [ ] Thread-safe hierarchy building
- [ ] Benchmark with varying thread counts

### Week 5-6: Phase 3 - Lazy Evaluation
- [ ] On-demand ancestor computation
- [ ] Cache management
- [ ] Incremental classification API

### Week 7-8: Phase 4 - Low-Level
- [ ] SIMD bitset operations
- [ ] Memory pools
- [ ] Profile-guided optimization

### Week 9-10: Phase 5 - Research
- [ ] EL saturation prototype
- [ ] Graph compression
- [ ] Hybrid algorithm selection

---

## Discussion Points with Codex/LLM

### Q1: BitSet Implementation Strategy
"Should we use:
- A) `bitvec` crate (high-level, safe)
- B) `fixedbitset` crate (battle-tested)
- C) Custom SIMD bitset (max performance)
- D) HashSet<u64> chunks (simple, portable)"

### Q2: Parallelism Granularity
"For parallel hierarchy building:
- A) One thread per root (coarse)
- B) Work-stealing at subtree level (medium)
- C) SIMD within a single hierarchy computation (fine)"

### Q3: Cache Invalidation Strategy
"For lazy ancestor caching:
- A) LRU cache with fixed size
- B) Full recomputation on any change
- C) Dependency tracking with partial updates
- D) No cache, always compute on demand"

### Q4: Memory vs Speed Tradeoff
"For 100K classes:
- A) Use 100MB RAM, get O(1) queries
- B) Use 10MB RAM, get O(log n) queries  
- C) Use 1MB RAM, get O(n) queries"

---

## Success Metrics

1. **hierarchy_10K.owl**: Beat HermiT (target: <4s, stretch: <1s)
2. **hierarchy_50K.owl**: Reasonable time (target: <10s)
3. **GO_Basic**: Complete classification in <5s (currently: timeout)
4. **Memory usage**: <2× current, ideally <500MB for 100K classes

---

## Risks and Mitigations

| Risk | Mitigation |
|------|------------|
| BitSet operations slower than HashSet for sparse data | Hybrid: HashSet for sparse, BitSet for dense |
| Parallel overhead > benefit for small hierarchies | Adaptive: sequential <1K, parallel >10K |
| Incremental updates too complex | Fall back to full recompute if change set >10% |
| Memory usage too high | Streaming + disk-backed for >100K |

---

## Next Steps

1. **Prototype Phase 1** (CompactHierarchy) - 2 days
2. **Benchmark** on all hierarchy sizes
3. **Decide** if Phases 2+ are needed based on results
4. **Iterate** based on profiling data

**Target**: Get hierarchy_10K to beat HermiT within 1 week.
