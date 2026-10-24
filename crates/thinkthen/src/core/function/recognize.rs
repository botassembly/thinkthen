//! Names proposed from deterministic pieces and judged through choice asks.

mod decode;

use crate::core::{
    Answer, Ask, AskAnswer, ChoiceOption, Detail, Error, ErrorKind, Outcome, Probability,
    Threshold, Verdict,
};

const TAGS: [(&str, &str); 5] = [
    (
        "BEGIN",
        "The first piece of a multi-piece proper name identifying a particular entity.",
    ),
    (
        "INSIDE",
        "A middle piece of a multi-piece proper name identifying a particular entity.",
    ),
    (
        "END",
        "The last piece of a multi-piece proper name identifying a particular entity.",
    ),
    (
        "SINGLE",
        "A complete one-piece proper name identifying a particular entity.",
    ),
    (
        "OUT",
        "Outside the proper name, including a category description even when it matches a listed kind.",
    ),
];

// Schema derives emit metadata maps in sibling impls, scoped to result values.
#[allow(
    clippy::disallowed_types,
    reason = "schema derivation builds metadata; evidence remains typed"
)]
mod output {
    use crate::core::Probability;
    use serde::Serialize;
    use std::collections::BTreeMap;
    /// A name recognized in the original text.
    #[derive(Debug, Clone, PartialEq, Serialize, schemars::JsonSchema)]
    pub struct Name {
        /// The original text covered by the name.
        pub text: String,
        /// Inclusive Unicode scalar offset.
        pub start: usize,
        /// Exclusive Unicode scalar offset.
        pub end: usize,
        /// The chosen kind, or ENTITY when kinds were omitted.
        pub kind: String,
        /// The least member-piece membership multiplied by the chosen kind's probability.
        /// This support score is not the probability of exact name boundaries.
        pub p: Probability,
    }

    /// Backend probabilities for one original piece.
    #[derive(Debug, Clone, PartialEq, Serialize, schemars::JsonSchema)]
    pub struct PieceDetail {
        /// The original piece.
        pub text: String,
        /// Inclusive Unicode scalar offset.
        pub start: usize,
        /// Exclusive Unicode scalar offset.
        pub end: usize,
        /// The five boundary tag probabilities.
        pub tags: BTreeMap<String, Probability>,
    }

    /// Backend kind probabilities for one decoded name, including dropped names.
    #[derive(Debug, Clone, PartialEq, Serialize, schemars::JsonSchema)]
    pub struct KindDetail {
        /// Inclusive Unicode scalar offset.
        pub start: usize,
        /// Exclusive Unicode scalar offset.
        pub end: usize,
        /// Each kind's probability, including none of these.
        pub kinds: BTreeMap<String, Probability>,
    }

    /// The underlying choice probabilities for name recognition.
    #[derive(Debug, Clone, PartialEq, Serialize, schemars::JsonSchema)]
    pub struct RecognizeDetail {
        /// Each piece's boundary probabilities in text order.
        pub pieces: Vec<PieceDetail>,
        /// Each decoded name's kind probabilities in text order.
        pub names: Vec<KindDetail>,
    }
}
pub use output::{KindDetail, Name, PieceDetail, RecognizeDetail};
/// A name-recognition question with optional ordered kinds.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct Recognize {
    kinds: Vec<ChoiceOption>,
    threshold: Threshold,
}

impl Recognize {
    pub(crate) fn definition(&self) -> super::identity::Definition<'_> {
        super::identity::Definition::Names {
            kinds: self.kinds.iter().map(Into::into).collect(),
            threshold: self.threshold.canonical(),
        }
    }

    /// Recognize names of these kinds, or arbitrary names with an empty list.
    ///
    /// # Errors
    /// Returns usage.question for blank, duplicate or reserved kinds.
    pub fn new(kinds: impl IntoIterator<Item = impl Into<String>>) -> Result<Self, Error> {
        let kinds = kinds
            .into_iter()
            .map(|kind| ChoiceOption::new(kind.into()))
            .collect::<Result<Vec<_>, _>>()?;
        for (i, kind) in kinds.iter().enumerate() {
            if kind.label() == "none of these"
                || kinds
                    .iter()
                    .take(i)
                    .any(|earlier| earlier.label() == kind.label())
            {
                return Err(Error::new(
                    ErrorKind::Question,
                    "recognize kinds must be distinct and cannot be none of these",
                ));
            }
        }
        Ok(Self {
            kinds,
            threshold: Threshold::default(),
        })
    }

    /// Set the minimum combined membership support and kind score.
    ///
    /// # Errors
    /// Returns usage.threshold when given a band.
    pub fn threshold(self, threshold: Threshold) -> Result<Self, Error> {
        Ok(Self {
            threshold: threshold.require_cut()?,
            ..self
        })
    }
}

type Decoded = (Vec<Piece>, Vec<Vec<Probability>>, Vec<decode::Span>);

#[derive(Debug)]
struct Piece {
    text: String,
    start: usize,
    end: usize,
    byte_start: usize,
    byte_end: usize,
    letter_or_digit: bool,
    mark: bool,
}

fn pieces(evidence: &str) -> Vec<Piece> {
    use unicode_general_category::{GeneralCategory as Category, get_general_category};

    let mut result: Vec<Piece> = Vec::new();
    let mut prefix = true;
    let mut word = false;
    for (offset, (byte, ch)) in evidence.char_indices().enumerate() {
        if prefix
            && matches!(
                ch,
                '\u{feff}' | '\u{200b}' | '\u{200c}' | '\u{200d}' | '\u{2060}'
            )
        {
            continue;
        }
        prefix = false;
        if ch.is_whitespace() {
            word = false;
            continue;
        }
        let category = get_general_category(ch);
        let letter_or_digit = matches!(
            category,
            Category::UppercaseLetter
                | Category::LowercaseLetter
                | Category::TitlecaseLetter
                | Category::ModifierLetter
                | Category::OtherLetter
                | Category::DecimalNumber
        );
        let separate = matches!(
            category,
            Category::ClosePunctuation
                | Category::ConnectorPunctuation
                | Category::DashPunctuation
                | Category::FinalPunctuation
                | Category::InitialPunctuation
                | Category::OpenPunctuation
                | Category::OtherPunctuation
                | Category::CurrencySymbol
                | Category::MathSymbol
                | Category::ModifierSymbol
                | Category::OtherSymbol
        );
        let joiner = matches!(
            category,
            Category::NonspacingMark | Category::EnclosingMark | Category::Format
        );
        if let Some(piece) = result.last_mut()
            && piece.end == offset
            && (joiner || (word && !separate))
        {
            piece.text.push(ch);
            piece.end = offset + 1;
            piece.byte_end = byte + ch.len_utf8();
            piece.letter_or_digit |= letter_or_digit;
        } else {
            result.push(Piece {
                text: ch.to_string(),
                start: offset,
                end: offset + 1,
                byte_start: byte,
                byte_end: byte + ch.len_utf8(),
                letter_or_digit,
                mark: separate,
            });
            word = true;
        }
        if !joiner {
            word = !separate;
        }
    }
    result
}

#[expect(
    clippy::indexing_slicing,
    reason = "piece indices come from decoding this same piece vector"
)]
fn snippet(evidence: &str, pieces: &[Piece], first: usize, last: usize) -> String {
    let left = pieces[first.saturating_sub(6)].byte_start;
    let right = pieces[(last + 6).min(pieces.len() - 1)].byte_end;
    let start = pieces[first].byte_start;
    let end = pieces[last].byte_end;
    format!(
        "{}[[{}]]{}",
        &evidence[left..start],
        &evidence[start..end],
        &evidence[end..right]
    )
}

#[expect(
    clippy::indexing_slicing,
    reason = "spans come from decoding these same pieces and probability rows"
)]
fn join_spans(
    pieces: &[Piece],
    probabilities: &[[f64; 5]],
    spans: Vec<decode::Span>,
) -> Vec<decode::Span> {
    let mut joined: Vec<decode::Span> = Vec::new();
    for span in spans {
        if let Some(left) = joined.last_mut() {
            let adjacent = span.first == left.last + 1;
            let bridge = span.first == left.last + 2
                && pieces[left.last + 1].mark
                && pieces[left.last].byte_end == pieces[left.last + 1].byte_start
                && pieces[left.last + 1].byte_end == pieces[span.first].byte_start;
            let before = probabilities[left.last];
            let after = probabilities[span.first];
            let supported = before[2] + before[3] > before[0] + before[1]
                && after[0] + after[3] > after[1] + after[2];
            if (adjacent || bridge) && !supported {
                left.p = left.p.min(span.p);
                if bridge {
                    left.p = left.p.min(1.0 - probabilities[left.last + 1][4]);
                }
                left.last = span.last;
                continue;
            }
        }
        joined.push(span);
    }
    joined
}

fn complete_brackets(pieces: &[Piece], first: usize, last: usize, limit: usize) -> usize {
    let mut closers = Vec::new();
    for character in pieces
        .iter()
        .skip(first)
        .take(last - first + 1)
        .flat_map(|piece| piece.text.chars())
    {
        match character {
            '(' => closers.push(')'),
            '[' => closers.push(']'),
            '{' => closers.push('}'),
            ')' | ']' | '}' if closers.pop() != Some(character) => return last,
            _ => {}
        }
    }
    let mut end = last;
    let mut edge = pieces.get(last).map_or(0, |piece| piece.byte_end);
    for (place, piece) in pieces.iter().enumerate().take(limit).skip(last + 1) {
        if closers
            .last()
            .copied()
            .is_none_or(|closer| piece.byte_start != edge || !piece.text.starts_with(closer))
        {
            break;
        }
        closers.pop();
        end = place;
        edge = piece.byte_end;
    }
    end
}

fn choices(answers: &[Result<AskAnswer, Error>]) -> Result<Vec<Vec<Probability>>, Error> {
    answers
        .iter()
        .map(|answer| {
            answer
                .as_ref()
                .map(|answer| answer.probabilities().to_vec())
                .map_err(Clone::clone)
        })
        .collect()
}

impl Recognize {
    pub(crate) fn initial(&self, evidence: &str) -> Vec<Ask> {
        let state: std::sync::Arc<str> = evidence.into();
        let pieces = pieces(evidence);
        let target = if self.kinds.is_empty() {
            "a proper name of a particular person, organization, place, product, work, event or other thing".to_owned()
        } else {
            format!(
                "a name of one of these kinds: {}",
                self.kinds
                    .iter()
                    .map(ChoiceOption::label)
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        };
        pieces
            .iter()
            .enumerate()
            .map(|(i, _)| Ask::Choice {
                instructions: super::quoted(
                    &snippet(evidence, &pieces, i, i),
                    &format!(
                        "What position does the [[ ]] piece have in {target}? Other names and ordinary words count as OUT. Dates, numbers and marks count as OUT when they are not part of the name's spelling. A category word before a name is OUT when it only describes the kind of thing named; include it only when it belongs to the proper name itself."
                    ),
                ),
                options: TAGS
                    .into_iter()
                    .map(|(tag, description)| option(tag).description(description))
                    .collect(),
            })
            .map(|ask| ask.with_state(state.clone()))
            .collect()
    }

    fn decoded(
        &self,
        evidence: &str,
        answers: &[Result<AskAnswer, Error>],
    ) -> Result<Decoded, Error> {
        let pieces = pieces(evidence);
        let probabilities = choices(answers)?;
        let weights = probabilities
            .iter()
            .map(|p| {
                let mut row = [0.0; 5];
                for (slot, p) in row.iter_mut().zip(p) {
                    *slot = p.get();
                }
                row
            })
            .collect::<Vec<_>>();
        let mut spans = join_spans(&pieces, &weights, decode::decode(&weights));
        let mut limit = pieces.len();
        for span in spans.iter_mut().rev() {
            span.last = complete_brackets(&pieces, span.first, span.last, limit);
            limit = span.first;
        }
        spans.retain(|span| {
            pieces
                .iter()
                .skip(span.first)
                .take(span.last - span.first + 1)
                .any(|piece| piece.letter_or_digit)
        });
        Ok((pieces, probabilities, spans))
    }

    pub(crate) fn kinds(
        &self,
        evidence: &str,
        answers: &[Result<AskAnswer, Error>],
    ) -> Result<Vec<Ask>, Error> {
        let (pieces, _, spans) = self.decoded(evidence, answers)?;
        if self.kinds.is_empty() {
            return Ok(Vec::new());
        }
        let state: std::sync::Arc<str> = evidence.into();
        let mut options = self.kinds.clone();
        options.push(
            option("none of these").description("The marked words are not a proper name, merely describe a category, or name something outside the listed kinds."),
        );
        Ok(spans
            .into_iter()
            .map(|span| Ask::Choice {
                instructions: super::quoted(
                    &snippet(evidence, &pieces, span.first, span.last),
                    "Consider what the words marked [[ ]] refer to in this text. Which listed kind of name fits them? Select none of these if the words are not a proper name, or if none of the listed kinds describes what they name. A word that merely names a category is not a proper name, even when that category is a listed kind. Select none of these for such a word.",
                ),
                options: options.clone(),
            })
            .map(|ask| ask.with_state(state.clone()))
            .collect())
    }

    #[expect(
        clippy::indexing_slicing,
        reason = "validated choice counts and decoded spans bound every index"
    )]
    pub(crate) fn finish(
        &self,
        evidence: &str,
        boundary: &[Result<AskAnswer, Error>],
        kind_answers: &[Result<AskAnswer, Error>],
    ) -> Result<(Vec<Name>, RecognizeDetail), Error> {
        let (pieces, probabilities, spans) = self.decoded(evidence, boundary)?;
        let kinds = choices(kind_answers)?;
        let mut detail = RecognizeDetail {
            pieces: pieces
                .iter()
                .zip(&probabilities)
                .map(|(piece, probabilities)| PieceDetail {
                    text: piece.text.clone(),
                    start: piece.start,
                    end: piece.end,
                    tags: TAGS
                        .iter()
                        .zip(probabilities)
                        .map(|((tag, _), p)| ((*tag).to_owned(), *p))
                        .collect(),
                })
                .collect(),
            names: Vec::new(),
        };
        let mut names = Vec::new();
        for (i, span) in spans.into_iter().enumerate() {
            let start = pieces[span.first].start;
            let end = pieces[span.last].end;
            let (kind, kind_p) = if self.kinds.is_empty() {
                ("ENTITY".to_owned(), 1.0)
            } else {
                let probabilities = &kinds[i];
                let mut labels: Vec<_> = self.kinds.iter().map(ChoiceOption::label).collect();
                labels.push("none of these");
                detail.names.push(KindDetail {
                    start,
                    end,
                    kinds: labels
                        .iter()
                        .zip(probabilities)
                        .map(|(label, p)| ((*label).to_owned(), *p))
                        .collect(),
                });
                let (winner, _) = crate::core::ask::argmax(probabilities);
                if winner == self.kinds.len() {
                    continue;
                }
                (labels[winner].to_owned(), probabilities[winner].get())
            };
            let p = probability(span.p * kind_p);
            if self.threshold.verdict(p) == Verdict::Yes {
                names.push(Name {
                    text: evidence.chars().skip(start).take(end - start).collect(),
                    start,
                    end,
                    kind,
                    p,
                });
            }
        }
        Ok((names, detail))
    }
}

#[expect(
    clippy::expect_used,
    reason = "internal option labels are fixed nonblank strings"
)]
fn option(label: &str) -> ChoiceOption {
    ChoiceOption::new(label).expect("fixed option")
}
#[expect(
    clippy::expect_used,
    reason = "support products are clamped to the probability interval"
)]
fn probability(value: f64) -> Probability {
    Probability::new(value.clamp(0.0, 1.0)).expect("bounded support")
}

impl super::Function for Recognize {
    const NAME: &'static str = "recognize";
    fn identity_as(&self, function: &str) -> String {
        super::identity(function, &self.definition())
    }
    fn try_asks_record(
        &self,
        record: &crate::core::AdmittedRecord,
        _limits: crate::core::Limits,
    ) -> Result<Vec<Ask>, Error> {
        Ok(self.initial(record.evidence()))
    }
    fn followup_record(
        context: crate::core::FunctionAnswers<'_, Self>,
        _limits: crate::core::Limits,
    ) -> Result<Vec<Ask>, Error> {
        let question = context.question();
        let record = context.record();
        let answers = context.answers();
        let evidence = record.evidence();
        if answers.len() > pieces(evidence).len() {
            Ok(Vec::new())
        } else {
            question.kinds(evidence, answers)
        }
    }
    fn read_record(
        context: crate::core::FunctionAnswers<'_, Self>,
        _limits: crate::core::Limits,
    ) -> Outcome {
        let question = context.question();
        let record = context.record();
        let answers = context.answers();
        let evidence = record.evidence();
        let count = pieces(evidence).len();
        let (boundary, kinds) = answers.split_at(count);
        question
            .finish(evidence, boundary, kinds)
            .map(|(names, detail)| Outcome::Answered {
                answer: Answer::Names(names),
                p: None,
                detail: Some(Detail::Recognize(detail)),
            })
            .unwrap_or_else(|error| Outcome::Failed { error })
    }
}
