use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::owl::{Axiom, ClassIRI, DatatypeIRI, LiteralOrIRI, Regards, IRI};

use super::Annotation;

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
pub struct AnnotationPropertyDomain(pub AnnotationPropertyIRI, pub ClassIRI, pub Vec<Annotation>);

impl AnnotationPropertyDomain {
    pub fn iri(&self) -> &AnnotationPropertyIRI {
        &self.0
    }
    pub fn class(&self) -> &ClassIRI {
        &self.1
    }
    pub fn annotations(&self) -> &Vec<Annotation> {
        &self.2
    }
}

#[derive(Debug)]
pub struct AnnotationPropertyRange(
    pub AnnotationPropertyIRI,
    pub DatatypeIRI,
    pub Vec<Annotation>,
);

impl AnnotationPropertyRange {
    pub fn iri(&self) -> &AnnotationPropertyIRI {
        &self.0
    }
    pub fn datatype(&self) -> &DatatypeIRI {
        &self.1
    }
    pub fn annotations(&self) -> &Vec<Annotation> {
        &self.2
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct AnnotationAssertion(
    pub AnnotationPropertyIRI,
    pub IRI,
    pub LiteralOrIRI,
    pub Vec<Annotation>,
);

impl AnnotationAssertion {
    pub fn iri(&self) -> &AnnotationPropertyIRI {
        &self.0
    }
    pub fn subject(&self) -> &IRI {
        &self.1
    }
    pub fn value(&self) -> &LiteralOrIRI {
        &self.2
    }
    pub fn annotations(&self) -> &Vec<Annotation> {
        &self.3
    }
}

impl Regards for AnnotationAssertion {
    fn regards(&self, iri: &IRI) -> bool {
        self.iri().as_iri() == iri
            || self.subject() == iri
            || match self.value() {
                LiteralOrIRI::IRI(i) => i == iri,
                _ => false,
            }
    }
}

impl From<AnnotationAssertion> for Axiom {
    fn from(aa: AnnotationAssertion) -> Self {
        Axiom::AnnotationAssertion(aa)
    }
}

#[cfg(feature = "wasm")]
mod wasm {
    use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen(typescript_custom_section)]
    const WASM_API1: &'static str = r#"
/**
 * [AnnotationProperty IRI, Subject IRI, value]
 */
export type AnnotationAssertion = [IRI, IRI, unknown];
"#;

    #[wasm_bindgen(typescript_custom_section)]
    const WASM_API2: &'static str = r#"
export type Annotation = [IRI, unknown];
"#;
}
