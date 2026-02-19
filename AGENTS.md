# AGENTS.md

This file is the default entry-point context for coding agents in this repository.

## Priority Docs (Read First)

1. `docs/experiments/TABLEAUXX_PARSER_ALGORITHM_REFERENCE.md`
2. `docs/experiments/PARSER_SPEED_DECISION_LOG.md`

If working on parser performance, treat these two files as the current source of truth.

## Current Parser Checkpoint

- Latest script-validated parse-only (structural mode):
  - `doid.owl`: `830 ms`
  - `go-basic.owl`: `3472 ms`
  - `uberon.owl`: `2952 ms`
- Reference artifacts:
  - `benchmarks/competitors/results/history/stages_doid_20260219_130432/stage_summary.csv`
  - `benchmarks/competitors/results/history/stages_go_basic_20260219_130710/stage_summary.csv`
  - `benchmarks/competitors/results/history/stages_uberon_20260219_130942/stage_summary.csv`
- Current dominant cost moved to:
  - structural subject/object resolution (not `apply_triple_terms_core`)

## Parser Work Rules

- Target file for RDF/XML parser work:
  - `src/parser/rdf_xml_streaming.rs`
- Keep semantic behavior consistent with current handlers:
  - `apply_triple_terms_core(...)`
- Do not silently drop valid triples in strict/production paths.
- Preserve stage timing signals used by benchmark harness (`parse_time_ms`, `reason_time_ms`).

## Benchmark Rules

- Use stage split benchmarks for parser decisions:
  - `benchmarks/competitors/scripts/run_stage_benchmark.sh`
  - `benchmarks/competitors/scripts/run_stage_suite.sh`
- Compare at minimum:
  - `go-basic.owl`
  - `uberon.owl`
- Primary KPI for parser changes:
  - `parse_only` from stage summary CSV

## Required Update After Parser Changes

When parser logic changes, update both:

1. `docs/experiments/TABLEAUXX_PARSER_ALGORITHM_REFERENCE.md`
2. `docs/experiments/PARSER_SPEED_DECISION_LOG.md`

Include:
- What changed (algorithmically)
- Why it changed
- Before/after benchmark artifact paths

## Important Env Flags

- `OWL2_REASONER_STRUCTURAL_XML_PARSER`
- `OWL2_REASONER_STRUCTURAL_XML_INTERNER`
- `OWL2_REASONER_EXPERIMENTAL_XML_PARSER`
- `OWL2_REASONER_EXPERIMENTAL_XML_STRICT`
- `OWL2_REASONER_STAGE_TIMING`
- `OWL2_REASONER_FORCE_TEXT`
- `OWL2_REASONER_BIN_ONLY`
- `OWL2_REASONER_AUTO_CACHE`
