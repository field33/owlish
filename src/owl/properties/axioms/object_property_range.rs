use crate::owl::{Axiom, ClassIRI, ObjectPropertyIRI};

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ObjectPropertyRange(pub ObjectPropertyIRI, pub ClassIRI);

impl From<ObjectPropertyRange> for Axiom {
    fn from(opr: ObjectPropertyRange) -> Self {
        Axiom::ObjectPropertyRange(opr)
    }
}

#[cfg(feature = "wasm")]
mod wasm {
    use wasm_bindgen::prelude::wasm_bindgen;
    #[wasm_bindgen(typescript_custom_section)]
    const WASM_API: &'static str = r#"
/**
 * [ObjectProperty IRI, Class IRI]
 */
export type ObjectPropertyRange = [IRI, IRI];
"#;
}
