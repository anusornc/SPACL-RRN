//! Manchester Syntax Parser (stub implementation)
use crate::core::error::{OwlError, OwlResult};
use crate::core::ontology::Ontology;
use crate::parser::OntologyParser;
use std::path::Path;

pub struct ManchesterParser;
pub struct ManchesterAST;

impl ManchesterParser {
    pub fn new() -> Self {
        Self
    }
}

impl OntologyParser for ManchesterParser {
    fn parse_str(&self, _content: &str) -> OwlResult<Ontology> {
        Err(OwlError::ParseError(
            "Manchester Syntax parser not yet implemented".to_string(),
        ))
    }

    fn parse_file(&self, _path: &Path) -> OwlResult<Ontology> {
        Err(OwlError::ParseError(
            "Manchester Syntax parser not yet implemented".to_string(),
        ))
    }

    fn format_name(&self) -> &'static str {
        "Manchester OWL Syntax"
    }
}

impl Default for ManchesterParser {
    fn default() -> Self {
        Self::new()
    }
}
