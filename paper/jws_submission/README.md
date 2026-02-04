# Journal of Web Semantics - Submission Package

**Paper Title**: SPACL: Speculative Parallel Tableaux with Adaptive Conflict Learning for Scalable OWL2 DL Reasoning

**Author**: <Author Name>  
**Affiliation**: <Institution>  
**Email**: <author.email@institution.edu>

**Submission Type**: Research Article

---

## 📦 Package Contents

```
jws_submission/
├── README.md                    # This file
├── main.tex                     # Main LaTeX manuscript
├── references.bib               # BibTeX bibliography (17 references)
│
├── scalability.pdf              # Figure 1 (PDF vector)
├── scalability.png              # Figure 1 (PNG raster)
├── throughput.pdf               # Figure 2 (PDF vector)
├── throughput.png               # Figure 2 (PNG raster)
├── speedup.pdf                  # Figure 3 (PDF vector)
├── speedup.png                  # Figure 3 (PNG raster)
│
└── [TO DO: architecture.pdf]    # Figure 4 - Architecture diagram
```

---

## 📋 Submission Checklist

### Manuscript Requirements

- [x] Abstract (200 words)
- [x] Keywords (6 keywords)
- [x] Highlights (5 bullet points)
- [x] Introduction with research questions
- [x] Related work section
- [x] Methodology/Algorithm section
- [x] Implementation section
- [x] Comprehensive evaluation
- [x] Conclusion with future work
- [x] Acknowledgments section (marked for completion)
- [x] References (17 citations)
- [ ] Appendix (optional, included)

### Figures and Tables

- [x] Figure 1: Scalability comparison (PDF + PNG)
- [x] Figure 2: Throughput comparison (PDF + PNG)
- [x] Figure 3: Speedup with crossover (PDF + PNG)
- [ ] Figure 4: Architecture diagram [TO DO]
- [ ] Figure 5: Threshold tuning graph [TO DO]
- [ ] Figure 6: Nogood effectiveness [TO DO]
- [x] Table 1: Scalability results
- [x] Table 2: State-of-the-art comparison

### Author Information

- [x] Author name: Anusorn Chaikaew
- [x] Affiliation: Mahidol University
- [x] Email: anusorn.chaikaew@student.mahidol.ac.th
- [ ] ORCID [TO DO]
- [ ] Address [TO DO]
- [ ] Phone [TO DO]

### Additional Materials

- [ ] Cover letter [TO DO]
- [ ] Response to reviewers (for revision)
- [ ] Supplementary materials (if applicable)

---

## 📝 Items Marked for Author Completion

The following items in `main.tex` are marked with `[TO DO]` and need your attention:

### Critical (Must Complete)

1. **Line 17-23**: Author affiliation address
   ```latex
   \affiliation[1]{organization={Mahidol University},
                   addressline={[Your Department Address]}, 
                   city={[City]},
                   postcode={[Postal Code]}, 
                   country={Thailand}}
   ```

2. **Line 27**: Corresponding author phone
   ```latex
   \cortext[cor1]{Corresponding author. Tel.: [Your Phone]...}
   ```

3. **Line 441**: GitHub repository URL
   ```latex
   SPACL is available at \url{[TO DO: Add GitHub repository URL]}...
   ```

4. **Line 444**: License information
   ```latex
   ...under the [TO DO: Add license] license.
   ```

### Figures to Create

5. **Line 139**: Architecture diagram
   ```latex
   \includegraphics[width=0.8\textwidth]{architecture.pdf}
   \textit{[TO DO: Create architecture diagram...]}
   ```

6. **Line 360**: Threshold tuning graph
   ```latex
   \textit{[TO DO: Add threshold tuning graph...]}
   ```

7. **Line 397**: Nogood effectiveness graph
   ```latex
   \textit{[TO DO: Add nogood effectiveness graph...]}
   ```

### Optional Sections

8. **Line 453**: Acknowledgments
   ```latex
   [TO DO: Add acknowledgments - advisors, funding sources, etc.]
   ```

9. **Appendix**: Additional details if needed

---

## 🏗️ How to Compile

### Requirements

- TeX Live 2022 or later (or MiKTeX)
- `elsarticle` document class (included in TeX Live)
- `algorithm` and `algpseudocode` packages
- `graphicx`, `booktabs`, `hyperref` packages

### Compilation Steps

```bash
cd jws_submission

# Compile with BibTeX
pdflatex main.tex
bibtex main
pdflatex main.tex
pdflatex main.tex

# Or use latexmk for automatic compilation
latexmk -pdf main.tex
```

### Output

- `main.pdf` - Final manuscript (should be ~10-12 pages)

---

## 📊 Paper Statistics

| Metric | Value |
|--------|-------|
| **Pages** | ~12 (including references) |
| **Figures** | 3 complete, 3 to create |
| **Tables** | 2 |
| **Algorithms** | 1 |
| **References** | 17 |
| **Equations** | 3 |

---

## 🎯 Journal of Web Semantics Specific Requirements

From the [Guide for Authors](https://www.sciencedirect.com/journal/journal-of-web-semantics/publish/guide-for-authors):

### Article Types

- **Research Article**: Full-length research papers (our submission)
- Typical length: 15-25 pages
- Our paper: ~12 pages (acceptable)

### Formatting

- [x] Use elsarticle document class
- [x] Preprint format for submission
- [x] Numbered reference style
- [x] Highlights included
- [x] Keywords included

### Submission Portal

Submit via: [Elsevier Editorial Manager](https://www.editorialmanager.com/jws)

Required files:
1. `main.tex` (manuscript)
2. `references.bib` (bibliography)
3. `*.pdf` figures
4. Cover letter (separate document)

---

## 🔬 Research Summary

### Novel Contributions

1. **First** OWL2 DL reasoner combining speculative parallelism with nogood learning
2. **Adaptive threshold** for automatic sequential/parallel selection
3. **Thread-local caching** reducing synchronization by 80%
4. **5× speedup** at 10K classes with <2× overhead for small ontologies

### Key Results

| Classes | Sequential | SPACL | Speedup |
|---------|------------|-------|---------|
| 100 | 13.3 µs | 20.9 µs | 0.64× |
| 1,000 | 159.7 µs | 158.4 µs | 1.01× |
| 10,000 | 1,865.3 µs | 382.3 µs | **4.88×** |

**Peak Throughput**: 26.2 million operations/second

---

## 📝 Abstract (for Editorial System)

```
We present SPACL (Speculative Parallel Tableaux with Adaptive Conflict Learning), 
a novel OWL2 DL reasoner that achieves significant performance improvements through 
speculative parallelism combined with conflict-driven learning. SPACL is the first 
DL reasoner to combine work-stealing parallelism with nogood learning, addressing 
the challenge of exponential search spaces in tableau-based reasoning.

Our key contributions include: (1) a speculative parallel tableaux algorithm; 
(2) an adaptive threshold mechanism; (3) thread-local nogood caching; and 
(4) a production-quality Rust implementation demonstrating 5× speedup at 10,000 
classes while maintaining <2× overhead for small ontologies.

SPACL achieves 26.2 million operations per second, outperforming sequential 
baselines and filling a critical gap in the semantic web tooling landscape.
```

**Keywords**: OWL2 DL, Tableaux Reasoning, Parallel Algorithms, Nogood Learning, Description Logics, Semantic Web

---

## ✅ Pre-Submission Checklist

Before submitting, ensure:

- [ ] All [TO DO] items completed
- [ ] PDF compiles without errors
- [ ] All figures visible in PDF
- [ ] All citations resolved
- [ ] Page count appropriate (10-15 pages)
- [ ] ORCID included
- [ ] Cover letter written
- [ ] GitHub repository public (if applicable)
- [ ] License file in repository

---

## 📧 Contact

For questions about this submission:

**Author**: Anusorn Chaikaew  
**Email**: anusorn.chaikaew@student.mahidol.ac.th  
**Institution**: Mahidol University

---

## 📄 License

[TO DO: Specify license for paper - typically retains copyright until publication]

Code repository: [TO DO: Add repository URL with license file]

---

**Last Updated**: February 2, 2026  
**Paper Version**: 1.0  
**Status**: Ready for submission after [TO DO] items completed
