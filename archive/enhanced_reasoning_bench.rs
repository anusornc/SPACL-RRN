use criterion::{black_box, criterion_group, criterion_main, Criterion};
use enhanced_owl_reasoner::*;
use owl2_reasoner::ontology::Ontology;

fn benchmark_enhanced_reasoner(c: &mut Criterion) {
    let ontology = create_test_ontology();
    
    c.bench_function("enhanced_reasoner_consistency", |b| {
        b.iter(|| {
            let mut reasoner = EnhancedOwlReasoner::new(black_box(ontology.clone())).unwrap();
            black_box(reasoner.is_consistent().unwrap())
        })
    });
}

fn create_test_ontology() -> Ontology {
    Ontology::new()
}

criterion_group!(benches, benchmark_enhanced_reasoner);
criterion_main!(benches);
