#!/bin/sh
API="
// CUSTOM API FROM extend_wasm_pkg.sh
// ------------------------------------------------------

export function matchValue(value, matcher) {
    if (
        matcher.string &&
        matcher.boolean &&
        matcher.dateTime &&
        matcher.number &&
        matcher.raw
    ) {
        switch (value._type) {
            case 'string':
            case 'langString':
                return matcher.string(value)
            case 'raw':
                return matcher.raw(value)
            case 'dateTime':
                return matcher.dateTime(value)
            case 'number':
                return matcher.number(value)
            case 'boolean':
                return matcher.boolean(value)
        }
    } else if (matcher.default) {
        switch (value._type) {
            case 'string':
            case 'langString':
                return matcher.string !== undefined
                    ? matcher.string(value)
                    : matcher.default(value)
            case 'raw':
                return matcher.raw !== undefined
                    ? matcher.raw(value)
                    : matcher.default(value)
            case 'dateTime':
                return matcher.dateTime !== undefined
                    ? matcher.dateTime(value)
                    : matcher.default(value)
            case 'number':
                return matcher.number !== undefined
                    ? matcher.number(value)
                    : matcher.default(value)
            case 'boolean':
                return matcher.boolean !== undefined
                    ? matcher.boolean(value)
                    : matcher.default(value)
            default:
                return matcher.default(value)
        }
    } else {
        throw new Error('Non exhaustive value match!')
    }
}


export function matchAxiom(axiom, matcher) {
    for (const key in axiom) {
        if (matcher[key]) {
            return matcher[key](axiom[key])
            break
        }
    }
}

export function matchDeclaration(declaration, matcher) {
    for (const key in declaration) {
        if (matcher[key]) {
            return matcher[key](declaration[key])
            break
        }
    }
}

export function matchClassConst(axiom, matcher) {
    for (const key in axiom) {
        if (matcher[key]) {
            return matcher[key](axiom[key])
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
