# Manuscript Revision Summary (Full vs Compact 16-page Version)

## Files
- Full backup: `paper/submission/manuscript_full_backup_20260302.tex`
- Current primary submission: `paper/submission/manuscript.tex`

## Page Count
- Full backup: about 42 pages
- Current primary submission: 16 pages

## What Was Removed or Compressed
- Removed the `highlights` block.
- Removed `Paper Organization`.
- Compressed `Related Work` from multiple subsections into two focused paragraphs.
- Replaced textbook-style formal material with a short implementation-aligned formal scope section.
  - Removed ALC syntax definitions.
  - Removed tableaux expansion-rules table.
  - Removed theorem/proof detail.
  - Removed long nogood-soundness proof machinery.
- Removed large algorithmic presentation overhead.
  - Removed long algorithm blocks.
  - Replaced the architecture-heavy presentation with a compact execution-flow summary.
- Compressed `Implementation` to the parts that matter for the reported evidence.
  - Kept system overview.
  - Kept structural RDF/XML parser/materialization path.
  - Removed parser feature inventory and low-level memory detail.
- Reduced `Evaluation` to the evidence that carries the paper.
  - Kept synthetic scalability table.
  - Kept adaptive-threshold behavior table.
  - Kept real-world parser-stage A/B table.
  - Kept Standard, Large, CEW, OWL2Bench tables.
  - Kept PATO and Konclude as supplementary tables.
  - Kept nogood ablation table.
  - Removed thread-scaling figure and several long support subsections.
- Compressed conclusion and availability into a shorter ending.

## What Was Reframed
- The paper is now explicitly framed as a combined systems paper.
  - Speculative reasoning remains part of the contribution.
  - Structural RDF/XML acceleration is acknowledged as the dominant source of the largest real-world wall-clock gains.
- Standard-suite results are now framed as deployment-level sanity checks rather than the strongest reasoning evidence.
- Adaptive-threshold claims are presented conservatively as a safe operating policy, not a uniquely optimal calibrated model.
- Supplementary evidence (`PATO`, `Konclude`, `ChEBI`) is clearly separated from the primary comparison basis.

## What Was Preserved
- Main benchmark story on the primary large biomedical panel.
- OWL2Bench core repeated-run evidence.
- Parser-stage A/B evidence.
- Nogood-learning evidence on inconsistent workloads.
- Public repository reference: `https://github.com/anusornc/SPACL`

## Tradeoff
- The 16-page version is much more submission-friendly and easier to read.
- The cost is reduced formal detail and reduced implementation detail compared with the 42-page version.
- The current version is the better submission candidate if the journal strongly prefers a concise manuscript.
