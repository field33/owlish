use crate::owl::{Annotation, ClassConstructor, LiteralOrIRI, ObjectPropertyConstructor};

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ObjectHasValue(
    pub ObjectPropertyConstructor,
    pub LiteralOrIRI,
    pub Vec<Annotation>,
);

impl ObjectHasValue {
    pub fn object_property(&self) -> &ObjectPropertyConstructor {
        &self.0
    }
    pub fn literal(&self) -> &LiteralOrIRI {
        &self.1
    }
    pub fn annotations(&self) -> &Vec<Annotation> {
        &self.2
    }
}

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
