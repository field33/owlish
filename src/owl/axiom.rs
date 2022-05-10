use crate::owl::*;

#[derive(Debug)]
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
            Axiom::EquivalentClasses(_) => false,       // TODO
            Axiom::DisjointClasses(_) => false,         // TODO
            Axiom::DatatypeDefinition(_) => false,      // TODO
            Axiom::ClassAssertion(_) => false,          // TODO
            Axiom::SameIndividual(_) => false,          // TODO
            Axiom::DifferentIndividuals(_) => false,    // TODO
            Axiom::ObjectPropertyAssertion(_) => false, // TODO
            Axiom::NegativeObjectPropertyAssertion(_) => false, // TODO
            Axiom::DataPropertyAssertion(_) => false,   // TODO
            Axiom::NegativeDataPropertyAssertion(_) => false, // TODO
            Axiom::HasKey(_) => false,                  // TODO
        }
    }
}
