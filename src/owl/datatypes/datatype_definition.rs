use crate::owl::{Annotation, DataPropertyIRI, DatatypeIRI, Value};
#[derive(Debug)]
pub enum Restriction {
    Numeric(DatatypeIRI, Value),
}

#[derive(Debug)]
pub struct DatatypeRestriction(pub(crate) DatatypeIRI, pub(crate) Vec<Restriction>);
#[derive(Debug)]
pub struct DataComplementOf(pub(crate) DataPropertyIRI);
#[derive(Debug)]
pub struct DataIntersectionOf(
    pub(crate) DataPropertyIRI,
    pub(crate) Box<DatatypeDefinitionConstructor>,
);
#[derive(Debug)]
pub struct DataUnionOf(
    pub(crate) DataPropertyIRI,
    pub(crate) Box<DatatypeDefinitionConstructor>,
);
#[derive(Debug)]
pub struct DataOneOf(pub(crate) Vec<Value>);
#[derive(Debug)]
pub enum DatatypeDefinitionConstructor {
    DatatypeRestriction(DatatypeRestriction),
    DataComplementOf(DataComplementOf),
    DataIntersectionOf(DataIntersectionOf),
    DataUnionOf(DataUnionOf),
    DataOneOf(DataOneOf),
}

impl From<DatatypeRestriction> for Box<DatatypeDefinitionConstructor> {
    fn from(c: DatatypeRestriction) -> Self {
        DatatypeDefinitionConstructor::DatatypeRestriction(c).into()
    }
}
impl From<DataComplementOf> for Box<DatatypeDefinitionConstructor> {
    fn from(c: DataComplementOf) -> Self {
        DatatypeDefinitionConstructor::DataComplementOf(c).into()
    }
}
impl From<DataIntersectionOf> for Box<DatatypeDefinitionConstructor> {
    fn from(c: DataIntersectionOf) -> Self {
        DatatypeDefinitionConstructor::DataIntersectionOf(c).into()
    }
}
impl From<DataUnionOf> for Box<DatatypeDefinitionConstructor> {
    fn from(c: DataUnionOf) -> Self {
        DatatypeDefinitionConstructor::DataUnionOf(c).into()
    }
}
impl From<DataOneOf> for Box<DatatypeDefinitionConstructor> {
    fn from(c: DataOneOf) -> Self {
        DatatypeDefinitionConstructor::DataOneOf(c).into()
    }
}

impl From<DatatypeRestriction> for DatatypeDefinitionConstructor {
    fn from(c: DatatypeRestriction) -> Self {
        DatatypeDefinitionConstructor::DatatypeRestriction(c)
    }
}
impl From<DataComplementOf> for DatatypeDefinitionConstructor {
    fn from(c: DataComplementOf) -> Self {
        DatatypeDefinitionConstructor::DataComplementOf(c)
    }
}
impl From<DataIntersectionOf> for DatatypeDefinitionConstructor {
    fn from(c: DataIntersectionOf) -> Self {
        DatatypeDefinitionConstructor::DataIntersectionOf(c)
    }
}
impl From<DataUnionOf> for DatatypeDefinitionConstructor {
    fn from(c: DataUnionOf) -> Self {
        DatatypeDefinitionConstructor::DataUnionOf(c)
    }
}
impl From<DataOneOf> for DatatypeDefinitionConstructor {
    fn from(c: DataOneOf) -> Self {
        DatatypeDefinitionConstructor::DataOneOf(c)
    }
}
#[derive(Debug)]
pub struct DatatypeDefinition(
    pub(crate) DataPropertyIRI,
    pub(crate) DatatypeDefinitionConstructor,
    pub(crate) Vec<Annotation>,
);
