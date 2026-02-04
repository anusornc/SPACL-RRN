//! OWL Functional Syntax Parser (stub implementation)
use crate::core::error::{OwlError, OwlResult};
use crate::core::ontology::Ontology;
use crate::parser::OntologyParser;
use std::path::Path;

pub struct OwlFunctionalSyntaxParser;

impl OwlFunctionalSyntaxParser {
    pub fn new() -> Self {
        Self
    }
}

impl OntologyParser for OwlFunctionalSyntaxParser {
    fn parse_str(&self, _content: &str) -> OwlResult<Ontology> {
        Err(OwlError::ParseError(
            "OWL Functional Syntax parser not yet implemented".to_string(),
        ))
    }

    fn parse_file(&self, _path: &Path) -> OwlResult<Ontology> {
        Err(OwlError::ParseError(
            "OWL Functional Syntax parser not yet implemented".to_string(),
        ))
    }

    fn format_name(&self) -> &'static str {
        "OWL Functional Syntax"
    }
}

impl Default for OwlFunctionalSyntaxParser {
    fn default() -> Self {
        Self::new()
    }
}
