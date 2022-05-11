use std::fmt::Display;

use crate::owl::{ClassIRI, DatatypeIRI, Regards, Value, IRI};

#[derive(Debug, Eq, PartialEq)]
pub struct AnnotationPropertyIRI(IRI);
impl AnnotationPropertyIRI {
    pub fn as_iri(&self) -> &IRI {
        &self.0
    }
}
impl From<IRI> for AnnotationPropertyIRI {
    fn from(iri: IRI) -> Self {
        Self(iri)
    }
}
impl Display for AnnotationPropertyIRI {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
pub struct AnnotationPropertyDomain(pub(crate) AnnotationPropertyIRI, pub(crate) ClassIRI);

#[derive(Debug)]
pub struct AnnotationPropertyRange(pub(crate) AnnotationPropertyIRI, pub(crate) DatatypeIRI);

#[derive(Debug)]
pub struct AnnotationAssertion(
    pub(crate) AnnotationPropertyIRI,
    pub(crate) IRI,
    pub(crate) Value,
);
impl AnnotationAssertion {
    pub fn iri(&self) -> &AnnotationPropertyIRI {
        &self.0
    }
    pub fn subject(&self) -> &IRI {
        &self.1
    }
    pub fn value(&self) -> &Value {
        &self.2
    }
}
impl Regards for AnnotationAssertion {
    fn regards(&self, iri: &IRI) -> bool {
        self.iri().as_iri() == iri
            || self.subject() == iri
            || match self.value() {
                Value::String(_) => false,
                Value::Integer(_) => false,
                Value::NonNegativeInteger(_) => false,
                Value::IRI(i) => i == iri,
            }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Annotation(pub AnnotationPropertyIRI, pub Value);

impl Annotation {
    pub fn iri(&self) -> &AnnotationPropertyIRI {
        &self.0
    }
    pub fn value(&self) -> &Value {
        &self.1
    }
}
