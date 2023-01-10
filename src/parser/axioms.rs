use super::collector::CollectedBlankNode;
use super::collector::MatcherHandler;
use crate::error::Error;
use crate::get_vars;
use crate::owl::AnnotationPropertyDomain;
use crate::owl::AnnotationPropertyRange;
use crate::owl::ClassAssertion;
use crate::owl::ClassConstructor;
use crate::owl::DataPropertyDomain;
use crate::owl::DataPropertyRange;
use crate::owl::EquivalentClasses;
use crate::owl::ObjectPropertyDomain;
use crate::owl::ObjectPropertyRange;
use crate::owl::SubAnnotationPropertyOf;
use crate::owl::SubClassOf;
use crate::owl::SubDataPropertyOf;
use crate::owl::SubObjectPropertyOf;
use crate::owl::IRI;
use crate::parser::matcher::RdfMatcher;
use crate::parser::matcher::Value;
use crate::rdf_match;
use std::collections::HashMap;
use std::ops::Deref;

pub(crate) fn match_axioms(
    matchers: &mut Vec<(RdfMatcher, MatcherHandler)>,
    prefixes: &HashMap<String, String>,
) -> Result<(), Error> {
    matchers.push((
        rdf_match!("ClassAssertions", prefixes,
            [*:x] [rdf:type] [*:cls] .
        )?,
        Box::new(|mstate, o, options| {
            if let Some(individual_iri) = mstate.last_iri("x") {
                if let Some(cls) = mstate.last_iri("cls") {
                    let individual_iri = IRI::new(individual_iri)?;
                    let cls = IRI::new(cls)?;
                    if o.class_declaration(&cls).is_some() || options.is_class(&cls) {
                        o.push_axiom(
                            ClassAssertion::new(cls.into(), individual_iri.into(), vec![]).into(),
                        )
                    }
                }
            }

            Ok(false)
        }),
    ));
    matchers.push((
        rdf_match!("SubClassOf", prefixes,
            [:x] [rdfs:subClassOf] [:object] .
        )?,
        Box::new(|mstate, o, _| {
            if let Some(vars) = get_vars!(mstate, x, object) {
                match vars.x {
                    Value::Iri(subject_iri_str) => match vars.object {
                        Value::Iri(object_iri_str) => {
                            o.push_axiom(
                                SubClassOf::new(
                                    ClassConstructor::IRI(IRI::new(subject_iri_str)?.into()).into(),
                                    ClassConstructor::IRI(IRI::new(object_iri_str)?.into()).into(),
                                    vec![],
                                )
                                .into(),
                            );
                        }
                        Value::Blank(bn) => {
                            if let Some(CollectedBlankNode::ClassConstructor(constr)) =
                                o.get_blank(bn)
                            {
                                o.push_axiom(
                                    SubClassOf::new(
                                        ClassConstructor::IRI(IRI::new(subject_iri_str)?.into())
                                            .into(),
                                        constr.clone(),
                                        vec![],
                                    )
                                    .into(),
                                );
                            }
                        }
                        Value::Literal { .. } => {
                            // TODO
                        }
                    },
                    Value::Blank(_) => {
                        // TODO
                    }
                    Value::Literal { .. } => {
                        // TODO
                    }
                }
            }
            Ok(false)
        }),
    ));
    matchers.push((
        rdf_match!("EquivalentClasses", prefixes,
            [:subject] [owl:equivalentClass] [:object] .
        )?,
        Box::new(|mstate, o, _| {
            if let Some(vars) = get_vars!(mstate, subject, object) {
                match vars.subject {
                    Value::Iri(subject_iri_str) => match vars.object {
                        Value::Iri(object_iri_str) => {
                            o.push_axiom(
                                EquivalentClasses::new(
                                    IRI::new(subject_iri_str)?.into(),
                                    IRI::new(object_iri_str)?.into(),
                                    vec![],
                                )
                                .into(),
                            );
                        }
                        Value::Blank(_) => {
                            // TODO
                        }
                        Value::Literal { .. } => {
                            // TODO
                        }
                    },
                    Value::Blank(_) => {
                        // TODO
                    }
                    Value::Literal { .. } => {
                        // TODO
                    }
                }
            }
            Ok(false)
        }),
    ));

    matchers.push((
        rdf_match!("PropertyDomain", prefixes,
            [iob:subject] [rdfs:domain] [iob:object] .
        )?,
        Box::new(|mstate, o, options| {
            if let Some(vars) = get_vars!(mstate, subject, object) {
                match vars.subject {
                    Value::Iri(subject_iri_str) => {
                        if let Ok(op_iri) = IRI::new(subject_iri_str) {
                            match vars.object {
                                Value::Iri(object_iri_str) => {
                                    if let Ok(class_iri) = IRI::new(object_iri_str) {
                                        if o.data_property_declaration(&op_iri).is_some()
                                            || options.is_data_prop(&op_iri)
                                        {
                                            o.push_axiom(
                                                DataPropertyDomain::new(
                                                    op_iri.into(),
                                                    class_iri.into(),
                                                    vec![],
                                                )
                                                .into(),
                                            );
                                        } else if o.object_property_declaration(&op_iri).is_some()
                                            || options.is_object_prop(&op_iri)
                                        {
                                            o.push_axiom(
                                                ObjectPropertyDomain::new(
                                                    op_iri.into(),
                                                    class_iri.into(),
                                                    vec![],
                                                )
                                                .into(),
                                            );
                                        } else if o
                                            .annotation_property_declaration(&op_iri)
                                            .is_some()
                                            || options.is_annotation_prop(&op_iri)
                                        {
                                            o.push_axiom(
                                                AnnotationPropertyDomain::new(
                                                    op_iri.into(),
                                                    class_iri.into(),
                                                    vec![],
                                                )
                                                .into(),
                                            );
                                        }
                                    }
                                }
                                Value::Blank(bn) => {
                                    if let Some(CollectedBlankNode::ClassConstructor(cc)) =
                                        o.get_blank(bn)
                                    {
                                        if o.data_property_declaration(&op_iri).is_some()
                                            || options.is_data_prop(&op_iri)
                                        {
                                            o.push_axiom(
                                                DataPropertyDomain::new(
                                                    op_iri.into(),
                                                    cc.deref().clone(),
                                                    vec![],
                                                )
                                                .into(),
                                            );
                                        } else if o.object_property_declaration(&op_iri).is_some()
                                            || options.is_object_prop(&op_iri)
                                        {
                                            o.push_axiom(
                                                ObjectPropertyDomain::new(
                                                    op_iri.into(),
                                                    cc.deref().clone(),
                                                    vec![],
                                                )
                                                .into(),
                                            );
                                        }
                                    }
                                }
                                Value::Literal { .. } => {
                                    // TODO
                                }
                            }
                        }
                    }
                    Value::Blank(_) => {
                        // TODO
                    }
                    Value::Literal { .. } => {
                        // TODO
                    }
                }
            }
            Ok(false)
        }),
    ));

    matchers.push((
        rdf_match!("PropertyRange", prefixes,
            [iob:subject] [rdfs:range] [iob:object] .
        )?,
        Box::new(|mstate, o, options| {
            if let Some(vars) = get_vars!(mstate, subject, object) {
                match vars.subject {
                    Value::Iri(subject_iri_str) => {
                        if let Ok(op_iri) = IRI::new(subject_iri_str) {
                            match vars.object {
                                Value::Iri(object_iri_str) => {
                                    if let Ok(class_iri) = IRI::new(object_iri_str) {
                                        if o.data_property_declaration(&op_iri).is_some()
                                            || options.is_data_prop(&op_iri)
                                        {
                                            o.push_axiom(
                                                DataPropertyRange::new(
                                                    op_iri.into(),
                                                    class_iri.into(),
                                                    vec![],
                                                )
                                                .into(),
                                            );
                                        } else if o.object_property_declaration(&op_iri).is_some()
                                            || options.is_object_prop(&op_iri)
                                        {
                                            o.push_axiom(
                                                ObjectPropertyRange::new(
                                                    op_iri.into(),
                                                    class_iri.into(),
                                                    vec![],
                                                )
                                                .into(),
                                            );
                                        } else if o
                                            .annotation_property_declaration(&op_iri)
                                            .is_some()
                                            || options.is_annotation_prop(&op_iri)
                                        {
                                            o.push_axiom(
                                                AnnotationPropertyRange::new(
                                                    op_iri.into(),
                                                    class_iri.into(),
                                                    vec![],
                                                )
                                                .into(),
                                            );
                                        }
                                    }
                                }
                                Value::Blank(bn) => {
                                    if let Some(cbn) = o.get_blank(bn) {
                                        println!("cbn {:?}", cbn);
                                        if let CollectedBlankNode::ClassConstructor(cc) = cbn {
                                            if o.object_property_declaration(&op_iri).is_some()
                                                || options.is_object_prop(&op_iri)
                                            {
                                                o.push_axiom(
                                                    ObjectPropertyRange::new(
                                                        op_iri.into(),
                                                        cc.deref().clone(),
                                                        vec![],
                                                    )
                                                    .into(),
                                                );
                                            }
                                        }
                                    }
                                }
                                Value::Literal { .. } => {
                                    // TODO
                                }
                            }
                        }
                    }
                    Value::Blank(_) => {
                        // TODO
                    }
                    Value::Literal { .. } => {
                        // TODO
                    }
                }
            }
            Ok(false)
        }),
    ));

    matchers.push((
        rdf_match!("SubProperty", prefixes,
            [iob:subject] [rdfs:subPropertyOf] [iob:object] .
        )?,
        Box::new(|mstate, o, options| {
            if let Some(vars) = get_vars!(mstate, subject, object) {
                match vars.subject {
                    Value::Iri(subject_iri_str) => match vars.object {
                        Value::Iri(object_iri_str) => {
                            if let Ok(subject) = IRI::new(subject_iri_str) {
                                if let Ok(object) = IRI::new(object_iri_str) {
                                    let subject_is_anno_prop =
                                        o.annotation_property_declaration(&subject).is_some()
                                            || options.is_annotation_prop(&subject);
                                    let subject_is_object_prop =
                                        o.object_property_declaration(&subject).is_some()
                                            || options.is_object_prop(&subject);
                                    let subject_is_data_prop =
                                        o.data_property_declaration(&subject).is_some()
                                            || options.is_data_prop(&subject);

                                    let object_is_anno_prop =
                                        o.annotation_property_declaration(&object).is_some()
                                            || options.is_annotation_prop(&object);
                                    let object_is_object_prop =
                                        o.object_property_declaration(&object).is_some()
                                            || options.is_object_prop(&object);
                                    let object_is_data_prop =
                                        o.data_property_declaration(&object).is_some()
                                            || options.is_data_prop(&object);

                                    if subject_is_anno_prop && object_is_anno_prop {
                                        o.push_axiom(
                                            SubAnnotationPropertyOf::new(
                                                subject.into(),
                                                object.into(),
                                                vec![],
                                            )
                                            .into(),
                                        );
                                    } else if subject_is_data_prop && object_is_data_prop {
                                        o.push_axiom(
                                            SubDataPropertyOf::new(
                                                subject.into(),
                                                object.into(),
                                                vec![],
                                            )
                                            .into(),
                                        );
                                    } else if subject_is_object_prop && object_is_object_prop {
                                        o.push_axiom(
                                            SubObjectPropertyOf::new(
                                                crate::owl::ObjectPropertyConstructor::IRI(
                                                    subject.into(),
                                                ),
                                                object.into(),
                                                vec![],
                                            )
                                            .into(),
                                        );
                                    }
                                }
                            }
                        }
                        Value::Blank(_) => {
                            // TODO: Add support for complex object property constructors
                        }
                        Value::Literal { .. } => {
                            // TODO
                        }
                    },
                    Value::Blank(_) => {
                        // TODO
                    }
                    Value::Literal { .. } => {
                        // TODO
                    }
                }
            }
            Ok(false)
        }),
    ));

    Ok(())
}
