//! Public request overlays retain native precedence, descriptions and list admission.
use super::*;

fn preview(engine: &Engine, function: &str, question: &str, options: &str) -> String {
    let request = Request::from_json(&format!(r#"{{"schema":"thinkthen.request/1","call":{{"function":"{function}","question":{{"kind":"definition","value":{question}}},"input":{{"kind":"text","text":"evidence"}},"options":{options}}}}}"#))
        .unwrap().admit().unwrap();
    let plan = engine.plan_request(&request, RequestEnvironment::default()).unwrap();
    String::from_utf8(plan.first_body().unwrap().to_vec()).unwrap()
}

#[test]
fn explicit_meanings_replace_saved_content_and_absence_retains_it() {
    let listener = Listener::answering(response).unwrap();
    let engine = engine(&listener);
    let question = r#"{"decide":"Fits?","true":"saved_yes_marker","false":"saved_no_marker"}"#;
    for (options, present, absent) in [
        (r#"{}"#, vec!["saved_yes_marker", "saved_no_marker"], vec![]),
        (r#"{"yes":{"definition":"replacement_yes_marker"}}"#, vec!["replacement_yes_marker", "saved_no_marker"], vec!["saved_yes_marker"]),
        (r#"{"no":["replacement_no_marker"]}"#, vec!["saved_yes_marker", "replacement_no_marker"], vec!["saved_no_marker"]),
        (r#"{"yes":null,"no":null}"#, vec![], vec!["saved_yes_marker", "saved_no_marker"]),
    ] {
        let body = preview(&engine, "decide", question, options);
        for marker in present { assert!(body.contains(marker), "{body}"); }
        for marker in absent { assert!(!body.contains(marker), "{body}"); }
    }
    assert_eq!(listener.count(), 0);
}

#[test]
fn whole_lists_replace_saved_lists_preserving_order_and_description_content() {
    let listener = Listener::answering(response).unwrap();
    let engine = engine(&listener);
    for (function, question, key) in [
        ("choose", r#"{"choose":"Which?","options":["saved_a_marker","saved_b_marker"]}"#, "labels"),
        ("tag", r#"{"tag":"Which?","labels":["saved_a_marker","saved_b_marker"]}"#, "labels"),
        ("score", r#"{"score":"Which?","levels":["saved_a_marker","saved_b_marker"]}"#, "levels"),
    ] {
        let options = format!(r#"{{"{key}":[{{"name":"replacement_z_marker","description":{{"detail":"description_z_marker"}}}},{{"name":"replacement_a_marker","description":["description_a_marker"]}}]}}"#);
        let body = preview(&engine, function, question, &options);
        assert!(!body.contains("saved_a_marker"));
        assert!(!body.contains("saved_b_marker"));
        assert!(body.find("replacement_z_marker").unwrap() < body.find("replacement_a_marker").unwrap());
        assert!(body.contains("description_z_marker"));
        assert!(body.contains("description_a_marker"));
        for list in ["[]", r#"[{"name":"duplicate"},{"name":"duplicate"}]"#, r#"[{"name":" "},{"name":"valid"}]"#] {
            let request = Request::from_json(&format!(r#"{{"schema":"thinkthen.request/1","call":{{"function":"{function}","question":{{"kind":"definition","value":{question}}},"input":{{"kind":"text","text":"evidence"}},"options":{{"{key}":{list}}}}}}}"#)).unwrap();
            assert!(request.admit().is_err());
        }
    }
    assert_eq!(listener.count(), 0);
}

#[test]
fn wording_controls_keep_wire_presence_and_refuse_wrong_verbs_before_file_reads() {
    let options: RequestOptions = serde_json::from_str(r#"{"yes":null,"labels":[{"name":"z"},{"name":"a","description":null}]}"#).unwrap();
    let serialized = serde_json::to_value(options).unwrap();
    assert!(serialized.get("yes").unwrap().is_null());
    assert!(serialized.get("no").is_none());
    assert!(serialized["labels"][0].get("description").is_none());
    assert!(serialized["labels"][1].get("description").unwrap().is_null());
    assert!(serde_json::from_str::<RequestOptions>(r#"{"labels":null}"#).is_err());
    for (function, options) in [("score", r#"{"yes":"meaning"}"#), ("decide", r#"{"labels":[]}"#), ("choose", r#"{"levels":[]}"#)] {
        let request = Request::from_json(&format!(r#"{{"schema":"thinkthen.request/1","call":{{"function":"{function}","question":{{"kind":"file","path":"absent-wording-file"}},"input":{{"kind":"text","text":"evidence"}},"options":{options}}}}}"#)).unwrap();
        assert_eq!(request.admit().unwrap_err().kind(), ErrorKind::Usage);
    }
}
