use serde_json::Value;

use crate::owl::{Annotation, ClassConstructor, ObjectPropertyConstructor};

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ObjectHasValue(
    pub ObjectPropertyConstructor,
    pub Value,
    pub Vec<Annotation>,
);

impl From<ObjectHasValue> for Box<ClassConstructor> {
    fn from(c: ObjectHasValue) -> Self {
        Box::new(ClassConstructor::ObjectHasValue(c))
    }
}
impl From<ObjectHasValue> for ClassConstructor {
    fn from(c: ObjectHasValue) -> Self {
        ClassConstructor::ObjectHasValue(c)
    }
}

impl ClassConstructor {
    pub fn object_has_value(&self) -> Option<&ObjectHasValue> {
        match self {
            ClassConstructor::ObjectHasValue(d) => Some(d),
            _ => None,
        }
    }
}

#[cfg(feature = "wasm")]
mod wasm {
    use wasm_bindgen::prelude::wasm_bindgen;
    #[wasm_bindgen(typescript_custom_section)]
    const WASM_API: &'static str = r#"
export type ObjectHasValue = [ObjectPropertyConstructor, unknown, Array<Annotation>];
"#;
}
