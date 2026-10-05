//! Public command names and reserved nouns at the compiled boundary.

use crate::harness::spawn;

#[test]
fn help_advertises_backend_checks_and_hides_the_published_alias() {
    let help = spawn(&["--help"], &[], b"").expect("help runs");
    assert_eq!(help.status.code(), Some(0));
    let text = String::from_utf8_lossy(&help.stdout);
    let names: Vec<_> = text
        .lines()
        .filter_map(|line| {
            line.strip_prefix("  ")
                .and_then(|line| line.split_whitespace().next())
        })
        .collect();
    assert!(names.contains(&"backends"), "{text}");
    assert!(!names.contains(&"check"), "{text}");
    let help = spawn(&["backends", "check", "--help"], &[], b"").expect("check help runs");
    assert_eq!(help.status.code(), Some(0));
    assert!(String::from_utf8_lossy(&help.stdout).contains("Usage: thinkthen backends check"));
}

#[test]
fn reserved_top_level_nouns_are_refused_without_sending() {
    let listener = crate::harness::Listener::serving(Vec::new()).expect("listener");
    for noun in [
        "questions",
        "items",
        "answers",
        "checks",
        "datasets",
        "runs",
        "setups",
        "findings",
        "people",
        "search",
    ] {
        for arguments in [vec![noun], vec!["help", noun]] {
            let output = spawn(
                &arguments,
                &[
                    ("THINKTHEN_BASE_URL", listener.base()),
                    ("THINKTHEN_API_KEY", "namespace-fake-key"),
                ],
                b"",
            )
            .expect("binary runs");
            assert_eq!(output.status.code(), Some(2), "{arguments:?}");
            assert!(output.stdout.is_empty(), "{arguments:?}");
            let said = String::from_utf8_lossy(&output.stderr);
            assert!(
                said.contains(&format!("unrecognized subcommand '{noun}'")),
                "{said}"
            );
            assert!(!said.contains("namespace-fake-key"), "{said}");
        }
    }
    assert!(listener.requests().is_empty());
    assert_eq!(listener.connections(), 0);
}
