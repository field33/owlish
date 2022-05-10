use crate::owl::{
    AnnotationAssertion, Axiom, ClassConstructor, ClassIRI, DataPropertyDomain, Declaration,
    Ontology, Regards, IRI,
};

impl Ontology {
    pub fn class(&self, iri: &IRI) -> Option<Class> {
        let mut declaration = None;
        for d in self.declarations() {
            if let Declaration::Class(class_iri) = d {
                if class_iri.as_iri() == iri {
                    declaration = Some(class_iri);
                }
            }
        }
        let mut axioms = Vec::new();
        for axiom in self.axioms() {
            if axiom.regards(iri) {
                axioms.push(axiom);
            }
        }

        Some(Class {
            iri: declaration?,
            axioms,
        })
    }
}

#[derive(Debug)]
pub struct Class<'a> {
    iri: &'a ClassIRI,
    axioms: Vec<&'a Axiom>,
}

impl<'a> Class<'a> {
    pub fn annotations(&self) -> Vec<&AnnotationAssertion> {
        self.axioms
            .iter()
            .filter_map(|a| match a {
                Axiom::AnnotationAssertion(a) => Some(a),
                _ => None,
            })
            .collect()
    }

    pub fn super_classes(&self) -> Vec<&'a ClassConstructor> {
        self.axioms
            .iter()
            .filter_map(|a| match a {
                Axiom::SubClassOf(sco) => Some(sco),
                _ => None,
            })
            .filter_map(|sco| {
                if sco.subject().is_iri(self.iri.as_iri()) {
                    Some(sco.parent())
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn data_property_domains(&self) -> Vec<&'a DataPropertyDomain> {
        self.axioms
            .iter()
            .filter_map(|a| match a {
                Axiom::DataPropertyDomain(d) => Some(d),
                _ => None,
            })
            .collect()
    }
}
