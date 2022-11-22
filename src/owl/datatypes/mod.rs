use crate::owl::{IndividualIRI, IRI};

mod constructors;
pub use constructors::*;
use serde::{Deserialize, Serialize};

use super::{Annotation, Axiom, Literal};

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct DatatypeIRI(pub IRI);

impl From<IRI> for DatatypeIRI {
    fn from(iri: IRI) -> Self {
        Self(iri)
    }
}
impl DatatypeIRI {
    pub fn as_iri(&self) -> &IRI {
        &self.0
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct DataPropertyIRI(IRI);

impl From<IRI> for DataPropertyIRI {
    fn from(iri: IRI) -> Self {
        Self(iri)
    }
}
impl DataPropertyIRI {
    pub fn as_iri(&self) -> &IRI {
        &self.0
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct DataPropertyAssertion {
    #[serde(rename = "dataPropertyIRI")]
    pub iri: DataPropertyIRI,
    #[serde(rename = "subjectIRI")]
    pub subject: IndividualIRI,
    #[serde(rename = "value")]
    pub value: Literal,
    #[serde(rename = "annotations")]
    pub annotations: Vec<Annotation>,
}

impl From<DataPropertyAssertion> for Axiom {
    fn from(dpa: DataPropertyAssertion) -> Self {
        Axiom::DataPropertyAssertion(dpa)
    }
}

impl DataPropertyAssertion {
    pub fn new(
        iri: DataPropertyIRI,
        subject: IndividualIRI,
        value: Literal,
        annotations: Vec<Annotation>,
    ) -> Self {
        Self {
            iri,
            subject,
            value,
            annotations,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct NegativeDataPropertyAssertion {
    #[serde(rename = "dataPropertyIRI")]
    pub data_property_iri: DataPropertyIRI,
    #[serde(rename = "subjectIRI")]
    pub subject: IndividualIRI,
    #[serde(rename = "value")]
    pub value: Literal,
    #[serde(rename = "annotations")]
    pub annotations: Vec<Annotation>,
}

impl NegativeDataPropertyAssertion {
    pub fn new(
        iri: DataPropertyIRI,
        subject: IndividualIRI,
        value: Literal,
        annotations: Vec<Annotation>,
    ) -> Self {
        Self {
            data_property_iri: iri,
            subject,
            value,
            annotations,
        }
    }
}

#[cfg(feature = "wasm")]
mod wasm {
    use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen(typescript_custom_section)]
    const WASM_API_1: &'static str = r#"
/**
 * Assigns a value (of the property with iri) to a subject Individual.
 */
export type DataPropertyAssertion = {
    /**
     * IRI of the property.
     */
    dataPropertyIRI: IRI,
    /**
     * IRI of the subject Individual.
     */
    subjectIRI: IRI,
    value: Value,
    annotations: Array<Annotation>,
};
"#;

    #[wasm_bindgen(typescript_custom_section)]
    const WASM_API_2: &'static str = r#"
/**
 * Opposite of DataPropertyAssertion.
 */
export type NegativeDataPropertyAssertion = {
    /**
     * IRI of the property.
     */
    dataPropertyIRI: IRI,
    /**
     * IRI of the subject Individual.
     */
    subjectIRI: IRI,
    value: Value,
    annotations: Array<Annotation>,
}
"#;
}
