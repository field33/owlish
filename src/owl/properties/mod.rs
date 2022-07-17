mod axioms;
pub use axioms::*;

mod constructors;
pub use constructors::*;

mod annotation_property;
pub use annotation_property::*;

mod object_properties;
pub use object_properties::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub enum ObjectPropertyConstructor {
    IRI(ObjectPropertyIRI),
    ObjectInverseOf(ObjectInverseOf),
    ObjectPropertyChain(ObjectPropertyChain),
}

#[wasm_bindgen(typescript_custom_section)]
const WASM_API: &'static str = r#"
export interface ObjectPropertyConstructor {
    /**
     * ObjectProperty IRI
     */
    IRI?: IRI
    ObjectInverseOf?: ObjectInverseOf
    ObjectPropertyChain?: ObjectPropertyChain
}
"#;
