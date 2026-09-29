//! Refusals at the compiled command boundary.
#![cfg(feature = "cli")]

use std::ffi::{OsStr, OsString};
use std::io::{self, Write};
use std::process::{Command, Output, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};

#[path = "../src/test_deadline/wait.rs"]
mod wait;

fn run(args: &[&OsStr], input: &[u8]) -> io::Result<Output> {
    static RUNS: AtomicUsize = AtomicUsize::new(0);
    let home = std::path::Path::new(env!("CARGO_TARGET_TMPDIR"))
        .join(format!("hints-{}", RUNS.fetch_add(1, Ordering::Relaxed)));
    let mut child = Command::new(env!("CARGO_BIN_EXE_thinkthen"))
        .env_clear()
        .env("HOME", home)
        .env("THINKTHEN_BASE_URL", "http://127.0.0.1:1/v1")
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let mut stdin = child
        .stdin
        .take()
        .ok_or_else(|| io::Error::other("stdin pipe"))?;
    let _ = stdin.write_all(input);
    drop(stdin);
    wait::finish(child, "hints")
}

#[expect(clippy::expect_used, reason = "the CLI must start")]
fn check(args: &[&str], input: &[u8], stderr: &str) {
    let args: Vec<&OsStr> = args.iter().map(OsStr::new).collect();
    let output = run(&args, input).expect("binary runs");
    assert_eq!(output.status.code(), Some(2), "{args:?}");
    assert!(output.stdout.is_empty(), "{args:?}");
    assert_eq!(String::from_utf8_lossy(&output.stderr), stderr, "{args:?}");
}

#[test]
fn a_guessed_verb_names_the_function_that_does_the_job() {
    const PICK: &str = "`choose` picks one option, and `tag` names every label that fits";
    const WRITE: &str = "thinkthen judges text and writes none";
    let cases = [
        ("grep", "`filter` keeps the records where the answer is yes"),
        (
            "if",
            "`decide` answers one yes or no question in its exit code",
        ),
        ("classify", PICK),
        ("switch", PICK),
        (
            "sort",
            "`rank` sorts records by how likely the answer is yes",
        ),
        ("summarize", WRITE),
        ("rewrite", WRITE),
    ];
    for (word, tail) in cases {
        check(
            &[word, "x"],
            b"",
            &format!("thinkthen: `{word}` is not a command; {tail}\n"),
        );
    }
    for args in [&["grep"][..], &["help", "grep"][..]] {
        check(
            args,
            b"",
            "thinkthen: `grep` is not a command; `filter` keeps the records where the answer is yes\n",
        );
    }
    for word in ["Grep", "gerp", "split"] {
        check(
            &[word, "x"],
            b"",
            &format!(
                "error: unrecognized subcommand '{word}'\n\nUsage: thinkthen [COMMAND]\n\nFor more information, try '--help'.\n"
            ),
        );
    }
}

#[test]
fn a_second_argument_says_where_the_evidence_goes() {
    const MESSAGE: &str = "thinkthen: the question is one argument and each option takes one value; quote a question of several words, and send evidence on standard input or as `--input FILE`\n";
    for verb in ["decide", "filter", "rank", "find"] {
        check(&[verb, "Q?", "README.md", "--no-cache"], b"", MESSAGE);
    }
    for args in [
        &["decide", "Is", "this", "urgent"][..],
        &["decide", "Q?", "--jsonl", "--field", "/a", "/b"],
        &["decide", "Q?", "--input", "a.txt", "b.txt"],
    ] {
        check(args, b"", MESSAGE);
    }
    #[cfg(unix)]
    {
        use std::os::unix::ffi::OsStringExt as _;
        let invalid = OsString::from_vec(vec![0xff, 0xfe]);
        let args = [
            OsStr::new("decide"),
            OsStr::new("Q?"),
            invalid.as_os_str(),
            OsStr::new("--no-cache"),
        ];
        let output = run(&args, b"").expect("binary runs");
        assert_eq!(output.status.code(), Some(2));
        assert!(output.stdout.is_empty());
        assert_eq!(String::from_utf8_lossy(&output.stderr), MESSAGE);
    }
}

#[test]
fn a_table_fed_to_jsonl_names_csv_and_lines() {
    const HINT: &str = "thinkthen: the record is not valid JSON; read a table with `--csv` or `--tsv`, and plain text with `--lines`\n";
    const PLAIN: &str = "thinkthen: the record is not valid JSON\n";
    for verb in ["filter", "find", "rank"] {
        let mut expected = HINT.to_owned();
        expected.push_str("thinkthen: stopped at record 1; 0 records finished");
        if verb == "rank" {
            expected.push_str(", and nothing was printed because an order needs every record");
        }
        expected.push('\n');
        check(
            &[verb, "Q?", "--jsonl", "--no-cache"],
            b"name,value\n\"a,b\",c\n",
            &expected,
        );
    }
    check(
        &["find", "Q?", "--jsonl", "--plan", "--no-cache"],
        b"name,value\n\"a,b\",c\n",
        &format!("{HINT}thinkthen: stopped at record 1; 0 records finished\n"),
    );
    check(
        &["find", "Q?", "--jsonl", "--plan", "--no-cache"],
        b"123\nnot-json\n",
        &format!("{PLAIN}thinkthen: stopped at record 2; 0 records finished\n"),
    );
}

#[test]
fn choose_help_names_tag() {
    let output = run(&[OsStr::new("choose"), OsStr::new("--help")], b"").expect("binary runs");
    assert_eq!(output.status.code(), Some(0));
    assert!(output.stderr.is_empty());
    assert!(
        String::from_utf8_lossy(&output.stdout)
            .lines()
            .any(|line| line == "Use `tag` when more than one answer can apply.")
    );
}
