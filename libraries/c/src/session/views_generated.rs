// Generated from Rust-derived complete result schema; do not edit.
#![allow(
    non_camel_case_types,
    missing_docs,
    clippy::too_many_lines,
    reason = "generated versioned C schema objects and tagged alternatives"
)]
use super::views::{Node, Storage, read_boolean, read_json, read_number, read_presence, read_text};
use super::views::{
    thinkthen_complete_extensions_v1, thinkthen_complete_json_v1, thinkthen_complete_utf8_v1,
};
use thinkthen::ErrorKind;
pub const THINKTHEN_COMPLETE_PRESENCE_MISSING_V1: u32 = 0;
pub const THINKTHEN_COMPLETE_PRESENCE_NULL_V1: u32 = 1;
pub const THINKTHEN_COMPLETE_PRESENCE_VALUE_V1: u32 = 2;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_annotation_member_map_entry_v1 {
    pub name: thinkthen_complete_utf8_v1,
    pub value: *const thinkthen_complete_annotation_member_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_annotation_member_map_v1 {
    pub data: *const thinkthen_complete_annotation_member_map_entry_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_annotation_member_map_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_annotation_member_map_v1, ErrorKind> {
    let mut values = Vec::new();
    for (key, item) in node.object()? {
        values.push(thinkthen_complete_annotation_member_map_entry_v1 {
            name: store.text(key),
            value: read_thinkthen_complete_annotation_member_v1(item, store)
                .map(|value| store.hold(value))?,
        });
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_annotation_member_map_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_utf8_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_utf8_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_u64_presence_v1 {
    pub presence: u32,
    pub value: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_json_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_json_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_position_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_position_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_physical_source_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_physical_source_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_annotation_v1 {
    pub answer_id: *const thinkthen_complete_answer_id_v1,
    pub answers: thinkthen_complete_annotation_member_map_v1,
    pub file: thinkthen_complete_utf8_presence_v1,
    pub first_line: thinkthen_complete_u64_presence_v1,
    pub index: thinkthen_complete_u64_presence_v1,
    pub input: thinkthen_complete_json_presence_v1,
    pub last_line: thinkthen_complete_u64_presence_v1,
    pub meta: *const thinkthen_complete_meta_v1,
    pub position: thinkthen_complete_position_presence_v1,
    pub schema: *const thinkthen_complete_version_v1,
    pub source: thinkthen_complete_physical_source_presence_v1,
    pub value: *const thinkthen_complete_annotated_row_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_annotation_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_annotation_v1, ErrorKind> {
    Ok(thinkthen_complete_annotation_v1 {
        answer_id: {
            let value = node.required("answer_id")?;
            read_thinkthen_complete_answer_id_v1(value, store).map(|value| store.hold(value))?
        },
        answers: {
            let value = node.required("answers")?;
            read_thinkthen_complete_annotation_member_map_v1(value, store)?
        },
        file: {
            let (presence, value) = read_presence(
                node.member("file"),
                false,
                thinkthen_complete_utf8_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence_v1 { presence, value }
        },
        first_line: {
            let (presence, value) =
                read_presence(node.member("first_line"), false, 0, read_number, store)?;
            thinkthen_complete_u64_presence_v1 { presence, value }
        },
        index: {
            let (presence, value) =
                read_presence(node.member("index"), false, 0, read_number, store)?;
            thinkthen_complete_u64_presence_v1 { presence, value }
        },
        input: {
            let (presence, value) = read_presence(
                node.member("input"),
                true,
                std::ptr::null(),
                read_json,
                store,
            )?;
            thinkthen_complete_json_presence_v1 { presence, value }
        },
        last_line: {
            let (presence, value) =
                read_presence(node.member("last_line"), false, 0, read_number, store)?;
            thinkthen_complete_u64_presence_v1 { presence, value }
        },
        meta: {
            let value = node.required("meta")?;
            read_thinkthen_complete_meta_v1(value, store).map(|value| store.hold(value))?
        },
        position: {
            let (presence, value) = read_presence(
                node.member("position"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_position_v1(value, store).map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_position_presence_v1 { presence, value }
        },
        schema: {
            let value = node.required("schema")?;
            read_thinkthen_complete_version_v1(value, store).map(|value| store.hold(value))?
        },
        source: {
            let (presence, value) = read_presence(
                node.member("source"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_physical_source_v1(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_physical_source_presence_v1 { presence, value }
        },
        value: {
            let value = node.required("value")?;
            read_thinkthen_complete_annotated_row_v1(value, store).map(|value| store.hold(value))?
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

pub const THINKTHEN_COMPLETE_ANNOTATION_MEMBER_ANSWER_ID_V1: u32 = 1;

pub const THINKTHEN_COMPLETE_ANNOTATION_MEMBER_FAILURE_ID_V1: u32 = 2;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_annotation_member_data_v1 {
    pub answer_id: *const thinkthen_complete_annotation_member_answer_id_v1,
    pub failure_id: *const thinkthen_complete_annotation_member_failure_id_v1,
}
impl std::fmt::Debug for thinkthen_complete_annotation_member_data_v1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("union payload")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_annotation_member_v1 {
    pub kind: u32,
    pub data: thinkthen_complete_annotation_member_data_v1,
}

pub(crate) fn read_thinkthen_complete_annotation_member_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_annotation_member_v1, ErrorKind> {
    if node.member("answer_id").is_some() {
        let value = read_thinkthen_complete_annotation_member_answer_id_v1(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_annotation_member_v1 {
            kind: THINKTHEN_COMPLETE_ANNOTATION_MEMBER_ANSWER_ID_V1,
            data: thinkthen_complete_annotation_member_data_v1 { answer_id: value },
        });
    }
    if node.member("failure_id").is_some() {
        let value = read_thinkthen_complete_annotation_member_failure_id_v1(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_annotation_member_v1 {
            kind: THINKTHEN_COMPLETE_ANNOTATION_MEMBER_FAILURE_ID_V1,
            data: thinkthen_complete_annotation_member_data_v1 { failure_id: value },
        });
    }
    Err(ErrorKind::Defect)
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_observation_list_v1 {
    pub data: *const *const thinkthen_complete_observation_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_observation_list_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_observation_list_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_observation_v1(item, store).map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_observation_list_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_question_source_list_v1 {
    pub data: *const *const thinkthen_complete_question_source_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_question_source_list_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_question_source_list_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_question_source_v1(item, store)
                .map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_question_source_list_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_threshold_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_threshold_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_usage_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_usage_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_value_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_value_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_annotation_member_answer_id_v1 {
    pub answer: *const thinkthen_complete_answer_v1,
    pub answer_id: *const thinkthen_complete_answer_id_v1,
    pub observations: thinkthen_complete_observation_list_v1,
    pub question: *const thinkthen_complete_question_document_v1,
    pub question_sources: thinkthen_complete_question_source_list_v1,
    pub request: thinkthen_complete_utf8_v1,
    pub threshold: thinkthen_complete_threshold_presence_v1,
    pub usage: thinkthen_complete_usage_presence_v1,
    pub value: thinkthen_complete_value_presence_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_annotation_member_answer_id_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_annotation_member_answer_id_v1, ErrorKind> {
    Ok(thinkthen_complete_annotation_member_answer_id_v1 {
        answer: {
            let value = node.required("answer")?;
            read_thinkthen_complete_answer_v1(value, store).map(|value| store.hold(value))?
        },
        answer_id: {
            let value = node.required("answer_id")?;
            read_thinkthen_complete_answer_id_v1(value, store).map(|value| store.hold(value))?
        },
        observations: {
            let value = node.required("observations")?;
            read_thinkthen_complete_observation_list_v1(value, store)?
        },
        question: {
            let value = node.required("question")?;
            read_thinkthen_complete_question_document_v1(value, store)
                .map(|value| store.hold(value))?
        },
        question_sources: {
            let value = node.required("question_sources")?;
            read_thinkthen_complete_question_source_list_v1(value, store)?
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
                    read_thinkthen_complete_threshold_v1(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_threshold_presence_v1 { presence, value }
        },
        usage: {
            let (presence, value) = read_presence(
                node.member("usage"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_usage_v1(value, store).map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_usage_presence_v1 { presence, value }
        },
        value: {
            let (presence, value) = read_presence(
                node.member("value"),
                true,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_value_v1(value, store).map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_value_presence_v1 { presence, value }
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
pub struct thinkthen_complete_annotation_member_failure_id_v1 {
    pub failure: *const thinkthen_complete_failure_v1,
    pub failure_id: *const thinkthen_complete_failure_id_v1,
    pub observations: thinkthen_complete_observation_list_v1,
    pub question: *const thinkthen_complete_question_document_v1,
    pub question_sources: thinkthen_complete_question_source_list_v1,
    pub request: thinkthen_complete_utf8_v1,
    pub threshold: thinkthen_complete_threshold_presence_v1,
    pub usage: thinkthen_complete_usage_presence_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_annotation_member_failure_id_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_annotation_member_failure_id_v1, ErrorKind> {
    Ok(thinkthen_complete_annotation_member_failure_id_v1 {
        failure: {
            let value = node.required("failure")?;
            read_thinkthen_complete_failure_v1(value, store).map(|value| store.hold(value))?
        },
        failure_id: {
            let value = node.required("failure_id")?;
            read_thinkthen_complete_failure_id_v1(value, store).map(|value| store.hold(value))?
        },
        observations: {
            let value = node.required("observations")?;
            read_thinkthen_complete_observation_list_v1(value, store)?
        },
        question: {
            let value = node.required("question")?;
            read_thinkthen_complete_question_document_v1(value, store)
                .map(|value| store.hold(value))?
        },
        question_sources: {
            let value = node.required("question_sources")?;
            read_thinkthen_complete_question_source_list_v1(value, store)?
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
                    read_thinkthen_complete_threshold_v1(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_threshold_presence_v1 { presence, value }
        },
        usage: {
            let (presence, value) = read_presence(
                node.member("usage"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_usage_v1(value, store).map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_usage_presence_v1 { presence, value }
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

pub const THINKTHEN_COMPLETE_ANNOTATION_VALUE_DECISION_V1: u32 = 1;

pub const THINKTHEN_COMPLETE_ANNOTATION_VALUE_CHOICE_V1: u32 = 2;

pub const THINKTHEN_COMPLETE_ANNOTATION_VALUE_SCORE_V1: u32 = 3;

pub const THINKTHEN_COMPLETE_ANNOTATION_VALUE_TAGS_V1: u32 = 4;

pub const THINKTHEN_COMPLETE_ANNOTATION_VALUE_FAILED_V1: u32 = 5;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_annotation_value_data_v1 {
    pub decision: *const thinkthen_complete_annotation_value_decision_v1,
    pub choice: *const thinkthen_complete_annotation_value_choice_v1,
    pub score: *const thinkthen_complete_annotation_value_score_v1,
    pub tags: *const thinkthen_complete_annotation_value_tags_v1,
    pub failed: *const thinkthen_complete_annotation_value_failed_v1,
}
impl std::fmt::Debug for thinkthen_complete_annotation_value_data_v1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("union payload")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_annotation_value_v1 {
    pub kind: u32,
    pub data: thinkthen_complete_annotation_value_data_v1,
}

pub(crate) fn read_thinkthen_complete_annotation_value_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_annotation_value_v1, ErrorKind> {
    if node.is_member("kind", "decision") {
        let value = read_thinkthen_complete_annotation_value_decision_v1(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_annotation_value_v1 {
            kind: THINKTHEN_COMPLETE_ANNOTATION_VALUE_DECISION_V1,
            data: thinkthen_complete_annotation_value_data_v1 { decision: value },
        });
    }
    if node.is_member("kind", "choice") {
        let value = read_thinkthen_complete_annotation_value_choice_v1(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_annotation_value_v1 {
            kind: THINKTHEN_COMPLETE_ANNOTATION_VALUE_CHOICE_V1,
            data: thinkthen_complete_annotation_value_data_v1 { choice: value },
        });
    }
    if node.is_member("kind", "score") {
        let value = read_thinkthen_complete_annotation_value_score_v1(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_annotation_value_v1 {
            kind: THINKTHEN_COMPLETE_ANNOTATION_VALUE_SCORE_V1,
            data: thinkthen_complete_annotation_value_data_v1 { score: value },
        });
    }
    if node.is_member("kind", "tags") {
        let value = read_thinkthen_complete_annotation_value_tags_v1(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_annotation_value_v1 {
            kind: THINKTHEN_COMPLETE_ANNOTATION_VALUE_TAGS_V1,
            data: thinkthen_complete_annotation_value_data_v1 { tags: value },
        });
    }
    if node.is_member("kind", "failed") {
        let value = read_thinkthen_complete_annotation_value_failed_v1(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_annotation_value_v1 {
            kind: THINKTHEN_COMPLETE_ANNOTATION_VALUE_FAILED_V1,
            data: thinkthen_complete_annotation_value_data_v1 { failed: value },
        });
    }
    Err(ErrorKind::Defect)
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_annotation_value_choice_v1 {
    pub kind: thinkthen_complete_utf8_v1,
    pub value: thinkthen_complete_utf8_presence_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_annotation_value_choice_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_annotation_value_choice_v1, ErrorKind> {
    Ok(thinkthen_complete_annotation_value_choice_v1 {
        kind: {
            let value = node.required("kind")?;
            read_text(value, store)?
        },
        value: {
            let (presence, value) = read_presence(
                node.member("value"),
                true,
                thinkthen_complete_utf8_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence_v1 { presence, value }
        },
        extensions: store.extensions(node, &["kind", "value"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_u32_presence_v1 {
    pub presence: u32,
    pub value: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_annotation_value_decision_v1 {
    pub kind: thinkthen_complete_utf8_v1,
    pub value: thinkthen_complete_u32_presence_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_annotation_value_decision_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_annotation_value_decision_v1, ErrorKind> {
    Ok(thinkthen_complete_annotation_value_decision_v1 {
        kind: {
            let value = node.required("kind")?;
            read_text(value, store)?
        },
        value: {
            let (presence, value) =
                read_presence(node.member("value"), true, 0, read_boolean, store)?;
            thinkthen_complete_u32_presence_v1 { presence, value }
        },
        extensions: store.extensions(node, &["kind", "value"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_annotation_value_failed_v1 {
    pub kind: thinkthen_complete_utf8_v1,
    pub value: *const thinkthen_complete_failure_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_annotation_value_failed_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_annotation_value_failed_v1, ErrorKind> {
    Ok(thinkthen_complete_annotation_value_failed_v1 {
        kind: {
            let value = node.required("kind")?;
            read_text(value, store)?
        },
        value: {
            let value = node.required("value")?;
            read_thinkthen_complete_failure_v1(value, store).map(|value| store.hold(value))?
        },
        extensions: store.extensions(node, &["kind", "value"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_annotation_value_score_v1 {
    pub kind: thinkthen_complete_utf8_v1,
    pub value: f64,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_annotation_value_score_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_annotation_value_score_v1, ErrorKind> {
    Ok(thinkthen_complete_annotation_value_score_v1 {
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
pub struct thinkthen_complete_string_list_v1 {
    pub data: *const thinkthen_complete_utf8_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_string_list_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_string_list_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(read_text(item, store)?);
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_string_list_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_annotation_value_tags_v1 {
    pub kind: thinkthen_complete_utf8_v1,
    pub value: thinkthen_complete_string_list_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_annotation_value_tags_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_annotation_value_tags_v1, ErrorKind> {
    Ok(thinkthen_complete_annotation_value_tags_v1 {
        kind: {
            let value = node.required("kind")?;
            read_text(value, store)?
        },
        value: {
            let value = node.required("value")?;
            read_thinkthen_complete_string_list_v1(value, store)?
        },
        extensions: store.extensions(node, &["kind", "value"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answer_id_v1 {
    pub value: thinkthen_complete_utf8_v1,
}

pub(crate) fn read_thinkthen_complete_answer_id_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_answer_id_v1, ErrorKind> {
    let value = read_text(node, store)?;
    Ok(thinkthen_complete_answer_id_v1 { value })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_relation_member_list_v1 {
    pub data: *const *const thinkthen_complete_relation_member_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_relation_member_list_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_relation_member_list_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_relation_member_v1(item, store)
                .map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_relation_member_list_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answers_v1 {
    pub questions: thinkthen_complete_relation_member_list_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_answers_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_answers_v1, ErrorKind> {
    Ok(thinkthen_complete_answers_v1 {
        questions: {
            let value = node.required("questions")?;
            read_thinkthen_complete_relation_member_list_v1(value, store)?
        },
        extensions: store.extensions(node, &["questions"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_image_list_v1 {
    pub data: *const *const thinkthen_complete_image_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_image_list_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_image_list_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(read_thinkthen_complete_image_v1(item, store).map(|value| store.hold(value))?);
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_image_list_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_image_list_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_image_list_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_rank_member_list_v1 {
    pub data: *const *const thinkthen_complete_rank_member_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_rank_member_list_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_rank_member_list_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_rank_member_v1(item, store).map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_rank_member_list_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_rank_member_list_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_rank_member_list_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_atomic_array_of_string_v1 {
    pub answer: *const thinkthen_complete_answer_v1,
    pub answer_id: *const thinkthen_complete_answer_id_v1,
    pub images: thinkthen_complete_image_list_presence_v1,
    pub index: thinkthen_complete_u64_presence_v1,
    pub input: thinkthen_complete_json_presence_v1,
    pub members: thinkthen_complete_rank_member_list_presence_v1,
    pub meta: *const thinkthen_complete_meta_v1,
    pub question: *const thinkthen_complete_question_document_v1,
    pub question_name: thinkthen_complete_utf8_presence_v1,
    pub schema: *const thinkthen_complete_version_v1,
    pub source: thinkthen_complete_physical_source_presence_v1,
    pub threshold: thinkthen_complete_threshold_presence_v1,
    pub value: thinkthen_complete_string_list_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_atomic_array_of_string_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_atomic_array_of_string_v1, ErrorKind> {
    Ok(thinkthen_complete_atomic_array_of_string_v1 {
        answer: {
            let value = node.required("answer")?;
            read_thinkthen_complete_answer_v1(value, store).map(|value| store.hold(value))?
        },
        answer_id: {
            let value = node.required("answer_id")?;
            read_thinkthen_complete_answer_id_v1(value, store).map(|value| store.hold(value))?
        },
        images: {
            let (presence, value) = read_presence(
                node.member("images"),
                false,
                thinkthen_complete_image_list_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_image_list_v1,
                store,
            )?;
            thinkthen_complete_image_list_presence_v1 { presence, value }
        },
        index: {
            let (presence, value) =
                read_presence(node.member("index"), false, 0, read_number, store)?;
            thinkthen_complete_u64_presence_v1 { presence, value }
        },
        input: {
            let (presence, value) = read_presence(
                node.member("input"),
                false,
                std::ptr::null(),
                read_json,
                store,
            )?;
            thinkthen_complete_json_presence_v1 { presence, value }
        },
        members: {
            let (presence, value) = read_presence(
                node.member("members"),
                false,
                thinkthen_complete_rank_member_list_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_rank_member_list_v1,
                store,
            )?;
            thinkthen_complete_rank_member_list_presence_v1 { presence, value }
        },
        meta: {
            let value = node.required("meta")?;
            read_thinkthen_complete_meta_v1(value, store).map(|value| store.hold(value))?
        },
        question: {
            let value = node.required("question")?;
            read_thinkthen_complete_question_document_v1(value, store)
                .map(|value| store.hold(value))?
        },
        question_name: {
            let (presence, value) = read_presence(
                node.member("question_name"),
                false,
                thinkthen_complete_utf8_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence_v1 { presence, value }
        },
        schema: {
            let value = node.required("schema")?;
            read_thinkthen_complete_version_v1(value, store).map(|value| store.hold(value))?
        },
        source: {
            let (presence, value) = read_presence(
                node.member("source"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_physical_source_v1(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_physical_source_presence_v1 { presence, value }
        },
        threshold: {
            let (presence, value) = read_presence(
                node.member("threshold"),
                true,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_threshold_v1(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_threshold_presence_v1 { presence, value }
        },
        value: {
            let value = node.required("value")?;
            read_thinkthen_complete_string_list_v1(value, store)?
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
pub struct thinkthen_complete_decide_value_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_decide_value_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_atomic_decide_value_v1 {
    pub answer: *const thinkthen_complete_answer_v1,
    pub answer_id: *const thinkthen_complete_answer_id_v1,
    pub images: thinkthen_complete_image_list_presence_v1,
    pub index: thinkthen_complete_u64_presence_v1,
    pub input: thinkthen_complete_json_presence_v1,
    pub members: thinkthen_complete_rank_member_list_presence_v1,
    pub meta: *const thinkthen_complete_meta_v1,
    pub question: *const thinkthen_complete_question_document_v1,
    pub question_name: thinkthen_complete_utf8_presence_v1,
    pub schema: *const thinkthen_complete_version_v1,
    pub source: thinkthen_complete_physical_source_presence_v1,
    pub threshold: thinkthen_complete_threshold_presence_v1,
    pub value: thinkthen_complete_decide_value_presence_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_atomic_decide_value_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_atomic_decide_value_v1, ErrorKind> {
    Ok(thinkthen_complete_atomic_decide_value_v1 {
        answer: {
            let value = node.required("answer")?;
            read_thinkthen_complete_answer_v1(value, store).map(|value| store.hold(value))?
        },
        answer_id: {
            let value = node.required("answer_id")?;
            read_thinkthen_complete_answer_id_v1(value, store).map(|value| store.hold(value))?
        },
        images: {
            let (presence, value) = read_presence(
                node.member("images"),
                false,
                thinkthen_complete_image_list_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_image_list_v1,
                store,
            )?;
            thinkthen_complete_image_list_presence_v1 { presence, value }
        },
        index: {
            let (presence, value) =
                read_presence(node.member("index"), false, 0, read_number, store)?;
            thinkthen_complete_u64_presence_v1 { presence, value }
        },
        input: {
            let (presence, value) = read_presence(
                node.member("input"),
                false,
                std::ptr::null(),
                read_json,
                store,
            )?;
            thinkthen_complete_json_presence_v1 { presence, value }
        },
        members: {
            let (presence, value) = read_presence(
                node.member("members"),
                false,
                thinkthen_complete_rank_member_list_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_rank_member_list_v1,
                store,
            )?;
            thinkthen_complete_rank_member_list_presence_v1 { presence, value }
        },
        meta: {
            let value = node.required("meta")?;
            read_thinkthen_complete_meta_v1(value, store).map(|value| store.hold(value))?
        },
        question: {
            let value = node.required("question")?;
            read_thinkthen_complete_question_document_v1(value, store)
                .map(|value| store.hold(value))?
        },
        question_name: {
            let (presence, value) = read_presence(
                node.member("question_name"),
                false,
                thinkthen_complete_utf8_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence_v1 { presence, value }
        },
        schema: {
            let value = node.required("schema")?;
            read_thinkthen_complete_version_v1(value, store).map(|value| store.hold(value))?
        },
        source: {
            let (presence, value) = read_presence(
                node.member("source"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_physical_source_v1(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_physical_source_presence_v1 { presence, value }
        },
        threshold: {
            let (presence, value) = read_presence(
                node.member("threshold"),
                true,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_threshold_v1(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_threshold_presence_v1 { presence, value }
        },
        value: {
            let (presence, value) = read_presence(
                node.member("value"),
                true,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_decide_value_v1(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_decide_value_presence_v1 { presence, value }
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
pub struct thinkthen_complete_atomic_non_zero_usize_v1 {
    pub answer: *const thinkthen_complete_answer_v1,
    pub answer_id: *const thinkthen_complete_answer_id_v1,
    pub images: thinkthen_complete_image_list_presence_v1,
    pub index: thinkthen_complete_u64_presence_v1,
    pub input: thinkthen_complete_json_presence_v1,
    pub members: thinkthen_complete_rank_member_list_presence_v1,
    pub meta: *const thinkthen_complete_meta_v1,
    pub question: *const thinkthen_complete_question_document_v1,
    pub question_name: thinkthen_complete_utf8_presence_v1,
    pub schema: *const thinkthen_complete_version_v1,
    pub source: thinkthen_complete_physical_source_presence_v1,
    pub threshold: thinkthen_complete_threshold_presence_v1,
    pub value: u64,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_atomic_non_zero_usize_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_atomic_non_zero_usize_v1, ErrorKind> {
    Ok(thinkthen_complete_atomic_non_zero_usize_v1 {
        answer: {
            let value = node.required("answer")?;
            read_thinkthen_complete_answer_v1(value, store).map(|value| store.hold(value))?
        },
        answer_id: {
            let value = node.required("answer_id")?;
            read_thinkthen_complete_answer_id_v1(value, store).map(|value| store.hold(value))?
        },
        images: {
            let (presence, value) = read_presence(
                node.member("images"),
                false,
                thinkthen_complete_image_list_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_image_list_v1,
                store,
            )?;
            thinkthen_complete_image_list_presence_v1 { presence, value }
        },
        index: {
            let (presence, value) =
                read_presence(node.member("index"), false, 0, read_number, store)?;
            thinkthen_complete_u64_presence_v1 { presence, value }
        },
        input: {
            let (presence, value) = read_presence(
                node.member("input"),
                false,
                std::ptr::null(),
                read_json,
                store,
            )?;
            thinkthen_complete_json_presence_v1 { presence, value }
        },
        members: {
            let (presence, value) = read_presence(
                node.member("members"),
                false,
                thinkthen_complete_rank_member_list_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_rank_member_list_v1,
                store,
            )?;
            thinkthen_complete_rank_member_list_presence_v1 { presence, value }
        },
        meta: {
            let value = node.required("meta")?;
            read_thinkthen_complete_meta_v1(value, store).map(|value| store.hold(value))?
        },
        question: {
            let value = node.required("question")?;
            read_thinkthen_complete_question_document_v1(value, store)
                .map(|value| store.hold(value))?
        },
        question_name: {
            let (presence, value) = read_presence(
                node.member("question_name"),
                false,
                thinkthen_complete_utf8_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence_v1 { presence, value }
        },
        schema: {
            let value = node.required("schema")?;
            read_thinkthen_complete_version_v1(value, store).map(|value| store.hold(value))?
        },
        source: {
            let (presence, value) = read_presence(
                node.member("source"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_physical_source_v1(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_physical_source_presence_v1 { presence, value }
        },
        threshold: {
            let (presence, value) = read_presence(
                node.member("threshold"),
                true,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_threshold_v1(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_threshold_presence_v1 { presence, value }
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
pub struct thinkthen_complete_atomic_nullable_string_v1 {
    pub answer: *const thinkthen_complete_answer_v1,
    pub answer_id: *const thinkthen_complete_answer_id_v1,
    pub images: thinkthen_complete_image_list_presence_v1,
    pub index: thinkthen_complete_u64_presence_v1,
    pub input: thinkthen_complete_json_presence_v1,
    pub members: thinkthen_complete_rank_member_list_presence_v1,
    pub meta: *const thinkthen_complete_meta_v1,
    pub question: *const thinkthen_complete_question_document_v1,
    pub question_name: thinkthen_complete_utf8_presence_v1,
    pub schema: *const thinkthen_complete_version_v1,
    pub source: thinkthen_complete_physical_source_presence_v1,
    pub threshold: thinkthen_complete_threshold_presence_v1,
    pub value: thinkthen_complete_utf8_presence_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_atomic_nullable_string_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_atomic_nullable_string_v1, ErrorKind> {
    Ok(thinkthen_complete_atomic_nullable_string_v1 {
        answer: {
            let value = node.required("answer")?;
            read_thinkthen_complete_answer_v1(value, store).map(|value| store.hold(value))?
        },
        answer_id: {
            let value = node.required("answer_id")?;
            read_thinkthen_complete_answer_id_v1(value, store).map(|value| store.hold(value))?
        },
        images: {
            let (presence, value) = read_presence(
                node.member("images"),
                false,
                thinkthen_complete_image_list_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_image_list_v1,
                store,
            )?;
            thinkthen_complete_image_list_presence_v1 { presence, value }
        },
        index: {
            let (presence, value) =
                read_presence(node.member("index"), false, 0, read_number, store)?;
            thinkthen_complete_u64_presence_v1 { presence, value }
        },
        input: {
            let (presence, value) = read_presence(
                node.member("input"),
                false,
                std::ptr::null(),
                read_json,
                store,
            )?;
            thinkthen_complete_json_presence_v1 { presence, value }
        },
        members: {
            let (presence, value) = read_presence(
                node.member("members"),
                false,
                thinkthen_complete_rank_member_list_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_rank_member_list_v1,
                store,
            )?;
            thinkthen_complete_rank_member_list_presence_v1 { presence, value }
        },
        meta: {
            let value = node.required("meta")?;
            read_thinkthen_complete_meta_v1(value, store).map(|value| store.hold(value))?
        },
        question: {
            let value = node.required("question")?;
            read_thinkthen_complete_question_document_v1(value, store)
                .map(|value| store.hold(value))?
        },
        question_name: {
            let (presence, value) = read_presence(
                node.member("question_name"),
                false,
                thinkthen_complete_utf8_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence_v1 { presence, value }
        },
        schema: {
            let value = node.required("schema")?;
            read_thinkthen_complete_version_v1(value, store).map(|value| store.hold(value))?
        },
        source: {
            let (presence, value) = read_presence(
                node.member("source"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_physical_source_v1(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_physical_source_presence_v1 { presence, value }
        },
        threshold: {
            let (presence, value) = read_presence(
                node.member("threshold"),
                true,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_threshold_v1(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_threshold_presence_v1 { presence, value }
        },
        value: {
            let (presence, value) = read_presence(
                node.member("value"),
                true,
                thinkthen_complete_utf8_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence_v1 { presence, value }
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
pub struct thinkthen_complete_atomic_boolean_v1 {
    pub answer: *const thinkthen_complete_answer_v1,
    pub answer_id: *const thinkthen_complete_answer_id_v1,
    pub images: thinkthen_complete_image_list_presence_v1,
    pub index: thinkthen_complete_u64_presence_v1,
    pub input: thinkthen_complete_json_presence_v1,
    pub members: thinkthen_complete_rank_member_list_presence_v1,
    pub meta: *const thinkthen_complete_meta_v1,
    pub question: *const thinkthen_complete_question_document_v1,
    pub question_name: thinkthen_complete_utf8_presence_v1,
    pub schema: *const thinkthen_complete_version_v1,
    pub source: thinkthen_complete_physical_source_presence_v1,
    pub threshold: thinkthen_complete_threshold_presence_v1,
    pub value: u32,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_atomic_boolean_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_atomic_boolean_v1, ErrorKind> {
    Ok(thinkthen_complete_atomic_boolean_v1 {
        answer: {
            let value = node.required("answer")?;
            read_thinkthen_complete_answer_v1(value, store).map(|value| store.hold(value))?
        },
        answer_id: {
            let value = node.required("answer_id")?;
            read_thinkthen_complete_answer_id_v1(value, store).map(|value| store.hold(value))?
        },
        images: {
            let (presence, value) = read_presence(
                node.member("images"),
                false,
                thinkthen_complete_image_list_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_image_list_v1,
                store,
            )?;
            thinkthen_complete_image_list_presence_v1 { presence, value }
        },
        index: {
            let (presence, value) =
                read_presence(node.member("index"), false, 0, read_number, store)?;
            thinkthen_complete_u64_presence_v1 { presence, value }
        },
        input: {
            let (presence, value) = read_presence(
                node.member("input"),
                false,
                std::ptr::null(),
                read_json,
                store,
            )?;
            thinkthen_complete_json_presence_v1 { presence, value }
        },
        members: {
            let (presence, value) = read_presence(
                node.member("members"),
                false,
                thinkthen_complete_rank_member_list_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_rank_member_list_v1,
                store,
            )?;
            thinkthen_complete_rank_member_list_presence_v1 { presence, value }
        },
        meta: {
            let value = node.required("meta")?;
            read_thinkthen_complete_meta_v1(value, store).map(|value| store.hold(value))?
        },
        question: {
            let value = node.required("question")?;
            read_thinkthen_complete_question_document_v1(value, store)
                .map(|value| store.hold(value))?
        },
        question_name: {
            let (presence, value) = read_presence(
                node.member("question_name"),
                false,
                thinkthen_complete_utf8_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence_v1 { presence, value }
        },
        schema: {
            let value = node.required("schema")?;
            read_thinkthen_complete_version_v1(value, store).map(|value| store.hold(value))?
        },
        source: {
            let (presence, value) = read_presence(
                node.member("source"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_physical_source_v1(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_physical_source_presence_v1 { presence, value }
        },
        threshold: {
            let (presence, value) = read_presence(
                node.member("threshold"),
                true,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_threshold_v1(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_threshold_presence_v1 { presence, value }
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
pub struct thinkthen_complete_atomic_double_v1 {
    pub answer: *const thinkthen_complete_answer_v1,
    pub answer_id: *const thinkthen_complete_answer_id_v1,
    pub images: thinkthen_complete_image_list_presence_v1,
    pub index: thinkthen_complete_u64_presence_v1,
    pub input: thinkthen_complete_json_presence_v1,
    pub members: thinkthen_complete_rank_member_list_presence_v1,
    pub meta: *const thinkthen_complete_meta_v1,
    pub question: *const thinkthen_complete_question_document_v1,
    pub question_name: thinkthen_complete_utf8_presence_v1,
    pub schema: *const thinkthen_complete_version_v1,
    pub source: thinkthen_complete_physical_source_presence_v1,
    pub threshold: thinkthen_complete_threshold_presence_v1,
    pub value: f64,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_atomic_double_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_atomic_double_v1, ErrorKind> {
    Ok(thinkthen_complete_atomic_double_v1 {
        answer: {
            let value = node.required("answer")?;
            read_thinkthen_complete_answer_v1(value, store).map(|value| store.hold(value))?
        },
        answer_id: {
            let value = node.required("answer_id")?;
            read_thinkthen_complete_answer_id_v1(value, store).map(|value| store.hold(value))?
        },
        images: {
            let (presence, value) = read_presence(
                node.member("images"),
                false,
                thinkthen_complete_image_list_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_image_list_v1,
                store,
            )?;
            thinkthen_complete_image_list_presence_v1 { presence, value }
        },
        index: {
            let (presence, value) =
                read_presence(node.member("index"), false, 0, read_number, store)?;
            thinkthen_complete_u64_presence_v1 { presence, value }
        },
        input: {
            let (presence, value) = read_presence(
                node.member("input"),
                false,
                std::ptr::null(),
                read_json,
                store,
            )?;
            thinkthen_complete_json_presence_v1 { presence, value }
        },
        members: {
            let (presence, value) = read_presence(
                node.member("members"),
                false,
                thinkthen_complete_rank_member_list_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_rank_member_list_v1,
                store,
            )?;
            thinkthen_complete_rank_member_list_presence_v1 { presence, value }
        },
        meta: {
            let value = node.required("meta")?;
            read_thinkthen_complete_meta_v1(value, store).map(|value| store.hold(value))?
        },
        question: {
            let value = node.required("question")?;
            read_thinkthen_complete_question_document_v1(value, store)
                .map(|value| store.hold(value))?
        },
        question_name: {
            let (presence, value) = read_presence(
                node.member("question_name"),
                false,
                thinkthen_complete_utf8_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence_v1 { presence, value }
        },
        schema: {
            let value = node.required("schema")?;
            read_thinkthen_complete_version_v1(value, store).map(|value| store.hold(value))?
        },
        source: {
            let (presence, value) = read_presence(
                node.member("source"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_physical_source_v1(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_physical_source_presence_v1 { presence, value }
        },
        threshold: {
            let (presence, value) = read_presence(
                node.member("threshold"),
                true,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_threshold_v1(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_threshold_presence_v1 { presence, value }
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
pub struct thinkthen_complete_attempt_v1 {
    pub ordinal: u64,
    pub outcome: *const thinkthen_complete_attempt_outcome_v1,
    pub request_id: thinkthen_complete_utf8_presence_v1,
    pub request_sha256: thinkthen_complete_utf8_v1,
    pub sdk_request_id: *const thinkthen_complete_sdk_request_id_v1,
    pub server_ms: thinkthen_complete_u64_presence_v1,
    pub status: thinkthen_complete_u64_presence_v1,
    pub wall_ms: u64,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_attempt_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_attempt_v1, ErrorKind> {
    Ok(thinkthen_complete_attempt_v1 {
        ordinal: {
            let value = node.required("ordinal")?;
            read_number(value, store)?
        },
        outcome: {
            let value = node.required("outcome")?;
            read_thinkthen_complete_attempt_outcome_v1(value, store)
                .map(|value| store.hold(value))?
        },
        request_id: {
            let (presence, value) = read_presence(
                node.member("request_id"),
                false,
                thinkthen_complete_utf8_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence_v1 { presence, value }
        },
        request_sha256: {
            let value = node.required("request_sha256")?;
            read_text(value, store)?
        },
        sdk_request_id: {
            let value = node.required("sdk_request_id")?;
            read_thinkthen_complete_sdk_request_id_v1(value, store)
                .map(|value| store.hold(value))?
        },
        server_ms: {
            let (presence, value) =
                read_presence(node.member("server_ms"), false, 0, read_number, store)?;
            thinkthen_complete_u64_presence_v1 { presence, value }
        },
        status: {
            let (presence, value) =
                read_presence(node.member("status"), false, 0, read_number, store)?;
            thinkthen_complete_u64_presence_v1 { presence, value }
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
pub struct thinkthen_complete_batch_string_v1 {
    pub kind: u32,
}

pub const THINKTHEN_COMPLETE_BATCH_STRING_MAX_V1: u32 = 1;

pub(crate) fn read_thinkthen_complete_batch_string_v1(
    node: &Node,
    _store: &mut Storage,
) -> Result<thinkthen_complete_batch_string_v1, ErrorKind> {
    let kind = match node.text()? {
        "max" => THINKTHEN_COMPLETE_BATCH_STRING_MAX_V1,
        _ => return Err(ErrorKind::Defect),
    };
    Ok(thinkthen_complete_batch_string_v1 { kind })
}

pub const THINKTHEN_COMPLETE_BATCH_INTEGER_V1: u32 = 1;

pub const THINKTHEN_COMPLETE_BATCH_STRING_V1: u32 = 2;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_batch_data_v1 {
    pub integer: u64,
    pub string: *const thinkthen_complete_batch_string_v1,
}
impl std::fmt::Debug for thinkthen_complete_batch_data_v1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("union payload")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_batch_v1 {
    pub kind: u32,
    pub data: thinkthen_complete_batch_data_v1,
}

pub(crate) fn read_thinkthen_complete_batch_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_batch_v1, ErrorKind> {
    if node.kind() == "number" {
        let value = read_number(node, store)?;
        return Ok(thinkthen_complete_batch_v1 {
            kind: THINKTHEN_COMPLETE_BATCH_INTEGER_V1,
            data: thinkthen_complete_batch_data_v1 { integer: value },
        });
    }
    if node.kind() == "string" {
        let value =
            read_thinkthen_complete_batch_string_v1(node, store).map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_batch_v1 {
            kind: THINKTHEN_COMPLETE_BATCH_STRING_V1,
            data: thinkthen_complete_batch_data_v1 { string: value },
        });
    }
    Err(ErrorKind::Defect)
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_boundary_mode_v1 {
    pub kind: u32,
}

pub const THINKTHEN_COMPLETE_BOUNDARY_MODE_BOUNDARY_ONLY_V1: u32 = 1;

pub(crate) fn read_thinkthen_complete_boundary_mode_v1(
    node: &Node,
    _store: &mut Storage,
) -> Result<thinkthen_complete_boundary_mode_v1, ErrorKind> {
    let kind = match node.text()? {
        "boundary_only" => THINKTHEN_COMPLETE_BOUNDARY_MODE_BOUNDARY_ONLY_V1,
        _ => return Err(ErrorKind::Defect),
    };
    Ok(thinkthen_complete_boundary_mode_v1 { kind })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_boundary_proposal_v1 {
    pub end: u64,
    pub length: u64,
    pub probability: f64,
    pub start: u64,
    pub text: thinkthen_complete_utf8_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_boundary_proposal_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_boundary_proposal_v1, ErrorKind> {
    Ok(thinkthen_complete_boundary_proposal_v1 {
        end: {
            let value = node.required("end")?;
            read_number(value, store)?
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
        extensions: store.extensions(node, &["end", "length", "probability", "start", "text"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_facts_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_facts_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_call_error_v1 {
    pub error: *const thinkthen_complete_error_v1,
    pub facts: thinkthen_complete_facts_presence_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_call_error_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_call_error_v1, ErrorKind> {
    Ok(thinkthen_complete_call_error_v1 {
        error: {
            let value = node.required("error")?;
            read_thinkthen_complete_error_v1(value, store).map(|value| store.hold(value))?
        },
        facts: {
            let (presence, value) = read_presence(
                node.member("facts"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_facts_v1(value, store).map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_facts_presence_v1 { presence, value }
        },
        extensions: store.extensions(node, &["error", "facts"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_call_id_v1 {
    pub value: thinkthen_complete_utf8_v1,
}

pub(crate) fn read_thinkthen_complete_call_id_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_call_id_v1, ErrorKind> {
    let value = read_text(node, store)?;
    Ok(thinkthen_complete_call_id_v1 { value })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_decide_value_v1 {
    pub value: *const thinkthen_complete_json_v1,
}

pub(crate) fn read_thinkthen_complete_decide_value_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_decide_value_v1, ErrorKind> {
    let value = read_json(node, store)?;
    Ok(thinkthen_complete_decide_value_v1 { value })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_entity_document_v1 {
    pub kind: thinkthen_complete_utf8_v1,
    pub name: thinkthen_complete_utf8_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_entity_document_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_entity_document_v1, ErrorKind> {
    Ok(thinkthen_complete_entity_document_v1 {
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
pub struct thinkthen_complete_input_denial_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_input_denial_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_send_denial_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_send_denial_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_error_v1 {
    pub estimated_input_denial: thinkthen_complete_input_denial_presence_v1,
    pub kind: *const thinkthen_complete_failure_kind_v1,
    pub message: thinkthen_complete_utf8_v1,
    pub retryable: u32,
    pub send_budget_denial: thinkthen_complete_send_denial_presence_v1,
    pub stopped: *const thinkthen_complete_stopped_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_error_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_error_v1, ErrorKind> {
    Ok(thinkthen_complete_error_v1 {
        estimated_input_denial: {
            let (presence, value) = read_presence(
                node.member("estimated_input_denial"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_input_denial_v1(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_input_denial_presence_v1 { presence, value }
        },
        kind: {
            let value = node.required("kind")?;
            read_thinkthen_complete_failure_kind_v1(value, store).map(|value| store.hold(value))?
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
                    read_thinkthen_complete_send_denial_v1(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_send_denial_presence_v1 { presence, value }
        },
        stopped: {
            let value = node.required("stopped")?;
            read_thinkthen_complete_stopped_v1(value, store).map(|value| store.hold(value))?
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

pub const THINKTHEN_COMPLETE_INPUT_DENIAL_INITIAL_REQUEST_V1: u32 = 1;

pub const THINKTHEN_COMPLETE_INPUT_DENIAL_ADDITIONAL_REQUEST_V1: u32 = 2;

pub const THINKTHEN_COMPLETE_INPUT_DENIAL_RETRY_V1: u32 = 3;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_input_denial_data_v1 {
    pub initial_request: *const thinkthen_complete_input_denial_initial_request_v1,
    pub additional_request: *const thinkthen_complete_input_denial_additional_request_v1,
    pub retry: *const thinkthen_complete_input_denial_retry_v1,
}
impl std::fmt::Debug for thinkthen_complete_input_denial_data_v1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("union payload")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_input_denial_v1 {
    pub kind: u32,
    pub data: thinkthen_complete_input_denial_data_v1,
}

pub(crate) fn read_thinkthen_complete_input_denial_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_input_denial_v1, ErrorKind> {
    if node.is_member("kind", "initial_request") {
        let value = read_thinkthen_complete_input_denial_initial_request_v1(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_input_denial_v1 {
            kind: THINKTHEN_COMPLETE_INPUT_DENIAL_INITIAL_REQUEST_V1,
            data: thinkthen_complete_input_denial_data_v1 {
                initial_request: value,
            },
        });
    }
    if node.is_member("kind", "additional_request") {
        let value = read_thinkthen_complete_input_denial_additional_request_v1(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_input_denial_v1 {
            kind: THINKTHEN_COMPLETE_INPUT_DENIAL_ADDITIONAL_REQUEST_V1,
            data: thinkthen_complete_input_denial_data_v1 {
                additional_request: value,
            },
        });
    }
    if node.is_member("kind", "retry") {
        let value = read_thinkthen_complete_input_denial_retry_v1(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_input_denial_v1 {
            kind: THINKTHEN_COMPLETE_INPUT_DENIAL_RETRY_V1,
            data: thinkthen_complete_input_denial_data_v1 { retry: value },
        });
    }
    Err(ErrorKind::Defect)
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_input_denial_additional_request_v1 {
    pub kind: thinkthen_complete_utf8_v1,
    pub limit: u64,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_input_denial_additional_request_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_input_denial_additional_request_v1, ErrorKind> {
    Ok(thinkthen_complete_input_denial_additional_request_v1 {
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
pub struct thinkthen_complete_input_denial_initial_request_v1 {
    pub kind: thinkthen_complete_utf8_v1,
    pub limit: u64,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_input_denial_initial_request_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_input_denial_initial_request_v1, ErrorKind> {
    Ok(thinkthen_complete_input_denial_initial_request_v1 {
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
pub struct thinkthen_complete_input_denial_retry_v1 {
    pub kind: thinkthen_complete_utf8_v1,
    pub last_status: u64,
    pub limit: u64,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_input_denial_retry_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_input_denial_retry_v1, ErrorKind> {
    Ok(thinkthen_complete_input_denial_retry_v1 {
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
pub struct thinkthen_complete_attempt_list_v1 {
    pub data: *const *const thinkthen_complete_attempt_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_attempt_list_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_attempt_list_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values
            .push(read_thinkthen_complete_attempt_v1(item, store).map(|value| store.hold(value))?);
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_attempt_list_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_attempt_list_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_attempt_list_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_persistence_observation_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_persistence_observation_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_facts_v1 {
    pub attempts: thinkthen_complete_attempt_list_presence_v1,
    pub cache_answers: u64,
    pub call_id: *const thinkthen_complete_call_id_v1,
    pub estimated_cost_usd: thinkthen_complete_utf8_presence_v1,
    pub held_model_mismatch: thinkthen_complete_u32_presence_v1,
    pub input_tokens: thinkthen_complete_u64_presence_v1,
    pub largest_request_bytes: u64,
    pub largest_request_estimated_input_tokens: thinkthen_complete_u64_presence_v1,
    pub model: thinkthen_complete_utf8_presence_v1,
    pub output_tokens: thinkthen_complete_u64_presence_v1,
    pub records: u64,
    pub requests_sent: u64,
    pub seconds: f64,
    pub token_estimate_method: thinkthen_complete_utf8_v1,
    pub usage_persistence: thinkthen_complete_persistence_observation_presence_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_facts_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_facts_v1, ErrorKind> {
    Ok(thinkthen_complete_facts_v1 {
        attempts: {
            let (presence, value) = read_presence(
                node.member("attempts"),
                false,
                thinkthen_complete_attempt_list_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_attempt_list_v1,
                store,
            )?;
            thinkthen_complete_attempt_list_presence_v1 { presence, value }
        },
        cache_answers: {
            let value = node.required("cache_answers")?;
            read_number(value, store)?
        },
        call_id: {
            let value = node.required("call_id")?;
            read_thinkthen_complete_call_id_v1(value, store).map(|value| store.hold(value))?
        },
        estimated_cost_usd: {
            let (presence, value) = read_presence(
                node.member("estimated_cost_usd"),
                false,
                thinkthen_complete_utf8_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence_v1 { presence, value }
        },
        held_model_mismatch: {
            let (presence, value) = read_presence(
                node.member("held_model_mismatch"),
                false,
                0,
                read_boolean,
                store,
            )?;
            thinkthen_complete_u32_presence_v1 { presence, value }
        },
        input_tokens: {
            let (presence, value) =
                read_presence(node.member("input_tokens"), false, 0, read_number, store)?;
            thinkthen_complete_u64_presence_v1 { presence, value }
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
            thinkthen_complete_u64_presence_v1 { presence, value }
        },
        model: {
            let (presence, value) = read_presence(
                node.member("model"),
                false,
                thinkthen_complete_utf8_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence_v1 { presence, value }
        },
        output_tokens: {
            let (presence, value) =
                read_presence(node.member("output_tokens"), false, 0, read_number, store)?;
            thinkthen_complete_u64_presence_v1 { presence, value }
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
                    read_thinkthen_complete_persistence_observation_v1(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_persistence_observation_presence_v1 { presence, value }
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
pub struct thinkthen_complete_failure_id_v1 {
    pub value: thinkthen_complete_utf8_v1,
}

pub(crate) fn read_thinkthen_complete_failure_id_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_failure_id_v1, ErrorKind> {
    let value = read_text(node, store)?;
    Ok(thinkthen_complete_failure_id_v1 { value })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_find_candidate_list_v1 {
    pub data: *const *const thinkthen_complete_find_candidate_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_find_candidate_list_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_find_candidate_list_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_find_candidate_v1(item, store)
                .map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_find_candidate_list_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_find_candidate_list_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_find_candidate_list_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_find_v1 {
    pub answer: *const thinkthen_complete_find_answer_v1,
    pub answer_id: *const thinkthen_complete_answer_id_v1,
    pub candidates: thinkthen_complete_find_candidate_list_presence_v1,
    pub file: thinkthen_complete_utf8_presence_v1,
    pub first_line: thinkthen_complete_u64_presence_v1,
    pub index: thinkthen_complete_u64_presence_v1,
    pub last_line: thinkthen_complete_u64_presence_v1,
    pub meta: *const thinkthen_complete_meta_v1,
    pub position: thinkthen_complete_position_presence_v1,
    pub question: *const thinkthen_complete_find_question_document_v1,
    pub schema: *const thinkthen_complete_version_v1,
    pub threshold: thinkthen_complete_json_presence_v1,
    pub value: thinkthen_complete_json_presence_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_find_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_find_v1, ErrorKind> {
    Ok(thinkthen_complete_find_v1 {
        answer: {
            let value = node.required("answer")?;
            read_thinkthen_complete_find_answer_v1(value, store).map(|value| store.hold(value))?
        },
        answer_id: {
            let value = node.required("answer_id")?;
            read_thinkthen_complete_answer_id_v1(value, store).map(|value| store.hold(value))?
        },
        candidates: {
            let (presence, value) = read_presence(
                node.member("candidates"),
                false,
                thinkthen_complete_find_candidate_list_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_find_candidate_list_v1,
                store,
            )?;
            thinkthen_complete_find_candidate_list_presence_v1 { presence, value }
        },
        file: {
            let (presence, value) = read_presence(
                node.member("file"),
                false,
                thinkthen_complete_utf8_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence_v1 { presence, value }
        },
        first_line: {
            let (presence, value) =
                read_presence(node.member("first_line"), false, 0, read_number, store)?;
            thinkthen_complete_u64_presence_v1 { presence, value }
        },
        index: {
            let (presence, value) =
                read_presence(node.member("index"), true, 0, read_number, store)?;
            thinkthen_complete_u64_presence_v1 { presence, value }
        },
        last_line: {
            let (presence, value) =
                read_presence(node.member("last_line"), false, 0, read_number, store)?;
            thinkthen_complete_u64_presence_v1 { presence, value }
        },
        meta: {
            let value = node.required("meta")?;
            read_thinkthen_complete_meta_v1(value, store).map(|value| store.hold(value))?
        },
        position: {
            let (presence, value) = read_presence(
                node.member("position"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_position_v1(value, store).map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_position_presence_v1 { presence, value }
        },
        question: {
            let value = node.required("question")?;
            read_thinkthen_complete_find_question_document_v1(value, store)
                .map(|value| store.hold(value))?
        },
        schema: {
            let value = node.required("schema")?;
            read_thinkthen_complete_version_v1(value, store).map(|value| store.hold(value))?
        },
        threshold: {
            let (presence, value) = read_presence(
                node.member("threshold"),
                true,
                std::ptr::null(),
                read_json,
                store,
            )?;
            thinkthen_complete_json_presence_v1 { presence, value }
        },
        value: {
            let (presence, value) = read_presence(
                node.member("value"),
                true,
                std::ptr::null(),
                read_json,
                store,
            )?;
            thinkthen_complete_json_presence_v1 { presence, value }
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
pub struct thinkthen_complete_find_candidate_v1 {
    pub index: thinkthen_complete_u64_presence_v1,
    pub input: thinkthen_complete_json_presence_v1,
    pub probability: f64,
    pub source: thinkthen_complete_physical_source_presence_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_find_candidate_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_find_candidate_v1, ErrorKind> {
    Ok(thinkthen_complete_find_candidate_v1 {
        index: {
            let (presence, value) =
                read_presence(node.member("index"), true, 0, read_number, store)?;
            thinkthen_complete_u64_presence_v1 { presence, value }
        },
        input: {
            let (presence, value) = read_presence(
                node.member("input"),
                true,
                std::ptr::null(),
                read_json,
                store,
            )?;
            thinkthen_complete_json_presence_v1 { presence, value }
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
                    read_thinkthen_complete_physical_source_v1(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_physical_source_presence_v1 { presence, value }
        },
        extensions: store.extensions(node, &["index", "input", "probability", "source"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_image_v1 {
    pub base64: thinkthen_complete_utf8_v1,
    pub height: u64,
    pub media: *const thinkthen_complete_image_media_v1,
    pub width: u64,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_image_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_image_v1, ErrorKind> {
    Ok(thinkthen_complete_image_v1 {
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
            read_thinkthen_complete_image_media_v1(value, store).map(|value| store.hold(value))?
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
pub struct thinkthen_complete_image_media_v1 {
    pub kind: u32,
}

pub const THINKTHEN_COMPLETE_IMAGE_MEDIA_IMAGE_JPEG_V1: u32 = 1;

pub const THINKTHEN_COMPLETE_IMAGE_MEDIA_IMAGE_PNG_V1: u32 = 2;

pub(crate) fn read_thinkthen_complete_image_media_v1(
    node: &Node,
    _store: &mut Storage,
) -> Result<thinkthen_complete_image_media_v1, ErrorKind> {
    let kind = match node.text()? {
        "image/jpeg" => THINKTHEN_COMPLETE_IMAGE_MEDIA_IMAGE_JPEG_V1,
        "image/png" => THINKTHEN_COMPLETE_IMAGE_MEDIA_IMAGE_PNG_V1,
        _ => return Err(ErrorKind::Defect),
    };
    Ok(thinkthen_complete_image_media_v1 { kind })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_call_id_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_call_id_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_immediate_facts_v1 {
    pub attempts: thinkthen_complete_attempt_list_presence_v1,
    pub cache_answers: u64,
    pub call_id: thinkthen_complete_call_id_presence_v1,
    pub estimated_cost_usd: thinkthen_complete_utf8_presence_v1,
    pub held_model_mismatch: thinkthen_complete_u32_presence_v1,
    pub input_tokens: thinkthen_complete_u64_presence_v1,
    pub largest_request_bytes: u64,
    pub largest_request_estimated_input_tokens: thinkthen_complete_u64_presence_v1,
    pub model: thinkthen_complete_utf8_presence_v1,
    pub output_tokens: thinkthen_complete_u64_presence_v1,
    pub records: u64,
    pub requests_sent: u64,
    pub seconds: f64,
    pub token_estimate_method: thinkthen_complete_utf8_v1,
    pub usage_persistence: thinkthen_complete_persistence_observation_presence_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_immediate_facts_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_immediate_facts_v1, ErrorKind> {
    Ok(thinkthen_complete_immediate_facts_v1 {
        attempts: {
            let (presence, value) = read_presence(
                node.member("attempts"),
                false,
                thinkthen_complete_attempt_list_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_attempt_list_v1,
                store,
            )?;
            thinkthen_complete_attempt_list_presence_v1 { presence, value }
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
                    read_thinkthen_complete_call_id_v1(value, store).map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_call_id_presence_v1 { presence, value }
        },
        estimated_cost_usd: {
            let (presence, value) = read_presence(
                node.member("estimated_cost_usd"),
                false,
                thinkthen_complete_utf8_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence_v1 { presence, value }
        },
        held_model_mismatch: {
            let (presence, value) = read_presence(
                node.member("held_model_mismatch"),
                false,
                0,
                read_boolean,
                store,
            )?;
            thinkthen_complete_u32_presence_v1 { presence, value }
        },
        input_tokens: {
            let (presence, value) =
                read_presence(node.member("input_tokens"), false, 0, read_number, store)?;
            thinkthen_complete_u64_presence_v1 { presence, value }
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
            thinkthen_complete_u64_presence_v1 { presence, value }
        },
        model: {
            let (presence, value) = read_presence(
                node.member("model"),
                false,
                thinkthen_complete_utf8_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence_v1 { presence, value }
        },
        output_tokens: {
            let (presence, value) =
                read_presence(node.member("output_tokens"), false, 0, read_number, store)?;
            thinkthen_complete_u64_presence_v1 { presence, value }
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
                    read_thinkthen_complete_persistence_observation_v1(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_persistence_observation_presence_v1 { presence, value }
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

pub const THINKTHEN_COMPLETE_INPUT_DECLARATION_STRING_ROOT_V1: u32 = 1;

pub const THINKTHEN_COMPLETE_INPUT_DECLARATION_OBJECT_ROOT_V1: u32 = 2;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_input_declaration_data_v1 {
    pub string_root: *const thinkthen_complete_input_declaration_string_root_v1,
    pub object_root: *const thinkthen_complete_input_declaration_object_root_v1,
}
impl std::fmt::Debug for thinkthen_complete_input_declaration_data_v1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("union payload")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_input_declaration_v1 {
    pub kind: u32,
    pub data: thinkthen_complete_input_declaration_data_v1,
}

pub(crate) fn read_thinkthen_complete_input_declaration_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_input_declaration_v1, ErrorKind> {
    if node.is_member("type", "string") {
        let value = read_thinkthen_complete_input_declaration_string_root_v1(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_input_declaration_v1 {
            kind: THINKTHEN_COMPLETE_INPUT_DECLARATION_STRING_ROOT_V1,
            data: thinkthen_complete_input_declaration_data_v1 { string_root: value },
        });
    }
    if node.is_member("type", "object") {
        let value = read_thinkthen_complete_input_declaration_object_root_v1(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_input_declaration_v1 {
            kind: THINKTHEN_COMPLETE_INPUT_DECLARATION_OBJECT_ROOT_V1,
            data: thinkthen_complete_input_declaration_data_v1 { object_root: value },
        });
    }
    Err(ErrorKind::Defect)
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_input_property_type_map_entry_v1 {
    pub name: thinkthen_complete_utf8_v1,
    pub value: *const thinkthen_complete_input_property_type_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_input_property_type_map_v1 {
    pub data: *const thinkthen_complete_input_property_type_map_entry_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_input_property_type_map_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_input_property_type_map_v1, ErrorKind> {
    let mut values = Vec::new();
    for (key, item) in node.object()? {
        values.push(thinkthen_complete_input_property_type_map_entry_v1 {
            name: store.text(key),
            value: read_thinkthen_complete_input_property_type_v1(item, store)
                .map(|value| store.hold(value))?,
        });
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_input_property_type_map_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_string_list_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_string_list_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_input_declaration_object_root_v1 {
    pub properties: thinkthen_complete_input_property_type_map_v1,
    pub required: thinkthen_complete_string_list_presence_v1,
    pub r#type: *const thinkthen_complete_object_type_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_input_declaration_object_root_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_input_declaration_object_root_v1, ErrorKind> {
    Ok(thinkthen_complete_input_declaration_object_root_v1 {
        properties: {
            let value = node.required("properties")?;
            read_thinkthen_complete_input_property_type_map_v1(value, store)?
        },
        required: {
            let (presence, value) = read_presence(
                node.member("required"),
                false,
                thinkthen_complete_string_list_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_string_list_v1,
                store,
            )?;
            thinkthen_complete_string_list_presence_v1 { presence, value }
        },
        r#type: {
            let value = node.required("type")?;
            read_thinkthen_complete_object_type_v1(value, store).map(|value| store.hold(value))?
        },
        extensions: store.extensions(node, &["properties", "required", "type"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_input_declaration_string_root_v1 {
    pub r#type: *const thinkthen_complete_string_type_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_input_declaration_string_root_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_input_declaration_string_root_v1, ErrorKind> {
    Ok(thinkthen_complete_input_declaration_string_root_v1 {
        r#type: {
            let value = node.required("type")?;
            read_thinkthen_complete_string_type_v1(value, store).map(|value| store.hold(value))?
        },
        extensions: store.extensions(node, &["type"])?,
    })
}

pub const THINKTHEN_COMPLETE_INPUT_PROPERTY_TYPE_STRING_V1: u32 = 1;

pub const THINKTHEN_COMPLETE_INPUT_PROPERTY_TYPE_NUMBER_V1: u32 = 2;

pub const THINKTHEN_COMPLETE_INPUT_PROPERTY_TYPE_BOOLEAN_V1: u32 = 3;

pub const THINKTHEN_COMPLETE_INPUT_PROPERTY_TYPE_ARRAY_V1: u32 = 4;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_input_property_type_data_v1 {
    pub string: *const thinkthen_complete_input_property_type_string_v1,
    pub number: *const thinkthen_complete_input_property_type_number_v1,
    pub boolean: *const thinkthen_complete_input_property_type_boolean_v1,
    pub array: *const thinkthen_complete_input_property_type_array_v1,
}
impl std::fmt::Debug for thinkthen_complete_input_property_type_data_v1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("union payload")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_input_property_type_v1 {
    pub kind: u32,
    pub data: thinkthen_complete_input_property_type_data_v1,
}

pub(crate) fn read_thinkthen_complete_input_property_type_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_input_property_type_v1, ErrorKind> {
    if node.is_member("type", "string") {
        let value = read_thinkthen_complete_input_property_type_string_v1(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_input_property_type_v1 {
            kind: THINKTHEN_COMPLETE_INPUT_PROPERTY_TYPE_STRING_V1,
            data: thinkthen_complete_input_property_type_data_v1 { string: value },
        });
    }
    if node.is_member("type", "number") {
        let value = read_thinkthen_complete_input_property_type_number_v1(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_input_property_type_v1 {
            kind: THINKTHEN_COMPLETE_INPUT_PROPERTY_TYPE_NUMBER_V1,
            data: thinkthen_complete_input_property_type_data_v1 { number: value },
        });
    }
    if node.is_member("type", "boolean") {
        let value = read_thinkthen_complete_input_property_type_boolean_v1(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_input_property_type_v1 {
            kind: THINKTHEN_COMPLETE_INPUT_PROPERTY_TYPE_BOOLEAN_V1,
            data: thinkthen_complete_input_property_type_data_v1 { boolean: value },
        });
    }
    if node.is_member("type", "array") {
        let value = read_thinkthen_complete_input_property_type_array_v1(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_input_property_type_v1 {
            kind: THINKTHEN_COMPLETE_INPUT_PROPERTY_TYPE_ARRAY_V1,
            data: thinkthen_complete_input_property_type_data_v1 { array: value },
        });
    }
    Err(ErrorKind::Defect)
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_input_property_type_array_v1 {
    pub items: *const thinkthen_complete_string_root_v1,
    pub r#type: thinkthen_complete_utf8_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_input_property_type_array_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_input_property_type_array_v1, ErrorKind> {
    Ok(thinkthen_complete_input_property_type_array_v1 {
        items: {
            let value = node.required("items")?;
            read_thinkthen_complete_string_root_v1(value, store).map(|value| store.hold(value))?
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
pub struct thinkthen_complete_input_property_type_boolean_v1 {
    pub r#type: thinkthen_complete_utf8_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_input_property_type_boolean_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_input_property_type_boolean_v1, ErrorKind> {
    Ok(thinkthen_complete_input_property_type_boolean_v1 {
        r#type: {
            let value = node.required("type")?;
            read_text(value, store)?
        },
        extensions: store.extensions(node, &["type"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_input_property_type_number_v1 {
    pub r#type: thinkthen_complete_utf8_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_input_property_type_number_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_input_property_type_number_v1, ErrorKind> {
    Ok(thinkthen_complete_input_property_type_number_v1 {
        r#type: {
            let value = node.required("type")?;
            read_text(value, store)?
        },
        extensions: store.extensions(node, &["type"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_input_property_type_string_v1 {
    pub r#type: thinkthen_complete_utf8_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_input_property_type_string_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_input_property_type_string_v1, ErrorKind> {
    Ok(thinkthen_complete_input_property_type_string_v1 {
        r#type: {
            let value = node.required("type")?;
            read_text(value, store)?
        },
        extensions: store.extensions(node, &["type"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_label_v1 {
    pub description: thinkthen_complete_json_presence_v1,
    pub name: thinkthen_complete_utf8_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_label_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_label_v1, ErrorKind> {
    Ok(thinkthen_complete_label_v1 {
        description: {
            let (presence, value) = read_presence(
                node.member("description"),
                false,
                std::ptr::null(),
                read_json,
                store,
            )?;
            thinkthen_complete_json_presence_v1 { presence, value }
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
pub struct thinkthen_complete_batch_setting_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_batch_setting_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_batch_warning_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_batch_warning_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_origin_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_origin_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_profile_warning_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_profile_warning_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_meta_v1 {
    pub answered_by: thinkthen_complete_utf8_presence_v1,
    pub attempts: thinkthen_complete_attempt_list_presence_v1,
    pub batch_setting: thinkthen_complete_batch_setting_presence_v1,
    pub batch_warning: thinkthen_complete_batch_warning_presence_v1,
    pub cached: u32,
    pub context_sha256: thinkthen_complete_utf8_presence_v1,
    pub failed_questions: u64,
    pub model: thinkthen_complete_utf8_v1,
    pub observations: thinkthen_complete_observation_list_v1,
    pub origin: thinkthen_complete_origin_presence_v1,
    pub profile_warning: thinkthen_complete_profile_warning_presence_v1,
    pub question_sha256: thinkthen_complete_utf8_presence_v1,
    pub question_sources: thinkthen_complete_question_source_list_v1,
    pub questions_sha256: thinkthen_complete_utf8_presence_v1,
    pub requests: thinkthen_complete_string_list_v1,
    pub requests_sent: u64,
    pub tool: thinkthen_complete_utf8_v1,
    pub url: thinkthen_complete_utf8_v1,
    pub usage: thinkthen_complete_usage_presence_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_meta_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_meta_v1, ErrorKind> {
    Ok(thinkthen_complete_meta_v1 {
        answered_by: {
            let (presence, value) = read_presence(
                node.member("answered_by"),
                false,
                thinkthen_complete_utf8_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence_v1 { presence, value }
        },
        attempts: {
            let (presence, value) = read_presence(
                node.member("attempts"),
                false,
                thinkthen_complete_attempt_list_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_attempt_list_v1,
                store,
            )?;
            thinkthen_complete_attempt_list_presence_v1 { presence, value }
        },
        batch_setting: {
            let (presence, value) = read_presence(
                node.member("batch_setting"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_batch_setting_v1(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_batch_setting_presence_v1 { presence, value }
        },
        batch_warning: {
            let (presence, value) = read_presence(
                node.member("batch_warning"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_batch_warning_v1(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_batch_warning_presence_v1 { presence, value }
        },
        cached: {
            let value = node.required("cached")?;
            read_boolean(value, store)?
        },
        context_sha256: {
            let (presence, value) = read_presence(
                node.member("context_sha256"),
                false,
                thinkthen_complete_utf8_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence_v1 { presence, value }
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
            read_thinkthen_complete_observation_list_v1(value, store)?
        },
        origin: {
            let (presence, value) = read_presence(
                node.member("origin"),
                true,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_origin_v1(value, store).map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_origin_presence_v1 { presence, value }
        },
        profile_warning: {
            let (presence, value) = read_presence(
                node.member("profile_warning"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_profile_warning_v1(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_profile_warning_presence_v1 { presence, value }
        },
        question_sha256: {
            let (presence, value) = read_presence(
                node.member("question_sha256"),
                false,
                thinkthen_complete_utf8_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence_v1 { presence, value }
        },
        question_sources: {
            let value = node.required("question_sources")?;
            read_thinkthen_complete_question_source_list_v1(value, store)?
        },
        questions_sha256: {
            let (presence, value) = read_presence(
                node.member("questions_sha256"),
                false,
                thinkthen_complete_utf8_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence_v1 { presence, value }
        },
        requests: {
            let value = node.required("requests")?;
            read_thinkthen_complete_string_list_v1(value, store)?
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
                    read_thinkthen_complete_usage_v1(value, store).map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_usage_presence_v1 { presence, value }
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
pub struct thinkthen_complete_object_type_v1 {
    pub kind: u32,
}

pub const THINKTHEN_COMPLETE_OBJECT_TYPE_OBJECT_V1: u32 = 1;

pub(crate) fn read_thinkthen_complete_object_type_v1(
    node: &Node,
    _store: &mut Storage,
) -> Result<thinkthen_complete_object_type_v1, ErrorKind> {
    let kind = match node.text()? {
        "object" => THINKTHEN_COMPLETE_OBJECT_TYPE_OBJECT_V1,
        _ => return Err(ErrorKind::Defect),
    };
    Ok(thinkthen_complete_object_type_v1 { kind })
}

pub const THINKTHEN_COMPLETE_OBSERVATION_OBSERVATION_ID_V1: u32 = 1;

pub const THINKTHEN_COMPLETE_OBSERVATION_FAILURE_ID_V1: u32 = 2;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_observation_data_v1 {
    pub observation_id: *const thinkthen_complete_observation_observation_id_v1,
    pub failure_id: *const thinkthen_complete_observation_failure_id_v1,
}
impl std::fmt::Debug for thinkthen_complete_observation_data_v1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("union payload")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_observation_v1 {
    pub kind: u32,
    pub data: thinkthen_complete_observation_data_v1,
}

pub(crate) fn read_thinkthen_complete_observation_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_observation_v1, ErrorKind> {
    if node.member("observation_id").is_some() {
        let value = read_thinkthen_complete_observation_observation_id_v1(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_observation_v1 {
            kind: THINKTHEN_COMPLETE_OBSERVATION_OBSERVATION_ID_V1,
            data: thinkthen_complete_observation_data_v1 {
                observation_id: value,
            },
        });
    }
    if node.member("failure_id").is_some() {
        let value = read_thinkthen_complete_observation_failure_id_v1(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_observation_v1 {
            kind: THINKTHEN_COMPLETE_OBSERVATION_FAILURE_ID_V1,
            data: thinkthen_complete_observation_data_v1 { failure_id: value },
        });
    }
    Err(ErrorKind::Defect)
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_observation_id_v1 {
    pub value: thinkthen_complete_utf8_v1,
}

pub(crate) fn read_thinkthen_complete_observation_id_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_observation_id_v1, ErrorKind> {
    let value = read_text(node, store)?;
    Ok(thinkthen_complete_observation_id_v1 { value })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_observation_failure_id_v1 {
    pub failure_id: *const thinkthen_complete_failure_id_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_observation_failure_id_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_observation_failure_id_v1, ErrorKind> {
    Ok(thinkthen_complete_observation_failure_id_v1 {
        failure_id: {
            let value = node.required("failure_id")?;
            read_thinkthen_complete_failure_id_v1(value, store).map(|value| store.hold(value))?
        },
        extensions: store.extensions(node, &["failure_id"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_observation_observation_id_v1 {
    pub observation_id: *const thinkthen_complete_observation_id_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_observation_observation_id_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_observation_observation_id_v1, ErrorKind> {
    Ok(thinkthen_complete_observation_observation_id_v1 {
        observation_id: {
            let value = node.required("observation_id")?;
            read_thinkthen_complete_observation_id_v1(value, store)
                .map(|value| store.hold(value))?
        },
        extensions: store.extensions(node, &["observation_id"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_origin_v1 {
    pub kind: u32,
}

pub const THINKTHEN_COMPLETE_ORIGIN_LIVE_V1: u32 = 1;

pub const THINKTHEN_COMPLETE_ORIGIN_CACHE_V1: u32 = 2;

pub const THINKTHEN_COMPLETE_ORIGIN_REPLAY_V1: u32 = 3;

pub const THINKTHEN_COMPLETE_ORIGIN_PROXY_V1: u32 = 4;

pub const THINKTHEN_COMPLETE_ORIGIN_MEMORY_V1: u32 = 5;

pub(crate) fn read_thinkthen_complete_origin_v1(
    node: &Node,
    _store: &mut Storage,
) -> Result<thinkthen_complete_origin_v1, ErrorKind> {
    let kind = match node.text()? {
        "live" => THINKTHEN_COMPLETE_ORIGIN_LIVE_V1,
        "cache" => THINKTHEN_COMPLETE_ORIGIN_CACHE_V1,
        "replay" => THINKTHEN_COMPLETE_ORIGIN_REPLAY_V1,
        "proxy" => THINKTHEN_COMPLETE_ORIGIN_PROXY_V1,
        "memory" => THINKTHEN_COMPLETE_ORIGIN_MEMORY_V1,
        _ => return Err(ErrorKind::Defect),
    };
    Ok(thinkthen_complete_origin_v1 { kind })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_persistence_observation_v1 {
    pub advice: thinkthen_complete_utf8_presence_v1,
    pub observed_at: thinkthen_complete_utf8_v1,
    pub state: *const thinkthen_complete_usage_persistence_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_persistence_observation_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_persistence_observation_v1, ErrorKind> {
    Ok(thinkthen_complete_persistence_observation_v1 {
        advice: {
            let (presence, value) = read_presence(
                node.member("advice"),
                false,
                thinkthen_complete_utf8_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence_v1 { presence, value }
        },
        observed_at: {
            let value = node.required("observed_at")?;
            read_text(value, store)?
        },
        state: {
            let value = node.required("state")?;
            read_thinkthen_complete_usage_persistence_v1(value, store)
                .map(|value| store.hold(value))?
        },
        extensions: store.extensions(node, &["advice", "observed_at", "state"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_physical_source_v1 {
    pub file: thinkthen_complete_utf8_v1,
    pub first_line: thinkthen_complete_u64_presence_v1,
    pub last_line: thinkthen_complete_u64_presence_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_physical_source_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_physical_source_v1, ErrorKind> {
    Ok(thinkthen_complete_physical_source_v1 {
        file: {
            let value = node.required("file")?;
            read_text(value, store)?
        },
        first_line: {
            let (presence, value) =
                read_presence(node.member("first_line"), false, 0, read_number, store)?;
            thinkthen_complete_u64_presence_v1 { presence, value }
        },
        last_line: {
            let (presence, value) =
                read_presence(node.member("last_line"), false, 0, read_number, store)?;
            thinkthen_complete_u64_presence_v1 { presence, value }
        },
        extensions: store.extensions(node, &["file", "first_line", "last_line"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_position_v1 {
    pub file: thinkthen_complete_utf8_presence_v1,
    pub first: thinkthen_complete_u64_presence_v1,
    pub images: thinkthen_complete_string_list_presence_v1,
    pub last: thinkthen_complete_u64_presence_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_position_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_position_v1, ErrorKind> {
    Ok(thinkthen_complete_position_v1 {
        file: {
            let (presence, value) = read_presence(
                node.member("file"),
                true,
                thinkthen_complete_utf8_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence_v1 { presence, value }
        },
        first: {
            let (presence, value) =
                read_presence(node.member("first"), false, 0, read_number, store)?;
            thinkthen_complete_u64_presence_v1 { presence, value }
        },
        images: {
            let (presence, value) = read_presence(
                node.member("images"),
                false,
                thinkthen_complete_string_list_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_string_list_v1,
                store,
            )?;
            thinkthen_complete_string_list_presence_v1 { presence, value }
        },
        last: {
            let (presence, value) =
                read_presence(node.member("last"), false, 0, read_number, store)?;
            thinkthen_complete_u64_presence_v1 { presence, value }
        },
        extensions: store.extensions(node, &["file", "first", "images", "last"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_question_name_v1 {
    pub value: thinkthen_complete_utf8_v1,
}

pub(crate) fn read_thinkthen_complete_question_name_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_question_name_v1, ErrorKind> {
    let value = read_text(node, store)?;
    Ok(thinkthen_complete_question_name_v1 { value })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_question_source_v1 {
    pub answered_by: thinkthen_complete_utf8_v1,
    pub batch_size: thinkthen_complete_u64_presence_v1,
    pub origin: *const thinkthen_complete_origin_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_question_source_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_question_source_v1, ErrorKind> {
    Ok(thinkthen_complete_question_source_v1 {
        answered_by: {
            let value = node.required("answered_by")?;
            read_text(value, store)?
        },
        batch_size: {
            let (presence, value) =
                read_presence(node.member("batch_size"), false, 0, read_number, store)?;
            thinkthen_complete_u64_presence_v1 { presence, value }
        },
        origin: {
            let value = node.required("origin")?;
            read_thinkthen_complete_origin_v1(value, store).map(|value| store.hold(value))?
        },
        extensions: store.extensions(node, &["answered_by", "batch_size", "origin"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_rank_member_v1 {
    pub name: thinkthen_complete_utf8_v1,
    pub result: *const thinkthen_complete_rank_member_result_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_rank_member_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_rank_member_v1, ErrorKind> {
    Ok(thinkthen_complete_rank_member_v1 {
        name: {
            let value = node.required("name")?;
            read_text(value, store)?
        },
        result: {
            let value = node.required("result")?;
            read_thinkthen_complete_rank_member_result_v1(value, store)
                .map(|value| store.hold(value))?
        },
        extensions: store.extensions(node, &["name", "result"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_rank_member_result_v1 {
    pub answer: *const thinkthen_complete_answer_v1,
    pub answer_id: *const thinkthen_complete_answer_id_v1,
    pub images: thinkthen_complete_image_list_presence_v1,
    pub meta: *const thinkthen_complete_meta_v1,
    pub question: *const thinkthen_complete_question_document_v1,
    pub schema: *const thinkthen_complete_version_v1,
    pub source: thinkthen_complete_physical_source_presence_v1,
    pub threshold: thinkthen_complete_json_presence_v1,
    pub value: u64,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_rank_member_result_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_rank_member_result_v1, ErrorKind> {
    Ok(thinkthen_complete_rank_member_result_v1 {
        answer: {
            let value = node.required("answer")?;
            read_thinkthen_complete_answer_v1(value, store).map(|value| store.hold(value))?
        },
        answer_id: {
            let value = node.required("answer_id")?;
            read_thinkthen_complete_answer_id_v1(value, store).map(|value| store.hold(value))?
        },
        images: {
            let (presence, value) = read_presence(
                node.member("images"),
                false,
                thinkthen_complete_image_list_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_image_list_v1,
                store,
            )?;
            thinkthen_complete_image_list_presence_v1 { presence, value }
        },
        meta: {
            let value = node.required("meta")?;
            read_thinkthen_complete_meta_v1(value, store).map(|value| store.hold(value))?
        },
        question: {
            let value = node.required("question")?;
            read_thinkthen_complete_question_document_v1(value, store)
                .map(|value| store.hold(value))?
        },
        schema: {
            let value = node.required("schema")?;
            read_thinkthen_complete_version_v1(value, store).map(|value| store.hold(value))?
        },
        source: {
            let (presence, value) = read_presence(
                node.member("source"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_physical_source_v1(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_physical_source_presence_v1 { presence, value }
        },
        threshold: {
            let (presence, value) = read_presence(
                node.member("threshold"),
                true,
                std::ptr::null(),
                read_json,
                store,
            )?;
            thinkthen_complete_json_presence_v1 { presence, value }
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
pub struct thinkthen_complete_recognition_v1 {
    pub answer: *const thinkthen_complete_recognition_odds_v1,
    pub answer_id: *const thinkthen_complete_answer_id_v1,
    pub file: thinkthen_complete_utf8_presence_v1,
    pub first_line: thinkthen_complete_u64_presence_v1,
    pub index: thinkthen_complete_u64_presence_v1,
    pub input: thinkthen_complete_json_presence_v1,
    pub last_line: thinkthen_complete_u64_presence_v1,
    pub meta: *const thinkthen_complete_meta_v1,
    pub position: thinkthen_complete_position_presence_v1,
    pub question: *const thinkthen_complete_recognize_question_document_v1,
    pub schema: *const thinkthen_complete_version_v1,
    pub source: thinkthen_complete_physical_source_presence_v1,
    pub value: *const thinkthen_complete_recognize_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_recognition_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_recognition_v1, ErrorKind> {
    Ok(thinkthen_complete_recognition_v1 {
        answer: {
            let value = node.required("answer")?;
            read_thinkthen_complete_recognition_odds_v1(value, store)
                .map(|value| store.hold(value))?
        },
        answer_id: {
            let value = node.required("answer_id")?;
            read_thinkthen_complete_answer_id_v1(value, store).map(|value| store.hold(value))?
        },
        file: {
            let (presence, value) = read_presence(
                node.member("file"),
                false,
                thinkthen_complete_utf8_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence_v1 { presence, value }
        },
        first_line: {
            let (presence, value) =
                read_presence(node.member("first_line"), false, 0, read_number, store)?;
            thinkthen_complete_u64_presence_v1 { presence, value }
        },
        index: {
            let (presence, value) =
                read_presence(node.member("index"), false, 0, read_number, store)?;
            thinkthen_complete_u64_presence_v1 { presence, value }
        },
        input: {
            let (presence, value) = read_presence(
                node.member("input"),
                false,
                std::ptr::null(),
                read_json,
                store,
            )?;
            thinkthen_complete_json_presence_v1 { presence, value }
        },
        last_line: {
            let (presence, value) =
                read_presence(node.member("last_line"), false, 0, read_number, store)?;
            thinkthen_complete_u64_presence_v1 { presence, value }
        },
        meta: {
            let value = node.required("meta")?;
            read_thinkthen_complete_meta_v1(value, store).map(|value| store.hold(value))?
        },
        position: {
            let (presence, value) = read_presence(
                node.member("position"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_position_v1(value, store).map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_position_presence_v1 { presence, value }
        },
        question: {
            let value = node.required("question")?;
            read_thinkthen_complete_recognize_question_document_v1(value, store)
                .map(|value| store.hold(value))?
        },
        schema: {
            let value = node.required("schema")?;
            read_thinkthen_complete_version_v1(value, store).map(|value| store.hold(value))?
        },
        source: {
            let (presence, value) = read_presence(
                node.member("source"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_physical_source_v1(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_physical_source_presence_v1 { presence, value }
        },
        value: {
            let value = node.required("value")?;
            read_thinkthen_complete_recognize_v1(value, store).map(|value| store.hold(value))?
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
pub struct thinkthen_complete_recognition_edge_v1 {
    pub either: u32,
    pub probability: f64,
    pub relation: thinkthen_complete_utf8_v1,
    pub source: *const thinkthen_complete_entity_v1,
    pub target: *const thinkthen_complete_entity_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_recognition_edge_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_recognition_edge_v1, ErrorKind> {
    Ok(thinkthen_complete_recognition_edge_v1 {
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
            read_thinkthen_complete_entity_v1(value, store).map(|value| store.hold(value))?
        },
        target: {
            let value = node.required("target")?;
            read_thinkthen_complete_entity_v1(value, store).map(|value| store.hold(value))?
        },
        extensions: store.extensions(
            node,
            &["either", "probability", "relation", "source", "target"],
        )?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_recognition_mode_v1 {
    pub kind: u32,
}

pub const THINKTHEN_COMPLETE_RECOGNITION_MODE_WHOLE_V1: u32 = 1;

pub const THINKTHEN_COMPLETE_RECOGNITION_MODE_BOUNDARY_ONLY_V1: u32 = 2;

pub(crate) fn read_thinkthen_complete_recognition_mode_v1(
    node: &Node,
    _store: &mut Storage,
) -> Result<thinkthen_complete_recognition_mode_v1, ErrorKind> {
    let kind = match node.text()? {
        "whole" => THINKTHEN_COMPLETE_RECOGNITION_MODE_WHOLE_V1,
        "boundary_only" => THINKTHEN_COMPLETE_RECOGNITION_MODE_BOUNDARY_ONLY_V1,
        _ => return Err(ErrorKind::Defect),
    };
    Ok(thinkthen_complete_recognition_mode_v1 { kind })
}

pub const THINKTHEN_COMPLETE_RECOGNITION_ODDS_RECOGNIZE_ANSWER_V1: u32 = 1;

pub const THINKTHEN_COMPLETE_RECOGNITION_ODDS_BOUNDARY_ODDS_V1: u32 = 2;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_recognition_odds_data_v1 {
    pub recognize_answer: *const thinkthen_complete_recognition_odds_recognize_answer_v1,
    pub boundary_odds: *const thinkthen_complete_recognition_odds_boundary_odds_v1,
}
impl std::fmt::Debug for thinkthen_complete_recognition_odds_data_v1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("union payload")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_recognition_odds_v1 {
    pub kind: u32,
    pub data: thinkthen_complete_recognition_odds_data_v1,
}

pub(crate) fn read_thinkthen_complete_recognition_odds_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_recognition_odds_v1, ErrorKind> {
    if node.member("names").is_some()
        && node.member("pairs").is_some()
        && node.member("pieces").is_some()
        && node.member("proposals").is_some()
    {
        let value = read_thinkthen_complete_recognition_odds_recognize_answer_v1(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_recognition_odds_v1 {
            kind: THINKTHEN_COMPLETE_RECOGNITION_ODDS_RECOGNIZE_ANSWER_V1,
            data: thinkthen_complete_recognition_odds_data_v1 {
                recognize_answer: value,
            },
        });
    }
    if node.member("pieces").is_some()
        && node.member("proposals").is_some()
        && node.member("names").is_none()
        && node.member("pairs").is_none()
    {
        let value = read_thinkthen_complete_recognition_odds_boundary_odds_v1(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_recognition_odds_v1 {
            kind: THINKTHEN_COMPLETE_RECOGNITION_ODDS_BOUNDARY_ODDS_V1,
            data: thinkthen_complete_recognition_odds_data_v1 {
                boundary_odds: value,
            },
        });
    }
    Err(ErrorKind::Defect)
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_piece_odds_list_v1 {
    pub data: *const *const thinkthen_complete_piece_odds_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_piece_odds_list_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_piece_odds_list_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_piece_odds_v1(item, store).map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_piece_odds_list_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_boundary_proposal_list_v1 {
    pub data: *const *const thinkthen_complete_boundary_proposal_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_boundary_proposal_list_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_boundary_proposal_list_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_boundary_proposal_v1(item, store)
                .map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_boundary_proposal_list_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_recognition_odds_boundary_odds_v1 {
    pub pieces: thinkthen_complete_piece_odds_list_v1,
    pub proposals: thinkthen_complete_boundary_proposal_list_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_recognition_odds_boundary_odds_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_recognition_odds_boundary_odds_v1, ErrorKind> {
    Ok(thinkthen_complete_recognition_odds_boundary_odds_v1 {
        pieces: {
            let value = node.required("pieces")?;
            read_thinkthen_complete_piece_odds_list_v1(value, store)?
        },
        proposals: {
            let value = node.required("proposals")?;
            read_thinkthen_complete_boundary_proposal_list_v1(value, store)?
        },
        extensions: store.extensions(node, &["pieces", "proposals"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_name_odds_list_v1 {
    pub data: *const *const thinkthen_complete_name_odds_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_name_odds_list_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_name_odds_list_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_name_odds_v1(item, store).map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_name_odds_list_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_pair_odds_list_v1 {
    pub data: *const *const thinkthen_complete_pair_odds_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_pair_odds_list_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_pair_odds_list_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_pair_odds_v1(item, store).map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_pair_odds_list_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_recognition_proposal_list_v1 {
    pub data: *const *const thinkthen_complete_recognition_proposal_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_recognition_proposal_list_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_recognition_proposal_list_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_recognition_proposal_v1(item, store)
                .map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_recognition_proposal_list_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_recognition_odds_recognize_answer_v1 {
    pub names: thinkthen_complete_name_odds_list_v1,
    pub pairs: thinkthen_complete_pair_odds_list_v1,
    pub pieces: thinkthen_complete_piece_odds_list_v1,
    pub proposals: thinkthen_complete_recognition_proposal_list_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_recognition_odds_recognize_answer_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_recognition_odds_recognize_answer_v1, ErrorKind> {
    Ok(thinkthen_complete_recognition_odds_recognize_answer_v1 {
        names: {
            let value = node.required("names")?;
            read_thinkthen_complete_name_odds_list_v1(value, store)?
        },
        pairs: {
            let value = node.required("pairs")?;
            read_thinkthen_complete_pair_odds_list_v1(value, store)?
        },
        pieces: {
            let value = node.required("pieces")?;
            read_thinkthen_complete_piece_odds_list_v1(value, store)?
        },
        proposals: {
            let value = node.required("proposals")?;
            read_thinkthen_complete_recognition_proposal_list_v1(value, store)?
        },
        extensions: store.extensions(node, &["names", "pairs", "pieces", "proposals"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_place_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_place_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_f64_presence_v1 {
    pub presence: u32,
    pub value: f64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_recognition_proposal_v1 {
    pub end: u64,
    pub kept: u32,
    pub kind: thinkthen_complete_utf8_presence_v1,
    pub selected: thinkthen_complete_place_presence_v1,
    pub span_probability: f64,
    pub start: u64,
    pub strength: thinkthen_complete_f64_presence_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_recognition_proposal_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_recognition_proposal_v1, ErrorKind> {
    Ok(thinkthen_complete_recognition_proposal_v1 {
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
                thinkthen_complete_utf8_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence_v1 { presence, value }
        },
        selected: {
            let (presence, value) = read_presence(
                node.member("selected"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_place_v1(value, store).map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_place_presence_v1 { presence, value }
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
            thinkthen_complete_f64_presence_v1 { presence, value }
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
pub struct thinkthen_complete_recognition_stage_context_v1 {
    pub boundary: thinkthen_complete_utf8_presence_v1,
    pub kind_edge: thinkthen_complete_utf8_presence_v1,
    pub relation: thinkthen_complete_utf8_presence_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_recognition_stage_context_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_recognition_stage_context_v1, ErrorKind> {
    Ok(thinkthen_complete_recognition_stage_context_v1 {
        boundary: {
            let (presence, value) = read_presence(
                node.member("boundary"),
                false,
                thinkthen_complete_utf8_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence_v1 { presence, value }
        },
        kind_edge: {
            let (presence, value) = read_presence(
                node.member("kind_edge"),
                false,
                thinkthen_complete_utf8_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence_v1 { presence, value }
        },
        relation: {
            let (presence, value) = read_presence(
                node.member("relation"),
                false,
                thinkthen_complete_utf8_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence_v1 { presence, value }
        },
        extensions: store.extensions(node, &["boundary", "kind_edge", "relation"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_input_source_list_v1 {
    pub data: *const *const thinkthen_complete_session_input_source_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_session_input_source_list_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_input_source_list_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_session_input_source_v1(item, store)
                .map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_session_input_source_list_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_input_source_list_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_session_input_source_list_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_related_entity_edge_list_v1 {
    pub data: *const *const thinkthen_complete_related_entity_edge_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_related_entity_edge_list_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_related_entity_edge_list_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_related_entity_edge_v1(item, store)
                .map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_related_entity_edge_list_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_relation_v1 {
    pub answer: *const thinkthen_complete_answers_v1,
    pub answer_id: *const thinkthen_complete_answer_id_v1,
    pub file: thinkthen_complete_utf8_presence_v1,
    pub first_line: thinkthen_complete_u64_presence_v1,
    pub index: thinkthen_complete_u64_presence_v1,
    pub input: thinkthen_complete_json_presence_v1,
    pub input_sources: thinkthen_complete_session_input_source_list_presence_v1,
    pub last_line: thinkthen_complete_u64_presence_v1,
    pub meta: *const thinkthen_complete_meta_v1,
    pub position: thinkthen_complete_position_presence_v1,
    pub question: *const thinkthen_complete_relate_question_document_v1,
    pub schema: *const thinkthen_complete_version_v1,
    pub value: thinkthen_complete_related_entity_edge_list_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_relation_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_relation_v1, ErrorKind> {
    Ok(thinkthen_complete_relation_v1 {
        answer: {
            let value = node.required("answer")?;
            read_thinkthen_complete_answers_v1(value, store).map(|value| store.hold(value))?
        },
        answer_id: {
            let value = node.required("answer_id")?;
            read_thinkthen_complete_answer_id_v1(value, store).map(|value| store.hold(value))?
        },
        file: {
            let (presence, value) = read_presence(
                node.member("file"),
                false,
                thinkthen_complete_utf8_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence_v1 { presence, value }
        },
        first_line: {
            let (presence, value) =
                read_presence(node.member("first_line"), false, 0, read_number, store)?;
            thinkthen_complete_u64_presence_v1 { presence, value }
        },
        index: {
            let (presence, value) =
                read_presence(node.member("index"), false, 0, read_number, store)?;
            thinkthen_complete_u64_presence_v1 { presence, value }
        },
        input: {
            let (presence, value) = read_presence(
                node.member("input"),
                false,
                std::ptr::null(),
                read_json,
                store,
            )?;
            thinkthen_complete_json_presence_v1 { presence, value }
        },
        input_sources: {
            let (presence, value) = read_presence(
                node.member("input_sources"),
                false,
                thinkthen_complete_session_input_source_list_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_session_input_source_list_v1,
                store,
            )?;
            thinkthen_complete_session_input_source_list_presence_v1 { presence, value }
        },
        last_line: {
            let (presence, value) =
                read_presence(node.member("last_line"), false, 0, read_number, store)?;
            thinkthen_complete_u64_presence_v1 { presence, value }
        },
        meta: {
            let value = node.required("meta")?;
            read_thinkthen_complete_meta_v1(value, store).map(|value| store.hold(value))?
        },
        position: {
            let (presence, value) = read_presence(
                node.member("position"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_position_v1(value, store).map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_position_presence_v1 { presence, value }
        },
        question: {
            let value = node.required("question")?;
            read_thinkthen_complete_relate_question_document_v1(value, store)
                .map(|value| store.hold(value))?
        },
        schema: {
            let value = node.required("schema")?;
            read_thinkthen_complete_version_v1(value, store).map(|value| store.hold(value))?
        },
        value: {
            let value = node.required("value")?;
            read_thinkthen_complete_related_entity_edge_list_v1(value, store)?
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
pub struct thinkthen_complete_relation_direction_v1 {
    pub kind: u32,
}

pub const THINKTHEN_COMPLETE_RELATION_DIRECTION_SOURCE_TO_TARGET_V1: u32 = 1;

pub const THINKTHEN_COMPLETE_RELATION_DIRECTION_EITHER_V1: u32 = 2;

pub(crate) fn read_thinkthen_complete_relation_direction_v1(
    node: &Node,
    _store: &mut Storage,
) -> Result<thinkthen_complete_relation_direction_v1, ErrorKind> {
    let kind = match node.text()? {
        "source_to_target" => THINKTHEN_COMPLETE_RELATION_DIRECTION_SOURCE_TO_TARGET_V1,
        "either" => THINKTHEN_COMPLETE_RELATION_DIRECTION_EITHER_V1,
        _ => return Err(ErrorKind::Defect),
    };
    Ok(thinkthen_complete_relation_direction_v1 { kind })
}

pub const THINKTHEN_COMPLETE_RELATION_ENDPOINT_RELATED_ENTITY_V1: u32 = 1;

pub const THINKTHEN_COMPLETE_RELATION_ENDPOINT_SOURCE_V1: u32 = 2;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_relation_endpoint_data_v1 {
    pub related_entity: *const thinkthen_complete_relation_endpoint_related_entity_v1,
    pub source: *const thinkthen_complete_relation_endpoint_source_v1,
}
impl std::fmt::Debug for thinkthen_complete_relation_endpoint_data_v1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("union payload")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_relation_endpoint_v1 {
    pub kind: u32,
    pub data: thinkthen_complete_relation_endpoint_data_v1,
}

pub(crate) fn read_thinkthen_complete_relation_endpoint_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_relation_endpoint_v1, ErrorKind> {
    if node.member("kind").is_some()
        && node.member("name").is_some()
        && node.member("file").is_none()
        && node.member("ordinal").is_none()
        && node.member("record").is_none()
    {
        let value = read_thinkthen_complete_relation_endpoint_related_entity_v1(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_relation_endpoint_v1 {
            kind: THINKTHEN_COMPLETE_RELATION_ENDPOINT_RELATED_ENTITY_V1,
            data: thinkthen_complete_relation_endpoint_data_v1 {
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
        let value = read_thinkthen_complete_relation_endpoint_source_v1(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_relation_endpoint_v1 {
            kind: THINKTHEN_COMPLETE_RELATION_ENDPOINT_SOURCE_V1,
            data: thinkthen_complete_relation_endpoint_data_v1 { source: value },
        });
    }
    Err(ErrorKind::Defect)
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_relation_endpoint_related_entity_v1 {
    pub kind: thinkthen_complete_utf8_v1,
    pub name: thinkthen_complete_utf8_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_relation_endpoint_related_entity_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_relation_endpoint_related_entity_v1, ErrorKind> {
    Ok(thinkthen_complete_relation_endpoint_related_entity_v1 {
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
pub struct thinkthen_complete_relation_endpoint_source_v1 {
    pub file: thinkthen_complete_utf8_presence_v1,
    pub first_line: thinkthen_complete_u64_presence_v1,
    pub kind: thinkthen_complete_utf8_v1,
    pub last_line: thinkthen_complete_u64_presence_v1,
    pub name: thinkthen_complete_utf8_v1,
    pub ordinal: u64,
    pub record: thinkthen_complete_json_presence_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_relation_endpoint_source_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_relation_endpoint_source_v1, ErrorKind> {
    Ok(thinkthen_complete_relation_endpoint_source_v1 {
        file: {
            let (presence, value) = read_presence(
                node.member("file"),
                true,
                thinkthen_complete_utf8_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence_v1 { presence, value }
        },
        first_line: {
            let (presence, value) =
                read_presence(node.member("first_line"), false, 0, read_number, store)?;
            thinkthen_complete_u64_presence_v1 { presence, value }
        },
        kind: {
            let value = node.required("kind")?;
            read_text(value, store)?
        },
        last_line: {
            let (presence, value) =
                read_presence(node.member("last_line"), false, 0, read_number, store)?;
            thinkthen_complete_u64_presence_v1 { presence, value }
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
            thinkthen_complete_json_presence_v1 { presence, value }
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

pub const THINKTHEN_COMPLETE_RELATION_MEMBER_ANSWER_ID_V1: u32 = 1;

pub const THINKTHEN_COMPLETE_RELATION_MEMBER_FAILURE_ID_V1: u32 = 2;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_relation_member_data_v1 {
    pub answer_id: *const thinkthen_complete_relation_member_answer_id_v1,
    pub failure_id: *const thinkthen_complete_relation_member_failure_id_v1,
}
impl std::fmt::Debug for thinkthen_complete_relation_member_data_v1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("union payload")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_relation_member_v1 {
    pub kind: u32,
    pub data: thinkthen_complete_relation_member_data_v1,
}

pub(crate) fn read_thinkthen_complete_relation_member_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_relation_member_v1, ErrorKind> {
    if node.member("answer_id").is_some() {
        let value = read_thinkthen_complete_relation_member_answer_id_v1(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_relation_member_v1 {
            kind: THINKTHEN_COMPLETE_RELATION_MEMBER_ANSWER_ID_V1,
            data: thinkthen_complete_relation_member_data_v1 { answer_id: value },
        });
    }
    if node.member("failure_id").is_some() {
        let value = read_thinkthen_complete_relation_member_failure_id_v1(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_relation_member_v1 {
            kind: THINKTHEN_COMPLETE_RELATION_MEMBER_FAILURE_ID_V1,
            data: thinkthen_complete_relation_member_data_v1 { failure_id: value },
        });
    }
    Err(ErrorKind::Defect)
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_related_entity_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_related_entity_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_relation_member_answer_id_v1 {
    pub direction: *const thinkthen_complete_relation_direction_v1,
    pub method: *const thinkthen_complete_relation_method_v1,
    pub observations: thinkthen_complete_observation_list_v1,
    pub question: *const thinkthen_complete_question_document_v1,
    pub question_sources: thinkthen_complete_question_source_list_v1,
    pub reads: thinkthen_complete_utf8_v1,
    pub relation: thinkthen_complete_utf8_v1,
    pub request: thinkthen_complete_utf8_v1,
    pub source: *const thinkthen_complete_related_entity_v1,
    pub target: thinkthen_complete_related_entity_presence_v1,
    pub threshold: *const thinkthen_complete_threshold_v1,
    pub usage: thinkthen_complete_usage_presence_v1,
    pub accepted: u32,
    pub answer: *const thinkthen_complete_answer_v1,
    pub answer_id: *const thinkthen_complete_answer_id_v1,
    pub probability: f64,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_relation_member_answer_id_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_relation_member_answer_id_v1, ErrorKind> {
    Ok(thinkthen_complete_relation_member_answer_id_v1 {
        direction: {
            let value = node.required("direction")?;
            read_thinkthen_complete_relation_direction_v1(value, store)
                .map(|value| store.hold(value))?
        },
        method: {
            let value = node.required("method")?;
            read_thinkthen_complete_relation_method_v1(value, store)
                .map(|value| store.hold(value))?
        },
        observations: {
            let value = node.required("observations")?;
            read_thinkthen_complete_observation_list_v1(value, store)?
        },
        question: {
            let value = node.required("question")?;
            read_thinkthen_complete_question_document_v1(value, store)
                .map(|value| store.hold(value))?
        },
        question_sources: {
            let value = node.required("question_sources")?;
            read_thinkthen_complete_question_source_list_v1(value, store)?
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
            read_thinkthen_complete_related_entity_v1(value, store)
                .map(|value| store.hold(value))?
        },
        target: {
            let (presence, value) = read_presence(
                node.member("target"),
                true,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_related_entity_v1(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_related_entity_presence_v1 { presence, value }
        },
        threshold: {
            let value = node.required("threshold")?;
            read_thinkthen_complete_threshold_v1(value, store).map(|value| store.hold(value))?
        },
        usage: {
            let (presence, value) = read_presence(
                node.member("usage"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_usage_v1(value, store).map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_usage_presence_v1 { presence, value }
        },
        accepted: {
            let value = node.required("accepted")?;
            read_boolean(value, store)?
        },
        answer: {
            let value = node.required("answer")?;
            read_thinkthen_complete_answer_v1(value, store).map(|value| store.hold(value))?
        },
        answer_id: {
            let value = node.required("answer_id")?;
            read_thinkthen_complete_answer_id_v1(value, store).map(|value| store.hold(value))?
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
pub struct thinkthen_complete_relation_member_failure_id_v1 {
    pub direction: *const thinkthen_complete_relation_direction_v1,
    pub method: *const thinkthen_complete_relation_method_v1,
    pub observations: thinkthen_complete_observation_list_v1,
    pub question: *const thinkthen_complete_question_document_v1,
    pub question_sources: thinkthen_complete_question_source_list_v1,
    pub reads: thinkthen_complete_utf8_v1,
    pub relation: thinkthen_complete_utf8_v1,
    pub request: thinkthen_complete_utf8_v1,
    pub source: *const thinkthen_complete_related_entity_v1,
    pub target: thinkthen_complete_related_entity_presence_v1,
    pub threshold: *const thinkthen_complete_threshold_v1,
    pub usage: thinkthen_complete_usage_presence_v1,
    pub failure: *const thinkthen_complete_failure_v1,
    pub failure_id: *const thinkthen_complete_failure_id_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_relation_member_failure_id_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_relation_member_failure_id_v1, ErrorKind> {
    Ok(thinkthen_complete_relation_member_failure_id_v1 {
        direction: {
            let value = node.required("direction")?;
            read_thinkthen_complete_relation_direction_v1(value, store)
                .map(|value| store.hold(value))?
        },
        method: {
            let value = node.required("method")?;
            read_thinkthen_complete_relation_method_v1(value, store)
                .map(|value| store.hold(value))?
        },
        observations: {
            let value = node.required("observations")?;
            read_thinkthen_complete_observation_list_v1(value, store)?
        },
        question: {
            let value = node.required("question")?;
            read_thinkthen_complete_question_document_v1(value, store)
                .map(|value| store.hold(value))?
        },
        question_sources: {
            let value = node.required("question_sources")?;
            read_thinkthen_complete_question_source_list_v1(value, store)?
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
            read_thinkthen_complete_related_entity_v1(value, store)
                .map(|value| store.hold(value))?
        },
        target: {
            let (presence, value) = read_presence(
                node.member("target"),
                true,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_related_entity_v1(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_related_entity_presence_v1 { presence, value }
        },
        threshold: {
            let value = node.required("threshold")?;
            read_thinkthen_complete_threshold_v1(value, store).map(|value| store.hold(value))?
        },
        usage: {
            let (presence, value) = read_presence(
                node.member("usage"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_usage_v1(value, store).map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_usage_presence_v1 { presence, value }
        },
        failure: {
            let value = node.required("failure")?;
            read_thinkthen_complete_failure_v1(value, store).map(|value| store.hold(value))?
        },
        failure_id: {
            let value = node.required("failure_id")?;
            read_thinkthen_complete_failure_id_v1(value, store).map(|value| store.hold(value))?
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
pub struct thinkthen_complete_relation_method_v1 {
    pub kind: u32,
}

pub const THINKTHEN_COMPLETE_RELATION_METHOD_YES_NO_V1: u32 = 1;

pub const THINKTHEN_COMPLETE_RELATION_METHOD_CHOICE_V1: u32 = 2;

pub(crate) fn read_thinkthen_complete_relation_method_v1(
    node: &Node,
    _store: &mut Storage,
) -> Result<thinkthen_complete_relation_method_v1, ErrorKind> {
    let kind = match node.text()? {
        "yes_no" => THINKTHEN_COMPLETE_RELATION_METHOD_YES_NO_V1,
        "choice" => THINKTHEN_COMPLETE_RELATION_METHOD_CHOICE_V1,
        _ => return Err(ErrorKind::Defect),
    };
    Ok(thinkthen_complete_relation_method_v1 { kind })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_request_function_v1 {
    pub kind: u32,
}

pub const THINKTHEN_COMPLETE_REQUEST_FUNCTION_DECIDE_V1: u32 = 1;

pub const THINKTHEN_COMPLETE_REQUEST_FUNCTION_CHOOSE_V1: u32 = 2;

pub const THINKTHEN_COMPLETE_REQUEST_FUNCTION_TAG_V1: u32 = 3;

pub const THINKTHEN_COMPLETE_REQUEST_FUNCTION_SCORE_V1: u32 = 4;

pub const THINKTHEN_COMPLETE_REQUEST_FUNCTION_FILTER_V1: u32 = 5;

pub const THINKTHEN_COMPLETE_REQUEST_FUNCTION_RANK_V1: u32 = 6;

pub const THINKTHEN_COMPLETE_REQUEST_FUNCTION_FIND_V1: u32 = 7;

pub const THINKTHEN_COMPLETE_REQUEST_FUNCTION_ANNOTATE_V1: u32 = 8;

pub const THINKTHEN_COMPLETE_REQUEST_FUNCTION_RECOGNIZE_V1: u32 = 9;

pub const THINKTHEN_COMPLETE_REQUEST_FUNCTION_RELATE_V1: u32 = 10;

pub(crate) fn read_thinkthen_complete_request_function_v1(
    node: &Node,
    _store: &mut Storage,
) -> Result<thinkthen_complete_request_function_v1, ErrorKind> {
    let kind = match node.text()? {
        "decide" => THINKTHEN_COMPLETE_REQUEST_FUNCTION_DECIDE_V1,
        "choose" => THINKTHEN_COMPLETE_REQUEST_FUNCTION_CHOOSE_V1,
        "tag" => THINKTHEN_COMPLETE_REQUEST_FUNCTION_TAG_V1,
        "score" => THINKTHEN_COMPLETE_REQUEST_FUNCTION_SCORE_V1,
        "filter" => THINKTHEN_COMPLETE_REQUEST_FUNCTION_FILTER_V1,
        "rank" => THINKTHEN_COMPLETE_REQUEST_FUNCTION_RANK_V1,
        "find" => THINKTHEN_COMPLETE_REQUEST_FUNCTION_FIND_V1,
        "annotate" => THINKTHEN_COMPLETE_REQUEST_FUNCTION_ANNOTATE_V1,
        "recognize" => THINKTHEN_COMPLETE_REQUEST_FUNCTION_RECOGNIZE_V1,
        "relate" => THINKTHEN_COMPLETE_REQUEST_FUNCTION_RELATE_V1,
        _ => return Err(ErrorKind::Defect),
    };
    Ok(thinkthen_complete_request_function_v1 { kind })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_sdk_request_id_v1 {
    pub value: thinkthen_complete_utf8_v1,
}

pub(crate) fn read_thinkthen_complete_sdk_request_id_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_sdk_request_id_v1, ErrorKind> {
    let value = read_text(node, store)?;
    Ok(thinkthen_complete_sdk_request_id_v1 { value })
}

pub const THINKTHEN_COMPLETE_SEND_DENIAL_BEFORE_FIRST_SEND_V1: u32 = 1;

pub const THINKTHEN_COMPLETE_SEND_DENIAL_BEFORE_ADDITIONAL_SEND_V1: u32 = 2;

pub const THINKTHEN_COMPLETE_SEND_DENIAL_BEFORE_RETRY_V1: u32 = 3;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_send_denial_data_v1 {
    pub before_first_send: *const thinkthen_complete_send_denial_before_first_send_v1,
    pub before_additional_send: *const thinkthen_complete_send_denial_before_additional_send_v1,
    pub before_retry: *const thinkthen_complete_send_denial_before_retry_v1,
}
impl std::fmt::Debug for thinkthen_complete_send_denial_data_v1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("union payload")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_send_denial_v1 {
    pub kind: u32,
    pub data: thinkthen_complete_send_denial_data_v1,
}

pub(crate) fn read_thinkthen_complete_send_denial_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_send_denial_v1, ErrorKind> {
    if node.is_member("kind", "before_first_send") {
        let value = read_thinkthen_complete_send_denial_before_first_send_v1(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_send_denial_v1 {
            kind: THINKTHEN_COMPLETE_SEND_DENIAL_BEFORE_FIRST_SEND_V1,
            data: thinkthen_complete_send_denial_data_v1 {
                before_first_send: value,
            },
        });
    }
    if node.is_member("kind", "before_additional_send") {
        let value = read_thinkthen_complete_send_denial_before_additional_send_v1(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_send_denial_v1 {
            kind: THINKTHEN_COMPLETE_SEND_DENIAL_BEFORE_ADDITIONAL_SEND_V1,
            data: thinkthen_complete_send_denial_data_v1 {
                before_additional_send: value,
            },
        });
    }
    if node.is_member("kind", "before_retry") {
        let value = read_thinkthen_complete_send_denial_before_retry_v1(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_send_denial_v1 {
            kind: THINKTHEN_COMPLETE_SEND_DENIAL_BEFORE_RETRY_V1,
            data: thinkthen_complete_send_denial_data_v1 {
                before_retry: value,
            },
        });
    }
    Err(ErrorKind::Defect)
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_send_denial_before_additional_send_v1 {
    pub kind: thinkthen_complete_utf8_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_send_denial_before_additional_send_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_send_denial_before_additional_send_v1, ErrorKind> {
    Ok(thinkthen_complete_send_denial_before_additional_send_v1 {
        kind: {
            let value = node.required("kind")?;
            read_text(value, store)?
        },
        extensions: store.extensions(node, &["kind"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_send_denial_before_first_send_v1 {
    pub kind: thinkthen_complete_utf8_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_send_denial_before_first_send_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_send_denial_before_first_send_v1, ErrorKind> {
    Ok(thinkthen_complete_send_denial_before_first_send_v1 {
        kind: {
            let value = node.required("kind")?;
            read_text(value, store)?
        },
        extensions: store.extensions(node, &["kind"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_send_denial_before_retry_v1 {
    pub kind: thinkthen_complete_utf8_v1,
    pub last_status: u64,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_send_denial_before_retry_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_send_denial_before_retry_v1, ErrorKind> {
    Ok(thinkthen_complete_send_denial_before_retry_v1 {
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

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_stop_cause_v1 {
    pub kind: u32,
}

pub const THINKTHEN_COMPLETE_STOP_CAUSE_USAGE_V1: u32 = 1;

pub const THINKTHEN_COMPLETE_STOP_CAUSE_LOCAL_V1: u32 = 2;

pub const THINKTHEN_COMPLETE_STOP_CAUSE_NO_KEY_V1: u32 = 3;

pub const THINKTHEN_COMPLETE_STOP_CAUSE_TRANSPORT_V1: u32 = 4;

pub const THINKTHEN_COMPLETE_STOP_CAUSE_STATUS_V1: u32 = 5;

pub const THINKTHEN_COMPLETE_STOP_CAUSE_TOO_LARGE_V1: u32 = 6;

pub const THINKTHEN_COMPLETE_STOP_CAUSE_REPLY_V1: u32 = 7;

pub const THINKTHEN_COMPLETE_STOP_CAUSE_BACKEND_V1: u32 = 8;

pub const THINKTHEN_COMPLETE_STOP_CAUSE_CANCELLED_V1: u32 = 9;

pub const THINKTHEN_COMPLETE_STOP_CAUSE_DEADLINE_V1: u32 = 10;

pub const THINKTHEN_COMPLETE_STOP_CAUSE_DEFECT_V1: u32 = 11;

pub(crate) fn read_thinkthen_complete_stop_cause_v1(
    node: &Node,
    _store: &mut Storage,
) -> Result<thinkthen_complete_stop_cause_v1, ErrorKind> {
    let kind = match node.text()? {
        "usage" => THINKTHEN_COMPLETE_STOP_CAUSE_USAGE_V1,
        "local" => THINKTHEN_COMPLETE_STOP_CAUSE_LOCAL_V1,
        "no_key" => THINKTHEN_COMPLETE_STOP_CAUSE_NO_KEY_V1,
        "transport" => THINKTHEN_COMPLETE_STOP_CAUSE_TRANSPORT_V1,
        "status" => THINKTHEN_COMPLETE_STOP_CAUSE_STATUS_V1,
        "too_large" => THINKTHEN_COMPLETE_STOP_CAUSE_TOO_LARGE_V1,
        "reply" => THINKTHEN_COMPLETE_STOP_CAUSE_REPLY_V1,
        "backend" => THINKTHEN_COMPLETE_STOP_CAUSE_BACKEND_V1,
        "cancelled" => THINKTHEN_COMPLETE_STOP_CAUSE_CANCELLED_V1,
        "deadline" => THINKTHEN_COMPLETE_STOP_CAUSE_DEADLINE_V1,
        "defect" => THINKTHEN_COMPLETE_STOP_CAUSE_DEFECT_V1,
        _ => return Err(ErrorKind::Defect),
    };
    Ok(thinkthen_complete_stop_cause_v1 { kind })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_stopped_v1 {
    pub at: thinkthen_complete_u64_presence_v1,
    pub cause: *const thinkthen_complete_stop_cause_v1,
    pub retryable: u32,
    pub status: thinkthen_complete_u64_presence_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_stopped_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_stopped_v1, ErrorKind> {
    Ok(thinkthen_complete_stopped_v1 {
        at: {
            let (presence, value) = read_presence(node.member("at"), false, 0, read_number, store)?;
            thinkthen_complete_u64_presence_v1 { presence, value }
        },
        cause: {
            let value = node.required("cause")?;
            read_thinkthen_complete_stop_cause_v1(value, store).map(|value| store.hold(value))?
        },
        retryable: {
            let value = node.required("retryable")?;
            read_boolean(value, store)?
        },
        status: {
            let (presence, value) =
                read_presence(node.member("status"), false, 0, read_number, store)?;
            thinkthen_complete_u64_presence_v1 { presence, value }
        },
        extensions: store.extensions(node, &["at", "cause", "retryable", "status"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_string_root_v1 {
    pub r#type: *const thinkthen_complete_string_type_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_string_root_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_string_root_v1, ErrorKind> {
    Ok(thinkthen_complete_string_root_v1 {
        r#type: {
            let value = node.required("type")?;
            read_thinkthen_complete_string_type_v1(value, store).map(|value| store.hold(value))?
        },
        extensions: store.extensions(node, &["type"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_string_type_v1 {
    pub kind: u32,
}

pub const THINKTHEN_COMPLETE_STRING_TYPE_STRING_V1: u32 = 1;

pub(crate) fn read_thinkthen_complete_string_type_v1(
    node: &Node,
    _store: &mut Storage,
) -> Result<thinkthen_complete_string_type_v1, ErrorKind> {
    let kind = match node.text()? {
        "string" => THINKTHEN_COMPLETE_STRING_TYPE_STRING_V1,
        _ => return Err(ErrorKind::Defect),
    };
    Ok(thinkthen_complete_string_type_v1 { kind })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_usage_v1 {
    pub input_tokens: thinkthen_complete_u64_presence_v1,
    pub output_tokens: thinkthen_complete_u64_presence_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_usage_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_usage_v1, ErrorKind> {
    Ok(thinkthen_complete_usage_v1 {
        input_tokens: {
            let (presence, value) =
                read_presence(node.member("input_tokens"), false, 0, read_number, store)?;
            thinkthen_complete_u64_presence_v1 { presence, value }
        },
        output_tokens: {
            let (presence, value) =
                read_presence(node.member("output_tokens"), false, 0, read_number, store)?;
            thinkthen_complete_u64_presence_v1 { presence, value }
        },
        extensions: store.extensions(node, &["input_tokens", "output_tokens"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_usage_persistence_v1 {
    pub kind: u32,
}

pub const THINKTHEN_COMPLETE_USAGE_PERSISTENCE_DISABLED_V1: u32 = 1;

pub const THINKTHEN_COMPLETE_USAGE_PERSISTENCE_PENDING_V1: u32 = 2;

pub const THINKTHEN_COMPLETE_USAGE_PERSISTENCE_WRITTEN_V1: u32 = 3;

pub const THINKTHEN_COMPLETE_USAGE_PERSISTENCE_FAILED_V1: u32 = 4;

pub(crate) fn read_thinkthen_complete_usage_persistence_v1(
    node: &Node,
    _store: &mut Storage,
) -> Result<thinkthen_complete_usage_persistence_v1, ErrorKind> {
    let kind = match node.text()? {
        "disabled" => THINKTHEN_COMPLETE_USAGE_PERSISTENCE_DISABLED_V1,
        "pending" => THINKTHEN_COMPLETE_USAGE_PERSISTENCE_PENDING_V1,
        "written" => THINKTHEN_COMPLETE_USAGE_PERSISTENCE_WRITTEN_V1,
        "failed" => THINKTHEN_COMPLETE_USAGE_PERSISTENCE_FAILED_V1,
        _ => return Err(ErrorKind::Defect),
    };
    Ok(thinkthen_complete_usage_persistence_v1 { kind })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_verb_v1 {
    pub kind: u32,
}

pub const THINKTHEN_COMPLETE_VERB_RECOGNIZE_V1: u32 = 1;

pub(crate) fn read_thinkthen_complete_verb_v1(
    node: &Node,
    _store: &mut Storage,
) -> Result<thinkthen_complete_verb_v1, ErrorKind> {
    let kind = match node.text()? {
        "recognize" => THINKTHEN_COMPLETE_VERB_RECOGNIZE_V1,
        _ => return Err(ErrorKind::Defect),
    };
    Ok(thinkthen_complete_verb_v1 { kind })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_version_v1 {
    pub kind: u32,
}

pub const THINKTHEN_COMPLETE_VERSION_THINKTHEN_RESULT_2_V1: u32 = 1;

pub(crate) fn read_thinkthen_complete_version_v1(
    node: &Node,
    _store: &mut Storage,
) -> Result<thinkthen_complete_version_v1, ErrorKind> {
    let kind = match node.text()? {
        "thinkthen.result/2" => THINKTHEN_COMPLETE_VERSION_THINKTHEN_RESULT_2_V1,
        _ => return Err(ErrorKind::Defect),
    };
    Ok(thinkthen_complete_version_v1 { kind })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_wording_version_v1 {
    pub value: u64,
}

pub(crate) fn read_thinkthen_complete_wording_version_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_wording_version_v1, ErrorKind> {
    let value = read_number(node, store)?;
    Ok(thinkthen_complete_wording_version_v1 { value })
}

pub const THINKTHEN_COMPLETE_ANNOTATED_FIELD_BOOLEAN_V1: u32 = 1;

pub const THINKTHEN_COMPLETE_ANNOTATED_FIELD_NULL_V1: u32 = 2;

pub const THINKTHEN_COMPLETE_ANNOTATED_FIELD_STRING_V1: u32 = 3;

pub const THINKTHEN_COMPLETE_ANNOTATED_FIELD_ARRAY_V1: u32 = 4;

pub const THINKTHEN_COMPLETE_ANNOTATED_FIELD_NUMBER_V1: u32 = 5;

pub const THINKTHEN_COMPLETE_ANNOTATED_FIELD_OBJECT_V1: u32 = 6;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_annotated_field_data_v1 {
    pub boolean: u32,
    pub null: u32,
    pub string: thinkthen_complete_utf8_v1,
    pub array: thinkthen_complete_string_list_v1,
    pub number: f64,
    pub object: *const thinkthen_complete_failed_v1,
}
impl std::fmt::Debug for thinkthen_complete_annotated_field_data_v1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("union payload")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_annotated_field_v1 {
    pub kind: u32,
    pub data: thinkthen_complete_annotated_field_data_v1,
}

pub(crate) fn read_thinkthen_complete_annotated_field_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_annotated_field_v1, ErrorKind> {
    if node.kind() == "boolean" {
        let value = read_boolean(node, store)?;
        return Ok(thinkthen_complete_annotated_field_v1 {
            kind: THINKTHEN_COMPLETE_ANNOTATED_FIELD_BOOLEAN_V1,
            data: thinkthen_complete_annotated_field_data_v1 { boolean: value },
        });
    }
    if node.kind() == "null" {
        let value = 0;
        return Ok(thinkthen_complete_annotated_field_v1 {
            kind: THINKTHEN_COMPLETE_ANNOTATED_FIELD_NULL_V1,
            data: thinkthen_complete_annotated_field_data_v1 { null: value },
        });
    }
    if node.kind() == "string" {
        let value = read_text(node, store)?;
        return Ok(thinkthen_complete_annotated_field_v1 {
            kind: THINKTHEN_COMPLETE_ANNOTATED_FIELD_STRING_V1,
            data: thinkthen_complete_annotated_field_data_v1 { string: value },
        });
    }
    if node.kind() == "array" {
        let value = read_thinkthen_complete_string_list_v1(node, store)?;
        return Ok(thinkthen_complete_annotated_field_v1 {
            kind: THINKTHEN_COMPLETE_ANNOTATED_FIELD_ARRAY_V1,
            data: thinkthen_complete_annotated_field_data_v1 { array: value },
        });
    }
    if node.kind() == "number" {
        let value = read_number(node, store)?;
        return Ok(thinkthen_complete_annotated_field_v1 {
            kind: THINKTHEN_COMPLETE_ANNOTATED_FIELD_NUMBER_V1,
            data: thinkthen_complete_annotated_field_data_v1 { number: value },
        });
    }
    if node.kind() == "object" {
        let value =
            read_thinkthen_complete_failed_v1(node, store).map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_annotated_field_v1 {
            kind: THINKTHEN_COMPLETE_ANNOTATED_FIELD_OBJECT_V1,
            data: thinkthen_complete_annotated_field_data_v1 { object: value },
        });
    }
    Err(ErrorKind::Defect)
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_annotated_field_map_entry_v1 {
    pub name: thinkthen_complete_utf8_v1,
    pub value: *const thinkthen_complete_annotated_field_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_annotated_field_map_v1 {
    pub data: *const thinkthen_complete_annotated_field_map_entry_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_annotated_field_map_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_annotated_field_map_v1, ErrorKind> {
    let mut values = Vec::new();
    for (key, item) in node.object()? {
        values.push(thinkthen_complete_annotated_field_map_entry_v1 {
            name: store.text(key),
            value: read_thinkthen_complete_annotated_field_v1(item, store)
                .map(|value| store.hold(value))?,
        });
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_annotated_field_map_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_annotated_row_v1 {
    pub value: thinkthen_complete_annotated_field_map_v1,
}

pub(crate) fn read_thinkthen_complete_annotated_row_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_annotated_row_v1, ErrorKind> {
    let value = read_thinkthen_complete_annotated_field_map_v1(node, store)?;
    Ok(thinkthen_complete_annotated_row_v1 { value })
}

pub const THINKTHEN_COMPLETE_ANSWER_YES_NO_V1: u32 = 1;

pub const THINKTHEN_COMPLETE_ANSWER_CHOICE_V1: u32 = 2;

pub const THINKTHEN_COMPLETE_ANSWER_TAG_V1: u32 = 3;

pub const THINKTHEN_COMPLETE_ANSWER_SCORE_V1: u32 = 4;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_answer_data_v1 {
    pub yes_no: *const thinkthen_complete_answer_yes_no_v1,
    pub choice: *const thinkthen_complete_answer_choice_v1,
    pub tag: *const thinkthen_complete_answer_tag_v1,
    pub score: *const thinkthen_complete_answer_score_v1,
}
impl std::fmt::Debug for thinkthen_complete_answer_data_v1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("union payload")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answer_v1 {
    pub kind: u32,
    pub data: thinkthen_complete_answer_data_v1,
}

pub(crate) fn read_thinkthen_complete_answer_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_answer_v1, ErrorKind> {
    if node.is_member("kind", "yes_no") {
        let value =
            read_thinkthen_complete_answer_yes_no_v1(node, store).map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_answer_v1 {
            kind: THINKTHEN_COMPLETE_ANSWER_YES_NO_V1,
            data: thinkthen_complete_answer_data_v1 { yes_no: value },
        });
    }
    if node.is_member("kind", "choice") {
        let value =
            read_thinkthen_complete_answer_choice_v1(node, store).map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_answer_v1 {
            kind: THINKTHEN_COMPLETE_ANSWER_CHOICE_V1,
            data: thinkthen_complete_answer_data_v1 { choice: value },
        });
    }
    if node.is_member("kind", "tag") {
        let value =
            read_thinkthen_complete_answer_tag_v1(node, store).map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_answer_v1 {
            kind: THINKTHEN_COMPLETE_ANSWER_TAG_V1,
            data: thinkthen_complete_answer_data_v1 { tag: value },
        });
    }
    if node.is_member("kind", "score") {
        let value =
            read_thinkthen_complete_answer_score_v1(node, store).map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_answer_v1 {
            kind: THINKTHEN_COMPLETE_ANSWER_SCORE_V1,
            data: thinkthen_complete_answer_data_v1 { score: value },
        });
    }
    Err(ErrorKind::Defect)
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_number_map_entry_v1 {
    pub name: thinkthen_complete_utf8_v1,
    pub value: f64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_number_map_v1 {
    pub data: *const thinkthen_complete_number_map_entry_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_number_map_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_number_map_v1, ErrorKind> {
    let mut values = Vec::new();
    for (key, item) in node.object()? {
        values.push(thinkthen_complete_number_map_entry_v1 {
            name: store.text(key),
            value: read_number(item, store)?,
        });
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_number_map_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answer_choice_v1 {
    pub confidence: thinkthen_complete_f64_presence_v1,
    pub kind: thinkthen_complete_utf8_v1,
    pub pick: thinkthen_complete_utf8_v1,
    pub probabilities: thinkthen_complete_number_map_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_answer_choice_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_answer_choice_v1, ErrorKind> {
    Ok(thinkthen_complete_answer_choice_v1 {
        confidence: {
            let (presence, value) =
                read_presence(node.member("confidence"), false, 0.0, read_number, store)?;
            thinkthen_complete_f64_presence_v1 { presence, value }
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
            read_thinkthen_complete_number_map_v1(value, store)?
        },
        extensions: store.extensions(node, &["confidence", "kind", "pick", "probabilities"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answer_score_v1 {
    pub confidence: thinkthen_complete_f64_presence_v1,
    pub kind: thinkthen_complete_utf8_v1,
    pub level: thinkthen_complete_utf8_v1,
    pub probabilities: thinkthen_complete_number_map_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_answer_score_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_answer_score_v1, ErrorKind> {
    Ok(thinkthen_complete_answer_score_v1 {
        confidence: {
            let (presence, value) =
                read_presence(node.member("confidence"), false, 0.0, read_number, store)?;
            thinkthen_complete_f64_presence_v1 { presence, value }
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
            read_thinkthen_complete_number_map_v1(value, store)?
        },
        extensions: store.extensions(node, &["confidence", "kind", "level", "probabilities"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answer_tag_v1 {
    pub kind: thinkthen_complete_utf8_v1,
    pub probabilities: thinkthen_complete_number_map_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_answer_tag_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_answer_tag_v1, ErrorKind> {
    Ok(thinkthen_complete_answer_tag_v1 {
        kind: {
            let value = node.required("kind")?;
            read_text(value, store)?
        },
        probabilities: {
            let value = node.required("probabilities")?;
            read_thinkthen_complete_number_map_v1(value, store)?
        },
        extensions: store.extensions(node, &["kind", "probabilities"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answer_yes_no_v1 {
    pub kind: thinkthen_complete_utf8_v1,
    pub probability: f64,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_answer_yes_no_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_answer_yes_no_v1, ErrorKind> {
    Ok(thinkthen_complete_answer_yes_no_v1 {
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
pub struct thinkthen_complete_attempt_outcome_v1 {
    pub kind: u32,
}

pub const THINKTHEN_COMPLETE_ATTEMPT_OUTCOME_OK_V1: u32 = 1;

pub const THINKTHEN_COMPLETE_ATTEMPT_OUTCOME_STATUS_V1: u32 = 2;

pub const THINKTHEN_COMPLETE_ATTEMPT_OUTCOME_TRANSPORT_V1: u32 = 3;

pub(crate) fn read_thinkthen_complete_attempt_outcome_v1(
    node: &Node,
    _store: &mut Storage,
) -> Result<thinkthen_complete_attempt_outcome_v1, ErrorKind> {
    let kind = match node.text()? {
        "ok" => THINKTHEN_COMPLETE_ATTEMPT_OUTCOME_OK_V1,
        "status" => THINKTHEN_COMPLETE_ATTEMPT_OUTCOME_STATUS_V1,
        "transport" => THINKTHEN_COMPLETE_ATTEMPT_OUTCOME_TRANSPORT_V1,
        _ => return Err(ErrorKind::Defect),
    };
    Ok(thinkthen_complete_attempt_outcome_v1 { kind })
}

pub const THINKTHEN_COMPLETE_BATCH_SETTING_INTEGER_V1: u32 = 1;

pub const THINKTHEN_COMPLETE_BATCH_SETTING_STRING_V1: u32 = 2;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_batch_setting_data_v1 {
    pub integer: u64,
    pub string: thinkthen_complete_utf8_v1,
}
impl std::fmt::Debug for thinkthen_complete_batch_setting_data_v1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("union payload")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_batch_setting_v1 {
    pub kind: u32,
    pub data: thinkthen_complete_batch_setting_data_v1,
}

pub(crate) fn read_thinkthen_complete_batch_setting_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_batch_setting_v1, ErrorKind> {
    if node.kind() == "number" {
        let value = read_number(node, store)?;
        return Ok(thinkthen_complete_batch_setting_v1 {
            kind: THINKTHEN_COMPLETE_BATCH_SETTING_INTEGER_V1,
            data: thinkthen_complete_batch_setting_data_v1 { integer: value },
        });
    }
    if node.kind() == "string" {
        let value = read_text(node, store)?;
        return Ok(thinkthen_complete_batch_setting_v1 {
            kind: THINKTHEN_COMPLETE_BATCH_SETTING_STRING_V1,
            data: thinkthen_complete_batch_setting_data_v1 { string: value },
        });
    }
    Err(ErrorKind::Defect)
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_batch_warning_v1 {
    pub running: *const thinkthen_complete_batch_setting_v1,
    pub tuned_for: *const thinkthen_complete_batch_setting_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_batch_warning_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_batch_warning_v1, ErrorKind> {
    Ok(thinkthen_complete_batch_warning_v1 {
        running: {
            let value = node.required("running")?;
            read_thinkthen_complete_batch_setting_v1(value, store).map(|value| store.hold(value))?
        },
        tuned_for: {
            let value = node.required("tuned_for")?;
            read_thinkthen_complete_batch_setting_v1(value, store).map(|value| store.hold(value))?
        },
        extensions: store.extensions(node, &["running", "tuned_for"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_entity_v1 {
    pub end: u64,
    pub file: thinkthen_complete_utf8_presence_v1,
    pub first_line: thinkthen_complete_u64_presence_v1,
    pub kind: thinkthen_complete_utf8_v1,
    pub last_line: thinkthen_complete_u64_presence_v1,
    pub length: u64,
    pub start: u64,
    pub strength: f64,
    pub text: thinkthen_complete_utf8_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_entity_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_entity_v1, ErrorKind> {
    Ok(thinkthen_complete_entity_v1 {
        end: {
            let value = node.required("end")?;
            read_number(value, store)?
        },
        file: {
            let (presence, value) = read_presence(
                node.member("file"),
                false,
                thinkthen_complete_utf8_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence_v1 { presence, value }
        },
        first_line: {
            let (presence, value) =
                read_presence(node.member("first_line"), false, 0, read_number, store)?;
            thinkthen_complete_u64_presence_v1 { presence, value }
        },
        kind: {
            let value = node.required("kind")?;
            read_text(value, store)?
        },
        last_line: {
            let (presence, value) =
                read_presence(node.member("last_line"), false, 0, read_number, store)?;
            thinkthen_complete_u64_presence_v1 { presence, value }
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
pub struct thinkthen_complete_entity_edge_v1 {
    pub either: thinkthen_complete_u32_presence_v1,
    pub probability: f64,
    pub relation: thinkthen_complete_utf8_v1,
    pub source: *const thinkthen_complete_entity_v1,
    pub target: *const thinkthen_complete_entity_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_entity_edge_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_entity_edge_v1, ErrorKind> {
    Ok(thinkthen_complete_entity_edge_v1 {
        either: {
            let (presence, value) =
                read_presence(node.member("either"), false, 0, read_boolean, store)?;
            thinkthen_complete_u32_presence_v1 { presence, value }
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
            read_thinkthen_complete_entity_v1(value, store).map(|value| store.hold(value))?
        },
        target: {
            let value = node.required("target")?;
            read_thinkthen_complete_entity_v1(value, store).map(|value| store.hold(value))?
        },
        extensions: store.extensions(
            node,
            &["either", "probability", "relation", "source", "target"],
        )?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_failed_v1 {
    pub failed: *const thinkthen_complete_failure_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_failed_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_failed_v1, ErrorKind> {
    Ok(thinkthen_complete_failed_v1 {
        failed: {
            let value = node.required("failed")?;
            read_thinkthen_complete_failure_v1(value, store).map(|value| store.hold(value))?
        },
        extensions: store.extensions(node, &["failed"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_failure_field_kind_v1 {
    pub kind: u32,
}

pub const THINKTHEN_COMPLETE_FAILURE_FIELD_KIND_BACKEND_V1: u32 = 1;

pub(crate) fn read_thinkthen_complete_failure_field_kind_v1(
    node: &Node,
    _store: &mut Storage,
) -> Result<thinkthen_complete_failure_field_kind_v1, ErrorKind> {
    let kind = match node.text()? {
        "backend" => THINKTHEN_COMPLETE_FAILURE_FIELD_KIND_BACKEND_V1,
        _ => return Err(ErrorKind::Defect),
    };
    Ok(thinkthen_complete_failure_field_kind_v1 { kind })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_failure_v1 {
    pub cause: *const thinkthen_complete_failure_cause_v1,
    pub kind: *const thinkthen_complete_failure_field_kind_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_failure_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_failure_v1, ErrorKind> {
    Ok(thinkthen_complete_failure_v1 {
        cause: {
            let value = node.required("cause")?;
            read_thinkthen_complete_failure_cause_v1(value, store).map(|value| store.hold(value))?
        },
        kind: {
            let value = node.required("kind")?;
            read_thinkthen_complete_failure_field_kind_v1(value, store)
                .map(|value| store.hold(value))?
        },
        extensions: store.extensions(node, &["cause", "kind"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_failure_cause_v1 {
    pub kind: u32,
}

pub const THINKTHEN_COMPLETE_FAILURE_CAUSE_MISSING_ANSWER_V1: u32 = 1;

pub const THINKTHEN_COMPLETE_FAILURE_CAUSE_WRONG_KIND_V1: u32 = 2;

pub const THINKTHEN_COMPLETE_FAILURE_CAUSE_MISSING_PROBABILITY_V1: u32 = 3;

pub const THINKTHEN_COMPLETE_FAILURE_CAUSE_INVALID_PROBABILITY_V1: u32 = 4;

pub const THINKTHEN_COMPLETE_FAILURE_CAUSE_INVALID_DISTRIBUTION_V1: u32 = 5;

pub const THINKTHEN_COMPLETE_FAILURE_CAUSE_UNEXPECTED_PROBABILITY_V1: u32 = 6;

pub(crate) fn read_thinkthen_complete_failure_cause_v1(
    node: &Node,
    _store: &mut Storage,
) -> Result<thinkthen_complete_failure_cause_v1, ErrorKind> {
    let kind = match node.text()? {
        "missing_answer" => THINKTHEN_COMPLETE_FAILURE_CAUSE_MISSING_ANSWER_V1,
        "wrong_kind" => THINKTHEN_COMPLETE_FAILURE_CAUSE_WRONG_KIND_V1,
        "missing_probability" => THINKTHEN_COMPLETE_FAILURE_CAUSE_MISSING_PROBABILITY_V1,
        "invalid_probability" => THINKTHEN_COMPLETE_FAILURE_CAUSE_INVALID_PROBABILITY_V1,
        "invalid_distribution" => THINKTHEN_COMPLETE_FAILURE_CAUSE_INVALID_DISTRIBUTION_V1,
        "unexpected_probability" => THINKTHEN_COMPLETE_FAILURE_CAUSE_UNEXPECTED_PROBABILITY_V1,
        _ => return Err(ErrorKind::Defect),
    };
    Ok(thinkthen_complete_failure_cause_v1 { kind })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_failure_kind_v1 {
    pub kind: u32,
}

pub const THINKTHEN_COMPLETE_FAILURE_KIND_USAGE_V1: u32 = 1;

pub const THINKTHEN_COMPLETE_FAILURE_KIND_BACKEND_V1: u32 = 2;

pub const THINKTHEN_COMPLETE_FAILURE_KIND_LOCAL_V1: u32 = 3;

pub const THINKTHEN_COMPLETE_FAILURE_KIND_CANCELLED_V1: u32 = 4;

pub const THINKTHEN_COMPLETE_FAILURE_KIND_DEADLINE_V1: u32 = 5;

pub const THINKTHEN_COMPLETE_FAILURE_KIND_DEFECT_V1: u32 = 6;

pub(crate) fn read_thinkthen_complete_failure_kind_v1(
    node: &Node,
    _store: &mut Storage,
) -> Result<thinkthen_complete_failure_kind_v1, ErrorKind> {
    let kind = match node.text()? {
        "usage" => THINKTHEN_COMPLETE_FAILURE_KIND_USAGE_V1,
        "backend" => THINKTHEN_COMPLETE_FAILURE_KIND_BACKEND_V1,
        "local" => THINKTHEN_COMPLETE_FAILURE_KIND_LOCAL_V1,
        "cancelled" => THINKTHEN_COMPLETE_FAILURE_KIND_CANCELLED_V1,
        "deadline" => THINKTHEN_COMPLETE_FAILURE_KIND_DEADLINE_V1,
        "defect" => THINKTHEN_COMPLETE_FAILURE_KIND_DEFECT_V1,
        _ => return Err(ErrorKind::Defect),
    };
    Ok(thinkthen_complete_failure_kind_v1 { kind })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_find_answer_v1 {
    pub confidence: thinkthen_complete_f64_presence_v1,
    pub kind: thinkthen_complete_utf8_v1,
    pub pick: thinkthen_complete_utf8_v1,
    pub probabilities: thinkthen_complete_number_map_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_find_answer_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_find_answer_v1, ErrorKind> {
    Ok(thinkthen_complete_find_answer_v1 {
        confidence: {
            let (presence, value) =
                read_presence(node.member("confidence"), false, 0.0, read_number, store)?;
            thinkthen_complete_f64_presence_v1 { presence, value }
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
            read_thinkthen_complete_number_map_v1(value, store)?
        },
        extensions: store.extensions(node, &["confidence", "kind", "pick", "probabilities"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_batch_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_batch_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_input_declaration_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_input_declaration_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_label_list_v1 {
    pub data: *const *const thinkthen_complete_label_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_label_list_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_label_list_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(read_thinkthen_complete_label_v1(item, store).map(|value| store.hold(value))?);
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_label_list_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_label_list_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_label_list_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_question_name_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_question_name_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_wording_version_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_wording_version_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_find_question_document_v1 {
    pub batch: thinkthen_complete_batch_presence_v1,
    pub context_schema: thinkthen_complete_input_declaration_presence_v1,
    pub item_schema: thinkthen_complete_input_declaration_presence_v1,
    pub label_details: thinkthen_complete_label_list_presence_v1,
    pub model: thinkthen_complete_utf8_presence_v1,
    pub name: thinkthen_complete_question_name_presence_v1,
    pub none: u32,
    pub on: thinkthen_complete_string_list_presence_v1,
    pub profile: thinkthen_complete_utf8_presence_v1,
    pub text: thinkthen_complete_json_presence_v1,
    pub verb: thinkthen_complete_utf8_v1,
    pub wording_version: thinkthen_complete_wording_version_presence_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_find_question_document_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_find_question_document_v1, ErrorKind> {
    Ok(thinkthen_complete_find_question_document_v1 {
        batch: {
            let (presence, value) = read_presence(
                node.member("batch"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_batch_v1(value, store).map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_batch_presence_v1 { presence, value }
        },
        context_schema: {
            let (presence, value) = read_presence(
                node.member("context_schema"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_input_declaration_v1(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_input_declaration_presence_v1 { presence, value }
        },
        item_schema: {
            let (presence, value) = read_presence(
                node.member("item_schema"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_input_declaration_v1(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_input_declaration_presence_v1 { presence, value }
        },
        label_details: {
            let (presence, value) = read_presence(
                node.member("label_details"),
                false,
                thinkthen_complete_label_list_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_label_list_v1,
                store,
            )?;
            thinkthen_complete_label_list_presence_v1 { presence, value }
        },
        model: {
            let (presence, value) = read_presence(
                node.member("model"),
                false,
                thinkthen_complete_utf8_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence_v1 { presence, value }
        },
        name: {
            let (presence, value) = read_presence(
                node.member("name"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_question_name_v1(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_question_name_presence_v1 { presence, value }
        },
        none: {
            let value = node.required("none")?;
            read_boolean(value, store)?
        },
        on: {
            let (presence, value) = read_presence(
                node.member("on"),
                false,
                thinkthen_complete_string_list_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_string_list_v1,
                store,
            )?;
            thinkthen_complete_string_list_presence_v1 { presence, value }
        },
        profile: {
            let (presence, value) = read_presence(
                node.member("profile"),
                false,
                thinkthen_complete_utf8_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence_v1 { presence, value }
        },
        text: {
            let (presence, value) = read_presence(
                node.member("text"),
                true,
                std::ptr::null(),
                read_json,
                store,
            )?;
            thinkthen_complete_json_presence_v1 { presence, value }
        },
        verb: {
            let value = node.required("verb")?;
            read_text(value, store)?
        },
        wording_version: {
            let (presence, value) = read_presence(
                node.member("wording_version"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_wording_version_v1(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_wording_version_presence_v1 { presence, value }
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
                "wording_version",
            ],
        )?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_number_map_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_number_map_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_name_odds_v1 {
    pub edges: thinkthen_complete_number_map_presence_v1,
    pub end: u64,
    pub kinds: thinkthen_complete_number_map_presence_v1,
    pub start: u64,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_name_odds_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_name_odds_v1, ErrorKind> {
    Ok(thinkthen_complete_name_odds_v1 {
        edges: {
            let (presence, value) = read_presence(
                node.member("edges"),
                true,
                thinkthen_complete_number_map_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_number_map_v1,
                store,
            )?;
            thinkthen_complete_number_map_presence_v1 { presence, value }
        },
        end: {
            let value = node.required("end")?;
            read_number(value, store)?
        },
        kinds: {
            let (presence, value) = read_presence(
                node.member("kinds"),
                true,
                thinkthen_complete_number_map_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_number_map_v1,
                store,
            )?;
            thinkthen_complete_number_map_presence_v1 { presence, value }
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
pub struct thinkthen_complete_pair_odds_v1 {
    pub probability: f64,
    pub relation: thinkthen_complete_utf8_v1,
    pub source: *const thinkthen_complete_place_v1,
    pub target: *const thinkthen_complete_place_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_pair_odds_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_pair_odds_v1, ErrorKind> {
    Ok(thinkthen_complete_pair_odds_v1 {
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
            read_thinkthen_complete_place_v1(value, store).map(|value| store.hold(value))?
        },
        target: {
            let value = node.required("target")?;
            read_thinkthen_complete_place_v1(value, store).map(|value| store.hold(value))?
        },
        extensions: store.extensions(node, &["probability", "relation", "source", "target"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_piece_odds_v1 {
    pub end: u64,
    pub start: u64,
    pub tags: thinkthen_complete_number_map_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_piece_odds_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_piece_odds_v1, ErrorKind> {
    Ok(thinkthen_complete_piece_odds_v1 {
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
            read_thinkthen_complete_number_map_v1(value, store)?
        },
        extensions: store.extensions(node, &["end", "start", "tags"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_place_v1 {
    pub end: u64,
    pub start: u64,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_place_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_place_v1, ErrorKind> {
    Ok(thinkthen_complete_place_v1 {
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
pub struct thinkthen_complete_profile_warning_v1 {
    pub running: thinkthen_complete_utf8_v1,
    pub tuned_for: thinkthen_complete_utf8_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_profile_warning_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_profile_warning_v1, ErrorKind> {
    Ok(thinkthen_complete_profile_warning_v1 {
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

pub const THINKTHEN_COMPLETE_QUESTION_DOCUMENT_DECIDE_V1: u32 = 1;

pub const THINKTHEN_COMPLETE_QUESTION_DOCUMENT_CHOOSE_V1: u32 = 2;

pub const THINKTHEN_COMPLETE_QUESTION_DOCUMENT_TAG_V1: u32 = 3;

pub const THINKTHEN_COMPLETE_QUESTION_DOCUMENT_SCORE_V1: u32 = 4;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_question_document_data_v1 {
    pub decide: *const thinkthen_complete_question_document_decide_v1,
    pub choose: *const thinkthen_complete_question_document_choose_v1,
    pub tag: *const thinkthen_complete_question_document_tag_v1,
    pub score: *const thinkthen_complete_question_document_score_v1,
}
impl std::fmt::Debug for thinkthen_complete_question_document_data_v1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("union payload")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_question_document_v1 {
    pub kind: u32,
    pub data: thinkthen_complete_question_document_data_v1,
}

pub(crate) fn read_thinkthen_complete_question_document_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_question_document_v1, ErrorKind> {
    if node.is_member("verb", "decide") {
        let value = read_thinkthen_complete_question_document_decide_v1(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_question_document_v1 {
            kind: THINKTHEN_COMPLETE_QUESTION_DOCUMENT_DECIDE_V1,
            data: thinkthen_complete_question_document_data_v1 { decide: value },
        });
    }
    if node.is_member("verb", "choose") {
        let value = read_thinkthen_complete_question_document_choose_v1(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_question_document_v1 {
            kind: THINKTHEN_COMPLETE_QUESTION_DOCUMENT_CHOOSE_V1,
            data: thinkthen_complete_question_document_data_v1 { choose: value },
        });
    }
    if node.is_member("verb", "tag") {
        let value = read_thinkthen_complete_question_document_tag_v1(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_question_document_v1 {
            kind: THINKTHEN_COMPLETE_QUESTION_DOCUMENT_TAG_V1,
            data: thinkthen_complete_question_document_data_v1 { tag: value },
        });
    }
    if node.is_member("verb", "score") {
        let value = read_thinkthen_complete_question_document_score_v1(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_question_document_v1 {
            kind: THINKTHEN_COMPLETE_QUESTION_DOCUMENT_SCORE_V1,
            data: thinkthen_complete_question_document_data_v1 { score: value },
        });
    }
    Err(ErrorKind::Defect)
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_question_document_choose_v1 {
    pub batch: thinkthen_complete_batch_presence_v1,
    pub context_schema: thinkthen_complete_input_declaration_presence_v1,
    pub item_schema: thinkthen_complete_input_declaration_presence_v1,
    pub label_details: thinkthen_complete_label_list_presence_v1,
    pub model: thinkthen_complete_utf8_presence_v1,
    pub name: thinkthen_complete_question_name_presence_v1,
    pub on: thinkthen_complete_string_list_presence_v1,
    pub profile: thinkthen_complete_utf8_presence_v1,
    pub wording_version: thinkthen_complete_wording_version_presence_v1,
    pub options: thinkthen_complete_string_list_v1,
    pub text: thinkthen_complete_json_presence_v1,
    pub verb: thinkthen_complete_utf8_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_question_document_choose_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_question_document_choose_v1, ErrorKind> {
    Ok(thinkthen_complete_question_document_choose_v1 {
        batch: {
            let (presence, value) = read_presence(
                node.member("batch"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_batch_v1(value, store).map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_batch_presence_v1 { presence, value }
        },
        context_schema: {
            let (presence, value) = read_presence(
                node.member("context_schema"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_input_declaration_v1(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_input_declaration_presence_v1 { presence, value }
        },
        item_schema: {
            let (presence, value) = read_presence(
                node.member("item_schema"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_input_declaration_v1(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_input_declaration_presence_v1 { presence, value }
        },
        label_details: {
            let (presence, value) = read_presence(
                node.member("label_details"),
                false,
                thinkthen_complete_label_list_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_label_list_v1,
                store,
            )?;
            thinkthen_complete_label_list_presence_v1 { presence, value }
        },
        model: {
            let (presence, value) = read_presence(
                node.member("model"),
                false,
                thinkthen_complete_utf8_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence_v1 { presence, value }
        },
        name: {
            let (presence, value) = read_presence(
                node.member("name"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_question_name_v1(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_question_name_presence_v1 { presence, value }
        },
        on: {
            let (presence, value) = read_presence(
                node.member("on"),
                false,
                thinkthen_complete_string_list_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_string_list_v1,
                store,
            )?;
            thinkthen_complete_string_list_presence_v1 { presence, value }
        },
        profile: {
            let (presence, value) = read_presence(
                node.member("profile"),
                false,
                thinkthen_complete_utf8_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence_v1 { presence, value }
        },
        wording_version: {
            let (presence, value) = read_presence(
                node.member("wording_version"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_wording_version_v1(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_wording_version_presence_v1 { presence, value }
        },
        options: {
            let value = node.required("options")?;
            read_thinkthen_complete_string_list_v1(value, store)?
        },
        text: {
            let (presence, value) = read_presence(
                node.member("text"),
                true,
                std::ptr::null(),
                read_json,
                store,
            )?;
            thinkthen_complete_json_presence_v1 { presence, value }
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
                "wording_version",
                "options",
                "text",
                "verb",
            ],
        )?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_question_document_decide_v1 {
    pub batch: thinkthen_complete_batch_presence_v1,
    pub context_schema: thinkthen_complete_input_declaration_presence_v1,
    pub item_schema: thinkthen_complete_input_declaration_presence_v1,
    pub label_details: thinkthen_complete_label_list_presence_v1,
    pub model: thinkthen_complete_utf8_presence_v1,
    pub name: thinkthen_complete_question_name_presence_v1,
    pub on: thinkthen_complete_string_list_presence_v1,
    pub profile: thinkthen_complete_utf8_presence_v1,
    pub wording_version: thinkthen_complete_wording_version_presence_v1,
    pub false_: thinkthen_complete_json_presence_v1,
    pub text: thinkthen_complete_json_presence_v1,
    pub true_: thinkthen_complete_json_presence_v1,
    pub verb: thinkthen_complete_utf8_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_question_document_decide_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_question_document_decide_v1, ErrorKind> {
    Ok(thinkthen_complete_question_document_decide_v1 {
        batch: {
            let (presence, value) = read_presence(
                node.member("batch"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_batch_v1(value, store).map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_batch_presence_v1 { presence, value }
        },
        context_schema: {
            let (presence, value) = read_presence(
                node.member("context_schema"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_input_declaration_v1(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_input_declaration_presence_v1 { presence, value }
        },
        item_schema: {
            let (presence, value) = read_presence(
                node.member("item_schema"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_input_declaration_v1(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_input_declaration_presence_v1 { presence, value }
        },
        label_details: {
            let (presence, value) = read_presence(
                node.member("label_details"),
                false,
                thinkthen_complete_label_list_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_label_list_v1,
                store,
            )?;
            thinkthen_complete_label_list_presence_v1 { presence, value }
        },
        model: {
            let (presence, value) = read_presence(
                node.member("model"),
                false,
                thinkthen_complete_utf8_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence_v1 { presence, value }
        },
        name: {
            let (presence, value) = read_presence(
                node.member("name"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_question_name_v1(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_question_name_presence_v1 { presence, value }
        },
        on: {
            let (presence, value) = read_presence(
                node.member("on"),
                false,
                thinkthen_complete_string_list_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_string_list_v1,
                store,
            )?;
            thinkthen_complete_string_list_presence_v1 { presence, value }
        },
        profile: {
            let (presence, value) = read_presence(
                node.member("profile"),
                false,
                thinkthen_complete_utf8_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence_v1 { presence, value }
        },
        wording_version: {
            let (presence, value) = read_presence(
                node.member("wording_version"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_wording_version_v1(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_wording_version_presence_v1 { presence, value }
        },
        false_: {
            let (presence, value) = read_presence(
                node.member("false"),
                false,
                std::ptr::null(),
                read_json,
                store,
            )?;
            thinkthen_complete_json_presence_v1 { presence, value }
        },
        text: {
            let (presence, value) = read_presence(
                node.member("text"),
                true,
                std::ptr::null(),
                read_json,
                store,
            )?;
            thinkthen_complete_json_presence_v1 { presence, value }
        },
        true_: {
            let (presence, value) = read_presence(
                node.member("true"),
                false,
                std::ptr::null(),
                read_json,
                store,
            )?;
            thinkthen_complete_json_presence_v1 { presence, value }
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
                "wording_version",
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
pub struct thinkthen_complete_question_document_score_v1 {
    pub batch: thinkthen_complete_batch_presence_v1,
    pub context_schema: thinkthen_complete_input_declaration_presence_v1,
    pub item_schema: thinkthen_complete_input_declaration_presence_v1,
    pub label_details: thinkthen_complete_label_list_presence_v1,
    pub model: thinkthen_complete_utf8_presence_v1,
    pub name: thinkthen_complete_question_name_presence_v1,
    pub on: thinkthen_complete_string_list_presence_v1,
    pub profile: thinkthen_complete_utf8_presence_v1,
    pub wording_version: thinkthen_complete_wording_version_presence_v1,
    pub levels: thinkthen_complete_string_list_v1,
    pub text: thinkthen_complete_json_presence_v1,
    pub verb: thinkthen_complete_utf8_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_question_document_score_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_question_document_score_v1, ErrorKind> {
    Ok(thinkthen_complete_question_document_score_v1 {
        batch: {
            let (presence, value) = read_presence(
                node.member("batch"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_batch_v1(value, store).map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_batch_presence_v1 { presence, value }
        },
        context_schema: {
            let (presence, value) = read_presence(
                node.member("context_schema"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_input_declaration_v1(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_input_declaration_presence_v1 { presence, value }
        },
        item_schema: {
            let (presence, value) = read_presence(
                node.member("item_schema"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_input_declaration_v1(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_input_declaration_presence_v1 { presence, value }
        },
        label_details: {
            let (presence, value) = read_presence(
                node.member("label_details"),
                false,
                thinkthen_complete_label_list_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_label_list_v1,
                store,
            )?;
            thinkthen_complete_label_list_presence_v1 { presence, value }
        },
        model: {
            let (presence, value) = read_presence(
                node.member("model"),
                false,
                thinkthen_complete_utf8_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence_v1 { presence, value }
        },
        name: {
            let (presence, value) = read_presence(
                node.member("name"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_question_name_v1(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_question_name_presence_v1 { presence, value }
        },
        on: {
            let (presence, value) = read_presence(
                node.member("on"),
                false,
                thinkthen_complete_string_list_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_string_list_v1,
                store,
            )?;
            thinkthen_complete_string_list_presence_v1 { presence, value }
        },
        profile: {
            let (presence, value) = read_presence(
                node.member("profile"),
                false,
                thinkthen_complete_utf8_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence_v1 { presence, value }
        },
        wording_version: {
            let (presence, value) = read_presence(
                node.member("wording_version"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_wording_version_v1(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_wording_version_presence_v1 { presence, value }
        },
        levels: {
            let value = node.required("levels")?;
            read_thinkthen_complete_string_list_v1(value, store)?
        },
        text: {
            let (presence, value) = read_presence(
                node.member("text"),
                true,
                std::ptr::null(),
                read_json,
                store,
            )?;
            thinkthen_complete_json_presence_v1 { presence, value }
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
                "wording_version",
                "levels",
                "text",
                "verb",
            ],
        )?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_question_document_tag_v1 {
    pub batch: thinkthen_complete_batch_presence_v1,
    pub context_schema: thinkthen_complete_input_declaration_presence_v1,
    pub item_schema: thinkthen_complete_input_declaration_presence_v1,
    pub label_details: thinkthen_complete_label_list_presence_v1,
    pub model: thinkthen_complete_utf8_presence_v1,
    pub name: thinkthen_complete_question_name_presence_v1,
    pub on: thinkthen_complete_string_list_presence_v1,
    pub profile: thinkthen_complete_utf8_presence_v1,
    pub wording_version: thinkthen_complete_wording_version_presence_v1,
    pub labels: thinkthen_complete_string_list_v1,
    pub text: thinkthen_complete_json_presence_v1,
    pub verb: thinkthen_complete_utf8_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_question_document_tag_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_question_document_tag_v1, ErrorKind> {
    Ok(thinkthen_complete_question_document_tag_v1 {
        batch: {
            let (presence, value) = read_presence(
                node.member("batch"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_batch_v1(value, store).map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_batch_presence_v1 { presence, value }
        },
        context_schema: {
            let (presence, value) = read_presence(
                node.member("context_schema"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_input_declaration_v1(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_input_declaration_presence_v1 { presence, value }
        },
        item_schema: {
            let (presence, value) = read_presence(
                node.member("item_schema"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_input_declaration_v1(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_input_declaration_presence_v1 { presence, value }
        },
        label_details: {
            let (presence, value) = read_presence(
                node.member("label_details"),
                false,
                thinkthen_complete_label_list_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_label_list_v1,
                store,
            )?;
            thinkthen_complete_label_list_presence_v1 { presence, value }
        },
        model: {
            let (presence, value) = read_presence(
                node.member("model"),
                false,
                thinkthen_complete_utf8_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence_v1 { presence, value }
        },
        name: {
            let (presence, value) = read_presence(
                node.member("name"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_question_name_v1(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_question_name_presence_v1 { presence, value }
        },
        on: {
            let (presence, value) = read_presence(
                node.member("on"),
                false,
                thinkthen_complete_string_list_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_string_list_v1,
                store,
            )?;
            thinkthen_complete_string_list_presence_v1 { presence, value }
        },
        profile: {
            let (presence, value) = read_presence(
                node.member("profile"),
                false,
                thinkthen_complete_utf8_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence_v1 { presence, value }
        },
        wording_version: {
            let (presence, value) = read_presence(
                node.member("wording_version"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_wording_version_v1(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_wording_version_presence_v1 { presence, value }
        },
        labels: {
            let value = node.required("labels")?;
            read_thinkthen_complete_string_list_v1(value, store)?
        },
        text: {
            let (presence, value) = read_presence(
                node.member("text"),
                true,
                std::ptr::null(),
                read_json,
                store,
            )?;
            thinkthen_complete_json_presence_v1 { presence, value }
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
                "wording_version",
                "labels",
                "text",
                "verb",
            ],
        )?,
    })
}

pub const THINKTHEN_COMPLETE_RECOGNIZE_ENTITIES_V1: u32 = 1;

pub const THINKTHEN_COMPLETE_RECOGNIZE_MODE_PROPOSALS_V1: u32 = 2;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_recognize_data_v1 {
    pub entities: *const thinkthen_complete_recognize_entities_v1,
    pub mode_proposals: *const thinkthen_complete_recognize_mode_proposals_v1,
}
impl std::fmt::Debug for thinkthen_complete_recognize_data_v1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("union payload")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_recognize_v1 {
    pub kind: u32,
    pub data: thinkthen_complete_recognize_data_v1,
}

pub(crate) fn read_thinkthen_complete_recognize_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_recognize_v1, ErrorKind> {
    if node.member("entities").is_some()
        && node.member("mode").is_none()
        && node.member("proposals").is_none()
    {
        let value = read_thinkthen_complete_recognize_entities_v1(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_recognize_v1 {
            kind: THINKTHEN_COMPLETE_RECOGNIZE_ENTITIES_V1,
            data: thinkthen_complete_recognize_data_v1 { entities: value },
        });
    }
    if node.member("mode").is_some()
        && node.member("proposals").is_some()
        && node.member("entities").is_none()
    {
        let value = read_thinkthen_complete_recognize_mode_proposals_v1(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_recognize_v1 {
            kind: THINKTHEN_COMPLETE_RECOGNIZE_MODE_PROPOSALS_V1,
            data: thinkthen_complete_recognize_data_v1 {
                mode_proposals: value,
            },
        });
    }
    Err(ErrorKind::Defect)
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_json_map_entry_v1 {
    pub name: thinkthen_complete_utf8_v1,
    pub value: *const thinkthen_complete_json_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_json_map_v1 {
    pub data: *const thinkthen_complete_json_map_entry_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_json_map_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_json_map_v1, ErrorKind> {
    let mut values = Vec::new();
    for (key, item) in node.object()? {
        values.push(thinkthen_complete_json_map_entry_v1 {
            name: store.text(key),
            value: read_json(item, store)?,
        });
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_json_map_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_recognition_mode_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_recognition_mode_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_relation_rule_list_v1 {
    pub data: *const *const thinkthen_complete_relation_rule_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_relation_rule_list_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_relation_rule_list_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_relation_rule_v1(item, store).map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_relation_rule_list_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_relation_rule_list_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_relation_rule_list_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_recognition_stage_context_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_recognition_stage_context_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_recognize_question_document_v1 {
    pub batch: thinkthen_complete_batch_presence_v1,
    pub context_schema: thinkthen_complete_input_declaration_presence_v1,
    pub entity_definition: thinkthen_complete_json_presence_v1,
    pub instructions: thinkthen_complete_json_presence_v1,
    pub item_schema: thinkthen_complete_input_declaration_presence_v1,
    pub kinds: thinkthen_complete_json_map_v1,
    pub label_details: thinkthen_complete_label_list_presence_v1,
    pub mode: thinkthen_complete_recognition_mode_presence_v1,
    pub model: thinkthen_complete_utf8_presence_v1,
    pub name: thinkthen_complete_question_name_presence_v1,
    pub on: thinkthen_complete_string_list_presence_v1,
    pub profile: thinkthen_complete_utf8_presence_v1,
    pub relation_threshold: thinkthen_complete_threshold_presence_v1,
    pub relations: thinkthen_complete_relation_rule_list_presence_v1,
    pub snippet_pieces: thinkthen_complete_u64_presence_v1,
    pub stage_context: thinkthen_complete_recognition_stage_context_presence_v1,
    pub threshold: *const thinkthen_complete_threshold_v1,
    pub verb: *const thinkthen_complete_verb_v1,
    pub wording_version: thinkthen_complete_wording_version_presence_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_recognize_question_document_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_recognize_question_document_v1, ErrorKind> {
    Ok(thinkthen_complete_recognize_question_document_v1 {
        batch: {
            let (presence, value) = read_presence(
                node.member("batch"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_batch_v1(value, store).map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_batch_presence_v1 { presence, value }
        },
        context_schema: {
            let (presence, value) = read_presence(
                node.member("context_schema"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_input_declaration_v1(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_input_declaration_presence_v1 { presence, value }
        },
        entity_definition: {
            let (presence, value) = read_presence(
                node.member("entity_definition"),
                false,
                std::ptr::null(),
                read_json,
                store,
            )?;
            thinkthen_complete_json_presence_v1 { presence, value }
        },
        instructions: {
            let (presence, value) = read_presence(
                node.member("instructions"),
                false,
                std::ptr::null(),
                read_json,
                store,
            )?;
            thinkthen_complete_json_presence_v1 { presence, value }
        },
        item_schema: {
            let (presence, value) = read_presence(
                node.member("item_schema"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_input_declaration_v1(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_input_declaration_presence_v1 { presence, value }
        },
        kinds: {
            let value = node.required("kinds")?;
            read_thinkthen_complete_json_map_v1(value, store)?
        },
        label_details: {
            let (presence, value) = read_presence(
                node.member("label_details"),
                false,
                thinkthen_complete_label_list_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_label_list_v1,
                store,
            )?;
            thinkthen_complete_label_list_presence_v1 { presence, value }
        },
        mode: {
            let (presence, value) = read_presence(
                node.member("mode"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_recognition_mode_v1(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_recognition_mode_presence_v1 { presence, value }
        },
        model: {
            let (presence, value) = read_presence(
                node.member("model"),
                false,
                thinkthen_complete_utf8_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence_v1 { presence, value }
        },
        name: {
            let (presence, value) = read_presence(
                node.member("name"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_question_name_v1(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_question_name_presence_v1 { presence, value }
        },
        on: {
            let (presence, value) = read_presence(
                node.member("on"),
                false,
                thinkthen_complete_string_list_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_string_list_v1,
                store,
            )?;
            thinkthen_complete_string_list_presence_v1 { presence, value }
        },
        profile: {
            let (presence, value) = read_presence(
                node.member("profile"),
                false,
                thinkthen_complete_utf8_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence_v1 { presence, value }
        },
        relation_threshold: {
            let (presence, value) = read_presence(
                node.member("relation_threshold"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_threshold_v1(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_threshold_presence_v1 { presence, value }
        },
        relations: {
            let (presence, value) = read_presence(
                node.member("relations"),
                false,
                thinkthen_complete_relation_rule_list_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_relation_rule_list_v1,
                store,
            )?;
            thinkthen_complete_relation_rule_list_presence_v1 { presence, value }
        },
        snippet_pieces: {
            let (presence, value) =
                read_presence(node.member("snippet_pieces"), false, 0, read_number, store)?;
            thinkthen_complete_u64_presence_v1 { presence, value }
        },
        stage_context: {
            let (presence, value) = read_presence(
                node.member("stage_context"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_recognition_stage_context_v1(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_recognition_stage_context_presence_v1 { presence, value }
        },
        threshold: {
            let value = node.required("threshold")?;
            read_thinkthen_complete_threshold_v1(value, store).map(|value| store.hold(value))?
        },
        verb: {
            let value = node.required("verb")?;
            read_thinkthen_complete_verb_v1(value, store).map(|value| store.hold(value))?
        },
        wording_version: {
            let (presence, value) = read_presence(
                node.member("wording_version"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_wording_version_v1(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_wording_version_presence_v1 { presence, value }
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
                "wording_version",
            ],
        )?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_entity_list_v1 {
    pub data: *const *const thinkthen_complete_entity_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_entity_list_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_entity_list_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(read_thinkthen_complete_entity_v1(item, store).map(|value| store.hold(value))?);
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_entity_list_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_entity_edge_list_v1 {
    pub data: *const *const thinkthen_complete_entity_edge_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_entity_edge_list_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_entity_edge_list_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_entity_edge_v1(item, store).map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_entity_edge_list_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_entity_edge_list_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_entity_edge_list_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_recognize_entities_v1 {
    pub entities: thinkthen_complete_entity_list_v1,
    pub relations: thinkthen_complete_entity_edge_list_presence_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_recognize_entities_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_recognize_entities_v1, ErrorKind> {
    Ok(thinkthen_complete_recognize_entities_v1 {
        entities: {
            let value = node.required("entities")?;
            read_thinkthen_complete_entity_list_v1(value, store)?
        },
        relations: {
            let (presence, value) = read_presence(
                node.member("relations"),
                false,
                thinkthen_complete_entity_edge_list_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_entity_edge_list_v1,
                store,
            )?;
            thinkthen_complete_entity_edge_list_presence_v1 { presence, value }
        },
        extensions: store.extensions(node, &["entities", "relations"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_recognize_mode_proposals_v1 {
    pub mode: *const thinkthen_complete_boundary_mode_v1,
    pub proposals: thinkthen_complete_boundary_proposal_list_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_recognize_mode_proposals_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_recognize_mode_proposals_v1, ErrorKind> {
    Ok(thinkthen_complete_recognize_mode_proposals_v1 {
        mode: {
            let value = node.required("mode")?;
            read_thinkthen_complete_boundary_mode_v1(value, store).map(|value| store.hold(value))?
        },
        proposals: {
            let value = node.required("proposals")?;
            read_thinkthen_complete_boundary_proposal_list_v1(value, store)?
        },
        extensions: store.extensions(node, &["mode", "proposals"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_relate_fields_v1 {
    pub kind: thinkthen_complete_utf8_v1,
    pub name: thinkthen_complete_utf8_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_relate_fields_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_relate_fields_v1, ErrorKind> {
    Ok(thinkthen_complete_relate_fields_v1 {
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
pub struct thinkthen_complete_relate_fields_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_relate_fields_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_relate_question_document_v1 {
    pub batch: thinkthen_complete_batch_presence_v1,
    pub context_schema: thinkthen_complete_input_declaration_presence_v1,
    pub fields: thinkthen_complete_relate_fields_presence_v1,
    pub item_schema: thinkthen_complete_input_declaration_presence_v1,
    pub label_details: thinkthen_complete_label_list_presence_v1,
    pub model: thinkthen_complete_utf8_presence_v1,
    pub name: thinkthen_complete_question_name_presence_v1,
    pub on: thinkthen_complete_string_list_presence_v1,
    pub profile: thinkthen_complete_utf8_presence_v1,
    pub relations: thinkthen_complete_relation_rule_list_v1,
    pub threshold: *const thinkthen_complete_threshold_v1,
    pub verb: thinkthen_complete_utf8_v1,
    pub wording_version: thinkthen_complete_wording_version_presence_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_relate_question_document_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_relate_question_document_v1, ErrorKind> {
    Ok(thinkthen_complete_relate_question_document_v1 {
        batch: {
            let (presence, value) = read_presence(
                node.member("batch"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_batch_v1(value, store).map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_batch_presence_v1 { presence, value }
        },
        context_schema: {
            let (presence, value) = read_presence(
                node.member("context_schema"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_input_declaration_v1(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_input_declaration_presence_v1 { presence, value }
        },
        fields: {
            let (presence, value) = read_presence(
                node.member("fields"),
                true,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_relate_fields_v1(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_relate_fields_presence_v1 { presence, value }
        },
        item_schema: {
            let (presence, value) = read_presence(
                node.member("item_schema"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_input_declaration_v1(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_input_declaration_presence_v1 { presence, value }
        },
        label_details: {
            let (presence, value) = read_presence(
                node.member("label_details"),
                false,
                thinkthen_complete_label_list_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_label_list_v1,
                store,
            )?;
            thinkthen_complete_label_list_presence_v1 { presence, value }
        },
        model: {
            let (presence, value) = read_presence(
                node.member("model"),
                false,
                thinkthen_complete_utf8_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence_v1 { presence, value }
        },
        name: {
            let (presence, value) = read_presence(
                node.member("name"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_question_name_v1(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_question_name_presence_v1 { presence, value }
        },
        on: {
            let (presence, value) = read_presence(
                node.member("on"),
                false,
                thinkthen_complete_string_list_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_string_list_v1,
                store,
            )?;
            thinkthen_complete_string_list_presence_v1 { presence, value }
        },
        profile: {
            let (presence, value) = read_presence(
                node.member("profile"),
                false,
                thinkthen_complete_utf8_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence_v1 { presence, value }
        },
        relations: {
            let value = node.required("relations")?;
            read_thinkthen_complete_relation_rule_list_v1(value, store)?
        },
        threshold: {
            let value = node.required("threshold")?;
            read_thinkthen_complete_threshold_v1(value, store).map(|value| store.hold(value))?
        },
        verb: {
            let value = node.required("verb")?;
            read_text(value, store)?
        },
        wording_version: {
            let (presence, value) = read_presence(
                node.member("wording_version"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_wording_version_v1(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_wording_version_presence_v1 { presence, value }
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
                "wording_version",
            ],
        )?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_related_entity_v1 {
    pub kind: thinkthen_complete_utf8_v1,
    pub name: thinkthen_complete_utf8_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_related_entity_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_related_entity_v1, ErrorKind> {
    Ok(thinkthen_complete_related_entity_v1 {
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
pub struct thinkthen_complete_related_entity_edge_v1 {
    pub either: thinkthen_complete_u32_presence_v1,
    pub probability: f64,
    pub relation: thinkthen_complete_utf8_v1,
    pub source: *const thinkthen_complete_relation_endpoint_v1,
    pub target: *const thinkthen_complete_relation_endpoint_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_related_entity_edge_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_related_entity_edge_v1, ErrorKind> {
    Ok(thinkthen_complete_related_entity_edge_v1 {
        either: {
            let (presence, value) =
                read_presence(node.member("either"), false, 0, read_boolean, store)?;
            thinkthen_complete_u32_presence_v1 { presence, value }
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
            read_thinkthen_complete_relation_endpoint_v1(value, store)
                .map(|value| store.hold(value))?
        },
        target: {
            let value = node.required("target")?;
            read_thinkthen_complete_relation_endpoint_v1(value, store)
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
pub struct thinkthen_complete_relation_rule_v1 {
    pub either: u32,
    pub name: thinkthen_complete_utf8_v1,
    pub reads: thinkthen_complete_utf8_v1,
    pub single: thinkthen_complete_u32_presence_v1,
    pub source: thinkthen_complete_utf8_v1,
    pub target: thinkthen_complete_utf8_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_relation_rule_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_relation_rule_v1, ErrorKind> {
    Ok(thinkthen_complete_relation_rule_v1 {
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
            thinkthen_complete_u32_presence_v1 { presence, value }
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
pub struct thinkthen_complete_session_annotation_v1 {
    pub name: thinkthen_complete_utf8_v1,
    pub value: *const thinkthen_complete_annotation_value_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_session_annotation_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_annotation_v1, ErrorKind> {
    Ok(thinkthen_complete_session_annotation_v1 {
        name: {
            let value = node.required("name")?;
            read_text(value, store)?
        },
        value: {
            let value = node.required("value")?;
            read_thinkthen_complete_annotation_value_v1(value, store)
                .map(|value| store.hold(value))?
        },
        extensions: store.extensions(node, &["name", "value"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_input_source_v1 {
    pub index: u64,
    pub source: *const thinkthen_complete_physical_source_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_session_input_source_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_input_source_v1, ErrorKind> {
    Ok(thinkthen_complete_session_input_source_v1 {
        index: {
            let value = node.required("index")?;
            read_number(value, store)?
        },
        source: {
            let value = node.required("source")?;
            read_thinkthen_complete_physical_source_v1(value, store)
                .map(|value| store.hold(value))?
        },
        extensions: store.extensions(node, &["index", "source"])?,
    })
}

pub const THINKTHEN_COMPLETE_SESSION_JUDGMENT_DECISION_V1: u32 = 1;

pub const THINKTHEN_COMPLETE_SESSION_JUDGMENT_CHOICE_V1: u32 = 2;

pub const THINKTHEN_COMPLETE_SESSION_JUDGMENT_SCORE_V1: u32 = 3;

pub const THINKTHEN_COMPLETE_SESSION_JUDGMENT_TAGS_V1: u32 = 4;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_session_judgment_data_v1 {
    pub decision: *const thinkthen_complete_session_judgment_decision_v1,
    pub choice: *const thinkthen_complete_session_judgment_choice_v1,
    pub score: *const thinkthen_complete_session_judgment_score_v1,
    pub tags: *const thinkthen_complete_session_judgment_tags_v1,
}
impl std::fmt::Debug for thinkthen_complete_session_judgment_data_v1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("union payload")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_judgment_v1 {
    pub kind: u32,
    pub data: thinkthen_complete_session_judgment_data_v1,
}

pub(crate) fn read_thinkthen_complete_session_judgment_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_judgment_v1, ErrorKind> {
    if node.is_member("kind", "decision") {
        let value = read_thinkthen_complete_session_judgment_decision_v1(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_session_judgment_v1 {
            kind: THINKTHEN_COMPLETE_SESSION_JUDGMENT_DECISION_V1,
            data: thinkthen_complete_session_judgment_data_v1 { decision: value },
        });
    }
    if node.is_member("kind", "choice") {
        let value = read_thinkthen_complete_session_judgment_choice_v1(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_session_judgment_v1 {
            kind: THINKTHEN_COMPLETE_SESSION_JUDGMENT_CHOICE_V1,
            data: thinkthen_complete_session_judgment_data_v1 { choice: value },
        });
    }
    if node.is_member("kind", "score") {
        let value = read_thinkthen_complete_session_judgment_score_v1(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_session_judgment_v1 {
            kind: THINKTHEN_COMPLETE_SESSION_JUDGMENT_SCORE_V1,
            data: thinkthen_complete_session_judgment_data_v1 { score: value },
        });
    }
    if node.is_member("kind", "tags") {
        let value = read_thinkthen_complete_session_judgment_tags_v1(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_session_judgment_v1 {
            kind: THINKTHEN_COMPLETE_SESSION_JUDGMENT_TAGS_V1,
            data: thinkthen_complete_session_judgment_data_v1 { tags: value },
        });
    }
    Err(ErrorKind::Defect)
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_judgment_choice_v1 {
    pub kind: thinkthen_complete_utf8_v1,
    pub value: thinkthen_complete_utf8_presence_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_session_judgment_choice_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_judgment_choice_v1, ErrorKind> {
    Ok(thinkthen_complete_session_judgment_choice_v1 {
        kind: {
            let value = node.required("kind")?;
            read_text(value, store)?
        },
        value: {
            let (presence, value) = read_presence(
                node.member("value"),
                true,
                thinkthen_complete_utf8_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence_v1 { presence, value }
        },
        extensions: store.extensions(node, &["kind", "value"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_judgment_decision_v1 {
    pub kind: thinkthen_complete_utf8_v1,
    pub value: thinkthen_complete_u32_presence_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_session_judgment_decision_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_judgment_decision_v1, ErrorKind> {
    Ok(thinkthen_complete_session_judgment_decision_v1 {
        kind: {
            let value = node.required("kind")?;
            read_text(value, store)?
        },
        value: {
            let (presence, value) =
                read_presence(node.member("value"), true, 0, read_boolean, store)?;
            thinkthen_complete_u32_presence_v1 { presence, value }
        },
        extensions: store.extensions(node, &["kind", "value"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_judgment_score_v1 {
    pub kind: thinkthen_complete_utf8_v1,
    pub value: f64,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_session_judgment_score_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_judgment_score_v1, ErrorKind> {
    Ok(thinkthen_complete_session_judgment_score_v1 {
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
pub struct thinkthen_complete_session_judgment_tags_v1 {
    pub kind: thinkthen_complete_utf8_v1,
    pub value: thinkthen_complete_string_list_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_session_judgment_tags_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_judgment_tags_v1, ErrorKind> {
    Ok(thinkthen_complete_session_judgment_tags_v1 {
        kind: {
            let value = node.required("kind")?;
            read_text(value, store)?
        },
        value: {
            let value = node.required("value")?;
            read_thinkthen_complete_string_list_v1(value, store)?
        },
        extensions: store.extensions(node, &["kind", "value"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_named_probability_v1 {
    pub name: thinkthen_complete_utf8_v1,
    pub probability: f64,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_session_named_probability_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_named_probability_v1, ErrorKind> {
    Ok(thinkthen_complete_session_named_probability_v1 {
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

pub const THINKTHEN_COMPLETE_SESSION_OBSERVATION_QUESTION_V1: u32 = 1;

pub const THINKTHEN_COMPLETE_SESSION_OBSERVATION_ROW_V1: u32 = 2;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_session_observation_data_v1 {
    pub question: *const thinkthen_complete_session_observation_question_v1,
    pub row: *const thinkthen_complete_session_observation_row_v1,
}
impl std::fmt::Debug for thinkthen_complete_session_observation_data_v1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("union payload")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_observation_v1 {
    pub kind: u32,
    pub data: thinkthen_complete_session_observation_data_v1,
}

pub(crate) fn read_thinkthen_complete_session_observation_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_observation_v1, ErrorKind> {
    if node.is_member("kind", "question") {
        let value = read_thinkthen_complete_session_observation_question_v1(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_session_observation_v1 {
            kind: THINKTHEN_COMPLETE_SESSION_OBSERVATION_QUESTION_V1,
            data: thinkthen_complete_session_observation_data_v1 { question: value },
        });
    }
    if node.is_member("kind", "row") {
        let value = read_thinkthen_complete_session_observation_row_v1(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_session_observation_v1 {
            kind: THINKTHEN_COMPLETE_SESSION_OBSERVATION_ROW_V1,
            data: thinkthen_complete_session_observation_data_v1 { row: value },
        });
    }
    Err(ErrorKind::Defect)
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_observation_question_v1 {
    pub detail: *const thinkthen_complete_session_question_detail_v1,
    pub index: u64,
    pub kind: thinkthen_complete_utf8_v1,
    pub member: thinkthen_complete_utf8_presence_v1,
    pub position: u64,
    pub stage: thinkthen_complete_utf8_presence_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_session_observation_question_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_observation_question_v1, ErrorKind> {
    Ok(thinkthen_complete_session_observation_question_v1 {
        detail: {
            let value = node.required("detail")?;
            read_thinkthen_complete_session_question_detail_v1(value, store)
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
                thinkthen_complete_utf8_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence_v1 { presence, value }
        },
        position: {
            let value = node.required("position")?;
            read_number(value, store)?
        },
        stage: {
            let (presence, value) = read_presence(
                node.member("stage"),
                false,
                thinkthen_complete_utf8_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence_v1 { presence, value }
        },
        extensions: store.extensions(
            node,
            &["detail", "index", "kind", "member", "position", "stage"],
        )?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_observation_row_v1 {
    pub index: u64,
    pub kind: thinkthen_complete_utf8_v1,
    pub value: *const thinkthen_complete_session_observed_row_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_session_observation_row_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_observation_row_v1, ErrorKind> {
    Ok(thinkthen_complete_session_observation_row_v1 {
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
            read_thinkthen_complete_session_observed_row_v1(value, store)
                .map(|value| store.hold(value))?
        },
        extensions: store.extensions(node, &["index", "kind", "value"])?,
    })
}

pub const THINKTHEN_COMPLETE_SESSION_OBSERVED_ROW_JUDGMENT_V1: u32 = 1;

pub const THINKTHEN_COMPLETE_SESSION_OBSERVED_ROW_ANNOTATED_V1: u32 = 2;

pub const THINKTHEN_COMPLETE_SESSION_OBSERVED_ROW_RECOGNIZED_V1: u32 = 3;

pub const THINKTHEN_COMPLETE_SESSION_OBSERVED_ROW_FIND_V1: u32 = 4;

pub const THINKTHEN_COMPLETE_SESSION_OBSERVED_ROW_RELATIONS_V1: u32 = 5;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_session_observed_row_data_v1 {
    pub judgment: *const thinkthen_complete_session_observed_row_judgment_v1,
    pub annotated: *const thinkthen_complete_session_observed_row_annotated_v1,
    pub recognized: *const thinkthen_complete_session_observed_row_recognized_v1,
    pub find: *const thinkthen_complete_session_observed_row_find_v1,
    pub relations: *const thinkthen_complete_session_observed_row_relations_v1,
}
impl std::fmt::Debug for thinkthen_complete_session_observed_row_data_v1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("union payload")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_observed_row_v1 {
    pub kind: u32,
    pub data: thinkthen_complete_session_observed_row_data_v1,
}

pub(crate) fn read_thinkthen_complete_session_observed_row_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_observed_row_v1, ErrorKind> {
    if node.is_member("kind", "judgment") {
        let value = read_thinkthen_complete_session_observed_row_judgment_v1(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_session_observed_row_v1 {
            kind: THINKTHEN_COMPLETE_SESSION_OBSERVED_ROW_JUDGMENT_V1,
            data: thinkthen_complete_session_observed_row_data_v1 { judgment: value },
        });
    }
    if node.is_member("kind", "annotated") {
        let value = read_thinkthen_complete_session_observed_row_annotated_v1(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_session_observed_row_v1 {
            kind: THINKTHEN_COMPLETE_SESSION_OBSERVED_ROW_ANNOTATED_V1,
            data: thinkthen_complete_session_observed_row_data_v1 { annotated: value },
        });
    }
    if node.is_member("kind", "recognized") {
        let value = read_thinkthen_complete_session_observed_row_recognized_v1(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_session_observed_row_v1 {
            kind: THINKTHEN_COMPLETE_SESSION_OBSERVED_ROW_RECOGNIZED_V1,
            data: thinkthen_complete_session_observed_row_data_v1 { recognized: value },
        });
    }
    if node.is_member("kind", "find") {
        let value = read_thinkthen_complete_session_observed_row_find_v1(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_session_observed_row_v1 {
            kind: THINKTHEN_COMPLETE_SESSION_OBSERVED_ROW_FIND_V1,
            data: thinkthen_complete_session_observed_row_data_v1 { find: value },
        });
    }
    if node.is_member("kind", "relations") {
        let value = read_thinkthen_complete_session_observed_row_relations_v1(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_session_observed_row_v1 {
            kind: THINKTHEN_COMPLETE_SESSION_OBSERVED_ROW_RELATIONS_V1,
            data: thinkthen_complete_session_observed_row_data_v1 { relations: value },
        });
    }
    Err(ErrorKind::Defect)
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_annotation_list_v1 {
    pub data: *const *const thinkthen_complete_session_annotation_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_session_annotation_list_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_annotation_list_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_session_annotation_v1(item, store)
                .map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_session_annotation_list_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_observed_row_annotated_v1 {
    pub kind: thinkthen_complete_utf8_v1,
    pub value: thinkthen_complete_session_annotation_list_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_session_observed_row_annotated_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_observed_row_annotated_v1, ErrorKind> {
    Ok(thinkthen_complete_session_observed_row_annotated_v1 {
        kind: {
            let value = node.required("kind")?;
            read_text(value, store)?
        },
        value: {
            let value = node.required("value")?;
            read_thinkthen_complete_session_annotation_list_v1(value, store)?
        },
        extensions: store.extensions(node, &["kind", "value"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_observed_row_find_v1 {
    pub kind: thinkthen_complete_utf8_v1,
    pub value: thinkthen_complete_u64_presence_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_session_observed_row_find_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_observed_row_find_v1, ErrorKind> {
    Ok(thinkthen_complete_session_observed_row_find_v1 {
        kind: {
            let value = node.required("kind")?;
            read_text(value, store)?
        },
        value: {
            let (presence, value) =
                read_presence(node.member("value"), true, 0, read_number, store)?;
            thinkthen_complete_u64_presence_v1 { presence, value }
        },
        extensions: store.extensions(node, &["kind", "value"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_observed_row_judgment_v1 {
    pub kind: thinkthen_complete_utf8_v1,
    pub value: *const thinkthen_complete_session_judgment_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_session_observed_row_judgment_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_observed_row_judgment_v1, ErrorKind> {
    Ok(thinkthen_complete_session_observed_row_judgment_v1 {
        kind: {
            let value = node.required("kind")?;
            read_text(value, store)?
        },
        value: {
            let value = node.required("value")?;
            read_thinkthen_complete_session_judgment_v1(value, store)
                .map(|value| store.hold(value))?
        },
        extensions: store.extensions(node, &["kind", "value"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_observed_row_recognized_v1 {
    pub kind: thinkthen_complete_utf8_v1,
    pub value: *const thinkthen_complete_session_recognition_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_session_observed_row_recognized_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_observed_row_recognized_v1, ErrorKind> {
    Ok(thinkthen_complete_session_observed_row_recognized_v1 {
        kind: {
            let value = node.required("kind")?;
            read_text(value, store)?
        },
        value: {
            let value = node.required("value")?;
            read_thinkthen_complete_session_recognition_v1(value, store)
                .map(|value| store.hold(value))?
        },
        extensions: store.extensions(node, &["kind", "value"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_relation_edge_list_v1 {
    pub data: *const *const thinkthen_complete_session_relation_edge_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_session_relation_edge_list_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_relation_edge_list_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_session_relation_edge_v1(item, store)
                .map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_session_relation_edge_list_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_observed_row_relations_v1 {
    pub kind: thinkthen_complete_utf8_v1,
    pub value: thinkthen_complete_session_relation_edge_list_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_session_observed_row_relations_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_observed_row_relations_v1, ErrorKind> {
    Ok(thinkthen_complete_session_observed_row_relations_v1 {
        kind: {
            let value = node.required("kind")?;
            read_text(value, store)?
        },
        value: {
            let value = node.required("value")?;
            read_thinkthen_complete_session_relation_edge_list_v1(value, store)?
        },
        extensions: store.extensions(node, &["kind", "value"])?,
    })
}

pub const THINKTHEN_COMPLETE_SESSION_PACKET_DECIDE_ROW_V1: u32 = 1;

pub const THINKTHEN_COMPLETE_SESSION_PACKET_CHOOSE_ROW_V1: u32 = 2;

pub const THINKTHEN_COMPLETE_SESSION_PACKET_TAG_ROW_V1: u32 = 3;

pub const THINKTHEN_COMPLETE_SESSION_PACKET_SCORE_ROW_V1: u32 = 4;

pub const THINKTHEN_COMPLETE_SESSION_PACKET_FILTER_ROW_V1: u32 = 5;

pub const THINKTHEN_COMPLETE_SESSION_PACKET_ANNOTATE_ROW_V1: u32 = 6;

pub const THINKTHEN_COMPLETE_SESSION_PACKET_DECIDE_AGGREGATE_V1: u32 = 7;

pub const THINKTHEN_COMPLETE_SESSION_PACKET_CHOOSE_AGGREGATE_V1: u32 = 8;

pub const THINKTHEN_COMPLETE_SESSION_PACKET_TAG_AGGREGATE_V1: u32 = 9;

pub const THINKTHEN_COMPLETE_SESSION_PACKET_SCORE_AGGREGATE_V1: u32 = 10;

pub const THINKTHEN_COMPLETE_SESSION_PACKET_FILTER_AGGREGATE_V1: u32 = 11;

pub const THINKTHEN_COMPLETE_SESSION_PACKET_RANK_AGGREGATE_V1: u32 = 12;

pub const THINKTHEN_COMPLETE_SESSION_PACKET_FIND_AGGREGATE_V1: u32 = 13;

pub const THINKTHEN_COMPLETE_SESSION_PACKET_ANNOTATE_AGGREGATE_V1: u32 = 14;

pub const THINKTHEN_COMPLETE_SESSION_PACKET_RECOGNIZE_AGGREGATE_V1: u32 = 15;

pub const THINKTHEN_COMPLETE_SESSION_PACKET_RELATE_AGGREGATE_V1: u32 = 16;

pub const THINKTHEN_COMPLETE_SESSION_PACKET_OBSERVATION_V1: u32 = 17;

pub const THINKTHEN_COMPLETE_SESSION_PACKET_TERMINAL_V1: u32 = 18;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_session_packet_data_v1 {
    pub decide_row: *const thinkthen_complete_session_packet_decide_row_v1,
    pub choose_row: *const thinkthen_complete_session_packet_choose_row_v1,
    pub tag_row: *const thinkthen_complete_session_packet_tag_row_v1,
    pub score_row: *const thinkthen_complete_session_packet_score_row_v1,
    pub filter_row: *const thinkthen_complete_session_packet_filter_row_v1,
    pub annotate_row: *const thinkthen_complete_session_packet_annotate_row_v1,
    pub decide_aggregate: *const thinkthen_complete_session_packet_decide_aggregate_v1,
    pub choose_aggregate: *const thinkthen_complete_session_packet_choose_aggregate_v1,
    pub tag_aggregate: *const thinkthen_complete_session_packet_tag_aggregate_v1,
    pub score_aggregate: *const thinkthen_complete_session_packet_score_aggregate_v1,
    pub filter_aggregate: *const thinkthen_complete_session_packet_filter_aggregate_v1,
    pub rank_aggregate: *const thinkthen_complete_session_packet_rank_aggregate_v1,
    pub find_aggregate: *const thinkthen_complete_session_packet_find_aggregate_v1,
    pub annotate_aggregate: *const thinkthen_complete_session_packet_annotate_aggregate_v1,
    pub recognize_aggregate: *const thinkthen_complete_session_packet_recognize_aggregate_v1,
    pub relate_aggregate: *const thinkthen_complete_session_packet_relate_aggregate_v1,
    pub observation: *const thinkthen_complete_session_packet_observation_v1,
    pub terminal: *const thinkthen_complete_session_packet_terminal_v1,
}
impl std::fmt::Debug for thinkthen_complete_session_packet_data_v1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("union payload")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_packet_v1 {
    pub kind: u32,
    pub data: thinkthen_complete_session_packet_data_v1,
}

pub(crate) fn read_thinkthen_complete_session_packet_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_packet_v1, ErrorKind> {
    if node.is_member("function", "decide") && node.is_member("kind", "row") {
        let value = read_thinkthen_complete_session_packet_decide_row_v1(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_session_packet_v1 {
            kind: THINKTHEN_COMPLETE_SESSION_PACKET_DECIDE_ROW_V1,
            data: thinkthen_complete_session_packet_data_v1 { decide_row: value },
        });
    }
    if node.is_member("function", "choose") && node.is_member("kind", "row") {
        let value = read_thinkthen_complete_session_packet_choose_row_v1(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_session_packet_v1 {
            kind: THINKTHEN_COMPLETE_SESSION_PACKET_CHOOSE_ROW_V1,
            data: thinkthen_complete_session_packet_data_v1 { choose_row: value },
        });
    }
    if node.is_member("function", "tag") && node.is_member("kind", "row") {
        let value = read_thinkthen_complete_session_packet_tag_row_v1(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_session_packet_v1 {
            kind: THINKTHEN_COMPLETE_SESSION_PACKET_TAG_ROW_V1,
            data: thinkthen_complete_session_packet_data_v1 { tag_row: value },
        });
    }
    if node.is_member("function", "score") && node.is_member("kind", "row") {
        let value = read_thinkthen_complete_session_packet_score_row_v1(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_session_packet_v1 {
            kind: THINKTHEN_COMPLETE_SESSION_PACKET_SCORE_ROW_V1,
            data: thinkthen_complete_session_packet_data_v1 { score_row: value },
        });
    }
    if node.is_member("function", "filter") && node.is_member("kind", "row") {
        let value = read_thinkthen_complete_session_packet_filter_row_v1(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_session_packet_v1 {
            kind: THINKTHEN_COMPLETE_SESSION_PACKET_FILTER_ROW_V1,
            data: thinkthen_complete_session_packet_data_v1 { filter_row: value },
        });
    }
    if node.is_member("function", "annotate") && node.is_member("kind", "row") {
        let value = read_thinkthen_complete_session_packet_annotate_row_v1(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_session_packet_v1 {
            kind: THINKTHEN_COMPLETE_SESSION_PACKET_ANNOTATE_ROW_V1,
            data: thinkthen_complete_session_packet_data_v1 {
                annotate_row: value,
            },
        });
    }
    if node.is_member("function", "decide") && node.is_member("kind", "aggregate") {
        let value = read_thinkthen_complete_session_packet_decide_aggregate_v1(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_session_packet_v1 {
            kind: THINKTHEN_COMPLETE_SESSION_PACKET_DECIDE_AGGREGATE_V1,
            data: thinkthen_complete_session_packet_data_v1 {
                decide_aggregate: value,
            },
        });
    }
    if node.is_member("function", "choose") && node.is_member("kind", "aggregate") {
        let value = read_thinkthen_complete_session_packet_choose_aggregate_v1(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_session_packet_v1 {
            kind: THINKTHEN_COMPLETE_SESSION_PACKET_CHOOSE_AGGREGATE_V1,
            data: thinkthen_complete_session_packet_data_v1 {
                choose_aggregate: value,
            },
        });
    }
    if node.is_member("function", "tag") && node.is_member("kind", "aggregate") {
        let value = read_thinkthen_complete_session_packet_tag_aggregate_v1(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_session_packet_v1 {
            kind: THINKTHEN_COMPLETE_SESSION_PACKET_TAG_AGGREGATE_V1,
            data: thinkthen_complete_session_packet_data_v1 {
                tag_aggregate: value,
            },
        });
    }
    if node.is_member("function", "score") && node.is_member("kind", "aggregate") {
        let value = read_thinkthen_complete_session_packet_score_aggregate_v1(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_session_packet_v1 {
            kind: THINKTHEN_COMPLETE_SESSION_PACKET_SCORE_AGGREGATE_V1,
            data: thinkthen_complete_session_packet_data_v1 {
                score_aggregate: value,
            },
        });
    }
    if node.is_member("function", "filter") && node.is_member("kind", "aggregate") {
        let value = read_thinkthen_complete_session_packet_filter_aggregate_v1(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_session_packet_v1 {
            kind: THINKTHEN_COMPLETE_SESSION_PACKET_FILTER_AGGREGATE_V1,
            data: thinkthen_complete_session_packet_data_v1 {
                filter_aggregate: value,
            },
        });
    }
    if node.is_member("function", "rank") && node.is_member("kind", "aggregate") {
        let value = read_thinkthen_complete_session_packet_rank_aggregate_v1(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_session_packet_v1 {
            kind: THINKTHEN_COMPLETE_SESSION_PACKET_RANK_AGGREGATE_V1,
            data: thinkthen_complete_session_packet_data_v1 {
                rank_aggregate: value,
            },
        });
    }
    if node.is_member("function", "find") && node.is_member("kind", "aggregate") {
        let value = read_thinkthen_complete_session_packet_find_aggregate_v1(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_session_packet_v1 {
            kind: THINKTHEN_COMPLETE_SESSION_PACKET_FIND_AGGREGATE_V1,
            data: thinkthen_complete_session_packet_data_v1 {
                find_aggregate: value,
            },
        });
    }
    if node.is_member("function", "annotate") && node.is_member("kind", "aggregate") {
        let value = read_thinkthen_complete_session_packet_annotate_aggregate_v1(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_session_packet_v1 {
            kind: THINKTHEN_COMPLETE_SESSION_PACKET_ANNOTATE_AGGREGATE_V1,
            data: thinkthen_complete_session_packet_data_v1 {
                annotate_aggregate: value,
            },
        });
    }
    if node.is_member("function", "recognize") && node.is_member("kind", "aggregate") {
        let value = read_thinkthen_complete_session_packet_recognize_aggregate_v1(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_session_packet_v1 {
            kind: THINKTHEN_COMPLETE_SESSION_PACKET_RECOGNIZE_AGGREGATE_V1,
            data: thinkthen_complete_session_packet_data_v1 {
                recognize_aggregate: value,
            },
        });
    }
    if node.is_member("function", "relate") && node.is_member("kind", "aggregate") {
        let value = read_thinkthen_complete_session_packet_relate_aggregate_v1(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_session_packet_v1 {
            kind: THINKTHEN_COMPLETE_SESSION_PACKET_RELATE_AGGREGATE_V1,
            data: thinkthen_complete_session_packet_data_v1 {
                relate_aggregate: value,
            },
        });
    }
    if node.is_member("kind", "observation") {
        let value = read_thinkthen_complete_session_packet_observation_v1(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_session_packet_v1 {
            kind: THINKTHEN_COMPLETE_SESSION_PACKET_OBSERVATION_V1,
            data: thinkthen_complete_session_packet_data_v1 { observation: value },
        });
    }
    if node.is_member("kind", "terminal") {
        let value = read_thinkthen_complete_session_packet_terminal_v1(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_session_packet_v1 {
            kind: THINKTHEN_COMPLETE_SESSION_PACKET_TERMINAL_V1,
            data: thinkthen_complete_session_packet_data_v1 { terminal: value },
        });
    }
    Err(ErrorKind::Defect)
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_annotation_list_v1 {
    pub data: *const *const thinkthen_complete_annotation_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_annotation_list_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_annotation_list_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_annotation_v1(item, store).map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_annotation_list_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_packet_annotate_aggregate_v1 {
    pub function: thinkthen_complete_utf8_v1,
    pub kind: thinkthen_complete_utf8_v1,
    pub value: thinkthen_complete_annotation_list_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_session_packet_annotate_aggregate_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_packet_annotate_aggregate_v1, ErrorKind> {
    Ok(thinkthen_complete_session_packet_annotate_aggregate_v1 {
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
            read_thinkthen_complete_annotation_list_v1(value, store)?
        },
        extensions: store.extensions(node, &["function", "kind", "value"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_packet_annotate_row_v1 {
    pub function: thinkthen_complete_utf8_v1,
    pub kind: thinkthen_complete_utf8_v1,
    pub value: *const thinkthen_complete_annotation_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_session_packet_annotate_row_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_packet_annotate_row_v1, ErrorKind> {
    Ok(thinkthen_complete_session_packet_annotate_row_v1 {
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
            read_thinkthen_complete_annotation_v1(value, store).map(|value| store.hold(value))?
        },
        extensions: store.extensions(node, &["function", "kind", "value"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_atomic_nullable_string_list_v1 {
    pub data: *const *const thinkthen_complete_atomic_nullable_string_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_atomic_nullable_string_list_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_atomic_nullable_string_list_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_atomic_nullable_string_v1(item, store)
                .map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_atomic_nullable_string_list_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_packet_choose_aggregate_v1 {
    pub function: thinkthen_complete_utf8_v1,
    pub kind: thinkthen_complete_utf8_v1,
    pub value: thinkthen_complete_atomic_nullable_string_list_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_session_packet_choose_aggregate_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_packet_choose_aggregate_v1, ErrorKind> {
    Ok(thinkthen_complete_session_packet_choose_aggregate_v1 {
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
            read_thinkthen_complete_atomic_nullable_string_list_v1(value, store)?
        },
        extensions: store.extensions(node, &["function", "kind", "value"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_packet_choose_row_v1 {
    pub function: thinkthen_complete_utf8_v1,
    pub kind: thinkthen_complete_utf8_v1,
    pub value: *const thinkthen_complete_atomic_nullable_string_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_session_packet_choose_row_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_packet_choose_row_v1, ErrorKind> {
    Ok(thinkthen_complete_session_packet_choose_row_v1 {
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
            read_thinkthen_complete_atomic_nullable_string_v1(value, store)
                .map(|value| store.hold(value))?
        },
        extensions: store.extensions(node, &["function", "kind", "value"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_atomic_decide_value_list_v1 {
    pub data: *const *const thinkthen_complete_atomic_decide_value_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_atomic_decide_value_list_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_atomic_decide_value_list_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_atomic_decide_value_v1(item, store)
                .map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_atomic_decide_value_list_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_packet_decide_aggregate_v1 {
    pub function: thinkthen_complete_utf8_v1,
    pub kind: thinkthen_complete_utf8_v1,
    pub value: thinkthen_complete_atomic_decide_value_list_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_session_packet_decide_aggregate_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_packet_decide_aggregate_v1, ErrorKind> {
    Ok(thinkthen_complete_session_packet_decide_aggregate_v1 {
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
            read_thinkthen_complete_atomic_decide_value_list_v1(value, store)?
        },
        extensions: store.extensions(node, &["function", "kind", "value"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_packet_decide_row_v1 {
    pub function: thinkthen_complete_utf8_v1,
    pub kind: thinkthen_complete_utf8_v1,
    pub value: *const thinkthen_complete_atomic_decide_value_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_session_packet_decide_row_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_packet_decide_row_v1, ErrorKind> {
    Ok(thinkthen_complete_session_packet_decide_row_v1 {
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
            read_thinkthen_complete_atomic_decide_value_v1(value, store)
                .map(|value| store.hold(value))?
        },
        extensions: store.extensions(node, &["function", "kind", "value"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_atomic_boolean_list_v1 {
    pub data: *const *const thinkthen_complete_atomic_boolean_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_atomic_boolean_list_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_atomic_boolean_list_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_atomic_boolean_v1(item, store)
                .map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_atomic_boolean_list_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_packet_filter_aggregate_v1 {
    pub function: thinkthen_complete_utf8_v1,
    pub kind: thinkthen_complete_utf8_v1,
    pub value: thinkthen_complete_atomic_boolean_list_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_session_packet_filter_aggregate_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_packet_filter_aggregate_v1, ErrorKind> {
    Ok(thinkthen_complete_session_packet_filter_aggregate_v1 {
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
            read_thinkthen_complete_atomic_boolean_list_v1(value, store)?
        },
        extensions: store.extensions(node, &["function", "kind", "value"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_packet_filter_row_v1 {
    pub function: thinkthen_complete_utf8_v1,
    pub kind: thinkthen_complete_utf8_v1,
    pub value: *const thinkthen_complete_atomic_boolean_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_session_packet_filter_row_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_packet_filter_row_v1, ErrorKind> {
    Ok(thinkthen_complete_session_packet_filter_row_v1 {
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
            read_thinkthen_complete_atomic_boolean_v1(value, store)
                .map(|value| store.hold(value))?
        },
        extensions: store.extensions(node, &["function", "kind", "value"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_packet_find_aggregate_v1 {
    pub function: thinkthen_complete_utf8_v1,
    pub kind: thinkthen_complete_utf8_v1,
    pub value: *const thinkthen_complete_find_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_session_packet_find_aggregate_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_packet_find_aggregate_v1, ErrorKind> {
    Ok(thinkthen_complete_session_packet_find_aggregate_v1 {
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
            read_thinkthen_complete_find_v1(value, store).map(|value| store.hold(value))?
        },
        extensions: store.extensions(node, &["function", "kind", "value"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_packet_observation_v1 {
    pub function: *const thinkthen_complete_request_function_v1,
    pub kind: thinkthen_complete_utf8_v1,
    pub value: *const thinkthen_complete_session_observation_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_session_packet_observation_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_packet_observation_v1, ErrorKind> {
    Ok(thinkthen_complete_session_packet_observation_v1 {
        function: {
            let value = node.required("function")?;
            read_thinkthen_complete_request_function_v1(value, store)
                .map(|value| store.hold(value))?
        },
        kind: {
            let value = node.required("kind")?;
            read_text(value, store)?
        },
        value: {
            let value = node.required("value")?;
            read_thinkthen_complete_session_observation_v1(value, store)
                .map(|value| store.hold(value))?
        },
        extensions: store.extensions(node, &["function", "kind", "value"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_atomic_non_zero_usize_list_v1 {
    pub data: *const *const thinkthen_complete_atomic_non_zero_usize_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_atomic_non_zero_usize_list_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_atomic_non_zero_usize_list_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_atomic_non_zero_usize_v1(item, store)
                .map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_atomic_non_zero_usize_list_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_packet_rank_aggregate_v1 {
    pub function: thinkthen_complete_utf8_v1,
    pub kind: thinkthen_complete_utf8_v1,
    pub value: thinkthen_complete_atomic_non_zero_usize_list_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_session_packet_rank_aggregate_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_packet_rank_aggregate_v1, ErrorKind> {
    Ok(thinkthen_complete_session_packet_rank_aggregate_v1 {
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
            read_thinkthen_complete_atomic_non_zero_usize_list_v1(value, store)?
        },
        extensions: store.extensions(node, &["function", "kind", "value"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_recognition_list_v1 {
    pub data: *const *const thinkthen_complete_recognition_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_recognition_list_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_recognition_list_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_recognition_v1(item, store).map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_recognition_list_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_packet_recognize_aggregate_v1 {
    pub function: thinkthen_complete_utf8_v1,
    pub kind: thinkthen_complete_utf8_v1,
    pub value: thinkthen_complete_recognition_list_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_session_packet_recognize_aggregate_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_packet_recognize_aggregate_v1, ErrorKind> {
    Ok(thinkthen_complete_session_packet_recognize_aggregate_v1 {
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
            read_thinkthen_complete_recognition_list_v1(value, store)?
        },
        extensions: store.extensions(node, &["function", "kind", "value"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_packet_relate_aggregate_v1 {
    pub function: thinkthen_complete_utf8_v1,
    pub kind: thinkthen_complete_utf8_v1,
    pub value: *const thinkthen_complete_relation_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_session_packet_relate_aggregate_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_packet_relate_aggregate_v1, ErrorKind> {
    Ok(thinkthen_complete_session_packet_relate_aggregate_v1 {
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
            read_thinkthen_complete_relation_v1(value, store).map(|value| store.hold(value))?
        },
        extensions: store.extensions(node, &["function", "kind", "value"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_atomic_double_list_v1 {
    pub data: *const *const thinkthen_complete_atomic_double_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_atomic_double_list_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_atomic_double_list_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_atomic_double_v1(item, store).map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_atomic_double_list_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_packet_score_aggregate_v1 {
    pub function: thinkthen_complete_utf8_v1,
    pub kind: thinkthen_complete_utf8_v1,
    pub value: thinkthen_complete_atomic_double_list_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_session_packet_score_aggregate_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_packet_score_aggregate_v1, ErrorKind> {
    Ok(thinkthen_complete_session_packet_score_aggregate_v1 {
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
            read_thinkthen_complete_atomic_double_list_v1(value, store)?
        },
        extensions: store.extensions(node, &["function", "kind", "value"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_packet_score_row_v1 {
    pub function: thinkthen_complete_utf8_v1,
    pub kind: thinkthen_complete_utf8_v1,
    pub value: *const thinkthen_complete_atomic_double_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_session_packet_score_row_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_packet_score_row_v1, ErrorKind> {
    Ok(thinkthen_complete_session_packet_score_row_v1 {
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
            read_thinkthen_complete_atomic_double_v1(value, store).map(|value| store.hold(value))?
        },
        extensions: store.extensions(node, &["function", "kind", "value"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_atomic_array_of_string_list_v1 {
    pub data: *const *const thinkthen_complete_atomic_array_of_string_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_atomic_array_of_string_list_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_atomic_array_of_string_list_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_atomic_array_of_string_v1(item, store)
                .map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_atomic_array_of_string_list_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_packet_tag_aggregate_v1 {
    pub function: thinkthen_complete_utf8_v1,
    pub kind: thinkthen_complete_utf8_v1,
    pub value: thinkthen_complete_atomic_array_of_string_list_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_session_packet_tag_aggregate_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_packet_tag_aggregate_v1, ErrorKind> {
    Ok(thinkthen_complete_session_packet_tag_aggregate_v1 {
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
            read_thinkthen_complete_atomic_array_of_string_list_v1(value, store)?
        },
        extensions: store.extensions(node, &["function", "kind", "value"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_packet_tag_row_v1 {
    pub function: thinkthen_complete_utf8_v1,
    pub kind: thinkthen_complete_utf8_v1,
    pub value: *const thinkthen_complete_atomic_array_of_string_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_session_packet_tag_row_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_packet_tag_row_v1, ErrorKind> {
    Ok(thinkthen_complete_session_packet_tag_row_v1 {
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
            read_thinkthen_complete_atomic_array_of_string_v1(value, store)
                .map(|value| store.hold(value))?
        },
        extensions: store.extensions(node, &["function", "kind", "value"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_call_error_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_call_error_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_packet_terminal_v1 {
    pub facts: thinkthen_complete_facts_presence_v1,
    pub failure: thinkthen_complete_call_error_presence_v1,
    pub kind: thinkthen_complete_utf8_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_session_packet_terminal_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_packet_terminal_v1, ErrorKind> {
    Ok(thinkthen_complete_session_packet_terminal_v1 {
        facts: {
            let (presence, value) = read_presence(
                node.member("facts"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_facts_v1(value, store).map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_facts_presence_v1 { presence, value }
        },
        failure: {
            let (presence, value) = read_presence(
                node.member("failure"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_call_error_v1(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_call_error_presence_v1 { presence, value }
        },
        kind: {
            let value = node.required("kind")?;
            read_text(value, store)?
        },
        extensions: store.extensions(node, &["facts", "failure", "kind"])?,
    })
}

pub const THINKTHEN_COMPLETE_SESSION_PROBABILITIES_YES_NO_V1: u32 = 1;

pub const THINKTHEN_COMPLETE_SESSION_PROBABILITIES_NAMED_V1: u32 = 2;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_session_probabilities_data_v1 {
    pub yes_no: *const thinkthen_complete_session_probabilities_yes_no_v1,
    pub named: *const thinkthen_complete_session_probabilities_named_v1,
}
impl std::fmt::Debug for thinkthen_complete_session_probabilities_data_v1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("union payload")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_probabilities_v1 {
    pub kind: u32,
    pub data: thinkthen_complete_session_probabilities_data_v1,
}

pub(crate) fn read_thinkthen_complete_session_probabilities_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_probabilities_v1, ErrorKind> {
    if node.is_member("kind", "yes_no") {
        let value = read_thinkthen_complete_session_probabilities_yes_no_v1(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_session_probabilities_v1 {
            kind: THINKTHEN_COMPLETE_SESSION_PROBABILITIES_YES_NO_V1,
            data: thinkthen_complete_session_probabilities_data_v1 { yes_no: value },
        });
    }
    if node.is_member("kind", "named") {
        let value = read_thinkthen_complete_session_probabilities_named_v1(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_session_probabilities_v1 {
            kind: THINKTHEN_COMPLETE_SESSION_PROBABILITIES_NAMED_V1,
            data: thinkthen_complete_session_probabilities_data_v1 { named: value },
        });
    }
    Err(ErrorKind::Defect)
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_named_probability_list_v1 {
    pub data: *const *const thinkthen_complete_session_named_probability_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_session_named_probability_list_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_named_probability_list_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_session_named_probability_v1(item, store)
                .map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_session_named_probability_list_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_probabilities_named_v1 {
    pub kind: thinkthen_complete_utf8_v1,
    pub value: thinkthen_complete_session_named_probability_list_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_session_probabilities_named_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_probabilities_named_v1, ErrorKind> {
    Ok(thinkthen_complete_session_probabilities_named_v1 {
        kind: {
            let value = node.required("kind")?;
            read_text(value, store)?
        },
        value: {
            let value = node.required("value")?;
            read_thinkthen_complete_session_named_probability_list_v1(value, store)?
        },
        extensions: store.extensions(node, &["kind", "value"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_probabilities_yes_no_v1 {
    pub kind: thinkthen_complete_utf8_v1,
    pub value: f64,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_session_probabilities_yes_no_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_probabilities_yes_no_v1, ErrorKind> {
    Ok(thinkthen_complete_session_probabilities_yes_no_v1 {
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
pub struct thinkthen_complete_answer_id_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_answer_id_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_failure_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_failure_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_failure_id_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_failure_id_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_json_list_v1 {
    pub data: *const *const thinkthen_complete_json_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_json_list_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_json_list_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(read_json(item, store)?);
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_json_list_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_probabilities_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_session_probabilities_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_token_usage_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_token_usage_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_question_detail_v1 {
    pub answer_id: thinkthen_complete_answer_id_presence_v1,
    pub cached: u32,
    pub confidence: thinkthen_complete_f64_presence_v1,
    pub failed_questions: u64,
    pub failure: thinkthen_complete_failure_presence_v1,
    pub failure_id: thinkthen_complete_failure_id_presence_v1,
    pub input: thinkthen_complete_json_presence_v1,
    pub input_source: thinkthen_complete_physical_source_presence_v1,
    pub input_sources: thinkthen_complete_session_input_source_list_v1,
    pub inputs: thinkthen_complete_json_list_v1,
    pub model: thinkthen_complete_utf8_v1,
    pub observations: thinkthen_complete_observation_list_v1,
    pub probabilities: thinkthen_complete_session_probabilities_presence_v1,
    pub question: *const thinkthen_complete_question_document_v1,
    pub question_sha256: thinkthen_complete_utf8_v1,
    pub question_sources: thinkthen_complete_question_source_list_v1,
    pub raw_pick: thinkthen_complete_utf8_presence_v1,
    pub reported_usage: thinkthen_complete_usage_presence_v1,
    pub requests: thinkthen_complete_string_list_v1,
    pub requests_sent: u64,
    pub threshold: thinkthen_complete_threshold_presence_v1,
    pub url: thinkthen_complete_utf8_v1,
    pub usage: thinkthen_complete_token_usage_presence_v1,
    pub value: thinkthen_complete_value_presence_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_session_question_detail_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_question_detail_v1, ErrorKind> {
    Ok(thinkthen_complete_session_question_detail_v1 {
        answer_id: {
            let (presence, value) = read_presence(
                node.member("answer_id"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_answer_id_v1(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_answer_id_presence_v1 { presence, value }
        },
        cached: {
            let value = node.required("cached")?;
            read_boolean(value, store)?
        },
        confidence: {
            let (presence, value) =
                read_presence(node.member("confidence"), false, 0.0, read_number, store)?;
            thinkthen_complete_f64_presence_v1 { presence, value }
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
                    read_thinkthen_complete_failure_v1(value, store).map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_failure_presence_v1 { presence, value }
        },
        failure_id: {
            let (presence, value) = read_presence(
                node.member("failure_id"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_failure_id_v1(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_failure_id_presence_v1 { presence, value }
        },
        input: {
            let (presence, value) = read_presence(
                node.member("input"),
                false,
                std::ptr::null(),
                read_json,
                store,
            )?;
            thinkthen_complete_json_presence_v1 { presence, value }
        },
        input_source: {
            let (presence, value) = read_presence(
                node.member("input_source"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_physical_source_v1(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_physical_source_presence_v1 { presence, value }
        },
        input_sources: {
            let value = node.required("input_sources")?;
            read_thinkthen_complete_session_input_source_list_v1(value, store)?
        },
        inputs: {
            let value = node.required("inputs")?;
            read_thinkthen_complete_json_list_v1(value, store)?
        },
        model: {
            let value = node.required("model")?;
            read_text(value, store)?
        },
        observations: {
            let value = node.required("observations")?;
            read_thinkthen_complete_observation_list_v1(value, store)?
        },
        probabilities: {
            let (presence, value) = read_presence(
                node.member("probabilities"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_session_probabilities_v1(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_session_probabilities_presence_v1 { presence, value }
        },
        question: {
            let value = node.required("question")?;
            read_thinkthen_complete_question_document_v1(value, store)
                .map(|value| store.hold(value))?
        },
        question_sha256: {
            let value = node.required("question_sha256")?;
            read_text(value, store)?
        },
        question_sources: {
            let value = node.required("question_sources")?;
            read_thinkthen_complete_question_source_list_v1(value, store)?
        },
        raw_pick: {
            let (presence, value) = read_presence(
                node.member("raw_pick"),
                false,
                thinkthen_complete_utf8_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_text,
                store,
            )?;
            thinkthen_complete_utf8_presence_v1 { presence, value }
        },
        reported_usage: {
            let (presence, value) = read_presence(
                node.member("reported_usage"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_usage_v1(value, store).map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_usage_presence_v1 { presence, value }
        },
        requests: {
            let value = node.required("requests")?;
            read_thinkthen_complete_string_list_v1(value, store)?
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
                    read_thinkthen_complete_threshold_v1(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_threshold_presence_v1 { presence, value }
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
                    read_thinkthen_complete_token_usage_v1(value, store)
                        .map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_token_usage_presence_v1 { presence, value }
        },
        value: {
            let (presence, value) = read_presence(
                node.member("value"),
                false,
                std::ptr::null(),
                |value, store| {
                    read_thinkthen_complete_value_v1(value, store).map(|value| store.hold(value))
                },
                store,
            )?;
            thinkthen_complete_value_presence_v1 { presence, value }
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
pub struct thinkthen_complete_boundary_proposal_list_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_boundary_proposal_list_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_recognition_edge_list_v1 {
    pub data: *const *const thinkthen_complete_recognition_edge_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_recognition_edge_list_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_recognition_edge_list_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_recognition_edge_v1(item, store)
                .map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_recognition_edge_list_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_recognition_edge_list_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_recognition_edge_list_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_recognition_v1 {
    pub entities: thinkthen_complete_entity_list_v1,
    pub mode: *const thinkthen_complete_recognition_mode_v1,
    pub proposals: thinkthen_complete_boundary_proposal_list_presence_v1,
    pub relations: thinkthen_complete_recognition_edge_list_presence_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_session_recognition_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_recognition_v1, ErrorKind> {
    Ok(thinkthen_complete_session_recognition_v1 {
        entities: {
            let value = node.required("entities")?;
            read_thinkthen_complete_entity_list_v1(value, store)?
        },
        mode: {
            let value = node.required("mode")?;
            read_thinkthen_complete_recognition_mode_v1(value, store)
                .map(|value| store.hold(value))?
        },
        proposals: {
            let (presence, value) = read_presence(
                node.member("proposals"),
                false,
                thinkthen_complete_boundary_proposal_list_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_boundary_proposal_list_v1,
                store,
            )?;
            thinkthen_complete_boundary_proposal_list_presence_v1 { presence, value }
        },
        relations: {
            let (presence, value) = read_presence(
                node.member("relations"),
                false,
                thinkthen_complete_recognition_edge_list_v1 {
                    data: std::ptr::null(),
                    len: 0,
                },
                read_thinkthen_complete_recognition_edge_list_v1,
                store,
            )?;
            thinkthen_complete_recognition_edge_list_presence_v1 { presence, value }
        },
        extensions: store.extensions(node, &["entities", "mode", "proposals", "relations"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_relation_edge_v1 {
    pub either: u32,
    pub probability: f64,
    pub relation: thinkthen_complete_utf8_v1,
    pub source: *const thinkthen_complete_entity_document_v1,
    pub target: *const thinkthen_complete_entity_document_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_session_relation_edge_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_relation_edge_v1, ErrorKind> {
    Ok(thinkthen_complete_session_relation_edge_v1 {
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
            read_thinkthen_complete_entity_document_v1(value, store)
                .map(|value| store.hold(value))?
        },
        target: {
            let value = node.required("target")?;
            read_thinkthen_complete_entity_document_v1(value, store)
                .map(|value| store.hold(value))?
        },
        extensions: store.extensions(
            node,
            &["either", "probability", "relation", "source", "target"],
        )?,
    })
}

pub const THINKTHEN_COMPLETE_THRESHOLD_NUMBER_V1: u32 = 1;

pub const THINKTHEN_COMPLETE_THRESHOLD_STRING_V1: u32 = 2;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_threshold_data_v1 {
    pub number: f64,
    pub string: thinkthen_complete_utf8_v1,
}
impl std::fmt::Debug for thinkthen_complete_threshold_data_v1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("union payload")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_threshold_v1 {
    pub kind: u32,
    pub data: thinkthen_complete_threshold_data_v1,
}

pub(crate) fn read_thinkthen_complete_threshold_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_threshold_v1, ErrorKind> {
    if node.kind() == "number" {
        let value = read_number(node, store)?;
        return Ok(thinkthen_complete_threshold_v1 {
            kind: THINKTHEN_COMPLETE_THRESHOLD_NUMBER_V1,
            data: thinkthen_complete_threshold_data_v1 { number: value },
        });
    }
    if node.kind() == "string" {
        let value = read_text(node, store)?;
        return Ok(thinkthen_complete_threshold_v1 {
            kind: THINKTHEN_COMPLETE_THRESHOLD_STRING_V1,
            data: thinkthen_complete_threshold_data_v1 { string: value },
        });
    }
    Err(ErrorKind::Defect)
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_token_usage_v1 {
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_token_usage_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_token_usage_v1, ErrorKind> {
    Ok(thinkthen_complete_token_usage_v1 {
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

pub const THINKTHEN_COMPLETE_VALUE_BOOLEAN_V1: u32 = 1;

pub const THINKTHEN_COMPLETE_VALUE_NULL_V1: u32 = 2;

pub const THINKTHEN_COMPLETE_VALUE_STRING_V1: u32 = 3;

pub const THINKTHEN_COMPLETE_VALUE_ARRAY_V1: u32 = 4;

pub const THINKTHEN_COMPLETE_VALUE_NUMBER_V1: u32 = 5;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_value_data_v1 {
    pub boolean: u32,
    pub null: u32,
    pub string: thinkthen_complete_utf8_v1,
    pub array: thinkthen_complete_string_list_v1,
    pub number: f64,
}
impl std::fmt::Debug for thinkthen_complete_value_data_v1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("union payload")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_value_v1 {
    pub kind: u32,
    pub data: thinkthen_complete_value_data_v1,
}

pub(crate) fn read_thinkthen_complete_value_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_value_v1, ErrorKind> {
    if node.kind() == "boolean" {
        let value = read_boolean(node, store)?;
        return Ok(thinkthen_complete_value_v1 {
            kind: THINKTHEN_COMPLETE_VALUE_BOOLEAN_V1,
            data: thinkthen_complete_value_data_v1 { boolean: value },
        });
    }
    if node.kind() == "null" {
        let value = 0;
        return Ok(thinkthen_complete_value_v1 {
            kind: THINKTHEN_COMPLETE_VALUE_NULL_V1,
            data: thinkthen_complete_value_data_v1 { null: value },
        });
    }
    if node.kind() == "string" {
        let value = read_text(node, store)?;
        return Ok(thinkthen_complete_value_v1 {
            kind: THINKTHEN_COMPLETE_VALUE_STRING_V1,
            data: thinkthen_complete_value_data_v1 { string: value },
        });
    }
    if node.kind() == "array" {
        let value = read_thinkthen_complete_string_list_v1(node, store)?;
        return Ok(thinkthen_complete_value_v1 {
            kind: THINKTHEN_COMPLETE_VALUE_ARRAY_V1,
            data: thinkthen_complete_value_data_v1 { array: value },
        });
    }
    if node.kind() == "number" {
        let value = read_number(node, store)?;
        return Ok(thinkthen_complete_value_v1 {
            kind: THINKTHEN_COMPLETE_VALUE_NUMBER_V1,
            data: thinkthen_complete_value_data_v1 { number: value },
        });
    }
    Err(ErrorKind::Defect)
}
