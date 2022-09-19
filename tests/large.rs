use owlish::{api::Ontology, owl::IRI, parser::ParserOptions};

#[test]
fn large() {
    env_logger::try_init().ok();
    let turtle = include_str!("./large.ttl");

    harriet::TurtleDocument::parse_full(turtle)
        .map_err(|e| format!("{}...", &format!("{:?}", e)[..200]))
        .expect("Could not parse with harriet");
    let o = Ontology::parse(
        turtle,
        ParserOptions::builder()
            .known(owlish::owl::Declaration::AnnotationProperty(
                IRI::new("http://query-server.field33.com/ontology/query-field")
                    .unwrap()
                    .into(),
                vec![],
            ))
            .build(),
    )
    .unwrap();
    println!("{:#?}", o);
    // assert_eq!(o.declarations().len(), 1914);
    // assert_eq!(o.axioms().len(), 806);
}
