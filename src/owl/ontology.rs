use crate::owl::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub enum Declaration {
    Class {
        iri: ClassIRI,
        annotations: Vec<Annotation>,
    },
    NamedIndividual {
        iri: IndividualIRI,
        annotations: Vec<Annotation>,
    },
    ObjectProperty {
        iri: ObjectPropertyIRI,
        annotations: Vec<Annotation>,
    },
    DataProperty {
        iri: DataPropertyIRI,
        annotations: Vec<Annotation>,
    },
    AnnotationProperty {
        iri: AnnotationPropertyIRI,
        annotations: Vec<Annotation>,
    },
    Datatype {
        iri: DatatypeIRI,
        annotations: Vec<Annotation>,
    },
}

impl Declaration {
    pub fn annotations(&self) -> &Vec<Annotation> {
        match &self {
            Declaration::Class {
                iri: _,
                annotations,
            } => annotations,
            Declaration::NamedIndividual {
                iri: _,
                annotations,
            } => annotations,
            Declaration::ObjectProperty {
                iri: _,
                annotations,
            } => annotations,
            Declaration::DataProperty {
                iri: _,
                annotations,
            } => annotations,
            Declaration::AnnotationProperty {
                iri: _,
                annotations,
            } => annotations,
            Declaration::Datatype {
                iri: _,
                annotations,
            } => annotations,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
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
    pub fn declarations_mut(&mut self) -> &mut Vec<Declaration> {
        &mut self.declarations
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_ser_de_declaration() {
        let d = Declaration::Class {
            iri: IRI::new("http://example.com").unwrap().into(),
            annotations: vec![],
        };
        let json = serde_json::to_string(&d).unwrap();
        assert_eq!(
            json,
            r#"{"Class":[{"_type":"IRI","string":"http://example.com"},[]]}"#
        );
    }
}
