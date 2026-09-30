//! The command: each backend sends only its own key, refusals send nothing,
//! and no marker reaches any output or file.

use crate::support::{Home, LIQUID_BASE, Proxy, TYPESAFE_BASE, by_marker, listener, only, said};

const LOCAL: &str = r#""local-d1":{"url":"URL","key_env":"LOCAL_D1_KEY","model":"jev-1.13.0"}"#;

fn config_with(backend: Option<&str>, local_url: &str) -> String {
    let backend = backend.map_or_else(String::new, |name| format!(r#","backend":"{name}""#));
    format!(
        r#"{{"schema":"thinkthen.config/1"{backend},"backends":{{{}}}}}"#,
        LOCAL.replace("URL", local_url)
    )
}

/// Decide over two lines, one request each, and return the output.
fn decide(home: &Home, flags: &[&str], changes: &[(&str, &str)]) -> std::process::Output {
    let input = home.evidence(&["alpha", "beta"]);
    let mut arguments = vec!["decide", "a refund?", "--lines", "--input", &input];
    arguments.extend_from_slice(flags);
    home.run(&arguments, &[&[("THINKTHEN_BATCH", "1")], changes].concat())
}

#[test]
fn each_backend_sends_only_its_own_key_and_the_unnamed_path_only_the_primary_key() {
    const AT: &str = "LISTENER";
    // (flags, changes, configuration's backend, marker place). `LISTENER`
    // stands for the listener's base; `local-d1` names it as its base.
    type Case = (
        &'static [&'static str],
        &'static [(&'static str, &'static str)],
        Option<&'static str>,
        usize,
    );
    let cases: [Case; 8] = [
        (&["--backend", "liquid", "--url", AT], &[], None, 1),
        (
            &["--backend", "liquid", "--url", AT],
            &[("LIQUIDAI_API_KEY", " ")],
            None,
            2,
        ),
        (
            &["--backend", "liquid", "--url", AT],
            &[("LIQUIDAI_API_KEY", "")],
            None,
            2,
        ),
        (&["--url", AT], &[], None, 3),
        (&[], &[("THINKTHEN_BASE_URL", AT)], Some("liquid"), 3),
        (&["--backend", "local-d1"], &[], None, 4),
        (&[], &[], Some("local-d1"), 4),
        (&["--backend", "typesafe", "--url", AT], &[], None, 0),
    ];
    for (index, (flags, changes, backend, place)) in cases.into_iter().enumerate() {
        let target = listener();
        let at = |value: &'static str| if value == AT { target.base() } else { value };
        let home = Home::new("own-key");
        home.config(&config_with(backend, target.base()));
        let mut flags: Vec<&str> = flags.iter().map(|&flag| at(flag)).collect();
        flags.push("--no-cache");
        let changes: Vec<(&str, &str)> = changes
            .iter()
            .map(|&(name, value)| (name, at(value)))
            .collect();
        let output = decide(&home, &flags, &changes);
        let (stdout, stderr) = said(&output);
        assert_eq!(output.status.code(), Some(0), "{index}: {stderr}");
        assert_eq!(stdout.lines().count(), 2, "{index}");
        assert_eq!(by_marker(&target), only(place, 2), "{index}");
        home.assert_no_marker_in_files();
    }
}

#[test]
fn two_backends_at_one_address_and_model_share_answers_and_never_keys() {
    let target = listener();
    let home = Home::new("shared-answers");
    home.config(&config_with(None, target.base()));
    let first = decide(
        &home,
        &["--backend", "typesafe", "--url", target.base()],
        &[],
    );
    let (answers, stderr) = said(&first);
    assert_eq!(first.status.code(), Some(0), "{stderr}");
    assert_eq!(by_marker(&target), only(0, 2));
    let second = decide(&home, &["--backend", "local-d1"], &[]);
    let (again, stderr) = said(&second);
    assert_eq!(second.status.code(), Some(0), "{stderr}");
    assert_eq!(again, answers);
    assert_eq!(
        by_marker(&target),
        [0; 7],
        "the second backend sent nothing"
    );
    assert_eq!(target.count(), 2);
    // A spend guard reads these two paths; `thinkthen.status/2` keeps them where status/1 had them.
    let status = home.run(&["status", "--json"], &[]);
    let (stdout, _) = said(&status);
    let value: serde_json::Value = serde_json::from_str(&stdout).expect("one JSON object");
    assert_eq!(value["schema"], "thinkthen.status/2");
    assert_eq!(
        value.pointer("/usage/total/requests_sent"),
        Some(&serde_json::json!(2))
    );
    assert_eq!(
        value.pointer("/usage/total/input_tokens"),
        Some(&serde_json::json!(18))
    );
    home.assert_no_marker_in_files();
}

#[test]
fn every_refusal_prints_its_exact_sentence_and_sends_nothing() {
    let proxy = Proxy::start();
    let stolen = format!(
        r#"{{"schema":"thinkthen.config/1","backends":{{"stolen":{{"url":"{LIQUID_BASE}","key_env":"TYPESAFE_API_KEY","model":"m"}}}}}}"#
    );
    let refusal = |name: &str, variable: &str, owner: &str, other: &str| {
        format!(
            "thinkthen: backend `{name}` reads `{variable}`, the key of backend `{owner}`, which never goes to the address of backend `{other}`\n"
        )
    };
    let invalid = "thinkthen: a backend name uses 1 to 32 lowercase letters, digits, and hyphens\n"
        .to_owned();
    let unknown = "thinkthen: unknown backend `nowhere`; the built-in backends are `liquid` and `typesafe`, and the configuration file may name more\n".to_owned();
    let marker = "Sk_config_marker_0334";
    let entry = |body: &str| {
        format!(r#"{{"schema":"thinkthen.config/1","backends":{{"local-d1":{body}}}}}"#)
    };
    // (flags, changes, configuration, standard error, exit code). A
    // configuration refusal exits 5, as every configuration refusal does.
    type Case<'a> = (Vec<&'a str>, Vec<(&'a str, &'a str)>, String, String, i32);
    let cases: Vec<Case<'_>> = vec![
        (vec!["--backend", "typesafe", "--url", LIQUID_BASE], vec![], String::new(), refusal("typesafe", "TYPESAFE_API_KEY", "typesafe", "liquid"), 2),
        (vec!["--backend", "liquid", "--url", TYPESAFE_BASE], vec![], String::new(), refusal("liquid", "LIQUIDAI_API_KEY", "liquid", "typesafe"), 2),
        (vec!["--backend", "stolen"], vec![], stolen.clone(), refusal("stolen", "TYPESAFE_API_KEY", "typesafe", "liquid"), 2),
        (vec![], vec![], stolen.replace(r#""backends""#, r#""backend":"stolen","backends""#), refusal("stolen", "TYPESAFE_API_KEY", "typesafe", "liquid"), 2),
        (vec!["--backend", "Liquid"], vec![], String::new(), invalid.clone(), 2),
        (vec!["--backend", "sk_marker_as_a_name"], vec![], String::new(), invalid.clone(), 2),
        (vec![], vec![("THINKTHEN_BACKEND", "Not A Name")], String::new(), invalid, 2),
        (vec!["--backend", "nowhere"], vec![], String::new(), unknown.clone(), 2),
        (vec![], vec![("THINKTHEN_BACKEND", "nowhere")], String::new(), unknown, 2),
        (vec!["--backend", "liquid"], vec![("LIQUIDAI_API_KEY", ""), ("LIQUID_API_KEY", " ")], String::new(), "thinkthen: the environment variable `LIQUIDAI_API_KEY` is unset or blank, so no key is sent\nthinkthen: stopped at record 1; 0 records finished\n".to_owned(), 4),
        (vec![], vec![], entry(r#"{"url":"http://127.0.0.1/v1","key_env":"K","model":"m"}"#).replace("local-d1", "liquid"), "thinkthen: configuration backend names a built-in backend; choose another name\n".to_owned(), 5),
        (vec![], vec![], entry(&format!(r#"{{"url":"http://127.0.0.1/v1","key_env":"{marker}","model":"m"}}"#)), "thinkthen: configuration backend field `key_env` names an environment variable: a capital letter or underscore, then capital letters, digits, and underscores\n".to_owned(), 5),
        (vec![], vec![], entry(r#"{"url":"http://127.0.0.1/v1","model":"m"}"#), "thinkthen: configuration backend entries need `url`, `key_env`, and `model`\n".to_owned(), 5),
        (vec![], vec![], entry(&format!(r#"{{"url":"http://127.0.0.1/v1","key_env":"K","model":"m","key":"{marker}"}}"#)), "thinkthen: configuration backend entries hold only `url`, `key_env`, and `model`\n".to_owned(), 5),
        (vec![], vec![], format!(r#"{{"schema":"thinkthen.config/1","api_key":"{marker}"}}"#), "thinkthen: the configuration file holds a field other than `schema`, `url`, `model`, `cache`, `cache_bytes`, `usd_per_million_input`, `usd_per_million_output`, `backend`, and `backends`\n".to_owned(), 5),
    ];
    for (index, (flags, changes, config, sentence, code)) in cases.into_iter().enumerate() {
        let home = Home::new("refused");
        home.config(&config);
        let changes = [&[("HTTPS_PROXY", proxy.url.as_str())], changes.as_slice()].concat();
        let output = decide(&home, &flags, &changes);
        let (stdout, stderr) = said(&output);
        assert!(!stderr.contains(marker), "{index}");
        assert_eq!(
            (stdout.as_str(), stderr.as_str()),
            ("", sentence.as_str()),
            "{index}"
        );
        assert_eq!(output.status.code(), Some(code), "{index}");
        home.assert_no_marker_in_files();
    }
    assert_eq!(proxy.count(), 0, "no refusal opened a connection");

    // The control: the same proxy sees a run that is allowed to connect.
    let home = Home::new("control");
    let output = decide(
        &home,
        &["--backend", "liquid", "--no-cache"],
        &[("HTTPS_PROXY", &proxy.url)],
    );
    let _said = said(&output);
    assert!(
        proxy.count() >= 1,
        "the proxy counts a permitted connection"
    );
    home.assert_no_marker_in_files();
}

#[test]
fn a_plan_a_recording_a_replay_and_check_name_the_backend_variable_and_hold_no_key() {
    let proxy = Proxy::start();
    let target = listener();
    let home = Home::new("plan-replay");
    let recording = home.path("recording");
    let proxied = [("HTTPS_PROXY", proxy.url.as_str())];
    let plan = decide(&home, &["--backend", "liquid", "--plan"], &proxied);
    let (stdout, stderr) = said(&plan);
    assert_eq!(plan.status.code(), Some(0), "{stderr}");
    assert!(stdout.starts_with(r#"{"url":"https://api.liquid.ai/decisions/v1/systemone","model":"d1:free","key_env":"LIQUIDAI_API_KEY","#), "{stdout}");
    let recorded = decide(
        &home,
        &[
            "--backend",
            "liquid",
            "--url",
            target.base(),
            "--record",
            &recording,
        ],
        &[],
    );
    let (answers, stderr) = said(&recorded);
    assert_eq!(recorded.status.code(), Some(0), "{stderr}");
    assert_eq!(by_marker(&target), only(1, 2));
    let replayed = decide(
        &home,
        &[
            "--backend",
            "liquid",
            "--url",
            target.base(),
            "--replay",
            &recording,
        ],
        &[("LIQUIDAI_API_KEY", ""), ("LIQUID_API_KEY", "")],
    );
    let (again, stderr) = said(&replayed);
    assert_eq!(replayed.status.code(), Some(0), "{stderr}");
    assert_eq!(again, answers);
    assert_eq!(
        by_marker(&target),
        [0; 7],
        "a replay reads no key and sends nothing"
    );
    assert_eq!(target.count(), 2);
    let check = home.run(&["check", "--backend", "liquid", "--plan"], &proxied);
    let (stdout, stderr) = said(&check);
    assert_eq!(check.status.code(), Some(0), "{stderr}");
    assert!(
        stdout.starts_with("url https://api.liquid.ai/decisions/v1/systemone\nprovider "),
        "{stdout}"
    );
    assert!(
        stdout.contains("\nmodel asked unspecified\nmodel sent d1:free\n"),
        "{stdout}"
    );
    assert_eq!(proxy.count(), 0, "a plan sends nothing");
    home.assert_no_marker_in_files();
}

#[test]
fn status_names_the_backend_its_key_variable_and_each_source() {
    let home = Home::new("status");
    let lines = |output: &std::process::Output| {
        let (stdout, stderr) = said(output);
        assert_eq!(output.status.code(), Some(0), "{stderr}");
        stdout
            .lines()
            .filter(|line| {
                ["backend ", "url", "model", "key_variable ", "api_key_set "]
                    .iter()
                    .any(|start| line.starts_with(start))
            })
            .collect::<Vec<_>>()
            .join("\n")
    };
    let liquid = "backend liquid\nurl https://api.liquid.ai/decisions/v1/systemone\nurl_source backend\nmodel d1:free\nmodel_source backend\nkey_variable LIQUIDAI_API_KEY\napi_key_set true";
    assert_eq!(
        lines(&home.run(&["status", "--backend", "liquid"], &[])),
        liquid
    );
    assert_eq!(
        lines(&home.run(&["status"], &[("THINKTHEN_BACKEND", "liquid")])),
        liquid
    );
    assert_eq!(
        lines(&home.run(
            &["status", "--backend", "liquid"],
            &[("LIQUIDAI_API_KEY", ""), ("LIQUID_API_KEY", "")]
        )),
        liquid.replace("api_key_set true", "api_key_set false")
    );
    home.config(&config_with(Some("local-d1"), "http://127.0.0.1:9/v1"));
    assert_eq!(
        lines(&home.run(
            &["status"],
            &[("THINKTHEN_BASE_URL", "http://localhost:7/v1")]
        )),
        "backend none\nurl http://localhost:7/v1/systemone\nurl_source environment\nmodel jev-1.13.0\nmodel_source built_in\nkey_variable THINKTHEN_API_KEY\napi_key_set true"
    );
    let json = home.run(&["status", "--json"], &[]);
    let (stdout, _) = said(&json);
    let value: serde_json::Value = serde_json::from_str(&stdout).expect("one JSON object");
    assert_eq!(value["schema"], "thinkthen.status/2");
    assert_eq!(
        value["backend"],
        serde_json::json!({"name": "local-d1", "url": "http://127.0.0.1:9/v1/systemone", "url_source": "backend", "model": "jev-1.13.0", "model_source": "backend", "key_variable": "LOCAL_D1_KEY", "api_key_set": true})
    );
    home.config(
        &config_with(Some("liquid"), "http://127.0.0.1:9/v1")
            .replace(r#""backend""#, r#""url":"http://127.0.0.1:8/v1","backend""#),
    );
    let json = home.run(&["status", "--json"], &[]);
    let (stdout, _) = said(&json);
    let value: serde_json::Value = serde_json::from_str(&stdout).expect("one JSON object");
    assert_eq!(value["backend"]["name"], "liquid");
    assert_eq!(value["backend"]["url_source"], "configuration");
    assert_eq!(value["backend"]["url"], "http://127.0.0.1:8/v1/systemone");
    home.assert_no_marker_in_files();
}
