//! OWL2 Reasoner CLI - Demo Application
//!
//! A command-line tool for reasoning with OWL2 ontologies.
//!
//! Usage:
//!   cargo run --bin owl2-reasoner -- <command> [options] <ontology_file>
//!
//! Commands:
//!   check       Check ontology consistency
//!   stats       Show ontology statistics
//!   compare     Compare Sequential vs SPACL performance
//!   classify    Classify ontology (TBox reasoning)
//!
//! Examples:
//!   cargo run --bin owl2-reasoner -- check tests/data/univ-bench.owl
//!   cargo run --bin owl2-reasoner -- stats tests/data/univ-bench.owl
//!   cargo run --bin owl2-reasoner -- compare tests/data/univ-bench.owl

use std::env;
use std::path::Path;
use std::time::Instant;

use owl2_reasoner::{
    Ontology, SimpleReasoner, SpeculativeTableauxReasoner,
    parser::ParserFactory,
};

fn print_usage() {
    println!("OWL2 Reasoner CLI - Demo Application");
    println!();
    println!("Usage: owl2-reasoner <command> [options] <ontology_file>");
    println!();
    println!("Commands:");
    println!("  check <file>       Check ontology consistency");
    println!("  check-auto <file>  Check with automatic reasoner selection");
    println!("  stats <file>       Show ontology statistics");
    println!("  compare <file>     Compare Sequential vs SPACL performance");
    println!("  help               Show this help message");
    println!();
    println!("Options:");
    println!("  -v, --verbose      Verbose output");
    println!();
    println!("Examples:");
    println!("  owl2-reasoner check tests/data/univ-bench.owl");
    println!("  owl2-reasoner check-auto benchmarks/ontologies/other/pato.owl");
    println!("  owl2-reasoner stats tests/data/univ-bench.owl");
    println!("  owl2-reasoner compare tests/data/univ-bench.owl");
}

fn load_ontology(path: &str) -> Result<Ontology, String> {
    let path = Path::new(path);
    if !path.exists() {
        return Err(format!("File not found: {}", path.display()));
    }
    
    println!("Loading ontology: {}", path.display());
    let start = Instant::now();
    
    let content = std::fs::read_to_string(path)
        .map_err(|e| format!("Failed to read file: {}", e))?;
    
    let parser = ParserFactory::auto_detect(&content)
        .ok_or_else(|| "Failed to detect file format".to_string())?;
    
    let ontology = parser.parse_str(&content)
        .map_err(|e| format!("Failed to parse ontology: {:?}", e))?;
    
    let load_time = start.elapsed();
    println!("✓ Loaded in {:?}", load_time);
    
    Ok(ontology)
}

/// Analyze ontology to determine best reasoning strategy
fn analyze_ontology(ontology: &Ontology) -> OntologyAnalysis {
    use owl2_reasoner::logic::axioms::class_expressions::ClassExpression;
    use owl2_reasoner::logic::axioms::Axiom;
    
    let mut disjunction_count = 0;
    let mut class_count = ontology.classes().len();
    let mut subclass_axiom_count = 0;
    let mut complex_axiom_count = 0;
    
    for axiom in ontology.axioms() {
        match axiom.as_ref() {
            Axiom::SubClassOf(_) => {
                subclass_axiom_count += 1;
            }
            Axiom::EquivalentClasses(_) => {
                complex_axiom_count += 1;
                // Note: EquivalentClasses stores class IRIs, not expressions
                // so we don't check for disjunctions here
            }
            Axiom::DisjointClasses(_) => {
                complex_axiom_count += 1;
            }
            _ => {}
        }
    }
    
    // Simple heuristic: if no disjunctions and mostly subclass axioms, it's a hierarchy
    let is_simple_hierarchy = disjunction_count == 0 && complex_axiom_count == 0;
    let is_large = class_count > 5000;
    
    OntologyAnalysis {
        class_count,
        disjunction_count,
        subclass_axiom_count,
        complex_axiom_count,
        is_simple_hierarchy,
        is_large,
        recommended_reasoner: if is_simple_hierarchy && is_large {
            ReasonerChoice::Simple
        } else if disjunction_count > 0 {
            ReasonerChoice::Speculative
        } else {
            ReasonerChoice::Simple
        },
    }
}

fn contains_disjunction(expr: &owl2_reasoner::logic::axioms::class_expressions::ClassExpression) -> bool {
    use owl2_reasoner::logic::axioms::class_expressions::ClassExpression;
    
    match expr {
        ClassExpression::ObjectUnionOf(_) => true,
        ClassExpression::ObjectIntersectionOf(operands) => {
            operands.iter().any(|op| contains_disjunction(op))
        }
        ClassExpression::ObjectComplementOf(inner) => contains_disjunction(inner),
        ClassExpression::ObjectSomeValuesFrom(_, inner) => contains_disjunction(inner),
        ClassExpression::ObjectAllValuesFrom(_, inner) => contains_disjunction(inner),
        _ => false,
    }
}

struct OntologyAnalysis {
    class_count: usize,
    disjunction_count: usize,
    subclass_axiom_count: usize,
    complex_axiom_count: usize,
    is_simple_hierarchy: bool,
    is_large: bool,
    recommended_reasoner: ReasonerChoice,
}

enum ReasonerChoice {
    Simple,
    Speculative,
}

fn cmd_check_auto(args: &[String]) {
    if args.is_empty() {
        eprintln!("Error: No ontology file specified");
        println!("Usage: owl2-reasoner check-auto <ontology_file>");
        return;
    }
    
    let path = &args[0];
    let ontology = match load_ontology(path) {
        Ok(o) => o,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };
    
    // Analyze ontology
    let analysis = analyze_ontology(&ontology);
    
    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║           ONTOLOGY ANALYSIS & REASONER SELECTION             ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();
    println!("Statistics:");
    println!("  Classes:          {}", analysis.class_count);
    println!("  SubClass axioms:  {}", analysis.subclass_axiom_count);
    println!("  Disjunctions:     {}", analysis.disjunction_count);
    println!("  Complex axioms:   {}", analysis.complex_axiom_count);
    println!();
    println!("Characteristics:");
    println!("  Simple hierarchy: {}", if analysis.is_simple_hierarchy { "Yes" } else { "No" });
    println!("  Large ontology:   {}", if analysis.is_large { "Yes" } else { "No" });
    println!();
    
    let start = Instant::now();
    let consistent = match analysis.recommended_reasoner {
        ReasonerChoice::Simple => {
            println!("Selected: SimpleReasoner (optimized for hierarchies)");
            let reasoner = SimpleReasoner::new(ontology);
            reasoner.is_consistent()
        }
        ReasonerChoice::Speculative => {
            println!("Selected: SpeculativeTableauxReasoner (SPACL with parallel disjunction handling)");
            let mut reasoner = SpeculativeTableauxReasoner::new(ontology);
            reasoner.is_consistent()
        }
    };
    
    let check_time = start.elapsed();
    
    match consistent {
        Ok(result) => {
            println!("\n✓ Consistency check complete in {:?}", check_time);
            println!();
            if result {
                println!("Result: ✅ CONSISTENT");
            } else {
                println!("Result: ❌ INCONSISTENT");
            }
        }
        Err(e) => {
            eprintln!("Error during reasoning: {:?}", e);
        }
    }
}

fn cmd_check(args: &[String]) {
    if args.is_empty() {
        eprintln!("Error: No ontology file specified");
        println!("Usage: owl2-reasoner check <ontology_file>");
        return;
    }
    
    let path = &args[0];
    let ontology = match load_ontology(path) {
        Ok(o) => o,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };
    
    println!("\nChecking consistency...");
    let start = Instant::now();
    
    let mut reasoner = SpeculativeTableauxReasoner::new(ontology);
    match reasoner.is_consistent() {
        Ok(consistent) => {
            let check_time = start.elapsed();
            println!("✓ Consistency check complete in {:?}", check_time);
            println!();
            if consistent {
                println!("Result: ✅ CONSISTENT");
            } else {
                println!("Result: ❌ INCONSISTENT");
            }
        }
        Err(e) => {
            eprintln!("Error during reasoning: {:?}", e);
        }
    }
}

fn cmd_stats(args: &[String]) {
    if args.is_empty() {
        eprintln!("Error: No ontology file specified");
        println!("Usage: owl2-reasoner stats <ontology_file>");
        return;
    }
    
    let path = &args[0];
    let ontology = match load_ontology(path) {
        Ok(o) => o,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };
    
    // Also run analysis
    let analysis = analyze_ontology(&ontology);
    
    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║                  ONTOLOGY STATISTICS                         ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();
    println!("Basic Counts:");
    println!("  Classes:             {}", ontology.classes().len());
    println!("  Object Properties:   {}", ontology.object_properties().len());
    println!("  Data Properties:     {}", ontology.data_properties().len());
    println!("  Named Individuals:   {}", ontology.named_individuals().len());
    println!("  Total Axioms:        {}", ontology.axioms().len());
    println!();
    
    println!("Analysis:");
    println!("  Disjunctions:        {}", analysis.disjunction_count);
    println!("  Simple hierarchy:    {}", if analysis.is_simple_hierarchy { "Yes" } else { "No" });
    println!();
    
    // Estimate expressivity
    let expressivity = if analysis.disjunction_count > 0 {
        "ALC (with disjunctions)"
    } else if analysis.complex_axiom_count > 0 {
        "AL (with complex axioms)"
    } else {
        "EL (simple hierarchy)"
    };
    println!("Expressivity: {}", expressivity);
    println!();
    
    println!("Recommended: {}", 
        match analysis.recommended_reasoner {
            ReasonerChoice::Simple => "SimpleReasoner (fast for hierarchies)",
            ReasonerChoice::Speculative => "SpeculativeTableauxReasoner (SPACL for disjunctions)",
        }
    );
}

fn cmd_compare(args: &[String]) {
    if args.is_empty() {
        eprintln!("Error: No ontology file specified");
        println!("Usage: owl2-reasoner compare <ontology_file>");
        return;
    }
    
    let path = &args[0];
    let ontology = match load_ontology(path) {
        Ok(o) => o,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };
    
    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║              REASONER PERFORMANCE COMPARISON                 ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();
    
    // SimpleReasoner
    println!("Running SimpleReasoner...");
    let start = Instant::now();
    let simple_reasoner = SimpleReasoner::new(ontology.clone());
    let simple_result = simple_reasoner.is_consistent();
    let simple_time = start.elapsed();
    println!("  Result: {:?} in {:?}", simple_result, simple_time);
    println!();
    
    // SpeculativeTableauxReasoner
    println!("Running SpeculativeTableauxReasoner (SPACL)...");
    let start = Instant::now();
    let mut speculative_reasoner = SpeculativeTableauxReasoner::new(ontology);
    let speculative_result = speculative_reasoner.is_consistent();
    let speculative_time = start.elapsed();
    println!("  Result: {:?} in {:?}", speculative_result, speculative_time);
    println!();
    
    // Summary
    println!("Summary:");
    println!("  SimpleReasoner:    {:?}", simple_time);
    println!("  SPACL:             {:?}", speculative_time);
    
    let speedup = simple_time.as_secs_f64() / speculative_time.as_secs_f64();
    if speedup > 1.0 {
        println!("  Speedup:           {:.2}x faster (SPACL)", speedup);
    } else {
        println!("  Speedup:           {:.2}x faster (Simple)", 1.0 / speedup);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_usage();
        return;
    }
    
    let command = &args[1];
    let command_args = &args[2..];
    
    match command.as_str() {
        "check" => cmd_check(command_args),
        "check-auto" => cmd_check_auto(command_args),
        "stats" => cmd_stats(command_args),
        "compare" => cmd_compare(command_args),
        "help" | "--help" | "-h" => print_usage(),
        _ => {
            eprintln!("Error: Unknown command '{}'", command);
            print_usage();
        }
    }
}
