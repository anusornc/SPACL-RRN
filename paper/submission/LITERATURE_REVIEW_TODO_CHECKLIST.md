# Literature Review TODO Checklist (Pre-Submission)

Purpose: finish only the remaining literature-dependent details before submission.

## How to use
- Find each marker in `paper/submission/manuscript.tex`.
- Replace `"<Add more details laters>"` with your finalized text.
- Keep claims conservative unless you have a verified quote/table/page reference.

## LIT-TODO-1
- Marker: `Literature verification note (LIT-TODO-1)`
- Location: Distributed Materialization Frameworks section
- File reference: `paper/submission/manuscript.tex`
- What to fill:
1. Add 1-3 sentences clarifying evidence quality for SPOWL/Cichlid/NORA comparisons.
2. If you add numeric speedups, include exact source location (paper section/page/table).
3. If numbers are uncertain, explicitly state that reports are heterogeneous and not directly comparable.
- Suggested fill template:
  - `<Add more details laters>` ->
  - `Reported gains in distributed materialization studies vary by dataset and rule profile; we therefore treat these works as evidence of scalability direction rather than directly comparable end-to-end speedups for full OWL2 DL reasoning.`

## LIT-TODO-2
- Marker: `Literature verification note (LIT-TODO-2)`
- Location: The Research Gap in DL Reasoning section
- File reference: `paper/submission/manuscript.tex`
- What to fill:
1. Briefly describe how you searched (keywords, venues, date range).
2. Clarify that the claim is bounded to open-source/documented systems.
3. Avoid absolute statements like "none exists" unless formally proven.
- Suggested fill template:
  - `<Add more details laters>` ->
  - `Our claim is limited to publicly documented and open-source implementations surveyed in the cited OWL reasoning and CSP/SAT literature; closed or unpublished systems may exist but were outside this review scope.`

## LIT-TODO-3
- Marker: `Literature verification note (LIT-TODO-3)`
- Location: Note on Performance (ORE/Konclude context)
- File reference: `paper/submission/manuscript.tex`
- What to fill:
1. Add precise contextual caveat on comparability (hardware, ontology mix, tasks).
2. If you cite any numeric range, provide exact source anchor (table/figure/page).
3. Keep this note as "contextual background", not a direct claim against your run.
- Suggested fill template:
  - `<Add more details laters>` ->
  - `We use ORE results as historical context only; direct numeric comparison is inappropriate without matched hardware, ontology set, and execution protocol.`

## Final checks before submit
- [ ] All three markers replaced.
- [ ] No unverified numeric claim remains in Related Work.
- [ ] No placeholder string `<Add more details laters>` remains in `manuscript.tex`.
- [ ] `paper/submission/compile.sh` passes.
