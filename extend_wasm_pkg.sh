#!/bin/sh
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
export class IRI {
    _type
    string
}
"
echo "$API" >> ./pkg/owlish.js
