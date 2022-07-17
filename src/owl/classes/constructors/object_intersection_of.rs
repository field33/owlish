use wasm_bindgen::prelude::wasm_bindgen;

use crate::owl::{Annotation, ClassConstructor};

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ObjectIntersectionOf(pub(crate) Vec<ClassConstructor>, pub(crate) Vec<Annotation>);

#[wasm_bindgen(typescript_custom_section)]
const WASM_API: &'static str = r#"
export type ObjectIntersectionOf = [Array<ClassConstructor>, Array<Annotation>];
"#;

impl From<ObjectIntersectionOf> for Box<ClassConstructor> {
    fn from(c: ObjectIntersectionOf) -> Self {
        Box::new(ClassConstructor::ObjectIntersectionOf(c))
    }
}

impl From<ObjectIntersectionOf> for ClassConstructor {
    fn from(c: ObjectIntersectionOf) -> Self {
        ClassConstructor::ObjectIntersectionOf(c)
    }
}

impl ClassConstructor {
    pub fn object_intersection_of(&self) -> Option<&ObjectIntersectionOf> {
        match self {
            ClassConstructor::ObjectIntersectionOf(d) => Some(d),
            _ => None,
        }
    }
}
