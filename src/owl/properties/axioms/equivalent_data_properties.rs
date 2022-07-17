use wasm_bindgen::prelude::wasm_bindgen;

use crate::owl::DataPropertyIRI;

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct EquivalentDataProperties(pub DataPropertyIRI, pub DataPropertyIRI);

#[wasm_bindgen(typescript_custom_section)]
const WASM_API: &'static str = r#"
/**
 * [DataProperty IRI, DataProperty IRI]
 */
export type EquivalentDataProperties = [IRI, IRI];
"#;
