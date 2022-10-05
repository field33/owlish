use crate::owl::{Annotation, Axiom, ClassConstructor};

/// Defines that the subject is a sub class of the object.
///
/// Structure `(subject, object, annotations)`.
#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct SubClassOf {
    #[serde(rename = "cls")]
    pub cls: Box<ClassConstructor>,
    #[serde(rename = "parentClass")]
    pub parent_class: Box<ClassConstructor>,
    #[serde(rename = "annotations")]
    pub annotations: Vec<Annotation>,
}

impl SubClassOf {
    pub fn new(
        cls: Box<ClassConstructor>,
        parent_class: Box<ClassConstructor>,
        annotations: Vec<Annotation>,
    ) -> Self {
        Self {
            cls,
            parent_class,
            annotations,
        }
    }
}

impl From<SubClassOf> for Box<ClassConstructor> {
    fn from(sco: SubClassOf) -> Self {
        Box::new(ClassConstructor::SubClassOf(sco))
    }
}
impl From<SubClassOf> for ClassConstructor {
    fn from(sco: SubClassOf) -> Self {
        ClassConstructor::SubClassOf(sco)
    }
}

impl ClassConstructor {
    pub fn sub_class_of(&self) -> Option<&SubClassOf> {
        match self {
            ClassConstructor::SubClassOf(s) => Some(s),
            _ => None,
        }
    }
}

impl From<SubClassOf> for Axiom {
    fn from(sco: SubClassOf) -> Self {
        Axiom::SubClassOf(sco)
    }
}

#[cfg(feature = "wasm")]
mod wasm {
    use wasm_bindgen::prelude::wasm_bindgen;
    #[wasm_bindgen(typescript_custom_section)]
    const WASM_API: &'static str = r#"
export type SubClassOf = {
    cls: ClassConstructor,
    parentClass: ClassConstructor,
    annotations: Array<Annotation>,
};
"#;
}
