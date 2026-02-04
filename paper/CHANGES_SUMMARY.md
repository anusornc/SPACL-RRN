# Summary of Changes - Journal Submission

## ⚠️ CRITICAL: Honest Reporting

**All fake/synthetic data has been removed from the paper.**

## What Has Been Actually Evaluated

### ✅ Real Measurements (Synthetic Hierarchies)
- **100, 500, 1,000, 5,000, 10,000 class hierarchies** - linear subclass chains
- Generated test data in `tests/data/hierarchy_*.owl`
- AMD Ryzen 9 5900X hardware
- Criterion.rs statistical framework (100 samples, 95% CI)

### ✅ LUBM Reference
- **univ-bench.owl** (13 classes) - Lehigh University Benchmark
- Used only for sanity testing

### ❌ NOT Evaluated (Honestly Disclosed)
- Real-world ontologies (NCI, GALEN, SNOMED CT)
- Direct comparison with Pellet/HermiT/Konclude on same hardware
- Inconsistent ontologies (nogood learning benefits underrepresented)

## Key Changes Made

### 1. Abstract - Honest About Synthetic Data
**Before:** "Comprehensive benchmarks show..."
**After:** "Comprehensive benchmarks on synthetic hierarchies (100--10,000 classes) show..."

### 2. Section 5.3 - Renamed and Clarified
**Before:** "Real-World Ontology Evaluation" (with fake data table)
**After:** "Planned Real-World Evaluation" (describing what will be done)

**Removed:** Table with fake NCI/GALEN/SNOMED CT performance numbers

**Added:** Honest description of what ontologies need to be downloaded and evaluated

### 3. Limitations - Expanded and Honest
**Added as FIRST limitation:**
> "Synthetic Benchmarks Only: Current evaluation uses synthetic linear hierarchies (100--10,000 classes). Real-world ontologies (NCI, GALEN, SNOMED CT from ORE benchmark suite) have not yet been evaluated. This is the most significant limitation of the current work."

### 4. Highlights - Clarified Scope
**Before:** "$5\times$ speedup at 10,000 classes"
**After:** "$5\times$ speedup at 10,000 classes (synthetic hierarchies)"

### 5. Conclusion - Honest About Scope
**Before:** "SPACL achieves $5\times$ speedup at 10,000 classes..."
**After:** "On synthetic hierarchies, SPACL achieves $5\times$ speedup at 10,000 classes..."

## What You Need to Do for Full Evaluation

### Step 1: Download ORE 2015 Benchmark Suite
```bash
# From: https://www.w3.org/wiki/Owl/Task_Forces/Reasoner/ORE_2015
# Download:
# - NCI Thesaurus (various sizes)
# - GALEN ontology
# - Gene Ontology (GO)
# - Other biomedical ontologies
```

### Step 2: Download BioPortal Ontologies
```bash
# From: https://bioportal.bioontology.org/
# Download OWL files for:
# - SNOMED CT (if license permits)
# - Other relevant ontologies
```

### Step 3: Implement OWL File Loading
Currently your benchmarks use generated hierarchies. You need to:
1. Add OWL/XML or RDF/XML parser
2. Convert to your internal representation
3. Run classification benchmarks

### Step 4: Run Actual Benchmarks
- Load each ontology
- Run 20 iterations with Criterion.rs
- Record mean ± std with 95% CI
- Compare sequential vs SPACL

### Step 5: Add Results to Paper
- Replace "Planned Real-World Evaluation" with actual results
- Remove limitation about synthetic benchmarks
- Update abstract and conclusion

## Files Changed

| File | Change |
|------|--------|
| `manuscript.tex` | Removed fake data, added honest disclosures |
| `references.bib` | Added LUBM reference, removed fake NCI ref |
| `manuscript.pdf` | Regenerated (23 pages, 490KB) |

## Package Location
```
/Users/anusornchaikaew/Work/Phd/Tableaux/tableauxx/paper/jws_submission_official.zip (550KB)
```

## Recommendation

**Current Status**: The paper is now honest about its limitations. The synthetic benchmarks still demonstrate:
- The speculative parallelism mechanism works
- The adaptive threshold is effective
- The nogood learning infrastructure exists

**For Journal Submission**: You have two options:
1. **Submit as-is** with honest limitations, promising real-world evaluation in future work
2. **Do the real-world evaluation first** (recommended), then submit with complete results

Option 2 is strongly recommended for a top-tier journal like JWS.
