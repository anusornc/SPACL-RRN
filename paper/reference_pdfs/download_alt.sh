#!/bin/bash
cd /home/admindigit/tableauxx/paper/reference_pdfs

echo "=== Trying Alternative Download Methods ==="
echo ""

# Try with different user agent and follow redirects
download_alt() {
    local num="$1"
    local name="$2"
    local url="$3"
    local output="$4"
    
    echo "[$num] $name"
    
    if [ -f "$output" ]; then
        echo "    Already exists"
        return 0
    fi
    
    # Try wget with user agent
    wget --user-agent="Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36" \
         --timeout=30 --tries=2 -q -O "$output" "$url" 2>/dev/null
    
    if [ -f "$output" ]; then
        FILETYPE=$(file -b "$output" 2>/dev/null)
        if echo "$FILETYPE" | grep -q "PDF"; then
            SIZE=$(ls -lh "$output" | awk '{print $5}')
            echo "    ✓ Downloaded ($SIZE)"
            return 0
        else
            # Check size - if small, probably HTML error
            SIZE=$(stat -c%s "$output" 2>/dev/null)
            if [ "$SIZE" -lt 10000 ]; then
                rm -f "$output"
                echo "    ✗ Blocked (HTML response)"
            else
                echo "    ? Unknown format, keeping for manual check"
            fi
        fi
    else
        echo "    ✗ Failed"
    fi
}

# Try JAIR with view page then PDF
download_alt "07" "Hypertableau" \
    "https://jair.org/index.php/jair/article/download/10672/25516" \
    "07_hypertableau_motik2009.pdf"

# Try BioPortal from PMC directly
download_alt "11" "BioPortal NAR 2009" \
    "https://www.ncbi.nlm.nih.gov/pmc/articles/PMC2808920/pdf/nar-37-W170.pdf" \
    "11_bioportal_noy2009.pdf"

# Try ResearchGate-style direct links
download_alt "15" "ELK" \
    "https://www.cs.ox.ac.uk/isg/publications/KaKrSi-ELK-ISWC12.pdf" \
    "15_elk_kazakov2012.pdf"

echo ""
ls -lh *.pdf 2>/dev/null
