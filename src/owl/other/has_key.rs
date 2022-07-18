use crate::owl::{ClassIRI, ObjectPropertyIRI};

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct HasKey(pub ClassIRI, pub Vec<ObjectPropertyIRI>);

#[cfg(feature = "wasm")]
mod wasm {
    use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen(typescript_custom_section)]
    const WASM_API: &'static str = r#"
/**
 * [Class IRI, Array<ObjectProperty IRI>]
 */
export type HasKey = [IRI, Array<IRI>]
"#;
}
