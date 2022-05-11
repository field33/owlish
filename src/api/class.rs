use crate::owl::{AnnotationAssertion, Axiom, ClassIRI, DataPropertyDomain, SubClassOf};

#[derive(Debug)]
pub struct Class<'a> {
    pub(crate) iri: &'a ClassIRI,
    pub(crate) axioms: Vec<&'a Axiom>,
}

impl<'a> Class<'a> {
    /// Get all annotations asserted with this class
    pub fn annotations(&self) -> Vec<&AnnotationAssertion> {
        self.axioms
            .iter()
            .filter_map(|a| match a {
                Axiom::AnnotationAssertion(a) => Some(a),
                _ => None,
            })
            .collect()
    }

    /// Get all direct super classes of this class
    pub fn super_classes(&self) -> Vec<&'a SubClassOf> {
        self.axioms
            .iter()
            .filter_map(|a| match a {
                Axiom::SubClassOf(sco) => {
                    if sco.subject().is_iri(self.iri.as_iri()) {
                        Some(sco)
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .collect()
    }

    /// Get all DataProperties whose domain contains this class.
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
