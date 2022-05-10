use crate::owl::{ClassConstructor, IndividualIRI};

#[derive(Debug)]
pub struct SameIndividual(pub(crate) IndividualIRI, pub(crate) IndividualIRI);
#[derive(Debug)]
pub struct DifferentIndividuals(pub(crate) IndividualIRI, pub(crate) IndividualIRI);
#[derive(Debug)]
pub struct ClassAssertion(pub(crate) ClassConstructor, pub(crate) IndividualIRI);
