//! Native host projection retains absence, authored null and opaque caller values.
use conformance_backend::{Backend, CASES, Canned, Listener};
use serde_json::{Value, json};
use thinkthen::{Engine, Request, RequestEnvironment, RequestValueTree};

fn field<'a>(tree: &'a RequestValueTree, name: &str) -> &'a RequestValueTree {
    let RequestValueTree::Map(fields) = tree else {
        panic!("expected a native map");
    };
    fields.get(name).expect("native field")
}

#[test]
fn partial_annotation_absence_and_find_none_are_distinct() -> Result<(), Box<dyn std::error::Error>> {
    let cases: Value = serde_json::from_str(CASES)?;
    let backend = Backend::start()?;
    for id in ["17-annotate-partial", "19-find-none"] {
        let case = cases["cases"]
            .as_array()
            .expect("shared cases")
            .iter()
            .find(|case| case["id"] == id)
            .expect("shared case");
        let verb = case["verb"].as_str().expect("function");
        let mut question = case
            .get("question_set")
            .unwrap_or(&case["question"])
            .clone();
        let (input, options) = if verb == "find" {
            let fields = question.as_object_mut().expect("find definition");
            let units = fields.remove("units").expect("candidate set");
            let none = fields.remove("none").expect("none setting");
            let items: Vec<_> = units
                .as_array()
                .expect("units")
                .iter()
                .map(|text| json!({"original":{"kind":"text","text":text}}))
                .collect();
            (
                json!({"kind":"units","items":items}),
                json!({"none":none}),
            )
        } else {
            (
                json!({"kind":"text","text":case["exchanges"][0]["evidence"]}),
                json!({}),
            )
        };
        let request = Request::named_from_json(
            verb,
            &format!(r#"{{"kind":"definition","value":{question}}}"#),
            &input.to_string(),
            &options.to_string(),
        )?;
        let engine = Engine::builder()
            .base_url(&format!("{}/case/{id}/v1", backend.origin()))?
            .api_key("sk-tree-loopback")?
            .no_cache()
            .max_retries(0)
            .build()?;
        let call = engine.call(verb, request.call.arguments, RequestEnvironment::default())?;
        let tree = call.value_tree()?;
        let value = field(&tree, "value");
        if verb == "annotate" {
            assert_eq!(field(value, "team"), &RequestValueTree::Absent);
            assert_eq!(field(value, "refund"), &RequestValueTree::Null);
        } else {
            assert_eq!(value, &RequestValueTree::Null);
            assert_eq!(field(&tree, "index"), &RequestValueTree::Absent);
            assert_eq!(
                field(&tree, "resolution"),
                &RequestValueTree::String("resolved".into())
            );
        }
    }
    Ok(())
}

#[test]
fn authored_objects_keep_exact_numbers_and_native_looking_keys() -> Result<(), Box<dyn std::error::Error>> {
    let listener = Listener::answering(|_| {
        Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.9}}}"#)
    })?;
    let engine = Engine::builder()
        .base_url(listener.base())?
        .api_key("sk-tree-loopback")?
        .no_cache()
        .build()?;
    let request = Request::named_from_json(
        "decide",
        r#"{"kind":"definition","value":{"decide":"Fits?","true":{"resolution":"failed","value":null,"Absent":"caller-secret","number":18446744073709551615,"decimal":0.125}}}"#,
        r#"{"kind":"text","text":"evidence"}"#,
        "{}",
    )?;
    let call = engine.call("decide", request.call.arguments, RequestEnvironment::default())?;
    let tree = call.value_tree()?;
    let value = field(&tree, "value");
    assert_eq!(field(value, "value"), &RequestValueTree::Null);
    assert_eq!(
        field(value, "resolution"),
        &RequestValueTree::String("failed".into())
    );
    assert_eq!(
        field(value, "Absent"),
        &RequestValueTree::String("caller-secret".into())
    );
    assert_eq!(
        field(value, "number"),
        &RequestValueTree::Number("18446744073709551615".into())
    );
    assert_eq!(field(value, "decimal"), &RequestValueTree::Number("0.125".into()));
    assert!(!format!("{tree:?}").contains("caller-secret"));
    assert_eq!(listener.count(), 1);
    Ok(())
}
