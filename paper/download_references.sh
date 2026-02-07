#!/bin/bash
# Download script for reference PDFs
# Note: Many papers require academic access. This script provides direct URLs where available.

TARGET_DIR="reference_pdfs"
mkdir -p "$TARGET_DIR"

echo "=== Reference PDF Download Script ==="
echo "Target directory: $TARGET_DIR"
echo ""
echo "NOTE: Many papers require library access or academic subscriptions."
echo "This script provides URLs for manual download."
echo ""

# Core References
echo "=== CORE REFERENCES (Open Access or Direct Links) ==="

# OWL 2 W3C (Open Access)
echo "[1/10] Downloading OWL 2 W3C Recommendation..."
curl -sL "https://www.w3.org/TR/owl2-overview/" -o "$TARGET_DIR/owl2_overview.html" 2>/dev/null || echo "      Manual: https://www.w3.org/TR/owl2-overview/"

# ELK Reasoner (Open Access from authors)
echo "[2/10] ELK paper (check authors' website)..."
echo "      https://www.cs.ox.ac.uk/isg/tools/ELK/"

# LUBM Benchmark (Open Access)
echo "[3/10] LUBM Benchmark..."
echo "      Search: https://scholar.google.com/scholar?q=Lehigh+University+Bench+LUBM+Guo+2005"

# BioPortal (Open Access)
echo "[4/10] BioPortal papers (Open Access)..."
echo "      https://www.ncbi.nlm.nih.gov/pmc/articles/PMC2808920/ (NAR 2009)"
echo "      https://jbiomedsem.biomedcentral.com/articles/10.1186/2041-1480-2-1"

# Open Source / GitHub
echo "[5/10] Openllet GitHub..."
echo "      https://github.com/GalacticOrganizer/openllet"

echo "[6/10] Crossbeam Rust..."
echo "      https://github.com/crossbeam-rs/crossbeam"

# Conference Papers (CEUR-WS are Open Access)
echo "[7/10] DL Workshop papers (CEUR-WS - Open Access)..."
echo "      DL 2001: http://ceur-ws.org/Vol-49/"
echo "      DL 2008: http://ceur-ws.org/Vol-353/"
echo "      DL 2013: http://ceur-ws.org/Vol-1014/"
echo "      DL 2017: http://ceur-ws.org/Vol-1879/"
echo "      DL 2020: http://ceur-ws.org/Vol-2663/"

echo "[8/10] ORE Workshop papers (CEUR-WS)..."
echo "      ORE 2013: http://ceur-ws.org/Vol-1015/"
echo "      ORE 2015: http://ceur-ws.org/Vol-1387/"

echo "[9/10] OWL2Bench (CEUR-WS)..."
echo "      http://ceur-ws.org/Vol-2781/"

echo "[10/10] Hypertableau (JAIR - Open Access)..."
echo "      https://jair.org/index.php/jair/article/view/10672"

echo ""
echo "=== PAPERS REQUIRING ACADEMIC ACCESS ==="
echo ""
echo "Springer (via institutional access):"
echo "  - FaCT++ (IJCAR 2006)"
echo "  - HermiT (JAR 2014)"
echo "  - Instance Store (ISWC 2003)"
echo "  - Model Caching (CADE 2020)"
echo "  - RACER (IJCAR 2001)"
echo ""
echo "Elsevier (via institutional access):"
echo "  - Pellet (J. Web Semantics 2007)"
echo "  - Konclude (J. Web Semantics 2014)"
echo "  - LUBM (Web Semantics 2005)"
echo ""
echo "IEEE (via institutional access):"
echo "  - GRASP (IEEE Trans. Computers 1999)"
echo "  - Parallel Architecture (ICPPW 2017)"
echo "  - Cichlid (IPDPS 2015)"
echo ""
echo "ACM (via institutional access):"
echo "  - Work Stealing (SPAA 1998)"
echo "  - Rust Language (SIGAda 2014)"
echo "  - SPOWL (SeBiDa 2017)"
echo ""

# Create a download list
cat > "$TARGET_DIR/download_links.txt" << 'EOF'
REFERENCE PDF DOWNLOAD LINKS
============================

=== OPEN ACCESS (Free to Download) ===

1. OWL 2 W3C Recommendation
   https://www.w3.org/TR/owl2-overview/
   https://www.w3.org/TR/owl2-primer/

2. ELK Reasoner
   https://link.springer.com/chapter/10.1007/978-3-642-35176-1_1
   Project: https://www.cs.ox.ac.uk/isg/tools/ELK/

3. DL Handbook (Preview)
   https://www.cs.ox.ac.uk/ian.horrocks/Publications/download/2003/p437-horrocks.pdf (Chapter)

4. BioPortal (Open Access)
   https://www.ncbi.nlm.nih.gov/pmc/articles/PMC2808920/
   https://jbiomedsem.biomedcentral.com/articles/10.1186/2041-1480-2-1

5. Hypertableau (Open Access - JAIR)
   https://jair.org/index.php/jair/article/view/10672

6. Description Logic Handbook Chapter (Baader & Nutt)
   https://www.cs.ox.ac.uk/ian.horrocks/Publications/download/2003/p437-horrocks.pdf

7. DL Workshop Papers (CEUR-WS)
   DL 2001: http://ceur-ws.org/Vol-49/
   DL 2008: http://ceur-ws.org/Vol-353/
   DL 2013: http://ceur-ws.org/Vol-1014/
   DL 2017: http://ceur-ws.org/Vol-1879/
   DL 2020: http://ceur-ws.org/Vol-2663/

8. ORE Workshop Papers
   ORE 2013: http://ceur-ws.org/Vol-1015/
   ORE 2015: http://ceur-ws.org/Vol-1387/

9. OWL2Bench
   http://ceur-ws.org/Vol-2781/paper7.pdf

10. OWL 2 Primer (W3C)
    https://www.w3.org/TR/owl2-primer/

=== ARXIV (Open Access) ===

11. Quan & Haarslev 2019 Framework
    https://arxiv.org/abs/1906.07749

=== ZENODO (Open Access) ===

12. Steigmiller 2021 Evaluation Data
    https://zenodo.org/record/4606565

13. Scioscia 2021 ORE Results
    https://zenodo.org/record/5013799

=== SPRINGER (Subscription Required) ===

14. FaCT++ (IJCAR 2006)
    https://doi.org/10.1007/11814771_26

15. HermiT (JAR 2014)
    https://doi.org/10.1007/s10817-014-9305-1

16. Instance Store (ISWC 2003)
    https://doi.org/10.1007/978-3-540-39718-2_3

17. Model Caching (CADE 2020)
    https://doi.org/10.1007/978-3-030-51074-9_4

18. ELK (ISWC 2012)
    https://doi.org/10.1007/978-3-642-35176-1_1

19. RACER (IJCAR 2001)
    https://doi.org/10.1007/3-540-45744-5_59

20. NACRE (POS 2020)
    https://doi.org/10.29007/DXNB

=== ELSEVIER (Subscription Required) ===

21. Pellet (Journal of Web Semantics 2007)
    https://doi.org/10.1016/j.websem.2007.03.004

22. Konclude (Journal of Web Semantics 2014)
    https://doi.org/10.1016/j.websem.2014.06.002

23. LUBM (Web Semantics 2005)
    https://doi.org/10.1016/j.websem.2005.09.001

24. Bilenchi 2021 Multiplatform
    https://doi.org/10.1016/j.websem.2021.100694

=== IEEE (Subscription Required) ===

25. GRASP (IEEE Trans. Computers 1999)
    https://doi.org/10.1109/12.769433

26. Quan 2017 Parallel Architecture
    https://doi.org/10.1109/ICPPW.2017.38

27. Cichlid (IPDPS 2015)
    https://doi.org/10.1109/IPDPS.2015.46

28. Liu 2019 Deep Learning
    https://doi.org/10.1109/ACCESS.2019.2937353

=== ACM (Subscription Required) ===

29. Work Stealing (SPAA 1998)
    https://doi.org/10.1145/277651.277678

30. Rust Language (SIGAda 2014)
    https://doi.org/10.1145/2692956.2692958

31. SPOWL (SeBiDa 2017)
    https://doi.org/10.1145/3070607.3070609

=== OTHER PUBLISHERS ===

32. Description Logic Handbook (Cambridge)
    https://www.cambridge.org/core/books/description-logic-handbook/...

33. Rust Book (No Starch Press)
    https://nostarch.com/rust

34. Amir 2017 CEDAR (IOS Press - Semantic Web)
    https://doi.org/10.3233/SW-170273

35. Faddoul 2015 Fork/Join (IJNC)
    https://doi.org/10.15803/IJNC.5.1_61

36. Wang 2019 ComR (IOS Press - Semantic Web)
    https://doi.org/10.3233/SW-180332

37. NORA 2023 (Wiley)
    https://doi.org/10.1002/spe.3135

38. Wu 2016 (Springer ISWC)
    https://doi.org/10.1007/978-3-319-48740-3_27

39. Consequence-Based Reasoning (JAIR)
    https://doi.org/10.1613/jair.1.11257

40. Eiter 2008 (Artificial Intelligence)
    https://doi.org/10.1016/j.artint.2008.04.002

=== THESES ===

41. Algahtani 2024 PhD (University of New Brunswick)
    Check UNB repository or ProQuest

42. Priya 2015 PhD (Wright State University)
    Check OhioLINK ETD or ProQuest

=== TECH REPORTS ===

43. Glimm 2014 Coupling (Ulm University)
    https://oparu.uni-ulm.de/xmlui/handle/123456789/...

=== WEB/CONFERENCE ONLY ===

44. Openllet GitHub
    https://github.com/GalacticOrganizer/openllet

45. Crossbeam
    https://github.com/crossbeam-rs/crossbeam

46. Absorption (DL Workshop)
    http://ceur-ws.org/Vol-353/

47. Blocking (DL Workshop)
    http://ceur-ws.org/Vol-49/

48. Steigmiller 2020 Parallelised
    http://ceur-ws.org/Vol-2663/

49. Kang 2012 Rigorous (JIST)
    Search: Google Scholar

50. Song 2013 Complete (DL Workshop)
    http://ceur-ws.org/Vol-1014/

51. Zhao 2017 ReAD (DL Workshop)
    http://ceur-ws.org/Vol-1879/

52. Cantone 2018 (Intelligenza Artificiale)
    https://content.iospress.com/articles/intelligenza-artificiale/ia180402

53. Kollia 2011 (ESWC)
    https://doi.org/10.1007/978-3-642-21064-8_26

54. Horrocks 2001 SHOQ(D) (IJCAI)
    https://www.ijcai.org/Proceedings/01/Papers/...

55. Parsia 2013 ORE
    http://ceur-ws.org/Vol-1015/

EOF

echo "Download links saved to: $TARGET_DIR/download_links.txt"
echo ""
echo "=== DOWNLOAD COMPLETE ==="
echo ""
echo "SUMMARY:"
echo "  - Some papers downloaded directly"
echo "  - Full list of URLs saved to: $TARGET_DIR/download_links.txt"
echo "  - Open access papers: ~20 available"
echo "  - Subscription papers: ~35 require institutional access"
echo ""
echo "To download remaining papers:"
echo "  1. Use your university library access"
echo "  2. Check ResearchGate (authors often post preprints)"
echo "  3. Contact authors directly for copies"
