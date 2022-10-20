use std::{collections::HashMap, rc::Rc};

use harriet::triple_production::RdfTriple;

use serde::{Deserialize, Serialize};

use crate::{
    api::Ontology,
    error::Error,
    owl::{well_known, Declaration, IRI},
    parser::matcher::{IRIOrBlank, MatchResult, RdfMatcher},
};

use self::matcher::{get_prefixes, MatcherState};

mod collector;
mod matcher;
use collector::*;

mod annotations;
mod axioms;
mod blank_nodes;
mod declarations;
mod object_property_assertions;
mod sequences;

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
        for phase in 0..3 {
            matchers.clear();
            matcher_instances.clear();
            match phase {
                0 => {
                    declarations::match_declarations(&mut matchers, &prefixes)?;
                    sequences::match_sequences(&mut matchers, &prefixes)?;
                    blank_nodes::match_blank_nodes(&mut matchers, &prefixes)?;
                    annotations::match_simple_annotation_assertions(&mut matchers, &prefixes)?;
                }
                1 => {
                    annotations::match_annotations(&mut matchers, &prefixes)?;
                }
                _ => {
                    axioms::match_axioms(&mut matchers, &prefixes)?;
                    object_property_assertions::push(&mut matchers, &prefixes)?;
                    annotations::match_annotation_assertions(&mut matchers, &prefixes)?;
                }
            }

            for triple in triples.iter() {
                for (matcher_id, (m, _)) in matchers.iter().enumerate() {
                    let subject: IRIOrBlank = triple.subject.clone().into();
                    // (1) Take each ongoing matcher state and check whether it matches this new triple
                    for (_, (matcher_id, triples, mstate, finished)) in matcher_instances.iter_mut()
                    {
                        let (m, _) = &matchers[*matcher_id];
                        parser_debug!(
                            m,
                            "         ################### Matching ({:?}, {})",
                            &subject,
                            m.name()
                        );

                        // (1) If so, keep matching. Maybe mark as finished.
                        if let MatchResult::Matched(f) = m.matches(triple.clone(), mstate) {
                            *finished = f;
                            if !triples.iter().any(|t| Rc::ptr_eq(t, triple)) {
                                triples.push(triple.clone());
                            }
                        }
                    }

                    // (1) Start matcher with new state (if there is no current matcher state)
                    let mut mstate = MatcherState::new();
                    if let std::collections::hash_map::Entry::Vacant(e) =
                        matcher_instances.entry(matcher_id)
                    {
                        if let MatchResult::Matched(finished) =
                            m.matches(triple.clone(), &mut mstate)
                        {
                            parser_debug!(
                                m,
                                "New matching state for ({:?}, {})",
                                &subject,
                                m.name()
                            );
                            e.insert((matcher_id, vec![triple.clone()], mstate, finished));
                        }
                    }
                }

                // (2) Handle matchers that fully matched a set of triples
                let mut finished_matcher_instances = Vec::new();
                // let mut resolved_triples = Vec::new();
                for (matcher_ins_id, (mid, _triples, mstate, finished)) in matcher_instances.iter()
                {
                    if *finished {
                        finished_matcher_instances.push(*matcher_ins_id);
                        let (_m, handler) = &matchers[*mid];
                        if !handler(mstate, &mut collector, &indexed_options)? {
                            // todo: did not meet semantic criteria
                        }
                    }
                }
                // (2) Remove all handled matcher instances
                for matcher_ins_id in finished_matcher_instances {
                    matcher_instances.remove(&matcher_ins_id);
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
    pub fn is_annotation(&self, iri: &IRI) -> bool {
        if let Some(i) = self.index.get(iri) {
            matches!(
                self.known.get(*i),
                Some(Declaration::AnnotationProperty(_, _))
            )
        } else {
            false
        }
    }

    pub fn is_data_prop(&self, iri: &IRI) -> bool {
        if let Some(i) = self.index.get(iri) {
            matches!(self.known.get(*i), Some(Declaration::DataProperty(_, _)))
        } else {
            false
        }
    }

    fn is_object_prop(&self, iri: &IRI) -> bool {
        if let Some(i) = self.index.get(iri) {
            matches!(self.known.get(*i), Some(Declaration::ObjectProperty(_, _)))
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
                Declaration::Class(iri, _) => iri.as_iri(),
                Declaration::NamedIndividual(iri, _) => iri.as_iri(),
                Declaration::ObjectProperty(iri, _) => iri.as_iri(),
                Declaration::DataProperty(iri, _) => iri.as_iri(),
                Declaration::AnnotationProperty(iri, _) => iri.as_iri(),
                Declaration::Datatype(iri, _) => iri.as_iri(),
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
            DataPropertyAssertion, Declaration, Literal, LiteralOrIRI, ObjectIntersectionOf,
            ObjectPropertyAssertion, ObjectPropertyDomain, ObjectPropertyRange, SubClassOf, IRI,
        },
        parser::ParserOptions,
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

        :hasWife rdfs:domain :Man ;
                 rdfs:range  :Woman .

        "##;

        harriet::TurtleDocument::parse_full(turtle).unwrap();
        let o = Ontology::parse(turtle, Default::default()).unwrap();
        assert_eq!(o.declarations().len(), 0);
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

        "##;

        harriet::TurtleDocument::parse_full(turtle).unwrap();
        let o = Ontology::parse(turtle, Default::default()).unwrap();
        assert_eq!(o.declarations().len(), 1);
        assert_eq!(o.axioms().len(), 7);
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
                .known(Declaration::ObjectProperty(
                    IRI::new("http://test#foo").unwrap().into(),
                    vec![],
                ))
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

        :Man rdfs:comment "test" .
        []   rdf:type               owl:Axiom ;
             owl:annotatedSource    :Man ;
             owl:annotatedProperty  rdfs:comment ;
             owl:annotatedTarget    "test" ;
             rdfs:comment           "States that every man is a person."^^xsd:string .

        "##;

        harriet::TurtleDocument::parse_full(turtle).unwrap();
        let o = Ontology::parse(turtle, Default::default()).unwrap();

        assert_eq!(o.declarations().len(), 1);
        assert_eq!(o.axioms().len(), 1);
        assert_eq!(
            o.axioms()[0],
            Axiom::AnnotationAssertion(AnnotationAssertion::new(
                IRI::new("http://www.w3.org/2000/01/rdf-schema#comment")
                    .unwrap()
                    .into(),
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
            .known(Declaration::AnnotationProperty(
                IRI::new("http://query-server.field33.com/ontology/query-field")
                    .unwrap()
                    .into(),
                vec![],
            ))
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
            o.axioms()[1],
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
            o.axioms()[2],
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
            o.axioms()[3],
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
            o.axioms()[4],
            DataPropertyAssertion::new(
                IRI::new("http://test#hasAge").unwrap().into(),
                IRI::new("http://test#Bob").unwrap().into(),
                Literal::Bool(true),
                vec![]
            )
            .into()
        );
        assert_eq!(
            o.axioms()[5],
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
            .known(Declaration::AnnotationProperty(
                IRI::new("http://query-server.field33.com/ontology/query-field")
                    .unwrap()
                    .into(),
                vec![],
            ))
            .build();

        let o = Ontology::parse(turtle, options).unwrap();

        assert_eq!(
            o.iri.as_str(),
            "http://query-server.field33.com/query/aaaa-bbbb-ccc-dddd"
        );

        assert_eq!(o.declarations().len(), 1);
        assert_eq!(
            o.declarations()[0],
            Declaration::NamedIndividual(
                IRI::new("http://example.com/ONTO1/Individual1")
                    .unwrap()
                    .into(),
                vec![]
            )
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

    // TODO: Support this
    // #[test]
    // fn computations() {
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
    //     onto1:Individual1LabelAnn1 rdf:type owl:Annotation .
    //     onto1:Individual1LabelAnn1 owl:annotatedSource onto1:Individual1 .
    //     onto1:Individual1LabelAnn1 owl:annotatedProperty rdfs:label .
    //     onto1:Individual1LabelAnn1 owl:annotatedTarget "Person 1" .
    //     onto1:Individual1LabelAnn1 query_server:query-field "my_label" .
    //     "#;

    //     harriet::TurtleDocument::parse_full(turtle).unwrap();
    //     let options = ParserOptions::builder()
    //     .known(Declaration::AnnotationProperty(
    //         IRI::new("http://query-server.field33.com/ontology/query-field")
    //             .unwrap()
    //             .into(),
    //         vec![],
    //     ))
    //     .build();
    //     let o = Ontology::parse(turtle, options).unwrap();

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

    //     assert_eq!(o.axioms().len(), 2);
    //     assert_eq!(
    //         o.axioms()[0],
    //         AnnotationAssertion(
    //             well_known::rdfs_label(),
    //             IRI::new("http://example.com/ONTO1/Individual1").unwrap(),
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
}
