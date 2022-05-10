use crate::owl::*;

pub enum Declaration {
    Class(ClassIRI),
    NamedIndividual(IndividualIRI),
    ObjectProperty(ObjectPropertyIRI),
    DataProperty(DataPropertyIRI),
    Datatype(DatatypeIRI),
}

pub struct Ontology {
    declarations: Vec<Declaration>,
    axioms: Vec<Axiom>,
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
}
