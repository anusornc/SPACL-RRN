# Journal of Web Semantics - Final Submission Package

**Paper Title**: SPACL: Speculative Parallel Tableaux with Adaptive Conflict Learning for Scalable OWL2 DL Reasoning

**Authors**: 
1. Anusorn Chaikaew (First Author) - anusorn_chaikaew@cmu.ac.th
2. Varin Chouvatut (Second Author) - varin.ch@cmu.ac.th
3. Ekkarat Boonchieng (Corresponding Author) - ekkarat.boonchieng@cmu.ac.th

**Affiliation**: Department of Computer Science, Faculty of Science, Chiang Mai University, Chiang Mai 50200, Thailand

**Submission Date**: February 2026

---

## 📦 Submission Package Contents

This package follows the **official Elsevier LaTeX submission guidelines** for Journal of Web Semantics.

### Required Files (All at Same Directory Level)

| File | Type | Description |
|------|------|-------------|
| `main.tex` | LaTeX Source | Main manuscript (uses elsarticle class) |
| `references.bib` | BibTeX | Bibliography (54 references) |
| `scalability.pdf` | PDF Vector | Figure 1 - Scalability comparison |
| `throughput.pdf` | PDF Vector | Figure 2 - Throughput comparison |
| `speedup.pdf` | PDF Vector | Figure 3 - Speedup with crossover |
| `scalability.png` | PNG Raster | Figure 1 (alternative format) |
| `throughput.png` | PNG Raster | Figure 2 (alternative format) |
| `speedup.png` | PNG Raster | Figure 3 (alternative format) |

---

## ✅ Elsevier Template Compliance

### Document Class
- **Class**: `elsarticle` (official Elsevier class)
- **Options**: `preprint,12pt`
- **Journal**: `Journal of Web Semantics`

### Reference Style
- **Style**: `elsarticle-num` (numbered citations)
- **Format**: BibTeX

### Formatting Checklist
- [x] Title and author information in frontmatter
- [x] Abstract (structured)
- [x] Keywords (6 keywords with \sep separator)
- [x] Highlights (5 bullet points)
- [x] Numbered sections
- [x] Numbered references
- [x] All figures in PDF format (vector graphics)
- [x] All files at same directory level (no subdirectories)

---

## 📋 Compilation Instructions

### Local Compilation

```bash
# Navigate to submission directory
cd jws_submission_final

# Compile with LaTeX
pdflatex main.tex
bibtex main
pdflatex main.tex
pdflatex main.tex

# Output: main.pdf
```

### Expected Output
- **PDF Size**: ~200-300 KB
- **Pages**: ~12-15 pages
- **Format**: Preprint format with line numbers

---

## 📤 Submission Instructions

### Step 1: Prepare Files
Ensure all files are in a single directory (no subfolders):
```
jws_submission_final/
├── main.tex
├── references.bib
├── scalability.pdf
├── throughput.pdf
├── speedup.pdf
└── ... (PNG files optional)
```

### Step 2: Create ZIP Archive
```bash
cd jws_submission_final
zip -r spacl_submission.zip *.tex *.bib *.pdf
```

### Step 3: Submit to Editorial Manager

**URL**: https://www.editorialmanager.com/jws

**Article Type**: Research Article

**Upload Files**:
1. **Manuscript**: `main.tex` (select "Manuscript" item type)
2. **Bibliography**: `references.bib` (select "Manuscript" item type)
3. **Figures**: `scalability.pdf`, `throughput.pdf`, `speedup.pdf` (select "Figure" item type)
4. **Compiled PDF**: `main.pdf` (select "Manuscript" item type)

**Important**: Do NOT upload files in subdirectories. Editorial Manager cannot process subdirectories.

---

## 📝 Metadata for Submission Form

### Title
SPACL: Speculative Parallel Tableaux with Adaptive Conflict Learning for Scalable OWL2 DL Reasoning

### Abstract
We present SPACL (Speculative Parallel Tableaux with Adaptive Conflict Learning), a novel OWL2 DL reasoner that achieves significant performance improvements through speculative parallelism combined with conflict-driven learning. SPACL is the first DL reasoner to combine work-stealing parallelism with nogood learning, addressing the challenge of exponential search spaces in tableau-based reasoning. Our key contributions include: (1) a speculative parallel tableaux algorithm; (2) an adaptive threshold mechanism; (3) thread-local nogood caching; and (4) a production-quality Rust implementation demonstrating 5× speedup at 10,000 classes. SPACL achieves 26.2 million operations per second, filling a critical gap in the semantic web tooling landscape.

### Keywords
OWL2 DL, Tableaux Reasoning, Parallel Algorithms, Nogood Learning, Description Logics, Semantic Web

### Highlights
- First OWL2 DL reasoner combining speculative parallelism with nogood learning
- Adaptive threshold mechanism for automatic sequential/parallel selection
- Thread-local caching reduces synchronization overhead by 80%
- 5× speedup at 10,000 classes with <2× overhead for small ontologies
- Production-quality Rust implementation with comprehensive benchmarks

---

## 🔍 Pre-Submission Checklist

### Content
- [ ] Author affiliation complete (address, city, postcode, phone)
- [ ] Acknowledgments section filled
- [ ] GitHub repository URL added
- [ ] All "<you write>" placeholders replaced
- [ ] ORCID added (optional but recommended)

### Technical
- [ ] PDF compiles without errors
- [ ] All citations resolve correctly
- [ ] All figures display properly
- [ ] Page count appropriate (12-15 pages)

### Files
- [ ] All files at same directory level
- [ ] No subdirectories
- [ ] ZIP archive created
- [ ] File naming correct (no spaces, simple names)

---

## 📧 Contact Information

**Corresponding Author**: Ekkarat Boonchieng  
**Email**: ekkarat.boonchieng@cmu.ac.th  
**Institution**: Chiang Mai University, Thailand

---

## 📄 LaTeX Template Information

**Template**: Elsevier Article (elsarticle)  
**Version**: Standard preprint format  
**Class File**: `elsarticle.cls` (included in TeX Live/MiKTeX)  
**Reference Style**: `elsarticle-num.bst` (numbered citations)

For template documentation:  
- Run: `texdoc elsarticle` (if installed)
- Or visit: https://www.elsevier.com/authors/policies-and-guidelines/latex-instructions

---

**Last Updated**: February 3, 2026  
**Package Version**: 1.0 (Submission Ready)
