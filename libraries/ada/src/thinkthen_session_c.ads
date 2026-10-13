-- Generated from the canonical C header by GNAT. Do not edit.
with Interfaces;
pragma Style_Checks (Off);
pragma Warnings (Off, "-gnatwu");
with Interfaces.C; use Interfaces.C;
with Interfaces.C.Strings;
with System;
package Thinkthen_Session_C is
   K_THINKTHEN_COBOL_OVERFLOW : constant := 1002;
   K_THINKTHEN_COMPLETE_ANNOTATED_FIELD_ARRAY_V1 : constant := 4;
   K_THINKTHEN_COMPLETE_ANNOTATED_FIELD_BOOLEAN_V1 : constant := 1;
   K_THINKTHEN_COMPLETE_ANNOTATED_FIELD_NULL_V1 : constant := 2;
   K_THINKTHEN_COMPLETE_ANNOTATED_FIELD_NUMBER_V1 : constant := 5;
   K_THINKTHEN_COMPLETE_ANNOTATED_FIELD_OBJECT_V1 : constant := 6;
   K_THINKTHEN_COMPLETE_ANNOTATED_FIELD_STRING_V1 : constant := 3;
   K_THINKTHEN_COMPLETE_ANNOTATION_MEMBER_ANSWER_ID_V1 : constant := 1;
   K_THINKTHEN_COMPLETE_ANNOTATION_MEMBER_FAILURE_ID_V1 : constant := 2;
   K_THINKTHEN_COMPLETE_ANNOTATION_VALUE_CHOICE_V1 : constant := 2;
   K_THINKTHEN_COMPLETE_ANNOTATION_VALUE_DECISION_V1 : constant := 1;
   K_THINKTHEN_COMPLETE_ANNOTATION_VALUE_FAILED_V1 : constant := 5;
   K_THINKTHEN_COMPLETE_ANNOTATION_VALUE_SCORE_V1 : constant := 3;
   K_THINKTHEN_COMPLETE_ANNOTATION_VALUE_TAGS_V1 : constant := 4;
   K_THINKTHEN_COMPLETE_ANSWER_CHOICE_V1 : constant := 2;
   K_THINKTHEN_COMPLETE_ANSWER_SCORE_V1 : constant := 4;
   K_THINKTHEN_COMPLETE_ANSWER_TAG_V1 : constant := 3;
   K_THINKTHEN_COMPLETE_ANSWER_YES_NO_V1 : constant := 1;
   K_THINKTHEN_COMPLETE_ATTEMPT_OUTCOME_OK_V1 : constant := 1;
   K_THINKTHEN_COMPLETE_ATTEMPT_OUTCOME_STATUS_V1 : constant := 2;
   K_THINKTHEN_COMPLETE_ATTEMPT_OUTCOME_TRANSPORT_V1 : constant := 3;
   K_THINKTHEN_COMPLETE_BATCH_INTEGER_V1 : constant := 1;
   K_THINKTHEN_COMPLETE_BATCH_SETTING_INTEGER_V1 : constant := 1;
   K_THINKTHEN_COMPLETE_BATCH_SETTING_STRING_V1 : constant := 2;
   K_THINKTHEN_COMPLETE_BATCH_STRING_MAX_V1 : constant := 1;
   K_THINKTHEN_COMPLETE_BATCH_STRING_V1 : constant := 2;
   K_THINKTHEN_COMPLETE_BOUNDARY_MODE_BOUNDARY_ONLY_V1 : constant := 1;
   K_THINKTHEN_COMPLETE_FAILURE_CAUSE_INVALID_DISTRIBUTION_V1 : constant := 5;
   K_THINKTHEN_COMPLETE_FAILURE_CAUSE_INVALID_PROBABILITY_V1 : constant := 4;
   K_THINKTHEN_COMPLETE_FAILURE_CAUSE_MISSING_ANSWER_V1 : constant := 1;
   K_THINKTHEN_COMPLETE_FAILURE_CAUSE_MISSING_PROBABILITY_V1 : constant := 3;
   K_THINKTHEN_COMPLETE_FAILURE_CAUSE_UNEXPECTED_PROBABILITY_V1 : constant := 6;
   K_THINKTHEN_COMPLETE_FAILURE_CAUSE_WRONG_KIND_V1 : constant := 2;
   K_THINKTHEN_COMPLETE_FAILURE_FIELD_KIND_BACKEND_V1 : constant := 1;
   K_THINKTHEN_COMPLETE_FAILURE_KIND_BACKEND_V1 : constant := 2;
   K_THINKTHEN_COMPLETE_FAILURE_KIND_CANCELLED_V1 : constant := 4;
   K_THINKTHEN_COMPLETE_FAILURE_KIND_DEADLINE_V1 : constant := 5;
   K_THINKTHEN_COMPLETE_FAILURE_KIND_DEFECT_V1 : constant := 6;
   K_THINKTHEN_COMPLETE_FAILURE_KIND_LOCAL_V1 : constant := 3;
   K_THINKTHEN_COMPLETE_FAILURE_KIND_USAGE_V1 : constant := 1;
   K_THINKTHEN_COMPLETE_IMAGE_MEDIA_IMAGE_JPEG_V1 : constant := 1;
   K_THINKTHEN_COMPLETE_IMAGE_MEDIA_IMAGE_PNG_V1 : constant := 2;
   K_THINKTHEN_COMPLETE_INPUT_DECLARATION_OBJECT_ROOT_V1 : constant := 2;
   K_THINKTHEN_COMPLETE_INPUT_DECLARATION_STRING_ROOT_V1 : constant := 1;
   K_THINKTHEN_COMPLETE_INPUT_DENIAL_ADDITIONAL_REQUEST_V1 : constant := 2;
   K_THINKTHEN_COMPLETE_INPUT_DENIAL_INITIAL_REQUEST_V1 : constant := 1;
   K_THINKTHEN_COMPLETE_INPUT_DENIAL_RETRY_V1 : constant := 3;
   K_THINKTHEN_COMPLETE_INPUT_PROPERTY_TYPE_ARRAY_V1 : constant := 4;
   K_THINKTHEN_COMPLETE_INPUT_PROPERTY_TYPE_BOOLEAN_V1 : constant := 3;
   K_THINKTHEN_COMPLETE_INPUT_PROPERTY_TYPE_NUMBER_V1 : constant := 2;
   K_THINKTHEN_COMPLETE_INPUT_PROPERTY_TYPE_STRING_V1 : constant := 1;
   K_THINKTHEN_COMPLETE_JSON_ARRAY_V1 : constant := 5;
   K_THINKTHEN_COMPLETE_JSON_BOOLEAN_V1 : constant := 2;
   K_THINKTHEN_COMPLETE_JSON_NULL_V1 : constant := 1;
   K_THINKTHEN_COMPLETE_JSON_NUMBER_V1 : constant := 3;
   K_THINKTHEN_COMPLETE_JSON_OBJECT_V1 : constant := 6;
   K_THINKTHEN_COMPLETE_JSON_STRING_V1 : constant := 4;
   K_THINKTHEN_COMPLETE_OBJECT_TYPE_OBJECT_V1 : constant := 1;
   K_THINKTHEN_COMPLETE_OBSERVATION_FAILURE_ID_V1 : constant := 2;
   K_THINKTHEN_COMPLETE_OBSERVATION_OBSERVATION_ID_V1 : constant := 1;
   K_THINKTHEN_COMPLETE_ORIGIN_CACHE_V1 : constant := 2;
   K_THINKTHEN_COMPLETE_ORIGIN_LIVE_V1 : constant := 1;
   K_THINKTHEN_COMPLETE_ORIGIN_MEMORY_V1 : constant := 5;
   K_THINKTHEN_COMPLETE_ORIGIN_PROXY_V1 : constant := 4;
   K_THINKTHEN_COMPLETE_ORIGIN_REPLAY_V1 : constant := 3;
   K_THINKTHEN_COMPLETE_READABLE_QUESTION_CHOOSE_V1 : constant := 2;
   K_THINKTHEN_COMPLETE_READABLE_QUESTION_DECIDE_V1 : constant := 1;
   K_THINKTHEN_COMPLETE_READABLE_QUESTION_SCORE_V1 : constant := 4;
   K_THINKTHEN_COMPLETE_READABLE_QUESTION_TAG_V1 : constant := 3;
   K_THINKTHEN_COMPLETE_RECOGNITION_MODE_BOUNDARY_ONLY_V1 : constant := 2;
   K_THINKTHEN_COMPLETE_RECOGNITION_MODE_WHOLE_V1 : constant := 1;
   K_THINKTHEN_COMPLETE_RECOGNITION_ODDS_BOUNDARY_ODDS_V1 : constant := 2;
   K_THINKTHEN_COMPLETE_RECOGNITION_ODDS_RECOGNIZE_ANSWER_V1 : constant := 1;
   K_THINKTHEN_COMPLETE_RECOGNIZE_ENTITIES_V1 : constant := 1;
   K_THINKTHEN_COMPLETE_RECOGNIZE_MODE_PROPOSALS_V1 : constant := 2;
   K_THINKTHEN_COMPLETE_RELATION_DIRECTION_EITHER_V1 : constant := 2;
   K_THINKTHEN_COMPLETE_RELATION_DIRECTION_SOURCE_TO_TARGET_V1 : constant := 1;
   K_THINKTHEN_COMPLETE_RELATION_ENDPOINT_RELATED_ENTITY_V1 : constant := 1;
   K_THINKTHEN_COMPLETE_RELATION_ENDPOINT_SOURCE_V1 : constant := 2;
   K_THINKTHEN_COMPLETE_RELATION_MEMBER_ANSWER_ID_V1 : constant := 1;
   K_THINKTHEN_COMPLETE_RELATION_MEMBER_FAILURE_ID_V1 : constant := 2;
   K_THINKTHEN_COMPLETE_RELATION_METHOD_CHOICE_V1 : constant := 2;
   K_THINKTHEN_COMPLETE_RELATION_METHOD_YES_NO_V1 : constant := 1;
   K_THINKTHEN_COMPLETE_REQUEST_FUNCTION_ANNOTATE_V1 : constant := 8;
   K_THINKTHEN_COMPLETE_REQUEST_FUNCTION_CHOOSE_V1 : constant := 2;
   K_THINKTHEN_COMPLETE_REQUEST_FUNCTION_DECIDE_V1 : constant := 1;
   K_THINKTHEN_COMPLETE_REQUEST_FUNCTION_FILTER_V1 : constant := 5;
   K_THINKTHEN_COMPLETE_REQUEST_FUNCTION_FIND_V1 : constant := 7;
   K_THINKTHEN_COMPLETE_REQUEST_FUNCTION_RANK_V1 : constant := 6;
   K_THINKTHEN_COMPLETE_REQUEST_FUNCTION_RECOGNIZE_V1 : constant := 9;
   K_THINKTHEN_COMPLETE_REQUEST_FUNCTION_RELATE_V1 : constant := 10;
   K_THINKTHEN_COMPLETE_REQUEST_FUNCTION_SCORE_V1 : constant := 4;
   K_THINKTHEN_COMPLETE_REQUEST_FUNCTION_TAG_V1 : constant := 3;
   K_THINKTHEN_COMPLETE_SEND_DENIAL_BEFORE_ADDITIONAL_SEND_V1 : constant := 2;
   K_THINKTHEN_COMPLETE_SEND_DENIAL_BEFORE_FIRST_SEND_V1 : constant := 1;
   K_THINKTHEN_COMPLETE_SEND_DENIAL_BEFORE_RETRY_V1 : constant := 3;
   K_THINKTHEN_COMPLETE_SESSION_JUDGMENT_CHOICE_V1 : constant := 2;
   K_THINKTHEN_COMPLETE_SESSION_JUDGMENT_DECISION_V1 : constant := 1;
   K_THINKTHEN_COMPLETE_SESSION_JUDGMENT_SCORE_V1 : constant := 3;
   K_THINKTHEN_COMPLETE_SESSION_JUDGMENT_TAGS_V1 : constant := 4;
   K_THINKTHEN_COMPLETE_SESSION_OBSERVATION_QUESTION_V1 : constant := 1;
   K_THINKTHEN_COMPLETE_SESSION_OBSERVATION_ROW_V1 : constant := 2;
   K_THINKTHEN_COMPLETE_SESSION_OBSERVED_ROW_ANNOTATED_V1 : constant := 2;
   K_THINKTHEN_COMPLETE_SESSION_OBSERVED_ROW_FIND_V1 : constant := 4;
   K_THINKTHEN_COMPLETE_SESSION_OBSERVED_ROW_JUDGMENT_V1 : constant := 1;
   K_THINKTHEN_COMPLETE_SESSION_OBSERVED_ROW_RECOGNIZED_V1 : constant := 3;
   K_THINKTHEN_COMPLETE_SESSION_OBSERVED_ROW_RELATIONS_V1 : constant := 5;
   K_THINKTHEN_COMPLETE_SESSION_PACKET_ANNOTATE_AGGREGATE_V1 : constant := 14;
   K_THINKTHEN_COMPLETE_SESSION_PACKET_ANNOTATE_ROW_V1 : constant := 6;
   K_THINKTHEN_COMPLETE_SESSION_PACKET_CHOOSE_AGGREGATE_V1 : constant := 8;
   K_THINKTHEN_COMPLETE_SESSION_PACKET_CHOOSE_ROW_V1 : constant := 2;
   K_THINKTHEN_COMPLETE_SESSION_PACKET_DECIDE_AGGREGATE_V1 : constant := 7;
   K_THINKTHEN_COMPLETE_SESSION_PACKET_DECIDE_ROW_V1 : constant := 1;
   K_THINKTHEN_COMPLETE_SESSION_PACKET_FILTER_AGGREGATE_V1 : constant := 11;
   K_THINKTHEN_COMPLETE_SESSION_PACKET_FILTER_ROW_V1 : constant := 5;
   K_THINKTHEN_COMPLETE_SESSION_PACKET_FIND_AGGREGATE_V1 : constant := 13;
   K_THINKTHEN_COMPLETE_SESSION_PACKET_OBSERVATION_V1 : constant := 17;
   K_THINKTHEN_COMPLETE_SESSION_PACKET_RANK_AGGREGATE_V1 : constant := 12;
   K_THINKTHEN_COMPLETE_SESSION_PACKET_RECOGNIZE_AGGREGATE_V1 : constant := 15;
   K_THINKTHEN_COMPLETE_SESSION_PACKET_RELATE_AGGREGATE_V1 : constant := 16;
   K_THINKTHEN_COMPLETE_SESSION_PACKET_SCORE_AGGREGATE_V1 : constant := 10;
   K_THINKTHEN_COMPLETE_SESSION_PACKET_SCORE_ROW_V1 : constant := 4;
   K_THINKTHEN_COMPLETE_SESSION_PACKET_TAG_AGGREGATE_V1 : constant := 9;
   K_THINKTHEN_COMPLETE_SESSION_PACKET_TAG_ROW_V1 : constant := 3;
   K_THINKTHEN_COMPLETE_SESSION_PACKET_TERMINAL_V1 : constant := 18;
   K_THINKTHEN_COMPLETE_SESSION_PROBABILITIES_NAMED_V1 : constant := 2;
   K_THINKTHEN_COMPLETE_SESSION_PROBABILITIES_YES_NO_V1 : constant := 1;
   K_THINKTHEN_COMPLETE_STOP_CAUSE_BACKEND_V1 : constant := 8;
   K_THINKTHEN_COMPLETE_STOP_CAUSE_CANCELLED_V1 : constant := 9;
   K_THINKTHEN_COMPLETE_STOP_CAUSE_DEADLINE_V1 : constant := 10;
   K_THINKTHEN_COMPLETE_STOP_CAUSE_DEFECT_V1 : constant := 11;
   K_THINKTHEN_COMPLETE_STOP_CAUSE_LOCAL_V1 : constant := 2;
   K_THINKTHEN_COMPLETE_STOP_CAUSE_NO_KEY_V1 : constant := 3;
   K_THINKTHEN_COMPLETE_STOP_CAUSE_REPLY_V1 : constant := 7;
   K_THINKTHEN_COMPLETE_STOP_CAUSE_STATUS_V1 : constant := 5;
   K_THINKTHEN_COMPLETE_STOP_CAUSE_TOO_LARGE_V1 : constant := 6;
   K_THINKTHEN_COMPLETE_STOP_CAUSE_TRANSPORT_V1 : constant := 4;
   K_THINKTHEN_COMPLETE_STOP_CAUSE_USAGE_V1 : constant := 1;
   K_THINKTHEN_COMPLETE_STRING_TYPE_STRING_V1 : constant := 1;
   K_THINKTHEN_COMPLETE_THRESHOLD_NUMBER_V1 : constant := 1;
   K_THINKTHEN_COMPLETE_THRESHOLD_STRING_V1 : constant := 2;
   K_THINKTHEN_COMPLETE_USAGE_PERSISTENCE_DISABLED_V1 : constant := 1;
   K_THINKTHEN_COMPLETE_USAGE_PERSISTENCE_FAILED_V1 : constant := 4;
   K_THINKTHEN_COMPLETE_USAGE_PERSISTENCE_PENDING_V1 : constant := 2;
   K_THINKTHEN_COMPLETE_USAGE_PERSISTENCE_WRITTEN_V1 : constant := 3;
   K_THINKTHEN_COMPLETE_VALUE_ARRAY_V1 : constant := 4;
   K_THINKTHEN_COMPLETE_VALUE_BOOLEAN_V1 : constant := 1;
   K_THINKTHEN_COMPLETE_VALUE_NULL_V1 : constant := 2;
   K_THINKTHEN_COMPLETE_VALUE_NUMBER_V1 : constant := 5;
   K_THINKTHEN_COMPLETE_VALUE_STRING_V1 : constant := 3;
   K_THINKTHEN_COMPLETE_VERB_RECOGNIZE_V1 : constant := 1;
   K_THINKTHEN_COMPLETE_VERSION_THINKTHEN_RESULT_2_V1 : constant := 1;
   K_THINKTHEN_EBACKEND : constant := 2;
   K_THINKTHEN_ECANCELLED : constant := 5;
   K_THINKTHEN_EDEADLINE : constant := 3;
   K_THINKTHEN_EDEFECT : constant := 6;
   K_THINKTHEN_ELOCAL : constant := 4;
   K_THINKTHEN_EUSAGE : constant := 1;
   K_THINKTHEN_EVENT_QUESTION_V1 : constant := 1;
   K_THINKTHEN_EVENT_ROW_V1 : constant := 2;
   K_THINKTHEN_REQUEST_ANNOTATE_QUESTION_ARM_1_V1 : constant := 1;
   K_THINKTHEN_REQUEST_ANNOTATE_QUESTION_ARM_2_V1 : constant := 2;
   K_THINKTHEN_REQUEST_ANNOTATE_QUESTION_ARM_3_V1 : constant := 3;
   K_THINKTHEN_REQUEST_ANNOTATE_QUESTION_ARM_4_V1 : constant := 4;
   K_THINKTHEN_REQUEST_BATCH_ARM_1_V1 : constant := 1;
   K_THINKTHEN_REQUEST_BATCH_ARM_2_V1 : constant := 2;
   K_THINKTHEN_REQUEST_CALL_THRESHOLD_ARM_1_V1 : constant := 1;
   K_THINKTHEN_REQUEST_CALL_THRESHOLD_ARM_2_V1 : constant := 2;
   K_THINKTHEN_REQUEST_CONTEXT_SCHEMA_ARM_1_V1 : constant := 1;
   K_THINKTHEN_REQUEST_CONTEXT_SCHEMA_ARM_2_V1 : constant := 2;
   K_THINKTHEN_REQUEST_CRITERION_ARM_1_V1 : constant := 1;
   K_THINKTHEN_REQUEST_CRITERION_ARM_2_V1 : constant := 2;
   K_THINKTHEN_REQUEST_CRITERION_ARM_3_V1 : constant := 3;
   K_THINKTHEN_REQUEST_CRITERION_ARM_4_V1 : constant := 4;
   K_THINKTHEN_REQUEST_CUT_ARM_1_V1 : constant := 1;
   K_THINKTHEN_REQUEST_CUT_ARM_2_V1 : constant := 2;
   K_THINKTHEN_REQUEST_DEFINITION_ARM_1_V1 : constant := 1;
   K_THINKTHEN_REQUEST_DEFINITION_ARM_2_V1 : constant := 2;
   K_THINKTHEN_REQUEST_DEFINITION_ARM_3_V1 : constant := 3;
   K_THINKTHEN_REQUEST_DEFINITION_ARM_4_V1 : constant := 4;
   K_THINKTHEN_REQUEST_DEFINITION_ARM_5_V1 : constant := 5;
   K_THINKTHEN_REQUEST_DEFINITION_ARM_6_V1 : constant := 6;
   K_THINKTHEN_REQUEST_DEFINITION_ARM_7_V1 : constant := 7;
   K_THINKTHEN_REQUEST_DEFINITION_ARM_8_V1 : constant := 8;
   K_THINKTHEN_REQUEST_DEFINITION_AUTHORED_SCORE_BATCH_ARM_1_V1 : constant := 1;
   K_THINKTHEN_REQUEST_DEFINITION_AUTHORED_SCORE_BATCH_ARM_2_V1 : constant := 2;
   K_THINKTHEN_REQUEST_DESCRIPTION_ARM_1_V1 : constant := 1;
   K_THINKTHEN_REQUEST_DESCRIPTION_ARM_2_V1 : constant := 2;
   K_THINKTHEN_REQUEST_DESCRIPTION_ARM_3_V1 : constant := 3;
   K_THINKTHEN_REQUEST_DESCRIPTION_ARM_4_V1 : constant := 4;
   K_THINKTHEN_REQUEST_FRAMING_ARM_1_V1 : constant := 1;
   K_THINKTHEN_REQUEST_FRAMING_ARM_2_V1 : constant := 2;
   K_THINKTHEN_REQUEST_FRAMING_ARM_3_V1 : constant := 3;
   K_THINKTHEN_REQUEST_FRAMING_ARM_4_V1 : constant := 4;
   K_THINKTHEN_REQUEST_FRAMING_ARM_5_V1 : constant := 5;
   K_THINKTHEN_REQUEST_IMAGE_ARM_1_V1 : constant := 1;
   K_THINKTHEN_REQUEST_IMAGE_ARM_2_V1 : constant := 2;
   K_THINKTHEN_REQUEST_IMAGE_MEDIA_ARM_1_V1 : constant := 1;
   K_THINKTHEN_REQUEST_IMAGE_MEDIA_ARM_2_V1 : constant := 2;
   K_THINKTHEN_REQUEST_INPUT_ARM_1_V1 : constant := 1;
   K_THINKTHEN_REQUEST_INPUT_ARM_2_V1 : constant := 2;
   K_THINKTHEN_REQUEST_INPUT_ARM_3_V1 : constant := 3;
   K_THINKTHEN_REQUEST_INPUT_ARM_4_V1 : constant := 4;
   K_THINKTHEN_REQUEST_INPUT_ARM_5_V1 : constant := 5;
   K_THINKTHEN_REQUEST_INPUT_ARM_6_V1 : constant := 6;
   K_THINKTHEN_REQUEST_INPUT_ARM_7_V1 : constant := 7;
   K_THINKTHEN_REQUEST_INPUT_DECLARATION_ARM_1_V1 : constant := 1;
   K_THINKTHEN_REQUEST_INPUT_DECLARATION_ARM_2_V1 : constant := 2;
   K_THINKTHEN_REQUEST_INPUT_PROPERTY_ARM_1_V1 : constant := 1;
   K_THINKTHEN_REQUEST_INPUT_PROPERTY_ARM_2_V1 : constant := 2;
   K_THINKTHEN_REQUEST_INPUT_PROPERTY_ARM_3_V1 : constant := 3;
   K_THINKTHEN_REQUEST_INPUT_PROPERTY_ARM_4_V1 : constant := 4;
   K_THINKTHEN_REQUEST_LABELS_ARM_1_V1 : constant := 1;
   K_THINKTHEN_REQUEST_LABELS_ARM_2_V1 : constant := 2;
   K_THINKTHEN_REQUEST_LEVELS_ARM_1_V1 : constant := 1;
   K_THINKTHEN_REQUEST_LEVELS_ARM_2_V1 : constant := 2;
   K_THINKTHEN_REQUEST_OPTIONS_ARM_1_V1 : constant := 1;
   K_THINKTHEN_REQUEST_OPTIONS_ARM_2_V1 : constant := 2;
   K_THINKTHEN_REQUEST_ORIGINAL_ARM_1_V1 : constant := 1;
   K_THINKTHEN_REQUEST_ORIGINAL_ARM_2_V1 : constant := 2;
   K_THINKTHEN_REQUEST_POINTERS_ARM_1_V1 : constant := 1;
   K_THINKTHEN_REQUEST_POINTERS_ARM_2_V1 : constant := 2;
   K_THINKTHEN_REQUEST_QUESTION_SELECTOR_ARM_1_V1 : constant := 1;
   K_THINKTHEN_REQUEST_QUESTION_SELECTOR_ARM_2_V1 : constant := 2;
   K_THINKTHEN_REQUEST_QUESTION_SELECTOR_ARM_3_V1 : constant := 3;
   K_THINKTHEN_REQUEST_QUESTION_SELECTOR_ARM_4_V1 : constant := 4;
   K_THINKTHEN_REQUEST_QUESTION_SELECTOR_ARM_5_V1 : constant := 5;
   K_THINKTHEN_REQUEST_QUESTION_TEXT_ARM_1_V1 : constant := 1;
   K_THINKTHEN_REQUEST_QUESTION_TEXT_ARM_2_V1 : constant := 2;
   K_THINKTHEN_REQUEST_QUESTION_TEXT_ARM_3_V1 : constant := 3;
   K_THINKTHEN_REQUEST_READER_FAILURE_ARM_1_V1 : constant := 1;
   K_THINKTHEN_REQUEST_READER_FAILURE_ARM_2_V1 : constant := 2;
   K_THINKTHEN_REQUEST_READER_FAILURE_ARM_3_V1 : constant := 3;
   K_THINKTHEN_REQUEST_READER_FAILURE_ARM_4_V1 : constant := 4;
   K_THINKTHEN_REQUEST_READER_MEDIA_ARM_1_V1 : constant := 1;
   K_THINKTHEN_REQUEST_READER_MEDIA_ARM_2_V1 : constant := 2;
   K_THINKTHEN_REQUEST_RECOGNITION_EXAMPLE_ARM_1_V1 : constant := 1;
   K_THINKTHEN_REQUEST_RECOGNITION_EXAMPLE_ARM_2_V1 : constant := 2;
   K_THINKTHEN_REQUEST_RECOGNITION_MODE_ARM_1_V1 : constant := 1;
   K_THINKTHEN_REQUEST_RECOGNITION_MODE_ARM_2_V1 : constant := 2;
   K_THINKTHEN_REQUEST_SOURCE_UNIT_ARM_1_V1 : constant := 1;
   K_THINKTHEN_REQUEST_SOURCE_UNIT_ARM_2_V1 : constant := 2;
   K_THINKTHEN_REQUEST_SOURCE_UNIT_ARM_3_V1 : constant := 3;
   K_THINKTHEN_REQUEST_THRESHOLD_ARM_1_V1 : constant := 1;
   K_THINKTHEN_REQUEST_THRESHOLD_ARM_2_V1 : constant := 2;
   K_THINKTHEN_SESSION_ACCEPTED_V1 : constant := 0;
   K_THINKTHEN_SESSION_CLOSED_V1 : constant := 2;
   K_THINKTHEN_SESSION_END_V1 : constant := 2;
   K_THINKTHEN_SESSION_FULL_V1 : constant := 1;
   K_THINKTHEN_SESSION_PENDING_V1 : constant := 1;
   K_THINKTHEN_SESSION_READY_CANCELLED_V1 : constant := 4;
   K_THINKTHEN_SESSION_READY_CLOSED_V1 : constant := 2;
   K_THINKTHEN_SESSION_READY_END_V1 : constant := 3;
   K_THINKTHEN_SESSION_READY_INPUT_V1 : constant := 1;
   K_THINKTHEN_SESSION_READY_RESULT_V1 : constant := 0;
   K_THINKTHEN_SESSION_READY_TIMEOUT_V1 : constant := 5;
   K_THINKTHEN_SESSION_RESULT_V1 : constant := 0;
   K_THINKTHEN_SESSION_WAIT_INPUT_V1 : constant := 1;
   K_THINKTHEN_SESSION_WAIT_RESULT_V1 : constant := 0;
   K_THINKTHEN_SESSION_WAIT_SETTLEMENT_V1 : constant := 2;
   type thinkthen_engine is null record;
   type thinkthen_session is null record;
   type thinkthen_session_result is null record;
   type thinkthen_complete_usage_persistence_v1 is record
      kind : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_utf8_v1 is record
      data : Interfaces.C.Strings.chars_ptr;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_reader_failure_producer_v1 is record
      reserved : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_text_v1 is record
      text : aliased thinkthen_complete_utf8_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_unsigned_v1 is record
      value : aliased Interfaces.Unsigned_64;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_session_source_location_v1 is record
      m_file : access constant thinkthen_request_text_v1;
      m_first_line : access constant thinkthen_request_unsigned_v1;
      m_last_line : access constant thinkthen_request_unsigned_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_reader_failure_io_v1 is record
      m_location : access constant thinkthen_request_session_source_location_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_reader_failure_utf_8_v1 is record
      m_location : access constant thinkthen_request_session_source_location_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_reader_failure_invalid_input_v1 is record
      m_location : access constant thinkthen_request_session_source_location_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_reader_failure_data_v1 (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            arm_1 : access constant thinkthen_request_reader_failure_producer_v1;
         when 1 =>
            arm_2 : access constant thinkthen_request_reader_failure_io_v1;
         when 2 =>
            arm_3 : access constant thinkthen_request_reader_failure_utf_8_v1;
         when others =>
            arm_4 : access constant thinkthen_request_reader_failure_invalid_input_v1;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_request_reader_failure_v1 is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_request_reader_failure_data_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_named_request_v1 is record
      c_function : aliased thinkthen_complete_utf8_v1;
      question_json : aliased thinkthen_complete_utf8_v1;
      input_json : aliased thinkthen_complete_utf8_v1;
      options_json : aliased thinkthen_complete_utf8_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_literal_image_jpeg_v1 is record
      reserved : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_literal_image_png_v1 is record
      reserved : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_image_media_data_v1 (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            arm_1 : access constant thinkthen_request_literal_image_jpeg_v1;
         when others =>
            arm_2 : access constant thinkthen_request_literal_image_png_v1;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_request_image_media_v1 is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_request_image_media_data_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_image_file_v1 is record
      m_media : access constant thinkthen_request_image_media_v1;
      m_path : access constant thinkthen_request_text_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_image_bytes_v1 is record
      m_bytes : access constant thinkthen_request_text_v1;
      m_media : access constant thinkthen_request_image_media_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_image_data_v1 (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            arm_1 : access constant thinkthen_request_image_file_v1;
         when others =>
            arm_2 : access constant thinkthen_request_image_bytes_v1;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_request_image_v1 is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_request_image_data_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_image_list_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_input_text_v1 is record
      m_images : access constant thinkthen_request_image_list_v1;
      m_text : access constant thinkthen_request_text_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_input_json_value_v1 is record
      text : aliased thinkthen_complete_utf8_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_input_json_v1 is record
      m_images : access constant thinkthen_request_image_list_v1;
      m_value : access constant thinkthen_request_input_json_value_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_json_value_v1 is record
      text : aliased thinkthen_complete_utf8_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_json_value_map_entry_v1 is record
      key : aliased thinkthen_complete_utf8_v1;
      value : access constant thinkthen_request_json_value_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_json_value_map_v1 is record
      data : access constant thinkthen_request_json_value_map_entry_v1;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_context_schema_data_v1 (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            arm_1 : access constant thinkthen_request_text_v1;
         when others =>
            arm_2 : access constant thinkthen_request_json_value_map_v1;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_request_context_schema_v1 is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_request_context_schema_data_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_recognition_example_entity_v1 is record
      m_end : access constant thinkthen_request_unsigned_v1;
      m_kind : access constant thinkthen_request_text_v1;
      m_start : access constant thinkthen_request_unsigned_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_recognition_example_entity_list_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_text_list_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_recognition_example_text_v1 is record
      m_entities : access constant thinkthen_request_recognition_example_entity_list_v1;
      m_kinds : access constant thinkthen_request_text_list_v1;
      m_text : access constant thinkthen_request_text_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_recognition_example_data_v1 (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            arm_1 : access constant thinkthen_request_text_v1;
         when others =>
            arm_2 : access constant thinkthen_request_recognition_example_text_v1;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_request_recognition_example_v1 is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_request_recognition_example_data_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_recognition_example_list_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_option_schema_v1 is record
      m_description : access constant thinkthen_request_json_value_v1;
      m_name : access constant thinkthen_request_text_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_option_schema_list_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_original_text_v1 is record
      m_text : access constant thinkthen_request_text_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_original_json_value_v1 is record
      text : aliased thinkthen_complete_utf8_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_original_json_v1 is record
      m_value : access constant thinkthen_request_original_json_value_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_original_data_v1 (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            arm_1 : access constant thinkthen_request_original_text_v1;
         when others =>
            arm_2 : access constant thinkthen_request_original_json_v1;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_request_original_v1 is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_request_original_data_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_recognition_seed_span_v1 is record
      m_end : access constant thinkthen_request_unsigned_v1;
      m_kind : access constant thinkthen_request_text_v1;
      m_start : access constant thinkthen_request_unsigned_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_recognition_seed_span_list_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_item_v1 is record
      m_context : access constant thinkthen_request_context_schema_v1;
      m_examples : access constant thinkthen_request_recognition_example_list_v1;
      m_images : access constant thinkthen_request_image_list_v1;
      m_options : access constant thinkthen_request_option_schema_list_v1;
      m_original : access constant thinkthen_request_original_v1;
      m_seed_spans : access constant thinkthen_request_recognition_seed_span_list_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_item_list_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_input_records_v1 is record
      m_items : access constant thinkthen_request_item_list_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_input_units_v1 is record
      m_items : access constant thinkthen_request_item_list_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_input_entities_v1 is record
      m_items : access constant thinkthen_request_item_list_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_literal_document_v1 is record
      reserved : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_literal_lines_v1 is record
      reserved : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_literal_jsonl_v1 is record
      reserved : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_literal_csv_v1 is record
      reserved : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_literal_tsv_v1 is record
      reserved : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_framing_data_v1 (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            arm_1 : access constant thinkthen_request_literal_document_v1;
         when 1 =>
            arm_2 : access constant thinkthen_request_literal_lines_v1;
         when 2 =>
            arm_3 : access constant thinkthen_request_literal_jsonl_v1;
         when 3 =>
            arm_4 : access constant thinkthen_request_literal_csv_v1;
         when others =>
            arm_5 : access constant thinkthen_request_literal_tsv_v1;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_request_framing_v1 is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_request_framing_data_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_literal_text_v1 is record
      reserved : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_literal_image_v1 is record
      reserved : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_reader_media_data_v1 (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            arm_1 : access constant thinkthen_request_literal_text_v1;
         when others =>
            arm_2 : access constant thinkthen_request_literal_image_v1;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_request_reader_media_v1 is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_request_reader_media_data_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_literal_line_v1 is record
      reserved : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_literal_window_v1 is record
      reserved : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_literal_file_v1 is record
      reserved : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_source_unit_data_v1 (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            arm_1 : access constant thinkthen_request_literal_line_v1;
         when 1 =>
            arm_2 : access constant thinkthen_request_literal_window_v1;
         when others =>
            arm_3 : access constant thinkthen_request_literal_file_v1;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_request_source_unit_v1 is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_request_source_unit_data_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_reader_v1 is record
      m_unit : access constant thinkthen_request_source_unit_v1;
      m_window : access constant thinkthen_request_unsigned_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_source_v1 is record
      m_framing : access constant thinkthen_request_framing_v1;
      m_media : access constant thinkthen_request_reader_media_v1;
      m_paths : access constant thinkthen_request_text_list_v1;
      m_reading : access constant thinkthen_request_reader_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_input_source_v1 is record
      m_source : access constant thinkthen_request_source_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_input_feed_v1 is record
      m_framing : access constant thinkthen_request_framing_v1;
      m_images : access constant thinkthen_request_image_list_v1;
      m_name : access constant thinkthen_request_text_v1;
      m_reading : access constant thinkthen_request_reader_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_input_data_v1 (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            arm_1 : access constant thinkthen_request_input_text_v1;
         when 1 =>
            arm_2 : access constant thinkthen_request_input_json_v1;
         when 2 =>
            arm_3 : access constant thinkthen_request_input_records_v1;
         when 3 =>
            arm_4 : access constant thinkthen_request_input_units_v1;
         when 4 =>
            arm_5 : access constant thinkthen_request_input_entities_v1;
         when 5 =>
            arm_6 : access constant thinkthen_request_input_source_v1;
         when others =>
            arm_7 : access constant thinkthen_request_input_feed_v1;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_request_input_v1 is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_request_input_data_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_boolean_v1 is record
      value : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_batch_data_v1 (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            arm_1 : access constant thinkthen_request_unsigned_v1;
         when others =>
            arm_2 : access constant thinkthen_request_text_v1;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_request_batch_v1 is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_request_batch_data_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_integer_v1 is record
      value : aliased Interfaces.Integer_64;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_literal_whole_v1 is record
      reserved : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_literal_boundary_only_v1 is record
      reserved : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_recognition_mode_data_v1 (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            arm_1 : access constant thinkthen_request_literal_whole_v1;
         when others =>
            arm_2 : access constant thinkthen_request_literal_boundary_only_v1;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_request_recognition_mode_v1 is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_request_recognition_mode_data_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_number_v1 is record
      value : aliased double;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_call_threshold_data_v1 (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            arm_1 : access constant thinkthen_request_number_v1;
         when others =>
            arm_2 : access constant thinkthen_request_text_v1;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_request_call_threshold_v1 is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_request_call_threshold_data_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_recognition_stage_context_v1 is record
      m_boundary : access constant thinkthen_request_text_v1;
      m_kind_edge : access constant thinkthen_request_text_v1;
      m_relation : access constant thinkthen_request_text_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_call_options_v1 is record
      m_attempts : access constant thinkthen_request_boolean_v1;
      m_batch : access constant thinkthen_request_batch_v1;
      m_context : access constant thinkthen_request_text_v1;
      m_context_field : access constant thinkthen_request_text_v1;
      m_deadline_ms : access constant thinkthen_request_integer_v1;
      m_details : access constant thinkthen_request_boolean_v1;
      m_examples : access constant thinkthen_request_recognition_example_list_v1;
      m_examples_field : access constant thinkthen_request_text_v1;
      m_field : access constant thinkthen_request_text_list_v1;
      m_files_only : access constant thinkthen_request_boolean_v1;
      m_max_requests_total : access constant thinkthen_request_unsigned_v1;
      m_mode : access constant thinkthen_request_recognition_mode_v1;
      m_model : access constant thinkthen_request_text_v1;
      m_none : access constant thinkthen_request_boolean_v1;
      m_options_field : access constant thinkthen_request_text_v1;
      m_relation_threshold : access constant thinkthen_request_call_threshold_v1;
      m_seed_spans : access constant thinkthen_request_recognition_seed_span_list_v1;
      m_seed_spans_field : access constant thinkthen_request_text_v1;
      m_snippet_pieces : access constant thinkthen_request_unsigned_v1;
      m_stage_context : access constant thinkthen_request_recognition_stage_context_v1;
      m_threshold : access constant thinkthen_request_call_threshold_v1;
      m_top : access constant thinkthen_request_unsigned_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_question_selector_text_v1 is record
      m_text : access constant thinkthen_request_text_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_literal_max_v1 is record
      reserved : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_definition_authored_score_batch_data_v1 (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            arm_1 : access constant thinkthen_request_literal_max_v1;
         when others =>
            arm_2 : access constant thinkthen_request_integer_v1;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_request_definition_authored_score_batch_v1 is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_request_definition_authored_score_batch_data_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_input_declaration_string_v1 is record
      reserved : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_input_property_string_v1 is record
      reserved : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_input_property_number_v1 is record
      reserved : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_input_property_boolean_v1 is record
      reserved : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_input_property_array_items_v1 is record
      reserved : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_input_property_array_v1 is record
      m_items : access constant thinkthen_request_input_property_array_items_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_input_property_data_v1 (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            arm_1 : access constant thinkthen_request_input_property_string_v1;
         when 1 =>
            arm_2 : access constant thinkthen_request_input_property_number_v1;
         when 2 =>
            arm_3 : access constant thinkthen_request_input_property_boolean_v1;
         when others =>
            arm_4 : access constant thinkthen_request_input_property_array_v1;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_request_input_property_v1 is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_request_input_property_data_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_input_property_map_entry_v1 is record
      key : aliased thinkthen_complete_utf8_v1;
      value : access constant thinkthen_request_input_property_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_input_property_map_v1 is record
      data : access constant thinkthen_request_input_property_map_entry_v1;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_input_declaration_object_v1 is record
      m_properties : access constant thinkthen_request_input_property_map_v1;
      m_required : access constant thinkthen_request_text_list_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_input_declaration_data_v1 (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            arm_1 : access constant thinkthen_request_input_declaration_string_v1;
         when others =>
            arm_2 : access constant thinkthen_request_input_declaration_object_v1;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_request_input_declaration_v1 is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_request_input_declaration_data_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_json_value_list_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_question_text_data_v1 (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            arm_1 : access constant thinkthen_request_text_v1;
         when 1 =>
            arm_2 : access constant thinkthen_request_json_value_map_v1;
         when others =>
            arm_3 : access constant thinkthen_request_json_value_list_v1;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_request_question_text_v1 is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_request_question_text_data_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_null_v1 is record
      reserved : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_criterion_data_v1 (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            arm_1 : access constant thinkthen_request_text_v1;
         when 1 =>
            arm_2 : access constant thinkthen_request_json_value_map_v1;
         when 2 =>
            arm_3 : access constant thinkthen_request_json_value_list_v1;
         when others =>
            arm_4 : access constant thinkthen_request_null_v1;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_request_criterion_v1 is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_request_criterion_data_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_name_v1 is record
      text : aliased thinkthen_complete_utf8_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_pointers_data_v1 (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            arm_1 : access constant thinkthen_request_text_v1;
         when others =>
            arm_2 : access constant thinkthen_request_text_list_v1;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_request_pointers_v1 is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_request_pointers_data_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_profile_v1 is record
      text : aliased thinkthen_complete_utf8_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_threshold_data_v1 (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            arm_1 : access constant thinkthen_request_number_v1;
         when others =>
            arm_2 : access constant thinkthen_request_text_v1;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_request_threshold_v1 is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_request_threshold_data_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_definition_authored_decide_v1 is record
      m_batch : access constant thinkthen_request_definition_authored_score_batch_v1;
      m_context_schema : access constant thinkthen_request_input_declaration_v1;
      m_decide : access constant thinkthen_request_question_text_v1;
      m_false : access constant thinkthen_request_criterion_v1;
      m_item_schema : access constant thinkthen_request_input_declaration_v1;
      m_model : access constant thinkthen_request_name_v1;
      m_name : access constant thinkthen_request_text_v1;
      m_on : access constant thinkthen_request_pointers_v1;
      m_profile : access constant thinkthen_request_profile_v1;
      m_threshold : access constant thinkthen_request_threshold_v1;
      m_true : access constant thinkthen_request_criterion_v1;
      m_wording_version : access constant thinkthen_request_integer_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_name_list_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_description_data_v1 (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            arm_1 : access constant thinkthen_request_text_v1;
         when 1 =>
            arm_2 : access constant thinkthen_request_json_value_map_v1;
         when 2 =>
            arm_3 : access constant thinkthen_request_json_value_list_v1;
         when others =>
            arm_4 : access constant thinkthen_request_null_v1;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_request_description_v1 is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_request_description_data_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_description_map_entry_v1 is record
      key : aliased thinkthen_complete_utf8_v1;
      value : access constant thinkthen_request_description_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_description_map_v1 is record
      data : access constant thinkthen_request_description_map_entry_v1;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_options_data_v1 (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            arm_1 : access constant thinkthen_request_name_list_v1;
         when others =>
            arm_2 : access constant thinkthen_request_description_map_v1;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_request_options_v1 is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_request_options_data_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_cut_data_v1 (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            arm_1 : access constant thinkthen_request_number_v1;
         when others =>
            arm_2 : access constant thinkthen_request_text_v1;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_request_cut_v1 is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_request_cut_data_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_definition_authored_choose_v1 is record
      m_batch : access constant thinkthen_request_definition_authored_score_batch_v1;
      m_choose : access constant thinkthen_request_question_text_v1;
      m_context_schema : access constant thinkthen_request_input_declaration_v1;
      m_item_schema : access constant thinkthen_request_input_declaration_v1;
      m_model : access constant thinkthen_request_name_v1;
      m_name : access constant thinkthen_request_text_v1;
      m_on : access constant thinkthen_request_pointers_v1;
      m_options : access constant thinkthen_request_options_v1;
      m_profile : access constant thinkthen_request_profile_v1;
      m_threshold : access constant thinkthen_request_cut_v1;
      m_wording_version : access constant thinkthen_request_integer_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_labels_data_v1 (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            arm_1 : access constant thinkthen_request_name_list_v1;
         when others =>
            arm_2 : access constant thinkthen_request_description_map_v1;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_request_labels_v1 is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_request_labels_data_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_definition_authored_tag_v1 is record
      m_batch : access constant thinkthen_request_definition_authored_score_batch_v1;
      m_context_schema : access constant thinkthen_request_input_declaration_v1;
      m_item_schema : access constant thinkthen_request_input_declaration_v1;
      m_labels : access constant thinkthen_request_labels_v1;
      m_model : access constant thinkthen_request_name_v1;
      m_name : access constant thinkthen_request_text_v1;
      m_on : access constant thinkthen_request_pointers_v1;
      m_profile : access constant thinkthen_request_profile_v1;
      m_tag : access constant thinkthen_request_question_text_v1;
      m_threshold : access constant thinkthen_request_cut_v1;
      m_wording_version : access constant thinkthen_request_integer_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_criterion_map_entry_v1 is record
      key : aliased thinkthen_complete_utf8_v1;
      value : access constant thinkthen_request_criterion_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_criterion_map_v1 is record
      data : access constant thinkthen_request_criterion_map_entry_v1;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_levels_data_v1 (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            arm_1 : access constant thinkthen_request_name_list_v1;
         when others =>
            arm_2 : access constant thinkthen_request_criterion_map_v1;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_request_levels_v1 is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_request_levels_data_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_definition_authored_score_v1 is record
      m_batch : access constant thinkthen_request_definition_authored_score_batch_v1;
      m_context_schema : access constant thinkthen_request_input_declaration_v1;
      m_item_schema : access constant thinkthen_request_input_declaration_v1;
      m_levels : access constant thinkthen_request_levels_v1;
      m_model : access constant thinkthen_request_name_v1;
      m_name : access constant thinkthen_request_text_v1;
      m_on : access constant thinkthen_request_pointers_v1;
      m_profile : access constant thinkthen_request_profile_v1;
      m_score : access constant thinkthen_request_question_text_v1;
      m_wording_version : access constant thinkthen_request_integer_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_relate_fields_v1 is record
      m_kind : access constant thinkthen_request_text_v1;
      m_name : access constant thinkthen_request_text_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_relation_v1 is record
      m_either : access constant thinkthen_request_boolean_v1;
      m_name : access constant thinkthen_request_name_v1;
      m_reads : access constant thinkthen_request_name_v1;
      m_single : access constant thinkthen_request_boolean_v1;
      m_source : access constant thinkthen_request_name_v1;
      m_target : access constant thinkthen_request_name_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_relation_list_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_relate_question_v1 is record
      m_fields : access constant thinkthen_request_relate_fields_v1;
      m_relations : access constant thinkthen_request_relation_list_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_definition_authored_relate_v1 is record
      m_context_schema : access constant thinkthen_request_input_declaration_v1;
      m_item_schema : access constant thinkthen_request_input_declaration_v1;
      m_model : access constant thinkthen_request_name_v1;
      m_name : access constant thinkthen_request_text_v1;
      m_profile : access constant thinkthen_request_profile_v1;
      m_relate : access constant thinkthen_request_relate_question_v1;
      m_threshold : access constant thinkthen_request_cut_v1;
      m_wording_version : access constant thinkthen_request_integer_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_definition_authored_find_v1 is record
      m_context_schema : access constant thinkthen_request_input_declaration_v1;
      m_find : access constant thinkthen_request_question_text_v1;
      m_item_schema : access constant thinkthen_request_input_declaration_v1;
      m_model : access constant thinkthen_request_name_v1;
      m_name : access constant thinkthen_request_text_v1;
      m_on : access constant thinkthen_request_pointers_v1;
      m_profile : access constant thinkthen_request_profile_v1;
      m_wording_version : access constant thinkthen_request_integer_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_recognize_question_v1 is record
      m_entity_definition : access constant thinkthen_request_question_text_v1;
      m_instructions : access constant thinkthen_request_question_text_v1;
      m_kinds : access constant thinkthen_request_description_map_v1;
      m_mode : access constant thinkthen_request_recognition_mode_v1;
      m_relations : access constant thinkthen_request_relation_list_v1;
      m_snippet_pieces : access constant thinkthen_request_unsigned_v1;
      m_stage_context : access constant thinkthen_request_recognition_stage_context_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_definition_recognize_definition_v1 is record
      m_context_schema : access constant thinkthen_request_input_declaration_v1;
      m_item_schema : access constant thinkthen_request_input_declaration_v1;
      m_model : access constant thinkthen_request_name_v1;
      m_name : access constant thinkthen_request_text_v1;
      m_on : access constant thinkthen_request_pointers_v1;
      m_profile : access constant thinkthen_request_profile_v1;
      m_recognize : access constant thinkthen_request_recognize_question_v1;
      m_relation_threshold : access constant thinkthen_request_cut_v1;
      m_threshold : access constant thinkthen_request_cut_v1;
      m_wording_version : access constant thinkthen_request_integer_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_annotate_question_decide_v1 is record
      m_context_schema : access constant thinkthen_request_input_declaration_v1;
      m_decide : access constant thinkthen_request_question_text_v1;
      m_false : access constant thinkthen_request_criterion_v1;
      m_item_schema : access constant thinkthen_request_input_declaration_v1;
      m_name : access constant thinkthen_request_text_v1;
      m_on : access constant thinkthen_request_pointers_v1;
      m_threshold : access constant thinkthen_request_threshold_v1;
      m_true : access constant thinkthen_request_criterion_v1;
      m_wording_version : access constant thinkthen_request_integer_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_annotate_question_choose_v1 is record
      m_choose : access constant thinkthen_request_question_text_v1;
      m_context_schema : access constant thinkthen_request_input_declaration_v1;
      m_item_schema : access constant thinkthen_request_input_declaration_v1;
      m_name : access constant thinkthen_request_text_v1;
      m_on : access constant thinkthen_request_pointers_v1;
      m_options : access constant thinkthen_request_options_v1;
      m_threshold : access constant thinkthen_request_cut_v1;
      m_wording_version : access constant thinkthen_request_integer_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_annotate_question_tag_v1 is record
      m_context_schema : access constant thinkthen_request_input_declaration_v1;
      m_item_schema : access constant thinkthen_request_input_declaration_v1;
      m_labels : access constant thinkthen_request_labels_v1;
      m_name : access constant thinkthen_request_text_v1;
      m_on : access constant thinkthen_request_pointers_v1;
      m_tag : access constant thinkthen_request_question_text_v1;
      m_threshold : access constant thinkthen_request_cut_v1;
      m_wording_version : access constant thinkthen_request_integer_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_annotate_question_score_v1 is record
      m_context_schema : access constant thinkthen_request_input_declaration_v1;
      m_item_schema : access constant thinkthen_request_input_declaration_v1;
      m_levels : access constant thinkthen_request_levels_v1;
      m_name : access constant thinkthen_request_text_v1;
      m_on : access constant thinkthen_request_pointers_v1;
      m_score : access constant thinkthen_request_question_text_v1;
      m_wording_version : access constant thinkthen_request_integer_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_annotate_question_data_v1 (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            arm_1 : access constant thinkthen_request_annotate_question_decide_v1;
         when 1 =>
            arm_2 : access constant thinkthen_request_annotate_question_choose_v1;
         when 2 =>
            arm_3 : access constant thinkthen_request_annotate_question_tag_v1;
         when others =>
            arm_4 : access constant thinkthen_request_annotate_question_score_v1;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_request_annotate_question_v1 is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_request_annotate_question_data_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_annotate_question_map_entry_v1 is record
      key : aliased thinkthen_complete_utf8_v1;
      value : access constant thinkthen_request_annotate_question_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_annotate_question_map_v1 is record
      data : access constant thinkthen_request_annotate_question_map_entry_v1;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_definition_annotate_definition_v1 is record
      m_batch : access constant thinkthen_request_json_value_v1;
      m_profile : access constant thinkthen_request_profile_v1;
      m_questions : access constant thinkthen_request_annotate_question_map_v1;
      m_threshold : access constant thinkthen_request_threshold_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_definition_data_v1 (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            arm_1 : access constant thinkthen_request_definition_authored_decide_v1;
         when 1 =>
            arm_2 : access constant thinkthen_request_definition_authored_choose_v1;
         when 2 =>
            arm_3 : access constant thinkthen_request_definition_authored_tag_v1;
         when 3 =>
            arm_4 : access constant thinkthen_request_definition_authored_score_v1;
         when 4 =>
            arm_5 : access constant thinkthen_request_definition_authored_relate_v1;
         when 5 =>
            arm_6 : access constant thinkthen_request_definition_authored_find_v1;
         when 6 =>
            arm_7 : access constant thinkthen_request_definition_recognize_definition_v1;
         when others =>
            arm_8 : access constant thinkthen_request_definition_annotate_definition_v1;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_request_definition_v1 is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_request_definition_data_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_question_selector_definition_v1 is record
      m_value : access constant thinkthen_request_definition_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_question_selector_file_v1 is record
      m_path : access constant thinkthen_request_text_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_question_selector_name_v1 is record
      m_name : access constant thinkthen_request_text_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_question_selector_reference_v1 is record
      m_reference : access constant thinkthen_request_text_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_question_selector_data_v1 (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            arm_1 : access constant thinkthen_request_question_selector_text_v1;
         when 1 =>
            arm_2 : access constant thinkthen_request_question_selector_definition_v1;
         when 2 =>
            arm_3 : access constant thinkthen_request_question_selector_file_v1;
         when 3 =>
            arm_4 : access constant thinkthen_request_question_selector_name_v1;
         when others =>
            arm_5 : access constant thinkthen_request_question_selector_reference_v1;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_request_question_selector_v1 is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_request_question_selector_data_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_arguments_v1 is record
      m_input : access constant thinkthen_request_input_v1;
      m_options : access constant thinkthen_request_call_options_v1;
      m_question : access constant thinkthen_request_question_selector_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_extension_v1 is record
      name : aliased thinkthen_complete_utf8_v1;
      json : aliased thinkthen_complete_utf8_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_extensions_v1 is record
      data : access constant thinkthen_complete_extension_v1;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answer_yes_no_v1 is record
      kind : aliased thinkthen_complete_utf8_v1;
      probability : aliased double;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_f64_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased double;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_number_map_entry_v1 is record
      name : aliased thinkthen_complete_utf8_v1;
      value : aliased double;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_number_map_v1 is record
      data : access constant thinkthen_complete_number_map_entry_v1;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answer_choice_v1 is record
      confidence : aliased thinkthen_complete_f64_presence_v1;
      kind : aliased thinkthen_complete_utf8_v1;
      pick : aliased thinkthen_complete_utf8_v1;
      probabilities : aliased thinkthen_complete_number_map_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answer_tag_v1 is record
      kind : aliased thinkthen_complete_utf8_v1;
      probabilities : aliased thinkthen_complete_number_map_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answer_score_v1 is record
      confidence : aliased thinkthen_complete_f64_presence_v1;
      kind : aliased thinkthen_complete_utf8_v1;
      level : aliased thinkthen_complete_utf8_v1;
      probabilities : aliased thinkthen_complete_number_map_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answer_data_v1 (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            yes_no : access constant thinkthen_complete_answer_yes_no_v1;
         when 1 =>
            choice : access constant thinkthen_complete_answer_choice_v1;
         when 2 =>
            tag : access constant thinkthen_complete_answer_tag_v1;
         when others =>
            score : access constant thinkthen_complete_answer_score_v1;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_answer_v1 is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_answer_data_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answer_id_v1 is record
      value : aliased thinkthen_complete_utf8_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_image_media_v1 is record
      kind : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_image_v1 is record
      base64 : aliased thinkthen_complete_utf8_v1;
      height : aliased Interfaces.Unsigned_64;
      media : access constant thinkthen_complete_image_media_v1;
      width : aliased Interfaces.Unsigned_64;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_image_list_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_image_list_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_image_list_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_u64_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased Interfaces.Unsigned_64;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_json_v1;
   type thinkthen_complete_json_array_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_json_entry_v1 is record
      name : aliased thinkthen_complete_utf8_v1;
      value : access constant thinkthen_complete_json_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_json_object_v1 is record
      data : access constant thinkthen_complete_json_entry_v1;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_json_data_v1 (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            boolean : aliased Interfaces.Unsigned_32;
         when 1 =>
            number : aliased thinkthen_complete_utf8_v1;
         when 2 =>
            string : aliased thinkthen_complete_utf8_v1;
         when 3 =>
            c_array : aliased thinkthen_complete_json_array_v1;
         when others =>
            object : aliased thinkthen_complete_json_object_v1;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_json_v1 is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_json_data_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_json_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_json_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_utf8_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_utf8_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_attempt_outcome_v1 is record
      kind : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_sdk_request_id_v1 is record
      value : aliased thinkthen_complete_utf8_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_attempt_v1 is record
      ordinal : aliased Interfaces.Unsigned_64;
      outcome : access constant thinkthen_complete_attempt_outcome_v1;
      request_id : aliased thinkthen_complete_utf8_presence_v1;
      request_sha256 : aliased thinkthen_complete_utf8_v1;
      sdk_request_id : access constant thinkthen_complete_sdk_request_id_v1;
      server_ms : aliased thinkthen_complete_u64_presence_v1;
      status : aliased thinkthen_complete_u64_presence_v1;
      wall_ms : aliased Interfaces.Unsigned_64;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_attempt_list_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_attempt_list_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_attempt_list_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_batch_setting_data_v1 (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            integer : aliased Interfaces.Unsigned_64;
         when others =>
            string : aliased thinkthen_complete_utf8_v1;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_batch_setting_v1 is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_batch_setting_data_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_batch_setting_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_batch_setting_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_batch_warning_v1 is record
      running : access constant thinkthen_complete_batch_setting_v1;
      tuned_for : access constant thinkthen_complete_batch_setting_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_batch_warning_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_batch_warning_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_observation_id_v1 is record
      value : aliased thinkthen_complete_utf8_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_observation_observation_id_v1 is record
      observation_id : access constant thinkthen_complete_observation_id_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_failure_id_v1 is record
      value : aliased thinkthen_complete_utf8_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_observation_failure_id_v1 is record
      failure_id : access constant thinkthen_complete_failure_id_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_observation_data_v1 (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            observation_id : access constant thinkthen_complete_observation_observation_id_v1;
         when others =>
            failure_id : access constant thinkthen_complete_observation_failure_id_v1;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_observation_v1 is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_observation_data_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_observation_list_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_origin_v1 is record
      kind : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_origin_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_origin_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_profile_warning_v1 is record
      running : aliased thinkthen_complete_utf8_v1;
      tuned_for : aliased thinkthen_complete_utf8_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_profile_warning_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_profile_warning_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_question_source_v1 is record
      answered_by : aliased thinkthen_complete_utf8_v1;
      batch_size : aliased thinkthen_complete_u64_presence_v1;
      origin : access constant thinkthen_complete_origin_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_question_source_list_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_string_list_v1 is record
      data : access constant thinkthen_complete_utf8_v1;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_usage_v1 is record
      input_tokens : aliased thinkthen_complete_u64_presence_v1;
      output_tokens : aliased thinkthen_complete_u64_presence_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_usage_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_usage_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_meta_v1 is record
      answered_by : aliased thinkthen_complete_utf8_presence_v1;
      attempts : aliased thinkthen_complete_attempt_list_presence_v1;
      batch_setting : aliased thinkthen_complete_batch_setting_presence_v1;
      batch_warning : aliased thinkthen_complete_batch_warning_presence_v1;
      cached : aliased Interfaces.Unsigned_32;
      context_sha256 : aliased thinkthen_complete_utf8_presence_v1;
      failed_questions : aliased Interfaces.Unsigned_64;
      model : aliased thinkthen_complete_utf8_v1;
      observations : aliased thinkthen_complete_observation_list_v1;
      origin : aliased thinkthen_complete_origin_presence_v1;
      profile_warning : aliased thinkthen_complete_profile_warning_presence_v1;
      question_sha256 : aliased thinkthen_complete_utf8_presence_v1;
      question_sources : aliased thinkthen_complete_question_source_list_v1;
      questions_sha256 : aliased thinkthen_complete_utf8_presence_v1;
      requests : aliased thinkthen_complete_string_list_v1;
      requests_sent : aliased Interfaces.Unsigned_64;
      tool : aliased thinkthen_complete_utf8_v1;
      url : aliased thinkthen_complete_utf8_v1;
      usage : aliased thinkthen_complete_usage_presence_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_batch_string_v1 is record
      kind : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_batch_data_v1 (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            integer : aliased Interfaces.Unsigned_64;
         when others =>
            string : access constant thinkthen_complete_batch_string_v1;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_batch_v1 is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_batch_data_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_batch_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_batch_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_string_type_v1 is record
      kind : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_input_declaration_string_root_v1 is record
      c_type : access constant thinkthen_complete_string_type_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_input_property_type_string_v1 is record
      c_type : aliased thinkthen_complete_utf8_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_input_property_type_number_v1 is record
      c_type : aliased thinkthen_complete_utf8_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_input_property_type_boolean_v1 is record
      c_type : aliased thinkthen_complete_utf8_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_string_root_v1 is record
      c_type : access constant thinkthen_complete_string_type_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_input_property_type_array_v1 is record
      items : access constant thinkthen_complete_string_root_v1;
      c_type : aliased thinkthen_complete_utf8_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_input_property_type_data_v1 (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            string : access constant thinkthen_complete_input_property_type_string_v1;
         when 1 =>
            number : access constant thinkthen_complete_input_property_type_number_v1;
         when 2 =>
            boolean : access constant thinkthen_complete_input_property_type_boolean_v1;
         when others =>
            c_array : access constant thinkthen_complete_input_property_type_array_v1;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_input_property_type_v1 is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_input_property_type_data_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_input_property_type_map_entry_v1 is record
      name : aliased thinkthen_complete_utf8_v1;
      value : access constant thinkthen_complete_input_property_type_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_input_property_type_map_v1 is record
      data : access constant thinkthen_complete_input_property_type_map_entry_v1;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_string_list_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_string_list_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_object_type_v1 is record
      kind : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_input_declaration_object_root_v1 is record
      properties : aliased thinkthen_complete_input_property_type_map_v1;
      required : aliased thinkthen_complete_string_list_presence_v1;
      c_type : access constant thinkthen_complete_object_type_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_input_declaration_data_v1 (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            string_root : access constant thinkthen_complete_input_declaration_string_root_v1;
         when others =>
            object_root : access constant thinkthen_complete_input_declaration_object_root_v1;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_input_declaration_v1 is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_input_declaration_data_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_input_declaration_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_input_declaration_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_label_v1 is record
      description : aliased thinkthen_complete_json_presence_v1;
      name : aliased thinkthen_complete_utf8_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_label_list_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_label_list_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_label_list_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_question_name_v1 is record
      value : aliased thinkthen_complete_utf8_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_question_name_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_question_name_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_wording_version_v1 is record
      value : aliased Interfaces.Unsigned_64;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_wording_version_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_wording_version_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_decide_v1 is record
      batch : aliased thinkthen_complete_batch_presence_v1;
      context_schema : aliased thinkthen_complete_input_declaration_presence_v1;
      item_schema : aliased thinkthen_complete_input_declaration_presence_v1;
      label_details : aliased thinkthen_complete_label_list_presence_v1;
      model : aliased thinkthen_complete_utf8_presence_v1;
      name : aliased thinkthen_complete_question_name_presence_v1;
      on : aliased thinkthen_complete_string_list_presence_v1;
      profile : aliased thinkthen_complete_utf8_presence_v1;
      wording_version : aliased thinkthen_complete_wording_version_presence_v1;
      false_u : aliased thinkthen_complete_json_presence_v1;
      text : aliased thinkthen_complete_json_presence_v1;
      true_u : aliased thinkthen_complete_json_presence_v1;
      verb : aliased thinkthen_complete_utf8_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_choose_v1 is record
      batch : aliased thinkthen_complete_batch_presence_v1;
      context_schema : aliased thinkthen_complete_input_declaration_presence_v1;
      item_schema : aliased thinkthen_complete_input_declaration_presence_v1;
      label_details : aliased thinkthen_complete_label_list_presence_v1;
      model : aliased thinkthen_complete_utf8_presence_v1;
      name : aliased thinkthen_complete_question_name_presence_v1;
      on : aliased thinkthen_complete_string_list_presence_v1;
      profile : aliased thinkthen_complete_utf8_presence_v1;
      wording_version : aliased thinkthen_complete_wording_version_presence_v1;
      options : aliased thinkthen_complete_string_list_v1;
      text : aliased thinkthen_complete_json_presence_v1;
      verb : aliased thinkthen_complete_utf8_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_tag_v1 is record
      batch : aliased thinkthen_complete_batch_presence_v1;
      context_schema : aliased thinkthen_complete_input_declaration_presence_v1;
      item_schema : aliased thinkthen_complete_input_declaration_presence_v1;
      label_details : aliased thinkthen_complete_label_list_presence_v1;
      model : aliased thinkthen_complete_utf8_presence_v1;
      name : aliased thinkthen_complete_question_name_presence_v1;
      on : aliased thinkthen_complete_string_list_presence_v1;
      profile : aliased thinkthen_complete_utf8_presence_v1;
      wording_version : aliased thinkthen_complete_wording_version_presence_v1;
      labels : aliased thinkthen_complete_string_list_v1;
      text : aliased thinkthen_complete_json_presence_v1;
      verb : aliased thinkthen_complete_utf8_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_score_v1 is record
      batch : aliased thinkthen_complete_batch_presence_v1;
      context_schema : aliased thinkthen_complete_input_declaration_presence_v1;
      item_schema : aliased thinkthen_complete_input_declaration_presence_v1;
      label_details : aliased thinkthen_complete_label_list_presence_v1;
      model : aliased thinkthen_complete_utf8_presence_v1;
      name : aliased thinkthen_complete_question_name_presence_v1;
      on : aliased thinkthen_complete_string_list_presence_v1;
      profile : aliased thinkthen_complete_utf8_presence_v1;
      wording_version : aliased thinkthen_complete_wording_version_presence_v1;
      levels : aliased thinkthen_complete_string_list_v1;
      text : aliased thinkthen_complete_json_presence_v1;
      verb : aliased thinkthen_complete_utf8_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_data_v1 (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            decide : access constant thinkthen_complete_readable_question_decide_v1;
         when 1 =>
            choose : access constant thinkthen_complete_readable_question_choose_v1;
         when 2 =>
            tag : access constant thinkthen_complete_readable_question_tag_v1;
         when others =>
            score : access constant thinkthen_complete_readable_question_score_v1;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_readable_question_v1 is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_readable_question_data_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_version_v1 is record
      kind : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_physical_source_v1 is record
      file : aliased thinkthen_complete_utf8_v1;
      first_line : aliased thinkthen_complete_u64_presence_v1;
      last_line : aliased thinkthen_complete_u64_presence_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_physical_source_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_physical_source_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_rank_member_result_v1 is record
      answer : access constant thinkthen_complete_answer_v1;
      answer_id : access constant thinkthen_complete_answer_id_v1;
      images : aliased thinkthen_complete_image_list_presence_v1;
      meta : access constant thinkthen_complete_meta_v1;
      question : access constant thinkthen_complete_readable_question_v1;
      schema : access constant thinkthen_complete_version_v1;
      source : aliased thinkthen_complete_physical_source_presence_v1;
      threshold : aliased thinkthen_complete_json_presence_v1;
      value : aliased Interfaces.Unsigned_64;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_rank_member_v1 is record
      name : aliased thinkthen_complete_utf8_v1;
      result : access constant thinkthen_complete_rank_member_result_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_rank_member_list_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_rank_member_list_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_rank_member_list_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_threshold_data_v1 (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            number : aliased double;
         when others =>
            string : aliased thinkthen_complete_utf8_v1;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_threshold_v1 is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_threshold_data_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_threshold_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_threshold_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_decide_value_v1 is record
      value : access constant thinkthen_complete_json_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_decide_value_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_decide_value_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_atomic_decide_value_v1 is record
      answer : access constant thinkthen_complete_answer_v1;
      answer_id : access constant thinkthen_complete_answer_id_v1;
      images : aliased thinkthen_complete_image_list_presence_v1;
      index : aliased thinkthen_complete_u64_presence_v1;
      input : aliased thinkthen_complete_json_presence_v1;
      members : aliased thinkthen_complete_rank_member_list_presence_v1;
      meta : access constant thinkthen_complete_meta_v1;
      question : access constant thinkthen_complete_readable_question_v1;
      question_name : aliased thinkthen_complete_utf8_presence_v1;
      schema : access constant thinkthen_complete_version_v1;
      source : aliased thinkthen_complete_physical_source_presence_v1;
      threshold : aliased thinkthen_complete_threshold_presence_v1;
      value : aliased thinkthen_complete_decide_value_presence_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_packet_decide_row_v1 is record
      c_function : aliased thinkthen_complete_utf8_v1;
      kind : aliased thinkthen_complete_utf8_v1;
      value : access constant thinkthen_complete_atomic_decide_value_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_atomic_nullable_string_v1 is record
      answer : access constant thinkthen_complete_answer_v1;
      answer_id : access constant thinkthen_complete_answer_id_v1;
      images : aliased thinkthen_complete_image_list_presence_v1;
      index : aliased thinkthen_complete_u64_presence_v1;
      input : aliased thinkthen_complete_json_presence_v1;
      members : aliased thinkthen_complete_rank_member_list_presence_v1;
      meta : access constant thinkthen_complete_meta_v1;
      question : access constant thinkthen_complete_readable_question_v1;
      question_name : aliased thinkthen_complete_utf8_presence_v1;
      schema : access constant thinkthen_complete_version_v1;
      source : aliased thinkthen_complete_physical_source_presence_v1;
      threshold : aliased thinkthen_complete_threshold_presence_v1;
      value : aliased thinkthen_complete_utf8_presence_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_packet_choose_row_v1 is record
      c_function : aliased thinkthen_complete_utf8_v1;
      kind : aliased thinkthen_complete_utf8_v1;
      value : access constant thinkthen_complete_atomic_nullable_string_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_atomic_array_of_string_v1 is record
      answer : access constant thinkthen_complete_answer_v1;
      answer_id : access constant thinkthen_complete_answer_id_v1;
      images : aliased thinkthen_complete_image_list_presence_v1;
      index : aliased thinkthen_complete_u64_presence_v1;
      input : aliased thinkthen_complete_json_presence_v1;
      members : aliased thinkthen_complete_rank_member_list_presence_v1;
      meta : access constant thinkthen_complete_meta_v1;
      question : access constant thinkthen_complete_readable_question_v1;
      question_name : aliased thinkthen_complete_utf8_presence_v1;
      schema : access constant thinkthen_complete_version_v1;
      source : aliased thinkthen_complete_physical_source_presence_v1;
      threshold : aliased thinkthen_complete_threshold_presence_v1;
      value : aliased thinkthen_complete_string_list_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_packet_tag_row_v1 is record
      c_function : aliased thinkthen_complete_utf8_v1;
      kind : aliased thinkthen_complete_utf8_v1;
      value : access constant thinkthen_complete_atomic_array_of_string_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_atomic_double_v1 is record
      answer : access constant thinkthen_complete_answer_v1;
      answer_id : access constant thinkthen_complete_answer_id_v1;
      images : aliased thinkthen_complete_image_list_presence_v1;
      index : aliased thinkthen_complete_u64_presence_v1;
      input : aliased thinkthen_complete_json_presence_v1;
      members : aliased thinkthen_complete_rank_member_list_presence_v1;
      meta : access constant thinkthen_complete_meta_v1;
      question : access constant thinkthen_complete_readable_question_v1;
      question_name : aliased thinkthen_complete_utf8_presence_v1;
      schema : access constant thinkthen_complete_version_v1;
      source : aliased thinkthen_complete_physical_source_presence_v1;
      threshold : aliased thinkthen_complete_threshold_presence_v1;
      value : aliased double;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_packet_score_row_v1 is record
      c_function : aliased thinkthen_complete_utf8_v1;
      kind : aliased thinkthen_complete_utf8_v1;
      value : access constant thinkthen_complete_atomic_double_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_atomic_boolean_v1 is record
      answer : access constant thinkthen_complete_answer_v1;
      answer_id : access constant thinkthen_complete_answer_id_v1;
      images : aliased thinkthen_complete_image_list_presence_v1;
      index : aliased thinkthen_complete_u64_presence_v1;
      input : aliased thinkthen_complete_json_presence_v1;
      members : aliased thinkthen_complete_rank_member_list_presence_v1;
      meta : access constant thinkthen_complete_meta_v1;
      question : access constant thinkthen_complete_readable_question_v1;
      question_name : aliased thinkthen_complete_utf8_presence_v1;
      schema : access constant thinkthen_complete_version_v1;
      source : aliased thinkthen_complete_physical_source_presence_v1;
      threshold : aliased thinkthen_complete_threshold_presence_v1;
      value : aliased Interfaces.Unsigned_32;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_packet_filter_row_v1 is record
      c_function : aliased thinkthen_complete_utf8_v1;
      kind : aliased thinkthen_complete_utf8_v1;
      value : access constant thinkthen_complete_atomic_boolean_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_value_data_v1 (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            boolean : aliased Interfaces.Unsigned_32;
         when 1 =>
            c_null : aliased Interfaces.Unsigned_32;
         when 2 =>
            string : aliased thinkthen_complete_utf8_v1;
         when 3 =>
            c_array : aliased thinkthen_complete_string_list_v1;
         when others =>
            number : aliased double;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_value_v1 is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_value_data_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_value_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_value_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_annotation_member_answer_id_v1 is record
      answer : access constant thinkthen_complete_answer_v1;
      answer_id : access constant thinkthen_complete_answer_id_v1;
      observations : aliased thinkthen_complete_observation_list_v1;
      question : access constant thinkthen_complete_readable_question_v1;
      question_sources : aliased thinkthen_complete_question_source_list_v1;
      request : aliased thinkthen_complete_utf8_v1;
      threshold : aliased thinkthen_complete_threshold_presence_v1;
      usage : aliased thinkthen_complete_usage_presence_v1;
      value : aliased thinkthen_complete_value_presence_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_failure_cause_v1 is record
      kind : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_failure_field_kind_v1 is record
      kind : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_failure_v1 is record
      cause : access constant thinkthen_complete_failure_cause_v1;
      kind : access constant thinkthen_complete_failure_field_kind_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_annotation_member_failure_id_v1 is record
      failure : access constant thinkthen_complete_failure_v1;
      failure_id : access constant thinkthen_complete_failure_id_v1;
      observations : aliased thinkthen_complete_observation_list_v1;
      question : access constant thinkthen_complete_readable_question_v1;
      question_sources : aliased thinkthen_complete_question_source_list_v1;
      request : aliased thinkthen_complete_utf8_v1;
      threshold : aliased thinkthen_complete_threshold_presence_v1;
      usage : aliased thinkthen_complete_usage_presence_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_annotation_member_data_v1 (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            answer_id : access constant thinkthen_complete_annotation_member_answer_id_v1;
         when others =>
            failure_id : access constant thinkthen_complete_annotation_member_failure_id_v1;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_annotation_member_v1 is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_annotation_member_data_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_annotation_member_map_entry_v1 is record
      name : aliased thinkthen_complete_utf8_v1;
      value : access constant thinkthen_complete_annotation_member_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_annotation_member_map_v1 is record
      data : access constant thinkthen_complete_annotation_member_map_entry_v1;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_position_v1 is record
      file : aliased thinkthen_complete_utf8_presence_v1;
      first : aliased thinkthen_complete_u64_presence_v1;
      images : aliased thinkthen_complete_string_list_presence_v1;
      last : aliased thinkthen_complete_u64_presence_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_position_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_position_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_failed_v1 is record
      failed : access constant thinkthen_complete_failure_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_annotated_field_data_v1 (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            boolean : aliased Interfaces.Unsigned_32;
         when 1 =>
            c_null : aliased Interfaces.Unsigned_32;
         when 2 =>
            string : aliased thinkthen_complete_utf8_v1;
         when 3 =>
            c_array : aliased thinkthen_complete_string_list_v1;
         when 4 =>
            number : aliased double;
         when others =>
            object : access constant thinkthen_complete_failed_v1;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_annotated_field_v1 is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_annotated_field_data_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_annotated_field_map_entry_v1 is record
      name : aliased thinkthen_complete_utf8_v1;
      value : access constant thinkthen_complete_annotated_field_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_annotated_field_map_v1 is record
      data : access constant thinkthen_complete_annotated_field_map_entry_v1;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_annotated_row_v1 is record
      value : aliased thinkthen_complete_annotated_field_map_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_annotation_v1 is record
      answer_id : access constant thinkthen_complete_answer_id_v1;
      answers : aliased thinkthen_complete_annotation_member_map_v1;
      file : aliased thinkthen_complete_utf8_presence_v1;
      first_line : aliased thinkthen_complete_u64_presence_v1;
      index : aliased thinkthen_complete_u64_presence_v1;
      input : aliased thinkthen_complete_json_presence_v1;
      last_line : aliased thinkthen_complete_u64_presence_v1;
      meta : access constant thinkthen_complete_meta_v1;
      position : aliased thinkthen_complete_position_presence_v1;
      schema : access constant thinkthen_complete_version_v1;
      source : aliased thinkthen_complete_physical_source_presence_v1;
      value : access constant thinkthen_complete_annotated_row_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_packet_annotate_row_v1 is record
      c_function : aliased thinkthen_complete_utf8_v1;
      kind : aliased thinkthen_complete_utf8_v1;
      value : access constant thinkthen_complete_annotation_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_atomic_decide_value_list_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_packet_decide_aggregate_v1 is record
      c_function : aliased thinkthen_complete_utf8_v1;
      kind : aliased thinkthen_complete_utf8_v1;
      value : aliased thinkthen_complete_atomic_decide_value_list_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_atomic_nullable_string_list_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_packet_choose_aggregate_v1 is record
      c_function : aliased thinkthen_complete_utf8_v1;
      kind : aliased thinkthen_complete_utf8_v1;
      value : aliased thinkthen_complete_atomic_nullable_string_list_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_atomic_array_of_string_list_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_packet_tag_aggregate_v1 is record
      c_function : aliased thinkthen_complete_utf8_v1;
      kind : aliased thinkthen_complete_utf8_v1;
      value : aliased thinkthen_complete_atomic_array_of_string_list_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_atomic_double_list_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_packet_score_aggregate_v1 is record
      c_function : aliased thinkthen_complete_utf8_v1;
      kind : aliased thinkthen_complete_utf8_v1;
      value : aliased thinkthen_complete_atomic_double_list_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_atomic_boolean_list_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_packet_filter_aggregate_v1 is record
      c_function : aliased thinkthen_complete_utf8_v1;
      kind : aliased thinkthen_complete_utf8_v1;
      value : aliased thinkthen_complete_atomic_boolean_list_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_atomic_non_zero_usize_v1 is record
      answer : access constant thinkthen_complete_answer_v1;
      answer_id : access constant thinkthen_complete_answer_id_v1;
      images : aliased thinkthen_complete_image_list_presence_v1;
      index : aliased thinkthen_complete_u64_presence_v1;
      input : aliased thinkthen_complete_json_presence_v1;
      members : aliased thinkthen_complete_rank_member_list_presence_v1;
      meta : access constant thinkthen_complete_meta_v1;
      question : access constant thinkthen_complete_readable_question_v1;
      question_name : aliased thinkthen_complete_utf8_presence_v1;
      schema : access constant thinkthen_complete_version_v1;
      source : aliased thinkthen_complete_physical_source_presence_v1;
      threshold : aliased thinkthen_complete_threshold_presence_v1;
      value : aliased Interfaces.Unsigned_64;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_atomic_non_zero_usize_list_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_packet_rank_aggregate_v1 is record
      c_function : aliased thinkthen_complete_utf8_v1;
      kind : aliased thinkthen_complete_utf8_v1;
      value : aliased thinkthen_complete_atomic_non_zero_usize_list_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_find_answer_v1 is record
      confidence : aliased thinkthen_complete_f64_presence_v1;
      kind : aliased thinkthen_complete_utf8_v1;
      pick : aliased thinkthen_complete_utf8_v1;
      probabilities : aliased thinkthen_complete_number_map_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_find_candidate_v1 is record
      index : aliased thinkthen_complete_u64_presence_v1;
      input : aliased thinkthen_complete_json_presence_v1;
      probability : aliased double;
      source : aliased thinkthen_complete_physical_source_presence_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_find_candidate_list_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_find_candidate_list_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_find_candidate_list_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_2_v1 is record
      batch : aliased thinkthen_complete_batch_presence_v1;
      context_schema : aliased thinkthen_complete_input_declaration_presence_v1;
      item_schema : aliased thinkthen_complete_input_declaration_presence_v1;
      label_details : aliased thinkthen_complete_label_list_presence_v1;
      model : aliased thinkthen_complete_utf8_presence_v1;
      name : aliased thinkthen_complete_question_name_presence_v1;
      none : aliased Interfaces.Unsigned_32;
      on : aliased thinkthen_complete_string_list_presence_v1;
      profile : aliased thinkthen_complete_utf8_presence_v1;
      text : aliased thinkthen_complete_json_presence_v1;
      verb : aliased thinkthen_complete_utf8_v1;
      wording_version : aliased thinkthen_complete_wording_version_presence_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_find_v1 is record
      answer : access constant thinkthen_complete_find_answer_v1;
      answer_id : access constant thinkthen_complete_answer_id_v1;
      candidates : aliased thinkthen_complete_find_candidate_list_presence_v1;
      file : aliased thinkthen_complete_utf8_presence_v1;
      first_line : aliased thinkthen_complete_u64_presence_v1;
      index : aliased thinkthen_complete_u64_presence_v1;
      last_line : aliased thinkthen_complete_u64_presence_v1;
      meta : access constant thinkthen_complete_meta_v1;
      position : aliased thinkthen_complete_position_presence_v1;
      question : access constant thinkthen_complete_readable_question_2_v1;
      schema : access constant thinkthen_complete_version_v1;
      threshold : aliased thinkthen_complete_json_presence_v1;
      value : aliased thinkthen_complete_json_presence_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_packet_find_aggregate_v1 is record
      c_function : aliased thinkthen_complete_utf8_v1;
      kind : aliased thinkthen_complete_utf8_v1;
      value : access constant thinkthen_complete_find_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_annotation_list_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_packet_annotate_aggregate_v1 is record
      c_function : aliased thinkthen_complete_utf8_v1;
      kind : aliased thinkthen_complete_utf8_v1;
      value : aliased thinkthen_complete_annotation_list_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_number_map_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_number_map_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_name_odds_v1 is record
      edges : aliased thinkthen_complete_number_map_presence_v1;
      c_end : aliased Interfaces.Unsigned_64;
      kinds : aliased thinkthen_complete_number_map_presence_v1;
      start : aliased Interfaces.Unsigned_64;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_name_odds_list_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_place_v1 is record
      c_end : aliased Interfaces.Unsigned_64;
      start : aliased Interfaces.Unsigned_64;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_pair_odds_v1 is record
      probability : aliased double;
      relation : aliased thinkthen_complete_utf8_v1;
      source : access constant thinkthen_complete_place_v1;
      target : access constant thinkthen_complete_place_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_pair_odds_list_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_piece_odds_v1 is record
      c_end : aliased Interfaces.Unsigned_64;
      start : aliased Interfaces.Unsigned_64;
      tags : aliased thinkthen_complete_number_map_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_piece_odds_list_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_place_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_place_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_recognition_proposal_v1 is record
      c_end : aliased Interfaces.Unsigned_64;
      kept : aliased Interfaces.Unsigned_32;
      kind : aliased thinkthen_complete_utf8_presence_v1;
      selected : aliased thinkthen_complete_place_presence_v1;
      span_probability : aliased double;
      start : aliased Interfaces.Unsigned_64;
      strength : aliased thinkthen_complete_f64_presence_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_recognition_proposal_list_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_recognition_odds_recognize_answer_v1 is record
      names : aliased thinkthen_complete_name_odds_list_v1;
      pairs : aliased thinkthen_complete_pair_odds_list_v1;
      pieces : aliased thinkthen_complete_piece_odds_list_v1;
      proposals : aliased thinkthen_complete_recognition_proposal_list_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_boundary_proposal_v1 is record
      c_end : aliased Interfaces.Unsigned_64;
      length : aliased Interfaces.Unsigned_64;
      probability : aliased double;
      start : aliased Interfaces.Unsigned_64;
      text : aliased thinkthen_complete_utf8_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_boundary_proposal_list_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_recognition_odds_boundary_odds_v1 is record
      pieces : aliased thinkthen_complete_piece_odds_list_v1;
      proposals : aliased thinkthen_complete_boundary_proposal_list_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_recognition_odds_data_v1 (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            recognize_answer : access constant thinkthen_complete_recognition_odds_recognize_answer_v1;
         when others =>
            boundary_odds : access constant thinkthen_complete_recognition_odds_boundary_odds_v1;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_recognition_odds_v1 is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_recognition_odds_data_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_json_map_entry_v1 is record
      name : aliased thinkthen_complete_utf8_v1;
      value : access constant thinkthen_complete_json_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_json_map_v1 is record
      data : access constant thinkthen_complete_json_map_entry_v1;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_recognition_mode_v1 is record
      kind : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_recognition_mode_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_recognition_mode_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_u32_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_relation_rule_v1 is record
      either : aliased Interfaces.Unsigned_32;
      name : aliased thinkthen_complete_utf8_v1;
      reads : aliased thinkthen_complete_utf8_v1;
      single : aliased thinkthen_complete_u32_presence_v1;
      source : aliased thinkthen_complete_utf8_v1;
      target : aliased thinkthen_complete_utf8_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_relation_rule_list_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_relation_rule_list_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_relation_rule_list_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_recognition_stage_context_v1 is record
      boundary : aliased thinkthen_complete_utf8_presence_v1;
      kind_edge : aliased thinkthen_complete_utf8_presence_v1;
      relation : aliased thinkthen_complete_utf8_presence_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_recognition_stage_context_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_recognition_stage_context_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_verb_v1 is record
      kind : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_3_v1 is record
      batch : aliased thinkthen_complete_batch_presence_v1;
      context_schema : aliased thinkthen_complete_input_declaration_presence_v1;
      entity_definition : aliased thinkthen_complete_json_presence_v1;
      instructions : aliased thinkthen_complete_json_presence_v1;
      item_schema : aliased thinkthen_complete_input_declaration_presence_v1;
      kinds : aliased thinkthen_complete_json_map_v1;
      label_details : aliased thinkthen_complete_label_list_presence_v1;
      mode : aliased thinkthen_complete_recognition_mode_presence_v1;
      model : aliased thinkthen_complete_utf8_presence_v1;
      name : aliased thinkthen_complete_question_name_presence_v1;
      on : aliased thinkthen_complete_string_list_presence_v1;
      profile : aliased thinkthen_complete_utf8_presence_v1;
      relation_threshold : aliased thinkthen_complete_threshold_presence_v1;
      relations : aliased thinkthen_complete_relation_rule_list_presence_v1;
      snippet_pieces : aliased thinkthen_complete_u64_presence_v1;
      stage_context : aliased thinkthen_complete_recognition_stage_context_presence_v1;
      threshold : access constant thinkthen_complete_threshold_v1;
      verb : access constant thinkthen_complete_verb_v1;
      wording_version : aliased thinkthen_complete_wording_version_presence_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_entity_v1 is record
      c_end : aliased Interfaces.Unsigned_64;
      file : aliased thinkthen_complete_utf8_presence_v1;
      first_line : aliased thinkthen_complete_u64_presence_v1;
      kind : aliased thinkthen_complete_utf8_v1;
      last_line : aliased thinkthen_complete_u64_presence_v1;
      length : aliased Interfaces.Unsigned_64;
      start : aliased Interfaces.Unsigned_64;
      strength : aliased double;
      text : aliased thinkthen_complete_utf8_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_entity_list_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_entity_edge_v1 is record
      either : aliased thinkthen_complete_u32_presence_v1;
      probability : aliased double;
      relation : aliased thinkthen_complete_utf8_v1;
      source : access constant thinkthen_complete_entity_v1;
      target : access constant thinkthen_complete_entity_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_entity_edge_list_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_entity_edge_list_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_entity_edge_list_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_recognize_entities_v1 is record
      entities : aliased thinkthen_complete_entity_list_v1;
      relations : aliased thinkthen_complete_entity_edge_list_presence_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_boundary_mode_v1 is record
      kind : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_recognize_mode_proposals_v1 is record
      mode : access constant thinkthen_complete_boundary_mode_v1;
      proposals : aliased thinkthen_complete_boundary_proposal_list_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_recognize_data_v1 (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            entities : access constant thinkthen_complete_recognize_entities_v1;
         when others =>
            mode_proposals : access constant thinkthen_complete_recognize_mode_proposals_v1;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_recognize_v1 is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_recognize_data_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_recognition_v1 is record
      answer : access constant thinkthen_complete_recognition_odds_v1;
      answer_id : access constant thinkthen_complete_answer_id_v1;
      file : aliased thinkthen_complete_utf8_presence_v1;
      first_line : aliased thinkthen_complete_u64_presence_v1;
      index : aliased thinkthen_complete_u64_presence_v1;
      input : aliased thinkthen_complete_json_presence_v1;
      last_line : aliased thinkthen_complete_u64_presence_v1;
      meta : access constant thinkthen_complete_meta_v1;
      position : aliased thinkthen_complete_position_presence_v1;
      question : access constant thinkthen_complete_readable_question_3_v1;
      schema : access constant thinkthen_complete_version_v1;
      source : aliased thinkthen_complete_physical_source_presence_v1;
      value : access constant thinkthen_complete_recognize_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_recognition_list_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_packet_recognize_aggregate_v1 is record
      c_function : aliased thinkthen_complete_utf8_v1;
      kind : aliased thinkthen_complete_utf8_v1;
      value : aliased thinkthen_complete_recognition_list_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_relation_direction_v1 is record
      kind : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_relation_method_v1 is record
      kind : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_related_entity_v1 is record
      kind : aliased thinkthen_complete_utf8_v1;
      name : aliased thinkthen_complete_utf8_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_related_entity_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_related_entity_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_relation_member_answer_id_v1 is record
      direction : access constant thinkthen_complete_relation_direction_v1;
      method : access constant thinkthen_complete_relation_method_v1;
      observations : aliased thinkthen_complete_observation_list_v1;
      question : access constant thinkthen_complete_readable_question_v1;
      question_sources : aliased thinkthen_complete_question_source_list_v1;
      reads : aliased thinkthen_complete_utf8_v1;
      relation : aliased thinkthen_complete_utf8_v1;
      request : aliased thinkthen_complete_utf8_v1;
      source : access constant thinkthen_complete_related_entity_v1;
      target : aliased thinkthen_complete_related_entity_presence_v1;
      threshold : access constant thinkthen_complete_threshold_v1;
      usage : aliased thinkthen_complete_usage_presence_v1;
      accepted : aliased Interfaces.Unsigned_32;
      answer : access constant thinkthen_complete_answer_v1;
      answer_id : access constant thinkthen_complete_answer_id_v1;
      probability : aliased double;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_relation_member_failure_id_v1 is record
      direction : access constant thinkthen_complete_relation_direction_v1;
      method : access constant thinkthen_complete_relation_method_v1;
      observations : aliased thinkthen_complete_observation_list_v1;
      question : access constant thinkthen_complete_readable_question_v1;
      question_sources : aliased thinkthen_complete_question_source_list_v1;
      reads : aliased thinkthen_complete_utf8_v1;
      relation : aliased thinkthen_complete_utf8_v1;
      request : aliased thinkthen_complete_utf8_v1;
      source : access constant thinkthen_complete_related_entity_v1;
      target : aliased thinkthen_complete_related_entity_presence_v1;
      threshold : access constant thinkthen_complete_threshold_v1;
      usage : aliased thinkthen_complete_usage_presence_v1;
      failure : access constant thinkthen_complete_failure_v1;
      failure_id : access constant thinkthen_complete_failure_id_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_relation_member_data_v1 (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            answer_id : access constant thinkthen_complete_relation_member_answer_id_v1;
         when others =>
            failure_id : access constant thinkthen_complete_relation_member_failure_id_v1;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_relation_member_v1 is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_relation_member_data_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_relation_member_list_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answers_v1 is record
      questions : aliased thinkthen_complete_relation_member_list_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_input_source_v1 is record
      index : aliased Interfaces.Unsigned_64;
      source : access constant thinkthen_complete_physical_source_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_input_source_list_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_input_source_list_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_session_input_source_list_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_relate_fields_v1 is record
      kind : aliased thinkthen_complete_utf8_v1;
      name : aliased thinkthen_complete_utf8_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_relate_fields_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_relate_fields_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_4_v1 is record
      batch : aliased thinkthen_complete_batch_presence_v1;
      context_schema : aliased thinkthen_complete_input_declaration_presence_v1;
      fields : aliased thinkthen_complete_relate_fields_presence_v1;
      item_schema : aliased thinkthen_complete_input_declaration_presence_v1;
      label_details : aliased thinkthen_complete_label_list_presence_v1;
      model : aliased thinkthen_complete_utf8_presence_v1;
      name : aliased thinkthen_complete_question_name_presence_v1;
      on : aliased thinkthen_complete_string_list_presence_v1;
      profile : aliased thinkthen_complete_utf8_presence_v1;
      relations : aliased thinkthen_complete_relation_rule_list_v1;
      threshold : access constant thinkthen_complete_threshold_v1;
      verb : aliased thinkthen_complete_utf8_v1;
      wording_version : aliased thinkthen_complete_wording_version_presence_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_relation_endpoint_related_entity_v1 is record
      kind : aliased thinkthen_complete_utf8_v1;
      name : aliased thinkthen_complete_utf8_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_relation_endpoint_source_v1 is record
      file : aliased thinkthen_complete_utf8_presence_v1;
      first_line : aliased thinkthen_complete_u64_presence_v1;
      kind : aliased thinkthen_complete_utf8_v1;
      last_line : aliased thinkthen_complete_u64_presence_v1;
      name : aliased thinkthen_complete_utf8_v1;
      ordinal : aliased Interfaces.Unsigned_64;
      c_record : aliased thinkthen_complete_json_presence_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_relation_endpoint_data_v1 (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            related_entity : access constant thinkthen_complete_relation_endpoint_related_entity_v1;
         when others =>
            source : access constant thinkthen_complete_relation_endpoint_source_v1;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_relation_endpoint_v1 is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_relation_endpoint_data_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_related_entity_edge_v1 is record
      either : aliased thinkthen_complete_u32_presence_v1;
      probability : aliased double;
      relation : aliased thinkthen_complete_utf8_v1;
      source : access constant thinkthen_complete_relation_endpoint_v1;
      target : access constant thinkthen_complete_relation_endpoint_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_related_entity_edge_list_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_relation_v1 is record
      answer : access constant thinkthen_complete_answers_v1;
      answer_id : access constant thinkthen_complete_answer_id_v1;
      file : aliased thinkthen_complete_utf8_presence_v1;
      first_line : aliased thinkthen_complete_u64_presence_v1;
      index : aliased thinkthen_complete_u64_presence_v1;
      input : aliased thinkthen_complete_json_presence_v1;
      input_sources : aliased thinkthen_complete_session_input_source_list_presence_v1;
      last_line : aliased thinkthen_complete_u64_presence_v1;
      meta : access constant thinkthen_complete_meta_v1;
      position : aliased thinkthen_complete_position_presence_v1;
      question : access constant thinkthen_complete_readable_question_4_v1;
      schema : access constant thinkthen_complete_version_v1;
      value : aliased thinkthen_complete_related_entity_edge_list_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_packet_relate_aggregate_v1 is record
      c_function : aliased thinkthen_complete_utf8_v1;
      kind : aliased thinkthen_complete_utf8_v1;
      value : access constant thinkthen_complete_relation_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_request_function_v1 is record
      kind : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answer_id_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_answer_id_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_failure_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_failure_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_failure_id_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_failure_id_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_json_list_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_probabilities_yes_no_v1 is record
      kind : aliased thinkthen_complete_utf8_v1;
      value : aliased double;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_named_probability_v1 is record
      name : aliased thinkthen_complete_utf8_v1;
      probability : aliased double;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_named_probability_list_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_probabilities_named_v1 is record
      kind : aliased thinkthen_complete_utf8_v1;
      value : aliased thinkthen_complete_session_named_probability_list_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_probabilities_data_v1 (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            yes_no : access constant thinkthen_complete_session_probabilities_yes_no_v1;
         when others =>
            named : access constant thinkthen_complete_session_probabilities_named_v1;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_session_probabilities_v1 is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_session_probabilities_data_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_probabilities_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_session_probabilities_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_token_usage_v1 is record
      input_tokens : aliased Interfaces.Unsigned_64;
      output_tokens : aliased Interfaces.Unsigned_64;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_token_usage_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_token_usage_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_question_detail_v1 is record
      answer_id : aliased thinkthen_complete_answer_id_presence_v1;
      cached : aliased Interfaces.Unsigned_32;
      confidence : aliased thinkthen_complete_f64_presence_v1;
      failed_questions : aliased Interfaces.Unsigned_64;
      failure : aliased thinkthen_complete_failure_presence_v1;
      failure_id : aliased thinkthen_complete_failure_id_presence_v1;
      input : aliased thinkthen_complete_json_presence_v1;
      input_source : aliased thinkthen_complete_physical_source_presence_v1;
      input_sources : aliased thinkthen_complete_session_input_source_list_v1;
      inputs : aliased thinkthen_complete_json_list_v1;
      model : aliased thinkthen_complete_utf8_v1;
      observations : aliased thinkthen_complete_observation_list_v1;
      probabilities : aliased thinkthen_complete_session_probabilities_presence_v1;
      question : access constant thinkthen_complete_readable_question_v1;
      question_sha256 : aliased thinkthen_complete_utf8_v1;
      question_sources : aliased thinkthen_complete_question_source_list_v1;
      raw_pick : aliased thinkthen_complete_utf8_presence_v1;
      reported_usage : aliased thinkthen_complete_usage_presence_v1;
      requests : aliased thinkthen_complete_string_list_v1;
      requests_sent : aliased Interfaces.Unsigned_64;
      threshold : aliased thinkthen_complete_threshold_presence_v1;
      url : aliased thinkthen_complete_utf8_v1;
      usage : aliased thinkthen_complete_token_usage_presence_v1;
      value : aliased thinkthen_complete_value_presence_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_observation_question_v1 is record
      detail : access constant thinkthen_complete_session_question_detail_v1;
      index : aliased Interfaces.Unsigned_64;
      kind : aliased thinkthen_complete_utf8_v1;
      member : aliased thinkthen_complete_utf8_presence_v1;
      position : aliased Interfaces.Unsigned_64;
      stage : aliased thinkthen_complete_utf8_presence_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_judgment_decision_v1 is record
      kind : aliased thinkthen_complete_utf8_v1;
      value : aliased thinkthen_complete_u32_presence_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_judgment_choice_v1 is record
      kind : aliased thinkthen_complete_utf8_v1;
      value : aliased thinkthen_complete_utf8_presence_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_judgment_score_v1 is record
      kind : aliased thinkthen_complete_utf8_v1;
      value : aliased double;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_judgment_tags_v1 is record
      kind : aliased thinkthen_complete_utf8_v1;
      value : aliased thinkthen_complete_string_list_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_judgment_data_v1 (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            decision : access constant thinkthen_complete_session_judgment_decision_v1;
         when 1 =>
            choice : access constant thinkthen_complete_session_judgment_choice_v1;
         when 2 =>
            score : access constant thinkthen_complete_session_judgment_score_v1;
         when others =>
            tags : access constant thinkthen_complete_session_judgment_tags_v1;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_session_judgment_v1 is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_session_judgment_data_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_observed_row_judgment_v1 is record
      kind : aliased thinkthen_complete_utf8_v1;
      value : access constant thinkthen_complete_session_judgment_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_annotation_value_decision_v1 is record
      kind : aliased thinkthen_complete_utf8_v1;
      value : aliased thinkthen_complete_u32_presence_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_annotation_value_choice_v1 is record
      kind : aliased thinkthen_complete_utf8_v1;
      value : aliased thinkthen_complete_utf8_presence_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_annotation_value_score_v1 is record
      kind : aliased thinkthen_complete_utf8_v1;
      value : aliased double;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_annotation_value_tags_v1 is record
      kind : aliased thinkthen_complete_utf8_v1;
      value : aliased thinkthen_complete_string_list_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_annotation_value_failed_v1 is record
      kind : aliased thinkthen_complete_utf8_v1;
      value : access constant thinkthen_complete_failure_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_annotation_value_data_v1 (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            decision : access constant thinkthen_complete_annotation_value_decision_v1;
         when 1 =>
            choice : access constant thinkthen_complete_annotation_value_choice_v1;
         when 2 =>
            score : access constant thinkthen_complete_annotation_value_score_v1;
         when 3 =>
            tags : access constant thinkthen_complete_annotation_value_tags_v1;
         when others =>
            failed : access constant thinkthen_complete_annotation_value_failed_v1;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_annotation_value_v1 is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_annotation_value_data_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_annotation_v1 is record
      name : aliased thinkthen_complete_utf8_v1;
      value : access constant thinkthen_complete_annotation_value_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_annotation_list_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_observed_row_annotated_v1 is record
      kind : aliased thinkthen_complete_utf8_v1;
      value : aliased thinkthen_complete_session_annotation_list_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_boundary_proposal_list_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_boundary_proposal_list_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_recognition_edge_v1 is record
      either : aliased Interfaces.Unsigned_32;
      probability : aliased double;
      relation : aliased thinkthen_complete_utf8_v1;
      source : access constant thinkthen_complete_entity_v1;
      target : access constant thinkthen_complete_entity_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_recognition_edge_list_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_recognition_edge_list_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_recognition_edge_list_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_recognition_v1 is record
      entities : aliased thinkthen_complete_entity_list_v1;
      mode : access constant thinkthen_complete_recognition_mode_v1;
      proposals : aliased thinkthen_complete_boundary_proposal_list_presence_v1;
      relations : aliased thinkthen_complete_recognition_edge_list_presence_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_observed_row_recognized_v1 is record
      kind : aliased thinkthen_complete_utf8_v1;
      value : access constant thinkthen_complete_session_recognition_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_observed_row_find_v1 is record
      kind : aliased thinkthen_complete_utf8_v1;
      value : aliased thinkthen_complete_u64_presence_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_entity_document_v1 is record
      kind : aliased thinkthen_complete_utf8_v1;
      name : aliased thinkthen_complete_utf8_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_relation_edge_v1 is record
      either : aliased Interfaces.Unsigned_32;
      probability : aliased double;
      relation : aliased thinkthen_complete_utf8_v1;
      source : access constant thinkthen_complete_entity_document_v1;
      target : access constant thinkthen_complete_entity_document_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_relation_edge_list_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_observed_row_relations_v1 is record
      kind : aliased thinkthen_complete_utf8_v1;
      value : aliased thinkthen_complete_session_relation_edge_list_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_observed_row_data_v1 (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            judgment : access constant thinkthen_complete_session_observed_row_judgment_v1;
         when 1 =>
            annotated : access constant thinkthen_complete_session_observed_row_annotated_v1;
         when 2 =>
            recognized : access constant thinkthen_complete_session_observed_row_recognized_v1;
         when 3 =>
            find : access constant thinkthen_complete_session_observed_row_find_v1;
         when others =>
            relations : access constant thinkthen_complete_session_observed_row_relations_v1;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_session_observed_row_v1 is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_session_observed_row_data_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_observation_row_v1 is record
      index : aliased Interfaces.Unsigned_64;
      kind : aliased thinkthen_complete_utf8_v1;
      value : access constant thinkthen_complete_session_observed_row_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_observation_data_v1 (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            question : access constant thinkthen_complete_session_observation_question_v1;
         when others =>
            row : access constant thinkthen_complete_session_observation_row_v1;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_session_observation_v1 is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_session_observation_data_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_packet_observation_v1 is record
      c_function : access constant thinkthen_complete_request_function_v1;
      kind : aliased thinkthen_complete_utf8_v1;
      value : access constant thinkthen_complete_session_observation_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_call_id_v1 is record
      value : aliased thinkthen_complete_utf8_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_persistence_observation_v1 is record
      advice : aliased thinkthen_complete_utf8_presence_v1;
      observed_at : aliased thinkthen_complete_utf8_v1;
      state : access constant thinkthen_complete_usage_persistence_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_persistence_observation_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_persistence_observation_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_facts_v1 is record
      attempts : aliased thinkthen_complete_attempt_list_presence_v1;
      cache_answers : aliased Interfaces.Unsigned_64;
      call_id : access constant thinkthen_complete_call_id_v1;
      estimated_cost_usd : aliased thinkthen_complete_utf8_presence_v1;
      held_model_mismatch : aliased thinkthen_complete_u32_presence_v1;
      input_tokens : aliased thinkthen_complete_u64_presence_v1;
      largest_request_bytes : aliased Interfaces.Unsigned_64;
      largest_request_estimated_input_tokens : aliased thinkthen_complete_u64_presence_v1;
      model : aliased thinkthen_complete_utf8_presence_v1;
      output_tokens : aliased thinkthen_complete_u64_presence_v1;
      records : aliased Interfaces.Unsigned_64;
      requests_sent : aliased Interfaces.Unsigned_64;
      seconds : aliased double;
      token_estimate_method : aliased thinkthen_complete_utf8_v1;
      usage_persistence : aliased thinkthen_complete_persistence_observation_presence_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_facts_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_facts_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_input_denial_initial_request_v1 is record
      kind : aliased thinkthen_complete_utf8_v1;
      limit : aliased Interfaces.Unsigned_64;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_input_denial_additional_request_v1 is record
      kind : aliased thinkthen_complete_utf8_v1;
      limit : aliased Interfaces.Unsigned_64;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_input_denial_retry_v1 is record
      kind : aliased thinkthen_complete_utf8_v1;
      last_status : aliased Interfaces.Unsigned_64;
      limit : aliased Interfaces.Unsigned_64;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_input_denial_data_v1 (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            initial_request : access constant thinkthen_complete_input_denial_initial_request_v1;
         when 1 =>
            additional_request : access constant thinkthen_complete_input_denial_additional_request_v1;
         when others =>
            retry : access constant thinkthen_complete_input_denial_retry_v1;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_input_denial_v1 is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_input_denial_data_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_input_denial_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_input_denial_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_failure_kind_v1 is record
      kind : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_send_denial_before_first_send_v1 is record
      kind : aliased thinkthen_complete_utf8_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_send_denial_before_additional_send_v1 is record
      kind : aliased thinkthen_complete_utf8_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_send_denial_before_retry_v1 is record
      kind : aliased thinkthen_complete_utf8_v1;
      last_status : aliased Interfaces.Unsigned_64;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_send_denial_data_v1 (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            before_first_send : access constant thinkthen_complete_send_denial_before_first_send_v1;
         when 1 =>
            before_additional_send : access constant thinkthen_complete_send_denial_before_additional_send_v1;
         when others =>
            before_retry : access constant thinkthen_complete_send_denial_before_retry_v1;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_send_denial_v1 is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_send_denial_data_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_send_denial_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_send_denial_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_stop_cause_v1 is record
      kind : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_stopped_v1 is record
      c_at : aliased thinkthen_complete_u64_presence_v1;
      cause : access constant thinkthen_complete_stop_cause_v1;
      retryable : aliased Interfaces.Unsigned_32;
      status : aliased thinkthen_complete_u64_presence_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_error_v1 is record
      estimated_input_denial : aliased thinkthen_complete_input_denial_presence_v1;
      kind : access constant thinkthen_complete_failure_kind_v1;
      message : aliased thinkthen_complete_utf8_v1;
      retryable : aliased Interfaces.Unsigned_32;
      send_budget_denial : aliased thinkthen_complete_send_denial_presence_v1;
      stopped : access constant thinkthen_complete_stopped_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_call_error_v1 is record
      error : access constant thinkthen_complete_error_v1;
      facts : aliased thinkthen_complete_facts_presence_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_call_error_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_call_error_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_packet_terminal_v1 is record
      facts : aliased thinkthen_complete_facts_presence_v1;
      failure : aliased thinkthen_complete_call_error_presence_v1;
      kind : aliased thinkthen_complete_utf8_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_packet_data_v1 (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            decide_row : access constant thinkthen_complete_session_packet_decide_row_v1;
         when 1 =>
            choose_row : access constant thinkthen_complete_session_packet_choose_row_v1;
         when 2 =>
            tag_row : access constant thinkthen_complete_session_packet_tag_row_v1;
         when 3 =>
            score_row : access constant thinkthen_complete_session_packet_score_row_v1;
         when 4 =>
            filter_row : access constant thinkthen_complete_session_packet_filter_row_v1;
         when 5 =>
            annotate_row : access constant thinkthen_complete_session_packet_annotate_row_v1;
         when 6 =>
            decide_aggregate : access constant thinkthen_complete_session_packet_decide_aggregate_v1;
         when 7 =>
            choose_aggregate : access constant thinkthen_complete_session_packet_choose_aggregate_v1;
         when 8 =>
            tag_aggregate : access constant thinkthen_complete_session_packet_tag_aggregate_v1;
         when 9 =>
            score_aggregate : access constant thinkthen_complete_session_packet_score_aggregate_v1;
         when 10 =>
            filter_aggregate : access constant thinkthen_complete_session_packet_filter_aggregate_v1;
         when 11 =>
            rank_aggregate : access constant thinkthen_complete_session_packet_rank_aggregate_v1;
         when 12 =>
            find_aggregate : access constant thinkthen_complete_session_packet_find_aggregate_v1;
         when 13 =>
            annotate_aggregate : access constant thinkthen_complete_session_packet_annotate_aggregate_v1;
         when 14 =>
            recognize_aggregate : access constant thinkthen_complete_session_packet_recognize_aggregate_v1;
         when 15 =>
            relate_aggregate : access constant thinkthen_complete_session_packet_relate_aggregate_v1;
         when 16 =>
            observation : access constant thinkthen_complete_session_packet_observation_v1;
         when others =>
            terminal : access constant thinkthen_complete_session_packet_terminal_v1;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_session_packet_v1 is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_session_packet_data_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_session_descriptor_v1 is record
      m_item : access constant thinkthen_request_item_v1;
      m_location : access constant thinkthen_request_session_source_location_v1;
   end record
   with Convention => C_Pass_By_Copy;
   function thinkthen_engine_finish_usage_status_v1
     (engine : access constant thinkthen_engine;
      out_state : access thinkthen_complete_usage_persistence_v1;
      out_advice : access thinkthen_complete_utf8_v1) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_engine_finish_usage_status_v1";
   procedure thinkthen_engine_free (engine : access thinkthen_engine)
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_engine_free";
   function thinkthen_engine_new return access thinkthen_engine
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_engine_new";
   function thinkthen_engine_new_with (settings_json : Interfaces.C.Strings.chars_ptr) return access thinkthen_engine
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_engine_new_with";
   function thinkthen_engine_usage_persistence_v1
     (engine : access constant thinkthen_engine;
      out_state : access thinkthen_complete_usage_persistence_v1;
      out_advice : access thinkthen_complete_utf8_v1) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_engine_usage_persistence_v1";
   function thinkthen_error_code (engine : access constant thinkthen_engine) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_error_code";
   function thinkthen_error_message (engine : access constant thinkthen_engine) return Interfaces.C.Strings.chars_ptr
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_error_message";
   function thinkthen_error_retryable (engine : access constant thinkthen_engine) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_error_retryable";
   procedure thinkthen_session_cancel (session : access thinkthen_session)
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_session_cancel";
   function thinkthen_session_error_message return Interfaces.C.Strings.chars_ptr
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_session_error_message";
   function thinkthen_session_finish
     (session : access thinkthen_session;
      failure_json : Interfaces.C.Strings.chars_ptr;
      failure_len : Interfaces.C.size_t) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_session_finish";
   function thinkthen_session_finish_typed (session : access constant thinkthen_session; failure : access constant thinkthen_request_reader_failure_v1) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_session_finish_typed";
   procedure thinkthen_session_free (session : access thinkthen_session)
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_session_free";
   function thinkthen_session_new_named_with_surface
     (engine : access constant thinkthen_engine;
      request : access constant thinkthen_named_request_v1;
      surface : thinkthen_complete_utf8_v1;
      c_out : System.Address) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_session_new_named_with_surface";
   function thinkthen_session_new_typed_with_surface
     (engine : access constant thinkthen_engine;
      c_function : thinkthen_complete_utf8_v1;
      arguments : access constant thinkthen_request_arguments_v1;
      surface_name : thinkthen_complete_utf8_v1;
      c_out : System.Address) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_session_new_typed_with_surface";
   function thinkthen_session_new_with_surface
     (engine : access constant thinkthen_engine;
      request_json : Interfaces.C.Strings.chars_ptr;
      request_len : Interfaces.C.size_t;
      surface : Interfaces.C.Strings.chars_ptr;
      surface_len : Interfaces.C.size_t;
      c_out : System.Address) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_session_new_with_surface";
   function thinkthen_session_next_packet
     (session : access thinkthen_session;
      interest : Interfaces.Unsigned_32;
      status : access Interfaces.Unsigned_32;
      c_out : System.Address) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_session_next_packet";
   procedure thinkthen_session_result_free (result : access thinkthen_session_result)
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_session_result_free";
   function thinkthen_session_result_view (result : access constant thinkthen_session_result; c_out : System.Address) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_session_result_view";
   function thinkthen_session_try_push
     (session : access thinkthen_session;
      descriptor_json : Interfaces.C.Strings.chars_ptr;
      descriptor_len : Interfaces.C.size_t;
      status : access Interfaces.Unsigned_32) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_session_try_push";
   function thinkthen_session_try_push_typed
     (session : access constant thinkthen_session;
      descriptor : access constant thinkthen_request_session_descriptor_v1;
      status : access Interfaces.Unsigned_32) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_session_try_push_typed";
   function thinkthen_session_try_read
     (session : access thinkthen_session;
      status : access Interfaces.Unsigned_32;
      c_out : System.Address) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_session_try_read";
   function thinkthen_session_wait
     (session : access constant thinkthen_session;
      interest : Interfaces.Unsigned_32;
      timeout_ms : Interfaces.Unsigned_32;
      out_ready : access Interfaces.Unsigned_32) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_session_wait";
end Thinkthen_Session_C;
pragma Style_Checks (On);
pragma Warnings (On, "-gnatwu");
