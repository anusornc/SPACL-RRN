# Submission Checklist

## Primary Files
- [ ] Final manuscript source is `paper/submission/manuscript.tex`
- [ ] Final compiled PDF is `paper/submission/manuscript.pdf`
- [ ] Page count is acceptable for the target journal
- [ ] Title in manuscript matches title in cover letter

## Consistency Checks
- [ ] System name is `SPACL` throughout the manuscript
- [ ] Public repository URL is `https://github.com/anusornc/SPACL`
- [ ] Abstract, introduction, conclusion, and cover letter use the same framing
- [ ] Main claim is framed as a combined reasoning-and-parser systems contribution
- [ ] Supplementary results (`PATO`, `ChEBI`, `Konclude`) are described as supplementary everywhere

## Technical Checks
- [ ] Manuscript compiles cleanly with `paper/submission/compile.sh`
- [ ] No undefined citations or references remain in `paper/submission/manuscript.log`
- [ ] Tables and figure references mentioned in the text are present and labeled correctly
- [ ] Benchmark numbers in abstract, body, and conclusion agree
- [ ] Timeout values are consistent across abstract, protocol, and tables

## Benchmark/Artifact Framing
- [ ] Synthetic scalability results are described as supporting evidence only
- [ ] Primary real-world evidence is clearly identified as DOID / GO-Basic / UBERON
- [ ] OWL2Bench is described as repeated-run external validation
- [ ] Standard suite is framed as deployment-level sanity evidence, not the strongest reasoning proof
- [ ] Adaptive-threshold wording remains conservative

## Availability and Reproducibility
- [ ] Public repo contains the code paths referenced in the manuscript
- [ ] `paper/submission/README.txt` still matches the current manuscript framing
- [ ] Artifact paths mentioned in the manuscript are still valid in the private repo
- [ ] Cover letter mentions the public repository only

## Administrative Items
- [ ] Author names, affiliations, and emails are correct
- [ ] Corresponding author line is correct
- [ ] Acknowledgments are intentional and up to date
- [ ] The originality / not-under-review statement in the cover letter is acceptable
- [ ] Any journal-specific metadata fields required by the submission system are prepared separately

## Local Backup Set
- [ ] Full backup retained: `paper/submission/manuscript_full_backup_20260302.tex`
- [ ] Full backup PDF retained: `paper/submission/manuscript_full_backup_20260302.pdf`
- [ ] Mid-length variant retained: `paper/submission/manuscript_mid_20260302.tex`

## Final Pre-Submit Actions
- [ ] Re-read abstract and conclusion once more after any last edits
- [ ] Rebuild PDF one final time immediately before upload
- [ ] Upload the same PDF that was last compiled locally
