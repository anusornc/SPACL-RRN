//! Real-World Ontology Benchmark
//!
//! This benchmark evaluates performance on real-world biomedical ontologies
//! with adaptive strategy selection (hierarchical/simple/SPACL).

use criterion::{black_box, criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use owl2_reasoner::{
    Ontology, OwlReasoner,
    ParserFactory,
    SimpleReasoner,
    SpeculativeTableauxReasoner,
    HierarchicalClassificationEngine,
    serializer::BinaryOntologyFormat,
    util::profiling::configure_iri_cache_for_large_ontology,
};
use owl2_reasoner::reasoner::grail_hierarchy::GrailClassificationEngine;

/// Configuration for benchmark runs
const SAMPLE_SIZE: usize = 10;
const MEASUREMENT_TIME_SECS: u64 = 30;

/// Ontology benchmark configuration
struct OntologyBenchmark {
    name: &'static str,
    path: &'static str,
    description: &'static str,
}

/// List of ontologies to benchmark
const ONTOLOGIES: &[OntologyBenchmark] = &[
    OntologyBenchmark {
        name: "LUBM",
        path: "tests/data/univ-bench.owl",
        description: "Lehigh University Benchmark",
    },
    OntologyBenchmark {
        name: "PATO",
        path: "benchmarks/ontologies/other/pato.owl",
        description: "Phenotype And Trait Ontology",
    },
    OntologyBenchmark {
        name: "DOID",
        path: "benchmarks/ontologies/other/doid.owl",
        description: "Disease Ontology",
    },
    OntologyBenchmark {
        name: "UBERON",
        path: "benchmarks/ontologies/other/uberon.owl",
        description: "Uberon Anatomy Ontology",
    },
    OntologyBenchmark {
        name: "GO_Basic",
        path: "benchmarks/ontologies/other/go-basic.owl",
        description: "Gene Ontology (basic)",
    },
];

/// Strategy selection enum
#[derive(Debug, Clone, Copy)]
enum Strategy {
    Hierarchical,
    Simple,
    SPACL,
}

/// Load an ontology from file
fn load_ontology(path: &str) -> Option<Ontology> {
    let path = Path::new(path);
    
    if !path.exists() {
        eprintln!("Warning: Ontology file not found: {}", path.display());
        return None;
    }

    // Prefer binary format if available to avoid costly parsing for large files.
    let bin_path = if path.extension().map(|e| e == "owlbin").unwrap_or(false) {
        path.to_path_buf()
    } else {
        path.with_extension("owlbin")
    };
    if bin_path.exists() {
        println!("Loading binary {}...", bin_path.display());
        let start = std::time::Instant::now();
        let mut file = std::fs::File::open(&bin_path).ok()?;
        let ontology = BinaryOntologyFormat::deserialize(&mut file).ok()?;
        println!("Loaded in {:?}: {} classes", start.elapsed(), ontology.classes().len());
        return Some(ontology);
    }

    // Pre-configure IRI cache based on file size to reduce allocations.
    if let Ok(metadata) = std::fs::metadata(path) {
        let file_size = metadata.len();
        let estimated_classes = (file_size / 50) as usize;
        if estimated_classes > 10_000 {
            configure_iri_cache_for_large_ontology(estimated_classes);
        }
    }
    
    let content = std::fs::read_to_string(path).ok()?;
    let parser = ParserFactory::auto_detect(&content)?;
    
    match parser.parse_str(&content) {
        Ok(ontology) => {
            println!("Loaded {}: {} classes", path.display(), ontology.classes().len());
            Some(ontology)
        }
        Err(e) => {
            eprintln!("Error parsing {}: {:?}", path.display(), e);
            None
        }
    }
}

/// Select the best strategy for an ontology
fn select_strategy(ontology: &Ontology) -> Strategy {
    // First check if hierarchical classification can handle it
    if HierarchicalClassificationEngine::can_handle(ontology) {
        return Strategy::Hierarchical;
    }
    
    // Check complexity metrics
    let class_count = ontology.classes().len();
    let axiom_count = ontology.axioms().len();
    
    // Estimate disjunction density
    let mut union_count = 0;
    for axiom in ontology.axioms() {
        if let Some(sub) = axiom.as_sub_class_of() {
            if contains_union(sub.super_class()) {
                union_count += 1;
            }
        }
    }
    
    let disjunction_density = if axiom_count > 0 {
        union_count as f64 / axiom_count as f64
    } else {
        0.0
    };
    
    // Thresholds for strategy selection
    const SPACL_MIN_CLASSES: usize = 100;
    const SPACL_MIN_DISJUNCTION_DENSITY: f64 = 0.01; // 1%
    
    if class_count >= SPACL_MIN_CLASSES && disjunction_density >= SPACL_MIN_DISJUNCTION_DENSITY {
        Strategy::SPACL
    } else {
        Strategy::Simple
    }
}

/// Check if a class expression contains a union
fn contains_union(expr: &owl2_reasoner::ClassExpression) -> bool {
    use owl2_reasoner::ClassExpression;
    
    match expr {
        ClassExpression::ObjectUnionOf(_) => true,
        ClassExpression::ObjectIntersectionOf(ops) => {
            ops.iter().any(|op| contains_union(op))
        }
        ClassExpression::ObjectComplementOf(inner) => contains_union(inner),
        ClassExpression::ObjectSomeValuesFrom(_, inner) => contains_union(inner),
        ClassExpression::ObjectAllValuesFrom(_, inner) => contains_union(inner),
        _ => false,
    }
}

/// Benchmark adaptive strategy selection
fn bench_adaptive_classification(c: &mut Criterion) {
    let mut group = c.benchmark_group("adaptive_classification");
    group.sample_size(SAMPLE_SIZE);
    group.measurement_time(Duration::from_secs(MEASUREMENT_TIME_SECS));
    
    for ont_config in ONTOLOGIES {
        let Some(ontology) = load_ontology(ont_config.path) else {
            println!("Skipping {} (file not found)", ont_config.name);
            continue;
        };
        let ontology = Arc::new(ontology);
        
        let strategy = select_strategy(&ontology);
        println!("{}: Using {:?} strategy", ont_config.name, strategy);
        
        match strategy {
            Strategy::Hierarchical => {
                let class_count = ontology.classes().len();
                if class_count > 10_000 {
                    group.bench_with_input(
                        BenchmarkId::new("hierarchical_grail", ont_config.name),
                        &ontology,
                        |b, ontology| {
                            b.iter_batched(
                                || Arc::clone(ontology),
                                |ontology| {
                                    let mut engine = GrailClassificationEngine::from_arc(ontology)
                                        .expect("Failed to create GRAIL engine");
                                    let _ = engine.classify();
                                    black_box(engine);
                                },
                                BatchSize::SmallInput,
                            );
                        },
                    );
                } else {
                    group.bench_with_input(
                        BenchmarkId::new("hierarchical", ont_config.name),
                        &ontology,
                        |b, ontology| {
                            b.iter_batched(
                                || Arc::clone(ontology),
                                |ontology| {
                                    let mut engine = HierarchicalClassificationEngine::from_arc(ontology);
                                    let _ = engine.classify();
                                    black_box(engine);
                                },
                                BatchSize::SmallInput,
                            );
                        },
                    );
                }
            }
            Strategy::Simple => {
                group.bench_with_input(
                    BenchmarkId::new("simple", ont_config.name),
                    &ontology,
                    |b, ontology| {
                        b.iter_batched(
                            || Arc::clone(ontology),
                            |ontology| {
                                let mut reasoner = SimpleReasoner::from_arc(ontology);
                                let _ = reasoner.is_consistent();
                                black_box(reasoner);
                            },
                            BatchSize::SmallInput,
                        );
                    },
                );
            }
            Strategy::SPACL => {
                group.bench_with_input(
                    BenchmarkId::new("spacl", ont_config.name),
                    &ontology,
                    |b, ontology| {
                        b.iter_batched(
                            || Arc::clone(ontology),
                            |ontology| {
                                let mut reasoner = SpeculativeTableauxReasoner::from_arc(ontology);
                                let _ = reasoner.is_consistent();
                                black_box(reasoner);
                            },
                            BatchSize::SmallInput,
                        );
                    },
                );
            }
        }
    }
    
    group.finish();
}

/// Sequential baseline using SimpleReasoner
fn bench_sequential_baseline(c: &mut Criterion) {
    let mut group = c.benchmark_group("sequential_baseline");
    group.sample_size(SAMPLE_SIZE);
    group.measurement_time(Duration::from_secs(MEASUREMENT_TIME_SECS));
    
    for ont_config in ONTOLOGIES {
        let Some(ontology) = load_ontology(ont_config.path) else {
            continue;
        };
        let ontology = Arc::new(ontology);
        
        group.bench_with_input(
            BenchmarkId::new("sequential", ont_config.name),
            &ontology,
            |b, ontology| {
                b.iter_batched(
                    || Arc::clone(ontology),
                    |ontology| {
                        let mut reasoner = SimpleReasoner::from_arc(ontology);
                        let _ = reasoner.is_consistent();
                        black_box(reasoner);
                    },
                    BatchSize::SmallInput,
                );
            },
        );
    }
    
    group.finish();
}

criterion_group!(
    benches,
    bench_adaptive_classification,
    bench_sequential_baseline
);
criterion_main!(benches);
