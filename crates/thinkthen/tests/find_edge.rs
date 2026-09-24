//! The public find boundary over free plans and committed recordings.

use std::fs;
use std::io;
use std::io::Write as _;
use std::path::PathBuf;
use std::process::{Command, Output, Stdio};

fn run(arguments: &[&str], input: &[u8]) -> io::Result<Output> {
    let mut child = Command::new(env!("CARGO_BIN_EXE_thinkthen"))
        .env_clear()
        .env("HOME", env!("CARGO_TARGET_TMPDIR"))
        .args(arguments)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    child
        .stdin
        .take()
        .ok_or_else(|| io::Error::other("no stdin"))?
        .write_all(input)?;
    child.wait_with_output()
}

fn status_without_output(arguments: &[&str], input: &[u8]) -> io::Result<std::process::ExitStatus> {
    let mut child = Command::new(env!("CARGO_BIN_EXE_thinkthen"))
        .env_clear()
        .env("HOME", env!("CARGO_TARGET_TMPDIR"))
        .args(arguments)
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;
    child
        .stdin
        .take()
        .ok_or_else(|| io::Error::other("no stdin"))?
        .write_all(input)?;
    child.wait()
}

fn lines(case: &str, size: usize, answer: Option<usize>) -> Vec<u8> {
    let mut input = String::new();
    for place in 1..=size {
        if answer == Some(place) {
            input.push_str(&format!(
                "The cobalt permit duration for marker TARGET-{} is {} days.\n",
                case.to_uppercase(),
                20 + place % 37
            ));
        } else {
            input.push_str(&format!(
                "Marker OTHER-{}-{place:03} uses an amber checklist reviewed every {} days.\n",
                case.to_uppercase(),
                2 + place % 19
            ));
        }
    }
    input.into_bytes()
}

fn recording() -> String {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../probes/find-0040/recording")
        .to_string_lossy()
        .into_owned()
}

#[test]
fn dry_run_builds_one_aggregate_request_and_names_the_framing() {
    let output = run(&["find", "Which?", "--dry-run"], b"alpha\nbeta\n").expect("binary runs");
    assert_eq!(output.status.code(), Some(0));
    let plan = String::from_utf8_lossy(&output.stdout);
    assert!(
        plan.contains(r#""input":{"framing":"lines","field":[]}"#),
        "{plan}"
    );
    assert!(plan.contains(r#""type":"choice""#), "{plan}");
    assert!(plan.contains(r#""state":"[{\"id\":\"u001\",\"evidence\":\"alpha\"},{\"id\":\"u002\",\"evidence\":\"beta\"}]""#), "{plan}");
}

#[test]
fn help_leads_with_whole_set_disclosure_and_all_three_bounds() {
    for flag in ["-h", "--help"] {
        let output = run(&["find", flag], b"").expect("binary runs");
        assert_eq!(output.status.code(), Some(0));
        let help = String::from_utf8_lossy(&output.stdout);
        assert!(help.contains("Every line or record leaves together and sees every other one"));
        if flag == "--help" {
            assert!(help.contains("2 to 255 lines or records, or 2 to 254 with --none"));
            assert!(help.contains("at most 16 MiB across the original input"));
            assert!(help.contains(
                "`find --none` prints nothing and exits 3 when `none` wins or ties for first."
            ));
            for cache_rule in [
                "cached by default in the platform cache folder",
                "Entries contain the judged text",
                "overriding THINKTHEN_CACHE and the platform default",
                "An explicit replay folder suppresses the platform default cache",
            ] {
                assert!(help.contains(cache_rule), "{cache_rule}\n{help}");
            }
        }
        for accepted in [
            "--lines",
            "--jsonl",
            "--field",
            "--details",
            "--input",
            "--url",
        ] {
            assert!(help.contains(accepted), "{flag}: {accepted}\n{help}");
        }
        for refused in ["--csv", "--tsv", "--jobs", "--threshold", "--raw"] {
            assert!(!help.contains(refused), "{flag}: {refused}\n{help}");
        }
    }
}

#[test]
fn count_refusals_are_exact_and_need_no_key() {
    let cases = [
        (
            vec!["find", "Which?"],
            b"one\n".as_slice(),
            "thinkthen: `find` takes 2 to 255 units\n",
        ),
        (
            vec!["find", "Which?", "--none"],
            b"one\n".as_slice(),
            "thinkthen: `find --none` takes 2 to 254 units\n",
        ),
    ];
    for (arguments, input, message) in cases {
        let output = run(&arguments, input).expect("binary runs");
        assert_eq!(output.status.code(), Some(2));
        assert!(output.stdout.is_empty());
        assert_eq!(String::from_utf8_lossy(&output.stderr), message);
    }
}

#[test]
fn table_and_jobs_options_are_absent_from_the_find_parser() {
    for (option, arguments) in [
        ("--csv", vec!["find", "Which?", "--csv"]),
        ("--tsv", vec!["find", "Which?", "--tsv"]),
        ("--jobs", vec!["find", "Which?", "--jobs", "2"]),
    ] {
        let output = run(&arguments, b"one\ntwo\n").expect("binary runs");
        assert_eq!(output.status.code(), Some(2));
        assert!(output.stdout.is_empty());
        assert_eq!(
            String::from_utf8_lossy(&output.stderr),
            format!(
                "error: unexpected argument '{option}' found\n\n  tip: to pass '{option}' as a value, use '-- {option}'\n\nUsage: thinkthen find <QUESTION>\n\nFor more information, try '--help'.\n"
            )
        );
    }
}

#[test]
fn empty_input_succeeds_without_reading_a_key() {
    let output = run(&["find", "Which unit answers?"], b"").expect("binary runs");
    assert_eq!(output.status.code(), Some(0));
    assert!(output.stdout.is_empty());
    assert!(output.stderr.is_empty());
}

#[test]
fn committed_reach_recordings_replay_the_selected_line_and_none() {
    let folder = recording();
    let answerable_question =
        "Which unit states the cobalt permit duration for marker TARGET-S100-A1?";
    let answerable = run(
        &["find", answerable_question, "--replay", &folder],
        &lines("s100-a1", 100, Some(7)),
    )
    .expect("binary runs");
    assert_eq!(answerable.status.code(), Some(0));
    assert_eq!(
        String::from_utf8_lossy(&answerable.stdout),
        "The cobalt permit duration for marker TARGET-S100-A1 is 27 days.\n"
    );
    let blank_question = "Which unit states the cobalt permit duration for marker TARGET-S100-B1?";
    let blank = run(
        &["find", blank_question, "--none", "--replay", &folder],
        &lines("s100-b1", 100, None),
    )
    .expect("binary runs");
    assert_eq!(blank.status.code(), Some(3));
    assert!(blank.stdout.is_empty());
}

#[test]
fn aggregate_byte_limit_refuses_one_byte_over_before_a_key() {
    let half = 16 * 1024 * 1024 / 2;
    let mut input = vec![b'a'; half - 1];
    input.push(b'\n');
    input.extend(std::iter::repeat_n(b'b', half));
    input.push(b'\n');
    let output = run(&["find", "Which?"], &input).expect("binary runs");
    assert_eq!(output.status.code(), Some(2));
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        "thinkthen: `find` reads at most 16 MiB across all units\n"
    );
}

#[test]
fn aggregate_byte_limit_accepts_the_exact_boundary() {
    let half = 16 * 1024 * 1024 / 2;
    let mut input = vec![b'a'; half - 1];
    input.push(b'\n');
    input.extend(std::iter::repeat_n(b'b', half));
    let status =
        status_without_output(&["find", "Which?", "--dry-run"], &input).expect("binary runs");
    assert_eq!(status.code(), Some(0));
}

#[test]
fn compiled_count_boundaries_accept_and_refuse_the_exact_edges() {
    let many = |count: usize| {
        (0..count)
            .map(|place| format!("unit {place}\n"))
            .collect::<String>()
    };
    for (count, none) in [(255, false), (254, true)] {
        let mut arguments = vec!["find", "Which?", "--dry-run"];
        if none {
            arguments.push("--none");
        }
        let output = run(&arguments, many(count).as_bytes()).expect("binary runs");
        assert_eq!(output.status.code(), Some(0), "{count} {none}");
    }
    for (count, none, message) in [
        (256, false, "thinkthen: `find` takes 2 to 255 units\n"),
        (255, true, "thinkthen: `find --none` takes 2 to 254 units\n"),
    ] {
        let mut arguments = vec!["find", "Which?"];
        if none {
            arguments.push("--none");
        }
        let output = run(&arguments, many(count).as_bytes()).expect("binary runs");
        assert_eq!(output.status.code(), Some(2));
        assert_eq!(String::from_utf8_lossy(&output.stderr), message);
    }

    let mut stops_at_256 = many(255).into_bytes();
    stops_at_256.extend_from_slice(b"unit 256\nPRIVATE\xff");
    let output = run(&["find", "Which?"], &stops_at_256).expect("binary runs");
    assert_eq!(output.status.code(), Some(2));
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        "thinkthen: `find` takes 2 to 255 units\n"
    );
}

#[test]
fn syntax_and_no_recording_utf8_refusals_repeat_no_input() {
    let marker = "PRIVATE-FIND-MARKER";
    let invalid_json = run(
        &["find", "Which?", "--jsonl", "--field", "/body"],
        format!("{{\"body\":\"safe\"}}\n{{{marker}\n").as_bytes(),
    )
    .expect("binary runs");
    assert_eq!(invalid_json.status.code(), Some(2));
    assert!(!String::from_utf8_lossy(&invalid_json.stderr).contains(marker));
    let invalid_utf8 = run(&["find", "Which?"], b"first\nsecond\xff\n").expect("binary runs");
    assert_eq!(invalid_utf8.status.code(), Some(5));
    assert_eq!(
        String::from_utf8_lossy(&invalid_utf8.stderr),
        concat!(
            "thinkthen: the record is not valid UTF-8\n",
            "thinkthen: stopped at record 2; 0 records finished\n",
        )
    );
}

#[test]
fn a_named_recording_appears_in_a_find_preflight_stop() {
    let folder = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("find-stopped-recording");
    let _removed = fs::remove_dir_all(&folder);
    fs::create_dir_all(&folder).expect("recording directory");
    let folder = folder.to_string_lossy();
    let output = run(
        &["find", "Which?", "--replay", &folder],
        b"first\nsecond\xff\n",
    )
    .expect("binary runs");
    assert_eq!(output.status.code(), Some(5));
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        concat!(
            "thinkthen: the record is not valid UTF-8\n",
            "thinkthen: stopped at record 2; 0 records finished, 0 records from a recording\n",
        )
    );
}

#[test]
fn input_file_and_directory_follow_the_shared_opened_handle_rules() {
    let root = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("find-input");
    let _removed = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).expect("directory");
    let file = root.join("units.txt");
    fs::write(&file, b"first\nsecond\n").expect("input file");
    let file_text = file.to_string_lossy();
    let accepted = run(
        &["find", "Which?", "--dry-run", "--input", &file_text],
        b"ignored",
    )
    .expect("file runs");
    assert_eq!(accepted.status.code(), Some(0));
    assert!(String::from_utf8_lossy(&accepted.stdout).contains(r#"\"evidence\":\"second\""#));

    let root_text = root.to_string_lossy();
    let refused = run(
        &["find", "Which?", "--dry-run", "--input", &root_text],
        b"ignored",
    )
    .expect("directory is refused");
    assert_eq!(refused.status.code(), Some(5));
    assert_eq!(
        String::from_utf8_lossy(&refused.stderr),
        "thinkthen: `--input` names a directory, and a directory is not an input file\n"
    );
}

#[test]
fn recorder_option_conflict_is_validated_before_empty_input_returns() {
    let output = run(
        &[
            "find",
            "Which?",
            "--record",
            "record-here",
            "--replay",
            "replay-there",
        ],
        b"",
    )
    .expect("binary runs");
    assert_eq!(output.status.code(), Some(2));
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        "thinkthen: --record and --replay name two different folders, and one run keeps one\n"
    );
}

#[test]
fn one_oversized_unit_gets_the_safe_aggregate_diagnostic() {
    let input = vec![b'x'; 16 * 1024 * 1024 + 1];
    let output = run(&["find", "Which?"], &input).expect("binary runs");
    assert_eq!(output.status.code(), Some(2));
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        "thinkthen: `find` reads at most 16 MiB across all units\n"
    );
}
