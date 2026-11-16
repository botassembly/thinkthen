#![cfg(test)]
//! The ideal Rust API returns ordinary enum and scalar values through public calls.
#[path = "../../loopback/src/lib.rs"]
pub mod loopback;
use thinkthen::{Choose, Decide, Key, Rank, Score, Settings, Tag};
thinkthen::choices! {
    /// Ticket topics in declaration order.
    pub enum Topic { Billing = "billing", Shipping = "shipping", Other = "other" }
}
#[test]
fn typed_domains_project_owned_rows_and_keep_original_records() {
    let server = loopback::Loopback::guide_scenario("yes").unwrap();
    let engine = Settings::default()
        .url(server.base())
        .api_key(Key::new(loopback::KEY).unwrap())
        .engine(|_| None)
        .unwrap();
    let question = Choose::<Topic>::new("What is the message about?").unwrap();
    let row = engine.run(&question, "Please refund my order.").unwrap();
    let answer: Topic = row.answer().unwrap();
    assert_eq!(answer, Topic::Billing);
    assert_eq!(row.to_string(), "billing");
    assert!(row.is_answered());
    assert!(!row.is_unsure());
    assert!(!row.is_failed());
    assert_eq!(row.detail().unwrap().labels().len(), 3);
    assert_eq!(serde_json::to_value(&row).unwrap()["answer"], "billing");
    let restored: thinkthen::Row<Choose<Topic>> =
        serde_json::from_str(&serde_json::to_string(&row).unwrap()).unwrap();
    assert_eq!(restored, row);
    let checks = engine
        .rows(&question, vec!["first".to_owned(), "second".to_owned()])
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    assert_eq!(checks[1].item(), "second");
    let record = thinkthen::Record::from_json(r#"{ "id" : "push", "text" : "body" }"#);
    let original = record.original().1.to_vec();
    let call = engine.open(&question).unwrap();
    let sender = call.sender();
    sender.send(record).unwrap();
    sender.close();
    let pushed = call.rows().next().unwrap().unwrap();
    assert_eq!(pushed.answer().unwrap(), Topic::Billing);
    assert_eq!(pushed.id().unwrap().as_str(), Some("push"));
    assert_eq!(pushed.item().original().1, original);

    let tags = engine
        .run(&Tag::<Topic>::new("Which topics appear?").unwrap(), "body")
        .unwrap();
    let _: &[Topic] = tags.answer().unwrap();
    assert_eq!(tags.to_string(), "billing\tshipping\tother");
    let urgency = engine
        .run(&Score::<Topic>::new("How urgent?").unwrap(), "body")
        .unwrap();
    let score: f64 = urgency.answer().unwrap();
    assert!((0.0..=1.0).contains(&score));
    let ranked = engine
        .rank(
            &Rank::<Topic>::new("How urgent?").unwrap(),
            vec!["first".to_owned(), "second".to_owned()],
        )
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let place: usize = ranked[0].answer().unwrap();
    assert_eq!(place, 1);
    let owned_place: usize = ranked[0].clone().into_answer().unwrap();
    assert_eq!(owned_place, 1);
    assert_eq!(ranked[0].item(), "first");
    let yes: bool = engine
        .run(&Decide::new("Ready?").unwrap(), "body")
        .unwrap()
        .answer()
        .unwrap();
    assert!(yes);
}

#[test]
fn typed_entries_and_saved_domains_refuse_call_mistakes_before_execution() {
    use thinkthen::{AnnotatableQuestion, Annotate, Code, NoAnswer, Threshold, Toolbox};
    for scenario in [
        "yes",
        "topic-unsure",
        "topic-failed",
        "refund-failed",
        "urgency-failed",
        "failed",
        "refund-unsure",
    ] {
        let server = loopback::Loopback::guide_scenario(scenario).unwrap();
        let engine = Settings::default()
            .url(server.base())
            .api_key(Key::new(loopback::KEY).unwrap())
            .engine(|_| None)
            .unwrap();
        let questions = Annotate::new([
            (
                "refund",
                AnnotatableQuestion::from(
                    Decide::new("Refund?")
                        .unwrap()
                        .threshold(Threshold::band(0.2, 0.8).unwrap()),
                ),
            ),
            ("topic", Choose::<Topic>::new("Topic?").unwrap().into()),
            ("urgency", Score::<Topic>::new("Urgent?").unwrap().into()),
        ])
        .unwrap();
        let form = engine.run(&questions, "body").unwrap();
        assert!(
            form.is_answered(),
            "{scenario}: parent should stay answered"
        );
        assert!(
            form.error().is_none(),
            "{scenario}: parent has no own failure"
        );
        let entries = form.answer().unwrap();
        assert_eq!(
            entries.decide("refund").unwrap().is_failed(),
            scenario == "refund-failed" || scenario == "failed"
        );
        assert_eq!(
            entries.choose::<Topic>("topic").unwrap().is_failed(),
            scenario == "topic-failed" || scenario == "failed"
        );
        assert_eq!(
            entries.score("urgency").unwrap().is_failed(),
            scenario == "urgency-failed" || scenario == "failed"
        );
        if scenario == "refund-unsure" {
            assert!(entries.decide("refund").unwrap().is_unsure());
            assert_eq!(form.clone().err_if_failed().unwrap(), form);
        }
        let count = server.count();
        assert_eq!(
            engine.run(&questions, "").unwrap_err().code(),
            Code::RecordEmpty
        );
        let failed_parent = engine
            .rows(&questions, [""])
            .unwrap()
            .next()
            .unwrap()
            .unwrap();
        assert!(failed_parent.is_failed());
        assert_eq!(failed_parent.error().unwrap().code(), Code::RecordEmpty);
        assert_eq!(server.count(), count, "blank parent reached the service");
        let topic = entries.choose::<Topic>("topic").unwrap();
        if scenario == "yes" {
            assert_eq!(topic.answer().unwrap(), Topic::Billing);
        }
        if scenario == "topic-unsure" {
            assert!(topic.is_unsure());
            assert!(matches!(topic.answer(), Err(NoAnswer::Unsure { .. })));
            assert_eq!(form.clone().err_if_failed().unwrap(), form);
        }
        if scenario.ends_with("failed") {
            assert!(form.clone().err_if_failed().is_err());
        }
        assert_eq!(
            entries.decide("absent").unwrap_err().code(),
            Code::UsageName
        );
        assert_eq!(
            entries.decide("topic").unwrap_err().code(),
            Code::UsageFunction
        );
        let stored = serde_json::to_value(&topic).unwrap();
        assert!(!stored.as_object().unwrap().contains_key("position"));
        let restored: thinkthen::Entry<Choose<Topic>> = serde_json::from_value(stored).unwrap();
        assert_eq!(restored, topic);
    }
    let toolbox=Toolbox::from_json(r#"{"entries":{"topic":{"question":{"function":"choose","question":"Topic?","choices":["billing","shipping","other"]}}}}"#).unwrap();
    let _: Choose<Topic> = toolbox.choose("topic").unwrap();
    assert_eq!(
        toolbox.decide("topic").unwrap_err().code(),
        Code::UsageFunction
    );
    assert_eq!(
        toolbox.decide("absent").unwrap_err().code(),
        Code::UsageName
    );
    for labels in [
        ["shipping", "billing", "other"],
        ["billing", "shipping", "outside"],
    ] {
        let saved = serde_json::json!({"entries":{"topic":{"question":{"function":"choose","question":"Topic?","choices":labels}}}});
        let toolbox = Toolbox::from_json(&saved.to_string()).unwrap();
        assert_eq!(
            toolbox.choose::<Topic>("topic").unwrap_err().code(),
            Code::UsageQuestion
        );
        let mut question = saved["entries"]["topic"]["question"].clone();
        question.as_object_mut().unwrap().remove("function");
        assert!(serde_json::from_value::<Choose<Topic>>(question).is_err());
    }
}

#[test]
fn enum_result_equality_excludes_mutable_nonclone_originals() {
    use std::sync::{Arc, Mutex};
    #[derive(Debug)]
    struct Ticket(Arc<Mutex<String>>);
    impl From<&Ticket> for thinkthen::Record {
        fn from(_: &Ticket) -> Self {
            Self::text("same judged body")
        }
    }
    let server = loopback::Loopback::guide_scenario("yes").unwrap();
    let engine = Settings::default()
        .url(server.base())
        .api_key(Key::new(loopback::KEY).unwrap())
        .engine(|_| None)
        .unwrap();
    let question = Choose::<Topic>::new("Topic?").unwrap();
    let first = Arc::new(Mutex::new("first caller object".to_owned()));
    let second = Arc::new(Mutex::new("second caller object".to_owned()));
    let left = engine
        .rows(&question, vec![Ticket(first.clone())])
        .unwrap()
        .next()
        .unwrap()
        .unwrap();
    let right = engine
        .rows(&question, vec![Ticket(second.clone())])
        .unwrap()
        .next()
        .unwrap()
        .unwrap();
    assert!(Arc::ptr_eq(&left.item().0, &first));
    assert!(Arc::ptr_eq(&right.item().0, &second));
    assert_eq!(left, right);
    *first.lock().unwrap() = "mutated after call".to_owned();
    assert_eq!(left, right);
}

#[test]
fn typed_form_entry_projection_keeps_native_and_saved_coordinates_distinct() {
    use thinkthen::{AnnotatableQuestion, Annotate, Find};
    let server = loopback::Loopback::guide_scenario("yes").unwrap();
    let engine = Settings::default()
        .url(server.base())
        .api_key(Key::new(loopback::KEY).unwrap())
        .engine(|_| None)
        .unwrap();
    let question = Annotate::new([
        (
            "line",
            AnnotatableQuestion::from(Find::new("Action?").unwrap()),
        ),
        ("topic", Choose::<Topic>::new("Topic?").unwrap().into()),
    ])
    .unwrap();
    let text = "😀\nPlease refund my order.";
    let form = engine.run(&question, text).unwrap();
    let entries = form.answer().unwrap();
    assert_eq!(
        entries.choose::<Topic>("topic").unwrap().answer().unwrap(),
        Topic::Billing
    );
    let line = entries.find("line").unwrap();
    let span = line.answer().unwrap().unwrap();
    assert_eq!(span.start(), 5);
    assert_eq!(&text[span.start()..span.end()], "Please refund my order.");
    let saved = serde_json::to_value(&line).unwrap();
    assert_eq!(saved["answer"]["start"], 2);
    let loaded: thinkthen::Entry<Find> = serde_json::from_value(saved.clone()).unwrap();
    assert_eq!(loaded.answer().unwrap().unwrap().start(), 2);
    assert_eq!(serde_json::to_value(&loaded).unwrap(), saved);
    assert_eq!(line, loaded);
}

#[test]
fn kept_stream_preserves_kept_order_and_one_terminal_failure() {
    use futures::StreamExt;
    use thinkthen::{Filter, Record};
    struct Ticket {
        text: &'static str,
        _pin: std::marker::PhantomPinned,
    }
    impl From<&Ticket> for Record {
        fn from(value: &Ticket) -> Self {
            if value.text == "missing" {
                Self::missing()
            } else {
                Self::text(value.text)
            }
        }
    }
    futures::executor::block_on(async {
        let server = loopback::Loopback::guide_scenario("yes").unwrap();
        let engine = Settings::default()
            .url(server.base())
            .api_key(Key::new(loopback::KEY).unwrap())
            .engine(|_| None)
            .unwrap();
        let records = ["missing", "first", "missing", "last"]
            .into_iter()
            .map(|text| Ticket {
                text,
                _pin: std::marker::PhantomPinned,
            })
            .collect::<Vec<_>>();
        let mut stream = engine
            .filter(&Filter::new("Keep?").unwrap(), records)
            .unwrap()
            .into_stream();
        assert_eq!(stream.next().await.unwrap().unwrap().text, "first");
        assert_eq!(stream.next().await.unwrap().unwrap().text, "last");
        let failure = match stream.next().await.unwrap() {
            Err(failure) => failure,
            Ok(_) => panic!("failed input was kept"),
        };
        assert_eq!(
            failure
                .rows()
                .iter()
                .map(|row| row.position())
                .collect::<Vec<_>>(),
            [1, 3]
        );
        assert!(stream.next().await.is_none());
    });
}

#[test]
fn choice_macro_uses_standard_types_even_when_caller_names_are_shadowed() {
    type Result<T> = std::result::Result<T, ()>;
    type String = ();
    let _: Result<String> = Ok(());
    thinkthen::choices! { enum Word { First="first", Second="second" } }
    assert_eq!(
        <Word as thinkthen::questions::Choice>::from_label(std::string::String::from("first"))
            .unwrap(),
        Word::First
    );
}

#[test]
fn printing_names_uses_saved_characters_while_executed_getters_keep_bytes() {
    let server = loopback::Loopback::guide_scenario("relations").unwrap();
    let engine = Settings::default()
        .url(server.base())
        .api_key(Key::new(loopback::KEY).unwrap())
        .engine(|_| None)
        .unwrap();
    let names = engine
        .run(
            &thinkthen::Recognize::new(["person", "organization"]).unwrap(),
            "Élodie works for Example Group.",
        )
        .unwrap();
    assert_eq!(names.answer().unwrap()[0].end(), 7);
    assert_eq!(names.answer().unwrap()[1].start(), 18);
    assert_eq!(
        names.to_string(),
        "person: Élodie (0..6)\torganization: Example Group (17..30)"
    );
    assert_eq!(
        serde_json::to_value(&names).unwrap()["answer"][1]["start"],
        17
    );
}

#[test]
fn scalar_failures_are_call_errors_but_one_item_bulk_calls_keep_failed_rows() {
    use thinkthen::{Code, NoAnswer, Record, Threshold};
    for scenario in ["yes", "unsure", "failed"] {
        let server = loopback::Loopback::guide_scenario(scenario).unwrap();
        let engine = Settings::default()
            .url(server.base())
            .api_key(Key::new(loopback::KEY).unwrap())
            .retry(
                thinkthen::Retry::new(
                    0,
                    std::time::Duration::ZERO,
                    std::time::Duration::from_secs(30),
                )
                .unwrap(),
            )
            .engine(|_| None)
            .unwrap();
        let question = Decide::new("Refund?")
            .unwrap()
            .threshold(Threshold::band(0.2, 0.8).unwrap());
        let result = engine.run(&question, "body");
        let bulk = engine
            .rows(&question, ["body"])
            .unwrap()
            .next()
            .unwrap()
            .unwrap();
        if scenario == "failed" {
            let error = result.unwrap_err();
            assert_eq!(&error, bulk.error().unwrap());
            assert_eq!(error.code(), Code::BackendStatus);
            assert_eq!(error.code().family(), thinkthen::Family::Backend);
            assert_eq!(error.http_status(), Some(503));
            assert_eq!(
                error.message(),
                "the backend answered status 503: {\"error\":\"unavailable\"}"
            );
            assert!(matches!(bulk.answer(), Err(NoAnswer::Failed(_))));
            assert!(matches!(bulk.into_answer(), Err(NoAnswer::Failed(_))));
        } else {
            let row = result.unwrap();
            assert!(!row.is_failed());
            assert!(row.error().is_none());
            assert_eq!(row.answer(), row.clone().into_answer());
            assert_eq!(row.is_unsure(), scenario == "unsure");
        }
        let count = server.count();
        let error = engine.run(&question, " \t\n").unwrap_err();
        assert_eq!(error.code(), Code::RecordEmpty);
        assert_eq!(server.count(), count, "blank input reached the service");
        assert_eq!(
            engine.run(&question, Record::missing()).unwrap_err().code(),
            Code::RecordMissing
        );
        assert_eq!(server.count(), count, "missing input reached the service");
    }
}
