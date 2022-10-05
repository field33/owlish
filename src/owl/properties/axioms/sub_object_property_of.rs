use crate::owl::{Annotation, ObjectPropertyConstructor, ObjectPropertyIRI};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct SubObjectPropertyOf {
    #[serde(rename = "objectProperty")]
    pub object_property: ObjectPropertyConstructor,
    #[serde(rename = "parentObjectPropertyIRI")]
    pub parent_object_property_iri: ObjectPropertyIRI,
    #[serde(rename = "annotations")]
    pub annotations: Vec<Annotation>,
}

impl SubObjectPropertyOf {
    pub fn new(
        object_property: ObjectPropertyConstructor,
        parent_object_property_iri: ObjectPropertyIRI,
        annotations: Vec<Annotation>,
    ) -> Self {
        Self {
            object_property,
            parent_object_property_iri,
            annotations,
        }
    }
}

#[cfg(feature = "wasm")]
mod wasm {
    use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen(typescript_custom_section)]
    const WASM_API: &'static str = r#"
export type SubObjectPropertyOf = {
    objectProperty: ObjectPropertyConstructor,
    parentObjectPropertyIRI: IRI,
    annotations: Array<Annotation>,
};
"#;
}
