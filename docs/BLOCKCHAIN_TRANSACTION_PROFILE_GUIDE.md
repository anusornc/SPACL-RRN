# Blockchain Transaction Reasoning Profile Guide

Last updated: 2026-02-17
Scope: consortium/permissioned blockchain networks using Tableauxx as semantic validation and reasoning core.

## 1. Objective

Define a production-ready approach for:
- high-throughput transaction ingestion,
- semantic interoperability across known participants,
- deterministic traceability from retail/customer back to source,
- profile-aware reasoning with strict governance.

This guide is intended for networks similar to:
- UHT milk supply chain,
- pharmaceutical provenance/serialization networks,
- other industry consortia with shared ontology standards.

## 2. Core Principle

Do not parse large `.owl` files in the hot transaction path.

Use:
- `compile once` per ontology version,
- `reuse many times` via `.owlbin`,
- process incoming transactions as event facts against preloaded ontology packs.

## 3. OWL2 Profile Recap for Operations

- `EL`: best for large taxonomies and low-latency classification-style reasoning.
- `QL`: best for query answering over large data stores.
- `RL`: best for rule-oriented inference and policy checks.
- `OWL 2 DL`: full expressiveness, highest cost; use as controlled fallback.

Operational recommendation:
- online path: `EL/RL-friendly subset`,
- offline or exceptional path: `DL-heavy` checks.

## 4. PROV-O and EPCIS in This Architecture

### 4.1 PROV-O

- Good model for provenance chains (`Entity`, `Activity`, `Agent`).
- Practical for cross-participant traceability when constrained to production-safe constructs.
- Useful for audit trails and historical causality.

### 4.2 GS1 EPCIS

- Primary transaction payload model (event-centric).
- Works as ABox/fact input stream in reasoning pipeline.
- Pair with:
  - JSON Schema or SHACL for shape validation,
  - ontology-based rules for semantic constraints.

## 5. Reference Architecture

### 5.1 Control Plane (Governance and Standards)

Defines network-wide semantic standards:
- Ontology Pack registry,
- profile policy (`EL-first`, allowed RL rules, DL fallback policy),
- version lifecycle and migration windows,
- compliance gates for participant onboarding.

### 5.2 Data Plane (Transaction Execution)

Per transaction:
1. Verify signature/participant authorization.
2. Validate payload shape (schema/SHACL).
3. Resolve `ontology_version` or `ontology_hash`.
4. Load matching precompiled `.owlbin` from local cache/store.
5. Apply reasoning checks.
6. Commit transaction and emit provenance links.

### 5.3 Storage Plane

- On-chain: minimal trusted state (hashes, pointers, signatures, key claims).
- Off-chain KG: full graph, rich query, analytics.
- Link on-chain/off-chain through content hashes and verifiable references.

## 6. Ontology Pack Specification

Each network should publish immutable ontology packs:

Required metadata:
- `pack_id`: logical name (e.g., `uht-core`, `pharma-core`),
- `version`: semantic version or epoch (`v1`, `2026.02`),
- `ontology_hash`: canonical digest of source ontology set,
- `owlbin_hash`: digest of compiled artifact,
- `profile_policy`: allowed profile subset for online path,
- `created_at`, `approved_by`, `approval_txid`.

Suggested package contents:
- source files (`.owl`, `.ttl`, `.rdf`, `.jsonld`),
- compiled artifact (`.owlbin`),
- schema assets (JSON Schema/SHACL),
- migration notes (`vN -> vN+1` constraints),
- conformance report artifact.

## 7. Transaction Envelope Specification (Recommended)

All participants submit a normalized envelope to prevent semantic ambiguity.

Machine-readable schema:
- `specs/blockchain/schemas/transaction-envelope.schema.json`
- Example: `specs/blockchain/examples/transaction-envelope.example.json`

### 7.1 Envelope Fields

- `tx_id`: unique transaction id.
- `network_id`: consortium network identifier.
- `participant_id`: authenticated sender identity.
- `industry_domain`: e.g., `uht`, `pharma`.
- `ontology_pack_id`: semantic standard id.
- `ontology_version`: required ontology version.
- `ontology_hash`: optional strict pin (recommended).
- `event_type`: EPCIS event type.
- `event_payload`: normalized EPCIS/provenance payload.
- `event_schema_version`: payload schema version.
- `timestamp_utc`: canonical event or submit time.
- `prev_event_refs`: optional prior event links.
- `signature`: participant signature.

### 7.2 Example Envelope (JSON)

```json
{
  "tx_id": "tx-01JH3Y4Y8K7GQ2R0N1",
  "network_id": "uht-th-consortium",
  "participant_id": "retail-001",
  "industry_domain": "uht",
  "ontology_pack_id": "uht-core",
  "ontology_version": "v1.2.0",
  "ontology_hash": "sha256:ab12...",
  "event_type": "ObjectEvent",
  "event_schema_version": "epcis-2.0.0",
  "timestamp_utc": "2026-02-17T08:11:25Z",
  "prev_event_refs": ["tx-01JH3Y4X..."],
  "event_payload": {
    "bizStep": "shipping",
    "epcList": ["urn:epc:id:sgtin:..."],
    "readPoint": "urn:epc:id:sgln:..."
  },
  "signature": "base64:..."
}
```

### 7.3 Validation Contract

Reject transaction if any check fails:
- envelope signature invalid,
- unauthorized participant for declared role,
- ontology pack/version not approved,
- payload shape invalid,
- semantic constraints violated.

## 8. `.owlbin` Lifecycle

### 8.1 Build Phase (One-Time per Version)

1. Collect ontology sources for target pack.
2. Run conformance/profile checks.
3. Compile to `.owlbin`.
4. Record hashes and sign release metadata.
5. Publish artifact to all validator nodes.

### 8.2 Runtime Phase (Many Transactions)

- Resolve ontology version/hash from transaction.
- If cache hit: load `.owlbin` immediately.
- If cache miss:
  - fetch trusted artifact by hash,
  - verify hash and signature,
  - store and activate.

### 8.3 Rebuild Trigger Conditions

Recompile only when:
- ontology version changes,
- artifact integrity check fails,
- parser/runtime compatibility version changes.

Do not rebuild per transaction.

## 9. UHT Traceability Walkthrough

Participants:
- Farmer,
- UHT Manufacturer,
- Logistics,
- Retail,
- Customer (query role).

Event chain example:
1. Farmer records raw milk batch creation.
2. Manufacturer records processing and packaging.
3. Logistics records transfer and custody updates.
4. Retail records receiving and shelf placement.
5. Customer scans product and queries full provenance path.

How reasoning is used:
- Validate role-step consistency (who may emit which event type).
- Validate temporal and custody constraints.
- Ensure trace graph continuity (no broken chain).
- Materialize provenance links for fast customer trace queries.

## 10. Performance and Fairness in Benchmarking

For fair and actionable benchmarking, publish three metrics:

1. `E2E cold wall time`
- includes startup, load, parse, reasoning, output.
- best for production SLO planning.

2. `Parse-only`
- isolate parser/load bottleneck.

3. `Reason-only warm`
- run after ontology is loaded/prepared.

Note:
- Different reasoners expose different internal timers.
- Cross-tool comparison should use normalized wall clock methodology.

## 11. Current Tableauxx Implications

- Parser/load remains the dominant cost on very large ontologies.
- Reasoning core on EL-like workloads is relatively cheap.
- Therefore, engineering priority is parser and ingestion pipeline optimization.

## 12. Security and Trust Requirements

- Sign ontology pack manifests.
- Pin transactions to approved ontology versions/hashes.
- Enforce deterministic parser/reasoning configuration in validator nodes.
- Log all semantic validation outcomes for audit.

## 13. Governance Model (Minimum)

Required governance procedures:
- ontology change proposal process,
- review and approval committee,
- scheduled activation block/time for new version,
- rollback policy and emergency freeze policy.

## 14. Implementation Checklist

Phase A: Standards and Contracts
- Define canonical transaction envelope.
- Define ontology pack metadata schema.
- Define profile policy for online/offline path.

Phase B: Runtime Integration
- Implement ontology resolver by version/hash.
- Implement `.owlbin` cache with integrity verification.
- Implement shape and semantic validation pipeline.

Phase C: Operations and Observability
- Track parse, reason, and wall metrics separately.
- Publish per-participant rejection reasons.
- Add migration dashboards and version adoption tracking.

## 15. Non-Goals

- Do not run full DL-heavy reasoning on every transaction.
- Do not allow arbitrary participant-provided ontology in hot path.
- Do not couple customer query latency to large ontology recompilation.

## 16. References

- W3C PROV-O: `https://www.w3.org/TR/prov-o/`
- GS1 EPCIS artifacts: `https://ref.gs1.org/standards/epcis/2.0.0/artefacts`
- GS1 EPCIS overview: `https://www.gs1.org/standards/epcis`
- Ontology pack manifest schema: `specs/blockchain/schemas/ontology-pack-manifest.schema.json`
- Ontology pack manifest example: `specs/blockchain/examples/ontology-pack-manifest.example.json`
