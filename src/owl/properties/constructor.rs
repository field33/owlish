use crate::owl::{ObjectInverseOf, ObjectPropertyChain, ObjectPropertyIRI};

#[derive(Debug)]
pub enum ObjectPropertyConstructor {
    IRI(ObjectPropertyIRI),
    ObjectInverseOf(ObjectInverseOf),
    ObjectPropertyChain(ObjectPropertyChain),
}
