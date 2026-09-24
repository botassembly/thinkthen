use super::super::{Aggregate, Running, config, profile};
use crate::args::{Cli, Command};
use crate::core::{Backend, ModelName, RecognizedName, Url};
use crate::edge::Environment;
use crate::http::Client;
use crate::prepared_request::PREPARATIONS;
use crate::recorder::Recorder;
use clap::Parser as _;
use std::time::Duration;

fn entity(name: &str, kind: &str, start: usize) -> RecognizedName {
    RecognizedName {
        name: name.to_owned(),
        kind: kind.to_owned(),
        start,
        end: start + name.len(),
        strength: 0.9,
    }
}

#[test]
fn recognition_sends_the_chunks_the_settled_relation_prepared() {
    let cli = Cli::try_parse_from([
        "thinkthen",
        "recognize",
        "--url",
        "http://127.0.0.1:9",
        "--timeout",
        "1",
        "--max-retries",
        "0",
        "--relation",
        "works_for=person:organization",
    ])
    .expect("recognize command");
    let Some(Command::Recognize(arguments)) = cli.command else {
        panic!("recognize command");
    };
    let spec = config::settle(&arguments).expect("spec");
    let environment = Environment::default();
    let running = Running {
        common: &arguments.common,
        environment: &environment,
        backend: Backend::from_parts(
            Url::new("http://127.0.0.1:9").expect("url"),
            ModelName::new("local-1").expect("model"),
        ),
        profile: None,
        mismatch: profile::Mismatch::new(None, None),
        recorder: Recorder::of(None, None).expect("no folders"),
        client: Client::new(Duration::from_secs(1), false),
    };
    let entities = [
        entity("Ada", "person", 0),
        entity("Acme", "organization", 8),
    ];
    PREPARATIONS.with(|count| count.set(0));
    let sent = super::recognize(
        &running,
        &spec,
        "Ada met Acme.",
        &entities,
        &mut Aggregate::default(),
    );
    assert!(sent.is_err(), "nothing answers the closed local port");
    assert_eq!(PREPARATIONS.with(std::cell::Cell::get), 1);
}
