use serde_json::Value;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::owl::{DatatypeIRI, Regards};

use super::DatatypeDefinitionConstructor;

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Restriction {
    Numeric(DatatypeIRI, Value),
}

#[wasm_bindgen(typescript_custom_section)]
const WASM_API: &'static str = r#"
export type Restriction = { Numeric: [IRI, number] };
"#;

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DatatypeRestriction(pub(crate) DatatypeIRI, pub(crate) Vec<Restriction>);

#[wasm_bindgen(typescript_custom_section)]
const WASM_API: &'static str = r#"
export type DatatypeRestriction = [IRI, Array<Restriction>];
"#;

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

impl DatatypeRestriction {
    pub fn datatype_iri(&self) -> &DatatypeIRI {
        &self.0
    }
    pub fn restrictions(&self) -> &Vec<Restriction> {
        &self.1
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
