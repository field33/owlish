use crate::owl::{Annotation, ClassConstructor};

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ObjectIntersectionOf {
    pub classes: Vec<ClassConstructor>,
    pub annotations: Vec<Annotation>,
}

impl ObjectIntersectionOf {
    pub fn new(classes: Vec<ClassConstructor>, annotations: Vec<Annotation>) -> Self {
        Self {
            classes,
            annotations,
        }
    }
}

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

#[cfg(feature = "wasm")]
mod wasm {
    use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen(typescript_custom_section)]
    const WASM_API: &'static str = r#"
export type ObjectIntersectionOf = {
    classes: Array<ClassConstructor>, 
    annotations: Array<Annotation>
};
"#;
}
