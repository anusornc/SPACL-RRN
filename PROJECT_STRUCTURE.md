# Tableauxx Project Structure

This document describes the organized directory structure of the Tableauxx project.

## Root Directory

```
tableauxx/
├── README.md              # Main project documentation
├── PROJECT_STRUCTURE.md   # This file
├── Cargo.toml            # Rust project configuration
├── Cargo.lock            # Dependency lock file
├── LICENSE               # MIT License
├── .gitignore           # Git ignore rules
│
├── src/                 # Source code (Rust)
├── tests/               # Test files and data
├── benches/             # Benchmark code
├── examples/            # Example applications
├── benchmarks/          # Benchmark data and ontologies
├── scripts/             # Utility scripts
├── results/             # Benchmark results
├── assets/              # Static assets
│
├── admin/               # Project administration
├── paper/               # Journal paper and submission
├── docs/                # Project documentation
├── archive/             # Archived/old files
└── backup_tableauxx/    # Backup files
```

## Directory Details

### `src/` - Source Code
Rust implementation of the OWL2 DL reasoner.

```
src/
├── app/           # Application logic
├── bin/           # Binary executables
├── core/          # Core data structures
├── logic/         # Logic and reasoning
├── parser/        # OWL parsers
├── reasoner/      # Reasoner implementations
├── serializer/    # Output serializers
├── storage/       # Storage backends
├── strategy/      # Reasoning strategies
└── util/          # Utility functions
```

### `admin/` - Project Administration
Project management documents organized by category.

```
admin/
├── acceptance/    # Acceptance and submission docs
│   ├── ACCEPTANCE_NOTIFICATION.md
│   ├── FINAL_REVISIONS_FOR_ACCEPTANCE.md
│   ├── FINAL_SUBMISSION_PACKAGE.md
│   └── REVIEWER_RESPONSE_PACKAGE.md
│
├── planning/      # Planning and roadmaps
│   ├── KIMI_HANDOFF_PROMPT.md
│   ├── OPTION_A_IMPLEMENTATION_PLAN.md
│   ├── OWL2_DL_IMPLEMENTATION_ROADMAP.md
│   └── PHASE3_OPTIMIZATIONS.md
│
├── reports/       # Progress and completion reports
│   ├── BENCHMARK_ANALYSIS_2026_02_06.md
│   ├── COMPLETION_REPORT.md
│   ├── COMPLETION_REPORT_FINAL.md
│   ├── ETHICAL_CORRECTIONS_REPORT.md
│   ├── FINAL_ETHICAL_VALIDATION.md
│   ├── FINAL_PROJECT_SUMMARY.md
│   ├── FINAL_SUMMARY.md
│   ├── PHASE1_IMPLEMENTATION_SUMMARY.md
│   ├── PHASE2_IMPLEMENTATION_SUMMARY.md
│   ├── PERFORMANCE_ROADMAP.md
│   └── UPDATED_BENCHMARK_RESULTS_20260207.md
│
└── revisions/     # Revision tracking
    ├── MANUSCRIPT_REVISIONS.md
    ├── NOGOOD_AUDIT_FINDINGS.md
    ├── REVIEW_RESPONSE_PLAN.md
    └── SCOPE_FIXES_CHECKLIST.md
```

### `paper/` - Journal Paper
All materials related to the Journal of Web Semantics submission.

```
paper/
├── submission/           # Final submission files
│   ├── manuscript.tex   # LaTeX source
│   ├── manuscript.pdf   # Compiled PDF
│   ├── references.bib   # Bibliography (36 refs)
│   ├── compile.sh       # Compilation script
│   └── MARKED_SECTIONS.txt  # Citation analysis
│
├── guides/              # Reference guides and validation
│   ├── ACADEMIC_INTEGRITY_WARNING.md
│   ├── PAPER_REVISION_GUIDE.md
│   ├── REFERENCE_VALIDATION_REPORT.md
│   ├── REVISION_SUMMARY.md
│   └── VERIFICATION_GUIDE.md
│
├── downloads/           # Downloaded reference PDFs
│   ├── *.pdf           # Downloaded papers
│   ├── MANUAL_DOWNLOAD_GUIDE.md
│   └── download_*.sh   # Download scripts
│
├── CHANGES_IMPLEMENTED.md   # Change history
├── CHANGES_SUMMARY.md       # Summary of changes
├── PROJECT_STATUS.md        # Current status
├── SUBMISSION_CHECKLIST.md  # Pre-submission checklist
│
├── elsarticle/          # LaTeX class files
├── figures/             # Figure files
├── tables/              # Table files
└── references/          # Reference management
```

### `docs/` - Project Documentation
Technical documentation for developers.

```
docs/
├── AGENTS.md                    # Agent guidelines
├── codebase_analysis.md         # Code analysis
├── DIRECTORY_STRUCTURE.md       # Directory info
├── FINAL_STATUS.md             # Final status
├── IMPLEMENTATION_PLAN.md      # Implementation plan
├── IMPLEMENTATION_TRACKING.md  # Progress tracking
├── NEXT_STEPS_PLAN.md          # Next steps
├── PROJECT_REORGANIZATION_COMPLETE.md
├── PROJECT_STRUCTURE.md        # Structure info
├── QUICK_START.md             # Quick start guide
├── README.md                  # Docs readme
├── REORGANIZATION_STATUS.md   # Reorg status
└── ROADMAP.md                 # Project roadmap

reports/       # Various reports
research/      # Research findings
```

### `tests/` - Test Suite
Test files and test data.

```
tests/
├── data/              # Test ontologies and data
│   ├── *.owl         # OWL test files
│   └── hierarchy_*/   # Hierarchy test data
└── *.rs              # Test code
```

### `benchmarks/` - Benchmark Data
Benchmark ontologies and competitor reasoners.

```
benchmarks/
├── competitors/       # Competitor reasoner JARs
└── ontologies/        # Benchmark ontologies
    ├── CHEBI/
    ├── DOID/
    ├── GO/
    ├── PATO/
    └── UBERON/
```

## Quick Access

### For Paper Submission:
```bash
cd paper/submission
./compile.sh          # Compile manuscript
```

### For Reference Verification:
```bash
cd paper/guides
cat PAPER_REVISION_GUIDE.md    # Search terms for citations
cat REVISION_SUMMARY.md        # Action plan
```

### For Development:
```bash
# Build
cargo build --release

# Test
cargo test

# Run benchmarks
cargo bench
```

## Important Files

| File | Location | Purpose |
|------|----------|---------|
| Main README | `/README.md` | Project overview |
| Project Structure | `/PROJECT_STRUCTURE.md` | This guide |
| Project Status | `/paper/PROJECT_STATUS.md` | Current status & checklist |
| Changes Log | `/paper/CHANGES_IMPLEMENTED.md` | Change history |
| Manuscript | `/paper/submission/manuscript.tex` | Paper source |
| Compile Script | `/paper/submission/compile.sh` | Build PDF |
| Reference Guide | `/paper/guides/PAPER_REVISION_GUIDE.md` | Citation guide |
| Revision Summary | `/paper/guides/REVISION_SUMMARY.md` | Action plan |

## Notes

- **Repository URL**: https://github.com/anusornc/tableauxx
- **Paper Status**: Ready for final verification
- **References**: 36 cited (cleaned from 57)
- **Pre-submission**: All 36 citations need verification

## Last Updated

February 8, 2026
