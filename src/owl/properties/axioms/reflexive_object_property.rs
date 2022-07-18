use crate::owl::ObjectPropertyIRI;

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ReflexiveObjectProperty(pub ObjectPropertyIRI);

#[cfg(feature = "wasm")]
mod wasm {
    use wasm_bindgen::prelude::wasm_bindgen;
    #[wasm_bindgen(typescript_custom_section)]
    const WASM_API: &'static str = r#"
/**
 * ObjectProperty IRI
 */
export type ReflexiveObjectProperty = IRI;
"#;
}
