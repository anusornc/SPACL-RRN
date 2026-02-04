//! Strategy selection and optimization for reasoning
//!
//! This module provides:
//! - Meta-reasoner for selecting the best reasoning strategy
//! - Evolutionary optimization for tuning parameters
//! - Profile validation (EL, QL, RL)

pub mod evolutionary;
pub mod meta_reasoner;
pub mod profiles;

// Re-exports
pub use evolutionary::{EvolutionaryOptimizer, EvolutionaryStrategy, PopulationStats};
pub use meta_reasoner::{MetaReasoner, ReasoningStrategy};
pub use profiles::{
    CachePriority, CacheStats, Owl2Profile, Owl2ProfileValidator, ProfileAnalysisReport,
    ProfileCacheConfig, ProfileValidationCache, ProfileValidationResult, ProfileValidator,
    ProfileViolation, ValidationStatistics,
};
