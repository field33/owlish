use crate::owl::{ObjectPropertyConstructor, ObjectPropertyIRI};

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ObjectInverseOf(pub ObjectPropertyIRI);

impl From<ObjectInverseOf> for ObjectPropertyConstructor {
    fn from(c: ObjectInverseOf) -> Self {
        Self::ObjectInverseOf(c)
    }
}

#[cfg(feature = "wasm")]
mod wasm {
    use wasm_bindgen::prelude::wasm_bindgen;
    #[wasm_bindgen(typescript_custom_section)]
    const WASM_API: &'static str = r#"
export type ObjectInverseOf = IRI
"#;
}
