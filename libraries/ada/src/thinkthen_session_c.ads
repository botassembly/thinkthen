-- Generated from the canonical C header by GNAT. Do not edit.
with Interfaces;
pragma Style_Checks (Off);
pragma Warnings (Off, "-gnatwu");
with Interfaces.C; use Interfaces.C;
with Interfaces.C.Strings;
with System;
package Thinkthen_Session_C is
   K_THINKTHEN_ANSWER_CHOICE_V1 : constant := 2;
   K_THINKTHEN_ANSWER_FIND_V1 : constant := 5;
   K_THINKTHEN_ANSWER_SCORE_V1 : constant := 4;
   K_THINKTHEN_ANSWER_TAG_V1 : constant := 3;
   K_THINKTHEN_ANSWER_YES_NO_V1 : constant := 1;
   K_THINKTHEN_ATTEMPT_OK_V1 : constant := 1;
   K_THINKTHEN_ATTEMPT_STATUS_V1 : constant := 2;
   K_THINKTHEN_ATTEMPT_TRANSPORT_V1 : constant := 3;
   K_THINKTHEN_BATCH_MAX_V1 : constant := 2;
   K_THINKTHEN_BATCH_RECORDS_V1 : constant := 1;
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
   K_THINKTHEN_COMPLETE_ESTIMATED_INPUT_DENIAL_ADDITIONAL_REQUEST_V1 : constant := 2;
   K_THINKTHEN_COMPLETE_ESTIMATED_INPUT_DENIAL_INITIAL_REQUEST_V1 : constant := 1;
   K_THINKTHEN_COMPLETE_ESTIMATED_INPUT_DENIAL_RETRY_V1 : constant := 3;
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
   K_THINKTHEN_COMPLETE_INPUT_DECLARATION_OBJECT_V1 : constant := 2;
   K_THINKTHEN_COMPLETE_INPUT_DECLARATION_STRING_V1 : constant := 1;
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
   K_THINKTHEN_COMPLETE_PRESENCE_MISSING_V1 : constant := 0;
   K_THINKTHEN_COMPLETE_PRESENCE_NULL_V1 : constant := 1;
   K_THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 : constant := 2;
   K_THINKTHEN_COMPLETE_READABLE_QUESTION_CHOOSE_V1 : constant := 2;
   K_THINKTHEN_COMPLETE_READABLE_QUESTION_DECIDE_V1 : constant := 1;
   K_THINKTHEN_COMPLETE_READABLE_QUESTION_SCORE_V1 : constant := 4;
   K_THINKTHEN_COMPLETE_READABLE_QUESTION_TAG_V1 : constant := 3;
   K_THINKTHEN_COMPLETE_RECOGNITION_MODE_BOUNDARY_ONLY_V1 : constant := 2;
   K_THINKTHEN_COMPLETE_RECOGNITION_MODE_WHOLE_V1 : constant := 1;
   K_THINKTHEN_COMPLETE_RECOGNITION_ODDS_FIELDS_NAMES_PAIRS_PIECES_PROPOSALS_V1 : constant := 1;
   K_THINKTHEN_COMPLETE_RECOGNITION_ODDS_FIELDS_PIECES_PROPOSALS_V1 : constant := 2;
   K_THINKTHEN_COMPLETE_RECOGNIZE_FIELDS_ENTITIES_V1 : constant := 1;
   K_THINKTHEN_COMPLETE_RECOGNIZE_FIELDS_MODE_PROPOSALS_V1 : constant := 2;
   K_THINKTHEN_COMPLETE_RELATED_ENTITY_EDGE_PROPERTIES_SOURCE_FIELDS_FILE_KIND_NAME_ORDINAL_RECORD_V1 : constant := 2;
   K_THINKTHEN_COMPLETE_RELATED_ENTITY_EDGE_PROPERTIES_SOURCE_FIELDS_KIND_NAME_V1 : constant := 1;
   K_THINKTHEN_COMPLETE_RELATION_DIRECTION_EITHER_V1 : constant := 2;
   K_THINKTHEN_COMPLETE_RELATION_DIRECTION_SOURCE_TO_TARGET_V1 : constant := 1;
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
   K_THINKTHEN_COMPLETE_SEND_BUDGET_DENIAL_BEFORE_ADDITIONAL_SEND_V1 : constant := 2;
   K_THINKTHEN_COMPLETE_SEND_BUDGET_DENIAL_BEFORE_FIRST_SEND_V1 : constant := 1;
   K_THINKTHEN_COMPLETE_SEND_BUDGET_DENIAL_BEFORE_RETRY_V1 : constant := 3;
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
   K_THINKTHEN_CONTENT_JSON_V1 : constant := 2;
   K_THINKTHEN_CONTENT_TEXT_V1 : constant := 1;
   K_THINKTHEN_DECIDE_AUTHORED_V1 : constant := 2;
   K_THINKTHEN_DECIDE_BOOLEAN_V1 : constant := 1;
   K_THINKTHEN_DECIDE_NULL_V1 : constant := 0;
   K_THINKTHEN_DIRECTION_EITHER_V1 : constant := 2;
   K_THINKTHEN_DIRECTION_SOURCE_TO_TARGET_V1 : constant := 1;
   K_THINKTHEN_EBACKEND : constant := 2;
   K_THINKTHEN_ECANCELLED : constant := 5;
   K_THINKTHEN_EDEADLINE : constant := 3;
   K_THINKTHEN_EDEFECT : constant := 6;
   K_THINKTHEN_ELOCAL : constant := 4;
   K_THINKTHEN_EUSAGE : constant := 1;
   K_THINKTHEN_EVENT_QUESTION_V1 : constant := 1;
   K_THINKTHEN_EVENT_ROW_V1 : constant := 2;
   K_THINKTHEN_FUNCTION_ANNOTATE_V1 : constant := 8;
   K_THINKTHEN_FUNCTION_CHOOSE_V1 : constant := 2;
   K_THINKTHEN_FUNCTION_DECIDE_V1 : constant := 1;
   K_THINKTHEN_FUNCTION_FILTER_V1 : constant := 5;
   K_THINKTHEN_FUNCTION_FIND_V1 : constant := 7;
   K_THINKTHEN_FUNCTION_RANK_V1 : constant := 6;
   K_THINKTHEN_FUNCTION_RECOGNIZE_V1 : constant := 9;
   K_THINKTHEN_FUNCTION_RELATE_V1 : constant := 10;
   K_THINKTHEN_FUNCTION_SCORE_V1 : constant := 4;
   K_THINKTHEN_FUNCTION_TAG_V1 : constant := 3;
   K_THINKTHEN_ID_FAILURE_V1 : constant := 2;
   K_THINKTHEN_ID_OBSERVATION_V1 : constant := 1;
   K_THINKTHEN_IMAGE_JPEG_V1 : constant := 1;
   K_THINKTHEN_IMAGE_PNG_V1 : constant := 2;
   K_THINKTHEN_MEMBER_FAILURE_V1 : constant := 2;
   K_THINKTHEN_MEMBER_INVALID_DISTRIBUTION_V1 : constant := 5;
   K_THINKTHEN_MEMBER_INVALID_PROBABILITY_V1 : constant := 4;
   K_THINKTHEN_MEMBER_MISSING_ANSWER_V1 : constant := 1;
   K_THINKTHEN_MEMBER_MISSING_PROBABILITY_V1 : constant := 3;
   K_THINKTHEN_MEMBER_SUCCESS_V1 : constant := 1;
   K_THINKTHEN_MEMBER_UNEXPECTED_PROBABILITY_V1 : constant := 6;
   K_THINKTHEN_MEMBER_WRONG_KIND_V1 : constant := 2;
   K_THINKTHEN_NO : constant := 0;
   K_THINKTHEN_NO_DEADLINE : constant := -1;
   K_THINKTHEN_OK : constant := 0;
   K_THINKTHEN_ORIGIN_CACHE_V1 : constant := 2;
   K_THINKTHEN_ORIGIN_LIVE_V1 : constant := 1;
   K_THINKTHEN_ORIGIN_MEMORY_V1 : constant := 5;
   K_THINKTHEN_ORIGIN_PROXY_V1 : constant := 4;
   K_THINKTHEN_ORIGIN_REPLAY_V1 : constant := 3;
   K_THINKTHEN_PROBABILITIES_NAMED_V1 : constant := 2;
   K_THINKTHEN_PROBABILITIES_YES_V1 : constant := 1;
   K_THINKTHEN_RELATION_CHOICE_V1 : constant := 2;
   K_THINKTHEN_RELATION_YES_NO_V1 : constant := 1;
   K_THINKTHEN_RESULT_FAILURE_V1 : constant := 2;
   K_THINKTHEN_RESULT_SUCCESS_V1 : constant := 1;
   K_THINKTHEN_RULE_BAND_V1 : constant := 3;
   K_THINKTHEN_RULE_CUT_V1 : constant := 2;
   K_THINKTHEN_RULE_DEFAULT_V1 : constant := 0;
   K_THINKTHEN_RULE_NULL_V1 : constant := 1;
   K_THINKTHEN_SESSION_ACCEPTED_V1 : constant := 0;
   K_THINKTHEN_SESSION_CLOSED_V1 : constant := 2;
   K_THINKTHEN_SESSION_END_V1 : constant := 2;
   K_THINKTHEN_SESSION_FULL_V1 : constant := 1;
   K_THINKTHEN_SESSION_PENDING_V1 : constant := 1;
   K_THINKTHEN_SESSION_RESULT_V1 : constant := 0;
   K_THINKTHEN_SOURCE_FILE_V1 : constant := 3;
   K_THINKTHEN_SOURCE_IMAGE_FILE_V1 : constant := 4;
   K_THINKTHEN_SOURCE_JSONL_V1 : constant := 5;
   K_THINKTHEN_SOURCE_LINE_V1 : constant := 1;
   K_THINKTHEN_SOURCE_WINDOW_V1 : constant := 2;
   K_THINKTHEN_STAGE_BOUNDARY_V1 : constant := 1;
   K_THINKTHEN_STAGE_EDGE_V1 : constant := 3;
   K_THINKTHEN_STAGE_KIND_V1 : constant := 2;
   K_THINKTHEN_STAGE_RELATION_V1 : constant := 4;
   K_THINKTHEN_STOP_BACKEND_V1 : constant := 8;
   K_THINKTHEN_STOP_CANCELLED_V1 : constant := 9;
   K_THINKTHEN_STOP_DEADLINE_V1 : constant := 11;
   K_THINKTHEN_STOP_DEFECT_V1 : constant := 10;
   K_THINKTHEN_STOP_LOCAL_V1 : constant := 2;
   K_THINKTHEN_STOP_NO_KEY_V1 : constant := 3;
   K_THINKTHEN_STOP_REPLY_V1 : constant := 7;
   K_THINKTHEN_STOP_STATUS_V1 : constant := 5;
   K_THINKTHEN_STOP_TOO_LARGE_V1 : constant := 6;
   K_THINKTHEN_STOP_TRANSPORT_V1 : constant := 4;
   K_THINKTHEN_STOP_USAGE_V1 : constant := 1;
   K_THINKTHEN_UNSURE : constant := 2;
   K_THINKTHEN_VERSION_MAJOR : constant := 0;
   K_THINKTHEN_VERSION_MINOR : constant := 2;
   K_THINKTHEN_VERSION_PATCH : constant := 0;
   K_THINKTHEN_YES : constant := 1;
   type thinkthen_cancel_token is null record;
   type thinkthen_input_declaration_kind_v1 is
     (THINKTHEN_DECLARATION_ABSENT_V1,
      THINKTHEN_DECLARATION_STRING_V1,
      THINKTHEN_DECLARATION_OBJECT_V1)
   with Convention => C;
   subtype thinkthen_input_property_kind_v1 is unsigned;
   thinkthen_input_property_kind_v1_THINKTHEN_PROPERTY_STRING_V1 : constant thinkthen_input_property_kind_v1 := 1;
   thinkthen_input_property_kind_v1_THINKTHEN_PROPERTY_NUMBER_V1 : constant thinkthen_input_property_kind_v1 := 2;
   thinkthen_input_property_kind_v1_THINKTHEN_PROPERTY_BOOLEAN_V1 : constant thinkthen_input_property_kind_v1 := 3;
   thinkthen_input_property_kind_v1_THINKTHEN_PROPERTY_STRING_LIST_V1 : constant thinkthen_input_property_kind_v1 := 4;
   subtype thinkthen_question_loader_role_v1 is unsigned;
   thinkthen_question_loader_role_v1_THINKTHEN_LOAD_ATOMIC_V1 : constant thinkthen_question_loader_role_v1 := 1;
   thinkthen_question_loader_role_v1_THINKTHEN_LOAD_SET_V1 : constant thinkthen_question_loader_role_v1 := 2;
   thinkthen_question_loader_role_v1_THINKTHEN_LOAD_DYNAMIC_CHOOSE_V1 : constant thinkthen_question_loader_role_v1 := 3;
   thinkthen_question_loader_role_v1_THINKTHEN_LOAD_RECOGNIZE_V1 : constant thinkthen_question_loader_role_v1 := 4;
   thinkthen_question_loader_role_v1_THINKTHEN_LOAD_RELATE_V1 : constant thinkthen_question_loader_role_v1 := 5;
   thinkthen_question_loader_role_v1_THINKTHEN_LOAD_RANK_V1 : constant thinkthen_question_loader_role_v1 := 6;
   thinkthen_question_loader_role_v1_THINKTHEN_LOAD_RANK_SET_V1 : constant thinkthen_question_loader_role_v1 := 7;
   thinkthen_question_loader_role_v1_THINKTHEN_LOAD_FIND_V1 : constant thinkthen_question_loader_role_v1 := 8;
   type thinkthen_batch is null record;
   type thinkthen_engine is null record;
   type thinkthen_image is null record;
   type thinkthen_question is null record;
   type thinkthen_result is null record;
   type thinkthen_session is null record;
   type thinkthen_session_result is null record;
   type thinkthen_source is null record;
   type thinkthen_string_v1 is record
      data : Interfaces.C.Strings.chars_ptr;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_content_v1 is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_string_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_optional_content_v1 is record
      present : aliased int;
      value : aliased thinkthen_content_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_optional_size_v1 is record
      present : aliased int;
      value : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_controls_v1 is record
      deadline_ms : aliased Interfaces.Integer_64;
      cancel : access thinkthen_cancel_token;
      context : aliased thinkthen_optional_content_v1;
      batch : aliased thinkthen_optional_size_v1;
      batch_max : aliased int;
      attempts : aliased int;
      surface : aliased thinkthen_string_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_answer is record
      outcome : aliased int;
      probability : aliased double;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_optional_string_v1 is record
      present : aliased int;
      value : aliased thinkthen_string_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_image_view_v1 is record
      media : aliased Interfaces.Unsigned_32;
      bytes : access Interfaces.Unsigned_8;
      bytes_len : aliased Interfaces.C.size_t;
      width : aliased Interfaces.Unsigned_32;
      height : aliased Interfaces.Unsigned_32;
      filename : aliased thinkthen_optional_string_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_optional_u64_v1 is record
      present : aliased int;
      value : aliased Interfaces.Unsigned_64;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_input_property_v1 is record
      name : aliased thinkthen_string_v1;
      kind : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_input_properties_v1 is record
      data : access constant thinkthen_input_property_v1;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_strings_v1 is record
      data : access constant thinkthen_string_v1;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_input_declaration_v1 is record
      kind : aliased Interfaces.Unsigned_32;
      properties : aliased thinkthen_input_properties_v1;
      required : aliased thinkthen_strings_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_question_author_v1 is record
      name : aliased thinkthen_optional_string_v1;
      wording_version : aliased thinkthen_optional_u64_v1;
      item_schema : aliased thinkthen_input_declaration_v1;
      context_schema : aliased thinkthen_input_declaration_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_optional_double_v1 is record
      present : aliased int;
      value : aliased double;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_choice_v1 is record
      name : aliased thinkthen_string_v1;
      description : aliased thinkthen_optional_content_v1;
      weight : aliased thinkthen_optional_double_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_choices_v1 is record
      data : access constant thinkthen_choice_v1;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_rule_v1 is record
      kind : aliased Interfaces.Unsigned_32;
      low : aliased double;
      high : aliased double;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_member_spec_v1 is record
      name : aliased thinkthen_string_v1;
      question : access constant thinkthen_question;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_member_specs_v1 is record
      data : access constant thinkthen_member_spec_v1;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_relation_v1 is record
      name : aliased thinkthen_string_v1;
      source : aliased thinkthen_string_v1;
      target : aliased thinkthen_string_v1;
      reads : aliased thinkthen_optional_string_v1;
      either : aliased int;
      single : aliased int;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_relations_v1 is record
      data : access constant thinkthen_relation_v1;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_question_spec_v1 is record
      kind : aliased Interfaces.Unsigned_32;
      text : aliased thinkthen_content_v1;
      yes : aliased thinkthen_optional_content_v1;
      no : aliased thinkthen_optional_content_v1;
      choices : aliased thinkthen_choices_v1;
      threshold : aliased thinkthen_rule_v1;
      relation_threshold : aliased thinkthen_rule_v1;
      model : aliased thinkthen_optional_string_v1;
      profile : aliased thinkthen_optional_string_v1;
      batch : aliased thinkthen_optional_size_v1;
      batch_max : aliased int;
      none : aliased int;
      on : aliased thinkthen_strings_v1;
      members : aliased thinkthen_member_specs_v1;
      kinds : aliased thinkthen_choices_v1;
      relations : aliased thinkthen_relations_v1;
      name_pointer : aliased thinkthen_optional_string_v1;
      kind_pointer : aliased thinkthen_optional_string_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_recognition_task_v1 is record
      instructions : aliased thinkthen_optional_string_v1;
      entity_definition : aliased thinkthen_optional_string_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_question_view_v1;
   type thinkthen_question_member_v1 is record
      name : aliased thinkthen_string_v1;
      question : access constant thinkthen_question_view_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_question_members_v1 is record
      data : access constant thinkthen_question_member_v1;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_question_view_v1 is record
      kind : aliased Interfaces.Unsigned_32;
      text : aliased thinkthen_content_v1;
      yes : aliased thinkthen_optional_content_v1;
      no : aliased thinkthen_optional_content_v1;
      choices : aliased thinkthen_choices_v1;
      threshold : aliased thinkthen_rule_v1;
      relation_threshold : aliased thinkthen_rule_v1;
      model : aliased thinkthen_optional_string_v1;
      profile : aliased thinkthen_optional_string_v1;
      batch : aliased thinkthen_optional_size_v1;
      batch_max : aliased int;
      none : aliased int;
      on : aliased thinkthen_strings_v1;
      members : aliased thinkthen_question_members_v1;
      kinds : aliased thinkthen_choices_v1;
      relations : aliased thinkthen_relations_v1;
      name_pointer : aliased thinkthen_optional_string_v1;
      kind_pointer : aliased thinkthen_optional_string_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_optional_question_v1 is record
      present : aliased int;
      value : aliased thinkthen_question_view_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_probability_v1 is record
      name : aliased thinkthen_string_v1;
      probability : aliased double;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_probabilities_v1 is record
      data : access constant thinkthen_probability_v1;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_named_answer_v1 is record
      pick : aliased thinkthen_string_v1;
      probabilities : aliased thinkthen_probabilities_v1;
      confidence : aliased thinkthen_optional_double_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_score_answer_v1 is record
      level : aliased thinkthen_string_v1;
      probabilities : aliased thinkthen_probabilities_v1;
      confidence : aliased thinkthen_optional_double_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_answer_data_v1 (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            probability : aliased double;
         when 1 =>
            choice : aliased thinkthen_named_answer_v1;
         when 2 =>
            tag : aliased thinkthen_probabilities_v1;
         when 3 =>
            score : aliased thinkthen_score_answer_v1;
         when others =>
            find : aliased thinkthen_named_answer_v1;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_answer_v1 is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_answer_data_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_optional_answer_v1 is record
      present : aliased int;
      value : aliased thinkthen_answer_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_optional_rule_v1 is record
      present : aliased int;
      value : aliased thinkthen_rule_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_location_v1 is record
      file : aliased thinkthen_optional_string_v1;
      first_line : aliased thinkthen_optional_size_v1;
      last_line : aliased thinkthen_optional_size_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_optional_location_v1 is record
      present : aliased int;
      value : aliased thinkthen_location_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_usage_v1 is record
      input_tokens : aliased Interfaces.Unsigned_64;
      output_tokens : aliased Interfaces.Unsigned_64;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_optional_usage_v1 is record
      present : aliased int;
      value : aliased thinkthen_usage_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_profile_warning_v1 is record
      tuned_for : aliased thinkthen_string_v1;
      running : aliased thinkthen_string_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_optional_profile_warning_v1 is record
      present : aliased int;
      value : aliased thinkthen_profile_warning_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_batch_v1 is record
      kind : aliased Interfaces.Unsigned_32;
      records : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_optional_batch_v1 is record
      present : aliased int;
      value : aliased thinkthen_batch_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_batch_warning_v1 is record
      tuned_for : aliased thinkthen_batch_v1;
      running : aliased thinkthen_batch_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_optional_batch_warning_v1 is record
      present : aliased int;
      value : aliased thinkthen_batch_warning_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_optional_u16_v1 is record
      present : aliased int;
      value : aliased Interfaces.Unsigned_16;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_attempt_v1 is record
      ordinal : aliased Interfaces.Unsigned_64;
      request_sha256 : aliased thinkthen_string_v1;
      wall_ms : aliased Interfaces.Unsigned_64;
      outcome : aliased Interfaces.Unsigned_32;
      sdk_request_id : aliased thinkthen_string_v1;
      status : aliased thinkthen_optional_u16_v1;
      server_ms : aliased thinkthen_optional_u64_v1;
      request_id : aliased thinkthen_optional_string_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_attempts_v1 is record
      data : access constant thinkthen_attempt_v1;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_optional_attempts_v1 is record
      present : aliased int;
      value : aliased thinkthen_attempts_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_optional_discriminator_v1 is record
      present : aliased int;
      value : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_question_source_v1 is record
      origin : aliased Interfaces.Unsigned_32;
      answered_by : aliased thinkthen_string_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_question_sources_v1 is record
      data : access constant thinkthen_question_source_v1;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_observation_identity_data_v1 (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            observation_id : aliased thinkthen_string_v1;
         when others =>
            failure_id : aliased thinkthen_string_v1;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_observation_identity_v1 is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_observation_identity_data_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_observation_identities_v1 is record
      data : access constant thinkthen_observation_identity_v1;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_meta_v1 is record
      tool : aliased thinkthen_string_v1;
      question_sha256 : aliased thinkthen_optional_string_v1;
      questions_sha256 : aliased thinkthen_optional_string_v1;
      url : aliased thinkthen_string_v1;
      model : aliased thinkthen_string_v1;
      usage : aliased thinkthen_optional_usage_v1;
      requests_sent : aliased Interfaces.Unsigned_64;
      cached : aliased int;
      requests : aliased thinkthen_strings_v1;
      failed_questions : aliased Interfaces.C.size_t;
      profile_warning : aliased thinkthen_optional_profile_warning_v1;
      batch_setting : aliased thinkthen_optional_batch_v1;
      batch_warning : aliased thinkthen_optional_batch_warning_v1;
      context_sha256 : aliased thinkthen_optional_string_v1;
      attempts : aliased thinkthen_optional_attempts_v1;
      origin : aliased thinkthen_optional_discriminator_v1;
      question_sources : aliased thinkthen_question_sources_v1;
      observations : aliased thinkthen_observation_identities_v1;
      answered_by : aliased thinkthen_optional_string_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_image_views_v1 is record
      data : access constant thinkthen_image_view_v1;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_optional_image_views_v1 is record
      present : aliased int;
      value : aliased thinkthen_image_views_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_row_v1 is record
      answer_id : aliased thinkthen_string_v1;
      input : aliased thinkthen_optional_content_v1;
      question : aliased thinkthen_optional_question_v1;
      answer : aliased thinkthen_optional_answer_v1;
      threshold : aliased thinkthen_optional_rule_v1;
      position : aliased thinkthen_optional_location_v1;
      input_file : aliased thinkthen_optional_string_v1;
      meta : aliased thinkthen_meta_v1;
      images : aliased thinkthen_optional_image_views_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_decide_value_data_v1 (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            boolean : aliased int;
         when others =>
            authored : aliased thinkthen_content_v1;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_decide_value_v1 is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_decide_value_data_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_member_value_data_v1 (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            decide : aliased thinkthen_decide_value_v1;
         when 1 =>
            choose : aliased thinkthen_optional_string_v1;
         when 2 =>
            tag : aliased thinkthen_strings_v1;
         when others =>
            score : aliased double;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_member_value_v1 is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_member_value_data_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_member_success_v1 is record
      answer_id : aliased thinkthen_string_v1;
      value : aliased thinkthen_member_value_v1;
      answer : aliased thinkthen_answer_v1;
      threshold : aliased thinkthen_rule_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_member_failure_v1 is record
      failure_id : aliased thinkthen_string_v1;
      cause : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_member_data_v1 (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            success : aliased thinkthen_member_success_v1;
         when others =>
            failure : aliased thinkthen_member_failure_v1;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_member_v1 is record
      name : aliased thinkthen_string_v1;
      request : aliased thinkthen_string_v1;
      question : aliased thinkthen_question_view_v1;
      state : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_member_data_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_members_v1 is record
      data : access constant thinkthen_member_v1;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_annotate_view_v1 is record
      common : aliased thinkthen_row_v1;
      answers : aliased thinkthen_members_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_choose_view_v1 is record
      common : aliased thinkthen_row_v1;
      value : aliased thinkthen_optional_string_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_decide_view_v1 is record
      common : aliased thinkthen_row_v1;
      value : aliased thinkthen_decide_value_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_reported_usage_v1 is record
      present : aliased int;
      input_tokens : aliased thinkthen_optional_u64_v1;
      output_tokens : aliased thinkthen_optional_u64_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_source_detail_v1 is record
      origin : aliased Interfaces.Unsigned_32;
      answered_by : aliased thinkthen_string_v1;
      batch_size : aliased thinkthen_optional_size_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_source_details_v1 is record
      data : access constant thinkthen_source_detail_v1;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_input_view_v1 is record
      original : aliased thinkthen_optional_content_v1;
      position : aliased thinkthen_optional_location_v1;
      images : aliased thinkthen_optional_image_views_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_input_views_v1 is record
      data : access constant thinkthen_input_view_v1;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_details_v1 is record
      question : aliased thinkthen_optional_question_v1;
      threshold : aliased thinkthen_optional_rule_v1;
      raw_pick : aliased thinkthen_optional_string_v1;
      usage : aliased thinkthen_reported_usage_v1;
      question_sources : aliased thinkthen_source_details_v1;
      observations : aliased thinkthen_observation_identities_v1;
      inputs : aliased thinkthen_input_views_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_filter_view_v1 is record
      common : aliased thinkthen_row_v1;
      value : aliased int;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_find_view_v1 is record
      common : aliased thinkthen_row_v1;
      value : aliased thinkthen_optional_content_v1;
      index : aliased thinkthen_optional_size_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_observed_probabilities_data_v1 (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            yes : aliased double;
         when others =>
            named : aliased thinkthen_probabilities_v1;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_observed_probabilities_v1 is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_observed_probabilities_data_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_observation_success_v1 is record
      answer_id : aliased thinkthen_string_v1;
      observation_id : aliased thinkthen_string_v1;
      value : aliased thinkthen_member_value_v1;
      probabilities : aliased thinkthen_observed_probabilities_v1;
      confidence : aliased thinkthen_optional_double_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_question_observation_data_v1 (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            success : aliased thinkthen_observation_success_v1;
         when others =>
            failure : aliased thinkthen_member_failure_v1;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_question_observation_v1 is record
      index : aliased Interfaces.C.size_t;
      member : aliased thinkthen_optional_string_v1;
      stage : aliased thinkthen_optional_discriminator_v1;
      position : aliased Interfaces.C.size_t;
      question_sha256 : aliased thinkthen_string_v1;
      model : aliased thinkthen_string_v1;
      url : aliased thinkthen_string_v1;
      requests : aliased thinkthen_strings_v1;
      requests_sent : aliased Interfaces.Unsigned_64;
      cached : aliased int;
      failed_questions : aliased Interfaces.C.size_t;
      usage : aliased thinkthen_optional_usage_v1;
      question_sources : aliased thinkthen_question_sources_v1;
      state : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_question_observation_data_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_tag_view_v1 is record
      common : aliased thinkthen_row_v1;
      value : aliased thinkthen_strings_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_score_view_v1 is record
      common : aliased thinkthen_row_v1;
      value : aliased double;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_rank_view_v1 is record
      common : aliased thinkthen_row_v1;
      value : aliased thinkthen_optional_size_v1;
      question_name : aliased thinkthen_optional_string_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_entity_v1 is record
      text : aliased thinkthen_string_v1;
      start : aliased Interfaces.C.size_t;
      c_end : aliased Interfaces.C.size_t;
      length : aliased Interfaces.C.size_t;
      kind : aliased thinkthen_string_v1;
      strength : aliased double;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_entities_v1 is record
      data : access constant thinkthen_entity_v1;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_entity_edge_v1 is record
      relation : aliased thinkthen_string_v1;
      source : aliased thinkthen_entity_v1;
      target : aliased thinkthen_entity_v1;
      probability : aliased double;
      either : aliased int;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_entity_edges_v1 is record
      data : access constant thinkthen_entity_edge_v1;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_optional_entity_edges_v1 is record
      present : aliased int;
      value : aliased thinkthen_entity_edges_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_recognize_value_v1 is record
      entities : aliased thinkthen_entities_v1;
      relations : aliased thinkthen_optional_entity_edges_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_piece_v1 is record
      start : aliased Interfaces.C.size_t;
      c_end : aliased Interfaces.C.size_t;
      tags : aliased thinkthen_probabilities_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_pieces_v1 is record
      data : access constant thinkthen_piece_v1;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_optional_probabilities_v1 is record
      present : aliased int;
      value : aliased thinkthen_probabilities_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_name_v1 is record
      start : aliased Interfaces.C.size_t;
      c_end : aliased Interfaces.C.size_t;
      kinds : aliased thinkthen_optional_probabilities_v1;
      edges : aliased thinkthen_optional_probabilities_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_names_v1 is record
      data : access constant thinkthen_name_v1;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_place_v1 is record
      start : aliased Interfaces.C.size_t;
      c_end : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_pair_v1 is record
      relation : aliased thinkthen_string_v1;
      source : aliased thinkthen_place_v1;
      target : aliased thinkthen_place_v1;
      probability : aliased double;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_pairs_v1 is record
      data : access constant thinkthen_pair_v1;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_recognize_answer_v1 is record
      pieces : aliased thinkthen_pieces_v1;
      names : aliased thinkthen_names_v1;
      pairs : aliased thinkthen_pairs_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_recognize_view_v1 is record
      common : aliased thinkthen_row_v1;
      value : aliased thinkthen_recognize_value_v1;
      answer : aliased thinkthen_recognize_answer_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_endpoint_v1 is record
      name : aliased thinkthen_string_v1;
      kind : aliased thinkthen_string_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_edge_v1 is record
      relation : aliased thinkthen_string_v1;
      source : aliased thinkthen_endpoint_v1;
      target : aliased thinkthen_endpoint_v1;
      probability : aliased double;
      either : aliased int;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_edges_v1 is record
      data : access constant thinkthen_edge_v1;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_optional_endpoint_v1 is record
      present : aliased int;
      value : aliased thinkthen_endpoint_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_relation_success_v1 is record
      answer_id : aliased thinkthen_string_v1;
      probability : aliased double;
      accepted : aliased int;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_relation_answer_data_v1 (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            success : aliased thinkthen_relation_success_v1;
         when others =>
            failure : aliased thinkthen_member_failure_v1;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_relation_answer_v1 is record
      relation : aliased thinkthen_string_v1;
      reads : aliased thinkthen_string_v1;
      method : aliased Interfaces.Unsigned_32;
      direction : aliased Interfaces.Unsigned_32;
      source : aliased thinkthen_endpoint_v1;
      target : aliased thinkthen_optional_endpoint_v1;
      request : aliased thinkthen_string_v1;
      state : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_relation_answer_data_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_relation_answers_v1 is record
      data : access constant thinkthen_relation_answer_v1;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_relate_view_v1 is record
      common : aliased thinkthen_row_v1;
      value : aliased thinkthen_edges_v1;
      questions : aliased thinkthen_relation_answers_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_row_observation_data_v1 (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            decide : aliased thinkthen_decide_view_v1;
         when 1 =>
            choose : aliased thinkthen_choose_view_v1;
         when 2 =>
            tag : aliased thinkthen_tag_view_v1;
         when 3 =>
            score : aliased thinkthen_score_view_v1;
         when 4 =>
            filter : aliased thinkthen_filter_view_v1;
         when 5 =>
            rank : aliased thinkthen_rank_view_v1;
         when 6 =>
            find : aliased thinkthen_find_view_v1;
         when 7 =>
            annotate : aliased thinkthen_annotate_view_v1;
         when 8 =>
            recognize : aliased thinkthen_recognize_view_v1;
         when others =>
            relate : aliased thinkthen_relate_view_v1;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_row_observation_v1 is record
      index : aliased Interfaces.C.size_t;
      c_function : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_row_observation_data_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_observation_data_v1 (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            question : aliased thinkthen_question_observation_v1;
         when others =>
            row : aliased thinkthen_row_observation_v1;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_observation_v1 is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_observation_data_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_source_entity_v1 is record
      entity : aliased thinkthen_entity_v1;
      position : aliased thinkthen_optional_location_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_source_entities_v1 is record
      data : access constant thinkthen_source_entity_v1;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_source_entity_edge_v1 is record
      relation : aliased thinkthen_string_v1;
      source : aliased thinkthen_source_entity_v1;
      target : aliased thinkthen_source_entity_v1;
      probability : aliased double;
      either : aliased int;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_source_entity_edges_v1 is record
      data : access constant thinkthen_source_entity_edge_v1;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_optional_source_entity_edges_v1 is record
      present : aliased int;
      value : aliased thinkthen_source_entity_edges_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_source_recognition_v1 is record
      present : aliased int;
      entities : aliased thinkthen_source_entities_v1;
      relations : aliased thinkthen_optional_source_entity_edges_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_source_endpoint_v1 is record
      ordinal : aliased Interfaces.C.size_t;
      endpoint : aliased thinkthen_endpoint_v1;
      c_record : aliased thinkthen_content_v1;
      position : aliased thinkthen_optional_location_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_source_edge_v1 is record
      relation : aliased thinkthen_string_v1;
      source : aliased thinkthen_source_endpoint_v1;
      target : aliased thinkthen_source_endpoint_v1;
      probability : aliased double;
      either : aliased int;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_source_edges_v1 is record
      data : access constant thinkthen_source_edge_v1;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_source_relations_v1 is record
      present : aliased int;
      edges : aliased thinkthen_source_edges_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_optional_meta_v1 is record
      present : aliased int;
      value : aliased thinkthen_meta_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_facts_v1 is record
      call_id : aliased thinkthen_string_v1;
      cache_answers : aliased Interfaces.Unsigned_64;
      estimated_cost_usd : aliased thinkthen_optional_string_v1;
      input_tokens : aliased thinkthen_optional_u64_v1;
      model : aliased thinkthen_optional_string_v1;
      output_tokens : aliased thinkthen_optional_u64_v1;
      records : aliased Interfaces.Unsigned_64;
      requests_sent : aliased Interfaces.Unsigned_64;
      seconds : aliased double;
      command_ms : aliased thinkthen_optional_u64_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_optional_facts_v1 is record
      present : aliased int;
      value : aliased thinkthen_facts_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_stopped_v1 is record
      c_at : aliased thinkthen_optional_size_v1;
      cause : aliased Interfaces.Unsigned_32;
      status : aliased thinkthen_optional_u16_v1;
      retryable : aliased int;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_optional_stopped_v1 is record
      present : aliased int;
      value : aliased thinkthen_stopped_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_error_v1 is record
      code : aliased int;
      message : aliased thinkthen_string_v1;
      retryable : aliased int;
      stopped : aliased thinkthen_optional_stopped_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_optional_error_v1 is record
      present : aliased int;
      value : aliased thinkthen_error_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_summary_v1 is record
      state : aliased Interfaces.Unsigned_32;
      schema : aliased thinkthen_string_v1;
      answer_id : aliased thinkthen_optional_string_v1;
      c_function : aliased thinkthen_optional_discriminator_v1;
      count : aliased Interfaces.C.size_t;
      observation_count : aliased Interfaces.C.size_t;
      meta : aliased thinkthen_optional_meta_v1;
      facts : aliased thinkthen_optional_facts_v1;
      attempts : aliased thinkthen_optional_attempts_v1;
      error : aliased thinkthen_optional_error_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_utf8_v1 is record
      data : Interfaces.C.Strings.chars_ptr;
      len : aliased Interfaces.C.size_t;
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
   type thinkthen_complete_answer_choice_field_confidence_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased double;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answer_choice_field_probabilities_entry_v1 is record
      name : aliased thinkthen_complete_utf8_v1;
      value : aliased double;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answer_choice_field_probabilities_v1 is record
      data : access constant thinkthen_complete_answer_choice_field_probabilities_entry_v1;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answer_choice_v1 is record
      confidence : aliased thinkthen_complete_answer_choice_field_confidence_presence_v1;
      kind : aliased thinkthen_complete_utf8_v1;
      pick : aliased thinkthen_complete_utf8_v1;
      probabilities : aliased thinkthen_complete_answer_choice_field_probabilities_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answer_tag_field_probabilities_entry_v1 is record
      name : aliased thinkthen_complete_utf8_v1;
      value : aliased double;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answer_tag_field_probabilities_v1 is record
      data : access constant thinkthen_complete_answer_tag_field_probabilities_entry_v1;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answer_tag_v1 is record
      kind : aliased thinkthen_complete_utf8_v1;
      probabilities : aliased thinkthen_complete_answer_tag_field_probabilities_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answer_score_field_confidence_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased double;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answer_score_field_probabilities_entry_v1 is record
      name : aliased thinkthen_complete_utf8_v1;
      value : aliased double;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answer_score_field_probabilities_v1 is record
      data : access constant thinkthen_complete_answer_score_field_probabilities_entry_v1;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answer_score_v1 is record
      confidence : aliased thinkthen_complete_answer_score_field_confidence_presence_v1;
      kind : aliased thinkthen_complete_utf8_v1;
      level : aliased thinkthen_complete_utf8_v1;
      probabilities : aliased thinkthen_complete_answer_score_field_probabilities_v1;
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
   type thinkthen_complete_atomic_decide_value_field_images_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_atomic_decide_value_field_images_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_atomic_decide_value_field_images_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_atomic_decide_value_field_index_presence_v1 is record
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
   type thinkthen_complete_atomic_decide_value_field_input_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_json_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_rank_member_result_field_images_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_rank_member_result_field_images_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_rank_member_result_field_images_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_meta_field_answered_by_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_utf8_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_attempt_outcome_v1 is record
      kind : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_attempt_field_request_id_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_utf8_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_sdk_request_id_v1 is record
      value : aliased thinkthen_complete_utf8_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_attempt_field_server_ms_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased Interfaces.Unsigned_64;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_attempt_field_status_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased Interfaces.Unsigned_64;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_attempt_v1 is record
      ordinal : aliased Interfaces.Unsigned_64;
      outcome : access constant thinkthen_complete_attempt_outcome_v1;
      request_id : aliased thinkthen_complete_attempt_field_request_id_presence_v1;
      request_sha256 : aliased thinkthen_complete_utf8_v1;
      sdk_request_id : access constant thinkthen_complete_sdk_request_id_v1;
      server_ms : aliased thinkthen_complete_attempt_field_server_ms_presence_v1;
      status : aliased thinkthen_complete_attempt_field_status_presence_v1;
      wall_ms : aliased Interfaces.Unsigned_64;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_meta_field_attempts_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_meta_field_attempts_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_meta_field_attempts_v1;
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
   type thinkthen_complete_meta_field_batch_setting_presence_v1 is record
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
   type thinkthen_complete_meta_field_batch_warning_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_batch_warning_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_meta_field_context_sha_256_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_utf8_v1;
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
   type thinkthen_complete_meta_field_observations_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_origin_v1 is record
      kind : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_meta_field_origin_presence_v1 is record
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
   type thinkthen_complete_meta_field_profile_warning_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_profile_warning_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_meta_field_question_sha_256_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_utf8_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_question_source_field_batch_size_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased Interfaces.Unsigned_64;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_question_source_v1 is record
      answered_by : aliased thinkthen_complete_utf8_v1;
      batch_size : aliased thinkthen_complete_question_source_field_batch_size_presence_v1;
      origin : access constant thinkthen_complete_origin_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_meta_field_question_sources_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_meta_field_questions_sha_256_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_utf8_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_meta_field_requests_v1 is record
      data : access constant thinkthen_complete_utf8_v1;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_usage_field_input_tokens_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased Interfaces.Unsigned_64;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_usage_field_output_tokens_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased Interfaces.Unsigned_64;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_usage_v1 is record
      input_tokens : aliased thinkthen_complete_usage_field_input_tokens_presence_v1;
      output_tokens : aliased thinkthen_complete_usage_field_output_tokens_presence_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_meta_field_usage_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_usage_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_meta_v1 is record
      answered_by : aliased thinkthen_complete_meta_field_answered_by_presence_v1;
      attempts : aliased thinkthen_complete_meta_field_attempts_presence_v1;
      batch_setting : aliased thinkthen_complete_meta_field_batch_setting_presence_v1;
      batch_warning : aliased thinkthen_complete_meta_field_batch_warning_presence_v1;
      cached : aliased Interfaces.Unsigned_32;
      context_sha256 : aliased thinkthen_complete_meta_field_context_sha_256_presence_v1;
      failed_questions : aliased Interfaces.Unsigned_64;
      model : aliased thinkthen_complete_utf8_v1;
      observations : aliased thinkthen_complete_meta_field_observations_v1;
      origin : aliased thinkthen_complete_meta_field_origin_presence_v1;
      profile_warning : aliased thinkthen_complete_meta_field_profile_warning_presence_v1;
      question_sha256 : aliased thinkthen_complete_meta_field_question_sha_256_presence_v1;
      question_sources : aliased thinkthen_complete_meta_field_question_sources_v1;
      questions_sha256 : aliased thinkthen_complete_meta_field_questions_sha_256_presence_v1;
      requests : aliased thinkthen_complete_meta_field_requests_v1;
      requests_sent : aliased Interfaces.Unsigned_64;
      tool : aliased thinkthen_complete_utf8_v1;
      url : aliased thinkthen_complete_utf8_v1;
      usage : aliased thinkthen_complete_meta_field_usage_presence_v1;
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
   type thinkthen_complete_readable_question_decide_field_batch_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_batch_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_string_type_v1 is record
      kind : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_input_declaration_string_v1 is record
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
   type thinkthen_complete_input_declaration_object_field_properties_entry_v1 is record
      name : aliased thinkthen_complete_utf8_v1;
      value : access constant thinkthen_complete_input_property_type_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_input_declaration_object_field_properties_v1 is record
      data : access constant thinkthen_complete_input_declaration_object_field_properties_entry_v1;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_input_declaration_object_field_required_v1 is record
      data : access constant thinkthen_complete_utf8_v1;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_input_declaration_object_field_required_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_input_declaration_object_field_required_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_object_type_v1 is record
      kind : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_input_declaration_object_v1 is record
      properties : aliased thinkthen_complete_input_declaration_object_field_properties_v1;
      required : aliased thinkthen_complete_input_declaration_object_field_required_presence_v1;
      c_type : access constant thinkthen_complete_object_type_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_input_declaration_data_v1 (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            string : access constant thinkthen_complete_input_declaration_string_v1;
         when others =>
            object : access constant thinkthen_complete_input_declaration_object_v1;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_input_declaration_v1 is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_input_declaration_data_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_decide_field_context_schema_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_input_declaration_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_decide_field_item_schema_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_input_declaration_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_label_field_description_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_json_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_label_v1 is record
      description : aliased thinkthen_complete_label_field_description_presence_v1;
      name : aliased thinkthen_complete_utf8_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_decide_field_label_details_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_decide_field_label_details_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_readable_question_decide_field_label_details_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_decide_field_model_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_utf8_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_question_name_v1 is record
      value : aliased thinkthen_complete_utf8_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_decide_field_name_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_question_name_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_decide_field_on_v1 is record
      data : access constant thinkthen_complete_utf8_v1;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_decide_field_on_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_readable_question_decide_field_on_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_decide_field_profile_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_utf8_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_wording_version_v1 is record
      value : aliased Interfaces.Unsigned_64;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_decide_field_wording_version_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_wording_version_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_decide_field_false_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_json_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_decide_field_text_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_json_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_decide_field_true_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_json_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_decide_v1 is record
      batch : aliased thinkthen_complete_readable_question_decide_field_batch_presence_v1;
      context_schema : aliased thinkthen_complete_readable_question_decide_field_context_schema_presence_v1;
      item_schema : aliased thinkthen_complete_readable_question_decide_field_item_schema_presence_v1;
      label_details : aliased thinkthen_complete_readable_question_decide_field_label_details_presence_v1;
      model : aliased thinkthen_complete_readable_question_decide_field_model_presence_v1;
      name : aliased thinkthen_complete_readable_question_decide_field_name_presence_v1;
      on : aliased thinkthen_complete_readable_question_decide_field_on_presence_v1;
      profile : aliased thinkthen_complete_readable_question_decide_field_profile_presence_v1;
      wording_version : aliased thinkthen_complete_readable_question_decide_field_wording_version_presence_v1;
      false_u : aliased thinkthen_complete_readable_question_decide_field_false_presence_v1;
      text : aliased thinkthen_complete_readable_question_decide_field_text_presence_v1;
      true_u : aliased thinkthen_complete_readable_question_decide_field_true_presence_v1;
      verb : aliased thinkthen_complete_utf8_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_choose_field_batch_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_batch_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_choose_field_context_schema_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_input_declaration_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_choose_field_item_schema_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_input_declaration_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_choose_field_label_details_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_choose_field_label_details_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_readable_question_choose_field_label_details_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_choose_field_model_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_utf8_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_choose_field_name_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_question_name_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_choose_field_on_v1 is record
      data : access constant thinkthen_complete_utf8_v1;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_choose_field_on_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_readable_question_choose_field_on_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_choose_field_profile_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_utf8_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_choose_field_wording_version_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_wording_version_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_choose_field_options_v1 is record
      data : access constant thinkthen_complete_utf8_v1;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_choose_field_text_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_json_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_choose_v1 is record
      batch : aliased thinkthen_complete_readable_question_choose_field_batch_presence_v1;
      context_schema : aliased thinkthen_complete_readable_question_choose_field_context_schema_presence_v1;
      item_schema : aliased thinkthen_complete_readable_question_choose_field_item_schema_presence_v1;
      label_details : aliased thinkthen_complete_readable_question_choose_field_label_details_presence_v1;
      model : aliased thinkthen_complete_readable_question_choose_field_model_presence_v1;
      name : aliased thinkthen_complete_readable_question_choose_field_name_presence_v1;
      on : aliased thinkthen_complete_readable_question_choose_field_on_presence_v1;
      profile : aliased thinkthen_complete_readable_question_choose_field_profile_presence_v1;
      wording_version : aliased thinkthen_complete_readable_question_choose_field_wording_version_presence_v1;
      options : aliased thinkthen_complete_readable_question_choose_field_options_v1;
      text : aliased thinkthen_complete_readable_question_choose_field_text_presence_v1;
      verb : aliased thinkthen_complete_utf8_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_tag_field_batch_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_batch_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_tag_field_context_schema_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_input_declaration_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_tag_field_item_schema_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_input_declaration_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_tag_field_label_details_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_tag_field_label_details_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_readable_question_tag_field_label_details_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_tag_field_model_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_utf8_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_tag_field_name_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_question_name_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_tag_field_on_v1 is record
      data : access constant thinkthen_complete_utf8_v1;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_tag_field_on_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_readable_question_tag_field_on_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_tag_field_profile_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_utf8_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_tag_field_wording_version_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_wording_version_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_tag_field_labels_v1 is record
      data : access constant thinkthen_complete_utf8_v1;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_tag_field_text_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_json_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_tag_v1 is record
      batch : aliased thinkthen_complete_readable_question_tag_field_batch_presence_v1;
      context_schema : aliased thinkthen_complete_readable_question_tag_field_context_schema_presence_v1;
      item_schema : aliased thinkthen_complete_readable_question_tag_field_item_schema_presence_v1;
      label_details : aliased thinkthen_complete_readable_question_tag_field_label_details_presence_v1;
      model : aliased thinkthen_complete_readable_question_tag_field_model_presence_v1;
      name : aliased thinkthen_complete_readable_question_tag_field_name_presence_v1;
      on : aliased thinkthen_complete_readable_question_tag_field_on_presence_v1;
      profile : aliased thinkthen_complete_readable_question_tag_field_profile_presence_v1;
      wording_version : aliased thinkthen_complete_readable_question_tag_field_wording_version_presence_v1;
      labels : aliased thinkthen_complete_readable_question_tag_field_labels_v1;
      text : aliased thinkthen_complete_readable_question_tag_field_text_presence_v1;
      verb : aliased thinkthen_complete_utf8_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_score_field_batch_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_batch_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_score_field_context_schema_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_input_declaration_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_score_field_item_schema_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_input_declaration_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_score_field_label_details_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_score_field_label_details_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_readable_question_score_field_label_details_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_score_field_model_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_utf8_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_score_field_name_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_question_name_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_score_field_on_v1 is record
      data : access constant thinkthen_complete_utf8_v1;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_score_field_on_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_readable_question_score_field_on_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_score_field_profile_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_utf8_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_score_field_wording_version_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_wording_version_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_score_field_levels_v1 is record
      data : access constant thinkthen_complete_utf8_v1;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_score_field_text_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_json_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_score_v1 is record
      batch : aliased thinkthen_complete_readable_question_score_field_batch_presence_v1;
      context_schema : aliased thinkthen_complete_readable_question_score_field_context_schema_presence_v1;
      item_schema : aliased thinkthen_complete_readable_question_score_field_item_schema_presence_v1;
      label_details : aliased thinkthen_complete_readable_question_score_field_label_details_presence_v1;
      model : aliased thinkthen_complete_readable_question_score_field_model_presence_v1;
      name : aliased thinkthen_complete_readable_question_score_field_name_presence_v1;
      on : aliased thinkthen_complete_readable_question_score_field_on_presence_v1;
      profile : aliased thinkthen_complete_readable_question_score_field_profile_presence_v1;
      wording_version : aliased thinkthen_complete_readable_question_score_field_wording_version_presence_v1;
      levels : aliased thinkthen_complete_readable_question_score_field_levels_v1;
      text : aliased thinkthen_complete_readable_question_score_field_text_presence_v1;
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
   type thinkthen_complete_physical_source_field_first_line_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased Interfaces.Unsigned_64;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_physical_source_field_last_line_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased Interfaces.Unsigned_64;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_physical_source_v1 is record
      file : aliased thinkthen_complete_utf8_v1;
      first_line : aliased thinkthen_complete_physical_source_field_first_line_presence_v1;
      last_line : aliased thinkthen_complete_physical_source_field_last_line_presence_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_rank_member_result_field_source_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_physical_source_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_rank_member_result_field_threshold_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_json_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_rank_member_result_v1 is record
      answer : access constant thinkthen_complete_answer_v1;
      answer_id : access constant thinkthen_complete_answer_id_v1;
      images : aliased thinkthen_complete_rank_member_result_field_images_presence_v1;
      meta : access constant thinkthen_complete_meta_v1;
      question : access constant thinkthen_complete_readable_question_v1;
      schema : access constant thinkthen_complete_version_v1;
      source : aliased thinkthen_complete_rank_member_result_field_source_presence_v1;
      threshold : aliased thinkthen_complete_rank_member_result_field_threshold_presence_v1;
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
   type thinkthen_complete_atomic_decide_value_field_members_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_atomic_decide_value_field_members_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_atomic_decide_value_field_members_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_atomic_decide_value_field_question_name_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_utf8_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_atomic_decide_value_field_source_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_physical_source_v1;
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
   type thinkthen_complete_atomic_decide_value_field_threshold_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_threshold_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_decide_value_v1 is record
      value : access constant thinkthen_complete_json_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_atomic_decide_value_field_value_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_decide_value_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_atomic_decide_value_v1 is record
      answer : access constant thinkthen_complete_answer_v1;
      answer_id : access constant thinkthen_complete_answer_id_v1;
      images : aliased thinkthen_complete_atomic_decide_value_field_images_presence_v1;
      index : aliased thinkthen_complete_atomic_decide_value_field_index_presence_v1;
      input : aliased thinkthen_complete_atomic_decide_value_field_input_presence_v1;
      members : aliased thinkthen_complete_atomic_decide_value_field_members_presence_v1;
      meta : access constant thinkthen_complete_meta_v1;
      question : access constant thinkthen_complete_readable_question_v1;
      question_name : aliased thinkthen_complete_atomic_decide_value_field_question_name_presence_v1;
      schema : access constant thinkthen_complete_version_v1;
      source : aliased thinkthen_complete_atomic_decide_value_field_source_presence_v1;
      threshold : aliased thinkthen_complete_atomic_decide_value_field_threshold_presence_v1;
      value : aliased thinkthen_complete_atomic_decide_value_field_value_presence_v1;
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
   type thinkthen_complete_atomic_nullable_string_field_images_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_atomic_nullable_string_field_images_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_atomic_nullable_string_field_images_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_atomic_nullable_string_field_index_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased Interfaces.Unsigned_64;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_atomic_nullable_string_field_input_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_json_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_atomic_nullable_string_field_members_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_atomic_nullable_string_field_members_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_atomic_nullable_string_field_members_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_atomic_nullable_string_field_question_name_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_utf8_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_atomic_nullable_string_field_source_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_physical_source_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_atomic_nullable_string_field_threshold_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_threshold_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_atomic_nullable_string_field_value_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_utf8_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_atomic_nullable_string_v1 is record
      answer : access constant thinkthen_complete_answer_v1;
      answer_id : access constant thinkthen_complete_answer_id_v1;
      images : aliased thinkthen_complete_atomic_nullable_string_field_images_presence_v1;
      index : aliased thinkthen_complete_atomic_nullable_string_field_index_presence_v1;
      input : aliased thinkthen_complete_atomic_nullable_string_field_input_presence_v1;
      members : aliased thinkthen_complete_atomic_nullable_string_field_members_presence_v1;
      meta : access constant thinkthen_complete_meta_v1;
      question : access constant thinkthen_complete_readable_question_v1;
      question_name : aliased thinkthen_complete_atomic_nullable_string_field_question_name_presence_v1;
      schema : access constant thinkthen_complete_version_v1;
      source : aliased thinkthen_complete_atomic_nullable_string_field_source_presence_v1;
      threshold : aliased thinkthen_complete_atomic_nullable_string_field_threshold_presence_v1;
      value : aliased thinkthen_complete_atomic_nullable_string_field_value_presence_v1;
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
   type thinkthen_complete_atomic_array_of_string_field_images_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_atomic_array_of_string_field_images_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_atomic_array_of_string_field_images_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_atomic_array_of_string_field_index_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased Interfaces.Unsigned_64;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_atomic_array_of_string_field_input_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_json_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_atomic_array_of_string_field_members_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_atomic_array_of_string_field_members_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_atomic_array_of_string_field_members_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_atomic_array_of_string_field_question_name_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_utf8_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_atomic_array_of_string_field_source_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_physical_source_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_atomic_array_of_string_field_threshold_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_threshold_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_atomic_array_of_string_field_value_v1 is record
      data : access constant thinkthen_complete_utf8_v1;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_atomic_array_of_string_v1 is record
      answer : access constant thinkthen_complete_answer_v1;
      answer_id : access constant thinkthen_complete_answer_id_v1;
      images : aliased thinkthen_complete_atomic_array_of_string_field_images_presence_v1;
      index : aliased thinkthen_complete_atomic_array_of_string_field_index_presence_v1;
      input : aliased thinkthen_complete_atomic_array_of_string_field_input_presence_v1;
      members : aliased thinkthen_complete_atomic_array_of_string_field_members_presence_v1;
      meta : access constant thinkthen_complete_meta_v1;
      question : access constant thinkthen_complete_readable_question_v1;
      question_name : aliased thinkthen_complete_atomic_array_of_string_field_question_name_presence_v1;
      schema : access constant thinkthen_complete_version_v1;
      source : aliased thinkthen_complete_atomic_array_of_string_field_source_presence_v1;
      threshold : aliased thinkthen_complete_atomic_array_of_string_field_threshold_presence_v1;
      value : aliased thinkthen_complete_atomic_array_of_string_field_value_v1;
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
   type thinkthen_complete_atomic_double_field_images_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_atomic_double_field_images_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_atomic_double_field_images_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_atomic_double_field_index_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased Interfaces.Unsigned_64;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_atomic_double_field_input_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_json_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_atomic_double_field_members_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_atomic_double_field_members_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_atomic_double_field_members_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_atomic_double_field_question_name_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_utf8_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_atomic_double_field_source_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_physical_source_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_atomic_double_field_threshold_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_threshold_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_atomic_double_v1 is record
      answer : access constant thinkthen_complete_answer_v1;
      answer_id : access constant thinkthen_complete_answer_id_v1;
      images : aliased thinkthen_complete_atomic_double_field_images_presence_v1;
      index : aliased thinkthen_complete_atomic_double_field_index_presence_v1;
      input : aliased thinkthen_complete_atomic_double_field_input_presence_v1;
      members : aliased thinkthen_complete_atomic_double_field_members_presence_v1;
      meta : access constant thinkthen_complete_meta_v1;
      question : access constant thinkthen_complete_readable_question_v1;
      question_name : aliased thinkthen_complete_atomic_double_field_question_name_presence_v1;
      schema : access constant thinkthen_complete_version_v1;
      source : aliased thinkthen_complete_atomic_double_field_source_presence_v1;
      threshold : aliased thinkthen_complete_atomic_double_field_threshold_presence_v1;
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
   type thinkthen_complete_atomic_boolean_field_images_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_atomic_boolean_field_images_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_atomic_boolean_field_images_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_atomic_boolean_field_index_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased Interfaces.Unsigned_64;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_atomic_boolean_field_input_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_json_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_atomic_boolean_field_members_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_atomic_boolean_field_members_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_atomic_boolean_field_members_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_atomic_boolean_field_question_name_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_utf8_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_atomic_boolean_field_source_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_physical_source_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_atomic_boolean_field_threshold_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_threshold_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_atomic_boolean_v1 is record
      answer : access constant thinkthen_complete_answer_v1;
      answer_id : access constant thinkthen_complete_answer_id_v1;
      images : aliased thinkthen_complete_atomic_boolean_field_images_presence_v1;
      index : aliased thinkthen_complete_atomic_boolean_field_index_presence_v1;
      input : aliased thinkthen_complete_atomic_boolean_field_input_presence_v1;
      members : aliased thinkthen_complete_atomic_boolean_field_members_presence_v1;
      meta : access constant thinkthen_complete_meta_v1;
      question : access constant thinkthen_complete_readable_question_v1;
      question_name : aliased thinkthen_complete_atomic_boolean_field_question_name_presence_v1;
      schema : access constant thinkthen_complete_version_v1;
      source : aliased thinkthen_complete_atomic_boolean_field_source_presence_v1;
      threshold : aliased thinkthen_complete_atomic_boolean_field_threshold_presence_v1;
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
   type thinkthen_complete_annotation_member_answer_id_field_observations_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_annotation_member_answer_id_field_question_sources_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_annotation_member_answer_id_field_threshold_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_threshold_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_annotation_member_answer_id_field_usage_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_usage_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_value_array_v1 is record
      data : access constant thinkthen_complete_utf8_v1;
      len : aliased Interfaces.C.size_t;
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
            c_array : aliased thinkthen_complete_value_array_v1;
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
   type thinkthen_complete_annotation_member_answer_id_field_value_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_value_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_annotation_member_answer_id_v1 is record
      answer : access constant thinkthen_complete_answer_v1;
      answer_id : access constant thinkthen_complete_answer_id_v1;
      observations : aliased thinkthen_complete_annotation_member_answer_id_field_observations_v1;
      question : access constant thinkthen_complete_readable_question_v1;
      question_sources : aliased thinkthen_complete_annotation_member_answer_id_field_question_sources_v1;
      request : aliased thinkthen_complete_utf8_v1;
      threshold : aliased thinkthen_complete_annotation_member_answer_id_field_threshold_presence_v1;
      usage : aliased thinkthen_complete_annotation_member_answer_id_field_usage_presence_v1;
      value : aliased thinkthen_complete_annotation_member_answer_id_field_value_presence_v1;
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
   type thinkthen_complete_annotation_member_failure_id_field_observations_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_annotation_member_failure_id_field_question_sources_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_annotation_member_failure_id_field_threshold_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_threshold_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_annotation_member_failure_id_field_usage_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_usage_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_annotation_member_failure_id_v1 is record
      failure : access constant thinkthen_complete_failure_v1;
      failure_id : access constant thinkthen_complete_failure_id_v1;
      observations : aliased thinkthen_complete_annotation_member_failure_id_field_observations_v1;
      question : access constant thinkthen_complete_readable_question_v1;
      question_sources : aliased thinkthen_complete_annotation_member_failure_id_field_question_sources_v1;
      request : aliased thinkthen_complete_utf8_v1;
      threshold : aliased thinkthen_complete_annotation_member_failure_id_field_threshold_presence_v1;
      usage : aliased thinkthen_complete_annotation_member_failure_id_field_usage_presence_v1;
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
   type thinkthen_complete_annotation_field_answers_entry_v1 is record
      name : aliased thinkthen_complete_utf8_v1;
      value : access constant thinkthen_complete_annotation_member_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_annotation_field_answers_v1 is record
      data : access constant thinkthen_complete_annotation_field_answers_entry_v1;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_annotation_field_file_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_utf8_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_annotation_field_first_line_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased Interfaces.Unsigned_64;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_annotation_field_index_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased Interfaces.Unsigned_64;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_annotation_field_input_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_json_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_annotation_field_last_line_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased Interfaces.Unsigned_64;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_position_field_file_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_utf8_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_position_field_first_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased Interfaces.Unsigned_64;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_position_field_images_v1 is record
      data : access constant thinkthen_complete_utf8_v1;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_position_field_images_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_position_field_images_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_position_field_last_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased Interfaces.Unsigned_64;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_position_v1 is record
      file : aliased thinkthen_complete_position_field_file_presence_v1;
      first : aliased thinkthen_complete_position_field_first_presence_v1;
      images : aliased thinkthen_complete_position_field_images_presence_v1;
      last : aliased thinkthen_complete_position_field_last_presence_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_annotation_field_position_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_position_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_annotation_field_source_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_physical_source_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_annotated_field_array_v1 is record
      data : access constant thinkthen_complete_utf8_v1;
      len : aliased Interfaces.C.size_t;
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
            c_array : aliased thinkthen_complete_annotated_field_array_v1;
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
   type thinkthen_complete_annotated_row_value_entry_v1 is record
      name : aliased thinkthen_complete_utf8_v1;
      value : access constant thinkthen_complete_annotated_field_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_annotated_row_value_v1 is record
      data : access constant thinkthen_complete_annotated_row_value_entry_v1;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_annotated_row_v1 is record
      value : aliased thinkthen_complete_annotated_row_value_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_annotation_v1 is record
      answer_id : access constant thinkthen_complete_answer_id_v1;
      answers : aliased thinkthen_complete_annotation_field_answers_v1;
      file : aliased thinkthen_complete_annotation_field_file_presence_v1;
      first_line : aliased thinkthen_complete_annotation_field_first_line_presence_v1;
      index : aliased thinkthen_complete_annotation_field_index_presence_v1;
      input : aliased thinkthen_complete_annotation_field_input_presence_v1;
      last_line : aliased thinkthen_complete_annotation_field_last_line_presence_v1;
      meta : access constant thinkthen_complete_meta_v1;
      position : aliased thinkthen_complete_annotation_field_position_presence_v1;
      schema : access constant thinkthen_complete_version_v1;
      source : aliased thinkthen_complete_annotation_field_source_presence_v1;
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
   type thinkthen_complete_session_packet_decide_aggregate_field_value_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_packet_decide_aggregate_v1 is record
      c_function : aliased thinkthen_complete_utf8_v1;
      kind : aliased thinkthen_complete_utf8_v1;
      value : aliased thinkthen_complete_session_packet_decide_aggregate_field_value_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_packet_choose_aggregate_field_value_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_packet_choose_aggregate_v1 is record
      c_function : aliased thinkthen_complete_utf8_v1;
      kind : aliased thinkthen_complete_utf8_v1;
      value : aliased thinkthen_complete_session_packet_choose_aggregate_field_value_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_packet_tag_aggregate_field_value_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_packet_tag_aggregate_v1 is record
      c_function : aliased thinkthen_complete_utf8_v1;
      kind : aliased thinkthen_complete_utf8_v1;
      value : aliased thinkthen_complete_session_packet_tag_aggregate_field_value_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_packet_score_aggregate_field_value_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_packet_score_aggregate_v1 is record
      c_function : aliased thinkthen_complete_utf8_v1;
      kind : aliased thinkthen_complete_utf8_v1;
      value : aliased thinkthen_complete_session_packet_score_aggregate_field_value_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_packet_filter_aggregate_field_value_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_packet_filter_aggregate_v1 is record
      c_function : aliased thinkthen_complete_utf8_v1;
      kind : aliased thinkthen_complete_utf8_v1;
      value : aliased thinkthen_complete_session_packet_filter_aggregate_field_value_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_atomic_non_zero_usize_field_images_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_atomic_non_zero_usize_field_images_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_atomic_non_zero_usize_field_images_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_atomic_non_zero_usize_field_index_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased Interfaces.Unsigned_64;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_atomic_non_zero_usize_field_input_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_json_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_atomic_non_zero_usize_field_members_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_atomic_non_zero_usize_field_members_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_atomic_non_zero_usize_field_members_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_atomic_non_zero_usize_field_question_name_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_utf8_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_atomic_non_zero_usize_field_source_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_physical_source_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_atomic_non_zero_usize_field_threshold_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_threshold_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_atomic_non_zero_usize_v1 is record
      answer : access constant thinkthen_complete_answer_v1;
      answer_id : access constant thinkthen_complete_answer_id_v1;
      images : aliased thinkthen_complete_atomic_non_zero_usize_field_images_presence_v1;
      index : aliased thinkthen_complete_atomic_non_zero_usize_field_index_presence_v1;
      input : aliased thinkthen_complete_atomic_non_zero_usize_field_input_presence_v1;
      members : aliased thinkthen_complete_atomic_non_zero_usize_field_members_presence_v1;
      meta : access constant thinkthen_complete_meta_v1;
      question : access constant thinkthen_complete_readable_question_v1;
      question_name : aliased thinkthen_complete_atomic_non_zero_usize_field_question_name_presence_v1;
      schema : access constant thinkthen_complete_version_v1;
      source : aliased thinkthen_complete_atomic_non_zero_usize_field_source_presence_v1;
      threshold : aliased thinkthen_complete_atomic_non_zero_usize_field_threshold_presence_v1;
      value : aliased Interfaces.Unsigned_64;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_packet_rank_aggregate_field_value_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_packet_rank_aggregate_v1 is record
      c_function : aliased thinkthen_complete_utf8_v1;
      kind : aliased thinkthen_complete_utf8_v1;
      value : aliased thinkthen_complete_session_packet_rank_aggregate_field_value_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_find_answer_field_confidence_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased double;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_find_answer_field_probabilities_entry_v1 is record
      name : aliased thinkthen_complete_utf8_v1;
      value : aliased double;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_find_answer_field_probabilities_v1 is record
      data : access constant thinkthen_complete_find_answer_field_probabilities_entry_v1;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_find_answer_v1 is record
      confidence : aliased thinkthen_complete_find_answer_field_confidence_presence_v1;
      kind : aliased thinkthen_complete_utf8_v1;
      pick : aliased thinkthen_complete_utf8_v1;
      probabilities : aliased thinkthen_complete_find_answer_field_probabilities_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_find_candidate_field_index_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased Interfaces.Unsigned_64;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_find_candidate_field_input_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_json_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_find_candidate_field_source_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_physical_source_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_find_candidate_v1 is record
      index : aliased thinkthen_complete_find_candidate_field_index_presence_v1;
      input : aliased thinkthen_complete_find_candidate_field_input_presence_v1;
      probability : aliased double;
      source : aliased thinkthen_complete_find_candidate_field_source_presence_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_find_field_candidates_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_find_field_candidates_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_find_field_candidates_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_find_field_file_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_utf8_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_find_field_first_line_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased Interfaces.Unsigned_64;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_find_field_index_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased Interfaces.Unsigned_64;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_find_field_last_line_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased Interfaces.Unsigned_64;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_find_field_position_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_position_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_2_field_batch_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_batch_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_2_field_context_schema_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_input_declaration_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_2_field_item_schema_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_input_declaration_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_2_field_label_details_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_2_field_label_details_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_readable_question_2_field_label_details_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_2_field_model_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_utf8_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_2_field_name_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_question_name_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_2_field_on_v1 is record
      data : access constant thinkthen_complete_utf8_v1;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_2_field_on_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_readable_question_2_field_on_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_2_field_profile_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_utf8_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_2_field_text_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_json_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_2_field_wording_version_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_wording_version_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_2_v1 is record
      batch : aliased thinkthen_complete_readable_question_2_field_batch_presence_v1;
      context_schema : aliased thinkthen_complete_readable_question_2_field_context_schema_presence_v1;
      item_schema : aliased thinkthen_complete_readable_question_2_field_item_schema_presence_v1;
      label_details : aliased thinkthen_complete_readable_question_2_field_label_details_presence_v1;
      model : aliased thinkthen_complete_readable_question_2_field_model_presence_v1;
      name : aliased thinkthen_complete_readable_question_2_field_name_presence_v1;
      none : aliased Interfaces.Unsigned_32;
      on : aliased thinkthen_complete_readable_question_2_field_on_presence_v1;
      profile : aliased thinkthen_complete_readable_question_2_field_profile_presence_v1;
      text : aliased thinkthen_complete_readable_question_2_field_text_presence_v1;
      verb : aliased thinkthen_complete_utf8_v1;
      wording_version : aliased thinkthen_complete_readable_question_2_field_wording_version_presence_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_find_field_threshold_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_json_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_find_field_value_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_json_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_find_v1 is record
      answer : access constant thinkthen_complete_find_answer_v1;
      answer_id : access constant thinkthen_complete_answer_id_v1;
      candidates : aliased thinkthen_complete_find_field_candidates_presence_v1;
      file : aliased thinkthen_complete_find_field_file_presence_v1;
      first_line : aliased thinkthen_complete_find_field_first_line_presence_v1;
      index : aliased thinkthen_complete_find_field_index_presence_v1;
      last_line : aliased thinkthen_complete_find_field_last_line_presence_v1;
      meta : access constant thinkthen_complete_meta_v1;
      position : aliased thinkthen_complete_find_field_position_presence_v1;
      question : access constant thinkthen_complete_readable_question_2_v1;
      schema : access constant thinkthen_complete_version_v1;
      threshold : aliased thinkthen_complete_find_field_threshold_presence_v1;
      value : aliased thinkthen_complete_find_field_value_presence_v1;
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
   type thinkthen_complete_session_packet_annotate_aggregate_field_value_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_packet_annotate_aggregate_v1 is record
      c_function : aliased thinkthen_complete_utf8_v1;
      kind : aliased thinkthen_complete_utf8_v1;
      value : aliased thinkthen_complete_session_packet_annotate_aggregate_field_value_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_name_odds_field_edges_entry_v1 is record
      name : aliased thinkthen_complete_utf8_v1;
      value : aliased double;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_name_odds_field_edges_v1 is record
      data : access constant thinkthen_complete_name_odds_field_edges_entry_v1;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_name_odds_field_edges_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_name_odds_field_edges_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_name_odds_field_kinds_entry_v1 is record
      name : aliased thinkthen_complete_utf8_v1;
      value : aliased double;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_name_odds_field_kinds_v1 is record
      data : access constant thinkthen_complete_name_odds_field_kinds_entry_v1;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_name_odds_field_kinds_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_name_odds_field_kinds_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_name_odds_v1 is record
      edges : aliased thinkthen_complete_name_odds_field_edges_presence_v1;
      c_end : aliased Interfaces.Unsigned_64;
      kinds : aliased thinkthen_complete_name_odds_field_kinds_presence_v1;
      start : aliased Interfaces.Unsigned_64;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_recognition_odds_fields_names_pairs_pieces_proposals_field_names_v1 is record
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
   type thinkthen_complete_recognition_odds_fields_names_pairs_pieces_proposals_field_pairs_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_piece_odds_field_tags_entry_v1 is record
      name : aliased thinkthen_complete_utf8_v1;
      value : aliased double;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_piece_odds_field_tags_v1 is record
      data : access constant thinkthen_complete_piece_odds_field_tags_entry_v1;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_piece_odds_v1 is record
      c_end : aliased Interfaces.Unsigned_64;
      start : aliased Interfaces.Unsigned_64;
      tags : aliased thinkthen_complete_piece_odds_field_tags_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_recognition_odds_fields_names_pairs_pieces_proposals_field_pieces_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_recognition_proposal_field_kind_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_utf8_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_recognition_proposal_field_selected_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_place_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_recognition_proposal_field_strength_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased double;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_recognition_proposal_v1 is record
      c_end : aliased Interfaces.Unsigned_64;
      kept : aliased Interfaces.Unsigned_32;
      kind : aliased thinkthen_complete_recognition_proposal_field_kind_presence_v1;
      selected : aliased thinkthen_complete_recognition_proposal_field_selected_presence_v1;
      span_probability : aliased double;
      start : aliased Interfaces.Unsigned_64;
      strength : aliased thinkthen_complete_recognition_proposal_field_strength_presence_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_recognition_odds_fields_names_pairs_pieces_proposals_field_proposals_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_recognition_odds_fields_names_pairs_pieces_proposals_v1 is record
      names : aliased thinkthen_complete_recognition_odds_fields_names_pairs_pieces_proposals_field_names_v1;
      pairs : aliased thinkthen_complete_recognition_odds_fields_names_pairs_pieces_proposals_field_pairs_v1;
      pieces : aliased thinkthen_complete_recognition_odds_fields_names_pairs_pieces_proposals_field_pieces_v1;
      proposals : aliased thinkthen_complete_recognition_odds_fields_names_pairs_pieces_proposals_field_proposals_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_recognition_odds_fields_pieces_proposals_field_pieces_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
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
   type thinkthen_complete_recognition_odds_fields_pieces_proposals_field_proposals_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_recognition_odds_fields_pieces_proposals_v1 is record
      pieces : aliased thinkthen_complete_recognition_odds_fields_pieces_proposals_field_pieces_v1;
      proposals : aliased thinkthen_complete_recognition_odds_fields_pieces_proposals_field_proposals_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_recognition_odds_data_v1 (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            fields_names_pairs_pieces_proposals : access constant thinkthen_complete_recognition_odds_fields_names_pairs_pieces_proposals_v1;
         when others =>
            fields_pieces_proposals : access constant thinkthen_complete_recognition_odds_fields_pieces_proposals_v1;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_recognition_odds_v1 is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_recognition_odds_data_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_recognition_field_file_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_utf8_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_recognition_field_first_line_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased Interfaces.Unsigned_64;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_recognition_field_index_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased Interfaces.Unsigned_64;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_recognition_field_input_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_json_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_recognition_field_last_line_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased Interfaces.Unsigned_64;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_recognition_field_position_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_position_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_3_field_batch_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_batch_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_3_field_context_schema_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_input_declaration_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_3_field_entity_definition_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_json_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_3_field_instructions_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_json_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_3_field_item_schema_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_input_declaration_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_3_field_kinds_entry_v1 is record
      name : aliased thinkthen_complete_utf8_v1;
      value : access constant thinkthen_complete_json_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_3_field_kinds_v1 is record
      data : access constant thinkthen_complete_readable_question_3_field_kinds_entry_v1;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_3_field_label_details_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_3_field_label_details_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_readable_question_3_field_label_details_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_recognition_mode_v1 is record
      kind : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_3_field_mode_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_recognition_mode_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_3_field_model_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_utf8_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_3_field_name_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_question_name_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_3_field_on_v1 is record
      data : access constant thinkthen_complete_utf8_v1;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_3_field_on_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_readable_question_3_field_on_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_3_field_profile_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_utf8_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_3_field_relation_threshold_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_threshold_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_relation_rule_field_single_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_relation_rule_v1 is record
      either : aliased Interfaces.Unsigned_32;
      name : aliased thinkthen_complete_utf8_v1;
      reads : aliased thinkthen_complete_utf8_v1;
      single : aliased thinkthen_complete_relation_rule_field_single_presence_v1;
      source : aliased thinkthen_complete_utf8_v1;
      target : aliased thinkthen_complete_utf8_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_3_field_relations_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_3_field_relations_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_readable_question_3_field_relations_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_3_field_snippet_pieces_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased Interfaces.Unsigned_64;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_recognition_stage_context_field_boundary_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_utf8_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_recognition_stage_context_field_kind_edge_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_utf8_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_recognition_stage_context_field_relation_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_utf8_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_recognition_stage_context_v1 is record
      boundary : aliased thinkthen_complete_recognition_stage_context_field_boundary_presence_v1;
      kind_edge : aliased thinkthen_complete_recognition_stage_context_field_kind_edge_presence_v1;
      relation : aliased thinkthen_complete_recognition_stage_context_field_relation_presence_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_3_field_stage_context_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_recognition_stage_context_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_verb_v1 is record
      kind : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_3_field_wording_version_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_wording_version_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_3_v1 is record
      batch : aliased thinkthen_complete_readable_question_3_field_batch_presence_v1;
      context_schema : aliased thinkthen_complete_readable_question_3_field_context_schema_presence_v1;
      entity_definition : aliased thinkthen_complete_readable_question_3_field_entity_definition_presence_v1;
      instructions : aliased thinkthen_complete_readable_question_3_field_instructions_presence_v1;
      item_schema : aliased thinkthen_complete_readable_question_3_field_item_schema_presence_v1;
      kinds : aliased thinkthen_complete_readable_question_3_field_kinds_v1;
      label_details : aliased thinkthen_complete_readable_question_3_field_label_details_presence_v1;
      mode : aliased thinkthen_complete_readable_question_3_field_mode_presence_v1;
      model : aliased thinkthen_complete_readable_question_3_field_model_presence_v1;
      name : aliased thinkthen_complete_readable_question_3_field_name_presence_v1;
      on : aliased thinkthen_complete_readable_question_3_field_on_presence_v1;
      profile : aliased thinkthen_complete_readable_question_3_field_profile_presence_v1;
      relation_threshold : aliased thinkthen_complete_readable_question_3_field_relation_threshold_presence_v1;
      relations : aliased thinkthen_complete_readable_question_3_field_relations_presence_v1;
      snippet_pieces : aliased thinkthen_complete_readable_question_3_field_snippet_pieces_presence_v1;
      stage_context : aliased thinkthen_complete_readable_question_3_field_stage_context_presence_v1;
      threshold : access constant thinkthen_complete_threshold_v1;
      verb : access constant thinkthen_complete_verb_v1;
      wording_version : aliased thinkthen_complete_readable_question_3_field_wording_version_presence_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_recognition_field_source_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_physical_source_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_entity_field_file_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_utf8_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_entity_field_first_line_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased Interfaces.Unsigned_64;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_entity_field_last_line_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased Interfaces.Unsigned_64;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_entity_v1 is record
      c_end : aliased Interfaces.Unsigned_64;
      file : aliased thinkthen_complete_entity_field_file_presence_v1;
      first_line : aliased thinkthen_complete_entity_field_first_line_presence_v1;
      kind : aliased thinkthen_complete_utf8_v1;
      last_line : aliased thinkthen_complete_entity_field_last_line_presence_v1;
      length : aliased Interfaces.Unsigned_64;
      start : aliased Interfaces.Unsigned_64;
      strength : aliased double;
      text : aliased thinkthen_complete_utf8_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_recognize_fields_entities_field_entities_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_entity_edge_field_either_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_entity_edge_v1 is record
      either : aliased thinkthen_complete_entity_edge_field_either_presence_v1;
      probability : aliased double;
      relation : aliased thinkthen_complete_utf8_v1;
      source : access constant thinkthen_complete_entity_v1;
      target : access constant thinkthen_complete_entity_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_recognize_fields_entities_field_relations_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_recognize_fields_entities_field_relations_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_recognize_fields_entities_field_relations_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_recognize_fields_entities_v1 is record
      entities : aliased thinkthen_complete_recognize_fields_entities_field_entities_v1;
      relations : aliased thinkthen_complete_recognize_fields_entities_field_relations_presence_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_boundary_mode_v1 is record
      kind : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_recognize_fields_mode_proposals_field_proposals_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_recognize_fields_mode_proposals_v1 is record
      mode : access constant thinkthen_complete_boundary_mode_v1;
      proposals : aliased thinkthen_complete_recognize_fields_mode_proposals_field_proposals_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_recognize_data_v1 (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            fields_entities : access constant thinkthen_complete_recognize_fields_entities_v1;
         when others =>
            fields_mode_proposals : access constant thinkthen_complete_recognize_fields_mode_proposals_v1;
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
      file : aliased thinkthen_complete_recognition_field_file_presence_v1;
      first_line : aliased thinkthen_complete_recognition_field_first_line_presence_v1;
      index : aliased thinkthen_complete_recognition_field_index_presence_v1;
      input : aliased thinkthen_complete_recognition_field_input_presence_v1;
      last_line : aliased thinkthen_complete_recognition_field_last_line_presence_v1;
      meta : access constant thinkthen_complete_meta_v1;
      position : aliased thinkthen_complete_recognition_field_position_presence_v1;
      question : access constant thinkthen_complete_readable_question_3_v1;
      schema : access constant thinkthen_complete_version_v1;
      source : aliased thinkthen_complete_recognition_field_source_presence_v1;
      value : access constant thinkthen_complete_recognize_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_packet_recognize_aggregate_field_value_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_packet_recognize_aggregate_v1 is record
      c_function : aliased thinkthen_complete_utf8_v1;
      kind : aliased thinkthen_complete_utf8_v1;
      value : aliased thinkthen_complete_session_packet_recognize_aggregate_field_value_v1;
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
   type thinkthen_complete_relation_member_answer_id_field_observations_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_relation_member_answer_id_field_question_sources_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_related_entity_v1 is record
      kind : aliased thinkthen_complete_utf8_v1;
      name : aliased thinkthen_complete_utf8_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_relation_member_answer_id_field_target_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_related_entity_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_relation_member_answer_id_field_usage_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_usage_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_relation_member_answer_id_v1 is record
      direction : access constant thinkthen_complete_relation_direction_v1;
      method : access constant thinkthen_complete_relation_method_v1;
      observations : aliased thinkthen_complete_relation_member_answer_id_field_observations_v1;
      question : access constant thinkthen_complete_readable_question_v1;
      question_sources : aliased thinkthen_complete_relation_member_answer_id_field_question_sources_v1;
      reads : aliased thinkthen_complete_utf8_v1;
      relation : aliased thinkthen_complete_utf8_v1;
      request : aliased thinkthen_complete_utf8_v1;
      source : access constant thinkthen_complete_related_entity_v1;
      target : aliased thinkthen_complete_relation_member_answer_id_field_target_presence_v1;
      threshold : access constant thinkthen_complete_threshold_v1;
      usage : aliased thinkthen_complete_relation_member_answer_id_field_usage_presence_v1;
      accepted : aliased Interfaces.Unsigned_32;
      answer : access constant thinkthen_complete_answer_v1;
      answer_id : access constant thinkthen_complete_answer_id_v1;
      probability : aliased double;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_relation_member_failure_id_field_observations_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_relation_member_failure_id_field_question_sources_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_relation_member_failure_id_field_target_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_related_entity_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_relation_member_failure_id_field_usage_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_usage_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_relation_member_failure_id_v1 is record
      direction : access constant thinkthen_complete_relation_direction_v1;
      method : access constant thinkthen_complete_relation_method_v1;
      observations : aliased thinkthen_complete_relation_member_failure_id_field_observations_v1;
      question : access constant thinkthen_complete_readable_question_v1;
      question_sources : aliased thinkthen_complete_relation_member_failure_id_field_question_sources_v1;
      reads : aliased thinkthen_complete_utf8_v1;
      relation : aliased thinkthen_complete_utf8_v1;
      request : aliased thinkthen_complete_utf8_v1;
      source : access constant thinkthen_complete_related_entity_v1;
      target : aliased thinkthen_complete_relation_member_failure_id_field_target_presence_v1;
      threshold : access constant thinkthen_complete_threshold_v1;
      usage : aliased thinkthen_complete_relation_member_failure_id_field_usage_presence_v1;
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
   type thinkthen_complete_answers_field_questions_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answers_v1 is record
      questions : aliased thinkthen_complete_answers_field_questions_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_relation_field_file_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_utf8_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_relation_field_first_line_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased Interfaces.Unsigned_64;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_relation_field_index_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased Interfaces.Unsigned_64;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_relation_field_input_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_json_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_input_source_v1 is record
      index : aliased Interfaces.Unsigned_64;
      source : access constant thinkthen_complete_physical_source_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_relation_field_input_sources_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_relation_field_input_sources_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_relation_field_input_sources_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_relation_field_last_line_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased Interfaces.Unsigned_64;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_relation_field_position_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_position_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_4_field_batch_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_batch_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_4_field_context_schema_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_input_declaration_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_relate_fields_v1 is record
      kind : aliased thinkthen_complete_utf8_v1;
      name : aliased thinkthen_complete_utf8_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_4_field_fields_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_relate_fields_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_4_field_item_schema_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_input_declaration_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_4_field_label_details_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_4_field_label_details_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_readable_question_4_field_label_details_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_4_field_model_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_utf8_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_4_field_name_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_question_name_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_4_field_on_v1 is record
      data : access constant thinkthen_complete_utf8_v1;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_4_field_on_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_readable_question_4_field_on_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_4_field_profile_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_utf8_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_4_field_relations_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_4_field_wording_version_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_wording_version_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_readable_question_4_v1 is record
      batch : aliased thinkthen_complete_readable_question_4_field_batch_presence_v1;
      context_schema : aliased thinkthen_complete_readable_question_4_field_context_schema_presence_v1;
      fields : aliased thinkthen_complete_readable_question_4_field_fields_presence_v1;
      item_schema : aliased thinkthen_complete_readable_question_4_field_item_schema_presence_v1;
      label_details : aliased thinkthen_complete_readable_question_4_field_label_details_presence_v1;
      model : aliased thinkthen_complete_readable_question_4_field_model_presence_v1;
      name : aliased thinkthen_complete_readable_question_4_field_name_presence_v1;
      on : aliased thinkthen_complete_readable_question_4_field_on_presence_v1;
      profile : aliased thinkthen_complete_readable_question_4_field_profile_presence_v1;
      relations : aliased thinkthen_complete_readable_question_4_field_relations_v1;
      threshold : access constant thinkthen_complete_threshold_v1;
      verb : aliased thinkthen_complete_utf8_v1;
      wording_version : aliased thinkthen_complete_readable_question_4_field_wording_version_presence_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_related_entity_edge_field_either_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_related_entity_edge_properties_source_fields_kind_name_v1 is record
      kind : aliased thinkthen_complete_utf8_v1;
      name : aliased thinkthen_complete_utf8_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_related_entity_edge_properties_source_fields_file_kind_name_ordinal_record_field_file_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_utf8_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_related_entity_edge_properties_source_fields_file_kind_name_ordinal_record_field_first_line_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased Interfaces.Unsigned_64;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_related_entity_edge_properties_source_fields_file_kind_name_ordinal_record_field_last_line_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased Interfaces.Unsigned_64;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_related_entity_edge_properties_source_fields_file_kind_name_ordinal_record_field_record_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_json_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_related_entity_edge_properties_source_fields_file_kind_name_ordinal_record_v1 is record
      file : aliased thinkthen_complete_related_entity_edge_properties_source_fields_file_kind_name_ordinal_record_field_file_presence_v1;
      first_line : aliased thinkthen_complete_related_entity_edge_properties_source_fields_file_kind_name_ordinal_record_field_first_line_presence_v1;
      kind : aliased thinkthen_complete_utf8_v1;
      last_line : aliased thinkthen_complete_related_entity_edge_properties_source_fields_file_kind_name_ordinal_record_field_last_line_presence_v1;
      name : aliased thinkthen_complete_utf8_v1;
      ordinal : aliased Interfaces.Unsigned_64;
      c_record : aliased thinkthen_complete_related_entity_edge_properties_source_fields_file_kind_name_ordinal_record_field_record_presence_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_related_entity_edge_properties_source_data_v1 (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            fields_kind_name : access constant thinkthen_complete_related_entity_edge_properties_source_fields_kind_name_v1;
         when others =>
            fields_file_kind_name_ordinal_record : access constant thinkthen_complete_related_entity_edge_properties_source_fields_file_kind_name_ordinal_record_v1;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_related_entity_edge_properties_source_v1 is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_related_entity_edge_properties_source_data_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_related_entity_edge_v1 is record
      either : aliased thinkthen_complete_related_entity_edge_field_either_presence_v1;
      probability : aliased double;
      relation : aliased thinkthen_complete_utf8_v1;
      source : access constant thinkthen_complete_related_entity_edge_properties_source_v1;
      target : access constant thinkthen_complete_related_entity_edge_properties_source_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_relation_field_value_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_relation_v1 is record
      answer : access constant thinkthen_complete_answers_v1;
      answer_id : access constant thinkthen_complete_answer_id_v1;
      file : aliased thinkthen_complete_relation_field_file_presence_v1;
      first_line : aliased thinkthen_complete_relation_field_first_line_presence_v1;
      index : aliased thinkthen_complete_relation_field_index_presence_v1;
      input : aliased thinkthen_complete_relation_field_input_presence_v1;
      input_sources : aliased thinkthen_complete_relation_field_input_sources_presence_v1;
      last_line : aliased thinkthen_complete_relation_field_last_line_presence_v1;
      meta : access constant thinkthen_complete_meta_v1;
      position : aliased thinkthen_complete_relation_field_position_presence_v1;
      question : access constant thinkthen_complete_readable_question_4_v1;
      schema : access constant thinkthen_complete_version_v1;
      value : aliased thinkthen_complete_relation_field_value_v1;
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
   type thinkthen_complete_session_question_detail_field_answer_id_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_answer_id_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_question_detail_field_confidence_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased double;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_question_detail_field_failure_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_failure_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_question_detail_field_failure_id_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_failure_id_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_question_detail_field_input_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_json_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_question_detail_field_input_source_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_physical_source_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_question_detail_field_input_sources_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_question_detail_field_inputs_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_question_detail_field_observations_v1 is record
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
   type thinkthen_complete_session_probabilities_named_field_value_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_probabilities_named_v1 is record
      kind : aliased thinkthen_complete_utf8_v1;
      value : aliased thinkthen_complete_session_probabilities_named_field_value_v1;
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
   type thinkthen_complete_session_question_detail_field_probabilities_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_session_probabilities_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_question_detail_field_question_sources_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_question_detail_field_raw_pick_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_utf8_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_question_detail_field_reported_usage_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_usage_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_question_detail_field_requests_v1 is record
      data : access constant thinkthen_complete_utf8_v1;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_question_detail_field_threshold_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_threshold_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_token_usage_v1 is record
      input_tokens : aliased Interfaces.Unsigned_64;
      output_tokens : aliased Interfaces.Unsigned_64;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_question_detail_field_usage_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_token_usage_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_question_detail_field_value_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_value_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_question_detail_v1 is record
      answer_id : aliased thinkthen_complete_session_question_detail_field_answer_id_presence_v1;
      cached : aliased Interfaces.Unsigned_32;
      confidence : aliased thinkthen_complete_session_question_detail_field_confidence_presence_v1;
      failed_questions : aliased Interfaces.Unsigned_64;
      failure : aliased thinkthen_complete_session_question_detail_field_failure_presence_v1;
      failure_id : aliased thinkthen_complete_session_question_detail_field_failure_id_presence_v1;
      input : aliased thinkthen_complete_session_question_detail_field_input_presence_v1;
      input_source : aliased thinkthen_complete_session_question_detail_field_input_source_presence_v1;
      input_sources : aliased thinkthen_complete_session_question_detail_field_input_sources_v1;
      inputs : aliased thinkthen_complete_session_question_detail_field_inputs_v1;
      model : aliased thinkthen_complete_utf8_v1;
      observations : aliased thinkthen_complete_session_question_detail_field_observations_v1;
      probabilities : aliased thinkthen_complete_session_question_detail_field_probabilities_presence_v1;
      question : access constant thinkthen_complete_readable_question_v1;
      question_sha256 : aliased thinkthen_complete_utf8_v1;
      question_sources : aliased thinkthen_complete_session_question_detail_field_question_sources_v1;
      raw_pick : aliased thinkthen_complete_session_question_detail_field_raw_pick_presence_v1;
      reported_usage : aliased thinkthen_complete_session_question_detail_field_reported_usage_presence_v1;
      requests : aliased thinkthen_complete_session_question_detail_field_requests_v1;
      requests_sent : aliased Interfaces.Unsigned_64;
      threshold : aliased thinkthen_complete_session_question_detail_field_threshold_presence_v1;
      url : aliased thinkthen_complete_utf8_v1;
      usage : aliased thinkthen_complete_session_question_detail_field_usage_presence_v1;
      value : aliased thinkthen_complete_session_question_detail_field_value_presence_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_observation_question_field_member_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_utf8_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_observation_question_field_stage_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_utf8_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_observation_question_v1 is record
      detail : access constant thinkthen_complete_session_question_detail_v1;
      index : aliased Interfaces.Unsigned_64;
      kind : aliased thinkthen_complete_utf8_v1;
      member : aliased thinkthen_complete_session_observation_question_field_member_presence_v1;
      position : aliased Interfaces.Unsigned_64;
      stage : aliased thinkthen_complete_session_observation_question_field_stage_presence_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_judgment_decision_field_value_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_judgment_decision_v1 is record
      kind : aliased thinkthen_complete_utf8_v1;
      value : aliased thinkthen_complete_session_judgment_decision_field_value_presence_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_judgment_choice_field_value_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_utf8_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_judgment_choice_v1 is record
      kind : aliased thinkthen_complete_utf8_v1;
      value : aliased thinkthen_complete_session_judgment_choice_field_value_presence_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_judgment_score_v1 is record
      kind : aliased thinkthen_complete_utf8_v1;
      value : aliased double;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_judgment_tags_field_value_v1 is record
      data : access constant thinkthen_complete_utf8_v1;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_judgment_tags_v1 is record
      kind : aliased thinkthen_complete_utf8_v1;
      value : aliased thinkthen_complete_session_judgment_tags_field_value_v1;
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
   type thinkthen_complete_annotation_value_decision_field_value_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_annotation_value_decision_v1 is record
      kind : aliased thinkthen_complete_utf8_v1;
      value : aliased thinkthen_complete_annotation_value_decision_field_value_presence_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_annotation_value_choice_field_value_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_utf8_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_annotation_value_choice_v1 is record
      kind : aliased thinkthen_complete_utf8_v1;
      value : aliased thinkthen_complete_annotation_value_choice_field_value_presence_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_annotation_value_score_v1 is record
      kind : aliased thinkthen_complete_utf8_v1;
      value : aliased double;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_annotation_value_tags_field_value_v1 is record
      data : access constant thinkthen_complete_utf8_v1;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_annotation_value_tags_v1 is record
      kind : aliased thinkthen_complete_utf8_v1;
      value : aliased thinkthen_complete_annotation_value_tags_field_value_v1;
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
   type thinkthen_complete_session_observed_row_annotated_field_value_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_observed_row_annotated_v1 is record
      kind : aliased thinkthen_complete_utf8_v1;
      value : aliased thinkthen_complete_session_observed_row_annotated_field_value_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_recognition_field_entities_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_recognition_field_proposals_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_recognition_field_proposals_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_session_recognition_field_proposals_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_recognition_edge_document_v1 is record
      either : aliased Interfaces.Unsigned_32;
      probability : aliased double;
      relation : aliased thinkthen_complete_utf8_v1;
      source : access constant thinkthen_complete_entity_v1;
      target : access constant thinkthen_complete_entity_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_recognition_field_relations_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_recognition_field_relations_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_session_recognition_field_relations_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_recognition_v1 is record
      entities : aliased thinkthen_complete_session_recognition_field_entities_v1;
      mode : access constant thinkthen_complete_recognition_mode_v1;
      proposals : aliased thinkthen_complete_session_recognition_field_proposals_presence_v1;
      relations : aliased thinkthen_complete_session_recognition_field_relations_presence_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_observed_row_recognized_v1 is record
      kind : aliased thinkthen_complete_utf8_v1;
      value : access constant thinkthen_complete_session_recognition_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_observed_row_find_field_value_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased Interfaces.Unsigned_64;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_observed_row_find_v1 is record
      kind : aliased thinkthen_complete_utf8_v1;
      value : aliased thinkthen_complete_session_observed_row_find_field_value_presence_v1;
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
   type thinkthen_complete_session_observed_row_relations_field_value_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_observed_row_relations_v1 is record
      kind : aliased thinkthen_complete_utf8_v1;
      value : aliased thinkthen_complete_session_observed_row_relations_field_value_v1;
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
   type thinkthen_complete_facts_field_attempts_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_facts_field_attempts_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_facts_field_attempts_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_call_id_v1 is record
      value : aliased thinkthen_complete_utf8_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_facts_field_estimated_cost_usd_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_utf8_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_facts_field_held_model_mismatch_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_facts_field_input_tokens_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased Interfaces.Unsigned_64;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_facts_field_largest_request_estimated_input_tokens_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased Interfaces.Unsigned_64;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_facts_field_model_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_utf8_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_facts_field_output_tokens_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased Interfaces.Unsigned_64;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_persistence_observation_field_advice_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_utf8_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_usage_persistence_v1 is record
      kind : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_persistence_observation_v1 is record
      advice : aliased thinkthen_complete_persistence_observation_field_advice_presence_v1;
      observed_at : aliased thinkthen_complete_utf8_v1;
      state : access constant thinkthen_complete_usage_persistence_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_facts_field_usage_persistence_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_persistence_observation_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_facts_v1 is record
      attempts : aliased thinkthen_complete_facts_field_attempts_presence_v1;
      cache_answers : aliased Interfaces.Unsigned_64;
      call_id : access constant thinkthen_complete_call_id_v1;
      estimated_cost_usd : aliased thinkthen_complete_facts_field_estimated_cost_usd_presence_v1;
      held_model_mismatch : aliased thinkthen_complete_facts_field_held_model_mismatch_presence_v1;
      input_tokens : aliased thinkthen_complete_facts_field_input_tokens_presence_v1;
      largest_request_bytes : aliased Interfaces.Unsigned_64;
      largest_request_estimated_input_tokens : aliased thinkthen_complete_facts_field_largest_request_estimated_input_tokens_presence_v1;
      model : aliased thinkthen_complete_facts_field_model_presence_v1;
      output_tokens : aliased thinkthen_complete_facts_field_output_tokens_presence_v1;
      records : aliased Interfaces.Unsigned_64;
      requests_sent : aliased Interfaces.Unsigned_64;
      seconds : aliased double;
      token_estimate_method : aliased thinkthen_complete_utf8_v1;
      usage_persistence : aliased thinkthen_complete_facts_field_usage_persistence_presence_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_packet_terminal_field_facts_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_facts_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_estimated_input_denial_initial_request_v1 is record
      kind : aliased thinkthen_complete_utf8_v1;
      limit : aliased Interfaces.Unsigned_64;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_estimated_input_denial_additional_request_v1 is record
      kind : aliased thinkthen_complete_utf8_v1;
      limit : aliased Interfaces.Unsigned_64;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_estimated_input_denial_retry_v1 is record
      kind : aliased thinkthen_complete_utf8_v1;
      last_status : aliased Interfaces.Unsigned_64;
      limit : aliased Interfaces.Unsigned_64;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_estimated_input_denial_data_v1 (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            initial_request : access constant thinkthen_complete_estimated_input_denial_initial_request_v1;
         when 1 =>
            additional_request : access constant thinkthen_complete_estimated_input_denial_additional_request_v1;
         when others =>
            retry : access constant thinkthen_complete_estimated_input_denial_retry_v1;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_estimated_input_denial_v1 is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_estimated_input_denial_data_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_error_field_estimated_input_denial_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_estimated_input_denial_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_failure_kind_v1 is record
      kind : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_send_budget_denial_before_first_send_v1 is record
      kind : aliased thinkthen_complete_utf8_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_send_budget_denial_before_additional_send_v1 is record
      kind : aliased thinkthen_complete_utf8_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_send_budget_denial_before_retry_v1 is record
      kind : aliased thinkthen_complete_utf8_v1;
      last_status : aliased Interfaces.Unsigned_64;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_send_budget_denial_data_v1 (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            before_first_send : access constant thinkthen_complete_send_budget_denial_before_first_send_v1;
         when 1 =>
            before_additional_send : access constant thinkthen_complete_send_budget_denial_before_additional_send_v1;
         when others =>
            before_retry : access constant thinkthen_complete_send_budget_denial_before_retry_v1;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_send_budget_denial_v1 is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_send_budget_denial_data_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_error_field_send_budget_denial_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_send_budget_denial_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_stopped_field_at_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased Interfaces.Unsigned_64;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_stop_cause_v1 is record
      kind : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_stopped_field_status_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased Interfaces.Unsigned_64;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_stopped_v1 is record
      c_at : aliased thinkthen_complete_stopped_field_at_presence_v1;
      cause : access constant thinkthen_complete_stop_cause_v1;
      retryable : aliased Interfaces.Unsigned_32;
      status : aliased thinkthen_complete_stopped_field_status_presence_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_error_v1 is record
      estimated_input_denial : aliased thinkthen_complete_error_field_estimated_input_denial_presence_v1;
      kind : access constant thinkthen_complete_failure_kind_v1;
      message : aliased thinkthen_complete_utf8_v1;
      retryable : aliased Interfaces.Unsigned_32;
      send_budget_denial : aliased thinkthen_complete_error_field_send_budget_denial_presence_v1;
      stopped : access constant thinkthen_complete_stopped_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_call_error_field_facts_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_facts_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_call_error_v1 is record
      error : access constant thinkthen_complete_error_v1;
      facts : aliased thinkthen_complete_call_error_field_facts_presence_v1;
      extensions : aliased thinkthen_complete_extensions_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_packet_terminal_field_failure_presence_v1 is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_call_error_v1;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_packet_terminal_v1 is record
      facts : aliased thinkthen_complete_session_packet_terminal_field_facts_presence_v1;
      failure : aliased thinkthen_complete_session_packet_terminal_field_failure_presence_v1;
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
   type thinkthen_source_spec_v1 is record
      paths : aliased thinkthen_strings_v1;
      unit : aliased Interfaces.Unsigned_32;
      window : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_images_v1 is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_record_v1 is record
      original : aliased thinkthen_optional_content_v1;
      context : aliased thinkthen_optional_content_v1;
      options : aliased thinkthen_choices_v1;
      images : aliased thinkthen_images_v1;
   end record
   with Convention => C_Pass_By_Copy;
   function thinkthen_annotate_batch_start
     (e : access constant thinkthen_engine;
      q : access constant thinkthen_question;
      s : access constant thinkthen_source;
      c : access constant thinkthen_controls_v1;
      c_out : System.Address) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_annotate_batch_start";
   function thinkthen_annotate_complete
     (engine : access constant thinkthen_engine;
      question : access constant thinkthen_question;
      source : access constant thinkthen_source;
      controls : access constant thinkthen_controls_v1;
      c_out : System.Address) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_annotate_complete";
   function thinkthen_batch_facts (owner : access constant thinkthen_batch; c_out : System.Address) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_batch_facts";
   procedure thinkthen_batch_free (owner : access thinkthen_batch)
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_batch_free";
   function thinkthen_batch_next (owner : access thinkthen_batch; c_out : System.Address) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_batch_next";
   function thinkthen_call (engine : access constant thinkthen_engine; request_json : Interfaces.C.Strings.chars_ptr) return Interfaces.C.Strings.chars_ptr
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_call";
   function thinkthen_call_opts
     (engine : access constant thinkthen_engine;
      request_json : Interfaces.C.Strings.chars_ptr;
      deadline_ms : Interfaces.Integer_64;
      cancel : access thinkthen_cancel_token) return Interfaces.C.Strings.chars_ptr
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_call_opts";
   procedure thinkthen_cancel (token : access thinkthen_cancel_token)
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_cancel";
   procedure thinkthen_cancel_token_free (token : access thinkthen_cancel_token)
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_cancel_token_free";
   function thinkthen_cancel_token_new return access thinkthen_cancel_token
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_cancel_token_new";
   function thinkthen_choose_batch_start
     (e : access constant thinkthen_engine;
      q : access constant thinkthen_question;
      s : access constant thinkthen_source;
      c : access constant thinkthen_controls_v1;
      c_out : System.Address) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_choose_batch_start";
   function thinkthen_choose_complete
     (engine : access constant thinkthen_engine;
      question : access constant thinkthen_question;
      source : access constant thinkthen_source;
      controls : access constant thinkthen_controls_v1;
      c_out : System.Address) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_choose_complete";
   function thinkthen_decide
     (engine : access constant thinkthen_engine;
      question_json : Interfaces.C.Strings.chars_ptr;
      text : Interfaces.C.Strings.chars_ptr;
      text_len : Interfaces.C.size_t;
      c_out : access thinkthen_answer) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_decide";
   function thinkthen_decide_batch_start
     (e : access constant thinkthen_engine;
      q : access constant thinkthen_question;
      s : access constant thinkthen_source;
      c : access constant thinkthen_controls_v1;
      c_out : System.Address) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_decide_batch_start";
   function thinkthen_decide_complete
     (engine : access constant thinkthen_engine;
      question : access constant thinkthen_question;
      source : access constant thinkthen_source;
      controls : access constant thinkthen_controls_v1;
      c_out : System.Address) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_decide_complete";
   function thinkthen_decide_many
     (engine : access constant thinkthen_engine;
      question_json : Interfaces.C.Strings.chars_ptr;
      texts : System.Address;
      lengths : access Interfaces.C.size_t;
      count : Interfaces.C.size_t;
      c_out : access thinkthen_answer) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_decide_many";
   function thinkthen_decide_many_opts
     (engine : access constant thinkthen_engine;
      question_json : Interfaces.C.Strings.chars_ptr;
      texts : System.Address;
      lengths : access Interfaces.C.size_t;
      count : Interfaces.C.size_t;
      deadline_ms : Interfaces.Integer_64;
      cancel : access thinkthen_cancel_token;
      c_out : access thinkthen_answer) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_decide_many_opts";
   function thinkthen_decide_many_with_facts
     (engine : access constant thinkthen_engine;
      question_json : Interfaces.C.Strings.chars_ptr;
      texts : System.Address;
      lengths : access Interfaces.C.size_t;
      count : Interfaces.C.size_t;
      c_out : access thinkthen_answer;
      facts_json : System.Address;
      facts_len : access Interfaces.C.size_t) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_decide_many_with_facts";
   function thinkthen_decide_many_with_facts_opts
     (engine : access constant thinkthen_engine;
      question_json : Interfaces.C.Strings.chars_ptr;
      texts : System.Address;
      lengths : access Interfaces.C.size_t;
      count : Interfaces.C.size_t;
      deadline_ms : Interfaces.Integer_64;
      cancel : access thinkthen_cancel_token;
      c_out : access thinkthen_answer;
      facts_json : System.Address;
      facts_len : access Interfaces.C.size_t) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_decide_many_with_facts_opts";
   function thinkthen_decide_opts
     (engine : access constant thinkthen_engine;
      question_json : Interfaces.C.Strings.chars_ptr;
      text : Interfaces.C.Strings.chars_ptr;
      text_len : Interfaces.C.size_t;
      deadline_ms : Interfaces.Integer_64;
      cancel : access thinkthen_cancel_token;
      c_out : access thinkthen_answer) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_decide_opts";
   function thinkthen_decide_with_facts
     (engine : access constant thinkthen_engine;
      question_json : Interfaces.C.Strings.chars_ptr;
      evidence : Interfaces.C.Strings.chars_ptr;
      evidence_len : Interfaces.C.size_t;
      c_out : access thinkthen_answer;
      facts_json : System.Address;
      facts_len : access Interfaces.C.size_t) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_decide_with_facts";
   function thinkthen_decide_with_facts_opts
     (engine : access constant thinkthen_engine;
      question_json : Interfaces.C.Strings.chars_ptr;
      evidence : Interfaces.C.Strings.chars_ptr;
      evidence_len : Interfaces.C.size_t;
      deadline_ms : Interfaces.Integer_64;
      cancel : access thinkthen_cancel_token;
      c_out : access thinkthen_answer;
      facts_json : System.Address;
      facts_len : access Interfaces.C.size_t) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_decide_with_facts_opts";
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
   function thinkthen_error_code (engine : access constant thinkthen_engine) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_error_code";
   function thinkthen_error_complete (engine : access constant thinkthen_engine; c_out : System.Address) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_error_complete";
   function thinkthen_error_facts_json (engine : access constant thinkthen_engine) return Interfaces.C.Strings.chars_ptr
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_error_facts_json";
   function thinkthen_error_message (engine : access constant thinkthen_engine) return Interfaces.C.Strings.chars_ptr
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_error_message";
   function thinkthen_error_retryable (engine : access constant thinkthen_engine) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_error_retryable";
   function thinkthen_filter_batch_start
     (e : access constant thinkthen_engine;
      q : access constant thinkthen_question;
      s : access constant thinkthen_source;
      c : access constant thinkthen_controls_v1;
      c_out : System.Address) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_filter_batch_start";
   function thinkthen_filter_complete
     (engine : access constant thinkthen_engine;
      question : access constant thinkthen_question;
      source : access constant thinkthen_source;
      controls : access constant thinkthen_controls_v1;
      c_out : System.Address) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_filter_complete";
   function thinkthen_find_complete
     (engine : access constant thinkthen_engine;
      question : access constant thinkthen_question;
      source : access constant thinkthen_source;
      controls : access constant thinkthen_controls_v1;
      c_out : System.Address) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_find_complete";
   procedure thinkthen_free_string (text : Interfaces.C.Strings.chars_ptr)
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_free_string";
   function thinkthen_image_clone
     (engine : access constant thinkthen_engine;
      bytes : access Interfaces.Unsigned_8;
      len : Interfaces.C.size_t;
      media : Interfaces.Unsigned_32;
      filename : thinkthen_optional_string_v1;
      c_out : System.Address) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_image_clone";
   procedure thinkthen_image_free (owner : access thinkthen_image)
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_image_free";
   function thinkthen_image_view (owner : access constant thinkthen_image; c_out : access thinkthen_image_view_v1) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_image_view";
   function thinkthen_plan_json
     (engine : access constant thinkthen_engine;
      plan_json : Interfaces.C.Strings.chars_ptr;
      c_out : System.Address;
      out_len : access Interfaces.C.size_t) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_plan_json";
   function thinkthen_question_author (owner : access constant thinkthen_question; c_out : access thinkthen_question_author_v1) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_question_author";
   function thinkthen_question_file
     (engine : access constant thinkthen_engine;
      path : Interfaces.C.Strings.chars_ptr;
      c_out : System.Address;
      out_len : access Interfaces.C.size_t) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_question_file";
   procedure thinkthen_question_free (owner : access thinkthen_question)
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_question_free";
   function thinkthen_question_load
     (engine : access constant thinkthen_engine;
      path : thinkthen_string_v1;
      c_out : System.Address) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_question_load";
   function thinkthen_question_load_named
     (engine : access constant thinkthen_engine;
      role : Interfaces.Unsigned_32;
      name : thinkthen_string_v1;
      c_out : System.Address) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_question_load_named";
   function thinkthen_question_load_reference
     (engine : access constant thinkthen_engine;
      role : Interfaces.Unsigned_32;
      reference : thinkthen_string_v1;
      c_out : System.Address) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_question_load_reference";
   function thinkthen_question_new
     (engine : access constant thinkthen_engine;
      spec : access constant thinkthen_question_spec_v1;
      c_out : System.Address) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_question_new";
   function thinkthen_question_new_authored
     (engine : access constant thinkthen_engine;
      spec : access constant thinkthen_question_spec_v1;
      author : access constant thinkthen_question_author_v1;
      c_out : System.Address) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_question_new_authored";
   function thinkthen_question_new_recognition_v1
     (engine : access constant thinkthen_engine;
      spec : access constant thinkthen_question_spec_v1;
      metadata : access constant thinkthen_question_author_v1;
      c_task : access constant thinkthen_recognition_task_v1;
      c_out : System.Address) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_question_new_recognition_v1";
   function thinkthen_question_parse
     (engine : access constant thinkthen_engine;
      role : Interfaces.Unsigned_32;
      json : thinkthen_string_v1;
      c_out : System.Address) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_question_parse";
   function thinkthen_question_recognition_task_v1 (owner : access constant thinkthen_question; c_out : access thinkthen_recognition_task_v1) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_question_recognition_task_v1";
   function thinkthen_rank_complete
     (engine : access constant thinkthen_engine;
      question : access constant thinkthen_question;
      source : access constant thinkthen_source;
      controls : access constant thinkthen_controls_v1;
      c_out : System.Address) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_rank_complete";
   function thinkthen_recognize
     (engine : access constant thinkthen_engine;
      spec_json : Interfaces.C.Strings.chars_ptr;
      text : Interfaces.C.Strings.chars_ptr;
      text_len : Interfaces.C.size_t;
      c_out : System.Address;
      out_len : access Interfaces.C.size_t) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_recognize";
   function thinkthen_recognize_complete
     (engine : access constant thinkthen_engine;
      question : access constant thinkthen_question;
      source : access constant thinkthen_source;
      controls : access constant thinkthen_controls_v1;
      c_out : System.Address) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_recognize_complete";
   function thinkthen_recognize_opts
     (engine : access constant thinkthen_engine;
      spec_json : Interfaces.C.Strings.chars_ptr;
      text : Interfaces.C.Strings.chars_ptr;
      text_len : Interfaces.C.size_t;
      deadline_ms : Interfaces.Integer_64;
      cancel : access thinkthen_cancel_token;
      c_out : System.Address;
      out_len : access Interfaces.C.size_t) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_recognize_opts";
   function thinkthen_recognize_with_facts
     (engine : access constant thinkthen_engine;
      spec_json : Interfaces.C.Strings.chars_ptr;
      evidence : Interfaces.C.Strings.chars_ptr;
      evidence_len : Interfaces.C.size_t;
      c_out : System.Address;
      out_len : access Interfaces.C.size_t;
      facts_json : System.Address;
      facts_len : access Interfaces.C.size_t) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_recognize_with_facts";
   function thinkthen_recognize_with_facts_opts
     (engine : access constant thinkthen_engine;
      spec_json : Interfaces.C.Strings.chars_ptr;
      evidence : Interfaces.C.Strings.chars_ptr;
      evidence_len : Interfaces.C.size_t;
      deadline_ms : Interfaces.Integer_64;
      cancel : access thinkthen_cancel_token;
      c_out : System.Address;
      out_len : access Interfaces.C.size_t;
      facts_json : System.Address;
      facts_len : access Interfaces.C.size_t) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_recognize_with_facts_opts";
   function thinkthen_relate
     (engine : access constant thinkthen_engine;
      spec_json : Interfaces.C.Strings.chars_ptr;
      texts : System.Address;
      lengths : access Interfaces.C.size_t;
      count : Interfaces.C.size_t;
      c_out : System.Address;
      out_len : access Interfaces.C.size_t) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_relate";
   function thinkthen_relate_complete
     (engine : access constant thinkthen_engine;
      question : access constant thinkthen_question;
      source : access constant thinkthen_source;
      controls : access constant thinkthen_controls_v1;
      c_out : System.Address) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_relate_complete";
   function thinkthen_relate_opts
     (engine : access constant thinkthen_engine;
      spec_json : Interfaces.C.Strings.chars_ptr;
      texts : System.Address;
      lengths : access Interfaces.C.size_t;
      count : Interfaces.C.size_t;
      deadline_ms : Interfaces.Integer_64;
      cancel : access thinkthen_cancel_token;
      c_out : System.Address;
      out_len : access Interfaces.C.size_t) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_relate_opts";
   function thinkthen_relate_with_facts
     (engine : access constant thinkthen_engine;
      spec_json : Interfaces.C.Strings.chars_ptr;
      texts : System.Address;
      lengths : access Interfaces.C.size_t;
      count : Interfaces.C.size_t;
      c_out : System.Address;
      out_len : access Interfaces.C.size_t;
      facts_json : System.Address;
      facts_len : access Interfaces.C.size_t) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_relate_with_facts";
   function thinkthen_relate_with_facts_opts
     (engine : access constant thinkthen_engine;
      spec_json : Interfaces.C.Strings.chars_ptr;
      texts : System.Address;
      lengths : access Interfaces.C.size_t;
      count : Interfaces.C.size_t;
      deadline_ms : Interfaces.Integer_64;
      cancel : access thinkthen_cancel_token;
      c_out : System.Address;
      out_len : access Interfaces.C.size_t;
      facts_json : System.Address;
      facts_len : access Interfaces.C.size_t) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_relate_with_facts_opts";
   function thinkthen_request_plan_json
     (engine : access constant thinkthen_engine;
      request_json : Interfaces.C.Strings.chars_ptr;
      request_len : Interfaces.C.size_t;
      c_out : System.Address;
      out_len : access Interfaces.C.size_t) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_request_plan_json";
   function thinkthen_result_annotate
     (owner : access constant thinkthen_result;
      c_at : Interfaces.C.size_t;
      c_out : access thinkthen_annotate_view_v1) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_result_annotate";
   function thinkthen_result_choose
     (owner : access constant thinkthen_result;
      c_at : Interfaces.C.size_t;
      c_out : access thinkthen_choose_view_v1) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_result_choose";
   function thinkthen_result_decide
     (owner : access constant thinkthen_result;
      c_at : Interfaces.C.size_t;
      c_out : access thinkthen_decide_view_v1) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_result_decide";
   function thinkthen_result_details
     (owner : access constant thinkthen_result;
      c_at : Interfaces.C.size_t;
      c_out : access thinkthen_details_v1) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_result_details";
   function thinkthen_result_filter
     (owner : access constant thinkthen_result;
      c_at : Interfaces.C.size_t;
      c_out : access thinkthen_filter_view_v1) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_result_filter";
   function thinkthen_result_find
     (owner : access constant thinkthen_result;
      c_at : Interfaces.C.size_t;
      c_out : access thinkthen_find_view_v1) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_result_find";
   procedure thinkthen_result_free (owner : access thinkthen_result)
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_result_free";
   function thinkthen_result_member_author
     (owner : access constant thinkthen_result;
      c_at : Interfaces.C.size_t;
      member : Interfaces.C.size_t;
      c_out : access thinkthen_question_author_v1) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_result_member_author";
   function thinkthen_result_observation
     (owner : access constant thinkthen_result;
      c_at : Interfaces.C.size_t;
      c_out : access thinkthen_observation_v1) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_result_observation";
   function thinkthen_result_observation_author
     (owner : access constant thinkthen_result;
      c_at : Interfaces.C.size_t;
      c_out : access thinkthen_question_author_v1) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_result_observation_author";
   function thinkthen_result_observation_details
     (owner : access constant thinkthen_result;
      c_at : Interfaces.C.size_t;
      c_out : access thinkthen_details_v1) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_result_observation_details";
   function thinkthen_result_question_author
     (owner : access constant thinkthen_result;
      c_at : Interfaces.C.size_t;
      c_out : access thinkthen_question_author_v1) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_result_question_author";
   function thinkthen_result_rank
     (owner : access constant thinkthen_result;
      c_at : Interfaces.C.size_t;
      c_out : access thinkthen_rank_view_v1) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_result_rank";
   function thinkthen_result_rank_member
     (owner : access constant thinkthen_result;
      row : Interfaces.C.size_t;
      member : Interfaces.C.size_t;
      c_out : access thinkthen_rank_view_v1) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_result_rank_member";
   function thinkthen_result_rank_member_count
     (owner : access constant thinkthen_result;
      row : Interfaces.C.size_t;
      c_out : access Interfaces.C.size_t) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_result_rank_member_count";
   function thinkthen_result_rank_member_details
     (owner : access constant thinkthen_result;
      row : Interfaces.C.size_t;
      member : Interfaces.C.size_t;
      c_out : access thinkthen_details_v1) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_result_rank_member_details";
   function thinkthen_result_recognition_task_v1
     (owner : access constant thinkthen_result;
      c_at : Interfaces.C.size_t;
      c_out : access thinkthen_recognition_task_v1) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_result_recognition_task_v1";
   function thinkthen_result_recognize
     (owner : access constant thinkthen_result;
      c_at : Interfaces.C.size_t;
      c_out : access thinkthen_recognize_view_v1) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_result_recognize";
   function thinkthen_result_relate
     (owner : access constant thinkthen_result;
      c_at : Interfaces.C.size_t;
      c_out : access thinkthen_relate_view_v1) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_result_relate";
   function thinkthen_result_row
     (owner : access constant thinkthen_result;
      c_at : Interfaces.C.size_t;
      c_out : access thinkthen_row_observation_v1) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_result_row";
   function thinkthen_result_score
     (owner : access constant thinkthen_result;
      c_at : Interfaces.C.size_t;
      c_out : access thinkthen_score_view_v1) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_result_score";
   function thinkthen_result_source_recognition
     (owner : access constant thinkthen_result;
      row : Interfaces.C.size_t;
      c_out : access thinkthen_source_recognition_v1) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_result_source_recognition";
   function thinkthen_result_source_relations
     (owner : access constant thinkthen_result;
      row : Interfaces.C.size_t;
      c_out : access thinkthen_source_relations_v1) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_result_source_relations";
   function thinkthen_result_summary (owner : access constant thinkthen_result; c_out : access thinkthen_summary_v1) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_result_summary";
   function thinkthen_result_tag
     (owner : access constant thinkthen_result;
      c_at : Interfaces.C.size_t;
      c_out : access thinkthen_tag_view_v1) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_result_tag";
   function thinkthen_score_batch_start
     (e : access constant thinkthen_engine;
      q : access constant thinkthen_question;
      s : access constant thinkthen_source;
      c : access constant thinkthen_controls_v1;
      c_out : System.Address) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_score_batch_start";
   function thinkthen_score_complete
     (engine : access constant thinkthen_engine;
      question : access constant thinkthen_question;
      source : access constant thinkthen_source;
      controls : access constant thinkthen_controls_v1;
      c_out : System.Address) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_score_complete";
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
   procedure thinkthen_session_free (session : access thinkthen_session)
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_session_free";
   function thinkthen_session_new
     (engine : access constant thinkthen_engine;
      request_json : Interfaces.C.Strings.chars_ptr;
      request_len : Interfaces.C.size_t;
      c_out : System.Address) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_session_new";
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
   procedure thinkthen_session_result_free (result : access thinkthen_session_result)
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_session_result_free";
   function thinkthen_session_result_json
     (result : access constant thinkthen_session_result;
      c_out : System.Address;
      out_len : access Interfaces.C.size_t) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_session_result_json";
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
   function thinkthen_session_try_read
     (session : access thinkthen_session;
      status : access Interfaces.Unsigned_32;
      c_out : System.Address) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_session_try_read";
   function thinkthen_source_files
     (engine : access constant thinkthen_engine;
      spec : access constant thinkthen_source_spec_v1;
      c_out : System.Address) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_source_files";
   procedure thinkthen_source_free (owner : access thinkthen_source)
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_source_free";
   function thinkthen_source_image_files
     (engine : access constant thinkthen_engine;
      spec : access constant thinkthen_source_spec_v1;
      c_out : System.Address) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_source_image_files";
   function thinkthen_source_records
     (engine : access constant thinkthen_engine;
      records : access constant thinkthen_record_v1;
      count : Interfaces.C.size_t;
      c_out : System.Address) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_source_records";
   function thinkthen_tag_batch_start
     (e : access constant thinkthen_engine;
      q : access constant thinkthen_question;
      s : access constant thinkthen_source;
      c : access constant thinkthen_controls_v1;
      c_out : System.Address) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_tag_batch_start";
   function thinkthen_tag_complete
     (engine : access constant thinkthen_engine;
      question : access constant thinkthen_question;
      source : access constant thinkthen_source;
      controls : access constant thinkthen_controls_v1;
      c_out : System.Address) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_tag_complete";
end Thinkthen_Session_C;
pragma Style_Checks (On);
pragma Warnings (On, "-gnatwu");
