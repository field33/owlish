use crate::owl::{Annotation, Axiom, ClassConstructor, IndividualIRI};

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct SameIndividual {
    pub individual1: IndividualIRI,
    pub individual2: IndividualIRI,
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
    pub individual1: IndividualIRI,
    pub individual2: IndividualIRI,
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
    pub cls: ClassConstructor,
    pub individual: IndividualIRI,
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

#[cfg(feature = "wasm")]
mod wasm {
    use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen(typescript_custom_section)]
    const WASM_API1: &'static str = r#"
export type SameIndividual = {
    individual1: IRI,
    individual2: IRI,
    annotations: Array<Annotation>,
};
"#;

    #[wasm_bindgen(typescript_custom_section)]
    const WASM_API2: &'static str = r#"
export type DifferentIndividuals = {
    individual1: IRI,
    individual2: IRI,
    annotations: Array<Annotation>,
};
"#;

    #[wasm_bindgen(typescript_custom_section)]
    const WASM_API3: &'static str = r#"
export type ClassAssertion = {
    cls: ClassConstructor,
    individual: IRI,
    annotations: Array<Annotation>,
};
"#;
}
