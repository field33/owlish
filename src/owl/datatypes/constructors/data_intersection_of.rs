use crate::owl::DataPropertyIRI;

use super::DatatypeDefinitionConstructor;

#[derive(Debug, Eq, PartialEq)]
pub struct DataIntersectionOf(pub DataPropertyIRI, pub Box<DatatypeDefinitionConstructor>);

impl From<DataIntersectionOf> for Box<DatatypeDefinitionConstructor> {
    fn from(c: DataIntersectionOf) -> Self {
        DatatypeDefinitionConstructor::DataIntersectionOf(c).into()
    }
}
impl From<DataIntersectionOf> for DatatypeDefinitionConstructor {
    fn from(c: DataIntersectionOf) -> Self {
        DatatypeDefinitionConstructor::DataIntersectionOf(c)
    }
}
