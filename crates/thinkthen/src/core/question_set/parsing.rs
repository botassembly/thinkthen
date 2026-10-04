//! Shared closed set grammar, with rank admission before normalization.
use super::{
    NamedQuestion, QuestionSet, QuestionSetError, check_name, json_error, nested, pointer_clash,
    profile, threshold,
};
use crate::core::{Cutting, Json, Pointer, QuestionFile, Typed, Verb, json_line, resolve};

pub(super) fn parse(text: &str, rank: bool) -> Result<QuestionSet, QuestionSetError> {
    let value = Json::parse(text).map_err(json_error)?;
    let Json::Object(members) = &value else {
        return Err(QuestionSetError::NotObject);
    };
    if value.member("questions").is_none() {
        return Err(QuestionSetError::MissingQuestions);
    }
    for (key, _) in members {
        if !["version", "threshold", "profile", "questions", "batch"].contains(&key.as_str()) {
            return Err(QuestionSetError::UnknownKey(key.clone()));
        }
    }
    match value.member("version") {
        Some(Json::Number(number)) if number.as_u64() == Some(1) => {}
        _ => {
            return Err(QuestionSetError::Shape {
                path: "version".to_owned(),
                wanted: "is the number 1",
            });
        }
    }
    if rank && value.member("threshold").is_some() {
        return Err(refused("threshold"));
    }
    let inherited = threshold(&value)?;
    let profile = profile(&value)?;
    let Some(Json::Object(entries)) = value.member("questions") else {
        return Err(QuestionSetError::Shape {
            path: "questions".to_owned(),
            wanted: "is an object",
        });
    };
    if entries.is_empty() {
        return Err(QuestionSetError::Empty);
    }
    let mut questions = Vec::with_capacity(entries.len());
    for (name, held) in entries {
        check_name(name)?;
        member(name, held, rank)?;
        let written = json_line(held).map_err(|_| QuestionSetError::Render)?;
        let file = QuestionFile::parse(&written).map_err(|error| nested(name, error))?;
        let typed = Typed {
            threshold: (file.verb() == Verb::Decide && !file.has_threshold())
                .then(|| inherited.map(|rule| rule.to_string()))
                .flatten(),
            cutting: if rank {
                Cutting::NoRule
            } else {
                Cutting::AsTheVerbAllows
            },
            ..Typed::default()
        };
        let resolved =
            resolve(file.verb(), None, Some(&file), &typed).map_err(|error| nested(name, error))?;
        pointer_clash(name, resolved.on())?;
        let on = if resolved.on().is_empty() {
            vec![Pointer::new("").map_err(|_| QuestionSetError::Render)?]
        } else {
            resolved.on().to_vec()
        };
        let question = resolved
            .question()
            .cloned()
            .ok_or(QuestionSetError::Render)?;
        questions.push(NamedQuestion {
            name: name.clone(),
            question,
            threshold: resolved.threshold(),
            on,
        });
    }
    Ok(QuestionSet {
        questions,
        profile,
        batch: value.member("batch").cloned(),
    })
}

fn refused(path: &str) -> QuestionSetError {
    QuestionSetError::Nested {
        path: path.to_owned(),
        why: "rank question sets take no threshold or on".to_owned(),
    }
}

fn rank_member(name: &str, held: &Json) -> Result<(), QuestionSetError> {
    for key in ["threshold", "on"] {
        if held.member(key).is_some() {
            return Err(refused(&format!("questions.{name}.{key}")));
        }
    }
    if held.member("decide").is_none() {
        return Err(QuestionSetError::Nested {
            path: format!("questions.{name}"),
            why: "rank takes only decide questions".to_owned(),
        });
    }
    Ok(())
}

fn member(name: &str, held: &Json, rank: bool) -> Result<(), QuestionSetError> {
    let Json::Object(fields) = held else {
        return Err(QuestionSetError::Shape {
            path: format!("questions.{name}"),
            wanted: "is one question object",
        });
    };
    if let Some((key, _)) = fields
        .iter()
        .find(|(key, _)| matches!(key.as_str(), "model" | "profile"))
    {
        return Err(QuestionSetError::UnknownKey(format!(
            "questions.{name}.{key}"
        )));
    }
    if rank {
        rank_member(name, held)?;
    }
    Ok(())
}
