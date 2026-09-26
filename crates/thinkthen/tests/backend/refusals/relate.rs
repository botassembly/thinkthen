//! The refusal rows and sentences of `relate`, whose complete-set grammar differs.

use super::{Refusal, only};

/// The complete-set refusals only `relate` has, each before any send.
pub(super) const RELATE: [Refusal; 10] = [
    Refusal {
        evidence: r#"[{"name":"marker-evidence-7b3ac5","kind":"record"},{"name":"marker-evidence-7b3ac5","kind":"record"}]"#,
        ..only(
            "a duplicate entity",
            &["relate"],
            &[],
            "the complete entity set contains no duplicate name-and-kind identity",
            2,
        )
    },
    Refusal {
        operands: Some(&["linked=record:person"]),
        ..only(
            "an absent concrete kind",
            &["relate"],
            &[],
            "a concrete relation kind is absent from the complete entity set",
            2,
        )
    },
    Refusal {
        operands: Some(&["linked"]),
        evidence: "{entities}",
        ..only(
            "a 256th entity",
            &["relate"],
            &["--lines"],
            "relate takes at most 255 entities",
            2,
        )
    },
    Refusal {
        operands: Some(&["linked"]),
        evidence: "marker-evidence-7b3ac5\n\nAcme\n",
        ..only(
            "a blank entity line",
            &["relate"],
            &["--lines"],
            "an entity name and kind are nonempty strings",
            2,
        )
    },
    Refusal {
        operands: Some(&["linked=record:record"]),
        evidence: "marker-evidence-7b3ac5\nAcme\n",
        ..only(
            "a typed rule over lines",
            &["relate"],
            &["--lines"],
            "--lines takes only bare relation names or NAME=*:*",
            2,
        )
    },
    Refusal {
        evidence: r#"[{"name":7,"kind":"marker-evidence-7b3ac5"}]"#,
        ..only(
            "an entity name that is not text",
            &["relate"],
            &[],
            "the entity value at `/name` is not a string",
            2,
        )
    },
    Refusal {
        operands: Some(&["linked=record"]),
        ..only(
            "a relation rule with one kind",
            &["relate"],
            &[],
            "a relate relation is NAME=SOURCE_KIND:TARGET_KIND, or a bare NAME",
            2,
        )
    },
    Refusal {
        operands: Some(&["linked:record"]),
        ..only(
            "a bare relation name holding a colon",
            &["relate"],
            &[],
            "a relate relation is NAME=SOURCE_KIND:TARGET_KIND, or a bare NAME",
            2,
        )
    },
    Refusal {
        operands: Some(&["linked", "@question.json"]),
        ..only(
            "a question file beside an inline rule",
            &["relate"],
            &[],
            "relate takes inline relation rules or one @FILE, never both",
            2,
        )
    },
    Refusal {
        operands: Some(&["@question.json"]),
        ..only(
            "either beside a question file",
            &["relate"],
            &["--either"],
            "`--either` applies only to inline relation rules; a question file sets either on each relation",
            2,
        )
    },
];

/// The sentence a verb gives in place of a shared row's, where its grammar differs.
pub(super) fn own_sentence(verb: &str, row: &str) -> Option<&'static str> {
    Some(match (verb, row) {
        ("recognize" | "relate", "a blank model") => "--model is text, not white space",
        ("relate", "blank evidence") => "the input is not valid JSON",
        ("relate", "a pointer in another language") => {
            "relate fields are RFC 6901 pointers named `name` and `kind`"
        }
        ("relate", "a pointer beside lines") => "--lines takes neither --field nor --kind-field",
        ("relate", "two pointers ending in one name") => "--field takes one pointer on `relate`",
        ("relate", "jobs on one document") => {
            "`relate` sends its requests in order, so it takes no --jobs"
        }
        _ => return None,
    })
}
