use crate::owl::{Annotation, ClassConstructor};

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ObjectUnionOf(pub Vec<ClassConstructor>, pub Vec<Annotation>);

impl From<ObjectUnionOf> for ClassConstructor {
    fn from(c: ObjectUnionOf) -> Self {
        ClassConstructor::ObjectUnionOf(c)
    }
}
impl From<ObjectUnionOf> for Box<ClassConstructor> {
    fn from(c: ObjectUnionOf) -> Self {
        Box::new(ClassConstructor::ObjectUnionOf(c))
    }
}

impl ClassConstructor {
    pub fn object_union_of(&self) -> Option<&ObjectUnionOf> {
        match self {
            ClassConstructor::ObjectUnionOf(d) => Some(d),
            _ => None,
        }
    }
}

#[cfg(feature = "wasm")]
mod wasm {
    use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen(typescript_custom_section)]
    const WASM_API: &'static str = r#"
export type ObjectUnionOf = [Array<ClassConstructor>, Array<Annotation>];
"#;
}
