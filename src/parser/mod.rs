use std::{collections::HashMap, rc::Rc};

use harriet::triple_production::RdfTriple;

use log::debug;
use serde::{Deserialize, Serialize};

use crate::{
    api::Ontology,
    error::Error,
    owl::{well_known, Declaration, IRI},
    parser::matcher::{display, print, IRIOrBlank, MatchResult, RdfMatcher},
};

use self::matcher::{get_prefixes, MatcherState};

mod collector;
mod matcher;
use collector::*;

mod annotations;
mod axioms;
mod blank_nodes;
mod data_props;
mod declarations;
mod object_property_assertions;
mod sequences;
pub mod triple;

static mut RDF_MATCHER: Option<Option<String>> = None;

#[macro_export]
macro_rules! parser_debug {
    ($m:ident, $($tokens:tt)*) => {{
        unsafe {
            if $crate::parser::RDF_MATCHER.is_none() {
                $crate::parser::RDF_MATCHER = Some(std::env::var("RDF_MATCHER").ok());
            }
            if let Some(Some(names)) = &$crate::parser::RDF_MATCHER {
                if names.split(",").find(|n| n.trim() == $m.name()).is_some() {
                    log::debug!($($tokens)*);
                }
            } else {
                log::debug!($($tokens)*);
            }
        }
    }};
}

impl Ontology {
    pub fn parse(ttl: &str, options: ParserOptions) -> Result<Self, Error> {
        let indexed_options: IndexedParserOptions = options.into();
        let ttl =
            harriet::TurtleDocument::parse_full(ttl).map_err(|e| Error::new(format!("{:?}", e)))?;

        let triples: Vec<Rc<RdfTriple>> =
            harriet::triple_production::TripleProducer::produce_for_document(&ttl)
                .map_err(|e| Error::new(format!("Failed to emit triples: {}", e)))?
                .into_iter()
                .map(Rc::new)
                .collect();

        let mut collector = OntologyCollector::new();

        let mut prefixes = get_prefixes(ttl);
        // handle non-existing well known prefixes
        if !prefixes.contains_key("rdf") {
            prefixes.insert("rdf".into(), well_known::rdf_base_str.into());
        }
        if !prefixes.contains_key("rdfs") {
            prefixes.insert("rdfs".into(), well_known::rdfs_base_str.into());
        }
        if !prefixes.contains_key("xsd") {
            prefixes.insert("xsd".into(), well_known::xsd_base_str.into());
        }
        if !prefixes.contains_key("owl") {
            prefixes.insert("owl".into(), well_known::owl_base_str.into());
        }

        let mut matchers: Vec<(RdfMatcher, MatcherHandler)> = Vec::new();

        type MatcherID = usize;
        type MatcherStateEntry<'a> = (MatcherID, Vec<Rc<RdfTriple<'a>>>, MatcherState<'a>, bool);
        // let mut finished_matches: Vec<(MatcherID, Vec<TripleID>, MatcherState)> = Vec::new();
        // subject node -> [matcher_id, matched_triples]
        let mut matcher_instances: HashMap<usize, MatcherStateEntry> = HashMap::new();
        // let mut matcher_instance_id = 0;

        for phase in 0..4 {
            matchers.clear();
            matcher_instances.clear();
            match phase {
                0 => {
                    declarations::match_declarations(&mut matchers, &prefixes)?;
                    sequences::match_sequences(&mut matchers, &prefixes)?;
                }
                1 => {
                    blank_nodes::match_blank_nodes(&mut matchers, &prefixes)?;
                    annotations::match_simple_annotation_assertions(&mut matchers, &prefixes)?;
                    data_props::match_simple_dataprop_assertions(&mut matchers, &prefixes)?;
                }
                2 => {
                    annotations::match_annotations(&mut matchers, &prefixes)?;
                }
                _ => {
                    axioms::match_axioms(&mut matchers, &prefixes)?;
                    object_property_assertions::push(&mut matchers, &prefixes)?;
                    annotations::match_annotation_assertions(&mut matchers, &prefixes)?;
                    data_props::match_dataprop_assertions(&mut matchers, &prefixes)?;
                }
            }

            for triple in triples.iter() {
                let subject: IRIOrBlank = triple.subject.clone().into();

                debug!("===============");
                debug!("{:?}", triple);
                debug!("===============");

                for (matcher_id, (m, _)) in matchers.iter().enumerate() {
                    let mut mstate = MatcherState::new();
                    parser_debug!(m, "################ {} #######################", m.name());
                    parser_debug!(m, "{}", display(triple));

                    // (1) Start matcher with new state (if there is no current matcher state)
                    if let MatchResult::Matched(finished) = m.matches(triple.clone(), &mut mstate) {
                        parser_debug!(m, "Matched for empty state: ({:?}, {})", &subject, m.name(),);

                        // (2) If matching already finished -> call handler and continue
                        if finished {
                            let (_m, handler) = &matchers[matcher_id];
                            if !handler(&mut mstate, &mut collector, &indexed_options)? {
                                // todo: did not meet semantic criteria
                            }
                        } else {
                            parser_debug!(m, "Check for ongoing matchers",);
                            // (3) Check if there is an existing matcher instance
                            match matcher_instances.get_mut(&matcher_id) {
                                Some((_, _, ongoing_mstate, _)) => {
                                    parser_debug!(m, "{}", print(m, &mstate));
                                    let (matcher, handler) = &matchers[matcher_id];
                                    // (4) If that does match as well -> it's state is now extended (by the new match)
                                    match matcher.matches(triple.clone(), ongoing_mstate) {
                                        MatchResult::Matched(finished) => {
                                            parser_debug!(
                                                m,
                                                "Matched ongoing matcher {:?} {} {} {:?}",
                                                triple,
                                                matcher_id,
                                                matcher.name(),
                                                ongoing_mstate.vars()
                                            );
                                            if finished {
                                                if !handler(
                                                    ongoing_mstate,
                                                    &mut collector,
                                                    &indexed_options,
                                                )? {
                                                    // todo: did not meet semantic criteria
                                                }
                                                matcher_instances.remove(&matcher_id);
                                            } else {
                                                // TODO?
                                            }
                                        }
                                        // (5) If there is an ongoing matcher of this kind
                                        //     but it did not match at all it means, it started to match
                                        //     on smth different, than what we found now:
                                        //     (e.g.) We started a UnionOf on one blank node but now got another blank node which maches as well.
                                        //     -> In such cases restart the matcher. For performance reasons only one instance of matcher runs
                                        //     at a time which means meaningful constructs of multiple triples can not bleed into each other.
                                        MatchResult::Nope => {
                                            parser_debug!(
                                                m,
                                                "No match for ongoing matcher: Replacing state...",
                                            );
                                            matcher_instances.insert(
                                                matcher_id,
                                                (matcher_id, vec![], mstate, finished),
                                            );
                                        }
                                    }
                                }
                                None => {
                                    // (6) If there is no ongoing matcher, save a new one
                                    parser_debug!(m, "No ongoing matchers",);
                                    matcher_instances
                                        .insert(matcher_id, (matcher_id, vec![], mstate, finished));
                                }
                            }
                        }
                    }
                }
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ParserOptions {
    known: Vec<Declaration>,
}

impl ParserOptions {
    pub fn builder() -> ParserOptionsBuilder {
        ParserOptionsBuilder {
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IndexedParserOptions {
    known: Vec<Declaration>,
    index: HashMap<IRI, usize>,
}
impl IndexedParserOptions {
    pub fn is_annotation_prop(&self, iri: &IRI) -> bool {
        if let Some(i) = self.index.get(iri) {
            matches!(
                self.known.get(*i),
                Some(Declaration::AnnotationProperty { .. })
            )
        } else {
            false
        }
    }

    pub fn is_data_prop(&self, iri: &IRI) -> bool {
        if let Some(i) = self.index.get(iri) {
            matches!(self.known.get(*i), Some(Declaration::DataProperty { .. }))
        } else {
            false
        }
    }

    fn is_object_prop(&self, iri: &IRI) -> bool {
        if let Some(i) = self.index.get(iri) {
            matches!(self.known.get(*i), Some(Declaration::ObjectProperty { .. }))
        } else {
            false
        }
    }

    pub fn is_class(&self, iri: &IRI) -> bool {
        if let Some(i) = self.index.get(iri) {
            matches!(self.known.get(*i), Some(Declaration::Class { .. }))
        } else {
            false
        }
    }
}

impl From<ParserOptions> for IndexedParserOptions {
    fn from(po: ParserOptions) -> Self {
        let mut index = HashMap::new();
        for (i, d) in po.known.iter().enumerate() {
            let iri = match d {
                Declaration::Class {
                    iri,
                    annotations: _,
                } => iri.as_iri(),
                Declaration::NamedIndividual {
                    iri,
                    annotations: _,
                } => iri.as_iri(),
                Declaration::ObjectProperty {
                    iri,
                    annotations: _,
                } => iri.as_iri(),
                Declaration::DataProperty {
                    iri,
                    annotations: _,
                } => iri.as_iri(),
                Declaration::AnnotationProperty {
                    iri,
                    annotations: _,
                } => iri.as_iri(),
                Declaration::Datatype {
                    iri,
                    annotations: _,
                } => iri.as_iri(),
            };
            index.insert(iri.clone(), i);
        }
        Self {
            known: po.known,
            index,
        }
    }
}

#[derive(Debug, Default)]
pub struct ParserOptionsBuilder {
    options: ParserOptions,
}

impl ParserOptionsBuilder {
    pub fn known(mut self, declaration: Declaration) -> Self {
        self.options.known.push(declaration);
        self
    }
    pub fn build(self) -> ParserOptions {
        self.options
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Number;

    use crate::{
        api::Ontology,
        owl::{
            well_known, Annotation, AnnotationAssertion, Axiom, ClassAssertion,
            DataPropertyAssertion, DataPropertyDomain, DataPropertyRange, Declaration,
            EquivalentClasses, Literal, LiteralOrIRI, ObjectIntersectionOf,
            ObjectPropertyAssertion, ObjectPropertyDomain, ObjectPropertyRange, ObjectUnionOf,
            SubAnnotationPropertyOf, SubClassOf, SubDataPropertyOf, SubObjectPropertyOf, IRI,
        },
        parser::{ParserOptions, ParserOptionsBuilder},
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
        let o: Ontology = Ontology::parse(turtle, Default::default()).unwrap();
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
        let o: Ontology = Ontology::parse(turtle, Default::default()).unwrap();
        assert_eq!(o.declarations().len(), 5);
        assert_eq!(
            *o.declarations().get(0).unwrap(),
            Declaration::Class {
                iri: IRI::new("http://test#Class1").unwrap().into(),
                annotations: vec![]
            }
        );
        assert_eq!(
            *o.declarations().get(1).unwrap(),
            Declaration::Class {
                iri: IRI::new("http://test#Class2").unwrap().into(),
                annotations: vec![]
            }
        );
        assert_eq!(
            *o.declarations().get(2).unwrap(),
            Declaration::Class {
                iri: IRI::new("http://test#Class3").unwrap().into(),
                annotations: vec![]
            }
        );
        assert_eq!(
            *o.declarations().get(3).unwrap(),
            Declaration::Class {
                iri: IRI::new("http://test#Class4").unwrap().into(),
                annotations: vec![]
            }
        );
        assert_eq!(
            *o.declarations().get(4).unwrap(),
            Declaration::Class {
                iri: IRI::new("http://test#Class5").unwrap().into(),
                annotations: vec![]
            }
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
        let o = Ontology::parse(turtle, Default::default()).unwrap();
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
        let o = Ontology::parse(turtle, Default::default()).unwrap();
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
        let o = Ontology::parse(turtle, Default::default()).unwrap();
        assert_eq!(o.declarations().len(), 10);
        assert_eq!(
            o.declarations().iter().fold(0, |acc, x| {
                acc + match x {
                    Declaration::ObjectProperty { .. } => 1,
                    _ => 0,
                }
            }),
            5
        );
        assert_eq!(
            o.declarations().iter().fold(0, |acc, x| {
                acc + match x {
                    Declaration::Datatype { .. } => 1,
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
        let o = Ontology::parse(turtle, Default::default()).unwrap();
        assert_eq!(o.declarations().len(), 5);
    }

    #[test]
    fn domain_and_range() {
        let turtle = r##"
        @prefix : <http://test#> .
        @prefix owl: <http://www.w3.org/2002/07/owl#> .
        @prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
        @prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
        <http://test#> rdf:type owl:Ontology .

        
        :hasWife rdf:type owl:ObjectProperty ;
                 rdfs:domain :Man ;
                 rdfs:range  :Woman .

        "##;

        harriet::TurtleDocument::parse_full(turtle).unwrap();
        let o = Ontology::parse(turtle, Default::default()).unwrap();
        assert_eq!(o.declarations().len(), 1);
        assert_eq!(o.axioms().len(), 2);

        assert_eq!(
            o.axioms()[0],
            ObjectPropertyDomain::new(
                IRI::new("http://test#hasWife").unwrap().into(),
                IRI::new("http://test#Man").unwrap().into(),
                vec![]
            )
            .into()
        );
        assert_eq!(
            o.axioms()[1],
            ObjectPropertyRange::new(
                IRI::new("http://test#hasWife").unwrap().into(),
                IRI::new("http://test#Woman").unwrap().into(),
                vec![]
            )
            .into()
        );
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
        :Person rdfs:comment "foo" .
        :Person rdfs:comment <http://test.org#Thing> .
        :Person rdfs:comment 42 .
        :Person rdfs:comment "3.14"^^xsd:float .
        :Person rdfs:comment "true"^^xsd:boolean .
        :Person rdfs:comment false .
        :Person rdfs:comment "2023-01-31T08:39:54"^^xsd:dateTime .

        "##;

        harriet::TurtleDocument::parse_full(turtle).unwrap();
        let o = Ontology::parse(turtle, Default::default()).unwrap();
        assert_eq!(o.declarations().len(), 1);
        assert_eq!(o.axioms().len(), 8);
        assert_eq!(
            o.axioms()[0],
            AnnotationAssertion::new(
                well_known::rdfs_comment(),
                IRI::new("http://test#Person").unwrap(),
                LiteralOrIRI::Literal(Literal::String("Represents the set of all people.".into())),
                vec![]
            )
            .into()
        );
        assert_eq!(
            o.axioms()[1],
            AnnotationAssertion::new(
                well_known::rdfs_comment(),
                IRI::new("http://test#Person").unwrap(),
                LiteralOrIRI::Literal(Literal::String("foo".into())),
                vec![]
            )
            .into()
        );
        assert_eq!(
            o.axioms()[2],
            AnnotationAssertion::new(
                well_known::rdfs_comment(),
                IRI::new("http://test#Person").unwrap(),
                LiteralOrIRI::IRI(IRI::new("http://test.org#Thing").unwrap()),
                vec![]
            )
            .into()
        );
        assert_eq!(
            o.axioms()[3],
            AnnotationAssertion::new(
                well_known::rdfs_comment(),
                IRI::new("http://test#Person").unwrap(),
                LiteralOrIRI::Literal(Literal::Number {
                    number: 42.into(),
                    type_iri: Some(well_known::xsd_integer())
                }),
                vec![]
            )
            .into()
        );
        assert_eq!(
            o.axioms()[4],
            AnnotationAssertion::new(
                well_known::rdfs_comment(),
                IRI::new("http://test#Person").unwrap(),
                LiteralOrIRI::Literal(Literal::Number {
                    number: Number::from_f64(3.14).unwrap(),
                    type_iri: Some(well_known::xsd_float())
                }),
                vec![]
            )
            .into()
        );
        assert_eq!(
            o.axioms()[5],
            AnnotationAssertion::new(
                well_known::rdfs_comment(),
                IRI::new("http://test#Person").unwrap(),
                LiteralOrIRI::Literal(Literal::Bool(true)),
                vec![]
            )
            .into()
        );
        assert_eq!(
            o.axioms()[6],
            AnnotationAssertion::new(
                well_known::rdfs_comment(),
                IRI::new("http://test#Person").unwrap(),
                LiteralOrIRI::Literal(Literal::Bool(false)),
                vec![]
            )
            .into()
        );
        assert_eq!(
            o.axioms()[7],
            AnnotationAssertion::new(
                well_known::rdfs_comment(),
                IRI::new("http://test#Person").unwrap(),
                LiteralOrIRI::Literal(Literal::DateTime("2023-01-31T08:39:54".into())),
                vec![]
            )
            .into()
        );
    }

    #[test]
    fn object_property_assertions() {
        env_logger::try_init().ok();
        let turtle = r##"
        @prefix : <http://test#> .
        @prefix owl: <http://www.w3.org/2002/07/owl#> .
        @prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
        @prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
        @prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

        <http://test#> rdf:type owl:Ontology .
        :Person rdf:type owl:Class .
        :Schmerson rdf:type owl:Class .
        :Person :foo :Schmerson .
        "##;

        harriet::TurtleDocument::parse_full(turtle).unwrap();
        let o = Ontology::parse(
            turtle,
            ParserOptions::builder()
                .known(Declaration::ObjectProperty {
                    iri: IRI::new("http://test#foo").unwrap().into(),
                    annotations: vec![],
                })
                .build(),
        )
        .unwrap();
        assert_eq!(o.declarations().len(), 2);
        assert_eq!(o.axioms().len(), 1);
        assert_eq!(
            o.axioms()[0],
            ObjectPropertyAssertion::new(
                IRI::new("http://test#foo").unwrap().into(),
                IRI::new("http://test#Person").unwrap().into(),
                IRI::new("http://test#Schmerson").unwrap().into(),
                vec![]
            )
            .into()
        );
    }

    #[test]
    fn object_property_range_domain() {
        env_logger::try_init().ok();
        let turtle = r##"
        @prefix : <http://test#> .
        @prefix owl: <http://www.w3.org/2002/07/owl#> .
        @prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
        @prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
        @prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

        <http://test#> rdf:type owl:Ontology .
        :A rdf:type owl:Class .
        :B rdf:type owl:Class .
        :C rdf:type owl:Class .
        :D rdf:type owl:Class .
        

        :O1 rdf:type owl:ObjectProperty .
        :O2 rdf:type owl:ObjectProperty .
        :O3 rdf:type owl:ObjectProperty .

        :O1 rdfs:domain :A .
        :O1 rdfs:range :B .

        :O3 rdfs:domain [ rdf:type owl:Class ;
                         owl:unionOf ( :A :B )
                       ] ;
            rdfs:range [ rdf:type owl:Class ;
                         owl:unionOf ( :C :D )
                       ] .
        "##;

        harriet::TurtleDocument::parse_full(turtle).unwrap();
        let o = Ontology::parse(
            turtle,
            ParserOptions::builder()
                .known(Declaration::ObjectProperty {
                    iri: IRI::new("http://test#foo").unwrap().into(),
                    annotations: vec![],
                })
                .build(),
        )
        .unwrap();
        println!("{:#?}", o);
        assert_eq!(o.declarations().len(), 7);
        assert_eq!(o.axioms().len(), 4);
        assert_eq!(
            o.axioms()[0],
            ObjectPropertyDomain::new(
                IRI::new("http://test#O1").unwrap().into(),
                IRI::new("http://test#A").unwrap().into(),
                vec![]
            )
            .into()
        );
        assert_eq!(
            o.axioms()[1],
            ObjectPropertyRange::new(
                IRI::new("http://test#O1").unwrap().into(),
                IRI::new("http://test#B").unwrap().into(),
                vec![]
            )
            .into()
        );
        assert_eq!(
            o.axioms()[2],
            ObjectPropertyDomain::new(
                IRI::new("http://test#O3").unwrap().into(),
                ObjectUnionOf::new(
                    vec![
                        IRI::new("http://test#A").unwrap().into(),
                        IRI::new("http://test#B").unwrap().into(),
                    ],
                    vec![]
                )
                .into(),
                vec![]
            )
            .into()
        );
        assert_eq!(
            o.axioms()[3],
            ObjectPropertyRange::new(
                IRI::new("http://test#O3").unwrap().into(),
                ObjectUnionOf::new(
                    vec![
                        IRI::new("http://test#C").unwrap().into(),
                        IRI::new("http://test#D").unwrap().into(),
                    ],
                    vec![]
                )
                .into(),
                vec![]
            )
            .into()
        );
    }

    #[test]
    fn object_property_range_domain_unsorted() {
        env_logger::try_init().ok();
        let turtle = r##"
        <http://field33.com/query_result/05b62f05-fc55-4f59-8e47-7563e0cc6ba5> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://www.w3.org/2002/07/owl#Ontology> .
             <http://field33.com/ontologies/@schmolo/ppr/CFAFBD> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://www.w3.org/2002/07/owl#ObjectProperty> .
             _:1aed1bc93811d23de40180a8654db2f6 <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://www.w3.org/2002/07/owl#Class> .
            _:84a5309bc04f5c90361bb9101e1ecd8b <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://www.w3.org/2002/07/owl#Class> .
            _:ca04f7914fb1f1bf6d53ebf2477f4e4e <http://www.w3.org/1999/02/22-rdf-syntax-ns#rest> _:a9dcc8a907b1c076413bd5b8f25392d7 .
            _:a9dcc8a907b1c076413bd5b8f25392d7 <http://www.w3.org/1999/02/22-rdf-syntax-ns#first> <http://field33.com/ontologies/@schmolo/ppr/RQ> .
            _:ca04f7914fb1f1bf6d53ebf2477f4e4e <http://www.w3.org/1999/02/22-rdf-syntax-ns#first> <http://field33.com/ontologies/@schmolo/ppr/CASL> .
            _:84a5309bc04f5c90361bb9101e1ecd8b <http://www.w3.org/2002/07/owl#unionOf> _:ca04f7914fb1f1bf6d53ebf2477f4e4e .
            _:84a5309bc04f5c90361bb9101e1ecd8b <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://www.w3.org/2002/07/owl#Class> .
            <http://field33.com/ontologies/@schmolo/ppr/CFAFBD> <http://www.w3.org/2000/01/rdf-schema#range> _:84a5309bc04f5c90361bb9101e1ecd8b .
            <http://field33.com/ontologies/@schmolo/ppr/CFAFBD> <http://www.w3.org/2000/01/rdf-schema#domain> <http://field33.com/ontologies/@schmolo/ppr/CFAR> .
            <http://field33.com/ontologies/@schmolo/ppr/CFAFBD> <http://www.w3.org/2000/01/rdf-schema#label> "Followed By"@en .
            <http://field33.com/ontologies/@schmolo/ppr/CFAFBD> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://www.w3.org/2002/07/owl#ObjectProperty> .
        "##;

        harriet::TurtleDocument::parse_full(turtle).unwrap();
        let o = Ontology::parse(
            turtle,
            ParserOptions::builder()
                .known(Declaration::ObjectProperty {
                    iri: IRI::new("http://test#foo").unwrap().into(),
                    annotations: vec![],
                })
                .build(),
        )
        .unwrap();
        println!("{:#?}", o);
        assert_eq!(o.declarations().len(), 2);
        assert_eq!(o.axioms().len(), 3);
        assert_eq!(
            o.axioms()[1],
            ObjectPropertyRange::new(
                IRI::new("http://field33.com/ontologies/@schmolo/ppr/CFAFBD")
                    .unwrap()
                    .into(),
                ObjectUnionOf::new(
                    vec![
                        IRI::new("http://field33.com/ontologies/@schmolo/ppr/CASL")
                            .unwrap()
                            .into(),
                        IRI::new("http://field33.com/ontologies/@schmolo/ppr/RQ")
                            .unwrap()
                            .into(),
                    ],
                    vec![]
                )
                .into(),
                vec![]
            )
            .into()
        );
        assert_eq!(
            o.axioms()[2],
            ObjectPropertyDomain::new(
                IRI::new("http://field33.com/ontologies/@schmolo/ppr/CFAFBD")
                    .unwrap()
                    .into(),
                IRI::new("http://field33.com/ontologies/@schmolo/ppr/CFAR")
                    .unwrap()
                    .into(),
                vec![]
            )
            .into()
        );
    }

    #[test]
    fn unordered_lists_with_union_of() {
        env_logger::try_init().ok();
        let turtle = r##"
        @prefix : <http://test#> .
        @prefix owl: <http://www.w3.org/2002/07/owl#> .
        @prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
        @prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
        @prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

        <http://test#> rdf:type owl:Ontology .
        :A rdf:type owl:Class .
        :B rdf:type owl:Class .
        :C rdf:type owl:Class .
        :D rdf:type owl:Class .
        
        :O1 rdf:type owl:ObjectProperty .
        :O2 rdf:type owl:ObjectProperty .
        :O3 rdf:type owl:ObjectProperty .

        :O1 rdfs:domain :A .
        :O1 rdfs:range :B .

        :O3 rdfs:domain _:94f820e0cfd1aa8d3c80ed2064719ab3 .
        _:94f820e0cfd1aa8d3c80ed2064719ab3 <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://www.w3.org/2002/07/owl#Class> .
        _:94f820e0cfd1aa8d3c80ed2064719ab3 <http://www.w3.org/2002/07/owl#unionOf> _:69012f991a9ca09eff4128f2a17dd3a6 .
        _:69012f991a9ca09eff4128f2a17dd3a6 <http://www.w3.org/1999/02/22-rdf-syntax-ns#first> :A .
        _:69012f991a9ca09eff4128f2a17dd3a6 <http://www.w3.org/1999/02/22-rdf-syntax-ns#rest> _:de25a34541cc5115325dd8d54feda424 .
        _:de25a34541cc5115325dd8d54feda424 <http://www.w3.org/1999/02/22-rdf-syntax-ns#first> :B .
        _:de25a34541cc5115325dd8d54feda424 <http://www.w3.org/1999/02/22-rdf-syntax-ns#rest> <http://www.w3.org/1999/02/22-rdf-syntax-ns#nil> .
  
        :O3 rdfs:range _:1 .
        _:1 <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://www.w3.org/2002/07/owl#Class> .
        _:1 <http://www.w3.org/2002/07/owl#unionOf> _:2 .
        _:3 <http://www.w3.org/1999/02/22-rdf-syntax-ns#first> :D .
        _:3 <http://www.w3.org/1999/02/22-rdf-syntax-ns#rest> <http://www.w3.org/1999/02/22-rdf-syntax-ns#nil> .
        _:2 <http://www.w3.org/1999/02/22-rdf-syntax-ns#first> :C .
        _:2 <http://www.w3.org/1999/02/22-rdf-syntax-ns#rest> _:3 .
        "##;

        harriet::TurtleDocument::parse_full(turtle).unwrap();
        let o = Ontology::parse(
            turtle,
            ParserOptions::builder()
                .known(Declaration::ObjectProperty {
                    iri: IRI::new("http://test#foo").unwrap().into(),
                    annotations: vec![],
                })
                .build(),
        )
        .unwrap();
        println!("{:#?}", o);
        assert_eq!(o.declarations().len(), 7);
        assert_eq!(o.axioms().len(), 4);
        assert_eq!(
            o.axioms()[0],
            ObjectPropertyDomain::new(
                IRI::new("http://test#O1").unwrap().into(),
                IRI::new("http://test#A").unwrap().into(),
                vec![]
            )
            .into()
        );
        assert_eq!(
            o.axioms()[1],
            ObjectPropertyRange::new(
                IRI::new("http://test#O1").unwrap().into(),
                IRI::new("http://test#B").unwrap().into(),
                vec![]
            )
            .into()
        );
        assert_eq!(
            o.axioms()[2],
            ObjectPropertyDomain::new(
                IRI::new("http://test#O3").unwrap().into(),
                ObjectUnionOf::new(
                    vec![
                        IRI::new("http://test#A").unwrap().into(),
                        IRI::new("http://test#B").unwrap().into(),
                    ],
                    vec![]
                )
                .into(),
                vec![]
            )
            .into()
        );
        assert_eq!(
            o.axioms()[3],
            ObjectPropertyRange::new(
                IRI::new("http://test#O3").unwrap().into(),
                ObjectUnionOf::new(
                    vec![
                        IRI::new("http://test#C").unwrap().into(),
                        IRI::new("http://test#D").unwrap().into(),
                    ],
                    vec![]
                )
                .into(),
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
        let o = Ontology::parse(turtle, Default::default()).unwrap();
        println!("{:#?}", o);
        assert_eq!(o.declarations().len(), 3);
        assert_eq!(o.axioms().len(), 1);
        assert_eq!(
            o.axioms()[0],
            Axiom::SubClassOf(SubClassOf::new(
                IRI::new("http://test#Grandfather").unwrap().into(),
                ObjectIntersectionOf::new(
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
        let o = Ontology::parse(turtle, Default::default()).unwrap();

        assert_eq!(o.declarations().len(), 2);
        assert_eq!(o.axioms().len(), 1);
        assert_eq!(
            o.axioms()[0],
            Axiom::SubClassOf(SubClassOf::new(
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
        let o = Ontology::parse(turtle, Default::default()).unwrap();

        println!("{:#?}", o);
        assert_eq!(o.declarations().len(), 2);
        assert_eq!(o.axioms().len(), 1);
        assert_eq!(
            o.axioms()[0],
            Axiom::SubClassOf(SubClassOf::new(
                IRI::new("http://test#Man").unwrap().into(),
                IRI::new("http://test#Person").unwrap().into(),
                vec![Annotation::new(
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
        assert_eq!(o.axioms().len(), 1);
        assert_eq!(
            o.axioms()[0],
            Axiom::AnnotationAssertion(AnnotationAssertion::new(
                IRI::new("http://test#bla").unwrap().into(),
                IRI::new("http://test#Man").unwrap(),
                Literal::String("test".into()).into(),
                vec![Annotation::new(
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

            :DP1 rdf:type owl:Axiom .
            :DP1 owl:annotatedSource :Bob .
            :DP1 owl:annotatedProperty :hasAge .
            :DP1 owl:annotatedTarget 123 .

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
                vec![
                    Annotation::new(
                        well_known::owl_annotatedSource().into(),
                        IRI::new("http://test#Annotation1").unwrap().into(),
                        vec![]
                    ),
                    Annotation::new(
                        IRI::new("http://test#createdAt").unwrap().into(),
                        "2019".into(),
                        vec![]
                    )
                ]
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
                vec![Annotation::new(
                    well_known::owl_annotatedSource().into(),
                    IRI::new("http://test#DP1").unwrap().into(),
                    vec![]
                )]
            )
            .into()
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
        assert_eq!(o.axioms().len(), 1);
        assert_eq!(
            o.axioms()[0],
            AnnotationAssertion::new(
                well_known::rdfs_label(),
                IRI::new("http://field33.com/dataset/foobar#7025935").unwrap(),
                Literal::String("Lorem Ipsum".into()).into(),
                vec![Annotation::new(
                    IRI::new("http://query-server.field33.com/ontology/query-field")
                        .unwrap()
                        .into(),
                    Literal::String("labels".into()).into(),
                    vec![]
                )]
            )
            .into()
        );
    }

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
        assert_eq!(o.axioms().len(), 1);
        assert_eq!(
            o.axioms()[0],
            Axiom::DataPropertyAssertion(DataPropertyAssertion::new(
                IRI::new("http://field33.com/ontologies/@fld33_domain/dora_metrics/TeamChangeFailureRate").unwrap().into(),
                IRI::new("http://field33.com/ontologies/@fld33_domain/dora_metrics/Team2").unwrap().into(),
                Literal::Number {
                    number: 0.into(),
                    type_iri: Some(well_known::xsd_decimal())
                },
                vec![Annotation::new(
                    IRI::new("http://query-server.field33.com/ontology/query-field").unwrap().into(),
                    LiteralOrIRI::Literal("index-1".into()),
                    vec![]
                )]
            ))
        );
    }

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
                vec![Annotation::new(
                    well_known::owl_annotatedSource().into(),
                    IRI::new("http://field33.com/org/org_evlGiemVNyAUTJ7D/node/f63c8031-a7d9-40db-ae87-be04c99537c7").unwrap().into(),
                    vec![]
                )]
            ))
        );
    }

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
                    .into(),
                "foo bar".into(),
                vec![
                    Annotation::new(
                        well_known::owl_annotatedSource().into(),
                        IRI::new("http://field33.com/ontologies/@fld33_domain/dora_metrics/E528ddffc3bf541ffe030ffc413c26b2c7aafa5cAnnotation3").unwrap().into(),
                        vec![]
                    ),
                    Annotation::new(
                        IRI::new("http://field33.com/ontologies/core_change_tracking/createdByImport").unwrap().into(),
                        "GitHub".into(),
                        vec![]
                    ),
                    Annotation::new(
                        IRI::new("http://field33.com/ontologies/core_change_tracking/createdAt").unwrap().into(),
                        Literal::DateTime("2023-02-07T14:42:17Z".into()).into(),
                        vec![]
                    ),
                ]
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
        let o = Ontology::parse(turtle, Default::default()).unwrap();

        assert_eq!(o.declarations().len(), 2);
        assert_eq!(o.axioms().len(), 1);
        assert_eq!(
            o.axioms()[0],
            ClassAssertion::new(
                IRI::new("http://test#Man").unwrap().into(),
                IRI::new("http://test#Bob").unwrap().into(),
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
        :Bob :hasAge "51"^^xsd:nonNegativeInteger .
        :Bob :hasAge 42 .
        :Bob :hasAge "3.14"^^xsd:float .
        :Bob :hasAge true .
        :Bob :hasAge "false"^^xsd:boolean .
        "##;

        harriet::TurtleDocument::parse_full(turtle).unwrap();
        let o = Ontology::parse(turtle, Default::default()).unwrap();

        assert_eq!(o.declarations().len(), 3);
        assert_eq!(o.axioms().len(), 6);
        assert_eq!(
            o.axioms()[0],
            DataPropertyAssertion::new(
                IRI::new("http://test#hasAge").unwrap().into(),
                IRI::new("http://test#Bob").unwrap().into(),
                Literal::Number {
                    number: 51.into(),
                    type_iri: Some(well_known::xsd_nonNegativeInteger())
                },
                vec![]
            )
            .into()
        );
        assert_eq!(
            o.axioms()[1],
            DataPropertyAssertion::new(
                IRI::new("http://test#hasAge").unwrap().into(),
                IRI::new("http://test#Bob").unwrap().into(),
                Literal::Number {
                    number: 42.into(),
                    type_iri: Some(well_known::xsd_integer())
                },
                vec![]
            )
            .into()
        );
        assert_eq!(
            o.axioms()[2],
            DataPropertyAssertion::new(
                IRI::new("http://test#hasAge").unwrap().into(),
                IRI::new("http://test#Bob").unwrap().into(),
                Literal::Number {
                    number: Number::from_f64(3.14).unwrap(),
                    type_iri: Some(well_known::xsd_float())
                },
                vec![]
            )
            .into()
        );
        assert_eq!(
            o.axioms()[3],
            DataPropertyAssertion::new(
                IRI::new("http://test#hasAge").unwrap().into(),
                IRI::new("http://test#Bob").unwrap().into(),
                Literal::Bool(true),
                vec![]
            )
            .into()
        );
        assert_eq!(
            o.axioms()[4],
            DataPropertyAssertion::new(
                IRI::new("http://test#hasAge").unwrap().into(),
                IRI::new("http://test#Bob").unwrap().into(),
                Literal::Bool(false),
                vec![]
            )
            .into()
        );
    }

    #[test]
    fn properties_that_are_data_and_annotations() {
        env_logger::try_init().ok();
        let turtle = r##"
        @prefix : <http://test#> .
        @prefix owl: <http://www.w3.org/2002/07/owl#> .
        @prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
        @prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
        @prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

        <http://test#> rdf:type owl:Ontology .

        :Person rdf:type owl:Class .
        :hasAge rdf:type owl:AnnotationProperty .
        :hasAge rdf:type owl:DatatypeProperty .


        :Person :hasAge "Test" .

        "##;

        harriet::TurtleDocument::parse_full(turtle).unwrap();
        let o = Ontology::parse(turtle, Default::default()).unwrap();
        println!("{:#?}", o);

        assert_eq!(o.declarations().len(), 3);
        assert_eq!(o.axioms().len(), 1);
        assert_eq!(
            o.declarations()[0],
            Declaration::Class {
                iri: IRI::new("http://test#Person").unwrap().into(),
                annotations: vec![]
            }
        );
        assert_eq!(
            o.declarations()[1],
            Declaration::AnnotationProperty {
                iri: IRI::new("http://test#hasAge").unwrap().into(),
                annotations: vec![]
            }
        );
        assert_eq!(
            o.declarations()[2],
            Declaration::DataProperty {
                iri: IRI::new("http://test#hasAge").unwrap().into(),
                annotations: vec![]
            }
        );
        assert_eq!(
            o.axioms()[0],
            AnnotationAssertion::new(
                IRI::new("http://test#hasAge").unwrap().into(),
                IRI::new("http://test#Person").unwrap(),
                "Test".into(),
                vec![]
            )
            .into()
        );
    }

    #[test]
    fn computations_with_blank() {
        env_logger::try_init().ok();
        let turtle = r#"
        @prefix onto1: <http://example.com/ONTO1/> .
        @prefix query_server: <http://query-server.field33.com/ontology/> .
        @prefix owl: <http://www.w3.org/2002/07/owl#> .
        @prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
        @prefix xml: <http://www.w3.org/XML/1998/namespace> .
        @prefix xsd: <http://www.w3.org/2001/XMLSchema#> .
        @prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .

        <http://query-server.field33.com/query/aaaa-bbbb-ccc-dddd> rdf:type owl:Ontology .

        onto1:Individual1 rdf:type owl:NamedIndividual .
        onto1:Individual1 rdfs:label "Person 1" .

        []   rdf:type                  owl:Axiom ;
             owl:annotatedSource       onto1:Individual1 ;
             owl:annotatedProperty     rdfs:label ;
             owl:annotatedTarget       "Person 1" ;
             query_server:query-field  "my_label"^^xsd:string .
        "#;

        harriet::TurtleDocument::parse_full(turtle).unwrap();

        let options = ParserOptions::builder()
            .known(Declaration::AnnotationProperty {
                iri: IRI::new("http://query-server.field33.com/ontology/query-field")
                    .unwrap()
                    .into(),
                annotations: vec![],
            })
            .build();

        let o = Ontology::parse(turtle, options).unwrap();

        assert_eq!(
            o.iri.as_str(),
            "http://query-server.field33.com/query/aaaa-bbbb-ccc-dddd"
        );

        assert_eq!(o.declarations().len(), 1);
        assert_eq!(
            o.declarations()[0],
            Declaration::NamedIndividual {
                iri: IRI::new("http://example.com/ONTO1/Individual1")
                    .unwrap()
                    .into(),
                annotations: vec![]
            }
        );

        assert_eq!(o.axioms().len(), 1);
        assert_eq!(
            o.axioms()[0],
            AnnotationAssertion::new(
                well_known::rdfs_label(),
                IRI::new("http://example.com/ONTO1/Individual1").unwrap(),
                Literal::String("Person 1".into()).into(),
                vec![Annotation::new(
                    IRI::new("http://query-server.field33.com/ontology/query-field")
                        .unwrap()
                        .into(),
                    Literal::String("my_label".into()).into(),
                    vec![]
                )]
            )
            .into()
        );
    }

    #[test]
    fn meta_ontology() {
        env_logger::try_init().ok();
        let turtle = r##"
        @prefix : <http://field33.com/ontologies/@fld33/meta#> .
        @prefix owl: <http://www.w3.org/2002/07/owl#> .
        @prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
        @prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
        @prefix xsd: <http://www.w3.org/2001/XMLSchema#> .
        @prefix registry: <http://field33.com/ontologies/REGISTRY/> .

        <http://field33.com/ontologies/@fld33/meta#> rdf:type owl:Ontology ;
            registry:packageName "@fld33/meta" ;
            rdfs:label "Meta"@en .

        # We need to know that Ontology is a thing
        owl:Ontology rdf:type owl:Class ;
            rdfs:label "Ontology"@en .

        # Graph styles

        :NodeShape rdf:type owl:Class ;
            rdfs:label "NodeShape" .

        :hasNodeShape rdf:type owl:ObjectProperty ;
            rdfs:domain owl:Thing ;
            rdfs:range :NodeShape ;
            rdfs:label "hasNodeShape" .

        :Circle rdf:type :NodeShape ;
            rdfs:label "Circle" .

        :Triangle rdf:type :NodeShape ;
            rdfs:label "Triangle" .

        :Square rdf:type :NodeShape ;
            rdfs:label "Square" .

        :Pill rdf:type :NodeShape ;
            rdfs:label "Pill" .

        # layout stuff

        :Layout rdf:type :owl:Class ;
            rdfs:label "Layout" .

        :FcoseLayout rdf:type :Layout ;
            rdfs:label "Fcose" .
        :TreeLayout rdf:type :Layout ;
            rdfs:label "Fcose" .
        :RandomLayout rdf:type :Layout ;
            rdfs:label "Fcose" .

        :hasLayout rdf:type owl:ObjectProperty ;
            rdfs:domain: owl:Ontology ;
            rdfs:range: :Layout ;
            rdfs:label "has layout" .

        # Facets

        :Facet rdf:type owl:Class .
        
        :Processes rdf:type :Facet .
        :BusinessObjects rdf:type :Facet .
        :OrganisationalAssets rdf:type :Facet .
        :Metrics rdf:type :Facet .
        :Objectives rdf:type :Facet .

        :hasFacet rdf:type owl:ObjectProperty ;
            rdfs:domain owl:Thing ;
            rdfs:range :Facet .
        "##;

        harriet::TurtleDocument::parse_full(turtle).unwrap();
        let o = Ontology::parse(turtle, Default::default()).unwrap();
        println!("{:#?}", o);
        assert_eq!(o.declarations().len(), 6);
        assert_eq!(o.axioms().len(), 27);
    }

    #[test]
    fn people_test() {
        env_logger::try_init().ok();
        let turtle = r##"
            @prefix : <http://field33.com/ontologies/@fld33/people/> .
            @prefix owl: <http://www.w3.org/2002/07/owl#> .
            @prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
            @prefix xml: <http://www.w3.org/XML/1998/namespace> .
            @prefix xsd: <http://www.w3.org/2001/XMLSchema#> .
            @prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
            @prefix registry: <http://field33.com/ontologies/REGISTRY/> .
            @base <http://field33.com/ontologies/@fld33/people/> .
            
            <http://field33.com/ontologies/@fld33/people/> rdf:type owl:Ontology ;
                                                            registry:author "Field 33 <contribution@field33.com>" ;
                                                            registry:canonicalPrefix "people" ;
                                                            registry:category "People" ,
                                                                            "Upper Ontology" ;
                                                            registry:keyword "Field 33 Package" ,
                                                                            "People" ,
                                                                            "Upper Ontology" ;
                                                            registry:ontologyFormatVersion "v1" ;
                                                            registry:packageName "@fld33/people" ;
                                                            registry:packageVersion "0.1.5" ;
                                                            registry:licenseSPDX "Apache-2.0" ;
                                                            registry:repository "https://github.com/field33/ontologies/tree/main/%40fld33/people" ;
                                                            registry:shortDescription "People upper ontology"@en ;
                                                            rdfs:comment "# People Ontology<br>This package is part of the upper ontologies we use at Field 33 and describes non restrictive concepts around people."@en ;
                                                            rdfs:label "People"@en .
            
            
            #################################################################
            #    Data properties
            #################################################################
            
            ###  http://field33.com/ontologies/@fld33/people/FirstName
            :FirstName rdf:type owl:DatatypeProperty ;
                    rdfs:subPropertyOf :Name ;
                    rdfs:domain :Person ;
                    rdfs:range xsd:string ;
                    rdfs:label "First Name"@en .
            
            
            ###  http://field33.com/ontologies/@fld33/people/LastName
            :LastName rdf:type owl:DatatypeProperty ;
                    rdfs:subPropertyOf :Name ;
                    rdfs:domain :Person ;
                    rdfs:range xsd:string ;
                    rdfs:label "Last Name"@en .
            
            
            ###  http://field33.com/ontologies/@fld33/people/Name
            :Name rdf:type owl:DatatypeProperty ;
                    rdfs:domain :Person ;
                    rdfs:range xsd:string ;
                    rdfs:label "Name"@en .
            
            
            #################################################################
            #    Classes
            #################################################################
            
            ###  http://field33.com/ontologies/@fld33/people/Individual
            :Individual rdf:type owl:Class ;
                        rdfs:subClassOf :Person ;
                        rdfs:label "Individual"@en .
            
            
            ###  http://field33.com/ontologies/@fld33/people/Person
            :Person rdf:type owl:Class ;
                    rdfs:label "Person"@en .
            
            
            ###  Generated by the OWL API (version 4.5.9.2019-02-01T07:24:44Z) https://github.com/owlcs/owlapi
        "##;

        harriet::TurtleDocument::parse_full(turtle).unwrap();
        let o = Ontology::parse(turtle, Default::default()).unwrap();
        // println!("{:#?}", o);

        assert_eq!(
            o.declarations()[0],
            Declaration::DataProperty {
                iri: IRI::new("http://field33.com/ontologies/@fld33/people/FirstName")
                    .unwrap()
                    .into(),
                annotations: vec![],
            }
        );
        assert_eq!(
            o.axioms()[8],
            DataPropertyDomain::new(
                IRI::new("http://field33.com/ontologies/@fld33/people/FirstName")
                    .unwrap()
                    .into(),
                IRI::new("http://field33.com/ontologies/@fld33/people/Person")
                    .unwrap()
                    .into(),
                vec![]
            )
            .into()
        );
        assert_eq!(
            o.axioms()[9],
            DataPropertyRange::new(
                IRI::new("http://field33.com/ontologies/@fld33/people/FirstName")
                    .unwrap()
                    .into(),
                IRI::new("http://www.w3.org/2001/XMLSchema#string")
                    .unwrap()
                    .into(),
                vec![]
            )
            .into()
        );
    }

    #[test]
    fn sub_properties() {
        env_logger::try_init().ok();
        let turtle = r##"
        @prefix : <http://test#> .
        @prefix owl: <http://www.w3.org/2002/07/owl#> .
        @prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
        @prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
        @prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

        <http://test#> rdf:type owl:Ontology .

        :aProp rdf:type owl:AnnotationProperty .
        :dProp rdf:type owl:DatatypeProperty .
        :oProp rdf:type owl:ObjectProperty .

        :aPropC rdf:type owl:AnnotationProperty .
        :dPropC rdf:type owl:DatatypeProperty .
        :oPropC rdf:type owl:ObjectProperty .

        :aPropC rdfs:subPropertyOf :aProp .
        :dPropC rdfs:subPropertyOf :dProp .
        :oPropC rdfs:subPropertyOf :oProp .
        "##;

        harriet::TurtleDocument::parse_full(turtle).unwrap();
        let o = Ontology::parse(turtle, Default::default()).unwrap();

        assert_eq!(o.declarations().len(), 6);
        assert_eq!(o.axioms().len(), 3);

        assert_eq!(
            o.axioms()[0],
            SubAnnotationPropertyOf::new(
                IRI::new("http://test#aPropC").unwrap().into(),
                IRI::new("http://test#aProp").unwrap().into(),
                vec![]
            )
            .into()
        );
        assert_eq!(
            o.axioms()[1],
            SubDataPropertyOf::new(
                IRI::new("http://test#dPropC").unwrap().into(),
                IRI::new("http://test#dProp").unwrap().into(),
                vec![]
            )
            .into()
        );
        assert_eq!(
            o.axioms()[2],
            SubObjectPropertyOf::new(
                crate::owl::ObjectPropertyConstructor::IRI(
                    IRI::new("http://test#oPropC").unwrap().into()
                ),
                IRI::new("http://test#oProp").unwrap().into(),
                vec![]
            )
            .into()
        );
    }

    #[test]
    fn equivalent_classes() {
        env_logger::try_init().ok();
        let turtle = r##"
        @prefix : <http://test#> .
        @prefix owl: <http://www.w3.org/2002/07/owl#> .
        @prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
        @prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
        @prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

        <http://test#> rdf:type owl:Ontology .

        :A rdf:type owl:Class .
        :B rdf:type owl:Class .
        :A owl:equivalentClass :B .

        "##;

        harriet::TurtleDocument::parse_full(turtle).unwrap();
        let o = Ontology::parse(turtle, Default::default()).unwrap();

        assert_eq!(o.declarations().len(), 2);
        assert_eq!(o.axioms().len(), 1);

        assert_eq!(
            o.axioms()[0],
            EquivalentClasses::new(
                IRI::new("http://test#A").unwrap().into(),
                IRI::new("http://test#B").unwrap().into(),
                vec![]
            )
            .into()
        );
    }

    #[test]
    fn computation() {
        env_logger::try_init().ok();
        let turtle = r##"
            @prefix :     <http://test#> .
            @prefix owl:  <http://www.w3.org/2002/07/owl#> .
            @prefix rdf:  <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
            @prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
            @prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
            @prefix fno:  <https://w3id.org/function/ontology#> .
            @prefix comp: <http://field33.com/ontologies/@fld33/computation/> .

            <http://test#> rdf:type owl:Ontology .

            ## required knowledge

            fno:Function rdf:type owl:Class .
            fno:Parameter rdf:type owl:Class .
            fno:expects rdf:type owl:ObjectProperty .
            fno:returns rdf:type owl:ObjectProperty .
            fno:predicate rdf:type owl:ObjectProperty .
            fno:type rdf:type owl:ObjectProperty .
            fno:required rdf:type owl:DatatypeProperty .
            fno:Output rdf:type owl:Class .

            comp:sparqlVariable rdf:type owl:DatatypeProperty .
            comp:sparqlQuery rdf:type owl:DatatypeProperty .

            ## actual test

            :AgileTeam rdf:type owl:Class .

            :DeploymentFrequency
                rdf:type           owl:DatatypeProperty ;
                rdfs:domain        :AgileTeam ;
                rdfs:range         xsd:decimal ;
                rdfs:label         "Deployment Frequency"@en .

            :DeploymentFrequencyOutput
                rdf:type            fno:Output ;
                fno:predicate       :DeploymentFrequency ;
                comp:sparqlVariable "DeploymentCountPerAgileTeam" ;
                fno:required        "false"^^xsd:boolean ;
                fno:type            xsd:decimal .

            :TeamParam
                rdf:type            fno:Parameter ;
                fno:predicate       comp:computationSubject ;
                comp:sparqlVariable "AgileTeam" ;
                fno:type            :AgileTeam ;
                fno:required        "true"^^xsd:boolean .

            :FromDateParam
                rdf:type             fno:Parameter ;
                comp:sparqlVariable "FromDate" ;
                fno:type            :createdAt ;
                fno:required        "true"^^xsd:boolean .

            :ToDateParam
                rdf:type            fno:Parameter ;
                comp:sparqlVariable "ToDate" ;
                fno:type            :createdAt ;
                fno:required        "true"^^xsd:boolean .
              
            :ComputeDeploymentFrequency
                rdf:type        fno:Function ;
                rdf:type        comp:Computation ;
                fno:expects      ( :TeamParam :FromDateParam :ToDateParam ) ;
                fno:returns      ( :DeploymentFrequencyOutput ) ;
                comp:sparqlQuery """
                    PREFIX : <http://field33.com/ontologies/@fld33_domain/dora_metrics/>
                    PREFIX owl: <http://www.w3.org/2002/07/owl#>
                    PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
                    PREFIX xml: <http://www.w3.org/XML/1998/namespace>
                    PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
                    PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>
                    PREFIX org: <http://field33.com/ontologies/@fld33/organization/>
                    PREFIX metric: <http://field33.com/ontologies/@fld33_domain/software_team_metric/>
                    PREFIX develop: <http://field33.com/ontologies/@fld33_domain/software_development/>

                    SELECT ?AgileTeam ?DeploymentCountPerAgileTeam
                    WHERE {
                        ?AgileTeam rdf:type :AgileTeam .
                        ?AgileTeam rdf:type owl:NamedIndividual .

                        LATERAL {
                            OPTIONAL
                            {
                                SELECT ?AgileTeam ?FromDate ?ToDate (count(?deployment) AS ?countDeploymentsOnDate)
                                WHERE {
                                    ?employee :partOf ?AgileTeam .
                                    ?employee rdf:type :Employee .
                                    ?employee rdf:type owl:NamedIndividual .

                                    ?deployment :authoredBy ?employee .
                                    ?deployment rdf:type develop:Deployment .
                                    ?deployment rdf:type owl:NamedIndividual .
                                    ?deployment :deploymentDate ?deploymentDate .
                                    
                                    BIND(
                                        xsd:date(CONCAT(
                                            YEAR(?deploymentDate),
                                            "-",
                                            MONTH(?deploymentDate),
                                            "-",
                                            DAY(?deploymentDate)
                                        )) AS ?dateGrouping)

                                    FILTER (?deploymentDate >= xsd:dateTime(?FromDate)) .
                                    FILTER (?deploymentDate <= xsd:dateTime(?ToDate)) .
                                } GROUP BY ?AgileTeam ?dateGrouping ?FromDate ?ToDate
                            }
                        }
                        BIND((xsd:dateTime(?ToDate) - xsd:dateTime(?FromDate)) AS ?duration) .
                        BIND(COALESCE(xsd:integer(REPLACE(STR(?duration), "[^0-9]", "", "i")), 0) AS ?d) .
                        # BIND( (?d / 7) AS ?numberOfWeeksBetween) .
                        BIND( coalesce(?countDeploymentsOnDate, 0) AS ?countOfDeployments) .
                        BIND( ( ?countOfDeployments / ?d ) AS ?DeploymentCountPerAgileTeam ) .
                    }
                """ .

        "##;

        // let a = harriet::TurtleDocument::parse_full(turtle).unwrap();
        // let ts = harriet::triple_production::TripleProducer::produce_for_document(&a).unwrap();
        // for t in ts {
        //     println!("{:?} {:?} {:?}", t.subject, t.predicate, t.object);
        // }

        let o = Ontology::parse(turtle, Default::default()).unwrap();
        println!("{:#?}", o);
    }
}
