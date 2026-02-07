#!/bin/bash
# Compile manuscript.tex to PDF
# Usage: ./compile.sh [clean]

set -e

echo "========================================"
echo "  SPACL Manuscript Compiler"
echo "========================================"
echo ""

# Get script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# Check for clean flag
if [ "$1" = "clean" ]; then
    echo "Cleaning auxiliary files..."
    rm -f *.aux *.bbl *.blg *.log *.out *.toc *.lof *.lot
    rm -f *.fls *.fdb_latexmk *.synctex.gz
    rm -f manuscript.pdf
    echo "Done!"
    exit 0
fi

# Check if Docker is available
if command -v docker &> /dev/null; then
    echo "Using Docker for compilation..."
    echo ""
    
    # Run pdflatex -> bibtex -> pdflatex -> pdflatex
    echo "[1/4] Running pdflatex (first pass)..."
    docker run --rm -v "$(pwd):/workdir" texlive/texlive:latest \
        pdflatex -interaction=nonstopmode -halt-on-error manuscript.tex 2>&1 | tail -20
    
    echo ""
    echo "[2/4] Running bibtex..."
    docker run --rm -v "$(pwd):/workdir" texlive/texlive:latest \
        bibtex manuscript 2>&1
    
    echo ""
    echo "[3/4] Running pdflatex (second pass)..."
    docker run --rm -v "$(pwd):/workdir" texlive/texlive:latest \
        pdflatex -interaction=nonstopmode -halt-on-error manuscript.tex 2>&1 | tail -5
    
    echo ""
    echo "[4/4] Running pdflatex (final pass)..."
    docker run --rm -v "$(pwd):/workdir" texlive/texlive:latest \
        pdflatex -interaction=nonstopmode -halt-on-error manuscript.tex 2>&1 | tail -5

# Check if local pdflatex is available
elif command -v pdflatex &> /dev/null; then
    echo "Using local LaTeX installation..."
    echo ""
    
    echo "[1/4] Running pdflatex (first pass)..."
    pdflatex -interaction=nonstopmode -halt-on-error manuscript.tex
    
    echo ""
    echo "[2/4] Running bibtex..."
    bibtex manuscript
    
    echo ""
    echo "[3/4] Running pdflatex (second pass)..."
    pdflatex -interaction=nonstopmode -halt-on-error manuscript.tex
    
    echo ""
    echo "[4/4] Running pdflatex (final pass)..."
    pdflatex -interaction=nonstopmode -halt-on-error manuscript.tex

else
    echo "ERROR: No LaTeX compiler found!"
    echo ""
    echo "Please install one of:"
    echo "  1. Docker (recommended)"
    echo "     sudo apt-get install docker.io"
    echo ""
    echo "  2. TeX Live"
    echo "     sudo apt-get install texlive-full"
    echo ""
    exit 1
fi

echo ""
echo "========================================"
echo "  Compilation Complete!"
echo "========================================"
echo ""

# Show result
if [ -f "manuscript.pdf" ]; then
    echo "✓ Output: manuscript.pdf"
    ls -lh manuscript.pdf
    echo ""
    
    # Check for warnings
    WARNINGS=$(grep -c "Warning" manuscript.log 2>/dev/null || echo 0)
    ERRORS=$(grep -c "Error" manuscript.log 2>/dev/null || echo 0)
    
    if [ "$ERRORS" -gt 0 ] 2>/dev/null; then
        echo "⚠ Errors found: $ERRORS"
        grep "Error" manuscript.log | head -5
    fi
    
    if [ "$WARNINGS" -gt 0 ] 2>/dev/null; then
        echo "ℹ Warnings: $WARNINGS (check manuscript.log for details)"
    fi
    
    echo ""
    echo "To clean auxiliary files: ./compile.sh clean"
else
    echo "✗ Compilation failed - manuscript.pdf not found"
    exit 1
fi
