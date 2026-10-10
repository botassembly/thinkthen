#ifndef THINKTHEN_H
#define THINKTHEN_H

/* Generated from Rust by cbindgen 0.29.4. Run sdlc/scripts/generate-c-header.py; do not edit. */

#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>
typedef struct thinkthen_cancel_token thinkthen_cancel_token;

/*
 C value `THINKTHEN_ANSWER_CHOICE_V1`.
 */
#define THINKTHEN_ANSWER_CHOICE_V1 2u

/*
 C value `THINKTHEN_ANSWER_FIND_V1`.
 */
#define THINKTHEN_ANSWER_FIND_V1 5u

/*
 C value `THINKTHEN_ANSWER_SCORE_V1`.
 */
#define THINKTHEN_ANSWER_SCORE_V1 4u

/*
 C value `THINKTHEN_ANSWER_TAG_V1`.
 */
#define THINKTHEN_ANSWER_TAG_V1 3u

/*
 C value `THINKTHEN_ANSWER_YES_NO_V1`.
 */
#define THINKTHEN_ANSWER_YES_NO_V1 1u

/*
 C value `THINKTHEN_ATTEMPT_OK_V1`.
 */
#define THINKTHEN_ATTEMPT_OK_V1 1u

/*
 C value `THINKTHEN_ATTEMPT_STATUS_V1`.
 */
#define THINKTHEN_ATTEMPT_STATUS_V1 2u

/*
 C value `THINKTHEN_ATTEMPT_TRANSPORT_V1`.
 */
#define THINKTHEN_ATTEMPT_TRANSPORT_V1 3u

/*
 C value `THINKTHEN_BATCH_MAX_V1`.
 */
#define THINKTHEN_BATCH_MAX_V1 2u

/*
 C value `THINKTHEN_BATCH_RECORDS_V1`.
 */
#define THINKTHEN_BATCH_RECORDS_V1 1u

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

#define THINKTHEN_COMPLETE_ANSWER_SCORE_V1 4

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

#define THINKTHEN_COMPLETE_ESTIMATED_INPUT_DENIAL_ADDITIONAL_REQUEST_V1 2

#define THINKTHEN_COMPLETE_ESTIMATED_INPUT_DENIAL_INITIAL_REQUEST_V1 1

#define THINKTHEN_COMPLETE_ESTIMATED_INPUT_DENIAL_RETRY_V1 3

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

#define THINKTHEN_COMPLETE_INPUT_DECLARATION_OBJECT_V1 2

#define THINKTHEN_COMPLETE_INPUT_DECLARATION_STRING_V1 1

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

#define THINKTHEN_COMPLETE_READABLE_QUESTION_CHOOSE_V1 2

#define THINKTHEN_COMPLETE_READABLE_QUESTION_DECIDE_V1 1

#define THINKTHEN_COMPLETE_READABLE_QUESTION_SCORE_V1 4

#define THINKTHEN_COMPLETE_READABLE_QUESTION_TAG_V1 3

#define THINKTHEN_COMPLETE_RECOGNITION_MODE_BOUNDARY_ONLY_V1 2

#define THINKTHEN_COMPLETE_RECOGNITION_MODE_WHOLE_V1 1

#define THINKTHEN_COMPLETE_RECOGNITION_ODDS_FIELDS_NAMES_PAIRS_PIECES_PROPOSALS_V1 1

#define THINKTHEN_COMPLETE_RECOGNITION_ODDS_FIELDS_PIECES_PROPOSALS_V1 2

#define THINKTHEN_COMPLETE_RECOGNIZE_FIELDS_ENTITIES_V1 1

#define THINKTHEN_COMPLETE_RECOGNIZE_FIELDS_MODE_PROPOSALS_V1 2

#define THINKTHEN_COMPLETE_RELATED_ENTITY_EDGE_PROPERTIES_SOURCE_FIELDS_FILE_KIND_NAME_ORDINAL_RECORD_V1 2

#define THINKTHEN_COMPLETE_RELATED_ENTITY_EDGE_PROPERTIES_SOURCE_FIELDS_KIND_NAME_V1 1

#define THINKTHEN_COMPLETE_RELATION_DIRECTION_EITHER_V1 2

#define THINKTHEN_COMPLETE_RELATION_DIRECTION_SOURCE_TO_TARGET_V1 1

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

#define THINKTHEN_COMPLETE_SEND_BUDGET_DENIAL_BEFORE_ADDITIONAL_SEND_V1 2

#define THINKTHEN_COMPLETE_SEND_BUDGET_DENIAL_BEFORE_FIRST_SEND_V1 1

#define THINKTHEN_COMPLETE_SEND_BUDGET_DENIAL_BEFORE_RETRY_V1 3

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
 C value `THINKTHEN_CONTENT_JSON_V1`.
 */
#define THINKTHEN_CONTENT_JSON_V1 2u

/*
 C value `THINKTHEN_CONTENT_TEXT_V1`.
 */
#define THINKTHEN_CONTENT_TEXT_V1 1u

/*
 C value `THINKTHEN_DECIDE_AUTHORED_V1`.
 */
#define THINKTHEN_DECIDE_AUTHORED_V1 2u

/*
 C value `THINKTHEN_DECIDE_BOOLEAN_V1`.
 */
#define THINKTHEN_DECIDE_BOOLEAN_V1 1u

/*
 C value `THINKTHEN_DECIDE_NULL_V1`.
 */
#define THINKTHEN_DECIDE_NULL_V1 0u

/*
 C value `THINKTHEN_DIRECTION_EITHER_V1`.
 */
#define THINKTHEN_DIRECTION_EITHER_V1 2u

/*
 C value `THINKTHEN_DIRECTION_SOURCE_TO_TARGET_V1`.
 */
#define THINKTHEN_DIRECTION_SOURCE_TO_TARGET_V1 1u

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
 C value `THINKTHEN_EVENT_QUESTION_V1`.
 */
#define THINKTHEN_EVENT_QUESTION_V1 1u

/*
 C value `THINKTHEN_EVENT_ROW_V1`.
 */
#define THINKTHEN_EVENT_ROW_V1 2u

/*
 C value `THINKTHEN_FUNCTION_ANNOTATE_V1`.
 */
#define THINKTHEN_FUNCTION_ANNOTATE_V1 8u

/*
 C value `THINKTHEN_FUNCTION_CHOOSE_V1`.
 */
#define THINKTHEN_FUNCTION_CHOOSE_V1 2u

/*
 C value `THINKTHEN_FUNCTION_DECIDE_V1`.
 */
#define THINKTHEN_FUNCTION_DECIDE_V1 1u

/*
 C value `THINKTHEN_FUNCTION_FILTER_V1`.
 */
#define THINKTHEN_FUNCTION_FILTER_V1 5u

/*
 C value `THINKTHEN_FUNCTION_FIND_V1`.
 */
#define THINKTHEN_FUNCTION_FIND_V1 7u

/*
 C value `THINKTHEN_FUNCTION_RANK_V1`.
 */
#define THINKTHEN_FUNCTION_RANK_V1 6u

/*
 C value `THINKTHEN_FUNCTION_RECOGNIZE_V1`.
 */
#define THINKTHEN_FUNCTION_RECOGNIZE_V1 9u

/*
 C value `THINKTHEN_FUNCTION_RELATE_V1`.
 */
#define THINKTHEN_FUNCTION_RELATE_V1 10u

/*
 C value `THINKTHEN_FUNCTION_SCORE_V1`.
 */
#define THINKTHEN_FUNCTION_SCORE_V1 4u

/*
 C value `THINKTHEN_FUNCTION_TAG_V1`.
 */
#define THINKTHEN_FUNCTION_TAG_V1 3u

/*
 C value `THINKTHEN_ID_FAILURE_V1`.
 */
#define THINKTHEN_ID_FAILURE_V1 2u

/*
 C value `THINKTHEN_ID_OBSERVATION_V1`.
 */
#define THINKTHEN_ID_OBSERVATION_V1 1u

/*
 C value `THINKTHEN_IMAGE_JPEG_V1`.
 */
#define THINKTHEN_IMAGE_JPEG_V1 1u

/*
 C value `THINKTHEN_IMAGE_PNG_V1`.
 */
#define THINKTHEN_IMAGE_PNG_V1 2u

/*
 C value `THINKTHEN_MEMBER_FAILURE_V1`.
 */
#define THINKTHEN_MEMBER_FAILURE_V1 2u

/*
 C value `THINKTHEN_MEMBER_INVALID_DISTRIBUTION_V1`.
 */
#define THINKTHEN_MEMBER_INVALID_DISTRIBUTION_V1 5u

/*
 C value `THINKTHEN_MEMBER_INVALID_PROBABILITY_V1`.
 */
#define THINKTHEN_MEMBER_INVALID_PROBABILITY_V1 4u

/*
 C value `THINKTHEN_MEMBER_MISSING_ANSWER_V1`.
 */
#define THINKTHEN_MEMBER_MISSING_ANSWER_V1 1u

/*
 C value `THINKTHEN_MEMBER_MISSING_PROBABILITY_V1`.
 */
#define THINKTHEN_MEMBER_MISSING_PROBABILITY_V1 3u

/*
 C value `THINKTHEN_MEMBER_SUCCESS_V1`.
 */
#define THINKTHEN_MEMBER_SUCCESS_V1 1u

/*
 C value `THINKTHEN_MEMBER_UNEXPECTED_PROBABILITY_V1`.
 */
#define THINKTHEN_MEMBER_UNEXPECTED_PROBABILITY_V1 6u

/*
 C value `THINKTHEN_MEMBER_WRONG_KIND_V1`.
 */
#define THINKTHEN_MEMBER_WRONG_KIND_V1 2u

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

/*
 C value `THINKTHEN_ORIGIN_CACHE_V1`.
 */
#define THINKTHEN_ORIGIN_CACHE_V1 2u

/*
 C value `THINKTHEN_ORIGIN_LIVE_V1`.
 */
#define THINKTHEN_ORIGIN_LIVE_V1 1u

/*
 reserved, never emitted in 0.2
 */
#define THINKTHEN_ORIGIN_MEMORY_V1 5u

/*
 reserved, never emitted in 0.2
 */
#define THINKTHEN_ORIGIN_PROXY_V1 4u

/*
 C value `THINKTHEN_ORIGIN_REPLAY_V1`.
 */
#define THINKTHEN_ORIGIN_REPLAY_V1 3u

/*
 C value `THINKTHEN_PROBABILITIES_NAMED_V1`.
 */
#define THINKTHEN_PROBABILITIES_NAMED_V1 2u

/*
 C value `THINKTHEN_PROBABILITIES_YES_V1`.
 */
#define THINKTHEN_PROBABILITIES_YES_V1 1u

/*
 C value `THINKTHEN_RELATION_CHOICE_V1`.
 */
#define THINKTHEN_RELATION_CHOICE_V1 2u

/*
 C value `THINKTHEN_RELATION_YES_NO_V1`.
 */
#define THINKTHEN_RELATION_YES_NO_V1 1u

/*
 C value `THINKTHEN_RESULT_FAILURE_V1`.
 */
#define THINKTHEN_RESULT_FAILURE_V1 2u

/*
 C value `THINKTHEN_RESULT_SUCCESS_V1`.
 */
#define THINKTHEN_RESULT_SUCCESS_V1 1u

/*
 C value `THINKTHEN_RULE_BAND_V1`.
 */
#define THINKTHEN_RULE_BAND_V1 3u

/*
 C value `THINKTHEN_RULE_CUT_V1`.
 */
#define THINKTHEN_RULE_CUT_V1 2u

/*
 input-only missing reading
 */
#define THINKTHEN_RULE_DEFAULT_V1 0u

/*
 C value `THINKTHEN_RULE_NULL_V1`.
 */
#define THINKTHEN_RULE_NULL_V1 1u

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
 Read transferred one independent packet owner.
 */
#define THINKTHEN_SESSION_RESULT_V1 0

/*
 C value `THINKTHEN_SOURCE_FILE_V1`.
 */
#define THINKTHEN_SOURCE_FILE_V1 3u

/*
 C value `THINKTHEN_SOURCE_IMAGE_FILE_V1`.
 */
#define THINKTHEN_SOURCE_IMAGE_FILE_V1 4u

/*
 C value `THINKTHEN_SOURCE_JSONL_V1`.
 */
#define THINKTHEN_SOURCE_JSONL_V1 5u

/*
 C value `THINKTHEN_SOURCE_LINE_V1`.
 */
#define THINKTHEN_SOURCE_LINE_V1 1u

/*
 C value `THINKTHEN_SOURCE_WINDOW_V1`.
 */
#define THINKTHEN_SOURCE_WINDOW_V1 2u

/*
 C value `THINKTHEN_STAGE_BOUNDARY_V1`.
 */
#define THINKTHEN_STAGE_BOUNDARY_V1 1u

/*
 C value `THINKTHEN_STAGE_EDGE_V1`.
 */
#define THINKTHEN_STAGE_EDGE_V1 3u

/*
 C value `THINKTHEN_STAGE_KIND_V1`.
 */
#define THINKTHEN_STAGE_KIND_V1 2u

/*
 C value `THINKTHEN_STAGE_RELATION_V1`.
 */
#define THINKTHEN_STAGE_RELATION_V1 4u

/*
 C value `THINKTHEN_STOP_BACKEND_V1`.
 */
#define THINKTHEN_STOP_BACKEND_V1 8u

/*
 C value `THINKTHEN_STOP_CANCELLED_V1`.
 */
#define THINKTHEN_STOP_CANCELLED_V1 9u

/*
 C value `THINKTHEN_STOP_DEADLINE_V1`.
 */
#define THINKTHEN_STOP_DEADLINE_V1 11u

/*
 C value `THINKTHEN_STOP_DEFECT_V1`.
 */
#define THINKTHEN_STOP_DEFECT_V1 10u

/*
 C value `THINKTHEN_STOP_LOCAL_V1`.
 */
#define THINKTHEN_STOP_LOCAL_V1 2u

/*
 C value `THINKTHEN_STOP_NO_KEY_V1`.
 */
#define THINKTHEN_STOP_NO_KEY_V1 3u

/*
 C value `THINKTHEN_STOP_REPLY_V1`.
 */
#define THINKTHEN_STOP_REPLY_V1 7u

/*
 C value `THINKTHEN_STOP_STATUS_V1`.
 */
#define THINKTHEN_STOP_STATUS_V1 5u

/*
 C value `THINKTHEN_STOP_TOO_LARGE_V1`.
 */
#define THINKTHEN_STOP_TOO_LARGE_V1 6u

/*
 C value `THINKTHEN_STOP_TRANSPORT_V1`.
 */
#define THINKTHEN_STOP_TRANSPORT_V1 4u

/*
 C value `THINKTHEN_STOP_USAGE_V1`.
 */
#define THINKTHEN_STOP_USAGE_V1 1u

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
 A same-thread native batch. Field order drops/joins the batch before its backing.
 */
typedef struct thinkthen_batch thinkthen_batch;

/*
 thinkthen.h is the single C header for thinkthen, version 0.2.0.
 Plain calls equal their _opts twin with THINKTHEN_NO_DEADLINE and a NULL token.
 Engines serve concurrent callers and rebuild state after a fork; free them
 only after all calls return. Free owned strings with thinkthen_free_string.
 Error messages and failure facts are borrowed; never free their pointers.
 See thinkthen_error_message for engine and NULL-engine pointer lifetimes.
 Nonzero returns leave all outputs unchanged. Eager calls have no partial
 rows; lazy batches retain completed prefixes. Bare typed, *_with_facts, collecting JSON, complete and batch forms retain their compatibility contracts. Frozen 0.1 exports keep names, layouts, signatures, codes and legacy JSON.
 Recommended 0.2 judgments use the owned thinkthen_session_* interface and its adjacent independent session/packet contracts. Shared engine construction and cleanup also serve it.
 Reference: https://github.com/botassembly/thinkthen/blob/main/libraries/c/DESIGN.md
 DESIGN.md references below name this online reference; archives retain
 their header/library contents.
 Argument refusals send nothing. NULL engine returns EUSAGE or NULL.
 question_json/request_json/spec_json must be NUL-terminated UTF-8.
 text reads exactly text_len bytes, never a terminator. NULL requires length
 zero; empty evidence still follows the engine's blank-evidence refusal.
 texts/lengths/bulk out have count entries; NULL is allowed only at count=0.
 Decide out and recognize/relate out/out_len must be nonnull.
 A NULL _opts token means no cancellation token.
 Opaque engine owner. Free after every concurrent call finishes.
 */
typedef struct thinkthen_engine thinkthen_engine;

/*
 Validated immutable image plus caller-authored filename, never guessed media.
 */
typedef struct thinkthen_image thinkthen_image;

/*
 Immutable question, independent of every constructor buffer/member handle.
 */
typedef struct thinkthen_question thinkthen_question;

/*
 Immutable result owner; every nested view allocation lives until this is freed.
 */
typedef struct thinkthen_result thinkthen_result;

/*
 Independent session owner. Free once after concurrent operations return.
 */
typedef struct thinkthen_session thinkthen_session;

/*
 Independent packet owner. It remains live after its session and engine are freed.
 */
typedef struct thinkthen_session_result thinkthen_session_result;

/*
 An immutable record snapshot or explicit shared native reader selection.
 */
typedef struct thinkthen_source thinkthen_source;

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
 C descriptor or borrowed view `ContentV1`.
 */
typedef struct thinkthen_content_v1 {
  /*
   C field `kind`.
   */
  uint32_t kind;
  /*
   C field `data`.
   */
  struct thinkthen_string_v1 data;
} thinkthen_content_v1;

/*
 C descriptor or borrowed view `OptionalContentV1`.
 */
typedef struct thinkthen_optional_content_v1 {
  /*
   C field `present`.
   */
  int present;
  /*
   C field `value`.
   */
  struct thinkthen_content_v1 value;
} thinkthen_optional_content_v1;

/*
 C descriptor or borrowed view `OptionalSizeV1`.
 */
typedef struct thinkthen_optional_size_v1 {
  /*
   C field `present`.
   */
  int present;
  /*
   C field `value`.
   */
  size_t value;
} thinkthen_optional_size_v1;

/*
 C descriptor or borrowed view `ControlsV1`.
 */
typedef struct thinkthen_controls_v1 {
  /*
   C field `deadline_ms`.
   */
  int64_t deadline_ms;
  /*
   C field `cancel`.
   */
  thinkthen_cancel_token *cancel;
  /*
   C field `context`.
   */
  struct thinkthen_optional_content_v1 context;
  /*
   C field `batch`.
   */
  struct thinkthen_optional_size_v1 batch;
  /*
   C field `batch_max`.
   */
  int batch_max;
  /*
   C field `attempts`.
   */
  int attempts;
  /*
   C field `surface`.
   */
  struct thinkthen_string_v1 surface;
} thinkthen_controls_v1;

/*
 The header's `thinkthen_answer`: the outcome code, then the probability
 of yes. Two fixed fields, never a third.
 */
typedef struct thinkthen_answer {
  /*
   `THINKTHEN_YES`, `THINKTHEN_NO`, or `THINKTHEN_UNSURE`.
   */
  int outcome;
  /*
   The probability the backend gave the yes side.
   */
  double probability;
} thinkthen_answer;

typedef struct thinkthen_complete_usage_persistence_v1 {
  uint32_t kind;
} thinkthen_complete_usage_persistence_v1;

typedef struct thinkthen_complete_utf8_v1 {
  const char *data;
  size_t len;
} thinkthen_complete_utf8_v1;

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
 C descriptor or borrowed view `ImageViewV1`.
 */
typedef struct thinkthen_image_view_v1 {
  /*
   C field `media`.
   */
  uint32_t media;
  /*
   C field `bytes`.
   */
  const uint8_t *bytes;
  /*
   C field `bytes_len`.
   */
  size_t bytes_len;
  /*
   C field `width`.
   */
  uint32_t width;
  /*
   C field `height`.
   */
  uint32_t height;
  /*
   C field `filename`.
   */
  struct thinkthen_optional_string_v1 filename;
} thinkthen_image_view_v1;

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
 C descriptor or borrowed view `OptionalDoubleV1`.
 */
typedef struct thinkthen_optional_double_v1 {
  /*
   C field `present`.
   */
  int present;
  /*
   C field `value`.
   */
  double value;
} thinkthen_optional_double_v1;

/*
 C descriptor or borrowed view `ChoiceV1`.
 */
typedef struct thinkthen_choice_v1 {
  /*
   C field `name`.
   */
  struct thinkthen_string_v1 name;
  /*
   C field `description`.
   */
  struct thinkthen_optional_content_v1 description;
  /*
   C field `weight`.
   */
  struct thinkthen_optional_double_v1 weight;
} thinkthen_choice_v1;

/*
 C descriptor or borrowed view `ChoicesV1`.
 */
typedef struct thinkthen_choices_v1 {
  /*
   C field `data`.
   */
  const struct thinkthen_choice_v1 *data;
  /*
   C field `len`.
   */
  size_t len;
} thinkthen_choices_v1;

/*
 CUT uses low; BAND uses low/high; NULL/DEFAULT use neither.
 */
typedef struct thinkthen_rule_v1 {
  /*
   C field `kind`.
   */
  uint32_t kind;
  /*
   C field `low`.
   */
  double low;
  /*
   C field `high`.
   */
  double high;
} thinkthen_rule_v1;

/*
 C descriptor or borrowed view `MemberSpecV1`.
 */
typedef struct thinkthen_member_spec_v1 {
  /*
   C field `name`.
   */
  struct thinkthen_string_v1 name;
  /*
   C field `question`.
   */
  const struct thinkthen_question *question;
} thinkthen_member_spec_v1;

/*
 C descriptor or borrowed view `MemberSpecsV1`.
 */
typedef struct thinkthen_member_specs_v1 {
  /*
   C field `data`.
   */
  const struct thinkthen_member_spec_v1 *data;
  /*
   C field `len`.
   */
  size_t len;
} thinkthen_member_specs_v1;

/*
 C descriptor or borrowed view `RelationV1`.
 */
typedef struct thinkthen_relation_v1 {
  /*
   C field `name`.
   */
  struct thinkthen_string_v1 name;
  /*
   C field `source`.
   */
  struct thinkthen_string_v1 source;
  /*
   C field `target`.
   */
  struct thinkthen_string_v1 target;
  /*
   C field `reads`.
   */
  struct thinkthen_optional_string_v1 reads;
  /*
   C field `either`.
   */
  int either;
  /*
   C field `single`.
   */
  int single;
} thinkthen_relation_v1;

/*
 C descriptor or borrowed view `RelationsV1`.
 */
typedef struct thinkthen_relations_v1 {
  /*
   C field `data`.
   */
  const struct thinkthen_relation_v1 *data;
  /*
   C field `len`.
   */
  size_t len;
} thinkthen_relations_v1;

/*
 C descriptor or borrowed view `QuestionSpecV1`.
 */
typedef struct thinkthen_question_spec_v1 {
  /*
   C field `kind`.
   */
  uint32_t kind;
  /*
   C field `text`.
   */
  struct thinkthen_content_v1 text;
  /*
   C field `yes`.
   */
  struct thinkthen_optional_content_v1 yes;
  /*
   C field `no`.
   */
  struct thinkthen_optional_content_v1 no;
  /*
   C field `choices`.
   */
  struct thinkthen_choices_v1 choices;
  /*
   C field `threshold`.
   */
  struct thinkthen_rule_v1 threshold;
  /*
   C field `relation_threshold`.
   */
  struct thinkthen_rule_v1 relation_threshold;
  /*
   C field `model`.
   */
  struct thinkthen_optional_string_v1 model;
  /*
   C field `profile`.
   */
  struct thinkthen_optional_string_v1 profile;
  /*
   C field `batch`.
   */
  struct thinkthen_optional_size_v1 batch;
  /*
   C field `batch_max`.
   */
  int batch_max;
  /*
   C field `none`.
   */
  int none;
  /*
   C field `on`.
   */
  struct thinkthen_strings_v1 on;
  /*
   C field `members`.
   */
  struct thinkthen_member_specs_v1 members;
  /*
   C field `kinds`.
   */
  struct thinkthen_choices_v1 kinds;
  /*
   C field `relations`.
   */
  struct thinkthen_relations_v1 relations;
  /*
   C field `name_pointer`.
   */
  struct thinkthen_optional_string_v1 name_pointer;
  /*
   C field `kind_pointer`.
   */
  struct thinkthen_optional_string_v1 kind_pointer;
} thinkthen_question_spec_v1;

/*
 Additive 0.3 recognition task; existing V1 question layouts remain unchanged.
 */
typedef struct thinkthen_recognition_task_v1 {
  /*
   C field `instructions`.
   */
  struct thinkthen_optional_string_v1 instructions;
  /*
   C field `entity_definition`.
   */
  struct thinkthen_optional_string_v1 entity_definition;
} thinkthen_recognition_task_v1;

/*
 C descriptor or borrowed view `QuestionMemberV1`.
 */
typedef struct thinkthen_question_member_v1 {
  /*
   C field `name`.
   */
  struct thinkthen_string_v1 name;
  /*
   C field `question`.
   */
  const struct thinkthen_question_view_v1 *question;
} thinkthen_question_member_v1;

/*
 C descriptor or borrowed view `QuestionMembersV1`.
 */
typedef struct thinkthen_question_members_v1 {
  /*
   C field `data`.
   */
  const struct thinkthen_question_member_v1 *data;
  /*
   C field `len`.
   */
  size_t len;
} thinkthen_question_members_v1;

/*
 Result questions expose nested questions directly, with no engine handle.
 */
typedef struct thinkthen_question_view_v1 {
  /*
   C field `kind`.
   */
  uint32_t kind;
  /*
   C field `text`.
   */
  struct thinkthen_content_v1 text;
  /*
   C field `yes`.
   */
  struct thinkthen_optional_content_v1 yes;
  /*
   C field `no`.
   */
  struct thinkthen_optional_content_v1 no;
  /*
   C field `choices`.
   */
  struct thinkthen_choices_v1 choices;
  /*
   C field `threshold`.
   */
  struct thinkthen_rule_v1 threshold;
  /*
   C field `relation_threshold`.
   */
  struct thinkthen_rule_v1 relation_threshold;
  /*
   C field `model`.
   */
  struct thinkthen_optional_string_v1 model;
  /*
   C field `profile`.
   */
  struct thinkthen_optional_string_v1 profile;
  /*
   C field `batch`.
   */
  struct thinkthen_optional_size_v1 batch;
  /*
   C field `batch_max`.
   */
  int batch_max;
  /*
   C field `none`.
   */
  int none;
  /*
   C field `on`.
   */
  struct thinkthen_strings_v1 on;
  /*
   C field `members`.
   */
  struct thinkthen_question_members_v1 members;
  /*
   C field `kinds`.
   */
  struct thinkthen_choices_v1 kinds;
  /*
   C field `relations`.
   */
  struct thinkthen_relations_v1 relations;
  /*
   C field `name_pointer`.
   */
  struct thinkthen_optional_string_v1 name_pointer;
  /*
   C field `kind_pointer`.
   */
  struct thinkthen_optional_string_v1 kind_pointer;
} thinkthen_question_view_v1;

/*
 C descriptor or borrowed view `OptionalQuestionV1`.
 */
typedef struct thinkthen_optional_question_v1 {
  /*
   C field `present`.
   */
  int present;
  /*
   C field `value`.
   */
  struct thinkthen_question_view_v1 value;
} thinkthen_optional_question_v1;

/*
 C descriptor or borrowed view `ProbabilityV1`.
 */
typedef struct thinkthen_probability_v1 {
  /*
   C field `name`.
   */
  struct thinkthen_string_v1 name;
  /*
   C field `probability`.
   */
  double probability;
} thinkthen_probability_v1;

/*
 C descriptor or borrowed view `ProbabilitiesV1`.
 */
typedef struct thinkthen_probabilities_v1 {
  /*
   C field `data`.
   */
  const struct thinkthen_probability_v1 *data;
  /*
   C field `len`.
   */
  size_t len;
} thinkthen_probabilities_v1;

/*
 C descriptor or borrowed view `NamedAnswerV1`.
 */
typedef struct thinkthen_named_answer_v1 {
  /*
   C field `pick`.
   */
  struct thinkthen_string_v1 pick;
  /*
   C field `probabilities`.
   */
  struct thinkthen_probabilities_v1 probabilities;
  /*
   C field `confidence`.
   */
  struct thinkthen_optional_double_v1 confidence;
} thinkthen_named_answer_v1;

/*
 C descriptor or borrowed view `ScoreAnswerV1`.
 */
typedef struct thinkthen_score_answer_v1 {
  /*
   C field `level`.
   */
  struct thinkthen_string_v1 level;
  /*
   C field `probabilities`.
   */
  struct thinkthen_probabilities_v1 probabilities;
  /*
   C field `confidence`.
   */
  struct thinkthen_optional_double_v1 confidence;
} thinkthen_score_answer_v1;

/*
 C union `AnswerDataV1`; the parent discriminator selects its active arm.
 */
typedef union thinkthen_answer_data_v1 {
  /*
   Active arm selected by the parent discriminator.
   */
  double probability;
  /*
   C field `choice`.
   */
  struct thinkthen_named_answer_v1 choice;
  /*
   C field `tag`.
   */
  struct thinkthen_probabilities_v1 tag;
  /*
   C field `score`.
   */
  struct thinkthen_score_answer_v1 score;
  /*
   C field `find`.
   */
  struct thinkthen_named_answer_v1 find;
} thinkthen_answer_data_v1;

/*
 C descriptor or borrowed view `AnswerV1`.
 */
typedef struct thinkthen_answer_v1 {
  /*
   C field `kind`.
   */
  uint32_t kind;
  /*
   C field `data`.
   */
  union thinkthen_answer_data_v1 data;
} thinkthen_answer_v1;

/*
 C descriptor or borrowed view `OptionalAnswerV1`.
 */
typedef struct thinkthen_optional_answer_v1 {
  /*
   C field `present`.
   */
  int present;
  /*
   C field `value`.
   */
  struct thinkthen_answer_v1 value;
} thinkthen_optional_answer_v1;

/*
 C descriptor or borrowed view `OptionalRuleV1`.
 */
typedef struct thinkthen_optional_rule_v1 {
  /*
   C field `present`.
   */
  int present;
  /*
   C field `value`.
   */
  struct thinkthen_rule_v1 value;
} thinkthen_optional_rule_v1;

/*
 C descriptor or borrowed view `LocationV1`.
 */
typedef struct thinkthen_location_v1 {
  /*
   C field `file`.
   */
  struct thinkthen_optional_string_v1 file;
  /*
   C field `first_line`.
   */
  struct thinkthen_optional_size_v1 first_line;
  /*
   C field `last_line`.
   */
  struct thinkthen_optional_size_v1 last_line;
} thinkthen_location_v1;

/*
 C descriptor or borrowed view `OptionalLocationV1`.
 */
typedef struct thinkthen_optional_location_v1 {
  /*
   C field `present`.
   */
  int present;
  /*
   C field `value`.
   */
  struct thinkthen_location_v1 value;
} thinkthen_optional_location_v1;

/*
 C descriptor or borrowed view `UsageV1`.
 */
typedef struct thinkthen_usage_v1 {
  /*
   C field `input_tokens`.
   */
  uint64_t input_tokens;
  /*
   C field `output_tokens`.
   */
  uint64_t output_tokens;
} thinkthen_usage_v1;

/*
 C descriptor or borrowed view `OptionalUsageV1`.
 */
typedef struct thinkthen_optional_usage_v1 {
  /*
   C field `present`.
   */
  int present;
  /*
   C field `value`.
   */
  struct thinkthen_usage_v1 value;
} thinkthen_optional_usage_v1;

/*
 C descriptor or borrowed view `ProfileWarningV1`.
 */
typedef struct thinkthen_profile_warning_v1 {
  /*
   C field `tuned_for`.
   */
  struct thinkthen_string_v1 tuned_for;
  /*
   C field `running`.
   */
  struct thinkthen_string_v1 running;
} thinkthen_profile_warning_v1;

/*
 C descriptor or borrowed view `OptionalProfileWarningV1`.
 */
typedef struct thinkthen_optional_profile_warning_v1 {
  /*
   C field `present`.
   */
  int present;
  /*
   C field `value`.
   */
  struct thinkthen_profile_warning_v1 value;
} thinkthen_optional_profile_warning_v1;

/*
 C descriptor or borrowed view `BatchV1`.
 */
typedef struct thinkthen_batch_v1 {
  /*
   C field `kind`.
   */
  uint32_t kind;
  /*
   C field `records`.
   */
  size_t records;
} thinkthen_batch_v1;

/*
 C descriptor or borrowed view `OptionalBatchV1`.
 */
typedef struct thinkthen_optional_batch_v1 {
  /*
   C field `present`.
   */
  int present;
  /*
   C field `value`.
   */
  struct thinkthen_batch_v1 value;
} thinkthen_optional_batch_v1;

/*
 C descriptor or borrowed view `BatchWarningV1`.
 */
typedef struct thinkthen_batch_warning_v1 {
  /*
   C field `tuned_for`.
   */
  struct thinkthen_batch_v1 tuned_for;
  /*
   C field `running`.
   */
  struct thinkthen_batch_v1 running;
} thinkthen_batch_warning_v1;

/*
 C descriptor or borrowed view `OptionalBatchWarningV1`.
 */
typedef struct thinkthen_optional_batch_warning_v1 {
  /*
   C field `present`.
   */
  int present;
  /*
   C field `value`.
   */
  struct thinkthen_batch_warning_v1 value;
} thinkthen_optional_batch_warning_v1;

/*
 C descriptor or borrowed view `OptionalU16V1`.
 */
typedef struct thinkthen_optional_u16_v1 {
  /*
   C field `present`.
   */
  int present;
  /*
   C field `value`.
   */
  uint16_t value;
} thinkthen_optional_u16_v1;

/*
 C descriptor or borrowed view `AttemptV1`.
 */
typedef struct thinkthen_attempt_v1 {
  /*
   C field `ordinal`.
   */
  uint64_t ordinal;
  /*
   C field `request_sha256`.
   */
  struct thinkthen_string_v1 request_sha256;
  /*
   C field `wall_ms`.
   */
  uint64_t wall_ms;
  /*
   C field `outcome`.
   */
  uint32_t outcome;
  /*
   C field `sdk_request_id`.
   */
  struct thinkthen_string_v1 sdk_request_id;
  /*
   C field `status`.
   */
  struct thinkthen_optional_u16_v1 status;
  /*
   C field `server_ms`.
   */
  struct thinkthen_optional_u64_v1 server_ms;
  /*
   C field `request_id`.
   */
  struct thinkthen_optional_string_v1 request_id;
} thinkthen_attempt_v1;

/*
 C descriptor or borrowed view `AttemptsV1`.
 */
typedef struct thinkthen_attempts_v1 {
  /*
   C field `data`.
   */
  const struct thinkthen_attempt_v1 *data;
  /*
   C field `len`.
   */
  size_t len;
} thinkthen_attempts_v1;

/*
 C descriptor or borrowed view `OptionalAttemptsV1`.
 */
typedef struct thinkthen_optional_attempts_v1 {
  /*
   C field `present`.
   */
  int present;
  /*
   C field `value`.
   */
  struct thinkthen_attempts_v1 value;
} thinkthen_optional_attempts_v1;

/*
 C descriptor or borrowed view `OptionalDiscriminatorV1`.
 */
typedef struct thinkthen_optional_discriminator_v1 {
  /*
   C field `present`.
   */
  int present;
  /*
   C field `value`.
   */
  uint32_t value;
} thinkthen_optional_discriminator_v1;

/*
 reserved, never emitted in 0.2
 */
typedef struct thinkthen_question_source_v1 {
  /*
   C field `origin`.
   */
  uint32_t origin;
  /*
   C field `answered_by`.
   */
  struct thinkthen_string_v1 answered_by;
} thinkthen_question_source_v1;

/*
 C descriptor or borrowed view `QuestionSourcesV1`.
 */
typedef struct thinkthen_question_sources_v1 {
  /*
   C field `data`.
   */
  const struct thinkthen_question_source_v1 *data;
  /*
   C field `len`.
   */
  size_t len;
} thinkthen_question_sources_v1;

/*
 C union `ObservationIdentityDataV1`; the parent discriminator selects its active arm.
 */
typedef union thinkthen_observation_identity_data_v1 {
  /*
   Active arm selected by the parent discriminator.
   */
  struct thinkthen_string_v1 observation_id;
  /*
   C field `failure_id`.
   */
  struct thinkthen_string_v1 failure_id;
} thinkthen_observation_identity_data_v1;

/*
 C descriptor or borrowed view `ObservationIdentityV1`.
 */
typedef struct thinkthen_observation_identity_v1 {
  /*
   C field `kind`.
   */
  uint32_t kind;
  /*
   C field `data`.
   */
  union thinkthen_observation_identity_data_v1 data;
} thinkthen_observation_identity_v1;

/*
 C descriptor or borrowed view `ObservationIdentitiesV1`.
 */
typedef struct thinkthen_observation_identities_v1 {
  /*
   C field `data`.
   */
  const struct thinkthen_observation_identity_v1 *data;
  /*
   C field `len`.
   */
  size_t len;
} thinkthen_observation_identities_v1;

/*
 C descriptor or borrowed view `MetaV1`.
 */
typedef struct thinkthen_meta_v1 {
  /*
   C field `tool`.
   */
  struct thinkthen_string_v1 tool;
  /*
   C field `question_sha256`.
   */
  struct thinkthen_optional_string_v1 question_sha256;
  /*
   C field `questions_sha256`.
   */
  struct thinkthen_optional_string_v1 questions_sha256;
  /*
   C field `url`.
   */
  struct thinkthen_string_v1 url;
  /*
   C field `model`.
   */
  struct thinkthen_string_v1 model;
  /*
   C field `usage`.
   */
  struct thinkthen_optional_usage_v1 usage;
  /*
   C field `requests_sent`.
   */
  uint64_t requests_sent;
  /*
   C field `cached`.
   */
  int cached;
  /*
   C field `requests`.
   */
  struct thinkthen_strings_v1 requests;
  /*
   C field `failed_questions`.
   */
  size_t failed_questions;
  /*
   C field `profile_warning`.
   */
  struct thinkthen_optional_profile_warning_v1 profile_warning;
  /*
   C field `batch_setting`.
   */
  struct thinkthen_optional_batch_v1 batch_setting;
  /*
   C field `batch_warning`.
   */
  struct thinkthen_optional_batch_warning_v1 batch_warning;
  /*
   C field `context_sha256`.
   */
  struct thinkthen_optional_string_v1 context_sha256;
  /*
   C field `attempts`.
   */
  struct thinkthen_optional_attempts_v1 attempts;
  /*
   C field `origin`.
   */
  struct thinkthen_optional_discriminator_v1 origin;
  /*
   C field `question_sources`.
   */
  struct thinkthen_question_sources_v1 question_sources;
  /*
   C field `observations`.
   */
  struct thinkthen_observation_identities_v1 observations;
  /*
   C field `answered_by`.
   */
  struct thinkthen_optional_string_v1 answered_by;
} thinkthen_meta_v1;

/*
 C descriptor or borrowed view `ImageViewsV1`.
 */
typedef struct thinkthen_image_views_v1 {
  /*
   C field `data`.
   */
  const struct thinkthen_image_view_v1 *data;
  /*
   C field `len`.
   */
  size_t len;
} thinkthen_image_views_v1;

/*
 C descriptor or borrowed view `OptionalImageViewsV1`.
 */
typedef struct thinkthen_optional_image_views_v1 {
  /*
   C field `present`.
   */
  int present;
  /*
   C field `value`.
   */
  struct thinkthen_image_views_v1 value;
} thinkthen_optional_image_views_v1;

/*
 C descriptor or borrowed view `RowV1`.
 */
typedef struct thinkthen_row_v1 {
  /*
   C field `answer_id`.
   */
  struct thinkthen_string_v1 answer_id;
  /*
   C field `input`.
   */
  struct thinkthen_optional_content_v1 input;
  /*
   C field `question`.
   */
  struct thinkthen_optional_question_v1 question;
  /*
   C field `answer`.
   */
  struct thinkthen_optional_answer_v1 answer;
  /*
   C field `threshold`.
   */
  struct thinkthen_optional_rule_v1 threshold;
  /*
   C field `position`.
   */
  struct thinkthen_optional_location_v1 position;
  /*
   C field `input_file`.
   */
  struct thinkthen_optional_string_v1 input_file;
  /*
   C field `meta`.
   */
  struct thinkthen_meta_v1 meta;
  /*
   C field `images`.
   */
  struct thinkthen_optional_image_views_v1 images;
} thinkthen_row_v1;

/*
 C union `DecideValueDataV1`; the parent discriminator selects its active arm.
 */
typedef union thinkthen_decide_value_data_v1 {
  /*
   Active arm selected by the parent discriminator.
   */
  int boolean;
  /*
   C field `authored`.
   */
  struct thinkthen_content_v1 authored;
} thinkthen_decide_value_data_v1;

/*
 BOOLEAN is used only for ordinary un-authored true/false decisions.
 AUTHORED preserves the chosen user meaning even if it spells a Boolean;
 successful uncertainty always uses NULL. No JSON decode for that case.
 */
typedef struct thinkthen_decide_value_v1 {
  /*
   C field `kind`.
   */
  uint32_t kind;
  /*
   C field `data`.
   */
  union thinkthen_decide_value_data_v1 data;
} thinkthen_decide_value_v1;

/*
 C union `MemberValueDataV1`; the parent discriminator selects its active arm.
 */
typedef union thinkthen_member_value_data_v1 {
  /*
   Active arm selected by the parent discriminator.
   */
  struct thinkthen_decide_value_v1 decide;
  /*
   C field `choose`.
   */
  struct thinkthen_optional_string_v1 choose;
  /*
   C field `tag`.
   */
  struct thinkthen_strings_v1 tag;
  /*
   C field `score`.
   */
  double score;
} thinkthen_member_value_data_v1;

/*
 Annotate admits these four kinds. The discriminator uses FUNCTION_*.
 A null decide/choose is a successful typed value, not member failure.
 */
typedef struct thinkthen_member_value_v1 {
  /*
   C field `kind`.
   */
  uint32_t kind;
  /*
   C field `data`.
   */
  union thinkthen_member_value_data_v1 data;
} thinkthen_member_value_v1;

/*
 C descriptor or borrowed view `MemberSuccessV1`.
 */
typedef struct thinkthen_member_success_v1 {
  /*
   C field `answer_id`.
   */
  struct thinkthen_string_v1 answer_id;
  /*
   C field `value`.
   */
  struct thinkthen_member_value_v1 value;
  /*
   C field `answer`.
   */
  struct thinkthen_answer_v1 answer;
  /*
   C field `threshold`.
   */
  struct thinkthen_rule_v1 threshold;
} thinkthen_member_success_v1;

/*
 C descriptor or borrowed view `MemberFailureV1`.
 */
typedef struct thinkthen_member_failure_v1 {
  /*
   C field `failure_id`.
   */
  struct thinkthen_string_v1 failure_id;
  /*
   C field `cause`.
   */
  uint32_t cause;
} thinkthen_member_failure_v1;

/*
 C union `MemberDataV1`; the parent discriminator selects its active arm.
 */
typedef union thinkthen_member_data_v1 {
  /*
   Active arm selected by the parent discriminator.
   */
  struct thinkthen_member_success_v1 success;
  /*
   C field `failure`.
   */
  struct thinkthen_member_failure_v1 failure;
} thinkthen_member_data_v1;

/*
 C descriptor or borrowed view `MemberV1`.
 */
typedef struct thinkthen_member_v1 {
  /*
   C field `name`.
   */
  struct thinkthen_string_v1 name;
  /*
   C field `request`.
   */
  struct thinkthen_string_v1 request;
  /*
   C field `question`.
   */
  struct thinkthen_question_view_v1 question;
  /*
   C field `state`.
   */
  uint32_t state;
  /*
   C field `data`.
   */
  union thinkthen_member_data_v1 data;
} thinkthen_member_v1;

/*
 C descriptor or borrowed view `MembersV1`.
 */
typedef struct thinkthen_members_v1 {
  /*
   C field `data`.
   */
  const struct thinkthen_member_v1 *data;
  /*
   C field `len`.
   */
  size_t len;
} thinkthen_members_v1;

/*
 C descriptor or borrowed view `AnnotateViewV1`.
 */
typedef struct thinkthen_annotate_view_v1 {
  /*
   C field `common`.
   */
  struct thinkthen_row_v1 common;
  /*
   C field `answers`.
   */
  struct thinkthen_members_v1 answers;
} thinkthen_annotate_view_v1;

/*
 C descriptor or borrowed view `ChooseViewV1`.
 */
typedef struct thinkthen_choose_view_v1 {
  /*
   C field `common`.
   */
  struct thinkthen_row_v1 common;
  /*
   C field `value`.
   */
  struct thinkthen_optional_string_v1 value;
} thinkthen_choose_view_v1;

/*
 C descriptor or borrowed view `DecideViewV1`.
 */
typedef struct thinkthen_decide_view_v1 {
  /*
   C field `common`.
   */
  struct thinkthen_row_v1 common;
  /*
   C field `value`.
   */
  struct thinkthen_decide_value_v1 value;
} thinkthen_decide_view_v1;

/*
 Additive complete detail accessors; all pointers borrow result ownership.
 The singular question-observation observation_id is NULL/zero-length for
 aggregate questions; details.observations retains every actual identity.
 */
typedef struct thinkthen_reported_usage_v1 {
  /*
   C field `present`.
   */
  int present;
  /*
   C field `input_tokens`.
   */
  struct thinkthen_optional_u64_v1 input_tokens;
  /*
   C field `output_tokens`.
   */
  struct thinkthen_optional_u64_v1 output_tokens;
} thinkthen_reported_usage_v1;

/*
 C descriptor or borrowed view `SourceDetailV1`.
 */
typedef struct thinkthen_source_detail_v1 {
  /*
   C field `origin`.
   */
  uint32_t origin;
  /*
   C field `answered_by`.
   */
  struct thinkthen_string_v1 answered_by;
  /*
   C field `batch_size`.
   */
  struct thinkthen_optional_size_v1 batch_size;
} thinkthen_source_detail_v1;

/*
 C descriptor or borrowed view `SourceDetailsV1`.
 */
typedef struct thinkthen_source_details_v1 {
  /*
   C field `data`.
   */
  const struct thinkthen_source_detail_v1 *data;
  /*
   C field `len`.
   */
  size_t len;
} thinkthen_source_details_v1;

/*
 C descriptor or borrowed view `InputViewV1`.
 */
typedef struct thinkthen_input_view_v1 {
  /*
   C field `original`.
   */
  struct thinkthen_optional_content_v1 original;
  /*
   C field `position`.
   */
  struct thinkthen_optional_location_v1 position;
  /*
   C field `images`.
   */
  struct thinkthen_optional_image_views_v1 images;
} thinkthen_input_view_v1;

/*
 C descriptor or borrowed view `InputViewsV1`.
 */
typedef struct thinkthen_input_views_v1 {
  /*
   C field `data`.
   */
  const struct thinkthen_input_view_v1 *data;
  /*
   C field `len`.
   */
  size_t len;
} thinkthen_input_views_v1;

/*
 C descriptor or borrowed view `DetailsV1`.
 */
typedef struct thinkthen_details_v1 {
  /*
   C field `question`.
   */
  struct thinkthen_optional_question_v1 question;
  /*
   C field `threshold`.
   */
  struct thinkthen_optional_rule_v1 threshold;
  /*
   C field `raw_pick`.
   */
  struct thinkthen_optional_string_v1 raw_pick;
  /*
   C field `usage`.
   */
  struct thinkthen_reported_usage_v1 usage;
  /*
   C field `question_sources`.
   */
  struct thinkthen_source_details_v1 question_sources;
  /*
   C field `observations`.
   */
  struct thinkthen_observation_identities_v1 observations;
  /*
   C field `inputs`.
   */
  struct thinkthen_input_views_v1 inputs;
} thinkthen_details_v1;

/*
 C descriptor or borrowed view `FilterViewV1`.
 */
typedef struct thinkthen_filter_view_v1 {
  /*
   C field `common`.
   */
  struct thinkthen_row_v1 common;
  /*
   C field `value`.
   */
  int value;
} thinkthen_filter_view_v1;

/*
 C descriptor or borrowed view `FindViewV1`.
 */
typedef struct thinkthen_find_view_v1 {
  /*
   C field `common`.
   */
  struct thinkthen_row_v1 common;
  /*
   C field `value`.
   */
  struct thinkthen_optional_content_v1 value;
  /*
   C field `index`.
   */
  struct thinkthen_optional_size_v1 index;
} thinkthen_find_view_v1;

/*
 C union `ObservedProbabilitiesDataV1`; the parent discriminator selects its active arm.
 */
typedef union thinkthen_observed_probabilities_data_v1 {
  /*
   Active arm selected by the parent discriminator.
   */
  double yes;
  /*
   C field `named`.
   */
  struct thinkthen_probabilities_v1 named;
} thinkthen_observed_probabilities_data_v1;

/*
 C descriptor or borrowed view `ObservedProbabilitiesV1`.
 */
typedef struct thinkthen_observed_probabilities_v1 {
  /*
   C field `kind`.
   */
  uint32_t kind;
  /*
   C field `data`.
   */
  union thinkthen_observed_probabilities_data_v1 data;
} thinkthen_observed_probabilities_v1;

/*
 C descriptor or borrowed view `ObservationSuccessV1`.
 */
typedef struct thinkthen_observation_success_v1 {
  /*
   C field `answer_id`.
   */
  struct thinkthen_string_v1 answer_id;
  /*
   C field `observation_id`.
   */
  struct thinkthen_string_v1 observation_id;
  /*
   C field `value`.
   */
  struct thinkthen_member_value_v1 value;
  /*
   C field `probabilities`.
   */
  struct thinkthen_observed_probabilities_v1 probabilities;
  /*
   C field `confidence`.
   */
  struct thinkthen_optional_double_v1 confidence;
} thinkthen_observation_success_v1;

/*
 C union `QuestionObservationDataV1`; the parent discriminator selects its active arm.
 */
typedef union thinkthen_question_observation_data_v1 {
  /*
   Active arm selected by the parent discriminator.
   */
  struct thinkthen_observation_success_v1 success;
  /*
   C field `failure`.
   */
  struct thinkthen_member_failure_v1 failure;
} thinkthen_question_observation_data_v1;

/*
 C descriptor or borrowed view `QuestionObservationV1`.
 */
typedef struct thinkthen_question_observation_v1 {
  /*
   C field `index`.
   */
  size_t index;
  /*
   C field `member`.
   */
  struct thinkthen_optional_string_v1 member;
  /*
   C field `stage`.
   */
  struct thinkthen_optional_discriminator_v1 stage;
  /*
   C field `position`.
   */
  size_t position;
  /*
   C field `question_sha256`.
   */
  struct thinkthen_string_v1 question_sha256;
  /*
   C field `model`.
   */
  struct thinkthen_string_v1 model;
  /*
   C field `url`.
   */
  struct thinkthen_string_v1 url;
  /*
   C field `requests`.
   */
  struct thinkthen_strings_v1 requests;
  /*
   C field `requests_sent`.
   */
  uint64_t requests_sent;
  /*
   C field `cached`.
   */
  int cached;
  /*
   C field `failed_questions`.
   */
  size_t failed_questions;
  /*
   C field `usage`.
   */
  struct thinkthen_optional_usage_v1 usage;
  /*
   C field `question_sources`.
   */
  struct thinkthen_question_sources_v1 question_sources;
  /*
   C field `state`.
   */
  uint32_t state;
  /*
   C field `data`.
   */
  union thinkthen_question_observation_data_v1 data;
} thinkthen_question_observation_v1;

/*
 C descriptor or borrowed view `TagViewV1`.
 */
typedef struct thinkthen_tag_view_v1 {
  /*
   C field `common`.
   */
  struct thinkthen_row_v1 common;
  /*
   C field `value`.
   */
  struct thinkthen_strings_v1 value;
} thinkthen_tag_view_v1;

/*
 C descriptor or borrowed view `ScoreViewV1`.
 */
typedef struct thinkthen_score_view_v1 {
  /*
   C field `common`.
   */
  struct thinkthen_row_v1 common;
  /*
   C field `value`.
   */
  double value;
} thinkthen_score_view_v1;

/*
 C descriptor or borrowed view `RankViewV1`.
 */
typedef struct thinkthen_rank_view_v1 {
  /*
   C field `common`.
   */
  struct thinkthen_row_v1 common;
  /*
   C field `value`.
   */
  struct thinkthen_optional_size_v1 value;
  /*
   C field `question_name`.
   */
  struct thinkthen_optional_string_v1 question_name;
} thinkthen_rank_view_v1;

/*
 C descriptor or borrowed view `EntityV1`.
 */
typedef struct thinkthen_entity_v1 {
  /*
   C field `text`.
   */
  struct thinkthen_string_v1 text;
  /*
   C field `start`.
   */
  size_t start;
  /*
   C field `end`.
   */
  size_t end;
  /*
   C field `length`.
   */
  size_t length;
  /*
   C field `kind`.
   */
  struct thinkthen_string_v1 kind;
  /*
   C field `strength`.
   */
  double strength;
} thinkthen_entity_v1;

/*
 C descriptor or borrowed view `EntitiesV1`.
 */
typedef struct thinkthen_entities_v1 {
  /*
   C field `data`.
   */
  const struct thinkthen_entity_v1 *data;
  /*
   C field `len`.
   */
  size_t len;
} thinkthen_entities_v1;

/*
 C descriptor or borrowed view `EntityEdgeV1`.
 */
typedef struct thinkthen_entity_edge_v1 {
  /*
   C field `relation`.
   */
  struct thinkthen_string_v1 relation;
  /*
   C field `source`.
   */
  struct thinkthen_entity_v1 source;
  /*
   C field `target`.
   */
  struct thinkthen_entity_v1 target;
  /*
   C field `probability`.
   */
  double probability;
  /*
   C field `either`.
   */
  int either;
} thinkthen_entity_edge_v1;

/*
 C descriptor or borrowed view `EntityEdgesV1`.
 */
typedef struct thinkthen_entity_edges_v1 {
  /*
   C field `data`.
   */
  const struct thinkthen_entity_edge_v1 *data;
  /*
   C field `len`.
   */
  size_t len;
} thinkthen_entity_edges_v1;

/*
 C descriptor or borrowed view `OptionalEntityEdgesV1`.
 */
typedef struct thinkthen_optional_entity_edges_v1 {
  /*
   C field `present`.
   */
  int present;
  /*
   C field `value`.
   */
  struct thinkthen_entity_edges_v1 value;
} thinkthen_optional_entity_edges_v1;

/*
 C descriptor or borrowed view `RecognizeValueV1`.
 */
typedef struct thinkthen_recognize_value_v1 {
  /*
   C field `entities`.
   */
  struct thinkthen_entities_v1 entities;
  /*
   C field `relations`.
   */
  struct thinkthen_optional_entity_edges_v1 relations;
} thinkthen_recognize_value_v1;

/*
 C descriptor or borrowed view `PieceV1`.
 */
typedef struct thinkthen_piece_v1 {
  /*
   C field `start`.
   */
  size_t start;
  /*
   C field `end`.
   */
  size_t end;
  /*
   C field `tags`.
   */
  struct thinkthen_probabilities_v1 tags;
} thinkthen_piece_v1;

/*
 C descriptor or borrowed view `PiecesV1`.
 */
typedef struct thinkthen_pieces_v1 {
  /*
   C field `data`.
   */
  const struct thinkthen_piece_v1 *data;
  /*
   C field `len`.
   */
  size_t len;
} thinkthen_pieces_v1;

/*
 C descriptor or borrowed view `OptionalProbabilitiesV1`.
 */
typedef struct thinkthen_optional_probabilities_v1 {
  /*
   C field `present`.
   */
  int present;
  /*
   C field `value`.
   */
  struct thinkthen_probabilities_v1 value;
} thinkthen_optional_probabilities_v1;

/*
 C descriptor or borrowed view `NameV1`.
 */
typedef struct thinkthen_name_v1 {
  /*
   C field `start`.
   */
  size_t start;
  /*
   C field `end`.
   */
  size_t end;
  /*
   C field `kinds`.
   */
  struct thinkthen_optional_probabilities_v1 kinds;
  /*
   C field `edges`.
   */
  struct thinkthen_optional_probabilities_v1 edges;
} thinkthen_name_v1;

/*
 C descriptor or borrowed view `NamesV1`.
 */
typedef struct thinkthen_names_v1 {
  /*
   C field `data`.
   */
  const struct thinkthen_name_v1 *data;
  /*
   C field `len`.
   */
  size_t len;
} thinkthen_names_v1;

/*
 C descriptor or borrowed view `PlaceV1`.
 */
typedef struct thinkthen_place_v1 {
  /*
   C field `start`.
   */
  size_t start;
  /*
   C field `end`.
   */
  size_t end;
} thinkthen_place_v1;

/*
 C descriptor or borrowed view `PairV1`.
 */
typedef struct thinkthen_pair_v1 {
  /*
   C field `relation`.
   */
  struct thinkthen_string_v1 relation;
  /*
   C field `source`.
   */
  struct thinkthen_place_v1 source;
  /*
   C field `target`.
   */
  struct thinkthen_place_v1 target;
  /*
   C field `probability`.
   */
  double probability;
} thinkthen_pair_v1;

/*
 C descriptor or borrowed view `PairsV1`.
 */
typedef struct thinkthen_pairs_v1 {
  /*
   C field `data`.
   */
  const struct thinkthen_pair_v1 *data;
  /*
   C field `len`.
   */
  size_t len;
} thinkthen_pairs_v1;

/*
 C descriptor or borrowed view `RecognizeAnswerV1`.
 */
typedef struct thinkthen_recognize_answer_v1 {
  /*
   C field `pieces`.
   */
  struct thinkthen_pieces_v1 pieces;
  /*
   C field `names`.
   */
  struct thinkthen_names_v1 names;
  /*
   C field `pairs`.
   */
  struct thinkthen_pairs_v1 pairs;
} thinkthen_recognize_answer_v1;

/*
 C descriptor or borrowed view `RecognizeViewV1`.
 */
typedef struct thinkthen_recognize_view_v1 {
  /*
   C field `common`.
   */
  struct thinkthen_row_v1 common;
  /*
   C field `value`.
   */
  struct thinkthen_recognize_value_v1 value;
  /*
   C field `answer`.
   */
  struct thinkthen_recognize_answer_v1 answer;
} thinkthen_recognize_view_v1;

/*
 C descriptor or borrowed view `EndpointV1`.
 */
typedef struct thinkthen_endpoint_v1 {
  /*
   C field `name`.
   */
  struct thinkthen_string_v1 name;
  /*
   C field `kind`.
   */
  struct thinkthen_string_v1 kind;
} thinkthen_endpoint_v1;

/*
 C descriptor or borrowed view `EdgeV1`.
 */
typedef struct thinkthen_edge_v1 {
  /*
   C field `relation`.
   */
  struct thinkthen_string_v1 relation;
  /*
   C field `source`.
   */
  struct thinkthen_endpoint_v1 source;
  /*
   C field `target`.
   */
  struct thinkthen_endpoint_v1 target;
  /*
   C field `probability`.
   */
  double probability;
  /*
   C field `either`.
   */
  int either;
} thinkthen_edge_v1;

/*
 C descriptor or borrowed view `EdgesV1`.
 */
typedef struct thinkthen_edges_v1 {
  /*
   C field `data`.
   */
  const struct thinkthen_edge_v1 *data;
  /*
   C field `len`.
   */
  size_t len;
} thinkthen_edges_v1;

/*
 C descriptor or borrowed view `OptionalEndpointV1`.
 */
typedef struct thinkthen_optional_endpoint_v1 {
  /*
   C field `present`.
   */
  int present;
  /*
   C field `value`.
   */
  struct thinkthen_endpoint_v1 value;
} thinkthen_optional_endpoint_v1;

/*
 C descriptor or borrowed view `RelationSuccessV1`.
 */
typedef struct thinkthen_relation_success_v1 {
  /*
   C field `answer_id`.
   */
  struct thinkthen_string_v1 answer_id;
  /*
   C field `probability`.
   */
  double probability;
  /*
   C field `accepted`.
   */
  int accepted;
} thinkthen_relation_success_v1;

/*
 C union `RelationAnswerDataV1`; the parent discriminator selects its active arm.
 */
typedef union thinkthen_relation_answer_data_v1 {
  /*
   Active arm selected by the parent discriminator.
   */
  struct thinkthen_relation_success_v1 success;
  /*
   C field `failure`.
   */
  struct thinkthen_member_failure_v1 failure;
} thinkthen_relation_answer_data_v1;

/*
 C descriptor or borrowed view `RelationAnswerV1`.
 */
typedef struct thinkthen_relation_answer_v1 {
  /*
   C field `relation`.
   */
  struct thinkthen_string_v1 relation;
  /*
   C field `reads`.
   */
  struct thinkthen_string_v1 reads;
  /*
   C field `method`.
   */
  uint32_t method;
  /*
   C field `direction`.
   */
  uint32_t direction;
  /*
   C field `source`.
   */
  struct thinkthen_endpoint_v1 source;
  /*
   C field `target`.
   */
  struct thinkthen_optional_endpoint_v1 target;
  /*
   C field `request`.
   */
  struct thinkthen_string_v1 request;
  /*
   C field `state`.
   */
  uint32_t state;
  /*
   C field `data`.
   */
  union thinkthen_relation_answer_data_v1 data;
} thinkthen_relation_answer_v1;

/*
 C descriptor or borrowed view `RelationAnswersV1`.
 */
typedef struct thinkthen_relation_answers_v1 {
  /*
   C field `data`.
   */
  const struct thinkthen_relation_answer_v1 *data;
  /*
   C field `len`.
   */
  size_t len;
} thinkthen_relation_answers_v1;

/*
 C descriptor or borrowed view `RelateViewV1`.
 */
typedef struct thinkthen_relate_view_v1 {
  /*
   C field `common`.
   */
  struct thinkthen_row_v1 common;
  /*
   C field `value`.
   */
  struct thinkthen_edges_v1 value;
  /*
   C field `questions`.
   */
  struct thinkthen_relation_answers_v1 questions;
} thinkthen_relate_view_v1;

/*
 C union `RowObservationDataV1`; the parent discriminator selects its active arm.
 */
typedef union thinkthen_row_observation_data_v1 {
  /*
   Active arm selected by the parent discriminator.
   */
  struct thinkthen_decide_view_v1 decide;
  /*
   C field `choose`.
   */
  struct thinkthen_choose_view_v1 choose;
  /*
   C field `tag`.
   */
  struct thinkthen_tag_view_v1 tag;
  /*
   C field `score`.
   */
  struct thinkthen_score_view_v1 score;
  /*
   C field `filter`.
   */
  struct thinkthen_filter_view_v1 filter;
  /*
   C field `rank`.
   */
  struct thinkthen_rank_view_v1 rank;
  /*
   C field `find`.
   */
  struct thinkthen_find_view_v1 find;
  /*
   C field `annotate`.
   */
  struct thinkthen_annotate_view_v1 annotate;
  /*
   C field `recognize`.
   */
  struct thinkthen_recognize_view_v1 recognize;
  /*
   C field `relate`.
   */
  struct thinkthen_relate_view_v1 relate;
} thinkthen_row_observation_data_v1;

/*
 C descriptor or borrowed view `RowObservationV1`.
 */
typedef struct thinkthen_row_observation_v1 {
  /*
   C field `index`.
   */
  size_t index;
  /*
   C field `function`.
   */
  uint32_t function;
  /*
   C field `data`.
   */
  union thinkthen_row_observation_data_v1 data;
} thinkthen_row_observation_v1;

/*
 C union `ObservationDataV1`; the parent discriminator selects its active arm.
 */
typedef union thinkthen_observation_data_v1 {
  /*
   Active arm selected by the parent discriminator.
   */
  struct thinkthen_question_observation_v1 question;
  /*
   C field `row`.
   */
  struct thinkthen_row_observation_v1 row;
} thinkthen_observation_data_v1;

/*
 C descriptor or borrowed view `ObservationV1`.
 */
typedef struct thinkthen_observation_v1 {
  /*
   C field `kind`.
   */
  uint32_t kind;
  /*
   C field `data`.
   */
  union thinkthen_observation_data_v1 data;
} thinkthen_observation_v1;

/*
 Additive located values come from native span/occurrence mapping. An
 unlocated call returns present=0. Every nested view borrows result ownership.
 */
typedef struct thinkthen_source_entity_v1 {
  /*
   C field `entity`.
   */
  struct thinkthen_entity_v1 entity;
  /*
   C field `position`.
   */
  struct thinkthen_optional_location_v1 position;
} thinkthen_source_entity_v1;

/*
 C descriptor or borrowed view `SourceEntitiesV1`.
 */
typedef struct thinkthen_source_entities_v1 {
  /*
   C field `data`.
   */
  const struct thinkthen_source_entity_v1 *data;
  /*
   C field `len`.
   */
  size_t len;
} thinkthen_source_entities_v1;

/*
 C descriptor or borrowed view `SourceEntityEdgeV1`.
 */
typedef struct thinkthen_source_entity_edge_v1 {
  /*
   C field `relation`.
   */
  struct thinkthen_string_v1 relation;
  /*
   C field `source`.
   */
  struct thinkthen_source_entity_v1 source;
  /*
   C field `target`.
   */
  struct thinkthen_source_entity_v1 target;
  /*
   C field `probability`.
   */
  double probability;
  /*
   C field `either`.
   */
  int either;
} thinkthen_source_entity_edge_v1;

/*
 C descriptor or borrowed view `SourceEntityEdgesV1`.
 */
typedef struct thinkthen_source_entity_edges_v1 {
  /*
   C field `data`.
   */
  const struct thinkthen_source_entity_edge_v1 *data;
  /*
   C field `len`.
   */
  size_t len;
} thinkthen_source_entity_edges_v1;

/*
 C descriptor or borrowed view `OptionalSourceEntityEdgesV1`.
 */
typedef struct thinkthen_optional_source_entity_edges_v1 {
  /*
   C field `present`.
   */
  int present;
  /*
   C field `value`.
   */
  struct thinkthen_source_entity_edges_v1 value;
} thinkthen_optional_source_entity_edges_v1;

/*
 C descriptor or borrowed view `SourceRecognitionV1`.
 */
typedef struct thinkthen_source_recognition_v1 {
  /*
   C field `present`.
   */
  int present;
  /*
   C field `entities`.
   */
  struct thinkthen_source_entities_v1 entities;
  /*
   C field `relations`.
   */
  struct thinkthen_optional_source_entity_edges_v1 relations;
} thinkthen_source_recognition_v1;

/*
 C descriptor or borrowed view `SourceEndpointV1`.
 */
typedef struct thinkthen_source_endpoint_v1 {
  /*
   C field `ordinal`.
   */
  size_t ordinal;
  /*
   C field `endpoint`.
   */
  struct thinkthen_endpoint_v1 endpoint;
  /*
   C field `record`.
   */
  struct thinkthen_content_v1 record;
  /*
   C field `position`.
   */
  struct thinkthen_optional_location_v1 position;
} thinkthen_source_endpoint_v1;

/*
 C descriptor or borrowed view `SourceEdgeV1`.
 */
typedef struct thinkthen_source_edge_v1 {
  /*
   C field `relation`.
   */
  struct thinkthen_string_v1 relation;
  /*
   C field `source`.
   */
  struct thinkthen_source_endpoint_v1 source;
  /*
   C field `target`.
   */
  struct thinkthen_source_endpoint_v1 target;
  /*
   C field `probability`.
   */
  double probability;
  /*
   C field `either`.
   */
  int either;
} thinkthen_source_edge_v1;

/*
 C descriptor or borrowed view `SourceEdgesV1`.
 */
typedef struct thinkthen_source_edges_v1 {
  /*
   C field `data`.
   */
  const struct thinkthen_source_edge_v1 *data;
  /*
   C field `len`.
   */
  size_t len;
} thinkthen_source_edges_v1;

/*
 C descriptor or borrowed view `SourceRelationsV1`.
 */
typedef struct thinkthen_source_relations_v1 {
  /*
   C field `present`.
   */
  int present;
  /*
   C field `edges`.
   */
  struct thinkthen_source_edges_v1 edges;
} thinkthen_source_relations_v1;

/*
 C descriptor or borrowed view `OptionalMetaV1`.
 */
typedef struct thinkthen_optional_meta_v1 {
  /*
   C field `present`.
   */
  int present;
  /*
   C field `value`.
   */
  struct thinkthen_meta_v1 value;
} thinkthen_optional_meta_v1;

/*
 origin.present=0 represents required JSON null, not omitted provenance.
 requests, question_sources, observations have the same length/order.
 Exactly one question digest is present: plural for annotate, singular else.
 No proxy field exists: 0.2 cannot activate it or emit proxy metadata.
 */
typedef struct thinkthen_facts_v1 {
  /*
   C field `call_id`.
   */
  struct thinkthen_string_v1 call_id;
  /*
   C field `cache_answers`.
   */
  uint64_t cache_answers;
  /*
   C field `estimated_cost_usd`.
   */
  struct thinkthen_optional_string_v1 estimated_cost_usd;
  /*
   C field `input_tokens`.
   */
  struct thinkthen_optional_u64_v1 input_tokens;
  /*
   C field `model`.
   */
  struct thinkthen_optional_string_v1 model;
  /*
   C field `output_tokens`.
   */
  struct thinkthen_optional_u64_v1 output_tokens;
  /*
   C field `records`.
   */
  uint64_t records;
  /*
   C field `requests_sent`.
   */
  uint64_t requests_sent;
  /*
   C field `seconds`.
   */
  double seconds;
  /*
   C field `command_ms`.
   */
  struct thinkthen_optional_u64_v1 command_ms;
} thinkthen_facts_v1;

/*
 C descriptor or borrowed view `OptionalFactsV1`.
 */
typedef struct thinkthen_optional_facts_v1 {
  /*
   C field `present`.
   */
  int present;
  /*
   C field `value`.
   */
  struct thinkthen_facts_v1 value;
} thinkthen_optional_facts_v1;

/*
 C descriptor or borrowed view `StoppedV1`.
 */
typedef struct thinkthen_stopped_v1 {
  /*
   C field `at`.
   */
  struct thinkthen_optional_size_v1 at;
  /*
   C field `cause`.
   */
  uint32_t cause;
  /*
   C field `status`.
   */
  struct thinkthen_optional_u16_v1 status;
  /*
   C field `retryable`.
   */
  int retryable;
} thinkthen_stopped_v1;

/*
 C descriptor or borrowed view `OptionalStoppedV1`.
 */
typedef struct thinkthen_optional_stopped_v1 {
  /*
   C field `present`.
   */
  int present;
  /*
   C field `value`.
   */
  struct thinkthen_stopped_v1 value;
} thinkthen_optional_stopped_v1;

/*
 C descriptor or borrowed view `ErrorV1`.
 */
typedef struct thinkthen_error_v1 {
  /*
   C field `code`.
   */
  int code;
  /*
   C field `message`.
   */
  struct thinkthen_string_v1 message;
  /*
   C field `retryable`.
   */
  int retryable;
  /*
   C field `stopped`.
   */
  struct thinkthen_optional_stopped_v1 stopped;
} thinkthen_error_v1;

/*
 C descriptor or borrowed view `OptionalErrorV1`.
 */
typedef struct thinkthen_optional_error_v1 {
  /*
   C field `present`.
   */
  int present;
  /*
   C field `value`.
   */
  struct thinkthen_error_v1 value;
} thinkthen_optional_error_v1;

/*
 C descriptor or borrowed view `SummaryV1`.
 */
typedef struct thinkthen_summary_v1 {
  /*
   C field `state`.
   */
  uint32_t state;
  /*
   C field `schema`.
   */
  struct thinkthen_string_v1 schema;
  /*
   C field `answer_id`.
   */
  struct thinkthen_optional_string_v1 answer_id;
  /*
   C field `function`.
   */
  struct thinkthen_optional_discriminator_v1 function;
  /*
   C field `count`.
   */
  size_t count;
  /*
   C field `observation_count`.
   */
  size_t observation_count;
  /*
   C field `meta`.
   */
  struct thinkthen_optional_meta_v1 meta;
  /*
   C field `facts`.
   */
  struct thinkthen_optional_facts_v1 facts;
  /*
   C field `attempts`.
   */
  struct thinkthen_optional_attempts_v1 attempts;
  /*
   C field `error`.
   */
  struct thinkthen_optional_error_v1 error;
} thinkthen_summary_v1;

typedef struct thinkthen_complete_extension_v1 {
  struct thinkthen_complete_utf8_v1 name;
  struct thinkthen_complete_utf8_v1 json;
} thinkthen_complete_extension_v1;

typedef struct thinkthen_complete_extensions_v1 {
  const struct thinkthen_complete_extension_v1 *data;
  size_t len;
} thinkthen_complete_extensions_v1;

typedef struct thinkthen_complete_answer_yes_no_v1 {
  struct thinkthen_complete_utf8_v1 kind;
  double probability;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_answer_yes_no_v1;

typedef struct thinkthen_complete_answer_choice_field_confidence_presence_v1 {
  uint32_t presence;
  double value;
} thinkthen_complete_answer_choice_field_confidence_presence_v1;

typedef struct thinkthen_complete_answer_choice_field_probabilities_entry_v1 {
  struct thinkthen_complete_utf8_v1 name;
  double value;
} thinkthen_complete_answer_choice_field_probabilities_entry_v1;

typedef struct thinkthen_complete_answer_choice_field_probabilities_v1 {
  const struct thinkthen_complete_answer_choice_field_probabilities_entry_v1 *data;
  size_t len;
} thinkthen_complete_answer_choice_field_probabilities_v1;

typedef struct thinkthen_complete_answer_choice_v1 {
  struct thinkthen_complete_answer_choice_field_confidence_presence_v1 confidence;
  struct thinkthen_complete_utf8_v1 kind;
  struct thinkthen_complete_utf8_v1 pick;
  struct thinkthen_complete_answer_choice_field_probabilities_v1 probabilities;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_answer_choice_v1;

typedef struct thinkthen_complete_answer_tag_field_probabilities_entry_v1 {
  struct thinkthen_complete_utf8_v1 name;
  double value;
} thinkthen_complete_answer_tag_field_probabilities_entry_v1;

typedef struct thinkthen_complete_answer_tag_field_probabilities_v1 {
  const struct thinkthen_complete_answer_tag_field_probabilities_entry_v1 *data;
  size_t len;
} thinkthen_complete_answer_tag_field_probabilities_v1;

typedef struct thinkthen_complete_answer_tag_v1 {
  struct thinkthen_complete_utf8_v1 kind;
  struct thinkthen_complete_answer_tag_field_probabilities_v1 probabilities;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_answer_tag_v1;

typedef struct thinkthen_complete_answer_score_field_confidence_presence_v1 {
  uint32_t presence;
  double value;
} thinkthen_complete_answer_score_field_confidence_presence_v1;

typedef struct thinkthen_complete_answer_score_field_probabilities_entry_v1 {
  struct thinkthen_complete_utf8_v1 name;
  double value;
} thinkthen_complete_answer_score_field_probabilities_entry_v1;

typedef struct thinkthen_complete_answer_score_field_probabilities_v1 {
  const struct thinkthen_complete_answer_score_field_probabilities_entry_v1 *data;
  size_t len;
} thinkthen_complete_answer_score_field_probabilities_v1;

typedef struct thinkthen_complete_answer_score_v1 {
  struct thinkthen_complete_answer_score_field_confidence_presence_v1 confidence;
  struct thinkthen_complete_utf8_v1 kind;
  struct thinkthen_complete_utf8_v1 level;
  struct thinkthen_complete_answer_score_field_probabilities_v1 probabilities;
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

typedef struct thinkthen_complete_atomic_decide_value_field_images_v1 {
  const struct thinkthen_complete_image_v1 *const *data;
  size_t len;
} thinkthen_complete_atomic_decide_value_field_images_v1;

typedef struct thinkthen_complete_atomic_decide_value_field_images_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_atomic_decide_value_field_images_v1 value;
} thinkthen_complete_atomic_decide_value_field_images_presence_v1;

typedef struct thinkthen_complete_atomic_decide_value_field_index_presence_v1 {
  uint32_t presence;
  uint64_t value;
} thinkthen_complete_atomic_decide_value_field_index_presence_v1;

typedef struct thinkthen_complete_json_array_v1 {
  const struct thinkthen_complete_json_v1 *const *data;
  size_t len;
} thinkthen_complete_json_array_v1;

typedef struct thinkthen_complete_json_entry_v1 {
  struct thinkthen_complete_utf8_v1 name;
  const struct thinkthen_complete_json_v1 *value;
} thinkthen_complete_json_entry_v1;

typedef struct thinkthen_complete_json_object_v1 {
  const struct thinkthen_complete_json_entry_v1 *data;
  size_t len;
} thinkthen_complete_json_object_v1;

typedef union thinkthen_complete_json_data_v1 {
  uint32_t boolean;
  struct thinkthen_complete_utf8_v1 number;
  struct thinkthen_complete_utf8_v1 string;
  struct thinkthen_complete_json_array_v1 array;
  struct thinkthen_complete_json_object_v1 object;
} thinkthen_complete_json_data_v1;

typedef struct thinkthen_complete_json_v1 {
  uint32_t kind;
  union thinkthen_complete_json_data_v1 data;
} thinkthen_complete_json_v1;

typedef struct thinkthen_complete_atomic_decide_value_field_input_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_json_v1 *value;
} thinkthen_complete_atomic_decide_value_field_input_presence_v1;

typedef struct thinkthen_complete_rank_member_result_field_images_v1 {
  const struct thinkthen_complete_image_v1 *const *data;
  size_t len;
} thinkthen_complete_rank_member_result_field_images_v1;

typedef struct thinkthen_complete_rank_member_result_field_images_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_rank_member_result_field_images_v1 value;
} thinkthen_complete_rank_member_result_field_images_presence_v1;

typedef struct thinkthen_complete_meta_field_answered_by_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_utf8_v1 value;
} thinkthen_complete_meta_field_answered_by_presence_v1;

typedef struct thinkthen_complete_attempt_outcome_v1 {
  uint32_t kind;
} thinkthen_complete_attempt_outcome_v1;

typedef struct thinkthen_complete_attempt_field_request_id_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_utf8_v1 value;
} thinkthen_complete_attempt_field_request_id_presence_v1;

typedef struct thinkthen_complete_sdk_request_id_v1 {
  struct thinkthen_complete_utf8_v1 value;
} thinkthen_complete_sdk_request_id_v1;

typedef struct thinkthen_complete_attempt_field_server_ms_presence_v1 {
  uint32_t presence;
  uint64_t value;
} thinkthen_complete_attempt_field_server_ms_presence_v1;

typedef struct thinkthen_complete_attempt_field_status_presence_v1 {
  uint32_t presence;
  uint64_t value;
} thinkthen_complete_attempt_field_status_presence_v1;

typedef struct thinkthen_complete_attempt_v1 {
  uint64_t ordinal;
  const struct thinkthen_complete_attempt_outcome_v1 *outcome;
  struct thinkthen_complete_attempt_field_request_id_presence_v1 request_id;
  struct thinkthen_complete_utf8_v1 request_sha256;
  const struct thinkthen_complete_sdk_request_id_v1 *sdk_request_id;
  struct thinkthen_complete_attempt_field_server_ms_presence_v1 server_ms;
  struct thinkthen_complete_attempt_field_status_presence_v1 status;
  uint64_t wall_ms;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_attempt_v1;

typedef struct thinkthen_complete_meta_field_attempts_v1 {
  const struct thinkthen_complete_attempt_v1 *const *data;
  size_t len;
} thinkthen_complete_meta_field_attempts_v1;

typedef struct thinkthen_complete_meta_field_attempts_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_meta_field_attempts_v1 value;
} thinkthen_complete_meta_field_attempts_presence_v1;

typedef union thinkthen_complete_batch_setting_data_v1 {
  uint64_t integer;
  struct thinkthen_complete_utf8_v1 string;
} thinkthen_complete_batch_setting_data_v1;

typedef struct thinkthen_complete_batch_setting_v1 {
  uint32_t kind;
  union thinkthen_complete_batch_setting_data_v1 data;
} thinkthen_complete_batch_setting_v1;

typedef struct thinkthen_complete_meta_field_batch_setting_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_batch_setting_v1 *value;
} thinkthen_complete_meta_field_batch_setting_presence_v1;

typedef struct thinkthen_complete_batch_warning_v1 {
  const struct thinkthen_complete_batch_setting_v1 *running;
  const struct thinkthen_complete_batch_setting_v1 *tuned_for;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_batch_warning_v1;

typedef struct thinkthen_complete_meta_field_batch_warning_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_batch_warning_v1 *value;
} thinkthen_complete_meta_field_batch_warning_presence_v1;

typedef struct thinkthen_complete_meta_field_context_sha_256_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_utf8_v1 value;
} thinkthen_complete_meta_field_context_sha_256_presence_v1;

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

typedef struct thinkthen_complete_meta_field_observations_v1 {
  const struct thinkthen_complete_observation_v1 *const *data;
  size_t len;
} thinkthen_complete_meta_field_observations_v1;

typedef struct thinkthen_complete_origin_v1 {
  uint32_t kind;
} thinkthen_complete_origin_v1;

typedef struct thinkthen_complete_meta_field_origin_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_origin_v1 *value;
} thinkthen_complete_meta_field_origin_presence_v1;

typedef struct thinkthen_complete_profile_warning_v1 {
  struct thinkthen_complete_utf8_v1 running;
  struct thinkthen_complete_utf8_v1 tuned_for;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_profile_warning_v1;

typedef struct thinkthen_complete_meta_field_profile_warning_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_profile_warning_v1 *value;
} thinkthen_complete_meta_field_profile_warning_presence_v1;

typedef struct thinkthen_complete_meta_field_question_sha_256_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_utf8_v1 value;
} thinkthen_complete_meta_field_question_sha_256_presence_v1;

typedef struct thinkthen_complete_question_source_field_batch_size_presence_v1 {
  uint32_t presence;
  uint64_t value;
} thinkthen_complete_question_source_field_batch_size_presence_v1;

typedef struct thinkthen_complete_question_source_v1 {
  struct thinkthen_complete_utf8_v1 answered_by;
  struct thinkthen_complete_question_source_field_batch_size_presence_v1 batch_size;
  const struct thinkthen_complete_origin_v1 *origin;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_question_source_v1;

typedef struct thinkthen_complete_meta_field_question_sources_v1 {
  const struct thinkthen_complete_question_source_v1 *const *data;
  size_t len;
} thinkthen_complete_meta_field_question_sources_v1;

typedef struct thinkthen_complete_meta_field_questions_sha_256_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_utf8_v1 value;
} thinkthen_complete_meta_field_questions_sha_256_presence_v1;

typedef struct thinkthen_complete_meta_field_requests_v1 {
  const struct thinkthen_complete_utf8_v1 *data;
  size_t len;
} thinkthen_complete_meta_field_requests_v1;

typedef struct thinkthen_complete_usage_field_input_tokens_presence_v1 {
  uint32_t presence;
  uint64_t value;
} thinkthen_complete_usage_field_input_tokens_presence_v1;

typedef struct thinkthen_complete_usage_field_output_tokens_presence_v1 {
  uint32_t presence;
  uint64_t value;
} thinkthen_complete_usage_field_output_tokens_presence_v1;

typedef struct thinkthen_complete_usage_v1 {
  struct thinkthen_complete_usage_field_input_tokens_presence_v1 input_tokens;
  struct thinkthen_complete_usage_field_output_tokens_presence_v1 output_tokens;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_usage_v1;

typedef struct thinkthen_complete_meta_field_usage_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_usage_v1 *value;
} thinkthen_complete_meta_field_usage_presence_v1;

typedef struct thinkthen_complete_meta_v1 {
  struct thinkthen_complete_meta_field_answered_by_presence_v1 answered_by;
  struct thinkthen_complete_meta_field_attempts_presence_v1 attempts;
  struct thinkthen_complete_meta_field_batch_setting_presence_v1 batch_setting;
  struct thinkthen_complete_meta_field_batch_warning_presence_v1 batch_warning;
  uint32_t cached;
  struct thinkthen_complete_meta_field_context_sha_256_presence_v1 context_sha256;
  uint64_t failed_questions;
  struct thinkthen_complete_utf8_v1 model;
  struct thinkthen_complete_meta_field_observations_v1 observations;
  struct thinkthen_complete_meta_field_origin_presence_v1 origin;
  struct thinkthen_complete_meta_field_profile_warning_presence_v1 profile_warning;
  struct thinkthen_complete_meta_field_question_sha_256_presence_v1 question_sha256;
  struct thinkthen_complete_meta_field_question_sources_v1 question_sources;
  struct thinkthen_complete_meta_field_questions_sha_256_presence_v1 questions_sha256;
  struct thinkthen_complete_meta_field_requests_v1 requests;
  uint64_t requests_sent;
  struct thinkthen_complete_utf8_v1 tool;
  struct thinkthen_complete_utf8_v1 url;
  struct thinkthen_complete_meta_field_usage_presence_v1 usage;
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

typedef struct thinkthen_complete_readable_question_decide_field_batch_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_batch_v1 *value;
} thinkthen_complete_readable_question_decide_field_batch_presence_v1;

typedef struct thinkthen_complete_string_type_v1 {
  uint32_t kind;
} thinkthen_complete_string_type_v1;

typedef struct thinkthen_complete_input_declaration_string_v1 {
  const struct thinkthen_complete_string_type_v1 *type;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_input_declaration_string_v1;

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

typedef struct thinkthen_complete_input_declaration_object_field_properties_entry_v1 {
  struct thinkthen_complete_utf8_v1 name;
  const struct thinkthen_complete_input_property_type_v1 *value;
} thinkthen_complete_input_declaration_object_field_properties_entry_v1;

typedef struct thinkthen_complete_input_declaration_object_field_properties_v1 {
  const struct thinkthen_complete_input_declaration_object_field_properties_entry_v1 *data;
  size_t len;
} thinkthen_complete_input_declaration_object_field_properties_v1;

typedef struct thinkthen_complete_input_declaration_object_field_required_v1 {
  const struct thinkthen_complete_utf8_v1 *data;
  size_t len;
} thinkthen_complete_input_declaration_object_field_required_v1;

typedef struct thinkthen_complete_input_declaration_object_field_required_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_input_declaration_object_field_required_v1 value;
} thinkthen_complete_input_declaration_object_field_required_presence_v1;

typedef struct thinkthen_complete_object_type_v1 {
  uint32_t kind;
} thinkthen_complete_object_type_v1;

typedef struct thinkthen_complete_input_declaration_object_v1 {
  struct thinkthen_complete_input_declaration_object_field_properties_v1 properties;
  struct thinkthen_complete_input_declaration_object_field_required_presence_v1 required;
  const struct thinkthen_complete_object_type_v1 *type;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_input_declaration_object_v1;

typedef union thinkthen_complete_input_declaration_data_v1 {
  const struct thinkthen_complete_input_declaration_string_v1 *string;
  const struct thinkthen_complete_input_declaration_object_v1 *object;
} thinkthen_complete_input_declaration_data_v1;

typedef struct thinkthen_complete_input_declaration_v1 {
  uint32_t kind;
  union thinkthen_complete_input_declaration_data_v1 data;
} thinkthen_complete_input_declaration_v1;

typedef struct thinkthen_complete_readable_question_decide_field_context_schema_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_input_declaration_v1 *value;
} thinkthen_complete_readable_question_decide_field_context_schema_presence_v1;

typedef struct thinkthen_complete_readable_question_decide_field_item_schema_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_input_declaration_v1 *value;
} thinkthen_complete_readable_question_decide_field_item_schema_presence_v1;

typedef struct thinkthen_complete_label_field_description_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_json_v1 *value;
} thinkthen_complete_label_field_description_presence_v1;

typedef struct thinkthen_complete_label_v1 {
  struct thinkthen_complete_label_field_description_presence_v1 description;
  struct thinkthen_complete_utf8_v1 name;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_label_v1;

typedef struct thinkthen_complete_readable_question_decide_field_label_details_v1 {
  const struct thinkthen_complete_label_v1 *const *data;
  size_t len;
} thinkthen_complete_readable_question_decide_field_label_details_v1;

typedef struct thinkthen_complete_readable_question_decide_field_label_details_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_readable_question_decide_field_label_details_v1 value;
} thinkthen_complete_readable_question_decide_field_label_details_presence_v1;

typedef struct thinkthen_complete_readable_question_decide_field_model_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_utf8_v1 value;
} thinkthen_complete_readable_question_decide_field_model_presence_v1;

typedef struct thinkthen_complete_question_name_v1 {
  struct thinkthen_complete_utf8_v1 value;
} thinkthen_complete_question_name_v1;

typedef struct thinkthen_complete_readable_question_decide_field_name_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_question_name_v1 *value;
} thinkthen_complete_readable_question_decide_field_name_presence_v1;

typedef struct thinkthen_complete_readable_question_decide_field_on_v1 {
  const struct thinkthen_complete_utf8_v1 *data;
  size_t len;
} thinkthen_complete_readable_question_decide_field_on_v1;

typedef struct thinkthen_complete_readable_question_decide_field_on_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_readable_question_decide_field_on_v1 value;
} thinkthen_complete_readable_question_decide_field_on_presence_v1;

typedef struct thinkthen_complete_readable_question_decide_field_profile_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_utf8_v1 value;
} thinkthen_complete_readable_question_decide_field_profile_presence_v1;

typedef struct thinkthen_complete_wording_version_v1 {
  uint64_t value;
} thinkthen_complete_wording_version_v1;

typedef struct thinkthen_complete_readable_question_decide_field_wording_version_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_wording_version_v1 *value;
} thinkthen_complete_readable_question_decide_field_wording_version_presence_v1;

typedef struct thinkthen_complete_readable_question_decide_field_false_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_json_v1 *value;
} thinkthen_complete_readable_question_decide_field_false_presence_v1;

typedef struct thinkthen_complete_readable_question_decide_field_text_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_json_v1 *value;
} thinkthen_complete_readable_question_decide_field_text_presence_v1;

typedef struct thinkthen_complete_readable_question_decide_field_true_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_json_v1 *value;
} thinkthen_complete_readable_question_decide_field_true_presence_v1;

typedef struct thinkthen_complete_readable_question_decide_v1 {
  struct thinkthen_complete_readable_question_decide_field_batch_presence_v1 batch;
  struct thinkthen_complete_readable_question_decide_field_context_schema_presence_v1 context_schema;
  struct thinkthen_complete_readable_question_decide_field_item_schema_presence_v1 item_schema;
  struct thinkthen_complete_readable_question_decide_field_label_details_presence_v1 label_details;
  struct thinkthen_complete_readable_question_decide_field_model_presence_v1 model;
  struct thinkthen_complete_readable_question_decide_field_name_presence_v1 name;
  struct thinkthen_complete_readable_question_decide_field_on_presence_v1 on;
  struct thinkthen_complete_readable_question_decide_field_profile_presence_v1 profile;
  struct thinkthen_complete_readable_question_decide_field_wording_version_presence_v1 wording_version;
  struct thinkthen_complete_readable_question_decide_field_false_presence_v1 false_;
  struct thinkthen_complete_readable_question_decide_field_text_presence_v1 text;
  struct thinkthen_complete_readable_question_decide_field_true_presence_v1 true_;
  struct thinkthen_complete_utf8_v1 verb;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_readable_question_decide_v1;

typedef struct thinkthen_complete_readable_question_choose_field_batch_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_batch_v1 *value;
} thinkthen_complete_readable_question_choose_field_batch_presence_v1;

typedef struct thinkthen_complete_readable_question_choose_field_context_schema_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_input_declaration_v1 *value;
} thinkthen_complete_readable_question_choose_field_context_schema_presence_v1;

typedef struct thinkthen_complete_readable_question_choose_field_item_schema_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_input_declaration_v1 *value;
} thinkthen_complete_readable_question_choose_field_item_schema_presence_v1;

typedef struct thinkthen_complete_readable_question_choose_field_label_details_v1 {
  const struct thinkthen_complete_label_v1 *const *data;
  size_t len;
} thinkthen_complete_readable_question_choose_field_label_details_v1;

typedef struct thinkthen_complete_readable_question_choose_field_label_details_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_readable_question_choose_field_label_details_v1 value;
} thinkthen_complete_readable_question_choose_field_label_details_presence_v1;

typedef struct thinkthen_complete_readable_question_choose_field_model_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_utf8_v1 value;
} thinkthen_complete_readable_question_choose_field_model_presence_v1;

typedef struct thinkthen_complete_readable_question_choose_field_name_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_question_name_v1 *value;
} thinkthen_complete_readable_question_choose_field_name_presence_v1;

typedef struct thinkthen_complete_readable_question_choose_field_on_v1 {
  const struct thinkthen_complete_utf8_v1 *data;
  size_t len;
} thinkthen_complete_readable_question_choose_field_on_v1;

typedef struct thinkthen_complete_readable_question_choose_field_on_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_readable_question_choose_field_on_v1 value;
} thinkthen_complete_readable_question_choose_field_on_presence_v1;

typedef struct thinkthen_complete_readable_question_choose_field_profile_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_utf8_v1 value;
} thinkthen_complete_readable_question_choose_field_profile_presence_v1;

typedef struct thinkthen_complete_readable_question_choose_field_wording_version_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_wording_version_v1 *value;
} thinkthen_complete_readable_question_choose_field_wording_version_presence_v1;

typedef struct thinkthen_complete_readable_question_choose_field_options_v1 {
  const struct thinkthen_complete_utf8_v1 *data;
  size_t len;
} thinkthen_complete_readable_question_choose_field_options_v1;

typedef struct thinkthen_complete_readable_question_choose_field_text_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_json_v1 *value;
} thinkthen_complete_readable_question_choose_field_text_presence_v1;

typedef struct thinkthen_complete_readable_question_choose_v1 {
  struct thinkthen_complete_readable_question_choose_field_batch_presence_v1 batch;
  struct thinkthen_complete_readable_question_choose_field_context_schema_presence_v1 context_schema;
  struct thinkthen_complete_readable_question_choose_field_item_schema_presence_v1 item_schema;
  struct thinkthen_complete_readable_question_choose_field_label_details_presence_v1 label_details;
  struct thinkthen_complete_readable_question_choose_field_model_presence_v1 model;
  struct thinkthen_complete_readable_question_choose_field_name_presence_v1 name;
  struct thinkthen_complete_readable_question_choose_field_on_presence_v1 on;
  struct thinkthen_complete_readable_question_choose_field_profile_presence_v1 profile;
  struct thinkthen_complete_readable_question_choose_field_wording_version_presence_v1 wording_version;
  struct thinkthen_complete_readable_question_choose_field_options_v1 options;
  struct thinkthen_complete_readable_question_choose_field_text_presence_v1 text;
  struct thinkthen_complete_utf8_v1 verb;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_readable_question_choose_v1;

typedef struct thinkthen_complete_readable_question_tag_field_batch_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_batch_v1 *value;
} thinkthen_complete_readable_question_tag_field_batch_presence_v1;

typedef struct thinkthen_complete_readable_question_tag_field_context_schema_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_input_declaration_v1 *value;
} thinkthen_complete_readable_question_tag_field_context_schema_presence_v1;

typedef struct thinkthen_complete_readable_question_tag_field_item_schema_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_input_declaration_v1 *value;
} thinkthen_complete_readable_question_tag_field_item_schema_presence_v1;

typedef struct thinkthen_complete_readable_question_tag_field_label_details_v1 {
  const struct thinkthen_complete_label_v1 *const *data;
  size_t len;
} thinkthen_complete_readable_question_tag_field_label_details_v1;

typedef struct thinkthen_complete_readable_question_tag_field_label_details_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_readable_question_tag_field_label_details_v1 value;
} thinkthen_complete_readable_question_tag_field_label_details_presence_v1;

typedef struct thinkthen_complete_readable_question_tag_field_model_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_utf8_v1 value;
} thinkthen_complete_readable_question_tag_field_model_presence_v1;

typedef struct thinkthen_complete_readable_question_tag_field_name_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_question_name_v1 *value;
} thinkthen_complete_readable_question_tag_field_name_presence_v1;

typedef struct thinkthen_complete_readable_question_tag_field_on_v1 {
  const struct thinkthen_complete_utf8_v1 *data;
  size_t len;
} thinkthen_complete_readable_question_tag_field_on_v1;

typedef struct thinkthen_complete_readable_question_tag_field_on_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_readable_question_tag_field_on_v1 value;
} thinkthen_complete_readable_question_tag_field_on_presence_v1;

typedef struct thinkthen_complete_readable_question_tag_field_profile_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_utf8_v1 value;
} thinkthen_complete_readable_question_tag_field_profile_presence_v1;

typedef struct thinkthen_complete_readable_question_tag_field_wording_version_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_wording_version_v1 *value;
} thinkthen_complete_readable_question_tag_field_wording_version_presence_v1;

typedef struct thinkthen_complete_readable_question_tag_field_labels_v1 {
  const struct thinkthen_complete_utf8_v1 *data;
  size_t len;
} thinkthen_complete_readable_question_tag_field_labels_v1;

typedef struct thinkthen_complete_readable_question_tag_field_text_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_json_v1 *value;
} thinkthen_complete_readable_question_tag_field_text_presence_v1;

typedef struct thinkthen_complete_readable_question_tag_v1 {
  struct thinkthen_complete_readable_question_tag_field_batch_presence_v1 batch;
  struct thinkthen_complete_readable_question_tag_field_context_schema_presence_v1 context_schema;
  struct thinkthen_complete_readable_question_tag_field_item_schema_presence_v1 item_schema;
  struct thinkthen_complete_readable_question_tag_field_label_details_presence_v1 label_details;
  struct thinkthen_complete_readable_question_tag_field_model_presence_v1 model;
  struct thinkthen_complete_readable_question_tag_field_name_presence_v1 name;
  struct thinkthen_complete_readable_question_tag_field_on_presence_v1 on;
  struct thinkthen_complete_readable_question_tag_field_profile_presence_v1 profile;
  struct thinkthen_complete_readable_question_tag_field_wording_version_presence_v1 wording_version;
  struct thinkthen_complete_readable_question_tag_field_labels_v1 labels;
  struct thinkthen_complete_readable_question_tag_field_text_presence_v1 text;
  struct thinkthen_complete_utf8_v1 verb;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_readable_question_tag_v1;

typedef struct thinkthen_complete_readable_question_score_field_batch_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_batch_v1 *value;
} thinkthen_complete_readable_question_score_field_batch_presence_v1;

typedef struct thinkthen_complete_readable_question_score_field_context_schema_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_input_declaration_v1 *value;
} thinkthen_complete_readable_question_score_field_context_schema_presence_v1;

typedef struct thinkthen_complete_readable_question_score_field_item_schema_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_input_declaration_v1 *value;
} thinkthen_complete_readable_question_score_field_item_schema_presence_v1;

typedef struct thinkthen_complete_readable_question_score_field_label_details_v1 {
  const struct thinkthen_complete_label_v1 *const *data;
  size_t len;
} thinkthen_complete_readable_question_score_field_label_details_v1;

typedef struct thinkthen_complete_readable_question_score_field_label_details_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_readable_question_score_field_label_details_v1 value;
} thinkthen_complete_readable_question_score_field_label_details_presence_v1;

typedef struct thinkthen_complete_readable_question_score_field_model_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_utf8_v1 value;
} thinkthen_complete_readable_question_score_field_model_presence_v1;

typedef struct thinkthen_complete_readable_question_score_field_name_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_question_name_v1 *value;
} thinkthen_complete_readable_question_score_field_name_presence_v1;

typedef struct thinkthen_complete_readable_question_score_field_on_v1 {
  const struct thinkthen_complete_utf8_v1 *data;
  size_t len;
} thinkthen_complete_readable_question_score_field_on_v1;

typedef struct thinkthen_complete_readable_question_score_field_on_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_readable_question_score_field_on_v1 value;
} thinkthen_complete_readable_question_score_field_on_presence_v1;

typedef struct thinkthen_complete_readable_question_score_field_profile_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_utf8_v1 value;
} thinkthen_complete_readable_question_score_field_profile_presence_v1;

typedef struct thinkthen_complete_readable_question_score_field_wording_version_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_wording_version_v1 *value;
} thinkthen_complete_readable_question_score_field_wording_version_presence_v1;

typedef struct thinkthen_complete_readable_question_score_field_levels_v1 {
  const struct thinkthen_complete_utf8_v1 *data;
  size_t len;
} thinkthen_complete_readable_question_score_field_levels_v1;

typedef struct thinkthen_complete_readable_question_score_field_text_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_json_v1 *value;
} thinkthen_complete_readable_question_score_field_text_presence_v1;

typedef struct thinkthen_complete_readable_question_score_v1 {
  struct thinkthen_complete_readable_question_score_field_batch_presence_v1 batch;
  struct thinkthen_complete_readable_question_score_field_context_schema_presence_v1 context_schema;
  struct thinkthen_complete_readable_question_score_field_item_schema_presence_v1 item_schema;
  struct thinkthen_complete_readable_question_score_field_label_details_presence_v1 label_details;
  struct thinkthen_complete_readable_question_score_field_model_presence_v1 model;
  struct thinkthen_complete_readable_question_score_field_name_presence_v1 name;
  struct thinkthen_complete_readable_question_score_field_on_presence_v1 on;
  struct thinkthen_complete_readable_question_score_field_profile_presence_v1 profile;
  struct thinkthen_complete_readable_question_score_field_wording_version_presence_v1 wording_version;
  struct thinkthen_complete_readable_question_score_field_levels_v1 levels;
  struct thinkthen_complete_readable_question_score_field_text_presence_v1 text;
  struct thinkthen_complete_utf8_v1 verb;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_readable_question_score_v1;

typedef union thinkthen_complete_readable_question_data_v1 {
  const struct thinkthen_complete_readable_question_decide_v1 *decide;
  const struct thinkthen_complete_readable_question_choose_v1 *choose;
  const struct thinkthen_complete_readable_question_tag_v1 *tag;
  const struct thinkthen_complete_readable_question_score_v1 *score;
} thinkthen_complete_readable_question_data_v1;

typedef struct thinkthen_complete_readable_question_v1 {
  uint32_t kind;
  union thinkthen_complete_readable_question_data_v1 data;
} thinkthen_complete_readable_question_v1;

typedef struct thinkthen_complete_version_v1 {
  uint32_t kind;
} thinkthen_complete_version_v1;

typedef struct thinkthen_complete_physical_source_field_first_line_presence_v1 {
  uint32_t presence;
  uint64_t value;
} thinkthen_complete_physical_source_field_first_line_presence_v1;

typedef struct thinkthen_complete_physical_source_field_last_line_presence_v1 {
  uint32_t presence;
  uint64_t value;
} thinkthen_complete_physical_source_field_last_line_presence_v1;

typedef struct thinkthen_complete_physical_source_v1 {
  struct thinkthen_complete_utf8_v1 file;
  struct thinkthen_complete_physical_source_field_first_line_presence_v1 first_line;
  struct thinkthen_complete_physical_source_field_last_line_presence_v1 last_line;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_physical_source_v1;

typedef struct thinkthen_complete_rank_member_result_field_source_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_physical_source_v1 *value;
} thinkthen_complete_rank_member_result_field_source_presence_v1;

typedef struct thinkthen_complete_rank_member_result_field_threshold_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_json_v1 *value;
} thinkthen_complete_rank_member_result_field_threshold_presence_v1;

typedef struct thinkthen_complete_rank_member_result_v1 {
  const struct thinkthen_complete_answer_v1 *answer;
  const struct thinkthen_complete_answer_id_v1 *answer_id;
  struct thinkthen_complete_rank_member_result_field_images_presence_v1 images;
  const struct thinkthen_complete_meta_v1 *meta;
  const struct thinkthen_complete_readable_question_v1 *question;
  const struct thinkthen_complete_version_v1 *schema;
  struct thinkthen_complete_rank_member_result_field_source_presence_v1 source;
  struct thinkthen_complete_rank_member_result_field_threshold_presence_v1 threshold;
  uint64_t value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_rank_member_result_v1;

typedef struct thinkthen_complete_rank_member_v1 {
  struct thinkthen_complete_utf8_v1 name;
  const struct thinkthen_complete_rank_member_result_v1 *result;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_rank_member_v1;

typedef struct thinkthen_complete_atomic_decide_value_field_members_v1 {
  const struct thinkthen_complete_rank_member_v1 *const *data;
  size_t len;
} thinkthen_complete_atomic_decide_value_field_members_v1;

typedef struct thinkthen_complete_atomic_decide_value_field_members_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_atomic_decide_value_field_members_v1 value;
} thinkthen_complete_atomic_decide_value_field_members_presence_v1;

typedef struct thinkthen_complete_atomic_decide_value_field_question_name_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_utf8_v1 value;
} thinkthen_complete_atomic_decide_value_field_question_name_presence_v1;

typedef struct thinkthen_complete_atomic_decide_value_field_source_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_physical_source_v1 *value;
} thinkthen_complete_atomic_decide_value_field_source_presence_v1;

typedef union thinkthen_complete_threshold_data_v1 {
  double number;
  struct thinkthen_complete_utf8_v1 string;
} thinkthen_complete_threshold_data_v1;

typedef struct thinkthen_complete_threshold_v1 {
  uint32_t kind;
  union thinkthen_complete_threshold_data_v1 data;
} thinkthen_complete_threshold_v1;

typedef struct thinkthen_complete_atomic_decide_value_field_threshold_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_threshold_v1 *value;
} thinkthen_complete_atomic_decide_value_field_threshold_presence_v1;

typedef struct thinkthen_complete_decide_value_v1 {
  const struct thinkthen_complete_json_v1 *value;
} thinkthen_complete_decide_value_v1;

typedef struct thinkthen_complete_atomic_decide_value_field_value_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_decide_value_v1 *value;
} thinkthen_complete_atomic_decide_value_field_value_presence_v1;

typedef struct thinkthen_complete_atomic_decide_value_v1 {
  const struct thinkthen_complete_answer_v1 *answer;
  const struct thinkthen_complete_answer_id_v1 *answer_id;
  struct thinkthen_complete_atomic_decide_value_field_images_presence_v1 images;
  struct thinkthen_complete_atomic_decide_value_field_index_presence_v1 index;
  struct thinkthen_complete_atomic_decide_value_field_input_presence_v1 input;
  struct thinkthen_complete_atomic_decide_value_field_members_presence_v1 members;
  const struct thinkthen_complete_meta_v1 *meta;
  const struct thinkthen_complete_readable_question_v1 *question;
  struct thinkthen_complete_atomic_decide_value_field_question_name_presence_v1 question_name;
  const struct thinkthen_complete_version_v1 *schema;
  struct thinkthen_complete_atomic_decide_value_field_source_presence_v1 source;
  struct thinkthen_complete_atomic_decide_value_field_threshold_presence_v1 threshold;
  struct thinkthen_complete_atomic_decide_value_field_value_presence_v1 value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_atomic_decide_value_v1;

typedef struct thinkthen_complete_session_packet_decide_row_v1 {
  struct thinkthen_complete_utf8_v1 function;
  struct thinkthen_complete_utf8_v1 kind;
  const struct thinkthen_complete_atomic_decide_value_v1 *value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_session_packet_decide_row_v1;

typedef struct thinkthen_complete_atomic_nullable_string_field_images_v1 {
  const struct thinkthen_complete_image_v1 *const *data;
  size_t len;
} thinkthen_complete_atomic_nullable_string_field_images_v1;

typedef struct thinkthen_complete_atomic_nullable_string_field_images_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_atomic_nullable_string_field_images_v1 value;
} thinkthen_complete_atomic_nullable_string_field_images_presence_v1;

typedef struct thinkthen_complete_atomic_nullable_string_field_index_presence_v1 {
  uint32_t presence;
  uint64_t value;
} thinkthen_complete_atomic_nullable_string_field_index_presence_v1;

typedef struct thinkthen_complete_atomic_nullable_string_field_input_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_json_v1 *value;
} thinkthen_complete_atomic_nullable_string_field_input_presence_v1;

typedef struct thinkthen_complete_atomic_nullable_string_field_members_v1 {
  const struct thinkthen_complete_rank_member_v1 *const *data;
  size_t len;
} thinkthen_complete_atomic_nullable_string_field_members_v1;

typedef struct thinkthen_complete_atomic_nullable_string_field_members_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_atomic_nullable_string_field_members_v1 value;
} thinkthen_complete_atomic_nullable_string_field_members_presence_v1;

typedef struct thinkthen_complete_atomic_nullable_string_field_question_name_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_utf8_v1 value;
} thinkthen_complete_atomic_nullable_string_field_question_name_presence_v1;

typedef struct thinkthen_complete_atomic_nullable_string_field_source_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_physical_source_v1 *value;
} thinkthen_complete_atomic_nullable_string_field_source_presence_v1;

typedef struct thinkthen_complete_atomic_nullable_string_field_threshold_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_threshold_v1 *value;
} thinkthen_complete_atomic_nullable_string_field_threshold_presence_v1;

typedef struct thinkthen_complete_atomic_nullable_string_field_value_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_utf8_v1 value;
} thinkthen_complete_atomic_nullable_string_field_value_presence_v1;

typedef struct thinkthen_complete_atomic_nullable_string_v1 {
  const struct thinkthen_complete_answer_v1 *answer;
  const struct thinkthen_complete_answer_id_v1 *answer_id;
  struct thinkthen_complete_atomic_nullable_string_field_images_presence_v1 images;
  struct thinkthen_complete_atomic_nullable_string_field_index_presence_v1 index;
  struct thinkthen_complete_atomic_nullable_string_field_input_presence_v1 input;
  struct thinkthen_complete_atomic_nullable_string_field_members_presence_v1 members;
  const struct thinkthen_complete_meta_v1 *meta;
  const struct thinkthen_complete_readable_question_v1 *question;
  struct thinkthen_complete_atomic_nullable_string_field_question_name_presence_v1 question_name;
  const struct thinkthen_complete_version_v1 *schema;
  struct thinkthen_complete_atomic_nullable_string_field_source_presence_v1 source;
  struct thinkthen_complete_atomic_nullable_string_field_threshold_presence_v1 threshold;
  struct thinkthen_complete_atomic_nullable_string_field_value_presence_v1 value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_atomic_nullable_string_v1;

typedef struct thinkthen_complete_session_packet_choose_row_v1 {
  struct thinkthen_complete_utf8_v1 function;
  struct thinkthen_complete_utf8_v1 kind;
  const struct thinkthen_complete_atomic_nullable_string_v1 *value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_session_packet_choose_row_v1;

typedef struct thinkthen_complete_atomic_array_of_string_field_images_v1 {
  const struct thinkthen_complete_image_v1 *const *data;
  size_t len;
} thinkthen_complete_atomic_array_of_string_field_images_v1;

typedef struct thinkthen_complete_atomic_array_of_string_field_images_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_atomic_array_of_string_field_images_v1 value;
} thinkthen_complete_atomic_array_of_string_field_images_presence_v1;

typedef struct thinkthen_complete_atomic_array_of_string_field_index_presence_v1 {
  uint32_t presence;
  uint64_t value;
} thinkthen_complete_atomic_array_of_string_field_index_presence_v1;

typedef struct thinkthen_complete_atomic_array_of_string_field_input_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_json_v1 *value;
} thinkthen_complete_atomic_array_of_string_field_input_presence_v1;

typedef struct thinkthen_complete_atomic_array_of_string_field_members_v1 {
  const struct thinkthen_complete_rank_member_v1 *const *data;
  size_t len;
} thinkthen_complete_atomic_array_of_string_field_members_v1;

typedef struct thinkthen_complete_atomic_array_of_string_field_members_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_atomic_array_of_string_field_members_v1 value;
} thinkthen_complete_atomic_array_of_string_field_members_presence_v1;

typedef struct thinkthen_complete_atomic_array_of_string_field_question_name_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_utf8_v1 value;
} thinkthen_complete_atomic_array_of_string_field_question_name_presence_v1;

typedef struct thinkthen_complete_atomic_array_of_string_field_source_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_physical_source_v1 *value;
} thinkthen_complete_atomic_array_of_string_field_source_presence_v1;

typedef struct thinkthen_complete_atomic_array_of_string_field_threshold_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_threshold_v1 *value;
} thinkthen_complete_atomic_array_of_string_field_threshold_presence_v1;

typedef struct thinkthen_complete_atomic_array_of_string_field_value_v1 {
  const struct thinkthen_complete_utf8_v1 *data;
  size_t len;
} thinkthen_complete_atomic_array_of_string_field_value_v1;

typedef struct thinkthen_complete_atomic_array_of_string_v1 {
  const struct thinkthen_complete_answer_v1 *answer;
  const struct thinkthen_complete_answer_id_v1 *answer_id;
  struct thinkthen_complete_atomic_array_of_string_field_images_presence_v1 images;
  struct thinkthen_complete_atomic_array_of_string_field_index_presence_v1 index;
  struct thinkthen_complete_atomic_array_of_string_field_input_presence_v1 input;
  struct thinkthen_complete_atomic_array_of_string_field_members_presence_v1 members;
  const struct thinkthen_complete_meta_v1 *meta;
  const struct thinkthen_complete_readable_question_v1 *question;
  struct thinkthen_complete_atomic_array_of_string_field_question_name_presence_v1 question_name;
  const struct thinkthen_complete_version_v1 *schema;
  struct thinkthen_complete_atomic_array_of_string_field_source_presence_v1 source;
  struct thinkthen_complete_atomic_array_of_string_field_threshold_presence_v1 threshold;
  struct thinkthen_complete_atomic_array_of_string_field_value_v1 value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_atomic_array_of_string_v1;

typedef struct thinkthen_complete_session_packet_tag_row_v1 {
  struct thinkthen_complete_utf8_v1 function;
  struct thinkthen_complete_utf8_v1 kind;
  const struct thinkthen_complete_atomic_array_of_string_v1 *value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_session_packet_tag_row_v1;

typedef struct thinkthen_complete_atomic_double_field_images_v1 {
  const struct thinkthen_complete_image_v1 *const *data;
  size_t len;
} thinkthen_complete_atomic_double_field_images_v1;

typedef struct thinkthen_complete_atomic_double_field_images_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_atomic_double_field_images_v1 value;
} thinkthen_complete_atomic_double_field_images_presence_v1;

typedef struct thinkthen_complete_atomic_double_field_index_presence_v1 {
  uint32_t presence;
  uint64_t value;
} thinkthen_complete_atomic_double_field_index_presence_v1;

typedef struct thinkthen_complete_atomic_double_field_input_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_json_v1 *value;
} thinkthen_complete_atomic_double_field_input_presence_v1;

typedef struct thinkthen_complete_atomic_double_field_members_v1 {
  const struct thinkthen_complete_rank_member_v1 *const *data;
  size_t len;
} thinkthen_complete_atomic_double_field_members_v1;

typedef struct thinkthen_complete_atomic_double_field_members_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_atomic_double_field_members_v1 value;
} thinkthen_complete_atomic_double_field_members_presence_v1;

typedef struct thinkthen_complete_atomic_double_field_question_name_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_utf8_v1 value;
} thinkthen_complete_atomic_double_field_question_name_presence_v1;

typedef struct thinkthen_complete_atomic_double_field_source_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_physical_source_v1 *value;
} thinkthen_complete_atomic_double_field_source_presence_v1;

typedef struct thinkthen_complete_atomic_double_field_threshold_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_threshold_v1 *value;
} thinkthen_complete_atomic_double_field_threshold_presence_v1;

typedef struct thinkthen_complete_atomic_double_v1 {
  const struct thinkthen_complete_answer_v1 *answer;
  const struct thinkthen_complete_answer_id_v1 *answer_id;
  struct thinkthen_complete_atomic_double_field_images_presence_v1 images;
  struct thinkthen_complete_atomic_double_field_index_presence_v1 index;
  struct thinkthen_complete_atomic_double_field_input_presence_v1 input;
  struct thinkthen_complete_atomic_double_field_members_presence_v1 members;
  const struct thinkthen_complete_meta_v1 *meta;
  const struct thinkthen_complete_readable_question_v1 *question;
  struct thinkthen_complete_atomic_double_field_question_name_presence_v1 question_name;
  const struct thinkthen_complete_version_v1 *schema;
  struct thinkthen_complete_atomic_double_field_source_presence_v1 source;
  struct thinkthen_complete_atomic_double_field_threshold_presence_v1 threshold;
  double value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_atomic_double_v1;

typedef struct thinkthen_complete_session_packet_score_row_v1 {
  struct thinkthen_complete_utf8_v1 function;
  struct thinkthen_complete_utf8_v1 kind;
  const struct thinkthen_complete_atomic_double_v1 *value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_session_packet_score_row_v1;

typedef struct thinkthen_complete_atomic_boolean_field_images_v1 {
  const struct thinkthen_complete_image_v1 *const *data;
  size_t len;
} thinkthen_complete_atomic_boolean_field_images_v1;

typedef struct thinkthen_complete_atomic_boolean_field_images_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_atomic_boolean_field_images_v1 value;
} thinkthen_complete_atomic_boolean_field_images_presence_v1;

typedef struct thinkthen_complete_atomic_boolean_field_index_presence_v1 {
  uint32_t presence;
  uint64_t value;
} thinkthen_complete_atomic_boolean_field_index_presence_v1;

typedef struct thinkthen_complete_atomic_boolean_field_input_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_json_v1 *value;
} thinkthen_complete_atomic_boolean_field_input_presence_v1;

typedef struct thinkthen_complete_atomic_boolean_field_members_v1 {
  const struct thinkthen_complete_rank_member_v1 *const *data;
  size_t len;
} thinkthen_complete_atomic_boolean_field_members_v1;

typedef struct thinkthen_complete_atomic_boolean_field_members_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_atomic_boolean_field_members_v1 value;
} thinkthen_complete_atomic_boolean_field_members_presence_v1;

typedef struct thinkthen_complete_atomic_boolean_field_question_name_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_utf8_v1 value;
} thinkthen_complete_atomic_boolean_field_question_name_presence_v1;

typedef struct thinkthen_complete_atomic_boolean_field_source_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_physical_source_v1 *value;
} thinkthen_complete_atomic_boolean_field_source_presence_v1;

typedef struct thinkthen_complete_atomic_boolean_field_threshold_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_threshold_v1 *value;
} thinkthen_complete_atomic_boolean_field_threshold_presence_v1;

typedef struct thinkthen_complete_atomic_boolean_v1 {
  const struct thinkthen_complete_answer_v1 *answer;
  const struct thinkthen_complete_answer_id_v1 *answer_id;
  struct thinkthen_complete_atomic_boolean_field_images_presence_v1 images;
  struct thinkthen_complete_atomic_boolean_field_index_presence_v1 index;
  struct thinkthen_complete_atomic_boolean_field_input_presence_v1 input;
  struct thinkthen_complete_atomic_boolean_field_members_presence_v1 members;
  const struct thinkthen_complete_meta_v1 *meta;
  const struct thinkthen_complete_readable_question_v1 *question;
  struct thinkthen_complete_atomic_boolean_field_question_name_presence_v1 question_name;
  const struct thinkthen_complete_version_v1 *schema;
  struct thinkthen_complete_atomic_boolean_field_source_presence_v1 source;
  struct thinkthen_complete_atomic_boolean_field_threshold_presence_v1 threshold;
  uint32_t value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_atomic_boolean_v1;

typedef struct thinkthen_complete_session_packet_filter_row_v1 {
  struct thinkthen_complete_utf8_v1 function;
  struct thinkthen_complete_utf8_v1 kind;
  const struct thinkthen_complete_atomic_boolean_v1 *value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_session_packet_filter_row_v1;

typedef struct thinkthen_complete_annotation_member_answer_id_field_observations_v1 {
  const struct thinkthen_complete_observation_v1 *const *data;
  size_t len;
} thinkthen_complete_annotation_member_answer_id_field_observations_v1;

typedef struct thinkthen_complete_annotation_member_answer_id_field_question_sources_v1 {
  const struct thinkthen_complete_question_source_v1 *const *data;
  size_t len;
} thinkthen_complete_annotation_member_answer_id_field_question_sources_v1;

typedef struct thinkthen_complete_annotation_member_answer_id_field_threshold_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_threshold_v1 *value;
} thinkthen_complete_annotation_member_answer_id_field_threshold_presence_v1;

typedef struct thinkthen_complete_annotation_member_answer_id_field_usage_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_usage_v1 *value;
} thinkthen_complete_annotation_member_answer_id_field_usage_presence_v1;

typedef struct thinkthen_complete_value_array_v1 {
  const struct thinkthen_complete_utf8_v1 *data;
  size_t len;
} thinkthen_complete_value_array_v1;

typedef union thinkthen_complete_value_data_v1 {
  uint32_t boolean;
  uint32_t null;
  struct thinkthen_complete_utf8_v1 string;
  struct thinkthen_complete_value_array_v1 array;
  double number;
} thinkthen_complete_value_data_v1;

typedef struct thinkthen_complete_value_v1 {
  uint32_t kind;
  union thinkthen_complete_value_data_v1 data;
} thinkthen_complete_value_v1;

typedef struct thinkthen_complete_annotation_member_answer_id_field_value_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_value_v1 *value;
} thinkthen_complete_annotation_member_answer_id_field_value_presence_v1;

typedef struct thinkthen_complete_annotation_member_answer_id_v1 {
  const struct thinkthen_complete_answer_v1 *answer;
  const struct thinkthen_complete_answer_id_v1 *answer_id;
  struct thinkthen_complete_annotation_member_answer_id_field_observations_v1 observations;
  const struct thinkthen_complete_readable_question_v1 *question;
  struct thinkthen_complete_annotation_member_answer_id_field_question_sources_v1 question_sources;
  struct thinkthen_complete_utf8_v1 request;
  struct thinkthen_complete_annotation_member_answer_id_field_threshold_presence_v1 threshold;
  struct thinkthen_complete_annotation_member_answer_id_field_usage_presence_v1 usage;
  struct thinkthen_complete_annotation_member_answer_id_field_value_presence_v1 value;
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

typedef struct thinkthen_complete_annotation_member_failure_id_field_observations_v1 {
  const struct thinkthen_complete_observation_v1 *const *data;
  size_t len;
} thinkthen_complete_annotation_member_failure_id_field_observations_v1;

typedef struct thinkthen_complete_annotation_member_failure_id_field_question_sources_v1 {
  const struct thinkthen_complete_question_source_v1 *const *data;
  size_t len;
} thinkthen_complete_annotation_member_failure_id_field_question_sources_v1;

typedef struct thinkthen_complete_annotation_member_failure_id_field_threshold_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_threshold_v1 *value;
} thinkthen_complete_annotation_member_failure_id_field_threshold_presence_v1;

typedef struct thinkthen_complete_annotation_member_failure_id_field_usage_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_usage_v1 *value;
} thinkthen_complete_annotation_member_failure_id_field_usage_presence_v1;

typedef struct thinkthen_complete_annotation_member_failure_id_v1 {
  const struct thinkthen_complete_failure_v1 *failure;
  const struct thinkthen_complete_failure_id_v1 *failure_id;
  struct thinkthen_complete_annotation_member_failure_id_field_observations_v1 observations;
  const struct thinkthen_complete_readable_question_v1 *question;
  struct thinkthen_complete_annotation_member_failure_id_field_question_sources_v1 question_sources;
  struct thinkthen_complete_utf8_v1 request;
  struct thinkthen_complete_annotation_member_failure_id_field_threshold_presence_v1 threshold;
  struct thinkthen_complete_annotation_member_failure_id_field_usage_presence_v1 usage;
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

typedef struct thinkthen_complete_annotation_field_answers_entry_v1 {
  struct thinkthen_complete_utf8_v1 name;
  const struct thinkthen_complete_annotation_member_v1 *value;
} thinkthen_complete_annotation_field_answers_entry_v1;

typedef struct thinkthen_complete_annotation_field_answers_v1 {
  const struct thinkthen_complete_annotation_field_answers_entry_v1 *data;
  size_t len;
} thinkthen_complete_annotation_field_answers_v1;

typedef struct thinkthen_complete_annotation_field_file_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_utf8_v1 value;
} thinkthen_complete_annotation_field_file_presence_v1;

typedef struct thinkthen_complete_annotation_field_first_line_presence_v1 {
  uint32_t presence;
  uint64_t value;
} thinkthen_complete_annotation_field_first_line_presence_v1;

typedef struct thinkthen_complete_annotation_field_index_presence_v1 {
  uint32_t presence;
  uint64_t value;
} thinkthen_complete_annotation_field_index_presence_v1;

typedef struct thinkthen_complete_annotation_field_input_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_json_v1 *value;
} thinkthen_complete_annotation_field_input_presence_v1;

typedef struct thinkthen_complete_annotation_field_last_line_presence_v1 {
  uint32_t presence;
  uint64_t value;
} thinkthen_complete_annotation_field_last_line_presence_v1;

typedef struct thinkthen_complete_position_field_file_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_utf8_v1 value;
} thinkthen_complete_position_field_file_presence_v1;

typedef struct thinkthen_complete_position_field_first_presence_v1 {
  uint32_t presence;
  uint64_t value;
} thinkthen_complete_position_field_first_presence_v1;

typedef struct thinkthen_complete_position_field_images_v1 {
  const struct thinkthen_complete_utf8_v1 *data;
  size_t len;
} thinkthen_complete_position_field_images_v1;

typedef struct thinkthen_complete_position_field_images_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_position_field_images_v1 value;
} thinkthen_complete_position_field_images_presence_v1;

typedef struct thinkthen_complete_position_field_last_presence_v1 {
  uint32_t presence;
  uint64_t value;
} thinkthen_complete_position_field_last_presence_v1;

typedef struct thinkthen_complete_position_v1 {
  struct thinkthen_complete_position_field_file_presence_v1 file;
  struct thinkthen_complete_position_field_first_presence_v1 first;
  struct thinkthen_complete_position_field_images_presence_v1 images;
  struct thinkthen_complete_position_field_last_presence_v1 last;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_position_v1;

typedef struct thinkthen_complete_annotation_field_position_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_position_v1 *value;
} thinkthen_complete_annotation_field_position_presence_v1;

typedef struct thinkthen_complete_annotation_field_source_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_physical_source_v1 *value;
} thinkthen_complete_annotation_field_source_presence_v1;

typedef struct thinkthen_complete_annotated_field_array_v1 {
  const struct thinkthen_complete_utf8_v1 *data;
  size_t len;
} thinkthen_complete_annotated_field_array_v1;

typedef struct thinkthen_complete_failed_v1 {
  const struct thinkthen_complete_failure_v1 *failed;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_failed_v1;

typedef union thinkthen_complete_annotated_field_data_v1 {
  uint32_t boolean;
  uint32_t null;
  struct thinkthen_complete_utf8_v1 string;
  struct thinkthen_complete_annotated_field_array_v1 array;
  double number;
  const struct thinkthen_complete_failed_v1 *object;
} thinkthen_complete_annotated_field_data_v1;

typedef struct thinkthen_complete_annotated_field_v1 {
  uint32_t kind;
  union thinkthen_complete_annotated_field_data_v1 data;
} thinkthen_complete_annotated_field_v1;

typedef struct thinkthen_complete_annotated_row_value_entry_v1 {
  struct thinkthen_complete_utf8_v1 name;
  const struct thinkthen_complete_annotated_field_v1 *value;
} thinkthen_complete_annotated_row_value_entry_v1;

typedef struct thinkthen_complete_annotated_row_value_v1 {
  const struct thinkthen_complete_annotated_row_value_entry_v1 *data;
  size_t len;
} thinkthen_complete_annotated_row_value_v1;

typedef struct thinkthen_complete_annotated_row_v1 {
  struct thinkthen_complete_annotated_row_value_v1 value;
} thinkthen_complete_annotated_row_v1;

typedef struct thinkthen_complete_annotation_v1 {
  const struct thinkthen_complete_answer_id_v1 *answer_id;
  struct thinkthen_complete_annotation_field_answers_v1 answers;
  struct thinkthen_complete_annotation_field_file_presence_v1 file;
  struct thinkthen_complete_annotation_field_first_line_presence_v1 first_line;
  struct thinkthen_complete_annotation_field_index_presence_v1 index;
  struct thinkthen_complete_annotation_field_input_presence_v1 input;
  struct thinkthen_complete_annotation_field_last_line_presence_v1 last_line;
  const struct thinkthen_complete_meta_v1 *meta;
  struct thinkthen_complete_annotation_field_position_presence_v1 position;
  const struct thinkthen_complete_version_v1 *schema;
  struct thinkthen_complete_annotation_field_source_presence_v1 source;
  const struct thinkthen_complete_annotated_row_v1 *value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_annotation_v1;

typedef struct thinkthen_complete_session_packet_annotate_row_v1 {
  struct thinkthen_complete_utf8_v1 function;
  struct thinkthen_complete_utf8_v1 kind;
  const struct thinkthen_complete_annotation_v1 *value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_session_packet_annotate_row_v1;

typedef struct thinkthen_complete_session_packet_decide_aggregate_field_value_v1 {
  const struct thinkthen_complete_atomic_decide_value_v1 *const *data;
  size_t len;
} thinkthen_complete_session_packet_decide_aggregate_field_value_v1;

typedef struct thinkthen_complete_session_packet_decide_aggregate_v1 {
  struct thinkthen_complete_utf8_v1 function;
  struct thinkthen_complete_utf8_v1 kind;
  struct thinkthen_complete_session_packet_decide_aggregate_field_value_v1 value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_session_packet_decide_aggregate_v1;

typedef struct thinkthen_complete_session_packet_choose_aggregate_field_value_v1 {
  const struct thinkthen_complete_atomic_nullable_string_v1 *const *data;
  size_t len;
} thinkthen_complete_session_packet_choose_aggregate_field_value_v1;

typedef struct thinkthen_complete_session_packet_choose_aggregate_v1 {
  struct thinkthen_complete_utf8_v1 function;
  struct thinkthen_complete_utf8_v1 kind;
  struct thinkthen_complete_session_packet_choose_aggregate_field_value_v1 value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_session_packet_choose_aggregate_v1;

typedef struct thinkthen_complete_session_packet_tag_aggregate_field_value_v1 {
  const struct thinkthen_complete_atomic_array_of_string_v1 *const *data;
  size_t len;
} thinkthen_complete_session_packet_tag_aggregate_field_value_v1;

typedef struct thinkthen_complete_session_packet_tag_aggregate_v1 {
  struct thinkthen_complete_utf8_v1 function;
  struct thinkthen_complete_utf8_v1 kind;
  struct thinkthen_complete_session_packet_tag_aggregate_field_value_v1 value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_session_packet_tag_aggregate_v1;

typedef struct thinkthen_complete_session_packet_score_aggregate_field_value_v1 {
  const struct thinkthen_complete_atomic_double_v1 *const *data;
  size_t len;
} thinkthen_complete_session_packet_score_aggregate_field_value_v1;

typedef struct thinkthen_complete_session_packet_score_aggregate_v1 {
  struct thinkthen_complete_utf8_v1 function;
  struct thinkthen_complete_utf8_v1 kind;
  struct thinkthen_complete_session_packet_score_aggregate_field_value_v1 value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_session_packet_score_aggregate_v1;

typedef struct thinkthen_complete_session_packet_filter_aggregate_field_value_v1 {
  const struct thinkthen_complete_atomic_boolean_v1 *const *data;
  size_t len;
} thinkthen_complete_session_packet_filter_aggregate_field_value_v1;

typedef struct thinkthen_complete_session_packet_filter_aggregate_v1 {
  struct thinkthen_complete_utf8_v1 function;
  struct thinkthen_complete_utf8_v1 kind;
  struct thinkthen_complete_session_packet_filter_aggregate_field_value_v1 value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_session_packet_filter_aggregate_v1;

typedef struct thinkthen_complete_atomic_non_zero_usize_field_images_v1 {
  const struct thinkthen_complete_image_v1 *const *data;
  size_t len;
} thinkthen_complete_atomic_non_zero_usize_field_images_v1;

typedef struct thinkthen_complete_atomic_non_zero_usize_field_images_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_atomic_non_zero_usize_field_images_v1 value;
} thinkthen_complete_atomic_non_zero_usize_field_images_presence_v1;

typedef struct thinkthen_complete_atomic_non_zero_usize_field_index_presence_v1 {
  uint32_t presence;
  uint64_t value;
} thinkthen_complete_atomic_non_zero_usize_field_index_presence_v1;

typedef struct thinkthen_complete_atomic_non_zero_usize_field_input_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_json_v1 *value;
} thinkthen_complete_atomic_non_zero_usize_field_input_presence_v1;

typedef struct thinkthen_complete_atomic_non_zero_usize_field_members_v1 {
  const struct thinkthen_complete_rank_member_v1 *const *data;
  size_t len;
} thinkthen_complete_atomic_non_zero_usize_field_members_v1;

typedef struct thinkthen_complete_atomic_non_zero_usize_field_members_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_atomic_non_zero_usize_field_members_v1 value;
} thinkthen_complete_atomic_non_zero_usize_field_members_presence_v1;

typedef struct thinkthen_complete_atomic_non_zero_usize_field_question_name_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_utf8_v1 value;
} thinkthen_complete_atomic_non_zero_usize_field_question_name_presence_v1;

typedef struct thinkthen_complete_atomic_non_zero_usize_field_source_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_physical_source_v1 *value;
} thinkthen_complete_atomic_non_zero_usize_field_source_presence_v1;

typedef struct thinkthen_complete_atomic_non_zero_usize_field_threshold_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_threshold_v1 *value;
} thinkthen_complete_atomic_non_zero_usize_field_threshold_presence_v1;

typedef struct thinkthen_complete_atomic_non_zero_usize_v1 {
  const struct thinkthen_complete_answer_v1 *answer;
  const struct thinkthen_complete_answer_id_v1 *answer_id;
  struct thinkthen_complete_atomic_non_zero_usize_field_images_presence_v1 images;
  struct thinkthen_complete_atomic_non_zero_usize_field_index_presence_v1 index;
  struct thinkthen_complete_atomic_non_zero_usize_field_input_presence_v1 input;
  struct thinkthen_complete_atomic_non_zero_usize_field_members_presence_v1 members;
  const struct thinkthen_complete_meta_v1 *meta;
  const struct thinkthen_complete_readable_question_v1 *question;
  struct thinkthen_complete_atomic_non_zero_usize_field_question_name_presence_v1 question_name;
  const struct thinkthen_complete_version_v1 *schema;
  struct thinkthen_complete_atomic_non_zero_usize_field_source_presence_v1 source;
  struct thinkthen_complete_atomic_non_zero_usize_field_threshold_presence_v1 threshold;
  uint64_t value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_atomic_non_zero_usize_v1;

typedef struct thinkthen_complete_session_packet_rank_aggregate_field_value_v1 {
  const struct thinkthen_complete_atomic_non_zero_usize_v1 *const *data;
  size_t len;
} thinkthen_complete_session_packet_rank_aggregate_field_value_v1;

typedef struct thinkthen_complete_session_packet_rank_aggregate_v1 {
  struct thinkthen_complete_utf8_v1 function;
  struct thinkthen_complete_utf8_v1 kind;
  struct thinkthen_complete_session_packet_rank_aggregate_field_value_v1 value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_session_packet_rank_aggregate_v1;

typedef struct thinkthen_complete_find_answer_field_confidence_presence_v1 {
  uint32_t presence;
  double value;
} thinkthen_complete_find_answer_field_confidence_presence_v1;

typedef struct thinkthen_complete_find_answer_field_probabilities_entry_v1 {
  struct thinkthen_complete_utf8_v1 name;
  double value;
} thinkthen_complete_find_answer_field_probabilities_entry_v1;

typedef struct thinkthen_complete_find_answer_field_probabilities_v1 {
  const struct thinkthen_complete_find_answer_field_probabilities_entry_v1 *data;
  size_t len;
} thinkthen_complete_find_answer_field_probabilities_v1;

typedef struct thinkthen_complete_find_answer_v1 {
  struct thinkthen_complete_find_answer_field_confidence_presence_v1 confidence;
  struct thinkthen_complete_utf8_v1 kind;
  struct thinkthen_complete_utf8_v1 pick;
  struct thinkthen_complete_find_answer_field_probabilities_v1 probabilities;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_find_answer_v1;

typedef struct thinkthen_complete_find_candidate_field_index_presence_v1 {
  uint32_t presence;
  uint64_t value;
} thinkthen_complete_find_candidate_field_index_presence_v1;

typedef struct thinkthen_complete_find_candidate_field_input_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_json_v1 *value;
} thinkthen_complete_find_candidate_field_input_presence_v1;

typedef struct thinkthen_complete_find_candidate_field_source_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_physical_source_v1 *value;
} thinkthen_complete_find_candidate_field_source_presence_v1;

typedef struct thinkthen_complete_find_candidate_v1 {
  struct thinkthen_complete_find_candidate_field_index_presence_v1 index;
  struct thinkthen_complete_find_candidate_field_input_presence_v1 input;
  double probability;
  struct thinkthen_complete_find_candidate_field_source_presence_v1 source;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_find_candidate_v1;

typedef struct thinkthen_complete_find_field_candidates_v1 {
  const struct thinkthen_complete_find_candidate_v1 *const *data;
  size_t len;
} thinkthen_complete_find_field_candidates_v1;

typedef struct thinkthen_complete_find_field_candidates_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_find_field_candidates_v1 value;
} thinkthen_complete_find_field_candidates_presence_v1;

typedef struct thinkthen_complete_find_field_file_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_utf8_v1 value;
} thinkthen_complete_find_field_file_presence_v1;

typedef struct thinkthen_complete_find_field_first_line_presence_v1 {
  uint32_t presence;
  uint64_t value;
} thinkthen_complete_find_field_first_line_presence_v1;

typedef struct thinkthen_complete_find_field_index_presence_v1 {
  uint32_t presence;
  uint64_t value;
} thinkthen_complete_find_field_index_presence_v1;

typedef struct thinkthen_complete_find_field_last_line_presence_v1 {
  uint32_t presence;
  uint64_t value;
} thinkthen_complete_find_field_last_line_presence_v1;

typedef struct thinkthen_complete_find_field_position_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_position_v1 *value;
} thinkthen_complete_find_field_position_presence_v1;

typedef struct thinkthen_complete_readable_question_2_field_batch_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_batch_v1 *value;
} thinkthen_complete_readable_question_2_field_batch_presence_v1;

typedef struct thinkthen_complete_readable_question_2_field_context_schema_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_input_declaration_v1 *value;
} thinkthen_complete_readable_question_2_field_context_schema_presence_v1;

typedef struct thinkthen_complete_readable_question_2_field_item_schema_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_input_declaration_v1 *value;
} thinkthen_complete_readable_question_2_field_item_schema_presence_v1;

typedef struct thinkthen_complete_readable_question_2_field_label_details_v1 {
  const struct thinkthen_complete_label_v1 *const *data;
  size_t len;
} thinkthen_complete_readable_question_2_field_label_details_v1;

typedef struct thinkthen_complete_readable_question_2_field_label_details_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_readable_question_2_field_label_details_v1 value;
} thinkthen_complete_readable_question_2_field_label_details_presence_v1;

typedef struct thinkthen_complete_readable_question_2_field_model_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_utf8_v1 value;
} thinkthen_complete_readable_question_2_field_model_presence_v1;

typedef struct thinkthen_complete_readable_question_2_field_name_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_question_name_v1 *value;
} thinkthen_complete_readable_question_2_field_name_presence_v1;

typedef struct thinkthen_complete_readable_question_2_field_on_v1 {
  const struct thinkthen_complete_utf8_v1 *data;
  size_t len;
} thinkthen_complete_readable_question_2_field_on_v1;

typedef struct thinkthen_complete_readable_question_2_field_on_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_readable_question_2_field_on_v1 value;
} thinkthen_complete_readable_question_2_field_on_presence_v1;

typedef struct thinkthen_complete_readable_question_2_field_profile_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_utf8_v1 value;
} thinkthen_complete_readable_question_2_field_profile_presence_v1;

typedef struct thinkthen_complete_readable_question_2_field_text_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_json_v1 *value;
} thinkthen_complete_readable_question_2_field_text_presence_v1;

typedef struct thinkthen_complete_readable_question_2_field_wording_version_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_wording_version_v1 *value;
} thinkthen_complete_readable_question_2_field_wording_version_presence_v1;

typedef struct thinkthen_complete_readable_question_2_v1 {
  struct thinkthen_complete_readable_question_2_field_batch_presence_v1 batch;
  struct thinkthen_complete_readable_question_2_field_context_schema_presence_v1 context_schema;
  struct thinkthen_complete_readable_question_2_field_item_schema_presence_v1 item_schema;
  struct thinkthen_complete_readable_question_2_field_label_details_presence_v1 label_details;
  struct thinkthen_complete_readable_question_2_field_model_presence_v1 model;
  struct thinkthen_complete_readable_question_2_field_name_presence_v1 name;
  uint32_t none;
  struct thinkthen_complete_readable_question_2_field_on_presence_v1 on;
  struct thinkthen_complete_readable_question_2_field_profile_presence_v1 profile;
  struct thinkthen_complete_readable_question_2_field_text_presence_v1 text;
  struct thinkthen_complete_utf8_v1 verb;
  struct thinkthen_complete_readable_question_2_field_wording_version_presence_v1 wording_version;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_readable_question_2_v1;

typedef struct thinkthen_complete_find_field_threshold_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_json_v1 *value;
} thinkthen_complete_find_field_threshold_presence_v1;

typedef struct thinkthen_complete_find_field_value_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_json_v1 *value;
} thinkthen_complete_find_field_value_presence_v1;

typedef struct thinkthen_complete_find_v1 {
  const struct thinkthen_complete_find_answer_v1 *answer;
  const struct thinkthen_complete_answer_id_v1 *answer_id;
  struct thinkthen_complete_find_field_candidates_presence_v1 candidates;
  struct thinkthen_complete_find_field_file_presence_v1 file;
  struct thinkthen_complete_find_field_first_line_presence_v1 first_line;
  struct thinkthen_complete_find_field_index_presence_v1 index;
  struct thinkthen_complete_find_field_last_line_presence_v1 last_line;
  const struct thinkthen_complete_meta_v1 *meta;
  struct thinkthen_complete_find_field_position_presence_v1 position;
  const struct thinkthen_complete_readable_question_2_v1 *question;
  const struct thinkthen_complete_version_v1 *schema;
  struct thinkthen_complete_find_field_threshold_presence_v1 threshold;
  struct thinkthen_complete_find_field_value_presence_v1 value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_find_v1;

typedef struct thinkthen_complete_session_packet_find_aggregate_v1 {
  struct thinkthen_complete_utf8_v1 function;
  struct thinkthen_complete_utf8_v1 kind;
  const struct thinkthen_complete_find_v1 *value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_session_packet_find_aggregate_v1;

typedef struct thinkthen_complete_session_packet_annotate_aggregate_field_value_v1 {
  const struct thinkthen_complete_annotation_v1 *const *data;
  size_t len;
} thinkthen_complete_session_packet_annotate_aggregate_field_value_v1;

typedef struct thinkthen_complete_session_packet_annotate_aggregate_v1 {
  struct thinkthen_complete_utf8_v1 function;
  struct thinkthen_complete_utf8_v1 kind;
  struct thinkthen_complete_session_packet_annotate_aggregate_field_value_v1 value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_session_packet_annotate_aggregate_v1;

typedef struct thinkthen_complete_name_odds_field_edges_entry_v1 {
  struct thinkthen_complete_utf8_v1 name;
  double value;
} thinkthen_complete_name_odds_field_edges_entry_v1;

typedef struct thinkthen_complete_name_odds_field_edges_v1 {
  const struct thinkthen_complete_name_odds_field_edges_entry_v1 *data;
  size_t len;
} thinkthen_complete_name_odds_field_edges_v1;

typedef struct thinkthen_complete_name_odds_field_edges_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_name_odds_field_edges_v1 value;
} thinkthen_complete_name_odds_field_edges_presence_v1;

typedef struct thinkthen_complete_name_odds_field_kinds_entry_v1 {
  struct thinkthen_complete_utf8_v1 name;
  double value;
} thinkthen_complete_name_odds_field_kinds_entry_v1;

typedef struct thinkthen_complete_name_odds_field_kinds_v1 {
  const struct thinkthen_complete_name_odds_field_kinds_entry_v1 *data;
  size_t len;
} thinkthen_complete_name_odds_field_kinds_v1;

typedef struct thinkthen_complete_name_odds_field_kinds_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_name_odds_field_kinds_v1 value;
} thinkthen_complete_name_odds_field_kinds_presence_v1;

typedef struct thinkthen_complete_name_odds_v1 {
  struct thinkthen_complete_name_odds_field_edges_presence_v1 edges;
  uint64_t end;
  struct thinkthen_complete_name_odds_field_kinds_presence_v1 kinds;
  uint64_t start;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_name_odds_v1;

typedef struct thinkthen_complete_recognition_odds_fields_names_pairs_pieces_proposals_field_names_v1 {
  const struct thinkthen_complete_name_odds_v1 *const *data;
  size_t len;
} thinkthen_complete_recognition_odds_fields_names_pairs_pieces_proposals_field_names_v1;

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

typedef struct thinkthen_complete_recognition_odds_fields_names_pairs_pieces_proposals_field_pairs_v1 {
  const struct thinkthen_complete_pair_odds_v1 *const *data;
  size_t len;
} thinkthen_complete_recognition_odds_fields_names_pairs_pieces_proposals_field_pairs_v1;

typedef struct thinkthen_complete_piece_odds_field_tags_entry_v1 {
  struct thinkthen_complete_utf8_v1 name;
  double value;
} thinkthen_complete_piece_odds_field_tags_entry_v1;

typedef struct thinkthen_complete_piece_odds_field_tags_v1 {
  const struct thinkthen_complete_piece_odds_field_tags_entry_v1 *data;
  size_t len;
} thinkthen_complete_piece_odds_field_tags_v1;

typedef struct thinkthen_complete_piece_odds_v1 {
  uint64_t end;
  uint64_t start;
  struct thinkthen_complete_piece_odds_field_tags_v1 tags;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_piece_odds_v1;

typedef struct thinkthen_complete_recognition_odds_fields_names_pairs_pieces_proposals_field_pieces_v1 {
  const struct thinkthen_complete_piece_odds_v1 *const *data;
  size_t len;
} thinkthen_complete_recognition_odds_fields_names_pairs_pieces_proposals_field_pieces_v1;

typedef struct thinkthen_complete_recognition_proposal_field_kind_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_utf8_v1 value;
} thinkthen_complete_recognition_proposal_field_kind_presence_v1;

typedef struct thinkthen_complete_recognition_proposal_field_selected_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_place_v1 *value;
} thinkthen_complete_recognition_proposal_field_selected_presence_v1;

typedef struct thinkthen_complete_recognition_proposal_field_strength_presence_v1 {
  uint32_t presence;
  double value;
} thinkthen_complete_recognition_proposal_field_strength_presence_v1;

typedef struct thinkthen_complete_recognition_proposal_v1 {
  uint64_t end;
  uint32_t kept;
  struct thinkthen_complete_recognition_proposal_field_kind_presence_v1 kind;
  struct thinkthen_complete_recognition_proposal_field_selected_presence_v1 selected;
  double span_probability;
  uint64_t start;
  struct thinkthen_complete_recognition_proposal_field_strength_presence_v1 strength;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_recognition_proposal_v1;

typedef struct thinkthen_complete_recognition_odds_fields_names_pairs_pieces_proposals_field_proposals_v1 {
  const struct thinkthen_complete_recognition_proposal_v1 *const *data;
  size_t len;
} thinkthen_complete_recognition_odds_fields_names_pairs_pieces_proposals_field_proposals_v1;

typedef struct thinkthen_complete_recognition_odds_fields_names_pairs_pieces_proposals_v1 {
  struct thinkthen_complete_recognition_odds_fields_names_pairs_pieces_proposals_field_names_v1 names;
  struct thinkthen_complete_recognition_odds_fields_names_pairs_pieces_proposals_field_pairs_v1 pairs;
  struct thinkthen_complete_recognition_odds_fields_names_pairs_pieces_proposals_field_pieces_v1 pieces;
  struct thinkthen_complete_recognition_odds_fields_names_pairs_pieces_proposals_field_proposals_v1 proposals;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_recognition_odds_fields_names_pairs_pieces_proposals_v1;

typedef struct thinkthen_complete_recognition_odds_fields_pieces_proposals_field_pieces_v1 {
  const struct thinkthen_complete_piece_odds_v1 *const *data;
  size_t len;
} thinkthen_complete_recognition_odds_fields_pieces_proposals_field_pieces_v1;

typedef struct thinkthen_complete_boundary_proposal_v1 {
  uint64_t end;
  uint64_t length;
  double probability;
  uint64_t start;
  struct thinkthen_complete_utf8_v1 text;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_boundary_proposal_v1;

typedef struct thinkthen_complete_recognition_odds_fields_pieces_proposals_field_proposals_v1 {
  const struct thinkthen_complete_boundary_proposal_v1 *const *data;
  size_t len;
} thinkthen_complete_recognition_odds_fields_pieces_proposals_field_proposals_v1;

typedef struct thinkthen_complete_recognition_odds_fields_pieces_proposals_v1 {
  struct thinkthen_complete_recognition_odds_fields_pieces_proposals_field_pieces_v1 pieces;
  struct thinkthen_complete_recognition_odds_fields_pieces_proposals_field_proposals_v1 proposals;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_recognition_odds_fields_pieces_proposals_v1;

typedef union thinkthen_complete_recognition_odds_data_v1 {
  const struct thinkthen_complete_recognition_odds_fields_names_pairs_pieces_proposals_v1 *fields_names_pairs_pieces_proposals;
  const struct thinkthen_complete_recognition_odds_fields_pieces_proposals_v1 *fields_pieces_proposals;
} thinkthen_complete_recognition_odds_data_v1;

typedef struct thinkthen_complete_recognition_odds_v1 {
  uint32_t kind;
  union thinkthen_complete_recognition_odds_data_v1 data;
} thinkthen_complete_recognition_odds_v1;

typedef struct thinkthen_complete_recognition_field_file_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_utf8_v1 value;
} thinkthen_complete_recognition_field_file_presence_v1;

typedef struct thinkthen_complete_recognition_field_first_line_presence_v1 {
  uint32_t presence;
  uint64_t value;
} thinkthen_complete_recognition_field_first_line_presence_v1;

typedef struct thinkthen_complete_recognition_field_index_presence_v1 {
  uint32_t presence;
  uint64_t value;
} thinkthen_complete_recognition_field_index_presence_v1;

typedef struct thinkthen_complete_recognition_field_input_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_json_v1 *value;
} thinkthen_complete_recognition_field_input_presence_v1;

typedef struct thinkthen_complete_recognition_field_last_line_presence_v1 {
  uint32_t presence;
  uint64_t value;
} thinkthen_complete_recognition_field_last_line_presence_v1;

typedef struct thinkthen_complete_recognition_field_position_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_position_v1 *value;
} thinkthen_complete_recognition_field_position_presence_v1;

typedef struct thinkthen_complete_readable_question_3_field_batch_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_batch_v1 *value;
} thinkthen_complete_readable_question_3_field_batch_presence_v1;

typedef struct thinkthen_complete_readable_question_3_field_context_schema_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_input_declaration_v1 *value;
} thinkthen_complete_readable_question_3_field_context_schema_presence_v1;

typedef struct thinkthen_complete_readable_question_3_field_entity_definition_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_json_v1 *value;
} thinkthen_complete_readable_question_3_field_entity_definition_presence_v1;

typedef struct thinkthen_complete_readable_question_3_field_instructions_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_json_v1 *value;
} thinkthen_complete_readable_question_3_field_instructions_presence_v1;

typedef struct thinkthen_complete_readable_question_3_field_item_schema_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_input_declaration_v1 *value;
} thinkthen_complete_readable_question_3_field_item_schema_presence_v1;

typedef struct thinkthen_complete_readable_question_3_field_kinds_entry_v1 {
  struct thinkthen_complete_utf8_v1 name;
  const struct thinkthen_complete_json_v1 *value;
} thinkthen_complete_readable_question_3_field_kinds_entry_v1;

typedef struct thinkthen_complete_readable_question_3_field_kinds_v1 {
  const struct thinkthen_complete_readable_question_3_field_kinds_entry_v1 *data;
  size_t len;
} thinkthen_complete_readable_question_3_field_kinds_v1;

typedef struct thinkthen_complete_readable_question_3_field_label_details_v1 {
  const struct thinkthen_complete_label_v1 *const *data;
  size_t len;
} thinkthen_complete_readable_question_3_field_label_details_v1;

typedef struct thinkthen_complete_readable_question_3_field_label_details_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_readable_question_3_field_label_details_v1 value;
} thinkthen_complete_readable_question_3_field_label_details_presence_v1;

typedef struct thinkthen_complete_recognition_mode_v1 {
  uint32_t kind;
} thinkthen_complete_recognition_mode_v1;

typedef struct thinkthen_complete_readable_question_3_field_mode_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_recognition_mode_v1 *value;
} thinkthen_complete_readable_question_3_field_mode_presence_v1;

typedef struct thinkthen_complete_readable_question_3_field_model_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_utf8_v1 value;
} thinkthen_complete_readable_question_3_field_model_presence_v1;

typedef struct thinkthen_complete_readable_question_3_field_name_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_question_name_v1 *value;
} thinkthen_complete_readable_question_3_field_name_presence_v1;

typedef struct thinkthen_complete_readable_question_3_field_on_v1 {
  const struct thinkthen_complete_utf8_v1 *data;
  size_t len;
} thinkthen_complete_readable_question_3_field_on_v1;

typedef struct thinkthen_complete_readable_question_3_field_on_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_readable_question_3_field_on_v1 value;
} thinkthen_complete_readable_question_3_field_on_presence_v1;

typedef struct thinkthen_complete_readable_question_3_field_profile_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_utf8_v1 value;
} thinkthen_complete_readable_question_3_field_profile_presence_v1;

typedef struct thinkthen_complete_readable_question_3_field_relation_threshold_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_threshold_v1 *value;
} thinkthen_complete_readable_question_3_field_relation_threshold_presence_v1;

typedef struct thinkthen_complete_relation_rule_field_single_presence_v1 {
  uint32_t presence;
  uint32_t value;
} thinkthen_complete_relation_rule_field_single_presence_v1;

typedef struct thinkthen_complete_relation_rule_v1 {
  uint32_t either;
  struct thinkthen_complete_utf8_v1 name;
  struct thinkthen_complete_utf8_v1 reads;
  struct thinkthen_complete_relation_rule_field_single_presence_v1 single;
  struct thinkthen_complete_utf8_v1 source;
  struct thinkthen_complete_utf8_v1 target;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_relation_rule_v1;

typedef struct thinkthen_complete_readable_question_3_field_relations_v1 {
  const struct thinkthen_complete_relation_rule_v1 *const *data;
  size_t len;
} thinkthen_complete_readable_question_3_field_relations_v1;

typedef struct thinkthen_complete_readable_question_3_field_relations_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_readable_question_3_field_relations_v1 value;
} thinkthen_complete_readable_question_3_field_relations_presence_v1;

typedef struct thinkthen_complete_readable_question_3_field_snippet_pieces_presence_v1 {
  uint32_t presence;
  uint64_t value;
} thinkthen_complete_readable_question_3_field_snippet_pieces_presence_v1;

typedef struct thinkthen_complete_recognition_stage_context_field_boundary_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_utf8_v1 value;
} thinkthen_complete_recognition_stage_context_field_boundary_presence_v1;

typedef struct thinkthen_complete_recognition_stage_context_field_kind_edge_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_utf8_v1 value;
} thinkthen_complete_recognition_stage_context_field_kind_edge_presence_v1;

typedef struct thinkthen_complete_recognition_stage_context_field_relation_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_utf8_v1 value;
} thinkthen_complete_recognition_stage_context_field_relation_presence_v1;

typedef struct thinkthen_complete_recognition_stage_context_v1 {
  struct thinkthen_complete_recognition_stage_context_field_boundary_presence_v1 boundary;
  struct thinkthen_complete_recognition_stage_context_field_kind_edge_presence_v1 kind_edge;
  struct thinkthen_complete_recognition_stage_context_field_relation_presence_v1 relation;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_recognition_stage_context_v1;

typedef struct thinkthen_complete_readable_question_3_field_stage_context_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_recognition_stage_context_v1 *value;
} thinkthen_complete_readable_question_3_field_stage_context_presence_v1;

typedef struct thinkthen_complete_verb_v1 {
  uint32_t kind;
} thinkthen_complete_verb_v1;

typedef struct thinkthen_complete_readable_question_3_field_wording_version_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_wording_version_v1 *value;
} thinkthen_complete_readable_question_3_field_wording_version_presence_v1;

typedef struct thinkthen_complete_readable_question_3_v1 {
  struct thinkthen_complete_readable_question_3_field_batch_presence_v1 batch;
  struct thinkthen_complete_readable_question_3_field_context_schema_presence_v1 context_schema;
  struct thinkthen_complete_readable_question_3_field_entity_definition_presence_v1 entity_definition;
  struct thinkthen_complete_readable_question_3_field_instructions_presence_v1 instructions;
  struct thinkthen_complete_readable_question_3_field_item_schema_presence_v1 item_schema;
  struct thinkthen_complete_readable_question_3_field_kinds_v1 kinds;
  struct thinkthen_complete_readable_question_3_field_label_details_presence_v1 label_details;
  struct thinkthen_complete_readable_question_3_field_mode_presence_v1 mode;
  struct thinkthen_complete_readable_question_3_field_model_presence_v1 model;
  struct thinkthen_complete_readable_question_3_field_name_presence_v1 name;
  struct thinkthen_complete_readable_question_3_field_on_presence_v1 on;
  struct thinkthen_complete_readable_question_3_field_profile_presence_v1 profile;
  struct thinkthen_complete_readable_question_3_field_relation_threshold_presence_v1 relation_threshold;
  struct thinkthen_complete_readable_question_3_field_relations_presence_v1 relations;
  struct thinkthen_complete_readable_question_3_field_snippet_pieces_presence_v1 snippet_pieces;
  struct thinkthen_complete_readable_question_3_field_stage_context_presence_v1 stage_context;
  const struct thinkthen_complete_threshold_v1 *threshold;
  const struct thinkthen_complete_verb_v1 *verb;
  struct thinkthen_complete_readable_question_3_field_wording_version_presence_v1 wording_version;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_readable_question_3_v1;

typedef struct thinkthen_complete_recognition_field_source_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_physical_source_v1 *value;
} thinkthen_complete_recognition_field_source_presence_v1;

typedef struct thinkthen_complete_entity_field_file_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_utf8_v1 value;
} thinkthen_complete_entity_field_file_presence_v1;

typedef struct thinkthen_complete_entity_field_first_line_presence_v1 {
  uint32_t presence;
  uint64_t value;
} thinkthen_complete_entity_field_first_line_presence_v1;

typedef struct thinkthen_complete_entity_field_last_line_presence_v1 {
  uint32_t presence;
  uint64_t value;
} thinkthen_complete_entity_field_last_line_presence_v1;

typedef struct thinkthen_complete_entity_v1 {
  uint64_t end;
  struct thinkthen_complete_entity_field_file_presence_v1 file;
  struct thinkthen_complete_entity_field_first_line_presence_v1 first_line;
  struct thinkthen_complete_utf8_v1 kind;
  struct thinkthen_complete_entity_field_last_line_presence_v1 last_line;
  uint64_t length;
  uint64_t start;
  double strength;
  struct thinkthen_complete_utf8_v1 text;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_entity_v1;

typedef struct thinkthen_complete_recognize_fields_entities_field_entities_v1 {
  const struct thinkthen_complete_entity_v1 *const *data;
  size_t len;
} thinkthen_complete_recognize_fields_entities_field_entities_v1;

typedef struct thinkthen_complete_entity_edge_field_either_presence_v1 {
  uint32_t presence;
  uint32_t value;
} thinkthen_complete_entity_edge_field_either_presence_v1;

typedef struct thinkthen_complete_entity_edge_v1 {
  struct thinkthen_complete_entity_edge_field_either_presence_v1 either;
  double probability;
  struct thinkthen_complete_utf8_v1 relation;
  const struct thinkthen_complete_entity_v1 *source;
  const struct thinkthen_complete_entity_v1 *target;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_entity_edge_v1;

typedef struct thinkthen_complete_recognize_fields_entities_field_relations_v1 {
  const struct thinkthen_complete_entity_edge_v1 *const *data;
  size_t len;
} thinkthen_complete_recognize_fields_entities_field_relations_v1;

typedef struct thinkthen_complete_recognize_fields_entities_field_relations_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_recognize_fields_entities_field_relations_v1 value;
} thinkthen_complete_recognize_fields_entities_field_relations_presence_v1;

typedef struct thinkthen_complete_recognize_fields_entities_v1 {
  struct thinkthen_complete_recognize_fields_entities_field_entities_v1 entities;
  struct thinkthen_complete_recognize_fields_entities_field_relations_presence_v1 relations;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_recognize_fields_entities_v1;

typedef struct thinkthen_complete_boundary_mode_v1 {
  uint32_t kind;
} thinkthen_complete_boundary_mode_v1;

typedef struct thinkthen_complete_recognize_fields_mode_proposals_field_proposals_v1 {
  const struct thinkthen_complete_boundary_proposal_v1 *const *data;
  size_t len;
} thinkthen_complete_recognize_fields_mode_proposals_field_proposals_v1;

typedef struct thinkthen_complete_recognize_fields_mode_proposals_v1 {
  const struct thinkthen_complete_boundary_mode_v1 *mode;
  struct thinkthen_complete_recognize_fields_mode_proposals_field_proposals_v1 proposals;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_recognize_fields_mode_proposals_v1;

typedef union thinkthen_complete_recognize_data_v1 {
  const struct thinkthen_complete_recognize_fields_entities_v1 *fields_entities;
  const struct thinkthen_complete_recognize_fields_mode_proposals_v1 *fields_mode_proposals;
} thinkthen_complete_recognize_data_v1;

typedef struct thinkthen_complete_recognize_v1 {
  uint32_t kind;
  union thinkthen_complete_recognize_data_v1 data;
} thinkthen_complete_recognize_v1;

typedef struct thinkthen_complete_recognition_v1 {
  const struct thinkthen_complete_recognition_odds_v1 *answer;
  const struct thinkthen_complete_answer_id_v1 *answer_id;
  struct thinkthen_complete_recognition_field_file_presence_v1 file;
  struct thinkthen_complete_recognition_field_first_line_presence_v1 first_line;
  struct thinkthen_complete_recognition_field_index_presence_v1 index;
  struct thinkthen_complete_recognition_field_input_presence_v1 input;
  struct thinkthen_complete_recognition_field_last_line_presence_v1 last_line;
  const struct thinkthen_complete_meta_v1 *meta;
  struct thinkthen_complete_recognition_field_position_presence_v1 position;
  const struct thinkthen_complete_readable_question_3_v1 *question;
  const struct thinkthen_complete_version_v1 *schema;
  struct thinkthen_complete_recognition_field_source_presence_v1 source;
  const struct thinkthen_complete_recognize_v1 *value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_recognition_v1;

typedef struct thinkthen_complete_session_packet_recognize_aggregate_field_value_v1 {
  const struct thinkthen_complete_recognition_v1 *const *data;
  size_t len;
} thinkthen_complete_session_packet_recognize_aggregate_field_value_v1;

typedef struct thinkthen_complete_session_packet_recognize_aggregate_v1 {
  struct thinkthen_complete_utf8_v1 function;
  struct thinkthen_complete_utf8_v1 kind;
  struct thinkthen_complete_session_packet_recognize_aggregate_field_value_v1 value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_session_packet_recognize_aggregate_v1;

typedef struct thinkthen_complete_relation_direction_v1 {
  uint32_t kind;
} thinkthen_complete_relation_direction_v1;

typedef struct thinkthen_complete_relation_method_v1 {
  uint32_t kind;
} thinkthen_complete_relation_method_v1;

typedef struct thinkthen_complete_relation_member_answer_id_field_observations_v1 {
  const struct thinkthen_complete_observation_v1 *const *data;
  size_t len;
} thinkthen_complete_relation_member_answer_id_field_observations_v1;

typedef struct thinkthen_complete_relation_member_answer_id_field_question_sources_v1 {
  const struct thinkthen_complete_question_source_v1 *const *data;
  size_t len;
} thinkthen_complete_relation_member_answer_id_field_question_sources_v1;

typedef struct thinkthen_complete_related_entity_v1 {
  struct thinkthen_complete_utf8_v1 kind;
  struct thinkthen_complete_utf8_v1 name;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_related_entity_v1;

typedef struct thinkthen_complete_relation_member_answer_id_field_target_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_related_entity_v1 *value;
} thinkthen_complete_relation_member_answer_id_field_target_presence_v1;

typedef struct thinkthen_complete_relation_member_answer_id_field_usage_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_usage_v1 *value;
} thinkthen_complete_relation_member_answer_id_field_usage_presence_v1;

typedef struct thinkthen_complete_relation_member_answer_id_v1 {
  const struct thinkthen_complete_relation_direction_v1 *direction;
  const struct thinkthen_complete_relation_method_v1 *method;
  struct thinkthen_complete_relation_member_answer_id_field_observations_v1 observations;
  const struct thinkthen_complete_readable_question_v1 *question;
  struct thinkthen_complete_relation_member_answer_id_field_question_sources_v1 question_sources;
  struct thinkthen_complete_utf8_v1 reads;
  struct thinkthen_complete_utf8_v1 relation;
  struct thinkthen_complete_utf8_v1 request;
  const struct thinkthen_complete_related_entity_v1 *source;
  struct thinkthen_complete_relation_member_answer_id_field_target_presence_v1 target;
  const struct thinkthen_complete_threshold_v1 *threshold;
  struct thinkthen_complete_relation_member_answer_id_field_usage_presence_v1 usage;
  uint32_t accepted;
  const struct thinkthen_complete_answer_v1 *answer;
  const struct thinkthen_complete_answer_id_v1 *answer_id;
  double probability;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_relation_member_answer_id_v1;

typedef struct thinkthen_complete_relation_member_failure_id_field_observations_v1 {
  const struct thinkthen_complete_observation_v1 *const *data;
  size_t len;
} thinkthen_complete_relation_member_failure_id_field_observations_v1;

typedef struct thinkthen_complete_relation_member_failure_id_field_question_sources_v1 {
  const struct thinkthen_complete_question_source_v1 *const *data;
  size_t len;
} thinkthen_complete_relation_member_failure_id_field_question_sources_v1;

typedef struct thinkthen_complete_relation_member_failure_id_field_target_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_related_entity_v1 *value;
} thinkthen_complete_relation_member_failure_id_field_target_presence_v1;

typedef struct thinkthen_complete_relation_member_failure_id_field_usage_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_usage_v1 *value;
} thinkthen_complete_relation_member_failure_id_field_usage_presence_v1;

typedef struct thinkthen_complete_relation_member_failure_id_v1 {
  const struct thinkthen_complete_relation_direction_v1 *direction;
  const struct thinkthen_complete_relation_method_v1 *method;
  struct thinkthen_complete_relation_member_failure_id_field_observations_v1 observations;
  const struct thinkthen_complete_readable_question_v1 *question;
  struct thinkthen_complete_relation_member_failure_id_field_question_sources_v1 question_sources;
  struct thinkthen_complete_utf8_v1 reads;
  struct thinkthen_complete_utf8_v1 relation;
  struct thinkthen_complete_utf8_v1 request;
  const struct thinkthen_complete_related_entity_v1 *source;
  struct thinkthen_complete_relation_member_failure_id_field_target_presence_v1 target;
  const struct thinkthen_complete_threshold_v1 *threshold;
  struct thinkthen_complete_relation_member_failure_id_field_usage_presence_v1 usage;
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

typedef struct thinkthen_complete_answers_field_questions_v1 {
  const struct thinkthen_complete_relation_member_v1 *const *data;
  size_t len;
} thinkthen_complete_answers_field_questions_v1;

typedef struct thinkthen_complete_answers_v1 {
  struct thinkthen_complete_answers_field_questions_v1 questions;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_answers_v1;

typedef struct thinkthen_complete_relation_field_file_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_utf8_v1 value;
} thinkthen_complete_relation_field_file_presence_v1;

typedef struct thinkthen_complete_relation_field_first_line_presence_v1 {
  uint32_t presence;
  uint64_t value;
} thinkthen_complete_relation_field_first_line_presence_v1;

typedef struct thinkthen_complete_relation_field_index_presence_v1 {
  uint32_t presence;
  uint64_t value;
} thinkthen_complete_relation_field_index_presence_v1;

typedef struct thinkthen_complete_relation_field_input_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_json_v1 *value;
} thinkthen_complete_relation_field_input_presence_v1;

typedef struct thinkthen_complete_session_input_source_v1 {
  uint64_t index;
  const struct thinkthen_complete_physical_source_v1 *source;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_session_input_source_v1;

typedef struct thinkthen_complete_relation_field_input_sources_v1 {
  const struct thinkthen_complete_session_input_source_v1 *const *data;
  size_t len;
} thinkthen_complete_relation_field_input_sources_v1;

typedef struct thinkthen_complete_relation_field_input_sources_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_relation_field_input_sources_v1 value;
} thinkthen_complete_relation_field_input_sources_presence_v1;

typedef struct thinkthen_complete_relation_field_last_line_presence_v1 {
  uint32_t presence;
  uint64_t value;
} thinkthen_complete_relation_field_last_line_presence_v1;

typedef struct thinkthen_complete_relation_field_position_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_position_v1 *value;
} thinkthen_complete_relation_field_position_presence_v1;

typedef struct thinkthen_complete_readable_question_4_field_batch_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_batch_v1 *value;
} thinkthen_complete_readable_question_4_field_batch_presence_v1;

typedef struct thinkthen_complete_readable_question_4_field_context_schema_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_input_declaration_v1 *value;
} thinkthen_complete_readable_question_4_field_context_schema_presence_v1;

typedef struct thinkthen_complete_relate_fields_v1 {
  struct thinkthen_complete_utf8_v1 kind;
  struct thinkthen_complete_utf8_v1 name;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_relate_fields_v1;

typedef struct thinkthen_complete_readable_question_4_field_fields_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_relate_fields_v1 *value;
} thinkthen_complete_readable_question_4_field_fields_presence_v1;

typedef struct thinkthen_complete_readable_question_4_field_item_schema_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_input_declaration_v1 *value;
} thinkthen_complete_readable_question_4_field_item_schema_presence_v1;

typedef struct thinkthen_complete_readable_question_4_field_label_details_v1 {
  const struct thinkthen_complete_label_v1 *const *data;
  size_t len;
} thinkthen_complete_readable_question_4_field_label_details_v1;

typedef struct thinkthen_complete_readable_question_4_field_label_details_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_readable_question_4_field_label_details_v1 value;
} thinkthen_complete_readable_question_4_field_label_details_presence_v1;

typedef struct thinkthen_complete_readable_question_4_field_model_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_utf8_v1 value;
} thinkthen_complete_readable_question_4_field_model_presence_v1;

typedef struct thinkthen_complete_readable_question_4_field_name_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_question_name_v1 *value;
} thinkthen_complete_readable_question_4_field_name_presence_v1;

typedef struct thinkthen_complete_readable_question_4_field_on_v1 {
  const struct thinkthen_complete_utf8_v1 *data;
  size_t len;
} thinkthen_complete_readable_question_4_field_on_v1;

typedef struct thinkthen_complete_readable_question_4_field_on_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_readable_question_4_field_on_v1 value;
} thinkthen_complete_readable_question_4_field_on_presence_v1;

typedef struct thinkthen_complete_readable_question_4_field_profile_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_utf8_v1 value;
} thinkthen_complete_readable_question_4_field_profile_presence_v1;

typedef struct thinkthen_complete_readable_question_4_field_relations_v1 {
  const struct thinkthen_complete_relation_rule_v1 *const *data;
  size_t len;
} thinkthen_complete_readable_question_4_field_relations_v1;

typedef struct thinkthen_complete_readable_question_4_field_wording_version_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_wording_version_v1 *value;
} thinkthen_complete_readable_question_4_field_wording_version_presence_v1;

typedef struct thinkthen_complete_readable_question_4_v1 {
  struct thinkthen_complete_readable_question_4_field_batch_presence_v1 batch;
  struct thinkthen_complete_readable_question_4_field_context_schema_presence_v1 context_schema;
  struct thinkthen_complete_readable_question_4_field_fields_presence_v1 fields;
  struct thinkthen_complete_readable_question_4_field_item_schema_presence_v1 item_schema;
  struct thinkthen_complete_readable_question_4_field_label_details_presence_v1 label_details;
  struct thinkthen_complete_readable_question_4_field_model_presence_v1 model;
  struct thinkthen_complete_readable_question_4_field_name_presence_v1 name;
  struct thinkthen_complete_readable_question_4_field_on_presence_v1 on;
  struct thinkthen_complete_readable_question_4_field_profile_presence_v1 profile;
  struct thinkthen_complete_readable_question_4_field_relations_v1 relations;
  const struct thinkthen_complete_threshold_v1 *threshold;
  struct thinkthen_complete_utf8_v1 verb;
  struct thinkthen_complete_readable_question_4_field_wording_version_presence_v1 wording_version;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_readable_question_4_v1;

typedef struct thinkthen_complete_related_entity_edge_field_either_presence_v1 {
  uint32_t presence;
  uint32_t value;
} thinkthen_complete_related_entity_edge_field_either_presence_v1;

typedef struct thinkthen_complete_related_entity_edge_properties_source_fields_kind_name_v1 {
  struct thinkthen_complete_utf8_v1 kind;
  struct thinkthen_complete_utf8_v1 name;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_related_entity_edge_properties_source_fields_kind_name_v1;

typedef struct thinkthen_complete_related_entity_edge_properties_source_fields_file_kind_name_ordinal_record_field_file_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_utf8_v1 value;
} thinkthen_complete_related_entity_edge_properties_source_fields_file_kind_name_ordinal_record_field_file_presence_v1;

typedef struct thinkthen_complete_related_entity_edge_properties_source_fields_file_kind_name_ordinal_record_field_first_line_presence_v1 {
  uint32_t presence;
  uint64_t value;
} thinkthen_complete_related_entity_edge_properties_source_fields_file_kind_name_ordinal_record_field_first_line_presence_v1;

typedef struct thinkthen_complete_related_entity_edge_properties_source_fields_file_kind_name_ordinal_record_field_last_line_presence_v1 {
  uint32_t presence;
  uint64_t value;
} thinkthen_complete_related_entity_edge_properties_source_fields_file_kind_name_ordinal_record_field_last_line_presence_v1;

typedef struct thinkthen_complete_related_entity_edge_properties_source_fields_file_kind_name_ordinal_record_field_record_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_json_v1 *value;
} thinkthen_complete_related_entity_edge_properties_source_fields_file_kind_name_ordinal_record_field_record_presence_v1;

typedef struct thinkthen_complete_related_entity_edge_properties_source_fields_file_kind_name_ordinal_record_v1 {
  struct thinkthen_complete_related_entity_edge_properties_source_fields_file_kind_name_ordinal_record_field_file_presence_v1 file;
  struct thinkthen_complete_related_entity_edge_properties_source_fields_file_kind_name_ordinal_record_field_first_line_presence_v1 first_line;
  struct thinkthen_complete_utf8_v1 kind;
  struct thinkthen_complete_related_entity_edge_properties_source_fields_file_kind_name_ordinal_record_field_last_line_presence_v1 last_line;
  struct thinkthen_complete_utf8_v1 name;
  uint64_t ordinal;
  struct thinkthen_complete_related_entity_edge_properties_source_fields_file_kind_name_ordinal_record_field_record_presence_v1 record;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_related_entity_edge_properties_source_fields_file_kind_name_ordinal_record_v1;

typedef union thinkthen_complete_related_entity_edge_properties_source_data_v1 {
  const struct thinkthen_complete_related_entity_edge_properties_source_fields_kind_name_v1 *fields_kind_name;
  const struct thinkthen_complete_related_entity_edge_properties_source_fields_file_kind_name_ordinal_record_v1 *fields_file_kind_name_ordinal_record;
} thinkthen_complete_related_entity_edge_properties_source_data_v1;

typedef struct thinkthen_complete_related_entity_edge_properties_source_v1 {
  uint32_t kind;
  union thinkthen_complete_related_entity_edge_properties_source_data_v1 data;
} thinkthen_complete_related_entity_edge_properties_source_v1;

typedef struct thinkthen_complete_related_entity_edge_v1 {
  struct thinkthen_complete_related_entity_edge_field_either_presence_v1 either;
  double probability;
  struct thinkthen_complete_utf8_v1 relation;
  const struct thinkthen_complete_related_entity_edge_properties_source_v1 *source;
  const struct thinkthen_complete_related_entity_edge_properties_source_v1 *target;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_related_entity_edge_v1;

typedef struct thinkthen_complete_relation_field_value_v1 {
  const struct thinkthen_complete_related_entity_edge_v1 *const *data;
  size_t len;
} thinkthen_complete_relation_field_value_v1;

typedef struct thinkthen_complete_relation_v1 {
  const struct thinkthen_complete_answers_v1 *answer;
  const struct thinkthen_complete_answer_id_v1 *answer_id;
  struct thinkthen_complete_relation_field_file_presence_v1 file;
  struct thinkthen_complete_relation_field_first_line_presence_v1 first_line;
  struct thinkthen_complete_relation_field_index_presence_v1 index;
  struct thinkthen_complete_relation_field_input_presence_v1 input;
  struct thinkthen_complete_relation_field_input_sources_presence_v1 input_sources;
  struct thinkthen_complete_relation_field_last_line_presence_v1 last_line;
  const struct thinkthen_complete_meta_v1 *meta;
  struct thinkthen_complete_relation_field_position_presence_v1 position;
  const struct thinkthen_complete_readable_question_4_v1 *question;
  const struct thinkthen_complete_version_v1 *schema;
  struct thinkthen_complete_relation_field_value_v1 value;
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

typedef struct thinkthen_complete_session_question_detail_field_answer_id_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_answer_id_v1 *value;
} thinkthen_complete_session_question_detail_field_answer_id_presence_v1;

typedef struct thinkthen_complete_session_question_detail_field_confidence_presence_v1 {
  uint32_t presence;
  double value;
} thinkthen_complete_session_question_detail_field_confidence_presence_v1;

typedef struct thinkthen_complete_session_question_detail_field_failure_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_failure_v1 *value;
} thinkthen_complete_session_question_detail_field_failure_presence_v1;

typedef struct thinkthen_complete_session_question_detail_field_failure_id_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_failure_id_v1 *value;
} thinkthen_complete_session_question_detail_field_failure_id_presence_v1;

typedef struct thinkthen_complete_session_question_detail_field_input_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_json_v1 *value;
} thinkthen_complete_session_question_detail_field_input_presence_v1;

typedef struct thinkthen_complete_session_question_detail_field_input_source_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_physical_source_v1 *value;
} thinkthen_complete_session_question_detail_field_input_source_presence_v1;

typedef struct thinkthen_complete_session_question_detail_field_input_sources_v1 {
  const struct thinkthen_complete_session_input_source_v1 *const *data;
  size_t len;
} thinkthen_complete_session_question_detail_field_input_sources_v1;

typedef struct thinkthen_complete_session_question_detail_field_inputs_v1 {
  const struct thinkthen_complete_json_v1 *const *data;
  size_t len;
} thinkthen_complete_session_question_detail_field_inputs_v1;

typedef struct thinkthen_complete_session_question_detail_field_observations_v1 {
  const struct thinkthen_complete_observation_v1 *const *data;
  size_t len;
} thinkthen_complete_session_question_detail_field_observations_v1;

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

typedef struct thinkthen_complete_session_probabilities_named_field_value_v1 {
  const struct thinkthen_complete_session_named_probability_v1 *const *data;
  size_t len;
} thinkthen_complete_session_probabilities_named_field_value_v1;

typedef struct thinkthen_complete_session_probabilities_named_v1 {
  struct thinkthen_complete_utf8_v1 kind;
  struct thinkthen_complete_session_probabilities_named_field_value_v1 value;
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

typedef struct thinkthen_complete_session_question_detail_field_probabilities_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_session_probabilities_v1 *value;
} thinkthen_complete_session_question_detail_field_probabilities_presence_v1;

typedef struct thinkthen_complete_session_question_detail_field_question_sources_v1 {
  const struct thinkthen_complete_question_source_v1 *const *data;
  size_t len;
} thinkthen_complete_session_question_detail_field_question_sources_v1;

typedef struct thinkthen_complete_session_question_detail_field_raw_pick_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_utf8_v1 value;
} thinkthen_complete_session_question_detail_field_raw_pick_presence_v1;

typedef struct thinkthen_complete_session_question_detail_field_reported_usage_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_usage_v1 *value;
} thinkthen_complete_session_question_detail_field_reported_usage_presence_v1;

typedef struct thinkthen_complete_session_question_detail_field_requests_v1 {
  const struct thinkthen_complete_utf8_v1 *data;
  size_t len;
} thinkthen_complete_session_question_detail_field_requests_v1;

typedef struct thinkthen_complete_session_question_detail_field_threshold_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_threshold_v1 *value;
} thinkthen_complete_session_question_detail_field_threshold_presence_v1;

typedef struct thinkthen_complete_token_usage_v1 {
  uint64_t input_tokens;
  uint64_t output_tokens;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_token_usage_v1;

typedef struct thinkthen_complete_session_question_detail_field_usage_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_token_usage_v1 *value;
} thinkthen_complete_session_question_detail_field_usage_presence_v1;

typedef struct thinkthen_complete_session_question_detail_field_value_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_value_v1 *value;
} thinkthen_complete_session_question_detail_field_value_presence_v1;

typedef struct thinkthen_complete_session_question_detail_v1 {
  struct thinkthen_complete_session_question_detail_field_answer_id_presence_v1 answer_id;
  uint32_t cached;
  struct thinkthen_complete_session_question_detail_field_confidence_presence_v1 confidence;
  uint64_t failed_questions;
  struct thinkthen_complete_session_question_detail_field_failure_presence_v1 failure;
  struct thinkthen_complete_session_question_detail_field_failure_id_presence_v1 failure_id;
  struct thinkthen_complete_session_question_detail_field_input_presence_v1 input;
  struct thinkthen_complete_session_question_detail_field_input_source_presence_v1 input_source;
  struct thinkthen_complete_session_question_detail_field_input_sources_v1 input_sources;
  struct thinkthen_complete_session_question_detail_field_inputs_v1 inputs;
  struct thinkthen_complete_utf8_v1 model;
  struct thinkthen_complete_session_question_detail_field_observations_v1 observations;
  struct thinkthen_complete_session_question_detail_field_probabilities_presence_v1 probabilities;
  const struct thinkthen_complete_readable_question_v1 *question;
  struct thinkthen_complete_utf8_v1 question_sha256;
  struct thinkthen_complete_session_question_detail_field_question_sources_v1 question_sources;
  struct thinkthen_complete_session_question_detail_field_raw_pick_presence_v1 raw_pick;
  struct thinkthen_complete_session_question_detail_field_reported_usage_presence_v1 reported_usage;
  struct thinkthen_complete_session_question_detail_field_requests_v1 requests;
  uint64_t requests_sent;
  struct thinkthen_complete_session_question_detail_field_threshold_presence_v1 threshold;
  struct thinkthen_complete_utf8_v1 url;
  struct thinkthen_complete_session_question_detail_field_usage_presence_v1 usage;
  struct thinkthen_complete_session_question_detail_field_value_presence_v1 value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_session_question_detail_v1;

typedef struct thinkthen_complete_session_observation_question_field_member_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_utf8_v1 value;
} thinkthen_complete_session_observation_question_field_member_presence_v1;

typedef struct thinkthen_complete_session_observation_question_field_stage_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_utf8_v1 value;
} thinkthen_complete_session_observation_question_field_stage_presence_v1;

typedef struct thinkthen_complete_session_observation_question_v1 {
  const struct thinkthen_complete_session_question_detail_v1 *detail;
  uint64_t index;
  struct thinkthen_complete_utf8_v1 kind;
  struct thinkthen_complete_session_observation_question_field_member_presence_v1 member;
  uint64_t position;
  struct thinkthen_complete_session_observation_question_field_stage_presence_v1 stage;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_session_observation_question_v1;

typedef struct thinkthen_complete_session_judgment_decision_field_value_presence_v1 {
  uint32_t presence;
  uint32_t value;
} thinkthen_complete_session_judgment_decision_field_value_presence_v1;

typedef struct thinkthen_complete_session_judgment_decision_v1 {
  struct thinkthen_complete_utf8_v1 kind;
  struct thinkthen_complete_session_judgment_decision_field_value_presence_v1 value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_session_judgment_decision_v1;

typedef struct thinkthen_complete_session_judgment_choice_field_value_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_utf8_v1 value;
} thinkthen_complete_session_judgment_choice_field_value_presence_v1;

typedef struct thinkthen_complete_session_judgment_choice_v1 {
  struct thinkthen_complete_utf8_v1 kind;
  struct thinkthen_complete_session_judgment_choice_field_value_presence_v1 value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_session_judgment_choice_v1;

typedef struct thinkthen_complete_session_judgment_score_v1 {
  struct thinkthen_complete_utf8_v1 kind;
  double value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_session_judgment_score_v1;

typedef struct thinkthen_complete_session_judgment_tags_field_value_v1 {
  const struct thinkthen_complete_utf8_v1 *data;
  size_t len;
} thinkthen_complete_session_judgment_tags_field_value_v1;

typedef struct thinkthen_complete_session_judgment_tags_v1 {
  struct thinkthen_complete_utf8_v1 kind;
  struct thinkthen_complete_session_judgment_tags_field_value_v1 value;
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

typedef struct thinkthen_complete_annotation_value_decision_field_value_presence_v1 {
  uint32_t presence;
  uint32_t value;
} thinkthen_complete_annotation_value_decision_field_value_presence_v1;

typedef struct thinkthen_complete_annotation_value_decision_v1 {
  struct thinkthen_complete_utf8_v1 kind;
  struct thinkthen_complete_annotation_value_decision_field_value_presence_v1 value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_annotation_value_decision_v1;

typedef struct thinkthen_complete_annotation_value_choice_field_value_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_utf8_v1 value;
} thinkthen_complete_annotation_value_choice_field_value_presence_v1;

typedef struct thinkthen_complete_annotation_value_choice_v1 {
  struct thinkthen_complete_utf8_v1 kind;
  struct thinkthen_complete_annotation_value_choice_field_value_presence_v1 value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_annotation_value_choice_v1;

typedef struct thinkthen_complete_annotation_value_score_v1 {
  struct thinkthen_complete_utf8_v1 kind;
  double value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_annotation_value_score_v1;

typedef struct thinkthen_complete_annotation_value_tags_field_value_v1 {
  const struct thinkthen_complete_utf8_v1 *data;
  size_t len;
} thinkthen_complete_annotation_value_tags_field_value_v1;

typedef struct thinkthen_complete_annotation_value_tags_v1 {
  struct thinkthen_complete_utf8_v1 kind;
  struct thinkthen_complete_annotation_value_tags_field_value_v1 value;
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

typedef struct thinkthen_complete_session_observed_row_annotated_field_value_v1 {
  const struct thinkthen_complete_session_annotation_v1 *const *data;
  size_t len;
} thinkthen_complete_session_observed_row_annotated_field_value_v1;

typedef struct thinkthen_complete_session_observed_row_annotated_v1 {
  struct thinkthen_complete_utf8_v1 kind;
  struct thinkthen_complete_session_observed_row_annotated_field_value_v1 value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_session_observed_row_annotated_v1;

typedef struct thinkthen_complete_session_recognition_field_entities_v1 {
  const struct thinkthen_complete_entity_v1 *const *data;
  size_t len;
} thinkthen_complete_session_recognition_field_entities_v1;

typedef struct thinkthen_complete_session_recognition_field_proposals_v1 {
  const struct thinkthen_complete_boundary_proposal_v1 *const *data;
  size_t len;
} thinkthen_complete_session_recognition_field_proposals_v1;

typedef struct thinkthen_complete_session_recognition_field_proposals_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_session_recognition_field_proposals_v1 value;
} thinkthen_complete_session_recognition_field_proposals_presence_v1;

typedef struct thinkthen_complete_recognition_edge_document_v1 {
  uint32_t either;
  double probability;
  struct thinkthen_complete_utf8_v1 relation;
  const struct thinkthen_complete_entity_v1 *source;
  const struct thinkthen_complete_entity_v1 *target;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_recognition_edge_document_v1;

typedef struct thinkthen_complete_session_recognition_field_relations_v1 {
  const struct thinkthen_complete_recognition_edge_document_v1 *const *data;
  size_t len;
} thinkthen_complete_session_recognition_field_relations_v1;

typedef struct thinkthen_complete_session_recognition_field_relations_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_session_recognition_field_relations_v1 value;
} thinkthen_complete_session_recognition_field_relations_presence_v1;

typedef struct thinkthen_complete_session_recognition_v1 {
  struct thinkthen_complete_session_recognition_field_entities_v1 entities;
  const struct thinkthen_complete_recognition_mode_v1 *mode;
  struct thinkthen_complete_session_recognition_field_proposals_presence_v1 proposals;
  struct thinkthen_complete_session_recognition_field_relations_presence_v1 relations;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_session_recognition_v1;

typedef struct thinkthen_complete_session_observed_row_recognized_v1 {
  struct thinkthen_complete_utf8_v1 kind;
  const struct thinkthen_complete_session_recognition_v1 *value;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_session_observed_row_recognized_v1;

typedef struct thinkthen_complete_session_observed_row_find_field_value_presence_v1 {
  uint32_t presence;
  uint64_t value;
} thinkthen_complete_session_observed_row_find_field_value_presence_v1;

typedef struct thinkthen_complete_session_observed_row_find_v1 {
  struct thinkthen_complete_utf8_v1 kind;
  struct thinkthen_complete_session_observed_row_find_field_value_presence_v1 value;
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

typedef struct thinkthen_complete_session_observed_row_relations_field_value_v1 {
  const struct thinkthen_complete_session_relation_edge_v1 *const *data;
  size_t len;
} thinkthen_complete_session_observed_row_relations_field_value_v1;

typedef struct thinkthen_complete_session_observed_row_relations_v1 {
  struct thinkthen_complete_utf8_v1 kind;
  struct thinkthen_complete_session_observed_row_relations_field_value_v1 value;
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

typedef struct thinkthen_complete_facts_field_attempts_v1 {
  const struct thinkthen_complete_attempt_v1 *const *data;
  size_t len;
} thinkthen_complete_facts_field_attempts_v1;

typedef struct thinkthen_complete_facts_field_attempts_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_facts_field_attempts_v1 value;
} thinkthen_complete_facts_field_attempts_presence_v1;

typedef struct thinkthen_complete_call_id_v1 {
  struct thinkthen_complete_utf8_v1 value;
} thinkthen_complete_call_id_v1;

typedef struct thinkthen_complete_facts_field_estimated_cost_usd_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_utf8_v1 value;
} thinkthen_complete_facts_field_estimated_cost_usd_presence_v1;

typedef struct thinkthen_complete_facts_field_held_model_mismatch_presence_v1 {
  uint32_t presence;
  uint32_t value;
} thinkthen_complete_facts_field_held_model_mismatch_presence_v1;

typedef struct thinkthen_complete_facts_field_input_tokens_presence_v1 {
  uint32_t presence;
  uint64_t value;
} thinkthen_complete_facts_field_input_tokens_presence_v1;

typedef struct thinkthen_complete_facts_field_largest_request_estimated_input_tokens_presence_v1 {
  uint32_t presence;
  uint64_t value;
} thinkthen_complete_facts_field_largest_request_estimated_input_tokens_presence_v1;

typedef struct thinkthen_complete_facts_field_model_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_utf8_v1 value;
} thinkthen_complete_facts_field_model_presence_v1;

typedef struct thinkthen_complete_facts_field_output_tokens_presence_v1 {
  uint32_t presence;
  uint64_t value;
} thinkthen_complete_facts_field_output_tokens_presence_v1;

typedef struct thinkthen_complete_persistence_observation_field_advice_presence_v1 {
  uint32_t presence;
  struct thinkthen_complete_utf8_v1 value;
} thinkthen_complete_persistence_observation_field_advice_presence_v1;

typedef struct thinkthen_complete_persistence_observation_v1 {
  struct thinkthen_complete_persistence_observation_field_advice_presence_v1 advice;
  struct thinkthen_complete_utf8_v1 observed_at;
  const struct thinkthen_complete_usage_persistence_v1 *state;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_persistence_observation_v1;

typedef struct thinkthen_complete_facts_field_usage_persistence_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_persistence_observation_v1 *value;
} thinkthen_complete_facts_field_usage_persistence_presence_v1;

typedef struct thinkthen_complete_facts_v1 {
  struct thinkthen_complete_facts_field_attempts_presence_v1 attempts;
  uint64_t cache_answers;
  const struct thinkthen_complete_call_id_v1 *call_id;
  struct thinkthen_complete_facts_field_estimated_cost_usd_presence_v1 estimated_cost_usd;
  struct thinkthen_complete_facts_field_held_model_mismatch_presence_v1 held_model_mismatch;
  struct thinkthen_complete_facts_field_input_tokens_presence_v1 input_tokens;
  uint64_t largest_request_bytes;
  struct thinkthen_complete_facts_field_largest_request_estimated_input_tokens_presence_v1 largest_request_estimated_input_tokens;
  struct thinkthen_complete_facts_field_model_presence_v1 model;
  struct thinkthen_complete_facts_field_output_tokens_presence_v1 output_tokens;
  uint64_t records;
  uint64_t requests_sent;
  double seconds;
  struct thinkthen_complete_utf8_v1 token_estimate_method;
  struct thinkthen_complete_facts_field_usage_persistence_presence_v1 usage_persistence;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_facts_v1;

typedef struct thinkthen_complete_session_packet_terminal_field_facts_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_facts_v1 *value;
} thinkthen_complete_session_packet_terminal_field_facts_presence_v1;

typedef struct thinkthen_complete_estimated_input_denial_initial_request_v1 {
  struct thinkthen_complete_utf8_v1 kind;
  uint64_t limit;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_estimated_input_denial_initial_request_v1;

typedef struct thinkthen_complete_estimated_input_denial_additional_request_v1 {
  struct thinkthen_complete_utf8_v1 kind;
  uint64_t limit;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_estimated_input_denial_additional_request_v1;

typedef struct thinkthen_complete_estimated_input_denial_retry_v1 {
  struct thinkthen_complete_utf8_v1 kind;
  uint64_t last_status;
  uint64_t limit;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_estimated_input_denial_retry_v1;

typedef union thinkthen_complete_estimated_input_denial_data_v1 {
  const struct thinkthen_complete_estimated_input_denial_initial_request_v1 *initial_request;
  const struct thinkthen_complete_estimated_input_denial_additional_request_v1 *additional_request;
  const struct thinkthen_complete_estimated_input_denial_retry_v1 *retry;
} thinkthen_complete_estimated_input_denial_data_v1;

typedef struct thinkthen_complete_estimated_input_denial_v1 {
  uint32_t kind;
  union thinkthen_complete_estimated_input_denial_data_v1 data;
} thinkthen_complete_estimated_input_denial_v1;

typedef struct thinkthen_complete_error_field_estimated_input_denial_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_estimated_input_denial_v1 *value;
} thinkthen_complete_error_field_estimated_input_denial_presence_v1;

typedef struct thinkthen_complete_failure_kind_v1 {
  uint32_t kind;
} thinkthen_complete_failure_kind_v1;

typedef struct thinkthen_complete_send_budget_denial_before_first_send_v1 {
  struct thinkthen_complete_utf8_v1 kind;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_send_budget_denial_before_first_send_v1;

typedef struct thinkthen_complete_send_budget_denial_before_additional_send_v1 {
  struct thinkthen_complete_utf8_v1 kind;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_send_budget_denial_before_additional_send_v1;

typedef struct thinkthen_complete_send_budget_denial_before_retry_v1 {
  struct thinkthen_complete_utf8_v1 kind;
  uint64_t last_status;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_send_budget_denial_before_retry_v1;

typedef union thinkthen_complete_send_budget_denial_data_v1 {
  const struct thinkthen_complete_send_budget_denial_before_first_send_v1 *before_first_send;
  const struct thinkthen_complete_send_budget_denial_before_additional_send_v1 *before_additional_send;
  const struct thinkthen_complete_send_budget_denial_before_retry_v1 *before_retry;
} thinkthen_complete_send_budget_denial_data_v1;

typedef struct thinkthen_complete_send_budget_denial_v1 {
  uint32_t kind;
  union thinkthen_complete_send_budget_denial_data_v1 data;
} thinkthen_complete_send_budget_denial_v1;

typedef struct thinkthen_complete_error_field_send_budget_denial_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_send_budget_denial_v1 *value;
} thinkthen_complete_error_field_send_budget_denial_presence_v1;

typedef struct thinkthen_complete_stopped_field_at_presence_v1 {
  uint32_t presence;
  uint64_t value;
} thinkthen_complete_stopped_field_at_presence_v1;

typedef struct thinkthen_complete_stop_cause_v1 {
  uint32_t kind;
} thinkthen_complete_stop_cause_v1;

typedef struct thinkthen_complete_stopped_field_status_presence_v1 {
  uint32_t presence;
  uint64_t value;
} thinkthen_complete_stopped_field_status_presence_v1;

typedef struct thinkthen_complete_stopped_v1 {
  struct thinkthen_complete_stopped_field_at_presence_v1 at;
  const struct thinkthen_complete_stop_cause_v1 *cause;
  uint32_t retryable;
  struct thinkthen_complete_stopped_field_status_presence_v1 status;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_stopped_v1;

typedef struct thinkthen_complete_error_v1 {
  struct thinkthen_complete_error_field_estimated_input_denial_presence_v1 estimated_input_denial;
  const struct thinkthen_complete_failure_kind_v1 *kind;
  struct thinkthen_complete_utf8_v1 message;
  uint32_t retryable;
  struct thinkthen_complete_error_field_send_budget_denial_presence_v1 send_budget_denial;
  const struct thinkthen_complete_stopped_v1 *stopped;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_error_v1;

typedef struct thinkthen_complete_call_error_field_facts_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_facts_v1 *value;
} thinkthen_complete_call_error_field_facts_presence_v1;

typedef struct thinkthen_complete_call_error_v1 {
  const struct thinkthen_complete_error_v1 *error;
  struct thinkthen_complete_call_error_field_facts_presence_v1 facts;
  struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_call_error_v1;

typedef struct thinkthen_complete_session_packet_terminal_field_failure_presence_v1 {
  uint32_t presence;
  const struct thinkthen_complete_call_error_v1 *value;
} thinkthen_complete_session_packet_terminal_field_failure_presence_v1;

typedef struct thinkthen_complete_session_packet_terminal_v1 {
  struct thinkthen_complete_session_packet_terminal_field_facts_presence_v1 facts;
  struct thinkthen_complete_session_packet_terminal_field_failure_presence_v1 failure;
  struct thinkthen_complete_utf8_v1 kind;
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

/*
 C descriptor or borrowed view `SourceSpecV1`.
 */
typedef struct thinkthen_source_spec_v1 {
  /*
   C field `paths`.
   */
  struct thinkthen_strings_v1 paths;
  /*
   C field `unit`.
   */
  uint32_t unit;
  /*
   C field `window`.
   */
  size_t window;
} thinkthen_source_spec_v1;

/*
 C descriptor or borrowed view `ImagesV1`.
 */
typedef struct thinkthen_images_v1 {
  /*
   C field `data`.
   */
  const struct thinkthen_image *const *data;
  /*
   C field `len`.
   */
  size_t len;
} thinkthen_images_v1;

/*
 C descriptor or borrowed view `RecordV1`.
 */
typedef struct thinkthen_record_v1 {
  /*
   C field `original`.
   */
  struct thinkthen_optional_content_v1 original;
  /*
   C field `context`.
   */
  struct thinkthen_optional_content_v1 context;
  /*
   C field `options`.
   */
  struct thinkthen_choices_v1 options;
  /*
   C field `images`.
   */
  struct thinkthen_images_v1 images;
} thinkthen_record_v1;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

/*
 Start an owned same-thread native batch without collecting its source.
 # Safety
 The installed header specifies counted storage, engine and same-thread lifetimes.
 */
int thinkthen_annotate_batch_start(const struct thinkthen_engine *e,
                                   const struct thinkthen_question *q,
                                   const struct thinkthen_source *s,
                                   const struct thinkthen_controls_v1 *c,
                                   struct thinkthen_batch **out);

/*
 Execute the named judgment with typed inputs and immutable complete results.
 # Safety
 Every pointer obeys include/thinkthen.h's counted-storage and lifetime rules.
 */
int thinkthen_annotate_complete(const struct thinkthen_engine *engine,
                                const struct thinkthen_question *question,
                                const struct thinkthen_source *source,
                                const struct thinkthen_controls_v1 *controls,
                                struct thinkthen_result **out);

/*
 Snapshot joined final facts; before termination return Usage without writing output.
 # Safety
 Owner/output follow the installed lifetime contract.
 */
int thinkthen_batch_facts(const struct thinkthen_batch *owner, struct thinkthen_result **out);

/*
 Drop the native batch and join its workers before releasing owned input backing.
 # Safety
 NULL or a live batch on its creating thread, freed exactly once; engine remains live.
 */
void thinkthen_batch_free(struct thinkthen_batch *owner);

/*
 Yield one independently owned result, or NULL on exhaustion, then one terminal error.
 # Safety
 Batch and output obey the installed same-thread/engine lifetime contract.
 */
int thinkthen_batch_next(struct thinkthen_batch *owner, struct thinkthen_result **out);

/*
 Call `thinkthen_call_opts` without a deadline or cancellation token.
 # Safety
 All pointers obey the corresponding options form's contract.
 The JSON door with no budget and no token: exactly
 `thinkthen_call_opts` with THINKTHEN_NO_DEADLINE and a null token.
 */
char *thinkthen_call(const struct thinkthen_engine *engine, const char *request_json);

/*
 The JSON door: one request, its answer as JSON text, or null.

 # Safety

 Every pointer follows the header's argument rules.
 Read one NUL-terminated UTF-8 JSON request; return owned answer JSON,
 freed once with thinkthen_free_string. Envelope/result schemas: DESIGN.md.
 Asking success returns {"value":VALUE,"facts":FACTS}; usage returns direct
 counters and takes no options. NULL means failure with no partial value;
 error_code/message/retryable and error_facts_json describe that failure.
 */
char *thinkthen_call_opts(const struct thinkthen_engine *engine,
                          const char *request_json,
                          int64_t deadline_ms,
                          thinkthen_cancel_token *cancel);

/*
 Fire a token from any thread; null is ignored.

 # Safety

 `token` is null or a live token.
 Fire a token: the calls carrying it start no new request or retry, let
 the requests they sent finish, and return THINKTHEN_ECANCELLED with no
 results, even when a sent request's reply arrives after the fire. A
 token is one-shot: a fire leaves it fired, a second fire is ignored,
 and no call re-arms it. Thread-safe from any thread, and it allocates
 nothing; a null token is accepted and ignored.
 */
void thinkthen_cancel(thinkthen_cancel_token *token);

/*
 Free a token; null is ignored.

 # Safety

 `token` is null or a live token no call is carrying.
 Free a token. NULL is accepted and ignored.
 */
void thinkthen_cancel_token_free(thinkthen_cancel_token *token);

/*
 Create a cancel token.
 Create a cancel token.
 */
thinkthen_cancel_token *thinkthen_cancel_token_new(void);

/*
 Start an owned same-thread native batch without collecting its source.
 # Safety
 The installed header specifies counted storage, engine and same-thread lifetimes.
 */
int thinkthen_choose_batch_start(const struct thinkthen_engine *e,
                                 const struct thinkthen_question *q,
                                 const struct thinkthen_source *s,
                                 const struct thinkthen_controls_v1 *c,
                                 struct thinkthen_batch **out);

/*
 Execute the named judgment with typed inputs and immutable complete results.
 # Safety
 Every pointer obeys include/thinkthen.h's counted-storage and lifetime rules.
 */
int thinkthen_choose_complete(const struct thinkthen_engine *engine,
                              const struct thinkthen_question *question,
                              const struct thinkthen_source *source,
                              const struct thinkthen_controls_v1 *controls,
                              struct thinkthen_result **out);

/*
 Call `thinkthen_decide_opts` without a deadline or cancellation token.
 # Safety
 All pointers obey the corresponding options form's contract.
 Ask one yes-or-no question of one text: exactly `thinkthen_decide_opts`
 with THINKTHEN_NO_DEADLINE and a null token. `question_json` is one
 decide question in the question-file grammar, or the bare text of a
 decide question at the cut of one half; `text` and `text_len` are the
 evidence. The judgment lands in `out` on THINKTHEN_OK.
 */
int thinkthen_decide(const struct thinkthen_engine *engine,
                     const char *question_json,
                     const char *text,
                     size_t text_len,
                     struct thinkthen_answer *out);

/*
 Start an owned same-thread native batch without collecting its source.
 # Safety
 The installed header specifies counted storage, engine and same-thread lifetimes.
 Lazy record batches use the existing native scheduler for decide, choose,
 tag, score, filter and annotate. Start clones question, source selection,
 shared context and cancellation flag; callers may free question/source/token
 after start. The engine must remain live through batch_free. Start, next,
 facts and free must run on the creating thread with exclusive batch access.
 Ordinary packing and real input pauses determine stages; there are no caller
 stage markers. A source_records snapshot retains finite source bytes, while
 source_files pulls the native reader lazily. Reader unit 5 explicitly reads
 JSONL as typed JSON; ordinary text units never infer JSON from text.
 next returns OK with one independently owned typed result, or OK with NULL
 after exhaustion. Completed rows precede one terminal error; that error
 leaves out unchanged, then next returns OK with NULL. Returned row results
 outlive the batch, question, source and engine. Per-row summary facts are
 absent while work continues; batch_facts returns an owned final summary
 after exhaustion/error, and EUSAGE before termination without writing out.
 batch_free stops and joins native workers before releasing owned backing.
 The existing eager complete calls keep their all-before-send admission rule.
 Rank, find, recognize and relate retain their native aggregate semantics
 through the complete calls; they have no invented lazy batch interface.
 */
int thinkthen_decide_batch_start(const struct thinkthen_engine *e,
                                 const struct thinkthen_question *q,
                                 const struct thinkthen_source *s,
                                 const struct thinkthen_controls_v1 *c,
                                 struct thinkthen_batch **out);

/*
 Execute the named judgment with typed inputs and immutable complete results.
 # Safety
 Every pointer obeys include/thinkthen.h's counted-storage and lifetime rules.
 */
int thinkthen_decide_complete(const struct thinkthen_engine *engine,
                              const struct thinkthen_question *question,
                              const struct thinkthen_source *source,
                              const struct thinkthen_controls_v1 *controls,
                              struct thinkthen_result **out);

/*
 Call `thinkthen_decide_many_opts` without a deadline or cancellation token.
 # Safety
 All pointers obey the corresponding options form's contract.
 Bulk decide keeps judgments in input order at the engine throttle.
 Equivalent to decide_many_opts with THINKTHEN_NO_DEADLINE and a NULL token.
 texts and lengths have count pointers/byte lengths; out has count answers.
 Inputs are borrowed for the call, and failure changes no output.
 */
int thinkthen_decide_many(const struct thinkthen_engine *engine,
                          const char *question_json,
                          const char *const *texts,
                          const size_t *lengths,
                          size_t count,
                          struct thinkthen_answer *out);

/*
 One question over every text, in input order; all rows or none.

 # Safety

 Every pointer follows the header's argument rules.
 The same bulk call with the options beside it.
 */
int thinkthen_decide_many_opts(const struct thinkthen_engine *engine,
                               const char *question_json,
                               const char *const *texts,
                               const size_t *lengths,
                               size_t count,
                               int64_t deadline_ms,
                               thinkthen_cancel_token *cancel,
                               struct thinkthen_answer *out);

/*
 Call `thinkthen_decide_many_with_facts_opts` without a deadline or cancellation token.
 # Safety
 All pointers obey the corresponding options form's contract.
 */
int thinkthen_decide_many_with_facts(const struct thinkthen_engine *engine,
                                     const char *question_json,
                                     const char *const *texts,
                                     const size_t *lengths,
                                     size_t count,
                                     struct thinkthen_answer *out,
                                     char **facts_json,
                                     size_t *facts_len);

/*
 Decide in input order and return final owned batch facts.

 # Safety
 Pointers and output storage obey the C header's rules.
 */
int thinkthen_decide_many_with_facts_opts(const struct thinkthen_engine *engine,
                                          const char *question_json,
                                          const char *const *texts,
                                          const size_t *lengths,
                                          size_t count,
                                          int64_t deadline_ms,
                                          thinkthen_cancel_token *cancel,
                                          struct thinkthen_answer *out,
                                          char **facts_json,
                                          size_t *facts_len);

/*
 One yes-or-no question over one text; the judgment lands in `out`.

 # Safety

 Every pointer follows the header's argument rules.
 The same call with the options beside it: `deadline_ms` is the budget
 and `cancel` is the token, documented on THINKTHEN_NO_DEADLINE.
 */
int thinkthen_decide_opts(const struct thinkthen_engine *engine,
                          const char *question_json,
                          const char *text,
                          size_t text_len,
                          int64_t deadline_ms,
                          thinkthen_cancel_token *cancel,
                          struct thinkthen_answer *out);

/*
 Call `thinkthen_decide_with_facts_opts` without a deadline or cancellation token.
 # Safety
 All pointers obey the corresponding options form's contract.
 Compatibility typed forms: each successful call owns final facts JSON beside
 its result. The facts object contains records, requests_sent, cache_answers,
 seconds, and optional input_tokens, output_tokens, and model. Free each
 returned JSON string with thinkthen_free_string. The original decide, decide_many, recognize and relate
 forms remain ABI-compatible bare-result forms; they do not return facts.
 A nonzero code changes no output slot. A started failure's facts remain
 available from thinkthen_error_facts_json under its borrowed lifetime.
 All output slots must be nonnull (except the zero-count answer array) and
 must not share an address. Counts times pointer, size_t, and answer sizes,
 and every text length, must fit PTRDIFF_MAX. The caller supplies live,
 aligned, adequately sized, nonoverlapping input and output storage.
 */
int thinkthen_decide_with_facts(const struct thinkthen_engine *engine,
                                const char *question_json,
                                const char *evidence,
                                size_t evidence_len,
                                struct thinkthen_answer *out,
                                char **facts_json,
                                size_t *facts_len);

/*
 Decide and return an owned facts JSON string.

 # Safety
 Pointers and output storage obey the C header's rules.
 */
int thinkthen_decide_with_facts_opts(const struct thinkthen_engine *engine,
                                     const char *question_json,
                                     const char *evidence,
                                     size_t evidence_len,
                                     int64_t deadline_ms,
                                     thinkthen_cancel_token *cancel,
                                     struct thinkthen_answer *out,
                                     char **facts_json,
                                     size_t *facts_len);

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
 Build with a UTF-8 JSON settings object; null uses the environment.

 # Safety

 `settings_json` is null or a live NUL-terminated string.
 Build from the environment plus a closed UTF-8 settings object; NULL/{}
 uses environment alone. Keys/types and token accounting: DESIGN.md.
 No key is accepted. Unknown/repeated keys or bad types return NULL/EUSAGE
 in the calling thread's null-engine slot. A named backend captures its key;
 an explicit base_url receives that key. Building sends nothing.
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
 Clone the calling thread's saved failure without altering its sticky error slot.
 # Safety
 Engine is NULL or live; out is writable when nonnull.
 Snapshot the calling thread's last failure for engine, or its failed-build
 slot for engine=NULL. Never clears/replaces that slot.
 With no saved failure: returns OK and writes *out=NULL (no allocation).
 With a pre-start failure: returns OK and writes an owned FAILURE result;
 facts/attempts are absent. With a started failure: returns OK and writes an
 owned FAILURE result with final facts and opt-in attempts, even if []
 because no send occurred. Both failure snapshots have absent schema,
 answer_id and function, count=observation_count=0, and meta.present=0.
 error is present; no origin/model/request/answer provenance is invented.
 The return value describes snapshot creation, not the saved failure code.
 out=NULL returns EUSAGE without changing the saved failure.
 */
int thinkthen_error_complete(const struct thinkthen_engine *engine, struct thinkthen_result **out);

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
 Start an owned same-thread native batch without collecting its source.
 # Safety
 The installed header specifies counted storage, engine and same-thread lifetimes.
 */
int thinkthen_filter_batch_start(const struct thinkthen_engine *e,
                                 const struct thinkthen_question *q,
                                 const struct thinkthen_source *s,
                                 const struct thinkthen_controls_v1 *c,
                                 struct thinkthen_batch **out);

/*
 Execute the named judgment with typed inputs and immutable complete results.
 # Safety
 Every pointer obeys include/thinkthen.h's counted-storage and lifetime rules.
 */
int thinkthen_filter_complete(const struct thinkthen_engine *engine,
                              const struct thinkthen_question *question,
                              const struct thinkthen_source *source,
                              const struct thinkthen_controls_v1 *controls,
                              struct thinkthen_result **out);

/*
 Execute the named judgment with typed inputs and immutable complete results.
 # Safety
 Every pointer obeys include/thinkthen.h's counted-storage and lifetime rules.
 */
int thinkthen_find_complete(const struct thinkthen_engine *engine,
                            const struct thinkthen_question *question,
                            const struct thinkthen_source *source,
                            const struct thinkthen_controls_v1 *controls,
                            struct thinkthen_result **out);

/*
 Free a string the door returned; null is ignored.

 # Safety

 `text` is null or a string the door returned and has not freed.
 Free a string `thinkthen_call`, `thinkthen_recognize`, or
 `thinkthen_relate` returned, or their `_opts` twins. NULL is accepted
 and ignored.
 */
void thinkthen_free_string(char *text);

/*
 Validate and clone compressed image bytes and an optional UTF-8 filename.
 # Safety
 Counted storage is readable and output writable until return.
 */
int thinkthen_image_clone(const struct thinkthen_engine *engine,
                          const uint8_t *bytes,
                          size_t len,
                          uint32_t media,
                          struct thinkthen_optional_string_v1 filename,
                          struct thinkthen_image **out);

/*
 Free an owned image; NULL is harmless.
 # Safety
 A nonnull image is live, freed once, after all borrowers finish.
 */
void thinkthen_image_free(struct thinkthen_image *owner);

/*
 Borrow original bytes, dimensions and filename from an immutable image owner.
 # Safety
 Owner is live throughout the view's use; output is writable.
 */
int thinkthen_image_view(const struct thinkthen_image *owner, struct thinkthen_image_view_v1 *out);

/*
 Preview one `thinkthen.plan-input/1` object as `plan` JSON text.

 # Safety

 Every pointer follows the header's argument rules.
 Preview a closed thinkthen.plan-input/1 object without key/cache reads
 or sends. Grammar and plan fields: DESIGN.md. Success owns NUL-terminated
 plan JSON in out and its byte length; free once with thinkthen_free_string.
 Unknown/repeated fields, bad verb/input/settings, conflicting question
 settings or NULL pointers return EUSAGE with both outputs unchanged.
 */
int thinkthen_plan_json(const struct thinkthen_engine *engine,
                        const char *plan_json,
                        char **out,
                        size_t *out_len);

/*
 Borrow the native author snapshot owned by this question.
 # Safety
 Question and output obey the header's lifetime/storage contract.
 Borrow metadata owned by this immutable question until question_free.
 */
int thinkthen_question_author(const struct thinkthen_question *owner,
                              struct thinkthen_question_author_v1 *out);

/*
 Read one named question into an owned validated JSON string.

 # Safety

 Every pointer follows the header's argument and lifetime rules.
 Read at most 1 MiB of named UTF-8 single-question JSON without sending.
 Success owns NUL-terminated original JSON in out and its byte length;
 free it once with thinkthen_free_string. NULL/bad UTF-8 path or output is
 EUSAGE; unreadable/overlarge/non-UTF-8/malformed files are ELOCAL,
 non-retryable. Failure leaves both outputs unchanged and discloses neither
 path nor contents. Grammar and usage examples: DESIGN.md.
 */
int thinkthen_question_file(const struct thinkthen_engine *engine,
                            const char *path,
                            char **out,
                            size_t *out_len);

/*
 Free the owned handle; NULL is harmless.
 # Safety
 A nonnull handle is live, freed once, with no concurrent borrowers.
 */
void thinkthen_question_free(struct thinkthen_question *owner);

/*
 Load a bounded native question or question-set file.
 # Safety
 All pointers obey the installed header's storage and lifetime contract.
 */
int thinkthen_question_load(const struct thinkthen_engine *engine,
                            struct thinkthen_string_v1 path,
                            struct thinkthen_question **out);

/*
 Load a named question through the selected native role's loader.
 # Safety
 All pointers follow include/thinkthen.h's storage and lifetime contract.
 */
int thinkthen_question_load_named(const struct thinkthen_engine *engine,
                                  uint32_t role,
                                  struct thinkthen_string_v1 name,
                                  struct thinkthen_question **out);

/*
 Load an explicit reference through the selected native role's loader.
 # Safety
 All pointers follow include/thinkthen.h's storage and lifetime contract.
 */
int thinkthen_question_load_reference(const struct thinkthen_engine *engine,
                                      uint32_t role,
                                      struct thinkthen_string_v1 reference,
                                      struct thinkthen_question **out);

/*
 Clone an admitted native question from counted descriptors.
 # Safety
 All pointers obey the installed header's storage and lifetime contract.
 Unpublished 0426 integration carriers, pending whole-ticket qualification.
 Complete calls use the existing native engine and immutable owned results.
 Constructors clone caller buffers and referenced question/image values.
 Image views borrow immutable image memory until image_free.
 Frees accept NULL; nonnull handles must be live and freed exactly once
 after all borrowers finish. Forged, stale and concurrently freed handles
 violate this contract. Null required pointers and malformed descriptors
 return EUSAGE. Every nonzero return leaves every output unchanged.
 */
int thinkthen_question_new(const struct thinkthen_engine *engine,
                           const struct thinkthen_question_spec_v1 *spec,
                           struct thinkthen_question **out);

/*
 Construct through native grammar with additive author metadata.
 # Safety
 All pointers follow include/thinkthen.h's storage and lifetime contract.
 Construct through the same native grammar, with separately counted author
 metadata. author=NULL means no author metadata. All inputs are cloned.
 */
int thinkthen_question_new_authored(const struct thinkthen_engine *engine,
                                    const struct thinkthen_question_spec_v1 *spec,
                                    const struct thinkthen_question_author_v1 *author,
                                    struct thinkthen_question **out);

/*
 Construct a recognition question with copied task wording.
 # Safety
 All active counted buffers, optional author and output obey the header contract.
 */
int thinkthen_question_new_recognition_v1(const struct thinkthen_engine *engine,
                                          const struct thinkthen_question_spec_v1 *spec,
                                          const struct thinkthen_question_author_v1 *metadata,
                                          const struct thinkthen_recognition_task_v1 *task,
                                          struct thinkthen_question **out);

/*
 Import counted saved-question grammar through an explicitly selected native parser.
 # Safety
 Counted bytes and output obey the header's storage contract.
 Import saved question-file JSON through an explicit native grammar role.
 This imports a question only; judgment calls and result fields remain typed.
 Inline grammar failures return EUSAGE; no role guessing or parser fallback.
 The immutable handle owns every byte independently of json's lifetime.
 */
int thinkthen_question_parse(const struct thinkthen_engine *engine,
                             uint32_t role,
                             struct thinkthen_string_v1 json,
                             struct thinkthen_question **out);

/*
 Borrow authored task wording from an immutable live recognition question.
 # Safety
 Owner and writable output remain live with no concurrent destruction.
 */
int thinkthen_question_recognition_task_v1(const struct thinkthen_question *owner,
                                           struct thinkthen_recognition_task_v1 *out);

/*
 Execute the named judgment with typed inputs and immutable complete results.
 # Safety
 Every pointer obeys include/thinkthen.h's counted-storage and lifetime rules.
 */
int thinkthen_rank_complete(const struct thinkthen_engine *engine,
                            const struct thinkthen_question *question,
                            const struct thinkthen_source *source,
                            const struct thinkthen_controls_v1 *controls,
                            struct thinkthen_result **out);

/*
 Call `thinkthen_recognize_opts` without a deadline or cancellation token.
 # Safety
 All pointers obey the corresponding options form's contract.
 Recognize a text using version-one recognize question JSON; follow cache.
 Equivalent to recognize_opts with THINKTHEN_NO_DEADLINE and a NULL token.
 Success owns {"entities": [...], "relations": [...]} JSON in out and its
 byte length in out_len; free with thinkthen_free_string. Entity offsets
 count code points. Failure returns its kind and leaves outputs unchanged.
 Question/result fields and default kind: DESIGN.md.
 */
int thinkthen_recognize(const struct thinkthen_engine *engine,
                        const char *spec_json,
                        const char *text,
                        size_t text_len,
                        char **out,
                        size_t *out_len);

/*
 Execute the named judgment with typed inputs and immutable complete results.
 # Safety
 Every pointer obeys include/thinkthen.h's counted-storage and lifetime rules.
 */
int thinkthen_recognize_complete(const struct thinkthen_engine *engine,
                                 const struct thinkthen_question *question,
                                 const struct thinkthen_source *source,
                                 const struct thinkthen_controls_v1 *controls,
                                 struct thinkthen_result **out);

/*
 Every name in one text and the relations the rules allow, as JSON.

 # Safety

 Every pointer follows the header's argument rules.
 The same call with the options beside it.
 */
int thinkthen_recognize_opts(const struct thinkthen_engine *engine,
                             const char *spec_json,
                             const char *text,
                             size_t text_len,
                             int64_t deadline_ms,
                             thinkthen_cancel_token *cancel,
                             char **out,
                             size_t *out_len);

/*
 Call `thinkthen_recognize_with_facts_opts` without a deadline or cancellation token.
 # Safety
 All pointers obey the corresponding options form's contract.
 */
int thinkthen_recognize_with_facts(const struct thinkthen_engine *engine,
                                   const char *spec_json,
                                   const char *evidence,
                                   size_t evidence_len,
                                   char **out,
                                   size_t *out_len,
                                   char **facts_json,
                                   size_t *facts_len);

/*
 Recognize and return owned result and facts JSON strings.

 # Safety
 Pointers and output storage obey the C header's rules.
 */
int thinkthen_recognize_with_facts_opts(const struct thinkthen_engine *engine,
                                        const char *spec_json,
                                        const char *evidence,
                                        size_t evidence_len,
                                        int64_t deadline_ms,
                                        thinkthen_cancel_token *cancel,
                                        char **out,
                                        size_t *out_len,
                                        char **facts_json,
                                        size_t *facts_len);

/*
 Call `thinkthen_relate_opts` without a deadline or cancellation token.
 # Safety
 All pointers obey the corresponding options form's contract.
 Relate count JSON records with counted byte lengths; count>255 is EUSAGE
 before any pointer read. Equivalent to relate_opts with THINKTHEN_NO_DEADLINE/NULL.
 Version-one relate JSON supplies rules; records carry name/kind. Other
 fields pointers are EUSAGE. Success owns {"edges": [...]} JSON in out
 with byte length in out_len; free with thinkthen_free_string. Failure
 returns its kind and leaves both outputs unchanged. Schema: DESIGN.md.
 */
int thinkthen_relate(const struct thinkthen_engine *engine,
                     const char *spec_json,
                     const char *const *texts,
                     const size_t *lengths,
                     size_t count,
                     char **out,
                     size_t *out_len);

/*
 Execute the named judgment with typed inputs and immutable complete results.
 # Safety
 Every pointer obeys include/thinkthen.h's counted-storage and lifetime rules.
 */
int thinkthen_relate_complete(const struct thinkthen_engine *engine,
                              const struct thinkthen_question *question,
                              const struct thinkthen_source *source,
                              const struct thinkthen_controls_v1 *controls,
                              struct thinkthen_result **out);

/*
 How the given records relate, as `{"edges": [...]}`.

 # Safety

 Every pointer follows the header's argument rules.
 The same call with the options beside it.
 */
int thinkthen_relate_opts(const struct thinkthen_engine *engine,
                          const char *spec_json,
                          const char *const *texts,
                          const size_t *lengths,
                          size_t count,
                          int64_t deadline_ms,
                          thinkthen_cancel_token *cancel,
                          char **out,
                          size_t *out_len);

/*
 Call `thinkthen_relate_with_facts_opts` without a deadline or cancellation token.
 # Safety
 All pointers obey the corresponding options form's contract.
 */
int thinkthen_relate_with_facts(const struct thinkthen_engine *engine,
                                const char *spec_json,
                                const char *const *texts,
                                const size_t *lengths,
                                size_t count,
                                char **out,
                                size_t *out_len,
                                char **facts_json,
                                size_t *facts_len);

/*
 Relate and return owned result and facts JSON strings.

 # Safety
 Pointers and output storage obey the C header's rules.
 */
int thinkthen_relate_with_facts_opts(const struct thinkthen_engine *engine,
                                     const char *spec_json,
                                     const char *const *texts,
                                     const size_t *lengths,
                                     size_t count,
                                     int64_t deadline_ms,
                                     thinkthen_cancel_token *cancel,
                                     char **out,
                                     size_t *out_len,
                                     char **facts_json,
                                     size_t *facts_len);

/*
 Preview canonical Request JSON without reading a key or cache or sending.
 Supports fixed atomic decide, choose, tag and score questions only; other functions and dynamic questions refuse. Sessions support all ten judgments.
 Success owns NUL-terminated plan JSON in out and its byte length in out_len;
 free once with thinkthen_free_string. Refusal leaves both outputs unchanged
 and records the safe calling-thread thinkthen_session_error_message.
 # Safety
 engine is live; request_json points to request_len readable bytes (NULL
 requires zero); out and out_len are nonnull writable, nonoverlapping slots.
 Extents fit Rust slices. The engine remains live until this call returns.
 */
int thinkthen_request_plan_json(const struct thinkthen_engine *engine,
                                const char *request_json,
                                size_t request_len,
                                char **out,
                                size_t *out_len);

/*
 Borrow one complete row; wrong functions and out-of-range ordinals return Usage.
 # Safety
 Owner and output obey the header's lifetime/storage contract.
 */
int thinkthen_result_annotate(const struct thinkthen_result *owner,
                              size_t at,
                              struct thinkthen_annotate_view_v1 *out);

/*
 Borrow one complete row; wrong functions and out-of-range ordinals return Usage.
 # Safety
 Owner and output obey the header's lifetime/storage contract.
 */
int thinkthen_result_choose(const struct thinkthen_result *owner,
                            size_t at,
                            struct thinkthen_choose_view_v1 *out);

/*
 Borrow one complete row; wrong functions and out-of-range ordinals return Usage.
 # Safety
 Owner and output obey the header's lifetime/storage contract.
 */
int thinkthen_result_decide(const struct thinkthen_result *owner,
                            size_t at,
                            struct thinkthen_decide_view_v1 *out);

/*
 Borrow additive full row details, including partial usage and original inputs.
 # Safety
 Owner and output obey the header's lifetime/storage contract.
 */
int thinkthen_result_details(const struct thinkthen_result *owner,
                             size_t at,
                             struct thinkthen_details_v1 *out);

/*
 Borrow one complete row; wrong functions and out-of-range ordinals return Usage.
 # Safety
 Owner and output obey the header's lifetime/storage contract.
 */
int thinkthen_result_filter(const struct thinkthen_result *owner,
                            size_t at,
                            struct thinkthen_filter_view_v1 *out);

/*
 Borrow one complete row; wrong functions and out-of-range ordinals return Usage.
 # Safety
 Owner and output obey the header's lifetime/storage contract.
 */
int thinkthen_result_find(const struct thinkthen_result *owner,
                          size_t at,
                          struct thinkthen_find_view_v1 *out);

/*
 Free an immutable result after every borrowed view is finished. NULL is harmless.
 # Safety
 A nonnull result was allocated by this library and has not been freed.
 */
void thinkthen_result_free(struct thinkthen_result *owner);

/*
 Borrow one annotation or rank member's actual authored metadata.
 # Safety
 Owner and output obey the header's lifetime/storage contract.
 */
int thinkthen_result_member_author(const struct thinkthen_result *owner,
                                   size_t at,
                                   size_t member,
                                   struct thinkthen_question_author_v1 *out);

/*
 Borrow an actual question or completed-row observation in native event order.
 # Safety
 Owner and output obey the header's lifetime/storage contract.
 */
int thinkthen_result_observation(const struct thinkthen_result *owner,
                                 size_t at,
                                 struct thinkthen_observation_v1 *out);

/*
 Borrow one native observation's actual authored metadata.
 # Safety
 Owner and output obey the header's lifetime/storage contract.
 */
int thinkthen_result_observation_author(const struct thinkthen_result *owner,
                                        size_t at,
                                        struct thinkthen_question_author_v1 *out);

/*
 Borrow the actual resolved observed question and all its native identities.
 # Safety
 Owner and output obey the header's lifetime/storage contract.
 */
int thinkthen_result_observation_details(const struct thinkthen_result *owner,
                                         size_t at,
                                         struct thinkthen_details_v1 *out);

/*
 Borrow the authored metadata of one complete row.
 # Safety
 Owner and output obey the header's lifetime/storage contract.
 Borrow native author snapshots until result_free. Row/member ordinals
 follow the existing named result accessors and annotation member order.
 Set envelopes and generated internal questions carry absent author fields.
 Invalid owner/output/ordinal or a member on a non-annotation row is Usage
 and leaves output unchanged. Observation includes both question/row events.
 */
int thinkthen_result_question_author(const struct thinkthen_result *owner,
                                     size_t at,
                                     struct thinkthen_question_author_v1 *out);

/*
 Borrow one complete row; wrong functions and out-of-range ordinals return Usage.
 # Safety
 Owner and output obey the header's lifetime/storage contract.
 */
int thinkthen_result_rank(const struct thinkthen_result *owner,
                          size_t at,
                          struct thinkthen_rank_view_v1 *out);

/*
 Borrow an actual saved rank member with its independent identity and probabilities.
 # Safety
 Owner and output obey the installed header's storage contract.
 */
int thinkthen_result_rank_member(const struct thinkthen_result *owner,
                                 size_t row,
                                 size_t member,
                                 struct thinkthen_rank_view_v1 *out);

/*
 Number of saved member judgments; simple rank has none.
 # Safety
 Owner and output obey the installed header's storage contract.
 Saved rank sets retain every ordered member. Simple rank has zero members.
 Each member view has its own answer ID, complete probabilities and metadata.
 Views borrow the result; invalid indices return EUSAGE without writing output.
 */
int thinkthen_result_rank_member_count(const struct thinkthen_result *owner,
                                       size_t row,
                                       size_t *out);

/*
 Borrow full member details, including partial usage and source batch sizes.
 # Safety
 Owner and output obey the installed header's storage contract.
 Full member details retain independently present usage dimensions and source
 batch sizes. Nested views borrow the result until result_free. NULL arguments,
 wrong function, and invalid row/member indices return EUSAGE without writing.
 */
int thinkthen_result_rank_member_details(const struct thinkthen_result *owner,
                                         size_t row,
                                         size_t member,
                                         struct thinkthen_details_v1 *out);

/*
 Borrow one recognition row's copied authored task wording.
 # Safety
 Owner and output obey the header's lifetime and full-sized storage contract.
 */
int thinkthen_result_recognition_task_v1(const struct thinkthen_result *owner,
                                         size_t at,
                                         struct thinkthen_recognition_task_v1 *out);

/*
 Borrow one complete row; wrong functions and out-of-range ordinals return Usage.
 # Safety
 Owner and output obey the header's lifetime/storage contract.
 */
int thinkthen_result_recognize(const struct thinkthen_result *owner,
                               size_t at,
                               struct thinkthen_recognize_view_v1 *out);

/*
 Borrow one complete row; wrong functions and out-of-range ordinals return Usage.
 # Safety
 Owner and output obey the header's lifetime/storage contract.
 */
int thinkthen_result_relate(const struct thinkthen_result *owner,
                            size_t at,
                            struct thinkthen_relate_view_v1 *out);

/*
 Borrow a final row at its output position, retaining its original input index.
 # Safety
 Owner and output obey the header's lifetime/storage contract.
 Borrow a final row by output position, retaining its original input index.
 Nested views borrow until result_free. NULL owner/output or an out-of-range
 position returns EUSAGE without writing output.
 */
int thinkthen_result_row(const struct thinkthen_result *owner,
                         size_t at,
                         struct thinkthen_row_observation_v1 *out);

/*
 Borrow one complete row; wrong functions and out-of-range ordinals return Usage.
 # Safety
 Owner and output obey the header's lifetime/storage contract.
 */
int thinkthen_result_score(const struct thinkthen_result *owner,
                           size_t at,
                           struct thinkthen_score_view_v1 *out);

/*
 Borrow native located recognition spans; absent for unlocated input.
 # Safety
 Owner and output obey the header's lifetime/storage contract.
 */
int thinkthen_result_source_recognition(const struct thinkthen_result *owner,
                                        size_t row,
                                        struct thinkthen_source_recognition_v1 *out);

/*
 Borrow native expanded relation occurrences; absent for unlocated input.
 # Safety
 Owner and output obey the header's lifetime/storage contract.
 */
int thinkthen_result_source_relations(const struct thinkthen_result *owner,
                                      size_t row,
                                      struct thinkthen_source_relations_v1 *out);

/*
 Borrow the immutable complete invocation summary.
 # Safety
 Owner and output obey the header's lifetime/storage contract.
 */
int thinkthen_result_summary(const struct thinkthen_result *owner,
                             struct thinkthen_summary_v1 *out);

/*
 Borrow one complete row; wrong functions and out-of-range ordinals return Usage.
 # Safety
 Owner and output obey the header's lifetime/storage contract.
 */
int thinkthen_result_tag(const struct thinkthen_result *owner,
                         size_t at,
                         struct thinkthen_tag_view_v1 *out);

/*
 Start an owned same-thread native batch without collecting its source.
 # Safety
 The installed header specifies counted storage, engine and same-thread lifetimes.
 */
int thinkthen_score_batch_start(const struct thinkthen_engine *e,
                                const struct thinkthen_question *q,
                                const struct thinkthen_source *s,
                                const struct thinkthen_controls_v1 *c,
                                struct thinkthen_batch **out);

/*
 Execute the named judgment with typed inputs and immutable complete results.
 # Safety
 Every pointer obeys include/thinkthen.h's counted-storage and lifetime rules.
 */
int thinkthen_score_complete(const struct thinkthen_engine *engine,
                             const struct thinkthen_question *question,
                             const struct thinkthen_source *source,
                             const struct thinkthen_controls_v1 *controls,
                             struct thinkthen_result **out);

/*
 Signal cancellation without waiting for a provider or final facts. NULL is ignored.
 # Safety
 session is NULL or live throughout the call.
 */
void thinkthen_session_cancel(struct thinkthen_session *session);

/*
 Borrow a safe UTF-8 immediate diagnostic, never NULL. Success preserves
 it; the next immediate session failure or thread exit ends its validity.
 Execution failures arrive as terminal packets and never change this slot.
 */
const char *thinkthen_session_error_message(void);

/*
 Fix input EOF or a canonical closed reader failure. NULL with zero means EOF;
 other inputs decode kind io, utf8 or invalid_input and optional location.
 No input bytes survive return. A changed finish refuses without changing intake.
 # Safety
 session is live and failure_json has failure_len readable UTF-8 bytes.
 */
int thinkthen_session_finish(struct thinkthen_session *session,
                             const char *failure_json,
                             size_t failure_len);

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
 Clone an explicit native text/image reader selection.
 # Safety
 All pointers obey the installed header's storage and lifetime contract.
 */
int thinkthen_source_files(const struct thinkthen_engine *engine,
                           const struct thinkthen_source_spec_v1 *spec,
                           struct thinkthen_source **out);

/*
 Free the owned handle; NULL is harmless.
 # Safety
 A nonnull handle is live, freed once, with no concurrent borrowers.
 */
void thinkthen_source_free(struct thinkthen_source *owner);

/*
 Clone an explicitly image-only native reader, validating physical units before opening paths.
 # Safety
 All pointers obey the installed header's storage and lifetime contract.
 Explicit image media with physical unit line/window/file from the same
 descriptor. Native admission rejects line/window before opening any path.
 Existing source_files unit IMAGE_FILE remains a whole-image convenience.
 */
int thinkthen_source_image_files(const struct thinkthen_engine *engine,
                                 const struct thinkthen_source_spec_v1 *spec,
                                 struct thinkthen_source **out);

/*
 Clone every counted original record before returning.
 # Safety
 All pointers obey the installed header's storage and lifetime contract.
 */
int thinkthen_source_records(const struct thinkthen_engine *engine,
                             const struct thinkthen_record_v1 *records,
                             size_t count,
                             struct thinkthen_source **out);

/*
 Start an owned same-thread native batch without collecting its source.
 # Safety
 The installed header specifies counted storage, engine and same-thread lifetimes.
 */
int thinkthen_tag_batch_start(const struct thinkthen_engine *e,
                              const struct thinkthen_question *q,
                              const struct thinkthen_source *s,
                              const struct thinkthen_controls_v1 *c,
                              struct thinkthen_batch **out);

/*
 Execute the named judgment with typed inputs and immutable complete results.
 # Safety
 Every pointer obeys include/thinkthen.h's counted-storage and lifetime rules.
 */
int thinkthen_tag_complete(const struct thinkthen_engine *engine,
                           const struct thinkthen_question *question,
                           const struct thinkthen_source *source,
                           const struct thinkthen_controls_v1 *controls,
                           struct thinkthen_result **out);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

#endif  /* THINKTHEN_H */
