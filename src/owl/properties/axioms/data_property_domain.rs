use wasm_bindgen::prelude::wasm_bindgen;

use crate::owl::{ClassIRI, DataPropertyIRI, Regards, IRI};

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DataPropertyDomain(pub DataPropertyIRI, pub ClassIRI);

impl DataPropertyDomain {
    pub fn iri(&self) -> &DataPropertyIRI {
        &self.0
    }
    pub fn class(&self) -> &ClassIRI {
        &self.1
    }
}

impl Regards for DataPropertyDomain {
    fn regards(&self, iri: &IRI) -> bool {
        self.iri().as_iri() == iri || self.class().as_iri() == iri
    }
}

#[wasm_bindgen(typescript_custom_section)]
const WASM_API: &'static str = r#"
/**
 * [DataProperty IRI, Class IRI]
 */
export type DataPropertyDomain = [IRI, IRI];
"#;
