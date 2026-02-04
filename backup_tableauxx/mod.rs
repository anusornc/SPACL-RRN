//! Enhanced reasoning components
//!
//! This module provides specialized reasoning implementations optimized
//! for different OWL2 profiles and use cases.

pub mod saturation;
pub mod transformation;

pub use saturation::*;
pub use transformation::*;

/// Common trait for all reasoning components
pub trait EnhancedReasoner {
    /// Check if the ontology is consistent
    fn is_consistent(&mut self) -> Result<bool, String>;
    
    /// Check if one class is a subclass of another
    fn is_subclass_of(&mut self, sub: &str, sup: &str) -> Result<bool, String>;
    
    /// Get all instances of a class
    fn get_instances(&mut self, class: &str) -> Result<Vec<String>, String>;
    
    /// Get performance metrics
    fn get_performance_metrics(&self) -> ReasoningMetrics;
}

/// Performance metrics for reasoning operations
#[derive(Debug, Default, Clone)]
pub struct ReasoningMetrics {
    pub total_time_ms: u64,
    pub memory_usage_mb: f64,
    pub cache_hits: u32,
    pub cache_misses: u32,
    pub rule_applications: u32,
}

impl ReasoningMetrics {
    /// Create new empty metrics
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Add time to metrics
    pub fn add_time(&mut self, time_ms: u64) {
        self.total_time_ms += time_ms;
    }
    
    /// Record cache hit
    pub fn record_cache_hit(&mut self) {
        self.cache_hits += 1;
    }
    
    /// Record cache miss
    pub fn record_cache_miss(&mut self) {
        self.cache_misses += 1;
    }
    
    /// Get cache hit ratio
    pub fn cache_hit_ratio(&self) -> f64 {
        if self.cache_hits + self.cache_misses == 0 {
            0.0
        } else {
            self.cache_hits as f64 / (self.cache_hits + self.cache_misses) as f64
        }
    }
}
