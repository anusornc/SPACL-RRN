#!/bin/bash
# Export manuscript.tex to DOCX using Pandoc (Docker preferred).
# Usage:
#   ./export_docx.sh [output.docx]

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

OUT_FILE="${1:-manuscript.docx}"

# Keep submission figures in sync with canonical figure outputs.
FIG_SRC_DIR="$SCRIPT_DIR/../figures"
for fig in scalability.pdf speedup.pdf throughput.pdf; do
    src="$FIG_SRC_DIR/$fig"
    if [ -f "$src" ]; then
        cp "$src" "$SCRIPT_DIR/$fig"
    fi
done

if command -v docker >/dev/null 2>&1; then
    echo "Using Docker Pandoc image..."
    docker run --rm \
        --user "$(id -u):$(id -g)" \
        -v "$SCRIPT_DIR:/workdir" \
        -w /workdir \
        pandoc/core:latest \
        manuscript.tex \
        --from=latex \
        --to=docx \
        --resource-path=. \
        -o "$OUT_FILE"
elif command -v pandoc >/dev/null 2>&1; then
    echo "Using local pandoc..."
    pandoc manuscript.tex \
        --from=latex \
        --to=docx \
        --resource-path=. \
        -o "$OUT_FILE"
else
    echo "ERROR: Neither docker nor pandoc is available." >&2
    echo "Install pandoc or use Docker, then rerun." >&2
    exit 1
fi

echo "DOCX created: $SCRIPT_DIR/$OUT_FILE"
echo "Note: review equations/algorithms in DOCX; some LaTeX math macros may remain as TeX."
