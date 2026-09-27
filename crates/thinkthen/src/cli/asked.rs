//! What one verb was asked, gathered from the command line and from a file.
//!
//! The file is read here, at the edge, and the core is handed its text. The
//! rule that settles which value wins lives in `thinkthen-core`, so a library
//! over that crate reaches the question a shell user reaches.

use std::fs;

use crate::core::{Cutting, Description, Json, QuestionFile, Resolved, Typed, Verb, resolve};

use crate::args::{
    ChooseArguments, Common, DecideArguments, FilterArguments, Meanings, RankArguments,
    ScoreArguments, TagArguments,
};
use crate::failure::Failure;

/// The path the first argument names, when it is `@` and a path.
fn path_of(question: &str) -> Option<&str> {
    question.strip_prefix('@')
}

/// A question file, if the first argument names one, and a `decide` file's raw `batch`.
type Top = (Option<QuestionFile>, Option<Json>);

/// Read and check the question file the first argument names, if it names
/// one, with a `decide` file's raw `batch`.
fn read_top(question: &str) -> Result<Top, Failure> {
    let Some(path) = path_of(question) else {
        return Ok((None, None));
    };
    let text = fs::read_to_string(path).map_err(Failure::OpenQuestionFile)?;
    let (file, batch) = QuestionFile::parse_top(&text)?;
    Ok((Some(file), batch))
}

fn read(question: &str) -> Result<Option<QuestionFile>, Failure> {
    Ok(read_top(question)?.0)
}

/// The question text the command line carries, or `None` when a file holds it.
const fn typed_text(question: &str, from_file: bool) -> Option<&str> {
    if from_file { None } else { Some(question) }
}

/// The pointers `--field` named, or `None` when it named none.
fn fields(common: &Common) -> Option<Vec<String>> {
    (!common.field.is_empty()).then(|| common.field.clone())
}

/// Settle one yes/no question, which is what all three record verbs ask.
///
/// `decide`, `filter`, and `rank` send the same request and read the same
/// question file. They differ in the rule each one can act on, which
/// [`Cutting`] carries, and in what each one prints.
fn yes_no(
    caller: &'static str,
    question: &str,
    meanings: &Meanings,
    threshold: Option<&String>,
    cutting: Cutting,
    common: &Common,
) -> Result<(Resolved, Option<Json>), Failure> {
    let (file, batch) = read_top(question)?;
    let typed = Typed {
        threshold: threshold.cloned(),
        yes: meanings.yes.clone(),
        no: meanings.no.clone(),
        model: common.model.clone(),
        on: fields(common),
        cutting,
        ..Typed::default()
    };
    resolve(
        Verb::Decide,
        typed_text(question, file.is_some()),
        file.as_ref(),
        &typed,
    )
    .map_err(|error| match error {
        crate::core::QuestionFileError::VerbMismatch { held, .. } if caller != "decide" => {
            Failure::QuestionKind {
                command: caller,
                held: held.word(),
            }
        }
        other => Failure::Question(other),
    })
    .map(|resolved| (resolved, batch))
}

/// Settle everything `decide` was asked.
pub(crate) fn decide(arguments: &DecideArguments) -> Result<(Resolved, Option<Json>), Failure> {
    yes_no(
        "decide",
        &arguments.question,
        &arguments.meanings,
        arguments.threshold.as_ref(),
        Cutting::AsTheVerbAllows,
        &arguments.common,
    )
}

/// Settle everything `filter` was asked, which takes a single cut alone.
pub(crate) fn filter(arguments: &FilterArguments) -> Result<(Resolved, Option<Json>), Failure> {
    yes_no(
        "filter",
        &arguments.question,
        &arguments.meanings,
        arguments.threshold.as_ref(),
        Cutting::OneCut,
        &arguments.common,
    )
}

/// Settle everything `rank` was asked, which reads no rule at all.
pub(crate) fn rank(arguments: &RankArguments) -> Result<(Resolved, Option<Json>), Failure> {
    yes_no(
        "rank",
        &arguments.question,
        &arguments.meanings,
        arguments.threshold.as_ref(),
        Cutting::NoRule,
        &arguments.common,
    )
}

/// Settle everything `choose` was asked.
pub(crate) fn choose(arguments: &ChooseArguments) -> Result<Resolved, Failure> {
    let file = read(&arguments.question)?;
    let listed = !arguments.options.is_empty();
    let described = !arguments.described.is_empty();
    if listed && described {
        return Err(Failure::OptionWithList);
    }
    if arguments.options_pointer.is_some() && (listed || described) {
        return Err(Failure::OptionsWithList);
    }
    let labels = match (listed, described) {
        (true, _) => Some(
            arguments
                .options
                .iter()
                .map(|name| (name.clone(), None))
                .collect(),
        ),
        (_, true) => Some(
            arguments
                .described
                .iter()
                .map(|entry| {
                    split(entry).map(|(name, described)| (name, described.map(Description::text)))
                })
                .collect::<Result<Vec<_>, Failure>>()?,
        ),
        _ => None,
    };
    let typed = Typed {
        threshold: arguments.threshold.clone(),
        labels,
        model: arguments.common.model.clone(),
        on: fields(&arguments.common),
        options_from_record: arguments.options_pointer.is_some(),
        ..Typed::default()
    };
    Ok(resolve(
        Verb::Choose,
        typed_text(&arguments.question, file.is_some()),
        file.as_ref(),
        &typed,
    )?)
}

/// Settle everything `tag` was asked.
pub(crate) fn tag(arguments: &TagArguments) -> Result<Resolved, Failure> {
    let file = read(&arguments.question)?;
    let listed = !arguments.labels.is_empty();
    let described = !arguments.described.is_empty();
    if listed && described {
        return Err(Failure::LabelWithList);
    }
    let labels = match (listed, described) {
        (true, _) => Some(
            arguments
                .labels
                .iter()
                .map(|name| (name.clone(), None))
                .collect(),
        ),
        (_, true) => Some(
            arguments
                .described
                .iter()
                .map(|entry| {
                    split_label(entry)
                        .map(|(name, described)| (name, described.map(Description::text)))
                })
                .collect::<Result<Vec<_>, Failure>>()?,
        ),
        _ => None,
    };
    let typed = Typed {
        threshold: arguments.threshold.clone(),
        labels,
        model: arguments.common.model.clone(),
        on: fields(&arguments.common),
        ..Typed::default()
    };
    Ok(resolve(
        Verb::Tag,
        typed_text(&arguments.question, file.is_some()),
        file.as_ref(),
        &typed,
    )?)
}

/// Settle everything `score` was asked.
pub(crate) fn score(arguments: &ScoreArguments) -> Result<Resolved, Failure> {
    let file = read(&arguments.question)?;
    let typed = Typed {
        // `score` takes no rule, and the core writes that refusal, so the
        // value reaches it rather than being refused twice.
        threshold: arguments.threshold.clone(),
        labels: (!arguments.levels.is_empty()).then(|| {
            arguments
                .levels
                .iter()
                .map(|name| (name.clone(), None))
                .collect()
        }),
        model: arguments.common.model.clone(),
        on: fields(&arguments.common),
        ..Typed::default()
    };
    Ok(resolve(
        Verb::Score,
        typed_text(&arguments.question, file.is_some()),
        file.as_ref(),
        &typed,
    )?)
}

/// Split one `--option` entry at its first `=`.
///
/// A blank description is no description, and the core applies that rule, so
/// `LABEL=` asks the same question `LABEL` alone asks.
fn split(entry: &str) -> Result<(String, Option<String>), Failure> {
    let (label, described) = entry.split_once('=').ok_or(Failure::OptionWithoutSign)?;
    Ok((label.to_owned(), Some(described.to_owned())))
}

/// Split one `--label` entry at its first `=`.
fn split_label(entry: &str) -> Result<(String, Option<String>), Failure> {
    let (label, described) = entry.split_once('=').ok_or(Failure::LabelWithoutSign)?;
    Ok((label.to_owned(), Some(described.to_owned())))
}

#[cfg(test)]
mod tests {
    use super::{path_of, split, typed_text};
    use crate::failure::Failure;

    #[test]
    fn a_question_that_begins_with_an_at_sign_names_a_file_and_nothing_else_does() {
        assert_eq!(path_of("@refund.json"), Some("refund.json"));
        assert_eq!(path_of("@"), Some(""));
        assert_eq!(path_of("Does this ask for a refund?"), None);
        assert_eq!(path_of("user@example.test asks for a refund"), None);
        assert_eq!(typed_text("@refund.json", true), None);
        assert_eq!(
            typed_text("asks for a refund", false),
            Some("asks for a refund")
        );
    }

    #[test]
    fn an_option_splits_at_its_first_equals_sign_and_needs_one() {
        let taken = |entry: &str| split(entry).expect("an option with a sign");
        assert_eq!(
            taken("late=It arrived late."),
            ("late".to_owned(), Some("It arrived late.".to_owned()))
        );
        assert_eq!(
            taken("late=a=b"),
            ("late".to_owned(), Some("a=b".to_owned()))
        );
        assert_eq!(taken("late="), ("late".to_owned(), Some(String::new())));
        assert!(matches!(split("late"), Err(Failure::OptionWithoutSign)));
    }
}
