# Parser Speed Decision Log

Date: 2026-03-01
Owner: Tableauxx parser optimization track
Goal: Make Tableauxx parser fastest under fair, reproducible, full-fidelity conditions.

## Latest Checkpoint (2026-03-01)

- Stage benchmark contract is now:
  - `parse_only`: parse stage extracted from cold text run
  - `reason_only_stage`: reason stage extracted from the same cold text run
- `.owlbin` diagnostics moved out of the main stage harness into:
  - `benchmarks/competitors/scripts/profile_bin_cache.sh`
- Rationale:
  - `owlbin` load cost is dominated by `serde/bincode` decode and can dwarf true reasoning time.
  - This was confirmed by fresh `CHEBI` and direct binary probes on `doid.owlbin`.
- Fresh diagnostic artifacts:
  - `benchmarks/competitors/results/history/stages_chebi_20260301_080949/stage_summary.csv`
  - direct probe: `doid.owlbin` with `binary_payload_decode_done ~= 24.8s`, `binary_payload_materialize_done ~= 0.129s`
- Current dominant cost in the benchmark KPI remains text parse, not reasoning.

## Previous Parser Checkpoint (2026-02-19)

- Structural parser is now the active optimization track.
- Script-validated parse-only results (large suite samples):
  - `doid.owl`: `11284 ms`
  - `go-basic.owl`: `60822 ms`
  - `uberon.owl`: `60795 ms`
- Compared to previous structural baselines:
  - doid: `-58.68%`
  - go-basic: `-61.58%`
  - uberon: `-58.31%`
- Current dominant cost is no longer `apply_triple_terms_core`; it shifted to structural subject/object resolution.

## 1) Benchmark Contract (No Guessing)

- Operation: `consistency`
- Suite: `large`
- Harness: `benchmarks/competitors/scripts/run_stage_benchmark.sh` and `benchmarks/competitors/scripts/run_stage_suite.sh`
- Fair split:
  - `parse_only`: parse stage extracted from cold text run
  - `reason_only_stage`: reason stage extracted from the same cold text run
- Optional diagnostics:
  - `benchmarks/competitors/scripts/profile_bin_cache.sh` for `.owlbin` load profiling
- Key KPI: `parse_time_ms` (primary), `wall_time_ms` (secondary), `reason_time_ms` (diagnostic)

## 2) Evidence That Bottleneck Is Parser

Stage suite run:
- `benchmarks/competitors/results/history/stage_suite_20260218_141806/stage_suite_summary.csv`

Observed parse share:
- doid: 92.07%
- go-basic: 97.57%
- pato: 92.43%
- uberon: 97.64%
- chebi: 98.47%

Conclusion:
- Parser is the dominant bottleneck across all large ontologies.
- Reasoner stage is already small (ms to low hundreds of ms).

## 3) Algorithmic Techniques Already Applied

- Streaming RDF/XML parse with per-triple processing.
- Hot predicate tagging (fast path for common OWL/RDFS predicates).
- IRI caches (`FxHashMap`) for predicate/object.
- Blank-node label and blank-node IRI caches.
- Subject cursor optimization to avoid repeated subject resolution and extra allocations.
- Pre-sized cache initialization to reduce rehash/allocation churn.
- Optional experimental pipeline (batched + interner + generational cache) under feature/env gates.

## 4) Latest Controlled A/B (Post Patch)

Baselines (from stage suite):
- go-basic baseline parse_only: `166257 ms`
  - `benchmarks/competitors/results/history/stages_go_basic_20260218_141939/stage_summary.csv`
- uberon baseline parse_only: `148011 ms`
  - `benchmarks/competitors/results/history/stages_uberon_20260218_142905/stage_summary.csv`

New runs after patch:
- go-basic new parse_only: `160557 ms` (improved ~3.43%)
  - `benchmarks/competitors/results/history/stages_go_basic_20260218_180356/stage_summary.csv`
- uberon new parse_only: `149460 ms` (regressed ~0.98% in this run)
  - `benchmarks/competitors/results/history/stages_uberon_20260218_181136/stage_summary.csv`

Interpretation:
- Improvement exists but is not yet robust across all large ontologies.
- Current changes are incremental, not structural breakthrough.

## 5) Stop-Rule (To Avoid Endless Trial-and-Error)

Use these gates for every optimization cycle:

1. Run A/B on `go-basic` and `uberon` with identical harness and env.
2. Accept a patch only if both conditions hold:
   - Mean parse improvement >= 5% across both files.
   - No regression worse than 1% on either file.
3. If 2 consecutive patch cycles fail the gate:
   - Stop micro-tuning.
   - Move to structural parser redesign phase.

## 6) Structural Phase Trigger and Scope

Trigger:
- Gate failure as defined above.

Scope:
- Two-phase parse architecture:
  - Phase A: high-throughput ingestion into compact intermediate representation.
  - Phase B: deterministic full-fidelity OWL axiom materialization.
- Preserve correctness (no silent drop of valid triples in strict mode).
- Keep reproducible benchmark outputs under existing harness.

## 7) Why This Is Defensible

- Decisions are grounded in measured stage split, not intuition.
- Every patch is judged by a fixed gate and reproducible artifacts.
- This creates a clear path from incremental tuning to structural redesign when required.

## 8) Structural Phase Start (Implemented)

Implemented:
- New two-phase structural parser mode behind env flag:
  - `OWL2_REASONER_STRUCTURAL_XML_PARSER=1`
- Code path:
  - `src/parser/rdf_xml_streaming.rs`
  - Phase A: ingest + interning to compact records
  - Phase B: materialize full ontology axioms via existing handlers
- Benchmark env forwarding added in:
  - `benchmarks/competitors/scripts/run_benchmarks.sh`

Validation runs:
- doid (structural):
  - `benchmarks/competitors/results/history/stages_doid_20260218_183604/stage_summary.csv`
  - parse_only `27307 ms` vs baseline `27769 ms` (~`-1.66%`)
- go-basic (structural):
  - `benchmarks/competitors/results/history/stages_go_basic_20260218_185126/stage_summary.csv`
  - parse_only `158306 ms` vs baseline `160557 ms` (~`-1.40%`)
- uberon (structural):
  - `benchmarks/competitors/results/history/stages_uberon_20260218_185933/stage_summary.csv`
  - parse_only `145843 ms` vs baseline `149460 ms` (~`-1.92%`)

Takeaway:
- Structural phase is active and measurable.
- Current gain is consistent but moderate (~1.4% to ~1.9% on large ontologies).
- Next iterations must target larger structural wins, not micro-cache tuning alone.

## 9) Deep Apply Profiling (2026-02-19)

Added instrumentation:
- `structural_apply_breakdown` emitted from `src/parser/rdf_xml_streaming.rs`
- Enabled via `OWL2_REASONER_STRUCTURAL_BREAKDOWN=1`

Measured hotspots (pre-optimization):
- doid:
  - apply share in materialize: `58.74%`
  - inside apply: `rdf:type 48.71%`, `other 43.94%`, `subclass 7.30%`
- go-basic:
  - apply share in materialize: `60.96%`
  - inside apply: `rdf:type 51.71%`, `other 36.54%`, `subclass 11.70%`
- uberon:
  - apply share in materialize: `59.30%`
  - inside apply: `rdf:type 62.82%`, `other 32.38%`, `subclass 4.75%`

Conclusion:
- `apply_triple_terms_core` is still dominant.
- Most time in `apply` is `rdf:type` and generic `other` property path.

## 10) Attempt That Failed (Documented Rollback)

Attempt:
- Deferred axiom runtime index rebuild (`begin_deferred_axiom_indexing/end_deferred_axiom_indexing`)

Result:
- doid parse regressed from ~`27.8s` to ~`29.2s`

Action:
- Reverted completely.
- Kept only profiling instrumentation.

## 11) High-Impact Fix Landed (2026-02-19)

Change:
- Added ontology membership checks to avoid repeated entity object construction:
  - `Ontology::contains_class_iri`
  - `Ontology::contains_named_individual_iri`
- Applied in:
  - `handle_type_assertion`
  - `handle_property_assertion`
  - file: `src/parser/rdf_xml_streaming.rs`

Why it worked:
- Prevented repeated `NamedIndividual::new(...)` and duplicate insert path for already-known IRIs.
- Reduced expensive work in `rdf:type` and `other` apply branches.

Measured improvement (local structural runs, same env):
- doid: `27778 ms` -> `21034 ms` (`-24.28%`)
- go-basic: `157342 ms` -> `124701 ms` (`-20.74%`)
- uberon: `148383 ms` -> `127129 ms` (`-14.32%`)

Updated apply composition after fix:
- doid: `rdf:type 75.24%`, `other 11.96%`, `subclass 12.74%`
- go-basic: `rdf:type 75.75%`, `other 7.90%`, `subclass 16.28%`
- uberon: `rdf:type 74.78%`, `other 16.88%`, `subclass 8.28%`

Next target:
- Optimize `rdf:type` path specifically (now dominant inside apply after de-dup fix).

## 12) Major Hot-Path Win (2026-02-19, Round 2)

Change:
- Replaced hot-path entity constructors that hit shared-IRI cache (`Class::new`, `NamedIndividual::new`, etc.)
  with direct shared-IRI construction via `Entity::from_shared_iri(Arc<IRI>)` inside parser handlers.
- Applied in:
  - `handle_type_assertion`
  - `handle_subclass_of`
  - `handle_domain`
  - `handle_property_assertion`
  - file: `src/parser/rdf_xml_streaming.rs`

Rationale:
- `*_::new` calls `create_entity_with_fallback` and shared-cache lookup path per triple.
- In parser hot loop this duplicated cache work even when `IRI` was already resolved.
- Using `from_shared_iri` avoids repeated cache roundtrips and reduces allocator/hash pressure.

Measured improvement (local structural runs, force text):
- doid: `21034 ms` -> `10796 ms` (`-48.67%`)
- go-basic: `124701 ms` -> `60091 ms` (`-51.81%`)
- uberon: `127129 ms` -> `60077 ms` (`-52.74%`)

New materialize composition after round 2:
- doid:
  - subject `55.43%`, object `40.26%`, apply `3.43%`
- go-basic:
  - subject `63.94%`, object `32.49%`, apply `2.85%`
- uberon:
  - subject `79.13%`, object `17.49%`, apply `2.71%`

Implication:
- `apply_triple_terms_core` is no longer the primary bottleneck.
- Next optimization focus must move to structural subject/object resolution paths.

Harness validation (stage benchmark script):
- doid:
  - old structural parse_only: `27307 ms`
    - `benchmarks/competitors/results/history/stages_doid_20260218_183604/stage_summary.csv`
  - new parse_only: `11284 ms`
    - `benchmarks/competitors/results/history/stages_doid_20260219_105105/stage_summary.csv`
  - improvement: `-58.68%`
- go-basic:
  - old structural parse_only: `158306 ms`
    - `benchmarks/competitors/results/history/stages_go_basic_20260218_185126/stage_summary.csv`
  - new parse_only: `60822 ms`
    - `benchmarks/competitors/results/history/stages_go_basic_20260219_105409/stage_summary.csv`
  - improvement: `-61.58%`
- uberon:
  - old structural parse_only: `145843 ms`
    - `benchmarks/competitors/results/history/stages_uberon_20260218_185933/stage_summary.csv`
  - new parse_only: `60795 ms`
    - `benchmarks/competitors/results/history/stages_uberon_20260219_105847/stage_summary.csv`
  - improvement: `-58.31%`

Mean improvement across 3 large ontologies (script-validated): `-59.52%`.

## 13) Structural Cache Indexing (2026-02-19, Round 3)

Change:
- Replaced hot `FxHashMap<StructuralTermId, IRI>` object cache with dense indexed cache:
  - `Vec<Option<IRI>>` keyed directly by `term_id`.
- Removed redundant named-subject cache layer and reused object IRI cache for named subjects.
- Kept bnode and predicate caches as hash maps for now (smaller win/risk profile).

Why:
- `term_id` is dense (`0..N`) from `StructuralInterner`, so hash map lookup was avoidable.
- Subject/object resolution dominated materialize after round 2.

Script-validated results:
- doid:
  - previous round: `11284 ms`
    - `benchmarks/competitors/results/history/stages_doid_20260219_105105/stage_summary.csv`
  - round 3: `11172 ms`
    - `benchmarks/competitors/results/history/stages_doid_20260219_114149/stage_summary.csv`
  - delta: `-0.99%`
- go-basic:
  - previous round: `60822 ms`
    - `benchmarks/competitors/results/history/stages_go_basic_20260219_105409/stage_summary.csv`
  - round 3: `60036 ms`
    - `benchmarks/competitors/results/history/stages_go_basic_20260219_114253/stage_summary.csv`
  - delta: `-1.29%`
- uberon:
  - previous round: `60795 ms`
    - `benchmarks/competitors/results/history/stages_uberon_20260219_105847/stage_summary.csv`
  - round 3: `60572 ms`
    - `benchmarks/competitors/results/history/stages_uberon_20260219_114729/stage_summary.csv`
  - delta: `-0.37%`

Round-3 mean improvement vs previous round: `-0.88%`.
Overall mean improvement vs initial structural baseline: `-59.88%`.

## 14) Structural IRI Fast Path (2026-02-19, Round 4)

Change:
- In structural materialization path, switched IRI construction from global-cache path to local fast path:
  - `IRI::new_unchecked(...)` for structural named-node and predicate IRIs.
- Converted remaining structural caches to dense indexed vectors:
  - bnode subject cache: `Vec<Option<IRI>>`
  - predicate cache: `Vec<Option<IRI>>`
  - bnode object label cache: `Vec<Option<String>>`

Why:
- Structural term IDs are dense and local; hash-map lookups and global IRI-cache churn were unnecessary.
- `IRI::new_unchecked` keeps minimal validity checks (non-empty + contains `:`) while bypassing global cache contention.

Script-validated results (stage harness):
- doid:
  - previous round: `11172 ms`
    - `benchmarks/competitors/results/history/stages_doid_20260219_114149/stage_summary.csv`
  - round 4: `830 ms`
    - `benchmarks/competitors/results/history/stages_doid_20260219_130432/stage_summary.csv`
  - delta: `-92.57%`
- go-basic:
  - previous round: `60036 ms`
    - `benchmarks/competitors/results/history/stages_go_basic_20260219_114253/stage_summary.csv`
  - round 4: `3472 ms`
    - `benchmarks/competitors/results/history/stages_go_basic_20260219_130710/stage_summary.csv`
  - delta: `-94.22%`
- uberon:
  - previous round: `60572 ms`
    - `benchmarks/competitors/results/history/stages_uberon_20260219_114729/stage_summary.csv`
  - round 4: `2952 ms`
    - `benchmarks/competitors/results/history/stages_uberon_20260219_130942/stage_summary.csv`
  - delta: `-95.13%`

Round-4 mean improvement vs previous round: `-93.97%`.
Overall mean improvement vs initial structural baseline: `-97.58%`.
