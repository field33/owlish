use crate::owl::{AnnotationAssertion, Axiom, ClassConstructor, IndividualIRI};

pub struct Individual<'a> {
    pub(crate) iri: &'a IndividualIRI,
    pub(crate) axioms: Vec<&'a Axiom>,
}
impl<'a> Individual<'a> {
    pub fn iri(&self) -> &IndividualIRI {
        self.iri
    }

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

    /// Get all classes associated with this individual.
    pub fn classes(&self) -> Vec<&ClassConstructor> {
        self.axioms
            .iter()
            .filter_map(|a| match a {
                Axiom::ClassAssertion(a) => Some(a.class_constructor()),
                _ => None,
            })
            .collect()
    }
}
