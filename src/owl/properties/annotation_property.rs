use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::owl::{Axiom, ClassIRI, DatatypeIRI, LiteralOrIRI, IRI, ResourceId};

use super::Annotation;

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct AnnotationPropertyIRI(IRI);
impl AnnotationPropertyIRI {
    pub fn as_iri(&self) -> &IRI {
        &self.0
    }

    pub fn as_str(&self) -> &str {
        self.as_iri().as_str()
    }
}
impl From<IRI> for AnnotationPropertyIRI {
    fn from(iri: IRI) -> Self {
        Self(iri)
    }
}
impl Display for AnnotationPropertyIRI {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct AnnotationPropertyDomain {
    #[serde(rename = "annotationIRI")]
    pub iri: AnnotationPropertyIRI,
    #[serde(rename = "classIRI")]
    pub class_iri: ClassIRI,
    #[serde(rename = "annotations")]
    pub annotations: Vec<Annotation>,
}

impl AnnotationPropertyDomain {
    pub fn new(iri: AnnotationPropertyIRI, cls: ClassIRI, annotations: Vec<Annotation>) -> Self {
        Self {
            iri,
            class_iri: cls,
            annotations,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct AnnotationPropertyRange {
    #[serde(rename = "annotationIRI")]
    pub iri: AnnotationPropertyIRI,
    #[serde(rename = "datatypeIRI")]
    pub datatype_iri: DatatypeIRI,
    #[serde(rename = "annotations")]
    pub annotations: Vec<Annotation>,
}

impl AnnotationPropertyRange {
    pub fn new(
        iri: AnnotationPropertyIRI,
        datatype_iri: DatatypeIRI,
        annotations: Vec<Annotation>,
    ) -> Self {
        Self {
            iri,
            datatype_iri,
            annotations,
        }
    }
}

impl From<AnnotationPropertyRange> for Axiom {
    fn from(a: AnnotationPropertyRange) -> Self {
        Self::AnnotationPropertyRange(a)
    }
}

impl From<AnnotationPropertyDomain> for Axiom {
    fn from(a: AnnotationPropertyDomain) -> Self {
        Self::AnnotationPropertyDomain(a)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct AnnotationAssertion {
    /// Known IDs of reifications of this assertion.
    #[serde(rename = "resourceIds")]
    pub resource_ids: Vec<ResourceId>,
    pub subject: ResourceId,
    #[serde(rename = "annotationIRI")]
    pub iri: AnnotationPropertyIRI,
    #[serde(rename = "value")]
    pub value: LiteralOrIRI,
    #[serde(rename = "annotations")]
    pub annotations: Vec<Annotation>,
}

impl AnnotationAssertion {
    pub fn new<S: Into<ResourceId>>(
        iri: AnnotationPropertyIRI,
        subject: S,
        value: LiteralOrIRI,
        annotations: Vec<Annotation>,
        resource_ids: Vec<ResourceId>,
    ) -> Self {
        Self {
            iri,
            subject: subject.into(),
            value,
            annotations,
            resource_ids
        }
    }
}

impl From<AnnotationAssertion> for Axiom {
    fn from(aa: AnnotationAssertion) -> Self {
        Axiom::AnnotationAssertion(aa)
    }
}

#[cfg(feature = "wasm")]
mod wasm {
    use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen(typescript_custom_section)]
    const WASM_API_ANNOTATION_ASSERTION: &'static str = r#"
/**
 * Assertion of an AnnotationProperty to some subject 
 */
export type AnnotationAssertion = {
    /**
     * The IRI of this annotation.
     */
    annotationIRI: IRI, 
    /**
     * The ResourceId of the subject.
     */
    subject: ResourceId,
    /**
     * The asserted value.
     */
    value: LiteralOrIRI, 
    annotations: Array<Annotation>
};
export type AnnotationAssertionDomain = {
    /**
     * The IRI of this annotation.
     */
    annotationIRI: IRI, 
    classIRI: IRI, 
    annotations: Array<Annotation>
};
export type AnnotationAssertionRange = {
    /**
     * The IRI of this annotation.
     */
    annotationIRI: IRI, 
    datatypeIRI: IRI, 
    annotations: Array<Annotation>
};
"#;

    #[wasm_bindgen(typescript_custom_section)]
    const WASM_API_ANNOTATION: &'static str = r#"
export type Annotation = {
    /**
     * The annotation IRI.
     */
    annotationIRI: IRI,
    /**
     * The annotation value.
     */
    value: LiteralOrIRI,
    annotations: Array<Annotation>,
};
"#;
}
