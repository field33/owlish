use crate::owl::{Annotation, Literal};

use super::DatatypeDefinitionConstructor;

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DataOneOf {
    pub literals: Vec<Literal>,
    pub annotations: Vec<Annotation>,
}

impl DataOneOf {
    pub fn new(literals: Vec<Literal>, annotations: Vec<Annotation>) -> Self {
        Self {
            literals,
            annotations,
        }
    }
}

impl From<DataOneOf> for Box<DatatypeDefinitionConstructor> {
    fn from(c: DataOneOf) -> Self {
        DatatypeDefinitionConstructor::DataOneOf(c).into()
    }
}
impl From<DataOneOf> for DatatypeDefinitionConstructor {
    fn from(c: DataOneOf) -> Self {
        DatatypeDefinitionConstructor::DataOneOf(c)
    }
}

#[cfg(feature = "wasm")]
mod wasm {
    use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen(typescript_custom_section)]
    const WASM_API: &'static str = r#"
/**
 * List of literals.
 */
export type DataOneOf = {
    literals: Array<Value>,
    annotations: Array<Annotation>,
};
"#;
}
