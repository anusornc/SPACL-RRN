# Tableauxx Parser Algorithm Reference

Date: 2026-03-01
Purpose: Stable parser-algorithm reference for future regression checks and architecture diffs.
Scope: Current parser stack used by `owl2-reasoner` and benchmark harness.

## 1. Top-Level Load Algorithm (`load_ontology_with_env`)

Code reference:
- `src/util/ontology_io.rs:174`

High-level logic:

```text
load_ontology_with_env(path):
  read env: BIN_ONLY, FORCE_TEXT, AUTO_CACHE
  derive bin_path from input (.owl -> .owlbin)

  if BIN_ONLY:
    require bin_path exists
    return deserialize(bin_path)

  if input is .owlbin:
    if FORCE_TEXT: error
    return deserialize(input)

  if !FORCE_TEXT and bin_path exists:
    try deserialize(bin_path)
      success -> return
      fail -> fallback to text parse

  configure IRI cache heuristic for large files
  parser = detect_parser(path)

  try parser.parse_file(path)
    success -> ontology
    fail:
      if fallback allowed -> parse_text(path)
      else -> error

  if AUTO_CACHE:
    serialize ontology to bin_path

  return ontology
```

Key behavior:
- Binary fast-path is preferred unless `FORCE_TEXT=1`.
- Text fallback policy controlled by:
  - `OWL2_REASONER_DISABLE_PARSE_FALLBACK`
  - `OWL2_REASONER_ENABLE_PARSE_FALLBACK`
  - `OWL2_REASONER_LARGE_PARSE`
- Stage timing logs are emitted when `OWL2_REASONER_STAGE_TIMING=1`.
- As of `2026-03-01`, benchmark stage-split decisions should not treat `.owlbin` load time as a reasoning proxy.
  Fresh probes show binary load cost is dominated by payload decode (`serde/bincode`) rather than
  ontology reasoning, so the stage harness now takes the reasoning metric from the same cold text run
  and moves bin-cache diagnostics to `benchmarks/competitors/scripts/profile_bin_cache.sh`.

## 2. Parser Selection (`ParserFactory`)

Code references:
- `src/parser/mod.rs:127` (`auto_detect`)
- `src/parser/mod.rs:110` (content-type mapping)
- `src/parser/rdf_xml.rs:36` (RDF/XML parser entry)

For RDF/XML:
- `ParserFactory::auto_detect` chooses `RdfXmlParser` when XML/RDF signatures are present.
- `RdfXmlParser` uses streaming parser in non-strict mode, with legacy fallback on error.

## 3. RDF/XML Mode Selection

Code references:
- `src/parser/rdf_xml_streaming.rs:1498`
- `src/parser/rdf_xml_streaming.rs:1503`
- `src/parser/rdf_xml_streaming.rs:1555`

Current precedence:

```text
parse_content():
  if EXPERIMENTAL_XML_PARSER=1 and feature enabled -> experimental mode
  else if STRUCTURAL_XML_PARSER=1 -> structural mode
  else -> standard streaming mode

parse_file():
  same precedence as parse_content()

parse_stream():
  if STRUCTURAL_XML_PARSER=1 -> structural mode
  else -> standard streaming mode
```

Notes:
- Experimental mode is gated by compile feature + env.
- Structural mode is env-only and uses the same semantic handlers as standard mode.

## 4. Standard Streaming Algorithm (Current Default Path)

Code references:
- `src/parser/rdf_xml_streaming.rs:1555`
- `src/parser/rdf_xml_streaming.rs:1595`
- `src/parser/rdf_xml_streaming.rs:1628`
- `src/parser/rdf_xml_streaming.rs:1667`

Pseudo-flow:

```text
parse_stream(reader):
  init parser state and progress tracker
  rio_xml.parse_all(triple => process_triple(triple))

process_triple(triple):
  subject_iri = subject_to_iri_with_cursor(triple.subject)
  predicate_tag = predicate_tag(triple.predicate.iri)
  object = process_object(triple.object)
  apply_triple_terms_core(subject_iri, predicate_tag, object)

subject_to_iri_with_cursor(subject):
  if subject equals last_subject (kind + lexical value):
    return cached IRI
  else:
    resolve via cached_object_iri / cached_blank_node_iri
    update cursor
    return resolved IRI
```

Semantic dispatch in `apply_triple_terms_core`:
- `rdf:type` -> type assertions
- `rdfs:subClassOf` -> subclass axioms
- `rdfs:domain` -> domain axioms
- `rdfs:range` -> range axioms
- `owl:disjointWith`, `owl:equivalentClass`, other `owl:*` -> OWL property handlers
- other predicates -> generic property assertions

## 5. Structural Two-Phase Algorithm (Current Structural Phase)

Code references:
- data model: `src/parser/rdf_xml_streaming.rs:84`
- interner: `src/parser/rdf_xml_streaming.rs:115`
- pipeline: `src/parser/rdf_xml_streaming.rs:1290`

Phase A (ingest + intern):

```text
for each triple from rio parser:
  convert subject/predicate/object to compact IDs using StructuralInterner
  store StructuralTripleRecord {subject_id, predicate_id, object_record}
```

Phase B (materialize):

```text
for each StructuralTripleRecord:
  resolve IDs -> subject IRI / predicate str / object term
  compute predicate_tag
  call apply_triple_terms_core(...)
```

Design goal:
- Split parsing and semantic materialization into separate phases.
- Reuse existing semantic handlers for fidelity consistency.

## 6. Experimental Parallel XML Algorithm (Feature-Gated)

Code references:
- `src/parser/rdf_xml_streaming.rs:1050`

Pipeline summary:
- Producer parses RDF/XML and emits raw batches.
- Worker threads intern and build compact records.
- Records are sorted by sequence and materialized through shared semantic path.
- Strict mode can fail on skipped unsupported terms.

## 7. Key Data Structures in Current Parser

- `PredicateTag` for fast predicate routing:
  - `src/parser/rdf_xml_streaming.rs:64`
- Subject cursor state:
  - `src/parser/rdf_xml_streaming.rs:78`
- Cache layers:
  - predicate/object/bnode caches in parser state:
    - `src/parser/rdf_xml_streaming.rs:347`
- Structural records/interner:
  - `src/parser/rdf_xml_streaming.rs:84`
  - `src/parser/rdf_xml_streaming.rs:115`

## 8. Environment Flags (Active Parser Control)

Primary flags:
- `OWL2_REASONER_FORCE_TEXT`
- `OWL2_REASONER_BIN_ONLY`
- `OWL2_REASONER_AUTO_CACHE`
- `OWL2_REASONER_LARGE_PARSE`
- `OWL2_REASONER_DISABLE_PARSE_FALLBACK`
- `OWL2_REASONER_ENABLE_PARSE_FALLBACK`
- `OWL2_REASONER_STAGE_TIMING`

RDF/XML mode flags:
- `OWL2_REASONER_EXPERIMENTAL_XML_PARSER`
- `OWL2_REASONER_EXPERIMENTAL_XML_STRICT`
- `OWL2_REASONER_STRUCTURAL_XML_PARSER`
- `OWL2_REASONER_STRUCTURAL_XML_INTERNER`

Forwarding to benchmark containers:
- `benchmarks/competitors/scripts/run_benchmarks.sh:64`

## 9. Change-Tracking Protocol (Required on Parser Changes)

When parser logic changes, update this file with:
1. Date + short rationale.
2. Affected algorithm section(s).
3. Updated function references (file + line).
4. Before/after benchmark artifacts:
   - stage benchmark CSV paths
   - parse-only deltas for `go-basic` and `uberon` at minimum

Suggested quick commands:

```bash
rg -n "parse_stream_structural|parse_stream_experimental|process_triple|apply_triple_terms_core" src/parser/rdf_xml_streaming.rs
rg -n "load_ontology_with_env|detect_parser|parse_text" src/util/ontology_io.rs
```

## 10. Current Status Snapshot

As of this document:
- Parser bottleneck is confirmed by stage split (parse dominates).
- Structural phase has started and is integrated.
- Structural path currently provides measurable parse improvements on large ontologies.
- Stage harness contract uses `parse_only` + `reason_only_stage` from the same cold text run.
- `.owlbin` diagnostics live in `benchmarks/competitors/scripts/profile_bin_cache.sh` because decode can dominate large-file timings.

## 11. Latest Update (2026-02-19)

New instrumentation:
- `structural_apply_breakdown` in `src/parser/rdf_xml_streaming.rs`
- Enabled by `OWL2_REASONER_STRUCTURAL_BREAKDOWN=1`

Latest optimization:
- Added ontology membership checks to avoid repeated construction/insertion attempts for:
  - class declarations
  - named individuals
- Implemented in:
  - `src/core/ontology.rs` (`contains_class_iri`, `contains_named_individual_iri`)
  - `src/parser/rdf_xml_streaming.rs` (`handle_type_assertion`, `handle_property_assertion`)
- Replaced hot-path entity constructors with direct shared-IRI constructors in parser handlers:
  - `Entity::from_shared_iri(Arc<IRI>)` for `Class`, `NamedIndividual`, `ObjectProperty`, `DataProperty`
  - avoids repeated shared-cache lookup during materialization
- Replaced structural object IRI cache lookup from hash map to dense indexed cache:
  - `Vec<Option<IRI>>` indexed by `StructuralTermId`
  - removed redundant named-subject cache layer for named-node subjects
- Switched structural IRI construction to fast local path:
  - `IRI::new_unchecked(...)` in structural object/predicate/subject resolution
  - keeps minimal validity checks while bypassing global IRI cache

Measured local parse-time deltas (structural mode, force text CLI):
- doid: `27778 ms -> 10796 ms` (`-61.14%`)
- go-basic: `157342 ms -> 60091 ms` (`-61.81%`)
- uberon: `148383 ms -> 60077 ms` (`-59.51%`)

Script-validated parse-only deltas (stage harness):
- doid: `27307 ms -> 830 ms` (`-96.96%`)
  - old: `benchmarks/competitors/results/history/stages_doid_20260218_183604/stage_summary.csv`
  - new: `benchmarks/competitors/results/history/stages_doid_20260219_130432/stage_summary.csv`
- go-basic: `158306 ms -> 3472 ms` (`-97.81%`)
  - old: `benchmarks/competitors/results/history/stages_go_basic_20260218_185126/stage_summary.csv`
  - new: `benchmarks/competitors/results/history/stages_go_basic_20260219_130710/stage_summary.csv`
- uberon: `145843 ms -> 2952 ms` (`-97.98%`)
  - old: `benchmarks/competitors/results/history/stages_uberon_20260218_185933/stage_summary.csv`
  - new: `benchmarks/competitors/results/history/stages_uberon_20260219_130942/stage_summary.csv`
