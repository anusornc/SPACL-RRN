================================================================================
SPACL: SPECULATIVE PARALLELISM AND CONFLICT LEARNING
                 FOR SCALABLE OWL ONTOLOGY REASONING
================================================================================

Paper Title:
  SPACL: Speculative Parallelism and Conflict Learning 
  for Scalable OWL Ontology Reasoning

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
  - manuscript.pdf        Final compiled PDF

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
2. Adaptive threshold mechanism achieving 18-73x speedup over naive parallelization
3. Thread-local caching reduces synchronization overhead by 80%
4. Global thread pool optimization reduces parallel overhead by 2.1x
5. 5x speedup at 10,000 classes (synthetic hierarchies) with <2x overhead
6. Comprehensive parser support: OWL2 Functional, Manchester Syntax, JSON-LD
7. Production-quality Rust implementation; real-world evaluation planned

================================================================================
KEYWORDS
================================================================================

OWL2 DL, Tableaux Reasoning, Parallel Algorithms, Nogood Learning, 
Description Logics, Semantic Web

================================================================================
CODE REPOSITORY
================================================================================

https://github.com/anusornchaikaew/tableauxx

LICENSE
--------

- Paper (LaTeX manuscript): Creative Commons Attribution 4.0 International (CC BY 4.0)
  https://creativecommons.org/licenses/by/4.0/
  
- Code (Rust implementation): MIT License
  https://opensource.org/licenses/MIT

All benchmarks and test data are included in the repository for 
reproducibility.

================================================================================
