use crate::owl::{Annotation, DataPropertyIRI, DatatypeRestriction};

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DataSomeValuesFrom {
    pub iri: DataPropertyIRI,
    pub restriction: DatatypeRestriction,
    pub annotations: Vec<Annotation>,
}

impl DataSomeValuesFrom {
    pub fn new(
        iri: DataPropertyIRI,
        restriction: DatatypeRestriction,
        annotations: Vec<Annotation>,
    ) -> Self {
        Self {
            iri,
            restriction,
            annotations,
        }
    }
}

#[cfg(feature = "wasm")]
mod wasm {
    use wasm_bindgen::prelude::wasm_bindgen;
    #[wasm_bindgen(typescript_custom_section)]
    const WASM_API: &'static str = r#"
export type DataSomeValuesFrom = {
    iri: IRI, 
    restriction: DatatypeRestriction,
    annotations: Array<Annotation>,
};
"#;
}
