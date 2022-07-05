use std::fmt::Display;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::owl::{Axiom, ClassIRI, DatatypeIRI, Regards, IRI};

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
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

#[derive(Debug, Clone, Deserialize, Serialize)]
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
                Value::String(i) => i == iri.as_str(),
                _ => false,
            }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Annotation(pub AnnotationPropertyIRI, pub Value);

impl Annotation {
    pub fn iri(&self) -> &AnnotationPropertyIRI {
        &self.0
    }
    pub fn value(&self) -> &Value {
        &self.1
    }
}

impl From<AnnotationAssertion> for Axiom {
    fn from(aa: AnnotationAssertion) -> Self {
        Axiom::AnnotationAssertion(aa)
    }
}
