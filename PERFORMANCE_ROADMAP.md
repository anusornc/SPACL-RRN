# SPACL Performance Optimization Roadmap

**Goal:** Enable efficient handling of 100K+ class ontologies before paper submission.

**Current Status:** 
- ✅ Small ontologies (<1K): Excellent performance (4,000x speedup)
- ✅ Medium ontologies (1K-10K): Good with auto-mode (375x speedup)
- ❌ Large ontologies (>10K): Parser bottleneck (5.8s load time)

---

## Phase 1: Binary Serialization Format (Days 1-2) ✅ COMPLETE

### Objective
Create a fast binary format to avoid OWL/XML parsing overhead.

### Implementation ✅
```rust
// Module: src/serializer/binary.rs

pub struct BinaryOntologyFormat;

impl BinaryOntologyFormat {
    pub fn serialize(ontology: &Ontology, writer: &mut impl Write) -> Result<()>;
    pub fn deserialize(reader: &mut impl Read) -> Result<Ontology>;
}
```

### Commands ✅
```bash
# Convert OWL to binary (one-time)
owl2-reasoner convert input.owl output.owlbin

# Load binary directly (auto-detected)
owl2-reasoner check output.owlbin
```

### Results
| Metric | OWL/XML | Binary | Speedup | Status |
|--------|---------|--------|---------|--------|
| Load 10K classes | 6.2s | 3.5s | **1.8x** | ✅ Implemented |
| Load 100K classes | Timeout | TBD | TBD | 🔄 Next phase |
| File size | 0.50MB | 0.52MB | 104% | ℹ️ Slightly larger |

### Lessons Learned
- Binary format is 1.8x faster, not 100x as hoped
- Bottleneck is IRI reconstruction, not file reading
- Need memory-mapped IRI cache for further optimization

---

## Phase 2: Streaming Parser (Week 1-2) 🔄 IN PROGRESS

### Objective
Parse OWL files incrementally without loading entire file into memory.

### Status
- ✅ Module structure created (`src/parser/streaming/`)
- ✅ Added quick-xml dependency
- ⚠️ **Blocked**: Borrow checker issues with event-based parsing

### Challenge
quick-xml's `read_event_into(&mut self.buf)` borrows buffer mutably, 
preventing calls to helper methods on `self`. Need refactoring approach.

### Options
1. **State Machine Parser**: Parse without helper methods (complex)
2. **Simpler Line Parser**: Parse line-by-line for basic SubClassOf (limited)
3. **Use Existing Parser**: Optimize current parser instead

### Memory Target
- Current: Load 5.5MB into String (~50MB+ peak)
- Target: Buffer 8KB chunks (~10MB max)
- Reduction: **5x less memory**

### Recommendation
**Skip Phase 2 for now**, use Phase 3 (Memory Profiling) instead to 
optimize the current parser. Return to streaming if memory is still 
problematic after optimization.

---

## Phase 3: Memory Profiling & Optimization (Week 2-3) 🔄 IN PROGRESS

### Completed ✅
- Memory profiling infrastructure (`src/util/profiling/`)
- Automatic IRI cache sizing based on file size
- Binary format with 1.8x speedup

### Results So Far
| Ontology | OWL/XML Load | Binary Load | Status |
|----------|--------------|-------------|--------|
| 1K classes | 35ms | 30ms | ✅ Fast |
| 10K classes | 5.9s | 3.5s | ✅ Good |
| 100K classes | 150s | >60s | ❌ Too slow |

### Bottleneck Analysis
The 100K ontology loading is still too slow. Root causes:
1. **IRI reconstruction** - Creating IRI objects is expensive
2. **Ontology building** - `ontology.add_class()` has overhead
3. **String duplication** - Each IRI string is stored multiple times

### Next Optimizations
1. **Lazy IRI loading** - Defer IRI creation until needed
2. **Batch ontology operations** - Add classes in bulk
3. **Memory-mapped files** - For binary format
4. **Parallel parsing** - Multi-threaded XML parsing

### Revised Targets
| Goal | Current | Target |
|------|---------|--------|
| 100K load time | 150s | <10s |
| Memory usage | Unknown | <2GB |
| Peak RSS | Unknown | <1.5GB |

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
