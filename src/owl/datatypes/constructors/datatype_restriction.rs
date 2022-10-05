use super::DatatypeDefinitionConstructor;
use crate::owl::{Annotation, DatatypeIRI, Literal};

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Restriction {
    Numeric(
        #[serde(rename = "iri")] DatatypeIRI,
        #[serde(rename = "value")] Literal,
    ),
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DatatypeRestriction {
    #[serde(rename = "datatypeIRI")]
    pub datatype_iri: DatatypeIRI,
    #[serde(rename = "restrictions")]
    pub restrictions: Vec<Restriction>,
    #[serde(rename = "annotations")]
    pub annotations: Vec<Annotation>,
}

impl DatatypeRestriction {
    pub fn new(
        datatype_iri: DatatypeIRI,
        restrictions: Vec<Restriction>,
        annotations: Vec<Annotation>,
    ) -> Self {
        Self {
            datatype_iri,
            restrictions,
            annotations,
        }
    }
}

impl From<DatatypeRestriction> for Box<DatatypeDefinitionConstructor> {
    fn from(c: DatatypeRestriction) -> Self {
        DatatypeDefinitionConstructor::DatatypeRestriction(c).into()
    }
}
impl From<DatatypeRestriction> for DatatypeDefinitionConstructor {
    fn from(c: DatatypeRestriction) -> Self {
        DatatypeDefinitionConstructor::DatatypeRestriction(c)
    }
}

#[cfg(feature = "wasm")]
mod wasm {
    use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen(typescript_custom_section)]
    const WASM_API1: &'static str = r#"
export type Restriction = { Numeric: {iri: IRI, value: number} };
"#;

    #[wasm_bindgen(typescript_custom_section)]
    const WASM_API2: &'static str = r#"
export type DatatypeRestriction = {
    datatypeIRI: IRI, 
    restrictions: Array<Restriction>,
    annotations: Array<Annotation>,
};
"#;
}
