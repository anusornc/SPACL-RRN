# Journal of Web Semantics Submission - TO DO List

**Paper**: SPACL: Speculative Parallel Tableaux with Adaptive Conflict Learning for Scalable OWL2 DL Reasoning  
**Author**: Anusorn Chaikaew

---

## 🔴 CRITICAL - Must Complete Before Submission

### 1. Author Information
- [ ] Add your department address (main.tex line 20)
- [ ] Add your city (main.tex line 21)
- [ ] Add your postal code (main.tex line 22)
- [ ] Add your phone number (main.tex line 27)
- [ ] Add your ORCID (optional but recommended)
- [ ] Add your website/portfolio URL (optional)

### 2. Repository and License
- [ ] Create GitHub repository for SPACL code
- [ ] Add LICENSE file to repository (MIT/Apache-2.0 recommended)
- [ ] Update repository URL in main.tex (line 441)
- [ ] Update license name in main.tex (line 444)

### 3. Acknowledgments
- [ ] Add acknowledgments section (main.tex line 453)
  - [ ] Advisor name
  - [ ] Funding sources (if any)
  - [ ] Helpful colleagues
  - [ ] Institution support

---

## 🟡 IMPORTANT - Strongly Recommended

### 4. Architecture Diagram (Figure 4)
- [ ] Create system architecture diagram showing:
  - [ ] Main reasoner component
  - [ ] Worker threads
  - [ ] Work queues
  - [ ] Nogood database
  - [ ] Adaptive controller
- [ ] Save as: `architecture.pdf` and `architecture.png`
- [ ] Add to jws_submission/ folder

### 5. Threshold Tuning Graph (Figure 5)
- [ ] Run benchmarks with different threshold values (50, 100, 200, 500)
- [ ] Create graph showing performance vs threshold
- [ ] Save as: `threshold.pdf` and `threshold.png`
- [ ] Update main.tex line 360

### 6. Nogood Effectiveness Graph (Figure 6)
- [ ] Run nogood effectiveness benchmark
- [ ] Collect hit rate data over time
- [ ] Create graph showing:
  - [ ] Nogood hit rate
  - [ ] Cache hit ratio
  - [ ] Branches pruned
- [ ] Save as: `nogood_effectiveness.pdf` and `nogood_effectiveness.png`
- [ ] Update main.tex line 397

---

## 🟢 NICE TO HAVE - Optional Enhancements

### 7. Additional Benchmarks (Appendix)
- [ ] Run 100K class benchmark
- [ ] Memory usage profiling
- [ ] Comparison with more reasoners (if accessible)

### 8. Code Documentation
- [ ] Add README to code repository
- [ ] Add installation instructions
- [ ] Add API documentation
- [ ] Add usage examples

### 9. Supplementary Materials
- [ ] Create supplementary materials document
- [ ] Include additional proofs (if any)
- [ ] Include extended benchmarks

---

## 📝 COVER LETTER (Required)

Create a cover letter with:

```
Dear Editor,

We submit our manuscript entitled "SPACL: Speculative Parallel Tableaux 
with Adaptive Conflict Learning for Scalable OWL2 DL Reasoning" for 
consideration for publication in the Journal of Web Semantics.

[Explain:
- Why this paper fits JWS
- Main contributions
- Why it's novel
- Practical impact
]

We confirm that this work is original and has not been published elsewhere.

Sincerely,
Anusorn Chaikaew
PhD Candidate, Mahidol University
```

---

## ✅ FINAL CHECKS

Before submission:

- [ ] Compile main.tex successfully
- [ ] Check all figures appear correctly
- [ ] Check all citations resolve
- [ ] Check page count (target: 12-15 pages)
- [ ] Check spelling and grammar
- [ ] Check author information complete
- [ ] Check highlights are accurate
- [ ] Check abstract is compelling

---

## 📤 SUBMISSION PROCESS

1. **Prepare files**:
   ```bash
   cd jws_submission
   pdflatex main.tex
   bibtex main
   pdflatex main.tex
   pdflatex main.tex
   ```

2. **Go to**: https://www.editorialmanager.com/jws

3. **Create account** (if first time)

4. **Start new submission**:
   - Article Type: Research Article
   - Section/Category: [Select appropriate]
   - Title: SPACL: Speculative Parallel Tableaux with Adaptive Conflict Learning for Scalable OWL2 DL Reasoning

5. **Upload files**:
   - Manuscript: main.tex
   - Bibliography: references.bib
   - Figures: *.pdf files
   - Cover Letter: cover_letter.pdf

6. **Enter metadata**:
   - Abstract (copy from main.tex)
   - Keywords: OWL2 DL, Tableaux Reasoning, Parallel Algorithms, Nogood Learning, Description Logics, Semantic Web
   - Authors: Anusorn Chaikaew
   - Affiliations: Mahidol University

7. **Suggest reviewers** (3-5 names):
   - [TO DO: Add names of experts in DL reasoning]
   - [TO DO: Add names of experts in parallel algorithms]
   - [TO DO: Add names of experts in Semantic Web]

8. **Submit**

---

## 📅 TIMELINE SUGGESTION

| Task | Time | Priority |
|------|------|----------|
| Complete author info | 15 min | 🔴 Critical |
| Set up GitHub repo | 30 min | 🔴 Critical |
| Create architecture diagram | 2 hours | 🟡 Important |
| Write acknowledgments | 30 min | 🔴 Critical |
| Create threshold graph | 2 hours | 🟡 Important |
| Create nogood graph | 2 hours | 🟡 Important |
| Write cover letter | 1 hour | 🟡 Important |
| Final checks | 1 hour | 🔴 Critical |
| Submit | 30 min | 🔴 Critical |
| **Total** | **~10 hours** | |

---

## 🆘 QUESTIONS TO CONSIDER

1. **Do you have a GitHub account?**
   - If no, create one at https://github.com

2. **What license for the code?**
   - MIT: Very permissive
   - Apache-2.0: Permissive with patent protection
   - GPL: Copyleft (less recommended for libraries)
   - Recommendation: MIT or Apache-2.0

3. **Who should be acknowledged?**
   - PhD advisor
   - Committee members
   - Funding agency (if applicable)
   - Collaborators

4. **Do you need co-authors?**
   - If advisor contributed significantly, consider adding
   - Check your institution's authorship guidelines

---

## 📊 COMPLETION STATUS

**Overall Progress**: ~80% complete

**What's Done**:
- ✅ Main manuscript written
- ✅ 3 figures created
- ✅ 2 tables created
- ✅ Bibliography complete
- ✅ LaTeX template formatted

**What's Left**:
- ⬜ Author details
- ⬜ Repository setup
- ⬜ 3 additional figures
- ⬜ Cover letter

---

**Last Updated**: February 2, 2026  
**Next Action**: Start with author information (15 minutes)
