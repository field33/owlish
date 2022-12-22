use crate::owl::{ClassConstructor, ClassIRI, ObjectPropertyIRI};

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ObjectExactCardinality {
    pub value: u64,
    #[serde(rename = "objectPropertyIRI")]
    pub object_property_iri: ObjectPropertyIRI,
    pub class_iri: Option<ClassIRI>,
}

impl ObjectExactCardinality {
    pub fn new(value: u64, object_property_iri: ObjectPropertyIRI, cls: Option<ClassIRI>) -> Self {
        Self {
            value,
            object_property_iri,
            class_iri: cls,
        }
    }
}

impl From<ObjectExactCardinality> for Box<ClassConstructor> {
    fn from(c: ObjectExactCardinality) -> Self {
        Box::new(ClassConstructor::ObjectExactCardinality(c))
    }
}
impl From<ObjectExactCardinality> for ClassConstructor {
    fn from(c: ObjectExactCardinality) -> Self {
        ClassConstructor::ObjectExactCardinality(c)
    }
}

impl ClassConstructor {
    pub fn object_exact_cardinality(&self) -> Option<&ObjectExactCardinality> {
        match self {
            ClassConstructor::ObjectExactCardinality(d) => Some(d),
            _ => None,
        }
    }
}

#[cfg(feature = "wasm")]
mod wasm {
    use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen(typescript_custom_section)]
    const WASM_API: &'static str = r#"
/**
 * 
 */
export type ObjectExactCardinality = {
    value: number, 
    objectPropertyIRI: IRI, 
    cls: IRI | undefined,
}
"#;
}
