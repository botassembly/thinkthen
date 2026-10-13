use super::session::Session;
use conformance_backend::Backend;
use serde_json::{Value, json};

#[test]
fn initialization_gates_tools_and_catalog_preserves_the_ten_tool_contract() {
    let backend = Backend::start().unwrap();
    let mut session = Session::launch(&format!("{}/generic/v1", backend.origin()));
    assert_eq!(
        session.request("tools/list", json!({}))["error"]["code"],
        -32000
    );
    let initialized = session.request(
        "initialize",
        json!({
            "protocolVersion":"unsupported", "capabilities":{},
            "clientInfo":{"name":"outside-in","version":"1"}
        }),
    );
    assert_eq!(initialized["result"]["protocolVersion"], "2025-11-25");
    assert_eq!(
        initialized["result"]["capabilities"],
        json!({"tools":{"listChanged":false}})
    );
    assert_eq!(
        session.request("tools/list", json!({}))["error"]["code"],
        -32000
    );
    session.send(json!({"jsonrpc":"2.0","method":"notifications/initialized"}));
    let catalog = session.request("tools/list", json!({}));
    let contract: Value =
        serde_json::from_str(include_str!("../../../../conformance/cases.json")).unwrap();
    let tools = catalog["result"]["tools"].as_array().unwrap();
    assert_eq!(
        tools
            .iter()
            .map(|tool| tool["name"].clone())
            .collect::<Vec<_>>(),
        *contract["parity"]["functions"].as_array().unwrap()
    );
    for tool in tools {
        assert_eq!(tool["inputSchema"]["additionalProperties"], false);
        assert!(
            tool["inputSchema"]["properties"]["options"]["properties"]
                .get("api_key")
                .is_none()
        );
        assert!(tool["outputSchema"]["$defs"].get("completeFacts").is_some());
    }
    assert_eq!(
        session.request("initialize", json!({}))["error"]["code"],
        -32600
    );
    for method in ["resources/list", "prompts/list", "private-payload"] {
        let reply = session.request(method, json!({}));
        assert_eq!(reply["error"]["code"], -32601);
        assert!(!reply.to_string().contains("private-payload"));
    }
    for id in [json!("7"), json!(7), json!(18446744073709551615u64)] {
        session.send(json!({"jsonrpc":"2.0","id":id,"method":"ping"}));
        let reply = session.receive();
        assert_eq!(reply["id"], id);
        assert_eq!(reply["result"], json!({}));
    }
    assert_eq!(backend.count(), 0);
    session.finish();
}
