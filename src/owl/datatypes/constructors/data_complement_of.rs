use crate::owl::DataPropertyIRI;

use super::DatatypeDefinitionConstructor;

#[derive(Debug, Eq, PartialEq)]
pub struct DataComplementOf(pub DataPropertyIRI);

impl From<DataComplementOf> for Box<DatatypeDefinitionConstructor> {
    fn from(c: DataComplementOf) -> Self {
        DatatypeDefinitionConstructor::DataComplementOf(c).into()
    }
}
impl From<DataComplementOf> for DatatypeDefinitionConstructor {
    fn from(c: DataComplementOf) -> Self {
        DatatypeDefinitionConstructor::DataComplementOf(c)
    }
}
