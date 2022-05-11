use crate::owl::{ClassIRI, ObjectPropertyIRI};

#[derive(Debug)]
pub struct ObjectPropertyDomain(pub ObjectPropertyIRI, pub ClassIRI);
