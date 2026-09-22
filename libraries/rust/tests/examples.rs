//! Every Rust function's example, run as one test.
//!
//! The file `examples.json` is keyed by function: the call and the answer
//! the null backend gives. The site's function pages and surface pages draw
//! their Rust tab from this file (site.md's one source), so an example
//! nobody runs cannot reach the site.
//!
//! Rust cannot evaluate a string, so each example's snippet must also
//! appear verbatim in this file — checked below — and the compiled call
//! beside it must answer what the file says. That is the mechanical tie
//! between the site's text and the run.
//!
//! Run through `./check.sh`, which sets `ENGINE_NULL=1`; a bare
//! `cargo test` names no backend and skips the test with a note.

use std::collections::HashMap;
use std::path::PathBuf;

use serde_json::Value;

use thinkthen::{Annotated, Edge, Engine, Error, Question, Recognize, Recognized, Relate};

mod common;

fn engine() -> Engine {
    Engine::from_env().expect("the stand-in never fails to build")
}

struct Example {
    snippet: String,
    expected: String,
    files: HashMap<String, String>,
}

fn file() -> HashMap<String, Example> {
    let data: Value =
        serde_json::from_str(include_str!("../examples.json")).expect("examples.json parses");
    data["examples"]
        .as_object()
        .expect("examples is an object")
        .iter()
        .map(|(name, held)| {
            let files = held["files"]
                .as_object()
                .into_iter()
                .flatten()
                .map(|(file, content)| {
                    (
                        file.clone(),
                        content.as_str().expect("a file's content is text").to_owned(),
                    )
                })
                .collect();
            (
                name.clone(),
                Example {
                    snippet: held["rust"].as_str().expect("a snippet is text").to_owned(),
                    expected: held["expected"].as_str().expect("an expected answer").to_owned(),
                    files,
                },
            )
        })
        .collect()
}

#[test]
fn every_function_example_answers_as_the_file_says() -> Result<(), Error> {
    if !common::note_missing_env("every_function_example_answers_as_the_file_says") {
        return Ok(());
    }
    let examples = file();
    assert_eq!(examples.len(), 10, "the ten functions are all here");
    let source = include_str!("examples.rs");
    for (name, example) in &examples {
        assert!(
            source.contains(&example.snippet),
            "{name}: the snippet is not in tests/examples.rs: {}",
            example.snippet
        );
    }

    let tt = engine();

    let decided = tt.decide("Is this a complaint?", "I demand a refund today")?.value().unwrap_or(false);
    assert_eq!(format!("{decided}"), examples["decide"].expected);

    let question = Question::choose(
        "Which team owns this?",
        &["the refund desk", "the maybe desk", "anywhere else"],
    )?
    .build()?;
    let picked = tt.choose(&question, "please route this ticket")?.unwrap_or_default();
    assert_eq!(picked, examples["choose"].expected);

    let question = Question::tag("Which words appear?", &["refund", "shipping"])?;
    let labels = tt.tag(&question, "the refund and the shipping")?.join(", ");
    assert_eq!(labels, examples["tag"].expected);

    let question = Question::score("How urgent is this?", &["Routine.", "Soon.", "Immediate."])?;
    let number = tt.score(&question, "maybe later")?.value;
    assert_eq!(format!("{number}"), examples["score"].expected);

    let records = ["good morning", "I demand a refund today"];
    let kept = tt.filter("Is this a complaint?", &records)?.join(" | ");
    assert_eq!(kept, examples["filter"].expected);

    let ranked = tt.rank("Is this a complaint?", &records)?;
    let order: Vec<&str> = ranked.iter().map(|one| records[one.index]).collect();
    assert_eq!(order.join(" | "), examples["rank"].expected);

    let units = ["good morning", "I want a refund"];
    let found = tt.find("Which line asks for money back?", &units)?;
    assert_eq!(
        found.index.map(|place| units[place]).unwrap_or("none"),
        examples["find"].expected
    );

    // annotate: the question file the example names, written into a
    // private folder this test becomes, then left behind.
    let workdir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("examples");
    std::fs::create_dir_all(&workdir).expect("the example folder is made");
    for (file, content) in &examples["annotate"].files {
        std::fs::write(workdir.join(file), content).expect("the question file is written");
    }
    let here = std::env::current_dir().expect("a current directory");
    std::env::set_current_dir(&workdir).expect("the example folder is entered");
    let rows = tt.annotate("form.json", &["maybe later"])?;
    std::env::set_current_dir(here).expect("the current directory is restored");
    let fields: Vec<String> = rows[0]
        .iter()
        .map(|(name, field)| format!("{name}={}", field_text(field)))
        .collect();
    assert_eq!(fields.join(" "), examples["annotate"].expected);

    let found = tt.recognize(&Recognize::from_json("{\"kinds\":[\"person\",\"organization\",\"place\"]}")?, "Maria Chen joined Northwind Freight in Chicago last spring.")?;
    let Recognized { entities, .. } = found;
    let names: Vec<String> = entities
        .iter()
        .map(|one| format!("{}|{}", one.text, one.kind))
        .collect();
    assert_eq!(names.join(" "), examples["recognize"].expected);

    let alerts = [
        "Checkout returns 500 at the payment step.",
        "Card charges are failing for every customer.",
        "The nightly export ran two hours late.",
        "The payments database ran out of disk space.",
    ];
    let edges = tt.relate(&Relate::from_json("{\"relations\":[{\"name\":\"caused_by\",\"source\":\"*\",\"target\":\"*\"}]}")?, &alerts)?;
    let Edge {
        name,
        source,
        target,
        probability,
        ..
    } = &edges[0];
    assert_eq!(
        format!("{name} {source} {target} {probability}"),
        examples["relate"].expected
    );
    Ok(())
}

/// One annotate field as the site shows it: the bare value or `null`, the
/// marker as its ruled JSON.
fn field_text(field: &Annotated) -> String {
    match field {
        Annotated::Decision(answer) => format!("{}", answer.value().unwrap_or(false)),
        Annotated::Choice(picked) => picked.clone().unwrap_or_else(|| "null".to_owned()),
        Annotated::Score(scored) => format!("{}", scored.value),
        Annotated::Tags(labels) => labels.join(","),
        Annotated::Failed(_) => serde_json::to_string(field).expect("the marker is JSON"),
    }
}
