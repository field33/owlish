use crate::owl::{IndividualIRI, ObjectPropertyConstructor, IRI};

#[derive(Debug, Clone, Eq, PartialEq)]
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

#[derive(Debug, Eq, PartialEq)]
pub struct ObjectPropertyAssertion(pub ObjectPropertyIRI, pub IndividualIRI, pub IndividualIRI);
#[derive(Debug, Eq, PartialEq)]
pub struct NegativeObjectPropertyAssertion(
    pub ObjectPropertyIRI,
    pub IndividualIRI,
    pub IndividualIRI,
);
