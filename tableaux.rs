//!
//! Implements a tableaux-based reasoning algorithm for OWL2 ontologies
//! based on SROIQ(D) description logic.

use crate::axioms::*;
use crate::entities::*;
use crate::error::{OwlError, OwlResult};
use crate::iri::IRI;
use crate::ontology::Ontology;

use bumpalo::Bump;
use hashbrown::HashMap;
use smallvec::SmallVec;
use std::cell::RefCell;
use std::collections::{HashSet, VecDeque};
use std::mem;
use std::ptr::NonNull;
use std::sync::Arc;

/// Optimized edge storage for tableaux graph
#[derive(Debug, Default)]
struct EdgeStorage {
    /// Optimized storage for edges using flat representation
    edges: Vec<(NodeId, IRI, NodeId)>,
    /// Index for fast lookups: (from_node, property) -> Vec<to_node>
    index: HashMap<(NodeId, IRI), SmallVec<[NodeId; 4]>>,
}

impl EdgeStorage {
    fn new() -> Self {
        Self {
            edges: Vec::new(),
            index: HashMap::default(),
        }
    }

    fn add_edge(&mut self, from: NodeId, property: &IRI, to: NodeId) {
        // Add to flat storage
        self.edges.push((from, property.clone(), to));

        // Update index
        let key = (from, property.clone());
        self.index.entry(key).or_insert_with(SmallVec::new).push(to);
    }

    fn get_targets(&self, from: NodeId, property: &IRI) -> Option<&[NodeId]> {
        let key = (from, property.clone());
        self.index.get(&key).map(|vec| vec.as_slice())
    }

    #[allow(dead_code)]
    fn clear(&mut self) {
        self.edges.clear();
        self.index.clear();
    }
}

/// Tableaux reasoning engine for OWL2 ontologies
pub struct TableauxReasoner {
    pub ontology: Arc<Ontology>,
    #[allow(dead_code)]
    rules: ReasoningRules,
    cache: ReasoningCache,
    /// Dependency-directed backtracking manager
    dependency_manager: DependencyManager,
}

/// Reasoning configuration options
#[derive(Debug, Clone)]
pub struct ReasoningConfig {
    /// Maximum depth for tableaux expansion
    pub max_depth: usize,
    /// Enable debugging output
    pub debug: bool,
    /// Enable incremental reasoning
    pub incremental: bool,
    /// Timeout in milliseconds
    pub timeout: Option<u64>,
}

impl Default for ReasoningConfig {
    fn default() -> Self {
        ReasoningConfig {
            max_depth: 1000,
            debug: false,
            incremental: true,
            timeout: Some(30000), // 30 seconds default
        }
    }
}

/// Tableaux node with optimized concept storage
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableauxNode {
    id: NodeId,
    /// Optimized concept storage using SmallVec for small sets
    concepts: SmallVec<[ClassExpression; 8]>,
    /// Lazy hashset for large concept sets
    concepts_hashset: Option<HashSet<ClassExpression>>,
