#![cfg(test)]
//! One shared span convention reaches answers and nested annotation detail.
#[path = "../../loopback/src/lib.rs"]
pub mod loopback;

use loopback::{KEY, Loopback};
use serde_json::json;
use thinkthen::results::AnyRow;
use thinkthen::{BackendName, Key, OffsetUnit, Request, Settings};

#[test]
fn both_wires_translate_annotation_answers_and_detail_without_surface_arithmetic() {
    let text = "é🦀\r\n E\u{301}lise";
    let case = json!({"replies":[
        {"criterion":"BEGIN","contains":"[[E\u{301}lise]]","distribution":{"BEGIN":0,"END":0,"INSIDE":0,"OUT":0,"SINGLE":1}},
        {"criterion":"BEGIN","distribution":{"BEGIN":0,"END":0,"INSIDE":0,"OUT":1,"SINGLE":0}},
        {"criterion":"person","distribution":{"person":1,"none of these":0}},
        {"distribution":{"u1":0.1,"u2":0.9,"none":0}}
    ]});
    let request=Request::from_json(&json!({"question":{"function":"annotate","field":"/body","questions":{"names":{"function":"recognize","kinds":["person"]},"other_names":{"function":"recognize","kinds":["person"],"field":"/other"},"line":{"function":"find","question":"Which line?"}}},"records":[{"body":text,"other":"abcd E\u{301}lise"}]}).to_string()).unwrap();
    for backend in [BackendName::SystemOne, BackendName::OpenAi] {
        let server = Loopback::case(&case).unwrap();
        let engine = Settings::default()
            .with_backend(backend)
            .url(server.base())
            .api_key(Key::new(KEY).unwrap())
            .batch(2_u64.try_into().unwrap())
            .engine(|_| None)
            .unwrap();
        for (unit, start, end, line_start, piece_start, piece_end) in [
            (OffsetUnit::CodePoints, 5, 11, 4, 1, 2),
            (OffsetUnit::CodePointsInclusive, 6, 11, 5, 2, 2),
            (OffsetUnit::Utf16, 6, 12, 5, 1, 3),
            (OffsetUnit::Utf8, 9, 16, 8, 2, 6),
        ] {
            let call = engine.open_request(&request, unit).unwrap();
            call.sender().close();
            let rows: Vec<_> = call.rows().collect::<Result<_, _>>().unwrap();
            assert_eq!(rows.len(), 1);
            let AnyRow::Annotate(form) = &rows[0] else {
                panic!("expected form")
            };
            let entries = form.answer().unwrap();
            let names = entries.recognize("names").unwrap().into_answer().unwrap();
            let line = entries
                .find("line")
                .unwrap()
                .into_answer()
                .unwrap()
                .unwrap();
            assert_eq!((names[0].start(), names[0].end()), (start, end));
            assert_eq!((line.start(), line.end()), (line_start, end));
            let thinkthen::results::AnnotationEntry::Recognize(
                thinkthen::results::Outcome::Answered {
                    detail: Some(detail),
                    ..
                },
            ) = entries.get("names").unwrap()
            else {
                panic!("expected detail")
            };
            assert_eq!(
                (detail.pieces()[1].start(), detail.pieces()[1].end()),
                (piece_start, piece_end)
            );
            let detached = entries.get("names").unwrap().clone();
            let other = entries
                .recognize("other_names")
                .unwrap()
                .into_answer()
                .unwrap();
            let other_start = if unit == OffsetUnit::CodePointsInclusive {
                6
            } else {
                5
            };
            let other_end = if unit == OffsetUnit::Utf8 { 12 } else { 11 };
            assert_eq!((other[0].start(), other[0].end()), (other_start, other_end));
            let detached_other = entries.get("other_names").unwrap().clone();

            #[cfg(feature = "native")]
            {
                let (host, coordinates) = thinkthen::native::host_row_json(&rows[0]).unwrap();
                let mut decoded: AnyRow = serde_json::from_value(host).unwrap();
                thinkthen::native::restore_row_coordinates(&mut decoded, &coordinates).unwrap();
                assert_eq!(
                    serde_json::to_value(decoded).unwrap(),
                    serde_json::to_value(&rows[0]).unwrap()
                );
            }
            let row = serde_json::to_value(&rows[0]).unwrap();
            let (start, end, line_start, piece_start, piece_end) = (5, 11, 4, 1, 2);
            assert_eq!(row["answer"]["names"]["answer"][0]["start"], start);
            assert_eq!(row["answer"]["names"]["answer"][0]["end"], end);
            assert_eq!(row["answer"]["names"]["detail"]["names"][0]["start"], start);
            assert_eq!(row["answer"]["names"]["detail"]["names"][0]["end"], end);
            assert_eq!(
                row["answer"]["names"]["detail"]["pieces"][1]["start"],
                piece_start
            );
            assert_eq!(
                row["answer"]["names"]["detail"]["pieces"][1]["end"],
                piece_end
            );
            assert_eq!(row["answer"]["line"]["answer"]["start"], line_start);
            assert_eq!(row["answer"]["line"]["answer"]["end"], end);
            assert_eq!(row["answer"]["other_names"]["answer"][0]["start"], 5);
            drop(rows);
            assert_eq!(detached_other.to_string(), "person: E\u{301}lise (5..11)");
            assert_eq!(detached.to_string(), "person: E\u{301}lise (5..11)");
            assert_eq!(
                serde_json::to_value(detached).unwrap()["answer"][0]["start"],
                5
            );
        }
    }
}

#[test]
fn relation_answers_and_pair_detail_save_the_same_characters_in_every_host_unit() {
    let mut case: serde_json::Value = serde_json::from_str(include_str!(
        "../../../conformance/typed/relate-either.json"
    ))
    .unwrap();
    let replies = case["replies"].as_array_mut().unwrap();
    for prefix in ["[[é]]", "[[🦀]]", "[[é🦀]]"] {
        replies.insert(
            0,
            json!({"criterion":"BEGIN","contains":prefix,
            "distribution":{"BEGIN":0,"END":0,"INSIDE":0,"OUT":1,"SINGLE":0}}),
        );
    }
    case["request"]["records"] = json!(["é🦀 Ada Acme Bea"]);
    let request = Request::from_json(&case["request"].to_string()).unwrap();
    let server = Loopback::case(&case).unwrap();
    let engine = Settings::default()
        .with_backend(BackendName::SystemOne)
        .url(server.base())
        .api_key(Key::new(KEY).unwrap())
        .engine(|_| None)
        .unwrap();
    let mut saved = None;
    for (unit, prefix) in [
        (OffsetUnit::CodePoints, 3),
        (OffsetUnit::Utf16, 4),
        (OffsetUnit::Utf8, 7),
    ] {
        let call = engine.open_request(&request, unit).unwrap();
        call.sender().close();
        let row = call.rows().next().unwrap().unwrap();
        let AnyRow::Relate(value) = &row else {
            panic!("expected relations")
        };
        let answer = value.answer().unwrap();
        assert!(!answer.is_empty());
        assert_eq!(
            (answer[0].source().start(), answer[0].source().end()),
            (prefix, prefix + 3)
        );
        let thinkthen::results::Outcome::Answered {
            detail: Some(detail),
            ..
        } = value.outcome()
        else {
            panic!("expected relation detail")
        };
        assert_eq!(
            (detail.names()[0].start(), detail.names()[0].end()),
            (prefix, prefix + 3)
        );
        assert_eq!(detail.pairs()[0].source().start(), prefix);
        let encoded = serde_json::to_value(&row).unwrap();
        assert_eq!(encoded["answer"][0]["source"]["start"], 3);
        assert_eq!(encoded["detail"]["pairs"][0]["source"]["start"], 3);
        if let Some(previous) = &saved {
            assert_eq!(&encoded, previous);
        }
        saved = Some(encoded);
    }
}
