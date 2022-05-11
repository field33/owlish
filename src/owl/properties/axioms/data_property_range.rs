use crate::owl::{DataPropertyIRI, DatatypeIRI};

#[derive(Debug)]
pub struct DataPropertyRange(pub DataPropertyIRI, pub DatatypeIRI);
