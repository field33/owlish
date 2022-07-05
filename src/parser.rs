use serde_json::Value;

use crate::{
    api::Ontology,
    owl::{
        well_known, AnnotationAssertion, AsymmetricObjectProperty, ClassIRI, Declaration,
        IRIBuilder, ObjectPropertyAssertion, ObjectPropertyDomain, ObjectPropertyRange, SubClassOf,
        SymmetricObjectProperty, IRI,
    },
};

impl Ontology {
    pub fn parse(ttl: &str) -> Result<Self, ()> {
        let td = harriet::TurtleDocument::parse_full(ttl).unwrap();
        Ok(td.into())
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

        for item in turtle.items {
            // parse_item(item, &mut ontology);
            item.parse(&mut ontology);
        }

        ontology
    }
}

impl<'a> Parse<'a> for harriet::Item<'a> {
    fn parse(&'a self, ontology: &mut Ontology) {
        match self {
            harriet::Item::Statement(stmnt) => stmnt.parse(ontology),
            harriet::Item::Comment(_) => {
                // ignore comments
            }
        };
    }
}

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
                // todo: do not panic
                ontology.iri = IRI::new(&base.iri.iri).unwrap()
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
            harriet::Triples::Labeled(subject, pol) => {
                let iri: Option<IRI> = match subject {
                    harriet::Subject::IRI(iri) => {
                        IRI::try_from((&ontology.iri_builder(), iri)).ok()
                    }
                    harriet::Subject::Collection(col) => {
                        todo!()
                    }
                };
                if let Some(iri) = iri {
                    for (predicate, object_list) in &pol.list {
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
                                        parse_rdf_type(ontology, &iri, &object_list.list)
                                    }
                                    (Some("rdfs"), Some("subClassOf")) => {
                                        parse_rdfs_sub_class_of(ontology, &iri, &object_list.list)
                                    }
                                    (Some("rdfs"), Some("label")) => {
                                        parse_rdfs_label(ontology, &iri, &object_list.list)
                                    }
                                    (Some("rdfs"), Some("range")) => {
                                        parse_rdfs_range(ontology, &iri, &object_list.list)
                                    }
                                    (Some("rdfs"), Some("domain")) => {
                                        parse_rdfs_domain(ontology, &iri, &object_list.list)
                                    }
                                    _ => {
                                        // TODO: ignore
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

fn parse_rdfs_range(ontology: &mut Ontology, subject_iri: &IRI, objects: &Vec<harriet::Object>) {
    let iri_builder = ontology.iri_builder();
    for obj in objects {
        match obj {
            harriet::Object::IRI(object_iri) => ontology.owl.axioms.push(
                ObjectPropertyRange(
                    subject_iri.clone().into(),
                    IRI::try_from((&iri_builder, object_iri)).unwrap().into(),
                )
                .into(),
            ),
            harriet::Object::Collection(_) => todo!(),
            harriet::Object::BlankNodePropertyList(_) => todo!(),
            harriet::Object::Literal(_) => todo!(),
        }
    }
}

fn parse_rdfs_domain(ontology: &mut Ontology, subject_iri: &IRI, objects: &Vec<harriet::Object>) {
    let iri_builder = ontology.iri_builder();
    for obj in objects {
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
        }
    }
}

fn parse_rdfs_label(ontology: &mut Ontology, subject_iri: &IRI, objects: &Vec<harriet::Object>) {
    for obj in objects {
        match obj {
            harriet::Object::Literal(label) => match label {
                harriet::Literal::RDFLiteral(lit) => ontology.owl.axioms.push(
                    AnnotationAssertion(
                        well_known::rdfs_label(),
                        subject_iri.clone(),
                        Value::from(lit.string.to_string()),
                    )
                    .into(),
                ),
                harriet::Literal::BooleanLiteral(_) => todo!(),
            },
            harriet::Object::IRI(_) => todo!(),
            harriet::Object::Collection(_) => todo!(),
            harriet::Object::BlankNodePropertyList(_) => todo!(),
        }
    }
}

fn parse_rdf_type(ontology: &mut Ontology, subject_iri: &IRI, objects: &Vec<harriet::Object>) {
    for object in objects {
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
                    }
                }
            }
            harriet::Object::Collection(_) => todo!(),
            harriet::Object::BlankNodePropertyList(_) => todo!(),
            harriet::Object::Literal(_) => todo!(),
        }
    }
}

fn parse_rdfs_sub_class_of(
    ontology: &mut Ontology,
    subject_iri: &IRI,
    objects: &Vec<harriet::Object>,
) {
    for object in objects {
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
            harriet::Object::BlankNodePropertyList(_) => todo!(),
            harriet::Object::Literal(_) => todo!(),
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
                .map(|iri| Ok(iri))
                .unwrap_or(Err(())),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::api::Ontology;

    #[test]
    fn test_parse_simple_ontology() {
        let turtle = r##"
@prefix : <http://field33.com/ontologies/@fld33/process/> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
@prefix xml: <http://www.w3.org/XML/1998/namespace> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
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
                                                 rdfs:label "Process"@en .

:Activity rdf:type owl:Class ;
    rdfs:label "Activity"@en .

:Process rdf:type owl:Class ;
    rdfs:label "Process"@en .

:SeriesOf rdf:type owl:ObjectProperty ;
    rdfs:domain :Activity ;
    rdfs:range :Process ;
    rdfs:label "Series of"@en .

"##;

        let td = harriet::TurtleDocument::parse_full(turtle).unwrap();

        let o: Ontology = td.into();

        println!("{:#?}", o);

        assert_eq!(
            o.iri.to_string(),
            "http://field33.com/ontologies/@fld33/process/"
        );

        assert_eq!(o.classes().len(), 2);
    }
}
