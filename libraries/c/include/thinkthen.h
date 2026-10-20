#ifndef THINKTHEN_H
#define THINKTHEN_H

#define THINKTHEN_INTERFACE_NUMBER 3120711980u

/* Generated from Rust by cbindgen 0.29.4. Run sdlc/scripts/generate-c-header.py; do not edit. */

#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>

/*
 Producer ended normally; the output is ignored.
 */
#define THINKTHEN_CALL_PRODUCER_END 1

/*
 Producer failed; no host exception text crosses this boundary.
 */
#define THINKTHEN_CALL_PRODUCER_FAILURE 2

/*
 Producer published one canonical RequestItem JSON value.
 */
#define THINKTHEN_CALL_PRODUCER_ITEM 0

/*
 A counted COBOL scalar exceeds its documented representation capacity.
 */
#define THINKTHEN_COBOL_OVERFLOW 1002

#define THINKTHEN_COMPLETE_ANNOTATED_FIELD_ARRAY 4

#define THINKTHEN_COMPLETE_ANNOTATED_FIELD_BOOLEAN 1

#define THINKTHEN_COMPLETE_ANNOTATED_FIELD_NULL 2

#define THINKTHEN_COMPLETE_ANNOTATED_FIELD_NUMBER 5

#define THINKTHEN_COMPLETE_ANNOTATED_FIELD_OBJECT 6

#define THINKTHEN_COMPLETE_ANNOTATED_FIELD_STRING 3

#define THINKTHEN_COMPLETE_ANNOTATION_MEMBER_ANSWER_ID 1

#define THINKTHEN_COMPLETE_ANNOTATION_MEMBER_FAILURE_ID 2

#define THINKTHEN_COMPLETE_ANNOTATION_VALUE_CHOICE 2

#define THINKTHEN_COMPLETE_ANNOTATION_VALUE_DECISION 1

#define THINKTHEN_COMPLETE_ANNOTATION_VALUE_FAILED 5

#define THINKTHEN_COMPLETE_ANNOTATION_VALUE_SCORE 3

#define THINKTHEN_COMPLETE_ANNOTATION_VALUE_TAGS 4

#define THINKTHEN_COMPLETE_ANSWER_CHOICE 2

#define THINKTHEN_COMPLETE_ANSWER_RESOLUTIONS_OBJECT 2

#define THINKTHEN_COMPLETE_ANSWER_RESOLUTIONS_STRING 1

#define THINKTHEN_COMPLETE_ANSWER_RESOLUTION_FAILED 3

#define THINKTHEN_COMPLETE_ANSWER_RESOLUTION_RESOLVED 1

#define THINKTHEN_COMPLETE_ANSWER_RESOLUTION_UNRESOLVED 2

#define THINKTHEN_COMPLETE_ANSWER_SCORE 4

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_10_ANY_OF_1_ITEMS_ARRAY 1

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_10_ANY_OF_1_ITEMS_NULL 2

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_10_ARRAY 1

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_10_ARRAY2 3

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_10_NULL 2

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_11_ANY_OF_1_ITEMS_ARRAY 1

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_11_ANY_OF_1_ITEMS_NULL 2

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_11_ARRAY 1

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_11_ARRAY2 3

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_11_NULL 2

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_13_ANY_OF_1_ITEMS_NULL 2

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_13_ANY_OF_1_ITEMS_OBJECT 1

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_13_ARRAY 3

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_13_NULL 2

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_13_OBJECT 1

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_14_ANY_OF_1_ITEMS_ARRAY 1

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_14_ANY_OF_1_ITEMS_NULL 2

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_14_ARRAY 1

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_14_ARRAY2 3

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_14_NULL 2

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_15_ANY_OF_1_ITEMS_NULL 2

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_15_ANY_OF_1_ITEMS_OBJECT 1

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_15_ARRAY 3

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_15_NULL 2

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_15_OBJECT 1

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_2_ARRAY 3

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_2_NULL 2

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_2_NUMBER 1

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_3_ARRAY 3

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_3_NULL 2

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_3_STRING 1

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_4_ANY_OF_1_ITEMS_INTEGER 1

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_4_ANY_OF_1_ITEMS_NULL 2

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_4_ARRAY 3

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_4_INTEGER 1

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_4_NULL 2

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_5_ARRAY 3

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_5_OBJECT 2

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_5_STRING 1

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_6_ANY_OF_1_ITEMS_NULL 2

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_6_ANY_OF_1_ITEMS_STRING 1

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_6_ARRAY 3

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_6_NULL 2

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_6_STRING 1

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_7_ANY_OF_1_ITEMS_ARRAY 1

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_7_ANY_OF_1_ITEMS_NULL 2

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_7_ARRAY 1

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_7_ARRAY2 3

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_7_NULL 2

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_8_ANY_OF_1_ITEMS_NULL 2

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_8_ANY_OF_1_ITEMS_NUMBER 1

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_8_ARRAY 3

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_8_NULL 2

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_8_NUMBER 1

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_9_ANY_OF_1_ITEMS_ARRAY 1

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_9_ANY_OF_1_ITEMS_NULL 2

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_9_ARRAY 1

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_9_ARRAY2 3

#define THINKTHEN_COMPLETE_ANSWER_SHAPE_9_NULL 2

#define THINKTHEN_COMPLETE_ANSWER_TAG 3

#define THINKTHEN_COMPLETE_ANSWER_YES_NO 1

#define THINKTHEN_COMPLETE_ATTEMPT_OUTCOME_OK 1

#define THINKTHEN_COMPLETE_ATTEMPT_OUTCOME_STATUS 2

#define THINKTHEN_COMPLETE_ATTEMPT_OUTCOME_TRANSPORT 3

#define THINKTHEN_COMPLETE_BATCH_INTEGER 1

#define THINKTHEN_COMPLETE_BATCH_SETTING_INTEGER 1

#define THINKTHEN_COMPLETE_BATCH_SETTING_STRING 2

#define THINKTHEN_COMPLETE_BATCH_STRING 2

#define THINKTHEN_COMPLETE_BATCH_STRING_MAX 1

#define THINKTHEN_COMPLETE_BOUNDARY_MODE_BOUNDARY_ONLY 1

#define THINKTHEN_COMPLETE_FAILURE_CAUSE_INVALID_DISTRIBUTION 5

#define THINKTHEN_COMPLETE_FAILURE_CAUSE_INVALID_PROBABILITY 4

#define THINKTHEN_COMPLETE_FAILURE_CAUSE_MISSING_ANSWER 1

#define THINKTHEN_COMPLETE_FAILURE_CAUSE_MISSING_PROBABILITY 3

#define THINKTHEN_COMPLETE_FAILURE_CAUSE_UNEXPECTED_PROBABILITY 6

#define THINKTHEN_COMPLETE_FAILURE_CAUSE_WRONG_KIND 2

#define THINKTHEN_COMPLETE_FAILURE_FIELD_KIND_BACKEND 1

#define THINKTHEN_COMPLETE_FAILURE_KIND_BACKEND 2

#define THINKTHEN_COMPLETE_FAILURE_KIND_CANCELLED 4

#define THINKTHEN_COMPLETE_FAILURE_KIND_DEADLINE 5

#define THINKTHEN_COMPLETE_FAILURE_KIND_DEFECT 6

#define THINKTHEN_COMPLETE_FAILURE_KIND_LOCAL 3

#define THINKTHEN_COMPLETE_FAILURE_KIND_USAGE 1

#define THINKTHEN_COMPLETE_IMAGE_MEDIA_IMAGE_JPEG 1

#define THINKTHEN_COMPLETE_IMAGE_MEDIA_IMAGE_PNG 2

#define THINKTHEN_COMPLETE_INPUT_DECLARATION_OBJECT_ROOT 2

#define THINKTHEN_COMPLETE_INPUT_DECLARATION_STRING_ROOT 1

#define THINKTHEN_COMPLETE_INPUT_DENIAL_ADDITIONAL_REQUEST 2

#define THINKTHEN_COMPLETE_INPUT_DENIAL_INITIAL_REQUEST 1

#define THINKTHEN_COMPLETE_INPUT_DENIAL_RETRY 3

#define THINKTHEN_COMPLETE_INPUT_PROPERTY_TYPE_ARRAY 4

#define THINKTHEN_COMPLETE_INPUT_PROPERTY_TYPE_BOOLEAN 3

#define THINKTHEN_COMPLETE_INPUT_PROPERTY_TYPE_NUMBER 2

#define THINKTHEN_COMPLETE_INPUT_PROPERTY_TYPE_STRING 1

#define THINKTHEN_COMPLETE_JSON_ARRAY 5

#define THINKTHEN_COMPLETE_JSON_BOOLEAN 2

#define THINKTHEN_COMPLETE_JSON_NULL 1

#define THINKTHEN_COMPLETE_JSON_NUMBER 3

#define THINKTHEN_COMPLETE_JSON_OBJECT 6

#define THINKTHEN_COMPLETE_JSON_STRING 4

#define THINKTHEN_COMPLETE_OBJECT_TYPE_OBJECT 1

#define THINKTHEN_COMPLETE_OBSERVATION_FAILURE_ID 2

#define THINKTHEN_COMPLETE_OBSERVATION_OBSERVATION_ID 1

#define THINKTHEN_COMPLETE_ORIGIN_CACHE 2

#define THINKTHEN_COMPLETE_ORIGIN_LIVE 1

#define THINKTHEN_COMPLETE_ORIGIN_MEMORY 5

#define THINKTHEN_COMPLETE_ORIGIN_PROXY 4

#define THINKTHEN_COMPLETE_ORIGIN_REPLAY 3

#define THINKTHEN_COMPLETE_PRESENCE_MISSING 0

#define THINKTHEN_COMPLETE_PRESENCE_NULL 1

#define THINKTHEN_COMPLETE_PRESENCE_VALUE 2

#define THINKTHEN_COMPLETE_QUESTION_DOCUMENT_CHOOSE 2

#define THINKTHEN_COMPLETE_QUESTION_DOCUMENT_DECIDE 1

#define THINKTHEN_COMPLETE_QUESTION_DOCUMENT_SCORE 4

#define THINKTHEN_COMPLETE_QUESTION_DOCUMENT_TAG 3

#define THINKTHEN_COMPLETE_RECOGNITION_MODE_BOUNDARY_ONLY 2

#define THINKTHEN_COMPLETE_RECOGNITION_MODE_WHOLE 1

#define THINKTHEN_COMPLETE_RECOGNITION_ODDS_BOUNDARY_ODDS 2

#define THINKTHEN_COMPLETE_RECOGNITION_ODDS_RECOGNIZE_ANSWER 1

#define THINKTHEN_COMPLETE_RECOGNIZE_ENTITIES 1

#define THINKTHEN_COMPLETE_RECOGNIZE_MODE_PROPOSALS 2

#define THINKTHEN_COMPLETE_RELATION_DIRECTION_EITHER 2

#define THINKTHEN_COMPLETE_RELATION_DIRECTION_SOURCE_TO_TARGET 1

#define THINKTHEN_COMPLETE_RELATION_ENDPOINT_RELATED_ENTITY 1

#define THINKTHEN_COMPLETE_RELATION_ENDPOINT_SOURCE 2

#define THINKTHEN_COMPLETE_RELATION_MEMBER_ANSWER_ID 1

#define THINKTHEN_COMPLETE_RELATION_MEMBER_FAILURE_ID 2

#define THINKTHEN_COMPLETE_RELATION_METHOD_CHOICE 2

#define THINKTHEN_COMPLETE_RELATION_METHOD_YES_NO 1

#define THINKTHEN_COMPLETE_REQUEST_FUNCTION_ANNOTATE 8

#define THINKTHEN_COMPLETE_REQUEST_FUNCTION_CHOOSE 2

#define THINKTHEN_COMPLETE_REQUEST_FUNCTION_DECIDE 1

#define THINKTHEN_COMPLETE_REQUEST_FUNCTION_FILTER 5

#define THINKTHEN_COMPLETE_REQUEST_FUNCTION_FIND 7

#define THINKTHEN_COMPLETE_REQUEST_FUNCTION_RANK 6

#define THINKTHEN_COMPLETE_REQUEST_FUNCTION_RECOGNIZE 9

#define THINKTHEN_COMPLETE_REQUEST_FUNCTION_RELATE 10

#define THINKTHEN_COMPLETE_REQUEST_FUNCTION_SCORE 4

#define THINKTHEN_COMPLETE_REQUEST_FUNCTION_TAG 3

#define THINKTHEN_COMPLETE_RESULT_TAG_THINKTHEN_RESULT 1

#define THINKTHEN_COMPLETE_SEND_DENIAL_BEFORE_ADDITIONAL_SEND 2

#define THINKTHEN_COMPLETE_SEND_DENIAL_BEFORE_FIRST_SEND 1

#define THINKTHEN_COMPLETE_SEND_DENIAL_BEFORE_RETRY 3

#define THINKTHEN_COMPLETE_SESSION_JUDGMENT_CHOICE 2

#define THINKTHEN_COMPLETE_SESSION_JUDGMENT_DECISION 1

#define THINKTHEN_COMPLETE_SESSION_JUDGMENT_SCORE 3

#define THINKTHEN_COMPLETE_SESSION_JUDGMENT_TAGS 4

#define THINKTHEN_COMPLETE_SESSION_OBSERVATION_QUESTION 1

#define THINKTHEN_COMPLETE_SESSION_OBSERVATION_ROW 2

#define THINKTHEN_COMPLETE_SESSION_OBSERVED_ROW_ANNOTATED 2

#define THINKTHEN_COMPLETE_SESSION_OBSERVED_ROW_FIND 4

#define THINKTHEN_COMPLETE_SESSION_OBSERVED_ROW_JUDGMENT 1

#define THINKTHEN_COMPLETE_SESSION_OBSERVED_ROW_RECOGNIZED 3

#define THINKTHEN_COMPLETE_SESSION_OBSERVED_ROW_RELATIONS 5

#define THINKTHEN_COMPLETE_SESSION_PACKET_ANNOTATE_AGGREGATE 14

#define THINKTHEN_COMPLETE_SESSION_PACKET_ANNOTATE_ROW 6

#define THINKTHEN_COMPLETE_SESSION_PACKET_CHOOSE_AGGREGATE 8

#define THINKTHEN_COMPLETE_SESSION_PACKET_CHOOSE_ROW 2

#define THINKTHEN_COMPLETE_SESSION_PACKET_DECIDE_AGGREGATE 7

#define THINKTHEN_COMPLETE_SESSION_PACKET_DECIDE_ROW 1

#define THINKTHEN_COMPLETE_SESSION_PACKET_FILTER_AGGREGATE 11

#define THINKTHEN_COMPLETE_SESSION_PACKET_FILTER_ROW 5

#define THINKTHEN_COMPLETE_SESSION_PACKET_FIND_AGGREGATE 13

#define THINKTHEN_COMPLETE_SESSION_PACKET_OBSERVATION 17

#define THINKTHEN_COMPLETE_SESSION_PACKET_RANK_AGGREGATE 12

#define THINKTHEN_COMPLETE_SESSION_PACKET_RECOGNIZE_AGGREGATE 15

#define THINKTHEN_COMPLETE_SESSION_PACKET_RELATE_AGGREGATE 16

#define THINKTHEN_COMPLETE_SESSION_PACKET_SCORE_AGGREGATE 10

#define THINKTHEN_COMPLETE_SESSION_PACKET_SCORE_ROW 4

#define THINKTHEN_COMPLETE_SESSION_PACKET_TAG_AGGREGATE 9

#define THINKTHEN_COMPLETE_SESSION_PACKET_TAG_ROW 3

#define THINKTHEN_COMPLETE_SESSION_PACKET_TERMINAL 18

#define THINKTHEN_COMPLETE_SESSION_PROBABILITIES_NAMED 2

#define THINKTHEN_COMPLETE_SESSION_PROBABILITIES_YES_NO 1

#define THINKTHEN_COMPLETE_SETTLED_ANSWER_ANNOTATE 10

#define THINKTHEN_COMPLETE_SETTLED_ANSWER_CHOOSE 2

#define THINKTHEN_COMPLETE_SETTLED_ANSWER_DECIDE 1

#define THINKTHEN_COMPLETE_SETTLED_ANSWER_FILTER 5

#define THINKTHEN_COMPLETE_SETTLED_ANSWER_FIND 7

#define THINKTHEN_COMPLETE_SETTLED_ANSWER_RANK 6

#define THINKTHEN_COMPLETE_SETTLED_ANSWER_RECOGNIZE 8

#define THINKTHEN_COMPLETE_SETTLED_ANSWER_RELATE 9

#define THINKTHEN_COMPLETE_SETTLED_ANSWER_SCORE 4

#define THINKTHEN_COMPLETE_SETTLED_ANSWER_TAG 3

#define THINKTHEN_COMPLETE_SETTLED_JUDGMENT_ARRAY 5

#define THINKTHEN_COMPLETE_SETTLED_JUDGMENT_BOOLEAN 1

#define THINKTHEN_COMPLETE_SETTLED_JUDGMENT_NULL 2

#define THINKTHEN_COMPLETE_SETTLED_JUDGMENT_NUMBER 4

#define THINKTHEN_COMPLETE_SETTLED_JUDGMENT_STRING 3

#define THINKTHEN_COMPLETE_SETTLED_RECOGNITION_ENTITIES 1

#define THINKTHEN_COMPLETE_SETTLED_RECOGNITION_MODE_PROPOSALS 2

#define THINKTHEN_COMPLETE_STOP_CAUSE_BACKEND 8

#define THINKTHEN_COMPLETE_STOP_CAUSE_CANCELLED 9

#define THINKTHEN_COMPLETE_STOP_CAUSE_DEADLINE 10

#define THINKTHEN_COMPLETE_STOP_CAUSE_DEFECT 11

#define THINKTHEN_COMPLETE_STOP_CAUSE_LOCAL 2

#define THINKTHEN_COMPLETE_STOP_CAUSE_NO_KEY 3

#define THINKTHEN_COMPLETE_STOP_CAUSE_REPLY 7

#define THINKTHEN_COMPLETE_STOP_CAUSE_STATUS 5

#define THINKTHEN_COMPLETE_STOP_CAUSE_TOO_LARGE 6

#define THINKTHEN_COMPLETE_STOP_CAUSE_TRANSPORT 4

#define THINKTHEN_COMPLETE_STOP_CAUSE_USAGE 1

#define THINKTHEN_COMPLETE_STRING_TYPE_STRING 1

#define THINKTHEN_COMPLETE_THRESHOLD_NUMBER 1

#define THINKTHEN_COMPLETE_THRESHOLD_STRING 2

#define THINKTHEN_COMPLETE_USAGE_PERSISTENCE_DISABLED 1

#define THINKTHEN_COMPLETE_USAGE_PERSISTENCE_FAILED 4

#define THINKTHEN_COMPLETE_USAGE_PERSISTENCE_PENDING 2

#define THINKTHEN_COMPLETE_USAGE_PERSISTENCE_WRITTEN 3

#define THINKTHEN_COMPLETE_VALUE_ARRAY 4

#define THINKTHEN_COMPLETE_VALUE_BOOLEAN 1

#define THINKTHEN_COMPLETE_VALUE_NULL 2

#define THINKTHEN_COMPLETE_VALUE_NUMBER 5

#define THINKTHEN_COMPLETE_VALUE_STRING 3

#define THINKTHEN_COMPLETE_VERB_RECOGNIZE 1

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

#define THINKTHEN_REQUEST_ANNOTATE_QUESTION_CHOOSE 2

#define THINKTHEN_REQUEST_ANNOTATE_QUESTION_DECIDE 1

#define THINKTHEN_REQUEST_ANNOTATE_QUESTION_SCORE 4

#define THINKTHEN_REQUEST_ANNOTATE_QUESTION_TAG 3

#define THINKTHEN_REQUEST_BATCH_TEXT 2

#define THINKTHEN_REQUEST_BATCH_UNSIGNED_VALUE 1

#define THINKTHEN_REQUEST_CACHE_DOCUMENT_DISABLED_CACHE 2

#define THINKTHEN_REQUEST_CACHE_DOCUMENT_TEXT 1

#define THINKTHEN_REQUEST_CALL_THRESHOLD_NUMBER 1

#define THINKTHEN_REQUEST_CALL_THRESHOLD_TEXT 2

#define THINKTHEN_REQUEST_CONTEXT_SCHEMA_JSON_VALUE_MAP 2

#define THINKTHEN_REQUEST_CONTEXT_SCHEMA_TEXT 1

#define THINKTHEN_REQUEST_CRITERION_JSON_VALUE_LIST 3

#define THINKTHEN_REQUEST_CRITERION_JSON_VALUE_MAP 2

#define THINKTHEN_REQUEST_CRITERION_NULL 4

#define THINKTHEN_REQUEST_CRITERION_TEXT 1

#define THINKTHEN_REQUEST_CUT_NUMBER 1

#define THINKTHEN_REQUEST_CUT_TEXT 2

#define THINKTHEN_REQUEST_DEFINITION_ANNOTATE_DEFINITION 8

#define THINKTHEN_REQUEST_DEFINITION_AUTHORED_CHOOSE 2

#define THINKTHEN_REQUEST_DEFINITION_AUTHORED_DECIDE 1

#define THINKTHEN_REQUEST_DEFINITION_AUTHORED_FIND 6

#define THINKTHEN_REQUEST_DEFINITION_AUTHORED_RELATE 5

#define THINKTHEN_REQUEST_DEFINITION_AUTHORED_SCORE 4

#define THINKTHEN_REQUEST_DEFINITION_AUTHORED_TAG 3

#define THINKTHEN_REQUEST_DEFINITION_RECOGNIZE_DEFINITION 7

#define THINKTHEN_REQUEST_DESCRIPTION_JSON_VALUE_LIST 3

#define THINKTHEN_REQUEST_DESCRIPTION_JSON_VALUE_MAP 2

#define THINKTHEN_REQUEST_DESCRIPTION_NULL 4

#define THINKTHEN_REQUEST_DESCRIPTION_TEXT 1

#define THINKTHEN_REQUEST_FRAMING_LITERAL_CSV 4

#define THINKTHEN_REQUEST_FRAMING_LITERAL_DOCUMENT 1

#define THINKTHEN_REQUEST_FRAMING_LITERAL_JSONL 3

#define THINKTHEN_REQUEST_FRAMING_LITERAL_LINES 2

#define THINKTHEN_REQUEST_FRAMING_LITERAL_TSV 5

#define THINKTHEN_REQUEST_IMAGE_BYTES 2

#define THINKTHEN_REQUEST_IMAGE_FILE 1

#define THINKTHEN_REQUEST_IMAGE_MEDIA_LITERAL_IMAGE_JPEG 1

#define THINKTHEN_REQUEST_IMAGE_MEDIA_LITERAL_IMAGE_PNG 2

#define THINKTHEN_REQUEST_INPUT_DECLARATION_OBJECT 2

#define THINKTHEN_REQUEST_INPUT_DECLARATION_STRING 1

#define THINKTHEN_REQUEST_INPUT_ENTITIES 5

#define THINKTHEN_REQUEST_INPUT_FEED 7

#define THINKTHEN_REQUEST_INPUT_JSON 2

#define THINKTHEN_REQUEST_INPUT_PROPERTY_ARRAY 4

#define THINKTHEN_REQUEST_INPUT_PROPERTY_BOOLEAN 3

#define THINKTHEN_REQUEST_INPUT_PROPERTY_NUMBER 2

#define THINKTHEN_REQUEST_INPUT_PROPERTY_STRING 1

#define THINKTHEN_REQUEST_INPUT_RECORDS 3

#define THINKTHEN_REQUEST_INPUT_SOURCE 6

#define THINKTHEN_REQUEST_INPUT_TEXT 1

#define THINKTHEN_REQUEST_INPUT_UNITS 4

#define THINKTHEN_REQUEST_LABELS_DESCRIPTION_MAP 2

#define THINKTHEN_REQUEST_LABELS_NAME_LIST 1

#define THINKTHEN_REQUEST_LEVELS_CRITERION_MAP 2

#define THINKTHEN_REQUEST_LEVELS_NAME_LIST 1

#define THINKTHEN_REQUEST_OPTIONAL_COUNT_NULL 2

#define THINKTHEN_REQUEST_OPTIONAL_COUNT_UNSIGNED_VALUE 1

#define THINKTHEN_REQUEST_OPTIONAL_TOTAL_NULL 2

#define THINKTHEN_REQUEST_OPTIONAL_TOTAL_UNSIGNED_VALUE 1

#define THINKTHEN_REQUEST_OPTIONS_DESCRIPTION_MAP 2

#define THINKTHEN_REQUEST_OPTIONS_NAME_LIST 1

#define THINKTHEN_REQUEST_ORIGINAL_JSON 2

#define THINKTHEN_REQUEST_ORIGINAL_TEXT 1

#define THINKTHEN_REQUEST_POINTERS_TEXT 1

#define THINKTHEN_REQUEST_POINTERS_TEXT_LIST 2

#define THINKTHEN_REQUEST_QUESTION_BATCH_INTEGER 2

#define THINKTHEN_REQUEST_QUESTION_BATCH_LITERAL_MAX 1

#define THINKTHEN_REQUEST_QUESTION_SELECTOR_DEFINITION 2

#define THINKTHEN_REQUEST_QUESTION_SELECTOR_FILE 3

#define THINKTHEN_REQUEST_QUESTION_SELECTOR_NAME 4

#define THINKTHEN_REQUEST_QUESTION_SELECTOR_REFERENCE 5

#define THINKTHEN_REQUEST_QUESTION_SELECTOR_TEXT 1

#define THINKTHEN_REQUEST_QUESTION_TEXT_JSON_VALUE_LIST 3

#define THINKTHEN_REQUEST_QUESTION_TEXT_JSON_VALUE_MAP 2

#define THINKTHEN_REQUEST_QUESTION_TEXT_TEXT 1

#define THINKTHEN_REQUEST_READER_FAILURE_INVALID_INPUT 4

#define THINKTHEN_REQUEST_READER_FAILURE_IO 2

#define THINKTHEN_REQUEST_READER_FAILURE_PRODUCER 1

#define THINKTHEN_REQUEST_READER_FAILURE_UTF_8 3

#define THINKTHEN_REQUEST_READER_MEDIA_LITERAL_IMAGE 2

#define THINKTHEN_REQUEST_READER_MEDIA_LITERAL_TEXT 1

#define THINKTHEN_REQUEST_RECOGNITION_EXAMPLE_EXAMPLE_TEXT 2

#define THINKTHEN_REQUEST_RECOGNITION_EXAMPLE_TEXT 1

#define THINKTHEN_REQUEST_RECOGNITION_MODE_LITERAL_BOUNDARY_ONLY 2

#define THINKTHEN_REQUEST_RECOGNITION_MODE_LITERAL_WHOLE 1

#define THINKTHEN_REQUEST_SOURCE_UNIT_LITERAL_FILE 3

#define THINKTHEN_REQUEST_SOURCE_UNIT_LITERAL_LINE 1

#define THINKTHEN_REQUEST_SOURCE_UNIT_LITERAL_WINDOW 2

#define THINKTHEN_REQUEST_THRESHOLD_NUMBER 1

#define THINKTHEN_REQUEST_THRESHOLD_TEXT 2

/*
 Push transferred one descriptor to the session.
 */
#define THINKTHEN_SESSION_ACCEPTED 0

/*
 Intake closed; stop advancing the producer.
 */
#define THINKTHEN_SESSION_CLOSED 2

/*
 Terminal was read and no more output will arrive.
 */
#define THINKTHEN_SESSION_END 2

/*
 Push retained nothing; retry the same descriptor.
 */
#define THINKTHEN_SESSION_FULL 1

/*
 Work has not settled and no packet is ready.
 */
#define THINKTHEN_SESSION_PENDING 1

/*
 Intake was cancelled; settlement can still be pending.
 */
#define THINKTHEN_SESSION_READY_CANCELLED 4

/*
 Caller-supplied input has closed.
 */
#define THINKTHEN_SESSION_READY_CLOSED 2

/*
 Every output packet has been consumed.
 */
#define THINKTHEN_SESSION_READY_END 3

/*
 Caller-supplied input has room.
 */
#define THINKTHEN_SESSION_READY_INPUT 1

/*
 A result or terminal packet is ready; no packet has been consumed.
 */
#define THINKTHEN_SESSION_READY_RESULT 0

/*
 The caller-given timeout passed.
 */
#define THINKTHEN_SESSION_READY_TIMEOUT 5

/*
 Read transferred one independent packet owner.
 */
#define THINKTHEN_SESSION_RESULT 0

/*
 Wait for result, input room/closure, end or cancel.
 */
#define THINKTHEN_SESSION_WAIT_INPUT 1

/*
 Wait for final output or cancellation requested since this reader armed.
 */
#define THINKTHEN_SESSION_WAIT_INTERRUPTIBLE_SETTLEMENT 4

/*
 Wait only for results, end or cancel; input room cannot wake this wait.
 */
#define THINKTHEN_SESSION_WAIT_RESULT 0

/*
 Drain output after cancel without waking repeatedly for cancellation.
 */
#define THINKTHEN_SESSION_WAIT_SETTLEMENT 2

/*
 Arm one exclusive settlement reader and check readiness immediately; timeout must be zero.
 */
#define THINKTHEN_SESSION_WAIT_SETTLEMENT_ARM 3

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
typedef enum thinkthen_input_declaration_kind {
  /*
   C grammar value `THINKTHEN_DECLARATION_ABSENT`.
   */
  THINKTHEN_DECLARATION_ABSENT = 0,
  /*
   C grammar value `THINKTHEN_DECLARATION_STRING`.
   */
  THINKTHEN_DECLARATION_STRING = 1,
  /*
   C grammar value `THINKTHEN_DECLARATION_OBJECT`.
   */
  THINKTHEN_DECLARATION_OBJECT = 2,
} thinkthen_input_declaration_kind;

/*
 Admitted native grammar values; descriptor storage remains uint32_t.
 */
typedef enum thinkthen_input_property_kind {
  /*
   C grammar value `THINKTHEN_PROPERTY_STRING`.
   */
  THINKTHEN_PROPERTY_STRING = 1,
  /*
   C grammar value `THINKTHEN_PROPERTY_NUMBER`.
   */
  THINKTHEN_PROPERTY_NUMBER = 2,
  /*
   C grammar value `THINKTHEN_PROPERTY_BOOLEAN`.
   */
  THINKTHEN_PROPERTY_BOOLEAN = 3,
  /*
   C grammar value `THINKTHEN_PROPERTY_STRING_LIST`.
   */
  THINKTHEN_PROPERTY_STRING_LIST = 4,
} thinkthen_input_property_kind;

/*
 Admitted native grammar values; descriptor storage remains uint32_t.
 */
typedef enum thinkthen_question_loader_role {
  /*
   C grammar value `THINKTHEN_LOAD_ATOMIC`.
   */
  THINKTHEN_LOAD_ATOMIC = 1,
  /*
   C grammar value `THINKTHEN_LOAD_SET`.
   */
  THINKTHEN_LOAD_SET = 2,
  /*
   C grammar value `THINKTHEN_LOAD_DYNAMIC_CHOOSE`.
   */
  THINKTHEN_LOAD_DYNAMIC_CHOOSE = 3,
  /*
   C grammar value `THINKTHEN_LOAD_RECOGNIZE`.
   */
  THINKTHEN_LOAD_RECOGNIZE = 4,
  /*
   C grammar value `THINKTHEN_LOAD_RELATE`.
   */
  THINKTHEN_LOAD_RELATE = 5,
  /*
   C grammar value `THINKTHEN_LOAD_RANK`.
   */
  THINKTHEN_LOAD_RANK = 6,
  /*
   C grammar value `THINKTHEN_LOAD_RANK_SET`.
   */
  THINKTHEN_LOAD_RANK_SET = 7,
  /*
   C grammar value `THINKTHEN_LOAD_FIND`.
   */
  THINKTHEN_LOAD_FIND = 8,
} thinkthen_question_loader_role;

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
 C descriptor or borrowed view `thinkthen_complete_utf8`.
 */
typedef struct thinkthen_complete_utf8 {
  /*
   C field `data`.
   */
  const char *data;
  /*
   C field `len`.
   */
  size_t len;
} thinkthen_complete_utf8;

typedef struct thinkthen_complete_usage_persistence {
  uint32_t kind;
} thinkthen_complete_usage_persistence;

typedef struct thinkthen_request_text {
  struct thinkthen_complete_utf8 text;
} thinkthen_request_text;

typedef struct thinkthen_request_unsigned {
  uint64_t value;
} thinkthen_request_unsigned;

typedef union thinkthen_request_batch_data {
  const struct thinkthen_request_unsigned *unsigned_value;
  const struct thinkthen_request_text *text;
} thinkthen_request_batch_data;

typedef struct thinkthen_request_batch {
  uint32_t kind;
  union thinkthen_request_batch_data data;
} thinkthen_request_batch;

typedef struct thinkthen_request_disabled_cache {
  uint32_t reserved;
} thinkthen_request_disabled_cache;

typedef union thinkthen_request_cache_document_data {
  const struct thinkthen_request_text *text;
  const struct thinkthen_request_disabled_cache *disabled_cache;
} thinkthen_request_cache_document_data;

typedef struct thinkthen_request_cache_document {
  uint32_t kind;
  union thinkthen_request_cache_document_data data;
} thinkthen_request_cache_document;

typedef struct thinkthen_request_null {
  uint32_t reserved;
} thinkthen_request_null;

typedef union thinkthen_request_optional_total_data {
  const struct thinkthen_request_unsigned *unsigned_value;
  const struct thinkthen_request_null *null;
} thinkthen_request_optional_total_data;

typedef struct thinkthen_request_optional_total {
  uint32_t kind;
  union thinkthen_request_optional_total_data data;
} thinkthen_request_optional_total;

typedef union thinkthen_request_optional_count_data {
  const struct thinkthen_request_unsigned *unsigned_value;
  const struct thinkthen_request_null *null;
} thinkthen_request_optional_count_data;

typedef struct thinkthen_request_optional_count {
  uint32_t kind;
  union thinkthen_request_optional_count_data data;
} thinkthen_request_optional_count;

/*
 C descriptor or borrowed view `thinkthen_complete_json_array`.
 */
typedef struct thinkthen_complete_json_array {
  /*
   C field `data`.
   */
  const struct thinkthen_complete_json *const *data;
  /*
   C field `len`.
   */
  size_t len;
} thinkthen_complete_json_array;

/*
 C descriptor or borrowed view `thinkthen_complete_json_entry`.
 */
typedef struct thinkthen_complete_json_entry {
  /*
   C field `name`.
   */
  struct thinkthen_complete_utf8 name;
  /*
   C field `value`.
   */
  const struct thinkthen_complete_json *value;
} thinkthen_complete_json_entry;

/*
 C descriptor or borrowed view `thinkthen_complete_json_object`.
 */
typedef struct thinkthen_complete_json_object {
  /*
   C field `data`.
   */
  const struct thinkthen_complete_json_entry *data;
  /*
   C field `len`.
   */
  size_t len;
} thinkthen_complete_json_object;

/*
 C descriptor or borrowed view `thinkthen_complete_json_data`.
 */
typedef union thinkthen_complete_json_data {
  /*
   C field `boolean`.
   */
  uint32_t boolean;
  /*
   C field `number`.
   */
  struct thinkthen_complete_utf8 number;
  /*
   C field `string`.
   */
  struct thinkthen_complete_utf8 string;
  /*
   C field `array`.
   */
  struct thinkthen_complete_json_array array;
  /*
   C field `object`.
   */
  struct thinkthen_complete_json_object object;
} thinkthen_complete_json_data;

/*
 C descriptor or borrowed view `thinkthen_complete_json`.
 */
typedef struct thinkthen_complete_json {
  /*
   C field `kind`.
   */
  uint32_t kind;
  /*
   C field `data`.
   */
  union thinkthen_complete_json_data data;
} thinkthen_complete_json;

typedef struct thinkthen_request_json_value {
  const struct thinkthen_complete_json *value;
} thinkthen_request_json_value;

typedef struct thinkthen_request_boolean {
  uint32_t value;
} thinkthen_request_boolean;

typedef struct thinkthen_request_engine_settings {
  const struct thinkthen_request_text *m_backend;
  const struct thinkthen_request_text *m_base_url;
  const struct thinkthen_request_batch *m_batch;
  const struct thinkthen_request_cache_document *m_cache;
  const struct thinkthen_request_optional_total *m_max_estimated_input_tokens_total;
  const struct thinkthen_request_unsigned *m_max_request_bytes;
  const struct thinkthen_request_optional_count *m_max_requests;
  const struct thinkthen_request_optional_total *m_max_requests_total;
  const struct thinkthen_request_unsigned *m_max_retries;
  const struct thinkthen_request_text *m_model;
  const struct thinkthen_request_text *m_profile;
  const struct thinkthen_request_json_value *m_proxy;
  const struct thinkthen_request_text *m_record;
  const struct thinkthen_request_boolean *m_refresh_cache;
  const struct thinkthen_request_text *m_replay;
  const struct thinkthen_request_unsigned *m_throttle;
  const struct thinkthen_request_unsigned *m_timeout;
  const struct thinkthen_request_text *m_usd_per_million_input;
  const struct thinkthen_request_text *m_usd_per_million_output;
} thinkthen_request_engine_settings;

typedef struct thinkthen_complete_attempt_outcome {
  uint32_t kind;
} thinkthen_complete_attempt_outcome;

typedef struct thinkthen_complete_utf8_presence {
  uint32_t presence;
  struct thinkthen_complete_utf8 value;
} thinkthen_complete_utf8_presence;

typedef struct thinkthen_complete_sdk_request_id {
  struct thinkthen_complete_utf8 value;
} thinkthen_complete_sdk_request_id;

typedef struct thinkthen_complete_u64_presence {
  uint32_t presence;
  uint64_t value;
} thinkthen_complete_u64_presence;

/*
 C descriptor or borrowed view `thinkthen_complete_extension`.
 */
typedef struct thinkthen_complete_extension {
  /*
   C field `name`.
   */
  struct thinkthen_complete_utf8 name;
  /*
   C field `json`.
   */
  struct thinkthen_complete_utf8 json;
  /*
   C field `value`.
   */
  const struct thinkthen_complete_json *value;
} thinkthen_complete_extension;

/*
 C descriptor or borrowed view `thinkthen_complete_extensions`.
 */
typedef struct thinkthen_complete_extensions {
  /*
   C field `data`.
   */
  const struct thinkthen_complete_extension *data;
  /*
   C field `len`.
   */
  size_t len;
} thinkthen_complete_extensions;

typedef struct thinkthen_complete_attempt {
  uint64_t ordinal;
  const struct thinkthen_complete_attempt_outcome *outcome;
  struct thinkthen_complete_utf8_presence request_id;
  struct thinkthen_complete_utf8 request_sha256;
  const struct thinkthen_complete_sdk_request_id *sdk_request_id;
  struct thinkthen_complete_u64_presence server_ms;
  struct thinkthen_complete_u64_presence status;
  uint64_t wall_ms;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_attempt;

typedef struct thinkthen_complete_attempt_list {
  const struct thinkthen_complete_attempt *const *data;
  size_t len;
} thinkthen_complete_attempt_list;

typedef struct thinkthen_complete_attempt_list_presence {
  uint32_t presence;
  struct thinkthen_complete_attempt_list value;
} thinkthen_complete_attempt_list_presence;

typedef struct thinkthen_complete_call_id {
  struct thinkthen_complete_utf8 value;
} thinkthen_complete_call_id;

typedef struct thinkthen_complete_call_id_presence {
  uint32_t presence;
  const struct thinkthen_complete_call_id *value;
} thinkthen_complete_call_id_presence;

typedef struct thinkthen_complete_u32_presence {
  uint32_t presence;
  uint32_t value;
} thinkthen_complete_u32_presence;

typedef struct thinkthen_complete_persistence_observation {
  struct thinkthen_complete_utf8_presence advice;
  struct thinkthen_complete_utf8 observed_at;
  const struct thinkthen_complete_usage_persistence *state;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_persistence_observation;

typedef struct thinkthen_complete_persistence_observation_presence {
  uint32_t presence;
  const struct thinkthen_complete_persistence_observation *value;
} thinkthen_complete_persistence_observation_presence;

typedef struct thinkthen_complete_immediate_facts {
  struct thinkthen_complete_attempt_list_presence attempts;
  uint64_t cache_answers;
  struct thinkthen_complete_call_id_presence call_id;
  struct thinkthen_complete_utf8_presence estimated_cost_usd;
  struct thinkthen_complete_u32_presence held_model_mismatch;
  struct thinkthen_complete_u64_presence input_tokens;
  uint64_t largest_request_bytes;
  struct thinkthen_complete_u64_presence largest_request_estimated_input_tokens;
  struct thinkthen_complete_utf8_presence model;
  struct thinkthen_complete_u64_presence output_tokens;
  uint64_t records;
  uint64_t requests_sent;
  double seconds;
  struct thinkthen_complete_utf8 token_estimate_method;
  struct thinkthen_complete_persistence_observation_presence usage_persistence;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_immediate_facts;

/*
 C descriptor or borrowed view `thinkthen_error_details`.
 */
typedef struct thinkthen_error_details {
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
  struct thinkthen_complete_utf8 message;
  /*
   C field `facts`.
   */
  const struct thinkthen_complete_immediate_facts *facts;
} thinkthen_error_details;

/*
 C descriptor or borrowed view `CountedString`.
 */
typedef struct thinkthen_string {
  /*
   C field `data`.
   */
  const char *data;
  /*
   C field `len`.
   */
  size_t len;
} thinkthen_string;

/*
 C descriptor or borrowed view `OptionalString`.
 */
typedef struct thinkthen_optional_string {
  /*
   C field `present`.
   */
  int present;
  /*
   C field `value`.
   */
  struct thinkthen_string value;
} thinkthen_optional_string;

/*
 C descriptor or borrowed view `InputPropertyView`.
 */
typedef struct thinkthen_input_property {
  /*
   C field `name`.
   */
  struct thinkthen_string name;
  /*
   C field `kind`.
   */
  uint32_t kind;
} thinkthen_input_property;

/*
 C descriptor or borrowed view `InputProperties`.
 */
typedef struct thinkthen_input_properties {
  /*
   C field `data`.
   */
  const struct thinkthen_input_property *data;
  /*
   C field `len`.
   */
  size_t len;
} thinkthen_input_properties;

/*
 C descriptor or borrowed view `Strings`.
 */
typedef struct thinkthen_strings {
  /*
   C field `data`.
   */
  const struct thinkthen_string *data;
  /*
   C field `len`.
   */
  size_t len;
} thinkthen_strings;

/*
 C descriptor or borrowed view `InputDeclarationView`.
 */
typedef struct thinkthen_input_declaration {
  /*
   C field `kind`.
   */
  uint32_t kind;
  /*
   C field `properties`.
   */
  struct thinkthen_input_properties properties;
  /*
   C field `required`.
   */
  struct thinkthen_strings required;
} thinkthen_input_declaration;

/*
 C descriptor or borrowed view `QuestionAuthorInfo`.
 */
typedef struct thinkthen_question_author_info {
  /*
   C field `name`.
   */
  struct thinkthen_optional_string name;
  /*
   C field `item_schema`.
   */
  struct thinkthen_input_declaration item_schema;
  /*
   C field `context_schema`.
   */
  struct thinkthen_input_declaration context_schema;
} thinkthen_question_author_info;

/*
 Existing canonical components for one named request. Every view is counted
 UTF-8, without a trailing NUL. No bytes or foreign pointers survive return.
 */
typedef struct thinkthen_named_request {
  /*
   Exact lower-case public function name.
   */
  struct thinkthen_complete_utf8 function;
  /*
   Existing canonical question selector JSON.
   */
  struct thinkthen_complete_utf8 question_json;
  /*
   Existing canonical input selector JSON.
   */
  struct thinkthen_complete_utf8 input_json;
  /*
   Existing canonical options JSON; pass {} for omitted controls.
   */
  struct thinkthen_complete_utf8 options_json;
} thinkthen_named_request;

typedef struct thinkthen_request_literal_image_jpeg {
  uint32_t reserved;
} thinkthen_request_literal_image_jpeg;

typedef struct thinkthen_request_literal_image_png {
  uint32_t reserved;
} thinkthen_request_literal_image_png;

typedef union thinkthen_request_image_media_data {
  const struct thinkthen_request_literal_image_jpeg *literal_image_jpeg;
  const struct thinkthen_request_literal_image_png *literal_image_png;
} thinkthen_request_image_media_data;

typedef struct thinkthen_request_image_media {
  uint32_t kind;
  union thinkthen_request_image_media_data data;
} thinkthen_request_image_media;

typedef struct thinkthen_request_image_file {
  const struct thinkthen_request_image_media *m_media;
  const struct thinkthen_request_text *m_path;
} thinkthen_request_image_file;

typedef struct thinkthen_request_image_bytes {
  const struct thinkthen_request_text *m_bytes;
  const struct thinkthen_request_image_media *m_media;
} thinkthen_request_image_bytes;

typedef union thinkthen_request_image_data {
  const struct thinkthen_request_image_file *file;
  const struct thinkthen_request_image_bytes *bytes;
} thinkthen_request_image_data;

typedef struct thinkthen_request_image {
  uint32_t kind;
  union thinkthen_request_image_data data;
} thinkthen_request_image;

typedef struct thinkthen_request_image_list {
  const struct thinkthen_request_image *const *data;
  size_t len;
} thinkthen_request_image_list;

typedef struct thinkthen_request_input_text {
  const struct thinkthen_request_image_list *m_images;
  const struct thinkthen_request_text *m_text;
} thinkthen_request_input_text;

typedef struct thinkthen_request_input_json_value {
  const struct thinkthen_complete_json *value;
} thinkthen_request_input_json_value;

typedef struct thinkthen_request_input_json {
  const struct thinkthen_request_image_list *m_images;
  const struct thinkthen_request_input_json_value *m_value;
} thinkthen_request_input_json;

typedef struct thinkthen_request_json_value_map_entry {
  struct thinkthen_complete_utf8 key;
  const struct thinkthen_request_json_value *value;
} thinkthen_request_json_value_map_entry;

typedef struct thinkthen_request_json_value_map {
  const struct thinkthen_request_json_value_map_entry *data;
  size_t len;
} thinkthen_request_json_value_map;

typedef union thinkthen_request_context_schema_data {
  const struct thinkthen_request_text *text;
  const struct thinkthen_request_json_value_map *json_value_map;
} thinkthen_request_context_schema_data;

typedef struct thinkthen_request_context_schema {
  uint32_t kind;
  union thinkthen_request_context_schema_data data;
} thinkthen_request_context_schema;

typedef struct thinkthen_request_recognition_example_entity {
  const struct thinkthen_request_unsigned *m_end;
  const struct thinkthen_request_text *m_kind;
  const struct thinkthen_request_unsigned *m_start;
} thinkthen_request_recognition_example_entity;

typedef struct thinkthen_request_recognition_example_entity_list {
  const struct thinkthen_request_recognition_example_entity *const *data;
  size_t len;
} thinkthen_request_recognition_example_entity_list;

typedef struct thinkthen_request_text_list {
  const struct thinkthen_request_text *const *data;
  size_t len;
} thinkthen_request_text_list;

typedef struct thinkthen_request_recognition_example_text {
  const struct thinkthen_request_recognition_example_entity_list *m_entities;
  const struct thinkthen_request_text_list *m_kinds;
  const struct thinkthen_request_text *m_text;
} thinkthen_request_recognition_example_text;

typedef union thinkthen_request_recognition_example_data {
  const struct thinkthen_request_text *text;
  const struct thinkthen_request_recognition_example_text *example_text;
} thinkthen_request_recognition_example_data;

typedef struct thinkthen_request_recognition_example {
  uint32_t kind;
  union thinkthen_request_recognition_example_data data;
} thinkthen_request_recognition_example;

typedef struct thinkthen_request_recognition_example_list {
  const struct thinkthen_request_recognition_example *const *data;
  size_t len;
} thinkthen_request_recognition_example_list;

typedef struct thinkthen_request_option_schema {
  const struct thinkthen_request_json_value *m_description;
  const struct thinkthen_request_text *m_name;
} thinkthen_request_option_schema;

typedef struct thinkthen_request_option_schema_list {
  const struct thinkthen_request_option_schema *const *data;
  size_t len;
} thinkthen_request_option_schema_list;

typedef struct thinkthen_request_original_text {
  const struct thinkthen_request_text *m_text;
} thinkthen_request_original_text;

typedef struct thinkthen_request_original_json_value {
  const struct thinkthen_complete_json *value;
} thinkthen_request_original_json_value;

typedef struct thinkthen_request_original_json {
  const struct thinkthen_request_original_json_value *m_value;
} thinkthen_request_original_json;

typedef union thinkthen_request_original_data {
  const struct thinkthen_request_original_text *text;
  const struct thinkthen_request_original_json *json;
} thinkthen_request_original_data;

typedef struct thinkthen_request_original {
  uint32_t kind;
  union thinkthen_request_original_data data;
} thinkthen_request_original;

typedef struct thinkthen_request_recognition_seed_span {
  const struct thinkthen_request_unsigned *m_end;
  const struct thinkthen_request_text *m_kind;
  const struct thinkthen_request_unsigned *m_start;
} thinkthen_request_recognition_seed_span;

typedef struct thinkthen_request_recognition_seed_span_list {
  const struct thinkthen_request_recognition_seed_span *const *data;
  size_t len;
} thinkthen_request_recognition_seed_span_list;

typedef struct thinkthen_request_item {
  const struct thinkthen_request_context_schema *m_context;
  const struct thinkthen_request_recognition_example_list *m_examples;
  const struct thinkthen_request_image_list *m_images;
  const struct thinkthen_request_option_schema_list *m_options;
  const struct thinkthen_request_original *m_original;
  const struct thinkthen_request_recognition_seed_span_list *m_seed_spans;
} thinkthen_request_item;

typedef struct thinkthen_request_item_list {
  const struct thinkthen_request_item *const *data;
  size_t len;
} thinkthen_request_item_list;

typedef struct thinkthen_request_input_records {
  const struct thinkthen_request_item_list *m_items;
} thinkthen_request_input_records;

typedef struct thinkthen_request_input_units {
  const struct thinkthen_request_item_list *m_items;
} thinkthen_request_input_units;

typedef struct thinkthen_request_input_entities {
  const struct thinkthen_request_item_list *m_items;
} thinkthen_request_input_entities;

typedef struct thinkthen_request_literal_document {
  uint32_t reserved;
} thinkthen_request_literal_document;

typedef struct thinkthen_request_literal_lines {
  uint32_t reserved;
} thinkthen_request_literal_lines;

typedef struct thinkthen_request_literal_jsonl {
  uint32_t reserved;
} thinkthen_request_literal_jsonl;

typedef struct thinkthen_request_literal_csv {
  uint32_t reserved;
} thinkthen_request_literal_csv;

typedef struct thinkthen_request_literal_tsv {
  uint32_t reserved;
} thinkthen_request_literal_tsv;

typedef union thinkthen_request_framing_data {
  const struct thinkthen_request_literal_document *literal_document;
  const struct thinkthen_request_literal_lines *literal_lines;
  const struct thinkthen_request_literal_jsonl *literal_jsonl;
  const struct thinkthen_request_literal_csv *literal_csv;
  const struct thinkthen_request_literal_tsv *literal_tsv;
} thinkthen_request_framing_data;

typedef struct thinkthen_request_framing {
  uint32_t kind;
  union thinkthen_request_framing_data data;
} thinkthen_request_framing;

typedef struct thinkthen_request_literal_text {
  uint32_t reserved;
} thinkthen_request_literal_text;

typedef struct thinkthen_request_literal_image {
  uint32_t reserved;
} thinkthen_request_literal_image;

typedef union thinkthen_request_reader_media_data {
  const struct thinkthen_request_literal_text *literal_text;
  const struct thinkthen_request_literal_image *literal_image;
} thinkthen_request_reader_media_data;

typedef struct thinkthen_request_reader_media {
  uint32_t kind;
  union thinkthen_request_reader_media_data data;
} thinkthen_request_reader_media;

typedef struct thinkthen_request_literal_line {
  uint32_t reserved;
} thinkthen_request_literal_line;

typedef struct thinkthen_request_literal_window {
  uint32_t reserved;
} thinkthen_request_literal_window;

typedef struct thinkthen_request_literal_file {
  uint32_t reserved;
} thinkthen_request_literal_file;

typedef union thinkthen_request_source_unit_data {
  const struct thinkthen_request_literal_line *literal_line;
  const struct thinkthen_request_literal_window *literal_window;
  const struct thinkthen_request_literal_file *literal_file;
} thinkthen_request_source_unit_data;

typedef struct thinkthen_request_source_unit {
  uint32_t kind;
  union thinkthen_request_source_unit_data data;
} thinkthen_request_source_unit;

typedef struct thinkthen_request_reader {
  const struct thinkthen_request_source_unit *m_unit;
  const struct thinkthen_request_unsigned *m_window;
} thinkthen_request_reader;

typedef struct thinkthen_request_source {
  const struct thinkthen_request_framing *m_framing;
  const struct thinkthen_request_reader_media *m_media;
  const struct thinkthen_request_text_list *m_paths;
  const struct thinkthen_request_reader *m_reading;
} thinkthen_request_source;

typedef struct thinkthen_request_input_source {
  const struct thinkthen_request_source *m_source;
} thinkthen_request_input_source;

typedef struct thinkthen_request_input_feed {
  const struct thinkthen_request_framing *m_framing;
  const struct thinkthen_request_image_list *m_images;
  const struct thinkthen_request_text *m_name;
  const struct thinkthen_request_reader *m_reading;
} thinkthen_request_input_feed;

typedef union thinkthen_request_input_data {
  const struct thinkthen_request_input_text *text;
  const struct thinkthen_request_input_json *json;
  const struct thinkthen_request_input_records *records;
  const struct thinkthen_request_input_units *units;
  const struct thinkthen_request_input_entities *entities;
  const struct thinkthen_request_input_source *source;
  const struct thinkthen_request_input_feed *feed;
} thinkthen_request_input_data;

typedef struct thinkthen_request_input {
  uint32_t kind;
  union thinkthen_request_input_data data;
} thinkthen_request_input;

typedef struct thinkthen_request_integer {
  int64_t value;
} thinkthen_request_integer;

typedef struct thinkthen_request_option_schema_2_description {
  const struct thinkthen_complete_json *value;
} thinkthen_request_option_schema_2_description;

typedef struct thinkthen_request_option_schema_2 {
  const struct thinkthen_request_option_schema_2_description *m_description;
  const struct thinkthen_request_text *m_name;
} thinkthen_request_option_schema_2;

typedef struct thinkthen_request_option_schema_2_list {
  const struct thinkthen_request_option_schema_2 *const *data;
  size_t len;
} thinkthen_request_option_schema_2_list;

typedef struct thinkthen_request_literal_whole {
  uint32_t reserved;
} thinkthen_request_literal_whole;

typedef struct thinkthen_request_literal_boundary_only {
  uint32_t reserved;
} thinkthen_request_literal_boundary_only;

typedef union thinkthen_request_recognition_mode_data {
  const struct thinkthen_request_literal_whole *literal_whole;
  const struct thinkthen_request_literal_boundary_only *literal_boundary_only;
} thinkthen_request_recognition_mode_data;

typedef struct thinkthen_request_recognition_mode {
  uint32_t kind;
  union thinkthen_request_recognition_mode_data data;
} thinkthen_request_recognition_mode;

typedef struct thinkthen_request_call_options_no {
  const struct thinkthen_complete_json *value;
} thinkthen_request_call_options_no;

typedef struct thinkthen_request_number {
  double value;
} thinkthen_request_number;

typedef union thinkthen_request_call_threshold_data {
  const struct thinkthen_request_number *number;
  const struct thinkthen_request_text *text;
} thinkthen_request_call_threshold_data;

typedef struct thinkthen_request_call_threshold {
  uint32_t kind;
  union thinkthen_request_call_threshold_data data;
} thinkthen_request_call_threshold;

typedef struct thinkthen_request_recognition_stage_context {
  const struct thinkthen_request_text *m_boundary;
  const struct thinkthen_request_text *m_kind_edge;
  const struct thinkthen_request_text *m_relation;
} thinkthen_request_recognition_stage_context;

typedef struct thinkthen_request_call_options_yes {
  const struct thinkthen_complete_json *value;
} thinkthen_request_call_options_yes;

typedef struct thinkthen_request_call_options {
  const struct thinkthen_request_boolean *m_attempts;
  const struct thinkthen_request_batch *m_batch;
  const struct thinkthen_request_text *m_context;
  const struct thinkthen_request_text *m_context_field;
  const struct thinkthen_request_integer *m_deadline_ms;
  const struct thinkthen_request_text *m_entity_definition;
  const struct thinkthen_request_recognition_example_list *m_examples;
  const struct thinkthen_request_text *m_examples_field;
  const struct thinkthen_request_text_list *m_field;
  const struct thinkthen_request_boolean *m_files_only;
  const struct thinkthen_request_text *m_instructions;
  const struct thinkthen_request_option_schema_2_list *m_labels;
  const struct thinkthen_request_option_schema_2_list *m_levels;
  const struct thinkthen_request_unsigned *m_max_requests_total;
  const struct thinkthen_request_unsigned *m_max_text_bytes;
  const struct thinkthen_request_recognition_mode *m_mode;
  const struct thinkthen_request_text *m_model;
  const struct thinkthen_request_call_options_no *m_no;
  const struct thinkthen_request_boolean *m_none;
  const struct thinkthen_request_text *m_options_field;
  const struct thinkthen_request_call_threshold *m_relation_threshold;
  const struct thinkthen_request_recognition_seed_span_list *m_seed_spans;
  const struct thinkthen_request_text *m_seed_spans_field;
  const struct thinkthen_request_unsigned *m_snippet_pieces;
  const struct thinkthen_request_recognition_stage_context *m_stage_context;
  const struct thinkthen_request_call_threshold *m_threshold;
  const struct thinkthen_request_unsigned *m_top;
  const struct thinkthen_request_call_options_yes *m_yes;
} thinkthen_request_call_options;

typedef struct thinkthen_request_question_selector_text {
  const struct thinkthen_request_text *m_text;
} thinkthen_request_question_selector_text;

typedef struct thinkthen_request_literal_max {
  uint32_t reserved;
} thinkthen_request_literal_max;

typedef union thinkthen_request_question_batch_data {
  const struct thinkthen_request_literal_max *literal_max;
  const struct thinkthen_request_integer *integer;
} thinkthen_request_question_batch_data;

typedef struct thinkthen_request_question_batch {
  uint32_t kind;
  union thinkthen_request_question_batch_data data;
} thinkthen_request_question_batch;

typedef struct thinkthen_request_input_declaration_string {
  uint32_t reserved;
} thinkthen_request_input_declaration_string;

typedef struct thinkthen_request_input_property_string {
  uint32_t reserved;
} thinkthen_request_input_property_string;

typedef struct thinkthen_request_input_property_number {
  uint32_t reserved;
} thinkthen_request_input_property_number;

typedef struct thinkthen_request_input_property_boolean {
  uint32_t reserved;
} thinkthen_request_input_property_boolean;

typedef struct thinkthen_request_input_property_array_items {
  uint32_t reserved;
} thinkthen_request_input_property_array_items;

typedef struct thinkthen_request_input_property_array {
  const struct thinkthen_request_input_property_array_items *m_items;
} thinkthen_request_input_property_array;

typedef union thinkthen_request_input_property_data {
  const struct thinkthen_request_input_property_string *string;
  const struct thinkthen_request_input_property_number *number;
  const struct thinkthen_request_input_property_boolean *boolean;
  const struct thinkthen_request_input_property_array *array;
} thinkthen_request_input_property_data;

typedef struct thinkthen_request_input_property {
  uint32_t kind;
  union thinkthen_request_input_property_data data;
} thinkthen_request_input_property;

typedef struct thinkthen_request_input_property_map_entry {
  struct thinkthen_complete_utf8 key;
  const struct thinkthen_request_input_property *value;
} thinkthen_request_input_property_map_entry;

typedef struct thinkthen_request_input_property_map {
  const struct thinkthen_request_input_property_map_entry *data;
  size_t len;
} thinkthen_request_input_property_map;

typedef struct thinkthen_request_input_declaration_object {
  const struct thinkthen_request_input_property_map *m_properties;
  const struct thinkthen_request_text_list *m_required;
} thinkthen_request_input_declaration_object;

typedef union thinkthen_request_input_declaration_data {
  const struct thinkthen_request_input_declaration_string *string;
  const struct thinkthen_request_input_declaration_object *object;
} thinkthen_request_input_declaration_data;

typedef struct thinkthen_request_input_declaration {
  uint32_t kind;
  union thinkthen_request_input_declaration_data data;
} thinkthen_request_input_declaration;

typedef struct thinkthen_request_json_value_list {
  const struct thinkthen_request_json_value *const *data;
  size_t len;
} thinkthen_request_json_value_list;

typedef union thinkthen_request_question_text_data {
  const struct thinkthen_request_text *text;
  const struct thinkthen_request_json_value_map *json_value_map;
  const struct thinkthen_request_json_value_list *json_value_list;
} thinkthen_request_question_text_data;

typedef struct thinkthen_request_question_text {
  uint32_t kind;
  union thinkthen_request_question_text_data data;
} thinkthen_request_question_text;

typedef union thinkthen_request_criterion_data {
  const struct thinkthen_request_text *text;
  const struct thinkthen_request_json_value_map *json_value_map;
  const struct thinkthen_request_json_value_list *json_value_list;
  const struct thinkthen_request_null *null;
} thinkthen_request_criterion_data;

typedef struct thinkthen_request_criterion {
  uint32_t kind;
  union thinkthen_request_criterion_data data;
} thinkthen_request_criterion;

typedef struct thinkthen_request_name {
  struct thinkthen_complete_utf8 text;
} thinkthen_request_name;

typedef union thinkthen_request_pointers_data {
  const struct thinkthen_request_text *text;
  const struct thinkthen_request_text_list *text_list;
} thinkthen_request_pointers_data;

typedef struct thinkthen_request_pointers {
  uint32_t kind;
  union thinkthen_request_pointers_data data;
} thinkthen_request_pointers;

typedef struct thinkthen_request_profile {
  struct thinkthen_complete_utf8 text;
} thinkthen_request_profile;

typedef union thinkthen_request_threshold_data {
  const struct thinkthen_request_number *number;
  const struct thinkthen_request_text *text;
} thinkthen_request_threshold_data;

typedef struct thinkthen_request_threshold {
  uint32_t kind;
  union thinkthen_request_threshold_data data;
} thinkthen_request_threshold;

typedef struct thinkthen_request_definition_authored_decide {
  const struct thinkthen_request_question_batch *m_batch;
  const struct thinkthen_request_input_declaration *m_context_schema;
  const struct thinkthen_request_question_text *m_decide;
  const struct thinkthen_request_criterion *m_false;
  const struct thinkthen_request_input_declaration *m_item_schema;
  const struct thinkthen_request_name *m_model;
  const struct thinkthen_request_text *m_name;
  const struct thinkthen_request_pointers *m_on;
  const struct thinkthen_request_profile *m_profile;
  const struct thinkthen_request_threshold *m_threshold;
  const struct thinkthen_request_criterion *m_true;
} thinkthen_request_definition_authored_decide;

typedef struct thinkthen_request_name_list {
  const struct thinkthen_request_name *const *data;
  size_t len;
} thinkthen_request_name_list;

typedef union thinkthen_request_description_data {
  const struct thinkthen_request_text *text;
  const struct thinkthen_request_json_value_map *json_value_map;
  const struct thinkthen_request_json_value_list *json_value_list;
  const struct thinkthen_request_null *null;
} thinkthen_request_description_data;

typedef struct thinkthen_request_description {
  uint32_t kind;
  union thinkthen_request_description_data data;
} thinkthen_request_description;

typedef struct thinkthen_request_description_map_entry {
  struct thinkthen_complete_utf8 key;
  const struct thinkthen_request_description *value;
} thinkthen_request_description_map_entry;

typedef struct thinkthen_request_description_map {
  const struct thinkthen_request_description_map_entry *data;
  size_t len;
} thinkthen_request_description_map;

typedef union thinkthen_request_options_data {
  const struct thinkthen_request_name_list *name_list;
  const struct thinkthen_request_description_map *description_map;
} thinkthen_request_options_data;

typedef struct thinkthen_request_options {
  uint32_t kind;
  union thinkthen_request_options_data data;
} thinkthen_request_options;

typedef union thinkthen_request_cut_data {
  const struct thinkthen_request_number *number;
  const struct thinkthen_request_text *text;
} thinkthen_request_cut_data;

typedef struct thinkthen_request_cut {
  uint32_t kind;
  union thinkthen_request_cut_data data;
} thinkthen_request_cut;

typedef struct thinkthen_request_definition_authored_choose {
  const struct thinkthen_request_question_batch *m_batch;
  const struct thinkthen_request_question_text *m_choose;
  const struct thinkthen_request_input_declaration *m_context_schema;
  const struct thinkthen_request_input_declaration *m_item_schema;
  const struct thinkthen_request_name *m_model;
  const struct thinkthen_request_text *m_name;
  const struct thinkthen_request_pointers *m_on;
  const struct thinkthen_request_options *m_options;
  const struct thinkthen_request_profile *m_profile;
  const struct thinkthen_request_cut *m_threshold;
} thinkthen_request_definition_authored_choose;

typedef union thinkthen_request_labels_data {
  const struct thinkthen_request_name_list *name_list;
  const struct thinkthen_request_description_map *description_map;
} thinkthen_request_labels_data;

typedef struct thinkthen_request_labels {
  uint32_t kind;
  union thinkthen_request_labels_data data;
} thinkthen_request_labels;

typedef struct thinkthen_request_definition_authored_tag {
  const struct thinkthen_request_question_batch *m_batch;
  const struct thinkthen_request_input_declaration *m_context_schema;
  const struct thinkthen_request_input_declaration *m_item_schema;
  const struct thinkthen_request_labels *m_labels;
  const struct thinkthen_request_name *m_model;
  const struct thinkthen_request_text *m_name;
  const struct thinkthen_request_pointers *m_on;
  const struct thinkthen_request_profile *m_profile;
  const struct thinkthen_request_question_text *m_tag;
  const struct thinkthen_request_cut *m_threshold;
} thinkthen_request_definition_authored_tag;

typedef struct thinkthen_request_criterion_map_entry {
  struct thinkthen_complete_utf8 key;
  const struct thinkthen_request_criterion *value;
} thinkthen_request_criterion_map_entry;

typedef struct thinkthen_request_criterion_map {
  const struct thinkthen_request_criterion_map_entry *data;
  size_t len;
} thinkthen_request_criterion_map;

typedef union thinkthen_request_levels_data {
  const struct thinkthen_request_name_list *name_list;
  const struct thinkthen_request_criterion_map *criterion_map;
} thinkthen_request_levels_data;

typedef struct thinkthen_request_levels {
  uint32_t kind;
  union thinkthen_request_levels_data data;
} thinkthen_request_levels;

typedef struct thinkthen_request_definition_authored_score {
  const struct thinkthen_request_question_batch *m_batch;
  const struct thinkthen_request_input_declaration *m_context_schema;
  const struct thinkthen_request_input_declaration *m_item_schema;
  const struct thinkthen_request_levels *m_levels;
  const struct thinkthen_request_name *m_model;
  const struct thinkthen_request_text *m_name;
  const struct thinkthen_request_pointers *m_on;
  const struct thinkthen_request_profile *m_profile;
  const struct thinkthen_request_question_text *m_score;
} thinkthen_request_definition_authored_score;

typedef struct thinkthen_request_relate_fields {
  const struct thinkthen_request_text *m_kind;
  const struct thinkthen_request_text *m_name;
} thinkthen_request_relate_fields;

typedef struct thinkthen_request_relation {
  const struct thinkthen_request_boolean *m_either;
  const struct thinkthen_request_name *m_name;
  const struct thinkthen_request_name *m_reads;
  const struct thinkthen_request_boolean *m_single;
  const struct thinkthen_request_name *m_source;
  const struct thinkthen_request_name *m_target;
} thinkthen_request_relation;

typedef struct thinkthen_request_relation_list {
  const struct thinkthen_request_relation *const *data;
  size_t len;
} thinkthen_request_relation_list;

typedef struct thinkthen_request_relate_question {
  const struct thinkthen_request_relate_fields *m_fields;
  const struct thinkthen_request_relation_list *m_relations;
} thinkthen_request_relate_question;

typedef struct thinkthen_request_definition_authored_relate {
  const struct thinkthen_request_input_declaration *m_context_schema;
  const struct thinkthen_request_input_declaration *m_item_schema;
  const struct thinkthen_request_name *m_model;
  const struct thinkthen_request_text *m_name;
  const struct thinkthen_request_profile *m_profile;
  const struct thinkthen_request_relate_question *m_relate;
  const struct thinkthen_request_cut *m_threshold;
} thinkthen_request_definition_authored_relate;

typedef struct thinkthen_request_definition_authored_find {
  const struct thinkthen_request_input_declaration *m_context_schema;
  const struct thinkthen_request_question_text *m_find;
  const struct thinkthen_request_input_declaration *m_item_schema;
  const struct thinkthen_request_name *m_model;
  const struct thinkthen_request_text *m_name;
  const struct thinkthen_request_pointers *m_on;
  const struct thinkthen_request_profile *m_profile;
} thinkthen_request_definition_authored_find;

typedef struct thinkthen_request_recognize_question {
  const struct thinkthen_request_question_text *m_entity_definition;
  const struct thinkthen_request_question_text *m_instructions;
  const struct thinkthen_request_description_map *m_kinds;
  const struct thinkthen_request_recognition_mode *m_mode;
  const struct thinkthen_request_relation_list *m_relations;
  const struct thinkthen_request_unsigned *m_snippet_pieces;
  const struct thinkthen_request_recognition_stage_context *m_stage_context;
} thinkthen_request_recognize_question;

typedef struct thinkthen_request_definition_recognize_definition {
  const struct thinkthen_request_input_declaration *m_context_schema;
  const struct thinkthen_request_input_declaration *m_item_schema;
  const struct thinkthen_request_name *m_model;
  const struct thinkthen_request_text *m_name;
  const struct thinkthen_request_pointers *m_on;
  const struct thinkthen_request_profile *m_profile;
  const struct thinkthen_request_recognize_question *m_recognize;
  const struct thinkthen_request_cut *m_relation_threshold;
  const struct thinkthen_request_cut *m_threshold;
} thinkthen_request_definition_recognize_definition;

typedef struct thinkthen_request_annotate_question_decide {
  const struct thinkthen_request_input_declaration *m_context_schema;
  const struct thinkthen_request_question_text *m_decide;
  const struct thinkthen_request_criterion *m_false;
  const struct thinkthen_request_input_declaration *m_item_schema;
  const struct thinkthen_request_text *m_name;
  const struct thinkthen_request_pointers *m_on;
  const struct thinkthen_request_threshold *m_threshold;
  const struct thinkthen_request_criterion *m_true;
} thinkthen_request_annotate_question_decide;

typedef struct thinkthen_request_annotate_question_choose {
  const struct thinkthen_request_question_text *m_choose;
  const struct thinkthen_request_input_declaration *m_context_schema;
  const struct thinkthen_request_input_declaration *m_item_schema;
  const struct thinkthen_request_text *m_name;
  const struct thinkthen_request_pointers *m_on;
  const struct thinkthen_request_options *m_options;
  const struct thinkthen_request_cut *m_threshold;
} thinkthen_request_annotate_question_choose;

typedef struct thinkthen_request_annotate_question_tag {
  const struct thinkthen_request_input_declaration *m_context_schema;
  const struct thinkthen_request_input_declaration *m_item_schema;
  const struct thinkthen_request_labels *m_labels;
  const struct thinkthen_request_text *m_name;
  const struct thinkthen_request_pointers *m_on;
  const struct thinkthen_request_question_text *m_tag;
  const struct thinkthen_request_cut *m_threshold;
} thinkthen_request_annotate_question_tag;

typedef struct thinkthen_request_annotate_question_score {
  const struct thinkthen_request_input_declaration *m_context_schema;
  const struct thinkthen_request_input_declaration *m_item_schema;
  const struct thinkthen_request_levels *m_levels;
  const struct thinkthen_request_text *m_name;
  const struct thinkthen_request_pointers *m_on;
  const struct thinkthen_request_question_text *m_score;
} thinkthen_request_annotate_question_score;

typedef union thinkthen_request_annotate_question_data {
  const struct thinkthen_request_annotate_question_decide *decide;
  const struct thinkthen_request_annotate_question_choose *choose;
  const struct thinkthen_request_annotate_question_tag *tag;
  const struct thinkthen_request_annotate_question_score *score;
} thinkthen_request_annotate_question_data;

typedef struct thinkthen_request_annotate_question {
  uint32_t kind;
  union thinkthen_request_annotate_question_data data;
} thinkthen_request_annotate_question;

typedef struct thinkthen_request_annotate_question_map_entry {
  struct thinkthen_complete_utf8 key;
  const struct thinkthen_request_annotate_question *value;
} thinkthen_request_annotate_question_map_entry;

typedef struct thinkthen_request_annotate_question_map {
  const struct thinkthen_request_annotate_question_map_entry *data;
  size_t len;
} thinkthen_request_annotate_question_map;

typedef struct thinkthen_request_definition_annotate_definition {
  const struct thinkthen_request_json_value *m_batch;
  const struct thinkthen_request_profile *m_profile;
  const struct thinkthen_request_annotate_question_map *m_questions;
  const struct thinkthen_request_threshold *m_threshold;
} thinkthen_request_definition_annotate_definition;

typedef union thinkthen_request_definition_data {
  const struct thinkthen_request_definition_authored_decide *authored_decide;
  const struct thinkthen_request_definition_authored_choose *authored_choose;
  const struct thinkthen_request_definition_authored_tag *authored_tag;
  const struct thinkthen_request_definition_authored_score *authored_score;
  const struct thinkthen_request_definition_authored_relate *authored_relate;
  const struct thinkthen_request_definition_authored_find *authored_find;
  const struct thinkthen_request_definition_recognize_definition *recognize_definition;
  const struct thinkthen_request_definition_annotate_definition *annotate_definition;
} thinkthen_request_definition_data;

typedef struct thinkthen_request_definition {
  uint32_t kind;
  union thinkthen_request_definition_data data;
} thinkthen_request_definition;

typedef struct thinkthen_request_question_selector_definition {
  const struct thinkthen_request_definition *m_value;
} thinkthen_request_question_selector_definition;

typedef struct thinkthen_request_question_selector_file {
  const struct thinkthen_request_text *m_path;
} thinkthen_request_question_selector_file;

typedef struct thinkthen_request_question_selector_name {
  const struct thinkthen_request_text *m_name;
} thinkthen_request_question_selector_name;

typedef struct thinkthen_request_question_selector_reference {
  const struct thinkthen_request_text *m_reference;
} thinkthen_request_question_selector_reference;

typedef union thinkthen_request_question_selector_data {
  const struct thinkthen_request_question_selector_text *text;
  const struct thinkthen_request_question_selector_definition *definition;
  const struct thinkthen_request_question_selector_file *file;
  const struct thinkthen_request_question_selector_name *name;
  const struct thinkthen_request_question_selector_reference *reference;
} thinkthen_request_question_selector_data;

typedef struct thinkthen_request_question_selector {
  uint32_t kind;
  union thinkthen_request_question_selector_data data;
} thinkthen_request_question_selector;

typedef struct thinkthen_request_arguments {
  const struct thinkthen_request_input *m_input;
  const struct thinkthen_request_call_options *m_options;
  const struct thinkthen_request_question_selector *m_question;
} thinkthen_request_arguments;

/*
 A producer returns ITEM and counted RequestItem JSON, END, or FAILURE.
 Bytes remain readable until the next invocation or until run_call returns.
 It runs synchronously on the run_call caller, and may reenter independent C operations.
 Never unwind or throw across this callback; retain private host exceptions locally.
 */
typedef uint32_t (*thinkthen_call_producer)(void*, struct thinkthen_complete_utf8*);

typedef struct thinkthen_request_reader_failure_producer {
  uint32_t reserved;
} thinkthen_request_reader_failure_producer;

typedef struct thinkthen_request_session_source_location {
  const struct thinkthen_request_text *m_file;
  const struct thinkthen_request_unsigned *m_first_line;
  const struct thinkthen_request_unsigned *m_last_line;
} thinkthen_request_session_source_location;

typedef struct thinkthen_request_reader_failure_io {
  const struct thinkthen_request_session_source_location *m_location;
} thinkthen_request_reader_failure_io;

typedef struct thinkthen_request_reader_failure_utf_8 {
  const struct thinkthen_request_session_source_location *m_location;
} thinkthen_request_reader_failure_utf_8;

typedef struct thinkthen_request_reader_failure_invalid_input {
  const struct thinkthen_request_session_source_location *m_location;
} thinkthen_request_reader_failure_invalid_input;

typedef union thinkthen_request_reader_failure_data {
  const struct thinkthen_request_reader_failure_producer *producer;
  const struct thinkthen_request_reader_failure_io *io;
  const struct thinkthen_request_reader_failure_utf_8 *utf_8;
  const struct thinkthen_request_reader_failure_invalid_input *invalid_input;
} thinkthen_request_reader_failure_data;

typedef struct thinkthen_request_reader_failure {
  uint32_t kind;
  union thinkthen_request_reader_failure_data data;
} thinkthen_request_reader_failure;

typedef struct thinkthen_request_session_descriptor {
  const struct thinkthen_request_item *m_item;
  const struct thinkthen_request_session_source_location *m_location;
} thinkthen_request_session_descriptor;

typedef struct thinkthen_complete_answer_yes_no {
  struct thinkthen_complete_utf8 kind;
  double probability;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_answer_yes_no;

typedef struct thinkthen_complete_f64_presence {
  uint32_t presence;
  double value;
} thinkthen_complete_f64_presence;

typedef struct thinkthen_complete_number_map_entry {
  struct thinkthen_complete_utf8 name;
  double value;
} thinkthen_complete_number_map_entry;

typedef struct thinkthen_complete_number_map {
  const struct thinkthen_complete_number_map_entry *data;
  size_t len;
} thinkthen_complete_number_map;

typedef struct thinkthen_complete_answer_choice {
  struct thinkthen_complete_f64_presence confidence;
  struct thinkthen_complete_utf8 kind;
  struct thinkthen_complete_utf8 pick;
  struct thinkthen_complete_number_map probabilities;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_answer_choice;

typedef struct thinkthen_complete_answer_tag {
  struct thinkthen_complete_utf8 kind;
  struct thinkthen_complete_number_map probabilities;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_answer_tag;

typedef struct thinkthen_complete_answer_score {
  struct thinkthen_complete_f64_presence confidence;
  struct thinkthen_complete_utf8 kind;
  struct thinkthen_complete_utf8 level;
  struct thinkthen_complete_number_map probabilities;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_answer_score;

typedef union thinkthen_complete_answer_data {
  const struct thinkthen_complete_answer_yes_no *yes_no;
  const struct thinkthen_complete_answer_choice *choice;
  const struct thinkthen_complete_answer_tag *tag;
  const struct thinkthen_complete_answer_score *score;
} thinkthen_complete_answer_data;

typedef struct thinkthen_complete_answer {
  uint32_t kind;
  union thinkthen_complete_answer_data data;
} thinkthen_complete_answer;

typedef struct thinkthen_complete_answer_id {
  struct thinkthen_complete_utf8 value;
} thinkthen_complete_answer_id;

typedef struct thinkthen_complete_image_media {
  uint32_t kind;
} thinkthen_complete_image_media;

typedef struct thinkthen_complete_image {
  struct thinkthen_complete_utf8 base64;
  uint64_t height;
  const struct thinkthen_complete_image_media *media;
  uint64_t width;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_image;

typedef struct thinkthen_complete_image_list {
  const struct thinkthen_complete_image *const *data;
  size_t len;
} thinkthen_complete_image_list;

typedef struct thinkthen_complete_image_list_presence {
  uint32_t presence;
  struct thinkthen_complete_image_list value;
} thinkthen_complete_image_list_presence;

typedef struct thinkthen_complete_json_presence {
  uint32_t presence;
  const struct thinkthen_complete_json *value;
} thinkthen_complete_json_presence;

typedef union thinkthen_complete_batch_setting_data {
  uint64_t integer;
  struct thinkthen_complete_utf8 string;
} thinkthen_complete_batch_setting_data;

typedef struct thinkthen_complete_batch_setting {
  uint32_t kind;
  union thinkthen_complete_batch_setting_data data;
} thinkthen_complete_batch_setting;

typedef struct thinkthen_complete_batch_setting_presence {
  uint32_t presence;
  const struct thinkthen_complete_batch_setting *value;
} thinkthen_complete_batch_setting_presence;

typedef struct thinkthen_complete_batch_warning {
  const struct thinkthen_complete_batch_setting *running;
  const struct thinkthen_complete_batch_setting *tuned_for;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_batch_warning;

typedef struct thinkthen_complete_batch_warning_presence {
  uint32_t presence;
  const struct thinkthen_complete_batch_warning *value;
} thinkthen_complete_batch_warning_presence;

typedef struct thinkthen_complete_observation_id {
  struct thinkthen_complete_utf8 value;
} thinkthen_complete_observation_id;

typedef struct thinkthen_complete_observation_observation_id {
  const struct thinkthen_complete_observation_id *observation_id;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_observation_observation_id;

typedef struct thinkthen_complete_failure_id {
  struct thinkthen_complete_utf8 value;
} thinkthen_complete_failure_id;

typedef struct thinkthen_complete_observation_failure_id {
  const struct thinkthen_complete_failure_id *failure_id;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_observation_failure_id;

typedef union thinkthen_complete_observation_data {
  const struct thinkthen_complete_observation_observation_id *observation_id;
  const struct thinkthen_complete_observation_failure_id *failure_id;
} thinkthen_complete_observation_data;

typedef struct thinkthen_complete_observation {
  uint32_t kind;
  union thinkthen_complete_observation_data data;
} thinkthen_complete_observation;

typedef struct thinkthen_complete_observation_list {
  const struct thinkthen_complete_observation *const *data;
  size_t len;
} thinkthen_complete_observation_list;

typedef struct thinkthen_complete_origin {
  uint32_t kind;
} thinkthen_complete_origin;

typedef struct thinkthen_complete_origin_presence {
  uint32_t presence;
  const struct thinkthen_complete_origin *value;
} thinkthen_complete_origin_presence;

typedef struct thinkthen_complete_profile_warning {
  struct thinkthen_complete_utf8 running;
  struct thinkthen_complete_utf8 tuned_for;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_profile_warning;

typedef struct thinkthen_complete_profile_warning_presence {
  uint32_t presence;
  const struct thinkthen_complete_profile_warning *value;
} thinkthen_complete_profile_warning_presence;

typedef struct thinkthen_complete_question_source {
  struct thinkthen_complete_utf8 answered_by;
  struct thinkthen_complete_u64_presence batch_size;
  const struct thinkthen_complete_origin *origin;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_question_source;

typedef struct thinkthen_complete_question_source_list {
  const struct thinkthen_complete_question_source *const *data;
  size_t len;
} thinkthen_complete_question_source_list;

typedef struct thinkthen_complete_string_list {
  const struct thinkthen_complete_utf8 *data;
  size_t len;
} thinkthen_complete_string_list;

typedef struct thinkthen_complete_usage {
  struct thinkthen_complete_u64_presence input_tokens;
  struct thinkthen_complete_u64_presence output_tokens;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_usage;

typedef struct thinkthen_complete_usage_presence {
  uint32_t presence;
  const struct thinkthen_complete_usage *value;
} thinkthen_complete_usage_presence;

typedef struct thinkthen_complete_meta {
  struct thinkthen_complete_utf8_presence answered_by;
  struct thinkthen_complete_attempt_list_presence attempts;
  struct thinkthen_complete_batch_setting_presence batch_setting;
  struct thinkthen_complete_batch_warning_presence batch_warning;
  uint32_t cached;
  struct thinkthen_complete_utf8_presence context_sha256;
  uint64_t failed_questions;
  struct thinkthen_complete_utf8 model;
  struct thinkthen_complete_observation_list observations;
  struct thinkthen_complete_origin_presence origin;
  struct thinkthen_complete_profile_warning_presence profile_warning;
  struct thinkthen_complete_utf8_presence question_sha256;
  struct thinkthen_complete_question_source_list question_sources;
  struct thinkthen_complete_utf8_presence questions_sha256;
  struct thinkthen_complete_string_list requests;
  uint64_t requests_sent;
  struct thinkthen_complete_utf8 tool;
  struct thinkthen_complete_utf8 url;
  struct thinkthen_complete_usage_presence usage;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_meta;

typedef struct thinkthen_complete_batch_string {
  uint32_t kind;
} thinkthen_complete_batch_string;

typedef union thinkthen_complete_batch_data {
  uint64_t integer;
  const struct thinkthen_complete_batch_string *string;
} thinkthen_complete_batch_data;

typedef struct thinkthen_complete_batch {
  uint32_t kind;
  union thinkthen_complete_batch_data data;
} thinkthen_complete_batch;

typedef struct thinkthen_complete_batch_presence {
  uint32_t presence;
  const struct thinkthen_complete_batch *value;
} thinkthen_complete_batch_presence;

typedef struct thinkthen_complete_string_type {
  uint32_t kind;
} thinkthen_complete_string_type;

typedef struct thinkthen_complete_input_declaration_string_root {
  const struct thinkthen_complete_string_type *type;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_input_declaration_string_root;

typedef struct thinkthen_complete_input_property_type_string {
  struct thinkthen_complete_utf8 type;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_input_property_type_string;

typedef struct thinkthen_complete_input_property_type_number {
  struct thinkthen_complete_utf8 type;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_input_property_type_number;

typedef struct thinkthen_complete_input_property_type_boolean {
  struct thinkthen_complete_utf8 type;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_input_property_type_boolean;

typedef struct thinkthen_complete_string_root {
  const struct thinkthen_complete_string_type *type;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_string_root;

typedef struct thinkthen_complete_input_property_type_array {
  const struct thinkthen_complete_string_root *items;
  struct thinkthen_complete_utf8 type;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_input_property_type_array;

typedef union thinkthen_complete_input_property_type_data {
  const struct thinkthen_complete_input_property_type_string *string;
  const struct thinkthen_complete_input_property_type_number *number;
  const struct thinkthen_complete_input_property_type_boolean *boolean;
  const struct thinkthen_complete_input_property_type_array *array;
} thinkthen_complete_input_property_type_data;

typedef struct thinkthen_complete_input_property_type {
  uint32_t kind;
  union thinkthen_complete_input_property_type_data data;
} thinkthen_complete_input_property_type;

typedef struct thinkthen_complete_input_property_type_map_entry {
  struct thinkthen_complete_utf8 name;
  const struct thinkthen_complete_input_property_type *value;
} thinkthen_complete_input_property_type_map_entry;

typedef struct thinkthen_complete_input_property_type_map {
  const struct thinkthen_complete_input_property_type_map_entry *data;
  size_t len;
} thinkthen_complete_input_property_type_map;

typedef struct thinkthen_complete_string_list_presence {
  uint32_t presence;
  struct thinkthen_complete_string_list value;
} thinkthen_complete_string_list_presence;

typedef struct thinkthen_complete_object_type {
  uint32_t kind;
} thinkthen_complete_object_type;

typedef struct thinkthen_complete_input_declaration_object_root {
  struct thinkthen_complete_input_property_type_map properties;
  struct thinkthen_complete_string_list_presence required;
  const struct thinkthen_complete_object_type *type;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_input_declaration_object_root;

typedef union thinkthen_complete_input_declaration_data {
  const struct thinkthen_complete_input_declaration_string_root *string_root;
  const struct thinkthen_complete_input_declaration_object_root *object_root;
} thinkthen_complete_input_declaration_data;

typedef struct thinkthen_complete_input_declaration {
  uint32_t kind;
  union thinkthen_complete_input_declaration_data data;
} thinkthen_complete_input_declaration;

typedef struct thinkthen_complete_input_declaration_presence {
  uint32_t presence;
  const struct thinkthen_complete_input_declaration *value;
} thinkthen_complete_input_declaration_presence;

typedef struct thinkthen_complete_label {
  struct thinkthen_complete_json_presence description;
  struct thinkthen_complete_utf8 name;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_label;

typedef struct thinkthen_complete_label_list {
  const struct thinkthen_complete_label *const *data;
  size_t len;
} thinkthen_complete_label_list;

typedef struct thinkthen_complete_label_list_presence {
  uint32_t presence;
  struct thinkthen_complete_label_list value;
} thinkthen_complete_label_list_presence;

typedef struct thinkthen_complete_question_name {
  struct thinkthen_complete_utf8 value;
} thinkthen_complete_question_name;

typedef struct thinkthen_complete_question_name_presence {
  uint32_t presence;
  const struct thinkthen_complete_question_name *value;
} thinkthen_complete_question_name_presence;

typedef struct thinkthen_complete_question_document_decide {
  struct thinkthen_complete_batch_presence batch;
  struct thinkthen_complete_input_declaration_presence context_schema;
  struct thinkthen_complete_input_declaration_presence item_schema;
  struct thinkthen_complete_label_list_presence label_details;
  struct thinkthen_complete_utf8_presence model;
  struct thinkthen_complete_question_name_presence name;
  struct thinkthen_complete_string_list_presence on;
  struct thinkthen_complete_utf8_presence profile;
  struct thinkthen_complete_json_presence false_;
  struct thinkthen_complete_json_presence text;
  struct thinkthen_complete_json_presence true_;
  struct thinkthen_complete_utf8 verb;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_question_document_decide;

typedef struct thinkthen_complete_question_document_choose {
  struct thinkthen_complete_batch_presence batch;
  struct thinkthen_complete_input_declaration_presence context_schema;
  struct thinkthen_complete_input_declaration_presence item_schema;
  struct thinkthen_complete_label_list_presence label_details;
  struct thinkthen_complete_utf8_presence model;
  struct thinkthen_complete_question_name_presence name;
  struct thinkthen_complete_string_list_presence on;
  struct thinkthen_complete_utf8_presence profile;
  struct thinkthen_complete_string_list options;
  struct thinkthen_complete_json_presence text;
  struct thinkthen_complete_utf8 verb;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_question_document_choose;

typedef struct thinkthen_complete_question_document_tag {
  struct thinkthen_complete_batch_presence batch;
  struct thinkthen_complete_input_declaration_presence context_schema;
  struct thinkthen_complete_input_declaration_presence item_schema;
  struct thinkthen_complete_label_list_presence label_details;
  struct thinkthen_complete_utf8_presence model;
  struct thinkthen_complete_question_name_presence name;
  struct thinkthen_complete_string_list_presence on;
  struct thinkthen_complete_utf8_presence profile;
  struct thinkthen_complete_string_list labels;
  struct thinkthen_complete_json_presence text;
  struct thinkthen_complete_utf8 verb;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_question_document_tag;

typedef struct thinkthen_complete_question_document_score {
  struct thinkthen_complete_batch_presence batch;
  struct thinkthen_complete_input_declaration_presence context_schema;
  struct thinkthen_complete_input_declaration_presence item_schema;
  struct thinkthen_complete_label_list_presence label_details;
  struct thinkthen_complete_utf8_presence model;
  struct thinkthen_complete_question_name_presence name;
  struct thinkthen_complete_string_list_presence on;
  struct thinkthen_complete_utf8_presence profile;
  struct thinkthen_complete_string_list levels;
  struct thinkthen_complete_json_presence text;
  struct thinkthen_complete_utf8 verb;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_question_document_score;

typedef union thinkthen_complete_question_document_data {
  const struct thinkthen_complete_question_document_decide *decide;
  const struct thinkthen_complete_question_document_choose *choose;
  const struct thinkthen_complete_question_document_tag *tag;
  const struct thinkthen_complete_question_document_score *score;
} thinkthen_complete_question_document_data;

typedef struct thinkthen_complete_question_document {
  uint32_t kind;
  union thinkthen_complete_question_document_data data;
} thinkthen_complete_question_document;

typedef struct thinkthen_complete_result_tag {
  uint32_t kind;
} thinkthen_complete_result_tag;

typedef struct thinkthen_complete_physical_source {
  struct thinkthen_complete_utf8 file;
  struct thinkthen_complete_u64_presence first_line;
  struct thinkthen_complete_u64_presence last_line;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_physical_source;

typedef struct thinkthen_complete_physical_source_presence {
  uint32_t presence;
  const struct thinkthen_complete_physical_source *value;
} thinkthen_complete_physical_source_presence;

typedef struct thinkthen_complete_rank_member_result {
  const struct thinkthen_complete_answer *answer;
  const struct thinkthen_complete_answer_id *answer_id;
  struct thinkthen_complete_image_list_presence images;
  const struct thinkthen_complete_meta *meta;
  const struct thinkthen_complete_question_document *question;
  const struct thinkthen_complete_result_tag *schema;
  struct thinkthen_complete_physical_source_presence source;
  struct thinkthen_complete_json_presence threshold;
  uint64_t value;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_rank_member_result;

typedef struct thinkthen_complete_rank_member {
  struct thinkthen_complete_utf8 name;
  const struct thinkthen_complete_rank_member_result *result;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_rank_member;

typedef struct thinkthen_complete_rank_member_list {
  const struct thinkthen_complete_rank_member *const *data;
  size_t len;
} thinkthen_complete_rank_member_list;

typedef struct thinkthen_complete_rank_member_list_presence {
  uint32_t presence;
  struct thinkthen_complete_rank_member_list value;
} thinkthen_complete_rank_member_list_presence;

typedef union thinkthen_complete_threshold_data {
  double number;
  struct thinkthen_complete_utf8 string;
} thinkthen_complete_threshold_data;

typedef struct thinkthen_complete_threshold {
  uint32_t kind;
  union thinkthen_complete_threshold_data data;
} thinkthen_complete_threshold;

typedef struct thinkthen_complete_threshold_presence {
  uint32_t presence;
  const struct thinkthen_complete_threshold *value;
} thinkthen_complete_threshold_presence;

typedef struct thinkthen_complete_decide_value {
  const struct thinkthen_complete_json *value;
} thinkthen_complete_decide_value;

typedef struct thinkthen_complete_decide_value_presence {
  uint32_t presence;
  const struct thinkthen_complete_decide_value *value;
} thinkthen_complete_decide_value_presence;

typedef struct thinkthen_complete_atomic_decide_value {
  const struct thinkthen_complete_answer *answer;
  const struct thinkthen_complete_answer_id *answer_id;
  struct thinkthen_complete_image_list_presence images;
  struct thinkthen_complete_u64_presence index;
  struct thinkthen_complete_json_presence input;
  struct thinkthen_complete_rank_member_list_presence members;
  const struct thinkthen_complete_meta *meta;
  const struct thinkthen_complete_question_document *question;
  struct thinkthen_complete_utf8_presence question_name;
  const struct thinkthen_complete_result_tag *schema;
  struct thinkthen_complete_physical_source_presence source;
  struct thinkthen_complete_threshold_presence threshold;
  struct thinkthen_complete_decide_value_presence value;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_atomic_decide_value;

typedef struct thinkthen_complete_session_packet_decide_row {
  struct thinkthen_complete_utf8 function;
  struct thinkthen_complete_utf8 kind;
  const struct thinkthen_complete_atomic_decide_value *value;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_session_packet_decide_row;

typedef struct thinkthen_complete_atomic_nullable_string {
  const struct thinkthen_complete_answer *answer;
  const struct thinkthen_complete_answer_id *answer_id;
  struct thinkthen_complete_image_list_presence images;
  struct thinkthen_complete_u64_presence index;
  struct thinkthen_complete_json_presence input;
  struct thinkthen_complete_rank_member_list_presence members;
  const struct thinkthen_complete_meta *meta;
  const struct thinkthen_complete_question_document *question;
  struct thinkthen_complete_utf8_presence question_name;
  const struct thinkthen_complete_result_tag *schema;
  struct thinkthen_complete_physical_source_presence source;
  struct thinkthen_complete_threshold_presence threshold;
  struct thinkthen_complete_utf8_presence value;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_atomic_nullable_string;

typedef struct thinkthen_complete_session_packet_choose_row {
  struct thinkthen_complete_utf8 function;
  struct thinkthen_complete_utf8 kind;
  const struct thinkthen_complete_atomic_nullable_string *value;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_session_packet_choose_row;

typedef struct thinkthen_complete_atomic_array_of_string {
  const struct thinkthen_complete_answer *answer;
  const struct thinkthen_complete_answer_id *answer_id;
  struct thinkthen_complete_image_list_presence images;
  struct thinkthen_complete_u64_presence index;
  struct thinkthen_complete_json_presence input;
  struct thinkthen_complete_rank_member_list_presence members;
  const struct thinkthen_complete_meta *meta;
  const struct thinkthen_complete_question_document *question;
  struct thinkthen_complete_utf8_presence question_name;
  const struct thinkthen_complete_result_tag *schema;
  struct thinkthen_complete_physical_source_presence source;
  struct thinkthen_complete_threshold_presence threshold;
  struct thinkthen_complete_string_list value;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_atomic_array_of_string;

typedef struct thinkthen_complete_session_packet_tag_row {
  struct thinkthen_complete_utf8 function;
  struct thinkthen_complete_utf8 kind;
  const struct thinkthen_complete_atomic_array_of_string *value;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_session_packet_tag_row;

typedef struct thinkthen_complete_atomic_double {
  const struct thinkthen_complete_answer *answer;
  const struct thinkthen_complete_answer_id *answer_id;
  struct thinkthen_complete_image_list_presence images;
  struct thinkthen_complete_u64_presence index;
  struct thinkthen_complete_json_presence input;
  struct thinkthen_complete_rank_member_list_presence members;
  const struct thinkthen_complete_meta *meta;
  const struct thinkthen_complete_question_document *question;
  struct thinkthen_complete_utf8_presence question_name;
  const struct thinkthen_complete_result_tag *schema;
  struct thinkthen_complete_physical_source_presence source;
  struct thinkthen_complete_threshold_presence threshold;
  double value;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_atomic_double;

typedef struct thinkthen_complete_session_packet_score_row {
  struct thinkthen_complete_utf8 function;
  struct thinkthen_complete_utf8 kind;
  const struct thinkthen_complete_atomic_double *value;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_session_packet_score_row;

typedef struct thinkthen_complete_atomic_boolean {
  const struct thinkthen_complete_answer *answer;
  const struct thinkthen_complete_answer_id *answer_id;
  struct thinkthen_complete_image_list_presence images;
  struct thinkthen_complete_u64_presence index;
  struct thinkthen_complete_json_presence input;
  struct thinkthen_complete_rank_member_list_presence members;
  const struct thinkthen_complete_meta *meta;
  const struct thinkthen_complete_question_document *question;
  struct thinkthen_complete_utf8_presence question_name;
  const struct thinkthen_complete_result_tag *schema;
  struct thinkthen_complete_physical_source_presence source;
  struct thinkthen_complete_threshold_presence threshold;
  uint32_t value;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_atomic_boolean;

typedef struct thinkthen_complete_session_packet_filter_row {
  struct thinkthen_complete_utf8 function;
  struct thinkthen_complete_utf8 kind;
  const struct thinkthen_complete_atomic_boolean *value;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_session_packet_filter_row;

typedef union thinkthen_complete_value_data {
  uint32_t boolean;
  uint32_t null;
  struct thinkthen_complete_utf8 string;
  struct thinkthen_complete_string_list array;
  double number;
} thinkthen_complete_value_data;

typedef struct thinkthen_complete_value {
  uint32_t kind;
  union thinkthen_complete_value_data data;
} thinkthen_complete_value;

typedef struct thinkthen_complete_value_presence {
  uint32_t presence;
  const struct thinkthen_complete_value *value;
} thinkthen_complete_value_presence;

typedef struct thinkthen_complete_annotation_member_answer_id {
  const struct thinkthen_complete_answer *answer;
  const struct thinkthen_complete_answer_id *answer_id;
  struct thinkthen_complete_observation_list observations;
  const struct thinkthen_complete_question_document *question;
  struct thinkthen_complete_question_source_list question_sources;
  struct thinkthen_complete_utf8 request;
  struct thinkthen_complete_threshold_presence threshold;
  struct thinkthen_complete_usage_presence usage;
  struct thinkthen_complete_value_presence value;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_annotation_member_answer_id;

typedef struct thinkthen_complete_failure_cause {
  uint32_t kind;
} thinkthen_complete_failure_cause;

typedef struct thinkthen_complete_failure_field_kind {
  uint32_t kind;
} thinkthen_complete_failure_field_kind;

typedef struct thinkthen_complete_failure {
  const struct thinkthen_complete_failure_cause *cause;
  const struct thinkthen_complete_failure_field_kind *kind;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_failure;

typedef struct thinkthen_complete_annotation_member_failure_id {
  const struct thinkthen_complete_failure *failure;
  const struct thinkthen_complete_failure_id *failure_id;
  struct thinkthen_complete_observation_list observations;
  const struct thinkthen_complete_question_document *question;
  struct thinkthen_complete_question_source_list question_sources;
  struct thinkthen_complete_utf8 request;
  struct thinkthen_complete_threshold_presence threshold;
  struct thinkthen_complete_usage_presence usage;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_annotation_member_failure_id;

typedef union thinkthen_complete_annotation_member_data {
  const struct thinkthen_complete_annotation_member_answer_id *answer_id;
  const struct thinkthen_complete_annotation_member_failure_id *failure_id;
} thinkthen_complete_annotation_member_data;

typedef struct thinkthen_complete_annotation_member {
  uint32_t kind;
  union thinkthen_complete_annotation_member_data data;
} thinkthen_complete_annotation_member;

typedef struct thinkthen_complete_annotation_member_map_entry {
  struct thinkthen_complete_utf8 name;
  const struct thinkthen_complete_annotation_member *value;
} thinkthen_complete_annotation_member_map_entry;

typedef struct thinkthen_complete_annotation_member_map {
  const struct thinkthen_complete_annotation_member_map_entry *data;
  size_t len;
} thinkthen_complete_annotation_member_map;

typedef struct thinkthen_complete_position {
  struct thinkthen_complete_utf8_presence file;
  struct thinkthen_complete_u64_presence first;
  struct thinkthen_complete_string_list_presence images;
  struct thinkthen_complete_u64_presence last;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_position;

typedef struct thinkthen_complete_position_presence {
  uint32_t presence;
  const struct thinkthen_complete_position *value;
} thinkthen_complete_position_presence;

typedef struct thinkthen_complete_failed {
  const struct thinkthen_complete_failure *failed;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_failed;

typedef union thinkthen_complete_annotated_field_data {
  uint32_t boolean;
  uint32_t null;
  struct thinkthen_complete_utf8 string;
  struct thinkthen_complete_string_list array;
  double number;
  const struct thinkthen_complete_failed *object;
} thinkthen_complete_annotated_field_data;

typedef struct thinkthen_complete_annotated_field {
  uint32_t kind;
  union thinkthen_complete_annotated_field_data data;
} thinkthen_complete_annotated_field;

typedef struct thinkthen_complete_annotated_field_map_entry {
  struct thinkthen_complete_utf8 name;
  const struct thinkthen_complete_annotated_field *value;
} thinkthen_complete_annotated_field_map_entry;

typedef struct thinkthen_complete_annotated_field_map {
  const struct thinkthen_complete_annotated_field_map_entry *data;
  size_t len;
} thinkthen_complete_annotated_field_map;

typedef struct thinkthen_complete_annotated_row {
  struct thinkthen_complete_annotated_field_map value;
} thinkthen_complete_annotated_row;

typedef struct thinkthen_complete_annotation {
  const struct thinkthen_complete_answer_id *answer_id;
  struct thinkthen_complete_annotation_member_map answers;
  struct thinkthen_complete_utf8_presence file;
  struct thinkthen_complete_u64_presence first_line;
  struct thinkthen_complete_u64_presence index;
  struct thinkthen_complete_json_presence input;
  struct thinkthen_complete_u64_presence last_line;
  const struct thinkthen_complete_meta *meta;
  struct thinkthen_complete_position_presence position;
  const struct thinkthen_complete_result_tag *schema;
  struct thinkthen_complete_physical_source_presence source;
  const struct thinkthen_complete_annotated_row *value;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_annotation;

typedef struct thinkthen_complete_session_packet_annotate_row {
  struct thinkthen_complete_utf8 function;
  struct thinkthen_complete_utf8 kind;
  const struct thinkthen_complete_annotation *value;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_session_packet_annotate_row;

typedef struct thinkthen_complete_atomic_decide_value_list {
  const struct thinkthen_complete_atomic_decide_value *const *data;
  size_t len;
} thinkthen_complete_atomic_decide_value_list;

typedef struct thinkthen_complete_session_packet_decide_aggregate {
  struct thinkthen_complete_utf8 function;
  struct thinkthen_complete_utf8 kind;
  struct thinkthen_complete_atomic_decide_value_list value;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_session_packet_decide_aggregate;

typedef struct thinkthen_complete_atomic_nullable_string_list {
  const struct thinkthen_complete_atomic_nullable_string *const *data;
  size_t len;
} thinkthen_complete_atomic_nullable_string_list;

typedef struct thinkthen_complete_session_packet_choose_aggregate {
  struct thinkthen_complete_utf8 function;
  struct thinkthen_complete_utf8 kind;
  struct thinkthen_complete_atomic_nullable_string_list value;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_session_packet_choose_aggregate;

typedef struct thinkthen_complete_atomic_array_of_string_list {
  const struct thinkthen_complete_atomic_array_of_string *const *data;
  size_t len;
} thinkthen_complete_atomic_array_of_string_list;

typedef struct thinkthen_complete_session_packet_tag_aggregate {
  struct thinkthen_complete_utf8 function;
  struct thinkthen_complete_utf8 kind;
  struct thinkthen_complete_atomic_array_of_string_list value;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_session_packet_tag_aggregate;

typedef struct thinkthen_complete_atomic_double_list {
  const struct thinkthen_complete_atomic_double *const *data;
  size_t len;
} thinkthen_complete_atomic_double_list;

typedef struct thinkthen_complete_session_packet_score_aggregate {
  struct thinkthen_complete_utf8 function;
  struct thinkthen_complete_utf8 kind;
  struct thinkthen_complete_atomic_double_list value;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_session_packet_score_aggregate;

typedef struct thinkthen_complete_atomic_boolean_list {
  const struct thinkthen_complete_atomic_boolean *const *data;
  size_t len;
} thinkthen_complete_atomic_boolean_list;

typedef struct thinkthen_complete_session_packet_filter_aggregate {
  struct thinkthen_complete_utf8 function;
  struct thinkthen_complete_utf8 kind;
  struct thinkthen_complete_atomic_boolean_list value;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_session_packet_filter_aggregate;

typedef struct thinkthen_complete_atomic_non_zero_usize {
  const struct thinkthen_complete_answer *answer;
  const struct thinkthen_complete_answer_id *answer_id;
  struct thinkthen_complete_image_list_presence images;
  struct thinkthen_complete_u64_presence index;
  struct thinkthen_complete_json_presence input;
  struct thinkthen_complete_rank_member_list_presence members;
  const struct thinkthen_complete_meta *meta;
  const struct thinkthen_complete_question_document *question;
  struct thinkthen_complete_utf8_presence question_name;
  const struct thinkthen_complete_result_tag *schema;
  struct thinkthen_complete_physical_source_presence source;
  struct thinkthen_complete_threshold_presence threshold;
  uint64_t value;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_atomic_non_zero_usize;

typedef struct thinkthen_complete_atomic_non_zero_usize_list {
  const struct thinkthen_complete_atomic_non_zero_usize *const *data;
  size_t len;
} thinkthen_complete_atomic_non_zero_usize_list;

typedef struct thinkthen_complete_session_packet_rank_aggregate {
  struct thinkthen_complete_utf8 function;
  struct thinkthen_complete_utf8 kind;
  struct thinkthen_complete_atomic_non_zero_usize_list value;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_session_packet_rank_aggregate;

typedef struct thinkthen_complete_find_answer {
  struct thinkthen_complete_f64_presence confidence;
  struct thinkthen_complete_utf8 kind;
  struct thinkthen_complete_utf8 pick;
  struct thinkthen_complete_number_map probabilities;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_find_answer;

typedef struct thinkthen_complete_find_candidate {
  struct thinkthen_complete_u64_presence index;
  struct thinkthen_complete_json_presence input;
  double probability;
  struct thinkthen_complete_physical_source_presence source;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_find_candidate;

typedef struct thinkthen_complete_find_candidate_list {
  const struct thinkthen_complete_find_candidate *const *data;
  size_t len;
} thinkthen_complete_find_candidate_list;

typedef struct thinkthen_complete_find_candidate_list_presence {
  uint32_t presence;
  struct thinkthen_complete_find_candidate_list value;
} thinkthen_complete_find_candidate_list_presence;

typedef struct thinkthen_complete_find_question_document {
  struct thinkthen_complete_batch_presence batch;
  struct thinkthen_complete_input_declaration_presence context_schema;
  struct thinkthen_complete_input_declaration_presence item_schema;
  struct thinkthen_complete_label_list_presence label_details;
  struct thinkthen_complete_utf8_presence model;
  struct thinkthen_complete_question_name_presence name;
  uint32_t none;
  struct thinkthen_complete_string_list_presence on;
  struct thinkthen_complete_utf8_presence profile;
  struct thinkthen_complete_json_presence text;
  struct thinkthen_complete_utf8 verb;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_find_question_document;

typedef struct thinkthen_complete_find {
  const struct thinkthen_complete_find_answer *answer;
  const struct thinkthen_complete_answer_id *answer_id;
  struct thinkthen_complete_find_candidate_list_presence candidates;
  struct thinkthen_complete_utf8_presence file;
  struct thinkthen_complete_u64_presence first_line;
  struct thinkthen_complete_u64_presence index;
  struct thinkthen_complete_u64_presence last_line;
  const struct thinkthen_complete_meta *meta;
  struct thinkthen_complete_position_presence position;
  const struct thinkthen_complete_find_question_document *question;
  const struct thinkthen_complete_result_tag *schema;
  struct thinkthen_complete_json_presence threshold;
  struct thinkthen_complete_json_presence value;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_find;

typedef struct thinkthen_complete_session_packet_find_aggregate {
  struct thinkthen_complete_utf8 function;
  struct thinkthen_complete_utf8 kind;
  const struct thinkthen_complete_find *value;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_session_packet_find_aggregate;

typedef struct thinkthen_complete_annotation_list {
  const struct thinkthen_complete_annotation *const *data;
  size_t len;
} thinkthen_complete_annotation_list;

typedef struct thinkthen_complete_session_packet_annotate_aggregate {
  struct thinkthen_complete_utf8 function;
  struct thinkthen_complete_utf8 kind;
  struct thinkthen_complete_annotation_list value;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_session_packet_annotate_aggregate;

typedef struct thinkthen_complete_number_map_presence {
  uint32_t presence;
  struct thinkthen_complete_number_map value;
} thinkthen_complete_number_map_presence;

typedef struct thinkthen_complete_name_odds {
  struct thinkthen_complete_number_map_presence edges;
  uint64_t end;
  struct thinkthen_complete_number_map_presence kinds;
  uint64_t start;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_name_odds;

typedef struct thinkthen_complete_name_odds_list {
  const struct thinkthen_complete_name_odds *const *data;
  size_t len;
} thinkthen_complete_name_odds_list;

typedef struct thinkthen_complete_place {
  uint64_t end;
  uint64_t start;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_place;

typedef struct thinkthen_complete_pair_odds {
  double probability;
  struct thinkthen_complete_utf8 relation;
  const struct thinkthen_complete_place *source;
  const struct thinkthen_complete_place *target;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_pair_odds;

typedef struct thinkthen_complete_pair_odds_list {
  const struct thinkthen_complete_pair_odds *const *data;
  size_t len;
} thinkthen_complete_pair_odds_list;

typedef struct thinkthen_complete_piece_odds {
  uint64_t end;
  uint64_t start;
  struct thinkthen_complete_number_map tags;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_piece_odds;

typedef struct thinkthen_complete_piece_odds_list {
  const struct thinkthen_complete_piece_odds *const *data;
  size_t len;
} thinkthen_complete_piece_odds_list;

typedef struct thinkthen_complete_place_presence {
  uint32_t presence;
  const struct thinkthen_complete_place *value;
} thinkthen_complete_place_presence;

typedef struct thinkthen_complete_recognition_proposal {
  uint64_t end;
  uint32_t kept;
  struct thinkthen_complete_utf8_presence kind;
  struct thinkthen_complete_place_presence selected;
  double span_probability;
  uint64_t start;
  struct thinkthen_complete_f64_presence strength;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_recognition_proposal;

typedef struct thinkthen_complete_recognition_proposal_list {
  const struct thinkthen_complete_recognition_proposal *const *data;
  size_t len;
} thinkthen_complete_recognition_proposal_list;

typedef struct thinkthen_complete_recognition_odds_recognize_answer {
  struct thinkthen_complete_name_odds_list names;
  struct thinkthen_complete_pair_odds_list pairs;
  struct thinkthen_complete_piece_odds_list pieces;
  struct thinkthen_complete_recognition_proposal_list proposals;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_recognition_odds_recognize_answer;

typedef struct thinkthen_complete_boundary_proposal {
  uint64_t end;
  struct thinkthen_complete_utf8_presence file;
  struct thinkthen_complete_u64_presence first_line;
  struct thinkthen_complete_u64_presence last_line;
  uint64_t length;
  double probability;
  uint64_t start;
  struct thinkthen_complete_utf8 text;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_boundary_proposal;

typedef struct thinkthen_complete_boundary_proposal_list {
  const struct thinkthen_complete_boundary_proposal *const *data;
  size_t len;
} thinkthen_complete_boundary_proposal_list;

typedef struct thinkthen_complete_recognition_odds_boundary_odds {
  struct thinkthen_complete_piece_odds_list pieces;
  struct thinkthen_complete_boundary_proposal_list proposals;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_recognition_odds_boundary_odds;

typedef union thinkthen_complete_recognition_odds_data {
  const struct thinkthen_complete_recognition_odds_recognize_answer *recognize_answer;
  const struct thinkthen_complete_recognition_odds_boundary_odds *boundary_odds;
} thinkthen_complete_recognition_odds_data;

typedef struct thinkthen_complete_recognition_odds {
  uint32_t kind;
  union thinkthen_complete_recognition_odds_data data;
} thinkthen_complete_recognition_odds;

typedef struct thinkthen_complete_json_map_entry {
  struct thinkthen_complete_utf8 name;
  const struct thinkthen_complete_json *value;
} thinkthen_complete_json_map_entry;

typedef struct thinkthen_complete_json_map {
  const struct thinkthen_complete_json_map_entry *data;
  size_t len;
} thinkthen_complete_json_map;

typedef struct thinkthen_complete_recognition_mode {
  uint32_t kind;
} thinkthen_complete_recognition_mode;

typedef struct thinkthen_complete_recognition_mode_presence {
  uint32_t presence;
  const struct thinkthen_complete_recognition_mode *value;
} thinkthen_complete_recognition_mode_presence;

typedef struct thinkthen_complete_relation_rule {
  uint32_t either;
  struct thinkthen_complete_utf8 name;
  struct thinkthen_complete_utf8 reads;
  struct thinkthen_complete_u32_presence single;
  struct thinkthen_complete_utf8 source;
  struct thinkthen_complete_utf8 target;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_relation_rule;

typedef struct thinkthen_complete_relation_rule_list {
  const struct thinkthen_complete_relation_rule *const *data;
  size_t len;
} thinkthen_complete_relation_rule_list;

typedef struct thinkthen_complete_relation_rule_list_presence {
  uint32_t presence;
  struct thinkthen_complete_relation_rule_list value;
} thinkthen_complete_relation_rule_list_presence;

typedef struct thinkthen_complete_recognition_stage_context {
  struct thinkthen_complete_utf8_presence boundary;
  struct thinkthen_complete_utf8_presence kind_edge;
  struct thinkthen_complete_utf8_presence relation;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_recognition_stage_context;

typedef struct thinkthen_complete_recognition_stage_context_presence {
  uint32_t presence;
  const struct thinkthen_complete_recognition_stage_context *value;
} thinkthen_complete_recognition_stage_context_presence;

typedef struct thinkthen_complete_verb {
  uint32_t kind;
} thinkthen_complete_verb;

typedef struct thinkthen_complete_recognize_question_document {
  struct thinkthen_complete_batch_presence batch;
  struct thinkthen_complete_input_declaration_presence context_schema;
  struct thinkthen_complete_json_presence entity_definition;
  struct thinkthen_complete_json_presence instructions;
  struct thinkthen_complete_input_declaration_presence item_schema;
  struct thinkthen_complete_json_map kinds;
  struct thinkthen_complete_label_list_presence label_details;
  struct thinkthen_complete_recognition_mode_presence mode;
  struct thinkthen_complete_utf8_presence model;
  struct thinkthen_complete_question_name_presence name;
  struct thinkthen_complete_string_list_presence on;
  struct thinkthen_complete_utf8_presence profile;
  struct thinkthen_complete_threshold_presence relation_threshold;
  struct thinkthen_complete_relation_rule_list_presence relations;
  struct thinkthen_complete_u64_presence snippet_pieces;
  struct thinkthen_complete_recognition_stage_context_presence stage_context;
  const struct thinkthen_complete_threshold *threshold;
  const struct thinkthen_complete_verb *verb;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_recognize_question_document;

typedef struct thinkthen_complete_entity {
  uint64_t end;
  struct thinkthen_complete_utf8_presence file;
  struct thinkthen_complete_u64_presence first_line;
  struct thinkthen_complete_utf8 kind;
  struct thinkthen_complete_u64_presence last_line;
  uint64_t length;
  uint64_t start;
  double strength;
  struct thinkthen_complete_utf8 text;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_entity;

typedef struct thinkthen_complete_entity_list {
  const struct thinkthen_complete_entity *const *data;
  size_t len;
} thinkthen_complete_entity_list;

typedef struct thinkthen_complete_entity_edge {
  struct thinkthen_complete_u32_presence either;
  double probability;
  struct thinkthen_complete_utf8 relation;
  const struct thinkthen_complete_entity *source;
  const struct thinkthen_complete_entity *target;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_entity_edge;

typedef struct thinkthen_complete_entity_edge_list {
  const struct thinkthen_complete_entity_edge *const *data;
  size_t len;
} thinkthen_complete_entity_edge_list;

typedef struct thinkthen_complete_entity_edge_list_presence {
  uint32_t presence;
  struct thinkthen_complete_entity_edge_list value;
} thinkthen_complete_entity_edge_list_presence;

typedef struct thinkthen_complete_recognize_entities {
  struct thinkthen_complete_entity_list entities;
  struct thinkthen_complete_entity_edge_list_presence relations;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_recognize_entities;

typedef struct thinkthen_complete_boundary_mode {
  uint32_t kind;
} thinkthen_complete_boundary_mode;

typedef struct thinkthen_complete_recognize_mode_proposals {
  const struct thinkthen_complete_boundary_mode *mode;
  struct thinkthen_complete_boundary_proposal_list proposals;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_recognize_mode_proposals;

typedef union thinkthen_complete_recognize_data {
  const struct thinkthen_complete_recognize_entities *entities;
  const struct thinkthen_complete_recognize_mode_proposals *mode_proposals;
} thinkthen_complete_recognize_data;

typedef struct thinkthen_complete_recognize {
  uint32_t kind;
  union thinkthen_complete_recognize_data data;
} thinkthen_complete_recognize;

typedef struct thinkthen_complete_recognition {
  const struct thinkthen_complete_recognition_odds *answer;
  const struct thinkthen_complete_answer_id *answer_id;
  struct thinkthen_complete_utf8_presence file;
  struct thinkthen_complete_u64_presence first_line;
  struct thinkthen_complete_u64_presence index;
  struct thinkthen_complete_json_presence input;
  struct thinkthen_complete_u64_presence last_line;
  const struct thinkthen_complete_meta *meta;
  struct thinkthen_complete_position_presence position;
  const struct thinkthen_complete_recognize_question_document *question;
  const struct thinkthen_complete_result_tag *schema;
  struct thinkthen_complete_physical_source_presence source;
  const struct thinkthen_complete_recognize *value;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_recognition;

typedef struct thinkthen_complete_recognition_list {
  const struct thinkthen_complete_recognition *const *data;
  size_t len;
} thinkthen_complete_recognition_list;

typedef struct thinkthen_complete_session_packet_recognize_aggregate {
  struct thinkthen_complete_utf8 function;
  struct thinkthen_complete_utf8 kind;
  struct thinkthen_complete_recognition_list value;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_session_packet_recognize_aggregate;

typedef struct thinkthen_complete_relation_direction {
  uint32_t kind;
} thinkthen_complete_relation_direction;

typedef struct thinkthen_complete_relation_method {
  uint32_t kind;
} thinkthen_complete_relation_method;

typedef struct thinkthen_complete_related_entity {
  struct thinkthen_complete_utf8 kind;
  struct thinkthen_complete_utf8 name;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_related_entity;

typedef struct thinkthen_complete_related_entity_presence {
  uint32_t presence;
  const struct thinkthen_complete_related_entity *value;
} thinkthen_complete_related_entity_presence;

typedef struct thinkthen_complete_relation_member_answer_id {
  const struct thinkthen_complete_relation_direction *direction;
  const struct thinkthen_complete_relation_method *method;
  struct thinkthen_complete_observation_list observations;
  const struct thinkthen_complete_question_document *question;
  struct thinkthen_complete_question_source_list question_sources;
  struct thinkthen_complete_utf8 reads;
  struct thinkthen_complete_utf8 relation;
  struct thinkthen_complete_utf8 request;
  const struct thinkthen_complete_related_entity *source;
  struct thinkthen_complete_related_entity_presence target;
  const struct thinkthen_complete_threshold *threshold;
  struct thinkthen_complete_usage_presence usage;
  uint32_t accepted;
  const struct thinkthen_complete_answer *answer;
  const struct thinkthen_complete_answer_id *answer_id;
  double probability;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_relation_member_answer_id;

typedef struct thinkthen_complete_relation_member_failure_id {
  const struct thinkthen_complete_relation_direction *direction;
  const struct thinkthen_complete_relation_method *method;
  struct thinkthen_complete_observation_list observations;
  const struct thinkthen_complete_question_document *question;
  struct thinkthen_complete_question_source_list question_sources;
  struct thinkthen_complete_utf8 reads;
  struct thinkthen_complete_utf8 relation;
  struct thinkthen_complete_utf8 request;
  const struct thinkthen_complete_related_entity *source;
  struct thinkthen_complete_related_entity_presence target;
  const struct thinkthen_complete_threshold *threshold;
  struct thinkthen_complete_usage_presence usage;
  const struct thinkthen_complete_failure *failure;
  const struct thinkthen_complete_failure_id *failure_id;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_relation_member_failure_id;

typedef union thinkthen_complete_relation_member_data {
  const struct thinkthen_complete_relation_member_answer_id *answer_id;
  const struct thinkthen_complete_relation_member_failure_id *failure_id;
} thinkthen_complete_relation_member_data;

typedef struct thinkthen_complete_relation_member {
  uint32_t kind;
  union thinkthen_complete_relation_member_data data;
} thinkthen_complete_relation_member;

typedef struct thinkthen_complete_relation_member_list {
  const struct thinkthen_complete_relation_member *const *data;
  size_t len;
} thinkthen_complete_relation_member_list;

typedef struct thinkthen_complete_answers {
  struct thinkthen_complete_relation_member_list questions;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_answers;

typedef struct thinkthen_complete_session_input_source {
  uint64_t index;
  const struct thinkthen_complete_physical_source *source;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_session_input_source;

typedef struct thinkthen_complete_session_input_source_list {
  const struct thinkthen_complete_session_input_source *const *data;
  size_t len;
} thinkthen_complete_session_input_source_list;

typedef struct thinkthen_complete_session_input_source_list_presence {
  uint32_t presence;
  struct thinkthen_complete_session_input_source_list value;
} thinkthen_complete_session_input_source_list_presence;

typedef struct thinkthen_complete_relate_fields {
  struct thinkthen_complete_utf8 kind;
  struct thinkthen_complete_utf8 name;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_relate_fields;

typedef struct thinkthen_complete_relate_fields_presence {
  uint32_t presence;
  const struct thinkthen_complete_relate_fields *value;
} thinkthen_complete_relate_fields_presence;

typedef struct thinkthen_complete_relate_question_document {
  struct thinkthen_complete_batch_presence batch;
  struct thinkthen_complete_input_declaration_presence context_schema;
  struct thinkthen_complete_relate_fields_presence fields;
  struct thinkthen_complete_input_declaration_presence item_schema;
  struct thinkthen_complete_label_list_presence label_details;
  struct thinkthen_complete_utf8_presence model;
  struct thinkthen_complete_question_name_presence name;
  struct thinkthen_complete_string_list_presence on;
  struct thinkthen_complete_utf8_presence profile;
  struct thinkthen_complete_relation_rule_list relations;
  const struct thinkthen_complete_threshold *threshold;
  struct thinkthen_complete_utf8 verb;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_relate_question_document;

typedef struct thinkthen_complete_relation_endpoint_related_entity {
  struct thinkthen_complete_utf8 kind;
  struct thinkthen_complete_utf8 name;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_relation_endpoint_related_entity;

typedef struct thinkthen_complete_relation_endpoint_source {
  struct thinkthen_complete_utf8_presence file;
  struct thinkthen_complete_u64_presence first_line;
  struct thinkthen_complete_utf8 kind;
  struct thinkthen_complete_u64_presence last_line;
  struct thinkthen_complete_utf8 name;
  uint64_t ordinal;
  struct thinkthen_complete_json_presence record;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_relation_endpoint_source;

typedef union thinkthen_complete_relation_endpoint_data {
  const struct thinkthen_complete_relation_endpoint_related_entity *related_entity;
  const struct thinkthen_complete_relation_endpoint_source *source;
} thinkthen_complete_relation_endpoint_data;

typedef struct thinkthen_complete_relation_endpoint {
  uint32_t kind;
  union thinkthen_complete_relation_endpoint_data data;
} thinkthen_complete_relation_endpoint;

typedef struct thinkthen_complete_related_entity_edge {
  struct thinkthen_complete_u32_presence either;
  double probability;
  struct thinkthen_complete_utf8 relation;
  const struct thinkthen_complete_relation_endpoint *source;
  const struct thinkthen_complete_relation_endpoint *target;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_related_entity_edge;

typedef struct thinkthen_complete_related_entity_edge_list {
  const struct thinkthen_complete_related_entity_edge *const *data;
  size_t len;
} thinkthen_complete_related_entity_edge_list;

typedef struct thinkthen_complete_relation {
  const struct thinkthen_complete_answers *answer;
  const struct thinkthen_complete_answer_id *answer_id;
  struct thinkthen_complete_utf8_presence file;
  struct thinkthen_complete_u64_presence first_line;
  struct thinkthen_complete_u64_presence index;
  struct thinkthen_complete_json_presence input;
  struct thinkthen_complete_session_input_source_list_presence input_sources;
  struct thinkthen_complete_u64_presence last_line;
  const struct thinkthen_complete_meta *meta;
  struct thinkthen_complete_position_presence position;
  const struct thinkthen_complete_relate_question_document *question;
  const struct thinkthen_complete_result_tag *schema;
  struct thinkthen_complete_related_entity_edge_list value;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_relation;

typedef struct thinkthen_complete_session_packet_relate_aggregate {
  struct thinkthen_complete_utf8 function;
  struct thinkthen_complete_utf8 kind;
  const struct thinkthen_complete_relation *value;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_session_packet_relate_aggregate;

typedef struct thinkthen_complete_request_function {
  uint32_t kind;
} thinkthen_complete_request_function;

typedef struct thinkthen_complete_answer_id_presence {
  uint32_t presence;
  const struct thinkthen_complete_answer_id *value;
} thinkthen_complete_answer_id_presence;

typedef struct thinkthen_complete_failure_presence {
  uint32_t presence;
  const struct thinkthen_complete_failure *value;
} thinkthen_complete_failure_presence;

typedef struct thinkthen_complete_failure_id_presence {
  uint32_t presence;
  const struct thinkthen_complete_failure_id *value;
} thinkthen_complete_failure_id_presence;

typedef struct thinkthen_complete_json_list {
  const struct thinkthen_complete_json *const *data;
  size_t len;
} thinkthen_complete_json_list;

typedef struct thinkthen_complete_session_probabilities_yes_no {
  struct thinkthen_complete_utf8 kind;
  double value;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_session_probabilities_yes_no;

typedef struct thinkthen_complete_session_named_probability {
  struct thinkthen_complete_utf8 name;
  double probability;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_session_named_probability;

typedef struct thinkthen_complete_session_named_probability_list {
  const struct thinkthen_complete_session_named_probability *const *data;
  size_t len;
} thinkthen_complete_session_named_probability_list;

typedef struct thinkthen_complete_session_probabilities_named {
  struct thinkthen_complete_utf8 kind;
  struct thinkthen_complete_session_named_probability_list value;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_session_probabilities_named;

typedef union thinkthen_complete_session_probabilities_data {
  const struct thinkthen_complete_session_probabilities_yes_no *yes_no;
  const struct thinkthen_complete_session_probabilities_named *named;
} thinkthen_complete_session_probabilities_data;

typedef struct thinkthen_complete_session_probabilities {
  uint32_t kind;
  union thinkthen_complete_session_probabilities_data data;
} thinkthen_complete_session_probabilities;

typedef struct thinkthen_complete_session_probabilities_presence {
  uint32_t presence;
  const struct thinkthen_complete_session_probabilities *value;
} thinkthen_complete_session_probabilities_presence;

typedef struct thinkthen_complete_token_usage {
  uint64_t input_tokens;
  uint64_t output_tokens;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_token_usage;

typedef struct thinkthen_complete_token_usage_presence {
  uint32_t presence;
  const struct thinkthen_complete_token_usage *value;
} thinkthen_complete_token_usage_presence;

typedef struct thinkthen_complete_session_question_detail {
  struct thinkthen_complete_answer_id_presence answer_id;
  uint32_t cached;
  struct thinkthen_complete_f64_presence confidence;
  uint64_t failed_questions;
  struct thinkthen_complete_failure_presence failure;
  struct thinkthen_complete_failure_id_presence failure_id;
  struct thinkthen_complete_json_presence input;
  struct thinkthen_complete_physical_source_presence input_source;
  struct thinkthen_complete_session_input_source_list input_sources;
  struct thinkthen_complete_json_list inputs;
  struct thinkthen_complete_utf8 model;
  struct thinkthen_complete_observation_list observations;
  struct thinkthen_complete_session_probabilities_presence probabilities;
  const struct thinkthen_complete_question_document *question;
  struct thinkthen_complete_utf8 question_sha256;
  struct thinkthen_complete_question_source_list question_sources;
  struct thinkthen_complete_utf8_presence raw_pick;
  struct thinkthen_complete_usage_presence reported_usage;
  struct thinkthen_complete_string_list requests;
  uint64_t requests_sent;
  struct thinkthen_complete_threshold_presence threshold;
  struct thinkthen_complete_utf8 url;
  struct thinkthen_complete_token_usage_presence usage;
  struct thinkthen_complete_value_presence value;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_session_question_detail;

typedef struct thinkthen_complete_session_observation_question {
  const struct thinkthen_complete_session_question_detail *detail;
  uint64_t index;
  struct thinkthen_complete_utf8 kind;
  struct thinkthen_complete_utf8_presence member;
  uint64_t position;
  struct thinkthen_complete_utf8_presence stage;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_session_observation_question;

typedef struct thinkthen_complete_session_judgment_decision {
  struct thinkthen_complete_utf8 kind;
  struct thinkthen_complete_u32_presence value;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_session_judgment_decision;

typedef struct thinkthen_complete_session_judgment_choice {
  struct thinkthen_complete_utf8 kind;
  struct thinkthen_complete_utf8_presence value;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_session_judgment_choice;

typedef struct thinkthen_complete_session_judgment_score {
  struct thinkthen_complete_utf8 kind;
  double value;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_session_judgment_score;

typedef struct thinkthen_complete_session_judgment_tags {
  struct thinkthen_complete_utf8 kind;
  struct thinkthen_complete_string_list value;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_session_judgment_tags;

typedef union thinkthen_complete_session_judgment_data {
  const struct thinkthen_complete_session_judgment_decision *decision;
  const struct thinkthen_complete_session_judgment_choice *choice;
  const struct thinkthen_complete_session_judgment_score *score;
  const struct thinkthen_complete_session_judgment_tags *tags;
} thinkthen_complete_session_judgment_data;

typedef struct thinkthen_complete_session_judgment {
  uint32_t kind;
  union thinkthen_complete_session_judgment_data data;
} thinkthen_complete_session_judgment;

typedef struct thinkthen_complete_session_observed_row_judgment {
  struct thinkthen_complete_utf8 kind;
  const struct thinkthen_complete_session_judgment *value;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_session_observed_row_judgment;

typedef struct thinkthen_complete_annotation_value_decision {
  struct thinkthen_complete_utf8 kind;
  struct thinkthen_complete_u32_presence value;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_annotation_value_decision;

typedef struct thinkthen_complete_annotation_value_choice {
  struct thinkthen_complete_utf8 kind;
  struct thinkthen_complete_utf8_presence value;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_annotation_value_choice;

typedef struct thinkthen_complete_annotation_value_score {
  struct thinkthen_complete_utf8 kind;
  double value;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_annotation_value_score;

typedef struct thinkthen_complete_annotation_value_tags {
  struct thinkthen_complete_utf8 kind;
  struct thinkthen_complete_string_list value;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_annotation_value_tags;

typedef struct thinkthen_complete_annotation_value_failed {
  struct thinkthen_complete_utf8 kind;
  const struct thinkthen_complete_failure *value;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_annotation_value_failed;

typedef union thinkthen_complete_annotation_value_data {
  const struct thinkthen_complete_annotation_value_decision *decision;
  const struct thinkthen_complete_annotation_value_choice *choice;
  const struct thinkthen_complete_annotation_value_score *score;
  const struct thinkthen_complete_annotation_value_tags *tags;
  const struct thinkthen_complete_annotation_value_failed *failed;
} thinkthen_complete_annotation_value_data;

typedef struct thinkthen_complete_annotation_value {
  uint32_t kind;
  union thinkthen_complete_annotation_value_data data;
} thinkthen_complete_annotation_value;

typedef struct thinkthen_complete_session_annotation {
  struct thinkthen_complete_utf8 name;
  const struct thinkthen_complete_annotation_value *value;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_session_annotation;

typedef struct thinkthen_complete_session_annotation_list {
  const struct thinkthen_complete_session_annotation *const *data;
  size_t len;
} thinkthen_complete_session_annotation_list;

typedef struct thinkthen_complete_session_observed_row_annotated {
  struct thinkthen_complete_utf8 kind;
  struct thinkthen_complete_session_annotation_list value;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_session_observed_row_annotated;

typedef struct thinkthen_complete_boundary_proposal_list_presence {
  uint32_t presence;
  struct thinkthen_complete_boundary_proposal_list value;
} thinkthen_complete_boundary_proposal_list_presence;

typedef struct thinkthen_complete_recognition_edge {
  uint32_t either;
  double probability;
  struct thinkthen_complete_utf8 relation;
  const struct thinkthen_complete_entity *source;
  const struct thinkthen_complete_entity *target;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_recognition_edge;

typedef struct thinkthen_complete_recognition_edge_list {
  const struct thinkthen_complete_recognition_edge *const *data;
  size_t len;
} thinkthen_complete_recognition_edge_list;

typedef struct thinkthen_complete_recognition_edge_list_presence {
  uint32_t presence;
  struct thinkthen_complete_recognition_edge_list value;
} thinkthen_complete_recognition_edge_list_presence;

typedef struct thinkthen_complete_session_recognition {
  struct thinkthen_complete_entity_list entities;
  const struct thinkthen_complete_recognition_mode *mode;
  struct thinkthen_complete_boundary_proposal_list_presence proposals;
  struct thinkthen_complete_recognition_edge_list_presence relations;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_session_recognition;

typedef struct thinkthen_complete_session_observed_row_recognized {
  struct thinkthen_complete_utf8 kind;
  const struct thinkthen_complete_session_recognition *value;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_session_observed_row_recognized;

typedef struct thinkthen_complete_session_observed_row_find {
  struct thinkthen_complete_utf8 kind;
  struct thinkthen_complete_u64_presence value;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_session_observed_row_find;

typedef struct thinkthen_complete_entity_document {
  struct thinkthen_complete_utf8 kind;
  struct thinkthen_complete_utf8 name;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_entity_document;

typedef struct thinkthen_complete_session_relation_edge {
  uint32_t either;
  double probability;
  struct thinkthen_complete_utf8 relation;
  const struct thinkthen_complete_entity_document *source;
  const struct thinkthen_complete_entity_document *target;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_session_relation_edge;

typedef struct thinkthen_complete_session_relation_edge_list {
  const struct thinkthen_complete_session_relation_edge *const *data;
  size_t len;
} thinkthen_complete_session_relation_edge_list;

typedef struct thinkthen_complete_session_observed_row_relations {
  struct thinkthen_complete_utf8 kind;
  struct thinkthen_complete_session_relation_edge_list value;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_session_observed_row_relations;

typedef union thinkthen_complete_session_observed_row_data {
  const struct thinkthen_complete_session_observed_row_judgment *judgment;
  const struct thinkthen_complete_session_observed_row_annotated *annotated;
  const struct thinkthen_complete_session_observed_row_recognized *recognized;
  const struct thinkthen_complete_session_observed_row_find *find;
  const struct thinkthen_complete_session_observed_row_relations *relations;
} thinkthen_complete_session_observed_row_data;

typedef struct thinkthen_complete_session_observed_row {
  uint32_t kind;
  union thinkthen_complete_session_observed_row_data data;
} thinkthen_complete_session_observed_row;

typedef struct thinkthen_complete_session_observation_row {
  uint64_t index;
  struct thinkthen_complete_utf8 kind;
  const struct thinkthen_complete_session_observed_row *value;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_session_observation_row;

typedef union thinkthen_complete_session_observation_data {
  const struct thinkthen_complete_session_observation_question *question;
  const struct thinkthen_complete_session_observation_row *row;
} thinkthen_complete_session_observation_data;

typedef struct thinkthen_complete_session_observation {
  uint32_t kind;
  union thinkthen_complete_session_observation_data data;
} thinkthen_complete_session_observation;

typedef struct thinkthen_complete_session_packet_observation {
  const struct thinkthen_complete_request_function *function;
  struct thinkthen_complete_utf8 kind;
  const struct thinkthen_complete_session_observation *value;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_session_packet_observation;

typedef union thinkthen_complete_answer_shape_4_any_of_1_items_data {
  uint64_t integer;
  uint32_t null;
} thinkthen_complete_answer_shape_4_any_of_1_items_data;

typedef struct thinkthen_complete_answer_shape_4_any_of_1_items {
  uint32_t kind;
  union thinkthen_complete_answer_shape_4_any_of_1_items_data data;
} thinkthen_complete_answer_shape_4_any_of_1_items;

typedef struct thinkthen_complete_answer_shape_4_any_of_1_items_list {
  const struct thinkthen_complete_answer_shape_4_any_of_1_items *const *data;
  size_t len;
} thinkthen_complete_answer_shape_4_any_of_1_items_list;

typedef union thinkthen_complete_answer_shape_4_data {
  uint64_t integer;
  uint32_t null;
  struct thinkthen_complete_answer_shape_4_any_of_1_items_list array;
} thinkthen_complete_answer_shape_4_data;

typedef struct thinkthen_complete_answer_shape_4 {
  uint32_t kind;
  union thinkthen_complete_answer_shape_4_data data;
} thinkthen_complete_answer_shape_4;

typedef struct thinkthen_complete_answer_shape_4_presence {
  uint32_t presence;
  const struct thinkthen_complete_answer_shape_4 *value;
} thinkthen_complete_answer_shape_4_presence;

typedef union thinkthen_complete_answer_shape_6_any_of_1_items_data {
  struct thinkthen_complete_utf8 string;
  uint32_t null;
} thinkthen_complete_answer_shape_6_any_of_1_items_data;

typedef struct thinkthen_complete_answer_shape_6_any_of_1_items {
  uint32_t kind;
  union thinkthen_complete_answer_shape_6_any_of_1_items_data data;
} thinkthen_complete_answer_shape_6_any_of_1_items;

typedef struct thinkthen_complete_answer_shape_6_any_of_1_items_list {
  const struct thinkthen_complete_answer_shape_6_any_of_1_items *const *data;
  size_t len;
} thinkthen_complete_answer_shape_6_any_of_1_items_list;

typedef union thinkthen_complete_answer_shape_3_data {
  struct thinkthen_complete_utf8 string;
  uint32_t null;
  struct thinkthen_complete_answer_shape_6_any_of_1_items_list array;
} thinkthen_complete_answer_shape_3_data;

typedef struct thinkthen_complete_answer_shape_3 {
  uint32_t kind;
  union thinkthen_complete_answer_shape_3_data data;
} thinkthen_complete_answer_shape_3;

typedef struct thinkthen_complete_answer_shape_3_presence {
  uint32_t presence;
  const struct thinkthen_complete_answer_shape_3 *value;
} thinkthen_complete_answer_shape_3_presence;

typedef union thinkthen_complete_answer_shape_8_any_of_1_items_data {
  double number;
  uint32_t null;
} thinkthen_complete_answer_shape_8_any_of_1_items_data;

typedef struct thinkthen_complete_answer_shape_8_any_of_1_items {
  uint32_t kind;
  union thinkthen_complete_answer_shape_8_any_of_1_items_data data;
} thinkthen_complete_answer_shape_8_any_of_1_items;

typedef struct thinkthen_complete_answer_shape_8_any_of_1_items_list {
  const struct thinkthen_complete_answer_shape_8_any_of_1_items *const *data;
  size_t len;
} thinkthen_complete_answer_shape_8_any_of_1_items_list;

typedef union thinkthen_complete_answer_shape_2_data {
  double number;
  uint32_t null;
  struct thinkthen_complete_answer_shape_8_any_of_1_items_list array;
} thinkthen_complete_answer_shape_2_data;

typedef struct thinkthen_complete_answer_shape_2 {
  uint32_t kind;
  union thinkthen_complete_answer_shape_2_data data;
} thinkthen_complete_answer_shape_2;

typedef struct thinkthen_complete_answer_shape_2_presence {
  uint32_t presence;
  const struct thinkthen_complete_answer_shape_2 *value;
} thinkthen_complete_answer_shape_2_presence;

typedef struct thinkthen_complete_answer_resolution {
  uint32_t kind;
} thinkthen_complete_answer_resolution;

typedef struct thinkthen_complete_answer_resolution_map_entry {
  struct thinkthen_complete_utf8 name;
  const struct thinkthen_complete_answer_resolution *value;
} thinkthen_complete_answer_resolution_map_entry;

typedef struct thinkthen_complete_answer_resolution_map {
  const struct thinkthen_complete_answer_resolution_map_entry *data;
  size_t len;
} thinkthen_complete_answer_resolution_map;

typedef union thinkthen_complete_answer_resolutions_data {
  const struct thinkthen_complete_answer_resolution *string;
  struct thinkthen_complete_answer_resolution_map object;
} thinkthen_complete_answer_resolutions_data;

typedef struct thinkthen_complete_answer_resolutions {
  uint32_t kind;
  union thinkthen_complete_answer_resolutions_data data;
} thinkthen_complete_answer_resolutions;

typedef struct thinkthen_complete_answer_resolutions_list {
  const struct thinkthen_complete_answer_resolutions *const *data;
  size_t len;
} thinkthen_complete_answer_resolutions_list;

typedef union thinkthen_complete_answer_shape_5_data {
  const struct thinkthen_complete_answer_resolution *string;
  struct thinkthen_complete_answer_resolution_map object;
  struct thinkthen_complete_answer_resolutions_list array;
} thinkthen_complete_answer_shape_5_data;

typedef struct thinkthen_complete_answer_shape_5 {
  uint32_t kind;
  union thinkthen_complete_answer_shape_5_data data;
} thinkthen_complete_answer_shape_5;

typedef struct thinkthen_complete_answer_shape {
  const struct thinkthen_complete_json *value;
} thinkthen_complete_answer_shape;

typedef struct thinkthen_complete_answer_shape_presence {
  uint32_t presence;
  const struct thinkthen_complete_answer_shape *value;
} thinkthen_complete_answer_shape_presence;

typedef struct thinkthen_complete_settled_answer_decide {
  struct thinkthen_complete_utf8 function;
  struct thinkthen_complete_answer_shape_4_presence index;
  struct thinkthen_complete_answer_shape_3_presence pick;
  struct thinkthen_complete_answer_shape_2_presence probability;
  const struct thinkthen_complete_answer_shape_5 *resolution;
  struct thinkthen_complete_answer_shape_presence value;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_settled_answer_decide;

typedef union thinkthen_complete_answer_shape_6_data {
  struct thinkthen_complete_utf8 string;
  uint32_t null;
  struct thinkthen_complete_answer_shape_6_any_of_1_items_list array;
} thinkthen_complete_answer_shape_6_data;

typedef struct thinkthen_complete_answer_shape_6 {
  uint32_t kind;
  union thinkthen_complete_answer_shape_6_data data;
} thinkthen_complete_answer_shape_6;

typedef struct thinkthen_complete_answer_shape_6_presence {
  uint32_t presence;
  const struct thinkthen_complete_answer_shape_6 *value;
} thinkthen_complete_answer_shape_6_presence;

typedef struct thinkthen_complete_settled_answer_choose {
  struct thinkthen_complete_utf8 function;
  struct thinkthen_complete_answer_shape_4_presence index;
  struct thinkthen_complete_answer_shape_3_presence pick;
  struct thinkthen_complete_answer_shape_2_presence probability;
  const struct thinkthen_complete_answer_shape_5 *resolution;
  struct thinkthen_complete_answer_shape_6_presence value;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_settled_answer_choose;

typedef union thinkthen_complete_answer_shape_7_any_of_1_items_data {
  struct thinkthen_complete_string_list array;
  uint32_t null;
} thinkthen_complete_answer_shape_7_any_of_1_items_data;

typedef struct thinkthen_complete_answer_shape_7_any_of_1_items {
  uint32_t kind;
  union thinkthen_complete_answer_shape_7_any_of_1_items_data data;
} thinkthen_complete_answer_shape_7_any_of_1_items;

typedef struct thinkthen_complete_answer_shape_7_any_of_1_items_list {
  const struct thinkthen_complete_answer_shape_7_any_of_1_items *const *data;
  size_t len;
} thinkthen_complete_answer_shape_7_any_of_1_items_list;

typedef union thinkthen_complete_answer_shape_7_data {
  struct thinkthen_complete_string_list array;
  uint32_t null;
  struct thinkthen_complete_answer_shape_7_any_of_1_items_list array2;
} thinkthen_complete_answer_shape_7_data;

typedef struct thinkthen_complete_answer_shape_7 {
  uint32_t kind;
  union thinkthen_complete_answer_shape_7_data data;
} thinkthen_complete_answer_shape_7;

typedef struct thinkthen_complete_answer_shape_7_presence {
  uint32_t presence;
  const struct thinkthen_complete_answer_shape_7 *value;
} thinkthen_complete_answer_shape_7_presence;

typedef struct thinkthen_complete_settled_answer_tag {
  struct thinkthen_complete_utf8 function;
  struct thinkthen_complete_answer_shape_4_presence index;
  struct thinkthen_complete_answer_shape_3_presence pick;
  struct thinkthen_complete_answer_shape_2_presence probability;
  const struct thinkthen_complete_answer_shape_5 *resolution;
  struct thinkthen_complete_answer_shape_7_presence value;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_settled_answer_tag;

typedef union thinkthen_complete_answer_shape_8_data {
  double number;
  uint32_t null;
  struct thinkthen_complete_answer_shape_8_any_of_1_items_list array;
} thinkthen_complete_answer_shape_8_data;

typedef struct thinkthen_complete_answer_shape_8 {
  uint32_t kind;
  union thinkthen_complete_answer_shape_8_data data;
} thinkthen_complete_answer_shape_8;

typedef struct thinkthen_complete_answer_shape_8_presence {
  uint32_t presence;
  const struct thinkthen_complete_answer_shape_8 *value;
} thinkthen_complete_answer_shape_8_presence;

typedef struct thinkthen_complete_settled_answer_score {
  struct thinkthen_complete_utf8 function;
  struct thinkthen_complete_answer_shape_4_presence index;
  struct thinkthen_complete_answer_shape_3_presence pick;
  struct thinkthen_complete_answer_shape_2_presence probability;
  const struct thinkthen_complete_answer_shape_5 *resolution;
  struct thinkthen_complete_answer_shape_8_presence value;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_settled_answer_score;

typedef union thinkthen_complete_answer_shape_10_any_of_1_items_data {
  struct thinkthen_complete_answer_shape_8_any_of_1_items_list array;
  uint32_t null;
} thinkthen_complete_answer_shape_10_any_of_1_items_data;

typedef struct thinkthen_complete_answer_shape_10_any_of_1_items {
  uint32_t kind;
  union thinkthen_complete_answer_shape_10_any_of_1_items_data data;
} thinkthen_complete_answer_shape_10_any_of_1_items;

typedef struct thinkthen_complete_answer_shape_10_any_of_1_items_list {
  const struct thinkthen_complete_answer_shape_10_any_of_1_items *const *data;
  size_t len;
} thinkthen_complete_answer_shape_10_any_of_1_items_list;

typedef union thinkthen_complete_answer_shape_10_data {
  struct thinkthen_complete_answer_shape_8_any_of_1_items_list array;
  uint32_t null;
  struct thinkthen_complete_answer_shape_10_any_of_1_items_list array2;
} thinkthen_complete_answer_shape_10_data;

typedef struct thinkthen_complete_answer_shape_10 {
  uint32_t kind;
  union thinkthen_complete_answer_shape_10_data data;
} thinkthen_complete_answer_shape_10;

typedef struct thinkthen_complete_answer_shape_10_presence {
  uint32_t presence;
  const struct thinkthen_complete_answer_shape_10 *value;
} thinkthen_complete_answer_shape_10_presence;

typedef struct thinkthen_complete_settled_original {
  const struct thinkthen_complete_json *value;
} thinkthen_complete_settled_original;

typedef struct thinkthen_complete_settled_original_list {
  const struct thinkthen_complete_settled_original *const *data;
  size_t len;
} thinkthen_complete_settled_original_list;

typedef union thinkthen_complete_answer_shape_9_any_of_1_items_data {
  struct thinkthen_complete_settled_original_list array;
  uint32_t null;
} thinkthen_complete_answer_shape_9_any_of_1_items_data;

typedef struct thinkthen_complete_answer_shape_9_any_of_1_items {
  uint32_t kind;
  union thinkthen_complete_answer_shape_9_any_of_1_items_data data;
} thinkthen_complete_answer_shape_9_any_of_1_items;

typedef struct thinkthen_complete_answer_shape_9_any_of_1_items_list {
  const struct thinkthen_complete_answer_shape_9_any_of_1_items *const *data;
  size_t len;
} thinkthen_complete_answer_shape_9_any_of_1_items_list;

typedef union thinkthen_complete_answer_shape_9_data {
  struct thinkthen_complete_settled_original_list array;
  uint32_t null;
  struct thinkthen_complete_answer_shape_9_any_of_1_items_list array2;
} thinkthen_complete_answer_shape_9_data;

typedef struct thinkthen_complete_answer_shape_9 {
  uint32_t kind;
  union thinkthen_complete_answer_shape_9_data data;
} thinkthen_complete_answer_shape_9;

typedef struct thinkthen_complete_answer_shape_9_presence {
  uint32_t presence;
  const struct thinkthen_complete_answer_shape_9 *value;
} thinkthen_complete_answer_shape_9_presence;

typedef struct thinkthen_complete_settled_answer_filter {
  struct thinkthen_complete_utf8 function;
  struct thinkthen_complete_answer_shape_4_presence index;
  struct thinkthen_complete_answer_shape_3_presence pick;
  struct thinkthen_complete_answer_shape_10_presence probability;
  const struct thinkthen_complete_answer_shape_5 *resolution;
  struct thinkthen_complete_answer_shape_9_presence value;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_settled_answer_filter;

typedef struct thinkthen_complete_integer_list {
  const uint64_t *data;
  size_t len;
} thinkthen_complete_integer_list;

typedef union thinkthen_complete_answer_shape_11_any_of_1_items_data {
  struct thinkthen_complete_integer_list array;
  uint32_t null;
} thinkthen_complete_answer_shape_11_any_of_1_items_data;

typedef struct thinkthen_complete_answer_shape_11_any_of_1_items {
  uint32_t kind;
  union thinkthen_complete_answer_shape_11_any_of_1_items_data data;
} thinkthen_complete_answer_shape_11_any_of_1_items;

typedef struct thinkthen_complete_answer_shape_11_any_of_1_items_list {
  const struct thinkthen_complete_answer_shape_11_any_of_1_items *const *data;
  size_t len;
} thinkthen_complete_answer_shape_11_any_of_1_items_list;

typedef union thinkthen_complete_answer_shape_11_data {
  struct thinkthen_complete_integer_list array;
  uint32_t null;
  struct thinkthen_complete_answer_shape_11_any_of_1_items_list array2;
} thinkthen_complete_answer_shape_11_data;

typedef struct thinkthen_complete_answer_shape_11 {
  uint32_t kind;
  union thinkthen_complete_answer_shape_11_data data;
} thinkthen_complete_answer_shape_11;

typedef struct thinkthen_complete_answer_shape_11_presence {
  uint32_t presence;
  const struct thinkthen_complete_answer_shape_11 *value;
} thinkthen_complete_answer_shape_11_presence;

typedef struct thinkthen_complete_settled_answer_rank {
  struct thinkthen_complete_utf8 function;
  struct thinkthen_complete_answer_shape_4_presence index;
  struct thinkthen_complete_answer_shape_3_presence pick;
  struct thinkthen_complete_answer_shape_10_presence probability;
  const struct thinkthen_complete_answer_shape_5 *resolution;
  struct thinkthen_complete_answer_shape_11_presence value;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_settled_answer_rank;

typedef struct thinkthen_complete_answer_shape_12 {
  const struct thinkthen_complete_json *value;
} thinkthen_complete_answer_shape_12;

typedef struct thinkthen_complete_answer_shape_12_presence {
  uint32_t presence;
  const struct thinkthen_complete_answer_shape_12 *value;
} thinkthen_complete_answer_shape_12_presence;

typedef struct thinkthen_complete_settled_answer_find {
  struct thinkthen_complete_utf8 function;
  struct thinkthen_complete_answer_shape_4_presence index;
  struct thinkthen_complete_answer_shape_3_presence pick;
  struct thinkthen_complete_answer_shape_2_presence probability;
  const struct thinkthen_complete_answer_shape_5 *resolution;
  struct thinkthen_complete_answer_shape_12_presence value;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_settled_answer_find;

typedef struct thinkthen_complete_settled_recognition_entities {
  struct thinkthen_complete_entity_list entities;
  struct thinkthen_complete_entity_edge_list_presence relations;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_settled_recognition_entities;

typedef struct thinkthen_complete_settled_recognition_mode_proposals {
  const struct thinkthen_complete_boundary_mode *mode;
  struct thinkthen_complete_boundary_proposal_list proposals;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_settled_recognition_mode_proposals;

typedef union thinkthen_complete_settled_recognition_data {
  const struct thinkthen_complete_settled_recognition_entities *entities;
  const struct thinkthen_complete_settled_recognition_mode_proposals *mode_proposals;
} thinkthen_complete_settled_recognition_data;

typedef struct thinkthen_complete_settled_recognition {
  uint32_t kind;
  union thinkthen_complete_settled_recognition_data data;
} thinkthen_complete_settled_recognition;

typedef union thinkthen_complete_answer_shape_13_any_of_1_items_data {
  const struct thinkthen_complete_settled_recognition *object;
  uint32_t null;
} thinkthen_complete_answer_shape_13_any_of_1_items_data;

typedef struct thinkthen_complete_answer_shape_13_any_of_1_items {
  uint32_t kind;
  union thinkthen_complete_answer_shape_13_any_of_1_items_data data;
} thinkthen_complete_answer_shape_13_any_of_1_items;

typedef struct thinkthen_complete_answer_shape_13_any_of_1_items_list {
  const struct thinkthen_complete_answer_shape_13_any_of_1_items *const *data;
  size_t len;
} thinkthen_complete_answer_shape_13_any_of_1_items_list;

typedef union thinkthen_complete_answer_shape_13_data {
  const struct thinkthen_complete_settled_recognition *object;
  uint32_t null;
  struct thinkthen_complete_answer_shape_13_any_of_1_items_list array;
} thinkthen_complete_answer_shape_13_data;

typedef struct thinkthen_complete_answer_shape_13 {
  uint32_t kind;
  union thinkthen_complete_answer_shape_13_data data;
} thinkthen_complete_answer_shape_13;

typedef struct thinkthen_complete_answer_shape_13_presence {
  uint32_t presence;
  const struct thinkthen_complete_answer_shape_13 *value;
} thinkthen_complete_answer_shape_13_presence;

typedef struct thinkthen_complete_settled_answer_recognize {
  struct thinkthen_complete_utf8 function;
  struct thinkthen_complete_answer_shape_4_presence index;
  struct thinkthen_complete_answer_shape_3_presence pick;
  struct thinkthen_complete_answer_shape_10_presence probability;
  const struct thinkthen_complete_answer_shape_5 *resolution;
  struct thinkthen_complete_answer_shape_13_presence value;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_settled_answer_recognize;

typedef struct thinkthen_complete_settled_relations {
  struct thinkthen_complete_related_entity_edge_list value;
} thinkthen_complete_settled_relations;

typedef union thinkthen_complete_answer_shape_14_any_of_1_items_data {
  const struct thinkthen_complete_settled_relations *array;
  uint32_t null;
} thinkthen_complete_answer_shape_14_any_of_1_items_data;

typedef struct thinkthen_complete_answer_shape_14_any_of_1_items {
  uint32_t kind;
  union thinkthen_complete_answer_shape_14_any_of_1_items_data data;
} thinkthen_complete_answer_shape_14_any_of_1_items;

typedef struct thinkthen_complete_answer_shape_14_any_of_1_items_list {
  const struct thinkthen_complete_answer_shape_14_any_of_1_items *const *data;
  size_t len;
} thinkthen_complete_answer_shape_14_any_of_1_items_list;

typedef union thinkthen_complete_answer_shape_14_data {
  const struct thinkthen_complete_settled_relations *array;
  uint32_t null;
  struct thinkthen_complete_answer_shape_14_any_of_1_items_list array2;
} thinkthen_complete_answer_shape_14_data;

typedef struct thinkthen_complete_answer_shape_14 {
  uint32_t kind;
  union thinkthen_complete_answer_shape_14_data data;
} thinkthen_complete_answer_shape_14;

typedef struct thinkthen_complete_answer_shape_14_presence {
  uint32_t presence;
  const struct thinkthen_complete_answer_shape_14 *value;
} thinkthen_complete_answer_shape_14_presence;

typedef struct thinkthen_complete_settled_answer_relate {
  struct thinkthen_complete_utf8 function;
  struct thinkthen_complete_answer_shape_4_presence index;
  struct thinkthen_complete_answer_shape_3_presence pick;
  struct thinkthen_complete_answer_shape_10_presence probability;
  const struct thinkthen_complete_answer_shape_5 *resolution;
  struct thinkthen_complete_answer_shape_14_presence value;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_settled_answer_relate;

typedef union thinkthen_complete_settled_judgment_data {
  uint32_t boolean;
  uint32_t null;
  struct thinkthen_complete_utf8 string;
  double number;
  struct thinkthen_complete_string_list array;
} thinkthen_complete_settled_judgment_data;

typedef struct thinkthen_complete_settled_judgment {
  uint32_t kind;
  union thinkthen_complete_settled_judgment_data data;
} thinkthen_complete_settled_judgment;

typedef struct thinkthen_complete_settled_judgment_map_entry {
  struct thinkthen_complete_utf8 name;
  const struct thinkthen_complete_settled_judgment *value;
} thinkthen_complete_settled_judgment_map_entry;

typedef struct thinkthen_complete_settled_judgment_map {
  const struct thinkthen_complete_settled_judgment_map_entry *data;
  size_t len;
} thinkthen_complete_settled_judgment_map;

typedef union thinkthen_complete_answer_shape_15_any_of_1_items_data {
  struct thinkthen_complete_settled_judgment_map object;
  uint32_t null;
} thinkthen_complete_answer_shape_15_any_of_1_items_data;

typedef struct thinkthen_complete_answer_shape_15_any_of_1_items {
  uint32_t kind;
  union thinkthen_complete_answer_shape_15_any_of_1_items_data data;
} thinkthen_complete_answer_shape_15_any_of_1_items;

typedef struct thinkthen_complete_answer_shape_15_any_of_1_items_list {
  const struct thinkthen_complete_answer_shape_15_any_of_1_items *const *data;
  size_t len;
} thinkthen_complete_answer_shape_15_any_of_1_items_list;

typedef union thinkthen_complete_answer_shape_15_data {
  struct thinkthen_complete_settled_judgment_map object;
  uint32_t null;
  struct thinkthen_complete_answer_shape_15_any_of_1_items_list array;
} thinkthen_complete_answer_shape_15_data;

typedef struct thinkthen_complete_answer_shape_15 {
  uint32_t kind;
  union thinkthen_complete_answer_shape_15_data data;
} thinkthen_complete_answer_shape_15;

typedef struct thinkthen_complete_answer_shape_15_presence {
  uint32_t presence;
  const struct thinkthen_complete_answer_shape_15 *value;
} thinkthen_complete_answer_shape_15_presence;

typedef struct thinkthen_complete_settled_answer_annotate {
  struct thinkthen_complete_utf8 function;
  struct thinkthen_complete_answer_shape_4_presence index;
  struct thinkthen_complete_answer_shape_3_presence pick;
  struct thinkthen_complete_answer_shape_2_presence probability;
  const struct thinkthen_complete_answer_shape_5 *resolution;
  struct thinkthen_complete_answer_shape_15_presence value;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_settled_answer_annotate;

typedef union thinkthen_complete_settled_answer_data {
  const struct thinkthen_complete_settled_answer_decide *decide;
  const struct thinkthen_complete_settled_answer_choose *choose;
  const struct thinkthen_complete_settled_answer_tag *tag;
  const struct thinkthen_complete_settled_answer_score *score;
  const struct thinkthen_complete_settled_answer_filter *filter;
  const struct thinkthen_complete_settled_answer_rank *rank;
  const struct thinkthen_complete_settled_answer_find *find;
  const struct thinkthen_complete_settled_answer_recognize *recognize;
  const struct thinkthen_complete_settled_answer_relate *relate;
  const struct thinkthen_complete_settled_answer_annotate *annotate;
} thinkthen_complete_settled_answer_data;

typedef struct thinkthen_complete_settled_answer {
  uint32_t kind;
  union thinkthen_complete_settled_answer_data data;
} thinkthen_complete_settled_answer;

typedef struct thinkthen_complete_settled_answer_presence {
  uint32_t presence;
  const struct thinkthen_complete_settled_answer *value;
} thinkthen_complete_settled_answer_presence;

typedef struct thinkthen_complete_facts {
  struct thinkthen_complete_attempt_list_presence attempts;
  uint64_t cache_answers;
  const struct thinkthen_complete_call_id *call_id;
  struct thinkthen_complete_utf8_presence estimated_cost_usd;
  struct thinkthen_complete_u32_presence held_model_mismatch;
  struct thinkthen_complete_u64_presence input_tokens;
  uint64_t largest_request_bytes;
  struct thinkthen_complete_u64_presence largest_request_estimated_input_tokens;
  struct thinkthen_complete_utf8_presence model;
  struct thinkthen_complete_u64_presence output_tokens;
  uint64_t records;
  uint64_t requests_sent;
  double seconds;
  struct thinkthen_complete_utf8 token_estimate_method;
  struct thinkthen_complete_persistence_observation_presence usage_persistence;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_facts;

typedef struct thinkthen_complete_facts_presence {
  uint32_t presence;
  const struct thinkthen_complete_facts *value;
} thinkthen_complete_facts_presence;

typedef struct thinkthen_complete_input_denial_initial_request {
  struct thinkthen_complete_utf8 kind;
  uint64_t limit;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_input_denial_initial_request;

typedef struct thinkthen_complete_input_denial_additional_request {
  struct thinkthen_complete_utf8 kind;
  uint64_t limit;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_input_denial_additional_request;

typedef struct thinkthen_complete_input_denial_retry {
  struct thinkthen_complete_utf8 kind;
  uint64_t last_status;
  uint64_t limit;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_input_denial_retry;

typedef union thinkthen_complete_input_denial_data {
  const struct thinkthen_complete_input_denial_initial_request *initial_request;
  const struct thinkthen_complete_input_denial_additional_request *additional_request;
  const struct thinkthen_complete_input_denial_retry *retry;
} thinkthen_complete_input_denial_data;

typedef struct thinkthen_complete_input_denial {
  uint32_t kind;
  union thinkthen_complete_input_denial_data data;
} thinkthen_complete_input_denial;

typedef struct thinkthen_complete_input_denial_presence {
  uint32_t presence;
  const struct thinkthen_complete_input_denial *value;
} thinkthen_complete_input_denial_presence;

typedef struct thinkthen_complete_failure_kind {
  uint32_t kind;
} thinkthen_complete_failure_kind;

typedef struct thinkthen_complete_send_denial_before_first_send {
  struct thinkthen_complete_utf8 kind;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_send_denial_before_first_send;

typedef struct thinkthen_complete_send_denial_before_additional_send {
  struct thinkthen_complete_utf8 kind;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_send_denial_before_additional_send;

typedef struct thinkthen_complete_send_denial_before_retry {
  struct thinkthen_complete_utf8 kind;
  uint64_t last_status;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_send_denial_before_retry;

typedef union thinkthen_complete_send_denial_data {
  const struct thinkthen_complete_send_denial_before_first_send *before_first_send;
  const struct thinkthen_complete_send_denial_before_additional_send *before_additional_send;
  const struct thinkthen_complete_send_denial_before_retry *before_retry;
} thinkthen_complete_send_denial_data;

typedef struct thinkthen_complete_send_denial {
  uint32_t kind;
  union thinkthen_complete_send_denial_data data;
} thinkthen_complete_send_denial;

typedef struct thinkthen_complete_send_denial_presence {
  uint32_t presence;
  const struct thinkthen_complete_send_denial *value;
} thinkthen_complete_send_denial_presence;

typedef struct thinkthen_complete_stop_cause {
  uint32_t kind;
} thinkthen_complete_stop_cause;

typedef struct thinkthen_complete_stopped {
  struct thinkthen_complete_u64_presence at;
  const struct thinkthen_complete_stop_cause *cause;
  uint32_t retryable;
  struct thinkthen_complete_u64_presence status;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_stopped;

typedef struct thinkthen_complete_error {
  struct thinkthen_complete_input_denial_presence estimated_input_denial;
  const struct thinkthen_complete_failure_kind *kind;
  struct thinkthen_complete_utf8 message;
  uint32_t retryable;
  struct thinkthen_complete_send_denial_presence send_budget_denial;
  const struct thinkthen_complete_stopped *stopped;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_error;

typedef struct thinkthen_complete_call_error {
  const struct thinkthen_complete_error *error;
  struct thinkthen_complete_facts_presence facts;
  const struct thinkthen_complete_answer_resolution *resolution;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_call_error;

typedef struct thinkthen_complete_call_error_presence {
  uint32_t presence;
  const struct thinkthen_complete_call_error *value;
} thinkthen_complete_call_error_presence;

typedef struct thinkthen_complete_session_packet_terminal {
  struct thinkthen_complete_settled_answer_presence answer;
  struct thinkthen_complete_facts_presence facts;
  struct thinkthen_complete_call_error_presence failure;
  struct thinkthen_complete_utf8 kind;
  const struct thinkthen_complete_answer_resolution *resolution;
  struct thinkthen_complete_extensions extensions;
} thinkthen_complete_session_packet_terminal;

typedef union thinkthen_complete_session_packet_data {
  const struct thinkthen_complete_session_packet_decide_row *decide_row;
  const struct thinkthen_complete_session_packet_choose_row *choose_row;
  const struct thinkthen_complete_session_packet_tag_row *tag_row;
  const struct thinkthen_complete_session_packet_score_row *score_row;
  const struct thinkthen_complete_session_packet_filter_row *filter_row;
  const struct thinkthen_complete_session_packet_annotate_row *annotate_row;
  const struct thinkthen_complete_session_packet_decide_aggregate *decide_aggregate;
  const struct thinkthen_complete_session_packet_choose_aggregate *choose_aggregate;
  const struct thinkthen_complete_session_packet_tag_aggregate *tag_aggregate;
  const struct thinkthen_complete_session_packet_score_aggregate *score_aggregate;
  const struct thinkthen_complete_session_packet_filter_aggregate *filter_aggregate;
  const struct thinkthen_complete_session_packet_rank_aggregate *rank_aggregate;
  const struct thinkthen_complete_session_packet_find_aggregate *find_aggregate;
  const struct thinkthen_complete_session_packet_annotate_aggregate *annotate_aggregate;
  const struct thinkthen_complete_session_packet_recognize_aggregate *recognize_aggregate;
  const struct thinkthen_complete_session_packet_relate_aggregate *relate_aggregate;
  const struct thinkthen_complete_session_packet_observation *observation;
  const struct thinkthen_complete_session_packet_terminal *terminal;
} thinkthen_complete_session_packet_data;

typedef struct thinkthen_complete_session_packet {
  uint32_t kind;
  union thinkthen_complete_session_packet_data data;
} thinkthen_complete_session_packet;

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
struct thinkthen_complete_utf8 thinkthen_call_error_facts_json(const struct thinkthen_call_error *owner);

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
struct thinkthen_complete_utf8 thinkthen_call_error_message(const struct thinkthen_call_error *owner);

/*
 Borrow the native one-based failing record position until error_free.
 NULL means the native position is absent, or owner is NULL. Never free or
 modify the returned pointer. It survives engine destruction and thread exit;
 immutable reads may run on any thread while the independent error stays live.
 # Safety
 owner is NULL or live for every reader of the returned position. Free the
 error only after all concurrent operations and borrowed reads finish.
 */
const size_t *thinkthen_call_error_position(const struct thinkthen_call_error *owner);

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
struct thinkthen_complete_utf8 thinkthen_call_result_json(const struct thinkthen_call_result *owner);

/*
 Finish this engine's current usage deltas and observe their persistence.
 Only usage-lock acquisition has a deadline; other filesystem work can take
 longer. Written covers current deltas only, not future calls or other engines.
 Failed carries static native safe advice readable after engine destruction
 while the library remains loaded. Every nonzero return preserves outputs.
 # Safety
 The engine and output obligations of usage_persistence apply.
 */
int thinkthen_engine_finish_usage_status(const struct thinkthen_engine *engine,
                                         struct thinkthen_complete_usage_persistence *out_state,
                                         struct thinkthen_complete_utf8 *out_advice);

/*
 Free an engine; null is ignored.

 # Safety

 `engine` is null or a live engine no other call is using.
 Free an engine. NULL is accepted and ignored. Free it only after every
 call on it has returned.
 */
void thinkthen_engine_free(struct thinkthen_engine *engine);

/*
 Build from the environment without sending. Failure returns NULL and an
 independent owned error; success writes NULL to error. NULL error discards
 diagnostics. Free returned errors once with thinkthen_call_error_free.
 No thread-local diagnostic slot changes.
 # Safety
 error is NULL or aligned writable pointer storage for the call.
 */
struct thinkthen_engine *thinkthen_engine_new(struct thinkthen_call_error **error);

/*
 Build from environment plus canonical typed settings. NULL uses environment alone.
 Copies the finite aligned carrier tree; no foreign pointer survives return.
 Construction sends nothing. Failure returns NULL and publishes an independent owned error.
 Success writes NULL to error. NULL error discards diagnostics; free an owned error once
 with thinkthen_call_error_free after all readers finish. No thread-local slot changes.
 # Safety
 settings is NULL or a live aligned settings carrier tree with counted UTF-8 extents.
 error is NULL or aligned writable pointer storage, separate from the input tree.
 */
struct thinkthen_engine *thinkthen_engine_new_typed(const struct thinkthen_request_engine_settings *settings,
                                                    struct thinkthen_call_error **error);

/*
 Build from environment plus closed UTF-8 JSON settings. NULL uses the environment.
 Construction sends nothing. Failure returns NULL and publishes an independent owned error.
 Success writes NULL to error. NULL error discards diagnostics; free an owned error once
 with thinkthen_call_error_free after all readers finish. No thread-local slot changes.
 # Safety
 settings_json is NULL or live NUL-terminated UTF-8 for the call.
 error is NULL or aligned writable pointer storage, separate from the input.
 */
struct thinkthen_engine *thinkthen_engine_new_with(const char *settings_json,
                                                   struct thinkthen_call_error **error);

/*
 Observe this engine's live usage persistence without waiting for a writer.
 On success assign both outputs; Failed is an observation, not an operation
 error. Advice is a static counted UTF-8 view, or NULL with zero length.
 Success preserves the calling-thread session diagnostic.
 # Safety
 engine is live for the call. Both outputs are nonnull, aligned, writable,
 nonoverlapping slots. Destruction waits for concurrent callers to return.
 */
int thinkthen_engine_usage_persistence(const struct thinkthen_engine *engine,
                                       struct thinkthen_complete_usage_persistence *out_state,
                                       struct thinkthen_complete_utf8 *out_advice);

/*
 The calling thread's last code here; with a null engine, the usage code.

 # Safety

 `engine` is null or a live engine.
 The code of the calling thread's last failure on this engine: the value
 the failing call returned, THINKTHEN_OK when nothing failed yet. Success
 does not clear it, so read it when a call fails. With a null engine it
 is THINKTHEN_EUSAGE because no engine holds a failure. Read construction
 failures from the constructor's independent thinkthen_call_error.
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
 any failure it names that nothing failed yet. With a null engine it names
 that no engine came. Read construction failures from the independent
 thinkthen_call_error returned by the constructor.
 */
const char *thinkthen_error_message(const struct thinkthen_engine *engine);

/*
 1 when the calling thread's last failure here could pass later.

 # Safety

 `engine` is null or a live engine.
 Whether the same call could pass later: 1 for a backend status the
 engine retries, such as busy or failing; 0 for a transport failure,
 which may already have reached the backend, for a refused key, and for
 every kind but the backend kind. Zero when nothing failed or engine is null.
 Read construction retryability from the constructor's owned error.
 */
int thinkthen_error_retryable(const struct thinkthen_engine *engine);

/*
 Borrow the immutable last engine/thread failure; NULL engine returns NULL.
 NULL means that slot holds no failure. Getters and successful engine calls preserve
 the view, including code, safe message, retryability and typed optional facts.
 Root and every nested value remain stable until this thread next fails on this
 engine, engine_free or thread exit. Construction errors have their own owner.
 Other threads cannot replace or move this thread slot.
 # Safety
 engine is NULL or live for the call and every borrowed engine view.
 */
const struct thinkthen_error_details *thinkthen_error_view(const struct thinkthen_engine *engine);

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
                              struct thinkthen_question_author_info *out);

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
 from immediate session diagnostics and owned construction errors.
 # Safety
 engine is live; json has readable counted UTF-8; out is aligned writable storage.
 */
int thinkthen_question_parse(const struct thinkthen_engine *engine,
                             uint32_t role,
                             struct thinkthen_string json,
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
                             const struct thinkthen_complete_json **out);

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
                                       const struct thinkthen_named_request *request,
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
                                      struct thinkthen_complete_utf8 function,
                                      const struct thinkthen_request_arguments *arguments,
                                      struct thinkthen_complete_utf8 surface_name,
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
                                       struct thinkthen_complete_utf8 function,
                                       const struct thinkthen_request_arguments *arguments,
                                       struct thinkthen_complete_utf8 surface_name,
                                       struct thinkthen_value **out);

/*
 Preview a full canonical Request through existing Rust admission and Plan serialization.
 Sends nothing and reads no key or cache. Unknown fields and unsupported previews refuse.
 Success transfers an immutable independent owner; free once with value_free.
 Source bytes and engine may be freed on return. Failures preserve out and occupy
 only the calling thread immediate session diagnostic, independently of engine diagnostics and owned construction errors.
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
 distinct storage. producer/context obey CallProducer and outlive the call.
 */
int32_t thinkthen_run_call(const struct thinkthen_engine *engine,
                           const struct thinkthen_named_request *request,
                           thinkthen_call_producer producer,
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
const struct thinkthen_error_details *thinkthen_session_error_view(void);

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
                                   const struct thinkthen_request_reader_failure *failure);

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
                                             const struct thinkthen_named_request *request,
                                             struct thinkthen_complete_utf8 surface,
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
                                             struct thinkthen_complete_utf8 function,
                                             const struct thinkthen_request_arguments *arguments,
                                             struct thinkthen_complete_utf8 surface_name,
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
                                 const struct thinkthen_request_session_descriptor *descriptor,
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
                                  const struct thinkthen_complete_session_packet **out);

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
                                     const struct thinkthen_request_session_descriptor *descriptor,
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
                         const struct thinkthen_complete_json **out);

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
