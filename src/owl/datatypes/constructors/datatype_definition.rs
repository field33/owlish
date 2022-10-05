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
pub struct DatatypeDefinition {
    #[serde(rename = "dataPropertyIRI")]
    pub data_property_iri: DataPropertyIRI,
    #[serde(rename = "datatypeDefinition")]
    pub datatype_definition: DatatypeDefinitionConstructor,
    #[serde(rename = "annotations")]
    pub annotations: Vec<Annotation>,
}

impl DatatypeDefinition {
    pub fn new(
        data_property_iri: DataPropertyIRI,
        datatype_definition: DatatypeDefinitionConstructor,
        annotations: Vec<Annotation>,
    ) -> Self {
        Self {
            data_property_iri,
            datatype_definition,
            annotations,
        }
    }
}

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
export type DatatypeDefinition = {
    dataPropertyIRI: IRI, 
    datatypeDefinition: DatatypeDefinitionConstructor, 
    annotations: Array<Annotation>,
};
"#;
}
