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

#[test]
fn help_advertises_runs_and_hides_the_published_audit_and_diff_aliases() {
    let help = spawn(&["--help"], &[], b"").expect("help runs");
    assert_eq!(help.status.code(), Some(0));
    let text = String::from_utf8_lossy(&help.stdout);
    for name in ["runs", "audit", "diff"] {
        let listed = text
            .lines()
            .any(|line| line.split_whitespace().next() == Some(name));
        assert_eq!(listed, name == "runs", "{text}");
    }
    for arguments in [vec!["runs", "--help"], vec!["help", "runs"]] {
        let help = spawn(&arguments, &[], b"").expect("runs help");
        assert_eq!(help.status.code(), Some(0));
        let text = String::from_utf8_lossy(&help.stdout);
        assert!(text.contains("Usage: thinkthen runs <COMMAND>"), "{text}");
        for name in ["audit", "diff"] {
            assert!(
                text.lines()
                    .any(|line| line.split_whitespace().next() == Some(name)),
                "{text}"
            );
        }
    }
    for name in ["audit", "diff"] {
        let help = spawn(&["runs", name, "--help"], &[], b"").expect("run command help");
        assert_eq!(help.status.code(), Some(0));
        let text = String::from_utf8_lossy(&help.stdout);
        assert!(
            text.contains(&format!("Usage: thinkthen runs {name}")),
            "{text}"
        );
    }
}

#[test]
fn runs_requires_an_admitted_subcommand_without_sending() {
    let listener = crate::harness::Listener::serving(Vec::new()).expect("listener");
    for arguments in [vec!["runs"], vec!["runs", "list"]] {
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
        assert!(said.contains("Usage: thinkthen runs <COMMAND>"), "{said}");
        assert!(!said.contains("namespace-fake-key"), "{said}");
    }
    assert!(listener.requests().is_empty());
    assert_eq!(listener.connections(), 0);
}
