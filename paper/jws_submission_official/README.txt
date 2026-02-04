================================================================================
JOURNAL OF WEB SEMANTICS - SUBMISSION PACKAGE
================================================================================

Paper Title:
  SPACL: Speculative Parallel Tableaux with Adaptive Conflict Learning 
  for Scalable OWL2 DL Reasoning

Authors:
  1. Anusorn Chaikaew (First Author)
     Email: anusorn_chaikaew@cmu.ac.th
  2. Varin Chouvatut (Second Author)
     Email: varin.ch@cmu.ac.th
  3. Ekkarat Boonchieng (Corresponding Author)
     Email: ekkarat.boonchieng@cmu.ac.th

Affiliation:
  Department of Computer Science, Faculty of Science
  Chiang Mai University, Chiang Mai 50200, Thailand

Corresponding Author Contact:
  Tel.: +66-5394-3413

Submission Date: February 3, 2026
Target Journal: Journal of Web Semantics (Elsevier)

================================================================================
IMPORTANT: EVALUATION SCOPE
================================================================================

This paper evaluates SPACL on SYNTHETIC HIERARCHIES (100-10,000 classes).

Real-world ontology evaluation (NCI, GALEN, SNOMED CT) is planned but NOT 
yet completed. This limitation is explicitly stated in the paper:
- Section 5.3: "Planned Real-World Evaluation"
- Section 6.2: "Limitations" (item 1)
- Abstract: specifies "synthetic hierarchies"

================================================================================
PACKAGE CONTENTS
================================================================================

Source Files:
  - manuscript.tex        Main LaTeX manuscript (elsarticle template)
  - references.bib        BibTeX bibliography (55 references)
  - elsarticle.cls        Elsevier document class
  - elsarticle-num.bst    Bibliographic style (numbered citations)

Figures (PDF vector format):
  - scalability.pdf       Figure 1: Scalability comparison
  - throughput.pdf        Figure 2: Throughput comparison
  - speedup.pdf           Figure 3: Speedup with crossover analysis

Compiled Output:
  - manuscript.pdf        Final compiled PDF (23 pages)

================================================================================
TEMPLATE INFORMATION
================================================================================

LaTeX Class:        elsarticle.cls (Elsevier official)
Document Options:   [preprint,12pt]
Reference Style:    elsarticle-num.bst (numbered citations)
Citation Format:    [1], [2], [3], etc.

This submission uses the official Elsevier elsarticle template.

================================================================================
COMPILATION INSTRUCTIONS
================================================================================

To compile the manuscript locally:

  pdflatex manuscript.tex
  bibtex manuscript
  pdflatex manuscript.tex
  pdflatex manuscript.tex

Required packages (standard LaTeX distribution):
  - amsmath, amssymb, amsfonts
  - algorithm, algpseudocode
  - graphicx, booktabs, multirow
  - subcaption, xcolor, hyperref
  - tikz (for architecture diagram)

================================================================================
SUBMISSION INSTRUCTIONS FOR EDITORIAL MANAGER
================================================================================

1. Visit: https://www.editorialmanager.com/jws

2. Article Type: Research Article

3. Upload Files (all files at same directory level):
   
   Item Type: Manuscript
   - manuscript.tex
   - references.bib
   - elsarticle.cls
   - elsarticle-num.bst
   
   Item Type: Figure
   - scalability.pdf
   - throughput.pdf
   - speedup.pdf
   
   Item Type: Manuscript (PDF)
   - manuscript.pdf

4. Important Notes:
   - All files must be at the same directory level (no subdirectories)
   - Use PDF format for all figures (vector graphics)
   - Include compiled PDF for reference

================================================================================
PAPER HIGHLIGHTS
================================================================================

1. First OWL2 DL reasoner combining speculative parallelism with nogood learning
2. Adaptive threshold mechanism for automatic sequential/parallel selection
3. Thread-local caching reduces synchronization overhead by 80%
4. 5x speedup at 10,000 classes (SYNTHETIC HIERARCHIES) with <2x overhead
5. Production-quality Rust implementation; real-world evaluation planned

================================================================================
KEYWORDS
================================================================================

OWL2 DL, Tableaux Reasoning, Parallel Algorithms, Nogood Learning, 
Description Logics, Semantic Web

================================================================================
EVALUATION DATA
================================================================================

Current Benchmarks (SYNTHETIC):
  - Linear subclass hierarchies: 100, 500, 1,000, 5,000, 10,000 classes
  - Hardware: AMD Ryzen 9 5900X (12 cores), 64GB RAM
  - Statistics: Criterion.rs with 100 samples, 95% confidence intervals

Planned Benchmarks (NOT YET COMPLETED):
  - Real-world ontologies from ORE 2015: NCI, GALEN, GO, SNOMED CT
  - Direct comparison with Pellet/HermiT/Konclude on identical hardware

================================================================================
ADDITIONAL INFORMATION
================================================================================

Code Repository:
  https://github.com/anusornchaikaew/spacl-reasoner

License: MIT

All benchmarks and test data are included in the repository for 
reproducibility.

================================================================================
