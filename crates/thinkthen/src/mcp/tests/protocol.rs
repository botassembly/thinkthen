use super::super::{
    output::{NativeObject, tool_result},
    protocol::{self, Fault, Id},
    tools,
};
use serde_json::{Value, json};
use std::io::Cursor;

#[test]
fn malformed_envelopes_do_not_echo_payloads_or_admit_null_ids() {
    for text in [
        r#"{"jsonrpc":"2.0","id":null,"method":"ping"}"#,
        r#"{"jsonrpc":"2.0","id":true,"method":"ping"}"#,
        r#"{"jsonrpc":"2.0","id":1.5,"method":"ping"}"#,
        r#"{"jsonrpc":"1.0","id":1,"method":"private-payload"}"#,
        r#"{"jsonrpc":"2.0","id":1,"id":2,"method":"private-payload"}"#,
        r#"[{"jsonrpc":"2.0","id":1,"method":"ping"}]"#,
    ] {
        let fault = protocol::parse(text.as_bytes()).expect_err(text);
        assert_eq!(fault.code, Fault::REQUEST.code);
        assert!(!fault.message.contains("private-payload"));
    }
    assert_eq!(
        protocol::parse(b"not JSON private-payload")
            .unwrap_err()
            .code,
        -32700
    );
}

#[test]
fn string_and_integer_ids_survive_without_coercion_and_debug_withholds_data() {
    for id in [json!("secret-id"), json!(7), json!(18446744073709551615u64)] {
        let wire = json!({"jsonrpc":"2.0","id":id,"method":"ping","params":{"secret":"private"}});
        let message = protocol::parse(wire.to_string().as_bytes()).expect("message");
        assert_eq!(
            serde_json::to_value(message.id.as_ref().unwrap()).unwrap(),
            id
        );
        assert!(!format!("{message:?}").contains("private"));
        assert!(!format!("{:?}", message.id).contains("secret-id"));
    }
    assert_ne!(Id::Signed(7), Id::Text("7".into()));
}

#[test]
fn framing_bounds_oversize_and_rejects_unterminated_utf8() {
    let mut exact = vec![b' '; protocol::MAX_MESSAGE - 1];
    exact.push(b'\n');
    assert_eq!(
        protocol::read_line(&mut Cursor::new(exact))
            .unwrap()
            .unwrap()
            .unwrap()
            .len(),
        protocol::MAX_MESSAGE
    );
    let excess = vec![b'x'; protocol::MAX_MESSAGE + 1];
    assert_eq!(
        protocol::read_line(&mut Cursor::new(excess))
            .unwrap()
            .unwrap()
            .unwrap_err()
            .code,
        Fault::SIZE.code
    );
    assert_eq!(
        protocol::read_line(&mut Cursor::new(b"{}"))
            .unwrap()
            .unwrap()
            .unwrap_err()
            .code,
        -32700
    );
    assert_eq!(protocol::parse(&[0xff, b'\n']).unwrap_err().code, -32700);
}

#[test]
fn false_null_and_partial_failure_are_preserved_as_successful_native_objects() {
    for value in [
        json!({"value":false}),
        json!({"value":null}),
        json!({"values":[null,false],"failed_questions":1,"error":{"kind":"backend"}}),
    ] {
        let reply =
            serde_json::to_value(tool_result(NativeObject::new(&value).unwrap(), false)).unwrap();
        assert_eq!(reply["isError"], false);
        assert_eq!(reply["structuredContent"], value);
        let text: Value =
            serde_json::from_str(reply["content"][0]["text"].as_str().unwrap()).unwrap();
        assert_eq!(text, value);
    }
    assert_eq!(
        serde_json::to_value(tool_result(
            NativeObject::new(&json!({"error":{"kind":"backend"}})).unwrap(),
            true
        ))
        .unwrap()["isError"],
        true
    );
}

#[test]
fn catalog_names_match_independent_function_inventory_and_only_advertise_tools() {
    let contract: Value =
        serde_json::from_str(include_str!("../../../../../conformance/cases.json")).unwrap();
    let list = tools::list(&json!({"type":"object","required":["native_fixture"]}));
    let names: Vec<_> = list["tools"]
        .as_array()
        .unwrap()
        .iter()
        .map(|tool| tool["name"].clone())
        .collect();
    assert_eq!(
        names,
        contract["parity"]["functions"].as_array().unwrap().to_vec()
    );
    for tool in list["tools"].as_array().unwrap() {
        assert_eq!(tool["inputSchema"]["additionalProperties"], false);
        assert_eq!(tool["outputSchema"]["required"], json!(["native_fixture"]));
        assert!(
            tool["inputSchema"]["properties"]["options"]["properties"]
                .get("api_key")
                .is_none()
        );
    }
}

#[test]
fn native_serialization_keeps_authored_probability_order_in_both_content_forms() {
    let raw = serde_json::value::RawValue::from_string(
        r#"{"probabilities":{"zebra":0.1,"alpha":0.9},"value":null}"#.into(),
    )
    .unwrap();
    let native = NativeObject::new(&raw).unwrap();
    let text = serde_json::to_string(&tool_result(native, false)).unwrap();
    assert!(text.contains(
        r#""structuredContent":{"probabilities":{"zebra":0.1,"alpha":0.9},"value":null}"#
    ));
    let value: Value = serde_json::from_str(&text).unwrap();
    assert_eq!(value["content"][0]["text"], raw.get());
}
