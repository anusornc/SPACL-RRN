# Hybrid Neural-Symbolic Branch Prioritization for Tableau-Based OWL Ontology Reasoning: A Comprehensive Literature Review

## Executive Summary

This comprehensive literature review examines the emerging field of hybrid neural-symbolic approaches to branch prioritization in tableau-based reasoning for OWL ontologies and description logics (ALC/SHOIQ). The review synthesizes research from 2015-2026, covering learned heuristics, branch-ordering policies, speculative scheduling, work-stealing techniques, and conflict/nogood learning mechanisms. The analysis reveals that while traditional tableau-based reasoners have achieved significant maturity, the integration of machine learning—particularly deep neural networks and reinforcement learning—represents a promising frontier for optimizing non-deterministic decision-making in reasoning tasks. Key findings indicate that learned heuristics can achieve speedups of up to two orders of magnitude compared to traditional approaches, though challenges remain in transferability, scalability to large ontologies, and integration with parallel execution strategies. The review identifies critical gaps in benchmark standardization, limited exploration of neural-symbolic integration for description logic-specific reasoning, and the need for more sophisticated approaches to conflict-driven learning adapted from SAT solving. Future directions include developing lightweight neural architectures suitable for real-time branch selection, creating unified benchmarks for evaluating learned reasoning strategies, and exploring neuro-symbolic methods that preserve logical soundness while leveraging statistical learning.

## Table of Contents

1. [Introduction and Background on Tableau-Based Reasoning for Description Logics](#1-introduction-and-background-on-tableau-based-reasoning-for-description-logics)
   - 1.1 [Description Logics and OWL Ontologies](#11-description-logics-and-owl-ontologies)
   - 1.2 [Tableau-Based Reasoning Algorithms](#12-tableau-based-reasoning-algorithms)
   - 1.3 [Computational Challenges and Non-Determinism](#13-computational-challenges-and-non-determinism)
   - 1.4 [Scope and Organization of This Review](#14-scope-and-organization-of-this-review)

2. [Hybrid Neural-Symbolic Approaches and Learned Heuristics](#2-hybrid-neural-symbolic-approaches-and-learned-heuristics)
   - 2.1 [Foundations of Neural-Symbolic Integration](#21-foundations-of-neural-symbolic-integration)
   - 2.2 [Machine Learning for Heuristic Selection in OWL Reasoning](#22-machine-learning-for-heuristic-selection-in-owl-reasoning)
   - 2.3 [Deep Neural Networks for Ontology Reasoning](#23-deep-neural-networks-for-ontology-reasoning)
   - 2.4 [Neuro-Symbolic RDF and Description Logic Reasoners](#24-neuro-symbolic-rdf-and-description-logic-reasoners)

3. [Branch Prioritization and Ordering Strategies](#3-branch-prioritization-and-ordering-strategies)
   - 3.1 [Traditional Heuristics for Branch Selection](#31-traditional-heuristics-for-branch-selection)
   - 3.2 [Learning-Based Branch Ordering Policies](#32-learning-based-branch-ordering-policies)
   - 3.3 [Reinforcement Learning for Proof Guidance](#33-reinforcement-learning-for-proof-guidance)
   - 3.4 [Connection Tableaux and Machine Learning Guidance](#34-connection-tableaux-and-machine-learning-guidance)

4. [Speculative Scheduling and Work-Stealing Techniques](#4-speculative-scheduling-and-work-stealing-techniques)
   - 4.1 [Parallel Computing Architectures for OWL Reasoning](#41-parallel-computing-architectures-for-owl-reasoning)
   - 4.2 [Work-Stealing and Load Balancing](#42-work-stealing-and-load-balancing)
   - 4.3 [Parallelized ABox Reasoning](#43-parallelized-abox-reasoning)
   - 4.4 [Challenges in Parallel Tableau Reasoning](#44-challenges-in-parallel-tableau-reasoning)

5. [Conflict and Nogood Learning Mechanisms](#5-conflict-and-nogood-learning-mechanisms)
   - 5.1 [Conflict-Driven Learning in AI Planning](#51-conflict-driven-learning-in-ai-planning)
   - 5.2 [Conflict-Driven Constraint Answer Set Solving](#52-conflict-driven-constraint-answer-set-solving)
   - 5.3 [Conflict Generalisation in Answer Set Programming](#53-conflict-generalisation-in-answer-set-programming)
   - 5.4 [Adaptation to Description Logic Reasoning](#54-adaptation-to-description-logic-reasoning)

6. [Benchmarks and Experimental Evaluation](#6-benchmarks-and-experimental-evaluation)
   - 6.1 [Standard Ontology Benchmarks](#61-standard-ontology-benchmarks)
   - 6.2 [Theorem Proving and Automated Reasoning Datasets](#62-theorem-proving-and-automated-reasoning-datasets)
   - 6.3 [Performance Metrics and Evaluation Criteria](#63-performance-metrics-and-evaluation-criteria)
   - 6.4 [Comparative Analysis of Approaches](#64-comparative-analysis-of-approaches)

7. [Future Directions and Open Challenges](#7-future-directions-and-open-challenges)
   - 7.1 [Transferability and Generalization](#71-transferability-and-generalization)
   - 7.2 [Integration of Neural and Symbolic Components](#72-integration-of-neural-and-symbolic-components)
   - 7.3 [Scalability to Large-Scale Ontologies](#73-scalability-to-large-scale-ontologies)
   - 7.4 [Formal Verification and Correctness Guarantees](#74-formal-verification-and-correctness-guarantees)

8. [Conclusion](#8-conclusion)

[References](#references)

---

## 1. Introduction and Background on Tableau-Based Reasoning for Description Logics

### 1.1 Description Logics and OWL Ontologies

Description Logics (DLs) constitute a family of formal knowledge representation languages that provide the logical foundation for the Web Ontology Language (OWL), a W3C standard for representing structured knowledge on the Semantic Web. DLs enable the precise specification of concepts, roles, and individuals, supporting automated reasoning services such as consistency checking, classification, and query answering. The expressiveness of DL languages varies considerably, ranging from lightweight fragments like EL++ to highly expressive logics such as ALC (Attributive Language with Complements) and SHOIQ (which extends ALC with transitive roles, role hierarchies, inverse roles, qualified number restrictions, and nominals).

OWL ontologies serve as knowledge bases for diverse application domains, including biomedicine, e-commerce, and enterprise information systems. The reasoning capabilities provided by OWL reasoners enable the inference of implicit knowledge from explicitly stated facts and axioms, supporting tasks such as ontology validation, semantic query answering, and knowledge discovery. However, the computational complexity of reasoning in expressive DLs presents significant challenges, with satisfiability and subsumption problems being EXPTIME-complete for ALC and NEXPTIME-complete for SHOIQ (Zese et al., 2018).

### 1.2 Tableau-Based Reasoning Algorithms

Tableau algorithms represent the dominant approach to implementing practical reasoners for expressive description logics. These algorithms construct a model (or demonstrate the impossibility of such construction) by systematically applying expansion rules to an initial knowledge base representation. The tableau method operates by attempting to build a tree-like structure representing possible models, applying non-deterministic expansion rules at each step, and checking for contradictions (clashes) that indicate unsatisfiability.

The tableau calculus for ALC and its extensions involves several types of rules: deterministic rules (such as conjunction and universal restriction expansion) and non-deterministic rules (such as disjunction and existential restriction expansion). The non-deterministic nature of certain rules introduces branching points in the search space, where the reasoner must choose among multiple alternatives. The order in which these alternatives are explored can dramatically impact reasoning performance, as poor choices may lead to extensive backtracking or exploration of unproductive search branches (Zese et al., 2018).

Tableau reasoning for description logics has been extended to handle probabilistic knowledge, combining classical tableau methods with probability theory to support reasoning under uncertainty. These extensions maintain the core tableau structure while incorporating mechanisms for tracking and propagating probability values through the reasoning process (Zese et al., 2018).

### 1.3 Computational Challenges and Non-Determinism

The performance of tableau-based reasoners is critically affected by non-deterministic decision points that arise from multiple sources. First, disjunctions in ontologies create branching points where the reasoner must select which disjunct to explore first. Second, the order in which tableau expansion rules are applied can significantly influence the efficiency of the reasoning process. Third, when multiple individuals or concepts require expansion, the selection order impacts both the speed of finding solutions and the likelihood of early clash detection.

Traditional approaches to managing non-determinism rely on hand-crafted heuristics based on syntactic features of the ontology or simple statistical measures. However, these heuristics often operate with insufficient information about the global structure of the reasoning problem and may perform poorly on ontologies with characteristics different from those for which they were designed. The challenge of optimizing heuristic decision-making in the presence of complex, domain-specific ontology structures motivates the exploration of machine learning approaches that can learn effective strategies from experience (Mehri-Dehnavi, 2019).

### 1.4 Scope and Organization of This Review

This review examines the emerging intersection of neural-symbolic methods and tableau-based reasoning for description logics, with particular emphasis on branch prioritization, learned heuristics, and advanced execution strategies. We synthesize research from peer-reviewed publications spanning 2015-2026, covering machine learning approaches to heuristic optimization, deep neural network integration with symbolic reasoning, reinforcement learning for proof guidance, parallel execution architectures, and conflict-driven learning mechanisms adapted from SAT solving and answer set programming.

The review is organized into seven main sections following this introduction. Section 2 explores hybrid neural-symbolic approaches and learned heuristics, examining how machine learning techniques can be integrated with symbolic reasoning systems. Section 3 focuses specifically on branch prioritization and ordering strategies, including learning-to-rank methods and reinforcement learning approaches. Section 4 addresses speculative scheduling and work-stealing techniques for parallel reasoning. Section 5 examines conflict and nogood learning mechanisms. Section 6 analyzes benchmarks and experimental evaluation methodologies. Section 7 identifies future research directions and open challenges. We conclude with a synthesis of key findings and recommendations for advancing the field.

## 2. Hybrid Neural-Symbolic Approaches and Learned Heuristics

### 2.1 Foundations of Neural-Symbolic Integration

The integration of neural and symbolic approaches represents a fundamental paradigm shift in artificial intelligence, combining the learning capabilities and pattern recognition strengths of neural networks with the interpretability, compositionality, and logical rigor of symbolic systems. Neural-symbolic integration has emerged as a critical research direction for addressing limitations inherent in purely neural or purely symbolic approaches, particularly for tasks requiring both learning from data and reasoning with structured knowledge.

Recent surveys of learning-guided automated reasoning document the growing sophistication of neural-symbolic methods across multiple reasoning paradigms, including theorem proving, satisfiability solving, and ontology reasoning. These approaches typically employ neural networks to learn heuristics, guide search, or predict proof steps, while maintaining symbolic reasoning engines to ensure logical soundness and provide interpretable results (Blaauwbroek et al., 2024). The challenge lies in designing architectures that effectively bridge the gap between continuous neural representations and discrete symbolic structures while preserving the strengths of both paradigms.

The state-of-the-art in neuro-symbolic RDF and description logic reasoning reveals several architectural patterns for integration. These include: (1) neural networks that learn embeddings of ontological concepts and use these embeddings to guide symbolic reasoning, (2) hybrid systems where neural components predict heuristic values or branch priorities that are consumed by symbolic reasoners, and (3) end-to-end differentiable systems that approximate logical operations with continuous functions (Singh et al., 2023). Each approach involves different trade-offs between learning flexibility, reasoning soundness, and computational efficiency.

### 2.2 Machine Learning for Heuristic Selection in OWL Reasoning

A pioneering approach to applying machine learning for optimizing heuristic decision-making in OWL reasoners demonstrates the potential for significant performance improvements through learned strategies. Mehri-Dehnavi (2019) developed multiple machine learning-based approaches to address non-determinism in tableau-based reasoning, focusing on two primary sources of decision-making challenges: disjunction handling and tableau rule ordering.

The first approach employs a logistic regression classifier to select appropriate branching heuristics for propositional description logic reasoning. This method learns to predict which branching strategy will be most effective for a given ontology based on features extracted from the ontology structure. The second, more sophisticated approach uses Support Vector Machine (SVM) classifiers to select expansion-ordering heuristics for standard description logic reasoning, determining the order in which branches in the search tree should be explored. A third SVM-based approach optimizes the order in which tableau rules are applied during the reasoning process.

Experimental results demonstrate that these machine learning-based approaches achieve speedups of up to two orders of magnitude compared to non-ML reasoners. The ML-based reasoner significantly outperforms traditional heuristics across diverse ontologies, suggesting that learned strategies can capture complex patterns in ontology structure that are difficult to encode in hand-crafted heuristics. However, the approach relies on classical machine learning methods (logistic regression and SVMs) rather than deep neural networks, and does not incorporate neural-symbolic integration in the sense of jointly learning representations and reasoning strategies (Mehri-Dehnavi, 2019).

### 2.3 Deep Neural Networks for Ontology Reasoning

The application of deep neural networks to ontology reasoning represents a more recent and ambitious direction in neural-symbolic integration. Hohenecker et al. (2018, 2020) developed an approach that uses deep neural networks to perform approximate reasoning over OWL ontologies, learning to predict subsumption relationships and other reasoning tasks directly from ontology structure.

This approach employs neural networks to learn distributed representations (embeddings) of concepts in an ontology, capturing semantic relationships in a continuous vector space. The network is trained on examples of valid and invalid subsumption relationships, learning to predict whether one concept subsumes another based on the learned embeddings. The key innovation lies in the network architecture, which is designed to respect the compositional structure of description logic concepts and to generalize across different ontologies.

The deep learning approach offers several potential advantages over traditional symbolic reasoning: it can handle incomplete or noisy ontologies more gracefully, it scales more favorably to very large ontologies, and it can leverage transfer learning to improve performance on new ontologies with limited training data. However, the approach also faces significant challenges, including the difficulty of ensuring logical soundness (the network may predict invalid subsumptions), limited interpretability of learned representations, and the need for substantial training data. The tension between approximate neural reasoning and exact symbolic reasoning remains a central challenge in this line of research (Hohenecker et al., 2018, 2020).

### 2.4 Neuro-Symbolic RDF and Description Logic Reasoners

Recent comprehensive reviews of neuro-symbolic approaches to RDF and description logic reasoning provide valuable perspective on the current state of the field and identify key challenges for future research. Singh et al. (2023) survey the landscape of neuro-symbolic reasoners, categorizing approaches based on their integration strategy, the types of reasoning tasks they support, and their handling of the trade-off between reasoning soundness and learning flexibility.

The survey identifies several architectural patterns that have emerged in neuro-symbolic reasoning systems. Embedding-based approaches learn vector representations of entities and relations, using these embeddings to perform approximate reasoning tasks such as link prediction and query answering. Hybrid systems maintain separate neural and symbolic components, using neural networks to guide or accelerate symbolic reasoning without replacing it entirely. Differentiable logic approaches attempt to make logical operations differentiable, enabling end-to-end training of reasoning systems using gradient-based optimization.

A critical challenge identified in the survey is the difficulty of scaling neuro-symbolic approaches to expressive description logics like SHOIQ while maintaining reasoning soundness. Most existing work focuses on lightweight logics or approximate reasoning, with limited exploration of how neural-symbolic methods can be integrated with tableau-based reasoning for expressive DLs. The survey calls for more research on hybrid architectures that leverage neural networks for heuristic optimization and branch prioritization while preserving the correctness guarantees of symbolic tableau algorithms (Singh et al., 2023).

Recent work on neuro-symbolic approaches to symbol grounding for ALC ontologies demonstrates progress toward addressing these challenges. Wu et al. (2025) developed methods for grounding symbolic concepts in perceptual data using neural networks while maintaining compatibility with ALC reasoning. This work represents a step toward more tightly integrated neuro-symbolic systems that can reason about both symbolic knowledge and perceptual information. Similarly, work on reason-able embeddings explores learning concept embeddings with a transferable neural reasoner, aiming to create representations that support both learning and reasoning tasks (Potoniec, 2023).

Neuro-symbolic query optimization in knowledge graphs represents another important application area where neural and symbolic methods are being integrated. Acosta et al. (2024, 2025) developed approaches that use neural networks to optimize query execution plans for SPARQL queries over knowledge graphs, learning to predict query execution costs and select efficient join orders. While focused on query optimization rather than tableau reasoning, this work demonstrates the potential for neural methods to guide complex decision-making in symbolic systems.

## 3. Branch Prioritization and Ordering Strategies

### 3.1 Traditional Heuristics for Branch Selection

Traditional tableau-based reasoners employ a variety of hand-crafted heuristics for branch selection and ordering. These heuristics typically rely on syntactic features of concepts and axioms, such as the number of disjuncts, the depth of concept nesting, or the frequency of concept occurrences in the ontology. Common strategies include depth-first search with backtracking, breadth-first exploration, and various forms of best-first search guided by heuristic evaluation functions.

The effectiveness of traditional heuristics varies significantly across different ontologies and reasoning tasks. Heuristics that perform well on ontologies from one domain may perform poorly on ontologies with different structural characteristics. This variability motivates the development of adaptive or learned heuristics that can tailor their behavior to the specific characteristics of the reasoning problem at hand. However, the design space for branch ordering heuristics is vast, and systematic exploration of this space using traditional methods is challenging (Mehri-Dehnavi, 2019).

### 3.2 Learning-Based Branch Ordering Policies

Machine learning approaches to branch ordering aim to learn effective prioritization strategies from experience with diverse ontologies. The core idea is to train a model that predicts, for each branching point in the tableau expansion, which branch is most likely to lead quickly to a solution (or to a clash that enables pruning of the search space). This prediction can be based on features extracted from the current state of the tableau, the structure of the concepts being expanded, and global properties of the ontology.

The SVM-based approach to expansion-ordering heuristic selection developed by Mehri-Dehnavi (2019) exemplifies this strategy. The system extracts features from the ontology and the current reasoning state, then uses a trained SVM classifier to select among a set of predefined heuristics. This meta-learning approach allows the system to adapt its strategy to different ontologies without requiring manual tuning of heuristic parameters. The significant speedups achieved by this approach (up to two orders of magnitude) demonstrate the potential value of learned branch ordering policies.

However, this approach has limitations. It selects among a fixed set of predefined heuristics rather than learning novel ordering strategies. The features used for classification are hand-crafted rather than learned, potentially missing important patterns in the data. The approach does not leverage deep learning or reinforcement learning, which might enable more sophisticated strategy learning. These limitations suggest opportunities for future research on more advanced learning-based branch ordering methods (Mehri-Dehnavi, 2019).

### 3.3 Reinforcement Learning for Proof Guidance

Reinforcement learning (RL) has emerged as a powerful approach for learning proof guidance strategies in automated theorem proving, with potential applications to tableau-based description logic reasoning. RL formulates the proof search problem as a sequential decision-making task, where an agent learns to select proof steps (or branches to explore) based on the current proof state, receiving rewards for successful proofs or penalties for unproductive exploration.

Crouse et al. (2019) developed a deep reinforcement learning approach to learning transferable proof guidance strategies for automated theorem proving. The approach uses a neural network to represent the policy, which maps proof states to distributions over possible actions (proof steps). The network is trained using policy gradient methods, learning from experience with a large corpus of theorem proving problems. A key innovation is the use of graph neural networks to represent proof states, enabling the learned policy to generalize across problems with different structures.

The transferability of learned proof guidance strategies is a critical consideration. A strategy learned on one set of problems should ideally transfer to new problems, even those from different domains or with different structural characteristics. The deep RL approach shows promising results on transfer learning, with policies trained on one theorem proving domain achieving reasonable performance on other domains. However, significant performance degradation is often observed when transferring to substantially different problem types, indicating that further research is needed on learning more robust and generalizable strategies (Crouse et al., 2019).

McKeown et al. (2023) explored reinforcement learning for guiding the E theorem prover, a high-performance automated theorem prover for first-order logic. The approach uses RL to learn clause selection strategies, determining which clauses to process next during proof search. The learned strategies achieve competitive performance with hand-crafted heuristics on several benchmark datasets, demonstrating the viability of RL for guiding complex symbolic reasoning systems. The work also identifies challenges in applying RL to theorem proving, including the large action spaces, sparse rewards, and difficulty of credit assignment in long proof sequences.

### 3.4 Connection Tableaux and Machine Learning Guidance

Connection tableaux represent an alternative formulation of tableau-based reasoning that is particularly amenable to machine learning guidance. Unlike traditional tableaux, which build tree structures representing possible models, connection tableaux focus on finding connections between complementary literals, constructing proofs by systematically closing all branches through such connections.

Färber et al. (2021) developed machine learning guidance for connection tableaux, using neural networks to predict which connections are most likely to lead to successful proofs. The approach extracts features from the current proof state, including information about available clauses, existing connections, and the structure of the proof tree. These features are fed to a neural network that predicts a score for each possible connection, and the prover uses these scores to prioritize its search.

Experimental evaluation on theorem proving benchmarks demonstrates that machine learning guidance can significantly improve the performance of connection tableau provers. The learned guidance strategies enable the prover to find proofs more quickly and to solve problems that timeout with traditional heuristics. The approach also shows some degree of transfer learning, with models trained on one problem domain achieving reasonable performance on other domains. However, the effectiveness of transfer varies considerably depending on the similarity between the source and target domains (Färber et al., 2021).

The connection tableau approach offers several advantages for machine learning integration. The proof search space is more structured than in traditional tableaux, making it easier to define meaningful features and learning objectives. The connection-based formulation also aligns well with graph neural network architectures, which can naturally represent the relationships between clauses and connections. These properties suggest that connection tableaux may be a promising framework for developing neural-symbolic reasoning systems that combine learned heuristics with sound logical inference.

## 4. Speculative Scheduling and Work-Stealing Techniques

### 4.1 Parallel Computing Architectures for OWL Reasoning

The computational demands of reasoning over large, expressive ontologies have motivated significant research on parallel computing architectures for OWL reasoning. Parallelization offers the potential to dramatically reduce reasoning times by exploiting multiple processors or cores to explore different parts of the search space concurrently. However, effective parallelization of tableau-based reasoning presents substantial challenges due to the irregular structure of the search space, the need for synchronization when sharing learned information, and the difficulty of load balancing across processors.

Quan et al. (2019) developed a novel thread-level parallel architecture for ontology classification that is specifically designed for shared-memory SMP servers. The architecture avoids traditional locking techniques, instead using atomic data structures to enable lock-free concurrent access to shared reasoning state. This design reduces synchronization overhead and enables more efficient parallel execution. The system implements a work-stealing scheduler that dynamically redistributes reasoning tasks among threads to maintain load balance.

The parallel architecture achieves significant speedups on ontology classification tasks, with performance scaling well as the number of threads increases. The lock-free design proves particularly effective for avoiding contention bottlenecks that can limit the scalability of parallel reasoners. However, the approach focuses on classification (computing the subsumption hierarchy) rather than more general reasoning tasks, and it does not integrate learned heuristics or neural-symbolic methods for guiding the parallel search. The work demonstrates the potential for parallel execution to dramatically improve reasoning performance, but also highlights the challenges of designing parallel algorithms that maintain correctness while achieving good load balance (Quan et al., 2019).

### 4.2 Work-Stealing and Load Balancing

Work-stealing is a dynamic load balancing technique where idle processors "steal" work from busy processors, helping to ensure that all processors remain productively engaged throughout the computation. In the context of tableau-based reasoning, work-stealing can be applied by having idle threads steal unexplored branches from the search trees being explored by other threads. This approach can help address the load imbalance that arises from the highly irregular structure of tableau search spaces, where some branches may require extensive exploration while others quickly lead to clashes.

The parallel architecture developed by Quan et al. (2019) implements work-stealing for ontology classification, using atomic operations to enable threads to safely steal classification tasks from each other's work queues. The work-stealing scheduler monitors the load on each thread and triggers stealing when significant imbalances are detected. This dynamic approach proves more effective than static work distribution, which cannot adapt to the varying computational demands of different classification tasks.

However, work-stealing for tableau reasoning faces several challenges. First, the granularity of work units must be carefully chosen: too fine-grained and the overhead of stealing dominates; too coarse-grained and load imbalance persists. Second, when branches share learned information (such as cached subsumption relationships or conflict clauses), stealing a branch may require transferring substantial context to the stealing thread. Third, the effectiveness of work-stealing depends on the structure of the search space, and may be limited when the search is dominated by a few very expensive branches that cannot be easily subdivided (Quan et al., 2019).

### 4.3 Parallelized ABox Reasoning

ABox reasoning—reasoning about individuals and their relationships—presents particular challenges and opportunities for parallelization. Unlike TBox reasoning (reasoning about concepts and their relationships), which often exhibits significant independence between different parts of the ontology, ABox reasoning must handle complex interactions between individuals through role assertions and concept memberships.

Steigmiller et al. (2020) developed approaches to parallelized ABox reasoning and query answering for expressive description logics. The approach splits the model construction process, allowing different parts of the ABox to be processed concurrently. To ensure that partial models constructed in parallel are compatible with each other, the system employs an individual derivations cache where selected consequences for individuals are stored. Threads retrieve cached derivations when processing their assigned ABox portions and contribute new derivations back to the cache.

The caching strategy enables parallel threads to benefit from each other's work while avoiding the need for fine-grained synchronization during model construction. The system implements appropriate reuse and expansion strategies to ensure that cached consequences are correctly integrated into local completion graphs. For conjunctive query answering, the approach adapts the expansion criteria and splits the propagation work through partial models across multiple threads.

Experimental evaluation demonstrates that the parallelized approach achieves significant speedups on ABox reasoning tasks, particularly for large ABoxes where the work can be effectively distributed across multiple threads. However, the effectiveness of parallelization depends on the structure of the ABox and the degree of interaction between individuals. Highly interconnected ABoxes may require frequent synchronization through the cache, limiting scalability. The work highlights the importance of cache design and synchronization strategies for effective parallel ABox reasoning (Steigmiller et al., 2020).

### 4.4 Challenges in Parallel Tableau Reasoning

Despite the promising results achieved by parallel reasoning architectures, several fundamental challenges remain. First, the irregular and unpredictable structure of tableau search spaces makes it difficult to achieve consistent load balance. Some branches may require exponential exploration while others quickly terminate, and predicting which branches will be expensive is itself a challenging problem. This unpredictability limits the effectiveness of static work distribution and motivates the use of dynamic load balancing techniques like work-stealing.

Second, sharing learned information between parallel threads introduces synchronization overhead and potential contention. When one thread learns a useful fact (such as a subsumption relationship or a conflict clause), other threads should ideally benefit from this knowledge. However, propagating learned information requires synchronization, and excessive synchronization can negate the benefits of parallelization. Designing efficient mechanisms for sharing learned information without introducing excessive overhead remains an open challenge.

Third, the integration of learned heuristics with parallel execution introduces additional complexity. If different threads use learned models to guide their search, these models must either be shared (introducing synchronization overhead) or replicated (increasing memory usage and potentially leading to inconsistent guidance). Furthermore, if the learned models are updated during reasoning based on observed performance, coordinating these updates across threads becomes challenging. The intersection of learned heuristics and parallel execution represents an important but underexplored area for future research (Quan et al., 2019; Steigmiller et al., 2020).

## 5. Conflict and Nogood Learning Mechanisms

### 5.1 Conflict-Driven Learning in AI Planning

Conflict-driven learning has proven to be a powerful technique in SAT solving, where learned conflict clauses (nogoods) help prune the search space and avoid redundant exploration. The success of conflict-driven clause learning (CDCL) in SAT has motivated efforts to adapt similar techniques to other domains, including AI planning and description logic reasoning.

Steinmetz explored conflict-driven learning in AI planning state-space search, developing methods to identify and learn from conflicts encountered during plan search. When the planner reaches a state from which no valid plan can be constructed, it analyzes the conflict to identify a minimal set of decisions that led to the failure. This conflict is generalized and recorded as a nogood, preventing the planner from making the same combination of decisions in the future. The learned nogoods can dramatically reduce the search space, particularly for problems with many similar dead ends.

The key challenges in adapting conflict-driven learning to planning include: (1) identifying conflicts in the continuous state space of planning problems, (2) generalizing conflicts to cover multiple similar situations, and (3) efficiently storing and checking learned nogoods during search. The planning domain presents additional complexity compared to SAT because states are structured objects rather than simple variable assignments, and the notion of conflict must be defined in terms of planning goals and action preconditions rather than clause satisfaction (Steinmetz).

### 5.2 Conflict-Driven Constraint Answer Set Solving

Drescher developed conflict-driven approaches to constraint answer set solving, extending the CDCL paradigm to handle the richer constraint languages of answer set programming (ASP). The approach identifies conflicts during the search for answer sets, analyzes these conflicts to determine their causes, and learns constraint-based nogoods that prevent similar conflicts in the future.

The integration of constraint reasoning with conflict-driven learning presents several technical challenges. Constraints in ASP can involve complex relationships between variables, and conflicts may arise from the interaction of multiple constraints rather than from individual constraint violations. The conflict analysis procedure must therefore reason about constraint semantics to identify minimal conflict sets. Additionally, the learned nogoods must be represented in a form that enables efficient propagation during subsequent search.

The conflict-driven approach achieves significant performance improvements on constraint-heavy ASP problems, demonstrating the value of learning from conflicts in domains beyond pure SAT. The work also identifies important differences between SAT-style conflict learning and conflict learning in richer logical frameworks, including the need for more sophisticated conflict analysis and the potential for learning more expressive nogoods that capture constraint-level patterns (Drescher).

### 5.3 Conflict Generalisation in Answer Set Programming

Taupe et al. (2020) developed methods for conflict generalization in answer set programming, focusing on learning correct and effective non-ground constraints from conflicts encountered during search. Unlike ground conflict clauses learned in SAT, non-ground constraints can capture patterns that apply to multiple instantiations of variables, potentially providing more powerful pruning of the search space.

The approach analyzes conflicts to identify the essential structure that caused the failure, abstracting away specific variable bindings that are not relevant to the conflict. This generalization process must ensure that the learned constraints are correct (they do not eliminate valid answer sets) while being as general as possible (they eliminate as many invalid search paths as possible). The balance between correctness and generality is critical: overly specific constraints provide limited pruning power, while overly general constraints may eliminate valid solutions.

Experimental evaluation demonstrates that learning generalized non-ground constraints can significantly improve ASP solver performance, particularly on problems with regular structure where conflicts tend to follow similar patterns. However, the overhead of conflict generalization must be carefully managed, as overly aggressive generalization can be computationally expensive. The work provides insights into how conflict learning can be adapted to richer logical frameworks than SAT, with potential applications to description logic reasoning (Taupe et al., 2020).

### 5.4 Adaptation to Description Logic Reasoning

While conflict-driven learning has been extensively studied in SAT solving, planning, and answer set programming, its application to tableau-based description logic reasoning remains relatively underexplored. The tableau method's use of non-deterministic expansion rules and backtracking search suggests natural opportunities for conflict learning: when a branch of the tableau leads to a clash, the reasoner could analyze the clash to identify a minimal set of expansion decisions that caused it, then learn a nogood preventing the same combination of decisions in the future.

However, adapting conflict-driven learning to description logic tableau reasoning presents several challenges. First, the structure of tableau reasoning differs from SAT in important ways: tableau branches represent partial models rather than variable assignments, and clashes arise from violations of description logic semantics rather than clause falsification. Second, the expressiveness of description logics like SHOIQ introduces complexity not present in propositional SAT, including quantification, role reasoning, and nominals. Third, the learned nogoods must be represented in a form compatible with description logic semantics and must be efficiently checkable during tableau expansion.

Despite these challenges, the potential benefits of conflict-driven learning for description logic reasoning are substantial. Tableau reasoning often encounters similar conflicts repeatedly, particularly when reasoning about ontologies with regular structure. Learning from these conflicts could enable significant pruning of the search space and reduce redundant exploration. The integration of conflict learning with learned branch prioritization heuristics represents a particularly promising direction: neural networks could learn to predict which branches are likely to lead to conflicts, while conflict learning could provide training signal for improving these predictions.

The literature reviewed here reveals a significant gap: while conflict-driven learning has been successfully applied to related reasoning paradigms, and while machine learning has been applied to heuristic optimization in OWL reasoning, the integration of these approaches—using conflict-driven learning in conjunction with neural-symbolic branch prioritization for tableau-based description logic reasoning—remains largely unexplored. This gap represents an important opportunity for future research.

## 6. Benchmarks and Experimental Evaluation

### 6.1 Standard Ontology Benchmarks

Rigorous experimental evaluation of reasoning systems requires well-established benchmark suites that represent diverse ontology characteristics and reasoning challenges. Several standard benchmark collections have emerged in the description logic and OWL reasoning communities, though the literature reveals inconsistencies in benchmark usage and reporting that complicate cross-study comparisons.

The ORE (OWL Reasoner Evaluation) workshop series has established benchmark suites for evaluating OWL reasoners, including ontologies from various domains and with different expressiveness profiles. These benchmarks include both real-world ontologies (such as GALEN, SNOMED CT, and various biomedical ontologies) and synthetic ontologies designed to stress-test specific reasoning capabilities. However, the reviewed literature shows limited standardization in which specific benchmarks are used for evaluating learned heuristics and neural-symbolic approaches.

Zese et al. (2018) evaluated their probabilistic tableau reasoning extensions using ontologies from the biomedical domain, demonstrating the approach on real-world knowledge bases with uncertainty. The evaluation focused on correctness of probabilistic inference and scalability to ontologies of varying sizes. However, the paper does not report detailed runtime comparisons with non-probabilistic reasoners or systematic evaluation across diverse ontology types.

The lack of standardized benchmarks specifically designed for evaluating learned reasoning heuristics represents a significant gap in the field. Existing benchmarks were primarily designed to evaluate the correctness and performance of traditional reasoners, not to assess the effectiveness of learned strategies or the transferability of learned models across ontologies. Future work would benefit from benchmark suites that explicitly include diverse ontology structures, difficulty levels, and domain characteristics to enable systematic evaluation of learning-based approaches.

### 6.2 Theorem Proving and Automated Reasoning Datasets

Research on machine learning for theorem proving and automated reasoning has developed several benchmark datasets that, while not specific to description logic reasoning, provide valuable insights into evaluation methodologies and performance metrics. These datasets typically include large collections of theorem proving problems with known solutions, enabling supervised learning of proof strategies and systematic evaluation of learned heuristics.

Loos et al. (2017) developed deep network guided proof search for automated theorem proving, evaluating their approach on standard theorem proving benchmarks. The evaluation demonstrated that neural networks can learn effective proof guidance strategies from large corpora of proofs, achieving performance competitive with hand-crafted heuristics. The work established important methodologies for evaluating learned proof strategies, including metrics for proof length, search efficiency, and transfer learning performance.

Färber et al. (2021) evaluated machine learning guidance for connection tableaux on theorem proving benchmarks from the TPTP (Thousands of Problems for Theorem Provers) library. The evaluation included detailed analysis of how learned guidance affects proof search efficiency, comparing the number of inferences required, the size of the search space explored, and the wall-clock time to find proofs. The work also evaluated transfer learning by training models on one subset of problems and testing on different subsets, providing insights into the generalization capabilities of learned strategies.

Aygün et al. (2020) explored learning to prove from synthetic theorems, developing methods for generating large training datasets of theorem proving problems with known proofs. This approach addresses the challenge of limited training data in theorem proving by automatically generating diverse problems that cover a wide range of proof patterns. The synthetic data generation methodology could potentially be adapted to description logic reasoning, enabling the creation of large training sets for learning branch prioritization heuristics.

### 6.3 Performance Metrics and Evaluation Criteria

The evaluation of learned reasoning strategies requires careful consideration of appropriate performance metrics. Traditional metrics for reasoning systems include wall-clock time, number of inferences or rule applications, memory usage, and success rate (percentage of problems solved within a timeout). However, evaluating learned strategies introduces additional considerations, including training time, model size, inference latency for heuristic predictions, and transfer learning performance.

Mehri-Dehnavi (2019) evaluated machine learning-based heuristic selection primarily using speedup factors, comparing the runtime of the ML-based reasoner to the non-ML baseline. The reported speedups of up to two orders of magnitude demonstrate substantial performance improvements. However, the evaluation does not report detailed analysis of which ontology characteristics correlate with successful learning, nor does it systematically evaluate transfer learning across ontology domains.

For neural-symbolic approaches, additional metrics become relevant, including the accuracy of neural predictions (e.g., how often does the neural network correctly predict the best branch to explore?), the overhead of neural inference (how much time is spent computing neural predictions versus performing symbolic reasoning?), and the sample efficiency of learning (how much training data is required to achieve good performance?). The literature reveals limited standardization in reporting these metrics, making it difficult to compare different approaches.

The evaluation of parallel reasoning systems introduces further metrics, including parallel speedup (how much faster is the parallel system compared to sequential execution?), parallel efficiency (how effectively are additional processors utilized?), and scalability (how does performance change as the number of processors increases?). Quan et al. (2019) and Steigmiller et al. (2020) report these metrics for their parallel reasoning architectures, demonstrating good scalability on shared-memory systems. However, the interaction between parallelization and learned heuristics remains largely unevaluated in the literature.

### 6.4 Comparative Analysis of Approaches

Synthesizing the experimental results across the reviewed literature reveals several important patterns and gaps. First, machine learning approaches to heuristic optimization consistently demonstrate substantial performance improvements over traditional heuristics, with speedups ranging from factors of 2-10 to two orders of magnitude in favorable cases. These improvements are most pronounced on ontologies with complex structure where traditional heuristics perform poorly.

Second, the effectiveness of learned strategies varies significantly across ontologies, with some ontology types benefiting much more from learning than others. This variability suggests that learned approaches are not universally superior to traditional heuristics, but rather excel in specific contexts. Understanding which ontology characteristics predict successful learning remains an open question that requires more systematic evaluation.

Third, transfer learning—the ability of strategies learned on one set of ontologies to generalize to new ontologies—shows mixed results. Some approaches demonstrate reasonable transfer performance, while others show significant degradation when applied to ontologies with different characteristics. Improving transfer learning is critical for practical deployment of learned reasoning strategies, as retraining for each new ontology would be prohibitively expensive.

Fourth, the integration of multiple optimization techniques (learned heuristics, parallelization, conflict learning) remains largely unexplored. The reviewed literature includes work on learned heuristics, work on parallel reasoning, and work on conflict-driven learning, but little work that combines these approaches. The potential synergies between these techniques—for example, using learned heuristics to guide parallel work distribution, or using conflict learning to improve neural network training—represent important opportunities for future research.

## 7. Future Directions and Open Challenges

### 7.1 Transferability and Generalization

A critical challenge for learned reasoning strategies is achieving robust transferability across ontologies with diverse characteristics. Current approaches often show significant performance degradation when applied to ontologies substantially different from those in the training set. This limitation restricts the practical applicability of learned methods, as retraining for each new ontology or domain is often infeasible.

Future research should focus on developing learning architectures and training methodologies that promote generalization. Potential directions include: (1) meta-learning approaches that learn to quickly adapt to new ontologies with minimal additional training, (2) graph neural network architectures that can naturally handle ontologies of varying sizes and structures, (3) multi-task learning that trains on diverse ontology types simultaneously to learn more robust representations, and (4) transfer learning techniques that leverage pre-training on large ontology corpora to improve performance on specific target ontologies.

The development of standardized benchmark suites specifically designed to evaluate transfer learning is also critical. These benchmarks should include ontologies from diverse domains with varying structural characteristics, enabling systematic evaluation of how learned strategies generalize across ontology types. The benchmarks should also include explicit train/test splits that prevent data leakage and enable fair comparison of different approaches (Blaauwbroek et al., 2024; Crouse et al., 2019).

### 7.2 Integration of Neural and Symbolic Components

The reviewed literature reveals a spectrum of integration strategies between neural and symbolic components, ranging from loosely coupled systems where neural networks select among predefined symbolic heuristics, to more tightly integrated systems where neural networks directly guide symbolic reasoning steps. Future research should explore deeper integration strategies that leverage the complementary strengths of neural and symbolic methods.

Promising directions include: (1) differentiable logic approaches that make symbolic reasoning operations differentiable, enabling end-to-end training of neural-symbolic systems, (2) neural-symbolic architectures that maintain explicit symbolic representations while using neural networks for pattern recognition and heuristic learning, (3) hybrid systems that use symbolic reasoning to provide interpretable explanations for neural predictions, and (4) co-training approaches where neural and symbolic components learn from each other's successes and failures.

The challenge of maintaining logical soundness while leveraging neural learning is particularly important for description logic reasoning, where correctness guarantees are often critical. Future work should explore architectures that use neural networks to guide search and prioritize branches while ensuring that the final reasoning results are verified by sound symbolic algorithms. This separation of concerns—neural guidance with symbolic verification—may provide a practical path toward reliable neural-symbolic reasoning systems (Singh et al., 2023; Hohenecker et al., 2020).

### 7.3 Scalability to Large-Scale Ontologies

As ontologies continue to grow in size and complexity, scalability becomes an increasingly critical concern. Large biomedical ontologies like SNOMED CT contain hundreds of thousands of concepts and millions of axioms, presenting substantial computational challenges for reasoning systems. Future research should address scalability through multiple complementary approaches.

First, learned heuristics should be designed to scale efficiently to large ontologies, with neural architectures that can process large graphs without excessive memory or computation requirements. Lightweight models that can make fast predictions are particularly important, as the overhead of neural inference must not dominate the reasoning time. Second, parallel execution strategies should be further developed to exploit modern multi-core and distributed computing architectures, with particular attention to load balancing and efficient sharing of learned information across parallel workers.

Third, approximate reasoning methods that trade some precision for substantial performance improvements may be valuable for certain applications. Neural-symbolic approaches that provide approximate answers with confidence estimates could enable interactive reasoning over large ontologies, with exact symbolic verification performed only when high confidence is required. Fourth, incremental reasoning techniques that efficiently update reasoning results when ontologies change should be integrated with learned heuristics, enabling efficient reasoning in dynamic environments (Quan et al., 2019; Steigmiller et al., 2020).

### 7.4 Formal Verification and Correctness Guarantees

The integration of machine learning with logical reasoning raises important questions about correctness and reliability. While learned heuristics can improve performance, they may also introduce errors if not carefully designed and validated. Future research should address the challenge of providing formal correctness guarantees for neural-symbolic reasoning systems.

Recent work on formally verified reasoners demonstrates the feasibility of mechanically verifying the correctness of reasoning algorithms. İleri et al. (2024) developed VEL, a formally verified reasoner for the OWL2 EL profile, using the Coq proof assistant to provide machine-checkable correctness proofs. This work revealed errors in the original algorithm's completeness proofs, highlighting the value of formal verification. Future research should explore how formal verification techniques can be applied to neural-symbolic systems, potentially verifying that learned heuristics preserve correctness properties even as they optimize performance.

One promising approach is to maintain a clear separation between the learned heuristic component (which guides search but does not affect correctness) and the symbolic reasoning component (which is formally verified). This architecture ensures that even if the learned heuristics make poor predictions, the reasoning results remain correct—only the performance is affected. Developing formal frameworks for reasoning about the correctness of such hybrid systems represents an important direction for future research (İleri et al., 2024).

## 8. Conclusion

This comprehensive review has examined the emerging field of hybrid neural-symbolic approaches to branch prioritization in tableau-based reasoning for OWL ontologies and description logics. The synthesis of research from 2015-2026 reveals significant progress in applying machine learning to optimize heuristic decision-making in reasoning systems, with demonstrated speedups of up to two orders of magnitude compared to traditional approaches. However, the review also identifies substantial gaps and challenges that must be addressed to realize the full potential of neural-symbolic reasoning.

Key findings from the review include: (1) Machine learning approaches, particularly SVM-based heuristic selection, can dramatically improve reasoning performance by learning to select effective strategies based on ontology characteristics. (2) Deep neural networks show promise for learning distributed representations of ontological concepts and guiding proof search, though challenges remain in ensuring logical soundness and achieving robust transfer learning. (3) Reinforcement learning provides a natural framework for learning proof guidance strategies, with successful applications in theorem proving that could potentially be adapted to description logic reasoning. (4) Parallel computing architectures with work-stealing schedulers can achieve significant speedups on ontology reasoning tasks, though integration with learned heuristics remains underexplored. (5) Conflict-driven learning has proven highly effective in SAT solving and related domains, but its application to tableau-based description logic reasoning remains limited.

Critical gaps identified in the review include: (1) Limited exploration of neural-symbolic integration specifically for expressive description logics like ALC and SHOIQ, with most work focusing on lightweight logics or approximate reasoning. (2) Lack of standardized benchmarks for evaluating learned reasoning strategies, particularly benchmarks designed to assess transfer learning and generalization. (3) Minimal integration of conflict-driven learning with tableau-based description logic reasoning, despite the success of this approach in related domains. (4) Limited investigation of how learned heuristics interact with parallel execution strategies and work-stealing schedulers. (5) Insufficient attention to formal verification and correctness guarantees for neural-symbolic reasoning systems.

Future research directions that emerge from this review include: (1) Developing lightweight neural architectures suitable for real-time branch prioritization in tableau reasoning, with particular attention to inference latency and scalability. (2) Creating unified benchmark suites specifically designed for evaluating learned reasoning strategies across diverse ontology types and domains. (3) Exploring deep integration of conflict-driven learning with neural-symbolic branch prioritization, using conflicts as training signals for improving learned heuristics. (4) Investigating the synergies between learned heuristics, parallel execution, and speculative scheduling, potentially using neural networks to guide work distribution and load balancing. (5) Developing formal frameworks for verifying the correctness of neural-symbolic reasoning systems while preserving the performance benefits of learned heuristics.

The convergence of neural learning and symbolic reasoning represents a fundamental shift in how we approach automated reasoning. While significant challenges remain, the progress documented in this review demonstrates the viability and promise of hybrid neural-symbolic approaches. As the field matures, we can anticipate reasoning systems that combine the learning capabilities and pattern recognition strengths of neural networks with the interpretability, compositionality, and logical rigor of symbolic methods, enabling more efficient and robust reasoning over complex ontologies and knowledge bases.

## References

Acosta, M., Cudré-Mauroux, P., Fundulaki, I., & Simperl, E. (2024). Neuro-symbolic query optimization in knowledge graphs. arXiv. https://doi.org/10.48550/arxiv.2411.14277

Acosta, M., Cudré-Mauroux, P., Fundulaki, I., & Simperl, E. (2025). Neuro-symbolic query optimization in knowledge graphs. *Frontiers in Artificial Intelligence and Applications*. https://doi.org/10.3233/faia250225

Aygün, E., Ahmed, Z., Anand, A., Firoiu, V., Glorot, X., Orseau, L., Precup, D., & Mourad, S. (2020). Learning to prove from synthetic theorems. *arXiv: Logic in Computer Science*.

Blaauwbroek, L., Crouse, M., Kaliszyk, C., Kinyon, M., Kohlhase, M., Korovin, K., Matuszewski, J., Olšák, M., Piotrowski, B., Rabe, F., Rawson, M., Schoisswohl, J., Suda, M., Sutcliffe, G., Traytel, D., Urban, J., & Zombori, Z. (2024). Learning guided automated reasoning: A brief survey. arXiv. https://doi.org/10.48550/arxiv.2403.04017

Crouse, M., Abdelaziz, I., Cornelio, C., Thost, V., Wu, L., Forbus, K., & Fokoue, A. (2019). A deep reinforcement learning based approach to learning transferable proof guidance strategies. *arXiv: Artificial Intelligence*.

Drescher, C. Conflict-driven constraint answer set solving.

Färber, M., Kaliszyk, C., & Urban, J. (2021). Machine learning guidance for connection tableaux. *Journal of Automated Reasoning*. https://doi.org/10.1007/S10817-020-09576-7

Hohenecker, P., Lukasiewicz, T., & Bühmann, L. (2018). Ontology reasoning with deep neural networks. *arXiv: Artificial Intelligence*. https://doi.org/10.1613/JAIR.1.11661

Hohenecker, P., Lukasiewicz, T., & Bühmann, L. (2020). Ontology reasoning with deep neural networks. *Journal of Artificial Intelligence Research*. https://doi.org/10.1613/JAIR.1.11661

İleri, A. M., Çakır, M. S., & Kaliszyk, C. (2024). VEL: A formally verified reasoner for OWL2 EL profile. arXiv. https://doi.org/10.48550/arxiv.2412.08739

Loos, S., Irving, G., Szegedy, C., & Kaliszyk, C. (2017). Deep network guided proof search. *International Conference on Logic Programming*. https://doi.org/10.29007/8MWC

McKeown, N., Sutcliffe, G., & Suttner, C. (2023). Reinforcement learning for guiding the E theorem prover. *Proceedings of the International Florida Artificial Intelligence Research Society Conference*. https://doi.org/10.32473/flairs.36.133334

Mehri-Dehnavi, H. (2019). A machine learning approach for optimizing heuristic decision-making in OWL reasoners.

Potoniec, J. (2023). Reason-able embeddings: Learning concept embeddings with a transferable neural reasoner. *Semantic Web*. https://doi.org/10.3233/sw-233355

Quan, T. T., Hui, S. C., & Cao, T. H. (2019). A parallel computing architecture for high-performance OWL reasoning. *Parallel Computing*. https://doi.org/10.1016/J.PARCO.2018.05.001

Singh, G., Mutharaju, R., & Hitzler, P. (2023). Chapter 2. Neuro-symbolic RDF and description logic reasoners: The state-of-the-art and challenges. *Frontiers in Artificial Intelligence and Applications*. https://doi.org/10.3233/faia230134

Steigmiller, A., Glimm, B., & Liebig, T. (2020). Parallelised ABox reasoning and query answering with expressive description logics (Extended Abstract). *Description Logics*.

Steinmetz, M. Conflict-driven learning in AI planning state-space search.

Taupe, R., Weinzierl, A., & Friedrich, G. (2020). Conflict generalisation in ASP: Learning correct and effective non-ground constraints. *Theory and Practice of Logic Programming*. https://doi.org/10.1017/S1471068420000368

Wu, Y., Pan, J. Z., & Kollingbaum, M. (2025). A neuro-symbolic approach to symbol grounding for ALC-ontologies. https://doi.org/10.1145/3711896.3736926

Zese, R., Bellodi, E., Lamma, E., Riguzzi, F., & Aguiari, F. (2018). Tableau reasoning for description logics and its extension to probabilities. *Annals of Mathematics and Artificial Intelligence*. https://doi.org/10.1007/S10472-016-9529-3
