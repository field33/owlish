use owlish::{
    api::Ontology,
    owl::{Axiom, DataPropertyAssertion, IRI},
    parser::ParserOptions,
};

mod reification;

#[test]
fn data_properties() {
    env_logger::try_init().ok();
    let turtle = include_str!("./data_properties.ttl");

    harriet::TurtleDocument::parse_full(turtle)
        .map_err(|e| format!("{}...", &format!("{:?}", e)[..200]))
        .expect("Could not parse with harriet");

    let o = Ontology::parse(
        turtle,
        ParserOptions::builder()
            .known(owlish::owl::Declaration::DataProperty {
                iri: IRI::new("http://field33.com/ontologies/test/TestDataProperty")
                    .unwrap()
                    .into(),
                annotations: vec![],
            })
            .build(),
    )
    .unwrap();
    // println!("{:#?}", o);
    assert_eq!(o.declarations().len(), 2);
    assert_eq!(o.axioms().len(), 2);
    assert_eq!(
        o.axioms()[0],
        Axiom::DataPropertyAssertion(DataPropertyAssertion::new(
            IRI::new("http://field33.com/ontologies/test/TestDataProperty")
                .unwrap()
                .into(),
            IRI::new("http://field33.com/dataset/test").unwrap().into(),
            "29.25".into(),
            vec![],
                vec![],
        ))
    )
}
