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
  - export_docx.sh        DOCX export helper (Pandoc via Docker/local)

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
DOCX EXPORT INSTRUCTIONS
================================================================================

Preferred (uses Docker Pandoc image, no local install required):

  ./export_docx.sh

Optional custom output filename:

  ./export_docx.sh manuscript_for_review.docx

Direct command equivalent:

  docker run --rm --user "$(id -u):$(id -g)" \
    -v "$(pwd):/workdir" -w /workdir pandoc/core:latest \
    manuscript.tex --from=latex --to=docx --resource-path=. \
    -o manuscript.docx

Important note:
  Pandoc may keep some advanced LaTeX math macros as raw TeX in DOCX.
  After export, review equations and algorithm blocks in Word before sharing.

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

1. Open ALC/SHOIQ implementation combining speculative parallelism with nogood learning
2. Adaptive threshold mechanism achieving 18-73x speedup over naive always-parallel execution on small cases
3. Thread-local caching keeps most repeated nogood lookups on the local hot path (82-95% local cache-hit rates in dedicated ablations)
4. Global thread pool optimization reduces parallel overhead by 2.1x
5. 5x speedup at 10,000 classes on synthetic disjunctive workloads with <2x overhead on small cases
6. Implemented parser support used in this study: RDF/XML, OWL2 Functional, Manchester Syntax, JSON-LD
7. Rust implementation with explicit reporting of benchmark scope and constraints

================================================================================
KEYWORDS
================================================================================

OWL2 DL, Tableaux Reasoning, Parallel Algorithms, Nogood Learning, 
Description Logics, Semantic Web

================================================================================
CODE REPOSITORY
================================================================================

https://github.com/anusornc/SPACL

LICENSE
--------

- Paper (LaTeX manuscript): Creative Commons Attribution 4.0 International (CC BY 4.0)
  https://creativecommons.org/licenses/by/4.0/
  
- Code (Rust implementation): MIT License
  https://opensource.org/licenses/MIT

All benchmarks and test data are included in the repository for 
reproducibility.

================================================================================
