// Generated counted carriers from the canonical request graph; do not edit.
#![allow(
    non_camel_case_types,
    missing_docs,
    clippy::too_many_lines,
    reason = "generated versioned request carriers and conversion"
)]
use super::{CarrierError, Context, Value, borrow, extent, optional};
use crate::session::views::thinkthen_complete_utf8_v1;
pub const THINKTHEN_REQUEST_READER_FAILURE_ARM_1_V1: u32 = 1;
pub const THINKTHEN_REQUEST_READER_FAILURE_ARM_2_V1: u32 = 2;
pub const THINKTHEN_REQUEST_READER_FAILURE_ARM_3_V1: u32 = 3;
pub const THINKTHEN_REQUEST_READER_FAILURE_ARM_4_V1: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_request_reader_failure_data_v1 {
    pub arm_1: *const thinkthen_request_reader_failure_producer_v1,
    pub arm_2: *const thinkthen_request_reader_failure_io_v1,
    pub arm_3: *const thinkthen_request_reader_failure_utf_8_v1,
    pub arm_4: *const thinkthen_request_reader_failure_invalid_input_v1,
}
impl std::fmt::Debug for thinkthen_request_reader_failure_data_v1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("request union payload")
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_reader_failure_v1 {
    pub kind: u32,
    pub data: thinkthen_request_reader_failure_data_v1,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_reader_failure_invalid_input_v1 {
    pub m_location: *const thinkthen_request_session_source_location_v1,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_session_source_location_v1 {
    pub m_file: *const thinkthen_request_text_v1,
    pub m_first_line: *const thinkthen_request_unsigned_v1,
    pub m_last_line: *const thinkthen_request_unsigned_v1,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_reader_failure_utf_8_v1 {
    pub m_location: *const thinkthen_request_session_source_location_v1,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_reader_failure_io_v1 {
    pub m_location: *const thinkthen_request_session_source_location_v1,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_reader_failure_producer_v1 {
    pub reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_session_descriptor_v1 {
    pub m_item: *const thinkthen_request_item_v1,
    pub m_location: *const thinkthen_request_session_source_location_v1,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_item_v1 {
    pub m_context: *const thinkthen_request_context_schema_v1,
    pub m_examples: *const thinkthen_request_recognition_example_list_v1,
    pub m_images: *const thinkthen_request_image_list_v1,
    pub m_options: *const thinkthen_request_option_schema_list_v1,
    pub m_original: *const thinkthen_request_original_v1,
    pub m_seed_spans: *const thinkthen_request_recognition_seed_span_list_v1,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_recognition_seed_span_v1 {
    pub m_end: *const thinkthen_request_unsigned_v1,
    pub m_kind: *const thinkthen_request_text_v1,
    pub m_start: *const thinkthen_request_unsigned_v1,
}
pub const THINKTHEN_REQUEST_ORIGINAL_ARM_1_V1: u32 = 1;
pub const THINKTHEN_REQUEST_ORIGINAL_ARM_2_V1: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_request_original_data_v1 {
    pub arm_1: *const thinkthen_request_original_text_v1,
    pub arm_2: *const thinkthen_request_original_json_v1,
}
impl std::fmt::Debug for thinkthen_request_original_data_v1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("request union payload")
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_original_v1 {
    pub kind: u32,
    pub data: thinkthen_request_original_data_v1,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_original_json_v1 {
    pub m_value: *const thinkthen_request_original_json_value_v1,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_original_text_v1 {
    pub m_text: *const thinkthen_request_text_v1,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_option_schema_v1 {
    pub m_description: *const thinkthen_request_json_value_v1,
    pub m_name: *const thinkthen_request_text_v1,
}
pub const THINKTHEN_REQUEST_IMAGE_ARM_1_V1: u32 = 1;
pub const THINKTHEN_REQUEST_IMAGE_ARM_2_V1: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_request_image_data_v1 {
    pub arm_1: *const thinkthen_request_image_file_v1,
    pub arm_2: *const thinkthen_request_image_bytes_v1,
}
impl std::fmt::Debug for thinkthen_request_image_data_v1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("request union payload")
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_image_v1 {
    pub kind: u32,
    pub data: thinkthen_request_image_data_v1,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_image_bytes_v1 {
    pub m_bytes: *const thinkthen_request_text_v1,
    pub m_media: *const thinkthen_request_image_media_v1,
}
pub const THINKTHEN_REQUEST_IMAGE_MEDIA_ARM_1_V1: u32 = 1;
pub const THINKTHEN_REQUEST_IMAGE_MEDIA_ARM_2_V1: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_request_image_media_data_v1 {
    pub arm_1: *const thinkthen_request_literal_image_jpeg_v1,
    pub arm_2: *const thinkthen_request_literal_image_png_v1,
}
impl std::fmt::Debug for thinkthen_request_image_media_data_v1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("request union payload")
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_image_media_v1 {
    pub kind: u32,
    pub data: thinkthen_request_image_media_data_v1,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_image_file_v1 {
    pub m_media: *const thinkthen_request_image_media_v1,
    pub m_path: *const thinkthen_request_text_v1,
}
pub const THINKTHEN_REQUEST_RECOGNITION_EXAMPLE_ARM_1_V1: u32 = 1;
pub const THINKTHEN_REQUEST_RECOGNITION_EXAMPLE_ARM_2_V1: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_request_recognition_example_data_v1 {
    pub arm_1: *const thinkthen_request_text_v1,
    pub arm_2: *const thinkthen_request_recognition_example_text_v1,
}
impl std::fmt::Debug for thinkthen_request_recognition_example_data_v1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("request union payload")
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_recognition_example_v1 {
    pub kind: u32,
    pub data: thinkthen_request_recognition_example_data_v1,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_recognition_example_text_v1 {
    pub m_entities: *const thinkthen_request_recognition_example_entity_list_v1,
    pub m_kinds: *const thinkthen_request_text_list_v1,
    pub m_text: *const thinkthen_request_text_v1,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_recognition_example_entity_v1 {
    pub m_end: *const thinkthen_request_unsigned_v1,
    pub m_kind: *const thinkthen_request_text_v1,
    pub m_start: *const thinkthen_request_unsigned_v1,
}
pub const THINKTHEN_REQUEST_CONTEXT_SCHEMA_ARM_1_V1: u32 = 1;
pub const THINKTHEN_REQUEST_CONTEXT_SCHEMA_ARM_2_V1: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_request_context_schema_data_v1 {
    pub arm_1: *const thinkthen_request_text_v1,
    pub arm_2: *const thinkthen_request_json_value_map_v1,
}
impl std::fmt::Debug for thinkthen_request_context_schema_data_v1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("request union payload")
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_context_schema_v1 {
    pub kind: u32,
    pub data: thinkthen_request_context_schema_data_v1,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_arguments_v1 {
    pub m_input: *const thinkthen_request_input_v1,
    pub m_options: *const thinkthen_request_call_options_v1,
    pub m_question: *const thinkthen_request_question_selector_v1,
}
pub const THINKTHEN_REQUEST_QUESTION_SELECTOR_ARM_1_V1: u32 = 1;
pub const THINKTHEN_REQUEST_QUESTION_SELECTOR_ARM_2_V1: u32 = 2;
pub const THINKTHEN_REQUEST_QUESTION_SELECTOR_ARM_3_V1: u32 = 3;
pub const THINKTHEN_REQUEST_QUESTION_SELECTOR_ARM_4_V1: u32 = 4;
pub const THINKTHEN_REQUEST_QUESTION_SELECTOR_ARM_5_V1: u32 = 5;
#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_request_question_selector_data_v1 {
    pub arm_1: *const thinkthen_request_question_selector_text_v1,
    pub arm_2: *const thinkthen_request_question_selector_definition_v1,
    pub arm_3: *const thinkthen_request_question_selector_file_v1,
    pub arm_4: *const thinkthen_request_question_selector_name_v1,
    pub arm_5: *const thinkthen_request_question_selector_reference_v1,
}
impl std::fmt::Debug for thinkthen_request_question_selector_data_v1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("request union payload")
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_question_selector_v1 {
    pub kind: u32,
    pub data: thinkthen_request_question_selector_data_v1,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_question_selector_reference_v1 {
    pub m_reference: *const thinkthen_request_text_v1,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_question_selector_name_v1 {
    pub m_name: *const thinkthen_request_text_v1,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_question_selector_file_v1 {
    pub m_path: *const thinkthen_request_text_v1,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_question_selector_definition_v1 {
    pub m_value: *const thinkthen_request_definition_v1,
}
pub const THINKTHEN_REQUEST_DEFINITION_ARM_1_V1: u32 = 1;
pub const THINKTHEN_REQUEST_DEFINITION_ARM_2_V1: u32 = 2;
pub const THINKTHEN_REQUEST_DEFINITION_ARM_3_V1: u32 = 3;
pub const THINKTHEN_REQUEST_DEFINITION_ARM_4_V1: u32 = 4;
pub const THINKTHEN_REQUEST_DEFINITION_ARM_5_V1: u32 = 5;
pub const THINKTHEN_REQUEST_DEFINITION_ARM_6_V1: u32 = 6;
pub const THINKTHEN_REQUEST_DEFINITION_ARM_7_V1: u32 = 7;
pub const THINKTHEN_REQUEST_DEFINITION_ARM_8_V1: u32 = 8;
#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_request_definition_data_v1 {
    pub arm_1: *const thinkthen_request_definition_authored_decide_v1,
    pub arm_2: *const thinkthen_request_definition_authored_choose_v1,
    pub arm_3: *const thinkthen_request_definition_authored_tag_v1,
    pub arm_4: *const thinkthen_request_definition_authored_score_v1,
    pub arm_5: *const thinkthen_request_definition_authored_relate_v1,
    pub arm_6: *const thinkthen_request_definition_authored_find_v1,
    pub arm_7: *const thinkthen_request_definition_recognize_definition_v1,
    pub arm_8: *const thinkthen_request_definition_annotate_definition_v1,
}
impl std::fmt::Debug for thinkthen_request_definition_data_v1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("request union payload")
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_definition_v1 {
    pub kind: u32,
    pub data: thinkthen_request_definition_data_v1,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_definition_annotate_definition_v1 {
    pub m_batch: *const thinkthen_request_json_value_v1,
    pub m_profile: *const thinkthen_request_profile_v1,
    pub m_questions: *const thinkthen_request_annotate_question_map_v1,
    pub m_threshold: *const thinkthen_request_threshold_v1,
}
pub const THINKTHEN_REQUEST_THRESHOLD_ARM_1_V1: u32 = 1;
pub const THINKTHEN_REQUEST_THRESHOLD_ARM_2_V1: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_request_threshold_data_v1 {
    pub arm_1: *const thinkthen_request_number_v1,
    pub arm_2: *const thinkthen_request_text_v1,
}
impl std::fmt::Debug for thinkthen_request_threshold_data_v1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("request union payload")
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_threshold_v1 {
    pub kind: u32,
    pub data: thinkthen_request_threshold_data_v1,
}
pub const THINKTHEN_REQUEST_ANNOTATE_QUESTION_ARM_1_V1: u32 = 1;
pub const THINKTHEN_REQUEST_ANNOTATE_QUESTION_ARM_2_V1: u32 = 2;
pub const THINKTHEN_REQUEST_ANNOTATE_QUESTION_ARM_3_V1: u32 = 3;
pub const THINKTHEN_REQUEST_ANNOTATE_QUESTION_ARM_4_V1: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_request_annotate_question_data_v1 {
    pub arm_1: *const thinkthen_request_annotate_question_decide_v1,
    pub arm_2: *const thinkthen_request_annotate_question_choose_v1,
    pub arm_3: *const thinkthen_request_annotate_question_tag_v1,
    pub arm_4: *const thinkthen_request_annotate_question_score_v1,
}
impl std::fmt::Debug for thinkthen_request_annotate_question_data_v1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("request union payload")
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_annotate_question_v1 {
    pub kind: u32,
    pub data: thinkthen_request_annotate_question_data_v1,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_annotate_question_score_v1 {
    pub m_context_schema: *const thinkthen_request_input_declaration_v1,
    pub m_item_schema: *const thinkthen_request_input_declaration_v1,
    pub m_levels: *const thinkthen_request_levels_v1,
    pub m_name: *const thinkthen_request_text_v1,
    pub m_on: *const thinkthen_request_pointers_v1,
    pub m_score: *const thinkthen_request_question_text_v1,
    pub m_wording_version: *const thinkthen_request_integer_v1,
}
pub const THINKTHEN_REQUEST_QUESTION_TEXT_ARM_1_V1: u32 = 1;
pub const THINKTHEN_REQUEST_QUESTION_TEXT_ARM_2_V1: u32 = 2;
pub const THINKTHEN_REQUEST_QUESTION_TEXT_ARM_3_V1: u32 = 3;
#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_request_question_text_data_v1 {
    pub arm_1: *const thinkthen_request_text_v1,
    pub arm_2: *const thinkthen_request_json_value_map_v1,
    pub arm_3: *const thinkthen_request_json_value_list_v1,
}
impl std::fmt::Debug for thinkthen_request_question_text_data_v1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("request union payload")
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_question_text_v1 {
    pub kind: u32,
    pub data: thinkthen_request_question_text_data_v1,
}
pub const THINKTHEN_REQUEST_POINTERS_ARM_1_V1: u32 = 1;
pub const THINKTHEN_REQUEST_POINTERS_ARM_2_V1: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_request_pointers_data_v1 {
    pub arm_1: *const thinkthen_request_text_v1,
    pub arm_2: *const thinkthen_request_text_list_v1,
}
impl std::fmt::Debug for thinkthen_request_pointers_data_v1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("request union payload")
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_pointers_v1 {
    pub kind: u32,
    pub data: thinkthen_request_pointers_data_v1,
}
pub const THINKTHEN_REQUEST_LEVELS_ARM_1_V1: u32 = 1;
pub const THINKTHEN_REQUEST_LEVELS_ARM_2_V1: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_request_levels_data_v1 {
    pub arm_1: *const thinkthen_request_name_list_v1,
    pub arm_2: *const thinkthen_request_criterion_map_v1,
}
impl std::fmt::Debug for thinkthen_request_levels_data_v1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("request union payload")
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_levels_v1 {
    pub kind: u32,
    pub data: thinkthen_request_levels_data_v1,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_name_v1 {
    pub text: thinkthen_complete_utf8_v1,
}
pub const THINKTHEN_REQUEST_CRITERION_ARM_1_V1: u32 = 1;
pub const THINKTHEN_REQUEST_CRITERION_ARM_2_V1: u32 = 2;
pub const THINKTHEN_REQUEST_CRITERION_ARM_3_V1: u32 = 3;
pub const THINKTHEN_REQUEST_CRITERION_ARM_4_V1: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_request_criterion_data_v1 {
    pub arm_1: *const thinkthen_request_text_v1,
    pub arm_2: *const thinkthen_request_json_value_map_v1,
    pub arm_3: *const thinkthen_request_json_value_list_v1,
    pub arm_4: *const thinkthen_request_null_v1,
}
impl std::fmt::Debug for thinkthen_request_criterion_data_v1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("request union payload")
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_criterion_v1 {
    pub kind: u32,
    pub data: thinkthen_request_criterion_data_v1,
}
pub const THINKTHEN_REQUEST_INPUT_DECLARATION_ARM_1_V1: u32 = 1;
pub const THINKTHEN_REQUEST_INPUT_DECLARATION_ARM_2_V1: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_request_input_declaration_data_v1 {
    pub arm_1: *const thinkthen_request_input_declaration_string_v1,
    pub arm_2: *const thinkthen_request_input_declaration_object_v1,
}
impl std::fmt::Debug for thinkthen_request_input_declaration_data_v1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("request union payload")
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_input_declaration_v1 {
    pub kind: u32,
    pub data: thinkthen_request_input_declaration_data_v1,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_input_declaration_object_v1 {
    pub m_properties: *const thinkthen_request_input_property_map_v1,
    pub m_required: *const thinkthen_request_text_list_v1,
}
pub const THINKTHEN_REQUEST_INPUT_PROPERTY_ARM_1_V1: u32 = 1;
pub const THINKTHEN_REQUEST_INPUT_PROPERTY_ARM_2_V1: u32 = 2;
pub const THINKTHEN_REQUEST_INPUT_PROPERTY_ARM_3_V1: u32 = 3;
pub const THINKTHEN_REQUEST_INPUT_PROPERTY_ARM_4_V1: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_request_input_property_data_v1 {
    pub arm_1: *const thinkthen_request_input_property_string_v1,
    pub arm_2: *const thinkthen_request_input_property_number_v1,
    pub arm_3: *const thinkthen_request_input_property_boolean_v1,
    pub arm_4: *const thinkthen_request_input_property_array_v1,
}
impl std::fmt::Debug for thinkthen_request_input_property_data_v1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("request union payload")
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_input_property_v1 {
    pub kind: u32,
    pub data: thinkthen_request_input_property_data_v1,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_input_property_array_v1 {
    pub m_items: *const thinkthen_request_input_property_array_items_v1,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_input_property_boolean_v1 {
    pub reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_input_property_number_v1 {
    pub reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_input_property_string_v1 {
    pub reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_input_declaration_string_v1 {
    pub reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_annotate_question_tag_v1 {
    pub m_context_schema: *const thinkthen_request_input_declaration_v1,
    pub m_item_schema: *const thinkthen_request_input_declaration_v1,
    pub m_labels: *const thinkthen_request_labels_v1,
    pub m_name: *const thinkthen_request_text_v1,
    pub m_on: *const thinkthen_request_pointers_v1,
    pub m_tag: *const thinkthen_request_question_text_v1,
    pub m_threshold: *const thinkthen_request_cut_v1,
    pub m_wording_version: *const thinkthen_request_integer_v1,
}
pub const THINKTHEN_REQUEST_CUT_ARM_1_V1: u32 = 1;
pub const THINKTHEN_REQUEST_CUT_ARM_2_V1: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_request_cut_data_v1 {
    pub arm_1: *const thinkthen_request_number_v1,
    pub arm_2: *const thinkthen_request_text_v1,
}
impl std::fmt::Debug for thinkthen_request_cut_data_v1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("request union payload")
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_cut_v1 {
    pub kind: u32,
    pub data: thinkthen_request_cut_data_v1,
}
pub const THINKTHEN_REQUEST_LABELS_ARM_1_V1: u32 = 1;
pub const THINKTHEN_REQUEST_LABELS_ARM_2_V1: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_request_labels_data_v1 {
    pub arm_1: *const thinkthen_request_name_list_v1,
    pub arm_2: *const thinkthen_request_description_map_v1,
}
impl std::fmt::Debug for thinkthen_request_labels_data_v1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("request union payload")
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_labels_v1 {
    pub kind: u32,
    pub data: thinkthen_request_labels_data_v1,
}
pub const THINKTHEN_REQUEST_DESCRIPTION_ARM_1_V1: u32 = 1;
pub const THINKTHEN_REQUEST_DESCRIPTION_ARM_2_V1: u32 = 2;
pub const THINKTHEN_REQUEST_DESCRIPTION_ARM_3_V1: u32 = 3;
pub const THINKTHEN_REQUEST_DESCRIPTION_ARM_4_V1: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_request_description_data_v1 {
    pub arm_1: *const thinkthen_request_text_v1,
    pub arm_2: *const thinkthen_request_json_value_map_v1,
    pub arm_3: *const thinkthen_request_json_value_list_v1,
    pub arm_4: *const thinkthen_request_null_v1,
}
impl std::fmt::Debug for thinkthen_request_description_data_v1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("request union payload")
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_description_v1 {
    pub kind: u32,
    pub data: thinkthen_request_description_data_v1,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_annotate_question_choose_v1 {
    pub m_choose: *const thinkthen_request_question_text_v1,
    pub m_context_schema: *const thinkthen_request_input_declaration_v1,
    pub m_item_schema: *const thinkthen_request_input_declaration_v1,
    pub m_name: *const thinkthen_request_text_v1,
    pub m_on: *const thinkthen_request_pointers_v1,
    pub m_options: *const thinkthen_request_options_v1,
    pub m_threshold: *const thinkthen_request_cut_v1,
    pub m_wording_version: *const thinkthen_request_integer_v1,
}
pub const THINKTHEN_REQUEST_OPTIONS_ARM_1_V1: u32 = 1;
pub const THINKTHEN_REQUEST_OPTIONS_ARM_2_V1: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_request_options_data_v1 {
    pub arm_1: *const thinkthen_request_name_list_v1,
    pub arm_2: *const thinkthen_request_description_map_v1,
}
impl std::fmt::Debug for thinkthen_request_options_data_v1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("request union payload")
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_options_v1 {
    pub kind: u32,
    pub data: thinkthen_request_options_data_v1,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_annotate_question_decide_v1 {
    pub m_context_schema: *const thinkthen_request_input_declaration_v1,
    pub m_decide: *const thinkthen_request_question_text_v1,
    pub m_false: *const thinkthen_request_criterion_v1,
    pub m_item_schema: *const thinkthen_request_input_declaration_v1,
    pub m_name: *const thinkthen_request_text_v1,
    pub m_on: *const thinkthen_request_pointers_v1,
    pub m_threshold: *const thinkthen_request_threshold_v1,
    pub m_true: *const thinkthen_request_criterion_v1,
    pub m_wording_version: *const thinkthen_request_integer_v1,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_profile_v1 {
    pub text: thinkthen_complete_utf8_v1,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_definition_recognize_definition_v1 {
    pub m_context_schema: *const thinkthen_request_input_declaration_v1,
    pub m_item_schema: *const thinkthen_request_input_declaration_v1,
    pub m_model: *const thinkthen_request_name_v1,
    pub m_name: *const thinkthen_request_text_v1,
    pub m_on: *const thinkthen_request_pointers_v1,
    pub m_profile: *const thinkthen_request_profile_v1,
    pub m_recognize: *const thinkthen_request_recognize_question_v1,
    pub m_relation_threshold: *const thinkthen_request_cut_v1,
    pub m_threshold: *const thinkthen_request_cut_v1,
    pub m_wording_version: *const thinkthen_request_integer_v1,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_recognize_question_v1 {
    pub m_entity_definition: *const thinkthen_request_question_text_v1,
    pub m_instructions: *const thinkthen_request_question_text_v1,
    pub m_kinds: *const thinkthen_request_description_map_v1,
    pub m_mode: *const thinkthen_request_recognition_mode_v1,
    pub m_relations: *const thinkthen_request_relation_list_v1,
    pub m_snippet_pieces: *const thinkthen_request_unsigned_v1,
    pub m_stage_context: *const thinkthen_request_recognition_stage_context_v1,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_recognition_stage_context_v1 {
    pub m_boundary: *const thinkthen_request_text_v1,
    pub m_kind_edge: *const thinkthen_request_text_v1,
    pub m_relation: *const thinkthen_request_text_v1,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_relation_v1 {
    pub m_either: *const thinkthen_request_boolean_v1,
    pub m_name: *const thinkthen_request_name_v1,
    pub m_reads: *const thinkthen_request_name_v1,
    pub m_single: *const thinkthen_request_boolean_v1,
    pub m_source: *const thinkthen_request_name_v1,
    pub m_target: *const thinkthen_request_name_v1,
}
pub const THINKTHEN_REQUEST_RECOGNITION_MODE_ARM_1_V1: u32 = 1;
pub const THINKTHEN_REQUEST_RECOGNITION_MODE_ARM_2_V1: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_request_recognition_mode_data_v1 {
    pub arm_1: *const thinkthen_request_literal_whole_v1,
    pub arm_2: *const thinkthen_request_literal_boundary_only_v1,
}
impl std::fmt::Debug for thinkthen_request_recognition_mode_data_v1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("request union payload")
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_recognition_mode_v1 {
    pub kind: u32,
    pub data: thinkthen_request_recognition_mode_data_v1,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_definition_authored_find_v1 {
    pub m_context_schema: *const thinkthen_request_input_declaration_v1,
    pub m_find: *const thinkthen_request_question_text_v1,
    pub m_item_schema: *const thinkthen_request_input_declaration_v1,
    pub m_model: *const thinkthen_request_name_v1,
    pub m_name: *const thinkthen_request_text_v1,
    pub m_on: *const thinkthen_request_pointers_v1,
    pub m_profile: *const thinkthen_request_profile_v1,
    pub m_wording_version: *const thinkthen_request_integer_v1,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_definition_authored_relate_v1 {
    pub m_context_schema: *const thinkthen_request_input_declaration_v1,
    pub m_item_schema: *const thinkthen_request_input_declaration_v1,
    pub m_model: *const thinkthen_request_name_v1,
    pub m_name: *const thinkthen_request_text_v1,
    pub m_profile: *const thinkthen_request_profile_v1,
    pub m_relate: *const thinkthen_request_relate_question_v1,
    pub m_threshold: *const thinkthen_request_cut_v1,
    pub m_wording_version: *const thinkthen_request_integer_v1,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_relate_question_v1 {
    pub m_fields: *const thinkthen_request_relate_fields_v1,
    pub m_relations: *const thinkthen_request_relation_list_v1,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_relate_fields_v1 {
    pub m_kind: *const thinkthen_request_text_v1,
    pub m_name: *const thinkthen_request_text_v1,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_definition_authored_score_v1 {
    pub m_batch: *const thinkthen_request_definition_authored_score_batch_v1,
    pub m_context_schema: *const thinkthen_request_input_declaration_v1,
    pub m_item_schema: *const thinkthen_request_input_declaration_v1,
    pub m_levels: *const thinkthen_request_levels_v1,
    pub m_model: *const thinkthen_request_name_v1,
    pub m_name: *const thinkthen_request_text_v1,
    pub m_on: *const thinkthen_request_pointers_v1,
    pub m_profile: *const thinkthen_request_profile_v1,
    pub m_score: *const thinkthen_request_question_text_v1,
    pub m_wording_version: *const thinkthen_request_integer_v1,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_definition_authored_tag_v1 {
    pub m_batch: *const thinkthen_request_definition_authored_score_batch_v1,
    pub m_context_schema: *const thinkthen_request_input_declaration_v1,
    pub m_item_schema: *const thinkthen_request_input_declaration_v1,
    pub m_labels: *const thinkthen_request_labels_v1,
    pub m_model: *const thinkthen_request_name_v1,
    pub m_name: *const thinkthen_request_text_v1,
    pub m_on: *const thinkthen_request_pointers_v1,
    pub m_profile: *const thinkthen_request_profile_v1,
    pub m_tag: *const thinkthen_request_question_text_v1,
    pub m_threshold: *const thinkthen_request_cut_v1,
    pub m_wording_version: *const thinkthen_request_integer_v1,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_definition_authored_choose_v1 {
    pub m_batch: *const thinkthen_request_definition_authored_score_batch_v1,
    pub m_choose: *const thinkthen_request_question_text_v1,
    pub m_context_schema: *const thinkthen_request_input_declaration_v1,
    pub m_item_schema: *const thinkthen_request_input_declaration_v1,
    pub m_model: *const thinkthen_request_name_v1,
    pub m_name: *const thinkthen_request_text_v1,
    pub m_on: *const thinkthen_request_pointers_v1,
    pub m_options: *const thinkthen_request_options_v1,
    pub m_profile: *const thinkthen_request_profile_v1,
    pub m_threshold: *const thinkthen_request_cut_v1,
    pub m_wording_version: *const thinkthen_request_integer_v1,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_definition_authored_decide_v1 {
    pub m_batch: *const thinkthen_request_definition_authored_score_batch_v1,
    pub m_context_schema: *const thinkthen_request_input_declaration_v1,
    pub m_decide: *const thinkthen_request_question_text_v1,
    pub m_false: *const thinkthen_request_criterion_v1,
    pub m_item_schema: *const thinkthen_request_input_declaration_v1,
    pub m_model: *const thinkthen_request_name_v1,
    pub m_name: *const thinkthen_request_text_v1,
    pub m_on: *const thinkthen_request_pointers_v1,
    pub m_profile: *const thinkthen_request_profile_v1,
    pub m_threshold: *const thinkthen_request_threshold_v1,
    pub m_true: *const thinkthen_request_criterion_v1,
    pub m_wording_version: *const thinkthen_request_integer_v1,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_question_selector_text_v1 {
    pub m_text: *const thinkthen_request_text_v1,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_call_options_v1 {
    pub m_attempts: *const thinkthen_request_boolean_v1,
    pub m_batch: *const thinkthen_request_batch_v1,
    pub m_context: *const thinkthen_request_text_v1,
    pub m_context_field: *const thinkthen_request_text_v1,
    pub m_deadline_ms: *const thinkthen_request_integer_v1,
    pub m_details: *const thinkthen_request_boolean_v1,
    pub m_examples: *const thinkthen_request_recognition_example_list_v1,
    pub m_examples_field: *const thinkthen_request_text_v1,
    pub m_field: *const thinkthen_request_text_list_v1,
    pub m_files_only: *const thinkthen_request_boolean_v1,
    pub m_max_requests_total: *const thinkthen_request_unsigned_v1,
    pub m_mode: *const thinkthen_request_recognition_mode_v1,
    pub m_model: *const thinkthen_request_text_v1,
    pub m_none: *const thinkthen_request_boolean_v1,
    pub m_options_field: *const thinkthen_request_text_v1,
    pub m_relation_threshold: *const thinkthen_request_call_threshold_v1,
    pub m_seed_spans: *const thinkthen_request_recognition_seed_span_list_v1,
    pub m_seed_spans_field: *const thinkthen_request_text_v1,
    pub m_snippet_pieces: *const thinkthen_request_unsigned_v1,
    pub m_stage_context: *const thinkthen_request_recognition_stage_context_v1,
    pub m_threshold: *const thinkthen_request_call_threshold_v1,
    pub m_top: *const thinkthen_request_unsigned_v1,
}
pub const THINKTHEN_REQUEST_CALL_THRESHOLD_ARM_1_V1: u32 = 1;
pub const THINKTHEN_REQUEST_CALL_THRESHOLD_ARM_2_V1: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_request_call_threshold_data_v1 {
    pub arm_1: *const thinkthen_request_number_v1,
    pub arm_2: *const thinkthen_request_text_v1,
}
impl std::fmt::Debug for thinkthen_request_call_threshold_data_v1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("request union payload")
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_call_threshold_v1 {
    pub kind: u32,
    pub data: thinkthen_request_call_threshold_data_v1,
}
pub const THINKTHEN_REQUEST_BATCH_ARM_1_V1: u32 = 1;
pub const THINKTHEN_REQUEST_BATCH_ARM_2_V1: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_request_batch_data_v1 {
    pub arm_1: *const thinkthen_request_unsigned_v1,
    pub arm_2: *const thinkthen_request_text_v1,
}
impl std::fmt::Debug for thinkthen_request_batch_data_v1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("request union payload")
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_batch_v1 {
    pub kind: u32,
    pub data: thinkthen_request_batch_data_v1,
}
pub const THINKTHEN_REQUEST_INPUT_ARM_1_V1: u32 = 1;
pub const THINKTHEN_REQUEST_INPUT_ARM_2_V1: u32 = 2;
pub const THINKTHEN_REQUEST_INPUT_ARM_3_V1: u32 = 3;
pub const THINKTHEN_REQUEST_INPUT_ARM_4_V1: u32 = 4;
pub const THINKTHEN_REQUEST_INPUT_ARM_5_V1: u32 = 5;
pub const THINKTHEN_REQUEST_INPUT_ARM_6_V1: u32 = 6;
pub const THINKTHEN_REQUEST_INPUT_ARM_7_V1: u32 = 7;
#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_request_input_data_v1 {
    pub arm_1: *const thinkthen_request_input_text_v1,
    pub arm_2: *const thinkthen_request_input_json_v1,
    pub arm_3: *const thinkthen_request_input_records_v1,
    pub arm_4: *const thinkthen_request_input_units_v1,
    pub arm_5: *const thinkthen_request_input_entities_v1,
    pub arm_6: *const thinkthen_request_input_source_v1,
    pub arm_7: *const thinkthen_request_input_feed_v1,
}
impl std::fmt::Debug for thinkthen_request_input_data_v1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("request union payload")
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_input_v1 {
    pub kind: u32,
    pub data: thinkthen_request_input_data_v1,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_input_feed_v1 {
    pub m_framing: *const thinkthen_request_framing_v1,
    pub m_images: *const thinkthen_request_image_list_v1,
    pub m_name: *const thinkthen_request_text_v1,
    pub m_reading: *const thinkthen_request_reader_v1,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_reader_v1 {
    pub m_unit: *const thinkthen_request_source_unit_v1,
    pub m_window: *const thinkthen_request_unsigned_v1,
}
pub const THINKTHEN_REQUEST_SOURCE_UNIT_ARM_1_V1: u32 = 1;
pub const THINKTHEN_REQUEST_SOURCE_UNIT_ARM_2_V1: u32 = 2;
pub const THINKTHEN_REQUEST_SOURCE_UNIT_ARM_3_V1: u32 = 3;
#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_request_source_unit_data_v1 {
    pub arm_1: *const thinkthen_request_literal_line_v1,
    pub arm_2: *const thinkthen_request_literal_window_v1,
    pub arm_3: *const thinkthen_request_literal_file_v1,
}
impl std::fmt::Debug for thinkthen_request_source_unit_data_v1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("request union payload")
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_source_unit_v1 {
    pub kind: u32,
    pub data: thinkthen_request_source_unit_data_v1,
}
pub const THINKTHEN_REQUEST_FRAMING_ARM_1_V1: u32 = 1;
pub const THINKTHEN_REQUEST_FRAMING_ARM_2_V1: u32 = 2;
pub const THINKTHEN_REQUEST_FRAMING_ARM_3_V1: u32 = 3;
pub const THINKTHEN_REQUEST_FRAMING_ARM_4_V1: u32 = 4;
pub const THINKTHEN_REQUEST_FRAMING_ARM_5_V1: u32 = 5;
#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_request_framing_data_v1 {
    pub arm_1: *const thinkthen_request_literal_document_v1,
    pub arm_2: *const thinkthen_request_literal_lines_v1,
    pub arm_3: *const thinkthen_request_literal_jsonl_v1,
    pub arm_4: *const thinkthen_request_literal_csv_v1,
    pub arm_5: *const thinkthen_request_literal_tsv_v1,
}
impl std::fmt::Debug for thinkthen_request_framing_data_v1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("request union payload")
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_framing_v1 {
    pub kind: u32,
    pub data: thinkthen_request_framing_data_v1,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_input_source_v1 {
    pub m_source: *const thinkthen_request_source_v1,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_source_v1 {
    pub m_framing: *const thinkthen_request_framing_v1,
    pub m_media: *const thinkthen_request_reader_media_v1,
    pub m_paths: *const thinkthen_request_text_list_v1,
    pub m_reading: *const thinkthen_request_reader_v1,
}
pub const THINKTHEN_REQUEST_READER_MEDIA_ARM_1_V1: u32 = 1;
pub const THINKTHEN_REQUEST_READER_MEDIA_ARM_2_V1: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_request_reader_media_data_v1 {
    pub arm_1: *const thinkthen_request_literal_text_v1,
    pub arm_2: *const thinkthen_request_literal_image_v1,
}
impl std::fmt::Debug for thinkthen_request_reader_media_data_v1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("request union payload")
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_reader_media_v1 {
    pub kind: u32,
    pub data: thinkthen_request_reader_media_data_v1,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_input_entities_v1 {
    pub m_items: *const thinkthen_request_item_list_v1,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_input_units_v1 {
    pub m_items: *const thinkthen_request_item_list_v1,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_input_records_v1 {
    pub m_items: *const thinkthen_request_item_list_v1,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_input_json_v1 {
    pub m_images: *const thinkthen_request_image_list_v1,
    pub m_value: *const thinkthen_request_input_json_value_v1,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_input_text_v1 {
    pub m_images: *const thinkthen_request_image_list_v1,
    pub m_text: *const thinkthen_request_text_v1,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_text_v1 {
    pub text: thinkthen_complete_utf8_v1,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_unsigned_v1 {
    pub value: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_recognition_example_list_v1 {
    pub data: *const *const thinkthen_request_recognition_example_v1,
    pub len: usize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_image_list_v1 {
    pub data: *const *const thinkthen_request_image_v1,
    pub len: usize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_option_schema_list_v1 {
    pub data: *const *const thinkthen_request_option_schema_v1,
    pub len: usize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_recognition_seed_span_list_v1 {
    pub data: *const *const thinkthen_request_recognition_seed_span_v1,
    pub len: usize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_original_json_value_v1 {
    pub text: thinkthen_complete_utf8_v1,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_json_value_v1 {
    pub text: thinkthen_complete_utf8_v1,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_literal_image_jpeg_v1 {
    pub reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_literal_image_png_v1 {
    pub reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_recognition_example_entity_list_v1 {
    pub data: *const *const thinkthen_request_recognition_example_entity_v1,
    pub len: usize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_text_list_v1 {
    pub data: *const *const thinkthen_request_text_v1,
    pub len: usize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_json_value_map_entry_v1 {
    pub key: thinkthen_complete_utf8_v1,
    pub value: *const thinkthen_request_json_value_v1,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_json_value_map_v1 {
    pub data: *const thinkthen_request_json_value_map_entry_v1,
    pub len: usize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_annotate_question_map_entry_v1 {
    pub key: thinkthen_complete_utf8_v1,
    pub value: *const thinkthen_request_annotate_question_v1,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_annotate_question_map_v1 {
    pub data: *const thinkthen_request_annotate_question_map_entry_v1,
    pub len: usize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_number_v1 {
    pub value: f64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_integer_v1 {
    pub value: i64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_json_value_list_v1 {
    pub data: *const *const thinkthen_request_json_value_v1,
    pub len: usize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_name_list_v1 {
    pub data: *const *const thinkthen_request_name_v1,
    pub len: usize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_criterion_map_entry_v1 {
    pub key: thinkthen_complete_utf8_v1,
    pub value: *const thinkthen_request_criterion_v1,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_criterion_map_v1 {
    pub data: *const thinkthen_request_criterion_map_entry_v1,
    pub len: usize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_null_v1 {
    pub reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_input_property_map_entry_v1 {
    pub key: thinkthen_complete_utf8_v1,
    pub value: *const thinkthen_request_input_property_v1,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_input_property_map_v1 {
    pub data: *const thinkthen_request_input_property_map_entry_v1,
    pub len: usize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_input_property_array_items_v1 {
    pub reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_description_map_entry_v1 {
    pub key: thinkthen_complete_utf8_v1,
    pub value: *const thinkthen_request_description_v1,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_description_map_v1 {
    pub data: *const thinkthen_request_description_map_entry_v1,
    pub len: usize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_relation_list_v1 {
    pub data: *const *const thinkthen_request_relation_v1,
    pub len: usize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_boolean_v1 {
    pub value: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_literal_whole_v1 {
    pub reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_literal_boundary_only_v1 {
    pub reserved: u32,
}
pub const THINKTHEN_REQUEST_DEFINITION_AUTHORED_SCORE_BATCH_ARM_1_V1: u32 = 1;
pub const THINKTHEN_REQUEST_DEFINITION_AUTHORED_SCORE_BATCH_ARM_2_V1: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_request_definition_authored_score_batch_data_v1 {
    pub arm_1: *const thinkthen_request_literal_max_v1,
    pub arm_2: *const thinkthen_request_integer_v1,
}
impl std::fmt::Debug for thinkthen_request_definition_authored_score_batch_data_v1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("request union payload")
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_definition_authored_score_batch_v1 {
    pub kind: u32,
    pub data: thinkthen_request_definition_authored_score_batch_data_v1,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_literal_line_v1 {
    pub reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_literal_window_v1 {
    pub reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_literal_file_v1 {
    pub reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_literal_document_v1 {
    pub reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_literal_lines_v1 {
    pub reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_literal_jsonl_v1 {
    pub reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_literal_csv_v1 {
    pub reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_literal_tsv_v1 {
    pub reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_literal_text_v1 {
    pub reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_literal_image_v1 {
    pub reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_item_list_v1 {
    pub data: *const *const thinkthen_request_item_v1,
    pub len: usize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_input_json_value_v1 {
    pub text: thinkthen_complete_utf8_v1,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_request_literal_max_v1 {
    pub reserved: u32,
}
pub(super) unsafe fn read_thinkthen_request_reader_failure_v1(
    pointer: *const thinkthen_request_reader_failure_v1,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok(match input.kind {
                THINKTHEN_REQUEST_READER_FAILURE_ARM_1_V1 => {
                    read_thinkthen_request_reader_failure_producer_v1(input.data.arm_1, context)?
                }
                THINKTHEN_REQUEST_READER_FAILURE_ARM_2_V1 => {
                    read_thinkthen_request_reader_failure_io_v1(input.data.arm_2, context)?
                }
                THINKTHEN_REQUEST_READER_FAILURE_ARM_3_V1 => {
                    read_thinkthen_request_reader_failure_utf_8_v1(input.data.arm_3, context)?
                }
                THINKTHEN_REQUEST_READER_FAILURE_ARM_4_V1 => {
                    read_thinkthen_request_reader_failure_invalid_input_v1(
                        input.data.arm_4,
                        context,
                    )?
                }
                _ => return Err(CarrierError::Invalid),
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_reader_failure_invalid_input_v1(
    pointer: *const thinkthen_request_reader_failure_invalid_input_v1,
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
                    read_thinkthen_request_session_source_location_v1,
                )?;
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_session_source_location_v1(
    pointer: *const thinkthen_request_session_source_location_v1,
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
                    read_thinkthen_request_text_v1(input.m_file, context)?,
                )];
                optional(
                    "first_line",
                    input.m_first_line,
                    context,
                    &mut fields,
                    read_thinkthen_request_unsigned_v1,
                )?;
                optional(
                    "last_line",
                    input.m_last_line,
                    context,
                    &mut fields,
                    read_thinkthen_request_unsigned_v1,
                )?;
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_reader_failure_utf_8_v1(
    pointer: *const thinkthen_request_reader_failure_utf_8_v1,
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
                    read_thinkthen_request_session_source_location_v1,
                )?;
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_reader_failure_io_v1(
    pointer: *const thinkthen_request_reader_failure_io_v1,
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
                    read_thinkthen_request_session_source_location_v1,
                )?;
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_reader_failure_producer_v1(
    pointer: *const thinkthen_request_reader_failure_producer_v1,
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
pub(super) unsafe fn read_thinkthen_request_session_descriptor_v1(
    pointer: *const thinkthen_request_session_descriptor_v1,
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
                    read_thinkthen_request_item_v1(input.m_item, context)?,
                )];
                optional(
                    "location",
                    input.m_location,
                    context,
                    &mut fields,
                    read_thinkthen_request_session_source_location_v1,
                )?;
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_item_v1(
    pointer: *const thinkthen_request_item_v1,
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
                    read_thinkthen_request_context_schema_v1,
                )?;
                optional(
                    "examples",
                    input.m_examples,
                    context,
                    &mut fields,
                    read_thinkthen_request_recognition_example_list_v1,
                )?;
                optional(
                    "images",
                    input.m_images,
                    context,
                    &mut fields,
                    read_thinkthen_request_image_list_v1,
                )?;
                optional(
                    "options",
                    input.m_options,
                    context,
                    &mut fields,
                    read_thinkthen_request_option_schema_list_v1,
                )?;
                optional(
                    "original",
                    input.m_original,
                    context,
                    &mut fields,
                    read_thinkthen_request_original_v1,
                )?;
                optional(
                    "seed_spans",
                    input.m_seed_spans,
                    context,
                    &mut fields,
                    read_thinkthen_request_recognition_seed_span_list_v1,
                )?;
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_recognition_seed_span_v1(
    pointer: *const thinkthen_request_recognition_seed_span_v1,
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
                        read_thinkthen_request_unsigned_v1(input.m_end, context)?,
                    ),
                    (
                        "start".into(),
                        read_thinkthen_request_unsigned_v1(input.m_start, context)?,
                    ),
                ];
                optional(
                    "kind",
                    input.m_kind,
                    context,
                    &mut fields,
                    read_thinkthen_request_text_v1,
                )?;
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_original_v1(
    pointer: *const thinkthen_request_original_v1,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok(match input.kind {
                THINKTHEN_REQUEST_ORIGINAL_ARM_1_V1 => {
                    read_thinkthen_request_original_text_v1(input.data.arm_1, context)?
                }
                THINKTHEN_REQUEST_ORIGINAL_ARM_2_V1 => {
                    read_thinkthen_request_original_json_v1(input.data.arm_2, context)?
                }
                _ => return Err(CarrierError::Invalid),
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_original_json_v1(
    pointer: *const thinkthen_request_original_json_v1,
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
                        read_thinkthen_request_original_json_value_v1(input.m_value, context)?,
                    ),
                ];
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_original_text_v1(
    pointer: *const thinkthen_request_original_text_v1,
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
                        read_thinkthen_request_text_v1(input.m_text, context)?,
                    ),
                ];
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_option_schema_v1(
    pointer: *const thinkthen_request_option_schema_v1,
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
                    read_thinkthen_request_text_v1(input.m_name, context)?,
                )];
                optional(
                    "description",
                    input.m_description,
                    context,
                    &mut fields,
                    read_thinkthen_request_json_value_v1,
                )?;
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_image_v1(
    pointer: *const thinkthen_request_image_v1,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok(match input.kind {
                THINKTHEN_REQUEST_IMAGE_ARM_1_V1 => {
                    read_thinkthen_request_image_file_v1(input.data.arm_1, context)?
                }
                THINKTHEN_REQUEST_IMAGE_ARM_2_V1 => {
                    read_thinkthen_request_image_bytes_v1(input.data.arm_2, context)?
                }
                _ => return Err(CarrierError::Invalid),
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_image_bytes_v1(
    pointer: *const thinkthen_request_image_bytes_v1,
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
                        read_thinkthen_request_text_v1(input.m_bytes, context)?,
                    ),
                    ("kind".into(), Value::literal("\"bytes\"")?),
                    (
                        "media".into(),
                        read_thinkthen_request_image_media_v1(input.m_media, context)?,
                    ),
                ];
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_image_media_v1(
    pointer: *const thinkthen_request_image_media_v1,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok(match input.kind {
                THINKTHEN_REQUEST_IMAGE_MEDIA_ARM_1_V1 => {
                    read_thinkthen_request_literal_image_jpeg_v1(input.data.arm_1, context)?
                }
                THINKTHEN_REQUEST_IMAGE_MEDIA_ARM_2_V1 => {
                    read_thinkthen_request_literal_image_png_v1(input.data.arm_2, context)?
                }
                _ => return Err(CarrierError::Invalid),
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_image_file_v1(
    pointer: *const thinkthen_request_image_file_v1,
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
                        read_thinkthen_request_text_v1(input.m_path, context)?,
                    ),
                ];
                optional(
                    "media",
                    input.m_media,
                    context,
                    &mut fields,
                    read_thinkthen_request_image_media_v1,
                )?;
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_recognition_example_v1(
    pointer: *const thinkthen_request_recognition_example_v1,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok(match input.kind {
                THINKTHEN_REQUEST_RECOGNITION_EXAMPLE_ARM_1_V1 => {
                    read_thinkthen_request_text_v1(input.data.arm_1, context)?
                }
                THINKTHEN_REQUEST_RECOGNITION_EXAMPLE_ARM_2_V1 => {
                    read_thinkthen_request_recognition_example_text_v1(input.data.arm_2, context)?
                }
                _ => return Err(CarrierError::Invalid),
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_recognition_example_text_v1(
    pointer: *const thinkthen_request_recognition_example_text_v1,
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
                        read_thinkthen_request_recognition_example_entity_list_v1(
                            input.m_entities,
                            context,
                        )?,
                    ),
                    (
                        "text".into(),
                        read_thinkthen_request_text_v1(input.m_text, context)?,
                    ),
                ];
                optional(
                    "kinds",
                    input.m_kinds,
                    context,
                    &mut fields,
                    read_thinkthen_request_text_list_v1,
                )?;
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_recognition_example_entity_v1(
    pointer: *const thinkthen_request_recognition_example_entity_v1,
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
                        read_thinkthen_request_unsigned_v1(input.m_end, context)?,
                    ),
                    (
                        "kind".into(),
                        read_thinkthen_request_text_v1(input.m_kind, context)?,
                    ),
                    (
                        "start".into(),
                        read_thinkthen_request_unsigned_v1(input.m_start, context)?,
                    ),
                ];
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_context_schema_v1(
    pointer: *const thinkthen_request_context_schema_v1,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok(match input.kind {
                THINKTHEN_REQUEST_CONTEXT_SCHEMA_ARM_1_V1 => {
                    read_thinkthen_request_text_v1(input.data.arm_1, context)?
                }
                THINKTHEN_REQUEST_CONTEXT_SCHEMA_ARM_2_V1 => {
                    read_thinkthen_request_json_value_map_v1(input.data.arm_2, context)?
                }
                _ => return Err(CarrierError::Invalid),
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_arguments_v1(
    pointer: *const thinkthen_request_arguments_v1,
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
                        read_thinkthen_request_input_v1(input.m_input, context)?,
                    ),
                    (
                        "question".into(),
                        read_thinkthen_request_question_selector_v1(input.m_question, context)?,
                    ),
                ];
                optional(
                    "options",
                    input.m_options,
                    context,
                    &mut fields,
                    read_thinkthen_request_call_options_v1,
                )?;
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_question_selector_v1(
    pointer: *const thinkthen_request_question_selector_v1,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok(match input.kind {
                THINKTHEN_REQUEST_QUESTION_SELECTOR_ARM_1_V1 => {
                    read_thinkthen_request_question_selector_text_v1(input.data.arm_1, context)?
                }
                THINKTHEN_REQUEST_QUESTION_SELECTOR_ARM_2_V1 => {
                    read_thinkthen_request_question_selector_definition_v1(
                        input.data.arm_2,
                        context,
                    )?
                }
                THINKTHEN_REQUEST_QUESTION_SELECTOR_ARM_3_V1 => {
                    read_thinkthen_request_question_selector_file_v1(input.data.arm_3, context)?
                }
                THINKTHEN_REQUEST_QUESTION_SELECTOR_ARM_4_V1 => {
                    read_thinkthen_request_question_selector_name_v1(input.data.arm_4, context)?
                }
                THINKTHEN_REQUEST_QUESTION_SELECTOR_ARM_5_V1 => {
                    read_thinkthen_request_question_selector_reference_v1(
                        input.data.arm_5,
                        context,
                    )?
                }
                _ => return Err(CarrierError::Invalid),
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_question_selector_reference_v1(
    pointer: *const thinkthen_request_question_selector_reference_v1,
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
                        read_thinkthen_request_text_v1(input.m_reference, context)?,
                    ),
                ];
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_question_selector_name_v1(
    pointer: *const thinkthen_request_question_selector_name_v1,
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
                        read_thinkthen_request_text_v1(input.m_name, context)?,
                    ),
                ];
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_question_selector_file_v1(
    pointer: *const thinkthen_request_question_selector_file_v1,
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
                        read_thinkthen_request_text_v1(input.m_path, context)?,
                    ),
                ];
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_question_selector_definition_v1(
    pointer: *const thinkthen_request_question_selector_definition_v1,
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
                        read_thinkthen_request_definition_v1(input.m_value, context)?,
                    ),
                ];
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_definition_v1(
    pointer: *const thinkthen_request_definition_v1,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok(match input.kind {
                THINKTHEN_REQUEST_DEFINITION_ARM_1_V1 => {
                    read_thinkthen_request_definition_authored_decide_v1(input.data.arm_1, context)?
                }
                THINKTHEN_REQUEST_DEFINITION_ARM_2_V1 => {
                    read_thinkthen_request_definition_authored_choose_v1(input.data.arm_2, context)?
                }
                THINKTHEN_REQUEST_DEFINITION_ARM_3_V1 => {
                    read_thinkthen_request_definition_authored_tag_v1(input.data.arm_3, context)?
                }
                THINKTHEN_REQUEST_DEFINITION_ARM_4_V1 => {
                    read_thinkthen_request_definition_authored_score_v1(input.data.arm_4, context)?
                }
                THINKTHEN_REQUEST_DEFINITION_ARM_5_V1 => {
                    read_thinkthen_request_definition_authored_relate_v1(input.data.arm_5, context)?
                }
                THINKTHEN_REQUEST_DEFINITION_ARM_6_V1 => {
                    read_thinkthen_request_definition_authored_find_v1(input.data.arm_6, context)?
                }
                THINKTHEN_REQUEST_DEFINITION_ARM_7_V1 => {
                    read_thinkthen_request_definition_recognize_definition_v1(
                        input.data.arm_7,
                        context,
                    )?
                }
                THINKTHEN_REQUEST_DEFINITION_ARM_8_V1 => {
                    read_thinkthen_request_definition_annotate_definition_v1(
                        input.data.arm_8,
                        context,
                    )?
                }
                _ => return Err(CarrierError::Invalid),
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_definition_annotate_definition_v1(
    pointer: *const thinkthen_request_definition_annotate_definition_v1,
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
                        "questions".into(),
                        read_thinkthen_request_annotate_question_map_v1(
                            input.m_questions,
                            context,
                        )?,
                    ),
                    ("version".into(), Value::literal("1")?),
                ];
                optional(
                    "batch",
                    input.m_batch,
                    context,
                    &mut fields,
                    read_thinkthen_request_json_value_v1,
                )?;
                optional(
                    "profile",
                    input.m_profile,
                    context,
                    &mut fields,
                    read_thinkthen_request_profile_v1,
                )?;
                optional(
                    "threshold",
                    input.m_threshold,
                    context,
                    &mut fields,
                    read_thinkthen_request_threshold_v1,
                )?;
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_threshold_v1(
    pointer: *const thinkthen_request_threshold_v1,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok(match input.kind {
                THINKTHEN_REQUEST_THRESHOLD_ARM_1_V1 => {
                    read_thinkthen_request_number_v1(input.data.arm_1, context)?
                }
                THINKTHEN_REQUEST_THRESHOLD_ARM_2_V1 => {
                    read_thinkthen_request_text_v1(input.data.arm_2, context)?
                }
                _ => return Err(CarrierError::Invalid),
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_annotate_question_v1(
    pointer: *const thinkthen_request_annotate_question_v1,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok(match input.kind {
                THINKTHEN_REQUEST_ANNOTATE_QUESTION_ARM_1_V1 => {
                    read_thinkthen_request_annotate_question_decide_v1(input.data.arm_1, context)?
                }
                THINKTHEN_REQUEST_ANNOTATE_QUESTION_ARM_2_V1 => {
                    read_thinkthen_request_annotate_question_choose_v1(input.data.arm_2, context)?
                }
                THINKTHEN_REQUEST_ANNOTATE_QUESTION_ARM_3_V1 => {
                    read_thinkthen_request_annotate_question_tag_v1(input.data.arm_3, context)?
                }
                THINKTHEN_REQUEST_ANNOTATE_QUESTION_ARM_4_V1 => {
                    read_thinkthen_request_annotate_question_score_v1(input.data.arm_4, context)?
                }
                _ => return Err(CarrierError::Invalid),
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_annotate_question_score_v1(
    pointer: *const thinkthen_request_annotate_question_score_v1,
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
                    read_thinkthen_request_question_text_v1(input.m_score, context)?,
                )];
                optional(
                    "context_schema",
                    input.m_context_schema,
                    context,
                    &mut fields,
                    read_thinkthen_request_input_declaration_v1,
                )?;
                optional(
                    "item_schema",
                    input.m_item_schema,
                    context,
                    &mut fields,
                    read_thinkthen_request_input_declaration_v1,
                )?;
                optional(
                    "levels",
                    input.m_levels,
                    context,
                    &mut fields,
                    read_thinkthen_request_levels_v1,
                )?;
                optional(
                    "name",
                    input.m_name,
                    context,
                    &mut fields,
                    read_thinkthen_request_text_v1,
                )?;
                optional(
                    "on",
                    input.m_on,
                    context,
                    &mut fields,
                    read_thinkthen_request_pointers_v1,
                )?;
                optional(
                    "wording_version",
                    input.m_wording_version,
                    context,
                    &mut fields,
                    read_thinkthen_request_integer_v1,
                )?;
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_question_text_v1(
    pointer: *const thinkthen_request_question_text_v1,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok(match input.kind {
                THINKTHEN_REQUEST_QUESTION_TEXT_ARM_1_V1 => {
                    read_thinkthen_request_text_v1(input.data.arm_1, context)?
                }
                THINKTHEN_REQUEST_QUESTION_TEXT_ARM_2_V1 => {
                    read_thinkthen_request_json_value_map_v1(input.data.arm_2, context)?
                }
                THINKTHEN_REQUEST_QUESTION_TEXT_ARM_3_V1 => {
                    read_thinkthen_request_json_value_list_v1(input.data.arm_3, context)?
                }
                _ => return Err(CarrierError::Invalid),
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_pointers_v1(
    pointer: *const thinkthen_request_pointers_v1,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok(match input.kind {
                THINKTHEN_REQUEST_POINTERS_ARM_1_V1 => {
                    read_thinkthen_request_text_v1(input.data.arm_1, context)?
                }
                THINKTHEN_REQUEST_POINTERS_ARM_2_V1 => {
                    read_thinkthen_request_text_list_v1(input.data.arm_2, context)?
                }
                _ => return Err(CarrierError::Invalid),
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_levels_v1(
    pointer: *const thinkthen_request_levels_v1,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok(match input.kind {
                THINKTHEN_REQUEST_LEVELS_ARM_1_V1 => {
                    read_thinkthen_request_name_list_v1(input.data.arm_1, context)?
                }
                THINKTHEN_REQUEST_LEVELS_ARM_2_V1 => {
                    read_thinkthen_request_criterion_map_v1(input.data.arm_2, context)?
                }
                _ => return Err(CarrierError::Invalid),
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_name_v1(
    pointer: *const thinkthen_request_name_v1,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe { Ok(Value::Text(context.text(input.text)?)) }
    })
}
pub(super) unsafe fn read_thinkthen_request_criterion_v1(
    pointer: *const thinkthen_request_criterion_v1,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok(match input.kind {
                THINKTHEN_REQUEST_CRITERION_ARM_1_V1 => {
                    read_thinkthen_request_text_v1(input.data.arm_1, context)?
                }
                THINKTHEN_REQUEST_CRITERION_ARM_2_V1 => {
                    read_thinkthen_request_json_value_map_v1(input.data.arm_2, context)?
                }
                THINKTHEN_REQUEST_CRITERION_ARM_3_V1 => {
                    read_thinkthen_request_json_value_list_v1(input.data.arm_3, context)?
                }
                THINKTHEN_REQUEST_CRITERION_ARM_4_V1 => {
                    read_thinkthen_request_null_v1(input.data.arm_4, context)?
                }
                _ => return Err(CarrierError::Invalid),
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_input_declaration_v1(
    pointer: *const thinkthen_request_input_declaration_v1,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok(match input.kind {
                THINKTHEN_REQUEST_INPUT_DECLARATION_ARM_1_V1 => {
                    read_thinkthen_request_input_declaration_string_v1(input.data.arm_1, context)?
                }
                THINKTHEN_REQUEST_INPUT_DECLARATION_ARM_2_V1 => {
                    read_thinkthen_request_input_declaration_object_v1(input.data.arm_2, context)?
                }
                _ => return Err(CarrierError::Invalid),
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_input_declaration_object_v1(
    pointer: *const thinkthen_request_input_declaration_object_v1,
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
                        read_thinkthen_request_input_property_map_v1(input.m_properties, context)?,
                    ),
                    ("type".into(), Value::literal("\"object\"")?),
                ];
                optional(
                    "required",
                    input.m_required,
                    context,
                    &mut fields,
                    read_thinkthen_request_text_list_v1,
                )?;
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_input_property_v1(
    pointer: *const thinkthen_request_input_property_v1,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok(match input.kind {
                THINKTHEN_REQUEST_INPUT_PROPERTY_ARM_1_V1 => {
                    read_thinkthen_request_input_property_string_v1(input.data.arm_1, context)?
                }
                THINKTHEN_REQUEST_INPUT_PROPERTY_ARM_2_V1 => {
                    read_thinkthen_request_input_property_number_v1(input.data.arm_2, context)?
                }
                THINKTHEN_REQUEST_INPUT_PROPERTY_ARM_3_V1 => {
                    read_thinkthen_request_input_property_boolean_v1(input.data.arm_3, context)?
                }
                THINKTHEN_REQUEST_INPUT_PROPERTY_ARM_4_V1 => {
                    read_thinkthen_request_input_property_array_v1(input.data.arm_4, context)?
                }
                _ => return Err(CarrierError::Invalid),
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_input_property_array_v1(
    pointer: *const thinkthen_request_input_property_array_v1,
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
                        read_thinkthen_request_input_property_array_items_v1(
                            input.m_items,
                            context,
                        )?,
                    ),
                    ("type".into(), Value::literal("\"array\"")?),
                ];
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_input_property_boolean_v1(
    pointer: *const thinkthen_request_input_property_boolean_v1,
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
pub(super) unsafe fn read_thinkthen_request_input_property_number_v1(
    pointer: *const thinkthen_request_input_property_number_v1,
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
pub(super) unsafe fn read_thinkthen_request_input_property_string_v1(
    pointer: *const thinkthen_request_input_property_string_v1,
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
pub(super) unsafe fn read_thinkthen_request_input_declaration_string_v1(
    pointer: *const thinkthen_request_input_declaration_string_v1,
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
pub(super) unsafe fn read_thinkthen_request_annotate_question_tag_v1(
    pointer: *const thinkthen_request_annotate_question_tag_v1,
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
                    read_thinkthen_request_question_text_v1(input.m_tag, context)?,
                )];
                optional(
                    "context_schema",
                    input.m_context_schema,
                    context,
                    &mut fields,
                    read_thinkthen_request_input_declaration_v1,
                )?;
                optional(
                    "item_schema",
                    input.m_item_schema,
                    context,
                    &mut fields,
                    read_thinkthen_request_input_declaration_v1,
                )?;
                optional(
                    "labels",
                    input.m_labels,
                    context,
                    &mut fields,
                    read_thinkthen_request_labels_v1,
                )?;
                optional(
                    "name",
                    input.m_name,
                    context,
                    &mut fields,
                    read_thinkthen_request_text_v1,
                )?;
                optional(
                    "on",
                    input.m_on,
                    context,
                    &mut fields,
                    read_thinkthen_request_pointers_v1,
                )?;
                optional(
                    "threshold",
                    input.m_threshold,
                    context,
                    &mut fields,
                    read_thinkthen_request_cut_v1,
                )?;
                optional(
                    "wording_version",
                    input.m_wording_version,
                    context,
                    &mut fields,
                    read_thinkthen_request_integer_v1,
                )?;
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_cut_v1(
    pointer: *const thinkthen_request_cut_v1,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok(match input.kind {
                THINKTHEN_REQUEST_CUT_ARM_1_V1 => {
                    read_thinkthen_request_number_v1(input.data.arm_1, context)?
                }
                THINKTHEN_REQUEST_CUT_ARM_2_V1 => {
                    read_thinkthen_request_text_v1(input.data.arm_2, context)?
                }
                _ => return Err(CarrierError::Invalid),
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_labels_v1(
    pointer: *const thinkthen_request_labels_v1,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok(match input.kind {
                THINKTHEN_REQUEST_LABELS_ARM_1_V1 => {
                    read_thinkthen_request_name_list_v1(input.data.arm_1, context)?
                }
                THINKTHEN_REQUEST_LABELS_ARM_2_V1 => {
                    read_thinkthen_request_description_map_v1(input.data.arm_2, context)?
                }
                _ => return Err(CarrierError::Invalid),
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_description_v1(
    pointer: *const thinkthen_request_description_v1,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok(match input.kind {
                THINKTHEN_REQUEST_DESCRIPTION_ARM_1_V1 => {
                    read_thinkthen_request_text_v1(input.data.arm_1, context)?
                }
                THINKTHEN_REQUEST_DESCRIPTION_ARM_2_V1 => {
                    read_thinkthen_request_json_value_map_v1(input.data.arm_2, context)?
                }
                THINKTHEN_REQUEST_DESCRIPTION_ARM_3_V1 => {
                    read_thinkthen_request_json_value_list_v1(input.data.arm_3, context)?
                }
                THINKTHEN_REQUEST_DESCRIPTION_ARM_4_V1 => {
                    read_thinkthen_request_null_v1(input.data.arm_4, context)?
                }
                _ => return Err(CarrierError::Invalid),
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_annotate_question_choose_v1(
    pointer: *const thinkthen_request_annotate_question_choose_v1,
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
                    read_thinkthen_request_question_text_v1(input.m_choose, context)?,
                )];
                optional(
                    "context_schema",
                    input.m_context_schema,
                    context,
                    &mut fields,
                    read_thinkthen_request_input_declaration_v1,
                )?;
                optional(
                    "item_schema",
                    input.m_item_schema,
                    context,
                    &mut fields,
                    read_thinkthen_request_input_declaration_v1,
                )?;
                optional(
                    "name",
                    input.m_name,
                    context,
                    &mut fields,
                    read_thinkthen_request_text_v1,
                )?;
                optional(
                    "on",
                    input.m_on,
                    context,
                    &mut fields,
                    read_thinkthen_request_pointers_v1,
                )?;
                optional(
                    "options",
                    input.m_options,
                    context,
                    &mut fields,
                    read_thinkthen_request_options_v1,
                )?;
                optional(
                    "threshold",
                    input.m_threshold,
                    context,
                    &mut fields,
                    read_thinkthen_request_cut_v1,
                )?;
                optional(
                    "wording_version",
                    input.m_wording_version,
                    context,
                    &mut fields,
                    read_thinkthen_request_integer_v1,
                )?;
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_options_v1(
    pointer: *const thinkthen_request_options_v1,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok(match input.kind {
                THINKTHEN_REQUEST_OPTIONS_ARM_1_V1 => {
                    read_thinkthen_request_name_list_v1(input.data.arm_1, context)?
                }
                THINKTHEN_REQUEST_OPTIONS_ARM_2_V1 => {
                    read_thinkthen_request_description_map_v1(input.data.arm_2, context)?
                }
                _ => return Err(CarrierError::Invalid),
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_annotate_question_decide_v1(
    pointer: *const thinkthen_request_annotate_question_decide_v1,
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
                    read_thinkthen_request_question_text_v1(input.m_decide, context)?,
                )];
                optional(
                    "context_schema",
                    input.m_context_schema,
                    context,
                    &mut fields,
                    read_thinkthen_request_input_declaration_v1,
                )?;
                optional(
                    "false",
                    input.m_false,
                    context,
                    &mut fields,
                    read_thinkthen_request_criterion_v1,
                )?;
                optional(
                    "item_schema",
                    input.m_item_schema,
                    context,
                    &mut fields,
                    read_thinkthen_request_input_declaration_v1,
                )?;
                optional(
                    "name",
                    input.m_name,
                    context,
                    &mut fields,
                    read_thinkthen_request_text_v1,
                )?;
                optional(
                    "on",
                    input.m_on,
                    context,
                    &mut fields,
                    read_thinkthen_request_pointers_v1,
                )?;
                optional(
                    "threshold",
                    input.m_threshold,
                    context,
                    &mut fields,
                    read_thinkthen_request_threshold_v1,
                )?;
                optional(
                    "true",
                    input.m_true,
                    context,
                    &mut fields,
                    read_thinkthen_request_criterion_v1,
                )?;
                optional(
                    "wording_version",
                    input.m_wording_version,
                    context,
                    &mut fields,
                    read_thinkthen_request_integer_v1,
                )?;
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_profile_v1(
    pointer: *const thinkthen_request_profile_v1,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe { Ok(Value::Text(context.text(input.text)?)) }
    })
}
pub(super) unsafe fn read_thinkthen_request_definition_recognize_definition_v1(
    pointer: *const thinkthen_request_definition_recognize_definition_v1,
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
                        "recognize".into(),
                        read_thinkthen_request_recognize_question_v1(input.m_recognize, context)?,
                    ),
                    ("version".into(), Value::literal("1")?),
                ];
                optional(
                    "context_schema",
                    input.m_context_schema,
                    context,
                    &mut fields,
                    read_thinkthen_request_input_declaration_v1,
                )?;
                optional(
                    "item_schema",
                    input.m_item_schema,
                    context,
                    &mut fields,
                    read_thinkthen_request_input_declaration_v1,
                )?;
                optional(
                    "model",
                    input.m_model,
                    context,
                    &mut fields,
                    read_thinkthen_request_name_v1,
                )?;
                optional(
                    "name",
                    input.m_name,
                    context,
                    &mut fields,
                    read_thinkthen_request_text_v1,
                )?;
                optional(
                    "on",
                    input.m_on,
                    context,
                    &mut fields,
                    read_thinkthen_request_pointers_v1,
                )?;
                optional(
                    "profile",
                    input.m_profile,
                    context,
                    &mut fields,
                    read_thinkthen_request_profile_v1,
                )?;
                optional(
                    "relation_threshold",
                    input.m_relation_threshold,
                    context,
                    &mut fields,
                    read_thinkthen_request_cut_v1,
                )?;
                optional(
                    "threshold",
                    input.m_threshold,
                    context,
                    &mut fields,
                    read_thinkthen_request_cut_v1,
                )?;
                optional(
                    "wording_version",
                    input.m_wording_version,
                    context,
                    &mut fields,
                    read_thinkthen_request_integer_v1,
                )?;
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_recognize_question_v1(
    pointer: *const thinkthen_request_recognize_question_v1,
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
                    read_thinkthen_request_question_text_v1,
                )?;
                optional(
                    "instructions",
                    input.m_instructions,
                    context,
                    &mut fields,
                    read_thinkthen_request_question_text_v1,
                )?;
                optional(
                    "kinds",
                    input.m_kinds,
                    context,
                    &mut fields,
                    read_thinkthen_request_description_map_v1,
                )?;
                optional(
                    "mode",
                    input.m_mode,
                    context,
                    &mut fields,
                    read_thinkthen_request_recognition_mode_v1,
                )?;
                optional(
                    "relations",
                    input.m_relations,
                    context,
                    &mut fields,
                    read_thinkthen_request_relation_list_v1,
                )?;
                optional(
                    "snippet_pieces",
                    input.m_snippet_pieces,
                    context,
                    &mut fields,
                    read_thinkthen_request_unsigned_v1,
                )?;
                optional(
                    "stage_context",
                    input.m_stage_context,
                    context,
                    &mut fields,
                    read_thinkthen_request_recognition_stage_context_v1,
                )?;
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_recognition_stage_context_v1(
    pointer: *const thinkthen_request_recognition_stage_context_v1,
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
                    read_thinkthen_request_text_v1,
                )?;
                optional(
                    "kind_edge",
                    input.m_kind_edge,
                    context,
                    &mut fields,
                    read_thinkthen_request_text_v1,
                )?;
                optional(
                    "relation",
                    input.m_relation,
                    context,
                    &mut fields,
                    read_thinkthen_request_text_v1,
                )?;
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_relation_v1(
    pointer: *const thinkthen_request_relation_v1,
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
                    read_thinkthen_request_name_v1(input.m_name, context)?,
                )];
                optional(
                    "either",
                    input.m_either,
                    context,
                    &mut fields,
                    read_thinkthen_request_boolean_v1,
                )?;
                optional(
                    "reads",
                    input.m_reads,
                    context,
                    &mut fields,
                    read_thinkthen_request_name_v1,
                )?;
                optional(
                    "single",
                    input.m_single,
                    context,
                    &mut fields,
                    read_thinkthen_request_boolean_v1,
                )?;
                optional(
                    "source",
                    input.m_source,
                    context,
                    &mut fields,
                    read_thinkthen_request_name_v1,
                )?;
                optional(
                    "target",
                    input.m_target,
                    context,
                    &mut fields,
                    read_thinkthen_request_name_v1,
                )?;
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_recognition_mode_v1(
    pointer: *const thinkthen_request_recognition_mode_v1,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok(match input.kind {
                THINKTHEN_REQUEST_RECOGNITION_MODE_ARM_1_V1 => {
                    read_thinkthen_request_literal_whole_v1(input.data.arm_1, context)?
                }
                THINKTHEN_REQUEST_RECOGNITION_MODE_ARM_2_V1 => {
                    read_thinkthen_request_literal_boundary_only_v1(input.data.arm_2, context)?
                }
                _ => return Err(CarrierError::Invalid),
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_definition_authored_find_v1(
    pointer: *const thinkthen_request_definition_authored_find_v1,
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
                    read_thinkthen_request_question_text_v1(input.m_find, context)?,
                )];
                optional(
                    "context_schema",
                    input.m_context_schema,
                    context,
                    &mut fields,
                    read_thinkthen_request_input_declaration_v1,
                )?;
                optional(
                    "item_schema",
                    input.m_item_schema,
                    context,
                    &mut fields,
                    read_thinkthen_request_input_declaration_v1,
                )?;
                optional(
                    "model",
                    input.m_model,
                    context,
                    &mut fields,
                    read_thinkthen_request_name_v1,
                )?;
                optional(
                    "name",
                    input.m_name,
                    context,
                    &mut fields,
                    read_thinkthen_request_text_v1,
                )?;
                optional(
                    "on",
                    input.m_on,
                    context,
                    &mut fields,
                    read_thinkthen_request_pointers_v1,
                )?;
                optional(
                    "profile",
                    input.m_profile,
                    context,
                    &mut fields,
                    read_thinkthen_request_profile_v1,
                )?;
                optional(
                    "wording_version",
                    input.m_wording_version,
                    context,
                    &mut fields,
                    read_thinkthen_request_integer_v1,
                )?;
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_definition_authored_relate_v1(
    pointer: *const thinkthen_request_definition_authored_relate_v1,
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
                        "relate".into(),
                        read_thinkthen_request_relate_question_v1(input.m_relate, context)?,
                    ),
                    ("version".into(), Value::literal("1")?),
                ];
                optional(
                    "context_schema",
                    input.m_context_schema,
                    context,
                    &mut fields,
                    read_thinkthen_request_input_declaration_v1,
                )?;
                optional(
                    "item_schema",
                    input.m_item_schema,
                    context,
                    &mut fields,
                    read_thinkthen_request_input_declaration_v1,
                )?;
                optional(
                    "model",
                    input.m_model,
                    context,
                    &mut fields,
                    read_thinkthen_request_name_v1,
                )?;
                optional(
                    "name",
                    input.m_name,
                    context,
                    &mut fields,
                    read_thinkthen_request_text_v1,
                )?;
                optional(
                    "profile",
                    input.m_profile,
                    context,
                    &mut fields,
                    read_thinkthen_request_profile_v1,
                )?;
                optional(
                    "threshold",
                    input.m_threshold,
                    context,
                    &mut fields,
                    read_thinkthen_request_cut_v1,
                )?;
                optional(
                    "wording_version",
                    input.m_wording_version,
                    context,
                    &mut fields,
                    read_thinkthen_request_integer_v1,
                )?;
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_relate_question_v1(
    pointer: *const thinkthen_request_relate_question_v1,
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
                    read_thinkthen_request_relation_list_v1(input.m_relations, context)?,
                )];
                optional(
                    "fields",
                    input.m_fields,
                    context,
                    &mut fields,
                    read_thinkthen_request_relate_fields_v1,
                )?;
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_relate_fields_v1(
    pointer: *const thinkthen_request_relate_fields_v1,
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
                        read_thinkthen_request_text_v1(input.m_kind, context)?,
                    ),
                    (
                        "name".into(),
                        read_thinkthen_request_text_v1(input.m_name, context)?,
                    ),
                ];
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_definition_authored_score_v1(
    pointer: *const thinkthen_request_definition_authored_score_v1,
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
                    read_thinkthen_request_question_text_v1(input.m_score, context)?,
                )];
                optional(
                    "batch",
                    input.m_batch,
                    context,
                    &mut fields,
                    read_thinkthen_request_definition_authored_score_batch_v1,
                )?;
                optional(
                    "context_schema",
                    input.m_context_schema,
                    context,
                    &mut fields,
                    read_thinkthen_request_input_declaration_v1,
                )?;
                optional(
                    "item_schema",
                    input.m_item_schema,
                    context,
                    &mut fields,
                    read_thinkthen_request_input_declaration_v1,
                )?;
                optional(
                    "levels",
                    input.m_levels,
                    context,
                    &mut fields,
                    read_thinkthen_request_levels_v1,
                )?;
                optional(
                    "model",
                    input.m_model,
                    context,
                    &mut fields,
                    read_thinkthen_request_name_v1,
                )?;
                optional(
                    "name",
                    input.m_name,
                    context,
                    &mut fields,
                    read_thinkthen_request_text_v1,
                )?;
                optional(
                    "on",
                    input.m_on,
                    context,
                    &mut fields,
                    read_thinkthen_request_pointers_v1,
                )?;
                optional(
                    "profile",
                    input.m_profile,
                    context,
                    &mut fields,
                    read_thinkthen_request_profile_v1,
                )?;
                optional(
                    "wording_version",
                    input.m_wording_version,
                    context,
                    &mut fields,
                    read_thinkthen_request_integer_v1,
                )?;
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_definition_authored_tag_v1(
    pointer: *const thinkthen_request_definition_authored_tag_v1,
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
                    read_thinkthen_request_question_text_v1(input.m_tag, context)?,
                )];
                optional(
                    "batch",
                    input.m_batch,
                    context,
                    &mut fields,
                    read_thinkthen_request_definition_authored_score_batch_v1,
                )?;
                optional(
                    "context_schema",
                    input.m_context_schema,
                    context,
                    &mut fields,
                    read_thinkthen_request_input_declaration_v1,
                )?;
                optional(
                    "item_schema",
                    input.m_item_schema,
                    context,
                    &mut fields,
                    read_thinkthen_request_input_declaration_v1,
                )?;
                optional(
                    "labels",
                    input.m_labels,
                    context,
                    &mut fields,
                    read_thinkthen_request_labels_v1,
                )?;
                optional(
                    "model",
                    input.m_model,
                    context,
                    &mut fields,
                    read_thinkthen_request_name_v1,
                )?;
                optional(
                    "name",
                    input.m_name,
                    context,
                    &mut fields,
                    read_thinkthen_request_text_v1,
                )?;
                optional(
                    "on",
                    input.m_on,
                    context,
                    &mut fields,
                    read_thinkthen_request_pointers_v1,
                )?;
                optional(
                    "profile",
                    input.m_profile,
                    context,
                    &mut fields,
                    read_thinkthen_request_profile_v1,
                )?;
                optional(
                    "threshold",
                    input.m_threshold,
                    context,
                    &mut fields,
                    read_thinkthen_request_cut_v1,
                )?;
                optional(
                    "wording_version",
                    input.m_wording_version,
                    context,
                    &mut fields,
                    read_thinkthen_request_integer_v1,
                )?;
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_definition_authored_choose_v1(
    pointer: *const thinkthen_request_definition_authored_choose_v1,
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
                    read_thinkthen_request_question_text_v1(input.m_choose, context)?,
                )];
                optional(
                    "batch",
                    input.m_batch,
                    context,
                    &mut fields,
                    read_thinkthen_request_definition_authored_score_batch_v1,
                )?;
                optional(
                    "context_schema",
                    input.m_context_schema,
                    context,
                    &mut fields,
                    read_thinkthen_request_input_declaration_v1,
                )?;
                optional(
                    "item_schema",
                    input.m_item_schema,
                    context,
                    &mut fields,
                    read_thinkthen_request_input_declaration_v1,
                )?;
                optional(
                    "model",
                    input.m_model,
                    context,
                    &mut fields,
                    read_thinkthen_request_name_v1,
                )?;
                optional(
                    "name",
                    input.m_name,
                    context,
                    &mut fields,
                    read_thinkthen_request_text_v1,
                )?;
                optional(
                    "on",
                    input.m_on,
                    context,
                    &mut fields,
                    read_thinkthen_request_pointers_v1,
                )?;
                optional(
                    "options",
                    input.m_options,
                    context,
                    &mut fields,
                    read_thinkthen_request_options_v1,
                )?;
                optional(
                    "profile",
                    input.m_profile,
                    context,
                    &mut fields,
                    read_thinkthen_request_profile_v1,
                )?;
                optional(
                    "threshold",
                    input.m_threshold,
                    context,
                    &mut fields,
                    read_thinkthen_request_cut_v1,
                )?;
                optional(
                    "wording_version",
                    input.m_wording_version,
                    context,
                    &mut fields,
                    read_thinkthen_request_integer_v1,
                )?;
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_definition_authored_decide_v1(
    pointer: *const thinkthen_request_definition_authored_decide_v1,
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
                    read_thinkthen_request_question_text_v1(input.m_decide, context)?,
                )];
                optional(
                    "batch",
                    input.m_batch,
                    context,
                    &mut fields,
                    read_thinkthen_request_definition_authored_score_batch_v1,
                )?;
                optional(
                    "context_schema",
                    input.m_context_schema,
                    context,
                    &mut fields,
                    read_thinkthen_request_input_declaration_v1,
                )?;
                optional(
                    "false",
                    input.m_false,
                    context,
                    &mut fields,
                    read_thinkthen_request_criterion_v1,
                )?;
                optional(
                    "item_schema",
                    input.m_item_schema,
                    context,
                    &mut fields,
                    read_thinkthen_request_input_declaration_v1,
                )?;
                optional(
                    "model",
                    input.m_model,
                    context,
                    &mut fields,
                    read_thinkthen_request_name_v1,
                )?;
                optional(
                    "name",
                    input.m_name,
                    context,
                    &mut fields,
                    read_thinkthen_request_text_v1,
                )?;
                optional(
                    "on",
                    input.m_on,
                    context,
                    &mut fields,
                    read_thinkthen_request_pointers_v1,
                )?;
                optional(
                    "profile",
                    input.m_profile,
                    context,
                    &mut fields,
                    read_thinkthen_request_profile_v1,
                )?;
                optional(
                    "threshold",
                    input.m_threshold,
                    context,
                    &mut fields,
                    read_thinkthen_request_threshold_v1,
                )?;
                optional(
                    "true",
                    input.m_true,
                    context,
                    &mut fields,
                    read_thinkthen_request_criterion_v1,
                )?;
                optional(
                    "wording_version",
                    input.m_wording_version,
                    context,
                    &mut fields,
                    read_thinkthen_request_integer_v1,
                )?;
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_question_selector_text_v1(
    pointer: *const thinkthen_request_question_selector_text_v1,
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
                        read_thinkthen_request_text_v1(input.m_text, context)?,
                    ),
                ];
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_call_options_v1(
    pointer: *const thinkthen_request_call_options_v1,
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
                    read_thinkthen_request_boolean_v1,
                )?;
                optional(
                    "batch",
                    input.m_batch,
                    context,
                    &mut fields,
                    read_thinkthen_request_batch_v1,
                )?;
                optional(
                    "context",
                    input.m_context,
                    context,
                    &mut fields,
                    read_thinkthen_request_text_v1,
                )?;
                optional(
                    "context_field",
                    input.m_context_field,
                    context,
                    &mut fields,
                    read_thinkthen_request_text_v1,
                )?;
                optional(
                    "deadline_ms",
                    input.m_deadline_ms,
                    context,
                    &mut fields,
                    read_thinkthen_request_integer_v1,
                )?;
                optional(
                    "details",
                    input.m_details,
                    context,
                    &mut fields,
                    read_thinkthen_request_boolean_v1,
                )?;
                optional(
                    "examples",
                    input.m_examples,
                    context,
                    &mut fields,
                    read_thinkthen_request_recognition_example_list_v1,
                )?;
                optional(
                    "examples_field",
                    input.m_examples_field,
                    context,
                    &mut fields,
                    read_thinkthen_request_text_v1,
                )?;
                optional(
                    "field",
                    input.m_field,
                    context,
                    &mut fields,
                    read_thinkthen_request_text_list_v1,
                )?;
                optional(
                    "files_only",
                    input.m_files_only,
                    context,
                    &mut fields,
                    read_thinkthen_request_boolean_v1,
                )?;
                optional(
                    "max_requests_total",
                    input.m_max_requests_total,
                    context,
                    &mut fields,
                    read_thinkthen_request_unsigned_v1,
                )?;
                optional(
                    "mode",
                    input.m_mode,
                    context,
                    &mut fields,
                    read_thinkthen_request_recognition_mode_v1,
                )?;
                optional(
                    "model",
                    input.m_model,
                    context,
                    &mut fields,
                    read_thinkthen_request_text_v1,
                )?;
                optional(
                    "none",
                    input.m_none,
                    context,
                    &mut fields,
                    read_thinkthen_request_boolean_v1,
                )?;
                optional(
                    "options_field",
                    input.m_options_field,
                    context,
                    &mut fields,
                    read_thinkthen_request_text_v1,
                )?;
                optional(
                    "relation_threshold",
                    input.m_relation_threshold,
                    context,
                    &mut fields,
                    read_thinkthen_request_call_threshold_v1,
                )?;
                optional(
                    "seed_spans",
                    input.m_seed_spans,
                    context,
                    &mut fields,
                    read_thinkthen_request_recognition_seed_span_list_v1,
                )?;
                optional(
                    "seed_spans_field",
                    input.m_seed_spans_field,
                    context,
                    &mut fields,
                    read_thinkthen_request_text_v1,
                )?;
                optional(
                    "snippet_pieces",
                    input.m_snippet_pieces,
                    context,
                    &mut fields,
                    read_thinkthen_request_unsigned_v1,
                )?;
                optional(
                    "stage_context",
                    input.m_stage_context,
                    context,
                    &mut fields,
                    read_thinkthen_request_recognition_stage_context_v1,
                )?;
                optional(
                    "threshold",
                    input.m_threshold,
                    context,
                    &mut fields,
                    read_thinkthen_request_call_threshold_v1,
                )?;
                optional(
                    "top",
                    input.m_top,
                    context,
                    &mut fields,
                    read_thinkthen_request_unsigned_v1,
                )?;
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_call_threshold_v1(
    pointer: *const thinkthen_request_call_threshold_v1,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok(match input.kind {
                THINKTHEN_REQUEST_CALL_THRESHOLD_ARM_1_V1 => {
                    read_thinkthen_request_number_v1(input.data.arm_1, context)?
                }
                THINKTHEN_REQUEST_CALL_THRESHOLD_ARM_2_V1 => {
                    read_thinkthen_request_text_v1(input.data.arm_2, context)?
                }
                _ => return Err(CarrierError::Invalid),
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_batch_v1(
    pointer: *const thinkthen_request_batch_v1,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok(match input.kind {
                THINKTHEN_REQUEST_BATCH_ARM_1_V1 => {
                    read_thinkthen_request_unsigned_v1(input.data.arm_1, context)?
                }
                THINKTHEN_REQUEST_BATCH_ARM_2_V1 => {
                    read_thinkthen_request_text_v1(input.data.arm_2, context)?
                }
                _ => return Err(CarrierError::Invalid),
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_input_v1(
    pointer: *const thinkthen_request_input_v1,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok(match input.kind {
                THINKTHEN_REQUEST_INPUT_ARM_1_V1 => {
                    read_thinkthen_request_input_text_v1(input.data.arm_1, context)?
                }
                THINKTHEN_REQUEST_INPUT_ARM_2_V1 => {
                    read_thinkthen_request_input_json_v1(input.data.arm_2, context)?
                }
                THINKTHEN_REQUEST_INPUT_ARM_3_V1 => {
                    read_thinkthen_request_input_records_v1(input.data.arm_3, context)?
                }
                THINKTHEN_REQUEST_INPUT_ARM_4_V1 => {
                    read_thinkthen_request_input_units_v1(input.data.arm_4, context)?
                }
                THINKTHEN_REQUEST_INPUT_ARM_5_V1 => {
                    read_thinkthen_request_input_entities_v1(input.data.arm_5, context)?
                }
                THINKTHEN_REQUEST_INPUT_ARM_6_V1 => {
                    read_thinkthen_request_input_source_v1(input.data.arm_6, context)?
                }
                THINKTHEN_REQUEST_INPUT_ARM_7_V1 => {
                    read_thinkthen_request_input_feed_v1(input.data.arm_7, context)?
                }
                _ => return Err(CarrierError::Invalid),
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_input_feed_v1(
    pointer: *const thinkthen_request_input_feed_v1,
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
                        read_thinkthen_request_text_v1(input.m_name, context)?,
                    ),
                ];
                optional(
                    "framing",
                    input.m_framing,
                    context,
                    &mut fields,
                    read_thinkthen_request_framing_v1,
                )?;
                optional(
                    "images",
                    input.m_images,
                    context,
                    &mut fields,
                    read_thinkthen_request_image_list_v1,
                )?;
                optional(
                    "reading",
                    input.m_reading,
                    context,
                    &mut fields,
                    read_thinkthen_request_reader_v1,
                )?;
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_reader_v1(
    pointer: *const thinkthen_request_reader_v1,
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
                    read_thinkthen_request_source_unit_v1,
                )?;
                optional(
                    "window",
                    input.m_window,
                    context,
                    &mut fields,
                    read_thinkthen_request_unsigned_v1,
                )?;
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_source_unit_v1(
    pointer: *const thinkthen_request_source_unit_v1,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok(match input.kind {
                THINKTHEN_REQUEST_SOURCE_UNIT_ARM_1_V1 => {
                    read_thinkthen_request_literal_line_v1(input.data.arm_1, context)?
                }
                THINKTHEN_REQUEST_SOURCE_UNIT_ARM_2_V1 => {
                    read_thinkthen_request_literal_window_v1(input.data.arm_2, context)?
                }
                THINKTHEN_REQUEST_SOURCE_UNIT_ARM_3_V1 => {
                    read_thinkthen_request_literal_file_v1(input.data.arm_3, context)?
                }
                _ => return Err(CarrierError::Invalid),
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_framing_v1(
    pointer: *const thinkthen_request_framing_v1,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok(match input.kind {
                THINKTHEN_REQUEST_FRAMING_ARM_1_V1 => {
                    read_thinkthen_request_literal_document_v1(input.data.arm_1, context)?
                }
                THINKTHEN_REQUEST_FRAMING_ARM_2_V1 => {
                    read_thinkthen_request_literal_lines_v1(input.data.arm_2, context)?
                }
                THINKTHEN_REQUEST_FRAMING_ARM_3_V1 => {
                    read_thinkthen_request_literal_jsonl_v1(input.data.arm_3, context)?
                }
                THINKTHEN_REQUEST_FRAMING_ARM_4_V1 => {
                    read_thinkthen_request_literal_csv_v1(input.data.arm_4, context)?
                }
                THINKTHEN_REQUEST_FRAMING_ARM_5_V1 => {
                    read_thinkthen_request_literal_tsv_v1(input.data.arm_5, context)?
                }
                _ => return Err(CarrierError::Invalid),
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_input_source_v1(
    pointer: *const thinkthen_request_input_source_v1,
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
                        read_thinkthen_request_source_v1(input.m_source, context)?,
                    ),
                ];
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_source_v1(
    pointer: *const thinkthen_request_source_v1,
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
                    read_thinkthen_request_text_list_v1(input.m_paths, context)?,
                )];
                optional(
                    "framing",
                    input.m_framing,
                    context,
                    &mut fields,
                    read_thinkthen_request_framing_v1,
                )?;
                optional(
                    "media",
                    input.m_media,
                    context,
                    &mut fields,
                    read_thinkthen_request_reader_media_v1,
                )?;
                optional(
                    "reading",
                    input.m_reading,
                    context,
                    &mut fields,
                    read_thinkthen_request_reader_v1,
                )?;
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_reader_media_v1(
    pointer: *const thinkthen_request_reader_media_v1,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok(match input.kind {
                THINKTHEN_REQUEST_READER_MEDIA_ARM_1_V1 => {
                    read_thinkthen_request_literal_text_v1(input.data.arm_1, context)?
                }
                THINKTHEN_REQUEST_READER_MEDIA_ARM_2_V1 => {
                    read_thinkthen_request_literal_image_v1(input.data.arm_2, context)?
                }
                _ => return Err(CarrierError::Invalid),
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_input_entities_v1(
    pointer: *const thinkthen_request_input_entities_v1,
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
                        read_thinkthen_request_item_list_v1(input.m_items, context)?,
                    ),
                    ("kind".into(), Value::literal("\"entities\"")?),
                ];
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_input_units_v1(
    pointer: *const thinkthen_request_input_units_v1,
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
                        read_thinkthen_request_item_list_v1(input.m_items, context)?,
                    ),
                    ("kind".into(), Value::literal("\"units\"")?),
                ];
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_input_records_v1(
    pointer: *const thinkthen_request_input_records_v1,
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
                        read_thinkthen_request_item_list_v1(input.m_items, context)?,
                    ),
                    ("kind".into(), Value::literal("\"records\"")?),
                ];
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_input_json_v1(
    pointer: *const thinkthen_request_input_json_v1,
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
                        read_thinkthen_request_input_json_value_v1(input.m_value, context)?,
                    ),
                ];
                optional(
                    "images",
                    input.m_images,
                    context,
                    &mut fields,
                    read_thinkthen_request_image_list_v1,
                )?;
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_input_text_v1(
    pointer: *const thinkthen_request_input_text_v1,
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
                        read_thinkthen_request_text_v1(input.m_text, context)?,
                    ),
                ];
                optional(
                    "images",
                    input.m_images,
                    context,
                    &mut fields,
                    read_thinkthen_request_image_list_v1,
                )?;
                Value::Object(fields)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_text_v1(
    pointer: *const thinkthen_request_text_v1,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe { Ok(Value::Text(context.text(input.text)?)) }
    })
}
pub(super) unsafe fn read_thinkthen_request_unsigned_v1(
    pointer: *const thinkthen_request_unsigned_v1,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|_context| Ok(Value::Number(input.value.into())))
}
pub(super) unsafe fn read_thinkthen_request_recognition_example_list_v1(
    pointer: *const thinkthen_request_recognition_example_list_v1,
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
                    values.push(read_thinkthen_request_recognition_example_v1(
                        *item, context,
                    )?);
                }
                Value::Array(values)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_image_list_v1(
    pointer: *const thinkthen_request_image_list_v1,
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
                    values.push(read_thinkthen_request_image_v1(*item, context)?);
                }
                Value::Array(values)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_option_schema_list_v1(
    pointer: *const thinkthen_request_option_schema_list_v1,
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
                    values.push(read_thinkthen_request_option_schema_v1(*item, context)?);
                }
                Value::Array(values)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_recognition_seed_span_list_v1(
    pointer: *const thinkthen_request_recognition_seed_span_list_v1,
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
                    values.push(read_thinkthen_request_recognition_seed_span_v1(
                        *item, context,
                    )?);
                }
                Value::Array(values)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_original_json_value_v1(
    pointer: *const thinkthen_request_original_json_value_v1,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe { Value::literal(&context.text(input.text)?) }
    })
}
pub(super) unsafe fn read_thinkthen_request_json_value_v1(
    pointer: *const thinkthen_request_json_value_v1,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe { Value::literal(&context.text(input.text)?) }
    })
}
pub(super) unsafe fn read_thinkthen_request_literal_image_jpeg_v1(
    pointer: *const thinkthen_request_literal_image_jpeg_v1,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let _ = unsafe { borrow(pointer)? };
    let _ = context;
    Value::literal("\"image/jpeg\"")
}
pub(super) unsafe fn read_thinkthen_request_literal_image_png_v1(
    pointer: *const thinkthen_request_literal_image_png_v1,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let _ = unsafe { borrow(pointer)? };
    let _ = context;
    Value::literal("\"image/png\"")
}
pub(super) unsafe fn read_thinkthen_request_recognition_example_entity_list_v1(
    pointer: *const thinkthen_request_recognition_example_entity_list_v1,
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
                    values.push(read_thinkthen_request_recognition_example_entity_v1(
                        *item, context,
                    )?);
                }
                Value::Array(values)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_text_list_v1(
    pointer: *const thinkthen_request_text_list_v1,
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
                    values.push(read_thinkthen_request_text_v1(*item, context)?);
                }
                Value::Array(values)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_json_value_map_v1(
    pointer: *const thinkthen_request_json_value_map_v1,
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
                        read_thinkthen_request_json_value_v1(item.value, context)?,
                    ));
                }
                Value::Object(values)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_annotate_question_map_v1(
    pointer: *const thinkthen_request_annotate_question_map_v1,
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
                        read_thinkthen_request_annotate_question_v1(item.value, context)?,
                    ));
                }
                Value::Object(values)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_number_v1(
    pointer: *const thinkthen_request_number_v1,
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
pub(super) unsafe fn read_thinkthen_request_integer_v1(
    pointer: *const thinkthen_request_integer_v1,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|_context| Ok(Value::Number(input.value.into())))
}
pub(super) unsafe fn read_thinkthen_request_json_value_list_v1(
    pointer: *const thinkthen_request_json_value_list_v1,
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
                    values.push(read_thinkthen_request_json_value_v1(*item, context)?);
                }
                Value::Array(values)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_name_list_v1(
    pointer: *const thinkthen_request_name_list_v1,
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
                    values.push(read_thinkthen_request_name_v1(*item, context)?);
                }
                Value::Array(values)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_criterion_map_v1(
    pointer: *const thinkthen_request_criterion_map_v1,
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
                        read_thinkthen_request_criterion_v1(item.value, context)?,
                    ));
                }
                Value::Object(values)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_null_v1(
    pointer: *const thinkthen_request_null_v1,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let _ = unsafe { borrow(pointer)? };
    let _ = context;
    Value::literal("null")
}
pub(super) unsafe fn read_thinkthen_request_input_property_map_v1(
    pointer: *const thinkthen_request_input_property_map_v1,
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
                        read_thinkthen_request_input_property_v1(item.value, context)?,
                    ));
                }
                Value::Object(values)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_input_property_array_items_v1(
    pointer: *const thinkthen_request_input_property_array_items_v1,
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
pub(super) unsafe fn read_thinkthen_request_description_map_v1(
    pointer: *const thinkthen_request_description_map_v1,
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
                        read_thinkthen_request_description_v1(item.value, context)?,
                    ));
                }
                Value::Object(values)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_relation_list_v1(
    pointer: *const thinkthen_request_relation_list_v1,
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
                    values.push(read_thinkthen_request_relation_v1(*item, context)?);
                }
                Value::Array(values)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_boolean_v1(
    pointer: *const thinkthen_request_boolean_v1,
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
pub(super) unsafe fn read_thinkthen_request_literal_whole_v1(
    pointer: *const thinkthen_request_literal_whole_v1,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let _ = unsafe { borrow(pointer)? };
    let _ = context;
    Value::literal("\"whole\"")
}
pub(super) unsafe fn read_thinkthen_request_literal_boundary_only_v1(
    pointer: *const thinkthen_request_literal_boundary_only_v1,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let _ = unsafe { borrow(pointer)? };
    let _ = context;
    Value::literal("\"boundary_only\"")
}
pub(super) unsafe fn read_thinkthen_request_definition_authored_score_batch_v1(
    pointer: *const thinkthen_request_definition_authored_score_batch_v1,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe {
            Ok(match input.kind {
                THINKTHEN_REQUEST_DEFINITION_AUTHORED_SCORE_BATCH_ARM_1_V1 => {
                    read_thinkthen_request_literal_max_v1(input.data.arm_1, context)?
                }
                THINKTHEN_REQUEST_DEFINITION_AUTHORED_SCORE_BATCH_ARM_2_V1 => {
                    read_thinkthen_request_integer_v1(input.data.arm_2, context)?
                }
                _ => return Err(CarrierError::Invalid),
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_literal_line_v1(
    pointer: *const thinkthen_request_literal_line_v1,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let _ = unsafe { borrow(pointer)? };
    let _ = context;
    Value::literal("\"line\"")
}
pub(super) unsafe fn read_thinkthen_request_literal_window_v1(
    pointer: *const thinkthen_request_literal_window_v1,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let _ = unsafe { borrow(pointer)? };
    let _ = context;
    Value::literal("\"window\"")
}
pub(super) unsafe fn read_thinkthen_request_literal_file_v1(
    pointer: *const thinkthen_request_literal_file_v1,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let _ = unsafe { borrow(pointer)? };
    let _ = context;
    Value::literal("\"file\"")
}
pub(super) unsafe fn read_thinkthen_request_literal_document_v1(
    pointer: *const thinkthen_request_literal_document_v1,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let _ = unsafe { borrow(pointer)? };
    let _ = context;
    Value::literal("\"document\"")
}
pub(super) unsafe fn read_thinkthen_request_literal_lines_v1(
    pointer: *const thinkthen_request_literal_lines_v1,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let _ = unsafe { borrow(pointer)? };
    let _ = context;
    Value::literal("\"lines\"")
}
pub(super) unsafe fn read_thinkthen_request_literal_jsonl_v1(
    pointer: *const thinkthen_request_literal_jsonl_v1,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let _ = unsafe { borrow(pointer)? };
    let _ = context;
    Value::literal("\"jsonl\"")
}
pub(super) unsafe fn read_thinkthen_request_literal_csv_v1(
    pointer: *const thinkthen_request_literal_csv_v1,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let _ = unsafe { borrow(pointer)? };
    let _ = context;
    Value::literal("\"csv\"")
}
pub(super) unsafe fn read_thinkthen_request_literal_tsv_v1(
    pointer: *const thinkthen_request_literal_tsv_v1,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let _ = unsafe { borrow(pointer)? };
    let _ = context;
    Value::literal("\"tsv\"")
}
pub(super) unsafe fn read_thinkthen_request_literal_text_v1(
    pointer: *const thinkthen_request_literal_text_v1,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let _ = unsafe { borrow(pointer)? };
    let _ = context;
    Value::literal("\"text\"")
}
pub(super) unsafe fn read_thinkthen_request_literal_image_v1(
    pointer: *const thinkthen_request_literal_image_v1,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let _ = unsafe { borrow(pointer)? };
    let _ = context;
    Value::literal("\"image\"")
}
pub(super) unsafe fn read_thinkthen_request_item_list_v1(
    pointer: *const thinkthen_request_item_list_v1,
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
                    values.push(read_thinkthen_request_item_v1(*item, context)?);
                }
                Value::Array(values)
            })
        }
    })
}
pub(super) unsafe fn read_thinkthen_request_input_json_value_v1(
    pointer: *const thinkthen_request_input_json_value_v1,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let input = unsafe { borrow(pointer)? };
    context.enter(|context| {
        // SAFETY: nested pointers retain the caller's live counted carrier graph.
        unsafe { Value::literal(&context.text(input.text)?) }
    })
}
pub(super) unsafe fn read_thinkthen_request_literal_max_v1(
    pointer: *const thinkthen_request_literal_max_v1,
    context: &mut Context,
) -> Result<Value, CarrierError> {
    // SAFETY: the caller retains every referenced aligned carrier and counted extent.
    let _ = unsafe { borrow(pointer)? };
    let _ = context;
    Value::literal("\"max\"")
}
