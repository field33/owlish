use std::collections::HashMap;

use harriet::triple_production::RdfBlankNode;

use crate::{api::Ontology, error::Error, owl::*, parser::matcher::Value};

use super::matcher::MatcherState;

/// Handle when a matcher matched. Returns whether the matched rules where actually
pub(crate) type MatcherHandler =
    Box<dyn Fn(&MatcherState, &mut OntologyCollector) -> Result<bool, Error>>;

#[derive(Debug, Clone)]
pub(crate) enum BlankNodeHandle {
    Annotate {
        subject: String,
        predicate: String,
        object: String,
    },
    ClassConstructor(Box<ClassConstructor>),
}

#[derive(Debug, Default)]
pub(crate) struct OntologyCollector {
    iri: Option<IRI>,
    declarations: Vec<Declaration>,
    axioms: Vec<Axiom>,
    // blank node -> IRI
    sequences: HashMap<RdfBlankNode, Vec<Value>>,
    // child node -> root node
    sequence_tree: HashMap<RdfBlankNode, Option<RdfBlankNode>>,

    blank_nodes: HashMap<RdfBlankNode, BlankNodeHandle>,
}

impl OntologyCollector {
    pub(crate) fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub(crate) fn set_iri(&mut self, iri: IRI) {
        self.iri = Some(iri);
    }

    pub(crate) fn push_declaration(&mut self, declaration: Declaration) {
        self.declarations.push(declaration)
    }

    pub(crate) fn push_axiom(&mut self, axiom: Axiom) {
        self.axioms.push(axiom);
    }

    pub(crate) fn _axioms(&mut self) -> &Vec<Axiom> {
        &self.axioms
    }

    pub(crate) fn axioms_mut(&mut self) -> &mut Vec<Axiom> {
        &mut self.axioms
    }

    pub(crate) fn insert_blank_node(&mut self, bn: RdfBlankNode, bnh: BlankNodeHandle) {
        self.blank_nodes.insert(bn, bnh);
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

    pub(crate) fn set_sequence_root(&mut self, root: &RdfBlankNode, value: Value) {
        self.sequence_tree.insert(root.clone(), None);
        self.sequences.insert(root.clone(), vec![value]);
    }

    pub(crate) fn get_sequence(&mut self, bn: &RdfBlankNode) -> Option<&mut Vec<Value>> {
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

    pub(crate) fn get_blank(&self, bn: &RdfBlankNode) -> Option<&BlankNodeHandle> {
        self.blank_nodes.get(bn)
    }

    pub(crate) fn annotation_property(
        &self,
        iri: &IRI,
    ) -> Option<(&AnnotationPropertyIRI, &Vec<Annotation>)> {
        self.declarations.iter().find_map(|d| match d {
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
        self.declarations.iter().find(|d| match d {
            Declaration::Class(iri, _) => iri.as_iri() == cls,
            _ => false,
        })
    }
    pub(crate) fn data_property_declaration(&self, dp: &IRI) -> Option<&Declaration> {
        self.declarations.iter().find(|d| match d {
            Declaration::DataProperty(iri, _) => iri.as_iri() == dp,
            _ => false,
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
                $($variable)+: &'a Value,
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
