#![cfg(feature = "command")]
#![cfg(test)]
//! Saved canonical requests preserve scalar offsets in answers and detail.
mod support;
use loopback::{KEY, Loopback};
use serde_json::{Value, json};
use std::{io::Write, process::Stdio};
#[test]
fn saved_requests_keep_code_point_offsets_in_answers_and_detail() {
    for recognize in [false, true] {
        let directory = tempfile::tempdir().unwrap();
        let server = Loopback::answering(move |q| {
            if !recognize {return json!({"type":"choice","probabilities":{"u1":0.1,"u2":0.9,"none":0.0}});}
            if q["criteria"].get("BEGIN").is_none() {return json!({"type":"choice","probabilities":{"person":1.0,"none of these":0.0}});}
            let named=q["instructions"].as_str().unwrap().contains("[[Élise]]");
            json!({"type":"choice","probabilities":{"BEGIN":0.0,"INSIDE":0.0,"END":0.0,"SINGLE":if named{1.0}else{0.0},"OUT":if named{0.0}else{1.0}}})
        }).unwrap();
        let question = if recognize {
            json!({"function":"recognize","kinds":["person"]})
        } else {
            json!({"function":"find","question":"Which line?"})
        };
        let path = directory.path().join("request.json");
        std::fs::write(&path, json!({"question":question}).to_string()).unwrap();
        let mut child = support::command()
            .args(["run", "--json"])
            .arg(path)
            .env("THINKTHEN_BASE_URL", server.base())
            .env("THINKTHEN_API_KEY", KEY)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        let text = if recognize {
            "é🦀   Élise"
        } else {
            "é🦀\r\n \n 答🙂 \r\n"
        };
        writeln!(child.stdin.take().unwrap(), "{}", json!(text)).unwrap();
        let output = child.wait_with_output().unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let row: Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(row["position"], 1);
        if recognize {
            assert_eq!(row["answer"].as_array().unwrap().len(), 1);
            assert_eq!(row["answer"][0]["text"], "Élise");
            assert_eq!(row["answer"][0]["start"], 5);
            assert_eq!(row["answer"][0]["end"], 11);
            assert_eq!(row["detail"]["pieces"][1]["end"], 2);
            assert_eq!(server.count(), 2);
        } else {
            assert_eq!(row["answer"], json!({"text":" 答🙂 ","start":6,"end":10}));
            assert_eq!(server.count(), 1);
        }
    }
}
