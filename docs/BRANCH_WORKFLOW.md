# Branch Workflow (Paper-Safe)

This project currently maintains two paper tracks and must avoid cross-contamination.

## Quick memory map

- `main`: stable line for the submitted primary paper (`paper/submission`)
- `paper1/submitted-freeze`: frozen snapshot of what was submitted (do not edit)
- `paper1/reviewer-r1`: branch for reviewer-requested revisions of the primary paper
- `exp/hybrid-rrn-paper`: branch for hybrid RRN experiments and `paper/submission_rrn`

## Daily safety check (run first)

```bash
git branch --show-current
git status --short --branch
```

If branch is wrong, switch before editing anything.

## Standard operations

### 1) Work on primary paper reviewer changes

```bash
git switch paper1/reviewer-r1
```

Edit only primary-paper-relevant files (typically under `paper/submission/` plus validated support docs/code).

### 2) Work on hybrid RRN paper/experiments

```bash
git switch exp/hybrid-rrn-paper
```

Edit hybrid files (`paper/submission_rrn/`, hybrid policy code, hybrid experiment docs/artifacts).

### 3) Keep submitted snapshot untouched

Never commit directly to:

```text
paper1/submitted-freeze
```

It exists as a permanent reference point.

## Cross-branch bugfix policy

If a fix is needed in both tracks:

1. implement and commit once in a dedicated fix branch
2. `cherry-pick` the same commit into each target branch
3. avoid broad merges between `exp/hybrid-rrn-paper` and `main`

Example:

```bash
git switch -c fix/shared-parser-bug main
# edit + commit
git switch paper1/reviewer-r1
git cherry-pick <commit_sha>
git switch exp/hybrid-rrn-paper
git cherry-pick <commit_sha>
```

## Push policy (recommended)

- Push hybrid track to private repo branch:
  - `origin exp/hybrid-rrn-paper`
- Push primary reviewer updates from:
  - `origin paper1/reviewer-r1`
- Only update `main` after explicit decision for release/integration.

## Paper mapping

- Primary paper:
  - `paper/submission/manuscript.tex`
- Hybrid companion paper:
  - `paper/submission_rrn/manuscript.tex`

Treat them as separate products with separate evidence boundaries.
