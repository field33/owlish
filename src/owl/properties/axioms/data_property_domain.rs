use crate::owl::{ClassIRI, DataPropertyIRI, Regards, IRI};

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
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
