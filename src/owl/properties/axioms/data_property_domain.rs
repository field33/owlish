use crate::owl::{ClassIRI, DataPropertyIRI, Regards, IRI};

#[derive(Debug)]
pub struct DataPropertyDomain(pub(crate) DataPropertyIRI, pub(crate) ClassIRI);

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
