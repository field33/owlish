use std::collections::HashMap;

use crate::{
    api::{Ontology, IRI},
    owl::{well_known, Axiom, ClassAssertion, ClassConstructor},
};

pub struct Computation {
    iri: IRI,
    axioms: Vec<Axiom>,
}

impl Computation {
    pub fn iri(&self) -> &IRI {
        &self.iri
    }
    pub fn axioms(&self) -> &Vec<Axiom> {
        &self.axioms
    }
}

trait GetComputations {
    fn computations(&self) -> Vec<Computation>;
}

impl GetComputations for Ontology {
    fn computations(&self) -> Vec<Computation> {
        let mut computations: HashMap<IRI, Computation> = HashMap::new();

        for axiom in self.owl.axioms() {
            if let Axiom::ClassAssertion(ClassAssertion {
                cls,
                individual,
                annotations: _,
            }) = axiom
            {
                let individual = individual.as_iri();
                if let ClassConstructor::IRI(iri) = cls {
                    let iri = iri.as_iri();
                    if iri == &well_known::fno_Function() {
                        computations.insert(
                            individual.clone(),
                            Computation {
                                iri: individual.clone(),
                                axioms: Vec::new(),
                            },
                        );
                    }
                }
            }
        }

        for axiom in self.owl.axioms() {
            if let Some(subject) = axiom.subject() {
                if let Some(comp) = computations.get_mut(subject) {
                    comp.axioms.push(axiom.clone());
                }
            }
        }
        computations.into_values().collect()
    }
}
