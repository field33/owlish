use wasm_bindgen::prelude::wasm_bindgen;

use crate::owl::ObjectPropertyIRI;

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct InverseObjectProperties(pub ObjectPropertyIRI, pub ObjectPropertyIRI);

#[wasm_bindgen(typescript_custom_section)]
const WASM_API: &'static str = r#"
/**
 * [ObjectProperty IRI, ObjectProperty IRI]
 */
export type InverseObjectProperties = [IRI, IRI];
"#;
