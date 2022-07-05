use std::collections::HashMap;

use crate::owl::{
    well_known, AnnotationAssertion, Axiom, ClassIRI, DataPropertyDomain, ObjectPropertyDomain,
    ObjectPropertyIRI, ObjectPropertyRange, SubClassOf,
};
use js_sys::Array;
use wasm_bindgen::prelude::*;

#[derive(Debug)]
pub struct Class<'a> {
    pub(crate) iri: &'a ClassIRI,
    pub(crate) axioms: Vec<&'a Axiom>,
}

impl<'a> Class<'a> {
    /// Get all annotations asserted with this class
    pub fn annotations(&self) -> Vec<&AnnotationAssertion> {
        self.axioms
            .iter()
            .filter_map(|a| match a {
                Axiom::AnnotationAssertion(a) => Some(a),
                _ => None,
            })
            .collect()
    }

    /// Get all direct super classes of this class
    pub fn super_classes(&self) -> Vec<&'a SubClassOf> {
        self.axioms
            .iter()
            .filter_map(|a| match a {
                Axiom::SubClassOf(sco) => {
                    if sco.subject().is_iri(self.iri.as_iri()) {
                        Some(sco)
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .collect()
    }

    /// Get all DataProperties whose domain contains this class.
    pub fn data_property_domains(&self) -> Vec<&'a DataPropertyDomain> {
        self.axioms
            .iter()
            .filter_map(|a| match a {
                Axiom::DataPropertyDomain(d) => Some(d),
                _ => None,
            })
            .collect()
    }

    /// Get all ObjectProperties whose domain contains this class.
    pub fn object_property_domains(&self) -> Vec<&'a ObjectPropertyDomain> {
        self.axioms
            .iter()
            .filter_map(|a| match a {
                Axiom::ObjectPropertyDomain(d) => Some(d),
                _ => None,
            })
            .collect()
    }

    /// Get all ObjectProperties whose range contains this class.
    pub fn object_property_ranges(&self) -> Vec<&'a ObjectPropertyRange> {
        self.axioms
            .iter()
            .filter_map(|a| match a {
                Axiom::ObjectPropertyRange(r) => Some(r),
                _ => None,
            })
            .collect()
    }

    pub fn to_owned_class(&self) -> OwnedClass {
        OwnedClass {
            iri: self.iri.clone(),
            axioms: self.axioms.iter().map(|a| (*a).clone()).collect(),
        }
    }
}

#[derive(Debug)]
#[wasm_bindgen]
pub struct OwnedClass {
    iri: ClassIRI,
    axioms: Vec<Axiom>,
}

#[wasm_bindgen]
impl OwnedClass {
    #[wasm_bindgen(getter)]
    pub fn iri(&self) -> String {
        self.iri.to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn label(&self) -> Option<String> {
        self.axioms.iter().find_map(|a| match a {
            Axiom::AnnotationAssertion(an) => {
                if well_known::rdfs_label() == an.0 {
                    an.2.as_str().map(|s| s.to_string())
                } else {
                    None
                }
            }
            _ => None,
        })
    }
}
