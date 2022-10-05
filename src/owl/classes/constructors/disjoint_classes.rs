use crate::owl::{Annotation, ClassConstructor};

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DisjointClasses {
    pub classes: Vec<ClassConstructor>,
    pub annotations: Vec<Annotation>,
}

impl DisjointClasses {
    pub fn new(classes: Vec<ClassConstructor>, annotations: Vec<Annotation>) -> Self {
        Self {
            classes,
            annotations,
        }
    }
}

impl From<DisjointClasses> for ClassConstructor {
    fn from(c: DisjointClasses) -> Self {
        ClassConstructor::DisjointClasses(c)
    }
}
impl From<DisjointClasses> for Box<ClassConstructor> {
    fn from(c: DisjointClasses) -> Self {
        Box::new(ClassConstructor::DisjointClasses(c))
    }
}

impl ClassConstructor {
    pub fn disjoint_classes(&self) -> Option<&DisjointClasses> {
        match self {
            ClassConstructor::DisjointClasses(d) => Some(d),
            _ => None,
        }
    }
}

#[cfg(feature = "wasm")]
mod wasm {
    use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen(typescript_custom_section)]
    const WASM_API: &'static str = r#"
export type DisjointClasses = {
    classes: Array<ClassConstructor>, 
    annotations: Array<Annotation>,
};
    "#;
}
