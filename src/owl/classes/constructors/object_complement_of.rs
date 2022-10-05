use crate::owl::{Annotation, ClassConstructor};

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ObjectComplementOf {
    pub cls: Box<ClassConstructor>,
    pub annotations: Vec<Annotation>,
}

impl ObjectComplementOf {
    pub fn new(cls: Box<ClassConstructor>, annotations: Vec<Annotation>) -> Self {
        Self { cls, annotations }
    }
}

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

#[cfg(feature = "wasm")]
mod wasm {
    use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen(typescript_custom_section)]
    const WASM_API: &'static str = r#"
export type ObjectComplementOf = {
    cls: ClassConstructor, 
    annotations: Array<Annotation>,
};
"#;
}
