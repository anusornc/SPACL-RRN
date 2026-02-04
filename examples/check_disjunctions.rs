//! Check what disjunctions are found in the ontology
use owl2_reasoner::SpeculativeTableauxReasoner;

fn main() {
    let content = std::fs::read_to_string("tests/data/disjunctive_test.owl").unwrap();
    let parser = owl2_reasoner::ParserFactory::auto_detect(&content).unwrap();
    let ontology = parser.parse_str(&content).unwrap();
    
    println!("Ontology loaded: {} classes, {} axioms", 
        ontology.classes().len(),
        ontology.axioms().len()
    );
    
    // Check for equivalent classes axioms
    let eq_axioms = ontology.equivalent_classes_axioms();
    println!("\nEquivalent classes axioms: {}", eq_axioms.len());
    
    for (i, ax) in eq_axioms.iter().enumerate() {
        println!("  Axiom {}: {:?}", i, ax);
        // Try to inspect the class expressions
        for expr in ax.class_expressions() {
            check_expression(expr, 2);
        }
    }
}

fn check_expression(expr: &owl2_reasoner::ClassExpression, indent: usize) {
    let prefix = "  ".repeat(indent);
    match expr {
        owl2_reasoner::ClassExpression::ObjectUnionOf(ops) => {
            println!("{}Found Union! ({} operands)", prefix, ops.len());
            for op in ops {
                check_expression(op, indent + 1);
            }
        }
        owl2_reasoner::ClassExpression::ObjectIntersectionOf(ops) => {
            println!("{}Found Intersection ({} operands)", prefix, ops.len());
            for op in ops {
                check_expression(op, indent + 1);
            }
        }
        owl2_reasoner::ClassExpression::Class(c) => {
            println!("{}Class: {}", prefix, c.iri());
        }
        _ => {
            println!("{}Other expression type", prefix);
        }
    }
}
