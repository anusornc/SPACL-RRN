# Large Ontology Optimization Plan

## Problem Analysis

### Current Bottlenecks for Large Ontologies (50K+ classes)

1. **Classification is O(n²)** - `discover_equivalences_by_reasoning()` and `discover_disjointness_by_reasoning()` check all class pairs
   - For 50K classes: ~1.25 billion pair checks
   - Each check may involve tableaux reasoning

2. **Sequential Class Processing** - Classes are processed one-by-one even in parallel mode

3. **No Early Termination** - Processes all pairs even when answer is obvious

4. **Memory Cloning** - `process_work_item_simple` clones entire ontology for each branch

5. **SimpleReasoner.compute_consistency() is shallow** - May return "consistent" without thorough checking

---

## Solution Architecture

### Phase 1: Smart Workload Detection (Priority: HIGH)

**Goal:** Detect non-disjunctive/simple ontologies and use fast path

```rust
pub struct OntologyCharacteristics {
    pub class_count: usize,
    pub disjunction_count: usize,
    pub complex_axiom_count: usize,
    pub hierarchy_depth: usize,
    pub is_tree_like: bool,
    pub complexity_score: f64,  // 0.0 = simple, 1.0 = highly complex
}

impl OntologyCharacteristics {
    pub fn analyze(ontology: &Ontology) -> Self {
        // Count disjunctions (ObjectUnionOf)
        // Count existentials/universals
        // Analyze hierarchy structure
        // Calculate complexity score
    }
    
    pub fn should_use_fast_path(&self) -> bool {
        self.disjunction_count == 0 && 
        self.complex_axiom_count < 100 &&
        self.is_tree_like
    }
}
```

**Implementation:**
- [ ] Create `src/strategy/ontology_analysis.rs`
- [ ] Analyze structure during ontology loading
- [ ] Cache characteristics for reuse

---

### Phase 2: Hierarchical Classification (Priority: HIGH)

**Goal:** Use topological sort + divide-and-conquer for O(n log n) classification

**Algorithm:**
```rust
/// Hierarchical classification using topological levels
pub fn classify_hierarchical(&mut self) -> OwlResult<ClassificationResult> {
    // 1. Build directed acyclic graph of direct relationships
    let dag = self.build_dag();
    
    // 2. Compute topological levels (depth in hierarchy)
    let levels = compute_topological_levels(&dag);
    
    // 3. Process level by level (bottom-up)
    for level in levels.iter().rev() {
        // Process classes in parallel within each level
        let results: Vec<_> = level.par_iter()
            .map(|class| self.classify_single_class(class))
            .collect();
        
        // Update hierarchy with results
        for (class, parents) in results {
            self.hierarchy.add_parents(class, parents);
        }
    }
    
    // 4. Transitive closure is now implicit in the level structure
    Ok(self.build_result())
}
```

**Benefits:**
- O(n log n) instead of O(n²) for tree-like hierarchies
- Natural parallelism at each level
- No redundant checks

**Implementation:**
- [ ] Implement topological level computation
- [ ] Add parallel level processing
- [ ] Integrate with existing ClassificationEngine

---

### Phase 3: Incremental Batch Classification (Priority: HIGH)

**Goal:** Classify in chunks with early termination

```rust
pub struct BatchClassificationConfig {
    pub batch_size: usize,          // Process 1000 classes at a time
    pub time_limit_per_batch: Duration,
    pub early_termination_threshold: f64,  // Stop if 95% classified
}

impl ClassificationEngine {
    pub fn classify_incremental(&mut self, config: BatchClassificationConfig) 
        -> OwlResult<ClassificationResult> {
        
        let classes: Vec<_> = self.ontology.classes().collect();
        let mut classified = HashSet::new();
        let mut results = ClassificationResult::default();
        
        for batch in classes.chunks(config.batch_size) {
            let batch_start = Instant::now();
            
            // Parallel batch processing
            let batch_results: Vec<_> = batch.par_iter()
                .filter(|c| !classified.contains(c.iri()))
                .map(|class| {
                    // Check only against already classified classes
                    let parents = self.find_parents_in_set(class, &classified);
                    (class.iri().clone(), parents)
                })
                .collect();
            
            // Update progress
            for (class_iri, parents) in batch_results {
                results.add_classification(class_iri, parents);
                classified.insert(class_iri);
            }
            
            // Check time limit
            if batch_start.elapsed() > config.time_limit_per_batch {
                results.mark_partial();
                break;
            }
            
            // Check early termination
            let progress = classified.len() as f64 / classes.len() as f64;
            if progress >= config.early_termination_threshold {
                break;
            }
        }
        
        Ok(results)
    }
}
```

**Implementation:**
- [ ] Add batch processing mode
- [ ] Add progress tracking
- [ ] Add graceful timeout handling

---

### Phase 4: Optimized Subclass Checking for Non-Disjunctive (Priority: MEDIUM)

**Goal:** Fast path for simple subclass queries

```rust
/// Optimized subclass check for non-disjunctive ontologies
pub fn is_subclass_of_fast(&self, sub: &IRI, sup: &IRI) -> bool {
    // Direct check from hierarchy (no reasoning needed)
    if let Some(parents) = self.hierarchy.direct_parents.get(sub) {
        if parents.contains(sup) {
            return true;
        }
    }
    
    // Transitive check using precomputed ancestors
    if let Some(ancestors) = self.transitive_cache.get(sub) {
        return ancestors.contains(sup);
    }
    
    // Fallback to BFS for unclassified classes
    self.bfs_subclass_check(sub, sup)
}

/// Precompute transitive closure for all classes
pub fn precompute_transitive_closure(&mut self) {
    // Use dynamic programming: 
    // ancestors[class] = direct_parents[class] ∪ ancestors[parent] for all parents
    
    let classes: Vec<_> = self.ontology.classes().collect();
    
    // Process in topological order
    for class in topological_order(&classes) {
        let mut ancestors = HashSet::new();
        
        if let Some(parents) = self.hierarchy.direct_parents.get(class.iri()) {
            for parent in parents {
                ancestors.insert(parent.clone());
                // Add parent's ancestors
                if let Some(parent_ancestors) = self.transitive_cache.get(parent) {
                    ancestors.extend(parent_ancestors.iter().cloned());
                }
            }
        }
        
        self.transitive_cache.insert(class.iri().clone(), ancestors);
    }
}
```

**Implementation:**
- [ ] Add transitive closure cache
- [ ] Add topological ordering
- [ ] Optimize BFS implementation

---

### Phase 5: Memory-Efficient Parallel Processing (Priority: MEDIUM)

**Goal:** Reduce memory overhead in SPACL for large ontologies

```rust
/// Memory-efficient branch processing
pub struct BranchProcessor {
    /// Shared reference to base ontology (Arc, never cloned)
    base_ontology: Arc<Ontology>,
    
    /// Differential updates per branch (minimal memory)
    branch_deltas: DashMap<BranchId, OntologyDelta>,
}

/// Delta represents changes from base ontology
pub struct OntologyDelta {
    added_axioms: Vec<Axiom>,
    added_assertions: Vec<ClassAssertion>,
}

impl BranchProcessor {
    /// Check consistency without cloning entire ontology
    pub fn check_consistent_with_delta(&self, branch_id: BranchId) -> OwlResult<bool> {
        let delta = self.branch_deltas.get(&branch_id).unwrap();
        
        // Create a view that combines base + delta
        let ontology_view = OntologyView {
            base: &self.base_ontology,
            delta: &delta,
        };
        
        // Check consistency using view
        self.check_consistency_view(ontology_view)
    }
}
```

**Implementation:**
- [ ] Create OntologyView for zero-copy ontology access
- [ ] Refactor SPACL to use deltas instead of clones
- [ ] Add memory pool for branch allocations

---

### Phase 6: Adaptive Timeout and Sampling (Priority: MEDIUM)

**Goal:** Smart timeout that adapts to ontology size

```rust
pub struct AdaptiveTimeout {
    pub base_timeout: Duration,
    pub class_count_factor: f64,
    pub complexity_factor: f64,
}

impl AdaptiveTimeout {
    pub fn calculate(&self, characteristics: &OntologyCharacteristics) -> Duration {
        let class_factor = (characteristics.class_count as f64 / 1000.0).sqrt();
        let complexity = characteristics.complexity_score;
        
        let multiplier = 1.0 + class_factor * self.class_count_factor 
                          + complexity * self.complexity_factor;
        
        self.base_timeout.mul_f64(multiplier)
    }
}

/// Classification with adaptive sampling
pub fn classify_with_sampling(&mut self, sample_rate: f64) -> OwlResult<ClassificationResult> {
    let classes: Vec<_> = self.ontology.classes().collect();
    let sample_size = (classes.len() as f64 * sample_rate) as usize;
    
    // Stratified sampling: pick representatives from different hierarchy levels
    let samples = self.stratified_sample(&classes, sample_size);
    
    // Classify samples
    let sample_results = self.classify_classes(&samples)?;
    
    // Infer results for remaining classes based on hierarchy proximity
    let inferred_results = self.infer_from_samples(&classes, &samples, &sample_results);
    
    Ok(inferred_results)
}
```

**Implementation:**
- [ ] Add adaptive timeout calculation
- [ ] Implement stratified sampling
- [ ] Add inference from samples

---

## Implementation Roadmap

### Week 1: Foundation
- [ ] Implement `OntologyCharacteristics` analyzer
- [ ] Add fast path detection
- [ ] Create benchmark suite for large ontologies

### Week 2: Hierarchical Classification
- [ ] Implement topological level computation
- [ ] Add parallel level processing
- [ ] Test on GO_Basic subset

### Week 3: Batch Processing
- [ ] Implement incremental classification
- [ ] Add progress tracking
- [ ] Add timeout handling

### Week 4: Memory Optimization
- [ ] Create OntologyView
- [ ] Refactor SPACL branch processing
- [ ] Memory profiling and optimization

### Week 5: Integration and Testing
- [ ] Integrate all optimizations
- [ ] Run full benchmark suite
- [ ] Performance comparison and documentation

---

## Expected Performance Improvements

| Metric | Current | Optimized | Improvement |
|--------|---------|-----------|-------------|
| GO_Basic (51K classes) | 858ms/class | 5-10ms/class | **85-170x** |
| Classification time | O(n²) | O(n log n) | **Theoretical** |
| Memory per branch | Full ontology | ~1KB delta | **1000x+** |
| Parallel efficiency | 20% | 80% | **4x** |

---

## Key Design Decisions

1. **Fast Path for Simple Ontologies**: Most real-world ontologies (GO, ChEBI) are primarily hierarchical with few disjunctions. The fast path handles 90%+ of cases.

2. **Topological Classification**: By processing hierarchically, we avoid redundant checks and enable natural parallelism.

3. **Incremental Processing**: Allows early termination and progress reporting, crucial for user experience.

4. **Memory Deltas**: Instead of cloning ontologies, track only changes. Massive memory savings for large ontologies.

5. **Adaptive Timeout**: Dynamic timeout based on actual complexity, not just class count.

---

## Testing Strategy

1. **Unit Tests**: Each optimization component
2. **Integration Tests**: Full classification on real ontologies
3. **Benchmark Suite**: Compare against baseline
4. **Memory Profiling**: Ensure no leaks or excessive allocation
5. **Correctness Verification**: Ensure optimizations don't change results

---

## Risk Mitigation

| Risk | Mitigation |
|------|------------|
| Incorrect fast path detection | Fallback to full reasoning on any doubt |
| Memory exhaustion | Streaming processing with configurable limits |
| Timeout too aggressive | User-configurable with sensible defaults |
| Race conditions in parallel code | Extensive testing with Miri and loom |

---

*Plan created: February 8, 2026*  
*Target completion: March 15, 2026*  
*Primary author: AI Assistant*
