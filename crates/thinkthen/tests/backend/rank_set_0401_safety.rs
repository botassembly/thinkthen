//! Closed admission, conflicts and whole-call failures with counted sends.
use super::rank_set_0401::{SET, answer, call, saved};
use crate::harness::{Canned, Listener};
use crate::intake_0401::text;
use std::io;

#[test]
fn authored_cuts_and_on_are_refused_before_normalization_and_sends() -> io::Result<()> {
    let cases = [
        (
            r#"{"version":1,"questions":{}}"#,
            "`questions` holds at least one named question",
        ),
        (
            r#"{"version":2,"questions":{"first":{"decide":"secret wording"}}}"#,
            "`version` is the number 1",
        ),
        (
            r#"{"version":1,"threshold":0.5,"questions":{"first":{"decide":"secret wording"}}}"#,
            "`threshold`: rank question sets take no threshold or on",
        ),
        (
            r#"{"version":1,"questions":{"first":{"decide":"secret wording","threshold":0.5}}}"#,
            "`questions.first.threshold`: rank question sets take no threshold or on",
        ),
        (
            r#"{"version":1,"questions":{"first":{"decide":"secret wording","on":""}}}"#,
            "`questions.first.on`: rank question sets take no threshold or on",
        ),
        (
            r#"{"version":1,"questions":{"first":{"decide":"secret wording","on":[""]}}}"#,
            "`questions.first.on`: rank question sets take no threshold or on",
        ),
        (
            r#"{"version":1,"questions":{"first":{"decide":"secret wording","on":"/private"}}}"#,
            "`questions.first.on`: rank question sets take no threshold or on",
        ),
        (
            r#"{"version":1,"questions":{"first":{"score":"secret wording","levels":["low","high"]}}}"#,
            "`questions.first`: rank takes only decide questions",
        ),
        (
            r#"{"version":1,"questions":{"first":{"choose":"secret wording","options":["a","b"]}}}"#,
            "`questions.first`: rank takes only decide questions",
        ),
        (
            r#"{"version":1,"questions":{"first":{"decide":"secret wording","unknown":"secret value"}}}"#,
            "the question set holds no key `questions.first.unknown`",
        ),
        (
            r#"{"version":1,"questions":{"first":{"decide":"secret wording","model":"secret value"}}}"#,
            "the question set holds no key `questions.first.model`",
        ),
        (
            r#"{"version":1,"questions":{"first":{"decide":"secret wording","profile":"secret value"}}}"#,
            "the question set holds no key `questions.first.profile`",
        ),
        (
            r#"{"version":1,"questions":{"Bad":{"decide":"secret wording"}}}"#,
            "`questions.Bad` uses lowercase letters, digits, and underscores, and is not empty",
        ),
        (
            r#"{"version":1,"batch":0,"questions":{"first":{"decide":"secret wording"}}}"#,
            "`batch` in the question file takes max or a whole number of at least 1",
        ),
        (
            r#"{"version":1,"questions":{"first":"secret value"}}"#,
            "`questions.first` is one question object",
        ),
    ];
    let listener = Listener::answering(answer)?;
    for (index, (set, why)) in cases.into_iter().enumerate() {
        let question = saved(&format!("rank-set-invalid-{index}"), set)?;
        let output = call(&listener, &question, &[], b"secret evidence\n")?;
        assert_eq!(output.status.code(), Some(5), "{}", text(&output.stderr));
        assert_eq!(output.stderr, format!("thinkthen: {why}\n").as_bytes());
        assert!(output.stdout.is_empty());
        assert_eq!(listener.connections(), 0);
    }
    Ok(())
}

#[test]
fn set_owned_meanings_and_existing_display_conflicts_fail_without_sends() -> io::Result<()> {
    let question = saved("rank-set-conflicts", SET)?;
    let listener = Listener::answering(answer)?;
    for flags in [
        vec!["--true", "secret override"],
        vec!["--false", "secret override"],
        vec!["--true", "secret override", "--false", "secret override"],
    ] {
        let output = call(&listener, &question, &flags, b"secret evidence\n")?;
        assert_eq!(output.status.code(), Some(2));
        assert_eq!(output.stderr, b"thinkthen: `rank` with a question set takes no --true or --false; put meanings in each member\n");
        assert!(output.stdout.is_empty());
    }
    for flags in [
        vec!["--details", "--scores"],
        vec!["--window", "2", "--field", "/body"],
        vec!["--top", "0"],
        vec!["--quiet"],
        vec!["--raw"],
    ] {
        let output = call(&listener, &question, &flags, b"secret evidence\n")?;
        assert_eq!(output.status.code(), Some(2), "{}", text(&output.stderr));
        assert!(output.stdout.is_empty());
        assert!(!text(&output.stderr).contains("secret evidence"));
    }
    assert_eq!(listener.connections(), 0);
    Ok(())
}

#[test]
fn malformed_or_duplicate_sets_never_fall_back_and_file_load_is_capped() -> io::Result<()> {
    let listener = Listener::answering(answer)?;
    for (index, set) in [
        r#"{"version":1,"questions":{broken"#,
        r#"{"version":1,"questions":{"first":{"decide":"secret"},"first":{"decide":"secret"}}}"#,
        r#"{"version":1,"questions":{"first":{"decide":"secret","decide":"secret"}}}"#,
    ]
    .into_iter()
    .enumerate()
    {
        let question = saved(&format!("rank-set-json-{index}"), set)?;
        let output = call(&listener, &question, &[], b"secret evidence\n")?;
        assert_eq!(output.status.code(), Some(5));
        assert!(output.stdout.is_empty());
        assert!(!text(&output.stderr).contains("secret"));
    }
    let question = saved(
        "rank-set-file-cap",
        &format!("{SET}{}", " ".repeat(1_048_576)),
    )?;
    let output = call(&listener, &question, &[], b"secret evidence\n")?;
    assert_eq!(output.status.code(), Some(5), "{}", text(&output.stderr));
    assert!(output.stdout.is_empty());
    assert_eq!(listener.connections(), 0);
    Ok(())
}

#[test]
fn a_late_failed_member_discards_every_held_rank_and_keeps_secrets_out() -> io::Result<()> {
    let question = saved("rank-set-late-failure", SET)?;
    for view in [
        vec![],
        vec!["--details"],
        vec!["-n", "--scores", "--around", "1", "--top", "1"],
    ] {
        let listener = Listener::answering(|body| {
            if text(body).contains("late") {
                // q1 succeeds but q2 is absent; annotate permits this partial
                // member failure. Set rank must reject the whole result.
                Canned::ok(r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.9}}}"#)
            } else {
                answer(body)
            }
        })?;
        let flags = [&["--batch", "1"][..], &view].concat();
        let output = call(&listener, &question, &flags, b"a\nlate secret evidence\n")?;
        assert_eq!(output.status.code(), Some(4), "{}", text(&output.stderr));
        assert!(output.stdout.is_empty());
        assert_eq!(listener.requests().len(), 2);
        assert!(!text(&output.stderr).contains("secret evidence"));
    }
    Ok(())
}
