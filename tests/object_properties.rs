use owlish::{
    api::Ontology,
    owl::{Axiom, ObjectPropertyDomain, ObjectPropertyRange, IRI},
    parser::ParserOptions,
};

#[test]
fn object_properties() {
    env_logger::try_init().ok();
    let turtle = include_str!("./object_properties.ttl");

    harriet::TurtleDocument::parse_full(turtle)
        .map_err(|e| format!("{}...", &format!("{:?}", e)[..200]))
        .expect("Could not parse with harriet");

    let o = Ontology::parse(
        turtle,
        ParserOptions::builder()
            .known(owlish::owl::Declaration::DataProperty(
                IRI::new("http://field33.com/ontologies/test/TestDataProperty")
                    .unwrap()
                    .into(),
                vec![],
            ))
            .build(),
    )
    .unwrap();
    assert_eq!(o.declarations().len(), 197);
    assert_eq!(o.axioms().len(), 382);

    let mut domain_to_check = None;
    let mut range_to_check = None;
    for a in o.axioms().iter() {
        if let Axiom::ObjectPropertyDomain(d) = &a {
            if d.object_property_iri
                .as_iri()
                .as_str()
                .ends_with("AccountabilityFulfillingOf")
            {
                domain_to_check = Some(d)
            }
        }
        if let Axiom::ObjectPropertyRange(r) = &a {
            if r.object_property_iri
                .as_iri()
                .as_str()
                .ends_with("AccountabilityFulfillingOf")
            {
                range_to_check = Some(r)
            }
        }
    }

    assert_eq!(
        domain_to_check,
        Some(&ObjectPropertyDomain::new(
            IRI::new(
                "http://field33.com/ontologies/EXTERNAL_as_innovation/AccountabilityFulfillingOf"
            )
            .unwrap()
            .into(),
            IRI::new("http://field33.com/ontologies/EXTERNAL_as_innovation/Accountability")
                .unwrap()
                .into(),
            vec![]
        ))
    );

    assert_eq!(
        range_to_check,
        Some(&ObjectPropertyRange::new(
            IRI::new(
                "http://field33.com/ontologies/EXTERNAL_as_innovation/AccountabilityFulfillingOf"
            )
            .unwrap()
            .into(),
            IRI::new("http://field33.com/ontologies/EXTERNAL_as_innovation/Purpose")
                .unwrap()
                .into(),
            vec![]
        ))
    );
}
