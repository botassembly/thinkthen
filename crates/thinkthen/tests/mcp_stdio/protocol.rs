use super::session::Session;
use conformance_backend::Backend;
use serde_json::{Value, json};

#[test]
fn catalog_help_command_explains_native_arguments_and_links_full_grammar() {
    use crate::child::ChildEnvironment as _;
    let home = std::env::temp_dir().join(format!("thinkthen-mcp-help-{}", std::process::id()));
    std::fs::create_dir(&home).unwrap();
    let output = crate::child::run::output(
        std::process::Command::new(env!("CARGO_BIN_EXE_thinkthen"))
            .args(["mcp", "--help"])
            .clear_environment()
            .isolated_home(&home)
            .env("LANG", "C.UTF-8"),
    );
    std::fs::remove_dir_all(home).unwrap();
    let output = output.unwrap();
    assert!(output.status.success());
    assert!(output.stderr.is_empty());
    let help = String::from_utf8(output.stdout).unwrap();
    for guidance in [
        "question_file, question_name, or question_reference",
        "options holds applicable native controls",
        "native admission validates the full grammar",
        "Annotate uses a versioned questions object",
        "rank accepts a decide/score question or an ordered question set",
        "https://github.com/botassembly/thinkthen/blob/main/specification/mcp.md",
        "https://github.com/botassembly/thinkthen/blob/main/libraries/mcp/README.md",
    ] {
        assert!(help.contains(guidance), "missing MCP guidance: {guidance}");
    }
}

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
    assert_catalog_contract(&catalog).unwrap();
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

fn assert_catalog_contract(catalog: &Value) -> Result<(), &'static str> {
    let contract: Value = serde_json::from_str(include_str!("../../../../conformance/cases.json"))
        .map_err(|_| "invalid fixture contract")?;
    let tools = catalog
        .pointer("/result/tools")
        .and_then(Value::as_array)
        .ok_or("missing catalog tools")?;
    assert_eq!(
        tools
            .iter()
            .map(|tool| tool.get("name").cloned().unwrap_or(Value::Null))
            .collect::<Vec<_>>(),
        *contract
            .pointer("/parity/functions")
            .and_then(Value::as_array)
            .ok_or("missing fixture functions")?
    );
    assert!(
        serde_json::to_vec(catalog)
            .map_err(|_| "catalog serialization failed")?
            .len()
            < 25_000,
        "catalog exceeds discovery budget"
    );
    for tool in tools {
        assert_eq!(
            tool.pointer("/inputSchema/additionalProperties"),
            Some(&json!(false))
        );
        assert!(
            tool.pointer("/inputSchema/properties/options/properties/api_key")
                .is_none()
        );
        assert!(tool.get("outputSchema").is_none());
        assert!(tool.pointer("/inputSchema/$defs").is_none());
        assert!(
            tool.get("description")
                .and_then(Value::as_str)
                .is_some_and(|description| description.contains("thinkthen mcp --help"))
        );
    }
    Ok(())
}

#[test]
fn valid_tool_calls_before_ready_refuse_without_backend_sends() {
    let backend = Backend::start().unwrap();
    let mut session = Session::launch(&format!("{}/generic/v1", backend.origin()));
    for initialized in [false, true] {
        if initialized {
            let reply = session.request(
                "initialize",
                json!({
                    "protocolVersion":"2025-11-25","capabilities":{},
                    "clientInfo":{"name":"outside-in","version":"1"}
                }),
            );
            assert_eq!(reply["result"]["protocolVersion"], "2025-11-25");
        }
        assert_eq!(
            session.request(
                "tools/call",
                json!({"name":"decide","arguments":{"question":"Q?","evidence":"x"}}),
            )["error"]["code"],
            -32000
        );
        assert_eq!(backend.count(), 0);
    }
    session.finish();
}
