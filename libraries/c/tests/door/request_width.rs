//! Counted default and explicit throttle mapping through the existing C driver.
use std::io::Write;
use std::time::Duration;

use conformance_backend::Backend;
use serde_json::json;

use crate::cases::{Script, replies};
use crate::{compile, crate_dir, finished, scratch, start_with, text};

#[test]
fn omitted_and_explicit_throttles_hold_ten_packed_requests() {
    let driver = compile(&crate_dir().join("tests/c/driver.c"));
    for (settings, cap) in [
        (json!({"batch": 2, "cache": false}), 8),
        (json!({"batch": 2, "cache": false, "throttle": 6}), 6),
    ] {
        let backend = Backend::start().expect("held backend");
        let base = format!("{}/arm/held/v1", backend.origin());
        let mut script = Script::default();
        script.ask("settings", &[&base, &settings.to_string()]);
        let records: Vec<String> = (0..20).map(|place| format!("record {place}")).collect();
        let mut fields = vec![base.as_str(), r#"{"decide":"Does this need attention?"}"#];
        fields.extend(records.iter().map(String::as_str));
        script.ask("many", &fields);
        let home = scratch(&format!("throttle-home-{cap}"));
        let mut child = start_with(
            &driver,
            &base,
            &[
                ("HOME", &home),
                ("XDG_CONFIG_HOME", &home.join("config")),
                ("XDG_CACHE_HOME", &home.join("cache")),
                ("XDG_STATE_HOME", &home.join("state")),
            ],
        );
        child
            .stdin
            .take()
            .expect("stdin")
            .write_all(&script.0)
            .expect("script");
        assert_eq!(backend.wait(cap), cap, "held arrivals");
        std::thread::sleep(Duration::from_millis(300));
        assert_eq!(backend.count(), cap, "the next request stays queued");
        backend.release();
        let output = finished(child);
        assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
        assert_eq!(text(&output.stderr), "");
        let said = replies(&output.stdout).expect("replies");
        assert_eq!(said.len(), 2);
        assert_eq!((said[0].0, said[1].0), (0, 0));
        let values: Vec<f64> = said[1]
            .1
            .split_whitespace()
            .map(|value| value.parse().expect("answer number"))
            .collect();
        assert_eq!(values, [1.0, 0.9].repeat(20));
        assert_eq!(backend.count(), 10, "all ten packed requests finished");
    }
}
