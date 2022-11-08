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
pub(crate) enum CollectedBlankNode {
    ClassConstructor(Box<ClassConstructor>),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub(crate) enum CollectedAnnotationKey<'a> {
    Bn(RdfBlankNode),
    Iri(Cow<'a, str>),
}

#[derive(Debug, Clone)]
pub(crate) struct CollectedAnnotation<'a> {
    pub(crate) subject: Cow<'a, str>,
    pub(crate) predicate: Cow<'a, str>,
    pub(crate) object: Cow<'a, str>,
}

#[derive(Debug, Default)]
pub(crate) struct OntologyCollector<'a> {
    iri: Option<IRI>,
    declarations: Vec<Declaration>,
    axioms: Vec<Axiom>,
    // blank node -> IRI
    sequences: HashMap<RdfBlankNode, Vec<Value<'a>>>,
    // child node -> root node
    sequence_tree: HashMap<RdfBlankNode, Option<RdfBlankNode>>,

    annotations: HashMap<CollectedAnnotationKey<'a>, CollectedAnnotation<'a>>,
    blank_nodes: HashMap<RdfBlankNode, CollectedBlankNode>,

    axiom_index: HashMap<(String, String, String), usize>,
    declaration_index: HashMap<String, usize>,
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
            Declaration::Class(iri, _) => iri.as_iri(),
            Declaration::NamedIndividual(iri, _) => iri.as_iri(),
            Declaration::ObjectProperty(iri, _) => iri.as_iri(),
            Declaration::DataProperty(iri, _) => iri.as_iri(),
            Declaration::AnnotationProperty(iri, _) => iri.as_iri(),
            Declaration::Datatype(iri, _) => iri.as_iri(),
        };
        self.declaration_index
            .insert(iri.to_string(), self.declarations.len());
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

    pub(crate) fn get_from_index_mut(&mut self, s: &str, p: &str, o: &str) -> Option<&mut Axiom> {
        self.axiom_index
            .get(&(s.into(), p.into(), o.into()))
            .and_then(|index| self.axioms.get_mut(*index))
    }

    // pub(crate) fn axioms_mut(&mut self) -> &mut Vec<Axiom> {
    //     &mut self.axioms
    // }

    pub(crate) fn insert_blank_node(&mut self, bn: RdfBlankNode, bnh: CollectedBlankNode) {
        self.blank_nodes.insert(bn, bnh);
    }

    pub(crate) fn insert_annotation(
        &mut self,
        key: CollectedAnnotationKey<'a>,
        value: CollectedAnnotation<'a>,
    ) {
        self.annotations.insert(key, value);
    }

    pub(crate) fn annotation(
        &self,
        ann: CollectedAnnotationKey<'a>,
    ) -> Option<&CollectedAnnotation<'a>> {
        self.annotations.get(&ann)
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

    pub(crate) fn set_sequence_root(&mut self, root: &RdfBlankNode, value: Value<'a>) {
        self.sequence_tree.insert(root.clone(), None);
        self.sequences.insert(root.clone(), vec![value]);
    }

    pub(crate) fn get_sequence(&mut self, bn: &RdfBlankNode) -> Option<&mut Vec<Value<'a>>> {
        match self.sequence_tree.get(bn) {
            Some(Some(root)) => self.sequences.get_mut(root),
            Some(None) => self.sequences.get_mut(bn),
            _ => None,
        }
    }

    pub(crate) fn set_sequence_tree(
        &mut self,
        parent: &RdfBlankNode,
        leaf: RdfBlankNode,
    ) -> Result<(), Error> {
        match self.sequence_tree.get(parent).cloned() {
            Some(None) => {
                self.sequence_tree.insert(leaf, Some(parent.clone()));
            }
            Some(Some(parent)) => {
                self.set_sequence_tree(&parent, leaf)?;
            }
            None => {
                return Err(Error::new("Failed to save sequence".into()));
            }
        }
        Ok(())
    }

    pub(crate) fn get_blank(&self, bn: &RdfBlankNode) -> Option<&CollectedBlankNode> {
        self.blank_nodes.get(bn)
    }

    pub(crate) fn annotation_property(
        &self,
        iri: &IRI,
    ) -> Option<(&AnnotationPropertyIRI, &Vec<Annotation>)> {
        self.declaration_index
            .get(iri.as_str())
            .and_then(|index| self.declarations.get(*index))
            .and_then(|d| match d {
                Declaration::AnnotationProperty(a, annotations) => {
                    if a.as_iri() == iri {
                        Some((a, annotations))
                    } else {
                        None
                    }
                }
                _ => None,
            })
    }

    pub(crate) fn class_declaration(&self, cls: &IRI) -> Option<&Declaration> {
        self.declaration_index
            .get(cls.as_str())
            .and_then(|index| self.declarations.get(*index))
            .and_then(|d| match d {
                Declaration::Class(_, _) => Some(d),
                _ => None,
            })
    }
    pub(crate) fn data_property_declaration(&self, dp: &IRI) -> Option<&Declaration> {
        self.declaration_index
            .get(dp.as_str())
            .and_then(|index| self.declarations.get(*index))
            .and_then(|d| match d {
                Declaration::DataProperty(_, _) => Some(d),
                _ => None,
            })
    }

    pub(crate) fn object_property_declaration(&self, iri: &IRI) -> Option<&Declaration> {
        self.declaration_index
            .get(iri.as_str())
            .and_then(|index| self.declarations.get(*index))
            .and_then(|d| match d {
                Declaration::ObjectProperty(_, _) => Some(d),
                _ => None,
            })
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
