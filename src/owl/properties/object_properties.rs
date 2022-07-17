use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::owl::{Axiom, IndividualIRI, ObjectPropertyConstructor, IRI};

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
pub struct ObjectPropertyAssertion(pub ObjectPropertyIRI, pub IndividualIRI, pub IndividualIRI);

#[wasm_bindgen(typescript_custom_section)]
const WASM_API: &'static str = r#"
/**
 * [ObjectProperty IRI, Individual IRI, Individual IRI]
 */
export type ObjectPropertyAssertion = [IRI, IRI, IRI]
"#;

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct NegativeObjectPropertyAssertion(
    pub ObjectPropertyIRI,
    pub IndividualIRI,
    pub IndividualIRI,
);

#[wasm_bindgen(typescript_custom_section)]
const WASM_API: &'static str = r#"
/**
 * [ObjectProperty IRI, Individual IRI, Individual IRI]
 */
export type NegativeObjectPropertyAssertion = [IRI, IRI, IRI]
"#;

impl From<ObjectPropertyAssertion> for Axiom {
    fn from(opa: ObjectPropertyAssertion) -> Self {
        Self::ObjectPropertyAssertion(opa)
    }
}
