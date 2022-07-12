use std::collections::HashMap;

use crate::owl::{Axiom, ClassIRI};

use super::Ontology;
use js_sys::Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl Ontology {
    #[wasm_bindgen(getter)]
    pub fn iri(&self) -> String {
        self.iri.to_string()
    }

    /// @returns ```js
    /// enum {
    ///     Class?: string,
    ///     NamedIndividual?: string,
    ///     ObjectProperty?: string,
    ///     DataProperty?: string,
    ///     Datatype?: string,
    /// }
    /// ```
    pub fn get_declarations(&self) -> Array {
        let array = Array::new();
        for d in self.declarations() {
            if let Ok(value) = JsValue::from_serde(d) {
                array.push(&value);
            }
        }
        array
    }

    /// @returns ```js
    /// enum {
    ///     AnnotationAssertion?: [annotation_prop_iri, iri, value],
    ///     ClassAssertion?: [class_iri, individual_iri],
    ///     ObjectPropertyDomain?: [object_propp_iri, class_iri],
    ///     ObjectPropertyRange?: [object_prop_iri, class_iri],
    ///     ObjectPropertyAssertion?: [object_prop_iri, subject_iri, object_iri],
    ///     ...
    /// }
    /// ```
    pub fn get_axioms(&self) -> Array {
        let array = Array::new();
        for a in self.axioms() {
            if let Ok(value) = JsValue::from_serde(a) {
                array.push(&value);
            }
        }
        array
    }

    // poc api below

    pub fn classes_owned(&self) -> Array {
        let array = Array::new();
        for cls in self.classes().iter().map(|c| c.to_owned_class()) {
            array.push(&cls.into());
        }
        array
    }

    #[wasm_bindgen(getter)]
    pub fn object_properties(&self) -> Array {
        let array = Array::new();
        let mut map: HashMap<String, (Option<ClassIRI>, Option<ClassIRI>)> = HashMap::new();
        for a in &self.owl.axioms {
            match a {
                Axiom::ObjectPropertyDomain(domain) => {
                    if let Some((d, _)) = map.get_mut(&domain.0.as_iri().to_string()) {
                        *d = Some(domain.1.clone())
                    } else {
                        map.insert(
                            domain.0.as_iri().to_string(),
                            (Some(domain.1.clone()), None),
                        );
                    }
                }
                Axiom::ObjectPropertyRange(range) => {
                    if let Some((_, r)) = map.get_mut(&range.0.as_iri().to_string()) {
                        *r = Some(range.1.clone())
                    } else {
                        map.insert(range.0.as_iri().to_string(), (None, Some(range.1.clone())));
                    }
                }
                _ => {}
            }
        }
        for (_, domain_and_range) in map {
            let entry = Array::new();
            if let (Some(domain), Some(range)) = domain_and_range {
                entry.push(&domain.to_string().into());
                entry.push(&range.to_string().into());
            }
        }
        array
    }
}
