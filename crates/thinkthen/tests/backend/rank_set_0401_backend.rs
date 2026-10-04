//! Rank sets preserve the landed named backend's address, model and key choice.
use super::support::{CONFIGURED, Home, MARKERS, listener, said};
use serde_json::{Value, json};

#[test]
fn rank_set_0401_uses_selected_backend_and_override_without_recording_credentials() {
    let target = listener();
    let home = Home::new("rank-set-0401-backend");
    home.config(&json!({"schema":"thinkthen.config/1","backends":{"selected":{"url":target.base(),"path":"decisions","key_env":CONFIGURED.0,"model":"chosen"}}}).to_string());
    let question = home.path("rank-set.json");
    std::fs::write(
        &question,
        r#"{"version":1,"questions":{"first":{"decide":"First?"},"second":{"decide":"Second?"}}}"#,
    )
    .expect("question file");
    let question = format!("@{question}");
    let input = home.evidence(&["alpha", "beta"]);
    let record = home.path("recording");
    for (extra, expected) in [
        (vec![], "chosen"),
        (vec!["--model", "override"], "override"),
    ] {
        let args = [
            &[
                "rank",
                &question,
                "--backend",
                "selected",
                "--input",
                &input,
                "--record",
                &record,
                "--no-cache",
                "--details",
            ][..],
            &extra,
        ]
        .concat();
        let out = home.run(&args, &[("THINKTHEN_API_KEY", "")]);
        assert_eq!(out.status.code(), Some(0), "{}", said(&out).1);
        let rows: Vec<Value> = said(&out)
            .0
            .lines()
            .map(|line| serde_json::from_str(line).expect("detail"))
            .collect();
        assert_eq!(rows.len(), 2);
        assert!(
            rows.iter()
                .all(|row| row["meta"]["model"] == expected && row["question_name"] == "first")
        );
        let requests = target.requests();
        assert_eq!(requests.len(), 1);
        assert_eq!(requests[0].line, "POST /v1/decisions HTTP/1.1");
        let body: Value = serde_json::from_slice(&requests[0].body).expect("body");
        assert_eq!(body["model"], expected);
        // Return the drained request's header comparison as a count only.
        assert_eq!(
            requests
                .iter()
                .filter(|request| request.header("authorization")
                    == Some(&format!("Bearer {}", CONFIGURED.1)))
                .count(),
            1
        );
        for (_, marker) in MARKERS {
            assert!(!format!("{}{}", said(&out).0, said(&out).1).contains(marker));
        }
        home.assert_no_marker_in_files();
    }
}
