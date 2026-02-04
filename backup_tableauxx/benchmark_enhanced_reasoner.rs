//! Benchmark example for the enhanced OWL reasoner
//!
//! This example demonstrates how to use the enhanced reasoner and compare
//! its performance against the original reasoners.

use enhanced_owl_reasoner::*;
use enhanced_owl_reasoner::benchmarking::*;
use owl2_reasoner::ontology::Ontology;
use std::time::Instant;

fn main() -> anyhow::Result<()> {
    env_logger::init();
    
    println!("Enhanced OWL Reasoner Benchmark");
    println!("================================");
    
    // Create benchmark configuration
    let config = BenchmarkConfig {
        timeout_ms: 30000,
        iterations: 3,
        profile_memory: true,
        verbose: true,
    };
    
    // Create benchmark suite
    let mut suite = BenchmarkSuite::new(config);
    
    // Load standard test ontologies
    println!("Loading standard test ontologies...");
    suite.load_standard_ontologies()?;
    
    // Run comprehensive benchmark
    println!("Running comprehensive benchmark...");
    let start_time = Instant::now();
    let summary = suite.run_benchmark()?;
    let total_time = start_time.elapsed();
    
    // Display results
    println!("\nBenchmark Results");
    println!("=================");
    println!("Total execution time: {:.2}s", total_time.as_secs_f64());
    println!("Total tests: {}", summary.total_tests);
    println!("Successful tests: {}", summary.successful_tests);
    println!("Failed tests: {}", summary.failed_tests);
    println!("Timeout tests: {}", summary.timeout_tests);
    println!("Average execution time: {:.2}ms", summary.avg_execution_time_ms);
    println!("Median execution time: {:.2}ms", summary.median_execution_time_ms);
    println!("Average memory usage: {:.2}MB", summary.avg_memory_usage_mb);
    
    // Display reasoner comparisons
    println!("\nReasoner Performance Comparison");
    println!("===============================");
    println!("{:<15} {:<12} {:<12} {:<12} {:<15}", 
             "Reasoner", "Success Rate", "Avg Time(ms)", "Avg Mem(MB)", "Performance");
    println!("{:-<75}", "");
    
    for comparison in &summary.reasoner_comparisons {
        println!("{:<15} {:<12.1}% {:<12.1} {:<12.1} {:<15.1}",
                 comparison.reasoner_name,
                 comparison.success_rate * 100.0,
                 comparison.avg_time_ms,
                 comparison.avg_memory_mb,
                 comparison.performance_score);
    }
    
    // Export results
    println!("\nExporting results...");
    suite.export_results(std::path::Path::new("benchmark_results.json"))?;
    suite.export_summary(&summary, std::path::Path::new("benchmark_summary.json"))?;
    
    // Test individual enhanced reasoner features
    println!("\nTesting Enhanced Reasoner Features");
    println!("==================================");
    test_enhanced_reasoner_features()?;
    
    // Test evolutionary optimization
    println!("\nTesting Evolutionary Optimization");
    println!("=================================");
    test_evolutionary_optimization()?;
    
    println!("\nBenchmark completed successfully!");
    Ok(())
}

fn test_enhanced_reasoner_features() -> anyhow::Result<()> {
    // Create a simple test ontology
    let ontology = create_test_ontology()?;
    
    // Test enhanced reasoner
    println!("Creating enhanced reasoner...");
    let mut enhanced_reasoner = EnhancedOwlReasoner::new(ontology.clone())?;
    
    // Test consistency checking
    println!("Testing consistency checking...");
    let start_time = Instant::now();
    let is_consistent = enhanced_reasoner.is_consistent()?;
    let consistency_time = start_time.elapsed();
    
    println!("Ontology is consistent: {}", is_consistent);
    println!("Consistency check time: {:.2}ms", consistency_time.as_millis());
    
    // Display performance statistics
    let stats = enhanced_reasoner.get_stats();
    println!("Performance Statistics:");
    println!("  Total reasoning time: {}ms", stats.total_reasoning_time_ms);
    println!("  Tableaux calls: {}", stats.tableaux_calls);
    println!("  Saturation calls: {}", stats.saturation_calls);
    println!("  Transformation calls: {}", stats.transformation_calls);
    println!("  Meta-reasoner overhead: {}ms", stats.meta_reasoner_overhead_ms);
    println!("  Cache hits: {}", stats.cache_hits);
    println!("  Cache misses: {}", stats.cache_misses);
    
    Ok(())
}

fn test_evolutionary_optimization() -> anyhow::Result<()> {
    use enhanced_owl_reasoner::evolutionary::*;
    
    // Create evolutionary optimizer
    println!("Creating evolutionary optimizer...");
    let mut optimizer = EvolutionaryOptimizer::new(20);
    
    // Simulate some performance data
    println!("Simulating evolutionary optimization...");
    for generation in 0..5 {
        println!("Generation {}: ", generation);
        
        // Simulate performance evaluation for each strategy
        for strategy_idx in 0..optimizer.population_size.min(5) {
            let performance = PerformanceRecord {
                execution_time_ms: rand::random::<u64>() % 5000 + 1000,
                memory_usage_mb: rand::random::<f64>() * 100.0 + 50.0,
                success: rand::random::<f64>() > 0.2, // 80% success rate
                ontology_features: "test_features".to_string(),
                reasoning_task: "consistency".to_string(),
            };
            
            optimizer.evaluate_fitness(strategy_idx, &performance);
        }
        
        // Evolve to next generation
        optimizer.evolve_generation()?;
        
        // Display population statistics
        let stats = optimizer.get_population_stats();
        println!("  Population size: {}", stats.population_size);
        println!("  Max fitness: {:.2}", stats.max_fitness);
        println!("  Avg fitness: {:.2}", stats.avg_fitness);
        println!("  Min fitness: {:.2}", stats.min_fitness);
    }
    
    // Display best strategy
    if let Some(best_strategy) = optimizer.get_best_strategy() {
        println!("Best strategy found:");
        println!("  Fitness: {:.2}", best_strategy.fitness);
        println!("  Performance history entries: {}", best_strategy.performance_history.len());
        println!("  Feature weights: {:?}", best_strategy.parameters.feature_weights);
    }
    
    Ok(())
}

fn create_test_ontology() -> anyhow::Result<Ontology> {
    use owl2_reasoner::entities::*;
    use owl2_reasoner::iri::IRI;
    
    let mut ontology = Ontology::new();
    
    // Create some basic classes
    let person_iri = IRI::new("http://example.org/Person")?;
    let student_iri = IRI::new("http://example.org/Student")?;
    let teacher_iri = IRI::new("http://example.org/Teacher")?;
    
    let person_class = Class::new(person_iri.clone());
    let student_class = Class::new(student_iri.clone());
    let teacher_class = Class::new(teacher_iri.clone());
    
    ontology.add_class(person_class)?;
    ontology.add_class(student_class)?;
    ontology.add_class(teacher_class)?;
    
    // Create subclass relationships
    let student_subclass_axiom = SubClassOfAxiom {
        sub_class: ClassExpression::Class(Class::new(student_iri)),
        super_class: ClassExpression::Class(Class::new(person_iri.clone())),
        annotations: vec![],
    };
    
    let teacher_subclass_axiom = SubClassOfAxiom {
        sub_class: ClassExpression::Class(Class::new(teacher_iri)),
        super_class: ClassExpression::Class(Class::new(person_iri)),
        annotations: vec![],
    };
    
    ontology.add_axiom(Axiom::SubClassOf(student_subclass_axiom))?;
    ontology.add_axiom(Axiom::SubClassOf(teacher_subclass_axiom))?;
    
    // Add some individuals
    let john_iri = IRI::new("http://example.org/John")?;
    let mary_iri = IRI::new("http://example.org/Mary")?;
    
    let john_assertion = ClassAssertionAxiom {
        class_expression: ClassExpression::Class(Class::new(student_iri)),
        individual: john_iri,
        annotations: vec![],
    };
    
    let mary_assertion = ClassAssertionAxiom {
        class_expression: ClassExpression::Class(Class::new(teacher_iri)),
        individual: mary_iri,
        annotations: vec![],
    };
    
    ontology.add_axiom(Axiom::ClassAssertion(john_assertion))?;
    ontology.add_axiom(Axiom::ClassAssertion(mary_assertion))?;
    
    Ok(ontology)
}
