/* Generated from selected canonical C declarations; do not edit. */
typedef struct thinkthen_engine thinkthen_engine;
typedef struct thinkthen_session thinkthen_session;
typedef struct thinkthen_session_result thinkthen_session_result;
typedef struct thinkthen_complete_usage_persistence_v1 {
    uint32_t kind;
} thinkthen_complete_usage_persistence_v1;
typedef struct thinkthen_complete_utf8_v1 {
    const char *data;
    size_t len;
} thinkthen_complete_utf8_v1;
typedef struct thinkthen_named_request_v1 {
    struct thinkthen_complete_utf8_v1 function;
    struct thinkthen_complete_utf8_v1 question_json;
    struct thinkthen_complete_utf8_v1 input_json;
    struct thinkthen_complete_utf8_v1 options_json;
} thinkthen_named_request_v1;
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
typedef struct thinkthen_complete_u64_presence_v1 {
    uint32_t presence;
    uint64_t value;
} thinkthen_complete_u64_presence_v1;
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
typedef struct thinkthen_complete_json_presence_v1 {
    uint32_t presence;
    const struct thinkthen_complete_json_v1 *value;
} thinkthen_complete_json_presence_v1;
typedef struct thinkthen_complete_utf8_presence_v1 {
    uint32_t presence;
    struct thinkthen_complete_utf8_v1 value;
} thinkthen_complete_utf8_presence_v1;
typedef struct thinkthen_complete_attempt_outcome_v1 {
    uint32_t kind;
} thinkthen_complete_attempt_outcome_v1;
typedef struct thinkthen_complete_sdk_request_id_v1 {
    struct thinkthen_complete_utf8_v1 value;
} thinkthen_complete_sdk_request_id_v1;
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
typedef struct thinkthen_complete_readable_question_decide_v1 {
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
} thinkthen_complete_readable_question_decide_v1;
typedef struct thinkthen_complete_readable_question_choose_v1 {
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
} thinkthen_complete_readable_question_choose_v1;
typedef struct thinkthen_complete_readable_question_tag_v1 {
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
} thinkthen_complete_readable_question_tag_v1;
typedef struct thinkthen_complete_readable_question_score_v1 {
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
    const struct thinkthen_complete_readable_question_v1 *question;
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
    const struct thinkthen_complete_readable_question_v1 *question;
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
    const struct thinkthen_complete_readable_question_v1 *question;
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
    const struct thinkthen_complete_readable_question_v1 *question;
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
    const struct thinkthen_complete_readable_question_v1 *question;
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
    const struct thinkthen_complete_readable_question_v1 *question;
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
    const struct thinkthen_complete_readable_question_v1 *question;
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
    const struct thinkthen_complete_readable_question_v1 *question;
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
    const struct thinkthen_complete_readable_question_v1 *question;
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
typedef struct thinkthen_complete_readable_question_2_v1 {
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
} thinkthen_complete_readable_question_2_v1;
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
    const struct thinkthen_complete_readable_question_2_v1 *question;
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
typedef struct thinkthen_complete_u32_presence_v1 {
    uint32_t presence;
    uint32_t value;
} thinkthen_complete_u32_presence_v1;
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
typedef struct thinkthen_complete_readable_question_3_v1 {
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
} thinkthen_complete_readable_question_3_v1;
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
    const struct thinkthen_complete_readable_question_3_v1 *question;
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
    const struct thinkthen_complete_readable_question_v1 *question;
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
    const struct thinkthen_complete_readable_question_v1 *question;
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
typedef struct thinkthen_complete_readable_question_4_v1 {
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
} thinkthen_complete_readable_question_4_v1;
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
    const struct thinkthen_complete_readable_question_4_v1 *question;
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
    const struct thinkthen_complete_readable_question_v1 *question;
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
typedef struct thinkthen_complete_call_id_v1 {
    struct thinkthen_complete_utf8_v1 value;
} thinkthen_complete_call_id_v1;
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
    struct thinkthen_complete_extensions_v1 extensions;
} thinkthen_complete_call_error_v1;
typedef struct thinkthen_complete_call_error_presence_v1 {
    uint32_t presence;
    const struct thinkthen_complete_call_error_v1 *value;
} thinkthen_complete_call_error_presence_v1;
typedef struct thinkthen_complete_session_packet_terminal_v1 {
    struct thinkthen_complete_facts_presence_v1 facts;
    struct thinkthen_complete_call_error_presence_v1 failure;
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
int thinkthen_engine_finish_usage_status_v1(const struct thinkthen_engine *engine,
struct thinkthen_complete_usage_persistence_v1 *out_state,
struct thinkthen_complete_utf8_v1 *out_advice);
void thinkthen_engine_free(struct thinkthen_engine *engine);
struct thinkthen_engine *thinkthen_engine_new_with(const char *settings_json);
int thinkthen_engine_usage_persistence_v1(const struct thinkthen_engine *engine,
struct thinkthen_complete_usage_persistence_v1 *out_state,
struct thinkthen_complete_utf8_v1 *out_advice);
int thinkthen_error_code(const struct thinkthen_engine *engine);
const char *thinkthen_error_message(const struct thinkthen_engine *engine);
void thinkthen_session_cancel(struct thinkthen_session *session);
const char *thinkthen_session_error_message(void);
int thinkthen_session_finish(struct thinkthen_session *session,
const char *failure_json,
size_t failure_len);
void thinkthen_session_free(struct thinkthen_session *session);
int thinkthen_session_new_named_with_surface(const struct thinkthen_engine *engine,
const struct thinkthen_named_request_v1 *request,
struct thinkthen_complete_utf8_v1 surface,
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
const struct thinkthen_complete_session_packet_v1 **out);
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
