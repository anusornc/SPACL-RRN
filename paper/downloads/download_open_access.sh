#!/bin/bash
cd /home/admindigit/tableauxx/paper/reference_pdfs

echo "========================================"
echo "DOWNLOADING OPEN ACCESS REFERENCES"
echo "========================================"
echo ""

SUCCESS=0
FAILED=0

# Function to download and verify PDF
download_pdf() {
    local num="$1"
    local name="$2"
    local url="$3"
    local output="$4"
    
    echo "[$num] $name"
    
    if [ -f "$output" ]; then
        SIZE=$(ls -lh "$output" 2>/dev/null | awk '{print $5}')
        echo "    ✓ Already exists ($SIZE)"
        return 0
    fi
    
    # Download with timeout
    curl -sL -m 45 "$url" -o "$output" 2>/dev/null
    
    if [ -f "$output" ]; then
        # Check if it's actually a PDF
        FILETYPE=$(file -b "$output" 2>/dev/null)
        if echo "$FILETYPE" | grep -q "PDF"; then
            SIZE=$(ls -lh "$output" | awk '{print $5}')
            echo "    ✓ Downloaded ($SIZE)"
            ((SUCCESS++))
            return 0
        else
            rm -f "$output"
            echo "    ✗ Not a valid PDF"
            ((FAILED++))
            return 1
        fi
    else
        echo "    ✗ Download failed"
        ((FAILED++))
        return 1
    fi
}

echo "=== CEUR-WS Workshop Papers (Open Access) ==="
echo ""

download_pdf "04" "ORE 2015 Report" \
    "http://ceur-ws.org/Vol-1387/paper_12.pdf" \
    "04_ore2015_parsia.pdf"

download_pdf "05" "Steigmiller 2020 Parallelised" \
    "http://ceur-ws.org/Vol-2663/paper_5.pdf" \
    "05_steigmiller2020_parallel.pdf"

download_pdf "06" "Kang 2012 Rigorous Characterization" \
    "http://ceur-ws.org/Vol-1006/paper_5.pdf" \
    "06_kang2012_rigorous.pdf"

echo ""
echo "=== JAIR (Open Access) ==="
echo ""

download_pdf "07" "Hypertableau (Motik et al., 2009)" \
    "https://jair.org/index.php/jair/article/download/10672/25516" \
    "07_hypertableau_motik2009.pdf"

download_pdf "08" "Consequence-Based Reasoning (Bate et al., 2018)" \
    "https://jair.org/index.php/jair/article/download/11257/26463" \
    "08_bate2018_consequence.pdf"

echo ""
echo "=== Zenodo (Open Access) ==="
echo ""

download_pdf "09" "Scioscia 2021 ORE Results" \
    "https://zenodo.org/record/5013799/files/paper.pdf?download=1" \
    "09_scioscia2021_ore.pdf"

download_pdf "10" "Steigmiller 2021 Evaluation Data" \
    "https://zenodo.org/record/4606565/files/paper.pdf?download=1" \
    "10_steigmiller2021_eval.pdf"

echo ""
echo "=== BioMed Central (Open Access) ==="
echo ""

download_pdf "11" "BioPortal NAR 2009" \
    "https://www.ncbi.nlm.nih.gov/pmc/articles/PMC2808920/pdf/nar-37-W170.pdf" \
    "11_bioportal_noy2009.pdf"

download_pdf "12" "BioPortal JBS 2011" \
    "https://jbiomedsem.biomedcentral.com/track/pdf/10.1186/2041-1480-2-1.pdf" \
    "12_bioportal_whetzel2011.pdf"

echo ""
echo "=== IOS Press Semantic Web (Check Open Access) ==="
echo ""

download_pdf "13" "CEDAR (Amir & Aït-Kaci, 2017)" \
    "https://content.iospress.com/download/semantic-web/sw273?id=semantic-web%2Fsw273" \
    "13_cedar_amir2017.pdf"

download_pdf "14" "ComR (Wang et al., 2019)" \
    "https://content.iospress.com/download/semantic-web/sw332?id=semantic-web%2Fsw332" \
    "14_comr_wang2019.pdf"

echo ""
echo "=== Author Websites / Preprints ==="
echo ""

download_pdf "15" "ELK Reasoner (Kazakov et al., 2012)" \
    "https://www.cs.ox.ac.uk/isg/publications/KaKrSi-ELK-ISWC12.pdf" \
    "15_elk_kazakov2012.pdf"

download_pdf "16" "Work Stealing (Arora et al., 1998)" \
    "http://supertech.csail.mit.edu/papers/steal.pdf" \
    "16_workstealing_arora1998.pdf"

download_pdf "17" "DL Handbook Chapter (Horrocks)" \
    "https://www.cs.ox.ac.uk/ian.horrocks/Publications/download/2003/p437-horrocks.pdf" \
    "17_dlhandbook_horrocks2003.pdf"

echo ""
echo "========================================"
echo "OPEN ACCESS DOWNLOAD COMPLETE"
echo "========================================"
echo ""
echo "Results:"
echo "  ✓ Successfully downloaded: $SUCCESS"
echo "  ✗ Failed: $FAILED"
echo ""
echo "Downloaded PDFs:"
ls -lh *.pdf 2>/dev/null | grep -v "^total" | awk '{print "  " $9 " (" $5 ")"}'
echo ""
echo "Total: $(ls *.pdf 2>/dev/null | wc -l) / 36"
