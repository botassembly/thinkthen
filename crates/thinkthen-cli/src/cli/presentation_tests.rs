//! Stays: command-owned argument admission and output rendering need direct dispatch with controlled input and writers.
//! Command hints use retained typed parser facts, never diagnostic sentences.
use super::*;
use clap::Parser as _;

#[test]
fn only_first_record_json_syntax_gets_the_json_lines_hint() {
    let cli = Cli::try_parse_from(["thinkthen", "decide", "Fits?", "--jsonl"]).unwrap();
    let environment = Environment::default();
    for (bytes, at, hint) in [
        (&b"{\"secret-marker\":"[..], 1, true),
        (&b"{\"secret-marker\":"[..], 2, false),
        (&b"{\"secret-marker\":1,\"secret-marker\":2}"[..], 1, false),
    ] {
        let error = crate::RecordReading::new(&[], None, None)
            .unwrap()
            .read_original(bytes, crate::RequestFraming::Jsonl)
            .unwrap_err();
        let failure = Failure::Stopped {
            at,
            finished: at - 1,
            replayed: 0,
            recording: false,
            held: false,
            cause: Box::new(Failure::record(error, true)),
        };
        let failure = told(failure, &cli, &environment);
        assert_eq!(
            matches!(&failure, Failure::Stopped { cause, .. } if matches!(cause.as_ref(), Failure::Usage(text) if *text == hint::NOT_JSON_LINES)),
            hint
        );
        assert!(!format!("{failure:?}").contains("secret-marker"));
    }
    assert_eq!(crate::render::safe_key("/a\n\"b"), "/a\\n\\\"b");
}
