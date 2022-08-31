use crate::owl::{Annotation, Axiom, ClassConstructor, IndividualIRI, Regards};

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct SameIndividual(
    #[serde(rename = "individual1")] pub IndividualIRI,
    #[serde(rename = "individual2")] pub IndividualIRI,
    #[serde(rename = "annotations")] pub Vec<Annotation>,
);

impl SameIndividual {
    pub fn individual1(&self) -> &IndividualIRI {
        &self.0
    }
    pub fn individual2(&self) -> &IndividualIRI {
        &self.1
    }
    pub fn annotations(&self) -> &Vec<Annotation> {
        &self.2
    }
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DifferentIndividuals(
    #[serde(rename = "individual1")] pub IndividualIRI,
    #[serde(rename = "individual2")] pub IndividualIRI,
    #[serde(rename = "annotations")] pub Vec<Annotation>,
);

impl DifferentIndividuals {
    pub fn individual1(&self) -> &IndividualIRI {
        &self.0
    }
    pub fn individual2(&self) -> &IndividualIRI {
        &self.1
    }
    pub fn annotations(&self) -> &Vec<Annotation> {
        &self.2
    }
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ClassAssertion(
    #[serde(rename = "cls")] pub ClassConstructor,
    #[serde(rename = "individual")] pub IndividualIRI,
    #[serde(rename = "annotations")] pub Vec<Annotation>,
);

impl ClassAssertion {
    pub fn class_constructor(&self) -> &ClassConstructor {
        &self.0
    }
    pub fn individual_iri(&self) -> &IndividualIRI {
        &self.1
    }
    pub fn annotations(&self) -> &Vec<Annotation> {
        &self.2
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
