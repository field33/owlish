use serde::{Deserialize, Serialize};

use crate::owl::*;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum Axiom {
    // Annotations
    AnnotationAssertion(AnnotationAssertion),
    AnnotationPropertyRange(AnnotationPropertyRange),
    AnnotationPropertyDomain(AnnotationPropertyDomain),
    // Properties
    SubObjectPropertyOf(SubObjectPropertyOf),
    SubDataPropertyOf(SubDataPropertyOf),
    SubAnnotationPropertyOf(SubAnnotationPropertyOf),
    EquivalentObjectProperties(EquivalentObjectProperties),
    EquivalentDataProperties(EquivalentDataProperties),
    InverseObjectProperties(InverseObjectProperties),
    DisjointObjectProperties(DisjointObjectProperties),
    ObjectPropertyDomain(ObjectPropertyDomain),
    ObjectPropertyRange(ObjectPropertyRange),
    DataPropertyDomain(DataPropertyDomain),
    DataPropertyRange(DataPropertyRange),
    SymmetricObjectProperty(SymmetricObjectProperty),
    AsymmetricObjectProperty(AsymmetricObjectProperty),
    ReflexiveObjectProperty(ReflexiveObjectProperty),
    IrreflexiveObjectProperty(IrreflexiveObjectProperty),
    FunctionalObjectProperty(FunctionalObjectProperty),
    InverseFunctionalObjectProperty(InverseFunctionalObjectProperty),
    TransitiveObjectProperty(TransitiveObjectProperty),
    FunctionalDataProperty(FunctionalDataProperty),
    // Classes
    SubClassOf(SubClassOf),
    EquivalentClasses(EquivalentClasses),
    DisjointClasses(DisjointClasses),
    // Datatypes
    DatatypeDefinition(DatatypeDefinition),
    // Individuals
    ClassAssertion(ClassAssertion),
    SameIndividual(SameIndividual),
    DifferentIndividuals(DifferentIndividuals),
    // ObjectProperties
    ObjectPropertyAssertion(ObjectPropertyAssertion),
    NegativeObjectPropertyAssertion(NegativeObjectPropertyAssertion),
    // DataProperties
    DataPropertyAssertion(DataPropertyAssertion),
    NegativeDataPropertyAssertion(NegativeDataPropertyAssertion),
    // Other
    HasKey(HasKey),
}

impl Axiom {
    pub fn annotations_mut(&mut self) -> &mut Vec<Annotation> {
        match self {
            Axiom::AnnotationAssertion(a) => &mut a.annotations,
            Axiom::AnnotationPropertyDomain(a) => &mut a.annotations,
            Axiom::AnnotationPropertyRange(a) => &mut a.annotations,
            Axiom::SubObjectPropertyOf(a) => &mut a.annotations,
            Axiom::SubAnnotationPropertyOf(a) => &mut a.annotations,
            Axiom::SubDataPropertyOf(a) => &mut a.annotations,
            Axiom::EquivalentObjectProperties(a) => &mut a.annotations,
            Axiom::EquivalentDataProperties(a) => &mut a.annotations,
            Axiom::InverseObjectProperties(a) => &mut a.annotations,
            Axiom::DisjointObjectProperties(a) => &mut a.annotations,
            Axiom::ObjectPropertyDomain(a) => &mut a.annotations,
            Axiom::ObjectPropertyRange(a) => &mut a.annotations,
            Axiom::DataPropertyDomain(a) => &mut a.annotations,
            Axiom::DataPropertyRange(a) => &mut a.annotations,
            Axiom::SymmetricObjectProperty(a) => &mut a.annotations,
            Axiom::AsymmetricObjectProperty(a) => &mut a.annotations,
            Axiom::ReflexiveObjectProperty(a) => &mut a.annotations,
            Axiom::IrreflexiveObjectProperty(a) => &mut a.annotations,
            Axiom::FunctionalObjectProperty(a) => &mut a.annotations,
            Axiom::InverseFunctionalObjectProperty(a) => &mut a.annotations,
            Axiom::TransitiveObjectProperty(a) => &mut a.annotations,
            Axiom::FunctionalDataProperty(a) => &mut a.annotations,
            Axiom::SubClassOf(a) => &mut a.annotations,
            Axiom::EquivalentClasses(a) => &mut a.annotations,
            Axiom::DisjointClasses(a) => &mut a.annotations,
            Axiom::DatatypeDefinition(a) => &mut a.annotations,
            Axiom::ClassAssertion(a) => &mut a.annotations,
            Axiom::SameIndividual(a) => &mut a.annotations,
            Axiom::DifferentIndividuals(a) => &mut a.annotations,
            Axiom::ObjectPropertyAssertion(a) => &mut a.annotations,
            Axiom::NegativeObjectPropertyAssertion(a) => &mut a.annotations,
            Axiom::DataPropertyAssertion(a) => &mut a.annotations,
            Axiom::NegativeDataPropertyAssertion(a) => &mut a.annotations,
            Axiom::HasKey(a) => &mut a.annotations,
        }
    }

    pub fn subject(&self) -> Option<&IRI> {
        match self {
            Axiom::AnnotationAssertion(a) => Some(&a.subject),
            Axiom::AnnotationPropertyRange(a) => Some(a.iri.as_iri()),
            Axiom::AnnotationPropertyDomain(a) => Some(a.iri.as_iri()),
            Axiom::SubObjectPropertyOf(a) => match &a.object_property {
                ObjectPropertyConstructor::IRI(iri) => Some(iri.as_iri()),
                ObjectPropertyConstructor::ObjectInverseOf(_) => None,
                ObjectPropertyConstructor::ObjectPropertyChain(_) => None,
            },
            Axiom::SubDataPropertyOf(a) => Some(a.subject_iri.as_iri()),
            Axiom::SubAnnotationPropertyOf(a) => Some(a.subject_iri.as_iri()),
            Axiom::EquivalentObjectProperties(a) => Some(a.object_property_iri_1.as_iri()),
            Axiom::EquivalentDataProperties(a) => Some(a.data_property_iri_1.as_iri()),
            Axiom::InverseObjectProperties(a) => Some(a.object_property_iri_1.as_iri()),
            Axiom::DisjointObjectProperties(a) => Some(a.object_property_iri_1.as_iri()),
            Axiom::ObjectPropertyDomain(a) => Some(a.iri.as_iri()),
            Axiom::ObjectPropertyRange(a) => Some(a.iri.as_iri()),
            Axiom::DataPropertyDomain(a) => Some(a.iri.as_iri()),
            Axiom::DataPropertyRange(a) => Some(a.iri.as_iri()),
            Axiom::SymmetricObjectProperty(a) => Some(a.object_property_iri.as_iri()),
            Axiom::AsymmetricObjectProperty(a) => Some(a.object_property_iri.as_iri()),
            Axiom::ReflexiveObjectProperty(a) => Some(a.object_property_iri.as_iri()),
            Axiom::IrreflexiveObjectProperty(a) => Some(a.object_property_iri.as_iri()),
            Axiom::FunctionalObjectProperty(a) => Some(a.object_property_iri.as_iri()),
            Axiom::InverseFunctionalObjectProperty(a) => Some(a.object_property_iri.as_iri()),
            Axiom::TransitiveObjectProperty(a) => Some(a.object_property_iri.as_iri()),
            Axiom::FunctionalDataProperty(a) => Some(a.data_property_iri.as_iri()),
            Axiom::SubClassOf(a) => match a.cls.as_ref() {
                ClassConstructor::IRI(iri) => Some(iri.as_iri()),
                _ => None,
            },
            Axiom::EquivalentClasses(a) => Some(a.class_iri.as_iri()),
            Axiom::DisjointClasses(_) => None,
            Axiom::DatatypeDefinition(a) => Some(a.data_property_iri.as_iri()),
            Axiom::ClassAssertion(a) => Some(a.individual.as_iri()),
            Axiom::SameIndividual(a) => Some(a.individual1.as_iri()),
            Axiom::DifferentIndividuals(a) => Some(a.individual1.as_iri()),
            Axiom::ObjectPropertyAssertion(a) => Some(a.subject.as_iri()),
            Axiom::NegativeObjectPropertyAssertion(a) => Some(a.subject.as_iri()),
            Axiom::DataPropertyAssertion(a) => Some(a.subject.as_iri()),
            Axiom::NegativeDataPropertyAssertion(a) => Some(a.subject.as_iri()),
            Axiom::HasKey(a) => Some(a.iri.as_iri()),
        }
    }
}

impl From<SubObjectPropertyOf> for Axiom {
    fn from(s: SubObjectPropertyOf) -> Self {
        Self::SubObjectPropertyOf(s)
    }
}

impl From<SubAnnotationPropertyOf> for Axiom {
    fn from(s: SubAnnotationPropertyOf) -> Self {
        Self::SubAnnotationPropertyOf(s)
    }
}

impl From<SubDataPropertyOf> for Axiom {
    fn from(s: SubDataPropertyOf) -> Self {
        Self::SubDataPropertyOf(s)
    }
}
