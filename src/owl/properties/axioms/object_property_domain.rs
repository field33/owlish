use wasm_bindgen::prelude::wasm_bindgen;

use crate::owl::{Axiom, ClassIRI, ObjectPropertyIRI};

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ObjectPropertyDomain(pub ObjectPropertyIRI, pub ClassIRI);

impl From<ObjectPropertyDomain> for Axiom {
    fn from(opd: ObjectPropertyDomain) -> Self {
        Axiom::ObjectPropertyDomain(opd)
    }
}

#[wasm_bindgen(typescript_custom_section)]
const WASM_API: &'static str = r#"
/**
 * [ObjectProperty IRI, Class IRI]
 */
export type ObjectPropertyDomain = [IRI, IRI];
"#;
