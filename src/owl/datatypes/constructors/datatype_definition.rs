use crate::owl::{Annotation, DataPropertyIRI};

use super::{DataComplementOf, DataIntersectionOf, DataOneOf, DataUnionOf, DatatypeRestriction};

#[derive(Debug, Eq, PartialEq)]
pub enum DatatypeDefinitionConstructor {
    DatatypeRestriction(DatatypeRestriction),
    DataComplementOf(DataComplementOf),
    DataIntersectionOf(DataIntersectionOf),
    DataUnionOf(DataUnionOf),
    DataOneOf(DataOneOf),
}

#[derive(Debug, Eq, PartialEq)]
pub struct DatatypeDefinition(
    pub DataPropertyIRI,
    pub DatatypeDefinitionConstructor,
    pub Vec<Annotation>,
);
