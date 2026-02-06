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
    Ontology, SimpleReasoner, SpeculativeTableauxReasoner, OwlReasoner,
    parser::ParserFactory,
};

fn print_usage() {
    println!("OWL2 Reasoner CLI - Demo Application");
    println!();
    println!("Usage: owl2-reasoner <command> [options] <ontology_file>");
    println!();
    println!("Commands:");
    println!("  check <file>       Check ontology consistency");
    println!("  stats <file>       Show ontology statistics");
    println!("  compare <file>     Compare Sequential vs SPACL performance");
    println!("  help               Show this help message");
    println!();
    println!("Options:");
    println!("  -v, --verbose      Verbose output");
    println!();
    println!("Examples:");
    println!("  owl2-reasoner check tests/data/univ-bench.owl");
    println!("  owl2-reasoner stats benchmarks/ontologies/other/pato.owl");
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
    
    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║                  ONTOLOGY STATISTICS                         ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();
    
    let classes = ontology.classes();
    let axioms = ontology.axioms();
    
    println!("Classes:        {}", classes.len());
    println!("Axioms:         {}", axioms.len());
    
    // Count axiom types
    use owl2_reasoner::logic::axioms::Axiom;
    let mut subclass_count = 0;
    let mut disjoint_count = 0;
    let mut equiv_count = 0;
    
    for axiom in axioms {
        match axiom.as_ref() {
            Axiom::SubClassOf(_) => subclass_count += 1,
            Axiom::DisjointClasses(_) => disjoint_count += 1,
            Axiom::EquivalentClasses(_) => equiv_count += 1,
            _ => {}
        }
    }
    
    println!("  SubClassOf:   {}", subclass_count);
    println!("  Disjoint:     {}", disjoint_count);
    println!("  Equivalent:   {}", equiv_count);
    
    // Try to detect expressivity
    println!();
    println!("Expressivity Features:");
    let has_unions = axioms.iter().any(|ax| {
        format!("{:?}", ax).contains("Union")
    });
    let has_intersections = axioms.iter().any(|ax| {
        format!("{:?}", ax).contains("Intersection")
    });
    
    println!("  Unions:         {}", if has_unions { "✓ Yes" } else { "✗ No" });
    println!("  Intersections:  {}", if has_intersections { "✓ Yes" } else { "✗ No" });
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
    println!("║           SEQUENTIAL VS SPACL COMPARISON                     ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();
    
    // Sequential
    println!("Running SEQUENTIAL reasoner...");
    let start = Instant::now();
    let seq_reasoner = SimpleReasoner::new(ontology.clone());
    let seq_result = seq_reasoner.is_consistent();
    let seq_time = start.elapsed();
    println!("✓ Sequential: {:?}", seq_time);
    
    // SPACL
    println!();
    println!("Running SPACL (parallel) reasoner...");
    let start = Instant::now();
    let mut spacl_reasoner = SpeculativeTableauxReasoner::new(ontology);
    let spacl_result = spacl_reasoner.is_consistent();
    let spacl_time = start.elapsed();
    println!("✓ SPACL: {:?}", spacl_time);
    
    // Results
    println!();
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║                      RESULTS                                 ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();
    println!("Sequential:  {:?}  (consistent: {})", seq_time, seq_result.unwrap_or(false));
    println!("SPACL:       {:?}  (consistent: {})", spacl_time, spacl_result.unwrap_or(false));
    
    if spacl_time.as_nanos() > 0 {
        let speedup = seq_time.as_nanos() as f64 / spacl_time.as_nanos() as f64;
        println!();
        println!("Speedup:     {:.2}x", speedup);
        
        if speedup > 1.0 {
            println!("Result:      ✅ SPACL is faster");
        } else if speedup > 0.8 {
            println!("Result:      ⚖️  Roughly equivalent");
        } else {
            println!("Result:      ⚠️  Sequential is faster");
        }
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
        "stats" => cmd_stats(command_args),
        "compare" => cmd_compare(command_args),
        "help" | "--help" | "-h" => print_usage(),
        _ => {
            eprintln!("Error: Unknown command '{}'", command);
            println!();
            print_usage();
        }
    }
}
