use std::path::Path;
use std::process::Command;
use std::time::Duration;

use thinkthen::{Answer, Engine, Entity, ErrorKind, Judgment, Question, Relate};

fn related(engine: &Engine, case: &serde_json::Value, step: &serde_json::Value, id: &str) {
    let entities = case["entities"]
        .as_array()
        .expect("entities")
        .iter()
        .map(|one| {
            Entity::new(
                one["name"].as_str().expect("name"),
                one["kind"].as_str().expect("kind"),
            )
            .expect("entity")
        })
        .collect::<Vec<_>>();
    let rule = Relate::from_json(
        r#"{"version":1,"relate":{"relations":[{"name":"linked","source":"item","target":"item"}]}}"#,
    )
    .expect("rule");
    let edges = engine.relate(&rule, entities).expect("relate").into_value();
    assert_eq!(edges.len(), step["edges"].as_u64().expect("edges") as usize, "{id}");
}

#[test]
fn builder_validates_new_settings_at_the_public_edge() {
    let timeout = Engine::builder()
        .timeout(Duration::ZERO)
        .expect_err("zero timeout");
    assert_eq!(
        (timeout.kind(), timeout.to_string().as_str()),
        (ErrorKind::Usage, "a timeout is a time above zero")
    );
    let profile = Engine::builder().profile("").expect_err("empty profile");
    assert_eq!(
        (profile.kind(), profile.to_string().as_str()),
        (ErrorKind::Usage, "a profile file is a path, not empty")
    );
    let replay = Engine::builder().replay("").expect_err("empty replay");
    assert_eq!(
        (replay.kind(), replay.to_string().as_str()),
        (ErrorKind::Usage, "a recording folder is a path, not empty")
    );
    let missing = std::env::temp_dir().join(format!("consumer-profile-missing-{}", std::process::id()));
    let error = Engine::builder().profile(&missing).expect("path").build().expect_err("missing");
    assert_eq!((error.kind(), error.to_string().as_str()),
        (ErrorKind::Local, "the profile file could not be read"));
    let bad = std::env::temp_dir().join(format!("consumer-profile-invalid-{}", std::process::id()));
    std::fs::write(&bad, r#"{"schema":"thinkthen.backend-profile/1","unknown":1}"#).expect("bad profile");
    let error = Engine::builder().profile(&bad).expect("path").build().expect_err("invalid");
    assert_eq!((error.kind(), error.to_string().as_str()),
        (ErrorKind::Local, "the profile file holds no unknown keys"));
    std::fs::remove_file(bad).expect("clean profile");
}

#[test]
fn an_old_recording_replays_read_only_without_a_key() {
    let source = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../../demos/27-test-with-no-network/recording");
    let copied = std::env::temp_dir().join(format!("thinkthen-0148-legacy-{}", std::process::id()));
    std::fs::create_dir(&copied).expect("fresh copy folder");
    let entry = std::fs::read_dir(&source)
        .expect("source")
        .next()
        .expect("entry")
        .expect("entry");
    let saved = std::fs::read(entry.path()).expect("saved bytes");
    std::fs::write(copied.join(entry.file_name()), &saved).expect("copy entry");
    let output = Command::new(std::env::current_exe().expect("test binary"))
        .args(["--exact", "--ignored", "settings::old_recording_child"])
        .env_clear()
        .env("THINKTHEN_0148_LEGACY", &copied)
        .env("THINKTHEN_CACHE", copied.join("seeded-cache"))
        .output()
        .expect("child");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stdout)
    );
    assert!(!copied.join(".thinkthen-backend.json").exists());
    assert_eq!(
        std::fs::read(copied.join(entry.file_name())).expect("unchanged"),
        saved
    );
    assert_eq!(std::fs::read_dir(&copied).expect("one entry").count(), 1);
    std::fs::remove_dir_all(copied).expect("clean copied folder");
}

#[test]
#[ignore = "the parent runs this in a cleared environment"]
fn old_recording_child() {
    let Ok(folder) = std::env::var("THINKTHEN_0148_LEGACY") else {
        return;
    };
    let engine = thinkthen::EngineBuilder::from_env()
        .expect("seeded builder")
        .replay(&folder)
        .expect("replay")
        .build()
        .expect("engine");
    let question =
        Question::decide("Does this report say what the person did before the problem appeared?")
            .expect("question")
            .cut();
    let report = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../demos/27-test-with-no-network/report.txt"
    ));
    let vague = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../demos/27-test-with-no-network/vague.txt"
    ));
    assert_eq!(
        engine.decide(&question, report).expect("saved").into_value(),
        Answer::Yes
    );
    let error = engine.decide(&question, vague).expect_err("local miss");
    assert_eq!(
        (error.kind(), error.to_string().as_str()),
        (
            ErrorKind::Local,
            "the replay folder holds no reply for this request"
        )
    );
    let writer = Engine::builder()
        .cache_at(&folder)
        .expect("cache path")
        .build()
        .expect("writer");
    let error = writer.decide(&question, report).expect_err("legacy writer");
    assert_eq!(
        (error.kind(), error.to_string().as_str()),
        (
            ErrorKind::Local,
            "the recording folder predates backend binding; replay it read-only or choose a new folder"
        )
    );
}

#[test]
fn the_builder_follows_the_command_folder_rules() {
    use conformance_backend::Backend;
    let backend = Backend::start().expect("loopback");
    let base = format!("{}/generic/v1", backend.origin());
    let root = std::env::temp_dir().join(format!("consumer-folders-{}", std::process::id()));
    std::fs::create_dir(&root).expect("fresh root");
    let saved = root.join("saved");
    let other = root.join("other");
    let missing = root.join("missing");
    let file = root.join("file");
    std::fs::write(&file, b"not a folder").expect("file");
    let ask = Question::decide("Does this need attention?").expect("question").cut();
    let error = Engine::builder().cache_at(&saved).expect("cache")
        .replay(&saved).expect("replay").build().expect_err("cache conflict");
    assert_eq!(error.to_string(), "a cache folder is record and replay on one folder, so it stands beside neither");
    let error = Engine::builder().record(&saved).expect("record")
        .replay(&other).expect("replay").build().expect_err("two folders");
    assert_eq!(error.to_string(), "record and replay name two different folders, and one engine keeps one");
    let error = Engine::builder().record(&file).expect("record path")
        .build().expect_err("file is not a folder");
    assert_eq!((error.kind(), error.to_string().as_str()),
        (ErrorKind::Local, "the recording folder names a file"));
    let miss = Engine::builder().base_url(&base).expect("base")
        .replay(&missing).expect("replay").build().expect("missing replay");
    assert_eq!(miss.decide(&ask, "never saved").expect_err("miss").kind(), ErrorKind::Local);
    assert!(!missing.exists(), "replay does not make a folder");
    let paired = Engine::builder().base_url(&base).expect("base")
        .api_key("loopback").expect("key").record(&saved).expect("record")
        .replay(&saved).expect("replay").build().expect("pair");
    assert_eq!(paired.decide(&ask, "one").expect("send").into_value(), Answer::Yes);
    assert_eq!(paired.decide(&ask, "one").expect("replay").into_value(), Answer::Yes);
    assert_eq!((backend.count(), paired.usage().cache_answers()), (1, 0));
    let recorder = Engine::builder().base_url(&base).expect("base")
        .api_key("loopback").expect("key").record(&saved).expect("record")
        .build().expect("record only");
    assert_eq!(recorder.decide(&ask, "one").expect("send again").into_value(), Answer::Yes);
    assert_eq!(backend.count(), 2);
    let reader = Engine::builder().base_url(&base).expect("base")
        .no_cache().replay(&saved).expect("replay").build().expect("reader");
    assert_eq!(reader.decide(&ask, "one").expect("saved").into_value(), Answer::Yes);
    assert_eq!(backend.count(), 2);
    let wrong = Engine::builder().base_url(&format!("{}/arm/503/v1", backend.origin()))
        .expect("other base").replay(&saved).expect("replay").build().expect("wrong reader");
    assert_eq!(wrong.decide(&ask, "one").expect_err("mismatch").kind(), ErrorKind::Local);
    assert_eq!(backend.count(), 2, "a mismatch sends nothing");
    std::fs::remove_dir_all(root).expect("clean root");
}

#[test]
#[expect(
    clippy::excessive_nesting,
    clippy::cognitive_complexity,
    clippy::too_many_lines,
    reason = "one shared corpus row maps settings, runs the engine, and checks its wire effect"
)]
fn every_shared_setting_reaches_the_public_engine() {
    use conformance_backend::Backend;
    use serde_json::Value;
    let corpus: Value =
        serde_json::from_str(include_str!("../../../../settings.json")).expect("cases");
    assert_eq!(corpus["schema"], "thinkthen.settings-cases/1");
    for case in corpus["cases"].as_array().expect("cases") {
        let backend = Backend::start().expect("backend");
        let id = case["id"].as_str().expect("id");
        let base = format!(
            "{}/{}",
            backend.origin(),
            case["arm"].as_str().expect("arm")
        );
        let folder =
            std::env::temp_dir().join(format!("consumer-settings-{id}-{}", std::process::id()));
        std::fs::create_dir(&folder).expect("fresh folder");
        let profile = folder.join("profile.json");
        if !case["profile"].is_null() {
            std::fs::write(&profile, case["profile"].to_string()).expect("profile");
        }
        let question = Question::decide(corpus["question"].as_str().expect("question"))
            .expect("question")
            .cut();
        for step in case["steps"].as_array().expect("steps") {
            let mut builder = Engine::builder().base_url(&base).expect("base").no_cache();
            for (name, value) in step["settings"].as_object().expect("settings") {
                builder = match name.as_str() {
                    "timeout" => builder
                        .timeout(Duration::from_secs(value.as_u64().expect("seconds")))
                        .expect("timeout"),
                    "max_retries" => builder.max_retries(value.as_u64().expect("retries") as u32),
                    "profile" => builder.profile(&profile).expect("profile"),
                    "model" => builder
                        .model(value.as_str().expect("model"))
                        .expect("model"),
                    "record" => builder.record(&folder).expect("record"),
                    "replay" => builder.replay(&folder).expect("replay"),
                    "cache" if value == &Value::Bool(false) => builder.no_cache(),
                    "cache" => builder.cache_at(&folder).expect("cache"),
                    "max_requests" => builder
                        .max_requests(Some(value.as_u64().expect("limit") as usize))
                        .expect("limit"),
                    "max_request_bytes" => builder
                        .max_request_bytes(value.as_u64().expect("bytes") as usize)
                        .expect("bytes"),
                    _ => panic!("unknown shared setting {name}"),
                };
            }
            let engine = builder.build().expect("engine");
            if step["verb"] == "relate" {
                related(&engine, case, step, id);
            } else if step["verb"] == "decide_many" {
                let records = step["records"].as_array().expect("records");
                let answers: Vec<_> = engine
                    .decide_many(&question, records.iter().map(|v| v.as_str().expect("text")))
                    .collect();
                assert_eq!(answers.len(), records.len(), "{id}");
                assert_eq!(
                    answers[2].as_ref().expect_err("limit").kind().name(),
                    step["error"],
                    "{id}"
                );
            } else {
                let text = step["text"].as_str().expect("text");
                if let Some(model) = step["model"].as_str() {
                    let details = engine.details(&question, text).expect("details");
                    assert_eq!(details.value().model(), model, "{id}");
                    assert_eq!(details.value().value(), &Judgment::Decision(Answer::Yes), "{id}");
                } else {
                    let result = engine.decide(&question, text);
                    if let Some(kind) = step["error"].as_str() {
                        assert_eq!(result.expect_err(id).kind().name(), kind, "{id}");
                    } else {
                        assert_eq!(result.expect(id).into_value(), Answer::Yes, "{id}");
                    }
                }
            }
            assert_eq!(
                backend.count(),
                step["count"].as_u64().expect("count") as usize,
                "{id}"
            );
        }
        if let Some(entries) = case["entries"].as_u64() {
            let count = std::fs::read_dir(&folder)
                .expect("folder")
                .filter(|entry| {
                    entry.as_ref().is_ok_and(|entry| {
                        entry.file_name() != ".thinkthen-backend.json"
                            && entry.path().extension().is_some_and(|ext| ext == "json")
                    })
                })
                .count();
            assert_eq!(count, entries as usize, "{id}");
        }
        std::fs::remove_dir_all(folder).expect("clean folder");
    }
}

#[test]
fn relate_follows_the_engine_profile_like_the_command() {
    use conformance_backend::Backend;
    use std::io::Write;
    use std::process::Stdio;
    use thinkthen::{Entity, Relate};
    let backend = Backend::start().expect("backend");
    let folder =
        std::env::temp_dir().join(format!("consumer-relate-profile-{}", std::process::id()));
    std::fs::create_dir(&folder).expect("fresh folder");
    let profile = folder.join("profile.json");
    std::fs::write(
        &profile,
        r#"{"schema":"thinkthen.backend-profile/1","name":"small","max_evidence_bytes":4}"#,
    )
    .expect("profile");
    let base = format!("{}/generic/v1", backend.origin());
    let engine = Engine::builder()
        .base_url(&base)
        .expect("base")
        .profile(&profile)
        .expect("profile")
        .no_cache()
        .build()
        .expect("engine");
    let ask = Relate::from_json(r#"{"version":1,"relate":{"relations":[{"name":"works_for","source":"person","target":"organization"}]}}"#).expect("rule");
    let entities = [
        Entity::new("Ada", "person").expect("person"),
        Entity::new("Acme", "organization").expect("organization"),
    ];
    let library = engine.relate(&ask, entities).expect_err("profile limit");
    let binary = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../target/debug/thinkthen");
    let mut command = Command::new(binary)
        .args([
            "relate",
            "works_for=person:organization",
            "--url",
            &base,
            "--profile",
        ])
        .arg(&profile)
        .env_clear()
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("command");
    command
        .stdin
        .take()
        .expect("input")
        .write_all(br#"[{"name":"Ada","kind":"person"},{"name":"Acme","kind":"organization"}]"#)
        .expect("entities");
    let output = command.wait_with_output().expect("output");
    let said = String::from_utf8_lossy(&output.stderr);
    assert_eq!(library.kind(), ErrorKind::Usage);
    assert_eq!(output.status.code(), Some(2), "{said}");
    assert!(
        said.contains("profile small allows at most 4 evidence bytes"),
        "{said}"
    );
    assert_eq!(backend.count(), 0);
    std::fs::remove_dir_all(folder).expect("clean folder");
}
