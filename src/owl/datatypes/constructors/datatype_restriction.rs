use super::DatatypeDefinitionConstructor;
use crate::owl::{Annotation, DatatypeIRI, Literal, Regards};

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Restriction {
    Numeric(
        #[serde(rename = "iri")] DatatypeIRI,
        #[serde(rename = "value")] Literal,
    ),
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DatatypeRestriction(
    #[serde(rename = "datatypeIRI")] pub DatatypeIRI,
    #[serde(rename = "restrictions")] pub Vec<Restriction>,
    #[serde(rename = "annotations")] pub Vec<Annotation>,
);

impl DatatypeRestriction {
    pub fn datatype_iri(&self) -> &DatatypeIRI {
        &self.0
    }
    pub fn restrictions(&self) -> &Vec<Restriction> {
        &self.1
    }
    pub fn annotations(&self) -> &Vec<Annotation> {
        &self.2
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

impl Regards for DatatypeRestriction {
    fn regards(&self, iri: &crate::owl::IRI) -> bool {
        self.datatype_iri().as_iri() == iri
            || self.restrictions().iter().any(|r| match r {
                Restriction::Numeric(d, _) => d.as_iri() == iri,
            })
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
