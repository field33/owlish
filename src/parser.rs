use crate::{
    api::Ontology,
    owl::{
        well_known, AnnotationAssertion, AsymmetricObjectProperty, ClassAssertion, ClassIRI,
        Declaration, IRIBuilder, LiteralOrIRI, ObjectPropertyAssertion, ObjectPropertyDomain,
        ObjectPropertyRange, SubClassOf, SymmetricObjectProperty, IRI,
    },
};

impl Ontology {
    pub fn parse(ttl: &str) -> Result<Self, String> {
        harriet::TurtleDocument::parse_full(ttl)
            .map_err(|e| format!("{:?}", e))
            .map(|td| td.into())
    }
}

trait Parse<'a> {
    fn parse(&'a self, ontology: &mut Ontology);
}

impl<'a> From<harriet::TurtleDocument<'a>> for Ontology {
    fn from(turtle: harriet::TurtleDocument<'a>) -> Self {
        let mut ontology = Self {
            iri: IRI::new("https://no_url").unwrap(),
            owl: crate::owl::Ontology::new(vec![], vec![]),
            imports: Default::default(),
        };

        for item in turtle.statements {
            // parse_item(item, &mut ontology);
            item.parse(&mut ontology);
        }

        ontology
    }
}

// impl<'a> Parse<'a> for harriet::Item<'a> {
//     fn parse(&'a self, ontology: &mut Ontology) {
//         match self {
//             harriet::Statement(stmnt) => stmnt.parse(ontology),
//             // harriet::Item::Comment(_) => {
//             // ignore comments
//             // }
//         };
//     }
// }

impl<'a> Parse<'a> for harriet::Statement<'a> {
    fn parse(&'a self, ontology: &mut Ontology) {
        match self {
            harriet::Statement::Directive(directive) => directive.parse(ontology),
            harriet::Statement::Triples(triples) => triples.parse(ontology),
        }
    }
}

impl<'a> Parse<'a> for harriet::Directive<'a> {
    fn parse(&'a self, ontology: &mut Ontology) {
        match self {
            harriet::Directive::Base(base) => {
                if let Ok(iri) = IRI::new(&base.iri.iri) {
                    ontology.iri = iri
                }
            }
            harriet::Directive::Prefix(prefix) => {
                //
                match prefix.prefix.as_ref().map(|s| s.as_ref()) {
                    None => {
                        ontology.iri = IRI::new(&prefix.iri.iri).unwrap();
                    }
                    Some(name) => {
                        ontology
                            .imports
                            .insert(name.into(), IRI::new(&prefix.iri.iri).unwrap());
                    }
                }
            }
            harriet::Directive::SparqlBase(_) => {}
            harriet::Directive::SparqlPrefix(_) => {}
        }
    }
}

impl<'a> Parse<'a> for harriet::Triples<'a> {
    fn parse(&'a self, ontology: &mut Ontology) {
        match self {
            harriet::Triples::Labeled(_, subject, pol) => {
                let iri: Option<IRI> = match subject {
                    harriet::Subject::IRI(iri) => {
                        IRI::try_from((&ontology.iri_builder(), iri)).ok()
                    }
                    harriet::Subject::Collection(_col) => {
                        todo!()
                    }
                    _ => {
                        todo!()
                    }
                };
                if let Some(subject_iri) = iri {
                    for (_, predicate, object_list, _) in &pol.list {
                        let objects = &object_list.list;

                        let rdf_type = harriet::IRI::PrefixedName(harriet::PrefixedName {
                            prefix: Some("rdf".into()),
                            name: Some("type".into()),
                        });
                        let predicate = match predicate {
                            harriet::Verb::IRI(iri) => iri,
                            harriet::Verb::A => &rdf_type,
                        };

                        match predicate {
                            harriet::IRI::PrefixedName(pn) => {
                                match (
                                    pn.prefix.as_ref().map(|s| s.as_ref()),
                                    pn.name.as_ref().map(|s| s.as_ref()),
                                ) {
                                    (Some("owl"), Some("imports")) => {
                                        // TODO: ignore?
                                    }
                                    (Some("rdf"), Some("type")) => {
                                        parse_rdf_type(ontology, &subject_iri, objects)
                                    }
                                    (Some("rdfs"), Some("subClassOf")) => {
                                        parse_rdfs_sub_class_of(ontology, &subject_iri, objects)
                                    }
                                    (Some("rdfs"), Some("label")) => {
                                        parse_rdfs_label(ontology, &subject_iri, objects)
                                    }
                                    (Some("rdfs"), Some("range")) => {
                                        parse_rdfs_range(ontology, &subject_iri, objects)
                                    }
                                    (Some("rdfs"), Some("domain")) => {
                                        parse_rdfs_domain(ontology, &subject_iri, objects)
                                    }
                                    (Some(prefix), Some(name)) => {
                                        if let Some(prop_iri) =
                                            ontology.iri_builder().from::<IRI>(prefix, name)
                                        {
                                            for (_, _, object) in objects {
                                                match object {
                                                    harriet::Object::IRI(iri) => {
                                                        match IRI::try_from((
                                                            &ontology.iri_builder(),
                                                            iri,
                                                        )) {
                                                            Ok(object_iri) => {
                                                                ontology.owl.axioms.push(
                                                                    ObjectPropertyAssertion(
                                                                        prop_iri.clone().into(),
                                                                        subject_iri.clone().into(),
                                                                        object_iri.into(),
                                                                        vec![],
                                                                    )
                                                                    .into(),
                                                                )
                                                            }
                                                            Err(_) => todo!(),
                                                        }
                                                    }
                                                    harriet::Object::Collection(_) => todo!(),
                                                    harriet::Object::BlankNodePropertyList(_) => {
                                                        todo!()
                                                    }
                                                    harriet::Object::Literal(literal) => {
                                                        match literal {
                                                            harriet::Literal::RDFLiteral(
                                                                rdf_lit,
                                                            ) => ontology.owl.axioms.push(
                                                                AnnotationAssertion(
                                                                    prop_iri.clone().into(),
                                                                    subject_iri.clone().into(),
                                                                    rdf_lit
                                                                        .string
                                                                        .to_string()
                                                                        .into(),
                                                                    vec![],
                                                                )
                                                                .into(),
                                                            ),
                                                            harriet::Literal::BooleanLiteral(b) => {
                                                                ontology.owl.axioms.push(
                                                                    AnnotationAssertion(
                                                                        prop_iri.clone().into(),
                                                                        subject_iri.clone().into(),
                                                                        b.bool.into(),
                                                                        vec![],
                                                                    )
                                                                    .into(),
                                                                )
                                                            }
                                                            _ => {
                                                                todo!()
                                                            }
                                                        }
                                                        // println!("literal {:?} {:?}", prefix, name);
                                                    }
                                                    _ => {
                                                        todo!()
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    _ => {
                                        // TODO: ignored
                                    }
                                }
                            }
                            harriet::IRI::IRIReference(_) => todo!(),
                        }
                    }
                }
            }
        }
    }
}

fn parse_rdfs_range(
    ontology: &mut Ontology,
    subject_iri: &IRI,
    objects: &Vec<(
        Option<harriet::Whitespace>,
        Option<harriet::Whitespace>,
        harriet::Object,
    )>,
) {
    let iri_builder = ontology.iri_builder();
    for (_, _, obj) in objects {
        match obj {
            harriet::Object::IRI(object_iri) => ontology.owl.axioms.push(
                ObjectPropertyRange(
                    subject_iri.clone().into(),
                    IRI::try_from((&iri_builder, object_iri)).unwrap().into(),
                )
                .into(),
            ),
            harriet::Object::Collection(_) => todo!(),
            harriet::Object::BlankNodePropertyList(bn) => {
                for (_, prop_iri, object_list, _) in &bn.list.list {
                    println!("{:?} {:?}", prop_iri, object_list)
                }
            }
            harriet::Object::Literal(_) => todo!(),
            _ => {}
        }
    }
}

fn parse_rdfs_domain(
    ontology: &mut Ontology,
    subject_iri: &IRI,
    objects: &Vec<(
        Option<harriet::Whitespace>,
        Option<harriet::Whitespace>,
        harriet::Object,
    )>,
) {
    let iri_builder = ontology.iri_builder();
    for (_, _, obj) in objects {
        match obj {
            harriet::Object::IRI(object_iri) => ontology.owl.axioms.push(
                ObjectPropertyDomain(
                    subject_iri.clone().into(),
                    IRI::try_from((&iri_builder, object_iri)).unwrap().into(),
                )
                .into(),
            ),
            harriet::Object::Collection(_) => todo!(),
            harriet::Object::BlankNodePropertyList(_) => todo!(),
            harriet::Object::Literal(_) => todo!(),
            _ => {}
        }
    }
}

fn parse_rdfs_label(
    ontology: &mut Ontology,
    subject_iri: &IRI,
    objects: &Vec<(
        Option<harriet::Whitespace>,
        Option<harriet::Whitespace>,
        harriet::Object,
    )>,
) {
    for (_, _, obj) in objects {
        match obj {
            harriet::Object::Literal(label) => match label {
                harriet::Literal::RDFLiteral(lit) => ontology.owl.axioms.push(
                    AnnotationAssertion(
                        well_known::rdfs_label(),
                        subject_iri.clone(),
                        LiteralOrIRI::from(lit.string.to_string()),
                        vec![],
                    )
                    .into(),
                ),
                harriet::Literal::BooleanLiteral(_) => todo!(),
                _ => {}
            },
            harriet::Object::IRI(_) => todo!(),
            harriet::Object::Collection(_) => todo!(),
            harriet::Object::BlankNodePropertyList(_) => todo!(),
            _ => {}
        }
    }
}

fn parse_rdf_type(
    ontology: &mut Ontology,
    subject_iri: &IRI,
    objects: &Vec<(
        Option<harriet::Whitespace>,
        Option<harriet::Whitespace>,
        harriet::Object,
    )>,
) {
    for (_, _, object) in objects {
        match object {
            harriet::Object::IRI(iri) => {
                if let Ok(iri) = IRI::try_from((&ontology.iri_builder(), iri)) {
                    if well_known::owl_Class().as_iri() == &iri {
                        ontology
                            .owl
                            .declarations
                            .push(Declaration::Class(subject_iri.clone().into()))
                    } else if well_known::owl_SymmetricProperty().as_iri() == &iri {
                        ontology
                            .owl
                            .axioms
                            .push(SymmetricObjectProperty(subject_iri.clone().into()).into());
                    } else if well_known::owl_AsymmetricProperty().as_iri() == &iri {
                        ontology
                            .owl
                            .axioms
                            .push(AsymmetricObjectProperty(subject_iri.clone().into()).into());
                    } else if !is_property(&iri) {
                        ontology.owl.axioms.push(
                            ClassAssertion(ClassIRI::from(iri).into(), subject_iri.clone().into())
                                .into(),
                        )
                    }
                }
            }
            harriet::Object::Collection(_) => todo!(),
            harriet::Object::BlankNodePropertyList(_) => todo!(),
            harriet::Object::Literal(_) => todo!(),
            _ => {}
        }
    }
}

fn is_property(iri: &IRI) -> bool {
    iri == well_known::owl_AsymmetricProperty().as_iri()
        || iri == well_known::owl_SymmetricProperty().as_iri()
        || iri == well_known::owl_ObjectProperty().as_iri()
}

fn parse_rdfs_sub_class_of(
    ontology: &mut Ontology,
    subject_iri: &IRI,
    objects: &Vec<(
        Option<harriet::Whitespace>,
        Option<harriet::Whitespace>,
        harriet::Object,
    )>,
) {
    for (_, _, object) in objects {
        match object {
            harriet::Object::IRI(iri) => {
                if let Ok(iri) = IRI::try_from((&ontology.iri_builder(), iri)) {
                    ontology.owl.axioms.push(
                        SubClassOf(
                            ClassIRI::from(subject_iri.clone()).into(),
                            ClassIRI::from(iri).into(),
                            vec![],
                        )
                        .into(),
                    );
                }
            }
            harriet::Object::Collection(_) => todo!(),
            harriet::Object::BlankNodePropertyList(_) => {
                todo!()
            }
            harriet::Object::Literal(_) => todo!(),
            _ => {
                todo!()
            }
        }
    }
}

impl<'a> TryFrom<(&IRIBuilder, &'a harriet::IRI<'a>)> for IRI {
    type Error = ();
    fn try_from((iri_builder, harriet_iri): (&IRIBuilder, &harriet::IRI)) -> Result<Self, ()> {
        match harriet_iri {
            harriet::IRI::IRIReference(iriref) => IRI::new(&iriref.iri).map_err(|_e| ()),
            harriet::IRI::PrefixedName(pn) => iri_builder
                .from_opt(&pn.prefix, &pn.name)
                .map(Ok)
                .unwrap_or(Err(())),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        api::Ontology,
        owl::{
            well_known, AnnotationAssertion, ClassAssertion, ClassIRI, ObjectPropertyAssertion,
            ObjectPropertyDomain, IRI,
        },
    };

    #[test]
    fn test_parse_meta_ontology() {
        let turtle = r##"
        @prefix : <http://field33.com/ontologies/@fld33/meta/> .
        @prefix owl: <http://www.w3.org/2002/07/owl#> .
        @prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
        @prefix xml: <http://www.w3.org/XML/1998/namespace> .
        @prefix xsd: <http://www.w3.org/2001/XMLSchema#> .
        @prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
        @prefix registry: <http://field33.com/ontologies/REGISTRY/> .
        @base <http://field33.com/ontologies/@fld33/process/> .

        <http://field33.com/ontologies/@fld33/meta/> rdf:type owl:Ontology ;
            registry:packageName "@fld33/meta" ;
            rdfs:label "Meta"@en .

        # We need to know that Ontology is a thing
        owl:Ontology rdf:type owl:Class ;
            rdfs:label "Activity"@en .

        :View rdf:type owl:Class ;
            rdfs:label "View" .

        # View formatting

        :NodeShape rdf:type owl:Class ;
            rdfs:label "NodeShape" .

        :hasNodeShape rdf:type owl:ObjectProperty ;
            rdfs:domain owl:Ontology ;
            rdfs:range :NodeShape ;
            rdfs:label "hasNodeShape" .

        :Circle rdf:type :NodeShape ;
            rdfs:label "Circle" .

        :Triangle rdf:type :NodeShape ;
            rdfs:label "Triangle" .

        # Individual views

        :GraphView rdf:type :View ;
            rdfs:label "GraphView" .
        "##;

        let td = harriet::TurtleDocument::parse_full(turtle).unwrap();

        let o: Ontology = td.into();

        assert_eq!(
            o.iri.to_string(),
            "http://field33.com/ontologies/@fld33/process/"
        );

        assert_eq!(o.axioms().len(), 15);
    }

    #[test]
    fn test_parse_simple_ontology() {
        let turtle = r##"
        @prefix : <http://field33.com/ontologies/@fld33/process/> .
        @prefix owl: <http://www.w3.org/2002/07/owl#> .
        @prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
        @prefix xml: <http://www.w3.org/XML/1998/namespace> .
        @prefix xsd: <http://www.w3.org/2001/XMLSchema#> .
        @prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
        @prefix meta: <http://field33.com/ontologies/@fld33/meta/> .
        @prefix registry: <http://field33.com/ontologies/REGISTRY/> .
        @base <http://field33.com/ontologies/@fld33/process/> .

        <http://field33.com/ontologies/@fld33/process/> rdf:type owl:Ontology ;
            registry:author "Field 33" ;
            registry:canonicalPrefix "process" ;
            registry:category "Process"@en ,
                            "Upper Ontology"@en ;
            registry:keyword "Field 33 Package"@en ,
                            "Process"@en ,
                            "Upper Ontology"@en ;
            registry:ontologyFormatVersion "v1" ;
            registry:packageName "@fld33/process" ;
            registry:packageVersion "0.1.2" ;
            registry:repository "https://github.com/field33/ontology-workspace/tree/main/%40fld33/process" ;
            registry:shortDescription "Process upper ontology"@en ;
            rdfs:comment "#Process Ontology<br>This package is part of the upper ontologies and describes concepts around processes."@en ;
            meta:hasNodeShape meta:Triangle ;
            rdfs:label "Processes"@en .

        :Activity rdf:type owl:Class ;
            rdfs:label "Activity"@en .

        :Process rdf:type owl:Class ;
            rdfs:label "Process"@en .

        :seriesOf rdf:type owl:ObjectProperty ;
            rdfs:domain :Process ;
            rdfs:range :Activity ;
            rdfs:label "Series of"@en .

        :hasNext rdf:type owl:ObjectProperty ;
            rdfs:domain :Activity ;
            rdfs:range :Activity ;
            rdfs:label "Has next"@en .
        "##;

        let td = harriet::TurtleDocument::parse_full(turtle).unwrap();

        let o: Ontology = td.into();

        assert_eq!(
            o.iri.to_string(),
            "http://field33.com/ontologies/@fld33/process/"
        );

        assert_eq!(
            o.axioms()[0],
            ClassAssertion(
                ClassIRI::from(well_known::owl_Ontology()).into(),
                IRI::new("http://field33.com/ontologies/@fld33/process/")
                    .unwrap()
                    .into()
            )
            .into()
        );
        assert_eq!(
            o.axioms()[14],
            ObjectPropertyAssertion(
                o.iri_builder().from("meta", "hasNodeShape").unwrap(),
                IRI::new("http://field33.com/ontologies/@fld33/process/")
                    .unwrap()
                    .into(),
                IRI::new("http://field33.com/ontologies/@fld33/meta/#Triangle")
                    .unwrap()
                    .into(),
                vec![]
            )
            .into()
        );
        assert_eq!(
            o.axioms()[15],
            AnnotationAssertion(
                well_known::rdfs_label(),
                IRI::new("http://field33.com/ontologies/@fld33/process/").unwrap(),
                "Processes".into(),
                vec![]
            )
            .into()
        );
        assert_eq!(
            o.axioms()[16],
            AnnotationAssertion(
                well_known::rdfs_label(),
                IRI::new("http://field33.com/ontologies/@fld33/process/#Activity").unwrap(),
                "Activity".into(),
                vec![]
            )
            .into()
        );
        assert_eq!(
            o.axioms()[17],
            AnnotationAssertion(
                well_known::rdfs_label(),
                IRI::new("http://field33.com/ontologies/@fld33/process/#Process").unwrap(),
                "Process".into(),
                vec![]
            )
            .into()
        );
        assert_eq!(
            o.axioms()[18],
            ObjectPropertyDomain(
                IRI::new("http://field33.com/ontologies/@fld33/process/#seriesOf")
                    .unwrap()
                    .into(),
                IRI::new("http://field33.com/ontologies/@fld33/process/#Process")
                    .unwrap()
                    .into(),
            )
            .into()
        );
        assert_eq!(o.axioms().len(), 24);
    }

    #[test]
    fn test_parse_process_individuals_ontology() {
        let turtle = r##"
            @prefix : <http://field33.com/ontologies/@fld33/process/Team1> .
            @prefix owl: <http://www.w3.org/2002/07/owl#> .
            @prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
            @prefix xml: <http://www.w3.org/XML/1998/namespace> .
            @prefix xsd: <http://www.w3.org/2001/XMLSchema#> .
            @prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
            @prefix registry: <http://field33.com/ontologies/REGISTRY/> .
            @prefix proc: <http://field33.com/ontologies/@fld33/process/> .
            @base <http://field33.com/ontologies/@fld33/process/> .


            <http://field33.com/ontologies/@fld33/process/> rdf:type owl:Ontology ;
                registry:author "Field 33" ;
                registry:canonicalPrefix "process" ;
                registry:category "Process"@en ,
                                "Upper Ontology"@en ;
                registry:keyword "Field 33 Package"@en ,
                                "Process"@en ,
                                "Upper Ontology"@en ;
                registry:ontologyFormatVersion "v1" ;
                registry:packageName "@fld33/process" ;
                registry:packageVersion "0.1.2" ;
                registry:repository "https://github.com/field33/ontology-workspace/tree/main/%40fld33/process" ;
                registry:shortDescription "Process upper ontology"@en ;
                rdfs:comment "#Process Ontology<br>This package is part of the upper ontologies and describes concepts around processes."@en ;
                rdfs:label "Process"@en .

            :SoftwareDevelopment rdf:type proc:Proc ;
            rdfs:label "Software development"@en .
            :ProductManagement rdf:type proc:Proc ;
            rdfs:label "Product management"@en .

            :Plan rdf:type proc:Activity ;
            rdfs:label "Plan"@en .
            :Develop rdf:type proc:Activity ;
            rdfs:label "Develop"@en .
            :Test rdf:type proc:Activity ;
            rdfs:label "Test"@en .
            :Release rdf:type proc:Activity ;
            rdfs:label "Release"@en .
            :Ship rdf:type proc:Activity ;
            rdfs:label "Ship"@en .

            :ProductManagement proc:SeriesOf proc:Planning .
            :SoftwareDevelopment proc:SeriesOf proc:Develop .
            :SoftwareDevelopment proc:SeriesOf proc:Test .
            :SoftwareDevelopment proc:SeriesOf proc:Release .
            :ProductManagement proc:SeriesOf proc:Planning .
        "##;

        let td = harriet::TurtleDocument::parse_full(turtle).unwrap();

        let o: Ontology = td.into();

        assert_eq!(
            o.iri.to_string(),
            "http://field33.com/ontologies/@fld33/process/"
        );

        assert_eq!(o.declarations().len(), 0);
        assert_eq!(o.axioms().len(), 34);
    }
}
