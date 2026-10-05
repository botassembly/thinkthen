//! Local runtime names use the production request path with no key.

use super::ollama::mimic;
use super::support::{Home, Proxy, said};
use serde_json::{Value, json};

const NAMES: [&str; 10] = [
    "decide",
    "choose",
    "tag",
    "score",
    "filter",
    "rank",
    "find",
    "annotate",
    "recognize",
    "relate",
];

#[test]
fn local_runtime_defaults_and_string_score_workaround_match_the_supported_servers() {
    for (name, variable, model, base) in [
        (
            "llamacpp",
            "LLAMACPP_API_KEY",
            "local",
            "http://localhost:8080/v1",
        ),
        (
            "mlx",
            "MLX_API_KEY",
            "strands-decider-2B-hobson-v19",
            "http://localhost:8000/v1",
        ),
    ] {
        let home = Home::new(name);
        let status = home.run(&["status", "--backend", name], &[]);
        let (stdout, stderr) = said(&status);
        assert_eq!((status.status.code(), stderr.as_str()), (Some(0), ""));
        assert!(stdout.contains(&format!("backend {name}\nurl {base}/systemone\n")));
        assert!(stdout.contains(&format!(
            "model {model}\nmodel_source backend\nkey_variable {variable}\napi_key_set false\n"
        )));
        let target = mimic(
            |question| {
                question["type"] == "score"
                    && question["criteria"]
                        .as_array()
                        .is_some_and(|levels| levels.iter().any(|level| !level.is_string()))
            },
            422,
        );
        let output = home.run(
            &[
                "backends",
                "check",
                "--backend",
                name,
                "--url",
                target.base(),
            ],
            &[],
        );
        let (stdout, stderr) = said(&output);
        assert_eq!(stderr, "");
        assert_eq!(
            output.status.code(),
            Some(if name == "mlx" { 0 } else { 4 }),
            "{stdout}"
        );
        for function in NAMES {
            assert!(
                stdout.contains(&format!("answered function {function}\n")),
                "{stdout}"
            );
        }
        let sent = target.requests();
        assert!(
            sent.iter()
                .all(|request| request.header("authorization").is_none())
        );
        let score: Value = serde_json::from_slice(&sent[2].body).expect("score wire");
        let expected = if name == "mlx" {
            json!([
                "fair",
                "The box was dented but the contents were fine.",
                "The box was intact"
            ])
        } else {
            json!([{}, "The box was dented but the contents were fine.", {"what":"The box was intact", "not_for":"a dented box"}])
        };
        assert_eq!(score["questions"]["q1"]["criteria"], expected);
        assert_eq!(score["model"], model);
        if name == "mlx" {
            assert!(stdout.contains("warning score: backend `mlx`"));
            assert!(!stdout.contains("Ollama"));
        } else {
            assert!(stdout.contains("critical score: the backend answered with status 422"));
        }
        home.assert_no_marker_in_files();
    }
}

#[test]
fn new_local_keys_cannot_go_to_a_hosted_builtin_and_missing_keys_send_nothing() {
    let proxy = Proxy::start();
    for (name, variable) in [("llamacpp", "LLAMACPP_API_KEY"), ("mlx", "MLX_API_KEY")] {
        let home = Home::new(name);
        let output = home.run(
            &[
                "backends",
                "check",
                "--backend",
                name,
                "--url",
                "https://local.example/v1",
            ],
            &[("HTTPS_PROXY", &proxy.url)],
        );
        let (stdout, stderr) = said(&output);
        assert_eq!(stdout, "");
        assert_eq!(
            stderr,
            format!(
                "thinkthen: the environment variable `{variable}` is unset or blank, so no key is sent\n"
            )
        );
        assert_eq!(output.status.code(), Some(4));
        let output = home.run(
            &[
                "backends",
                "check",
                "--backend",
                name,
                "--url",
                "https://api.typesafe.ai/v1",
            ],
            &[
                (variable, "fake-local-runtime-secret"),
                ("HTTPS_PROXY", &proxy.url),
            ],
        );
        let (stdout, stderr) = said(&output);
        assert_eq!(stdout, "");
        assert_eq!(
            stderr,
            format!(
                "thinkthen: backend `{name}` reads `{variable}`, the key of backend `{name}`, which never goes to the address of backend `typesafe`\n"
            )
        );
        assert_eq!(output.status.code(), Some(2));
        assert!(!stderr.contains("fake-local-runtime-secret"));
    }
    assert_eq!(proxy.count(), 0);
}

#[test]
fn minimal_recognition_follows_its_answer_and_empty_relations_still_answer() {
    use conformance_backend::{Canned, Listener};
    for boundary in ["OUT", "SINGLE"] {
        let target = Listener::answering(move |body| {
            let request: Value = serde_json::from_slice(body).expect("wire");
            let answers = request["questions"].as_object().expect("questions").iter().map(|(name, question)| {
                let answer = stage_answer(question, boundary);
                format!("\"{name}\":{answer}")
            }).collect::<Vec<_>>().join(",");
            Canned::ok(&format!(r#"{{"model":{},"answers":{{{answers}}},"usage":{{"input_tokens":1,"output_tokens":1}}}}"#, request["model"]))
        }).expect("server");
        let home = Home::new("local-stages");
        let output = home.run(
            &[
                "backends",
                "check",
                "--backend",
                "mlx",
                "--url",
                target.base(),
            ],
            &[],
        );
        let (stdout, stderr) = said(&output);
        assert_eq!(
            (output.status.code(), stderr.as_str()),
            (Some(0), ""),
            "{stdout}"
        );
        for name in NAMES {
            assert!(
                stdout.contains(&format!("answered function {name}\n")),
                "{stdout}"
            );
        }
        let sent = target.requests();
        assert_eq!(sent.len(), if boundary == "OUT" { 14 } else { 15 });
        let recognition: Value = serde_json::from_slice(&sent[12].body).expect("boundary request");
        assert!(
            recognition["questions"]["q1"]["criteria"]
                .get("SINGLE")
                .is_some()
        );
        if boundary == "SINGLE" {
            let kind: Value = serde_json::from_slice(&sent[13].body).expect("kind request");
            assert!(kind["questions"]["q1"]["criteria"].get("person").is_some());
        }
        let relation: Value = serde_json::from_slice(&sent.last().expect("relation request").body)
            .expect("relation wire");
        assert_eq!(
            relation["questions"]
                .as_object()
                .expect("ordered pairs")
                .len(),
            2
        );
        assert!(
            sent.iter()
                .all(|request| request.header("authorization").is_none())
        );
    }
}

#[test]
fn local_text_and_authored_descriptions_cannot_reuse_each_others_cached_answer() {
    let target = mimic(|_| false, 200);
    let home = Home::new("local-cache");
    let question = home.path("question.json");
    std::fs::write(&question, r#"{"choose":"Which team?","options":{"billing":{"what":"Money matters","not_for":"shipping"},"other":"Anything else."}}"#).expect("question");
    let argument = format!("@{question}");
    let evidence = home.evidence(&["Please refund my order."]);
    for (name, requests) in [("mlx", 1), ("llamacpp", 2), ("mlx", 2), ("llamacpp", 2)] {
        let output = home.run(
            &[
                "choose",
                &argument,
                "--backend",
                name,
                "--model",
                "local",
                "--url",
                target.base(),
                "--input",
                &evidence,
            ],
            &[],
        );
        let (stdout, stderr) = said(&output);
        assert_eq!(
            (output.status.code(), stderr.as_str()),
            (Some(0), ""),
            "{stdout}"
        );
        assert_eq!(target.count(), requests);
    }
}

#[test]
fn mlx_plan_names_its_own_workaround_and_sends_nothing() {
    let target = mimic(|_| false, 200);
    let home = Home::new("mlx-plan");
    let output = home.run(
        &[
            "backends",
            "check",
            "--backend",
            "mlx",
            "--url",
            target.base(),
            "--plan",
        ],
        &[],
    );
    let (stdout, stderr) = said(&output);
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(
        stderr,
        "thinkthen: backend `mlx` sends each description object as its `what` text, a temporary workaround for the Strands server schema, so its other fields are left out\n"
    );
    assert!(stdout.contains("\"criteria\":[\"fair\",\"The box was dented but the contents were fine.\",\"The box was intact\"]"));
    assert_eq!(target.count(), 0);
}

#[test]
fn a_function_profile_limit_refuses_the_whole_check_before_any_request() {
    let target = mimic(|_| false, 200);
    let home = Home::new("local-profile");
    home.config(r#"{"schema":"thinkthen.config/1","backends":{"mlx":{"profile":{"schema":"thinkthen.backend-profile/1","name":"narrow","max_options":4}}}}"#);
    for flags in [&[][..], &["--plan"][..]] {
        let arguments = [
            &[
                "backends",
                "check",
                "--backend",
                "mlx",
                "--url",
                target.base(),
            ][..],
            flags,
        ]
        .concat();
        let output = home.run(&arguments, &[]);
        let (stdout, stderr) = said(&output);
        assert_eq!(stdout, "");
        assert_eq!(output.status.code(), Some(2), "{stderr}");
        assert_eq!(
            stderr,
            "thinkthen: profile narrow allows at most 4 options; this request has 5\n"
        );
        assert_eq!(target.count(), 0);
    }
}

fn stage_answer(question: &Value, boundary: &str) -> String {
    if question["type"] == "noul" {
        return r#"{"type":"noul","noul":0.0}"#.to_owned();
    }
    if question["criteria"].get("BEGIN").is_some() {
        let distribution = ["BEGIN", "INSIDE", "END", "SINGLE", "OUT"]
            .map(|tag| format!("\"{tag}\":{}", u8::from(tag == boundary)))
            .join(",");
        return format!(r#"{{"type":"choice","probabilities":{{{distribution}}}}}"#);
    }
    super::ollama::answer(question)
}
