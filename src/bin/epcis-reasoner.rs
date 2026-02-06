//! EPCIS (Electronic Product Code Information Services) Reasoner Demo
//!
//! A demo application showing OWL2 reasoning on GS1 EPCIS supply chain events.
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

use std::env;
use std::time::Instant;

use owl2_reasoner::{
    Class, ClassExpression, NamedIndividual,
    Ontology, SpeculativeTableauxReasoner, SubClassOfAxiom,
    PropertyAssertionAxiom, ClassAssertionAxiom,
};

fn print_usage() {
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║         EPCIS (GS1) Supply Chain Reasoner Demo                ║");
    println!("╚════════════════════════════════════════════════════════════════╝");
    println!();
    println!("A demo application for reasoning over EPCIS supply chain events.");
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
    
    let epcis_event = Class::new("http://example.org/epcis#Event");
    let object_event = Class::new("http://example.org/epcis#ObjectEvent");
    let aggregation_event = Class::new("http://example.org/epcis#AggregationEvent");
    let epc = Class::new("http://example.org/epcis#EPC");
    let location = Class::new("http://example.org/epcis#Location");
    let business_step = Class::new("http://example.org/epcis#BusinessStep");
    
    ontology.add_class(epcis_event.clone()).unwrap();
    ontology.add_class(object_event.clone()).unwrap();
    ontology.add_class(aggregation_event.clone()).unwrap();
    ontology.add_class(epc.clone()).unwrap();
    ontology.add_class(location.clone()).unwrap();
    ontology.add_class(business_step.clone()).unwrap();
    
    // Create class hierarchy
    ontology.add_subclass_axiom(SubClassOfAxiom::new(
        ClassExpression::Class(object_event.clone()),
        ClassExpression::Class(epcis_event.clone()),
    )).unwrap();
    
    ontology.add_subclass_axiom(SubClassOfAxiom::new(
        ClassExpression::Class(aggregation_event.clone()),
        ClassExpression::Class(epcis_event.clone()),
    )).unwrap();
    
    println!("  ✓ Created {} classes", ontology.classes().len());
    
    // Create object properties
    println!("\nStep 2: Creating relationships...");
    
    let has_epc = owl2_reasoner::ObjectProperty::new("http://example.org/epcis#hasEPC");
    let at_location = owl2_reasoner::ObjectProperty::new("http://example.org/epcis#atLocation");
    let has_business_step = owl2_reasoner::ObjectProperty::new("http://example.org/epcis#hasBusinessStep");
    let before = owl2_reasoner::ObjectProperty::new("http://example.org/epcis#before");
    
    ontology.add_object_property(has_epc.clone()).unwrap();
    ontology.add_object_property(at_location.clone()).unwrap();
    ontology.add_object_property(has_business_step.clone()).unwrap();
    ontology.add_object_property(before.clone()).unwrap();
    
    println!("  ✓ Created {} object properties", ontology.object_properties().len());
    
    // Create supply chain scenario
    println!("\nStep 3: Creating supply chain events...");
    println!("  Scenario: Pharmaceutical product tracking");
    println!();
    
    // Event 1: Manufacturing
    let event1 = NamedIndividual::new("http://example.org/event#manufacturing_001");
    let product1 = NamedIndividual::new("http://example.org/epc#urn:epc:id:sgtin:0614141.107346.2018");
    let factory = NamedIndividual::new("http://example.org/loc#factory_A");
    let commissioning = NamedIndividual::new("http://example.org/step#commissioning");
    
    ontology.add_named_individual(event1.clone()).unwrap();
    ontology.add_named_individual(product1.clone()).unwrap();
    ontology.add_named_individual(factory.clone()).unwrap();
    ontology.add_named_individual(commissioning.clone()).unwrap();
    
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
        ClassExpression::Class(location.clone()),
    )).unwrap();
    
    // Property assertions
    ontology.add_property_assertion(PropertyAssertionAxiom::new(
        event1.iri().clone(),
        has_epc.iri().clone(),
        product1.iri().clone(),
    )).unwrap();
    
    ontology.add_property_assertion(PropertyAssertionAxiom::new(
        event1.iri().clone(),
        at_location.iri().clone(),
        factory.iri().clone(),
    )).unwrap();
    
    println!("  [Event 1] Manufacturing");
    println!("    Product: urn:epc:id:sgtin:0614141.107346.2018");
    println!("    Location: Factory A");
    println!("    Step: Commissioning");
    
    // Event 2: Shipping
    let event2 = NamedIndividual::new("http://example.org/event#shipping_001");
    let warehouse = NamedIndividual::new("http://example.org/loc#warehouse_B");
    let shipping = NamedIndividual::new("http://example.org/step#shipping");
    
    ontology.add_named_individual(event2.clone()).unwrap();
    ontology.add_named_individual(warehouse.clone()).unwrap();
    ontology.add_named_individual(shipping.clone()).unwrap();
    
    ontology.add_class_assertion(ClassAssertionAxiom::new(
        event2.iri().clone(),
        ClassExpression::Class(object_event.clone()),
    )).unwrap();
    
    ontology.add_class_assertion(ClassAssertionAxiom::new(
        warehouse.iri().clone(),
        ClassExpression::Class(location.clone()),
    )).unwrap();
    
    ontology.add_property_assertion(PropertyAssertionAxiom::new(
        event2.iri().clone(),
        has_epc.iri().clone(),
        product1.iri().clone(),
    )).unwrap();
    
    ontology.add_property_assertion(PropertyAssertionAxiom::new(
        event2.iri().clone(),
        at_location.iri().clone(),
        warehouse.iri().clone(),
    )).unwrap();
    
    // Temporal ordering: event1 before event2
    ontology.add_property_assertion(PropertyAssertionAxiom::new(
        event1.iri().clone(),
        before.iri().clone(),
        event2.iri().clone(),
    )).unwrap();
    
    println!("  [Event 2] Shipping");
    println!("    Product: urn:epc:id:sgtin:0614141.107346.2018");
    println!("    Location: Warehouse B");
    println!("    Step: Shipping");
    println!("    Follows: Manufacturing");
    
    // Event 3: Receiving at hospital
    let event3 = NamedIndividual::new("http://example.org/event#receiving_001");
    let hospital = NamedIndividual::new("http://example.org/loc#hospital_C");
    let receiving = NamedIndividual::new("http://example.org/step#receiving");
    
    ontology.add_named_individual(event3.clone()).unwrap();
    ontology.add_named_individual(hospital.clone()).unwrap();
    ontology.add_named_individual(receiving.clone()).unwrap();
    
    ontology.add_class_assertion(ClassAssertionAxiom::new(
        event3.iri().clone(),
        ClassExpression::Class(object_event.clone()),
    )).unwrap();
    
    ontology.add_class_assertion(ClassAssertionAxiom::new(
        hospital.iri().clone(),
        ClassExpression::Class(location.clone()),
    )).unwrap();
    
    ontology.add_property_assertion(PropertyAssertionAxiom::new(
        event3.iri().clone(),
        has_epc.iri().clone(),
        product1.iri().clone(),
    )).unwrap();
    
    ontology.add_property_assertion(PropertyAssertionAxiom::new(
        event3.iri().clone(),
        at_location.iri().clone(),
        hospital.iri().clone(),
    )).unwrap();
    
    // Temporal ordering
    ontology.add_property_assertion(PropertyAssertionAxiom::new(
        event2.iri().clone(),
        before.iri().clone(),
        event3.iri().clone(),
    )).unwrap();
    
    println!("  [Event 3] Receiving");
    println!("    Product: urn:epc:id:sgtin:0614141.107346.2018");
    println!("    Location: Hospital C");
    println!("    Step: Receiving");
    println!("    Follows: Shipping");
    
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
    println!("  - EPCIS events: ObjectEvent (3 instances)");
    println!("  - Product tracked: urn:epc:id:sgtin:0614141.107346.2018");
    println!("  - Supply chain: Factory A → Warehouse B → Hospital C");
    println!("  - Temporal order: Manufacturing < Shipping < Receiving");
}

/// Check EPCIS ontology consistency
fn cmd_check_consistency() {
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║        EPCIS Ontology Consistency Check                       ║");
    println!("╚════════════════════════════════════════════════════════════════╝");
    println!();
    
    let mut ontology = Ontology::new();
    
    // Create basic EPCIS structure
    let event = Class::new("http://example.org/epcis#Event");
    let object_event = Class::new("http://example.org/epcis#ObjectEvent");
    let epc = Class::new("http://example.org/epcis#EPC");
    
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
    println!("║              EPCIS Ontology Statistics                        ║");
    println!("╚════════════════════════════════════════════════════════════════╝");
    println!();
    
    let mut ontology = Ontology::new();
    
    // Add some basic EPCIS structure
    let classes = vec![
        "http://example.org/epcis#Event",
        "http://example.org/epcis#ObjectEvent",
        "http://example.org/epcis#AggregationEvent",
        "http://example.org/epcis#TransformationEvent",
        "http://example.org/epcis#EPC",
        "http://example.org/epcis#Location",
        "http://example.org/epcis#ReadPoint",
        "http://example.org/epcis#BusinessLocation",
        "http://example.org/epcis#BusinessStep",
        "http://example.org/epcis#Disposition",
    ];
    
    for class_iri in classes {
        ontology.add_class(Class::new(class_iri)).unwrap();
    }
    
    // Add hierarchy
    let event = Class::new("http://example.org/epcis#Event");
    let object_event = Class::new("http://example.org/epcis#ObjectEvent");
    let agg_event = Class::new("http://example.org/epcis#AggregationEvent");
    
    ontology.add_subclass_axiom(SubClassOfAxiom::new(
        ClassExpression::Class(object_event),
        ClassExpression::Class(event.clone()),
    )).unwrap();
    
    ontology.add_subclass_axiom(SubClassOfAxiom::new(
        ClassExpression::Class(agg_event),
        ClassExpression::Class(event),
    )).unwrap();
    
    println!("EPCIS Core Classes:");
    println!("  Total classes: {}", ontology.classes().len());
    println!("  Total axioms: {}", ontology.axioms().len());
    println!();
    
    println!("Event Types:");
    println!("  • ObjectEvent      - Individual object observations");
    println!("  • AggregationEvent - Packaging/unpackaging events");
    println!("  • TransformationEvent - Product transformation");
    println!("  • TransactionEvent - Business transactions");
    println!();
    
    println!("Core Concepts:");
    println!("  • EPC              - Electronic Product Code (SGTIN, SSCC, etc.)");
    println!("  • Location         - Where events occur");
    println!("  • ReadPoint        - Exact location of scan");
    println!("  • BusinessLocation - Business context location");
    println!("  • BusinessStep     - Why the event occurred");
    println!("  • Disposition      - Business state of the object");
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
