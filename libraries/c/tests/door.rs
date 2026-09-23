//! The door's null suite: the typed doors, the error codes a host sees,
//! and the JSON door's replies, all against the in-process backend.
//!
//! Run through `./check.sh`, which sets `THINKTHEN_NULL=1`.

use std::ffi::{c_char, CString};

use thinkthen::thinkthen_engine;

const YES: i32 = 1;
const NO: i32 = 0;
const UNSURE: i32 = 2;

/// A judgment that was never written, so a refusal is visible in the out.
const UNWRITTEN: thinkthen::thinkthen_answer = thinkthen::thinkthen_answer {
    outcome: 7,
    probability: -1.0,
};

/// The header's `THINKTHEN_NO_DEADLINE`.
const NO_DEADLINE: i64 = -1;

/// A null-backend engine. A bare `cargo test` fails here by name, never
/// on an answer it could not get (R2-28).
unsafe fn engine() -> *mut thinkthen_engine {
    assert_eq!(
        std::env::var("THINKTHEN_NULL").as_deref(),
        Ok("1"),
        "run this through check.sh, which sets THINKTHEN_NULL=1"
    );
    unsafe {
        thinkthen::thinkthen_engine_new()
    }
}

unsafe fn message(engine: *const thinkthen_engine) -> String {
    unsafe {
        std::ffi::CStr::from_ptr(thinkthen::thinkthen_error_message(engine))
            .to_string_lossy()
            .into_owned()
    }
}

#[test]
fn the_bare_question_and_the_grammar_settle_the_same() {
    unsafe {
        let engine = engine();
        let bare = CString::new("Does the customer ask for a refund?").expect("static");
        let grammar = CString::new(
            r#"{"decide": "Does the customer ask for a refund?", "threshold": 0.5}"#,
        )
        .expect("static");
        let evidence =
            CString::new("I want a refund for order 9").expect("static");
        let text_len = evidence.as_bytes().len();

        let mut from_bare = thinkthen::thinkthen_answer { outcome: NO, probability: 0.0 };
        let mut from_grammar = thinkthen::thinkthen_answer { outcome: NO, probability: 0.0 };

        let bare_code = thinkthen::thinkthen_decide(
            engine,
            bare.as_ptr(),
            evidence.as_ptr(),
            text_len,
            &mut from_bare,
        );
        let grammar_code = thinkthen::thinkthen_decide(
            engine,
            grammar.as_ptr(),
            evidence.as_ptr(),
            text_len,
            &mut from_grammar,
        );

        assert_eq!(bare_code, 0, "the bare question asks");
        assert_eq!(grammar_code, 0, "the grammar asks");
        assert_eq!(from_bare.outcome, from_grammar.outcome);
        assert_eq!(from_bare.probability, from_grammar.probability);
        thinkthen::thinkthen_engine_free(engine);
    }
}

#[test]
fn a_band_reaches_the_unsure_code() {
    unsafe {
        let engine = engine();
        let question = CString::new(
            r#"{"decide": "Is this a maybe?", "threshold": "0.2:0.8"}"#,
        )
        .expect("static");
        let evidence = CString::new("maybe a discount").expect("static");
        let mut answer = thinkthen::thinkthen_answer { outcome: NO, probability: 0.0 };
        let code = thinkthen::thinkthen_decide(
            engine,
            question.as_ptr(),
            evidence.as_ptr(),
            evidence.as_bytes().len(),
            &mut answer,
        );
        assert_eq!(code, 0);
        assert_eq!(answer.outcome, UNSURE, "the band calls the middle answer unsure");
        thinkthen::thinkthen_engine_free(engine);
    }
}

#[test]
fn decide_many_keeps_the_input_order() {
    unsafe {
        let engine = engine();
        let question = CString::new("Does the customer ask for a refund?").expect("static");
        let texts: Vec<CString> = ["I want a refund for order 9", "just saying hi", "maybe a discount"]
            .iter()
            .map(|text| CString::new(*text).expect("no NUL"))
            .collect();
        let lengths: Vec<usize> = texts.iter().map(|text| text.as_bytes().len()).collect();
        let pointers: Vec<*const c_char> =
            texts.iter().map(|text| text.as_ptr()).collect();
        let mut out = [
            thinkthen::thinkthen_answer { outcome: NO, probability: 0.0 },
            thinkthen::thinkthen_answer { outcome: NO, probability: 0.0 },
            thinkthen::thinkthen_answer { outcome: NO, probability: 0.0 },
        ];
        let code = thinkthen::thinkthen_decide_many(
            engine,
            question.as_ptr(),
            pointers.as_ptr(),
            lengths.as_ptr(),
            3,
            out.as_mut_ptr(),
        );
        assert_eq!(code, 0);
        assert_eq!(out[0].outcome, YES);
        assert_eq!(out[1].outcome, NO);
        assert_eq!(out[2].outcome, YES);
        thinkthen::thinkthen_engine_free(engine);
    }
}

#[test]
fn a_broken_question_is_the_usage_code_with_nothing_sent() {
    unsafe {
        let engine = engine();
        let broken = CString::new(r#"{"choose": "Pick.", "options": []}"#).expect("static");
        let evidence = CString::new("anything").expect("static");
        let usage_before = usage_of(engine);
        let mut answer = thinkthen::thinkthen_answer { outcome: NO, probability: 0.0 };
        let code = thinkthen::thinkthen_decide(
            engine,
            broken.as_ptr(),
            evidence.as_ptr(),
            evidence.as_bytes().len(),
            &mut answer,
        );
        assert_eq!(code, 1, "usage is the first code");
        assert!(!message(engine).is_empty());
        assert_eq!(
            thinkthen::thinkthen_error_retryable(engine),
            0,
            "a usage failure is not retryable"
        );
        assert_eq!(
            usage_of(engine),
            usage_before,
            "nothing was sent for a usage failure"
        );
        thinkthen::thinkthen_engine_free(engine);
    }
}

#[test]
fn the_json_door_answers_every_verb_it_carries() {    unsafe {
        let engine = engine();

        let decide = take(json(
            engine,
            r#"{"decide": "Does the customer ask for a refund?", "evidence": "I want a refund for order 9"}"#,
        ));
        assert_eq!(decide["answer"], serde_json::json!(true), "decide answers");

        let band = take(json(
            engine,
            r#"{"decide": "Is this a maybe?", "threshold": "0.2:0.8", "evidence": "maybe a discount"}"#,
        ));
        assert_eq!(band["answer"], serde_json::json!(null), "unsure is null");

        let choice = take(json(
            engine,
            r#"{"choose": "Which team owns this?", "options": ["the refund desk", "the maybe desk", "anywhere else"], "threshold": 0.5, "evidence": "please route this ticket"}"#,
        ));
        assert_eq!(
            choice["answer"],
            serde_json::json!("the refund desk"),
            "choose names the winning option"
        );

        let scored = take(json(
            engine,
            r#"{"score": "How strong?", "levels": ["low", "mid", "high"], "evidence": "I want a refund for order 9"}"#,
        ));
        assert!(scored["answer"].is_number(), "score gives the position");
        assert!(scored["nearest"].is_string(), "the nearest level rides beside");

        let tags = take(json(
            engine,
            r#"{"tag": "Name the labels.", "labels": ["refund", "complaint"], "evidence": "I want a refund for order 9"}"#,
        ));
        assert!(tags["answer"].is_array(), "tag gives the labels that held");

        let filtered = take(json(
            engine,
            r#"{"decide": "Does the customer ask for a refund?", "threshold": 0.5, "records": ["I want a refund for order 9", "just saying hi", "maybe a discount"]}"#,
        ));
        assert_eq!(
            filtered["indexes"],
            serde_json::json!([0, 2]),
            "filter keeps the refund records"
        );

        let audit = take(json(
            engine,
            r#"{"decide": "Does the customer ask for a refund?", "evidence": "I want a refund for order 9", "details": true}"#,
        ));
        assert_eq!(audit["answer"], serde_json::json!(true));
        assert!(audit["model"].is_string());
        assert!(audit["digest"].is_string());
        // 0053 and 0054: one 64-figure digest a logical request, and
        // failed_questions always present, zero for one good question.
        let requests = audit["requests"].as_array().expect("requests is an array");
        assert_eq!(requests.len(), 1);
        assert_eq!(requests[0].as_str().unwrap_or_default().len(), 64);
        assert_eq!(audit["failed_questions"], serde_json::json!(0));
        // Settled 2026-09-21: nearest is null on a decide question and the
        // level's name on a score question.
        assert_eq!(audit["nearest"], serde_json::Value::Null);

        let scored_audit = take(json(
            engine,
            r#"{"score": "How urgent is this?", "levels": ["Routine.", "Soon.", "Immediate."], "evidence": "I want a refund now", "details": true}"#,
        ));
        assert!(
            scored_audit["nearest"].is_string(),
            "the nearest level rides in details on a score question"
        );

        // `details: false` is not the audit view: the request answers as
        // its plain verb, with none of the audit fields. Before this, any
        // `details` key — false included — turned the audit view on.
        let not_the_view = take(json(
            engine,
            r#"{"decide": "Does the customer ask for a refund?", "evidence": "I want a refund for order 9", "details": false}"#,
        ));
        assert_eq!(
            not_the_view["answer"],
            serde_json::json!(true),
            "the plain decide shape answers"
        );
        assert!(
            not_the_view["probability"].is_number(),
            "the plain decide shape carries its probability"
        );
        assert!(
            not_the_view.get("model").is_none()
                && not_the_view.get("digest").is_none()
                && not_the_view.get("sends").is_none()
                && not_the_view.get("requests").is_none()
                && not_the_view.get("failed_questions").is_none(),
            "details: false carries no audit field: {not_the_view}"
        );

        thinkthen::thinkthen_engine_free(engine);
    }
}

/// The shapes `e44d492` landed: the ruled record rows on the bulk forms,
/// and the failed marker on an annotate field (0054), never `null`.
#[test]
fn the_new_shapes_ride_the_json_door() {
    unsafe {
        let engine = engine();

        // The ruled record row (go-ahead item 4): the door's own two
        // fields, `input` and `value`, in input order.
        let kept = take(json(
            engine,
            r#"{"decide": "Is this a complaint?", "records": ["good morning", "I demand a refund today"]}"#,
        ));
        let indexes = kept["indexes"].as_array().expect("indexes is an array");
        let records = ["good morning", "I demand a refund today"];
        let rows: Vec<serde_json::Value> = indexes
            .iter()
            .map(|index| {
                let place = index.as_u64().unwrap_or(0) as usize;
                serde_json::json!({ "input": records[place], "value": true })
            })
            .collect();
        assert_eq!(
            rows,
            vec![serde_json::json!({ "input": "I demand a refund today", "value": true })]
        );

        // The failed marker on the one synthesized partial record: the
        // ruled JSON object, and the good answer beside it.
        assert!(
            fixture_compiled(),
            "the synthetic-partial fixture is not compiled; check.sh builds it"
        );
        let annotated = take(json(
            engine,
            r#"{"annotate": {"version": 1, "questions": {"refund": {"decide": "Is this a refund request?", "threshold": 0.5}, "topic": {"decide": "Is this a billing problem?", "threshold": 0.5}}}, "records": ["order 4471: charged twice, please refund"]}"#,
        ));
        let row = &annotated["answer"][0];
        assert_eq!(row["refund"], serde_json::json!(true));
        assert_eq!(
            row["topic"],
            serde_json::json!({"failed": {"kind": "backend", "cause": "missing_answer"}})
        );

        thinkthen::thinkthen_engine_free(engine);
    }
}

/// One JSON-door call through the door's own entry point.
unsafe fn json(engine: *const thinkthen_engine, text: &str) -> *mut c_char {
    unsafe {
        let request = CString::new(text).expect("no NUL");
        thinkthen::thinkthen_call(engine, request.as_ptr())
    }
}

/// Run one JSON-door call, parse the reply, and free the string.
unsafe fn take(pointer: *mut c_char) -> serde_json::Value {
    unsafe {
        assert!(!pointer.is_null(), "the call succeeded");
        let text = std::ffi::CStr::from_ptr(pointer).to_string_lossy().into_owned();
        thinkthen::thinkthen_free_string(pointer);
        serde_json::from_str(&text).expect("the reply is JSON")
    }
}

/// The door's own counters through the JSON door.
unsafe fn usage_of(engine: *const thinkthen_engine) -> u64 {
    unsafe {
        let request = CString::new(r#"{"usage": true}"#).expect("static");
        let reply = thinkthen::thinkthen_call(engine, request.as_ptr());
        let usage = take(reply);
        usage["requests"].as_u64().unwrap_or(0)
    }
}

/// One `thinkthen_recognize` call: the code and the JSON string, freed.
unsafe fn recognize(
    engine: *const thinkthen_engine,
    spec: &str,
    text: &str,
) -> (i32, String) {
    unsafe { recognize_with(engine, spec, text, NO_DEADLINE, std::ptr::null()) }
}

/// One `thinkthen_recognize_opts` call with the budget and token given.
unsafe fn recognize_with(
    engine: *const thinkthen_engine,
    spec: &str,
    text: &str,
    deadline_ms: i64,
    cancel: *const thinkthen::thinkthen_cancel_token,
) -> (i32, String) {
    unsafe {
    let spec = CString::new(spec).expect("no NUL");
    let mut out: *mut c_char = std::ptr::null_mut();
    let mut out_len: usize = 0;
    let code = thinkthen::thinkthen_recognize_opts(
        engine,
        spec.as_ptr(),
        text.as_ptr() as *const c_char,
        text.len(),
        deadline_ms,
        cancel,
        &mut out,
        &mut out_len,
    );
    if code != 0 {
        return (code, message(engine));
    }
    let json = std::ffi::CStr::from_ptr(out).to_string_lossy().into_owned();
    assert_eq!(
        json.len(),
        out_len,
        "the length is the returned string's own"
    );
    thinkthen::thinkthen_free_string(out);
    (code, json)
    }
}


/// One `thinkthen_relate` call: the code and the JSON string, freed.
unsafe fn relate(
    engine: *const thinkthen_engine,
    spec: &str,
    records: &[&str],
) -> (i32, String) {
    unsafe { relate_with(engine, spec, records, NO_DEADLINE, std::ptr::null()) }
}

/// One `thinkthen_relate_opts` call with the budget and token given.
unsafe fn relate_with(
    engine: *const thinkthen_engine,
    spec: &str,
    records: &[&str],
    deadline_ms: i64,
    cancel: *const thinkthen::thinkthen_cancel_token,
) -> (i32, String) {
    unsafe {
    let spec = CString::new(spec).expect("no NUL");
    let texts: Vec<CString> = records
        .iter()
        .map(|record| CString::new(*record).expect("no NUL"))
        .collect();
    let lengths: Vec<usize> = texts.iter().map(|text| text.as_bytes().len()).collect();
    let pointers: Vec<*const c_char> = texts.iter().map(|text| text.as_ptr()).collect();
    let mut out: *mut c_char = std::ptr::null_mut();
    let mut out_len: usize = 0;
    let code = thinkthen::thinkthen_relate_opts(
        engine,
        spec.as_ptr(),
        pointers.as_ptr(),
        lengths.as_ptr(),
        records.len(),
        deadline_ms,
        cancel,
        &mut out,
        &mut out_len,
    );
    if code != 0 {
        return (code, message(engine));
    }
    let json = std::ffi::CStr::from_ptr(out).to_string_lossy().into_owned();
    assert_eq!(json.len(), out_len);
    thinkthen::thinkthen_free_string(out);
    (code, json)
    }
}


/// The C host's one conversion: walk the text, count code points, keep the
/// byte positions of `start` and one past `end`, the way the example does.
fn byte_range(text: &str, start: usize, end: usize) -> (usize, usize) {
    let mut from = 0usize;
    let mut to = text.len();
    for (point, (byte, _)) in text.char_indices().enumerate() {
        if point == start {
            from = byte;
        }
        if point == end {
            to = byte;
            return (from, to);
        }
    }
    (from, to)
}

#[test]
fn recognize_answers_from_the_recording() {
    unsafe {
        let engine = engine();
        let spec = r#"{"kinds": ["person", "organization", "place"],
            "relations": [{"name": "works_for", "source": "person", "target": "organization"}]}"#;
        let (code, json) = recognize(
            engine,
            spec,
            "Maria Chen joined Northwind Freight in Chicago last spring.",
        );
        assert_eq!(code, 0, "{json}");
        let answer: serde_json::Value = serde_json::from_str(&json).expect("JSON");
        let names = answer["entities"].as_array().expect("names");
        assert_eq!(names.len(), 3, "{json}");
        assert_eq!(names[0]["text"], "Maria Chen");
        assert_eq!(names[0]["kind"], "person");
        assert_eq!(names[0]["start"], 0);
        assert_eq!(names[0]["end"], 10);
        assert_eq!(names[0]["strength"], 0.98);
        assert_eq!(names[2]["text"], "Chicago");
        assert_eq!(names[2]["strength"], 0.6693);
        let relations = answer["relations"].as_array().expect("relations");
        assert_eq!(relations.len(), 1, "{json}");
        assert_eq!(relations[0]["name"], "works_for");
        assert_eq!(relations[0]["source"], 1);
        assert_eq!(relations[0]["target"], 2);
        assert_eq!(relations[0]["probability"], 1.0);
        assert!(!json.contains("confidence"), "the vendor's word stays out: {json}");
        assert!(!json.contains("\"from\""), "the ruled ends are source and target: {json}");
        thinkthen::thinkthen_engine_free(engine);
    }
}

/// The deck's rules on the Maria Chen sentence, pinned: the C01 recording
/// covers `works_for` and `based_in`, and the deck also asks `located_in`,
/// so the call refuses and names what the recording covers.
#[test]
fn recognize_refuses_the_decks_unrecorded_rule() {
    unsafe {
        let engine = engine();
        let spec = r#"{"kinds": ["person", "organization", "place"],
            "relations": [{"name": "works_for", "source": "person", "target": "organization"},
                          {"name": "located_in", "source": "*", "target": "place"}]}"#;
        let usage_before = usage_of(engine);
        let (code, text) = recognize(
            engine,
            spec,
            "Maria Chen joined Northwind Freight in Chicago last spring.",
        );
        assert_eq!(code, 1, "the usage kind: {text}");
        assert!(text.contains("located_in"), "{text}");
        assert!(text.contains("works_for, based_in"), "{text}");
        assert_eq!(
            usage_of(engine),
            usage_before,
            "a refusal sends nothing"
        );
        thinkthen::thinkthen_engine_free(engine);
    }
}

#[test]
fn a_text_with_no_recording_is_refused() {
    unsafe {
        let engine = engine();
        let (code, text) = recognize(engine, r#"{"kinds": ["person"]}"#, "unrecorded.");
        assert_eq!(code, 1, "{text}");
        assert!(text.contains("no recorded answer for the text"), "{text}");
        thinkthen::thinkthen_engine_free(engine);
    }
}

/// The C offset proof: the contract counts code points, C slices bytes,
/// and the emoji case makes the two units disagree by the emoji's width.
#[test]
fn recognize_offsets_slice_in_bytes_on_the_emoji_text() {
    unsafe {
        let engine = engine();
        let text = "Le café 😀 Maria Chen arrived.";
        let (code, json) = recognize(engine, r#"{"kinds": ["person"]}"#, text);
        assert_eq!(code, 0, "{json}");
        let answer: serde_json::Value = serde_json::from_str(&json).expect("JSON");
        let name = &answer["entities"][0];
        assert_eq!(name["text"], "Maria Chen");
        assert_eq!(name["strength"], 0.94);
        let start = name["start"].as_u64().expect("a number") as usize;
        let end = name["end"].as_u64().expect("a number") as usize;
        assert_eq!((start, end), (10, 20), "code points: {json}");
        let (from, to) = byte_range(text, start, end);
        assert_eq!((from, to), (14, 24), "bytes, the emoji is four wide");
        assert_eq!(&text[from..to], "Maria Chen", "the slice is the name");
        thinkthen::thinkthen_engine_free(engine);
    }
}

#[test]
fn relate_answers_from_the_recording() {
    unsafe {
        let engine = engine();
        let alerts = [
            "Alert 1: Checkout returns 500 at the payment step.",
            "Alert 2: Card charges are failing for every customer.",
            "Alert 3: The nightly export ran two hours late.",
            "Alert 4: The payments database ran out of disk space.",
        ];
        let spec = r#"{"relations": [{"name": "caused_by", "source": "*", "target": "*"}],
            "either": ["same_as"], "threshold": 0.7}"#;
        let (code, json) = relate(engine, spec, &alerts);
        assert_eq!(code, 0, "{json}");
        let answer: serde_json::Value = serde_json::from_str(&json).expect("JSON");
        let edges = answer["edges"].as_array().expect("edges");
        assert_eq!(edges.len(), 2, "the 0.7 bar keeps two: {json}");
        assert_eq!(edges[0]["name"], "caused_by");
        assert_eq!(edges[0]["source"], 1);
        assert_eq!(edges[0]["target"], 4);
        assert!(
            (edges[0]["probability"].as_f64().expect("a number") - 0.71).abs() < 1e-9,
            "{json}"
        );
        assert_eq!(edges[1]["source"], 2);
        assert_eq!(edges[1]["target"], 4);
        assert!(!json.contains("\"from\""), "source and target: {json}");
        thinkthen::thinkthen_engine_free(engine);
    }
}

#[test]
fn relate_refuses_more_than_255_records() {
    unsafe {
        let engine = engine();
        let records: Vec<String> = (0..256).map(|index| format!("record {index}")).collect();
        let borrowed: Vec<&str> = records.iter().map(String::as_str).collect();
        let usage_before = usage_of(engine);
        let (code, text) = relate(
            engine,
            r#"{"relations": [{"name": "caused_by", "source": "*", "target": "*"}]}"#,
            &borrowed,
        );
        assert_eq!(code, 1, "the usage kind: {text}");
        assert!(text.contains("255"), "{text}");
        assert_eq!(usage_of(engine), usage_before, "a refusal sends nothing");
        thinkthen::thinkthen_engine_free(engine);
    }
}

/// The error surface: the JSON door's failure code is retrievable, the
/// last failure stays until the next failure, and a success does not
/// clear it (NOTES finding 2, closed by `thinkthen_error_code`).
#[test]
fn the_json_doors_failure_code_is_retrievable() {
    unsafe {
        let engine = engine();
        assert_eq!(
            thinkthen::thinkthen_error_code(engine),
            0,
            "nothing failed yet"
        );
        let request = CString::new(r#"{"nope": true}"#).expect("static");
        let reply = thinkthen::thinkthen_call(engine, request.as_ptr());
        assert!(reply.is_null(), "the request carries no verb");
        assert_eq!(
            thinkthen::thinkthen_error_code(engine),
            1,
            "the usage refusal is readable as a code: {}",
            message(engine)
        );
        assert!(message(engine).contains("no verb"), "{}", message(engine));

        let counters = CString::new(r#"{"usage": true}"#).expect("static");
        let reply = thinkthen::thinkthen_call(engine, counters.as_ptr());
        assert!(!reply.is_null(), "the counters answer");
        thinkthen::thinkthen_free_string(reply);
        assert_eq!(
            thinkthen::thinkthen_error_code(engine),
            1,
            "success does not clear the last failure"
        );
        thinkthen::thinkthen_engine_free(engine);
    }
}

/// Review 5: the door read `rank` and `usage` by key presence, so
/// `"rank": false` ranked and `"usage": false` answered the counters. The
/// flags are read by value, and a value that is not a boolean is refused.
#[test]
fn the_doors_flags_are_read_by_value() {
    unsafe {
        let engine = engine();
        let filter = r#""decide": "Does the customer ask for a refund?", "threshold": 0.5, "records": ["I want a refund for order 9", "just saying hi"]"#;

        let reply = json(engine, &format!("{{{filter}, \"rank\": false}}"));
        assert!(!reply.is_null(), "rank: false answers: {}", message(engine));
        let not_ranked = take(reply);
        assert_eq!(
            not_ranked["indexes"],
            serde_json::json!([0]),
            "rank: false filters: {not_ranked}"
        );
        let rank = r#""decide": "Does the customer ask for a refund?", "records": ["I want a refund for order 9", "just saying hi"]"#;
        let reply = json(engine, &format!("{{{rank}, \"rank\": true}}"));
        assert!(!reply.is_null(), "rank: true answers: {}", message(engine));
        let ranked = take(reply);
        assert!(ranked["answer"].is_array(), "rank: true ranks: {ranked}");

        let no_counters = json(engine, r#"{"usage": false}"#);
        assert!(no_counters.is_null(), "usage: false is not the counters");
        assert!(message(engine).contains("no verb"), "{}", message(engine));

        for (key, request) in [
            ("details", r#"{"decide": "Does the customer ask for a refund?", "evidence": "I want a refund for order 9", "details": "true"}"#.to_string()),
            ("rank", format!("{{{filter}, \"rank\": 1}}")),
            ("rank", r#"{"decide": "Does the customer ask for a refund?", "evidence": "I want a refund for order 9", "rank": 1}"#.to_string()),
            ("usage", r#"{"usage": "yes"}"#.to_string()),
        ] {
            let usage_before = usage_of(engine);
            let reply = json(engine, &request);
            assert!(reply.is_null(), "{key} with a non-boolean value is refused");
            assert_eq!(thinkthen::thinkthen_error_code(engine), 1, "{}", message(engine));
            assert!(
                message(engine).contains(&format!("the {key} key takes true or false")),
                "{}",
                message(engine)
            );
            assert_eq!(usage_of(engine), usage_before, "a refusal sends nothing");
        }
        thinkthen::thinkthen_engine_free(engine);
    }
}

/// The header's promise: every plain spelling is exactly its `_opts` twin
/// called with THINKTHEN_NO_DEADLINE and a null token, on the answer path
/// and on the error path alike.
#[test]
fn the_plain_call_is_its_opts_twin() {
    unsafe {
        let engine = engine();
        let question = CString::new("Does the customer ask for a refund?").expect("static");
        let evidence = CString::new("I want a refund for order 9").expect("static");
        let broken = CString::new(r#"{"choose": "Pick.", "options": []}"#).expect("static");

        // decide: the answer path.
        let mut plain = UNWRITTEN;
        let mut opts = UNWRITTEN;
        let plain_code = thinkthen::thinkthen_decide(
            engine,
            question.as_ptr(),
            evidence.as_ptr(),
            evidence.as_bytes().len(),
            &mut plain,
        );
        let opts_code = thinkthen::thinkthen_decide_opts(
            engine,
            question.as_ptr(),
            evidence.as_ptr(),
            evidence.as_bytes().len(),
            NO_DEADLINE,
            std::ptr::null(),
            &mut opts,
        );
        assert_eq!(plain_code, 0, "{}", message(engine));
        assert_eq!(opts_code, plain_code);
        assert_eq!(opts.outcome, plain.outcome);
        assert_eq!(opts.probability, plain.probability);

        // decide: the error path.
        let mut plain = UNWRITTEN;
        let mut opts = UNWRITTEN;
        let plain_code = thinkthen::thinkthen_decide(
            engine,
            broken.as_ptr(),
            evidence.as_ptr(),
            evidence.as_bytes().len(),
            &mut plain,
        );
        let plain_message = message(engine);
        let opts_code = thinkthen::thinkthen_decide_opts(
            engine,
            broken.as_ptr(),
            evidence.as_ptr(),
            evidence.as_bytes().len(),
            NO_DEADLINE,
            std::ptr::null(),
            &mut opts,
        );
        assert_eq!(plain_code, 1);
        assert_eq!(opts_code, plain_code);
        assert_eq!(message(engine), plain_message, "the same refusal names the same failure");
        assert_eq!(plain.outcome, 7, "a refusal writes nothing");
        assert_eq!(opts.outcome, 7);

        // decide_many: both paths.
        let texts: Vec<CString> = ["I want a refund for order 9", "just saying hi"]
            .iter()
            .map(|text| CString::new(*text).expect("no NUL"))
            .collect();
        let lengths: Vec<usize> = texts.iter().map(|text| text.as_bytes().len()).collect();
        let pointers: Vec<*const c_char> = texts.iter().map(|text| text.as_ptr()).collect();
        let mut plain = [UNWRITTEN; 2];
        let mut opts = [UNWRITTEN; 2];
        let plain_code = thinkthen::thinkthen_decide_many(
            engine,
            question.as_ptr(),
            pointers.as_ptr(),
            lengths.as_ptr(),
            2,
            plain.as_mut_ptr(),
        );
        let opts_code = thinkthen::thinkthen_decide_many_opts(
            engine,
            question.as_ptr(),
            pointers.as_ptr(),
            lengths.as_ptr(),
            2,
            NO_DEADLINE,
            std::ptr::null(),
            opts.as_mut_ptr(),
        );
        assert_eq!(plain_code, 0, "{}", message(engine));
        assert_eq!(opts_code, plain_code);
        assert_eq!(opts[0].outcome, plain[0].outcome);
        assert_eq!(opts[1].outcome, plain[1].outcome);

        let mut plain = [UNWRITTEN; 2];
        let mut opts = [UNWRITTEN; 2];
        let plain_code = thinkthen::thinkthen_decide_many(
            engine,
            broken.as_ptr(),
            pointers.as_ptr(),
            lengths.as_ptr(),
            2,
            plain.as_mut_ptr(),
        );
        let plain_message = message(engine);
        let opts_code = thinkthen::thinkthen_decide_many_opts(
            engine,
            broken.as_ptr(),
            pointers.as_ptr(),
            lengths.as_ptr(),
            2,
            NO_DEADLINE,
            std::ptr::null(),
            opts.as_mut_ptr(),
        );
        assert_eq!(plain_code, 1);
        assert_eq!(opts_code, plain_code);
        assert_eq!(message(engine), plain_message);
        assert_eq!(plain[0].outcome, 7);
        assert_eq!(opts[0].outcome, 7);

        // The JSON door: both paths.
        let request = CString::new(
            r#"{"decide": "Does the customer ask for a refund?", "evidence": "I want a refund for order 9"}"#,
        )
        .expect("static");
        let plain_reply = thinkthen::thinkthen_call(engine, request.as_ptr());
        let opts_reply =
            thinkthen::thinkthen_call_opts(engine, request.as_ptr(), NO_DEADLINE, std::ptr::null());
        assert!(!plain_reply.is_null());
        assert_eq!(
            std::ffi::CStr::from_ptr(plain_reply).to_bytes(),
            std::ffi::CStr::from_ptr(opts_reply).to_bytes()
        );
        thinkthen::thinkthen_free_string(plain_reply);
        thinkthen::thinkthen_free_string(opts_reply);

        let broken_request = CString::new(r#"{"nope": true}"#).expect("static");
        let plain_reply = thinkthen::thinkthen_call(engine, broken_request.as_ptr());
        let plain_message = message(engine);
        let opts_reply =
            thinkthen::thinkthen_call_opts(engine, broken_request.as_ptr(), NO_DEADLINE, std::ptr::null());
        assert!(plain_reply.is_null());
        assert!(opts_reply.is_null());
        assert_eq!(message(engine), plain_message);

        // recognize: both paths.
        let spec =
            r#"{"kinds": ["person", "organization", "place"], "relations": [{"name": "works_for", "source": "person", "target": "organization"}]}"#;
        let sentence = "Maria Chen joined Northwind Freight in Chicago last spring.";
        let (plain_code, plain_json) = recognize(engine, spec, sentence);
        let (opts_code, opts_json) =
            recognize_with(engine, spec, sentence, NO_DEADLINE, std::ptr::null());
        assert_eq!(plain_code, 0, "{plain_json}");
        assert_eq!(opts_code, plain_code);
        assert_eq!(opts_json, plain_json);

        let (plain_code, plain_message) = recognize(engine, r#"{"kinds": ["person"]}"#, "unrecorded.");
        let (opts_code, opts_message) = recognize_with(
            engine,
            r#"{"kinds": ["person"]}"#,
            "unrecorded.",
            NO_DEADLINE,
            std::ptr::null(),
        );
        assert_eq!(plain_code, 1);
        assert_eq!(opts_code, plain_code);
        assert_eq!(opts_message, plain_message);

        // relate: both paths.
        let alerts = [
            "Alert 1: Checkout returns 500 at the payment step.",
            "Alert 2: Card charges are failing for every customer.",
            "Alert 3: The nightly export ran two hours late.",
            "Alert 4: The payments database ran out of disk space.",
        ];
        let rule = r#"{"relations": [{"name": "caused_by", "source": "*", "target": "*"}]}"#;
        let (plain_code, plain_json) = relate(engine, rule, &alerts);
        let (opts_code, opts_json) =
            relate_with(engine, rule, &alerts, NO_DEADLINE, std::ptr::null());
        assert_eq!(plain_code, 0, "{plain_json}");
        assert_eq!(opts_code, plain_code);
        assert_eq!(opts_json, plain_json);

        let many: Vec<String> = (0..256).map(|index| format!("record {index}")).collect();
        let borrowed: Vec<&str> = many.iter().map(String::as_str).collect();
        let (plain_code, plain_message) = relate(engine, rule, &borrowed);
        let (opts_code, opts_message) =
            relate_with(engine, rule, &borrowed, NO_DEADLINE, std::ptr::null());
        assert_eq!(plain_code, 1);
        assert_eq!(opts_code, plain_code);
        assert_eq!(opts_message, plain_message);

        thinkthen::thinkthen_engine_free(engine);
    }
}

/// Whether this build carries the stand-in's synthesized partial failure.
fn fixture_compiled() -> bool {
    cfg!(feature = "synthetic-partial")
}
