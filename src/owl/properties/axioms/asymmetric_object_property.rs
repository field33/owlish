use wasm_bindgen::prelude::wasm_bindgen;

use crate::owl::{Axiom, ObjectPropertyIRI};

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct AsymmetricObjectProperty(pub ObjectPropertyIRI);

impl From<AsymmetricObjectProperty> for Axiom {
    fn from(aop: AsymmetricObjectProperty) -> Self {
        Self::AsymmetricObjectProperty(aop)
    }
}

#[wasm_bindgen(typescript_custom_section)]
const WASM_API: &'static str = r#"
/**
 * ObjectProperty IRI
 */
export type AsymmetricObjectProperty = IRI;
"#;
