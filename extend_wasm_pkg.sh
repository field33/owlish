#!/bin/sh
sed -i 's/declarations(): Array<any>;/declarations(): Array<Declaration>;/g' pkg/owlish.d.ts
sed -i 's/axioms(): Array<any>;/axioms(): Array<Axiom>;/g' pkg/owlish.d.ts

API="
// CUSTOM API FROM extend_wasm_pkg.sh
// ------------------------------------------------------

export function matchAxiom(axiom, matcher) {
    for (const key in axiom) {
        if (matcher[key]) {
            matcher[key](axiom[key])
            break
        }
    }
}

export function matchDeclaration(axiom, matcher) {
    for (const key in axiom) {
        if (matcher[key]) {
            matcher[key](axiom[key])
            break
        }
    }
}

export function matchClassConst(axiom, matcher) {
    for (const key in axiom) {
        if (matcher[key]) {
            matcher[key](axiom[key])
            break
        }
    }
}

export const well_known = {
    xsd_integer: Iri(\"http://www.w3.org/2001/XMLSchema#integer\"),
    xsd_nonNegativeInteger: Iri(\"http://www.w3.org/2001/XMLSchema#nonNegativeInteger\"),
    xsd_minExclusive: Iri(\"http://www.w3.org/2001/XMLSchema#minExclusive\"),
    xsd_minInclusive: Iri(\"http://www.w3.org/2001/XMLSchema#minInclusive\"),
    xsd_maxInclusive: Iri(\"http://www.w3.org/2001/XMLSchema#maxInclusive\"),
    xsd_maxExclusive: Iri(\"http://www.w3.org/2001/XMLSchema#maxExclusive\"),
    rdfs_comment: Iri(\"http://www.w3.org/2000/01/rdf-schema#comment\"),
    rdfs_label: Iri(\"http://www.w3.org/2000/01/rdf-schema#label\"),
    rdf_type: Iri(\"http://www.w3.org/1999/02/22-rdf-syntax-ns#type\"),
    owl_Ontology: Iri(\"http://www.w3.org/2002/07/owl#Ontology\"),
    owl_Thing: Iri(\"http://www.w3.org/2002/07/owl#Thing\"),
    owl_Class: Iri(\"http://www.w3.org/2002/07/owl#Class\"),
    owl_AsymmetricProperty: Iri(\"http://www.w3.org/2002/07/owl#AsymmetricProperty\"),
    owl_SymmetricProperty: Iri(\"http://www.w3.org/2002/07/owl#SymmetricProperty\"),
    owl_ObjectProperty: Iri(\"http://www.w3.org/2002/07/owl#ObjectProperty\"),
}

export function Iri(iri) {
    return {
        _type: \"IRI\",
        string: iri,
    }
}
"
echo "$API" >> ./pkg/owlish.js