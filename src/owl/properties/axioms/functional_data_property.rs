use crate::owl::DataPropertyIRI;

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct FunctionalDataProperty(pub DataPropertyIRI);

#[cfg(feature = "wasm")]
mod wasm {
    use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen(typescript_custom_section)]
    const WASM_API: &'static str = r#"
/**
 * DataProperty IRI
 */
export type FunctionalDataProperty = IRI;
"#;
}
