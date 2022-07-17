use wasm_bindgen::prelude::wasm_bindgen;

use crate::owl::{Axiom, ClassConstructor, IndividualIRI, Regards};

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct SameIndividual(pub IndividualIRI, pub IndividualIRI);

#[wasm_bindgen(typescript_custom_section)]
const WASM_API: &'static str = r#"
/**
 * [Individual IRI, Individual IRI]
 */
export type SameIndividual = [IRI, IRI]
"#;

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DifferentIndividuals(pub IndividualIRI, pub IndividualIRI);

#[wasm_bindgen(typescript_custom_section)]
const WASM_API: &'static str = r#"
/**
 * [Individual IRI, Individual IRI]
 */
export type DifferentIndividuals = [IRI, IRI]
"#;

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ClassAssertion(pub ClassConstructor, pub IndividualIRI);

#[wasm_bindgen(typescript_custom_section)]
const WASM_API: &'static str = r#"
export type ClassAssertion = [ClassConstructor, IRI]
"#;

impl ClassAssertion {
    pub fn class_constructor(&self) -> &ClassConstructor {
        &self.0
    }
    pub fn individual_iri(&self) -> &IndividualIRI {
        &self.1
    }
}

impl Regards for ClassAssertion {
    fn regards(&self, iri: &crate::owl::IRI) -> bool {
        self.individual_iri().as_iri() == iri || self.class_constructor().regards(iri)
    }
}

impl From<ClassAssertion> for Axiom {
    fn from(ca: ClassAssertion) -> Self {
        Axiom::ClassAssertion(ca)
    }
}
