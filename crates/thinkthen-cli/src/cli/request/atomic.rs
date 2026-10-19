//! Command flag capture; public request admission owns question and list grammar.
use crate::cli::args::{ChooseArguments, Meanings, TagArguments};
use crate::failure::Failure;
use thinkthen::{Description, Question, RecordOption, RequestOptions, RequestQuestion};

pub(super) fn meanings(options: &mut RequestOptions, meanings: &Meanings) -> Result<(), Failure> {
    // The public authored-content grammar retains explicit blank values so the
    // native resolver supplies the existing meaning-specific refusal sentence.
    let mut fields = serde_json::Map::new();
    for (name, value) in [("yes", &meanings.yes), ("no", &meanings.no)] {
        if let Some(value) = value {
            fields.insert(name.to_owned(), serde_json::Value::String(value.clone()));
        }
    }
    let captured: RequestOptions = serde_json::from_value(serde_json::Value::Object(fields))
        .map_err(|_| Failure::Defect("command meanings did not match their public grammar"))?;
    options.yes = captured.yes;
    options.no = captured.no;
    Ok(())
}

pub(super) fn bare(names: &[String]) -> Vec<RecordOption> {
    names.iter().map(|name| RecordOption { name: name.clone(), description: None }).collect()
}

fn described(entries: &[String], label: bool) -> Result<Vec<RecordOption>, Failure> {
    entries.iter().map(|entry| {
        let (name, text) = entry.split_once('=').ok_or(if label {
            Failure::LabelWithoutSign
        } else { Failure::OptionWithoutSign })?;
        let description = if text.trim().is_empty() { None } else {
            Some(Description::text(text).map_err(Failure::from)?)
        };
        Ok(RecordOption { name: name.to_owned(), description })
    }).collect()
}

pub(super) fn choice_options(arguments: &ChooseArguments) -> Result<Option<Vec<RecordOption>>, Failure> {
    let listed = !arguments.options.is_empty();
    let explained = !arguments.described.is_empty();
    if listed && explained { return Err(Failure::OptionWithList); }
    if arguments.options_pointer.is_some() && (listed || explained) {
        return Err(Failure::OptionsWithList);
    }
    if listed { Ok(Some(bare(&arguments.options))) }
    else if explained { described(&arguments.described, false).map(Some) }
    else { Ok(None) }
}

pub(super) fn tag_options(arguments: &TagArguments) -> Result<Option<Vec<RecordOption>>, Failure> {
    let listed = !arguments.labels.is_empty();
    let explained = !arguments.described.is_empty();
    if listed && explained { return Err(Failure::LabelWithList); }
    if listed { Ok(Some(bare(&arguments.labels))) }
    else if explained { described(&arguments.described, true).map(Some) }
    else { Ok(None) }
}

pub(super) fn choose_selector(arguments: &ChooseArguments) -> Result<RequestQuestion, Failure> {
    if arguments.question.starts_with('@') {
        return Ok(super::selector(&arguments.question));
    }
    if arguments.options_pointer.is_some() {
        return Question::choose_records(&arguments.question)
            .map(|question| RequestQuestion::Definition { value: question.into() })
            .map_err(Failure::from);
    }
    Ok(super::selector(&arguments.question))
}
