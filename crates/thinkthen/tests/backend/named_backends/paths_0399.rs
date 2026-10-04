//! Ticket 0399: relative paths, complete criteria, key guards and wire identity.
use super::support::{Home, Proxy, listener, said};
use conformance_backend::{Canned, Listener};
use serde_json::{Value, json};

const PATH_REFUSAL: &str = "thinkthen: configuration backend field `path` must be one or more segments of letters, digits, and `-._~@`, joined by `/`\n";
const TYPE_REFUSAL: &str = "thinkthen: configuration backend entries are objects whose `url`, `path`, `key_env`, and `model` are strings\n";

fn config(base: &str, path: &Value) -> String {
    json!({"schema":"thinkthen.config/1","backends":{"local-pp":{
        "url":base,"path":path,"key_env":"LOCAL_D1_KEY","model":"m"
    }}})
    .to_string()
}

fn decide(home: &Home, flags: &[&str], changes: &[(&str, &str)]) -> std::process::Output {
    let input = home.evidence(&["alpha"]);
    let mut args = vec!["decide", "A?", "--input", &input];
    args.extend_from_slice(flags);
    home.run(&args, changes)
}

#[test]
fn relative_paths_and_builtin_overrides_send_to_exact_suffixes() {
    for path in [
        "decisions",
        "api/alpha/decisions",
        "ai/run/@cf/cloudflare/clef",
        &"x".repeat(128),
    ] {
        let target = listener();
        let base = format!("{}/prefix/v1///", target.base());
        let home = Home::new("paths-0399");
        home.config(&config(&base, &json!(path)));
        let out = decide(&home, &["--backend", "local-pp", "--no-cache"], &[]);
        assert_eq!(out.status.code(), Some(0), "{}", said(&out).1);
        assert_eq!(target.count(), 1);
        let requests = target.requests();
        assert_eq!(
            requests[0].line,
            format!("POST /v1/prefix/v1/{path} HTTP/1.1")
        );
        home.assert_no_marker_in_files();
    }
    for (name, path, variable) in [
        ("perplexity", "decisions", "PERPLEXITY_API_KEY"),
        ("openrouter", "systemone", "OPENROUTER_API_KEY"),
    ] {
        let target = listener();
        let home = Home::new("built-path-0399");
        let out = decide(
            &home,
            &["--backend", name, "--url", target.base(), "--no-cache"],
            &[(variable, "marker-new-0399")],
        );
        assert_eq!(out.status.code(), Some(0), "{}", said(&out).1);
        assert_eq!(target.count(), 1);
        let requests = target.requests();
        assert_eq!(requests[0].line, format!("POST /v1/{path} HTTP/1.1"));
        assert_eq!(
            requests[0].header("authorization"),
            Some("Bearer marker-new-0399")
        );
        assert!(!format!("{}{}", said(&out).0, said(&out).1).contains("marker-new"));
    }
    let target = listener();
    let home = Home::new("unnamed-path-0399");
    let out = decide(&home, &["--url", target.base(), "--no-cache"], &[]);
    assert_eq!(out.status.code(), Some(0));
    assert_eq!(target.count(), 1);
    assert_eq!(target.requests()[0].line, "POST /v1/systemone HTTP/1.1");
}

#[test]
fn invalid_paths_are_value_free_and_send_nothing() {
    let target = listener();
    let home = Home::new("refused-path-0399");
    let mut paths = vec![
        "",
        "/decisions",
        "decisions/",
        "a//b",
        ".",
        "..",
        "a/./b",
        "a/../b",
        "a?b",
        "a#b",
        "a b",
        "%2e",
        "a%2fb",
        "https://x",
        "é",
        "sk-path-marker-0399?secret",
    ]
    .into_iter()
    .map(json_string)
    .collect::<Vec<_>>();
    paths.extend([json!("x".repeat(129)), json!(7), Value::Null, json!(["a"])]);
    for path in paths {
        home.config(&config(target.base(), &path));
        let out = decide(&home, &["--backend", "local-pp"], &[]);
        assert_eq!(out.status.code(), Some(5), "{path}");
        assert_eq!(
            said(&out),
            (
                String::new(),
                if path.is_string() {
                    PATH_REFUSAL
                } else {
                    TYPE_REFUSAL
                }
                .to_owned()
            )
        );
        assert_eq!(target.count(), 0);
    }
    let entry = json!({"schema":"thinkthen.config/1","backends":{"local-pp":{"url":7,"key_env":"K","model":"m"}}});
    home.config(&entry.to_string());
    let out = decide(&home, &["--backend", "local-pp"], &[]);
    assert_eq!(out.status.code(), Some(5));
    assert_eq!(said(&out).1, TYPE_REFUSAL);
    assert_eq!(target.count(), 0);
}

fn json_string(text: &str) -> Value {
    json!(text)
}

#[test]
fn every_new_provider_pair_refuses_before_connecting_or_reading_keys() {
    let proxy = Proxy::start();
    let home = Home::new("guards-0399");
    let hosts = [
        ("typesafe", "api.typesafe.ai", "TYPESAFE_API_KEY"),
        ("liquid", "api.liquid.ai", "LIQUIDAI_API_KEY"),
        ("perplexity", "api.perplexity.ai", "PERPLEXITY_API_KEY"),
        ("openrouter", "openrouter.ai", "OPENROUTER_API_KEY"),
    ];
    for (owner, _, variable) in hosts {
        for (other, host, _) in hosts {
            if owner == other
                || ![owner, other]
                    .iter()
                    .any(|name| ["perplexity", "openrouter"].contains(name))
            {
                continue;
            }
            for host in [
                host.to_owned(),
                format!("{}.", host.to_uppercase()),
                host.replacen('.', "%2E", 1),
            ] {
                let url = format!("https://{host}:443/prefix");
                let out = decide(
                    &home,
                    &["--backend", owner, "--url", &url],
                    &[(variable, ""), ("HTTPS_PROXY", &proxy.url)],
                );
                assert_eq!(out.status.code(), Some(2));
                assert_eq!(
                    said(&out).1,
                    format!(
                        "thinkthen: backend `{owner}` reads `{variable}`, the key of backend `{owner}`, which never goes to the address of backend `{other}`\n"
                    )
                );
                assert_eq!(proxy.count(), 0);
            }
        }
    }
}

#[test]
fn both_sides_preserve_authored_values_and_omit_absent_criteria() {
    let cases = [
        (
            json!({"true":"Yes"}),
            Some(json!({"true":"Yes","false":{}})),
        ),
        (json!({"false":"No"}), Some(json!({"true":{},"false":"No"}))),
        (
            json!({"true":"Yes","false":"No"}),
            Some(json!({"true":"Yes","false":"No"})),
        ),
        (json!({}), None),
        (json!({"true":null,"false":null}), None),
        (
            json!({"true":{"what":"Yes","examples":["x"]}}),
            Some(json!({"true":{"what":"Yes","examples":["x"]},"false":{}})),
        ),
        (
            json!({"true":["x",1],"false":null}),
            Some(json!({"true":["x",1],"false":{}})),
        ),
        (json!({"true":{}}), Some(json!({"true":{},"false":{}}))),
    ];
    let target = listener();
    let home = Home::new("sides-0399");
    for (fields, expected) in cases {
        let mut question = fields.as_object().expect("fields").clone();
        question.insert("decide".into(), json!("A?"));
        let file = home.root.join("question.json");
        std::fs::write(&file, Value::Object(question).to_string()).expect("question file");
        for backend in [Some("openrouter"), None] {
            let mut flags = vec!["--url", target.base(), "--no-cache", "--model", "m"];
            if let Some(backend) = backend {
                flags.extend(["--backend", backend]);
            }
            let input = home.evidence(&["alpha"]);
            let reference = format!("@{}", file.display());
            let mut args = vec!["decide", &reference, "--input", &input];
            args.extend(flags);
            let out = home.run(&args, &[("OPENROUTER_API_KEY", "fixture0399")]);
            assert_eq!(out.status.code(), Some(0), "{}", said(&out).1);
            let request: Value =
                serde_json::from_slice(&target.requests()[0].body).expect("request");
            let authored = fields
                .as_object()
                .expect("fields")
                .iter()
                .filter(|(_, value)| !value.is_null())
                .map(|(key, value)| (key.clone(), value.clone()))
                .collect::<serde_json::Map<_, _>>();
            let wanted = if backend.is_some() {
                expected.clone()
            } else {
                (!authored.is_empty()).then_some(Value::Object(authored))
            };
            assert_eq!(request["questions"]["q1"].get("criteria"), wanted.as_ref());
            assert!(said(&out).1.is_empty());
        }
    }
}

#[test]
fn encoded_bytes_separate_described_cache_and_share_bare_cache() {
    for described in [true, false] {
        let target = listener();
        let home = Home::new("cache-sides-0399");
        for backend in [true, false, true, false] {
            let mut flags = vec!["--url", target.base(), "--model", "m"];
            if backend {
                flags.extend(["--backend", "openrouter"]);
            }
            if described {
                flags.extend(["--true", "Yes"]);
            }
            let out = decide(&home, &flags, &[("OPENROUTER_API_KEY", "fixture0399")]);
            assert_eq!(out.status.code(), Some(0), "{}", said(&out).1);
        }
        assert_eq!(target.count(), if described { 2 } else { 1 });
    }
}

fn schema_listener() -> Listener {
    Listener::answering(|body| {
        let request: Value = serde_json::from_slice(body).expect("request JSON");
        let questions = request["questions"].as_object().expect("questions");
        if questions.values().any(|q| q["type"] == "noul" && q.get("criteria").is_some_and(|c| c.get("false").is_none_or(Value::is_null))) {
            return Canned::status(400, r#"{"detail":"both sides required"}"#);
        }
        let answers = questions.iter().map(|(name,q)| (name.clone(), schema_answer(q))).collect::<serde_json::Map<_,_>>();
        Canned::ok(&json!({"model":request["model"],"answers":answers,"id":"reply0399","provider":"fixture","usage":{"input_tokens":9,"output_tokens":3,"cost":0.0001}}).to_string())
    }).expect("schema listener")
}

fn schema_answer(q: &Value) -> Value {
    if q["type"] == "noul" {
        return json!({"type":"noul","noul":0.92});
    }
    let keys = match &q["criteria"] {
        Value::Object(c) => c.keys().cloned().collect::<Vec<_>>(),
        Value::Array(c) => (0..c.len()).map(|n| n.to_string()).collect(),
        _ => panic!("criteria"),
    };
    let count = keys.len();
    let probabilities = keys
        .into_iter()
        .enumerate()
        .map(|(i, k)| {
            let share = if i == 0 {
                0.9
            } else {
                0.1 / (count - 1) as f64
            };
            (k, json!(share))
        })
        .collect::<serde_json::Map<_, _>>();
    json!({"type":q["type"],"probabilities":probabilities,"confidence":0.9})
}

fn fill_tag_sides(body: &mut Value) {
    for q in body["questions"]
        .as_object_mut()
        .expect("questions")
        .values_mut()
    {
        if let Some(c) = q.get_mut("criteria") {
            c.as_object_mut()
                .expect("criteria")
                .insert("false".into(), json!({}));
        }
    }
}

#[test]
fn check_satisfies_both_sides_and_plan_pins_exact_requests() {
    let target = schema_listener();
    let home = Home::new("check-sides-0399");
    let out = home.run(
        &["check", "--backend", "openrouter", "--url", target.base()],
        &[("OPENROUTER_API_KEY", "fixture0399")],
    );
    let (stdout, stderr) = said(&out);
    assert_eq!((out.status.code(), stderr.as_str()), (Some(0), ""));
    assert_eq!(
        stdout
            .lines()
            .filter(|line| line.starts_with("ok "))
            .count(),
        8
    );
    assert!(stdout.ends_with("critical 0, warning 0\n"));
    assert_eq!(target.count(), 4);
    let out = home.run(&["check", "--url", target.base()], &[]);
    assert_eq!(out.status.code(), Some(4));
    let stdout = said(&out).0;
    assert!(stdout.contains("critical noul:"));
    assert!(stdout.contains("critical mixed:"));
    assert_eq!(target.count(), 8);
    let out = home.run(
        &[
            "check",
            "--backend",
            "openrouter",
            "--model",
            "jev-1.13.0",
            "--plan",
        ],
        &[],
    );
    assert_eq!(out.status.code(), Some(0), "{}", said(&out).1);
    let bodies: String = said(&out)
        .0
        .lines()
        .filter_map(|line| line.strip_prefix("request "))
        .filter_map(|line| line.split_once(' ').map(|(_, body)| format!("{body}\n")))
        .collect();
    let expected =
        include_str!("../../../../../specification/fixtures/check/requests-both-sides.jsonl");
    assert_eq!(bodies, expected);
    assert_eq!(target.count(), 8);
}

#[test]
fn manual_check_caps_bind_admission_and_all_sixteen_retry_attempts() {
    for name in ["perplexity", "openrouter"] {
        for cap in ["1", "10000"] {
            let target =
                Listener::answering(|_| Canned::status(503, "{}")).expect("retry listener");
            let home = Home::new("cap-0399");
            let out = home.run(
                &["check", "--backend", name, "--url", target.base()],
                &[("THINKTHEN_MAX_ESTIMATED_INPUT_TOKENS_TOTAL", cap)],
            );
            assert_eq!(out.status.code(), Some(if cap == "1" { 2 } else { 4 }));
            assert_eq!(target.count(), if cap == "1" { 0 } else { 16 });
            if cap == "1" {
                assert!(
                    said(&out)
                        .1
                        .contains("would be exceeded before this call's first request")
                );
            }
            home.assert_no_marker_in_files();
        }
    }
}

#[test]
fn tag_fills_each_described_label_and_choice_and_score_keep_authored_bytes() {
    let home = Home::new("shapes-0399");
    let target = schema_listener();
    let cases = [
        (
            "tag",
            r#"{"tag":"Which?","labels":{"s":"Text","o":{"what":"Yes"},"a":["x",1],"e":{},"n":null,"u":null}}"#,
        ),
        (
            "choose",
            r#"{"choose":"Which?","options":{"s":"Text","o":{"what":"Yes"},"a":["x",1],"e":{},"n":null}}"#,
        ),
        (
            "score",
            r#"{"score":"Which?","levels":{"s":"Text","o":{"what":"Yes"},"a":["x",1],"e":{},"n":null}}"#,
        ),
    ];
    for (verb, question) in cases {
        let file = home.root.join("shape.json");
        std::fs::write(&file, question).expect("question file");
        let reference = format!("@{}", file.display());
        let input = home.evidence(&["alpha"]);
        let mut bodies = Vec::new();
        for named in [true, false] {
            let mut args = vec![
                verb,
                &reference,
                "--input",
                &input,
                "--url",
                target.base(),
                "--model",
                "m",
                "--no-cache",
            ];
            if named {
                args.extend(["--backend", "openrouter"]);
            }
            // Plans pin bytes for the unnamed one-sided tag without the mimic's refusal.
            args.push("--plan");
            let out = home.run(&args, &[]);
            assert_eq!(out.status.code(), Some(0), "{}", said(&out).1);
            assert!(said(&out).1.is_empty());
            let plan: Value = serde_json::from_str(said(&out).0.lines().next().expect("plan line"))
                .expect("asking plan JSON");
            bodies.push(plan["request"].clone());
        }
        if verb == "tag" {
            let mut expected = bodies[1].clone();
            fill_tag_sides(&mut expected);
            assert_eq!(bodies[0], expected);
            assert_eq!(
                bodies[0]["questions"]
                    .as_object()
                    .expect("questions")
                    .values()
                    .filter(|q| q.get("criteria").is_some())
                    .count(),
                4
            );
        } else {
            assert_eq!(bodies[0], bodies[1]);
        }
    }
    assert_eq!(target.count(), 0);
}

#[test]
fn new_builtin_names_preserve_the_rate_only_configuration_contract() {
    let home = Home::new("names-0399");
    for name in ["perplexity", "openrouter"] {
        let custom = json!({"schema":"thinkthen.config/1","backends":{name:{"url":"http://127.0.0.1/v1","key_env":"K","model":"m"}}});
        home.config(&custom.to_string());
        let out = home.run(&["check", "--backend", name, "--plan"], &[]);
        assert_eq!(out.status.code(), Some(5));
        assert_eq!(
            said(&out).1,
            "thinkthen: a configuration entry for a built-in backend holds `requests_per_minute` and nothing else\n"
        );
        let rated =
            json!({"schema":"thinkthen.config/1","backends":{name:{"requests_per_minute":60}}});
        home.config(&rated.to_string());
        let out = home.run(&["check", "--backend", name, "--plan"], &[]);
        assert_eq!(out.status.code(), Some(0), "{}", said(&out).1);
        let (url, model) = if name == "perplexity" {
            (
                "https://api.perplexity.ai/v1/decisions",
                "pplx-decider-v1-27b",
            )
        } else {
            (
                "https://openrouter.ai/api/v1/systemone",
                "typesafe/jev-1.13",
            )
        };
        assert!(said(&out).0.contains(&format!("url {url}\n")));
        assert!(said(&out).0.contains(&format!("model sent {model}\n")));
    }
}
