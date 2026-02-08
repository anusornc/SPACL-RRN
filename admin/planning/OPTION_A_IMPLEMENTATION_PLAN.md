# Option A: Full Codebase Fix + Manuscript Revision

## Timeline: 4-5 Weeks

---

## Week 1-2: Nogood Learning Correctness

### Goal: Prove or fix nogood soundness

**Day 1-2: Audit Current Implementation**
- [ ] Trace nogood extraction from contradiction
- [ ] Identify all code paths that create nogoods
- [ ] Document current implementation

**Day 3-4: Formal Specification**
- [ ] Write down nogood definition formally
- [ ] Define soundness criterion
- [ ] Identify required invariants

**Day 5-7: Implementation Fix (if needed)**
- [ ] Add dependency tracking OR
- [ ] Restrict nogoods to specific safe cases OR
- [ ] Add runtime soundness checks

**Day 8-10: Testing**
- [ ] Create test suite for nogood correctness
- [ ] Test on known unsatisfiable ontologies
- [ ] Verify no false positives (unsound pruning)
- [ ] Verify no false negatives (missed pruning)

**Deliverable:** 
- Working, tested nogood learning
- Document proving soundness
- Test coverage report

---

## Week 3: SHOIQ Completeness

### Goal: Implement missing features for claimed SHOIQ support

**Current Status Check:**
- [ ] What SHOIQ features exist?
- [ ] What needs implementation?
- [ ] What's stubbed vs working?

**Implementation Priority:**
1. **Nominals** (OneOf, {a,b,c}) - if not implemented
2. **Cardinality restrictions** (min/max/exact) - partial
3. **Role hierarchies** - check if working
4. **Transitive roles** - verify implementation
5. **Inverse properties** - check completeness

**Testing:**
- [ ] Create SHOIQ test ontologies
- [ ] Verify termination (blocking)
- [ ] Test against reference reasoner

**Deliverable:**
- Complete SHOIQ implementation
- Feature matrix vs standard
- Test suite passing

---

## Week 4: Integration & Benchmarking

### Goal: Solid integration tests and benchmarks

**Integration Tests:**
- [ ] ORE-style benchmark suite
- [ ] OWL2 Test Cases
- [ ] Stress tests (10K, 50K, 100K)
- [ ] Correctness oracle comparison

**Benchmarking:**
- [ ] HermiT comparison (all tests)
- [ ] Pellet comparison (if working)
- [ ] ELK comparison (for hierarchies)
- [ ] Document all results

**Deliverable:**
- Passing test suite
- Benchmark report
- Performance validation

---

## Week 5: Manuscript Revision

### Goal: Update paper with solid foundation

**Section Updates:**
1. **Abstract**: Accurate scope (SHOIQ), key numbers
2. **Introduction**: Honest positioning
3. **Related Work**: Distinguish from competitors
4. **Algorithm**: Formal correctness proofs
5. **Implementation**: Feature matrix
6. **Evaluation**: New benchmark data

**New Sections:**
- [ ] "Supported Logic Fragment" - detailed breakdown
- [ ] "Correctness" - formal arguments
- [ ] "Comparison with Established Reasoners" - HermiT/Pellet

**Figures/Tables:**
- [ ] Updated feature comparison table
- [ ] Competitor benchmark table
- [ ] Correctness test results

**Deliverable:**
- Complete revised manuscript
- Supplementary materials
- Reproducibility package

---

## Detailed Task List by Component

### Nogood Learning (Week 1-2)

```rust
// Current implementation to audit:
fn extract_nogood(&self, clash: &Clash) -> Nogood {
    // What exactly goes into the nogood?
    // Is it minimal?
    // Is it sound?
}
```

**Tests Needed:**
- [ ] Nogood is subset of clashing assertions
- [ ] Nogood actually implies contradiction
- [ ] No over-generalization
- [ ] Subsumption check works correctly

### SHOIQ Features (Week 3)

**Feature Matrix:**
| Feature | Status | Test Coverage |
|---------|--------|---------------|
| ALC | ✅ Working | Full |
| H (role hierarchy) | ? | ? |
| O (nominals) | ? | ? |
| I (inverse roles) | ? | ? |
| Q (cardinality) | Partial | Partial |
| Blocking | ? | ? |

**Implementation Tasks:**
- [ ] Audit current implementation
- [ ] Implement missing features
- [ ] Add comprehensive tests
- [ ] Document limitations

### Benchmarking (Week 4)

**Competitor Tests:**
```bash
# Already have:
./benchmarks/competitors/scripts/comprehensive_benchmark.sh

# Need to add:
- ELK (for hierarchies)
- Konclude (if possible)
- ORE benchmark suite
- OWL2 DL test cases
```

**Metrics:**
- [ ] Correctness (pass/fail vs reference)
- [ ] Performance (time, memory)
- [ ] Scalability (10K, 50K, 100K)
- [ ] Feature coverage

---

## Risk Mitigation

### Risk 1: Nogood unsound → can't fix easily
**Mitigation:** Have backup plan to scope down to work-stealing only

### Risk 2: SHOIQ features too complex
**Mitigation:** Scope to ALC+ if needed, document as future work

### Risk 3: Competitor benchmarks don't look good
**Mitigation:** Honest reporting, focus on disjunctive strength

### Risk 4: Timeline slips
**Mitigation:** Weekly checkpoints, scope reduction options identified

---

## Success Criteria

Before resubmitting, we MUST have:

✅ **Nogood learning**: Tested, documented, provably sound (or removed)
✅ **SHOIQ support**: Implemented features work correctly
✅ **Correctness**: Passes reference test suite
✅ **Benchmarks**: Documented comparison with HermiT/Pellet
✅ **Manuscript**: Honest scope, no contradictions, full reproducibility

---

## Checkpoint Schedule

| Week | Checkpoint | Go/No-Go |
|------|-----------|----------|
| Week 1 | Nogood audit complete | Decide: fix or remove |
| Week 2 | Nogood tests passing | Green light for SHOIQ |
| Week 3 | SHOIQ features working | Green light for benchmarking |
| Week 4 | All benchmarks done | Green light for writing |
| Week 5 | Manuscript complete | Ready to resubmit |

---

## First Task: Nogood Audit

Let me start with the nogood correctness audit:

1. Read current nogood extraction code
2. Identify all creation points
3. Write formal specification
4. Design test suite
5. Fix if needed

Ready to start?
