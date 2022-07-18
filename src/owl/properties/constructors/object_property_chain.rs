use crate::owl::{ObjectPropertyConstructor, ObjectPropertyIRI};

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ObjectPropertyChain(pub Vec<ObjectPropertyIRI>);

impl From<ObjectPropertyChain> for ObjectPropertyConstructor {
    fn from(c: ObjectPropertyChain) -> Self {
        Self::ObjectPropertyChain(c)
    }
}

#[cfg(feature = "wasm")]
mod wasm {
    use wasm_bindgen::prelude::wasm_bindgen;
    #[wasm_bindgen(typescript_custom_section)]
    const WASM_API: &'static str = r#"
export type ObjectPropertyChain = Array<IRI>
"#;
}
