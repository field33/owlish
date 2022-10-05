use crate::owl::{Annotation, ClassConstructor, ClassIRI, ObjectPropertyConstructor};

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ObjectAllValuesFrom {
    #[serde(rename = "objectProperty")]
    pub object_property: ObjectPropertyConstructor,
    pub cls: ClassIRI,
    pub annotations: Vec<Annotation>,
}

impl ObjectAllValuesFrom {
    pub fn new(
        object_property: ObjectPropertyConstructor,
        cls: ClassIRI,
        annotations: Vec<Annotation>,
    ) -> Self {
        Self {
            object_property,
            cls,
            annotations,
        }
    }
}

impl From<ObjectAllValuesFrom> for Box<ClassConstructor> {
    fn from(c: ObjectAllValuesFrom) -> Self {
        Box::new(ClassConstructor::ObjectAllValuesFrom(c))
    }
}

impl From<ObjectAllValuesFrom> for ClassConstructor {
    fn from(c: ObjectAllValuesFrom) -> Self {
        ClassConstructor::ObjectAllValuesFrom(c)
    }
}

impl ClassConstructor {
    pub fn object_all_values_from(&self) -> Option<&ObjectAllValuesFrom> {
        match self {
            ClassConstructor::ObjectAllValuesFrom(d) => Some(d),
            _ => None,
        }
    }
}

#[cfg(feature = "wasm")]
mod wasm {
    use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen(typescript_custom_section)]
    const WASM_API: &'static str = r#"
export type ObjectAllValuesFrom = {
    objectProperty: ObjectPropertyConstructor, 
    cls: IRI,
    annotations: Array<Annotation>,
};
"#;
}
