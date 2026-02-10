#![allow(unused_imports, unused_variables, unused_mut, dead_code)]
//! Benchmark on simple disjunctive ontology
use owl2_reasoner::{SimpleReasoner, SpeculativeTableauxReasoner, SpeculativeConfig};
use std::time::Instant;

fn main() {
    println!("========================================");
    println!("Simple Disjunctive Benchmark");
    println!("========================================\n");
    
    let content = std::fs::read_to_string("tests/data/disjunctive_simple.owl").unwrap();
    let parser = owl2_reasoner::ParserFactory::auto_detect(&content).unwrap();
    let ontology = parser.parse_str(&content).unwrap();
    
    println!("Classes: {}", ontology.classes().len());
    println!("Axioms: {}", ontology.axioms().len());
    
    // Sequential
    println!("\n--- Sequential ---");
    let start = Instant::now();
    let seq = SimpleReasoner::new(ontology.clone());
    let seq_result = seq.is_consistent().unwrap();
    let seq_time = start.elapsed();
    println!("Result: {} in {:?}", seq_result, seq_time);
    
    // SPACL forced parallel
    println!("\n--- SPACL (forced parallel) ---");
    let start = Instant::now();
    let mut config = SpeculativeConfig::default();
    config.parallel_threshold = 5;
    let mut spacl = SpeculativeTableauxReasoner::with_config(ontology, config);
    let spacl_result = spacl.is_consistent().unwrap();
    let spacl_time = start.elapsed();
    let stats = spacl.get_stats();
    
    println!("Result: {} in {:?}", spacl_result, spacl_time);
    println!("Branches created: {}", stats.branches_created);
    
    if seq_result == spacl_result {
        println!("\n✓ Results match!");
    }
    
    let speedup = seq_time.as_micros() as f64 / spacl_time.as_micros() as f64;
    if speedup > 1.0 {
        println!("🚀 Speedup: {:.2}x", speedup);
    } else {
        println!("⚠️  Overhead: {:.2}x", 1.0/speedup);
    }
    
    println!("\n========================================");
}
