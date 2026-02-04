//! Benchmark SPACL on disjunctive ontology
use owl2_reasoner::{SimpleReasoner, SpeculativeTableauxReasoner, SpeculativeConfig};
use std::time::Instant;

fn main() {
    println!("========================================");
    println!("SPACL Disjunctive Ontology Benchmark");
    println!("========================================\n");
    
    let content = std::fs::read_to_string("tests/data/disjunctive_test.owl").unwrap();
    let parser = owl2_reasoner::ParserFactory::auto_detect(&content).unwrap();
    let ontology = parser.parse_str(&content).unwrap();
    
    let class_count = ontology.classes().len();
    let axiom_count = ontology.axioms().len();
    
    println!("Ontology: disjunctive_test.owl");
    println!("Classes: {}", class_count);
    println!("Axioms: {}", axiom_count);
    println!("Structure: (A ⊔ B) ⊓ (C ⊔ D) ⊓ E + 50 subclasses of E");
    println!();
    
    // Sequential benchmark
    println!("--- Sequential ---");
    let start = Instant::now();
    let seq = SimpleReasoner::new(ontology.clone());
    let seq_result = seq.is_consistent().unwrap();
    let seq_time = start.elapsed();
    println!("Result: {} in {:?}", seq_result, seq_time);
    
    // SPACL benchmark - force parallel mode
    println!("\n--- SPACL (parallel mode) ---");
    let start = Instant::now();
    let mut config = SpeculativeConfig::default();
    config.parallel_threshold = 5; // Force parallel
    let mut spacl = SpeculativeTableauxReasoner::with_config(ontology.clone(), config);
    let spacl_result = spacl.is_consistent().unwrap();
    let spacl_time = start.elapsed();
    let stats = spacl.get_stats();
    
    println!("Result: {} in {:?}", spacl_result, spacl_time);
    println!("Branches created: {}", stats.branches_created);
    println!("Branches pruned: {}", stats.branches_pruned);
    println!("Nogoods learned: {}", stats.nogoods_learned);
    
    // Verify correctness
    if seq_result == spacl_result {
        println!("\n✓ Results match!");
    } else {
        println!("\n✗ MISMATCH!");
    }
    
    // Calculate performance
    let seq_ms = seq_time.as_millis() as f64;
    let spacl_ms = spacl_time.as_millis() as f64;
    
    if spacl_ms > 0.0 {
        let speedup = seq_ms / spacl_ms;
        if speedup > 1.0 {
            println!("\n🚀 Speedup: {:.2}x", speedup);
        } else {
            println!("\n⚠️  Overhead: {:.2}x", 1.0/speedup);
        }
    }
    
    println!("\n========================================");
}
