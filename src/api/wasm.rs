#![allow(non_snake_case)]

use super::Ontology;
use js_sys::Array;
use wasm_bindgen::{prelude::*, JsCast};

#[wasm_bindgen]
impl Ontology {
    /// Create an ontology based on a turtle formatted string.
    pub fn parseTurtle(ttl: String) -> Option<Ontology> {
        Ontology::parse(&ttl).ok()
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
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "IRI")]
    pub type IRI;
}

#[wasm_bindgen]
pub fn Iri(iri: &str) -> Option<IRI> {
    crate::owl::IRI::new(iri)
        .ok()
        .and_then(|iri| JsValue::from_serde(&iri).ok())
        .and_then(|iri| iri.dyn_into().ok())
}
