use crate::owl::{Annotation, ClassConstructor, ObjectPropertyConstructor};

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ObjectHasSelf(pub ObjectPropertyConstructor, pub Vec<Annotation>);

impl From<ObjectHasSelf> for Box<ClassConstructor> {
    fn from(c: ObjectHasSelf) -> Self {
        Box::new(ClassConstructor::ObjectHasSelf(c))
    }
}
impl From<ObjectHasSelf> for ClassConstructor {
    fn from(c: ObjectHasSelf) -> Self {
        ClassConstructor::ObjectHasSelf(c)
    }
}

impl ClassConstructor {
    pub fn object_has_self(&self) -> Option<&ObjectHasSelf> {
        match self {
            ClassConstructor::ObjectHasSelf(d) => Some(d),
            _ => None,
        }
    }
}

#[cfg(feature = "wasm")]
mod wasm {
    use wasm_bindgen::prelude::wasm_bindgen;
    #[wasm_bindgen(typescript_custom_section)]
    const WASM_API: &'static str = r#"
export type ObjectHasSelf = [ObjectPropertyConstructor, Array<Annotation>];
"#;
}
