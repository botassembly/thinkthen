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
unsafe fn choices(value: ChoicesV1) -> Result<Object, Failure> {
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
    Ok(object)
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
pub(super) unsafe fn build(spec: &QuestionSpecV1) -> Result<String, Failure> {
    let mut body = Object::default();
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
                let raw: Box<RawValue> = serde_json::from_str(&question.json)
                    .map_err(|_| Failure::defect("saved question cannot be read"))?;
                members.put(read::string(member.name)?, raw)?;
            }
            body.put("version", 1)?;
            body.put("questions", members)?;
        } else if spec.kind >= 9 {
            let mut inner = Object::default();
            if spec.kind == 9 {
                inner.put("kinds", choices(spec.kinds)?)?;
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
                6 => "rank",
                _ => "find",
            };
            body.put(verb, read::content(spec.text)?)?;
            if spec.choices.len != 0 {
                body.put(
                    match spec.kind {
                        2 => "options",
                        3 => "labels",
                        4 => "levels",
                        _ => "options",
                    },
                    choices(spec.choices)?,
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
    serde_json::to_string(&body)
        .map_err(|_| Failure::defect("question descriptor cannot be written"))
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
        if spec.none != 0 {
            body.put("none", true)?;
        }
        if spec.on.len != 0 {
            body.put("on", read::strings(spec.on)?)?;
        }
    }
    Ok(())
}

pub(super) unsafe fn plain(
    spec: &QuestionSpecV1,
) -> Result<crate::current::QuestionHandle, Failure> {
    read::flag(spec.none)?;
    if spec.choices.len != 0
        || spec.members.len != 0
        || spec.on.len != 0
        || spec.kinds.len != 0
        || spec.relations.len != 0
        || spec.yes.present != 0
        || spec.no.present != 0
        || spec.model.present != 0
        || spec.profile.present != 0
        || spec.batch.present != 0
        || spec.batch_max != 0
        || spec.name_pointer.present != 0
        || spec.kind_pointer.present != 0
        || spec.threshold.kind > 1
        || spec.relation_threshold.kind != 0
        || (spec.kind == 6 && spec.none != 0)
    {
        return Err(Failure::usage(
            "current plain rank/find accept text and find's none only",
        ));
    }
    // SAFETY: the active text buffer follows the descriptor's counted extent.
    let content = unsafe { read::content(spec.text) }?;
    let crate::current::Content::Text(text) = content else {
        return Err(Failure::usage("current plain rank/find require text"));
    };
    let mut json = Object::default();
    json.put(if spec.kind == 6 { "rank" } else { "find" }, &text)?;
    if spec.none != 0 {
        json.put("none", true)?;
    }
    let json = serde_json::to_string(&json)
        .map_err(|_| Failure::defect("native plain question could not be written"))?;
    crate::current::plain(spec.kind, text, spec.none != 0, json)
}
