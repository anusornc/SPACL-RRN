# SPACL Paper - Journal of Web Semantics Submission

**Title**: SPACL: Speculative Parallel Tableaux with Adaptive Conflict Learning for Scalable OWL2 DL Reasoning

## Submission Package

The official submission is in the `submission/` directory:

```
paper/submission/
├── manuscript.tex          # Main LaTeX manuscript
├── manuscript.pdf          # Compiled PDF
├── references.bib          # Bibliography
├── elsarticle.cls          # Elsevier class file
├── elsarticle-num.bst      # Bibliography style
├── scalability.pdf         # Figure 1
├── throughput.pdf          # Figure 2
├── speedup.pdf             # Figure 3
└── README.txt              # Submission instructions
```

## Authors

- **Anusorn Chaikaew** (First Author) - anusorn_chaikaew@cmu.ac.th
- **Varin Chouvatut** (Second Author) - varin.ch@cmu.ac.th  
- **Ekkarat Boonchieng** (Corresponding Author) - ekkarat.boonchieng@cmu.ac.th

**Affiliation**: Department of Computer Science, Faculty of Science, 
Chiang Mai University, Chiang Mai 50200, Thailand

## Key Contributions

1. **Speculative Parallelism with Work-Stealing**: Dynamic task distribution
2. **Conflict-Driven Nogood Learning**: Thread-local caching (80% overhead reduction)
3. **Adaptive Parallelism Threshold**: 18-73× speedup over naive parallelization
4. **Global Thread Pool Optimization**: 2.1× overhead reduction
5. **Comprehensive Parser Suite**: OWL2 Functional, Manchester, JSON-LD

## Performance Highlights

| Metric | Value |
|--------|-------|
| Max Speedup | 4.88× at 10,000 classes |
| Peak Throughput | 26.2M operations/second |
| Small Case Speedup | 18-73× vs always-parallel |
| Thread Pool Improvement | 2.1× (235μs → 112μs) |

## Building the Paper

```bash
cd paper/submission
pdflatex manuscript.tex
bibtex manuscript
pdflatex manuscript.tex
pdflatex manuscript.tex
```

## Repository

https://github.com/anusornchaikaew/tableauxx
