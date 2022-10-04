use owlish::api::Ontology;

#[test]
fn apqc() {
    env_logger::try_init().ok();
    let turtle = include_str!("./apqc.ttl");

    harriet::TurtleDocument::parse_full(turtle)
        .map_err(|e| format!("{}...", &format!("{:?}", e)[..200]))
        .expect("Could not parse with harriet");

    let o = Ontology::parse(turtle, Default::default()).unwrap();

    assert_eq!(o.declarations().len(), 174);
    assert_eq!(o.axioms().len(), 314);
}
