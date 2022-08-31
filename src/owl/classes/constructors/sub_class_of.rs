use crate::owl::{Annotation, Axiom, ClassConstructor, Regards, IRI};

/// Defines that the subject is a sub class of the object.
///
/// Structure `(subject, object, annotations)`.
#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct SubClassOf(
    #[serde(rename = "cls")] pub Box<ClassConstructor>,
    #[serde(rename = "parentClass")] pub Box<ClassConstructor>,
    #[serde(rename = "annotations")] pub Vec<Annotation>,
);

impl SubClassOf {
    pub fn subject(&self) -> &ClassConstructor {
        &self.0
    }
    pub fn parent(&self) -> &ClassConstructor {
        &self.1
    }
    pub fn annotations(&self) -> &Vec<Annotation> {
        &self.2
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

impl Regards for SubClassOf {
    fn regards(&self, iri: &IRI) -> bool {
        self.subject().regards(iri) || self.parent().regards(iri)
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
