use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct NegativeObjectPropertyAssertion(
    pub ObjectPropertyIRI,
    pub IndividualIRI,
    pub IndividualIRI,
);

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
