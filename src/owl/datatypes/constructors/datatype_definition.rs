use super::{DataComplementOf, DataIntersectionOf, DataOneOf, DataUnionOf, DatatypeRestriction};
use crate::owl::{Annotation, DataPropertyIRI};

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum DatatypeDefinitionConstructor {
    DatatypeRestriction(DatatypeRestriction),
    DataComplementOf(DataComplementOf),
    DataIntersectionOf(DataIntersectionOf),
    DataUnionOf(DataUnionOf),
    DataOneOf(DataOneOf),
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DatatypeDefinition(
    pub DataPropertyIRI,
    pub DatatypeDefinitionConstructor,
    pub Vec<Annotation>,
);

#[cfg(feature = "wasm")]
mod wasm {
    use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen(typescript_custom_section)]
    const WASM_API1: &'static str = r#"
export interface DatatypeDefinitionConstructor {
    DatatypeRestriction?: DatatypeRestriction
    DataComplementOf?: DataComplementOf
    DataIntersectionOf?: DataIntersectionOf
    DataUnionOf?: DataUnionOf
    DataOneOf?: DataOneOf
}
"#;

    #[wasm_bindgen(typescript_custom_section)]
    const WASM_API2: &'static str = r#"
/**
 * [DataProperty IRI, DatatypeDefinitionConstructor, annotations]
 */
export type DatatypeDefinition = [IRI, DatatypeDefinitionConstructor, Array<Annotation>];
"#;
}
