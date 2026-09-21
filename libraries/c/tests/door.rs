//! The door's null suite: the typed doors, the error codes a host sees,
//! and the JSON door's replies, all against the in-process backend.
//!
//! Run through `./check.sh`, which sets `ENGINE_NULL=1`.

use std::ffi::{c_char, CString};

use thinkthen::thinkthen_engine;

const YES: i32 = 1;
const NO: i32 = 0;
const UNSURE: i32 = 2;

unsafe fn engine() -> *mut thinkthen_engine {
    thinkthen::thinkthen_engine_new()
}

unsafe fn message(engine: *const thinkthen_engine) -> String {
    std::ffi::CStr::from_ptr(thinkthen::thinkthen_error_message(engine))
        .to_string_lossy()
        .into_owned()
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
        let annotated = take(json(
            engine,
            r#"{"annotate": {"questions": {"refund": {"decide": "Is this a refund request?", "threshold": 0.5}, "topic": {"decide": "Is this a billing problem?", "threshold": 0.5}}}, "records": ["order 4471: charged twice, please refund"]}"#,
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
    let request = CString::new(text).expect("no NUL");
    thinkthen::thinkthen_call(engine, request.as_ptr())
}

/// Run one JSON-door call, parse the reply, and free the string.
unsafe fn take(pointer: *mut c_char) -> serde_json::Value {
    assert!(!pointer.is_null(), "the call succeeded");
    let text = std::ffi::CStr::from_ptr(pointer).to_string_lossy().into_owned();
    thinkthen::thinkthen_free_string(pointer);
    serde_json::from_str(&text).expect("the reply is JSON")
}

/// The door's own counters through the JSON door.
unsafe fn usage_of(engine: *const thinkthen_engine) -> u64 {
    let request = CString::new(r#"{"usage": true}"#).expect("static");
    let reply = thinkthen::thinkthen_call(engine, request.as_ptr());
    let usage = take(reply);
    usage["requests"].as_u64().unwrap_or(0)
}

/// One `thinkthen_recognize` call: the code and the JSON string, freed.
unsafe fn recognize(
    engine: *const thinkthen_engine,
    spec: &str,
    text: &str,
) -> (i32, String) {
    let spec = CString::new(spec).expect("no NUL");
    let mut out: *mut c_char = std::ptr::null_mut();
    let mut out_len: usize = 0;
    let code = thinkthen::thinkthen_recognize(
        engine,
        spec.as_ptr(),
        text.as_ptr() as *const c_char,
        text.as_bytes().len(),
        &mut out,
        &mut out_len,
    );
    if code != 0 {
        return (code, message(engine));
    }
    let json = std::ffi::CStr::from_ptr(out).to_string_lossy().into_owned();
    assert_eq!(
        json.as_bytes().len(),
        out_len,
        "the length is the returned string's own"
    );
    thinkthen::thinkthen_free_string(out);
    (code, json)
}

/// One `thinkthen_relate` call: the code and the JSON string, freed.
unsafe fn relate(
    engine: *const thinkthen_engine,
    spec: &str,
    records: &[&str],
) -> (i32, String) {
    let spec = CString::new(spec).expect("no NUL");
    let texts: Vec<CString> = records
        .iter()
        .map(|record| CString::new(*record).expect("no NUL"))
        .collect();
    let lengths: Vec<usize> = texts.iter().map(|text| text.as_bytes().len()).collect();
    let pointers: Vec<*const c_char> = texts.iter().map(|text| text.as_ptr()).collect();
    let mut out: *mut c_char = std::ptr::null_mut();
    let mut out_len: usize = 0;
    let code = thinkthen::thinkthen_relate(
        engine,
        spec.as_ptr(),
        pointers.as_ptr(),
        lengths.as_ptr(),
        records.len(),
        &mut out,
        &mut out_len,
    );
    if code != 0 {
        return (code, message(engine));
    }
    let json = std::ffi::CStr::from_ptr(out).to_string_lossy().into_owned();
    assert_eq!(json.as_bytes().len(), out_len);
    thinkthen::thinkthen_free_string(out);
    (code, json)
}

/// The C host's one conversion: walk the text, count code points, keep the
/// byte positions of `start` and one past `end`, the way the example does.
fn byte_range(text: &str, start: usize, end: usize) -> (usize, usize) {
    let mut points = 0usize;
    let mut from = 0usize;
    let mut to = text.len();
    for (byte, _) in text.char_indices() {
        if points == start {
            from = byte;
        }
        if points == end {
            to = byte;
            return (from, to);
        }
        points += 1;
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
            "either": ["same_as"], "threshold": 0.9}"#;
        let (code, json) = relate(engine, spec, &alerts);
        assert_eq!(code, 0, "{json}");
        let answer: serde_json::Value = serde_json::from_str(&json).expect("JSON");
        let edges = answer["edges"].as_array().expect("edges");
        assert_eq!(edges.len(), 2, "the 0.9 bar keeps two: {json}");
        assert_eq!(edges[0]["name"], "caused_by");
        assert_eq!(edges[0]["source"], 1);
        assert_eq!(edges[0]["target"], 4);
        assert!(
            (edges[0]["probability"].as_f64().expect("a number") - 0.94).abs() < 1e-9,
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
