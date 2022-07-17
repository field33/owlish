use wasm_bindgen::prelude::wasm_bindgen;

use crate::owl::{Annotation, DataPropertyIRI};

use super::{DataComplementOf, DataIntersectionOf, DataOneOf, DataUnionOf, DatatypeRestriction};

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum DatatypeDefinitionConstructor {
    DatatypeRestriction(DatatypeRestriction),
    DataComplementOf(DataComplementOf),
    DataIntersectionOf(DataIntersectionOf),
    DataUnionOf(DataUnionOf),
    DataOneOf(DataOneOf),
}

#[wasm_bindgen(typescript_custom_section)]
const WASM_API: &'static str = r#"
export interface DatatypeDefinitionConstructor {
    DatatypeRestriction?: DatatypeRestriction
    DataComplementOf?: DataComplementOf
    DataIntersectionOf?: DataIntersectionOf
    DataUnionOf?: DataUnionOf
    DataOneOf?: DataOneOf
}
"#;

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DatatypeDefinition(
    pub DataPropertyIRI,
    pub DatatypeDefinitionConstructor,
    pub Vec<Annotation>,
);

#[wasm_bindgen(typescript_custom_section)]
const WASM_API: &'static str = r#"
/**
 * [DataProperty IRI, DatatypeDefinitionConstructor, annotations]
 */
export type DatatypeDefinition = [IRI, DatatypeDefinitionConstructor, Array<Annotation>];
"#;
