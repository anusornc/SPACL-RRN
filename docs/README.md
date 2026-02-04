# Tableauxx Documentation

Welcome to the Tableauxx OWL2 Reasoner documentation.

## 📚 Documentation Index

### Getting Started
- [README.md](README.md) - Project overview and quick start
- [DIRECTORY_STRUCTURE.md](DIRECTORY_STRUCTURE.md) - Project directory organization
- [PROJECT_STRUCTURE.md](PROJECT_STRUCTURE.md) - Source code structure

### Algorithm & Research
- [SPACL_ALGORITHM.md](SPACL_ALGORITHM.md) - **Novel SPACL Algorithm** (Speculative Parallel Tableaux)
- [research/](research/) - Research papers and findings
  - A Novel Hybrid and Evolutionary Approach to Ontology Reasoning
  - Tableau Algorithm Research Findings
  - research_findings.md

### Development
- [IMPLEMENTATION_PLAN.md](IMPLEMENTATION_PLAN.md) - Development roadmap
- [REORGANIZATION_STATUS.md](REORGANIZATION_STATUS.md) - Project reorganization tracking
- [FINAL_STATUS.md](FINAL_STATUS.md) - Current status summary

### Reports
- [reports/](reports/) - Project reports
  - รายงานการพัฒนา Tableau Reasoner (Thai)
  - รายงานผลการทดสอบ Enhanced Reasoner (Thai)
  - แนะนำ Ontology มาตรฐานสำหรับการทดสอบ (Thai)
  - standard_ontology_benchmarks.md

### Analysis
- [codebase_analysis.md](codebase_analysis.md) - Codebase analysis
- [AGENTS.md](AGENTS.md) - Agent configuration
- [CLAUDE.md](CLAUDE.md) - Claude-specific instructions

## 🚀 Quick Links

| Topic | File |
|-------|------|
| **Novel Algorithm** | [SPACL_ALGORITHM.md](SPACL_ALGORITHM.md) |
| **Directory Layout** | [DIRECTORY_STRUCTURE.md](DIRECTORY_STRUCTURE.md) |
| **Source Structure** | [PROJECT_STRUCTURE.md](PROJECT_STRUCTURE.md) |
| **Development Plan** | [IMPLEMENTATION_PLAN.md](IMPLEMENTATION_PLAN.md) |

## 🔬 SPACL Algorithm

The **Speculative Parallel Tableaux with Adaptive Conflict Learning (SPACL)** is a novel contribution:

- **Speculative Parallelism**: Explores branches in parallel using work-stealing
- **Conflict-Driven Learning**: Learns from contradictions (nogoods)
- **Adaptive Tuning**: Self-optimizing parameters using evolutionary algorithms

See [SPACL_ALGORITHM.md](SPACL_ALGORITHM.md) for details.

## 📊 Benchmarks

Benchmarks are located in `../benches/`:
- `spacl_vs_sequential.rs` - Compare SPACL vs sequential tableaux

Results are saved to `../results/`.

## 🏗️ Project Organization

```
Project Root
├── docs/          # This directory - all documentation
├── src/           # Source code
├── benches/       # Benchmarks
├── scripts/       # Python scripts
├── assets/        # Images and diagrams
├── results/       # Benchmark results
└── tests/data/    # Test ontologies
```

See [DIRECTORY_STRUCTURE.md](DIRECTORY_STRUCTURE.md) for complete layout.

## 📝 Contributing to Documentation

When adding new documentation:
1. Place in appropriate subdirectory (`reports/`, `research/`, or root)
2. Update this README with a link
3. Follow existing naming conventions

---

**Last Updated**: February 2025
**Status**: Project reorganized and all tests passing ✅
