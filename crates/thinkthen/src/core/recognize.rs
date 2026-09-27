//! Pure name recognition planning and assembly.

use serde::Serialize;

use crate::core::text::Withheld;

const DETECTION_WORDS: &str = "The snippet shows five consecutive words from a news document; the word in question is wrapped in [[ ]]. <BEGINNING> marks the start of the document and <END> the end. Decide whether the wrapped word is part of the name of an entity: a person, an organization, a place, or another named entity such as a nationality, an event, a product, or a creative work. Ordinary words, dates, and numbers that are not part of such a name are not named.";
const KIND_WORDS: &str = "The snippet shows five consecutive words from a news document; the word in question is wrapped in [[ ]]. <BEGINNING> marks the start of the document and <END> the end. If the wrapped word is part of an entity's name, which kind of entity is it part of? Answer for every word; the answer only matters when the word is part of a name.";

#[derive(Clone, Eq, PartialEq)]
pub(crate) struct Token {
    text: String,
    byte_start: usize,
    byte_end: usize,
    start: usize,
    end: usize,
}

/// A token is evidence, so `Debug` withholds its text and keeps its places.
impl std::fmt::Debug for Token {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("Token")
            .field("text", &Withheld(self.text.len()))
            .field("byte_start", &self.byte_start)
            .field("byte_end", &self.byte_end)
            .field("start", &self.start)
            .field("end", &self.end)
            .finish()
    }
}

impl Token {
    pub(crate) fn text(&self) -> &str {
        &self.text
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct TokenAnswer {
    pub(crate) detected: bool,
    pub(crate) detection_probability: f64,
    pub(crate) kind: usize,
    pub(crate) kind_probabilities: Vec<f64>,
}

#[derive(Clone, PartialEq, Serialize)]
pub(crate) struct RecognizedName {
    pub(crate) name: String,
    pub(crate) kind: String,
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) strength: f64,
}

/// A name is evidence, so `Debug` withholds it and keeps the configured kind.
impl std::fmt::Debug for RecognizedName {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("RecognizedName")
            .field("name", &Withheld(self.name.len()))
            .field("kind", &self.kind)
            .field("start", &self.start)
            .field("end", &self.end)
            .field("strength", &self.strength)
            .finish()
    }
}

pub(crate) fn tokenize(text: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut piece_start = None;
    for (byte, character) in text
        .char_indices()
        .chain(std::iter::once((text.len(), ' ')))
    {
        if !character.is_whitespace() {
            piece_start.get_or_insert(byte);
            continue;
        }
        if let Some(start) = piece_start.take() {
            split_piece(text, start, byte, &mut tokens);
        }
    }
    tokens
}

fn split_piece(text: &str, start: usize, end: usize, tokens: &mut Vec<Token>) {
    let Some(piece) = text.get(start..end) else {
        return;
    };
    let trailing: Vec<(usize, char)> = piece
        .char_indices()
        .rev()
        .take_while(|(_, character)| ".!?,:;".contains(*character))
        .collect();
    let stem_end = trailing.last().map_or(end, |(byte, _)| start + *byte);
    if stem_end > start {
        push_token(text, start, stem_end, tokens);
    }
    for (byte, character) in trailing.into_iter().rev() {
        let punctuation_start = start + byte;
        push_token(
            text,
            punctuation_start,
            punctuation_start + character.len_utf8(),
            tokens,
        );
    }
}

fn push_token(text: &str, byte_start: usize, byte_end: usize, tokens: &mut Vec<Token>) {
    let Some(word) = text.get(byte_start..byte_end) else {
        return;
    };
    let start = text
        .get(..byte_start)
        .map_or(0, |prefix| prefix.chars().count());
    tokens.push(Token {
        text: word.to_owned(),
        byte_start,
        byte_end,
        start,
        end: start + word.chars().count(),
    });
}

pub(crate) fn recognition_questions(
    tokens: &[Token],
) -> Result<Vec<crate::core::Question>, crate::core::question::LabelsError> {
    let mut questions = Vec::with_capacity(tokens.len().saturating_mul(2));
    for place in 0..tokens.len() {
        questions.push(choice_question(
            DETECTION_WORDS,
            window(tokens, place),
            [
                ("IN", "This word is part of an entity's name."),
                ("OUT", "This word is not part of any entity's name."),
            ],
        )?);
    }
    Ok(questions)
}

pub(crate) fn kind_questions(
    tokens: &[Token],
    kinds: &[(String, Option<crate::core::Description>)],
) -> Result<Vec<crate::core::Question>, crate::core::question::LabelsError> {
    if kinds.len() == 1 {
        return Ok(Vec::new());
    }
    let labels = crate::core::Labels::described(kinds.to_vec())?;
    let mut questions = Vec::with_capacity(tokens.len());
    for place in 0..tokens.len() {
        let text = format!("{KIND_WORDS}\n\nSnippet: {}", window(tokens, place));
        questions.push(crate::core::Question::Choose {
            text: crate::core::QuestionText::new(text)
                .map_err(|_| crate::core::question::LabelsError::OptionBlank)?,
            options: labels.clone(),
        });
    }
    Ok(questions)
}

fn choice_question<const N: usize>(
    words: &str,
    snippet: String,
    options: [(&str, &str); N],
) -> Result<crate::core::Question, crate::core::question::LabelsError> {
    let listed = options
        .into_iter()
        .map(|(name, description)| {
            (
                name.to_owned(),
                Some(crate::core::Description::text(description)),
            )
        })
        .collect();
    Ok(crate::core::Question::Choose {
        text: crate::core::QuestionText::new(format!("{words}\n\nSnippet: {snippet}"))
            .map_err(|_| crate::core::question::LabelsError::OptionBlank)?,
        options: crate::core::Labels::described(listed)?,
    })
}

pub(crate) fn window(tokens: &[Token], place: usize) -> String {
    let at = |index: isize| -> &str {
        if index < 0 {
            return "<BEGINNING>";
        }
        tokens
            .get(usize::try_from(index).unwrap_or(usize::MAX))
            .map_or("<END>", Token::text)
    };
    let center = isize::try_from(place).unwrap_or(isize::MAX);
    format!(
        "{} {} [[{}]] {} {}",
        at(center - 2),
        at(center - 1),
        at(center),
        at(center + 1),
        at(center + 2)
    )
}

pub(crate) fn assemble(
    text: &str,
    tokens: &[Token],
    answers: &[TokenAnswer],
    kinds: &[String],
    threshold: f64,
) -> Vec<RecognizedName> {
    let mut candidates = Vec::new();
    let mut place = 0;
    while place < tokens.len() {
        if !answers.get(place).is_some_and(|answer| answer.detected) {
            place += 1;
            continue;
        }
        let start = place;
        while answers.get(place).is_some_and(|answer| answer.detected) {
            place += 1;
        }
        let mut end = place.saturating_sub(1);
        if end > start
            && tokens
                .get(end)
                .is_some_and(|token| token.text.eq_ignore_ascii_case("'s"))
        {
            end = end.saturating_sub(1);
        }
        if let Some(candidate) = candidate(text, tokens, answers, kinds, start, end)
            && candidate.strength >= threshold
        {
            candidates.push(candidate);
        }
    }
    candidates
}

fn candidate(
    text: &str,
    tokens: &[Token],
    answers: &[TokenAnswer],
    kinds: &[String],
    start: usize,
    end: usize,
) -> Option<RecognizedName> {
    let winner = kind_vote(answers, start, end)?;
    let kind = kinds.get(winner)?.clone();
    let first = tokens.get(start)?;
    let last = tokens.get(end)?;
    let name = text.get(first.byte_start..last.byte_end)?.to_owned();
    let mut least = 1.0_f64;
    let mut kind_total = 0.0;
    let mut count = 0_usize;
    for answer in answers.iter().skip(start).take(end - start + 1) {
        least = least.min(answer.detection_probability);
        kind_total += answer
            .kind_probabilities
            .get(winner)
            .copied()
            .unwrap_or(0.0);
        count += 1;
    }
    let mean = kind_total / count as f64;
    let strength = format!("{:.4}", least * mean)
        .parse()
        .unwrap_or(least * mean);
    Some(RecognizedName {
        name,
        kind,
        start: first.start,
        end: last.end,
        strength,
    })
}

fn kind_vote(answers: &[TokenAnswer], start: usize, end: usize) -> Option<usize> {
    let first = answers.get(start)?.kind;
    let greatest_kind = answers
        .iter()
        .skip(start)
        .take(end - start + 1)
        .map(|answer| answer.kind)
        .max()
        .unwrap_or(first);
    let mut counts = vec![0_usize; greatest_kind.saturating_add(1)];
    for answer in answers.iter().skip(start).take(end - start + 1) {
        if let Some(count) = counts.get_mut(answer.kind) {
            *count += 1;
        }
    }
    let winner_count = counts.iter().copied().max().unwrap_or(0);
    let winner = answers
        .iter()
        .skip(start)
        .take(end - start + 1)
        .find(|answer| counts.get(answer.kind) == Some(&winner_count))
        .map_or(first, |answer| answer.kind);
    let length = end - start + 1;
    let probabilities: Vec<f64> = answers
        .iter()
        .skip(start)
        .take(length)
        .filter(|answer| answer.kind == winner)
        .filter_map(|answer| answer.kind_probabilities.get(winner).copied())
        .collect();
    let mean = probabilities.iter().sum::<f64>() / probabilities.len().max(1) as f64;
    Some(if winner_count.saturating_mul(2) < length || mean < 0.5 {
        first
    } else {
        winner
    })
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {
    use super::{TokenAnswer, assemble, kind_questions, tokenize, window};

    #[test]
    fn tokenization_keeps_internal_marks_and_peels_trailing_punctuation() {
        let tokens = tokenize("st. bruno's. Karst & Vellum hired");
        let words: Vec<&str> = tokens.iter().map(|token| token.text.as_str()).collect();
        assert_eq!(words, ["st", ".", "bruno's", ".", "Karst", "&", "Vellum", "hired"]);
        assert_eq!(window(&tokens, 0), "<BEGINNING> <BEGINNING> [[st]] . bruno's");
        assert_eq!(window(&tokens, 6), "Karst & [[Vellum]] hired <END>");
    }

    #[test]
    fn offsets_count_unicode_scalars_in_the_exact_source() {
        let tokens = tokenize("é 😀 e\u{301}lan.");
        let found: Vec<(&str, usize, usize)> = tokens.iter().map(|token| (token.text.as_str(), token.start, token.end)).collect();
        assert_eq!(found, [("é", 0, 1), ("😀", 2, 3), ("e\u{301}lan", 4, 9), (".", 9, 10)]);
    }

    #[derive(serde::Deserialize)]
    struct KeyLine { text: String, value: KeyValue }
    #[derive(serde::Deserialize)]
    struct KeyValue { entities: Vec<KeyName> }
    #[derive(serde::Deserialize)]
    struct KeyName { start: usize, end: usize }

    /// Ticket 0164: a key name is reachable when one word starts where it starts and one ends where it ends.
    #[test]
    fn todays_words_reach_322_of_the_name_keys_372_names() {
        let (mut reached, mut names, mut words) = (0, 0, 0);
        for line in include_str!("../../../../specification/fixtures/recognize/names.jsonl").lines() {
            let line: KeyLine = serde_json::from_str(line).unwrap();
            let tokens = tokenize(&line.text);
            words += tokens.len();
            for name in line.value.entities {
                names += 1;
                let edge = |at: fn(&super::Token) -> usize, place| tokens.iter().any(|token| at(token) == place);
                reached += usize::from(edge(|token| token.start, name.start) && edge(|token| token.end, name.end));
            }
        }
        assert_eq!((reached, names, words), (322, 372, 1735));
    }

    fn answer(detected: bool, detection_probability: f64, kind: usize, probabilities: &[f64]) -> TokenAnswer {
        TokenAnswer { detected, detection_probability, kind, kind_probabilities: probabilities.to_vec() }
    }

    #[test]
    fn assembly_votes_rounds_cuts_inclusively_and_keeps_repeated_names() {
        let text = "Ada Lovelace met Ada Lovelace";
        let tokens = tokenize(text);
        let answers = [
            answer(true, 0.8, 0, &[0.8, 0.2]),
            answer(true, 0.9, 0, &[0.7, 0.3]),
            answer(false, 0.1, 1, &[0.2, 0.8]),
            answer(true, 0.8, 0, &[0.8, 0.2]),
            answer(true, 0.9, 1, &[0.7, 0.3]),
        ];
        let names = assemble(text, &tokens, &answers, &["person".into(), "place".into()], 0.6);
        assert_eq!(names.len(), 2);
        assert_eq!((names[0].start, names[0].end, names[0].strength), (0, 12, 0.6));
        assert_eq!((names[1].start, names[1].end), (17, 29));
    }

    #[test]
    fn tied_kind_vote_keeps_the_first_encountered_kind() {
        let text = "Ada Labs";
        let tokens = tokenize(text);
        let answers = [
            answer(true, 1.0, 0, &[0.9, 0.1]),
            answer(true, 1.0, 1, &[0.1, 0.9]),
        ];
        let names = assemble(text, &tokens, &answers, &["place".into(), "person".into()], 0.5);
        assert_eq!(names[0].kind, "place");
    }

    #[test]
    fn one_kind_needs_no_model_choice() {
        let tokens = tokenize("Ada");
        assert!(kind_questions(&tokens, &[("person".into(), None)]).unwrap().is_empty());
    }

    #[test]
    fn no_names_connectors_and_trailing_possessives_follow_the_fixed_policy() {
        let text = "Lessing 's play and Acme";
        let tokens = tokenize(text);
        let answers = [
            answer(true, 1.0, 0, &[1.0]),
            answer(true, 1.0, 0, &[1.0]),
            answer(false, 1.0, 0, &[1.0]),
            answer(false, 1.0, 0, &[1.0]),
            answer(true, 1.0, 0, &[1.0]),
        ];
        let names = assemble(text, &tokens, &answers, &["person".into()], 0.5);
        assert_eq!(names.iter().map(|name| name.name.as_str()).collect::<Vec<_>>(), ["Lessing", "Acme"]);
        let none = vec![answer(false, 1.0, 0, &[1.0]); tokens.len()];
        assert!(assemble(text, &tokens, &none, &["person".into()], 0.5).is_empty());
    }
}
