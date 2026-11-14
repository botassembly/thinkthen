#![cfg(test)]
#![allow(
    unsafe_code,
    reason = "exercise public native row and buffer ownership"
)]
//! Canonical native serialization preserves character coordinates independently of host views.
use serde_json::{Value, json};
use std::{
    mem::{size_of, zeroed},
    ptr,
};
use thinkthen_ffi::*;

unsafe fn row(server: &loopback::Loopback, request: &str, unit: u32) -> HostRow {
    // SAFETY: the helper retains the same counted inputs and owned outputs.
    unsafe { row_input(server, request, unit, None) }
}

unsafe fn row_input(
    server: &loopback::Loopback,
    request: &str,
    unit: u32,
    raw: Option<&str>,
) -> HostRow {
    // SAFETY: delegate the same counted inputs and output lifetime.
    unsafe { row_input_at(server, request, unit, raw, 1) }
}

unsafe fn row_input_at(
    server: &loopback::Loopback,
    request: &str,
    unit: u32,
    raw: Option<&str>,
    position: u64,
) -> HostRow {
    let settings =
        json!({"backend":"systemone","url":server.base(),"api_key":loopback::KEY}).to_string();
    // SAFETY: every counted input and initialized output lives until its matching cleanup.
    unsafe {
        let mut client = ptr::null_mut();
        assert_eq!(
            thinkthen_client_open_json(
                settings.as_ptr(),
                settings.len(),
                &mut client,
                ptr::null_mut()
            ),
            0
        );
        let mut call = ptr::null_mut();
        let opened = if let Some(raw) = raw {
            let input = RecordInput {
                struct_size: size_of::<RecordInput>() as u32,
                kind: thinkthen::records::RecordKind::Json.abi(),
                data: raw.as_ptr(),
                len: raw.len(),
            };
            thinkthen_call_open_records_json(
                client,
                request.as_ptr(),
                request.len(),
                &input,
                1,
                size_of::<RecordInput>(),
                unit,
                &mut call,
                ptr::null_mut(),
            )
        } else {
            thinkthen_call_open_json(
                client,
                request.as_ptr(),
                request.len(),
                unit,
                &mut call,
                ptr::null_mut(),
            )
        };
        assert_eq!(opened, 0);
        thinkthen_call_end_input(call);
        let mut result: HostRow = zeroed();
        result.struct_size = size_of::<HostRow>() as u32;
        for _ in 0..position {
            assert_eq!(thinkthen_call_next(call, &mut result, ptr::null_mut()), 0);
        }
        let mut end: HostRow = zeroed();
        end.struct_size = size_of::<HostRow>() as u32;
        assert_eq!(
            thinkthen_call_next(call, &mut end, ptr::null_mut()),
            THINKTHEN_END
        );
        thinkthen_call_free(call);
        thinkthen_client_free(client);
        result
    }
}

unsafe fn decode(mut buffer: Buffer) -> Value {
    // SAFETY: the engine lends the bytes until the independently owned buffer is freed.
    unsafe {
        let value =
            serde_json::from_slice(std::slice::from_raw_parts(buffer.data, buffer.len)).unwrap();
        thinkthen_buffer_free(&mut buffer);
        value
    }
}

unsafe fn saved(row: &HostRow) -> Buffer {
    let mut output = Buffer::default();
    // SAFETY: the live borrowed row and exclusive buffer outlive serialization.
    unsafe {
        assert_eq!(
            thinkthen_row_to_json(row, &mut output, ptr::null_mut()),
            THINKTHEN_OK
        );
    }
    output
}

unsafe fn host_span(row: &HostRow) -> (u64, u64) {
    // SAFETY: the copied value owns its native span until freed below.
    unsafe {
        let mut value = ptr::null_mut();
        assert_eq!(thinkthen_row_values(row, &mut value, ptr::null_mut()), 0);
        let mut span = SpanView::default();
        assert_eq!(thinkthen_value_span(value, &mut span, ptr::null_mut()), 0);
        assert_eq!(span.present, 1);
        thinkthen_value_free(value);
        (span.start, span.end)
    }
}

#[test]
fn serialization_keeps_native_spans_and_outlives_row_call_and_client() {
    let server = loopback::Loopback::case(&json!({"replies":[
        {"distribution":{"u1":0.1,"u2":0.9,"none":0}}
    ]}))
    .unwrap();
    let request = json!({"question":{"function":"find","question":"Which line?","field":"/text"},
        "records":[{"id":"row-01","text":"é🦀\r\n E\u{301}lise"}]})
    .to_string();
    for (unit, host) in [(0, (8, 16)), (1, (5, 12)), (2, (4, 11)), (3, (5, 11))] {
        // SAFETY: returned rows and buffers remain owned until explicit cleanup.
        unsafe {
            let mut result = row(&server, &request, unit);
            assert_eq!(host_span(&result), host);
            let encoded = saved(&result);
            assert_eq!(
                host_span(&result),
                host,
                "serialization must not mutate host getters"
            );
            thinkthen_row_free(&mut result);
            let value = decode(encoded);
            assert_eq!(value["function"], "find");
            assert_eq!(value["status"], "answered");
            assert_eq!(value["position"], 1);
            assert_eq!(value["id"], "row-01");
            assert_eq!(value["probability"], 0.9);
            assert_eq!(
                value["answer"],
                json!({"text":" E\u{301}lise","start":4,"end":11})
            );
            assert!(value.get("item").is_none());
            assert_eq!(value["source"]["backend"], "systemone");
            assert_eq!(
                value["source"]["question_identity"].as_str().unwrap().len(),
                64
            );
        }
    }
    assert_eq!(server.count(), 4);
}

#[test]
fn unsure_failure_and_answered_null_are_serialized_without_an_answer_read() {
    for (name, status) in [
        ("decide-unsure", "unsure"),
        ("tag-backend-reply", "failed"),
        ("find-no-answer", "answered"),
    ] {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../conformance/typed")
            .join(format!("{name}.json"));
        let case: Value = serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
        let server = loopback::Loopback::case(&case).unwrap();
        // SAFETY: the row and result buffer are independent engine owners.
        unsafe {
            let mut result = row(&server, &case["request"].to_string(), 0);
            let encoded = saved(&result);
            thinkthen_row_free(&mut result);
            let value = decode(encoded);
            assert_eq!(value["status"], status);
            assert_eq!(value["position"], 1);
            assert!(value.get("item").is_none());
            assert!(value["source"]["model"].is_string());
            if status == "answered" {
                assert_eq!(value.get("answer"), Some(&Value::Null));
            } else {
                assert!(value.get("answer").is_none());
            }
            if status == "failed" {
                assert_eq!(value["error"]["code"], "backend.reply");
                assert!(!value["error"]["message"].as_str().unwrap().is_empty());
                assert!(value.get("probability").is_none());
            } else {
                assert!(value["probability"].is_number());
                assert!(value.get("error").is_none());
            }
        }
    }
}

#[test]
fn raw_non_string_json_and_form_field_overrides_use_engine_character_coordinates() {
    let case = json!({"replies":[
        {"contains":"Which JSON line?","distribution":{"u1":1,"none":0}},
        {"criterion":"BEGIN","contains":"[[E\u{301}lise]]","distribution":{"BEGIN":0,"END":0,"INSIDE":0,"OUT":0,"SINGLE":1}},
        {"criterion":"BEGIN","distribution":{"BEGIN":0,"END":0,"INSIDE":0,"OUT":1,"SINGLE":0}},
        {"criterion":"person","distribution":{"person":1,"none of these":0}},
        {"distribution":{"u1":0.1,"u2":0.9,"none":0}}
    ]});
    let server = loopback::Loopback::case(&case).unwrap();
    let raw = "{ \"id\" : \"spaced\", \"body\" : \"é🦀\\r\\n Élise\", \"other\" : \"abcd Élise\", \"json\" : { \"unicode\" : \"é🦀\" , \"count\" : 3 } }";
    let question = json!({"function":"annotate","field":"/body","questions":{
        "names":{"function":"recognize","kinds":["person"]},
        "other_names":{"function":"recognize","kinds":["person"],"field":"/other"},
        "line":{"function":"find","question":"Which line?"},
        "missing":{"function":"find","question":"Which line?","field":"/absent"},
        "raw":{"function":"find","question":"Which JSON line?","field":"/json"}
    }});
    let request = json!({"question":question}).to_string();
    let mut previous = None;
    for unit in [0, 1, 2] {
        // SAFETY: metadata retained by the row remains live after the call/client are freed.
        unsafe {
            let mut result = row_input(&server, &request, unit, Some(raw));
            assert_eq!(
                result.error_code, 0,
                "a failed child is not a parent failure"
            );
            let encoded = saved(&result);
            thinkthen_row_free(&mut result);
            let value = decode(encoded);
            assert_eq!(value["id"], "spaced");
            assert!(value.get("error").is_none());
            assert_eq!(value["answer"]["missing"]["status"], "failed");
            assert_eq!(
                value["answer"]["missing"]["error"]["code"],
                "record.missing"
            );
            assert_eq!(value["answer"]["names"]["answer"][0]["start"], 5);
            assert_eq!(value["answer"]["names"]["answer"][0]["end"], 11);
            assert_eq!(value["answer"]["other_names"]["answer"][0]["start"], 5);
            assert_eq!(value["answer"]["line"]["answer"]["start"], 4);
            assert_eq!(value["answer"]["names"]["detail"]["pieces"][1]["start"], 1);
            let raw_answer = &value["answer"]["raw"]["answer"];
            assert_eq!(
                raw_answer["text"],
                "{ \"unicode\" : \"é🦀\" , \"count\" : 3 }"
            );
            assert_eq!(
                raw_answer["end"].as_u64().unwrap(),
                raw_answer["text"].as_str().unwrap().chars().count() as u64
            );
            if let Some(previous) = &previous {
                assert_eq!(&value, previous);
            }
            previous = Some(value);
        }
    }
}

#[test]
fn relation_answer_and_details_serialize_the_retained_character_endpoints() {
    let mut case: Value = serde_json::from_str(include_str!(
        "../../../conformance/typed/relate-either.json"
    ))
    .unwrap();
    for prefix in ["[[é]]", "[[🦀]]", "[[é🦀]]"] {
        case["replies"].as_array_mut().unwrap().insert(
            0,
            json!({"criterion":"BEGIN","contains":prefix,
            "distribution":{"BEGIN":0,"END":0,"INSIDE":0,"OUT":1,"SINGLE":0}}),
        );
    }
    case["request"]["records"] = json!(["é🦀 Ada Acme Bea"]);
    let server = loopback::Loopback::case(&case).unwrap();
    let mut previous = None;
    for unit in [0, 1, 2] {
        // SAFETY: each serialization borrows a live row, then its buffer owns the saved bytes.
        unsafe {
            let mut result = row(&server, &case["request"].to_string(), unit);
            let encoded = saved(&result);
            thinkthen_row_free(&mut result);
            let value = decode(encoded);
            assert_eq!(value["answer"][0]["source"]["start"], 3);
            assert_eq!(value["detail"]["pairs"][0]["source"]["start"], 3);
            if let Some(previous) = &previous {
                assert_eq!(&value, previous);
            }
            previous = Some(value);
        }
    }
}

unsafe fn detached_metadata(child: &HostRow, parent: &Value, selected: Option<&str>) {
    // SAFETY: the retained row and value snapshot own all borrowed metadata views.
    unsafe {
        assert_eq!(child.position, 2);
        let mut id = IdView::default();
        assert_eq!(thinkthen_row_id(child, &mut id, ptr::null_mut()), 0);
        assert_eq!(id.kind, thinkthen::records::IdKind::Unsigned.abi());
        assert_eq!(id.unsigned_value, u64::MAX);
        let mut snapshot = ptr::null_mut();
        assert_eq!(
            thinkthen_row_values(child, &mut snapshot, ptr::null_mut()),
            0
        );
        let mut source = SourceView::default();
        assert_eq!(
            thinkthen_value_source(snapshot, &mut source, ptr::null_mut()),
            0
        );
        let identity = std::str::from_utf8(std::slice::from_raw_parts(
            source.question_identity.data,
            source.question_identity.len,
        ))
        .unwrap();
        assert_eq!(
            identity,
            parent["source"]["question_identity"].as_str().unwrap()
        );
        thinkthen_value_free(snapshot);
        if let Some(expected) = selected {
            let mut text = TextView::default();
            assert_eq!(
                thinkthen_row_selected_text(child, ptr::null(), 0, &mut text, ptr::null_mut()),
                0
            );
            assert_eq!(
                std::slice::from_raw_parts(text.data, text.len),
                expected.as_bytes()
            );
        }
    }
}

#[test]
fn detached_entries_reload_as_canonical_rows_after_parent_and_client_cleanup() {
    let server = loopback::Loopback::case(&json!({"replies":[
        {"contains":"Which line?","distribution":{"u1":0.1,"u2":0.9,"none":0}},
        {"contains":"Unsure?","probability":0.5},
        {"contains":"Failed?","answer":null}
    ]}))
    .unwrap();
    let text = "é🦀\r\n E\u{301}lise";
    let record = json!({"id":u64::MAX,"text":text});
    let request=json!({"name":"detached-audit","question":{"function":"annotate","field":"/text","questions":{
        "action":{"function":"find","question":"Which line?"},
        "uncertain":{"function":"decide","question":"Unsure?","threshold":{"low":0.2,"high":0.8}},
        "failed":{"function":"decide","question":"Failed?"}
    }},"records":[record,record]}).to_string();
    for unit in [0, 1] {
        // SAFETY: each child and buffer is independently owned until its explicit cleanup.
        unsafe {
            let mut parent = row_input_at(&server, &request, unit, None, 2);
            let parent_json = decode(saved(&parent));
            assert_eq!(parent_json["position"], 2);
            assert_eq!(parent_json["id"], json!(u64::MAX));
            assert_eq!(parent_json["source"]["name"], "detached-audit");
            let mut children = Vec::new();
            for name in ["action", "uncertain", "failed"] {
                let mut child: HostRow = zeroed();
                child.struct_size = size_of::<HostRow>() as u32;
                assert_eq!(
                    thinkthen_row_answer_entry(
                        &parent,
                        name.as_ptr(),
                        name.len(),
                        &mut child,
                        ptr::null_mut()
                    ),
                    0
                );
                assert!(parent_json["answer"][name].get("position").is_none());
                assert!(parent_json["answer"][name].get("source").is_none());
                children.push((name, child));
            }
            thinkthen_row_free(&mut parent);
            for (name, mut child) in children {
                detached_metadata(&child, &parent_json, (name == "action").then_some(text));
                let value = decode(saved(&child));
                let loaded: thinkthen::results::AnyRow = serde_json::from_value(value.clone())
                    .expect("a detached saved entry must reload as an ordinary canonical row");
                assert_eq!(loaded.position(), 2);
                assert_eq!(value["id"], parent_json["id"]);
                assert_eq!(value["source"], parent_json["source"]);
                let mut expected = parent_json["answer"][name].clone();
                for key in ["position", "id", "source"] {
                    expected[key] = parent_json[key].clone();
                }
                assert_eq!(value, expected);
                let mut projected = Buffer::default();
                assert_eq!(
                    thinkthen_row_host_json(&child, &mut projected, ptr::null_mut()),
                    0
                );
                let host = decode(projected);
                assert_eq!(host["position"], 2);
                assert_eq!(host["source"], parent_json["source"]);
                if name == "action" {
                    assert_eq!(value["status"], "answered");
                    assert_eq!(value["answer"]["start"], 4);
                    let offsets = host_span(&child);
                    assert_eq!(offsets, if unit == 0 { (8, 16) } else { (5, 12) });
                    assert_eq!(host["answer"]["start"], offsets.0);
                } else {
                    assert_eq!(
                        value["status"],
                        if name == "uncertain" {
                            "unsure"
                        } else {
                            "failed"
                        }
                    );
                }
                thinkthen_row_free(&mut child);
            }
        }
    }
}
