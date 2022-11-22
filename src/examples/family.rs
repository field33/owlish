use crate::api::Ontology as ApiOntology;
use crate::owl::well_known as wk;
use crate::owl::*;

pub fn family() -> ApiOntology {
    let other_ont = IRI::builder("https://example.com/otherOnt#").unwrap();
    let iri = IRI::builder("https://example.com/family#").unwrap();
    let owl = Ontology::new(
        vec![
            Declaration::NamedIndividual {
                iri: iri.new("John"),
                annotations: vec![],
            },
            Declaration::NamedIndividual {
                iri: iri.new("Mary"),
                annotations: vec![],
            },
            Declaration::NamedIndividual {
                iri: iri.new("Jim"),
                annotations: vec![],
            },
            Declaration::NamedIndividual {
                iri: iri.new("James"),
                annotations: vec![],
            },
            Declaration::NamedIndividual {
                iri: iri.new("Jack"),
                annotations: vec![],
            },
            Declaration::NamedIndividual {
                iri: iri.new("Bill"),
                annotations: vec![],
            },
            Declaration::NamedIndividual {
                iri: iri.new("Susan"),
                annotations: vec![],
            },
            //
            Declaration::Class {
                iri: iri.new("Person"),
                annotations: vec![],
            },
            Declaration::Class {
                iri: iri.new("Woman"),
                annotations: vec![],
            },
            Declaration::Class {
                iri: iri.new("Parent"),
                annotations: vec![],
            },
            Declaration::Class {
                iri: iri.new("Father"),
                annotations: vec![],
            },
            Declaration::Class {
                iri: iri.new("Mother"),
                annotations: vec![],
            },
            Declaration::Class {
                iri: iri.new("SocialRole"),
                annotations: vec![],
            },
            Declaration::Class {
                iri: iri.new("Man"),
                annotations: vec![],
            },
            Declaration::Class {
                iri: iri.new("Teenager"),
                annotations: vec![],
            },
            Declaration::Class {
                iri: iri.new("ChildlessPerson"),
                annotations: vec![],
            },
            Declaration::Class {
                iri: iri.new("Human"),
                annotations: vec![],
            },
            Declaration::Class {
                iri: iri.new("Female"),
                annotations: vec![],
            },
            Declaration::Class {
                iri: iri.new("HappyPerson"),
                annotations: vec![],
            },
            Declaration::Class {
                iri: iri.new("JohnsChildren"),
                annotations: vec![],
            },
            Declaration::Class {
                iri: iri.new("NarcisticPerson"),
                annotations: vec![],
            },
            Declaration::Class {
                iri: iri.new("Dead"),
                annotations: vec![],
            },
            Declaration::Class {
                iri: iri.new("Orphan"),
                annotations: vec![],
            },
            Declaration::Class {
                iri: iri.new("Adult"),
                annotations: vec![],
            },
            Declaration::Class {
                iri: iri.new("YoungChild"),
                annotations: vec![],
            },
            //
            Declaration::ObjectProperty {
                iri: iri.new("hasWife"),
                annotations: vec![],
            },
            Declaration::ObjectProperty {
                iri: iri.new("hasChild"),
                annotations: vec![],
            },
            Declaration::ObjectProperty {
                iri: iri.new("hasDaughter"),
                annotations: vec![],
            },
            Declaration::ObjectProperty {
                iri: iri.new("loves"),
                annotations: vec![],
            },
            Declaration::ObjectProperty {
                iri: iri.new("hasSpouse"),
                annotations: vec![],
            },
            Declaration::ObjectProperty {
                iri: iri.new("hasGrandparent"),
                annotations: vec![],
            },
            Declaration::ObjectProperty {
                iri: iri.new("hasParent"),
                annotations: vec![],
            },
            Declaration::ObjectProperty {
                iri: iri.new("hasBrother"),
                annotations: vec![],
            },
            Declaration::ObjectProperty {
                iri: iri.new("hasUncle"),
                annotations: vec![],
            },
            Declaration::ObjectProperty {
                iri: iri.new("hasSon"),
                annotations: vec![],
            },
            Declaration::ObjectProperty {
                iri: iri.new("hasAncestor"),
                annotations: vec![],
            },
            Declaration::ObjectProperty {
                iri: iri.new("hasHusband"),
                annotations: vec![],
            },
            //
            Declaration::DataProperty {
                iri: iri.new("hasAge"),
                annotations: vec![],
            },
            Declaration::DataProperty {
                iri: iri.new("hasSSN"),
                annotations: vec![],
            },
            //
            Declaration::Datatype {
                iri: iri.new("personAge"),
                annotations: vec![],
            },
            Declaration::Datatype {
                iri: iri.new("minorAge"),
                annotations: vec![],
            },
            Declaration::Datatype {
                iri: iri.new("majorAge"),
                annotations: vec![],
            },
            Declaration::Datatype {
                iri: iri.new("toddlerAge"),
                annotations: vec![],
            },
        ],
        vec![
            Axiom::AnnotationAssertion(AnnotationAssertion::new(
                iri.new("comment"),
                iri.new("Person"),
                LiteralOrIRI::from("Represents the set of all people"),
                vec![],
            )),
            Axiom::SubObjectPropertyOf(SubObjectPropertyOf::new(
                iri.new::<ObjectPropertyIRI>("hasWife").into(),
                iri.new("hasSpouse"),
                vec![],
            )),
            Axiom::SubObjectPropertyOf(SubObjectPropertyOf::new(
                ObjectPropertyChain(vec![iri.new("hasParent"), iri.new("hasParent")]).into(),
                iri.new("hasGrandparent"),
                vec![],
            )),
            Axiom::SubObjectPropertyOf(SubObjectPropertyOf::new(
                ObjectPropertyChain(vec![iri.new("hasFather"), iri.new("hasBrother")]).into(),
                iri.new("hasUncle"),
                vec![],
            )),
            Axiom::SubObjectPropertyOf(SubObjectPropertyOf::new(
                iri.new::<ObjectPropertyIRI>("hasFather").into(),
                iri.new("hasParent"),
                vec![],
            )),
            //
            Axiom::EquivalentObjectProperties(EquivalentObjectProperties::new(
                iri.new("hasChild"),
                other_ont.new("child"),
                vec![],
            )),
            Axiom::InverseObjectProperties(InverseObjectProperties::new(
                iri.new("hasChild"),
                other_ont.new("child"),
                vec![],
            )),
            Axiom::EquivalentDataProperties(EquivalentDataProperties::new(
                iri.new("hasAge"),
                other_ont.new("age"),
                vec![],
            )),
            Axiom::DisjointObjectProperties(DisjointObjectProperties::new(
                iri.new("hasSon"),
                iri.new("hasDaughter"),
                vec![],
            )),
            Axiom::ObjectPropertyDomain(ObjectPropertyDomain::new(
                iri.new("hasWife"),
                iri.new("Man"),
                vec![],
            )),
            Axiom::ObjectPropertyRange(ObjectPropertyRange::new(
                iri.new("hasWife"),
                iri.new("Woman"),
                vec![],
            )),
            Axiom::DataPropertyDomain(DataPropertyDomain::new(
                iri.new("hasAge"),
                iri.class("Person").into(),
                vec![],
            )),
            Axiom::DataPropertyRange(DataPropertyRange::new(
                iri.new("hasAge"),
                well_known::xsd_nonNegativeInteger(),
                vec![],
            )),
            //
            Axiom::SymmetricObjectProperty(SymmetricObjectProperty::new(
                iri.new("hasSpouse"),
                vec![],
            )),
            Axiom::AsymmetricObjectProperty(AsymmetricObjectProperty::new(
                iri.new("hasChild"),
                vec![],
            )),
            Axiom::DisjointObjectProperties(DisjointObjectProperties::new(
                iri.new("hasParent"),
                iri.new("hasSpouse"),
                vec![],
            )),
            Axiom::ReflexiveObjectProperty(ReflexiveObjectProperty::new(
                iri.new("hasRelative"),
                vec![],
            )),
            Axiom::IrreflexiveObjectProperty(IrreflexiveObjectProperty::new(
                iri.new("parentOf"),
                vec![],
            )),
            Axiom::FunctionalObjectProperty(FunctionalObjectProperty::new(
                iri.new("hasHusband"),
                vec![],
            )),
            Axiom::InverseFunctionalObjectProperty(InverseFunctionalObjectProperty::new(
                iri.new("hasHusband"),
                vec![],
            )),
            Axiom::TransitiveObjectProperty(TransitiveObjectProperty::new(
                iri.new("hasAncestor"),
                vec![],
            )),
            Axiom::FunctionalDataProperty(FunctionalDataProperty::new(iri.new("hasAge"), vec![])),
            //
            Axiom::SubClassOf(SubClassOf::new(
                iri.class("Woman").into(),
                iri.class("Person").into(),
                vec![],
            )),
            Axiom::SubClassOf(SubClassOf::new(
                iri.class("Mother").into(),
                iri.class("Woman").into(),
                vec![],
            )),
            Axiom::SubClassOf(SubClassOf::new(
                iri.class("Grandfather").into(),
                ObjectIntersectionOf::new(
                    vec![iri.class("Man").into(), iri.class("Parent").into()],
                    vec![],
                )
                .into(),
                vec![],
            )),
            Axiom::SubClassOf(SubClassOf::new(
                iri.class("Teenager").into(),
                DataSomeValuesFrom::new(
                    iri.new("hasAge"),
                    DatatypeRestriction::new(
                        wk::xsd_integer(),
                        vec![
                            Restriction::Numeric {
                                datatype_iri: wk::xsd_minExclusive(),
                                value: Literal::from(12u8),
                            },
                            Restriction::Numeric {
                                datatype_iri: wk::xsd_maxInclusive(),
                                value: Literal::from(19u8),
                            },
                        ],
                        vec![],
                    ),
                    vec![],
                )
                .into(),
                vec![],
            )),
            Axiom::SubClassOf(SubClassOf::new(
                iri.class("Man").into(),
                iri.class("Person").into(),
                vec![Annotation::new(
                    wk::rdfs_comment(),
                    LiteralOrIRI::from("States that every man is a person"),
                    vec![],
                )],
            )),
            Axiom::SubClassOf(SubClassOf::new(
                iri.class("Father").into(),
                ObjectIntersectionOf::new(
                    vec![iri.class("Man").into(), iri.class("Parent").into()],
                    vec![],
                )
                .into(),
                vec![],
            )),
            Axiom::SubClassOf(SubClassOf::new(
                iri.class("ChildlessPerson").into(),
                ObjectIntersectionOf::new(
                    vec![
                        iri.class("Person").into(),
                        ObjectComplementOf::new(
                            ObjectSomeValuesFrom::new(
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
            Axiom::SubClassOf(SubClassOf::new(
                ObjectIntersectionOf::new(
                    vec![
                        ObjectOneOf::new(
                            vec![iri.new("Mary"), iri.new("Bill"), iri.new("Meg")],
                            vec![],
                        )
                        .into(),
                        iri.class("Female").into(),
                    ],
                    vec![],
                )
                .into(),
                ObjectIntersectionOf::new(
                    vec![
                        iri.class("Parent").into(),
                        ObjectMaxCardinality::new(1, iri.new("hasChild"), None).into(),
                        ObjectAllValuesFrom::new(
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
            Axiom::EquivalentClasses(EquivalentClasses::new(
                iri.class("Person"),
                iri.class("Human").into(),
                vec![],
            )),
            Axiom::EquivalentClasses(EquivalentClasses::new(
                iri.class("Person"),
                ObjectIntersectionOf::new(
                    vec![iri.class("Woman").into(), iri.class("Parent").into()],
                    vec![],
                )
                .into(),
                vec![],
            )),
            Axiom::EquivalentClasses(EquivalentClasses::new(
                iri.class("ChildlessPerson"),
                ObjectUnionOf::new(
                    vec![iri.class("Mother").into(), iri.class("Father").into()],
                    vec![],
                )
                .into(),
                vec![],
            )),
            Axiom::EquivalentClasses(EquivalentClasses::new(
                iri.class("ChildlessPerson"),
                ObjectIntersectionOf::new(
                    vec![
                        iri.class("Person").into(),
                        ObjectComplementOf::new(iri.class("Parent").into(), vec![]).into(),
                    ],
                    vec![],
                )
                .into(),
                vec![],
            )),
            Axiom::EquivalentClasses(EquivalentClasses::new(
                iri.class("Parent"),
                ObjectSomeValuesFrom::new(
                    iri.new::<ObjectPropertyIRI>("hasChild").into(),
                    iri.class("Person"),
                    vec![],
                )
                .into(),
                vec![],
            )),
            Axiom::EquivalentClasses(EquivalentClasses::new(
                iri.class("HappyPerson"),
                ObjectIntersectionOf::new(
                    vec![
                        ObjectAllValuesFrom::new(
                            iri.new::<ObjectPropertyIRI>("hasChild").into(),
                            iri.class("HappyPerson"),
                            vec![],
                        )
                        .into(),
                        ObjectSomeValuesFrom::new(
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
            Axiom::EquivalentClasses(EquivalentClasses::new(
                iri.class("JohnsChildren"),
                ObjectHasValue::new(
                    iri.new::<ObjectPropertyIRI>("hasParent").into(),
                    iri.new::<IRI>("John").into(),
                    vec![],
                )
                .into(),
                vec![],
            )),
            Axiom::EquivalentClasses(EquivalentClasses::new(
                iri.class("NarcisticPerson"),
                ObjectHasSelf::new(iri.new::<ObjectPropertyIRI>("loves").into(), vec![]).into(),
                vec![],
            )),
            Axiom::EquivalentClasses(EquivalentClasses::new(
                iri.class("MyBirthdayGuests"),
                ObjectOneOf::new(
                    vec![iri.new("Bill"), iri.new("John"), iri.new("Mary")],
                    vec![],
                )
                .into(),
                vec![],
            )),
            Axiom::EquivalentClasses(EquivalentClasses::new(
                iri.class("Orphan"),
                ObjectAllValuesFrom::new(
                    ObjectInverseOf(iri.new("hasChild")).into(),
                    iri.class("Dead"),
                    vec![],
                )
                .into(),
                vec![],
            )),
            Axiom::EquivalentClasses(EquivalentClasses::new(
                iri.class("Adult"),
                other_ont.new::<ClassIRI>("Grownup").into(),
                vec![],
            )),
            Axiom::EquivalentClasses(EquivalentClasses::new(
                iri.class("Parent"),
                ObjectSomeValuesFrom::new(
                    iri.new::<ObjectPropertyIRI>("hasChild").into(),
                    iri.class("Person"),
                    vec![],
                )
                .into(),
                vec![],
            )),
            Axiom::DisjointClasses(DisjointClasses::new(
                vec![iri.class("Woman").into(), iri.class("Man").into()],
                vec![],
            )),
            Axiom::DisjointClasses(DisjointClasses::new(
                vec![
                    iri.class("Mother").into(),
                    iri.class("Father").into(),
                    iri.class("YoungChild").into(),
                ],
                vec![],
            )),
            Axiom::HasKey(HasKey::new(
                iri.class("Person"),
                vec![iri.new("hasSSN")],
                vec![],
            )),
            //
            Axiom::DatatypeDefinition(DatatypeDefinition::new(
                iri.new("personAge"),
                DatatypeRestriction::new(
                    wk::xsd_integer(),
                    vec![
                        Restriction::Numeric {
                            datatype_iri: wk::xsd_minInclusive(),
                            value: Literal::from(0u8),
                        },
                        Restriction::Numeric {
                            datatype_iri: wk::xsd_maxInclusive(),
                            value: Literal::from(150u8),
                        },
                    ],
                    vec![],
                )
                .into(),
                vec![],
            )),
            Axiom::DatatypeDefinition(DatatypeDefinition::new(
                iri.new("minorAge"),
                DatatypeRestriction::new(
                    wk::xsd_integer(),
                    vec![
                        Restriction::Numeric {
                            datatype_iri: wk::xsd_minExclusive(),
                            value: Literal::from(0u8),
                        },
                        Restriction::Numeric {
                            datatype_iri: wk::xsd_maxInclusive(),
                            value: Literal::from(18u8),
                        },
                    ],
                    vec![],
                )
                .into(),
                vec![],
            )),
            Axiom::DatatypeDefinition(DatatypeDefinition::new(
                iri.new("majorAge"),
                DataIntersectionOf::new(
                    iri.new("personAge"),
                    DataComplementOf::new(iri.new("minorAge"), vec![]).into(),
                    vec![],
                )
                .into(),
                vec![],
            )),
            Axiom::DatatypeDefinition(DatatypeDefinition::new(
                iri.new("toddlerAge"),
                DataOneOf::new(vec![Literal::from(1u8), Literal::from(2u8)], vec![]).into(),
                vec![],
            )),
            //
            Axiom::ClassAssertion(ClassAssertion::new(
                iri.class("Person").into(),
                iri.new("Mary"),
                vec![],
            )),
            Axiom::ClassAssertion(ClassAssertion::new(
                iri.class("Woman").into(),
                iri.new("Mary"),
                vec![],
            )),
            Axiom::ClassAssertion(ClassAssertion::new(
                ObjectIntersectionOf::new(
                    vec![
                        iri.class("Person").into(),
                        ObjectComplementOf::new(iri.class("Parent").into(), vec![]).into(),
                    ],
                    vec![],
                )
                .into(),
                iri.new("Jack"),
                vec![],
            )),
            Axiom::ClassAssertion(ClassAssertion::new(
                ObjectMaxCardinality::new(4, iri.new("hasChild"), iri.class("Parent").into())
                    .into(),
                iri.new("John"),
                vec![],
            )),
            Axiom::ClassAssertion(ClassAssertion::new(
                ObjectMinCardinality::new(2, iri.new("hasChild"), iri.class("Parent").into())
                    .into(),
                iri.new("john"),
                vec![],
            )),
            Axiom::ClassAssertion(ClassAssertion::new(
                ObjectExactCardinality::new(3, iri.new("hasChild"), iri.class("Parent").into())
                    .into(),
                iri.new("john"),
                vec![],
            )),
            Axiom::ClassAssertion(ClassAssertion::new(
                ObjectExactCardinality::new(5, iri.new("hasChild"), None).into(),
                iri.new("john"),
                vec![],
            )),
            Axiom::ClassAssertion(ClassAssertion::new(
                iri.class("Father").into(),
                iri.new("John"),
                vec![],
            )),
            Axiom::ClassAssertion(ClassAssertion::new(
                iri.class("SicialRole").into(),
                iri.new("Father"),
                vec![],
            )),
            //
            Axiom::ObjectPropertyAssertion(ObjectPropertyAssertion::new(
                iri.new("hasWife"),
                iri.new("John"),
                iri.new("Mary"),
                vec![],
            )),
            Axiom::NegativeObjectPropertyAssertion(NegativeObjectPropertyAssertion::new(
                iri.new("hasWife"),
                iri.new("Bill"),
                iri.new("Mary"),
                vec![],
            )),
            Axiom::NegativeObjectPropertyAssertion(NegativeObjectPropertyAssertion::new(
                iri.new("hasDaughter"),
                iri.new("Bill"),
                iri.new("Susan"),
                vec![],
            )),
            Axiom::DataPropertyAssertion(DataPropertyAssertion::new(
                iri.new("hasAge"),
                iri.new("John"),
                Literal::from(51u8),
                vec![],
            )),
            Axiom::NegativeDataPropertyAssertion(NegativeDataPropertyAssertion::new(
                iri.new("hasAge"),
                iri.new("Jack"),
                Literal::from(53u8),
                vec![],
            )),
            Axiom::SameIndividual(SameIndividual::new(
                iri.new("James"),
                iri.new("Jim"),
                vec![],
            )),
            Axiom::SameIndividual(SameIndividual::new(
                iri.new("John"),
                other_ont.new("JohnBrown"),
                vec![],
            )),
            Axiom::SameIndividual(SameIndividual::new(
                iri.new("Mary"),
                other_ont.new("MaryBrown"),
                vec![],
            )),
            Axiom::DifferentIndividuals(DifferentIndividuals::new(
                iri.new("John"),
                iri.new("Bill"),
                vec![],
            )),
        ],
    );
    ApiOntology::from((iri.base(), owl))
}
