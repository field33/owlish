use crate::owl::{Annotation, ClassConstructor, ClassIRI};

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct EquivalentClasses(
    pub ClassIRI,
    pub Box<ClassConstructor>,
    pub Vec<Annotation>,
);

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
/**
 * [Class IRI, ClassConstructor, annotations]
 */
export type EquivalentClasses = [IRI, ClassConstructor, Array<Annotation>];
"#;
}
