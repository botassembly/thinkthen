/* Generated from selected canonical C declarations; do not edit. */
typedef struct thinkthen_call_error thinkthen_call_error;
typedef struct thinkthen_engine thinkthen_engine;
typedef struct thinkthen_session thinkthen_session;
typedef struct thinkthen_session_result thinkthen_session_result;
typedef struct thinkthen_value thinkthen_value;
typedef struct thinkthen_complete_utf8 {
    const char *data;
    size_t len;
} thinkthen_complete_utf8;
typedef struct thinkthen_complete_usage_persistence {
    uint32_t kind;
} thinkthen_complete_usage_persistence;
typedef struct thinkthen_complete_json_array {
    const struct thinkthen_complete_json *const *data;
    size_t len;
} thinkthen_complete_json_array;
typedef struct thinkthen_complete_json_entry {
    struct thinkthen_complete_utf8 name;
    const struct thinkthen_complete_json *value;
} thinkthen_complete_json_entry;
typedef struct thinkthen_complete_json_object {
    const struct thinkthen_complete_json_entry *data;
    size_t len;
} thinkthen_complete_json_object;
typedef union thinkthen_complete_json_data {
    uint32_t boolean;
    struct thinkthen_complete_utf8 number;
    struct thinkthen_complete_utf8 string;
    struct thinkthen_complete_json_array array;
    struct thinkthen_complete_json_object object;
} thinkthen_complete_json_data;
typedef struct thinkthen_complete_json {
    uint32_t kind;
    union thinkthen_complete_json_data data;
} thinkthen_complete_json;
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
typedef struct thinkthen_complete_extension {
    struct thinkthen_complete_utf8 name;
    struct thinkthen_complete_utf8 json;
    const struct thinkthen_complete_json *value;
} thinkthen_complete_extension;
typedef struct thinkthen_complete_extensions {
    const struct thinkthen_complete_extension *data;
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
typedef struct thinkthen_error_details {
    int code;
    int retryable;
    struct thinkthen_complete_utf8 message;
    const struct thinkthen_complete_immediate_facts *facts;
} thinkthen_error_details;
typedef struct thinkthen_named_request {
    struct thinkthen_complete_utf8 function;
    struct thinkthen_complete_utf8 question_json;
    struct thinkthen_complete_utf8 input_json;
    struct thinkthen_complete_utf8 options_json;
} thinkthen_named_request;
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
int thinkthen_engine_finish_usage_status(const struct thinkthen_engine *engine,
struct thinkthen_complete_usage_persistence *out_state,
struct thinkthen_complete_utf8 *out_advice);
void thinkthen_engine_free(struct thinkthen_engine *engine);
struct thinkthen_engine *thinkthen_engine_new_with(const char *settings_json,
struct thinkthen_call_error **error);
int thinkthen_engine_usage_persistence(const struct thinkthen_engine *engine,
struct thinkthen_complete_usage_persistence *out_state,
struct thinkthen_complete_utf8 *out_advice);
int thinkthen_error_code(const struct thinkthen_engine *engine);
const char *thinkthen_error_message(const struct thinkthen_engine *engine);
int thinkthen_error_retryable(const struct thinkthen_engine *engine);
const struct thinkthen_error_details *thinkthen_error_view(const struct thinkthen_engine *engine);
int thinkthen_request_plan_named_value(const struct thinkthen_engine *engine,
const struct thinkthen_named_request *request,
struct thinkthen_value **out);
void thinkthen_session_cancel(struct thinkthen_session *session);
const char *thinkthen_session_error_message(void);
const struct thinkthen_error_details *thinkthen_session_error_view(void);
int thinkthen_session_finish(struct thinkthen_session *session,
const char *failure_json,
size_t failure_len);
void thinkthen_session_free(struct thinkthen_session *session);
int thinkthen_session_new_named_with_surface(const struct thinkthen_engine *engine,
const struct thinkthen_named_request *request,
struct thinkthen_complete_utf8 surface,
struct thinkthen_session **out);
int thinkthen_session_next_packet(struct thinkthen_session *session,
uint32_t interest,
uint32_t *status,
struct thinkthen_session_result **out);
int thinkthen_session_push(struct thinkthen_session *session,
const char *descriptor_json,
size_t descriptor_len,
uint32_t *status);
void thinkthen_session_result_free(struct thinkthen_session_result *result);
int thinkthen_session_result_json(const struct thinkthen_session_result *result,
const char **out,
size_t *out_len);
int thinkthen_session_result_view(const struct thinkthen_session_result *result,
const struct thinkthen_complete_session_packet **out);
int thinkthen_session_try_push(struct thinkthen_session *session,
const char *descriptor_json,
size_t descriptor_len,
uint32_t *status);
int thinkthen_session_try_read(struct thinkthen_session *session,
uint32_t *status,
struct thinkthen_session_result **out);
int thinkthen_session_wait(const struct thinkthen_session *session,
uint32_t interest,
uint32_t timeout_ms,
uint32_t *out_ready);
void thinkthen_value_free(struct thinkthen_value *owner);
int thinkthen_value_view(const struct thinkthen_value *owner,
const struct thinkthen_complete_json **out);
