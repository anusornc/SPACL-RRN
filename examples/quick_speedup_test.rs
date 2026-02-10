#![allow(unused_imports, unused_variables, unused_mut, dead_code)]
use owl2_reasoner::{SimpleReasoner, SpeculativeTableauxReasoner, SpeculativeConfig};
use std::time::Instant;

fn main() {
    // Small test with univ-bench
    let content = std::fs::read_to_string("tests/data/univ-bench.owl").unwrap();
    let parser = owl2_reasoner::ParserFactory::auto_detect(&content).unwrap();
    let ontology = parser.parse_str(&content).unwrap();
    
    println!("Univ-bench: {} classes, {} axioms", 
        ontology.classes().len(),
        ontology.axioms().len()
    );
    
    // Sequential
    let start = Instant::now();
    let seq = SimpleReasoner::new(ontology.clone());
    let seq_result = seq.is_consistent().unwrap();
    let seq_time = start.elapsed();
    println!("Sequential: {} in {:?}", seq_result, seq_time);
    
    // SPACL with low threshold to force parallel
    let start = Instant::now();
    let mut config = SpeculativeConfig::default();
    config.parallel_threshold = 5; // Force parallel
    let mut spacl = SpeculativeTableauxReasoner::with_config(ontology.clone(), config);
    let spacl_result = spacl.is_consistent().unwrap();
    let spacl_time = start.elapsed();
    let stats = spacl.get_stats();
    
    println!("SPACL: {} in {:?}", spacl_result, spacl_time);
    println!("  Branches created: {}", stats.branches_created);
    println!("  Branches pruned: {}", stats.branches_pruned);
    println!("  Nogoods learned: {}", stats.nogoods_learned);
    
    let speedup = seq_time.as_micros() as f64 / spacl_time.as_micros() as f64;
    println!("  Speedup: {:.2}x", speedup);
}
