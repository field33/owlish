use crate::owl::{ClassIRI, ObjectPropertyIRI};

#[derive(Debug)]
pub struct ObjectPropertyRange(pub ObjectPropertyIRI, pub ClassIRI);
