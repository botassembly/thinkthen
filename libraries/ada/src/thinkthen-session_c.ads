-- Generated from the canonical C header by GNAT. Do not edit.
with Ada.Unchecked_Conversion;
with Interfaces;
pragma Style_Checks (Off);
pragma Warnings (Off, "-gnatwu");
with Interfaces.C; use Interfaces.C;
with Interfaces.C.Strings;
with System;
private package Thinkthen.Session_C is
   K_THINKTHEN_COBOL_OVERFLOW : constant := 1002;
   K_THINKTHEN_COMPLETE_ANNOTATED_FIELD_ARRAY : constant := 4;
   K_THINKTHEN_COMPLETE_ANNOTATED_FIELD_BOOLEAN : constant := 1;
   K_THINKTHEN_COMPLETE_ANNOTATED_FIELD_NULL : constant := 2;
   K_THINKTHEN_COMPLETE_ANNOTATED_FIELD_NUMBER : constant := 5;
   K_THINKTHEN_COMPLETE_ANNOTATED_FIELD_OBJECT : constant := 6;
   K_THINKTHEN_COMPLETE_ANNOTATED_FIELD_STRING : constant := 3;
   K_THINKTHEN_COMPLETE_ANNOTATION_MEMBER_ANSWER_ID : constant := 1;
   K_THINKTHEN_COMPLETE_ANNOTATION_MEMBER_FAILURE_ID : constant := 2;
   K_THINKTHEN_COMPLETE_ANNOTATION_VALUE_CHOICE : constant := 2;
   K_THINKTHEN_COMPLETE_ANNOTATION_VALUE_DECISION : constant := 1;
   K_THINKTHEN_COMPLETE_ANNOTATION_VALUE_FAILED : constant := 5;
   K_THINKTHEN_COMPLETE_ANNOTATION_VALUE_SCORE : constant := 3;
   K_THINKTHEN_COMPLETE_ANNOTATION_VALUE_TAGS : constant := 4;
   K_THINKTHEN_COMPLETE_ANSWER_CHOICE : constant := 2;
   K_THINKTHEN_COMPLETE_ANSWER_RESOLUTIONS_OBJECT : constant := 2;
   K_THINKTHEN_COMPLETE_ANSWER_RESOLUTIONS_STRING : constant := 1;
   K_THINKTHEN_COMPLETE_ANSWER_RESOLUTION_FAILED : constant := 3;
   K_THINKTHEN_COMPLETE_ANSWER_RESOLUTION_RESOLVED : constant := 1;
   K_THINKTHEN_COMPLETE_ANSWER_RESOLUTION_UNRESOLVED : constant := 2;
   K_THINKTHEN_COMPLETE_ANSWER_SCORE : constant := 4;
   K_THINKTHEN_COMPLETE_ANSWER_SHAPE_10_ANY_OF_1_ITEMS_ARRAY : constant := 1;
   K_THINKTHEN_COMPLETE_ANSWER_SHAPE_10_ANY_OF_1_ITEMS_NULL : constant := 2;
   K_THINKTHEN_COMPLETE_ANSWER_SHAPE_10_ARRAY : constant := 1;
   K_THINKTHEN_COMPLETE_ANSWER_SHAPE_10_ARRAY2 : constant := 3;
   K_THINKTHEN_COMPLETE_ANSWER_SHAPE_10_NULL : constant := 2;
   K_THINKTHEN_COMPLETE_ANSWER_SHAPE_11_ANY_OF_1_ITEMS_ARRAY : constant := 1;
   K_THINKTHEN_COMPLETE_ANSWER_SHAPE_11_ANY_OF_1_ITEMS_NULL : constant := 2;
   K_THINKTHEN_COMPLETE_ANSWER_SHAPE_11_ARRAY : constant := 1;
   K_THINKTHEN_COMPLETE_ANSWER_SHAPE_11_ARRAY2 : constant := 3;
   K_THINKTHEN_COMPLETE_ANSWER_SHAPE_11_NULL : constant := 2;
   K_THINKTHEN_COMPLETE_ANSWER_SHAPE_13_ANY_OF_1_ITEMS_NULL : constant := 2;
   K_THINKTHEN_COMPLETE_ANSWER_SHAPE_13_ANY_OF_1_ITEMS_OBJECT : constant := 1;
   K_THINKTHEN_COMPLETE_ANSWER_SHAPE_13_ARRAY : constant := 3;
   K_THINKTHEN_COMPLETE_ANSWER_SHAPE_13_NULL : constant := 2;
   K_THINKTHEN_COMPLETE_ANSWER_SHAPE_13_OBJECT : constant := 1;
   K_THINKTHEN_COMPLETE_ANSWER_SHAPE_14_ANY_OF_1_ITEMS_ARRAY : constant := 1;
   K_THINKTHEN_COMPLETE_ANSWER_SHAPE_14_ANY_OF_1_ITEMS_NULL : constant := 2;
   K_THINKTHEN_COMPLETE_ANSWER_SHAPE_14_ARRAY : constant := 1;
   K_THINKTHEN_COMPLETE_ANSWER_SHAPE_14_ARRAY2 : constant := 3;
   K_THINKTHEN_COMPLETE_ANSWER_SHAPE_14_NULL : constant := 2;
   K_THINKTHEN_COMPLETE_ANSWER_SHAPE_15_ANY_OF_1_ITEMS_NULL : constant := 2;
   K_THINKTHEN_COMPLETE_ANSWER_SHAPE_15_ANY_OF_1_ITEMS_OBJECT : constant := 1;
   K_THINKTHEN_COMPLETE_ANSWER_SHAPE_15_ARRAY : constant := 3;
   K_THINKTHEN_COMPLETE_ANSWER_SHAPE_15_NULL : constant := 2;
   K_THINKTHEN_COMPLETE_ANSWER_SHAPE_15_OBJECT : constant := 1;
   K_THINKTHEN_COMPLETE_ANSWER_SHAPE_2_ARRAY : constant := 3;
   K_THINKTHEN_COMPLETE_ANSWER_SHAPE_2_NULL : constant := 2;
   K_THINKTHEN_COMPLETE_ANSWER_SHAPE_2_NUMBER : constant := 1;
   K_THINKTHEN_COMPLETE_ANSWER_SHAPE_3_ARRAY : constant := 3;
   K_THINKTHEN_COMPLETE_ANSWER_SHAPE_3_NULL : constant := 2;
   K_THINKTHEN_COMPLETE_ANSWER_SHAPE_3_STRING : constant := 1;
   K_THINKTHEN_COMPLETE_ANSWER_SHAPE_4_ANY_OF_1_ITEMS_INTEGER : constant := 1;
   K_THINKTHEN_COMPLETE_ANSWER_SHAPE_4_ANY_OF_1_ITEMS_NULL : constant := 2;
   K_THINKTHEN_COMPLETE_ANSWER_SHAPE_4_ARRAY : constant := 3;
   K_THINKTHEN_COMPLETE_ANSWER_SHAPE_4_INTEGER : constant := 1;
   K_THINKTHEN_COMPLETE_ANSWER_SHAPE_4_NULL : constant := 2;
   K_THINKTHEN_COMPLETE_ANSWER_SHAPE_5_ARRAY : constant := 3;
   K_THINKTHEN_COMPLETE_ANSWER_SHAPE_5_OBJECT : constant := 2;
   K_THINKTHEN_COMPLETE_ANSWER_SHAPE_5_STRING : constant := 1;
   K_THINKTHEN_COMPLETE_ANSWER_SHAPE_6_ANY_OF_1_ITEMS_NULL : constant := 2;
   K_THINKTHEN_COMPLETE_ANSWER_SHAPE_6_ANY_OF_1_ITEMS_STRING : constant := 1;
   K_THINKTHEN_COMPLETE_ANSWER_SHAPE_6_ARRAY : constant := 3;
   K_THINKTHEN_COMPLETE_ANSWER_SHAPE_6_NULL : constant := 2;
   K_THINKTHEN_COMPLETE_ANSWER_SHAPE_6_STRING : constant := 1;
   K_THINKTHEN_COMPLETE_ANSWER_SHAPE_7_ANY_OF_1_ITEMS_ARRAY : constant := 1;
   K_THINKTHEN_COMPLETE_ANSWER_SHAPE_7_ANY_OF_1_ITEMS_NULL : constant := 2;
   K_THINKTHEN_COMPLETE_ANSWER_SHAPE_7_ARRAY : constant := 1;
   K_THINKTHEN_COMPLETE_ANSWER_SHAPE_7_ARRAY2 : constant := 3;
   K_THINKTHEN_COMPLETE_ANSWER_SHAPE_7_NULL : constant := 2;
   K_THINKTHEN_COMPLETE_ANSWER_SHAPE_8_ANY_OF_1_ITEMS_NULL : constant := 2;
   K_THINKTHEN_COMPLETE_ANSWER_SHAPE_8_ANY_OF_1_ITEMS_NUMBER : constant := 1;
   K_THINKTHEN_COMPLETE_ANSWER_SHAPE_8_ARRAY : constant := 3;
   K_THINKTHEN_COMPLETE_ANSWER_SHAPE_8_NULL : constant := 2;
   K_THINKTHEN_COMPLETE_ANSWER_SHAPE_8_NUMBER : constant := 1;
   K_THINKTHEN_COMPLETE_ANSWER_SHAPE_9_ANY_OF_1_ITEMS_ARRAY : constant := 1;
   K_THINKTHEN_COMPLETE_ANSWER_SHAPE_9_ANY_OF_1_ITEMS_NULL : constant := 2;
   K_THINKTHEN_COMPLETE_ANSWER_SHAPE_9_ARRAY : constant := 1;
   K_THINKTHEN_COMPLETE_ANSWER_SHAPE_9_ARRAY2 : constant := 3;
   K_THINKTHEN_COMPLETE_ANSWER_SHAPE_9_NULL : constant := 2;
   K_THINKTHEN_COMPLETE_ANSWER_TAG : constant := 3;
   K_THINKTHEN_COMPLETE_ANSWER_YES_NO : constant := 1;
   K_THINKTHEN_COMPLETE_ATTEMPT_OUTCOME_OK : constant := 1;
   K_THINKTHEN_COMPLETE_ATTEMPT_OUTCOME_STATUS : constant := 2;
   K_THINKTHEN_COMPLETE_ATTEMPT_OUTCOME_TRANSPORT : constant := 3;
   K_THINKTHEN_COMPLETE_BATCH_INTEGER : constant := 1;
   K_THINKTHEN_COMPLETE_BATCH_SETTING_INTEGER : constant := 1;
   K_THINKTHEN_COMPLETE_BATCH_SETTING_STRING : constant := 2;
   K_THINKTHEN_COMPLETE_BATCH_STRING : constant := 2;
   K_THINKTHEN_COMPLETE_BATCH_STRING_MAX : constant := 1;
   K_THINKTHEN_COMPLETE_BOUNDARY_MODE_BOUNDARY_ONLY : constant := 1;
   K_THINKTHEN_COMPLETE_FAILURE_CAUSE_INVALID_DISTRIBUTION : constant := 5;
   K_THINKTHEN_COMPLETE_FAILURE_CAUSE_INVALID_PROBABILITY : constant := 4;
   K_THINKTHEN_COMPLETE_FAILURE_CAUSE_MISSING_ANSWER : constant := 1;
   K_THINKTHEN_COMPLETE_FAILURE_CAUSE_MISSING_PROBABILITY : constant := 3;
   K_THINKTHEN_COMPLETE_FAILURE_CAUSE_UNEXPECTED_PROBABILITY : constant := 6;
   K_THINKTHEN_COMPLETE_FAILURE_CAUSE_WRONG_KIND : constant := 2;
   K_THINKTHEN_COMPLETE_FAILURE_FIELD_KIND_BACKEND : constant := 1;
   K_THINKTHEN_COMPLETE_FAILURE_KIND_BACKEND : constant := 2;
   K_THINKTHEN_COMPLETE_FAILURE_KIND_CANCELLED : constant := 4;
   K_THINKTHEN_COMPLETE_FAILURE_KIND_DEADLINE : constant := 5;
   K_THINKTHEN_COMPLETE_FAILURE_KIND_DEFECT : constant := 6;
   K_THINKTHEN_COMPLETE_FAILURE_KIND_LOCAL : constant := 3;
   K_THINKTHEN_COMPLETE_FAILURE_KIND_USAGE : constant := 1;
   K_THINKTHEN_COMPLETE_IMAGE_MEDIA_IMAGE_JPEG : constant := 1;
   K_THINKTHEN_COMPLETE_IMAGE_MEDIA_IMAGE_PNG : constant := 2;
   K_THINKTHEN_COMPLETE_INPUT_DECLARATION_OBJECT_ROOT : constant := 2;
   K_THINKTHEN_COMPLETE_INPUT_DECLARATION_STRING_ROOT : constant := 1;
   K_THINKTHEN_COMPLETE_INPUT_DENIAL_ADDITIONAL_REQUEST : constant := 2;
   K_THINKTHEN_COMPLETE_INPUT_DENIAL_INITIAL_REQUEST : constant := 1;
   K_THINKTHEN_COMPLETE_INPUT_DENIAL_RETRY : constant := 3;
   K_THINKTHEN_COMPLETE_INPUT_PROPERTY_TYPE_ARRAY : constant := 4;
   K_THINKTHEN_COMPLETE_INPUT_PROPERTY_TYPE_BOOLEAN : constant := 3;
   K_THINKTHEN_COMPLETE_INPUT_PROPERTY_TYPE_NUMBER : constant := 2;
   K_THINKTHEN_COMPLETE_INPUT_PROPERTY_TYPE_STRING : constant := 1;
   K_THINKTHEN_COMPLETE_JSON_ARRAY : constant := 5;
   K_THINKTHEN_COMPLETE_JSON_BOOLEAN : constant := 2;
   K_THINKTHEN_COMPLETE_JSON_NULL : constant := 1;
   K_THINKTHEN_COMPLETE_JSON_NUMBER : constant := 3;
   K_THINKTHEN_COMPLETE_JSON_OBJECT : constant := 6;
   K_THINKTHEN_COMPLETE_JSON_STRING : constant := 4;
   K_THINKTHEN_COMPLETE_OBJECT_TYPE_OBJECT : constant := 1;
   K_THINKTHEN_COMPLETE_OBSERVATION_FAILURE_ID : constant := 2;
   K_THINKTHEN_COMPLETE_OBSERVATION_OBSERVATION_ID : constant := 1;
   K_THINKTHEN_COMPLETE_ORIGIN_CACHE : constant := 2;
   K_THINKTHEN_COMPLETE_ORIGIN_LIVE : constant := 1;
   K_THINKTHEN_COMPLETE_ORIGIN_MEMORY : constant := 5;
   K_THINKTHEN_COMPLETE_ORIGIN_PROXY : constant := 4;
   K_THINKTHEN_COMPLETE_ORIGIN_REPLAY : constant := 3;
   K_THINKTHEN_COMPLETE_QUESTION_DOCUMENT_CHOOSE : constant := 2;
   K_THINKTHEN_COMPLETE_QUESTION_DOCUMENT_DECIDE : constant := 1;
   K_THINKTHEN_COMPLETE_QUESTION_DOCUMENT_SCORE : constant := 4;
   K_THINKTHEN_COMPLETE_QUESTION_DOCUMENT_TAG : constant := 3;
   K_THINKTHEN_COMPLETE_RECOGNITION_MODE_BOUNDARY_ONLY : constant := 2;
   K_THINKTHEN_COMPLETE_RECOGNITION_MODE_WHOLE : constant := 1;
   K_THINKTHEN_COMPLETE_RECOGNITION_ODDS_BOUNDARY_ODDS : constant := 2;
   K_THINKTHEN_COMPLETE_RECOGNITION_ODDS_RECOGNIZE_ANSWER : constant := 1;
   K_THINKTHEN_COMPLETE_RECOGNIZE_ENTITIES : constant := 1;
   K_THINKTHEN_COMPLETE_RECOGNIZE_MODE_PROPOSALS : constant := 2;
   K_THINKTHEN_COMPLETE_RELATION_DIRECTION_EITHER : constant := 2;
   K_THINKTHEN_COMPLETE_RELATION_DIRECTION_SOURCE_TO_TARGET : constant := 1;
   K_THINKTHEN_COMPLETE_RELATION_ENDPOINT_RELATED_ENTITY : constant := 1;
   K_THINKTHEN_COMPLETE_RELATION_ENDPOINT_SOURCE : constant := 2;
   K_THINKTHEN_COMPLETE_RELATION_MEMBER_ANSWER_ID : constant := 1;
   K_THINKTHEN_COMPLETE_RELATION_MEMBER_FAILURE_ID : constant := 2;
   K_THINKTHEN_COMPLETE_RELATION_METHOD_CHOICE : constant := 2;
   K_THINKTHEN_COMPLETE_RELATION_METHOD_YES_NO : constant := 1;
   K_THINKTHEN_COMPLETE_REQUEST_FUNCTION_ANNOTATE : constant := 8;
   K_THINKTHEN_COMPLETE_REQUEST_FUNCTION_CHOOSE : constant := 2;
   K_THINKTHEN_COMPLETE_REQUEST_FUNCTION_DECIDE : constant := 1;
   K_THINKTHEN_COMPLETE_REQUEST_FUNCTION_FILTER : constant := 5;
   K_THINKTHEN_COMPLETE_REQUEST_FUNCTION_FIND : constant := 7;
   K_THINKTHEN_COMPLETE_REQUEST_FUNCTION_RANK : constant := 6;
   K_THINKTHEN_COMPLETE_REQUEST_FUNCTION_RECOGNIZE : constant := 9;
   K_THINKTHEN_COMPLETE_REQUEST_FUNCTION_RELATE : constant := 10;
   K_THINKTHEN_COMPLETE_REQUEST_FUNCTION_SCORE : constant := 4;
   K_THINKTHEN_COMPLETE_REQUEST_FUNCTION_TAG : constant := 3;
   K_THINKTHEN_COMPLETE_RESULT_TAG_THINKTHEN_RESULT : constant := 1;
   K_THINKTHEN_COMPLETE_SEND_DENIAL_BEFORE_ADDITIONAL_SEND : constant := 2;
   K_THINKTHEN_COMPLETE_SEND_DENIAL_BEFORE_FIRST_SEND : constant := 1;
   K_THINKTHEN_COMPLETE_SEND_DENIAL_BEFORE_RETRY : constant := 3;
   K_THINKTHEN_COMPLETE_SESSION_JUDGMENT_CHOICE : constant := 2;
   K_THINKTHEN_COMPLETE_SESSION_JUDGMENT_DECISION : constant := 1;
   K_THINKTHEN_COMPLETE_SESSION_JUDGMENT_SCORE : constant := 3;
   K_THINKTHEN_COMPLETE_SESSION_JUDGMENT_TAGS : constant := 4;
   K_THINKTHEN_COMPLETE_SESSION_OBSERVATION_QUESTION : constant := 1;
   K_THINKTHEN_COMPLETE_SESSION_OBSERVATION_ROW : constant := 2;
   K_THINKTHEN_COMPLETE_SESSION_OBSERVED_ROW_ANNOTATED : constant := 2;
   K_THINKTHEN_COMPLETE_SESSION_OBSERVED_ROW_FIND : constant := 4;
   K_THINKTHEN_COMPLETE_SESSION_OBSERVED_ROW_JUDGMENT : constant := 1;
   K_THINKTHEN_COMPLETE_SESSION_OBSERVED_ROW_RECOGNIZED : constant := 3;
   K_THINKTHEN_COMPLETE_SESSION_OBSERVED_ROW_RELATIONS : constant := 5;
   K_THINKTHEN_COMPLETE_SESSION_PACKET_ANNOTATE_AGGREGATE : constant := 14;
   K_THINKTHEN_COMPLETE_SESSION_PACKET_ANNOTATE_ROW : constant := 6;
   K_THINKTHEN_COMPLETE_SESSION_PACKET_CHOOSE_AGGREGATE : constant := 8;
   K_THINKTHEN_COMPLETE_SESSION_PACKET_CHOOSE_ROW : constant := 2;
   K_THINKTHEN_COMPLETE_SESSION_PACKET_DECIDE_AGGREGATE : constant := 7;
   K_THINKTHEN_COMPLETE_SESSION_PACKET_DECIDE_ROW : constant := 1;
   K_THINKTHEN_COMPLETE_SESSION_PACKET_FILTER_AGGREGATE : constant := 11;
   K_THINKTHEN_COMPLETE_SESSION_PACKET_FILTER_ROW : constant := 5;
   K_THINKTHEN_COMPLETE_SESSION_PACKET_FIND_AGGREGATE : constant := 13;
   K_THINKTHEN_COMPLETE_SESSION_PACKET_OBSERVATION : constant := 17;
   K_THINKTHEN_COMPLETE_SESSION_PACKET_RANK_AGGREGATE : constant := 12;
   K_THINKTHEN_COMPLETE_SESSION_PACKET_RECOGNIZE_AGGREGATE : constant := 15;
   K_THINKTHEN_COMPLETE_SESSION_PACKET_RELATE_AGGREGATE : constant := 16;
   K_THINKTHEN_COMPLETE_SESSION_PACKET_SCORE_AGGREGATE : constant := 10;
   K_THINKTHEN_COMPLETE_SESSION_PACKET_SCORE_ROW : constant := 4;
   K_THINKTHEN_COMPLETE_SESSION_PACKET_TAG_AGGREGATE : constant := 9;
   K_THINKTHEN_COMPLETE_SESSION_PACKET_TAG_ROW : constant := 3;
   K_THINKTHEN_COMPLETE_SESSION_PACKET_TERMINAL : constant := 18;
   K_THINKTHEN_COMPLETE_SESSION_PROBABILITIES_NAMED : constant := 2;
   K_THINKTHEN_COMPLETE_SESSION_PROBABILITIES_YES_NO : constant := 1;
   K_THINKTHEN_COMPLETE_SETTLED_ANSWER_ANNOTATE : constant := 10;
   K_THINKTHEN_COMPLETE_SETTLED_ANSWER_CHOOSE : constant := 2;
   K_THINKTHEN_COMPLETE_SETTLED_ANSWER_DECIDE : constant := 1;
   K_THINKTHEN_COMPLETE_SETTLED_ANSWER_FILTER : constant := 5;
   K_THINKTHEN_COMPLETE_SETTLED_ANSWER_FIND : constant := 7;
   K_THINKTHEN_COMPLETE_SETTLED_ANSWER_RANK : constant := 6;
   K_THINKTHEN_COMPLETE_SETTLED_ANSWER_RECOGNIZE : constant := 8;
   K_THINKTHEN_COMPLETE_SETTLED_ANSWER_RELATE : constant := 9;
   K_THINKTHEN_COMPLETE_SETTLED_ANSWER_SCORE : constant := 4;
   K_THINKTHEN_COMPLETE_SETTLED_ANSWER_TAG : constant := 3;
   K_THINKTHEN_COMPLETE_SETTLED_JUDGMENT_ARRAY : constant := 5;
   K_THINKTHEN_COMPLETE_SETTLED_JUDGMENT_BOOLEAN : constant := 1;
   K_THINKTHEN_COMPLETE_SETTLED_JUDGMENT_NULL : constant := 2;
   K_THINKTHEN_COMPLETE_SETTLED_JUDGMENT_NUMBER : constant := 4;
   K_THINKTHEN_COMPLETE_SETTLED_JUDGMENT_STRING : constant := 3;
   K_THINKTHEN_COMPLETE_SETTLED_RECOGNITION_ENTITIES : constant := 1;
   K_THINKTHEN_COMPLETE_SETTLED_RECOGNITION_MODE_PROPOSALS : constant := 2;
   K_THINKTHEN_COMPLETE_STOP_CAUSE_BACKEND : constant := 8;
   K_THINKTHEN_COMPLETE_STOP_CAUSE_CANCELLED : constant := 9;
   K_THINKTHEN_COMPLETE_STOP_CAUSE_DEADLINE : constant := 10;
   K_THINKTHEN_COMPLETE_STOP_CAUSE_DEFECT : constant := 11;
   K_THINKTHEN_COMPLETE_STOP_CAUSE_LOCAL : constant := 2;
   K_THINKTHEN_COMPLETE_STOP_CAUSE_NO_KEY : constant := 3;
   K_THINKTHEN_COMPLETE_STOP_CAUSE_REPLY : constant := 7;
   K_THINKTHEN_COMPLETE_STOP_CAUSE_STATUS : constant := 5;
   K_THINKTHEN_COMPLETE_STOP_CAUSE_TOO_LARGE : constant := 6;
   K_THINKTHEN_COMPLETE_STOP_CAUSE_TRANSPORT : constant := 4;
   K_THINKTHEN_COMPLETE_STOP_CAUSE_USAGE : constant := 1;
   K_THINKTHEN_COMPLETE_STRING_TYPE_STRING : constant := 1;
   K_THINKTHEN_COMPLETE_THRESHOLD_NUMBER : constant := 1;
   K_THINKTHEN_COMPLETE_THRESHOLD_STRING : constant := 2;
   K_THINKTHEN_COMPLETE_USAGE_PERSISTENCE_DISABLED : constant := 1;
   K_THINKTHEN_COMPLETE_USAGE_PERSISTENCE_FAILED : constant := 4;
   K_THINKTHEN_COMPLETE_USAGE_PERSISTENCE_PENDING : constant := 2;
   K_THINKTHEN_COMPLETE_USAGE_PERSISTENCE_WRITTEN : constant := 3;
   K_THINKTHEN_COMPLETE_VALUE_ARRAY : constant := 4;
   K_THINKTHEN_COMPLETE_VALUE_BOOLEAN : constant := 1;
   K_THINKTHEN_COMPLETE_VALUE_NULL : constant := 2;
   K_THINKTHEN_COMPLETE_VALUE_NUMBER : constant := 5;
   K_THINKTHEN_COMPLETE_VALUE_STRING : constant := 3;
   K_THINKTHEN_COMPLETE_VERB_RECOGNIZE : constant := 1;
   K_THINKTHEN_EBACKEND : constant := 2;
   K_THINKTHEN_ECANCELLED : constant := 5;
   K_THINKTHEN_EDEADLINE : constant := 3;
   K_THINKTHEN_EDEFECT : constant := 6;
   K_THINKTHEN_ELOCAL : constant := 4;
   K_THINKTHEN_EUSAGE : constant := 1;
   K_THINKTHEN_REQUEST_ANNOTATE_QUESTION_CHOOSE : constant := 2;
   K_THINKTHEN_REQUEST_ANNOTATE_QUESTION_DECIDE : constant := 1;
   K_THINKTHEN_REQUEST_ANNOTATE_QUESTION_SCORE : constant := 4;
   K_THINKTHEN_REQUEST_ANNOTATE_QUESTION_TAG : constant := 3;
   K_THINKTHEN_REQUEST_BATCH_TEXT : constant := 2;
   K_THINKTHEN_REQUEST_BATCH_UNSIGNED_VALUE : constant := 1;
   K_THINKTHEN_REQUEST_CACHE_DOCUMENT_DISABLED_CACHE : constant := 2;
   K_THINKTHEN_REQUEST_CACHE_DOCUMENT_TEXT : constant := 1;
   K_THINKTHEN_REQUEST_CALL_THRESHOLD_NUMBER : constant := 1;
   K_THINKTHEN_REQUEST_CALL_THRESHOLD_TEXT : constant := 2;
   K_THINKTHEN_REQUEST_CONTEXT_SCHEMA_JSON_VALUE_MAP : constant := 2;
   K_THINKTHEN_REQUEST_CONTEXT_SCHEMA_TEXT : constant := 1;
   K_THINKTHEN_REQUEST_CRITERION_JSON_VALUE_LIST : constant := 3;
   K_THINKTHEN_REQUEST_CRITERION_JSON_VALUE_MAP : constant := 2;
   K_THINKTHEN_REQUEST_CRITERION_NULL : constant := 4;
   K_THINKTHEN_REQUEST_CRITERION_TEXT : constant := 1;
   K_THINKTHEN_REQUEST_CUT_NUMBER : constant := 1;
   K_THINKTHEN_REQUEST_CUT_TEXT : constant := 2;
   K_THINKTHEN_REQUEST_DEFINITION_ANNOTATE_DEFINITION : constant := 8;
   K_THINKTHEN_REQUEST_DEFINITION_AUTHORED_CHOOSE : constant := 2;
   K_THINKTHEN_REQUEST_DEFINITION_AUTHORED_DECIDE : constant := 1;
   K_THINKTHEN_REQUEST_DEFINITION_AUTHORED_FIND : constant := 6;
   K_THINKTHEN_REQUEST_DEFINITION_AUTHORED_RELATE : constant := 5;
   K_THINKTHEN_REQUEST_DEFINITION_AUTHORED_SCORE : constant := 4;
   K_THINKTHEN_REQUEST_DEFINITION_AUTHORED_TAG : constant := 3;
   K_THINKTHEN_REQUEST_DEFINITION_RECOGNIZE_DEFINITION : constant := 7;
   K_THINKTHEN_REQUEST_DESCRIPTION_JSON_VALUE_LIST : constant := 3;
   K_THINKTHEN_REQUEST_DESCRIPTION_JSON_VALUE_MAP : constant := 2;
   K_THINKTHEN_REQUEST_DESCRIPTION_NULL : constant := 4;
   K_THINKTHEN_REQUEST_DESCRIPTION_TEXT : constant := 1;
   K_THINKTHEN_REQUEST_FRAMING_LITERAL_CSV : constant := 4;
   K_THINKTHEN_REQUEST_FRAMING_LITERAL_DOCUMENT : constant := 1;
   K_THINKTHEN_REQUEST_FRAMING_LITERAL_JSONL : constant := 3;
   K_THINKTHEN_REQUEST_FRAMING_LITERAL_LINES : constant := 2;
   K_THINKTHEN_REQUEST_FRAMING_LITERAL_TSV : constant := 5;
   K_THINKTHEN_REQUEST_IMAGE_BYTES : constant := 2;
   K_THINKTHEN_REQUEST_IMAGE_FILE : constant := 1;
   K_THINKTHEN_REQUEST_IMAGE_MEDIA_LITERAL_IMAGE_JPEG : constant := 1;
   K_THINKTHEN_REQUEST_IMAGE_MEDIA_LITERAL_IMAGE_PNG : constant := 2;
   K_THINKTHEN_REQUEST_INPUT_DECLARATION_OBJECT : constant := 2;
   K_THINKTHEN_REQUEST_INPUT_DECLARATION_STRING : constant := 1;
   K_THINKTHEN_REQUEST_INPUT_ENTITIES : constant := 5;
   K_THINKTHEN_REQUEST_INPUT_FEED : constant := 7;
   K_THINKTHEN_REQUEST_INPUT_JSON : constant := 2;
   K_THINKTHEN_REQUEST_INPUT_PROPERTY_ARRAY : constant := 4;
   K_THINKTHEN_REQUEST_INPUT_PROPERTY_BOOLEAN : constant := 3;
   K_THINKTHEN_REQUEST_INPUT_PROPERTY_NUMBER : constant := 2;
   K_THINKTHEN_REQUEST_INPUT_PROPERTY_STRING : constant := 1;
   K_THINKTHEN_REQUEST_INPUT_RECORDS : constant := 3;
   K_THINKTHEN_REQUEST_INPUT_SOURCE : constant := 6;
   K_THINKTHEN_REQUEST_INPUT_TEXT : constant := 1;
   K_THINKTHEN_REQUEST_INPUT_UNITS : constant := 4;
   K_THINKTHEN_REQUEST_LABELS_DESCRIPTION_MAP : constant := 2;
   K_THINKTHEN_REQUEST_LABELS_NAME_LIST : constant := 1;
   K_THINKTHEN_REQUEST_LEVELS_CRITERION_MAP : constant := 2;
   K_THINKTHEN_REQUEST_LEVELS_NAME_LIST : constant := 1;
   K_THINKTHEN_REQUEST_OPTIONAL_COUNT_NULL : constant := 2;
   K_THINKTHEN_REQUEST_OPTIONAL_COUNT_UNSIGNED_VALUE : constant := 1;
   K_THINKTHEN_REQUEST_OPTIONAL_TOTAL_NULL : constant := 2;
   K_THINKTHEN_REQUEST_OPTIONAL_TOTAL_UNSIGNED_VALUE : constant := 1;
   K_THINKTHEN_REQUEST_OPTIONS_DESCRIPTION_MAP : constant := 2;
   K_THINKTHEN_REQUEST_OPTIONS_NAME_LIST : constant := 1;
   K_THINKTHEN_REQUEST_ORIGINAL_JSON : constant := 2;
   K_THINKTHEN_REQUEST_ORIGINAL_TEXT : constant := 1;
   K_THINKTHEN_REQUEST_POINTERS_TEXT : constant := 1;
   K_THINKTHEN_REQUEST_POINTERS_TEXT_LIST : constant := 2;
   K_THINKTHEN_REQUEST_QUESTION_BATCH_INTEGER : constant := 2;
   K_THINKTHEN_REQUEST_QUESTION_BATCH_LITERAL_MAX : constant := 1;
   K_THINKTHEN_REQUEST_QUESTION_SELECTOR_DEFINITION : constant := 2;
   K_THINKTHEN_REQUEST_QUESTION_SELECTOR_FILE : constant := 3;
   K_THINKTHEN_REQUEST_QUESTION_SELECTOR_NAME : constant := 4;
   K_THINKTHEN_REQUEST_QUESTION_SELECTOR_REFERENCE : constant := 5;
   K_THINKTHEN_REQUEST_QUESTION_SELECTOR_TEXT : constant := 1;
   K_THINKTHEN_REQUEST_QUESTION_TEXT_JSON_VALUE_LIST : constant := 3;
   K_THINKTHEN_REQUEST_QUESTION_TEXT_JSON_VALUE_MAP : constant := 2;
   K_THINKTHEN_REQUEST_QUESTION_TEXT_TEXT : constant := 1;
   K_THINKTHEN_REQUEST_READER_FAILURE_INVALID_INPUT : constant := 4;
   K_THINKTHEN_REQUEST_READER_FAILURE_IO : constant := 2;
   K_THINKTHEN_REQUEST_READER_FAILURE_PRODUCER : constant := 1;
   K_THINKTHEN_REQUEST_READER_FAILURE_UTF_8 : constant := 3;
   K_THINKTHEN_REQUEST_READER_MEDIA_LITERAL_IMAGE : constant := 2;
   K_THINKTHEN_REQUEST_READER_MEDIA_LITERAL_TEXT : constant := 1;
   K_THINKTHEN_REQUEST_RECOGNITION_EXAMPLE_EXAMPLE_TEXT : constant := 2;
   K_THINKTHEN_REQUEST_RECOGNITION_EXAMPLE_TEXT : constant := 1;
   K_THINKTHEN_REQUEST_RECOGNITION_MODE_LITERAL_BOUNDARY_ONLY : constant := 2;
   K_THINKTHEN_REQUEST_RECOGNITION_MODE_LITERAL_WHOLE : constant := 1;
   K_THINKTHEN_REQUEST_SOURCE_UNIT_LITERAL_FILE : constant := 3;
   K_THINKTHEN_REQUEST_SOURCE_UNIT_LITERAL_LINE : constant := 1;
   K_THINKTHEN_REQUEST_SOURCE_UNIT_LITERAL_WINDOW : constant := 2;
   K_THINKTHEN_REQUEST_THRESHOLD_NUMBER : constant := 1;
   K_THINKTHEN_REQUEST_THRESHOLD_TEXT : constant := 2;
   K_THINKTHEN_SESSION_ACCEPTED : constant := 0;
   K_THINKTHEN_SESSION_CLOSED : constant := 2;
   K_THINKTHEN_SESSION_END : constant := 2;
   K_THINKTHEN_SESSION_FULL : constant := 1;
   K_THINKTHEN_SESSION_PENDING : constant := 1;
   K_THINKTHEN_SESSION_READY_CANCELLED : constant := 4;
   K_THINKTHEN_SESSION_READY_CLOSED : constant := 2;
   K_THINKTHEN_SESSION_READY_END : constant := 3;
   K_THINKTHEN_SESSION_READY_INPUT : constant := 1;
   K_THINKTHEN_SESSION_READY_RESULT : constant := 0;
   K_THINKTHEN_SESSION_READY_TIMEOUT : constant := 5;
   K_THINKTHEN_SESSION_RESULT : constant := 0;
   K_THINKTHEN_SESSION_WAIT_INPUT : constant := 1;
   K_THINKTHEN_SESSION_WAIT_INTERRUPTIBLE_SETTLEMENT : constant := 4;
   K_THINKTHEN_SESSION_WAIT_RESULT : constant := 0;
   K_THINKTHEN_SESSION_WAIT_SETTLEMENT : constant := 2;
   K_THINKTHEN_SESSION_WAIT_SETTLEMENT_ARM : constant := 3;
   type thinkthen_call_error is null record;
   type thinkthen_engine is null record;
   type thinkthen_question is null record;
   type thinkthen_session is null record;
   type thinkthen_session_result is null record;
   type thinkthen_value is null record;
   type thinkthen_complete_utf8 is record
      data : Interfaces.C.Strings.chars_ptr;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_usage_persistence is record
      kind : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_text is record
      text : aliased thinkthen_complete_utf8;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_unsigned is record
      value : aliased Interfaces.Unsigned_64;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_batch_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            unsigned_value : access constant thinkthen_request_unsigned;
         when others =>
            text : access constant thinkthen_request_text;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_request_batch is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_request_batch_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_disabled_cache is record
      reserved : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_cache_document_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            text : access constant thinkthen_request_text;
         when others =>
            disabled_cache : access constant thinkthen_request_disabled_cache;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_request_cache_document is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_request_cache_document_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_null is record
      reserved : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_optional_total_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            unsigned_value : access constant thinkthen_request_unsigned;
         when others =>
            c_null : access constant thinkthen_request_null;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_request_optional_total is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_request_optional_total_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_optional_count_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            unsigned_value : access constant thinkthen_request_unsigned;
         when others =>
            c_null : access constant thinkthen_request_null;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_request_optional_count is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_request_optional_count_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_json;
   type thinkthen_complete_json_array is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_json_entry is record
      name : aliased thinkthen_complete_utf8;
      value : access constant thinkthen_complete_json;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_json_object is record
      data : access constant thinkthen_complete_json_entry;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_json_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            boolean : aliased Interfaces.Unsigned_32;
         when 1 =>
            number : aliased thinkthen_complete_utf8;
         when 2 =>
            string : aliased thinkthen_complete_utf8;
         when 3 =>
            c_array : aliased thinkthen_complete_json_array;
         when others =>
            object : aliased thinkthen_complete_json_object;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_json is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_json_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_json_value is record
      value : access constant thinkthen_complete_json;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_boolean is record
      value : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_engine_settings is record
      m_backend : access constant thinkthen_request_text;
      m_base_url : access constant thinkthen_request_text;
      m_batch : access constant thinkthen_request_batch;
      m_cache : access constant thinkthen_request_cache_document;
      m_max_estimated_input_tokens_total : access constant thinkthen_request_optional_total;
      m_max_request_bytes : access constant thinkthen_request_unsigned;
      m_max_requests : access constant thinkthen_request_optional_count;
      m_max_requests_total : access constant thinkthen_request_optional_total;
      m_max_retries : access constant thinkthen_request_unsigned;
      m_model : access constant thinkthen_request_text;
      m_profile : access constant thinkthen_request_text;
      m_proxy : access constant thinkthen_request_json_value;
      m_record : access constant thinkthen_request_text;
      m_refresh_cache : access constant thinkthen_request_boolean;
      m_replay : access constant thinkthen_request_text;
      m_throttle : access constant thinkthen_request_unsigned;
      m_timeout : access constant thinkthen_request_unsigned;
      m_usd_per_million_input : access constant thinkthen_request_text;
      m_usd_per_million_output : access constant thinkthen_request_text;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_attempt_outcome is record
      kind : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_utf8_presence is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_utf8;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_sdk_request_id is record
      value : aliased thinkthen_complete_utf8;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_u64_presence is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased Interfaces.Unsigned_64;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_extension is record
      name : aliased thinkthen_complete_utf8;
      json : aliased thinkthen_complete_utf8;
      value : access constant thinkthen_complete_json;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_extensions is record
      data : access constant thinkthen_complete_extension;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_attempt is record
      ordinal : aliased Interfaces.Unsigned_64;
      outcome : access constant thinkthen_complete_attempt_outcome;
      request_id : aliased thinkthen_complete_utf8_presence;
      request_sha256 : aliased thinkthen_complete_utf8;
      sdk_request_id : access constant thinkthen_complete_sdk_request_id;
      server_ms : aliased thinkthen_complete_u64_presence;
      status : aliased thinkthen_complete_u64_presence;
      wall_ms : aliased Interfaces.Unsigned_64;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_attempt_list is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_attempt_list_presence is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_attempt_list;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_call_id is record
      value : aliased thinkthen_complete_utf8;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_call_id_presence is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_call_id;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_u32_presence is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_persistence_observation is record
      advice : aliased thinkthen_complete_utf8_presence;
      observed_at : aliased thinkthen_complete_utf8;
      state : access constant thinkthen_complete_usage_persistence;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_persistence_observation_presence is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_persistence_observation;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_immediate_facts is record
      attempts : aliased thinkthen_complete_attempt_list_presence;
      cache_answers : aliased Interfaces.Unsigned_64;
      call_id : aliased thinkthen_complete_call_id_presence;
      estimated_cost_usd : aliased thinkthen_complete_utf8_presence;
      held_model_mismatch : aliased thinkthen_complete_u32_presence;
      input_tokens : aliased thinkthen_complete_u64_presence;
      largest_request_bytes : aliased Interfaces.Unsigned_64;
      largest_request_estimated_input_tokens : aliased thinkthen_complete_u64_presence;
      model : aliased thinkthen_complete_utf8_presence;
      output_tokens : aliased thinkthen_complete_u64_presence;
      records : aliased Interfaces.Unsigned_64;
      requests_sent : aliased Interfaces.Unsigned_64;
      seconds : aliased double;
      token_estimate_method : aliased thinkthen_complete_utf8;
      usage_persistence : aliased thinkthen_complete_persistence_observation_presence;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_error_details is record
      code : aliased int;
      retryable : aliased int;
      message : aliased thinkthen_complete_utf8;
      facts : access constant thinkthen_complete_immediate_facts;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_string is record
      data : Interfaces.C.Strings.chars_ptr;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_optional_string is record
      present : aliased int;
      value : aliased thinkthen_string;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_input_property is record
      name : aliased thinkthen_string;
      kind : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_input_properties is record
      data : access constant thinkthen_input_property;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_strings is record
      data : access constant thinkthen_string;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_input_declaration is record
      kind : aliased Interfaces.Unsigned_32;
      properties : aliased thinkthen_input_properties;
      required : aliased thinkthen_strings;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_question_author_info is record
      name : aliased thinkthen_optional_string;
      item_schema : aliased thinkthen_input_declaration;
      context_schema : aliased thinkthen_input_declaration;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_named_request is record
      c_function : aliased thinkthen_complete_utf8;
      question_json : aliased thinkthen_complete_utf8;
      input_json : aliased thinkthen_complete_utf8;
      options_json : aliased thinkthen_complete_utf8;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_literal_image_jpeg is record
      reserved : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_literal_image_png is record
      reserved : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_image_media_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            literal_image_jpeg : access constant thinkthen_request_literal_image_jpeg;
         when others =>
            literal_image_png : access constant thinkthen_request_literal_image_png;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_request_image_media is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_request_image_media_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_image_file is record
      m_media : access constant thinkthen_request_image_media;
      m_path : access constant thinkthen_request_text;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_image_bytes is record
      m_bytes : access constant thinkthen_request_text;
      m_media : access constant thinkthen_request_image_media;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_image_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            file : access constant thinkthen_request_image_file;
         when others =>
            bytes : access constant thinkthen_request_image_bytes;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_request_image is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_request_image_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_image_list is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_input_text is record
      m_images : access constant thinkthen_request_image_list;
      m_text : access constant thinkthen_request_text;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_input_json_value is record
      value : access constant thinkthen_complete_json;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_input_json is record
      m_images : access constant thinkthen_request_image_list;
      m_value : access constant thinkthen_request_input_json_value;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_json_value_map_entry is record
      key : aliased thinkthen_complete_utf8;
      value : access constant thinkthen_request_json_value;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_json_value_map is record
      data : access constant thinkthen_request_json_value_map_entry;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_context_schema_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            text : access constant thinkthen_request_text;
         when others =>
            json_value_map : access constant thinkthen_request_json_value_map;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_request_context_schema is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_request_context_schema_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_recognition_example_entity is record
      m_end : access constant thinkthen_request_unsigned;
      m_kind : access constant thinkthen_request_text;
      m_start : access constant thinkthen_request_unsigned;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_recognition_example_entity_list is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_text_list is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_recognition_example_text is record
      m_entities : access constant thinkthen_request_recognition_example_entity_list;
      m_kinds : access constant thinkthen_request_text_list;
      m_text : access constant thinkthen_request_text;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_recognition_example_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            text : access constant thinkthen_request_text;
         when others =>
            example_text : access constant thinkthen_request_recognition_example_text;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_request_recognition_example is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_request_recognition_example_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_recognition_example_list is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_option_schema is record
      m_description : access constant thinkthen_request_json_value;
      m_name : access constant thinkthen_request_text;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_option_schema_list is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_original_text is record
      m_text : access constant thinkthen_request_text;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_original_json_value is record
      value : access constant thinkthen_complete_json;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_original_json is record
      m_value : access constant thinkthen_request_original_json_value;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_original_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            text : access constant thinkthen_request_original_text;
         when others =>
            json : access constant thinkthen_request_original_json;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_request_original is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_request_original_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_recognition_seed_span is record
      m_end : access constant thinkthen_request_unsigned;
      m_kind : access constant thinkthen_request_text;
      m_start : access constant thinkthen_request_unsigned;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_recognition_seed_span_list is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_item is record
      m_context : access constant thinkthen_request_context_schema;
      m_examples : access constant thinkthen_request_recognition_example_list;
      m_images : access constant thinkthen_request_image_list;
      m_options : access constant thinkthen_request_option_schema_list;
      m_original : access constant thinkthen_request_original;
      m_seed_spans : access constant thinkthen_request_recognition_seed_span_list;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_item_list is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_input_records is record
      m_items : access constant thinkthen_request_item_list;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_input_units is record
      m_items : access constant thinkthen_request_item_list;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_input_entities is record
      m_items : access constant thinkthen_request_item_list;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_literal_document is record
      reserved : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_literal_lines is record
      reserved : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_literal_jsonl is record
      reserved : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_literal_csv is record
      reserved : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_literal_tsv is record
      reserved : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_framing_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            literal_document : access constant thinkthen_request_literal_document;
         when 1 =>
            literal_lines : access constant thinkthen_request_literal_lines;
         when 2 =>
            literal_jsonl : access constant thinkthen_request_literal_jsonl;
         when 3 =>
            literal_csv : access constant thinkthen_request_literal_csv;
         when others =>
            literal_tsv : access constant thinkthen_request_literal_tsv;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_request_framing is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_request_framing_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_literal_text is record
      reserved : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_literal_image is record
      reserved : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_reader_media_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            literal_text : access constant thinkthen_request_literal_text;
         when others =>
            literal_image : access constant thinkthen_request_literal_image;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_request_reader_media is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_request_reader_media_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_literal_line is record
      reserved : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_literal_window is record
      reserved : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_literal_file is record
      reserved : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_source_unit_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            literal_line : access constant thinkthen_request_literal_line;
         when 1 =>
            literal_window : access constant thinkthen_request_literal_window;
         when others =>
            literal_file : access constant thinkthen_request_literal_file;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_request_source_unit is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_request_source_unit_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_reader is record
      m_unit : access constant thinkthen_request_source_unit;
      m_window : access constant thinkthen_request_unsigned;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_source is record
      m_framing : access constant thinkthen_request_framing;
      m_media : access constant thinkthen_request_reader_media;
      m_paths : access constant thinkthen_request_text_list;
      m_reading : access constant thinkthen_request_reader;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_input_source is record
      m_source : access constant thinkthen_request_source;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_input_feed is record
      m_framing : access constant thinkthen_request_framing;
      m_images : access constant thinkthen_request_image_list;
      m_name : access constant thinkthen_request_text;
      m_reading : access constant thinkthen_request_reader;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_input_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            text : access constant thinkthen_request_input_text;
         when 1 =>
            json : access constant thinkthen_request_input_json;
         when 2 =>
            records : access constant thinkthen_request_input_records;
         when 3 =>
            units : access constant thinkthen_request_input_units;
         when 4 =>
            entities : access constant thinkthen_request_input_entities;
         when 5 =>
            source : access constant thinkthen_request_input_source;
         when others =>
            feed : access constant thinkthen_request_input_feed;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_request_input is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_request_input_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_integer is record
      value : aliased Interfaces.Integer_64;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_option_schema_2_description is record
      value : access constant thinkthen_complete_json;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_option_schema_2 is record
      m_description : access constant thinkthen_request_option_schema_2_description;
      m_name : access constant thinkthen_request_text;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_option_schema_2_list is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_literal_whole is record
      reserved : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_literal_boundary_only is record
      reserved : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_recognition_mode_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            literal_whole : access constant thinkthen_request_literal_whole;
         when others =>
            literal_boundary_only : access constant thinkthen_request_literal_boundary_only;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_request_recognition_mode is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_request_recognition_mode_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_call_options_no is record
      value : access constant thinkthen_complete_json;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_number is record
      value : aliased double;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_call_threshold_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            number : access constant thinkthen_request_number;
         when others =>
            text : access constant thinkthen_request_text;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_request_call_threshold is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_request_call_threshold_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_recognition_stage_context is record
      m_boundary : access constant thinkthen_request_text;
      m_kind_edge : access constant thinkthen_request_text;
      m_relation : access constant thinkthen_request_text;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_call_options_yes is record
      value : access constant thinkthen_complete_json;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_call_options is record
      m_attempts : access constant thinkthen_request_boolean;
      m_batch : access constant thinkthen_request_batch;
      m_context : access constant thinkthen_request_text;
      m_context_field : access constant thinkthen_request_text;
      m_deadline_ms : access constant thinkthen_request_integer;
      m_entity_definition : access constant thinkthen_request_text;
      m_examples : access constant thinkthen_request_recognition_example_list;
      m_examples_field : access constant thinkthen_request_text;
      m_field : access constant thinkthen_request_text_list;
      m_files_only : access constant thinkthen_request_boolean;
      m_instructions : access constant thinkthen_request_text;
      m_labels : access constant thinkthen_request_option_schema_2_list;
      m_levels : access constant thinkthen_request_option_schema_2_list;
      m_max_requests_total : access constant thinkthen_request_unsigned;
      m_max_text_bytes : access constant thinkthen_request_unsigned;
      m_mode : access constant thinkthen_request_recognition_mode;
      m_model : access constant thinkthen_request_text;
      m_no : access constant thinkthen_request_call_options_no;
      m_none : access constant thinkthen_request_boolean;
      m_options_field : access constant thinkthen_request_text;
      m_relation_threshold : access constant thinkthen_request_call_threshold;
      m_seed_spans : access constant thinkthen_request_recognition_seed_span_list;
      m_seed_spans_field : access constant thinkthen_request_text;
      m_snippet_pieces : access constant thinkthen_request_unsigned;
      m_stage_context : access constant thinkthen_request_recognition_stage_context;
      m_threshold : access constant thinkthen_request_call_threshold;
      m_top : access constant thinkthen_request_unsigned;
      m_yes : access constant thinkthen_request_call_options_yes;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_question_selector_text is record
      m_text : access constant thinkthen_request_text;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_literal_max is record
      reserved : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_question_batch_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            literal_max : access constant thinkthen_request_literal_max;
         when others =>
            integer : access constant thinkthen_request_integer;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_request_question_batch is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_request_question_batch_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_input_declaration_string is record
      reserved : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_input_property_string is record
      reserved : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_input_property_number is record
      reserved : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_input_property_boolean is record
      reserved : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_input_property_array_items is record
      reserved : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_input_property_array is record
      m_items : access constant thinkthen_request_input_property_array_items;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_input_property_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            string : access constant thinkthen_request_input_property_string;
         when 1 =>
            number : access constant thinkthen_request_input_property_number;
         when 2 =>
            boolean : access constant thinkthen_request_input_property_boolean;
         when others =>
            c_array : access constant thinkthen_request_input_property_array;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_request_input_property is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_request_input_property_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_input_property_map_entry is record
      key : aliased thinkthen_complete_utf8;
      value : access constant thinkthen_request_input_property;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_input_property_map is record
      data : access constant thinkthen_request_input_property_map_entry;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_input_declaration_object is record
      m_properties : access constant thinkthen_request_input_property_map;
      m_required : access constant thinkthen_request_text_list;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_input_declaration_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            string : access constant thinkthen_request_input_declaration_string;
         when others =>
            object : access constant thinkthen_request_input_declaration_object;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_request_input_declaration is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_request_input_declaration_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_json_value_list is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_question_text_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            text : access constant thinkthen_request_text;
         when 1 =>
            json_value_map : access constant thinkthen_request_json_value_map;
         when others =>
            json_value_list : access constant thinkthen_request_json_value_list;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_request_question_text is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_request_question_text_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_criterion_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            text : access constant thinkthen_request_text;
         when 1 =>
            json_value_map : access constant thinkthen_request_json_value_map;
         when 2 =>
            json_value_list : access constant thinkthen_request_json_value_list;
         when others =>
            c_null : access constant thinkthen_request_null;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_request_criterion is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_request_criterion_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_name is record
      text : aliased thinkthen_complete_utf8;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_pointers_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            text : access constant thinkthen_request_text;
         when others =>
            text_list : access constant thinkthen_request_text_list;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_request_pointers is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_request_pointers_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_profile is record
      text : aliased thinkthen_complete_utf8;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_threshold_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            number : access constant thinkthen_request_number;
         when others =>
            text : access constant thinkthen_request_text;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_request_threshold is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_request_threshold_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_definition_authored_decide is record
      m_batch : access constant thinkthen_request_question_batch;
      m_context_schema : access constant thinkthen_request_input_declaration;
      m_decide : access constant thinkthen_request_question_text;
      m_false : access constant thinkthen_request_criterion;
      m_item_schema : access constant thinkthen_request_input_declaration;
      m_model : access constant thinkthen_request_name;
      m_name : access constant thinkthen_request_text;
      m_on : access constant thinkthen_request_pointers;
      m_profile : access constant thinkthen_request_profile;
      m_threshold : access constant thinkthen_request_threshold;
      m_true : access constant thinkthen_request_criterion;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_name_list is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_description_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            text : access constant thinkthen_request_text;
         when 1 =>
            json_value_map : access constant thinkthen_request_json_value_map;
         when 2 =>
            json_value_list : access constant thinkthen_request_json_value_list;
         when others =>
            c_null : access constant thinkthen_request_null;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_request_description is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_request_description_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_description_map_entry is record
      key : aliased thinkthen_complete_utf8;
      value : access constant thinkthen_request_description;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_description_map is record
      data : access constant thinkthen_request_description_map_entry;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_options_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            name_list : access constant thinkthen_request_name_list;
         when others =>
            description_map : access constant thinkthen_request_description_map;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_request_options is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_request_options_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_cut_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            number : access constant thinkthen_request_number;
         when others =>
            text : access constant thinkthen_request_text;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_request_cut is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_request_cut_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_definition_authored_choose is record
      m_batch : access constant thinkthen_request_question_batch;
      m_choose : access constant thinkthen_request_question_text;
      m_context_schema : access constant thinkthen_request_input_declaration;
      m_item_schema : access constant thinkthen_request_input_declaration;
      m_model : access constant thinkthen_request_name;
      m_name : access constant thinkthen_request_text;
      m_on : access constant thinkthen_request_pointers;
      m_options : access constant thinkthen_request_options;
      m_profile : access constant thinkthen_request_profile;
      m_threshold : access constant thinkthen_request_cut;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_labels_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            name_list : access constant thinkthen_request_name_list;
         when others =>
            description_map : access constant thinkthen_request_description_map;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_request_labels is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_request_labels_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_definition_authored_tag is record
      m_batch : access constant thinkthen_request_question_batch;
      m_context_schema : access constant thinkthen_request_input_declaration;
      m_item_schema : access constant thinkthen_request_input_declaration;
      m_labels : access constant thinkthen_request_labels;
      m_model : access constant thinkthen_request_name;
      m_name : access constant thinkthen_request_text;
      m_on : access constant thinkthen_request_pointers;
      m_profile : access constant thinkthen_request_profile;
      m_tag : access constant thinkthen_request_question_text;
      m_threshold : access constant thinkthen_request_cut;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_criterion_map_entry is record
      key : aliased thinkthen_complete_utf8;
      value : access constant thinkthen_request_criterion;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_criterion_map is record
      data : access constant thinkthen_request_criterion_map_entry;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_levels_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            name_list : access constant thinkthen_request_name_list;
         when others =>
            criterion_map : access constant thinkthen_request_criterion_map;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_request_levels is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_request_levels_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_definition_authored_score is record
      m_batch : access constant thinkthen_request_question_batch;
      m_context_schema : access constant thinkthen_request_input_declaration;
      m_item_schema : access constant thinkthen_request_input_declaration;
      m_levels : access constant thinkthen_request_levels;
      m_model : access constant thinkthen_request_name;
      m_name : access constant thinkthen_request_text;
      m_on : access constant thinkthen_request_pointers;
      m_profile : access constant thinkthen_request_profile;
      m_score : access constant thinkthen_request_question_text;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_relate_fields is record
      m_kind : access constant thinkthen_request_text;
      m_name : access constant thinkthen_request_text;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_relation is record
      m_either : access constant thinkthen_request_boolean;
      m_name : access constant thinkthen_request_name;
      m_reads : access constant thinkthen_request_name;
      m_single : access constant thinkthen_request_boolean;
      m_source : access constant thinkthen_request_name;
      m_target : access constant thinkthen_request_name;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_relation_list is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_relate_question is record
      m_fields : access constant thinkthen_request_relate_fields;
      m_relations : access constant thinkthen_request_relation_list;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_definition_authored_relate is record
      m_context_schema : access constant thinkthen_request_input_declaration;
      m_item_schema : access constant thinkthen_request_input_declaration;
      m_model : access constant thinkthen_request_name;
      m_name : access constant thinkthen_request_text;
      m_profile : access constant thinkthen_request_profile;
      m_relate : access constant thinkthen_request_relate_question;
      m_threshold : access constant thinkthen_request_cut;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_definition_authored_find is record
      m_context_schema : access constant thinkthen_request_input_declaration;
      m_find : access constant thinkthen_request_question_text;
      m_item_schema : access constant thinkthen_request_input_declaration;
      m_model : access constant thinkthen_request_name;
      m_name : access constant thinkthen_request_text;
      m_on : access constant thinkthen_request_pointers;
      m_profile : access constant thinkthen_request_profile;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_recognize_question is record
      m_entity_definition : access constant thinkthen_request_question_text;
      m_instructions : access constant thinkthen_request_question_text;
      m_kinds : access constant thinkthen_request_description_map;
      m_mode : access constant thinkthen_request_recognition_mode;
      m_relations : access constant thinkthen_request_relation_list;
      m_snippet_pieces : access constant thinkthen_request_unsigned;
      m_stage_context : access constant thinkthen_request_recognition_stage_context;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_definition_recognize_definition is record
      m_context_schema : access constant thinkthen_request_input_declaration;
      m_item_schema : access constant thinkthen_request_input_declaration;
      m_model : access constant thinkthen_request_name;
      m_name : access constant thinkthen_request_text;
      m_on : access constant thinkthen_request_pointers;
      m_profile : access constant thinkthen_request_profile;
      m_recognize : access constant thinkthen_request_recognize_question;
      m_relation_threshold : access constant thinkthen_request_cut;
      m_threshold : access constant thinkthen_request_cut;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_annotate_question_decide is record
      m_context_schema : access constant thinkthen_request_input_declaration;
      m_decide : access constant thinkthen_request_question_text;
      m_false : access constant thinkthen_request_criterion;
      m_item_schema : access constant thinkthen_request_input_declaration;
      m_name : access constant thinkthen_request_text;
      m_on : access constant thinkthen_request_pointers;
      m_threshold : access constant thinkthen_request_threshold;
      m_true : access constant thinkthen_request_criterion;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_annotate_question_choose is record
      m_choose : access constant thinkthen_request_question_text;
      m_context_schema : access constant thinkthen_request_input_declaration;
      m_item_schema : access constant thinkthen_request_input_declaration;
      m_name : access constant thinkthen_request_text;
      m_on : access constant thinkthen_request_pointers;
      m_options : access constant thinkthen_request_options;
      m_threshold : access constant thinkthen_request_cut;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_annotate_question_tag is record
      m_context_schema : access constant thinkthen_request_input_declaration;
      m_item_schema : access constant thinkthen_request_input_declaration;
      m_labels : access constant thinkthen_request_labels;
      m_name : access constant thinkthen_request_text;
      m_on : access constant thinkthen_request_pointers;
      m_tag : access constant thinkthen_request_question_text;
      m_threshold : access constant thinkthen_request_cut;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_annotate_question_score is record
      m_context_schema : access constant thinkthen_request_input_declaration;
      m_item_schema : access constant thinkthen_request_input_declaration;
      m_levels : access constant thinkthen_request_levels;
      m_name : access constant thinkthen_request_text;
      m_on : access constant thinkthen_request_pointers;
      m_score : access constant thinkthen_request_question_text;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_annotate_question_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            decide : access constant thinkthen_request_annotate_question_decide;
         when 1 =>
            choose : access constant thinkthen_request_annotate_question_choose;
         when 2 =>
            tag : access constant thinkthen_request_annotate_question_tag;
         when others =>
            score : access constant thinkthen_request_annotate_question_score;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_request_annotate_question is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_request_annotate_question_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_annotate_question_map_entry is record
      key : aliased thinkthen_complete_utf8;
      value : access constant thinkthen_request_annotate_question;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_annotate_question_map is record
      data : access constant thinkthen_request_annotate_question_map_entry;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_definition_annotate_definition is record
      m_batch : access constant thinkthen_request_json_value;
      m_profile : access constant thinkthen_request_profile;
      m_questions : access constant thinkthen_request_annotate_question_map;
      m_threshold : access constant thinkthen_request_threshold;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_definition_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            authored_decide : access constant thinkthen_request_definition_authored_decide;
         when 1 =>
            authored_choose : access constant thinkthen_request_definition_authored_choose;
         when 2 =>
            authored_tag : access constant thinkthen_request_definition_authored_tag;
         when 3 =>
            authored_score : access constant thinkthen_request_definition_authored_score;
         when 4 =>
            authored_relate : access constant thinkthen_request_definition_authored_relate;
         when 5 =>
            authored_find : access constant thinkthen_request_definition_authored_find;
         when 6 =>
            recognize_definition : access constant thinkthen_request_definition_recognize_definition;
         when others =>
            annotate_definition : access constant thinkthen_request_definition_annotate_definition;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_request_definition is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_request_definition_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_question_selector_definition is record
      m_value : access constant thinkthen_request_definition;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_question_selector_file is record
      m_path : access constant thinkthen_request_text;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_question_selector_name is record
      m_name : access constant thinkthen_request_text;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_question_selector_reference is record
      m_reference : access constant thinkthen_request_text;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_question_selector_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            text : access constant thinkthen_request_question_selector_text;
         when 1 =>
            definition : access constant thinkthen_request_question_selector_definition;
         when 2 =>
            file : access constant thinkthen_request_question_selector_file;
         when 3 =>
            name : access constant thinkthen_request_question_selector_name;
         when others =>
            reference : access constant thinkthen_request_question_selector_reference;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_request_question_selector is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_request_question_selector_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_arguments is record
      m_input : access constant thinkthen_request_input;
      m_options : access constant thinkthen_request_call_options;
      m_question : access constant thinkthen_request_question_selector;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_reader_failure_producer is record
      reserved : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_session_source_location is record
      m_file : access constant thinkthen_request_text;
      m_first_line : access constant thinkthen_request_unsigned;
      m_last_line : access constant thinkthen_request_unsigned;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_reader_failure_io is record
      m_location : access constant thinkthen_request_session_source_location;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_reader_failure_utf_8 is record
      m_location : access constant thinkthen_request_session_source_location;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_reader_failure_invalid_input is record
      m_location : access constant thinkthen_request_session_source_location;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_reader_failure_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            producer : access constant thinkthen_request_reader_failure_producer;
         when 1 =>
            io : access constant thinkthen_request_reader_failure_io;
         when 2 =>
            utf_8 : access constant thinkthen_request_reader_failure_utf_8;
         when others =>
            invalid_input : access constant thinkthen_request_reader_failure_invalid_input;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_request_reader_failure is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_request_reader_failure_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_request_session_descriptor is record
      m_item : access constant thinkthen_request_item;
      m_location : access constant thinkthen_request_session_source_location;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answer_yes_no is record
      kind : aliased thinkthen_complete_utf8;
      probability : aliased double;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_f64_presence is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased double;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_number_map_entry is record
      name : aliased thinkthen_complete_utf8;
      value : aliased double;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_number_map is record
      data : access constant thinkthen_complete_number_map_entry;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answer_choice is record
      confidence : aliased thinkthen_complete_f64_presence;
      kind : aliased thinkthen_complete_utf8;
      pick : aliased thinkthen_complete_utf8;
      probabilities : aliased thinkthen_complete_number_map;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answer_tag is record
      kind : aliased thinkthen_complete_utf8;
      probabilities : aliased thinkthen_complete_number_map;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answer_score is record
      confidence : aliased thinkthen_complete_f64_presence;
      kind : aliased thinkthen_complete_utf8;
      level : aliased thinkthen_complete_utf8;
      probabilities : aliased thinkthen_complete_number_map;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answer_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            yes_no : access constant thinkthen_complete_answer_yes_no;
         when 1 =>
            choice : access constant thinkthen_complete_answer_choice;
         when 2 =>
            tag : access constant thinkthen_complete_answer_tag;
         when others =>
            score : access constant thinkthen_complete_answer_score;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_answer is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_answer_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answer_id is record
      value : aliased thinkthen_complete_utf8;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_image_media is record
      kind : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_image is record
      base64 : aliased thinkthen_complete_utf8;
      height : aliased Interfaces.Unsigned_64;
      media : access constant thinkthen_complete_image_media;
      width : aliased Interfaces.Unsigned_64;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_image_list is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_image_list_presence is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_image_list;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_json_presence is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_json;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_batch_setting_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            integer : aliased Interfaces.Unsigned_64;
         when others =>
            string : aliased thinkthen_complete_utf8;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_batch_setting is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_batch_setting_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_batch_setting_presence is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_batch_setting;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_batch_warning is record
      running : access constant thinkthen_complete_batch_setting;
      tuned_for : access constant thinkthen_complete_batch_setting;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_batch_warning_presence is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_batch_warning;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_observation_id is record
      value : aliased thinkthen_complete_utf8;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_observation_observation_id is record
      observation_id : access constant thinkthen_complete_observation_id;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_failure_id is record
      value : aliased thinkthen_complete_utf8;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_observation_failure_id is record
      failure_id : access constant thinkthen_complete_failure_id;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_observation_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            observation_id : access constant thinkthen_complete_observation_observation_id;
         when others =>
            failure_id : access constant thinkthen_complete_observation_failure_id;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_observation is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_observation_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_observation_list is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_origin is record
      kind : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_origin_presence is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_origin;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_profile_warning is record
      running : aliased thinkthen_complete_utf8;
      tuned_for : aliased thinkthen_complete_utf8;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_profile_warning_presence is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_profile_warning;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_question_source is record
      answered_by : aliased thinkthen_complete_utf8;
      batch_size : aliased thinkthen_complete_u64_presence;
      origin : access constant thinkthen_complete_origin;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_question_source_list is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_string_list is record
      data : access constant thinkthen_complete_utf8;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_usage is record
      input_tokens : aliased thinkthen_complete_u64_presence;
      output_tokens : aliased thinkthen_complete_u64_presence;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_usage_presence is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_usage;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_meta is record
      answered_by : aliased thinkthen_complete_utf8_presence;
      attempts : aliased thinkthen_complete_attempt_list_presence;
      batch_setting : aliased thinkthen_complete_batch_setting_presence;
      batch_warning : aliased thinkthen_complete_batch_warning_presence;
      cached : aliased Interfaces.Unsigned_32;
      context_sha256 : aliased thinkthen_complete_utf8_presence;
      failed_questions : aliased Interfaces.Unsigned_64;
      model : aliased thinkthen_complete_utf8;
      observations : aliased thinkthen_complete_observation_list;
      origin : aliased thinkthen_complete_origin_presence;
      profile_warning : aliased thinkthen_complete_profile_warning_presence;
      question_sha256 : aliased thinkthen_complete_utf8_presence;
      question_sources : aliased thinkthen_complete_question_source_list;
      questions_sha256 : aliased thinkthen_complete_utf8_presence;
      requests : aliased thinkthen_complete_string_list;
      requests_sent : aliased Interfaces.Unsigned_64;
      tool : aliased thinkthen_complete_utf8;
      url : aliased thinkthen_complete_utf8;
      usage : aliased thinkthen_complete_usage_presence;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_batch_string is record
      kind : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_batch_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            integer : aliased Interfaces.Unsigned_64;
         when others =>
            string : access constant thinkthen_complete_batch_string;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_batch is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_batch_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_batch_presence is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_batch;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_string_type is record
      kind : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_input_declaration_string_root is record
      c_type : access constant thinkthen_complete_string_type;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_input_property_type_string is record
      c_type : aliased thinkthen_complete_utf8;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_input_property_type_number is record
      c_type : aliased thinkthen_complete_utf8;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_input_property_type_boolean is record
      c_type : aliased thinkthen_complete_utf8;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_string_root is record
      c_type : access constant thinkthen_complete_string_type;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_input_property_type_array is record
      items : access constant thinkthen_complete_string_root;
      c_type : aliased thinkthen_complete_utf8;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_input_property_type_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            string : access constant thinkthen_complete_input_property_type_string;
         when 1 =>
            number : access constant thinkthen_complete_input_property_type_number;
         when 2 =>
            boolean : access constant thinkthen_complete_input_property_type_boolean;
         when others =>
            c_array : access constant thinkthen_complete_input_property_type_array;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_input_property_type is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_input_property_type_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_input_property_type_map_entry is record
      name : aliased thinkthen_complete_utf8;
      value : access constant thinkthen_complete_input_property_type;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_input_property_type_map is record
      data : access constant thinkthen_complete_input_property_type_map_entry;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_string_list_presence is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_string_list;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_object_type is record
      kind : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_input_declaration_object_root is record
      properties : aliased thinkthen_complete_input_property_type_map;
      required : aliased thinkthen_complete_string_list_presence;
      c_type : access constant thinkthen_complete_object_type;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_input_declaration_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            string_root : access constant thinkthen_complete_input_declaration_string_root;
         when others =>
            object_root : access constant thinkthen_complete_input_declaration_object_root;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_input_declaration is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_input_declaration_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_input_declaration_presence is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_input_declaration;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_label is record
      description : aliased thinkthen_complete_json_presence;
      name : aliased thinkthen_complete_utf8;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_label_list is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_label_list_presence is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_label_list;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_question_name is record
      value : aliased thinkthen_complete_utf8;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_question_name_presence is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_question_name;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_question_document_decide is record
      batch : aliased thinkthen_complete_batch_presence;
      context_schema : aliased thinkthen_complete_input_declaration_presence;
      item_schema : aliased thinkthen_complete_input_declaration_presence;
      label_details : aliased thinkthen_complete_label_list_presence;
      model : aliased thinkthen_complete_utf8_presence;
      name : aliased thinkthen_complete_question_name_presence;
      on : aliased thinkthen_complete_string_list_presence;
      profile : aliased thinkthen_complete_utf8_presence;
      false_u : aliased thinkthen_complete_json_presence;
      text : aliased thinkthen_complete_json_presence;
      true_u : aliased thinkthen_complete_json_presence;
      verb : aliased thinkthen_complete_utf8;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_question_document_choose is record
      batch : aliased thinkthen_complete_batch_presence;
      context_schema : aliased thinkthen_complete_input_declaration_presence;
      item_schema : aliased thinkthen_complete_input_declaration_presence;
      label_details : aliased thinkthen_complete_label_list_presence;
      model : aliased thinkthen_complete_utf8_presence;
      name : aliased thinkthen_complete_question_name_presence;
      on : aliased thinkthen_complete_string_list_presence;
      profile : aliased thinkthen_complete_utf8_presence;
      options : aliased thinkthen_complete_string_list;
      text : aliased thinkthen_complete_json_presence;
      verb : aliased thinkthen_complete_utf8;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_question_document_tag is record
      batch : aliased thinkthen_complete_batch_presence;
      context_schema : aliased thinkthen_complete_input_declaration_presence;
      item_schema : aliased thinkthen_complete_input_declaration_presence;
      label_details : aliased thinkthen_complete_label_list_presence;
      model : aliased thinkthen_complete_utf8_presence;
      name : aliased thinkthen_complete_question_name_presence;
      on : aliased thinkthen_complete_string_list_presence;
      profile : aliased thinkthen_complete_utf8_presence;
      labels : aliased thinkthen_complete_string_list;
      text : aliased thinkthen_complete_json_presence;
      verb : aliased thinkthen_complete_utf8;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_question_document_score is record
      batch : aliased thinkthen_complete_batch_presence;
      context_schema : aliased thinkthen_complete_input_declaration_presence;
      item_schema : aliased thinkthen_complete_input_declaration_presence;
      label_details : aliased thinkthen_complete_label_list_presence;
      model : aliased thinkthen_complete_utf8_presence;
      name : aliased thinkthen_complete_question_name_presence;
      on : aliased thinkthen_complete_string_list_presence;
      profile : aliased thinkthen_complete_utf8_presence;
      levels : aliased thinkthen_complete_string_list;
      text : aliased thinkthen_complete_json_presence;
      verb : aliased thinkthen_complete_utf8;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_question_document_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            decide : access constant thinkthen_complete_question_document_decide;
         when 1 =>
            choose : access constant thinkthen_complete_question_document_choose;
         when 2 =>
            tag : access constant thinkthen_complete_question_document_tag;
         when others =>
            score : access constant thinkthen_complete_question_document_score;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_question_document is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_question_document_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_result_tag is record
      kind : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_physical_source is record
      file : aliased thinkthen_complete_utf8;
      first_line : aliased thinkthen_complete_u64_presence;
      last_line : aliased thinkthen_complete_u64_presence;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_physical_source_presence is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_physical_source;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_rank_member_result is record
      answer : access constant thinkthen_complete_answer;
      answer_id : access constant thinkthen_complete_answer_id;
      images : aliased thinkthen_complete_image_list_presence;
      meta : access constant thinkthen_complete_meta;
      question : access constant thinkthen_complete_question_document;
      schema : access constant thinkthen_complete_result_tag;
      source : aliased thinkthen_complete_physical_source_presence;
      threshold : aliased thinkthen_complete_json_presence;
      value : aliased Interfaces.Unsigned_64;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_rank_member is record
      name : aliased thinkthen_complete_utf8;
      result : access constant thinkthen_complete_rank_member_result;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_rank_member_list is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_rank_member_list_presence is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_rank_member_list;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_threshold_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            number : aliased double;
         when others =>
            string : aliased thinkthen_complete_utf8;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_threshold is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_threshold_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_threshold_presence is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_threshold;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_decide_value is record
      value : access constant thinkthen_complete_json;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_decide_value_presence is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_decide_value;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_atomic_decide_value is record
      answer : access constant thinkthen_complete_answer;
      answer_id : access constant thinkthen_complete_answer_id;
      images : aliased thinkthen_complete_image_list_presence;
      index : aliased thinkthen_complete_u64_presence;
      input : aliased thinkthen_complete_json_presence;
      members : aliased thinkthen_complete_rank_member_list_presence;
      meta : access constant thinkthen_complete_meta;
      question : access constant thinkthen_complete_question_document;
      question_name : aliased thinkthen_complete_utf8_presence;
      schema : access constant thinkthen_complete_result_tag;
      source : aliased thinkthen_complete_physical_source_presence;
      threshold : aliased thinkthen_complete_threshold_presence;
      value : aliased thinkthen_complete_decide_value_presence;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_packet_decide_row is record
      c_function : aliased thinkthen_complete_utf8;
      kind : aliased thinkthen_complete_utf8;
      value : access constant thinkthen_complete_atomic_decide_value;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_atomic_nullable_string is record
      answer : access constant thinkthen_complete_answer;
      answer_id : access constant thinkthen_complete_answer_id;
      images : aliased thinkthen_complete_image_list_presence;
      index : aliased thinkthen_complete_u64_presence;
      input : aliased thinkthen_complete_json_presence;
      members : aliased thinkthen_complete_rank_member_list_presence;
      meta : access constant thinkthen_complete_meta;
      question : access constant thinkthen_complete_question_document;
      question_name : aliased thinkthen_complete_utf8_presence;
      schema : access constant thinkthen_complete_result_tag;
      source : aliased thinkthen_complete_physical_source_presence;
      threshold : aliased thinkthen_complete_threshold_presence;
      value : aliased thinkthen_complete_utf8_presence;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_packet_choose_row is record
      c_function : aliased thinkthen_complete_utf8;
      kind : aliased thinkthen_complete_utf8;
      value : access constant thinkthen_complete_atomic_nullable_string;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_atomic_array_of_string is record
      answer : access constant thinkthen_complete_answer;
      answer_id : access constant thinkthen_complete_answer_id;
      images : aliased thinkthen_complete_image_list_presence;
      index : aliased thinkthen_complete_u64_presence;
      input : aliased thinkthen_complete_json_presence;
      members : aliased thinkthen_complete_rank_member_list_presence;
      meta : access constant thinkthen_complete_meta;
      question : access constant thinkthen_complete_question_document;
      question_name : aliased thinkthen_complete_utf8_presence;
      schema : access constant thinkthen_complete_result_tag;
      source : aliased thinkthen_complete_physical_source_presence;
      threshold : aliased thinkthen_complete_threshold_presence;
      value : aliased thinkthen_complete_string_list;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_packet_tag_row is record
      c_function : aliased thinkthen_complete_utf8;
      kind : aliased thinkthen_complete_utf8;
      value : access constant thinkthen_complete_atomic_array_of_string;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_atomic_double is record
      answer : access constant thinkthen_complete_answer;
      answer_id : access constant thinkthen_complete_answer_id;
      images : aliased thinkthen_complete_image_list_presence;
      index : aliased thinkthen_complete_u64_presence;
      input : aliased thinkthen_complete_json_presence;
      members : aliased thinkthen_complete_rank_member_list_presence;
      meta : access constant thinkthen_complete_meta;
      question : access constant thinkthen_complete_question_document;
      question_name : aliased thinkthen_complete_utf8_presence;
      schema : access constant thinkthen_complete_result_tag;
      source : aliased thinkthen_complete_physical_source_presence;
      threshold : aliased thinkthen_complete_threshold_presence;
      value : aliased double;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_packet_score_row is record
      c_function : aliased thinkthen_complete_utf8;
      kind : aliased thinkthen_complete_utf8;
      value : access constant thinkthen_complete_atomic_double;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_atomic_boolean is record
      answer : access constant thinkthen_complete_answer;
      answer_id : access constant thinkthen_complete_answer_id;
      images : aliased thinkthen_complete_image_list_presence;
      index : aliased thinkthen_complete_u64_presence;
      input : aliased thinkthen_complete_json_presence;
      members : aliased thinkthen_complete_rank_member_list_presence;
      meta : access constant thinkthen_complete_meta;
      question : access constant thinkthen_complete_question_document;
      question_name : aliased thinkthen_complete_utf8_presence;
      schema : access constant thinkthen_complete_result_tag;
      source : aliased thinkthen_complete_physical_source_presence;
      threshold : aliased thinkthen_complete_threshold_presence;
      value : aliased Interfaces.Unsigned_32;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_packet_filter_row is record
      c_function : aliased thinkthen_complete_utf8;
      kind : aliased thinkthen_complete_utf8;
      value : access constant thinkthen_complete_atomic_boolean;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_value_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            boolean : aliased Interfaces.Unsigned_32;
         when 1 =>
            c_null : aliased Interfaces.Unsigned_32;
         when 2 =>
            string : aliased thinkthen_complete_utf8;
         when 3 =>
            c_array : aliased thinkthen_complete_string_list;
         when others =>
            number : aliased double;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_value is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_value_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_value_presence is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_value;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_annotation_member_answer_id is record
      answer : access constant thinkthen_complete_answer;
      answer_id : access constant thinkthen_complete_answer_id;
      observations : aliased thinkthen_complete_observation_list;
      question : access constant thinkthen_complete_question_document;
      question_sources : aliased thinkthen_complete_question_source_list;
      request : aliased thinkthen_complete_utf8;
      threshold : aliased thinkthen_complete_threshold_presence;
      usage : aliased thinkthen_complete_usage_presence;
      value : aliased thinkthen_complete_value_presence;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_failure_cause is record
      kind : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_failure_field_kind is record
      kind : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_failure is record
      cause : access constant thinkthen_complete_failure_cause;
      kind : access constant thinkthen_complete_failure_field_kind;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_annotation_member_failure_id is record
      failure : access constant thinkthen_complete_failure;
      failure_id : access constant thinkthen_complete_failure_id;
      observations : aliased thinkthen_complete_observation_list;
      question : access constant thinkthen_complete_question_document;
      question_sources : aliased thinkthen_complete_question_source_list;
      request : aliased thinkthen_complete_utf8;
      threshold : aliased thinkthen_complete_threshold_presence;
      usage : aliased thinkthen_complete_usage_presence;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_annotation_member_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            answer_id : access constant thinkthen_complete_annotation_member_answer_id;
         when others =>
            failure_id : access constant thinkthen_complete_annotation_member_failure_id;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_annotation_member is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_annotation_member_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_annotation_member_map_entry is record
      name : aliased thinkthen_complete_utf8;
      value : access constant thinkthen_complete_annotation_member;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_annotation_member_map is record
      data : access constant thinkthen_complete_annotation_member_map_entry;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_position is record
      file : aliased thinkthen_complete_utf8_presence;
      first : aliased thinkthen_complete_u64_presence;
      images : aliased thinkthen_complete_string_list_presence;
      last : aliased thinkthen_complete_u64_presence;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_position_presence is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_position;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_failed is record
      failed : access constant thinkthen_complete_failure;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_annotated_field_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            boolean : aliased Interfaces.Unsigned_32;
         when 1 =>
            c_null : aliased Interfaces.Unsigned_32;
         when 2 =>
            string : aliased thinkthen_complete_utf8;
         when 3 =>
            c_array : aliased thinkthen_complete_string_list;
         when 4 =>
            number : aliased double;
         when others =>
            object : access constant thinkthen_complete_failed;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_annotated_field is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_annotated_field_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_annotated_field_map_entry is record
      name : aliased thinkthen_complete_utf8;
      value : access constant thinkthen_complete_annotated_field;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_annotated_field_map is record
      data : access constant thinkthen_complete_annotated_field_map_entry;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_annotated_row is record
      value : aliased thinkthen_complete_annotated_field_map;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_annotation is record
      answer_id : access constant thinkthen_complete_answer_id;
      answers : aliased thinkthen_complete_annotation_member_map;
      file : aliased thinkthen_complete_utf8_presence;
      first_line : aliased thinkthen_complete_u64_presence;
      index : aliased thinkthen_complete_u64_presence;
      input : aliased thinkthen_complete_json_presence;
      last_line : aliased thinkthen_complete_u64_presence;
      meta : access constant thinkthen_complete_meta;
      position : aliased thinkthen_complete_position_presence;
      schema : access constant thinkthen_complete_result_tag;
      source : aliased thinkthen_complete_physical_source_presence;
      value : access constant thinkthen_complete_annotated_row;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_packet_annotate_row is record
      c_function : aliased thinkthen_complete_utf8;
      kind : aliased thinkthen_complete_utf8;
      value : access constant thinkthen_complete_annotation;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_atomic_decide_value_list is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_packet_decide_aggregate is record
      c_function : aliased thinkthen_complete_utf8;
      kind : aliased thinkthen_complete_utf8;
      value : aliased thinkthen_complete_atomic_decide_value_list;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_atomic_nullable_string_list is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_packet_choose_aggregate is record
      c_function : aliased thinkthen_complete_utf8;
      kind : aliased thinkthen_complete_utf8;
      value : aliased thinkthen_complete_atomic_nullable_string_list;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_atomic_array_of_string_list is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_packet_tag_aggregate is record
      c_function : aliased thinkthen_complete_utf8;
      kind : aliased thinkthen_complete_utf8;
      value : aliased thinkthen_complete_atomic_array_of_string_list;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_atomic_double_list is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_packet_score_aggregate is record
      c_function : aliased thinkthen_complete_utf8;
      kind : aliased thinkthen_complete_utf8;
      value : aliased thinkthen_complete_atomic_double_list;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_atomic_boolean_list is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_packet_filter_aggregate is record
      c_function : aliased thinkthen_complete_utf8;
      kind : aliased thinkthen_complete_utf8;
      value : aliased thinkthen_complete_atomic_boolean_list;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_atomic_non_zero_usize is record
      answer : access constant thinkthen_complete_answer;
      answer_id : access constant thinkthen_complete_answer_id;
      images : aliased thinkthen_complete_image_list_presence;
      index : aliased thinkthen_complete_u64_presence;
      input : aliased thinkthen_complete_json_presence;
      members : aliased thinkthen_complete_rank_member_list_presence;
      meta : access constant thinkthen_complete_meta;
      question : access constant thinkthen_complete_question_document;
      question_name : aliased thinkthen_complete_utf8_presence;
      schema : access constant thinkthen_complete_result_tag;
      source : aliased thinkthen_complete_physical_source_presence;
      threshold : aliased thinkthen_complete_threshold_presence;
      value : aliased Interfaces.Unsigned_64;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_atomic_non_zero_usize_list is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_packet_rank_aggregate is record
      c_function : aliased thinkthen_complete_utf8;
      kind : aliased thinkthen_complete_utf8;
      value : aliased thinkthen_complete_atomic_non_zero_usize_list;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_find_answer is record
      confidence : aliased thinkthen_complete_f64_presence;
      kind : aliased thinkthen_complete_utf8;
      pick : aliased thinkthen_complete_utf8;
      probabilities : aliased thinkthen_complete_number_map;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_find_candidate is record
      index : aliased thinkthen_complete_u64_presence;
      input : aliased thinkthen_complete_json_presence;
      probability : aliased double;
      source : aliased thinkthen_complete_physical_source_presence;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_find_candidate_list is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_find_candidate_list_presence is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_find_candidate_list;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_find_question_document is record
      batch : aliased thinkthen_complete_batch_presence;
      context_schema : aliased thinkthen_complete_input_declaration_presence;
      item_schema : aliased thinkthen_complete_input_declaration_presence;
      label_details : aliased thinkthen_complete_label_list_presence;
      model : aliased thinkthen_complete_utf8_presence;
      name : aliased thinkthen_complete_question_name_presence;
      none : aliased Interfaces.Unsigned_32;
      on : aliased thinkthen_complete_string_list_presence;
      profile : aliased thinkthen_complete_utf8_presence;
      text : aliased thinkthen_complete_json_presence;
      verb : aliased thinkthen_complete_utf8;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_find is record
      answer : access constant thinkthen_complete_find_answer;
      answer_id : access constant thinkthen_complete_answer_id;
      candidates : aliased thinkthen_complete_find_candidate_list_presence;
      file : aliased thinkthen_complete_utf8_presence;
      first_line : aliased thinkthen_complete_u64_presence;
      index : aliased thinkthen_complete_u64_presence;
      last_line : aliased thinkthen_complete_u64_presence;
      meta : access constant thinkthen_complete_meta;
      position : aliased thinkthen_complete_position_presence;
      question : access constant thinkthen_complete_find_question_document;
      schema : access constant thinkthen_complete_result_tag;
      threshold : aliased thinkthen_complete_json_presence;
      value : aliased thinkthen_complete_json_presence;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_packet_find_aggregate is record
      c_function : aliased thinkthen_complete_utf8;
      kind : aliased thinkthen_complete_utf8;
      value : access constant thinkthen_complete_find;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_annotation_list is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_packet_annotate_aggregate is record
      c_function : aliased thinkthen_complete_utf8;
      kind : aliased thinkthen_complete_utf8;
      value : aliased thinkthen_complete_annotation_list;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_number_map_presence is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_number_map;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_name_odds is record
      edges : aliased thinkthen_complete_number_map_presence;
      c_end : aliased Interfaces.Unsigned_64;
      kinds : aliased thinkthen_complete_number_map_presence;
      start : aliased Interfaces.Unsigned_64;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_name_odds_list is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_place is record
      c_end : aliased Interfaces.Unsigned_64;
      start : aliased Interfaces.Unsigned_64;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_pair_odds is record
      probability : aliased double;
      relation : aliased thinkthen_complete_utf8;
      source : access constant thinkthen_complete_place;
      target : access constant thinkthen_complete_place;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_pair_odds_list is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_piece_odds is record
      c_end : aliased Interfaces.Unsigned_64;
      start : aliased Interfaces.Unsigned_64;
      tags : aliased thinkthen_complete_number_map;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_piece_odds_list is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_place_presence is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_place;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_recognition_proposal is record
      c_end : aliased Interfaces.Unsigned_64;
      kept : aliased Interfaces.Unsigned_32;
      kind : aliased thinkthen_complete_utf8_presence;
      selected : aliased thinkthen_complete_place_presence;
      span_probability : aliased double;
      start : aliased Interfaces.Unsigned_64;
      strength : aliased thinkthen_complete_f64_presence;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_recognition_proposal_list is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_recognition_odds_recognize_answer is record
      names : aliased thinkthen_complete_name_odds_list;
      pairs : aliased thinkthen_complete_pair_odds_list;
      pieces : aliased thinkthen_complete_piece_odds_list;
      proposals : aliased thinkthen_complete_recognition_proposal_list;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_boundary_proposal is record
      c_end : aliased Interfaces.Unsigned_64;
      file : aliased thinkthen_complete_utf8_presence;
      first_line : aliased thinkthen_complete_u64_presence;
      last_line : aliased thinkthen_complete_u64_presence;
      length : aliased Interfaces.Unsigned_64;
      probability : aliased double;
      start : aliased Interfaces.Unsigned_64;
      text : aliased thinkthen_complete_utf8;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_boundary_proposal_list is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_recognition_odds_boundary_odds is record
      pieces : aliased thinkthen_complete_piece_odds_list;
      proposals : aliased thinkthen_complete_boundary_proposal_list;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_recognition_odds_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            recognize_answer : access constant thinkthen_complete_recognition_odds_recognize_answer;
         when others =>
            boundary_odds : access constant thinkthen_complete_recognition_odds_boundary_odds;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_recognition_odds is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_recognition_odds_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_json_map_entry is record
      name : aliased thinkthen_complete_utf8;
      value : access constant thinkthen_complete_json;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_json_map is record
      data : access constant thinkthen_complete_json_map_entry;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_recognition_mode is record
      kind : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_recognition_mode_presence is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_recognition_mode;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_relation_rule is record
      either : aliased Interfaces.Unsigned_32;
      name : aliased thinkthen_complete_utf8;
      reads : aliased thinkthen_complete_utf8;
      single : aliased thinkthen_complete_u32_presence;
      source : aliased thinkthen_complete_utf8;
      target : aliased thinkthen_complete_utf8;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_relation_rule_list is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_relation_rule_list_presence is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_relation_rule_list;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_recognition_stage_context is record
      boundary : aliased thinkthen_complete_utf8_presence;
      kind_edge : aliased thinkthen_complete_utf8_presence;
      relation : aliased thinkthen_complete_utf8_presence;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_recognition_stage_context_presence is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_recognition_stage_context;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_verb is record
      kind : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_recognize_question_document is record
      batch : aliased thinkthen_complete_batch_presence;
      context_schema : aliased thinkthen_complete_input_declaration_presence;
      entity_definition : aliased thinkthen_complete_json_presence;
      instructions : aliased thinkthen_complete_json_presence;
      item_schema : aliased thinkthen_complete_input_declaration_presence;
      kinds : aliased thinkthen_complete_json_map;
      label_details : aliased thinkthen_complete_label_list_presence;
      mode : aliased thinkthen_complete_recognition_mode_presence;
      model : aliased thinkthen_complete_utf8_presence;
      name : aliased thinkthen_complete_question_name_presence;
      on : aliased thinkthen_complete_string_list_presence;
      profile : aliased thinkthen_complete_utf8_presence;
      relation_threshold : aliased thinkthen_complete_threshold_presence;
      relations : aliased thinkthen_complete_relation_rule_list_presence;
      snippet_pieces : aliased thinkthen_complete_u64_presence;
      stage_context : aliased thinkthen_complete_recognition_stage_context_presence;
      threshold : access constant thinkthen_complete_threshold;
      verb : access constant thinkthen_complete_verb;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_entity is record
      c_end : aliased Interfaces.Unsigned_64;
      file : aliased thinkthen_complete_utf8_presence;
      first_line : aliased thinkthen_complete_u64_presence;
      kind : aliased thinkthen_complete_utf8;
      last_line : aliased thinkthen_complete_u64_presence;
      length : aliased Interfaces.Unsigned_64;
      start : aliased Interfaces.Unsigned_64;
      strength : aliased double;
      text : aliased thinkthen_complete_utf8;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_entity_list is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_entity_edge is record
      either : aliased thinkthen_complete_u32_presence;
      probability : aliased double;
      relation : aliased thinkthen_complete_utf8;
      source : access constant thinkthen_complete_entity;
      target : access constant thinkthen_complete_entity;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_entity_edge_list is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_entity_edge_list_presence is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_entity_edge_list;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_recognize_entities is record
      entities : aliased thinkthen_complete_entity_list;
      relations : aliased thinkthen_complete_entity_edge_list_presence;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_boundary_mode is record
      kind : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_recognize_mode_proposals is record
      mode : access constant thinkthen_complete_boundary_mode;
      proposals : aliased thinkthen_complete_boundary_proposal_list;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_recognize_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            entities : access constant thinkthen_complete_recognize_entities;
         when others =>
            mode_proposals : access constant thinkthen_complete_recognize_mode_proposals;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_recognize is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_recognize_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_recognition is record
      answer : access constant thinkthen_complete_recognition_odds;
      answer_id : access constant thinkthen_complete_answer_id;
      file : aliased thinkthen_complete_utf8_presence;
      first_line : aliased thinkthen_complete_u64_presence;
      index : aliased thinkthen_complete_u64_presence;
      input : aliased thinkthen_complete_json_presence;
      last_line : aliased thinkthen_complete_u64_presence;
      meta : access constant thinkthen_complete_meta;
      position : aliased thinkthen_complete_position_presence;
      question : access constant thinkthen_complete_recognize_question_document;
      schema : access constant thinkthen_complete_result_tag;
      source : aliased thinkthen_complete_physical_source_presence;
      value : access constant thinkthen_complete_recognize;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_recognition_list is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_packet_recognize_aggregate is record
      c_function : aliased thinkthen_complete_utf8;
      kind : aliased thinkthen_complete_utf8;
      value : aliased thinkthen_complete_recognition_list;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_relation_direction is record
      kind : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_relation_method is record
      kind : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_related_entity is record
      kind : aliased thinkthen_complete_utf8;
      name : aliased thinkthen_complete_utf8;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_related_entity_presence is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_related_entity;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_relation_member_answer_id is record
      direction : access constant thinkthen_complete_relation_direction;
      method : access constant thinkthen_complete_relation_method;
      observations : aliased thinkthen_complete_observation_list;
      question : access constant thinkthen_complete_question_document;
      question_sources : aliased thinkthen_complete_question_source_list;
      reads : aliased thinkthen_complete_utf8;
      relation : aliased thinkthen_complete_utf8;
      request : aliased thinkthen_complete_utf8;
      source : access constant thinkthen_complete_related_entity;
      target : aliased thinkthen_complete_related_entity_presence;
      threshold : access constant thinkthen_complete_threshold;
      usage : aliased thinkthen_complete_usage_presence;
      accepted : aliased Interfaces.Unsigned_32;
      answer : access constant thinkthen_complete_answer;
      answer_id : access constant thinkthen_complete_answer_id;
      probability : aliased double;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_relation_member_failure_id is record
      direction : access constant thinkthen_complete_relation_direction;
      method : access constant thinkthen_complete_relation_method;
      observations : aliased thinkthen_complete_observation_list;
      question : access constant thinkthen_complete_question_document;
      question_sources : aliased thinkthen_complete_question_source_list;
      reads : aliased thinkthen_complete_utf8;
      relation : aliased thinkthen_complete_utf8;
      request : aliased thinkthen_complete_utf8;
      source : access constant thinkthen_complete_related_entity;
      target : aliased thinkthen_complete_related_entity_presence;
      threshold : access constant thinkthen_complete_threshold;
      usage : aliased thinkthen_complete_usage_presence;
      failure : access constant thinkthen_complete_failure;
      failure_id : access constant thinkthen_complete_failure_id;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_relation_member_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            answer_id : access constant thinkthen_complete_relation_member_answer_id;
         when others =>
            failure_id : access constant thinkthen_complete_relation_member_failure_id;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_relation_member is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_relation_member_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_relation_member_list is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answers is record
      questions : aliased thinkthen_complete_relation_member_list;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_input_source is record
      index : aliased Interfaces.Unsigned_64;
      source : access constant thinkthen_complete_physical_source;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_input_source_list is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_input_source_list_presence is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_session_input_source_list;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_relate_fields is record
      kind : aliased thinkthen_complete_utf8;
      name : aliased thinkthen_complete_utf8;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_relate_fields_presence is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_relate_fields;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_relate_question_document is record
      batch : aliased thinkthen_complete_batch_presence;
      context_schema : aliased thinkthen_complete_input_declaration_presence;
      fields : aliased thinkthen_complete_relate_fields_presence;
      item_schema : aliased thinkthen_complete_input_declaration_presence;
      label_details : aliased thinkthen_complete_label_list_presence;
      model : aliased thinkthen_complete_utf8_presence;
      name : aliased thinkthen_complete_question_name_presence;
      on : aliased thinkthen_complete_string_list_presence;
      profile : aliased thinkthen_complete_utf8_presence;
      relations : aliased thinkthen_complete_relation_rule_list;
      threshold : access constant thinkthen_complete_threshold;
      verb : aliased thinkthen_complete_utf8;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_relation_endpoint_related_entity is record
      kind : aliased thinkthen_complete_utf8;
      name : aliased thinkthen_complete_utf8;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_relation_endpoint_source is record
      file : aliased thinkthen_complete_utf8_presence;
      first_line : aliased thinkthen_complete_u64_presence;
      kind : aliased thinkthen_complete_utf8;
      last_line : aliased thinkthen_complete_u64_presence;
      name : aliased thinkthen_complete_utf8;
      ordinal : aliased Interfaces.Unsigned_64;
      c_record : aliased thinkthen_complete_json_presence;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_relation_endpoint_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            related_entity : access constant thinkthen_complete_relation_endpoint_related_entity;
         when others =>
            source : access constant thinkthen_complete_relation_endpoint_source;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_relation_endpoint is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_relation_endpoint_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_related_entity_edge is record
      either : aliased thinkthen_complete_u32_presence;
      probability : aliased double;
      relation : aliased thinkthen_complete_utf8;
      source : access constant thinkthen_complete_relation_endpoint;
      target : access constant thinkthen_complete_relation_endpoint;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_related_entity_edge_list is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_relation is record
      answer : access constant thinkthen_complete_answers;
      answer_id : access constant thinkthen_complete_answer_id;
      file : aliased thinkthen_complete_utf8_presence;
      first_line : aliased thinkthen_complete_u64_presence;
      index : aliased thinkthen_complete_u64_presence;
      input : aliased thinkthen_complete_json_presence;
      input_sources : aliased thinkthen_complete_session_input_source_list_presence;
      last_line : aliased thinkthen_complete_u64_presence;
      meta : access constant thinkthen_complete_meta;
      position : aliased thinkthen_complete_position_presence;
      question : access constant thinkthen_complete_relate_question_document;
      schema : access constant thinkthen_complete_result_tag;
      value : aliased thinkthen_complete_related_entity_edge_list;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_packet_relate_aggregate is record
      c_function : aliased thinkthen_complete_utf8;
      kind : aliased thinkthen_complete_utf8;
      value : access constant thinkthen_complete_relation;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_request_function is record
      kind : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answer_id_presence is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_answer_id;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_failure_presence is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_failure;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_failure_id_presence is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_failure_id;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_json_list is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_probabilities_yes_no is record
      kind : aliased thinkthen_complete_utf8;
      value : aliased double;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_named_probability is record
      name : aliased thinkthen_complete_utf8;
      probability : aliased double;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_named_probability_list is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_probabilities_named is record
      kind : aliased thinkthen_complete_utf8;
      value : aliased thinkthen_complete_session_named_probability_list;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_probabilities_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            yes_no : access constant thinkthen_complete_session_probabilities_yes_no;
         when others =>
            named : access constant thinkthen_complete_session_probabilities_named;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_session_probabilities is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_session_probabilities_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_probabilities_presence is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_session_probabilities;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_token_usage is record
      input_tokens : aliased Interfaces.Unsigned_64;
      output_tokens : aliased Interfaces.Unsigned_64;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_token_usage_presence is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_token_usage;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_question_detail is record
      answer_id : aliased thinkthen_complete_answer_id_presence;
      cached : aliased Interfaces.Unsigned_32;
      confidence : aliased thinkthen_complete_f64_presence;
      failed_questions : aliased Interfaces.Unsigned_64;
      failure : aliased thinkthen_complete_failure_presence;
      failure_id : aliased thinkthen_complete_failure_id_presence;
      input : aliased thinkthen_complete_json_presence;
      input_source : aliased thinkthen_complete_physical_source_presence;
      input_sources : aliased thinkthen_complete_session_input_source_list;
      inputs : aliased thinkthen_complete_json_list;
      model : aliased thinkthen_complete_utf8;
      observations : aliased thinkthen_complete_observation_list;
      probabilities : aliased thinkthen_complete_session_probabilities_presence;
      question : access constant thinkthen_complete_question_document;
      question_sha256 : aliased thinkthen_complete_utf8;
      question_sources : aliased thinkthen_complete_question_source_list;
      raw_pick : aliased thinkthen_complete_utf8_presence;
      reported_usage : aliased thinkthen_complete_usage_presence;
      requests : aliased thinkthen_complete_string_list;
      requests_sent : aliased Interfaces.Unsigned_64;
      threshold : aliased thinkthen_complete_threshold_presence;
      url : aliased thinkthen_complete_utf8;
      usage : aliased thinkthen_complete_token_usage_presence;
      value : aliased thinkthen_complete_value_presence;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_observation_question is record
      detail : access constant thinkthen_complete_session_question_detail;
      index : aliased Interfaces.Unsigned_64;
      kind : aliased thinkthen_complete_utf8;
      member : aliased thinkthen_complete_utf8_presence;
      position : aliased Interfaces.Unsigned_64;
      stage : aliased thinkthen_complete_utf8_presence;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_judgment_decision is record
      kind : aliased thinkthen_complete_utf8;
      value : aliased thinkthen_complete_u32_presence;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_judgment_choice is record
      kind : aliased thinkthen_complete_utf8;
      value : aliased thinkthen_complete_utf8_presence;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_judgment_score is record
      kind : aliased thinkthen_complete_utf8;
      value : aliased double;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_judgment_tags is record
      kind : aliased thinkthen_complete_utf8;
      value : aliased thinkthen_complete_string_list;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_judgment_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            decision : access constant thinkthen_complete_session_judgment_decision;
         when 1 =>
            choice : access constant thinkthen_complete_session_judgment_choice;
         when 2 =>
            score : access constant thinkthen_complete_session_judgment_score;
         when others =>
            tags : access constant thinkthen_complete_session_judgment_tags;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_session_judgment is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_session_judgment_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_observed_row_judgment is record
      kind : aliased thinkthen_complete_utf8;
      value : access constant thinkthen_complete_session_judgment;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_annotation_value_decision is record
      kind : aliased thinkthen_complete_utf8;
      value : aliased thinkthen_complete_u32_presence;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_annotation_value_choice is record
      kind : aliased thinkthen_complete_utf8;
      value : aliased thinkthen_complete_utf8_presence;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_annotation_value_score is record
      kind : aliased thinkthen_complete_utf8;
      value : aliased double;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_annotation_value_tags is record
      kind : aliased thinkthen_complete_utf8;
      value : aliased thinkthen_complete_string_list;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_annotation_value_failed is record
      kind : aliased thinkthen_complete_utf8;
      value : access constant thinkthen_complete_failure;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_annotation_value_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            decision : access constant thinkthen_complete_annotation_value_decision;
         when 1 =>
            choice : access constant thinkthen_complete_annotation_value_choice;
         when 2 =>
            score : access constant thinkthen_complete_annotation_value_score;
         when 3 =>
            tags : access constant thinkthen_complete_annotation_value_tags;
         when others =>
            failed : access constant thinkthen_complete_annotation_value_failed;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_annotation_value is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_annotation_value_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_annotation is record
      name : aliased thinkthen_complete_utf8;
      value : access constant thinkthen_complete_annotation_value;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_annotation_list is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_observed_row_annotated is record
      kind : aliased thinkthen_complete_utf8;
      value : aliased thinkthen_complete_session_annotation_list;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_boundary_proposal_list_presence is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_boundary_proposal_list;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_recognition_edge is record
      either : aliased Interfaces.Unsigned_32;
      probability : aliased double;
      relation : aliased thinkthen_complete_utf8;
      source : access constant thinkthen_complete_entity;
      target : access constant thinkthen_complete_entity;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_recognition_edge_list is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_recognition_edge_list_presence is record
      presence : aliased Interfaces.Unsigned_32;
      value : aliased thinkthen_complete_recognition_edge_list;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_recognition is record
      entities : aliased thinkthen_complete_entity_list;
      mode : access constant thinkthen_complete_recognition_mode;
      proposals : aliased thinkthen_complete_boundary_proposal_list_presence;
      relations : aliased thinkthen_complete_recognition_edge_list_presence;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_observed_row_recognized is record
      kind : aliased thinkthen_complete_utf8;
      value : access constant thinkthen_complete_session_recognition;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_observed_row_find is record
      kind : aliased thinkthen_complete_utf8;
      value : aliased thinkthen_complete_u64_presence;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_entity_document is record
      kind : aliased thinkthen_complete_utf8;
      name : aliased thinkthen_complete_utf8;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_relation_edge is record
      either : aliased Interfaces.Unsigned_32;
      probability : aliased double;
      relation : aliased thinkthen_complete_utf8;
      source : access constant thinkthen_complete_entity_document;
      target : access constant thinkthen_complete_entity_document;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_relation_edge_list is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_observed_row_relations is record
      kind : aliased thinkthen_complete_utf8;
      value : aliased thinkthen_complete_session_relation_edge_list;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_observed_row_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            judgment : access constant thinkthen_complete_session_observed_row_judgment;
         when 1 =>
            annotated : access constant thinkthen_complete_session_observed_row_annotated;
         when 2 =>
            recognized : access constant thinkthen_complete_session_observed_row_recognized;
         when 3 =>
            find : access constant thinkthen_complete_session_observed_row_find;
         when others =>
            relations : access constant thinkthen_complete_session_observed_row_relations;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_session_observed_row is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_session_observed_row_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_observation_row is record
      index : aliased Interfaces.Unsigned_64;
      kind : aliased thinkthen_complete_utf8;
      value : access constant thinkthen_complete_session_observed_row;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_observation_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            question : access constant thinkthen_complete_session_observation_question;
         when others =>
            row : access constant thinkthen_complete_session_observation_row;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_session_observation is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_session_observation_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_packet_observation is record
      c_function : access constant thinkthen_complete_request_function;
      kind : aliased thinkthen_complete_utf8;
      value : access constant thinkthen_complete_session_observation;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answer_shape_4_any_of_1_items_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            integer : aliased Interfaces.Unsigned_64;
         when others =>
            c_null : aliased Interfaces.Unsigned_32;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_answer_shape_4_any_of_1_items is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_answer_shape_4_any_of_1_items_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answer_shape_4_any_of_1_items_list is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answer_shape_4_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            integer : aliased Interfaces.Unsigned_64;
         when 1 =>
            c_null : aliased Interfaces.Unsigned_32;
         when others =>
            c_array : aliased thinkthen_complete_answer_shape_4_any_of_1_items_list;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_answer_shape_4 is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_answer_shape_4_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answer_shape_4_presence is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_answer_shape_4;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answer_shape_6_any_of_1_items_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            string : aliased thinkthen_complete_utf8;
         when others =>
            c_null : aliased Interfaces.Unsigned_32;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_answer_shape_6_any_of_1_items is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_answer_shape_6_any_of_1_items_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answer_shape_6_any_of_1_items_list is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answer_shape_3_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            string : aliased thinkthen_complete_utf8;
         when 1 =>
            c_null : aliased Interfaces.Unsigned_32;
         when others =>
            c_array : aliased thinkthen_complete_answer_shape_6_any_of_1_items_list;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_answer_shape_3 is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_answer_shape_3_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answer_shape_3_presence is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_answer_shape_3;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answer_shape_8_any_of_1_items_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            number : aliased double;
         when others =>
            c_null : aliased Interfaces.Unsigned_32;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_answer_shape_8_any_of_1_items is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_answer_shape_8_any_of_1_items_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answer_shape_8_any_of_1_items_list is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answer_shape_2_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            number : aliased double;
         when 1 =>
            c_null : aliased Interfaces.Unsigned_32;
         when others =>
            c_array : aliased thinkthen_complete_answer_shape_8_any_of_1_items_list;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_answer_shape_2 is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_answer_shape_2_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answer_shape_2_presence is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_answer_shape_2;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answer_resolution is record
      kind : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answer_resolution_map_entry is record
      name : aliased thinkthen_complete_utf8;
      value : access constant thinkthen_complete_answer_resolution;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answer_resolution_map is record
      data : access constant thinkthen_complete_answer_resolution_map_entry;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answer_resolutions_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            string : access constant thinkthen_complete_answer_resolution;
         when others =>
            object : aliased thinkthen_complete_answer_resolution_map;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_answer_resolutions is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_answer_resolutions_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answer_resolutions_list is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answer_shape_5_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            string : access constant thinkthen_complete_answer_resolution;
         when 1 =>
            object : aliased thinkthen_complete_answer_resolution_map;
         when others =>
            c_array : aliased thinkthen_complete_answer_resolutions_list;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_answer_shape_5 is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_answer_shape_5_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answer_shape is record
      value : access constant thinkthen_complete_json;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answer_shape_presence is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_answer_shape;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_settled_answer_decide is record
      c_function : aliased thinkthen_complete_utf8;
      index : aliased thinkthen_complete_answer_shape_4_presence;
      pick : aliased thinkthen_complete_answer_shape_3_presence;
      probability : aliased thinkthen_complete_answer_shape_2_presence;
      resolution : access constant thinkthen_complete_answer_shape_5;
      value : aliased thinkthen_complete_answer_shape_presence;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answer_shape_6_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            string : aliased thinkthen_complete_utf8;
         when 1 =>
            c_null : aliased Interfaces.Unsigned_32;
         when others =>
            c_array : aliased thinkthen_complete_answer_shape_6_any_of_1_items_list;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_answer_shape_6 is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_answer_shape_6_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answer_shape_6_presence is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_answer_shape_6;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_settled_answer_choose is record
      c_function : aliased thinkthen_complete_utf8;
      index : aliased thinkthen_complete_answer_shape_4_presence;
      pick : aliased thinkthen_complete_answer_shape_3_presence;
      probability : aliased thinkthen_complete_answer_shape_2_presence;
      resolution : access constant thinkthen_complete_answer_shape_5;
      value : aliased thinkthen_complete_answer_shape_6_presence;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answer_shape_7_any_of_1_items_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            c_array : aliased thinkthen_complete_string_list;
         when others =>
            c_null : aliased Interfaces.Unsigned_32;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_answer_shape_7_any_of_1_items is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_answer_shape_7_any_of_1_items_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answer_shape_7_any_of_1_items_list is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answer_shape_7_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            c_array : aliased thinkthen_complete_string_list;
         when 1 =>
            c_null : aliased Interfaces.Unsigned_32;
         when others =>
            array2 : aliased thinkthen_complete_answer_shape_7_any_of_1_items_list;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_answer_shape_7 is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_answer_shape_7_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answer_shape_7_presence is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_answer_shape_7;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_settled_answer_tag is record
      c_function : aliased thinkthen_complete_utf8;
      index : aliased thinkthen_complete_answer_shape_4_presence;
      pick : aliased thinkthen_complete_answer_shape_3_presence;
      probability : aliased thinkthen_complete_answer_shape_2_presence;
      resolution : access constant thinkthen_complete_answer_shape_5;
      value : aliased thinkthen_complete_answer_shape_7_presence;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answer_shape_8_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            number : aliased double;
         when 1 =>
            c_null : aliased Interfaces.Unsigned_32;
         when others =>
            c_array : aliased thinkthen_complete_answer_shape_8_any_of_1_items_list;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_answer_shape_8 is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_answer_shape_8_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answer_shape_8_presence is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_answer_shape_8;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_settled_answer_score is record
      c_function : aliased thinkthen_complete_utf8;
      index : aliased thinkthen_complete_answer_shape_4_presence;
      pick : aliased thinkthen_complete_answer_shape_3_presence;
      probability : aliased thinkthen_complete_answer_shape_2_presence;
      resolution : access constant thinkthen_complete_answer_shape_5;
      value : aliased thinkthen_complete_answer_shape_8_presence;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answer_shape_10_any_of_1_items_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            c_array : aliased thinkthen_complete_answer_shape_8_any_of_1_items_list;
         when others =>
            c_null : aliased Interfaces.Unsigned_32;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_answer_shape_10_any_of_1_items is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_answer_shape_10_any_of_1_items_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answer_shape_10_any_of_1_items_list is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answer_shape_10_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            c_array : aliased thinkthen_complete_answer_shape_8_any_of_1_items_list;
         when 1 =>
            c_null : aliased Interfaces.Unsigned_32;
         when others =>
            array2 : aliased thinkthen_complete_answer_shape_10_any_of_1_items_list;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_answer_shape_10 is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_answer_shape_10_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answer_shape_10_presence is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_answer_shape_10;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_settled_original is record
      value : access constant thinkthen_complete_json;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_settled_original_list is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answer_shape_9_any_of_1_items_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            c_array : aliased thinkthen_complete_settled_original_list;
         when others =>
            c_null : aliased Interfaces.Unsigned_32;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_answer_shape_9_any_of_1_items is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_answer_shape_9_any_of_1_items_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answer_shape_9_any_of_1_items_list is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answer_shape_9_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            c_array : aliased thinkthen_complete_settled_original_list;
         when 1 =>
            c_null : aliased Interfaces.Unsigned_32;
         when others =>
            array2 : aliased thinkthen_complete_answer_shape_9_any_of_1_items_list;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_answer_shape_9 is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_answer_shape_9_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answer_shape_9_presence is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_answer_shape_9;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_settled_answer_filter is record
      c_function : aliased thinkthen_complete_utf8;
      index : aliased thinkthen_complete_answer_shape_4_presence;
      pick : aliased thinkthen_complete_answer_shape_3_presence;
      probability : aliased thinkthen_complete_answer_shape_10_presence;
      resolution : access constant thinkthen_complete_answer_shape_5;
      value : aliased thinkthen_complete_answer_shape_9_presence;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_integer_list is record
      data : access Interfaces.Unsigned_64;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answer_shape_11_any_of_1_items_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            c_array : aliased thinkthen_complete_integer_list;
         when others =>
            c_null : aliased Interfaces.Unsigned_32;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_answer_shape_11_any_of_1_items is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_answer_shape_11_any_of_1_items_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answer_shape_11_any_of_1_items_list is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answer_shape_11_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            c_array : aliased thinkthen_complete_integer_list;
         when 1 =>
            c_null : aliased Interfaces.Unsigned_32;
         when others =>
            array2 : aliased thinkthen_complete_answer_shape_11_any_of_1_items_list;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_answer_shape_11 is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_answer_shape_11_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answer_shape_11_presence is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_answer_shape_11;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_settled_answer_rank is record
      c_function : aliased thinkthen_complete_utf8;
      index : aliased thinkthen_complete_answer_shape_4_presence;
      pick : aliased thinkthen_complete_answer_shape_3_presence;
      probability : aliased thinkthen_complete_answer_shape_10_presence;
      resolution : access constant thinkthen_complete_answer_shape_5;
      value : aliased thinkthen_complete_answer_shape_11_presence;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answer_shape_12 is record
      value : access constant thinkthen_complete_json;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answer_shape_12_presence is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_answer_shape_12;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_settled_answer_find is record
      c_function : aliased thinkthen_complete_utf8;
      index : aliased thinkthen_complete_answer_shape_4_presence;
      pick : aliased thinkthen_complete_answer_shape_3_presence;
      probability : aliased thinkthen_complete_answer_shape_2_presence;
      resolution : access constant thinkthen_complete_answer_shape_5;
      value : aliased thinkthen_complete_answer_shape_12_presence;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_settled_recognition_entities is record
      entities : aliased thinkthen_complete_entity_list;
      relations : aliased thinkthen_complete_entity_edge_list_presence;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_settled_recognition_mode_proposals is record
      mode : access constant thinkthen_complete_boundary_mode;
      proposals : aliased thinkthen_complete_boundary_proposal_list;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_settled_recognition_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            entities : access constant thinkthen_complete_settled_recognition_entities;
         when others =>
            mode_proposals : access constant thinkthen_complete_settled_recognition_mode_proposals;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_settled_recognition is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_settled_recognition_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answer_shape_13_any_of_1_items_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            object : access constant thinkthen_complete_settled_recognition;
         when others =>
            c_null : aliased Interfaces.Unsigned_32;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_answer_shape_13_any_of_1_items is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_answer_shape_13_any_of_1_items_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answer_shape_13_any_of_1_items_list is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answer_shape_13_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            object : access constant thinkthen_complete_settled_recognition;
         when 1 =>
            c_null : aliased Interfaces.Unsigned_32;
         when others =>
            c_array : aliased thinkthen_complete_answer_shape_13_any_of_1_items_list;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_answer_shape_13 is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_answer_shape_13_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answer_shape_13_presence is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_answer_shape_13;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_settled_answer_recognize is record
      c_function : aliased thinkthen_complete_utf8;
      index : aliased thinkthen_complete_answer_shape_4_presence;
      pick : aliased thinkthen_complete_answer_shape_3_presence;
      probability : aliased thinkthen_complete_answer_shape_10_presence;
      resolution : access constant thinkthen_complete_answer_shape_5;
      value : aliased thinkthen_complete_answer_shape_13_presence;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_settled_relations is record
      value : aliased thinkthen_complete_related_entity_edge_list;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answer_shape_14_any_of_1_items_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            c_array : access constant thinkthen_complete_settled_relations;
         when others =>
            c_null : aliased Interfaces.Unsigned_32;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_answer_shape_14_any_of_1_items is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_answer_shape_14_any_of_1_items_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answer_shape_14_any_of_1_items_list is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answer_shape_14_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            c_array : access constant thinkthen_complete_settled_relations;
         when 1 =>
            c_null : aliased Interfaces.Unsigned_32;
         when others =>
            array2 : aliased thinkthen_complete_answer_shape_14_any_of_1_items_list;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_answer_shape_14 is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_answer_shape_14_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answer_shape_14_presence is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_answer_shape_14;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_settled_answer_relate is record
      c_function : aliased thinkthen_complete_utf8;
      index : aliased thinkthen_complete_answer_shape_4_presence;
      pick : aliased thinkthen_complete_answer_shape_3_presence;
      probability : aliased thinkthen_complete_answer_shape_10_presence;
      resolution : access constant thinkthen_complete_answer_shape_5;
      value : aliased thinkthen_complete_answer_shape_14_presence;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_settled_judgment_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            boolean : aliased Interfaces.Unsigned_32;
         when 1 =>
            c_null : aliased Interfaces.Unsigned_32;
         when 2 =>
            string : aliased thinkthen_complete_utf8;
         when 3 =>
            number : aliased double;
         when others =>
            c_array : aliased thinkthen_complete_string_list;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_settled_judgment is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_settled_judgment_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_settled_judgment_map_entry is record
      name : aliased thinkthen_complete_utf8;
      value : access constant thinkthen_complete_settled_judgment;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_settled_judgment_map is record
      data : access constant thinkthen_complete_settled_judgment_map_entry;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answer_shape_15_any_of_1_items_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            object : aliased thinkthen_complete_settled_judgment_map;
         when others =>
            c_null : aliased Interfaces.Unsigned_32;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_answer_shape_15_any_of_1_items is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_answer_shape_15_any_of_1_items_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answer_shape_15_any_of_1_items_list is record
      data : System.Address;
      len : aliased Interfaces.C.size_t;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answer_shape_15_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            object : aliased thinkthen_complete_settled_judgment_map;
         when 1 =>
            c_null : aliased Interfaces.Unsigned_32;
         when others =>
            c_array : aliased thinkthen_complete_answer_shape_15_any_of_1_items_list;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_answer_shape_15 is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_answer_shape_15_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_answer_shape_15_presence is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_answer_shape_15;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_settled_answer_annotate is record
      c_function : aliased thinkthen_complete_utf8;
      index : aliased thinkthen_complete_answer_shape_4_presence;
      pick : aliased thinkthen_complete_answer_shape_3_presence;
      probability : aliased thinkthen_complete_answer_shape_2_presence;
      resolution : access constant thinkthen_complete_answer_shape_5;
      value : aliased thinkthen_complete_answer_shape_15_presence;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_settled_answer_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            decide : access constant thinkthen_complete_settled_answer_decide;
         when 1 =>
            choose : access constant thinkthen_complete_settled_answer_choose;
         when 2 =>
            tag : access constant thinkthen_complete_settled_answer_tag;
         when 3 =>
            score : access constant thinkthen_complete_settled_answer_score;
         when 4 =>
            filter : access constant thinkthen_complete_settled_answer_filter;
         when 5 =>
            rank : access constant thinkthen_complete_settled_answer_rank;
         when 6 =>
            find : access constant thinkthen_complete_settled_answer_find;
         when 7 =>
            recognize : access constant thinkthen_complete_settled_answer_recognize;
         when 8 =>
            relate : access constant thinkthen_complete_settled_answer_relate;
         when others =>
            annotate : access constant thinkthen_complete_settled_answer_annotate;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_settled_answer is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_settled_answer_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_settled_answer_presence is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_settled_answer;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_facts is record
      attempts : aliased thinkthen_complete_attempt_list_presence;
      cache_answers : aliased Interfaces.Unsigned_64;
      call_id : access constant thinkthen_complete_call_id;
      estimated_cost_usd : aliased thinkthen_complete_utf8_presence;
      held_model_mismatch : aliased thinkthen_complete_u32_presence;
      input_tokens : aliased thinkthen_complete_u64_presence;
      largest_request_bytes : aliased Interfaces.Unsigned_64;
      largest_request_estimated_input_tokens : aliased thinkthen_complete_u64_presence;
      model : aliased thinkthen_complete_utf8_presence;
      output_tokens : aliased thinkthen_complete_u64_presence;
      records : aliased Interfaces.Unsigned_64;
      requests_sent : aliased Interfaces.Unsigned_64;
      seconds : aliased double;
      token_estimate_method : aliased thinkthen_complete_utf8;
      usage_persistence : aliased thinkthen_complete_persistence_observation_presence;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_facts_presence is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_facts;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_input_denial_initial_request is record
      kind : aliased thinkthen_complete_utf8;
      limit : aliased Interfaces.Unsigned_64;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_input_denial_additional_request is record
      kind : aliased thinkthen_complete_utf8;
      limit : aliased Interfaces.Unsigned_64;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_input_denial_retry is record
      kind : aliased thinkthen_complete_utf8;
      last_status : aliased Interfaces.Unsigned_64;
      limit : aliased Interfaces.Unsigned_64;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_input_denial_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            initial_request : access constant thinkthen_complete_input_denial_initial_request;
         when 1 =>
            additional_request : access constant thinkthen_complete_input_denial_additional_request;
         when others =>
            retry : access constant thinkthen_complete_input_denial_retry;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_input_denial is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_input_denial_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_input_denial_presence is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_input_denial;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_failure_kind is record
      kind : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_send_denial_before_first_send is record
      kind : aliased thinkthen_complete_utf8;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_send_denial_before_additional_send is record
      kind : aliased thinkthen_complete_utf8;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_send_denial_before_retry is record
      kind : aliased thinkthen_complete_utf8;
      last_status : aliased Interfaces.Unsigned_64;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_send_denial_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            before_first_send : access constant thinkthen_complete_send_denial_before_first_send;
         when 1 =>
            before_additional_send : access constant thinkthen_complete_send_denial_before_additional_send;
         when others =>
            before_retry : access constant thinkthen_complete_send_denial_before_retry;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_send_denial is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_send_denial_data;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_send_denial_presence is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_send_denial;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_stop_cause is record
      kind : aliased Interfaces.Unsigned_32;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_stopped is record
      c_at : aliased thinkthen_complete_u64_presence;
      cause : access constant thinkthen_complete_stop_cause;
      retryable : aliased Interfaces.Unsigned_32;
      status : aliased thinkthen_complete_u64_presence;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_error is record
      estimated_input_denial : aliased thinkthen_complete_input_denial_presence;
      kind : access constant thinkthen_complete_failure_kind;
      message : aliased thinkthen_complete_utf8;
      retryable : aliased Interfaces.Unsigned_32;
      send_budget_denial : aliased thinkthen_complete_send_denial_presence;
      stopped : access constant thinkthen_complete_stopped;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_call_error is record
      error : access constant thinkthen_complete_error;
      facts : aliased thinkthen_complete_facts_presence;
      resolution : access constant thinkthen_complete_answer_resolution;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_call_error_presence is record
      presence : aliased Interfaces.Unsigned_32;
      value : access constant thinkthen_complete_call_error;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_packet_terminal is record
      answer : aliased thinkthen_complete_settled_answer_presence;
      facts : aliased thinkthen_complete_facts_presence;
      failure : aliased thinkthen_complete_call_error_presence;
      kind : aliased thinkthen_complete_utf8;
      resolution : access constant thinkthen_complete_answer_resolution;
      extensions : aliased thinkthen_complete_extensions;
   end record
   with Convention => C_Pass_By_Copy;
   type thinkthen_complete_session_packet_data (discr : unsigned := 0) is record
      case discr is
         when 0 =>
            decide_row : access constant thinkthen_complete_session_packet_decide_row;
         when 1 =>
            choose_row : access constant thinkthen_complete_session_packet_choose_row;
         when 2 =>
            tag_row : access constant thinkthen_complete_session_packet_tag_row;
         when 3 =>
            score_row : access constant thinkthen_complete_session_packet_score_row;
         when 4 =>
            filter_row : access constant thinkthen_complete_session_packet_filter_row;
         when 5 =>
            annotate_row : access constant thinkthen_complete_session_packet_annotate_row;
         when 6 =>
            decide_aggregate : access constant thinkthen_complete_session_packet_decide_aggregate;
         when 7 =>
            choose_aggregate : access constant thinkthen_complete_session_packet_choose_aggregate;
         when 8 =>
            tag_aggregate : access constant thinkthen_complete_session_packet_tag_aggregate;
         when 9 =>
            score_aggregate : access constant thinkthen_complete_session_packet_score_aggregate;
         when 10 =>
            filter_aggregate : access constant thinkthen_complete_session_packet_filter_aggregate;
         when 11 =>
            rank_aggregate : access constant thinkthen_complete_session_packet_rank_aggregate;
         when 12 =>
            find_aggregate : access constant thinkthen_complete_session_packet_find_aggregate;
         when 13 =>
            annotate_aggregate : access constant thinkthen_complete_session_packet_annotate_aggregate;
         when 14 =>
            recognize_aggregate : access constant thinkthen_complete_session_packet_recognize_aggregate;
         when 15 =>
            relate_aggregate : access constant thinkthen_complete_session_packet_relate_aggregate;
         when 16 =>
            observation : access constant thinkthen_complete_session_packet_observation;
         when others =>
            terminal : access constant thinkthen_complete_session_packet_terminal;
      end case;
   end record
   with Convention => C_Pass_By_Copy,
        Unchecked_Union => True;
   type thinkthen_complete_session_packet is record
      kind : aliased Interfaces.Unsigned_32;
      data : aliased thinkthen_complete_session_packet_data;
   end record
   with Convention => C_Pass_By_Copy;
   function thinkthen_engine_finish_usage_status
     (engine : access constant thinkthen_engine;
      out_state : access thinkthen_complete_usage_persistence;
      out_advice : access thinkthen_complete_utf8) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_engine_finish_usage_status";
   procedure thinkthen_engine_free (engine : access thinkthen_engine)
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_engine_free";
   function thinkthen_engine_new (error : System.Address) return access thinkthen_engine
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_engine_new";
   function thinkthen_engine_new_typed (settings : access constant thinkthen_request_engine_settings; error : System.Address) return access thinkthen_engine
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_engine_new_typed";
   function thinkthen_engine_new_with (settings_json : Interfaces.C.Strings.chars_ptr; error : System.Address) return access thinkthen_engine
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_engine_new_with";
   function thinkthen_engine_usage_persistence
     (engine : access constant thinkthen_engine;
      out_state : access thinkthen_complete_usage_persistence;
      out_advice : access thinkthen_complete_utf8) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_engine_usage_persistence";
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
   function thinkthen_error_view (engine : access constant thinkthen_engine) return access constant thinkthen_error_details
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_error_view";
   procedure thinkthen_free_string (text : Interfaces.C.Strings.chars_ptr)
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_free_string";
   function thinkthen_question_author (owner : access constant thinkthen_question; c_out : access thinkthen_question_author_info) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_question_author";
   procedure thinkthen_question_free (owner : access thinkthen_question)
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_question_free";
   function thinkthen_question_parse
     (engine : access constant thinkthen_engine;
      role : Interfaces.Unsigned_32;
      json : thinkthen_string;
      c_out : System.Address) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_question_parse";
   function thinkthen_question_value (owner : access constant thinkthen_question; c_out : System.Address) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_question_value";
   function thinkthen_request_plan_typed_json
     (engine : access constant thinkthen_engine;
      c_function : thinkthen_complete_utf8;
      arguments : access constant thinkthen_request_arguments;
      surface_name : thinkthen_complete_utf8;
      c_out : System.Address;
      out_len : access Interfaces.C.size_t) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_request_plan_typed_json";
   function thinkthen_request_plan_typed_value
     (engine : access constant thinkthen_engine;
      c_function : thinkthen_complete_utf8;
      arguments : access constant thinkthen_request_arguments;
      surface_name : thinkthen_complete_utf8;
      c_out : System.Address) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_request_plan_typed_value";
   procedure thinkthen_session_cancel (session : access thinkthen_session)
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_session_cancel";
   function thinkthen_session_error_message return Interfaces.C.Strings.chars_ptr
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_session_error_message";
   function thinkthen_session_error_view return access constant thinkthen_error_details
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_session_error_view";
   function thinkthen_session_finish
     (session : access thinkthen_session;
      failure_json : Interfaces.C.Strings.chars_ptr;
      failure_len : Interfaces.C.size_t) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_session_finish";
   function thinkthen_session_finish_typed (session : access constant thinkthen_session; failure : access constant thinkthen_request_reader_failure) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_session_finish_typed";
   procedure thinkthen_session_free (session : access thinkthen_session)
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_session_free";
   function thinkthen_session_new_named_with_surface
     (engine : access constant thinkthen_engine;
      request : access constant thinkthen_named_request;
      surface : thinkthen_complete_utf8;
      c_out : System.Address) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_session_new_named_with_surface";
   function thinkthen_session_new_typed_with_surface
     (engine : access constant thinkthen_engine;
      c_function : thinkthen_complete_utf8;
      arguments : access constant thinkthen_request_arguments;
      surface_name : thinkthen_complete_utf8;
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
   function thinkthen_session_push
     (session : access thinkthen_session;
      descriptor_json : Interfaces.C.Strings.chars_ptr;
      descriptor_len : Interfaces.C.size_t;
      status : access Interfaces.Unsigned_32) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_session_push";
   function thinkthen_session_push_typed
     (session : access constant thinkthen_session;
      descriptor : access constant thinkthen_request_session_descriptor;
      status : access Interfaces.Unsigned_32) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_session_push_typed";
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
      descriptor : access constant thinkthen_request_session_descriptor;
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
   procedure thinkthen_value_free (owner : access thinkthen_value)
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_value_free";
   function thinkthen_value_view (owner : access constant thinkthen_value; c_out : System.Address) return int
   with Import => True,
        Convention => C,
        External_Name => "thinkthen_value_view";
type Engine_Access is access all thinkthen_engine;
function Engine_Pointer is new Ada.Unchecked_Conversion (System.Address, Engine_Access);
function Engine_Address is new Ada.Unchecked_Conversion (Engine_Access, System.Address);
type Session_Access is access all thinkthen_session;
function Session_Pointer is new Ada.Unchecked_Conversion (System.Address, Session_Access);
function Session_Address is new Ada.Unchecked_Conversion (Session_Access, System.Address);
type Packet_Access is access all thinkthen_session_result;
function Packet_Pointer is new Ada.Unchecked_Conversion (System.Address, Packet_Access);
function Packet_Address is new Ada.Unchecked_Conversion (Packet_Access, System.Address);
end Thinkthen.Session_C;
pragma Style_Checks (On);
pragma Warnings (On, "-gnatwu");
