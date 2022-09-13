#![allow(non_snake_case)]

use crate::owl::Literal;

use super::Ontology;
use js_sys::{Array, Number};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::console::error_1;

#[wasm_bindgen]
impl Ontology {
    /// Create an ontology based on a turtle formatted string.
    pub fn parseTurtle(ttl: String, options: ParserOptions) -> Option<Ontology> {
        if let Some(json) = js_sys::JSON::stringify(&options)
            .ok()
            .and_then(|s| s.as_string())
        {
            if let Ok(options) = serde_json::from_str(&json) {
                Ontology::parse(&ttl, options).ok()
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Create an ontology based on a json serialized owlish::Ontology.
    pub fn parse_json(json: String) -> Option<Ontology> {
        serde_json::from_str(&json).ok()
    }

    /// Get the IRI of this ontology.
    #[wasm_bindgen(getter)]
    pub fn iri(&self) -> IRI {
        JsValue::from_serde(&self.iri).unwrap().into()
    }

    /// Get all OWL declarations of this ontology.
    #[wasm_bindgen(js_name = "declarations")]
    pub fn wasm_declarations(&self) -> DeclarationArray {
        let array = Array::new();
        for d in self.declarations() {
            if let Ok(value) = JsValue::from_serde(d) {
                array.push(&value);
            }
        }
        array.unchecked_into()
    }

    /// Get all OWL axioms of this ontology.
    #[wasm_bindgen(js_name = "axioms")]
    pub fn wasm_axioms(&self) -> AxiomArray {
        let array = Array::new();
        for a in self.axioms() {
            if let Ok(value) = JsValue::from_serde(a) {
                array.push(&value);
            }
        }
        array.unchecked_into()
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Array<Axiom>")]
    pub type AxiomArray;
    #[wasm_bindgen(typescript_type = "Array<Declaration>")]
    pub type DeclarationArray;
}

#[wasm_bindgen(typescript_custom_section)]
const ONTOLOGY_TS_API: &'static str = r#"
interface Declaration {
    Class?: IRI,
    NamedIndividual?: IRI,
    ObjectProperty?: IRI,
    DataProperty?: IRI,
    AnnotationProperty?: IRI,
    Datatype?: IRI,
}

interface Axiom {
    AnnotationAssertion?: AnnotationAssertion
    SubObjectPropertyOf?: SubObjectPropertyOf
    EquivalentObjectProperties?: EquivalentObjectProperties
    EquivalentDataProperties?: EquivalentDataProperties
    InverseObjectProperties?: InverseObjectProperties
    DisjointObjectProperties?: DisjointObjectProperties
    ObjectPropertyDomain?: ObjectPropertyDomain
    ObjectPropertyRange?: ObjectPropertyRange
    DataPropertyDomain?: DataPropertyDomain
    DataPropertyRange?: DataPropertyRange
    SymmetricObjectProperty?: SymmetricObjectProperty
    AsymmetricObjectProperty?: AsymmetricObjectProperty
    ReflexiveObjectProperty?: ReflexiveObjectProperty
    IrreflexiveObjectProperty?: IrreflexiveObjectProperty
    FunctionalObjectProperty?: FunctionalObjectProperty
    InverseFunctionalObjectProperty?: InverseFunctionalObjectProperty
    TransitiveObjectProperty?: TransitiveObjectProperty
    FunctionalDataProperty?: FunctionalDataProperty
    SubClassOf?: SubClassOf
    EquivalentClasses?: EquivalentClasses
    DisjointClasses?: DisjointClasses
    DatatypeDefinition?: DatatypeDefinition
    ClassAssertion?: ClassAssertion
    SameIndividual?: SameIndividual
    DifferentIndividuals?: DifferentIndividuals
    ObjectPropertyAssertion?: ObjectPropertyAssertion
    NegativeObjectPropertyAssertion?: NegativeObjectPropertyAssertion
    DataPropertyAssertion?: DataPropertyAssertion
    NegativeDataPropertyAssertion?: NegativeDataPropertyAssertion
    HasKey?: HasKey
}

export interface AxiomMatcher<R> {
    AnnotationAssertion?: (a: AnnotationAssertion) => R
    SubObjectPropertyOf?: (a: SubObjectPropertyOf) => R
    EquivalentObjectProperties?: (a: EquivalentObjectProperties) => R
    EquivalentDataProperties?: (a: EquivalentDataProperties) => R
    InverseObjectProperties?: (a: InverseObjectProperties) => R
    DisjointObjectProperties?: (a: DisjointObjectProperties) => R
    ObjectPropertyDomain?: (a: ObjectPropertyDomain) => R
    ObjectPropertyRange?: (a: ObjectPropertyRange) => R
    DataPropertyDomain?: (a: DataPropertyDomain) => R
    DataPropertyRange?: (a: DataPropertyRange) => R
    SymmetricObjectProperty?: (a: SymmetricObjectProperty) => R
    AsymmetricObjectProperty?: (a: AsymmetricObjectProperty) => R
    ReflexiveObjectProperty?: (a: ReflexiveObjectProperty) => R
    IrreflexiveObjectProperty?: (a: IrreflexiveObjectProperty) => R
    FunctionalObjectProperty?: (a: FunctionalObjectProperty) => R
    InverseFunctionalObjectProperty?: (a: InverseFunctionalObjectProperty) => R
    TransitiveObjectProperty?: (a: TransitiveObjectProperty) => R
    FunctionalDataProperty?: (a: FunctionalDataProperty) => R
    SubClassOf?: (a: SubClassOf) => R
    EquivalentClasses?: (a: EquivalentClasses) => R
    DisjointClasses?: (a: DisjointClasses) => R
    DatatypeDefinition?: (a: DatatypeDefinition) => R
    ClassAssertion?: (a: ClassAssertion) => R
    SameIndividual?: (a: SameIndividual) => R
    DifferentIndividuals?: (a: DifferentIndividuals) => R
    ObjectPropertyAssertion?: (a: ObjectPropertyAssertion) => R
    NegativeObjectPropertyAssertion?: (a: NegativeObjectPropertyAssertion) => R
    DataPropertyAssertion?: (a: DataPropertyAssertion) => R
    NegativeDataPropertyAssertion?: (a: NegativeDataPropertyAssertion) => R
    HasKey?: (a: HasKey) => R
}

export function matchAxiom<R>(axiom: Axiom, matcher: AxiomMatcher<R>): R

interface DeclarationMatcher<R> {
    Class?: (iri: IRI) => R,
    NamedIndividual?: (iri: IRI) => R,
    ObjectProperty?: (iri: IRI) => R,
    DataProperty?: (iri: IRI) => R,
    Datatype?: (iri: IRI) => R,
}

export function matchDeclaration<R>(declaration: Declaration, matcher: DeclarationMatcher<R>): R

export interface IRI {
    _type: "IRI",
    string: string
}

export type Value = {
    _type: "string",
    value: string,
    datatypeIri: string,
    lang: null,
} | {
    _type: "raw",
    value: Uint8Array,
    datatypeIri: string,
    lang: null,
} | {
    _type: "dateTime",
    value: string,
    datatypeIri: string,
    lang: null,
} | {
    _type: "langString",
    value: string,
    datatypeIri: string,
    lang: string,
} | {
    _type: "number",
    value: number,
    datatypeIri: string,
    lang: null,
} | {
    _type: "boolean",
    value: boolean,
    datatypeIri: string,
    lang: null,
}


type ValueMatcher<R> =
    | {
          string?: (
              value:
                  | Extract<
                        Value,
                        {
                            _type: 'string'
                        }
                    >
                  | Extract<
                        Value,
                        {
                            _type: 'langString'
                        }
                    >
          ) => R
          raw?: (
              value: Extract<
                  Value,
                  {
                      _type: 'raw'
                  }
              >
          ) => R
          dateTime?: (
              value: Extract<
                  Value,
                  {
                      _type: 'dateTime'
                  }
              >
          ) => R
          number?: (
              value: Extract<
                  Value,
                  {
                      _type: 'number'
                  }
              >
          ) => R
          boolean?: (
              value: Extract<
                  Value,
                  {
                      _type: 'boolean'
                  }
              >
          ) => R
          default: (value: Value) => R
      }
    | {
          string: (
              value:
                  | Extract<
                        Value,
                        {
                            _type: 'string'
                        }
                    >
                  | Extract<
                        Value,
                        {
                            _type: 'langString'
                        }
                    >
          ) => R
          raw: (
              value: Extract<
                  Value,
                  {
                      _type: 'raw'
                  }
              >
          ) => R
          dateTime: (
              value: Extract<
                  Value,
                  {
                      _type: 'dateTime'
                  }
              >
          ) => R
          number: (
              value: Extract<
                  Value,
                  {
                      _type: 'number'
                  }
              >
          ) => R
          boolean: (
              value: Extract<
                  Value,
                  {
                      _type: 'boolean'
                  }
              >
          ) => R
          default?: (value: Value) => R
      }

export function matchValue<R>(value: Value, matcher: ValueMatcher<R>): R;
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "IRI")]
    pub type IRI;
    #[wasm_bindgen(typescript_type = "Value")]
    pub type Value;
    #[wasm_bindgen(typescript_type = "ParserOptions")]
    pub type ParserOptions;

}

#[wasm_bindgen]
pub fn Iri(iri: &str) -> Option<IRI> {
    match crate::owl::IRI::new(iri) {
        Ok(iri) => iri_to_js_iri(&iri),
        Err(e) => {
            error_1(&format!("Failed to parse IRI {}: {}", iri, e).into());
            None
        }
    }
}

fn iri_to_js_iri(iri: &crate::owl::IRI) -> Option<IRI> {
    match JsValue::from_serde(iri) {
        Ok(jsv) => Some(jsv.unchecked_into()),
        Err(e) => {
            error_1(&format!("Failed to create JS value from IRI {}: {}", iri, e).into());
            None
        }
    }
}

#[wasm_bindgen]
pub fn set_query_parameter(iri: &IRI, name: &str, value: &str) -> Option<IRI> {
    match iri.into_serde::<crate::owl::IRI>() {
        Ok(mut iri) => {
            if let Err(e) = iri.set_query_parameter(name, value) {
                error_1(&e.into())
            }
            iri_to_js_iri(&iri)
        }
        Err(e) => {
            error_1(&format!("Failed to parse JS IRI into Rust IRI: {:?}", e).into());
            None
        }
    }
}

#[wasm_bindgen]
#[allow(non_camel_case_types)]
pub struct well_known {
    // generate namespace in wasm
}

#[wasm_bindgen]
impl well_known {
    pub fn owl_AnnotationProperty() -> IRI {
        iri_to_js_iri(crate::owl::well_known::owl_AnnotationProperty().as_iri()).unwrap()
    }
    pub fn owl_AsymmetricProperty() -> IRI {
        iri_to_js_iri(crate::owl::well_known::owl_AsymmetricProperty().as_iri()).unwrap()
    }
    pub fn owl_Class() -> IRI {
        iri_to_js_iri(crate::owl::well_known::owl_Class().as_iri()).unwrap()
    }
    pub fn owl_ObjectProperty() -> IRI {
        iri_to_js_iri(crate::owl::well_known::owl_ObjectProperty().as_iri()).unwrap()
    }
    pub fn owl_Ontology() -> IRI {
        iri_to_js_iri(&crate::owl::well_known::owl_Ontology()).unwrap()
    }
    pub fn owl_SymmetricProperty() -> IRI {
        iri_to_js_iri(crate::owl::well_known::owl_SymmetricProperty().as_iri()).unwrap()
    }
    pub fn owl_Thing() -> IRI {
        iri_to_js_iri(crate::owl::well_known::owl_Thing().as_iri()).unwrap()
    }

    pub fn rdf_type() -> IRI {
        iri_to_js_iri(crate::owl::well_known::rdf_type().as_iri()).unwrap()
    }
    pub fn rdfs_comment() -> IRI {
        iri_to_js_iri(crate::owl::well_known::rdfs_comment().as_iri()).unwrap()
    }
    pub fn rdfs_label() -> IRI {
        iri_to_js_iri(crate::owl::well_known::rdfs_label().as_iri()).unwrap()
    }
    pub fn rdfs_subClassOf() -> IRI {
        iri_to_js_iri(crate::owl::well_known::rdfs_subClassOf().as_iri()).unwrap()
    }

    pub fn xsd_boolean() -> IRI {
        iri_to_js_iri(crate::owl::well_known::xsd_boolean().as_iri()).unwrap()
    }
    pub fn xsd_string() -> IRI {
        iri_to_js_iri(crate::owl::well_known::xsd_float().as_iri()).unwrap()
    }
    pub fn xsd_float() -> IRI {
        iri_to_js_iri(crate::owl::well_known::xsd_float().as_iri()).unwrap()
    }
    pub fn xsd_dateTime() -> IRI {
        iri_to_js_iri(crate::owl::well_known::xsd_dateTime().as_iri()).unwrap()
    }
    pub fn xsd_integer() -> IRI {
        iri_to_js_iri(crate::owl::well_known::xsd_integer().as_iri()).unwrap()
    }
    pub fn xsd_maxExclusive() -> IRI {
        iri_to_js_iri(crate::owl::well_known::xsd_maxExclusive().as_iri()).unwrap()
    }
    pub fn xsd_maxInclusive() -> IRI {
        iri_to_js_iri(crate::owl::well_known::xsd_maxInclusive().as_iri()).unwrap()
    }
    pub fn xsd_minExclusive() -> IRI {
        iri_to_js_iri(crate::owl::well_known::xsd_minExclusive().as_iri()).unwrap()
    }
    pub fn xsd_minInclusive() -> IRI {
        iri_to_js_iri(crate::owl::well_known::xsd_minInclusive().as_iri()).unwrap()
    }
    pub fn xsd_nonNegativeInteger() -> IRI {
        iri_to_js_iri(crate::owl::well_known::xsd_nonNegativeInteger().as_iri()).unwrap()
    }
}

fn value_to_js_value(value: &crate::owl::Literal) -> Option<Value> {
    match JsValue::from_serde(value) {
        Ok(jsv) => Some(jsv.unchecked_into()),
        Err(e) => {
            error_1(&format!("Failed to create JS value from value {:?}: {}", value, e).into());
            None
        }
    }
}

#[wasm_bindgen]
pub fn StringValue(value: &str) -> Option<Value> {
    let lit = Literal::String(value.into());
    value_to_js_value(&lit)
}

#[wasm_bindgen]
pub fn DateTimeValue(value: &str) -> Option<Value> {
    let lit = Literal::DateTime(value.into());
    value_to_js_value(&lit)
}

#[wasm_bindgen]
pub fn NumericValue(value: Number) -> Option<Value> {
    let number = if let Some(f) = value.as_f64() { f } else { 0.0 };
    if number.is_nan() {
        return None;
    }
    let lit = if number.fract() == 0.0 {
        Literal::Number {
            number: serde_json::Number::from_f64(number).unwrap(), // NAN is handled above
            type_iri: crate::owl::well_known::xsd_integer().into(),
        }
    } else {
        Literal::Number {
            number: serde_json::Number::from_f64(number).unwrap(), // NAN is handled above
            type_iri: crate::owl::well_known::xsd_float().into(),
        }
    };
    value_to_js_value(&lit)
}

#[wasm_bindgen]
pub fn NonNegativeNumericValue(value: Number) -> Option<Value> {
    let number = if let Some(f) = value.as_f64() { f } else { 0.0 };
    if number.is_nan() {
        return None;
    }
    let lit = if number.fract() == 0.0 {
        Literal::Number {
            number: serde_json::Number::from_f64(number).unwrap(), // NAN is handled above
            type_iri: crate::owl::well_known::xsd_nonNegativeInteger().into(),
        }
    } else {
        Literal::Number {
            number: serde_json::Number::from_f64(number).unwrap(), // NAN is handled above
            type_iri: crate::owl::well_known::xsd_float().into(),
        }
    };
    value_to_js_value(&lit)
}

#[wasm_bindgen(typescript_custom_section)]
const PARSER_OPTIONS_TS_API: &'static str = r#"
interface ParserOptions {
    entries: Record<ParserOptionKey, ParserOptionValue>
}

type ParserOptionKey = "Known"
interface ParserOptionValue { 
    Known?: Array<Declaration>
}
"#;
