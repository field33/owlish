use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::owl::{ObjectPropertyConstructor, ObjectPropertyIRI};

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct SubObjectPropertyOf(pub ObjectPropertyConstructor, pub ObjectPropertyIRI);

#[wasm_bindgen(typescript_custom_section)]
const WASM_API: &'static str = r#"
/**
 * [ObjectPropertyConstructor, ObjectProperty IRI]
 */
export type SubObjectPropertyOf = [ObjectPropertyConstructor, IRI];
"#;
