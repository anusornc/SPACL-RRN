# Real-World Ontology Benchmark Setup

## Summary

Successfully created a working benchmark infrastructure for real-world ontologies.

## What Was Done

### 1. Created Ontology Download Script
**File**: `benchmarks/download_ontologies.sh`

- Downloads freely available ontologies
- Currently downloaded: **Gene Ontology (GO)** - 112MB, ~45,000 classes
- Creates README with instructions for obtaining other ontologies:
  - NCI Thesaurus
  - SNOMED CT
  - GALEN
  - Other ORE 2015 benchmarks

### 2. Created Benchmark Code
**File**: `benches/ontology_benchmark.rs`

Features:
- Loads OWL files (RDF/XML format)
- Runs both Sequential and SPACL reasoners
- Uses Criterion.rs for statistical benchmarking
- Reports mean time with confidence intervals

### 3. Registered in Cargo.toml
Added to `Cargo.toml`:
```toml
[[bench]]
name = "ontology_benchmark"
harness = false
```

## Benchmark Results

### LUBM (Lehigh University Benchmark)
- **Classes**: 8 (small ontology)
- **Sequential**: 3.27 µs
- **SPACL**: 6.66 µs
- **Observation**: SPACL has overhead on tiny ontologies (expected)

### GO (Gene Ontology)
- **Status**: File too large (112MB)
- **Issue**: Parser has file size limits
- **Solution needed**: Increase parser limits or use fragments

## How to Run

```bash
# Run ontology benchmarks
cargo bench --bench ontology_benchmark

# Download more ontologies
bash benchmarks/download_ontologies.sh
```

## Next Steps for Real-World Evaluation

### 1. Get NCI Thesaurus
```bash
# Download from: https://ncithesaurus.nci.nih.gov/
# Place in: benchmarks/ontologies/bioportal/
```

### 2. Handle Large Files
The GO ontology (112MB) exceeds current parser limits. Options:
- Increase `MAX_FILE_SIZE` in parser config
- Use ontology fragments/slices
- Pre-process to extract relevant axioms

### 3. Run Full Benchmark Suite
```bash
# After obtaining ontologies:
cargo bench --bench ontology_benchmark -- --output-format=csv > results.csv
```

### 4. Update Paper
Once you have real results, update the paper:
- Replace "Planned Real-World Evaluation" with actual results
- Update Table 4 with real timing data
- Remove limitation about synthetic benchmarks only

## Files Modified/Created

| File | Purpose |
|------|---------|
| `benchmarks/download_ontologies.sh` | Download real ontologies |
| `benches/ontology_benchmark.rs` | Benchmark runner |
| `benchmarks/ontologies/other/go-basic.owl` | Downloaded GO ontology (112MB) |
| `Cargo.toml` | Added bench entries |
| `benches/real_world_benchmark.rs` | Alternative comprehensive benchmark |

## Known Limitations

1. **Parser File Size Limit**: GO (112MB) exceeds current limit
2. **Memory Usage**: Large ontologies may require >64GB RAM
3. **NCI/GALEN/SNOMED**: Need manual download (licensing)
4. **Format Support**: Only RDF/XML tested; OWL Functional Syntax not fully implemented
