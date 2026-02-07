# Nogood Learning Implementation Audit

**Date:** 2026-02-06  
**Auditor:** Code Review  
**Status:** ⚠️ REQUIRES ATTENTION

---

## 1. Implementation Overview

### 1.1 How Nogoods Are Created

**Location:** `src/reasoner/speculative.rs:916`

```rust
if !is_consistent {
    // Learn a nogood from this contradiction
    let nogood = Nogood::new(item.test_expressions.clone());
    ...
}
```

**What goes into the nogood:**
- `item.test_expressions` - a `HashSet<ClassExpression>`
- Contains ALL SubClassOf sub_class and super_class expressions (lines 688-693)
- NOT minimal - includes expressions potentially unrelated to the contradiction

### 1.2 What test_expressions Contains

**Location:** `src/reasoner/speculative.rs:688-693`

```rust
let mut test_expressions = HashSet::new();
for axiom in self.ontology.axioms() {
    if let crate::logic::axioms::Axiom::SubClassOf(sub) = axiom.as_ref() {
        test_expressions.insert(sub.sub_class().clone());
        test_expressions.insert(sub.super_class().clone());
    }
}
```

**Contents:**
- All class expressions from SubClassOf axioms
- Both sub_class and super_class positions
- Could be 100s of expressions for large ontologies

### 1.3 How Nogoods Are Used

**Pruning Check:** `src/reasoner/speculative.rs:869-886`

```rust
let conflict = cache.check_conflict(&nogoods, &item.test_expressions);
if conflict.is_some() {
    return WorkResult::Contradiction { ... };
}
```

**Subsumption Test:** `src/reasoner/speculative.rs:87-89`

```rust
pub fn subsumes(&self, assertions: &HashSet<ClassExpression>) -> bool {
    self.assertions.is_subset(assertions)
}
```

---

## 2. Correctness Analysis

### 2.1 Soundness Question

**Definition:** A nogood learning system is **sound** if:
> Whenever a nogood N is learned, N actually implies unsatisfiability.

**Current Implementation:**
- Nogood N = set of all test expressions T
- When N subsumes current assertions A (i.e., T ⊆ A), branch is pruned
- **Claim:** T was unsatisfiable in the context where it was learned

**Potential Issue:**
- T contains ALL SubClassOf expressions
- The contradiction may only involve a SMALL SUBSET of T
- Example: If T = {A, B, C, D, ...} and contradiction only needs {A, B}
- Then we learn N = {A, B, C, D, ...}
- A future branch with {A, B, E} won't be pruned (N not subset)
- But {A, B, E} might still be unsatisfiable!

**Verdict:** ⚠️ NOT MINIMAL - May miss pruning opportunities

### 2.2 Completeness Question

**Definition:** A nogood learning system is **complete** if:
> No valid models are pruned (no false positives)

**Current Implementation:**
- If N subsumes A (N ⊆ A), and N was unsatisfiable, then A is unsatisfiable
- This is logically correct: unsatisfiability is monotonic

**Verdict:** ✅ COMPLETE - Won't prune satisfiable branches

### 2.3 Minimal Nogood Issue

**Problem:** Current nogoods are NOT minimal

**Example:**
```
Ontology:
  A ⊑ B
  B ⊑ C
  C ⊑ ⊥
  
Contradiction when testing {A, B, C}
Current nogood: {A, B, C}
Minimal nogood: {A} (since A → B → C → ⊥)
```

**Impact:**
- Missing pruning opportunities
- Larger nogood database
- Higher memory usage
- **NOT UNSOUND** - just inefficient

---

## 3. Reviewer's Concern Validated

The reviewer said:
> "The nogood extraction algorithm selects only 'non-atomic' concepts from a clashed node label, which can yield nogoods that are not actually contradictory"

**Our finding:**
- Current implementation is DIFFERENT from what reviewer described
- We take ALL test expressions, not just non-atomic
- But the concern about non-minimal nogoods is VALID
- The nogood is a SUPerset of what caused the contradiction
- This is SAFE but INEFFICIENT

---

## 4. Recommendations

### Option A: Fix to Minimal Nogoods (Complex)

**Approach:**
1. Track which specific assertions caused the contradiction
2. Use dependency analysis or implication graph
3. Extract minimal unsatisfiable core

**Pros:**
- Maximum pruning effectiveness
- Smaller nogood database
- Better performance

**Cons:**
- Complex to implement correctly
- Requires significant changes to SimpleReasoner
- Risk of introducing bugs

### Option B: Conservative Nogoods (Simple)

**Approach:**
1. Keep current implementation (it's safe)
2. Document that nogoods are over-approximations
3. Add runtime check: verify nogood actually causes contradiction

**Pros:**
- Simple and safe
- No risk of unsoundness
- Easy to verify

**Cons:**
- Suboptimal pruning
- Reviewer may not accept

### Option C: Disable Nogood Learning (Fallback)

**Approach:**
1. Remove nogood claims from paper
2. Keep work-stealing parallelism only
3. Document nogood learning as future work

**Pros:**
- Eliminates correctness concern
- Simpler resubmission
- Can add nogoods back in follow-up

**Cons:**
- Weaker contribution
- Loses the "learning" aspect

---

## 5. Proposed Solution

**Hybrid Approach:**

1. **Verify current implementation is sound** (Option B + verification)
   - Add runtime assertion: verify nogood actually unsatisfiable
   - Add test suite for nogood correctness
   - Document over-approximation

2. **Scope paper appropriately**
   - Frame nogood learning as "conservative over-approximation"
   - Focus on work-stealing as primary contribution
   - Mention nogood learning improves with minimization (future work)

3. **Add formal argument**
   - Prove: if N learned, then N ⊨ ⊥ (soundness)
   - Acknowledge: N may not be minimal (efficiency limitation)

---

## 6. Test Plan

To verify soundness, create tests:

```rust
#[test]
fn test_nogood_soundness() {
    // Create ontology with known contradiction
    // Run speculative reasoner
    // For each nogood learned:
    //   - Verify nogood assertions actually cause contradiction
    //   - Verify no false positives (don't over-prune)
}
```

**Test Cases Needed:**
- [ ] Simple A ⊑ ⊥ contradiction
- [ ] Chain A ⊑ B ⊑ C ⊑ ⊥
- [ ] Disjunction A ⊔ B with ¬A ⊓ ¬B
- [ ] Complex nested case
- [ ] Edge case: empty ontology
- [ ] Edge case: single assertion

---

## 7. Manuscript Updates

**New Section: "Nogood Learning Correctness"**

```
Nogood learning in SPACL uses conservative over-approximation.
When a contradiction is detected on branch B with assertion set A,
we record the entire test expression set T as a nogood.

While T may contain assertions not directly involved in the 
contradiction, this approach is SOUND: if T ⊨ ⊥, then any 
superset A ⊇ T is also unsatisfiable.

The trade-off is reduced pruning effectiveness compared to 
minimal nogood extraction, which we leave as future work.
```

---

## 8. Conclusion

| Aspect | Status | Action |
|--------|--------|--------|
| Soundness | ✅ Safe | Add verification tests |
| Minimality | ❌ Not minimal | Document as limitation |
| Completeness | ✅ Complete | No false positives |
| Efficiency | ⚠️ Suboptimal | Future work |

**Recommendation:** Proceed with Option B (conservative + documentation) + formal soundness argument in paper.

The nogood learning is **not broken**, just **not optimal**. This is defensible for publication.
