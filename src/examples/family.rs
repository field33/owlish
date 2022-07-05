use serde_json::Value;

use crate::api::Ontology as ApiOntology;
use crate::owl::well_known as wk;
use crate::owl::*;

pub fn family() -> ApiOntology {
    let other_ont = IRI::builder("https://example.com/otherOnt#").unwrap();
    let iri = IRI::builder("https://example.com/family#").unwrap();
    let owl = Ontology::new(
        vec![
            Declaration::NamedIndividual(iri.new("John")),
            Declaration::NamedIndividual(iri.new("Mary")),
            Declaration::NamedIndividual(iri.new("Jim")),
            Declaration::NamedIndividual(iri.new("James")),
            Declaration::NamedIndividual(iri.new("Jack")),
            Declaration::NamedIndividual(iri.new("Bill")),
            Declaration::NamedIndividual(iri.new("Susan")),
            //
            Declaration::Class(iri.new("Person")),
            Declaration::Class(iri.new("Woman")),
            Declaration::Class(iri.new("Parent")),
            Declaration::Class(iri.new("Father")),
            Declaration::Class(iri.new("Mother")),
            Declaration::Class(iri.new("SocialRole")),
            Declaration::Class(iri.new("Man")),
            Declaration::Class(iri.new("Teenager")),
            Declaration::Class(iri.new("ChildlessPerson")),
            Declaration::Class(iri.new("Human")),
            Declaration::Class(iri.new("Female")),
            Declaration::Class(iri.new("HappyPerson")),
            Declaration::Class(iri.new("JohnsChildren")),
            Declaration::Class(iri.new("NarcisticPerson")),
            Declaration::Class(iri.new("Dead")),
            Declaration::Class(iri.new("Orphan")),
            Declaration::Class(iri.new("Adult")),
            Declaration::Class(iri.new("YoungChild")),
            //
            Declaration::ObjectProperty(iri.new("hasWife")),
            Declaration::ObjectProperty(iri.new("hasChild")),
            Declaration::ObjectProperty(iri.new("hasDaughter")),
            Declaration::ObjectProperty(iri.new("loves")),
            Declaration::ObjectProperty(iri.new("hasSpouse")),
            Declaration::ObjectProperty(iri.new("hasGrandparent")),
            Declaration::ObjectProperty(iri.new("hasParent")),
            Declaration::ObjectProperty(iri.new("hasBrother")),
            Declaration::ObjectProperty(iri.new("hasUncle")),
            Declaration::ObjectProperty(iri.new("hasSon")),
            Declaration::ObjectProperty(iri.new("hasAncestor")),
            Declaration::ObjectProperty(iri.new("hasHusband")),
            //
            Declaration::DataProperty(iri.new("hasAge")),
            Declaration::DataProperty(iri.new("hasSSN")),
            //
            Declaration::Datatype(iri.new("personAge")),
            Declaration::Datatype(iri.new("minorAge")),
            Declaration::Datatype(iri.new("majorAge")),
            Declaration::Datatype(iri.new("toddlerAge")),
        ],
        vec![
            Axiom::AnnotationAssertion(AnnotationAssertion(
                iri.new("comment"),
                iri.new("Person"),
                Value::from("Represents the set of all people"),
            )),
            Axiom::SubObjectPropertyOf(SubObjectPropertyOf(
                iri.new::<ObjectPropertyIRI>("hasWife").into(),
                iri.new("hasSpouse"),
            )),
            Axiom::SubObjectPropertyOf(SubObjectPropertyOf(
                ObjectPropertyChain(vec![iri.new("hasParent"), iri.new("hasParent")]).into(),
                iri.new("hasGrandparent"),
            )),
            Axiom::SubObjectPropertyOf(SubObjectPropertyOf(
                ObjectPropertyChain(vec![iri.new("hasFather"), iri.new("hasBrother")]).into(),
                iri.new("hasUncle"),
            )),
            Axiom::SubObjectPropertyOf(SubObjectPropertyOf(
                iri.new::<ObjectPropertyIRI>("hasFather").into(),
                iri.new("hasParent"),
            )),
            //
            Axiom::EquivalentObjectProperties(EquivalentObjectProperties(
                iri.new("hasChild"),
                other_ont.new("child"),
            )),
            Axiom::InverseObjectProperties(InverseObjectProperties(
                iri.new("hasChild"),
                other_ont.new("child"),
            )),
            Axiom::EquivalentDataProperties(EquivalentDataProperties(
                iri.new("hasAge"),
                other_ont.new("age"),
            )),
            Axiom::DisjointObjectProperties(DisjointObjectProperties(
                iri.new("hasSon"),
                iri.new("hasDaughter"),
            )),
            Axiom::ObjectPropertyDomain(ObjectPropertyDomain(iri.new("hasWife"), iri.new("Man"))),
            Axiom::ObjectPropertyRange(ObjectPropertyRange(iri.new("hasWife"), iri.new("Woman"))),
            Axiom::DataPropertyDomain(DataPropertyDomain(iri.new("hasAge"), iri.class("Person"))),
            Axiom::DataPropertyRange(DataPropertyRange(
                iri.new("hasAge"),
                well_known::xsd_nonNegativeInteger(),
            )),
            //
            Axiom::SymmetricObjectProperty(SymmetricObjectProperty(iri.new("hasSpouse"))),
            Axiom::AsymmetricObjectProperty(AsymmetricObjectProperty(iri.new("hasChild"))),
            Axiom::DisjointObjectProperties(DisjointObjectProperties(
                iri.new("hasParent"),
                iri.new("hasSpouse"),
            )),
            Axiom::ReflexiveObjectProperty(ReflexiveObjectProperty(iri.new("hasRelative"))),
            Axiom::IrreflexiveObjectProperty(IrreflexiveObjectProperty(iri.new("parentOf"))),
            Axiom::FunctionalObjectProperty(FunctionalObjectProperty(iri.new("hasHusband"))),
            Axiom::InverseFunctionalObjectProperty(InverseFunctionalObjectProperty(
                iri.new("hasHusband"),
            )),
            Axiom::TransitiveObjectProperty(TransitiveObjectProperty(iri.new("hasAncestor"))),
            Axiom::FunctionalDataProperty(FunctionalDataProperty(iri.new("hasAge"))),
            //
            Axiom::SubClassOf(SubClassOf(
                iri.class("Woman").into(),
                iri.class("Person").into(),
                vec![],
            )),
            Axiom::SubClassOf(SubClassOf(
                iri.class("Mother").into(),
                iri.class("Woman").into(),
                vec![],
            )),
            Axiom::SubClassOf(SubClassOf(
                iri.class("Grandfather").into(),
                ObjectIntersectionOf(
                    vec![iri.class("Man").into(), iri.class("Parent").into()],
                    vec![],
                )
                .into(),
                vec![],
            )),
            Axiom::SubClassOf(SubClassOf(
                iri.class("Teenager").into(),
                DataSomeValuesFrom(
                    iri.new("hasAge"),
                    DatatypeRestriction(
                        wk::xsd_integer(),
                        vec![
                            Restriction::Numeric(wk::xsd_minExclusive(), Value::from(12u8)),
                            Restriction::Numeric(wk::xsd_maxInclusive(), Value::from(19u8)),
                        ],
                    ),
                )
                .into(),
                vec![],
            )),
            Axiom::SubClassOf(SubClassOf(
                iri.class("Man").into(),
                iri.class("Person").into(),
                vec![Annotation(
                    wk::rdfs_comment(),
                    Value::from("States that every man is a person"),
                )],
            )),
            Axiom::SubClassOf(SubClassOf(
                iri.class("Father").into(),
                ObjectIntersectionOf(
                    vec![iri.class("Man").into(), iri.class("Parent").into()],
                    vec![],
                )
                .into(),
                vec![],
            )),
            Axiom::SubClassOf(SubClassOf(
                iri.class("ChildlessPerson").into(),
                ObjectIntersectionOf(
                    vec![
                        iri.class("Person").into(),
                        ObjectComplementOf(
                            ObjectSomeValuesFrom(
                                ObjectInverseOf(iri.new("hasParent")).into(),
                                wk::owl_Thing(),
                                vec![],
                            )
                            .into(),
                            vec![],
                        )
                        .into(),
                    ],
                    vec![],
                )
                .into(),
                vec![],
            )),
            Axiom::SubClassOf(SubClassOf(
                ObjectIntersectionOf(
                    vec![
                        ObjectOneOf(
                            vec![iri.new("Mary"), iri.new("Bill"), iri.new("Meg")],
                            vec![],
                        )
                        .into(),
                        iri.class("Female").into(),
                    ],
                    vec![],
                )
                .into(),
                ObjectIntersectionOf(
                    vec![
                        iri.class("Parent").into(),
                        ObjectMaxCardinality(1, iri.new("hasChild"), None).into(),
                        ObjectAllValuesFrom(
                            iri.new::<ObjectPropertyIRI>("hasChild").into(),
                            iri.new("Female"),
                            vec![],
                        )
                        .into(),
                    ],
                    vec![],
                )
                .into(),
                vec![],
            )),
            //
            Axiom::EquivalentClasses(EquivalentClasses(
                iri.class("Person"),
                iri.class("Human").into(),
                vec![],
            )),
            Axiom::EquivalentClasses(EquivalentClasses(
                iri.class("Person"),
                ObjectIntersectionOf(
                    vec![iri.class("Woman").into(), iri.class("Parent").into()],
                    vec![],
                )
                .into(),
                vec![],
            )),
            Axiom::EquivalentClasses(EquivalentClasses(
                iri.class("ChildlessPerson"),
                ObjectUnionOf(
                    vec![iri.class("Mother").into(), iri.class("Father").into()],
                    vec![],
                )
                .into(),
                vec![],
            )),
            Axiom::EquivalentClasses(EquivalentClasses(
                iri.class("ChildlessPerson"),
                ObjectIntersectionOf(
                    vec![
                        iri.class("Person").into(),
                        ObjectComplementOf(iri.class("Parent").into(), vec![]).into(),
                    ],
                    vec![],
                )
                .into(),
                vec![],
            )),
            Axiom::EquivalentClasses(EquivalentClasses(
                iri.class("Parent"),
                ObjectSomeValuesFrom(
                    iri.new::<ObjectPropertyIRI>("hasChild").into(),
                    iri.class("Person"),
                    vec![],
                )
                .into(),
                vec![],
            )),
            Axiom::EquivalentClasses(EquivalentClasses(
                iri.class("HappyPerson"),
                ObjectIntersectionOf(
                    vec![
                        ObjectAllValuesFrom(
                            iri.new::<ObjectPropertyIRI>("hasChild").into(),
                            iri.class("HappyPerson"),
                            vec![],
                        )
                        .into(),
                        ObjectSomeValuesFrom(
                            iri.new::<ObjectPropertyIRI>("hasChild").into(),
                            iri.class("HappyPerson"),
                            vec![],
                        )
                        .into(),
                    ],
                    vec![],
                )
                .into(),
                vec![],
            )),
            Axiom::EquivalentClasses(EquivalentClasses(
                iri.class("JohnsChildren"),
                ObjectHasValue(
                    iri.new::<ObjectPropertyIRI>("hasParent").into(),
                    iri.new::<IRI>("John").into(),
                    vec![],
                )
                .into(),
                vec![],
            )),
            Axiom::EquivalentClasses(EquivalentClasses(
                iri.class("NarcisticPerson"),
                ObjectHasSelf(iri.new::<ObjectPropertyIRI>("loves").into(), vec![]).into(),
                vec![],
            )),
            Axiom::EquivalentClasses(EquivalentClasses(
                iri.class("MyBirthdayGuests"),
                ObjectOneOf(
                    vec![iri.new("Bill"), iri.new("John"), iri.new("Mary")],
                    vec![],
                )
                .into(),
                vec![],
            )),
            Axiom::EquivalentClasses(EquivalentClasses(
                iri.class("Orphan"),
                ObjectAllValuesFrom(
                    ObjectInverseOf(iri.new("hasChild")).into(),
                    iri.class("Dead"),
                    vec![],
                )
                .into(),
                vec![],
            )),
            Axiom::EquivalentClasses(EquivalentClasses(
                iri.class("Adult"),
                other_ont.new::<ClassIRI>("Grownup").into(),
                vec![],
            )),
            Axiom::EquivalentClasses(EquivalentClasses(
                iri.class("Parent"),
                ObjectSomeValuesFrom(
                    iri.new::<ObjectPropertyIRI>("hasChild").into(),
                    iri.class("Person"),
                    vec![],
                )
                .into(),
                vec![],
            )),
            Axiom::DisjointClasses(DisjointClasses(
                vec![iri.class("Woman").into(), iri.class("Man").into()],
                vec![],
            )),
            Axiom::DisjointClasses(DisjointClasses(
                vec![
                    iri.class("Mother").into(),
                    iri.class("Father").into(),
                    iri.class("YoungChild").into(),
                ],
                vec![],
            )),
            Axiom::HasKey(HasKey(iri.class("Person"), vec![iri.new("hasSSN")])),
            //
            Axiom::DatatypeDefinition(DatatypeDefinition(
                iri.new("personAge"),
                DatatypeRestriction(
                    wk::xsd_integer(),
                    vec![
                        Restriction::Numeric(wk::xsd_minInclusive(), Value::from(0u8)),
                        Restriction::Numeric(wk::xsd_maxInclusive(), Value::from(150u8)),
                    ],
                )
                .into(),
                vec![],
            )),
            Axiom::DatatypeDefinition(DatatypeDefinition(
                iri.new("minorAge"),
                DatatypeRestriction(
                    wk::xsd_integer(),
                    vec![
                        Restriction::Numeric(wk::xsd_minExclusive(), Value::from(0u8)),
                        Restriction::Numeric(wk::xsd_maxInclusive(), Value::from(18u8)),
                    ],
                )
                .into(),
                vec![],
            )),
            Axiom::DatatypeDefinition(DatatypeDefinition(
                iri.new("majorAge"),
                DataIntersectionOf(
                    iri.new("personAge"),
                    DataComplementOf(iri.new("minorAge")).into(),
                )
                .into(),
                vec![],
            )),
            Axiom::DatatypeDefinition(DatatypeDefinition(
                iri.new("toddlerAge"),
                DataOneOf(vec![Value::from(1u8), Value::from(2u8)]).into(),
                vec![],
            )),
            //
            Axiom::ClassAssertion(ClassAssertion(iri.class("Person").into(), iri.new("Mary"))),
            Axiom::ClassAssertion(ClassAssertion(iri.class("Woman").into(), iri.new("Mary"))),
            Axiom::ClassAssertion(ClassAssertion(
                ObjectIntersectionOf(
                    vec![
                        iri.class("Person").into(),
                        ObjectComplementOf(iri.class("Parent").into(), vec![]).into(),
                    ],
                    vec![],
                )
                .into(),
                iri.new("Jack"),
            )),
            Axiom::ClassAssertion(ClassAssertion(
                ObjectMaxCardinality(4, iri.new("hasChild"), iri.class("Parent").into()).into(),
                iri.new("John"),
            )),
            Axiom::ClassAssertion(ClassAssertion(
                ObjectMinCardinality(2, iri.new("hasChild"), iri.class("Parent").into()).into(),
                iri.new("john"),
            )),
            Axiom::ClassAssertion(ClassAssertion(
                ObjectExactCardinality(3, iri.new("hasChild"), iri.class("Parent").into()).into(),
                iri.new("john"),
            )),
            Axiom::ClassAssertion(ClassAssertion(
                ObjectExactCardinality(5, iri.new("hasChild"), None).into(),
                iri.new("john"),
            )),
            Axiom::ClassAssertion(ClassAssertion(iri.class("Father").into(), iri.new("John"))),
            Axiom::ClassAssertion(ClassAssertion(
                iri.class("SicialRole").into(),
                iri.new("Father"),
            )),
            //
            Axiom::ObjectPropertyAssertion(ObjectPropertyAssertion(
                iri.new("hasWife"),
                iri.new("John"),
                iri.new("Mary"),
            )),
            Axiom::NegativeObjectPropertyAssertion(NegativeObjectPropertyAssertion(
                iri.new("hasWife"),
                iri.new("Bill"),
                iri.new("Mary"),
            )),
            Axiom::NegativeObjectPropertyAssertion(NegativeObjectPropertyAssertion(
                iri.new("hasDaughter"),
                iri.new("Bill"),
                iri.new("Susan"),
            )),
            Axiom::DataPropertyAssertion(DataPropertyAssertion(
                iri.new("hasAge"),
                iri.new("John"),
                Value::from(51u8),
            )),
            Axiom::NegativeDataPropertyAssertion(NegativeDataPropertyAssertion(
                iri.new("hasAge"),
                iri.new("Jack"),
                Value::from(53u8),
            )),
            Axiom::SameIndividual(SameIndividual(iri.new("James"), iri.new("Jim"))),
            Axiom::SameIndividual(SameIndividual(iri.new("John"), other_ont.new("JohnBrown"))),
            Axiom::SameIndividual(SameIndividual(iri.new("Mary"), other_ont.new("MaryBrown"))),
            Axiom::DifferentIndividuals(DifferentIndividuals(iri.new("John"), iri.new("Bill"))),
        ],
    );
    ApiOntology::from((iri.base(), owl))
}
