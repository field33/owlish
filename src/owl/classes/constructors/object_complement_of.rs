use wasm_bindgen::prelude::wasm_bindgen;

use crate::owl::{Annotation, ClassConstructor};

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ObjectComplementOf(pub(crate) Box<ClassConstructor>, pub(crate) Vec<Annotation>);

#[wasm_bindgen(typescript_custom_section)]
const WASM_API: &'static str = r#"
export type ObjectComplementOf = [ClassConstructor, Array<Annotation>];
"#;

impl From<ObjectComplementOf> for Box<ClassConstructor> {
    fn from(c: ObjectComplementOf) -> Self {
        Box::new(ClassConstructor::ObjectComplementOf(c))
    }
}
impl From<ObjectComplementOf> for ClassConstructor {
    fn from(c: ObjectComplementOf) -> Self {
        ClassConstructor::ObjectComplementOf(c)
    }
}

impl ClassConstructor {
    pub fn object_complement_of(&self) -> Option<&ObjectComplementOf> {
        match self {
            ClassConstructor::ObjectComplementOf(d) => Some(d),
            _ => None,
        }
    }
}
