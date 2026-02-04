# Real-World Ontology Benchmarks

## Required Ontologies for SPACL Evaluation

### ORE 2015 Competition Ontologies
The ORE 2015 Reasoner Evaluation used the following ontology categories:

#### 1. Biomedical Ontologies (High Priority)
- **NCI Thesaurus** (National Cancer Institute)
  - Size: ~100,000 classes
  - Profile: OWL 2 DL
  - Source: https://ncithesaurus.nci.nih.gov/
  - File: Thesaurus.owl

- **SNOMED CT** (Systematized Nomenclature of Medicine)
  - Size: ~350,000 classes
  - Profile: OWL 2 EL (but has complex extensions)
  - Note: Requires license for full version
  - Source: https://www.snomed.org/

- **Gene Ontology (GO)**
  - Size: ~45,000 classes
  - Profile: OWL 2
  - Source: http://geneontology.org/
  - File: go.owl

#### 2. General Ontologies
- **GALEN** (Medical terminology)
  - Size: ~30,000 classes
  - Profile: OWL 2 DL
  - Source: http://www.opengalen.org/

- **LUBM** (Lehigh University Benchmark)
  - Size: 43 classes (but generates large datasets)
  - Profile: OWL 2
  - Source: Included in this repo: tests/data/univ-bench.owl

#### 3. ORE 2015 Specific
Download from: https://zenodo.org/record/2570394
(or search "ORE 2015 reasoner evaluation benchmark")

Categories:
- Classification (TBox reasoning)
- Consistency checking
- Instance checking (ABox reasoning)
- Query answering

### Alternative Sources

#### BioPortal
https://bioportal.bioontology.org/
- Download OWL files directly
- Requires API key for bulk download

#### Ontology Lookup Service (OLS)
https://www.ebi.ac.uk/ols4/
- REST API available
- Can download OWL/JSON formats

### Installation Instructions

1. Download ontologies to appropriate directories:
   ```
   ontologies/ore2015/
   ontologies/bioportal/
   ontologies/other/
   ```

2. Verify file formats (should be .owl, .rdf, or .ttl)

3. Run benchmark:
   ```
   cargo bench --bench real_world_benchmark
   ```

### File Naming Convention

Please name files descriptively:
- `nci-thesaurus.owl`
- `go-basic.owl`
- `galen-module.owl`
- etc.

### Expected Benchmark Results

Based on published ORE 2015 results:

| Ontology | Classes | Konclude (s) | Pellet (s) | HermiT (s) |
|----------|---------|--------------|------------|------------|
| NCI 1% | 2,400 | 0.8 | 12.3 | 18.5 |
| NCI 10% | 24,000 | 4.2 | 89.7 | 142.3 |
| GO | 45,000 | 2.1 | 45.2 | 67.8 |

Note: Times vary by hardware and specific classification tasks.
