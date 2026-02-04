# Kimi Handoff Prompt - Tableauxx Project

## 📋 Project Overview
**Tableauxx** is a high-performance OWL2 DL Reasoner implementing the novel **SPACL** (Speculative Parallel Tableaux with Adaptive Conflict Learning) algorithm in Rust.

## 🚀 Quick Start Commands

```bash
# Build the library
cargo build --lib

# Run all tests
cargo test --lib

# Run benchmarks
cargo bench

# Build binary
cargo build --bin owl2_validation
```

## 📁 Project Structure (CRITICAL - Read This First)

```
tableauxx/
├── src/                      # Main Rust source code
│   ├── core/                # IRI, entities, ontology structures
│   ├── logic/axioms/        # OWL axioms & class expressions
│   ├── parser/              # RDF/XML, Turtle, OWL/XML parsers
│   ├── reasoner/            # Reasoning engines
│   │   ├── tableaux/        # Core tableaux implementation
│   │   ├── speculative.rs   # SPACL algorithm (NOVEL)
│   │   ├── simple.rs        # Sequential baseline
│   │   └── ...
│   ├── strategy/            # EL/QL/RL profile optimizations, meta-reasoner
│   └── util/                # Caches, memory protection, config
├── benches/                 # Criterion benchmarks
├── tests/data/              # Test ontologies (hierarchy_*.owl, univ-bench.owl)
├── benchmarks/              # Real-world ontology benchmarks (download script)
├── paper/                   # JWS paper submission (LaTeX, PDFs)
├── docs/                    # Documentation (AGENTS.md, SPACL_ALGORITHM.md, etc.)
└── results/                 # Benchmark results, reports
```

## ⚠️ Current Constraints (IMPORTANT)

### Disk Space Issues
- **Current free space:** ~12 GB
- **GitHub file limit:** 100 MB
- **Problem file:** `go-basic.owl` (112 MB) - EXCLUDED from repo, needs manual download

### Benchmarks That CANNOT Run (due to space/size limits)
1. Gene Ontology (GO) - 112 MB, exceeds git limit
2. NCI Thesaurus - 200+ MB
3. SNOMED CT - 400+ MB
4. Large ORE 2015 ontologies

### Benchmarks That CAN Run
```bash
# Small-scale benchmarks (safe to run)
cargo bench --bench scalability           # 100-10K classes (in-memory)
cargo bench --bench spacl_vs_sequential   # Sequential vs SPACL
cargo bench --bench quick_benchmark       # Fast validation
cargo bench --bench extreme_scale         # 10K-100K (uses hierarchy files)
```

## 📊 Recent Work Completed

### 1. Major Reorganization (DONE)
- Archived legacy code to `archive/` and `backup_tableauxx/`
- Reorganized entire project with proper Rust structure
- Created `src/` with modular architecture
- Set up `benches/` with Criterion benchmarks

### 2. Paper Writing (IN PROGRESS)
- Location: `paper/` directory
- Target: Journal of Web Semantics (JWS)
- Template: Elsevier elsarticle
- Key files:
  - `paper/main.tex` - Main LaTeX manuscript
  - `paper/jws_submission_final/` - Final submission files
  - `paper/figures/` - Graphs (scalability.pdf, speedup.pdf, throughput.pdf)

### 3. Benchmark Infrastructure (READY)
- Criterion.rs benchmarks in `benches/`
- Download script: `benchmarks/download_ontologies.sh`
- Test ontologies in `tests/data/`

## 🔧 Next Steps (Priority Order)

### HIGH Priority
1. **Verify build works on new machine:**
   ```bash
   cargo build --lib
   cargo test --lib
   ```

2. **Run quick benchmark to verify:**
   ```bash
   cargo bench --bench quick_benchmark
   ```

3. **Complete paper if needed:**
   - Check `paper/jws_submission_final/main.tex`
   - Verify all figures compile
   - Review submission checklist: `paper/SUBMISSION_CHECKLIST.md`

### MEDIUM Priority
4. **Download real-world ontologies** (if disk space allows):
   ```bash
   ./benchmarks/download_ontologies.sh
   ```

5. **Run full benchmark suite:**
   ```bash
   cargo bench  # Will take time
   ```

6. **Implement missing features** (if paper requires):
   - Check `docs/IMPLEMENTATION_PLAN.md`
   - Check `docs/ROADMAP.md`

## 📚 Key Documentation Files (Read These)

1. `AGENTS.md` - Project overview, conventions, architecture
2. `docs/SPACL_ALGORITHM.md` - Algorithm details
3. `docs/IMPLEMENTATION_PLAN.md` - What's implemented vs planned
4. `paper/SUBMISSION_CHECKLIST.md` - Paper submission status
5. `README.md` - Quick start guide

## 🔍 Current Git Status

- **Branch:** main
- **Status:** Clean (pushed to origin)
- **Recent commits:**
  1. "Update paper files, add benchmark files, and update config"
  2. "Reorganize project structure: archive legacy code..."

## 🎯 If You Need to Continue Paper Work

### Quick Paper Commands:
```bash
cd paper
pdflatex main.tex
bibtex main
pdflatex main.tex
pdflatex main.tex
```

### Check These Files:
- `paper/jws_submission_final/main.pdf` - Latest compiled PDF
- `paper/main.tex` - Main source
- `paper/references/bibliography.bib` - References
- `paper/figures/*.pdf` - Generated figures

## 💡 Pro Tips

1. **Before any major work:** Run `cargo test --lib` to verify everything works
2. **Disk space check:** Run `df -h` before downloading large ontologies
3. **Benchmark results:** Saved to `target/criterion/` and `results/`
4. **Parser limits:** Check `src/util/config.rs` for file size limits

## 🆘 Troubleshooting

### If build fails:
```bash
cargo clean
cargo build --lib
```

### If benchmarks fail:
- Check disk space: `df -h`
- Check test data exists: `ls tests/data/`
- Try quick benchmark first: `cargo bench --bench quick_benchmark`

### If paper won't compile:
```bash
cd paper
rm -f *.aux *.bbl *.blg *.out
pdflatex main.tex
```

## 📎 Context Summary

This is a PhD research project for an OWL2 DL reasoner with a novel parallel algorithm (SPACL). The code is Rust-based, benchmarks use Criterion.rs, and there's a paper in progress for Journal of Web Semantics. The project was recently reorganized and is now in a clean state ready for continuation.

---

**Start by reading:** `AGENTS.md` and `docs/SPACL_ALGORITHM.md`
**Then run:** `cargo build --lib && cargo test --lib`
**Then decide:** Continue paper work OR implement more features OR run benchmarks
