use crate::owl::{Annotation, Axiom, ClassConstructor, ObjectPropertyIRI};

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ObjectPropertyRange {
    #[serde(rename = "objectPropertyIRI")]
    pub object_property_iri: ObjectPropertyIRI,
    #[serde(rename = "cls")]
    pub cls: ClassConstructor,
    #[serde(rename = "annotations")]
    pub annotations: Vec<Annotation>,
}

impl ObjectPropertyRange {
    pub fn new(
        object_property_iri: ObjectPropertyIRI,
        cls: ClassConstructor,
        annotations: Vec<Annotation>,
    ) -> Self {
        Self {
            object_property_iri,
            cls,
            annotations,
        }
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
    cls: ClassConstructor,
    annotations: Array<Annotation>,
};
"#;
}
