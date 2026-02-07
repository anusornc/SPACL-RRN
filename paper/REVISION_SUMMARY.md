# PAPER REVISION SUMMARY

## The Problem

Your manuscript contains **36 citations** that were NOT verified against the actual papers. This is a serious academic integrity issue.

## What I've Created For You

### 1. PAPER_REVISION_GUIDE.md
Complete guide with:
- ✅ Search terms for all 36 citations
- ✅ Database locations (IEEE, ScienceDirect, etc.)
- ✅ Priority order (which papers to read first)
- ✅ Specific claims to verify
- ✅ Database access instructions

### 2. MARKED_SECTIONS.txt
Line-by-line breakdown of manuscript showing:
- Every citation that needs verification
- The specific claim made
- Whether it's CRITICAL or optional
- Line numbers for easy reference

### 3. ACADEMIC_INTEGRITY_WARNING.md
Explanation of why this matters and consequences of not fixing it.

---

## Your Action Plan

### Phase 1: Fix Critical Errors (1-2 days)

**CRITICAL ISSUE 1: Wrong Citation for Chase-Lev Algorithm**
```
Location: manuscript.tex line 554
Problem: You cite Arora 1998 for Chase-Lev algorithm
Fix: Add citation for Chase & Lev 2005
```
**Search:** "Dynamic Circular Work-Stealing Deque Chase Lev 2005"

**CRITICAL ISSUE 2: Wrong Citation for ELK**
```
Location: manuscript.tex line 242
Problem: You cite Bate 2018 for ELK, but that paper is about consequence-based reasoning
Fix: Find correct ELK citation (probably Kazakov 2012)
```

**CRITICAL ISSUE 3: Duplicate Pellet Citations**
```
Location: manuscript.tex lines 136, 236
Problem: You have both \cite{pellet} and \cite{sirin2007pellet}
Fix: Check if same paper, remove duplicate
```

### Phase 2: Verify Numbers (2-3 days)

These specific numbers MUST be verified:

| Your Claim | Citation | Action |
|------------|----------|--------|
| "approximately 10x speedups" | gu2015cichlid | Find exact number in paper |
| "96.9% reduction" | wang2019comr | Verify exact percentage |
| "161x speedups" | algahtani2024mp | Verify exact number |
| "5-50x faster" | parsia2015ore | Verify exact range |
| "dominant source of complexity" | faddoul2015fork | Find exact quote + page |

### Phase 3: Read Core Papers (1 week)

**Priority 1 (9 papers):**
1. owl2 - W3C standard
2. dlhandbook - DL theory
3. pellet - Pellet reasoner
4. hermit - HermiT reasoner
5. konclude - Konclude reasoner
6. motik2009hypertableau - Hypertableau algorithm
7. marques1999grasp - GRASP/CDCL
8. quan2017parallel - Parallel OWL reasoning
9. quan2019framework - Framework paper

**Priority 2 (7 papers):**
10. faddoul2015fork - Fork/Join
11. steigmiller2020parallelised - Parallel ABox
12. glorian2020nacre - NACRE (CRITICAL for your gap claim)
13. gu2015cichlid - Cichlid (verify 10x number)
14. wang2019comr - ComR (verify 96.9%)
15. algahtani2024mp - PhD thesis (verify 161x)
16. parsia2015ore - ORE benchmarks (verify 5-50x)

### Phase 4: Read Supporting Papers (1 week)

Remaining 20 papers - verify as needed.

---

## How to Search

### For IEEE Papers:
1. Connect to university VPN
2. Go to https://ieeexplore.ieee.org/
3. Search using terms from PAPER_REVISION_GUIDE.md
4. Download PDF

### For ScienceDirect (Elsevier):
1. Connect to university VPN
2. Go to https://sciencedirect.com/
3. Search using terms from guide
4. Download PDF

### For Springer:
1. Connect to university VPN
2. Go to https://link.springer.com/
3. Search using DOI from guide
4. Download PDF

### For Free Papers:
- CEUR-WS: http://ceur-ws.org/
- JAIR: https://jair.org/
- arXiv: https://arxiv.org/

---

## What to Do With Each Paper

### Step 1: Download
Use search terms from guide to find and download PDF.

### Step 2: Find Relevant Section
Use Ctrl+F to search for keywords from your citation.

### Step 3: Verify the Claim
Ask yourself:
- Does the paper actually say what I claimed?
- Is my interpretation accurate?
- Are my specific numbers correct?

### Step 4: Fix Manuscript
If needed:
- Rewrite claim to match paper
- Add page number for quotes
- Correct specific numbers
- Remove citation if paper doesn't support claim

---

## Example Verification

### Example 1: Cichlid Speedup

**Your claim:** "approximately 10x average speedups"

**Action:**
1. Search IEEE Xplore: "Cichlid Large Scale RDFS OWL Reasoning Spark Gu 2015"
2. Download PDF
3. Find results/experiments section
4. Look for speedup numbers

**Possible outcomes:**
- ✓ Paper says "approximately 10x" → Keep citation
- ✗ Paper says "8.5x average" → Change to "8.5x"
- ✗ Paper says nothing specific → Remove specific number

### Example 2: Faddoul Non-determinism

**Your claim:** "identify non-determinism as the dominant source of complexity"

**Action:**
1. Find paper using search terms
2. Look for section on complexity/non-determinism
3. Find exact quote

**Possible outcomes:**
- ✓ Find quote: "Non-determinism is the dominant source" → Add page number
- ✗ Find different wording → Change to match exact quote
- ✗ Can't find claim → Remove or find better citation

---

## Red Flags in Your Paper

These are signs of potentially problematic citations:

1. **Vague claims:** "Smith et al. proposed a method" (no specifics)
2. **Overgeneralization:** "Many researchers have shown..." (no specific papers)
3. **Specific numbers without pages:** "10x speedup" (where in the paper?)
4. **Interpretation as fact:** Your analysis presented as paper's claim
5. **Wrong paper cited:** ELK cited to wrong paper
6. **Wrong algorithm cited:** Chase-Lev cited to wrong paper

---

## Files Location

All guide files are in:
```
/home/admindigit/tableauxx/paper/
├── PAPER_REVISION_GUIDE.md      # Main guide with search terms
├── MARKED_SECTIONS.txt          # Line-by-line citation analysis
├── ACADEMIC_INTEGRITY_WARNING.md # Why this matters
├── REVISION_SUMMARY.md          # This file
└── reference_pdfs/              # Downloaded PDFs (3 so far)
    ├── 01_owl2bench_singh2020.pdf
    ├── 02_quan2019_framework.pdf
    └── 03_song2013_complete.pdf
```

Your manuscript is at:
```
/home/admindigit/tableauxx/paper/submission/manuscript.tex
```

---

## Timeline

| Phase | Time | Task |
|-------|------|------|
| 1 | 1-2 days | Fix 3 critical errors |
| 2 | 2-3 days | Verify 5 specific numbers |
| 3 | 1 week | Read 16 priority papers |
| 4 | 1 week | Read 20 supporting papers |
| 5 | 2-3 days | Fix manuscript based on readings |
| **Total** | **3-4 weeks** | **Complete revision** |

---

## DO NOT SUBMIT Until:

- [ ] All 36 papers have been read
- [ ] Chase-Lev citation fixed
- [ ] ELK citation fixed
- [ ] Duplicate Pellet citation resolved
- [ ] All 5 specific numbers verified
- [ ] All direct quotes have page numbers
- [ ] Manuscript updated with corrections

---

## Need Help?

If you can't find a paper:
1. Try ResearchGate: https://researchgate.net
2. Email the author directly
3. Request through inter-library loan
4. Ask me for alternative search terms

---

## Final Note

**Better to delay submission than to submit with unverified citations.**

Academic misconduct allegations can damage your career permanently. Take the time to do this right.

Good luck!
