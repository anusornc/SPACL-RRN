# Paper Revision Guide - Fill in the Blanks

## Summary of Issues Found

Your paper contains 36 citations across multiple sections. Here's what needs your attention:

| Section | Citations | Status | Action Needed |
|---------|-----------|--------|---------------|
| Introduction | 7 | ⚠️ Needs Verification | Read papers, verify claims |
| Related Work | 24 | ⚠️ Needs Verification | Read papers, verify claims |
| Methodology | 2 | ⚠️ Needs Verification | Verify algorithm descriptions |
| Experiments | 8 | ⚠️ Needs Verification | Verify benchmark data |

---

## PRIORITY 1: Core References (MUST READ FIRST)

### 1. OWL 2 W3C Standard (owl2)
- **Search Terms:** "OWL 2 W3C recommendation Motik 2009"
- **Database:** W3C.org (FREE)
- **URL:** https://www.w3.org/TR/owl2-overview/
- **What to Verify:** Is OWL2 DL correctly described as the standard?

### 2. Description Logic Handbook (dlhandbook)
- **Search Terms:** "Description Logic Handbook Baader 2003"
- **Database:** Cambridge University Press
- **What to Verify:** Chapter on tableaux algorithms - page numbers for your claims

### 3. Pellet Reasoner (pellet)
- **Search Terms:** "Pellet OWL DL reasoner Sirin 2007"
- **Database:** ScienceDirect - Journal of Web Semantics
- **DOI:** 10.1016/j.websem.2007.03.004
- **What to Verify:** Does it explicitly say it is "sequential"?

### 4. HermiT Reasoner (hermit)
- **Search Terms:** "HermiT OWL 2 reasoner Glimm 2014"
- **Database:** Springer - Journal of Automated Reasoning
- **DOI:** 10.1007/s10817-014-9305-1
- **What to Verify:** Does it claim to "beat FaCT++ and Pellet on many difficult benchmarks"?

### 5. Konclude Reasoner (konclude)
- **Search Terms:** "Konclude Steigmiller 2014"
- **Database:** ScienceDirect - Journal of Web Semantics
- **DOI:** 10.1016/j.websem.2014.06.002
- **What to Verify:** Does it discuss parallel reasoning? Is it commercial/closed-source?

### 6. Hypertableau Algorithm (motik2009hypertableau)
- **Search Terms:** "Hypertableau Reasoning Description Logics Motik"
- **Database:** JAIR (FREE) - Journal of AI Research
- **URL:** https://jair.org/index.php/jair/article/view/10672
- **What to Verify:** Algorithm description, comparison to standard tableaux

### 7. GRASP SAT Solver (marques1999grasp)
- **Search Terms:** "GRASP Search Algorithm Propositional Satisfiability Marques-Silva"
- **Database:** IEEE Xplore
- **DOI:** 10.1109/12.769433
- **What to Verify:** CDCL algorithm description

### 8. Quan 2017 Parallel (quan2017parallel)
- **Search Terms:** "Parallel Shared-Memory OWL Classification Quan Haarslev"
- **Database:** IEEE Xplore - ICPPW 2017
- **DOI:** 10.1109/ICPPW.2017.38
- **What to Verify:** Speedup claims, sequential algorithm discussion

### 9. Quan 2019 Framework (quan2019framework)
- **Search Terms:** "Framework Parallelizing OWL Classification Quan"
- **Database:** arXiv (FREE)
- **URL:** https://arxiv.org/abs/1906.07749
- **What to Verify:** Order-of-magnitude improvements claim

---

## PRIORITY 2: Your Key Claims (CRITICAL TO VERIFY)

### 10. Fork/Join Non-determinism (faddoul2015fork)
- **Search Terms:** "Handling Non-determinism Description Logics Fork Join Faddoul"
- **Database:** International Journal of Networking and Computing
- **What to Verify:** Does it say non-determinism is "dominant source of complexity"? PAGE NUMBER?

### 11. Redundant Exploration (steigmiller2020parallelised)
- **Search Terms:** "Parallelised ABox Reasoning Steigmiller Glimm"
- **Database:** CEUR-WS (FREE) - DL Workshop 2020
- **URL:** http://ceur-ws.org/Vol-2663/
- **What to Verify:** Repeated exploration of contradictions - exact quote?

### 12. NACRE / Nogood Gap (glorian2020nacre)
- **Search Terms:** "NACRE Nogood Clause Reasoning Engine Glorian"
- **Database:** CEUR-WS (FREE) - POS 2020
- **What to Verify:** Does it discuss DL reasoning? What gap does it identify?
- **⚠️ CRITICAL:** Your main research gap claim depends on this

### 13. Cichlid Speedup (gu2015cichlid)
- **Search Terms:** "Cichlid Large Scale RDFS OWL Reasoning Spark Gu"
- **Database:** IEEE Xplore - IPDPS 2015
- **DOI:** 10.1109/IPDPS.2015.46
- **What to Verify:** Exact speedup - you wrote "approximately 10x" - is this accurate?

### 14. ComR Performance (wang2019comr)
- **Search Terms:** "ComR Combined OWL EL Full Reasoner Wang"
- **Database:** IOS Press - Semantic Web Journal
- **What to Verify:** Exact percentage - you wrote "96.9% reduction" - verify!

### 15. Algahtani Speedup (algahtani2024mp)
- **Search Terms:** "MP-HTHEDL Parallel Hypothesis Evaluation Description Logic"
- **Database:** University repository (PhD thesis)
- **What to Verify:** Exact speedup - you wrote "161x" - verify!

### 16. ORE Benchmarks (parsia2015ore)
- **Search Terms:** "OWL Reasoner Evaluation ORE 2015 Competition Parsia"
- **Database:** CEUR-WS (FREE)
- **URL:** http://ceur-ws.org/Vol-1387/
- **What to Verify:** Timeout/failure rates, reasoner rankings

---

## PRIORITY 3: Supporting References (Verify as Needed)

### 17. BioPortal (bioportal)
- **Search Terms:** "BioPortal biomedical ontologies Whetzel"
- **Database:** BioMed Central (FREE)
- **What to Verify:** Dataset descriptions

### 18. LUBM (lubm)
- **Search Terms:** "Lehigh University Bench LUBM Guo"
- **Database:** ScienceDirect - Web Semantics
- **What to Verify:** Benchmark specifications

### 19. Work Stealing (workstealing)
- **Search Terms:** "Thread scheduling multiprogramming multiprocessors work stealing Arora"
- **Database:** ACM Digital Library - SPAA 1998
- **What to Verify:** Algorithm description
- **⚠️ ISSUE:** You cite this for "Chase-Lev algorithm" but Chase-Lev is a DIFFERENT paper from 2005

### 20. ELK Reasoner (elk)
- **Search Terms:** "ELK polynomial procedures efficient reasoning Kazakov"
- **Database:** Springer - ISWC 2012
- **What to Verify:** OWL EL profile limitations

### 21. Bate 2018 (bate2018consequence)
- **Search Terms:** "Consequence-Based Reasoning Description Logics Disjunctions Bate"
- **Database:** JAIR (FREE)
- **What to Verify:** Is ELK discussed? Or different paper?

---

## SPECIFIC CLAIMS TO VERIFY

### Line 183: Non-determinism as "dominant source of complexity"
```
Faddoul and MacCaull identify non-determinism as the dominant source 
of complexity in expressive description logics
```
**Action:** Find exact quote in Faddoul 2015. Get page number.

### Line 203: Cichlid "approximately 10x speedups"
```
Gu et al. developed Cichlid, reporting approximately 10x average speedups
```
**Action:** Find exact number in Gu 2015. Is it "approximately 10x" or different?

### Line 238: HermiT "beats FaCT++ and Pellet on many difficult benchmarks"
```
HermiT... reported to beat FaCT++ and Pellet on many difficult benchmarks
```
**Action:** Find this claim in Glimm 2014. Exact wording? Page number?

### Line 240: Konclude "significant performance improvements"
```
Konclude... shows significant performance improvements over pure tableau-based engines
```
**Action:** Verify in Song 2013 and Glimm 2014.

### Line 964: ORE "Konclude as 5-50x faster"
```
Published ORE 2015 benchmarks report Konclude as 5-50x faster than Pellet/HermiT
```
**Action:** Find EXACT numbers in Parsia 2015. Is it "5-50x" or different range?

---

## DATABASE ACCESS GUIDE

### Free/Open Access (No Login)
- **CEUR-WS:** http://ceur-ws.org/ - Workshop papers
- **JAIR:** https://jair.org/ - AI research journal
- **arXiv:** https://arxiv.org/ - Preprints
- **Zenodo:** https://zenodo.org/ - Research data
- **BioMed Central:** https://biomedcentral.com/ - Biomedical
- **W3C:** https://www.w3.org/TR/ - Web standards

### Requires University VPN
- **IEEE Xplore:** https://ieeexplore.ieee.org/
- **ACM Digital Library:** https://dl.acm.org/
- **ScienceDirect (Elsevier):** https://sciencedirect.com/
- **SpringerLink:** https://link.springer.com/
- **Wiley:** https://onlinelibrary.wiley.com/

### Alternative Sources
- **ResearchGate:** https://researchgate.net/ - Authors post preprints
- **Google Scholar:** https://scholar.google.com/ - Look for [PDF] links
- **Author websites:** Many post PDFs on personal pages

---

## RECOMMENDED READING ORDER

### Week 1: Core References (Priority 1)
Day 1-2: owl2, dlhandbook, pellet, hermit
Day 3-4: konclude, motik2009hypertableau, marques1999grasp
Day 5: quan2017parallel, quan2019framework

### Week 2: Your Claims (Priority 2)
Day 6-7: faddoul2015fork, steigmiller2020parallelised
Day 8-9: glorian2020nacre (CRITICAL), gu2015cichlid
Day 10: wang2019comr, algahtani2024mp
Day 11: parsia2015ore

### Week 3: Supporting (Priority 3)
Day 12-15: Remaining 20 papers

---

## VERIFICATION CHECKLIST

For EACH citation, verify:
- [ ] Did I read the full paper?
- [ ] Does the cited claim match the paper's actual findings?
- [ ] Is the page/section reference accurate?
- [ ] Are specific numbers (10x, 96.9%, 161x, 5-50x) EXACTLY correct?
- [ ] Is my interpretation fair to the authors?

---

## COMMON ISSUES TO FIX

### Issue 1: Chase-Lev Algorithm
You cite \cite{workstealing} for Chase-Lev algorithm, but Arora 1998 is DIFFERENT from Chase-Lev 2005.
**Fix:** Add proper citation for Chase-Lev 2005.

### Issue 2: Duplicate Pellet Citations
You have both \cite{pellet} and \cite{sirin2007pellet} - are these the same paper?
**Fix:** Verify and remove duplicate if needed.

### Issue 3: ELK Citation
You cite bate2018consequence for ELK, but this paper is about consequence-based reasoning, not ELK specifically.
**Fix:** Verify if ELK is discussed in this paper or find correct citation.

---

## DO NOT SUBMIT UNTIL:

1. [ ] All 36 papers have been read
2. [ ] All specific numbers (10x, 96.9%, 161x, 5-50x) are verified
3. [ ] All claims match what papers actually say
4. [ ] Page numbers added for direct quotes
5. [ ] Duplicate citations removed
6. [ ] Missing citations added (e.g., Chase-Lev 2005)

**Estimated time: 2-3 weeks of focused work**
