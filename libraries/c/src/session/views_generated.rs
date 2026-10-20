// Generated from Rust-derived complete result schema; do not edit.
#![allow(
    non_camel_case_types,
    missing_docs,
    clippy::too_many_lines,
    reason = "generated versioned C schema objects and tagged alternatives"
)]
use super::views::{Node, Storage, read_boolean, read_json, read_number, read_presence, read_text};
use super::views::{
    thinkthen_complete_extensions, thinkthen_complete_json, thinkthen_complete_utf8,
};
use thinkthen::ErrorKind;
pub const THINKTHEN_COMPLETE_PRESENCE_MISSING: u32 = 0;
pub const THINKTHEN_COMPLETE_PRESENCE_NULL: u32 = 1;
pub const THINKTHEN_COMPLETE_PRESENCE_VALUE: u32 = 2;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_annotation_member_map_entry {
    pub name: thinkthen_complete_utf8,
    pub value: *const thinkthen_complete_annotation_member,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_annotation_member_map {
    pub data: *const thinkthen_complete_annotation_member_map_entry,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_annotation_member_map(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_annotation_member_map, ErrorKind> {
    let mut values = Vec::new();
    for (key, item) in node.object()? {
        values.push(thinkthen_complete_annotation_member_map_entry {
            name: store.text(key),
            value: read_thinkthen_complete_annotation_member(item, store)
                .map(|value| store.hold(value))?,
        });
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_annotation_member_map { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_utf8_presence {
    pub presence: u32,
    pub value: thinkthen_complete_utf8,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_u64_presence {
    pub presence: u32,
    pub value: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_json_presence {
    pub presence: u32,
    pub value: *const thinkthen_complete_json,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_position_presence {
    pub presence: u32,
    pub value: *const thinkthen_complete_position,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_physical_source_presence {
    pub presence: u32,
    pub value: *const thinkthen_complete_physical_source,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_annotation {
    pub answer_id: *const thinkthen_complete_answer_id,
    pub answers: thinkthen_complete_annotation_member_map,
    pub file: thinkthen_complete_utf8_presence,
    pub first_line: thinkthen_complete_u64_presence,
    pub index: thinkthen_complete_u64_presence,
    pub input: thinkthen_complete_json_presence,
    pub last_line: thinkthen_complete_u64_presence,
    pub meta: *const thinkthen_complete_meta,
    pub position: thinkthen_complete_position_presence,
    pub schema: *const thinkthen_complete_result_tag,
    pub source: thinkthen_complete_physical_source_presence,
    pub value: *const thinkthen_complete_annotated_row,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_annotation(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_annotation, ErrorKind> {
    Ok(thinkthen_complete_annotation {
        answer_id: {
            let value = node.required("answer_id")?;
            read_thinkthen_complete_answer_id(value, store).map(|value| store.hold(value))?
        },
        answers: {
            let value = node.required("answers")?;
            read_thinkthen_complete_annotation_member_map(value, store)?
        },
        file: {
            let (presence, value) = read_presence(
                node.member("file"),
                false,
                thinkthen_complete_utf8 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence { presence, value }
        },
        first_line: {
            let (presence, value) =
                read_presence(node.member("first_line"), false, 0, read_number, store)?;
            thinkthen_complete_u64_presence { presence, value }
        },
        index: {
            let (presence, value) =
                read_presence(node.member("index"), false, 0, read_number, store)?;
            thinkthen_complete_u64_presence { presence, value }
        },
        input: {
            let (presence, value) = read_presence(
                node.member("input"),
                true,
                std::ptr::null(),
                read_json,
                store,
            )?;
            thinkthen_complete_json_presence { presence, value }
        },
        last_line: {
            let (presence, value) =
                read_presence(node.member("last_line"), false, 0, read_number, store)?;
            thinkthen_complete_u64_presence { presence, value }
        },
        meta: {
            let value = node.required("meta")?;
            read_thinkthen_complete_meta(value, store).map(|value| store.hold(value))?
        },
        position: {
            let (presence, value) = read_presence(
                node.member("position"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_position(value, store).map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_position_presence { presence, value }
        },
        schema: {
            let value = node.required("schema")?;
            read_thinkthen_complete_result_tag(value, store).map(|value| store.hold(value))?
        },
        source: {
            let (presence, value) = read_presence(
                node.member("source"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_physical_source(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_physical_source_presence { presence, value }
        },
        value: {
            let value = node.required("value")?;
            read_thinkthen_complete_annotated_row(value, store).map(|value| store.hold(value))?
        },
        extensions: store.extensions(
            node,
            &[
                "answer_id",
                "answers",
                "file",
                "first_line",
                "index",
                "input",
                "last_line",
                "meta",
                "position",
                "schema",
                "source",
                "value",
            ],
        )?,
    })
}

pub const THINKTHEN_COMPLETE_ANNOTATION_MEMBER_ANSWER_ID: u32 = 1;

pub const THINKTHEN_COMPLETE_ANNOTATION_MEMBER_FAILURE_ID: u32 = 2;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_annotation_member_data {
    pub answer_id: *const thinkthen_complete_annotation_member_answer_id,
    pub failure_id: *const thinkthen_complete_annotation_member_failure_id,
}
impl std::fmt::Debug for thinkthen_complete_annotation_member_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("union payload")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_annotation_member {
    pub kind: u32,
    pub data: thinkthen_complete_annotation_member_data,
}

pub(crate) fn read_thinkthen_complete_annotation_member(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_annotation_member, ErrorKind> {
    if node.member("answer_id").is_some() {
        let value = read_thinkthen_complete_annotation_member_answer_id(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_annotation_member {
            kind: THINKTHEN_COMPLETE_ANNOTATION_MEMBER_ANSWER_ID,
            data: thinkthen_complete_annotation_member_data { answer_id: value },
        });
    }
    if node.member("failure_id").is_some() {
        let value = read_thinkthen_complete_annotation_member_failure_id(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_annotation_member {
            kind: THINKTHEN_COMPLETE_ANNOTATION_MEMBER_FAILURE_ID,
            data: thinkthen_complete_annotation_member_data { failure_id: value },
        });
    }
    Err(ErrorKind::Defect)
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_observation_list {
    pub data: *const *const thinkthen_complete_observation,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_observation_list(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_observation_list, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values
            .push(read_thinkthen_complete_observation(item, store).map(|value| store.hold(value))?);
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_observation_list { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_question_source_list {
    pub data: *const *const thinkthen_complete_question_source,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_question_source_list(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_question_source_list, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_question_source(item, store).map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_question_source_list { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_threshold_presence {
    pub presence: u32,
    pub value: *const thinkthen_complete_threshold,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_usage_presence {
    pub presence: u32,
    pub value: *const thinkthen_complete_usage,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_value_presence {
    pub presence: u32,
    pub value: *const thinkthen_complete_value,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_annotation_member_answer_id {
    pub answer: *const thinkthen_complete_answer,
    pub answer_id: *const thinkthen_complete_answer_id,
    pub observations: thinkthen_complete_observation_list,
    pub question: *const thinkthen_complete_question_document,
    pub question_sources: thinkthen_complete_question_source_list,
    pub request: thinkthen_complete_utf8,
    pub threshold: thinkthen_complete_threshold_presence,
    pub usage: thinkthen_complete_usage_presence,
    pub value: thinkthen_complete_value_presence,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_annotation_member_answer_id(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_annotation_member_answer_id, ErrorKind> {
    Ok(thinkthen_complete_annotation_member_answer_id {
        answer: {
            let value = node.required("answer")?;
            read_thinkthen_complete_answer(value, store).map(|value| store.hold(value))?
        },
        answer_id: {
            let value = node.required("answer_id")?;
            read_thinkthen_complete_answer_id(value, store).map(|value| store.hold(value))?
        },
        observations: {
            let value = node.required("observations")?;
            read_thinkthen_complete_observation_list(value, store)?
        },
        question: {
            let value = node.required("question")?;
            read_thinkthen_complete_question_document(value, store)
                .map(|value| store.hold(value))?
        },
        question_sources: {
            let value = node.required("question_sources")?;
            read_thinkthen_complete_question_source_list(value, store)?
        },
        request: {
            let value = node.required("request")?;
            read_text(value, store)?
        },
        threshold: {
            let (presence, value) = read_presence(
                node.member("threshold"),
                true,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_threshold(value, store).map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_threshold_presence { presence, value }
        },
        usage: {
            let (presence, value) = read_presence(
                node.member("usage"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_usage(value, store).map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_usage_presence { presence, value }
        },
        value: {
            let (presence, value) = read_presence(
                node.member("value"),
                true,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_value(value, store).map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_value_presence { presence, value }
        },
        extensions: store.extensions(
            node,
            &[
                "answer",
                "answer_id",
                "observations",
                "question",
                "question_sources",
                "request",
                "threshold",
                "usage",
                "value",
            ],
        )?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_annotation_member_failure_id {
    pub failure: *const thinkthen_complete_failure,
    pub failure_id: *const thinkthen_complete_failure_id,
    pub observations: thinkthen_complete_observation_list,
    pub question: *const thinkthen_complete_question_document,
    pub question_sources: thinkthen_complete_question_source_list,
    pub request: thinkthen_complete_utf8,
    pub threshold: thinkthen_complete_threshold_presence,
    pub usage: thinkthen_complete_usage_presence,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_annotation_member_failure_id(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_annotation_member_failure_id, ErrorKind> {
    Ok(thinkthen_complete_annotation_member_failure_id {
        failure: {
            let value = node.required("failure")?;
            read_thinkthen_complete_failure(value, store).map(|value| store.hold(value))?
        },
        failure_id: {
            let value = node.required("failure_id")?;
            read_thinkthen_complete_failure_id(value, store).map(|value| store.hold(value))?
        },
        observations: {
            let value = node.required("observations")?;
            read_thinkthen_complete_observation_list(value, store)?
        },
        question: {
            let value = node.required("question")?;
            read_thinkthen_complete_question_document(value, store)
                .map(|value| store.hold(value))?
        },
        question_sources: {
            let value = node.required("question_sources")?;
            read_thinkthen_complete_question_source_list(value, store)?
        },
        request: {
            let value = node.required("request")?;
            read_text(value, store)?
        },
        threshold: {
            let (presence, value) = read_presence(
                node.member("threshold"),
                true,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_threshold(value, store).map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_threshold_presence { presence, value }
        },
        usage: {
            let (presence, value) = read_presence(
                node.member("usage"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_usage(value, store).map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_usage_presence { presence, value }
        },
        extensions: store.extensions(
            node,
            &[
                "failure",
                "failure_id",
                "observations",
                "question",
                "question_sources",
                "request",
                "threshold",
                "usage",
            ],
        )?,
    })
}

pub const THINKTHEN_COMPLETE_ANNOTATION_VALUE_DECISION: u32 = 1;

pub const THINKTHEN_COMPLETE_ANNOTATION_VALUE_CHOICE: u32 = 2;

pub const THINKTHEN_COMPLETE_ANNOTATION_VALUE_SCORE: u32 = 3;

pub const THINKTHEN_COMPLETE_ANNOTATION_VALUE_TAGS: u32 = 4;

pub const THINKTHEN_COMPLETE_ANNOTATION_VALUE_FAILED: u32 = 5;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_annotation_value_data {
    pub decision: *const thinkthen_complete_annotation_value_decision,
    pub choice: *const thinkthen_complete_annotation_value_choice,
    pub score: *const thinkthen_complete_annotation_value_score,
    pub tags: *const thinkthen_complete_annotation_value_tags,
    pub failed: *const thinkthen_complete_annotation_value_failed,
}
impl std::fmt::Debug for thinkthen_complete_annotation_value_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("union payload")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_annotation_value {
    pub kind: u32,
    pub data: thinkthen_complete_annotation_value_data,
}

pub(crate) fn read_thinkthen_complete_annotation_value(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_annotation_value, ErrorKind> {
    if node.is_member("kind", "decision") {
        let value = read_thinkthen_complete_annotation_value_decision(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_annotation_value {
            kind: THINKTHEN_COMPLETE_ANNOTATION_VALUE_DECISION,
            data: thinkthen_complete_annotation_value_data { decision: value },
        });
    }
    if node.is_member("kind", "choice") {
        let value = read_thinkthen_complete_annotation_value_choice(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_annotation_value {
            kind: THINKTHEN_COMPLETE_ANNOTATION_VALUE_CHOICE,
            data: thinkthen_complete_annotation_value_data { choice: value },
        });
    }
    if node.is_member("kind", "score") {
        let value = read_thinkthen_complete_annotation_value_score(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_annotation_value {
            kind: THINKTHEN_COMPLETE_ANNOTATION_VALUE_SCORE,
            data: thinkthen_complete_annotation_value_data { score: value },
        });
    }
    if node.is_member("kind", "tags") {
        let value = read_thinkthen_complete_annotation_value_tags(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_annotation_value {
            kind: THINKTHEN_COMPLETE_ANNOTATION_VALUE_TAGS,
            data: thinkthen_complete_annotation_value_data { tags: value },
        });
    }
    if node.is_member("kind", "failed") {
        let value = read_thinkthen_complete_annotation_value_failed(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_annotation_value {
            kind: THINKTHEN_COMPLETE_ANNOTATION_VALUE_FAILED,
            data: thinkthen_complete_annotation_value_data { failed: value },
        });
    }
    Err(ErrorKind::Defect)
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_annotation_value_choice {
    pub kind: thinkthen_complete_utf8,
    pub value: thinkthen_complete_utf8_presence,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_annotation_value_choice(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_annotation_value_choice, ErrorKind> {
    Ok(thinkthen_complete_annotation_value_choice {
        kind: {
            let value = node.required("kind")?;
            read_text(value, store)?
        },
        value: {
            let (presence, value) = read_presence(
                node.member("value"),
                true,
                thinkthen_complete_utf8 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence { presence, value }
        },
        extensions: store.extensions(node, &["kind", "value"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_u32_presence {
    pub presence: u32,
    pub value: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_annotation_value_decision {
    pub kind: thinkthen_complete_utf8,
    pub value: thinkthen_complete_u32_presence,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_annotation_value_decision(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_annotation_value_decision, ErrorKind> {
    Ok(thinkthen_complete_annotation_value_decision {
        kind: {
            let value = node.required("kind")?;
            read_text(value, store)?
        },
        value: {
            let (presence, value) =
                read_presence(node.member("value"), true, 0, read_boolean, store)?;
            thinkthen_complete_u32_presence { presence, value }
        },
        extensions: store.extensions(node, &["kind", "value"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_annotation_value_failed {
    pub kind: thinkthen_complete_utf8,
    pub value: *const thinkthen_complete_failure,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_annotation_value_failed(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_annotation_value_failed, ErrorKind> {
    Ok(thinkthen_complete_annotation_value_failed {
        kind: {
            let value = node.required("kind")?;
            read_text(value, store)?
        },
        value: {
            let value = node.required("value")?;
            read_thinkthen_complete_failure(value, store).map(|value| store.hold(value))?
        },
        extensions: store.extensions(node, &["kind", "value"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_annotation_value_score {
    pub kind: thinkthen_complete_utf8,
    pub value: f64,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_annotation_value_score(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_annotation_value_score, ErrorKind> {
    Ok(thinkthen_complete_annotation_value_score {
        kind: {
            let value = node.required("kind")?;
            read_text(value, store)?
        },
        value: {
            let value = node.required("value")?;
            read_number(value, store)?
        },
        extensions: store.extensions(node, &["kind", "value"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_string_list {
    pub data: *const thinkthen_complete_utf8,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_string_list(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_string_list, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(read_text(item, store)?);
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_string_list { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_annotation_value_tags {
    pub kind: thinkthen_complete_utf8,
    pub value: thinkthen_complete_string_list,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_annotation_value_tags(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_annotation_value_tags, ErrorKind> {
    Ok(thinkthen_complete_annotation_value_tags {
        kind: {
            let value = node.required("kind")?;
            read_text(value, store)?
        },
        value: {
            let value = node.required("value")?;
            read_thinkthen_complete_string_list(value, store)?
        },
        extensions: store.extensions(node, &["kind", "value"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answer_id {
    pub value: thinkthen_complete_utf8,
}

pub(crate) fn read_thinkthen_complete_answer_id(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_answer_id, ErrorKind> {
    let value = read_text(node, store)?;
    Ok(thinkthen_complete_answer_id { value })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answer_resolution {
    pub kind: u32,
}

pub const THINKTHEN_COMPLETE_ANSWER_RESOLUTION_RESOLVED: u32 = 1;

pub const THINKTHEN_COMPLETE_ANSWER_RESOLUTION_UNRESOLVED: u32 = 2;

pub const THINKTHEN_COMPLETE_ANSWER_RESOLUTION_FAILED: u32 = 3;

pub(crate) fn read_thinkthen_complete_answer_resolution(
    node: &Node,
    _store: &mut Storage,
) -> Result<thinkthen_complete_answer_resolution, ErrorKind> {
    let kind = match node.text()? {
        "resolved" => THINKTHEN_COMPLETE_ANSWER_RESOLUTION_RESOLVED,
        "unresolved" => THINKTHEN_COMPLETE_ANSWER_RESOLUTION_UNRESOLVED,
        "failed" => THINKTHEN_COMPLETE_ANSWER_RESOLUTION_FAILED,
        _ => return Err(ErrorKind::Defect),
    };
    Ok(thinkthen_complete_answer_resolution { kind })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answer_resolution_map_entry {
    pub name: thinkthen_complete_utf8,
    pub value: *const thinkthen_complete_answer_resolution,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answer_resolution_map {
    pub data: *const thinkthen_complete_answer_resolution_map_entry,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_answer_resolution_map(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_answer_resolution_map, ErrorKind> {
    let mut values = Vec::new();
    for (key, item) in node.object()? {
        values.push(thinkthen_complete_answer_resolution_map_entry {
            name: store.text(key),
            value: read_thinkthen_complete_answer_resolution(item, store)
                .map(|value| store.hold(value))?,
        });
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_answer_resolution_map { data, len })
}

pub const THINKTHEN_COMPLETE_ANSWER_RESOLUTIONS_STRING: u32 = 1;

pub const THINKTHEN_COMPLETE_ANSWER_RESOLUTIONS_OBJECT: u32 = 2;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_answer_resolutions_data {
    pub string: *const thinkthen_complete_answer_resolution,
    pub object: thinkthen_complete_answer_resolution_map,
}
impl std::fmt::Debug for thinkthen_complete_answer_resolutions_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("union payload")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answer_resolutions {
    pub kind: u32,
    pub data: thinkthen_complete_answer_resolutions_data,
}

pub(crate) fn read_thinkthen_complete_answer_resolutions(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_answer_resolutions, ErrorKind> {
    if node.kind() == "string" {
        let value = read_thinkthen_complete_answer_resolution(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_answer_resolutions {
            kind: THINKTHEN_COMPLETE_ANSWER_RESOLUTIONS_STRING,
            data: thinkthen_complete_answer_resolutions_data { string: value },
        });
    }
    if node.kind() == "object" {
        let value = read_thinkthen_complete_answer_resolution_map(node, store)?;
        return Ok(thinkthen_complete_answer_resolutions {
            kind: THINKTHEN_COMPLETE_ANSWER_RESOLUTIONS_OBJECT,
            data: thinkthen_complete_answer_resolutions_data { object: value },
        });
    }
    Err(ErrorKind::Defect)
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answer_shape {
    pub value: *const thinkthen_complete_json,
}

pub(crate) fn read_thinkthen_complete_answer_shape(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_answer_shape, ErrorKind> {
    let value = read_json(node, store)?;
    Ok(thinkthen_complete_answer_shape { value })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answer_shape_8_any_of_1_items_list {
    pub data: *const *const thinkthen_complete_answer_shape_8_any_of_1_items,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_answer_shape_8_any_of_1_items_list(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_answer_shape_8_any_of_1_items_list, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_answer_shape_8_any_of_1_items(item, store)
                .map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_answer_shape_8_any_of_1_items_list { data, len })
}

fn matches_union_item_0(node: &Node) -> bool {
    node.kind() == "number" || node.kind() == "null"
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answer_shape_10_any_of_1_items_list {
    pub data: *const *const thinkthen_complete_answer_shape_10_any_of_1_items,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_answer_shape_10_any_of_1_items_list(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_answer_shape_10_any_of_1_items_list, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_answer_shape_10_any_of_1_items(item, store)
                .map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_answer_shape_10_any_of_1_items_list { data, len })
}

fn matches_union_item_1(node: &Node) -> bool {
    node.array()
        .is_ok_and(|items| items.iter().all(matches_union_item_0))
        || node.kind() == "null"
}

pub const THINKTHEN_COMPLETE_ANSWER_SHAPE_10_ARRAY: u32 = 1;

pub const THINKTHEN_COMPLETE_ANSWER_SHAPE_10_NULL: u32 = 2;

pub const THINKTHEN_COMPLETE_ANSWER_SHAPE_10_ARRAY2: u32 = 3;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_answer_shape_10_data {
    pub array: thinkthen_complete_answer_shape_8_any_of_1_items_list,
    pub null: u32,
    pub array2: thinkthen_complete_answer_shape_10_any_of_1_items_list,
}
impl std::fmt::Debug for thinkthen_complete_answer_shape_10_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("union payload")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answer_shape_10 {
    pub kind: u32,
    pub data: thinkthen_complete_answer_shape_10_data,
}

pub(crate) fn read_thinkthen_complete_answer_shape_10(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_answer_shape_10, ErrorKind> {
    if node
        .array()
        .is_ok_and(|items| items.iter().all(matches_union_item_0))
    {
        let value = read_thinkthen_complete_answer_shape_8_any_of_1_items_list(node, store)?;
        return Ok(thinkthen_complete_answer_shape_10 {
            kind: THINKTHEN_COMPLETE_ANSWER_SHAPE_10_ARRAY,
            data: thinkthen_complete_answer_shape_10_data { array: value },
        });
    }
    if node.kind() == "null" {
        let value = 0;
        return Ok(thinkthen_complete_answer_shape_10 {
            kind: THINKTHEN_COMPLETE_ANSWER_SHAPE_10_NULL,
            data: thinkthen_complete_answer_shape_10_data { null: value },
        });
    }
    if node
        .array()
        .is_ok_and(|items| items.iter().all(matches_union_item_1))
    {
        let value = read_thinkthen_complete_answer_shape_10_any_of_1_items_list(node, store)?;
        return Ok(thinkthen_complete_answer_shape_10 {
            kind: THINKTHEN_COMPLETE_ANSWER_SHAPE_10_ARRAY2,
            data: thinkthen_complete_answer_shape_10_data { array2: value },
        });
    }
    Err(ErrorKind::Defect)
}

pub const THINKTHEN_COMPLETE_ANSWER_SHAPE_10_ANY_OF_1_ITEMS_ARRAY: u32 = 1;

pub const THINKTHEN_COMPLETE_ANSWER_SHAPE_10_ANY_OF_1_ITEMS_NULL: u32 = 2;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_answer_shape_10_any_of_1_items_data {
    pub array: thinkthen_complete_answer_shape_8_any_of_1_items_list,
    pub null: u32,
}
impl std::fmt::Debug for thinkthen_complete_answer_shape_10_any_of_1_items_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("union payload")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answer_shape_10_any_of_1_items {
    pub kind: u32,
    pub data: thinkthen_complete_answer_shape_10_any_of_1_items_data,
}

pub(crate) fn read_thinkthen_complete_answer_shape_10_any_of_1_items(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_answer_shape_10_any_of_1_items, ErrorKind> {
    if node
        .array()
        .is_ok_and(|items| items.iter().all(matches_union_item_0))
    {
        let value = read_thinkthen_complete_answer_shape_8_any_of_1_items_list(node, store)?;
        return Ok(thinkthen_complete_answer_shape_10_any_of_1_items {
            kind: THINKTHEN_COMPLETE_ANSWER_SHAPE_10_ANY_OF_1_ITEMS_ARRAY,
            data: thinkthen_complete_answer_shape_10_any_of_1_items_data { array: value },
        });
    }
    if node.kind() == "null" {
        let value = 0;
        return Ok(thinkthen_complete_answer_shape_10_any_of_1_items {
            kind: THINKTHEN_COMPLETE_ANSWER_SHAPE_10_ANY_OF_1_ITEMS_NULL,
            data: thinkthen_complete_answer_shape_10_any_of_1_items_data { null: value },
        });
    }
    Err(ErrorKind::Defect)
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_integer_list {
    pub data: *const u64,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_integer_list(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_integer_list, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(read_number(item, store)?);
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_integer_list { data, len })
}

fn matches_union_item_2(node: &Node) -> bool {
    node.kind() == "number"
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answer_shape_11_any_of_1_items_list {
    pub data: *const *const thinkthen_complete_answer_shape_11_any_of_1_items,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_answer_shape_11_any_of_1_items_list(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_answer_shape_11_any_of_1_items_list, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_answer_shape_11_any_of_1_items(item, store)
                .map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_answer_shape_11_any_of_1_items_list { data, len })
}

fn matches_union_item_3(node: &Node) -> bool {
    node.array()
        .is_ok_and(|items| items.iter().all(matches_union_item_2))
        || node.kind() == "null"
}

pub const THINKTHEN_COMPLETE_ANSWER_SHAPE_11_ARRAY: u32 = 1;

pub const THINKTHEN_COMPLETE_ANSWER_SHAPE_11_NULL: u32 = 2;

pub const THINKTHEN_COMPLETE_ANSWER_SHAPE_11_ARRAY2: u32 = 3;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_answer_shape_11_data {
    pub array: thinkthen_complete_integer_list,
    pub null: u32,
    pub array2: thinkthen_complete_answer_shape_11_any_of_1_items_list,
}
impl std::fmt::Debug for thinkthen_complete_answer_shape_11_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("union payload")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answer_shape_11 {
    pub kind: u32,
    pub data: thinkthen_complete_answer_shape_11_data,
}

pub(crate) fn read_thinkthen_complete_answer_shape_11(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_answer_shape_11, ErrorKind> {
    if node
        .array()
        .is_ok_and(|items| items.iter().all(matches_union_item_2))
    {
        let value = read_thinkthen_complete_integer_list(node, store)?;
        return Ok(thinkthen_complete_answer_shape_11 {
            kind: THINKTHEN_COMPLETE_ANSWER_SHAPE_11_ARRAY,
            data: thinkthen_complete_answer_shape_11_data { array: value },
        });
    }
    if node.kind() == "null" {
        let value = 0;
        return Ok(thinkthen_complete_answer_shape_11 {
            kind: THINKTHEN_COMPLETE_ANSWER_SHAPE_11_NULL,
            data: thinkthen_complete_answer_shape_11_data { null: value },
        });
    }
    if node
        .array()
        .is_ok_and(|items| items.iter().all(matches_union_item_3))
    {
        let value = read_thinkthen_complete_answer_shape_11_any_of_1_items_list(node, store)?;
        return Ok(thinkthen_complete_answer_shape_11 {
            kind: THINKTHEN_COMPLETE_ANSWER_SHAPE_11_ARRAY2,
            data: thinkthen_complete_answer_shape_11_data { array2: value },
        });
    }
    Err(ErrorKind::Defect)
}

pub const THINKTHEN_COMPLETE_ANSWER_SHAPE_11_ANY_OF_1_ITEMS_ARRAY: u32 = 1;

pub const THINKTHEN_COMPLETE_ANSWER_SHAPE_11_ANY_OF_1_ITEMS_NULL: u32 = 2;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_answer_shape_11_any_of_1_items_data {
    pub array: thinkthen_complete_integer_list,
    pub null: u32,
}
impl std::fmt::Debug for thinkthen_complete_answer_shape_11_any_of_1_items_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("union payload")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answer_shape_11_any_of_1_items {
    pub kind: u32,
    pub data: thinkthen_complete_answer_shape_11_any_of_1_items_data,
}

pub(crate) fn read_thinkthen_complete_answer_shape_11_any_of_1_items(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_answer_shape_11_any_of_1_items, ErrorKind> {
    if node
        .array()
        .is_ok_and(|items| items.iter().all(matches_union_item_2))
    {
        let value = read_thinkthen_complete_integer_list(node, store)?;
        return Ok(thinkthen_complete_answer_shape_11_any_of_1_items {
            kind: THINKTHEN_COMPLETE_ANSWER_SHAPE_11_ANY_OF_1_ITEMS_ARRAY,
            data: thinkthen_complete_answer_shape_11_any_of_1_items_data { array: value },
        });
    }
    if node.kind() == "null" {
        let value = 0;
        return Ok(thinkthen_complete_answer_shape_11_any_of_1_items {
            kind: THINKTHEN_COMPLETE_ANSWER_SHAPE_11_ANY_OF_1_ITEMS_NULL,
            data: thinkthen_complete_answer_shape_11_any_of_1_items_data { null: value },
        });
    }
    Err(ErrorKind::Defect)
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answer_shape_12 {
    pub value: *const thinkthen_complete_json,
}

pub(crate) fn read_thinkthen_complete_answer_shape_12(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_answer_shape_12, ErrorKind> {
    let value = read_json(node, store)?;
    Ok(thinkthen_complete_answer_shape_12 { value })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answer_shape_13_any_of_1_items_list {
    pub data: *const *const thinkthen_complete_answer_shape_13_any_of_1_items,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_answer_shape_13_any_of_1_items_list(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_answer_shape_13_any_of_1_items_list, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_answer_shape_13_any_of_1_items(item, store)
                .map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_answer_shape_13_any_of_1_items_list { data, len })
}

fn matches_union_item_4(node: &Node) -> bool {
    node.kind() == "object" || node.kind() == "null"
}

pub const THINKTHEN_COMPLETE_ANSWER_SHAPE_13_OBJECT: u32 = 1;

pub const THINKTHEN_COMPLETE_ANSWER_SHAPE_13_NULL: u32 = 2;

pub const THINKTHEN_COMPLETE_ANSWER_SHAPE_13_ARRAY: u32 = 3;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_answer_shape_13_data {
    pub object: *const thinkthen_complete_settled_recognition,
    pub null: u32,
    pub array: thinkthen_complete_answer_shape_13_any_of_1_items_list,
}
impl std::fmt::Debug for thinkthen_complete_answer_shape_13_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("union payload")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answer_shape_13 {
    pub kind: u32,
    pub data: thinkthen_complete_answer_shape_13_data,
}

pub(crate) fn read_thinkthen_complete_answer_shape_13(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_answer_shape_13, ErrorKind> {
    if node.kind() == "object" {
        let value = read_thinkthen_complete_settled_recognition(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_answer_shape_13 {
            kind: THINKTHEN_COMPLETE_ANSWER_SHAPE_13_OBJECT,
            data: thinkthen_complete_answer_shape_13_data { object: value },
        });
    }
    if node.kind() == "null" {
        let value = 0;
        return Ok(thinkthen_complete_answer_shape_13 {
            kind: THINKTHEN_COMPLETE_ANSWER_SHAPE_13_NULL,
            data: thinkthen_complete_answer_shape_13_data { null: value },
        });
    }
    if node
        .array()
        .is_ok_and(|items| items.iter().all(matches_union_item_4))
    {
        let value = read_thinkthen_complete_answer_shape_13_any_of_1_items_list(node, store)?;
        return Ok(thinkthen_complete_answer_shape_13 {
            kind: THINKTHEN_COMPLETE_ANSWER_SHAPE_13_ARRAY,
            data: thinkthen_complete_answer_shape_13_data { array: value },
        });
    }
    Err(ErrorKind::Defect)
}

pub const THINKTHEN_COMPLETE_ANSWER_SHAPE_13_ANY_OF_1_ITEMS_OBJECT: u32 = 1;

pub const THINKTHEN_COMPLETE_ANSWER_SHAPE_13_ANY_OF_1_ITEMS_NULL: u32 = 2;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_answer_shape_13_any_of_1_items_data {
    pub object: *const thinkthen_complete_settled_recognition,
    pub null: u32,
}
impl std::fmt::Debug for thinkthen_complete_answer_shape_13_any_of_1_items_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("union payload")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answer_shape_13_any_of_1_items {
    pub kind: u32,
    pub data: thinkthen_complete_answer_shape_13_any_of_1_items_data,
}

pub(crate) fn read_thinkthen_complete_answer_shape_13_any_of_1_items(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_answer_shape_13_any_of_1_items, ErrorKind> {
    if node.kind() == "object" {
        let value = read_thinkthen_complete_settled_recognition(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_answer_shape_13_any_of_1_items {
            kind: THINKTHEN_COMPLETE_ANSWER_SHAPE_13_ANY_OF_1_ITEMS_OBJECT,
            data: thinkthen_complete_answer_shape_13_any_of_1_items_data { object: value },
        });
    }
    if node.kind() == "null" {
        let value = 0;
        return Ok(thinkthen_complete_answer_shape_13_any_of_1_items {
            kind: THINKTHEN_COMPLETE_ANSWER_SHAPE_13_ANY_OF_1_ITEMS_NULL,
            data: thinkthen_complete_answer_shape_13_any_of_1_items_data { null: value },
        });
    }
    Err(ErrorKind::Defect)
}

fn matches_union_item_5(node: &Node) -> bool {
    node.kind() == "object"
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answer_shape_14_any_of_1_items_list {
    pub data: *const *const thinkthen_complete_answer_shape_14_any_of_1_items,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_answer_shape_14_any_of_1_items_list(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_answer_shape_14_any_of_1_items_list, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_answer_shape_14_any_of_1_items(item, store)
                .map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_answer_shape_14_any_of_1_items_list { data, len })
}

fn matches_union_item_6(node: &Node) -> bool {
    node.array()
        .is_ok_and(|items| items.iter().all(matches_union_item_5))
        || node.kind() == "null"
}

pub const THINKTHEN_COMPLETE_ANSWER_SHAPE_14_ARRAY: u32 = 1;

pub const THINKTHEN_COMPLETE_ANSWER_SHAPE_14_NULL: u32 = 2;

pub const THINKTHEN_COMPLETE_ANSWER_SHAPE_14_ARRAY2: u32 = 3;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_answer_shape_14_data {
    pub array: *const thinkthen_complete_settled_relations,
    pub null: u32,
    pub array2: thinkthen_complete_answer_shape_14_any_of_1_items_list,
}
impl std::fmt::Debug for thinkthen_complete_answer_shape_14_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("union payload")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answer_shape_14 {
    pub kind: u32,
    pub data: thinkthen_complete_answer_shape_14_data,
}

pub(crate) fn read_thinkthen_complete_answer_shape_14(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_answer_shape_14, ErrorKind> {
    if node
        .array()
        .is_ok_and(|items| items.iter().all(matches_union_item_5))
    {
        let value = read_thinkthen_complete_settled_relations(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_answer_shape_14 {
            kind: THINKTHEN_COMPLETE_ANSWER_SHAPE_14_ARRAY,
            data: thinkthen_complete_answer_shape_14_data { array: value },
        });
    }
    if node.kind() == "null" {
        let value = 0;
        return Ok(thinkthen_complete_answer_shape_14 {
            kind: THINKTHEN_COMPLETE_ANSWER_SHAPE_14_NULL,
            data: thinkthen_complete_answer_shape_14_data { null: value },
        });
    }
    if node
        .array()
        .is_ok_and(|items| items.iter().all(matches_union_item_6))
    {
        let value = read_thinkthen_complete_answer_shape_14_any_of_1_items_list(node, store)?;
        return Ok(thinkthen_complete_answer_shape_14 {
            kind: THINKTHEN_COMPLETE_ANSWER_SHAPE_14_ARRAY2,
            data: thinkthen_complete_answer_shape_14_data { array2: value },
        });
    }
    Err(ErrorKind::Defect)
}

pub const THINKTHEN_COMPLETE_ANSWER_SHAPE_14_ANY_OF_1_ITEMS_ARRAY: u32 = 1;

pub const THINKTHEN_COMPLETE_ANSWER_SHAPE_14_ANY_OF_1_ITEMS_NULL: u32 = 2;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_answer_shape_14_any_of_1_items_data {
    pub array: *const thinkthen_complete_settled_relations,
    pub null: u32,
}
impl std::fmt::Debug for thinkthen_complete_answer_shape_14_any_of_1_items_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("union payload")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answer_shape_14_any_of_1_items {
    pub kind: u32,
    pub data: thinkthen_complete_answer_shape_14_any_of_1_items_data,
}

pub(crate) fn read_thinkthen_complete_answer_shape_14_any_of_1_items(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_answer_shape_14_any_of_1_items, ErrorKind> {
    if node
        .array()
        .is_ok_and(|items| items.iter().all(matches_union_item_5))
    {
        let value = read_thinkthen_complete_settled_relations(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_answer_shape_14_any_of_1_items {
            kind: THINKTHEN_COMPLETE_ANSWER_SHAPE_14_ANY_OF_1_ITEMS_ARRAY,
            data: thinkthen_complete_answer_shape_14_any_of_1_items_data { array: value },
        });
    }
    if node.kind() == "null" {
        let value = 0;
        return Ok(thinkthen_complete_answer_shape_14_any_of_1_items {
            kind: THINKTHEN_COMPLETE_ANSWER_SHAPE_14_ANY_OF_1_ITEMS_NULL,
            data: thinkthen_complete_answer_shape_14_any_of_1_items_data { null: value },
        });
    }
    Err(ErrorKind::Defect)
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_settled_judgment_map_entry {
    pub name: thinkthen_complete_utf8,
    pub value: *const thinkthen_complete_settled_judgment,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_settled_judgment_map {
    pub data: *const thinkthen_complete_settled_judgment_map_entry,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_settled_judgment_map(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_settled_judgment_map, ErrorKind> {
    let mut values = Vec::new();
    for (key, item) in node.object()? {
        values.push(thinkthen_complete_settled_judgment_map_entry {
            name: store.text(key),
            value: read_thinkthen_complete_settled_judgment(item, store)
                .map(|value| store.hold(value))?,
        });
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_settled_judgment_map { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answer_shape_15_any_of_1_items_list {
    pub data: *const *const thinkthen_complete_answer_shape_15_any_of_1_items,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_answer_shape_15_any_of_1_items_list(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_answer_shape_15_any_of_1_items_list, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_answer_shape_15_any_of_1_items(item, store)
                .map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_answer_shape_15_any_of_1_items_list { data, len })
}

fn matches_union_item_7(node: &Node) -> bool {
    node.kind() == "object" || node.kind() == "null"
}

pub const THINKTHEN_COMPLETE_ANSWER_SHAPE_15_OBJECT: u32 = 1;

pub const THINKTHEN_COMPLETE_ANSWER_SHAPE_15_NULL: u32 = 2;

pub const THINKTHEN_COMPLETE_ANSWER_SHAPE_15_ARRAY: u32 = 3;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_answer_shape_15_data {
    pub object: thinkthen_complete_settled_judgment_map,
    pub null: u32,
    pub array: thinkthen_complete_answer_shape_15_any_of_1_items_list,
}
impl std::fmt::Debug for thinkthen_complete_answer_shape_15_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("union payload")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answer_shape_15 {
    pub kind: u32,
    pub data: thinkthen_complete_answer_shape_15_data,
}

pub(crate) fn read_thinkthen_complete_answer_shape_15(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_answer_shape_15, ErrorKind> {
    if node.kind() == "object" {
        let value = read_thinkthen_complete_settled_judgment_map(node, store)?;
        return Ok(thinkthen_complete_answer_shape_15 {
            kind: THINKTHEN_COMPLETE_ANSWER_SHAPE_15_OBJECT,
            data: thinkthen_complete_answer_shape_15_data { object: value },
        });
    }
    if node.kind() == "null" {
        let value = 0;
        return Ok(thinkthen_complete_answer_shape_15 {
            kind: THINKTHEN_COMPLETE_ANSWER_SHAPE_15_NULL,
            data: thinkthen_complete_answer_shape_15_data { null: value },
        });
    }
    if node
        .array()
        .is_ok_and(|items| items.iter().all(matches_union_item_7))
    {
        let value = read_thinkthen_complete_answer_shape_15_any_of_1_items_list(node, store)?;
        return Ok(thinkthen_complete_answer_shape_15 {
            kind: THINKTHEN_COMPLETE_ANSWER_SHAPE_15_ARRAY,
            data: thinkthen_complete_answer_shape_15_data { array: value },
        });
    }
    Err(ErrorKind::Defect)
}

pub const THINKTHEN_COMPLETE_ANSWER_SHAPE_15_ANY_OF_1_ITEMS_OBJECT: u32 = 1;

pub const THINKTHEN_COMPLETE_ANSWER_SHAPE_15_ANY_OF_1_ITEMS_NULL: u32 = 2;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_answer_shape_15_any_of_1_items_data {
    pub object: thinkthen_complete_settled_judgment_map,
    pub null: u32,
}
impl std::fmt::Debug for thinkthen_complete_answer_shape_15_any_of_1_items_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("union payload")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answer_shape_15_any_of_1_items {
    pub kind: u32,
    pub data: thinkthen_complete_answer_shape_15_any_of_1_items_data,
}

pub(crate) fn read_thinkthen_complete_answer_shape_15_any_of_1_items(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_answer_shape_15_any_of_1_items, ErrorKind> {
    if node.kind() == "object" {
        let value = read_thinkthen_complete_settled_judgment_map(node, store)?;
        return Ok(thinkthen_complete_answer_shape_15_any_of_1_items {
            kind: THINKTHEN_COMPLETE_ANSWER_SHAPE_15_ANY_OF_1_ITEMS_OBJECT,
            data: thinkthen_complete_answer_shape_15_any_of_1_items_data { object: value },
        });
    }
    if node.kind() == "null" {
        let value = 0;
        return Ok(thinkthen_complete_answer_shape_15_any_of_1_items {
            kind: THINKTHEN_COMPLETE_ANSWER_SHAPE_15_ANY_OF_1_ITEMS_NULL,
            data: thinkthen_complete_answer_shape_15_any_of_1_items_data { null: value },
        });
    }
    Err(ErrorKind::Defect)
}

pub const THINKTHEN_COMPLETE_ANSWER_SHAPE_2_NUMBER: u32 = 1;

pub const THINKTHEN_COMPLETE_ANSWER_SHAPE_2_NULL: u32 = 2;

pub const THINKTHEN_COMPLETE_ANSWER_SHAPE_2_ARRAY: u32 = 3;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_answer_shape_2_data {
    pub number: f64,
    pub null: u32,
    pub array: thinkthen_complete_answer_shape_8_any_of_1_items_list,
}
impl std::fmt::Debug for thinkthen_complete_answer_shape_2_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("union payload")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answer_shape_2 {
    pub kind: u32,
    pub data: thinkthen_complete_answer_shape_2_data,
}

pub(crate) fn read_thinkthen_complete_answer_shape_2(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_answer_shape_2, ErrorKind> {
    if node.kind() == "number" {
        let value = read_number(node, store)?;
        return Ok(thinkthen_complete_answer_shape_2 {
            kind: THINKTHEN_COMPLETE_ANSWER_SHAPE_2_NUMBER,
            data: thinkthen_complete_answer_shape_2_data { number: value },
        });
    }
    if node.kind() == "null" {
        let value = 0;
        return Ok(thinkthen_complete_answer_shape_2 {
            kind: THINKTHEN_COMPLETE_ANSWER_SHAPE_2_NULL,
            data: thinkthen_complete_answer_shape_2_data { null: value },
        });
    }
    if node
        .array()
        .is_ok_and(|items| items.iter().all(matches_union_item_0))
    {
        let value = read_thinkthen_complete_answer_shape_8_any_of_1_items_list(node, store)?;
        return Ok(thinkthen_complete_answer_shape_2 {
            kind: THINKTHEN_COMPLETE_ANSWER_SHAPE_2_ARRAY,
            data: thinkthen_complete_answer_shape_2_data { array: value },
        });
    }
    Err(ErrorKind::Defect)
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answer_shape_6_any_of_1_items_list {
    pub data: *const *const thinkthen_complete_answer_shape_6_any_of_1_items,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_answer_shape_6_any_of_1_items_list(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_answer_shape_6_any_of_1_items_list, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_answer_shape_6_any_of_1_items(item, store)
                .map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_answer_shape_6_any_of_1_items_list { data, len })
}

fn matches_union_item_8(node: &Node) -> bool {
    node.kind() == "string" || node.kind() == "null"
}

pub const THINKTHEN_COMPLETE_ANSWER_SHAPE_3_STRING: u32 = 1;

pub const THINKTHEN_COMPLETE_ANSWER_SHAPE_3_NULL: u32 = 2;

pub const THINKTHEN_COMPLETE_ANSWER_SHAPE_3_ARRAY: u32 = 3;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_answer_shape_3_data {
    pub string: thinkthen_complete_utf8,
    pub null: u32,
    pub array: thinkthen_complete_answer_shape_6_any_of_1_items_list,
}
impl std::fmt::Debug for thinkthen_complete_answer_shape_3_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("union payload")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answer_shape_3 {
    pub kind: u32,
    pub data: thinkthen_complete_answer_shape_3_data,
}

pub(crate) fn read_thinkthen_complete_answer_shape_3(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_answer_shape_3, ErrorKind> {
    if node.kind() == "string" {
        let value = read_text(node, store)?;
        return Ok(thinkthen_complete_answer_shape_3 {
            kind: THINKTHEN_COMPLETE_ANSWER_SHAPE_3_STRING,
            data: thinkthen_complete_answer_shape_3_data { string: value },
        });
    }
    if node.kind() == "null" {
        let value = 0;
        return Ok(thinkthen_complete_answer_shape_3 {
            kind: THINKTHEN_COMPLETE_ANSWER_SHAPE_3_NULL,
            data: thinkthen_complete_answer_shape_3_data { null: value },
        });
    }
    if node
        .array()
        .is_ok_and(|items| items.iter().all(matches_union_item_8))
    {
        let value = read_thinkthen_complete_answer_shape_6_any_of_1_items_list(node, store)?;
        return Ok(thinkthen_complete_answer_shape_3 {
            kind: THINKTHEN_COMPLETE_ANSWER_SHAPE_3_ARRAY,
            data: thinkthen_complete_answer_shape_3_data { array: value },
        });
    }
    Err(ErrorKind::Defect)
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answer_shape_4_any_of_1_items_list {
    pub data: *const *const thinkthen_complete_answer_shape_4_any_of_1_items,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_answer_shape_4_any_of_1_items_list(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_answer_shape_4_any_of_1_items_list, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_answer_shape_4_any_of_1_items(item, store)
                .map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_answer_shape_4_any_of_1_items_list { data, len })
}

fn matches_union_item_9(node: &Node) -> bool {
    node.kind() == "number" || node.kind() == "null"
}

pub const THINKTHEN_COMPLETE_ANSWER_SHAPE_4_INTEGER: u32 = 1;

pub const THINKTHEN_COMPLETE_ANSWER_SHAPE_4_NULL: u32 = 2;

pub const THINKTHEN_COMPLETE_ANSWER_SHAPE_4_ARRAY: u32 = 3;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_answer_shape_4_data {
    pub integer: u64,
    pub null: u32,
    pub array: thinkthen_complete_answer_shape_4_any_of_1_items_list,
}
impl std::fmt::Debug for thinkthen_complete_answer_shape_4_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("union payload")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answer_shape_4 {
    pub kind: u32,
    pub data: thinkthen_complete_answer_shape_4_data,
}

pub(crate) fn read_thinkthen_complete_answer_shape_4(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_answer_shape_4, ErrorKind> {
    if node.kind() == "number" {
        let value = read_number(node, store)?;
        return Ok(thinkthen_complete_answer_shape_4 {
            kind: THINKTHEN_COMPLETE_ANSWER_SHAPE_4_INTEGER,
            data: thinkthen_complete_answer_shape_4_data { integer: value },
        });
    }
    if node.kind() == "null" {
        let value = 0;
        return Ok(thinkthen_complete_answer_shape_4 {
            kind: THINKTHEN_COMPLETE_ANSWER_SHAPE_4_NULL,
            data: thinkthen_complete_answer_shape_4_data { null: value },
        });
    }
    if node
        .array()
        .is_ok_and(|items| items.iter().all(matches_union_item_9))
    {
        let value = read_thinkthen_complete_answer_shape_4_any_of_1_items_list(node, store)?;
        return Ok(thinkthen_complete_answer_shape_4 {
            kind: THINKTHEN_COMPLETE_ANSWER_SHAPE_4_ARRAY,
            data: thinkthen_complete_answer_shape_4_data { array: value },
        });
    }
    Err(ErrorKind::Defect)
}

pub const THINKTHEN_COMPLETE_ANSWER_SHAPE_4_ANY_OF_1_ITEMS_INTEGER: u32 = 1;

pub const THINKTHEN_COMPLETE_ANSWER_SHAPE_4_ANY_OF_1_ITEMS_NULL: u32 = 2;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_answer_shape_4_any_of_1_items_data {
    pub integer: u64,
    pub null: u32,
}
impl std::fmt::Debug for thinkthen_complete_answer_shape_4_any_of_1_items_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("union payload")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answer_shape_4_any_of_1_items {
    pub kind: u32,
    pub data: thinkthen_complete_answer_shape_4_any_of_1_items_data,
}

pub(crate) fn read_thinkthen_complete_answer_shape_4_any_of_1_items(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_answer_shape_4_any_of_1_items, ErrorKind> {
    if node.kind() == "number" {
        let value = read_number(node, store)?;
        return Ok(thinkthen_complete_answer_shape_4_any_of_1_items {
            kind: THINKTHEN_COMPLETE_ANSWER_SHAPE_4_ANY_OF_1_ITEMS_INTEGER,
            data: thinkthen_complete_answer_shape_4_any_of_1_items_data { integer: value },
        });
    }
    if node.kind() == "null" {
        let value = 0;
        return Ok(thinkthen_complete_answer_shape_4_any_of_1_items {
            kind: THINKTHEN_COMPLETE_ANSWER_SHAPE_4_ANY_OF_1_ITEMS_NULL,
            data: thinkthen_complete_answer_shape_4_any_of_1_items_data { null: value },
        });
    }
    Err(ErrorKind::Defect)
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answer_resolutions_list {
    pub data: *const *const thinkthen_complete_answer_resolutions,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_answer_resolutions_list(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_answer_resolutions_list, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_answer_resolutions(item, store)
                .map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_answer_resolutions_list { data, len })
}

fn matches_union_item_10(node: &Node) -> bool {
    node.kind() == "string" || node.kind() == "object"
}

pub const THINKTHEN_COMPLETE_ANSWER_SHAPE_5_STRING: u32 = 1;

pub const THINKTHEN_COMPLETE_ANSWER_SHAPE_5_OBJECT: u32 = 2;

pub const THINKTHEN_COMPLETE_ANSWER_SHAPE_5_ARRAY: u32 = 3;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_answer_shape_5_data {
    pub string: *const thinkthen_complete_answer_resolution,
    pub object: thinkthen_complete_answer_resolution_map,
    pub array: thinkthen_complete_answer_resolutions_list,
}
impl std::fmt::Debug for thinkthen_complete_answer_shape_5_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("union payload")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answer_shape_5 {
    pub kind: u32,
    pub data: thinkthen_complete_answer_shape_5_data,
}

pub(crate) fn read_thinkthen_complete_answer_shape_5(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_answer_shape_5, ErrorKind> {
    if node.kind() == "string" {
        let value = read_thinkthen_complete_answer_resolution(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_answer_shape_5 {
            kind: THINKTHEN_COMPLETE_ANSWER_SHAPE_5_STRING,
            data: thinkthen_complete_answer_shape_5_data { string: value },
        });
    }
    if node.kind() == "object" {
        let value = read_thinkthen_complete_answer_resolution_map(node, store)?;
        return Ok(thinkthen_complete_answer_shape_5 {
            kind: THINKTHEN_COMPLETE_ANSWER_SHAPE_5_OBJECT,
            data: thinkthen_complete_answer_shape_5_data { object: value },
        });
    }
    if node
        .array()
        .is_ok_and(|items| items.iter().all(matches_union_item_10))
    {
        let value = read_thinkthen_complete_answer_resolutions_list(node, store)?;
        return Ok(thinkthen_complete_answer_shape_5 {
            kind: THINKTHEN_COMPLETE_ANSWER_SHAPE_5_ARRAY,
            data: thinkthen_complete_answer_shape_5_data { array: value },
        });
    }
    Err(ErrorKind::Defect)
}

pub const THINKTHEN_COMPLETE_ANSWER_SHAPE_6_STRING: u32 = 1;

pub const THINKTHEN_COMPLETE_ANSWER_SHAPE_6_NULL: u32 = 2;

pub const THINKTHEN_COMPLETE_ANSWER_SHAPE_6_ARRAY: u32 = 3;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_answer_shape_6_data {
    pub string: thinkthen_complete_utf8,
    pub null: u32,
    pub array: thinkthen_complete_answer_shape_6_any_of_1_items_list,
}
impl std::fmt::Debug for thinkthen_complete_answer_shape_6_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("union payload")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answer_shape_6 {
    pub kind: u32,
    pub data: thinkthen_complete_answer_shape_6_data,
}

pub(crate) fn read_thinkthen_complete_answer_shape_6(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_answer_shape_6, ErrorKind> {
    if node.kind() == "string" {
        let value = read_text(node, store)?;
        return Ok(thinkthen_complete_answer_shape_6 {
            kind: THINKTHEN_COMPLETE_ANSWER_SHAPE_6_STRING,
            data: thinkthen_complete_answer_shape_6_data { string: value },
        });
    }
    if node.kind() == "null" {
        let value = 0;
        return Ok(thinkthen_complete_answer_shape_6 {
            kind: THINKTHEN_COMPLETE_ANSWER_SHAPE_6_NULL,
            data: thinkthen_complete_answer_shape_6_data { null: value },
        });
    }
    if node
        .array()
        .is_ok_and(|items| items.iter().all(matches_union_item_8))
    {
        let value = read_thinkthen_complete_answer_shape_6_any_of_1_items_list(node, store)?;
        return Ok(thinkthen_complete_answer_shape_6 {
            kind: THINKTHEN_COMPLETE_ANSWER_SHAPE_6_ARRAY,
            data: thinkthen_complete_answer_shape_6_data { array: value },
        });
    }
    Err(ErrorKind::Defect)
}

pub const THINKTHEN_COMPLETE_ANSWER_SHAPE_6_ANY_OF_1_ITEMS_STRING: u32 = 1;

pub const THINKTHEN_COMPLETE_ANSWER_SHAPE_6_ANY_OF_1_ITEMS_NULL: u32 = 2;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_answer_shape_6_any_of_1_items_data {
    pub string: thinkthen_complete_utf8,
    pub null: u32,
}
impl std::fmt::Debug for thinkthen_complete_answer_shape_6_any_of_1_items_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("union payload")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answer_shape_6_any_of_1_items {
    pub kind: u32,
    pub data: thinkthen_complete_answer_shape_6_any_of_1_items_data,
}

pub(crate) fn read_thinkthen_complete_answer_shape_6_any_of_1_items(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_answer_shape_6_any_of_1_items, ErrorKind> {
    if node.kind() == "string" {
        let value = read_text(node, store)?;
        return Ok(thinkthen_complete_answer_shape_6_any_of_1_items {
            kind: THINKTHEN_COMPLETE_ANSWER_SHAPE_6_ANY_OF_1_ITEMS_STRING,
            data: thinkthen_complete_answer_shape_6_any_of_1_items_data { string: value },
        });
    }
    if node.kind() == "null" {
        let value = 0;
        return Ok(thinkthen_complete_answer_shape_6_any_of_1_items {
            kind: THINKTHEN_COMPLETE_ANSWER_SHAPE_6_ANY_OF_1_ITEMS_NULL,
            data: thinkthen_complete_answer_shape_6_any_of_1_items_data { null: value },
        });
    }
    Err(ErrorKind::Defect)
}

fn matches_union_item_11(node: &Node) -> bool {
    node.kind() == "string"
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answer_shape_7_any_of_1_items_list {
    pub data: *const *const thinkthen_complete_answer_shape_7_any_of_1_items,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_answer_shape_7_any_of_1_items_list(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_answer_shape_7_any_of_1_items_list, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_answer_shape_7_any_of_1_items(item, store)
                .map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_answer_shape_7_any_of_1_items_list { data, len })
}

fn matches_union_item_12(node: &Node) -> bool {
    node.array()
        .is_ok_and(|items| items.iter().all(matches_union_item_11))
        || node.kind() == "null"
}

pub const THINKTHEN_COMPLETE_ANSWER_SHAPE_7_ARRAY: u32 = 1;

pub const THINKTHEN_COMPLETE_ANSWER_SHAPE_7_NULL: u32 = 2;

pub const THINKTHEN_COMPLETE_ANSWER_SHAPE_7_ARRAY2: u32 = 3;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_answer_shape_7_data {
    pub array: thinkthen_complete_string_list,
    pub null: u32,
    pub array2: thinkthen_complete_answer_shape_7_any_of_1_items_list,
}
impl std::fmt::Debug for thinkthen_complete_answer_shape_7_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("union payload")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answer_shape_7 {
    pub kind: u32,
    pub data: thinkthen_complete_answer_shape_7_data,
}

pub(crate) fn read_thinkthen_complete_answer_shape_7(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_answer_shape_7, ErrorKind> {
    if node
        .array()
        .is_ok_and(|items| items.iter().all(matches_union_item_11))
    {
        let value = read_thinkthen_complete_string_list(node, store)?;
        return Ok(thinkthen_complete_answer_shape_7 {
            kind: THINKTHEN_COMPLETE_ANSWER_SHAPE_7_ARRAY,
            data: thinkthen_complete_answer_shape_7_data { array: value },
        });
    }
    if node.kind() == "null" {
        let value = 0;
        return Ok(thinkthen_complete_answer_shape_7 {
            kind: THINKTHEN_COMPLETE_ANSWER_SHAPE_7_NULL,
            data: thinkthen_complete_answer_shape_7_data { null: value },
        });
    }
    if node
        .array()
        .is_ok_and(|items| items.iter().all(matches_union_item_12))
    {
        let value = read_thinkthen_complete_answer_shape_7_any_of_1_items_list(node, store)?;
        return Ok(thinkthen_complete_answer_shape_7 {
            kind: THINKTHEN_COMPLETE_ANSWER_SHAPE_7_ARRAY2,
            data: thinkthen_complete_answer_shape_7_data { array2: value },
        });
    }
    Err(ErrorKind::Defect)
}

pub const THINKTHEN_COMPLETE_ANSWER_SHAPE_7_ANY_OF_1_ITEMS_ARRAY: u32 = 1;

pub const THINKTHEN_COMPLETE_ANSWER_SHAPE_7_ANY_OF_1_ITEMS_NULL: u32 = 2;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_answer_shape_7_any_of_1_items_data {
    pub array: thinkthen_complete_string_list,
    pub null: u32,
}
impl std::fmt::Debug for thinkthen_complete_answer_shape_7_any_of_1_items_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("union payload")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answer_shape_7_any_of_1_items {
    pub kind: u32,
    pub data: thinkthen_complete_answer_shape_7_any_of_1_items_data,
}

pub(crate) fn read_thinkthen_complete_answer_shape_7_any_of_1_items(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_answer_shape_7_any_of_1_items, ErrorKind> {
    if node
        .array()
        .is_ok_and(|items| items.iter().all(matches_union_item_11))
    {
        let value = read_thinkthen_complete_string_list(node, store)?;
        return Ok(thinkthen_complete_answer_shape_7_any_of_1_items {
            kind: THINKTHEN_COMPLETE_ANSWER_SHAPE_7_ANY_OF_1_ITEMS_ARRAY,
            data: thinkthen_complete_answer_shape_7_any_of_1_items_data { array: value },
        });
    }
    if node.kind() == "null" {
        let value = 0;
        return Ok(thinkthen_complete_answer_shape_7_any_of_1_items {
            kind: THINKTHEN_COMPLETE_ANSWER_SHAPE_7_ANY_OF_1_ITEMS_NULL,
            data: thinkthen_complete_answer_shape_7_any_of_1_items_data { null: value },
        });
    }
    Err(ErrorKind::Defect)
}

pub const THINKTHEN_COMPLETE_ANSWER_SHAPE_8_NUMBER: u32 = 1;

pub const THINKTHEN_COMPLETE_ANSWER_SHAPE_8_NULL: u32 = 2;

pub const THINKTHEN_COMPLETE_ANSWER_SHAPE_8_ARRAY: u32 = 3;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_answer_shape_8_data {
    pub number: f64,
    pub null: u32,
    pub array: thinkthen_complete_answer_shape_8_any_of_1_items_list,
}
impl std::fmt::Debug for thinkthen_complete_answer_shape_8_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("union payload")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answer_shape_8 {
    pub kind: u32,
    pub data: thinkthen_complete_answer_shape_8_data,
}

pub(crate) fn read_thinkthen_complete_answer_shape_8(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_answer_shape_8, ErrorKind> {
    if node.kind() == "number" {
        let value = read_number(node, store)?;
        return Ok(thinkthen_complete_answer_shape_8 {
            kind: THINKTHEN_COMPLETE_ANSWER_SHAPE_8_NUMBER,
            data: thinkthen_complete_answer_shape_8_data { number: value },
        });
    }
    if node.kind() == "null" {
        let value = 0;
        return Ok(thinkthen_complete_answer_shape_8 {
            kind: THINKTHEN_COMPLETE_ANSWER_SHAPE_8_NULL,
            data: thinkthen_complete_answer_shape_8_data { null: value },
        });
    }
    if node
        .array()
        .is_ok_and(|items| items.iter().all(matches_union_item_0))
    {
        let value = read_thinkthen_complete_answer_shape_8_any_of_1_items_list(node, store)?;
        return Ok(thinkthen_complete_answer_shape_8 {
            kind: THINKTHEN_COMPLETE_ANSWER_SHAPE_8_ARRAY,
            data: thinkthen_complete_answer_shape_8_data { array: value },
        });
    }
    Err(ErrorKind::Defect)
}

pub const THINKTHEN_COMPLETE_ANSWER_SHAPE_8_ANY_OF_1_ITEMS_NUMBER: u32 = 1;

pub const THINKTHEN_COMPLETE_ANSWER_SHAPE_8_ANY_OF_1_ITEMS_NULL: u32 = 2;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_answer_shape_8_any_of_1_items_data {
    pub number: f64,
    pub null: u32,
}
impl std::fmt::Debug for thinkthen_complete_answer_shape_8_any_of_1_items_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("union payload")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answer_shape_8_any_of_1_items {
    pub kind: u32,
    pub data: thinkthen_complete_answer_shape_8_any_of_1_items_data,
}

pub(crate) fn read_thinkthen_complete_answer_shape_8_any_of_1_items(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_answer_shape_8_any_of_1_items, ErrorKind> {
    if node.kind() == "number" {
        let value = read_number(node, store)?;
        return Ok(thinkthen_complete_answer_shape_8_any_of_1_items {
            kind: THINKTHEN_COMPLETE_ANSWER_SHAPE_8_ANY_OF_1_ITEMS_NUMBER,
            data: thinkthen_complete_answer_shape_8_any_of_1_items_data { number: value },
        });
    }
    if node.kind() == "null" {
        let value = 0;
        return Ok(thinkthen_complete_answer_shape_8_any_of_1_items {
            kind: THINKTHEN_COMPLETE_ANSWER_SHAPE_8_ANY_OF_1_ITEMS_NULL,
            data: thinkthen_complete_answer_shape_8_any_of_1_items_data { null: value },
        });
    }
    Err(ErrorKind::Defect)
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_settled_original_list {
    pub data: *const *const thinkthen_complete_settled_original,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_settled_original_list(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_settled_original_list, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_settled_original(item, store).map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_settled_original_list { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answer_shape_9_any_of_1_items_list {
    pub data: *const *const thinkthen_complete_answer_shape_9_any_of_1_items,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_answer_shape_9_any_of_1_items_list(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_answer_shape_9_any_of_1_items_list, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_answer_shape_9_any_of_1_items(item, store)
                .map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_answer_shape_9_any_of_1_items_list { data, len })
}

fn matches_union_item_13(node: &Node) -> bool {
    node.kind() == "array" || node.kind() == "null"
}

pub const THINKTHEN_COMPLETE_ANSWER_SHAPE_9_ARRAY: u32 = 1;

pub const THINKTHEN_COMPLETE_ANSWER_SHAPE_9_NULL: u32 = 2;

pub const THINKTHEN_COMPLETE_ANSWER_SHAPE_9_ARRAY2: u32 = 3;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_answer_shape_9_data {
    pub array: thinkthen_complete_settled_original_list,
    pub null: u32,
    pub array2: thinkthen_complete_answer_shape_9_any_of_1_items_list,
}
impl std::fmt::Debug for thinkthen_complete_answer_shape_9_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("union payload")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answer_shape_9 {
    pub kind: u32,
    pub data: thinkthen_complete_answer_shape_9_data,
}

pub(crate) fn read_thinkthen_complete_answer_shape_9(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_answer_shape_9, ErrorKind> {
    if node.kind() == "array" {
        let value = read_thinkthen_complete_settled_original_list(node, store)?;
        return Ok(thinkthen_complete_answer_shape_9 {
            kind: THINKTHEN_COMPLETE_ANSWER_SHAPE_9_ARRAY,
            data: thinkthen_complete_answer_shape_9_data { array: value },
        });
    }
    if node.kind() == "null" {
        let value = 0;
        return Ok(thinkthen_complete_answer_shape_9 {
            kind: THINKTHEN_COMPLETE_ANSWER_SHAPE_9_NULL,
            data: thinkthen_complete_answer_shape_9_data { null: value },
        });
    }
    if node
        .array()
        .is_ok_and(|items| items.iter().all(matches_union_item_13))
    {
        let value = read_thinkthen_complete_answer_shape_9_any_of_1_items_list(node, store)?;
        return Ok(thinkthen_complete_answer_shape_9 {
            kind: THINKTHEN_COMPLETE_ANSWER_SHAPE_9_ARRAY2,
            data: thinkthen_complete_answer_shape_9_data { array2: value },
        });
    }
    Err(ErrorKind::Defect)
}

pub const THINKTHEN_COMPLETE_ANSWER_SHAPE_9_ANY_OF_1_ITEMS_ARRAY: u32 = 1;

pub const THINKTHEN_COMPLETE_ANSWER_SHAPE_9_ANY_OF_1_ITEMS_NULL: u32 = 2;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_answer_shape_9_any_of_1_items_data {
    pub array: thinkthen_complete_settled_original_list,
    pub null: u32,
}
impl std::fmt::Debug for thinkthen_complete_answer_shape_9_any_of_1_items_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("union payload")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answer_shape_9_any_of_1_items {
    pub kind: u32,
    pub data: thinkthen_complete_answer_shape_9_any_of_1_items_data,
}

pub(crate) fn read_thinkthen_complete_answer_shape_9_any_of_1_items(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_answer_shape_9_any_of_1_items, ErrorKind> {
    if node.kind() == "array" {
        let value = read_thinkthen_complete_settled_original_list(node, store)?;
        return Ok(thinkthen_complete_answer_shape_9_any_of_1_items {
            kind: THINKTHEN_COMPLETE_ANSWER_SHAPE_9_ANY_OF_1_ITEMS_ARRAY,
            data: thinkthen_complete_answer_shape_9_any_of_1_items_data { array: value },
        });
    }
    if node.kind() == "null" {
        let value = 0;
        return Ok(thinkthen_complete_answer_shape_9_any_of_1_items {
            kind: THINKTHEN_COMPLETE_ANSWER_SHAPE_9_ANY_OF_1_ITEMS_NULL,
            data: thinkthen_complete_answer_shape_9_any_of_1_items_data { null: value },
        });
    }
    Err(ErrorKind::Defect)
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_relation_member_list {
    pub data: *const *const thinkthen_complete_relation_member,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_relation_member_list(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_relation_member_list, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_relation_member(item, store).map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_relation_member_list { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answers {
    pub questions: thinkthen_complete_relation_member_list,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_answers(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_answers, ErrorKind> {
    Ok(thinkthen_complete_answers {
        questions: {
            let value = node.required("questions")?;
            read_thinkthen_complete_relation_member_list(value, store)?
        },
        extensions: store.extensions(node, &["questions"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_image_list {
    pub data: *const *const thinkthen_complete_image,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_image_list(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_image_list, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(read_thinkthen_complete_image(item, store).map(|value| store.hold(value))?);
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_image_list { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_image_list_presence {
    pub presence: u32,
    pub value: thinkthen_complete_image_list,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_rank_member_list {
    pub data: *const *const thinkthen_complete_rank_member,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_rank_member_list(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_rank_member_list, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values
            .push(read_thinkthen_complete_rank_member(item, store).map(|value| store.hold(value))?);
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_rank_member_list { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_rank_member_list_presence {
    pub presence: u32,
    pub value: thinkthen_complete_rank_member_list,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_atomic_array_of_string {
    pub answer: *const thinkthen_complete_answer,
    pub answer_id: *const thinkthen_complete_answer_id,
    pub images: thinkthen_complete_image_list_presence,
    pub index: thinkthen_complete_u64_presence,
    pub input: thinkthen_complete_json_presence,
    pub members: thinkthen_complete_rank_member_list_presence,
    pub meta: *const thinkthen_complete_meta,
    pub question: *const thinkthen_complete_question_document,
    pub question_name: thinkthen_complete_utf8_presence,
    pub schema: *const thinkthen_complete_result_tag,
    pub source: thinkthen_complete_physical_source_presence,
    pub threshold: thinkthen_complete_threshold_presence,
    pub value: thinkthen_complete_string_list,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_atomic_array_of_string(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_atomic_array_of_string, ErrorKind> {
    Ok(thinkthen_complete_atomic_array_of_string {
        answer: {
            let value = node.required("answer")?;
            read_thinkthen_complete_answer(value, store).map(|value| store.hold(value))?
        },
        answer_id: {
            let value = node.required("answer_id")?;
            read_thinkthen_complete_answer_id(value, store).map(|value| store.hold(value))?
        },
        images: {
            let (presence, value) = read_presence(
                node.member("images"),
                false,
                thinkthen_complete_image_list {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_image_list,
                store,
            )?;
            thinkthen_complete_image_list_presence { presence, value }
        },
        index: {
            let (presence, value) =
                read_presence(node.member("index"), false, 0, read_number, store)?;
            thinkthen_complete_u64_presence { presence, value }
        },
        input: {
            let (presence, value) = read_presence(
                node.member("input"),
                false,
                std::ptr::null(),
                read_json,
                store,
            )?;
            thinkthen_complete_json_presence { presence, value }
        },
        members: {
            let (presence, value) = read_presence(
                node.member("members"),
                false,
                thinkthen_complete_rank_member_list {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_rank_member_list,
                store,
            )?;
            thinkthen_complete_rank_member_list_presence { presence, value }
        },
        meta: {
            let value = node.required("meta")?;
            read_thinkthen_complete_meta(value, store).map(|value| store.hold(value))?
        },
        question: {
            let value = node.required("question")?;
            read_thinkthen_complete_question_document(value, store)
                .map(|value| store.hold(value))?
        },
        question_name: {
            let (presence, value) = read_presence(
                node.member("question_name"),
                false,
                thinkthen_complete_utf8 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence { presence, value }
        },
        schema: {
            let value = node.required("schema")?;
            read_thinkthen_complete_result_tag(value, store).map(|value| store.hold(value))?
        },
        source: {
            let (presence, value) = read_presence(
                node.member("source"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_physical_source(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_physical_source_presence { presence, value }
        },
        threshold: {
            let (presence, value) = read_presence(
                node.member("threshold"),
                true,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_threshold(value, store).map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_threshold_presence { presence, value }
        },
        value: {
            let value = node.required("value")?;
            read_thinkthen_complete_string_list(value, store)?
        },
        extensions: store.extensions(
            node,
            &[
                "answer",
                "answer_id",
                "images",
                "index",
                "input",
                "members",
                "meta",
                "question",
                "question_name",
                "schema",
                "source",
                "threshold",
                "value",
            ],
        )?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_decide_value_presence {
    pub presence: u32,
    pub value: *const thinkthen_complete_decide_value,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_atomic_decide_value {
    pub answer: *const thinkthen_complete_answer,
    pub answer_id: *const thinkthen_complete_answer_id,
    pub images: thinkthen_complete_image_list_presence,
    pub index: thinkthen_complete_u64_presence,
    pub input: thinkthen_complete_json_presence,
    pub members: thinkthen_complete_rank_member_list_presence,
    pub meta: *const thinkthen_complete_meta,
    pub question: *const thinkthen_complete_question_document,
    pub question_name: thinkthen_complete_utf8_presence,
    pub schema: *const thinkthen_complete_result_tag,
    pub source: thinkthen_complete_physical_source_presence,
    pub threshold: thinkthen_complete_threshold_presence,
    pub value: thinkthen_complete_decide_value_presence,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_atomic_decide_value(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_atomic_decide_value, ErrorKind> {
    Ok(thinkthen_complete_atomic_decide_value {
        answer: {
            let value = node.required("answer")?;
            read_thinkthen_complete_answer(value, store).map(|value| store.hold(value))?
        },
        answer_id: {
            let value = node.required("answer_id")?;
            read_thinkthen_complete_answer_id(value, store).map(|value| store.hold(value))?
        },
        images: {
            let (presence, value) = read_presence(
                node.member("images"),
                false,
                thinkthen_complete_image_list {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_image_list,
                store,
            )?;
            thinkthen_complete_image_list_presence { presence, value }
        },
        index: {
            let (presence, value) =
                read_presence(node.member("index"), false, 0, read_number, store)?;
            thinkthen_complete_u64_presence { presence, value }
        },
        input: {
            let (presence, value) = read_presence(
                node.member("input"),
                false,
                std::ptr::null(),
                read_json,
                store,
            )?;
            thinkthen_complete_json_presence { presence, value }
        },
        members: {
            let (presence, value) = read_presence(
                node.member("members"),
                false,
                thinkthen_complete_rank_member_list {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_rank_member_list,
                store,
            )?;
            thinkthen_complete_rank_member_list_presence { presence, value }
        },
        meta: {
            let value = node.required("meta")?;
            read_thinkthen_complete_meta(value, store).map(|value| store.hold(value))?
        },
        question: {
            let value = node.required("question")?;
            read_thinkthen_complete_question_document(value, store)
                .map(|value| store.hold(value))?
        },
        question_name: {
            let (presence, value) = read_presence(
                node.member("question_name"),
                false,
                thinkthen_complete_utf8 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence { presence, value }
        },
        schema: {
            let value = node.required("schema")?;
            read_thinkthen_complete_result_tag(value, store).map(|value| store.hold(value))?
        },
        source: {
            let (presence, value) = read_presence(
                node.member("source"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_physical_source(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_physical_source_presence { presence, value }
        },
        threshold: {
            let (presence, value) = read_presence(
                node.member("threshold"),
                true,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_threshold(value, store).map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_threshold_presence { presence, value }
        },
        value: {
            let (presence, value) = read_presence(
                node.member("value"),
                true,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_decide_value(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_decide_value_presence { presence, value }
        },
        extensions: store.extensions(
            node,
            &[
                "answer",
                "answer_id",
                "images",
                "index",
                "input",
                "members",
                "meta",
                "question",
                "question_name",
                "schema",
                "source",
                "threshold",
                "value",
            ],
        )?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_atomic_non_zero_usize {
    pub answer: *const thinkthen_complete_answer,
    pub answer_id: *const thinkthen_complete_answer_id,
    pub images: thinkthen_complete_image_list_presence,
    pub index: thinkthen_complete_u64_presence,
    pub input: thinkthen_complete_json_presence,
    pub members: thinkthen_complete_rank_member_list_presence,
    pub meta: *const thinkthen_complete_meta,
    pub question: *const thinkthen_complete_question_document,
    pub question_name: thinkthen_complete_utf8_presence,
    pub schema: *const thinkthen_complete_result_tag,
    pub source: thinkthen_complete_physical_source_presence,
    pub threshold: thinkthen_complete_threshold_presence,
    pub value: u64,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_atomic_non_zero_usize(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_atomic_non_zero_usize, ErrorKind> {
    Ok(thinkthen_complete_atomic_non_zero_usize {
        answer: {
            let value = node.required("answer")?;
            read_thinkthen_complete_answer(value, store).map(|value| store.hold(value))?
        },
        answer_id: {
            let value = node.required("answer_id")?;
            read_thinkthen_complete_answer_id(value, store).map(|value| store.hold(value))?
        },
        images: {
            let (presence, value) = read_presence(
                node.member("images"),
                false,
                thinkthen_complete_image_list {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_image_list,
                store,
            )?;
            thinkthen_complete_image_list_presence { presence, value }
        },
        index: {
            let (presence, value) =
                read_presence(node.member("index"), false, 0, read_number, store)?;
            thinkthen_complete_u64_presence { presence, value }
        },
        input: {
            let (presence, value) = read_presence(
                node.member("input"),
                false,
                std::ptr::null(),
                read_json,
                store,
            )?;
            thinkthen_complete_json_presence { presence, value }
        },
        members: {
            let (presence, value) = read_presence(
                node.member("members"),
                false,
                thinkthen_complete_rank_member_list {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_rank_member_list,
                store,
            )?;
            thinkthen_complete_rank_member_list_presence { presence, value }
        },
        meta: {
            let value = node.required("meta")?;
            read_thinkthen_complete_meta(value, store).map(|value| store.hold(value))?
        },
        question: {
            let value = node.required("question")?;
            read_thinkthen_complete_question_document(value, store)
                .map(|value| store.hold(value))?
        },
        question_name: {
            let (presence, value) = read_presence(
                node.member("question_name"),
                false,
                thinkthen_complete_utf8 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence { presence, value }
        },
        schema: {
            let value = node.required("schema")?;
            read_thinkthen_complete_result_tag(value, store).map(|value| store.hold(value))?
        },
        source: {
            let (presence, value) = read_presence(
                node.member("source"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_physical_source(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_physical_source_presence { presence, value }
        },
        threshold: {
            let (presence, value) = read_presence(
                node.member("threshold"),
                true,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_threshold(value, store).map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_threshold_presence { presence, value }
        },
        value: {
            let value = node.required("value")?;
            read_number(value, store)?
        },
        extensions: store.extensions(
            node,
            &[
                "answer",
                "answer_id",
                "images",
                "index",
                "input",
                "members",
                "meta",
                "question",
                "question_name",
                "schema",
                "source",
                "threshold",
                "value",
            ],
        )?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_atomic_nullable_string {
    pub answer: *const thinkthen_complete_answer,
    pub answer_id: *const thinkthen_complete_answer_id,
    pub images: thinkthen_complete_image_list_presence,
    pub index: thinkthen_complete_u64_presence,
    pub input: thinkthen_complete_json_presence,
    pub members: thinkthen_complete_rank_member_list_presence,
    pub meta: *const thinkthen_complete_meta,
    pub question: *const thinkthen_complete_question_document,
    pub question_name: thinkthen_complete_utf8_presence,
    pub schema: *const thinkthen_complete_result_tag,
    pub source: thinkthen_complete_physical_source_presence,
    pub threshold: thinkthen_complete_threshold_presence,
    pub value: thinkthen_complete_utf8_presence,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_atomic_nullable_string(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_atomic_nullable_string, ErrorKind> {
    Ok(thinkthen_complete_atomic_nullable_string {
        answer: {
            let value = node.required("answer")?;
            read_thinkthen_complete_answer(value, store).map(|value| store.hold(value))?
        },
        answer_id: {
            let value = node.required("answer_id")?;
            read_thinkthen_complete_answer_id(value, store).map(|value| store.hold(value))?
        },
        images: {
            let (presence, value) = read_presence(
                node.member("images"),
                false,
                thinkthen_complete_image_list {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_image_list,
                store,
            )?;
            thinkthen_complete_image_list_presence { presence, value }
        },
        index: {
            let (presence, value) =
                read_presence(node.member("index"), false, 0, read_number, store)?;
            thinkthen_complete_u64_presence { presence, value }
        },
        input: {
            let (presence, value) = read_presence(
                node.member("input"),
                false,
                std::ptr::null(),
                read_json,
                store,
            )?;
            thinkthen_complete_json_presence { presence, value }
        },
        members: {
            let (presence, value) = read_presence(
                node.member("members"),
                false,
                thinkthen_complete_rank_member_list {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_rank_member_list,
                store,
            )?;
            thinkthen_complete_rank_member_list_presence { presence, value }
        },
        meta: {
            let value = node.required("meta")?;
            read_thinkthen_complete_meta(value, store).map(|value| store.hold(value))?
        },
        question: {
            let value = node.required("question")?;
            read_thinkthen_complete_question_document(value, store)
                .map(|value| store.hold(value))?
        },
        question_name: {
            let (presence, value) = read_presence(
                node.member("question_name"),
                false,
                thinkthen_complete_utf8 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence { presence, value }
        },
        schema: {
            let value = node.required("schema")?;
            read_thinkthen_complete_result_tag(value, store).map(|value| store.hold(value))?
        },
        source: {
            let (presence, value) = read_presence(
                node.member("source"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_physical_source(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_physical_source_presence { presence, value }
        },
        threshold: {
            let (presence, value) = read_presence(
                node.member("threshold"),
                true,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_threshold(value, store).map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_threshold_presence { presence, value }
        },
        value: {
            let (presence, value) = read_presence(
                node.member("value"),
                true,
                thinkthen_complete_utf8 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence { presence, value }
        },
        extensions: store.extensions(
            node,
            &[
                "answer",
                "answer_id",
                "images",
                "index",
                "input",
                "members",
                "meta",
                "question",
                "question_name",
                "schema",
                "source",
                "threshold",
                "value",
            ],
        )?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_atomic_boolean {
    pub answer: *const thinkthen_complete_answer,
    pub answer_id: *const thinkthen_complete_answer_id,
    pub images: thinkthen_complete_image_list_presence,
    pub index: thinkthen_complete_u64_presence,
    pub input: thinkthen_complete_json_presence,
    pub members: thinkthen_complete_rank_member_list_presence,
    pub meta: *const thinkthen_complete_meta,
    pub question: *const thinkthen_complete_question_document,
    pub question_name: thinkthen_complete_utf8_presence,
    pub schema: *const thinkthen_complete_result_tag,
    pub source: thinkthen_complete_physical_source_presence,
    pub threshold: thinkthen_complete_threshold_presence,
    pub value: u32,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_atomic_boolean(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_atomic_boolean, ErrorKind> {
    Ok(thinkthen_complete_atomic_boolean {
        answer: {
            let value = node.required("answer")?;
            read_thinkthen_complete_answer(value, store).map(|value| store.hold(value))?
        },
        answer_id: {
            let value = node.required("answer_id")?;
            read_thinkthen_complete_answer_id(value, store).map(|value| store.hold(value))?
        },
        images: {
            let (presence, value) = read_presence(
                node.member("images"),
                false,
                thinkthen_complete_image_list {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_image_list,
                store,
            )?;
            thinkthen_complete_image_list_presence { presence, value }
        },
        index: {
            let (presence, value) =
                read_presence(node.member("index"), false, 0, read_number, store)?;
            thinkthen_complete_u64_presence { presence, value }
        },
        input: {
            let (presence, value) = read_presence(
                node.member("input"),
                false,
                std::ptr::null(),
                read_json,
                store,
            )?;
            thinkthen_complete_json_presence { presence, value }
        },
        members: {
            let (presence, value) = read_presence(
                node.member("members"),
                false,
                thinkthen_complete_rank_member_list {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_rank_member_list,
                store,
            )?;
            thinkthen_complete_rank_member_list_presence { presence, value }
        },
        meta: {
            let value = node.required("meta")?;
            read_thinkthen_complete_meta(value, store).map(|value| store.hold(value))?
        },
        question: {
            let value = node.required("question")?;
            read_thinkthen_complete_question_document(value, store)
                .map(|value| store.hold(value))?
        },
        question_name: {
            let (presence, value) = read_presence(
                node.member("question_name"),
                false,
                thinkthen_complete_utf8 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence { presence, value }
        },
        schema: {
            let value = node.required("schema")?;
            read_thinkthen_complete_result_tag(value, store).map(|value| store.hold(value))?
        },
        source: {
            let (presence, value) = read_presence(
                node.member("source"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_physical_source(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_physical_source_presence { presence, value }
        },
        threshold: {
            let (presence, value) = read_presence(
                node.member("threshold"),
                true,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_threshold(value, store).map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_threshold_presence { presence, value }
        },
        value: {
            let value = node.required("value")?;
            read_boolean(value, store)?
        },
        extensions: store.extensions(
            node,
            &[
                "answer",
                "answer_id",
                "images",
                "index",
                "input",
                "members",
                "meta",
                "question",
                "question_name",
                "schema",
                "source",
                "threshold",
                "value",
            ],
        )?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_atomic_double {
    pub answer: *const thinkthen_complete_answer,
    pub answer_id: *const thinkthen_complete_answer_id,
    pub images: thinkthen_complete_image_list_presence,
    pub index: thinkthen_complete_u64_presence,
    pub input: thinkthen_complete_json_presence,
    pub members: thinkthen_complete_rank_member_list_presence,
    pub meta: *const thinkthen_complete_meta,
    pub question: *const thinkthen_complete_question_document,
    pub question_name: thinkthen_complete_utf8_presence,
    pub schema: *const thinkthen_complete_result_tag,
    pub source: thinkthen_complete_physical_source_presence,
    pub threshold: thinkthen_complete_threshold_presence,
    pub value: f64,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_atomic_double(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_atomic_double, ErrorKind> {
    Ok(thinkthen_complete_atomic_double {
        answer: {
            let value = node.required("answer")?;
            read_thinkthen_complete_answer(value, store).map(|value| store.hold(value))?
        },
        answer_id: {
            let value = node.required("answer_id")?;
            read_thinkthen_complete_answer_id(value, store).map(|value| store.hold(value))?
        },
        images: {
            let (presence, value) = read_presence(
                node.member("images"),
                false,
                thinkthen_complete_image_list {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_image_list,
                store,
            )?;
            thinkthen_complete_image_list_presence { presence, value }
        },
        index: {
            let (presence, value) =
                read_presence(node.member("index"), false, 0, read_number, store)?;
            thinkthen_complete_u64_presence { presence, value }
        },
        input: {
            let (presence, value) = read_presence(
                node.member("input"),
                false,
                std::ptr::null(),
                read_json,
                store,
            )?;
            thinkthen_complete_json_presence { presence, value }
        },
        members: {
            let (presence, value) = read_presence(
                node.member("members"),
                false,
                thinkthen_complete_rank_member_list {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_rank_member_list,
                store,
            )?;
            thinkthen_complete_rank_member_list_presence { presence, value }
        },
        meta: {
            let value = node.required("meta")?;
            read_thinkthen_complete_meta(value, store).map(|value| store.hold(value))?
        },
        question: {
            let value = node.required("question")?;
            read_thinkthen_complete_question_document(value, store)
                .map(|value| store.hold(value))?
        },
        question_name: {
            let (presence, value) = read_presence(
                node.member("question_name"),
                false,
                thinkthen_complete_utf8 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence { presence, value }
        },
        schema: {
            let value = node.required("schema")?;
            read_thinkthen_complete_result_tag(value, store).map(|value| store.hold(value))?
        },
        source: {
            let (presence, value) = read_presence(
                node.member("source"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_physical_source(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_physical_source_presence { presence, value }
        },
        threshold: {
            let (presence, value) = read_presence(
                node.member("threshold"),
                true,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_threshold(value, store).map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_threshold_presence { presence, value }
        },
        value: {
            let value = node.required("value")?;
            read_number(value, store)?
        },
        extensions: store.extensions(
            node,
            &[
                "answer",
                "answer_id",
                "images",
                "index",
                "input",
                "members",
                "meta",
                "question",
                "question_name",
                "schema",
                "source",
                "threshold",
                "value",
            ],
        )?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_attempt {
    pub ordinal: u64,
    pub outcome: *const thinkthen_complete_attempt_outcome,
    pub request_id: thinkthen_complete_utf8_presence,
    pub request_sha256: thinkthen_complete_utf8,
    pub sdk_request_id: *const thinkthen_complete_sdk_request_id,
    pub server_ms: thinkthen_complete_u64_presence,
    pub status: thinkthen_complete_u64_presence,
    pub wall_ms: u64,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_attempt(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_attempt, ErrorKind> {
    Ok(thinkthen_complete_attempt {
        ordinal: {
            let value = node.required("ordinal")?;
            read_number(value, store)?
        },
        outcome: {
            let value = node.required("outcome")?;
            read_thinkthen_complete_attempt_outcome(value, store).map(|value| store.hold(value))?
        },
        request_id: {
            let (presence, value) = read_presence(
                node.member("request_id"),
                false,
                thinkthen_complete_utf8 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence { presence, value }
        },
        request_sha256: {
            let value = node.required("request_sha256")?;
            read_text(value, store)?
        },
        sdk_request_id: {
            let value = node.required("sdk_request_id")?;
            read_thinkthen_complete_sdk_request_id(value, store).map(|value| store.hold(value))?
        },
        server_ms: {
            let (presence, value) =
                read_presence(node.member("server_ms"), false, 0, read_number, store)?;
            thinkthen_complete_u64_presence { presence, value }
        },
        status: {
            let (presence, value) =
                read_presence(node.member("status"), false, 0, read_number, store)?;
            thinkthen_complete_u64_presence { presence, value }
        },
        wall_ms: {
            let value = node.required("wall_ms")?;
            read_number(value, store)?
        },
        extensions: store.extensions(
            node,
            &[
                "ordinal",
                "outcome",
                "request_id",
                "request_sha256",
                "sdk_request_id",
                "server_ms",
                "status",
                "wall_ms",
            ],
        )?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_batch_string {
    pub kind: u32,
}

pub const THINKTHEN_COMPLETE_BATCH_STRING_MAX: u32 = 1;

pub(crate) fn read_thinkthen_complete_batch_string(
    node: &Node,
    _store: &mut Storage,
) -> Result<thinkthen_complete_batch_string, ErrorKind> {
    let kind = match node.text()? {
        "max" => THINKTHEN_COMPLETE_BATCH_STRING_MAX,
        _ => return Err(ErrorKind::Defect),
    };
    Ok(thinkthen_complete_batch_string { kind })
}

pub const THINKTHEN_COMPLETE_BATCH_INTEGER: u32 = 1;

pub const THINKTHEN_COMPLETE_BATCH_STRING: u32 = 2;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_batch_data {
    pub integer: u64,
    pub string: *const thinkthen_complete_batch_string,
}
impl std::fmt::Debug for thinkthen_complete_batch_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("union payload")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_batch {
    pub kind: u32,
    pub data: thinkthen_complete_batch_data,
}

pub(crate) fn read_thinkthen_complete_batch(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_batch, ErrorKind> {
    if node.kind() == "number" {
        let value = read_number(node, store)?;
        return Ok(thinkthen_complete_batch {
            kind: THINKTHEN_COMPLETE_BATCH_INTEGER,
            data: thinkthen_complete_batch_data { integer: value },
        });
    }
    if node.kind() == "string" {
        let value =
            read_thinkthen_complete_batch_string(node, store).map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_batch {
            kind: THINKTHEN_COMPLETE_BATCH_STRING,
            data: thinkthen_complete_batch_data { string: value },
        });
    }
    Err(ErrorKind::Defect)
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_boundary_mode {
    pub kind: u32,
}

pub const THINKTHEN_COMPLETE_BOUNDARY_MODE_BOUNDARY_ONLY: u32 = 1;

pub(crate) fn read_thinkthen_complete_boundary_mode(
    node: &Node,
    _store: &mut Storage,
) -> Result<thinkthen_complete_boundary_mode, ErrorKind> {
    let kind = match node.text()? {
        "boundary_only" => THINKTHEN_COMPLETE_BOUNDARY_MODE_BOUNDARY_ONLY,
        _ => return Err(ErrorKind::Defect),
    };
    Ok(thinkthen_complete_boundary_mode { kind })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_boundary_proposal {
    pub end: u64,
    pub file: thinkthen_complete_utf8_presence,
    pub first_line: thinkthen_complete_u64_presence,
    pub last_line: thinkthen_complete_u64_presence,
    pub length: u64,
    pub probability: f64,
    pub start: u64,
    pub text: thinkthen_complete_utf8,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_boundary_proposal(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_boundary_proposal, ErrorKind> {
    Ok(thinkthen_complete_boundary_proposal {
        end: {
            let value = node.required("end")?;
            read_number(value, store)?
        },
        file: {
            let (presence, value) = read_presence(
                node.member("file"),
                false,
                thinkthen_complete_utf8 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence { presence, value }
        },
        first_line: {
            let (presence, value) =
                read_presence(node.member("first_line"), false, 0, read_number, store)?;
            thinkthen_complete_u64_presence { presence, value }
        },
        last_line: {
            let (presence, value) =
                read_presence(node.member("last_line"), false, 0, read_number, store)?;
            thinkthen_complete_u64_presence { presence, value }
        },
        length: {
            let value = node.required("length")?;
            read_number(value, store)?
        },
        probability: {
            let value = node.required("probability")?;
            read_number(value, store)?
        },
        start: {
            let value = node.required("start")?;
            read_number(value, store)?
        },
        text: {
            let value = node.required("text")?;
            read_text(value, store)?
        },
        extensions: store.extensions(
            node,
            &[
                "end",
                "file",
                "first_line",
                "last_line",
                "length",
                "probability",
                "start",
                "text",
            ],
        )?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_facts_presence {
    pub presence: u32,
    pub value: *const thinkthen_complete_facts,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_call_error {
    pub error: *const thinkthen_complete_error,
    pub facts: thinkthen_complete_facts_presence,
    pub resolution: *const thinkthen_complete_answer_resolution,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_call_error(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_call_error, ErrorKind> {
    Ok(thinkthen_complete_call_error {
        error: {
            let value = node.required("error")?;
            read_thinkthen_complete_error(value, store).map(|value| store.hold(value))?
        },
        facts: {
            let (presence, value) = read_presence(
                node.member("facts"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_facts(value, store).map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_facts_presence { presence, value }
        },
        resolution: {
            let value = node.required("resolution")?;
            read_thinkthen_complete_answer_resolution(value, store)
                .map(|value| store.hold(value))?
        },
        extensions: store.extensions(node, &["error", "facts", "resolution"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_call_id {
    pub value: thinkthen_complete_utf8,
}

pub(crate) fn read_thinkthen_complete_call_id(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_call_id, ErrorKind> {
    let value = read_text(node, store)?;
    Ok(thinkthen_complete_call_id { value })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_decide_value {
    pub value: *const thinkthen_complete_json,
}

pub(crate) fn read_thinkthen_complete_decide_value(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_decide_value, ErrorKind> {
    let value = read_json(node, store)?;
    Ok(thinkthen_complete_decide_value { value })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_entity_document {
    pub kind: thinkthen_complete_utf8,
    pub name: thinkthen_complete_utf8,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_entity_document(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_entity_document, ErrorKind> {
    Ok(thinkthen_complete_entity_document {
        kind: {
            let value = node.required("kind")?;
            read_text(value, store)?
        },
        name: {
            let value = node.required("name")?;
            read_text(value, store)?
        },
        extensions: store.extensions(node, &["kind", "name"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_input_denial_presence {
    pub presence: u32,
    pub value: *const thinkthen_complete_input_denial,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_send_denial_presence {
    pub presence: u32,
    pub value: *const thinkthen_complete_send_denial,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_error {
    pub estimated_input_denial: thinkthen_complete_input_denial_presence,
    pub kind: *const thinkthen_complete_failure_kind,
    pub message: thinkthen_complete_utf8,
    pub retryable: u32,
    pub send_budget_denial: thinkthen_complete_send_denial_presence,
    pub stopped: *const thinkthen_complete_stopped,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_error(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_error, ErrorKind> {
    Ok(thinkthen_complete_error {
        estimated_input_denial: {
            let (presence, value) = read_presence(
                node.member("estimated_input_denial"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_input_denial(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_input_denial_presence { presence, value }
        },
        kind: {
            let value = node.required("kind")?;
            read_thinkthen_complete_failure_kind(value, store).map(|value| store.hold(value))?
        },
        message: {
            let value = node.required("message")?;
            read_text(value, store)?
        },
        retryable: {
            let value = node.required("retryable")?;
            read_boolean(value, store)?
        },
        send_budget_denial: {
            let (presence, value) = read_presence(
                node.member("send_budget_denial"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_send_denial(value, store).map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_send_denial_presence { presence, value }
        },
        stopped: {
            let value = node.required("stopped")?;
            read_thinkthen_complete_stopped(value, store).map(|value| store.hold(value))?
        },
        extensions: store.extensions(
            node,
            &[
                "estimated_input_denial",
                "kind",
                "message",
                "retryable",
                "send_budget_denial",
                "stopped",
            ],
        )?,
    })
}

pub const THINKTHEN_COMPLETE_INPUT_DENIAL_INITIAL_REQUEST: u32 = 1;

pub const THINKTHEN_COMPLETE_INPUT_DENIAL_ADDITIONAL_REQUEST: u32 = 2;

pub const THINKTHEN_COMPLETE_INPUT_DENIAL_RETRY: u32 = 3;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_input_denial_data {
    pub initial_request: *const thinkthen_complete_input_denial_initial_request,
    pub additional_request: *const thinkthen_complete_input_denial_additional_request,
    pub retry: *const thinkthen_complete_input_denial_retry,
}
impl std::fmt::Debug for thinkthen_complete_input_denial_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("union payload")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_input_denial {
    pub kind: u32,
    pub data: thinkthen_complete_input_denial_data,
}

pub(crate) fn read_thinkthen_complete_input_denial(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_input_denial, ErrorKind> {
    if node.is_member("kind", "initial_request") {
        let value = read_thinkthen_complete_input_denial_initial_request(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_input_denial {
            kind: THINKTHEN_COMPLETE_INPUT_DENIAL_INITIAL_REQUEST,
            data: thinkthen_complete_input_denial_data {
                initial_request: value,
            },
        });
    }
    if node.is_member("kind", "additional_request") {
        let value = read_thinkthen_complete_input_denial_additional_request(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_input_denial {
            kind: THINKTHEN_COMPLETE_INPUT_DENIAL_ADDITIONAL_REQUEST,
            data: thinkthen_complete_input_denial_data {
                additional_request: value,
            },
        });
    }
    if node.is_member("kind", "retry") {
        let value = read_thinkthen_complete_input_denial_retry(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_input_denial {
            kind: THINKTHEN_COMPLETE_INPUT_DENIAL_RETRY,
            data: thinkthen_complete_input_denial_data { retry: value },
        });
    }
    Err(ErrorKind::Defect)
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_input_denial_additional_request {
    pub kind: thinkthen_complete_utf8,
    pub limit: u64,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_input_denial_additional_request(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_input_denial_additional_request, ErrorKind> {
    Ok(thinkthen_complete_input_denial_additional_request {
        kind: {
            let value = node.required("kind")?;
            read_text(value, store)?
        },
        limit: {
            let value = node.required("limit")?;
            read_number(value, store)?
        },
        extensions: store.extensions(node, &["kind", "limit"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_input_denial_initial_request {
    pub kind: thinkthen_complete_utf8,
    pub limit: u64,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_input_denial_initial_request(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_input_denial_initial_request, ErrorKind> {
    Ok(thinkthen_complete_input_denial_initial_request {
        kind: {
            let value = node.required("kind")?;
            read_text(value, store)?
        },
        limit: {
            let value = node.required("limit")?;
            read_number(value, store)?
        },
        extensions: store.extensions(node, &["kind", "limit"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_input_denial_retry {
    pub kind: thinkthen_complete_utf8,
    pub last_status: u64,
    pub limit: u64,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_input_denial_retry(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_input_denial_retry, ErrorKind> {
    Ok(thinkthen_complete_input_denial_retry {
        kind: {
            let value = node.required("kind")?;
            read_text(value, store)?
        },
        last_status: {
            let value = node.required("last_status")?;
            read_number(value, store)?
        },
        limit: {
            let value = node.required("limit")?;
            read_number(value, store)?
        },
        extensions: store.extensions(node, &["kind", "last_status", "limit"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_attempt_list {
    pub data: *const *const thinkthen_complete_attempt,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_attempt_list(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_attempt_list, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(read_thinkthen_complete_attempt(item, store).map(|value| store.hold(value))?);
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_attempt_list { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_attempt_list_presence {
    pub presence: u32,
    pub value: thinkthen_complete_attempt_list,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_persistence_observation_presence {
    pub presence: u32,
    pub value: *const thinkthen_complete_persistence_observation,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_facts {
    pub attempts: thinkthen_complete_attempt_list_presence,
    pub cache_answers: u64,
    pub call_id: *const thinkthen_complete_call_id,
    pub estimated_cost_usd: thinkthen_complete_utf8_presence,
    pub held_model_mismatch: thinkthen_complete_u32_presence,
    pub input_tokens: thinkthen_complete_u64_presence,
    pub largest_request_bytes: u64,
    pub largest_request_estimated_input_tokens: thinkthen_complete_u64_presence,
    pub model: thinkthen_complete_utf8_presence,
    pub output_tokens: thinkthen_complete_u64_presence,
    pub records: u64,
    pub requests_sent: u64,
    pub seconds: f64,
    pub token_estimate_method: thinkthen_complete_utf8,
    pub usage_persistence: thinkthen_complete_persistence_observation_presence,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_facts(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_facts, ErrorKind> {
    Ok(thinkthen_complete_facts {
        attempts: {
            let (presence, value) = read_presence(
                node.member("attempts"),
                false,
                thinkthen_complete_attempt_list {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_attempt_list,
                store,
            )?;
            thinkthen_complete_attempt_list_presence { presence, value }
        },
        cache_answers: {
            let value = node.required("cache_answers")?;
            read_number(value, store)?
        },
        call_id: {
            let value = node.required("call_id")?;
            read_thinkthen_complete_call_id(value, store).map(|value| store.hold(value))?
        },
        estimated_cost_usd: {
            let (presence, value) = read_presence(
                node.member("estimated_cost_usd"),
                false,
                thinkthen_complete_utf8 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence { presence, value }
        },
        held_model_mismatch: {
            let (presence, value) = read_presence(
                node.member("held_model_mismatch"),
                false,
                0,
                read_boolean,
                store,
            )?;
            thinkthen_complete_u32_presence { presence, value }
        },
        input_tokens: {
            let (presence, value) =
                read_presence(node.member("input_tokens"), false, 0, read_number, store)?;
            thinkthen_complete_u64_presence { presence, value }
        },
        largest_request_bytes: {
            let value = node.required("largest_request_bytes")?;
            read_number(value, store)?
        },
        largest_request_estimated_input_tokens: {
            let (presence, value) = read_presence(
                node.member("largest_request_estimated_input_tokens"),
                true,
                0,
                read_number,
                store,
            )?;
            thinkthen_complete_u64_presence { presence, value }
        },
        model: {
            let (presence, value) = read_presence(
                node.member("model"),
                false,
                thinkthen_complete_utf8 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence { presence, value }
        },
        output_tokens: {
            let (presence, value) =
                read_presence(node.member("output_tokens"), false, 0, read_number, store)?;
            thinkthen_complete_u64_presence { presence, value }
        },
        records: {
            let value = node.required("records")?;
            read_number(value, store)?
        },
        requests_sent: {
            let value = node.required("requests_sent")?;
            read_number(value, store)?
        },
        seconds: {
            let value = node.required("seconds")?;
            read_number(value, store)?
        },
        token_estimate_method: {
            let value = node.required("token_estimate_method")?;
            read_text(value, store)?
        },
        usage_persistence: {
            let (presence, value) = read_presence(
                node.member("usage_persistence"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_persistence_observation(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_persistence_observation_presence { presence, value }
        },
        extensions: store.extensions(
            node,
            &[
                "attempts",
                "cache_answers",
                "call_id",
                "estimated_cost_usd",
                "held_model_mismatch",
                "input_tokens",
                "largest_request_bytes",
                "largest_request_estimated_input_tokens",
                "model",
                "output_tokens",
                "records",
                "requests_sent",
                "seconds",
                "token_estimate_method",
                "usage_persistence",
            ],
        )?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_failure_id {
    pub value: thinkthen_complete_utf8,
}

pub(crate) fn read_thinkthen_complete_failure_id(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_failure_id, ErrorKind> {
    let value = read_text(node, store)?;
    Ok(thinkthen_complete_failure_id { value })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_find_candidate_list {
    pub data: *const *const thinkthen_complete_find_candidate,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_find_candidate_list(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_find_candidate_list, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_find_candidate(item, store).map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_find_candidate_list { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_find_candidate_list_presence {
    pub presence: u32,
    pub value: thinkthen_complete_find_candidate_list,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_find {
    pub answer: *const thinkthen_complete_find_answer,
    pub answer_id: *const thinkthen_complete_answer_id,
    pub candidates: thinkthen_complete_find_candidate_list_presence,
    pub file: thinkthen_complete_utf8_presence,
    pub first_line: thinkthen_complete_u64_presence,
    pub index: thinkthen_complete_u64_presence,
    pub last_line: thinkthen_complete_u64_presence,
    pub meta: *const thinkthen_complete_meta,
    pub position: thinkthen_complete_position_presence,
    pub question: *const thinkthen_complete_find_question_document,
    pub schema: *const thinkthen_complete_result_tag,
    pub threshold: thinkthen_complete_json_presence,
    pub value: thinkthen_complete_json_presence,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_find(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_find, ErrorKind> {
    Ok(thinkthen_complete_find {
        answer: {
            let value = node.required("answer")?;
            read_thinkthen_complete_find_answer(value, store).map(|value| store.hold(value))?
        },
        answer_id: {
            let value = node.required("answer_id")?;
            read_thinkthen_complete_answer_id(value, store).map(|value| store.hold(value))?
        },
        candidates: {
            let (presence, value) = read_presence(
                node.member("candidates"),
                false,
                thinkthen_complete_find_candidate_list {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_find_candidate_list,
                store,
            )?;
            thinkthen_complete_find_candidate_list_presence { presence, value }
        },
        file: {
            let (presence, value) = read_presence(
                node.member("file"),
                false,
                thinkthen_complete_utf8 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence { presence, value }
        },
        first_line: {
            let (presence, value) =
                read_presence(node.member("first_line"), false, 0, read_number, store)?;
            thinkthen_complete_u64_presence { presence, value }
        },
        index: {
            let (presence, value) =
                read_presence(node.member("index"), true, 0, read_number, store)?;
            thinkthen_complete_u64_presence { presence, value }
        },
        last_line: {
            let (presence, value) =
                read_presence(node.member("last_line"), false, 0, read_number, store)?;
            thinkthen_complete_u64_presence { presence, value }
        },
        meta: {
            let value = node.required("meta")?;
            read_thinkthen_complete_meta(value, store).map(|value| store.hold(value))?
        },
        position: {
            let (presence, value) = read_presence(
                node.member("position"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_position(value, store).map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_position_presence { presence, value }
        },
        question: {
            let value = node.required("question")?;
            read_thinkthen_complete_find_question_document(value, store)
                .map(|value| store.hold(value))?
        },
        schema: {
            let value = node.required("schema")?;
            read_thinkthen_complete_result_tag(value, store).map(|value| store.hold(value))?
        },
        threshold: {
            let (presence, value) = read_presence(
                node.member("threshold"),
                true,
                std::ptr::null(),
                read_json,
                store,
            )?;
            thinkthen_complete_json_presence { presence, value }
        },
        value: {
            let (presence, value) = read_presence(
                node.member("value"),
                true,
                std::ptr::null(),
                read_json,
                store,
            )?;
            thinkthen_complete_json_presence { presence, value }
        },
        extensions: store.extensions(
            node,
            &[
                "answer",
                "answer_id",
                "candidates",
                "file",
                "first_line",
                "index",
                "last_line",
                "meta",
                "position",
                "question",
                "schema",
                "threshold",
                "value",
            ],
        )?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_find_candidate {
    pub index: thinkthen_complete_u64_presence,
    pub input: thinkthen_complete_json_presence,
    pub probability: f64,
    pub source: thinkthen_complete_physical_source_presence,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_find_candidate(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_find_candidate, ErrorKind> {
    Ok(thinkthen_complete_find_candidate {
        index: {
            let (presence, value) =
                read_presence(node.member("index"), true, 0, read_number, store)?;
            thinkthen_complete_u64_presence { presence, value }
        },
        input: {
            let (presence, value) = read_presence(
                node.member("input"),
                true,
                std::ptr::null(),
                read_json,
                store,
            )?;
            thinkthen_complete_json_presence { presence, value }
        },
        probability: {
            let value = node.required("probability")?;
            read_number(value, store)?
        },
        source: {
            let (presence, value) = read_presence(
                node.member("source"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_physical_source(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_physical_source_presence { presence, value }
        },
        extensions: store.extensions(node, &["index", "input", "probability", "source"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_image {
    pub base64: thinkthen_complete_utf8,
    pub height: u64,
    pub media: *const thinkthen_complete_image_media,
    pub width: u64,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_image(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_image, ErrorKind> {
    Ok(thinkthen_complete_image {
        base64: {
            let value = node.required("base64")?;
            read_text(value, store)?
        },
        height: {
            let value = node.required("height")?;
            read_number(value, store)?
        },
        media: {
            let value = node.required("media")?;
            read_thinkthen_complete_image_media(value, store).map(|value| store.hold(value))?
        },
        width: {
            let value = node.required("width")?;
            read_number(value, store)?
        },
        extensions: store.extensions(node, &["base64", "height", "media", "width"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_image_media {
    pub kind: u32,
}

pub const THINKTHEN_COMPLETE_IMAGE_MEDIA_IMAGE_JPEG: u32 = 1;

pub const THINKTHEN_COMPLETE_IMAGE_MEDIA_IMAGE_PNG: u32 = 2;

pub(crate) fn read_thinkthen_complete_image_media(
    node: &Node,
    _store: &mut Storage,
) -> Result<thinkthen_complete_image_media, ErrorKind> {
    let kind = match node.text()? {
        "image/jpeg" => THINKTHEN_COMPLETE_IMAGE_MEDIA_IMAGE_JPEG,
        "image/png" => THINKTHEN_COMPLETE_IMAGE_MEDIA_IMAGE_PNG,
        _ => return Err(ErrorKind::Defect),
    };
    Ok(thinkthen_complete_image_media { kind })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_call_id_presence {
    pub presence: u32,
    pub value: *const thinkthen_complete_call_id,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_immediate_facts {
    pub attempts: thinkthen_complete_attempt_list_presence,
    pub cache_answers: u64,
    pub call_id: thinkthen_complete_call_id_presence,
    pub estimated_cost_usd: thinkthen_complete_utf8_presence,
    pub held_model_mismatch: thinkthen_complete_u32_presence,
    pub input_tokens: thinkthen_complete_u64_presence,
    pub largest_request_bytes: u64,
    pub largest_request_estimated_input_tokens: thinkthen_complete_u64_presence,
    pub model: thinkthen_complete_utf8_presence,
    pub output_tokens: thinkthen_complete_u64_presence,
    pub records: u64,
    pub requests_sent: u64,
    pub seconds: f64,
    pub token_estimate_method: thinkthen_complete_utf8,
    pub usage_persistence: thinkthen_complete_persistence_observation_presence,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_immediate_facts(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_immediate_facts, ErrorKind> {
    Ok(thinkthen_complete_immediate_facts {
        attempts: {
            let (presence, value) = read_presence(
                node.member("attempts"),
                false,
                thinkthen_complete_attempt_list {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_attempt_list,
                store,
            )?;
            thinkthen_complete_attempt_list_presence { presence, value }
        },
        cache_answers: {
            let value = node.required("cache_answers")?;
            read_number(value, store)?
        },
        call_id: {
            let (presence, value) = read_presence(
                node.member("call_id"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_call_id(value, store).map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_call_id_presence { presence, value }
        },
        estimated_cost_usd: {
            let (presence, value) = read_presence(
                node.member("estimated_cost_usd"),
                false,
                thinkthen_complete_utf8 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence { presence, value }
        },
        held_model_mismatch: {
            let (presence, value) = read_presence(
                node.member("held_model_mismatch"),
                false,
                0,
                read_boolean,
                store,
            )?;
            thinkthen_complete_u32_presence { presence, value }
        },
        input_tokens: {
            let (presence, value) =
                read_presence(node.member("input_tokens"), false, 0, read_number, store)?;
            thinkthen_complete_u64_presence { presence, value }
        },
        largest_request_bytes: {
            let value = node.required("largest_request_bytes")?;
            read_number(value, store)?
        },
        largest_request_estimated_input_tokens: {
            let (presence, value) = read_presence(
                node.member("largest_request_estimated_input_tokens"),
                true,
                0,
                read_number,
                store,
            )?;
            thinkthen_complete_u64_presence { presence, value }
        },
        model: {
            let (presence, value) = read_presence(
                node.member("model"),
                false,
                thinkthen_complete_utf8 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence { presence, value }
        },
        output_tokens: {
            let (presence, value) =
                read_presence(node.member("output_tokens"), false, 0, read_number, store)?;
            thinkthen_complete_u64_presence { presence, value }
        },
        records: {
            let value = node.required("records")?;
            read_number(value, store)?
        },
        requests_sent: {
            let value = node.required("requests_sent")?;
            read_number(value, store)?
        },
        seconds: {
            let value = node.required("seconds")?;
            read_number(value, store)?
        },
        token_estimate_method: {
            let value = node.required("token_estimate_method")?;
            read_text(value, store)?
        },
        usage_persistence: {
            let (presence, value) = read_presence(
                node.member("usage_persistence"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_persistence_observation(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_persistence_observation_presence { presence, value }
        },
        extensions: store.extensions(
            node,
            &[
                "attempts",
                "cache_answers",
                "call_id",
                "estimated_cost_usd",
                "held_model_mismatch",
                "input_tokens",
                "largest_request_bytes",
                "largest_request_estimated_input_tokens",
                "model",
                "output_tokens",
                "records",
                "requests_sent",
                "seconds",
                "token_estimate_method",
                "usage_persistence",
            ],
        )?,
    })
}

pub const THINKTHEN_COMPLETE_INPUT_DECLARATION_STRING_ROOT: u32 = 1;

pub const THINKTHEN_COMPLETE_INPUT_DECLARATION_OBJECT_ROOT: u32 = 2;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_input_declaration_data {
    pub string_root: *const thinkthen_complete_input_declaration_string_root,
    pub object_root: *const thinkthen_complete_input_declaration_object_root,
}
impl std::fmt::Debug for thinkthen_complete_input_declaration_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("union payload")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_input_declaration {
    pub kind: u32,
    pub data: thinkthen_complete_input_declaration_data,
}

pub(crate) fn read_thinkthen_complete_input_declaration(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_input_declaration, ErrorKind> {
    if node.is_member("type", "string") {
        let value = read_thinkthen_complete_input_declaration_string_root(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_input_declaration {
            kind: THINKTHEN_COMPLETE_INPUT_DECLARATION_STRING_ROOT,
            data: thinkthen_complete_input_declaration_data { string_root: value },
        });
    }
    if node.is_member("type", "object") {
        let value = read_thinkthen_complete_input_declaration_object_root(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_input_declaration {
            kind: THINKTHEN_COMPLETE_INPUT_DECLARATION_OBJECT_ROOT,
            data: thinkthen_complete_input_declaration_data { object_root: value },
        });
    }
    Err(ErrorKind::Defect)
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_input_property_type_map_entry {
    pub name: thinkthen_complete_utf8,
    pub value: *const thinkthen_complete_input_property_type,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_input_property_type_map {
    pub data: *const thinkthen_complete_input_property_type_map_entry,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_input_property_type_map(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_input_property_type_map, ErrorKind> {
    let mut values = Vec::new();
    for (key, item) in node.object()? {
        values.push(thinkthen_complete_input_property_type_map_entry {
            name: store.text(key),
            value: read_thinkthen_complete_input_property_type(item, store)
                .map(|value| store.hold(value))?,
        });
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_input_property_type_map { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_string_list_presence {
    pub presence: u32,
    pub value: thinkthen_complete_string_list,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_input_declaration_object_root {
    pub properties: thinkthen_complete_input_property_type_map,
    pub required: thinkthen_complete_string_list_presence,
    pub r#type: *const thinkthen_complete_object_type,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_input_declaration_object_root(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_input_declaration_object_root, ErrorKind> {
    Ok(thinkthen_complete_input_declaration_object_root {
        properties: {
            let value = node.required("properties")?;
            read_thinkthen_complete_input_property_type_map(value, store)?
        },
        required: {
            let (presence, value) = read_presence(
                node.member("required"),
                false,
                thinkthen_complete_string_list {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_string_list,
                store,
            )?;
            thinkthen_complete_string_list_presence { presence, value }
        },
        r#type: {
            let value = node.required("type")?;
            read_thinkthen_complete_object_type(value, store).map(|value| store.hold(value))?
        },
        extensions: store.extensions(node, &["properties", "required", "type"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_input_declaration_string_root {
    pub r#type: *const thinkthen_complete_string_type,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_input_declaration_string_root(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_input_declaration_string_root, ErrorKind> {
    Ok(thinkthen_complete_input_declaration_string_root {
        r#type: {
            let value = node.required("type")?;
            read_thinkthen_complete_string_type(value, store).map(|value| store.hold(value))?
        },
        extensions: store.extensions(node, &["type"])?,
    })
}

pub const THINKTHEN_COMPLETE_INPUT_PROPERTY_TYPE_STRING: u32 = 1;

pub const THINKTHEN_COMPLETE_INPUT_PROPERTY_TYPE_NUMBER: u32 = 2;

pub const THINKTHEN_COMPLETE_INPUT_PROPERTY_TYPE_BOOLEAN: u32 = 3;

pub const THINKTHEN_COMPLETE_INPUT_PROPERTY_TYPE_ARRAY: u32 = 4;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_input_property_type_data {
    pub string: *const thinkthen_complete_input_property_type_string,
    pub number: *const thinkthen_complete_input_property_type_number,
    pub boolean: *const thinkthen_complete_input_property_type_boolean,
    pub array: *const thinkthen_complete_input_property_type_array,
}
impl std::fmt::Debug for thinkthen_complete_input_property_type_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("union payload")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_input_property_type {
    pub kind: u32,
    pub data: thinkthen_complete_input_property_type_data,
}

pub(crate) fn read_thinkthen_complete_input_property_type(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_input_property_type, ErrorKind> {
    if node.is_member("type", "string") {
        let value = read_thinkthen_complete_input_property_type_string(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_input_property_type {
            kind: THINKTHEN_COMPLETE_INPUT_PROPERTY_TYPE_STRING,
            data: thinkthen_complete_input_property_type_data { string: value },
        });
    }
    if node.is_member("type", "number") {
        let value = read_thinkthen_complete_input_property_type_number(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_input_property_type {
            kind: THINKTHEN_COMPLETE_INPUT_PROPERTY_TYPE_NUMBER,
            data: thinkthen_complete_input_property_type_data { number: value },
        });
    }
    if node.is_member("type", "boolean") {
        let value = read_thinkthen_complete_input_property_type_boolean(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_input_property_type {
            kind: THINKTHEN_COMPLETE_INPUT_PROPERTY_TYPE_BOOLEAN,
            data: thinkthen_complete_input_property_type_data { boolean: value },
        });
    }
    if node.is_member("type", "array") {
        let value = read_thinkthen_complete_input_property_type_array(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_input_property_type {
            kind: THINKTHEN_COMPLETE_INPUT_PROPERTY_TYPE_ARRAY,
            data: thinkthen_complete_input_property_type_data { array: value },
        });
    }
    Err(ErrorKind::Defect)
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_input_property_type_array {
    pub items: *const thinkthen_complete_string_root,
    pub r#type: thinkthen_complete_utf8,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_input_property_type_array(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_input_property_type_array, ErrorKind> {
    Ok(thinkthen_complete_input_property_type_array {
        items: {
            let value = node.required("items")?;
            read_thinkthen_complete_string_root(value, store).map(|value| store.hold(value))?
        },
        r#type: {
            let value = node.required("type")?;
            read_text(value, store)?
        },
        extensions: store.extensions(node, &["items", "type"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_input_property_type_boolean {
    pub r#type: thinkthen_complete_utf8,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_input_property_type_boolean(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_input_property_type_boolean, ErrorKind> {
    Ok(thinkthen_complete_input_property_type_boolean {
        r#type: {
            let value = node.required("type")?;
            read_text(value, store)?
        },
        extensions: store.extensions(node, &["type"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_input_property_type_number {
    pub r#type: thinkthen_complete_utf8,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_input_property_type_number(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_input_property_type_number, ErrorKind> {
    Ok(thinkthen_complete_input_property_type_number {
        r#type: {
            let value = node.required("type")?;
            read_text(value, store)?
        },
        extensions: store.extensions(node, &["type"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_input_property_type_string {
    pub r#type: thinkthen_complete_utf8,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_input_property_type_string(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_input_property_type_string, ErrorKind> {
    Ok(thinkthen_complete_input_property_type_string {
        r#type: {
            let value = node.required("type")?;
            read_text(value, store)?
        },
        extensions: store.extensions(node, &["type"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_label {
    pub description: thinkthen_complete_json_presence,
    pub name: thinkthen_complete_utf8,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_label(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_label, ErrorKind> {
    Ok(thinkthen_complete_label {
        description: {
            let (presence, value) = read_presence(
                node.member("description"),
                false,
                std::ptr::null(),
                read_json,
                store,
            )?;
            thinkthen_complete_json_presence { presence, value }
        },
        name: {
            let value = node.required("name")?;
            read_text(value, store)?
        },
        extensions: store.extensions(node, &["description", "name"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_batch_setting_presence {
    pub presence: u32,
    pub value: *const thinkthen_complete_batch_setting,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_batch_warning_presence {
    pub presence: u32,
    pub value: *const thinkthen_complete_batch_warning,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_origin_presence {
    pub presence: u32,
    pub value: *const thinkthen_complete_origin,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_profile_warning_presence {
    pub presence: u32,
    pub value: *const thinkthen_complete_profile_warning,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_meta {
    pub answered_by: thinkthen_complete_utf8_presence,
    pub attempts: thinkthen_complete_attempt_list_presence,
    pub batch_setting: thinkthen_complete_batch_setting_presence,
    pub batch_warning: thinkthen_complete_batch_warning_presence,
    pub cached: u32,
    pub context_sha256: thinkthen_complete_utf8_presence,
    pub failed_questions: u64,
    pub model: thinkthen_complete_utf8,
    pub observations: thinkthen_complete_observation_list,
    pub origin: thinkthen_complete_origin_presence,
    pub profile_warning: thinkthen_complete_profile_warning_presence,
    pub question_sha256: thinkthen_complete_utf8_presence,
    pub question_sources: thinkthen_complete_question_source_list,
    pub questions_sha256: thinkthen_complete_utf8_presence,
    pub requests: thinkthen_complete_string_list,
    pub requests_sent: u64,
    pub tool: thinkthen_complete_utf8,
    pub url: thinkthen_complete_utf8,
    pub usage: thinkthen_complete_usage_presence,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_meta(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_meta, ErrorKind> {
    Ok(thinkthen_complete_meta {
        answered_by: {
            let (presence, value) = read_presence(
                node.member("answered_by"),
                false,
                thinkthen_complete_utf8 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence { presence, value }
        },
        attempts: {
            let (presence, value) = read_presence(
                node.member("attempts"),
                false,
                thinkthen_complete_attempt_list {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_attempt_list,
                store,
            )?;
            thinkthen_complete_attempt_list_presence { presence, value }
        },
        batch_setting: {
            let (presence, value) = read_presence(
                node.member("batch_setting"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_batch_setting(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_batch_setting_presence { presence, value }
        },
        batch_warning: {
            let (presence, value) = read_presence(
                node.member("batch_warning"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_batch_warning(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_batch_warning_presence { presence, value }
        },
        cached: {
            let value = node.required("cached")?;
            read_boolean(value, store)?
        },
        context_sha256: {
            let (presence, value) = read_presence(
                node.member("context_sha256"),
                false,
                thinkthen_complete_utf8 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence { presence, value }
        },
        failed_questions: {
            let value = node.required("failed_questions")?;
            read_number(value, store)?
        },
        model: {
            let value = node.required("model")?;
            read_text(value, store)?
        },
        observations: {
            let value = node.required("observations")?;
            read_thinkthen_complete_observation_list(value, store)?
        },
        origin: {
            let (presence, value) = read_presence(
                node.member("origin"),
                true,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_origin(value, store).map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_origin_presence { presence, value }
        },
        profile_warning: {
            let (presence, value) = read_presence(
                node.member("profile_warning"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_profile_warning(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_profile_warning_presence { presence, value }
        },
        question_sha256: {
            let (presence, value) = read_presence(
                node.member("question_sha256"),
                false,
                thinkthen_complete_utf8 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence { presence, value }
        },
        question_sources: {
            let value = node.required("question_sources")?;
            read_thinkthen_complete_question_source_list(value, store)?
        },
        questions_sha256: {
            let (presence, value) = read_presence(
                node.member("questions_sha256"),
                false,
                thinkthen_complete_utf8 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence { presence, value }
        },
        requests: {
            let value = node.required("requests")?;
            read_thinkthen_complete_string_list(value, store)?
        },
        requests_sent: {
            let value = node.required("requests_sent")?;
            read_number(value, store)?
        },
        tool: {
            let value = node.required("tool")?;
            read_text(value, store)?
        },
        url: {
            let value = node.required("url")?;
            read_text(value, store)?
        },
        usage: {
            let (presence, value) = read_presence(
                node.member("usage"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_usage(value, store).map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_usage_presence { presence, value }
        },
        extensions: store.extensions(
            node,
            &[
                "answered_by",
                "attempts",
                "batch_setting",
                "batch_warning",
                "cached",
                "context_sha256",
                "failed_questions",
                "model",
                "observations",
                "origin",
                "profile_warning",
                "question_sha256",
                "question_sources",
                "questions_sha256",
                "requests",
                "requests_sent",
                "tool",
                "url",
                "usage",
            ],
        )?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_object_type {
    pub kind: u32,
}

pub const THINKTHEN_COMPLETE_OBJECT_TYPE_OBJECT: u32 = 1;

pub(crate) fn read_thinkthen_complete_object_type(
    node: &Node,
    _store: &mut Storage,
) -> Result<thinkthen_complete_object_type, ErrorKind> {
    let kind = match node.text()? {
        "object" => THINKTHEN_COMPLETE_OBJECT_TYPE_OBJECT,
        _ => return Err(ErrorKind::Defect),
    };
    Ok(thinkthen_complete_object_type { kind })
}

pub const THINKTHEN_COMPLETE_OBSERVATION_OBSERVATION_ID: u32 = 1;

pub const THINKTHEN_COMPLETE_OBSERVATION_FAILURE_ID: u32 = 2;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_observation_data {
    pub observation_id: *const thinkthen_complete_observation_observation_id,
    pub failure_id: *const thinkthen_complete_observation_failure_id,
}
impl std::fmt::Debug for thinkthen_complete_observation_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("union payload")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_observation {
    pub kind: u32,
    pub data: thinkthen_complete_observation_data,
}

pub(crate) fn read_thinkthen_complete_observation(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_observation, ErrorKind> {
    if node.member("observation_id").is_some() {
        let value = read_thinkthen_complete_observation_observation_id(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_observation {
            kind: THINKTHEN_COMPLETE_OBSERVATION_OBSERVATION_ID,
            data: thinkthen_complete_observation_data {
                observation_id: value,
            },
        });
    }
    if node.member("failure_id").is_some() {
        let value = read_thinkthen_complete_observation_failure_id(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_observation {
            kind: THINKTHEN_COMPLETE_OBSERVATION_FAILURE_ID,
            data: thinkthen_complete_observation_data { failure_id: value },
        });
    }
    Err(ErrorKind::Defect)
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_observation_id {
    pub value: thinkthen_complete_utf8,
}

pub(crate) fn read_thinkthen_complete_observation_id(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_observation_id, ErrorKind> {
    let value = read_text(node, store)?;
    Ok(thinkthen_complete_observation_id { value })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_observation_failure_id {
    pub failure_id: *const thinkthen_complete_failure_id,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_observation_failure_id(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_observation_failure_id, ErrorKind> {
    Ok(thinkthen_complete_observation_failure_id {
        failure_id: {
            let value = node.required("failure_id")?;
            read_thinkthen_complete_failure_id(value, store).map(|value| store.hold(value))?
        },
        extensions: store.extensions(node, &["failure_id"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_observation_observation_id {
    pub observation_id: *const thinkthen_complete_observation_id,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_observation_observation_id(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_observation_observation_id, ErrorKind> {
    Ok(thinkthen_complete_observation_observation_id {
        observation_id: {
            let value = node.required("observation_id")?;
            read_thinkthen_complete_observation_id(value, store).map(|value| store.hold(value))?
        },
        extensions: store.extensions(node, &["observation_id"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_origin {
    pub kind: u32,
}

pub const THINKTHEN_COMPLETE_ORIGIN_LIVE: u32 = 1;

pub const THINKTHEN_COMPLETE_ORIGIN_CACHE: u32 = 2;

pub const THINKTHEN_COMPLETE_ORIGIN_REPLAY: u32 = 3;

pub const THINKTHEN_COMPLETE_ORIGIN_PROXY: u32 = 4;

pub const THINKTHEN_COMPLETE_ORIGIN_MEMORY: u32 = 5;

pub(crate) fn read_thinkthen_complete_origin(
    node: &Node,
    _store: &mut Storage,
) -> Result<thinkthen_complete_origin, ErrorKind> {
    let kind = match node.text()? {
        "live" => THINKTHEN_COMPLETE_ORIGIN_LIVE,
        "cache" => THINKTHEN_COMPLETE_ORIGIN_CACHE,
        "replay" => THINKTHEN_COMPLETE_ORIGIN_REPLAY,
        "proxy" => THINKTHEN_COMPLETE_ORIGIN_PROXY,
        "memory" => THINKTHEN_COMPLETE_ORIGIN_MEMORY,
        _ => return Err(ErrorKind::Defect),
    };
    Ok(thinkthen_complete_origin { kind })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_persistence_observation {
    pub advice: thinkthen_complete_utf8_presence,
    pub observed_at: thinkthen_complete_utf8,
    pub state: *const thinkthen_complete_usage_persistence,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_persistence_observation(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_persistence_observation, ErrorKind> {
    Ok(thinkthen_complete_persistence_observation {
        advice: {
            let (presence, value) = read_presence(
                node.member("advice"),
                false,
                thinkthen_complete_utf8 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence { presence, value }
        },
        observed_at: {
            let value = node.required("observed_at")?;
            read_text(value, store)?
        },
        state: {
            let value = node.required("state")?;
            read_thinkthen_complete_usage_persistence(value, store)
                .map(|value| store.hold(value))?
        },
        extensions: store.extensions(node, &["advice", "observed_at", "state"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_physical_source {
    pub file: thinkthen_complete_utf8,
    pub first_line: thinkthen_complete_u64_presence,
    pub last_line: thinkthen_complete_u64_presence,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_physical_source(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_physical_source, ErrorKind> {
    Ok(thinkthen_complete_physical_source {
        file: {
            let value = node.required("file")?;
            read_text(value, store)?
        },
        first_line: {
            let (presence, value) =
                read_presence(node.member("first_line"), false, 0, read_number, store)?;
            thinkthen_complete_u64_presence { presence, value }
        },
        last_line: {
            let (presence, value) =
                read_presence(node.member("last_line"), false, 0, read_number, store)?;
            thinkthen_complete_u64_presence { presence, value }
        },
        extensions: store.extensions(node, &["file", "first_line", "last_line"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_position {
    pub file: thinkthen_complete_utf8_presence,
    pub first: thinkthen_complete_u64_presence,
    pub images: thinkthen_complete_string_list_presence,
    pub last: thinkthen_complete_u64_presence,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_position(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_position, ErrorKind> {
    Ok(thinkthen_complete_position {
        file: {
            let (presence, value) = read_presence(
                node.member("file"),
                true,
                thinkthen_complete_utf8 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence { presence, value }
        },
        first: {
            let (presence, value) =
                read_presence(node.member("first"), false, 0, read_number, store)?;
            thinkthen_complete_u64_presence { presence, value }
        },
        images: {
            let (presence, value) = read_presence(
                node.member("images"),
                false,
                thinkthen_complete_string_list {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_string_list,
                store,
            )?;
            thinkthen_complete_string_list_presence { presence, value }
        },
        last: {
            let (presence, value) =
                read_presence(node.member("last"), false, 0, read_number, store)?;
            thinkthen_complete_u64_presence { presence, value }
        },
        extensions: store.extensions(node, &["file", "first", "images", "last"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_question_name {
    pub value: thinkthen_complete_utf8,
}

pub(crate) fn read_thinkthen_complete_question_name(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_question_name, ErrorKind> {
    let value = read_text(node, store)?;
    Ok(thinkthen_complete_question_name { value })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_question_source {
    pub answered_by: thinkthen_complete_utf8,
    pub batch_size: thinkthen_complete_u64_presence,
    pub origin: *const thinkthen_complete_origin,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_question_source(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_question_source, ErrorKind> {
    Ok(thinkthen_complete_question_source {
        answered_by: {
            let value = node.required("answered_by")?;
            read_text(value, store)?
        },
        batch_size: {
            let (presence, value) =
                read_presence(node.member("batch_size"), false, 0, read_number, store)?;
            thinkthen_complete_u64_presence { presence, value }
        },
        origin: {
            let value = node.required("origin")?;
            read_thinkthen_complete_origin(value, store).map(|value| store.hold(value))?
        },
        extensions: store.extensions(node, &["answered_by", "batch_size", "origin"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_rank_member {
    pub name: thinkthen_complete_utf8,
    pub result: *const thinkthen_complete_rank_member_result,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_rank_member(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_rank_member, ErrorKind> {
    Ok(thinkthen_complete_rank_member {
        name: {
            let value = node.required("name")?;
            read_text(value, store)?
        },
        result: {
            let value = node.required("result")?;
            read_thinkthen_complete_rank_member_result(value, store)
                .map(|value| store.hold(value))?
        },
        extensions: store.extensions(node, &["name", "result"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_rank_member_result {
    pub answer: *const thinkthen_complete_answer,
    pub answer_id: *const thinkthen_complete_answer_id,
    pub images: thinkthen_complete_image_list_presence,
    pub meta: *const thinkthen_complete_meta,
    pub question: *const thinkthen_complete_question_document,
    pub schema: *const thinkthen_complete_result_tag,
    pub source: thinkthen_complete_physical_source_presence,
    pub threshold: thinkthen_complete_json_presence,
    pub value: u64,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_rank_member_result(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_rank_member_result, ErrorKind> {
    Ok(thinkthen_complete_rank_member_result {
        answer: {
            let value = node.required("answer")?;
            read_thinkthen_complete_answer(value, store).map(|value| store.hold(value))?
        },
        answer_id: {
            let value = node.required("answer_id")?;
            read_thinkthen_complete_answer_id(value, store).map(|value| store.hold(value))?
        },
        images: {
            let (presence, value) = read_presence(
                node.member("images"),
                false,
                thinkthen_complete_image_list {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_image_list,
                store,
            )?;
            thinkthen_complete_image_list_presence { presence, value }
        },
        meta: {
            let value = node.required("meta")?;
            read_thinkthen_complete_meta(value, store).map(|value| store.hold(value))?
        },
        question: {
            let value = node.required("question")?;
            read_thinkthen_complete_question_document(value, store)
                .map(|value| store.hold(value))?
        },
        schema: {
            let value = node.required("schema")?;
            read_thinkthen_complete_result_tag(value, store).map(|value| store.hold(value))?
        },
        source: {
            let (presence, value) = read_presence(
                node.member("source"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_physical_source(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_physical_source_presence { presence, value }
        },
        threshold: {
            let (presence, value) = read_presence(
                node.member("threshold"),
                true,
                std::ptr::null(),
                read_json,
                store,
            )?;
            thinkthen_complete_json_presence { presence, value }
        },
        value: {
            let value = node.required("value")?;
            read_number(value, store)?
        },
        extensions: store.extensions(
            node,
            &[
                "answer",
                "answer_id",
                "images",
                "meta",
                "question",
                "schema",
                "source",
                "threshold",
                "value",
            ],
        )?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_recognition {
    pub answer: *const thinkthen_complete_recognition_odds,
    pub answer_id: *const thinkthen_complete_answer_id,
    pub file: thinkthen_complete_utf8_presence,
    pub first_line: thinkthen_complete_u64_presence,
    pub index: thinkthen_complete_u64_presence,
    pub input: thinkthen_complete_json_presence,
    pub last_line: thinkthen_complete_u64_presence,
    pub meta: *const thinkthen_complete_meta,
    pub position: thinkthen_complete_position_presence,
    pub question: *const thinkthen_complete_recognize_question_document,
    pub schema: *const thinkthen_complete_result_tag,
    pub source: thinkthen_complete_physical_source_presence,
    pub value: *const thinkthen_complete_recognize,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_recognition(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_recognition, ErrorKind> {
    Ok(thinkthen_complete_recognition {
        answer: {
            let value = node.required("answer")?;
            read_thinkthen_complete_recognition_odds(value, store).map(|value| store.hold(value))?
        },
        answer_id: {
            let value = node.required("answer_id")?;
            read_thinkthen_complete_answer_id(value, store).map(|value| store.hold(value))?
        },
        file: {
            let (presence, value) = read_presence(
                node.member("file"),
                false,
                thinkthen_complete_utf8 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence { presence, value }
        },
        first_line: {
            let (presence, value) =
                read_presence(node.member("first_line"), false, 0, read_number, store)?;
            thinkthen_complete_u64_presence { presence, value }
        },
        index: {
            let (presence, value) =
                read_presence(node.member("index"), false, 0, read_number, store)?;
            thinkthen_complete_u64_presence { presence, value }
        },
        input: {
            let (presence, value) = read_presence(
                node.member("input"),
                false,
                std::ptr::null(),
                read_json,
                store,
            )?;
            thinkthen_complete_json_presence { presence, value }
        },
        last_line: {
            let (presence, value) =
                read_presence(node.member("last_line"), false, 0, read_number, store)?;
            thinkthen_complete_u64_presence { presence, value }
        },
        meta: {
            let value = node.required("meta")?;
            read_thinkthen_complete_meta(value, store).map(|value| store.hold(value))?
        },
        position: {
            let (presence, value) = read_presence(
                node.member("position"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_position(value, store).map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_position_presence { presence, value }
        },
        question: {
            let value = node.required("question")?;
            read_thinkthen_complete_recognize_question_document(value, store)
                .map(|value| store.hold(value))?
        },
        schema: {
            let value = node.required("schema")?;
            read_thinkthen_complete_result_tag(value, store).map(|value| store.hold(value))?
        },
        source: {
            let (presence, value) = read_presence(
                node.member("source"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_physical_source(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_physical_source_presence { presence, value }
        },
        value: {
            let value = node.required("value")?;
            read_thinkthen_complete_recognize(value, store).map(|value| store.hold(value))?
        },
        extensions: store.extensions(
            node,
            &[
                "answer",
                "answer_id",
                "file",
                "first_line",
                "index",
                "input",
                "last_line",
                "meta",
                "position",
                "question",
                "schema",
                "source",
                "value",
            ],
        )?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_recognition_edge {
    pub either: u32,
    pub probability: f64,
    pub relation: thinkthen_complete_utf8,
    pub source: *const thinkthen_complete_entity,
    pub target: *const thinkthen_complete_entity,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_recognition_edge(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_recognition_edge, ErrorKind> {
    Ok(thinkthen_complete_recognition_edge {
        either: {
            let value = node.required("either")?;
            read_boolean(value, store)?
        },
        probability: {
            let value = node.required("probability")?;
            read_number(value, store)?
        },
        relation: {
            let value = node.required("relation")?;
            read_text(value, store)?
        },
        source: {
            let value = node.required("source")?;
            read_thinkthen_complete_entity(value, store).map(|value| store.hold(value))?
        },
        target: {
            let value = node.required("target")?;
            read_thinkthen_complete_entity(value, store).map(|value| store.hold(value))?
        },
        extensions: store.extensions(
            node,
            &["either", "probability", "relation", "source", "target"],
        )?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_recognition_mode {
    pub kind: u32,
}

pub const THINKTHEN_COMPLETE_RECOGNITION_MODE_WHOLE: u32 = 1;

pub const THINKTHEN_COMPLETE_RECOGNITION_MODE_BOUNDARY_ONLY: u32 = 2;

pub(crate) fn read_thinkthen_complete_recognition_mode(
    node: &Node,
    _store: &mut Storage,
) -> Result<thinkthen_complete_recognition_mode, ErrorKind> {
    let kind = match node.text()? {
        "whole" => THINKTHEN_COMPLETE_RECOGNITION_MODE_WHOLE,
        "boundary_only" => THINKTHEN_COMPLETE_RECOGNITION_MODE_BOUNDARY_ONLY,
        _ => return Err(ErrorKind::Defect),
    };
    Ok(thinkthen_complete_recognition_mode { kind })
}

pub const THINKTHEN_COMPLETE_RECOGNITION_ODDS_RECOGNIZE_ANSWER: u32 = 1;

pub const THINKTHEN_COMPLETE_RECOGNITION_ODDS_BOUNDARY_ODDS: u32 = 2;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_recognition_odds_data {
    pub recognize_answer: *const thinkthen_complete_recognition_odds_recognize_answer,
    pub boundary_odds: *const thinkthen_complete_recognition_odds_boundary_odds,
}
impl std::fmt::Debug for thinkthen_complete_recognition_odds_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("union payload")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_recognition_odds {
    pub kind: u32,
    pub data: thinkthen_complete_recognition_odds_data,
}

pub(crate) fn read_thinkthen_complete_recognition_odds(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_recognition_odds, ErrorKind> {
    if node.member("names").is_some()
        && node.member("pairs").is_some()
        && node.member("pieces").is_some()
        && node.member("proposals").is_some()
    {
        let value = read_thinkthen_complete_recognition_odds_recognize_answer(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_recognition_odds {
            kind: THINKTHEN_COMPLETE_RECOGNITION_ODDS_RECOGNIZE_ANSWER,
            data: thinkthen_complete_recognition_odds_data {
                recognize_answer: value,
            },
        });
    }
    if node.member("pieces").is_some()
        && node.member("proposals").is_some()
        && node.member("names").is_none()
        && node.member("pairs").is_none()
    {
        let value = read_thinkthen_complete_recognition_odds_boundary_odds(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_recognition_odds {
            kind: THINKTHEN_COMPLETE_RECOGNITION_ODDS_BOUNDARY_ODDS,
            data: thinkthen_complete_recognition_odds_data {
                boundary_odds: value,
            },
        });
    }
    Err(ErrorKind::Defect)
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_piece_odds_list {
    pub data: *const *const thinkthen_complete_piece_odds,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_piece_odds_list(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_piece_odds_list, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values
            .push(read_thinkthen_complete_piece_odds(item, store).map(|value| store.hold(value))?);
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_piece_odds_list { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_boundary_proposal_list {
    pub data: *const *const thinkthen_complete_boundary_proposal,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_boundary_proposal_list(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_boundary_proposal_list, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_boundary_proposal(item, store)
                .map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_boundary_proposal_list { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_recognition_odds_boundary_odds {
    pub pieces: thinkthen_complete_piece_odds_list,
    pub proposals: thinkthen_complete_boundary_proposal_list,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_recognition_odds_boundary_odds(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_recognition_odds_boundary_odds, ErrorKind> {
    Ok(thinkthen_complete_recognition_odds_boundary_odds {
        pieces: {
            let value = node.required("pieces")?;
            read_thinkthen_complete_piece_odds_list(value, store)?
        },
        proposals: {
            let value = node.required("proposals")?;
            read_thinkthen_complete_boundary_proposal_list(value, store)?
        },
        extensions: store.extensions(node, &["pieces", "proposals"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_name_odds_list {
    pub data: *const *const thinkthen_complete_name_odds,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_name_odds_list(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_name_odds_list, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(read_thinkthen_complete_name_odds(item, store).map(|value| store.hold(value))?);
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_name_odds_list { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_pair_odds_list {
    pub data: *const *const thinkthen_complete_pair_odds,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_pair_odds_list(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_pair_odds_list, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(read_thinkthen_complete_pair_odds(item, store).map(|value| store.hold(value))?);
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_pair_odds_list { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_recognition_proposal_list {
    pub data: *const *const thinkthen_complete_recognition_proposal,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_recognition_proposal_list(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_recognition_proposal_list, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_recognition_proposal(item, store)
                .map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_recognition_proposal_list { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_recognition_odds_recognize_answer {
    pub names: thinkthen_complete_name_odds_list,
    pub pairs: thinkthen_complete_pair_odds_list,
    pub pieces: thinkthen_complete_piece_odds_list,
    pub proposals: thinkthen_complete_recognition_proposal_list,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_recognition_odds_recognize_answer(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_recognition_odds_recognize_answer, ErrorKind> {
    Ok(thinkthen_complete_recognition_odds_recognize_answer {
        names: {
            let value = node.required("names")?;
            read_thinkthen_complete_name_odds_list(value, store)?
        },
        pairs: {
            let value = node.required("pairs")?;
            read_thinkthen_complete_pair_odds_list(value, store)?
        },
        pieces: {
            let value = node.required("pieces")?;
            read_thinkthen_complete_piece_odds_list(value, store)?
        },
        proposals: {
            let value = node.required("proposals")?;
            read_thinkthen_complete_recognition_proposal_list(value, store)?
        },
        extensions: store.extensions(node, &["names", "pairs", "pieces", "proposals"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_place_presence {
    pub presence: u32,
    pub value: *const thinkthen_complete_place,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_f64_presence {
    pub presence: u32,
    pub value: f64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_recognition_proposal {
    pub end: u64,
    pub kept: u32,
    pub kind: thinkthen_complete_utf8_presence,
    pub selected: thinkthen_complete_place_presence,
    pub span_probability: f64,
    pub start: u64,
    pub strength: thinkthen_complete_f64_presence,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_recognition_proposal(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_recognition_proposal, ErrorKind> {
    Ok(thinkthen_complete_recognition_proposal {
        end: {
            let value = node.required("end")?;
            read_number(value, store)?
        },
        kept: {
            let value = node.required("kept")?;
            read_boolean(value, store)?
        },
        kind: {
            let (presence, value) = read_presence(
                node.member("kind"),
                false,
                thinkthen_complete_utf8 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence { presence, value }
        },
        selected: {
            let (presence, value) = read_presence(
                node.member("selected"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_place(value, store).map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_place_presence { presence, value }
        },
        span_probability: {
            let value = node.required("span_probability")?;
            read_number(value, store)?
        },
        start: {
            let value = node.required("start")?;
            read_number(value, store)?
        },
        strength: {
            let (presence, value) =
                read_presence(node.member("strength"), false, 0.0, read_number, store)?;
            thinkthen_complete_f64_presence { presence, value }
        },
        extensions: store.extensions(
            node,
            &[
                "end",
                "kept",
                "kind",
                "selected",
                "span_probability",
                "start",
                "strength",
            ],
        )?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_recognition_stage_context {
    pub boundary: thinkthen_complete_utf8_presence,
    pub kind_edge: thinkthen_complete_utf8_presence,
    pub relation: thinkthen_complete_utf8_presence,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_recognition_stage_context(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_recognition_stage_context, ErrorKind> {
    Ok(thinkthen_complete_recognition_stage_context {
        boundary: {
            let (presence, value) = read_presence(
                node.member("boundary"),
                false,
                thinkthen_complete_utf8 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence { presence, value }
        },
        kind_edge: {
            let (presence, value) = read_presence(
                node.member("kind_edge"),
                false,
                thinkthen_complete_utf8 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence { presence, value }
        },
        relation: {
            let (presence, value) = read_presence(
                node.member("relation"),
                false,
                thinkthen_complete_utf8 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence { presence, value }
        },
        extensions: store.extensions(node, &["boundary", "kind_edge", "relation"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_input_source_list {
    pub data: *const *const thinkthen_complete_session_input_source,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_session_input_source_list(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_input_source_list, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_session_input_source(item, store)
                .map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_session_input_source_list { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_input_source_list_presence {
    pub presence: u32,
    pub value: thinkthen_complete_session_input_source_list,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_related_entity_edge_list {
    pub data: *const *const thinkthen_complete_related_entity_edge,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_related_entity_edge_list(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_related_entity_edge_list, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_related_entity_edge(item, store)
                .map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_related_entity_edge_list { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_relation {
    pub answer: *const thinkthen_complete_answers,
    pub answer_id: *const thinkthen_complete_answer_id,
    pub file: thinkthen_complete_utf8_presence,
    pub first_line: thinkthen_complete_u64_presence,
    pub index: thinkthen_complete_u64_presence,
    pub input: thinkthen_complete_json_presence,
    pub input_sources: thinkthen_complete_session_input_source_list_presence,
    pub last_line: thinkthen_complete_u64_presence,
    pub meta: *const thinkthen_complete_meta,
    pub position: thinkthen_complete_position_presence,
    pub question: *const thinkthen_complete_relate_question_document,
    pub schema: *const thinkthen_complete_result_tag,
    pub value: thinkthen_complete_related_entity_edge_list,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_relation(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_relation, ErrorKind> {
    Ok(thinkthen_complete_relation {
        answer: {
            let value = node.required("answer")?;
            read_thinkthen_complete_answers(value, store).map(|value| store.hold(value))?
        },
        answer_id: {
            let value = node.required("answer_id")?;
            read_thinkthen_complete_answer_id(value, store).map(|value| store.hold(value))?
        },
        file: {
            let (presence, value) = read_presence(
                node.member("file"),
                false,
                thinkthen_complete_utf8 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence { presence, value }
        },
        first_line: {
            let (presence, value) =
                read_presence(node.member("first_line"), false, 0, read_number, store)?;
            thinkthen_complete_u64_presence { presence, value }
        },
        index: {
            let (presence, value) =
                read_presence(node.member("index"), false, 0, read_number, store)?;
            thinkthen_complete_u64_presence { presence, value }
        },
        input: {
            let (presence, value) = read_presence(
                node.member("input"),
                false,
                std::ptr::null(),
                read_json,
                store,
            )?;
            thinkthen_complete_json_presence { presence, value }
        },
        input_sources: {
            let (presence, value) = read_presence(
                node.member("input_sources"),
                false,
                thinkthen_complete_session_input_source_list {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_session_input_source_list,
                store,
            )?;
            thinkthen_complete_session_input_source_list_presence { presence, value }
        },
        last_line: {
            let (presence, value) =
                read_presence(node.member("last_line"), false, 0, read_number, store)?;
            thinkthen_complete_u64_presence { presence, value }
        },
        meta: {
            let value = node.required("meta")?;
            read_thinkthen_complete_meta(value, store).map(|value| store.hold(value))?
        },
        position: {
            let (presence, value) = read_presence(
                node.member("position"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_position(value, store).map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_position_presence { presence, value }
        },
        question: {
            let value = node.required("question")?;
            read_thinkthen_complete_relate_question_document(value, store)
                .map(|value| store.hold(value))?
        },
        schema: {
            let value = node.required("schema")?;
            read_thinkthen_complete_result_tag(value, store).map(|value| store.hold(value))?
        },
        value: {
            let value = node.required("value")?;
            read_thinkthen_complete_related_entity_edge_list(value, store)?
        },
        extensions: store.extensions(
            node,
            &[
                "answer",
                "answer_id",
                "file",
                "first_line",
                "index",
                "input",
                "input_sources",
                "last_line",
                "meta",
                "position",
                "question",
                "schema",
                "value",
            ],
        )?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_relation_direction {
    pub kind: u32,
}

pub const THINKTHEN_COMPLETE_RELATION_DIRECTION_SOURCE_TO_TARGET: u32 = 1;

pub const THINKTHEN_COMPLETE_RELATION_DIRECTION_EITHER: u32 = 2;

pub(crate) fn read_thinkthen_complete_relation_direction(
    node: &Node,
    _store: &mut Storage,
) -> Result<thinkthen_complete_relation_direction, ErrorKind> {
    let kind = match node.text()? {
        "source_to_target" => THINKTHEN_COMPLETE_RELATION_DIRECTION_SOURCE_TO_TARGET,
        "either" => THINKTHEN_COMPLETE_RELATION_DIRECTION_EITHER,
        _ => return Err(ErrorKind::Defect),
    };
    Ok(thinkthen_complete_relation_direction { kind })
}

pub const THINKTHEN_COMPLETE_RELATION_ENDPOINT_RELATED_ENTITY: u32 = 1;

pub const THINKTHEN_COMPLETE_RELATION_ENDPOINT_SOURCE: u32 = 2;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_relation_endpoint_data {
    pub related_entity: *const thinkthen_complete_relation_endpoint_related_entity,
    pub source: *const thinkthen_complete_relation_endpoint_source,
}
impl std::fmt::Debug for thinkthen_complete_relation_endpoint_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("union payload")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_relation_endpoint {
    pub kind: u32,
    pub data: thinkthen_complete_relation_endpoint_data,
}

pub(crate) fn read_thinkthen_complete_relation_endpoint(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_relation_endpoint, ErrorKind> {
    if node.member("kind").is_some()
        && node.member("name").is_some()
        && node.member("file").is_none()
        && node.member("ordinal").is_none()
        && node.member("record").is_none()
    {
        let value = read_thinkthen_complete_relation_endpoint_related_entity(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_relation_endpoint {
            kind: THINKTHEN_COMPLETE_RELATION_ENDPOINT_RELATED_ENTITY,
            data: thinkthen_complete_relation_endpoint_data {
                related_entity: value,
            },
        });
    }
    if node.member("file").is_some()
        && node.member("kind").is_some()
        && node.member("name").is_some()
        && node.member("ordinal").is_some()
        && node.member("record").is_some()
    {
        let value = read_thinkthen_complete_relation_endpoint_source(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_relation_endpoint {
            kind: THINKTHEN_COMPLETE_RELATION_ENDPOINT_SOURCE,
            data: thinkthen_complete_relation_endpoint_data { source: value },
        });
    }
    Err(ErrorKind::Defect)
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_relation_endpoint_related_entity {
    pub kind: thinkthen_complete_utf8,
    pub name: thinkthen_complete_utf8,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_relation_endpoint_related_entity(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_relation_endpoint_related_entity, ErrorKind> {
    Ok(thinkthen_complete_relation_endpoint_related_entity {
        kind: {
            let value = node.required("kind")?;
            read_text(value, store)?
        },
        name: {
            let value = node.required("name")?;
            read_text(value, store)?
        },
        extensions: store.extensions(node, &["kind", "name"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_relation_endpoint_source {
    pub file: thinkthen_complete_utf8_presence,
    pub first_line: thinkthen_complete_u64_presence,
    pub kind: thinkthen_complete_utf8,
    pub last_line: thinkthen_complete_u64_presence,
    pub name: thinkthen_complete_utf8,
    pub ordinal: u64,
    pub record: thinkthen_complete_json_presence,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_relation_endpoint_source(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_relation_endpoint_source, ErrorKind> {
    Ok(thinkthen_complete_relation_endpoint_source {
        file: {
            let (presence, value) = read_presence(
                node.member("file"),
                true,
                thinkthen_complete_utf8 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence { presence, value }
        },
        first_line: {
            let (presence, value) =
                read_presence(node.member("first_line"), false, 0, read_number, store)?;
            thinkthen_complete_u64_presence { presence, value }
        },
        kind: {
            let value = node.required("kind")?;
            read_text(value, store)?
        },
        last_line: {
            let (presence, value) =
                read_presence(node.member("last_line"), false, 0, read_number, store)?;
            thinkthen_complete_u64_presence { presence, value }
        },
        name: {
            let value = node.required("name")?;
            read_text(value, store)?
        },
        ordinal: {
            let value = node.required("ordinal")?;
            read_number(value, store)?
        },
        record: {
            let (presence, value) = read_presence(
                node.member("record"),
                true,
                std::ptr::null(),
                read_json,
                store,
            )?;
            thinkthen_complete_json_presence { presence, value }
        },
        extensions: store.extensions(
            node,
            &[
                "file",
                "first_line",
                "kind",
                "last_line",
                "name",
                "ordinal",
                "record",
            ],
        )?,
    })
}

pub const THINKTHEN_COMPLETE_RELATION_MEMBER_ANSWER_ID: u32 = 1;

pub const THINKTHEN_COMPLETE_RELATION_MEMBER_FAILURE_ID: u32 = 2;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_relation_member_data {
    pub answer_id: *const thinkthen_complete_relation_member_answer_id,
    pub failure_id: *const thinkthen_complete_relation_member_failure_id,
}
impl std::fmt::Debug for thinkthen_complete_relation_member_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("union payload")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_relation_member {
    pub kind: u32,
    pub data: thinkthen_complete_relation_member_data,
}

pub(crate) fn read_thinkthen_complete_relation_member(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_relation_member, ErrorKind> {
    if node.member("answer_id").is_some() {
        let value = read_thinkthen_complete_relation_member_answer_id(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_relation_member {
            kind: THINKTHEN_COMPLETE_RELATION_MEMBER_ANSWER_ID,
            data: thinkthen_complete_relation_member_data { answer_id: value },
        });
    }
    if node.member("failure_id").is_some() {
        let value = read_thinkthen_complete_relation_member_failure_id(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_relation_member {
            kind: THINKTHEN_COMPLETE_RELATION_MEMBER_FAILURE_ID,
            data: thinkthen_complete_relation_member_data { failure_id: value },
        });
    }
    Err(ErrorKind::Defect)
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_related_entity_presence {
    pub presence: u32,
    pub value: *const thinkthen_complete_related_entity,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_relation_member_answer_id {
    pub direction: *const thinkthen_complete_relation_direction,
    pub method: *const thinkthen_complete_relation_method,
    pub observations: thinkthen_complete_observation_list,
    pub question: *const thinkthen_complete_question_document,
    pub question_sources: thinkthen_complete_question_source_list,
    pub reads: thinkthen_complete_utf8,
    pub relation: thinkthen_complete_utf8,
    pub request: thinkthen_complete_utf8,
    pub source: *const thinkthen_complete_related_entity,
    pub target: thinkthen_complete_related_entity_presence,
    pub threshold: *const thinkthen_complete_threshold,
    pub usage: thinkthen_complete_usage_presence,
    pub accepted: u32,
    pub answer: *const thinkthen_complete_answer,
    pub answer_id: *const thinkthen_complete_answer_id,
    pub probability: f64,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_relation_member_answer_id(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_relation_member_answer_id, ErrorKind> {
    Ok(thinkthen_complete_relation_member_answer_id {
        direction: {
            let value = node.required("direction")?;
            read_thinkthen_complete_relation_direction(value, store)
                .map(|value| store.hold(value))?
        },
        method: {
            let value = node.required("method")?;
            read_thinkthen_complete_relation_method(value, store).map(|value| store.hold(value))?
        },
        observations: {
            let value = node.required("observations")?;
            read_thinkthen_complete_observation_list(value, store)?
        },
        question: {
            let value = node.required("question")?;
            read_thinkthen_complete_question_document(value, store)
                .map(|value| store.hold(value))?
        },
        question_sources: {
            let value = node.required("question_sources")?;
            read_thinkthen_complete_question_source_list(value, store)?
        },
        reads: {
            let value = node.required("reads")?;
            read_text(value, store)?
        },
        relation: {
            let value = node.required("relation")?;
            read_text(value, store)?
        },
        request: {
            let value = node.required("request")?;
            read_text(value, store)?
        },
        source: {
            let value = node.required("source")?;
            read_thinkthen_complete_related_entity(value, store).map(|value| store.hold(value))?
        },
        target: {
            let (presence, value) = read_presence(
                node.member("target"),
                true,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_related_entity(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_related_entity_presence { presence, value }
        },
        threshold: {
            let value = node.required("threshold")?;
            read_thinkthen_complete_threshold(value, store).map(|value| store.hold(value))?
        },
        usage: {
            let (presence, value) = read_presence(
                node.member("usage"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_usage(value, store).map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_usage_presence { presence, value }
        },
        accepted: {
            let value = node.required("accepted")?;
            read_boolean(value, store)?
        },
        answer: {
            let value = node.required("answer")?;
            read_thinkthen_complete_answer(value, store).map(|value| store.hold(value))?
        },
        answer_id: {
            let value = node.required("answer_id")?;
            read_thinkthen_complete_answer_id(value, store).map(|value| store.hold(value))?
        },
        probability: {
            let value = node.required("probability")?;
            read_number(value, store)?
        },
        extensions: store.extensions(
            node,
            &[
                "direction",
                "method",
                "observations",
                "question",
                "question_sources",
                "reads",
                "relation",
                "request",
                "source",
                "target",
                "threshold",
                "usage",
                "accepted",
                "answer",
                "answer_id",
                "probability",
            ],
        )?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_relation_member_failure_id {
    pub direction: *const thinkthen_complete_relation_direction,
    pub method: *const thinkthen_complete_relation_method,
    pub observations: thinkthen_complete_observation_list,
    pub question: *const thinkthen_complete_question_document,
    pub question_sources: thinkthen_complete_question_source_list,
    pub reads: thinkthen_complete_utf8,
    pub relation: thinkthen_complete_utf8,
    pub request: thinkthen_complete_utf8,
    pub source: *const thinkthen_complete_related_entity,
    pub target: thinkthen_complete_related_entity_presence,
    pub threshold: *const thinkthen_complete_threshold,
    pub usage: thinkthen_complete_usage_presence,
    pub failure: *const thinkthen_complete_failure,
    pub failure_id: *const thinkthen_complete_failure_id,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_relation_member_failure_id(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_relation_member_failure_id, ErrorKind> {
    Ok(thinkthen_complete_relation_member_failure_id {
        direction: {
            let value = node.required("direction")?;
            read_thinkthen_complete_relation_direction(value, store)
                .map(|value| store.hold(value))?
        },
        method: {
            let value = node.required("method")?;
            read_thinkthen_complete_relation_method(value, store).map(|value| store.hold(value))?
        },
        observations: {
            let value = node.required("observations")?;
            read_thinkthen_complete_observation_list(value, store)?
        },
        question: {
            let value = node.required("question")?;
            read_thinkthen_complete_question_document(value, store)
                .map(|value| store.hold(value))?
        },
        question_sources: {
            let value = node.required("question_sources")?;
            read_thinkthen_complete_question_source_list(value, store)?
        },
        reads: {
            let value = node.required("reads")?;
            read_text(value, store)?
        },
        relation: {
            let value = node.required("relation")?;
            read_text(value, store)?
        },
        request: {
            let value = node.required("request")?;
            read_text(value, store)?
        },
        source: {
            let value = node.required("source")?;
            read_thinkthen_complete_related_entity(value, store).map(|value| store.hold(value))?
        },
        target: {
            let (presence, value) = read_presence(
                node.member("target"),
                true,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_related_entity(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_related_entity_presence { presence, value }
        },
        threshold: {
            let value = node.required("threshold")?;
            read_thinkthen_complete_threshold(value, store).map(|value| store.hold(value))?
        },
        usage: {
            let (presence, value) = read_presence(
                node.member("usage"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_usage(value, store).map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_usage_presence { presence, value }
        },
        failure: {
            let value = node.required("failure")?;
            read_thinkthen_complete_failure(value, store).map(|value| store.hold(value))?
        },
        failure_id: {
            let value = node.required("failure_id")?;
            read_thinkthen_complete_failure_id(value, store).map(|value| store.hold(value))?
        },
        extensions: store.extensions(
            node,
            &[
                "direction",
                "method",
                "observations",
                "question",
                "question_sources",
                "reads",
                "relation",
                "request",
                "source",
                "target",
                "threshold",
                "usage",
                "failure",
                "failure_id",
            ],
        )?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_relation_method {
    pub kind: u32,
}

pub const THINKTHEN_COMPLETE_RELATION_METHOD_YES_NO: u32 = 1;

pub const THINKTHEN_COMPLETE_RELATION_METHOD_CHOICE: u32 = 2;

pub(crate) fn read_thinkthen_complete_relation_method(
    node: &Node,
    _store: &mut Storage,
) -> Result<thinkthen_complete_relation_method, ErrorKind> {
    let kind = match node.text()? {
        "yes_no" => THINKTHEN_COMPLETE_RELATION_METHOD_YES_NO,
        "choice" => THINKTHEN_COMPLETE_RELATION_METHOD_CHOICE,
        _ => return Err(ErrorKind::Defect),
    };
    Ok(thinkthen_complete_relation_method { kind })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_request_function {
    pub kind: u32,
}

pub const THINKTHEN_COMPLETE_REQUEST_FUNCTION_DECIDE: u32 = 1;

pub const THINKTHEN_COMPLETE_REQUEST_FUNCTION_CHOOSE: u32 = 2;

pub const THINKTHEN_COMPLETE_REQUEST_FUNCTION_TAG: u32 = 3;

pub const THINKTHEN_COMPLETE_REQUEST_FUNCTION_SCORE: u32 = 4;

pub const THINKTHEN_COMPLETE_REQUEST_FUNCTION_FILTER: u32 = 5;

pub const THINKTHEN_COMPLETE_REQUEST_FUNCTION_RANK: u32 = 6;

pub const THINKTHEN_COMPLETE_REQUEST_FUNCTION_FIND: u32 = 7;

pub const THINKTHEN_COMPLETE_REQUEST_FUNCTION_ANNOTATE: u32 = 8;

pub const THINKTHEN_COMPLETE_REQUEST_FUNCTION_RECOGNIZE: u32 = 9;

pub const THINKTHEN_COMPLETE_REQUEST_FUNCTION_RELATE: u32 = 10;

pub(crate) fn read_thinkthen_complete_request_function(
    node: &Node,
    _store: &mut Storage,
) -> Result<thinkthen_complete_request_function, ErrorKind> {
    let kind = match node.text()? {
        "decide" => THINKTHEN_COMPLETE_REQUEST_FUNCTION_DECIDE,
        "choose" => THINKTHEN_COMPLETE_REQUEST_FUNCTION_CHOOSE,
        "tag" => THINKTHEN_COMPLETE_REQUEST_FUNCTION_TAG,
        "score" => THINKTHEN_COMPLETE_REQUEST_FUNCTION_SCORE,
        "filter" => THINKTHEN_COMPLETE_REQUEST_FUNCTION_FILTER,
        "rank" => THINKTHEN_COMPLETE_REQUEST_FUNCTION_RANK,
        "find" => THINKTHEN_COMPLETE_REQUEST_FUNCTION_FIND,
        "annotate" => THINKTHEN_COMPLETE_REQUEST_FUNCTION_ANNOTATE,
        "recognize" => THINKTHEN_COMPLETE_REQUEST_FUNCTION_RECOGNIZE,
        "relate" => THINKTHEN_COMPLETE_REQUEST_FUNCTION_RELATE,
        _ => return Err(ErrorKind::Defect),
    };
    Ok(thinkthen_complete_request_function { kind })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_result_tag {
    pub kind: u32,
}

pub const THINKTHEN_COMPLETE_RESULT_TAG_THINKTHEN_RESULT: u32 = 1;

pub(crate) fn read_thinkthen_complete_result_tag(
    node: &Node,
    _store: &mut Storage,
) -> Result<thinkthen_complete_result_tag, ErrorKind> {
    let kind = match node.text()? {
        "thinkthen.result" => THINKTHEN_COMPLETE_RESULT_TAG_THINKTHEN_RESULT,
        _ => return Err(ErrorKind::Defect),
    };
    Ok(thinkthen_complete_result_tag { kind })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_sdk_request_id {
    pub value: thinkthen_complete_utf8,
}

pub(crate) fn read_thinkthen_complete_sdk_request_id(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_sdk_request_id, ErrorKind> {
    let value = read_text(node, store)?;
    Ok(thinkthen_complete_sdk_request_id { value })
}

pub const THINKTHEN_COMPLETE_SEND_DENIAL_BEFORE_FIRST_SEND: u32 = 1;

pub const THINKTHEN_COMPLETE_SEND_DENIAL_BEFORE_ADDITIONAL_SEND: u32 = 2;

pub const THINKTHEN_COMPLETE_SEND_DENIAL_BEFORE_RETRY: u32 = 3;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_send_denial_data {
    pub before_first_send: *const thinkthen_complete_send_denial_before_first_send,
    pub before_additional_send: *const thinkthen_complete_send_denial_before_additional_send,
    pub before_retry: *const thinkthen_complete_send_denial_before_retry,
}
impl std::fmt::Debug for thinkthen_complete_send_denial_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("union payload")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_send_denial {
    pub kind: u32,
    pub data: thinkthen_complete_send_denial_data,
}

pub(crate) fn read_thinkthen_complete_send_denial(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_send_denial, ErrorKind> {
    if node.is_member("kind", "before_first_send") {
        let value = read_thinkthen_complete_send_denial_before_first_send(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_send_denial {
            kind: THINKTHEN_COMPLETE_SEND_DENIAL_BEFORE_FIRST_SEND,
            data: thinkthen_complete_send_denial_data {
                before_first_send: value,
            },
        });
    }
    if node.is_member("kind", "before_additional_send") {
        let value = read_thinkthen_complete_send_denial_before_additional_send(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_send_denial {
            kind: THINKTHEN_COMPLETE_SEND_DENIAL_BEFORE_ADDITIONAL_SEND,
            data: thinkthen_complete_send_denial_data {
                before_additional_send: value,
            },
        });
    }
    if node.is_member("kind", "before_retry") {
        let value = read_thinkthen_complete_send_denial_before_retry(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_send_denial {
            kind: THINKTHEN_COMPLETE_SEND_DENIAL_BEFORE_RETRY,
            data: thinkthen_complete_send_denial_data {
                before_retry: value,
            },
        });
    }
    Err(ErrorKind::Defect)
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_send_denial_before_additional_send {
    pub kind: thinkthen_complete_utf8,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_send_denial_before_additional_send(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_send_denial_before_additional_send, ErrorKind> {
    Ok(thinkthen_complete_send_denial_before_additional_send {
        kind: {
            let value = node.required("kind")?;
            read_text(value, store)?
        },
        extensions: store.extensions(node, &["kind"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_send_denial_before_first_send {
    pub kind: thinkthen_complete_utf8,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_send_denial_before_first_send(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_send_denial_before_first_send, ErrorKind> {
    Ok(thinkthen_complete_send_denial_before_first_send {
        kind: {
            let value = node.required("kind")?;
            read_text(value, store)?
        },
        extensions: store.extensions(node, &["kind"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_send_denial_before_retry {
    pub kind: thinkthen_complete_utf8,
    pub last_status: u64,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_send_denial_before_retry(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_send_denial_before_retry, ErrorKind> {
    Ok(thinkthen_complete_send_denial_before_retry {
        kind: {
            let value = node.required("kind")?;
            read_text(value, store)?
        },
        last_status: {
            let value = node.required("last_status")?;
            read_number(value, store)?
        },
        extensions: store.extensions(node, &["kind", "last_status"])?,
    })
}

pub const THINKTHEN_COMPLETE_SETTLED_JUDGMENT_BOOLEAN: u32 = 1;

pub const THINKTHEN_COMPLETE_SETTLED_JUDGMENT_NULL: u32 = 2;

pub const THINKTHEN_COMPLETE_SETTLED_JUDGMENT_STRING: u32 = 3;

pub const THINKTHEN_COMPLETE_SETTLED_JUDGMENT_NUMBER: u32 = 4;

pub const THINKTHEN_COMPLETE_SETTLED_JUDGMENT_ARRAY: u32 = 5;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_settled_judgment_data {
    pub boolean: u32,
    pub null: u32,
    pub string: thinkthen_complete_utf8,
    pub number: f64,
    pub array: thinkthen_complete_string_list,
}
impl std::fmt::Debug for thinkthen_complete_settled_judgment_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("union payload")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_settled_judgment {
    pub kind: u32,
    pub data: thinkthen_complete_settled_judgment_data,
}

pub(crate) fn read_thinkthen_complete_settled_judgment(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_settled_judgment, ErrorKind> {
    if node.kind() == "boolean" {
        let value = read_boolean(node, store)?;
        return Ok(thinkthen_complete_settled_judgment {
            kind: THINKTHEN_COMPLETE_SETTLED_JUDGMENT_BOOLEAN,
            data: thinkthen_complete_settled_judgment_data { boolean: value },
        });
    }
    if node.kind() == "null" {
        let value = 0;
        return Ok(thinkthen_complete_settled_judgment {
            kind: THINKTHEN_COMPLETE_SETTLED_JUDGMENT_NULL,
            data: thinkthen_complete_settled_judgment_data { null: value },
        });
    }
    if node.kind() == "string" {
        let value = read_text(node, store)?;
        return Ok(thinkthen_complete_settled_judgment {
            kind: THINKTHEN_COMPLETE_SETTLED_JUDGMENT_STRING,
            data: thinkthen_complete_settled_judgment_data { string: value },
        });
    }
    if node.kind() == "number" {
        let value = read_number(node, store)?;
        return Ok(thinkthen_complete_settled_judgment {
            kind: THINKTHEN_COMPLETE_SETTLED_JUDGMENT_NUMBER,
            data: thinkthen_complete_settled_judgment_data { number: value },
        });
    }
    if node
        .array()
        .is_ok_and(|items| items.iter().all(matches_union_item_11))
    {
        let value = read_thinkthen_complete_string_list(node, store)?;
        return Ok(thinkthen_complete_settled_judgment {
            kind: THINKTHEN_COMPLETE_SETTLED_JUDGMENT_ARRAY,
            data: thinkthen_complete_settled_judgment_data { array: value },
        });
    }
    Err(ErrorKind::Defect)
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_settled_original {
    pub value: *const thinkthen_complete_json,
}

pub(crate) fn read_thinkthen_complete_settled_original(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_settled_original, ErrorKind> {
    let value = read_json(node, store)?;
    Ok(thinkthen_complete_settled_original { value })
}

pub const THINKTHEN_COMPLETE_SETTLED_RECOGNITION_ENTITIES: u32 = 1;

pub const THINKTHEN_COMPLETE_SETTLED_RECOGNITION_MODE_PROPOSALS: u32 = 2;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_settled_recognition_data {
    pub entities: *const thinkthen_complete_settled_recognition_entities,
    pub mode_proposals: *const thinkthen_complete_settled_recognition_mode_proposals,
}
impl std::fmt::Debug for thinkthen_complete_settled_recognition_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("union payload")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_settled_recognition {
    pub kind: u32,
    pub data: thinkthen_complete_settled_recognition_data,
}

pub(crate) fn read_thinkthen_complete_settled_recognition(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_settled_recognition, ErrorKind> {
    if node.member("entities").is_some()
        && node.member("mode").is_none()
        && node.member("proposals").is_none()
    {
        let value = read_thinkthen_complete_settled_recognition_entities(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_settled_recognition {
            kind: THINKTHEN_COMPLETE_SETTLED_RECOGNITION_ENTITIES,
            data: thinkthen_complete_settled_recognition_data { entities: value },
        });
    }
    if node.member("mode").is_some()
        && node.member("proposals").is_some()
        && node.member("entities").is_none()
    {
        let value = read_thinkthen_complete_settled_recognition_mode_proposals(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_settled_recognition {
            kind: THINKTHEN_COMPLETE_SETTLED_RECOGNITION_MODE_PROPOSALS,
            data: thinkthen_complete_settled_recognition_data {
                mode_proposals: value,
            },
        });
    }
    Err(ErrorKind::Defect)
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_entity_list {
    pub data: *const *const thinkthen_complete_entity,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_entity_list(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_entity_list, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(read_thinkthen_complete_entity(item, store).map(|value| store.hold(value))?);
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_entity_list { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_entity_edge_list {
    pub data: *const *const thinkthen_complete_entity_edge,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_entity_edge_list(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_entity_edge_list, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values
            .push(read_thinkthen_complete_entity_edge(item, store).map(|value| store.hold(value))?);
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_entity_edge_list { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_entity_edge_list_presence {
    pub presence: u32,
    pub value: thinkthen_complete_entity_edge_list,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_settled_recognition_entities {
    pub entities: thinkthen_complete_entity_list,
    pub relations: thinkthen_complete_entity_edge_list_presence,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_settled_recognition_entities(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_settled_recognition_entities, ErrorKind> {
    Ok(thinkthen_complete_settled_recognition_entities {
        entities: {
            let value = node.required("entities")?;
            read_thinkthen_complete_entity_list(value, store)?
        },
        relations: {
            let (presence, value) = read_presence(
                node.member("relations"),
                false,
                thinkthen_complete_entity_edge_list {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_entity_edge_list,
                store,
            )?;
            thinkthen_complete_entity_edge_list_presence { presence, value }
        },
        extensions: store.extensions(node, &["entities", "relations"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_settled_recognition_mode_proposals {
    pub mode: *const thinkthen_complete_boundary_mode,
    pub proposals: thinkthen_complete_boundary_proposal_list,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_settled_recognition_mode_proposals(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_settled_recognition_mode_proposals, ErrorKind> {
    Ok(thinkthen_complete_settled_recognition_mode_proposals {
        mode: {
            let value = node.required("mode")?;
            read_thinkthen_complete_boundary_mode(value, store).map(|value| store.hold(value))?
        },
        proposals: {
            let value = node.required("proposals")?;
            read_thinkthen_complete_boundary_proposal_list(value, store)?
        },
        extensions: store.extensions(node, &["mode", "proposals"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_settled_relations {
    pub value: thinkthen_complete_related_entity_edge_list,
}

pub(crate) fn read_thinkthen_complete_settled_relations(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_settled_relations, ErrorKind> {
    let value = read_thinkthen_complete_related_entity_edge_list(node, store)?;
    Ok(thinkthen_complete_settled_relations { value })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_stop_cause {
    pub kind: u32,
}

pub const THINKTHEN_COMPLETE_STOP_CAUSE_USAGE: u32 = 1;

pub const THINKTHEN_COMPLETE_STOP_CAUSE_LOCAL: u32 = 2;

pub const THINKTHEN_COMPLETE_STOP_CAUSE_NO_KEY: u32 = 3;

pub const THINKTHEN_COMPLETE_STOP_CAUSE_TRANSPORT: u32 = 4;

pub const THINKTHEN_COMPLETE_STOP_CAUSE_STATUS: u32 = 5;

pub const THINKTHEN_COMPLETE_STOP_CAUSE_TOO_LARGE: u32 = 6;

pub const THINKTHEN_COMPLETE_STOP_CAUSE_REPLY: u32 = 7;

pub const THINKTHEN_COMPLETE_STOP_CAUSE_BACKEND: u32 = 8;

pub const THINKTHEN_COMPLETE_STOP_CAUSE_CANCELLED: u32 = 9;

pub const THINKTHEN_COMPLETE_STOP_CAUSE_DEADLINE: u32 = 10;

pub const THINKTHEN_COMPLETE_STOP_CAUSE_DEFECT: u32 = 11;

pub(crate) fn read_thinkthen_complete_stop_cause(
    node: &Node,
    _store: &mut Storage,
) -> Result<thinkthen_complete_stop_cause, ErrorKind> {
    let kind = match node.text()? {
        "usage" => THINKTHEN_COMPLETE_STOP_CAUSE_USAGE,
        "local" => THINKTHEN_COMPLETE_STOP_CAUSE_LOCAL,
        "no_key" => THINKTHEN_COMPLETE_STOP_CAUSE_NO_KEY,
        "transport" => THINKTHEN_COMPLETE_STOP_CAUSE_TRANSPORT,
        "status" => THINKTHEN_COMPLETE_STOP_CAUSE_STATUS,
        "too_large" => THINKTHEN_COMPLETE_STOP_CAUSE_TOO_LARGE,
        "reply" => THINKTHEN_COMPLETE_STOP_CAUSE_REPLY,
        "backend" => THINKTHEN_COMPLETE_STOP_CAUSE_BACKEND,
        "cancelled" => THINKTHEN_COMPLETE_STOP_CAUSE_CANCELLED,
        "deadline" => THINKTHEN_COMPLETE_STOP_CAUSE_DEADLINE,
        "defect" => THINKTHEN_COMPLETE_STOP_CAUSE_DEFECT,
        _ => return Err(ErrorKind::Defect),
    };
    Ok(thinkthen_complete_stop_cause { kind })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_stopped {
    pub at: thinkthen_complete_u64_presence,
    pub cause: *const thinkthen_complete_stop_cause,
    pub retryable: u32,
    pub status: thinkthen_complete_u64_presence,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_stopped(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_stopped, ErrorKind> {
    Ok(thinkthen_complete_stopped {
        at: {
            let (presence, value) = read_presence(node.member("at"), false, 0, read_number, store)?;
            thinkthen_complete_u64_presence { presence, value }
        },
        cause: {
            let value = node.required("cause")?;
            read_thinkthen_complete_stop_cause(value, store).map(|value| store.hold(value))?
        },
        retryable: {
            let value = node.required("retryable")?;
            read_boolean(value, store)?
        },
        status: {
            let (presence, value) =
                read_presence(node.member("status"), false, 0, read_number, store)?;
            thinkthen_complete_u64_presence { presence, value }
        },
        extensions: store.extensions(node, &["at", "cause", "retryable", "status"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_string_root {
    pub r#type: *const thinkthen_complete_string_type,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_string_root(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_string_root, ErrorKind> {
    Ok(thinkthen_complete_string_root {
        r#type: {
            let value = node.required("type")?;
            read_thinkthen_complete_string_type(value, store).map(|value| store.hold(value))?
        },
        extensions: store.extensions(node, &["type"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_string_type {
    pub kind: u32,
}

pub const THINKTHEN_COMPLETE_STRING_TYPE_STRING: u32 = 1;

pub(crate) fn read_thinkthen_complete_string_type(
    node: &Node,
    _store: &mut Storage,
) -> Result<thinkthen_complete_string_type, ErrorKind> {
    let kind = match node.text()? {
        "string" => THINKTHEN_COMPLETE_STRING_TYPE_STRING,
        _ => return Err(ErrorKind::Defect),
    };
    Ok(thinkthen_complete_string_type { kind })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_usage {
    pub input_tokens: thinkthen_complete_u64_presence,
    pub output_tokens: thinkthen_complete_u64_presence,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_usage(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_usage, ErrorKind> {
    Ok(thinkthen_complete_usage {
        input_tokens: {
            let (presence, value) =
                read_presence(node.member("input_tokens"), false, 0, read_number, store)?;
            thinkthen_complete_u64_presence { presence, value }
        },
        output_tokens: {
            let (presence, value) =
                read_presence(node.member("output_tokens"), false, 0, read_number, store)?;
            thinkthen_complete_u64_presence { presence, value }
        },
        extensions: store.extensions(node, &["input_tokens", "output_tokens"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_usage_persistence {
    pub kind: u32,
}

pub const THINKTHEN_COMPLETE_USAGE_PERSISTENCE_DISABLED: u32 = 1;

pub const THINKTHEN_COMPLETE_USAGE_PERSISTENCE_PENDING: u32 = 2;

pub const THINKTHEN_COMPLETE_USAGE_PERSISTENCE_WRITTEN: u32 = 3;

pub const THINKTHEN_COMPLETE_USAGE_PERSISTENCE_FAILED: u32 = 4;

pub(crate) fn read_thinkthen_complete_usage_persistence(
    node: &Node,
    _store: &mut Storage,
) -> Result<thinkthen_complete_usage_persistence, ErrorKind> {
    let kind = match node.text()? {
        "disabled" => THINKTHEN_COMPLETE_USAGE_PERSISTENCE_DISABLED,
        "pending" => THINKTHEN_COMPLETE_USAGE_PERSISTENCE_PENDING,
        "written" => THINKTHEN_COMPLETE_USAGE_PERSISTENCE_WRITTEN,
        "failed" => THINKTHEN_COMPLETE_USAGE_PERSISTENCE_FAILED,
        _ => return Err(ErrorKind::Defect),
    };
    Ok(thinkthen_complete_usage_persistence { kind })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_verb {
    pub kind: u32,
}

pub const THINKTHEN_COMPLETE_VERB_RECOGNIZE: u32 = 1;

pub(crate) fn read_thinkthen_complete_verb(
    node: &Node,
    _store: &mut Storage,
) -> Result<thinkthen_complete_verb, ErrorKind> {
    let kind = match node.text()? {
        "recognize" => THINKTHEN_COMPLETE_VERB_RECOGNIZE,
        _ => return Err(ErrorKind::Defect),
    };
    Ok(thinkthen_complete_verb { kind })
}

pub const THINKTHEN_COMPLETE_ANNOTATED_FIELD_BOOLEAN: u32 = 1;

pub const THINKTHEN_COMPLETE_ANNOTATED_FIELD_NULL: u32 = 2;

pub const THINKTHEN_COMPLETE_ANNOTATED_FIELD_STRING: u32 = 3;

pub const THINKTHEN_COMPLETE_ANNOTATED_FIELD_ARRAY: u32 = 4;

pub const THINKTHEN_COMPLETE_ANNOTATED_FIELD_NUMBER: u32 = 5;

pub const THINKTHEN_COMPLETE_ANNOTATED_FIELD_OBJECT: u32 = 6;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_annotated_field_data {
    pub boolean: u32,
    pub null: u32,
    pub string: thinkthen_complete_utf8,
    pub array: thinkthen_complete_string_list,
    pub number: f64,
    pub object: *const thinkthen_complete_failed,
}
impl std::fmt::Debug for thinkthen_complete_annotated_field_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("union payload")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_annotated_field {
    pub kind: u32,
    pub data: thinkthen_complete_annotated_field_data,
}

pub(crate) fn read_thinkthen_complete_annotated_field(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_annotated_field, ErrorKind> {
    if node.kind() == "boolean" {
        let value = read_boolean(node, store)?;
        return Ok(thinkthen_complete_annotated_field {
            kind: THINKTHEN_COMPLETE_ANNOTATED_FIELD_BOOLEAN,
            data: thinkthen_complete_annotated_field_data { boolean: value },
        });
    }
    if node.kind() == "null" {
        let value = 0;
        return Ok(thinkthen_complete_annotated_field {
            kind: THINKTHEN_COMPLETE_ANNOTATED_FIELD_NULL,
            data: thinkthen_complete_annotated_field_data { null: value },
        });
    }
    if node.kind() == "string" {
        let value = read_text(node, store)?;
        return Ok(thinkthen_complete_annotated_field {
            kind: THINKTHEN_COMPLETE_ANNOTATED_FIELD_STRING,
            data: thinkthen_complete_annotated_field_data { string: value },
        });
    }
    if node
        .array()
        .is_ok_and(|items| items.iter().all(matches_union_item_11))
    {
        let value = read_thinkthen_complete_string_list(node, store)?;
        return Ok(thinkthen_complete_annotated_field {
            kind: THINKTHEN_COMPLETE_ANNOTATED_FIELD_ARRAY,
            data: thinkthen_complete_annotated_field_data { array: value },
        });
    }
    if node.kind() == "number" {
        let value = read_number(node, store)?;
        return Ok(thinkthen_complete_annotated_field {
            kind: THINKTHEN_COMPLETE_ANNOTATED_FIELD_NUMBER,
            data: thinkthen_complete_annotated_field_data { number: value },
        });
    }
    if node.kind() == "object" {
        let value = read_thinkthen_complete_failed(node, store).map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_annotated_field {
            kind: THINKTHEN_COMPLETE_ANNOTATED_FIELD_OBJECT,
            data: thinkthen_complete_annotated_field_data { object: value },
        });
    }
    Err(ErrorKind::Defect)
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_annotated_field_map_entry {
    pub name: thinkthen_complete_utf8,
    pub value: *const thinkthen_complete_annotated_field,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_annotated_field_map {
    pub data: *const thinkthen_complete_annotated_field_map_entry,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_annotated_field_map(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_annotated_field_map, ErrorKind> {
    let mut values = Vec::new();
    for (key, item) in node.object()? {
        values.push(thinkthen_complete_annotated_field_map_entry {
            name: store.text(key),
            value: read_thinkthen_complete_annotated_field(item, store)
                .map(|value| store.hold(value))?,
        });
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_annotated_field_map { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_annotated_row {
    pub value: thinkthen_complete_annotated_field_map,
}

pub(crate) fn read_thinkthen_complete_annotated_row(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_annotated_row, ErrorKind> {
    let value = read_thinkthen_complete_annotated_field_map(node, store)?;
    Ok(thinkthen_complete_annotated_row { value })
}

pub const THINKTHEN_COMPLETE_ANSWER_YES_NO: u32 = 1;

pub const THINKTHEN_COMPLETE_ANSWER_CHOICE: u32 = 2;

pub const THINKTHEN_COMPLETE_ANSWER_TAG: u32 = 3;

pub const THINKTHEN_COMPLETE_ANSWER_SCORE: u32 = 4;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_answer_data {
    pub yes_no: *const thinkthen_complete_answer_yes_no,
    pub choice: *const thinkthen_complete_answer_choice,
    pub tag: *const thinkthen_complete_answer_tag,
    pub score: *const thinkthen_complete_answer_score,
}
impl std::fmt::Debug for thinkthen_complete_answer_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("union payload")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answer {
    pub kind: u32,
    pub data: thinkthen_complete_answer_data,
}

pub(crate) fn read_thinkthen_complete_answer(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_answer, ErrorKind> {
    if node.is_member("kind", "yes_no") {
        let value =
            read_thinkthen_complete_answer_yes_no(node, store).map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_answer {
            kind: THINKTHEN_COMPLETE_ANSWER_YES_NO,
            data: thinkthen_complete_answer_data { yes_no: value },
        });
    }
    if node.is_member("kind", "choice") {
        let value =
            read_thinkthen_complete_answer_choice(node, store).map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_answer {
            kind: THINKTHEN_COMPLETE_ANSWER_CHOICE,
            data: thinkthen_complete_answer_data { choice: value },
        });
    }
    if node.is_member("kind", "tag") {
        let value =
            read_thinkthen_complete_answer_tag(node, store).map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_answer {
            kind: THINKTHEN_COMPLETE_ANSWER_TAG,
            data: thinkthen_complete_answer_data { tag: value },
        });
    }
    if node.is_member("kind", "score") {
        let value =
            read_thinkthen_complete_answer_score(node, store).map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_answer {
            kind: THINKTHEN_COMPLETE_ANSWER_SCORE,
            data: thinkthen_complete_answer_data { score: value },
        });
    }
    Err(ErrorKind::Defect)
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_number_map_entry {
    pub name: thinkthen_complete_utf8,
    pub value: f64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_number_map {
    pub data: *const thinkthen_complete_number_map_entry,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_number_map(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_number_map, ErrorKind> {
    let mut values = Vec::new();
    for (key, item) in node.object()? {
        values.push(thinkthen_complete_number_map_entry {
            name: store.text(key),
            value: read_number(item, store)?,
        });
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_number_map { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answer_choice {
    pub confidence: thinkthen_complete_f64_presence,
    pub kind: thinkthen_complete_utf8,
    pub pick: thinkthen_complete_utf8,
    pub probabilities: thinkthen_complete_number_map,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_answer_choice(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_answer_choice, ErrorKind> {
    Ok(thinkthen_complete_answer_choice {
        confidence: {
            let (presence, value) =
                read_presence(node.member("confidence"), false, 0.0, read_number, store)?;
            thinkthen_complete_f64_presence { presence, value }
        },
        kind: {
            let value = node.required("kind")?;
            read_text(value, store)?
        },
        pick: {
            let value = node.required("pick")?;
            read_text(value, store)?
        },
        probabilities: {
            let value = node.required("probabilities")?;
            read_thinkthen_complete_number_map(value, store)?
        },
        extensions: store.extensions(node, &["confidence", "kind", "pick", "probabilities"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answer_score {
    pub confidence: thinkthen_complete_f64_presence,
    pub kind: thinkthen_complete_utf8,
    pub level: thinkthen_complete_utf8,
    pub probabilities: thinkthen_complete_number_map,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_answer_score(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_answer_score, ErrorKind> {
    Ok(thinkthen_complete_answer_score {
        confidence: {
            let (presence, value) =
                read_presence(node.member("confidence"), false, 0.0, read_number, store)?;
            thinkthen_complete_f64_presence { presence, value }
        },
        kind: {
            let value = node.required("kind")?;
            read_text(value, store)?
        },
        level: {
            let value = node.required("level")?;
            read_text(value, store)?
        },
        probabilities: {
            let value = node.required("probabilities")?;
            read_thinkthen_complete_number_map(value, store)?
        },
        extensions: store.extensions(node, &["confidence", "kind", "level", "probabilities"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answer_tag {
    pub kind: thinkthen_complete_utf8,
    pub probabilities: thinkthen_complete_number_map,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_answer_tag(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_answer_tag, ErrorKind> {
    Ok(thinkthen_complete_answer_tag {
        kind: {
            let value = node.required("kind")?;
            read_text(value, store)?
        },
        probabilities: {
            let value = node.required("probabilities")?;
            read_thinkthen_complete_number_map(value, store)?
        },
        extensions: store.extensions(node, &["kind", "probabilities"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answer_yes_no {
    pub kind: thinkthen_complete_utf8,
    pub probability: f64,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_answer_yes_no(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_answer_yes_no, ErrorKind> {
    Ok(thinkthen_complete_answer_yes_no {
        kind: {
            let value = node.required("kind")?;
            read_text(value, store)?
        },
        probability: {
            let value = node.required("probability")?;
            read_number(value, store)?
        },
        extensions: store.extensions(node, &["kind", "probability"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_attempt_outcome {
    pub kind: u32,
}

pub const THINKTHEN_COMPLETE_ATTEMPT_OUTCOME_OK: u32 = 1;

pub const THINKTHEN_COMPLETE_ATTEMPT_OUTCOME_STATUS: u32 = 2;

pub const THINKTHEN_COMPLETE_ATTEMPT_OUTCOME_TRANSPORT: u32 = 3;

pub(crate) fn read_thinkthen_complete_attempt_outcome(
    node: &Node,
    _store: &mut Storage,
) -> Result<thinkthen_complete_attempt_outcome, ErrorKind> {
    let kind = match node.text()? {
        "ok" => THINKTHEN_COMPLETE_ATTEMPT_OUTCOME_OK,
        "status" => THINKTHEN_COMPLETE_ATTEMPT_OUTCOME_STATUS,
        "transport" => THINKTHEN_COMPLETE_ATTEMPT_OUTCOME_TRANSPORT,
        _ => return Err(ErrorKind::Defect),
    };
    Ok(thinkthen_complete_attempt_outcome { kind })
}

pub const THINKTHEN_COMPLETE_BATCH_SETTING_INTEGER: u32 = 1;

pub const THINKTHEN_COMPLETE_BATCH_SETTING_STRING: u32 = 2;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_batch_setting_data {
    pub integer: u64,
    pub string: thinkthen_complete_utf8,
}
impl std::fmt::Debug for thinkthen_complete_batch_setting_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("union payload")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_batch_setting {
    pub kind: u32,
    pub data: thinkthen_complete_batch_setting_data,
}

pub(crate) fn read_thinkthen_complete_batch_setting(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_batch_setting, ErrorKind> {
    if node.kind() == "number" {
        let value = read_number(node, store)?;
        return Ok(thinkthen_complete_batch_setting {
            kind: THINKTHEN_COMPLETE_BATCH_SETTING_INTEGER,
            data: thinkthen_complete_batch_setting_data { integer: value },
        });
    }
    if node.kind() == "string" {
        let value = read_text(node, store)?;
        return Ok(thinkthen_complete_batch_setting {
            kind: THINKTHEN_COMPLETE_BATCH_SETTING_STRING,
            data: thinkthen_complete_batch_setting_data { string: value },
        });
    }
    Err(ErrorKind::Defect)
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_batch_warning {
    pub running: *const thinkthen_complete_batch_setting,
    pub tuned_for: *const thinkthen_complete_batch_setting,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_batch_warning(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_batch_warning, ErrorKind> {
    Ok(thinkthen_complete_batch_warning {
        running: {
            let value = node.required("running")?;
            read_thinkthen_complete_batch_setting(value, store).map(|value| store.hold(value))?
        },
        tuned_for: {
            let value = node.required("tuned_for")?;
            read_thinkthen_complete_batch_setting(value, store).map(|value| store.hold(value))?
        },
        extensions: store.extensions(node, &["running", "tuned_for"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_entity {
    pub end: u64,
    pub file: thinkthen_complete_utf8_presence,
    pub first_line: thinkthen_complete_u64_presence,
    pub kind: thinkthen_complete_utf8,
    pub last_line: thinkthen_complete_u64_presence,
    pub length: u64,
    pub start: u64,
    pub strength: f64,
    pub text: thinkthen_complete_utf8,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_entity(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_entity, ErrorKind> {
    Ok(thinkthen_complete_entity {
        end: {
            let value = node.required("end")?;
            read_number(value, store)?
        },
        file: {
            let (presence, value) = read_presence(
                node.member("file"),
                false,
                thinkthen_complete_utf8 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence { presence, value }
        },
        first_line: {
            let (presence, value) =
                read_presence(node.member("first_line"), false, 0, read_number, store)?;
            thinkthen_complete_u64_presence { presence, value }
        },
        kind: {
            let value = node.required("kind")?;
            read_text(value, store)?
        },
        last_line: {
            let (presence, value) =
                read_presence(node.member("last_line"), false, 0, read_number, store)?;
            thinkthen_complete_u64_presence { presence, value }
        },
        length: {
            let value = node.required("length")?;
            read_number(value, store)?
        },
        start: {
            let value = node.required("start")?;
            read_number(value, store)?
        },
        strength: {
            let value = node.required("strength")?;
            read_number(value, store)?
        },
        text: {
            let value = node.required("text")?;
            read_text(value, store)?
        },
        extensions: store.extensions(
            node,
            &[
                "end",
                "file",
                "first_line",
                "kind",
                "last_line",
                "length",
                "start",
                "strength",
                "text",
            ],
        )?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_entity_edge {
    pub either: thinkthen_complete_u32_presence,
    pub probability: f64,
    pub relation: thinkthen_complete_utf8,
    pub source: *const thinkthen_complete_entity,
    pub target: *const thinkthen_complete_entity,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_entity_edge(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_entity_edge, ErrorKind> {
    Ok(thinkthen_complete_entity_edge {
        either: {
            let (presence, value) =
                read_presence(node.member("either"), false, 0, read_boolean, store)?;
            thinkthen_complete_u32_presence { presence, value }
        },
        probability: {
            let value = node.required("probability")?;
            read_number(value, store)?
        },
        relation: {
            let value = node.required("relation")?;
            read_text(value, store)?
        },
        source: {
            let value = node.required("source")?;
            read_thinkthen_complete_entity(value, store).map(|value| store.hold(value))?
        },
        target: {
            let value = node.required("target")?;
            read_thinkthen_complete_entity(value, store).map(|value| store.hold(value))?
        },
        extensions: store.extensions(
            node,
            &["either", "probability", "relation", "source", "target"],
        )?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_failed {
    pub failed: *const thinkthen_complete_failure,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_failed(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_failed, ErrorKind> {
    Ok(thinkthen_complete_failed {
        failed: {
            let value = node.required("failed")?;
            read_thinkthen_complete_failure(value, store).map(|value| store.hold(value))?
        },
        extensions: store.extensions(node, &["failed"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_failure_field_kind {
    pub kind: u32,
}

pub const THINKTHEN_COMPLETE_FAILURE_FIELD_KIND_BACKEND: u32 = 1;

pub(crate) fn read_thinkthen_complete_failure_field_kind(
    node: &Node,
    _store: &mut Storage,
) -> Result<thinkthen_complete_failure_field_kind, ErrorKind> {
    let kind = match node.text()? {
        "backend" => THINKTHEN_COMPLETE_FAILURE_FIELD_KIND_BACKEND,
        _ => return Err(ErrorKind::Defect),
    };
    Ok(thinkthen_complete_failure_field_kind { kind })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_failure {
    pub cause: *const thinkthen_complete_failure_cause,
    pub kind: *const thinkthen_complete_failure_field_kind,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_failure(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_failure, ErrorKind> {
    Ok(thinkthen_complete_failure {
        cause: {
            let value = node.required("cause")?;
            read_thinkthen_complete_failure_cause(value, store).map(|value| store.hold(value))?
        },
        kind: {
            let value = node.required("kind")?;
            read_thinkthen_complete_failure_field_kind(value, store)
                .map(|value| store.hold(value))?
        },
        extensions: store.extensions(node, &["cause", "kind"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_failure_cause {
    pub kind: u32,
}

pub const THINKTHEN_COMPLETE_FAILURE_CAUSE_MISSING_ANSWER: u32 = 1;

pub const THINKTHEN_COMPLETE_FAILURE_CAUSE_WRONG_KIND: u32 = 2;

pub const THINKTHEN_COMPLETE_FAILURE_CAUSE_MISSING_PROBABILITY: u32 = 3;

pub const THINKTHEN_COMPLETE_FAILURE_CAUSE_INVALID_PROBABILITY: u32 = 4;

pub const THINKTHEN_COMPLETE_FAILURE_CAUSE_INVALID_DISTRIBUTION: u32 = 5;

pub const THINKTHEN_COMPLETE_FAILURE_CAUSE_UNEXPECTED_PROBABILITY: u32 = 6;

pub(crate) fn read_thinkthen_complete_failure_cause(
    node: &Node,
    _store: &mut Storage,
) -> Result<thinkthen_complete_failure_cause, ErrorKind> {
    let kind = match node.text()? {
        "missing_answer" => THINKTHEN_COMPLETE_FAILURE_CAUSE_MISSING_ANSWER,
        "wrong_kind" => THINKTHEN_COMPLETE_FAILURE_CAUSE_WRONG_KIND,
        "missing_probability" => THINKTHEN_COMPLETE_FAILURE_CAUSE_MISSING_PROBABILITY,
        "invalid_probability" => THINKTHEN_COMPLETE_FAILURE_CAUSE_INVALID_PROBABILITY,
        "invalid_distribution" => THINKTHEN_COMPLETE_FAILURE_CAUSE_INVALID_DISTRIBUTION,
        "unexpected_probability" => THINKTHEN_COMPLETE_FAILURE_CAUSE_UNEXPECTED_PROBABILITY,
        _ => return Err(ErrorKind::Defect),
    };
    Ok(thinkthen_complete_failure_cause { kind })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_failure_kind {
    pub kind: u32,
}

pub const THINKTHEN_COMPLETE_FAILURE_KIND_USAGE: u32 = 1;

pub const THINKTHEN_COMPLETE_FAILURE_KIND_BACKEND: u32 = 2;

pub const THINKTHEN_COMPLETE_FAILURE_KIND_LOCAL: u32 = 3;

pub const THINKTHEN_COMPLETE_FAILURE_KIND_CANCELLED: u32 = 4;

pub const THINKTHEN_COMPLETE_FAILURE_KIND_DEADLINE: u32 = 5;

pub const THINKTHEN_COMPLETE_FAILURE_KIND_DEFECT: u32 = 6;

pub(crate) fn read_thinkthen_complete_failure_kind(
    node: &Node,
    _store: &mut Storage,
) -> Result<thinkthen_complete_failure_kind, ErrorKind> {
    let kind = match node.text()? {
        "usage" => THINKTHEN_COMPLETE_FAILURE_KIND_USAGE,
        "backend" => THINKTHEN_COMPLETE_FAILURE_KIND_BACKEND,
        "local" => THINKTHEN_COMPLETE_FAILURE_KIND_LOCAL,
        "cancelled" => THINKTHEN_COMPLETE_FAILURE_KIND_CANCELLED,
        "deadline" => THINKTHEN_COMPLETE_FAILURE_KIND_DEADLINE,
        "defect" => THINKTHEN_COMPLETE_FAILURE_KIND_DEFECT,
        _ => return Err(ErrorKind::Defect),
    };
    Ok(thinkthen_complete_failure_kind { kind })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_find_answer {
    pub confidence: thinkthen_complete_f64_presence,
    pub kind: thinkthen_complete_utf8,
    pub pick: thinkthen_complete_utf8,
    pub probabilities: thinkthen_complete_number_map,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_find_answer(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_find_answer, ErrorKind> {
    Ok(thinkthen_complete_find_answer {
        confidence: {
            let (presence, value) =
                read_presence(node.member("confidence"), false, 0.0, read_number, store)?;
            thinkthen_complete_f64_presence { presence, value }
        },
        kind: {
            let value = node.required("kind")?;
            read_text(value, store)?
        },
        pick: {
            let value = node.required("pick")?;
            read_text(value, store)?
        },
        probabilities: {
            let value = node.required("probabilities")?;
            read_thinkthen_complete_number_map(value, store)?
        },
        extensions: store.extensions(node, &["confidence", "kind", "pick", "probabilities"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_batch_presence {
    pub presence: u32,
    pub value: *const thinkthen_complete_batch,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_input_declaration_presence {
    pub presence: u32,
    pub value: *const thinkthen_complete_input_declaration,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_label_list {
    pub data: *const *const thinkthen_complete_label,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_label_list(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_label_list, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(read_thinkthen_complete_label(item, store).map(|value| store.hold(value))?);
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_label_list { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_label_list_presence {
    pub presence: u32,
    pub value: thinkthen_complete_label_list,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_question_name_presence {
    pub presence: u32,
    pub value: *const thinkthen_complete_question_name,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_find_question_document {
    pub batch: thinkthen_complete_batch_presence,
    pub context_schema: thinkthen_complete_input_declaration_presence,
    pub item_schema: thinkthen_complete_input_declaration_presence,
    pub label_details: thinkthen_complete_label_list_presence,
    pub model: thinkthen_complete_utf8_presence,
    pub name: thinkthen_complete_question_name_presence,
    pub none: u32,
    pub on: thinkthen_complete_string_list_presence,
    pub profile: thinkthen_complete_utf8_presence,
    pub text: thinkthen_complete_json_presence,
    pub verb: thinkthen_complete_utf8,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_find_question_document(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_find_question_document, ErrorKind> {
    Ok(thinkthen_complete_find_question_document {
        batch: {
            let (presence, value) = read_presence(
                node.member("batch"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_batch(value, store).map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_batch_presence { presence, value }
        },
        context_schema: {
            let (presence, value) = read_presence(
                node.member("context_schema"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_input_declaration(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_input_declaration_presence { presence, value }
        },
        item_schema: {
            let (presence, value) = read_presence(
                node.member("item_schema"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_input_declaration(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_input_declaration_presence { presence, value }
        },
        label_details: {
            let (presence, value) = read_presence(
                node.member("label_details"),
                false,
                thinkthen_complete_label_list {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_label_list,
                store,
            )?;
            thinkthen_complete_label_list_presence { presence, value }
        },
        model: {
            let (presence, value) = read_presence(
                node.member("model"),
                false,
                thinkthen_complete_utf8 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence { presence, value }
        },
        name: {
            let (presence, value) = read_presence(
                node.member("name"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_question_name(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_question_name_presence { presence, value }
        },
        none: {
            let value = node.required("none")?;
            read_boolean(value, store)?
        },
        on: {
            let (presence, value) = read_presence(
                node.member("on"),
                false,
                thinkthen_complete_string_list {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_string_list,
                store,
            )?;
            thinkthen_complete_string_list_presence { presence, value }
        },
        profile: {
            let (presence, value) = read_presence(
                node.member("profile"),
                false,
                thinkthen_complete_utf8 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence { presence, value }
        },
        text: {
            let (presence, value) = read_presence(
                node.member("text"),
                true,
                std::ptr::null(),
                read_json,
                store,
            )?;
            thinkthen_complete_json_presence { presence, value }
        },
        verb: {
            let value = node.required("verb")?;
            read_text(value, store)?
        },
        extensions: store.extensions(
            node,
            &[
                "batch",
                "context_schema",
                "item_schema",
                "label_details",
                "model",
                "name",
                "none",
                "on",
                "profile",
                "text",
                "verb",
            ],
        )?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_number_map_presence {
    pub presence: u32,
    pub value: thinkthen_complete_number_map,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_name_odds {
    pub edges: thinkthen_complete_number_map_presence,
    pub end: u64,
    pub kinds: thinkthen_complete_number_map_presence,
    pub start: u64,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_name_odds(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_name_odds, ErrorKind> {
    Ok(thinkthen_complete_name_odds {
        edges: {
            let (presence, value) = read_presence(
                node.member("edges"),
                true,
                thinkthen_complete_number_map {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_number_map,
                store,
            )?;
            thinkthen_complete_number_map_presence { presence, value }
        },
        end: {
            let value = node.required("end")?;
            read_number(value, store)?
        },
        kinds: {
            let (presence, value) = read_presence(
                node.member("kinds"),
                true,
                thinkthen_complete_number_map {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_number_map,
                store,
            )?;
            thinkthen_complete_number_map_presence { presence, value }
        },
        start: {
            let value = node.required("start")?;
            read_number(value, store)?
        },
        extensions: store.extensions(node, &["edges", "end", "kinds", "start"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_pair_odds {
    pub probability: f64,
    pub relation: thinkthen_complete_utf8,
    pub source: *const thinkthen_complete_place,
    pub target: *const thinkthen_complete_place,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_pair_odds(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_pair_odds, ErrorKind> {
    Ok(thinkthen_complete_pair_odds {
        probability: {
            let value = node.required("probability")?;
            read_number(value, store)?
        },
        relation: {
            let value = node.required("relation")?;
            read_text(value, store)?
        },
        source: {
            let value = node.required("source")?;
            read_thinkthen_complete_place(value, store).map(|value| store.hold(value))?
        },
        target: {
            let value = node.required("target")?;
            read_thinkthen_complete_place(value, store).map(|value| store.hold(value))?
        },
        extensions: store.extensions(node, &["probability", "relation", "source", "target"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_piece_odds {
    pub end: u64,
    pub start: u64,
    pub tags: thinkthen_complete_number_map,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_piece_odds(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_piece_odds, ErrorKind> {
    Ok(thinkthen_complete_piece_odds {
        end: {
            let value = node.required("end")?;
            read_number(value, store)?
        },
        start: {
            let value = node.required("start")?;
            read_number(value, store)?
        },
        tags: {
            let value = node.required("tags")?;
            read_thinkthen_complete_number_map(value, store)?
        },
        extensions: store.extensions(node, &["end", "start", "tags"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_place {
    pub end: u64,
    pub start: u64,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_place(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_place, ErrorKind> {
    Ok(thinkthen_complete_place {
        end: {
            let value = node.required("end")?;
            read_number(value, store)?
        },
        start: {
            let value = node.required("start")?;
            read_number(value, store)?
        },
        extensions: store.extensions(node, &["end", "start"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_profile_warning {
    pub running: thinkthen_complete_utf8,
    pub tuned_for: thinkthen_complete_utf8,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_profile_warning(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_profile_warning, ErrorKind> {
    Ok(thinkthen_complete_profile_warning {
        running: {
            let value = node.required("running")?;
            read_text(value, store)?
        },
        tuned_for: {
            let value = node.required("tuned_for")?;
            read_text(value, store)?
        },
        extensions: store.extensions(node, &["running", "tuned_for"])?,
    })
}

pub const THINKTHEN_COMPLETE_QUESTION_DOCUMENT_DECIDE: u32 = 1;

pub const THINKTHEN_COMPLETE_QUESTION_DOCUMENT_CHOOSE: u32 = 2;

pub const THINKTHEN_COMPLETE_QUESTION_DOCUMENT_TAG: u32 = 3;

pub const THINKTHEN_COMPLETE_QUESTION_DOCUMENT_SCORE: u32 = 4;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_question_document_data {
    pub decide: *const thinkthen_complete_question_document_decide,
    pub choose: *const thinkthen_complete_question_document_choose,
    pub tag: *const thinkthen_complete_question_document_tag,
    pub score: *const thinkthen_complete_question_document_score,
}
impl std::fmt::Debug for thinkthen_complete_question_document_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("union payload")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_question_document {
    pub kind: u32,
    pub data: thinkthen_complete_question_document_data,
}

pub(crate) fn read_thinkthen_complete_question_document(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_question_document, ErrorKind> {
    if node.is_member("verb", "decide") {
        let value = read_thinkthen_complete_question_document_decide(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_question_document {
            kind: THINKTHEN_COMPLETE_QUESTION_DOCUMENT_DECIDE,
            data: thinkthen_complete_question_document_data { decide: value },
        });
    }
    if node.is_member("verb", "choose") {
        let value = read_thinkthen_complete_question_document_choose(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_question_document {
            kind: THINKTHEN_COMPLETE_QUESTION_DOCUMENT_CHOOSE,
            data: thinkthen_complete_question_document_data { choose: value },
        });
    }
    if node.is_member("verb", "tag") {
        let value = read_thinkthen_complete_question_document_tag(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_question_document {
            kind: THINKTHEN_COMPLETE_QUESTION_DOCUMENT_TAG,
            data: thinkthen_complete_question_document_data { tag: value },
        });
    }
    if node.is_member("verb", "score") {
        let value = read_thinkthen_complete_question_document_score(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_question_document {
            kind: THINKTHEN_COMPLETE_QUESTION_DOCUMENT_SCORE,
            data: thinkthen_complete_question_document_data { score: value },
        });
    }
    Err(ErrorKind::Defect)
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_question_document_choose {
    pub batch: thinkthen_complete_batch_presence,
    pub context_schema: thinkthen_complete_input_declaration_presence,
    pub item_schema: thinkthen_complete_input_declaration_presence,
    pub label_details: thinkthen_complete_label_list_presence,
    pub model: thinkthen_complete_utf8_presence,
    pub name: thinkthen_complete_question_name_presence,
    pub on: thinkthen_complete_string_list_presence,
    pub profile: thinkthen_complete_utf8_presence,
    pub options: thinkthen_complete_string_list,
    pub text: thinkthen_complete_json_presence,
    pub verb: thinkthen_complete_utf8,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_question_document_choose(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_question_document_choose, ErrorKind> {
    Ok(thinkthen_complete_question_document_choose {
        batch: {
            let (presence, value) = read_presence(
                node.member("batch"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_batch(value, store).map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_batch_presence { presence, value }
        },
        context_schema: {
            let (presence, value) = read_presence(
                node.member("context_schema"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_input_declaration(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_input_declaration_presence { presence, value }
        },
        item_schema: {
            let (presence, value) = read_presence(
                node.member("item_schema"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_input_declaration(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_input_declaration_presence { presence, value }
        },
        label_details: {
            let (presence, value) = read_presence(
                node.member("label_details"),
                false,
                thinkthen_complete_label_list {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_label_list,
                store,
            )?;
            thinkthen_complete_label_list_presence { presence, value }
        },
        model: {
            let (presence, value) = read_presence(
                node.member("model"),
                false,
                thinkthen_complete_utf8 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence { presence, value }
        },
        name: {
            let (presence, value) = read_presence(
                node.member("name"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_question_name(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_question_name_presence { presence, value }
        },
        on: {
            let (presence, value) = read_presence(
                node.member("on"),
                false,
                thinkthen_complete_string_list {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_string_list,
                store,
            )?;
            thinkthen_complete_string_list_presence { presence, value }
        },
        profile: {
            let (presence, value) = read_presence(
                node.member("profile"),
                false,
                thinkthen_complete_utf8 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence { presence, value }
        },
        options: {
            let value = node.required("options")?;
            read_thinkthen_complete_string_list(value, store)?
        },
        text: {
            let (presence, value) = read_presence(
                node.member("text"),
                true,
                std::ptr::null(),
                read_json,
                store,
            )?;
            thinkthen_complete_json_presence { presence, value }
        },
        verb: {
            let value = node.required("verb")?;
            read_text(value, store)?
        },
        extensions: store.extensions(
            node,
            &[
                "batch",
                "context_schema",
                "item_schema",
                "label_details",
                "model",
                "name",
                "on",
                "profile",
                "options",
                "text",
                "verb",
            ],
        )?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_question_document_decide {
    pub batch: thinkthen_complete_batch_presence,
    pub context_schema: thinkthen_complete_input_declaration_presence,
    pub item_schema: thinkthen_complete_input_declaration_presence,
    pub label_details: thinkthen_complete_label_list_presence,
    pub model: thinkthen_complete_utf8_presence,
    pub name: thinkthen_complete_question_name_presence,
    pub on: thinkthen_complete_string_list_presence,
    pub profile: thinkthen_complete_utf8_presence,
    pub false_: thinkthen_complete_json_presence,
    pub text: thinkthen_complete_json_presence,
    pub true_: thinkthen_complete_json_presence,
    pub verb: thinkthen_complete_utf8,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_question_document_decide(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_question_document_decide, ErrorKind> {
    Ok(thinkthen_complete_question_document_decide {
        batch: {
            let (presence, value) = read_presence(
                node.member("batch"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_batch(value, store).map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_batch_presence { presence, value }
        },
        context_schema: {
            let (presence, value) = read_presence(
                node.member("context_schema"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_input_declaration(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_input_declaration_presence { presence, value }
        },
        item_schema: {
            let (presence, value) = read_presence(
                node.member("item_schema"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_input_declaration(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_input_declaration_presence { presence, value }
        },
        label_details: {
            let (presence, value) = read_presence(
                node.member("label_details"),
                false,
                thinkthen_complete_label_list {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_label_list,
                store,
            )?;
            thinkthen_complete_label_list_presence { presence, value }
        },
        model: {
            let (presence, value) = read_presence(
                node.member("model"),
                false,
                thinkthen_complete_utf8 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence { presence, value }
        },
        name: {
            let (presence, value) = read_presence(
                node.member("name"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_question_name(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_question_name_presence { presence, value }
        },
        on: {
            let (presence, value) = read_presence(
                node.member("on"),
                false,
                thinkthen_complete_string_list {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_string_list,
                store,
            )?;
            thinkthen_complete_string_list_presence { presence, value }
        },
        profile: {
            let (presence, value) = read_presence(
                node.member("profile"),
                false,
                thinkthen_complete_utf8 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence { presence, value }
        },
        false_: {
            let (presence, value) = read_presence(
                node.member("false"),
                false,
                std::ptr::null(),
                read_json,
                store,
            )?;
            thinkthen_complete_json_presence { presence, value }
        },
        text: {
            let (presence, value) = read_presence(
                node.member("text"),
                true,
                std::ptr::null(),
                read_json,
                store,
            )?;
            thinkthen_complete_json_presence { presence, value }
        },
        true_: {
            let (presence, value) = read_presence(
                node.member("true"),
                false,
                std::ptr::null(),
                read_json,
                store,
            )?;
            thinkthen_complete_json_presence { presence, value }
        },
        verb: {
            let value = node.required("verb")?;
            read_text(value, store)?
        },
        extensions: store.extensions(
            node,
            &[
                "batch",
                "context_schema",
                "item_schema",
                "label_details",
                "model",
                "name",
                "on",
                "profile",
                "false",
                "text",
                "true",
                "verb",
            ],
        )?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_question_document_score {
    pub batch: thinkthen_complete_batch_presence,
    pub context_schema: thinkthen_complete_input_declaration_presence,
    pub item_schema: thinkthen_complete_input_declaration_presence,
    pub label_details: thinkthen_complete_label_list_presence,
    pub model: thinkthen_complete_utf8_presence,
    pub name: thinkthen_complete_question_name_presence,
    pub on: thinkthen_complete_string_list_presence,
    pub profile: thinkthen_complete_utf8_presence,
    pub levels: thinkthen_complete_string_list,
    pub text: thinkthen_complete_json_presence,
    pub verb: thinkthen_complete_utf8,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_question_document_score(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_question_document_score, ErrorKind> {
    Ok(thinkthen_complete_question_document_score {
        batch: {
            let (presence, value) = read_presence(
                node.member("batch"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_batch(value, store).map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_batch_presence { presence, value }
        },
        context_schema: {
            let (presence, value) = read_presence(
                node.member("context_schema"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_input_declaration(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_input_declaration_presence { presence, value }
        },
        item_schema: {
            let (presence, value) = read_presence(
                node.member("item_schema"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_input_declaration(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_input_declaration_presence { presence, value }
        },
        label_details: {
            let (presence, value) = read_presence(
                node.member("label_details"),
                false,
                thinkthen_complete_label_list {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_label_list,
                store,
            )?;
            thinkthen_complete_label_list_presence { presence, value }
        },
        model: {
            let (presence, value) = read_presence(
                node.member("model"),
                false,
                thinkthen_complete_utf8 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence { presence, value }
        },
        name: {
            let (presence, value) = read_presence(
                node.member("name"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_question_name(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_question_name_presence { presence, value }
        },
        on: {
            let (presence, value) = read_presence(
                node.member("on"),
                false,
                thinkthen_complete_string_list {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_string_list,
                store,
            )?;
            thinkthen_complete_string_list_presence { presence, value }
        },
        profile: {
            let (presence, value) = read_presence(
                node.member("profile"),
                false,
                thinkthen_complete_utf8 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence { presence, value }
        },
        levels: {
            let value = node.required("levels")?;
            read_thinkthen_complete_string_list(value, store)?
        },
        text: {
            let (presence, value) = read_presence(
                node.member("text"),
                true,
                std::ptr::null(),
                read_json,
                store,
            )?;
            thinkthen_complete_json_presence { presence, value }
        },
        verb: {
            let value = node.required("verb")?;
            read_text(value, store)?
        },
        extensions: store.extensions(
            node,
            &[
                "batch",
                "context_schema",
                "item_schema",
                "label_details",
                "model",
                "name",
                "on",
                "profile",
                "levels",
                "text",
                "verb",
            ],
        )?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_question_document_tag {
    pub batch: thinkthen_complete_batch_presence,
    pub context_schema: thinkthen_complete_input_declaration_presence,
    pub item_schema: thinkthen_complete_input_declaration_presence,
    pub label_details: thinkthen_complete_label_list_presence,
    pub model: thinkthen_complete_utf8_presence,
    pub name: thinkthen_complete_question_name_presence,
    pub on: thinkthen_complete_string_list_presence,
    pub profile: thinkthen_complete_utf8_presence,
    pub labels: thinkthen_complete_string_list,
    pub text: thinkthen_complete_json_presence,
    pub verb: thinkthen_complete_utf8,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_question_document_tag(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_question_document_tag, ErrorKind> {
    Ok(thinkthen_complete_question_document_tag {
        batch: {
            let (presence, value) = read_presence(
                node.member("batch"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_batch(value, store).map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_batch_presence { presence, value }
        },
        context_schema: {
            let (presence, value) = read_presence(
                node.member("context_schema"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_input_declaration(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_input_declaration_presence { presence, value }
        },
        item_schema: {
            let (presence, value) = read_presence(
                node.member("item_schema"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_input_declaration(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_input_declaration_presence { presence, value }
        },
        label_details: {
            let (presence, value) = read_presence(
                node.member("label_details"),
                false,
                thinkthen_complete_label_list {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_label_list,
                store,
            )?;
            thinkthen_complete_label_list_presence { presence, value }
        },
        model: {
            let (presence, value) = read_presence(
                node.member("model"),
                false,
                thinkthen_complete_utf8 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence { presence, value }
        },
        name: {
            let (presence, value) = read_presence(
                node.member("name"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_question_name(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_question_name_presence { presence, value }
        },
        on: {
            let (presence, value) = read_presence(
                node.member("on"),
                false,
                thinkthen_complete_string_list {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_string_list,
                store,
            )?;
            thinkthen_complete_string_list_presence { presence, value }
        },
        profile: {
            let (presence, value) = read_presence(
                node.member("profile"),
                false,
                thinkthen_complete_utf8 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence { presence, value }
        },
        labels: {
            let value = node.required("labels")?;
            read_thinkthen_complete_string_list(value, store)?
        },
        text: {
            let (presence, value) = read_presence(
                node.member("text"),
                true,
                std::ptr::null(),
                read_json,
                store,
            )?;
            thinkthen_complete_json_presence { presence, value }
        },
        verb: {
            let value = node.required("verb")?;
            read_text(value, store)?
        },
        extensions: store.extensions(
            node,
            &[
                "batch",
                "context_schema",
                "item_schema",
                "label_details",
                "model",
                "name",
                "on",
                "profile",
                "labels",
                "text",
                "verb",
            ],
        )?,
    })
}

pub const THINKTHEN_COMPLETE_RECOGNIZE_ENTITIES: u32 = 1;

pub const THINKTHEN_COMPLETE_RECOGNIZE_MODE_PROPOSALS: u32 = 2;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_recognize_data {
    pub entities: *const thinkthen_complete_recognize_entities,
    pub mode_proposals: *const thinkthen_complete_recognize_mode_proposals,
}
impl std::fmt::Debug for thinkthen_complete_recognize_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("union payload")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_recognize {
    pub kind: u32,
    pub data: thinkthen_complete_recognize_data,
}

pub(crate) fn read_thinkthen_complete_recognize(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_recognize, ErrorKind> {
    if node.member("entities").is_some()
        && node.member("mode").is_none()
        && node.member("proposals").is_none()
    {
        let value = read_thinkthen_complete_recognize_entities(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_recognize {
            kind: THINKTHEN_COMPLETE_RECOGNIZE_ENTITIES,
            data: thinkthen_complete_recognize_data { entities: value },
        });
    }
    if node.member("mode").is_some()
        && node.member("proposals").is_some()
        && node.member("entities").is_none()
    {
        let value = read_thinkthen_complete_recognize_mode_proposals(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_recognize {
            kind: THINKTHEN_COMPLETE_RECOGNIZE_MODE_PROPOSALS,
            data: thinkthen_complete_recognize_data {
                mode_proposals: value,
            },
        });
    }
    Err(ErrorKind::Defect)
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_json_map_entry {
    pub name: thinkthen_complete_utf8,
    pub value: *const thinkthen_complete_json,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_json_map {
    pub data: *const thinkthen_complete_json_map_entry,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_json_map(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_json_map, ErrorKind> {
    let mut values = Vec::new();
    for (key, item) in node.object()? {
        values.push(thinkthen_complete_json_map_entry {
            name: store.text(key),
            value: read_json(item, store)?,
        });
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_json_map { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_recognition_mode_presence {
    pub presence: u32,
    pub value: *const thinkthen_complete_recognition_mode,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_relation_rule_list {
    pub data: *const *const thinkthen_complete_relation_rule,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_relation_rule_list(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_relation_rule_list, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_relation_rule(item, store).map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_relation_rule_list { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_relation_rule_list_presence {
    pub presence: u32,
    pub value: thinkthen_complete_relation_rule_list,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_recognition_stage_context_presence {
    pub presence: u32,
    pub value: *const thinkthen_complete_recognition_stage_context,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_recognize_question_document {
    pub batch: thinkthen_complete_batch_presence,
    pub context_schema: thinkthen_complete_input_declaration_presence,
    pub entity_definition: thinkthen_complete_json_presence,
    pub instructions: thinkthen_complete_json_presence,
    pub item_schema: thinkthen_complete_input_declaration_presence,
    pub kinds: thinkthen_complete_json_map,
    pub label_details: thinkthen_complete_label_list_presence,
    pub mode: thinkthen_complete_recognition_mode_presence,
    pub model: thinkthen_complete_utf8_presence,
    pub name: thinkthen_complete_question_name_presence,
    pub on: thinkthen_complete_string_list_presence,
    pub profile: thinkthen_complete_utf8_presence,
    pub relation_threshold: thinkthen_complete_threshold_presence,
    pub relations: thinkthen_complete_relation_rule_list_presence,
    pub snippet_pieces: thinkthen_complete_u64_presence,
    pub stage_context: thinkthen_complete_recognition_stage_context_presence,
    pub threshold: *const thinkthen_complete_threshold,
    pub verb: *const thinkthen_complete_verb,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_recognize_question_document(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_recognize_question_document, ErrorKind> {
    Ok(thinkthen_complete_recognize_question_document {
        batch: {
            let (presence, value) = read_presence(
                node.member("batch"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_batch(value, store).map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_batch_presence { presence, value }
        },
        context_schema: {
            let (presence, value) = read_presence(
                node.member("context_schema"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_input_declaration(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_input_declaration_presence { presence, value }
        },
        entity_definition: {
            let (presence, value) = read_presence(
                node.member("entity_definition"),
                false,
                std::ptr::null(),
                read_json,
                store,
            )?;
            thinkthen_complete_json_presence { presence, value }
        },
        instructions: {
            let (presence, value) = read_presence(
                node.member("instructions"),
                false,
                std::ptr::null(),
                read_json,
                store,
            )?;
            thinkthen_complete_json_presence { presence, value }
        },
        item_schema: {
            let (presence, value) = read_presence(
                node.member("item_schema"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_input_declaration(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_input_declaration_presence { presence, value }
        },
        kinds: {
            let value = node.required("kinds")?;
            read_thinkthen_complete_json_map(value, store)?
        },
        label_details: {
            let (presence, value) = read_presence(
                node.member("label_details"),
                false,
                thinkthen_complete_label_list {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_label_list,
                store,
            )?;
            thinkthen_complete_label_list_presence { presence, value }
        },
        mode: {
            let (presence, value) = read_presence(
                node.member("mode"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_recognition_mode(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_recognition_mode_presence { presence, value }
        },
        model: {
            let (presence, value) = read_presence(
                node.member("model"),
                false,
                thinkthen_complete_utf8 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence { presence, value }
        },
        name: {
            let (presence, value) = read_presence(
                node.member("name"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_question_name(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_question_name_presence { presence, value }
        },
        on: {
            let (presence, value) = read_presence(
                node.member("on"),
                false,
                thinkthen_complete_string_list {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_string_list,
                store,
            )?;
            thinkthen_complete_string_list_presence { presence, value }
        },
        profile: {
            let (presence, value) = read_presence(
                node.member("profile"),
                false,
                thinkthen_complete_utf8 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence { presence, value }
        },
        relation_threshold: {
            let (presence, value) = read_presence(
                node.member("relation_threshold"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_threshold(value, store).map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_threshold_presence { presence, value }
        },
        relations: {
            let (presence, value) = read_presence(
                node.member("relations"),
                false,
                thinkthen_complete_relation_rule_list {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_relation_rule_list,
                store,
            )?;
            thinkthen_complete_relation_rule_list_presence { presence, value }
        },
        snippet_pieces: {
            let (presence, value) =
                read_presence(node.member("snippet_pieces"), false, 0, read_number, store)?;
            thinkthen_complete_u64_presence { presence, value }
        },
        stage_context: {
            let (presence, value) = read_presence(
                node.member("stage_context"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_recognition_stage_context(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_recognition_stage_context_presence { presence, value }
        },
        threshold: {
            let value = node.required("threshold")?;
            read_thinkthen_complete_threshold(value, store).map(|value| store.hold(value))?
        },
        verb: {
            let value = node.required("verb")?;
            read_thinkthen_complete_verb(value, store).map(|value| store.hold(value))?
        },
        extensions: store.extensions(
            node,
            &[
                "batch",
                "context_schema",
                "entity_definition",
                "instructions",
                "item_schema",
                "kinds",
                "label_details",
                "mode",
                "model",
                "name",
                "on",
                "profile",
                "relation_threshold",
                "relations",
                "snippet_pieces",
                "stage_context",
                "threshold",
                "verb",
            ],
        )?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_recognize_entities {
    pub entities: thinkthen_complete_entity_list,
    pub relations: thinkthen_complete_entity_edge_list_presence,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_recognize_entities(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_recognize_entities, ErrorKind> {
    Ok(thinkthen_complete_recognize_entities {
        entities: {
            let value = node.required("entities")?;
            read_thinkthen_complete_entity_list(value, store)?
        },
        relations: {
            let (presence, value) = read_presence(
                node.member("relations"),
                false,
                thinkthen_complete_entity_edge_list {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_entity_edge_list,
                store,
            )?;
            thinkthen_complete_entity_edge_list_presence { presence, value }
        },
        extensions: store.extensions(node, &["entities", "relations"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_recognize_mode_proposals {
    pub mode: *const thinkthen_complete_boundary_mode,
    pub proposals: thinkthen_complete_boundary_proposal_list,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_recognize_mode_proposals(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_recognize_mode_proposals, ErrorKind> {
    Ok(thinkthen_complete_recognize_mode_proposals {
        mode: {
            let value = node.required("mode")?;
            read_thinkthen_complete_boundary_mode(value, store).map(|value| store.hold(value))?
        },
        proposals: {
            let value = node.required("proposals")?;
            read_thinkthen_complete_boundary_proposal_list(value, store)?
        },
        extensions: store.extensions(node, &["mode", "proposals"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_relate_fields {
    pub kind: thinkthen_complete_utf8,
    pub name: thinkthen_complete_utf8,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_relate_fields(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_relate_fields, ErrorKind> {
    Ok(thinkthen_complete_relate_fields {
        kind: {
            let value = node.required("kind")?;
            read_text(value, store)?
        },
        name: {
            let value = node.required("name")?;
            read_text(value, store)?
        },
        extensions: store.extensions(node, &["kind", "name"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_relate_fields_presence {
    pub presence: u32,
    pub value: *const thinkthen_complete_relate_fields,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_relate_question_document {
    pub batch: thinkthen_complete_batch_presence,
    pub context_schema: thinkthen_complete_input_declaration_presence,
    pub fields: thinkthen_complete_relate_fields_presence,
    pub item_schema: thinkthen_complete_input_declaration_presence,
    pub label_details: thinkthen_complete_label_list_presence,
    pub model: thinkthen_complete_utf8_presence,
    pub name: thinkthen_complete_question_name_presence,
    pub on: thinkthen_complete_string_list_presence,
    pub profile: thinkthen_complete_utf8_presence,
    pub relations: thinkthen_complete_relation_rule_list,
    pub threshold: *const thinkthen_complete_threshold,
    pub verb: thinkthen_complete_utf8,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_relate_question_document(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_relate_question_document, ErrorKind> {
    Ok(thinkthen_complete_relate_question_document {
        batch: {
            let (presence, value) = read_presence(
                node.member("batch"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_batch(value, store).map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_batch_presence { presence, value }
        },
        context_schema: {
            let (presence, value) = read_presence(
                node.member("context_schema"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_input_declaration(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_input_declaration_presence { presence, value }
        },
        fields: {
            let (presence, value) = read_presence(
                node.member("fields"),
                true,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_relate_fields(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_relate_fields_presence { presence, value }
        },
        item_schema: {
            let (presence, value) = read_presence(
                node.member("item_schema"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_input_declaration(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_input_declaration_presence { presence, value }
        },
        label_details: {
            let (presence, value) = read_presence(
                node.member("label_details"),
                false,
                thinkthen_complete_label_list {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_label_list,
                store,
            )?;
            thinkthen_complete_label_list_presence { presence, value }
        },
        model: {
            let (presence, value) = read_presence(
                node.member("model"),
                false,
                thinkthen_complete_utf8 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence { presence, value }
        },
        name: {
            let (presence, value) = read_presence(
                node.member("name"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_question_name(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_question_name_presence { presence, value }
        },
        on: {
            let (presence, value) = read_presence(
                node.member("on"),
                false,
                thinkthen_complete_string_list {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_string_list,
                store,
            )?;
            thinkthen_complete_string_list_presence { presence, value }
        },
        profile: {
            let (presence, value) = read_presence(
                node.member("profile"),
                false,
                thinkthen_complete_utf8 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence { presence, value }
        },
        relations: {
            let value = node.required("relations")?;
            read_thinkthen_complete_relation_rule_list(value, store)?
        },
        threshold: {
            let value = node.required("threshold")?;
            read_thinkthen_complete_threshold(value, store).map(|value| store.hold(value))?
        },
        verb: {
            let value = node.required("verb")?;
            read_text(value, store)?
        },
        extensions: store.extensions(
            node,
            &[
                "batch",
                "context_schema",
                "fields",
                "item_schema",
                "label_details",
                "model",
                "name",
                "on",
                "profile",
                "relations",
                "threshold",
                "verb",
            ],
        )?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_related_entity {
    pub kind: thinkthen_complete_utf8,
    pub name: thinkthen_complete_utf8,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_related_entity(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_related_entity, ErrorKind> {
    Ok(thinkthen_complete_related_entity {
        kind: {
            let value = node.required("kind")?;
            read_text(value, store)?
        },
        name: {
            let value = node.required("name")?;
            read_text(value, store)?
        },
        extensions: store.extensions(node, &["kind", "name"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_related_entity_edge {
    pub either: thinkthen_complete_u32_presence,
    pub probability: f64,
    pub relation: thinkthen_complete_utf8,
    pub source: *const thinkthen_complete_relation_endpoint,
    pub target: *const thinkthen_complete_relation_endpoint,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_related_entity_edge(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_related_entity_edge, ErrorKind> {
    Ok(thinkthen_complete_related_entity_edge {
        either: {
            let (presence, value) =
                read_presence(node.member("either"), false, 0, read_boolean, store)?;
            thinkthen_complete_u32_presence { presence, value }
        },
        probability: {
            let value = node.required("probability")?;
            read_number(value, store)?
        },
        relation: {
            let value = node.required("relation")?;
            read_text(value, store)?
        },
        source: {
            let value = node.required("source")?;
            read_thinkthen_complete_relation_endpoint(value, store)
                .map(|value| store.hold(value))?
        },
        target: {
            let value = node.required("target")?;
            read_thinkthen_complete_relation_endpoint(value, store)
                .map(|value| store.hold(value))?
        },
        extensions: store.extensions(
            node,
            &["either", "probability", "relation", "source", "target"],
        )?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_relation_rule {
    pub either: u32,
    pub name: thinkthen_complete_utf8,
    pub reads: thinkthen_complete_utf8,
    pub single: thinkthen_complete_u32_presence,
    pub source: thinkthen_complete_utf8,
    pub target: thinkthen_complete_utf8,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_relation_rule(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_relation_rule, ErrorKind> {
    Ok(thinkthen_complete_relation_rule {
        either: {
            let value = node.required("either")?;
            read_boolean(value, store)?
        },
        name: {
            let value = node.required("name")?;
            read_text(value, store)?
        },
        reads: {
            let value = node.required("reads")?;
            read_text(value, store)?
        },
        single: {
            let (presence, value) =
                read_presence(node.member("single"), false, 0, read_boolean, store)?;
            thinkthen_complete_u32_presence { presence, value }
        },
        source: {
            let value = node.required("source")?;
            read_text(value, store)?
        },
        target: {
            let value = node.required("target")?;
            read_text(value, store)?
        },
        extensions: store.extensions(
            node,
            &["either", "name", "reads", "single", "source", "target"],
        )?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_annotation {
    pub name: thinkthen_complete_utf8,
    pub value: *const thinkthen_complete_annotation_value,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_session_annotation(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_annotation, ErrorKind> {
    Ok(thinkthen_complete_session_annotation {
        name: {
            let value = node.required("name")?;
            read_text(value, store)?
        },
        value: {
            let value = node.required("value")?;
            read_thinkthen_complete_annotation_value(value, store).map(|value| store.hold(value))?
        },
        extensions: store.extensions(node, &["name", "value"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_input_source {
    pub index: u64,
    pub source: *const thinkthen_complete_physical_source,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_session_input_source(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_input_source, ErrorKind> {
    Ok(thinkthen_complete_session_input_source {
        index: {
            let value = node.required("index")?;
            read_number(value, store)?
        },
        source: {
            let value = node.required("source")?;
            read_thinkthen_complete_physical_source(value, store).map(|value| store.hold(value))?
        },
        extensions: store.extensions(node, &["index", "source"])?,
    })
}

pub const THINKTHEN_COMPLETE_SESSION_JUDGMENT_DECISION: u32 = 1;

pub const THINKTHEN_COMPLETE_SESSION_JUDGMENT_CHOICE: u32 = 2;

pub const THINKTHEN_COMPLETE_SESSION_JUDGMENT_SCORE: u32 = 3;

pub const THINKTHEN_COMPLETE_SESSION_JUDGMENT_TAGS: u32 = 4;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_session_judgment_data {
    pub decision: *const thinkthen_complete_session_judgment_decision,
    pub choice: *const thinkthen_complete_session_judgment_choice,
    pub score: *const thinkthen_complete_session_judgment_score,
    pub tags: *const thinkthen_complete_session_judgment_tags,
}
impl std::fmt::Debug for thinkthen_complete_session_judgment_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("union payload")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_judgment {
    pub kind: u32,
    pub data: thinkthen_complete_session_judgment_data,
}

pub(crate) fn read_thinkthen_complete_session_judgment(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_judgment, ErrorKind> {
    if node.is_member("kind", "decision") {
        let value = read_thinkthen_complete_session_judgment_decision(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_session_judgment {
            kind: THINKTHEN_COMPLETE_SESSION_JUDGMENT_DECISION,
            data: thinkthen_complete_session_judgment_data { decision: value },
        });
    }
    if node.is_member("kind", "choice") {
        let value = read_thinkthen_complete_session_judgment_choice(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_session_judgment {
            kind: THINKTHEN_COMPLETE_SESSION_JUDGMENT_CHOICE,
            data: thinkthen_complete_session_judgment_data { choice: value },
        });
    }
    if node.is_member("kind", "score") {
        let value = read_thinkthen_complete_session_judgment_score(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_session_judgment {
            kind: THINKTHEN_COMPLETE_SESSION_JUDGMENT_SCORE,
            data: thinkthen_complete_session_judgment_data { score: value },
        });
    }
    if node.is_member("kind", "tags") {
        let value = read_thinkthen_complete_session_judgment_tags(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_session_judgment {
            kind: THINKTHEN_COMPLETE_SESSION_JUDGMENT_TAGS,
            data: thinkthen_complete_session_judgment_data { tags: value },
        });
    }
    Err(ErrorKind::Defect)
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_judgment_choice {
    pub kind: thinkthen_complete_utf8,
    pub value: thinkthen_complete_utf8_presence,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_session_judgment_choice(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_judgment_choice, ErrorKind> {
    Ok(thinkthen_complete_session_judgment_choice {
        kind: {
            let value = node.required("kind")?;
            read_text(value, store)?
        },
        value: {
            let (presence, value) = read_presence(
                node.member("value"),
                true,
                thinkthen_complete_utf8 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence { presence, value }
        },
        extensions: store.extensions(node, &["kind", "value"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_judgment_decision {
    pub kind: thinkthen_complete_utf8,
    pub value: thinkthen_complete_u32_presence,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_session_judgment_decision(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_judgment_decision, ErrorKind> {
    Ok(thinkthen_complete_session_judgment_decision {
        kind: {
            let value = node.required("kind")?;
            read_text(value, store)?
        },
        value: {
            let (presence, value) =
                read_presence(node.member("value"), true, 0, read_boolean, store)?;
            thinkthen_complete_u32_presence { presence, value }
        },
        extensions: store.extensions(node, &["kind", "value"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_judgment_score {
    pub kind: thinkthen_complete_utf8,
    pub value: f64,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_session_judgment_score(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_judgment_score, ErrorKind> {
    Ok(thinkthen_complete_session_judgment_score {
        kind: {
            let value = node.required("kind")?;
            read_text(value, store)?
        },
        value: {
            let value = node.required("value")?;
            read_number(value, store)?
        },
        extensions: store.extensions(node, &["kind", "value"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_judgment_tags {
    pub kind: thinkthen_complete_utf8,
    pub value: thinkthen_complete_string_list,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_session_judgment_tags(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_judgment_tags, ErrorKind> {
    Ok(thinkthen_complete_session_judgment_tags {
        kind: {
            let value = node.required("kind")?;
            read_text(value, store)?
        },
        value: {
            let value = node.required("value")?;
            read_thinkthen_complete_string_list(value, store)?
        },
        extensions: store.extensions(node, &["kind", "value"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_named_probability {
    pub name: thinkthen_complete_utf8,
    pub probability: f64,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_session_named_probability(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_named_probability, ErrorKind> {
    Ok(thinkthen_complete_session_named_probability {
        name: {
            let value = node.required("name")?;
            read_text(value, store)?
        },
        probability: {
            let value = node.required("probability")?;
            read_number(value, store)?
        },
        extensions: store.extensions(node, &["name", "probability"])?,
    })
}

pub const THINKTHEN_COMPLETE_SESSION_OBSERVATION_QUESTION: u32 = 1;

pub const THINKTHEN_COMPLETE_SESSION_OBSERVATION_ROW: u32 = 2;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_session_observation_data {
    pub question: *const thinkthen_complete_session_observation_question,
    pub row: *const thinkthen_complete_session_observation_row,
}
impl std::fmt::Debug for thinkthen_complete_session_observation_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("union payload")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_observation {
    pub kind: u32,
    pub data: thinkthen_complete_session_observation_data,
}

pub(crate) fn read_thinkthen_complete_session_observation(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_observation, ErrorKind> {
    if node.is_member("kind", "question") {
        let value = read_thinkthen_complete_session_observation_question(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_session_observation {
            kind: THINKTHEN_COMPLETE_SESSION_OBSERVATION_QUESTION,
            data: thinkthen_complete_session_observation_data { question: value },
        });
    }
    if node.is_member("kind", "row") {
        let value = read_thinkthen_complete_session_observation_row(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_session_observation {
            kind: THINKTHEN_COMPLETE_SESSION_OBSERVATION_ROW,
            data: thinkthen_complete_session_observation_data { row: value },
        });
    }
    Err(ErrorKind::Defect)
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_observation_question {
    pub detail: *const thinkthen_complete_session_question_detail,
    pub index: u64,
    pub kind: thinkthen_complete_utf8,
    pub member: thinkthen_complete_utf8_presence,
    pub position: u64,
    pub stage: thinkthen_complete_utf8_presence,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_session_observation_question(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_observation_question, ErrorKind> {
    Ok(thinkthen_complete_session_observation_question {
        detail: {
            let value = node.required("detail")?;
            read_thinkthen_complete_session_question_detail(value, store)
                .map(|value| store.hold(value))?
        },
        index: {
            let value = node.required("index")?;
            read_number(value, store)?
        },
        kind: {
            let value = node.required("kind")?;
            read_text(value, store)?
        },
        member: {
            let (presence, value) = read_presence(
                node.member("member"),
                false,
                thinkthen_complete_utf8 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence { presence, value }
        },
        position: {
            let value = node.required("position")?;
            read_number(value, store)?
        },
        stage: {
            let (presence, value) = read_presence(
                node.member("stage"),
                false,
                thinkthen_complete_utf8 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence { presence, value }
        },
        extensions: store.extensions(
            node,
            &["detail", "index", "kind", "member", "position", "stage"],
        )?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_observation_row {
    pub index: u64,
    pub kind: thinkthen_complete_utf8,
    pub value: *const thinkthen_complete_session_observed_row,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_session_observation_row(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_observation_row, ErrorKind> {
    Ok(thinkthen_complete_session_observation_row {
        index: {
            let value = node.required("index")?;
            read_number(value, store)?
        },
        kind: {
            let value = node.required("kind")?;
            read_text(value, store)?
        },
        value: {
            let value = node.required("value")?;
            read_thinkthen_complete_session_observed_row(value, store)
                .map(|value| store.hold(value))?
        },
        extensions: store.extensions(node, &["index", "kind", "value"])?,
    })
}

pub const THINKTHEN_COMPLETE_SESSION_OBSERVED_ROW_JUDGMENT: u32 = 1;

pub const THINKTHEN_COMPLETE_SESSION_OBSERVED_ROW_ANNOTATED: u32 = 2;

pub const THINKTHEN_COMPLETE_SESSION_OBSERVED_ROW_RECOGNIZED: u32 = 3;

pub const THINKTHEN_COMPLETE_SESSION_OBSERVED_ROW_FIND: u32 = 4;

pub const THINKTHEN_COMPLETE_SESSION_OBSERVED_ROW_RELATIONS: u32 = 5;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_session_observed_row_data {
    pub judgment: *const thinkthen_complete_session_observed_row_judgment,
    pub annotated: *const thinkthen_complete_session_observed_row_annotated,
    pub recognized: *const thinkthen_complete_session_observed_row_recognized,
    pub find: *const thinkthen_complete_session_observed_row_find,
    pub relations: *const thinkthen_complete_session_observed_row_relations,
}
impl std::fmt::Debug for thinkthen_complete_session_observed_row_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("union payload")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_observed_row {
    pub kind: u32,
    pub data: thinkthen_complete_session_observed_row_data,
}

pub(crate) fn read_thinkthen_complete_session_observed_row(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_observed_row, ErrorKind> {
    if node.is_member("kind", "judgment") {
        let value = read_thinkthen_complete_session_observed_row_judgment(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_session_observed_row {
            kind: THINKTHEN_COMPLETE_SESSION_OBSERVED_ROW_JUDGMENT,
            data: thinkthen_complete_session_observed_row_data { judgment: value },
        });
    }
    if node.is_member("kind", "annotated") {
        let value = read_thinkthen_complete_session_observed_row_annotated(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_session_observed_row {
            kind: THINKTHEN_COMPLETE_SESSION_OBSERVED_ROW_ANNOTATED,
            data: thinkthen_complete_session_observed_row_data { annotated: value },
        });
    }
    if node.is_member("kind", "recognized") {
        let value = read_thinkthen_complete_session_observed_row_recognized(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_session_observed_row {
            kind: THINKTHEN_COMPLETE_SESSION_OBSERVED_ROW_RECOGNIZED,
            data: thinkthen_complete_session_observed_row_data { recognized: value },
        });
    }
    if node.is_member("kind", "find") {
        let value = read_thinkthen_complete_session_observed_row_find(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_session_observed_row {
            kind: THINKTHEN_COMPLETE_SESSION_OBSERVED_ROW_FIND,
            data: thinkthen_complete_session_observed_row_data { find: value },
        });
    }
    if node.is_member("kind", "relations") {
        let value = read_thinkthen_complete_session_observed_row_relations(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_session_observed_row {
            kind: THINKTHEN_COMPLETE_SESSION_OBSERVED_ROW_RELATIONS,
            data: thinkthen_complete_session_observed_row_data { relations: value },
        });
    }
    Err(ErrorKind::Defect)
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_annotation_list {
    pub data: *const *const thinkthen_complete_session_annotation,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_session_annotation_list(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_annotation_list, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_session_annotation(item, store)
                .map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_session_annotation_list { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_observed_row_annotated {
    pub kind: thinkthen_complete_utf8,
    pub value: thinkthen_complete_session_annotation_list,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_session_observed_row_annotated(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_observed_row_annotated, ErrorKind> {
    Ok(thinkthen_complete_session_observed_row_annotated {
        kind: {
            let value = node.required("kind")?;
            read_text(value, store)?
        },
        value: {
            let value = node.required("value")?;
            read_thinkthen_complete_session_annotation_list(value, store)?
        },
        extensions: store.extensions(node, &["kind", "value"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_observed_row_find {
    pub kind: thinkthen_complete_utf8,
    pub value: thinkthen_complete_u64_presence,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_session_observed_row_find(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_observed_row_find, ErrorKind> {
    Ok(thinkthen_complete_session_observed_row_find {
        kind: {
            let value = node.required("kind")?;
            read_text(value, store)?
        },
        value: {
            let (presence, value) =
                read_presence(node.member("value"), true, 0, read_number, store)?;
            thinkthen_complete_u64_presence { presence, value }
        },
        extensions: store.extensions(node, &["kind", "value"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_observed_row_judgment {
    pub kind: thinkthen_complete_utf8,
    pub value: *const thinkthen_complete_session_judgment,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_session_observed_row_judgment(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_observed_row_judgment, ErrorKind> {
    Ok(thinkthen_complete_session_observed_row_judgment {
        kind: {
            let value = node.required("kind")?;
            read_text(value, store)?
        },
        value: {
            let value = node.required("value")?;
            read_thinkthen_complete_session_judgment(value, store).map(|value| store.hold(value))?
        },
        extensions: store.extensions(node, &["kind", "value"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_observed_row_recognized {
    pub kind: thinkthen_complete_utf8,
    pub value: *const thinkthen_complete_session_recognition,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_session_observed_row_recognized(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_observed_row_recognized, ErrorKind> {
    Ok(thinkthen_complete_session_observed_row_recognized {
        kind: {
            let value = node.required("kind")?;
            read_text(value, store)?
        },
        value: {
            let value = node.required("value")?;
            read_thinkthen_complete_session_recognition(value, store)
                .map(|value| store.hold(value))?
        },
        extensions: store.extensions(node, &["kind", "value"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_relation_edge_list {
    pub data: *const *const thinkthen_complete_session_relation_edge,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_session_relation_edge_list(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_relation_edge_list, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_session_relation_edge(item, store)
                .map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_session_relation_edge_list { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_observed_row_relations {
    pub kind: thinkthen_complete_utf8,
    pub value: thinkthen_complete_session_relation_edge_list,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_session_observed_row_relations(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_observed_row_relations, ErrorKind> {
    Ok(thinkthen_complete_session_observed_row_relations {
        kind: {
            let value = node.required("kind")?;
            read_text(value, store)?
        },
        value: {
            let value = node.required("value")?;
            read_thinkthen_complete_session_relation_edge_list(value, store)?
        },
        extensions: store.extensions(node, &["kind", "value"])?,
    })
}

pub const THINKTHEN_COMPLETE_SESSION_PACKET_DECIDE_ROW: u32 = 1;

pub const THINKTHEN_COMPLETE_SESSION_PACKET_CHOOSE_ROW: u32 = 2;

pub const THINKTHEN_COMPLETE_SESSION_PACKET_TAG_ROW: u32 = 3;

pub const THINKTHEN_COMPLETE_SESSION_PACKET_SCORE_ROW: u32 = 4;

pub const THINKTHEN_COMPLETE_SESSION_PACKET_FILTER_ROW: u32 = 5;

pub const THINKTHEN_COMPLETE_SESSION_PACKET_ANNOTATE_ROW: u32 = 6;

pub const THINKTHEN_COMPLETE_SESSION_PACKET_DECIDE_AGGREGATE: u32 = 7;

pub const THINKTHEN_COMPLETE_SESSION_PACKET_CHOOSE_AGGREGATE: u32 = 8;

pub const THINKTHEN_COMPLETE_SESSION_PACKET_TAG_AGGREGATE: u32 = 9;

pub const THINKTHEN_COMPLETE_SESSION_PACKET_SCORE_AGGREGATE: u32 = 10;

pub const THINKTHEN_COMPLETE_SESSION_PACKET_FILTER_AGGREGATE: u32 = 11;

pub const THINKTHEN_COMPLETE_SESSION_PACKET_RANK_AGGREGATE: u32 = 12;

pub const THINKTHEN_COMPLETE_SESSION_PACKET_FIND_AGGREGATE: u32 = 13;

pub const THINKTHEN_COMPLETE_SESSION_PACKET_ANNOTATE_AGGREGATE: u32 = 14;

pub const THINKTHEN_COMPLETE_SESSION_PACKET_RECOGNIZE_AGGREGATE: u32 = 15;

pub const THINKTHEN_COMPLETE_SESSION_PACKET_RELATE_AGGREGATE: u32 = 16;

pub const THINKTHEN_COMPLETE_SESSION_PACKET_OBSERVATION: u32 = 17;

pub const THINKTHEN_COMPLETE_SESSION_PACKET_TERMINAL: u32 = 18;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_session_packet_data {
    pub decide_row: *const thinkthen_complete_session_packet_decide_row,
    pub choose_row: *const thinkthen_complete_session_packet_choose_row,
    pub tag_row: *const thinkthen_complete_session_packet_tag_row,
    pub score_row: *const thinkthen_complete_session_packet_score_row,
    pub filter_row: *const thinkthen_complete_session_packet_filter_row,
    pub annotate_row: *const thinkthen_complete_session_packet_annotate_row,
    pub decide_aggregate: *const thinkthen_complete_session_packet_decide_aggregate,
    pub choose_aggregate: *const thinkthen_complete_session_packet_choose_aggregate,
    pub tag_aggregate: *const thinkthen_complete_session_packet_tag_aggregate,
    pub score_aggregate: *const thinkthen_complete_session_packet_score_aggregate,
    pub filter_aggregate: *const thinkthen_complete_session_packet_filter_aggregate,
    pub rank_aggregate: *const thinkthen_complete_session_packet_rank_aggregate,
    pub find_aggregate: *const thinkthen_complete_session_packet_find_aggregate,
    pub annotate_aggregate: *const thinkthen_complete_session_packet_annotate_aggregate,
    pub recognize_aggregate: *const thinkthen_complete_session_packet_recognize_aggregate,
    pub relate_aggregate: *const thinkthen_complete_session_packet_relate_aggregate,
    pub observation: *const thinkthen_complete_session_packet_observation,
    pub terminal: *const thinkthen_complete_session_packet_terminal,
}
impl std::fmt::Debug for thinkthen_complete_session_packet_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("union payload")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_packet {
    pub kind: u32,
    pub data: thinkthen_complete_session_packet_data,
}

pub(crate) fn read_thinkthen_complete_session_packet(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_packet, ErrorKind> {
    if node.is_member("function", "decide") && node.is_member("kind", "row") {
        let value = read_thinkthen_complete_session_packet_decide_row(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_session_packet {
            kind: THINKTHEN_COMPLETE_SESSION_PACKET_DECIDE_ROW,
            data: thinkthen_complete_session_packet_data { decide_row: value },
        });
    }
    if node.is_member("function", "choose") && node.is_member("kind", "row") {
        let value = read_thinkthen_complete_session_packet_choose_row(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_session_packet {
            kind: THINKTHEN_COMPLETE_SESSION_PACKET_CHOOSE_ROW,
            data: thinkthen_complete_session_packet_data { choose_row: value },
        });
    }
    if node.is_member("function", "tag") && node.is_member("kind", "row") {
        let value = read_thinkthen_complete_session_packet_tag_row(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_session_packet {
            kind: THINKTHEN_COMPLETE_SESSION_PACKET_TAG_ROW,
            data: thinkthen_complete_session_packet_data { tag_row: value },
        });
    }
    if node.is_member("function", "score") && node.is_member("kind", "row") {
        let value = read_thinkthen_complete_session_packet_score_row(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_session_packet {
            kind: THINKTHEN_COMPLETE_SESSION_PACKET_SCORE_ROW,
            data: thinkthen_complete_session_packet_data { score_row: value },
        });
    }
    if node.is_member("function", "filter") && node.is_member("kind", "row") {
        let value = read_thinkthen_complete_session_packet_filter_row(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_session_packet {
            kind: THINKTHEN_COMPLETE_SESSION_PACKET_FILTER_ROW,
            data: thinkthen_complete_session_packet_data { filter_row: value },
        });
    }
    if node.is_member("function", "annotate") && node.is_member("kind", "row") {
        let value = read_thinkthen_complete_session_packet_annotate_row(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_session_packet {
            kind: THINKTHEN_COMPLETE_SESSION_PACKET_ANNOTATE_ROW,
            data: thinkthen_complete_session_packet_data {
                annotate_row: value,
            },
        });
    }
    if node.is_member("function", "decide") && node.is_member("kind", "aggregate") {
        let value = read_thinkthen_complete_session_packet_decide_aggregate(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_session_packet {
            kind: THINKTHEN_COMPLETE_SESSION_PACKET_DECIDE_AGGREGATE,
            data: thinkthen_complete_session_packet_data {
                decide_aggregate: value,
            },
        });
    }
    if node.is_member("function", "choose") && node.is_member("kind", "aggregate") {
        let value = read_thinkthen_complete_session_packet_choose_aggregate(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_session_packet {
            kind: THINKTHEN_COMPLETE_SESSION_PACKET_CHOOSE_AGGREGATE,
            data: thinkthen_complete_session_packet_data {
                choose_aggregate: value,
            },
        });
    }
    if node.is_member("function", "tag") && node.is_member("kind", "aggregate") {
        let value = read_thinkthen_complete_session_packet_tag_aggregate(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_session_packet {
            kind: THINKTHEN_COMPLETE_SESSION_PACKET_TAG_AGGREGATE,
            data: thinkthen_complete_session_packet_data {
                tag_aggregate: value,
            },
        });
    }
    if node.is_member("function", "score") && node.is_member("kind", "aggregate") {
        let value = read_thinkthen_complete_session_packet_score_aggregate(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_session_packet {
            kind: THINKTHEN_COMPLETE_SESSION_PACKET_SCORE_AGGREGATE,
            data: thinkthen_complete_session_packet_data {
                score_aggregate: value,
            },
        });
    }
    if node.is_member("function", "filter") && node.is_member("kind", "aggregate") {
        let value = read_thinkthen_complete_session_packet_filter_aggregate(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_session_packet {
            kind: THINKTHEN_COMPLETE_SESSION_PACKET_FILTER_AGGREGATE,
            data: thinkthen_complete_session_packet_data {
                filter_aggregate: value,
            },
        });
    }
    if node.is_member("function", "rank") && node.is_member("kind", "aggregate") {
        let value = read_thinkthen_complete_session_packet_rank_aggregate(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_session_packet {
            kind: THINKTHEN_COMPLETE_SESSION_PACKET_RANK_AGGREGATE,
            data: thinkthen_complete_session_packet_data {
                rank_aggregate: value,
            },
        });
    }
    if node.is_member("function", "find") && node.is_member("kind", "aggregate") {
        let value = read_thinkthen_complete_session_packet_find_aggregate(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_session_packet {
            kind: THINKTHEN_COMPLETE_SESSION_PACKET_FIND_AGGREGATE,
            data: thinkthen_complete_session_packet_data {
                find_aggregate: value,
            },
        });
    }
    if node.is_member("function", "annotate") && node.is_member("kind", "aggregate") {
        let value = read_thinkthen_complete_session_packet_annotate_aggregate(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_session_packet {
            kind: THINKTHEN_COMPLETE_SESSION_PACKET_ANNOTATE_AGGREGATE,
            data: thinkthen_complete_session_packet_data {
                annotate_aggregate: value,
            },
        });
    }
    if node.is_member("function", "recognize") && node.is_member("kind", "aggregate") {
        let value = read_thinkthen_complete_session_packet_recognize_aggregate(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_session_packet {
            kind: THINKTHEN_COMPLETE_SESSION_PACKET_RECOGNIZE_AGGREGATE,
            data: thinkthen_complete_session_packet_data {
                recognize_aggregate: value,
            },
        });
    }
    if node.is_member("function", "relate") && node.is_member("kind", "aggregate") {
        let value = read_thinkthen_complete_session_packet_relate_aggregate(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_session_packet {
            kind: THINKTHEN_COMPLETE_SESSION_PACKET_RELATE_AGGREGATE,
            data: thinkthen_complete_session_packet_data {
                relate_aggregate: value,
            },
        });
    }
    if node.is_member("kind", "observation") {
        let value = read_thinkthen_complete_session_packet_observation(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_session_packet {
            kind: THINKTHEN_COMPLETE_SESSION_PACKET_OBSERVATION,
            data: thinkthen_complete_session_packet_data { observation: value },
        });
    }
    if node.is_member("kind", "terminal") {
        let value = read_thinkthen_complete_session_packet_terminal(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_session_packet {
            kind: THINKTHEN_COMPLETE_SESSION_PACKET_TERMINAL,
            data: thinkthen_complete_session_packet_data { terminal: value },
        });
    }
    Err(ErrorKind::Defect)
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_annotation_list {
    pub data: *const *const thinkthen_complete_annotation,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_annotation_list(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_annotation_list, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values
            .push(read_thinkthen_complete_annotation(item, store).map(|value| store.hold(value))?);
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_annotation_list { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_packet_annotate_aggregate {
    pub function: thinkthen_complete_utf8,
    pub kind: thinkthen_complete_utf8,
    pub value: thinkthen_complete_annotation_list,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_session_packet_annotate_aggregate(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_packet_annotate_aggregate, ErrorKind> {
    Ok(thinkthen_complete_session_packet_annotate_aggregate {
        function: {
            let value = node.required("function")?;
            read_text(value, store)?
        },
        kind: {
            let value = node.required("kind")?;
            read_text(value, store)?
        },
        value: {
            let value = node.required("value")?;
            read_thinkthen_complete_annotation_list(value, store)?
        },
        extensions: store.extensions(node, &["function", "kind", "value"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_packet_annotate_row {
    pub function: thinkthen_complete_utf8,
    pub kind: thinkthen_complete_utf8,
    pub value: *const thinkthen_complete_annotation,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_session_packet_annotate_row(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_packet_annotate_row, ErrorKind> {
    Ok(thinkthen_complete_session_packet_annotate_row {
        function: {
            let value = node.required("function")?;
            read_text(value, store)?
        },
        kind: {
            let value = node.required("kind")?;
            read_text(value, store)?
        },
        value: {
            let value = node.required("value")?;
            read_thinkthen_complete_annotation(value, store).map(|value| store.hold(value))?
        },
        extensions: store.extensions(node, &["function", "kind", "value"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_atomic_nullable_string_list {
    pub data: *const *const thinkthen_complete_atomic_nullable_string,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_atomic_nullable_string_list(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_atomic_nullable_string_list, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_atomic_nullable_string(item, store)
                .map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_atomic_nullable_string_list { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_packet_choose_aggregate {
    pub function: thinkthen_complete_utf8,
    pub kind: thinkthen_complete_utf8,
    pub value: thinkthen_complete_atomic_nullable_string_list,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_session_packet_choose_aggregate(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_packet_choose_aggregate, ErrorKind> {
    Ok(thinkthen_complete_session_packet_choose_aggregate {
        function: {
            let value = node.required("function")?;
            read_text(value, store)?
        },
        kind: {
            let value = node.required("kind")?;
            read_text(value, store)?
        },
        value: {
            let value = node.required("value")?;
            read_thinkthen_complete_atomic_nullable_string_list(value, store)?
        },
        extensions: store.extensions(node, &["function", "kind", "value"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_packet_choose_row {
    pub function: thinkthen_complete_utf8,
    pub kind: thinkthen_complete_utf8,
    pub value: *const thinkthen_complete_atomic_nullable_string,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_session_packet_choose_row(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_packet_choose_row, ErrorKind> {
    Ok(thinkthen_complete_session_packet_choose_row {
        function: {
            let value = node.required("function")?;
            read_text(value, store)?
        },
        kind: {
            let value = node.required("kind")?;
            read_text(value, store)?
        },
        value: {
            let value = node.required("value")?;
            read_thinkthen_complete_atomic_nullable_string(value, store)
                .map(|value| store.hold(value))?
        },
        extensions: store.extensions(node, &["function", "kind", "value"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_atomic_decide_value_list {
    pub data: *const *const thinkthen_complete_atomic_decide_value,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_atomic_decide_value_list(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_atomic_decide_value_list, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_atomic_decide_value(item, store)
                .map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_atomic_decide_value_list { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_packet_decide_aggregate {
    pub function: thinkthen_complete_utf8,
    pub kind: thinkthen_complete_utf8,
    pub value: thinkthen_complete_atomic_decide_value_list,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_session_packet_decide_aggregate(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_packet_decide_aggregate, ErrorKind> {
    Ok(thinkthen_complete_session_packet_decide_aggregate {
        function: {
            let value = node.required("function")?;
            read_text(value, store)?
        },
        kind: {
            let value = node.required("kind")?;
            read_text(value, store)?
        },
        value: {
            let value = node.required("value")?;
            read_thinkthen_complete_atomic_decide_value_list(value, store)?
        },
        extensions: store.extensions(node, &["function", "kind", "value"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_packet_decide_row {
    pub function: thinkthen_complete_utf8,
    pub kind: thinkthen_complete_utf8,
    pub value: *const thinkthen_complete_atomic_decide_value,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_session_packet_decide_row(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_packet_decide_row, ErrorKind> {
    Ok(thinkthen_complete_session_packet_decide_row {
        function: {
            let value = node.required("function")?;
            read_text(value, store)?
        },
        kind: {
            let value = node.required("kind")?;
            read_text(value, store)?
        },
        value: {
            let value = node.required("value")?;
            read_thinkthen_complete_atomic_decide_value(value, store)
                .map(|value| store.hold(value))?
        },
        extensions: store.extensions(node, &["function", "kind", "value"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_atomic_boolean_list {
    pub data: *const *const thinkthen_complete_atomic_boolean,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_atomic_boolean_list(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_atomic_boolean_list, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_atomic_boolean(item, store).map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_atomic_boolean_list { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_packet_filter_aggregate {
    pub function: thinkthen_complete_utf8,
    pub kind: thinkthen_complete_utf8,
    pub value: thinkthen_complete_atomic_boolean_list,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_session_packet_filter_aggregate(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_packet_filter_aggregate, ErrorKind> {
    Ok(thinkthen_complete_session_packet_filter_aggregate {
        function: {
            let value = node.required("function")?;
            read_text(value, store)?
        },
        kind: {
            let value = node.required("kind")?;
            read_text(value, store)?
        },
        value: {
            let value = node.required("value")?;
            read_thinkthen_complete_atomic_boolean_list(value, store)?
        },
        extensions: store.extensions(node, &["function", "kind", "value"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_packet_filter_row {
    pub function: thinkthen_complete_utf8,
    pub kind: thinkthen_complete_utf8,
    pub value: *const thinkthen_complete_atomic_boolean,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_session_packet_filter_row(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_packet_filter_row, ErrorKind> {
    Ok(thinkthen_complete_session_packet_filter_row {
        function: {
            let value = node.required("function")?;
            read_text(value, store)?
        },
        kind: {
            let value = node.required("kind")?;
            read_text(value, store)?
        },
        value: {
            let value = node.required("value")?;
            read_thinkthen_complete_atomic_boolean(value, store).map(|value| store.hold(value))?
        },
        extensions: store.extensions(node, &["function", "kind", "value"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_packet_find_aggregate {
    pub function: thinkthen_complete_utf8,
    pub kind: thinkthen_complete_utf8,
    pub value: *const thinkthen_complete_find,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_session_packet_find_aggregate(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_packet_find_aggregate, ErrorKind> {
    Ok(thinkthen_complete_session_packet_find_aggregate {
        function: {
            let value = node.required("function")?;
            read_text(value, store)?
        },
        kind: {
            let value = node.required("kind")?;
            read_text(value, store)?
        },
        value: {
            let value = node.required("value")?;
            read_thinkthen_complete_find(value, store).map(|value| store.hold(value))?
        },
        extensions: store.extensions(node, &["function", "kind", "value"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_packet_observation {
    pub function: *const thinkthen_complete_request_function,
    pub kind: thinkthen_complete_utf8,
    pub value: *const thinkthen_complete_session_observation,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_session_packet_observation(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_packet_observation, ErrorKind> {
    Ok(thinkthen_complete_session_packet_observation {
        function: {
            let value = node.required("function")?;
            read_thinkthen_complete_request_function(value, store).map(|value| store.hold(value))?
        },
        kind: {
            let value = node.required("kind")?;
            read_text(value, store)?
        },
        value: {
            let value = node.required("value")?;
            read_thinkthen_complete_session_observation(value, store)
                .map(|value| store.hold(value))?
        },
        extensions: store.extensions(node, &["function", "kind", "value"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_atomic_non_zero_usize_list {
    pub data: *const *const thinkthen_complete_atomic_non_zero_usize,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_atomic_non_zero_usize_list(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_atomic_non_zero_usize_list, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_atomic_non_zero_usize(item, store)
                .map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_atomic_non_zero_usize_list { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_packet_rank_aggregate {
    pub function: thinkthen_complete_utf8,
    pub kind: thinkthen_complete_utf8,
    pub value: thinkthen_complete_atomic_non_zero_usize_list,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_session_packet_rank_aggregate(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_packet_rank_aggregate, ErrorKind> {
    Ok(thinkthen_complete_session_packet_rank_aggregate {
        function: {
            let value = node.required("function")?;
            read_text(value, store)?
        },
        kind: {
            let value = node.required("kind")?;
            read_text(value, store)?
        },
        value: {
            let value = node.required("value")?;
            read_thinkthen_complete_atomic_non_zero_usize_list(value, store)?
        },
        extensions: store.extensions(node, &["function", "kind", "value"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_recognition_list {
    pub data: *const *const thinkthen_complete_recognition,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_recognition_list(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_recognition_list, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values
            .push(read_thinkthen_complete_recognition(item, store).map(|value| store.hold(value))?);
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_recognition_list { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_packet_recognize_aggregate {
    pub function: thinkthen_complete_utf8,
    pub kind: thinkthen_complete_utf8,
    pub value: thinkthen_complete_recognition_list,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_session_packet_recognize_aggregate(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_packet_recognize_aggregate, ErrorKind> {
    Ok(thinkthen_complete_session_packet_recognize_aggregate {
        function: {
            let value = node.required("function")?;
            read_text(value, store)?
        },
        kind: {
            let value = node.required("kind")?;
            read_text(value, store)?
        },
        value: {
            let value = node.required("value")?;
            read_thinkthen_complete_recognition_list(value, store)?
        },
        extensions: store.extensions(node, &["function", "kind", "value"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_packet_relate_aggregate {
    pub function: thinkthen_complete_utf8,
    pub kind: thinkthen_complete_utf8,
    pub value: *const thinkthen_complete_relation,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_session_packet_relate_aggregate(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_packet_relate_aggregate, ErrorKind> {
    Ok(thinkthen_complete_session_packet_relate_aggregate {
        function: {
            let value = node.required("function")?;
            read_text(value, store)?
        },
        kind: {
            let value = node.required("kind")?;
            read_text(value, store)?
        },
        value: {
            let value = node.required("value")?;
            read_thinkthen_complete_relation(value, store).map(|value| store.hold(value))?
        },
        extensions: store.extensions(node, &["function", "kind", "value"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_atomic_double_list {
    pub data: *const *const thinkthen_complete_atomic_double,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_atomic_double_list(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_atomic_double_list, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_atomic_double(item, store).map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_atomic_double_list { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_packet_score_aggregate {
    pub function: thinkthen_complete_utf8,
    pub kind: thinkthen_complete_utf8,
    pub value: thinkthen_complete_atomic_double_list,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_session_packet_score_aggregate(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_packet_score_aggregate, ErrorKind> {
    Ok(thinkthen_complete_session_packet_score_aggregate {
        function: {
            let value = node.required("function")?;
            read_text(value, store)?
        },
        kind: {
            let value = node.required("kind")?;
            read_text(value, store)?
        },
        value: {
            let value = node.required("value")?;
            read_thinkthen_complete_atomic_double_list(value, store)?
        },
        extensions: store.extensions(node, &["function", "kind", "value"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_packet_score_row {
    pub function: thinkthen_complete_utf8,
    pub kind: thinkthen_complete_utf8,
    pub value: *const thinkthen_complete_atomic_double,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_session_packet_score_row(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_packet_score_row, ErrorKind> {
    Ok(thinkthen_complete_session_packet_score_row {
        function: {
            let value = node.required("function")?;
            read_text(value, store)?
        },
        kind: {
            let value = node.required("kind")?;
            read_text(value, store)?
        },
        value: {
            let value = node.required("value")?;
            read_thinkthen_complete_atomic_double(value, store).map(|value| store.hold(value))?
        },
        extensions: store.extensions(node, &["function", "kind", "value"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_atomic_array_of_string_list {
    pub data: *const *const thinkthen_complete_atomic_array_of_string,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_atomic_array_of_string_list(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_atomic_array_of_string_list, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_atomic_array_of_string(item, store)
                .map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_atomic_array_of_string_list { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_packet_tag_aggregate {
    pub function: thinkthen_complete_utf8,
    pub kind: thinkthen_complete_utf8,
    pub value: thinkthen_complete_atomic_array_of_string_list,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_session_packet_tag_aggregate(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_packet_tag_aggregate, ErrorKind> {
    Ok(thinkthen_complete_session_packet_tag_aggregate {
        function: {
            let value = node.required("function")?;
            read_text(value, store)?
        },
        kind: {
            let value = node.required("kind")?;
            read_text(value, store)?
        },
        value: {
            let value = node.required("value")?;
            read_thinkthen_complete_atomic_array_of_string_list(value, store)?
        },
        extensions: store.extensions(node, &["function", "kind", "value"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_packet_tag_row {
    pub function: thinkthen_complete_utf8,
    pub kind: thinkthen_complete_utf8,
    pub value: *const thinkthen_complete_atomic_array_of_string,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_session_packet_tag_row(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_packet_tag_row, ErrorKind> {
    Ok(thinkthen_complete_session_packet_tag_row {
        function: {
            let value = node.required("function")?;
            read_text(value, store)?
        },
        kind: {
            let value = node.required("kind")?;
            read_text(value, store)?
        },
        value: {
            let value = node.required("value")?;
            read_thinkthen_complete_atomic_array_of_string(value, store)
                .map(|value| store.hold(value))?
        },
        extensions: store.extensions(node, &["function", "kind", "value"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_settled_answer_presence {
    pub presence: u32,
    pub value: *const thinkthen_complete_settled_answer,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_call_error_presence {
    pub presence: u32,
    pub value: *const thinkthen_complete_call_error,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_packet_terminal {
    pub answer: thinkthen_complete_settled_answer_presence,
    pub facts: thinkthen_complete_facts_presence,
    pub failure: thinkthen_complete_call_error_presence,
    pub kind: thinkthen_complete_utf8,
    pub resolution: *const thinkthen_complete_answer_resolution,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_session_packet_terminal(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_packet_terminal, ErrorKind> {
    Ok(thinkthen_complete_session_packet_terminal {
        answer: {
            let (presence, value) = read_presence(
                node.member("answer"),
                true,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_settled_answer(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_settled_answer_presence { presence, value }
        },
        facts: {
            let (presence, value) = read_presence(
                node.member("facts"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_facts(value, store).map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_facts_presence { presence, value }
        },
        failure: {
            let (presence, value) = read_presence(
                node.member("failure"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_call_error(value, store).map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_call_error_presence { presence, value }
        },
        kind: {
            let value = node.required("kind")?;
            read_text(value, store)?
        },
        resolution: {
            let value = node.required("resolution")?;
            read_thinkthen_complete_answer_resolution(value, store)
                .map(|value| store.hold(value))?
        },
        extensions: store
            .extensions(node, &["answer", "facts", "failure", "kind", "resolution"])?,
    })
}

pub const THINKTHEN_COMPLETE_SESSION_PROBABILITIES_YES_NO: u32 = 1;

pub const THINKTHEN_COMPLETE_SESSION_PROBABILITIES_NAMED: u32 = 2;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_session_probabilities_data {
    pub yes_no: *const thinkthen_complete_session_probabilities_yes_no,
    pub named: *const thinkthen_complete_session_probabilities_named,
}
impl std::fmt::Debug for thinkthen_complete_session_probabilities_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("union payload")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_probabilities {
    pub kind: u32,
    pub data: thinkthen_complete_session_probabilities_data,
}

pub(crate) fn read_thinkthen_complete_session_probabilities(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_probabilities, ErrorKind> {
    if node.is_member("kind", "yes_no") {
        let value = read_thinkthen_complete_session_probabilities_yes_no(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_session_probabilities {
            kind: THINKTHEN_COMPLETE_SESSION_PROBABILITIES_YES_NO,
            data: thinkthen_complete_session_probabilities_data { yes_no: value },
        });
    }
    if node.is_member("kind", "named") {
        let value = read_thinkthen_complete_session_probabilities_named(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_session_probabilities {
            kind: THINKTHEN_COMPLETE_SESSION_PROBABILITIES_NAMED,
            data: thinkthen_complete_session_probabilities_data { named: value },
        });
    }
    Err(ErrorKind::Defect)
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_named_probability_list {
    pub data: *const *const thinkthen_complete_session_named_probability,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_session_named_probability_list(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_named_probability_list, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_session_named_probability(item, store)
                .map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_session_named_probability_list { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_probabilities_named {
    pub kind: thinkthen_complete_utf8,
    pub value: thinkthen_complete_session_named_probability_list,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_session_probabilities_named(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_probabilities_named, ErrorKind> {
    Ok(thinkthen_complete_session_probabilities_named {
        kind: {
            let value = node.required("kind")?;
            read_text(value, store)?
        },
        value: {
            let value = node.required("value")?;
            read_thinkthen_complete_session_named_probability_list(value, store)?
        },
        extensions: store.extensions(node, &["kind", "value"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_probabilities_yes_no {
    pub kind: thinkthen_complete_utf8,
    pub value: f64,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_session_probabilities_yes_no(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_probabilities_yes_no, ErrorKind> {
    Ok(thinkthen_complete_session_probabilities_yes_no {
        kind: {
            let value = node.required("kind")?;
            read_text(value, store)?
        },
        value: {
            let value = node.required("value")?;
            read_number(value, store)?
        },
        extensions: store.extensions(node, &["kind", "value"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answer_id_presence {
    pub presence: u32,
    pub value: *const thinkthen_complete_answer_id,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_failure_presence {
    pub presence: u32,
    pub value: *const thinkthen_complete_failure,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_failure_id_presence {
    pub presence: u32,
    pub value: *const thinkthen_complete_failure_id,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_json_list {
    pub data: *const *const thinkthen_complete_json,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_json_list(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_json_list, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(read_json(item, store)?);
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_json_list { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_probabilities_presence {
    pub presence: u32,
    pub value: *const thinkthen_complete_session_probabilities,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_token_usage_presence {
    pub presence: u32,
    pub value: *const thinkthen_complete_token_usage,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_question_detail {
    pub answer_id: thinkthen_complete_answer_id_presence,
    pub cached: u32,
    pub confidence: thinkthen_complete_f64_presence,
    pub failed_questions: u64,
    pub failure: thinkthen_complete_failure_presence,
    pub failure_id: thinkthen_complete_failure_id_presence,
    pub input: thinkthen_complete_json_presence,
    pub input_source: thinkthen_complete_physical_source_presence,
    pub input_sources: thinkthen_complete_session_input_source_list,
    pub inputs: thinkthen_complete_json_list,
    pub model: thinkthen_complete_utf8,
    pub observations: thinkthen_complete_observation_list,
    pub probabilities: thinkthen_complete_session_probabilities_presence,
    pub question: *const thinkthen_complete_question_document,
    pub question_sha256: thinkthen_complete_utf8,
    pub question_sources: thinkthen_complete_question_source_list,
    pub raw_pick: thinkthen_complete_utf8_presence,
    pub reported_usage: thinkthen_complete_usage_presence,
    pub requests: thinkthen_complete_string_list,
    pub requests_sent: u64,
    pub threshold: thinkthen_complete_threshold_presence,
    pub url: thinkthen_complete_utf8,
    pub usage: thinkthen_complete_token_usage_presence,
    pub value: thinkthen_complete_value_presence,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_session_question_detail(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_question_detail, ErrorKind> {
    Ok(thinkthen_complete_session_question_detail {
        answer_id: {
            let (presence, value) = read_presence(
                node.member("answer_id"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_answer_id(value, store).map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_answer_id_presence { presence, value }
        },
        cached: {
            let value = node.required("cached")?;
            read_boolean(value, store)?
        },
        confidence: {
            let (presence, value) =
                read_presence(node.member("confidence"), false, 0.0, read_number, store)?;
            thinkthen_complete_f64_presence { presence, value }
        },
        failed_questions: {
            let value = node.required("failed_questions")?;
            read_number(value, store)?
        },
        failure: {
            let (presence, value) = read_presence(
                node.member("failure"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_failure(value, store).map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_failure_presence { presence, value }
        },
        failure_id: {
            let (presence, value) = read_presence(
                node.member("failure_id"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_failure_id(value, store).map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_failure_id_presence { presence, value }
        },
        input: {
            let (presence, value) = read_presence(
                node.member("input"),
                false,
                std::ptr::null(),
                read_json,
                store,
            )?;
            thinkthen_complete_json_presence { presence, value }
        },
        input_source: {
            let (presence, value) = read_presence(
                node.member("input_source"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_physical_source(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_physical_source_presence { presence, value }
        },
        input_sources: {
            let value = node.required("input_sources")?;
            read_thinkthen_complete_session_input_source_list(value, store)?
        },
        inputs: {
            let value = node.required("inputs")?;
            read_thinkthen_complete_json_list(value, store)?
        },
        model: {
            let value = node.required("model")?;
            read_text(value, store)?
        },
        observations: {
            let value = node.required("observations")?;
            read_thinkthen_complete_observation_list(value, store)?
        },
        probabilities: {
            let (presence, value) = read_presence(
                node.member("probabilities"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_session_probabilities(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_session_probabilities_presence { presence, value }
        },
        question: {
            let value = node.required("question")?;
            read_thinkthen_complete_question_document(value, store)
                .map(|value| store.hold(value))?
        },
        question_sha256: {
            let value = node.required("question_sha256")?;
            read_text(value, store)?
        },
        question_sources: {
            let value = node.required("question_sources")?;
            read_thinkthen_complete_question_source_list(value, store)?
        },
        raw_pick: {
            let (presence, value) = read_presence(
                node.member("raw_pick"),
                false,
                thinkthen_complete_utf8 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence { presence, value }
        },
        reported_usage: {
            let (presence, value) = read_presence(
                node.member("reported_usage"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_usage(value, store).map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_usage_presence { presence, value }
        },
        requests: {
            let value = node.required("requests")?;
            read_thinkthen_complete_string_list(value, store)?
        },
        requests_sent: {
            let value = node.required("requests_sent")?;
            read_number(value, store)?
        },
        threshold: {
            let (presence, value) = read_presence(
                node.member("threshold"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_threshold(value, store).map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_threshold_presence { presence, value }
        },
        url: {
            let value = node.required("url")?;
            read_text(value, store)?
        },
        usage: {
            let (presence, value) = read_presence(
                node.member("usage"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_token_usage(value, store).map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_token_usage_presence { presence, value }
        },
        value: {
            let (presence, value) = read_presence(
                node.member("value"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_value(value, store).map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_value_presence { presence, value }
        },
        extensions: store.extensions(
            node,
            &[
                "answer_id",
                "cached",
                "confidence",
                "failed_questions",
                "failure",
                "failure_id",
                "input",
                "input_source",
                "input_sources",
                "inputs",
                "model",
                "observations",
                "probabilities",
                "question",
                "question_sha256",
                "question_sources",
                "raw_pick",
                "reported_usage",
                "requests",
                "requests_sent",
                "threshold",
                "url",
                "usage",
                "value",
            ],
        )?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_boundary_proposal_list_presence {
    pub presence: u32,
    pub value: thinkthen_complete_boundary_proposal_list,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_recognition_edge_list {
    pub data: *const *const thinkthen_complete_recognition_edge,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_recognition_edge_list(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_recognition_edge_list, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_recognition_edge(item, store).map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_recognition_edge_list { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_recognition_edge_list_presence {
    pub presence: u32,
    pub value: thinkthen_complete_recognition_edge_list,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_recognition {
    pub entities: thinkthen_complete_entity_list,
    pub mode: *const thinkthen_complete_recognition_mode,
    pub proposals: thinkthen_complete_boundary_proposal_list_presence,
    pub relations: thinkthen_complete_recognition_edge_list_presence,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_session_recognition(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_recognition, ErrorKind> {
    Ok(thinkthen_complete_session_recognition {
        entities: {
            let value = node.required("entities")?;
            read_thinkthen_complete_entity_list(value, store)?
        },
        mode: {
            let value = node.required("mode")?;
            read_thinkthen_complete_recognition_mode(value, store).map(|value| store.hold(value))?
        },
        proposals: {
            let (presence, value) = read_presence(
                node.member("proposals"),
                false,
                thinkthen_complete_boundary_proposal_list {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_boundary_proposal_list,
                store,
            )?;
            thinkthen_complete_boundary_proposal_list_presence { presence, value }
        },
        relations: {
            let (presence, value) = read_presence(
                node.member("relations"),
                false,
                thinkthen_complete_recognition_edge_list {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_recognition_edge_list,
                store,
            )?;
            thinkthen_complete_recognition_edge_list_presence { presence, value }
        },
        extensions: store.extensions(node, &["entities", "mode", "proposals", "relations"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_relation_edge {
    pub either: u32,
    pub probability: f64,
    pub relation: thinkthen_complete_utf8,
    pub source: *const thinkthen_complete_entity_document,
    pub target: *const thinkthen_complete_entity_document,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_session_relation_edge(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_relation_edge, ErrorKind> {
    Ok(thinkthen_complete_session_relation_edge {
        either: {
            let value = node.required("either")?;
            read_boolean(value, store)?
        },
        probability: {
            let value = node.required("probability")?;
            read_number(value, store)?
        },
        relation: {
            let value = node.required("relation")?;
            read_text(value, store)?
        },
        source: {
            let value = node.required("source")?;
            read_thinkthen_complete_entity_document(value, store).map(|value| store.hold(value))?
        },
        target: {
            let value = node.required("target")?;
            read_thinkthen_complete_entity_document(value, store).map(|value| store.hold(value))?
        },
        extensions: store.extensions(
            node,
            &["either", "probability", "relation", "source", "target"],
        )?,
    })
}

pub const THINKTHEN_COMPLETE_SETTLED_ANSWER_DECIDE: u32 = 1;

pub const THINKTHEN_COMPLETE_SETTLED_ANSWER_CHOOSE: u32 = 2;

pub const THINKTHEN_COMPLETE_SETTLED_ANSWER_TAG: u32 = 3;

pub const THINKTHEN_COMPLETE_SETTLED_ANSWER_SCORE: u32 = 4;

pub const THINKTHEN_COMPLETE_SETTLED_ANSWER_FILTER: u32 = 5;

pub const THINKTHEN_COMPLETE_SETTLED_ANSWER_RANK: u32 = 6;

pub const THINKTHEN_COMPLETE_SETTLED_ANSWER_FIND: u32 = 7;

pub const THINKTHEN_COMPLETE_SETTLED_ANSWER_RECOGNIZE: u32 = 8;

pub const THINKTHEN_COMPLETE_SETTLED_ANSWER_RELATE: u32 = 9;

pub const THINKTHEN_COMPLETE_SETTLED_ANSWER_ANNOTATE: u32 = 10;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_settled_answer_data {
    pub decide: *const thinkthen_complete_settled_answer_decide,
    pub choose: *const thinkthen_complete_settled_answer_choose,
    pub tag: *const thinkthen_complete_settled_answer_tag,
    pub score: *const thinkthen_complete_settled_answer_score,
    pub filter: *const thinkthen_complete_settled_answer_filter,
    pub rank: *const thinkthen_complete_settled_answer_rank,
    pub find: *const thinkthen_complete_settled_answer_find,
    pub recognize: *const thinkthen_complete_settled_answer_recognize,
    pub relate: *const thinkthen_complete_settled_answer_relate,
    pub annotate: *const thinkthen_complete_settled_answer_annotate,
}
impl std::fmt::Debug for thinkthen_complete_settled_answer_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("union payload")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_settled_answer {
    pub kind: u32,
    pub data: thinkthen_complete_settled_answer_data,
}

pub(crate) fn read_thinkthen_complete_settled_answer(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_settled_answer, ErrorKind> {
    if node.is_member("function", "decide") {
        let value = read_thinkthen_complete_settled_answer_decide(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_settled_answer {
            kind: THINKTHEN_COMPLETE_SETTLED_ANSWER_DECIDE,
            data: thinkthen_complete_settled_answer_data { decide: value },
        });
    }
    if node.is_member("function", "choose") {
        let value = read_thinkthen_complete_settled_answer_choose(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_settled_answer {
            kind: THINKTHEN_COMPLETE_SETTLED_ANSWER_CHOOSE,
            data: thinkthen_complete_settled_answer_data { choose: value },
        });
    }
    if node.is_member("function", "tag") {
        let value = read_thinkthen_complete_settled_answer_tag(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_settled_answer {
            kind: THINKTHEN_COMPLETE_SETTLED_ANSWER_TAG,
            data: thinkthen_complete_settled_answer_data { tag: value },
        });
    }
    if node.is_member("function", "score") {
        let value = read_thinkthen_complete_settled_answer_score(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_settled_answer {
            kind: THINKTHEN_COMPLETE_SETTLED_ANSWER_SCORE,
            data: thinkthen_complete_settled_answer_data { score: value },
        });
    }
    if node.is_member("function", "filter") {
        let value = read_thinkthen_complete_settled_answer_filter(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_settled_answer {
            kind: THINKTHEN_COMPLETE_SETTLED_ANSWER_FILTER,
            data: thinkthen_complete_settled_answer_data { filter: value },
        });
    }
    if node.is_member("function", "rank") {
        let value = read_thinkthen_complete_settled_answer_rank(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_settled_answer {
            kind: THINKTHEN_COMPLETE_SETTLED_ANSWER_RANK,
            data: thinkthen_complete_settled_answer_data { rank: value },
        });
    }
    if node.is_member("function", "find") {
        let value = read_thinkthen_complete_settled_answer_find(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_settled_answer {
            kind: THINKTHEN_COMPLETE_SETTLED_ANSWER_FIND,
            data: thinkthen_complete_settled_answer_data { find: value },
        });
    }
    if node.is_member("function", "recognize") {
        let value = read_thinkthen_complete_settled_answer_recognize(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_settled_answer {
            kind: THINKTHEN_COMPLETE_SETTLED_ANSWER_RECOGNIZE,
            data: thinkthen_complete_settled_answer_data { recognize: value },
        });
    }
    if node.is_member("function", "relate") {
        let value = read_thinkthen_complete_settled_answer_relate(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_settled_answer {
            kind: THINKTHEN_COMPLETE_SETTLED_ANSWER_RELATE,
            data: thinkthen_complete_settled_answer_data { relate: value },
        });
    }
    if node.is_member("function", "annotate") {
        let value = read_thinkthen_complete_settled_answer_annotate(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_settled_answer {
            kind: THINKTHEN_COMPLETE_SETTLED_ANSWER_ANNOTATE,
            data: thinkthen_complete_settled_answer_data { annotate: value },
        });
    }
    Err(ErrorKind::Defect)
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answer_shape_4_presence {
    pub presence: u32,
    pub value: *const thinkthen_complete_answer_shape_4,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answer_shape_3_presence {
    pub presence: u32,
    pub value: *const thinkthen_complete_answer_shape_3,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answer_shape_2_presence {
    pub presence: u32,
    pub value: *const thinkthen_complete_answer_shape_2,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answer_shape_15_presence {
    pub presence: u32,
    pub value: *const thinkthen_complete_answer_shape_15,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_settled_answer_annotate {
    pub function: thinkthen_complete_utf8,
    pub index: thinkthen_complete_answer_shape_4_presence,
    pub pick: thinkthen_complete_answer_shape_3_presence,
    pub probability: thinkthen_complete_answer_shape_2_presence,
    pub resolution: *const thinkthen_complete_answer_shape_5,
    pub value: thinkthen_complete_answer_shape_15_presence,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_settled_answer_annotate(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_settled_answer_annotate, ErrorKind> {
    Ok(thinkthen_complete_settled_answer_annotate {
        function: {
            let value = node.required("function")?;
            read_text(value, store)?
        },
        index: {
            let (presence, value) = read_presence(
                node.member("index"),
                true,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_answer_shape_4(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_answer_shape_4_presence { presence, value }
        },
        pick: {
            let (presence, value) = read_presence(
                node.member("pick"),
                true,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_answer_shape_3(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_answer_shape_3_presence { presence, value }
        },
        probability: {
            let (presence, value) = read_presence(
                node.member("probability"),
                true,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_answer_shape_2(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_answer_shape_2_presence { presence, value }
        },
        resolution: {
            let value = node.required("resolution")?;
            read_thinkthen_complete_answer_shape_5(value, store).map(|value| store.hold(value))?
        },
        value: {
            let (presence, value) = read_presence(
                node.member("value"),
                true,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_answer_shape_15(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_answer_shape_15_presence { presence, value }
        },
        extensions: store.extensions(
            node,
            &[
                "function",
                "index",
                "pick",
                "probability",
                "resolution",
                "value",
            ],
        )?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answer_shape_6_presence {
    pub presence: u32,
    pub value: *const thinkthen_complete_answer_shape_6,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_settled_answer_choose {
    pub function: thinkthen_complete_utf8,
    pub index: thinkthen_complete_answer_shape_4_presence,
    pub pick: thinkthen_complete_answer_shape_3_presence,
    pub probability: thinkthen_complete_answer_shape_2_presence,
    pub resolution: *const thinkthen_complete_answer_shape_5,
    pub value: thinkthen_complete_answer_shape_6_presence,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_settled_answer_choose(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_settled_answer_choose, ErrorKind> {
    Ok(thinkthen_complete_settled_answer_choose {
        function: {
            let value = node.required("function")?;
            read_text(value, store)?
        },
        index: {
            let (presence, value) = read_presence(
                node.member("index"),
                true,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_answer_shape_4(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_answer_shape_4_presence { presence, value }
        },
        pick: {
            let (presence, value) = read_presence(
                node.member("pick"),
                true,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_answer_shape_3(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_answer_shape_3_presence { presence, value }
        },
        probability: {
            let (presence, value) = read_presence(
                node.member("probability"),
                true,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_answer_shape_2(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_answer_shape_2_presence { presence, value }
        },
        resolution: {
            let value = node.required("resolution")?;
            read_thinkthen_complete_answer_shape_5(value, store).map(|value| store.hold(value))?
        },
        value: {
            let (presence, value) = read_presence(
                node.member("value"),
                true,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_answer_shape_6(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_answer_shape_6_presence { presence, value }
        },
        extensions: store.extensions(
            node,
            &[
                "function",
                "index",
                "pick",
                "probability",
                "resolution",
                "value",
            ],
        )?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answer_shape_presence {
    pub presence: u32,
    pub value: *const thinkthen_complete_answer_shape,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_settled_answer_decide {
    pub function: thinkthen_complete_utf8,
    pub index: thinkthen_complete_answer_shape_4_presence,
    pub pick: thinkthen_complete_answer_shape_3_presence,
    pub probability: thinkthen_complete_answer_shape_2_presence,
    pub resolution: *const thinkthen_complete_answer_shape_5,
    pub value: thinkthen_complete_answer_shape_presence,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_settled_answer_decide(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_settled_answer_decide, ErrorKind> {
    Ok(thinkthen_complete_settled_answer_decide {
        function: {
            let value = node.required("function")?;
            read_text(value, store)?
        },
        index: {
            let (presence, value) = read_presence(
                node.member("index"),
                true,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_answer_shape_4(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_answer_shape_4_presence { presence, value }
        },
        pick: {
            let (presence, value) = read_presence(
                node.member("pick"),
                true,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_answer_shape_3(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_answer_shape_3_presence { presence, value }
        },
        probability: {
            let (presence, value) = read_presence(
                node.member("probability"),
                true,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_answer_shape_2(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_answer_shape_2_presence { presence, value }
        },
        resolution: {
            let value = node.required("resolution")?;
            read_thinkthen_complete_answer_shape_5(value, store).map(|value| store.hold(value))?
        },
        value: {
            let (presence, value) = read_presence(
                node.member("value"),
                true,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_answer_shape(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_answer_shape_presence { presence, value }
        },
        extensions: store.extensions(
            node,
            &[
                "function",
                "index",
                "pick",
                "probability",
                "resolution",
                "value",
            ],
        )?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answer_shape_10_presence {
    pub presence: u32,
    pub value: *const thinkthen_complete_answer_shape_10,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answer_shape_9_presence {
    pub presence: u32,
    pub value: *const thinkthen_complete_answer_shape_9,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_settled_answer_filter {
    pub function: thinkthen_complete_utf8,
    pub index: thinkthen_complete_answer_shape_4_presence,
    pub pick: thinkthen_complete_answer_shape_3_presence,
    pub probability: thinkthen_complete_answer_shape_10_presence,
    pub resolution: *const thinkthen_complete_answer_shape_5,
    pub value: thinkthen_complete_answer_shape_9_presence,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_settled_answer_filter(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_settled_answer_filter, ErrorKind> {
    Ok(thinkthen_complete_settled_answer_filter {
        function: {
            let value = node.required("function")?;
            read_text(value, store)?
        },
        index: {
            let (presence, value) = read_presence(
                node.member("index"),
                true,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_answer_shape_4(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_answer_shape_4_presence { presence, value }
        },
        pick: {
            let (presence, value) = read_presence(
                node.member("pick"),
                true,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_answer_shape_3(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_answer_shape_3_presence { presence, value }
        },
        probability: {
            let (presence, value) = read_presence(
                node.member("probability"),
                true,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_answer_shape_10(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_answer_shape_10_presence { presence, value }
        },
        resolution: {
            let value = node.required("resolution")?;
            read_thinkthen_complete_answer_shape_5(value, store).map(|value| store.hold(value))?
        },
        value: {
            let (presence, value) = read_presence(
                node.member("value"),
                true,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_answer_shape_9(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_answer_shape_9_presence { presence, value }
        },
        extensions: store.extensions(
            node,
            &[
                "function",
                "index",
                "pick",
                "probability",
                "resolution",
                "value",
            ],
        )?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answer_shape_12_presence {
    pub presence: u32,
    pub value: *const thinkthen_complete_answer_shape_12,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_settled_answer_find {
    pub function: thinkthen_complete_utf8,
    pub index: thinkthen_complete_answer_shape_4_presence,
    pub pick: thinkthen_complete_answer_shape_3_presence,
    pub probability: thinkthen_complete_answer_shape_2_presence,
    pub resolution: *const thinkthen_complete_answer_shape_5,
    pub value: thinkthen_complete_answer_shape_12_presence,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_settled_answer_find(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_settled_answer_find, ErrorKind> {
    Ok(thinkthen_complete_settled_answer_find {
        function: {
            let value = node.required("function")?;
            read_text(value, store)?
        },
        index: {
            let (presence, value) = read_presence(
                node.member("index"),
                true,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_answer_shape_4(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_answer_shape_4_presence { presence, value }
        },
        pick: {
            let (presence, value) = read_presence(
                node.member("pick"),
                true,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_answer_shape_3(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_answer_shape_3_presence { presence, value }
        },
        probability: {
            let (presence, value) = read_presence(
                node.member("probability"),
                true,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_answer_shape_2(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_answer_shape_2_presence { presence, value }
        },
        resolution: {
            let value = node.required("resolution")?;
            read_thinkthen_complete_answer_shape_5(value, store).map(|value| store.hold(value))?
        },
        value: {
            let (presence, value) = read_presence(
                node.member("value"),
                true,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_answer_shape_12(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_answer_shape_12_presence { presence, value }
        },
        extensions: store.extensions(
            node,
            &[
                "function",
                "index",
                "pick",
                "probability",
                "resolution",
                "value",
            ],
        )?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answer_shape_11_presence {
    pub presence: u32,
    pub value: *const thinkthen_complete_answer_shape_11,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_settled_answer_rank {
    pub function: thinkthen_complete_utf8,
    pub index: thinkthen_complete_answer_shape_4_presence,
    pub pick: thinkthen_complete_answer_shape_3_presence,
    pub probability: thinkthen_complete_answer_shape_10_presence,
    pub resolution: *const thinkthen_complete_answer_shape_5,
    pub value: thinkthen_complete_answer_shape_11_presence,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_settled_answer_rank(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_settled_answer_rank, ErrorKind> {
    Ok(thinkthen_complete_settled_answer_rank {
        function: {
            let value = node.required("function")?;
            read_text(value, store)?
        },
        index: {
            let (presence, value) = read_presence(
                node.member("index"),
                true,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_answer_shape_4(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_answer_shape_4_presence { presence, value }
        },
        pick: {
            let (presence, value) = read_presence(
                node.member("pick"),
                true,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_answer_shape_3(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_answer_shape_3_presence { presence, value }
        },
        probability: {
            let (presence, value) = read_presence(
                node.member("probability"),
                true,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_answer_shape_10(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_answer_shape_10_presence { presence, value }
        },
        resolution: {
            let value = node.required("resolution")?;
            read_thinkthen_complete_answer_shape_5(value, store).map(|value| store.hold(value))?
        },
        value: {
            let (presence, value) = read_presence(
                node.member("value"),
                true,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_answer_shape_11(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_answer_shape_11_presence { presence, value }
        },
        extensions: store.extensions(
            node,
            &[
                "function",
                "index",
                "pick",
                "probability",
                "resolution",
                "value",
            ],
        )?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answer_shape_13_presence {
    pub presence: u32,
    pub value: *const thinkthen_complete_answer_shape_13,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_settled_answer_recognize {
    pub function: thinkthen_complete_utf8,
    pub index: thinkthen_complete_answer_shape_4_presence,
    pub pick: thinkthen_complete_answer_shape_3_presence,
    pub probability: thinkthen_complete_answer_shape_10_presence,
    pub resolution: *const thinkthen_complete_answer_shape_5,
    pub value: thinkthen_complete_answer_shape_13_presence,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_settled_answer_recognize(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_settled_answer_recognize, ErrorKind> {
    Ok(thinkthen_complete_settled_answer_recognize {
        function: {
            let value = node.required("function")?;
            read_text(value, store)?
        },
        index: {
            let (presence, value) = read_presence(
                node.member("index"),
                true,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_answer_shape_4(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_answer_shape_4_presence { presence, value }
        },
        pick: {
            let (presence, value) = read_presence(
                node.member("pick"),
                true,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_answer_shape_3(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_answer_shape_3_presence { presence, value }
        },
        probability: {
            let (presence, value) = read_presence(
                node.member("probability"),
                true,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_answer_shape_10(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_answer_shape_10_presence { presence, value }
        },
        resolution: {
            let value = node.required("resolution")?;
            read_thinkthen_complete_answer_shape_5(value, store).map(|value| store.hold(value))?
        },
        value: {
            let (presence, value) = read_presence(
                node.member("value"),
                true,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_answer_shape_13(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_answer_shape_13_presence { presence, value }
        },
        extensions: store.extensions(
            node,
            &[
                "function",
                "index",
                "pick",
                "probability",
                "resolution",
                "value",
            ],
        )?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answer_shape_14_presence {
    pub presence: u32,
    pub value: *const thinkthen_complete_answer_shape_14,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_settled_answer_relate {
    pub function: thinkthen_complete_utf8,
    pub index: thinkthen_complete_answer_shape_4_presence,
    pub pick: thinkthen_complete_answer_shape_3_presence,
    pub probability: thinkthen_complete_answer_shape_10_presence,
    pub resolution: *const thinkthen_complete_answer_shape_5,
    pub value: thinkthen_complete_answer_shape_14_presence,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_settled_answer_relate(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_settled_answer_relate, ErrorKind> {
    Ok(thinkthen_complete_settled_answer_relate {
        function: {
            let value = node.required("function")?;
            read_text(value, store)?
        },
        index: {
            let (presence, value) = read_presence(
                node.member("index"),
                true,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_answer_shape_4(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_answer_shape_4_presence { presence, value }
        },
        pick: {
            let (presence, value) = read_presence(
                node.member("pick"),
                true,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_answer_shape_3(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_answer_shape_3_presence { presence, value }
        },
        probability: {
            let (presence, value) = read_presence(
                node.member("probability"),
                true,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_answer_shape_10(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_answer_shape_10_presence { presence, value }
        },
        resolution: {
            let value = node.required("resolution")?;
            read_thinkthen_complete_answer_shape_5(value, store).map(|value| store.hold(value))?
        },
        value: {
            let (presence, value) = read_presence(
                node.member("value"),
                true,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_answer_shape_14(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_answer_shape_14_presence { presence, value }
        },
        extensions: store.extensions(
            node,
            &[
                "function",
                "index",
                "pick",
                "probability",
                "resolution",
                "value",
            ],
        )?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answer_shape_8_presence {
    pub presence: u32,
    pub value: *const thinkthen_complete_answer_shape_8,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_settled_answer_score {
    pub function: thinkthen_complete_utf8,
    pub index: thinkthen_complete_answer_shape_4_presence,
    pub pick: thinkthen_complete_answer_shape_3_presence,
    pub probability: thinkthen_complete_answer_shape_2_presence,
    pub resolution: *const thinkthen_complete_answer_shape_5,
    pub value: thinkthen_complete_answer_shape_8_presence,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_settled_answer_score(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_settled_answer_score, ErrorKind> {
    Ok(thinkthen_complete_settled_answer_score {
        function: {
            let value = node.required("function")?;
            read_text(value, store)?
        },
        index: {
            let (presence, value) = read_presence(
                node.member("index"),
                true,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_answer_shape_4(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_answer_shape_4_presence { presence, value }
        },
        pick: {
            let (presence, value) = read_presence(
                node.member("pick"),
                true,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_answer_shape_3(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_answer_shape_3_presence { presence, value }
        },
        probability: {
            let (presence, value) = read_presence(
                node.member("probability"),
                true,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_answer_shape_2(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_answer_shape_2_presence { presence, value }
        },
        resolution: {
            let value = node.required("resolution")?;
            read_thinkthen_complete_answer_shape_5(value, store).map(|value| store.hold(value))?
        },
        value: {
            let (presence, value) = read_presence(
                node.member("value"),
                true,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_answer_shape_8(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_answer_shape_8_presence { presence, value }
        },
        extensions: store.extensions(
            node,
            &[
                "function",
                "index",
                "pick",
                "probability",
                "resolution",
                "value",
            ],
        )?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answer_shape_7_presence {
    pub presence: u32,
    pub value: *const thinkthen_complete_answer_shape_7,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_settled_answer_tag {
    pub function: thinkthen_complete_utf8,
    pub index: thinkthen_complete_answer_shape_4_presence,
    pub pick: thinkthen_complete_answer_shape_3_presence,
    pub probability: thinkthen_complete_answer_shape_2_presence,
    pub resolution: *const thinkthen_complete_answer_shape_5,
    pub value: thinkthen_complete_answer_shape_7_presence,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_settled_answer_tag(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_settled_answer_tag, ErrorKind> {
    Ok(thinkthen_complete_settled_answer_tag {
        function: {
            let value = node.required("function")?;
            read_text(value, store)?
        },
        index: {
            let (presence, value) = read_presence(
                node.member("index"),
                true,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_answer_shape_4(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_answer_shape_4_presence { presence, value }
        },
        pick: {
            let (presence, value) = read_presence(
                node.member("pick"),
                true,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_answer_shape_3(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_answer_shape_3_presence { presence, value }
        },
        probability: {
            let (presence, value) = read_presence(
                node.member("probability"),
                true,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_answer_shape_2(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_answer_shape_2_presence { presence, value }
        },
        resolution: {
            let value = node.required("resolution")?;
            read_thinkthen_complete_answer_shape_5(value, store).map(|value| store.hold(value))?
        },
        value: {
            let (presence, value) = read_presence(
                node.member("value"),
                true,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_answer_shape_7(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_answer_shape_7_presence { presence, value }
        },
        extensions: store.extensions(
            node,
            &[
                "function",
                "index",
                "pick",
                "probability",
                "resolution",
                "value",
            ],
        )?,
    })
}

pub const THINKTHEN_COMPLETE_THRESHOLD_NUMBER: u32 = 1;

pub const THINKTHEN_COMPLETE_THRESHOLD_STRING: u32 = 2;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_threshold_data {
    pub number: f64,
    pub string: thinkthen_complete_utf8,
}
impl std::fmt::Debug for thinkthen_complete_threshold_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("union payload")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_threshold {
    pub kind: u32,
    pub data: thinkthen_complete_threshold_data,
}

pub(crate) fn read_thinkthen_complete_threshold(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_threshold, ErrorKind> {
    if node.kind() == "number" {
        let value = read_number(node, store)?;
        return Ok(thinkthen_complete_threshold {
            kind: THINKTHEN_COMPLETE_THRESHOLD_NUMBER,
            data: thinkthen_complete_threshold_data { number: value },
        });
    }
    if node.kind() == "string" {
        let value = read_text(node, store)?;
        return Ok(thinkthen_complete_threshold {
            kind: THINKTHEN_COMPLETE_THRESHOLD_STRING,
            data: thinkthen_complete_threshold_data { string: value },
        });
    }
    Err(ErrorKind::Defect)
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_token_usage {
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub extensions: thinkthen_complete_extensions,
}

pub(crate) fn read_thinkthen_complete_token_usage(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_token_usage, ErrorKind> {
    Ok(thinkthen_complete_token_usage {
        input_tokens: {
            let value = node.required("input_tokens")?;
            read_number(value, store)?
        },
        output_tokens: {
            let value = node.required("output_tokens")?;
            read_number(value, store)?
        },
        extensions: store.extensions(node, &["input_tokens", "output_tokens"])?,
    })
}

pub const THINKTHEN_COMPLETE_VALUE_BOOLEAN: u32 = 1;

pub const THINKTHEN_COMPLETE_VALUE_NULL: u32 = 2;

pub const THINKTHEN_COMPLETE_VALUE_STRING: u32 = 3;

pub const THINKTHEN_COMPLETE_VALUE_ARRAY: u32 = 4;

pub const THINKTHEN_COMPLETE_VALUE_NUMBER: u32 = 5;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_value_data {
    pub boolean: u32,
    pub null: u32,
    pub string: thinkthen_complete_utf8,
    pub array: thinkthen_complete_string_list,
    pub number: f64,
}
impl std::fmt::Debug for thinkthen_complete_value_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("union payload")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_value {
    pub kind: u32,
    pub data: thinkthen_complete_value_data,
}

pub(crate) fn read_thinkthen_complete_value(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_value, ErrorKind> {
    if node.kind() == "boolean" {
        let value = read_boolean(node, store)?;
        return Ok(thinkthen_complete_value {
            kind: THINKTHEN_COMPLETE_VALUE_BOOLEAN,
            data: thinkthen_complete_value_data { boolean: value },
        });
    }
    if node.kind() == "null" {
        let value = 0;
        return Ok(thinkthen_complete_value {
            kind: THINKTHEN_COMPLETE_VALUE_NULL,
            data: thinkthen_complete_value_data { null: value },
        });
    }
    if node.kind() == "string" {
        let value = read_text(node, store)?;
        return Ok(thinkthen_complete_value {
            kind: THINKTHEN_COMPLETE_VALUE_STRING,
            data: thinkthen_complete_value_data { string: value },
        });
    }
    if node
        .array()
        .is_ok_and(|items| items.iter().all(matches_union_item_11))
    {
        let value = read_thinkthen_complete_string_list(node, store)?;
        return Ok(thinkthen_complete_value {
            kind: THINKTHEN_COMPLETE_VALUE_ARRAY,
            data: thinkthen_complete_value_data { array: value },
        });
    }
    if node.kind() == "number" {
        let value = read_number(node, store)?;
        return Ok(thinkthen_complete_value {
            kind: THINKTHEN_COMPLETE_VALUE_NUMBER,
            data: thinkthen_complete_value_data { number: value },
        });
    }
    Err(ErrorKind::Defect)
}
