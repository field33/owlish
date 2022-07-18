use crate::owl::{ObjectPropertyConstructor, ObjectPropertyIRI};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct SubObjectPropertyOf(pub ObjectPropertyConstructor, pub ObjectPropertyIRI);

#[cfg(feature = "wasm")]
mod wasm {
    use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen(typescript_custom_section)]
    const WASM_API: &'static str = r#"
/**
 * [ObjectPropertyConstructor, ObjectProperty IRI]
 */
export type SubObjectPropertyOf = [ObjectPropertyConstructor, IRI];
"#;
}
