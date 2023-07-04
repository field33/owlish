use owlish::api::Ontology;

#[test]
fn large() {
    env_logger::try_init().ok();
    let turtle = include_str!("../benches/large.ttl");

    harriet::TurtleDocument::parse_full(turtle)
        .map_err(|e| format!("{}...", &format!("{:?}", e)[..200]))
        .expect("Could not parse with harriet");

    let o = Ontology::parse(turtle, Default::default()).unwrap();

    assert_eq!(o.declarations().len(), 34452);
    // TODO: not sure how this "correct number was originally determined
    // assert_eq!(o.axioms().len(), 14508);
}
