use crate::owl::{Annotation, ClassConstructor, ClassIRI, ObjectPropertyConstructor};

/// Class construction based on properties.
#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ObjectSomeValuesFrom {
    #[serde(rename = "objectProperty")]
    pub object_property: ObjectPropertyConstructor,
    pub cls: ClassIRI,
    pub annotations: Vec<Annotation>,
}

impl ObjectSomeValuesFrom {
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

impl From<ObjectSomeValuesFrom> for ClassConstructor {
    fn from(c: ObjectSomeValuesFrom) -> Self {
        ClassConstructor::ObjectSomeValuesFrom(c)
    }
}

impl From<ObjectSomeValuesFrom> for Box<ClassConstructor> {
    fn from(c: ObjectSomeValuesFrom) -> Self {
        Box::new(ClassConstructor::ObjectSomeValuesFrom(c))
    }
}

impl ClassConstructor {
    pub fn object_some_values_from(&self) -> Option<&ObjectSomeValuesFrom> {
        match self {
            ClassConstructor::ObjectSomeValuesFrom(d) => Some(d),
            _ => None,
        }
    }
}

#[cfg(feature = "wasm")]
mod wasm {
    use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen(typescript_custom_section)]
    const WASM_API: &'static str = r#"
    export type ObjectSomeValuesFrom = {
        objectProperty: ObjectPropertyConstructor,
        cls: IRI,
        annotations: Array<Annotation>,
};
    "#;
}
