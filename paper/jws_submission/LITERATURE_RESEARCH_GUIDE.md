# Literature Research Guide

**Paper**: SPACL: Speculative Parallel Tableaux with Adaptive Conflict Learning for Scalable OWL2 DL Reasoning

**Purpose**: This guide helps you find real data to replace placeholder sections in the paper.

---

## 🔍 Where to Search

### Primary Sources
1. **Google Scholar** (https://scholar.google.com)
   - Best for academic papers
   - Use quotes for exact phrases: "parallel OWL reasoning"

2. **Semantic Scholar** (https://www.semanticscholar.org)
   - Good for finding related papers
   - Shows citation counts

3. **Scopus** (https://www.scopus.com) - if you have access
   - Comprehensive coverage
   - Good for citation analysis

4. **OpenAlex** (https://openalex.org)
   - Open source academic database
   - Free API available

5. **DBLP** (https://dblp.org)
   - Computer science focused
   - Good for conference papers

---

## 📝 Sections Needing Real Data

### 1. Related Work - Parallel DL Reasoning

**What to find**: Recent papers (2015-2024) on parallel OWL/description logic reasoning

**Search queries**:
- "parallel OWL reasoning"
- "concurrent description logic"
- "parallel ontology classification"
- "GPU OWL reasoning"
- "distributed description logic"

**What to look for**:
- Performance numbers
- Comparison with sequential approaches
- Scalability claims
- Architecture descriptions

**Where to add in paper**: Section 2.1, last paragraph before "Our work differs..."

---

### 2. Related Work - Learning in DL Reasoning

**What to find**: Papers on machine learning/optimization in DL reasoning

**Search queries**:
- "machine learning OWL reasoning"
- "learning description logic"
- "optimization ontology reasoning"
- "neural-symbolic OWL"

**What to look for**:
- Learning approaches
- Performance improvements
- Integration with traditional reasoners

**Where to add in paper**: Section 2.2, paragraph starting with "\textit{<you write>: Add discussion...}"

---

### 3. Related Work - Recent Tableaux Optimizations

**What to find**: Recent optimizations (2019-2024)

**Search queries**:
- "tableaux optimization OWL 2019"
- "tableaux optimization OWL 2020"
- "tableaux optimization OWL 2021"
- "tableaux optimization OWL 2022"
- "tableaux optimization OWL 2023"
- "tableaux optimization OWL 2024"

**Where to add in paper**: Section 2.3, paragraph starting with "\textit{<you write>: Add discussion...}"

---

### 4. Related Work - Rust for Semantic Web

**What to find**: Rust-based semantic web tools

**Search queries**:
- "Rust RDF"
- "Rust ontology"
- "Rust semantic web"
- "Rust SPARQL"

**Also check**: GitHub for Rust semantic web projects
- https://github.com/search?q=rust+rdf
- https://github.com/search?q=rust+owl

**Where to add in paper**: Section 2.4, paragraph starting with "\textit{<you write>: Add discussion...}"

---

### 5. Performance Comparison - Real Benchmark Data

**This is CRITICAL** - Table 2 needs real data

**What to find**: Published benchmark results for:
- Pellet
- HermiT
- ELK
- Konclude
- FaCT++
- JFact
- Openllet

**Key sources**:

#### A. ORE Workshop Series (OWL Reasoner Evaluation)
**Search**: "ORE 2013 OWL reasoner evaluation"
**Search**: "ORE 2014 OWL reasoner evaluation"
**Search**: "ORE 2015 OWL reasoner evaluation"

These workshops contain benchmark results comparing reasoners.
- ORE 2013: http://ceur-ws.org/Vol-1015/
- ORE 2014: http://ceur-ws.org/Vol-1207/
- ORE 2015: http://ceur-ws.org/Vol-1387/

#### B. OWL2Bench Paper
**Search**: "OWL2Bench ISWC 2020"
**Paper**: "OWL2Bench: A Benchmark for OWL 2 Reasoners"
**Authors**: Sumit Bhatia et al.

This paper compares 6 reasoners on standardized benchmarks.

#### C. HermiT Paper
**Search**: "HermiT: An OWL 2 Reasoner" (2014)
**Authors**: Glimm, Horrocks, Motik, Stoilos, Wang
**Journal**: Journal of Automated Reasoning

Contains comparison with Pellet and FaCT++.

#### D. Konclude Paper
**Search**: "Konclude: System Description"
**Journal**: Journal of Web Semantics, 2014
**Authors**: Steigmiller, Liebig, Glimm

Contains performance comparisons.

#### E. Recent Comparison Studies
**Search**: "performance evaluation OWL 2 DL reasoners 2023"
**Search**: "OWL reasoner comparison 2022"

**Where to add in paper**: 
- Table 2 (comparison table)
- Section 5.4 text discussion

---

### 6. Threshold Tuning Graph

**What to create**: Graph showing performance at different threshold values

**What you need**:
- Run benchmarks with threshold = 10, 50, 100, 200, 500
- Measure performance at each setting
- Plot speedup vs threshold for different ontology sizes

**Where to add**: Section 5.3, Figure 5

---

### 7. Nogood Effectiveness Data

**What to collect**:
- Nogood hit rates during reasoning
- Cache hit ratios (local vs global)
- Branches pruned statistics
- Memory overhead

**How to get**: 
- Run SPACL with statistics collection enabled
- Extract stats from SpeculativeStats struct
- Create graphs

**Where to add**:
- Section 5.5 (nogood effectiveness)
- Figure 6

---

## 🎯 Priority Order

### HIGH PRIORITY (Critical for paper validity)
1. ✅ **Table 2 comparison data** - Find real benchmark numbers from ORE workshops
2. ✅ **ORE workshop citations** - Cite the official benchmark results
3. ✅ **Recent parallel reasoning papers** - Show awareness of field

### MEDIUM PRIORITY (Strengthens paper)
4. ✅ **Threshold tuning graph** - Show optimal parameter selection
5. ✅ **Nogood effectiveness data** - Validate learning benefits
6. ✅ **Additional recent citations** - Show current state of field

### LOW PRIORITY (Nice to have)
7. ✅ **Rust semantic web tools** - Context for implementation choice
8. ✅ **Additional implementation details** - Appendix material

---

## 📊 Specific Data Needed for Table 2

Current Table 2 has placeholders. You need to find real numbers for:

| Reasoner | 1K-class Time | Source |
|----------|---------------|--------|
| Pellet | ? ms | ORE 2013/2014 results |
| HermiT | ? ms | ORE 2013/2014 results |
| ELK | ? ms | ELK paper or ORE results |
| Konclude | ? ms | Konclude paper |
| FaCT++ | ? ms | ORE results |

**How to find**:
1. Download ORE workshop papers from CEUR-WS.org
2. Look for tables with classification times
3. Find ontologies with ~1000 classes
4. Extract the times

**Note**: Make sure to:
- Use the same ontology size (1K classes)
- Use classification time (not consistency checking)
- Use the same hardware if possible
- Cite the source

---

## 📚 Recommended Papers to Find

### Must-Have (for Table 2)
1. **ORE 2013 Results**
   - Title: "OWL Reasoner Evaluation (ORE) Workshop 2013 Results"
   - URL: http://ceur-ws.org/Vol-1015/ore2013_report.pdf

2. **HermiT 2014 Paper**
   - Title: "HermiT: An OWL 2 Reasoner"
   - Journal: Journal of Automated Reasoning
   - Year: 2014

3. **Konclude 2014 Paper**
   - Title: "Konclude: System Description"
   - Journal: Journal of Web Semantics
   - Year: 2014

### Should-Have (for related work)
4. **OWL2Bench 2020**
   - Title: "OWL2Bench: A Benchmark for OWL 2 Reasoners"
   - Conference: ISWC 2020

5. **Recent Parallel Reasoning Survey**
   - Search: "parallel ontology reasoning survey 2020"

---

## 🔧 How to Add Citations

1. Find the paper on Google Scholar
2. Click "Cite" button
3. Select "BibTeX"
4. Copy the citation
5. Paste into `references.bib`
6. Use the citation key in `main.tex` like: `\cite{key}`

---

## ⚠️ Important Notes

### About Comparison Data
- **DO NOT make up numbers** - only use published benchmark results
- If you can't find exact numbers, remove the comparison or state "no published benchmark data available"
- It's better to have no data than fake data

### About ORE Workshops
- ORE = OWL Reasoner Evaluation
- Held annually 2013-2015
- Published in CEUR-WS.org (free access)
- Contains systematic benchmarks of multiple reasoners
- This is your best source for comparison data

### About ELK
- ELK only supports OWL 2 EL (lighter profile)
- Full DL is not supported
- Note this in the paper
- Still useful for comparison if your test ontologies are in EL

---

## ✅ Checklist

Before submission, verify:
- [ ] Table 2 has real benchmark data OR is removed
- [ ] All citations in references.bib are real papers
- [ ] All "<you write>" sections are completed or removed
- [ ] ORE workshop papers are cited for benchmark claims
- [ ] No estimated/placeholder numbers in comparison tables

---

## 📧 Need Help?

If you can't find specific papers:
1. Check the paper's references section
2. Look for survey papers (they cite many sources)
3. Ask your advisor for suggestions
4. Check recent PhD theses on semantic web reasoning

---

**Last Updated**: February 2, 2026  
**Status**: Awaiting literature research completion
