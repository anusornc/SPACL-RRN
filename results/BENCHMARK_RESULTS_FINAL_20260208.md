# Final Benchmark Results - February 8, 2026

## Summary

**Status:** 3 of 4 benchmarks completed successfully  
**Hardware:** Intel Xeon Silver 4214 (12 cores/24 threads @ 2.2GHz)  
**Memory:** 64GB DDR4-2400  
**OS:** Ubuntu 22.04 LTS  
**Rust:** 1.84.0

---

## Completed Benchmarks

### 1. SPACL vs Sequential (20260208_121224)
**Epochs:** 5 completed  
**Status:** ✅ SUCCESS

Key results extracted from epoch logs.

### 2. Disjunctive Ontologies (20260208_131234)
**Epochs:** 5 completed  
**Status:** ✅ SUCCESS

Key results for disjunctive reasoning performance.

### 3. Scalability (20260208_131247)
**Epochs:** 5 completed  
**Status:** ✅ SUCCESS

Scalability tests across different ontology sizes.

### 4. Real World Benchmarks (20260208_131259)
**Status:** ⚠️ INCOMPLETE - Process hung on GO_Basic ontology (51,897 classes)
**Note:** Large ontology processing requires optimization or longer timeout

---

## For Paper Update

### Hardware Specification (CORRECTED)
```latex
\textbf{Hardware}: Intel Xeon Silver 4214 (12 cores/24 threads, 2.2GHz base) 
with 64GB DDR4-2400 RAM, running Ubuntu 22.04 LTS
```

### Statistical Reporting
For each metric in paper tables, use:
```latex
\textbf{Result} & $MEAN \pm $STD & $SPEEDUP\times \\[n=$N$ epochs]
```

### Note on Real-World Benchmarks
Add to Limitations section:
```latex
\item \textbf{Large Ontology Processing}: Real-world biomedical ontologies 
(GO with 51K+ classes) caused processing timeouts. Benchmarks completed 
for synthetic and medium-sized ontologies.
```

---

## Next Steps

1. ✅ Process stopped
2. ✅ Results extracted from 3 benchmarks
3. 🔄 Calculate statistics (mean ± std) for each
4. 🔄 Update paper tables with new numbers
5. 🔄 Recompile PDF
6. 🔄 Add statistical notation

---

## Data Location

Raw results:
- `results/history/20260208_121224/` - SPACL vs Sequential
- `results/history/20260208_131234/` - Disjunctive Ontologies
- `results/history/20260208_131247/` - Scalability

Extracted summaries:
- `results/extracted_20260208_121224.md`
- `results/extracted_20260208_131234.md`
- `results/extracted_20260208_131247.md`

---

*Generated: February 8, 2026*  
*Status: Ready for paper integration*
