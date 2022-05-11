use crate::owl::{IndividualIRI, Value, IRI};

mod constructors;
pub use constructors::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DatatypeIRI(pub IRI);
impl From<IRI> for DatatypeIRI {
    fn from(iri: IRI) -> Self {
        Self(iri)
    }
}
impl DatatypeIRI {
    pub fn as_iri(&self) -> &IRI {
        &self.0
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DataPropertyIRI(IRI);
impl From<IRI> for DataPropertyIRI {
    fn from(iri: IRI) -> Self {
        Self(iri)
    }
}
impl DataPropertyIRI {
    pub fn as_iri(&self) -> &IRI {
        &self.0
    }
}
#[derive(Debug)]
pub struct DataPropertyAssertion(pub DataPropertyIRI, pub IndividualIRI, pub Value);
#[derive(Debug)]
pub struct NegativeDataPropertyAssertion(pub DataPropertyIRI, pub IndividualIRI, pub Value);
