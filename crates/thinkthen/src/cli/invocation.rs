//! Map syntax to the shared strict question admission boundary.
use super::options::{self, usage, value};
use crate::{Error, Request};
use clap::ArgMatches;
use serde_json::{Map, Value, json};
use std::io::Read;

pub(super) fn request(function: &str, args: &ArgMatches) -> Result<Request, Error> {
    let mut question = Map::from_iter([("function".into(), function.into())]);
    match function {
        "annotate" => {
            let path = args
                .get_one::<String>("question")
                .ok_or_else(|| usage("toolbox", "a toolbox path", "absent path"))?;
            let file = std::fs::File::open(path)
                .map_err(|_| super::local("the toolbox could not be opened"))?;
            let mut bytes = Vec::new();
            file.take((crate::TOOLBOX_LIMIT + 1) as u64)
                .read_to_end(&mut bytes)
                .map_err(|_| super::local("the toolbox could not be read"))?;
            if bytes.len() > crate::TOOLBOX_LIMIT {
                return Err(usage(
                    "toolbox",
                    "a toolbox within its byte limit",
                    "toolbox exceeds its byte limit",
                ));
            }
            let annotation: crate::core::questions::AnnotateWire = serde_json::from_slice(&bytes)
                .map_err(|error| {
                Error::new(
                    crate::ErrorKind::Question,
                    crate::core::diagnostic::context("toolbox", &error.to_string()),
                )
            })?;
            return overrides(
                Request::new(crate::questions::Annotate::try_from(annotation)?),
                args,
            );
        }
        "recognize" | "relate" => {}
        _ => {
            question.insert(
                "question".into(),
                args.get_one::<String>("question")
                    .cloned()
                    .unwrap_or_default()
                    .into(),
            );
        }
    }
    let function_name: crate::FunctionName = serde_json::from_value(function.into())
        .map_err(|_| usage("function", "a registered function", function))?;
    let spec = function_name.command();
    if let Some((key, field, env, _)) = spec.list {
        let list = options::list(args, key, env)?
            .ok_or_else(|| usage(key, "a supplied list", "absent option"))?;
        question.insert(field.into(), list);
    }
    if function == "relate" {
        question.insert("rules".into(), rules(args)?);
    }
    use crate::core::questions::command::ThresholdOption;
    if !matches!(spec.threshold, ThresholdOption::None) {
        let keys: &[&str] = if matches!(spec.threshold, ThresholdOption::Band) {
            &["threshold", "low", "high"]
        } else {
            &["threshold"]
        };
        let values = options::group(args, keys);
        let mut values = values.into_iter();
        let threshold = values.next().flatten();
        let low = values.next().flatten();
        let high = values.next().flatten();
        if threshold.is_some() && (low.is_some() || high.is_some()) {
            return Err(usage(
                "threshold",
                "threshold or low and high",
                "both forms",
            ));
        }
        let threshold = match (threshold, low, high) {
            (Some(cut), None, None) => Some(options::number("threshold", &cut)?),
            (None, Some(low), Some(high)) => Some(
                json!({"low":options::number("low", &low)?,"high":options::number("high", &high)?}),
            ),
            (None, None, None) => None,
            _ => return Err(usage("threshold", "low and high together", "one bound")),
        };
        if let Some(threshold) = threshold {
            question.insert("threshold".into(), threshold);
        }
    }
    if function == "rank"
        && let Some(top) = value(args, "top")
    {
        question.insert("top".into(), options::number("top", &top)?);
    }
    overrides(
        Request::from_json(&json!({"question":question}).to_string())?,
        args,
    )
}

fn rules(args: &ArgMatches) -> Result<Value, Error> {
    let mut rules = if let Some(values) = args.get_many::<String>("rule") {
        let values: Vec<_> = values.collect();
        Value::Array(
            values
                .chunks_exact(3)
                .map(|chunk| {
                    let mut fields = chunk.iter();
                    json!({"relation":fields.next(),"source":fields.next(),"target":fields.next()})
                })
                .collect(),
        )
    } else {
        std::env::var("THINKTHEN_RULES")
            .ok()
            .map(|text| {
                serde_json::from_str(&text)
                    .map_err(|_| usage("rules", "a JSON array", "invalid JSON (contents redacted)"))
            })
            .transpose()?
            .ok_or_else(|| usage("rule", "at least one relation rule", "absent option"))?
    };
    if args.get_many::<String>("rule").is_none() {
        let array = rules
            .as_array_mut()
            .ok_or_else(|| usage("rules", "an array", "another JSON type (contents redacted)"))?;
        for rule in array {
            let fields = rule.as_object_mut().ok_or_else(|| {
                usage(
                    "rules",
                    "an array of objects",
                    "another JSON type (contents redacted)",
                )
            })?;
            const FIELDS: &[&str] = &["name", "source", "target", "either"];
            if let Some(name) = fields.keys().find(|name| !FIELDS.contains(&name.as_str())) {
                return Err(options::unknown("rules", name, FIELDS));
            }
            let name = fields
                .remove("name")
                .ok_or_else(|| usage("rules.name", "one name field", "absent field"))?;
            fields.insert("relation".into(), name);
        }
    }
    if let Some(either) = options::boolean("either", value(args, "either"))? {
        let array = rules
            .as_array_mut()
            .ok_or_else(|| usage("rules", "an array", "another JSON type (contents redacted)"))?;
        for rule in array {
            let fields = rule.as_object_mut().ok_or_else(|| {
                usage(
                    "rules",
                    "an array of objects",
                    "another JSON type (contents redacted)",
                )
            })?;
            fields.insert("either".into(), either.into());
        }
    }
    Ok(rules)
}

pub(super) fn overrides(request: Request, args: &ArgMatches) -> Result<Request, Error> {
    let evidence = value(args, "evidence");
    let name = if args.try_get_one::<String>("name").is_ok() {
        value(args, "name")
    } else {
        None
    };
    if evidence.is_none() && name.is_none() {
        return Ok(request);
    }
    let mut encoded = serde_json::to_value(request)
        .map_err(|_| super::local("the request could not be encoded"))?;
    if let Some(evidence) = evidence {
        encoded
            .get_mut("question")
            .and_then(Value::as_object_mut)
            .ok_or_else(|| super::local("the request question could not be encoded"))?
            .insert("evidence".into(), evidence.into());
    }
    if let Some(name) = name {
        encoded
            .as_object_mut()
            .ok_or_else(|| super::local("the request could not be encoded"))?
            .insert("name".into(), name.into());
    }
    Request::from_json(&encoded.to_string())
}
