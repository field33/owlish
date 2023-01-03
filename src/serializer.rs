use std::{cmp::Ordering, collections::HashMap};

use crate::{
    api::Ontology,
    owl::{
        well_known, AnnotationPropertyIRI, ClassConstructor, ClassIRI, DataPropertyIRI,
        DatatypeIRI, IndividualIRI, Literal, LiteralOrIRI, ObjectPropertyConstructor,
        ObjectPropertyIRI, IRI,
    },
};

pub trait ToTtl {
    fn ttl(&self) -> String;
}

enum Triple {
    T(String, String, String),
    Comment(String),
    LB,
}

fn t(s: String, p: String, o: String) -> Triple {
    Triple::T(s, p, o)
}

impl ToTtl for Ontology {
    fn ttl(&self) -> String {
        let mut triples: Vec<Triple> = Vec::new();

        let mut sorted_imports: Vec<(&String, &IRI)> = self.imports.iter().collect();
        sorted_imports.sort_by(|a, b| {
            #[allow(clippy::comparison_chain)]
            if a.0.len() > b.0.len() {
                Ordering::Greater
            } else if a.0.len() == b.0.len() {
                a.0.cmp(b.0)
            } else {
                Ordering::Less
            }
        });

        for (pre, iri) in sorted_imports {
            triples.push(t(
                "@prefix".into(),
                format!("{}:", pre),
                iri.ttl(&Default::default()),
            ));
        }

        triples.push(Triple::LB);

        triples.push(t(
            self.iri.ttl(&Default::default()),
            well_known::rdf_type().ttl(&self.imports),
            well_known::owl_Ontology().ttl(&self.imports),
        ));

        triples.push(Triple::LB);
        triples.push(Triple::Comment("#### Declarations #####".into()));
        triples.push(Triple::LB);

        let imports = &self.imports;

        for d in self.declarations() {
            match d {
                crate::owl::Declaration::Class {
                    iri,
                    annotations: _,
                } => triples.push(t(
                    iri.ttl(imports),
                    well_known::rdf_type().ttl(imports),
                    well_known::owl_Class().ttl(imports),
                )),
                crate::owl::Declaration::NamedIndividual {
                    iri,
                    annotations: _,
                } => triples.push(t(
                    iri.ttl(imports),
                    well_known::rdf_type().ttl(imports),
                    well_known::owl_NamedIndividual().ttl(imports),
                )),
                crate::owl::Declaration::ObjectProperty {
                    iri,
                    annotations: _,
                } => triples.push(t(
                    iri.ttl(imports),
                    well_known::rdf_type().ttl(imports),
                    well_known::owl_ObjectProperty().ttl(imports),
                )),
                crate::owl::Declaration::DataProperty {
                    iri,
                    annotations: _,
                } => triples.push(t(
                    iri.ttl(imports),
                    well_known::rdf_type().ttl(imports),
                    well_known::owl_DatatypeProperty().ttl(imports),
                )),
                crate::owl::Declaration::AnnotationProperty {
                    iri,
                    annotations: _,
                } => triples.push(t(
                    iri.ttl(imports),
                    well_known::rdf_type().ttl(imports),
                    well_known::owl_AnnotationProperty().ttl(imports),
                )),
                crate::owl::Declaration::Datatype {
                    iri,
                    annotations: _,
                } => triples.push(t(
                    iri.ttl(imports),
                    well_known::rdf_type().ttl(imports),
                    well_known::owl_Datatype().ttl(imports),
                )),
            }
        }

        let mut class_assertions: Vec<(Triple, Vec<Triple>)> = Vec::new();
        let mut sub_class_ofs: Vec<(Triple, Vec<Triple>)> = Vec::new();

        let mut anno_prop_assertions: Vec<Triple> = Vec::new();
        let mut anno_prop_domains_ranges: Vec<Triple> = Vec::new();

        let mut data_prop_assertions: Vec<Triple> = Vec::new();
        let mut data_prop_domains_ranges: Vec<Triple> = Vec::new();

        let mut obj_prop_assertions: Vec<Triple> = Vec::new();
        let mut obj_prop_domains_ranges: Vec<Triple> = Vec::new();

        for a in self.axioms() {
            match a {
                crate::owl::Axiom::AnnotationAssertion(a) => {
                    anno_prop_assertions.push(t(
                        a.subject.ttl(imports),
                        a.iri.ttl(imports),
                        a.value.ttl(imports),
                    ));
                }
                crate::owl::Axiom::DataPropertyAssertion(d) => {
                    data_prop_assertions.push(t(
                        d.subject.ttl(imports),
                        d.iri.ttl(imports),
                        d.value.ttl(imports),
                    ));
                }
                crate::owl::Axiom::ClassAssertion(c) => {
                    let (blank_node, triples) = class_triples(&c.cls, imports, 1);
                    class_assertions.push((
                        t(
                            c.individual.ttl(imports),
                            well_known::rdf_type().ttl(imports),
                            blank_node,
                        ),
                        triples,
                    ));
                }
                crate::owl::Axiom::ObjectPropertyAssertion(o) => {
                    obj_prop_assertions.push(t(
                        o.subject.ttl(imports),
                        o.iri.ttl(imports),
                        o.object.ttl(imports),
                    ));
                }
                crate::owl::Axiom::AnnotationPropertyRange(a) => {
                    anno_prop_domains_ranges.push(t(
                        a.iri.ttl(imports),
                        well_known::rdfs_range().ttl(imports),
                        a.datatype_iri.ttl(imports),
                    ));
                }
                crate::owl::Axiom::AnnotationPropertyDomain(a) => {
                    anno_prop_domains_ranges.push(t(
                        a.iri.ttl(imports),
                        well_known::rdfs_range().ttl(imports),
                        a.class_iri.ttl(imports),
                    ));
                }

                crate::owl::Axiom::DataPropertyDomain(a) => {
                    data_prop_domains_ranges.push(t(
                        a.iri.ttl(imports),
                        well_known::rdfs_domain().ttl(imports),
                        class_triples(&a.cls, imports, 1).0,
                    ));
                }
                crate::owl::Axiom::DataPropertyRange(a) => {
                    data_prop_domains_ranges.push(t(
                        a.iri.ttl(imports),
                        well_known::rdfs_range().ttl(imports),
                        a.datatype_iri.ttl(imports),
                    ));
                }

                crate::owl::Axiom::ObjectPropertyDomain(a) => {
                    obj_prop_domains_ranges.push(t(
                        a.iri.ttl(imports),
                        well_known::rdfs_domain().ttl(imports),
                        class_triples(&a.cls, imports, 1).0,
                    ));
                }
                crate::owl::Axiom::ObjectPropertyRange(a) => {
                    obj_prop_domains_ranges.push(t(
                        a.iri.ttl(imports),
                        well_known::rdfs_range().ttl(imports),
                        class_triples(&a.cls, imports, 1).0,
                    ));
                }

                crate::owl::Axiom::SubClassOf(sco) => {
                    let mut context = Vec::new();
                    let (cls, extra) = class_triples(&sco.cls, imports, 1);
                    for t in extra {
                        context.push(t);
                    }
                    let (pcls, extra) = class_triples(&sco.parent_class, imports, 1);
                    for t in extra {
                        context.push(t);
                    }
                    let subclass = well_known::rdfs_subClassOf().ttl(imports);
                    sub_class_ofs.push((t(cls, subclass, pcls), context));
                }

                crate::owl::Axiom::SubObjectPropertyOf(_) => {}
                crate::owl::Axiom::SubDataPropertyOf(_) => {}
                crate::owl::Axiom::SubAnnotationPropertyOf(_) => {}
                crate::owl::Axiom::EquivalentObjectProperties(_) => {}
                crate::owl::Axiom::EquivalentDataProperties(_) => {}
                crate::owl::Axiom::InverseObjectProperties(_) => {}
                crate::owl::Axiom::DisjointObjectProperties(_) => {}
                crate::owl::Axiom::SymmetricObjectProperty(_) => {}
                crate::owl::Axiom::AsymmetricObjectProperty(_) => {}
                crate::owl::Axiom::ReflexiveObjectProperty(_) => {}
                crate::owl::Axiom::IrreflexiveObjectProperty(_) => {}
                crate::owl::Axiom::FunctionalObjectProperty(_) => {}
                crate::owl::Axiom::InverseFunctionalObjectProperty(_) => {}
                crate::owl::Axiom::TransitiveObjectProperty(_) => {}
                crate::owl::Axiom::FunctionalDataProperty(_) => {}
                crate::owl::Axiom::EquivalentClasses(_) => {}
                crate::owl::Axiom::DisjointClasses(_) => {}
                crate::owl::Axiom::DatatypeDefinition(_) => {}
                crate::owl::Axiom::SameIndividual(_) => {}
                crate::owl::Axiom::DifferentIndividuals(_) => {}
                crate::owl::Axiom::NegativeObjectPropertyAssertion(_) => {}
                crate::owl::Axiom::NegativeDataPropertyAssertion(_) => {}
                crate::owl::Axiom::HasKey(_) => {}
            }
        }

        if !anno_prop_domains_ranges.is_empty() {
            triples.push(Triple::LB);
            triples.push(Triple::Comment("#### AnnotationProperties #####".into()));
            triples.push(Triple::LB);

            for t in anno_prop_domains_ranges {
                triples.push(t);
            }
        }

        if !data_prop_domains_ranges.is_empty() {
            triples.push(Triple::LB);
            triples.push(Triple::Comment("#### DataProperties #####".into()));
            triples.push(Triple::LB);

            for t in data_prop_domains_ranges {
                triples.push(t);
            }
        }

        if !obj_prop_domains_ranges.is_empty() {
            triples.push(Triple::LB);
            triples.push(Triple::Comment("#### ObjectProperties #####".into()));
            triples.push(Triple::LB);

            for t in obj_prop_domains_ranges {
                triples.push(t);
            }
        }

        triples.push(Triple::LB);
        triples.push(Triple::Comment("#### ClassAssertions #####".into()));
        triples.push(Triple::LB);

        for (t, context) in class_assertions {
            triples.push(t);
            for t in context {
                triples.push(t);
            }
        }
        for (t, context) in sub_class_ofs {
            triples.push(t);
            for t in context {
                triples.push(t);
            }
        }

        triples.push(Triple::LB);
        triples.push(Triple::Comment("#### AnnotationAssertions #####".into()));
        triples.push(Triple::LB);

        for t in anno_prop_assertions {
            triples.push(t);
        }

        triples.push(Triple::LB);
        triples.push(Triple::Comment("#### DataPropertyAssertions #####".into()));
        triples.push(Triple::LB);

        for t in data_prop_assertions {
            triples.push(t);
        }

        triples.push(Triple::LB);
        triples.push(Triple::Comment(
            "#### ObjectPropertyAssertions #####".into(),
        ));
        triples.push(Triple::LB);

        for t in obj_prop_assertions {
            triples.push(t);
        }

        let mut ttl = "".to_string();
        for triple in triples {
            match triple {
                Triple::T(s, p, o) => {
                    ttl = format!("{}{} {} {} . \n", ttl, s, p, o);
                }
                Triple::Comment(c) => {
                    ttl = format!("{}#{}\n", ttl, c);
                }
                Triple::LB => ttl = format!("{}\n", ttl),
            }
        }
        ttl
    }
}

fn indentation(level: usize) -> String {
    String::from_utf8(vec![b' '; level * 4]).unwrap()
}

// fn bn() -> String {
//     format!("_:{}", uuid::Uuid::new_v4())
// }

pub trait IriToTtl {
    fn ttl(&self, imports: &HashMap<String, IRI>) -> String;
}

impl IriToTtl for Literal {
    fn ttl(&self, imports: &HashMap<String, IRI>) -> String {
        match self {
            Literal::Raw { data, type_iri } => {
                format!("\"{:?}\"^^{}", data, type_iri.ttl(imports))
            }
            Literal::String(s) => format!("\"{}\"", s),
            Literal::DateTime(d) => {
                format!("\"{}\"^^{}", d, well_known::xsd_dateTime().ttl(imports))
            }
            Literal::LangString { string, lang } => format!("\"{}\"@{}", string, lang),
            Literal::Number { number, type_iri } => match type_iri {
                Some(type_iri) => format!("\"{}\"^^{}", number, type_iri.ttl(imports)),
                None => format!("{}", number),
            },
            Literal::Bool(b) => format!("{}", b),
        }
    }
}

impl IriToTtl for LiteralOrIRI {
    fn ttl(&self, imports: &HashMap<String, IRI>) -> String {
        match self {
            LiteralOrIRI::IRI(iri) => iri.ttl(imports),
            LiteralOrIRI::Literal(l) => l.ttl(imports),
        }
    }
}

impl IriToTtl for IRI {
    fn ttl(&self, imports: &HashMap<String, IRI>) -> String {
        let s = self.to_string();
        for (prefix, prefix_iri) in imports {
            let p = prefix_iri.as_str();
            if s.starts_with(p) {
                return format!("{}:{}", prefix, s.replace(p, ""));
            }
        }
        format!("<{}>", self.as_str())
    }
}
impl IriToTtl for AnnotationPropertyIRI {
    fn ttl(&self, imports: &HashMap<String, IRI>) -> String {
        self.as_iri().ttl(imports)
    }
}
impl IriToTtl for ClassIRI {
    fn ttl(&self, imports: &HashMap<String, IRI>) -> String {
        self.as_iri().ttl(imports)
    }
}
impl IriToTtl for DatatypeIRI {
    fn ttl(&self, imports: &HashMap<String, IRI>) -> String {
        self.as_iri().ttl(imports)
    }
}
impl IriToTtl for IndividualIRI {
    fn ttl(&self, imports: &HashMap<String, IRI>) -> String {
        self.as_iri().ttl(imports)
    }
}
impl IriToTtl for DataPropertyIRI {
    fn ttl(&self, imports: &HashMap<String, IRI>) -> String {
        self.as_iri().ttl(imports)
    }
}
impl IriToTtl for ObjectPropertyIRI {
    fn ttl(&self, imports: &HashMap<String, IRI>) -> String {
        self.as_iri().ttl(imports)
    }
}

fn property_triples(
    prop: &ObjectPropertyConstructor,
    imports: &HashMap<String, IRI>,
) -> (String, Vec<Triple>) {
    match prop {
        ObjectPropertyConstructor::IRI(iri) => (iri.ttl(imports), Vec::new()),
        ObjectPropertyConstructor::ObjectInverseOf(inv) => {
            let inverse_of = well_known::owl_inverseOf().ttl(imports);
            (
                format!("[ {inverse_of} {} ]", inv.0.ttl(imports)),
                Vec::new(),
            )
        }
        ObjectPropertyConstructor::ObjectPropertyChain(_) => todo!(),
    }
}

fn class_triples(
    cls: &ClassConstructor,
    imports: &HashMap<String, IRI>,
    level: usize,
) -> (String, Vec<Triple>) {
    let indent_sub1 = indentation(0.max(level - 1));
    let indent = indentation(level);
    match cls {
        ClassConstructor::IRI(iri) => (iri.ttl(imports), Vec::new()),
        ClassConstructor::ObjectIntersectionOf(inter) => {
            // let root_bn = bn();
            // let mut context = Vec::new();
            // context.push(t(
            //     root_bn.clone(),
            //     well_known::rdf_type().ttl(imports),
            //     well_known::owl_Class().ttl(imports),
            // ));

            // let first_bn = bn();
            // context.push(t(
            //     root_bn.clone(),
            //     well_known::owl_intersectionOf().ttl(imports),
            //     first_bn.clone(),
            // ));
            // let mut next_bn = first_bn.clone();
            // for cls in &inter.classes {
            //     let (object, ctx) = class_triples(cls, imports);

            //     // add rest part of former iteration (the last one will have no rest)
            //     if next_bn != first_bn {
            //         context.push(t(
            //             next_bn.clone(),
            //             well_known::rdf_rest().ttl(imports),
            //             object.clone(),
            //         ));
            //     }

            //     // add first of this iteration
            //     context.push(t(
            //         next_bn.clone(),
            //         well_known::rdf_first().ttl(imports),
            //         object.clone(),
            //     ));
            //     next_bn = bn();
            //     for c in ctx {
            //         context.push(c);
            //     }
            // }
            // let mut s = "(".into();
            // for cls in &inter.classes {
            //     let (cls, _) = class_triples(cls, imports);
            //     s = format!("{} {}", s, cls)
            // }
            let typ = well_known::rdf_type().ttl(imports);
            let cls = well_known::owl_Class().ttl(imports);
            let owl_intersection_of = well_known::owl_intersectionOf().ttl(imports);
            (
                format!(
                    "[\n{indent}{typ} {cls} ;\n{indent}{owl_intersection_of} ({})\n{indent_sub1}]",
                    inter.classes.iter().fold(String::new(), |acc, x| {
                        let (c, _) = class_triples(x, imports, level + 1);
                        format!("{} {}", acc, c)
                    })
                ),
                Default::default(),
            )
        }
        ClassConstructor::SubClassOf(_) => todo!(),
        ClassConstructor::DataSomeValuesFrom(d) => {
            let typ = well_known::rdf_type().ttl(imports);
            let owl_restriction = well_known::owl_Restriction().ttl(imports);
            // let mut on_class = String::new();
            // let owl_cardinality = if let Some(iri) = &d.class_iri {
            //     on_class = format!(
            //         ";\n{indent}{} {}",
            //         well_known::owl_onClass().ttl(imports),
            //         iri.ttl(imports),
            //     );
            //     well_known::owl_minCardinality().ttl(imports)
            // } else {
            //     well_known::owl_minQualifiedCardinality().ttl(imports)
            // };
            // let cardinality = Literal::Number {
            //     number: d.value.into(),
            //     type_iri: well_known::xsd_nonNegativeInteger().into(),
            // }
            // .ttl(imports);
            let some_values_from = well_known::owl_someValuesFrom().ttl(imports);
            let on_prop = well_known::owl_onProperty().ttl(imports);
            let prop = d.data_property_iri.ttl(imports);
            let restriction = restriction(&d.restriction, imports, level + 1);
            (
                format!(
                    "[\n{indent}{typ} {owl_restriction} ;\n{indent}{on_prop} {prop} ;\n{indent}{some_values_from} {restriction} \n{indent_sub1}]"
                ),
                Vec::new(),
            )
        }
        ClassConstructor::EquivalentClasses(_) => todo!(),
        ClassConstructor::DisjointClasses(_) => todo!(),
        ClassConstructor::ObjectComplementOf(oco) => {
            let typ = well_known::rdf_type().ttl(imports);
            let cls = well_known::owl_Class().ttl(imports);
            let owl_complement_of = well_known::owl_complementOf().ttl(imports);

            (
                format!(
                    "[\n{indent}{typ} {cls} ;\n{indent}{owl_complement_of} {}\n{indent_sub1}]",
                    class_triples(&oco.cls, imports, level + 1).0
                ),
                Vec::new(),
            )
        }
        ClassConstructor::ObjectMaxCardinality(omc) => {
            // let root_bn = bn();
            // let mut context = Vec::new();
            // context.push(t(
            //     root_bn.clone(),
            //     well_known::rdf_type().ttl(imports),
            //     well_known::owl_Restriction().ttl(imports),
            // ));

            // context.push(t(
            //     root_bn.clone(),
            //     well_known::owl_onProperty().ttl(imports),
            //     omc.object_property_iri.ttl(imports),
            // ));

            // if let Some(iri) = &omc.class_iri {
            //     context.push(t(
            //         root_bn.clone(),
            //         well_known::owl_maxQualifiedCardinality().ttl(imports),
            //         Literal::Number {
            //             number: omc.value.into(),
            //             type_iri: well_known::xsd_nonNegativeInteger().into(),
            //         }
            //         .ttl(imports),
            //     ));
            //     context.push(t(
            //         root_bn.clone(),
            //         well_known::owl_onClass().ttl(imports),
            //         iri.ttl(imports),
            //     ));
            // } else {
            //     context.push(t(
            //         root_bn.clone(),
            //         well_known::owl_maxCardinality().ttl(imports),
            //         Literal::Number {
            //             number: omc.value.into(),
            //             type_iri: well_known::xsd_nonNegativeInteger().into(),
            //         }
            //         .ttl(imports),
            //     ));
            // }
            // (root_bn, context)
            let indent_sub1 = indentation(0.max(level - 1));
            let indent = indentation(level);
            let typ = well_known::rdf_type().ttl(imports);
            let restriction = well_known::owl_Restriction().ttl(imports);
            let mut on_class = String::new();
            let owl_cardinality = if let Some(iri) = &omc.class_iri {
                on_class = format!(
                    ";\n{indent}{} {}",
                    well_known::owl_onClass().ttl(imports),
                    iri.ttl(imports),
                );
                well_known::owl_maxCardinality().ttl(imports)
            } else {
                well_known::owl_maxQualifiedCardinality().ttl(imports)
            };
            let cardinality = Literal::Number {
                number: omc.value.into(),
                type_iri: well_known::xsd_nonNegativeInteger().into(),
            }
            .ttl(imports);
            let on_prop = well_known::owl_onProperty().ttl(imports);
            let prop = omc.object_property_iri.ttl(imports);

            (
                format!(
                    "[\n{indent}{typ} {restriction} ;\n{indent}{owl_cardinality} {cardinality} ;\n{indent}{on_prop} {prop} {on_class} \n{indent_sub1}]"
                ),
                Vec::new(),
            )
        }
        ClassConstructor::ObjectUnionOf(_) => todo!(),
        ClassConstructor::ObjectSomeValuesFrom(o) => {
            let typ = well_known::rdf_type().ttl(imports);
            let owl_restriction = well_known::owl_Restriction().ttl(imports);
            let owl_some_values_from = well_known::owl_someValuesFrom().ttl(imports);
            let on_property = well_known::owl_onProperty().ttl(imports);
            (
                format!(
                    "[\n{indent}{typ} {owl_restriction} ;\n{indent}{on_property} {} ;\n{indent}{owl_some_values_from} {}\n{indent_sub1}]",
                    property_triples(&o.object_property, imports).0,
                    o.class_iri.ttl(imports)
                ),
                Vec::new(),
            )
        }
        ClassConstructor::ObjectMinCardinality(omc) => {
            let typ = well_known::rdf_type().ttl(imports);
            let restriction = well_known::owl_Restriction().ttl(imports);
            let mut on_class = String::new();
            let owl_cardinality = if let Some(iri) = &omc.class_iri {
                on_class = format!(
                    ";\n{indent}{} {}",
                    well_known::owl_onClass().ttl(imports),
                    iri.ttl(imports),
                );
                well_known::owl_minCardinality().ttl(imports)
            } else {
                well_known::owl_minQualifiedCardinality().ttl(imports)
            };
            let cardinality = Literal::Number {
                number: omc.value.into(),
                type_iri: well_known::xsd_nonNegativeInteger().into(),
            }
            .ttl(imports);
            let on_prop = well_known::owl_onProperty().ttl(imports);
            let prop = omc.object_property_iri.ttl(imports);

            (
                format!(
                    "[\n{indent}{typ} {restriction} ;\n{indent}{owl_cardinality} {cardinality} ;\n{indent}{on_prop} {prop} {on_class} \n{indent_sub1}]"
                ),
                Vec::new(),
            )
        }
        ClassConstructor::ObjectExactCardinality(oec) => {
            let typ = well_known::rdf_type().ttl(imports);
            let restriction = well_known::owl_Restriction().ttl(imports);
            let mut on_class = String::new();
            let owl_cardinality = if let Some(iri) = &oec.class_iri {
                on_class = format!(
                    ";\n{indent}{} {}",
                    well_known::owl_onClass().ttl(imports),
                    iri.ttl(imports),
                );
                well_known::owl_cardinality().ttl(imports)
            } else {
                well_known::owl_qualifiedCardinality().ttl(imports)
            };
            let cardinality = Literal::Number {
                number: oec.value.into(),
                type_iri: well_known::xsd_nonNegativeInteger().into(),
            }
            .ttl(imports);
            let on_prop = well_known::owl_onProperty().ttl(imports);
            let prop = oec.object_property_iri.ttl(imports);

            (
                format!(
                    "[\n{indent}{typ} {restriction} ;\n{indent}{owl_cardinality} {cardinality} ;\n{indent}{on_prop} {prop} {on_class} \n{indent_sub1}]"
                ),
                Vec::new(),
            )
        }
        ClassConstructor::ObjectAllValuesFrom(o) => {
            let typ = well_known::rdf_type().ttl(imports);
            let cls = well_known::owl_Class().ttl(imports);
            let on_prop = well_known::owl_onProperty().ttl(imports);
            let owl_all_from = well_known::owl_allValuesFrom().ttl(imports);
            (
                format!(
                    "[\n{indent}{typ} {cls} ;\n{indent}{on_prop} {} ;\n{indent}{owl_all_from} {} \n{indent_sub1}]",
                    property_triples(&o.object_property, imports).0,
                    o.class_iri.ttl(imports)
                ),
                Vec::new(),
            )
        }
        ClassConstructor::ObjectOneOf(o) => {
            let typ = well_known::rdf_type().ttl(imports);
            let cls = well_known::owl_Class().ttl(imports);
            let owl_one_of = well_known::owl_oneOf().ttl(imports);
            (
                format!(
                    "[\n{indent}{typ} {cls} ;\n{indent}{owl_one_of} ({}) \n{indent_sub1}]",
                    o.individuals.iter().fold(String::new(), |acc, x| format!(
                        "{} {}",
                        acc,
                        x.ttl(imports)
                    ))
                ),
                Vec::new(),
            )
        }
        ClassConstructor::ObjectHasValue(_) => todo!(),
        ClassConstructor::ObjectHasSelf(_) => todo!(),
    }
}

fn restriction(
    restriction: &crate::owl::DatatypeRestriction,
    imports: &HashMap<String, IRI>,
    level: usize,
) -> String {
    let indent_sub1 = indentation(0.max(level - 1));
    let indent_add1 = indentation(0.max(level + 1));
    let indent = indentation(level);
    let typ = well_known::rdf_type().ttl(imports);
    let rdfs_datatype = well_known::rdfs_Datatype().ttl(imports);
    let on_datatype = well_known::owl_onDatatype().ttl(imports);
    let datatype = restriction.datatype_iri.ttl(imports);
    let with_restrictions = well_known::owl_withRestrictions().ttl(imports);
    format!(
        "[\n{indent}{typ} {rdfs_datatype} ;\n{indent}{on_datatype} {datatype} ;\n{indent}{with_restrictions} ({}\n{indent})\n{indent_sub1}]",
        restriction.restrictions.iter().fold(String::new(), |acc, x| format!("{} {}", acc, match x {
            crate::owl::Restriction::Numeric { datatype_iri, value } => {
                format!("\n{indent_add1}[{} {}]", datatype_iri.ttl(imports), value.ttl(imports))
            },
        }))
    )
}

#[cfg(test)]
mod tests {
    use crate::owl::well_known;

    use super::ToTtl;

    const EXPECTED: &str = include_str!("expected.ttl");

    #[test]
    fn test() {
        let mut onto = crate::examples::family();
        onto.imports.insert("".into(), onto.iri.clone());
        onto.imports.insert("owl".into(), well_known::owl());
        onto.imports.insert("rdfs".into(), well_known::rdfs());
        onto.imports.insert("rdf".into(), well_known::rdf());
        onto.imports.insert("xsd".into(), well_known::xsd());
        // let mut fi = std::fs::File::create("expected.ttl").unwrap();
        // write!(fi, "{}", onto.ttl()).unwrap();
        assert_eq!(onto.ttl(), EXPECTED)
    }
}
