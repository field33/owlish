use crate::owl::{Annotation, ClassConstructor, ClassIRI};

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct EquivalentClasses(
    #[serde(rename = "classIRI")] pub ClassIRI,
    #[serde(rename = "class")] pub Box<ClassConstructor>,
    #[serde(rename = "annotations")] pub Vec<Annotation>,
);

impl EquivalentClasses {
    pub fn class_iri(&self) -> &ClassIRI {
        &self.0
    }
    pub fn class(&self) -> &ClassConstructor {
        &self.1
    }
    pub fn annotations(&self) -> &Vec<Annotation> {
        &self.2
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
    class: ClassConstructor, 
    annotations: Array<Annotation>,
};
"#;
}
