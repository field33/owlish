use std::rc::Rc;

use harriet::triple_production::RdfTriple;
use log::debug;

use crate::{
    api::Ontology,
    error::Error,
    parser::matcher::{display, IRIOrBlank, MatchResult, RdfMatcher},
};

use self::matcher::{get_prefixes, MatcherState};

mod collector;
mod matcher;
use collector::*;

mod annotations;
mod axioms;
mod blank_nodes;
mod declarations;
mod sequences;

#[macro_export]
macro_rules! parser_debug {
    ($m:ident, $($tokens:tt)*) => {{
        if let Ok(name) = std::env::var("RDF_MATCHER") {
            if name == $m.name() {
                debug!($($tokens)*);
            }
        } else {
            debug!($($tokens)*);
        }
    }};
}

impl Ontology {
    pub fn parse(ttl: &str) -> Result<Self, Error> {
        let ttl =
            harriet::TurtleDocument::parse_full(ttl).map_err(|e| Error::new(format!("{:?}", e)))?;

        let triples: Vec<Rc<RdfTriple>> =
            harriet::triple_production::TripleProducer::produce_for_document(&ttl)
                .map_err(|e| Error::new(format!("Failed to emit triples: {}", e)))?
                .into_iter()
                .map(Rc::new)
                .collect();

        let mut collector = OntologyCollector::new();

        let prefixes = get_prefixes(ttl);

        let mut matchers: Vec<(RdfMatcher, MatcherHandler)> = Vec::new();

        declarations::match_declarations(&mut matchers, &prefixes)?;
        sequences::match_sequences(&mut matchers, &prefixes)?;
        blank_nodes::match_blank_nodes(&mut matchers, &prefixes)?;
        axioms::match_axioms(&mut matchers, &prefixes)?;
        annotations::match_annotations(&mut matchers, &prefixes)?;

        type MatcherID = usize;
        type TripleID = usize;

        // let mut finished_matches: Vec<(MatcherID, Vec<TripleID>, MatcherState)> = Vec::new();
        // subject node -> [matcher_id, matched_triples]
        let mut started_matches: Vec<(MatcherID, Vec<TripleID>, MatcherState, bool)> = Vec::new();

        let print_triples = if let Ok(a) = std::env::var("RDF_TRIPLES") {
            a == "1"
        } else {
            false
        };
        for (triple_id, triple) in triples.iter().enumerate() {
            if print_triples {
                println!("{}", display(triple));
            }
            for (matcher_id, (m, _)) in matchers.iter().enumerate() {
                let subject: IRIOrBlank = triple.subject.clone().into();

                // (1) Take each ongoing matcher state and check whether it matches this new triple
                for (matcher_id, triples, mstate, finished) in started_matches.iter_mut() {
                    let (m, _) = &matchers[*matcher_id];
                    parser_debug!(
                        m,
                        "         ################### Matching ({:?}, {})",
                        &subject,
                        m.name()
                    );

                    // (1) If so, keep matching. Maybe mark as finished.
                    if let MatchResult::Matched(f) = m.matches(triple.clone(), mstate) {
                        triples.push(triple_id);
                        *finished = f;
                    }
                }

                // (1) Anyways match with new state and add to started if it matches
                let mut mstate = MatcherState::new();
                if let MatchResult::Matched(finished) = m.matches(triple.clone(), &mut mstate) {
                    parser_debug!(m, "New matching state for ({:?}, {})", &subject, m.name());
                    started_matches.push((matcher_id, vec![triple_id], mstate, finished));
                }

                for (mid, _, mstate, finished) in &started_matches {
                    if *finished {
                        let (_m, handler) = &matchers[*mid];
                        if !handler(mstate, &mut collector)? {
                            // todo: did not meet semantic criteria
                        }
                    }
                }

                started_matches.retain_mut(|(_, _, _, f)| !*f);
            }
        }
        Ok(collector.ontology())
    }
}

impl From<Vec<String>> for Error {
    fn from(errors: Vec<String>) -> Self {
        Error::new(format!("{:#?}", errors))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        api::Ontology,
        owl::{
            well_known, Annotation, AnnotationAssertion, Axiom, ClassAssertion,
            DataPropertyAssertion, Declaration, Literal, LiteralOrIRI, ObjectIntersectionOf,
            SubClassOf, IRI,
        },
    };

    #[test]
    fn ontology() {
        env_logger::try_init().ok();
        let turtle = r##"
        @prefix : <http://test#> .
        @prefix owl: <http://www.w3.org/2002/07/owl#> .
        @prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
        @prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
        
        <http://test#> rdf:type owl:Ontology .
        
        "##;

        harriet::TurtleDocument::parse_full(turtle).unwrap();
        let o: Ontology = Ontology::parse(turtle).unwrap();
        assert_eq!(o.iri.to_string(), "http://test#");
    }

    #[test]
    fn class_declarations() {
        let turtle = r##"
        @prefix : <http://test#> .
        @prefix owl: <http://www.w3.org/2002/07/owl#> .
        @prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
        @prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
        <http://test#> rdf:type owl:Ontology .

        :Class1 rdf:type owl:Class .
        :Class2 rdf:type owl:Class .
        :Class3 rdf:type owl:Class .
        :Class4 rdf:type owl:Class .
        :Class5 rdf:type owl:Class .

        "##;

        harriet::TurtleDocument::parse_full(turtle).unwrap();
        let o: Ontology = Ontology::parse(turtle).unwrap();
        assert_eq!(o.declarations().len(), 5);
        assert_eq!(
            *o.declarations().get(0).unwrap(),
            Declaration::Class(IRI::new("http://test#Class1").unwrap().into(), vec![])
        );
        assert_eq!(
            *o.declarations().get(1).unwrap(),
            Declaration::Class(IRI::new("http://test#Class2").unwrap().into(), vec![])
        );
        assert_eq!(
            *o.declarations().get(2).unwrap(),
            Declaration::Class(IRI::new("http://test#Class3").unwrap().into(), vec![])
        );
        assert_eq!(
            *o.declarations().get(3).unwrap(),
            Declaration::Class(IRI::new("http://test#Class4").unwrap().into(), vec![])
        );
        assert_eq!(
            *o.declarations().get(4).unwrap(),
            Declaration::Class(IRI::new("http://test#Class5").unwrap().into(), vec![])
        );
    }

    #[test]
    fn datatype_declarations() {
        let turtle = r##"
        @prefix : <http://test#> .
        @prefix owl: <http://www.w3.org/2002/07/owl#> .
        @prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
        @prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
        <http://test#> rdf:type owl:Ontology .

        :test1 rdf:type rdfs:Datatype .
        :test2 rdf:type rdfs:Datatype .
        :test3 rdf:type rdfs:Datatype .
        :test4 rdf:type rdfs:Datatype .
        :test5 rdf:type rdfs:Datatype .

        "##;

        harriet::TurtleDocument::parse_full(turtle).unwrap();
        let o = Ontology::parse(turtle).unwrap();
        assert_eq!(o.declarations().len(), 5);
    }

    #[test]
    fn object_property_declarations() {
        let turtle = r##"
        @prefix : <http://test#> .
        @prefix owl: <http://www.w3.org/2002/07/owl#> .
        @prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
        @prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
        <http://test#> rdf:type owl:Ontology .

        :test1 rdf:type owl:ObjectProperty .
        :test2 rdf:type owl:ObjectProperty .
        :test3 rdf:type owl:ObjectProperty .
        :test4 rdf:type owl:ObjectProperty .
        :test5 rdf:type owl:ObjectProperty .

        "##;

        harriet::TurtleDocument::parse_full(turtle).unwrap();
        let o = Ontology::parse(turtle).unwrap();
        assert_eq!(o.declarations().len(), 5);
    }

    #[test]
    fn two_different_matchers() {
        let turtle = r##"
        @prefix : <http://test#> .
        @prefix owl: <http://www.w3.org/2002/07/owl#> .
        @prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
        @prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
        <http://test#> rdf:type owl:Ontology .

        :test1 rdf:type rdfs:Datatype .
        :test1 rdf:type owl:ObjectProperty .
        :test2 rdf:type rdfs:Datatype .
        :test2 rdf:type owl:ObjectProperty .
        :test3 rdf:type rdfs:Datatype .
        :test3 rdf:type owl:ObjectProperty .
        :test4 rdf:type rdfs:Datatype .
        :test4 rdf:type owl:ObjectProperty .
        :test5 rdf:type rdfs:Datatype .
        :test5 rdf:type owl:ObjectProperty .

        "##;

        harriet::TurtleDocument::parse_full(turtle).unwrap();
        let o = Ontology::parse(turtle).unwrap();
        assert_eq!(o.declarations().len(), 10);
        assert_eq!(
            o.declarations().iter().fold(0, |acc, x| {
                acc + match x {
                    Declaration::ObjectProperty(_, _) => 1,
                    _ => 0,
                }
            }),
            5
        );
        assert_eq!(
            o.declarations().iter().fold(0, |acc, x| {
                acc + match x {
                    Declaration::Datatype(_, _) => 1,
                    _ => 0,
                }
            }),
            5
        );
    }

    #[test]
    fn datatype_property_declarations() {
        let turtle = r##"
        @prefix : <http://test#> .
        @prefix owl: <http://www.w3.org/2002/07/owl#> .
        @prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
        @prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
        <http://test#> rdf:type owl:Ontology .

        :test1 rdf:type owl:DatatypeProperty .
        :test2 rdf:type owl:DatatypeProperty .
        :test3 rdf:type owl:DatatypeProperty .
        :test4 rdf:type owl:DatatypeProperty .
        :test5 rdf:type owl:DatatypeProperty .

        "##;

        harriet::TurtleDocument::parse_full(turtle).unwrap();
        let o = Ontology::parse(turtle).unwrap();
        assert_eq!(o.declarations().len(), 5);
    }

    #[test]
    fn annotation_assertions() {
        env_logger::try_init().ok();
        let turtle = r##"
        @prefix : <http://test#> .
        @prefix owl: <http://www.w3.org/2002/07/owl#> .
        @prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
        @prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
        @prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

        <http://test#> rdf:type owl:Ontology .
        :Person rdf:type owl:Class .
        :Person rdfs:comment "Represents the set of all people."^^xsd:string .

        "##;

        harriet::TurtleDocument::parse_full(turtle).unwrap();
        let o = Ontology::parse(turtle).unwrap();
        assert_eq!(o.declarations().len(), 1);
        assert_eq!(o.axioms().len(), 1);
        assert_eq!(
            o.axioms()[0],
            AnnotationAssertion(
                well_known::rdfs_comment(),
                IRI::new("http://test#Person").unwrap(),
                LiteralOrIRI::Literal(Literal::String("Represents the set of all people.".into())),
                vec![]
            )
            .into()
        );
    }

    #[test]
    fn intersection_of() {
        env_logger::try_init().ok();
        let turtle = r##"
        @prefix : <http://test#> .
        @prefix owl: <http://www.w3.org/2002/07/owl#> .
        @prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
        @prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
        @prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

        <http://test#> rdf:type owl:Ontology .

        :Man rdf:type owl:Class .
        :Parent rdf:type owl:Class .
        :Grandfather rdf:type owl:Class .

        :Grandfather  rdfs:subClassOf  [
            rdf:type            owl:Class ;
            owl:intersectionOf  ( :Man  :Parent )
        ] .

        "##;

        harriet::TurtleDocument::parse_full(turtle).unwrap();
        let o = Ontology::parse(turtle).unwrap();

        assert_eq!(o.declarations().len(), 3);
        assert_eq!(o.axioms().len(), 1);
        assert_eq!(
            o.axioms()[0],
            Axiom::SubClassOf(SubClassOf(
                IRI::new("http://test#Grandfather").unwrap().into(),
                ObjectIntersectionOf(
                    vec![
                        IRI::new("http://test#Man").unwrap().into(),
                        IRI::new("http://test#Parent").unwrap().into(),
                    ],
                    vec![]
                )
                .into(),
                vec![]
            ))
        );
    }

    #[test]
    fn sub_class_of() {
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

        :Man rdfs:subClassOf :Person .
        "##;

        harriet::TurtleDocument::parse_full(turtle).unwrap();
        let o = Ontology::parse(turtle).unwrap();

        assert_eq!(o.declarations().len(), 2);
        assert_eq!(o.axioms().len(), 1);
        assert_eq!(
            o.axioms()[0],
            Axiom::SubClassOf(SubClassOf(
                IRI::new("http://test#Man").unwrap().into(),
                IRI::new("http://test#Person").unwrap().into(),
                vec![]
            ))
        );
    }

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
        let o = Ontology::parse(turtle).unwrap();

        assert_eq!(o.declarations().len(), 2);
        assert_eq!(o.axioms().len(), 1);
        assert_eq!(
            o.axioms()[0],
            Axiom::SubClassOf(SubClassOf(
                IRI::new("http://test#Man").unwrap().into(),
                IRI::new("http://test#Person").unwrap().into(),
                vec![Annotation(
                    well_known::rdfs_comment(),
                    LiteralOrIRI::Literal(Literal::String(
                        "States that every man is a person.".into()
                    )),
                    vec![]
                )]
            ))
        );
    }

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

        :Man rdfs:comment "test" .
        []   rdf:type               owl:Axiom ;
             owl:annotatedSource    :Man ;
             owl:annotatedProperty  rdfs:comment ;
             owl:annotatedTarget    "test" ;
             rdfs:comment           "States that every man is a person."^^xsd:string .

        "##;

        harriet::TurtleDocument::parse_full(turtle).unwrap();
        let o = Ontology::parse(turtle).unwrap();

        assert_eq!(o.declarations().len(), 1);
        assert_eq!(o.axioms().len(), 1);
        assert_eq!(
            o.axioms()[0],
            Axiom::AnnotationAssertion(AnnotationAssertion(
                IRI::new("http://www.w3.org/2000/01/rdf-schema#comment")
                    .unwrap()
                    .into(),
                IRI::new("http://test#Man").unwrap(),
                Literal::String("test".into()).into(),
                vec![Annotation(
                    well_known::rdfs_comment(),
                    LiteralOrIRI::Literal(Literal::String(
                        "States that every man is a person.".into()
                    )),
                    vec![]
                )]
            ))
        );
    }

    #[test]
    fn class_assertion() {
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

        :Bob rdf:type :Man .
        "##;

        harriet::TurtleDocument::parse_full(turtle).unwrap();
        let o = Ontology::parse(turtle).unwrap();

        assert_eq!(o.declarations().len(), 2);
        assert_eq!(o.axioms().len(), 1);
        assert_eq!(
            o.axioms()[0],
            ClassAssertion(
                IRI::new("http://test#Bob").unwrap().into(),
                IRI::new("http://test#Man").unwrap().into(),
                vec![]
            )
            .into()
        );
    }

    #[test]
    fn data_properties() {
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
        :hasAge rdf:type owl:DatatypeProperty .

        :Bob rdf:type :Man .
        :Bob :hasAge "51" .
        "##;

        harriet::TurtleDocument::parse_full(turtle).unwrap();
        let o = Ontology::parse(turtle).unwrap();

        assert_eq!(o.declarations().len(), 3);
        assert_eq!(o.axioms().len(), 2);
        assert_eq!(
            o.axioms()[1],
            DataPropertyAssertion(
                IRI::new("http://test#hasAge").unwrap().into(),
                IRI::new("http://test#Bob").unwrap().into(),
                Literal::String("51".into()),
                vec![]
            )
            .into()
        );
    }

    // #[test]
    // fn computations_with_blank() {
    //     env_logger::try_init().ok();
    //     let turtle = r#"
    //     @prefix onto1: <http://example.com/ONTO1/> .
    //     @prefix query_server: <http://query-server.field33.com/ontology/> .
    //     @prefix owl: <http://www.w3.org/2002/07/owl#> .
    //     @prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
    //     @prefix xml: <http://www.w3.org/XML/1998/namespace> .
    //     @prefix xsd: <http://www.w3.org/2001/XMLSchema#> .
    //     @prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .

    //     <http://query-server.field33.com/query/aaaa-bbbb-ccc-dddd> rdf:type owl:Ontology .

    //     onto1:Individual1 rdf:type owl:NamedIndividual .
    //     onto1:Individual1 rdfs:label "Person 1" .

    //     []   rdf:type                  owl:Axiom ;
    //          owl:annotatedSource       onto1:Individual1 ;
    //          owl:annotatedProperty     rdfs:label ;
    //          owl:annotatedTarget       "Person 1" ;
    //          query_server:query-field  "my_label"^^xsd:string .
    //     "#;

    //     harriet::TurtleDocument::parse_full(turtle).unwrap();
    //     let o = Ontology::parse(turtle).unwrap();

    //     println!("{:#?}", o);

    //     assert_eq!(
    //         o.iri.as_str(),
    //         "http://query-server.field33.com/query/aaaa-bbbb-ccc-dddd"
    //     );

    //     assert_eq!(o.declarations().len(), 1);
    //     assert_eq!(
    //         o.declarations()[0],
    //         Declaration::NamedIndividual(
    //             IRI::new("http://example.com/ONTO1/Individual1")
    //                 .unwrap()
    //                 .into(),
    //             vec![]
    //         )
    //     );

    //     assert_eq!(o.axioms().len(), 1);
    //     assert_eq!(
    //         o.axioms()[0],
    //         AnnotationAssertion(
    //             well_known::rdfs_label(),
    //             IRI::new("http://example.com/ONTO1/Individual1")
    //                 .unwrap(),
    //             Literal::String("Person 1".into()).into(),
    //             vec![Annotation(
    //                 IRI::new("http://query-server.field33.com/ontology/query-field")
    //                     .unwrap()
    //                     .into(),
    //                 Literal::String("my_label".into()).into(),
    //                 vec![]
    //             )]
    //         )
    //         .into()
    //     );
    // }

    // #[test]
    // fn computations() {
    //     let turtle = r#"
    //     @prefix onto1: <http://example.com/ONTO1/> .
    //     @prefix query_server: <http://query-server.field33.com/ontology/> .
    //     @prefix owl: <http://www.w3.org/2002/07/owl#> .
    //     @prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
    //     @prefix xml: <http://www.w3.org/XML/1998/namespace> .
    //     @prefix xsd: <http://www.w3.org/2001/XMLSchema#> .
    //     @prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .

    //     <http://query-server.field33.com/query/aaaa-bbbb-ccc-dddd> rdf:type owl:Ontology .

    //     onto1:Individual1 rdf:type owl:NamedIndividual .
    //     onto1:Individual1 rdfs:label "Person 1" .
    //     onto1:Individual1LabelAnn1 rdf:type owl:Annotation .
    //     onto1:Individual1LabelAnn1 owl:annotatedSource onto1:Individual1 .
    //     onto1:Individual1LabelAnn1 owl:annotatedProperty rdfs:label .
    //     onto1:Individual1LabelAnn1 owl:annotatedTarget "Person 1" .
    //     onto1:Individual1LabelAnn1 query_server:query-field "my_label" .
    //     "#;

    //     harriet::TurtleDocument::parse_full(turtle).unwrap();
    //     let o = Ontology::parse(turtle).unwrap();

    //     assert_eq!(
    //         o.iri.as_str(),
    //         "http://query-server.field33.com/query/aaaa-bbbb-ccc-dddd"
    //     );

    //     assert_eq!(o.declarations().len(), 1);
    //     assert_eq!(
    //         o.declarations()[0],
    //         Declaration::NamedIndividual(
    //             IRI::new("http://example.com/ONTO1/Individual1")
    //                 .unwrap()
    //                 .into(),
    //             vec![]
    //         )
    //     );

    //     assert_eq!(o.axioms().len(), 1);
    //     assert_eq!(
    //         o.axioms()[0],
    //         AnnotationAssertion(
    //             well_known::rdfs_label(),
    //             IRI::new("http://example.com/ONTO1/Individual1")
    //                 .unwrap()
    //                 .into(),
    //             Literal::String("Person 1".into()).into(),
    //             vec![]
    //         )
    //         .into()
    //     );
    // }
}
