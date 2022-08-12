use serde::{Deserialize, Serialize};

use crate::owl::{Axiom, IndividualIRI, ObjectPropertyConstructor, IRI};

use super::Annotation;

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
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

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ObjectPropertyAssertion(
    pub ObjectPropertyIRI,
    pub IndividualIRI,
    pub IndividualIRI,
    pub Vec<Annotation>,
);

impl ObjectPropertyAssertion {
    pub fn iri(&self) -> &ObjectPropertyIRI {
        &self.0
    }
    pub fn subject(&self) -> &IndividualIRI {
        &self.1
    }
    pub fn object(&self) -> &IndividualIRI {
        &self.2
    }
    pub fn annotations(&self) -> &Vec<Annotation> {
        &self.3
    }
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct NegativeObjectPropertyAssertion(
    pub ObjectPropertyIRI,
    pub IndividualIRI,
    pub IndividualIRI,
    pub Vec<Annotation>,
);

impl NegativeObjectPropertyAssertion {
    pub fn iri(&self) -> &ObjectPropertyIRI {
        &self.0
    }
    pub fn subject(&self) -> &IndividualIRI {
        &self.1
    }
    pub fn object(&self) -> &IndividualIRI {
        &self.2
    }
    pub fn annotations(&self) -> &Vec<Annotation> {
        &self.3
    }
}

impl From<ObjectPropertyAssertion> for Axiom {
    fn from(opa: ObjectPropertyAssertion) -> Self {
        Self::ObjectPropertyAssertion(opa)
    }
}

#[cfg(feature = "wasm")]
mod wasm {
    use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen(typescript_custom_section)]
    const WASM_API1: &'static str = r#"
/**
 * [ObjectProperty IRI, Individual IRI, Individual IRI]
 */
export type ObjectPropertyAssertion = [IRI, IRI, IRI]
"#;

    #[wasm_bindgen(typescript_custom_section)]
    const WASM_API2: &'static str = r#"
/**
 * [ObjectProperty IRI, Individual IRI, Individual IRI]
 */
export type NegativeObjectPropertyAssertion = [IRI, IRI, IRI]
"#;
}
