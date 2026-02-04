//! JSON-LD Parser (stub implementation)
use crate::core::error::{OwlError, OwlResult};
use crate::core::ontology::Ontology;
use crate::parser::OntologyParser;
use std::path::Path;

pub struct JsonLdParser;

impl JsonLdParser {
    pub fn new() -> Self {
        Self
    }
}

impl OntologyParser for JsonLdParser {
    fn parse_str(&self, _content: &str) -> OwlResult<Ontology> {
        Err(OwlError::ParseError(
            "JSON-LD parser not yet implemented".to_string(),
        ))
    }

    fn parse_file(&self, _path: &Path) -> OwlResult<Ontology> {
        Err(OwlError::ParseError(
            "JSON-LD parser not yet implemented".to_string(),
        ))
    }

    fn format_name(&self) -> &'static str {
        "JSON-LD"
    }
}

impl Default for JsonLdParser {
    fn default() -> Self {
        Self::new()
    }
}
