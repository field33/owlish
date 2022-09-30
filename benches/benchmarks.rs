use criterion::{criterion_group, criterion_main, Bencher, Criterion};
use owlish::{api::Ontology, owl::IRI, parser::ParserOptions};

fn parser_benchmarks(c: &mut Criterion) {
    env_logger::try_init().ok();
    c.bench_function("35000 individuals, 14000 axioms", |b| {
        parser_bench(b, include_str!("./large.ttl"))
    });
    c.bench_function("2000 individuals, 800 axioms", |b| {
        parser_bench(b, include_str!("./medium.ttl"))
    });
    c.bench_function("1000 individuals, 0 aximos", |b| {
        parser_bench(b, include_str!("./small.ttl"))
    });
}

criterion_group! {
    name = parser;
    config = Criterion::default().significance_level(0.2).sample_size(std::env::var("SAMPLES").ok().and_then(|s| s.parse::<usize>().ok()).unwrap_or(10));
    targets = parser_benchmarks
}
criterion_main!(parser);

pub fn parser_bench(b: &mut Bencher, turtle: &str) {
    harriet::TurtleDocument::parse_full(turtle)
        .map_err(|e| format!("{}...", &format!("{:?}", e)[..200]))
        .expect("Could not parse with harriet");
    b.iter(|| {
        Ontology::parse(
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
    })
}
