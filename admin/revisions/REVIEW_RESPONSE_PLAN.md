# Response Plan for Journal Review

## Review Outcome: Major Revision Recommended

The Codex review identified **valid issues** that need addressing before resubmission.

---

## 🔴 Critical Issues (Must Fix)

### 1. Logic Fragment Scope
**Current:** Claims "OWL2 DL" support
**Reality:** ALC/SHOIQ implementation
**Fix:** 
- [ ] Change title/abstract to "ALC/SHOIQ Reasoner"
- [ ] Add explicit section: "Supported Logic Fragment"
- [ ] Remove OWL2 DL claims, add "with pathway to full OWL2 DL"

### 2. Contradictory Experimental Claims
**Issue:** "Planned but not evaluated" vs results table
**Fix:**
- [ ] Clarify which BioPortal results are preliminary vs measured
- [ ] Add timeline: "Initial results" vs "Full evaluation"
- [ ] Fix README to match manuscript

### 3. Add Our New Benchmark Data
**What we have:**
- 535x speedup vs HermiT (disjunctive)
- 1.4x speedup on 10K hierarchies (binary format)
- Competitor comparison on 7 test ontologies
**Fix:**
- [ ] Add new section: "Comparison with Established Reasoners"
- [ ] Include HermiT/Pellet benchmark table
- [ ] Reference `BENCHMARK_ANALYSIS_2026_02_06.md`

---

## 🟡 Important Issues (Should Fix)

### 4. Reframe Contribution
**Current:** "General purpose OWL2 DL reasoner"
**Better:** "Optimized reasoner for disjunctive ontologies"

**New positioning:**
```
SPACL achieves 535x speedup on disjunctive ontologies vs HermiT.
For taxonomic hierarchies, use sequential mode or established reasoners.
```

### 5. Fix Parsing Time Claims
**Issue:** "30+ minutes" vs "100-1000ms" in tables
**Fix:**
- [ ] Clarify: 30min = XML loading + parsing
- [ ] 1000ms = reasoning only (parsing excluded)
- [ ] Separate "ontology loading" from "reasoning time" metrics

### 6. Address Nogood Soundness
**Options:**
- [ ] Add formal proof of nogood extraction (preferred)
- [ ] Or: Remove nogood claims, focus on work-stealing only
- [ ] Or: Add "preliminary nogood learning" with caveats

---

## 🟢 Minor Issues (Nice to Fix)

### 7. Writing Improvements
- [ ] Remove "super-linear speedup" claim
- [ ] Fix "ALC forms basis of OWL2 DL" → be precise
- [ ] Remove subjective terms ("production-quality", "honest")
- [ ] Clarify crossbeam vs rayon usage

### 8. Reproducibility
- [ ] Add commit ID to manuscript
- [ ] Include benchmark generation scripts
- [ ] Link to repository with full artifact

---

## 📋 Revised Timeline

| Week | Task |
|------|------|
| Week 1 | Fix scope claims (OWL2 DL → ALC/SHOIQ) |
| Week 2 | Add new benchmark section with HermiT/Pellet data |
| Week 3 | Fix contradictions, clarify parsing vs reasoning |
| Week 4 | Add nogood soundness proof or reframe |
| Week 5 | Writing polish, reproducibility package |

---

## 🎯 Key Message for Resubmission

**Title Option:**
"SPACL: Speculative Parallelism and Conflict Learning for Scalable ALC/SHOIQ Reasoning"

**Abstract Lead:**
"We present SPACL, the first open-source ALC/SHOIQ reasoner combining speculative work-stealing parallelism with adaptive conflict learning. On disjunctive ontologies, SPACL achieves 535× speedup over HermiT (6ms vs 3,269ms)."

**Honest Positioning:**
- ✅ Excellent for disjunctive ontologies
- ✅ First work-stealing + nogood combination
- ⚠️ Sequential preferred for pure hierarchies

---

## 💡 Strengths to Emphasize

1. **535x speedup on disjunctive** - validated against HermiT
2. **First open-source** work-stealing + nogood
3. **Adaptive threshold** - correctly identifies when to use sequential
4. **Binary format** - 2.1x loading improvement
5. **Rust implementation** - memory safety, performance

---

## ❓ Questions for Authors

1. Can we prove nogood soundness, or should we scope it down?
2. Should we add ELK comparison (optimized for hierarchies)?
3. Do we have time to implement blocking for full SHOIQ?

---

**Bottom Line:** The core contribution is **solid**. Fix the scope overstatement, add competitor benchmarks, and resubmit as Major Revision.
