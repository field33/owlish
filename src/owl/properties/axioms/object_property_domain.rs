use crate::owl::{ClassIRI, ObjectPropertyIRI};

#[derive(Debug)]
pub struct ObjectPropertyDomain(pub(crate) ObjectPropertyIRI, pub(crate) ClassIRI);
