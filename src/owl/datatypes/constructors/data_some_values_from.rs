use wasm_bindgen::prelude::wasm_bindgen;

use crate::owl::{DataPropertyIRI, DatatypeRestriction, Regards};

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DataSomeValuesFrom(pub DataPropertyIRI, pub DatatypeRestriction);

#[wasm_bindgen(typescript_custom_section)]
const WASM_API: &'static str = r#"
export type DataSomeValuesFrom = [IRI, DatatypeRestriction];
"#;

impl DataSomeValuesFrom {
    pub fn data_property_iri(&self) -> &DataPropertyIRI {
        &self.0
    }
    pub fn datatype_restriction(&self) -> &DatatypeRestriction {
        &self.1
    }
}

impl Regards for DataSomeValuesFrom {
    fn regards(&self, iri: &crate::owl::IRI) -> bool {
        self.data_property_iri().as_iri() == iri || self.datatype_restriction().regards(iri)
    }
}
