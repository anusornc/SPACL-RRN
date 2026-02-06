//! Binary serialization format for Ontologies
//!
//! This format is designed for fast loading of large ontologies.
//! It is NOT a standard format - it's an internal optimization.
//!
//! Format version 1:
//! ```
//! [Header] (32 bytes)
//!   - magic: [u8; 4] = b"OWLB"
//!   - version: u32 = 1
//!   - class_count: u64
//!   - property_count: u64
//!   - axiom_count: u64
//!   - reserved: [u8; 8]
//!
//! [String Table]
//!   - string_count: u64
//!   - strings: [length: u32, bytes: [u8; length]]*
//!
//! [Classes Section]
//!   - class_iris: [string_id: u64]*
//!
//! [Object Properties Section]
//!   - property_iris: [string_id: u64]*
//!
//! [Axioms Section]
//!   - axioms: [type: u8, data: ...]*
//! ```

use std::io::{Read, Write, Result as IoResult};


use crate::core::iri::IRI;
use crate::core::ontology::Ontology;
use crate::logic::axioms::Axiom;

const MAGIC: &[u8; 4] = b"OWLB";
const VERSION: u32 = 1;
const HEADER_SIZE: usize = 32;

/// Binary ontology format for fast serialization
pub struct BinaryOntologyFormat;

impl BinaryOntologyFormat {
    /// Serialize an ontology to binary format
    pub fn serialize<W: Write>(ontology: &Ontology, writer: &mut W) -> IoResult<()> {
        // Collect all strings for string table
        let mut string_table = StringTable::new();
        
        // Build string table from classes
        for class in ontology.classes() {
            string_table.add(class.iri().as_str());
        }
        
        // Build string table from properties
        for prop in ontology.object_properties() {
            string_table.add(prop.iri().as_str());
        }
        
        // Write header
        Self::write_header(
            writer,
            ontology.classes().len() as u64,
            ontology.object_properties().len() as u64,
            ontology.axioms().len() as u64,
        )?;
        
        // Write string table
        string_table.write(writer)?;
        
        // Write classes
        Self::write_classes(writer, ontology, &string_table)?;
        
        // Write properties
        Self::write_properties(writer, ontology, &string_table)?;
        
        // Write axioms (simplified - just count for now)
        Self::write_axioms_placeholder(writer, ontology)?;
        
        Ok(())
    }
    
    /// Deserialize an ontology from binary format
    pub fn deserialize<R: Read>(reader: &mut R) -> IoResult<Ontology> {
        // Read and verify header
        let header = Self::read_header(reader)?;
        
        if &header.magic != MAGIC {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid binary format: wrong magic number"
            ));
        }
        
        if header.version != VERSION {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Unsupported version: {}", header.version)
            ));
        }
        
        // Read string table
        let string_table = StringTable::read(reader)?;
        
        // Create new ontology
        let mut ontology = Ontology::new();
        
        // Read classes
        Self::read_classes(reader, &mut ontology, &string_table, header.class_count)?;
        
        // Read properties
        Self::read_properties(reader, &mut ontology, &string_table, header.property_count)?;
        
        // Read axioms (placeholder for now)
        Self::read_axioms_placeholder(reader, &mut ontology, header.axiom_count)?;
        
        Ok(ontology)
    }
    
    // Private helper methods
    
    fn write_header<W: Write>(
        writer: &mut W,
        class_count: u64,
        property_count: u64,
        axiom_count: u64,
    ) -> IoResult<()> {
        writer.write_all(MAGIC)?;
        writer.write_all(&VERSION.to_le_bytes())?;
        writer.write_all(&class_count.to_le_bytes())?;
        writer.write_all(&property_count.to_le_bytes())?;
        writer.write_all(&axiom_count.to_le_bytes())?;
        writer.write_all(&[0u8; 8])?; // reserved
        Ok(())
    }
    
    fn read_header<R: Read>(reader: &mut R) -> IoResult<Header> {
        let mut magic = [0u8; 4];
        reader.read_exact(&mut magic)?;
        
        let mut version = [0u8; 4];
        reader.read_exact(&mut version)?;
        let version = u32::from_le_bytes(version);
        
        let mut class_count = [0u8; 8];
        reader.read_exact(&mut class_count)?;
        let class_count = u64::from_le_bytes(class_count);
        
        let mut property_count = [0u8; 8];
        reader.read_exact(&mut property_count)?;
        let property_count = u64::from_le_bytes(property_count);
        
        let mut axiom_count = [0u8; 8];
        reader.read_exact(&mut axiom_count)?;
        let axiom_count = u64::from_le_bytes(axiom_count);
        
        let mut reserved = [0u8; 8];
        reader.read_exact(&mut reserved)?;
        
        Ok(Header {
            magic,
            version,
            class_count,
            property_count,
            axiom_count,
            _reserved: reserved,
        })
    }
    
    fn write_classes<W: Write>(
        writer: &mut W,
        ontology: &Ontology,
        string_table: &StringTable,
    ) -> IoResult<()> {
        for class in ontology.classes() {
            let id = string_table.get_id(class.iri().as_str())
                .expect("IRI not in string table");
            writer.write_all(&id.to_le_bytes())?;
        }
        Ok(())
    }
    
    fn read_classes<R: Read>(
        reader: &mut R,
        ontology: &mut Ontology,
        string_table: &StringTable,
        count: u64,
    ) -> IoResult<()> {
        use crate::core::entities::Class;
        
        for _ in 0..count {
            let mut id_bytes = [0u8; 8];
            reader.read_exact(&mut id_bytes)?;
            let id = u64::from_le_bytes(id_bytes);
            
            let iri_str = string_table.get_string(id)
                .ok_or_else(|| std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Invalid string ID"
                ))?;
            
            let iri = IRI::new(iri_str);
            let class = Class::new(iri);
            ontology.add_class(class).map_err(|e| {
                std::io::Error::new(std::io::ErrorKind::Other, format!("{:?}", e))
            })?;
        }
        Ok(())
    }
    
    fn write_properties<W: Write>(
        _writer: &mut W,
        _ontology: &Ontology,
        _string_table: &StringTable,
    ) -> IoResult<()> {
        // Placeholder - full implementation later
        Ok(())
    }
    
    fn read_properties<R: Read>(
        _reader: &mut R,
        _ontology: &mut Ontology,
        _string_table: &StringTable,
        _count: u64,
    ) -> IoResult<()> {
        // Placeholder - full implementation later
        Ok(())
    }
    
    fn write_axioms_placeholder<W: Write>(
        _writer: &mut W,
        _ontology: &Ontology,
    ) -> IoResult<()> {
        // Placeholder - full implementation later
        Ok(())
    }
    
    fn read_axioms_placeholder<R: Read>(
        _reader: &mut R,
        _ontology: &mut Ontology,
        _count: u64,
    ) -> IoResult<()> {
        // Placeholder - full implementation later
        Ok(())
    }
}

#[derive(Debug)]
struct Header {
    magic: [u8; 4],
    version: u32,
    class_count: u64,
    property_count: u64,
    axiom_count: u64,
    _reserved: [u8; 8],
}

/// String table for deduplication
struct StringTable {
    strings: Vec<String>,
    id_map: std::collections::HashMap<String, u64>,
}

impl StringTable {
    fn new() -> Self {
        Self {
            strings: Vec::new(),
            id_map: std::collections::HashMap::new(),
        }
    }
    
    fn add(&mut self, s: &str) -> u64 {
        if let Some(&id) = self.id_map.get(s) {
            return id;
        }
        let id = self.strings.len() as u64;
        self.strings.push(s.to_string());
        self.id_map.insert(s.to_string(), id);
        id
    }
    
    fn get_id(&self, s: &str) -> Option<u64> {
        self.id_map.get(s).copied()
    }
    
    fn get_string(&self, id: u64) -> Option<&str> {
        self.strings.get(id as usize).map(|s| s.as_str())
    }
    
    fn write<W: Write>(&self, writer: &mut W) -> IoResult<()> {
        // Write count
        writer.write_all(&(self.strings.len() as u64).to_le_bytes())?;
        
        // Write each string
        for s in &self.strings {
            let bytes = s.as_bytes();
            writer.write_all(&(bytes.len() as u32).to_le_bytes())?;
            writer.write_all(bytes)?;
        }
        Ok(())
    }
    
    fn read<R: Read>(reader: &mut R) -> IoResult<Self> {
        let mut count_bytes = [0u8; 8];
        reader.read_exact(&mut count_bytes)?;
        let count = u64::from_le_bytes(count_bytes);
        
        let mut strings = Vec::with_capacity(count as usize);
        let mut id_map = std::collections::HashMap::new();
        
        for i in 0..count {
            let mut len_bytes = [0u8; 4];
            reader.read_exact(&mut len_bytes)?;
            let len = u32::from_le_bytes(len_bytes) as usize;
            
            let mut bytes = vec![0u8; len];
            reader.read_exact(&mut bytes)?;
            
            let s = String::from_utf8(bytes).map_err(|e| {
                std::io::Error::new(std::io::ErrorKind::InvalidData, e)
            })?;
            
            id_map.insert(s.clone(), i);
            strings.push(s);
        }
        
        Ok(Self { strings, id_map })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::logic::Class;
    
    #[test]
    fn test_serialize_deserialize_empty() {
        let ontology = Ontology::new();
        let mut buffer = Vec::new();
        
        BinaryOntologyFormat::serialize(&ontology, &mut buffer).unwrap();
        
        let mut reader = &buffer[..];
        let deserialized = BinaryOntologyFormat::deserialize(&mut reader).unwrap();
        
        assert_eq!(deserialized.classes().len(), 0);
    }
    
    #[test]
    fn test_serialize_deserialize_with_classes() {
        let mut ontology = Ontology::new();
        ontology.add_class(Class::new(IRI::new("http://example.org/A"))).unwrap();
        ontology.add_class(Class::new(IRI::new("http://example.org/B"))).unwrap();
        
        let mut buffer = Vec::new();
        BinaryOntologyFormat::serialize(&ontology, &mut buffer).unwrap();
        
        let mut reader = &buffer[..];
        let deserialized = BinaryOntologyFormat::deserialize(&mut reader).unwrap();
        
        assert_eq!(deserialized.classes().len(), 2);
    }
    
    #[test]
    fn test_invalid_magic() {
        let data = b"XXXX\x01\x00\x00\x00"; // Wrong magic
        let mut reader = &data[..];
        
        assert!(BinaryOntologyFormat::deserialize(&mut reader).is_err());
    }
}
