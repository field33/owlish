use crate::owl::ClassConstructor;

use crate::owl::IRI;

use crate::owl::ObjectIntersectionOf;
use crate::parser::matcher::Value;

use crate::rdf_match;

use crate::error::Error;

use std::collections::HashMap;

use crate::parser::matcher::RdfMatcher;

use super::collector::BlankNodeHandle;
use super::collector::MatcherHandler;

pub(crate) fn match_blank_nodes(
    matchers: &mut Vec<(RdfMatcher, MatcherHandler)>,
    prefixes: &HashMap<String, String>,
) -> Result<(), Error> {
    // TODO: parse all kinds of blank nodes to something like `Map<blank_node, AxiomOrDecl>`.

    // annotations on things
    matchers.push((
        rdf_match!("Annotation", prefixes,
            [:subject] [*:predicate] [:object] .
            [_:a] [rdf:type] [owl:Axiom] .
            [_:a] [owl:annotatedSource] [:subject] .
            [_:a] [owl:annotatedProperty] [*:predicate] .
            [_:a] [owl:annotatedTarget] [:object] .
        )?,
        Box::new(|mstate, o| {
            if let Some(bn) = mstate.last("a") {
                match bn {
                    Value::Blank(bn) => {
                        if let Some(Value::Iri(subject)) = mstate.last("subject") {
                            if let Some(Value::Iri(predicate)) = mstate.last("predicate") {
                                match mstate.last("object") {
                                    Some(Value::Iri(object)) => {
                                        o.insert_blank_node(
                                            bn.clone(),
                                            BlankNodeHandle::Annotate {
                                                subject: subject.clone(),
                                                predicate: predicate.clone(),
                                                object: object.clone(),
                                            },
                                        );
                                    }
                                    Some(Value::Literal {
                                        lexical_form,
                                        datatype_iri: _,
                                        language_tag: _,
                                    }) => {
                                        o.insert_blank_node(
                                            bn.clone(),
                                            BlankNodeHandle::Annotate {
                                                subject: subject.clone(),
                                                predicate: predicate.clone(),
                                                object: lexical_form.clone(),
                                            },
                                        );
                                    }
                                    _ => todo!(),
                                }
                            }
                        }
                    }
                    _ => todo!(),
                }
            }

            // println!("TEST");
            // if o.axioms().is_empty() {
            //     return Ok(false);
            // }
            // if let Some(subject) = mstate.last_iri("subject") {
            // if let Some(annotation_iri) = mstate.last_iri("annotation") {
            //     if let Some(literal) = mstate.get("value") {
            // let annotation_iri = IRI::new(annotation_iri)?;
            // let literal: LiteralOrIRI = match literal {
            //     Value::Iri(iri) => LiteralOrIRI::IRI(IRI::new(iri)?),
            //     Value::Blank(_) => return Ok(false),
            //     Value::Literal {
            //         lexical_form,
            //         datatype_iri: _,
            //         language_tag: _,
            //     } => LiteralOrIRI::Literal(Literal::String(lexical_form.clone())),
            // };
            // let subject: ClassConstructor = IRI::new(subject)?.into();

            // let last_axiom = o.axioms().len() - 1;
            // if let Some(axiom) = o.axiom_mut(last_axiom) {
            //     // if the annotated axiom is the last one take a shortcut
            //     if let Some(annotations) = is_axiom(axiom, &subject) {
            //         annotations.push(Annotation(annotation_iri.into(), literal, vec![]));
            //         return Ok(true);
            //     }
            // } else {
            //     // Otherwise search for it.
            //     for a in o.axioms_mut().iter_mut() {
            //         if let Some(annotations) = is_axiom(a, &subject) {
            //             annotations.push(Annotation(annotation_iri.into(), literal, vec![]));
            //             return Ok(true);
            //         }
            //     }
            // }
            //     }
            // }
            // }
            Ok(false)
        }),
    ));

    // ObjectIntersectionOf
    matchers.push((
        rdf_match!("ObjectIntersectionOf", prefixes,
            [_:x] [rdf:type] [owl:Class] .
            [_:x] [owl:intersectionOf] [:object] .
        )?,
        Box::new(|mstate, o| {
            if let Some(Value::Blank(bn)) = mstate.get("x") {
                if let Some(Value::Blank(obj)) = mstate.get("object") {
                    if let Some(seq) = o.get_sequence(obj) {
                        if seq.iter().all(|v| matches!(v, Value::Iri(_))) {
                            let mut classes = Vec::new();
                            for v in seq {
                                if let Value::Iri(iri) = v {
                                    let iri = IRI::new(iri)?;
                                    classes.push(ClassConstructor::IRI(iri.into()));
                                }
                            }
                            o.insert_blank_node(
                                bn.clone(),
                                BlankNodeHandle::ClassConstructor(Box::new(
                                    ClassConstructor::ObjectIntersectionOf(ObjectIntersectionOf(
                                        classes,
                                        vec![],
                                    )),
                                )),
                            );
                            return Ok(true);
                        } else {
                            todo!("support deeper nested blank nodes")
                        }
                    } else {
                        return Err(Error::new("Could not find referenced sequence".into()));
                    }
                }
            }
            Ok(false)
        }),
    ));
    Ok(())
}
