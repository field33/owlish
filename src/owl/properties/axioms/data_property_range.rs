use crate::owl::{DataPropertyIRI, DatatypeIRI};

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DataPropertyRange(pub DataPropertyIRI, pub DatatypeIRI);

#[cfg(feature = "wasm")]
mod wasm {
    use wasm_bindgen::prelude::wasm_bindgen;
    #[wasm_bindgen(typescript_custom_section)]
    const WASM_API: &'static str = r#"
/**
 * [DataProperty IRI, Datatype IRI]
 */
export type DataPropertyRange = [IRI, IRI];
"#;
}
