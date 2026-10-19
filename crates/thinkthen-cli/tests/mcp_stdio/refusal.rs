use super::session::Session;
use conformance_backend::Backend;
use serde_json::json;

#[test]
fn invalid_selectors_and_credentials_refuse_before_backend_dispatch() {
    let backend = Backend::start().unwrap();
    let mut session = Session::launch(&format!("{}/generic/v1", backend.origin()));
    session.initialize();
    for arguments in [
        json!({"question":"q","question_file":"absent","evidence":"x"}),
        json!({"question":"q","evidence":"x","records":[]}),
        json!({"question":"q","evidence":"x","inputs":[]}),
        json!({"question":"q","inputs":[{"text":"x","json":null}]}),
        json!({"question":"q","source":{"paths":[]}}),
        json!({"question":"q","source":{"paths":["absent"],"unit":"file","window":2}}),
        json!({"question":"q","evidence":"x","unrecognized":"fake-secret"}),
        json!({"question":"q","evidence":"x","options":{"batch":0}}),
        json!({"question":"q","evidence":"x","options":{"field":"invalid-pointer"}}),
        json!({"question":"q","evidence":"x","options":{"deadline_ms":null}}),
    ] {
        let reply = session.request("tools/call", json!({"name":"decide","arguments":arguments}));
        assert_refused(&reply);
        assert!(!reply.to_string().contains("fake-secret"));
    }
    let contract: serde_json::Value =
        serde_json::from_str(include_str!("../../../../conformance/cases.json")).unwrap();
    for tool in contract["parity"]["functions"].as_array().unwrap() {
        let reply = session.request("tools/call", json!({"name":tool,"arguments":{
            "question":"private-question","evidence":"private-evidence","options":{"api_key":"fake-secret"}
        }}));
        assert_eq!(reply["error"]["code"], -32602);
        for secret in ["fake-secret", "private-question", "private-evidence"] {
            assert!(!reply.to_string().contains(secret));
        }
    }
    assert_eq!(backend.count(), 0);
    session.finish();
}

#[test]
fn initially_cancelled_and_expired_calls_preserve_errors_and_send_nothing() {
    let backend = Backend::start().unwrap();
    let mut session = Session::launch(&format!("{}/generic/v1", backend.origin()));
    session.initialize();
    for (options, kind) in [
        (json!({"cancelled":true}), "cancelled"),
        (json!({"deadline_ms":0}), "deadline"),
    ] {
        let result = session.call(
            "decide",
            json!({"question":"Q?","evidence":"x","options":options}),
        );
        assert_eq!(result["isError"], true);
        assert_eq!(result["structuredContent"]["error"]["kind"], kind);
        assert!(result["structuredContent"].get("value").is_none());
    }
    assert_eq!(backend.count(), 0);
    session.finish();
}

#[test]
fn text_only_tools_refuse_images_before_open_or_send() {
    let backend = Backend::start().unwrap();
    let mut session = Session::launch(&format!("{}/generic/v1", backend.origin()));
    session.initialize();
    let contract: serde_json::Value =
        serde_json::from_str(include_str!("../../../../conformance/cases.json")).unwrap();
    for tool in [
        "tag",
        "filter",
        "rank",
        "find",
        "annotate",
        "recognize",
        "relate",
    ] {
        let case = contract["cases"]
            .as_array()
            .unwrap()
            .iter()
            .find(|case| case["verb"] == tool)
            .unwrap();
        let question = if ["filter", "rank", "find"].contains(&tool) {
            json!("Q?")
        } else {
            case.get("question")
                .or_else(|| case.get("question_set"))
                .unwrap()
                .clone()
        };
        for arguments in [
            json!({"question":question,"images":["does-not-exist.png"]}),
            json!({"question":question,"inputs":[{"text":"x","images":["does-not-exist.png"]}]}),
            json!({"question":question,"source":{"paths":["does-not-exist"],"unit":"file","media":"image"}}),
        ] {
            let reply = session.request("tools/call", json!({"name":tool,"arguments":arguments}));
            assert_refused(&reply);
            assert!(
                reply["result"]["structuredContent"]["error"]["message"]
                    .as_str()
                    .unwrap()
                    .contains("image"),
                "{reply}"
            );
        }
    }
    assert_eq!(backend.count(), 0);
    session.finish();
}

fn assert_refused(reply: &serde_json::Value) {
    if let Some(error) = reply.get("error") {
        assert_eq!(error.get("code"), Some(&json!(-32602)));
    } else {
        assert_eq!(
            reply.pointer("/result/isError"),
            Some(&json!(true)),
            "{reply}"
        );
        assert_eq!(
            reply.pointer("/result/structuredContent/error/kind"),
            Some(&json!("usage"))
        );
        assert!(reply.pointer("/result/structuredContent/facts").is_none());
    }
}
