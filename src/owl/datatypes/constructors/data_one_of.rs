use crate::owl::Value;

use super::DatatypeDefinitionConstructor;

#[derive(Debug, Eq, PartialEq)]
pub struct DataOneOf(pub Vec<Value>);

impl From<DataOneOf> for Box<DatatypeDefinitionConstructor> {
    fn from(c: DataOneOf) -> Self {
        DatatypeDefinitionConstructor::DataOneOf(c).into()
    }
}
impl From<DataOneOf> for DatatypeDefinitionConstructor {
    fn from(c: DataOneOf) -> Self {
        DatatypeDefinitionConstructor::DataOneOf(c)
    }
}