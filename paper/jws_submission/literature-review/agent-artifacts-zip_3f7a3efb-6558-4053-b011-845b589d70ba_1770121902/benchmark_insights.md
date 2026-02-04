## TL;DR

ORE competitions report runtimes and correctness over classification, consistency and realization tasks across many reasoners; benchmark suites (ORE, OWL2Bench, evOWLuator) and datasets (BioPortal, SNOMED, ChEMBL, Reactome, UniProt) provide raw runtimes and energy traces. Detailed per-reasoner numeric summaries are available in those resources but a consolidated size-vs-time table is not present in the supplied corpus; some papers report relative speed and scaling behaviour for specific engines. 

----

## ORE workshop results

The ORE workshops and competitions compared many OWL reasoners on standard tasks and produced corpora, execution frameworks and results that are the primary empirical source for cross-reasoner comparisons. The competitions ran classification, consistency and realization tasks over OWL 2 DL and EL profiles and reported runtimes, correctness/robustness and task-by-task success rates for participating systems [1] [2]. 

- **Competition scope** 14 reasoner submissions participated in the competitions and offline runs, and the framework supported multiple tracks (OWL 2 DL and EL) and tasks (classification, consistency, realization) [1] [2].  
- **Reported metrics** ORE reported per-task **execution time**, **success/failure** (timeouts/errors), and comparative rankings per track; later work using the ORE outputs also measured **energy consumption** and produced CSV traces for runtime/energy analysis [2] [3].  
- **Reasoners compared** The ORE corpora and associated evaluation datasets include or were used to evaluate engines such as FaCT++, HermiT, Pellet, JFact, Konclude, TrOWL, Mini‑ME and others; a recent evOWLuator dataset lists FaCT++, HermiT, JFact, Konclude, Mini‑ME, Mini‑ME Swift, Pellet and TrOWL among evaluated systems [4] [2].

----

## Reasoner performance summary

This section summarizes reported qualitative and publication-backed performance characteristics for the named reasoners; where concrete numeric timings are not present in the supplied corpus the statement notes insufficient evidence.

Pellet  
- Opening paragraph: Pellet appears repeatedly in ORE evaluations and independent comparisons; publications treat it as a widely used tableau-based reasoner whose performance is often outperformed by newer optimisations on large or complex ontologies.  
- Details: ORE and follow-up evaluation datasets include Pellet (e.g., Pellet 2.3.1 in evOWLuator runs) as a competitor across tasks and ontologies [4] [2]. Specific per-size classification times for Pellet are not consolidated in the supplied corpus and thus **insufficient evidence** exists here to state typical numeric classification times for varying ontology sizes.

HermiT  
- Opening paragraph: HermiT is presented as a hypertableau-based reasoner that targets reduction of nondeterminism and model size and is often reported as fast on complex ontologies; ORE and comparative studies include it among the main DL reasoners.  
- Details: The HermiT system papers report that hypertableau optimisations yield substantially improved classification performance on many difficult ontologies and show HermiT beating FaCT++ and Pellet on a number of benchmarks in the system evaluation [5]. ORE datasets also include HermiT runs and aggregate success/time results for classification tasks [1] [2]. The supplied corpus does not provide a single cleaned numeric table of classification times by ontology size for HermiT, so **insufficient evidence** is available here to give a universal size-to-time mapping.

Konclude  
- Opening paragraph: Konclude is described as combining tableau and saturation techniques to scale to large ontologies (e.g., medical terminologies) and is explicitly evaluated on large real-world ontologies in the literature.  
- Details: Publications describing Konclude report improved reasoning performance through optimisations and tight coupling of tableau and saturation procedures and explicitly evaluate it on large ontologies such as SNOMED and other corpora, showing significant improvements in reasoning performance over pure tableau-based engines for many ontologies [6] [7]. Konclude-related evaluation archives (parallelised ABox reasoning dataset) provide raw results and ontologies used in published experiments [8]. The supplied corpus does not include a single canonical numeric table mapping ontology class counts to classification times for Konclude, therefore **insufficient evidence** exists here for concrete per-size timings.

ELK  
- Opening paragraph: ELK is a consequence/saturation‑based reasoner for the OWL EL profile; it is repeatedly cited as extremely fast for EL-profile classification but limited to the EL expressivity profile.  
- Details: ELK is commonly included in benchmark runs for EL-profile tasks and is used in OWL2Bench demonstrations; ELK’s strength is very low classification times on EL ontologies but it cannot handle OWL 2 DL features outside EL, which is a documented limitation [9]. The supplied corpus does not provide a single numeric table of classification times across ontology sizes for ELK in general; for specific EL ontologies ORE/evOWLuator resources hold the raw timings [4] but a consolidated numeric summary is **insufficient evidence** here.

FaCT++  
- Opening paragraph: FaCT++ is a widely used tableau-based OWL 2 reasoner that features in many comparative studies; it is often competitive on a range of ontologies but can be outperformed by specialised or newer optimised engines on some datasets.  
- Details: Comparative work that explicitly studies FaCT++ alongside HermiT, Pellet and TrOWL shows variation in ranking by ontology and notes that FaCT++ is competitive on many ontologies in practice [10] [5]. Specific classification-time scalings by class-count are not consolidated in the supplied corpus, so **insufficient evidence** exists here to provide general numeric scalings.

JFact  
- Opening paragraph: JFact (a Java port of FaCT++) appears in ORE and evOWLuator runs as an evaluated engine; reports treat it similarly to FaCT++ in qualitative comparisons.  
- Details: The evOWLuator dataset and ORE resources list JFact among evaluated reasoners for classification and other tasks [4] [2]. The supplied corpus does not provide general numeric scaling figures for JFact across ontology sizes; therefore **insufficient evidence** exists to provide precise classification-time numbers here.

----

## Benchmark frameworks and datasets

Benchmarking frameworks and datasets exist to support reproducible evaluation and allow retrieval of raw timings and energy traces; they differ in scope (OWL DL vs profiles, ABox vs TBox, synthetic vs real ontologies). OWL2Bench and ORE are central resources for reasoner evaluation.

- **OWL2Bench**  a customizable benchmark aimed at OWL 2 reasoners that targets construct coverage, size scaling and query evaluation and demonstrates the benchmark by running multiple engines including ELK [9].  
- **ORE resources and corpora**  ORE provides the competition framework, corpora of real-world ontologies and the descriptions needed to re-run competitions; the 2015 resources paper publishes corpora and framework details for reuse [11].  
- **evOWLuator and energy-aware framework**  a multiplatform, energy-aware benchmarking framework and associated CSV datasets provide correctness, runtime and energy measurements for reasoners including FaCT++, HermiT, JFact, Konclude, Mini‑ME, Pellet and TrOWL across ORE and BioPortal ontologies [4] [3].  
- **Large real-world ontologies**  benchmark datasets referenced or used in evaluations include SNOMED, ChEMBL, Reactome and UniProt in Konclude/ABox datasets and public ORE/BioPortal corpora used across competitions and follow-up experiments [8] [4].  
- **Measured metrics** Typical measured metrics across these frameworks are **classification time**, **consistency checking time**, **realisation/realization time**, **success/failure counts (timeouts/errors)**, **energy consumption traces**, and for some ABox studies **query throughput/response time** [2] [3] [8].

----

## Scalability challenges and typical reporting gaps

Published evaluations highlight where reasoners struggle and point to open gaps in consolidated numeric reporting across ontology sizes; the supplied corpus contains relative results and raw datasets but not a single merged size-to-time table covering all engines.

- **Observed scalability limits** ORE runs and follow-up studies document many cases where reasoners time out or fail on challenging, user-submitted ontologies, motivating per-engine behaviour characterisation and automated selection/meta‑reasoning approaches [1] [2] [12].  
- **Profile-related scaling** Saturation/consequence‑based EL reasoners scale to very large EL ontologies (e.g., SNOMED) efficiently, whereas tableau-based OWL 2 DL reasoners may struggle on very large, highly cyclic or highly expressive ontologies unless specialised optimisations are applied [6] [7].  
- **Where breakdowns occur** Papers point to difficulty for expressive DL (OWL 2 DL) classification at large scale; Konclude and hybrid approaches were introduced specifically to handle very large ontologies and show significant improvements on those corpora in their evaluations [6] [7]. ORE reports also show failures/timeouts on some realistic ontologies for several reasoners [1] [2].  
- **Typical numeric reporting gap** The supplied corpus provides raw CSVs and per-experiment timings (e.g., evOWLuator and Konclude datasets) but not a single consolidated mapping such as “100s classes -> X sec, 1000s -> Y sec, 10000s -> Z sec” for each reasoner; producing such a table requires reprocessing the published experiment CSVs and/or the prediction models described in the literature [4] [3] [12]. Because that consolidated numeric mapping is not present in the provided papers, the statement of per-size typical classification times is **insufficient evidence** in the current corpus.

- **Where to obtain concrete numbers** For concrete per-ontology runtimes and energy traces consult: the ORE resources and competition result archives [11] the evOWLuator CSV datasets (DOIs provided in the dataset records) [4] and the Konclude evaluation archives that include raw results and ontologies used for the large-ontology experiments [8] [3]. These contain the per-ontology runtimes from which size-to-time mappings can be derived.

----