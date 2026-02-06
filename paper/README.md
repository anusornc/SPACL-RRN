# SPACL Paper

**Title**: SPACL: Speculative Parallelism and Conflict Learning for Scalable OWL Ontology Reasoning

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

## License

This paper is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](LICENSE).

You are free to:
- **Share** — copy and redistribute the material in any medium or format
- **Adapt** — remix, transform, and build upon the material for any purpose, even commercially

Under the terms that you give appropriate credit to the authors.

## Repository

https://github.com/anusornchaikaew/tableauxx

**Note**: The repository contains:
- **Code** (Rust implementation): MIT License
- **Paper** (LaTeX manuscript): CC BY 4.0 License
