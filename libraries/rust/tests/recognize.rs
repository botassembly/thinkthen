//! `recognize` and `relate` on the Rust surface, under the null backend.
//!
//! The samples are the Rust sections of `recognize-surfaces.md`, run
//! against the stand-in's recordings. The offset proof is the strictest
//! on this surface: the contract counts code points and Rust slices
//! bytes, so the emoji case decides. Run through `./check.sh`, which sets
//! `ENGINE_NULL=1`.

use std::time::Duration;

use thinkthen::{
    Cancel, Edge, Engine, ErrorKind, Kind, Options, Recognize, Relate, byte_range, name_in,
};

fn engine() -> Engine {
    Engine::from_env().expect("the stand-in never fails to build")
}

/// The deck sentence, case `28-recognize-C01`.
const SENTENCE: &str = "Maria Chen joined Northwind Freight in Chicago last spring.";

/// Case `68-recognize-C41`: the accented letter and the emoji before the
/// name, built for the per-host offset proofs.
const EMOJI: &str = "Le café 😀 Maria Chen arrived.";

/// The four alerts behind `69-relate-R01-demo`.
const ALERTS: [&str; 4] = [
    "Checkout returns 500 at the payment step.",
    "Card charges are failing for every customer.",
    "The nightly export ran two hours late.",
    "The payments database ran out of disk space.",
];

/// The slide's ask meets a recordings gap, and the gap is the finding.
///
/// The Rust section of `recognize-surfaces.md` asks `works_for` and
/// `located_in` over the deck sentence. The recording behind that sentence
/// covers `works_for` and `based_in`, so `located_in` answers the usage
/// kind naming what the recording does cover. Filed in NOTES.md as a
/// recordings-coverage finding for the recognize package; the deck call
/// still stands as the acceptance for the real engine.
#[test]
fn the_slide_ask_meets_the_recordings_gap() -> Result<(), Box<dyn std::error::Error>> {
    let tt = engine();
    let ask = Recognize::new()
        .kinds(["person", "organization", "place"])
        .relation(
            "works_for",
            Kind::named("person"),
            Kind::named("organization"),
        )?
        .relation("located_in", Kind::Any, Kind::named("place"))?;
    let error = tt
        .recognize(&ask, SENTENCE)
        .err()
        .expect("the recording does not carry located_in on the deck sentence");
    assert_eq!(error.kind, ErrorKind::Usage);
    assert!(
        error.message.contains("located_in") && error.message.contains("works_for"),
        "the refusal names the missing and the covered rules: {}",
        error.message
    );
    Ok(())
}

/// The recorded shape answers fully: entities, the works_for relation, and
/// every name slicing out of the text in byte indexing.
#[test]
fn the_recorded_shape_answers_fully() -> Result<(), Box<dyn std::error::Error>> {
    let tt = engine();
    let ask = Recognize::from_json(
        r#"{"kinds": ["person", "organization", "place"],
            "relations": [{"name": "works_for", "from": "person", "to": "organization"},
                          {"name": "based_in", "from": "organization", "to": "place"}],
            "threshold": 0.5, "relation_threshold": 0.5}"#,
    )?;
    let found = tt.recognize(&ask, SENTENCE)?;
    let names: Vec<(&str, &str, usize, usize)> = found
        .entities
        .iter()
        .map(|entity| {
            (
                entity.text.as_str(),
                entity.kind.as_str(),
                entity.start,
                entity.end,
            )
        })
        .collect();
    assert_eq!(
        names,
        vec![
            ("Maria Chen", "person", 0, 10),
            ("Northwind Freight", "organization", 18, 35),
            ("Chicago", "place", 39, 46),
        ]
    );
    assert_eq!(found.relations.len(), 1, "the recording holds one relation");
    let relation = &found.relations[0];
    assert_eq!(relation.name, "works_for");
    assert_eq!((relation.source, relation.target), (1, 2));
    assert!((relation.probability - 1.0).abs() < 1e-9);
    for entity in &found.entities {
        assert_eq!(name_in(entity, SENTENCE), entity.text, "byte slicing holds");
    }
    assert_eq!(byte_range(&found.entities[0], SENTENCE), 0..10);
    Ok(())
}

/// The emoji case: code-point offsets, byte slicing, and why the raw slice
/// is wrong.
#[test]
fn the_emoji_case_slices_in_bytes() -> Result<(), Box<dyn std::error::Error>> {
    let tt = engine();
    let ask = Recognize::from_json(r#"{"kinds": ["person"], "relations": []}"#)?;
    let found = tt.recognize(&ask, EMOJI)?;
    assert_eq!(found.entities.len(), 1);
    let entity = &found.entities[0];
    assert_eq!(entity.text, "Maria Chen");
    // The contract counts code points: the name starts at code point 10.
    assert_eq!((entity.start, entity.end), (10, 20));
    // Rust slices bytes: é is two bytes and 😀 is four, so the name starts
    // at byte 14. The code-point offset is not a byte offset — byte 10 sits
    // inside the emoji — and that is exactly why name_in exists.
    assert_eq!(byte_range(entity, EMOJI), 14..24);
    assert!(!EMOJI.is_char_boundary(10), "byte 10 cuts the emoji");
    assert!(EMOJI.is_char_boundary(14), "byte 14 starts the name");
    assert_eq!(name_in(entity, EMOJI), "Maria Chen");
    Ok(())
}

/// The any-kind end: `Kind::Any` is Rust's `"*"`, and a rule that uses it
/// answers end to end.
#[test]
fn the_any_kind_end_answers() -> Result<(), Box<dyn std::error::Error>> {
    let tt = engine();
    assert_eq!(Kind::Any.to_string(), "*");
    let ask = Recognize::new().kinds(["place"]).relation(
        "located_in",
        Kind::Any,
        Kind::named("place"),
    )?;
    let found = tt.recognize(&ask, "The road from Hull to Leeds was closed.")?;
    let names: Vec<&str> = found
        .entities
        .iter()
        .map(|entity| entity.text.as_str())
        .collect();
    assert_eq!(names, vec!["Hull", "Leeds"]);
    assert!(
        found.relations.is_empty(),
        "the recording found no located_in pair"
    );
    Ok(())
}

/// The relate slide runs as drawn, and the threshold picks the demo's
/// first edge.
#[test]
fn the_relate_slide_runs_as_drawn() -> Result<(), Box<dyn std::error::Error>> {
    let tt = engine();
    let ask = Relate::new()
        .relation("caused_by", Kind::Any, Kind::Any)?
        .either("same_as", Kind::Any)?;
    let edges: Vec<Edge> = tt.relate(&ask, &ALERTS)?;
    let seen: Vec<(&str, u64, u64)> = edges
        .iter()
        .map(|edge| (edge.name.as_str(), edge.source, edge.target))
        .collect();
    assert_eq!(
        seen,
        vec![
            ("caused_by", 1, 2),
            ("caused_by", 1, 4),
            ("caused_by", 2, 4),
            ("caused_by", 3, 4),
        ]
    );
    let probabilities: Vec<f64> = edges.iter().map(|edge| edge.probability).collect();
    for (got, wanted) in probabilities.iter().zip([0.59, 0.94, 0.94, 0.84]) {
        assert!((got - wanted).abs() < 1e-9, "expected {wanted}, got {got}");
    }
    // The Python section's comment, 0.9: the 0.59 edge drops and the first
    // surviving edge is the 1-to-4 one at 0.94.
    let strict = ask.threshold(0.9)?;
    let edges = tt.relate(&strict, &ALERTS)?;
    assert_eq!(edges[0].source, 1);
    assert_eq!(edges[0].target, 4);
    assert!((edges[0].probability - 0.94).abs() < 1e-9);
    Ok(())
}

/// `relate` refuses 256 records with the usage kind, before anything else.
#[test]
fn relate_refuses_more_than_255() -> Result<(), Box<dyn std::error::Error>> {
    let tt = engine();
    let ask = Relate::new().relation("caused_by", Kind::Any, Kind::Any)?;
    let records: Vec<&str> = vec!["one alert"; 256];
    let error = tt
        .relate(&ask, &records)
        .err()
        .expect("256 is past the limit");
    assert_eq!(error.kind, ErrorKind::Usage);
    assert!(
        error.message.contains("255") && error.message.contains("256"),
        "the refusal names the limit and the count: {}",
        error.message
    );
    Ok(())
}

/// The kinds reachable offline map to the contract's own arms: usage,
/// deadline, and cancelled; backend needs the wire and defect is not
/// fabricable.
#[test]
fn the_reachable_error_kinds_map() -> Result<(), Box<dyn std::error::Error>> {
    let tt = engine();
    let ask = Recognize::new().kinds(["person"]);

    let unknown = tt
        .recognize(&ask, "No recording covers this text.")
        .err()
        .expect("an unrecorded text answers the usage kind");
    assert_eq!(unknown.kind, ErrorKind::Usage);

    let rule = Recognize::new().kinds(["person"]).relation(
        "works_for",
        Kind::Any,
        Kind::Named("company".to_owned()),
    )?;
    let outside = tt
        .recognize(&rule, SENTENCE)
        .err()
        .expect("a named end outside the asked kinds is a usage error");
    assert_eq!(outside.kind, ErrorKind::Usage);

    let spent = tt
        .recognize_opts(&ask, SENTENCE, Options::new().deadline_in(Duration::ZERO))
        .err()
        .expect("a spent deadline answers the deadline kind");
    assert_eq!(spent.kind, ErrorKind::Deadline);

    let token = Cancel::new();
    token.cancel();
    let stopped = tt
        .recognize_opts(&ask, SENTENCE, Options::new().cancel(&token))
        .err()
        .expect("a fired token answers the cancelled kind");
    assert_eq!(stopped.kind, ErrorKind::Cancelled);
    Ok(())
}
