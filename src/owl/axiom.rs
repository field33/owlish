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
}

impl From<SubObjectPropertyOf> for Axiom {
    fn from(s: SubObjectPropertyOf) -> Self {
        Self::SubObjectPropertyOf(s)
    }
}
