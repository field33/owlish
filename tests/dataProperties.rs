use owlish::{
    api::Ontology,
    owl::{Axiom, DataPropertyAssertion, IRI},
    parser::ParserOptions,
};

#[test]
fn data_properties() {
    env_logger::try_init().ok();
    let turtle = include_str!("./dataProperties.ttl");

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
    assert_eq!(o.declarations().len(), 2);
    assert_eq!(o.axioms().len(), 2);
    assert_eq!(
        o.axioms()[1],
        Axiom::DataPropertyAssertion(DataPropertyAssertion::new(
            IRI::new("http://field33.com/ontologies/test/TestDataProperty")
                .unwrap()
                .into(),
            IRI::new("http://field33.com/dataset/test").unwrap().into(),
            "29.25".into(),
            vec![]
        ))
    )
}
