//! A single-answer rule asks one menu per source with `none` last (ticket
//! 0342): the exact question, the edge each top label makes, the detail
//! entry, rule order beside pair rules, the cache, and the refusals.

use std::fs;
use std::path::PathBuf;

use serde_json::{Value, json};

use super::{answered, plan_json, run, scripted};
use crate::harness::{Canned, Listener, spawn};

const PEOPLE: &[u8] = br#"[{"name":"Ada","kind":"person"},{"name":"Acme","kind":"organization"},{"name":"Initech","kind":"organization"},{"name":"Grace","kind":"person"}]"#;

/// Write one relate file and return its `@FILE` operand.
fn file(name: &str, relations: &str) -> String {
    let folder = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("relate-menu");
    fs::create_dir_all(&folder).expect("folder");
    let path = folder.join(format!("{name}.json"));
    let text = format!(r#"{{"version":1,"relate":{{"relations":[{relations}]}}}}"#);
    fs::write(&path, text).expect("relate file");
    format!("@{}", path.display())
}

const WORKS_FOR: &str = r#"{"name":"works_for","source":"person","target":"organization","reads":"works for","single":true}"#;

fn choice(i2: f64, i3: f64, none: f64) -> String {
    json!({"type":"choice","probabilities":{"i2":i2,"i3":i3,"none":none}}).to_string()
}

#[test]
fn a_single_rule_asks_one_described_menu_per_source_and_the_plan_shows_it() {
    let listener = Listener::answering(answered).expect("listener");
    let rule = file("works-for", WORKS_FOR);
    let plan = plan_json(&run(&listener, &[&rule, "--plan"], PEOPLE));
    assert_eq!(listener.connections(), 0);
    assert_eq!(plan["relations"][0]["method"], "choice");
    assert_eq!(plan["relations"][0]["logical_questions"], 2);
    let body: Value =
        serde_json::from_str(plan["requests"][0]["body_utf8"].as_str().expect("body"))
            .expect("body");
    let options = json!({"i2":"Item 2 (organization \"Acme\")","i3":"Item 3 (organization \"Initech\")","none":"No listed organization."});
    assert_eq!(
        body["questions"],
        json!({
            "q1": {"type":"choice","instructions":"Which listed organization fills the blank: Item 1 (person \"Ada\") works for ___? Choose none if no listed organization does.","criteria":options},
            "q2": {"type":"choice","instructions":"Which listed organization fills the blank: Item 4 (person \"Grace\") works for ___? Choose none if no listed organization does.","criteria":options},
        })
    );
    // A same-kind wildcard menu leaves its own source out, and a lone name
    // has no candidate, so it asks nothing and sends nothing.
    let rule = file(
        "mentor",
        r#"{"name":"mentors","source":"*","target":"*","single":true}"#,
    );
    let plan = plan_json(&run(
        &listener,
        &[&rule, "--plan"],
        br#"[{"name":"Ada","kind":"person"},{"name":"Grace","kind":"person"}]"#,
    ));
    let body: Value =
        serde_json::from_str(plan["requests"][0]["body_utf8"].as_str().expect("body"))
            .expect("body");
    assert_eq!(
        body["questions"]["q2"],
        json!({"type":"choice","instructions":"Which listed entity fills the blank: Item 2 (person \"Grace\") mentors ___? Choose none if no listed entity does.","criteria":{"i1":"Item 1 (person \"Ada\")","none":"No listed entity."}})
    );
    let alone = run(&listener, &[&rule], br#"[{"name":"Ada","kind":"person"}]"#);
    assert_eq!(alone.status.code(), Some(0));
    assert!(alone.stdout.is_empty());
    assert_eq!(listener.connections(), 0);
}

/// Each row: the menu's probabilities for i2, i3 and none, and the entry's
/// target, probability and accepted marker; an accepted entry is the edge.
#[test]
fn the_top_label_makes_the_one_edge_and_the_detail_entry() {
    let acme = json!({"name":"Acme","kind":"organization"});
    let initech = json!({"name":"Initech","kind":"organization"});
    let rows = [
        (
            "a target on top at the cut",
            (0.5, 0.3, 0.2),
            Some(&acme),
            0.5,
            true,
        ),
        ("none on top", (0.3, 0.1, 0.6), None, 0.6, false),
        (
            "a target on top below the cut",
            (0.4, 0.3, 0.3),
            Some(&acme),
            0.4,
            false,
        ),
        (
            "an exact tie of two targets",
            (0.5, 0.5, 0.0),
            None,
            0.5,
            false,
        ),
        (
            "an exact tie of a target and none",
            (0.1, 0.45, 0.45),
            None,
            0.45,
            false,
        ),
        (
            "the second target on top",
            (0.2, 0.7, 0.1),
            Some(&initech),
            0.7,
            true,
        ),
    ];
    let rule = file("works-for", WORKS_FOR);
    let input = br#"[{"name":"Ada","kind":"person"},{"name":"Acme","kind":"organization"},{"name":"Initech","kind":"organization"}]"#;
    for (row, (i2, i3, none), target, probability, accepted) in rows {
        let answer = choice(i2, i3, none);
        let listener = Listener::answering(move |_| {
            Canned::ok(&format!(
                r#"{{"model":"local-1","answers":{{"q1":{answer}}}}}"#
            ))
        })
        .expect("listener");
        let details = run(&listener, &[&rule, "--details"], input);
        assert_eq!(details.status.code(), Some(0), "{row}");
        let result: Value = serde_json::from_slice(&details.stdout).expect("details");
        assert_eq!(result["question"]["relations"][0]["single"], true, "{row}");
        let mut entry = result["answer"]["questions"][0].clone();
        entry.as_object_mut().expect("entry").remove("request");
        assert_eq!(
            entry,
            json!({"relation":"works_for","reads":"works for","method":"choice","direction":"source_to_target","source":{"name":"Ada","kind":"person"},"target":target,"probability":probability,"accepted":accepted}),
            "{row}"
        );
        let bare = run(&listener, &[&rule], input);
        let edges: Vec<Value> = String::from_utf8_lossy(&bare.stdout)
            .lines()
            .map(|line| serde_json::from_str(line).expect("edge"))
            .collect();
        let expected: Vec<Value> = if accepted {
            vec![
                json!({"relation":"works_for","source":{"name":"Ada","kind":"person"},"target":target,"probability":probability}),
            ]
        } else {
            Vec::new()
        };
        assert_eq!(edges, expected, "{row}");
    }
}

#[test]
fn a_failed_menu_keeps_its_source_and_the_run_exits_six() {
    let listener = scripted(&[
        r#"{"type":"choice","probabilities":{"i2":0.9,"i3":0.05,"none":0.05}}"#,
        r#"{"type":"noul","noul":0.9}"#,
    ]);
    let rule = file("works-for", WORKS_FOR);
    let output = run(&listener, &[&rule, "--details"], PEOPLE);
    assert_eq!(output.status.code(), Some(6));
    let result: Value = serde_json::from_slice(&output.stdout).expect("details");
    assert_eq!(result["meta"]["failed_questions"], 1);
    let failed = &result["answer"]["questions"][1];
    assert_eq!(failed["source"], json!({"name":"Grace","kind":"person"}));
    assert_eq!(failed["target"], Value::Null);
    assert!(failed.get("probability").is_none() && failed.get("accepted").is_none());
    assert!(failed.get("failure").is_some());
    assert_eq!(result["value"].as_array().expect("edges").len(), 1);
}

#[test]
fn pair_rules_and_menus_keep_rule_order_in_one_request() {
    let listener = scripted(&[
        r#"{"type":"noul","noul":0.9}"#,
        r#"{"type":"noul","noul":0.1}"#,
        r#"{"type":"choice","probabilities":{"i2":0.1,"i3":0.8,"none":0.1}}"#,
        r#"{"type":"choice","probabilities":{"i2":0.1,"i3":0.1,"none":0.8}}"#,
    ]);
    let rule = file(
        "mixed",
        &format!(r#"{{"name":"knows","source":"person","target":"person"}},{WORKS_FOR}"#),
    );
    let output = run(&listener, &[&rule], PEOPLE);
    assert_eq!(output.status.code(), Some(0));
    let [request] = listener.requests().try_into().expect("one request");
    let body: Value = serde_json::from_slice(&request.body).expect("body");
    let kinds: Vec<&str> = body["questions"]
        .as_object()
        .expect("questions")
        .values()
        .map(|question| question["type"].as_str().expect("type"))
        .collect();
    assert_eq!(kinds, ["noul", "noul", "choice", "choice"]);
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        concat!(
            r#"{"relation":"knows","source":{"name":"Ada","kind":"person"},"target":{"name":"Grace","kind":"person"},"probability":0.9}"#,
            "\n",
            r#"{"relation":"works_for","source":{"name":"Ada","kind":"person"},"target":{"name":"Initech","kind":"organization"},"probability":0.8}"#,
            "\n"
        )
    );
}

#[test]
fn an_unchanged_rerun_answers_every_menu_from_the_cache() {
    let root = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("relate-menu-cache");
    let _removed = fs::remove_dir_all(&root);
    let cache = root.join("cache").to_string_lossy().into_owned();
    let xdg = root.join("xdg").to_string_lossy().into_owned();
    let listener =
        scripted(&[r#"{"type":"choice","probabilities":{"i2":0.7,"i3":0.2,"none":0.1}}"#]);
    let rule = file("works-for", WORKS_FOR);
    let arguments = [
        "relate",
        &rule,
        "--url",
        listener.base(),
        "--model",
        "local-1",
        "--cache",
        &cache,
    ];
    let environment = [
        ("THINKTHEN_API_KEY", "secret"),
        ("XDG_CACHE_HOME", xdg.as_str()),
    ];
    let first = spawn(&arguments, &environment, PEOPLE).expect("first run");
    assert_eq!(first.status.code(), Some(0));
    assert_eq!(listener.requests().len(), 1);
    let second = spawn(&arguments, &environment, PEOPLE).expect("second run");
    assert_eq!(second.status.code(), Some(0));
    assert!(listener.requests().is_empty());
    assert_eq!(first.stdout, second.stdout);
    assert_eq!(String::from_utf8_lossy(&first.stdout).lines().count(), 2);
}

#[test]
fn single_with_either_a_single_recognize_rule_and_an_option_limit_send_nothing() {
    let listener = Listener::answering(answered).expect("listener");
    let both = file(
        "both",
        r#"{"name":"married_to","source":"person","target":"person","single":true,"either":true}"#,
    );
    let refused = run(&listener, &[&both], PEOPLE);
    assert_eq!(refused.status.code(), Some(5));
    assert_eq!(
        String::from_utf8_lossy(&refused.stderr),
        "thinkthen: a single-answer relation is directed, so `single` and `either` do not mix\n"
    );
    let folder = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("relate-menu");
    let recognize = folder.join("recognize.json");
    fs::write(&recognize, r#"{"version":1,"recognize":{"kinds":["person","organization"],"relations":[{"name":"works_for","source":"person","target":"organization","single":true}]}}"#).expect("recognize file");
    let refused = spawn(
        &[
            "recognize",
            &format!("@{}", recognize.display()),
            "--url",
            listener.base(),
            "--no-cache",
        ],
        &[("THINKTHEN_API_KEY", "secret-value")],
        b"Ada works for Acme.",
    )
    .expect("recognize");
    assert_eq!(
        refused.status.code(),
        Some(5),
        "{}",
        String::from_utf8_lossy(&refused.stderr)
    );
    let profile = folder.join("two-options.json");
    fs::write(
        &profile,
        r#"{"schema":"thinkthen.backend-profile/1","name":"two-options","max_options":2}"#,
    )
    .expect("profile");
    let rule = file("works-for", WORKS_FOR);
    let refused = run(
        &listener,
        &[&rule, "--profile", profile.to_str().expect("path")],
        PEOPLE,
    );
    assert_eq!(refused.status.code(), Some(2));
    assert_eq!(
        String::from_utf8_lossy(&refused.stderr),
        "thinkthen: profile two-options allows at most 2 options; this request has 3\n"
    );
    assert_eq!(listener.connections(), 0);
}

#[test]
fn single_false_names_the_same_question_as_no_single() {
    let listener = Listener::answering(answered).expect("listener");
    let input = br#"[{"name":"Ada","kind":"person"},{"name":"Acme","kind":"organization"}]"#;
    let digests: Vec<Value> = [
        r#"{"name":"works_for","source":"person","target":"organization"}"#,
        r#"{"name":"works_for","source":"person","target":"organization","single":false}"#,
    ]
    .iter()
    .enumerate()
    .map(|(place, relation)| {
        let rule = file(&format!("plain-{place}"), relation);
        let output = run(&listener, &[&rule, "--details"], input);
        assert_eq!(output.status.code(), Some(0));
        let result: Value = serde_json::from_slice(&output.stdout).expect("details");
        result["meta"]["question_sha256"].clone()
    })
    .collect();
    // `a_saved_relate_name_reaches_details_identity_and_warning` pins a
    // file without `single` at its old digest.
    assert_eq!(digests[0], digests[1]);
}
