use crate::owl::{ObjectPropertyConstructor, ObjectPropertyIRI};

#[derive(Debug)]
pub struct SubObjectPropertyOf(
    pub ObjectPropertyConstructor,
    pub ObjectPropertyIRI,
);
