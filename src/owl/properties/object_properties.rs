use serde::{Deserialize, Serialize};

use crate::{
    error::Error,
    owl::{Axiom, IRIList, IndividualIRI, ObjectPropertyConstructor, IRI},
};
use crate::owl::ResourceId;

use super::Annotation;

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct ObjectPropertyIRI(IRI);

impl From<IRI> for ObjectPropertyIRI {
    fn from(iri: IRI) -> Self {
        Self(iri)
    }
}
impl From<ObjectPropertyIRI> for ObjectPropertyConstructor {
    fn from(iri: ObjectPropertyIRI) -> Self {
        Self::IRI(iri)
    }
}
impl TryFrom<&str> for ObjectPropertyIRI {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        IRI::try_from(value).map(|iri| iri.into())
    }
}
impl ObjectPropertyIRI {
    pub fn as_iri(&self) -> &IRI {
        &self.0
    }
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ObjectPropertyAssertion {
    /// Known IDs of reifications of this assertion.
    #[serde(rename = "resourceIds")]
    pub resource_ids: Vec<ResourceId>,
    pub subject: IndividualIRI,
    pub iri: ObjectPropertyIRI,
    pub object: IRIList,
    pub annotations: Vec<Annotation>,
}

impl ObjectPropertyAssertion {
    pub fn new(
        iri: ObjectPropertyIRI,
        subject: IndividualIRI,
        object: IndividualIRI,
        annotations: Vec<Annotation>,
        resource_ids: Vec<ResourceId>,
    ) -> Self {
        Self {
            iri,
            subject,
            object: IRIList::IRI(object.as_iri().clone()),
            annotations,
            resource_ids,
        }
    }
    pub fn new_with_list(
        iri: ObjectPropertyIRI,
        subject: IndividualIRI,
        object: Vec<IRI>,
        annotations: Vec<Annotation>,
        resource_ids: Vec<ResourceId>,
    ) -> Self {
        Self {
            resource_ids,
            iri,
            subject,
            object: IRIList::List(object),
            annotations,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct NegativeObjectPropertyAssertion {
    pub iri: ObjectPropertyIRI,
    pub subject: IndividualIRI,
    pub object: IndividualIRI,
    pub annotations: Vec<Annotation>,
}

impl NegativeObjectPropertyAssertion {
    pub fn new(
        iri: ObjectPropertyIRI,
        subject: IndividualIRI,
        object: IndividualIRI,
        annotations: Vec<Annotation>,
    ) -> Self {
        Self {
            iri,
            subject,
            object,
            annotations,
        }
    }
}
impl From<ObjectPropertyAssertion> for Axiom {
    fn from(opa: ObjectPropertyAssertion) -> Self {
        Self::ObjectPropertyAssertion(opa)
    }
}

#[cfg(feature = "wasm")]
mod wasm {
    use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen(typescript_custom_section)]
    const WASM_API1: &'static str = r#"
/**
 * Assigngs an ObjectProperty to two Individuals.
 */
export type ObjectPropertyAssertion = {
    /**
     * The IRI of the property.
     */
    iri: IRI,
    /**
     * The subject Individual.
     */
    subject: IRI,
    /**
     * The object Individual(s).
     */
    object: IRIList,
    annotations: Array<Annotation>,
};
"#;

    #[wasm_bindgen(typescript_custom_section)]
    const WASM_API2: &'static str = r#"
/**
 * Opposite of ObjectPropertyAssertion.
 */
export type NegativeObjectPropertyAssertion = {
    /**
     * The IRI of the property.
     */
    iri: IRI,
    /**
     * The subject Individual.
     */
    subject: IRI,
    /**
     * The object Individual
     */
    object: IRI,
    annotations: Array<Annotation>,
};
"#;
}
