//! Literal portable spelling and cut oracle beside the shared batch planner.

use serde::Deserialize;
use sha2::{Digest as _, Sha256};

use super::{BatchRecord, Batcher, LOOPBACK, Setting, backend, decide, run, text};
use crate::core::digest::hex;
use crate::core::json::Json;
use crate::core::render::json_line;
use crate::core::text::Evidence;

const CORPUS: &str =
    include_str!("../../../../../../specification/fixtures/batching/portable-records.json");
const BODIES: [&str; 3] = [
    include_str!("../../../../../../specification/fixtures/batching/portable-1.request.json"),
    include_str!("../../../../../../specification/fixtures/batching/portable-2.request.json"),
    include_str!("../../../../../../specification/fixtures/batching/portable-3.request.json"),
];

#[derive(Deserialize)]
struct PortableFixture {
    compact: Vec<String>,
    heads: Vec<String>,
    cut: Vec<bool>,
    members: Vec<Vec<usize>>,
    closed: Vec<String>,
    digests: Vec<String>,
}

#[derive(Deserialize)]
struct TextFixture {
    texts: Vec<String>,
    #[serde(flatten)]
    expected: PortableFixture,
}

#[test]
fn selected_text_spelling_sets_the_portable_cuts_and_full_requests() {
    let corpus: TextFixture = serde_json::from_str(CORPUS).expect("literal corpus");
    let texts = &corpus.texts;
    let PortableFixture {
        compact,
        heads,
        cut: cuts,
        members: expected_members,
        closed,
        digests,
    } = &corpus.expected;
    assert_eq!(texts.len(), 5);
    assert_eq!((compact.len(), heads.len(), cuts.len()), (5, 5, 5));
    for (at, value) in texts.iter().enumerate() {
        let selected = text(value);
        let bytes = json_line(&selected.value).expect("selected JSON");
        assert_eq!(bytes, compact[at]);
        let hash = Sha256::digest(bytes.as_bytes());
        assert_eq!(hex(&hash[..8]), heads[at]);
        assert_eq!(hash[6] & 0x0f == 0 && hash[7] == 0, cuts[at]);
    }

    let records = texts.iter().map(|value| text(value)).collect();
    let planned = run(
        Batcher::new(
            backend(LOOPBACK, "jev-1.13.0"),
            None,
            decide("Is it relevant?"),
            Setting::Max,
            None,
        )
        .expect("batcher"),
        records,
    )
    .expect("planned batches");
    assert_eq!(planned.len(), 3);
    let mut next = 0;
    for (at, batch) in planned.iter().enumerate() {
        let members: Vec<usize> = (next..next + batch.questions.len()).collect();
        next += members.len();
        assert_eq!(members, expected_members[at]);
        assert_eq!(
            format!("{:?}", batch.closed).to_ascii_lowercase(),
            closed[at]
        );
    }
    assert_eq!(next, 5);
    for (batch, literal) in planned.iter().zip(BODIES) {
        assert_eq!(
            batch.body,
            literal
                .strip_suffix('\n')
                .expect("one fixture newline")
                .as_bytes()
        );
    }
    for (batch, digest) in planned.iter().zip(digests) {
        assert_eq!(batch.digest.as_str(), digest);
    }

    let escaped = Sha256::digest(br#""caf\u00e9-5544""#);
    assert_eq!(hex(&escaped[..8]), "72afc19814d82dd9");
    assert!(!(escaped[6] & 0x0f == 0 && escaped[7] == 0));
}

#[test]
fn structured_selected_values_keep_order_and_numeric_kinds() {
    let oracle: PortableFixture = serde_json::from_str(include_str!(
        "../../../../../../specification/fixtures/batching/portable-structured.json"
    ))
    .expect("structured oracle");
    let input =
        include_str!("../../../../../../specification/fixtures/batching/portable-structured.jsonl");
    let rows: Vec<&str> = input.lines().collect();
    assert_eq!(rows.len(), 3);
    let mut records = Vec::new();
    for (at, raw) in rows.iter().enumerate() {
        let selected = Json::parse(raw).expect("structured record");
        let compact = json_line(&selected).expect("compact record");
        assert_eq!(compact, oracle.compact[at]);
        let hash = Sha256::digest(compact.as_bytes());
        assert_eq!(hex(&hash[..8]), oracle.heads[at]);
        assert_eq!(hash[6] & 0x0f == 0 && hash[7] == 0, oracle.cut[at]);
        records.push(BatchRecord {
            evidence: Evidence::new(&compact).expect("evidence"),
            value: selected,
        });
    }
    let planned = run(
        Batcher::new(
            backend(LOOPBACK, "jev-1.13.0"),
            None,
            decide("Is it relevant?"),
            Setting::Max,
            None,
        )
        .expect("batcher"),
        records,
    )
    .expect("structured batches");
    assert_eq!(planned.len(), 2);
    let bodies = [
        include_str!(
            "../../../../../../specification/fixtures/batching/portable-structured-1.request.json"
        ),
        include_str!(
            "../../../../../../specification/fixtures/batching/portable-structured-2.request.json"
        ),
    ];
    let mut next = 0;
    for (at, (batch, literal)) in planned.iter().zip(bodies).enumerate() {
        let members: Vec<usize> = (next..next + batch.questions.len()).collect();
        next += members.len();
        assert_eq!(members, oracle.members[at]);
        assert_eq!(
            format!("{:?}", batch.closed).to_ascii_lowercase(),
            oracle.closed[at]
        );
        assert_eq!(
            batch.body,
            literal
                .strip_suffix('\n')
                .expect("fixture newline")
                .as_bytes()
        );
        assert_eq!(batch.digest.as_str(), oracle.digests[at]);
    }
    assert_eq!(next, 3);
}
