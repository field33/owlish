use crate::owl::{ClassIRI, ObjectPropertyIRI};

#[derive(Debug)]
pub struct ObjectPropertyRange(pub(crate) ObjectPropertyIRI, pub(crate) ClassIRI);
