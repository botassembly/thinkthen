//! Generated packet conversion preserves extension bytes and arbitrary JSON order.
#![allow(
    unsafe_code,
    clippy::expect_used,
    clippy::indexing_slicing,
    reason = "fixture borrows only live owner-backed C views"
)]
use super::*;
use crate::complete_session_views as generated;

fn text(value: thinkthen_complete_utf8_v1) -> String {
    if value.len == 0 {
        return String::new();
    }
    // SAFETY: this helper reads only live projection-owned buffers.
    let bytes = unsafe { std::slice::from_raw_parts(value.data.cast::<u8>(), value.len) };
    String::from_utf8(bytes.to_vec()).expect("UTF-8 view")
}
#[test]
fn complete_view_retains_missing_null_and_unknown_members_at_each_object_depth() {
    // The terminal/null envelope is the existing generated-reader fixture;
    // extra failure members exercise the same extension rule at nested depths.
    let projection = Projection::new(r#"{"kind":"terminal","facts":null,"failure":{"error":{"kind":"local","message":"safe","retryable":false,"stopped":{"cause":"local","retryable":false,"future_stop":17},"future_error":{"z":1e400,"a":-0}},"future_failure":[false,null]},"future":{"nested":false}}"#).expect("complete fixture");
    // SAFETY: the projection owns this tagged graph throughout these reads.
    let terminal = unsafe {
        let packet = &*projection.root;
        assert_eq!(
            packet.kind,
            generated::THINKTHEN_COMPLETE_SESSION_PACKET_TERMINAL_V1
        );
        &*packet.data.terminal
    };
    assert_eq!(
        terminal.facts.presence,
        generated::THINKTHEN_COMPLETE_PRESENCE_NULL_V1
    );
    assert_eq!(
        terminal.failure.presence,
        generated::THINKTHEN_COMPLETE_PRESENCE_VALUE_V1
    );
    // SAFETY: selected failure arm and all nested buffers share projection lifetime.
    let failure = unsafe { &*terminal.failure.value };
    assert_eq!(
        failure.facts.presence,
        generated::THINKTHEN_COMPLETE_PRESENCE_MISSING_V1
    );
    let error = unsafe { &*failure.error };
    let stopped = unsafe { &*error.stopped };
    for (extensions, key, raw) in [
        (terminal.extensions, "future", r#"{"nested":false}"#),
        (failure.extensions, "future_failure", "[false,null]"),
        (error.extensions, "future_error", r#"{"z":1e400,"a":-0}"#),
        (stopped.extensions, "future_stop", "17"),
    ] {
        assert_eq!(extensions.len, 1);
        let entry = unsafe { &*extensions.data };
        assert_eq!(
            (text(entry.name), text(entry.json)),
            (key.to_owned(), raw.to_owned())
        );
    }
}
#[test]
fn arbitrary_authored_json_keeps_order_and_exact_numeric_tokens_in_owned_views() {
    let mut store = Storage::default();
    let view = {
        let input = String::from(
            r#"{"z":1e400,"a":-0,"later":[null,true,"x",{"outer":{"inner":{"leaf":"kept"}}}]}"#,
        );
        let node = Node::parse(&input).expect("authored JSON");
        generated::read_thinkthen_complete_decide_value_v1(&node, &mut store)
            .expect("arbitrary value")
            .value
    };
    // SAFETY: input node is gone; storage still owns every copied nested view.
    let object = unsafe {
        assert_eq!((*view).kind, THINKTHEN_COMPLETE_JSON_OBJECT_V1);
        (*view).data.object
    };
    let entries = unsafe { std::slice::from_raw_parts(object.data, object.len) };
    assert_eq!(
        entries
            .iter()
            .map(|entry| text(entry.name))
            .collect::<Vec<_>>(),
        ["z", "a", "later"]
    );
    for (entry, expected) in entries.iter().take(2).zip(["1e400", "-0"]) {
        let number = unsafe { &*entry.value };
        assert_eq!(number.kind, THINKTHEN_COMPLETE_JSON_NUMBER_V1);
        assert_eq!(text(unsafe { number.data.number }), expected);
    }
    let array = unsafe { (*entries[2].value).data.array };
    let values = unsafe { std::slice::from_raw_parts(array.data, array.len) };
    assert_eq!(
        values
            .iter()
            .map(|value| unsafe { (**value).kind })
            .collect::<Vec<_>>(),
        [
            THINKTHEN_COMPLETE_JSON_NULL_V1,
            THINKTHEN_COMPLETE_JSON_BOOLEAN_V1,
            THINKTHEN_COMPLETE_JSON_STRING_V1,
            THINKTHEN_COMPLETE_JSON_OBJECT_V1
        ]
    );
    let mut nested = values[3];
    for key in ["outer", "inner", "leaf"] {
        let object = unsafe { (*nested).data.object };
        let entry = unsafe { &*object.data };
        assert_eq!(text(entry.name), key);
        nested = entry.value;
    }
    assert_eq!(unsafe { (*nested).kind }, THINKTHEN_COMPLETE_JSON_STRING_V1);
    assert_eq!(text(unsafe { (*nested).data.string }), "kept");
}
