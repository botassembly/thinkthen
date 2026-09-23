//! Recognition command and question-file precedence.

use std::fs;

use crate::args::{Common, RecognizeArguments};
use crate::core::{
    Description, Framing, RecognizeConfigError, RecognizeKinds, RecognizeSpec, RelationRule,
};
use crate::failure::Failure;
use crate::table::Kind as TableKind;

pub(super) fn settle(arguments: &RecognizeArguments) -> Result<RecognizeSpec, Failure> {
    let file = arguments
        .kinds
        .first()
        .and_then(|kind| kind.strip_prefix('@'));
    let mut spec = if let Some(path) = file {
        if arguments.kinds.len() != 1
            || !arguments.described.is_empty()
            || !arguments.relations.is_empty()
        {
            return Err(error(true, RecognizeConfigError::Shape));
        }
        let text = fs::read_to_string(path).map_err(Failure::OpenQuestionFile)?;
        RecognizeSpec::parse(&text).map_err(|why| error(true, why))?
    } else {
        let kinds = command_kinds(arguments)?;
        let relations = arguments
            .relations
            .iter()
            .map(|held| command_rule(held))
            .collect::<Result<Vec<_>, _>>()?;
        RecognizeSpec::from_parts(
            kinds,
            relations,
            arguments.threshold.as_deref(),
            arguments.relation_threshold.as_deref(),
        )
        .map_err(|why| error(false, why))?
    };
    if file.is_some() {
        if let Some(threshold) = arguments.threshold.as_deref() {
            spec.threshold = threshold
                .parse()
                .map_err(|_| error(false, RecognizeConfigError::Threshold))?;
        }
        if let Some(threshold) = arguments.relation_threshold.as_deref() {
            spec.relation_threshold = threshold
                .parse()
                .map_err(|_| error(false, RecognizeConfigError::Threshold))?;
        }
        if !spec.threshold.is_cut() || !spec.relation_threshold.is_cut() {
            return Err(error(false, RecognizeConfigError::Threshold));
        }
    }
    Ok(spec)
}

fn command_kinds(arguments: &RecognizeArguments) -> Result<RecognizeKinds, Failure> {
    if !arguments.kinds.is_empty() && !arguments.described.is_empty() {
        return Err(error(false, RecognizeConfigError::Kinds));
    }
    if !arguments.described.is_empty() {
        return arguments
            .described
            .iter()
            .map(|entry| {
                let (name, description) = entry
                    .split_once('=')
                    .ok_or_else(|| error(false, RecognizeConfigError::Kinds))?;
                Ok((name.to_owned(), Some(Description::text(description))))
            })
            .collect();
    }
    let names = if arguments.kinds.is_empty() {
        return Ok(default_kinds());
    } else {
        arguments.kinds.clone()
    };
    Ok(names.into_iter().map(|name| (name, None)).collect())
}

fn default_kinds() -> RecognizeKinds {
    vec![
        (
            "person".to_owned(),
            Some(Description::text("Part of a person's name.")),
        ),
        (
            "organization".to_owned(),
            Some(Description::text(
                "Part of the name of an organization: a company, band, team, agency, government body, or media outlet.",
            )),
        ),
        (
            "place".to_owned(),
            Some(Description::text(
                "Part of the name of a place: a country, region, city, or geographic feature.",
            )),
        ),
    ]
}

fn command_rule(text: &str) -> Result<RelationRule, Failure> {
    let (name, ends) = text
        .split_once('=')
        .ok_or_else(|| error(false, RecognizeConfigError::Relation))?;
    let (source, target) = ends
        .split_once(':')
        .ok_or_else(|| error(false, RecognizeConfigError::Relation))?;
    Ok(RelationRule {
        name: name.to_owned(),
        source: source.to_owned(),
        target: target.to_owned(),
        either: false,
        reads: name.replace('_', " "),
    })
}

pub(super) fn error(file: bool, error: RecognizeConfigError) -> Failure {
    Failure::Recognize(crate::cli::failure::recognize::Error::Config { file, error })
}

pub(super) fn framing(common: &Common) -> Framing {
    if common.lines {
        Framing::Lines
    } else if common.jsonl {
        Framing::Jsonl
    } else if common.csv {
        Framing::Csv
    } else if common.tsv {
        Framing::Tsv
    } else {
        Framing::Document
    }
}

pub(super) fn table_kind(common: &Common) -> Option<TableKind> {
    common
        .csv
        .then_some(TableKind::Csv)
        .or_else(|| common.tsv.then_some(TableKind::Tsv))
}
