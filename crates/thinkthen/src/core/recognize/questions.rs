//! The step-1 and step-2 questions, and the windows of text each request carries.

use std::ops::Range;

use super::bilou::TAGS;
use super::pieces::Piece;
use crate::core::question::LabelsError;
use crate::core::{Description, Labels, Question, QuestionText, RecognizeSpec};

/// The admitted caller width, or the shared compatibility default.
fn width(spec: Option<&RecognizeSpec>) -> usize {
    let value = spec.map_or(RecognizeSpec::DEFAULT_SNIPPET_PIECES, |s| s.snippet_pieces);
    usize::try_from(value).unwrap_or(usize::MAX)
}

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
const ENTITY_TAG_WORDS: [&str; 5] = [
    "first token of an entity of two or more tokens",
    "a middle token of an entity",
    "last token of an entity of two or more tokens",
    "a one-token entity",
    "not part of an entity",
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
pub(crate) fn shown(count: usize, first: usize, last: usize, width: usize) -> (usize, usize) {
    (
        first.saturating_sub(width),
        last.saturating_add(width).min(count.saturating_sub(1)),
    )
}

/// The byte range of pieces `first` to `last`, inclusive.
pub(crate) fn bytes(pieces: &[Piece], first: usize, last: usize) -> Range<usize> {
    let start = pieces.get(first).map_or(0, |piece| piece.byte_start);
    let end = pieces.get(last).map_or(start, |piece| piece.byte_end);
    start..end
}

/// The byte range of the text a request over pieces `first` to `last` carries.
pub(crate) fn evidence(pieces: &[Piece], first: usize, last: usize, width: usize) -> Range<usize> {
    let (from, to) = shown(pieces.len(), first, last, width);
    bytes(pieces, from, to)
}

/// The text around pieces `first` to `last`, with them wrapped in `[[ ]]`.
fn marked(
    text: &str,
    pieces: &[Piece],
    stretch: (usize, usize),
    ellipses: bool,
    width: usize,
) -> String {
    let (first, last) = stretch;
    let (from, to) = shown(pieces.len(), first, last, width);
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

/// The full caller declaration is shared by every membership and boundary question.
fn task_words(spec: Option<&RecognizeSpec>) -> Option<String> {
    let spec = spec?;
    if spec.instructions.is_none()
        && spec.entity_definition.is_none()
        && !spec.kinds.iter().any(|(_, d)| {
            d.as_ref()
                .is_some_and(|d| !d.blank() && !matches!(d.as_json(), crate::core::Json::Null))
        })
    {
        return None;
    }
    let mut words = String::from(
        "Recognize literal entity spans in the text according to this caller declaration.",
    );
    if let Some(value) = &spec.instructions {
        words.push_str("\nInstructions: ");
        words.push_str(value.as_json().as_str().unwrap_or_default());
    }
    words.push_str("\nEntity definition: ");
    words.push_str(
        spec.entity_definition
            .as_ref()
            .and_then(|v| v.as_json().as_str())
            .unwrap_or("A literal span that satisfies the caller's instructions and listed kinds."),
    );
    if !spec.kinds.is_empty() {
        words.push_str("\nKinds and descriptions: ");
        // A structured description retains its JSON shape and authored label order.
        for (name, description) in &spec.kinds {
            words.push_str(&format!("\n{name}: "));
            if let Some(description) = description {
                words.push_str(&serde_json::to_string(description).unwrap_or_default());
            }
        }
    }
    Some(words)
}

/// One pick-one BILOU question per piece in `group`.
pub(crate) fn step_one_questions(
    text: &str,
    pieces: &[Piece],
    group: Range<usize>,
    kinds: &[&str],
    spec: Option<&RecognizeSpec>,
) -> Result<Vec<Question>, LabelsError> {
    let custom = task_words(spec);
    let words = custom.as_ref().map_or_else(
        || step_one_words(kinds),
        |task| format!("{task}\n{SPLIT} Where does the [[ ]] token stand in an entity?"),
    );
    let options = Labels::described(
        TAGS.iter()
            .zip(if custom.is_some() {
                ENTITY_TAG_WORDS
            } else {
                TAG_WORDS
            })
            .map(|(tag, meaning)| ((*tag).to_owned(), Some(Description::text(meaning))))
            .collect(),
    )?;
    group
        .map(|place| {
            Ok(Question::Choose {
                text: text_of(format!(
                    "{words}\n\nSnippet: {}",
                    marked(text, pieces, (place, place), false, width(spec))
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
    spec: Option<&RecognizeSpec>,
) -> Result<Question, LabelsError> {
    let custom = task_words(spec);
    let words = custom.as_ref().map_or_else(|| KIND_WORDS.to_owned(), |task|
        format!("{task}\nWhich listed kind applies to the entity wrapped in [[ ]]? Choose none of these when the span fails the caller declaration or no listed kind applies."));
    let mut options = kinds.to_vec();
    options.push((
        NONE_OF_THESE.to_owned(),
        Some(Description::text(if custom.is_some() {
            "The span fails the caller declaration or no listed kind applies."
        } else {
            DECLINE_WORDS
        })),
    ));
    Ok(Question::Choose {
        text: text_of(format!(
            "{words}\n\nText: {}",
            marked(text, pieces, name, false, width(spec))
        ))?,
        options: Labels::recognition_menu(options)?,
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

/// A stretch's edge label: its text with every run of white space or control
/// characters shown as one space. It is empty when the stretch holds only those.
pub(crate) fn edge_label(text: &str, pieces: &[Piece], (first, last): (usize, usize)) -> String {
    text.get(bytes(pieces, first, last))
        .unwrap_or_default()
        .split(|c: char| c.is_whitespace() || c.is_control())
        .filter(|word| !word.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

/// The edge question over `options`, each labelled by [`edge_label`]. The
/// description keeps the real snippet.
pub(crate) fn edge_question(
    text: &str,
    pieces: &[Piece],
    options: &[(usize, usize)],
    spec: Option<&RecognizeSpec>,
) -> Result<Question, LabelsError> {
    let words = task_words(spec).map_or_else(|| EDGE_WORDS.to_owned(), |task|
        format!("{task}\nPick the option that wraps exactly the whole requested entity. Retain literal punctuation that belongs to the entity, including decimal points, slashes and web-address punctuation. Leave surrounding sentence punctuation outside."));
    let mut labels: Vec<(String, Option<Description>)> = Vec::with_capacity(options.len());
    for (first, last) in options.iter().copied() {
        let mut label = edge_label(text, pieces, (first, last));
        while labels.iter().any(|(held, _)| *held == label) {
            label.push(' ');
        }
        let shown = marked(text, pieces, (first, last), true, width(spec));
        labels.push((label, Some(Description::text(shown))));
    }
    let found = options.first().copied().unwrap_or_default();
    Ok(Question::Choose {
        text: text_of(format!(
            "{words}\n\nText: {}",
            marked(text, pieces, found, false, width(spec))
        ))?,
        options: Labels::described(labels)?,
    })
}
