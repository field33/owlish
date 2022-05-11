mod axioms;
pub use axioms::*;

mod constructors;
pub use constructors::*;

mod annotation_property;
pub use annotation_property::*;

mod object_properties;
pub use object_properties::*;

#[derive(Debug, Eq, PartialEq)]
pub enum ObjectPropertyConstructor {
    IRI(ObjectPropertyIRI),
    ObjectInverseOf(ObjectInverseOf),
    ObjectPropertyChain(ObjectPropertyChain),
}
