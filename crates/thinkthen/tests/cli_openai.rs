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
    for backend in ["systemone", "openai"] {
        for from_environment in [false, true] {
            for explicit_url in [false, true] {
                let system = Loopback::noul(0.9).unwrap();
                let openai = Loopback::start(|_| {
                    Reply::json(
                        200,
                        &json!({"answers":[{"type":"predicate","probability":0.9}]}),
                    )
                })
                .unwrap();
                let (key, server, other, path) = if backend == "systemone" {
                    ("synthetic-system", &system, &openai, "/v1/systemone")
                } else {
                    ("synthetic-openai", &openai, &system, "/v1/decisions")
                };
                let mut command = support::command();
                command
                    .args(["decide", "Is it urgent?"])
                    .env(
                        "THINKTHEN_BACKEND",
                        if from_environment { backend } else { "unknown" },
                    )
                    .env("THINKTHEN_API_KEY", "synthetic-system")
                    .env("OPENAI_API_KEY", "synthetic-openai")
                    .env("THINKTHEN_BASE_URL", other.base())
                    .env("THINKTHEN_URL", system.base())
                    .env("OPENAI_BASE_URL", openai.base())
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped());
                if !from_environment {
                    command.args(["--backend", backend]);
                }
                if explicit_url {
                    // Both environment addresses conflict with the user-typed override.
                    command
                        .env("THINKTHEN_URL", other.base())
                        .env("OPENAI_BASE_URL", other.base())
                        .args(["--url", server.base()]);
                }
                let mut child = command.spawn().unwrap();
                child
                    .stdin
                    .take()
                    .unwrap()
                    .write_all(b"Please respond now.")
                    .unwrap();
                let output = child.wait_with_output().unwrap();
                assert_eq!(
                    other.count(),
                    0,
                    "the other provider must receive no requests"
                );
                assert_eq!(output.status.code(), Some(0));
                assert_eq!(output.stdout, b"true\n");
                assert!(output.stderr.is_empty());
                let requests = server.requests();
                assert_eq!(requests.len(), 1);
                assert!(
                    requests[0].header("authorization") == Some(format!("Bearer {key}").as_str())
                );
                assert_eq!(requests[0].path, path);
            }
        }
    }
}
