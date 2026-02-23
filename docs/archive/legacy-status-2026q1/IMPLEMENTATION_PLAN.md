# Implementation Plan for Remaining Components

This document outlines the plan to implement the remaining components of the Tableauxx hybrid reasoner.

## Current Status Overview

| Component | Status | Priority |
|-----------|--------|----------|
| ALC Tableau (Python) | ✅ Complete | - |
| Meta-Reasoner (Rust) | ✅ Framework | P3 |
| Evolutionary Optimizer (Rust) | ✅ Structure | P3 |
| **Core OWL Data Structures** | 🚧 Needed | **P1** |
| **OWL Parsers** | 🚧 Needed | **P1** |
| **SROIQ(D) Tableaux** | 🚧 Stub | **P1** |
| **EL++ Saturation** | 🚧 Stub | **P2** |
| **EL++ Transformation** | 🚧 Stub | **P2** |

---

## Phase 1: Core OWL Data Structures (Priority: P1)

### 1.1 OWL Entities

Create `src/owl/` module with core OWL entities:

```rust
// src/owl/entities.rs
pub struct Class(IRI);
pub struct ObjectProperty(IRI);
pub struct DataProperty(IRI);
pub struct NamedIndividual(IRI);
pub struct Datatype(IRI);
```

### 1.2 Class Expressions (SROIQ)

```rust
// src/owl/expressions.rs
pub enum ClassExpression {
    // Basic
    Class(Class),
    
    // Boolean connectives
    ObjectIntersectionOf(Vec<ClassExpression>),
    ObjectUnionOf(Vec<ClassExpression>),
    ObjectComplementOf(Box<ClassExpression>),
    
    // Restrictions
    ObjectSomeValuesFrom(ObjectProperty, Box<ClassExpression>),
    ObjectAllValuesFrom(ObjectProperty, Box<ClassExpression>),
    ObjectHasValue(ObjectProperty, NamedIndividual),
    ObjectMinCardinality(u32, ObjectProperty, Option<Box<ClassExpression>>),
    ObjectMaxCardinality(u32, ObjectProperty, Option<Box<ClassExpression>>),
    ObjectExactCardinality(u32, ObjectProperty, Option<Box<ClassExpression>>),
    
    // Nominals
    ObjectOneOf(Vec<NamedIndividual>),
    
    // Data restrictions
    DataSomeValuesFrom(DataProperty, DatatypeRestriction),
    DataAllValuesFrom(DataProperty, DatatypeRestriction),
    // ... etc
}
```

### 1.3 Axioms

```rust
// src/owl/axioms.rs
pub enum Axiom {
    // Class axioms
    SubClassOf(ClassExpression, ClassExpression),
    EquivalentClasses(Vec<ClassExpression>),
    DisjointClasses(Vec<ClassExpression>),
    DisjointUnion(Class, Vec<ClassExpression>),
    
    // Property axioms
    SubObjectPropertyOf(ObjectProperty, ObjectProperty),
    EquivalentObjectProperties(Vec<ObjectProperty>),
    DisjointObjectProperties(Vec<ObjectProperty>),
    InverseObjectProperties(ObjectProperty, ObjectProperty),
    ObjectPropertyDomain(ObjectProperty, ClassExpression),
    ObjectPropertyRange(ObjectProperty, ClassExpression),
    FunctionalObjectProperty(ObjectProperty),
    InverseFunctionalObjectProperty(ObjectProperty),
    ReflexiveObjectProperty(ObjectProperty),
    IrreflexiveObjectProperty(ObjectProperty),
    SymmetricObjectProperty(ObjectProperty),
    AsymmetricObjectProperty(ObjectProperty),
    TransitiveObjectProperty(ObjectProperty),
    
    // Assertions
    ClassAssertion(ClassExpression, NamedIndividual),
    ObjectPropertyAssertion(ObjectProperty, NamedIndividual, NamedIndividual),
    NegativeObjectPropertyAssertion(ObjectProperty, NamedIndividual, NamedIndividual),
    SameIndividual(Vec<NamedIndividual>),
    DifferentIndividuals(Vec<NamedIndividual>),
    
    // Keys
    HasKey(ClassExpression, Vec<ObjectProperty>, Vec<DataProperty>),
}
```

### 1.4 Ontology Structure

```rust
// src/owl/ontology.rs
pub struct Ontology {
    iri: Option<IRI>,
    axioms: HashSet<Axiom>,
    
    // Indexes for fast access
    class_index: HashMap<Class, Vec<Axiom>>,
    property_index: HashMap<ObjectProperty, Vec<Axiom>>,
    individual_index: HashMap<NamedIndividual, Vec<Axiom>>,
}

impl Ontology {
    pub fn new() -> Self { ... }
    pub fn add_axiom(&mut self, axiom: Axiom) { ... }
    pub fn get_axioms(&self) -> &HashSet<Axiom> { ... }
    pub fn get_class_axioms(&self, class: &Class) -> &[Axiom] { ... }
    // ... etc
}
```

### Phase 1 Deliverables
- [ ] `src/owl/mod.rs` - Module structure
- [ ] `src/owl/entities.rs` - Basic OWL entities
- [ ] `src/owl/expressions.rs` - SROIQ class expressions
- [ ] `src/owl/axioms.rs` - All OWL axiom types
- [ ] `src/owl/ontology.rs` - Ontology container with indexes
- [ ] Unit tests for all data structures

**Estimated Effort**: 2-3 weeks

---

## Phase 2: OWL Parsers (Priority: P1)

### 2.1 Turtle Parser

Use `rio_turtle` crate to parse Turtle format:

```rust
// src/parsers/turtle.rs
use rio_turtle::TurtleParser;
use rio_api::parser::TriplesParser;

pub struct TurtleOntologyParser;

impl TurtleOntologyParser {
    pub fn parse_file(path: &Path) -> Result<Ontology, ParseError> {
        // Parse triples and convert to OWL axioms
    }
    
    pub fn parse_str(input: &str) -> Result<Ontology, ParseError> {
        // Parse from string
    }
}
```

### 2.2 RDF/XML Parser

Use `rio_xml` for RDF/XML:

```rust
// src/parsers/rdf_xml.rs
use rio_xml::RdfXmlParser;

pub struct RdfXmlOntologyParser;

impl RdfXmlOntologyParser {
    pub fn parse_file(path: &Path) -> Result<Ontology, ParseError> { ... }
}
```

### 2.3 OWL/XML Parser

Use `xmltree` or `serde-xml-rs`:

```rust
// src/parsers/owl_xml.rs
use xmltree::Element;

pub struct OwlXmlParser;

impl OwlXmlParser {
    pub fn parse_file(path: &Path) -> Result<Ontology, ParseError> { ... }
}
```

### 2.4 OWL Functional Syntax Parser

```rust
// src/parsers/functional.rs
pub struct FunctionalSyntaxParser;

impl FunctionalSyntaxParser {
    // Parse Manchester-like syntax
}
```

### Phase 2 Deliverables
- [ ] `src/parsers/mod.rs` - Parser module
- [ ] `src/parsers/turtle.rs` - Turtle format
- [ ] `src/parsers/rdf_xml.rs` - RDF/XML format
- [ ] `src/parsers/owl_xml.rs` - OWL/XML format
- [ ] `src/parsers/functional.rs` - Functional syntax
- [ ] Test with sample ontologies (univ-bench.owl, etc.)

**Estimated Effort**: 3-4 weeks

---

## Phase 3: SROIQ(D) Tableaux Reasoner (Priority: P1)

This is the core reasoning engine. Port the working Python ALC tableau to Rust and extend to SROIQ(D).

### 3.1 Tableaux Node Structure

```rust
// src/reasoning/tableaux/node.rs
pub struct TableauxNode {
    id: NodeId,
    /// Concept assertions: individual -> set of class expressions
    concept_assertions: HashMap<IndividualId, HashSet<ClassExpression>>,
    /// Role assertions: (source, property) -> targets
    role_assertions: HashMap<(IndividualId, ObjectProperty), Vec<IndividualId>>,
    /// Parent node for backtracking
    parent: Option<NodeId>,
    /// Applied rules tracking
    applied_rules: HashSet<RuleApplication>,
    /// Dependency tracking for backtracking
    dependencies: DependencySet,
}
```

### 3.2 Expansion Rules

```rust
// src/reasoning/tableaux/rules.rs
pub trait ExpansionRule {
    fn is_applicable(&self, node: &TableauxNode) -> Vec<RuleApplication>;
    fn apply(&self, node: &TableauxNode, app: RuleApplication) -> Vec<TableauxNode>;
}

/// ⊓-rule: If C ⊓ D ∈ L(x), add C and D to L(x)
pub struct ConjunctionRule;

/// ⊔-rule: If C ⊔ D ∈ L(x), branch with C or D
pub struct DisjunctionRule;

/// ∃-rule: If ∃R.C ∈ L(x), create new y with R(x,y) and C(y)
pub struct ExistentialRule;

/// ∀-rule: If ∀R.C ∈ L(x) and R(x,y), add C(y)
pub struct UniversalRule;

/// ≥-rule: For number restrictions
pub struct MinCardinalityRule;

/// ≤-rule: Merge individuals for max cardinality
pub struct MaxCardinalityRule;

/// Nominal rule: Handle {a}
pub struct NominalRule;
```

### 3.3 Blocking Strategy

```rust
// src/reasoning/tableaux/blocking.rs
pub trait BlockingStrategy {
    fn is_blocked(&self, node: &TableauxNode, individual: IndividualId) -> bool;
}

/// Subset blocking: block if L(x) ⊆ L(y)
pub struct SubsetBlocking;

/// Equality blocking
pub struct EqualityBlocking;

/// Pair-wise blocking for SROIQ
pub struct PairwiseBlocking;
```

### 3.4 Main Reasoner

```rust
// src/reasoning/tableaux/reasoner.rs
pub struct TableauxReasoner {
    ontology: Arc<Ontology>,
    config: TableauxConfig,
    rules: Vec<Box<dyn ExpansionRule>>,
    blocking: Box<dyn BlockingStrategy>,
    cache: TableauxCache,
}

impl TableauxReasoner {
    pub fn is_consistent(&mut self) -> Result<bool, ReasoningError> {
        // Initialize with owl:Thing
        // Apply expansion rules until clash or complete
        // Handle branching with backtracking
    }
    
    pub fn is_satisfiable(&mut self, class: &ClassExpression) -> Result<bool, ReasoningError> {
        // Test if class expression is satisfiable
    }
    
    pub fn classify(&mut self) -> Result<Taxonomy, ReasoningError> {
        // Compute class hierarchy
    }
}
```

### 3.5 Integration with Existing Code

Replace the stub in `src/tableaux.rs` with the real implementation.

### Phase 3 Deliverables
- [ ] Port ALC tableau from Python to Rust
- [ ] Extend to SROIQ (add roles, cardinality, nominals)
- [ ] Add datatype support (D)
- [ ] Implement multiple blocking strategies
- [ ] Add dependency-directed backtracking
- [ ] Integrate with meta-reasoner
- [ ] Comprehensive tests (100+ test cases)

**Estimated Effort**: 6-8 weeks

---

## Phase 4: EL++ Saturation Engine (Priority: P2)

EL++ is polynomial-time and important for large ontologies.

### 4.1 Normal Form Conversion

```rust
// src/reasoning/el/normal_form.rs
pub fn to_el_normal_form(axiom: &Axiom) -> Vec<ELAxiom> {
    // Convert general axioms to EL normal form
    // C ⊑ D where C is:
    //   - A (atomic)
    //   - A₁ ⊓ A₂ (conjunction)
    //   - ∃r.A (existential)
    //   - {a} (nominal)
}
```

### 4.2 Completion Rules

```rust
// src/reasoning/el/rules.rs
pub struct CompletionRules {
    subsumptions: HashMap<Class, HashSet<Class>>,
    relations: HashMap<(Class, ObjectProperty), HashSet<Class>>,
}

impl CompletionRules {
    /// CR1: If A ⊑ B and B ⊑ C, then A ⊑ C
    pub fn apply_subsumption_rule(&mut self) { ... }
    
    /// CR2: If A ⊑ ∃r.B and B ⊑ C, then A ⊑ ∃r.C
    pub fn apply_relation_rule(&mut self) { ... }
    
    /// CR3: If A ⊑ ∃r.B and B ⊑ ⊥, then A ⊑ ⊥
    pub fn detect_inconsistency(&self) -> bool { ... }
    
    /// CR4: Handle nominals
    pub fn apply_nominal_rule(&mut self) { ... }
}
```

### 4.3 Saturation Algorithm

```rust
// src/reasoning/el/saturator.rs
pub struct ELSaturator {
    ontology: Arc<Ontology>,
    state: CompletionRules,
}

impl ELSaturator {
    pub fn saturate(&mut self) -> Result<bool, ReasoningError> {
        // Apply completion rules until fixpoint
        // Return true if consistent
    }
    
    pub fn is_subclass_of(&self, sub: &Class, sup: &Class) -> bool {
        // Check if sub ⊑ sup in saturated state
        self.state.subsumptions.get(sub).map_or(false, |s| s.contains(sup))
    }
}
```

### Phase 4 Deliverables
- [ ] EL normal form conversion
- [ ] Completion rules implementation
- [ ] Saturation algorithm
- [ ] Integration with meta-reasoner (EL profile detection)
- [ ] Tests with EL ontologies

**Estimated Effort**: 3-4 weeks

---

## Phase 5: EL++ to Datalog Transformation (Priority: P2)

Transform EL++ ontologies to Datalog for efficient reasoning.

### 5.1 Datalog Representation

```rust
// src/reasoning/datalog/ast.rs
pub struct DatalogProgram {
    facts: Vec<Fact>,
    rules: Vec<Rule>,
    queries: Vec<Query>,
}

pub struct Fact(pub Predicate, pub Vec<Constant>);
pub struct Rule {
    pub head: Atom,
    pub body: Vec<Atom>,
}
```

### 5.2 Transformation Rules

```rust
// src/reasoning/datalog/transform.rs
pub struct ELToDatalogTransformer;

impl ELToDatalogTransformer {
    /// Transform C ⊑ D to Datalog rule
    pub fn transform_subsumption(&self, sub: &Class, sup: &Class) -> Rule {
        // sup(X) :- sub(X).
    }
    
    /// Transform ∃r.C to Datalog
    pub fn transform_existential(&self, prop: &ObjectProperty, class: &Class) -> (Rule, Fact) {
        // Create auxiliary predicate
    }
    
    /// Transform C(a) to Datalog fact
    pub fn transform_assertion(&self, class: &Class, individual: &NamedIndividual) -> Fact {
        // class(constant).
    }
}
```

### 5.3 Datalog Engine Integration

Use an existing Datalog engine or implement a simple one:

```rust
// src/reasoning/datalog/engine.rs
pub struct DatalogEngine {
    program: DatalogProgram,
}

impl DatalogEngine {
    pub fn evaluate(&mut self) -> Result<EvaluationResult, ReasoningError> {
        // Semi-naive evaluation
    }
}
```

### Phase 5 Deliverables
- [ ] Datalog AST definitions
- [ ] EL++ to Datalog transformation
- [ ] Datalog engine (or integration with existing)
- [ ] Integration with meta-reasoner
- [ ] Tests

**Estimated Effort**: 3-4 weeks

---

## Phase 6: Integration & Benchmarks (Priority: P2)

### 6.1 Meta-Reasoner Integration

Ensure meta-reasoner properly selects between:
- EL Saturation (for EL profile)
- EL Datalog (for EL++ profile)
- Tableaux (for full SROIQ)

### 6.2 Standard Test Ontologies

Download and test with:
- [ ] LUBM (Lehigh University Benchmark)
- [ ] Gene Ontology (GO)
- [ ] ORE Competition benchmarks
- [ ] SNOMED CT (if available)

### 6.3 Comparative Benchmarks

Compare against:
- [ ] HermiT
- [ ] Pellet
- [ ] ELK
- [ ] Konclude

### 6.4 Performance Optimization

- [ ] Profile and optimize hotspots
- [ ] Implement caching strategies
- [ ] Parallel processing where applicable

### Phase 6 Deliverables
- [ ] Working integration of all components
- [ ] Real benchmarks against standard ontologies
- [ ] Comparative analysis with existing reasoners
- [ ] Performance optimization
- [ ] Final documentation

**Estimated Effort**: 4-6 weeks

---

## Total Timeline

| Phase | Duration | Cumulative |
|-------|----------|------------|
| Phase 1: Data Structures | 2-3 weeks | 2-3 weeks |
| Phase 2: Parsers | 3-4 weeks | 5-7 weeks |
| Phase 3: SROIQ Tableaux | 6-8 weeks | 11-15 weeks |
| Phase 4: EL Saturation | 3-4 weeks | 14-19 weeks |
| Phase 5: Datalog Transform | 3-4 weeks | 17-23 weeks |
| Phase 6: Integration | 4-6 weeks | 21-29 weeks |

**Total Estimated Time: 5-7 months** (for full implementation)

---

## Intermediate Milestones

### Milestone 1: Basic Reasoning (Week 8)
- Core data structures complete
- Turtle parser working
- ALC tableau in Rust working
- Can reason with simple ontologies

### Milestone 2: SROIQ Support (Week 15)
- Full SROIQ tableaux
- RDF/XML and OWL/XML parsers
- Can handle expressive ontologies

### Milestone 3: Hybrid Reasoner (Week 23)
- EL saturation working
- EL++ to Datalog working
- Meta-reasoner selecting strategies
- Competitive performance on benchmarks

---

## Next Steps

1. **Start with Phase 1**: Create the core OWL data structures
2. **Parallel work**: Can work on parsers while developing tableaux
3. **Test-driven**: Write tests as we go
4. **Incremental integration**: Connect components as they're ready

Would you like to start with Phase 1, or focus on a specific component first?
