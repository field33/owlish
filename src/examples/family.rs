use crate::api::Ontology as ApiOntology;
use crate::owl::well_known as wk;
use crate::owl::*;

pub fn family() -> ApiOntology {
    let other_ont = IRI::builder("https://example.com/otherOnt#").unwrap();
    let iri = IRI::builder("https://example.com/family#").unwrap();
    let owl = Ontology::new(
        vec![
            Declaration::NamedIndividual(iri.new("John"), vec![]),
            Declaration::NamedIndividual(iri.new("Mary"), vec![]),
            Declaration::NamedIndividual(iri.new("Jim"), vec![]),
            Declaration::NamedIndividual(iri.new("James"), vec![]),
            Declaration::NamedIndividual(iri.new("Jack"), vec![]),
            Declaration::NamedIndividual(iri.new("Bill"), vec![]),
            Declaration::NamedIndividual(iri.new("Susan"), vec![]),
            //
            Declaration::Class(iri.new("Person"), vec![]),
            Declaration::Class(iri.new("Woman"), vec![]),
            Declaration::Class(iri.new("Parent"), vec![]),
            Declaration::Class(iri.new("Father"), vec![]),
            Declaration::Class(iri.new("Mother"), vec![]),
            Declaration::Class(iri.new("SocialRole"), vec![]),
            Declaration::Class(iri.new("Man"), vec![]),
            Declaration::Class(iri.new("Teenager"), vec![]),
            Declaration::Class(iri.new("ChildlessPerson"), vec![]),
            Declaration::Class(iri.new("Human"), vec![]),
            Declaration::Class(iri.new("Female"), vec![]),
            Declaration::Class(iri.new("HappyPerson"), vec![]),
            Declaration::Class(iri.new("JohnsChildren"), vec![]),
            Declaration::Class(iri.new("NarcisticPerson"), vec![]),
            Declaration::Class(iri.new("Dead"), vec![]),
            Declaration::Class(iri.new("Orphan"), vec![]),
            Declaration::Class(iri.new("Adult"), vec![]),
            Declaration::Class(iri.new("YoungChild"), vec![]),
            //
            Declaration::ObjectProperty(iri.new("hasWife"), vec![]),
            Declaration::ObjectProperty(iri.new("hasChild"), vec![]),
            Declaration::ObjectProperty(iri.new("hasDaughter"), vec![]),
            Declaration::ObjectProperty(iri.new("loves"), vec![]),
            Declaration::ObjectProperty(iri.new("hasSpouse"), vec![]),
            Declaration::ObjectProperty(iri.new("hasGrandparent"), vec![]),
            Declaration::ObjectProperty(iri.new("hasParent"), vec![]),
            Declaration::ObjectProperty(iri.new("hasBrother"), vec![]),
            Declaration::ObjectProperty(iri.new("hasUncle"), vec![]),
            Declaration::ObjectProperty(iri.new("hasSon"), vec![]),
            Declaration::ObjectProperty(iri.new("hasAncestor"), vec![]),
            Declaration::ObjectProperty(iri.new("hasHusband"), vec![]),
            //
            Declaration::DataProperty(iri.new("hasAge"), vec![]),
            Declaration::DataProperty(iri.new("hasSSN"), vec![]),
            //
            Declaration::Datatype(iri.new("personAge"), vec![]),
            Declaration::Datatype(iri.new("minorAge"), vec![]),
            Declaration::Datatype(iri.new("majorAge"), vec![]),
            Declaration::Datatype(iri.new("toddlerAge"), vec![]),
        ],
        vec![
            Axiom::AnnotationAssertion(AnnotationAssertion(
                iri.new("comment"),
                iri.new("Person"),
                LiteralOrIRI::from("Represents the set of all people"),
                vec![],
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
                            Restriction::Numeric(wk::xsd_minExclusive(), Literal::from(12u8)),
                            Restriction::Numeric(wk::xsd_maxInclusive(), Literal::from(19u8)),
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
                    LiteralOrIRI::from("States that every man is a person"),
                    vec![],
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
                        Restriction::Numeric(wk::xsd_minInclusive(), Literal::from(0u8)),
                        Restriction::Numeric(wk::xsd_maxInclusive(), Literal::from(150u8)),
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
                        Restriction::Numeric(wk::xsd_minExclusive(), Literal::from(0u8)),
                        Restriction::Numeric(wk::xsd_maxInclusive(), Literal::from(18u8)),
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
                DataOneOf(vec![Literal::from(1u8), Literal::from(2u8)]).into(),
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
                vec![],
            )),
            Axiom::NegativeObjectPropertyAssertion(NegativeObjectPropertyAssertion(
                iri.new("hasWife"),
                iri.new("Bill"),
                iri.new("Mary"),
                vec![],
            )),
            Axiom::NegativeObjectPropertyAssertion(NegativeObjectPropertyAssertion(
                iri.new("hasDaughter"),
                iri.new("Bill"),
                iri.new("Susan"),
                vec![],
            )),
            Axiom::DataPropertyAssertion(DataPropertyAssertion(
                iri.new("hasAge"),
                iri.new("John"),
                Literal::from(51u8),
                vec![],
            )),
            Axiom::NegativeDataPropertyAssertion(NegativeDataPropertyAssertion(
                iri.new("hasAge"),
                iri.new("Jack"),
                Literal::from(53u8),
                vec![],
            )),
            Axiom::SameIndividual(SameIndividual(iri.new("James"), iri.new("Jim"))),
            Axiom::SameIndividual(SameIndividual(iri.new("John"), other_ont.new("JohnBrown"))),
            Axiom::SameIndividual(SameIndividual(iri.new("Mary"), other_ont.new("MaryBrown"))),
            Axiom::DifferentIndividuals(DifferentIndividuals(iri.new("John"), iri.new("Bill"))),
        ],
    );
    ApiOntology::from((iri.base(), owl))
}
