#![cfg(test)]
//! Backend address and credential ownership through the command.
#![cfg(feature = "command")]

#[path = "../../loopback/src/lib.rs"]
pub mod loopback;

mod support;

use std::io::Write;
use std::process::Stdio;

use loopback::{Loopback, Reply};
use serde_json::json;

#[test]
fn command_routes_each_backends_key_only_to_its_own_address() {
    let system = Loopback::noul(0.9).unwrap();
    let openai = Loopback::start(|_| {
        Reply::json(
            200,
            &json!({"answers":[{"type":"predicate","probability":0.9}]}),
        )
    })
    .unwrap();
    for (backend, key, server, retired_url) in [
        ("systemone", "synthetic-system", &system, openai.base()),
        ("openai", "synthetic-openai", &openai, system.base()),
    ] {
        let mut command = support::command();
        command
            .args(["decide", "Is it urgent?", "--backend", backend])
            .env("THINKTHEN_API_KEY", "synthetic-system")
            .env("OPENAI_API_KEY", "synthetic-openai")
            .env("THINKTHEN_BASE_URL", system.base())
            .env("THINKTHEN_URL", retired_url)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        if backend == "openai" {
            command.args(["--url", openai.base()]);
        }
        let mut child = command.spawn().unwrap();
        child
            .stdin
            .take()
            .unwrap()
            .write_all(b"Please respond now.")
            .unwrap();
        let output = child.wait_with_output().unwrap();
        assert_eq!(output.status.code(), Some(0));
        assert_eq!(output.stdout, b"true\n");
        assert!(output.stderr.is_empty());
        let requests = server.requests();
        assert_eq!(requests.len(), 1);
        assert!(requests[0].header("authorization") == Some(format!("Bearer {key}").as_str()));
        if backend == "systemone" {
            assert_eq!(requests[0].path, "/v1/systemone");
            assert_eq!(openai.count(), 0);
        } else {
            assert_eq!(requests[0].path, "/v1/decisions");
            assert_eq!(system.count(), 1);
        }
    }
}
