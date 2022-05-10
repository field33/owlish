use crate::owl::{IndividualIRI, ObjectPropertyConstructor, IRI};

#[derive(Debug, Clone)]
pub struct ObjectPropertyIRI(IRI);
impl From<IRI> for ObjectPropertyIRI {
    fn from(iri: IRI) -> Self {
        Self(iri)
    }
}
impl From<ObjectPropertyIRI> for ObjectPropertyConstructor {
    fn from(iri: ObjectPropertyIRI) -> Self {
        Self::IRI(iri)
    }
}
impl ObjectPropertyIRI {
    pub fn as_iri(&self) -> &IRI {
        &self.0
    }
}

#[derive(Debug)]
pub struct ObjectPropertyAssertion(pub(crate) ObjectPropertyIRI, pub(crate) IndividualIRI, pub(crate) IndividualIRI);
#[derive(Debug)]
pub struct NegativeObjectPropertyAssertion(
    pub(crate) ObjectPropertyIRI,
    pub(crate) IndividualIRI,
    pub(crate) IndividualIRI,
);

#[derive(Debug)]
pub struct ObjectPropertyChain(pub Vec<ObjectPropertyIRI>);

impl From<ObjectPropertyChain> for ObjectPropertyConstructor {
    fn from(c: ObjectPropertyChain) -> Self {
        Self::ObjectPropertyChain(c)
    }
}
