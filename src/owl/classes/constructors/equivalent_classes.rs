use crate::owl::{Annotation, ClassConstructor, ClassIRI};

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct EquivalentClasses {
    #[serde(rename = "classIRI")]
    pub class_iri: ClassIRI,
    pub cls: Box<ClassConstructor>,
    pub annotations: Vec<Annotation>,
}

impl EquivalentClasses {
    pub fn new(
        class_iri: ClassIRI,
        cls: Box<ClassConstructor>,
        annotations: Vec<Annotation>,
    ) -> Self {
        Self {
            class_iri,
            cls,
            annotations,
        }
    }
}

impl From<EquivalentClasses> for Box<ClassConstructor> {
    fn from(c: EquivalentClasses) -> Self {
        Box::new(ClassConstructor::EquivalentClasses(c))
    }
}
impl From<EquivalentClasses> for ClassConstructor {
    fn from(c: EquivalentClasses) -> Self {
        ClassConstructor::EquivalentClasses(c)
    }
}

impl ClassConstructor {
    pub fn equivalent_classes(&self) -> Option<&EquivalentClasses> {
        match self {
            ClassConstructor::EquivalentClasses(d) => Some(d),
            _ => None,
        }
    }
}

#[cfg(feature = "wasm")]
mod wasm {
    use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen(typescript_custom_section)]
    const WASM_API: &'static str = r#"
export type EquivalentClasses = {
    classIRI: IRI,
    cls: ClassConstructor, 
    annotations: Array<Annotation>,
};
"#;
}
