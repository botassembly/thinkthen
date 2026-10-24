#![cfg(feature = "command")]
#![cfg(test)]
//! Quality measurement through the script, real command and canned backend.

use loopback::{KEY, Loopback, Reply};
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::path::Path;
use std::process::Command;

fn answer(q: &Value, openai: bool) -> Value {
    let words = q["instructions"].as_str().unwrap();
    let choices: Vec<String> = if openai {
        q["choices"]
            .as_array()
            .map(|items| {
                items
                    .iter()
                    .map(|item| item["value"].as_str().unwrap().to_owned())
                    .collect()
            })
            .unwrap_or_default()
    } else {
        q["criteria"]
            .as_object()
            .map(|items| items.keys().cloned().collect())
            .unwrap_or_default()
    };
    if choices.is_empty() {
        return if openai {
            json!({"type":"predicate","probability":1.0})
        } else {
            json!({"type":"noul","noul":1.0})
        };
    }
    let selected = if choices.iter().any(|x| x == "BEGIN") {
        if words.contains("[[Ada]]") || words.contains("[[Acme]]") {
            "SINGLE"
        } else {
            "OUT"
        }
    } else if choices.iter().any(|x| x == "company") {
        if words.contains("[[Acme]]") {
            "company"
        } else {
            "person"
        }
    } else {
        "u1"
    };
    if openai {
        json!({"type":"choice","probabilities":choices.iter().map(|x| json!({"value":x,"probability":if x==selected {1.} else {0.}})).collect::<Vec<_>>()})
    } else {
        json!({"type":"choice","probabilities":choices.iter().map(|x| (x.clone(), if x==selected {1.} else {0.})).collect::<BTreeMap<_,_>>()})
    }
}

#[test]
fn scores_real_answers_with_misses_extras_and_selected_backend() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let temporary = std::env::temp_dir().join(format!("thinkthen-quality-{}", std::process::id()));
    std::fs::create_dir_all(&temporary).unwrap();
    let binary = temporary.join("command with spaces");
    std::os::unix::fs::symlink(env!("CARGO_BIN_EXE_thinkthen"), &binary).unwrap();
    let cases = [
        json!({"id":"find-right","function":"find","args":["Which?"],"text":"Ada\nAcme","expected":[[0,3]]}),
        json!({"id":"find-wrong","function":"find","args":["Which?"],"text":"Ada\nAcme","expected":[[4,8]]}),
        json!({"id":"find-empty","function":"find","args":["Which?"],"text":"Ada\nAcme","expected":[]}),
        json!({"id":"names","function":"recognize","args":["person","company"],"text":"Ada joined Acme Missing","expected":[["person",0,3],["person",16,23]]}),
        json!({"id":"edges-right","function":"relate","args":["works_for=person:company"],"text":"Ada joined Acme!","expected":[["works_for","Ada","Acme!"]]}),
        json!({"id":"edges-extra","function":"relate","args":["works_for=person:company"],"text":"Ada joined Acme","expected":[]}),
    ];
    std::fs::write(
        temporary.join("cases.jsonl"),
        cases
            .iter()
            .map(Value::to_string)
            .collect::<Vec<_>>()
            .join("\n"),
    )
    .unwrap();
    std::fs::write(
        temporary.join("comparison.json"),
        json!([{"function":"find","backend":"systemone","precision":0.75,"comparison":"canned reference"}]).to_string(),
    ).unwrap();
    for adapter in ["systemone", "openai"] {
        let openai = adapter == "openai";
        let server = Loopback::start(move |request| {
            let body = request.json().unwrap();
            let answers = if openai {
                json!(
                    body["questions"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|q| answer(q, true))
                        .collect::<Vec<_>>()
                )
            } else {
                json!(
                    body["questions"]
                        .as_object()
                        .unwrap()
                        .iter()
                        .map(|(key, q)| (key.clone(), answer(q, false)))
                        .collect::<BTreeMap<_, _>>()
                )
            };
            Reply::json(200, &json!({"answers":answers}))
        })
        .unwrap();
        let mut command = Command::new("python3");
        command.env_clear();
        if let Some(profile) = std::env::var_os("LLVM_PROFILE_FILE") {
            command.env("LLVM_PROFILE_FILE", profile);
        }
        let output = command
            .arg(root.join("scripts/quality"))
            .arg(&binary)
            .arg(adapter)
            .arg("--url")
            .arg(server.base())
            .arg("--data")
            .arg(&temporary)
            .env("THINKTHEN_API_KEY", KEY)
            .env("OPENAI_API_KEY", KEY)
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let stdout = String::from_utf8(output.stdout).unwrap();
        let comparison: Value = serde_json::from_str(stdout.lines().last().unwrap()).unwrap();
        assert_eq!(comparison["precision"], 0.75);
        assert_eq!(comparison["measured_backend"], adapter);
        assert_eq!(comparison["measured"]["tp"], 1);
        assert_eq!(comparison["measured"]["fp"], 2);
        assert_eq!(comparison["measured"]["fn"], 1);
        for expected in [
            format!("find\t{adapter}\t3\t1\t2\t1\t0.333333\t0.500000\t0.400000"),
            format!("recognize\t{adapter}\t1\t1\t1\t1\t0.500000\t0.500000\t0.500000"),
            format!("relate\t{adapter}\t2\t1\t1\t0\t0.500000\t1.000000\t0.666667"),
        ] {
            assert!(stdout.lines().any(|line| line == expected), "{stdout}");
        }
        assert!(!stdout.contains(KEY));
        assert!(!server.requests().is_empty());
        assert!(server.requests().iter().all(|request| request.path
            == if openai {
                "/v1/decisions"
            } else {
                "/v1/systemone"
            }));
    }
    std::fs::remove_dir_all(temporary).unwrap();
}
