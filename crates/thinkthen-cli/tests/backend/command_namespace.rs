//! Public command names and reserved nouns at the compiled boundary.

use crate::harness::spawn;

#[test]
fn help_advertises_backend_checks_and_omits_the_retired_name() {
    let help = spawn(&["--help"], &[], b"").expect("help runs");
    assert_eq!(help.status.code(), Some(0));
    let text = crate::harness::printed(&help);
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
    assert!(crate::harness::printed(&help).contains("Usage: thinkthen backends check"));
}

#[test]
fn retired_and_reserved_commands_refuse_before_setup_without_sending() {
    let listener = crate::harness::Listener::serving(Vec::new()).expect("listener");
    for noun in [
        "check",
        "audit",
        "diff",
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
        for arguments in [vec![noun], vec![noun, "--help"], vec!["help", noun]] {
            let output = spawn(
                &arguments,
                &[
                    ("THINKTHEN_BASE_URL", listener.base()),
                    ("THINKTHEN_API_KEY", "namespace-fake-key"),
                    ("THINKTHEN_BACKEND", "invalid-namespace-backend"),
                ],
                b"",
            )
            .expect("binary runs");
            assert_eq!(output.status.code(), Some(2), "{arguments:?}");
            assert!(output.stdout.is_empty(), "{arguments:?}");
            let said = crate::harness::said(&output);
            assert!(
                said.contains(&format!("unrecognized subcommand '{noun}'")),
                "{said}"
            );
            assert!(!said.contains("namespace-fake-key"), "{said}");
            assert!(!said.contains("invalid-namespace-backend"), "{said}");
        }
    }
    assert!(listener.requests().is_empty());
    assert_eq!(listener.connections(), 0);
}

#[test]
fn help_advertises_runs_and_omits_the_retired_audit_and_diff_names() {
    let help = spawn(&["--help"], &[], b"").expect("help runs");
    assert_eq!(help.status.code(), Some(0));
    let text = crate::harness::printed(&help);
    for name in ["runs", "audit", "diff"] {
        let listed = text
            .lines()
            .any(|line| line.split_whitespace().next() == Some(name));
        assert_eq!(listed, name == "runs", "{text}");
    }
    for arguments in [vec!["runs", "--help"], vec!["help", "runs"]] {
        let help = spawn(&arguments, &[], b"").expect("runs help");
        assert_eq!(help.status.code(), Some(0));
        let text = crate::harness::printed(&help);
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
        let text = crate::harness::printed(&help);
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
        let said = crate::harness::said(&output);
        assert!(said.contains("Usage: thinkthen runs <COMMAND>"), "{said}");
        assert!(!said.contains("namespace-fake-key"), "{said}");
    }
    assert!(listener.requests().is_empty());
    assert_eq!(listener.connections(), 0);
}
