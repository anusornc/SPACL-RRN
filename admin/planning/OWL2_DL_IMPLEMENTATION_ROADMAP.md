# OWL2 DL (SROIQ(D)) Full Implementation Roadmap

## Executive Summary

**Current:** ALC/SHOIQ (usable, published)  
**Target:** SROIQ(D) - Full OWL2 DL  
**Timeline:** 4-6 months full-time  
**Recommendation:** Phased approach - extend gradually post-publication

---

## Missing Components Analysis

### 1. Transitive Roles (S) - 2-3 weeks
**Current:** Partial support exists  
**Status:** `TransitivePropertyAxiom` defined, tableaux has `transitive_properties`  

**Implementation needed:**
- [ ] Complete transitive rule application in tableaux
- [ ] Role composition for transitive chains
- [ ] Interaction with role hierarchies
- [ ] Test: `(hasPart some (hasPart some Leg)) ⊑ (hasPart some Leg)`

```rust
// New tableau rule needed:
// If R is transitive, R(a,b) ∈ L(x), and R(b,c) ∈ L(y), 
// then add R(a,c) to L(z) for appropriate z
```

---

### 2. Complex Role Inclusions/Chains (R) - 4-6 weeks
**Current:** Simple role hierarchies only  
**Missing:** Property chains (e.g., `hasParent ∘ hasBrother ⊑ hasUncle`)

**Implementation needed:**
- [ ] `SubPropertyChainOfAxiom` (already exists!)
- [ ] Role chain expansion in preprocessing
- [ ] Tableaux rules for chain decomposition
- [ ] Interaction with transitive roles
- [ ] Test: `hasParent ∘ hasBrother ⊑ hasUncle`

**Complexity:** High - chains create new role assertions dynamically

```rust
// Preprocessing: expand chains into virtual properties
// hasParent ∘ hasBrother ⊑ hasUncle
// becomes: for each x,y,z: if hasParent(x,y) and hasBrother(y,z) then hasUncle(x,z)
```

---

### 3. Nominals (O) - 2-3 weeks  
**Current:** Partial - `NamedIndividual` exists  
**Missing:** `ObjectHasValue`, `ObjectOneOf` reasoning

**Implementation needed:**
- [ ] `ObjectHasValue` class expression handling
- [ ] `ObjectOneOf` (enumerated classes)
- [ ] Nominal introduction rule
- [ ] Blocking strategy updates for nominals
- [ ] Test: `{John}` (singleton class)

```rust
// New class expressions:
ObjectHasValue(ObjectProperty, NamedIndividual)  // ∃hasPet.{Fido}
ObjectOneOf(Vec<NamedIndividual>)                // {John, Mary, Bob}
```

---

### 4. Datatypes (D) - 6-8 weeks ⚠️ HARDEST
**Current:** None  
**Missing:** Everything

**Implementation needed:**

#### 4.1 Datatype Infrastructure
- [ ] `Datatype` entity type
- [ ] Built-in datatypes: xsd:string, xsd:integer, xsd:decimal, xsd:boolean, xsd:dateTime
- [ ] Literal values with language tags and datatypes
- [ ] Datatype IRI handling

#### 4.2 Datatype Property Support
- [ ] `DataProperty` (exists but unused)
- [ ] `DataSomeValuesFrom` - ∃hasAge.xsd:integer
- [ ] `DataAllValuesFrom` - ∀hasAge.xsd:integer  
- [ ] `DataHasValue` - ∃hasAge.{42^^xsd:integer}
- [ ] `DataMinCardinality`, `DataMaxCardinality`, `DataExactCardinality`

#### 4.3 Facets
- [ ] `DatatypeRestriction` with facets
- [ ] xsd:minInclusive, xsd:maxInclusive
- [ ] xsd:minExclusive, xsd:maxExclusive
- [ ] xsd:pattern (regex)
- [ ] xsd:length, xsd:minLength, xsd:maxLength

#### 4.4 Datatype Reasoner
- [ ] Integration with datatype oracle
- [ ] Range checking for numeric types
- [ ] Pattern matching for strings
- [ ] Disjointness (xsd:integer ∩ xsd:string = ⊥)

```rust
// Example usage:
DataSomeValuesFrom {
    property: hasAge,
    datatype: DatatypeRestriction {
        base: xsd:integer,
        facets: [minInclusive(18), maxInclusive(65)]
    }
}
```

---

### 5. Keys (hasKey) - 3-4 weeks (Optional)
**Current:** None  
**Missing:** `HasKeyAxiom`

**Implementation needed:**
- [ ] `HasKeyAxiom` (already exists!)
- [ ] Key-based identification rule
- [ ] Merge equivalent individuals by key
- [ ] Interaction with nominals

```rust
// hasKey(Person) = {hasSSN}
// If hasSSN(a, "123") and hasSSN(b, "123"), then a = b
```

---

### 6. Advanced Cardinality (Q) - 2-3 weeks
**Current:** Partial  
**Missing:** Qualified cardinality with complex fillers

**Implementation needed:**
- [ ] `ObjectMinCardinality(n, R, C)` - at least n R-successors in C
- [ ] `ObjectMaxCardinality(n, R, C)` - at most n R-successors in C
- [ ] `ObjectExactCardinality(n, R, C)` - exactly n R-successors in C
- [ ] Chosen individual strategy for ≥
- [ ] At-most rule with merging for ≤

```rust
// Already have (unqualified):
ObjectMinCardinality(3, hasChild)  // at least 3 children

// Need to support (qualified):
ObjectMinCardinality(2, hasChild, Male)  // at least 2 sons
```

---

## Implementation Phases

### Phase 1: Quick Wins (1 month)
Focus: Features that are mostly there, just need completion

| Feature | Effort | Impact |
|---------|--------|--------|
| Transitive roles | 2-3 weeks | High - commonly used |
| Qualified cardinality | 2-3 weeks | Medium |
| Nominals | 2-3 weeks | High - OWL 2 feature |
| **Total** | **6-9 weeks** | **High value** |

### Phase 2: Complex Roles (1.5 months)
Focus: Property chains - hardest structural feature

| Feature | Effort | Impact |
|---------|--------|--------|
| Role chains | 4-6 weeks | Medium |
| Role hierarchy + chain interaction | 2 weeks | Medium |
| **Total** | **6-8 weeks** | **Medium value** |

### Phase 3: Datatypes (2 months) ⚠️ HARDEST
Focus: Full datatype support

| Feature | Effort | Impact |
|---------|--------|--------|
| Datatype infrastructure | 2 weeks | Critical |
| Data property reasoning | 2 weeks | High |
| Facets | 2 weeks | Medium |
| Datatype oracle | 3-4 weeks | Critical |
| **Total** | **9-10 weeks** | **Very High** |

### Phase 4: Keys (2-3 weeks)
Focus: HasKey axiom

| Feature | Effort | Impact |
|---------|--------|--------|
| Keys implementation | 2-3 weeks | Low-Medium |
| **Total** | **2-3 weeks** | **Low value** |

---

## Total Timeline

| Phase | Duration | Cumulative |
|-------|----------|------------|
| Phase 1 (Quick wins) | 2 months | 2 months |
| Phase 2 (Role chains) | 2 months | 4 months |
| Phase 3 (Datatypes) | 2.5 months | 6.5 months |
| Phase 4 (Keys) | 0.5 months | 7 months |
| Testing & Integration | 1 month | 8 months |

**Total: 8 months full-time work**

---

## Alternative: Minimal Datatype Support (3 months total)

If datatypes are too complex, a practical alternative:

**Include:**
- ✅ Transitive roles
- ✅ Qualified cardinality  
- ✅ Nominals
- ✅ Role chains
- ⚠️ **Minimal datatypes**: xsd:string, xsd:integer only, no facets

**Exclude:**
- ❌ Full datatype facet reasoning
- ❌ Datatype restrictions
- ❌ Keys

**Timeline:** 3 months  
**Result:** SROIQ (without D) - covers 90% of real-world ontologies

---

## Recommendation

### Option A: Phased Implementation (Recommended)

**Immediate (publish current ALC/SHOIQ):**
- Submit current paper as ALC/SHOIQ reasoner
- Honest scope: "pathway to full OWL2 DL"

**Post-publication:**
- Phase 1 (3 months): Quick wins (transitive, nominals, qualified cardinality)
- Update paper: "Now supports SROIQ" (without D)

**Future work:**
- Phase 2-3 (6 months): Role chains + datatypes
- Final paper: "Full OWL2 DL (SROIQ(D))"

### Option B: Delay Publication (Not Recommended)

Wait 8 months for full OWL2 DL before publishing.

**Risk:**
- Competitors may publish similar work
- No current validation
- Delayed impact

---

## Technical Challenges

### 1. Datatype Oracle
OWL2 DL requires a datatype "oracle" that can answer:
- Is this literal in this datatype?
- Are these two datatypes disjoint?
- Is this facet restriction satisfiable?

**Solutions:**
- Use Apache Jena's datatype handling (port from Java)
- Implement custom type system
- Integrate with XSD library

### 2. Role Chain Expansion
Property chains can create infinite loops:
```
R ∘ S ⊑ R  (dangerous - can expand forever)
```

**Solutions:**
- Preprocessing to detect problematic chains
- Cycle detection in expansion
- Limit expansion depth

### 3. Nominal Blocking
Standard ALC blocking doesn't work with nominals:
```
{a} ⊓ ∃R.{b}  - nominals need special handling
```

**Solutions:**
- Modified blocking conditions
- Nominal-aware clash detection
- Dynamic individual creation

---

## Testing Strategy

### OWL2 Test Suites
1. **OWL 2 Test Cases** (W3C) - ~1000 tests
2. **ORE 2015 Benchmarks** - real ontologies
3. **BioPortal** - large real-world ontologies
4. **Generated stress tests** - synthetic corner cases

### Minimum Viable Tests
For each new feature:
- 10 unit tests
- 5 integration tests
- 2 real ontology tests

---

## Conclusion

### Can we make it fully support OWL2 DL?
**Yes, but:**
- 6-8 months full-time work
- Datatypes are the hardest part (2+ months alone)
- Significant complexity increase
- Current ALC/SHOIQ is already publishable

### Recommended Path
1. **Now:** Publish as ALC/SHOIQ (current state)
2. **Phase 1:** Add transitive + nominals + qualified cardinality (3 months)
3. **Update:** Publish SROIQ extension  
4. **Phase 2:** Add datatypes (4-6 months)
5. **Final:** Full SROIQ(D) support

This gives:
- ✅ Early publication and impact
- ✅ Validated approach
- ✅ Gradual complexity increase
- ✅ Multiple publication points

**Bottom line:** Full OWL2 DL is achievable but requires 6-8 months. The current ALC/SHOIQ implementation is a solid foundation and worth publishing now.
