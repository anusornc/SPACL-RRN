# Consultation Request for Codex (OpenAI)

## Problem Context

We have an OWL2 DL reasoner written in Rust. The hierarchical classification engine for taxonomic ontologies shows poor scaling:

**Benchmark Results (Current Implementation):**
| Classes | Time | vs HermiT |
|---------|------|-----------|
| 1,000 | 90ms | ~equal |
| 5,000 | 2.5s | ~equal |
| 10,000 | 6s | 0.69x slower |
| 50,000 | timeout (180s+) | much slower |

**Current Algorithm:**
```rust
fn compute_transitive_closure(&mut self) {
    // BFS from each node, using HashSet union
    for class in all_classes {
        let mut queue = VecDeque::new();
        queue.push_back(class);
        
        while let Some(current) = queue.pop_front() {
            for parent in direct_parents[current] {
                ancestors[class].insert(parent);
                ancestors[class].extend(&ancestors[parent]); // O(n) copy!
                queue.push_back(parent);
            }
        }
    }
}
```

This is O(n²) for chains (n classes × depth n).

## Failed Attempt: Compact BitSet

We tried replacing HashMap<IRI, HashSet<IRI>> with Vec<BitSet>:

```rust
struct CompactHierarchy {
    ancestors: Vec<CompactBitSet>, // n bitsets × n bits each = O(n²) memory
}
```

**Result:** 1.5-3.8x SLOWER than original!
- Chain of 10K: 12.5 MB memory vs 80 KB for HashSet
- Cache thrashing from random BitSet access

## Proposed Solutions

### Option A: Linear Algorithm (Topological Sort + DP)
```rust
fn compute_linear(&self) {
    // 1. Topological sort (Kahn's algorithm)
    let order = topological_sort(&graph);
    
    // 2. Single pass DP
    for v in order {
        for parent in &direct_parents[v] {
            ancestors[v].insert(parent);
            ancestors[v].extend(&ancestors[parent]); // Reuse computed!
        }
    }
}
```
- Time: O(n + e) for entire hierarchy
- Space: O(n + e)
- For chain: O(n) instead of O(n²)

### Option B: Lazy Evaluation
```rust
struct LazyHierarchy {
    direct_parents: HashMap<IRI, Vec<IRI>>,
    cache: LruCache<IRI, Arc<HashSet<IRI>>>,
}

fn get_ancestors(&mut self, class: &IRI) -> Arc<HashSet<IRI>> {
    if let Some(cached) = self.cache.get(class) {
        return cached.clone();
    }
    // Compute on demand via DFS
    let ancestors = self.compute_dfs(class);
    self.cache.put(class.clone(), Arc::new(ancestors.clone()));
    Arc::new(ancestors)
}
```
- Only compute what's queried
- Best case: O(k × depth) for k queries vs O(n²)

### Option C: Hybrid Sparse/Dense Storage
```rust
enum ParentSet {
    Small([IRI; 2], u8),     // 0-2 parents: inline, no allocation
    Medium(SmallVec<[IRI; 8]>), // 3-16: stack
    Large(BitSet),           // 17+: dense
}
```
- Chains: Zero heap allocation
- DAGs: Use appropriate representation

### Option D: Parallel Subtrees
```rust
fn compute_parallel(&self) {
    let roots = self.find_roots();
    roots.par_iter().map(|r| self.compute_subtree(r)).collect()
}
```
- Process independent branches in parallel
- Best for bushy hierarchies (like GO)

## Questions for Codex

1. **Which approach should we prioritize?** 
   - Is Option A (linear algorithm) the right first step?
   - Should we combine multiple approaches?

2. **Implementation details for Option A:**
   - Should we use petgraph's toposort or custom implementation?
   - For ancestor union, is HashSet::extend efficient enough, or should we use Vec + sort + dedup?
   - How to handle cycles (SCC detection with tarjan_scc)?

3. **Memory layout optimization:**
   - Current: HashMap<IRI, HashSet<IRI>> - random access, cache misses
   - Better: Vec<SmallVec<[IRI; 4]>> with IRI→index mapping?
   - Or: Keep HashMap but optimize HashSet (rustc_hash::FxHashSet)?

4. **Lazy vs Eager:**
   - For a classification API that returns full hierarchy: Is lazy useful?
   - Or should we stick to eager but make it linear time?

5. **Rust-specific optimizations:**
   - Would arenas (bumpalo) help with allocation overhead?
   - Should we use Arc<IRI> interning to reduce cloning?
   - Is it worth using unsafe for unchecked index access in hot loops?

6. **Validation strategy:**
   - How to verify correctness on large ontologies (GO: 50K classes)?
   - Should we compare output against HermiT for validation?

## Target Performance

| Classes | Current | HermiT | Target |
|---------|---------|--------|--------|
| 10,000 | 6,000ms | 4,170ms | <500ms (10x faster) |
| 50,000 | timeout | ~8,000ms | <5,000ms (beat HermiT) |

## Constraints

- Must handle arbitrary OWL subclass axioms (not just trees)
- Must detect cycles (inconsistent ontologies)
- Must return full ClassHierarchy for compatibility
- Memory should be <1GB for 100K classes

## Request

Please analyze these options and recommend:
1. The optimal implementation strategy
2. Specific Rust crates/techniques to use
3. Order of implementation (priorities)
4. Potential pitfalls to avoid
5. Expected performance improvement for each phase

We need to beat HermiT on 10K+ class hierarchies. What's the best path forward?
