//! Complete menus and byte admission through both public transports and stored answers.
use super::{Canned, Listener, PathBuf, fs};
use serde_json::{Value, json};
use thinkthen::{Engine, EngineBuilder, Kind, Recognize};

fn answer(body: &[u8]) -> Canned {
    let request: Value = serde_json::from_slice(body).unwrap();
    let openai = request["questions"].is_array();
    let questions: Vec<(&str, &Value)> = if openai {
        request["questions"]
            .as_array()
            .unwrap()
            .iter()
            .map(|q| (q["name"].as_str().unwrap(), q))
            .collect()
    } else {
        request["questions"]
            .as_object()
            .unwrap()
            .iter()
            .map(|(n, q)| (n.as_str(), q))
            .collect()
    };
    let mut answers = Vec::new();
    for (name, q) in questions {
        let labels: Vec<&str> = if openai {
            q["choices"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v["value"].as_str().unwrap())
                .collect()
        } else {
            q["criteria"]
                .as_object()
                .unwrap()
                .keys()
                .map(String::as_str)
                .collect()
        };
        let picked = if labels.contains(&"SINGLE") {
            super::tag(q["instructions"].as_str().unwrap())
        } else {
            "kind000"
        };
        let probabilities = labels
            .iter()
            .map(|label| ((*label).to_owned(), Value::from(u8::from(*label == picked))))
            .collect::<Vec<_>>();
        let value = if openai {
            json!({"type":"choice","name":name,"choice":picked,"confidence":1.0,
                "probabilities":probabilities.iter().map(|(v,p)|json!({"value":v,"probability":p})).collect::<Vec<_>>()})
        } else {
            json!({"type":"choice","choice":picked,"probabilities":probabilities.into_iter().collect::<serde_json::Map<_,_>>()})
        };
        answers.push((name.to_owned(), value));
    }
    let answers = if openai {
        Value::Array(answers.into_iter().map(|(_, v)| v).collect())
    } else {
        Value::Object(answers.into_iter().collect())
    };
    Canned::ok(&json!({"model":"custom-fixed","answers":answers,"usage":{"input_tokens":11,"output_tokens":2}}).to_string())
}
fn builder(listener: &Listener, backend: &str) -> EngineBuilder {
    Engine::builder()
        .backend(backend)
        .unwrap()
        .base_url(listener.base())
        .unwrap()
        .model("custom-fixed")
        .unwrap()
        .api_key("fake")
        .unwrap()
        .no_cache()
}
fn menu(count: usize, description: Option<&str>) -> Recognize {
    let mut ask = Recognize::builder();
    for n in 0..count {
        ask = ask
            .kind(
                Kind::new(
                    &format!("kind{n:03}"),
                    description.map(|v| thinkthen::Description::text(v).unwrap()),
                )
                .unwrap(),
            )
            .unwrap();
    }
    ask.build().unwrap()
}
fn largest(requests: &[conformance_backend::Recorded]) -> usize {
    requests.iter().map(|r| r.body.len()).max().unwrap()
}
#[test]
fn complete_255_and_256_kind_menus_survive_both_live_and_stored_routes() {
    for backend in ["typesafe", "openai"] {
        for count in [255, 256] {
            let listener = Listener::answering(answer).unwrap();
            let directory = PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
                .join(format!("complete-menu-{backend}-{count}"));
            let _removed = fs::remove_dir_all(&directory);
            let ask = menu(count, None);
            let engine = builder(&listener, backend)
                .record(&directory)
                .unwrap()
                .build()
                .unwrap();
            let call = engine.recognize(&ask, "Ada").unwrap();
            assert_eq!(call.value().entities()[0].kind(), "kind000");
            let sent = listener.requests();
            assert_eq!(sent.len(), 2);
            let last: Value = serde_json::from_slice(&sent[1].body).unwrap();
            let labels = if backend == "openai" {
                last["questions"][0]["choices"].as_array().unwrap().len()
            } else {
                last["questions"]["q1"]["criteria"]
                    .as_object()
                    .unwrap()
                    .len()
            };
            assert_eq!(labels, count + 1);
            let facts = serde_json::to_value(call.facts()).unwrap();
            assert_eq!(facts["largest_request_bytes"], largest(&sent));
            assert_eq!(
                facts["largest_request_estimated_input_tokens"],
                (largest(&sent) * 908).div_ceil(1000)
            );
            assert_eq!(facts["token_estimate_method"], "encoded-body-bytes-908-v1");
            let replay = builder(&listener, backend)
                .replay(&directory)
                .unwrap()
                .build()
                .unwrap();
            assert_eq!(
                replay
                    .recognize(&ask, "Ada")
                    .unwrap()
                    .facts()
                    .requests_sent(),
                0
            );
            assert_eq!(listener.count(), 2);
        }
    }
}
#[test]
fn encoded_unicode_and_escaping_fit_exactly_and_refuse_one_byte_excess_before_sending() {
    for backend in ["typesafe", "openai"] {
        let listener = Listener::answering(answer).unwrap();
        let ask = menu(1, Some("é \"quoted\" \\\n"));
        let engine = builder(&listener, backend).build().unwrap();
        engine.recognize(&ask, "Ada").unwrap();
        let sent = listener.requests();
        let exact = largest(&sent);
        let engine = builder(&listener, backend)
            .max_request_bytes(exact)
            .unwrap()
            .build()
            .unwrap();
        engine.recognize(&ask, "Ada").unwrap();
        assert_eq!(listener.count(), 4);
        let engine = builder(&listener, backend)
            .max_request_bytes(exact - 1)
            .unwrap()
            .build()
            .unwrap();
        let error = engine.recognize(&ask, "Ada").unwrap_err();
        assert_eq!(listener.count(), 4);
        assert_eq!(error.facts().unwrap().requests_sent(), 0);
        assert!(!error.detail().message().contains("quoted"));
        assert_eq!(
            serde_json::to_value(error.facts().unwrap()).unwrap()["largest_request_bytes"],
            0
        );
    }
}

#[test]
fn derived_span_refusal_keeps_boundary_sends_usage_and_recordings() {
    for backend in ["typesafe", "openai"] {
        let listener = Listener::answering(answer).unwrap();
        let ask = menu(1, None);
        let text = std::iter::repeat_n("Ada", 300)
            .collect::<Vec<_>>()
            .join(" ");
        let profile = r#"{"schema":"thinkthen.backend-profile/1","name":"one","max_questions":1}"#;
        let engine = builder(&listener, backend)
            .profile_json(profile)
            .unwrap()
            .build()
            .unwrap();
        engine.recognize(&ask, &text).unwrap();
        let baseline = listener.requests();
        let boundary_max = baseline
            .iter()
            .filter_map(|r| {
                let q: Value = serde_json::from_slice(&r.body).unwrap();
                let has = if backend == "openai" {
                    q["questions"][0]["choices"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .any(|c| c["value"] == "BEGIN")
                } else {
                    q["questions"]["q1"]["criteria"].get("BEGIN").is_some()
                };
                has.then_some(r.body.len())
            })
            .max()
            .unwrap();
        assert!(
            largest(&baseline) > boundary_max,
            "later body must exceed boundary bodies"
        );
        let directory =
            PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!("derived-span-{backend}"));
        let _removed = fs::remove_dir_all(&directory);
        let engine = builder(&listener, backend)
            .profile_json(profile)
            .unwrap()
            .max_request_bytes(boundary_max)
            .unwrap()
            .record(&directory)
            .unwrap()
            .build()
            .unwrap();
        let prior = listener.count();
        let error = engine.recognize(&ask, &text).unwrap_err();
        let sent = listener.count() - prior;
        assert_eq!(sent, baseline.len() - 1);
        assert!(sent > 0);
        let facts = error.facts().unwrap();
        assert_eq!(facts.requests_sent(), sent as u64);
        assert_eq!(facts.input_tokens(), Some(11 * sent as u64));
        let facts = serde_json::to_value(facts).unwrap();
        assert_eq!(facts["largest_request_bytes"], boundary_max);
        assert!(!error.detail().message().contains("Ada"));
        assert!(fs::read_dir(&directory).unwrap().next().is_some());
    }
}

#[test]
fn contextual_probe_admits_complete_menu_and_plan_maxima_exclude_probe() {
    for backend in ["typesafe", "openai"] {
        let listener = Listener::answering(answer).unwrap();
        let context = "é \"context\" \\\n";
        let ask = menu(256, None);
        let engine = builder(&listener, backend).build().unwrap();
        engine
            .recognize_with(&ask, "Ada", thinkthen::CallOptions::new().context(context))
            .unwrap();
        let sent = listener.requests();
        assert_eq!(sent.len(), 2);
        assert!(
            sent[1].body.len() > sent[0].body.len(),
            "kind probe is larger than prepared boundary execution"
        );
        let directory =
            PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!("menu-context-{backend}"));
        fs::create_dir_all(&directory).unwrap();
        let file = directory.join("question.json");
        let kinds = (0..256)
            .map(|n| (format!("kind{n:03}"), Value::Null))
            .collect::<serde_json::Map<_, _>>();
        fs::write(
            &file,
            json!({"version":1,"recognize":{"kinds":kinds}}).to_string(),
        )
        .unwrap();
        let file = format!("@{}", file.display());
        let context_file = directory.join("context.txt");
        fs::write(&context_file, context).unwrap();
        let context_path = context_file.to_str().unwrap();
        let options = [
            "recognize",
            &file,
            "--backend",
            backend,
            "--model",
            "custom-fixed",
            "--url",
            listener.base(),
            "--context",
            context_path,
            "--no-cache",
        ];
        let output = super::spawn(&[&options[..], &["--plan"]].concat(), &[], b"Ada").unwrap();
        assert_eq!(
            output.status.code(),
            Some(0),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let stdout = String::from_utf8(output.stdout).unwrap();
        let rows = stdout
            .lines()
            .map(|line| serde_json::from_str::<Value>(line).unwrap())
            .collect::<Vec<_>>();
        assert_eq!(rows[1]["largest_request_bytes"], sent[0].body.len());
        assert_eq!(
            rows[1]["largest_request_estimated_input_tokens"],
            (sent[0].body.len() * 908).div_ceil(1000)
        );
        assert_eq!(
            rows[1]["token_estimate_method"],
            "encoded-body-bytes-908-v1"
        );
        assert_eq!(listener.count(), 2);
        let limit = (sent[1].body.len() - 1).to_string();
        for action in [vec!["--plan"], vec!["--facts"]] {
            let output = super::spawn(
                &[&options[..], &["--max-request-bytes", &limit], &action[..]].concat(),
                &[("THINKTHEN_API_KEY", "fake")],
                b"Ada",
            )
            .unwrap();
            assert_eq!(output.status.code(), Some(2));
            assert_eq!(listener.count(), 2);
            assert!(output.stdout.is_empty());
            assert!(!String::from_utf8_lossy(&output.stderr).contains("context\""));
        }
        let engine = builder(&listener, backend)
            .profile_json(
                r#"{"schema":"thinkthen.backend-profile/1","name":"explicit","max_options":256}"#,
            )
            .unwrap()
            .build()
            .unwrap();
        let error = engine.recognize(&ask, "Ada").unwrap_err();
        assert_eq!(error.facts().unwrap().requests_sent(), 0);
        assert_eq!(listener.count(), 2);
    }
}

#[test]
fn stage_context_counts_exact_encoded_bytes_before_sending() {
    for backend in ["typesafe", "openai"] {
        for boundary in [true, false] {
            let listener = Listener::answering(answer).unwrap();
            let context = "é \"stage\" \\\n".repeat(100);
            let ask = if boundary {
                menu(1, None).boundary_context(&context)
            } else {
                menu(1, None).kind_edge_context(&context)
            };
            builder(&listener, backend)
                .build()
                .unwrap()
                .recognize(&ask, "Ada")
                .unwrap();
            let exact = largest(&listener.requests());
            builder(&listener, backend)
                .max_request_bytes(exact)
                .unwrap()
                .build()
                .unwrap()
                .recognize(&ask, "Ada")
                .unwrap();
            let before = listener.count();
            let error = builder(&listener, backend)
                .max_request_bytes(exact - 1)
                .unwrap()
                .build()
                .unwrap()
                .recognize(&ask, "Ada")
                .unwrap_err();
            assert!(error.detail().message().contains("request"), "{error}");
            assert_eq!(listener.count(), before);
        }
    }
}
