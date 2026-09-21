//! The canonical form of one question, and the digest that names it.
//!
//! A row carries `meta.question_sha256`, so a reader can tell two runs apart
//! that asked almost the same thing. The form the digest is taken over is
//! written out in `specification/question-file.md`, so another implementation
//! of this tool reaches the same sixty-four figures.

use serde::ser::SerializeMap as _;
use serde::{Serialize, Serializer};
use sha2::{Digest as _, Sha256};

use crate::core::question::{Labels, Question};
use crate::core::render::{RenderError, json_line};
use crate::core::threshold::Threshold;

/// Write bytes as lowercase hexadecimal.
///
/// One writer serves the recording key and the question digest, so the two
/// kinds of name are spelled the same way.
pub(crate) fn hex(bytes: &[u8]) -> String {
    bytes
        .iter()
        .flat_map(|byte| [byte >> 4, byte & 0x0f])
        .map(nibble)
        .collect()
}

/// Write one half of a byte as its hexadecimal figure.
fn nibble(value: u8) -> char {
    char::from(if value < 10 {
        b'0' + value
    } else {
        b'a' + value - 10
    })
}

/// The SHA-256 of the canonical form of this question, in lowercase hex.
///
/// The threshold rides with the question, because a cut tuned on labeled cases
/// belongs to the question it was tuned for. `threshold` is `None` on a verb
/// that takes no rule, and the key is then absent from the canonical form.
///
/// # Errors
///
/// Returns [`RenderError`] when the canonical form cannot be written as JSON.
pub(crate) fn question_sha256(
    question: &Question,
    threshold: Option<Threshold>,
) -> Result<String, RenderError> {
    let canonical = json_line(&Canonical {
        question,
        threshold,
    })?;
    let mut hasher = Sha256::new();
    hasher.update(canonical.as_bytes());
    Ok(hex(&hasher.finalize()))
}

/// One question in the form the digest is taken over.
///
/// It is not the `question` field of a result. That field prints a pick's
/// options as the list of names a reader wants to see, and the digest has to
/// separate two runs whose options carry different descriptions.
pub(crate) struct Canonical<'a> {
    question: &'a Question,
    threshold: Option<Threshold>,
}

impl<'a> Canonical<'a> {
    pub(crate) const fn new(question: &'a Question, threshold: Option<Threshold>) -> Self {
        Self {
            question,
            threshold,
        }
    }
}

impl Serialize for Canonical<'_> {
    /// Write the keys in the order `question-file.md` fixes and no others.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(None)?;
        match self.question {
            Question::Decide { text, yes, no } => {
                map.serialize_entry("verb", "decide")?;
                map.serialize_entry("text", text.as_str())?;
                if let Some(yes) = yes {
                    map.serialize_entry("true", yes.as_str())?;
                }
                if let Some(no) = no {
                    map.serialize_entry("false", no.as_str())?;
                }
            }
            Question::Choose { text, options } => {
                map.serialize_entry("verb", "choose")?;
                map.serialize_entry("text", text.as_str())?;
                map.serialize_entry("options", &Described(options))?;
            }
            Question::Tag { text, labels } => {
                map.serialize_entry("verb", "tag")?;
                map.serialize_entry("text", text.as_str())?;
                map.serialize_entry("labels", &Described(labels))?;
            }
            Question::Score { text, levels } => {
                map.serialize_entry("verb", "score")?;
                map.serialize_entry("text", text.as_str())?;
                map.serialize_entry("levels", levels)?;
            }
        }
        if let Some(threshold) = self.threshold {
            map.serialize_entry("threshold", &threshold)?;
        }
        map.end()
    }
}

/// The options of a pick, as a map from each option to its description.
struct Described<'a>(&'a Labels);

impl Serialize for Described<'_> {
    /// Write each name in the order it was given, with `null` for no description.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_map(self.0.descriptions())
    }
}

#[cfg(test)]
mod tests {
    use super::{Canonical, question_sha256};
    use crate::core::question::{Labels, Question};
    use crate::core::render::json_line;
    use crate::core::text::{Meaning, QuestionText};
    use crate::core::threshold::Threshold;

    fn text(value: &str) -> QuestionText {
        QuestionText::new(value).expect("not blank")
    }

    fn listed(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| (*value).to_owned()).collect()
    }

    fn band() -> Threshold {
        "0.2:0.8".parse().expect("a band")
    }

    /// The three questions the specification pins a digest for.
    fn pinned() -> [(Question, Option<Threshold>); 3] {
        [
            (
                Question::Decide {
                    text: text("Does this message ask for a refund?"),
                    yes: Some(Meaning::new("The writer asks for money back.").expect("not blank")),
                    no: Some(
                        Meaning::new("The writer asks for anything else.").expect("not blank"),
                    ),
                },
                Some(band()),
            ),
            (
                Question::Choose {
                    text: text("Which team owns this request?"),
                    options: Labels::described(vec![
                        ("billing".to_owned(), Some("Money and invoices.".to_owned())),
                        ("shipping".to_owned(), Some("Parcels and dates.".to_owned())),
                        ("other".to_owned(), None),
                    ])
                    .expect("three options"),
                },
                None,
            ),
            (
                Question::Score {
                    text: text("How much disruption does this report?"),
                    levels: Labels::levels(listed(&["None.", "Some.", "Blocked."]))
                        .expect("three levels"),
                },
                None,
            ),
        ]
    }

    #[test]
    fn the_canonical_form_is_the_document_the_specification_writes_out() {
        let written = |question: &Question, threshold| {
            json_line(&Canonical {
                question,
                threshold,
            })
            .expect("a canonical form is writable")
        };
        let [(decide, cut), (choose, _), (score, _)] = pinned();
        assert_eq!(
            written(&decide, cut),
            concat!(
                r#"{"verb":"decide","text":"Does this message ask for a refund?","#,
                r#""true":"The writer asks for money back.","#,
                r#""false":"The writer asks for anything else.","threshold":"0.2:0.8"}"#,
            )
        );
        assert_eq!(
            written(&choose, None),
            concat!(
                r#"{"verb":"choose","text":"Which team owns this request?","#,
                r#""options":{"billing":"Money and invoices.","shipping":"Parcels and dates.","#,
                r#""other":null}}"#,
            )
        );
        assert_eq!(
            written(&score, None),
            concat!(
                r#"{"verb":"score","text":"How much disruption does this report?","#,
                r#""levels":["None.","Some.","Blocked."]}"#,
            )
        );
    }

    #[test]
    fn the_three_pinned_questions_keep_the_digests_the_page_prints() {
        let digests: Vec<String> = pinned()
            .iter()
            .map(|(question, threshold)| {
                question_sha256(question, *threshold).expect("a question is writable")
            })
            .collect();
        assert_eq!(
            digests,
            [
                "879e7c887684e9b40ff7904ebbcf9b3c545ca84f3657df876a22d7f16218810d",
                "6466cfebbbc92e7d21501d45013f89fc72cf6a533a9020b82edc1784956222fe",
                "831eb29bdbcb62c91bba7790ab0430d40ac2d8e7b866ed1134dab764646f2d34",
            ]
        );
    }

    #[test]
    fn the_tag_canonical_form_and_digest_stay_pinned() {
        let question = Question::Tag {
            text: text("Which topics?"),
            labels: Labels::tags(vec![
                ("billing".to_owned(), None),
                (
                    "urgent".to_owned(),
                    Some("The item needs prompt attention.".to_owned()),
                ),
            ])
            .expect("two tags"),
        };
        let threshold = Some(Threshold::default());
        assert_eq!(
            json_line(&Canonical::new(&question, threshold)).expect("canonical JSON"),
            r#"{"verb":"tag","text":"Which topics?","labels":{"billing":null,"urgent":"The item needs prompt attention."},"threshold":0.5}"#
        );
        assert_eq!(
            question_sha256(&question, threshold).expect("digest"),
            "00b00cf7e1d55b2bb16356f583da7d2dab8fb538f459d859f817392b59efdedf"
        );
    }

    #[test]
    fn a_digest_is_sixty_four_lowercase_hexadecimal_figures() {
        for (question, threshold) in pinned() {
            let digest = question_sha256(&question, threshold).expect("a question is writable");
            assert_eq!(digest.len(), 64, "{digest}");
            assert!(
                digest
                    .chars()
                    .all(|figure| figure.is_ascii_hexdigit() && !figure.is_ascii_uppercase()),
                "{digest}"
            );
        }
    }

    #[test]
    fn a_list_of_names_and_a_map_of_null_descriptions_are_one_question() {
        let of = |options: Labels| {
            question_sha256(
                &Question::Choose {
                    text: text("Which team owns this request?"),
                    options,
                },
                None,
            )
            .expect("a question is writable")
        };
        let listed = of(Labels::options(listed(&["billing", "other"])).expect("two options"));
        let mapped = of(Labels::described(vec![
            ("billing".to_owned(), None),
            ("other".to_owned(), Some("   ".to_owned())),
        ])
        .expect("two options"));
        assert_eq!(listed, mapped);

        let described = of(Labels::described(vec![
            ("billing".to_owned(), Some("Money and invoices.".to_owned())),
            ("other".to_owned(), None),
        ])
        .expect("two options"));
        assert_ne!(listed, described);
    }

    #[test]
    fn every_part_of_a_question_moves_the_digest() {
        let plain = Question::Decide {
            text: text("Does this message ask for a refund?"),
            yes: None,
            no: None,
        };
        let cut = Threshold::default();
        let base = question_sha256(&plain, Some(cut)).expect("a question is writable");
        let moved = [
            (
                Question::Decide {
                    text: text("Does this message ask for a discount?"),
                    yes: None,
                    no: None,
                },
                Some(cut),
            ),
            (
                Question::Decide {
                    text: text("Does this message ask for a refund?"),
                    yes: Some(Meaning::new("Money back.").expect("not blank")),
                    no: None,
                },
                Some(cut),
            ),
            (plain.clone(), Some(band())),
        ];
        for (question, threshold) in moved {
            let digest = question_sha256(&question, threshold).expect("a question is writable");
            assert_ne!(digest, base, "{question:?} {threshold:?}");
        }
        // The default cut and the same cut typed out name one rule, so they
        // name one question and one digest.
        assert_eq!(
            question_sha256(&plain, Some("0.5".parse().expect("a cut")))
                .expect("a question is writable"),
            base
        );
    }
}
