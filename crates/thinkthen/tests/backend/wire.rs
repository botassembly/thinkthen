//! The bytes one request carries, pinned for each of the three question types.
//!
//! Ticket 0020 moved the adapter's name, its default address, its default
//! model, and its endpoint path into the adapter's module. Nothing there may
//! change a request by a byte, because the digest of these bytes names the
//! recording a run replays. Each case holds the body the tool sent before the
//! move, written out rather than encoded again, so a change in the encoder
//! cannot move the test with it.

use crate::harness::{Canned, Listener, spawn};

/// The evidence the first three cases send, as one line with no escape in it.
const EVIDENCE: &str = "Refund me please.";

/// Evidence that makes the JSON writer work.
///
/// It carries a quote, a line feed, a tab, a control byte, a letter with an
/// accent, and a dash outside ASCII. ADR 0017 calls the escaping the hardest
/// rule to hold still, because one language's JSON writer escapes every
/// character outside ASCII and another leaves it as it is. A digest is taken
/// over these bytes, so a recording is found again only while they stay put.
const HOSTILE: &str = "He said \"no\".\nCafé — 3 days.\t\u{1}";

/// The model every case names, so no default reaches the pinned bytes.
const MODEL: &str = "local-1";

/// A yes/no reply, enough for the run to read an answer and stop.
const ANSWERED: &str = concat!(
    r#"{"model":"jev-1.13.0","answers":{"q1":{"type":"noul","noul":0.92}},"#,
    r#""usage":{"input_tokens":312,"output_tokens":48}}"#,
);

/// A pick over the three options the `choose` case sends.
const PICKED: &str = concat!(
    r#"{"model":"jev-1.13.0","answers":{"q1":{"type":"choice","choice":"defect","#,
    r#""confidence":0.9,"probabilities":{"defect":0.9,"process":0.06,"other":0.04}}},"#,
    r#""usage":{"input_tokens":312,"output_tokens":48}}"#,
);

/// A placement over the three levels the `score` case sends.
const PLACED: &str = concat!(
    r#"{"model":"jev-1.13.0","answers":{"q1":{"type":"score","score":1.8,"#,
    r#""confidence":0.8,"probabilities":{"0":0.0,"1":0.2,"2":0.8}}},"#,
    r#""usage":{"input_tokens":312,"output_tokens":48}}"#,
);

/// The body a `decide` carrying both meanings sends.
const DECIDED_BODY: &str = concat!(
    r#"{"state":"Refund me please.","model":"local-1","questions":{"q1":{"type":"noul","#,
    r#""instructions":"asks for a refund","#,
    r#""criteria":{"true":"Money back.","false":"Anything else."}}}}"#,
);

/// The body a `choose` over three options sends.
const PICKED_BODY: &str = concat!(
    r#"{"state":"Refund me please.","model":"local-1","questions":{"q1":{"type":"choice","#,
    r#""instructions":"what kind","#,
    r#""criteria":{"defect":null,"process":null,"other":null}}}}"#,
);

/// The body a `score` over three levels sends.
const PLACED_BODY: &str = concat!(
    r#"{"state":"Refund me please.","model":"local-1","questions":{"q1":{"type":"score","#,
    r#""instructions":"how urgent","criteria":["low","medium","high"]}}}"#,
);

/// The body the hostile evidence sends, escape for escape.
///
/// The quote, the line feed, and the tab travel as two characters each and the
/// control byte as six. The accented letter and the dash travel as the bytes
/// they are, with no escape at all, which is the half of the rule a writer is
/// most likely to change.
const HOSTILE_BODY: &str = concat!(
    r#"{"state":"He said \"no\".\nCafé — 3 days.\t\u0001","model":"local-1","#,
    r#""questions":{"q1":{"type":"noul","instructions":"asks for a refund"}}}"#,
);

#[test]
fn each_question_type_sends_the_bytes_it_sent_before_the_adapter_owned_its_defaults() {
    let cases = [
        (
            &[
                "decide",
                "asks for a refund",
                "--true",
                "Money back.",
                "--false",
                "Anything else.",
            ][..],
            EVIDENCE,
            ANSWERED,
            DECIDED_BODY,
        ),
        (
            &["choose", "what kind", "defect", "process", "other"][..],
            EVIDENCE,
            PICKED,
            PICKED_BODY,
        ),
        (
            &["score", "how urgent", "low", "medium", "high"][..],
            EVIDENCE,
            PLACED,
            PLACED_BODY,
        ),
        (
            &["decide", "asks for a refund"][..],
            HOSTILE,
            ANSWERED,
            HOSTILE_BODY,
        ),
    ];

    for (asked, evidence, response, expected) in cases {
        let listener = Listener::serving(vec![Canned::ok(response)]).expect("a loopback listener");
        let named = ["--url", listener.base(), "--model", MODEL];

        let output = spawn(
            &[asked, &named[..]].concat(),
            &[("THINKTHEN_API_KEY", "sk-test-value")],
            evidence.as_bytes(),
        )
        .expect("the compiled binary runs");

        let requests = listener.requests();
        assert_eq!(requests.len(), 1, "{asked:?}");
        let request = requests.first().expect("one request reached the listener");
        assert_eq!(request.line, "POST /v1/systemone HTTP/1.1", "{asked:?}");
        // The text is compared first, so a failure reads as a sentence. The
        // bytes are compared after it, because the digest is taken over bytes
        // and a lossy reading would hide a byte that is not text.
        assert_eq!(
            String::from_utf8_lossy(&request.body),
            expected,
            "{asked:?}"
        );
        assert_eq!(request.body, expected.as_bytes(), "{asked:?}");
        assert_eq!(output.status.code(), Some(0), "{asked:?}");
    }
}
