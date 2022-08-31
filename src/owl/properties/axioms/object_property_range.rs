use crate::owl::{Annotation, Axiom, ClassIRI, ObjectPropertyIRI};

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ObjectPropertyRange(
    #[serde(rename = "objectPropertyIRI")] pub ObjectPropertyIRI,
    #[serde(rename = "classIRI")] pub ClassIRI,
    #[serde(rename = "annotations")] pub Vec<Annotation>,
);

impl ObjectPropertyRange {
    pub fn iri(&self) -> &ObjectPropertyIRI {
        &self.0
    }
    pub fn class_iri(&self) -> &ClassIRI {
        &self.1
    }
    pub fn annotations(&self) -> &Vec<Annotation> {
        &self.2
    }
}

impl From<ObjectPropertyRange> for Axiom {
    fn from(opr: ObjectPropertyRange) -> Self {
        Axiom::ObjectPropertyRange(opr)
    }
}

#[cfg(feature = "wasm")]
mod wasm {
    use wasm_bindgen::prelude::wasm_bindgen;
    #[wasm_bindgen(typescript_custom_section)]
    const WASM_API: &'static str = r#"
export type ObjectPropertyRange = {
    objectPropertyIRI: IRI,
    classIRI: IRI,
    annotations: Array<Annotation>,
};
"#;
}
