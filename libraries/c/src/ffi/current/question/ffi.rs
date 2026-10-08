//! Serialize descriptors into the existing native grammar, preserving order.
#![allow(
    unsafe_code,
    reason = "question descriptors contain counted caller buffers and member handles"
)]
#![deny(unsafe_op_in_unsafe_fn)]
use super::{
    carriers::{ChoicesV1, QuestionSpecV1, RelationsV1, RuleV1},
    read,
};
use crate::failures::Failure;
use serde::Serialize;
use serde::ser::SerializeMap;
use serde_json::value::RawValue;
#[derive(Default)]
struct Object(Vec<(String, Box<RawValue>)>);
impl Serialize for Object {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = s.serialize_map(Some(self.0.len()))?;
        for (k, v) in &self.0 {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}
impl Object {
    fn put(&mut self, key: &str, value: impl Serialize) -> Result<(), Failure> {
        let json = serde_json::to_string(&value)
            .map_err(|_| Failure::usage("descriptor cannot be serialized"))?;
        let raw = RawValue::from_string(json)
            .map_err(|_| Failure::defect("descriptor serialization failed"))?;
        self.0.push((key.to_owned(), raw));
        Ok(())
    }
}
unsafe fn choices(value: ChoicesV1, score: bool) -> Result<Box<RawValue>, Failure> {
    // SAFETY: caller provided the live counted choice array.
    let values = unsafe { read::slice(value.data, value.len) }?;
    for choice in values {
        read::flag(choice.description.present)?;
        if read::flag(choice.weight.present)? {
            return Err(Failure::usage(
                "current native choices do not accept authored weights",
            ));
        }
    }
    if score && values.iter().all(|choice| choice.description.present == 0) {
        let names = values
            .iter()
            .map(|choice| {
                // SAFETY: each initialized label has its counted string extent.
                unsafe { read::string(choice.name) }
            })
            .collect::<Result<Vec<_>, _>>()?;
        return serde_json::value::to_raw_value(&names)
            .map_err(|_| Failure::defect("score levels could not be written"));
    }
    if score && values.iter().any(|choice| choice.description.present == 0) {
        return Err(Failure::usage("give every level a description, or none"));
    }
    let mut object = Object::default();
    // SAFETY: declared array and entry buffers follow the header extents.
    for choice in unsafe { read::slice(value.data, value.len) }? {
        if read::flag(choice.weight.present)? {
            return Err(Failure::usage(
                "current native choices do not accept authored weights",
            ));
        }
        // SAFETY: active fields are counted UTF-8/content.
        unsafe {
            object.put(
                read::string(choice.name)?,
                read::optional_content(choice.description)?,
            )?;
        }
    }
    serde_json::value::to_raw_value(&object)
        .map_err(|_| Failure::defect("question choices could not be written"))
}
fn rule(object: &mut Object, key: &str, value: RuleV1) -> Result<(), Failure> {
    match value.kind {
        0 | 1 => Ok(()),
        2 if value.low.is_finite() => object.put(key, value.low),
        3 if value.low.is_finite() && value.high.is_finite() => {
            object.put(key, format!("{}:{}", value.low, value.high))
        }
        _ => Err(Failure::usage("invalid threshold descriptor")),
    }
}
unsafe fn relations(value: RelationsV1) -> Result<Vec<Object>, Failure> {
    // SAFETY: array and strings follow counted extents.
    unsafe { read::slice(value.data, value.len) }?
        .iter()
        .map(|r| {
            let mut held = Object::default();
            // SAFETY: initialized entries have readable counted fields.
            unsafe {
                held.put("name", read::string(r.name)?)?;
                held.put("source", read::string(r.source)?)?;
                held.put("target", read::string(r.target)?)?;
                if let Some(reads) = read::optional_string(r.reads)? {
                    held.put("reads", reads)?;
                }
            }
            if read::flag(r.either)? {
                held.put("either", true)?;
            }
            if read::flag(r.single)? {
                held.put("single", true)?;
            }
            Ok(held)
        })
        .collect()
}
pub(super) unsafe fn build(
    spec: &QuestionSpecV1,
    author: Option<&crate::current::author::Author>,
    task: Option<crate::ffi::carriers::RecognitionTaskV1>,
) -> Result<String, Failure> {
    let mut body = Object::default();
    if task.is_some() && spec.kind != 9 {
        return Err(Failure::usage(
            "recognition task requires a recognition question",
        ));
    }
    if !(1..=10).contains(&spec.kind) {
        return Err(Failure::usage("invalid question kind"));
    }
    read::flag(spec.batch.present)?;
    read::flag(spec.none)?;
    read::flag(spec.batch_max)?;
    if spec.batch.present != 0 && spec.batch_max != 0 {
        return Err(Failure::usage("batch and batch_max conflict"));
    }
    let set = spec.kind == 8 || (spec.kind == 6 && spec.members.len != 0);
    // SAFETY: all descriptor fields obey the header's active-field storage rules.
    unsafe {
        if set {
            let mut members = Object::default();
            for member in read::slice(spec.members.data, spec.members.len)? {
                let question = read::reference(member.question)?;
                if question.json.is_empty() {
                    return Err(Failure::usage(
                        "native named set members await authored serialization",
                    ));
                }
                let raw: Box<RawValue> = serde_json::from_str(&question.json)
                    .map_err(|_| Failure::defect("saved question cannot be read"))?;
                members.put(read::string(member.name)?, raw)?;
            }
            body.put("version", 1)?;
            body.put("questions", members)?;
        } else if spec.kind >= 9 {
            let mut inner = Object::default();
            if spec.kind == 9 {
                inner.put("kinds", choices(spec.kinds, false)?)?;
                if let Some(task) = task {
                    if let Some(text) = read::optional_string(task.instructions)? {
                        inner.put("instructions", text)?;
                    }
                    if let Some(text) = read::optional_string(task.entity_definition)? {
                        inner.put("entity_definition", text)?;
                    }
                }
            }
            inner.put("relations", relations(spec.relations)?)?;
            if spec.name_pointer.present != 0 || spec.kind_pointer.present != 0 {
                let mut fields = Object::default();
                if let Some(name) = read::optional_string(spec.name_pointer)? {
                    fields.put("name", name)?;
                }
                if let Some(kind) = read::optional_string(spec.kind_pointer)? {
                    fields.put("kind", kind)?;
                }
                inner.put("fields", fields)?;
            }
            body.put("version", 1)?;
            body.put(
                if spec.kind == 9 {
                    "recognize"
                } else {
                    "relate"
                },
                inner,
            )?;
        } else {
            let verb = match spec.kind {
                1 | 5 => "decide",
                2 => "choose",
                3 => "tag",
                4 => "score",
                6 if spec.choices.len != 0 => "score",
                6 => "decide",
                _ => "find",
            };
            body.put(verb, read::content(spec.text)?)?;
            if spec.choices.len != 0 {
                body.put(
                    match spec.kind {
                        2 => "options",
                        3 => "labels",
                        4 | 6 => "levels",
                        _ => "options",
                    },
                    choices(spec.choices, spec.kind == 4 || spec.kind == 6)?,
                )?;
            }
            if let Some(yes) = read::optional_content(spec.yes)? {
                body.put("true", yes)?;
            }
            if let Some(no) = read::optional_content(spec.no)? {
                body.put("false", no)?;
            }
        }
        controls(&mut body, spec, set)?;
    }
    append_author(&mut body, author)?;
    serde_json::to_string(&body)
        .map_err(|_| Failure::defect("question descriptor cannot be written"))
}

fn append_author(
    body: &mut Object,
    author: Option<&crate::current::author::Author>,
) -> Result<(), Failure> {
    if let Some(author) = author {
        if let Some(name) = &author.name {
            body.put("name", name.as_str())?;
        }
        if let Some(version) = author.version {
            body.put("wording_version", version.get())?;
        }
        if let Some(schema) = &author.item {
            body.put("item_schema", schema)?;
        }
        if let Some(schema) = &author.context {
            body.put("context_schema", schema)?;
        }
    }
    Ok(())
}

unsafe fn controls(body: &mut Object, spec: &QuestionSpecV1, set: bool) -> Result<(), Failure> {
    // SAFETY: shared fields obey the same active-payload counted storage contract.
    unsafe {
        if !set && spec.members.len != 0 {
            return Err(Failure::usage("members require annotate or rank set"));
        }
        if spec.kind < 9 && (spec.kinds.len != 0 || spec.relations.len != 0) {
            return Err(Failure::usage("kinds/relations require entity functions"));
        }
        if (set || spec.kind >= 9)
            && (spec.text.kind != 0
                || spec.yes.present != 0
                || spec.no.present != 0
                || spec.choices.len != 0)
        {
            return Err(Failure::usage("irrelevant question fields must be empty"));
        }
        read::flag(spec.name_pointer.present)?;
        read::flag(spec.kind_pointer.present)?;
        if spec.kind != 10 && (spec.name_pointer.present != 0 || spec.kind_pointer.present != 0) {
            return Err(Failure::usage("field pointers require relate"));
        }
        rule(body, "threshold", spec.threshold)?;
        if spec.threshold.kind == 1 && matches!(spec.kind, 1 | 3 | 5 | 9 | 10) {
            return Err(Failure::usage("this question requires a threshold"));
        }
        if spec.kind == 9 && spec.relation_threshold.kind == 1 {
            return Err(Failure::usage(
                "native recognition requires a relation threshold",
            ));
        }
        if spec.relation_threshold.kind > 1 {
            rule(body, "relation_threshold", spec.relation_threshold)?;
        }
        if let Some(model) = read::optional_string(spec.model)? {
            body.put("model", model)?;
        }
        if let Some(profile) = read::optional_string(spec.profile)? {
            body.put("profile", profile)?;
        }
        if spec.batch.present != 0 {
            body.put("batch", spec.batch.value)?;
        }
        if spec.batch_max != 0 {
            body.put("batch", "max")?;
        }
        if spec.none != 0 && spec.kind != 7 {
            body.put("none", true)?;
        }
        if spec.on.len != 0 {
            body.put("on", read::strings(spec.on)?)?;
        }
    }
    Ok(())
}

pub(super) unsafe fn descriptor(
    spec: &QuestionSpecV1,
) -> Result<crate::current::QuestionData, Failure> {
    // SAFETY: called only after native validation of live initialized descriptors.
    unsafe {
        let members = read::slice(spec.members.data, spec.members.len)?
            .iter()
            .map(|member| {
                Ok((
                    read::string(member.name)?.to_owned(),
                    read::reference(member.question)?.clone(),
                ))
            })
            .collect::<Result<Vec<_>, Failure>>()?;
        let relations = read::slice(spec.relations.data, spec.relations.len)?
            .iter()
            .map(|r| {
                Ok(crate::current::descriptors::Relation {
                    name: read::string(r.name)?.to_owned(),
                    source: read::string(r.source)?.to_owned(),
                    target: read::string(r.target)?.to_owned(),
                    reads: read::optional_string(r.reads)?,
                    either: read::flag(r.either)?,
                    single: read::flag(r.single)?,
                })
            })
            .collect::<Result<Vec<_>, Failure>>()?;
        Ok(crate::current::QuestionData {
            kind: spec.kind,
            text: if spec.kind < 8 && spec.members.len == 0 {
                Some(read::content(spec.text)?)
            } else {
                None
            },
            yes: read::optional_content(spec.yes)?,
            no: read::optional_content(spec.no)?,
            choices: read::choices(spec.choices)?,
            threshold: spec.threshold,
            relation_threshold: spec.relation_threshold,
            model: read::optional_string(spec.model)?,
            profile: read::optional_string(spec.profile)?,
            batch: if read::flag(spec.batch.present)? {
                Some(spec.batch.value)
            } else {
                None
            },
            batch_max: read::flag(spec.batch_max)?,
            none: read::flag(spec.none)?,
            on: read::strings(spec.on)?,
            members,
            kinds: read::choices(spec.kinds)?,
            relations,
            name_pointer: read::optional_string(spec.name_pointer)?,
            kind_pointer: read::optional_string(spec.kind_pointer)?,
        })
    }
}
