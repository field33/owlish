use wasm_bindgen::prelude::wasm_bindgen;

use crate::owl::{Axiom, ObjectPropertyIRI};

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct SymmetricObjectProperty(pub ObjectPropertyIRI);

impl From<SymmetricObjectProperty> for Axiom {
    fn from(sop: SymmetricObjectProperty) -> Self {
        Self::SymmetricObjectProperty(sop)
    }
}

#[wasm_bindgen(typescript_custom_section)]
const WASM_API: &'static str = r#"
/**
 * ObjectProperty IRI
 */
export type SymmetricObjectProperty = IRI;
"#;
