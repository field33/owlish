use owlish::owl::{IRIList, ResourceId};
use owlish::{
    api::Ontology,
    owl::{
        well_known, Annotation, AnnotationAssertion, Axiom, DataPropertyAssertion, Declaration,
        Literal, LiteralOrIRI, ObjectPropertyAssertion, SubClassOf, IRI,
    },
    parser::{ParserOptions, ParserOptionsBuilder},
};

#[test]
fn annotations_on_sub_class_of() {
    env_logger::try_init().ok();
    let turtle = r##"
        @prefix : <http://test#> .
        @prefix owl: <http://www.w3.org/2002/07/owl#> .
        @prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
        @prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
        @prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

        <http://test#> rdf:type owl:Ontology .

        :Man rdf:type owl:Class .
        :Person rdf:type owl:Class .

        :Man rdfs:subClassOf        :Person .
        []   rdf:type               owl:Axiom ;
             owl:annotatedSource    :Man ;
             owl:annotatedProperty  rdfs:subClassOf ;
             owl:annotatedTarget    :Person ;
             rdfs:comment           "States that every man is a person."^^xsd:string .

        "##;

    harriet::TurtleDocument::parse_full(turtle).unwrap();
    let o = Ontology::parse(turtle, Default::default()).unwrap();

    println!("{:#?}", o);
    assert_eq!(o.declarations().len(), 2);
    assert_eq!(o.axioms().len(), 2);
    assert_eq!(
        o.axioms()[1],
        Axiom::SubClassOf(SubClassOf::new(
            IRI::new("http://test#Man").unwrap().into(),
            IRI::new("http://test#Person").unwrap().into(),
            vec![Annotation::new(
                well_known::rdfs_comment(),
                LiteralOrIRI::Literal(Literal::String("States that every man is a person.".into())),
                vec![]
            )]
        ))
    );
}

/// Reification test:
/// What is reified: AnnotationAssertion
/// Reification subject:
///   - Subject: IRI
///   - Object: Literal
/// Reification ID: BlankNode
/// Assertions stated on reification: AnnotationAssertion
#[test]
fn annotations_on_annotations() {
    env_logger::try_init().ok();
    let turtle = r##"
        @prefix : <http://test#> .
        @prefix owl: <http://www.w3.org/2002/07/owl#> .
        @prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
        @prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
        @prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

        <http://test#> rdf:type owl:Ontology .

        :Man rdf:type owl:Class .
        :bla rdf:type owl:AnnotationProperty .

        :Man :bla "test" .
        []   rdf:type               owl:Axiom ;
             owl:annotatedSource    :Man ;
             owl:annotatedProperty  :bla ;
             owl:annotatedTarget    "test" ;
             rdfs:comment           "States that every man is a person."^^xsd:string .

        "##;

    harriet::TurtleDocument::parse_full(turtle).unwrap();
    let o = Ontology::parse(turtle, Default::default()).unwrap();

    println!("{:#?}", o);
    assert_eq!(o.declarations().len(), 2);
    assert_eq!(o.axioms().len(), 2);

    let Axiom::AnnotationAssertion(reified_annotation) = o.axioms()[0].clone() else {
        panic!("Did not parse as AnnotationAssertion");
    };
    assert_eq!(
        reified_annotation.subject,
        IRI::new("http://test#Man").unwrap().into()
    );
    assert_eq!(reified_annotation.iri, IRI::new("http://test#bla").unwrap().into());
    assert_eq!(
        reified_annotation.value,
        Literal::String("test".into()).into(),
    );
    assert_eq!(reified_annotation.resource_ids.len(), 1);
    assert!(reified_annotation.resource_ids[0].is_blank_node());

    let annotations_on_annotation =
        o.annotation_assertions_for_resource_id(&reified_annotation.resource_ids[0]);
    assert_eq!(annotations_on_annotation.len(), 1);
    assert_eq!(
        annotations_on_annotation[0].clone(),
        AnnotationAssertion::new(
            well_known::rdfs_comment(),
            reified_annotation.resource_ids[0].clone(),
            LiteralOrIRI::Literal(Literal::String("States that every man is a person.".into())),
            vec![],
            vec![]
        )
    );
}

#[test]
fn reification() {
    env_logger::try_init().ok();
    let turtle = r##"
            @prefix : <http://test#> .
            @prefix owl: <http://www.w3.org/2002/07/owl#> .
            @prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
            @prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
            @prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

            <http://test#> rdf:type owl:Ontology .

            :Bob rdf:type owl:NamedIndividual .
            :description rdf:type owl:AnnotationProperty .
            :hasAge rdf:type owl:DatatypeProperty .
            :createdAt rdf:type owl:AnnotationProperty .

            :Annotation1 rdf:type owl:Axiom .
            :Annotation1 owl:annotatedSource :Bob .
            :Annotation1 owl:annotatedProperty :description .
            :Annotation1 owl:annotatedTarget "Bob is Bob" .

            :DPA1 rdf:type owl:Axiom .
            :DPA1 owl:annotatedSource :Bob .
            :DPA1 owl:annotatedProperty :hasAge .
            :DPA1 owl:annotatedTarget 123 .

            # Annotation on annotation
            :Annotation1 :createdAt "2019" .

            :Bob :description "Bob is Bob" .

            :Bob :hasAge 123 .
        "##;

    let options = ParserOptions::builder().build();

    harriet::TurtleDocument::parse_full(turtle).unwrap();
    let o = Ontology::parse(turtle, options).unwrap();

    println!("{:#?}", o);
    assert_eq!(o.declarations().len(), 4);
    assert_eq!(o.axioms().len(), 3);
    assert_eq!(
        o.axioms()[0],
        AnnotationAssertion::new(
            IRI::new("http://test#createdAt").unwrap().into(),
            IRI::new("http://test#Annotation1").unwrap(),
            Literal::String("2019".into()).into(),
            vec![],
            vec![]
        )
        .into()
    );
    assert_eq!(
        o.axioms()[1],
        AnnotationAssertion::new(
            IRI::new("http://test#description").unwrap().into(),
            IRI::new("http://test#Bob").unwrap(),
            Literal::String("Bob is Bob".into()).into(),
            vec![],
            vec![ResourceId::IRI(
                IRI::new("http://test#Annotation1").unwrap()
            )]
        )
        .into()
    );
    assert_eq!(
        o.axioms()[2],
        DataPropertyAssertion::new(
            IRI::new("http://test#hasAge").unwrap().into(),
            IRI::new("http://test#Bob").unwrap().into(),
            Literal::Number {
                number: 123.into(),
                type_iri: Some(well_known::xsd_integer())
            },
            vec![],
            vec![ResourceId::IRI(IRI::new("http://test#DPA1").unwrap())]
        )
        .into()
    );

    let annotations_on_annotation = o.annotation_assertions_for_resource_id(&ResourceId::IRI(
        IRI::new("http://test#Annotation1").unwrap(),
    ));
    assert_eq!(annotations_on_annotation.len(), 1);
    assert_eq!(
        Axiom::AnnotationAssertion(annotations_on_annotation[0].clone()),
        o.axioms()[0]
    );
}

#[test]
fn annotations_on_annotations_unsorted() {
    env_logger::try_init().ok();
    let turtle = r##"
            <http://field33.com/query_result/bcb90f6f-d1bf-42ea-b5f3-249d7d56fad1> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://www.w3.org/2002/07/owl#Ontology> .

            _:e8ba5acdba3222687d32b847815b45f9 <http://www.w3.org/2002/07/owl#annotatedSource> <http://field33.com/dataset/foobar#7025935> .

            _:e8ba5acdba3222687d32b847815b45f9 <http://query-server.field33.com/ontology/query-field> "labels" .
            _:e8ba5acdba3222687d32b847815b45f9 <http://www.w3.org/2002/07/owl#annotatedTarget> "Lorem Ipsum" .
            _:e8ba5acdba3222687d32b847815b45f9 <http://www.w3.org/2002/07/owl#annotatedProperty> <http://www.w3.org/2000/01/rdf-schema#label> .
            _:e8ba5acdba3222687d32b847815b45f9 <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://www.w3.org/2002/07/owl#Axiom> .
            <http://field33.com/dataset/foobar#7025935> <http://www.w3.org/2000/01/rdf-schema#label> "Lorem Ipsum" .
            <http://field33.com/dataset/foobar#7025935> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://www.w3.org/2002/07/owl#NamedIndividual> .
        "##;

    let options = ParserOptions::builder()
        .known(Declaration::AnnotationProperty {
            iri: IRI::new("http://query-server.field33.com/ontology/query-field")
                .unwrap()
                .into(),
            annotations: vec![],
        })
        .build();

    harriet::TurtleDocument::parse_full(turtle).unwrap();
    let o = Ontology::parse(turtle, options).unwrap();

    println!("{:#?}", o);
    assert_eq!(o.declarations().len(), 1);
    assert_eq!(o.axioms().len(), 2);

    let Axiom::AnnotationAssertion(reified_annotation) = o.axioms()[1].clone() else {
            panic!("Did not parse as AnnotationAssertion");
        };
    assert_eq!(
        reified_annotation.subject,
        IRI::new("http://field33.com/dataset/foobar#7025935")
            .unwrap()
            .into()
    );
    assert_eq!(reified_annotation.iri, well_known::rdfs_label());
    assert_eq!(
        reified_annotation.value,
        Literal::String("Lorem Ipsum".into()).into()
    );
    assert_eq!(reified_annotation.resource_ids.len(), 1);
    assert!(reified_annotation.resource_ids[0].is_blank_node());

    let annotations_on_annotation =
        o.annotation_assertions_for_resource_id(&reified_annotation.resource_ids[0]);
    assert_eq!(annotations_on_annotation.len(), 1);
    assert_eq!(
        annotations_on_annotation[0].clone(),
        AnnotationAssertion::new(
            IRI::new("http://query-server.field33.com/ontology/query-field")
                .unwrap()
                .into(),
            reified_annotation.resource_ids[0].clone(),
            Literal::String("labels".into()).into(),
            vec![],
            vec![]
        )
    );
}

/// What is reified: DataPropertyAssertion
/// Reification subject:
///   - Subject: IRI
///   - Object: Literal
/// Reification ID: BlankNode
/// Assertions stated on reification: AnnotationAssertion
#[test]
fn annotations_on_data_property_assertions() {
    env_logger::try_init().ok();
    let turtle = r##"
            <http://field33.com/query_result/00000000-0000-0000-0000-000000000000> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://www.w3.org/2002/07/owl#Ontology> .
            _:896a965c9c5ef70e6855ff27a3009712 <http://query-server.field33.com/ontology/query-field> "index-1" .
            _:896a965c9c5ef70e6855ff27a3009712 <http://www.w3.org/2002/07/owl#annotatedTarget> "0"^^<http://www.w3.org/2001/XMLSchema#decimal> .
            _:896a965c9c5ef70e6855ff27a3009712 <http://www.w3.org/2002/07/owl#annotatedProperty> <http://field33.com/ontologies/@fld33_domain/dora_metrics/TeamChangeFailureRate> .
            _:896a965c9c5ef70e6855ff27a3009712 <http://www.w3.org/2002/07/owl#annotatedSource> <http://field33.com/ontologies/@fld33_domain/dora_metrics/Team2> .
            _:896a965c9c5ef70e6855ff27a3009712 <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://www.w3.org/2002/07/owl#Axiom> .
            <http://field33.com/ontologies/@fld33_domain/dora_metrics/Team2> <http://field33.com/ontologies/@fld33_domain/dora_metrics/TeamChangeFailureRate> "0"^^<http://www.w3.org/2001/XMLSchema#decimal> .

        "##;

    harriet::TurtleDocument::parse_full(turtle).unwrap();
    let o = Ontology::parse(
            turtle,
            ParserOptionsBuilder::default()
                .known(Declaration::AnnotationProperty{
                    iri: IRI::new("http://query-server.field33.com/ontology/query-field")
                        .unwrap()
                        .into(),
                    annotations: vec![],
                })
                .known(Declaration::DataProperty{
                    iri: IRI::new("http://field33.com/ontologies/@fld33_domain/dora_metrics/TeamChangeFailureRate")
                        .unwrap()
                        .into(),
                    annotations: vec![],
                })
                .build(),
        )
            .unwrap();
    println!("{:#?}", o);
    assert_eq!(o.declarations().len(), 0);
    assert_eq!(o.axioms().len(), 2);

    let Axiom::DataPropertyAssertion(dpa) = o.axioms()[1].clone() else {
            panic!("Not an OPA")
        };
    assert_eq!(
        dpa.subject,
        IRI::new("http://field33.com/ontologies/@fld33_domain/dora_metrics/Team2")
            .unwrap()
            .into()
    );
    assert_eq!(
        dpa.iri,
        IRI::new("http://field33.com/ontologies/@fld33_domain/dora_metrics/TeamChangeFailureRate")
            .unwrap()
            .into()
    );
    assert_eq!(
        dpa.value,
        Literal::Number {
            number: 0.into(),
            type_iri: Some(well_known::xsd_decimal())
        }
    );

    assert_eq!(dpa.resource_ids.len(), 1);
    assert!(dpa.resource_ids[0].is_blank_node());

    let annotations_on_annotation = o.annotation_assertions_for_resource_id(&dpa.resource_ids[0]);
    assert_eq!(annotations_on_annotation.len(), 1);
    assert_eq!(
        annotations_on_annotation[0].clone(),
        AnnotationAssertion::new(
            IRI::new("http://query-server.field33.com/ontology/query-field")
                .unwrap()
                .into(),
            dpa.resource_ids[0].clone(),
            // LiteralOrIRI::IRI(IRI::new("urn:test").unwrap().into()),
            LiteralOrIRI::Literal("index-1".into()),
            vec![],
            vec![]
        )
    );
}

/// What is reified: DataPropertyAssertion
/// Reification subject:
///   - Subject: IRI
///   - Object: Literal
/// Reification ID: BlankNode
/// Assertions stated on reification: AnnotationAssertion with IRI object
#[test]
fn annotations_on_data_property_assertions_with_iri_object() {
    env_logger::try_init().ok();
    let turtle = r##"
            <http://field33.com/query_result/00000000-0000-0000-0000-000000000000> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://www.w3.org/2002/07/owl#Ontology> .
            _:896a965c9c5ef70e6855ff27a3009712 <http://query-server.field33.com/ontology/query-field> <urn:test> .
            _:896a965c9c5ef70e6855ff27a3009712 <http://www.w3.org/2002/07/owl#annotatedTarget> "0"^^<http://www.w3.org/2001/XMLSchema#decimal> .
            _:896a965c9c5ef70e6855ff27a3009712 <http://www.w3.org/2002/07/owl#annotatedProperty> <http://field33.com/ontologies/@fld33_domain/dora_metrics/TeamChangeFailureRate> .
            _:896a965c9c5ef70e6855ff27a3009712 <http://www.w3.org/2002/07/owl#annotatedSource> <http://field33.com/ontologies/@fld33_domain/dora_metrics/Team2> .
            _:896a965c9c5ef70e6855ff27a3009712 <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://www.w3.org/2002/07/owl#Axiom> .
            <http://field33.com/ontologies/@fld33_domain/dora_metrics/Team2> <http://field33.com/ontologies/@fld33_domain/dora_metrics/TeamChangeFailureRate> "0"^^<http://www.w3.org/2001/XMLSchema#decimal> .

        "##;

    harriet::TurtleDocument::parse_full(turtle).unwrap();
    let o = Ontology::parse(
            turtle,
            ParserOptionsBuilder::default()
                .known(Declaration::AnnotationProperty{
                    iri: IRI::new("http://query-server.field33.com/ontology/query-field")
                        .unwrap()
                        .into(),
                    annotations: vec![],
                })
                .known(Declaration::DataProperty{
                    iri: IRI::new("http://field33.com/ontologies/@fld33_domain/dora_metrics/TeamChangeFailureRate")
                        .unwrap()
                        .into(),
                    annotations: vec![],
                })
                .build(),
        )
            .unwrap();
    println!("{:#?}", o);
    assert_eq!(o.declarations().len(), 0);
    assert_eq!(o.axioms().len(), 2);

    let Axiom::DataPropertyAssertion(dpa) = o.axioms()[1].clone() else {
            panic!("Not an OPA")
        };
    assert_eq!(
        dpa.subject,
        IRI::new("http://field33.com/ontologies/@fld33_domain/dora_metrics/Team2")
            .unwrap()
            .into()
    );
    assert_eq!(
        dpa.iri,
        IRI::new("http://field33.com/ontologies/@fld33_domain/dora_metrics/TeamChangeFailureRate")
            .unwrap()
            .into()
    );
    assert_eq!(
        dpa.value,
        Literal::Number {
            number: 0.into(),
            type_iri: Some(well_known::xsd_decimal())
        }
    );

    assert_eq!(dpa.resource_ids.len(), 1);
    assert!(dpa.resource_ids[0].is_blank_node());

    let annotations_on_annotation = o.annotation_assertions_for_resource_id(&dpa.resource_ids[0]);
    assert_eq!(annotations_on_annotation.len(), 1);
    assert_eq!(
        annotations_on_annotation[0].clone(),
        AnnotationAssertion::new(
            IRI::new("http://query-server.field33.com/ontology/query-field")
                .unwrap()
                .into(),
            dpa.resource_ids[0].clone(),
            LiteralOrIRI::IRI(IRI::new("urn:test").unwrap().into()),
            vec![],
            vec![]
        )
    );
}

/// What is reified: ObjectPropertyAssertion
/// Reification subject:
///   - Subject: IRI
///   - Object: IRI
/// Reification ID: IRI
/// Assertions stated on reification: None
#[test]
fn annotations_on_object_property_assertions() {
    env_logger::try_init().ok();
    let turtle = r##"
            <http://field33.com/query_result/00000000-0000-0000-0000-000000000000> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://www.w3.org/2002/07/owl#Ontology> .
            <http://field33.com/query_result/2060abc6-f459-47ed-9248-0b7fe12c971c> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://www.w3.org/2002/07/owl#Ontology> .
            <http://www.w3.org/2000/01/rdf-schema#label> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://www.w3.org/2002/07/owl#AnnotationProperty> .
            <http://field33.com/ontologies/@fld33/relations/Has> <http://www.w3.org/2000/01/rdf-schema#label> "Has"@en .
            <http://field33.com/ontologies/@fld33/relations/Has> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://www.w3.org/2002/07/owl#ObjectProperty> .
            <http://field33.com/org/org_evlGiemVNyAUTJ7D/node/f63c8031-a7d9-40db-ae87-be04c99537c7> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://www.w3.org/2002/07/owl#Axiom> .
            <http://field33.com/org/org_evlGiemVNyAUTJ7D/node/f63c8031-a7d9-40db-ae87-be04c99537c7> <http://www.w3.org/2002/07/owl#annotatedTarget> <http://field33.com/org/org_evlGiemVNyAUTJ7D/node/fe6fdda1-fc21-4b99-9269-8c19fc6359b8> .
            <http://field33.com/org/org_evlGiemVNyAUTJ7D/node/f63c8031-a7d9-40db-ae87-be04c99537c7> <http://www.w3.org/2002/07/owl#annotatedProperty> <http://field33.com/ontologies/@fld33/relations/Has> .
            <http://field33.com/org/org_evlGiemVNyAUTJ7D/node/f63c8031-a7d9-40db-ae87-be04c99537c7> <http://www.w3.org/2002/07/owl#annotatedSource> <http://field33.com/org/org_evlGiemVNyAUTJ7D/node/1afda1af-bbde-48de-a5d7-5f43d389b2a6> .
            <http://field33.com/org/org_evlGiemVNyAUTJ7D/node/1afda1af-bbde-48de-a5d7-5f43d389b2a6> <http://field33.com/ontologies/@fld33/relations/Has> <http://field33.com/org/org_evlGiemVNyAUTJ7D/node/fe6fdda1-fc21-4b99-9269-8c19fc6359b8> .


        "##;

    harriet::TurtleDocument::parse_full(turtle).unwrap();
    let o = Ontology::parse(
        turtle,
        ParserOptionsBuilder::default()
            .known(Declaration::AnnotationProperty {
                iri: IRI::new("http://query-server.field33.com/ontology/query-field")
                    .unwrap()
                    .into(),
                annotations: vec![],
            })
            .build(),
    )
    .unwrap();
    println!("{:#?}", o);
    assert_eq!(o.declarations().len(), 2);
    assert_eq!(o.axioms().len(), 2);
    assert_eq!(
            o.axioms()[1],
            Axiom::ObjectPropertyAssertion(ObjectPropertyAssertion::new(
                IRI::new("http://field33.com/ontologies/@fld33/relations/Has")
                    .unwrap()
                    .into(),
                IRI::new("http://field33.com/org/org_evlGiemVNyAUTJ7D/node/1afda1af-bbde-48de-a5d7-5f43d389b2a6")
                    .unwrap()
                    .into(),
                IRI::new("http://field33.com/org/org_evlGiemVNyAUTJ7D/node/fe6fdda1-fc21-4b99-9269-8c19fc6359b8")
                    .unwrap()
                    .into(),
                vec![],
                vec![ResourceId::IRI(IRI::new("http://field33.com/org/org_evlGiemVNyAUTJ7D/node/f63c8031-a7d9-40db-ae87-be04c99537c7").unwrap())]
            ))
        );
}

/// What is reified: ObjectPropertyAssertion
/// Reification subject:
///   - Subject: IRI
///   - Object: IRI
/// Reification ID: BlankNode
/// Assertions stated on reification: AnnotationAssertion
#[test]
fn annotations_on_object_property_assertions_blank_node() {
    env_logger::try_init().ok();
    let turtle = r##"
            <http://field33.com/query_result/00000000-0000-0000-0000-000000000000> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://www.w3.org/2002/07/owl#Ontology> .
            <http://field33.com/query_result/2060abc6-f459-47ed-9248-0b7fe12c971c> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://www.w3.org/2002/07/owl#Ontology> .
            <http://www.w3.org/2000/01/rdf-schema#label> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://www.w3.org/2002/07/owl#AnnotationProperty> .
            <http://field33.com/ontologies/@fld33/relations/Has> <http://www.w3.org/2000/01/rdf-schema#label> "Has"@en .
            <http://field33.com/ontologies/@fld33/relations/Has> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://www.w3.org/2002/07/owl#ObjectProperty> .
            <http://field33.com/ontologies/core_change_tracking/createdByImport> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://www.w3.org/2002/07/owl#AnnotationProperty> .
            _:f63c8031 <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://www.w3.org/2002/07/owl#Axiom> .
            _:f63c8031 <http://www.w3.org/2002/07/owl#annotatedSource> <http://field33.com/org/org_evlGiemVNyAUTJ7D/node/1afda1af-bbde-48de-a5d7-5f43d389b2a6> .
            _:f63c8031 <http://www.w3.org/2002/07/owl#annotatedProperty> <http://field33.com/ontologies/@fld33/relations/Has> .
            _:f63c8031 <http://www.w3.org/2002/07/owl#annotatedTarget> <http://field33.com/org/org_evlGiemVNyAUTJ7D/node/fe6fdda1-fc21-4b99-9269-8c19fc6359b8> .
            _:f63c8031 <http://field33.com/ontologies/core_change_tracking/createdByImport> "GitHub" .
            <http://field33.com/org/org_evlGiemVNyAUTJ7D/node/1afda1af-bbde-48de-a5d7-5f43d389b2a6> <http://field33.com/ontologies/@fld33/relations/Has> <http://field33.com/org/org_evlGiemVNyAUTJ7D/node/fe6fdda1-fc21-4b99-9269-8c19fc6359b8> .
        "##;

    harriet::TurtleDocument::parse_full(turtle).unwrap();
    let o = Ontology::parse(
        turtle,
        ParserOptionsBuilder::default()
            .known(Declaration::AnnotationProperty {
                iri: IRI::new("http://query-server.field33.com/ontology/query-field")
                    .unwrap()
                    .into(),
                annotations: vec![],
            })
            .build(),
    )
    .unwrap();
    println!("{:#?}", o);
    assert_eq!(o.declarations().len(), 3);
    assert_eq!(o.axioms().len(), 3);
    let Axiom::ObjectPropertyAssertion(opa) = o.axioms()[2].clone() else {
            panic!("Not an OPA")
        };
    assert_eq!(
        opa.iri,
        IRI::new("http://field33.com/ontologies/@fld33/relations/Has")
            .unwrap()
            .into()
    );
    assert_eq!(
        opa.subject,
        IRI::new(
            "http://field33.com/org/org_evlGiemVNyAUTJ7D/node/1afda1af-bbde-48de-a5d7-5f43d389b2a6"
        )
        .unwrap()
        .into()
    );
    assert_eq!(opa.object, IRIList::IRI(IRI::new("http://field33.com/org/org_evlGiemVNyAUTJ7D/node/fe6fdda1-fc21-4b99-9269-8c19fc6359b8")
            .unwrap())
            );
    assert_eq!(
        opa.annotations,
        vec![Annotation::new(
            IRI::new("http://field33.com/ontologies/core_change_tracking/createdByImport")
                .unwrap()
                .into(),
            "GitHub".into(),
            vec![]
        ),]
    );
    assert_eq!(opa.resource_ids.len(), 1);
    assert!(opa.resource_ids[0].is_blank_node());
}

/// What is reified: AnnotationAssertion
/// Reification subject:
///   - Subject: IRI
///   - Object: Literal
/// Reification ID: IRI
/// Assertions stated on reification: AnnotationAssertion
#[test]
fn annotations_on_annotation_assertions() {
    env_logger::try_init().ok();
    let turtle = r##"
            <http://field33.com/query_result/00000000-0000-0000-0000-000000000000> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://www.w3.org/2002/07/owl#Ontology> .
            <http://field33.com/ontologies/@fld33_domain/dora_metrics/E528ddffc3bf541ffe030ffc413c26b2c7aafa5c> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://www.w3.org/2002/07/owl#NamedIndividual> .
            <http://www.w3.org/2000/01/rdf-schema#comment> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://www.w3.org/2002/07/owl#AnnotationProperty> .

            <http://field33.com/ontologies/core_change_tracking/createdByImport> <http://www.w3.org/2000/01/rdf-schema#label> "Created By"@en .
            <http://field33.com/ontologies/core_change_tracking/createdByImport> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://www.w3.org/2002/07/owl#AnnotationProperty> .
            <http://field33.com/ontologies/@fld33_domain/dora_metrics/E528ddffc3bf541ffe030ffc413c26b2c7aafa5cAnnotation3> <http://field33.com/ontologies/core_change_tracking/createdByImport> "GitHub" .
            <http://field33.com/ontologies/@fld33_domain/dora_metrics/E528ddffc3bf541ffe030ffc413c26b2c7aafa5cAnnotation3> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://www.w3.org/2002/07/owl#Axiom> .
            <http://field33.com/ontologies/@fld33_domain/dora_metrics/E528ddffc3bf541ffe030ffc413c26b2c7aafa5cAnnotation3> <http://www.w3.org/2002/07/owl#annotatedTarget> "foo bar" .
            <http://field33.com/ontologies/@fld33_domain/dora_metrics/E528ddffc3bf541ffe030ffc413c26b2c7aafa5cAnnotation3> <http://www.w3.org/2002/07/owl#annotatedProperty> <http://www.w3.org/2000/01/rdf-schema#comment> .
            <http://field33.com/ontologies/@fld33_domain/dora_metrics/E528ddffc3bf541ffe030ffc413c26b2c7aafa5cAnnotation3> <http://www.w3.org/2002/07/owl#annotatedSource> <http://field33.com/ontologies/@fld33_domain/dora_metrics/E528ddffc3bf541ffe030ffc413c26b2c7aafa5c> .
            <http://field33.com/ontologies/@fld33_domain/dora_metrics/E528ddffc3bf541ffe030ffc413c26b2c7aafa5c> <http://www.w3.org/2000/01/rdf-schema#comment> "foo bar" .

            <http://field33.com/ontologies/core_change_tracking/createdAt> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://www.w3.org/2002/07/owl#AnnotationProperty> .
            <http://field33.com/ontologies/@fld33_domain/dora_metrics/E528ddffc3bf541ffe030ffc413c26b2c7aafa5cAnnotation3> <http://field33.com/ontologies/core_change_tracking/createdAt> "2023-02-07T14:42:17Z"^^<http://www.w3.org/2001/XMLSchema#dateTime> .
        "##;

    harriet::TurtleDocument::parse_full(turtle).unwrap();
    let o = Ontology::parse(
        turtle,
        ParserOptionsBuilder::default()
            .known(Declaration::AnnotationProperty {
                iri: IRI::new("http://query-server.field33.com/ontology/query-field")
                    .unwrap()
                    .into(),
                annotations: vec![],
            })
            .build(),
    )
    .unwrap();
    println!("{:#?}", o);
    assert_eq!(o.declarations().len(), 4);
    assert_eq!(o.axioms().len(), 4);
    assert_eq!(
            o.axioms()[2],
            Axiom::AnnotationAssertion(AnnotationAssertion::new(
                IRI::new("http://www.w3.org/2000/01/rdf-schema#comment")
                    .unwrap()
                    .into(),
                IRI::new("http://field33.com/ontologies/@fld33_domain/dora_metrics/E528ddffc3bf541ffe030ffc413c26b2c7aafa5c")
                    .unwrap()
                    ,
                "foo bar".into(),
                vec![],
                    vec![ResourceId::IRI(IRI::new("http://field33.com/ontologies/@fld33_domain/dora_metrics/E528ddffc3bf541ffe030ffc413c26b2c7aafa5cAnnotation3").unwrap())]
            ))
        );

    let annotations_on_annotation = o.annotation_assertions_for_resource_id(&ResourceId::IRI(
            IRI::new("http://field33.com/ontologies/@fld33_domain/dora_metrics/E528ddffc3bf541ffe030ffc413c26b2c7aafa5cAnnotation3").unwrap(),
        ));
    assert_eq!(annotations_on_annotation.len(), 2);
    assert_eq!(
            annotations_on_annotation[0].clone(),
            AnnotationAssertion::new(
                IRI::new("http://field33.com/ontologies/core_change_tracking/createdByImport").unwrap().into(),
                IRI::new("http://field33.com/ontologies/@fld33_domain/dora_metrics/E528ddffc3bf541ffe030ffc413c26b2c7aafa5cAnnotation3").unwrap(),
                "GitHub".into(),
                vec![],
vec![]
            ),
        );
    assert_eq!(
            annotations_on_annotation[1].clone(),
            AnnotationAssertion::new(
                IRI::new("http://field33.com/ontologies/core_change_tracking/createdAt").unwrap().into(),
                IRI::new("http://field33.com/ontologies/@fld33_domain/dora_metrics/E528ddffc3bf541ffe030ffc413c26b2c7aafa5cAnnotation3").unwrap(),
                Literal::DateTime("2023-02-07T14:42:17Z".into()).into(),
                vec![],
                vec![]
            ),
        );
}
