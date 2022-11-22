use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::owl::{Axiom, ClassIRI, DatatypeIRI, LiteralOrIRI, IRI};

use super::Annotation;

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct AnnotationPropertyIRI(IRI);
impl AnnotationPropertyIRI {
    pub fn as_iri(&self) -> &IRI {
        &self.0
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
    pub iri: AnnotationPropertyIRI,
    pub cls: ClassIRI,
    pub annotations: Vec<Annotation>,
}

impl AnnotationPropertyDomain {
    pub fn new(iri: AnnotationPropertyIRI, cls: ClassIRI, annotations: Vec<Annotation>) -> Self {
        Self {
            iri,
            cls,
            annotations,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct AnnotationPropertyRange {
    pub iri: AnnotationPropertyIRI,
    pub datatype_iri: DatatypeIRI,
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
    pub iri: AnnotationPropertyIRI,
    pub subject: IRI,
    pub value: LiteralOrIRI,
    pub annotations: Vec<Annotation>,
}

impl AnnotationAssertion {
    pub fn new(
        iri: AnnotationPropertyIRI,
        subject: IRI,
        value: LiteralOrIRI,
        annotations: Vec<Annotation>,
    ) -> Self {
        Self {
            iri,
            subject,
            value,
            annotations,
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
     * The IRI of this annoration.
     */
    iri: IRI, 
    /**
     * The subject IRI.
     */
    subject: IRI, 
    /**
     * The asserted value.
     */
    value: LiteralOrIRI, 
    annotations: Array<Annotation>
};
"#;

    #[wasm_bindgen(typescript_custom_section)]
    const WASM_API_ANNOTATION: &'static str = r#"
export type Annotation = {
    /**
     * The annotation IRI.
     */
    iri: IRI,
    /**
     * The annotated value.
     */
    value: LiteralOrIRI,
    annotations: Array<Annotation>,
};
"#;
}
