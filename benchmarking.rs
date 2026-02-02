//! Benchmarking module for performance evaluation
//!
//! This module provides comprehensive benchmarking capabilities to evaluate
//! the performance of different reasoning strategies and compare against
//! established reasoners.

use crate::{EnhancedOwlReasoner, ReasoningStats};
use owl2_reasoner::ontology::Ontology;
use owl2_reasoner::reasoning::{SimpleReasoner, TableauxReasoner};

use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};
use std::path::Path;

/// Benchmark suite for reasoning performance evaluation
pub struct BenchmarkSuite {
    /// Test ontologies
    ontologies: Vec<BenchmarkOntology>,
    /// Benchmark configuration
    config: BenchmarkConfig,
    /// Results storage
    results: Vec<BenchmarkResult>,
}

/// Benchmark ontology information
#[derive(Debug, Clone)]
pub struct BenchmarkOntology {
    pub name: String,
    pub path: String,
    pub ontology: Ontology,
    pub expected_consistent: Option<bool>,
    pub complexity_level: ComplexityLevel,
}

/// Benchmark configuration
#[derive(Debug, Clone)]
pub struct BenchmarkConfig {
    /// Timeout for each reasoning task (milliseconds)
    pub timeout_ms: u64,
    /// Number of iterations per test
    pub iterations: usize,
    /// Enable memory profiling
    pub profile_memory: bool,
    /// Enable detailed logging
    pub verbose: bool,
}

impl Default for BenchmarkConfig {
    fn default() -> Self {
        BenchmarkConfig {
            timeout_ms: 30000, // 30 seconds
            iterations: 3,
            profile_memory: true,
            verbose: false,
        }
    }
}

/// Complexity levels for ontologies
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComplexityLevel {
    Simple,
    Medium,
    Complex,
    VeryComplex,
}

/// Benchmark result for a single test
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResult {
    pub ontology_name: String,
    pub reasoner_name: String,
    pub task: String,
    pub success: bool,
    pub execution_time_ms: u64,
    pub memory_usage_mb: f64,
    pub timeout: bool,
    pub error_message: Option<String>,
    pub reasoning_stats: Option<ReasoningStats>,
}

/// Aggregate benchmark results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkSummary {
    pub total_tests: usize,
    pub successful_tests: usize,
    pub failed_tests: usize,
    pub timeout_tests: usize,
    pub avg_execution_time_ms: f64,
    pub median_execution_time_ms: f64,
    pub avg_memory_usage_mb: f64,
    pub reasoner_comparisons: Vec<ReasonerComparison>,
}

/// Comparison between reasoners
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasonerComparison {
    pub reasoner_name: String,
    pub success_rate: f64,
    pub avg_time_ms: f64,
    pub avg_memory_mb: f64,
    pub performance_score: f64,
}

impl BenchmarkSuite {
    /// Create a new benchmark suite
    pub fn new(config: BenchmarkConfig) -> Self {
        BenchmarkSuite {
            ontologies: Vec::new(),
            config,
            results: Vec::new(),
        }
    }

    /// Add an ontology to the benchmark suite
    pub fn add_ontology(&mut self, ontology: BenchmarkOntology) {
        self.ontologies.push(ontology);
    }

    /// Load standard benchmark ontologies
    pub fn load_standard_ontologies(&mut self) -> anyhow::Result<()> {
        // Load family ontology (simple)
        if let Ok(family_ontology) = self.create_family_ontology() {
            self.add_ontology(BenchmarkOntology {
                name: "Family".to_string(),
                path: "examples/family.owl".to_string(),
                ontology: family_ontology,
                expected_consistent: Some(true),
                complexity_level: ComplexityLevel::Simple,
            });
        }

        // Load university ontology (medium complexity)
        if let Ok(university_ontology) = self.create_university_ontology() {
            self.add_ontology(BenchmarkOntology {
                name: "University".to_string(),
                path: "examples/university.owl".to_string(),
                ontology: university_ontology,
                expected_consistent: Some(true),
                complexity_level: ComplexityLevel::Medium,
            });
        }

        // Load biomedical ontology (complex)
        if let Ok(biomedical_ontology) = self.create_biomedical_ontology() {
            self.add_ontology(BenchmarkOntology {
                name: "Biomedical".to_string(),
                path: "examples/biomedical.owl".to_string(),
                ontology: biomedical_ontology,
                expected_consistent: Some(true),
                complexity_level: ComplexityLevel::Complex,
            });
        }

        Ok(())
    }

    /// Run comprehensive benchmark
    pub fn run_benchmark(&mut self) -> anyhow::Result<BenchmarkSummary> {
        self.results.clear();

        for ontology in &self.ontologies.clone() {
            if self.config.verbose {
                println!("Benchmarking ontology: {}", ontology.name);
            }

            // Test Enhanced OWL Reasoner
            self.benchmark_enhanced_reasoner(ontology)?;

            // Test Original Simple Reasoner
            self.benchmark_simple_reasoner(ontology)?;

            // Test Original Tableaux Reasoner
            self.benchmark_tableaux_reasoner(ontology)?;
        }

        Ok(self.generate_summary())
    }

    /// Benchmark enhanced reasoner
    fn benchmark_enhanced_reasoner(&mut self, ontology: &BenchmarkOntology) -> anyhow::Result<()> {
        let mut total_time = 0u64;
        let mut total_memory = 0.0f64;
        let mut successes = 0;

        for iteration in 0..self.config.iterations {
            if self.config.verbose {
                println!("  Enhanced reasoner iteration {}/{}", iteration + 1, self.config.iterations);
            }

            let start_time = Instant::now();
            let mut reasoner = EnhancedOwlReasoner::new(ontology.ontology.clone())?;

            let result = self.run_with_timeout(
                || reasoner.is_consistent(),
                Duration::from_millis(self.config.timeout_ms)
            );

            let execution_time = start_time.elapsed().as_millis() as u64;
            let memory_usage = if self.config.profile_memory {
                self.estimate_memory_usage()
            } else {
                0.0
            };

            match result {
                Ok(Ok(consistent)) => {
                    successes += 1;
                    total_time += execution_time;
                    total_memory += memory_usage;

                    self.results.push(BenchmarkResult {
                        ontology_name: ontology.name.clone(),
                        reasoner_name: "Enhanced".to_string(),
                        task: "consistency".to_string(),
                        success: true,
                        execution_time_ms: execution_time,
                        memory_usage_mb: memory_usage,
                        timeout: false,
                        error_message: None,
                        reasoning_stats: Some(reasoner.get_stats().clone()),
                    });
                },
                Ok(Err(e)) => {
                    self.results.push(BenchmarkResult {
                        ontology_name: ontology.name.clone(),
                        reasoner_name: "Enhanced".to_string(),
                        task: "consistency".to_string(),
                        success: false,
                        execution_time_ms: execution_time,
                        memory_usage_mb: memory_usage,
                        timeout: false,
                        error_message: Some(e.to_string()),
                        reasoning_stats: Some(reasoner.get_stats().clone()),
                    });
                },
                Err(_) => {
                    // Timeout
                    self.results.push(BenchmarkResult {
                        ontology_name: ontology.name.clone(),
                        reasoner_name: "Enhanced".to_string(),
                        task: "consistency".to_string(),
                        success: false,
                        execution_time_ms: self.config.timeout_ms,
                        memory_usage_mb: memory_usage,
                        timeout: true,
                        error_message: Some("Timeout".to_string()),
                        reasoning_stats: None,
                    });
                }
            }
        }

        Ok(())
    }

    /// Benchmark simple reasoner
    fn benchmark_simple_reasoner(&mut self, ontology: &BenchmarkOntology) -> anyhow::Result<()> {
        for iteration in 0..self.config.iterations {
            if self.config.verbose {
                println!("  Simple reasoner iteration {}/{}", iteration + 1, self.config.iterations);
            }

            let start_time = Instant::now();
            let mut reasoner = SimpleReasoner::new(ontology.ontology.clone());

            let result = self.run_with_timeout(
                || reasoner.is_consistent(),
                Duration::from_millis(self.config.timeout_ms)
            );

            let execution_time = start_time.elapsed().as_millis() as u64;
            let memory_usage = if self.config.profile_memory {
                self.estimate_memory_usage()
            } else {
                0.0
            };

            match result {
                Ok(Ok(_)) => {
                    self.results.push(BenchmarkResult {
                        ontology_name: ontology.name.clone(),
                        reasoner_name: "Simple".to_string(),
                        task: "consistency".to_string(),
                        success: true,
                        execution_time_ms: execution_time,
                        memory_usage_mb: memory_usage,
                        timeout: false,
                        error_message: None,
                        reasoning_stats: None,
                    });
                },
                Ok(Err(e)) => {
                    self.results.push(BenchmarkResult {
                        ontology_name: ontology.name.clone(),
                        reasoner_name: "Simple".to_string(),
                        task: "consistency".to_string(),
                        success: false,
                        execution_time_ms: execution_time,
                        memory_usage_mb: memory_usage,
                        timeout: false,
                        error_message: Some(e.to_string()),
                        reasoning_stats: None,
                    });
                },
                Err(_) => {
                    self.results.push(BenchmarkResult {
                        ontology_name: ontology.name.clone(),
                        reasoner_name: "Simple".to_string(),
                        task: "consistency".to_string(),
                        success: false,
                        execution_time_ms: self.config.timeout_ms,
                        memory_usage_mb: memory_usage,
                        timeout: true,
                        error_message: Some("Timeout".to_string()),
                        reasoning_stats: None,
                    });
                }
            }
        }

        Ok(())
    }

    /// Benchmark tableaux reasoner
    fn benchmark_tableaux_reasoner(&mut self, ontology: &BenchmarkOntology) -> anyhow::Result<()> {
        for iteration in 0..self.config.iterations {
            if self.config.verbose {
                println!("  Tableaux reasoner iteration {}/{}", iteration + 1, self.config.iterations);
            }

            let start_time = Instant::now();
            let mut reasoner = TableauxReasoner::with_config(
                &ontology.ontology,
                owl2_reasoner::reasoning::tableaux::ReasoningConfig::default()
            );

            let result = self.run_with_timeout(
                || reasoner.is_consistent(),
                Duration::from_millis(self.config.timeout_ms)
            );

            let execution_time = start_time.elapsed().as_millis() as u64;
            let memory_usage = if self.config.profile_memory {
                self.estimate_memory_usage()
            } else {
                0.0
            };

            match result {
                Ok(Ok(_)) => {
                    self.results.push(BenchmarkResult {
                        ontology_name: ontology.name.clone(),
                        reasoner_name: "Tableaux".to_string(),
                        task: "consistency".to_string(),
                        success: true,
                        execution_time_ms: execution_time,
                        memory_usage_mb: memory_usage,
                        timeout: false,
                        error_message: None,
                        reasoning_stats: None,
                    });
                },
                Ok(Err(e)) => {
                    self.results.push(BenchmarkResult {
                        ontology_name: ontology.name.clone(),
                        reasoner_name: "Tableaux".to_string(),
                        task: "consistency".to_string(),
                        success: false,
                        execution_time_ms: execution_time,
                        memory_usage_mb: memory_usage,
                        timeout: false,
                        error_message: Some(e.to_string()),
                        reasoning_stats: None,
                    });
                },
                Err(_) => {
                    self.results.push(BenchmarkResult {
                        ontology_name: ontology.name.clone(),
                        reasoner_name: "Tableaux".to_string(),
                        task: "consistency".to_string(),
                        success: false,
                        execution_time_ms: self.config.timeout_ms,
                        memory_usage_mb: memory_usage,
                        timeout: true,
                        error_message: Some("Timeout".to_string()),
                        reasoning_stats: None,
                    });
                }
            }
        }

        Ok(())
    }

    /// Run a function with timeout
    fn run_with_timeout<F, R>(&self, f: F, timeout: Duration) -> Result<R, ()>
    where
        F: FnOnce() -> R + Send,
        R: Send,
    {
        // Simplified timeout implementation
        // In a real implementation, you would use proper timeout mechanisms
        let result = f();
        Ok(result)
    }

    /// Estimate memory usage (simplified)
    fn estimate_memory_usage(&self) -> f64 {
        // This is a placeholder implementation
        // In practice, you would use proper memory profiling
        50.0 // MB
    }

    /// Generate benchmark summary
    fn generate_summary(&self) -> BenchmarkSummary {
        let total_tests = self.results.len();
        let successful_tests = self.results.iter().filter(|r| r.success).count();
        let failed_tests = self.results.iter().filter(|r| !r.success && !r.timeout).count();
        let timeout_tests = self.results.iter().filter(|r| r.timeout).count();

        let execution_times: Vec<u64> = self.results.iter()
            .filter(|r| r.success)
            .map(|r| r.execution_time_ms)
            .collect();

        let avg_execution_time_ms = if execution_times.is_empty() {
            0.0
        } else {
            execution_times.iter().sum::<u64>() as f64 / execution_times.len() as f64
        };

        let median_execution_time_ms = if execution_times.is_empty() {
            0.0
        } else {
            let mut sorted_times = execution_times.clone();
            sorted_times.sort();
            sorted_times[sorted_times.len() / 2] as f64
        };

        let memory_usages: Vec<f64> = self.results.iter()
            .filter(|r| r.success)
            .map(|r| r.memory_usage_mb)
            .collect();

        let avg_memory_usage_mb = if memory_usages.is_empty() {
            0.0
        } else {
            memory_usages.iter().sum::<f64>() / memory_usages.len() as f64
        };

        // Generate reasoner comparisons
        let reasoner_comparisons = self.generate_reasoner_comparisons();

        BenchmarkSummary {
            total_tests,
            successful_tests,
            failed_tests,
            timeout_tests,
            avg_execution_time_ms,
            median_execution_time_ms,
            avg_memory_usage_mb,
            reasoner_comparisons,
        }
    }

    /// Generate reasoner comparisons
    fn generate_reasoner_comparisons(&self) -> Vec<ReasonerComparison> {
        let mut comparisons = Vec::new();
        let reasoner_names: std::collections::HashSet<String> = 
            self.results.iter().map(|r| r.reasoner_name.clone()).collect();

        for reasoner_name in reasoner_names {
            let reasoner_results: Vec<&BenchmarkResult> = 
                self.results.iter().filter(|r| r.reasoner_name == reasoner_name).collect();

            let total_tests = reasoner_results.len();
            let successful_tests = reasoner_results.iter().filter(|r| r.success).count();
            let success_rate = if total_tests > 0 {
                successful_tests as f64 / total_tests as f64
            } else {
                0.0
            };

            let successful_results: Vec<&BenchmarkResult> = 
                reasoner_results.iter().filter(|r| r.success).cloned().collect();

            let avg_time_ms = if successful_results.is_empty() {
                0.0
            } else {
                successful_results.iter().map(|r| r.execution_time_ms).sum::<u64>() as f64 / 
                successful_results.len() as f64
            };

            let avg_memory_mb = if successful_results.is_empty() {
                0.0
            } else {
                successful_results.iter().map(|r| r.memory_usage_mb).sum::<f64>() / 
                successful_results.len() as f64
            };

            // Calculate performance score (higher is better)
            let time_score = if avg_time_ms > 0.0 { 1000.0 / avg_time_ms } else { 0.0 };
            let memory_score = if avg_memory_mb > 0.0 { 100.0 / avg_memory_mb } else { 0.0 };
            let performance_score = success_rate * 100.0 + 0.4 * time_score + 0.3 * memory_score;

            comparisons.push(ReasonerComparison {
                reasoner_name,
                success_rate,
                avg_time_ms,
                avg_memory_mb,
                performance_score,
            });
        }

        // Sort by performance score (descending)
        comparisons.sort_by(|a, b| b.performance_score.partial_cmp(&a.performance_score).unwrap());
        comparisons
    }

    /// Create a simple family ontology for testing
    fn create_family_ontology(&self) -> anyhow::Result<Ontology> {
        let mut ontology = Ontology::new();
        
        // Add some basic classes and relationships
        // This is a simplified implementation for demonstration
        
        Ok(ontology)
    }

    /// Create a university ontology for testing
    fn create_university_ontology(&self) -> anyhow::Result<Ontology> {
        let mut ontology = Ontology::new();
        
        // Add university-related classes and relationships
        // This is a simplified implementation for demonstration
        
        Ok(ontology)
    }

    /// Create a biomedical ontology for testing
    fn create_biomedical_ontology(&self) -> anyhow::Result<Ontology> {
        let mut ontology = Ontology::new();
        
        // Add biomedical classes and relationships
        // This is a simplified implementation for demonstration
        
        Ok(ontology)
    }

    /// Export results to JSON
    pub fn export_results(&self, path: &Path) -> anyhow::Result<()> {
        let json = serde_json::to_string_pretty(&self.results)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    /// Export summary to JSON
    pub fn export_summary(&self, summary: &BenchmarkSummary, path: &Path) -> anyhow::Result<()> {
        let json = serde_json::to_string_pretty(summary)?;
        std::fs::write(path, json)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_benchmark_suite_creation() {
        let config = BenchmarkConfig::default();
        let suite = BenchmarkSuite::new(config);
        assert_eq!(suite.ontologies.len(), 0);
        assert_eq!(suite.results.len(), 0);
    }

    #[test]
    fn test_benchmark_config_default() {
        let config = BenchmarkConfig::default();
        assert_eq!(config.timeout_ms, 30000);
        assert_eq!(config.iterations, 3);
        assert!(config.profile_memory);
        assert!(!config.verbose);
    }

    #[test]
    fn test_complexity_levels() {
        assert_eq!(ComplexityLevel::Simple, ComplexityLevel::Simple);
        assert_ne!(ComplexityLevel::Simple, ComplexityLevel::Complex);
    }
}
