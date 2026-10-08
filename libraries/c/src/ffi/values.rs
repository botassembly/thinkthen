//! Canonical C values. Descriptor discriminators remain u32 so unknown host values are validated safely.
/// C value `THINKTHEN_VERSION_MAJOR`.
pub const THINKTHEN_VERSION_MAJOR: i32 = 0;
/// C value `THINKTHEN_VERSION_MINOR`.
pub const THINKTHEN_VERSION_MINOR: i32 = 2;
/// C value `THINKTHEN_VERSION_PATCH`.
pub const THINKTHEN_VERSION_PATCH: i32 = 0;
/// C value `THINKTHEN_OK`.
pub const THINKTHEN_OK: i32 = 0;
/// C value `THINKTHEN_EUSAGE`.
pub const THINKTHEN_EUSAGE: i32 = 1;
/// C value `THINKTHEN_EBACKEND`.
pub const THINKTHEN_EBACKEND: i32 = 2;
/// C value `THINKTHEN_EDEADLINE`.
pub const THINKTHEN_EDEADLINE: i32 = 3;
/// C value `THINKTHEN_ELOCAL`.
pub const THINKTHEN_ELOCAL: i32 = 4;
/// C value `THINKTHEN_ECANCELLED`.
pub const THINKTHEN_ECANCELLED: i32 = 5;
/// C value `THINKTHEN_EDEFECT`.
pub const THINKTHEN_EDEFECT: i32 = 6;
/// C value `THINKTHEN_NO_DEADLINE`.
pub const THINKTHEN_NO_DEADLINE: i64 = -1i64;
/// C value `THINKTHEN_YES`.
pub const THINKTHEN_YES: i32 = 1;
/// C value `THINKTHEN_NO`.
pub const THINKTHEN_NO: i32 = 0;
/// C value `THINKTHEN_UNSURE`.
pub const THINKTHEN_UNSURE: i32 = 2;
/// C value `THINKTHEN_CONTENT_TEXT_V1`.
pub const THINKTHEN_CONTENT_TEXT_V1: u32 = 1u32;
/// C value `THINKTHEN_CONTENT_JSON_V1`.
pub const THINKTHEN_CONTENT_JSON_V1: u32 = 2u32;
/// C value `THINKTHEN_RULE_DEFAULT_V1`.
pub const THINKTHEN_RULE_DEFAULT_V1: u32 = 0u32;
/// C value `THINKTHEN_RULE_NULL_V1`.
pub const THINKTHEN_RULE_NULL_V1: u32 = 1u32;
/// C value `THINKTHEN_RULE_CUT_V1`.
pub const THINKTHEN_RULE_CUT_V1: u32 = 2u32;
/// C value `THINKTHEN_RULE_BAND_V1`.
pub const THINKTHEN_RULE_BAND_V1: u32 = 3u32;
/// C value `THINKTHEN_FUNCTION_DECIDE_V1`.
pub const THINKTHEN_FUNCTION_DECIDE_V1: u32 = 1u32;
/// C value `THINKTHEN_FUNCTION_CHOOSE_V1`.
pub const THINKTHEN_FUNCTION_CHOOSE_V1: u32 = 2u32;
/// C value `THINKTHEN_FUNCTION_TAG_V1`.
pub const THINKTHEN_FUNCTION_TAG_V1: u32 = 3u32;
/// C value `THINKTHEN_FUNCTION_SCORE_V1`.
pub const THINKTHEN_FUNCTION_SCORE_V1: u32 = 4u32;
/// C value `THINKTHEN_FUNCTION_FILTER_V1`.
pub const THINKTHEN_FUNCTION_FILTER_V1: u32 = 5u32;
/// C value `THINKTHEN_FUNCTION_RANK_V1`.
pub const THINKTHEN_FUNCTION_RANK_V1: u32 = 6u32;
/// C value `THINKTHEN_FUNCTION_FIND_V1`.
pub const THINKTHEN_FUNCTION_FIND_V1: u32 = 7u32;
/// C value `THINKTHEN_FUNCTION_ANNOTATE_V1`.
pub const THINKTHEN_FUNCTION_ANNOTATE_V1: u32 = 8u32;
/// C value `THINKTHEN_FUNCTION_RECOGNIZE_V1`.
pub const THINKTHEN_FUNCTION_RECOGNIZE_V1: u32 = 9u32;
/// C value `THINKTHEN_FUNCTION_RELATE_V1`.
pub const THINKTHEN_FUNCTION_RELATE_V1: u32 = 10u32;
/// C value `THINKTHEN_IMAGE_JPEG_V1`.
pub const THINKTHEN_IMAGE_JPEG_V1: u32 = 1u32;
/// C value `THINKTHEN_IMAGE_PNG_V1`.
pub const THINKTHEN_IMAGE_PNG_V1: u32 = 2u32;
/// C value `THINKTHEN_SOURCE_LINE_V1`.
pub const THINKTHEN_SOURCE_LINE_V1: u32 = 1u32;
/// C value `THINKTHEN_SOURCE_WINDOW_V1`.
pub const THINKTHEN_SOURCE_WINDOW_V1: u32 = 2u32;
/// C value `THINKTHEN_SOURCE_FILE_V1`.
pub const THINKTHEN_SOURCE_FILE_V1: u32 = 3u32;
/// C value `THINKTHEN_SOURCE_IMAGE_FILE_V1`.
pub const THINKTHEN_SOURCE_IMAGE_FILE_V1: u32 = 4u32;
/// C value `THINKTHEN_SOURCE_JSONL_V1`.
pub const THINKTHEN_SOURCE_JSONL_V1: u32 = 5u32;
/// C value `THINKTHEN_DECIDE_NULL_V1`.
pub const THINKTHEN_DECIDE_NULL_V1: u32 = 0u32;
/// C value `THINKTHEN_DECIDE_BOOLEAN_V1`.
pub const THINKTHEN_DECIDE_BOOLEAN_V1: u32 = 1u32;
/// C value `THINKTHEN_DECIDE_AUTHORED_V1`.
pub const THINKTHEN_DECIDE_AUTHORED_V1: u32 = 2u32;
/// C value `THINKTHEN_ANSWER_YES_NO_V1`.
pub const THINKTHEN_ANSWER_YES_NO_V1: u32 = 1u32;
/// C value `THINKTHEN_ANSWER_CHOICE_V1`.
pub const THINKTHEN_ANSWER_CHOICE_V1: u32 = 2u32;
/// C value `THINKTHEN_ANSWER_TAG_V1`.
pub const THINKTHEN_ANSWER_TAG_V1: u32 = 3u32;
/// C value `THINKTHEN_ANSWER_SCORE_V1`.
pub const THINKTHEN_ANSWER_SCORE_V1: u32 = 4u32;
/// C value `THINKTHEN_ANSWER_FIND_V1`.
pub const THINKTHEN_ANSWER_FIND_V1: u32 = 5u32;
/// C value `THINKTHEN_MEMBER_SUCCESS_V1`.
pub const THINKTHEN_MEMBER_SUCCESS_V1: u32 = 1u32;
/// C value `THINKTHEN_MEMBER_FAILURE_V1`.
pub const THINKTHEN_MEMBER_FAILURE_V1: u32 = 2u32;
/// C value `THINKTHEN_MEMBER_MISSING_ANSWER_V1`.
pub const THINKTHEN_MEMBER_MISSING_ANSWER_V1: u32 = 1u32;
/// C value `THINKTHEN_MEMBER_WRONG_KIND_V1`.
pub const THINKTHEN_MEMBER_WRONG_KIND_V1: u32 = 2u32;
/// C value `THINKTHEN_MEMBER_MISSING_PROBABILITY_V1`.
pub const THINKTHEN_MEMBER_MISSING_PROBABILITY_V1: u32 = 3u32;
/// C value `THINKTHEN_MEMBER_INVALID_PROBABILITY_V1`.
pub const THINKTHEN_MEMBER_INVALID_PROBABILITY_V1: u32 = 4u32;
/// C value `THINKTHEN_MEMBER_INVALID_DISTRIBUTION_V1`.
pub const THINKTHEN_MEMBER_INVALID_DISTRIBUTION_V1: u32 = 5u32;
/// C value `THINKTHEN_MEMBER_UNEXPECTED_PROBABILITY_V1`.
pub const THINKTHEN_MEMBER_UNEXPECTED_PROBABILITY_V1: u32 = 6u32;
/// C value `THINKTHEN_RELATION_YES_NO_V1`.
pub const THINKTHEN_RELATION_YES_NO_V1: u32 = 1u32;
/// C value `THINKTHEN_RELATION_CHOICE_V1`.
pub const THINKTHEN_RELATION_CHOICE_V1: u32 = 2u32;
/// C value `THINKTHEN_DIRECTION_SOURCE_TO_TARGET_V1`.
pub const THINKTHEN_DIRECTION_SOURCE_TO_TARGET_V1: u32 = 1u32;
/// C value `THINKTHEN_DIRECTION_EITHER_V1`.
pub const THINKTHEN_DIRECTION_EITHER_V1: u32 = 2u32;
/// C value `THINKTHEN_ORIGIN_LIVE_V1`.
pub const THINKTHEN_ORIGIN_LIVE_V1: u32 = 1u32;
/// C value `THINKTHEN_ORIGIN_CACHE_V1`.
pub const THINKTHEN_ORIGIN_CACHE_V1: u32 = 2u32;
/// C value `THINKTHEN_ORIGIN_REPLAY_V1`.
pub const THINKTHEN_ORIGIN_REPLAY_V1: u32 = 3u32;
/// C value `THINKTHEN_ORIGIN_PROXY_V1`.
pub const THINKTHEN_ORIGIN_PROXY_V1: u32 = 4u32;
/// C value `THINKTHEN_ORIGIN_MEMORY_V1`.
pub const THINKTHEN_ORIGIN_MEMORY_V1: u32 = 5u32;
/// C value `THINKTHEN_ID_OBSERVATION_V1`.
pub const THINKTHEN_ID_OBSERVATION_V1: u32 = 1u32;
/// C value `THINKTHEN_ID_FAILURE_V1`.
pub const THINKTHEN_ID_FAILURE_V1: u32 = 2u32;
/// C value `THINKTHEN_BATCH_RECORDS_V1`.
pub const THINKTHEN_BATCH_RECORDS_V1: u32 = 1u32;
/// C value `THINKTHEN_BATCH_MAX_V1`.
pub const THINKTHEN_BATCH_MAX_V1: u32 = 2u32;
/// C value `THINKTHEN_ATTEMPT_OK_V1`.
pub const THINKTHEN_ATTEMPT_OK_V1: u32 = 1u32;
/// C value `THINKTHEN_ATTEMPT_STATUS_V1`.
pub const THINKTHEN_ATTEMPT_STATUS_V1: u32 = 2u32;
/// C value `THINKTHEN_ATTEMPT_TRANSPORT_V1`.
pub const THINKTHEN_ATTEMPT_TRANSPORT_V1: u32 = 3u32;
/// C value `THINKTHEN_STOP_USAGE_V1`.
pub const THINKTHEN_STOP_USAGE_V1: u32 = 1u32;
/// C value `THINKTHEN_STOP_LOCAL_V1`.
pub const THINKTHEN_STOP_LOCAL_V1: u32 = 2u32;
/// C value `THINKTHEN_STOP_NO_KEY_V1`.
pub const THINKTHEN_STOP_NO_KEY_V1: u32 = 3u32;
/// C value `THINKTHEN_STOP_TRANSPORT_V1`.
pub const THINKTHEN_STOP_TRANSPORT_V1: u32 = 4u32;
/// C value `THINKTHEN_STOP_STATUS_V1`.
pub const THINKTHEN_STOP_STATUS_V1: u32 = 5u32;
/// C value `THINKTHEN_STOP_TOO_LARGE_V1`.
pub const THINKTHEN_STOP_TOO_LARGE_V1: u32 = 6u32;
/// C value `THINKTHEN_STOP_REPLY_V1`.
pub const THINKTHEN_STOP_REPLY_V1: u32 = 7u32;
/// C value `THINKTHEN_STOP_BACKEND_V1`.
pub const THINKTHEN_STOP_BACKEND_V1: u32 = 8u32;
/// C value `THINKTHEN_STOP_CANCELLED_V1`.
pub const THINKTHEN_STOP_CANCELLED_V1: u32 = 9u32;
/// C value `THINKTHEN_STOP_DEFECT_V1`.
pub const THINKTHEN_STOP_DEFECT_V1: u32 = 10u32;
/// C value `THINKTHEN_STOP_DEADLINE_V1`.
pub const THINKTHEN_STOP_DEADLINE_V1: u32 = 11u32;
/// C value `THINKTHEN_STAGE_BOUNDARY_V1`.
pub const THINKTHEN_STAGE_BOUNDARY_V1: u32 = 1u32;
/// C value `THINKTHEN_STAGE_KIND_V1`.
pub const THINKTHEN_STAGE_KIND_V1: u32 = 2u32;
/// C value `THINKTHEN_STAGE_EDGE_V1`.
pub const THINKTHEN_STAGE_EDGE_V1: u32 = 3u32;
/// C value `THINKTHEN_STAGE_RELATION_V1`.
pub const THINKTHEN_STAGE_RELATION_V1: u32 = 4u32;
/// C value `THINKTHEN_PROBABILITIES_YES_V1`.
pub const THINKTHEN_PROBABILITIES_YES_V1: u32 = 1u32;
/// C value `THINKTHEN_PROBABILITIES_NAMED_V1`.
pub const THINKTHEN_PROBABILITIES_NAMED_V1: u32 = 2u32;
/// C value `THINKTHEN_EVENT_QUESTION_V1`.
pub const THINKTHEN_EVENT_QUESTION_V1: u32 = 1u32;
/// C value `THINKTHEN_EVENT_ROW_V1`.
pub const THINKTHEN_EVENT_ROW_V1: u32 = 2u32;
/// C value `THINKTHEN_RESULT_SUCCESS_V1`.
pub const THINKTHEN_RESULT_SUCCESS_V1: u32 = 1u32;
/// C value `THINKTHEN_RESULT_FAILURE_V1`.
pub const THINKTHEN_RESULT_FAILURE_V1: u32 = 2u32;
/// Admitted native grammar values; descriptor storage remains uint32_t.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[allow(
    non_camel_case_types,
    reason = "preserve the public C enumerator names"
)]
pub enum InputDeclarationKindV1 {
    /// C grammar value `THINKTHEN_DECLARATION_ABSENT_V1`.
    THINKTHEN_DECLARATION_ABSENT_V1 = 0,
    /// C grammar value `THINKTHEN_DECLARATION_STRING_V1`.
    THINKTHEN_DECLARATION_STRING_V1 = 1,
    /// C grammar value `THINKTHEN_DECLARATION_OBJECT_V1`.
    THINKTHEN_DECLARATION_OBJECT_V1 = 2,
}
/// Admitted native grammar values; descriptor storage remains uint32_t.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[allow(
    non_camel_case_types,
    reason = "preserve the public C enumerator names"
)]
pub enum InputPropertyKindV1 {
    /// C grammar value `THINKTHEN_PROPERTY_STRING_V1`.
    THINKTHEN_PROPERTY_STRING_V1 = 1,
    /// C grammar value `THINKTHEN_PROPERTY_NUMBER_V1`.
    THINKTHEN_PROPERTY_NUMBER_V1 = 2,
    /// C grammar value `THINKTHEN_PROPERTY_BOOLEAN_V1`.
    THINKTHEN_PROPERTY_BOOLEAN_V1 = 3,
    /// C grammar value `THINKTHEN_PROPERTY_STRING_LIST_V1`.
    THINKTHEN_PROPERTY_STRING_LIST_V1 = 4,
}
/// Admitted native grammar values; descriptor storage remains uint32_t.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[allow(
    non_camel_case_types,
    reason = "preserve the public C enumerator names"
)]
pub enum QuestionLoaderRoleV1 {
    /// C grammar value `THINKTHEN_LOAD_ATOMIC_V1`.
    THINKTHEN_LOAD_ATOMIC_V1 = 1,
    /// C grammar value `THINKTHEN_LOAD_SET_V1`.
    THINKTHEN_LOAD_SET_V1 = 2,
    /// C grammar value `THINKTHEN_LOAD_DYNAMIC_CHOOSE_V1`.
    THINKTHEN_LOAD_DYNAMIC_CHOOSE_V1 = 3,
    /// C grammar value `THINKTHEN_LOAD_RECOGNIZE_V1`.
    THINKTHEN_LOAD_RECOGNIZE_V1 = 4,
    /// C grammar value `THINKTHEN_LOAD_RELATE_V1`.
    THINKTHEN_LOAD_RELATE_V1 = 5,
    /// C grammar value `THINKTHEN_LOAD_RANK_V1`.
    THINKTHEN_LOAD_RANK_V1 = 6,
    /// C grammar value `THINKTHEN_LOAD_RANK_SET_V1`.
    THINKTHEN_LOAD_RANK_SET_V1 = 7,
    /// C grammar value `THINKTHEN_LOAD_FIND_V1`.
    THINKTHEN_LOAD_FIND_V1 = 8,
}

/// Validated uint32 descriptor spelling for `THINKTHEN_DECLARATION_ABSENT_V1`.
pub(crate) const DECLARATION_ABSENT_V1: u32 =
    InputDeclarationKindV1::THINKTHEN_DECLARATION_ABSENT_V1 as u32;

/// Validated uint32 descriptor spelling for `THINKTHEN_DECLARATION_STRING_V1`.
pub(crate) const DECLARATION_STRING_V1: u32 =
    InputDeclarationKindV1::THINKTHEN_DECLARATION_STRING_V1 as u32;

/// Validated uint32 descriptor spelling for `THINKTHEN_DECLARATION_OBJECT_V1`.
pub(crate) const DECLARATION_OBJECT_V1: u32 =
    InputDeclarationKindV1::THINKTHEN_DECLARATION_OBJECT_V1 as u32;

/// Validated uint32 descriptor spelling for `THINKTHEN_PROPERTY_STRING_V1`.
pub(crate) const PROPERTY_STRING_V1: u32 = InputPropertyKindV1::THINKTHEN_PROPERTY_STRING_V1 as u32;

/// Validated uint32 descriptor spelling for `THINKTHEN_PROPERTY_NUMBER_V1`.
pub(crate) const PROPERTY_NUMBER_V1: u32 = InputPropertyKindV1::THINKTHEN_PROPERTY_NUMBER_V1 as u32;

/// Validated uint32 descriptor spelling for `THINKTHEN_PROPERTY_BOOLEAN_V1`.
pub(crate) const PROPERTY_BOOLEAN_V1: u32 =
    InputPropertyKindV1::THINKTHEN_PROPERTY_BOOLEAN_V1 as u32;

/// Validated uint32 descriptor spelling for `THINKTHEN_PROPERTY_STRING_LIST_V1`.
pub(crate) const PROPERTY_STRING_LIST_V1: u32 =
    InputPropertyKindV1::THINKTHEN_PROPERTY_STRING_LIST_V1 as u32;

/// Validated uint32 descriptor spelling for `THINKTHEN_LOAD_ATOMIC_V1`.
pub(crate) const LOAD_ATOMIC_V1: u32 = QuestionLoaderRoleV1::THINKTHEN_LOAD_ATOMIC_V1 as u32;

/// Validated uint32 descriptor spelling for `THINKTHEN_LOAD_SET_V1`.
pub(crate) const LOAD_SET_V1: u32 = QuestionLoaderRoleV1::THINKTHEN_LOAD_SET_V1 as u32;

/// Validated uint32 descriptor spelling for `THINKTHEN_LOAD_DYNAMIC_CHOOSE_V1`.
pub(crate) const LOAD_DYNAMIC_CHOOSE_V1: u32 =
    QuestionLoaderRoleV1::THINKTHEN_LOAD_DYNAMIC_CHOOSE_V1 as u32;

/// Validated uint32 descriptor spelling for `THINKTHEN_LOAD_RECOGNIZE_V1`.
pub(crate) const LOAD_RECOGNIZE_V1: u32 = QuestionLoaderRoleV1::THINKTHEN_LOAD_RECOGNIZE_V1 as u32;

/// Validated uint32 descriptor spelling for `THINKTHEN_LOAD_RELATE_V1`.
pub(crate) const LOAD_RELATE_V1: u32 = QuestionLoaderRoleV1::THINKTHEN_LOAD_RELATE_V1 as u32;

/// Validated uint32 descriptor spelling for `THINKTHEN_LOAD_RANK_V1`.
pub(crate) const LOAD_RANK_V1: u32 = QuestionLoaderRoleV1::THINKTHEN_LOAD_RANK_V1 as u32;

/// Validated uint32 descriptor spelling for `THINKTHEN_LOAD_RANK_SET_V1`.
pub(crate) const LOAD_RANK_SET_V1: u32 = QuestionLoaderRoleV1::THINKTHEN_LOAD_RANK_SET_V1 as u32;

/// Validated uint32 descriptor spelling for `THINKTHEN_LOAD_FIND_V1`.
pub(crate) const LOAD_FIND_V1: u32 = QuestionLoaderRoleV1::THINKTHEN_LOAD_FIND_V1 as u32;
