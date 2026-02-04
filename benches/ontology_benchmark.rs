//! Simple Real-World Ontology Benchmark
//! 
//! Run with: cargo bench --bench ontology_benchmark

use criterion::{criterion_group, criterion_main, Criterion};
use std::path::Path;
use std::time::Duration;

use owl2_reasoner::{
    Ontology, OntologyParser,
    SimpleReasoner,
    SpeculativeTableauxReasoner,
};

/// Benchmark ontologies
const LUBM_PATH: &str = "tests/data/univ-bench.owl";
const GO_PATH: &str = "benchmarks/ontologies/other/go-basic.owl";

fn load_ontology(path: &str) -> Option<Ontology> {
    use owl2_reasoner::parser::{RdfXmlParser, OntologyParser};
    
    let path = Path::new(path);
    if !path.exists() {
        println!("File not found: {}", path.display());
        return None;
    }
    
    // Try RDF/XML parser first (most .owl files are RDF/XML)
    println!("Loading {}...", path.display());
    let start = std::time::Instant::now();
    
    let parser = RdfXmlParser::new();
    let result = parser.parse_file(path);
    let elapsed = start.elapsed();
    
    match result {
        Ok(ontology) => {
            println!("Loaded in {:?}: {} classes", elapsed, ontology.classes().len());
            Some(ontology)
        }
        Err(e) => {
            println!("Error loading: {:?}", e);
            None
        }
    }
}

fn bench_lubm(c: &mut Criterion) {
    let Some(ontology) = load_ontology(LUBM_PATH) else {
        println!("LUBM ontology not found, skipping benchmark");
        return;
    };
    
    let mut group = c.benchmark_group("lubm_consistency");
    group.measurement_time(Duration::from_secs(10));
    
    // Sequential benchmark
    group.bench_function("sequential", |b| {
        b.iter(|| {
            let reasoner = SimpleReasoner::new(ontology.clone());
            let _ = reasoner.is_consistent();
        });
    });
    
    // SPACL benchmark
    group.bench_function("spacl", |b| {
        b.iter(|| {
            let mut reasoner = SpeculativeTableauxReasoner::new(ontology.clone());
            let _ = reasoner.is_consistent();
        });
    });
    
    group.finish();
}

fn bench_go(c: &mut Criterion) {
    let Some(ontology) = load_ontology(GO_PATH) else {
        println!("GO ontology not found, skipping benchmark");
        return;
    };
    
    println!("Running GO benchmark (this may take a while)...");
    
    let mut group = c.benchmark_group("go_consistency");
    group.measurement_time(Duration::from_secs(60));
    group.sample_size(5); // Fewer samples for large ontology
    
    // Sequential benchmark
    group.bench_function("sequential", |b| {
        b.iter(|| {
            let reasoner = SimpleReasoner::new(ontology.clone());
            let _ = reasoner.is_consistent();
        });
    });
    
    // SPACL benchmark
    group.bench_function("spacl", |b| {
        b.iter(|| {
            let mut reasoner = SpeculativeTableauxReasoner::new(ontology.clone());
            let _ = reasoner.is_consistent();
        });
    });
    
    group.finish();
}

criterion_group!(benches, bench_lubm, bench_go);
criterion_main!(benches);
