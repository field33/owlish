#![allow(non_snake_case)]

use crate::owl::Literal;

use super::Ontology;
use js_sys::{Array, Number, JSON};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::console::error_1;
use crate::computation::GetComputations;

#[wasm_bindgen]
impl Ontology {
    /// Returns the json serialized version of this ontology or undefined if it could not be serialized.
    pub fn to_string(&self) -> Option<String> {
        serde_json::to_string(self).ok()
    }

    pub fn append(&mut self, other: Self) {
        for d in other.owl.declarations {
            self.owl.declarations.push(d);
        }
        for a in other.owl.axioms {
            self.owl.axioms.push(a);
        }
        for (key, import) in other.imports {
            if !self.imports.contains_key(&key) {
                self.imports.insert(key, import);
            }
        }
    }

    /// Create an ontology based on a turtle formatted string.
    pub fn parseTurtle(ttl: String, options: ParserOptions) -> Option<Ontology> {
        match js_sys::JSON::stringify(&options) {
            Ok(json) => match json.as_string() {
                Some(json) => match serde_json::from_str(&json) {
                    Ok(options) => match Ontology::parse(&ttl, options) {
                        Ok(o) => Some(o),
                        Err(e) => {
                            error_1(&format!("Failed to parse ontology: {:?}", e).into());
                            None
                        }
                    },
                    Err(e) => {
                        error_1(&format!("Invalid parser options: {}", e).into());
                        None
                    }
                },
                None => {
                    error_1(
                        &format!("Invalid parser options: Could not stringify provided object")
                            .into(),
                    );
                    None
                }
            },
            Err(e) => {
                error_1(&format!("Invalid parser options: {:?}", e).into());
                None
            }
        }
    }

    /// Create an ontology based on a json serialized owlish::Ontology.
    pub fn parse_json(json: String) -> Option<Ontology> {
        serde_json::from_str(&json).ok()
    }

    /// Get the IRI of this ontology.
    #[wasm_bindgen(getter, js_name = "iri")]
    pub fn get_iri(&self) -> IRI {
        let s = serde_json::to_string(&self.iri).unwrap();
        JSON::parse(&s).unwrap().into()
    }

    /// Get all OWL declarations of this ontology.
    #[wasm_bindgen(js_name = "declarations")]
    pub fn wasm_declarations(&self) -> DeclarationArray {
        let array = Array::new();
        for d in self.declarations() {
            if let Ok(s) = serde_json::to_string(&d) {
                if let Ok(value) = JSON::parse(&s) {
                    array.push(&value);
                }
            }
        }
        array.unchecked_into()
    }

    /// Get all OWL axioms of this ontology.
    #[wasm_bindgen(js_name = "axioms")]
    pub fn wasm_axioms(&self) -> AxiomArray {
        let array = Array::new();
        for a in self.axioms() {
            if let Ok(s) = serde_json::to_string(&a) {
                if let Ok(value) = JSON::parse(&s) {
                    array.push(&value);
                }
            }
        }
        array.unchecked_into()
    }

    /// Get all FNO/Oxolotl computations of this ontology.
    #[wasm_bindgen(js_name = "computations")]
    pub fn wasm_computations(&self) -> ComputationArray {
        let array = Array::new();
        for a in self.computations() {
            if let Ok(s) = serde_json::to_string(&a) {
                if let Ok(value) = JSON::parse(&s) {
                    array.push(&value);
                }
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
    #[wasm_bindgen(typescript_type = "Array<Computation>")]
    pub type ComputationArray;
}

#[wasm_bindgen(typescript_custom_section)]
const ONTOLOGY_TS_API: &'static str = r#"
interface Declaration {
    Class?: {iri: IRI, annotations: Array<Annotation>},
    NamedIndividual?: {iri: IRI, annotations: Array<Annotation>},
    ObjectProperty?: {iri: IRI, annotations: Array<Annotation>},
    DataProperty?: {iri: IRI, annotations: Array<Annotation>},
    AnnotationProperty?: {iri: IRI, annotations: Array<Annotation>},
    Datatype?: {iri: IRI, annotations: Array<Annotation>},
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

interface Computation {
    iri: IRI,
    axioms: Array<Axiom>,
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
    Class?: (cls: Required<Declaration>["Class"]) => R,
    NamedIndividual?: (individual: Required<Declaration>["NamedIndividual"]) => R,
    ObjectProperty?: (objectProp: Required<Declaration>["ObjectProperty"]) => R,
    DataProperty?: (dataProp: Required<Declaration>["DataProperty"]) => R,
    AnnotationProperty?: (annotationProp: Required<Declaration>["AnnotationProperty"]) => R,
    Datatype?: (datatype: Required<Declaration>["Datatype"]) => R,

}

export function matchDeclaration<R>(declaration: Declaration, matcher: DeclarationMatcher<R>): R

export interface IRI {
    _type: "IRI",
    string: string
}

export type Value = {
    _type: "string",
    value: string,
    datatypeIRI: string,
    lang: null,
} | {
    _type: "raw",
    value: Uint8Array,
    datatypeIRI: string,
    lang: null,
} | {
    _type: "dateTime",
    value: string,
    datatypeIRI: string,
    lang: null,
} | {
    _type: "langString",
    value: string,
    datatypeIRI: string,
    lang: string,
} | {
    _type: "number",
    value: number,
    datatypeIRI: string,
    lang: null,
} | {
    _type: "boolean",
    value: boolean,
    datatypeIRI: string,
    lang: null,
} | {
    _type: "duration",
    value: Duration,
    datatypeIRI: string,
    lang: null,
}

// Duration format based on `date-fns` NPM package.
export interface Duration {
  years?: number
  months?: number
  days?: number
  hours?: number
  minutes?: number
  seconds?: number
}

export type LiteralOrIRI = { _type: "Literal", Literal: Value } | { _type: "IRI", IRI: IRI }

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
          duration?: (
              value: Extract<
                  Value,
                  {
                      _type: 'duration'
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
          duration: (
              value: Extract<
                  Value,
                  {
                      _type: 'duration'
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
    #[wasm_bindgen(typescript_type = "Triple")]
    pub type Triple;
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
    match serde_json::to_string(iri) {
        Ok(s) => match JSON::parse(&s) {
            Ok(v) => Some(v.into()),
            Err(e) => {
                error_1(&format!("Failed to create JS value from IRI {}: {:?}", iri, e).into());
                None
            }
        },
        Err(e) => {
            error_1(&format!("Failed to create JS value from IRI {}: {}", iri, e).into());
            None
        }
    }
}

#[wasm_bindgen]
pub fn set_query_parameter(iri: &IRI, name: &str, value: &str) -> Option<IRI> {
    match JSON::stringify(iri) {
        Ok(s) => match s.as_string() {
            Some(s) => match serde_json::from_str::<crate::owl::IRI>(&s) {
                Ok(mut iri) => {
                    if let Err(e) = iri.set_query_parameter(name, value) {
                        error_1(&e.into())
                    }
                    iri_to_js_iri(&iri)
                }
                Err(e) => {
                    error_1(&format!("Failed to set query parameter: {}", e).into());
                    None
                }
            },
            None => {
                error_1(
                    &format!("Failed to set query parameter: Value could not be stringified.")
                        .into(),
                );
                None
            }
        },
        Err(e) => {
            error_1(&format!("Failed to set query parameter: {:?}", e).into());
            None
        }
    }
}

#[wasm_bindgen]
pub fn replace_leaf(iri: &IRI, new_leaf_name: Option<String>) -> Option<IRI> {
    match JSON::stringify(iri) {
        Ok(s) => match s.as_string() {
            Some(s) => match serde_json::from_str::<crate::owl::IRI>(&s) {
                Ok(mut iri) => {
                    if let Err(e) = iri.set_leaf(new_leaf_name) {
                        error_1(&e.into());
                        return None;
                    }
                    iri_to_js_iri(&iri)
                }
                Err(e) => {
                    error_1(&format!("Failed to set query parameter: {}", e).into());
                    None
                }
            },
            None => {
                error_1(
                    &format!("Failed to set query parameter: Value could not be stringified.")
                        .into(),
                );
                None
            }
        },
        Err(e) => {
            error_1(&format!("Failed to set query parameter: {:?}", e).into());
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
        iri_to_js_iri(&crate::owl::well_known::owl_AnnotationProperty()).unwrap()
    }
    pub fn owl_AsymmetricProperty() -> IRI {
        iri_to_js_iri(&crate::owl::well_known::owl_AsymmetricProperty()).unwrap()
    }
    pub fn owl_Class() -> IRI {
        iri_to_js_iri(crate::owl::well_known::owl_Class().as_iri()).unwrap()
    }
    pub fn owl_ObjectProperty() -> IRI {
        iri_to_js_iri(&crate::owl::well_known::owl_ObjectProperty()).unwrap()
    }
    pub fn owl_Ontology() -> IRI {
        iri_to_js_iri(&crate::owl::well_known::owl_Ontology()).unwrap()
    }
    pub fn owl_SymmetricProperty() -> IRI {
        iri_to_js_iri(&crate::owl::well_known::owl_SymmetricProperty()).unwrap()
    }
    pub fn owl_Thing() -> IRI {
        iri_to_js_iri(crate::owl::well_known::owl_Thing().as_iri()).unwrap()
    }
    pub fn owl_allValuesFrom() -> IRI {
        iri_to_js_iri(&crate::owl::well_known::owl_allValuesFrom()).unwrap()
    }
    pub fn owl() -> IRI {
        iri_to_js_iri(&crate::owl::well_known::owl()).unwrap()
    }
    pub fn owl_cardinality() -> IRI {
        iri_to_js_iri(&crate::owl::well_known::owl_cardinality()).unwrap()
    }
    pub fn owl_Restriction() -> IRI {
        iri_to_js_iri(crate::owl::well_known::owl_Restriction().as_iri()).unwrap()
    }
    pub fn owl_complementOf() -> IRI {
        iri_to_js_iri(&crate::owl::well_known::owl_complementOf()).unwrap()
    }
    pub fn owl_Datatype() -> IRI {
        iri_to_js_iri(&crate::owl::well_known::owl_Datatype()).unwrap()
    }
    pub fn owl_DatatypeProperty() -> IRI {
        iri_to_js_iri(&crate::owl::well_known::owl_DatatypeProperty()).unwrap()
    }
    pub fn owl_hasSelf() -> IRI {
        iri_to_js_iri(&crate::owl::well_known::owl_hasSelf()).unwrap()
    }
    pub fn owl_annotatedSource() -> IRI {
        iri_to_js_iri(&crate::owl::well_known::owl_annotatedSource()).unwrap()
    }
    pub fn owl_intersectionOf() -> IRI {
        iri_to_js_iri(&crate::owl::well_known::owl_intersectionOf()).unwrap()
    }
    pub fn owl_qualifiedCardinality() -> IRI {
        iri_to_js_iri(&crate::owl::well_known::owl_qualifiedCardinality()).unwrap()
    }
    pub fn owl_maxCardinality() -> IRI {
        iri_to_js_iri(&crate::owl::well_known::owl_maxCardinality()).unwrap()
    }
    pub fn owl_maxQualifiedCardinality() -> IRI {
        iri_to_js_iri(&crate::owl::well_known::owl_maxQualifiedCardinality()).unwrap()
    }
    pub fn owl_minQualifiedCardinality() -> IRI {
        iri_to_js_iri(&crate::owl::well_known::owl_minQualifiedCardinality()).unwrap()
    }
    pub fn owl_NamedIndividual() -> IRI {
        iri_to_js_iri(crate::owl::well_known::owl_NamedIndividual().as_iri()).unwrap()
    }
    pub fn owl_onClass() -> IRI {
        iri_to_js_iri(&crate::owl::well_known::owl_onClass()).unwrap()
    }
    pub fn owl_onDataRange() -> IRI {
        iri_to_js_iri(&crate::owl::well_known::owl_onDataRange()).unwrap()
    }
    pub fn owl_oneOf() -> IRI {
        iri_to_js_iri(&crate::owl::well_known::owl_oneOf()).unwrap()
    }
    pub fn owl_onProperties() -> IRI {
        iri_to_js_iri(&crate::owl::well_known::owl_onProperties()).unwrap()
    }
    pub fn owl_onProperty() -> IRI {
        iri_to_js_iri(&crate::owl::well_known::owl_onProperty()).unwrap()
    }
    pub fn owl_someValuesFrom() -> IRI {
        iri_to_js_iri(&crate::owl::well_known::owl_someValuesFrom()).unwrap()
    }
    pub fn owl_unionOf() -> IRI {
        iri_to_js_iri(&crate::owl::well_known::owl_unionOf()).unwrap()
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
        iri_to_js_iri(crate::owl::well_known::xsd_string().as_iri()).unwrap()
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
    match serde_json::to_string(value) {
        Ok(s) => match JSON::parse(&s) {
            Ok(value) => Some(value.unchecked_into()),
            Err(e) => {
                error_1(
                    &format!("Failed to create JS value from value {:?}: {:?}", value, e).into(),
                );
                None
            }
        },
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
    known: Array<Declaration>
}
"#;

#[wasm_bindgen(typescript_custom_section)]
const TRIPLE: &'static str = r#"
interface Triple {
    subject: IRI,
    predicate: IRI,
    value: LiteralOrIRI
}
"#;

#[wasm_bindgen]
pub fn parse_triple(triple: String) -> Option<Triple> {
    let t = crate::parser::triple::parse_triple(&triple);
    match serde_json::to_string(&t) {
        Ok(s) => match JSON::parse(&s) {
            Ok(value) => Some(value.unchecked_into()),
            Err(e) => {
                error_1(
                    &format!(
                        "Failed to create JS triple from value {:?}: {:?}",
                        triple, e
                    )
                    .into(),
                );
                None
            }
        },
        Err(e) => {
            error_1(&format!("Failed to create JS triple from value {:?}: {}", triple, e).into());
            None
        }
    }
}
