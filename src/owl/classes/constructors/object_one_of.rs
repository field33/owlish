use crate::owl::{Annotation, ClassConstructor, IndividualIRI};

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ObjectOneOf {
    #[serde(rename = "individualIRIs")]
    pub individuals: Vec<IndividualIRI>,
    pub annotations: Vec<Annotation>,
}

impl ObjectOneOf {
    pub fn new(individuals: Vec<IndividualIRI>, annotations: Vec<Annotation>) -> Self {
        Self {
            individuals,
            annotations,
        }
    }
}

impl From<ObjectOneOf> for Box<ClassConstructor> {
    fn from(c: ObjectOneOf) -> Self {
        Box::new(ClassConstructor::ObjectOneOf(c))
    }
}

impl From<ObjectOneOf> for ClassConstructor {
    fn from(c: ObjectOneOf) -> Self {
        ClassConstructor::ObjectOneOf(c)
    }
}

impl ClassConstructor {
    pub fn object_one_of(&self) -> Option<&ObjectOneOf> {
        match self {
            ClassConstructor::ObjectOneOf(d) => Some(d),
            _ => None,
        }
    }
}

#[cfg(feature = "wasm")]
mod wasm {
    use wasm_bindgen::prelude::wasm_bindgen;
    #[wasm_bindgen(typescript_custom_section)]
    const WASM_API: &'static str = r#"
export type ObjectOneOf = {
    individualIRIs: Array<IRI>, 
    annotations: Array<Annotation>
};
"#;
}
