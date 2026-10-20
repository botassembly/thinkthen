//! Capture recognition flags through supported native question builders.
use crate::cli::args::RecognizeArguments;
use crate::failure::Failure;
use crate::{Description, Kind, Recognize, RelationRule, RequestOptions, Threshold};

const KINDS: &str = "recognize takes distinct, nonblank kinds";
const RELATION: &str =
    "a recognize relation has a distinct name, source, target, optional reads, and optional either";
const SHAPE: &str = "a recognize question file is one closed version-one object";
const THRESHOLD: &str = "recognize thresholds are single cuts above zero and at most one";
const BOUNDARY: &str = "boundary_only recognition takes no relations, relation threshold, kind_edge context or relation context";

pub(super) fn inline(arguments: &RecognizeArguments) -> Result<Recognize, Failure> {
    if !arguments.kinds.is_empty() && !arguments.described.is_empty() {
        return Err(Failure::Usage(KINDS));
    }
    // Capture descriptions before parsing rules; blank authored descriptions remain
    // present in the public content carrier, preserving native question identity.
    let kinds = kinds(arguments)?;
    // The command's spelling errors precede native kind validation. Content
    // errors in a syntactically valid rule follow it, as the original CLI did.
    for text in &arguments.relations {
        match text.split_once('=') {
            None if text.contains(':') => return Err(Failure::Usage(RELATION)),
            Some((_, ends)) => {
                let Some((source, target)) = ends.split_once(':') else {
                    return Err(Failure::Usage(RELATION));
                };
                if source.contains(['=', ':']) || target.contains(['=', ':']) {
                    return Err(Failure::Usage(RELATION));
                }
            }
            _ => {}
        }
    }
    let kinds = kinds
        .into_iter()
        .map(|(name, description)| Kind::new(&name, description).map_err(|_| Failure::Usage(KINDS)))
        .collect::<Result<Vec<_>, _>>()?;
    let mut kind_admission = Recognize::builder();
    for kind in &kinds {
        kind_admission = kind_admission.kind(kind.clone())?;
    }
    // Use native admission for reserved names and the complete kind grammar.
    kind_admission.build().map_err(Failure::from)?;
    let mut builder = Recognize::builder();
    for kind in kinds {
        builder = builder.kind(kind)?;
    }
    for text in &arguments.relations {
        let rule = RelationRule::parse_inline(text, false).map_err(|_| Failure::Usage(RELATION))?;
        builder = builder
            .relation(rule)
            .map_err(|_| Failure::Usage(RELATION))?;
    }
    // Native static kind/rule admission precedes threshold and task-wording flags.
    // RequestOptions retains the actual overrides for preview and execution.
    let mut question = builder.build().map_err(Failure::from)?;
    for cut in [&arguments.threshold, &arguments.relation_threshold]
        .into_iter()
        .flatten()
    {
        if cut
            .parse::<Threshold>()
            .ok()
            .and_then(Threshold::cut_value)
            .is_none()
        {
            return Err(Failure::Usage(THRESHOLD));
        }
    }
    for wording in [&arguments.instructions, &arguments.entity_definition]
        .into_iter()
        .flatten()
    {
        crate::Question::decide(wording).map_err(|_| Failure::Usage(SHAPE))?;
    }
    if let Some(mode) = arguments.mode.as_deref() {
        question = question.with_mode(match mode {
            "whole" => crate::RecognitionMode::Whole,
            "boundary_only" => crate::RecognitionMode::BoundaryOnly,
            _ => return Err(Failure::Usage("recognize mode is whole or boundary_only")),
        });
    }
    question = question.with_stage_context(crate::RecognitionStageContext {
        boundary: arguments.boundary_context.clone(),
        kind_edge: arguments.kind_edge_context.clone(),
        relation: arguments.relation_context.clone(),
    });
    if arguments.mode.as_deref() == Some("boundary_only") && arguments.relation_threshold.is_some()
    {
        return Err(Failure::Usage(BOUNDARY));
    }
    // This existing public operation also validates boundary-only stage controls.
    let snippet = arguments
        .snippet_pieces
        .unwrap_or_else(|| question.reading().snippet_pieces());
    question.with_snippet_pieces(snippet).map_err(Failure::from)
}

type Kinds = Vec<(String, Option<Description>)>;

fn kinds(arguments: &RecognizeArguments) -> Result<Kinds, Failure> {
    if arguments.described.is_empty() {
        return Ok(arguments
            .kinds
            .iter()
            .map(|name| (name.clone(), None))
            .collect());
    }
    let entries = arguments
        .described
        .iter()
        .map(|entry| {
            let (name, description) = entry.split_once('=').ok_or(Failure::Recognize(
                crate::failure::recognize::Error::KindWithoutSign,
            ))?;
            Ok(serde_json::json!({ "name": name, "description": description }))
        })
        .collect::<Result<Vec<_>, Failure>>()?;
    let captured: RequestOptions = serde_json::from_value(serde_json::json!({ "labels": entries }))
        .map_err(|_| Failure::Defect("command kinds did not match their public grammar"))?;
    Ok(captured
        .labels
        .unwrap_or_default()
        .into_iter()
        .map(|option| (option.name, option.description))
        .collect())
}

#[cfg(test)]
mod tests {
    //! Stays: command recognition parsing must preserve overrides and original example validation.
    use super::*;
    use crate::cli::args::{Cli, Command};
    use clap::Parser as _;

    fn parse(words: &[&str]) -> RecognizeArguments {
        let cli = Cli::try_parse_from(words).expect("command");
        let Some(Command::Recognize(arguments)) = cli.command else {
            panic!("recognize command")
        };
        arguments
    }

    #[test]
    fn public_inline_kinds_keep_order_blank_descriptions_and_native_identity() {
        let arguments = parse(&[
            "thinkthen",
            "recognize",
            "--kind",
            "place=Where",
            "--kind",
            "person= ",
        ]);
        let question = inline(&arguments).unwrap();
        let native = Recognize::from_json(
            r#"{"version":1,"recognize":{"kinds":{"place":"Where","person":" "}}}"#,
        )
        .unwrap();
        assert_eq!(
            crate::RequestDefinition::from(question)
                .question_sha256()
                .unwrap(),
            crate::RequestDefinition::from(native)
                .question_sha256()
                .unwrap()
        );
    }

    #[test]
    fn public_inline_refusals_keep_command_precedence() {
        for (words, expected) in [
            (
                vec!["thinkthen", "recognize", "ENTITY", "--relation", "bad:rule"],
                RELATION,
            ),
            (
                vec!["thinkthen", "recognize", "ENTITY", "--relation", " "],
                "recognize reserves the kind names none of these, ENTITY and ANY in any ASCII case",
            ),
            (
                vec![
                    "thinkthen",
                    "recognize",
                    "person",
                    "person",
                    "--threshold",
                    "0",
                ],
                KINDS,
            ),
            (
                vec![
                    "thinkthen",
                    "recognize",
                    "person",
                    "--threshold",
                    "0",
                    "--instructions",
                    " ",
                ],
                THRESHOLD,
            ),
            (
                vec![
                    "thinkthen",
                    "recognize",
                    "person",
                    "--instructions",
                    " ",
                    "--mode",
                    "bad",
                ],
                SHAPE,
            ),
            (
                vec![
                    "thinkthen",
                    "recognize",
                    "person",
                    "--mode",
                    "boundary_only",
                    "--relation-threshold",
                    "0.8",
                ],
                BOUNDARY,
            ),
        ] {
            let error = inline(&parse(&words)).unwrap_err();
            assert_eq!(crate::failure::sentence(&error), (2, expected.to_owned()));
        }
    }
}
