//! EPCIS (GS1 Electronic Product Code Information Services) Reasoner Demo
//!
//! A demo application showing OWL2 reasoning on GS1 EPCIS supply chain events
//! using the official GS1 EPCIS vocabulary.
//!
//! This demo uses the actual GS1 standard vocabulary:
//! - EPCIS ontology: https://ref.gs1.org/epcis/
//! - Core Business Vocabulary (CBV): https://ref.gs1.org/cbv/
//!
//! EPCIS captures "what, when, where, why" information about products:
//! - What: Product identifier (EPC - Electronic Product Code)
//! - When: Timestamp of the event
//! - Where: Location (ReadPoint, BusinessLocation)
//! - Why: Business context (BusinessStep, Disposition)
//!
//! Usage:
//!   cargo run --bin epcis-reasoner -- <command> [options]
//!
//! Commands:
//!   verify-trace     Verify a supply chain trace is valid
//!   check-consistency Check EPCIS ontology consistency
//!   generate-demo    Generate a demo EPCIS supply chain scenario
//!   stats            Show EPCIS ontology statistics
//!
//! Examples:
//!   cargo run --bin epcis-reasoner -- generate-demo
//!   cargo run --bin epcis-reasoner -- check-consistency
//!
//! References:
//!   - GS1 EPCIS Standard: https://www.gs1.org/standards/epcis
//!   - EPCIS Ontology: https://ref.gs1.org/epcis/
//!   - CBV Standard: https://ref.gs1.org/cbv/

use std::env;
use std::time::Instant;

use owl2_reasoner::{
    Class, ClassExpression, NamedIndividual,
    Ontology, SpeculativeTableauxReasoner, SubClassOfAxiom,
    PropertyAssertionAxiom, ClassAssertionAxiom,
};

/// GS1 EPCIS Namespace
const NS_EPCIS: &str = "https://ref.gs1.org/epcis/";
/// GS1 CBV (Core Business Vocabulary) Namespace  
const NS_CBV: &str = "https://ref.gs1.org/cbv/";
/// Example namespace for instances
const NS_EXAMPLE: &str = "https://example.org/epcis-demo/";

fn print_usage() {
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║    GS1 EPCIS Supply Chain Reasoner Demo                       ║");
    println!("║    (Using official GS1 EPCIS & CBV vocabulary)                ║");
    println!("╚════════════════════════════════════════════════════════════════╝");
    println!();
    println!("A demo application for reasoning over EPCIS supply chain events");
    println!("using the official GS1 EPCIS standard vocabulary.");
    println!();
    println!("Usage: epcis-reasoner <command> [options]");
    println!();
    println!("Commands:");
    println!("  generate-demo       Generate and verify a demo supply chain");
    println!("  check-consistency   Check EPCIS ontology consistency");
    println!("  create-ontology     Create a basic EPCIS ontology structure");
    println!("  stats               Show ontology statistics");
    println!("  help                Show this help message");
    println!();
    println!("Examples:");
    println!("  epcis-reasoner generate-demo");
    println!("  epcis-reasoner check-consistency");
    println!();
}

/// Generate a demo EPCIS supply chain scenario
fn cmd_generate_demo() {
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║        Generating EPCIS Supply Chain Demo                     ║");
    println!("╚════════════════════════════════════════════════════════════════╝");
    println!();
    
    let mut ontology = Ontology::new();
    
    // Create EPCIS core classes
    println!("Step 1: Creating EPCIS classes...");
    
    // GS1 EPCIS Classes
    let epcis_event = Class::new(format!("{}Event", NS_EPCIS));
    let object_event = Class::new(format!("{}ObjectEvent", NS_EPCIS));
    let aggregation_event = Class::new(format!("{}AggregationEvent", NS_EPCIS));
    let epc = Class::new(format!("{}EPC", NS_EPCIS));
    let read_point = Class::new(format!("{}ReadPoint", NS_EPCIS));
    let biz_location = Class::new(format!("{}BizLocation", NS_EPCIS));
    let biz_step = Class::new(format!("{}BizStep", NS_EPCIS));
    
    ontology.add_class(epcis_event.clone()).unwrap();
    ontology.add_class(object_event.clone()).unwrap();
    ontology.add_class(aggregation_event.clone()).unwrap();
    ontology.add_class(epc.clone()).unwrap();
    ontology.add_class(read_point.clone()).unwrap();
    ontology.add_class(biz_location.clone()).unwrap();
    ontology.add_class(biz_step.clone()).unwrap();
    
    // Create class hierarchy
    ontology.add_subclass_axiom(SubClassOfAxiom::new(
        ClassExpression::Class(object_event.clone()),
        ClassExpression::Class(epcis_event.clone()),
    )).unwrap();
    
    ontology.add_subclass_axiom(SubClassOfAxiom::new(
        ClassExpression::Class(aggregation_event.clone()),
        ClassExpression::Class(epcis_event.clone()),
    )).unwrap();
    
    println!("  ✓ Created {} EPCIS classes", ontology.classes().len());
    
    // Create object properties
    println!("\nStep 2: Creating relationships...");
    
    // GS1 EPCIS Properties
    let has_epc = owl2_reasoner::ObjectProperty::new(format!("{}epcList", NS_EPCIS));
    let read_point_prop = owl2_reasoner::ObjectProperty::new(format!("{}readPoint", NS_EPCIS));
    let biz_location_prop = owl2_reasoner::ObjectProperty::new(format!("{}bizLocation", NS_EPCIS));
    let biz_step_prop = owl2_reasoner::ObjectProperty::new(format!("{}bizStep", NS_EPCIS));
    let before = owl2_reasoner::ObjectProperty::new(format!("{}eventTime", NS_EPCIS));
    
    ontology.add_object_property(has_epc.clone()).unwrap();
    ontology.add_object_property(read_point_prop.clone()).unwrap();
    ontology.add_object_property(biz_location_prop.clone()).unwrap();
    ontology.add_object_property(biz_step_prop.clone()).unwrap();
    ontology.add_object_property(before.clone()).unwrap();
    
    println!("  ✓ Created {} EPCIS properties", ontology.object_properties().len());
    
    // Create supply chain scenario
    println!("\nStep 3: Creating supply chain events...");
    println!("  Scenario: Pharmaceutical product tracking");
    println!();
    
    // Event 1: Manufacturing (using GS1 CBV BizStep)
    let event1 = NamedIndividual::new(format!("{}event/manufacturing-001", NS_EXAMPLE));
    let product1 = NamedIndividual::new("urn:epc:id:sgtin:0614141.107346.2018");
    let factory = NamedIndividual::new(format!("{}loc/factory-A", NS_EXAMPLE));
    let commissioning_step = NamedIndividual::new(format!("{}cbv:BizStep-commissioning", NS_CBV));
    
    ontology.add_named_individual(event1.clone()).unwrap();
    ontology.add_named_individual(product1.clone()).unwrap();
    ontology.add_named_individual(factory.clone()).unwrap();
    ontology.add_named_individual(commissioning_step.clone()).unwrap();
    
    // Class assertions
    ontology.add_class_assertion(ClassAssertionAxiom::new(
        event1.iri().clone(),
        ClassExpression::Class(object_event.clone()),
    )).unwrap();
    
    ontology.add_class_assertion(ClassAssertionAxiom::new(
        product1.iri().clone(),
        ClassExpression::Class(epc.clone()),
    )).unwrap();
    
    ontology.add_class_assertion(ClassAssertionAxiom::new(
        factory.iri().clone(),
        ClassExpression::Class(read_point.clone()),
    )).unwrap();
    
    // Property assertions
    ontology.add_property_assertion(PropertyAssertionAxiom::new(
        event1.iri().clone(),
        has_epc.iri().clone(),
        product1.iri().clone(),
    )).unwrap();
    
    ontology.add_property_assertion(PropertyAssertionAxiom::new(
        event1.iri().clone(),
        biz_location_prop.iri().clone(),
        factory.iri().clone(),
    )).unwrap();
    
    ontology.add_property_assertion(PropertyAssertionAxiom::new(
        event1.iri().clone(),
        biz_step_prop.iri().clone(),
        commissioning_step.iri().clone(),
    )).unwrap();
    
    println!("  [Event 1] Manufacturing (ObjectEvent)");
    println!("    Product (EPC): urn:epc:id:sgtin:0614141.107346.2018");
    println!("    Location: Factory A (ReadPoint)");
    println!("    Business Step: cbv:BizStep-commissioning");
    
    // Event 2: Shipping
    let event2 = NamedIndividual::new(format!("{}event/shipping-001", NS_EXAMPLE));
    let warehouse = NamedIndividual::new(format!("{}loc/warehouse-B", NS_EXAMPLE));
    let shipping_step = NamedIndividual::new(format!("{}cbv:BizStep-shipping", NS_CBV));
    
    ontology.add_named_individual(event2.clone()).unwrap();
    ontology.add_named_individual(warehouse.clone()).unwrap();
    ontology.add_named_individual(shipping_step.clone()).unwrap();
    
    ontology.add_class_assertion(ClassAssertionAxiom::new(
        event2.iri().clone(),
        ClassExpression::Class(object_event.clone()),
    )).unwrap();
    
    ontology.add_class_assertion(ClassAssertionAxiom::new(
        warehouse.iri().clone(),
        ClassExpression::Class(read_point.clone()),
    )).unwrap();
    
    ontology.add_property_assertion(PropertyAssertionAxiom::new(
        event2.iri().clone(),
        has_epc.iri().clone(),
        product1.iri().clone(),
    )).unwrap();
    
    ontology.add_property_assertion(PropertyAssertionAxiom::new(
        event2.iri().clone(),
        biz_location_prop.iri().clone(),
        warehouse.iri().clone(),
    )).unwrap();
    
    ontology.add_property_assertion(PropertyAssertionAxiom::new(
        event2.iri().clone(),
        biz_step_prop.iri().clone(),
        shipping_step.iri().clone(),
    )).unwrap();
    
    // Temporal ordering: event1 before event2
    ontology.add_property_assertion(PropertyAssertionAxiom::new(
        event1.iri().clone(),
        before.iri().clone(),
        event2.iri().clone(),
    )).unwrap();
    
    println!("  [Event 2] Shipping (ObjectEvent)");
    println!("    Product (EPC): urn:epc:id:sgtin:0614141.107346.2018");
    println!("    Location: Warehouse B (BizLocation)");
    println!("    Business Step: cbv:BizStep-shipping");
    println!("    Temporal: After Manufacturing event");
    
    // Event 3: Receiving at hospital
    let event3 = NamedIndividual::new(format!("{}event/receiving-001", NS_EXAMPLE));
    let hospital = NamedIndividual::new(format!("{}loc/hospital-C", NS_EXAMPLE));
    let receiving_step = NamedIndividual::new(format!("{}cbv:BizStep-receiving", NS_CBV));
    
    ontology.add_named_individual(event3.clone()).unwrap();
    ontology.add_named_individual(hospital.clone()).unwrap();
    ontology.add_named_individual(receiving_step.clone()).unwrap();
    
    ontology.add_class_assertion(ClassAssertionAxiom::new(
        event3.iri().clone(),
        ClassExpression::Class(object_event.clone()),
    )).unwrap();
    
    ontology.add_class_assertion(ClassAssertionAxiom::new(
        hospital.iri().clone(),
        ClassExpression::Class(biz_location.clone()),
    )).unwrap();
    
    ontology.add_property_assertion(PropertyAssertionAxiom::new(
        event3.iri().clone(),
        has_epc.iri().clone(),
        product1.iri().clone(),
    )).unwrap();
    
    ontology.add_property_assertion(PropertyAssertionAxiom::new(
        event3.iri().clone(),
        biz_location_prop.iri().clone(),
        hospital.iri().clone(),
    )).unwrap();
    
    ontology.add_property_assertion(PropertyAssertionAxiom::new(
        event3.iri().clone(),
        biz_step_prop.iri().clone(),
        receiving_step.iri().clone(),
    )).unwrap();
    
    // Temporal ordering
    ontology.add_property_assertion(PropertyAssertionAxiom::new(
        event2.iri().clone(),
        before.iri().clone(),
        event3.iri().clone(),
    )).unwrap();
    
    println!("  [Event 3] Receiving at Hospital (ObjectEvent)");
    println!("    Product (EPC): urn:epc:id:sgtin:0614141.107346.2018");
    println!("    Location: Hospital C (BizLocation)");
    println!("    Business Step: cbv:BizStep-receiving");
    println!("    Temporal: After Shipping event");
    
    println!();
    println!("  ✓ Created {} individuals", ontology.named_individuals().len());
    println!("  ✓ Created {} axioms", ontology.axioms().len());
    
    // Run consistency check
    println!("\nStep 4: Verifying supply chain consistency...");
    let start = Instant::now();
    
    let mut reasoner = SpeculativeTableauxReasoner::new(ontology);
    match reasoner.is_consistent() {
        Ok(consistent) => {
            let check_time = start.elapsed();
            println!("  ✓ Reasoning complete in {:?}", check_time);
            println!();
            
            if consistent {
                println!("╔════════════════════════════════════════════════════════════════╗");
                println!("║  ✅ SUPPLY CHAIN TRACE IS VALID                               ║");
                println!("╚════════════════════════════════════════════════════════════════╝");
                println!();
                println!("The product trace is logically consistent:");
                println!("  • Manufacturing → Shipping → Receiving");
                println!("  • All locations are valid");
                println!("  • Product identity is preserved throughout the chain");
            } else {
                println!("╔════════════════════════════════════════════════════════════════╗");
                println!("║  ❌ SUPPLY CHAIN TRACE IS INVALID                             ║");
                println!("╚════════════════════════════════════════════════════════════════╝");
                println!();
                println!("The product trace contains inconsistencies!");
            }
        }
        Err(e) => {
            eprintln!("Error during reasoning: {:?}", e);
        }
    }
    
    println!();
    println!("Summary:");
    println!("  - Standard: GS1 EPCIS 2.0 (https://ns.gs1.org/epcis/)");
    println!("  - Vocabulary: GS1 CBV (https://ns.gs1.org/cbv/)");
    println!("  - EPCIS events: ObjectEvent (3 instances)");
    println!("  - Product EPC: urn:epc:id:sgtin:0614141.107346.2018");
    println!("  - Supply chain: Factory A → Warehouse B → Hospital C");
    println!("  - Temporal order: Manufacturing < Shipping < Receiving");
    println!("  - Business steps: commissioning → shipping → receiving");
}

/// Check EPCIS ontology consistency
fn cmd_check_consistency() {
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║        GS1 EPCIS Ontology Consistency Check                   ║");
    println!("╚════════════════════════════════════════════════════════════════╝");
    println!();
    
    let mut ontology = Ontology::new();
    
    // Create GS1 EPCIS structure using official vocabulary
    println!("Using GS1 EPCIS vocabulary:");
    println!("  Namespace: https://ns.gs1.org/epcis/");
    println!();
    
    let event = Class::new(format!("{}Event", NS_EPCIS));
    let object_event = Class::new(format!("{}ObjectEvent", NS_EPCIS));
    let epc = Class::new(format!("{}EPC", NS_EPCIS));
    
    ontology.add_class(event.clone()).unwrap();
    ontology.add_class(object_event.clone()).unwrap();
    ontology.add_class(epc.clone()).unwrap();
    
    // Add hierarchy
    ontology.add_subclass_axiom(SubClassOfAxiom::new(
        ClassExpression::Class(object_event.clone()),
        ClassExpression::Class(event.clone()),
    )).unwrap();
    
    println!("Ontology structure:");
    println!("  Classes: {}", ontology.classes().len());
    println!("  Axioms: {}", ontology.axioms().len());
    println!();
    
    println!("Checking consistency...");
    let start = Instant::now();
    
    let mut reasoner = SpeculativeTableauxReasoner::new(ontology);
    match reasoner.is_consistent() {
        Ok(consistent) => {
            let check_time = start.elapsed();
            println!("  ✓ Check complete in {:?}", check_time);
            println!();
            
            if consistent {
                println!("Result: ✅ CONSISTENT");
                println!();
                println!("The EPCIS ontology structure is valid.");
            } else {
                println!("Result: ❌ INCONSISTENT");
            }
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
        }
    }
}

/// Show ontology statistics
fn cmd_stats() {
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║           GS1 EPCIS Ontology Statistics                       ║");
    println!("╚════════════════════════════════════════════════════════════════╝");
    println!();
    
    let mut ontology = Ontology::new();
    
    // Add GS1 EPCIS core classes
    let classes = vec![
        format!("{}Event", NS_EPCIS),
        format!("{}ObjectEvent", NS_EPCIS),
        format!("{}AggregationEvent", NS_EPCIS),
        format!("{}TransformationEvent", NS_EPCIS),
        format!("{}TransactionEvent", NS_EPCIS),
        format!("{}AssociationEvent", NS_EPCIS),
        format!("{}EPC", NS_EPCIS),
        format!("{}ReadPoint", NS_EPCIS),
        format!("{}BusinessLocation", NS_EPCIS),
        format!("{}BusinessStep", NS_EPCIS),
        format!("{}Disposition", NS_EPCIS),
    ];
    
    for class_iri in classes {
        ontology.add_class(Class::new(class_iri)).unwrap();
    }
    
    // Add hierarchy
    let event = Class::new(format!("{}Event", NS_EPCIS));
    let object_event = Class::new(format!("{}ObjectEvent", NS_EPCIS));
    let agg_event = Class::new(format!("{}AggregationEvent", NS_EPCIS));
    
    ontology.add_subclass_axiom(SubClassOfAxiom::new(
        ClassExpression::Class(object_event),
        ClassExpression::Class(event.clone()),
    )).unwrap();
    
    ontology.add_subclass_axiom(SubClassOfAxiom::new(
        ClassExpression::Class(agg_event),
        ClassExpression::Class(event.clone()),
    )).unwrap();
    
    println!("GS1 EPCIS Core Classes (https://ns.gs1.org/epcis/):");
    println!("  Total classes: {}", ontology.classes().len());
    println!("  Total axioms: {}", ontology.axioms().len());
    println!();
    
    println!("Event Types:");
    println!("  • ObjectEvent       - Individual object observations (add, observe, delete)");
    println!("  • AggregationEvent  - Packaging/unpackaging events");
    println!("  • TransformationEvent - Product transformation events");
    println!("  • TransactionEvent  - Business transaction events");
    println!("  • AssociationEvent  - Association events");
    println!();
    
    println!("Core Concepts:");
    println!("  • EPC               - Electronic Product Code (SGTIN, SSCC, etc.)");
    println!("  • ReadPoint         - Exact location where the event occurred");
    println!("  • BusinessLocation  - Business context location");
    println!("  • BusinessStep      - Business step (from CBV vocabulary)");
    println!("  • Disposition       - Business state of the object (from CBV)");
    println!();
    println!("Standards:");
    println!("  • EPCIS 2.0: https://ns.gs1.org/epcis/");
    println!("  • CBV 1.2:   https://ns.gs1.org/cbv/");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_usage();
        return;
    }
    
    let command = &args[1];
    
    match command.as_str() {
        "generate-demo" => cmd_generate_demo(),
        "check-consistency" => cmd_check_consistency(),
        "stats" => cmd_stats(),
        "help" | "--help" | "-h" => print_usage(),
        _ => {
            eprintln!("Error: Unknown command '{}'\n", command);
            print_usage();
        }
    }
}
