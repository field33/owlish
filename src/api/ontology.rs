use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::owl::{Axiom, Declaration, IRIBuilder, IRI};

#[cfg(feature = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Ontology {
    pub(crate) iri: IRI,
    pub(crate) imports: HashMap<String, IRI>,
    pub(crate) owl: crate::owl::Ontology,
}

#[cfg(not(feature = "wasm"))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
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

    pub fn iri(&self) -> &IRI {
        &self.iri
    }

    /// Get the map of all imports
    pub fn imports(&self) -> &HashMap<String, IRI> {
        &self.imports
    }

    /// push the given iri with name as import.
    /// If an import for this name already existed the old iri is returned.
    pub fn push_import(&mut self, name: &str, iri: IRI) -> Option<IRI> {
        self.imports.insert(name.into(), iri)
    }

    /// Get a IRIBuilder to create new iris based on imports for this ontology.
    pub fn iri_builder(&self) -> IRIBuilder {
        IRIBuilder::construct(self.iri.clone(), &self.imports)
    }

    /// Get all OWL declarations of this ontology.
    pub fn declarations(&self) -> &Vec<Declaration> {
        &self.owl.declarations
    }

    /// Get all OWL axioms of this ontology.
    pub fn axioms(&self) -> &Vec<Axiom> {
        &self.owl.axioms
    }
}

/// mutation api
impl Ontology {
    /// Set the owl data
    pub fn set_owl(&mut self, owl: crate::owl::Ontology) {
        self.owl = owl
    }

    /// Push the given OWL axiom to this ontology
    pub fn push_axiom(&mut self, axiom: Axiom) {
        self.owl.axioms.push(axiom)
    }

    /// Push the given OWL declaration to this ontology
    pub fn push_declaration(&mut self, declaration: Declaration) {
        self.owl.declarations.push(declaration)
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
