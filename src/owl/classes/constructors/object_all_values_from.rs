use crate::owl::{Annotation, ClassConstructor, ClassIRI, ObjectPropertyConstructor};

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ObjectAllValuesFrom(
    pub ObjectPropertyConstructor,
    pub ClassIRI,
    pub Vec<Annotation>,
);

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
export type ObjectAllValuesFrom = [ObjectPropertyConstructor, IRI, Array<Annotation>];
"#;
}
