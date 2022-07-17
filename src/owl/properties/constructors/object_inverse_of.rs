use wasm_bindgen::prelude::wasm_bindgen;

use crate::owl::{ObjectPropertyConstructor, ObjectPropertyIRI};

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ObjectInverseOf(pub ObjectPropertyIRI);

impl From<ObjectInverseOf> for ObjectPropertyConstructor {
    fn from(c: ObjectInverseOf) -> Self {
        Self::ObjectInverseOf(c)
    }
}

#[wasm_bindgen(typescript_custom_section)]
const WASM_API: &'static str = r#"
export type ObjectInverseOf = IRI
"#;
