# SPACL Performance Optimization Roadmap

**Goal:** Enable efficient handling of 100K+ class ontologies before paper submission.

**Current Status:** 
- ✅ Small ontologies (<1K): Excellent performance (4,000x speedup)
- ✅ Medium ontologies (1K-10K): Good with auto-mode (375x speedup)
- ❌ Large ontologies (>10K): Parser bottleneck (5.8s load time)

---

## Phase 1: Binary Serialization Format (Days 1-2)

### Objective
Create a fast binary format to avoid OWL/XML parsing overhead.

### Implementation
```rust
// New module: src/serializer/binary.rs

pub struct BinaryOntologyFormat;

impl BinaryOntologyFormat {
    /// Serialize ontology to binary format
    pub fn serialize(ontology: &Ontology, writer: &mut impl Write) -> Result<()>;
    
    /// Deserialize from binary format
    pub fn deserialize(reader: &mut impl Read) -> Result<Ontology>;
}
```

### Format Design
```
[Header]
- Magic: "OWLB" (4 bytes)
- Version: u32
- Class count: u64
- Property count: u64
- Axiom count: u64

[Class Section]
- IRI strings (length-prefixed)

[Property Section]
- Object properties
- Data properties

[Axiom Section]
- Axiom type: u8
- Serialized axiom data
```

### Commands to Add
```bash
# Convert OWL to binary (one-time)
owl2-reasoner convert input.owl output.owlbin

# Load binary directly
owl2-reasoner check output.owlbin
```

### Expected Performance
| Metric | Current (OWL) | Target (Binary) | Speedup |
|--------|---------------|-----------------|---------|
| Load 10K classes | 5.8s | ~50ms | **116x** |
| Load 100K classes | Timeout | ~500ms | **∞** |
| File size | 5.5MB | ~2MB | **2.75x smaller** |

---

## Phase 2: Streaming Parser (Week 1-2)

### Objective
Parse OWL files incrementally without loading entire file into memory.

### Implementation
```rust
// New module: src/parser/streaming.rs

pub struct StreamingOntologyParser<R: Read> {
    reader: R,
    buffer: Vec<u8>,
    // ...
}

impl<R: Read> StreamingOntologyParser<R> {
    /// Create new streaming parser
    pub fn new(reader: R) -> Self;
    
    /// Parse next axiom without loading entire file
    pub fn next_axiom(&mut self) -> Result<Option<Axiom>>;
    
    /// Process axioms incrementally
    pub fn parse_incremental<F>(&mut self, handler: F) -> Result<()>
    where F: FnMut(Axiom) -> Result<()>;
}
```

### Memory Optimization
- Current: Load 5.5MB into String + parse to AST (~50MB+)
- Streaming: Buffer 8KB chunks, parse axioms one at a time (~10MB max)

### Progress Reporting
```bash
Loading ontology: large.owl
[====================>  ] 85% (85,000/100,000 classes)
```

---

## Phase 3: Memory Profiling & Optimization (Week 2-3)

### Tools to Use
```bash
# Memory profiling
cargo install cargo-valgrind
cargo valgrind --bin owl2-reasoner -- check large.owl

# Heap analysis
cargo install dhat
cargo run --features dhat-heap --bin owl2-reasoner -- check large.owl
```

### Targets
1. **IRI Storage**: Use string interning for repeated IRIs
2. **Axiom Storage**: Use arena allocators
3. **Reasoner State**: Minimize cloning during speculation

### Expected Improvements
- Memory usage: 50% reduction
- Clone operations: 80% reduction

---

## Phase 4: Benchmark 100K+ Classes (Week 3-4)

### Test Suite
```bash
# Generate test ontologies
scripts/generate_large_ontologies.py --sizes 50K,100K,250K,500K

# Benchmark all
./benchmarks/competitors/scripts/comprehensive_benchmark.sh
```

### Success Criteria
| Ontology Size | Load Time | Reasoning Time | Memory |
|---------------|-----------|----------------|--------|
| 50K classes | < 300ms | < 100ms | < 500MB |
| 100K classes | < 500ms | < 200ms | < 1GB |
| 250K classes | < 1s | < 500ms | < 2GB |

---

## Phase 5: Update Paper (Week 4-5)

### New Claims to Make
```latex
% Stronger claims with optimized implementation
\begin{itemize}
    \item Sub-second loading for ontologies up to 100K classes
    \item 1,000x+ speedup over HermiT for all tested sizes
    \item Memory-efficient: <1GB for 100K class ontologies
    \item Handles BioPortal-scale ontologies (GO, UBERON, PATO)
\end{itemize}
```

### New Tables to Add
1. **Scalability to 100K+ classes**
2. **Memory usage comparison**
3. **BioPortal real-world benchmark** (with optimized parser)

---

## Implementation Schedule

| Week | Phase | Deliverable |
|------|-------|-------------|
| Week 1 | Phase 1 | Binary serialization working |
| Week 2 | Phase 2 | Streaming parser prototype |
| Week 3 | Phase 2+3 | Optimized parser + profiling |
| Week 4 | Phase 4 | 100K+ benchmarks |
| Week 5 | Phase 5 | Updated paper with results |

---

## Risk Mitigation

| Risk | Mitigation |
|------|------------|
| Binary format breaks | Version field + backward compatibility tests |
| Streaming too complex | Fall back to binary-only for large files |
| Memory still high | Document limitations, focus on 50K max |

---

## Success Metrics

✅ **Ready to Publish When:**
1. Can load 100K class ontology in < 1 second
2. Reasoning on 100K classes completes in < 5 seconds
3. Memory usage < 2GB for 100K classes
4. All BioPortal test ontologies load successfully

---

## Current Blockers to Address

1. **Parser Performance**
   - [ ] Profile current parser bottlenecks
   - [ ] Implement binary format
   - [ ] Implement streaming for OWL/XML

2. **SPACL on Large Hierarchies**
   - [ ] Verify auto-mode handles 100K correctly
   - [ ] Consider hierarchical optimization for taxonomies

3. **Test Infrastructure**
   - [ ] Generate 50K, 100K, 250K test ontologies
   - [ ] Set up CI for large file benchmarks

---

**Start Date:** Today  
**Target Paper Submission:** After Phase 5 complete (Week 5)  
**Buffer:** +1 week for unexpected issues

Let's build something worth publishing! 🚀
