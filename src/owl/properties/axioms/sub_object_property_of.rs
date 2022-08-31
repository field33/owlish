use crate::owl::{Annotation, ObjectPropertyConstructor, ObjectPropertyIRI};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct SubObjectPropertyOf(
    #[serde(rename = "objectProperty")] pub ObjectPropertyConstructor,
    #[serde(rename = "parentObjectPropertyIRI")] pub ObjectPropertyIRI,
    #[serde(rename = "annotations")] pub Vec<Annotation>,
);

impl SubObjectPropertyOf {
    pub fn object_property(&self) -> &ObjectPropertyConstructor {
        &self.0
    }
    pub fn object_property_iri(&self) -> &ObjectPropertyIRI {
        &self.1
    }
    pub fn annotations(&self) -> &Vec<Annotation> {
        &self.2
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
