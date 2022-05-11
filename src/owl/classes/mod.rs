use std::fmt::Display;

use crate::owl::{DataSomeValuesFrom, Regards, IRI};

mod constructors;
pub use constructors::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassIRI(IRI);

impl Display for ClassIRI {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ClassIRI {
    pub fn as_iri(&self) -> &IRI {
        &self.0
    }
}

impl From<IRI> for ClassIRI {
    fn from(iri: IRI) -> Self {
        Self(iri)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum ClassConstructor {
    IRI(ClassIRI),
    SubClassOf(SubClassOf),
    DataSomeValuesFrom(DataSomeValuesFrom),
    EquivalentClasses(EquivalentClasses),
    DisjointClasses(DisjointClasses),
    ObjectComplementOf(ObjectComplementOf),
    ObjectIntersectionOf(ObjectIntersectionOf),
    ObjectUnionOf(ObjectUnionOf),
    ObjectSomeValuesFrom(ObjectSomeValuesFrom),
    ObjectMaxCardinality(ObjectMaxCardinality),
    ObjectMinCardinality(ObjectMinCardinality),
    ObjectExactCardinality(ObjectExactCardinality),
    ObjectAllValuesFrom(ObjectAllValuesFrom),
    ObjectOneOf(ObjectOneOf),
    ObjectHasValue(ObjectHasValue),
    ObjectHasSelf(ObjectHasSelf),
}

impl From<ClassIRI> for Box<ClassConstructor> {
    fn from(iri: ClassIRI) -> Self {
        Box::new(ClassConstructor::IRI(iri))
    }
}
impl From<ClassIRI> for ClassConstructor {
    fn from(iri: ClassIRI) -> Self {
        ClassConstructor::IRI(iri)
    }
}

impl ClassConstructor {
    pub fn is_iri(&self, iri: &IRI) -> bool {
        match self {
            Self::IRI(i) => i.as_iri() == iri,
            _ => false,
        }
    }
    pub fn iri(&self) -> Option<&ClassIRI> {
        match self {
            Self::IRI(iri) => Some(iri),
            _ => None,
        }
    }
}

impl Regards for ClassConstructor {
    fn regards(&self, iri: &IRI) -> bool {
        match self {
            ClassConstructor::IRI(i) => i.as_iri() == iri,
            ClassConstructor::SubClassOf(c) => c.regards(iri),
            ClassConstructor::DataSomeValuesFrom(c) => c.regards(iri),
            ClassConstructor::EquivalentClasses(_) => false, // TODO
            ClassConstructor::DisjointClasses(_) => false,   // TODO
            ClassConstructor::ObjectComplementOf(_) => false, // TODO
            ClassConstructor::ObjectIntersectionOf(_) => false, // TODO
            ClassConstructor::ObjectUnionOf(_) => false,     // TODO
            ClassConstructor::ObjectSomeValuesFrom(_) => false, // TODO
            ClassConstructor::ObjectMaxCardinality(_) => false, // TODO
            ClassConstructor::ObjectMinCardinality(_) => false, // TODO
            ClassConstructor::ObjectExactCardinality(_) => false, // TODO
            ClassConstructor::ObjectAllValuesFrom(_) => false, // TODO
            ClassConstructor::ObjectOneOf(_) => false,       // TODO
            ClassConstructor::ObjectHasValue(_) => false,    // TODO
            ClassConstructor::ObjectHasSelf(_) => false,     // TODO
        }
    }
}

// from data values

impl From<DataSomeValuesFrom> for ClassConstructor {
    fn from(c: DataSomeValuesFrom) -> Self {
        ClassConstructor::DataSomeValuesFrom(c)
    }
}
impl From<DataSomeValuesFrom> for Box<ClassConstructor> {
    fn from(c: DataSomeValuesFrom) -> Self {
        Box::new(ClassConstructor::DataSomeValuesFrom(c))
    }
}
