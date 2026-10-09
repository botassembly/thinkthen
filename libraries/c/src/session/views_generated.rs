// Generated from Rust-derived complete result schema; do not edit.
#![allow(
    non_camel_case_types,
    missing_docs,
    reason = "generated versioned C declarations"
)]
use super::views::{Node, Storage, read_json, zero};
use super::views::{
    thinkthen_complete_extensions_v1, thinkthen_complete_json_v1, thinkthen_complete_utf8_v1,
};
use thinkthen::ErrorKind;
pub const THINKTHEN_COMPLETE_PRESENCE_MISSING_V1: u32 = 0;
pub const THINKTHEN_COMPLETE_PRESENCE_NULL_V1: u32 = 1;
pub const THINKTHEN_COMPLETE_PRESENCE_VALUE_V1: u32 = 2;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_annotation_field_answers_entry_v1 {
    pub name: thinkthen_complete_utf8_v1,
    pub value: *const thinkthen_complete_annotation_member_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_annotation_field_answers_v1 {
    pub data: *const thinkthen_complete_annotation_field_answers_entry_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_annotation_field_answers_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_annotation_field_answers_v1, ErrorKind> {
    let mut values = Vec::new();
    for (key, item) in node.object()? {
        values.push(thinkthen_complete_annotation_field_answers_entry_v1 {
            name: store.text(key),
            value: read_thinkthen_complete_annotation_member_v1(item, store)
                .map(|value| store.hold(value))?,
        });
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_annotation_field_answers_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_annotation_field_file_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_utf8_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_annotation_field_first_line_presence_v1 {
    pub presence: u32,
    pub value: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_annotation_field_index_presence_v1 {
    pub presence: u32,
    pub value: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_annotation_field_input_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_json_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_annotation_field_last_line_presence_v1 {
    pub presence: u32,
    pub value: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_annotation_field_position_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_position_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_annotation_field_source_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_physical_source_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_annotation_v1 {
    pub answer_id: *const thinkthen_complete_answer_id_v1,
    pub answers: thinkthen_complete_annotation_field_answers_v1,
    pub file: thinkthen_complete_annotation_field_file_presence_v1,
    pub first_line: thinkthen_complete_annotation_field_first_line_presence_v1,
    pub index: thinkthen_complete_annotation_field_index_presence_v1,
    pub input: thinkthen_complete_annotation_field_input_presence_v1,
    pub last_line: thinkthen_complete_annotation_field_last_line_presence_v1,
    pub meta: *const thinkthen_complete_meta_v1,
    pub position: thinkthen_complete_annotation_field_position_presence_v1,
    pub schema: *const thinkthen_complete_version_v1,
    pub source: thinkthen_complete_annotation_field_source_presence_v1,
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
            read_thinkthen_complete_annotation_field_answers_v1(value, store)?
        },
        file: match node.member("file") {
            None => thinkthen_complete_annotation_field_file_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_annotation_field_file_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: store.text(value.text()?),
            },
        },
        first_line: match node.member("first_line") {
            None => thinkthen_complete_annotation_field_first_line_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_annotation_field_first_line_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: value.number()?,
            },
        },
        index: match node.member("index") {
            None => thinkthen_complete_annotation_field_index_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_annotation_field_index_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: value.number()?,
            },
        },
        input: match node.member("input") {
            None => return Err(ErrorKind::Defect),
            Some(value) if value.kind() == "null" => {
                thinkthen_complete_annotation_field_input_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_NULL_V1,
                    value: zero(),
                }
            }
            Some(value) => thinkthen_complete_annotation_field_input_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_json(value, store)?,
            },
        },
        last_line: match node.member("last_line") {
            None => thinkthen_complete_annotation_field_last_line_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_annotation_field_last_line_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: value.number()?,
            },
        },
        meta: {
            let value = node.required("meta")?;
            read_thinkthen_complete_meta_v1(value, store).map(|value| store.hold(value))?
        },
        position: match node.member("position") {
            None => thinkthen_complete_annotation_field_position_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_annotation_field_position_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_position_v1(value, store)
                    .map(|value| store.hold(value))?,
            },
        },
        schema: {
            let value = node.required("schema")?;
            read_thinkthen_complete_version_v1(value, store).map(|value| store.hold(value))?
        },
        source: match node.member("source") {
            None => thinkthen_complete_annotation_field_source_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_annotation_field_source_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_physical_source_v1(value, store)
                    .map(|value| store.hold(value))?,
            },
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
pub struct thinkthen_complete_annotation_member_answer_id_field_observations_v1 {
    pub data: *const *const thinkthen_complete_observation_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_annotation_member_answer_id_field_observations_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_annotation_member_answer_id_field_observations_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_observation_v1(item, store).map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_annotation_member_answer_id_field_observations_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_annotation_member_answer_id_field_question_sources_v1 {
    pub data: *const *const thinkthen_complete_question_source_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_annotation_member_answer_id_field_question_sources_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_annotation_member_answer_id_field_question_sources_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_question_source_v1(item, store)
                .map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_annotation_member_answer_id_field_question_sources_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_annotation_member_answer_id_field_threshold_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_threshold_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_annotation_member_answer_id_field_usage_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_usage_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_annotation_member_answer_id_field_value_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_value_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_annotation_member_answer_id_v1 {
    pub answer: *const thinkthen_complete_answer_v1,
    pub answer_id: *const thinkthen_complete_answer_id_v1,
    pub observations: thinkthen_complete_annotation_member_answer_id_field_observations_v1,
    pub question: *const thinkthen_complete_readable_question_v1,
    pub question_sources: thinkthen_complete_annotation_member_answer_id_field_question_sources_v1,
    pub request: thinkthen_complete_utf8_v1,
    pub threshold: thinkthen_complete_annotation_member_answer_id_field_threshold_presence_v1,
    pub usage: thinkthen_complete_annotation_member_answer_id_field_usage_presence_v1,
    pub value: thinkthen_complete_annotation_member_answer_id_field_value_presence_v1,
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
            read_thinkthen_complete_annotation_member_answer_id_field_observations_v1(value, store)?
        },
        question: {
            let value = node.required("question")?;
            read_thinkthen_complete_readable_question_v1(value, store)
                .map(|value| store.hold(value))?
        },
        question_sources: {
            let value = node.required("question_sources")?;
            read_thinkthen_complete_annotation_member_answer_id_field_question_sources_v1(
                value, store,
            )?
        },
        request: {
            let value = node.required("request")?;
            store.text(value.text()?)
        },
        threshold: match node.member("threshold") {
            None => return Err(ErrorKind::Defect),
            Some(value) if value.kind() == "null" => {
                thinkthen_complete_annotation_member_answer_id_field_threshold_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_NULL_V1,
                    value: zero(),
                }
            }
            Some(value) => {
                thinkthen_complete_annotation_member_answer_id_field_threshold_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                    value: read_thinkthen_complete_threshold_v1(value, store)
                        .map(|value| store.hold(value))?,
                }
            }
        },
        usage: match node.member("usage") {
            None => thinkthen_complete_annotation_member_answer_id_field_usage_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_annotation_member_answer_id_field_usage_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_usage_v1(value, store)
                    .map(|value| store.hold(value))?,
            },
        },
        value: match node.member("value") {
            None => return Err(ErrorKind::Defect),
            Some(value) if value.kind() == "null" => {
                thinkthen_complete_annotation_member_answer_id_field_value_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_NULL_V1,
                    value: zero(),
                }
            }
            Some(value) => thinkthen_complete_annotation_member_answer_id_field_value_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_value_v1(value, store)
                    .map(|value| store.hold(value))?,
            },
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
pub struct thinkthen_complete_annotation_member_failure_id_field_observations_v1 {
    pub data: *const *const thinkthen_complete_observation_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_annotation_member_failure_id_field_observations_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_annotation_member_failure_id_field_observations_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_observation_v1(item, store).map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_annotation_member_failure_id_field_observations_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_annotation_member_failure_id_field_question_sources_v1 {
    pub data: *const *const thinkthen_complete_question_source_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_annotation_member_failure_id_field_question_sources_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_annotation_member_failure_id_field_question_sources_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_question_source_v1(item, store)
                .map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_annotation_member_failure_id_field_question_sources_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_annotation_member_failure_id_field_threshold_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_threshold_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_annotation_member_failure_id_field_usage_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_usage_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_annotation_member_failure_id_v1 {
    pub failure: *const thinkthen_complete_failure_v1,
    pub failure_id: *const thinkthen_complete_failure_id_v1,
    pub observations: thinkthen_complete_annotation_member_failure_id_field_observations_v1,
    pub question: *const thinkthen_complete_readable_question_v1,
    pub question_sources: thinkthen_complete_annotation_member_failure_id_field_question_sources_v1,
    pub request: thinkthen_complete_utf8_v1,
    pub threshold: thinkthen_complete_annotation_member_failure_id_field_threshold_presence_v1,
    pub usage: thinkthen_complete_annotation_member_failure_id_field_usage_presence_v1,
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
            read_thinkthen_complete_annotation_member_failure_id_field_observations_v1(
                value, store,
            )?
        },
        question: {
            let value = node.required("question")?;
            read_thinkthen_complete_readable_question_v1(value, store)
                .map(|value| store.hold(value))?
        },
        question_sources: {
            let value = node.required("question_sources")?;
            read_thinkthen_complete_annotation_member_failure_id_field_question_sources_v1(
                value, store,
            )?
        },
        request: {
            let value = node.required("request")?;
            store.text(value.text()?)
        },
        threshold: match node.member("threshold") {
            None => return Err(ErrorKind::Defect),
            Some(value) if value.kind() == "null" => {
                thinkthen_complete_annotation_member_failure_id_field_threshold_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_NULL_V1,
                    value: zero(),
                }
            }
            Some(value) => {
                thinkthen_complete_annotation_member_failure_id_field_threshold_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                    value: read_thinkthen_complete_threshold_v1(value, store)
                        .map(|value| store.hold(value))?,
                }
            }
        },
        usage: match node.member("usage") {
            None => thinkthen_complete_annotation_member_failure_id_field_usage_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => {
                thinkthen_complete_annotation_member_failure_id_field_usage_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                    value: read_thinkthen_complete_usage_v1(value, store)
                        .map(|value| store.hold(value))?,
                }
            }
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
pub struct thinkthen_complete_annotation_value_choice_field_value_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_utf8_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_annotation_value_choice_v1 {
    pub kind: thinkthen_complete_utf8_v1,
    pub value: thinkthen_complete_annotation_value_choice_field_value_presence_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_annotation_value_choice_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_annotation_value_choice_v1, ErrorKind> {
    Ok(thinkthen_complete_annotation_value_choice_v1 {
        kind: {
            let value = node.required("kind")?;
            store.text(value.text()?)
        },
        value: match node.member("value") {
            None => return Err(ErrorKind::Defect),
            Some(value) if value.kind() == "null" => {
                thinkthen_complete_annotation_value_choice_field_value_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_NULL_V1,
                    value: zero(),
                }
            }
            Some(value) => thinkthen_complete_annotation_value_choice_field_value_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: store.text(value.text()?),
            },
        },
        extensions: store.extensions(node, &["kind", "value"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_annotation_value_decision_field_value_presence_v1 {
    pub presence: u32,
    pub value: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_annotation_value_decision_v1 {
    pub kind: thinkthen_complete_utf8_v1,
    pub value: thinkthen_complete_annotation_value_decision_field_value_presence_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_annotation_value_decision_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_annotation_value_decision_v1, ErrorKind> {
    Ok(thinkthen_complete_annotation_value_decision_v1 {
        kind: {
            let value = node.required("kind")?;
            store.text(value.text()?)
        },
        value: match node.member("value") {
            None => return Err(ErrorKind::Defect),
            Some(value) if value.kind() == "null" => {
                thinkthen_complete_annotation_value_decision_field_value_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_NULL_V1,
                    value: zero(),
                }
            }
            Some(value) => thinkthen_complete_annotation_value_decision_field_value_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: value.boolean()?,
            },
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
            store.text(value.text()?)
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
            store.text(value.text()?)
        },
        value: {
            let value = node.required("value")?;
            value.number()?
        },
        extensions: store.extensions(node, &["kind", "value"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_annotation_value_tags_field_value_v1 {
    pub data: *const thinkthen_complete_utf8_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_annotation_value_tags_field_value_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_annotation_value_tags_field_value_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(store.text(item.text()?));
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_annotation_value_tags_field_value_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_annotation_value_tags_v1 {
    pub kind: thinkthen_complete_utf8_v1,
    pub value: thinkthen_complete_annotation_value_tags_field_value_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_annotation_value_tags_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_annotation_value_tags_v1, ErrorKind> {
    Ok(thinkthen_complete_annotation_value_tags_v1 {
        kind: {
            let value = node.required("kind")?;
            store.text(value.text()?)
        },
        value: {
            let value = node.required("value")?;
            read_thinkthen_complete_annotation_value_tags_field_value_v1(value, store)?
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
    let value = store.text(node.text()?);
    Ok(thinkthen_complete_answer_id_v1 { value })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answers_field_questions_v1 {
    pub data: *const *const thinkthen_complete_relation_member_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_answers_field_questions_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_answers_field_questions_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_relation_member_v1(item, store)
                .map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_answers_field_questions_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answers_v1 {
    pub questions: thinkthen_complete_answers_field_questions_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_answers_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_answers_v1, ErrorKind> {
    Ok(thinkthen_complete_answers_v1 {
        questions: {
            let value = node.required("questions")?;
            read_thinkthen_complete_answers_field_questions_v1(value, store)?
        },
        extensions: store.extensions(node, &["questions"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_atomic_array_of_string_field_images_v1 {
    pub data: *const *const thinkthen_complete_image_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_atomic_array_of_string_field_images_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_atomic_array_of_string_field_images_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(read_thinkthen_complete_image_v1(item, store).map(|value| store.hold(value))?);
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_atomic_array_of_string_field_images_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_atomic_array_of_string_field_images_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_atomic_array_of_string_field_images_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_atomic_array_of_string_field_index_presence_v1 {
    pub presence: u32,
    pub value: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_atomic_array_of_string_field_input_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_json_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_atomic_array_of_string_field_members_v1 {
    pub data: *const *const thinkthen_complete_rank_member_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_atomic_array_of_string_field_members_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_atomic_array_of_string_field_members_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_rank_member_v1(item, store).map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_atomic_array_of_string_field_members_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_atomic_array_of_string_field_members_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_atomic_array_of_string_field_members_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_atomic_array_of_string_field_question_name_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_utf8_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_atomic_array_of_string_field_source_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_physical_source_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_atomic_array_of_string_field_threshold_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_threshold_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_atomic_array_of_string_field_value_v1 {
    pub data: *const thinkthen_complete_utf8_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_atomic_array_of_string_field_value_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_atomic_array_of_string_field_value_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(store.text(item.text()?));
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_atomic_array_of_string_field_value_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_atomic_array_of_string_v1 {
    pub answer: *const thinkthen_complete_answer_v1,
    pub answer_id: *const thinkthen_complete_answer_id_v1,
    pub images: thinkthen_complete_atomic_array_of_string_field_images_presence_v1,
    pub index: thinkthen_complete_atomic_array_of_string_field_index_presence_v1,
    pub input: thinkthen_complete_atomic_array_of_string_field_input_presence_v1,
    pub members: thinkthen_complete_atomic_array_of_string_field_members_presence_v1,
    pub meta: *const thinkthen_complete_meta_v1,
    pub question: *const thinkthen_complete_readable_question_v1,
    pub question_name: thinkthen_complete_atomic_array_of_string_field_question_name_presence_v1,
    pub schema: *const thinkthen_complete_version_v1,
    pub source: thinkthen_complete_atomic_array_of_string_field_source_presence_v1,
    pub threshold: thinkthen_complete_atomic_array_of_string_field_threshold_presence_v1,
    pub value: thinkthen_complete_atomic_array_of_string_field_value_v1,
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
        images: match node.member("images") {
            None => thinkthen_complete_atomic_array_of_string_field_images_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_atomic_array_of_string_field_images_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_atomic_array_of_string_field_images_v1(
                    value, store,
                )?,
            },
        },
        index: match node.member("index") {
            None => thinkthen_complete_atomic_array_of_string_field_index_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_atomic_array_of_string_field_index_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: value.number()?,
            },
        },
        input: match node.member("input") {
            None => thinkthen_complete_atomic_array_of_string_field_input_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) if value.kind() == "null" => {
                thinkthen_complete_atomic_array_of_string_field_input_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_NULL_V1,
                    value: zero(),
                }
            }
            Some(value) => thinkthen_complete_atomic_array_of_string_field_input_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_json(value, store)?,
            },
        },
        members: match node.member("members") {
            None => thinkthen_complete_atomic_array_of_string_field_members_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_atomic_array_of_string_field_members_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_atomic_array_of_string_field_members_v1(
                    value, store,
                )?,
            },
        },
        meta: {
            let value = node.required("meta")?;
            read_thinkthen_complete_meta_v1(value, store).map(|value| store.hold(value))?
        },
        question: {
            let value = node.required("question")?;
            read_thinkthen_complete_readable_question_v1(value, store)
                .map(|value| store.hold(value))?
        },
        question_name: match node.member("question_name") {
            None => thinkthen_complete_atomic_array_of_string_field_question_name_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => {
                thinkthen_complete_atomic_array_of_string_field_question_name_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                    value: store.text(value.text()?),
                }
            }
        },
        schema: {
            let value = node.required("schema")?;
            read_thinkthen_complete_version_v1(value, store).map(|value| store.hold(value))?
        },
        source: match node.member("source") {
            None => thinkthen_complete_atomic_array_of_string_field_source_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_atomic_array_of_string_field_source_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_physical_source_v1(value, store)
                    .map(|value| store.hold(value))?,
            },
        },
        threshold: match node.member("threshold") {
            None => return Err(ErrorKind::Defect),
            Some(value) if value.kind() == "null" => {
                thinkthen_complete_atomic_array_of_string_field_threshold_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_NULL_V1,
                    value: zero(),
                }
            }
            Some(value) => thinkthen_complete_atomic_array_of_string_field_threshold_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_threshold_v1(value, store)
                    .map(|value| store.hold(value))?,
            },
        },
        value: {
            let value = node.required("value")?;
            read_thinkthen_complete_atomic_array_of_string_field_value_v1(value, store)?
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
pub struct thinkthen_complete_atomic_decide_value_field_images_v1 {
    pub data: *const *const thinkthen_complete_image_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_atomic_decide_value_field_images_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_atomic_decide_value_field_images_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(read_thinkthen_complete_image_v1(item, store).map(|value| store.hold(value))?);
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_atomic_decide_value_field_images_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_atomic_decide_value_field_images_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_atomic_decide_value_field_images_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_atomic_decide_value_field_index_presence_v1 {
    pub presence: u32,
    pub value: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_atomic_decide_value_field_input_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_json_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_atomic_decide_value_field_members_v1 {
    pub data: *const *const thinkthen_complete_rank_member_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_atomic_decide_value_field_members_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_atomic_decide_value_field_members_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_rank_member_v1(item, store).map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_atomic_decide_value_field_members_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_atomic_decide_value_field_members_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_atomic_decide_value_field_members_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_atomic_decide_value_field_question_name_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_utf8_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_atomic_decide_value_field_source_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_physical_source_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_atomic_decide_value_field_threshold_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_threshold_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_atomic_decide_value_field_value_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_decide_value_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_atomic_decide_value_v1 {
    pub answer: *const thinkthen_complete_answer_v1,
    pub answer_id: *const thinkthen_complete_answer_id_v1,
    pub images: thinkthen_complete_atomic_decide_value_field_images_presence_v1,
    pub index: thinkthen_complete_atomic_decide_value_field_index_presence_v1,
    pub input: thinkthen_complete_atomic_decide_value_field_input_presence_v1,
    pub members: thinkthen_complete_atomic_decide_value_field_members_presence_v1,
    pub meta: *const thinkthen_complete_meta_v1,
    pub question: *const thinkthen_complete_readable_question_v1,
    pub question_name: thinkthen_complete_atomic_decide_value_field_question_name_presence_v1,
    pub schema: *const thinkthen_complete_version_v1,
    pub source: thinkthen_complete_atomic_decide_value_field_source_presence_v1,
    pub threshold: thinkthen_complete_atomic_decide_value_field_threshold_presence_v1,
    pub value: thinkthen_complete_atomic_decide_value_field_value_presence_v1,
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
        images: match node.member("images") {
            None => thinkthen_complete_atomic_decide_value_field_images_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_atomic_decide_value_field_images_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_atomic_decide_value_field_images_v1(value, store)?,
            },
        },
        index: match node.member("index") {
            None => thinkthen_complete_atomic_decide_value_field_index_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_atomic_decide_value_field_index_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: value.number()?,
            },
        },
        input: match node.member("input") {
            None => thinkthen_complete_atomic_decide_value_field_input_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) if value.kind() == "null" => {
                thinkthen_complete_atomic_decide_value_field_input_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_NULL_V1,
                    value: zero(),
                }
            }
            Some(value) => thinkthen_complete_atomic_decide_value_field_input_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_json(value, store)?,
            },
        },
        members: match node.member("members") {
            None => thinkthen_complete_atomic_decide_value_field_members_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_atomic_decide_value_field_members_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_atomic_decide_value_field_members_v1(value, store)?,
            },
        },
        meta: {
            let value = node.required("meta")?;
            read_thinkthen_complete_meta_v1(value, store).map(|value| store.hold(value))?
        },
        question: {
            let value = node.required("question")?;
            read_thinkthen_complete_readable_question_v1(value, store)
                .map(|value| store.hold(value))?
        },
        question_name: match node.member("question_name") {
            None => thinkthen_complete_atomic_decide_value_field_question_name_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_atomic_decide_value_field_question_name_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: store.text(value.text()?),
            },
        },
        schema: {
            let value = node.required("schema")?;
            read_thinkthen_complete_version_v1(value, store).map(|value| store.hold(value))?
        },
        source: match node.member("source") {
            None => thinkthen_complete_atomic_decide_value_field_source_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_atomic_decide_value_field_source_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_physical_source_v1(value, store)
                    .map(|value| store.hold(value))?,
            },
        },
        threshold: match node.member("threshold") {
            None => return Err(ErrorKind::Defect),
            Some(value) if value.kind() == "null" => {
                thinkthen_complete_atomic_decide_value_field_threshold_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_NULL_V1,
                    value: zero(),
                }
            }
            Some(value) => thinkthen_complete_atomic_decide_value_field_threshold_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_threshold_v1(value, store)
                    .map(|value| store.hold(value))?,
            },
        },
        value: match node.member("value") {
            None => return Err(ErrorKind::Defect),
            Some(value) if value.kind() == "null" => {
                thinkthen_complete_atomic_decide_value_field_value_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_NULL_V1,
                    value: zero(),
                }
            }
            Some(value) => thinkthen_complete_atomic_decide_value_field_value_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_decide_value_v1(value, store)
                    .map(|value| store.hold(value))?,
            },
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
pub struct thinkthen_complete_atomic_non_zero_usize_field_images_v1 {
    pub data: *const *const thinkthen_complete_image_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_atomic_non_zero_usize_field_images_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_atomic_non_zero_usize_field_images_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(read_thinkthen_complete_image_v1(item, store).map(|value| store.hold(value))?);
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_atomic_non_zero_usize_field_images_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_atomic_non_zero_usize_field_images_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_atomic_non_zero_usize_field_images_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_atomic_non_zero_usize_field_index_presence_v1 {
    pub presence: u32,
    pub value: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_atomic_non_zero_usize_field_input_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_json_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_atomic_non_zero_usize_field_members_v1 {
    pub data: *const *const thinkthen_complete_rank_member_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_atomic_non_zero_usize_field_members_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_atomic_non_zero_usize_field_members_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_rank_member_v1(item, store).map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_atomic_non_zero_usize_field_members_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_atomic_non_zero_usize_field_members_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_atomic_non_zero_usize_field_members_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_atomic_non_zero_usize_field_question_name_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_utf8_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_atomic_non_zero_usize_field_source_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_physical_source_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_atomic_non_zero_usize_field_threshold_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_threshold_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_atomic_non_zero_usize_v1 {
    pub answer: *const thinkthen_complete_answer_v1,
    pub answer_id: *const thinkthen_complete_answer_id_v1,
    pub images: thinkthen_complete_atomic_non_zero_usize_field_images_presence_v1,
    pub index: thinkthen_complete_atomic_non_zero_usize_field_index_presence_v1,
    pub input: thinkthen_complete_atomic_non_zero_usize_field_input_presence_v1,
    pub members: thinkthen_complete_atomic_non_zero_usize_field_members_presence_v1,
    pub meta: *const thinkthen_complete_meta_v1,
    pub question: *const thinkthen_complete_readable_question_v1,
    pub question_name: thinkthen_complete_atomic_non_zero_usize_field_question_name_presence_v1,
    pub schema: *const thinkthen_complete_version_v1,
    pub source: thinkthen_complete_atomic_non_zero_usize_field_source_presence_v1,
    pub threshold: thinkthen_complete_atomic_non_zero_usize_field_threshold_presence_v1,
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
        images: match node.member("images") {
            None => thinkthen_complete_atomic_non_zero_usize_field_images_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_atomic_non_zero_usize_field_images_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_atomic_non_zero_usize_field_images_v1(value, store)?,
            },
        },
        index: match node.member("index") {
            None => thinkthen_complete_atomic_non_zero_usize_field_index_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_atomic_non_zero_usize_field_index_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: value.number()?,
            },
        },
        input: match node.member("input") {
            None => thinkthen_complete_atomic_non_zero_usize_field_input_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) if value.kind() == "null" => {
                thinkthen_complete_atomic_non_zero_usize_field_input_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_NULL_V1,
                    value: zero(),
                }
            }
            Some(value) => thinkthen_complete_atomic_non_zero_usize_field_input_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_json(value, store)?,
            },
        },
        members: match node.member("members") {
            None => thinkthen_complete_atomic_non_zero_usize_field_members_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_atomic_non_zero_usize_field_members_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_atomic_non_zero_usize_field_members_v1(
                    value, store,
                )?,
            },
        },
        meta: {
            let value = node.required("meta")?;
            read_thinkthen_complete_meta_v1(value, store).map(|value| store.hold(value))?
        },
        question: {
            let value = node.required("question")?;
            read_thinkthen_complete_readable_question_v1(value, store)
                .map(|value| store.hold(value))?
        },
        question_name: match node.member("question_name") {
            None => thinkthen_complete_atomic_non_zero_usize_field_question_name_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => {
                thinkthen_complete_atomic_non_zero_usize_field_question_name_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                    value: store.text(value.text()?),
                }
            }
        },
        schema: {
            let value = node.required("schema")?;
            read_thinkthen_complete_version_v1(value, store).map(|value| store.hold(value))?
        },
        source: match node.member("source") {
            None => thinkthen_complete_atomic_non_zero_usize_field_source_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_atomic_non_zero_usize_field_source_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_physical_source_v1(value, store)
                    .map(|value| store.hold(value))?,
            },
        },
        threshold: match node.member("threshold") {
            None => return Err(ErrorKind::Defect),
            Some(value) if value.kind() == "null" => {
                thinkthen_complete_atomic_non_zero_usize_field_threshold_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_NULL_V1,
                    value: zero(),
                }
            }
            Some(value) => thinkthen_complete_atomic_non_zero_usize_field_threshold_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_threshold_v1(value, store)
                    .map(|value| store.hold(value))?,
            },
        },
        value: {
            let value = node.required("value")?;
            value.number()?
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
pub struct thinkthen_complete_atomic_nullable_string_field_images_v1 {
    pub data: *const *const thinkthen_complete_image_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_atomic_nullable_string_field_images_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_atomic_nullable_string_field_images_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(read_thinkthen_complete_image_v1(item, store).map(|value| store.hold(value))?);
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_atomic_nullable_string_field_images_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_atomic_nullable_string_field_images_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_atomic_nullable_string_field_images_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_atomic_nullable_string_field_index_presence_v1 {
    pub presence: u32,
    pub value: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_atomic_nullable_string_field_input_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_json_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_atomic_nullable_string_field_members_v1 {
    pub data: *const *const thinkthen_complete_rank_member_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_atomic_nullable_string_field_members_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_atomic_nullable_string_field_members_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_rank_member_v1(item, store).map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_atomic_nullable_string_field_members_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_atomic_nullable_string_field_members_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_atomic_nullable_string_field_members_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_atomic_nullable_string_field_question_name_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_utf8_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_atomic_nullable_string_field_source_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_physical_source_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_atomic_nullable_string_field_threshold_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_threshold_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_atomic_nullable_string_field_value_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_utf8_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_atomic_nullable_string_v1 {
    pub answer: *const thinkthen_complete_answer_v1,
    pub answer_id: *const thinkthen_complete_answer_id_v1,
    pub images: thinkthen_complete_atomic_nullable_string_field_images_presence_v1,
    pub index: thinkthen_complete_atomic_nullable_string_field_index_presence_v1,
    pub input: thinkthen_complete_atomic_nullable_string_field_input_presence_v1,
    pub members: thinkthen_complete_atomic_nullable_string_field_members_presence_v1,
    pub meta: *const thinkthen_complete_meta_v1,
    pub question: *const thinkthen_complete_readable_question_v1,
    pub question_name: thinkthen_complete_atomic_nullable_string_field_question_name_presence_v1,
    pub schema: *const thinkthen_complete_version_v1,
    pub source: thinkthen_complete_atomic_nullable_string_field_source_presence_v1,
    pub threshold: thinkthen_complete_atomic_nullable_string_field_threshold_presence_v1,
    pub value: thinkthen_complete_atomic_nullable_string_field_value_presence_v1,
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
        images: match node.member("images") {
            None => thinkthen_complete_atomic_nullable_string_field_images_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_atomic_nullable_string_field_images_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_atomic_nullable_string_field_images_v1(
                    value, store,
                )?,
            },
        },
        index: match node.member("index") {
            None => thinkthen_complete_atomic_nullable_string_field_index_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_atomic_nullable_string_field_index_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: value.number()?,
            },
        },
        input: match node.member("input") {
            None => thinkthen_complete_atomic_nullable_string_field_input_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) if value.kind() == "null" => {
                thinkthen_complete_atomic_nullable_string_field_input_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_NULL_V1,
                    value: zero(),
                }
            }
            Some(value) => thinkthen_complete_atomic_nullable_string_field_input_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_json(value, store)?,
            },
        },
        members: match node.member("members") {
            None => thinkthen_complete_atomic_nullable_string_field_members_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_atomic_nullable_string_field_members_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_atomic_nullable_string_field_members_v1(
                    value, store,
                )?,
            },
        },
        meta: {
            let value = node.required("meta")?;
            read_thinkthen_complete_meta_v1(value, store).map(|value| store.hold(value))?
        },
        question: {
            let value = node.required("question")?;
            read_thinkthen_complete_readable_question_v1(value, store)
                .map(|value| store.hold(value))?
        },
        question_name: match node.member("question_name") {
            None => thinkthen_complete_atomic_nullable_string_field_question_name_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => {
                thinkthen_complete_atomic_nullable_string_field_question_name_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                    value: store.text(value.text()?),
                }
            }
        },
        schema: {
            let value = node.required("schema")?;
            read_thinkthen_complete_version_v1(value, store).map(|value| store.hold(value))?
        },
        source: match node.member("source") {
            None => thinkthen_complete_atomic_nullable_string_field_source_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_atomic_nullable_string_field_source_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_physical_source_v1(value, store)
                    .map(|value| store.hold(value))?,
            },
        },
        threshold: match node.member("threshold") {
            None => return Err(ErrorKind::Defect),
            Some(value) if value.kind() == "null" => {
                thinkthen_complete_atomic_nullable_string_field_threshold_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_NULL_V1,
                    value: zero(),
                }
            }
            Some(value) => thinkthen_complete_atomic_nullable_string_field_threshold_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_threshold_v1(value, store)
                    .map(|value| store.hold(value))?,
            },
        },
        value: match node.member("value") {
            None => return Err(ErrorKind::Defect),
            Some(value) if value.kind() == "null" => {
                thinkthen_complete_atomic_nullable_string_field_value_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_NULL_V1,
                    value: zero(),
                }
            }
            Some(value) => thinkthen_complete_atomic_nullable_string_field_value_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: store.text(value.text()?),
            },
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
pub struct thinkthen_complete_atomic_boolean_field_images_v1 {
    pub data: *const *const thinkthen_complete_image_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_atomic_boolean_field_images_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_atomic_boolean_field_images_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(read_thinkthen_complete_image_v1(item, store).map(|value| store.hold(value))?);
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_atomic_boolean_field_images_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_atomic_boolean_field_images_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_atomic_boolean_field_images_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_atomic_boolean_field_index_presence_v1 {
    pub presence: u32,
    pub value: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_atomic_boolean_field_input_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_json_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_atomic_boolean_field_members_v1 {
    pub data: *const *const thinkthen_complete_rank_member_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_atomic_boolean_field_members_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_atomic_boolean_field_members_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_rank_member_v1(item, store).map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_atomic_boolean_field_members_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_atomic_boolean_field_members_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_atomic_boolean_field_members_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_atomic_boolean_field_question_name_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_utf8_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_atomic_boolean_field_source_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_physical_source_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_atomic_boolean_field_threshold_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_threshold_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_atomic_boolean_v1 {
    pub answer: *const thinkthen_complete_answer_v1,
    pub answer_id: *const thinkthen_complete_answer_id_v1,
    pub images: thinkthen_complete_atomic_boolean_field_images_presence_v1,
    pub index: thinkthen_complete_atomic_boolean_field_index_presence_v1,
    pub input: thinkthen_complete_atomic_boolean_field_input_presence_v1,
    pub members: thinkthen_complete_atomic_boolean_field_members_presence_v1,
    pub meta: *const thinkthen_complete_meta_v1,
    pub question: *const thinkthen_complete_readable_question_v1,
    pub question_name: thinkthen_complete_atomic_boolean_field_question_name_presence_v1,
    pub schema: *const thinkthen_complete_version_v1,
    pub source: thinkthen_complete_atomic_boolean_field_source_presence_v1,
    pub threshold: thinkthen_complete_atomic_boolean_field_threshold_presence_v1,
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
        images: match node.member("images") {
            None => thinkthen_complete_atomic_boolean_field_images_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_atomic_boolean_field_images_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_atomic_boolean_field_images_v1(value, store)?,
            },
        },
        index: match node.member("index") {
            None => thinkthen_complete_atomic_boolean_field_index_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_atomic_boolean_field_index_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: value.number()?,
            },
        },
        input: match node.member("input") {
            None => thinkthen_complete_atomic_boolean_field_input_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) if value.kind() == "null" => {
                thinkthen_complete_atomic_boolean_field_input_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_NULL_V1,
                    value: zero(),
                }
            }
            Some(value) => thinkthen_complete_atomic_boolean_field_input_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_json(value, store)?,
            },
        },
        members: match node.member("members") {
            None => thinkthen_complete_atomic_boolean_field_members_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_atomic_boolean_field_members_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_atomic_boolean_field_members_v1(value, store)?,
            },
        },
        meta: {
            let value = node.required("meta")?;
            read_thinkthen_complete_meta_v1(value, store).map(|value| store.hold(value))?
        },
        question: {
            let value = node.required("question")?;
            read_thinkthen_complete_readable_question_v1(value, store)
                .map(|value| store.hold(value))?
        },
        question_name: match node.member("question_name") {
            None => thinkthen_complete_atomic_boolean_field_question_name_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_atomic_boolean_field_question_name_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: store.text(value.text()?),
            },
        },
        schema: {
            let value = node.required("schema")?;
            read_thinkthen_complete_version_v1(value, store).map(|value| store.hold(value))?
        },
        source: match node.member("source") {
            None => thinkthen_complete_atomic_boolean_field_source_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_atomic_boolean_field_source_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_physical_source_v1(value, store)
                    .map(|value| store.hold(value))?,
            },
        },
        threshold: match node.member("threshold") {
            None => return Err(ErrorKind::Defect),
            Some(value) if value.kind() == "null" => {
                thinkthen_complete_atomic_boolean_field_threshold_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_NULL_V1,
                    value: zero(),
                }
            }
            Some(value) => thinkthen_complete_atomic_boolean_field_threshold_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_threshold_v1(value, store)
                    .map(|value| store.hold(value))?,
            },
        },
        value: {
            let value = node.required("value")?;
            value.boolean()?
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
pub struct thinkthen_complete_atomic_double_field_images_v1 {
    pub data: *const *const thinkthen_complete_image_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_atomic_double_field_images_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_atomic_double_field_images_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(read_thinkthen_complete_image_v1(item, store).map(|value| store.hold(value))?);
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_atomic_double_field_images_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_atomic_double_field_images_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_atomic_double_field_images_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_atomic_double_field_index_presence_v1 {
    pub presence: u32,
    pub value: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_atomic_double_field_input_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_json_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_atomic_double_field_members_v1 {
    pub data: *const *const thinkthen_complete_rank_member_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_atomic_double_field_members_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_atomic_double_field_members_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_rank_member_v1(item, store).map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_atomic_double_field_members_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_atomic_double_field_members_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_atomic_double_field_members_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_atomic_double_field_question_name_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_utf8_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_atomic_double_field_source_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_physical_source_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_atomic_double_field_threshold_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_threshold_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_atomic_double_v1 {
    pub answer: *const thinkthen_complete_answer_v1,
    pub answer_id: *const thinkthen_complete_answer_id_v1,
    pub images: thinkthen_complete_atomic_double_field_images_presence_v1,
    pub index: thinkthen_complete_atomic_double_field_index_presence_v1,
    pub input: thinkthen_complete_atomic_double_field_input_presence_v1,
    pub members: thinkthen_complete_atomic_double_field_members_presence_v1,
    pub meta: *const thinkthen_complete_meta_v1,
    pub question: *const thinkthen_complete_readable_question_v1,
    pub question_name: thinkthen_complete_atomic_double_field_question_name_presence_v1,
    pub schema: *const thinkthen_complete_version_v1,
    pub source: thinkthen_complete_atomic_double_field_source_presence_v1,
    pub threshold: thinkthen_complete_atomic_double_field_threshold_presence_v1,
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
        images: match node.member("images") {
            None => thinkthen_complete_atomic_double_field_images_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_atomic_double_field_images_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_atomic_double_field_images_v1(value, store)?,
            },
        },
        index: match node.member("index") {
            None => thinkthen_complete_atomic_double_field_index_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_atomic_double_field_index_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: value.number()?,
            },
        },
        input: match node.member("input") {
            None => thinkthen_complete_atomic_double_field_input_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) if value.kind() == "null" => {
                thinkthen_complete_atomic_double_field_input_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_NULL_V1,
                    value: zero(),
                }
            }
            Some(value) => thinkthen_complete_atomic_double_field_input_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_json(value, store)?,
            },
        },
        members: match node.member("members") {
            None => thinkthen_complete_atomic_double_field_members_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_atomic_double_field_members_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_atomic_double_field_members_v1(value, store)?,
            },
        },
        meta: {
            let value = node.required("meta")?;
            read_thinkthen_complete_meta_v1(value, store).map(|value| store.hold(value))?
        },
        question: {
            let value = node.required("question")?;
            read_thinkthen_complete_readable_question_v1(value, store)
                .map(|value| store.hold(value))?
        },
        question_name: match node.member("question_name") {
            None => thinkthen_complete_atomic_double_field_question_name_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_atomic_double_field_question_name_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: store.text(value.text()?),
            },
        },
        schema: {
            let value = node.required("schema")?;
            read_thinkthen_complete_version_v1(value, store).map(|value| store.hold(value))?
        },
        source: match node.member("source") {
            None => thinkthen_complete_atomic_double_field_source_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_atomic_double_field_source_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_physical_source_v1(value, store)
                    .map(|value| store.hold(value))?,
            },
        },
        threshold: match node.member("threshold") {
            None => return Err(ErrorKind::Defect),
            Some(value) if value.kind() == "null" => {
                thinkthen_complete_atomic_double_field_threshold_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_NULL_V1,
                    value: zero(),
                }
            }
            Some(value) => thinkthen_complete_atomic_double_field_threshold_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_threshold_v1(value, store)
                    .map(|value| store.hold(value))?,
            },
        },
        value: {
            let value = node.required("value")?;
            value.number()?
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
pub struct thinkthen_complete_attempt_field_request_id_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_utf8_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_attempt_field_server_ms_presence_v1 {
    pub presence: u32,
    pub value: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_attempt_field_status_presence_v1 {
    pub presence: u32,
    pub value: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_attempt_v1 {
    pub ordinal: u64,
    pub outcome: *const thinkthen_complete_attempt_outcome_v1,
    pub request_id: thinkthen_complete_attempt_field_request_id_presence_v1,
    pub request_sha256: thinkthen_complete_utf8_v1,
    pub sdk_request_id: *const thinkthen_complete_sdk_request_id_v1,
    pub server_ms: thinkthen_complete_attempt_field_server_ms_presence_v1,
    pub status: thinkthen_complete_attempt_field_status_presence_v1,
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
            value.number()?
        },
        outcome: {
            let value = node.required("outcome")?;
            read_thinkthen_complete_attempt_outcome_v1(value, store)
                .map(|value| store.hold(value))?
        },
        request_id: match node.member("request_id") {
            None => thinkthen_complete_attempt_field_request_id_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_attempt_field_request_id_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: store.text(value.text()?),
            },
        },
        request_sha256: {
            let value = node.required("request_sha256")?;
            store.text(value.text()?)
        },
        sdk_request_id: {
            let value = node.required("sdk_request_id")?;
            read_thinkthen_complete_sdk_request_id_v1(value, store)
                .map(|value| store.hold(value))?
        },
        server_ms: match node.member("server_ms") {
            None => thinkthen_complete_attempt_field_server_ms_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_attempt_field_server_ms_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: value.number()?,
            },
        },
        status: match node.member("status") {
            None => thinkthen_complete_attempt_field_status_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_attempt_field_status_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: value.number()?,
            },
        },
        wall_ms: {
            let value = node.required("wall_ms")?;
            value.number()?
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
        let value = node.number()?;
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
            value.number()?
        },
        length: {
            let value = node.required("length")?;
            value.number()?
        },
        probability: {
            let value = node.required("probability")?;
            value.number()?
        },
        start: {
            let value = node.required("start")?;
            value.number()?
        },
        text: {
            let value = node.required("text")?;
            store.text(value.text()?)
        },
        extensions: store.extensions(node, &["end", "length", "probability", "start", "text"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_call_error_field_facts_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_facts_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_call_error_v1 {
    pub error: *const thinkthen_complete_error_v1,
    pub facts: thinkthen_complete_call_error_field_facts_presence_v1,
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
        facts: match node.member("facts") {
            None => thinkthen_complete_call_error_field_facts_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_call_error_field_facts_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_facts_v1(value, store)
                    .map(|value| store.hold(value))?,
            },
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
    let value = store.text(node.text()?);
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
            store.text(value.text()?)
        },
        name: {
            let value = node.required("name")?;
            store.text(value.text()?)
        },
        extensions: store.extensions(node, &["kind", "name"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_error_field_estimated_input_denial_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_estimated_input_denial_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_error_field_send_budget_denial_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_send_budget_denial_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_error_v1 {
    pub estimated_input_denial: thinkthen_complete_error_field_estimated_input_denial_presence_v1,
    pub kind: *const thinkthen_complete_failure_kind_v1,
    pub message: thinkthen_complete_utf8_v1,
    pub retryable: u32,
    pub send_budget_denial: thinkthen_complete_error_field_send_budget_denial_presence_v1,
    pub stopped: *const thinkthen_complete_stopped_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_error_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_error_v1, ErrorKind> {
    Ok(thinkthen_complete_error_v1 {
        estimated_input_denial: match node.member("estimated_input_denial") {
            None => thinkthen_complete_error_field_estimated_input_denial_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_error_field_estimated_input_denial_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_estimated_input_denial_v1(value, store)
                    .map(|value| store.hold(value))?,
            },
        },
        kind: {
            let value = node.required("kind")?;
            read_thinkthen_complete_failure_kind_v1(value, store).map(|value| store.hold(value))?
        },
        message: {
            let value = node.required("message")?;
            store.text(value.text()?)
        },
        retryable: {
            let value = node.required("retryable")?;
            value.boolean()?
        },
        send_budget_denial: match node.member("send_budget_denial") {
            None => thinkthen_complete_error_field_send_budget_denial_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_error_field_send_budget_denial_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_send_budget_denial_v1(value, store)
                    .map(|value| store.hold(value))?,
            },
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

pub const THINKTHEN_COMPLETE_ESTIMATED_INPUT_DENIAL_INITIAL_REQUEST_V1: u32 = 1;

pub const THINKTHEN_COMPLETE_ESTIMATED_INPUT_DENIAL_ADDITIONAL_REQUEST_V1: u32 = 2;

pub const THINKTHEN_COMPLETE_ESTIMATED_INPUT_DENIAL_RETRY_V1: u32 = 3;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_estimated_input_denial_data_v1 {
    pub initial_request: *const thinkthen_complete_estimated_input_denial_initial_request_v1,
    pub additional_request: *const thinkthen_complete_estimated_input_denial_additional_request_v1,
    pub retry: *const thinkthen_complete_estimated_input_denial_retry_v1,
}
impl std::fmt::Debug for thinkthen_complete_estimated_input_denial_data_v1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("union payload")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_estimated_input_denial_v1 {
    pub kind: u32,
    pub data: thinkthen_complete_estimated_input_denial_data_v1,
}

pub(crate) fn read_thinkthen_complete_estimated_input_denial_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_estimated_input_denial_v1, ErrorKind> {
    if node.is_member("kind", "initial_request") {
        let value = read_thinkthen_complete_estimated_input_denial_initial_request_v1(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_estimated_input_denial_v1 {
            kind: THINKTHEN_COMPLETE_ESTIMATED_INPUT_DENIAL_INITIAL_REQUEST_V1,
            data: thinkthen_complete_estimated_input_denial_data_v1 {
                initial_request: value,
            },
        });
    }
    if node.is_member("kind", "additional_request") {
        let value =
            read_thinkthen_complete_estimated_input_denial_additional_request_v1(node, store)
                .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_estimated_input_denial_v1 {
            kind: THINKTHEN_COMPLETE_ESTIMATED_INPUT_DENIAL_ADDITIONAL_REQUEST_V1,
            data: thinkthen_complete_estimated_input_denial_data_v1 {
                additional_request: value,
            },
        });
    }
    if node.is_member("kind", "retry") {
        let value = read_thinkthen_complete_estimated_input_denial_retry_v1(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_estimated_input_denial_v1 {
            kind: THINKTHEN_COMPLETE_ESTIMATED_INPUT_DENIAL_RETRY_V1,
            data: thinkthen_complete_estimated_input_denial_data_v1 { retry: value },
        });
    }
    Err(ErrorKind::Defect)
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_estimated_input_denial_additional_request_v1 {
    pub kind: thinkthen_complete_utf8_v1,
    pub limit: u64,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_estimated_input_denial_additional_request_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_estimated_input_denial_additional_request_v1, ErrorKind> {
    Ok(
        thinkthen_complete_estimated_input_denial_additional_request_v1 {
            kind: {
                let value = node.required("kind")?;
                store.text(value.text()?)
            },
            limit: {
                let value = node.required("limit")?;
                value.number()?
            },
            extensions: store.extensions(node, &["kind", "limit"])?,
        },
    )
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_estimated_input_denial_initial_request_v1 {
    pub kind: thinkthen_complete_utf8_v1,
    pub limit: u64,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_estimated_input_denial_initial_request_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_estimated_input_denial_initial_request_v1, ErrorKind> {
    Ok(
        thinkthen_complete_estimated_input_denial_initial_request_v1 {
            kind: {
                let value = node.required("kind")?;
                store.text(value.text()?)
            },
            limit: {
                let value = node.required("limit")?;
                value.number()?
            },
            extensions: store.extensions(node, &["kind", "limit"])?,
        },
    )
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_estimated_input_denial_retry_v1 {
    pub kind: thinkthen_complete_utf8_v1,
    pub last_status: u64,
    pub limit: u64,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_estimated_input_denial_retry_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_estimated_input_denial_retry_v1, ErrorKind> {
    Ok(thinkthen_complete_estimated_input_denial_retry_v1 {
        kind: {
            let value = node.required("kind")?;
            store.text(value.text()?)
        },
        last_status: {
            let value = node.required("last_status")?;
            value.number()?
        },
        limit: {
            let value = node.required("limit")?;
            value.number()?
        },
        extensions: store.extensions(node, &["kind", "last_status", "limit"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_facts_field_attempts_v1 {
    pub data: *const *const thinkthen_complete_attempt_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_facts_field_attempts_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_facts_field_attempts_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values
            .push(read_thinkthen_complete_attempt_v1(item, store).map(|value| store.hold(value))?);
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_facts_field_attempts_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_facts_field_attempts_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_facts_field_attempts_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_facts_field_estimated_cost_usd_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_utf8_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_facts_field_held_model_mismatch_presence_v1 {
    pub presence: u32,
    pub value: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_facts_field_input_tokens_presence_v1 {
    pub presence: u32,
    pub value: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_facts_field_largest_request_estimated_input_tokens_presence_v1 {
    pub presence: u32,
    pub value: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_facts_field_model_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_utf8_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_facts_field_output_tokens_presence_v1 {
    pub presence: u32,
    pub value: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_facts_field_usage_persistence_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_persistence_observation_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_facts_v1 {
    pub attempts: thinkthen_complete_facts_field_attempts_presence_v1,
    pub cache_answers: u64,
    pub call_id: *const thinkthen_complete_call_id_v1,
    pub estimated_cost_usd: thinkthen_complete_facts_field_estimated_cost_usd_presence_v1,
    pub held_model_mismatch: thinkthen_complete_facts_field_held_model_mismatch_presence_v1,
    pub input_tokens: thinkthen_complete_facts_field_input_tokens_presence_v1,
    pub largest_request_bytes: u64,
    pub largest_request_estimated_input_tokens:
        thinkthen_complete_facts_field_largest_request_estimated_input_tokens_presence_v1,
    pub model: thinkthen_complete_facts_field_model_presence_v1,
    pub output_tokens: thinkthen_complete_facts_field_output_tokens_presence_v1,
    pub records: u64,
    pub requests_sent: u64,
    pub seconds: f64,
    pub token_estimate_method: thinkthen_complete_utf8_v1,
    pub usage_persistence: thinkthen_complete_facts_field_usage_persistence_presence_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_facts_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_facts_v1, ErrorKind> {
    Ok(thinkthen_complete_facts_v1 {
        attempts: match node.member("attempts") {
            None => thinkthen_complete_facts_field_attempts_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_facts_field_attempts_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_facts_field_attempts_v1(value, store)?,
            },
        },
        cache_answers: {
            let value = node.required("cache_answers")?;
            value.number()?
        },
        call_id: {
            let value = node.required("call_id")?;
            read_thinkthen_complete_call_id_v1(value, store).map(|value| store.hold(value))?
        },
        estimated_cost_usd: match node.member("estimated_cost_usd") {
            None => thinkthen_complete_facts_field_estimated_cost_usd_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_facts_field_estimated_cost_usd_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: store.text(value.text()?),
            },
        },
        held_model_mismatch: match node.member("held_model_mismatch") {
            None => thinkthen_complete_facts_field_held_model_mismatch_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_facts_field_held_model_mismatch_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: value.boolean()?,
            },
        },
        input_tokens: match node.member("input_tokens") {
            None => thinkthen_complete_facts_field_input_tokens_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_facts_field_input_tokens_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: value.number()?,
            },
        },
        largest_request_bytes: {
            let value = node.required("largest_request_bytes")?;
            value.number()?
        },
        largest_request_estimated_input_tokens: match node
            .member("largest_request_estimated_input_tokens")
        {
            None => return Err(ErrorKind::Defect),
            Some(value) if value.kind() == "null" => {
                thinkthen_complete_facts_field_largest_request_estimated_input_tokens_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_NULL_V1,
                    value: zero(),
                }
            }
            Some(value) => {
                thinkthen_complete_facts_field_largest_request_estimated_input_tokens_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                    value: value.number()?,
                }
            }
        },
        model: match node.member("model") {
            None => thinkthen_complete_facts_field_model_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_facts_field_model_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: store.text(value.text()?),
            },
        },
        output_tokens: match node.member("output_tokens") {
            None => thinkthen_complete_facts_field_output_tokens_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_facts_field_output_tokens_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: value.number()?,
            },
        },
        records: {
            let value = node.required("records")?;
            value.number()?
        },
        requests_sent: {
            let value = node.required("requests_sent")?;
            value.number()?
        },
        seconds: {
            let value = node.required("seconds")?;
            value.number()?
        },
        token_estimate_method: {
            let value = node.required("token_estimate_method")?;
            store.text(value.text()?)
        },
        usage_persistence: match node.member("usage_persistence") {
            None => thinkthen_complete_facts_field_usage_persistence_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_facts_field_usage_persistence_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_persistence_observation_v1(value, store)
                    .map(|value| store.hold(value))?,
            },
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
    let value = store.text(node.text()?);
    Ok(thinkthen_complete_failure_id_v1 { value })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_find_field_candidates_v1 {
    pub data: *const *const thinkthen_complete_find_candidate_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_find_field_candidates_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_find_field_candidates_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_find_candidate_v1(item, store)
                .map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_find_field_candidates_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_find_field_candidates_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_find_field_candidates_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_find_field_file_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_utf8_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_find_field_first_line_presence_v1 {
    pub presence: u32,
    pub value: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_find_field_index_presence_v1 {
    pub presence: u32,
    pub value: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_find_field_last_line_presence_v1 {
    pub presence: u32,
    pub value: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_find_field_position_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_position_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_find_field_threshold_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_json_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_find_field_value_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_json_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_find_v1 {
    pub answer: *const thinkthen_complete_find_answer_v1,
    pub answer_id: *const thinkthen_complete_answer_id_v1,
    pub candidates: thinkthen_complete_find_field_candidates_presence_v1,
    pub file: thinkthen_complete_find_field_file_presence_v1,
    pub first_line: thinkthen_complete_find_field_first_line_presence_v1,
    pub index: thinkthen_complete_find_field_index_presence_v1,
    pub last_line: thinkthen_complete_find_field_last_line_presence_v1,
    pub meta: *const thinkthen_complete_meta_v1,
    pub position: thinkthen_complete_find_field_position_presence_v1,
    pub question: *const thinkthen_complete_readable_question_2_v1,
    pub schema: *const thinkthen_complete_version_v1,
    pub threshold: thinkthen_complete_find_field_threshold_presence_v1,
    pub value: thinkthen_complete_find_field_value_presence_v1,
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
        candidates: match node.member("candidates") {
            None => thinkthen_complete_find_field_candidates_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_find_field_candidates_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_find_field_candidates_v1(value, store)?,
            },
        },
        file: match node.member("file") {
            None => thinkthen_complete_find_field_file_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_find_field_file_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: store.text(value.text()?),
            },
        },
        first_line: match node.member("first_line") {
            None => thinkthen_complete_find_field_first_line_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_find_field_first_line_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: value.number()?,
            },
        },
        index: match node.member("index") {
            None => return Err(ErrorKind::Defect),
            Some(value) if value.kind() == "null" => {
                thinkthen_complete_find_field_index_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_NULL_V1,
                    value: zero(),
                }
            }
            Some(value) => thinkthen_complete_find_field_index_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: value.number()?,
            },
        },
        last_line: match node.member("last_line") {
            None => thinkthen_complete_find_field_last_line_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_find_field_last_line_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: value.number()?,
            },
        },
        meta: {
            let value = node.required("meta")?;
            read_thinkthen_complete_meta_v1(value, store).map(|value| store.hold(value))?
        },
        position: match node.member("position") {
            None => thinkthen_complete_find_field_position_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_find_field_position_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_position_v1(value, store)
                    .map(|value| store.hold(value))?,
            },
        },
        question: {
            let value = node.required("question")?;
            read_thinkthen_complete_readable_question_2_v1(value, store)
                .map(|value| store.hold(value))?
        },
        schema: {
            let value = node.required("schema")?;
            read_thinkthen_complete_version_v1(value, store).map(|value| store.hold(value))?
        },
        threshold: match node.member("threshold") {
            None => return Err(ErrorKind::Defect),
            Some(value) if value.kind() == "null" => {
                thinkthen_complete_find_field_threshold_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_NULL_V1,
                    value: zero(),
                }
            }
            Some(value) => thinkthen_complete_find_field_threshold_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_json(value, store)?,
            },
        },
        value: match node.member("value") {
            None => return Err(ErrorKind::Defect),
            Some(value) if value.kind() == "null" => {
                thinkthen_complete_find_field_value_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_NULL_V1,
                    value: zero(),
                }
            }
            Some(value) => thinkthen_complete_find_field_value_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_json(value, store)?,
            },
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
pub struct thinkthen_complete_find_candidate_field_index_presence_v1 {
    pub presence: u32,
    pub value: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_find_candidate_field_input_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_json_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_find_candidate_field_source_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_physical_source_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_find_candidate_v1 {
    pub index: thinkthen_complete_find_candidate_field_index_presence_v1,
    pub input: thinkthen_complete_find_candidate_field_input_presence_v1,
    pub probability: f64,
    pub source: thinkthen_complete_find_candidate_field_source_presence_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_find_candidate_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_find_candidate_v1, ErrorKind> {
    Ok(thinkthen_complete_find_candidate_v1 {
        index: match node.member("index") {
            None => return Err(ErrorKind::Defect),
            Some(value) if value.kind() == "null" => {
                thinkthen_complete_find_candidate_field_index_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_NULL_V1,
                    value: zero(),
                }
            }
            Some(value) => thinkthen_complete_find_candidate_field_index_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: value.number()?,
            },
        },
        input: match node.member("input") {
            None => return Err(ErrorKind::Defect),
            Some(value) if value.kind() == "null" => {
                thinkthen_complete_find_candidate_field_input_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_NULL_V1,
                    value: zero(),
                }
            }
            Some(value) => thinkthen_complete_find_candidate_field_input_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_json(value, store)?,
            },
        },
        probability: {
            let value = node.required("probability")?;
            value.number()?
        },
        source: match node.member("source") {
            None => thinkthen_complete_find_candidate_field_source_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_find_candidate_field_source_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_physical_source_v1(value, store)
                    .map(|value| store.hold(value))?,
            },
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
            store.text(value.text()?)
        },
        height: {
            let value = node.required("height")?;
            value.number()?
        },
        media: {
            let value = node.required("media")?;
            read_thinkthen_complete_image_media_v1(value, store).map(|value| store.hold(value))?
        },
        width: {
            let value = node.required("width")?;
            value.number()?
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

pub const THINKTHEN_COMPLETE_INPUT_DECLARATION_STRING_V1: u32 = 1;

pub const THINKTHEN_COMPLETE_INPUT_DECLARATION_OBJECT_V1: u32 = 2;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_input_declaration_data_v1 {
    pub string: *const thinkthen_complete_input_declaration_string_v1,
    pub object: *const thinkthen_complete_input_declaration_object_v1,
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
        let value = read_thinkthen_complete_input_declaration_string_v1(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_input_declaration_v1 {
            kind: THINKTHEN_COMPLETE_INPUT_DECLARATION_STRING_V1,
            data: thinkthen_complete_input_declaration_data_v1 { string: value },
        });
    }
    if node.is_member("type", "object") {
        let value = read_thinkthen_complete_input_declaration_object_v1(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_input_declaration_v1 {
            kind: THINKTHEN_COMPLETE_INPUT_DECLARATION_OBJECT_V1,
            data: thinkthen_complete_input_declaration_data_v1 { object: value },
        });
    }
    Err(ErrorKind::Defect)
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_input_declaration_object_field_properties_entry_v1 {
    pub name: thinkthen_complete_utf8_v1,
    pub value: *const thinkthen_complete_input_property_type_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_input_declaration_object_field_properties_v1 {
    pub data: *const thinkthen_complete_input_declaration_object_field_properties_entry_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_input_declaration_object_field_properties_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_input_declaration_object_field_properties_v1, ErrorKind> {
    let mut values = Vec::new();
    for (key, item) in node.object()? {
        values.push(
            thinkthen_complete_input_declaration_object_field_properties_entry_v1 {
                name: store.text(key),
                value: read_thinkthen_complete_input_property_type_v1(item, store)
                    .map(|value| store.hold(value))?,
            },
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_input_declaration_object_field_properties_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_input_declaration_object_field_required_v1 {
    pub data: *const thinkthen_complete_utf8_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_input_declaration_object_field_required_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_input_declaration_object_field_required_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(store.text(item.text()?));
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_input_declaration_object_field_required_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_input_declaration_object_field_required_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_input_declaration_object_field_required_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_input_declaration_object_v1 {
    pub properties: thinkthen_complete_input_declaration_object_field_properties_v1,
    pub required: thinkthen_complete_input_declaration_object_field_required_presence_v1,
    pub r#type: *const thinkthen_complete_object_type_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_input_declaration_object_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_input_declaration_object_v1, ErrorKind> {
    Ok(thinkthen_complete_input_declaration_object_v1 {
        properties: {
            let value = node.required("properties")?;
            read_thinkthen_complete_input_declaration_object_field_properties_v1(value, store)?
        },
        required: match node.member("required") {
            None => thinkthen_complete_input_declaration_object_field_required_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_input_declaration_object_field_required_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_input_declaration_object_field_required_v1(
                    value, store,
                )?,
            },
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
pub struct thinkthen_complete_input_declaration_string_v1 {
    pub r#type: *const thinkthen_complete_string_type_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_input_declaration_string_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_input_declaration_string_v1, ErrorKind> {
    Ok(thinkthen_complete_input_declaration_string_v1 {
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
            store.text(value.text()?)
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
            store.text(value.text()?)
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
            store.text(value.text()?)
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
            store.text(value.text()?)
        },
        extensions: store.extensions(node, &["type"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_label_field_description_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_json_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_label_v1 {
    pub description: thinkthen_complete_label_field_description_presence_v1,
    pub name: thinkthen_complete_utf8_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_label_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_label_v1, ErrorKind> {
    Ok(thinkthen_complete_label_v1 {
        description: match node.member("description") {
            None => thinkthen_complete_label_field_description_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) if value.kind() == "null" => {
                thinkthen_complete_label_field_description_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_NULL_V1,
                    value: zero(),
                }
            }
            Some(value) => thinkthen_complete_label_field_description_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_json(value, store)?,
            },
        },
        name: {
            let value = node.required("name")?;
            store.text(value.text()?)
        },
        extensions: store.extensions(node, &["description", "name"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_meta_field_answered_by_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_utf8_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_meta_field_attempts_v1 {
    pub data: *const *const thinkthen_complete_attempt_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_meta_field_attempts_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_meta_field_attempts_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values
            .push(read_thinkthen_complete_attempt_v1(item, store).map(|value| store.hold(value))?);
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_meta_field_attempts_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_meta_field_attempts_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_meta_field_attempts_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_meta_field_batch_setting_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_batch_setting_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_meta_field_batch_warning_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_batch_warning_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_meta_field_context_sha_256_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_utf8_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_meta_field_observations_v1 {
    pub data: *const *const thinkthen_complete_observation_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_meta_field_observations_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_meta_field_observations_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_observation_v1(item, store).map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_meta_field_observations_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_meta_field_origin_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_origin_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_meta_field_profile_warning_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_profile_warning_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_meta_field_question_sha_256_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_utf8_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_meta_field_question_sources_v1 {
    pub data: *const *const thinkthen_complete_question_source_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_meta_field_question_sources_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_meta_field_question_sources_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_question_source_v1(item, store)
                .map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_meta_field_question_sources_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_meta_field_questions_sha_256_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_utf8_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_meta_field_requests_v1 {
    pub data: *const thinkthen_complete_utf8_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_meta_field_requests_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_meta_field_requests_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(store.text(item.text()?));
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_meta_field_requests_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_meta_field_usage_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_usage_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_meta_v1 {
    pub answered_by: thinkthen_complete_meta_field_answered_by_presence_v1,
    pub attempts: thinkthen_complete_meta_field_attempts_presence_v1,
    pub batch_setting: thinkthen_complete_meta_field_batch_setting_presence_v1,
    pub batch_warning: thinkthen_complete_meta_field_batch_warning_presence_v1,
    pub cached: u32,
    pub context_sha256: thinkthen_complete_meta_field_context_sha_256_presence_v1,
    pub failed_questions: u64,
    pub model: thinkthen_complete_utf8_v1,
    pub observations: thinkthen_complete_meta_field_observations_v1,
    pub origin: thinkthen_complete_meta_field_origin_presence_v1,
    pub profile_warning: thinkthen_complete_meta_field_profile_warning_presence_v1,
    pub question_sha256: thinkthen_complete_meta_field_question_sha_256_presence_v1,
    pub question_sources: thinkthen_complete_meta_field_question_sources_v1,
    pub questions_sha256: thinkthen_complete_meta_field_questions_sha_256_presence_v1,
    pub requests: thinkthen_complete_meta_field_requests_v1,
    pub requests_sent: u64,
    pub tool: thinkthen_complete_utf8_v1,
    pub url: thinkthen_complete_utf8_v1,
    pub usage: thinkthen_complete_meta_field_usage_presence_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_meta_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_meta_v1, ErrorKind> {
    Ok(thinkthen_complete_meta_v1 {
        answered_by: match node.member("answered_by") {
            None => thinkthen_complete_meta_field_answered_by_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_meta_field_answered_by_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: store.text(value.text()?),
            },
        },
        attempts: match node.member("attempts") {
            None => thinkthen_complete_meta_field_attempts_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_meta_field_attempts_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_meta_field_attempts_v1(value, store)?,
            },
        },
        batch_setting: match node.member("batch_setting") {
            None => thinkthen_complete_meta_field_batch_setting_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_meta_field_batch_setting_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_batch_setting_v1(value, store)
                    .map(|value| store.hold(value))?,
            },
        },
        batch_warning: match node.member("batch_warning") {
            None => thinkthen_complete_meta_field_batch_warning_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_meta_field_batch_warning_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_batch_warning_v1(value, store)
                    .map(|value| store.hold(value))?,
            },
        },
        cached: {
            let value = node.required("cached")?;
            value.boolean()?
        },
        context_sha256: match node.member("context_sha256") {
            None => thinkthen_complete_meta_field_context_sha_256_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_meta_field_context_sha_256_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: store.text(value.text()?),
            },
        },
        failed_questions: {
            let value = node.required("failed_questions")?;
            value.number()?
        },
        model: {
            let value = node.required("model")?;
            store.text(value.text()?)
        },
        observations: {
            let value = node.required("observations")?;
            read_thinkthen_complete_meta_field_observations_v1(value, store)?
        },
        origin: match node.member("origin") {
            None => return Err(ErrorKind::Defect),
            Some(value) if value.kind() == "null" => {
                thinkthen_complete_meta_field_origin_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_NULL_V1,
                    value: zero(),
                }
            }
            Some(value) => thinkthen_complete_meta_field_origin_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_origin_v1(value, store)
                    .map(|value| store.hold(value))?,
            },
        },
        profile_warning: match node.member("profile_warning") {
            None => thinkthen_complete_meta_field_profile_warning_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_meta_field_profile_warning_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_profile_warning_v1(value, store)
                    .map(|value| store.hold(value))?,
            },
        },
        question_sha256: match node.member("question_sha256") {
            None => thinkthen_complete_meta_field_question_sha_256_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_meta_field_question_sha_256_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: store.text(value.text()?),
            },
        },
        question_sources: {
            let value = node.required("question_sources")?;
            read_thinkthen_complete_meta_field_question_sources_v1(value, store)?
        },
        questions_sha256: match node.member("questions_sha256") {
            None => thinkthen_complete_meta_field_questions_sha_256_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_meta_field_questions_sha_256_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: store.text(value.text()?),
            },
        },
        requests: {
            let value = node.required("requests")?;
            read_thinkthen_complete_meta_field_requests_v1(value, store)?
        },
        requests_sent: {
            let value = node.required("requests_sent")?;
            value.number()?
        },
        tool: {
            let value = node.required("tool")?;
            store.text(value.text()?)
        },
        url: {
            let value = node.required("url")?;
            store.text(value.text()?)
        },
        usage: match node.member("usage") {
            None => thinkthen_complete_meta_field_usage_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_meta_field_usage_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_usage_v1(value, store)
                    .map(|value| store.hold(value))?,
            },
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
    let value = store.text(node.text()?);
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
pub struct thinkthen_complete_persistence_observation_field_advice_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_utf8_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_persistence_observation_v1 {
    pub advice: thinkthen_complete_persistence_observation_field_advice_presence_v1,
    pub observed_at: thinkthen_complete_utf8_v1,
    pub state: *const thinkthen_complete_usage_persistence_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_persistence_observation_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_persistence_observation_v1, ErrorKind> {
    Ok(thinkthen_complete_persistence_observation_v1 {
        advice: match node.member("advice") {
            None => thinkthen_complete_persistence_observation_field_advice_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_persistence_observation_field_advice_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: store.text(value.text()?),
            },
        },
        observed_at: {
            let value = node.required("observed_at")?;
            store.text(value.text()?)
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
pub struct thinkthen_complete_physical_source_field_first_line_presence_v1 {
    pub presence: u32,
    pub value: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_physical_source_field_last_line_presence_v1 {
    pub presence: u32,
    pub value: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_physical_source_v1 {
    pub file: thinkthen_complete_utf8_v1,
    pub first_line: thinkthen_complete_physical_source_field_first_line_presence_v1,
    pub last_line: thinkthen_complete_physical_source_field_last_line_presence_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_physical_source_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_physical_source_v1, ErrorKind> {
    Ok(thinkthen_complete_physical_source_v1 {
        file: {
            let value = node.required("file")?;
            store.text(value.text()?)
        },
        first_line: match node.member("first_line") {
            None => thinkthen_complete_physical_source_field_first_line_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_physical_source_field_first_line_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: value.number()?,
            },
        },
        last_line: match node.member("last_line") {
            None => thinkthen_complete_physical_source_field_last_line_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_physical_source_field_last_line_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: value.number()?,
            },
        },
        extensions: store.extensions(node, &["file", "first_line", "last_line"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_position_field_file_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_utf8_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_position_field_first_presence_v1 {
    pub presence: u32,
    pub value: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_position_field_images_v1 {
    pub data: *const thinkthen_complete_utf8_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_position_field_images_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_position_field_images_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(store.text(item.text()?));
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_position_field_images_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_position_field_images_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_position_field_images_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_position_field_last_presence_v1 {
    pub presence: u32,
    pub value: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_position_v1 {
    pub file: thinkthen_complete_position_field_file_presence_v1,
    pub first: thinkthen_complete_position_field_first_presence_v1,
    pub images: thinkthen_complete_position_field_images_presence_v1,
    pub last: thinkthen_complete_position_field_last_presence_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_position_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_position_v1, ErrorKind> {
    Ok(thinkthen_complete_position_v1 {
        file: match node.member("file") {
            None => return Err(ErrorKind::Defect),
            Some(value) if value.kind() == "null" => {
                thinkthen_complete_position_field_file_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_NULL_V1,
                    value: zero(),
                }
            }
            Some(value) => thinkthen_complete_position_field_file_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: store.text(value.text()?),
            },
        },
        first: match node.member("first") {
            None => thinkthen_complete_position_field_first_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_position_field_first_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: value.number()?,
            },
        },
        images: match node.member("images") {
            None => thinkthen_complete_position_field_images_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_position_field_images_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_position_field_images_v1(value, store)?,
            },
        },
        last: match node.member("last") {
            None => thinkthen_complete_position_field_last_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_position_field_last_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: value.number()?,
            },
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
    let value = store.text(node.text()?);
    Ok(thinkthen_complete_question_name_v1 { value })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_question_source_field_batch_size_presence_v1 {
    pub presence: u32,
    pub value: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_question_source_v1 {
    pub answered_by: thinkthen_complete_utf8_v1,
    pub batch_size: thinkthen_complete_question_source_field_batch_size_presence_v1,
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
            store.text(value.text()?)
        },
        batch_size: match node.member("batch_size") {
            None => thinkthen_complete_question_source_field_batch_size_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_question_source_field_batch_size_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: value.number()?,
            },
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
            store.text(value.text()?)
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
pub struct thinkthen_complete_rank_member_result_field_images_v1 {
    pub data: *const *const thinkthen_complete_image_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_rank_member_result_field_images_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_rank_member_result_field_images_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(read_thinkthen_complete_image_v1(item, store).map(|value| store.hold(value))?);
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_rank_member_result_field_images_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_rank_member_result_field_images_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_rank_member_result_field_images_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_rank_member_result_field_source_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_physical_source_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_rank_member_result_field_threshold_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_json_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_rank_member_result_v1 {
    pub answer: *const thinkthen_complete_answer_v1,
    pub answer_id: *const thinkthen_complete_answer_id_v1,
    pub images: thinkthen_complete_rank_member_result_field_images_presence_v1,
    pub meta: *const thinkthen_complete_meta_v1,
    pub question: *const thinkthen_complete_readable_question_v1,
    pub schema: *const thinkthen_complete_version_v1,
    pub source: thinkthen_complete_rank_member_result_field_source_presence_v1,
    pub threshold: thinkthen_complete_rank_member_result_field_threshold_presence_v1,
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
        images: match node.member("images") {
            None => thinkthen_complete_rank_member_result_field_images_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_rank_member_result_field_images_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_rank_member_result_field_images_v1(value, store)?,
            },
        },
        meta: {
            let value = node.required("meta")?;
            read_thinkthen_complete_meta_v1(value, store).map(|value| store.hold(value))?
        },
        question: {
            let value = node.required("question")?;
            read_thinkthen_complete_readable_question_v1(value, store)
                .map(|value| store.hold(value))?
        },
        schema: {
            let value = node.required("schema")?;
            read_thinkthen_complete_version_v1(value, store).map(|value| store.hold(value))?
        },
        source: match node.member("source") {
            None => thinkthen_complete_rank_member_result_field_source_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_rank_member_result_field_source_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_physical_source_v1(value, store)
                    .map(|value| store.hold(value))?,
            },
        },
        threshold: match node.member("threshold") {
            None => return Err(ErrorKind::Defect),
            Some(value) if value.kind() == "null" => {
                thinkthen_complete_rank_member_result_field_threshold_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_NULL_V1,
                    value: zero(),
                }
            }
            Some(value) => thinkthen_complete_rank_member_result_field_threshold_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_json(value, store)?,
            },
        },
        value: {
            let value = node.required("value")?;
            value.number()?
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

pub const THINKTHEN_COMPLETE_READABLE_QUESTION_DECIDE_V1: u32 = 1;

pub const THINKTHEN_COMPLETE_READABLE_QUESTION_CHOOSE_V1: u32 = 2;

pub const THINKTHEN_COMPLETE_READABLE_QUESTION_TAG_V1: u32 = 3;

pub const THINKTHEN_COMPLETE_READABLE_QUESTION_SCORE_V1: u32 = 4;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_readable_question_data_v1 {
    pub decide: *const thinkthen_complete_readable_question_decide_v1,
    pub choose: *const thinkthen_complete_readable_question_choose_v1,
    pub tag: *const thinkthen_complete_readable_question_tag_v1,
    pub score: *const thinkthen_complete_readable_question_score_v1,
}
impl std::fmt::Debug for thinkthen_complete_readable_question_data_v1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("union payload")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_v1 {
    pub kind: u32,
    pub data: thinkthen_complete_readable_question_data_v1,
}

pub(crate) fn read_thinkthen_complete_readable_question_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_readable_question_v1, ErrorKind> {
    if node.is_member("verb", "decide") {
        let value = read_thinkthen_complete_readable_question_decide_v1(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_readable_question_v1 {
            kind: THINKTHEN_COMPLETE_READABLE_QUESTION_DECIDE_V1,
            data: thinkthen_complete_readable_question_data_v1 { decide: value },
        });
    }
    if node.is_member("verb", "choose") {
        let value = read_thinkthen_complete_readable_question_choose_v1(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_readable_question_v1 {
            kind: THINKTHEN_COMPLETE_READABLE_QUESTION_CHOOSE_V1,
            data: thinkthen_complete_readable_question_data_v1 { choose: value },
        });
    }
    if node.is_member("verb", "tag") {
        let value = read_thinkthen_complete_readable_question_tag_v1(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_readable_question_v1 {
            kind: THINKTHEN_COMPLETE_READABLE_QUESTION_TAG_V1,
            data: thinkthen_complete_readable_question_data_v1 { tag: value },
        });
    }
    if node.is_member("verb", "score") {
        let value = read_thinkthen_complete_readable_question_score_v1(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_readable_question_v1 {
            kind: THINKTHEN_COMPLETE_READABLE_QUESTION_SCORE_V1,
            data: thinkthen_complete_readable_question_data_v1 { score: value },
        });
    }
    Err(ErrorKind::Defect)
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_2_field_batch_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_batch_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_2_field_context_schema_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_input_declaration_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_2_field_item_schema_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_input_declaration_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_2_field_label_details_v1 {
    pub data: *const *const thinkthen_complete_label_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_readable_question_2_field_label_details_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_readable_question_2_field_label_details_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(read_thinkthen_complete_label_v1(item, store).map(|value| store.hold(value))?);
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_readable_question_2_field_label_details_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_2_field_label_details_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_readable_question_2_field_label_details_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_2_field_model_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_utf8_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_2_field_name_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_question_name_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_2_field_on_v1 {
    pub data: *const thinkthen_complete_utf8_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_readable_question_2_field_on_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_readable_question_2_field_on_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(store.text(item.text()?));
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_readable_question_2_field_on_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_2_field_on_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_readable_question_2_field_on_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_2_field_profile_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_utf8_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_2_field_text_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_json_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_2_field_wording_version_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_wording_version_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_2_v1 {
    pub batch: thinkthen_complete_readable_question_2_field_batch_presence_v1,
    pub context_schema: thinkthen_complete_readable_question_2_field_context_schema_presence_v1,
    pub item_schema: thinkthen_complete_readable_question_2_field_item_schema_presence_v1,
    pub label_details: thinkthen_complete_readable_question_2_field_label_details_presence_v1,
    pub model: thinkthen_complete_readable_question_2_field_model_presence_v1,
    pub name: thinkthen_complete_readable_question_2_field_name_presence_v1,
    pub none: u32,
    pub on: thinkthen_complete_readable_question_2_field_on_presence_v1,
    pub profile: thinkthen_complete_readable_question_2_field_profile_presence_v1,
    pub text: thinkthen_complete_readable_question_2_field_text_presence_v1,
    pub verb: thinkthen_complete_utf8_v1,
    pub wording_version: thinkthen_complete_readable_question_2_field_wording_version_presence_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_readable_question_2_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_readable_question_2_v1, ErrorKind> {
    Ok(thinkthen_complete_readable_question_2_v1 {
        batch: match node.member("batch") {
            None => thinkthen_complete_readable_question_2_field_batch_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_readable_question_2_field_batch_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_batch_v1(value, store)
                    .map(|value| store.hold(value))?,
            },
        },
        context_schema: match node.member("context_schema") {
            None => thinkthen_complete_readable_question_2_field_context_schema_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => {
                thinkthen_complete_readable_question_2_field_context_schema_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                    value: read_thinkthen_complete_input_declaration_v1(value, store)
                        .map(|value| store.hold(value))?,
                }
            }
        },
        item_schema: match node.member("item_schema") {
            None => thinkthen_complete_readable_question_2_field_item_schema_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_readable_question_2_field_item_schema_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_input_declaration_v1(value, store)
                    .map(|value| store.hold(value))?,
            },
        },
        label_details: match node.member("label_details") {
            None => thinkthen_complete_readable_question_2_field_label_details_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_readable_question_2_field_label_details_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_readable_question_2_field_label_details_v1(
                    value, store,
                )?,
            },
        },
        model: match node.member("model") {
            None => thinkthen_complete_readable_question_2_field_model_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_readable_question_2_field_model_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: store.text(value.text()?),
            },
        },
        name: match node.member("name") {
            None => thinkthen_complete_readable_question_2_field_name_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_readable_question_2_field_name_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_question_name_v1(value, store)
                    .map(|value| store.hold(value))?,
            },
        },
        none: {
            let value = node.required("none")?;
            value.boolean()?
        },
        on: match node.member("on") {
            None => thinkthen_complete_readable_question_2_field_on_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_readable_question_2_field_on_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_readable_question_2_field_on_v1(value, store)?,
            },
        },
        profile: match node.member("profile") {
            None => thinkthen_complete_readable_question_2_field_profile_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_readable_question_2_field_profile_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: store.text(value.text()?),
            },
        },
        text: match node.member("text") {
            None => return Err(ErrorKind::Defect),
            Some(value) if value.kind() == "null" => {
                thinkthen_complete_readable_question_2_field_text_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_NULL_V1,
                    value: zero(),
                }
            }
            Some(value) => thinkthen_complete_readable_question_2_field_text_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_json(value, store)?,
            },
        },
        verb: {
            let value = node.required("verb")?;
            store.text(value.text()?)
        },
        wording_version: match node.member("wording_version") {
            None => thinkthen_complete_readable_question_2_field_wording_version_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => {
                thinkthen_complete_readable_question_2_field_wording_version_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                    value: read_thinkthen_complete_wording_version_v1(value, store)
                        .map(|value| store.hold(value))?,
                }
            }
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
pub struct thinkthen_complete_readable_question_3_field_batch_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_batch_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_3_field_context_schema_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_input_declaration_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_3_field_entity_definition_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_json_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_3_field_instructions_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_json_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_3_field_item_schema_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_input_declaration_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_3_field_kinds_entry_v1 {
    pub name: thinkthen_complete_utf8_v1,
    pub value: *const thinkthen_complete_json_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_3_field_kinds_v1 {
    pub data: *const thinkthen_complete_readable_question_3_field_kinds_entry_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_readable_question_3_field_kinds_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_readable_question_3_field_kinds_v1, ErrorKind> {
    let mut values = Vec::new();
    for (key, item) in node.object()? {
        values.push(
            thinkthen_complete_readable_question_3_field_kinds_entry_v1 {
                name: store.text(key),
                value: read_json(item, store)?,
            },
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_readable_question_3_field_kinds_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_3_field_label_details_v1 {
    pub data: *const *const thinkthen_complete_label_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_readable_question_3_field_label_details_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_readable_question_3_field_label_details_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(read_thinkthen_complete_label_v1(item, store).map(|value| store.hold(value))?);
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_readable_question_3_field_label_details_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_3_field_label_details_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_readable_question_3_field_label_details_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_3_field_mode_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_recognition_mode_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_3_field_model_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_utf8_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_3_field_name_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_question_name_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_3_field_on_v1 {
    pub data: *const thinkthen_complete_utf8_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_readable_question_3_field_on_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_readable_question_3_field_on_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(store.text(item.text()?));
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_readable_question_3_field_on_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_3_field_on_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_readable_question_3_field_on_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_3_field_profile_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_utf8_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_3_field_relation_threshold_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_threshold_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_3_field_relations_v1 {
    pub data: *const *const thinkthen_complete_relation_rule_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_readable_question_3_field_relations_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_readable_question_3_field_relations_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_relation_rule_v1(item, store).map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_readable_question_3_field_relations_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_3_field_relations_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_readable_question_3_field_relations_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_3_field_snippet_pieces_presence_v1 {
    pub presence: u32,
    pub value: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_3_field_stage_context_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_recognition_stage_context_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_3_field_wording_version_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_wording_version_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_3_v1 {
    pub batch: thinkthen_complete_readable_question_3_field_batch_presence_v1,
    pub context_schema: thinkthen_complete_readable_question_3_field_context_schema_presence_v1,
    pub entity_definition:
        thinkthen_complete_readable_question_3_field_entity_definition_presence_v1,
    pub instructions: thinkthen_complete_readable_question_3_field_instructions_presence_v1,
    pub item_schema: thinkthen_complete_readable_question_3_field_item_schema_presence_v1,
    pub kinds: thinkthen_complete_readable_question_3_field_kinds_v1,
    pub label_details: thinkthen_complete_readable_question_3_field_label_details_presence_v1,
    pub mode: thinkthen_complete_readable_question_3_field_mode_presence_v1,
    pub model: thinkthen_complete_readable_question_3_field_model_presence_v1,
    pub name: thinkthen_complete_readable_question_3_field_name_presence_v1,
    pub on: thinkthen_complete_readable_question_3_field_on_presence_v1,
    pub profile: thinkthen_complete_readable_question_3_field_profile_presence_v1,
    pub relation_threshold:
        thinkthen_complete_readable_question_3_field_relation_threshold_presence_v1,
    pub relations: thinkthen_complete_readable_question_3_field_relations_presence_v1,
    pub snippet_pieces: thinkthen_complete_readable_question_3_field_snippet_pieces_presence_v1,
    pub stage_context: thinkthen_complete_readable_question_3_field_stage_context_presence_v1,
    pub threshold: *const thinkthen_complete_threshold_v1,
    pub verb: *const thinkthen_complete_verb_v1,
    pub wording_version: thinkthen_complete_readable_question_3_field_wording_version_presence_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_readable_question_3_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_readable_question_3_v1, ErrorKind> {
    Ok(thinkthen_complete_readable_question_3_v1 {
        batch: match node.member("batch") {
            None => thinkthen_complete_readable_question_3_field_batch_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_readable_question_3_field_batch_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_batch_v1(value, store)
                    .map(|value| store.hold(value))?,
            },
        },
        context_schema: match node.member("context_schema") {
            None => thinkthen_complete_readable_question_3_field_context_schema_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => {
                thinkthen_complete_readable_question_3_field_context_schema_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                    value: read_thinkthen_complete_input_declaration_v1(value, store)
                        .map(|value| store.hold(value))?,
                }
            }
        },
        entity_definition: match node.member("entity_definition") {
            None => thinkthen_complete_readable_question_3_field_entity_definition_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) if value.kind() == "null" => {
                thinkthen_complete_readable_question_3_field_entity_definition_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_NULL_V1,
                    value: zero(),
                }
            }
            Some(value) => {
                thinkthen_complete_readable_question_3_field_entity_definition_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                    value: read_json(value, store)?,
                }
            }
        },
        instructions: match node.member("instructions") {
            None => thinkthen_complete_readable_question_3_field_instructions_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) if value.kind() == "null" => {
                thinkthen_complete_readable_question_3_field_instructions_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_NULL_V1,
                    value: zero(),
                }
            }
            Some(value) => thinkthen_complete_readable_question_3_field_instructions_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_json(value, store)?,
            },
        },
        item_schema: match node.member("item_schema") {
            None => thinkthen_complete_readable_question_3_field_item_schema_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_readable_question_3_field_item_schema_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_input_declaration_v1(value, store)
                    .map(|value| store.hold(value))?,
            },
        },
        kinds: {
            let value = node.required("kinds")?;
            read_thinkthen_complete_readable_question_3_field_kinds_v1(value, store)?
        },
        label_details: match node.member("label_details") {
            None => thinkthen_complete_readable_question_3_field_label_details_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_readable_question_3_field_label_details_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_readable_question_3_field_label_details_v1(
                    value, store,
                )?,
            },
        },
        mode: match node.member("mode") {
            None => thinkthen_complete_readable_question_3_field_mode_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_readable_question_3_field_mode_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_recognition_mode_v1(value, store)
                    .map(|value| store.hold(value))?,
            },
        },
        model: match node.member("model") {
            None => thinkthen_complete_readable_question_3_field_model_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_readable_question_3_field_model_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: store.text(value.text()?),
            },
        },
        name: match node.member("name") {
            None => thinkthen_complete_readable_question_3_field_name_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_readable_question_3_field_name_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_question_name_v1(value, store)
                    .map(|value| store.hold(value))?,
            },
        },
        on: match node.member("on") {
            None => thinkthen_complete_readable_question_3_field_on_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_readable_question_3_field_on_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_readable_question_3_field_on_v1(value, store)?,
            },
        },
        profile: match node.member("profile") {
            None => thinkthen_complete_readable_question_3_field_profile_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_readable_question_3_field_profile_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: store.text(value.text()?),
            },
        },
        relation_threshold: match node.member("relation_threshold") {
            None => thinkthen_complete_readable_question_3_field_relation_threshold_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => {
                thinkthen_complete_readable_question_3_field_relation_threshold_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                    value: read_thinkthen_complete_threshold_v1(value, store)
                        .map(|value| store.hold(value))?,
                }
            }
        },
        relations: match node.member("relations") {
            None => thinkthen_complete_readable_question_3_field_relations_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_readable_question_3_field_relations_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_readable_question_3_field_relations_v1(
                    value, store,
                )?,
            },
        },
        snippet_pieces: match node.member("snippet_pieces") {
            None => thinkthen_complete_readable_question_3_field_snippet_pieces_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => {
                thinkthen_complete_readable_question_3_field_snippet_pieces_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                    value: value.number()?,
                }
            }
        },
        stage_context: match node.member("stage_context") {
            None => thinkthen_complete_readable_question_3_field_stage_context_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_readable_question_3_field_stage_context_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_recognition_stage_context_v1(value, store)
                    .map(|value| store.hold(value))?,
            },
        },
        threshold: {
            let value = node.required("threshold")?;
            read_thinkthen_complete_threshold_v1(value, store).map(|value| store.hold(value))?
        },
        verb: {
            let value = node.required("verb")?;
            read_thinkthen_complete_verb_v1(value, store).map(|value| store.hold(value))?
        },
        wording_version: match node.member("wording_version") {
            None => thinkthen_complete_readable_question_3_field_wording_version_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => {
                thinkthen_complete_readable_question_3_field_wording_version_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                    value: read_thinkthen_complete_wording_version_v1(value, store)
                        .map(|value| store.hold(value))?,
                }
            }
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
pub struct thinkthen_complete_readable_question_4_field_batch_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_batch_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_4_field_context_schema_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_input_declaration_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_4_field_fields_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_relate_fields_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_4_field_item_schema_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_input_declaration_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_4_field_label_details_v1 {
    pub data: *const *const thinkthen_complete_label_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_readable_question_4_field_label_details_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_readable_question_4_field_label_details_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(read_thinkthen_complete_label_v1(item, store).map(|value| store.hold(value))?);
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_readable_question_4_field_label_details_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_4_field_label_details_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_readable_question_4_field_label_details_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_4_field_model_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_utf8_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_4_field_name_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_question_name_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_4_field_on_v1 {
    pub data: *const thinkthen_complete_utf8_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_readable_question_4_field_on_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_readable_question_4_field_on_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(store.text(item.text()?));
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_readable_question_4_field_on_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_4_field_on_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_readable_question_4_field_on_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_4_field_profile_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_utf8_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_4_field_relations_v1 {
    pub data: *const *const thinkthen_complete_relation_rule_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_readable_question_4_field_relations_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_readable_question_4_field_relations_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_relation_rule_v1(item, store).map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_readable_question_4_field_relations_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_4_field_wording_version_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_wording_version_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_4_v1 {
    pub batch: thinkthen_complete_readable_question_4_field_batch_presence_v1,
    pub context_schema: thinkthen_complete_readable_question_4_field_context_schema_presence_v1,
    pub fields: thinkthen_complete_readable_question_4_field_fields_presence_v1,
    pub item_schema: thinkthen_complete_readable_question_4_field_item_schema_presence_v1,
    pub label_details: thinkthen_complete_readable_question_4_field_label_details_presence_v1,
    pub model: thinkthen_complete_readable_question_4_field_model_presence_v1,
    pub name: thinkthen_complete_readable_question_4_field_name_presence_v1,
    pub on: thinkthen_complete_readable_question_4_field_on_presence_v1,
    pub profile: thinkthen_complete_readable_question_4_field_profile_presence_v1,
    pub relations: thinkthen_complete_readable_question_4_field_relations_v1,
    pub threshold: *const thinkthen_complete_threshold_v1,
    pub verb: thinkthen_complete_utf8_v1,
    pub wording_version: thinkthen_complete_readable_question_4_field_wording_version_presence_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_readable_question_4_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_readable_question_4_v1, ErrorKind> {
    Ok(thinkthen_complete_readable_question_4_v1 {
        batch: match node.member("batch") {
            None => thinkthen_complete_readable_question_4_field_batch_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_readable_question_4_field_batch_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_batch_v1(value, store)
                    .map(|value| store.hold(value))?,
            },
        },
        context_schema: match node.member("context_schema") {
            None => thinkthen_complete_readable_question_4_field_context_schema_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => {
                thinkthen_complete_readable_question_4_field_context_schema_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                    value: read_thinkthen_complete_input_declaration_v1(value, store)
                        .map(|value| store.hold(value))?,
                }
            }
        },
        fields: match node.member("fields") {
            None => return Err(ErrorKind::Defect),
            Some(value) if value.kind() == "null" => {
                thinkthen_complete_readable_question_4_field_fields_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_NULL_V1,
                    value: zero(),
                }
            }
            Some(value) => thinkthen_complete_readable_question_4_field_fields_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_relate_fields_v1(value, store)
                    .map(|value| store.hold(value))?,
            },
        },
        item_schema: match node.member("item_schema") {
            None => thinkthen_complete_readable_question_4_field_item_schema_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_readable_question_4_field_item_schema_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_input_declaration_v1(value, store)
                    .map(|value| store.hold(value))?,
            },
        },
        label_details: match node.member("label_details") {
            None => thinkthen_complete_readable_question_4_field_label_details_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_readable_question_4_field_label_details_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_readable_question_4_field_label_details_v1(
                    value, store,
                )?,
            },
        },
        model: match node.member("model") {
            None => thinkthen_complete_readable_question_4_field_model_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_readable_question_4_field_model_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: store.text(value.text()?),
            },
        },
        name: match node.member("name") {
            None => thinkthen_complete_readable_question_4_field_name_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_readable_question_4_field_name_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_question_name_v1(value, store)
                    .map(|value| store.hold(value))?,
            },
        },
        on: match node.member("on") {
            None => thinkthen_complete_readable_question_4_field_on_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_readable_question_4_field_on_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_readable_question_4_field_on_v1(value, store)?,
            },
        },
        profile: match node.member("profile") {
            None => thinkthen_complete_readable_question_4_field_profile_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_readable_question_4_field_profile_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: store.text(value.text()?),
            },
        },
        relations: {
            let value = node.required("relations")?;
            read_thinkthen_complete_readable_question_4_field_relations_v1(value, store)?
        },
        threshold: {
            let value = node.required("threshold")?;
            read_thinkthen_complete_threshold_v1(value, store).map(|value| store.hold(value))?
        },
        verb: {
            let value = node.required("verb")?;
            store.text(value.text()?)
        },
        wording_version: match node.member("wording_version") {
            None => thinkthen_complete_readable_question_4_field_wording_version_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => {
                thinkthen_complete_readable_question_4_field_wording_version_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                    value: read_thinkthen_complete_wording_version_v1(value, store)
                        .map(|value| store.hold(value))?,
                }
            }
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
pub struct thinkthen_complete_readable_question_choose_field_batch_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_batch_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_choose_field_context_schema_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_input_declaration_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_choose_field_item_schema_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_input_declaration_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_choose_field_label_details_v1 {
    pub data: *const *const thinkthen_complete_label_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_readable_question_choose_field_label_details_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_readable_question_choose_field_label_details_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(read_thinkthen_complete_label_v1(item, store).map(|value| store.hold(value))?);
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_readable_question_choose_field_label_details_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_choose_field_label_details_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_readable_question_choose_field_label_details_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_choose_field_model_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_utf8_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_choose_field_name_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_question_name_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_choose_field_on_v1 {
    pub data: *const thinkthen_complete_utf8_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_readable_question_choose_field_on_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_readable_question_choose_field_on_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(store.text(item.text()?));
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_readable_question_choose_field_on_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_choose_field_on_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_readable_question_choose_field_on_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_choose_field_profile_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_utf8_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_choose_field_wording_version_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_wording_version_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_choose_field_options_v1 {
    pub data: *const thinkthen_complete_utf8_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_readable_question_choose_field_options_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_readable_question_choose_field_options_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(store.text(item.text()?));
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_readable_question_choose_field_options_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_choose_field_text_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_json_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_choose_v1 {
    pub batch: thinkthen_complete_readable_question_choose_field_batch_presence_v1,
    pub context_schema:
        thinkthen_complete_readable_question_choose_field_context_schema_presence_v1,
    pub item_schema: thinkthen_complete_readable_question_choose_field_item_schema_presence_v1,
    pub label_details: thinkthen_complete_readable_question_choose_field_label_details_presence_v1,
    pub model: thinkthen_complete_readable_question_choose_field_model_presence_v1,
    pub name: thinkthen_complete_readable_question_choose_field_name_presence_v1,
    pub on: thinkthen_complete_readable_question_choose_field_on_presence_v1,
    pub profile: thinkthen_complete_readable_question_choose_field_profile_presence_v1,
    pub wording_version:
        thinkthen_complete_readable_question_choose_field_wording_version_presence_v1,
    pub options: thinkthen_complete_readable_question_choose_field_options_v1,
    pub text: thinkthen_complete_readable_question_choose_field_text_presence_v1,
    pub verb: thinkthen_complete_utf8_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_readable_question_choose_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_readable_question_choose_v1, ErrorKind> {
    Ok(thinkthen_complete_readable_question_choose_v1 {
        batch: match node.member("batch") {
            None => thinkthen_complete_readable_question_choose_field_batch_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_readable_question_choose_field_batch_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_batch_v1(value, store)
                    .map(|value| store.hold(value))?,
            },
        },
        context_schema: match node.member("context_schema") {
            None => thinkthen_complete_readable_question_choose_field_context_schema_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => {
                thinkthen_complete_readable_question_choose_field_context_schema_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                    value: read_thinkthen_complete_input_declaration_v1(value, store)
                        .map(|value| store.hold(value))?,
                }
            }
        },
        item_schema: match node.member("item_schema") {
            None => thinkthen_complete_readable_question_choose_field_item_schema_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => {
                thinkthen_complete_readable_question_choose_field_item_schema_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                    value: read_thinkthen_complete_input_declaration_v1(value, store)
                        .map(|value| store.hold(value))?,
                }
            }
        },
        label_details: match node.member("label_details") {
            None => thinkthen_complete_readable_question_choose_field_label_details_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => {
                thinkthen_complete_readable_question_choose_field_label_details_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                    value: read_thinkthen_complete_readable_question_choose_field_label_details_v1(
                        value, store,
                    )?,
                }
            }
        },
        model: match node.member("model") {
            None => thinkthen_complete_readable_question_choose_field_model_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_readable_question_choose_field_model_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: store.text(value.text()?),
            },
        },
        name: match node.member("name") {
            None => thinkthen_complete_readable_question_choose_field_name_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_readable_question_choose_field_name_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_question_name_v1(value, store)
                    .map(|value| store.hold(value))?,
            },
        },
        on: match node.member("on") {
            None => thinkthen_complete_readable_question_choose_field_on_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_readable_question_choose_field_on_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_readable_question_choose_field_on_v1(value, store)?,
            },
        },
        profile: match node.member("profile") {
            None => thinkthen_complete_readable_question_choose_field_profile_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_readable_question_choose_field_profile_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: store.text(value.text()?),
            },
        },
        wording_version: match node.member("wording_version") {
            None => thinkthen_complete_readable_question_choose_field_wording_version_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => {
                thinkthen_complete_readable_question_choose_field_wording_version_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                    value: read_thinkthen_complete_wording_version_v1(value, store)
                        .map(|value| store.hold(value))?,
                }
            }
        },
        options: {
            let value = node.required("options")?;
            read_thinkthen_complete_readable_question_choose_field_options_v1(value, store)?
        },
        text: match node.member("text") {
            None => return Err(ErrorKind::Defect),
            Some(value) if value.kind() == "null" => {
                thinkthen_complete_readable_question_choose_field_text_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_NULL_V1,
                    value: zero(),
                }
            }
            Some(value) => thinkthen_complete_readable_question_choose_field_text_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_json(value, store)?,
            },
        },
        verb: {
            let value = node.required("verb")?;
            store.text(value.text()?)
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
pub struct thinkthen_complete_readable_question_decide_field_batch_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_batch_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_decide_field_context_schema_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_input_declaration_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_decide_field_item_schema_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_input_declaration_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_decide_field_label_details_v1 {
    pub data: *const *const thinkthen_complete_label_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_readable_question_decide_field_label_details_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_readable_question_decide_field_label_details_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(read_thinkthen_complete_label_v1(item, store).map(|value| store.hold(value))?);
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_readable_question_decide_field_label_details_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_decide_field_label_details_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_readable_question_decide_field_label_details_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_decide_field_model_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_utf8_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_decide_field_name_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_question_name_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_decide_field_on_v1 {
    pub data: *const thinkthen_complete_utf8_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_readable_question_decide_field_on_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_readable_question_decide_field_on_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(store.text(item.text()?));
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_readable_question_decide_field_on_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_decide_field_on_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_readable_question_decide_field_on_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_decide_field_profile_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_utf8_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_decide_field_wording_version_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_wording_version_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_decide_field_false_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_json_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_decide_field_text_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_json_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_decide_field_true_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_json_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_decide_v1 {
    pub batch: thinkthen_complete_readable_question_decide_field_batch_presence_v1,
    pub context_schema:
        thinkthen_complete_readable_question_decide_field_context_schema_presence_v1,
    pub item_schema: thinkthen_complete_readable_question_decide_field_item_schema_presence_v1,
    pub label_details: thinkthen_complete_readable_question_decide_field_label_details_presence_v1,
    pub model: thinkthen_complete_readable_question_decide_field_model_presence_v1,
    pub name: thinkthen_complete_readable_question_decide_field_name_presence_v1,
    pub on: thinkthen_complete_readable_question_decide_field_on_presence_v1,
    pub profile: thinkthen_complete_readable_question_decide_field_profile_presence_v1,
    pub wording_version:
        thinkthen_complete_readable_question_decide_field_wording_version_presence_v1,
    pub false_: thinkthen_complete_readable_question_decide_field_false_presence_v1,
    pub text: thinkthen_complete_readable_question_decide_field_text_presence_v1,
    pub true_: thinkthen_complete_readable_question_decide_field_true_presence_v1,
    pub verb: thinkthen_complete_utf8_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_readable_question_decide_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_readable_question_decide_v1, ErrorKind> {
    Ok(thinkthen_complete_readable_question_decide_v1 {
        batch: match node.member("batch") {
            None => thinkthen_complete_readable_question_decide_field_batch_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_readable_question_decide_field_batch_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_batch_v1(value, store)
                    .map(|value| store.hold(value))?,
            },
        },
        context_schema: match node.member("context_schema") {
            None => thinkthen_complete_readable_question_decide_field_context_schema_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => {
                thinkthen_complete_readable_question_decide_field_context_schema_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                    value: read_thinkthen_complete_input_declaration_v1(value, store)
                        .map(|value| store.hold(value))?,
                }
            }
        },
        item_schema: match node.member("item_schema") {
            None => thinkthen_complete_readable_question_decide_field_item_schema_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => {
                thinkthen_complete_readable_question_decide_field_item_schema_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                    value: read_thinkthen_complete_input_declaration_v1(value, store)
                        .map(|value| store.hold(value))?,
                }
            }
        },
        label_details: match node.member("label_details") {
            None => thinkthen_complete_readable_question_decide_field_label_details_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => {
                thinkthen_complete_readable_question_decide_field_label_details_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                    value: read_thinkthen_complete_readable_question_decide_field_label_details_v1(
                        value, store,
                    )?,
                }
            }
        },
        model: match node.member("model") {
            None => thinkthen_complete_readable_question_decide_field_model_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_readable_question_decide_field_model_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: store.text(value.text()?),
            },
        },
        name: match node.member("name") {
            None => thinkthen_complete_readable_question_decide_field_name_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_readable_question_decide_field_name_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_question_name_v1(value, store)
                    .map(|value| store.hold(value))?,
            },
        },
        on: match node.member("on") {
            None => thinkthen_complete_readable_question_decide_field_on_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_readable_question_decide_field_on_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_readable_question_decide_field_on_v1(value, store)?,
            },
        },
        profile: match node.member("profile") {
            None => thinkthen_complete_readable_question_decide_field_profile_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_readable_question_decide_field_profile_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: store.text(value.text()?),
            },
        },
        wording_version: match node.member("wording_version") {
            None => thinkthen_complete_readable_question_decide_field_wording_version_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => {
                thinkthen_complete_readable_question_decide_field_wording_version_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                    value: read_thinkthen_complete_wording_version_v1(value, store)
                        .map(|value| store.hold(value))?,
                }
            }
        },
        false_: match node.member("false") {
            None => thinkthen_complete_readable_question_decide_field_false_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) if value.kind() == "null" => {
                thinkthen_complete_readable_question_decide_field_false_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_NULL_V1,
                    value: zero(),
                }
            }
            Some(value) => thinkthen_complete_readable_question_decide_field_false_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_json(value, store)?,
            },
        },
        text: match node.member("text") {
            None => return Err(ErrorKind::Defect),
            Some(value) if value.kind() == "null" => {
                thinkthen_complete_readable_question_decide_field_text_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_NULL_V1,
                    value: zero(),
                }
            }
            Some(value) => thinkthen_complete_readable_question_decide_field_text_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_json(value, store)?,
            },
        },
        true_: match node.member("true") {
            None => thinkthen_complete_readable_question_decide_field_true_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) if value.kind() == "null" => {
                thinkthen_complete_readable_question_decide_field_true_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_NULL_V1,
                    value: zero(),
                }
            }
            Some(value) => thinkthen_complete_readable_question_decide_field_true_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_json(value, store)?,
            },
        },
        verb: {
            let value = node.required("verb")?;
            store.text(value.text()?)
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
pub struct thinkthen_complete_readable_question_score_field_batch_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_batch_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_score_field_context_schema_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_input_declaration_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_score_field_item_schema_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_input_declaration_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_score_field_label_details_v1 {
    pub data: *const *const thinkthen_complete_label_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_readable_question_score_field_label_details_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_readable_question_score_field_label_details_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(read_thinkthen_complete_label_v1(item, store).map(|value| store.hold(value))?);
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_readable_question_score_field_label_details_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_score_field_label_details_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_readable_question_score_field_label_details_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_score_field_model_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_utf8_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_score_field_name_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_question_name_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_score_field_on_v1 {
    pub data: *const thinkthen_complete_utf8_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_readable_question_score_field_on_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_readable_question_score_field_on_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(store.text(item.text()?));
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_readable_question_score_field_on_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_score_field_on_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_readable_question_score_field_on_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_score_field_profile_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_utf8_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_score_field_wording_version_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_wording_version_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_score_field_levels_v1 {
    pub data: *const thinkthen_complete_utf8_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_readable_question_score_field_levels_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_readable_question_score_field_levels_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(store.text(item.text()?));
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_readable_question_score_field_levels_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_score_field_text_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_json_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_score_v1 {
    pub batch: thinkthen_complete_readable_question_score_field_batch_presence_v1,
    pub context_schema: thinkthen_complete_readable_question_score_field_context_schema_presence_v1,
    pub item_schema: thinkthen_complete_readable_question_score_field_item_schema_presence_v1,
    pub label_details: thinkthen_complete_readable_question_score_field_label_details_presence_v1,
    pub model: thinkthen_complete_readable_question_score_field_model_presence_v1,
    pub name: thinkthen_complete_readable_question_score_field_name_presence_v1,
    pub on: thinkthen_complete_readable_question_score_field_on_presence_v1,
    pub profile: thinkthen_complete_readable_question_score_field_profile_presence_v1,
    pub wording_version:
        thinkthen_complete_readable_question_score_field_wording_version_presence_v1,
    pub levels: thinkthen_complete_readable_question_score_field_levels_v1,
    pub text: thinkthen_complete_readable_question_score_field_text_presence_v1,
    pub verb: thinkthen_complete_utf8_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_readable_question_score_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_readable_question_score_v1, ErrorKind> {
    Ok(thinkthen_complete_readable_question_score_v1 {
        batch: match node.member("batch") {
            None => thinkthen_complete_readable_question_score_field_batch_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_readable_question_score_field_batch_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_batch_v1(value, store)
                    .map(|value| store.hold(value))?,
            },
        },
        context_schema: match node.member("context_schema") {
            None => thinkthen_complete_readable_question_score_field_context_schema_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => {
                thinkthen_complete_readable_question_score_field_context_schema_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                    value: read_thinkthen_complete_input_declaration_v1(value, store)
                        .map(|value| store.hold(value))?,
                }
            }
        },
        item_schema: match node.member("item_schema") {
            None => thinkthen_complete_readable_question_score_field_item_schema_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => {
                thinkthen_complete_readable_question_score_field_item_schema_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                    value: read_thinkthen_complete_input_declaration_v1(value, store)
                        .map(|value| store.hold(value))?,
                }
            }
        },
        label_details: match node.member("label_details") {
            None => thinkthen_complete_readable_question_score_field_label_details_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => {
                thinkthen_complete_readable_question_score_field_label_details_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                    value: read_thinkthen_complete_readable_question_score_field_label_details_v1(
                        value, store,
                    )?,
                }
            }
        },
        model: match node.member("model") {
            None => thinkthen_complete_readable_question_score_field_model_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_readable_question_score_field_model_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: store.text(value.text()?),
            },
        },
        name: match node.member("name") {
            None => thinkthen_complete_readable_question_score_field_name_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_readable_question_score_field_name_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_question_name_v1(value, store)
                    .map(|value| store.hold(value))?,
            },
        },
        on: match node.member("on") {
            None => thinkthen_complete_readable_question_score_field_on_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_readable_question_score_field_on_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_readable_question_score_field_on_v1(value, store)?,
            },
        },
        profile: match node.member("profile") {
            None => thinkthen_complete_readable_question_score_field_profile_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_readable_question_score_field_profile_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: store.text(value.text()?),
            },
        },
        wording_version: match node.member("wording_version") {
            None => thinkthen_complete_readable_question_score_field_wording_version_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => {
                thinkthen_complete_readable_question_score_field_wording_version_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                    value: read_thinkthen_complete_wording_version_v1(value, store)
                        .map(|value| store.hold(value))?,
                }
            }
        },
        levels: {
            let value = node.required("levels")?;
            read_thinkthen_complete_readable_question_score_field_levels_v1(value, store)?
        },
        text: match node.member("text") {
            None => return Err(ErrorKind::Defect),
            Some(value) if value.kind() == "null" => {
                thinkthen_complete_readable_question_score_field_text_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_NULL_V1,
                    value: zero(),
                }
            }
            Some(value) => thinkthen_complete_readable_question_score_field_text_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_json(value, store)?,
            },
        },
        verb: {
            let value = node.required("verb")?;
            store.text(value.text()?)
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
pub struct thinkthen_complete_readable_question_tag_field_batch_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_batch_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_tag_field_context_schema_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_input_declaration_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_tag_field_item_schema_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_input_declaration_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_tag_field_label_details_v1 {
    pub data: *const *const thinkthen_complete_label_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_readable_question_tag_field_label_details_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_readable_question_tag_field_label_details_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(read_thinkthen_complete_label_v1(item, store).map(|value| store.hold(value))?);
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_readable_question_tag_field_label_details_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_tag_field_label_details_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_readable_question_tag_field_label_details_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_tag_field_model_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_utf8_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_tag_field_name_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_question_name_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_tag_field_on_v1 {
    pub data: *const thinkthen_complete_utf8_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_readable_question_tag_field_on_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_readable_question_tag_field_on_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(store.text(item.text()?));
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_readable_question_tag_field_on_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_tag_field_on_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_readable_question_tag_field_on_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_tag_field_profile_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_utf8_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_tag_field_wording_version_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_wording_version_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_tag_field_labels_v1 {
    pub data: *const thinkthen_complete_utf8_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_readable_question_tag_field_labels_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_readable_question_tag_field_labels_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(store.text(item.text()?));
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_readable_question_tag_field_labels_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_tag_field_text_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_json_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_readable_question_tag_v1 {
    pub batch: thinkthen_complete_readable_question_tag_field_batch_presence_v1,
    pub context_schema: thinkthen_complete_readable_question_tag_field_context_schema_presence_v1,
    pub item_schema: thinkthen_complete_readable_question_tag_field_item_schema_presence_v1,
    pub label_details: thinkthen_complete_readable_question_tag_field_label_details_presence_v1,
    pub model: thinkthen_complete_readable_question_tag_field_model_presence_v1,
    pub name: thinkthen_complete_readable_question_tag_field_name_presence_v1,
    pub on: thinkthen_complete_readable_question_tag_field_on_presence_v1,
    pub profile: thinkthen_complete_readable_question_tag_field_profile_presence_v1,
    pub wording_version: thinkthen_complete_readable_question_tag_field_wording_version_presence_v1,
    pub labels: thinkthen_complete_readable_question_tag_field_labels_v1,
    pub text: thinkthen_complete_readable_question_tag_field_text_presence_v1,
    pub verb: thinkthen_complete_utf8_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_readable_question_tag_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_readable_question_tag_v1, ErrorKind> {
    Ok(thinkthen_complete_readable_question_tag_v1 {
        batch: match node.member("batch") {
            None => thinkthen_complete_readable_question_tag_field_batch_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_readable_question_tag_field_batch_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_batch_v1(value, store)
                    .map(|value| store.hold(value))?,
            },
        },
        context_schema: match node.member("context_schema") {
            None => thinkthen_complete_readable_question_tag_field_context_schema_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => {
                thinkthen_complete_readable_question_tag_field_context_schema_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                    value: read_thinkthen_complete_input_declaration_v1(value, store)
                        .map(|value| store.hold(value))?,
                }
            }
        },
        item_schema: match node.member("item_schema") {
            None => thinkthen_complete_readable_question_tag_field_item_schema_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_readable_question_tag_field_item_schema_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_input_declaration_v1(value, store)
                    .map(|value| store.hold(value))?,
            },
        },
        label_details: match node.member("label_details") {
            None => thinkthen_complete_readable_question_tag_field_label_details_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => {
                thinkthen_complete_readable_question_tag_field_label_details_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                    value: read_thinkthen_complete_readable_question_tag_field_label_details_v1(
                        value, store,
                    )?,
                }
            }
        },
        model: match node.member("model") {
            None => thinkthen_complete_readable_question_tag_field_model_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_readable_question_tag_field_model_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: store.text(value.text()?),
            },
        },
        name: match node.member("name") {
            None => thinkthen_complete_readable_question_tag_field_name_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_readable_question_tag_field_name_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_question_name_v1(value, store)
                    .map(|value| store.hold(value))?,
            },
        },
        on: match node.member("on") {
            None => thinkthen_complete_readable_question_tag_field_on_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_readable_question_tag_field_on_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_readable_question_tag_field_on_v1(value, store)?,
            },
        },
        profile: match node.member("profile") {
            None => thinkthen_complete_readable_question_tag_field_profile_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_readable_question_tag_field_profile_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: store.text(value.text()?),
            },
        },
        wording_version: match node.member("wording_version") {
            None => thinkthen_complete_readable_question_tag_field_wording_version_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => {
                thinkthen_complete_readable_question_tag_field_wording_version_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                    value: read_thinkthen_complete_wording_version_v1(value, store)
                        .map(|value| store.hold(value))?,
                }
            }
        },
        labels: {
            let value = node.required("labels")?;
            read_thinkthen_complete_readable_question_tag_field_labels_v1(value, store)?
        },
        text: match node.member("text") {
            None => return Err(ErrorKind::Defect),
            Some(value) if value.kind() == "null" => {
                thinkthen_complete_readable_question_tag_field_text_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_NULL_V1,
                    value: zero(),
                }
            }
            Some(value) => thinkthen_complete_readable_question_tag_field_text_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_json(value, store)?,
            },
        },
        verb: {
            let value = node.required("verb")?;
            store.text(value.text()?)
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

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_recognition_field_file_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_utf8_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_recognition_field_first_line_presence_v1 {
    pub presence: u32,
    pub value: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_recognition_field_index_presence_v1 {
    pub presence: u32,
    pub value: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_recognition_field_input_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_json_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_recognition_field_last_line_presence_v1 {
    pub presence: u32,
    pub value: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_recognition_field_position_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_position_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_recognition_field_source_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_physical_source_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_recognition_v1 {
    pub answer: *const thinkthen_complete_recognition_odds_v1,
    pub answer_id: *const thinkthen_complete_answer_id_v1,
    pub file: thinkthen_complete_recognition_field_file_presence_v1,
    pub first_line: thinkthen_complete_recognition_field_first_line_presence_v1,
    pub index: thinkthen_complete_recognition_field_index_presence_v1,
    pub input: thinkthen_complete_recognition_field_input_presence_v1,
    pub last_line: thinkthen_complete_recognition_field_last_line_presence_v1,
    pub meta: *const thinkthen_complete_meta_v1,
    pub position: thinkthen_complete_recognition_field_position_presence_v1,
    pub question: *const thinkthen_complete_readable_question_3_v1,
    pub schema: *const thinkthen_complete_version_v1,
    pub source: thinkthen_complete_recognition_field_source_presence_v1,
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
        file: match node.member("file") {
            None => thinkthen_complete_recognition_field_file_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_recognition_field_file_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: store.text(value.text()?),
            },
        },
        first_line: match node.member("first_line") {
            None => thinkthen_complete_recognition_field_first_line_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_recognition_field_first_line_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: value.number()?,
            },
        },
        index: match node.member("index") {
            None => thinkthen_complete_recognition_field_index_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_recognition_field_index_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: value.number()?,
            },
        },
        input: match node.member("input") {
            None => thinkthen_complete_recognition_field_input_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) if value.kind() == "null" => {
                thinkthen_complete_recognition_field_input_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_NULL_V1,
                    value: zero(),
                }
            }
            Some(value) => thinkthen_complete_recognition_field_input_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_json(value, store)?,
            },
        },
        last_line: match node.member("last_line") {
            None => thinkthen_complete_recognition_field_last_line_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_recognition_field_last_line_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: value.number()?,
            },
        },
        meta: {
            let value = node.required("meta")?;
            read_thinkthen_complete_meta_v1(value, store).map(|value| store.hold(value))?
        },
        position: match node.member("position") {
            None => thinkthen_complete_recognition_field_position_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_recognition_field_position_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_position_v1(value, store)
                    .map(|value| store.hold(value))?,
            },
        },
        question: {
            let value = node.required("question")?;
            read_thinkthen_complete_readable_question_3_v1(value, store)
                .map(|value| store.hold(value))?
        },
        schema: {
            let value = node.required("schema")?;
            read_thinkthen_complete_version_v1(value, store).map(|value| store.hold(value))?
        },
        source: match node.member("source") {
            None => thinkthen_complete_recognition_field_source_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_recognition_field_source_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_physical_source_v1(value, store)
                    .map(|value| store.hold(value))?,
            },
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
pub struct thinkthen_complete_recognition_edge_document_v1 {
    pub either: u32,
    pub probability: f64,
    pub relation: thinkthen_complete_utf8_v1,
    pub source: *const thinkthen_complete_entity_v1,
    pub target: *const thinkthen_complete_entity_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_recognition_edge_document_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_recognition_edge_document_v1, ErrorKind> {
    Ok(thinkthen_complete_recognition_edge_document_v1 {
        either: {
            let value = node.required("either")?;
            value.boolean()?
        },
        probability: {
            let value = node.required("probability")?;
            value.number()?
        },
        relation: {
            let value = node.required("relation")?;
            store.text(value.text()?)
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

pub const THINKTHEN_COMPLETE_RECOGNITION_ODDS_FIELDS_NAMES_PAIRS_PIECES_PROPOSALS_V1: u32 = 1;

pub const THINKTHEN_COMPLETE_RECOGNITION_ODDS_FIELDS_PIECES_PROPOSALS_V1: u32 = 2;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_recognition_odds_data_v1 {
    pub fields_names_pairs_pieces_proposals:
        *const thinkthen_complete_recognition_odds_fields_names_pairs_pieces_proposals_v1,
    pub fields_pieces_proposals:
        *const thinkthen_complete_recognition_odds_fields_pieces_proposals_v1,
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
        let value =
            read_thinkthen_complete_recognition_odds_fields_names_pairs_pieces_proposals_v1(
                node, store,
            )
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_recognition_odds_v1 {
            kind: THINKTHEN_COMPLETE_RECOGNITION_ODDS_FIELDS_NAMES_PAIRS_PIECES_PROPOSALS_V1,
            data: thinkthen_complete_recognition_odds_data_v1 {
                fields_names_pairs_pieces_proposals: value,
            },
        });
    }
    if node.member("pieces").is_some()
        && node.member("proposals").is_some()
        && node.member("names").is_none()
        && node.member("pairs").is_none()
    {
        let value =
            read_thinkthen_complete_recognition_odds_fields_pieces_proposals_v1(node, store)
                .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_recognition_odds_v1 {
            kind: THINKTHEN_COMPLETE_RECOGNITION_ODDS_FIELDS_PIECES_PROPOSALS_V1,
            data: thinkthen_complete_recognition_odds_data_v1 {
                fields_pieces_proposals: value,
            },
        });
    }
    Err(ErrorKind::Defect)
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_recognition_odds_fields_names_pairs_pieces_proposals_field_names_v1 {
    pub data: *const *const thinkthen_complete_name_odds_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_recognition_odds_fields_names_pairs_pieces_proposals_field_names_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<
    thinkthen_complete_recognition_odds_fields_names_pairs_pieces_proposals_field_names_v1,
    ErrorKind,
> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_name_odds_v1(item, store).map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(
        thinkthen_complete_recognition_odds_fields_names_pairs_pieces_proposals_field_names_v1 {
            data,
            len,
        },
    )
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_recognition_odds_fields_names_pairs_pieces_proposals_field_pairs_v1 {
    pub data: *const *const thinkthen_complete_pair_odds_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_recognition_odds_fields_names_pairs_pieces_proposals_field_pairs_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<
    thinkthen_complete_recognition_odds_fields_names_pairs_pieces_proposals_field_pairs_v1,
    ErrorKind,
> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_pair_odds_v1(item, store).map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(
        thinkthen_complete_recognition_odds_fields_names_pairs_pieces_proposals_field_pairs_v1 {
            data,
            len,
        },
    )
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_recognition_odds_fields_names_pairs_pieces_proposals_field_pieces_v1 {
    pub data: *const *const thinkthen_complete_piece_odds_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_recognition_odds_fields_names_pairs_pieces_proposals_field_pieces_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<
    thinkthen_complete_recognition_odds_fields_names_pairs_pieces_proposals_field_pieces_v1,
    ErrorKind,
> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_piece_odds_v1(item, store).map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(
        thinkthen_complete_recognition_odds_fields_names_pairs_pieces_proposals_field_pieces_v1 {
            data,
            len,
        },
    )
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_recognition_odds_fields_names_pairs_pieces_proposals_field_proposals_v1
{
    pub data: *const *const thinkthen_complete_recognition_proposal_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_recognition_odds_fields_names_pairs_pieces_proposals_field_proposals_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<
    thinkthen_complete_recognition_odds_fields_names_pairs_pieces_proposals_field_proposals_v1,
    ErrorKind,
> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_recognition_proposal_v1(item, store)
                .map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_recognition_odds_fields_names_pairs_pieces_proposals_field_proposals_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_recognition_odds_fields_names_pairs_pieces_proposals_v1 {
    pub names:
        thinkthen_complete_recognition_odds_fields_names_pairs_pieces_proposals_field_names_v1,
    pub pairs:
        thinkthen_complete_recognition_odds_fields_names_pairs_pieces_proposals_field_pairs_v1,
    pub pieces:
        thinkthen_complete_recognition_odds_fields_names_pairs_pieces_proposals_field_pieces_v1,
    pub proposals:
        thinkthen_complete_recognition_odds_fields_names_pairs_pieces_proposals_field_proposals_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_recognition_odds_fields_names_pairs_pieces_proposals_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_recognition_odds_fields_names_pairs_pieces_proposals_v1, ErrorKind> {
    Ok(
        thinkthen_complete_recognition_odds_fields_names_pairs_pieces_proposals_v1 {
            names: {
                let value = node.required("names")?;
                read_thinkthen_complete_recognition_odds_fields_names_pairs_pieces_proposals_field_names_v1(value, store)?
            },
            pairs: {
                let value = node.required("pairs")?;
                read_thinkthen_complete_recognition_odds_fields_names_pairs_pieces_proposals_field_pairs_v1(value, store)?
            },
            pieces: {
                let value = node.required("pieces")?;
                read_thinkthen_complete_recognition_odds_fields_names_pairs_pieces_proposals_field_pieces_v1(value, store)?
            },
            proposals: {
                let value = node.required("proposals")?;
                read_thinkthen_complete_recognition_odds_fields_names_pairs_pieces_proposals_field_proposals_v1(value, store)?
            },
            extensions: store.extensions(node, &["names", "pairs", "pieces", "proposals"])?,
        },
    )
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_recognition_odds_fields_pieces_proposals_field_pieces_v1 {
    pub data: *const *const thinkthen_complete_piece_odds_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_recognition_odds_fields_pieces_proposals_field_pieces_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_recognition_odds_fields_pieces_proposals_field_pieces_v1, ErrorKind>
{
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_piece_odds_v1(item, store).map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_recognition_odds_fields_pieces_proposals_field_pieces_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_recognition_odds_fields_pieces_proposals_field_proposals_v1 {
    pub data: *const *const thinkthen_complete_boundary_proposal_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_recognition_odds_fields_pieces_proposals_field_proposals_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_recognition_odds_fields_pieces_proposals_field_proposals_v1, ErrorKind>
{
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_boundary_proposal_v1(item, store)
                .map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(
        thinkthen_complete_recognition_odds_fields_pieces_proposals_field_proposals_v1 {
            data,
            len,
        },
    )
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_recognition_odds_fields_pieces_proposals_v1 {
    pub pieces: thinkthen_complete_recognition_odds_fields_pieces_proposals_field_pieces_v1,
    pub proposals: thinkthen_complete_recognition_odds_fields_pieces_proposals_field_proposals_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_recognition_odds_fields_pieces_proposals_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_recognition_odds_fields_pieces_proposals_v1, ErrorKind> {
    Ok(
        thinkthen_complete_recognition_odds_fields_pieces_proposals_v1 {
            pieces: {
                let value = node.required("pieces")?;
                read_thinkthen_complete_recognition_odds_fields_pieces_proposals_field_pieces_v1(
                    value, store,
                )?
            },
            proposals: {
                let value = node.required("proposals")?;
                read_thinkthen_complete_recognition_odds_fields_pieces_proposals_field_proposals_v1(
                    value, store,
                )?
            },
            extensions: store.extensions(node, &["pieces", "proposals"])?,
        },
    )
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_recognition_proposal_field_kind_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_utf8_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_recognition_proposal_field_selected_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_place_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_recognition_proposal_field_strength_presence_v1 {
    pub presence: u32,
    pub value: f64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_recognition_proposal_v1 {
    pub end: u64,
    pub kept: u32,
    pub kind: thinkthen_complete_recognition_proposal_field_kind_presence_v1,
    pub selected: thinkthen_complete_recognition_proposal_field_selected_presence_v1,
    pub span_probability: f64,
    pub start: u64,
    pub strength: thinkthen_complete_recognition_proposal_field_strength_presence_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_recognition_proposal_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_recognition_proposal_v1, ErrorKind> {
    Ok(thinkthen_complete_recognition_proposal_v1 {
        end: {
            let value = node.required("end")?;
            value.number()?
        },
        kept: {
            let value = node.required("kept")?;
            value.boolean()?
        },
        kind: match node.member("kind") {
            None => thinkthen_complete_recognition_proposal_field_kind_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_recognition_proposal_field_kind_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: store.text(value.text()?),
            },
        },
        selected: match node.member("selected") {
            None => thinkthen_complete_recognition_proposal_field_selected_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_recognition_proposal_field_selected_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_place_v1(value, store)
                    .map(|value| store.hold(value))?,
            },
        },
        span_probability: {
            let value = node.required("span_probability")?;
            value.number()?
        },
        start: {
            let value = node.required("start")?;
            value.number()?
        },
        strength: match node.member("strength") {
            None => thinkthen_complete_recognition_proposal_field_strength_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_recognition_proposal_field_strength_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: value.number()?,
            },
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
pub struct thinkthen_complete_recognition_stage_context_field_boundary_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_utf8_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_recognition_stage_context_field_kind_edge_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_utf8_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_recognition_stage_context_field_relation_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_utf8_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_recognition_stage_context_v1 {
    pub boundary: thinkthen_complete_recognition_stage_context_field_boundary_presence_v1,
    pub kind_edge: thinkthen_complete_recognition_stage_context_field_kind_edge_presence_v1,
    pub relation: thinkthen_complete_recognition_stage_context_field_relation_presence_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_recognition_stage_context_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_recognition_stage_context_v1, ErrorKind> {
    Ok(thinkthen_complete_recognition_stage_context_v1 {
        boundary: match node.member("boundary") {
            None => thinkthen_complete_recognition_stage_context_field_boundary_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => {
                thinkthen_complete_recognition_stage_context_field_boundary_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                    value: store.text(value.text()?),
                }
            }
        },
        kind_edge: match node.member("kind_edge") {
            None => thinkthen_complete_recognition_stage_context_field_kind_edge_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => {
                thinkthen_complete_recognition_stage_context_field_kind_edge_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                    value: store.text(value.text()?),
                }
            }
        },
        relation: match node.member("relation") {
            None => thinkthen_complete_recognition_stage_context_field_relation_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => {
                thinkthen_complete_recognition_stage_context_field_relation_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                    value: store.text(value.text()?),
                }
            }
        },
        extensions: store.extensions(node, &["boundary", "kind_edge", "relation"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_relation_field_file_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_utf8_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_relation_field_first_line_presence_v1 {
    pub presence: u32,
    pub value: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_relation_field_index_presence_v1 {
    pub presence: u32,
    pub value: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_relation_field_input_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_json_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_relation_field_last_line_presence_v1 {
    pub presence: u32,
    pub value: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_relation_field_position_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_position_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_relation_field_value_v1 {
    pub data: *const *const thinkthen_complete_related_entity_edge_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_relation_field_value_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_relation_field_value_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_related_entity_edge_v1(item, store)
                .map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_relation_field_value_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_relation_v1 {
    pub answer: *const thinkthen_complete_answers_v1,
    pub answer_id: *const thinkthen_complete_answer_id_v1,
    pub file: thinkthen_complete_relation_field_file_presence_v1,
    pub first_line: thinkthen_complete_relation_field_first_line_presence_v1,
    pub index: thinkthen_complete_relation_field_index_presence_v1,
    pub input: thinkthen_complete_relation_field_input_presence_v1,
    pub last_line: thinkthen_complete_relation_field_last_line_presence_v1,
    pub meta: *const thinkthen_complete_meta_v1,
    pub position: thinkthen_complete_relation_field_position_presence_v1,
    pub question: *const thinkthen_complete_readable_question_4_v1,
    pub schema: *const thinkthen_complete_version_v1,
    pub value: thinkthen_complete_relation_field_value_v1,
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
        file: match node.member("file") {
            None => thinkthen_complete_relation_field_file_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_relation_field_file_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: store.text(value.text()?),
            },
        },
        first_line: match node.member("first_line") {
            None => thinkthen_complete_relation_field_first_line_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_relation_field_first_line_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: value.number()?,
            },
        },
        index: match node.member("index") {
            None => thinkthen_complete_relation_field_index_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_relation_field_index_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: value.number()?,
            },
        },
        input: match node.member("input") {
            None => thinkthen_complete_relation_field_input_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) if value.kind() == "null" => {
                thinkthen_complete_relation_field_input_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_NULL_V1,
                    value: zero(),
                }
            }
            Some(value) => thinkthen_complete_relation_field_input_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_json(value, store)?,
            },
        },
        last_line: match node.member("last_line") {
            None => thinkthen_complete_relation_field_last_line_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_relation_field_last_line_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: value.number()?,
            },
        },
        meta: {
            let value = node.required("meta")?;
            read_thinkthen_complete_meta_v1(value, store).map(|value| store.hold(value))?
        },
        position: match node.member("position") {
            None => thinkthen_complete_relation_field_position_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_relation_field_position_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_position_v1(value, store)
                    .map(|value| store.hold(value))?,
            },
        },
        question: {
            let value = node.required("question")?;
            read_thinkthen_complete_readable_question_4_v1(value, store)
                .map(|value| store.hold(value))?
        },
        schema: {
            let value = node.required("schema")?;
            read_thinkthen_complete_version_v1(value, store).map(|value| store.hold(value))?
        },
        value: {
            let value = node.required("value")?;
            read_thinkthen_complete_relation_field_value_v1(value, store)?
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
pub struct thinkthen_complete_relation_member_answer_id_field_observations_v1 {
    pub data: *const *const thinkthen_complete_observation_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_relation_member_answer_id_field_observations_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_relation_member_answer_id_field_observations_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_observation_v1(item, store).map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_relation_member_answer_id_field_observations_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_relation_member_answer_id_field_question_sources_v1 {
    pub data: *const *const thinkthen_complete_question_source_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_relation_member_answer_id_field_question_sources_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_relation_member_answer_id_field_question_sources_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_question_source_v1(item, store)
                .map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_relation_member_answer_id_field_question_sources_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_relation_member_answer_id_field_target_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_related_entity_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_relation_member_answer_id_field_usage_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_usage_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_relation_member_answer_id_v1 {
    pub direction: *const thinkthen_complete_relation_direction_v1,
    pub method: *const thinkthen_complete_relation_method_v1,
    pub observations: thinkthen_complete_relation_member_answer_id_field_observations_v1,
    pub question: *const thinkthen_complete_readable_question_v1,
    pub question_sources: thinkthen_complete_relation_member_answer_id_field_question_sources_v1,
    pub reads: thinkthen_complete_utf8_v1,
    pub relation: thinkthen_complete_utf8_v1,
    pub request: thinkthen_complete_utf8_v1,
    pub source: *const thinkthen_complete_related_entity_v1,
    pub target: thinkthen_complete_relation_member_answer_id_field_target_presence_v1,
    pub threshold: *const thinkthen_complete_threshold_v1,
    pub usage: thinkthen_complete_relation_member_answer_id_field_usage_presence_v1,
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
            read_thinkthen_complete_relation_member_answer_id_field_observations_v1(value, store)?
        },
        question: {
            let value = node.required("question")?;
            read_thinkthen_complete_readable_question_v1(value, store)
                .map(|value| store.hold(value))?
        },
        question_sources: {
            let value = node.required("question_sources")?;
            read_thinkthen_complete_relation_member_answer_id_field_question_sources_v1(
                value, store,
            )?
        },
        reads: {
            let value = node.required("reads")?;
            store.text(value.text()?)
        },
        relation: {
            let value = node.required("relation")?;
            store.text(value.text()?)
        },
        request: {
            let value = node.required("request")?;
            store.text(value.text()?)
        },
        source: {
            let value = node.required("source")?;
            read_thinkthen_complete_related_entity_v1(value, store)
                .map(|value| store.hold(value))?
        },
        target: match node.member("target") {
            None => return Err(ErrorKind::Defect),
            Some(value) if value.kind() == "null" => {
                thinkthen_complete_relation_member_answer_id_field_target_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_NULL_V1,
                    value: zero(),
                }
            }
            Some(value) => thinkthen_complete_relation_member_answer_id_field_target_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_related_entity_v1(value, store)
                    .map(|value| store.hold(value))?,
            },
        },
        threshold: {
            let value = node.required("threshold")?;
            read_thinkthen_complete_threshold_v1(value, store).map(|value| store.hold(value))?
        },
        usage: match node.member("usage") {
            None => thinkthen_complete_relation_member_answer_id_field_usage_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_relation_member_answer_id_field_usage_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_usage_v1(value, store)
                    .map(|value| store.hold(value))?,
            },
        },
        accepted: {
            let value = node.required("accepted")?;
            value.boolean()?
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
            value.number()?
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
pub struct thinkthen_complete_relation_member_failure_id_field_observations_v1 {
    pub data: *const *const thinkthen_complete_observation_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_relation_member_failure_id_field_observations_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_relation_member_failure_id_field_observations_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_observation_v1(item, store).map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_relation_member_failure_id_field_observations_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_relation_member_failure_id_field_question_sources_v1 {
    pub data: *const *const thinkthen_complete_question_source_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_relation_member_failure_id_field_question_sources_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_relation_member_failure_id_field_question_sources_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_question_source_v1(item, store)
                .map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_relation_member_failure_id_field_question_sources_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_relation_member_failure_id_field_target_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_related_entity_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_relation_member_failure_id_field_usage_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_usage_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_relation_member_failure_id_v1 {
    pub direction: *const thinkthen_complete_relation_direction_v1,
    pub method: *const thinkthen_complete_relation_method_v1,
    pub observations: thinkthen_complete_relation_member_failure_id_field_observations_v1,
    pub question: *const thinkthen_complete_readable_question_v1,
    pub question_sources: thinkthen_complete_relation_member_failure_id_field_question_sources_v1,
    pub reads: thinkthen_complete_utf8_v1,
    pub relation: thinkthen_complete_utf8_v1,
    pub request: thinkthen_complete_utf8_v1,
    pub source: *const thinkthen_complete_related_entity_v1,
    pub target: thinkthen_complete_relation_member_failure_id_field_target_presence_v1,
    pub threshold: *const thinkthen_complete_threshold_v1,
    pub usage: thinkthen_complete_relation_member_failure_id_field_usage_presence_v1,
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
            read_thinkthen_complete_relation_member_failure_id_field_observations_v1(value, store)?
        },
        question: {
            let value = node.required("question")?;
            read_thinkthen_complete_readable_question_v1(value, store)
                .map(|value| store.hold(value))?
        },
        question_sources: {
            let value = node.required("question_sources")?;
            read_thinkthen_complete_relation_member_failure_id_field_question_sources_v1(
                value, store,
            )?
        },
        reads: {
            let value = node.required("reads")?;
            store.text(value.text()?)
        },
        relation: {
            let value = node.required("relation")?;
            store.text(value.text()?)
        },
        request: {
            let value = node.required("request")?;
            store.text(value.text()?)
        },
        source: {
            let value = node.required("source")?;
            read_thinkthen_complete_related_entity_v1(value, store)
                .map(|value| store.hold(value))?
        },
        target: match node.member("target") {
            None => return Err(ErrorKind::Defect),
            Some(value) if value.kind() == "null" => {
                thinkthen_complete_relation_member_failure_id_field_target_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_NULL_V1,
                    value: zero(),
                }
            }
            Some(value) => thinkthen_complete_relation_member_failure_id_field_target_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_related_entity_v1(value, store)
                    .map(|value| store.hold(value))?,
            },
        },
        threshold: {
            let value = node.required("threshold")?;
            read_thinkthen_complete_threshold_v1(value, store).map(|value| store.hold(value))?
        },
        usage: match node.member("usage") {
            None => thinkthen_complete_relation_member_failure_id_field_usage_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_relation_member_failure_id_field_usage_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_usage_v1(value, store)
                    .map(|value| store.hold(value))?,
            },
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
    let value = store.text(node.text()?);
    Ok(thinkthen_complete_sdk_request_id_v1 { value })
}

pub const THINKTHEN_COMPLETE_SEND_BUDGET_DENIAL_BEFORE_FIRST_SEND_V1: u32 = 1;

pub const THINKTHEN_COMPLETE_SEND_BUDGET_DENIAL_BEFORE_ADDITIONAL_SEND_V1: u32 = 2;

pub const THINKTHEN_COMPLETE_SEND_BUDGET_DENIAL_BEFORE_RETRY_V1: u32 = 3;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_send_budget_denial_data_v1 {
    pub before_first_send: *const thinkthen_complete_send_budget_denial_before_first_send_v1,
    pub before_additional_send:
        *const thinkthen_complete_send_budget_denial_before_additional_send_v1,
    pub before_retry: *const thinkthen_complete_send_budget_denial_before_retry_v1,
}
impl std::fmt::Debug for thinkthen_complete_send_budget_denial_data_v1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("union payload")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_send_budget_denial_v1 {
    pub kind: u32,
    pub data: thinkthen_complete_send_budget_denial_data_v1,
}

pub(crate) fn read_thinkthen_complete_send_budget_denial_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_send_budget_denial_v1, ErrorKind> {
    if node.is_member("kind", "before_first_send") {
        let value = read_thinkthen_complete_send_budget_denial_before_first_send_v1(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_send_budget_denial_v1 {
            kind: THINKTHEN_COMPLETE_SEND_BUDGET_DENIAL_BEFORE_FIRST_SEND_V1,
            data: thinkthen_complete_send_budget_denial_data_v1 {
                before_first_send: value,
            },
        });
    }
    if node.is_member("kind", "before_additional_send") {
        let value =
            read_thinkthen_complete_send_budget_denial_before_additional_send_v1(node, store)
                .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_send_budget_denial_v1 {
            kind: THINKTHEN_COMPLETE_SEND_BUDGET_DENIAL_BEFORE_ADDITIONAL_SEND_V1,
            data: thinkthen_complete_send_budget_denial_data_v1 {
                before_additional_send: value,
            },
        });
    }
    if node.is_member("kind", "before_retry") {
        let value = read_thinkthen_complete_send_budget_denial_before_retry_v1(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_send_budget_denial_v1 {
            kind: THINKTHEN_COMPLETE_SEND_BUDGET_DENIAL_BEFORE_RETRY_V1,
            data: thinkthen_complete_send_budget_denial_data_v1 {
                before_retry: value,
            },
        });
    }
    Err(ErrorKind::Defect)
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_send_budget_denial_before_additional_send_v1 {
    pub kind: thinkthen_complete_utf8_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_send_budget_denial_before_additional_send_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_send_budget_denial_before_additional_send_v1, ErrorKind> {
    Ok(
        thinkthen_complete_send_budget_denial_before_additional_send_v1 {
            kind: {
                let value = node.required("kind")?;
                store.text(value.text()?)
            },
            extensions: store.extensions(node, &["kind"])?,
        },
    )
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_send_budget_denial_before_first_send_v1 {
    pub kind: thinkthen_complete_utf8_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_send_budget_denial_before_first_send_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_send_budget_denial_before_first_send_v1, ErrorKind> {
    Ok(thinkthen_complete_send_budget_denial_before_first_send_v1 {
        kind: {
            let value = node.required("kind")?;
            store.text(value.text()?)
        },
        extensions: store.extensions(node, &["kind"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_send_budget_denial_before_retry_v1 {
    pub kind: thinkthen_complete_utf8_v1,
    pub last_status: u64,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_send_budget_denial_before_retry_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_send_budget_denial_before_retry_v1, ErrorKind> {
    Ok(thinkthen_complete_send_budget_denial_before_retry_v1 {
        kind: {
            let value = node.required("kind")?;
            store.text(value.text()?)
        },
        last_status: {
            let value = node.required("last_status")?;
            value.number()?
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
pub struct thinkthen_complete_stopped_field_at_presence_v1 {
    pub presence: u32,
    pub value: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_stopped_field_status_presence_v1 {
    pub presence: u32,
    pub value: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_stopped_v1 {
    pub at: thinkthen_complete_stopped_field_at_presence_v1,
    pub cause: *const thinkthen_complete_stop_cause_v1,
    pub retryable: u32,
    pub status: thinkthen_complete_stopped_field_status_presence_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_stopped_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_stopped_v1, ErrorKind> {
    Ok(thinkthen_complete_stopped_v1 {
        at: match node.member("at") {
            None => thinkthen_complete_stopped_field_at_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_stopped_field_at_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: value.number()?,
            },
        },
        cause: {
            let value = node.required("cause")?;
            read_thinkthen_complete_stop_cause_v1(value, store).map(|value| store.hold(value))?
        },
        retryable: {
            let value = node.required("retryable")?;
            value.boolean()?
        },
        status: match node.member("status") {
            None => thinkthen_complete_stopped_field_status_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_stopped_field_status_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: value.number()?,
            },
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
pub struct thinkthen_complete_usage_field_input_tokens_presence_v1 {
    pub presence: u32,
    pub value: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_usage_field_output_tokens_presence_v1 {
    pub presence: u32,
    pub value: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_usage_v1 {
    pub input_tokens: thinkthen_complete_usage_field_input_tokens_presence_v1,
    pub output_tokens: thinkthen_complete_usage_field_output_tokens_presence_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_usage_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_usage_v1, ErrorKind> {
    Ok(thinkthen_complete_usage_v1 {
        input_tokens: match node.member("input_tokens") {
            None => thinkthen_complete_usage_field_input_tokens_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_usage_field_input_tokens_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: value.number()?,
            },
        },
        output_tokens: match node.member("output_tokens") {
            None => thinkthen_complete_usage_field_output_tokens_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_usage_field_output_tokens_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: value.number()?,
            },
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
    _store: &mut Storage,
) -> Result<thinkthen_complete_wording_version_v1, ErrorKind> {
    let value = node.number()?;
    Ok(thinkthen_complete_wording_version_v1 { value })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_annotated_field_array_v1 {
    pub data: *const thinkthen_complete_utf8_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_annotated_field_array_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_annotated_field_array_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(store.text(item.text()?));
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_annotated_field_array_v1 { data, len })
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
    pub array: thinkthen_complete_annotated_field_array_v1,
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
        let value = node.boolean()?;
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
        let value = store.text(node.text()?);
        return Ok(thinkthen_complete_annotated_field_v1 {
            kind: THINKTHEN_COMPLETE_ANNOTATED_FIELD_STRING_V1,
            data: thinkthen_complete_annotated_field_data_v1 { string: value },
        });
    }
    if node.kind() == "array" {
        let value = read_thinkthen_complete_annotated_field_array_v1(node, store)?;
        return Ok(thinkthen_complete_annotated_field_v1 {
            kind: THINKTHEN_COMPLETE_ANNOTATED_FIELD_ARRAY_V1,
            data: thinkthen_complete_annotated_field_data_v1 { array: value },
        });
    }
    if node.kind() == "number" {
        let value = node.number()?;
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
pub struct thinkthen_complete_annotated_row_value_entry_v1 {
    pub name: thinkthen_complete_utf8_v1,
    pub value: *const thinkthen_complete_annotated_field_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_annotated_row_value_v1 {
    pub data: *const thinkthen_complete_annotated_row_value_entry_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_annotated_row_value_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_annotated_row_value_v1, ErrorKind> {
    let mut values = Vec::new();
    for (key, item) in node.object()? {
        values.push(thinkthen_complete_annotated_row_value_entry_v1 {
            name: store.text(key),
            value: read_thinkthen_complete_annotated_field_v1(item, store)
                .map(|value| store.hold(value))?,
        });
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_annotated_row_value_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_annotated_row_v1 {
    pub value: thinkthen_complete_annotated_row_value_v1,
}

pub(crate) fn read_thinkthen_complete_annotated_row_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_annotated_row_v1, ErrorKind> {
    let value = read_thinkthen_complete_annotated_row_value_v1(node, store)?;
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
pub struct thinkthen_complete_answer_choice_field_confidence_presence_v1 {
    pub presence: u32,
    pub value: f64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answer_choice_field_probabilities_entry_v1 {
    pub name: thinkthen_complete_utf8_v1,
    pub value: f64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answer_choice_field_probabilities_v1 {
    pub data: *const thinkthen_complete_answer_choice_field_probabilities_entry_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_answer_choice_field_probabilities_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_answer_choice_field_probabilities_v1, ErrorKind> {
    let mut values = Vec::new();
    for (key, item) in node.object()? {
        values.push(
            thinkthen_complete_answer_choice_field_probabilities_entry_v1 {
                name: store.text(key),
                value: item.number()?,
            },
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_answer_choice_field_probabilities_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answer_choice_v1 {
    pub confidence: thinkthen_complete_answer_choice_field_confidence_presence_v1,
    pub kind: thinkthen_complete_utf8_v1,
    pub pick: thinkthen_complete_utf8_v1,
    pub probabilities: thinkthen_complete_answer_choice_field_probabilities_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_answer_choice_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_answer_choice_v1, ErrorKind> {
    Ok(thinkthen_complete_answer_choice_v1 {
        confidence: match node.member("confidence") {
            None => thinkthen_complete_answer_choice_field_confidence_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_answer_choice_field_confidence_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: value.number()?,
            },
        },
        kind: {
            let value = node.required("kind")?;
            store.text(value.text()?)
        },
        pick: {
            let value = node.required("pick")?;
            store.text(value.text()?)
        },
        probabilities: {
            let value = node.required("probabilities")?;
            read_thinkthen_complete_answer_choice_field_probabilities_v1(value, store)?
        },
        extensions: store.extensions(node, &["confidence", "kind", "pick", "probabilities"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answer_score_field_confidence_presence_v1 {
    pub presence: u32,
    pub value: f64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answer_score_field_probabilities_entry_v1 {
    pub name: thinkthen_complete_utf8_v1,
    pub value: f64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answer_score_field_probabilities_v1 {
    pub data: *const thinkthen_complete_answer_score_field_probabilities_entry_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_answer_score_field_probabilities_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_answer_score_field_probabilities_v1, ErrorKind> {
    let mut values = Vec::new();
    for (key, item) in node.object()? {
        values.push(
            thinkthen_complete_answer_score_field_probabilities_entry_v1 {
                name: store.text(key),
                value: item.number()?,
            },
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_answer_score_field_probabilities_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answer_score_v1 {
    pub confidence: thinkthen_complete_answer_score_field_confidence_presence_v1,
    pub kind: thinkthen_complete_utf8_v1,
    pub level: thinkthen_complete_utf8_v1,
    pub probabilities: thinkthen_complete_answer_score_field_probabilities_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_answer_score_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_answer_score_v1, ErrorKind> {
    Ok(thinkthen_complete_answer_score_v1 {
        confidence: match node.member("confidence") {
            None => thinkthen_complete_answer_score_field_confidence_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_answer_score_field_confidence_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: value.number()?,
            },
        },
        kind: {
            let value = node.required("kind")?;
            store.text(value.text()?)
        },
        level: {
            let value = node.required("level")?;
            store.text(value.text()?)
        },
        probabilities: {
            let value = node.required("probabilities")?;
            read_thinkthen_complete_answer_score_field_probabilities_v1(value, store)?
        },
        extensions: store.extensions(node, &["confidence", "kind", "level", "probabilities"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answer_tag_field_probabilities_entry_v1 {
    pub name: thinkthen_complete_utf8_v1,
    pub value: f64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answer_tag_field_probabilities_v1 {
    pub data: *const thinkthen_complete_answer_tag_field_probabilities_entry_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_answer_tag_field_probabilities_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_answer_tag_field_probabilities_v1, ErrorKind> {
    let mut values = Vec::new();
    for (key, item) in node.object()? {
        values.push(thinkthen_complete_answer_tag_field_probabilities_entry_v1 {
            name: store.text(key),
            value: item.number()?,
        });
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_answer_tag_field_probabilities_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_answer_tag_v1 {
    pub kind: thinkthen_complete_utf8_v1,
    pub probabilities: thinkthen_complete_answer_tag_field_probabilities_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_answer_tag_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_answer_tag_v1, ErrorKind> {
    Ok(thinkthen_complete_answer_tag_v1 {
        kind: {
            let value = node.required("kind")?;
            store.text(value.text()?)
        },
        probabilities: {
            let value = node.required("probabilities")?;
            read_thinkthen_complete_answer_tag_field_probabilities_v1(value, store)?
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
            store.text(value.text()?)
        },
        probability: {
            let value = node.required("probability")?;
            value.number()?
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
        let value = node.number()?;
        return Ok(thinkthen_complete_batch_setting_v1 {
            kind: THINKTHEN_COMPLETE_BATCH_SETTING_INTEGER_V1,
            data: thinkthen_complete_batch_setting_data_v1 { integer: value },
        });
    }
    if node.kind() == "string" {
        let value = store.text(node.text()?);
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
pub struct thinkthen_complete_entity_field_file_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_utf8_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_entity_field_first_line_presence_v1 {
    pub presence: u32,
    pub value: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_entity_field_last_line_presence_v1 {
    pub presence: u32,
    pub value: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_entity_v1 {
    pub end: u64,
    pub file: thinkthen_complete_entity_field_file_presence_v1,
    pub first_line: thinkthen_complete_entity_field_first_line_presence_v1,
    pub kind: thinkthen_complete_utf8_v1,
    pub last_line: thinkthen_complete_entity_field_last_line_presence_v1,
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
            value.number()?
        },
        file: match node.member("file") {
            None => thinkthen_complete_entity_field_file_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_entity_field_file_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: store.text(value.text()?),
            },
        },
        first_line: match node.member("first_line") {
            None => thinkthen_complete_entity_field_first_line_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_entity_field_first_line_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: value.number()?,
            },
        },
        kind: {
            let value = node.required("kind")?;
            store.text(value.text()?)
        },
        last_line: match node.member("last_line") {
            None => thinkthen_complete_entity_field_last_line_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_entity_field_last_line_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: value.number()?,
            },
        },
        length: {
            let value = node.required("length")?;
            value.number()?
        },
        start: {
            let value = node.required("start")?;
            value.number()?
        },
        strength: {
            let value = node.required("strength")?;
            value.number()?
        },
        text: {
            let value = node.required("text")?;
            store.text(value.text()?)
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
pub struct thinkthen_complete_entity_edge_field_either_presence_v1 {
    pub presence: u32,
    pub value: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_entity_edge_v1 {
    pub either: thinkthen_complete_entity_edge_field_either_presence_v1,
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
        either: match node.member("either") {
            None => thinkthen_complete_entity_edge_field_either_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_entity_edge_field_either_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: value.boolean()?,
            },
        },
        probability: {
            let value = node.required("probability")?;
            value.number()?
        },
        relation: {
            let value = node.required("relation")?;
            store.text(value.text()?)
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
pub struct thinkthen_complete_find_answer_field_confidence_presence_v1 {
    pub presence: u32,
    pub value: f64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_find_answer_field_probabilities_entry_v1 {
    pub name: thinkthen_complete_utf8_v1,
    pub value: f64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_find_answer_field_probabilities_v1 {
    pub data: *const thinkthen_complete_find_answer_field_probabilities_entry_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_find_answer_field_probabilities_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_find_answer_field_probabilities_v1, ErrorKind> {
    let mut values = Vec::new();
    for (key, item) in node.object()? {
        values.push(
            thinkthen_complete_find_answer_field_probabilities_entry_v1 {
                name: store.text(key),
                value: item.number()?,
            },
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_find_answer_field_probabilities_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_find_answer_v1 {
    pub confidence: thinkthen_complete_find_answer_field_confidence_presence_v1,
    pub kind: thinkthen_complete_utf8_v1,
    pub pick: thinkthen_complete_utf8_v1,
    pub probabilities: thinkthen_complete_find_answer_field_probabilities_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_find_answer_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_find_answer_v1, ErrorKind> {
    Ok(thinkthen_complete_find_answer_v1 {
        confidence: match node.member("confidence") {
            None => thinkthen_complete_find_answer_field_confidence_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_find_answer_field_confidence_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: value.number()?,
            },
        },
        kind: {
            let value = node.required("kind")?;
            store.text(value.text()?)
        },
        pick: {
            let value = node.required("pick")?;
            store.text(value.text()?)
        },
        probabilities: {
            let value = node.required("probabilities")?;
            read_thinkthen_complete_find_answer_field_probabilities_v1(value, store)?
        },
        extensions: store.extensions(node, &["confidence", "kind", "pick", "probabilities"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_name_odds_field_edges_entry_v1 {
    pub name: thinkthen_complete_utf8_v1,
    pub value: f64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_name_odds_field_edges_v1 {
    pub data: *const thinkthen_complete_name_odds_field_edges_entry_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_name_odds_field_edges_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_name_odds_field_edges_v1, ErrorKind> {
    let mut values = Vec::new();
    for (key, item) in node.object()? {
        values.push(thinkthen_complete_name_odds_field_edges_entry_v1 {
            name: store.text(key),
            value: item.number()?,
        });
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_name_odds_field_edges_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_name_odds_field_edges_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_name_odds_field_edges_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_name_odds_field_kinds_entry_v1 {
    pub name: thinkthen_complete_utf8_v1,
    pub value: f64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_name_odds_field_kinds_v1 {
    pub data: *const thinkthen_complete_name_odds_field_kinds_entry_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_name_odds_field_kinds_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_name_odds_field_kinds_v1, ErrorKind> {
    let mut values = Vec::new();
    for (key, item) in node.object()? {
        values.push(thinkthen_complete_name_odds_field_kinds_entry_v1 {
            name: store.text(key),
            value: item.number()?,
        });
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_name_odds_field_kinds_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_name_odds_field_kinds_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_name_odds_field_kinds_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_name_odds_v1 {
    pub edges: thinkthen_complete_name_odds_field_edges_presence_v1,
    pub end: u64,
    pub kinds: thinkthen_complete_name_odds_field_kinds_presence_v1,
    pub start: u64,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_name_odds_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_name_odds_v1, ErrorKind> {
    Ok(thinkthen_complete_name_odds_v1 {
        edges: match node.member("edges") {
            None => return Err(ErrorKind::Defect),
            Some(value) if value.kind() == "null" => {
                thinkthen_complete_name_odds_field_edges_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_NULL_V1,
                    value: zero(),
                }
            }
            Some(value) => thinkthen_complete_name_odds_field_edges_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_name_odds_field_edges_v1(value, store)?,
            },
        },
        end: {
            let value = node.required("end")?;
            value.number()?
        },
        kinds: match node.member("kinds") {
            None => return Err(ErrorKind::Defect),
            Some(value) if value.kind() == "null" => {
                thinkthen_complete_name_odds_field_kinds_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_NULL_V1,
                    value: zero(),
                }
            }
            Some(value) => thinkthen_complete_name_odds_field_kinds_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_name_odds_field_kinds_v1(value, store)?,
            },
        },
        start: {
            let value = node.required("start")?;
            value.number()?
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
            value.number()?
        },
        relation: {
            let value = node.required("relation")?;
            store.text(value.text()?)
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
pub struct thinkthen_complete_piece_odds_field_tags_entry_v1 {
    pub name: thinkthen_complete_utf8_v1,
    pub value: f64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_piece_odds_field_tags_v1 {
    pub data: *const thinkthen_complete_piece_odds_field_tags_entry_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_piece_odds_field_tags_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_piece_odds_field_tags_v1, ErrorKind> {
    let mut values = Vec::new();
    for (key, item) in node.object()? {
        values.push(thinkthen_complete_piece_odds_field_tags_entry_v1 {
            name: store.text(key),
            value: item.number()?,
        });
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_piece_odds_field_tags_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_piece_odds_v1 {
    pub end: u64,
    pub start: u64,
    pub tags: thinkthen_complete_piece_odds_field_tags_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_piece_odds_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_piece_odds_v1, ErrorKind> {
    Ok(thinkthen_complete_piece_odds_v1 {
        end: {
            let value = node.required("end")?;
            value.number()?
        },
        start: {
            let value = node.required("start")?;
            value.number()?
        },
        tags: {
            let value = node.required("tags")?;
            read_thinkthen_complete_piece_odds_field_tags_v1(value, store)?
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
            value.number()?
        },
        start: {
            let value = node.required("start")?;
            value.number()?
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
            store.text(value.text()?)
        },
        tuned_for: {
            let value = node.required("tuned_for")?;
            store.text(value.text()?)
        },
        extensions: store.extensions(node, &["running", "tuned_for"])?,
    })
}

pub const THINKTHEN_COMPLETE_RECOGNIZE_FIELDS_ENTITIES_V1: u32 = 1;

pub const THINKTHEN_COMPLETE_RECOGNIZE_FIELDS_MODE_PROPOSALS_V1: u32 = 2;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_recognize_data_v1 {
    pub fields_entities: *const thinkthen_complete_recognize_fields_entities_v1,
    pub fields_mode_proposals: *const thinkthen_complete_recognize_fields_mode_proposals_v1,
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
        let value = read_thinkthen_complete_recognize_fields_entities_v1(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_recognize_v1 {
            kind: THINKTHEN_COMPLETE_RECOGNIZE_FIELDS_ENTITIES_V1,
            data: thinkthen_complete_recognize_data_v1 {
                fields_entities: value,
            },
        });
    }
    if node.member("mode").is_some()
        && node.member("proposals").is_some()
        && node.member("entities").is_none()
    {
        let value = read_thinkthen_complete_recognize_fields_mode_proposals_v1(node, store)
            .map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_recognize_v1 {
            kind: THINKTHEN_COMPLETE_RECOGNIZE_FIELDS_MODE_PROPOSALS_V1,
            data: thinkthen_complete_recognize_data_v1 {
                fields_mode_proposals: value,
            },
        });
    }
    Err(ErrorKind::Defect)
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_recognize_fields_entities_field_entities_v1 {
    pub data: *const *const thinkthen_complete_entity_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_recognize_fields_entities_field_entities_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_recognize_fields_entities_field_entities_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(read_thinkthen_complete_entity_v1(item, store).map(|value| store.hold(value))?);
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_recognize_fields_entities_field_entities_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_recognize_fields_entities_field_relations_v1 {
    pub data: *const *const thinkthen_complete_entity_edge_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_recognize_fields_entities_field_relations_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_recognize_fields_entities_field_relations_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_entity_edge_v1(item, store).map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_recognize_fields_entities_field_relations_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_recognize_fields_entities_field_relations_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_recognize_fields_entities_field_relations_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_recognize_fields_entities_v1 {
    pub entities: thinkthen_complete_recognize_fields_entities_field_entities_v1,
    pub relations: thinkthen_complete_recognize_fields_entities_field_relations_presence_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_recognize_fields_entities_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_recognize_fields_entities_v1, ErrorKind> {
    Ok(thinkthen_complete_recognize_fields_entities_v1 {
        entities: {
            let value = node.required("entities")?;
            read_thinkthen_complete_recognize_fields_entities_field_entities_v1(value, store)?
        },
        relations: match node.member("relations") {
            None => thinkthen_complete_recognize_fields_entities_field_relations_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => {
                thinkthen_complete_recognize_fields_entities_field_relations_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                    value: read_thinkthen_complete_recognize_fields_entities_field_relations_v1(
                        value, store,
                    )?,
                }
            }
        },
        extensions: store.extensions(node, &["entities", "relations"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_recognize_fields_mode_proposals_field_proposals_v1 {
    pub data: *const *const thinkthen_complete_boundary_proposal_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_recognize_fields_mode_proposals_field_proposals_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_recognize_fields_mode_proposals_field_proposals_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_boundary_proposal_v1(item, store)
                .map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_recognize_fields_mode_proposals_field_proposals_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_recognize_fields_mode_proposals_v1 {
    pub mode: *const thinkthen_complete_boundary_mode_v1,
    pub proposals: thinkthen_complete_recognize_fields_mode_proposals_field_proposals_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_recognize_fields_mode_proposals_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_recognize_fields_mode_proposals_v1, ErrorKind> {
    Ok(thinkthen_complete_recognize_fields_mode_proposals_v1 {
        mode: {
            let value = node.required("mode")?;
            read_thinkthen_complete_boundary_mode_v1(value, store).map(|value| store.hold(value))?
        },
        proposals: {
            let value = node.required("proposals")?;
            read_thinkthen_complete_recognize_fields_mode_proposals_field_proposals_v1(
                value, store,
            )?
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
            store.text(value.text()?)
        },
        name: {
            let value = node.required("name")?;
            store.text(value.text()?)
        },
        extensions: store.extensions(node, &["kind", "name"])?,
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
            store.text(value.text()?)
        },
        name: {
            let value = node.required("name")?;
            store.text(value.text()?)
        },
        extensions: store.extensions(node, &["kind", "name"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_related_entity_edge_field_either_presence_v1 {
    pub presence: u32,
    pub value: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_related_entity_edge_v1 {
    pub either: thinkthen_complete_related_entity_edge_field_either_presence_v1,
    pub probability: f64,
    pub relation: thinkthen_complete_utf8_v1,
    pub source: *const thinkthen_complete_related_entity_edge_properties_source_v1,
    pub target: *const thinkthen_complete_related_entity_edge_properties_source_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_related_entity_edge_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_related_entity_edge_v1, ErrorKind> {
    Ok(thinkthen_complete_related_entity_edge_v1 {
        either: match node.member("either") {
            None => thinkthen_complete_related_entity_edge_field_either_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_related_entity_edge_field_either_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: value.boolean()?,
            },
        },
        probability: {
            let value = node.required("probability")?;
            value.number()?
        },
        relation: {
            let value = node.required("relation")?;
            store.text(value.text()?)
        },
        source: {
            let value = node.required("source")?;
            read_thinkthen_complete_related_entity_edge_properties_source_v1(value, store)
                .map(|value| store.hold(value))?
        },
        target: {
            let value = node.required("target")?;
            read_thinkthen_complete_related_entity_edge_properties_source_v1(value, store)
                .map(|value| store.hold(value))?
        },
        extensions: store.extensions(
            node,
            &["either", "probability", "relation", "source", "target"],
        )?,
    })
}

pub const THINKTHEN_COMPLETE_RELATED_ENTITY_EDGE_PROPERTIES_SOURCE_FIELDS_KIND_NAME_V1: u32 = 1;

pub const THINKTHEN_COMPLETE_RELATED_ENTITY_EDGE_PROPERTIES_SOURCE_FIELDS_FILE_KIND_NAME_ORDINAL_RECORD_V1: u32 = 2;

#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_related_entity_edge_properties_source_data_v1 {
    pub fields_kind_name: *const thinkthen_complete_related_entity_edge_properties_source_fields_kind_name_v1,
    pub fields_file_kind_name_ordinal_record: *const thinkthen_complete_related_entity_edge_properties_source_fields_file_kind_name_ordinal_record_v1,
}
impl std::fmt::Debug for thinkthen_complete_related_entity_edge_properties_source_data_v1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("union payload")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_related_entity_edge_properties_source_v1 {
    pub kind: u32,
    pub data: thinkthen_complete_related_entity_edge_properties_source_data_v1,
}

pub(crate) fn read_thinkthen_complete_related_entity_edge_properties_source_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_related_entity_edge_properties_source_v1, ErrorKind> {
    if node.member("kind").is_some()
        && node.member("name").is_some()
        && node.member("file").is_none()
        && node.member("ordinal").is_none()
        && node.member("record").is_none()
    {
        let value =
            read_thinkthen_complete_related_entity_edge_properties_source_fields_kind_name_v1(
                node, store,
            )
            .map(|value| store.hold(value))?;
        return Ok(
            thinkthen_complete_related_entity_edge_properties_source_v1 {
                kind: THINKTHEN_COMPLETE_RELATED_ENTITY_EDGE_PROPERTIES_SOURCE_FIELDS_KIND_NAME_V1,
                data: thinkthen_complete_related_entity_edge_properties_source_data_v1 {
                    fields_kind_name: value,
                },
            },
        );
    }
    if node.member("file").is_some()
        && node.member("kind").is_some()
        && node.member("name").is_some()
        && node.member("ordinal").is_some()
        && node.member("record").is_some()
    {
        let value = read_thinkthen_complete_related_entity_edge_properties_source_fields_file_kind_name_ordinal_record_v1(node, store).map(|value| store.hold(value))?;
        return Ok(thinkthen_complete_related_entity_edge_properties_source_v1 { kind: THINKTHEN_COMPLETE_RELATED_ENTITY_EDGE_PROPERTIES_SOURCE_FIELDS_FILE_KIND_NAME_ORDINAL_RECORD_V1, data: thinkthen_complete_related_entity_edge_properties_source_data_v1 { fields_file_kind_name_ordinal_record: value } });
    }
    Err(ErrorKind::Defect)
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_related_entity_edge_properties_source_fields_file_kind_name_ordinal_record_field_file_presence_v1
{
    pub presence: u32,
    pub value: thinkthen_complete_utf8_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_related_entity_edge_properties_source_fields_file_kind_name_ordinal_record_field_first_line_presence_v1
{
    pub presence: u32,
    pub value: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_related_entity_edge_properties_source_fields_file_kind_name_ordinal_record_field_last_line_presence_v1
{
    pub presence: u32,
    pub value: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_related_entity_edge_properties_source_fields_file_kind_name_ordinal_record_field_record_presence_v1
{
    pub presence: u32,
    pub value: *const thinkthen_complete_json_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_related_entity_edge_properties_source_fields_file_kind_name_ordinal_record_v1 {
    pub file: thinkthen_complete_related_entity_edge_properties_source_fields_file_kind_name_ordinal_record_field_file_presence_v1,
    pub first_line: thinkthen_complete_related_entity_edge_properties_source_fields_file_kind_name_ordinal_record_field_first_line_presence_v1,
    pub kind: thinkthen_complete_utf8_v1,
    pub last_line: thinkthen_complete_related_entity_edge_properties_source_fields_file_kind_name_ordinal_record_field_last_line_presence_v1,
    pub name: thinkthen_complete_utf8_v1,
    pub ordinal: u64,
    pub record: thinkthen_complete_related_entity_edge_properties_source_fields_file_kind_name_ordinal_record_field_record_presence_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_related_entity_edge_properties_source_fields_file_kind_name_ordinal_record_v1(node: &Node, store: &mut Storage) -> Result<thinkthen_complete_related_entity_edge_properties_source_fields_file_kind_name_ordinal_record_v1, ErrorKind>{
    Ok(thinkthen_complete_related_entity_edge_properties_source_fields_file_kind_name_ordinal_record_v1 {
        file: match node.member("file") {
            None => return Err(ErrorKind::Defect),
            Some(value) if value.kind() == "null" => thinkthen_complete_related_entity_edge_properties_source_fields_file_kind_name_ordinal_record_field_file_presence_v1 { presence: THINKTHEN_COMPLETE_PRESENCE_NULL_V1, value: zero() },
            Some(value) => thinkthen_complete_related_entity_edge_properties_source_fields_file_kind_name_ordinal_record_field_file_presence_v1 { presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1, value: store.text(value.text()?) },
        },
        first_line: match node.member("first_line") {
            None => thinkthen_complete_related_entity_edge_properties_source_fields_file_kind_name_ordinal_record_field_first_line_presence_v1 { presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1, value: zero() },
            Some(value) => thinkthen_complete_related_entity_edge_properties_source_fields_file_kind_name_ordinal_record_field_first_line_presence_v1 { presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1, value: value.number()? },
        },
        kind: { let value = node.required("kind")?; store.text(value.text()?) },
        last_line: match node.member("last_line") {
            None => thinkthen_complete_related_entity_edge_properties_source_fields_file_kind_name_ordinal_record_field_last_line_presence_v1 { presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1, value: zero() },
            Some(value) => thinkthen_complete_related_entity_edge_properties_source_fields_file_kind_name_ordinal_record_field_last_line_presence_v1 { presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1, value: value.number()? },
        },
        name: { let value = node.required("name")?; store.text(value.text()?) },
        ordinal: { let value = node.required("ordinal")?; value.number()? },
        record: match node.member("record") {
            None => return Err(ErrorKind::Defect),
            Some(value) if value.kind() == "null" => thinkthen_complete_related_entity_edge_properties_source_fields_file_kind_name_ordinal_record_field_record_presence_v1 { presence: THINKTHEN_COMPLETE_PRESENCE_NULL_V1, value: zero() },
            Some(value) => thinkthen_complete_related_entity_edge_properties_source_fields_file_kind_name_ordinal_record_field_record_presence_v1 { presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1, value: read_json(value, store)? },
        },
        extensions: store.extensions(node, &["file", "first_line", "kind", "last_line", "name", "ordinal", "record"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_related_entity_edge_properties_source_fields_kind_name_v1 {
    pub kind: thinkthen_complete_utf8_v1,
    pub name: thinkthen_complete_utf8_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_related_entity_edge_properties_source_fields_kind_name_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_related_entity_edge_properties_source_fields_kind_name_v1, ErrorKind>
{
    Ok(
        thinkthen_complete_related_entity_edge_properties_source_fields_kind_name_v1 {
            kind: {
                let value = node.required("kind")?;
                store.text(value.text()?)
            },
            name: {
                let value = node.required("name")?;
                store.text(value.text()?)
            },
            extensions: store.extensions(node, &["kind", "name"])?,
        },
    )
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_relation_rule_field_single_presence_v1 {
    pub presence: u32,
    pub value: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_relation_rule_v1 {
    pub either: u32,
    pub name: thinkthen_complete_utf8_v1,
    pub reads: thinkthen_complete_utf8_v1,
    pub single: thinkthen_complete_relation_rule_field_single_presence_v1,
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
            value.boolean()?
        },
        name: {
            let value = node.required("name")?;
            store.text(value.text()?)
        },
        reads: {
            let value = node.required("reads")?;
            store.text(value.text()?)
        },
        single: match node.member("single") {
            None => thinkthen_complete_relation_rule_field_single_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_relation_rule_field_single_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: value.boolean()?,
            },
        },
        source: {
            let value = node.required("source")?;
            store.text(value.text()?)
        },
        target: {
            let value = node.required("target")?;
            store.text(value.text()?)
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
            store.text(value.text()?)
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
            value.number()?
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
pub struct thinkthen_complete_session_judgment_choice_field_value_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_utf8_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_judgment_choice_v1 {
    pub kind: thinkthen_complete_utf8_v1,
    pub value: thinkthen_complete_session_judgment_choice_field_value_presence_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_session_judgment_choice_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_judgment_choice_v1, ErrorKind> {
    Ok(thinkthen_complete_session_judgment_choice_v1 {
        kind: {
            let value = node.required("kind")?;
            store.text(value.text()?)
        },
        value: match node.member("value") {
            None => return Err(ErrorKind::Defect),
            Some(value) if value.kind() == "null" => {
                thinkthen_complete_session_judgment_choice_field_value_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_NULL_V1,
                    value: zero(),
                }
            }
            Some(value) => thinkthen_complete_session_judgment_choice_field_value_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: store.text(value.text()?),
            },
        },
        extensions: store.extensions(node, &["kind", "value"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_judgment_decision_field_value_presence_v1 {
    pub presence: u32,
    pub value: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_judgment_decision_v1 {
    pub kind: thinkthen_complete_utf8_v1,
    pub value: thinkthen_complete_session_judgment_decision_field_value_presence_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_session_judgment_decision_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_judgment_decision_v1, ErrorKind> {
    Ok(thinkthen_complete_session_judgment_decision_v1 {
        kind: {
            let value = node.required("kind")?;
            store.text(value.text()?)
        },
        value: match node.member("value") {
            None => return Err(ErrorKind::Defect),
            Some(value) if value.kind() == "null" => {
                thinkthen_complete_session_judgment_decision_field_value_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_NULL_V1,
                    value: zero(),
                }
            }
            Some(value) => thinkthen_complete_session_judgment_decision_field_value_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: value.boolean()?,
            },
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
            store.text(value.text()?)
        },
        value: {
            let value = node.required("value")?;
            value.number()?
        },
        extensions: store.extensions(node, &["kind", "value"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_judgment_tags_field_value_v1 {
    pub data: *const thinkthen_complete_utf8_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_session_judgment_tags_field_value_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_judgment_tags_field_value_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(store.text(item.text()?));
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_session_judgment_tags_field_value_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_judgment_tags_v1 {
    pub kind: thinkthen_complete_utf8_v1,
    pub value: thinkthen_complete_session_judgment_tags_field_value_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_session_judgment_tags_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_judgment_tags_v1, ErrorKind> {
    Ok(thinkthen_complete_session_judgment_tags_v1 {
        kind: {
            let value = node.required("kind")?;
            store.text(value.text()?)
        },
        value: {
            let value = node.required("value")?;
            read_thinkthen_complete_session_judgment_tags_field_value_v1(value, store)?
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
            store.text(value.text()?)
        },
        probability: {
            let value = node.required("probability")?;
            value.number()?
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
pub struct thinkthen_complete_session_observation_question_field_member_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_utf8_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_observation_question_field_stage_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_utf8_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_observation_question_v1 {
    pub detail: *const thinkthen_complete_session_question_detail_v1,
    pub index: u64,
    pub kind: thinkthen_complete_utf8_v1,
    pub member: thinkthen_complete_session_observation_question_field_member_presence_v1,
    pub position: u64,
    pub stage: thinkthen_complete_session_observation_question_field_stage_presence_v1,
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
            value.number()?
        },
        kind: {
            let value = node.required("kind")?;
            store.text(value.text()?)
        },
        member: match node.member("member") {
            None => thinkthen_complete_session_observation_question_field_member_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => {
                thinkthen_complete_session_observation_question_field_member_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                    value: store.text(value.text()?),
                }
            }
        },
        position: {
            let value = node.required("position")?;
            value.number()?
        },
        stage: match node.member("stage") {
            None => thinkthen_complete_session_observation_question_field_stage_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => {
                thinkthen_complete_session_observation_question_field_stage_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                    value: store.text(value.text()?),
                }
            }
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
            value.number()?
        },
        kind: {
            let value = node.required("kind")?;
            store.text(value.text()?)
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
pub struct thinkthen_complete_session_observed_row_annotated_field_value_v1 {
    pub data: *const *const thinkthen_complete_session_annotation_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_session_observed_row_annotated_field_value_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_observed_row_annotated_field_value_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_session_annotation_v1(item, store)
                .map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_session_observed_row_annotated_field_value_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_observed_row_annotated_v1 {
    pub kind: thinkthen_complete_utf8_v1,
    pub value: thinkthen_complete_session_observed_row_annotated_field_value_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_session_observed_row_annotated_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_observed_row_annotated_v1, ErrorKind> {
    Ok(thinkthen_complete_session_observed_row_annotated_v1 {
        kind: {
            let value = node.required("kind")?;
            store.text(value.text()?)
        },
        value: {
            let value = node.required("value")?;
            read_thinkthen_complete_session_observed_row_annotated_field_value_v1(value, store)?
        },
        extensions: store.extensions(node, &["kind", "value"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_observed_row_find_field_value_presence_v1 {
    pub presence: u32,
    pub value: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_observed_row_find_v1 {
    pub kind: thinkthen_complete_utf8_v1,
    pub value: thinkthen_complete_session_observed_row_find_field_value_presence_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_session_observed_row_find_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_observed_row_find_v1, ErrorKind> {
    Ok(thinkthen_complete_session_observed_row_find_v1 {
        kind: {
            let value = node.required("kind")?;
            store.text(value.text()?)
        },
        value: match node.member("value") {
            None => return Err(ErrorKind::Defect),
            Some(value) if value.kind() == "null" => {
                thinkthen_complete_session_observed_row_find_field_value_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_NULL_V1,
                    value: zero(),
                }
            }
            Some(value) => thinkthen_complete_session_observed_row_find_field_value_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: value.number()?,
            },
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
            store.text(value.text()?)
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
            store.text(value.text()?)
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
pub struct thinkthen_complete_session_observed_row_relations_field_value_v1 {
    pub data: *const *const thinkthen_complete_session_relation_edge_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_session_observed_row_relations_field_value_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_observed_row_relations_field_value_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_session_relation_edge_v1(item, store)
                .map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_session_observed_row_relations_field_value_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_observed_row_relations_v1 {
    pub kind: thinkthen_complete_utf8_v1,
    pub value: thinkthen_complete_session_observed_row_relations_field_value_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_session_observed_row_relations_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_observed_row_relations_v1, ErrorKind> {
    Ok(thinkthen_complete_session_observed_row_relations_v1 {
        kind: {
            let value = node.required("kind")?;
            store.text(value.text()?)
        },
        value: {
            let value = node.required("value")?;
            read_thinkthen_complete_session_observed_row_relations_field_value_v1(value, store)?
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
pub struct thinkthen_complete_session_packet_annotate_aggregate_field_value_v1 {
    pub data: *const *const thinkthen_complete_annotation_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_session_packet_annotate_aggregate_field_value_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_packet_annotate_aggregate_field_value_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_annotation_v1(item, store).map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_session_packet_annotate_aggregate_field_value_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_packet_annotate_aggregate_v1 {
    pub function: thinkthen_complete_utf8_v1,
    pub kind: thinkthen_complete_utf8_v1,
    pub value: thinkthen_complete_session_packet_annotate_aggregate_field_value_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_session_packet_annotate_aggregate_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_packet_annotate_aggregate_v1, ErrorKind> {
    Ok(thinkthen_complete_session_packet_annotate_aggregate_v1 {
        function: {
            let value = node.required("function")?;
            store.text(value.text()?)
        },
        kind: {
            let value = node.required("kind")?;
            store.text(value.text()?)
        },
        value: {
            let value = node.required("value")?;
            read_thinkthen_complete_session_packet_annotate_aggregate_field_value_v1(value, store)?
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
            store.text(value.text()?)
        },
        kind: {
            let value = node.required("kind")?;
            store.text(value.text()?)
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
pub struct thinkthen_complete_session_packet_choose_aggregate_field_value_v1 {
    pub data: *const *const thinkthen_complete_atomic_nullable_string_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_session_packet_choose_aggregate_field_value_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_packet_choose_aggregate_field_value_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_atomic_nullable_string_v1(item, store)
                .map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_session_packet_choose_aggregate_field_value_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_packet_choose_aggregate_v1 {
    pub function: thinkthen_complete_utf8_v1,
    pub kind: thinkthen_complete_utf8_v1,
    pub value: thinkthen_complete_session_packet_choose_aggregate_field_value_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_session_packet_choose_aggregate_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_packet_choose_aggregate_v1, ErrorKind> {
    Ok(thinkthen_complete_session_packet_choose_aggregate_v1 {
        function: {
            let value = node.required("function")?;
            store.text(value.text()?)
        },
        kind: {
            let value = node.required("kind")?;
            store.text(value.text()?)
        },
        value: {
            let value = node.required("value")?;
            read_thinkthen_complete_session_packet_choose_aggregate_field_value_v1(value, store)?
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
            store.text(value.text()?)
        },
        kind: {
            let value = node.required("kind")?;
            store.text(value.text()?)
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
pub struct thinkthen_complete_session_packet_decide_aggregate_field_value_v1 {
    pub data: *const *const thinkthen_complete_atomic_decide_value_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_session_packet_decide_aggregate_field_value_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_packet_decide_aggregate_field_value_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_atomic_decide_value_v1(item, store)
                .map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_session_packet_decide_aggregate_field_value_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_packet_decide_aggregate_v1 {
    pub function: thinkthen_complete_utf8_v1,
    pub kind: thinkthen_complete_utf8_v1,
    pub value: thinkthen_complete_session_packet_decide_aggregate_field_value_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_session_packet_decide_aggregate_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_packet_decide_aggregate_v1, ErrorKind> {
    Ok(thinkthen_complete_session_packet_decide_aggregate_v1 {
        function: {
            let value = node.required("function")?;
            store.text(value.text()?)
        },
        kind: {
            let value = node.required("kind")?;
            store.text(value.text()?)
        },
        value: {
            let value = node.required("value")?;
            read_thinkthen_complete_session_packet_decide_aggregate_field_value_v1(value, store)?
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
            store.text(value.text()?)
        },
        kind: {
            let value = node.required("kind")?;
            store.text(value.text()?)
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
pub struct thinkthen_complete_session_packet_filter_aggregate_field_value_v1 {
    pub data: *const *const thinkthen_complete_atomic_boolean_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_session_packet_filter_aggregate_field_value_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_packet_filter_aggregate_field_value_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_atomic_boolean_v1(item, store)
                .map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_session_packet_filter_aggregate_field_value_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_packet_filter_aggregate_v1 {
    pub function: thinkthen_complete_utf8_v1,
    pub kind: thinkthen_complete_utf8_v1,
    pub value: thinkthen_complete_session_packet_filter_aggregate_field_value_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_session_packet_filter_aggregate_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_packet_filter_aggregate_v1, ErrorKind> {
    Ok(thinkthen_complete_session_packet_filter_aggregate_v1 {
        function: {
            let value = node.required("function")?;
            store.text(value.text()?)
        },
        kind: {
            let value = node.required("kind")?;
            store.text(value.text()?)
        },
        value: {
            let value = node.required("value")?;
            read_thinkthen_complete_session_packet_filter_aggregate_field_value_v1(value, store)?
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
            store.text(value.text()?)
        },
        kind: {
            let value = node.required("kind")?;
            store.text(value.text()?)
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
            store.text(value.text()?)
        },
        kind: {
            let value = node.required("kind")?;
            store.text(value.text()?)
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
            store.text(value.text()?)
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
pub struct thinkthen_complete_session_packet_rank_aggregate_field_value_v1 {
    pub data: *const *const thinkthen_complete_atomic_non_zero_usize_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_session_packet_rank_aggregate_field_value_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_packet_rank_aggregate_field_value_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_atomic_non_zero_usize_v1(item, store)
                .map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_session_packet_rank_aggregate_field_value_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_packet_rank_aggregate_v1 {
    pub function: thinkthen_complete_utf8_v1,
    pub kind: thinkthen_complete_utf8_v1,
    pub value: thinkthen_complete_session_packet_rank_aggregate_field_value_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_session_packet_rank_aggregate_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_packet_rank_aggregate_v1, ErrorKind> {
    Ok(thinkthen_complete_session_packet_rank_aggregate_v1 {
        function: {
            let value = node.required("function")?;
            store.text(value.text()?)
        },
        kind: {
            let value = node.required("kind")?;
            store.text(value.text()?)
        },
        value: {
            let value = node.required("value")?;
            read_thinkthen_complete_session_packet_rank_aggregate_field_value_v1(value, store)?
        },
        extensions: store.extensions(node, &["function", "kind", "value"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_packet_recognize_aggregate_field_value_v1 {
    pub data: *const *const thinkthen_complete_recognition_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_session_packet_recognize_aggregate_field_value_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_packet_recognize_aggregate_field_value_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_recognition_v1(item, store).map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_session_packet_recognize_aggregate_field_value_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_packet_recognize_aggregate_v1 {
    pub function: thinkthen_complete_utf8_v1,
    pub kind: thinkthen_complete_utf8_v1,
    pub value: thinkthen_complete_session_packet_recognize_aggregate_field_value_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_session_packet_recognize_aggregate_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_packet_recognize_aggregate_v1, ErrorKind> {
    Ok(thinkthen_complete_session_packet_recognize_aggregate_v1 {
        function: {
            let value = node.required("function")?;
            store.text(value.text()?)
        },
        kind: {
            let value = node.required("kind")?;
            store.text(value.text()?)
        },
        value: {
            let value = node.required("value")?;
            read_thinkthen_complete_session_packet_recognize_aggregate_field_value_v1(value, store)?
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
            store.text(value.text()?)
        },
        kind: {
            let value = node.required("kind")?;
            store.text(value.text()?)
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
pub struct thinkthen_complete_session_packet_score_aggregate_field_value_v1 {
    pub data: *const *const thinkthen_complete_atomic_double_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_session_packet_score_aggregate_field_value_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_packet_score_aggregate_field_value_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_atomic_double_v1(item, store).map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_session_packet_score_aggregate_field_value_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_packet_score_aggregate_v1 {
    pub function: thinkthen_complete_utf8_v1,
    pub kind: thinkthen_complete_utf8_v1,
    pub value: thinkthen_complete_session_packet_score_aggregate_field_value_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_session_packet_score_aggregate_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_packet_score_aggregate_v1, ErrorKind> {
    Ok(thinkthen_complete_session_packet_score_aggregate_v1 {
        function: {
            let value = node.required("function")?;
            store.text(value.text()?)
        },
        kind: {
            let value = node.required("kind")?;
            store.text(value.text()?)
        },
        value: {
            let value = node.required("value")?;
            read_thinkthen_complete_session_packet_score_aggregate_field_value_v1(value, store)?
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
            store.text(value.text()?)
        },
        kind: {
            let value = node.required("kind")?;
            store.text(value.text()?)
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
pub struct thinkthen_complete_session_packet_tag_aggregate_field_value_v1 {
    pub data: *const *const thinkthen_complete_atomic_array_of_string_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_session_packet_tag_aggregate_field_value_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_packet_tag_aggregate_field_value_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_atomic_array_of_string_v1(item, store)
                .map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_session_packet_tag_aggregate_field_value_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_packet_tag_aggregate_v1 {
    pub function: thinkthen_complete_utf8_v1,
    pub kind: thinkthen_complete_utf8_v1,
    pub value: thinkthen_complete_session_packet_tag_aggregate_field_value_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_session_packet_tag_aggregate_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_packet_tag_aggregate_v1, ErrorKind> {
    Ok(thinkthen_complete_session_packet_tag_aggregate_v1 {
        function: {
            let value = node.required("function")?;
            store.text(value.text()?)
        },
        kind: {
            let value = node.required("kind")?;
            store.text(value.text()?)
        },
        value: {
            let value = node.required("value")?;
            read_thinkthen_complete_session_packet_tag_aggregate_field_value_v1(value, store)?
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
            store.text(value.text()?)
        },
        kind: {
            let value = node.required("kind")?;
            store.text(value.text()?)
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
pub struct thinkthen_complete_session_packet_terminal_field_facts_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_facts_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_packet_terminal_field_failure_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_call_error_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_packet_terminal_v1 {
    pub facts: thinkthen_complete_session_packet_terminal_field_facts_presence_v1,
    pub failure: thinkthen_complete_session_packet_terminal_field_failure_presence_v1,
    pub kind: thinkthen_complete_utf8_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_session_packet_terminal_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_packet_terminal_v1, ErrorKind> {
    Ok(thinkthen_complete_session_packet_terminal_v1 {
        facts: match node.member("facts") {
            None => thinkthen_complete_session_packet_terminal_field_facts_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_session_packet_terminal_field_facts_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_facts_v1(value, store)
                    .map(|value| store.hold(value))?,
            },
        },
        failure: match node.member("failure") {
            None => thinkthen_complete_session_packet_terminal_field_failure_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_session_packet_terminal_field_failure_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_call_error_v1(value, store)
                    .map(|value| store.hold(value))?,
            },
        },
        kind: {
            let value = node.required("kind")?;
            store.text(value.text()?)
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
pub struct thinkthen_complete_session_probabilities_named_field_value_v1 {
    pub data: *const *const thinkthen_complete_session_named_probability_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_session_probabilities_named_field_value_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_probabilities_named_field_value_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_session_named_probability_v1(item, store)
                .map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_session_probabilities_named_field_value_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_probabilities_named_v1 {
    pub kind: thinkthen_complete_utf8_v1,
    pub value: thinkthen_complete_session_probabilities_named_field_value_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_session_probabilities_named_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_probabilities_named_v1, ErrorKind> {
    Ok(thinkthen_complete_session_probabilities_named_v1 {
        kind: {
            let value = node.required("kind")?;
            store.text(value.text()?)
        },
        value: {
            let value = node.required("value")?;
            read_thinkthen_complete_session_probabilities_named_field_value_v1(value, store)?
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
            store.text(value.text()?)
        },
        value: {
            let value = node.required("value")?;
            value.number()?
        },
        extensions: store.extensions(node, &["kind", "value"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_question_detail_field_answer_id_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_answer_id_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_question_detail_field_confidence_presence_v1 {
    pub presence: u32,
    pub value: f64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_question_detail_field_failure_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_failure_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_question_detail_field_failure_id_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_failure_id_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_question_detail_field_input_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_json_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_question_detail_field_input_source_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_physical_source_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_question_detail_field_input_sources_v1 {
    pub data: *const *const thinkthen_complete_session_input_source_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_session_question_detail_field_input_sources_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_question_detail_field_input_sources_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_session_input_source_v1(item, store)
                .map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_session_question_detail_field_input_sources_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_question_detail_field_inputs_v1 {
    pub data: *const *const thinkthen_complete_json_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_session_question_detail_field_inputs_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_question_detail_field_inputs_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(read_json(item, store)?);
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_session_question_detail_field_inputs_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_question_detail_field_observations_v1 {
    pub data: *const *const thinkthen_complete_observation_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_session_question_detail_field_observations_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_question_detail_field_observations_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_observation_v1(item, store).map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_session_question_detail_field_observations_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_question_detail_field_probabilities_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_session_probabilities_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_question_detail_field_question_sources_v1 {
    pub data: *const *const thinkthen_complete_question_source_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_session_question_detail_field_question_sources_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_question_detail_field_question_sources_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_question_source_v1(item, store)
                .map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_session_question_detail_field_question_sources_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_question_detail_field_raw_pick_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_utf8_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_question_detail_field_reported_usage_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_usage_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_question_detail_field_requests_v1 {
    pub data: *const thinkthen_complete_utf8_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_session_question_detail_field_requests_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_question_detail_field_requests_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(store.text(item.text()?));
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_session_question_detail_field_requests_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_question_detail_field_threshold_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_threshold_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_question_detail_field_usage_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_token_usage_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_question_detail_field_value_presence_v1 {
    pub presence: u32,
    pub value: *const thinkthen_complete_value_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_question_detail_v1 {
    pub answer_id: thinkthen_complete_session_question_detail_field_answer_id_presence_v1,
    pub cached: u32,
    pub confidence: thinkthen_complete_session_question_detail_field_confidence_presence_v1,
    pub failed_questions: u64,
    pub failure: thinkthen_complete_session_question_detail_field_failure_presence_v1,
    pub failure_id: thinkthen_complete_session_question_detail_field_failure_id_presence_v1,
    pub input: thinkthen_complete_session_question_detail_field_input_presence_v1,
    pub input_source: thinkthen_complete_session_question_detail_field_input_source_presence_v1,
    pub input_sources: thinkthen_complete_session_question_detail_field_input_sources_v1,
    pub inputs: thinkthen_complete_session_question_detail_field_inputs_v1,
    pub model: thinkthen_complete_utf8_v1,
    pub observations: thinkthen_complete_session_question_detail_field_observations_v1,
    pub probabilities: thinkthen_complete_session_question_detail_field_probabilities_presence_v1,
    pub question: *const thinkthen_complete_readable_question_v1,
    pub question_sha256: thinkthen_complete_utf8_v1,
    pub question_sources: thinkthen_complete_session_question_detail_field_question_sources_v1,
    pub raw_pick: thinkthen_complete_session_question_detail_field_raw_pick_presence_v1,
    pub reported_usage: thinkthen_complete_session_question_detail_field_reported_usage_presence_v1,
    pub requests: thinkthen_complete_session_question_detail_field_requests_v1,
    pub requests_sent: u64,
    pub threshold: thinkthen_complete_session_question_detail_field_threshold_presence_v1,
    pub url: thinkthen_complete_utf8_v1,
    pub usage: thinkthen_complete_session_question_detail_field_usage_presence_v1,
    pub value: thinkthen_complete_session_question_detail_field_value_presence_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_session_question_detail_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_question_detail_v1, ErrorKind> {
    Ok(thinkthen_complete_session_question_detail_v1 {
        answer_id: match node.member("answer_id") {
            None => thinkthen_complete_session_question_detail_field_answer_id_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_session_question_detail_field_answer_id_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_answer_id_v1(value, store)
                    .map(|value| store.hold(value))?,
            },
        },
        cached: {
            let value = node.required("cached")?;
            value.boolean()?
        },
        confidence: match node.member("confidence") {
            None => thinkthen_complete_session_question_detail_field_confidence_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => {
                thinkthen_complete_session_question_detail_field_confidence_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                    value: value.number()?,
                }
            }
        },
        failed_questions: {
            let value = node.required("failed_questions")?;
            value.number()?
        },
        failure: match node.member("failure") {
            None => thinkthen_complete_session_question_detail_field_failure_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_session_question_detail_field_failure_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_failure_v1(value, store)
                    .map(|value| store.hold(value))?,
            },
        },
        failure_id: match node.member("failure_id") {
            None => thinkthen_complete_session_question_detail_field_failure_id_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => {
                thinkthen_complete_session_question_detail_field_failure_id_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                    value: read_thinkthen_complete_failure_id_v1(value, store)
                        .map(|value| store.hold(value))?,
                }
            }
        },
        input: match node.member("input") {
            None => thinkthen_complete_session_question_detail_field_input_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) if value.kind() == "null" => {
                thinkthen_complete_session_question_detail_field_input_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_NULL_V1,
                    value: zero(),
                }
            }
            Some(value) => thinkthen_complete_session_question_detail_field_input_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_json(value, store)?,
            },
        },
        input_source: match node.member("input_source") {
            None => thinkthen_complete_session_question_detail_field_input_source_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => {
                thinkthen_complete_session_question_detail_field_input_source_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                    value: read_thinkthen_complete_physical_source_v1(value, store)
                        .map(|value| store.hold(value))?,
                }
            }
        },
        input_sources: {
            let value = node.required("input_sources")?;
            read_thinkthen_complete_session_question_detail_field_input_sources_v1(value, store)?
        },
        inputs: {
            let value = node.required("inputs")?;
            read_thinkthen_complete_session_question_detail_field_inputs_v1(value, store)?
        },
        model: {
            let value = node.required("model")?;
            store.text(value.text()?)
        },
        observations: {
            let value = node.required("observations")?;
            read_thinkthen_complete_session_question_detail_field_observations_v1(value, store)?
        },
        probabilities: match node.member("probabilities") {
            None => thinkthen_complete_session_question_detail_field_probabilities_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => {
                thinkthen_complete_session_question_detail_field_probabilities_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                    value: read_thinkthen_complete_session_probabilities_v1(value, store)
                        .map(|value| store.hold(value))?,
                }
            }
        },
        question: {
            let value = node.required("question")?;
            read_thinkthen_complete_readable_question_v1(value, store)
                .map(|value| store.hold(value))?
        },
        question_sha256: {
            let value = node.required("question_sha256")?;
            store.text(value.text()?)
        },
        question_sources: {
            let value = node.required("question_sources")?;
            read_thinkthen_complete_session_question_detail_field_question_sources_v1(value, store)?
        },
        raw_pick: match node.member("raw_pick") {
            None => thinkthen_complete_session_question_detail_field_raw_pick_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_session_question_detail_field_raw_pick_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: store.text(value.text()?),
            },
        },
        reported_usage: match node.member("reported_usage") {
            None => thinkthen_complete_session_question_detail_field_reported_usage_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => {
                thinkthen_complete_session_question_detail_field_reported_usage_presence_v1 {
                    presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                    value: read_thinkthen_complete_usage_v1(value, store)
                        .map(|value| store.hold(value))?,
                }
            }
        },
        requests: {
            let value = node.required("requests")?;
            read_thinkthen_complete_session_question_detail_field_requests_v1(value, store)?
        },
        requests_sent: {
            let value = node.required("requests_sent")?;
            value.number()?
        },
        threshold: match node.member("threshold") {
            None => thinkthen_complete_session_question_detail_field_threshold_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_session_question_detail_field_threshold_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_threshold_v1(value, store)
                    .map(|value| store.hold(value))?,
            },
        },
        url: {
            let value = node.required("url")?;
            store.text(value.text()?)
        },
        usage: match node.member("usage") {
            None => thinkthen_complete_session_question_detail_field_usage_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_session_question_detail_field_usage_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_token_usage_v1(value, store)
                    .map(|value| store.hold(value))?,
            },
        },
        value: match node.member("value") {
            None => thinkthen_complete_session_question_detail_field_value_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_session_question_detail_field_value_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_value_v1(value, store)
                    .map(|value| store.hold(value))?,
            },
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
pub struct thinkthen_complete_session_recognition_field_entities_v1 {
    pub data: *const *const thinkthen_complete_entity_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_session_recognition_field_entities_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_recognition_field_entities_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(read_thinkthen_complete_entity_v1(item, store).map(|value| store.hold(value))?);
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_session_recognition_field_entities_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_recognition_field_proposals_v1 {
    pub data: *const *const thinkthen_complete_boundary_proposal_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_session_recognition_field_proposals_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_recognition_field_proposals_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_boundary_proposal_v1(item, store)
                .map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_session_recognition_field_proposals_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_recognition_field_proposals_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_session_recognition_field_proposals_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_recognition_field_relations_v1 {
    pub data: *const *const thinkthen_complete_recognition_edge_document_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_session_recognition_field_relations_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_recognition_field_relations_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(
            read_thinkthen_complete_recognition_edge_document_v1(item, store)
                .map(|value| store.hold(value))?,
        );
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_session_recognition_field_relations_v1 { data, len })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_recognition_field_relations_presence_v1 {
    pub presence: u32,
    pub value: thinkthen_complete_session_recognition_field_relations_v1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_session_recognition_v1 {
    pub entities: thinkthen_complete_session_recognition_field_entities_v1,
    pub mode: *const thinkthen_complete_recognition_mode_v1,
    pub proposals: thinkthen_complete_session_recognition_field_proposals_presence_v1,
    pub relations: thinkthen_complete_session_recognition_field_relations_presence_v1,
    pub extensions: thinkthen_complete_extensions_v1,
}

pub(crate) fn read_thinkthen_complete_session_recognition_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_session_recognition_v1, ErrorKind> {
    Ok(thinkthen_complete_session_recognition_v1 {
        entities: {
            let value = node.required("entities")?;
            read_thinkthen_complete_session_recognition_field_entities_v1(value, store)?
        },
        mode: {
            let value = node.required("mode")?;
            read_thinkthen_complete_recognition_mode_v1(value, store)
                .map(|value| store.hold(value))?
        },
        proposals: match node.member("proposals") {
            None => thinkthen_complete_session_recognition_field_proposals_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_session_recognition_field_proposals_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_session_recognition_field_proposals_v1(
                    value, store,
                )?,
            },
        },
        relations: match node.member("relations") {
            None => thinkthen_complete_session_recognition_field_relations_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1,
                value: zero(),
            },
            Some(value) => thinkthen_complete_session_recognition_field_relations_presence_v1 {
                presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
                value: read_thinkthen_complete_session_recognition_field_relations_v1(
                    value, store,
                )?,
            },
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
            value.boolean()?
        },
        probability: {
            let value = node.required("probability")?;
            value.number()?
        },
        relation: {
            let value = node.required("relation")?;
            store.text(value.text()?)
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
        let value = node.number()?;
        return Ok(thinkthen_complete_threshold_v1 {
            kind: THINKTHEN_COMPLETE_THRESHOLD_NUMBER_V1,
            data: thinkthen_complete_threshold_data_v1 { number: value },
        });
    }
    if node.kind() == "string" {
        let value = store.text(node.text()?);
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
            value.number()?
        },
        output_tokens: {
            let value = node.required("output_tokens")?;
            value.number()?
        },
        extensions: store.extensions(node, &["input_tokens", "output_tokens"])?,
    })
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_value_array_v1 {
    pub data: *const thinkthen_complete_utf8_v1,
    pub len: usize,
}

pub(crate) fn read_thinkthen_complete_value_array_v1(
    node: &Node,
    store: &mut Storage,
) -> Result<thinkthen_complete_value_array_v1, ErrorKind> {
    let mut values = Vec::new();
    for item in node.array()? {
        values.push(store.text(item.text()?));
    }
    let (data, len) = store.slice(values);
    Ok(thinkthen_complete_value_array_v1 { data, len })
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
    pub array: thinkthen_complete_value_array_v1,
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
        let value = node.boolean()?;
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
        let value = store.text(node.text()?);
        return Ok(thinkthen_complete_value_v1 {
            kind: THINKTHEN_COMPLETE_VALUE_STRING_V1,
            data: thinkthen_complete_value_data_v1 { string: value },
        });
    }
    if node.kind() == "array" {
        let value = read_thinkthen_complete_value_array_v1(node, store)?;
        return Ok(thinkthen_complete_value_v1 {
            kind: THINKTHEN_COMPLETE_VALUE_ARRAY_V1,
            data: thinkthen_complete_value_data_v1 { array: value },
        });
    }
    if node.kind() == "number" {
        let value = node.number()?;
        return Ok(thinkthen_complete_value_v1 {
            kind: THINKTHEN_COMPLETE_VALUE_NUMBER_V1,
            data: thinkthen_complete_value_data_v1 { number: value },
        });
    }
    Err(ErrorKind::Defect)
}
