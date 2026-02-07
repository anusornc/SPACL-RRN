# OWL2 DL → ALC/SHOIQ Scope Fixes

**Total Occurrences:** 17 in manuscript.tex

---

## Critical Fixes Required (SPACL Claims)

### Line 88 (Abstract) - MUST FIX
```diff
- We present SPACL (Speculative Parallel Tableaux with Adaptive Conflict Learning), 
- a novel OWL2 DL reasoner that achieves significant performance improvements
+ We present SPACL (Speculative Parallel Tableaux with Adaptive Conflict Learning), 
+ a novel ALC/SHOIQ reasoner that achieves significant performance improvements
```

### Line 92 (Abstract) - MUST FIX
```diff
- SPACL is the first open-source OWL2 DL reasoner to provide both speculative 
- parallelism and conflict-driven learning with adaptive threshold selection.
+ SPACL is the first open-source ALC/SHOIQ reasoner to provide both speculative 
+ parallelism and conflict-driven learning with adaptive threshold selection.
```

### Line 99 (Keywords) - MUST FIX
```diff
- OWL2 DL \sep Tableaux Reasoning \sep Parallel Algorithms
+ ALC/SHOIQ \sep Tableaux Reasoning \sep Parallel Algorithms
```

### Line 108 (Highlights) - MUST FIX
```diff
- \item First OWL2 DL reasoner combining speculative parallelism with nogood learning
+ \item First ALC/SHOIQ reasoner combining speculative parallelism with nogood learning
```

### Line 246 (Contributions) - MUST FIX
```diff
- \textbf{SPACL is the first system to integrate work-stealing parallelism with 
- nogood learning specifically for OWL2 DL tableau reasoning}
+ \textbf{SPACL is the first system to integrate work-stealing parallelism with 
+ nogood learning specifically for ALC/SHOIQ tableau reasoning}
```

### Line 456 (Figure) - MUST FIX
```diff
- \node[box] (input) {OWL2 DL\\Input};
+ \node[box] (input) {ALC/SHOIQ\\Input};
```

### Line 984 (Conclusion) - MUST FIX
```diff
- We presented SPACL, the first open-source OWL2 DL reasoner combining speculative 
- parallelism with conflict-driven learning.
+ We presented SPACL, the first open-source ALC/SHOIQ reasoner combining speculative 
+ parallelism with conflict-driven learning.
```

---

## Contextual References (CAN KEEP with modifications)

### Line 125 (Introduction - Contextual)
```diff
- with OWL2 DL serving as the W3C standard ontology language
+ with OWL2 DL serving as the W3C standard ontology language.
+ This work focuses on the ALC/SHOIQ fragment, which forms the basis of OWL2 DL.
```

### Line 167 (Section Title - Contextual)
```diff
- \subsection{Fundamental Challenges in OWL2 DL Reasoning}
+ \subsection{Fundamental Challenges in OWL2 DL Reasoning}
+ \label{sec:challenges}
+
+ Note: While the challenges apply to full OWL2 DL, our solution targets 
+ the ALC/SHOIQ fragment specifically.
```

### Line 169 (Contextual - Keep)
Keep as is: "OWL 2 DL (Web Ontology Language 2 Description Logic) reasoning..."
This is background context, not a claim about SPACL.

### Line 173 (Contextual - Keep)
Keep as is: "The primary obstacle to scalable OWL2 DL reasoning..."
This is discussing the general problem, not SPACL's capabilities.

### Line 193 (Contextual - Keep)
Keep as is: "less applicable to full OWL2 DL sound-and-complete tableau procedures"
This is about other approaches, not SPACL.

### Line 207 (Gap Analysis - Reword)
```diff
- \textbf{there is insufficient evidence in the literature that conflict-driven 
- nogood learning has been integrated into tableau-based OWL2 DL reasoners}
+ \textbf{there is insufficient evidence in the literature that conflict-driven 
+ nogood learning has been integrated into tableau-based ALC/SHOIQ reasoners}
```

### Line 209 (Gap Analysis - Reword)
```diff
- \textbf{there is no documented evidence of their integration into a single 
- adaptive, conflict-driven parallel tableau reasoner for OWL2 DL}.
+ \textbf{there is no documented evidence of their integration into a single 
+ adaptive, conflict-driven parallel tableau reasoner for ALC/SHOIQ}.
```

### Line 232 (Related Work - Keep)
Keep as is: "limited to EL expressivity (cannot handle OWL 2 DL features outside EL)"
This is about ELK, not SPACL.

### Line 280 (Formal Framework - Add Clarification)
```diff
- We operate in the Description Logic $\mathcal{ALC}$ (Attributive Language 
- with Complement), which forms the basis of OWL2 DL.
+ We operate in the Description Logic $\mathcal{ALC}$ (Attributive Language 
+ with Complement), which forms the basis of OWL2 DL. Our implementation 
+ extends to $\mathcal{ALCHOIQ}$ (ALC with role hierarchies, nominals, 
+ inverse roles, and qualified cardinality restrictions).
```

---

## Already Fixed (Limitations Section)

### Line 1012 - ALREADY CORRECT
```latex
\item \textbf{Profile Support}: SPACL currently supports ALC/SHOIQ. 
      Full OWL2 DL (with datatypes, keys) requires additional implementation.
```
This is already correct - it acknowledges the limitation.

---

## Summary of Changes

| Category | Count | Action |
|----------|-------|--------|
| **Critical (SPACL claims)** | 7 | Change to ALC/SHOIQ |
| **Gap analysis** | 2 | Change to ALC/SHOIQ |
| **Contextual background** | 5 | Keep, add clarifications |
| **Limitations (already correct)** | 1 | No change |
| **Keywords** | 1 | Change to ALC/SHOIQ |
| **Figure label** | 1 | Change to ALC/SHOIQ |
| **TOTAL** | **17** | **10 changes + 7 contextual** |

---

## Verification Commands

After fixes:
```bash
# Should return only contextual references
grep -n "OWL2 DL\|OWL 2 DL" paper/submission/manuscript.tex | \
  grep -v "standard\|general\|challenge\|problem\|W3C\|ELK"

# Should show ALC/SHOIQ claims
grep -c "ALC/SHOIQ\|ALCHOIQ" paper/submission/manuscript.tex
# Expected: 10+
```

---

## Post-Fix Checklist

- [ ] 7 critical SPACL claims changed to ALC/SHOIQ
- [ ] 2 gap analysis statements reworded
- [ ] Keywords updated
- [ ] Figure label updated
- [ ] Contextual references clarified
- [ ] PDF regenerates without errors
- [ ] grep for "OWL2 DL reasoner" returns 0 results
