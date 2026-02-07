#!/bin/bash
cd /home/admindigit/tableauxx/paper/reference_pdfs

echo "=== Trying Direct PDF Links ==="
echo ""

# Some papers have direct PDF links that work better

# 1. Hypertableau from JAIR (try direct)
echo "[1] Hypertableau (trying alternative)..."
curl -sL -A "Mozilla/5.0" "https://jair.org/index.php/jair/article/download/10672/25516/25153" -o "07_hypertableau.pdf" --max-time 30
if file "07_hypertableau.pdf" | grep -q "PDF"; then
    echo "    ✓ Success"
else
    rm -f "07_hypertableau.pdf"
    echo "    ✗ Failed"
fi

# 2. Bate 2018 from JAIR
echo "[2] Bate 2018 (trying alternative)..."
curl -sL -A "Mozilla/5.0" "https://jair.org/index.php/jair/article/download/11257/26463/26047" -o "08_bate2018.pdf" --max-time 30
if file "08_bate2018.pdf" | grep -q "PDF"; then
    echo "    ✓ Success"
else
    rm -f "08_bate2018.pdf"
    echo "    ✗ Failed"
fi

# 3. Work Stealing from MIT
echo "[3] Work Stealing MIT..."
curl -sL "http://supertech.csail.mit.edu/papers/steal.pdf" -o "16_workstealing.pdf" --max-time 30
if file "16_workstealing.pdf" | grep -q "PDF"; then
    echo "    ✓ Success"
else
    rm -f "16_workstealing.pdf"
    echo "    ✗ Failed"
fi

# 4. Try Semantic Web journal direct
echo "[4] CEDAR 2017..."
curl -sL "https://content.iospress.com/articles/semantic-web/sw273" -o "tmp.html" --max-time 20
if [ -f "tmp.html" ]; then
    # Try to extract PDF link
    PDFLINK=$(grep -o 'href="[^"]*\.pdf[^"]*"' tmp.html | head -1 | sed 's/href="//;s/"$//')
    if [ -n "$PDFLINK" ]; then
        echo "    Found PDF link: $PDFLINK"
    fi
    rm -f tmp.html
fi
echo "    (Needs manual download)"

echo ""
echo "=== Results ==="
ls -lh *.pdf 2>/dev/null | tail -10
