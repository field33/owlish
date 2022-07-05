use crate::owl::{ClassConstructor, IndividualIRI, Regards};

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct SameIndividual(pub IndividualIRI, pub IndividualIRI);
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct DifferentIndividuals(pub IndividualIRI, pub IndividualIRI);
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct ClassAssertion(pub ClassConstructor, pub IndividualIRI);
impl ClassAssertion {
    pub fn class_constructor(&self) -> &ClassConstructor {
        &self.0
    }
    pub fn individual_iri(&self) -> &IndividualIRI {
        &self.1
    }
}

impl Regards for ClassAssertion {
    fn regards(&self, iri: &crate::owl::IRI) -> bool {
        self.individual_iri().as_iri() == iri || self.class_constructor().regards(iri)
    }
}
