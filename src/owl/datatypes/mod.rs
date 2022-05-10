mod datatype_definition;
pub use datatype_definition::*;

mod data_some_values_from;
pub use data_some_values_from::*;

use crate::owl::{IndividualIRI, Value, IRI};

#[derive(Debug, Clone)]
pub struct DatatypeIRI(pub IRI);
impl From<IRI> for DatatypeIRI {
    fn from(iri: IRI) -> Self {
        Self(iri)
    }
}

#[derive(Debug, Clone)]
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
pub struct DataPropertyAssertion(
    pub(crate) DataPropertyIRI,
    pub(crate) IndividualIRI,
    pub(crate) Value,
);
#[derive(Debug)]
pub struct NegativeDataPropertyAssertion(
    pub(crate) DataPropertyIRI,
    pub(crate) IndividualIRI,
    pub(crate) Value,
);
