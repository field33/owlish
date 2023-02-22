use std::collections::HashMap;

use crate::serializer::IriToTtl;

use super::IRI;

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum IRIList {
    IRI(IRI),
    List(Vec<IRI>),
}

impl IriToTtl for IRIList {
    fn ttl(&self, imports: &HashMap<String, IRI>) -> String {
        match self {
            IRIList::IRI(iri) => iri.ttl(imports),
            IRIList::List(iris) => format!(
                "( {} )",
                iris.iter()
                    .map(|iri| iri.ttl(imports))
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::owl::IRI;

    use super::IRIList;

    #[test]
    fn serialize() {
        let il = IRIList::List(vec![IRI::new("http://test#").unwrap()]);
        assert_eq!(serde_json::to_string(&il).unwrap(), "{\"List\":[{\"_type\":\"IRI\",\"string\":\"http://test#\"}]}");
        let il = IRIList::IRI(IRI::new("http://test#").unwrap());
        assert_eq!(serde_json::to_string(&il).unwrap(), "{\"IRI\":{\"_type\":\"IRI\",\"string\":\"http://test#\"}}");
    }
}

#[cfg(feature = "wasm")]
mod wasm {
    use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen(typescript_custom_section)]
    const WASM_API: &'static str = r#"
/**
 * Either a single IRI or a list of IRIs.
 */
export interface IRIList {
    IRI?: IRI,
    List?: IRI[],
}
"#;
}
