#![cfg(test)]
//! One shared span convention reaches answers and nested annotation detail.
use loopback::{KEY, Loopback};
use serde_json::json;
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
    let request=Request::from_json(&json!({"question":{"function":"annotate","evidence":"/body","questions":{"names":{"function":"recognize","kinds":["person"]},"line":{"function":"find","question":"Which line?"}}},"records":[{"body":text}]}).to_string()).unwrap();
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
            let row = serde_json::to_value(&rows[0]).unwrap();
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
        }
    }
}
