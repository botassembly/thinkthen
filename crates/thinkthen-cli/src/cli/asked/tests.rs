//! Stays: the command-owned asked-question representation must preserve authored order and selection without exposing native internals.
//! Native file admission and command source labels share one retained definition.
use super::*;
use crate::{BatchSetting, Request, RequestArguments, RequestInput, RequestOptions, RequestQuestion};
use std::sync::atomic::{AtomicU64, Ordering};

struct Folder(std::path::PathBuf);
impl Folder {
    fn new() -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let path = std::env::temp_dir().join(format!(
            "thinkthen-command-settings-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir(&path).unwrap();
        Self(path)
    }
}
impl Drop for Folder {
    fn drop(&mut self) {
        let _removed = std::fs::remove_dir_all(&self.0);
    }
}

#[test]
fn caller_setup_tiers_preserve_the_saved_definition_and_native_preview() {
    let folder = Folder::new();
    let path = folder.0.join("question.json");
    std::fs::write(&path, r#"{"decide":"question-marker","true":"saved meaning","model":"file-model","on":["/old"],"batch":2,"profile":"file-profile","threshold":0.7}"#).unwrap();
    let mut admitted = Request::named(
        "decide",
        RequestArguments {
            question: RequestQuestion::File { path: path.clone() },
            input: RequestInput::Json {
                value: crate::RawRecord::json(r#"{"old":"excluded","new":"selected"}"#).unwrap(),
                images: Vec::new(),
            },
            options: RequestOptions {
                model: Some("caller-model".to_owned()),
                field: Some(vec!["/new".to_owned()]),
                threshold: Some(crate::RequestThreshold::Cut(0.8)),
                yes: Some(crate::Description::text("caller meaning").unwrap()),
                ..RequestOptions::default()
            },
        },
    )
    .unwrap()
    .admit()
    .unwrap();
    let (settings, tier, set) = prepare(&mut admitted).unwrap();
    assert!(set.is_none());
    assert_eq!(settings.model.as_deref(), Some("caller-model"));
    assert_eq!(settings.profile.as_deref(), Some("file-profile"));
    assert_eq!(settings.on, ["/new"]);
    assert!(settings.saved && settings.literal_question && tier.tuned);
    assert_eq!(
        tier.batch.as_ref().map(crate::judge::file_batch).transpose().unwrap(),
        Some(BatchSetting::Records(
            std::num::NonZeroUsize::new(2).unwrap()
        ))
    );
    assert_eq!(
        crate::render::json_line(&settings.sources).unwrap(),
        r#"{"question":"file","true":"command line","false":"default","threshold":"command line","on":"command line","model":"command line"}"#
    );
    assert!(!format!("{settings:?} {admitted:?}").contains("question-marker"));
    std::fs::remove_file(path).unwrap();
    assert!(admitted.resolve_question().is_ok());
    let listener =
        conformance_backend::Listener::answering(|_| conformance_backend::Canned::ok("{}"))
            .unwrap();
    let engine = crate::Engine::builder()
        .base_url(listener.base())
        .unwrap()
        .api_key("fixture-key")
        .unwrap()
        .no_cache()
        .build()
        .unwrap();
    let plan = engine
        .plan_request(&admitted, crate::RequestEnvironment::default())
        .unwrap();
    assert_eq!(plan.records(), 1);
    assert!(plan.first_body().is_some());
    assert!(listener.requests().is_empty());
    assert_eq!(listener.count(), 0);
}
