//! The step-1 and step-2 questions, and the windows of text each request carries.

use std::ops::Range;

use super::bilou::TAGS;
use super::pieces::Piece;
use crate::core::question::LabelsError;
use crate::core::{Description, Labels, Question, QuestionText};

/// How many pieces each side of a piece or name a snippet and a request show.
pub(crate) const WINDOW: usize = 6;

/// How many piece questions one step-1 request holds at most.
pub(crate) const GROUP: usize = 40;

/// The answer that declines a name in step 2.
pub(crate) const NONE_OF_THESE: &str = "none of these";

const SPLIT: &str = "Tokens are split at spaces and at each punctuation mark.";
const MARKS_OUT: &str =
    "dates, numbers, and marks that are not part of a name's own spelling are OUT.";
const TAG_WORDS: [&str; 5] = [
    "first token of a name of two or more tokens",
    "a middle token of a name",
    "last token of a name of two or more tokens",
    "a one-token name",
    "not part of a name",
];
const KIND_WORDS: &str = "In the text below, some words are wrapped in [[ ]]. Going by what they refer to in this text, which listed kind of name are they? Choose none of these when they are not a proper name, or when they name something that no listed kind covers.";
const DECLINE_WORDS: &str = "They are not a proper name, or no listed kind covers what they name.";
const EDGE_WORDS: &str = "In the text below, a name was found at the words wrapped in [[ ]]. Each option wraps a slightly different stretch of the text. Pick the option that wraps exactly the whole name. A punctuation mark that is part of the name's own spelling belongs inside it. A mark that belongs to the sentence around the name stays outside.";

/// The piece places each step-1 request asks about, 40 at most, in order.
pub(crate) fn step_one_groups(count: usize) -> Vec<Range<usize>> {
    (0..count)
        .step_by(GROUP)
        .map(|first| first..count.min(first + GROUP))
        .collect()
}

/// The ranges of `names` that share one step-2 request: the names whose first
/// piece sits in one step-1 request. `names` are in text order.
pub(crate) fn name_groups(names: &[(usize, usize)]) -> Vec<Range<usize>> {
    let mut groups: Vec<Range<usize>> = Vec::new();
    for (place, (first, _)) in names.iter().enumerate() {
        match groups.last_mut() {
            Some(group)
                if names
                    .get(group.start)
                    .is_some_and(|(held, _)| held / GROUP == first / GROUP) =>
            {
                group.end = place + 1;
            }
            _ => groups.push(place..place + 1),
        }
    }
    groups
}

/// The inclusive piece places shown around pieces `first` to `last`.
pub(crate) fn shown(count: usize, first: usize, last: usize) -> (usize, usize) {
    (
        first.saturating_sub(WINDOW),
        last.saturating_add(WINDOW).min(count.saturating_sub(1)),
    )
}

/// The byte range of pieces `first` to `last`, inclusive.
pub(crate) fn bytes(pieces: &[Piece], first: usize, last: usize) -> Range<usize> {
    let start = pieces.get(first).map_or(0, |piece| piece.byte_start);
    let end = pieces.get(last).map_or(start, |piece| piece.byte_end);
    start..end
}

/// The byte range of the text a request over pieces `first` to `last` carries.
pub(crate) fn evidence(pieces: &[Piece], first: usize, last: usize) -> Range<usize> {
    let (from, to) = shown(pieces.len(), first, last);
    bytes(pieces, from, to)
}

/// The text around pieces `first` to `last`, with them wrapped in `[[ ]]`.
fn marked(text: &str, pieces: &[Piece], stretch: (usize, usize), ellipses: bool) -> String {
    let (first, last) = stretch;
    let (from, to) = shown(pieces.len(), first, last);
    let outer = bytes(pieces, from, to);
    let inner = bytes(pieces, first, last);
    let part = |range: Range<usize>| text.get(range).unwrap_or_default();
    format!(
        "{}{}[[{}]]{}{}",
        if ellipses && from > 0 { "..." } else { "" },
        part(outer.start..inner.start),
        part(inner.clone()),
        part(inner.end..outer.end),
        if ellipses && to + 1 < pieces.len() {
            "..."
        } else {
            ""
        },
    )
}

fn text_of(words: String) -> Result<QuestionText, LabelsError> {
    QuestionText::new(words).map_err(|_| LabelsError::OptionBlank)
}

/// The step-1 question words: labelled when the caller gave kinds.
fn step_one_words(kinds: &[&str]) -> String {
    if kinds.is_empty() {
        return format!(
            "{SPLIT} Where does the [[ ]] token stand in a name? A name is the proper name of a particular person, organisation, place, product, work, event or other thing. Ordinary words, {MARKS_OUT}"
        );
    }
    format!(
        "{SPLIT} Where does the [[ ]] token stand in a name of one of these kinds: {}? Other names, ordinary words, {MARKS_OUT}",
        kinds.join(", ")
    )
}

/// One pick-one BILOU question per piece in `group`.
pub(crate) fn step_one_questions(
    text: &str,
    pieces: &[Piece],
    group: Range<usize>,
    kinds: &[&str],
) -> Result<Vec<Question>, LabelsError> {
    let words = step_one_words(kinds);
    let options = Labels::described(
        TAGS.iter()
            .zip(TAG_WORDS)
            .map(|(tag, meaning)| ((*tag).to_owned(), Some(Description::text(meaning))))
            .collect(),
    )?;
    group
        .map(|place| {
            Ok(Question::Choose {
                text: text_of(format!(
                    "{words}\n\nSnippet: {}",
                    marked(text, pieces, (place, place), false)
                ))?,
                options: options.clone(),
            })
        })
        .collect()
}

/// The kind question for one found name: the caller's kinds, then `none of these`.
pub(crate) fn kind_question(
    text: &str,
    pieces: &[Piece],
    name: (usize, usize),
    kinds: &[(String, Option<Description>)],
) -> Result<Question, LabelsError> {
    let mut options = kinds.to_vec();
    options.push((
        NONE_OF_THESE.to_owned(),
        Some(Description::text(DECLINE_WORDS)),
    ));
    Ok(Question::Choose {
        text: text_of(format!(
            "{KIND_WORDS}\n\nText: {}",
            marked(text, pieces, name, false)
        ))?,
        options: Labels::described(options)?,
    })
}

/// The stretches the edge question offers, the name as found first: plus a
/// touching mark at the right, plus one at the left, less a last mark, less a
/// first mark.
pub(crate) fn edge_options(
    text: &str,
    pieces: &[Piece],
    name: (usize, usize),
) -> Vec<(usize, usize)> {
    let (first, last) = name;
    let piece = |place: usize| pieces.get(place);
    let touching_mark = |place: Option<usize>, edge: Option<Piece>, right: bool| {
        let (Some(place), Some(edge)) = (place, edge) else {
            return false;
        };
        piece(place).is_some_and(|held| {
            held.is_mark(text)
                && if right {
                    held.byte_start == edge.byte_end
                } else {
                    held.byte_end == edge.byte_start
                }
        })
    };
    let mut options = vec![name];
    if touching_mark(Some(last + 1), piece(last).copied(), true) {
        options.push((first, last + 1));
    }
    if touching_mark(first.checked_sub(1), piece(first).copied(), false) {
        options.push((first - 1, last));
    }
    let is_mark = |place: usize| piece(place).is_some_and(|held| held.is_mark(text));
    if last > first && is_mark(last) {
        options.push((first, last - 1));
    }
    if last > first && is_mark(first) {
        options.push((first + 1, last));
    }
    options
}

/// The edge question over `options`. Each label is its own text with every
/// white-space run shown as one space. The description keeps the real snippet.
pub(crate) fn edge_question(
    text: &str,
    pieces: &[Piece],
    options: &[(usize, usize)],
) -> Result<Question, LabelsError> {
    let mut labels: Vec<(String, Option<Description>)> = Vec::with_capacity(options.len());
    for (first, last) in options.iter().copied() {
        let mut label = text
            .get(bytes(pieces, first, last))
            .unwrap_or_default()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        while labels.iter().any(|(held, _)| *held == label) {
            label.push(' ');
        }
        let shown = marked(text, pieces, (first, last), true);
        labels.push((label, Some(Description::text(shown))));
    }
    let found = options.first().copied().unwrap_or_default();
    Ok(Question::Choose {
        text: text_of(format!(
            "{EDGE_WORDS}\n\nText: {}",
            marked(text, pieces, found, false)
        ))?,
        options: Labels::described(labels)?,
    })
}
