use crate::owl::DataPropertyIRI;

use super::DatatypeDefinitionConstructor;

#[derive(Debug, Eq, PartialEq)]
pub struct DataUnionOf(pub DataPropertyIRI, pub Box<DatatypeDefinitionConstructor>);

impl From<DataUnionOf> for Box<DatatypeDefinitionConstructor> {
    fn from(c: DataUnionOf) -> Self {
        DatatypeDefinitionConstructor::DataUnionOf(c).into()
    }
}
impl From<DataUnionOf> for DatatypeDefinitionConstructor {
    fn from(c: DataUnionOf) -> Self {
        DatatypeDefinitionConstructor::DataUnionOf(c)
    }
}
