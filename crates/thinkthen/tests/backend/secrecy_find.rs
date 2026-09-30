//! The aggregate `find` command over every shared backend and recording route.

use std::io;

use crate::harness::{Listener, spawn};
use crate::secrecy::{
    CLOSED, EVIDENCE, HOSTILE, HOSTILE_SCHEMA_MARKER, PATHS, QUESTION, Route, damage_fixture,
    environment, folder, nothing_leaked,
};

const ANSWER: &str = concat!(
    r#""type":"choice","choice":"u001","#,
    r#""probabilities":{"u001":0.9,"u002":0.1}"#,
);

fn sweep(route: &Route, details: bool) -> io::Result<()> {
    let view = if details { "details" } else { "bare" };
    let case = format!("{}-find-{view}", route.named.replace(' ', "-"));
    let into = folder(&case)?;
    let dir = into.join("recording");
    let listener = Listener::serving(route.answers.script(ANSWER))?;
    let evidence = format!("{EVIDENCE} first\n{EVIDENCE} second\n");
    let named = |argument: &&str| match *argument {
        "{dir}" => dir.to_string_lossy().into_owned(),
        "{closed}" => CLOSED.to_owned(),
        other => other.to_owned(),
    };
    let mut asked = vec!["find".to_owned(), QUESTION.to_owned()];
    if !route.adds.contains(&"--url") {
        asked.extend(["--url".to_owned(), listener.base().to_owned()]);
    }
    asked.extend(["--model".to_owned(), "local-1".to_owned()]);
    if details {
        asked.push("--details".to_owned());
    }
    asked.extend(route.adds.iter().map(named));

    if route.primed {
        let priming: Vec<&str> = asked
            .iter()
            .map(String::as_str)
            .map(|argument| {
                if argument == "--replay" {
                    "--record"
                } else {
                    argument
                }
            })
            .collect();
        let first = spawn(&priming, &environment(true), evidence.as_bytes())?;
        assert_eq!(first.status.code(), Some(0), "{case}: priming");
    }
    // `find` keeps its answers in the question store, so damage reaches the
    // fixture a replay reads.
    let says = match route.damage {
        Some(damage) => {
            damage_fixture(&dir, damage)?;
            route.fixture_says
        }
        None => route.says,
    };

    let arguments: Vec<&str> = asked.iter().map(String::as_str).collect();
    let output = spawn(&arguments, &environment(route.keyed), evidence.as_bytes())?;
    assert_eq!(
        output.status.code(),
        Some(route.code),
        "{case}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        listener.requests().len(),
        route.requests,
        "{case}: requests"
    );
    if let Some(says) = says {
        let diagnostic = String::from_utf8_lossy(&output.stderr);
        assert!(diagnostic.contains(says), "{case}: {diagnostic}");
    }
    nothing_leaked(&case, &output, &into);
    if route.damage == Some(HOSTILE) {
        for bytes in [&output.stdout, &output.stderr] {
            let text = String::from_utf8_lossy(bytes);
            for forbidden in [HOSTILE_SCHEMA_MARKER, EVIDENCE] {
                assert!(!text.contains(forbidden), "{case}: {forbidden}");
            }
        }
    }
    Ok(())
}

#[test]
fn find_crosses_every_shared_backend_and_recording_route_without_a_leak() {
    for route in &PATHS {
        for details in [false, true] {
            sweep(route, details).expect("find secrecy route");
        }
    }
}
