use crate::owl::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum Declaration {
    Class(ClassIRI),
    NamedIndividual(IndividualIRI),
    ObjectProperty(ObjectPropertyIRI),
    DataProperty(DataPropertyIRI),
    Datatype(DatatypeIRI),
}

#[derive(Debug)]
pub struct Ontology {
    pub(crate) declarations: Vec<Declaration>,
    pub(crate) axioms: Vec<Axiom>,
}

impl Ontology {
    pub fn new(declarations: Vec<Declaration>, axioms: Vec<Axiom>) -> Self {
        Self {
            declarations,
            axioms,
        }
    }
    pub fn declarations(&self) -> &Vec<Declaration> {
        &self.declarations
    }
    pub fn axioms(&self) -> &Vec<Axiom> {
        &self.axioms
    }
    pub fn axioms_mut(&mut self) -> &mut Vec<Axiom> {
        &mut self.axioms
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_ser_de_declaration() {
        let d = Declaration::Class(IRI::new("http://example.com").unwrap().into());
        let json = serde_json::to_string(&d).unwrap();
        assert_eq!(
            json,
            r#"{"Class":{"_type":"IRI","string":"http://example.com"}}"#
        );
    }
}
