//! Every applicable shared case through the C door, on the conformance backend.
//!
//! Each case runs in its own `tests/c/driver.c` process, through the JSON door
//! and, where one fits, a typed function. Expected request digests were
//! recorded against the canonical URL, so each is recomputed for the URL the
//! backend served. A record function's row lists question keys by ADR 0111,
//! so its digests become the keys of the request each digest named. Two cases
//! do not apply to the door:
//!
//! - `18-annotate-two-groups` recorded each group in its own request, and ADR
//!   0111 section 5 packs a record's groups into one, as the command's wire run
//!   and the public API consumer also skip it.
//! - `25-defect-fault` injects an internal invariant failure, which no outside
//!   boundary reaches. The panic test in `src/failures.rs` covers the kind.

use std::collections::{BTreeMap, BTreeSet};
use std::io::Write;
use std::path::{Path, PathBuf};

use conformance_backend::Backend;
use serde_json::value::RawValue;
use serde_json::{Value, json};

use crate::{compile, crate_dir, run, scratch, text};

#[path = "batching.rs"]
mod batching;
mod plan;
#[path = "portable.rs"]
mod portable;
mod wire;

use plan::plan;
pub(crate) use wire::{keys, member, renamed_verb, replies, string, with};
use wire::{parsed, same};

const CASES: &str = include_str!("../../../../conformance/cases.json");
const CANONICAL: &str = "https://api.typesafe.ai/v1/systemone";
const SKIPPED: [&str; 2] = ["18-annotate-two-groups", "25-defect-fault"];

type Checked<T = ()> = Result<T, String>;
pub(crate) type Members = BTreeMap<String, Box<RawValue>>;
/// One driver answer: its code and its bytes.
type Reply = (i32, String);
type Judge<'a> = Box<dyn Fn(&[Reply]) -> Checked + 'a>;

/// The requests one case sends through the driver, in order.
#[derive(Default)]
pub(crate) struct Script(pub(crate) Vec<u8>);

impl Script {
    pub(crate) fn ask(&mut self, verb: &str, fields: &[&str]) {
        self.0
            .extend(format!("{verb} {}\n", fields.len()).as_bytes());
        for field in fields {
            self.0
                .extend(format!("{}\n{field}\n", field.len()).as_bytes());
        }
    }
}

#[test]
fn every_applicable_shared_case_passes_through_the_door() {
    let written: Members = serde_json::from_str(CASES).expect("the shared cases");
    let cases: Vec<Members> = serde_json::from_str(written["cases"].get()).expect("a case list");
    let selected = selected_ids(&cases).expect("a valid shared case selector");
    let selected_count = selected.as_ref().map_or(cases.len(), BTreeSet::len);
    let driver = compile(&crate_dir().join("tests/c/driver.c"));
    let backend = Backend::start().expect("the conformance backend");
    let mut failures = Vec::new();
    let mut ran = 0;
    let mut not_run = 0;
    for case in &cases {
        let id = string(case, "id");
        if selected.as_ref().is_some_and(|ids| !ids.contains(&id)) {
            continue;
        }
        if SKIPPED.contains(&id.as_str()) {
            not_run += 1;
            writeln!(
                std::io::stderr().lock(),
                "{id}: not run by the C door (internal injection or repacked)"
            )
            .expect("write skipped case to stderr");
            continue;
        }
        ran += 1;
        let mut script = Script::default();
        script.ask("env", &["THINKTHEN_BATCH", "1"]);
        let cache = scratch(&format!("case-{id}"));
        script.ask("env", &["THINKTHEN_CACHE", &cache.display().to_string()]);
        let checked = plan(&backend, case, &mut script)
            .and_then(|judge| driven(&driver, &script, &judge))
            .and_then(|()| stored(&backend, case, &cache));
        if let Err(why) = checked {
            failures.push(format!("{id}: {why}"));
        }
    }
    writeln!(
        std::io::stderr().lock(),
        "C door: total={} selected={selected_count} pass={} fail={} not_run={not_run} unselected={}",
        cases.len(),
        ran - failures.len(),
        failures.len(),
        cases.len() - selected_count
    )
    .expect("write case counts to stderr");
    assert_eq!(ran + not_run, selected_count);
    assert!(failures.is_empty(), "{failures:#?}");
}

/// ADR 0111: a record function's case runs on the question store, which
/// holds one row per good answer.
fn stored(backend: &Backend, case: &Members, folder: &Path) -> Checked {
    let expect = member(case, "expect");
    let verb = string(case, "verb");
    if expect.get("error").is_some() || matches!(verb.as_str(), "find" | "recognize" | "relate") {
        return Ok(());
    }
    let served = format!(
        "{}/case/{}/v1/systemone",
        backend.origin(),
        string(case, "id")
    );
    let mut wanted = BTreeSet::new();
    for exchange in member(case, "exchanges").as_array().into_iter().flatten() {
        wanted.extend(keys(
            &served,
            exchange["request"].as_str().unwrap_or_default().as_bytes(),
        )?);
    }
    let failed = expect["success"]["failed_questions"].as_u64().unwrap_or(0);
    let want = i64::try_from(wanted.len()).map_err(|error| error.to_string())?
        - i64::try_from(failed).map_err(|error| error.to_string())?;
    let store = folder.join("thinkthen.sqlite");
    let held: i64 = if store.is_file() {
        rusqlite::Connection::open(store)
            .and_then(|db| db.query_row("SELECT count(*) FROM answers", [], |row| row.get(0)))
            .map_err(|error| error.to_string())?
    } else {
        0
    };
    same("stored answers", &json!(held), &json!(want))
}

/// Read one optional absolute ID list, and refuse duplicate or unknown IDs.
fn selected_ids(cases: &[Members]) -> Checked<Option<BTreeSet<String>>> {
    let mut available = BTreeSet::new();
    for case in cases {
        let id = string(case, "id");
        if id.is_empty() {
            return Err("a shared case has no ID".to_owned());
        }
        if !available.insert(id.clone()) {
            return Err(format!("duplicate shared case `{id}`"));
        }
    }
    let Some(path) = std::env::var_os("THINKTHEN_CONFORMANCE_IDS") else {
        return Ok(None);
    };
    let path = PathBuf::from(path);
    if !path.is_absolute() {
        return Err("THINKTHEN_CONFORMANCE_IDS takes an absolute path".to_owned());
    }
    let text =
        std::fs::read_to_string(&path).map_err(|error| format!("{}: {error}", path.display()))?;
    let mut selected = BTreeSet::new();
    for id in text
        .lines()
        .map(str::trim)
        .filter(|id| !id.is_empty() && !id.starts_with('#'))
    {
        if !selected.insert(id.to_owned()) {
            return Err(format!("duplicate selected case `{id}`"));
        }
    }
    if selected.is_empty() {
        return Err("the selected case list is empty".to_owned());
    }
    for id in &selected {
        if !available.contains(id) {
            return Err(format!(
                "selected case `{id}` is absent from the shared corpus"
            ));
        }
    }
    Ok(Some(selected))
}

/// The retry signal after a failure: 1 for a status the engine retries,
/// 0 for a refused key.
#[test]
fn a_retried_status_is_retryable_and_a_refused_key_is_not() {
    let backend = Backend::start().expect("the conformance backend");
    let asked = r#"{"decide":"Does this need attention?"}"#;
    let mut script = Script::default();
    let folder = scratch("retryable");
    for arm in ["503", "401"] {
        // A cache folder answers one backend address, so each arm gets its own.
        let cache = folder.join(arm).display().to_string();
        script.ask("env", &["THINKTHEN_CACHE", &cache]);
        let path = if arm == "401" { "status/401" } else { arm };
        let base = format!("{}/arm/{path}/v1", backend.origin());
        script.ask("retryable", &[&base, asked, "Is this urgent?"]);
    }
    let driver = compile(&crate_dir().join("tests/c/driver.c"));
    let output = run(&driver, "", &script.0);
    assert!(output.status.success(), "{}", text(&output.stderr));
    let got = replies(&output.stdout).expect("framed replies");
    let want = [(0, "2 1".to_owned()), (0, "2 0".to_owned())];
    assert_eq!(got, want);
}

/// Run one case's requests through the driver and judge its replies.
fn driven(driver: &Path, script: &Script, judge: &Judge<'_>) -> Checked {
    let output = run(driver, "", &script.0);
    if !output.status.success() {
        return Err(format!("the driver failed: {}", text(&output.stderr)));
    }
    judge(&replies(&output.stdout)?)
}

/// ADR 0056: a name `recognize` found carries `text` in place of `name`, and
/// relate reads it as the name. `name` wins when a record holds both.
#[test]
fn relate_reads_what_recognize_found() {
    let driver = compile(&crate_dir().join("tests/c/driver.c"));
    let backend = Backend::start().expect("the conformance backend");
    let base = format!("{}/generic/v1", backend.origin());
    let recognize = r#"{"version":1,"recognize":{"kinds":{"person":null}}}"#;
    let relate = r#"{"version":1,"relate":{"relations":[{"name":"knows","source":"person","target":"person"}]}}"#;
    let mut script = Script::default();
    script.ask("recognize", &[&base, recognize, "Maria Chen arrived."]);
    let found = replies(&run(&driver, &base, &script.0).stdout).expect("a reply");
    let found = parsed(&found[0]).expect("names");
    let records: Vec<String> = found["entities"]
        .as_array()
        .expect("entities")
        .iter()
        .map(ToString::to_string)
        .collect();
    let named = [
        r#"{"name":"Maria Chen","text":"not this","kind":"person"}"#,
        r#"{"name":"arrived.","kind":"person"}"#,
    ];
    let mut script = Script::default();
    for each in [records.iter().map(String::as_str).collect(), named.to_vec()] {
        script.ask("relate", &[vec![base.as_str(), relate], each].concat());
    }
    let got = replies(&run(&driver, &base, &script.0).stdout).expect("replies");
    let by_text = parsed(&got[0]).expect("edges");
    assert_eq!(by_text, parsed(&got[1]).expect("edges"));
    assert_eq!(by_text["edges"].as_array().map(Vec::len), Some(2));
}
