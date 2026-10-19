#ifndef THINKTHEN_H
#define THINKTHEN_H

/* Generated from Rust by cbindgen 0.29.4. Run sdlc/scripts/generate-c-header.py; do not edit. */

#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>

/*
 Producer ended normally; the output is ignored.
 */
#define THINKTHEN_CALL_PRODUCER_END_V1 1

/*
 Producer failed; no host exception text crosses this boundary.
 */
#define THINKTHEN_CALL_PRODUCER_FAILURE_V1 2

/*
 Producer published one canonical RequestItem JSON value.
 */
#define THINKTHEN_CALL_PRODUCER_ITEM_V1 0

/*
 A counted COBOL scalar exceeds its documented representation capacity.
 */
#define THINKTHEN_COBOL_OVERFLOW 1002

#define THINKTHEN_COMPLETE_ANNOTATED_FIELD_ARRAY_V1 4

#define THINKTHEN_COMPLETE_ANNOTATED_FIELD_BOOLEAN_V1 1

#define THINKTHEN_COMPLETE_ANNOTATED_FIELD_NULL_V1 2

#define THINKTHEN_COMPLETE_ANNOTATED_FIELD_NUMBER_V1 5

#define THINKTHEN_COMPLETE_ANNOTATED_FIELD_OBJECT_V1 6

#define THINKTHEN_COMPLETE_ANNOTATED_FIELD_STRING_V1 3

#define THINKTHEN_COMPLETE_ANNOTATION_MEMBER_ANSWER_ID_V1 1

#define THINKTHEN_COMPLETE_ANNOTATION_MEMBER_FAILURE_ID_V1 2

#define THINKTHEN_COMPLETE_ANNOTATION_VALUE_CHOICE_V1 2

#define THINKTHEN_COMPLETE_ANNOTATION_VALUE_DECISION_V1 1

#define THINKTHEN_COMPLETE_ANNOTATION_VALUE_FAILED_V1 5

#define THINKTHEN_COMPLETE_ANNOTATION_VALUE_SCORE_V1 3

#define THINKTHEN_COMPLETE_ANNOTATION_VALUE_TAGS_V1 4

#define THINKTHEN_COMPLETE_ANSWER_CHOICE_V1 2

#define THINKTHEN_COMPLETE_ANSWER_RESOLUTIONS_OBJECT_V1 2

#define THINKTHEN_COMPLETE_ANSWER_RESOLUTIONS_STRING_V1 1

#define THINKTHEN_COMPLETE_ANSWER_RESOLUTION_FAILED_V1 3

#define THINKTHEN_COMPLETE_ANSWER_RESOLUTION_RESOLVED_V1 1

#define THINKTHEN_COMPLETE_ANSWER_RESOLUTION_UNRESOLVED_V1 2

#define THINKTHEN_COMPLETE_ANSWER_SCORE_V1 4

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_10_ANY_OF_1_ITEMS_ARRAY_V1 1

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_10_ANY_OF_1_ITEMS_NULL_V1 2

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_10_ARRAY2_V1 3

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_10_ARRAY_V1 1

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_10_NULL_V1 2

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_11_ANY_OF_1_ITEMS_ARRAY_V1 1

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_11_ANY_OF_1_ITEMS_NULL_V1 2

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_11_ARRAY2_V1 3

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_11_ARRAY_V1 1

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_11_NULL_V1 2

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_13_ANY_OF_1_ITEMS_NULL_V1 2

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_13_ANY_OF_1_ITEMS_OBJECT_V1 1

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_13_ARRAY_V1 3

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_13_NULL_V1 2

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_13_OBJECT_V1 1

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_14_ANY_OF_1_ITEMS_ARRAY_V1 1

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_14_ANY_OF_1_ITEMS_NULL_V1 2

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_14_ARRAY2_V1 3

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_14_ARRAY_V1 1

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_14_NULL_V1 2

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_15_ANY_OF_1_ITEMS_NULL_V1 2

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_15_ANY_OF_1_ITEMS_OBJECT_V1 1

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_15_ARRAY_V1 3

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_15_NULL_V1 2

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_15_OBJECT_V1 1

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_2_ARRAY_V1 3

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_2_NULL_V1 2

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_2_NUMBER_V1 1

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_3_ARRAY_V1 3

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_3_NULL_V1 2

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_3_STRING_V1 1

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_4_ANY_OF_1_ITEMS_INTEGER_V1 1

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_4_ANY_OF_1_ITEMS_NULL_V1 2

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_4_ARRAY_V1 3

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_4_INTEGER_V1 1

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_4_NULL_V1 2

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_5_ARRAY_V1 3

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_5_OBJECT_V1 2

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_5_STRING_V1 1

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_6_ANY_OF_1_ITEMS_NULL_V1 2

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_6_ANY_OF_1_ITEMS_STRING_V1 1

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_6_ARRAY_V1 3

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_6_NULL_V1 2

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_6_STRING_V1 1

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_7_ANY_OF_1_ITEMS_ARRAY_V1 1

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_7_ANY_OF_1_ITEMS_NULL_V1 2

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_7_ARRAY2_V1 3

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_7_ARRAY_V1 1

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_7_NULL_V1 2

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_8_ANY_OF_1_ITEMS_NULL_V1 2

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_8_ANY_OF_1_ITEMS_NUMBER_V1 1

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_8_ARRAY_V1 3

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_8_NULL_V1 2

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_8_NUMBER_V1 1

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_9_ANY_OF_1_ITEMS_ARRAY_V1 1

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_9_ANY_OF_1_ITEMS_NULL_V1 2

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_9_ARRAY2_V1 3

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_9_ARRAY_V1 1

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_9_NULL_V1 2

#define THINKTHEN_COMPLETE_ANSWER_TAG_V1 3

#define THINKTHEN_COMPLETE_ANSWER_YES_NO_V1 1

#define THINKTHEN_COMPLETE_ATTEMPT_OUTCOME_OK_V1 1

#define THINKTHEN_COMPLETE_ATTEMPT_OUTCOME_STATUS_V1 2

#define THINKTHEN_COMPLETE_ATTEMPT_OUTCOME_TRANSPORT_V1 3

#define THINKTHEN_COMPLETE_BATCH_INTEGER_V1 1

#define THINKTHEN_COMPLETE_BATCH_SETTING_INTEGER_V1 1

#define THINKTHEN_COMPLETE_BATCH_SETTING_STRING_V1 2

#define THINKTHEN_COMPLETE_BATCH_STRING_MAX_V1 1

#define THINKTHEN_COMPLETE_BATCH_STRING_V1 2

#define THINKTHEN_COMPLETE_BOUNDARY_MODE_BOUNDARY_ONLY_V1 1

#define THINKTHEN_COMPLETE_FAILURE_CAUSE_INVALID_DISTRIBUTION_V1 5

#define THINKTHEN_COMPLETE_FAILURE_CAUSE_INVALID_PROBABILITY_V1 4

#define THINKTHEN_COMPLETE_FAILURE_CAUSE_MISSING_ANSWER_V1 1

#define THINKTHEN_COMPLETE_FAILURE_CAUSE_MISSING_PROBABILITY_V1 3

#define THINKTHEN_COMPLETE_FAILURE_CAUSE_UNEXPECTED_PROBABILITY_V1 6

#define THINKTHEN_COMPLETE_FAILURE_CAUSE_WRONG_KIND_V1 2

#define THINKTHEN_COMPLETE_FAILURE_FIELD_KIND_BACKEND_V1 1

#define THINKTHEN_COMPLETE_FAILURE_KIND_BACKEND_V1 2

#define THINKTHEN_COMPLETE_FAILURE_KIND_CANCELLED_V1 4

#define THINKTHEN_COMPLETE_FAILURE_KIND_DEADLINE_V1 5

#define THINKTHEN_COMPLETE_FAILURE_KIND_DEFECT_V1 6

#define THINKTHEN_COMPLETE_FAILURE_KIND_LOCAL_V1 3

#define THINKTHEN_COMPLETE_FAILURE_KIND_USAGE_V1 1

#define THINKTHEN_COMPLETE_IMAGE_MEDIA_IMAGE_JPEG_V1 1

#define THINKTHEN_COMPLETE_IMAGE_MEDIA_IMAGE_PNG_V1 2

#define THINKTHEN_COMPLETE_INPUT_DECLARATION_OBJECT_ROOT_V1 2

#define THINKTHEN_COMPLETE_INPUT_DECLARATION_STRING_ROOT_V1 1

#define THINKTHEN_COMPLETE_INPUT_DENIAL_ADDITIONAL_REQUEST_V1 2

#define THINKTHEN_COMPLETE_INPUT_DENIAL_INITIAL_REQUEST_V1 1

#define THINKTHEN_COMPLETE_INPUT_DENIAL_RETRY_V1 3

#define THINKTHEN_COMPLETE_INPUT_PROPERTY_TYPE_ARRAY_V1 4

#define THINKTHEN_COMPLETE_INPUT_PROPERTY_TYPE_BOOLEAN_V1 3

#define THINKTHEN_COMPLETE_INPUT_PROPERTY_TYPE_NUMBER_V1 2

#define THINKTHEN_COMPLETE_INPUT_PROPERTY_TYPE_STRING_V1 1

#define THINKTHEN_COMPLETE_JSON_ARRAY_V1 5

#define THINKTHEN_COMPLETE_JSON_BOOLEAN_V1 2

#define THINKTHEN_COMPLETE_JSON_NULL_V1 1

#define THINKTHEN_COMPLETE_JSON_NUMBER_V1 3

#define THINKTHEN_COMPLETE_JSON_OBJECT_V1 6

#define THINKTHEN_COMPLETE_JSON_STRING_V1 4

#define THINKTHEN_COMPLETE_OBJECT_TYPE_OBJECT_V1 1

#define THINKTHEN_COMPLETE_OBSERVATION_FAILURE_ID_V1 2

#define THINKTHEN_COMPLETE_OBSERVATION_OBSERVATION_ID_V1 1

#define THINKTHEN_COMPLETE_ORIGIN_CACHE_V1 2

#define THINKTHEN_COMPLETE_ORIGIN_LIVE_V1 1

#define THINKTHEN_COMPLETE_ORIGIN_MEMORY_V1 5

#define THINKTHEN_COMPLETE_ORIGIN_PROXY_V1 4

#define THINKTHEN_COMPLETE_ORIGIN_REPLAY_V1 3

#define THINKTHEN_COMPLETE_PRESENCE_MISSING_V1 0

#define THINKTHEN_COMPLETE_PRESENCE_NULL_V1 1

#define THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 2

#define THINKTHEN_COMPLETE_QUESTION_DOCUMENT_CHOOSE_V1 2

#define THINKTHEN_COMPLETE_QUESTION_DOCUMENT_DECIDE_V1 1

#define THINKTHEN_COMPLETE_QUESTION_DOCUMENT_SCORE_V1 4

#define THINKTHEN_COMPLETE_QUESTION_DOCUMENT_TAG_V1 3

#define THINKTHEN_COMPLETE_RECOGNITION_MODE_BOUNDARY_ONLY_V1 2

#define THINKTHEN_COMPLETE_RECOGNITION_MODE_WHOLE_V1 1

#define THINKTHEN_COMPLETE_RECOGNITION_ODDS_BOUNDARY_ODDS_V1 2

#define THINKTHEN_COMPLETE_RECOGNITION_ODDS_RECOGNIZE_ANSWER_V1 1

#define THINKTHEN_COMPLETE_RECOGNIZE_ENTITIES_V1 1

#define THINKTHEN_COMPLETE_RECOGNIZE_MODE_PROPOSALS_V1 2

#define THINKTHEN_COMPLETE_RELATION_DIRECTION_EITHER_V1 2

#define THINKTHEN_COMPLETE_RELATION_DIRECTION_SOURCE_TO_TARGET_V1 1

#define THINKTHEN_COMPLETE_RELATION_ENDPOINT_RELATED_ENTITY_V1 1

#define THINKTHEN_COMPLETE_RELATION_ENDPOINT_SOURCE_V1 2

#define THINKTHEN_COMPLETE_RELATION_MEMBER_ANSWER_ID_V1 1

#define THINKTHEN_COMPLETE_RELATION_MEMBER_FAILURE_ID_V1 2

#define THINKTHEN_COMPLETE_RELATION_METHOD_CHOICE_V1 2

#define THINKTHEN_COMPLETE_RELATION_METHOD_YES_NO_V1 1

#define THINKTHEN_COMPLETE_REQUEST_FUNCTION_ANNOTATE_V1 8

#define THINKTHEN_COMPLETE_REQUEST_FUNCTION_CHOOSE_V1 2

#define THINKTHEN_COMPLETE_REQUEST_FUNCTION_DECIDE_V1 1

#define THINKTHEN_COMPLETE_REQUEST_FUNCTION_FILTER_V1 5

#define THINKTHEN_COMPLETE_REQUEST_FUNCTION_FIND_V1 7

#define THINKTHEN_COMPLETE_REQUEST_FUNCTION_RANK_V1 6

#define THINKTHEN_COMPLETE_REQUEST_FUNCTION_RECOGNIZE_V1 9

#define THINKTHEN_COMPLETE_REQUEST_FUNCTION_RELATE_V1 10

#define THINKTHEN_COMPLETE_REQUEST_FUNCTION_SCORE_V1 4

#define THINKTHEN_COMPLETE_REQUEST_FUNCTION_TAG_V1 3

#define THINKTHEN_COMPLETE_SEND_DENIAL_BEFORE_ADDITIONAL_SEND_V1 2

#define THINKTHEN_COMPLETE_SEND_DENIAL_BEFORE_FIRST_SEND_V1 1

#define THINKTHEN_COMPLETE_SEND_DENIAL_BEFORE_RETRY_V1 3

#define THINKTHEN_COMPLETE_SESSION_JUDGMENT_CHOICE_V1 2

#define THINKTHEN_COMPLETE_SESSION_JUDGMENT_DECISION_V1 1

#define THINKTHEN_COMPLETE_SESSION_JUDGMENT_SCORE_V1 3

#define THINKTHEN_COMPLETE_SESSION_JUDGMENT_TAGS_V1 4

#define THINKTHEN_COMPLETE_SESSION_OBSERVATION_QUESTION_V1 1

#define THINKTHEN_COMPLETE_SESSION_OBSERVATION_ROW_V1 2

#define THINKTHEN_COMPLETE_SESSION_OBSERVED_ROW_ANNOTATED_V1 2

#define THINKTHEN_COMPLETE_SESSION_OBSERVED_ROW_FIND_V1 4

#define THINKTHEN_COMPLETE_SESSION_OBSERVED_ROW_JUDGMENT_V1 1

#define THINKTHEN_COMPLETE_SESSION_OBSERVED_ROW_RECOGNIZED_V1 3

#define THINKTHEN_COMPLETE_SESSION_OBSERVED_ROW_RELATIONS_V1 5

#define THINKTHEN_COMPLETE_SESSION_PACKET_ANNOTATE_AGGREGATE_V1 14

#define THINKTHEN_COMPLETE_SESSION_PACKET_ANNOTATE_ROW_V1 6

#define THINKTHEN_COMPLETE_SESSION_PACKET_CHOOSE_AGGREGATE_V1 8

#define THINKTHEN_COMPLETE_SESSION_PACKET_CHOOSE_ROW_V1 2

#define THINKTHEN_COMPLETE_SESSION_PACKET_DECIDE_AGGREGATE_V1 7

#define THINKTHEN_COMPLETE_SESSION_PACKET_DECIDE_ROW_V1 1

#define THINKTHEN_COMPLETE_SESSION_PACKET_FILTER_AGGREGATE_V1 11

#define THINKTHEN_COMPLETE_SESSION_PACKET_FILTER_ROW_V1 5

#define THINKTHEN_COMPLETE_SESSION_PACKET_FIND_AGGREGATE_V1 13

#define THINKTHEN_COMPLETE_SESSION_PACKET_OBSERVATION_V1 17

#define THINKTHEN_COMPLETE_SESSION_PACKET_RANK_AGGREGATE_V1 12

#define THINKTHEN_COMPLETE_SESSION_PACKET_RECOGNIZE_AGGREGATE_V1 15

#define THINKTHEN_COMPLETE_SESSION_PACKET_RELATE_AGGREGATE_V1 16

#define THINKTHEN_COMPLETE_SESSION_PACKET_SCORE_AGGREGATE_V1 10

#define THINKTHEN_COMPLETE_SESSION_PACKET_SCORE_ROW_V1 4

#define THINKTHEN_COMPLETE_SESSION_PACKET_TAG_AGGREGATE_V1 9

#define THINKTHEN_COMPLETE_SESSION_PACKET_TAG_ROW_V1 3

#define THINKTHEN_COMPLETE_SESSION_PACKET_TERMINAL_V1 18

#define THINKTHEN_COMPLETE_SESSION_PROBABILITIES_NAMED_V1 2

#define THINKTHEN_COMPLETE_SESSION_PROBABILITIES_YES_NO_V1 1

#define THINKTHEN_COMPLETE_SETTLED_ANSWER_ANNOTATE_V1 10

#define THINKTHEN_COMPLETE_SETTLED_ANSWER_CHOOSE_V1 2

#define THINKTHEN_COMPLETE_SETTLED_ANSWER_DECIDE_V1 1

#define THINKTHEN_COMPLETE_SETTLED_ANSWER_FILTER_V1 5

#define THINKTHEN_COMPLETE_SETTLED_ANSWER_FIND_V1 7

#define THINKTHEN_COMPLETE_SETTLED_ANSWER_RANK_V1 6

#define THINKTHEN_COMPLETE_SETTLED_ANSWER_RECOGNIZE_V1 8

#define THINKTHEN_COMPLETE_SETTLED_ANSWER_RELATE_V1 9

#define THINKTHEN_COMPLETE_SETTLED_ANSWER_SCORE_V1 4

#define THINKTHEN_COMPLETE_SETTLED_ANSWER_TAG_V1 3

#define THINKTHEN_COMPLETE_SETTLED_JUDGMENT_ARRAY_V1 5

#define THINKTHEN_COMPLETE_SETTLED_JUDGMENT_BOOLEAN_V1 1

#define THINKTHEN_COMPLETE_SETTLED_JUDGMENT_NULL_V1 2

#define THINKTHEN_COMPLETE_SETTLED_JUDGMENT_NUMBER_V1 4

#define THINKTHEN_COMPLETE_SETTLED_JUDGMENT_STRING_V1 3

#define THINKTHEN_COMPLETE_SETTLED_RECOGNITION_ENTITIES_V1 1

#define THINKTHEN_COMPLETE_SETTLED_RECOGNITION_MODE_PROPOSALS_V1 2

#define THINKTHEN_COMPLETE_STOP_CAUSE_BACKEND_V1 8

#define THINKTHEN_COMPLETE_STOP_CAUSE_CANCELLED_V1 9

#define THINKTHEN_COMPLETE_STOP_CAUSE_DEADLINE_V1 10

#define THINKTHEN_COMPLETE_STOP_CAUSE_DEFECT_V1 11

#define THINKTHEN_COMPLETE_STOP_CAUSE_LOCAL_V1 2

#define THINKTHEN_COMPLETE_STOP_CAUSE_NO_KEY_V1 3

#define THINKTHEN_COMPLETE_STOP_CAUSE_REPLY_V1 7

#define THINKTHEN_COMPLETE_STOP_CAUSE_STATUS_V1 5

#define THINKTHEN_COMPLETE_STOP_CAUSE_TOO_LARGE_V1 6

#define THINKTHEN_COMPLETE_STOP_CAUSE_TRANSPORT_V1 4

#define THINKTHEN_COMPLETE_STOP_CAUSE_USAGE_V1 1

#define THINKTHEN_COMPLETE_STRING_TYPE_STRING_V1 1

#define THINKTHEN_COMPLETE_THRESHOLD_NUMBER_V1 1

#define THINKTHEN_COMPLETE_THRESHOLD_STRING_V1 2

#define THINKTHEN_COMPLETE_USAGE_PERSISTENCE_DISABLED_V1 1

#define THINKTHEN_COMPLETE_USAGE_PERSISTENCE_FAILED_V1 4

#define THINKTHEN_COMPLETE_USAGE_PERSISTENCE_PENDING_V1 2

#define THINKTHEN_COMPLETE_USAGE_PERSISTENCE_WRITTEN_V1 3

#define THINKTHEN_COMPLETE_VALUE_ARRAY_V1 4

#define THINKTHEN_COMPLETE_VALUE_BOOLEAN_V1 1

#define THINKTHEN_COMPLETE_VALUE_NULL_V1 2

#define THINKTHEN_COMPLETE_VALUE_NUMBER_V1 5

#define THINKTHEN_COMPLETE_VALUE_STRING_V1 3

#define THINKTHEN_COMPLETE_VERB_RECOGNIZE_V1 1

#define THINKTHEN_COMPLETE_VERSION_THINKTHEN_RESULT_2_V1 1

/*
 the wire failed or refused
 */
#define THINKTHEN_EBACKEND 2

/*
 the token fired; sent requests finished and no rows came
 */
#define THINKTHEN_ECANCELLED 5

/*
 the caller's own budget ran out; no rows came
 */
#define THINKTHEN_EDEADLINE 3

/*
 the engine broke its own contract
 */
#define THINKTHEN_EDEFECT 6

/*
 a named local file, cache, or recording failed
 */
#define THINKTHEN_ELOCAL 4

/*
 the arguments/input broke the grammar; the rejected stage was not sent
 */
#define THINKTHEN_EUSAGE 1

/*
 Exact C interface compatibility number. Compare with thinkthen_version at load.
 Change this number whenever the C contract becomes incompatible.
 */
#define THINKTHEN_INTERFACE_NUMBER 1

/*
 C value `THINKTHEN_NO`.
 */
#define THINKTHEN_NO 0

/*
 _opts budgets are exact int64_t milliseconds from the call.
 THINKTHEN_NO_DEADLINE (-1) sets none; zero returns EDEADLINE before sending.
 Other negatives or values above 4294967295000 ms return EUSAGE before sending.
 Positive budgets stop within one tick. Clamp elapsed computed budgets at
 zero so an expired deadline never becomes the no-deadline sentinel.
 */
#define THINKTHEN_NO_DEADLINE -1ll

/*
 Zero means success; codes 1..6 identify failures, never answers.
 On failure every output stays unchanged; error_code, error_message,
 error_retryable and error_facts_json describe the calling thread's failure.
 */
#define THINKTHEN_OK 0

#define THINKTHEN_REQUEST_ANNOTATE_QUESTION_CHOOSE_V1 2

#define THINKTHEN_REQUEST_ANNOTATE_QUESTION_DECIDE_V1 1

#define THINKTHEN_REQUEST_ANNOTATE_QUESTION_SCORE_V1 4

#define THINKTHEN_REQUEST_ANNOTATE_QUESTION_TAG_V1 3

#define THINKTHEN_REQUEST_BATCH_TEXT_V1 2

#define THINKTHEN_REQUEST_BATCH_UNSIGNED_VALUE_V1 1

#define THINKTHEN_REQUEST_CACHE_DOCUMENT_DISABLED_CACHE_V1 2

#define THINKTHEN_REQUEST_CACHE_DOCUMENT_TEXT_V1 1

#define THINKTHEN_REQUEST_CALL_THRESHOLD_NUMBER_V1 1

#define THINKTHEN_REQUEST_CALL_THRESHOLD_TEXT_V1 2

#define THINKTHEN_REQUEST_CONTEXT_SCHEMA_JSON_VALUE_MAP_V1 2

#define THINKTHEN_REQUEST_CONTEXT_SCHEMA_TEXT_V1 1

#define THINKTHEN_REQUEST_CRITERION_JSON_VALUE_LIST_V1 3

#define THINKTHEN_REQUEST_CRITERION_JSON_VALUE_MAP_V1 2

#define THINKTHEN_REQUEST_CRITERION_NULL_V1 4

#define THINKTHEN_REQUEST_CRITERION_TEXT_V1 1

#define THINKTHEN_REQUEST_CUT_NUMBER_V1 1

#define THINKTHEN_REQUEST_CUT_TEXT_V1 2

#define THINKTHEN_REQUEST_DEFINITION_ANNOTATE_DEFINITION_V1 8

#define THINKTHEN_REQUEST_DEFINITION_AUTHORED_CHOOSE_V1 2

#define THINKTHEN_REQUEST_DEFINITION_AUTHORED_DECIDE_V1 1

#define THINKTHEN_REQUEST_DEFINITION_AUTHORED_FIND_V1 6

#define THINKTHEN_REQUEST_DEFINITION_AUTHORED_RELATE_V1 5

#define THINKTHEN_REQUEST_DEFINITION_AUTHORED_SCORE_V1 4

#define THINKTHEN_REQUEST_DEFINITION_AUTHORED_TAG_V1 3

#define THINKTHEN_REQUEST_DEFINITION_RECOGNIZE_DEFINITION_V1 7

#define THINKTHEN_REQUEST_DESCRIPTION_JSON_VALUE_LIST_V1 3

#define THINKTHEN_REQUEST_DESCRIPTION_JSON_VALUE_MAP_V1 2

#define THINKTHEN_REQUEST_DESCRIPTION_NULL_V1 4

#define THINKTHEN_REQUEST_DESCRIPTION_TEXT_V1 1

#define THINKTHEN_REQUEST_FRAMING_LITERAL_CSV_V1 4

#define THINKTHEN_REQUEST_FRAMING_LITERAL_DOCUMENT_V1 1

#define THINKTHEN_REQUEST_FRAMING_LITERAL_JSONL_V1 3

#define THINKTHEN_REQUEST_FRAMING_LITERAL_LINES_V1 2

#define THINKTHEN_REQUEST_FRAMING_LITERAL_TSV_V1 5

#define THINKTHEN_REQUEST_IMAGE_BYTES_V1 2

#define THINKTHEN_REQUEST_IMAGE_FILE_V1 1

#define THINKTHEN_REQUEST_IMAGE_MEDIA_LITERAL_IMAGE_JPEG_V1 1

#define THINKTHEN_REQUEST_IMAGE_MEDIA_LITERAL_IMAGE_PNG_V1 2

#define THINKTHEN_REQUEST_INPUT_DECLARATION_OBJECT_V1 2

#define THINKTHEN_REQUEST_INPUT_DECLARATION_STRING_V1 1

#define THINKTHEN_REQUEST_INPUT_ENTITIES_V1 5

#define THINKTHEN_REQUEST_INPUT_FEED_V1 7

#define THINKTHEN_REQUEST_INPUT_JSON_V1 2

#define THINKTHEN_REQUEST_INPUT_PROPERTY_ARRAY_V1 4

#define THINKTHEN_REQUEST_INPUT_PROPERTY_BOOLEAN_V1 3

#define THINKTHEN_REQUEST_INPUT_PROPERTY_NUMBER_V1 2

#define THINKTHEN_REQUEST_INPUT_PROPERTY_STRING_V1 1

#define THINKTHEN_REQUEST_INPUT_RECORDS_V1 3

#define THINKTHEN_REQUEST_INPUT_SOURCE_V1 6

#define THINKTHEN_REQUEST_INPUT_TEXT_V1 1

#define THINKTHEN_REQUEST_INPUT_UNITS_V1 4

#define THINKTHEN_REQUEST_LABELS_DESCRIPTION_MAP_V1 2

#define THINKTHEN_REQUEST_LABELS_NAME_LIST_V1 1

#define THINKTHEN_REQUEST_LEVELS_CRITERION_MAP_V1 2

#define THINKTHEN_REQUEST_LEVELS_NAME_LIST_V1 1

#define THINKTHEN_REQUEST_OPTIONAL_COUNT_NULL_V1 2

#define THINKTHEN_REQUEST_OPTIONAL_COUNT_UNSIGNED_VALUE_V1 1

#define THINKTHEN_REQUEST_OPTIONAL_TOTAL_NULL_V1 2

#define THINKTHEN_REQUEST_OPTIONAL_TOTAL_UNSIGNED_VALUE_V1 1

#define THINKTHEN_REQUEST_OPTIONS_DESCRIPTION_MAP_V1 2

#define THINKTHEN_REQUEST_OPTIONS_NAME_LIST_V1 1

#define THINKTHEN_REQUEST_ORIGINAL_JSON_V1 2

#define THINKTHEN_REQUEST_ORIGINAL_TEXT_V1 1

#define THINKTHEN_REQUEST_POINTERS_TEXT_LIST_V1 2

#define THINKTHEN_REQUEST_POINTERS_TEXT_V1 1

#define THINKTHEN_REQUEST_QUESTION_BATCH_INTEGER_V1 2

#define THINKTHEN_REQUEST_QUESTION_BATCH_LITERAL_MAX_V1 1

#define THINKTHEN_REQUEST_QUESTION_SELECTOR_DEFINITION_V1 2

#define THINKTHEN_REQUEST_QUESTION_SELECTOR_FILE_V1 3

#define THINKTHEN_REQUEST_QUESTION_SELECTOR_NAME_V1 4

#define THINKTHEN_REQUEST_QUESTION_SELECTOR_REFERENCE_V1 5

#define THINKTHEN_REQUEST_QUESTION_SELECTOR_TEXT_V1 1

#define THINKTHEN_REQUEST_QUESTION_TEXT_JSON_VALUE_LIST_V1 3

#define THINKTHEN_REQUEST_QUESTION_TEXT_JSON_VALUE_MAP_V1 2

#define THINKTHEN_REQUEST_QUESTION_TEXT_TEXT_V1 1

#define THINKTHEN_REQUEST_READER_FAILURE_INVALID_INPUT_V1 4

#define THINKTHEN_REQUEST_READER_FAILURE_IO_V1 2

#define THINKTHEN_REQUEST_READER_FAILURE_PRODUCER_V1 1

#define THINKTHEN_REQUEST_READER_FAILURE_UTF_8_V1 3

#define THINKTHEN_REQUEST_READER_MEDIA_LITERAL_IMAGE_V1 2

#define THINKTHEN_REQUEST_READER_MEDIA_LITERAL_TEXT_V1 1

#define THINKTHEN_REQUEST_RECOGNITION_EXAMPLE_EXAMPLE_TEXT_V1 2

#define THINKTHEN_REQUEST_RECOGNITION_EXAMPLE_TEXT_V1 1

#define THINKTHEN_REQUEST_RECOGNITION_MODE_LITERAL_BOUNDARY_ONLY_V1 2

#define THINKTHEN_REQUEST_RECOGNITION_MODE_LITERAL_WHOLE_V1 1

#define THINKTHEN_REQUEST_SOURCE_UNIT_LITERAL_FILE_V1 3

#define THINKTHEN_REQUEST_SOURCE_UNIT_LITERAL_LINE_V1 1

#define THINKTHEN_REQUEST_SOURCE_UNIT_LITERAL_WINDOW_V1 2

#define THINKTHEN_REQUEST_THRESHOLD_NUMBER_V1 1

#define THINKTHEN_REQUEST_THRESHOLD_TEXT_V1 2

/*
 Push transferred one descriptor to the session.
 */
#define THINKTHEN_SESSION_ACCEPTED_V1 0

/*
 Intake closed; stop advancing the producer.
 */
#define THINKTHEN_SESSION_CLOSED_V1 2

/*
 Terminal was read and no more output will arrive.
 */
#define THINKTHEN_SESSION_END_V1 2

/*
 Push retained nothing; retry the same descriptor.
 */
#define THINKTHEN_SESSION_FULL_V1 1

/*
 Work has not settled and no packet is ready.
 */
#define THINKTHEN_SESSION_PENDING_V1 1

/*
 Intake was cancelled; settlement can still be pending.
 */
#define THINKTHEN_SESSION_READY_CANCELLED_V1 4

/*
 Caller-supplied input has closed.
 */
#define THINKTHEN_SESSION_READY_CLOSED_V1 2

/*
 Every output packet has been consumed.
 */
#define THINKTHEN_SESSION_READY_END_V1 3

/*
 Caller-supplied input has room.
 */
#define THINKTHEN_SESSION_READY_INPUT_V1 1

/*
 A result or terminal packet is ready; no packet has been consumed.
 */
#define THINKTHEN_SESSION_READY_RESULT_V1 0

/*
 The caller-given timeout passed.
 */
#define THINKTHEN_SESSION_READY_TIMEOUT_V1 5

/*
 Read transferred one independent packet owner.
 */
#define THINKTHEN_SESSION_RESULT_V1 0

/*
 Wait for result, input room/closure, end or cancel.
 */
#define THINKTHEN_SESSION_WAIT_INPUT_V1 1

/*
 Wait for final output or cancellation requested since this reader armed.
 */
#define THINKTHEN_SESSION_WAIT_INTERRUPTIBLE_SETTLEMENT_V1 4

/*
 Wait only for results, end or cancel; input room cannot wake this wait.
 */
#define THINKTHEN_SESSION_WAIT_RESULT_V1 0

/*
 Arm one exclusive settlement reader and check readiness immediately; timeout must be zero.
 */
#define THINKTHEN_SESSION_WAIT_SETTLEMENT_ARM_V1 3

/*
 Drain output after cancel without waking repeatedly for cancellation.
 */
#define THINKTHEN_SESSION_WAIT_SETTLEMENT_V1 2

/*
 C value `THINKTHEN_UNSURE`.
 */
#define THINKTHEN_UNSURE 2

/*
 Version 0.2.0, the version of the library this header ships with.
 Version 0.1.0 is the first release.
 */
#define THINKTHEN_VERSION_MAJOR 0

/*
 C value `THINKTHEN_VERSION_MINOR`.
 */
#define THINKTHEN_VERSION_MINOR 2

/*
 C value `THINKTHEN_VERSION_PATCH`.
 */
#define THINKTHEN_VERSION_PATCH 0

/*
 The three answers a yes-or-no question gives. UNSURE is the machine word;
 the specification says "not sure" in prose.
 */
#define THINKTHEN_YES 1

/*
 Admitted native grammar values; descriptor storage remains uint32_t.
 */
typedef enum thinkthen_input_declaration_kind_v1 {
  /*
   C grammar value `THINKTHEN_DECLARATION_ABSENT_V1`.
   */
  THINKTHEN_DECLARATION_ABSENT_V1 = 0,
  /*
   C grammar value `THINKTHEN_DECLARATION_STRING_V1`.
   */
  THINKTHEN_DECLARATION_STRING_V1 = 1,
  /*
   C grammar value `THINKTHEN_DECLARATION_OBJECT_V1`.
   */
  THINKTHEN_DECLARATION_OBJECT_V1 = 2,
} thinkthen_input_declaration_kind_v1;

/*
 Admitted native grammar values; descriptor storage remains uint32_t.
 */
typedef enum thinkthen_input_property_kind_v1 {
  /*
   C grammar value `THINKTHEN_PROPERTY_STRING_V1`.
   */
  THINKTHEN_PROPERTY_STRING_V1 = 1,
  /*
   C grammar value `THINKTHEN_PROPERTY_NUMBER_V1`.
   */
  THINKTHEN_PROPERTY_NUMBER_V1 = 2,
  /*
   C grammar value `THINKTHEN_PROPERTY_BOOLEAN_V1`.
   */
  THINKTHEN_PROPERTY_BOOLEAN_V1 = 3,
  /*
   C grammar value `THINKTHEN_PROPERTY_STRING_LIST_V1`.
   */
  THINKTHEN_PROPERTY_STRING_LIST_V1 = 4,
} thinkthen_input_property_kind_v1;

/*
 Admitted native grammar values; descriptor storage remains uint32_t.
 */
typedef enum thinkthen_question_loader_role_v1 {
  /*
   C grammar value `THINKTHEN_LOAD_ATOMIC_V1`.
   */
  THINKTHEN_LOAD_ATOMIC_V1 = 1,
  /*
   C grammar value `THINKTHEN_LOAD_SET_V1`.
   */
  THINKTHEN_LOAD_SET_V1 = 2,
  /*
   C grammar value `THINKTHEN_LOAD_DYNAMIC_CHOOSE_V1`.
   */
  THINKTHEN_LOAD_DYNAMIC_CHOOSE_V1 = 3,
  /*
   C grammar value `THINKTHEN_LOAD_RECOGNIZE_V1`.
   */
  THINKTHEN_LOAD_RECOGNIZE_V1 = 4,
  /*
   C grammar value `THINKTHEN_LOAD_RELATE_V1`.
   */
  THINKTHEN_LOAD_RELATE_V1 = 5,
  /*
   C grammar value `THINKTHEN_LOAD_RANK_V1`.
   */
  THINKTHEN_LOAD_RANK_V1 = 6,
  /*
   C grammar value `THINKTHEN_LOAD_RANK_SET_V1`.
   */
  THINKTHEN_LOAD_RANK_SET_V1 = 7,
  /*
   C grammar value `THINKTHEN_LOAD_FIND_V1`.
   */
  THINKTHEN_LOAD_FIND_V1 = 8,
} thinkthen_question_loader_role_v1;

/*
 A caller-owned cancellation token. Keep alive until the call and all cancels return.
 */
typedef struct thinkthen_call_cancel thinkthen_call_cancel;

/*
 Independent safe failure details. Send or read on any thread; free after all readers.
 */
typedef struct thinkthen_call_error thinkthen_call_error;

/*
 One independent complete Call in the existing canonical JSON grammar.
 */
typedef struct thinkthen_call_result thinkthen_call_result;

/*
 The current C interface owns engines, parsed questions, sessions and packet views.
 Free each owner after its last use. Request preview returns an owned string
 released with thinkthen_free_string. Diagnostics borrow per-thread storage.
 Session cancellation uses thinkthen_session_cancel.
 Opaque engine owner. Free after every concurrent call finishes.
 */
typedef struct thinkthen_engine thinkthen_engine;

/*
 Immutable question metadata, independent of every constructor buffer.
 */
typedef struct thinkthen_question thinkthen_question;

/*
 Independent session owner. Free once after concurrent operations return.
 */
typedef struct thinkthen_session thinkthen_session;

/*
 Independent packet owner. It remains live after its session and engine are freed.
 */
typedef struct thinkthen_session_result thinkthen_session_result;

/*
 Independent immutable canonical value owner. Free after every borrower finishes.
 */
typedef struct thinkthen_value thinkthen_value;

/*
 C descriptor or borrowed view `thinkthen_complete_utf8_v1`.
 */
typedef struct thinkthen_complete_utf8_v1 {
  /*
   C field `data`.
   */
  const char *data;
  /*
   C field `len`.
   */
  size_t len;
} thinkthen_complete_utf8_v1;

typedef struct thinkthen_complete_usage_persistence_v1 {
  uint32_t kind;
} thinkthen_complete_usage_persistence_v1;

typedef struct thinkthen_request_text_v1 {
  struct thinkthen_complete_utf8_v1 text;
} thinkthen_request_text_v1;

typedef struct thinkthen_request_unsigned_v1 {
  uint64_t value;
} thinkthen_request_unsigned_v1;

typedef union thinkthen_request_batch_data_v1 {
  const struct thinkthen_request_unsigned_v1 *unsigned_value;
  const struct thinkthen_request_text_v1 *text;
} thinkthen_request_batch_data_v1;

typedef struct thinkthen_request_batch_v1 {
  uint32_t kind;
  union thinkthen_request_batch_data_v1 data;
} thinkthen_request_batch_v1;

typedef struct thinkthen_request_disabled_cache_v1 {
  uint32_t reserved;
} thinkthen_request_disabled_cache_v1;

typedef union thinkthen_request_cache_document_data_v1 {
  const struct thinkthen_request_text_v1 *text;
  const struct thinkthen_request_disabled_cache_v1 *disabled_cache;
} thinkthen_request_cache_document_data_v1;

typedef struct thinkthen_request_cache_document_v1 {
  uint32_t kind;
  union thinkthen_request_cache_document_data_v1 data;
} thinkthen_request_cache_document_v1;

typedef struct thinkthen_request_null_v1 {
  uint32_t reserved;
} thinkthen_request_null_v1;

typedef union thinkthen_request_optional_total_data_v1 {
  const struct thinkthen_request_unsigned_v1 *unsigned_value;
  const struct thinkthen_request_null_v1 *null;
} thinkthen_request_optional_total_data_v1;

typedef struct thinkthen_request_optional_total_v1 {
  uint32_t kind;
  union thinkthen_request_optional_total_data_v1 data;
} thinkthen_request_optional_total_v1;

typedef union thinkthen_request_optional_count_data_v1 {
  const struct thinkthen_request_unsigned_v1 *unsigned_value;
  const struct thinkthen_request_null_v1 *null;
} thinkthen_request_optional_count_data_v1;

typedef struct thinkthen_request_optional_count_v1 {
  uint32_t kind;
  union thinkthen_request_optional_count_data_v1 data;
} thinkthen_request_optional_count_v1;

/*
 C descriptor or borrowed view `thinkthen_complete_json_array_v1`.
 */
typedef struct thinkthen_complete_json_array_v1 {
  /*
   C field `data`.
   */
  const struct thinkthen_complete_json_v1 *const *data;
  /*
   C field `len`.
   */
  size_t len;
} thinkthen_complete_json_array_v1;

/*
 C descriptor or borrowed view `thinkthen_complete_json_entry_v1`.
 */
typedef struct thinkthen_complete_json_entry_v1 {
  /*
   C field `name`.
   */
  struct thinkthen_complete_utf8_v1 name;
  /*
   C field `value`.
   */
  const struct thinkthen_complete_json_v1 *value;
} thinkthen_complete_json_entry_v1;

/*
 C descriptor or borrowed view `thinkthen_complete_json_object_v1`.
 */
typedef struct thinkthen_complete_json_object_v1 {
  /*
   C field `data`.
   */
  const struct thinkthen_complete_json_entry_v1 *data;
  /*
   C field `len`.
   */
  size_t len;
} thinkthen_complete_json_object_v1;

/*
 C descriptor or borrowed view `thinkthen_complete_json_data_v1`.
 */
typedef union thinkthen_complete_json_data_v1 {
  /*
   C field `boolean`.
   */
  uint32_t boolean;
  /*
   C field `number`.
   */
  struct thinkthen_complete_utf8_v1 number;
  /*
   C field `string`.
   */
  struct thinkthen_complete_utf8_v1 string;
  /*
   C field `array`.
   */
  struct thinkthen_complete_json_array_v1 array;
  /*
   C field `object`.
   */
  struct thinkthen_complete_json_object_v1 object;
} thinkthen_complete_json_data_v1;

/*
 C descriptor or borrowed view `thinkthen_complete_json_v1`.
 */
typedef struct thinkthen_complete_json_v1 {
  /*
   C field `kind`.
   */
  uint32_t kind;
  /*
   C field `data`.
   */
  union thinkthen_complete_json_data_v1 data;
} thinkthen_complete_json_v1;

typedef struct thinkthen_request_json_value_v1 {
  const struct thinkthen_complete_json_v1 *value;
} thinkthen_request_json_value_v1;

typedef struct thinkthen_request_boolean_v1 {
  uint32_t value;
} thinkthen_request_boolean_v1;

typedef struct thinkthen_request_engine_settings_v1 {
  const struct thinkthen_request_text_v1 *m_backend;
  const struct thinkthen_request_text_v1 *m_base_url;
  const struct thinkthen_request_batch_v1 *m_batch;
  const struct thinkthen_request_cache_document_v1 *m_cache;
  const struct thinkthen_request_optional_total_v1 *m_max_estimated_input_tokens_total;
  const struct thinkthen_request_unsigned_v1 *m_max_request_bytes;
  const struct thinkthen_request_optional_count_v1 *m_max_requests;
  const struct thinkthen_request_optional_total_v1 *m_max_requests_total;
  const struct thinkthen_request_unsigned_v1 *m_max_retries;
  const struct thinkthen_request_text_v1 *m_model;
  const struct thinkthen_request_text_v1 *m_profile;
  const struct thinkthen_request_json_value_v1 *m_proxy;
  const struct thinkthen_request_text_v1 *m_record;
  const struct thinkthen_request_boolean_v1 *m_refresh_cache;
  const struct thinkthen_request_text_v1 *m_replay;
  const struct thinkthen_request_unsigned_v1 *m_throttle;
  const struct thinkthen_request_unsigned_v1 *m_timeout;
  const struct thinkthen_request_text_v1 *m_usd_per_million_input;
  const struct thinkthen_request_text_v1 *m_usd_per_million_output;
} thinkthen_request_engine_settings_v1;

typedef struct thinkthen_complete_attempt_outcome_v1 {
  uint32_t kind;
} thinkthen_complete_attempt_outcome_v1;

typedef struct thinkthen_complete_utf8_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_utf8_v1 value;
} thinkthen_complete_utf8_presence_v1;

typedef struct thinkthen_complete_sdk_request_id_v1 {
  struct thinkthen_complete_utf8_v1 value;
} thinkthen_complete_sdk_request_id_v1;

typedef struct thinkthen_complete_u64_presence_v1 {
  uint32_t presence;
  uint64_t value;
} thinkthen_complete_u64_presence_v1;

/*
 C descriptor or borrowed view `thinkthen_complete_extension_v1`.
 */
typedef struct thinkthen_complete_extension_v1 {
  /*
   C field `name`.
   */
  struct thinkthen_complete_utf8_v1 name;
  /*
   C field `json`.
   */
  struct thinkthen_complete_utf8_v1 json;
  /*
   C field `value`.
   */
  const struct thinkthen_complete_json_v1 *value;
} thinkthen_complete_extension_v1;

/*
 C descriptor or borrowed view `thinkthen_complete_extensions_v1`.
 */
typedef struct thinkthen_complete_extensions_v1 {
  /*
   C field `data`.
   */
  const struct thinkthen_complete_extension_v1 *data;
  /*
   C field `len`.
   */
  size_t len;
} thinkthen_complete_extensions_v1;

typedef struct thinkthen_complete_attempt_v1 {
  uint64_t ordinal;
  const struct thinkthen_complete_attempt_outcome_v1 *outcome;
  struct thinkthen_complete_utf8_presence_v1 request_id;
  struct thinkthen_complete_utf8_v1 request_sha256;
  const struct thinkthen_complete_sdk_request_id_v1 *sdk_request_id;
  struct thinkthen_complete_u64_presence_v1 server_ms;
  struct thinkthen_complete_u64_presence_v1 status;
  uint64_t wall_ms;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_attempt_v1;

typedef struct thinkthen_complete_attempt_list_v1 {
  const struct thinkthen_complete_attempt_v1 *const *data;
  size_t len;
} thinkthen_complete_attempt_list_v1;

typedef struct thinkthen_complete_attempt_list_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_attempt_list_v1 value;
} thinkthen_complete_attempt_list_presence_v1;

typedef struct thinkthen_complete_call_id_v1 {
  struct thinkthen_complete_utf8_v1 value;
} thinkthen_complete_call_id_v1;

typedef struct thinkthen_complete_call_id_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_call_id_v1 *value;
} thinkthen_complete_call_id_presence_v1;

typedef struct thinkthen_complete_u32_presence_v1 {
  uint32_t presence;
  uint32_t value;
} thinkthen_complete_u32_presence_v1;

typedef struct thinkthen_complete_persistence_observation_v1 {
  struct thinkthen_complete_utf8_presence_v1 advice;
  struct thinkthen_complete_utf8_v1 observed_at;
  const struct thinkthen_complete_usage_persistence_v1 *state;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_persistence_observation_v1;

typedef struct thinkthen_complete_persistence_observation_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_persistence_observation_v1 *value;
} thinkthen_complete_persistence_observation_presence_v1;

typedef struct thinkthen_complete_immediate_facts_v1 {
  struct thinkthen_complete_attempt_list_presence_v1 attempts;
  uint64_t cache_answers;
  struct thinkthen_complete_call_id_presence_v1 call_id;
  struct thinkthen_complete_utf8_presence_v1 estimated_cost_usd;
  struct thinkthen_complete_u32_presence_v1 held_model_mismatch;
  struct thinkthen_complete_u64_presence_v1 input_tokens;
  uint64_t largest_request_bytes;
  struct thinkthen_complete_u64_presence_v1 largest_request_estimated_input_tokens;
  struct thinkthen_complete_utf8_presence_v1 model;
  struct thinkthen_complete_u64_presence_v1 output_tokens;
  uint64_t records;
  uint64_t requests_sent;
  double seconds;
  struct thinkthen_complete_utf8_v1 token_estimate_method;
  struct thinkthen_complete_persistence_observation_presence_v1 usage_persistence;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_immediate_facts_v1;

/*
 C descriptor or borrowed view `thinkthen_error_view_v1`.
 */
typedef struct thinkthen_error_view_v1 {
  /*
   C field `code`.
   */
  int code;
  /*
   C field `retryable`.
   */
  int retryable;
  /*
   C field `message`.
   */
  struct thinkthen_complete_utf8_v1 message;
  /*
   C field `facts`.
   */
  const struct thinkthen_complete_immediate_facts_v1 *facts;
} thinkthen_error_view_v1;

/*
 C descriptor or borrowed view `StringV1`.
 */
typedef struct thinkthen_string_v1 {
  /*
   C field `data`.
   */
  const char *data;
  /*
   C field `len`.
   */
  size_t len;
} thinkthen_string_v1;

/*
 C descriptor or borrowed view `OptionalStringV1`.
 */
typedef struct thinkthen_optional_string_v1 {
  /*
   C field `present`.
   */
  int present;
  /*
   C field `value`.
   */
  struct thinkthen_string_v1 value;
} thinkthen_optional_string_v1;

/*
 C descriptor or borrowed view `OptionalU64V1`.
 */
typedef struct thinkthen_optional_u64_v1 {
  /*
   C field `present`.
   */
  int present;
  /*
   C field `value`.
   */
  uint64_t value;
} thinkthen_optional_u64_v1;

/*
 C descriptor or borrowed view `InputPropertyV1`.
 */
typedef struct thinkthen_input_property_v1 {
  /*
   C field `name`.
   */
  struct thinkthen_string_v1 name;
  /*
   C field `kind`.
   */
  uint32_t kind;
} thinkthen_input_property_v1;

/*
 C descriptor or borrowed view `InputPropertiesV1`.
 */
typedef struct thinkthen_input_properties_v1 {
  /*
   C field `data`.
   */
  const struct thinkthen_input_property_v1 *data;
  /*
   C field `len`.
   */
  size_t len;
} thinkthen_input_properties_v1;

/*
 C descriptor or borrowed view `StringsV1`.
 */
typedef struct thinkthen_strings_v1 {
  /*
   C field `data`.
   */
  const struct thinkthen_string_v1 *data;
  /*
   C field `len`.
   */
  size_t len;
} thinkthen_strings_v1;

/*
 C descriptor or borrowed view `InputDeclarationV1`.
 */
typedef struct thinkthen_input_declaration_v1 {
  /*
   C field `kind`.
   */
  uint32_t kind;
  /*
   C field `properties`.
   */
  struct thinkthen_input_properties_v1 properties;
  /*
   C field `required`.
   */
  struct thinkthen_strings_v1 required;
} thinkthen_input_declaration_v1;

/*
 C descriptor or borrowed view `QuestionAuthorV1`.
 */
typedef struct thinkthen_question_author_v1 {
  /*
   C field `name`.
   */
  struct thinkthen_optional_string_v1 name;
  /*
   C field `wording_version`.
   */
  struct thinkthen_optional_u64_v1 wording_version;
  /*
   C field `item_schema`.
   */
  struct thinkthen_input_declaration_v1 item_schema;
  /*
   C field `context_schema`.
   */
  struct thinkthen_input_declaration_v1 context_schema;
} thinkthen_question_author_v1;

/*
 Existing canonical components for one named request. Every view is counted
 UTF-8, without a trailing NUL. No bytes or foreign pointers survive return.
 */
typedef struct thinkthen_named_request_v1 {
  /*
   Exact lower-case public function name.
   */
  struct thinkthen_complete_utf8_v1 function;
  /*
   Existing canonical question selector JSON.
   */
  struct thinkthen_complete_utf8_v1 question_json;
  /*
   Existing canonical input selector JSON.
   */
  struct thinkthen_complete_utf8_v1 input_json;
  /*
   Existing canonical options JSON; pass {} for omitted controls.
   */
  struct thinkthen_complete_utf8_v1 options_json;
} thinkthen_named_request_v1;

typedef struct thinkthen_request_literal_image_jpeg_v1 {
  uint32_t reserved;
} thinkthen_request_literal_image_jpeg_v1;

typedef struct thinkthen_request_literal_image_png_v1 {
  uint32_t reserved;
} thinkthen_request_literal_image_png_v1;

typedef union thinkthen_request_image_media_data_v1 {
  const struct thinkthen_request_literal_image_jpeg_v1 *literal_image_jpeg;
  const struct thinkthen_request_literal_image_png_v1 *literal_image_png;
} thinkthen_request_image_media_data_v1;

typedef struct thinkthen_request_image_media_v1 {
  uint32_t kind;
  union thinkthen_request_image_media_data_v1 data;
} thinkthen_request_image_media_v1;

typedef struct thinkthen_request_image_file_v1 {
  const struct thinkthen_request_image_media_v1 *m_media;
  const struct thinkthen_request_text_v1 *m_path;
} thinkthen_request_image_file_v1;

typedef struct thinkthen_request_image_bytes_v1 {
  const struct thinkthen_request_text_v1 *m_bytes;
  const struct thinkthen_request_image_media_v1 *m_media;
} thinkthen_request_image_bytes_v1;

typedef union thinkthen_request_image_data_v1 {
  const struct thinkthen_request_image_file_v1 *file;
  const struct thinkthen_request_image_bytes_v1 *bytes;
} thinkthen_request_image_data_v1;

typedef struct thinkthen_request_image_v1 {
  uint32_t kind;
  union thinkthen_request_image_data_v1 data;
} thinkthen_request_image_v1;

typedef struct thinkthen_request_image_list_v1 {
  const struct thinkthen_request_image_v1 *const *data;
  size_t len;
} thinkthen_request_image_list_v1;

typedef struct thinkthen_request_input_text_v1 {
  const struct thinkthen_request_image_list_v1 *m_images;
  const struct thinkthen_request_text_v1 *m_text;
} thinkthen_request_input_text_v1;

typedef struct thinkthen_request_input_json_value_v1 {
  const struct thinkthen_complete_json_v1 *value;
} thinkthen_request_input_json_value_v1;

typedef struct thinkthen_request_input_json_v1 {
  const struct thinkthen_request_image_list_v1 *m_images;
  const struct thinkthen_request_input_json_value_v1 *m_value;
} thinkthen_request_input_json_v1;

typedef struct thinkthen_request_json_value_map_entry_v1 {
  struct thinkthen_complete_utf8_v1 key;
  const struct thinkthen_request_json_value_v1 *value;
} thinkthen_request_json_value_map_entry_v1;

typedef struct thinkthen_request_json_value_map_v1 {
  const struct thinkthen_request_json_value_map_entry_v1 *data;
  size_t len;
} thinkthen_request_json_value_map_v1;

typedef union thinkthen_request_context_schema_data_v1 {
  const struct thinkthen_request_text_v1 *text;
  const struct thinkthen_request_json_value_map_v1 *json_value_map;
} thinkthen_request_context_schema_data_v1;

typedef struct thinkthen_request_context_schema_v1 {
  uint32_t kind;
  union thinkthen_request_context_schema_data_v1 data;
} thinkthen_request_context_schema_v1;

typedef struct thinkthen_request_recognition_example_entity_v1 {
  const struct thinkthen_request_unsigned_v1 *m_end;
  const struct thinkthen_request_text_v1 *m_kind;
  const struct thinkthen_request_unsigned_v1 *m_start;
} thinkthen_request_recognition_example_entity_v1;

typedef struct thinkthen_request_recognition_example_entity_list_v1 {
  const struct thinkthen_request_recognition_example_entity_v1 *const *data;
  size_t len;
} thinkthen_request_recognition_example_entity_list_v1;

typedef struct thinkthen_request_text_list_v1 {
  const struct thinkthen_request_text_v1 *const *data;
  size_t len;
} thinkthen_request_text_list_v1;

typedef struct thinkthen_request_recognition_example_text_v1 {
  const struct thinkthen_request_recognition_example_entity_list_v1 *m_entities;
  const struct thinkthen_request_text_list_v1 *m_kinds;
  const struct thinkthen_request_text_v1 *m_text;
} thinkthen_request_recognition_example_text_v1;

typedef union thinkthen_request_recognition_example_data_v1 {
  const struct thinkthen_request_text_v1 *text;
  const struct thinkthen_request_recognition_example_text_v1 *example_text;
} thinkthen_request_recognition_example_data_v1;

typedef struct thinkthen_request_recognition_example_v1 {
  uint32_t kind;
  union thinkthen_request_recognition_example_data_v1 data;
} thinkthen_request_recognition_example_v1;

typedef struct thinkthen_request_recognition_example_list_v1 {
  const struct thinkthen_request_recognition_example_v1 *const *data;
  size_t len;
} thinkthen_request_recognition_example_list_v1;

typedef struct thinkthen_request_option_schema_v1 {
  const struct thinkthen_request_json_value_v1 *m_description;
  const struct thinkthen_request_text_v1 *m_name;
} thinkthen_request_option_schema_v1;

typedef struct thinkthen_request_option_schema_list_v1 {
  const struct thinkthen_request_option_schema_v1 *const *data;
  size_t len;
} thinkthen_request_option_schema_list_v1;

typedef struct thinkthen_request_original_text_v1 {
  const struct thinkthen_request_text_v1 *m_text;
} thinkthen_request_original_text_v1;

typedef struct thinkthen_request_original_json_value_v1 {
  const struct thinkthen_complete_json_v1 *value;
} thinkthen_request_original_json_value_v1;

typedef struct thinkthen_request_original_json_v1 {
  const struct thinkthen_request_original_json_value_v1 *m_value;
} thinkthen_request_original_json_v1;

typedef union thinkthen_request_original_data_v1 {
  const struct thinkthen_request_original_text_v1 *text;
  const struct thinkthen_request_original_json_v1 *json;
} thinkthen_request_original_data_v1;

typedef struct thinkthen_request_original_v1 {
  uint32_t kind;
  union thinkthen_request_original_data_v1 data;
} thinkthen_request_original_v1;

typedef struct thinkthen_request_recognition_seed_span_v1 {
  const struct thinkthen_request_unsigned_v1 *m_end;
  const struct thinkthen_request_text_v1 *m_kind;
  const struct thinkthen_request_unsigned_v1 *m_start;
} thinkthen_request_recognition_seed_span_v1;

typedef struct thinkthen_request_recognition_seed_span_list_v1 {
  const struct thinkthen_request_recognition_seed_span_v1 *const *data;
  size_t len;
} thinkthen_request_recognition_seed_span_list_v1;

typedef struct thinkthen_request_item_v1 {
  const struct thinkthen_request_context_schema_v1 *m_context;
  const struct thinkthen_request_recognition_example_list_v1 *m_examples;
  const struct thinkthen_request_image_list_v1 *m_images;
  const struct thinkthen_request_option_schema_list_v1 *m_options;
  const struct thinkthen_request_original_v1 *m_original;
  const struct thinkthen_request_recognition_seed_span_list_v1 *m_seed_spans;
} thinkthen_request_item_v1;

typedef struct thinkthen_request_item_list_v1 {
  const struct thinkthen_request_item_v1 *const *data;
  size_t len;
} thinkthen_request_item_list_v1;

typedef struct thinkthen_request_input_records_v1 {
  const struct thinkthen_request_item_list_v1 *m_items;
} thinkthen_request_input_records_v1;

typedef struct thinkthen_request_input_units_v1 {
  const struct thinkthen_request_item_list_v1 *m_items;
} thinkthen_request_input_units_v1;

typedef struct thinkthen_request_input_entities_v1 {
  const struct thinkthen_request_item_list_v1 *m_items;
} thinkthen_request_input_entities_v1;

typedef struct thinkthen_request_literal_document_v1 {
  uint32_t reserved;
} thinkthen_request_literal_document_v1;

typedef struct thinkthen_request_literal_lines_v1 {
  uint32_t reserved;
} thinkthen_request_literal_lines_v1;

typedef struct thinkthen_request_literal_jsonl_v1 {
  uint32_t reserved;
} thinkthen_request_literal_jsonl_v1;

typedef struct thinkthen_request_literal_csv_v1 {
  uint32_t reserved;
} thinkthen_request_literal_csv_v1;

typedef struct thinkthen_request_literal_tsv_v1 {
  uint32_t reserved;
} thinkthen_request_literal_tsv_v1;

typedef union thinkthen_request_framing_data_v1 {
  const struct thinkthen_request_literal_document_v1 *literal_document;
  const struct thinkthen_request_literal_lines_v1 *literal_lines;
  const struct thinkthen_request_literal_jsonl_v1 *literal_jsonl;
  const struct thinkthen_request_literal_csv_v1 *literal_csv;
  const struct thinkthen_request_literal_tsv_v1 *literal_tsv;
} thinkthen_request_framing_data_v1;

typedef struct thinkthen_request_framing_v1 {
  uint32_t kind;
  union thinkthen_request_framing_data_v1 data;
} thinkthen_request_framing_v1;

typedef struct thinkthen_request_literal_text_v1 {
  uint32_t reserved;
} thinkthen_request_literal_text_v1;

typedef struct thinkthen_request_literal_image_v1 {
  uint32_t reserved;
} thinkthen_request_literal_image_v1;

typedef union thinkthen_request_reader_media_data_v1 {
  const struct thinkthen_request_literal_text_v1 *literal_text;
  const struct thinkthen_request_literal_image_v1 *literal_image;
} thinkthen_request_reader_media_data_v1;

typedef struct thinkthen_request_reader_media_v1 {
  uint32_t kind;
  union thinkthen_request_reader_media_data_v1 data;
} thinkthen_request_reader_media_v1;

typedef struct thinkthen_request_literal_line_v1 {
  uint32_t reserved;
} thinkthen_request_literal_line_v1;

typedef struct thinkthen_request_literal_window_v1 {
  uint32_t reserved;
} thinkthen_request_literal_window_v1;

typedef struct thinkthen_request_literal_file_v1 {
  uint32_t reserved;
} thinkthen_request_literal_file_v1;

typedef union thinkthen_request_source_unit_data_v1 {
  const struct thinkthen_request_literal_line_v1 *literal_line;
  const struct thinkthen_request_literal_window_v1 *literal_window;
  const struct thinkthen_request_literal_file_v1 *literal_file;
} thinkthen_request_source_unit_data_v1;

typedef struct thinkthen_request_source_unit_v1 {
  uint32_t kind;
  union thinkthen_request_source_unit_data_v1 data;
} thinkthen_request_source_unit_v1;

typedef struct thinkthen_request_reader_v1 {
  const struct thinkthen_request_source_unit_v1 *m_unit;
  const struct thinkthen_request_unsigned_v1 *m_window;
} thinkthen_request_reader_v1;

typedef struct thinkthen_request_source_v1 {
  const struct thinkthen_request_framing_v1 *m_framing;
  const struct thinkthen_request_reader_media_v1 *m_media;
  const struct thinkthen_request_text_list_v1 *m_paths;
  const struct thinkthen_request_reader_v1 *m_reading;
} thinkthen_request_source_v1;

typedef struct thinkthen_request_input_source_v1 {
  const struct thinkthen_request_source_v1 *m_source;
} thinkthen_request_input_source_v1;

typedef struct thinkthen_request_input_feed_v1 {
  const struct thinkthen_request_framing_v1 *m_framing;
  const struct thinkthen_request_image_list_v1 *m_images;
  const struct thinkthen_request_text_v1 *m_name;
  const struct thinkthen_request_reader_v1 *m_reading;
} thinkthen_request_input_feed_v1;

typedef union thinkthen_request_input_data_v1 {
  const struct thinkthen_request_input_text_v1 *text;
  const struct thinkthen_request_input_json_v1 *json;
  const struct thinkthen_request_input_records_v1 *records;
  const struct thinkthen_request_input_units_v1 *units;
  const struct thinkthen_request_input_entities_v1 *entities;
  const struct thinkthen_request_input_source_v1 *source;
  const struct thinkthen_request_input_feed_v1 *feed;
} thinkthen_request_input_data_v1;

typedef struct thinkthen_request_input_v1 {
  uint32_t kind;
  union thinkthen_request_input_data_v1 data;
} thinkthen_request_input_v1;

typedef struct thinkthen_request_integer_v1 {
  int64_t value;
} thinkthen_request_integer_v1;

typedef struct thinkthen_request_literal_whole_v1 {
  uint32_t reserved;
} thinkthen_request_literal_whole_v1;

typedef struct thinkthen_request_literal_boundary_only_v1 {
  uint32_t reserved;
} thinkthen_request_literal_boundary_only_v1;

typedef union thinkthen_request_recognition_mode_data_v1 {
  const struct thinkthen_request_literal_whole_v1 *literal_whole;
  const struct thinkthen_request_literal_boundary_only_v1 *literal_boundary_only;
} thinkthen_request_recognition_mode_data_v1;

typedef struct thinkthen_request_recognition_mode_v1 {
  uint32_t kind;
  union thinkthen_request_recognition_mode_data_v1 data;
} thinkthen_request_recognition_mode_v1;

typedef struct thinkthen_request_number_v1 {
  double value;
} thinkthen_request_number_v1;

typedef union thinkthen_request_call_threshold_data_v1 {
  const struct thinkthen_request_number_v1 *number;
  const struct thinkthen_request_text_v1 *text;
} thinkthen_request_call_threshold_data_v1;

typedef struct thinkthen_request_call_threshold_v1 {
  uint32_t kind;
  union thinkthen_request_call_threshold_data_v1 data;
} thinkthen_request_call_threshold_v1;

typedef struct thinkthen_request_recognition_stage_context_v1 {
  const struct thinkthen_request_text_v1 *m_boundary;
  const struct thinkthen_request_text_v1 *m_kind_edge;
  const struct thinkthen_request_text_v1 *m_relation;
} thinkthen_request_recognition_stage_context_v1;

typedef struct thinkthen_request_call_options_v1 {
  const struct thinkthen_request_boolean_v1 *m_attempts;
  const struct thinkthen_request_batch_v1 *m_batch;
  const struct thinkthen_request_text_v1 *m_context;
  const struct thinkthen_request_text_v1 *m_context_field;
  const struct thinkthen_request_integer_v1 *m_deadline_ms;
  const struct thinkthen_request_recognition_example_list_v1 *m_examples;
  const struct thinkthen_request_text_v1 *m_examples_field;
  const struct thinkthen_request_text_list_v1 *m_field;
  const struct thinkthen_request_boolean_v1 *m_files_only;
  const struct thinkthen_request_unsigned_v1 *m_max_requests_total;
  const struct thinkthen_request_recognition_mode_v1 *m_mode;
  const struct thinkthen_request_text_v1 *m_model;
  const struct thinkthen_request_boolean_v1 *m_none;
  const struct thinkthen_request_text_v1 *m_options_field;
  const struct thinkthen_request_call_threshold_v1 *m_relation_threshold;
  const struct thinkthen_request_recognition_seed_span_list_v1 *m_seed_spans;
  const struct thinkthen_request_text_v1 *m_seed_spans_field;
  const struct thinkthen_request_unsigned_v1 *m_snippet_pieces;
  const struct thinkthen_request_recognition_stage_context_v1 *m_stage_context;
  const struct thinkthen_request_call_threshold_v1 *m_threshold;
  const struct thinkthen_request_unsigned_v1 *m_top;
} thinkthen_request_call_options_v1;

typedef struct thinkthen_request_question_selector_text_v1 {
  const struct thinkthen_request_text_v1 *m_text;
} thinkthen_request_question_selector_text_v1;

typedef struct thinkthen_request_literal_max_v1 {
  uint32_t reserved;
} thinkthen_request_literal_max_v1;

typedef union thinkthen_request_question_batch_data_v1 {
  const struct thinkthen_request_literal_max_v1 *literal_max;
  const struct thinkthen_request_integer_v1 *integer;
} thinkthen_request_question_batch_data_v1;

typedef struct thinkthen_request_question_batch_v1 {
  uint32_t kind;
  union thinkthen_request_question_batch_data_v1 data;
} thinkthen_request_question_batch_v1;

typedef struct thinkthen_request_input_declaration_string_v1 {
  uint32_t reserved;
} thinkthen_request_input_declaration_string_v1;

typedef struct thinkthen_request_input_property_string_v1 {
  uint32_t reserved;
} thinkthen_request_input_property_string_v1;

typedef struct thinkthen_request_input_property_number_v1 {
  uint32_t reserved;
} thinkthen_request_input_property_number_v1;

typedef struct thinkthen_request_input_property_boolean_v1 {
  uint32_t reserved;
} thinkthen_request_input_property_boolean_v1;

typedef struct thinkthen_request_input_property_array_items_v1 {
  uint32_t reserved;
} thinkthen_request_input_property_array_items_v1;

typedef struct thinkthen_request_input_property_array_v1 {
  const struct thinkthen_request_input_property_array_items_v1 *m_items;
} thinkthen_request_input_property_array_v1;

typedef union thinkthen_request_input_property_data_v1 {
  const struct thinkthen_request_input_property_string_v1 *string;
  const struct thinkthen_request_input_property_number_v1 *number;
  const struct thinkthen_request_input_property_boolean_v1 *boolean;
  const struct thinkthen_request_input_property_array_v1 *array;
} thinkthen_request_input_property_data_v1;

typedef struct thinkthen_request_input_property_v1 {
  uint32_t kind;
  union thinkthen_request_input_property_data_v1 data;
} thinkthen_request_input_property_v1;

typedef struct thinkthen_request_input_property_map_entry_v1 {
  struct thinkthen_complete_utf8_v1 key;
  const struct thinkthen_request_input_property_v1 *value;
} thinkthen_request_input_property_map_entry_v1;

typedef struct thinkthen_request_input_property_map_v1 {
  const struct thinkthen_request_input_property_map_entry_v1 *data;
  size_t len;
} thinkthen_request_input_property_map_v1;

typedef struct thinkthen_request_input_declaration_object_v1 {
  const struct thinkthen_request_input_property_map_v1 *m_properties;
  const struct thinkthen_request_text_list_v1 *m_required;
} thinkthen_request_input_declaration_object_v1;

typedef union thinkthen_request_input_declaration_data_v1 {
  const struct thinkthen_request_input_declaration_string_v1 *string;
  const struct thinkthen_request_input_declaration_object_v1 *object;
} thinkthen_request_input_declaration_data_v1;

typedef struct thinkthen_request_input_declaration_v1 {
  uint32_t kind;
  union thinkthen_request_input_declaration_data_v1 data;
} thinkthen_request_input_declaration_v1;

typedef struct thinkthen_request_json_value_list_v1 {
  const struct thinkthen_request_json_value_v1 *const *data;
  size_t len;
} thinkthen_request_json_value_list_v1;

typedef union thinkthen_request_question_text_data_v1 {
  const struct thinkthen_request_text_v1 *text;
  const struct thinkthen_request_json_value_map_v1 *json_value_map;
  const struct thinkthen_request_json_value_list_v1 *json_value_list;
} thinkthen_request_question_text_data_v1;

typedef struct thinkthen_request_question_text_v1 {
  uint32_t kind;
  union thinkthen_request_question_text_data_v1 data;
} thinkthen_request_question_text_v1;

typedef union thinkthen_request_criterion_data_v1 {
  const struct thinkthen_request_text_v1 *text;
  const struct thinkthen_request_json_value_map_v1 *json_value_map;
  const struct thinkthen_request_json_value_list_v1 *json_value_list;
  const struct thinkthen_request_null_v1 *null;
} thinkthen_request_criterion_data_v1;

typedef struct thinkthen_request_criterion_v1 {
  uint32_t kind;
  union thinkthen_request_criterion_data_v1 data;
} thinkthen_request_criterion_v1;

typedef struct thinkthen_request_name_v1 {
  struct thinkthen_complete_utf8_v1 text;
} thinkthen_request_name_v1;

typedef union thinkthen_request_pointers_data_v1 {
  const struct thinkthen_request_text_v1 *text;
  const struct thinkthen_request_text_list_v1 *text_list;
} thinkthen_request_pointers_data_v1;

typedef struct thinkthen_request_pointers_v1 {
  uint32_t kind;
  union thinkthen_request_pointers_data_v1 data;
} thinkthen_request_pointers_v1;

typedef struct thinkthen_request_profile_v1 {
  struct thinkthen_complete_utf8_v1 text;
} thinkthen_request_profile_v1;

typedef union thinkthen_request_threshold_data_v1 {
  const struct thinkthen_request_number_v1 *number;
  const struct thinkthen_request_text_v1 *text;
} thinkthen_request_threshold_data_v1;

typedef struct thinkthen_request_threshold_v1 {
  uint32_t kind;
  union thinkthen_request_threshold_data_v1 data;
} thinkthen_request_threshold_v1;

typedef struct thinkthen_request_definition_authored_decide_v1 {
  const struct thinkthen_request_question_batch_v1 *m_batch;
  const struct thinkthen_request_input_declaration_v1 *m_context_schema;
  const struct thinkthen_request_question_text_v1 *m_decide;
  const struct thinkthen_request_criterion_v1 *m_false;
  const struct thinkthen_request_input_declaration_v1 *m_item_schema;
  const struct thinkthen_request_name_v1 *m_model;
  const struct thinkthen_request_text_v1 *m_name;
  const struct thinkthen_request_pointers_v1 *m_on;
  const struct thinkthen_request_profile_v1 *m_profile;
  const struct thinkthen_request_threshold_v1 *m_threshold;
  const struct thinkthen_request_criterion_v1 *m_true;
  const struct thinkthen_request_integer_v1 *m_wording_version;
} thinkthen_request_definition_authored_decide_v1;

typedef struct thinkthen_request_name_list_v1 {
  const struct thinkthen_request_name_v1 *const *data;
  size_t len;
} thinkthen_request_name_list_v1;

typedef union thinkthen_request_description_data_v1 {
  const struct thinkthen_request_text_v1 *text;
  const struct thinkthen_request_json_value_map_v1 *json_value_map;
  const struct thinkthen_request_json_value_list_v1 *json_value_list;
  const struct thinkthen_request_null_v1 *null;
} thinkthen_request_description_data_v1;

typedef struct thinkthen_request_description_v1 {
  uint32_t kind;
  union thinkthen_request_description_data_v1 data;
} thinkthen_request_description_v1;

typedef struct thinkthen_request_description_map_entry_v1 {
  struct thinkthen_complete_utf8_v1 key;
  const struct thinkthen_request_description_v1 *value;
} thinkthen_request_description_map_entry_v1;

typedef struct thinkthen_request_description_map_v1 {
  const struct thinkthen_request_description_map_entry_v1 *data;
  size_t len;
} thinkthen_request_description_map_v1;

typedef union thinkthen_request_options_data_v1 {
  const struct thinkthen_request_name_list_v1 *name_list;
  const struct thinkthen_request_description_map_v1 *description_map;
} thinkthen_request_options_data_v1;

typedef struct thinkthen_request_options_v1 {
  uint32_t kind;
  union thinkthen_request_options_data_v1 data;
} thinkthen_request_options_v1;

typedef union thinkthen_request_cut_data_v1 {
  const struct thinkthen_request_number_v1 *number;
  const struct thinkthen_request_text_v1 *text;
} thinkthen_request_cut_data_v1;

typedef struct thinkthen_request_cut_v1 {
  uint32_t kind;
  union thinkthen_request_cut_data_v1 data;
} thinkthen_request_cut_v1;

typedef struct thinkthen_request_definition_authored_choose_v1 {
  const struct thinkthen_request_question_batch_v1 *m_batch;
  const struct thinkthen_request_question_text_v1 *m_choose;
  const struct thinkthen_request_input_declaration_v1 *m_context_schema;
  const struct thinkthen_request_input_declaration_v1 *m_item_schema;
  const struct thinkthen_request_name_v1 *m_model;
  const struct thinkthen_request_text_v1 *m_name;
  const struct thinkthen_request_pointers_v1 *m_on;
  const struct thinkthen_request_options_v1 *m_options;
  const struct thinkthen_request_profile_v1 *m_profile;
  const struct thinkthen_request_cut_v1 *m_threshold;
  const struct thinkthen_request_integer_v1 *m_wording_version;
} thinkthen_request_definition_authored_choose_v1;

typedef union thinkthen_request_labels_data_v1 {
  const struct thinkthen_request_name_list_v1 *name_list;
  const struct thinkthen_request_description_map_v1 *description_map;
} thinkthen_request_labels_data_v1;

typedef struct thinkthen_request_labels_v1 {
  uint32_t kind;
  union thinkthen_request_labels_data_v1 data;
} thinkthen_request_labels_v1;

typedef struct thinkthen_request_definition_authored_tag_v1 {
  const struct thinkthen_request_question_batch_v1 *m_batch;
  const struct thinkthen_request_input_declaration_v1 *m_context_schema;
  const struct thinkthen_request_input_declaration_v1 *m_item_schema;
  const struct thinkthen_request_labels_v1 *m_labels;
  const struct thinkthen_request_name_v1 *m_model;
  const struct thinkthen_request_text_v1 *m_name;
  const struct thinkthen_request_pointers_v1 *m_on;
  const struct thinkthen_request_profile_v1 *m_profile;
  const struct thinkthen_request_question_text_v1 *m_tag;
  const struct thinkthen_request_cut_v1 *m_threshold;
  const struct thinkthen_request_integer_v1 *m_wording_version;
} thinkthen_request_definition_authored_tag_v1;

typedef struct thinkthen_request_criterion_map_entry_v1 {
  struct thinkthen_complete_utf8_v1 key;
  const struct thinkthen_request_criterion_v1 *value;
} thinkthen_request_criterion_map_entry_v1;

typedef struct thinkthen_request_criterion_map_v1 {
  const struct thinkthen_request_criterion_map_entry_v1 *data;
  size_t len;
} thinkthen_request_criterion_map_v1;

typedef union thinkthen_request_levels_data_v1 {
  const struct thinkthen_request_name_list_v1 *name_list;
  const struct thinkthen_request_criterion_map_v1 *criterion_map;
} thinkthen_request_levels_data_v1;

typedef struct thinkthen_request_levels_v1 {
  uint32_t kind;
  union thinkthen_request_levels_data_v1 data;
} thinkthen_request_levels_v1;

typedef struct thinkthen_request_definition_authored_score_v1 {
  const struct thinkthen_request_question_batch_v1 *m_batch;
  const struct thinkthen_request_input_declaration_v1 *m_context_schema;
  const struct thinkthen_request_input_declaration_v1 *m_item_schema;
  const struct thinkthen_request_levels_v1 *m_levels;
  const struct thinkthen_request_name_v1 *m_model;
  const struct thinkthen_request_text_v1 *m_name;
  const struct thinkthen_request_pointers_v1 *m_on;
  const struct thinkthen_request_profile_v1 *m_profile;
  const struct thinkthen_request_question_text_v1 *m_score;
  const struct thinkthen_request_integer_v1 *m_wording_version;
} thinkthen_request_definition_authored_score_v1;

typedef struct thinkthen_request_relate_fields_v1 {
  const struct thinkthen_request_text_v1 *m_kind;
  const struct thinkthen_request_text_v1 *m_name;
} thinkthen_request_relate_fields_v1;

typedef struct thinkthen_request_relation_v1 {
  const struct thinkthen_request_boolean_v1 *m_either;
  const struct thinkthen_request_name_v1 *m_name;
  const struct thinkthen_request_name_v1 *m_reads;
  const struct thinkthen_request_boolean_v1 *m_single;
  const struct thinkthen_request_name_v1 *m_source;
  const struct thinkthen_request_name_v1 *m_target;
} thinkthen_request_relation_v1;

typedef struct thinkthen_request_relation_list_v1 {
  const struct thinkthen_request_relation_v1 *const *data;
  size_t len;
} thinkthen_request_relation_list_v1;

typedef struct thinkthen_request_relate_question_v1 {
  const struct thinkthen_request_relate_fields_v1 *m_fields;
  const struct thinkthen_request_relation_list_v1 *m_relations;
} thinkthen_request_relate_question_v1;

typedef struct thinkthen_request_definition_authored_relate_v1 {
  const struct thinkthen_request_input_declaration_v1 *m_context_schema;
  const struct thinkthen_request_input_declaration_v1 *m_item_schema;
  const struct thinkthen_request_name_v1 *m_model;
  const struct thinkthen_request_text_v1 *m_name;
  const struct thinkthen_request_profile_v1 *m_profile;
  const struct thinkthen_request_relate_question_v1 *m_relate;
  const struct thinkthen_request_cut_v1 *m_threshold;
  const struct thinkthen_request_integer_v1 *m_wording_version;
} thinkthen_request_definition_authored_relate_v1;

typedef struct thinkthen_request_definition_authored_find_v1 {
  const struct thinkthen_request_input_declaration_v1 *m_context_schema;
  const struct thinkthen_request_question_text_v1 *m_find;
  const struct thinkthen_request_input_declaration_v1 *m_item_schema;
  const struct thinkthen_request_name_v1 *m_model;
  const struct thinkthen_request_text_v1 *m_name;
  const struct thinkthen_request_pointers_v1 *m_on;
  const struct thinkthen_request_profile_v1 *m_profile;
  const struct thinkthen_request_integer_v1 *m_wording_version;
} thinkthen_request_definition_authored_find_v1;

typedef struct thinkthen_request_recognize_question_v1 {
  const struct thinkthen_request_question_text_v1 *m_entity_definition;
  const struct thinkthen_request_question_text_v1 *m_instructions;
  const struct thinkthen_request_description_map_v1 *m_kinds;
  const struct thinkthen_request_recognition_mode_v1 *m_mode;
  const struct thinkthen_request_relation_list_v1 *m_relations;
  const struct thinkthen_request_unsigned_v1 *m_snippet_pieces;
  const struct thinkthen_request_recognition_stage_context_v1 *m_stage_context;
} thinkthen_request_recognize_question_v1;

typedef struct thinkthen_request_definition_recognize_definition_v1 {
  const struct thinkthen_request_input_declaration_v1 *m_context_schema;
  const struct thinkthen_request_input_declaration_v1 *m_item_schema;
  const struct thinkthen_request_name_v1 *m_model;
  const struct thinkthen_request_text_v1 *m_name;
  const struct thinkthen_request_pointers_v1 *m_on;
  const struct thinkthen_request_profile_v1 *m_profile;
  const struct thinkthen_request_recognize_question_v1 *m_recognize;
  const struct thinkthen_request_cut_v1 *m_relation_threshold;
  const struct thinkthen_request_cut_v1 *m_threshold;
  const struct thinkthen_request_integer_v1 *m_wording_version;
} thinkthen_request_definition_recognize_definition_v1;

typedef struct thinkthen_request_annotate_question_decide_v1 {
  const struct thinkthen_request_input_declaration_v1 *m_context_schema;
  const struct thinkthen_request_question_text_v1 *m_decide;
  const struct thinkthen_request_criterion_v1 *m_false;
  const struct thinkthen_request_input_declaration_v1 *m_item_schema;
  const struct thinkthen_request_text_v1 *m_name;
  const struct thinkthen_request_pointers_v1 *m_on;
  const struct thinkthen_request_threshold_v1 *m_threshold;
  const struct thinkthen_request_criterion_v1 *m_true;
  const struct thinkthen_request_integer_v1 *m_wording_version;
} thinkthen_request_annotate_question_decide_v1;

typedef struct thinkthen_request_annotate_question_choose_v1 {
  const struct thinkthen_request_question_text_v1 *m_choose;
  const struct thinkthen_request_input_declaration_v1 *m_context_schema;
  const struct thinkthen_request_input_declaration_v1 *m_item_schema;
  const struct thinkthen_request_text_v1 *m_name;
  const struct thinkthen_request_pointers_v1 *m_on;
  const struct thinkthen_request_options_v1 *m_options;
  const struct thinkthen_request_cut_v1 *m_threshold;
  const struct thinkthen_request_integer_v1 *m_wording_version;
} thinkthen_request_annotate_question_choose_v1;

typedef struct thinkthen_request_annotate_question_tag_v1 {
  const struct thinkthen_request_input_declaration_v1 *m_context_schema;
  const struct thinkthen_request_input_declaration_v1 *m_item_schema;
  const struct thinkthen_request_labels_v1 *m_labels;
  const struct thinkthen_request_text_v1 *m_name;
  const struct thinkthen_request_pointers_v1 *m_on;
  const struct thinkthen_request_question_text_v1 *m_tag;
  const struct thinkthen_request_cut_v1 *m_threshold;
  const struct thinkthen_request_integer_v1 *m_wording_version;
} thinkthen_request_annotate_question_tag_v1;

typedef struct thinkthen_request_annotate_question_score_v1 {
  const struct thinkthen_request_input_declaration_v1 *m_context_schema;
  const struct thinkthen_request_input_declaration_v1 *m_item_schema;
  const struct thinkthen_request_levels_v1 *m_levels;
  const struct thinkthen_request_text_v1 *m_name;
  const struct thinkthen_request_pointers_v1 *m_on;
  const struct thinkthen_request_question_text_v1 *m_score;
  const struct thinkthen_request_integer_v1 *m_wording_version;
} thinkthen_request_annotate_question_score_v1;

typedef union thinkthen_request_annotate_question_data_v1 {
  const struct thinkthen_request_annotate_question_decide_v1 *decide;
  const struct thinkthen_request_annotate_question_choose_v1 *choose;
  const struct thinkthen_request_annotate_question_tag_v1 *tag;
  const struct thinkthen_request_annotate_question_score_v1 *score;
} thinkthen_request_annotate_question_data_v1;

typedef struct thinkthen_request_annotate_question_v1 {
  uint32_t kind;
  union thinkthen_request_annotate_question_data_v1 data;
} thinkthen_request_annotate_question_v1;

typedef struct thinkthen_request_annotate_question_map_entry_v1 {
  struct thinkthen_complete_utf8_v1 key;
  const struct thinkthen_request_annotate_question_v1 *value;
} thinkthen_request_annotate_question_map_entry_v1;

typedef struct thinkthen_request_annotate_question_map_v1 {
  const struct thinkthen_request_annotate_question_map_entry_v1 *data;
  size_t len;
} thinkthen_request_annotate_question_map_v1;

typedef struct thinkthen_request_definition_annotate_definition_v1 {
  const struct thinkthen_request_json_value_v1 *m_batch;
  const struct thinkthen_request_profile_v1 *m_profile;
  const struct thinkthen_request_annotate_question_map_v1 *m_questions;
  const struct thinkthen_request_threshold_v1 *m_threshold;
} thinkthen_request_definition_annotate_definition_v1;

typedef union thinkthen_request_definition_data_v1 {
  const struct thinkthen_request_definition_authored_decide_v1 *authored_decide;
  const struct thinkthen_request_definition_authored_choose_v1 *authored_choose;
  const struct thinkthen_request_definition_authored_tag_v1 *authored_tag;
  const struct thinkthen_request_definition_authored_score_v1 *authored_score;
  const struct thinkthen_request_definition_authored_relate_v1 *authored_relate;
  const struct thinkthen_request_definition_authored_find_v1 *authored_find;
  const struct thinkthen_request_definition_recognize_definition_v1 *recognize_definition;
  const struct thinkthen_request_definition_annotate_definition_v1 *annotate_definition;
} thinkthen_request_definition_data_v1;

typedef struct thinkthen_request_definition_v1 {
  uint32_t kind;
  union thinkthen_request_definition_data_v1 data;
} thinkthen_request_definition_v1;

typedef struct thinkthen_request_question_selector_definition_v1 {
  const struct thinkthen_request_definition_v1 *m_value;
} thinkthen_request_question_selector_definition_v1;

typedef struct thinkthen_request_question_selector_file_v1 {
  const struct thinkthen_request_text_v1 *m_path;
} thinkthen_request_question_selector_file_v1;

typedef struct thinkthen_request_question_selector_name_v1 {
  const struct thinkthen_request_text_v1 *m_name;
} thinkthen_request_question_selector_name_v1;

typedef struct thinkthen_request_question_selector_reference_v1 {
  const struct thinkthen_request_text_v1 *m_reference;
} thinkthen_request_question_selector_reference_v1;

typedef union thinkthen_request_question_selector_data_v1 {
  const struct thinkthen_request_question_selector_text_v1 *text;
  const struct thinkthen_request_question_selector_definition_v1 *definition;
  const struct thinkthen_request_question_selector_file_v1 *file;
  const struct thinkthen_request_question_selector_name_v1 *name;
  const struct thinkthen_request_question_selector_reference_v1 *reference;
} thinkthen_request_question_selector_data_v1;

typedef struct thinkthen_request_question_selector_v1 {
  uint32_t kind;
  union thinkthen_request_question_selector_data_v1 data;
} thinkthen_request_question_selector_v1;

typedef struct thinkthen_request_arguments_v1 {
  const struct thinkthen_request_input_v1 *m_input;
  const struct thinkthen_request_call_options_v1 *m_options;
  const struct thinkthen_request_question_selector_v1 *m_question;
} thinkthen_request_arguments_v1;

/*
 A producer returns ITEM and counted RequestItem JSON, END, or FAILURE.
 Bytes remain readable until the next invocation or until run_call returns.
 It runs synchronously on the run_call caller, and may reenter independent C operations.
 Never unwind or throw across this callback; retain private host exceptions locally.
 */
typedef uint32_t (*thinkthen_call_producer_v1)(void*, struct thinkthen_complete_utf8_v1*);

typedef struct thinkthen_request_reader_failure_producer_v1 {
  uint32_t reserved;
} thinkthen_request_reader_failure_producer_v1;

typedef struct thinkthen_request_session_source_location_v1 {
  const struct thinkthen_request_text_v1 *m_file;
  const struct thinkthen_request_unsigned_v1 *m_first_line;
  const struct thinkthen_request_unsigned_v1 *m_last_line;
} thinkthen_request_session_source_location_v1;

typedef struct thinkthen_request_reader_failure_io_v1 {
  const struct thinkthen_request_session_source_location_v1 *m_location;
} thinkthen_request_reader_failure_io_v1;

typedef struct thinkthen_request_reader_failure_utf_8_v1 {
  const struct thinkthen_request_session_source_location_v1 *m_location;
} thinkthen_request_reader_failure_utf_8_v1;

typedef struct thinkthen_request_reader_failure_invalid_input_v1 {
  const struct thinkthen_request_session_source_location_v1 *m_location;
} thinkthen_request_reader_failure_invalid_input_v1;

typedef union thinkthen_request_reader_failure_data_v1 {
  const struct thinkthen_request_reader_failure_producer_v1 *producer;
  const struct thinkthen_request_reader_failure_io_v1 *io;
  const struct thinkthen_request_reader_failure_utf_8_v1 *utf_8;
  const struct thinkthen_request_reader_failure_invalid_input_v1 *invalid_input;
} thinkthen_request_reader_failure_data_v1;

typedef struct thinkthen_request_reader_failure_v1 {
  uint32_t kind;
  union thinkthen_request_reader_failure_data_v1 data;
} thinkthen_request_reader_failure_v1;

typedef struct thinkthen_request_session_descriptor_v1 {
  const struct thinkthen_request_item_v1 *m_item;
  const struct thinkthen_request_session_source_location_v1 *m_location;
} thinkthen_request_session_descriptor_v1;

typedef struct thinkthen_complete_answer_yes_no_v1 {
  struct thinkthen_complete_utf8_v1 kind;
  double probability;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_answer_yes_no_v1;

typedef struct thinkthen_complete_f64_presence_v1 {
  uint32_t presence;
  double value;
} thinkthen_complete_f64_presence_v1;

typedef struct thinkthen_complete_number_map_entry_v1 {
  struct thinkthen_complete_utf8_v1 name;
  double value;
} thinkthen_complete_number_map_entry_v1;

typedef struct thinkthen_complete_number_map_v1 {
  const struct thinkthen_complete_number_map_entry_v1 *data;
  size_t len;
} thinkthen_complete_number_map_v1;

typedef struct thinkthen_complete_answer_choice_v1 {
  struct thinkthen_complete_f64_presence_v1 confidence;
  struct thinkthen_complete_utf8_v1 kind;
  struct thinkthen_complete_utf8_v1 pick;
  struct thinkthen_complete_number_map_v1 probabilities;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_answer_choice_v1;

typedef struct thinkthen_complete_answer_tag_v1 {
  struct thinkthen_complete_utf8_v1 kind;
  struct thinkthen_complete_number_map_v1 probabilities;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_answer_tag_v1;

typedef struct thinkthen_complete_answer_score_v1 {
  struct thinkthen_complete_f64_presence_v1 confidence;
  struct thinkthen_complete_utf8_v1 kind;
  struct thinkthen_complete_utf8_v1 level;
  struct thinkthen_complete_number_map_v1 probabilities;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_answer_score_v1;

typedef union thinkthen_complete_answer_data_v1 {
  const struct thinkthen_complete_answer_yes_no_v1 *yes_no;
  const struct thinkthen_complete_answer_choice_v1 *choice;
  const struct thinkthen_complete_answer_tag_v1 *tag;
  const struct thinkthen_complete_answer_score_v1 *score;
} thinkthen_complete_answer_data_v1;

typedef struct thinkthen_complete_answer_v1 {
  uint32_t kind;
  union thinkthen_complete_answer_data_v1 data;
} thinkthen_complete_answer_v1;

typedef struct thinkthen_complete_answer_id_v1 {
  struct thinkthen_complete_utf8_v1 value;
} thinkthen_complete_answer_id_v1;

typedef struct thinkthen_complete_image_media_v1 {
  uint32_t kind;
} thinkthen_complete_image_media_v1;

typedef struct thinkthen_complete_image_v1 {
  struct thinkthen_complete_utf8_v1 base64;
  uint64_t height;
  const struct thinkthen_complete_image_media_v1 *media;
  uint64_t width;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_image_v1;

typedef struct thinkthen_complete_image_list_v1 {
  const struct thinkthen_complete_image_v1 *const *data;
  size_t len;
} thinkthen_complete_image_list_v1;

typedef struct thinkthen_complete_image_list_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_image_list_v1 value;
} thinkthen_complete_image_list_presence_v1;

typedef struct thinkthen_complete_json_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_json_v1 *value;
} thinkthen_complete_json_presence_v1;

typedef union thinkthen_complete_batch_setting_data_v1 {
  uint64_t integer;
  struct thinkthen_complete_utf8_v1 string;
} thinkthen_complete_batch_setting_data_v1;

typedef struct thinkthen_complete_batch_setting_v1 {
  uint32_t kind;
  union thinkthen_complete_batch_setting_data_v1 data;
} thinkthen_complete_batch_setting_v1;

typedef struct thinkthen_complete_batch_setting_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_batch_setting_v1 *value;
} thinkthen_complete_batch_setting_presence_v1;

typedef struct thinkthen_complete_batch_warning_v1 {
  const struct thinkthen_complete_batch_setting_v1 *running;
  const struct thinkthen_complete_batch_setting_v1 *tuned_for;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_batch_warning_v1;

typedef struct thinkthen_complete_batch_warning_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_batch_warning_v1 *value;
} thinkthen_complete_batch_warning_presence_v1;

typedef struct thinkthen_complete_observation_id_v1 {
  struct thinkthen_complete_utf8_v1 value;
} thinkthen_complete_observation_id_v1;

typedef struct thinkthen_complete_observation_observation_id_v1 {
  const struct thinkthen_complete_observation_id_v1 *observation_id;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_observation_observation_id_v1;

typedef struct thinkthen_complete_failure_id_v1 {
  struct thinkthen_complete_utf8_v1 value;
} thinkthen_complete_failure_id_v1;

typedef struct thinkthen_complete_observation_failure_id_v1 {
  const struct thinkthen_complete_failure_id_v1 *failure_id;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_observation_failure_id_v1;

typedef union thinkthen_complete_observation_data_v1 {
  const struct thinkthen_complete_observation_observation_id_v1 *observation_id;
  const struct thinkthen_complete_observation_failure_id_v1 *failure_id;
} thinkthen_complete_observation_data_v1;

typedef struct thinkthen_complete_observation_v1 {
  uint32_t kind;
  union thinkthen_complete_observation_data_v1 data;
} thinkthen_complete_observation_v1;

typedef struct thinkthen_complete_observation_list_v1 {
  const struct thinkthen_complete_observation_v1 *const *data;
  size_t len;
} thinkthen_complete_observation_list_v1;

typedef struct thinkthen_complete_origin_v1 {
  uint32_t kind;
} thinkthen_complete_origin_v1;

typedef struct thinkthen_complete_origin_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_origin_v1 *value;
} thinkthen_complete_origin_presence_v1;

typedef struct thinkthen_complete_profile_warning_v1 {
  struct thinkthen_complete_utf8_v1 running;
  struct thinkthen_complete_utf8_v1 tuned_for;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_profile_warning_v1;

typedef struct thinkthen_complete_profile_warning_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_profile_warning_v1 *value;
} thinkthen_complete_profile_warning_presence_v1;

typedef struct thinkthen_complete_question_source_v1 {
  struct thinkthen_complete_utf8_v1 answered_by;
  struct thinkthen_complete_u64_presence_v1 batch_size;
  const struct thinkthen_complete_origin_v1 *origin;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_question_source_v1;

typedef struct thinkthen_complete_question_source_list_v1 {
  const struct thinkthen_complete_question_source_v1 *const *data;
  size_t len;
} thinkthen_complete_question_source_list_v1;

typedef struct thinkthen_complete_string_list_v1 {
  const struct thinkthen_complete_utf8_v1 *data;
  size_t len;
} thinkthen_complete_string_list_v1;

typedef struct thinkthen_complete_usage_v1 {
  struct thinkthen_complete_u64_presence_v1 input_tokens;
  struct thinkthen_complete_u64_presence_v1 output_tokens;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_usage_v1;

typedef struct thinkthen_complete_usage_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_usage_v1 *value;
} thinkthen_complete_usage_presence_v1;

typedef struct thinkthen_complete_meta_v1 {
  struct thinkthen_complete_utf8_presence_v1 answered_by;
  struct thinkthen_complete_attempt_list_presence_v1 attempts;
  struct thinkthen_complete_batch_setting_presence_v1 batch_setting;
  struct thinkthen_complete_batch_warning_presence_v1 batch_warning;
  uint32_t cached;
  struct thinkthen_complete_utf8_presence_v1 context_sha256;
  uint64_t failed_questions;
  struct thinkthen_complete_utf8_v1 model;
  struct thinkthen_complete_observation_list_v1 observations;
  struct thinkthen_complete_origin_presence_v1 origin;
  struct thinkthen_complete_profile_warning_presence_v1 profile_warning;
  struct thinkthen_complete_utf8_presence_v1 question_sha256;
  struct thinkthen_complete_question_source_list_v1 question_sources;
  struct thinkthen_complete_utf8_presence_v1 questions_sha256;
  struct thinkthen_complete_string_list_v1 requests;
  uint64_t requests_sent;
  struct thinkthen_complete_utf8_v1 tool;
  struct thinkthen_complete_utf8_v1 url;
  struct thinkthen_complete_usage_presence_v1 usage;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_meta_v1;

typedef struct thinkthen_complete_batch_string_v1 {
  uint32_t kind;
} thinkthen_complete_batch_string_v1;

typedef union thinkthen_complete_batch_data_v1 {
  uint64_t integer;
  const struct thinkthen_complete_batch_string_v1 *string;
} thinkthen_complete_batch_data_v1;

typedef struct thinkthen_complete_batch_v1 {
  uint32_t kind;
  union thinkthen_complete_batch_data_v1 data;
} thinkthen_complete_batch_v1;

typedef struct thinkthen_complete_batch_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_batch_v1 *value;
} thinkthen_complete_batch_presence_v1;

typedef struct thinkthen_complete_string_type_v1 {
  uint32_t kind;
} thinkthen_complete_string_type_v1;

typedef struct thinkthen_complete_input_declaration_string_root_v1 {
  const struct thinkthen_complete_string_type_v1 *type;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_input_declaration_string_root_v1;

typedef struct thinkthen_complete_input_property_type_string_v1 {
  struct thinkthen_complete_utf8_v1 type;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_input_property_type_string_v1;

typedef struct thinkthen_complete_input_property_type_number_v1 {
  struct thinkthen_complete_utf8_v1 type;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_input_property_type_number_v1;

typedef struct thinkthen_complete_input_property_type_boolean_v1 {
  struct thinkthen_complete_utf8_v1 type;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_input_property_type_boolean_v1;

typedef struct thinkthen_complete_string_root_v1 {
  const struct thinkthen_complete_string_type_v1 *type;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_string_root_v1;

typedef struct thinkthen_complete_input_property_type_array_v1 {
  const struct thinkthen_complete_string_root_v1 *items;
  struct thinkthen_complete_utf8_v1 type;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_input_property_type_array_v1;

typedef union thinkthen_complete_input_property_type_data_v1 {
  const struct thinkthen_complete_input_property_type_string_v1 *string;
  const struct thinkthen_complete_input_property_type_number_v1 *number;
  const struct thinkthen_complete_input_property_type_boolean_v1 *boolean;
  const struct thinkthen_complete_input_property_type_array_v1 *array;
} thinkthen_complete_input_property_type_data_v1;

typedef struct thinkthen_complete_input_property_type_v1 {
  uint32_t kind;
  union thinkthen_complete_input_property_type_data_v1 data;
} thinkthen_complete_input_property_type_v1;

typedef struct thinkthen_complete_input_property_type_map_entry_v1 {
  struct thinkthen_complete_utf8_v1 name;
  const struct thinkthen_complete_input_property_type_v1 *value;
} thinkthen_complete_input_property_type_map_entry_v1;

typedef struct thinkthen_complete_input_property_type_map_v1 {
  const struct thinkthen_complete_input_property_type_map_entry_v1 *data;
  size_t len;
} thinkthen_complete_input_property_type_map_v1;

typedef struct thinkthen_complete_string_list_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_string_list_v1 value;
} thinkthen_complete_string_list_presence_v1;

typedef struct thinkthen_complete_object_type_v1 {
  uint32_t kind;
} thinkthen_complete_object_type_v1;

typedef struct thinkthen_complete_input_declaration_object_root_v1 {
  struct thinkthen_complete_input_property_type_map_v1 properties;
  struct thinkthen_complete_string_list_presence_v1 required;
  const struct thinkthen_complete_object_type_v1 *type;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_input_declaration_object_root_v1;

typedef union thinkthen_complete_input_declaration_data_v1 {
  const struct thinkthen_complete_input_declaration_string_root_v1 *string_root;
  const struct thinkthen_complete_input_declaration_object_root_v1 *object_root;
} thinkthen_complete_input_declaration_data_v1;

typedef struct thinkthen_complete_input_declaration_v1 {
  uint32_t kind;
  union thinkthen_complete_input_declaration_data_v1 data;
} thinkthen_complete_input_declaration_v1;

typedef struct thinkthen_complete_input_declaration_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_input_declaration_v1 *value;
} thinkthen_complete_input_declaration_presence_v1;

typedef struct thinkthen_complete_label_v1 {
  struct thinkthen_complete_json_presence_v1 description;
  struct thinkthen_complete_utf8_v1 name;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_label_v1;

typedef struct thinkthen_complete_label_list_v1 {
  const struct thinkthen_complete_label_v1 *const *data;
  size_t len;
} thinkthen_complete_label_list_v1;

typedef struct thinkthen_complete_label_list_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_label_list_v1 value;
} thinkthen_complete_label_list_presence_v1;

typedef struct thinkthen_complete_question_name_v1 {
  struct thinkthen_complete_utf8_v1 value;
} thinkthen_complete_question_name_v1;

typedef struct thinkthen_complete_question_name_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_question_name_v1 *value;
} thinkthen_complete_question_name_presence_v1;

typedef struct thinkthen_complete_wording_version_v1 {
  uint64_t value;
} thinkthen_complete_wording_version_v1;

typedef struct thinkthen_complete_wording_version_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_wording_version_v1 *value;
} thinkthen_complete_wording_version_presence_v1;

typedef struct thinkthen_complete_question_document_decide_v1 {
  struct thinkthen_complete_batch_presence_v1 batch;
  struct thinkthen_complete_input_declaration_presence_v1 context_schema;
  struct thinkthen_complete_input_declaration_presence_v1 item_schema;
  struct thinkthen_complete_label_list_presence_v1 label_details;
  struct thinkthen_complete_utf8_presence_v1 model;
  struct thinkthen_complete_question_name_presence_v1 name;
  struct thinkthen_complete_string_list_presence_v1 on;
  struct thinkthen_complete_utf8_presence_v1 profile;
  struct thinkthen_complete_wording_version_presence_v1 wording_version;
  struct thinkthen_complete_json_presence_v1 false_;
  struct thinkthen_complete_json_presence_v1 text;
  struct thinkthen_complete_json_presence_v1 true_;
  struct thinkthen_complete_utf8_v1 verb;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_question_document_decide_v1;

typedef struct thinkthen_complete_question_document_choose_v1 {
  struct thinkthen_complete_batch_presence_v1 batch;
  struct thinkthen_complete_input_declaration_presence_v1 context_schema;
  struct thinkthen_complete_input_declaration_presence_v1 item_schema;
  struct thinkthen_complete_label_list_presence_v1 label_details;
  struct thinkthen_complete_utf8_presence_v1 model;
  struct thinkthen_complete_question_name_presence_v1 name;
  struct thinkthen_complete_string_list_presence_v1 on;
  struct thinkthen_complete_utf8_presence_v1 profile;
  struct thinkthen_complete_wording_version_presence_v1 wording_version;
  struct thinkthen_complete_string_list_v1 options;
  struct thinkthen_complete_json_presence_v1 text;
  struct thinkthen_complete_utf8_v1 verb;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_question_document_choose_v1;

typedef struct thinkthen_complete_question_document_tag_v1 {
  struct thinkthen_complete_batch_presence_v1 batch;
  struct thinkthen_complete_input_declaration_presence_v1 context_schema;
  struct thinkthen_complete_input_declaration_presence_v1 item_schema;
  struct thinkthen_complete_label_list_presence_v1 label_details;
  struct thinkthen_complete_utf8_presence_v1 model;
  struct thinkthen_complete_question_name_presence_v1 name;
  struct thinkthen_complete_string_list_presence_v1 on;
  struct thinkthen_complete_utf8_presence_v1 profile;
  struct thinkthen_complete_wording_version_presence_v1 wording_version;
  struct thinkthen_complete_string_list_v1 labels;
  struct thinkthen_complete_json_presence_v1 text;
  struct thinkthen_complete_utf8_v1 verb;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_question_document_tag_v1;

typedef struct thinkthen_complete_question_document_score_v1 {
  struct thinkthen_complete_batch_presence_v1 batch;
  struct thinkthen_complete_input_declaration_presence_v1 context_schema;
  struct thinkthen_complete_input_declaration_presence_v1 item_schema;
  struct thinkthen_complete_label_list_presence_v1 label_details;
  struct thinkthen_complete_utf8_presence_v1 model;
  struct thinkthen_complete_question_name_presence_v1 name;
  struct thinkthen_complete_string_list_presence_v1 on;
  struct thinkthen_complete_utf8_presence_v1 profile;
  struct thinkthen_complete_wording_version_presence_v1 wording_version;
  struct thinkthen_complete_string_list_v1 levels;
  struct thinkthen_complete_json_presence_v1 text;
  struct thinkthen_complete_utf8_v1 verb;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_question_document_score_v1;

typedef union thinkthen_complete_question_document_data_v1 {
  const struct thinkthen_complete_question_document_decide_v1 *decide;
  const struct thinkthen_complete_question_document_choose_v1 *choose;
  const struct thinkthen_complete_question_document_tag_v1 *tag;
  const struct thinkthen_complete_question_document_score_v1 *score;
} thinkthen_complete_question_document_data_v1;

typedef struct thinkthen_complete_question_document_v1 {
  uint32_t kind;
  union thinkthen_complete_question_document_data_v1 data;
} thinkthen_complete_question_document_v1;

typedef struct thinkthen_complete_version_v1 {
  uint32_t kind;
} thinkthen_complete_version_v1;

typedef struct thinkthen_complete_physical_source_v1 {
  struct thinkthen_complete_utf8_v1 file;
  struct thinkthen_complete_u64_presence_v1 first_line;
  struct thinkthen_complete_u64_presence_v1 last_line;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_physical_source_v1;

typedef struct thinkthen_complete_physical_source_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_physical_source_v1 *value;
} thinkthen_complete_physical_source_presence_v1;

typedef struct thinkthen_complete_rank_member_result_v1 {
  const struct thinkthen_complete_answer_v1 *answer;
  const struct thinkthen_complete_answer_id_v1 *answer_id;
  struct thinkthen_complete_image_list_presence_v1 images;
  const struct thinkthen_complete_meta_v1 *meta;
  const struct thinkthen_complete_question_document_v1 *question;
  const struct thinkthen_complete_version_v1 *schema;
  struct thinkthen_complete_physical_source_presence_v1 source;
  struct thinkthen_complete_json_presence_v1 threshold;
  uint64_t value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_rank_member_result_v1;

typedef struct thinkthen_complete_rank_member_v1 {
  struct thinkthen_complete_utf8_v1 name;
  const struct thinkthen_complete_rank_member_result_v1 *result;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_rank_member_v1;

typedef struct thinkthen_complete_rank_member_list_v1 {
  const struct thinkthen_complete_rank_member_v1 *const *data;
  size_t len;
} thinkthen_complete_rank_member_list_v1;

typedef struct thinkthen_complete_rank_member_list_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_rank_member_list_v1 value;
} thinkthen_complete_rank_member_list_presence_v1;

typedef union thinkthen_complete_threshold_data_v1 {
  double number;
  struct thinkthen_complete_utf8_v1 string;
} thinkthen_complete_threshold_data_v1;

typedef struct thinkthen_complete_threshold_v1 {
  uint32_t kind;
  union thinkthen_complete_threshold_data_v1 data;
} thinkthen_complete_threshold_v1;

typedef struct thinkthen_complete_threshold_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_threshold_v1 *value;
} thinkthen_complete_threshold_presence_v1;

typedef struct thinkthen_complete_decide_value_v1 {
  const struct thinkthen_complete_json_v1 *value;
} thinkthen_complete_decide_value_v1;

typedef struct thinkthen_complete_decide_value_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_decide_value_v1 *value;
} thinkthen_complete_decide_value_presence_v1;

typedef struct thinkthen_complete_atomic_decide_value_v1 {
  const struct thinkthen_complete_answer_v1 *answer;
  const struct thinkthen_complete_answer_id_v1 *answer_id;
  struct thinkthen_complete_image_list_presence_v1 images;
  struct thinkthen_complete_u64_presence_v1 index;
  struct thinkthen_complete_json_presence_v1 input;
  struct thinkthen_complete_rank_member_list_presence_v1 members;
  const struct thinkthen_complete_meta_v1 *meta;
  const struct thinkthen_complete_question_document_v1 *question;
  struct thinkthen_complete_utf8_presence_v1 question_name;
  const struct thinkthen_complete_version_v1 *schema;
  struct thinkthen_complete_physical_source_presence_v1 source;
  struct thinkthen_complete_threshold_presence_v1 threshold;
  struct thinkthen_complete_decide_value_presence_v1 value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_atomic_decide_value_v1;

typedef struct thinkthen_complete_session_packet_decide_row_v1 {
  struct thinkthen_complete_utf8_v1 function;
  struct thinkthen_complete_utf8_v1 kind;
  const struct thinkthen_complete_atomic_decide_value_v1 *value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_session_packet_decide_row_v1;

typedef struct thinkthen_complete_atomic_nullable_string_v1 {
  const struct thinkthen_complete_answer_v1 *answer;
  const struct thinkthen_complete_answer_id_v1 *answer_id;
  struct thinkthen_complete_image_list_presence_v1 images;
  struct thinkthen_complete_u64_presence_v1 index;
  struct thinkthen_complete_json_presence_v1 input;
  struct thinkthen_complete_rank_member_list_presence_v1 members;
  const struct thinkthen_complete_meta_v1 *meta;
  const struct thinkthen_complete_question_document_v1 *question;
  struct thinkthen_complete_utf8_presence_v1 question_name;
  const struct thinkthen_complete_version_v1 *schema;
  struct thinkthen_complete_physical_source_presence_v1 source;
  struct thinkthen_complete_threshold_presence_v1 threshold;
  struct thinkthen_complete_utf8_presence_v1 value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_atomic_nullable_string_v1;

typedef struct thinkthen_complete_session_packet_choose_row_v1 {
  struct thinkthen_complete_utf8_v1 function;
  struct thinkthen_complete_utf8_v1 kind;
  const struct thinkthen_complete_atomic_nullable_string_v1 *value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_session_packet_choose_row_v1;

typedef struct thinkthen_complete_atomic_array_of_string_v1 {
  const struct thinkthen_complete_answer_v1 *answer;
  const struct thinkthen_complete_answer_id_v1 *answer_id;
  struct thinkthen_complete_image_list_presence_v1 images;
  struct thinkthen_complete_u64_presence_v1 index;
  struct thinkthen_complete_json_presence_v1 input;
  struct thinkthen_complete_rank_member_list_presence_v1 members;
  const struct thinkthen_complete_meta_v1 *meta;
  const struct thinkthen_complete_question_document_v1 *question;
  struct thinkthen_complete_utf8_presence_v1 question_name;
  const struct thinkthen_complete_version_v1 *schema;
  struct thinkthen_complete_physical_source_presence_v1 source;
  struct thinkthen_complete_threshold_presence_v1 threshold;
  struct thinkthen_complete_string_list_v1 value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_atomic_array_of_string_v1;

typedef struct thinkthen_complete_session_packet_tag_row_v1 {
  struct thinkthen_complete_utf8_v1 function;
  struct thinkthen_complete_utf8_v1 kind;
  const struct thinkthen_complete_atomic_array_of_string_v1 *value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_session_packet_tag_row_v1;

typedef struct thinkthen_complete_atomic_double_v1 {
  const struct thinkthen_complete_answer_v1 *answer;
  const struct thinkthen_complete_answer_id_v1 *answer_id;
  struct thinkthen_complete_image_list_presence_v1 images;
  struct thinkthen_complete_u64_presence_v1 index;
  struct thinkthen_complete_json_presence_v1 input;
  struct thinkthen_complete_rank_member_list_presence_v1 members;
  const struct thinkthen_complete_meta_v1 *meta;
  const struct thinkthen_complete_question_document_v1 *question;
  struct thinkthen_complete_utf8_presence_v1 question_name;
  const struct thinkthen_complete_version_v1 *schema;
  struct thinkthen_complete_physical_source_presence_v1 source;
  struct thinkthen_complete_threshold_presence_v1 threshold;
  double value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_atomic_double_v1;

typedef struct thinkthen_complete_session_packet_score_row_v1 {
  struct thinkthen_complete_utf8_v1 function;
  struct thinkthen_complete_utf8_v1 kind;
  const struct thinkthen_complete_atomic_double_v1 *value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_session_packet_score_row_v1;

typedef struct thinkthen_complete_atomic_boolean_v1 {
  const struct thinkthen_complete_answer_v1 *answer;
  const struct thinkthen_complete_answer_id_v1 *answer_id;
  struct thinkthen_complete_image_list_presence_v1 images;
  struct thinkthen_complete_u64_presence_v1 index;
  struct thinkthen_complete_json_presence_v1 input;
  struct thinkthen_complete_rank_member_list_presence_v1 members;
  const struct thinkthen_complete_meta_v1 *meta;
  const struct thinkthen_complete_question_document_v1 *question;
  struct thinkthen_complete_utf8_presence_v1 question_name;
  const struct thinkthen_complete_version_v1 *schema;
  struct thinkthen_complete_physical_source_presence_v1 source;
  struct thinkthen_complete_threshold_presence_v1 threshold;
  uint32_t value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_atomic_boolean_v1;

typedef struct thinkthen_complete_session_packet_filter_row_v1 {
  struct thinkthen_complete_utf8_v1 function;
  struct thinkthen_complete_utf8_v1 kind;
  const struct thinkthen_complete_atomic_boolean_v1 *value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_session_packet_filter_row_v1;

typedef union thinkthen_complete_value_data_v1 {
  uint32_t boolean;
  uint32_t null;
  struct thinkthen_complete_utf8_v1 string;
  struct thinkthen_complete_string_list_v1 array;
  double number;
} thinkthen_complete_value_data_v1;

typedef struct thinkthen_complete_value_v1 {
  uint32_t kind;
  union thinkthen_complete_value_data_v1 data;
} thinkthen_complete_value_v1;

typedef struct thinkthen_complete_value_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_value_v1 *value;
} thinkthen_complete_value_presence_v1;

typedef struct thinkthen_complete_annotation_member_answer_id_v1 {
  const struct thinkthen_complete_answer_v1 *answer;
  const struct thinkthen_complete_answer_id_v1 *answer_id;
  struct thinkthen_complete_observation_list_v1 observations;
  const struct thinkthen_complete_question_document_v1 *question;
  struct thinkthen_complete_question_source_list_v1 question_sources;
  struct thinkthen_complete_utf8_v1 request;
  struct thinkthen_complete_threshold_presence_v1 threshold;
  struct thinkthen_complete_usage_presence_v1 usage;
  struct thinkthen_complete_value_presence_v1 value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_annotation_member_answer_id_v1;

typedef struct thinkthen_complete_failure_cause_v1 {
  uint32_t kind;
} thinkthen_complete_failure_cause_v1;

typedef struct thinkthen_complete_failure_field_kind_v1 {
  uint32_t kind;
} thinkthen_complete_failure_field_kind_v1;

typedef struct thinkthen_complete_failure_v1 {
  const struct thinkthen_complete_failure_cause_v1 *cause;
  const struct thinkthen_complete_failure_field_kind_v1 *kind;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_failure_v1;

typedef struct thinkthen_complete_annotation_member_failure_id_v1 {
  const struct thinkthen_complete_failure_v1 *failure;
  const struct thinkthen_complete_failure_id_v1 *failure_id;
  struct thinkthen_complete_observation_list_v1 observations;
  const struct thinkthen_complete_question_document_v1 *question;
  struct thinkthen_complete_question_source_list_v1 question_sources;
  struct thinkthen_complete_utf8_v1 request;
  struct thinkthen_complete_threshold_presence_v1 threshold;
  struct thinkthen_complete_usage_presence_v1 usage;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_annotation_member_failure_id_v1;

typedef union thinkthen_complete_annotation_member_data_v1 {
  const struct thinkthen_complete_annotation_member_answer_id_v1 *answer_id;
  const struct thinkthen_complete_annotation_member_failure_id_v1 *failure_id;
} thinkthen_complete_annotation_member_data_v1;

typedef struct thinkthen_complete_annotation_member_v1 {
  uint32_t kind;
  union thinkthen_complete_annotation_member_data_v1 data;
} thinkthen_complete_annotation_member_v1;

typedef struct thinkthen_complete_annotation_member_map_entry_v1 {
  struct thinkthen_complete_utf8_v1 name;
  const struct thinkthen_complete_annotation_member_v1 *value;
} thinkthen_complete_annotation_member_map_entry_v1;

typedef struct thinkthen_complete_annotation_member_map_v1 {
  const struct thinkthen_complete_annotation_member_map_entry_v1 *data;
  size_t len;
} thinkthen_complete_annotation_member_map_v1;

typedef struct thinkthen_complete_position_v1 {
  struct thinkthen_complete_utf8_presence_v1 file;
  struct thinkthen_complete_u64_presence_v1 first;
  struct thinkthen_complete_string_list_presence_v1 images;
  struct thinkthen_complete_u64_presence_v1 last;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_position_v1;

typedef struct thinkthen_complete_position_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_position_v1 *value;
} thinkthen_complete_position_presence_v1;

typedef struct thinkthen_complete_failed_v1 {
  const struct thinkthen_complete_failure_v1 *failed;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_failed_v1;

typedef union thinkthen_complete_annotated_field_data_v1 {
  uint32_t boolean;
  uint32_t null;
  struct thinkthen_complete_utf8_v1 string;
  struct thinkthen_complete_string_list_v1 array;
  double number;
  const struct thinkthen_complete_failed_v1 *object;
} thinkthen_complete_annotated_field_data_v1;

typedef struct thinkthen_complete_annotated_field_v1 {
  uint32_t kind;
  union thinkthen_complete_annotated_field_data_v1 data;
} thinkthen_complete_annotated_field_v1;

typedef struct thinkthen_complete_annotated_field_map_entry_v1 {
  struct thinkthen_complete_utf8_v1 name;
  const struct thinkthen_complete_annotated_field_v1 *value;
} thinkthen_complete_annotated_field_map_entry_v1;

typedef struct thinkthen_complete_annotated_field_map_v1 {
  const struct thinkthen_complete_annotated_field_map_entry_v1 *data;
  size_t len;
} thinkthen_complete_annotated_field_map_v1;

typedef struct thinkthen_complete_annotated_row_v1 {
  struct thinkthen_complete_annotated_field_map_v1 value;
} thinkthen_complete_annotated_row_v1;

typedef struct thinkthen_complete_annotation_v1 {
  const struct thinkthen_complete_answer_id_v1 *answer_id;
  struct thinkthen_complete_annotation_member_map_v1 answers;
  struct thinkthen_complete_utf8_presence_v1 file;
  struct thinkthen_complete_u64_presence_v1 first_line;
  struct thinkthen_complete_u64_presence_v1 index;
  struct thinkthen_complete_json_presence_v1 input;
  struct thinkthen_complete_u64_presence_v1 last_line;
  const struct thinkthen_complete_meta_v1 *meta;
  struct thinkthen_complete_position_presence_v1 position;
  const struct thinkthen_complete_version_v1 *schema;
  struct thinkthen_complete_physical_source_presence_v1 source;
  const struct thinkthen_complete_annotated_row_v1 *value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_annotation_v1;

typedef struct thinkthen_complete_session_packet_annotate_row_v1 {
  struct thinkthen_complete_utf8_v1 function;
  struct thinkthen_complete_utf8_v1 kind;
  const struct thinkthen_complete_annotation_v1 *value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_session_packet_annotate_row_v1;

typedef struct thinkthen_complete_atomic_decide_value_list_v1 {
  const struct thinkthen_complete_atomic_decide_value_v1 *const *data;
  size_t len;
} thinkthen_complete_atomic_decide_value_list_v1;

typedef struct thinkthen_complete_session_packet_decide_aggregate_v1 {
  struct thinkthen_complete_utf8_v1 function;
  struct thinkthen_complete_utf8_v1 kind;
  struct thinkthen_complete_atomic_decide_value_list_v1 value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_session_packet_decide_aggregate_v1;

typedef struct thinkthen_complete_atomic_nullable_string_list_v1 {
  const struct thinkthen_complete_atomic_nullable_string_v1 *const *data;
  size_t len;
} thinkthen_complete_atomic_nullable_string_list_v1;

typedef struct thinkthen_complete_session_packet_choose_aggregate_v1 {
  struct thinkthen_complete_utf8_v1 function;
  struct thinkthen_complete_utf8_v1 kind;
  struct thinkthen_complete_atomic_nullable_string_list_v1 value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_session_packet_choose_aggregate_v1;

typedef struct thinkthen_complete_atomic_array_of_string_list_v1 {
  const struct thinkthen_complete_atomic_array_of_string_v1 *const *data;
  size_t len;
} thinkthen_complete_atomic_array_of_string_list_v1;

typedef struct thinkthen_complete_session_packet_tag_aggregate_v1 {
  struct thinkthen_complete_utf8_v1 function;
  struct thinkthen_complete_utf8_v1 kind;
  struct thinkthen_complete_atomic_array_of_string_list_v1 value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_session_packet_tag_aggregate_v1;

typedef struct thinkthen_complete_atomic_double_list_v1 {
  const struct thinkthen_complete_atomic_double_v1 *const *data;
  size_t len;
} thinkthen_complete_atomic_double_list_v1;

typedef struct thinkthen_complete_session_packet_score_aggregate_v1 {
  struct thinkthen_complete_utf8_v1 function;
  struct thinkthen_complete_utf8_v1 kind;
  struct thinkthen_complete_atomic_double_list_v1 value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_session_packet_score_aggregate_v1;

typedef struct thinkthen_complete_atomic_boolean_list_v1 {
  const struct thinkthen_complete_atomic_boolean_v1 *const *data;
  size_t len;
} thinkthen_complete_atomic_boolean_list_v1;

typedef struct thinkthen_complete_session_packet_filter_aggregate_v1 {
  struct thinkthen_complete_utf8_v1 function;
  struct thinkthen_complete_utf8_v1 kind;
  struct thinkthen_complete_atomic_boolean_list_v1 value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_session_packet_filter_aggregate_v1;

typedef struct thinkthen_complete_atomic_non_zero_usize_v1 {
  const struct thinkthen_complete_answer_v1 *answer;
  const struct thinkthen_complete_answer_id_v1 *answer_id;
  struct thinkthen_complete_image_list_presence_v1 images;
  struct thinkthen_complete_u64_presence_v1 index;
  struct thinkthen_complete_json_presence_v1 input;
  struct thinkthen_complete_rank_member_list_presence_v1 members;
  const struct thinkthen_complete_meta_v1 *meta;
  const struct thinkthen_complete_question_document_v1 *question;
  struct thinkthen_complete_utf8_presence_v1 question_name;
  const struct thinkthen_complete_version_v1 *schema;
  struct thinkthen_complete_physical_source_presence_v1 source;
  struct thinkthen_complete_threshold_presence_v1 threshold;
  uint64_t value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_atomic_non_zero_usize_v1;

typedef struct thinkthen_complete_atomic_non_zero_usize_list_v1 {
  const struct thinkthen_complete_atomic_non_zero_usize_v1 *const *data;
  size_t len;
} thinkthen_complete_atomic_non_zero_usize_list_v1;

typedef struct thinkthen_complete_session_packet_rank_aggregate_v1 {
  struct thinkthen_complete_utf8_v1 function;
  struct thinkthen_complete_utf8_v1 kind;
  struct thinkthen_complete_atomic_non_zero_usize_list_v1 value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_session_packet_rank_aggregate_v1;

typedef struct thinkthen_complete_find_answer_v1 {
  struct thinkthen_complete_f64_presence_v1 confidence;
  struct thinkthen_complete_utf8_v1 kind;
  struct thinkthen_complete_utf8_v1 pick;
  struct thinkthen_complete_number_map_v1 probabilities;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_find_answer_v1;

typedef struct thinkthen_complete_find_candidate_v1 {
  struct thinkthen_complete_u64_presence_v1 index;
  struct thinkthen_complete_json_presence_v1 input;
  double probability;
  struct thinkthen_complete_physical_source_presence_v1 source;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_find_candidate_v1;

typedef struct thinkthen_complete_find_candidate_list_v1 {
  const struct thinkthen_complete_find_candidate_v1 *const *data;
  size_t len;
} thinkthen_complete_find_candidate_list_v1;

typedef struct thinkthen_complete_find_candidate_list_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_find_candidate_list_v1 value;
} thinkthen_complete_find_candidate_list_presence_v1;

typedef struct thinkthen_complete_find_question_document_v1 {
  struct thinkthen_complete_batch_presence_v1 batch;
  struct thinkthen_complete_input_declaration_presence_v1 context_schema;
  struct thinkthen_complete_input_declaration_presence_v1 item_schema;
  struct thinkthen_complete_label_list_presence_v1 label_details;
  struct thinkthen_complete_utf8_presence_v1 model;
  struct thinkthen_complete_question_name_presence_v1 name;
  uint32_t none;
  struct thinkthen_complete_string_list_presence_v1 on;
  struct thinkthen_complete_utf8_presence_v1 profile;
  struct thinkthen_complete_json_presence_v1 text;
  struct thinkthen_complete_utf8_v1 verb;
  struct thinkthen_complete_wording_version_presence_v1 wording_version;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_find_question_document_v1;

typedef struct thinkthen_complete_find_v1 {
  const struct thinkthen_complete_find_answer_v1 *answer;
  const struct thinkthen_complete_answer_id_v1 *answer_id;
  struct thinkthen_complete_find_candidate_list_presence_v1 candidates;
  struct thinkthen_complete_utf8_presence_v1 file;
  struct thinkthen_complete_u64_presence_v1 first_line;
  struct thinkthen_complete_u64_presence_v1 index;
  struct thinkthen_complete_u64_presence_v1 last_line;
  const struct thinkthen_complete_meta_v1 *meta;
  struct thinkthen_complete_position_presence_v1 position;
  const struct thinkthen_complete_find_question_document_v1 *question;
  const struct thinkthen_complete_version_v1 *schema;
  struct thinkthen_complete_json_presence_v1 threshold;
  struct thinkthen_complete_json_presence_v1 value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_find_v1;

typedef struct thinkthen_complete_session_packet_find_aggregate_v1 {
  struct thinkthen_complete_utf8_v1 function;
  struct thinkthen_complete_utf8_v1 kind;
  const struct thinkthen_complete_find_v1 *value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_session_packet_find_aggregate_v1;

typedef struct thinkthen_complete_annotation_list_v1 {
  const struct thinkthen_complete_annotation_v1 *const *data;
  size_t len;
} thinkthen_complete_annotation_list_v1;

typedef struct thinkthen_complete_session_packet_annotate_aggregate_v1 {
  struct thinkthen_complete_utf8_v1 function;
  struct thinkthen_complete_utf8_v1 kind;
  struct thinkthen_complete_annotation_list_v1 value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_session_packet_annotate_aggregate_v1;

typedef struct thinkthen_complete_number_map_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_number_map_v1 value;
} thinkthen_complete_number_map_presence_v1;

typedef struct thinkthen_complete_name_odds_v1 {
  struct thinkthen_complete_number_map_presence_v1 edges;
  uint64_t end;
  struct thinkthen_complete_number_map_presence_v1 kinds;
  uint64_t start;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_name_odds_v1;

typedef struct thinkthen_complete_name_odds_list_v1 {
  const struct thinkthen_complete_name_odds_v1 *const *data;
  size_t len;
} thinkthen_complete_name_odds_list_v1;

typedef struct thinkthen_complete_place_v1 {
  uint64_t end;
  uint64_t start;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_place_v1;

typedef struct thinkthen_complete_pair_odds_v1 {
  double probability;
  struct thinkthen_complete_utf8_v1 relation;
  const struct thinkthen_complete_place_v1 *source;
  const struct thinkthen_complete_place_v1 *target;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_pair_odds_v1;

typedef struct thinkthen_complete_pair_odds_list_v1 {
  const struct thinkthen_complete_pair_odds_v1 *const *data;
  size_t len;
} thinkthen_complete_pair_odds_list_v1;

typedef struct thinkthen_complete_piece_odds_v1 {
  uint64_t end;
  uint64_t start;
  struct thinkthen_complete_number_map_v1 tags;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_piece_odds_v1;

typedef struct thinkthen_complete_piece_odds_list_v1 {
  const struct thinkthen_complete_piece_odds_v1 *const *data;
  size_t len;
} thinkthen_complete_piece_odds_list_v1;

typedef struct thinkthen_complete_place_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_place_v1 *value;
} thinkthen_complete_place_presence_v1;

typedef struct thinkthen_complete_recognition_proposal_v1 {
  uint64_t end;
  uint32_t kept;
  struct thinkthen_complete_utf8_presence_v1 kind;
  struct thinkthen_complete_place_presence_v1 selected;
  double span_probability;
  uint64_t start;
  struct thinkthen_complete_f64_presence_v1 strength;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_recognition_proposal_v1;

typedef struct thinkthen_complete_recognition_proposal_list_v1 {
  const struct thinkthen_complete_recognition_proposal_v1 *const *data;
  size_t len;
} thinkthen_complete_recognition_proposal_list_v1;

typedef struct thinkthen_complete_recognition_odds_recognize_answer_v1 {
  struct thinkthen_complete_name_odds_list_v1 names;
  struct thinkthen_complete_pair_odds_list_v1 pairs;
  struct thinkthen_complete_piece_odds_list_v1 pieces;
  struct thinkthen_complete_recognition_proposal_list_v1 proposals;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_recognition_odds_recognize_answer_v1;

typedef struct thinkthen_complete_boundary_proposal_v1 {
  uint64_t end;
  struct thinkthen_complete_utf8_presence_v1 file;
  struct thinkthen_complete_u64_presence_v1 first_line;
  struct thinkthen_complete_u64_presence_v1 last_line;
  uint64_t length;
  double probability;
  uint64_t start;
  struct thinkthen_complete_utf8_v1 text;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_boundary_proposal_v1;

typedef struct thinkthen_complete_boundary_proposal_list_v1 {
  const struct thinkthen_complete_boundary_proposal_v1 *const *data;
  size_t len;
} thinkthen_complete_boundary_proposal_list_v1;

typedef struct thinkthen_complete_recognition_odds_boundary_odds_v1 {
  struct thinkthen_complete_piece_odds_list_v1 pieces;
  struct thinkthen_complete_boundary_proposal_list_v1 proposals;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_recognition_odds_boundary_odds_v1;

typedef union thinkthen_complete_recognition_odds_data_v1 {
  const struct thinkthen_complete_recognition_odds_recognize_answer_v1 *recognize_answer;
  const struct thinkthen_complete_recognition_odds_boundary_odds_v1 *boundary_odds;
} thinkthen_complete_recognition_odds_data_v1;

typedef struct thinkthen_complete_recognition_odds_v1 {
  uint32_t kind;
  union thinkthen_complete_recognition_odds_data_v1 data;
} thinkthen_complete_recognition_odds_v1;

typedef struct thinkthen_complete_json_map_entry_v1 {
  struct thinkthen_complete_utf8_v1 name;
  const struct thinkthen_complete_json_v1 *value;
} thinkthen_complete_json_map_entry_v1;

typedef struct thinkthen_complete_json_map_v1 {
  const struct thinkthen_complete_json_map_entry_v1 *data;
  size_t len;
} thinkthen_complete_json_map_v1;

typedef struct thinkthen_complete_recognition_mode_v1 {
  uint32_t kind;
} thinkthen_complete_recognition_mode_v1;

typedef struct thinkthen_complete_recognition_mode_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_recognition_mode_v1 *value;
} thinkthen_complete_recognition_mode_presence_v1;

typedef struct thinkthen_complete_relation_rule_v1 {
  uint32_t either;
  struct thinkthen_complete_utf8_v1 name;
  struct thinkthen_complete_utf8_v1 reads;
  struct thinkthen_complete_u32_presence_v1 single;
  struct thinkthen_complete_utf8_v1 source;
  struct thinkthen_complete_utf8_v1 target;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_relation_rule_v1;

typedef struct thinkthen_complete_relation_rule_list_v1 {
  const struct thinkthen_complete_relation_rule_v1 *const *data;
  size_t len;
} thinkthen_complete_relation_rule_list_v1;

typedef struct thinkthen_complete_relation_rule_list_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_relation_rule_list_v1 value;
} thinkthen_complete_relation_rule_list_presence_v1;

typedef struct thinkthen_complete_recognition_stage_context_v1 {
  struct thinkthen_complete_utf8_presence_v1 boundary;
  struct thinkthen_complete_utf8_presence_v1 kind_edge;
  struct thinkthen_complete_utf8_presence_v1 relation;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_recognition_stage_context_v1;

typedef struct thinkthen_complete_recognition_stage_context_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_recognition_stage_context_v1 *value;
} thinkthen_complete_recognition_stage_context_presence_v1;

typedef struct thinkthen_complete_verb_v1 {
  uint32_t kind;
} thinkthen_complete_verb_v1;

typedef struct thinkthen_complete_recognize_question_document_v1 {
  struct thinkthen_complete_batch_presence_v1 batch;
  struct thinkthen_complete_input_declaration_presence_v1 context_schema;
  struct thinkthen_complete_json_presence_v1 entity_definition;
  struct thinkthen_complete_json_presence_v1 instructions;
  struct thinkthen_complete_input_declaration_presence_v1 item_schema;
  struct thinkthen_complete_json_map_v1 kinds;
  struct thinkthen_complete_label_list_presence_v1 label_details;
  struct thinkthen_complete_recognition_mode_presence_v1 mode;
  struct thinkthen_complete_utf8_presence_v1 model;
  struct thinkthen_complete_question_name_presence_v1 name;
  struct thinkthen_complete_string_list_presence_v1 on;
  struct thinkthen_complete_utf8_presence_v1 profile;
  struct thinkthen_complete_threshold_presence_v1 relation_threshold;
  struct thinkthen_complete_relation_rule_list_presence_v1 relations;
  struct thinkthen_complete_u64_presence_v1 snippet_pieces;
  struct thinkthen_complete_recognition_stage_context_presence_v1 stage_context;
  const struct thinkthen_complete_threshold_v1 *threshold;
  const struct thinkthen_complete_verb_v1 *verb;
  struct thinkthen_complete_wording_version_presence_v1 wording_version;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_recognize_question_document_v1;

typedef struct thinkthen_complete_entity_v1 {
  uint64_t end;
  struct thinkthen_complete_utf8_presence_v1 file;
  struct thinkthen_complete_u64_presence_v1 first_line;
  struct thinkthen_complete_utf8_v1 kind;
  struct thinkthen_complete_u64_presence_v1 last_line;
  uint64_t length;
  uint64_t start;
  double strength;
  struct thinkthen_complete_utf8_v1 text;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_entity_v1;

typedef struct thinkthen_complete_entity_list_v1 {
  const struct thinkthen_complete_entity_v1 *const *data;
  size_t len;
} thinkthen_complete_entity_list_v1;

typedef struct thinkthen_complete_entity_edge_v1 {
  struct thinkthen_complete_u32_presence_v1 either;
  double probability;
  struct thinkthen_complete_utf8_v1 relation;
  const struct thinkthen_complete_entity_v1 *source;
  const struct thinkthen_complete_entity_v1 *target;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_entity_edge_v1;

typedef struct thinkthen_complete_entity_edge_list_v1 {
  const struct thinkthen_complete_entity_edge_v1 *const *data;
  size_t len;
} thinkthen_complete_entity_edge_list_v1;

typedef struct thinkthen_complete_entity_edge_list_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_entity_edge_list_v1 value;
} thinkthen_complete_entity_edge_list_presence_v1;

typedef struct thinkthen_complete_recognize_entities_v1 {
  struct thinkthen_complete_entity_list_v1 entities;
  struct thinkthen_complete_entity_edge_list_presence_v1 relations;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_recognize_entities_v1;

typedef struct thinkthen_complete_boundary_mode_v1 {
  uint32_t kind;
} thinkthen_complete_boundary_mode_v1;

typedef struct thinkthen_complete_recognize_mode_proposals_v1 {
  const struct thinkthen_complete_boundary_mode_v1 *mode;
  struct thinkthen_complete_boundary_proposal_list_v1 proposals;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_recognize_mode_proposals_v1;

typedef union thinkthen_complete_recognize_data_v1 {
  const struct thinkthen_complete_recognize_entities_v1 *entities;
  const struct thinkthen_complete_recognize_mode_proposals_v1 *mode_proposals;
} thinkthen_complete_recognize_data_v1;

typedef struct thinkthen_complete_recognize_v1 {
  uint32_t kind;
  union thinkthen_complete_recognize_data_v1 data;
} thinkthen_complete_recognize_v1;

typedef struct thinkthen_complete_recognition_v1 {
  const struct thinkthen_complete_recognition_odds_v1 *answer;
  const struct thinkthen_complete_answer_id_v1 *answer_id;
  struct thinkthen_complete_utf8_presence_v1 file;
  struct thinkthen_complete_u64_presence_v1 first_line;
  struct thinkthen_complete_u64_presence_v1 index;
  struct thinkthen_complete_json_presence_v1 input;
  struct thinkthen_complete_u64_presence_v1 last_line;
  const struct thinkthen_complete_meta_v1 *meta;
  struct thinkthen_complete_position_presence_v1 position;
  const struct thinkthen_complete_recognize_question_document_v1 *question;
  const struct thinkthen_complete_version_v1 *schema;
  struct thinkthen_complete_physical_source_presence_v1 source;
  const struct thinkthen_complete_recognize_v1 *value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_recognition_v1;

typedef struct thinkthen_complete_recognition_list_v1 {
  const struct thinkthen_complete_recognition_v1 *const *data;
  size_t len;
} thinkthen_complete_recognition_list_v1;

typedef struct thinkthen_complete_session_packet_recognize_aggregate_v1 {
  struct thinkthen_complete_utf8_v1 function;
  struct thinkthen_complete_utf8_v1 kind;
  struct thinkthen_complete_recognition_list_v1 value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_session_packet_recognize_aggregate_v1;

typedef struct thinkthen_complete_relation_direction_v1 {
  uint32_t kind;
} thinkthen_complete_relation_direction_v1;

typedef struct thinkthen_complete_relation_method_v1 {
  uint32_t kind;
} thinkthen_complete_relation_method_v1;

typedef struct thinkthen_complete_related_entity_v1 {
  struct thinkthen_complete_utf8_v1 kind;
  struct thinkthen_complete_utf8_v1 name;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_related_entity_v1;

typedef struct thinkthen_complete_related_entity_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_related_entity_v1 *value;
} thinkthen_complete_related_entity_presence_v1;

typedef struct thinkthen_complete_relation_member_answer_id_v1 {
  const struct thinkthen_complete_relation_direction_v1 *direction;
  const struct thinkthen_complete_relation_method_v1 *method;
  struct thinkthen_complete_observation_list_v1 observations;
  const struct thinkthen_complete_question_document_v1 *question;
  struct thinkthen_complete_question_source_list_v1 question_sources;
  struct thinkthen_complete_utf8_v1 reads;
  struct thinkthen_complete_utf8_v1 relation;
  struct thinkthen_complete_utf8_v1 request;
  const struct thinkthen_complete_related_entity_v1 *source;
  struct thinkthen_complete_related_entity_presence_v1 target;
  const struct thinkthen_complete_threshold_v1 *threshold;
  struct thinkthen_complete_usage_presence_v1 usage;
  uint32_t accepted;
  const struct thinkthen_complete_answer_v1 *answer;
  const struct thinkthen_complete_answer_id_v1 *answer_id;
  double probability;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_relation_member_answer_id_v1;

typedef struct thinkthen_complete_relation_member_failure_id_v1 {
  const struct thinkthen_complete_relation_direction_v1 *direction;
  const struct thinkthen_complete_relation_method_v1 *method;
  struct thinkthen_complete_observation_list_v1 observations;
  const struct thinkthen_complete_question_document_v1 *question;
  struct thinkthen_complete_question_source_list_v1 question_sources;
  struct thinkthen_complete_utf8_v1 reads;
  struct thinkthen_complete_utf8_v1 relation;
  struct thinkthen_complete_utf8_v1 request;
  const struct thinkthen_complete_related_entity_v1 *source;
  struct thinkthen_complete_related_entity_presence_v1 target;
  const struct thinkthen_complete_threshold_v1 *threshold;
  struct thinkthen_complete_usage_presence_v1 usage;
  const struct thinkthen_complete_failure_v1 *failure;
  const struct thinkthen_complete_failure_id_v1 *failure_id;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_relation_member_failure_id_v1;

typedef union thinkthen_complete_relation_member_data_v1 {
  const struct thinkthen_complete_relation_member_answer_id_v1 *answer_id;
  const struct thinkthen_complete_relation_member_failure_id_v1 *failure_id;
} thinkthen_complete_relation_member_data_v1;

typedef struct thinkthen_complete_relation_member_v1 {
  uint32_t kind;
  union thinkthen_complete_relation_member_data_v1 data;
} thinkthen_complete_relation_member_v1;

typedef struct thinkthen_complete_relation_member_list_v1 {
  const struct thinkthen_complete_relation_member_v1 *const *data;
  size_t len;
} thinkthen_complete_relation_member_list_v1;

typedef struct thinkthen_complete_answers_v1 {
  struct thinkthen_complete_relation_member_list_v1 questions;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_answers_v1;

typedef struct thinkthen_complete_session_input_source_v1 {
  uint64_t index;
  const struct thinkthen_complete_physical_source_v1 *source;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_session_input_source_v1;

typedef struct thinkthen_complete_session_input_source_list_v1 {
  const struct thinkthen_complete_session_input_source_v1 *const *data;
  size_t len;
} thinkthen_complete_session_input_source_list_v1;

typedef struct thinkthen_complete_session_input_source_list_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_session_input_source_list_v1 value;
} thinkthen_complete_session_input_source_list_presence_v1;

typedef struct thinkthen_complete_relate_fields_v1 {
  struct thinkthen_complete_utf8_v1 kind;
  struct thinkthen_complete_utf8_v1 name;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_relate_fields_v1;

typedef struct thinkthen_complete_relate_fields_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_relate_fields_v1 *value;
} thinkthen_complete_relate_fields_presence_v1;

typedef struct thinkthen_complete_relate_question_document_v1 {
  struct thinkthen_complete_batch_presence_v1 batch;
  struct thinkthen_complete_input_declaration_presence_v1 context_schema;
  struct thinkthen_complete_relate_fields_presence_v1 fields;
  struct thinkthen_complete_input_declaration_presence_v1 item_schema;
  struct thinkthen_complete_label_list_presence_v1 label_details;
  struct thinkthen_complete_utf8_presence_v1 model;
  struct thinkthen_complete_question_name_presence_v1 name;
  struct thinkthen_complete_string_list_presence_v1 on;
  struct thinkthen_complete_utf8_presence_v1 profile;
  struct thinkthen_complete_relation_rule_list_v1 relations;
  const struct thinkthen_complete_threshold_v1 *threshold;
  struct thinkthen_complete_utf8_v1 verb;
  struct thinkthen_complete_wording_version_presence_v1 wording_version;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_relate_question_document_v1;

typedef struct thinkthen_complete_relation_endpoint_related_entity_v1 {
  struct thinkthen_complete_utf8_v1 kind;
  struct thinkthen_complete_utf8_v1 name;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_relation_endpoint_related_entity_v1;

typedef struct thinkthen_complete_relation_endpoint_source_v1 {
  struct thinkthen_complete_utf8_presence_v1 file;
  struct thinkthen_complete_u64_presence_v1 first_line;
  struct thinkthen_complete_utf8_v1 kind;
  struct thinkthen_complete_u64_presence_v1 last_line;
  struct thinkthen_complete_utf8_v1 name;
  uint64_t ordinal;
  struct thinkthen_complete_json_presence_v1 record;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_relation_endpoint_source_v1;

typedef union thinkthen_complete_relation_endpoint_data_v1 {
  const struct thinkthen_complete_relation_endpoint_related_entity_v1 *related_entity;
  const struct thinkthen_complete_relation_endpoint_source_v1 *source;
} thinkthen_complete_relation_endpoint_data_v1;

typedef struct thinkthen_complete_relation_endpoint_v1 {
  uint32_t kind;
  union thinkthen_complete_relation_endpoint_data_v1 data;
} thinkthen_complete_relation_endpoint_v1;

typedef struct thinkthen_complete_related_entity_edge_v1 {
  struct thinkthen_complete_u32_presence_v1 either;
  double probability;
  struct thinkthen_complete_utf8_v1 relation;
  const struct thinkthen_complete_relation_endpoint_v1 *source;
  const struct thinkthen_complete_relation_endpoint_v1 *target;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_related_entity_edge_v1;

typedef struct thinkthen_complete_related_entity_edge_list_v1 {
  const struct thinkthen_complete_related_entity_edge_v1 *const *data;
  size_t len;
} thinkthen_complete_related_entity_edge_list_v1;

typedef struct thinkthen_complete_relation_v1 {
  const struct thinkthen_complete_answers_v1 *answer;
  const struct thinkthen_complete_answer_id_v1 *answer_id;
  struct thinkthen_complete_utf8_presence_v1 file;
  struct thinkthen_complete_u64_presence_v1 first_line;
  struct thinkthen_complete_u64_presence_v1 index;
  struct thinkthen_complete_json_presence_v1 input;
  struct thinkthen_complete_session_input_source_list_presence_v1 input_sources;
  struct thinkthen_complete_u64_presence_v1 last_line;
  const struct thinkthen_complete_meta_v1 *meta;
  struct thinkthen_complete_position_presence_v1 position;
  const struct thinkthen_complete_relate_question_document_v1 *question;
  const struct thinkthen_complete_version_v1 *schema;
  struct thinkthen_complete_related_entity_edge_list_v1 value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_relation_v1;

typedef struct thinkthen_complete_session_packet_relate_aggregate_v1 {
  struct thinkthen_complete_utf8_v1 function;
  struct thinkthen_complete_utf8_v1 kind;
  const struct thinkthen_complete_relation_v1 *value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_session_packet_relate_aggregate_v1;

typedef struct thinkthen_complete_request_function_v1 {
  uint32_t kind;
} thinkthen_complete_request_function_v1;

typedef struct thinkthen_complete_answer_id_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_answer_id_v1 *value;
} thinkthen_complete_answer_id_presence_v1;

typedef struct thinkthen_complete_failure_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_failure_v1 *value;
} thinkthen_complete_failure_presence_v1;

typedef struct thinkthen_complete_failure_id_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_failure_id_v1 *value;
} thinkthen_complete_failure_id_presence_v1;

typedef struct thinkthen_complete_json_list_v1 {
  const struct thinkthen_complete_json_v1 *const *data;
  size_t len;
} thinkthen_complete_json_list_v1;

typedef struct thinkthen_complete_session_probabilities_yes_no_v1 {
  struct thinkthen_complete_utf8_v1 kind;
  double value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_session_probabilities_yes_no_v1;

typedef struct thinkthen_complete_session_named_probability_v1 {
  struct thinkthen_complete_utf8_v1 name;
  double probability;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_session_named_probability_v1;

typedef struct thinkthen_complete_session_named_probability_list_v1 {
  const struct thinkthen_complete_session_named_probability_v1 *const *data;
  size_t len;
} thinkthen_complete_session_named_probability_list_v1;

typedef struct thinkthen_complete_session_probabilities_named_v1 {
  struct thinkthen_complete_utf8_v1 kind;
  struct thinkthen_complete_session_named_probability_list_v1 value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_session_probabilities_named_v1;

typedef union thinkthen_complete_session_probabilities_data_v1 {
  const struct thinkthen_complete_session_probabilities_yes_no_v1 *yes_no;
  const struct thinkthen_complete_session_probabilities_named_v1 *named;
} thinkthen_complete_session_probabilities_data_v1;

typedef struct thinkthen_complete_session_probabilities_v1 {
  uint32_t kind;
  union thinkthen_complete_session_probabilities_data_v1 data;
} thinkthen_complete_session_probabilities_v1;

typedef struct thinkthen_complete_session_probabilities_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_session_probabilities_v1 *value;
} thinkthen_complete_session_probabilities_presence_v1;

typedef struct thinkthen_complete_token_usage_v1 {
  uint64_t input_tokens;
  uint64_t output_tokens;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_token_usage_v1;

typedef struct thinkthen_complete_token_usage_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_token_usage_v1 *value;
} thinkthen_complete_token_usage_presence_v1;

typedef struct thinkthen_complete_session_question_detail_v1 {
  struct thinkthen_complete_answer_id_presence_v1 answer_id;
  uint32_t cached;
  struct thinkthen_complete_f64_presence_v1 confidence;
  uint64_t failed_questions;
  struct thinkthen_complete_failure_presence_v1 failure;
  struct thinkthen_complete_failure_id_presence_v1 failure_id;
  struct thinkthen_complete_json_presence_v1 input;
  struct thinkthen_complete_physical_source_presence_v1 input_source;
  struct thinkthen_complete_session_input_source_list_v1 input_sources;
  struct thinkthen_complete_json_list_v1 inputs;
  struct thinkthen_complete_utf8_v1 model;
  struct thinkthen_complete_observation_list_v1 observations;
  struct thinkthen_complete_session_probabilities_presence_v1 probabilities;
  const struct thinkthen_complete_question_document_v1 *question;
  struct thinkthen_complete_utf8_v1 question_sha256;
  struct thinkthen_complete_question_source_list_v1 question_sources;
  struct thinkthen_complete_utf8_presence_v1 raw_pick;
  struct thinkthen_complete_usage_presence_v1 reported_usage;
  struct thinkthen_complete_string_list_v1 requests;
  uint64_t requests_sent;
  struct thinkthen_complete_threshold_presence_v1 threshold;
  struct thinkthen_complete_utf8_v1 url;
  struct thinkthen_complete_token_usage_presence_v1 usage;
  struct thinkthen_complete_value_presence_v1 value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_session_question_detail_v1;

typedef struct thinkthen_complete_session_observation_question_v1 {
  const struct thinkthen_complete_session_question_detail_v1 *detail;
  uint64_t index;
  struct thinkthen_complete_utf8_v1 kind;
  struct thinkthen_complete_utf8_presence_v1 member;
  uint64_t position;
  struct thinkthen_complete_utf8_presence_v1 stage;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_session_observation_question_v1;

typedef struct thinkthen_complete_session_judgment_decision_v1 {
  struct thinkthen_complete_utf8_v1 kind;
  struct thinkthen_complete_u32_presence_v1 value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_session_judgment_decision_v1;

typedef struct thinkthen_complete_session_judgment_choice_v1 {
  struct thinkthen_complete_utf8_v1 kind;
  struct thinkthen_complete_utf8_presence_v1 value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_session_judgment_choice_v1;

typedef struct thinkthen_complete_session_judgment_score_v1 {
  struct thinkthen_complete_utf8_v1 kind;
  double value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_session_judgment_score_v1;

typedef struct thinkthen_complete_session_judgment_tags_v1 {
  struct thinkthen_complete_utf8_v1 kind;
  struct thinkthen_complete_string_list_v1 value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_session_judgment_tags_v1;

typedef union thinkthen_complete_session_judgment_data_v1 {
  const struct thinkthen_complete_session_judgment_decision_v1 *decision;
  const struct thinkthen_complete_session_judgment_choice_v1 *choice;
  const struct thinkthen_complete_session_judgment_score_v1 *score;
  const struct thinkthen_complete_session_judgment_tags_v1 *tags;
} thinkthen_complete_session_judgment_data_v1;

typedef struct thinkthen_complete_session_judgment_v1 {
  uint32_t kind;
  union thinkthen_complete_session_judgment_data_v1 data;
} thinkthen_complete_session_judgment_v1;

typedef struct thinkthen_complete_session_observed_row_judgment_v1 {
  struct thinkthen_complete_utf8_v1 kind;
  const struct thinkthen_complete_session_judgment_v1 *value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_session_observed_row_judgment_v1;

typedef struct thinkthen_complete_annotation_value_decision_v1 {
  struct thinkthen_complete_utf8_v1 kind;
  struct thinkthen_complete_u32_presence_v1 value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_annotation_value_decision_v1;

typedef struct thinkthen_complete_annotation_value_choice_v1 {
  struct thinkthen_complete_utf8_v1 kind;
  struct thinkthen_complete_utf8_presence_v1 value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_annotation_value_choice_v1;

typedef struct thinkthen_complete_annotation_value_score_v1 {
  struct thinkthen_complete_utf8_v1 kind;
  double value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_annotation_value_score_v1;

typedef struct thinkthen_complete_annotation_value_tags_v1 {
  struct thinkthen_complete_utf8_v1 kind;
  struct thinkthen_complete_string_list_v1 value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_annotation_value_tags_v1;

typedef struct thinkthen_complete_annotation_value_failed_v1 {
  struct thinkthen_complete_utf8_v1 kind;
  const struct thinkthen_complete_failure_v1 *value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_annotation_value_failed_v1;

typedef union thinkthen_complete_annotation_value_data_v1 {
  const struct thinkthen_complete_annotation_value_decision_v1 *decision;
  const struct thinkthen_complete_annotation_value_choice_v1 *choice;
  const struct thinkthen_complete_annotation_value_score_v1 *score;
  const struct thinkthen_complete_annotation_value_tags_v1 *tags;
  const struct thinkthen_complete_annotation_value_failed_v1 *failed;
} thinkthen_complete_annotation_value_data_v1;

typedef struct thinkthen_complete_annotation_value_v1 {
  uint32_t kind;
  union thinkthen_complete_annotation_value_data_v1 data;
} thinkthen_complete_annotation_value_v1;

typedef struct thinkthen_complete_session_annotation_v1 {
  struct thinkthen_complete_utf8_v1 name;
  const struct thinkthen_complete_annotation_value_v1 *value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_session_annotation_v1;

typedef struct thinkthen_complete_session_annotation_list_v1 {
  const struct thinkthen_complete_session_annotation_v1 *const *data;
  size_t len;
} thinkthen_complete_session_annotation_list_v1;

typedef struct thinkthen_complete_session_observed_row_annotated_v1 {
  struct thinkthen_complete_utf8_v1 kind;
  struct thinkthen_complete_session_annotation_list_v1 value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_session_observed_row_annotated_v1;

typedef struct thinkthen_complete_boundary_proposal_list_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_boundary_proposal_list_v1 value;
} thinkthen_complete_boundary_proposal_list_presence_v1;

typedef struct thinkthen_complete_recognition_edge_v1 {
  uint32_t either;
  double probability;
  struct thinkthen_complete_utf8_v1 relation;
  const struct thinkthen_complete_entity_v1 *source;
  const struct thinkthen_complete_entity_v1 *target;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_recognition_edge_v1;

typedef struct thinkthen_complete_recognition_edge_list_v1 {
  const struct thinkthen_complete_recognition_edge_v1 *const *data;
  size_t len;
} thinkthen_complete_recognition_edge_list_v1;

typedef struct thinkthen_complete_recognition_edge_list_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_recognition_edge_list_v1 value;
} thinkthen_complete_recognition_edge_list_presence_v1;

typedef struct thinkthen_complete_session_recognition_v1 {
  struct thinkthen_complete_entity_list_v1 entities;
  const struct thinkthen_complete_recognition_mode_v1 *mode;
  struct thinkthen_complete_boundary_proposal_list_presence_v1 proposals;
  struct thinkthen_complete_recognition_edge_list_presence_v1 relations;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_session_recognition_v1;

typedef struct thinkthen_complete_session_observed_row_recognized_v1 {
  struct thinkthen_complete_utf8_v1 kind;
  const struct thinkthen_complete_session_recognition_v1 *value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_session_observed_row_recognized_v1;

typedef struct thinkthen_complete_session_observed_row_find_v1 {
  struct thinkthen_complete_utf8_v1 kind;
  struct thinkthen_complete_u64_presence_v1 value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_session_observed_row_find_v1;

typedef struct thinkthen_complete_entity_document_v1 {
  struct thinkthen_complete_utf8_v1 kind;
  struct thinkthen_complete_utf8_v1 name;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_entity_document_v1;

typedef struct thinkthen_complete_session_relation_edge_v1 {
  uint32_t either;
  double probability;
  struct thinkthen_complete_utf8_v1 relation;
  const struct thinkthen_complete_entity_document_v1 *source;
  const struct thinkthen_complete_entity_document_v1 *target;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_session_relation_edge_v1;

typedef struct thinkthen_complete_session_relation_edge_list_v1 {
  const struct thinkthen_complete_session_relation_edge_v1 *const *data;
  size_t len;
} thinkthen_complete_session_relation_edge_list_v1;

typedef struct thinkthen_complete_session_observed_row_relations_v1 {
  struct thinkthen_complete_utf8_v1 kind;
  struct thinkthen_complete_session_relation_edge_list_v1 value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_session_observed_row_relations_v1;

typedef union thinkthen_complete_session_observed_row_data_v1 {
  const struct thinkthen_complete_session_observed_row_judgment_v1 *judgment;
  const struct thinkthen_complete_session_observed_row_annotated_v1 *annotated;
  const struct thinkthen_complete_session_observed_row_recognized_v1 *recognized;
  const struct thinkthen_complete_session_observed_row_find_v1 *find;
  const struct thinkthen_complete_session_observed_row_relations_v1 *relations;
} thinkthen_complete_session_observed_row_data_v1;

typedef struct thinkthen_complete_session_observed_row_v1 {
  uint32_t kind;
  union thinkthen_complete_session_observed_row_data_v1 data;
} thinkthen_complete_session_observed_row_v1;

typedef struct thinkthen_complete_session_observation_row_v1 {
  uint64_t index;
  struct thinkthen_complete_utf8_v1 kind;
  const struct thinkthen_complete_session_observed_row_v1 *value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_session_observation_row_v1;

typedef union thinkthen_complete_session_observation_data_v1 {
  const struct thinkthen_complete_session_observation_question_v1 *question;
  const struct thinkthen_complete_session_observation_row_v1 *row;
} thinkthen_complete_session_observation_data_v1;

typedef struct thinkthen_complete_session_observation_v1 {
  uint32_t kind;
  union thinkthen_complete_session_observation_data_v1 data;
} thinkthen_complete_session_observation_v1;

typedef struct thinkthen_complete_session_packet_observation_v1 {
  const struct thinkthen_complete_request_function_v1 *function;
  struct thinkthen_complete_utf8_v1 kind;
  const struct thinkthen_complete_session_observation_v1 *value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_session_packet_observation_v1;

typedef union thinkthen_complete_answer_shape_4_any_of_1_items_data_v1 {
  uint64_t integer;
  uint32_t null;
} thinkthen_complete_answer_shape_4_any_of_1_items_data_v1;

typedef struct thinkthen_complete_answer_shape_4_any_of_1_items_v1 {
  uint32_t kind;
  union thinkthen_complete_answer_shape_4_any_of_1_items_data_v1 data;
} thinkthen_complete_answer_shape_4_any_of_1_items_v1;

typedef struct thinkthen_complete_answer_shape_4_any_of_1_items_list_v1 {
  const struct thinkthen_complete_answer_shape_4_any_of_1_items_v1 *const *data;
  size_t len;
} thinkthen_complete_answer_shape_4_any_of_1_items_list_v1;

typedef union thinkthen_complete_answer_shape_4_data_v1 {
  uint64_t integer;
  uint32_t null;
  struct thinkthen_complete_answer_shape_4_any_of_1_items_list_v1 array;
} thinkthen_complete_answer_shape_4_data_v1;

typedef struct thinkthen_complete_answer_shape_4_v1 {
  uint32_t kind;
  union thinkthen_complete_answer_shape_4_data_v1 data;
} thinkthen_complete_answer_shape_4_v1;

typedef struct thinkthen_complete_answer_shape_4_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_answer_shape_4_v1 *value;
} thinkthen_complete_answer_shape_4_presence_v1;

typedef union thinkthen_complete_answer_shape_6_any_of_1_items_data_v1 {
  struct thinkthen_complete_utf8_v1 string;
  uint32_t null;
} thinkthen_complete_answer_shape_6_any_of_1_items_data_v1;

typedef struct thinkthen_complete_answer_shape_6_any_of_1_items_v1 {
  uint32_t kind;
  union thinkthen_complete_answer_shape_6_any_of_1_items_data_v1 data;
} thinkthen_complete_answer_shape_6_any_of_1_items_v1;

typedef struct thinkthen_complete_answer_shape_6_any_of_1_items_list_v1 {
  const struct thinkthen_complete_answer_shape_6_any_of_1_items_v1 *const *data;
  size_t len;
} thinkthen_complete_answer_shape_6_any_of_1_items_list_v1;

typedef union thinkthen_complete_answer_shape_3_data_v1 {
  struct thinkthen_complete_utf8_v1 string;
  uint32_t null;
  struct thinkthen_complete_answer_shape_6_any_of_1_items_list_v1 array;
} thinkthen_complete_answer_shape_3_data_v1;

typedef struct thinkthen_complete_answer_shape_3_v1 {
  uint32_t kind;
  union thinkthen_complete_answer_shape_3_data_v1 data;
} thinkthen_complete_answer_shape_3_v1;

typedef struct thinkthen_complete_answer_shape_3_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_answer_shape_3_v1 *value;
} thinkthen_complete_answer_shape_3_presence_v1;

typedef union thinkthen_complete_answer_shape_8_any_of_1_items_data_v1 {
  double number;
  uint32_t null;
} thinkthen_complete_answer_shape_8_any_of_1_items_data_v1;

typedef struct thinkthen_complete_answer_shape_8_any_of_1_items_v1 {
  uint32_t kind;
  union thinkthen_complete_answer_shape_8_any_of_1_items_data_v1 data;
} thinkthen_complete_answer_shape_8_any_of_1_items_v1;

typedef struct thinkthen_complete_answer_shape_8_any_of_1_items_list_v1 {
  const struct thinkthen_complete_answer_shape_8_any_of_1_items_v1 *const *data;
  size_t len;
} thinkthen_complete_answer_shape_8_any_of_1_items_list_v1;

typedef union thinkthen_complete_answer_shape_2_data_v1 {
  double number;
  uint32_t null;
  struct thinkthen_complete_answer_shape_8_any_of_1_items_list_v1 array;
} thinkthen_complete_answer_shape_2_data_v1;

typedef struct thinkthen_complete_answer_shape_2_v1 {
  uint32_t kind;
  union thinkthen_complete_answer_shape_2_data_v1 data;
} thinkthen_complete_answer_shape_2_v1;

typedef struct thinkthen_complete_answer_shape_2_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_answer_shape_2_v1 *value;
} thinkthen_complete_answer_shape_2_presence_v1;

typedef struct thinkthen_complete_answer_resolution_v1 {
  uint32_t kind;
} thinkthen_complete_answer_resolution_v1;

typedef struct thinkthen_complete_answer_resolution_map_entry_v1 {
  struct thinkthen_complete_utf8_v1 name;
  const struct thinkthen_complete_answer_resolution_v1 *value;
} thinkthen_complete_answer_resolution_map_entry_v1;

typedef struct thinkthen_complete_answer_resolution_map_v1 {
  const struct thinkthen_complete_answer_resolution_map_entry_v1 *data;
  size_t len;
} thinkthen_complete_answer_resolution_map_v1;

typedef union thinkthen_complete_answer_resolutions_data_v1 {
  const struct thinkthen_complete_answer_resolution_v1 *string;
  struct thinkthen_complete_answer_resolution_map_v1 object;
} thinkthen_complete_answer_resolutions_data_v1;

typedef struct thinkthen_complete_answer_resolutions_v1 {
  uint32_t kind;
  union thinkthen_complete_answer_resolutions_data_v1 data;
} thinkthen_complete_answer_resolutions_v1;

typedef struct thinkthen_complete_answer_resolutions_list_v1 {
  const struct thinkthen_complete_answer_resolutions_v1 *const *data;
  size_t len;
} thinkthen_complete_answer_resolutions_list_v1;

typedef union thinkthen_complete_answer_shape_5_data_v1 {
  const struct thinkthen_complete_answer_resolution_v1 *string;
  struct thinkthen_complete_answer_resolution_map_v1 object;
  struct thinkthen_complete_answer_resolutions_list_v1 array;
} thinkthen_complete_answer_shape_5_data_v1;

typedef struct thinkthen_complete_answer_shape_5_v1 {
  uint32_t kind;
  union thinkthen_complete_answer_shape_5_data_v1 data;
} thinkthen_complete_answer_shape_5_v1;

typedef struct thinkthen_complete_answer_shape_v1 {
  const struct thinkthen_complete_json_v1 *value;
} thinkthen_complete_answer_shape_v1;

typedef struct thinkthen_complete_answer_shape_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_answer_shape_v1 *value;
} thinkthen_complete_answer_shape_presence_v1;

typedef struct thinkthen_complete_settled_answer_decide_v1 {
  struct thinkthen_complete_utf8_v1 function;
  struct thinkthen_complete_answer_shape_4_presence_v1 index;
  struct thinkthen_complete_answer_shape_3_presence_v1 pick;
  struct thinkthen_complete_answer_shape_2_presence_v1 probability;
  const struct thinkthen_complete_answer_shape_5_v1 *resolution;
  struct thinkthen_complete_answer_shape_presence_v1 value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_settled_answer_decide_v1;

typedef union thinkthen_complete_answer_shape_6_data_v1 {
  struct thinkthen_complete_utf8_v1 string;
  uint32_t null;
  struct thinkthen_complete_answer_shape_6_any_of_1_items_list_v1 array;
} thinkthen_complete_answer_shape_6_data_v1;

typedef struct thinkthen_complete_answer_shape_6_v1 {
  uint32_t kind;
  union thinkthen_complete_answer_shape_6_data_v1 data;
} thinkthen_complete_answer_shape_6_v1;

typedef struct thinkthen_complete_answer_shape_6_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_answer_shape_6_v1 *value;
} thinkthen_complete_answer_shape_6_presence_v1;

typedef struct thinkthen_complete_settled_answer_choose_v1 {
  struct thinkthen_complete_utf8_v1 function;
  struct thinkthen_complete_answer_shape_4_presence_v1 index;
  struct thinkthen_complete_answer_shape_3_presence_v1 pick;
  struct thinkthen_complete_answer_shape_2_presence_v1 probability;
  const struct thinkthen_complete_answer_shape_5_v1 *resolution;
  struct thinkthen_complete_answer_shape_6_presence_v1 value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_settled_answer_choose_v1;

typedef union thinkthen_complete_answer_shape_7_any_of_1_items_data_v1 {
  struct thinkthen_complete_string_list_v1 array;
  uint32_t null;
} thinkthen_complete_answer_shape_7_any_of_1_items_data_v1;

typedef struct thinkthen_complete_answer_shape_7_any_of_1_items_v1 {
  uint32_t kind;
  union thinkthen_complete_answer_shape_7_any_of_1_items_data_v1 data;
} thinkthen_complete_answer_shape_7_any_of_1_items_v1;

typedef struct thinkthen_complete_answer_shape_7_any_of_1_items_list_v1 {
  const struct thinkthen_complete_answer_shape_7_any_of_1_items_v1 *const *data;
  size_t len;
} thinkthen_complete_answer_shape_7_any_of_1_items_list_v1;

typedef union thinkthen_complete_answer_shape_7_data_v1 {
  struct thinkthen_complete_string_list_v1 array;
  uint32_t null;
  struct thinkthen_complete_answer_shape_7_any_of_1_items_list_v1 array2;
} thinkthen_complete_answer_shape_7_data_v1;

typedef struct thinkthen_complete_answer_shape_7_v1 {
  uint32_t kind;
  union thinkthen_complete_answer_shape_7_data_v1 data;
} thinkthen_complete_answer_shape_7_v1;

typedef struct thinkthen_complete_answer_shape_7_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_answer_shape_7_v1 *value;
} thinkthen_complete_answer_shape_7_presence_v1;

typedef struct thinkthen_complete_settled_answer_tag_v1 {
  struct thinkthen_complete_utf8_v1 function;
  struct thinkthen_complete_answer_shape_4_presence_v1 index;
  struct thinkthen_complete_answer_shape_3_presence_v1 pick;
  struct thinkthen_complete_answer_shape_2_presence_v1 probability;
  const struct thinkthen_complete_answer_shape_5_v1 *resolution;
  struct thinkthen_complete_answer_shape_7_presence_v1 value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_settled_answer_tag_v1;

typedef union thinkthen_complete_answer_shape_8_data_v1 {
  double number;
  uint32_t null;
  struct thinkthen_complete_answer_shape_8_any_of_1_items_list_v1 array;
} thinkthen_complete_answer_shape_8_data_v1;

typedef struct thinkthen_complete_answer_shape_8_v1 {
  uint32_t kind;
  union thinkthen_complete_answer_shape_8_data_v1 data;
} thinkthen_complete_answer_shape_8_v1;

typedef struct thinkthen_complete_answer_shape_8_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_answer_shape_8_v1 *value;
} thinkthen_complete_answer_shape_8_presence_v1;

typedef struct thinkthen_complete_settled_answer_score_v1 {
  struct thinkthen_complete_utf8_v1 function;
  struct thinkthen_complete_answer_shape_4_presence_v1 index;
  struct thinkthen_complete_answer_shape_3_presence_v1 pick;
  struct thinkthen_complete_answer_shape_2_presence_v1 probability;
  const struct thinkthen_complete_answer_shape_5_v1 *resolution;
  struct thinkthen_complete_answer_shape_8_presence_v1 value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_settled_answer_score_v1;

typedef union thinkthen_complete_answer_shape_10_any_of_1_items_data_v1 {
  struct thinkthen_complete_answer_shape_8_any_of_1_items_list_v1 array;
  uint32_t null;
} thinkthen_complete_answer_shape_10_any_of_1_items_data_v1;

typedef struct thinkthen_complete_answer_shape_10_any_of_1_items_v1 {
  uint32_t kind;
  union thinkthen_complete_answer_shape_10_any_of_1_items_data_v1 data;
} thinkthen_complete_answer_shape_10_any_of_1_items_v1;

typedef struct thinkthen_complete_answer_shape_10_any_of_1_items_list_v1 {
  const struct thinkthen_complete_answer_shape_10_any_of_1_items_v1 *const *data;
  size_t len;
} thinkthen_complete_answer_shape_10_any_of_1_items_list_v1;

typedef union thinkthen_complete_answer_shape_10_data_v1 {
  struct thinkthen_complete_answer_shape_8_any_of_1_items_list_v1 array;
  uint32_t null;
  struct thinkthen_complete_answer_shape_10_any_of_1_items_list_v1 array2;
} thinkthen_complete_answer_shape_10_data_v1;

typedef struct thinkthen_complete_answer_shape_10_v1 {
  uint32_t kind;
  union thinkthen_complete_answer_shape_10_data_v1 data;
} thinkthen_complete_answer_shape_10_v1;

typedef struct thinkthen_complete_answer_shape_10_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_answer_shape_10_v1 *value;
} thinkthen_complete_answer_shape_10_presence_v1;

typedef struct thinkthen_complete_settled_original_v1 {
  const struct thinkthen_complete_json_v1 *value;
} thinkthen_complete_settled_original_v1;

typedef struct thinkthen_complete_settled_original_list_v1 {
  const struct thinkthen_complete_settled_original_v1 *const *data;
  size_t len;
} thinkthen_complete_settled_original_list_v1;

typedef union thinkthen_complete_answer_shape_9_any_of_1_items_data_v1 {
  struct thinkthen_complete_settled_original_list_v1 array;
  uint32_t null;
} thinkthen_complete_answer_shape_9_any_of_1_items_data_v1;

typedef struct thinkthen_complete_answer_shape_9_any_of_1_items_v1 {
  uint32_t kind;
  union thinkthen_complete_answer_shape_9_any_of_1_items_data_v1 data;
} thinkthen_complete_answer_shape_9_any_of_1_items_v1;

typedef struct thinkthen_complete_answer_shape_9_any_of_1_items_list_v1 {
  const struct thinkthen_complete_answer_shape_9_any_of_1_items_v1 *const *data;
  size_t len;
} thinkthen_complete_answer_shape_9_any_of_1_items_list_v1;

typedef union thinkthen_complete_answer_shape_9_data_v1 {
  struct thinkthen_complete_settled_original_list_v1 array;
  uint32_t null;
  struct thinkthen_complete_answer_shape_9_any_of_1_items_list_v1 array2;
} thinkthen_complete_answer_shape_9_data_v1;

typedef struct thinkthen_complete_answer_shape_9_v1 {
  uint32_t kind;
  union thinkthen_complete_answer_shape_9_data_v1 data;
} thinkthen_complete_answer_shape_9_v1;

typedef struct thinkthen_complete_answer_shape_9_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_answer_shape_9_v1 *value;
} thinkthen_complete_answer_shape_9_presence_v1;

typedef struct thinkthen_complete_settled_answer_filter_v1 {
  struct thinkthen_complete_utf8_v1 function;
  struct thinkthen_complete_answer_shape_4_presence_v1 index;
  struct thinkthen_complete_answer_shape_3_presence_v1 pick;
  struct thinkthen_complete_answer_shape_10_presence_v1 probability;
  const struct thinkthen_complete_answer_shape_5_v1 *resolution;
  struct thinkthen_complete_answer_shape_9_presence_v1 value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_settled_answer_filter_v1;

typedef struct thinkthen_complete_integer_list_v1 {
  const uint64_t *data;
  size_t len;
} thinkthen_complete_integer_list_v1;

typedef union thinkthen_complete_answer_shape_11_any_of_1_items_data_v1 {
  struct thinkthen_complete_integer_list_v1 array;
  uint32_t null;
} thinkthen_complete_answer_shape_11_any_of_1_items_data_v1;

typedef struct thinkthen_complete_answer_shape_11_any_of_1_items_v1 {
  uint32_t kind;
  union thinkthen_complete_answer_shape_11_any_of_1_items_data_v1 data;
} thinkthen_complete_answer_shape_11_any_of_1_items_v1;

typedef struct thinkthen_complete_answer_shape_11_any_of_1_items_list_v1 {
  const struct thinkthen_complete_answer_shape_11_any_of_1_items_v1 *const *data;
  size_t len;
} thinkthen_complete_answer_shape_11_any_of_1_items_list_v1;

typedef union thinkthen_complete_answer_shape_11_data_v1 {
  struct thinkthen_complete_integer_list_v1 array;
  uint32_t null;
  struct thinkthen_complete_answer_shape_11_any_of_1_items_list_v1 array2;
} thinkthen_complete_answer_shape_11_data_v1;

typedef struct thinkthen_complete_answer_shape_11_v1 {
  uint32_t kind;
  union thinkthen_complete_answer_shape_11_data_v1 data;
} thinkthen_complete_answer_shape_11_v1;

typedef struct thinkthen_complete_answer_shape_11_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_answer_shape_11_v1 *value;
} thinkthen_complete_answer_shape_11_presence_v1;

typedef struct thinkthen_complete_settled_answer_rank_v1 {
  struct thinkthen_complete_utf8_v1 function;
  struct thinkthen_complete_answer_shape_4_presence_v1 index;
  struct thinkthen_complete_answer_shape_3_presence_v1 pick;
  struct thinkthen_complete_answer_shape_10_presence_v1 probability;
  const struct thinkthen_complete_answer_shape_5_v1 *resolution;
  struct thinkthen_complete_answer_shape_11_presence_v1 value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_settled_answer_rank_v1;

typedef struct thinkthen_complete_answer_shape_12_v1 {
  const struct thinkthen_complete_json_v1 *value;
} thinkthen_complete_answer_shape_12_v1;

typedef struct thinkthen_complete_answer_shape_12_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_answer_shape_12_v1 *value;
} thinkthen_complete_answer_shape_12_presence_v1;

typedef struct thinkthen_complete_settled_answer_find_v1 {
  struct thinkthen_complete_utf8_v1 function;
  struct thinkthen_complete_answer_shape_4_presence_v1 index;
  struct thinkthen_complete_answer_shape_3_presence_v1 pick;
  struct thinkthen_complete_answer_shape_2_presence_v1 probability;
  const struct thinkthen_complete_answer_shape_5_v1 *resolution;
  struct thinkthen_complete_answer_shape_12_presence_v1 value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_settled_answer_find_v1;

typedef struct thinkthen_complete_settled_recognition_entities_v1 {
  struct thinkthen_complete_entity_list_v1 entities;
  struct thinkthen_complete_entity_edge_list_presence_v1 relations;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_settled_recognition_entities_v1;

typedef struct thinkthen_complete_settled_recognition_mode_proposals_v1 {
  const struct thinkthen_complete_boundary_mode_v1 *mode;
  struct thinkthen_complete_boundary_proposal_list_v1 proposals;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_settled_recognition_mode_proposals_v1;

typedef union thinkthen_complete_settled_recognition_data_v1 {
  const struct thinkthen_complete_settled_recognition_entities_v1 *entities;
  const struct thinkthen_complete_settled_recognition_mode_proposals_v1 *mode_proposals;
} thinkthen_complete_settled_recognition_data_v1;

typedef struct thinkthen_complete_settled_recognition_v1 {
  uint32_t kind;
  union thinkthen_complete_settled_recognition_data_v1 data;
} thinkthen_complete_settled_recognition_v1;

typedef union thinkthen_complete_answer_shape_13_any_of_1_items_data_v1 {
  const struct thinkthen_complete_settled_recognition_v1 *object;
  uint32_t null;
} thinkthen_complete_answer_shape_13_any_of_1_items_data_v1;

typedef struct thinkthen_complete_answer_shape_13_any_of_1_items_v1 {
  uint32_t kind;
  union thinkthen_complete_answer_shape_13_any_of_1_items_data_v1 data;
} thinkthen_complete_answer_shape_13_any_of_1_items_v1;

typedef struct thinkthen_complete_answer_shape_13_any_of_1_items_list_v1 {
  const struct thinkthen_complete_answer_shape_13_any_of_1_items_v1 *const *data;
  size_t len;
} thinkthen_complete_answer_shape_13_any_of_1_items_list_v1;

typedef union thinkthen_complete_answer_shape_13_data_v1 {
  const struct thinkthen_complete_settled_recognition_v1 *object;
  uint32_t null;
  struct thinkthen_complete_answer_shape_13_any_of_1_items_list_v1 array;
} thinkthen_complete_answer_shape_13_data_v1;

typedef struct thinkthen_complete_answer_shape_13_v1 {
  uint32_t kind;
  union thinkthen_complete_answer_shape_13_data_v1 data;
} thinkthen_complete_answer_shape_13_v1;

typedef struct thinkthen_complete_answer_shape_13_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_answer_shape_13_v1 *value;
} thinkthen_complete_answer_shape_13_presence_v1;

typedef struct thinkthen_complete_settled_answer_recognize_v1 {
  struct thinkthen_complete_utf8_v1 function;
  struct thinkthen_complete_answer_shape_4_presence_v1 index;
  struct thinkthen_complete_answer_shape_3_presence_v1 pick;
  struct thinkthen_complete_answer_shape_10_presence_v1 probability;
  const struct thinkthen_complete_answer_shape_5_v1 *resolution;
  struct thinkthen_complete_answer_shape_13_presence_v1 value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_settled_answer_recognize_v1;

typedef struct thinkthen_complete_settled_relations_v1 {
  struct thinkthen_complete_related_entity_edge_list_v1 value;
} thinkthen_complete_settled_relations_v1;

typedef union thinkthen_complete_answer_shape_14_any_of_1_items_data_v1 {
  const struct thinkthen_complete_settled_relations_v1 *array;
  uint32_t null;
} thinkthen_complete_answer_shape_14_any_of_1_items_data_v1;

typedef struct thinkthen_complete_answer_shape_14_any_of_1_items_v1 {
  uint32_t kind;
  union thinkthen_complete_answer_shape_14_any_of_1_items_data_v1 data;
} thinkthen_complete_answer_shape_14_any_of_1_items_v1;

typedef struct thinkthen_complete_answer_shape_14_any_of_1_items_list_v1 {
  const struct thinkthen_complete_answer_shape_14_any_of_1_items_v1 *const *data;
  size_t len;
} thinkthen_complete_answer_shape_14_any_of_1_items_list_v1;

typedef union thinkthen_complete_answer_shape_14_data_v1 {
  const struct thinkthen_complete_settled_relations_v1 *array;
  uint32_t null;
  struct thinkthen_complete_answer_shape_14_any_of_1_items_list_v1 array2;
} thinkthen_complete_answer_shape_14_data_v1;

typedef struct thinkthen_complete_answer_shape_14_v1 {
  uint32_t kind;
  union thinkthen_complete_answer_shape_14_data_v1 data;
} thinkthen_complete_answer_shape_14_v1;

typedef struct thinkthen_complete_answer_shape_14_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_answer_shape_14_v1 *value;
} thinkthen_complete_answer_shape_14_presence_v1;

typedef struct thinkthen_complete_settled_answer_relate_v1 {
  struct thinkthen_complete_utf8_v1 function;
  struct thinkthen_complete_answer_shape_4_presence_v1 index;
  struct thinkthen_complete_answer_shape_3_presence_v1 pick;
  struct thinkthen_complete_answer_shape_10_presence_v1 probability;
  const struct thinkthen_complete_answer_shape_5_v1 *resolution;
  struct thinkthen_complete_answer_shape_14_presence_v1 value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_settled_answer_relate_v1;

typedef union thinkthen_complete_settled_judgment_data_v1 {
  uint32_t boolean;
  uint32_t null;
  struct thinkthen_complete_utf8_v1 string;
  double number;
  struct thinkthen_complete_string_list_v1 array;
} thinkthen_complete_settled_judgment_data_v1;

typedef struct thinkthen_complete_settled_judgment_v1 {
  uint32_t kind;
  union thinkthen_complete_settled_judgment_data_v1 data;
} thinkthen_complete_settled_judgment_v1;

typedef struct thinkthen_complete_settled_judgment_map_entry_v1 {
  struct thinkthen_complete_utf8_v1 name;
  const struct thinkthen_complete_settled_judgment_v1 *value;
} thinkthen_complete_settled_judgment_map_entry_v1;

typedef struct thinkthen_complete_settled_judgment_map_v1 {
  const struct thinkthen_complete_settled_judgment_map_entry_v1 *data;
  size_t len;
} thinkthen_complete_settled_judgment_map_v1;

typedef union thinkthen_complete_answer_shape_15_any_of_1_items_data_v1 {
  struct thinkthen_complete_settled_judgment_map_v1 object;
  uint32_t null;
} thinkthen_complete_answer_shape_15_any_of_1_items_data_v1;

typedef struct thinkthen_complete_answer_shape_15_any_of_1_items_v1 {
  uint32_t kind;
  union thinkthen_complete_answer_shape_15_any_of_1_items_data_v1 data;
} thinkthen_complete_answer_shape_15_any_of_1_items_v1;

typedef struct thinkthen_complete_answer_shape_15_any_of_1_items_list_v1 {
  const struct thinkthen_complete_answer_shape_15_any_of_1_items_v1 *const *data;
  size_t len;
} thinkthen_complete_answer_shape_15_any_of_1_items_list_v1;

typedef union thinkthen_complete_answer_shape_15_data_v1 {
  struct thinkthen_complete_settled_judgment_map_v1 object;
  uint32_t null;
  struct thinkthen_complete_answer_shape_15_any_of_1_items_list_v1 array;
} thinkthen_complete_answer_shape_15_data_v1;

typedef struct thinkthen_complete_answer_shape_15_v1 {
  uint32_t kind;
  union thinkthen_complete_answer_shape_15_data_v1 data;
} thinkthen_complete_answer_shape_15_v1;

typedef struct thinkthen_complete_answer_shape_15_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_answer_shape_15_v1 *value;
} thinkthen_complete_answer_shape_15_presence_v1;

typedef struct thinkthen_complete_settled_answer_annotate_v1 {
  struct thinkthen_complete_utf8_v1 function;
  struct thinkthen_complete_answer_shape_4_presence_v1 index;
  struct thinkthen_complete_answer_shape_3_presence_v1 pick;
  struct thinkthen_complete_answer_shape_2_presence_v1 probability;
  const struct thinkthen_complete_answer_shape_5_v1 *resolution;
  struct thinkthen_complete_answer_shape_15_presence_v1 value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_settled_answer_annotate_v1;

typedef union thinkthen_complete_settled_answer_data_v1 {
  const struct thinkthen_complete_settled_answer_decide_v1 *decide;
  const struct thinkthen_complete_settled_answer_choose_v1 *choose;
  const struct thinkthen_complete_settled_answer_tag_v1 *tag;
  const struct thinkthen_complete_settled_answer_score_v1 *score;
  const struct thinkthen_complete_settled_answer_filter_v1 *filter;
  const struct thinkthen_complete_settled_answer_rank_v1 *rank;
  const struct thinkthen_complete_settled_answer_find_v1 *find;
  const struct thinkthen_complete_settled_answer_recognize_v1 *recognize;
  const struct thinkthen_complete_settled_answer_relate_v1 *relate;
  const struct thinkthen_complete_settled_answer_annotate_v1 *annotate;
} thinkthen_complete_settled_answer_data_v1;

typedef struct thinkthen_complete_settled_answer_v1 {
  uint32_t kind;
  union thinkthen_complete_settled_answer_data_v1 data;
} thinkthen_complete_settled_answer_v1;

typedef struct thinkthen_complete_settled_answer_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_settled_answer_v1 *value;
} thinkthen_complete_settled_answer_presence_v1;

typedef struct thinkthen_complete_facts_v1 {
  struct thinkthen_complete_attempt_list_presence_v1 attempts;
  uint64_t cache_answers;
  const struct thinkthen_complete_call_id_v1 *call_id;
  struct thinkthen_complete_utf8_presence_v1 estimated_cost_usd;
  struct thinkthen_complete_u32_presence_v1 held_model_mismatch;
  struct thinkthen_complete_u64_presence_v1 input_tokens;
  uint64_t largest_request_bytes;
  struct thinkthen_complete_u64_presence_v1 largest_request_estimated_input_tokens;
  struct thinkthen_complete_utf8_presence_v1 model;
  struct thinkthen_complete_u64_presence_v1 output_tokens;
  uint64_t records;
  uint64_t requests_sent;
  double seconds;
  struct thinkthen_complete_utf8_v1 token_estimate_method;
  struct thinkthen_complete_persistence_observation_presence_v1 usage_persistence;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_facts_v1;

typedef struct thinkthen_complete_facts_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_facts_v1 *value;
} thinkthen_complete_facts_presence_v1;

typedef struct thinkthen_complete_input_denial_initial_request_v1 {
  struct thinkthen_complete_utf8_v1 kind;
  uint64_t limit;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_input_denial_initial_request_v1;

typedef struct thinkthen_complete_input_denial_additional_request_v1 {
  struct thinkthen_complete_utf8_v1 kind;
  uint64_t limit;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_input_denial_additional_request_v1;

typedef struct thinkthen_complete_input_denial_retry_v1 {
  struct thinkthen_complete_utf8_v1 kind;
  uint64_t last_status;
  uint64_t limit;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_input_denial_retry_v1;

typedef union thinkthen_complete_input_denial_data_v1 {
  const struct thinkthen_complete_input_denial_initial_request_v1 *initial_request;
  const struct thinkthen_complete_input_denial_additional_request_v1 *additional_request;
  const struct thinkthen_complete_input_denial_retry_v1 *retry;
} thinkthen_complete_input_denial_data_v1;

typedef struct thinkthen_complete_input_denial_v1 {
  uint32_t kind;
  union thinkthen_complete_input_denial_data_v1 data;
} thinkthen_complete_input_denial_v1;

typedef struct thinkthen_complete_input_denial_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_input_denial_v1 *value;
} thinkthen_complete_input_denial_presence_v1;

typedef struct thinkthen_complete_failure_kind_v1 {
  uint32_t kind;
} thinkthen_complete_failure_kind_v1;

typedef struct thinkthen_complete_send_denial_before_first_send_v1 {
  struct thinkthen_complete_utf8_v1 kind;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_send_denial_before_first_send_v1;

typedef struct thinkthen_complete_send_denial_before_additional_send_v1 {
  struct thinkthen_complete_utf8_v1 kind;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_send_denial_before_additional_send_v1;

typedef struct thinkthen_complete_send_denial_before_retry_v1 {
  struct thinkthen_complete_utf8_v1 kind;
  uint64_t last_status;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_send_denial_before_retry_v1;

typedef union thinkthen_complete_send_denial_data_v1 {
  const struct thinkthen_complete_send_denial_before_first_send_v1 *before_first_send;
  const struct thinkthen_complete_send_denial_before_additional_send_v1 *before_additional_send;
  const struct thinkthen_complete_send_denial_before_retry_v1 *before_retry;
} thinkthen_complete_send_denial_data_v1;

typedef struct thinkthen_complete_send_denial_v1 {
  uint32_t kind;
  union thinkthen_complete_send_denial_data_v1 data;
} thinkthen_complete_send_denial_v1;

typedef struct thinkthen_complete_send_denial_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_send_denial_v1 *value;
} thinkthen_complete_send_denial_presence_v1;

typedef struct thinkthen_complete_stop_cause_v1 {
  uint32_t kind;
} thinkthen_complete_stop_cause_v1;

typedef struct thinkthen_complete_stopped_v1 {
  struct thinkthen_complete_u64_presence_v1 at;
  const struct thinkthen_complete_stop_cause_v1 *cause;
  uint32_t retryable;
  struct thinkthen_complete_u64_presence_v1 status;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_stopped_v1;

typedef struct thinkthen_complete_error_v1 {
  struct thinkthen_complete_input_denial_presence_v1 estimated_input_denial;
  const struct thinkthen_complete_failure_kind_v1 *kind;
  struct thinkthen_complete_utf8_v1 message;
  uint32_t retryable;
  struct thinkthen_complete_send_denial_presence_v1 send_budget_denial;
  const struct thinkthen_complete_stopped_v1 *stopped;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_error_v1;

typedef struct thinkthen_complete_call_error_v1 {
  const struct thinkthen_complete_error_v1 *error;
  struct thinkthen_complete_facts_presence_v1 facts;
  const struct thinkthen_complete_answer_resolution_v1 *resolution;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_call_error_v1;

typedef struct thinkthen_complete_call_error_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_call_error_v1 *value;
} thinkthen_complete_call_error_presence_v1;

typedef struct thinkthen_complete_session_packet_terminal_v1 {
  struct thinkthen_complete_settled_answer_presence_v1 answer;
  struct thinkthen_complete_facts_presence_v1 facts;
  struct thinkthen_complete_call_error_presence_v1 failure;
  struct thinkthen_complete_utf8_v1 kind;
  const struct thinkthen_complete_answer_resolution_v1 *resolution;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_session_packet_terminal_v1;

typedef union thinkthen_complete_session_packet_data_v1 {
  const struct thinkthen_complete_session_packet_decide_row_v1 *decide_row;
  const struct thinkthen_complete_session_packet_choose_row_v1 *choose_row;
  const struct thinkthen_complete_session_packet_tag_row_v1 *tag_row;
  const struct thinkthen_complete_session_packet_score_row_v1 *score_row;
  const struct thinkthen_complete_session_packet_filter_row_v1 *filter_row;
  const struct thinkthen_complete_session_packet_annotate_row_v1 *annotate_row;
  const struct thinkthen_complete_session_packet_decide_aggregate_v1 *decide_aggregate;
  const struct thinkthen_complete_session_packet_choose_aggregate_v1 *choose_aggregate;
  const struct thinkthen_complete_session_packet_tag_aggregate_v1 *tag_aggregate;
  const struct thinkthen_complete_session_packet_score_aggregate_v1 *score_aggregate;
  const struct thinkthen_complete_session_packet_filter_aggregate_v1 *filter_aggregate;
  const struct thinkthen_complete_session_packet_rank_aggregate_v1 *rank_aggregate;
  const struct thinkthen_complete_session_packet_find_aggregate_v1 *find_aggregate;
  const struct thinkthen_complete_session_packet_annotate_aggregate_v1 *annotate_aggregate;
  const struct thinkthen_complete_session_packet_recognize_aggregate_v1 *recognize_aggregate;
  const struct thinkthen_complete_session_packet_relate_aggregate_v1 *relate_aggregate;
  const struct thinkthen_complete_session_packet_observation_v1 *observation;
  const struct thinkthen_complete_session_packet_terminal_v1 *terminal;
} thinkthen_complete_session_packet_data_v1;

typedef struct thinkthen_complete_session_packet_v1 {
  uint32_t kind;
  union thinkthen_complete_session_packet_data_v1 data;
} thinkthen_complete_session_packet_v1;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

/*
 Cancel from any thread. NULL is ignored.
 # Safety
 token is NULL or live until every concurrent call/cancel returns.
 */
void thinkthen_call_cancel_fire(const struct thinkthen_call_cancel *token);

/*
 Free this independent owner; NULL is ignored.
 # Safety
 owner is NULL or unfreed, after all concurrent operations and borrowed reads finish.
 */
void thinkthen_call_cancel_free(struct thinkthen_call_cancel *owner);

/*
 Allocate an independent cancellation token; NULL only if the boundary panics.
 */
struct thinkthen_call_cancel *thinkthen_call_cancel_new(void);

/*
 Read the error kind as the existing status code. NULL returns EUSAGE.
 # Safety
 owner is NULL or live throughout this read.
 */
int32_t thinkthen_call_error_code(const struct thinkthen_call_error *owner);

/*
 Borrow existing canonical Facts JSON until error_free; absent facts return empty text.
 # Safety
 owner is NULL or live for every reader of the returned text.
 */
struct thinkthen_complete_utf8_v1 thinkthen_call_error_facts_json(const struct thinkthen_call_error *owner);

/*
 Free this independent owner; NULL is ignored.
 # Safety
 owner is NULL or unfreed, after all concurrent operations and borrowed reads finish.
 */
void thinkthen_call_error_free(struct thinkthen_call_error *owner);

/*
 Borrow safe counted error text until error_free. NULL returns empty text.
 # Safety
 owner is NULL or live for every reader of the returned text.
 */
struct thinkthen_complete_utf8_v1 thinkthen_call_error_message(const struct thinkthen_call_error *owner);

/*
 Read retryability from any thread; NULL returns false.
 # Safety
 owner is NULL or live throughout this read.
 */
int32_t thinkthen_call_error_retryable(const struct thinkthen_call_error *owner);

/*
 Free this independent owner; NULL is ignored.
 # Safety
 owner is NULL or unfreed, after all concurrent operations and borrowed reads finish.
 */
void thinkthen_call_result_free(struct thinkthen_call_result *owner);

/*
 Borrow complete canonical Call JSON until result_free. NULL returns empty text.
 # Safety
 owner is NULL or live for every reader of the returned text.
 */
struct thinkthen_complete_utf8_v1 thinkthen_call_result_json(const struct thinkthen_call_result *owner);

/*
 Finish this engine's current usage deltas and observe their persistence.
 Only usage-lock acquisition has a deadline; other filesystem work can take
 longer. Written covers current deltas only, not future calls or other engines.
 Failed carries static native safe advice readable after engine destruction
 while the library remains loaded. Every nonzero return preserves outputs.
 # Safety
 The engine and output obligations of usage_persistence_v1 apply.
 */
int thinkthen_engine_finish_usage_status_v1(const struct thinkthen_engine *engine,
                                            struct thinkthen_complete_usage_persistence_v1 *out_state,
                                            struct thinkthen_complete_utf8_v1 *out_advice);

/*
 Free an engine; null is ignored.

 # Safety

 `engine` is null or a live engine no other call is using.
 Free an engine. NULL is accepted and ignored. Free it only after every
 call on it has returned.
 */
void thinkthen_engine_free(struct thinkthen_engine *engine);

/*
 Build an engine from the environment; null when it cannot be built, and
 the error functions then answer with a null engine for the failure.
 Build from the command's environment, including address/key/cache and
 XDG configuration/cache defaults. Building sends nothing. Invalid settings
 return NULL/EUSAGE; unreadable cache/configuration returns NULL/ELOCAL.
 The NULL-engine error accessors retain the calling thread's failed build.
 */
struct thinkthen_engine *thinkthen_engine_new(void);

/*
 Build from environment plus canonical typed settings. NULL uses environment alone.
 Copies the finite aligned carrier tree; no foreign pointer survives return.
 Construction sends nothing. Failure returns NULL in the calling thread constructor slot.
 # Safety
 settings is NULL or a live aligned settings carrier tree with counted UTF-8 extents.
 */
struct thinkthen_engine *thinkthen_engine_new_typed(const struct thinkthen_request_engine_settings_v1 *settings);

/*
 Build from environment plus closed UTF-8 JSON settings. NULL uses the environment.
 Construction sends nothing; failures return NULL and occupy the constructor slot.
 # Safety
 settings_json is NULL or live NUL-terminated UTF-8 for the call.
 */
struct thinkthen_engine *thinkthen_engine_new_with(const char *settings_json);

/*
 Observe this engine's live usage persistence without waiting for a writer.
 On success assign both outputs; Failed is an observation, not an operation
 error. Advice is a static counted UTF-8 view, or NULL with zero length.
 Success preserves the calling-thread session diagnostic.
 # Safety
 engine is live for the call. Both outputs are nonnull, aligned, writable,
 nonoverlapping slots. Destruction waits for concurrent callers to return.
 */
int thinkthen_engine_usage_persistence_v1(const struct thinkthen_engine *engine,
                                          struct thinkthen_complete_usage_persistence_v1 *out_state,
                                          struct thinkthen_complete_utf8_v1 *out_advice);

/*
 The calling thread's last code here; with a null engine, its last failed
 build's code, else the usage code.

 # Safety

 `engine` is null or a live engine.
 The code of the calling thread's last failure on this engine: the value
 the failing call returned, THINKTHEN_OK when nothing failed yet. Success
 does not clear it, so read it when a call fails. With a null engine it
 is the calling thread's last failed build's code, else THINKTHEN_EUSAGE,
 because no engine holds a failure.
 */
int thinkthen_error_code(const struct thinkthen_engine *engine);

/*
 Borrow the last failure's call facts on this thread and engine, or null.

 # Safety

 `engine` is null or a live engine.
 Borrow the calling thread's last failed call's facts on this engine, or NULL
 when no failure with started-call facts exists. The pointer has the same
 lifetime as thinkthen_error_message and must not be freed.
 */
const char *thinkthen_error_facts_json(const struct thinkthen_engine *engine);

/*
 The calling thread's last message on this engine; never null.

 # Safety

 `engine` is null or a live engine.
 The message for the last failure the calling thread recorded on this
 engine. Its borrowed pointer stays valid until that thread records its
 next failure on this engine, the engine is freed, or the thread exits.
 Another thread's calls never replace it. Never free the pointer.
 A deadline's message names the limit and its value. Never NULL: before
 any failure it names that nothing failed yet. With a null engine it is
 the calling thread's last failed build's message in a distinct slot, valid
 until that thread's next thinkthen_engine_new or thinkthen_engine_new_with
 call, or its exit. Otherwise it names that no engine came.
 */
const char *thinkthen_error_message(const struct thinkthen_engine *engine);

/*
 1 when the calling thread's last failure here could pass later.

 # Safety

 `engine` is null or a live engine.
 Whether the same call could pass later: 1 for a backend status the
 engine retries, such as busy or failing; 0 for a transport failure,
 which may already have reached the backend, for a refused key, and for
 every kind but the backend kind. Zero when nothing failed. With a null
 engine it follows the calling thread's last failed build, else zero.
 */
int thinkthen_error_retryable(const struct thinkthen_engine *engine);

/*
 Borrow the immutable last engine/thread failure, or constructor failure for NULL.
 NULL means that slot holds no failure. Getters and successful engine calls preserve
 the view, including code, safe message, retryability and typed optional facts.
 Root and every nested value remain stable until this thread next fails on this
 engine, engine_free or thread exit. A NULL-engine view ends at the next constructor.
 Other threads cannot replace or move this thread slot.
 # Safety
 engine is NULL or live for the call and every borrowed engine view.
 */
const struct thinkthen_error_view_v1 *thinkthen_error_view(const struct thinkthen_engine *engine);

/*
 Free a string the interface returned; null is ignored.

 # Safety

 `text` is null or a string the interface returned and has not freed.
 Free the owned JSON returned by thinkthen_request_plan_json. NULL is accepted.
 */
void thinkthen_free_string(char *text);

/*
 Borrow immutable metadata until question_free; publish only on success.
 # Safety
 owner is live and out is aligned writable storage.
 */
int thinkthen_question_author(const struct thinkthen_question *owner,
                              struct thinkthen_question_author_v1 *out);

/*
 Free the question owner after every borrower finishes; NULL is ignored.
 # Safety
 owner is NULL or a live unfreed question no concurrent call uses.
 */
void thinkthen_question_free(struct thinkthen_question *owner);

/*
 Import counted saved-question grammar through the explicitly chosen native role.
 Grammar failures return EUSAGE; no guessing or parser fallback. The immutable
 owner copies every byte and does not judge. Engine-thread failures stay independent
 from immediate session and constructor slots.
 # Safety
 engine is live; json has readable counted UTF-8; out is aligned writable storage.
 */
int thinkthen_question_parse(const struct thinkthen_engine *engine,
                             uint32_t role,
                             struct thinkthen_string_v1 json,
                             struct thinkthen_question **out);

/*
 Borrow the already-admitted saved grammar, preserving omissions and ordered values.
 Root and nested backing remain immutable and stable until question_free, including
 after source bytes or the parsing engine are freed. Author metadata is unchanged.
 NULL owner or output returns EUSAGE; refusal preserves output and diagnostic slots.
 # Safety
 owner is live for every borrower; out is aligned writable pointer storage.
 */
int thinkthen_question_value(const struct thinkthen_question *owner,
                             const struct thinkthen_complete_json_v1 **out);

/*
 Preview canonical Request JSON without reading a key or cache or sending.
 Only fixed atomic questions support previews; dynamic questions and other functions refuse.
 Success owns NUL-terminated JSON; free once with thinkthen_free_string.
 Errors preserve output slots and use the calling thread session diagnostic.
 # Safety
 Engine is live; request_json has request_len readable bytes; NULL requires zero.
 Output slots are nonnull writable and nonoverlapping. Extents fit Rust slices.
 */
int thinkthen_request_plan_json(const struct thinkthen_engine *engine,
                                const char *request_json,
                                size_t request_len,
                                char **out,
                                size_t *out_len);

/*
 Preview named canonical components through existing Rust admission, without
 reading a key/cache, constructing a session or sending. Fixed atomic questions
 match the existing Plan serializer; unsupported functions/questions refuse.
 Success transfers an independent owned canonical value; free with value_free.
 Source bytes and engine may be freed on return. Failure leaves out untouched
 and records only the calling thread immediate session diagnostic.
 # Safety
 engine is live; request is an aligned live record with readable counted UTF-8.
 out is nonnull aligned writable pointer storage.
 */
int thinkthen_request_plan_named_value(const struct thinkthen_engine *engine,
                                       const struct thinkthen_named_request_v1 *request,
                                       struct thinkthen_value **out);

/*
 Preview typed request arguments through Rust admission without sending.
 Supports the same fixed atomic questions and plan JSON as the JSON preview.
 COBOL scalar capacity applies before admission. No carrier pointer survives return.
 Success owns NUL-terminated JSON; free once with thinkthen_free_string.
 Refusals preserve both output slots and use the calling thread session diagnostic.
 # Safety
 Engine is live; function and surface are counted UTF-8 views. The finite aligned
 argument tree remains readable until return. Outputs are writable and nonoverlapping.
 */
int thinkthen_request_plan_typed_json(const struct thinkthen_engine *engine,
                                      struct thinkthen_complete_utf8_v1 function,
                                      const struct thinkthen_request_arguments_v1 *arguments,
                                      struct thinkthen_complete_utf8_v1 surface_name,
                                      char **out,
                                      size_t *out_len);

/*
 Preview typed arguments through existing Rust admission without sessions or sends.
 Exact Plan values match the existing Rust serializer. COBOL whole arbitrary-value
 capacity applies before admission; input pointers never survive return.
 Success transfers an independent immutable canonical value owner; free with value_free.
 Refusal preserves out and records only this thread immediate session diagnostic.
 # Safety
 engine is live; function/surface are readable counted UTF-8. The aligned argument
 tree has valid counted extents, and out is nonnull aligned writable pointer storage.
 */
int thinkthen_request_plan_typed_value(const struct thinkthen_engine *engine,
                                       struct thinkthen_complete_utf8_v1 function,
                                       const struct thinkthen_request_arguments_v1 *arguments,
                                       struct thinkthen_complete_utf8_v1 surface_name,
                                       struct thinkthen_value **out);

/*
 Preview a full canonical Request through existing Rust admission and Plan serialization.
 Sends nothing and reads no key or cache. Unknown fields and unsupported previews refuse.
 Success transfers an immutable independent owner; free once with value_free.
 Source bytes and engine may be freed on return. Failures preserve out and occupy
 only the calling thread immediate session diagnostic, independently of engine/constructor slots.
 # Safety
 engine is live; request_json has request_len readable UTF-8 bytes. NULL requires
 zero, which is invalid Request JSON. Extents fit Rust slices; out is nonnull
 aligned writable pointer storage.
 */
int thinkthen_request_plan_value(const struct thinkthen_engine *engine,
                                 const char *request_json,
                                 size_t request_len,
                                 struct thinkthen_value **out);

/*
 Run one named call to completion using canonical counted question/input/options JSON.
 Producer is optional and supplies the declared feed. Its callback runs only on this
 calling thread. Async hosts dedicate one worker to this operation and cancel through
 the token from another thread. Rust owns all admission, scheduling and stop rules.
 On success publish only result; on failure publish only an independent error.
 The other output remains unchanged. No legacy per-thread error slot is changed.
 Both output slots are required and must be distinct; invalid slots return EUSAGE
 without publishing. Every returned owner survives engine destruction and thread exit.
 # Safety
 engine and optional cancel are live for the whole call. request and its counted
 UTF-8 components remain readable until return. Output slots are aligned writable
 distinct storage. producer/context obey CallProducerV1 and outlive the call.
 */
int32_t thinkthen_run_call(const struct thinkthen_engine *engine,
                           const struct thinkthen_named_request_v1 *request,
                           thinkthen_call_producer_v1 producer,
                           void *context,
                           const struct thinkthen_call_cancel *cancel,
                           struct thinkthen_call_result **result,
                           struct thinkthen_call_error **error);

/*
 Signal cancellation without waiting for a provider or final facts. NULL is ignored.
 Interrupt any settlement reader armed before this call, even after prior cancel.
 # Safety
 session is NULL or live throughout the call.
 */
void thinkthen_session_cancel(struct thinkthen_session *session);

/*
 Borrow the native facts JSON of the last immediate session failure, or NULL.
 The calling thread owns these bytes until its next immediate session failure
 or thread exit. Success and engine construction preserve the same pointer.
 */
const char *thinkthen_session_error_facts_json(void);

/*
 Borrow a safe UTF-8 immediate diagnostic, never NULL. Success preserves
 it; the next immediate session failure or thread exit ends its validity.
 Execution failures arrive as terminal packets and never change this slot.
 */
const char *thinkthen_session_error_message(void);

/*
 Read the retry signal of the calling thread's last immediate session failure.
 Returns zero before a failure. Success and engine construction preserve it.
 */
int thinkthen_session_error_retryable(void);

/*
 Borrow the calling thread immediate session failure, or NULL before failure.
 Success, getters, engine calls and terminal packets preserve this independent slot.
 The root, safe message and every typed optional fact remain valid until the next
 immediate session failure on this thread or thread exit.
 */
const struct thinkthen_error_view_v1 *thinkthen_session_error_view(void);

/*
 Fix input EOF or a canonical closed reader failure. NULL with zero means EOF;
 other inputs decode kind io, utf8, invalid_input or producer and optional location.
 No input bytes survive return. A changed finish refuses without changing intake.
 # Safety
 session is live and failure_json has failure_len readable UTF-8 bytes.
 */
int thinkthen_session_finish(struct thinkthen_session *session,
                             const char *failure_json,
                             size_t failure_len);

/*
 Finish input with a counted typed failure; NULL means EOF.
 Rust preserves the safe diagnostic after the completed prefix and final facts.
 # Safety
 session is live. A nonnull failure satisfies the finite aligned carrier-tree
 obligations of the typed constructor. No foreign pointer survives return.
 */
int thinkthen_session_finish_typed(const struct thinkthen_session *session,
                                   const struct thinkthen_request_reader_failure_v1 *failure);

/*
 Close the output receiver and release this owner without joining native work.
 NULL is ignored. Free once after all concurrent operations return.
 # Safety
 session is NULL or an unfreed owner no operation is using.
 */
void thinkthen_session_free(struct thinkthen_session *session);

/*
 Admit length-delimited UTF-8 request JSON and create an owned native session.
 The engine identifies this invocation as the C surface.
 Inputs may be freed or overwritten after return. The engine may be freed
 after construction; the worker owns its engine. Immediate errors leave out
 unchanged and record only the calling-thread session error slot.
 # Safety
 engine is live, request_json points at request_len readable bytes (NULL
 requires zero), and out is writable. Extents must fit Rust slices.
 */
int thinkthen_session_new(const struct thinkthen_engine *engine,
                          const char *request_json,
                          size_t request_len,
                          struct thinkthen_session **out);

/*
 Construct one named request through Rust and create its owned native session.
 Components use the existing canonical question, input and options grammars.
 Materialized Records become find Units or relate Entities in shared Rust;
 every materialized descriptor is decoded before session creation. Sources
 and caller feeds retain their selectors. Unknown names say "unknown function".
 Ownership, diagnostics and output rules match thinkthen_session_new.
 # Safety
 engine is live; request points to a readable aligned component structure.
 Every view points to its counted readable UTF-8 bytes (NULL requires zero);
 extents fit Rust slices. out is nonnull writable pointer storage. All inputs
 may be freed or overwritten after return; the engine may also be freed.
 */
int thinkthen_session_new_named_with_surface(const struct thinkthen_engine *engine,
                                             const struct thinkthen_named_request_v1 *request,
                                             struct thinkthen_complete_utf8_v1 surface,
                                             struct thinkthen_session **out);

/*
 Construct a named request from counted typed argument records through Rust.
 The carrier tree copies strings, arrays, maps and explicit null alternatives;
 no pointer survives return. Rust alone admits questions, controls and evidence.
 COBOL retains its 8192-byte capacity per scalar, including image/base64 text;
 overflow returns THINKTHEN_COBOL_OVERFLOW before constructing a session.
 All output, error-slot and engine lifetime rules match the named constructor.
 # Safety
 engine is live and out is writable, nonnull. function and surface are counted
 UTF-8 views. arguments and its entire finite carrier tree remain aligned and
 readable throughout the call. Each pointer/length covers its declared element
 type and fits Rust slices. Required pointers are nonnull; optional NULL means
 missing. A tagged null arm means explicit null. Inputs may be freed on return.
 */
int thinkthen_session_new_typed_with_surface(const struct thinkthen_engine *engine,
                                             struct thinkthen_complete_utf8_v1 function,
                                             const struct thinkthen_request_arguments_v1 *arguments,
                                             struct thinkthen_complete_utf8_v1 surface_name,
                                             struct thinkthen_session **out);

/*
 Create an owned session attributed to its outer language wrapper.
 surface names an exact native Surface token, without a trailing NUL.
 Invalid tokens return Usage without retaining or printing their spelling.
 All ownership, diagnostics and output rules match thinkthen_session_new.
 # Safety
 The thinkthen_session_new obligations apply. surface points at surface_len
 readable UTF-8 bytes; NULL requires zero. Extents must fit Rust slices.
 */
int thinkthen_session_new_with_surface(const struct thinkthen_engine *engine,
                                       const char *request_json,
                                       size_t request_len,
                                       const char *surface,
                                       size_t surface_len,
                                       struct thinkthen_session **out);

/*
 Block and transfer one packet without polling or joining provider work.
 interest is WAIT_RESULT or WAIT_SETTLEMENT. RESULT wakes promptly on cancel
 with ECANCELLED; output already queued wins. SETTLEMENT drains later terminal
 facts after cancel. RESULT and END match try_read, including packet ownership.
 Errors preserve both output slots and affect only this thread's session error.
 # Safety
 The try_read slot obligations apply. Keep session live throughout the wait;
 concurrent cancel/push/finish are synchronized. Free after every call returns.
 */
int thinkthen_session_next_packet(struct thinkthen_session *session,
                                  uint32_t interest,
                                  uint32_t *status,
                                  struct thinkthen_session_result **out);

/*
 Wait for input room and transfer one descriptor.
 Park unread output in session order; unread answers consume memory.
 Push never waits for a reader. Closure or cancellation returns
 CLOSED without decoding the payload. Failures preserve status and publish
 only the calling thread diagnostic. No input bytes survive return.
 # Safety
 session stays live throughout the wait. descriptor_json has descriptor_len
 readable bytes (NULL requires zero); status is nonnull writable storage.
 */
int thinkthen_session_push(struct thinkthen_session *session,
                           const char *descriptor_json,
                           size_t descriptor_len,
                           uint32_t *status);

/*
 Wait for input room and transfer a typed descriptor.
 Park unread output in session order; unread answers consume memory.
 Push never waits for a reader. Closed intake retains nothing and never inspects the descriptor tree.
 No input pointer survives return; status is published only on success.
 # Safety
 session is live, status is writable and nonnull, and descriptor satisfies
 the finite aligned carrier-tree obligations of the typed constructor.
 */
int thinkthen_session_push_typed(const struct thinkthen_session *session,
                                 const struct thinkthen_request_session_descriptor_v1 *descriptor,
                                 uint32_t *status);

/*
 Release an independent packet owner. NULL is ignored.
 # Safety
 result is NULL or an unfreed owner no operation or borrowed view is using.
 */
void thinkthen_session_result_free(struct thinkthen_session_result *result);

/*
 Borrow canonical immutable packet JSON, including an owned trailing NUL.
 The length excludes that terminator. Bytes stay valid until result_free,
 even after session_free and engine_free. Concurrent access is synchronized.
 Immediate failure preserves both outputs and records the session diagnostic.
 # Safety
 result is live throughout the call. out and out_len are nonnull, writable,
 distinct addresses; alignment and partial overlaps are caller duties.
 */
int thinkthen_session_result_json(const struct thinkthen_session_result *result,
                                  const char **out,
                                  size_t *out_len);

/*
 Borrow the complete immutable typed packet graph. Nested views retain every
 known field and unknown extension, and survive engine/session destruction.
 Repeated and concurrent access returns the same view. Only result_free ends
 all view lifetimes. Failure leaves out unchanged.
 # Safety
 result stays live during access and while using any borrowed view; out is
 nonnull, aligned writable pointer storage. Free only after all borrowers finish.
 */
int thinkthen_session_result_view(const struct thinkthen_session_result *result,
                                  const struct thinkthen_complete_session_packet_v1 **out);

/*
 Admit one owned descriptor through the shared native decoder without waiting.
 ACCEPTED retains decoded data; FULL and CLOSED retain nothing. Check native
 capacity before decoding or allocating retained content. A concurrent closure
 returns CLOSED without publishing. On OK status always receives a value.
 # Safety
 session is live, descriptor_json has descriptor_len readable UTF-8 bytes
 (NULL requires zero), and status is nonnull and writable.
 */
int thinkthen_session_try_push(struct thinkthen_session *session,
                               const char *descriptor_json,
                               size_t descriptor_len,
                               uint32_t *status);

/*
 Push a counted typed feed descriptor. Rust owns framing and admission.
 No input pointer survives return; status is published only on success.
 # Safety
 session is live, status is writable and nonnull, and descriptor satisfies
 the finite aligned carrier-tree obligations of the typed constructor.
 */
int thinkthen_session_try_push_typed(const struct thinkthen_session *session,
                                     const struct thinkthen_request_session_descriptor_v1 *descriptor,
                                     uint32_t *status);

/*
 Transfer one owned packet without waiting for native work.
 RESULT transfers an owner; PENDING and END write NULL to out. Validate both
 output slots before popping. A result survives session_free and engine_free.
 # Safety
 session is live. status and out are writable, nonnull, distinct addresses.
 Alignment, readable/writable ranges and partial overlaps are caller duties.
 */
int thinkthen_session_try_read(struct thinkthen_session *session,
                               uint32_t *status,
                               struct thinkthen_session_result **out);

/*
 Block for requested readiness without consuming a packet or joining provider
 work. timeout_ms is the maximum condition wait in milliseconds; zero checks
 readiness immediately. RESULT ignores input room; INPUT also watches intake;
 SETTLEMENT ignores cancel to drain final packets after cancellation.
 SETTLEMENT_ARM discards earlier cancel notifications and checks readiness
 immediately; timeout_ms must be zero or Usage preserves outputs and state. One exclusive reader arms before checking
 its cancellation/disposal state, then uses INTERRUPTIBLE_SETTLEMENT to wait
 for output or a subsequent cancel. Other readers must not rearm its wait. Output
 readiness takes priority over cancel. Success preserves thread-local errors;
 failures preserve out_ready and set only the calling thread's diagnostic.
 # Safety
 session is live for the whole wait, including concurrent cancel/read/push.
 Shared operations are synchronized internally. out_ready is writable and
 nonnull. Free only after every concurrent operation, including wait, returns.
 */
int thinkthen_session_wait(const struct thinkthen_session *session,
                           uint32_t interest,
                           uint32_t timeout_ms,
                           uint32_t *out_ready);

/*
 Free an independent canonical value owner; NULL is ignored.
 # Safety
 owner is NULL or unfreed, after every operation and borrowed view finishes.
 */
void thinkthen_value_free(struct thinkthen_value *owner);

/*
 Parse one arbitrary JSON value through the existing Rust parser.
 Numbers retain their original decimal bytes, including large integers and exponents;
 object keys, null and false are preserved without request-shape admission.
 Success transfers an immutable independent owner; free once with value_free.
 Input bytes may be freed on return. Malformed JSON, UTF-8 or extents return EUSAGE,
 preserve out and occupy only the calling thread immediate session diagnostic.
 # Safety
 json has len readable UTF-8 bytes; NULL requires zero, which is invalid JSON.
 Extents fit Rust slices; out is nonnull aligned writable pointer storage.
 */
int thinkthen_value_parse(const char *json, size_t len, struct thinkthen_value **out);

/*
 Borrow the immutable canonical value root and nested backing until value_free.
 Repeated and concurrent getters preserve identical pointers and prior diagnostics.
 NULL owner or output returns EUSAGE without publishing; outputs remain untouched.
 # Safety
 owner remains live for every borrower; out is aligned writable pointer storage.
 */
int thinkthen_value_view(const struct thinkthen_value *owner,
                         const struct thinkthen_complete_json_v1 **out);

/*
 Return the loaded library's NUL-terminated package version and write its interface
 number. The returned immutable text lives until the library is unloaded; never free
 it. This bootstrap signature stays stable even when the interface number changes.
 It allocates nothing, sends nothing, and changes no engine or thread error state.

 Check once when loading the library, before creating an engine or calling any other
 operation. Compare the written number exactly with THINKTHEN_INTERFACE_NUMBER.
 On mismatch, report "ThinkThen C interface mismatch: header requires %u, loaded
 library %s provides %u" with the header number, returned version and loaded number,
 then refuse all other operations. A dynamic loader must also refuse a library
 missing this symbol. Package version differences alone do not imply incompatibility.

 # Safety
 interface_number is nonnull, aligned writable storage for one uint32_t, retained
 throughout this call. The library remains loaded while reading the returned text.
 */
const char *thinkthen_version(uint32_t *interface_number);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

#endif  /* THINKTHEN_H */
