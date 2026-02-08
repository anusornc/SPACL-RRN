#!/bin/bash
# Manual download helper with instructions

echo "========================================"
echo "36 REFERENCE DOWNLOAD HELPER"
echo "========================================"
echo ""
echo "This script will guide you through downloading"
echo "all 36 cited references."
echo ""

REFERENCES=(
    "1|OWL2Bench (Singh et al., 2020)|http://ceur-ws.org/Vol-2781/paper7.pdf|open"
    "2|Quan 2019 Framework|https://arxiv.org/pdf/1906.07749.pdf|open"
    "3|Song 2013 Complete|http://ceur-ws.org/Vol-1014/paper_7.pdf|open"
    "4|Hypertableau (Motik et al., 2009)|https://jair.org/index.php/jair/article/view/10672|jair"
    "5|Bate 2018 Consequence-Based|https://jair.org/index.php/jair/article/view/11257|jair"
    "6|BioPortal 2011|https://jbiomedsem.biomedcentral.com/articles/10.1186/2041-1480-2-1|biomed"
    "7|BioPortal 2009|https://www.ncbi.nlm.nih.gov/pmc/articles/PMC2808920/|pmc"
    "8|ORE 2015 Report|http://ceur-ws.org/Vol-1387/|ceur"
    "9|Steigmiller 2020|http://ceur-ws.org/Vol-2663/|ceur"
    "10|HermiT (Glimm et al., 2014)|https://link.springer.com/article/10.1007/s10817-014-9305-1|springer"
    "11|Pellet (Sirin et al., 2007)|https://www.sciencedirect.com/science/article/pii/S1570826807000055|elsevier"
    "12|Konclude (Steigmiller et al., 2014)|https://www.sciencedirect.com/science/article/pii/S1570826814000392|elsevier"
    "13|ELK (Kazakov et al., 2012)|https://link.springer.com/chapter/10.1007/978-3-642-35176-1_1|springer"
    "14|GRASP (Marques-Silva & Sakallah, 1999)|https://ieeexplore.ieee.org/document/769433|ieee"
    "15|Quan 2017 Parallel|https://ieeexplore.ieee.org/document/8106070|ieee"
    "16|Cichlid (Gu et al., 2015)|https://ieeexplore.ieee.org/document/7284319|ieee"
    "17|Liu 2019 Deep Learning|https://ieeexplore.ieee.org/document/8842639|ieee"
    "18|SPOWL (Liu & McBrien, 2017)|https://dl.acm.org/doi/10.1145/3070607.3070609|acm"
    "19|Work Stealing (Arora et al., 1998)|https://dl.acm.org/doi/10.1145/277651.277678|acm"
    "20|Bilenchi 2021 Multiplatform|https://www.sciencedirect.com/science/article/pii/S1570826821000393|elsevier"
    "21|Wu 2016 Parallel|https://link.springer.com/chapter/10.1007/978-3-319-48740-3_27|springer"
    "22|NORA (Benítez-Hidalgo et al., 2023)|https://onlinelibrary.wiley.com/doi/10.1002/spe.3135|wiley"
    "23|CEDAR (Amir & Aït-Kaci, 2017)|https://content.iospress.com/articles/semantic-web/sw273|ios"
    "24|ComR (Wang et al., 2019)|https://content.iospress.com/articles/semantic-web/sw332|ios"
    "25|OWL 2 Overview|https://www.w3.org/TR/owl2-overview/|w3c"
    "26|OWL 2 Primer|https://www.w3.org/TR/owl2-primer/|w3c"
    "27|DL Handbook|https://www.cambridge.org/core/books/description-logic-handbook/|cambridge"
    "28|Scioscia 2021 ORE|https://zenodo.org/record/5013799|zenodo"
    "29|Steigmiller 2021 Eval|https://zenodo.org/record/4606565|zenodo"
    "30|Algahtani 2024 PhD|https://unbscholar.lib.unb.ca/|unb"
    "31|Openllet GitHub|https://github.com/GalacticOrganizer/openllet|github"
    "32|Crossbeam|https://github.com/crossbeam-rs/crossbeam|github"
    "33|Glimm 2014 Coupling|https://oparu.uni-ulm.de/xmlui/handle/123456789/3211|oparu"
    "34|Faddoul 2015 Fork/Join|https://www.ijnc.org/index.php/ijnc/article/view/61|ijnc"
    "35|Kang 2012|http://ceur-ws.org/Vol-1006/|ceur"
    "36|Amir 2017 CEDAR|https://content.iospress.com/articles/semantic-web/sw273|ios"
)

download_ref() {
    local num="$1"
    local name="$2"
    local url="$3"
    local type="$4"
    local filename=$(printf "%02d_%s.pdf" "$num" "$(echo "$name" | tr ' ' '_' | tr -d '()' | cut -c1-30)")
    
    echo ""
    echo "========================================"
    echo "[$num/36] $name"
    echo "========================================"
    
    if [ -f "$filename" ]; then
        SIZE=$(ls -lh "$filename" 2>/dev/null | awk '{print $5}')
        echo "✓ Already downloaded ($SIZE)"
        return 0
    fi
    
    echo "URL: $url"
    echo ""
    
    case "$type" in
        open|ceur|jair|arxiv|zenodo|biomed|pmc|w3c|github|oparu|ijnc)
            echo "Access: 🔓 OPEN ACCESS"
            echo ""
            read -p "Download now? (y/n): " answer
            if [ "$answer" = "y" ]; then
                curl -sL -m 60 "$url" -o "$filename"
                if [ -f "$filename" ]; then
                    FILETYPE=$(file -b "$filename" | head -c 20)
                    if echo "$FILETYPE" | grep -q "PDF"; then
                        SIZE=$(ls -lh "$filename" | awk '{print $5}')
                        echo "✓ Downloaded ($SIZE)"
                    else
                        rm -f "$filename"
                        echo "✗ Not a PDF (may need manual download)"
                        echo "Please visit: $url"
                    fi
                else
                    echo "✗ Download failed"
                    echo "Please visit: $url"
                fi
            else
                echo "Skipped. Visit: $url"
            fi
            ;;
        springer|elsevier|ieee|acm|wiley|ios|cambridge|unb)
            echo "Access: 🔐 REQUIRES INSTITUTIONAL ACCESS"
            echo ""
            echo "Options:"
            echo "  1. Use university VPN/proxy"
            echo "  2. Search ResearchGate for preprint"
            echo "  3. Contact author directly"
            echo ""
            echo "URL: $url"
            read -p "Press Enter to continue..."
            ;;
    esac
}

# Main loop
for ref in "${REFERENCES[@]}"; do
    IFS='|' read -r num name url type <<< "$ref"
    download_ref "$num" "$name" "$url" "$type"
done

echo ""
echo "========================================"
echo "DOWNLOAD COMPLETE"
echo "========================================"
echo ""
ls -lh *.pdf 2>/dev/null | grep -v "^total"
echo ""
echo "Total PDFs: $(ls *.pdf 2>/dev/null | wc -l) / 36"
