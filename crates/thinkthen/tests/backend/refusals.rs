//! Every way the command line and one record are refused, read for both markers.
//!
//! `secrecy.rs` drives the backend paths and owns the reader. This page drives
//! the refusals: every usage error the binary has, and every local failure a
//! command line can reach. A refusal enters by adding one row to `REFUSALS`,
//! and a command by adding one row to [`VERBS`].

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::harness::{Canned, Listener, spawn};

mod relate;
use crate::secrecy::{EVIDENCE, KEY, QUESTION, VERBS, asks_question, evidence, nothing_leaked};
use relate::{RELATE, own_sentence};

/// One refusal, driven over every verb that has it.
struct Refusal {
    named: &'static str,
    /// The verbs this refusal exists on. Empty means every verb.
    verbs: &'static [&'static str],
    /// The operands in place of the verb's own, or `None` for the verb's own.
    operands: Option<&'static [&'static str]>,
    /// What the command line adds. `{dir}` becomes this run's own folder.
    adds: &'static [&'static str],
    /// The question in place of the shared one. Empty keeps the shared one.
    question: &'static str,
    /// The evidence on standard input. Empty is the marker evidence, `{huge}`
    /// is the marker evidence padded past the record limit, and `{binary}` is
    /// the marker evidence followed by bytes that are not text.
    evidence: &'static str,
    /// The part of the message that names this refusal and no other.
    ///
    /// Without it a row could reach some other refusal at the same exit code
    /// and pass, which is how a table of refusals rots into a table of twos.
    says: &'static str,
    /// The exit code the refusal earns, which pins that the path really ran.
    code: i32,
}

/// One refusal on every verb, with the verb's own operands and question.
const fn every(
    named: &'static str,
    adds: &'static [&'static str],
    says: &'static str,
    code: i32,
) -> Refusal {
    Refusal {
        named,
        verbs: &[],
        operands: None,
        adds,
        question: "",
        evidence: "",
        says,
        code,
    }
}

/// One refusal on the verbs named, with the verb's own operands and question.
const fn only(
    named: &'static str,
    verbs: &'static [&'static str],
    adds: &'static [&'static str],
    says: &'static str,
    code: i32,
) -> Refusal {
    Refusal {
        verbs,
        ..every(named, adds, says, code)
    }
}

/// Every refusal a command line or one record can reach.
const REFUSALS: [Refusal; 33] = [
    every(
        "a base that is no address",
        &["--url", "ftp://127.0.0.1/v1"],
        "a base address begins with",
        2,
    ),
    every(
        "a base that is white space",
        &["--url", " "],
        "a URL is text, not white space",
        2,
    ),
    every(
        "a base carrying user information",
        &["--url", "https://a@b/v1"],
        "carries no user information",
        2,
    ),
    every(
        "a base with an empty port",
        &["--url", "http://127.0.0.1:/v1"],
        "a port is digits naming a number from 0 to 65535",
        2,
    ),
    every(
        "a base with a signed port",
        &["--url", "http://127.0.0.1:+80/v1"],
        "a port is digits naming a number from 0 to 65535",
        2,
    ),
    every(
        "a base with an out of range port",
        &["--url", "http://127.0.0.1:65536/v1"],
        "a port is digits naming a number from 0 to 65535",
        2,
    ),
    every(
        "a base carrying a query",
        &["--url", "https://127.0.0.1/v1?secret=value"],
        "a base address carries no query or fragment",
        2,
    ),
    every(
        "a base carrying a fragment",
        &["--url", "https://127.0.0.1/v1#secret"],
        "a base address carries no query or fragment",
        2,
    ),
    every(
        "a base under plain http",
        &["--url", "http://example.com/v1"],
        "sends the key across the network in clear text",
        2,
    ),
    every(
        "a blank model",
        &["--model", "  "],
        "a model name is text, not white space",
        2,
    ),
    every(
        "a model with a line break inside",
        &["--model", "jev\n1.13.0"],
        "a model name holds no control character or white space but a plain space",
        2,
    ),
    Refusal {
        question: " ",
        ..every(
            "a blank question",
            &[],
            "a question is text, not white space",
            2,
        )
    },
    Refusal {
        evidence: "   ",
        ..every("blank evidence", &[], "the evidence is empty or blank", 2)
    },
    Refusal {
        evidence: "{huge}",
        ..every(
            "a record over the limit",
            &[],
            "the record is over 16 MiB",
            2,
        )
    },
    Refusal {
        evidence: "{binary}",
        ..every(
            "a record that is not text",
            &[],
            "the evidence is not valid UTF-8",
            5,
        )
    },
    every(
        "a pointer in another language",
        &["--field", "$.body"],
        "a pointer is RFC 6901, so it is empty or begins with `/`",
        2,
    ),
    every(
        "a pointer beside lines",
        &["--lines", "--field", "/body"],
        "a text line has no members",
        2,
    ),
    every(
        "two pointers ending in one name",
        &["--jsonl", "--field", "/a/text", "--field", "/b/text"],
        "two pointers end in `text`",
        2,
    ),
    Refusal {
        evidence: "{\"other\":\"x\"}\n",
        ..every(
            "a pointer that finds nothing",
            &["--jsonl", "--field", "/body"],
            "the record holds nothing at `/body`",
            2,
        )
    },
    Refusal {
        evidence: "not json\n",
        ..every(
            "a record that is not JSON",
            &["--jsonl", "--field", "/body"],
            "the record is not valid JSON",
            2,
        )
    },
    every(
        "two recording folders",
        &["--record", "{dir}", "--replay", "{dir}2"],
        "name two different folders",
        2,
    ),
    every(
        "a plan beside a recording",
        &["--plan", "--record", "{dir}"],
        "--plan sends nothing",
        2,
    ),
    every(
        "a cache beside a recording",
        &["--cache", "{dir}", "--record", "{dir}"],
        "--cache is --record and --replay on one folder",
        2,
    ),
    // `relate` takes --jobs over its one entity set (ticket 0143).
    only(
        "jobs on one document",
        &["decide", "choose", "tag", "score", "recognize"],
        &["--jobs", "2"],
        "--jobs bounds the requests in flight",
        2,
    ),
    every(
        "an input file that is not there",
        &["--input", "{dir}/absent"],
        "--input could not be opened",
        5,
    ),
    only(
        "a threshold that is no threshold",
        &["decide", "choose"],
        &["--threshold", "nope"],
        "a threshold is a decimal fraction",
        2,
    ),
    only(
        "two views of one answer",
        &["decide", "choose"],
        &["--quiet", "--details"],
        "--quiet prints nothing",
        2,
    ),
    only(
        "a bare label beside another view",
        &["choose"],
        &["--raw", "--details"],
        "--raw prints a bare label",
        2,
    ),
    only(
        "a band on a pick",
        &["choose"],
        &["--threshold", "0.1:0.9"],
        "`choose` takes a single cut and never a band",
        2,
    ),
    only(
        "a rule on a placement",
        &["score"],
        &["--threshold", "0.5"],
        "`score` takes no rule",
        2,
    ),
    only(
        "raw on a decision",
        &["decide"],
        &["--raw"],
        "`decide` prints JSON; `choose --raw` prints a bare label",
        2,
    ),
    only(
        "raw on a score",
        &["score"],
        &["--raw"],
        "`score` prints a JSON number; `choose --raw` prints a bare label",
        2,
    ),
    only(
        "quiet on a score",
        &["score"],
        &["--quiet"],
        "`score` has no answer exit code, so --quiet would discard its result",
        2,
    ),
];

/// The refusals that need operands of their own, which the rows above keep.
const OPERANDS: [Refusal; 6] = [
    Refusal {
        operands: Some(&["one"]),
        ..only(
            "a pick of one option",
            &["choose"],
            &[],
            "`choose` takes 2 to 255 options",
            2,
        )
    },
    Refusal {
        operands: Some(&["late", "la\u{7}te"]),
        ..only(
            "a label holding a control character",
            &["choose"],
            &[],
            "an option is one line of printable text",
            2,
        )
    },
    Refusal {
        operands: Some(&["late", "  "]),
        ..only(
            "a label that is white space",
            &["choose"],
            &[],
            "an option is text, not white space",
            2,
        )
    },
    Refusal {
        operands: Some(&["none"]),
        ..only(
            "a placement on one level",
            &["score"],
            &[],
            "`score` takes 2 to 10 levels",
            2,
        )
    },
    Refusal {
        operands: Some(&["late", "lost"]),
        ..only(
            "a candidate list beside a pointer to one",
            &["choose"],
            &["--jsonl", "--options", "/codes"],
            "--options takes the options from each record",
            2,
        )
    },
    Refusal {
        operands: Some(&[]),
        ..only(
            "a pointer to a candidate list without jsonl",
            &["choose"],
            &["--options", "/codes"],
            "--options needs --jsonl",
            2,
        )
    },
];

/// The refusal a run carrying its answer in the exit code reaches over records.
const OVER_RECORDS: Refusal = Refusal {
    evidence: "{\"body\":\"x\"}\n",
    ..only(
        "an answer asked to set the code over records",
        &["decide", "choose"],
        &["--quiet", "--jsonl", "--field", "/body"],
        "--quiet carries the answer in the exit code",
        2,
    )
};

/// A folder this run owns, remade so each run starts empty.
fn folder(named: &str) -> io::Result<PathBuf> {
    let path = Path::new(env!("CARGO_TARGET_TMPDIR"))
        .join("refusals")
        .join(named);
    let _absent = fs::remove_dir_all(&path);
    fs::create_dir_all(&path)?;
    Ok(path)
}

/// The evidence one row sends, with the two computed spellings filled in.
fn evidence_of(named: &str) -> Vec<u8> {
    match named {
        "" => EVIDENCE.as_bytes().to_vec(),
        "{huge}" => {
            let mut bytes = EVIDENCE.as_bytes().to_vec();
            bytes.resize(crate::support::MAX_RECORD_BYTES + 1, b'x');
            bytes
        }
        "{binary}" => [EVIDENCE.as_bytes(), &[0xff, 0xfe]].concat(),
        "{entities}" => (0..256)
            .map(|place| format!("{EVIDENCE}-{place}\n"))
            .collect::<String>()
            .into_bytes(),
        text => text.as_bytes().to_vec(),
    }
}

/// Drive one refusal over one verb and read everything the run wrote.
fn refuse(refusal: &Refusal, verb: (&str, &[&str], &str), listener: &Listener) -> io::Result<()> {
    let (name, own, _) = verb;
    if (!refusal.verbs.is_empty() && !refusal.verbs.contains(&name))
        || (!asks_question(name) && refusal.named == "a blank question")
    {
        return Ok(());
    }
    let named = format!("{}-{name}", refusal.named.replace(' ', "-"));
    let into = folder(&named)?;
    let dir = into.join("recording");
    let filled = |argument: &&str| {
        (*argument)
            .replace("{dir}", &dir.to_string_lossy())
            .to_owned()
    };
    let operands = refusal.operands.unwrap_or(own);
    let question = if refusal.question.is_empty() {
        QUESTION
    } else {
        refusal.question
    };
    let mut asked = vec![name.to_owned()];
    if asks_question(name) {
        asked.push(question.to_owned());
    }
    asked.extend(operands.iter().map(|operand| (*operand).to_owned()));
    // A row that names its own base keeps it. Every other row points at the
    // listener, which answers nothing, so a row that stopped refusing would
    // fail on the count at the end rather than pass quietly.
    if !refusal.adds.contains(&"--url") {
        asked.extend(["--url".to_owned(), listener.base().to_owned()]);
    }
    asked.extend(refusal.adds.iter().map(filled));
    let arguments: Vec<&str> = asked.iter().map(String::as_str).collect();

    let output = spawn(
        &arguments,
        &[("THINKTHEN_API_KEY", KEY)],
        &if refusal.evidence.is_empty() {
            evidence(name, None)
        } else {
            evidence_of(refusal.evidence)
        },
    )?;

    assert_eq!(
        output.status.code(),
        Some(refusal.code),
        "{named}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        output.stdout.is_empty(),
        "{named}: a refusal prints nothing"
    );
    let message = String::from_utf8_lossy(&output.stderr);
    let says = own_sentence(name, refusal.named).unwrap_or(refusal.says);
    assert!(
        message.contains(says),
        "{named}: the message names another refusal\n{message}"
    );
    nothing_leaked(&named, &output, &into);
    Ok(())
}

/// Every refusal, over every verb that has it, sends nothing and leaks nothing.
///
/// One listener serves every case and answers nothing at all, so the count at
/// the end proves no refusal reached a backend.
#[test]
fn no_refusal_on_any_command_writes_the_key_quotes_the_evidence_or_sends_anything() {
    let listener = Listener::serving(Vec::<Canned>::new()).expect("a loopback listener");

    for refusal in REFUSALS
        .iter()
        .chain(&OPERANDS)
        .chain([&OVER_RECORDS])
        .chain(&RELATE)
    {
        for verb in VERBS {
            refuse(refusal, verb, &listener).expect("the compiled binary runs");
        }
    }

    assert!(
        listener.requests().is_empty(),
        "a refused command line opens no connection"
    );
    assert_eq!(listener.connections(), 0);
}
