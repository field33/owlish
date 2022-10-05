use serde::{Deserialize, Serialize};

use crate::owl::{Axiom, IndividualIRI, ObjectPropertyConstructor, IRI};

use super::Annotation;

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct ObjectPropertyIRI(IRI);

impl From<IRI> for ObjectPropertyIRI {
    fn from(iri: IRI) -> Self {
        Self(iri)
    }
}
impl From<ObjectPropertyIRI> for ObjectPropertyConstructor {
    fn from(iri: ObjectPropertyIRI) -> Self {
        Self::IRI(iri)
    }
}
impl ObjectPropertyIRI {
    pub fn as_iri(&self) -> &IRI {
        &self.0
    }
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ObjectPropertyAssertion {
    pub iri: ObjectPropertyIRI,
    pub subject: IndividualIRI,
    pub object: IndividualIRI,
    pub annotations: Vec<Annotation>,
}

impl ObjectPropertyAssertion {
    pub fn new(
        iri: ObjectPropertyIRI,
        subject: IndividualIRI,
        object: IndividualIRI,
        annotations: Vec<Annotation>,
    ) -> Self {
        Self {
            iri,
            subject,
            object,
            annotations,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct NegativeObjectPropertyAssertion {
    pub iri: ObjectPropertyIRI,
    pub subject: IndividualIRI,
    pub object: IndividualIRI,
    pub annotations: Vec<Annotation>,
}

impl NegativeObjectPropertyAssertion {
    pub fn new(
        iri: ObjectPropertyIRI,
        subject: IndividualIRI,
        object: IndividualIRI,
        annotations: Vec<Annotation>,
    ) -> Self {
        Self {
            iri,
            subject,
            object,
            annotations,
        }
    }
}
impl From<ObjectPropertyAssertion> for Axiom {
    fn from(opa: ObjectPropertyAssertion) -> Self {
        Self::ObjectPropertyAssertion(opa)
    }
}

#[cfg(feature = "wasm")]
mod wasm {
    use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen(typescript_custom_section)]
    const WASM_API1: &'static str = r#"
/**
 * Assigngs an ObjectProperty to two Individuals.
 */
export type ObjectPropertyAssertion = {
    /**
     * The IRI of the property.
     */
    iri: IRI,
    /**
     * The subject Individual.
     */
    subject: IRI,
    /**
     * The object Individual
     */
    object: IRI,
    annotations: Array<Annotation>,
};
"#;

    #[wasm_bindgen(typescript_custom_section)]
    const WASM_API2: &'static str = r#"
/**
 * Opposite of ObjectPropertyAssertion.
 */
export type NegativeObjectPropertyAssertion = {
    /**
     * The IRI of the property.
     */
    iri: IRI,
    /**
     * The subject Individual.
     */
    subject: IRI,
    /**
     * The object Individual
     */
    object: IRI,
    annotations: Array<Annotation>,
};
"#;
}
