# SPACL-RRN Manuscript Track

This directory is a separate paper track for a hybrid neural-symbolic study.

- Baseline paper track: `paper/submission/`
- Hybrid track (this directory): `paper/submission_rrn/`

## Scope

This track evaluates an optional RRN-guided branch prioritization layer on top of SPACL.

Safety boundary:
- RRN guidance only affects branch ordering.
- Final consistency outcome remains symbolic in SPACL.
- Baseline behavior must remain unchanged when hybrid mode is disabled.

## Build

```bash
cd paper/submission_rrn
./compile.sh
```

## Export DOCX

```bash
cd paper/submission_rrn
./export_docx.sh
```

## Included files

- `manuscript.tex`
- `references.bib`
- `elsarticle.cls`
- `elsarticle-num.bst`
- `compile.sh`
- `export_docx.sh`

## Current status

This manuscript is a working split for the hybrid RRN study and will be updated in lockstep with benchmark artifacts from branch `exp/hybrid-rrn-paper`.
