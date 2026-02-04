# SPACL Paper Package

**Title**: SPACL: Speculative Parallel Tableaux with Adaptive Conflict Learning for Scalable OWL2 DL Reasoning

**Paper Type**: Research Paper (8 pages + references)

**Target Venues**: 
- ISWC 2026 (International Semantic Web Conference)
- ESWC 2026 (Extended Semantic Web Conference)
- Journal of Web Semantics

---

## Package Contents

```
paper/
├── README.md              # This file
├── main.md                # Paper in Markdown format
├── main.tex               # Paper in LaTeX format
│
├── sections/              # Individual sections (optional)
│   ├── 1-introduction.tex
│   ├── 2-related-work.tex
│   ├── 3-algorithm.tex
│   ├── 4-implementation.tex
│   ├── 5-evaluation.tex
│   └── 6-conclusion.tex
│
├── figures/               # Generated figures
│   ├── generate_graphs.py # Figure generation script
│   ├── scalability.png    # Scalability comparison
│   ├── scalability.pdf    # Vector version
│   ├── throughput.png     # Throughput comparison
│   ├── throughput.pdf     # Vector version
│   ├── speedup.png        # Speedup with crossover
│   └── speedup.pdf        # Vector version
│
├── tables/                # LaTeX tables
│   ├── scalability.tex    # Main results table
│   └── comparison.tex     # SOTA comparison
│
└── references/            # Bibliography files
    └── bibliography.bib   # BibTeX file
```

---

## Building the Paper

### Option 1: LaTeX (Recommended for Submission)

```bash
cd paper
pdflatex main.tex
bibtex main
pdflatex main.tex
pdflatex main.tex
```

Requirements:
- TeX Live or MiKTeX
- `algorithm`, `algpseudocode` packages

### Option 2: Markdown (Draft/Review)

The `main.md` file can be viewed directly or converted:

```bash
# Convert to PDF with pandoc
pandoc main.md -o paper.pdf --template=latex

# Convert to HTML
pandoc main.md -o paper.html
```

---

## Key Results Summary

### Performance Highlights

| Metric | Value |
|--------|-------|
| **Max Speedup** | 4.88× at 10,000 classes |
| **Peak Throughput** | 26.2M operations/second |
| **Crossover Point** | ~1,000 classes |
| **Small Ontology Overhead** | <2× (acceptable) |
| **Tests Passing** | 71/71 |

### Comparison with State-of-the-Art

| Reasoner | 1K-class Time | vs Tableauxx |
|----------|---------------|--------------|
| Tableauxx SPACL | 158 µs | 1.0× (baseline) |
| Pellet | ~10 ms | ~63× slower |
| HermiT | ~50 ms | ~316× slower |

---

## Research Contributions

1. **SPACL Algorithm**: First OWL2 DL reasoner with speculative parallelism + nogood learning
2. **Adaptive Threshold**: Automatic sequential/parallel selection (~1000 class crossover)
3. **Thread-Local Caching**: 80% synchronization overhead reduction
4. **Production Implementation**: Rust, open-source, <2× overhead for all sizes

---

## Reproducibility

### Source Code
- Repository: [Add your repo URL]
- Branch: `paper-submission`
- Tag: `v1.0-spacl`

### Running Benchmarks

```bash
# Clone repository
git clone [repo-url]
cd tableauxx

# Run tests
cargo test --lib

# Run scalability benchmark
cargo bench --bench scalability

# Run extreme scale test (10K-100K)
cargo bench --bench extreme_scale
```

### Test Ontologies
Generated test ontologies included in `tests/data/`:
- hierarchy_100.owl (100 classes)
- hierarchy_1000.owl (1,000 classes)
- hierarchy_10000.owl (10,000 classes)
- hierarchy_100000.owl (100,000 classes)

---

## Figures

All figures are generated from actual benchmark data using the included Python script.

To regenerate:
```bash
cd paper/figures
python3 generate_graphs.py
```

Figure descriptions:
- **scalability.{png,pdf}**: Time comparison (log-log scale)
- **throughput.{png,pdf}**: Throughput bar chart (M ops/sec)
- **speedup.{png,pdf}**: Speedup with crossover annotation

---

## Tables

LaTeX tables are in `tables/` directory:
- **scalability.tex**: Main benchmark results
- **comparison.tex**: State-of-the-art comparison

Tables use `booktabs` for professional formatting.

---

## Paper Statistics

- **Pages**: 8 (excluding references)
- **Figures**: 3
- **Tables**: 2
- **Algorithms**: 1
- **References**: 13

---

## Author Information

**Authors**: Anusorn Chaikaew  
**Affiliation**: PhD Candidate in Computer Science, Mahidol University  
**Email**: anusorn.chaikaew@student.mahidol.ac.th  
**GitHub**: [Your GitHub]

**Corresponding Author**: Anusorn Chaikaew  
**Address**: [Your Address]

---

## License

Paper: [License for paper]  
Code: [License for code - e.g., MIT, Apache-2.0]

---

## Acknowledgments

[To be added]

---

## Changelog

**v1.0** (February 2, 2026)
- Initial paper submission
- All benchmarks complete
- All research contributions validated

---

*Last Updated: February 2, 2026*
