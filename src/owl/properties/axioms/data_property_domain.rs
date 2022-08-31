use crate::owl::{Annotation, ClassIRI, DataPropertyIRI, Regards, IRI};

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DataPropertyDomain(
    #[serde(rename = "dataPropertyIRI")] pub DataPropertyIRI,
    #[serde(rename = "classIRI")] pub ClassIRI,
    #[serde(rename = "annotations")] pub Vec<Annotation>,
);

impl DataPropertyDomain {
    pub fn iri(&self) -> &DataPropertyIRI {
        &self.0
    }
    pub fn class_iri(&self) -> &ClassIRI {
        &self.1
    }
    pub fn annotations(&self) -> &Vec<Annotation> {
        &self.2
    }
}

impl Regards for DataPropertyDomain {
    fn regards(&self, iri: &IRI) -> bool {
        self.iri().as_iri() == iri || self.class_iri().as_iri() == iri
    }
}

#[cfg(feature = "wasm")]
mod wasm {
    use wasm_bindgen::prelude::wasm_bindgen;
    #[wasm_bindgen(typescript_custom_section)]
    const WASM_API: &'static str = r#"
export type DataPropertyDomain = {
    dataPropertyIRI: IRI,
    classIRI: IRI,
    annotations: Array<Annotation>,
};
"#;
}
