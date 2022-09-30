use owlish::api::Ontology;

#[test]
fn small() {
    env_logger::try_init().ok();
    let turtle = include_str!("../benches/small.ttl");

    harriet::TurtleDocument::parse_full(turtle)
        .map_err(|e| format!("{}...", &format!("{:?}", e)[..200]))
        .expect("Could not parse with harriet");
        
    let o = Ontology::parse(turtle, Default::default()).unwrap();

    assert_eq!(o.declarations().len(), 1000);
    assert_eq!(o.axioms().len(), 0);
}
