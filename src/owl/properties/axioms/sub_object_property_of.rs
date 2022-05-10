use crate::owl::{ObjectPropertyConstructor, ObjectPropertyIRI};

#[derive(Debug)]
pub struct SubObjectPropertyOf(
    pub(crate) ObjectPropertyConstructor,
    pub(crate) ObjectPropertyIRI,
);
