// Generated counted carriers from the canonical request graph; do not edit.
#![allow(
    non_camel_case_types,
    missing_docs,
    clippy::too_many_lines,
    reason = "generated versioned request carriers and conversion"
)]
use super::{CarrierError, Context, Value, borrow, extent, optional};
use crate::session::views::{
    THINKTHEN_COMPLETE_JSON_ARRAY, THINKTHEN_COMPLETE_JSON_BOOLEAN, THINKTHEN_COMPLETE_JSON_NULL,
    THINKTHEN_COMPLETE_JSON_NUMBER, THINKTHEN_COMPLETE_JSON_OBJECT, THINKTHEN_COMPLETE_JSON_STRING,
    thinkthen_complete_json, thinkthen_complete_utf8,
};
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_engine_settings {
    pub m_backend: *const thinkthen_request_text,
    pub m_base_url: *const thinkthen_request_text,
    pub m_batch: *const thinkthen_request_batch,
    pub m_cache: *const thinkthen_request_cache_document,
    pub m_max_estimated_input_tokens_total: *const thinkthen_request_optional_total,
    pub m_max_request_bytes: *const thinkthen_request_unsigned,
    pub m_max_requests: *const thinkthen_request_optional_count,
    pub m_max_requests_total: *const thinkthen_request_optional_total,
    pub m_max_retries: *const thinkthen_request_unsigned,
    pub m_model: *const thinkthen_request_text,
    pub m_profile: *const thinkthen_request_text,
    pub m_proxy: *const thinkthen_request_json_value,
    pub m_record: *const thinkthen_request_text,
    pub m_refresh_cache: *const thinkthen_request_boolean,
    pub m_replay: *const thinkthen_request_text,
    pub m_throttle: *const thinkthen_request_unsigned,
    pub m_timeout: *const thinkthen_request_unsigned,
    pub m_usd_per_million_input: *const thinkthen_request_text,
    pub m_usd_per_million_output: *const thinkthen_request_text,
}
pub const THINKTHEN_REQUEST_CACHE_DOCUMENT_TEXT: u32 = 1;
pub const THINKTHEN_REQUEST_CACHE_DOCUMENT_DISABLED_CACHE: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_request_cache_document_data {
    pub text: *const thinkthen_request_text,
    pub disabled_cache: *const thinkthen_request_disabled_cache,
}
impl std::fmt::Debug for thinkthen_request_cache_document_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("request union payload")
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_cache_document {
    pub kind: u32,
    pub data: thinkthen_request_cache_document_data,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_disabled_cache {
    pub reserved: u32,
}
pub const THINKTHEN_REQUEST_BATCH_UNSIGNED_VALUE: u32 = 1;
pub const THINKTHEN_REQUEST_BATCH_TEXT: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_request_batch_data {
    pub unsigned_value: *const thinkthen_request_unsigned,
    pub text: *const thinkthen_request_text,
}
impl std::fmt::Debug for thinkthen_request_batch_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("request union payload")
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_batch {
    pub kind: u32,
    pub data: thinkthen_request_batch_data,
}
pub const THINKTHEN_REQUEST_READER_FAILURE_PRODUCER: u32 = 1;
pub const THINKTHEN_REQUEST_READER_FAILURE_IO: u32 = 2;
pub const THINKTHEN_REQUEST_READER_FAILURE_UTF_8: u32 = 3;
pub const THINKTHEN_REQUEST_READER_FAILURE_INVALID_INPUT: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_request_reader_failure_data {
    pub producer: *const thinkthen_request_reader_failure_producer,
    pub io: *const thinkthen_request_reader_failure_io,
    pub utf_8: *const thinkthen_request_reader_failure_utf_8,
    pub invalid_input: *const thinkthen_request_reader_failure_invalid_input,
}
impl std::fmt::Debug for thinkthen_request_reader_failure_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("request union payload")
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_reader_failure {
    pub kind: u32,
    pub data: thinkthen_request_reader_failure_data,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_reader_failure_invalid_input {
    pub m_location: *const thinkthen_request_session_source_location,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_session_source_location {
    pub m_file: *const thinkthen_request_text,
    pub m_first_line: *const thinkthen_request_unsigned,
    pub m_last_line: *const thinkthen_request_unsigned,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_reader_failure_utf_8 {
    pub m_location: *const thinkthen_request_session_source_location,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_reader_failure_io {
    pub m_location: *const thinkthen_request_session_source_location,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_reader_failure_producer {
    pub reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_session_descriptor {
    pub m_item: *const thinkthen_request_item,
    pub m_location: *const thinkthen_request_session_source_location,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_item {
    pub m_context: *const thinkthen_request_context_schema,
    pub m_examples: *const thinkthen_request_recognition_example_list,
    pub m_images: *const thinkthen_request_image_list,
    pub m_options: *const thinkthen_request_option_schema_list,
    pub m_original: *const thinkthen_request_original,
    pub m_seed_spans: *const thinkthen_request_recognition_seed_span_list,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_recognition_seed_span {
    pub m_end: *const thinkthen_request_unsigned,
    pub m_kind: *const thinkthen_request_text,
    pub m_start: *const thinkthen_request_unsigned,
}
pub const THINKTHEN_REQUEST_ORIGINAL_TEXT: u32 = 1;
pub const THINKTHEN_REQUEST_ORIGINAL_JSON: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_request_original_data {
    pub text: *const thinkthen_request_original_text,
    pub json: *const thinkthen_request_original_json,
}
impl std::fmt::Debug for thinkthen_request_original_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("request union payload")
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_original {
    pub kind: u32,
    pub data: thinkthen_request_original_data,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_original_json {
    pub m_value: *const thinkthen_request_original_json_value,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_original_text {
    pub m_text: *const thinkthen_request_text,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_option_schema {
    pub m_description: *const thinkthen_request_json_value,
    pub m_name: *const thinkthen_request_text,
}
pub const THINKTHEN_REQUEST_IMAGE_FILE: u32 = 1;
pub const THINKTHEN_REQUEST_IMAGE_BYTES: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_request_image_data {
    pub file: *const thinkthen_request_image_file,
    pub bytes: *const thinkthen_request_image_bytes,
}
impl std::fmt::Debug for thinkthen_request_image_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("request union payload")
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_image {
    pub kind: u32,
    pub data: thinkthen_request_image_data,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_image_bytes {
    pub m_bytes: *const thinkthen_request_text,
    pub m_media: *const thinkthen_request_image_media,
}
pub const THINKTHEN_REQUEST_IMAGE_MEDIA_LITERAL_IMAGE_JPEG: u32 = 1;
pub const THINKTHEN_REQUEST_IMAGE_MEDIA_LITERAL_IMAGE_PNG: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_request_image_media_data {
    pub literal_image_jpeg: *const thinkthen_request_literal_image_jpeg,
    pub literal_image_png: *const thinkthen_request_literal_image_png,
}
impl std::fmt::Debug for thinkthen_request_image_media_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("request union payload")
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_image_media {
    pub kind: u32,
    pub data: thinkthen_request_image_media_data,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_image_file {
    pub m_media: *const thinkthen_request_image_media,
    pub m_path: *const thinkthen_request_text,
}
pub const THINKTHEN_REQUEST_RECOGNITION_EXAMPLE_TEXT: u32 = 1;
pub const THINKTHEN_REQUEST_RECOGNITION_EXAMPLE_EXAMPLE_TEXT: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_request_recognition_example_data {
    pub text: *const thinkthen_request_text,
    pub example_text: *const thinkthen_request_recognition_example_text,
}
impl std::fmt::Debug for thinkthen_request_recognition_example_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("request union payload")
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_recognition_example {
    pub kind: u32,
    pub data: thinkthen_request_recognition_example_data,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_recognition_example_text {
    pub m_entities: *const thinkthen_request_recognition_example_entity_list,
    pub m_kinds: *const thinkthen_request_text_list,
    pub m_text: *const thinkthen_request_text,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_recognition_example_entity {
    pub m_end: *const thinkthen_request_unsigned,
    pub m_kind: *const thinkthen_request_text,
    pub m_start: *const thinkthen_request_unsigned,
}
pub const THINKTHEN_REQUEST_CONTEXT_SCHEMA_TEXT: u32 = 1;
pub const THINKTHEN_REQUEST_CONTEXT_SCHEMA_JSON_VALUE_MAP: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_request_context_schema_data {
    pub text: *const thinkthen_request_text,
    pub json_value_map: *const thinkthen_request_json_value_map,
}
impl std::fmt::Debug for thinkthen_request_context_schema_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("request union payload")
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_context_schema {
    pub kind: u32,
    pub data: thinkthen_request_context_schema_data,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_arguments {
    pub m_input: *const thinkthen_request_input,
    pub m_options: *const thinkthen_request_call_options,
    pub m_question: *const thinkthen_request_question_selector,
}
pub const THINKTHEN_REQUEST_QUESTION_SELECTOR_TEXT: u32 = 1;
pub const THINKTHEN_REQUEST_QUESTION_SELECTOR_DEFINITION: u32 = 2;
pub const THINKTHEN_REQUEST_QUESTION_SELECTOR_FILE: u32 = 3;
pub const THINKTHEN_REQUEST_QUESTION_SELECTOR_NAME: u32 = 4;
pub const THINKTHEN_REQUEST_QUESTION_SELECTOR_REFERENCE: u32 = 5;
#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_request_question_selector_data {
    pub text: *const thinkthen_request_question_selector_text,
    pub definition: *const thinkthen_request_question_selector_definition,
    pub file: *const thinkthen_request_question_selector_file,
    pub name: *const thinkthen_request_question_selector_name,
    pub reference: *const thinkthen_request_question_selector_reference,
}
impl std::fmt::Debug for thinkthen_request_question_selector_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("request union payload")
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_question_selector {
    pub kind: u32,
    pub data: thinkthen_request_question_selector_data,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_question_selector_reference {
    pub m_reference: *const thinkthen_request_text,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_question_selector_name {
    pub m_name: *const thinkthen_request_text,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_question_selector_file {
    pub m_path: *const thinkthen_request_text,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_question_selector_definition {
    pub m_value: *const thinkthen_request_definition,
}
pub const THINKTHEN_REQUEST_DEFINITION_AUTHORED_DECIDE: u32 = 1;
pub const THINKTHEN_REQUEST_DEFINITION_AUTHORED_CHOOSE: u32 = 2;
pub const THINKTHEN_REQUEST_DEFINITION_AUTHORED_TAG: u32 = 3;
pub const THINKTHEN_REQUEST_DEFINITION_AUTHORED_SCORE: u32 = 4;
pub const THINKTHEN_REQUEST_DEFINITION_AUTHORED_RELATE: u32 = 5;
pub const THINKTHEN_REQUEST_DEFINITION_AUTHORED_FIND: u32 = 6;
pub const THINKTHEN_REQUEST_DEFINITION_RECOGNIZE_DEFINITION: u32 = 7;
pub const THINKTHEN_REQUEST_DEFINITION_ANNOTATE_DEFINITION: u32 = 8;
#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_request_definition_data {
    pub authored_decide: *const thinkthen_request_definition_authored_decide,
    pub authored_choose: *const thinkthen_request_definition_authored_choose,
    pub authored_tag: *const thinkthen_request_definition_authored_tag,
    pub authored_score: *const thinkthen_request_definition_authored_score,
    pub authored_relate: *const thinkthen_request_definition_authored_relate,
    pub authored_find: *const thinkthen_request_definition_authored_find,
    pub recognize_definition: *const thinkthen_request_definition_recognize_definition,
    pub annotate_definition: *const thinkthen_request_definition_annotate_definition,
}
impl std::fmt::Debug for thinkthen_request_definition_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("request union payload")
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_definition {
    pub kind: u32,
    pub data: thinkthen_request_definition_data,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_definition_annotate_definition {
    pub m_batch: *const thinkthen_request_json_value,
    pub m_profile: *const thinkthen_request_profile,
    pub m_questions: *const thinkthen_request_annotate_question_map,
    pub m_threshold: *const thinkthen_request_threshold,
}
pub const THINKTHEN_REQUEST_THRESHOLD_NUMBER: u32 = 1;
pub const THINKTHEN_REQUEST_THRESHOLD_TEXT: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_request_threshold_data {
    pub number: *const thinkthen_request_number,
    pub text: *const thinkthen_request_text,
}
impl std::fmt::Debug for thinkthen_request_threshold_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("request union payload")
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_threshold {
    pub kind: u32,
    pub data: thinkthen_request_threshold_data,
}
pub const THINKTHEN_REQUEST_ANNOTATE_QUESTION_DECIDE: u32 = 1;
pub const THINKTHEN_REQUEST_ANNOTATE_QUESTION_CHOOSE: u32 = 2;
pub const THINKTHEN_REQUEST_ANNOTATE_QUESTION_TAG: u32 = 3;
pub const THINKTHEN_REQUEST_ANNOTATE_QUESTION_SCORE: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_request_annotate_question_data {
    pub decide: *const thinkthen_request_annotate_question_decide,
    pub choose: *const thinkthen_request_annotate_question_choose,
    pub tag: *const thinkthen_request_annotate_question_tag,
    pub score: *const thinkthen_request_annotate_question_score,
}
impl std::fmt::Debug for thinkthen_request_annotate_question_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("request union payload")
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_annotate_question {
    pub kind: u32,
    pub data: thinkthen_request_annotate_question_data,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_annotate_question_score {
    pub m_context_schema: *const thinkthen_request_input_declaration,
    pub m_item_schema: *const thinkthen_request_input_declaration,
    pub m_levels: *const thinkthen_request_levels,
    pub m_name: *const thinkthen_request_text,
    pub m_on: *const thinkthen_request_pointers,
    pub m_score: *const thinkthen_request_question_text,
}
pub const THINKTHEN_REQUEST_QUESTION_TEXT_TEXT: u32 = 1;
pub const THINKTHEN_REQUEST_QUESTION_TEXT_JSON_VALUE_MAP: u32 = 2;
pub const THINKTHEN_REQUEST_QUESTION_TEXT_JSON_VALUE_LIST: u32 = 3;
#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_request_question_text_data {
    pub text: *const thinkthen_request_text,
    pub json_value_map: *const thinkthen_request_json_value_map,
    pub json_value_list: *const thinkthen_request_json_value_list,
}
impl std::fmt::Debug for thinkthen_request_question_text_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("request union payload")
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_question_text {
    pub kind: u32,
    pub data: thinkthen_request_question_text_data,
}
pub const THINKTHEN_REQUEST_POINTERS_TEXT: u32 = 1;
pub const THINKTHEN_REQUEST_POINTERS_TEXT_LIST: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_request_pointers_data {
    pub text: *const thinkthen_request_text,
    pub text_list: *const thinkthen_request_text_list,
}
impl std::fmt::Debug for thinkthen_request_pointers_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("request union payload")
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_pointers {
    pub kind: u32,
    pub data: thinkthen_request_pointers_data,
}
pub const THINKTHEN_REQUEST_LEVELS_NAME_LIST: u32 = 1;
pub const THINKTHEN_REQUEST_LEVELS_CRITERION_MAP: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_request_levels_data {
    pub name_list: *const thinkthen_request_name_list,
    pub criterion_map: *const thinkthen_request_criterion_map,
}
impl std::fmt::Debug for thinkthen_request_levels_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("request union payload")
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_levels {
    pub kind: u32,
    pub data: thinkthen_request_levels_data,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_name {
    pub text: thinkthen_complete_utf8,
}
pub const THINKTHEN_REQUEST_CRITERION_TEXT: u32 = 1;
pub const THINKTHEN_REQUEST_CRITERION_JSON_VALUE_MAP: u32 = 2;
pub const THINKTHEN_REQUEST_CRITERION_JSON_VALUE_LIST: u32 = 3;
pub const THINKTHEN_REQUEST_CRITERION_NULL: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_request_criterion_data {
    pub text: *const thinkthen_request_text,
    pub json_value_map: *const thinkthen_request_json_value_map,
    pub json_value_list: *const thinkthen_request_json_value_list,
    pub null: *const thinkthen_request_null,
}
impl std::fmt::Debug for thinkthen_request_criterion_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("request union payload")
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_criterion {
    pub kind: u32,
    pub data: thinkthen_request_criterion_data,
}
pub const THINKTHEN_REQUEST_INPUT_DECLARATION_STRING: u32 = 1;
pub const THINKTHEN_REQUEST_INPUT_DECLARATION_OBJECT: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_request_input_declaration_data {
    pub string: *const thinkthen_request_input_declaration_string,
    pub object: *const thinkthen_request_input_declaration_object,
}
impl std::fmt::Debug for thinkthen_request_input_declaration_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("request union payload")
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_input_declaration {
    pub kind: u32,
    pub data: thinkthen_request_input_declaration_data,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_input_declaration_object {
    pub m_properties: *const thinkthen_request_input_property_map,
    pub m_required: *const thinkthen_request_text_list,
}
pub const THINKTHEN_REQUEST_INPUT_PROPERTY_STRING: u32 = 1;
pub const THINKTHEN_REQUEST_INPUT_PROPERTY_NUMBER: u32 = 2;
pub const THINKTHEN_REQUEST_INPUT_PROPERTY_BOOLEAN: u32 = 3;
pub const THINKTHEN_REQUEST_INPUT_PROPERTY_ARRAY: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_request_input_property_data {
    pub string: *const thinkthen_request_input_property_string,
    pub number: *const thinkthen_request_input_property_number,
    pub boolean: *const thinkthen_request_input_property_boolean,
    pub array: *const thinkthen_request_input_property_array,
}
impl std::fmt::Debug for thinkthen_request_input_property_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("request union payload")
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_input_property {
    pub kind: u32,
    pub data: thinkthen_request_input_property_data,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_input_property_array {
    pub m_items: *const thinkthen_request_input_property_array_items,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_input_property_boolean {
    pub reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_input_property_number {
    pub reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_input_property_string {
    pub reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_input_declaration_string {
    pub reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_annotate_question_tag {
    pub m_context_schema: *const thinkthen_request_input_declaration,
    pub m_item_schema: *const thinkthen_request_input_declaration,
    pub m_labels: *const thinkthen_request_labels,
    pub m_name: *const thinkthen_request_text,
    pub m_on: *const thinkthen_request_pointers,
    pub m_tag: *const thinkthen_request_question_text,
    pub m_threshold: *const thinkthen_request_cut,
}
pub const THINKTHEN_REQUEST_CUT_NUMBER: u32 = 1;
pub const THINKTHEN_REQUEST_CUT_TEXT: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_request_cut_data {
    pub number: *const thinkthen_request_number,
    pub text: *const thinkthen_request_text,
}
impl std::fmt::Debug for thinkthen_request_cut_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("request union payload")
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_cut {
    pub kind: u32,
    pub data: thinkthen_request_cut_data,
}
pub const THINKTHEN_REQUEST_LABELS_NAME_LIST: u32 = 1;
pub const THINKTHEN_REQUEST_LABELS_DESCRIPTION_MAP: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_request_labels_data {
    pub name_list: *const thinkthen_request_name_list,
    pub description_map: *const thinkthen_request_description_map,
}
impl std::fmt::Debug for thinkthen_request_labels_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("request union payload")
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_labels {
    pub kind: u32,
    pub data: thinkthen_request_labels_data,
}
pub const THINKTHEN_REQUEST_DESCRIPTION_TEXT: u32 = 1;
pub const THINKTHEN_REQUEST_DESCRIPTION_JSON_VALUE_MAP: u32 = 2;
pub const THINKTHEN_REQUEST_DESCRIPTION_JSON_VALUE_LIST: u32 = 3;
pub const THINKTHEN_REQUEST_DESCRIPTION_NULL: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_request_description_data {
    pub text: *const thinkthen_request_text,
    pub json_value_map: *const thinkthen_request_json_value_map,
    pub json_value_list: *const thinkthen_request_json_value_list,
    pub null: *const thinkthen_request_null,
}
impl std::fmt::Debug for thinkthen_request_description_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("request union payload")
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_description {
    pub kind: u32,
    pub data: thinkthen_request_description_data,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_annotate_question_choose {
    pub m_choose: *const thinkthen_request_question_text,
    pub m_context_schema: *const thinkthen_request_input_declaration,
    pub m_item_schema: *const thinkthen_request_input_declaration,
    pub m_name: *const thinkthen_request_text,
    pub m_on: *const thinkthen_request_pointers,
    pub m_options: *const thinkthen_request_options,
    pub m_threshold: *const thinkthen_request_cut,
}
pub const THINKTHEN_REQUEST_OPTIONS_NAME_LIST: u32 = 1;
pub const THINKTHEN_REQUEST_OPTIONS_DESCRIPTION_MAP: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_request_options_data {
    pub name_list: *const thinkthen_request_name_list,
    pub description_map: *const thinkthen_request_description_map,
}
impl std::fmt::Debug for thinkthen_request_options_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("request union payload")
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_options {
    pub kind: u32,
    pub data: thinkthen_request_options_data,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_annotate_question_decide {
    pub m_context_schema: *const thinkthen_request_input_declaration,
    pub m_decide: *const thinkthen_request_question_text,
    pub m_false: *const thinkthen_request_criterion,
    pub m_item_schema: *const thinkthen_request_input_declaration,
    pub m_name: *const thinkthen_request_text,
    pub m_on: *const thinkthen_request_pointers,
    pub m_threshold: *const thinkthen_request_threshold,
    pub m_true: *const thinkthen_request_criterion,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_profile {
    pub text: thinkthen_complete_utf8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_definition_recognize_definition {
    pub m_context_schema: *const thinkthen_request_input_declaration,
    pub m_item_schema: *const thinkthen_request_input_declaration,
    pub m_model: *const thinkthen_request_name,
    pub m_name: *const thinkthen_request_text,
    pub m_on: *const thinkthen_request_pointers,
    pub m_profile: *const thinkthen_request_profile,
    pub m_recognize: *const thinkthen_request_recognize_question,
    pub m_relation_threshold: *const thinkthen_request_cut,
    pub m_threshold: *const thinkthen_request_cut,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_recognize_question {
    pub m_entity_definition: *const thinkthen_request_question_text,
    pub m_instructions: *const thinkthen_request_question_text,
    pub m_kinds: *const thinkthen_request_description_map,
    pub m_mode: *const thinkthen_request_recognition_mode,
    pub m_relations: *const thinkthen_request_relation_list,
    pub m_snippet_pieces: *const thinkthen_request_unsigned,
    pub m_stage_context: *const thinkthen_request_recognition_stage_context,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_recognition_stage_context {
    pub m_boundary: *const thinkthen_request_text,
    pub m_kind_edge: *const thinkthen_request_text,
    pub m_relation: *const thinkthen_request_text,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_relation {
    pub m_either: *const thinkthen_request_boolean,
    pub m_name: *const thinkthen_request_name,
    pub m_reads: *const thinkthen_request_name,
    pub m_single: *const thinkthen_request_boolean,
    pub m_source: *const thinkthen_request_name,
    pub m_target: *const thinkthen_request_name,
}
pub const THINKTHEN_REQUEST_RECOGNITION_MODE_LITERAL_WHOLE: u32 = 1;
pub const THINKTHEN_REQUEST_RECOGNITION_MODE_LITERAL_BOUNDARY_ONLY: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_request_recognition_mode_data {
    pub literal_whole: *const thinkthen_request_literal_whole,
    pub literal_boundary_only: *const thinkthen_request_literal_boundary_only,
}
impl std::fmt::Debug for thinkthen_request_recognition_mode_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("request union payload")
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_recognition_mode {
    pub kind: u32,
    pub data: thinkthen_request_recognition_mode_data,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_definition_authored_find {
    pub m_context_schema: *const thinkthen_request_input_declaration,
    pub m_find: *const thinkthen_request_question_text,
    pub m_item_schema: *const thinkthen_request_input_declaration,
    pub m_model: *const thinkthen_request_name,
    pub m_name: *const thinkthen_request_text,
    pub m_on: *const thinkthen_request_pointers,
    pub m_profile: *const thinkthen_request_profile,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_definition_authored_relate {
    pub m_context_schema: *const thinkthen_request_input_declaration,
    pub m_item_schema: *const thinkthen_request_input_declaration,
    pub m_model: *const thinkthen_request_name,
    pub m_name: *const thinkthen_request_text,
    pub m_profile: *const thinkthen_request_profile,
    pub m_relate: *const thinkthen_request_relate_question,
    pub m_threshold: *const thinkthen_request_cut,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_relate_question {
    pub m_fields: *const thinkthen_request_relate_fields,
    pub m_relations: *const thinkthen_request_relation_list,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_relate_fields {
    pub m_kind: *const thinkthen_request_text,
    pub m_name: *const thinkthen_request_text,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_definition_authored_score {
    pub m_batch: *const thinkthen_request_question_batch,
    pub m_context_schema: *const thinkthen_request_input_declaration,
    pub m_item_schema: *const thinkthen_request_input_declaration,
    pub m_levels: *const thinkthen_request_levels,
    pub m_model: *const thinkthen_request_name,
    pub m_name: *const thinkthen_request_text,
    pub m_on: *const thinkthen_request_pointers,
    pub m_profile: *const thinkthen_request_profile,
    pub m_score: *const thinkthen_request_question_text,
}
pub const THINKTHEN_REQUEST_QUESTION_BATCH_LITERAL_MAX: u32 = 1;
pub const THINKTHEN_REQUEST_QUESTION_BATCH_INTEGER: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_request_question_batch_data {
    pub literal_max: *const thinkthen_request_literal_max,
    pub integer: *const thinkthen_request_integer,
}
impl std::fmt::Debug for thinkthen_request_question_batch_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("request union payload")
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_question_batch {
    pub kind: u32,
    pub data: thinkthen_request_question_batch_data,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_definition_authored_tag {
    pub m_batch: *const thinkthen_request_question_batch,
    pub m_context_schema: *const thinkthen_request_input_declaration,
    pub m_item_schema: *const thinkthen_request_input_declaration,
    pub m_labels: *const thinkthen_request_labels,
    pub m_model: *const thinkthen_request_name,
    pub m_name: *const thinkthen_request_text,
    pub m_on: *const thinkthen_request_pointers,
    pub m_profile: *const thinkthen_request_profile,
    pub m_tag: *const thinkthen_request_question_text,
    pub m_threshold: *const thinkthen_request_cut,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_definition_authored_choose {
    pub m_batch: *const thinkthen_request_question_batch,
    pub m_choose: *const thinkthen_request_question_text,
    pub m_context_schema: *const thinkthen_request_input_declaration,
    pub m_item_schema: *const thinkthen_request_input_declaration,
    pub m_model: *const thinkthen_request_name,
    pub m_name: *const thinkthen_request_text,
    pub m_on: *const thinkthen_request_pointers,
    pub m_options: *const thinkthen_request_options,
    pub m_profile: *const thinkthen_request_profile,
    pub m_threshold: *const thinkthen_request_cut,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_definition_authored_decide {
    pub m_batch: *const thinkthen_request_question_batch,
    pub m_context_schema: *const thinkthen_request_input_declaration,
    pub m_decide: *const thinkthen_request_question_text,
    pub m_false: *const thinkthen_request_criterion,
    pub m_item_schema: *const thinkthen_request_input_declaration,
    pub m_model: *const thinkthen_request_name,
    pub m_name: *const thinkthen_request_text,
    pub m_on: *const thinkthen_request_pointers,
    pub m_profile: *const thinkthen_request_profile,
    pub m_threshold: *const thinkthen_request_threshold,
    pub m_true: *const thinkthen_request_criterion,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_question_selector_text {
    pub m_text: *const thinkthen_request_text,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_call_options {
    pub m_attempts: *const thinkthen_request_boolean,
    pub m_batch: *const thinkthen_request_batch,
    pub m_context: *const thinkthen_request_text,
    pub m_context_field: *const thinkthen_request_text,
    pub m_deadline_ms: *const thinkthen_request_integer,
    pub m_entity_definition: *const thinkthen_request_text,
    pub m_examples: *const thinkthen_request_recognition_example_list,
    pub m_examples_field: *const thinkthen_request_text,
    pub m_field: *const thinkthen_request_text_list,
    pub m_files_only: *const thinkthen_request_boolean,
    pub m_instructions: *const thinkthen_request_text,
    pub m_labels: *const thinkthen_request_option_schema_2_list,
    pub m_levels: *const thinkthen_request_option_schema_2_list,
    pub m_max_requests_total: *const thinkthen_request_unsigned,
    pub m_max_text_bytes: *const thinkthen_request_unsigned,
    pub m_mode: *const thinkthen_request_recognition_mode,
    pub m_model: *const thinkthen_request_text,
    pub m_no: *const thinkthen_request_call_options_no,
    pub m_none: *const thinkthen_request_boolean,
    pub m_options_field: *const thinkthen_request_text,
    pub m_relation_threshold: *const thinkthen_request_call_threshold,
    pub m_seed_spans: *const thinkthen_request_recognition_seed_span_list,
    pub m_seed_spans_field: *const thinkthen_request_text,
    pub m_snippet_pieces: *const thinkthen_request_unsigned,
    pub m_stage_context: *const thinkthen_request_recognition_stage_context,
    pub m_threshold: *const thinkthen_request_call_threshold,
    pub m_top: *const thinkthen_request_unsigned,
    pub m_yes: *const thinkthen_request_call_options_yes,
}
pub const THINKTHEN_REQUEST_CALL_THRESHOLD_NUMBER: u32 = 1;
pub const THINKTHEN_REQUEST_CALL_THRESHOLD_TEXT: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_request_call_threshold_data {
    pub number: *const thinkthen_request_number,
    pub text: *const thinkthen_request_text,
}
impl std::fmt::Debug for thinkthen_request_call_threshold_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("request union payload")
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_call_threshold {
    pub kind: u32,
    pub data: thinkthen_request_call_threshold_data,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_option_schema_2 {
    pub m_description: *const thinkthen_request_option_schema_2_description,
    pub m_name: *const thinkthen_request_text,
}
pub const THINKTHEN_REQUEST_INPUT_TEXT: u32 = 1;
pub const THINKTHEN_REQUEST_INPUT_JSON: u32 = 2;
pub const THINKTHEN_REQUEST_INPUT_RECORDS: u32 = 3;
pub const THINKTHEN_REQUEST_INPUT_UNITS: u32 = 4;
pub const THINKTHEN_REQUEST_INPUT_ENTITIES: u32 = 5;
pub const THINKTHEN_REQUEST_INPUT_SOURCE: u32 = 6;
pub const THINKTHEN_REQUEST_INPUT_FEED: u32 = 7;
#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_request_input_data {
    pub text: *const thinkthen_request_input_text,
    pub json: *const thinkthen_request_input_json,
    pub records: *const thinkthen_request_input_records,
    pub units: *const thinkthen_request_input_units,
    pub entities: *const thinkthen_request_input_entities,
    pub source: *const thinkthen_request_input_source,
    pub feed: *const thinkthen_request_input_feed,
}
impl std::fmt::Debug for thinkthen_request_input_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("request union payload")
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_input {
    pub kind: u32,
    pub data: thinkthen_request_input_data,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_input_feed {
    pub m_framing: *const thinkthen_request_framing,
    pub m_images: *const thinkthen_request_image_list,
    pub m_name: *const thinkthen_request_text,
    pub m_reading: *const thinkthen_request_reader,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_reader {
    pub m_unit: *const thinkthen_request_source_unit,
    pub m_window: *const thinkthen_request_unsigned,
}
pub const THINKTHEN_REQUEST_SOURCE_UNIT_LITERAL_LINE: u32 = 1;
pub const THINKTHEN_REQUEST_SOURCE_UNIT_LITERAL_WINDOW: u32 = 2;
pub const THINKTHEN_REQUEST_SOURCE_UNIT_LITERAL_FILE: u32 = 3;
#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_request_source_unit_data {
    pub literal_line: *const thinkthen_request_literal_line,
    pub literal_window: *const thinkthen_request_literal_window,
    pub literal_file: *const thinkthen_request_literal_file,
}
impl std::fmt::Debug for thinkthen_request_source_unit_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("request union payload")
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_source_unit {
    pub kind: u32,
    pub data: thinkthen_request_source_unit_data,
}
pub const THINKTHEN_REQUEST_FRAMING_LITERAL_DOCUMENT: u32 = 1;
pub const THINKTHEN_REQUEST_FRAMING_LITERAL_LINES: u32 = 2;
pub const THINKTHEN_REQUEST_FRAMING_LITERAL_JSONL: u32 = 3;
pub const THINKTHEN_REQUEST_FRAMING_LITERAL_CSV: u32 = 4;
pub const THINKTHEN_REQUEST_FRAMING_LITERAL_TSV: u32 = 5;
#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_request_framing_data {
    pub literal_document: *const thinkthen_request_literal_document,
    pub literal_lines: *const thinkthen_request_literal_lines,
    pub literal_jsonl: *const thinkthen_request_literal_jsonl,
    pub literal_csv: *const thinkthen_request_literal_csv,
    pub literal_tsv: *const thinkthen_request_literal_tsv,
}
impl std::fmt::Debug for thinkthen_request_framing_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("request union payload")
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_framing {
    pub kind: u32,
    pub data: thinkthen_request_framing_data,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_input_source {
    pub m_source: *const thinkthen_request_source,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_source {
    pub m_framing: *const thinkthen_request_framing,
    pub m_media: *const thinkthen_request_reader_media,
    pub m_paths: *const thinkthen_request_text_list,
    pub m_reading: *const thinkthen_request_reader,
}
pub const THINKTHEN_REQUEST_READER_MEDIA_LITERAL_TEXT: u32 = 1;
pub const THINKTHEN_REQUEST_READER_MEDIA_LITERAL_IMAGE: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_request_reader_media_data {
    pub literal_text: *const thinkthen_request_literal_text,
    pub literal_image: *const thinkthen_request_literal_image,
}
impl std::fmt::Debug for thinkthen_request_reader_media_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("request union payload")
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_reader_media {
    pub kind: u32,
    pub data: thinkthen_request_reader_media_data,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_input_entities {
    pub m_items: *const thinkthen_request_item_list,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_input_units {
    pub m_items: *const thinkthen_request_item_list,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_input_records {
    pub m_items: *const thinkthen_request_item_list,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_input_json {
    pub m_images: *const thinkthen_request_image_list,
    pub m_value: *const thinkthen_request_input_json_value,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_input_text {
    pub m_images: *const thinkthen_request_image_list,
    pub m_text: *const thinkthen_request_text,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_text {
    pub text: thinkthen_complete_utf8,
}
pub const THINKTHEN_REQUEST_OPTIONAL_TOTAL_UNSIGNED_VALUE: u32 = 1;
pub const THINKTHEN_REQUEST_OPTIONAL_TOTAL_NULL: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_request_optional_total_data {
    pub unsigned_value: *const thinkthen_request_unsigned,
    pub null: *const thinkthen_request_null,
}
impl std::fmt::Debug for thinkthen_request_optional_total_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("request union payload")
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_optional_total {
    pub kind: u32,
    pub data: thinkthen_request_optional_total_data,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_unsigned {
    pub value: u64,
}
pub const THINKTHEN_REQUEST_OPTIONAL_COUNT_UNSIGNED_VALUE: u32 = 1;
pub const THINKTHEN_REQUEST_OPTIONAL_COUNT_NULL: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_request_optional_count_data {
    pub unsigned_value: *const thinkthen_request_unsigned,
    pub null: *const thinkthen_request_null,
}
impl std::fmt::Debug for thinkthen_request_optional_count_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("request union payload")
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_optional_count {
    pub kind: u32,
    pub data: thinkthen_request_optional_count_data,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_json_value {
    pub value: *const thinkthen_complete_json,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_boolean {
    pub value: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_recognition_example_list {
    pub data: *const *const thinkthen_request_recognition_example,
    pub len: usize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_image_list {
    pub data: *const *const thinkthen_request_image,
    pub len: usize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_option_schema_list {
    pub data: *const *const thinkthen_request_option_schema,
    pub len: usize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_recognition_seed_span_list {
    pub data: *const *const thinkthen_request_recognition_seed_span,
    pub len: usize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_original_json_value {
    pub value: *const thinkthen_complete_json,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_literal_image_jpeg {
    pub reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_literal_image_png {
    pub reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_recognition_example_entity_list {
    pub data: *const *const thinkthen_request_recognition_example_entity,
    pub len: usize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_text_list {
    pub data: *const *const thinkthen_request_text,
    pub len: usize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_json_value_map_entry {
    pub key: thinkthen_complete_utf8,
    pub value: *const thinkthen_request_json_value,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_json_value_map {
    pub data: *const thinkthen_request_json_value_map_entry,
    pub len: usize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_annotate_question_map_entry {
    pub key: thinkthen_complete_utf8,
    pub value: *const thinkthen_request_annotate_question,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_annotate_question_map {
    pub data: *const thinkthen_request_annotate_question_map_entry,
    pub len: usize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_number {
    pub value: f64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_json_value_list {
    pub data: *const *const thinkthen_request_json_value,
    pub len: usize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_name_list {
    pub data: *const *const thinkthen_request_name,
    pub len: usize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_criterion_map_entry {
    pub key: thinkthen_complete_utf8,
    pub value: *const thinkthen_request_criterion,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_criterion_map {
    pub data: *const thinkthen_request_criterion_map_entry,
    pub len: usize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_null {
    pub reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_input_property_map_entry {
    pub key: thinkthen_complete_utf8,
    pub value: *const thinkthen_request_input_property,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_input_property_map {
    pub data: *const thinkthen_request_input_property_map_entry,
    pub len: usize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_input_property_array_items {
    pub reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_description_map_entry {
    pub key: thinkthen_complete_utf8,
    pub value: *const thinkthen_request_description,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_description_map {
    pub data: *const thinkthen_request_description_map_entry,
    pub len: usize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_relation_list {
    pub data: *const *const thinkthen_request_relation,
    pub len: usize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_literal_whole {
    pub reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_literal_boundary_only {
    pub reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_literal_max {
    pub reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_integer {
    pub value: i64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_option_schema_2_list {
    pub data: *const *const thinkthen_request_option_schema_2,
    pub len: usize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_call_options_no {
    pub value: *const thinkthen_complete_json,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_call_options_yes {
    pub value: *const thinkthen_complete_json,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_option_schema_2_description {
    pub value: *const thinkthen_complete_json,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_literal_line {
    pub reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_literal_window {
    pub reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_literal_file {
    pub reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_literal_document {
    pub reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_literal_lines {
    pub reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_literal_jsonl {
    pub reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_literal_csv {
    pub reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_literal_tsv {
    pub reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_literal_text {
    pub reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_literal_image {
    pub reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_item_list {
    pub data: *const *const thinkthen_request_item,
    pub len: usize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_input_json_value {
    pub value: *const thinkthen_complete_json,
}
unsafe fn read_json_value(
    pointer: *const thinkthen_complete_json,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains its aligned node and all counted children.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: only the validated tag selects a payload; each extent is checked.
        unsafe {
            match input.kind {
                THINKTHEN_COMPLETE_JSON_NULL => Value::literal("null"),
                THINKTHEN_COMPLETE_JSON_BOOLEAN => Ok(Value::Boolean(match input.data.boolean {
                    0 => false,
                    1 => true,
                    _ => return Err(CarrierError::Invalid),
                })),
                THINKTHEN_COMPLETE_JSON_NUMBER => {
                    let number = context.text(input.data.number)?;
                    if !matches!(number.as_bytes().first(), Some(b'-' | b'0'..=b'9')) {
                        return Err(CarrierError::Invalid);
                    }
                    Value::literal(&number)
                }
                THINKTHEN_COMPLETE_JSON_STRING => Ok(Value::Text(context.text(input.data.string)?)),
                THINKTHEN_COMPLETE_JSON_ARRAY => {
                    let input = input.data.array;
                    let mut values = Vec::new();
                    for item in extent(input.data, input.len)? {
                        values.push(read_json_value(*item, context)?);
                    }
                    Ok(Value::Array(values))
                }
                THINKTHEN_COMPLETE_JSON_OBJECT => {
                    let input = input.data.object;
                    let mut values = Vec::new();
                    for item in extent(input.data, input.len)? {
                        values.push((
                            context.text(item.name)?,
                            read_json_value(item.value, context)?,
                        ));
                    }
                    Ok(Value::Object(values))
                }
                _ => Err(CarrierError::Invalid),
            }
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_engine_settings(
    pointer: *const thinkthen_request_engine_settings,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok({
                let mut fields = vec![];
                optional(
                    "backend",
                    input.m_backend,
                    context,
                    &mut fields,
                    read_thinkthen_request_text,
                )?;
                optional(
                    "base_url",
                    input.m_base_url,
                    context,
                    &mut fields,
                    read_thinkthen_request_text,
                )?;
                optional(
                    "batch",
                    input.m_batch,
                    context,
                    &mut fields,
                    read_thinkthen_request_batch,
                )?;
                optional(
                    "cache",
                    input.m_cache,
                    context,
                    &mut fields,
                    read_thinkthen_request_cache_document,
                )?;
                optional(
                    "max_estimated_input_tokens_total",
                    input.m_max_estimated_input_tokens_total,
                    context,
                    &mut fields,
                    read_thinkthen_request_optional_total,
                )?;
                optional(
                    "max_request_bytes",
                    input.m_max_request_bytes,
                    context,
                    &mut fields,
                    read_thinkthen_request_unsigned,
                )?;
                optional(
                    "max_requests",
                    input.m_max_requests,
                    context,
                    &mut fields,
                    read_thinkthen_request_optional_count,
                )?;
                optional(
                    "max_requests_total",
                    input.m_max_requests_total,
                    context,
                    &mut fields,
                    read_thinkthen_request_optional_total,
                )?;
                optional(
                    "max_retries",
                    input.m_max_retries,
                    context,
                    &mut fields,
                    read_thinkthen_request_unsigned,
                )?;
                optional(
                    "model",
                    input.m_model,
                    context,
                    &mut fields,
                    read_thinkthen_request_text,
                )?;
                optional(
                    "profile",
                    input.m_profile,
                    context,
                    &mut fields,
                    read_thinkthen_request_text,
                )?;
                optional(
                    "proxy",
                    input.m_proxy,
                    context,
                    &mut fields,
                    read_thinkthen_request_json_value,
                )?;
                optional(
                    "record",
                    input.m_record,
                    context,
                    &mut fields,
                    read_thinkthen_request_text,
                )?;
                optional(
                    "refresh_cache",
                    input.m_refresh_cache,
                    context,
                    &mut fields,
                    read_thinkthen_request_boolean,
                )?;
                optional(
                    "replay",
                    input.m_replay,
                    context,
                    &mut fields,
                    read_thinkthen_request_text,
                )?;
                optional(
                    "throttle",
                    input.m_throttle,
                    context,
                    &mut fields,
                    read_thinkthen_request_unsigned,
                )?;
                optional(
                    "timeout",
                    input.m_timeout,
                    context,
                    &mut fields,
                    read_thinkthen_request_unsigned,
                )?;
                optional(
                    "usd_per_million_input",
                    input.m_usd_per_million_input,
                    context,
                    &mut fields,
                    read_thinkthen_request_text,
                )?;
                optional(
                    "usd_per_million_output",
                    input.m_usd_per_million_output,
                    context,
                    &mut fields,
                    read_thinkthen_request_text,
                )?;
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_cache_document(
    pointer: *const thinkthen_request_cache_document,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok(match input.kind {
                THINKTHEN_REQUEST_CACHE_DOCUMENT_TEXT => {
                    read_thinkthen_request_text(input.data.text, context)?
                }
                THINKTHEN_REQUEST_CACHE_DOCUMENT_DISABLED_CACHE => {
                    read_thinkthen_request_disabled_cache(input.data.disabled_cache, context)?
                }
                _ => return Err(CarrierError::Invalid),
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_disabled_cache(
    pointer: *const thinkthen_request_disabled_cache,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let _ = unsafe { borrow(pointer)? };
    let _ = context;
    Value::literal("false")
}
pub(super) unsafe fn read_thinkthen_request_batch(
    pointer: *const thinkthen_request_batch,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok(match input.kind {
                THINKTHEN_REQUEST_BATCH_UNSIGNED_VALUE => {
                    read_thinkthen_request_unsigned(input.data.unsigned_value, context)?
                }
                THINKTHEN_REQUEST_BATCH_TEXT => {
                    read_thinkthen_request_text(input.data.text, context)?
                }
                _ => return Err(CarrierError::Invalid),
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_reader_failure(
    pointer: *const thinkthen_request_reader_failure,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok(match input.kind {
                THINKTHEN_REQUEST_READER_FAILURE_PRODUCER => {
                    read_thinkthen_request_reader_failure_producer(input.data.producer, context)?
                }
                THINKTHEN_REQUEST_READER_FAILURE_IO => {
                    read_thinkthen_request_reader_failure_io(input.data.io, context)?
                }
                THINKTHEN_REQUEST_READER_FAILURE_UTF_8 => {
                    read_thinkthen_request_reader_failure_utf_8(input.data.utf_8, context)?
                }
                THINKTHEN_REQUEST_READER_FAILURE_INVALID_INPUT => {
                    read_thinkthen_request_reader_failure_invalid_input(
                        input.data.invalid_input,
                        context,
                    )?
                }
                _ => return Err(CarrierError::Invalid),
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_reader_failure_invalid_input(
    pointer: *const thinkthen_request_reader_failure_invalid_input,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok({
                let mut fields = vec![("kind".into(), Value::literal("\"invalid_input\"")?)];
                optional(
                    "location",
                    input.m_location,
                    context,
                    &mut fields,
                    read_thinkthen_request_session_source_location,
                )?;
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_session_source_location(
    pointer: *const thinkthen_request_session_source_location,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok({
                let mut fields = vec![(
                    "file".into(),
                    read_thinkthen_request_text(input.m_file, context)?,
                )];
                optional(
                    "first_line",
                    input.m_first_line,
                    context,
                    &mut fields,
                    read_thinkthen_request_unsigned,
                )?;
                optional(
                    "last_line",
                    input.m_last_line,
                    context,
                    &mut fields,
                    read_thinkthen_request_unsigned,
                )?;
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_reader_failure_utf_8(
    pointer: *const thinkthen_request_reader_failure_utf_8,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok({
                let mut fields = vec![("kind".into(), Value::literal("\"utf8\"")?)];
                optional(
                    "location",
                    input.m_location,
                    context,
                    &mut fields,
                    read_thinkthen_request_session_source_location,
                )?;
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_reader_failure_io(
    pointer: *const thinkthen_request_reader_failure_io,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok({
                let mut fields = vec![("kind".into(), Value::literal("\"io\"")?)];
                optional(
                    "location",
                    input.m_location,
                    context,
                    &mut fields,
                    read_thinkthen_request_session_source_location,
                )?;
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_reader_failure_producer(
    pointer: *const thinkthen_request_reader_failure_producer,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let _input = unsafe { borrow(pointer)? };
    context.enter(|_context| {
        Ok({
            let fields = vec![("kind".into(), Value::literal("\"producer\"")?)];
            Value::Object(fields)
        })
    })
}
pub(super) unsafe fn read_thinkthen_request_session_descriptor(
    pointer: *const thinkthen_request_session_descriptor,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok({
                let mut fields = vec![(
                    "item".into(),
                    read_thinkthen_request_item(input.m_item, context)?,
                )];
                optional(
                    "location",
                    input.m_location,
                    context,
                    &mut fields,
                    read_thinkthen_request_session_source_location,
                )?;
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_item(
    pointer: *const thinkthen_request_item,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok({
                let mut fields = vec![];
                optional(
                    "context",
                    input.m_context,
                    context,
                    &mut fields,
                    read_thinkthen_request_context_schema,
                )?;
                optional(
                    "examples",
                    input.m_examples,
                    context,
                    &mut fields,
                    read_thinkthen_request_recognition_example_list,
                )?;
                optional(
                    "images",
                    input.m_images,
                    context,
                    &mut fields,
                    read_thinkthen_request_image_list,
                )?;
                optional(
                    "options",
                    input.m_options,
                    context,
                    &mut fields,
                    read_thinkthen_request_option_schema_list,
                )?;
                optional(
                    "original",
                    input.m_original,
                    context,
                    &mut fields,
                    read_thinkthen_request_original,
                )?;
                optional(
                    "seed_spans",
                    input.m_seed_spans,
                    context,
                    &mut fields,
                    read_thinkthen_request_recognition_seed_span_list,
                )?;
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_recognition_seed_span(
    pointer: *const thinkthen_request_recognition_seed_span,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok({
                let mut fields = vec![
                    (
                        "end".into(),
                        read_thinkthen_request_unsigned(input.m_end, context)?,
                    ),
                    (
                        "start".into(),
                        read_thinkthen_request_unsigned(input.m_start, context)?,
                    ),
                ];
                optional(
                    "kind",
                    input.m_kind,
                    context,
                    &mut fields,
                    read_thinkthen_request_text,
                )?;
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_original(
    pointer: *const thinkthen_request_original,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok(match input.kind {
                THINKTHEN_REQUEST_ORIGINAL_TEXT => {
                    read_thinkthen_request_original_text(input.data.text, context)?
                }
                THINKTHEN_REQUEST_ORIGINAL_JSON => {
                    read_thinkthen_request_original_json(input.data.json, context)?
                }
                _ => return Err(CarrierError::Invalid),
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_original_json(
    pointer: *const thinkthen_request_original_json,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok({
                let fields = vec![
                    ("kind".into(), Value::literal("\"json\"")?),
                    (
                        "value".into(),
                        read_thinkthen_request_original_json_value(input.m_value, context)?,
                    ),
                ];
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_original_text(
    pointer: *const thinkthen_request_original_text,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok({
                let fields = vec![
                    ("kind".into(), Value::literal("\"text\"")?),
                    (
                        "text".into(),
                        read_thinkthen_request_text(input.m_text, context)?,
                    ),
                ];
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_option_schema(
    pointer: *const thinkthen_request_option_schema,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok({
                let mut fields = vec![(
                    "name".into(),
                    read_thinkthen_request_text(input.m_name, context)?,
                )];
                optional(
                    "description",
                    input.m_description,
                    context,
                    &mut fields,
                    read_thinkthen_request_json_value,
                )?;
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_image(
    pointer: *const thinkthen_request_image,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok(match input.kind {
                THINKTHEN_REQUEST_IMAGE_FILE => {
                    read_thinkthen_request_image_file(input.data.file, context)?
                }
                THINKTHEN_REQUEST_IMAGE_BYTES => {
                    read_thinkthen_request_image_bytes(input.data.bytes, context)?
                }
                _ => return Err(CarrierError::Invalid),
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_image_bytes(
    pointer: *const thinkthen_request_image_bytes,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok({
                let fields = vec![
                    (
                        "bytes".into(),
                        read_thinkthen_request_text(input.m_bytes, context)?,
                    ),
                    ("kind".into(), Value::literal("\"bytes\"")?),
                    (
                        "media".into(),
                        read_thinkthen_request_image_media(input.m_media, context)?,
                    ),
                ];
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_image_media(
    pointer: *const thinkthen_request_image_media,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok(match input.kind {
                THINKTHEN_REQUEST_IMAGE_MEDIA_LITERAL_IMAGE_JPEG => {
                    read_thinkthen_request_literal_image_jpeg(
                        input.data.literal_image_jpeg,
                        context,
                    )?
                }
                THINKTHEN_REQUEST_IMAGE_MEDIA_LITERAL_IMAGE_PNG => {
                    read_thinkthen_request_literal_image_png(input.data.literal_image_png, context)?
                }
                _ => return Err(CarrierError::Invalid),
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_image_file(
    pointer: *const thinkthen_request_image_file,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok({
                let mut fields = vec![
                    ("kind".into(), Value::literal("\"file\"")?),
                    (
                        "path".into(),
                        read_thinkthen_request_text(input.m_path, context)?,
                    ),
                ];
                optional(
                    "media",
                    input.m_media,
                    context,
                    &mut fields,
                    read_thinkthen_request_image_media,
                )?;
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_recognition_example(
    pointer: *const thinkthen_request_recognition_example,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok(match input.kind {
                THINKTHEN_REQUEST_RECOGNITION_EXAMPLE_TEXT => {
                    read_thinkthen_request_text(input.data.text, context)?
                }
                THINKTHEN_REQUEST_RECOGNITION_EXAMPLE_EXAMPLE_TEXT => {
                    read_thinkthen_request_recognition_example_text(
                        input.data.example_text,
                        context,
                    )?
                }
                _ => return Err(CarrierError::Invalid),
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_recognition_example_text(
    pointer: *const thinkthen_request_recognition_example_text,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok({
                let mut fields = vec![
                    (
                        "entities".into(),
                        read_thinkthen_request_recognition_example_entity_list(
                            input.m_entities,
                            context,
                        )?,
                    ),
                    (
                        "text".into(),
                        read_thinkthen_request_text(input.m_text, context)?,
                    ),
                ];
                optional(
                    "kinds",
                    input.m_kinds,
                    context,
                    &mut fields,
                    read_thinkthen_request_text_list,
                )?;
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_recognition_example_entity(
    pointer: *const thinkthen_request_recognition_example_entity,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok({
                let fields = vec![
                    (
                        "end".into(),
                        read_thinkthen_request_unsigned(input.m_end, context)?,
                    ),
                    (
                        "kind".into(),
                        read_thinkthen_request_text(input.m_kind, context)?,
                    ),
                    (
                        "start".into(),
                        read_thinkthen_request_unsigned(input.m_start, context)?,
                    ),
                ];
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_context_schema(
    pointer: *const thinkthen_request_context_schema,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok(match input.kind {
                THINKTHEN_REQUEST_CONTEXT_SCHEMA_TEXT => {
                    read_thinkthen_request_text(input.data.text, context)?
                }
                THINKTHEN_REQUEST_CONTEXT_SCHEMA_JSON_VALUE_MAP => {
                    read_thinkthen_request_json_value_map(input.data.json_value_map, context)?
                }
                _ => return Err(CarrierError::Invalid),
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_arguments(
    pointer: *const thinkthen_request_arguments,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok({
                let mut fields = vec![
                    (
                        "input".into(),
                        read_thinkthen_request_input(input.m_input, context)?,
                    ),
                    (
                        "question".into(),
                        read_thinkthen_request_question_selector(input.m_question, context)?,
                    ),
                ];
                optional(
                    "options",
                    input.m_options,
                    context,
                    &mut fields,
                    read_thinkthen_request_call_options,
                )?;
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_question_selector(
    pointer: *const thinkthen_request_question_selector,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok(match input.kind {
                THINKTHEN_REQUEST_QUESTION_SELECTOR_TEXT => {
                    read_thinkthen_request_question_selector_text(input.data.text, context)?
                }
                THINKTHEN_REQUEST_QUESTION_SELECTOR_DEFINITION => {
                    read_thinkthen_request_question_selector_definition(
                        input.data.definition,
                        context,
                    )?
                }
                THINKTHEN_REQUEST_QUESTION_SELECTOR_FILE => {
                    read_thinkthen_request_question_selector_file(input.data.file, context)?
                }
                THINKTHEN_REQUEST_QUESTION_SELECTOR_NAME => {
                    read_thinkthen_request_question_selector_name(input.data.name, context)?
                }
                THINKTHEN_REQUEST_QUESTION_SELECTOR_REFERENCE => {
                    read_thinkthen_request_question_selector_reference(
                        input.data.reference,
                        context,
                    )?
                }
                _ => return Err(CarrierError::Invalid),
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_question_selector_reference(
    pointer: *const thinkthen_request_question_selector_reference,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok({
                let fields = vec![
                    ("kind".into(), Value::literal("\"reference\"")?),
                    (
                        "reference".into(),
                        read_thinkthen_request_text(input.m_reference, context)?,
                    ),
                ];
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_question_selector_name(
    pointer: *const thinkthen_request_question_selector_name,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok({
                let fields = vec![
                    ("kind".into(), Value::literal("\"name\"")?),
                    (
                        "name".into(),
                        read_thinkthen_request_text(input.m_name, context)?,
                    ),
                ];
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_question_selector_file(
    pointer: *const thinkthen_request_question_selector_file,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok({
                let fields = vec![
                    ("kind".into(), Value::literal("\"file\"")?),
                    (
                        "path".into(),
                        read_thinkthen_request_text(input.m_path, context)?,
                    ),
                ];
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_question_selector_definition(
    pointer: *const thinkthen_request_question_selector_definition,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok({
                let fields = vec![
                    ("kind".into(), Value::literal("\"definition\"")?),
                    (
                        "value".into(),
                        read_thinkthen_request_definition(input.m_value, context)?,
                    ),
                ];
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_definition(
    pointer: *const thinkthen_request_definition,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok(match input.kind {
                THINKTHEN_REQUEST_DEFINITION_AUTHORED_DECIDE => {
                    read_thinkthen_request_definition_authored_decide(
                        input.data.authored_decide,
                        context,
                    )?
                }
                THINKTHEN_REQUEST_DEFINITION_AUTHORED_CHOOSE => {
                    read_thinkthen_request_definition_authored_choose(
                        input.data.authored_choose,
                        context,
                    )?
                }
                THINKTHEN_REQUEST_DEFINITION_AUTHORED_TAG => {
                    read_thinkthen_request_definition_authored_tag(
                        input.data.authored_tag,
                        context,
                    )?
                }
                THINKTHEN_REQUEST_DEFINITION_AUTHORED_SCORE => {
                    read_thinkthen_request_definition_authored_score(
                        input.data.authored_score,
                        context,
                    )?
                }
                THINKTHEN_REQUEST_DEFINITION_AUTHORED_RELATE => {
                    read_thinkthen_request_definition_authored_relate(
                        input.data.authored_relate,
                        context,
                    )?
                }
                THINKTHEN_REQUEST_DEFINITION_AUTHORED_FIND => {
                    read_thinkthen_request_definition_authored_find(
                        input.data.authored_find,
                        context,
                    )?
                }
                THINKTHEN_REQUEST_DEFINITION_RECOGNIZE_DEFINITION => {
                    read_thinkthen_request_definition_recognize_definition(
                        input.data.recognize_definition,
                        context,
                    )?
                }
                THINKTHEN_REQUEST_DEFINITION_ANNOTATE_DEFINITION => {
                    read_thinkthen_request_definition_annotate_definition(
                        input.data.annotate_definition,
                        context,
                    )?
                }
                _ => return Err(CarrierError::Invalid),
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_definition_annotate_definition(
    pointer: *const thinkthen_request_definition_annotate_definition,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok({
                let mut fields = vec![(
                    "questions".into(),
                    read_thinkthen_request_annotate_question_map(input.m_questions, context)?,
                )];
                optional(
                    "batch",
                    input.m_batch,
                    context,
                    &mut fields,
                    read_thinkthen_request_json_value,
                )?;
                optional(
                    "profile",
                    input.m_profile,
                    context,
                    &mut fields,
                    read_thinkthen_request_profile,
                )?;
                optional(
                    "threshold",
                    input.m_threshold,
                    context,
                    &mut fields,
                    read_thinkthen_request_threshold,
                )?;
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_threshold(
    pointer: *const thinkthen_request_threshold,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok(match input.kind {
                THINKTHEN_REQUEST_THRESHOLD_NUMBER => {
                    read_thinkthen_request_number(input.data.number, context)?
                }
                THINKTHEN_REQUEST_THRESHOLD_TEXT => {
                    read_thinkthen_request_text(input.data.text, context)?
                }
                _ => return Err(CarrierError::Invalid),
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_annotate_question(
    pointer: *const thinkthen_request_annotate_question,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok(match input.kind {
                THINKTHEN_REQUEST_ANNOTATE_QUESTION_DECIDE => {
                    read_thinkthen_request_annotate_question_decide(input.data.decide, context)?
                }
                THINKTHEN_REQUEST_ANNOTATE_QUESTION_CHOOSE => {
                    read_thinkthen_request_annotate_question_choose(input.data.choose, context)?
                }
                THINKTHEN_REQUEST_ANNOTATE_QUESTION_TAG => {
                    read_thinkthen_request_annotate_question_tag(input.data.tag, context)?
                }
                THINKTHEN_REQUEST_ANNOTATE_QUESTION_SCORE => {
                    read_thinkthen_request_annotate_question_score(input.data.score, context)?
                }
                _ => return Err(CarrierError::Invalid),
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_annotate_question_score(
    pointer: *const thinkthen_request_annotate_question_score,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok({
                let mut fields = vec![(
                    "score".into(),
                    read_thinkthen_request_question_text(input.m_score, context)?,
                )];
                optional(
                    "context_schema",
                    input.m_context_schema,
                    context,
                    &mut fields,
                    read_thinkthen_request_input_declaration,
                )?;
                optional(
                    "item_schema",
                    input.m_item_schema,
                    context,
                    &mut fields,
                    read_thinkthen_request_input_declaration,
                )?;
                optional(
                    "levels",
                    input.m_levels,
                    context,
                    &mut fields,
                    read_thinkthen_request_levels,
                )?;
                optional(
                    "name",
                    input.m_name,
                    context,
                    &mut fields,
                    read_thinkthen_request_text,
                )?;
                optional(
                    "on",
                    input.m_on,
                    context,
                    &mut fields,
                    read_thinkthen_request_pointers,
                )?;
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_question_text(
    pointer: *const thinkthen_request_question_text,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok(match input.kind {
                THINKTHEN_REQUEST_QUESTION_TEXT_TEXT => {
                    read_thinkthen_request_text(input.data.text, context)?
                }
                THINKTHEN_REQUEST_QUESTION_TEXT_JSON_VALUE_MAP => {
                    read_thinkthen_request_json_value_map(input.data.json_value_map, context)?
                }
                THINKTHEN_REQUEST_QUESTION_TEXT_JSON_VALUE_LIST => {
                    read_thinkthen_request_json_value_list(input.data.json_value_list, context)?
                }
                _ => return Err(CarrierError::Invalid),
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_pointers(
    pointer: *const thinkthen_request_pointers,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok(match input.kind {
                THINKTHEN_REQUEST_POINTERS_TEXT => {
                    read_thinkthen_request_text(input.data.text, context)?
                }
                THINKTHEN_REQUEST_POINTERS_TEXT_LIST => {
                    read_thinkthen_request_text_list(input.data.text_list, context)?
                }
                _ => return Err(CarrierError::Invalid),
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_levels(
    pointer: *const thinkthen_request_levels,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok(match input.kind {
                THINKTHEN_REQUEST_LEVELS_NAME_LIST => {
                    read_thinkthen_request_name_list(input.data.name_list, context)?
                }
                THINKTHEN_REQUEST_LEVELS_CRITERION_MAP => {
                    read_thinkthen_request_criterion_map(input.data.criterion_map, context)?
                }
                _ => return Err(CarrierError::Invalid),
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_name(
    pointer: *const thinkthen_request_name,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe { Ok(Value::Text(context.text(input.text)?)) }
    })
}
pub(super) unsafe fn read_thinkthen_request_criterion(
    pointer: *const thinkthen_request_criterion,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok(match input.kind {
                THINKTHEN_REQUEST_CRITERION_TEXT => {
                    read_thinkthen_request_text(input.data.text, context)?
                }
                THINKTHEN_REQUEST_CRITERION_JSON_VALUE_MAP => {
                    read_thinkthen_request_json_value_map(input.data.json_value_map, context)?
                }
                THINKTHEN_REQUEST_CRITERION_JSON_VALUE_LIST => {
                    read_thinkthen_request_json_value_list(input.data.json_value_list, context)?
                }
                THINKTHEN_REQUEST_CRITERION_NULL => {
                    read_thinkthen_request_null(input.data.null, context)?
                }
                _ => return Err(CarrierError::Invalid),
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_input_declaration(
    pointer: *const thinkthen_request_input_declaration,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok(match input.kind {
                THINKTHEN_REQUEST_INPUT_DECLARATION_STRING => {
                    read_thinkthen_request_input_declaration_string(input.data.string, context)?
                }
                THINKTHEN_REQUEST_INPUT_DECLARATION_OBJECT => {
                    read_thinkthen_request_input_declaration_object(input.data.object, context)?
                }
                _ => return Err(CarrierError::Invalid),
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_input_declaration_object(
    pointer: *const thinkthen_request_input_declaration_object,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok({
                let mut fields = vec![
                    (
                        "properties".into(),
                        read_thinkthen_request_input_property_map(input.m_properties, context)?,
                    ),
                    ("type".into(), Value::literal("\"object\"")?),
                ];
                optional(
                    "required",
                    input.m_required,
                    context,
                    &mut fields,
                    read_thinkthen_request_text_list,
                )?;
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_input_property(
    pointer: *const thinkthen_request_input_property,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok(match input.kind {
                THINKTHEN_REQUEST_INPUT_PROPERTY_STRING => {
                    read_thinkthen_request_input_property_string(input.data.string, context)?
                }
                THINKTHEN_REQUEST_INPUT_PROPERTY_NUMBER => {
                    read_thinkthen_request_input_property_number(input.data.number, context)?
                }
                THINKTHEN_REQUEST_INPUT_PROPERTY_BOOLEAN => {
                    read_thinkthen_request_input_property_boolean(input.data.boolean, context)?
                }
                THINKTHEN_REQUEST_INPUT_PROPERTY_ARRAY => {
                    read_thinkthen_request_input_property_array(input.data.array, context)?
                }
                _ => return Err(CarrierError::Invalid),
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_input_property_array(
    pointer: *const thinkthen_request_input_property_array,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok({
                let fields = vec![
                    (
                        "items".into(),
                        read_thinkthen_request_input_property_array_items(input.m_items, context)?,
                    ),
                    ("type".into(), Value::literal("\"array\"")?),
                ];
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_input_property_boolean(
    pointer: *const thinkthen_request_input_property_boolean,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let _input = unsafe { borrow(pointer)? };
    context.enter(|_context| {
        Ok({
            let fields = vec![("type".into(), Value::literal("\"boolean\"")?)];
            Value::Object(fields)
        })
    })
}
pub(super) unsafe fn read_thinkthen_request_input_property_number(
    pointer: *const thinkthen_request_input_property_number,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let _input = unsafe { borrow(pointer)? };
    context.enter(|_context| {
        Ok({
            let fields = vec![("type".into(), Value::literal("\"number\"")?)];
            Value::Object(fields)
        })
    })
}
pub(super) unsafe fn read_thinkthen_request_input_property_string(
    pointer: *const thinkthen_request_input_property_string,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let _input = unsafe { borrow(pointer)? };
    context.enter(|_context| {
        Ok({
            let fields = vec![("type".into(), Value::literal("\"string\"")?)];
            Value::Object(fields)
        })
    })
}
pub(super) unsafe fn read_thinkthen_request_input_declaration_string(
    pointer: *const thinkthen_request_input_declaration_string,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let _input = unsafe { borrow(pointer)? };
    context.enter(|_context| {
        Ok({
            let fields = vec![("type".into(), Value::literal("\"string\"")?)];
            Value::Object(fields)
        })
    })
}
pub(super) unsafe fn read_thinkthen_request_annotate_question_tag(
    pointer: *const thinkthen_request_annotate_question_tag,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok({
                let mut fields = vec![(
                    "tag".into(),
                    read_thinkthen_request_question_text(input.m_tag, context)?,
                )];
                optional(
                    "context_schema",
                    input.m_context_schema,
                    context,
                    &mut fields,
                    read_thinkthen_request_input_declaration,
                )?;
                optional(
                    "item_schema",
                    input.m_item_schema,
                    context,
                    &mut fields,
                    read_thinkthen_request_input_declaration,
                )?;
                optional(
                    "labels",
                    input.m_labels,
                    context,
                    &mut fields,
                    read_thinkthen_request_labels,
                )?;
                optional(
                    "name",
                    input.m_name,
                    context,
                    &mut fields,
                    read_thinkthen_request_text,
                )?;
                optional(
                    "on",
                    input.m_on,
                    context,
                    &mut fields,
                    read_thinkthen_request_pointers,
                )?;
                optional(
                    "threshold",
                    input.m_threshold,
                    context,
                    &mut fields,
                    read_thinkthen_request_cut,
                )?;
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_cut(
    pointer: *const thinkthen_request_cut,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok(match input.kind {
                THINKTHEN_REQUEST_CUT_NUMBER => {
                    read_thinkthen_request_number(input.data.number, context)?
                }
                THINKTHEN_REQUEST_CUT_TEXT => {
                    read_thinkthen_request_text(input.data.text, context)?
                }
                _ => return Err(CarrierError::Invalid),
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_labels(
    pointer: *const thinkthen_request_labels,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok(match input.kind {
                THINKTHEN_REQUEST_LABELS_NAME_LIST => {
                    read_thinkthen_request_name_list(input.data.name_list, context)?
                }
                THINKTHEN_REQUEST_LABELS_DESCRIPTION_MAP => {
                    read_thinkthen_request_description_map(input.data.description_map, context)?
                }
                _ => return Err(CarrierError::Invalid),
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_description(
    pointer: *const thinkthen_request_description,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok(match input.kind {
                THINKTHEN_REQUEST_DESCRIPTION_TEXT => {
                    read_thinkthen_request_text(input.data.text, context)?
                }
                THINKTHEN_REQUEST_DESCRIPTION_JSON_VALUE_MAP => {
                    read_thinkthen_request_json_value_map(input.data.json_value_map, context)?
                }
                THINKTHEN_REQUEST_DESCRIPTION_JSON_VALUE_LIST => {
                    read_thinkthen_request_json_value_list(input.data.json_value_list, context)?
                }
                THINKTHEN_REQUEST_DESCRIPTION_NULL => {
                    read_thinkthen_request_null(input.data.null, context)?
                }
                _ => return Err(CarrierError::Invalid),
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_annotate_question_choose(
    pointer: *const thinkthen_request_annotate_question_choose,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok({
                let mut fields = vec![(
                    "choose".into(),
                    read_thinkthen_request_question_text(input.m_choose, context)?,
                )];
                optional(
                    "context_schema",
                    input.m_context_schema,
                    context,
                    &mut fields,
                    read_thinkthen_request_input_declaration,
                )?;
                optional(
                    "item_schema",
                    input.m_item_schema,
                    context,
                    &mut fields,
                    read_thinkthen_request_input_declaration,
                )?;
                optional(
                    "name",
                    input.m_name,
                    context,
                    &mut fields,
                    read_thinkthen_request_text,
                )?;
                optional(
                    "on",
                    input.m_on,
                    context,
                    &mut fields,
                    read_thinkthen_request_pointers,
                )?;
                optional(
                    "options",
                    input.m_options,
                    context,
                    &mut fields,
                    read_thinkthen_request_options,
                )?;
                optional(
                    "threshold",
                    input.m_threshold,
                    context,
                    &mut fields,
                    read_thinkthen_request_cut,
                )?;
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_options(
    pointer: *const thinkthen_request_options,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok(match input.kind {
                THINKTHEN_REQUEST_OPTIONS_NAME_LIST => {
                    read_thinkthen_request_name_list(input.data.name_list, context)?
                }
                THINKTHEN_REQUEST_OPTIONS_DESCRIPTION_MAP => {
                    read_thinkthen_request_description_map(input.data.description_map, context)?
                }
                _ => return Err(CarrierError::Invalid),
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_annotate_question_decide(
    pointer: *const thinkthen_request_annotate_question_decide,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok({
                let mut fields = vec![(
                    "decide".into(),
                    read_thinkthen_request_question_text(input.m_decide, context)?,
                )];
                optional(
                    "context_schema",
                    input.m_context_schema,
                    context,
                    &mut fields,
                    read_thinkthen_request_input_declaration,
                )?;
                optional(
                    "false",
                    input.m_false,
                    context,
                    &mut fields,
                    read_thinkthen_request_criterion,
                )?;
                optional(
                    "item_schema",
                    input.m_item_schema,
                    context,
                    &mut fields,
                    read_thinkthen_request_input_declaration,
                )?;
                optional(
                    "name",
                    input.m_name,
                    context,
                    &mut fields,
                    read_thinkthen_request_text,
                )?;
                optional(
                    "on",
                    input.m_on,
                    context,
                    &mut fields,
                    read_thinkthen_request_pointers,
                )?;
                optional(
                    "threshold",
                    input.m_threshold,
                    context,
                    &mut fields,
                    read_thinkthen_request_threshold,
                )?;
                optional(
                    "true",
                    input.m_true,
                    context,
                    &mut fields,
                    read_thinkthen_request_criterion,
                )?;
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_profile(
    pointer: *const thinkthen_request_profile,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe { Ok(Value::Text(context.text(input.text)?)) }
    })
}
pub(super) unsafe fn read_thinkthen_request_definition_recognize_definition(
    pointer: *const thinkthen_request_definition_recognize_definition,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok({
                let mut fields = vec![(
                    "recognize".into(),
                    read_thinkthen_request_recognize_question(input.m_recognize, context)?,
                )];
                optional(
                    "context_schema",
                    input.m_context_schema,
                    context,
                    &mut fields,
                    read_thinkthen_request_input_declaration,
                )?;
                optional(
                    "item_schema",
                    input.m_item_schema,
                    context,
                    &mut fields,
                    read_thinkthen_request_input_declaration,
                )?;
                optional(
                    "model",
                    input.m_model,
                    context,
                    &mut fields,
                    read_thinkthen_request_name,
                )?;
                optional(
                    "name",
                    input.m_name,
                    context,
                    &mut fields,
                    read_thinkthen_request_text,
                )?;
                optional(
                    "on",
                    input.m_on,
                    context,
                    &mut fields,
                    read_thinkthen_request_pointers,
                )?;
                optional(
                    "profile",
                    input.m_profile,
                    context,
                    &mut fields,
                    read_thinkthen_request_profile,
                )?;
                optional(
                    "relation_threshold",
                    input.m_relation_threshold,
                    context,
                    &mut fields,
                    read_thinkthen_request_cut,
                )?;
                optional(
                    "threshold",
                    input.m_threshold,
                    context,
                    &mut fields,
                    read_thinkthen_request_cut,
                )?;
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_recognize_question(
    pointer: *const thinkthen_request_recognize_question,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok({
                let mut fields = vec![];
                optional(
                    "entity_definition",
                    input.m_entity_definition,
                    context,
                    &mut fields,
                    read_thinkthen_request_question_text,
                )?;
                optional(
                    "instructions",
                    input.m_instructions,
                    context,
                    &mut fields,
                    read_thinkthen_request_question_text,
                )?;
                optional(
                    "kinds",
                    input.m_kinds,
                    context,
                    &mut fields,
                    read_thinkthen_request_description_map,
                )?;
                optional(
                    "mode",
                    input.m_mode,
                    context,
                    &mut fields,
                    read_thinkthen_request_recognition_mode,
                )?;
                optional(
                    "relations",
                    input.m_relations,
                    context,
                    &mut fields,
                    read_thinkthen_request_relation_list,
                )?;
                optional(
                    "snippet_pieces",
                    input.m_snippet_pieces,
                    context,
                    &mut fields,
                    read_thinkthen_request_unsigned,
                )?;
                optional(
                    "stage_context",
                    input.m_stage_context,
                    context,
                    &mut fields,
                    read_thinkthen_request_recognition_stage_context,
                )?;
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_recognition_stage_context(
    pointer: *const thinkthen_request_recognition_stage_context,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok({
                let mut fields = vec![];
                optional(
                    "boundary",
                    input.m_boundary,
                    context,
                    &mut fields,
                    read_thinkthen_request_text,
                )?;
                optional(
                    "kind_edge",
                    input.m_kind_edge,
                    context,
                    &mut fields,
                    read_thinkthen_request_text,
                )?;
                optional(
                    "relation",
                    input.m_relation,
                    context,
                    &mut fields,
                    read_thinkthen_request_text,
                )?;
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_relation(
    pointer: *const thinkthen_request_relation,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok({
                let mut fields = vec![(
                    "name".into(),
                    read_thinkthen_request_name(input.m_name, context)?,
                )];
                optional(
                    "either",
                    input.m_either,
                    context,
                    &mut fields,
                    read_thinkthen_request_boolean,
                )?;
                optional(
                    "reads",
                    input.m_reads,
                    context,
                    &mut fields,
                    read_thinkthen_request_name,
                )?;
                optional(
                    "single",
                    input.m_single,
                    context,
                    &mut fields,
                    read_thinkthen_request_boolean,
                )?;
                optional(
                    "source",
                    input.m_source,
                    context,
                    &mut fields,
                    read_thinkthen_request_name,
                )?;
                optional(
                    "target",
                    input.m_target,
                    context,
                    &mut fields,
                    read_thinkthen_request_name,
                )?;
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_recognition_mode(
    pointer: *const thinkthen_request_recognition_mode,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok(match input.kind {
                THINKTHEN_REQUEST_RECOGNITION_MODE_LITERAL_WHOLE => {
                    read_thinkthen_request_literal_whole(input.data.literal_whole, context)?
                }
                THINKTHEN_REQUEST_RECOGNITION_MODE_LITERAL_BOUNDARY_ONLY => {
                    read_thinkthen_request_literal_boundary_only(
                        input.data.literal_boundary_only,
                        context,
                    )?
                }
                _ => return Err(CarrierError::Invalid),
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_definition_authored_find(
    pointer: *const thinkthen_request_definition_authored_find,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok({
                let mut fields = vec![(
                    "find".into(),
                    read_thinkthen_request_question_text(input.m_find, context)?,
                )];
                optional(
                    "context_schema",
                    input.m_context_schema,
                    context,
                    &mut fields,
                    read_thinkthen_request_input_declaration,
                )?;
                optional(
                    "item_schema",
                    input.m_item_schema,
                    context,
                    &mut fields,
                    read_thinkthen_request_input_declaration,
                )?;
                optional(
                    "model",
                    input.m_model,
                    context,
                    &mut fields,
                    read_thinkthen_request_name,
                )?;
                optional(
                    "name",
                    input.m_name,
                    context,
                    &mut fields,
                    read_thinkthen_request_text,
                )?;
                optional(
                    "on",
                    input.m_on,
                    context,
                    &mut fields,
                    read_thinkthen_request_pointers,
                )?;
                optional(
                    "profile",
                    input.m_profile,
                    context,
                    &mut fields,
                    read_thinkthen_request_profile,
                )?;
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_definition_authored_relate(
    pointer: *const thinkthen_request_definition_authored_relate,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok({
                let mut fields = vec![(
                    "relate".into(),
                    read_thinkthen_request_relate_question(input.m_relate, context)?,
                )];
                optional(
                    "context_schema",
                    input.m_context_schema,
                    context,
                    &mut fields,
                    read_thinkthen_request_input_declaration,
                )?;
                optional(
                    "item_schema",
                    input.m_item_schema,
                    context,
                    &mut fields,
                    read_thinkthen_request_input_declaration,
                )?;
                optional(
                    "model",
                    input.m_model,
                    context,
                    &mut fields,
                    read_thinkthen_request_name,
                )?;
                optional(
                    "name",
                    input.m_name,
                    context,
                    &mut fields,
                    read_thinkthen_request_text,
                )?;
                optional(
                    "profile",
                    input.m_profile,
                    context,
                    &mut fields,
                    read_thinkthen_request_profile,
                )?;
                optional(
                    "threshold",
                    input.m_threshold,
                    context,
                    &mut fields,
                    read_thinkthen_request_cut,
                )?;
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_relate_question(
    pointer: *const thinkthen_request_relate_question,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok({
                let mut fields = vec![(
                    "relations".into(),
                    read_thinkthen_request_relation_list(input.m_relations, context)?,
                )];
                optional(
                    "fields",
                    input.m_fields,
                    context,
                    &mut fields,
                    read_thinkthen_request_relate_fields,
                )?;
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_relate_fields(
    pointer: *const thinkthen_request_relate_fields,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok({
                let fields = vec![
                    (
                        "kind".into(),
                        read_thinkthen_request_text(input.m_kind, context)?,
                    ),
                    (
                        "name".into(),
                        read_thinkthen_request_text(input.m_name, context)?,
                    ),
                ];
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_definition_authored_score(
    pointer: *const thinkthen_request_definition_authored_score,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok({
                let mut fields = vec![(
                    "score".into(),
                    read_thinkthen_request_question_text(input.m_score, context)?,
                )];
                optional(
                    "batch",
                    input.m_batch,
                    context,
                    &mut fields,
                    read_thinkthen_request_question_batch,
                )?;
                optional(
                    "context_schema",
                    input.m_context_schema,
                    context,
                    &mut fields,
                    read_thinkthen_request_input_declaration,
                )?;
                optional(
                    "item_schema",
                    input.m_item_schema,
                    context,
                    &mut fields,
                    read_thinkthen_request_input_declaration,
                )?;
                optional(
                    "levels",
                    input.m_levels,
                    context,
                    &mut fields,
                    read_thinkthen_request_levels,
                )?;
                optional(
                    "model",
                    input.m_model,
                    context,
                    &mut fields,
                    read_thinkthen_request_name,
                )?;
                optional(
                    "name",
                    input.m_name,
                    context,
                    &mut fields,
                    read_thinkthen_request_text,
                )?;
                optional(
                    "on",
                    input.m_on,
                    context,
                    &mut fields,
                    read_thinkthen_request_pointers,
                )?;
                optional(
                    "profile",
                    input.m_profile,
                    context,
                    &mut fields,
                    read_thinkthen_request_profile,
                )?;
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_question_batch(
    pointer: *const thinkthen_request_question_batch,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok(match input.kind {
                THINKTHEN_REQUEST_QUESTION_BATCH_LITERAL_MAX => {
                    read_thinkthen_request_literal_max(input.data.literal_max, context)?
                }
                THINKTHEN_REQUEST_QUESTION_BATCH_INTEGER => {
                    read_thinkthen_request_integer(input.data.integer, context)?
                }
                _ => return Err(CarrierError::Invalid),
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_definition_authored_tag(
    pointer: *const thinkthen_request_definition_authored_tag,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok({
                let mut fields = vec![(
                    "tag".into(),
                    read_thinkthen_request_question_text(input.m_tag, context)?,
                )];
                optional(
                    "batch",
                    input.m_batch,
                    context,
                    &mut fields,
                    read_thinkthen_request_question_batch,
                )?;
                optional(
                    "context_schema",
                    input.m_context_schema,
                    context,
                    &mut fields,
                    read_thinkthen_request_input_declaration,
                )?;
                optional(
                    "item_schema",
                    input.m_item_schema,
                    context,
                    &mut fields,
                    read_thinkthen_request_input_declaration,
                )?;
                optional(
                    "labels",
                    input.m_labels,
                    context,
                    &mut fields,
                    read_thinkthen_request_labels,
                )?;
                optional(
                    "model",
                    input.m_model,
                    context,
                    &mut fields,
                    read_thinkthen_request_name,
                )?;
                optional(
                    "name",
                    input.m_name,
                    context,
                    &mut fields,
                    read_thinkthen_request_text,
                )?;
                optional(
                    "on",
                    input.m_on,
                    context,
                    &mut fields,
                    read_thinkthen_request_pointers,
                )?;
                optional(
                    "profile",
                    input.m_profile,
                    context,
                    &mut fields,
                    read_thinkthen_request_profile,
                )?;
                optional(
                    "threshold",
                    input.m_threshold,
                    context,
                    &mut fields,
                    read_thinkthen_request_cut,
                )?;
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_definition_authored_choose(
    pointer: *const thinkthen_request_definition_authored_choose,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok({
                let mut fields = vec![(
                    "choose".into(),
                    read_thinkthen_request_question_text(input.m_choose, context)?,
                )];
                optional(
                    "batch",
                    input.m_batch,
                    context,
                    &mut fields,
                    read_thinkthen_request_question_batch,
                )?;
                optional(
                    "context_schema",
                    input.m_context_schema,
                    context,
                    &mut fields,
                    read_thinkthen_request_input_declaration,
                )?;
                optional(
                    "item_schema",
                    input.m_item_schema,
                    context,
                    &mut fields,
                    read_thinkthen_request_input_declaration,
                )?;
                optional(
                    "model",
                    input.m_model,
                    context,
                    &mut fields,
                    read_thinkthen_request_name,
                )?;
                optional(
                    "name",
                    input.m_name,
                    context,
                    &mut fields,
                    read_thinkthen_request_text,
                )?;
                optional(
                    "on",
                    input.m_on,
                    context,
                    &mut fields,
                    read_thinkthen_request_pointers,
                )?;
                optional(
                    "options",
                    input.m_options,
                    context,
                    &mut fields,
                    read_thinkthen_request_options,
                )?;
                optional(
                    "profile",
                    input.m_profile,
                    context,
                    &mut fields,
                    read_thinkthen_request_profile,
                )?;
                optional(
                    "threshold",
                    input.m_threshold,
                    context,
                    &mut fields,
                    read_thinkthen_request_cut,
                )?;
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_definition_authored_decide(
    pointer: *const thinkthen_request_definition_authored_decide,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok({
                let mut fields = vec![(
                    "decide".into(),
                    read_thinkthen_request_question_text(input.m_decide, context)?,
                )];
                optional(
                    "batch",
                    input.m_batch,
                    context,
                    &mut fields,
                    read_thinkthen_request_question_batch,
                )?;
                optional(
                    "context_schema",
                    input.m_context_schema,
                    context,
                    &mut fields,
                    read_thinkthen_request_input_declaration,
                )?;
                optional(
                    "false",
                    input.m_false,
                    context,
                    &mut fields,
                    read_thinkthen_request_criterion,
                )?;
                optional(
                    "item_schema",
                    input.m_item_schema,
                    context,
                    &mut fields,
                    read_thinkthen_request_input_declaration,
                )?;
                optional(
                    "model",
                    input.m_model,
                    context,
                    &mut fields,
                    read_thinkthen_request_name,
                )?;
                optional(
                    "name",
                    input.m_name,
                    context,
                    &mut fields,
                    read_thinkthen_request_text,
                )?;
                optional(
                    "on",
                    input.m_on,
                    context,
                    &mut fields,
                    read_thinkthen_request_pointers,
                )?;
                optional(
                    "profile",
                    input.m_profile,
                    context,
                    &mut fields,
                    read_thinkthen_request_profile,
                )?;
                optional(
                    "threshold",
                    input.m_threshold,
                    context,
                    &mut fields,
                    read_thinkthen_request_threshold,
                )?;
                optional(
                    "true",
                    input.m_true,
                    context,
                    &mut fields,
                    read_thinkthen_request_criterion,
                )?;
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_question_selector_text(
    pointer: *const thinkthen_request_question_selector_text,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok({
                let fields = vec![
                    ("kind".into(), Value::literal("\"text\"")?),
                    (
                        "text".into(),
                        read_thinkthen_request_text(input.m_text, context)?,
                    ),
                ];
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_call_options(
    pointer: *const thinkthen_request_call_options,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok({
                let mut fields = vec![];
                optional(
                    "attempts",
                    input.m_attempts,
                    context,
                    &mut fields,
                    read_thinkthen_request_boolean,
                )?;
                optional(
                    "batch",
                    input.m_batch,
                    context,
                    &mut fields,
                    read_thinkthen_request_batch,
                )?;
                optional(
                    "context",
                    input.m_context,
                    context,
                    &mut fields,
                    read_thinkthen_request_text,
                )?;
                optional(
                    "context_field",
                    input.m_context_field,
                    context,
                    &mut fields,
                    read_thinkthen_request_text,
                )?;
                optional(
                    "deadline_ms",
                    input.m_deadline_ms,
                    context,
                    &mut fields,
                    read_thinkthen_request_integer,
                )?;
                optional(
                    "entity_definition",
                    input.m_entity_definition,
                    context,
                    &mut fields,
                    read_thinkthen_request_text,
                )?;
                optional(
                    "examples",
                    input.m_examples,
                    context,
                    &mut fields,
                    read_thinkthen_request_recognition_example_list,
                )?;
                optional(
                    "examples_field",
                    input.m_examples_field,
                    context,
                    &mut fields,
                    read_thinkthen_request_text,
                )?;
                optional(
                    "field",
                    input.m_field,
                    context,
                    &mut fields,
                    read_thinkthen_request_text_list,
                )?;
                optional(
                    "files_only",
                    input.m_files_only,
                    context,
                    &mut fields,
                    read_thinkthen_request_boolean,
                )?;
                optional(
                    "instructions",
                    input.m_instructions,
                    context,
                    &mut fields,
                    read_thinkthen_request_text,
                )?;
                optional(
                    "labels",
                    input.m_labels,
                    context,
                    &mut fields,
                    read_thinkthen_request_option_schema_2_list,
                )?;
                optional(
                    "levels",
                    input.m_levels,
                    context,
                    &mut fields,
                    read_thinkthen_request_option_schema_2_list,
                )?;
                optional(
                    "max_requests_total",
                    input.m_max_requests_total,
                    context,
                    &mut fields,
                    read_thinkthen_request_unsigned,
                )?;
                optional(
                    "max_text_bytes",
                    input.m_max_text_bytes,
                    context,
                    &mut fields,
                    read_thinkthen_request_unsigned,
                )?;
                optional(
                    "mode",
                    input.m_mode,
                    context,
                    &mut fields,
                    read_thinkthen_request_recognition_mode,
                )?;
                optional(
                    "model",
                    input.m_model,
                    context,
                    &mut fields,
                    read_thinkthen_request_text,
                )?;
                optional(
                    "no",
                    input.m_no,
                    context,
                    &mut fields,
                    read_thinkthen_request_call_options_no,
                )?;
                optional(
                    "none",
                    input.m_none,
                    context,
                    &mut fields,
                    read_thinkthen_request_boolean,
                )?;
                optional(
                    "options_field",
                    input.m_options_field,
                    context,
                    &mut fields,
                    read_thinkthen_request_text,
                )?;
                optional(
                    "relation_threshold",
                    input.m_relation_threshold,
                    context,
                    &mut fields,
                    read_thinkthen_request_call_threshold,
                )?;
                optional(
                    "seed_spans",
                    input.m_seed_spans,
                    context,
                    &mut fields,
                    read_thinkthen_request_recognition_seed_span_list,
                )?;
                optional(
                    "seed_spans_field",
                    input.m_seed_spans_field,
                    context,
                    &mut fields,
                    read_thinkthen_request_text,
                )?;
                optional(
                    "snippet_pieces",
                    input.m_snippet_pieces,
                    context,
                    &mut fields,
                    read_thinkthen_request_unsigned,
                )?;
                optional(
                    "stage_context",
                    input.m_stage_context,
                    context,
                    &mut fields,
                    read_thinkthen_request_recognition_stage_context,
                )?;
                optional(
                    "threshold",
                    input.m_threshold,
                    context,
                    &mut fields,
                    read_thinkthen_request_call_threshold,
                )?;
                optional(
                    "top",
                    input.m_top,
                    context,
                    &mut fields,
                    read_thinkthen_request_unsigned,
                )?;
                optional(
                    "yes",
                    input.m_yes,
                    context,
                    &mut fields,
                    read_thinkthen_request_call_options_yes,
                )?;
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_call_threshold(
    pointer: *const thinkthen_request_call_threshold,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok(match input.kind {
                THINKTHEN_REQUEST_CALL_THRESHOLD_NUMBER => {
                    read_thinkthen_request_number(input.data.number, context)?
                }
                THINKTHEN_REQUEST_CALL_THRESHOLD_TEXT => {
                    read_thinkthen_request_text(input.data.text, context)?
                }
                _ => return Err(CarrierError::Invalid),
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_option_schema_2(
    pointer: *const thinkthen_request_option_schema_2,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok({
                let mut fields = vec![(
                    "name".into(),
                    read_thinkthen_request_text(input.m_name, context)?,
                )];
                optional(
                    "description",
                    input.m_description,
                    context,
                    &mut fields,
                    read_thinkthen_request_option_schema_2_description,
                )?;
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_input(
    pointer: *const thinkthen_request_input,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok(match input.kind {
                THINKTHEN_REQUEST_INPUT_TEXT => {
                    read_thinkthen_request_input_text(input.data.text, context)?
                }
                THINKTHEN_REQUEST_INPUT_JSON => {
                    read_thinkthen_request_input_json(input.data.json, context)?
                }
                THINKTHEN_REQUEST_INPUT_RECORDS => {
                    read_thinkthen_request_input_records(input.data.records, context)?
                }
                THINKTHEN_REQUEST_INPUT_UNITS => {
                    read_thinkthen_request_input_units(input.data.units, context)?
                }
                THINKTHEN_REQUEST_INPUT_ENTITIES => {
                    read_thinkthen_request_input_entities(input.data.entities, context)?
                }
                THINKTHEN_REQUEST_INPUT_SOURCE => {
                    read_thinkthen_request_input_source(input.data.source, context)?
                }
                THINKTHEN_REQUEST_INPUT_FEED => {
                    read_thinkthen_request_input_feed(input.data.feed, context)?
                }
                _ => return Err(CarrierError::Invalid),
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_input_feed(
    pointer: *const thinkthen_request_input_feed,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok({
                let mut fields = vec![
                    ("kind".into(), Value::literal("\"feed\"")?),
                    (
                        "name".into(),
                        read_thinkthen_request_text(input.m_name, context)?,
                    ),
                ];
                optional(
                    "framing",
                    input.m_framing,
                    context,
                    &mut fields,
                    read_thinkthen_request_framing,
                )?;
                optional(
                    "images",
                    input.m_images,
                    context,
                    &mut fields,
                    read_thinkthen_request_image_list,
                )?;
                optional(
                    "reading",
                    input.m_reading,
                    context,
                    &mut fields,
                    read_thinkthen_request_reader,
                )?;
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_reader(
    pointer: *const thinkthen_request_reader,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok({
                let mut fields = vec![];
                optional(
                    "unit",
                    input.m_unit,
                    context,
                    &mut fields,
                    read_thinkthen_request_source_unit,
                )?;
                optional(
                    "window",
                    input.m_window,
                    context,
                    &mut fields,
                    read_thinkthen_request_unsigned,
                )?;
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_source_unit(
    pointer: *const thinkthen_request_source_unit,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok(match input.kind {
                THINKTHEN_REQUEST_SOURCE_UNIT_LITERAL_LINE => {
                    read_thinkthen_request_literal_line(input.data.literal_line, context)?
                }
                THINKTHEN_REQUEST_SOURCE_UNIT_LITERAL_WINDOW => {
                    read_thinkthen_request_literal_window(input.data.literal_window, context)?
                }
                THINKTHEN_REQUEST_SOURCE_UNIT_LITERAL_FILE => {
                    read_thinkthen_request_literal_file(input.data.literal_file, context)?
                }
                _ => return Err(CarrierError::Invalid),
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_framing(
    pointer: *const thinkthen_request_framing,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok(match input.kind {
                THINKTHEN_REQUEST_FRAMING_LITERAL_DOCUMENT => {
                    read_thinkthen_request_literal_document(input.data.literal_document, context)?
                }
                THINKTHEN_REQUEST_FRAMING_LITERAL_LINES => {
                    read_thinkthen_request_literal_lines(input.data.literal_lines, context)?
                }
                THINKTHEN_REQUEST_FRAMING_LITERAL_JSONL => {
                    read_thinkthen_request_literal_jsonl(input.data.literal_jsonl, context)?
                }
                THINKTHEN_REQUEST_FRAMING_LITERAL_CSV => {
                    read_thinkthen_request_literal_csv(input.data.literal_csv, context)?
                }
                THINKTHEN_REQUEST_FRAMING_LITERAL_TSV => {
                    read_thinkthen_request_literal_tsv(input.data.literal_tsv, context)?
                }
                _ => return Err(CarrierError::Invalid),
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_input_source(
    pointer: *const thinkthen_request_input_source,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok({
                let fields = vec![
                    ("kind".into(), Value::literal("\"source\"")?),
                    (
                        "source".into(),
                        read_thinkthen_request_source(input.m_source, context)?,
                    ),
                ];
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_source(
    pointer: *const thinkthen_request_source,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok({
                let mut fields = vec![(
                    "paths".into(),
                    read_thinkthen_request_text_list(input.m_paths, context)?,
                )];
                optional(
                    "framing",
                    input.m_framing,
                    context,
                    &mut fields,
                    read_thinkthen_request_framing,
                )?;
                optional(
                    "media",
                    input.m_media,
                    context,
                    &mut fields,
                    read_thinkthen_request_reader_media,
                )?;
                optional(
                    "reading",
                    input.m_reading,
                    context,
                    &mut fields,
                    read_thinkthen_request_reader,
                )?;
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_reader_media(
    pointer: *const thinkthen_request_reader_media,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok(match input.kind {
                THINKTHEN_REQUEST_READER_MEDIA_LITERAL_TEXT => {
                    read_thinkthen_request_literal_text(input.data.literal_text, context)?
                }
                THINKTHEN_REQUEST_READER_MEDIA_LITERAL_IMAGE => {
                    read_thinkthen_request_literal_image(input.data.literal_image, context)?
                }
                _ => return Err(CarrierError::Invalid),
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_input_entities(
    pointer: *const thinkthen_request_input_entities,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok({
                let fields = vec![
                    (
                        "items".into(),
                        read_thinkthen_request_item_list(input.m_items, context)?,
                    ),
                    ("kind".into(), Value::literal("\"entities\"")?),
                ];
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_input_units(
    pointer: *const thinkthen_request_input_units,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok({
                let fields = vec![
                    (
                        "items".into(),
                        read_thinkthen_request_item_list(input.m_items, context)?,
                    ),
                    ("kind".into(), Value::literal("\"units\"")?),
                ];
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_input_records(
    pointer: *const thinkthen_request_input_records,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok({
                let fields = vec![
                    (
                        "items".into(),
                        read_thinkthen_request_item_list(input.m_items, context)?,
                    ),
                    ("kind".into(), Value::literal("\"records\"")?),
                ];
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_input_json(
    pointer: *const thinkthen_request_input_json,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok({
                let mut fields = vec![
                    ("kind".into(), Value::literal("\"json\"")?),
                    (
                        "value".into(),
                        read_thinkthen_request_input_json_value(input.m_value, context)?,
                    ),
                ];
                optional(
                    "images",
                    input.m_images,
                    context,
                    &mut fields,
                    read_thinkthen_request_image_list,
                )?;
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_input_text(
    pointer: *const thinkthen_request_input_text,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok({
                let mut fields = vec![
                    ("kind".into(), Value::literal("\"text\"")?),
                    (
                        "text".into(),
                        read_thinkthen_request_text(input.m_text, context)?,
                    ),
                ];
                optional(
                    "images",
                    input.m_images,
                    context,
                    &mut fields,
                    read_thinkthen_request_image_list,
                )?;
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_text(
    pointer: *const thinkthen_request_text,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe { Ok(Value::Text(context.text(input.text)?)) }
    })
}
pub(super) unsafe fn read_thinkthen_request_optional_total(
    pointer: *const thinkthen_request_optional_total,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok(match input.kind {
                THINKTHEN_REQUEST_OPTIONAL_TOTAL_UNSIGNED_VALUE => {
                    read_thinkthen_request_unsigned(input.data.unsigned_value, context)?
                }
                THINKTHEN_REQUEST_OPTIONAL_TOTAL_NULL => {
                    read_thinkthen_request_null(input.data.null, context)?
                }
                _ => return Err(CarrierError::Invalid),
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_unsigned(
    pointer: *const thinkthen_request_unsigned,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|_context| Ok(Value::Number(input.value.into())))
}
pub(super) unsafe fn read_thinkthen_request_optional_count(
    pointer: *const thinkthen_request_optional_count,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok(match input.kind {
                THINKTHEN_REQUEST_OPTIONAL_COUNT_UNSIGNED_VALUE => {
                    read_thinkthen_request_unsigned(input.data.unsigned_value, context)?
                }
                THINKTHEN_REQUEST_OPTIONAL_COUNT_NULL => {
                    read_thinkthen_request_null(input.data.null, context)?
                }
                _ => return Err(CarrierError::Invalid),
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_json_value(
    pointer: *const thinkthen_request_json_value,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok({
                let value = read_json_value(input.value, context)?;
                if value.json().map_err(|_| CarrierError::Invalid)?.len() > context.scalar_capacity
                {
                    return Err(CarrierError::Overflow);
                }
                value
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_boolean(
    pointer: *const thinkthen_request_boolean,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|_context| {
        Ok(Value::Boolean(match input.value {
            0 => false,
            1 => true,
            _ => return Err(CarrierError::Invalid),
        }))
    })
}
pub(super) unsafe fn read_thinkthen_request_recognition_example_list(
    pointer: *const thinkthen_request_recognition_example_list,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok({
                let mut values = Vec::new();
                for item in extent(input.data, input.len)? {
                    values.push(read_thinkthen_request_recognition_example(*item, context)?);
                }
                Value::Array(values)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_image_list(
    pointer: *const thinkthen_request_image_list,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok({
                let mut values = Vec::new();
                for item in extent(input.data, input.len)? {
                    values.push(read_thinkthen_request_image(*item, context)?);
                }
                Value::Array(values)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_option_schema_list(
    pointer: *const thinkthen_request_option_schema_list,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok({
                let mut values = Vec::new();
                for item in extent(input.data, input.len)? {
                    values.push(read_thinkthen_request_option_schema(*item, context)?);
                }
                Value::Array(values)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_recognition_seed_span_list(
    pointer: *const thinkthen_request_recognition_seed_span_list,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok({
                let mut values = Vec::new();
                for item in extent(input.data, input.len)? {
                    values.push(read_thinkthen_request_recognition_seed_span(
                        *item, context,
                    )?);
                }
                Value::Array(values)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_original_json_value(
    pointer: *const thinkthen_request_original_json_value,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok({
                let value = read_json_value(input.value, context)?;
                if value.json().map_err(|_| CarrierError::Invalid)?.len() > context.scalar_capacity
                {
                    return Err(CarrierError::Overflow);
                }
                value
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_literal_image_jpeg(
    pointer: *const thinkthen_request_literal_image_jpeg,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let _ = unsafe { borrow(pointer)? };
    let _ = context;
    Value::literal("\"image/jpeg\"")
}
pub(super) unsafe fn read_thinkthen_request_literal_image_png(
    pointer: *const thinkthen_request_literal_image_png,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let _ = unsafe { borrow(pointer)? };
    let _ = context;
    Value::literal("\"image/png\"")
}
pub(super) unsafe fn read_thinkthen_request_recognition_example_entity_list(
    pointer: *const thinkthen_request_recognition_example_entity_list,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok({
                let mut values = Vec::new();
                for item in extent(input.data, input.len)? {
                    values.push(read_thinkthen_request_recognition_example_entity(
                        *item, context,
                    )?);
                }
                Value::Array(values)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_text_list(
    pointer: *const thinkthen_request_text_list,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok({
                let mut values = Vec::new();
                for item in extent(input.data, input.len)? {
                    values.push(read_thinkthen_request_text(*item, context)?);
                }
                Value::Array(values)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_json_value_map(
    pointer: *const thinkthen_request_json_value_map,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok({
                let mut values = Vec::new();
                for item in extent(input.data, input.len)? {
                    values.push((
                        context.text(item.key)?,
                        read_thinkthen_request_json_value(item.value, context)?,
                    ));
                }
                Value::Object(values)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_annotate_question_map(
    pointer: *const thinkthen_request_annotate_question_map,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok({
                let mut values = Vec::new();
                for item in extent(input.data, input.len)? {
                    values.push((
                        context.text(item.key)?,
                        read_thinkthen_request_annotate_question(item.value, context)?,
                    ));
                }
                Value::Object(values)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_number(
    pointer: *const thinkthen_request_number,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|_context| {
        Ok(Value::Number(
            serde_json::Number::from_f64(input.value).ok_or(CarrierError::Invalid)?,
        ))
    })
}
pub(super) unsafe fn read_thinkthen_request_json_value_list(
    pointer: *const thinkthen_request_json_value_list,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok({
                let mut values = Vec::new();
                for item in extent(input.data, input.len)? {
                    values.push(read_thinkthen_request_json_value(*item, context)?);
                }
                Value::Array(values)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_name_list(
    pointer: *const thinkthen_request_name_list,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok({
                let mut values = Vec::new();
                for item in extent(input.data, input.len)? {
                    values.push(read_thinkthen_request_name(*item, context)?);
                }
                Value::Array(values)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_criterion_map(
    pointer: *const thinkthen_request_criterion_map,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok({
                let mut values = Vec::new();
                for item in extent(input.data, input.len)? {
                    values.push((
                        context.text(item.key)?,
                        read_thinkthen_request_criterion(item.value, context)?,
                    ));
                }
                Value::Object(values)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_null(
    pointer: *const thinkthen_request_null,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let _ = unsafe { borrow(pointer)? };
    let _ = context;
    Value::literal("null")
}
pub(super) unsafe fn read_thinkthen_request_input_property_map(
    pointer: *const thinkthen_request_input_property_map,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok({
                let mut values = Vec::new();
                for item in extent(input.data, input.len)? {
                    values.push((
                        context.text(item.key)?,
                        read_thinkthen_request_input_property(item.value, context)?,
                    ));
                }
                Value::Object(values)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_input_property_array_items(
    pointer: *const thinkthen_request_input_property_array_items,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let _input = unsafe { borrow(pointer)? };
    context.enter(|_context| {
        Ok({
            let fields = vec![("type".into(), Value::literal("\"string\"")?)];
            Value::Object(fields)
        })
    })
}
pub(super) unsafe fn read_thinkthen_request_description_map(
    pointer: *const thinkthen_request_description_map,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok({
                let mut values = Vec::new();
                for item in extent(input.data, input.len)? {
                    values.push((
                        context.text(item.key)?,
                        read_thinkthen_request_description(item.value, context)?,
                    ));
                }
                Value::Object(values)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_relation_list(
    pointer: *const thinkthen_request_relation_list,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok({
                let mut values = Vec::new();
                for item in extent(input.data, input.len)? {
                    values.push(read_thinkthen_request_relation(*item, context)?);
                }
                Value::Array(values)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_literal_whole(
    pointer: *const thinkthen_request_literal_whole,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let _ = unsafe { borrow(pointer)? };
    let _ = context;
    Value::literal("\"whole\"")
}
pub(super) unsafe fn read_thinkthen_request_literal_boundary_only(
    pointer: *const thinkthen_request_literal_boundary_only,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let _ = unsafe { borrow(pointer)? };
    let _ = context;
    Value::literal("\"boundary_only\"")
}
pub(super) unsafe fn read_thinkthen_request_literal_max(
    pointer: *const thinkthen_request_literal_max,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let _ = unsafe { borrow(pointer)? };
    let _ = context;
    Value::literal("\"max\"")
}
pub(super) unsafe fn read_thinkthen_request_integer(
    pointer: *const thinkthen_request_integer,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|_context| Ok(Value::Number(input.value.into())))
}
pub(super) unsafe fn read_thinkthen_request_option_schema_2_list(
    pointer: *const thinkthen_request_option_schema_2_list,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok({
                let mut values = Vec::new();
                for item in extent(input.data, input.len)? {
                    values.push(read_thinkthen_request_option_schema_2(*item, context)?);
                }
                Value::Array(values)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_call_options_no(
    pointer: *const thinkthen_request_call_options_no,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok({
                let value = read_json_value(input.value, context)?;
                if value.json().map_err(|_| CarrierError::Invalid)?.len() > context.scalar_capacity
                {
                    return Err(CarrierError::Overflow);
                }
                value
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_call_options_yes(
    pointer: *const thinkthen_request_call_options_yes,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok({
                let value = read_json_value(input.value, context)?;
                if value.json().map_err(|_| CarrierError::Invalid)?.len() > context.scalar_capacity
                {
                    return Err(CarrierError::Overflow);
                }
                value
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_option_schema_2_description(
    pointer: *const thinkthen_request_option_schema_2_description,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok({
                let value = read_json_value(input.value, context)?;
                if value.json().map_err(|_| CarrierError::Invalid)?.len() > context.scalar_capacity
                {
                    return Err(CarrierError::Overflow);
                }
                value
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_literal_line(
    pointer: *const thinkthen_request_literal_line,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let _ = unsafe { borrow(pointer)? };
    let _ = context;
    Value::literal("\"line\"")
}
pub(super) unsafe fn read_thinkthen_request_literal_window(
    pointer: *const thinkthen_request_literal_window,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let _ = unsafe { borrow(pointer)? };
    let _ = context;
    Value::literal("\"window\"")
}
pub(super) unsafe fn read_thinkthen_request_literal_file(
    pointer: *const thinkthen_request_literal_file,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let _ = unsafe { borrow(pointer)? };
    let _ = context;
    Value::literal("\"file\"")
}
pub(super) unsafe fn read_thinkthen_request_literal_document(
    pointer: *const thinkthen_request_literal_document,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let _ = unsafe { borrow(pointer)? };
    let _ = context;
    Value::literal("\"document\"")
}
pub(super) unsafe fn read_thinkthen_request_literal_lines(
    pointer: *const thinkthen_request_literal_lines,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let _ = unsafe { borrow(pointer)? };
    let _ = context;
    Value::literal("\"lines\"")
}
pub(super) unsafe fn read_thinkthen_request_literal_jsonl(
    pointer: *const thinkthen_request_literal_jsonl,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let _ = unsafe { borrow(pointer)? };
    let _ = context;
    Value::literal("\"jsonl\"")
}
pub(super) unsafe fn read_thinkthen_request_literal_csv(
    pointer: *const thinkthen_request_literal_csv,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let _ = unsafe { borrow(pointer)? };
    let _ = context;
    Value::literal("\"csv\"")
}
pub(super) unsafe fn read_thinkthen_request_literal_tsv(
    pointer: *const thinkthen_request_literal_tsv,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let _ = unsafe { borrow(pointer)? };
    let _ = context;
    Value::literal("\"tsv\"")
}
pub(super) unsafe fn read_thinkthen_request_literal_text(
    pointer: *const thinkthen_request_literal_text,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let _ = unsafe { borrow(pointer)? };
    let _ = context;
    Value::literal("\"text\"")
}
pub(super) unsafe fn read_thinkthen_request_literal_image(
    pointer: *const thinkthen_request_literal_image,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let _ = unsafe { borrow(pointer)? };
    let _ = context;
    Value::literal("\"image\"")
}
pub(super) unsafe fn read_thinkthen_request_item_list(
    pointer: *const thinkthen_request_item_list,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok({
                let mut values = Vec::new();
                for item in extent(input.data, input.len)? {
                    values.push(read_thinkthen_request_item(*item, context)?);
                }
                Value::Array(values)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_input_json_value(
    pointer: *const thinkthen_request_input_json_value,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok({
                let value = read_json_value(input.value, context)?;
                if value.json().map_err(|_| CarrierError::Invalid)?.len() > context.scalar_capacity
                {
                    return Err(CarrierError::Overflow);
                }
                value
            })
        }
    })
}
