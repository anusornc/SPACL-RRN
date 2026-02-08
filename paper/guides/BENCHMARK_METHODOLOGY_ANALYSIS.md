# Benchmark Methodology Analysis & Recommendations

## Executive Summary

**Date:** February 8, 2026  
**Status:** Hardware corrected, methodology recommendations provided

---

## 1. Hardware Correction Applied

### Previous Issue
- **Paper claimed:** AMD Ryzen 9 5900X (3.7-4.8 GHz)
- **Actual system:** Intel Xeon Silver 4214 (2.2 GHz)
- **Impact:** Serious reproducibility issue

### Correction Made
Updated `paper/submission/manuscript.tex`:
```latex
% Line 750
\textbf{Hardware}: Intel Xeon Silver 4214 (12 cores/24 threads, 2.2GHz base) 
with 64GB DDR4-2400 RAM, running Ubuntu 22.04 LTS

% Line 774 (was Apple Silicon - now consistent)
\textbf{Hardware}: Same as Section~\ref{sec:setup}: Intel Xeon Silver 4214...
```

**Note:** This is a significant hardware change that may affect:
- Absolute performance numbers (Xeon is ~40-50% slower than Ryzen)
- Speedup ratios (may remain similar if sequential baseline also slower)
- Power efficiency comparisons

---

## 2. Multiple Benchmark Runs: SHOULD YOU DO IT?

### Answer: **YES, ABSOLUTELY**

Here's why multiple runs (epochs) are essential:

#### A. Statistical Significance

**Single Run Problems:**
- ❌ Subject to system noise (background processes)
- ❌ Doesn't account for thermal throttling
- ❌ Can't detect outliers
- ❌ No confidence intervals
- ❌ Reviewers will question reliability

**Multiple Runs (3-5 epochs) Provide:**
- ✅ Mean and standard deviation
- ✅ Confidence intervals (95% CI)
- ✅ Detection of outliers
- ✅ Statistical validation
- ✅ Reproducibility verification

#### B. Academic Standards

**Top-tier conferences/journals expect:**
- Multiple runs (typically 5-10)
- Statistical analysis (mean ± std)
- Variance reporting
- Outlier handling

**Examples from literature:**
- ORE Workshop: 10 runs, discard outliers
- ISWC papers: 5-10 runs, report median
- Journal of Web Semantics: Statistical significance tests

#### C. Practical Benefits

| # Epochs | Confidence | Use Case |
|----------|------------|----------|
| 1 | Low | Quick testing only |
| 3 | Medium | Development, exploration |
| 5 | High | **Conference submission** |
| 10 | Very High | Journal submission, final results |
| 30+ | Publication | Only for specific statistical claims |

---

## 3. Recommended Benchmark Protocol

### For Paper Submission (Minimum Viable)

```
For EACH benchmark configuration:
├── Run 5 epochs (or 10 for critical claims)
├── Discard warm-up runs (first 1-2 epochs)
├── Calculate: mean, std, 95% CI
├── Report: mean ± std [min, max]
└── Note: System configuration, load
```

### Detailed Protocol

#### Phase 1: System Preparation
1. **Isolate system**
   - Close unnecessary applications
   - Disable CPU frequency scaling (optional)
   - Monitor background processes

2. **Warm-up**
   - Run 2-3 "throwaway" epochs
   - Allow CPU to reach steady-state temperature
   - Cache warm-up (if applicable)

#### Phase 2: Data Collection
3. **Run 5-10 epochs**
   - Record each result separately
   - Note any system events
   - Check for outliers (>2 std from mean)

4. **Validation**
   - Coefficient of Variation (CV) < 10% desired
   - If CV > 20%, investigate (thermal, load, etc.)

#### Phase 3: Analysis
5. **Statistics**
   - Calculate mean, std, median
   - 95% confidence intervals
   - Identify and justify outliers

6. **Reporting**
   - Mean ± standard deviation
   - Range [min, max]
   - Sample size (n=5 or n=10)

---

## 4. What Should You Do NOW?

### Option A: Re-run All Benchmarks (Recommended if time permits)

**Time Required:** 2-3 days

**Steps:**
1. Use new script: `./scripts/run_benchmark_v2.sh <name> 5`
2. Run for EACH benchmark in paper:
   - spacl_vs_sequential
   - disjunctive_ontologies
   - hierarchy_scaling
   - LUBM
3. Collect 5 epochs for each
4. Calculate new statistics
5. Update ALL tables in paper

**Pros:**
- Statistically sound results
- Reviewer confidence
- Reproducible methodology

**Cons:**
- Time consuming
- Numbers may change significantly

### Option B: Use Existing Results with Caveats

**If you must submit soon:**

1. **Document current methodology**
   - State: "Results from single runs due to time constraints"
   - Add: "Multi-epoch validation planned for camera-ready"

2. **Add variance estimates**
   - If you have ANY repeat runs, show consistency
   - Use Criterion.rs variance (if available)

3. **Acknowledge limitation**
   - Add to Limitations section
   - Explain statistical validation planned

---

## 5. Updated Hardware Impact Analysis

### Performance Expectations

| Metric | AMD Ryzen 9 5900X (Claimed) | Intel Xeon Silver 4214 (Actual) | Impact |
|--------|------------------------------|----------------------------------|--------|
| **Base Clock** | 3.7 GHz | 2.2 GHz | ~40% slower |
| **Boost Clock** | 4.8 GHz | ~3.2 GHz | ~33% slower |
| **IPC** | Zen 3 (high) | Skylake (moderate) | ~15-20% lower |
| **Expected Slowdown** | - | - | **~50-60% slower** |

### What This Means

**If numbers were from AMD:**
- Actual Xeon results would be ~50% slower
- Speedup ratios (595×, 686×) might remain similar
  - If both sequential AND parallel slow down equally
  - Ratio = (slow_seq / slow_parallel) ≈ (fast_seq / fast_parallel)
- Absolute times would increase significantly

**Critical Decision:**
1. Re-run everything on Xeon with proper epochs
2. OR find AMD system and re-run there
3. OR update paper to reflect Xeon results

---

## 6. My Recommendation

### Before Submission - MUST DO:

1. **Fix hardware in paper** ✅ DONE
   - Updated to Intel Xeon Silver 4214
   - Consistent throughout document

2. **Run proper benchmarks**
   ```bash
   # For each benchmark in paper:
   ./scripts/run_benchmark_v2.sh spacl_vs_sequential 5
   ./scripts/run_benchmark_v2.sh disjunctive_ontologies 5
   # ... etc
   ```

3. **Update ALL performance numbers**
   - Tables, figures, abstract, conclusion
   - Recalculate speedups
   - Add statistical measures (mean ± std)

4. **Add to Limitations**
   ```latex
   \item \textbf{Benchmark Duration}: Due to computational constraints, 
   benchmarks were run with 5 epochs. Longer statistical validation 
   (30+ epochs) would provide tighter confidence intervals.
   ```

5. **Document methodology**
   - Section 5.1.2 already has protocol
   - Add: "Each benchmark run consists of 5 epochs; 
   reported results show mean ± standard deviation"

---

## 7. Time Estimate

| Task | Time | Priority |
|------|------|----------|
| Re-run benchmarks (5 epochs × 5 configs) | 1-2 days | **Critical** |
| Update all tables in paper | 4-6 hours | **Critical** |
| Recompile PDF | 30 min | Required |
| Proofread changes | 2-3 hours | Required |
| **Total** | **2-3 days** | - |

---

## 8. Bottom Line

**Question:** Should you run multiple benchmarks?

**Answer:** **YES - Absolutely Required**

- Single-run benchmarks are NOT acceptable for publication
- Multiple epochs (5-10) provide statistical validity
- Current hardware correction requires re-running anyway
- 2-3 days investment prevents rejection at review

**Without proper multi-epoch benchmarks:**
- ❌ Reviewers will question reliability
- ❌ Results may not be reproducible
- ❌ Risk of rejection or revision request
- ❌ Academic credibility damage

**With proper multi-epoch benchmarks:**
- ✅ Statistical confidence
- ✅ Reproducible results
- ✅ Reviewer trust
- ✅ Publication-ready quality

---

## Next Steps

1. ✅ Hardware corrected in paper
2. 🔄 Run all benchmarks with 5 epochs
3. 🔄 Update performance numbers
4. 🔄 Add statistical reporting
5. 🔄 Recompile and verify

**Estimated completion:** 2-3 days

---

*Analysis prepared: February 8, 2026*  
*By: Project Assistant*  
*Status: Hardware fixed, awaiting benchmark re-runs*
