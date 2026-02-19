# parser-onto Assessment (2026-02-17)

## Scope

Evaluate whether `https://github.com/anusornc/parser-onto` contains a parser/reasoning algorithm that can be trusted as evidence of "beating ELK", and whether it can help Tableauxx parser bottlenecks.

## What Was Run

Commands (local machine):

```bash
cd /tmp/parser-onto
go build -o chebi-parser .

# chebi.owl
/usr/bin/time -f "elapsed_sec=%e max_rss_kb=%M" \
  ./chebi-parser -input /home/admindigit/tableauxx/benchmarks/ontologies/other/chebi.owl \
  -output /dev/null -format owl

# doid/go-basic/pato/uberon
for f in doid.owl go-basic.owl pato.owl uberon.owl; do
  /usr/bin/time -f "elapsed_sec=%e max_rss_kb=%M" \
    ./chebi-parser -input /home/admindigit/tableauxx/benchmarks/ontologies/other/$f \
    -output /dev/null -format owl
done
```

## Observed Runtime

`parser-onto` (OWL parse + JSON write to `/dev/null`):

| Ontology | `parser-onto` elapsed (s) |
|---|---:|
| doid.owl | 1.06 |
| go-basic.owl | 4.49 |
| pato.owl | 0.89 |
| uberon.owl | 3.86 |
| chebi.owl | 31.46 |

Reference Tableauxx parse from current head-to-head runs:

| Ontology | Tableauxx parse (s) |
|---|---:|
| doid.owl | 28.64 |
| go-basic.owl | 162.75 |
| pato.owl | 28.27 |
| uberon.owl | 151.85 |
| chebi.owl | TO at 900s |

Source runs:
- `benchmarks/competitors/results/history/head2head_doid_20260217/results.csv`
- `benchmarks/competitors/results/history/head2head_go_basic_20260217/results.csv`
- `benchmarks/competitors/results/history/head2head_pato_20260217/results.csv`
- `benchmarks/competitors/results/history/head2head_uberon_20260217/results.csv`
- `benchmarks/competitors/results/history/head2head_chebi_900_20260217/results.csv`

## Important Validity Limits

This is **not** a fair parser-vs-parser benchmark of equivalent semantics:

1. `parser-onto` OWL parser is domain-focused and drops most structures.
2. It only extracts a subset centered on OWL classes, direct `subClassOf`, and simple `owl:Restriction` with `someValuesFrom` in `ontology/owl_parser.go`.
3. Many elements are skipped by `decoder.Skip()` in default branches.
4. Repo has no test suite (`CLAUDE.md`) and benchmark script references missing paths (`./cmd/classify`).
5. Parallel reasoner path is currently a stub (`SaturateParallel` falls back to `Saturate`).

Code references:
- `/tmp/parser-onto/ontology/owl_parser.go` (default `decoder.Skip()` path and limited extraction)
- `/tmp/parser-onto/reasoner/parallel.go` (parallel fallback)
- `/tmp/parser-onto/rust-impl/src/lib.rs` (`CR10` commented as skipped)
- `/tmp/parser-onto/CLAUDE.md` (no test suite)
- `/tmp/parser-onto/rust-impl/benchmark.sh` (expects `./cmd/classify`, external `/tmp/elk`, external OFN file)

## Verdict

`parser-onto` demonstrates a very fast **domain-specific OWL extraction pipeline**, not a drop-in full OWL 2 parser replacement.

Its repository currently does **not** provide robust, reproducible evidence for the claim "beats ELK" under equivalent task/input conditions.

## Status Update (2026-02-19)

This assessment remains valid semantically (fidelity gap is still real), but Tableauxx parser speed baseline has changed significantly since this document was first written.

New Tableauxx stage-harness parse-only results:

| Ontology | Previous structural parse (ms) | Current parse (ms) | Delta |
|---|---:|---:|---:|
| doid.owl | 27307 | 830 | -96.96% |
| go-basic.owl | 158306 | 3472 | -97.81% |
| uberon.owl | 145843 | 2952 | -97.98% |

Artifacts:
- `benchmarks/competitors/results/history/stages_doid_20260219_130432/stage_summary.csv`
- `benchmarks/competitors/results/history/stages_go_basic_20260219_130710/stage_summary.csv`
- `benchmarks/competitors/results/history/stages_uberon_20260219_130942/stage_summary.csv`

## Practical Value for Tableauxx

Despite limits, it provides useful design patterns we can adapt:

1. Stream-first parsing with aggressive pre-allocation.
2. Narrow, profile-specific extraction fast path for EL workloads.
3. Specialized mode for large biomedical ontologies with strict fallback to full parser if unsupported constructs are detected.

Recommended integration strategy:

1. Add `EL-fast-parse` mode in Tableauxx as an optional path.
2. Guard with semantic checks and fallback to full parser.
3. Benchmark under the same file format, same task, same timeout policy as existing harness.
