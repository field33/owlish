use crate::owl::{DataPropertyIRI, DatatypeIRI};

#[derive(Debug)]
pub struct DataPropertyRange(pub(crate) DataPropertyIRI, pub(crate) DatatypeIRI);
