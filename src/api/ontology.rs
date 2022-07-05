use std::collections::HashMap;

use super::Individual;
use crate::api::Class;
use crate::owl::{
    Axiom, ClassIRI, Declaration, IRIBuilder, IndividualIRI, ObjectPropertyAssertion, Regards, IRI,
};
use wasm_bindgen::prelude::*;

/// The field33 representation of an ontology
///
/// This is a concatenation of multiple concepts used by field33 to represent wisdom:
/// - An OWL2 ontology that may contain
///   - Classes
///   - Individuals
///   - Annotation(Properties)
///   - DataProperties
///   - ObjectProperties
#[wasm_bindgen]
#[derive(Debug)]
pub struct Ontology {
    pub(crate) iri: IRI,
    pub(crate) imports: HashMap<String, IRI>,
    pub(crate) owl: crate::owl::Ontology,
}

impl Ontology {
    /// Creates a new Ontology
    pub fn new(iri: IRI) -> Self {
        Self {
            iri,
            imports: Default::default(),
            owl: crate::owl::Ontology::new(vec![], vec![]),
        }
    }

    pub fn set_owl(&mut self, owl: crate::owl::Ontology) {
        self.owl = owl
    }

    pub fn iri_builder(&self) -> IRIBuilder {
        IRIBuilder::construct(self.iri.clone(), &self.imports)
    }

    /// Get the class of the given IRI.
    pub fn class(&self, iri: &ClassIRI) -> Option<Class> {
        let mut declaration = None;
        for d in self.owl.declarations() {
            if let Declaration::Class(class_iri) = d {
                if class_iri == iri {
                    declaration = Some(class_iri);
                }
            }
        }
        let mut axioms = Vec::new();
        for axiom in self.owl.axioms() {
            if axiom.regards(iri.as_iri()) {
                axioms.push(axiom);
            }
        }
        Some(Class {
            iri: declaration?,
            axioms,
        })
    }

    /// Get all classes in this ontology.
    pub fn classes(&self) -> Vec<Class> {
        let mut classes = Vec::new();
        for d in self.owl.declarations() {
            if let Declaration::Class(class_iri) = d {
                let mut axioms = Vec::new();
                for axiom in self.owl.axioms() {
                    if axiom.regards(class_iri.as_iri()) {
                        axioms.push(axiom);
                    }
                }
                classes.push(Class {
                    iri: class_iri,
                    axioms,
                })
            }
        }
        classes
    }

    /// Get the Individual with the given IRI.
    pub fn individual(&self, iri: &IndividualIRI) -> Option<Individual> {
        let mut declaration = None;
        for d in self.owl.declarations() {
            if let Declaration::NamedIndividual(individual_iri) = d {
                if individual_iri == iri {
                    declaration = Some(individual_iri);
                }
            }
        }
        let mut axioms = Vec::new();
        for axiom in self.owl.axioms() {
            if axiom.regards(iri.as_iri()) {
                axioms.push(axiom);
            }
        }

        Some(Individual {
            iri: declaration?,
            axioms,
        })
    }

    /// Get all individuals in this ontology.
    pub fn individuals(&self) -> Vec<Individual> {
        let mut individuals = Vec::new();
        for d in self.owl.declarations() {
            if let Declaration::NamedIndividual(individual_iri) = d {
                let mut axioms = Vec::new();
                for axiom in self.owl.axioms() {
                    if axiom.regards(individual_iri.as_iri()) {
                        axioms.push(axiom);
                    }
                }
                individuals.push(Individual {
                    iri: individual_iri,
                    axioms,
                })
            }
        }
        individuals
    }
}

impl Ontology {
    pub fn declarations(&self) -> &Vec<Declaration> {
        &self.owl.declarations
    }

    pub fn axioms(&self) -> &Vec<Axiom> {
        &self.owl.axioms
    }
}

impl From<(IRI, crate::owl::Ontology)> for Ontology {
    fn from((iri, owl): (IRI, crate::owl::Ontology)) -> Self {
        Self {
            iri,
            imports: Default::default(),
            owl,
        }
    }
}
