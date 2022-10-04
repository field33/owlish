use serde::{Deserialize, Serialize};

use crate::owl::*;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum Axiom {
    // Annotations
    AnnotationAssertion(AnnotationAssertion),
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
            Axiom::AnnotationAssertion(a) => &mut a.3,
            Axiom::SubObjectPropertyOf(a) => &mut a.2,
            Axiom::EquivalentObjectProperties(a) => &mut a.2,
            Axiom::EquivalentDataProperties(a) => &mut a.2,
            Axiom::InverseObjectProperties(a) => &mut a.2,
            Axiom::DisjointObjectProperties(a) => &mut a.2,
            Axiom::ObjectPropertyDomain(a) => &mut a.2,
            Axiom::ObjectPropertyRange(a) => &mut a.2,
            Axiom::DataPropertyDomain(a) => &mut a.2,
            Axiom::DataPropertyRange(a) => &mut a.2,
            Axiom::SymmetricObjectProperty(a) => &mut a.1,
            Axiom::AsymmetricObjectProperty(a) => &mut a.1,
            Axiom::ReflexiveObjectProperty(a) => &mut a.1,
            Axiom::IrreflexiveObjectProperty(a) => &mut a.1,
            Axiom::FunctionalObjectProperty(a) => &mut a.1,
            Axiom::InverseFunctionalObjectProperty(a) => &mut a.1,
            Axiom::TransitiveObjectProperty(a) => &mut a.1,
            Axiom::FunctionalDataProperty(a) => &mut a.1,
            Axiom::SubClassOf(a) => &mut a.2,
            Axiom::EquivalentClasses(a) => &mut a.2,
            Axiom::DisjointClasses(a) => &mut a.1,
            Axiom::DatatypeDefinition(a) => &mut a.2,
            Axiom::ClassAssertion(a) => &mut a.2,
            Axiom::SameIndividual(a) => &mut a.2,
            Axiom::DifferentIndividuals(a) => &mut a.2,
            Axiom::ObjectPropertyAssertion(a) => &mut a.3,
            Axiom::NegativeObjectPropertyAssertion(a) => &mut a.3,
            Axiom::DataPropertyAssertion(a) => &mut a.3,
            Axiom::NegativeDataPropertyAssertion(a) => &mut a.3,
            Axiom::HasKey(a) => &mut a.2,
        }
    }
}

impl From<SubObjectPropertyOf> for Axiom {
    fn from(s: SubObjectPropertyOf) -> Self {
        Self::SubObjectPropertyOf(s)
    }
}

impl Regards for Axiom {
    fn regards(&self, iri: &IRI) -> bool {
        match self {
            Axiom::AnnotationAssertion(a) => a.regards(iri),
            Axiom::SubObjectPropertyOf(_) => false, // TODO
            Axiom::EquivalentObjectProperties(_) => false, // TODO
            Axiom::EquivalentDataProperties(_) => false, // TODO
            Axiom::InverseObjectProperties(_) => false, // TODO
            Axiom::DisjointObjectProperties(_) => false, // TODO
            Axiom::ObjectPropertyDomain(_) => false, // TODO
            Axiom::ObjectPropertyRange(_) => false, // TODO
            Axiom::DataPropertyDomain(d) => d.regards(iri),
            Axiom::DataPropertyRange(_) => false,        // TODO
            Axiom::SymmetricObjectProperty(_) => false,  // TODO
            Axiom::AsymmetricObjectProperty(_) => false, // TODO
            Axiom::ReflexiveObjectProperty(_) => false,  // TODO
            Axiom::IrreflexiveObjectProperty(_) => false, // TODO
            Axiom::FunctionalObjectProperty(_) => false, // TODO
            Axiom::InverseFunctionalObjectProperty(_) => false, // TODO
            Axiom::TransitiveObjectProperty(_) => false, // TODO
            Axiom::FunctionalDataProperty(_) => false,   // TODO
            Axiom::SubClassOf(s) => s.regards(iri),
            Axiom::EquivalentClasses(_) => false,  // TODO
            Axiom::DisjointClasses(_) => false,    // TODO
            Axiom::DatatypeDefinition(_) => false, // TODO
            Axiom::ClassAssertion(ca) => ca.regards(iri), // TODO
            Axiom::SameIndividual(_) => false,     // TODO
            Axiom::DifferentIndividuals(_) => false, // TODO
            Axiom::ObjectPropertyAssertion(_) => false, // TODO
            Axiom::NegativeObjectPropertyAssertion(_) => false, // TODO
            Axiom::DataPropertyAssertion(_) => false, // TODO
            Axiom::NegativeDataPropertyAssertion(_) => false, // TODO
            Axiom::HasKey(_) => false,             // TODO
        }
    }
}
