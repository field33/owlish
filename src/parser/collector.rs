use std::{borrow::Cow, collections::HashMap};

use harriet::triple_production::RdfBlankNode;

use crate::{api::Ontology, error::Error, owl::*, parser::matcher::Value};

use super::{matcher::MatcherState, IndexedParserOptions};

/// Handle when a matcher matched. Returns whether the matched rules where actually
pub(crate) type MatcherHandler<'a> = Box<
    dyn Fn(
        &MatcherState<'a>,
        &mut OntologyCollector<'a>,
        &IndexedParserOptions,
    ) -> Result<bool, Error>,
>;

#[derive(Debug, Clone)]
pub(crate) enum CollectedBlankNode<'a> {
    ClassConstructor(Box<ClassConstructor>),
    Sequence {
        first: Option<Value<'a>>,
        rest: Option<RdfBlankNode>,
    },
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub(crate) enum CollectedReificationKey<'a> {
    Bn(RdfBlankNode),
    Iri(Cow<'a, str>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct CollectedReification<'a> {
    pub(crate) subject: Cow<'a, str>,
    pub(crate) predicate: Cow<'a, str>,
    pub(crate) object: Cow<'a, str>,
}

#[derive(Debug, Default)]
pub(crate) struct OntologyCollector<'a> {
    iri: Option<IRI>,
    declarations: Vec<Declaration>,
    axioms: Vec<Axiom>,

    // reified triples
    reifications: HashMap<CollectedReificationKey<'a>, CollectedReification<'a>>,
    // TODO: we probably don't need both. There may be a conceptional bug
    reifications_rev: HashMap<CollectedReification<'a>, CollectedReificationKey<'a>>,

    // annotation definitions that were assigned to other things
    // (to handle multiple assertions for one annotation which is assigned to e.g. one data prop assertion)
    used_annotations: HashMap<String, Vec<usize>>,

    pub blank_nodes: HashMap<RdfBlankNode, CollectedBlankNode<'a>>,

    axiom_index: HashMap<(String, String, String), usize>,
    declaration_index: HashMap<String, Vec<usize>>,
}

impl<'a> OntologyCollector<'a> {
    pub(crate) fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub(crate) fn set_iri(&mut self, iri: IRI) {
        self.iri = Some(iri);
    }

    pub(crate) fn push_declaration(&mut self, declaration: Declaration) {
        let iri = match &declaration {
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
        if let Some(list) = self.declaration_index.get_mut(&iri.to_string()) {
            list.push(self.declarations.len());
        } else {
            self.declaration_index
                .insert(iri.to_string(), vec![self.declarations.len()]);
        }
        self.declarations.push(declaration)
    }

    pub(crate) fn push_axiom(&mut self, axiom: Axiom) {
        match &axiom {
            Axiom::SubClassOf(sco) => {
                if let ClassConstructor::IRI(subject) = sco.cls.as_ref() {
                    if let ClassConstructor::IRI(parent) = sco.parent_class.as_ref() {
                        self.axiom_index.insert(
                            (
                                subject.as_iri().to_string(),
                                well_known::rdfs_subClassOf_str.to_string(),
                                parent.as_iri().to_string(),
                            ),
                            self.axioms.len(),
                        );
                    }
                }
            }
            Axiom::AnnotationAssertion(ann) => {
                let sub = &ann.subject;
                let iri = &ann.iri;
                let val = &ann.value;
                self.axiom_index.insert(
                    (sub.to_string(), iri.to_string(), val.to_string()),
                    self.axioms.len(),
                );
            }
            Axiom::DataPropertyAssertion(ann) => {
                let sub = &ann.subject;
                let iri = &ann.iri;
                let val = &ann.value;
                self.axiom_index.insert(
                    (
                        sub.as_iri().to_string(),
                        iri.as_iri().to_string(),
                        val.to_string(),
                    ),
                    self.axioms.len(),
                );
            }
            _ => {
                // TODO
            }
        }
        self.axioms.push(axiom);
    }

    pub(crate) fn get_from_index_mut(
        &mut self,
        s: &str,
        p: &str,
        o: &str,
    ) -> Option<(&mut Axiom, usize)> {
        self.axiom_index
            .get(&(s.into(), p.into(), o.into()))
            .and_then(|index| self.axioms.get_mut(*index).map(|a| (a, *index)))
    }

    pub(crate) fn insert_blank_node(&mut self, bn: RdfBlankNode, bnh: CollectedBlankNode<'a>) {
        self.blank_nodes.insert(bn, bnh);
    }

    pub(crate) fn insert_reification(
        &mut self,
        key: CollectedReificationKey<'a>,
        value: CollectedReification<'a>,
    ) {
        self.reifications.insert(key.clone(), value.clone());
        self.reifications_rev.insert(value, key);
    }

    pub(crate) fn annotation(
        &self,
        ann: CollectedReificationKey<'a>,
    ) -> Option<&CollectedReification<'a>> {
        self.reifications.get(&ann)
    }

    pub(crate) fn annotation_on_triple(
        &self,
        ann: &CollectedReification<'a>,
    ) -> Option<&CollectedReificationKey<'a>> {
        self.reifications_rev.get(ann)
    }

    pub(crate) fn ontology(self) -> Ontology {
        let mut o = Ontology::new(self.iri.unwrap());

        for d in self.declarations {
            o.push_declaration(d)
        }
        for a in self.axioms {
            o.push_axiom(a);
        }

        o
    }

    // pub(crate) fn set_sequence_root(&mut self, root: &RdfBlankNode, value: Value<'a>) {
    //     self.sequence_tree.insert(root.clone(), None);
    //     self.sequences.insert(root.clone(), vec![value]);
    // }

    // pub(crate) fn get_sequence(&mut self, bn: &RdfBlankNode) -> Option<&mut Vec<Value<'a>>> {
    //     match self.sequence_tree.get(bn) {
    //         Some(Some(root)) => self.sequences.get_mut(root),
    //         Some(None) => self.sequences.get_mut(bn),
    //         _ => None,
    //     }
    // }

    // pub(crate) fn set_sequence_tree(
    //     &mut self,
    //     parent: &RdfBlankNode,
    //     leaf: RdfBlankNode,
    // ) -> Result<(), Error> {
    //     match self.sequence_tree.get(parent).cloned() {
    //         Some(None) => {
    //             self.sequence_tree.insert(leaf, Some(parent.clone()));
    //         }
    //         Some(Some(parent)) => {
    //             self.set_sequence_tree(&parent, leaf)?;
    //         }
    //         None => {
    //             return Err(Error::new("Failed to save sequence".into()));
    //         }
    //     }
    //     Ok(())
    // }

    pub(crate) fn get_blank(&self, bn: &RdfBlankNode) -> Option<&CollectedBlankNode> {
        self.blank_nodes.get(bn)
    }

    pub(crate) fn annotation_property_declaration(
        &self,
        iri: &IRI,
    ) -> Option<(&AnnotationPropertyIRI, &Vec<Annotation>)> {
        self.declaration_index
            .get(iri.as_str())
            .map(|indexes| {
                indexes
                    .iter()
                    .filter_map(|i| self.declarations.get(*i))
                    .collect::<Vec<&Declaration>>()
            })
            .and_then(|ds| {
                for d in ds {
                    if let Declaration::AnnotationProperty {
                        iri: a,
                        annotations,
                    } = d
                    {
                        return Some((a, annotations));
                    }
                }
                None
            })
    }

    pub(crate) fn class_declaration(&self, cls: &IRI) -> Option<&Declaration> {
        self.declaration_index
            .get(cls.as_str())
            .map(|indexes| {
                indexes
                    .iter()
                    .filter_map(|i| self.declarations.get(*i))
                    .collect::<Vec<&Declaration>>()
            })
            .and_then(|ds| {
                for d in ds {
                    if let Declaration::Class { .. } = d {
                        return Some(d);
                    }
                }
                None
            })
    }

    pub(crate) fn _individual_declaration(&self, iri: &IRI) -> Option<&Declaration> {
        self.declaration_index
            .get(iri.as_str())
            .map(|indexes| {
                indexes
                    .iter()
                    .filter_map(|i| self.declarations.get(*i))
                    .collect::<Vec<&Declaration>>()
            })
            .and_then(|ds| {
                for d in ds {
                    if let Declaration::NamedIndividual { .. } = d {
                        return Some(d);
                    }
                }
                None
            })
    }

    pub(crate) fn data_property_declaration(&self, iri: &IRI) -> Option<&Declaration> {
        self.declaration_index
            .get(iri.as_str())
            .map(|indexes| {
                indexes
                    .iter()
                    .filter_map(|i| self.declarations.get(*i))
                    .collect::<Vec<&Declaration>>()
            })
            .and_then(|ds| {
                for d in ds {
                    if let Declaration::DataProperty { .. } = d {
                        return Some(d);
                    }
                }
                None
            })
    }

    pub(crate) fn object_property_declaration(&self, iri: &IRI) -> Option<&Declaration> {
        self.declaration_index
            .get(iri.as_str())
            .map(|indexes| {
                indexes
                    .iter()
                    .filter_map(|i| self.declarations.get(*i))
                    .collect::<Vec<&Declaration>>()
            })
            .and_then(|ds| {
                for d in ds {
                    if let Declaration::ObjectProperty { .. } = d {
                        return Some(d);
                    }
                }
                None
            })
    }

    pub(crate) fn get_sequence(&self, bn: &RdfBlankNode) -> Option<Vec<Value<'a>>> {
        if let Some(CollectedBlankNode::Sequence { first, rest }) = self.blank_nodes.get(bn) {
            let mut values: Vec<Value<'a>> = Vec::new();
            if let Some(value) = first {
                values.push(value.clone());
            }
            let mut the_rest = rest;
            while let Some(r) = the_rest {
                the_rest = &None;
                if let Some(CollectedBlankNode::Sequence { first, rest }) = self.blank_nodes.get(r)
                {
                    if let Some(value) = first {
                        values.push(value.clone());
                    }
                    the_rest = rest;
                }
            }
            Some(values)
        } else {
            None
        }
    }

    // pub(crate) fn get_blank_mut(&self, bn: &RdfBlankNode) -> Option<&mut CollectedBlankNode> {
    //     self.blank_nodes.get_mut(bn)
    // }

    pub(crate) fn update_blank_node_sequence(
        &mut self,
        bn: &RdfBlankNode,
        first: Option<Value<'a>>,
        rest: Option<RdfBlankNode>,
    ) {
        if let Some(CollectedBlankNode::Sequence { first: f, rest: r }) =
            self.blank_nodes.get_mut(bn)
        {
            if let Some(first) = first {
                *f = Some(first)
            }
            if let Some(rest) = rest {
                *r = Some(rest)
            }
        }
    }

    pub(crate) fn insert_used_annotation(&mut self, anno_iri: &str, index: usize) {
        if let Some(indexes) = self.used_annotations.get_mut(anno_iri) {
            indexes.push(index)
        } else {
            self.used_annotations
                .insert(anno_iri.to_string(), vec![index]);
        }
    }
    pub(crate) fn get_used_annotation(&self, anno_iri: &str) -> Option<&Vec<usize>> {
        self.used_annotations.get(anno_iri)
    }

    pub(crate) fn axiom_mut(&mut self, i: usize) -> Option<&mut Axiom> {
        self.axioms.get_mut(i)
    }
}

pub(crate) fn get_iri_var(name: &str, mstate: &MatcherState) -> Result<Option<IRI>, Error> {
    if let Some(var) = mstate.get(name) {
        match var {
            Value::Iri(var) => {
                let iri = IRI::new(var)?;
                Ok(Some(iri))
            }
            Value::Blank(_) => Ok(None),
            Value::Literal { .. } => Ok(None),
        }
    } else {
        Ok(None)
    }
}

#[macro_export]
macro_rules! get_vars {

    (
        $mstate:ident,
        $($($variable:ident)+$(,)?)+
    ) => {{
        let mut result = true;
        $(
            let $($variable)+ = $mstate.get(stringify!($($variable)+));
            if $($variable)+.is_none() {
                result = false;
            }
            let $($variable)+ = $($variable)+.unwrap();
        )+
        struct Vars<'a> {
            $(
                $($variable)+: &'a Value<'a>,
            )+
        }
        if result {
            // Some((
                // ))
            Some(Vars {
                $(
                    $($variable)+,
                )+
            })
        } else {
            None
        }
    }};
}
