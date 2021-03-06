use crate::owl::{IndividualIRI, IRI};

mod constructors;
pub use constructors::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
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

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
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
#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DataPropertyAssertion(pub DataPropertyIRI, pub IndividualIRI, pub Value);

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct NegativeDataPropertyAssertion(pub DataPropertyIRI, pub IndividualIRI, pub Value);

#[cfg(feature = "wasm")]
mod wasm {
    use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen(typescript_custom_section)]
    const WASM_API_1: &'static str = r#"
/**
 * [DataProperty IRI, Individual IRI, Value]
 */
export type DataPropertyAssertion = [IRI, IRI, unknown]
"#;

    #[wasm_bindgen(typescript_custom_section)]
    const WASM_API_2: &'static str = r#"
/**
 * [DataProperty IRI, Individual IRI, Value]
 */
export type NegativeDataPropertyAssertion = [IRI, IRI, unknown]
"#;
}
