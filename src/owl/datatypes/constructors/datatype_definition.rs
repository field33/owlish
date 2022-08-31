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
    #[serde(rename = "dataPropertyIRI")] pub DataPropertyIRI,
    #[serde(rename = "datatypeDefinition")] pub DatatypeDefinitionConstructor,
    #[serde(rename = "annotations")] pub Vec<Annotation>,
);

impl DatatypeDefinition {
    pub fn data_property_iri(&self) -> &DataPropertyIRI {
        &self.0
    }
    pub fn datatype_definition(&self) -> &DatatypeDefinitionConstructor {
        &self.1
    }
    pub fn annotations(&self) -> &Vec<Annotation> {
        &self.2
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
