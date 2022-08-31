use crate::owl::{Annotation, DataPropertyIRI, DatatypeRestriction, Regards};

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DataSomeValuesFrom(
    #[serde(rename = "iri")] pub DataPropertyIRI,
    #[serde(rename = "restriction")] pub DatatypeRestriction,
    #[serde(rename = "annotations")] pub Vec<Annotation>,
);

impl DataSomeValuesFrom {
    pub fn iri(&self) -> &DataPropertyIRI {
        &self.0
    }
    pub fn restriction(&self) -> &DatatypeRestriction {
        &self.1
    }
    pub fn annotations(&self) -> &Vec<Annotation> {
        &self.2
    }
}

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
