//! Operator declarations through the actual SDK/CLI and counted loopback transport.
use super::{BLUE, JPEG, RED, fixture, folder, input, question, url};
use conformance_backend::{Canned, Listener};
use serde_json::{Value, json};
use std::num::NonZeroUsize;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};
use thinkthen::{
    Answer, BatchSetting, CallOptions, CancelToken, Engine, ErrorKind, ImageMedia, Question,
};

#[path = "../../src/test_deadline/child.rs"]
mod child;
use child::ChildEnvironment as _;

const CLEF: &str =
    include_str!("../../../../specification/fixtures/images/local/clef-profile.json");
const FLASH: &str =
    include_str!("../../../../specification/fixtures/images/local/clef-flash-profile.json");
const IMAJEV: &str =
    include_str!("../../../../specification/fixtures/images/local/imajev-profile.json");
const REPLY: &str = r#"{"model":"local","answers":{"q1":{"type":"noul","noul":0.9}},"usage":{"input_tokens":123,"output_tokens":0}}"#;

fn build(
    listener: &Listener,
    backend: &str,
    model: &str,
    profile: &str,
) -> thinkthen::EngineBuilder {
    Engine::builder()
        .backend(backend)
        .unwrap()
        .base_url(listener.base())
        .unwrap()
        .model(model)
        .unwrap()
        .profile_json(profile)
        .unwrap()
        .api_key("PRIVATE_LOCAL_KEY")
        .unwrap()
        .no_cache()
        .max_retries(0)
}
fn pair() -> thinkthen::QuestionInput {
    input(
        Some("Compare originals."),
        vec![
            fixture(RED, ImageMedia::Png),
            fixture(BLUE, ImageMedia::Png),
        ],
    )
}

#[test]
fn admitted_profiles_send_original_jpeg_png_order_and_duplicates_through_scalar_doors() {
    for (profile, model) in [
        (CLEF, "clef-local-0036"),
        (FLASH, "flash-local-0036"),
        (IMAJEV, "imajev-2b"),
    ] {
        let listener = Listener::answering(|_| Canned::ok(REPLY)).unwrap();
        let engine = build(&listener, "llamacpp", model, profile)
            .build()
            .unwrap();
        let images = input(
            Some("Compare originals."),
            vec![
                fixture(JPEG, ImageMedia::Jpeg),
                fixture(RED, ImageMedia::Png),
            ],
        );
        let call = engine.decide_input(&question(), &images).unwrap();
        assert_eq!(call.value(), &Answer::Yes);
        assert_eq!(call.facts().input_tokens(), Some(123));
        let duplicate = input(None, vec![fixture(RED, ImageMedia::Png); 2]);
        engine.decide_input(&question(), &duplicate).unwrap();
        let bodies = listener.requests();
        assert_eq!(bodies.len(), 2);
        assert_eq!(bodies[0].line, "POST /v1/systemone HTTP/1.1");
        assert_eq!(
            serde_json::from_slice::<Value>(&bodies[0].body).unwrap(),
            json!({
                "state":"Compare originals.","model":model,"questions":{"q1":{"type":"noul","instructions":"Is red visible?"}},
                "images":[url(JPEG,"image/jpeg"),url(RED,"image/png")]
            })
        );
        assert_eq!(
            serde_json::from_slice::<Value>(&bodies[1].body).unwrap()["images"],
            json!([url(RED, "image/png"), url(RED, "image/png")])
        );
        let chosen=Listener::answering(|_|Canned::ok(r#"{"model":"local","answers":{"q1":{"type":"choice","probabilities":{"red":0.8,"blue":0.2}}}}"#)).unwrap();
        let engine = build(&chosen, "llamacpp", model, profile).build().unwrap();
        let choose = Question::choose::<super::Color>("Which color?")
            .unwrap()
            .option(super::Color::Red, None)
            .unwrap()
            .option(super::Color::Blue, None)
            .unwrap()
            .build()
            .unwrap();
        assert_eq!(
            engine.choose_input(&choose, &pair()).unwrap().value(),
            &Some(super::Color::Red)
        );
        let scored = Listener::answering(|_| {
            Canned::ok(include_str!(
                "../../../../specification/fixtures/images/liquid-score-reply.json"
            ))
        })
        .unwrap();
        let engine = build(&scored, "llamacpp", model, profile).build().unwrap();
        let score = Question::score("How red?")
            .unwrap()
            .level("none", None)
            .unwrap()
            .level("all", None)
            .unwrap()
            .build()
            .unwrap();
        assert_eq!(*engine.score_input(&score, &pair()).unwrap().value(), 0.8);
        for captured in [chosen.requests(), scored.requests()] {
            assert_eq!(captured.len(), 1);
            assert_eq!(
                serde_json::from_slice::<Value>(&captured[0].body).unwrap()["images"],
                json!([url(RED, "image/png"), url(BLUE, "image/png")])
            );
        }
    }
}

#[test]
fn unknown_unprofiled_incompatible_routes_and_overrides_send_nothing_while_text_still_works() {
    let listener = Listener::answering(|_| Canned::ok(REPLY)).unwrap();
    for (backend, model, profile) in [
        ("llamacpp", "other", CLEF),
        ("mlx", "imajev-2b", IMAJEV),
        ("ollama", "clef-local-0036", CLEF),
        ("openrouter", "cloudflare/clef", CLEF),
        ("liquid", "d1", CLEF),
    ] {
        let engine = build(&listener, backend, model, profile).build().unwrap();
        assert_eq!(
            engine
                .decide_input(&question(), &pair())
                .unwrap_err()
                .kind(),
            ErrorKind::Usage
        );
    }
    let unprofiled = Engine::builder()
        .backend("llamacpp")
        .unwrap()
        .base_url(listener.base())
        .unwrap()
        .model("clef-local-0036")
        .unwrap()
        .no_cache()
        .build()
        .unwrap();
    assert_eq!(
        unprofiled
            .decide_input(&question(), &pair())
            .unwrap_err()
            .kind(),
        ErrorKind::Usage
    );
    let unknown = CLEF.replace("clef-llamacpp-v0.6.0-0036", "clef");
    let error = Engine::builder().profile_json(&unknown).unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Usage);
    assert!(!format!("{error:?}").contains("clef-local-0036"));
    let engine = build(&listener, "llamacpp", "clef-local-0036", CLEF)
        .build()
        .unwrap();
    let overridden = Question::decide("Is red visible?")
        .unwrap()
        .model("other")
        .unwrap()
        .cut();
    assert_eq!(
        engine
            .decide_input(&overridden, &pair())
            .unwrap_err()
            .kind(),
        ErrorKind::Usage
    );
    assert_eq!(listener.count(), 0);
    engine.decide(&overridden, "Original text.").unwrap();
    assert_eq!(listener.count(), 1);
    assert!(
        serde_json::from_slice::<Value>(&listener.requests()[0].body)
            .unwrap()
            .get("images")
            .is_none()
    );
}

#[test]
fn exact_final_json_and_caller_limits_are_checked_before_sending() {
    let listener = Listener::answering(|_| Canned::ok(REPLY)).unwrap();
    let engine = build(&listener, "llamacpp", "clef-local-0036", CLEF)
        .build()
        .unwrap();
    let image = fixture(RED, ImageMedia::Png);
    engine
        .decide_input(&question(), &input(None, vec![image.clone()]))
        .unwrap();
    let base = listener.requests()[0].body.len();
    let at_limit = input(Some(&"x".repeat(2_800_000 - base)), vec![image.clone()]);
    engine.decide_input(&question(), &at_limit).unwrap();
    assert_eq!(listener.requests()[0].body.len(), 2_800_000);
    let over = input(Some(&"x".repeat(2_800_001 - base)), vec![image.clone()]);
    assert_eq!(
        engine.decide_input(&question(), &over).unwrap_err().kind(),
        ErrorKind::Usage
    );
    for profile in [
        CLEF.replace(
            "\"image_profile\":",
            &format!("\"max_request_bytes\":{},\"image_profile\":", base - 1),
        ),
        CLEF.replace(
            "\"image_profile\":",
            "\"max_evidence_bytes\":1,\"image_profile\":",
        ),
    ] {
        let engine = build(&listener, "llamacpp", "clef-local-0036", &profile)
            .build()
            .unwrap();
        assert_eq!(
            engine
                .decide_input(&question(), &pair())
                .unwrap_err()
                .kind(),
            ErrorKind::Usage
        );
    }
    let engine = build(&listener, "llamacpp", "clef-local-0036", CLEF)
        .max_request_bytes(base - 1)
        .unwrap()
        .build()
        .unwrap();
    assert_eq!(
        engine
            .decide_input(&question(), &input(None, vec![image]))
            .unwrap_err()
            .kind(),
        ErrorKind::Usage
    );
    let choose = Question::choose::<super::Color>("Which color?")
        .unwrap()
        .option(super::Color::Red, None)
        .unwrap()
        .option(super::Color::Blue, None)
        .unwrap()
        .build()
        .unwrap();
    let limited = CLEF.replace("\"image_profile\":", "\"max_options\":1,\"image_profile\":");
    let limited = build(&listener, "llamacpp", "clef-local-0036", &limited)
        .build()
        .unwrap();
    assert_eq!(
        limited.choose_input(&choose, &pair()).unwrap_err().kind(),
        ErrorKind::Usage
    );
    assert_eq!(listener.count(), 2);
}

#[test]
fn envelope_excess_is_refused_with_zero_loopback_requests() {
    let listener = Listener::answering(|_| Canned::ok(REPLY)).unwrap();
    let engine = build(&listener, "llamacpp", "clef-local-0036", CLEF)
        .build()
        .unwrap();
    for images in [
        vec![fixture(RED, ImageMedia::Png); 3],
        vec![fixture(
            include_bytes!("../../../../specification/fixtures/images/local/bytes-1048577.png"),
            ImageMedia::Png,
        )],
        vec![fixture(
            include_bytes!("../../../../specification/fixtures/images/local/edge-1025.png"),
            ImageMedia::Png,
        )],
    ] {
        let error = engine
            .decide_input(&question(), &input(Some("PRIVATE_CONTEXT"), images))
            .unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Usage);
        assert!(!format!("{error:?} {error}").contains("PRIVATE_"));
    }
    assert_eq!(listener.count(), 0);
}

#[test]
fn retries_cache_replay_cancellation_and_batch_changes_keep_the_same_image_question() {
    let attempts = AtomicUsize::new(0);
    let listener = Listener::answering(move |_| {
        if attempts.fetch_add(1, Ordering::Relaxed) == 0 {
            Canned::status(503, "retry")
        } else {
            Canned::ok(REPLY)
        }
    })
    .unwrap();
    let place = folder();
    let engine = build(&listener, "llamacpp", "clef-local-0036", CLEF)
        .max_retries(1)
        .cache_at(&place)
        .unwrap()
        .build()
        .unwrap();
    let rows = engine
        .decide_input_many_with(
            &question(),
            vec![pair(), pair()],
            CallOptions::new().batch(BatchSetting::Records(NonZeroUsize::MIN)),
        )
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    assert_eq!(rows.len(), 2);
    assert_eq!(listener.count(), 2);
    let bodies = listener.requests();
    assert_eq!(bodies[0].body, bodies[1].body);
    let again = engine
        .decide_input_many_with(
            &question(),
            vec![pair(), pair()],
            CallOptions::new().batch(BatchSetting::Max),
        )
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    assert_eq!(again.len(), 2);
    assert_eq!(listener.count(), 2);
    let replay = build(&listener, "llamacpp", "clef-local-0036", CLEF)
        .replay(&place)
        .unwrap()
        .build()
        .unwrap();
    assert_eq!(
        replay
            .decide_input(&question(), &pair())
            .unwrap()
            .facts()
            .requests_sent(),
        0
    );
    let reversed = input(
        Some("Compare originals."),
        vec![
            fixture(BLUE, ImageMedia::Png),
            fixture(RED, ImageMedia::Png),
        ],
    );
    assert_eq!(
        replay
            .decide_input(&question(), &reversed)
            .unwrap_err()
            .kind(),
        ErrorKind::Local
    );
    let token = CancelToken::new();
    token.cancel();
    assert_eq!(
        engine
            .decide_input_with(&question(), &pair(), CallOptions::new().cancel(&token))
            .unwrap_err()
            .kind(),
        ErrorKind::Cancelled
    );
    assert_eq!(listener.count(), 2);
    drop(replay);
    drop(engine);
    std::fs::remove_dir_all(place).unwrap();
}

#[test]
fn cli_profile_uses_the_existing_file_door_and_refuses_unknown_ids_without_sends() {
    let listener = Listener::answering(|_| Canned::ok(REPLY)).unwrap();
    let place = folder();
    let profile = place.join("profile.json");
    let image = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../specification/fixtures/images/red.png");
    let call = || {
        let mut command = Command::new(env!("CARGO_BIN_EXE_thinkthen"));
        command
            .clear_environment()
            .home(&place)
            .env("THINKTHEN_API_KEY", "PRIVATE_LOCAL_KEY")
            .args([
                "decide",
                "Is red visible?",
                "--backend",
                "llamacpp",
                "--model",
                "clef-local-0036",
                "--url",
                listener.base(),
                "--profile",
            ])
            .arg(&profile)
            .arg("--image")
            .arg(&image)
            .args(["--no-cache", "--max-retries", "0"])
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        child::wait::finish(command.spawn().unwrap(), "local image profile CLI").unwrap()
    };
    std::fs::write(
        &profile,
        CLEF.replace("clef-llamacpp-v0.6.0-0036", "unknown"),
    )
    .unwrap();
    let refused = call();
    assert_eq!(refused.status.code(), Some(5));
    assert_eq!(listener.count(), 0);
    assert!(!String::from_utf8_lossy(&refused.stderr).contains("PRIVATE_LOCAL_KEY"));
    std::fs::write(&profile, CLEF).unwrap();
    let admitted = call();
    assert_eq!(
        admitted.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&admitted.stderr)
    );
    assert_eq!(admitted.stdout, b"true\n");
    assert_eq!(listener.count(), 1);
    assert!(!String::from_utf8_lossy(&admitted.stderr).contains("PRIVATE_LOCAL_KEY"));
    std::fs::remove_dir_all(place).unwrap();
}

#[test]
fn server_context_rejection_returns_a_failure_and_sends_the_complete_original_prompt() {
    let listener = Listener::answering(|_| Canned::status(400, "context overflow")).unwrap();
    let engine = build(&listener, "llamacpp", "clef-local-0036", CLEF)
        .build()
        .unwrap();
    let error = engine.decide_input(&question(), &pair()).unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Backend);
    assert!(!format!("{error:?}").contains("PRIVATE_LOCAL_KEY"));
    assert_eq!(listener.count(), 1);
    let body: Value = serde_json::from_slice(&listener.requests()[0].body).unwrap();
    assert_eq!(body["state"], "Compare originals.");
    assert_eq!(
        body["images"],
        json!([url(RED, "image/png"), url(BLUE, "image/png")])
    );
}

#[test]
fn native_partial_usage_recording_keeps_input_unknown_mass_and_absent_output_verbatim() {
    let exchange: Value = serde_json::from_str(include_str!(
        "../../../../specification/fixtures/images/local/0036-imajev-partial.json"
    ))
    .unwrap();
    let response = exchange["response"].clone();
    let original = response.clone();
    let listener = Listener::answering(move |_| Canned::ok(&response.to_string())).unwrap();
    let place = folder();
    let engine = build(&listener, "llamacpp", "imajev-2b", IMAJEV)
        .record(&place)
        .unwrap()
        .build()
        .unwrap();
    // Decoding partial observations belongs to the native owner. This test
    // protects raw persistence independently of normalized decoding success.
    let _ = engine.details_input(&question(), &pair());
    assert_eq!(listener.count(), 1);
    let files = std::fs::read_dir(place.join("exchanges"))
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    assert_eq!(files.len(), 1);
    let recorded: Value = serde_json::from_slice(&std::fs::read(files[0].path()).unwrap()).unwrap();
    assert_eq!(recorded["response"], original);
    assert_eq!(recorded["response"]["usage"]["input_tokens"], 887);
    assert!(recorded["response"]["usage"].get("output_tokens").is_none());
    assert_eq!(
        recorded["response"]["answers"]["q1"]["noul"],
        0.6316676506859308
    );
    assert_eq!(
        recorded["response"]["answers"]["q1"]["unknown_probability"],
        0.0009922848031868846
    );
    assert_eq!(recorded["response"]["answers"]["q1"]["abstained"], false);
    drop(engine);
    std::fs::remove_dir_all(place).unwrap();
}
