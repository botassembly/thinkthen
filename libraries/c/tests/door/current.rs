//! Public counted constructors and immutable image ownership under the existing sanitizer.
use super::{compile, crate_dir, run_with, text};
use conformance_backend::Backend;
#[test]
fn immutable_image_bytes_dimensions_and_filename_outlive_the_engine() {
    let backend = Backend::start().expect("loopback");
    let image = crate_dir().join("../../specification/fixtures/images/red.png");
    let jpeg = crate_dir().join("../../specification/fixtures/images/red.jpg");
    let output = run_with(
        &compile(&crate_dir().join("tests/c/carriers.c")),
        &format!("{}/generic/v1", backend.origin()),
        b"",
        &[("TYPED_IMAGE", &image), ("TYPED_JPEG", &jpeg)],
    );
    assert_eq!(
        (output.status.code(), text(&output.stderr)),
        (Some(0), String::new())
    );
    assert_eq!(backend.count(), 0);
}
#[test]
fn all_ten_question_constructors_and_invalid_counted_descriptors_send_nothing() {
    let backend = Backend::start().expect("loopback");
    let path = crate_dir().join("../../specification/fixtures/files/questions.json");
    let output = run_with(
        &compile(&crate_dir().join("tests/c/carrier_inputs.c")),
        &format!("{}/generic/v1", backend.origin()),
        b"",
        &[("TYPED_QUESTION", &path)],
    );
    assert_eq!(
        (output.status.code(), text(&output.stderr)),
        (Some(0), String::new())
    );
    assert_eq!(backend.count(), 0);
}
#[test]
fn all_ten_complete_calls_expose_actual_values_identities_attempts_and_owned_views() {
    for file_mode in [false, true] {
        let backend = Backend::start().expect("loopback");
        let path = crate_dir().join("../../specification/fixtures/files/documents");
        let extra = if file_mode {
            vec![("TYPED_SOURCE", path.as_path())]
        } else {
            Vec::new()
        };
        let output = run_with(
            &compile(&crate_dir().join("tests/c/complete_calls.c")),
            &format!("{}/generic/v1", backend.origin()),
            b"",
            &extra,
        );
        assert_eq!(
            (output.status.code(), text(&output.stderr)),
            (Some(0), String::new())
        );
        assert_eq!(backend.count(), 13);
    }
}
#[test]
fn complete_failure_snapshots_keep_actual_stops_and_final_joined_facts() {
    for (code, arm, sends) in [
        (1, "generic", 0),
        (2, "arm/status/401", 1),
        (3, "generic", 0),
        (5, "generic", 0),
    ] {
        let backend = Backend::start().expect("loopback");
        let mode = std::path::PathBuf::from(code.to_string());
        let output = run_with(
            &compile(&crate_dir().join("tests/c/complete_failures.c")),
            &format!("{}/{arm}/v1", backend.origin()),
            b"",
            &[("TYPED_FAILURE", &mode)],
        );
        assert_eq!(
            (output.status.code(), text(&output.stderr)),
            (Some(0), String::new())
        );
        assert_eq!(backend.count(), sends);
    }
}
#[test]
fn complete_record_controls_record_reread_and_cache_use_one_native_store() {
    let backend = Backend::start().expect("loopback");
    let recording = super::scratch("complete-recording");
    let cache = super::scratch("complete-cache");
    let output = run_with(
        &compile(&crate_dir().join("tests/c/complete_controls.c")),
        &format!("{}/arm/full/capture/v1", backend.origin()),
        b"",
        &[("TYPED_RECORDING", &recording), ("TYPED_CACHE", &cache)],
    );
    assert_eq!(
        (output.status.code(), text(&output.stderr)),
        (Some(0), String::new())
    );
    assert_eq!(backend.count(), 3);
    let capture: serde_json::Value =
        serde_json::from_str(&backend.capture()).expect("captured requests");
    let bodies = capture["bodies"].as_array().expect("bodies");
    assert_eq!(bodies.len(), 3);
    for body in bodies {
        let request: serde_json::Value =
            serde_json::from_str(body.as_str().expect("body text")).expect("request");
        assert_eq!(request["state"], "record-context");
        assert_eq!(request["model"], "fixed");
        assert_eq!(
            request["questions"]["q1"]["instructions"],
            "The text is \"evidence\". Need attention?"
        );
    }
    let choice: serde_json::Value =
        serde_json::from_str(bodies[1].as_str().expect("choose body")).expect("request");
    assert_eq!(
        choice["questions"]["q1"]["criteria"],
        serde_json::json!({"runtime-first":{"z":false,"a":null},"runtime-last":null})
    );
}
#[test]
fn complete_images_keep_order_and_execute_three_routes_and_native_file_reading() {
    let fixtures = crate_dir().join("../../specification/fixtures/images");
    let replies = [
        "liquid-decide-reply.json",
        "liquid-choose-reply.json",
        "liquid-score-reply.json",
        "liquid-decide-reply.json",
    ]
    .iter()
    .map(|name| std::fs::read_to_string(fixtures.join(name)).expect("saved reply"))
    .collect::<Vec<_>>();
    let next = std::sync::atomic::AtomicUsize::new(0);
    let backend = conformance_backend::Listener::answering(move |_| {
        let at = next.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        conformance_backend::Canned::ok(&replies[at])
    })
    .expect("loopback");
    let output = run_with(
        &compile(&crate_dir().join("tests/c/complete_images.c")),
        backend.base(),
        b"",
        &[
            ("LIQUIDAI_API_KEY", std::path::Path::new(super::KEY)),
            ("TYPED_IMAGE", &fixtures.join("red.png")),
            ("TYPED_BLUE", &fixtures.join("blue.png")),
        ],
    );
    assert_eq!(
        (output.status.code(), text(&output.stderr)),
        (Some(0), String::new())
    );
    assert_eq!(backend.count(), 4);
    let requests = backend.requests();
    for request in requests.iter().take(3) {
        let body: serde_json::Value = serde_json::from_slice(&request.body).expect("request");
        assert_eq!(body["images"].as_array().expect("ordered images").len(), 3);
        assert_eq!(body["images"][0], body["images"][2]);
        assert_ne!(body["images"][0], body["images"][1]);
        assert_eq!(body["state"], "Compare originals.");
        assert_eq!(body["model"], "d1");
    }
}
#[test]
fn complete_text_only_image_refusals_send_nothing_for_all_seven_functions() {
    let backend = Backend::start().expect("loopback");
    let fixtures = crate_dir().join("../../specification/fixtures/images");
    let mode = std::path::PathBuf::from("1");
    let output = run_with(
        &compile(&crate_dir().join("tests/c/complete_images.c")),
        &format!("{}/generic/v1", backend.origin()),
        b"",
        &[
            ("TYPED_IMAGE", &fixtures.join("red.png")),
            ("TYPED_BLUE", &fixtures.join("blue.png")),
            ("TYPED_REFUSE", &mode),
        ],
    );
    assert_eq!(
        (output.status.code(), text(&output.stderr)),
        (Some(0), String::new())
    );
    assert_eq!(backend.count(), 0);
}

#[test]
fn complete_native_readings_keep_projection_originals_authored_nulls_and_failed_members() {
    for partial in [false, true] {
        let backend = Backend::start().expect("loopback");
        let folder = super::scratch(if partial {
            "complete-partial"
        } else {
            "complete-selected"
        });
        let selected = folder.join("selected.json");
        std::fs::write(
            &selected,
            r#"{"body":"selected evidence","other":"withheld"}"#,
        )
        .expect("source");
        let annotation = folder.join("annotate.json");
        std::fs::write(
            &annotation,
            r#"{"version":1,"questions":{"ready":{"decide":"Need attention?","on":["/body"]}}}"#,
        )
        .expect("question");
        let mode = std::path::PathBuf::from("1");
        let mut extra = vec![
            ("TYPED_SELECTED_FILE", selected.as_path()),
            ("TYPED_ANNOTATE_FILE", annotation.as_path()),
        ];
        if partial {
            extra.push(("TYPED_PARTIAL", mode.as_path()));
        }
        let arm = if partial {
            "arm/malformed/missing_answer"
        } else {
            "arm/full/capture"
        };
        let output = run_with(
            &compile(&crate_dir().join("tests/c/complete_readings.c")),
            &format!("{}/{arm}/v1", backend.origin()),
            b"",
            &extra,
        );
        assert_eq!(
            (output.status.code(), text(&output.stderr)),
            (Some(0), String::new())
        );
        assert_eq!(backend.count(), if partial { 1 } else { 4 });
        if !partial {
            let capture: serde_json::Value =
                serde_json::from_str(&backend.capture()).expect("captured");
            let bodies = capture["bodies"].as_array().expect("bodies");
            assert_eq!(bodies.len(), 4);
            for at in [1, 2] {
                let body = bodies[at].as_str().expect("request body");
                assert!(body.contains("selected evidence") && !body.contains("withheld"));
            }
        }
    }
}

#[test]
fn complete_author_declarations_and_explicit_object_context_keep_native_types() {
    for (mode, sends) in [("success", 3), ("refused", 0)] {
        let backend = Backend::start().expect("owned loopback");
        let mode = std::path::PathBuf::from(mode);
        let output = run_with(
            &compile(&crate_dir().join("tests/c/complete_authors.c")),
            &format!("{}/arm/full/capture/v1", backend.origin()),
            b"",
            &[("TYPED_AUTHOR_MODE", &mode)],
        );
        assert_eq!(
            (output.status.code(), text(&output.stderr)),
            (Some(0), String::new())
        );
        assert_eq!(backend.count(), sends);
        if sends != 0 {
            let capture: serde_json::Value =
                serde_json::from_str(&backend.capture()).expect("capture");
            let requests = capture["bodies"].as_array().expect("three bodies");
            assert_eq!(requests.len(), 3);
            let bodies = requests
                .iter()
                .map(|b| {
                    serde_json::from_str::<serde_json::Value>(b.as_str().expect("body"))
                        .expect("request")
                })
                .collect::<Vec<_>>();
            assert_eq!(
                bodies[0]["state"],
                serde_json::json!({"flag": false, "unselected": null})
            );
            assert_eq!(bodies[1]["state"], "shared fallback");
            assert_eq!(
                bodies[2]["state"],
                "Each question quotes the text it asks about."
            );
            assert!(
                bodies[0]["questions"]["q1"]["instructions"]
                    .as_str()
                    .expect("instructions")
                    .contains("typed evidence")
            );
        }
    }
}
#[test]
fn complete_native_named_and_reference_loaders_execute_roles_and_hold_path_precedence() {
    let backend = Backend::start().expect("owned loopback");
    let config = super::scratch("named-config");
    let questions = config.join("thinkthen/questions");
    std::fs::create_dir_all(&questions).expect("questions");
    for (name, body) in [
        (
            "catalog",
            r#"{"name":"catalog","wording_version":31,"decide":"Need attention?","model":"jev-latest","batch":2}"#,
        ),
        (
            "mismatch",
            r#"{"name":"different","decide":"Need attention?"}"#,
        ),
        ("broken", r#"{"name":"broken","decide":"Need attention?"}"#),
        (
            "members",
            r#"{"version":1,"questions":{"ready":{"name":"member-check","decide":"Need attention?"}}}"#,
        ),
        (
            "dynamic",
            r#"{"name":"dynamic","wording_version":33,"choose":"Choose one?"}"#,
        ),
        (
            "recognition",
            r#"{"version":1,"name":"recognition","wording_version":33,"recognize":{"kinds":{},"relations":[]}}"#,
        ),
        (
            "relation",
            r#"{"version":1,"name":"relation","wording_version":33,"relate":{"relations":[{"name":"supports","source":"*","target":"*"}]}}"#,
        ),
        (
            "ranking",
            r#"{"name":"ranking","wording_version":34,"decide":"Rank this?"}"#,
        ),
        (
            "rank-members",
            r#"{"version":1,"questions":{"first":{"decide":"Criterion A?"},"second":{"decide":"Criterion B?"}}}"#,
        ),
        (
            "finding",
            r#"{"name":"finding","wording_version":34,"find":"Find this?"}"#,
        ),
    ] {
        std::fs::write(questions.join(format!("{name}.json")), body).expect("named question");
    }
    let cwd = super::scratch("named-working-directory");
    std::fs::write(
        cwd.join("catalog"),
        r#"{"name":"catalog","wording_version":32,"decide":"Need attention?"}"#,
    )
    .expect("local collision");
    std::fs::write(cwd.join("broken"), "not question JSON").expect("invalid local collision");
    let mode = std::path::PathBuf::from("named");
    let output = run_with(
        &compile(&crate_dir().join("tests/c/complete_authors.c")),
        &format!("{}/generic/v1", backend.origin()),
        b"",
        &[
            ("TYPED_AUTHOR_MODE", &mode),
            ("XDG_CONFIG_HOME", &config),
            ("TYPED_WORKING_DIRECTORY", &cwd),
        ],
    );
    assert_eq!(
        (output.status.code(), text(&output.stderr)),
        (Some(0), String::new())
    );
    assert_eq!(backend.count(), 11);
}

#[test]
fn saved_rank_sets_retain_turns_order_and_each_native_member_judgment() {
    let backend = Backend::start().expect("owned backend");
    let output = super::run(
        &compile(&crate_dir().join("tests/c/complete_rank_sets.c")),
        &format!("{}/arm/full/capture/v1", backend.origin()),
        b"",
    );
    assert_eq!(
        (output.status.code(), text(&output.stderr)),
        (Some(0), String::new())
    );
    assert_eq!(backend.count(), 1);
    let capture: serde_json::Value = serde_json::from_str(&backend.capture()).expect("captured");
    let body = capture["bodies"][0].as_str().expect("request");
    assert!(
        body.contains("Criterion A?")
            && body.contains("Criterion B?")
            && body.contains("One")
            && body.contains("Two")
    );
}
