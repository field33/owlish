use crate::owl::{Annotation, Axiom, ClassConstructor, EquivalentClasses, IndividualIRI};

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct SameIndividual {
    #[serde(rename = "individualIRI1")]
    pub individual1: IndividualIRI,
    #[serde(rename = "individualIRI2")]
    pub individual2: IndividualIRI,
    #[serde(rename = "annotations")]
    pub annotations: Vec<Annotation>,
}

impl SameIndividual {
    pub fn new(
        individual1: IndividualIRI,
        individual2: IndividualIRI,
        annotations: Vec<Annotation>,
    ) -> Self {
        Self {
            individual1,
            individual2,
            annotations,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DifferentIndividuals {
    #[serde(rename = "individualIRI1")]
    pub individual1: IndividualIRI,
    #[serde(rename = "individualIRI2")]
    pub individual2: IndividualIRI,
    #[serde(rename = "annotations")]
    pub annotations: Vec<Annotation>,
}

impl DifferentIndividuals {
    pub fn new(
        individual1: IndividualIRI,
        individual2: IndividualIRI,
        annotations: Vec<Annotation>,
    ) -> Self {
        Self {
            individual1,
            individual2,
            annotations,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ClassAssertion {
    #[serde(rename = "cls")]
    pub cls: ClassConstructor,
    #[serde(rename = "individualIRI")]
    pub individual: IndividualIRI,
    #[serde(rename = "annotations")]
    pub annotations: Vec<Annotation>,
}

impl ClassAssertion {
    pub fn new(
        cls: ClassConstructor,
        individual: IndividualIRI,
        annotations: Vec<Annotation>,
    ) -> Self {
        Self {
            cls,
            individual,
            annotations,
        }
    }
}

impl From<ClassAssertion> for Axiom {
    fn from(ca: ClassAssertion) -> Self {
        Axiom::ClassAssertion(ca)
    }
}

impl From<EquivalentClasses> for Axiom {
    fn from(ec: EquivalentClasses) -> Self {
        Axiom::EquivalentClasses(ec)
    }
}

#[cfg(feature = "wasm")]
mod wasm {
    use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen(typescript_custom_section)]
    const WASM_API1: &'static str = r#"
export type SameIndividual = {
    individualIRI1: IRI,
    individualIRI2: IRI,
    annotations: Array<Annotation>,
};
"#;

    #[wasm_bindgen(typescript_custom_section)]
    const WASM_API2: &'static str = r#"
export type DifferentIndividuals = {
    individualIRI1: IRI,
    individualIRI2: IRI,
    annotations: Array<Annotation>,
};
"#;

    #[wasm_bindgen(typescript_custom_section)]
    const WASM_API3: &'static str = r#"
export type ClassAssertion = {
    cls: ClassConstructor,
    individualIRI: IRI,
    annotations: Array<Annotation>,
};
"#;
}
