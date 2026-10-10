typedef struct thinkthen_cancel_token thinkthen_cancel_token;




























































































































































































































































typedef enum thinkthen_input_declaration_kind_v1 {

  THINKTHEN_DECLARATION_ABSENT_V1 = 0,

  THINKTHEN_DECLARATION_STRING_V1 = 1,

  THINKTHEN_DECLARATION_OBJECT_V1 = 2,
} thinkthen_input_declaration_kind_v1;


typedef enum thinkthen_input_property_kind_v1 {

  THINKTHEN_PROPERTY_STRING_V1 = 1,

  THINKTHEN_PROPERTY_NUMBER_V1 = 2,

  THINKTHEN_PROPERTY_BOOLEAN_V1 = 3,

  THINKTHEN_PROPERTY_STRING_LIST_V1 = 4,
} thinkthen_input_property_kind_v1;


typedef enum thinkthen_question_loader_role_v1 {

  THINKTHEN_LOAD_ATOMIC_V1 = 1,

  THINKTHEN_LOAD_SET_V1 = 2,

  THINKTHEN_LOAD_DYNAMIC_CHOOSE_V1 = 3,

  THINKTHEN_LOAD_RECOGNIZE_V1 = 4,

  THINKTHEN_LOAD_RELATE_V1 = 5,

  THINKTHEN_LOAD_RANK_V1 = 6,

  THINKTHEN_LOAD_RANK_SET_V1 = 7,

  THINKTHEN_LOAD_FIND_V1 = 8,
} thinkthen_question_loader_role_v1;


typedef struct thinkthen_batch thinkthen_batch;


typedef struct thinkthen_engine thinkthen_engine;


typedef struct thinkthen_image thinkthen_image;


typedef struct thinkthen_question thinkthen_question;


typedef struct thinkthen_result thinkthen_result;


typedef struct thinkthen_session thinkthen_session;


typedef struct thinkthen_session_result thinkthen_session_result;


typedef struct thinkthen_source thinkthen_source;


typedef struct thinkthen_string_v1 {

  const char *data;

  size_t len;
} thinkthen_string_v1;


typedef struct thinkthen_content_v1 {

  uint32_t kind;

  struct thinkthen_string_v1 data;
} thinkthen_content_v1;


typedef struct thinkthen_optional_content_v1 {

  int present;

  struct thinkthen_content_v1 value;
} thinkthen_optional_content_v1;


typedef struct thinkthen_optional_size_v1 {

  int present;

  size_t value;
} thinkthen_optional_size_v1;


typedef struct thinkthen_controls_v1 {

  int64_t deadline_ms;

  thinkthen_cancel_token *cancel;

  struct thinkthen_optional_content_v1 context;

  struct thinkthen_optional_size_v1 batch;

  int batch_max;

  int attempts;

  struct thinkthen_string_v1 surface;
} thinkthen_controls_v1;


typedef struct thinkthen_answer {

  int outcome;

  double probability;
} thinkthen_answer;

typedef struct thinkthen_complete_usage_persistence_v1 {
  uint32_t kind;
} thinkthen_complete_usage_persistence_v1;

typedef struct thinkthen_complete_utf8_v1 {
  const char *data;
  size_t len;
} thinkthen_complete_utf8_v1;


typedef struct thinkthen_optional_string_v1 {

  int present;

  struct thinkthen_string_v1 value;
} thinkthen_optional_string_v1;


typedef struct thinkthen_image_view_v1 {

  uint32_t media;

  const uint8_t *bytes;

  size_t bytes_len;

  uint32_t width;

  uint32_t height;

  struct thinkthen_optional_string_v1 filename;
} thinkthen_image_view_v1;


typedef struct thinkthen_optional_u64_v1 {

  int present;

  uint64_t value;
} thinkthen_optional_u64_v1;


typedef struct thinkthen_input_property_v1 {

  struct thinkthen_string_v1 name;

  uint32_t kind;
} thinkthen_input_property_v1;


typedef struct thinkthen_input_properties_v1 {

  const struct thinkthen_input_property_v1 *data;

  size_t len;
} thinkthen_input_properties_v1;


typedef struct thinkthen_strings_v1 {

  const struct thinkthen_string_v1 *data;

  size_t len;
} thinkthen_strings_v1;


typedef struct thinkthen_input_declaration_v1 {

  uint32_t kind;

  struct thinkthen_input_properties_v1 properties;

  struct thinkthen_strings_v1 required;
} thinkthen_input_declaration_v1;


typedef struct thinkthen_question_author_v1 {

  struct thinkthen_optional_string_v1 name;

  struct thinkthen_optional_u64_v1 wording_version;

  struct thinkthen_input_declaration_v1 item_schema;

  struct thinkthen_input_declaration_v1 context_schema;
} thinkthen_question_author_v1;


typedef struct thinkthen_optional_double_v1 {

  int present;

  double value;
} thinkthen_optional_double_v1;


typedef struct thinkthen_choice_v1 {

  struct thinkthen_string_v1 name;

  struct thinkthen_optional_content_v1 description;

  struct thinkthen_optional_double_v1 weight;
} thinkthen_choice_v1;


typedef struct thinkthen_choices_v1 {

  const struct thinkthen_choice_v1 *data;

  size_t len;
} thinkthen_choices_v1;


typedef struct thinkthen_rule_v1 {

  uint32_t kind;

  double low;

  double high;
} thinkthen_rule_v1;


typedef struct thinkthen_member_spec_v1 {

  struct thinkthen_string_v1 name;

  const struct thinkthen_question *question;
} thinkthen_member_spec_v1;


typedef struct thinkthen_member_specs_v1 {

  const struct thinkthen_member_spec_v1 *data;

  size_t len;
} thinkthen_member_specs_v1;


typedef struct thinkthen_relation_v1 {

  struct thinkthen_string_v1 name;

  struct thinkthen_string_v1 source;

  struct thinkthen_string_v1 target;

  struct thinkthen_optional_string_v1 reads;

  int either;

  int single;
} thinkthen_relation_v1;


typedef struct thinkthen_relations_v1 {

  const struct thinkthen_relation_v1 *data;

  size_t len;
} thinkthen_relations_v1;


typedef struct thinkthen_question_spec_v1 {

  uint32_t kind;

  struct thinkthen_content_v1 text;

  struct thinkthen_optional_content_v1 yes;

  struct thinkthen_optional_content_v1 no;

  struct thinkthen_choices_v1 choices;

  struct thinkthen_rule_v1 threshold;

  struct thinkthen_rule_v1 relation_threshold;

  struct thinkthen_optional_string_v1 model;

  struct thinkthen_optional_string_v1 profile;

  struct thinkthen_optional_size_v1 batch;

  int batch_max;

  int none;

  struct thinkthen_strings_v1 on;

  struct thinkthen_member_specs_v1 members;

  struct thinkthen_choices_v1 kinds;

  struct thinkthen_relations_v1 relations;

  struct thinkthen_optional_string_v1 name_pointer;

  struct thinkthen_optional_string_v1 kind_pointer;
} thinkthen_question_spec_v1;


typedef struct thinkthen_recognition_task_v1 {

  struct thinkthen_optional_string_v1 instructions;

  struct thinkthen_optional_string_v1 entity_definition;
} thinkthen_recognition_task_v1;


typedef struct thinkthen_question_member_v1 {

  struct thinkthen_string_v1 name;

  const struct thinkthen_question_view_v1 *question;
} thinkthen_question_member_v1;


typedef struct thinkthen_question_members_v1 {

  const struct thinkthen_question_member_v1 *data;

  size_t len;
} thinkthen_question_members_v1;


typedef struct thinkthen_question_view_v1 {

  uint32_t kind;

  struct thinkthen_content_v1 text;

  struct thinkthen_optional_content_v1 yes;

  struct thinkthen_optional_content_v1 no;

  struct thinkthen_choices_v1 choices;

  struct thinkthen_rule_v1 threshold;

  struct thinkthen_rule_v1 relation_threshold;

  struct thinkthen_optional_string_v1 model;

  struct thinkthen_optional_string_v1 profile;

  struct thinkthen_optional_size_v1 batch;

  int batch_max;

  int none;

  struct thinkthen_strings_v1 on;

  struct thinkthen_question_members_v1 members;

  struct thinkthen_choices_v1 kinds;

  struct thinkthen_relations_v1 relations;

  struct thinkthen_optional_string_v1 name_pointer;

  struct thinkthen_optional_string_v1 kind_pointer;
} thinkthen_question_view_v1;


typedef struct thinkthen_optional_question_v1 {

  int present;

  struct thinkthen_question_view_v1 value;
} thinkthen_optional_question_v1;


typedef struct thinkthen_probability_v1 {

  struct thinkthen_string_v1 name;

  double probability;
} thinkthen_probability_v1;


typedef struct thinkthen_probabilities_v1 {

  const struct thinkthen_probability_v1 *data;

  size_t len;
} thinkthen_probabilities_v1;


typedef struct thinkthen_named_answer_v1 {

  struct thinkthen_string_v1 pick;

  struct thinkthen_probabilities_v1 probabilities;

  struct thinkthen_optional_double_v1 confidence;
} thinkthen_named_answer_v1;


typedef struct thinkthen_score_answer_v1 {

  struct thinkthen_string_v1 level;

  struct thinkthen_probabilities_v1 probabilities;

  struct thinkthen_optional_double_v1 confidence;
} thinkthen_score_answer_v1;


typedef union thinkthen_answer_data_v1 {

  double probability;

  struct thinkthen_named_answer_v1 choice;

  struct thinkthen_probabilities_v1 tag;

  struct thinkthen_score_answer_v1 score;

  struct thinkthen_named_answer_v1 find;
} thinkthen_answer_data_v1;


typedef struct thinkthen_answer_v1 {

  uint32_t kind;

  union thinkthen_answer_data_v1 data;
} thinkthen_answer_v1;


typedef struct thinkthen_optional_answer_v1 {

  int present;

  struct thinkthen_answer_v1 value;
} thinkthen_optional_answer_v1;


typedef struct thinkthen_optional_rule_v1 {

  int present;

  struct thinkthen_rule_v1 value;
} thinkthen_optional_rule_v1;


typedef struct thinkthen_location_v1 {

  struct thinkthen_optional_string_v1 file;

  struct thinkthen_optional_size_v1 first_line;

  struct thinkthen_optional_size_v1 last_line;
} thinkthen_location_v1;


typedef struct thinkthen_optional_location_v1 {

  int present;

  struct thinkthen_location_v1 value;
} thinkthen_optional_location_v1;


typedef struct thinkthen_usage_v1 {

  uint64_t input_tokens;

  uint64_t output_tokens;
} thinkthen_usage_v1;


typedef struct thinkthen_optional_usage_v1 {

  int present;

  struct thinkthen_usage_v1 value;
} thinkthen_optional_usage_v1;


typedef struct thinkthen_profile_warning_v1 {

  struct thinkthen_string_v1 tuned_for;

  struct thinkthen_string_v1 running;
} thinkthen_profile_warning_v1;


typedef struct thinkthen_optional_profile_warning_v1 {

  int present;

  struct thinkthen_profile_warning_v1 value;
} thinkthen_optional_profile_warning_v1;


typedef struct thinkthen_batch_v1 {

  uint32_t kind;

  size_t records;
} thinkthen_batch_v1;


typedef struct thinkthen_optional_batch_v1 {

  int present;

  struct thinkthen_batch_v1 value;
} thinkthen_optional_batch_v1;


typedef struct thinkthen_batch_warning_v1 {

  struct thinkthen_batch_v1 tuned_for;

  struct thinkthen_batch_v1 running;
} thinkthen_batch_warning_v1;


typedef struct thinkthen_optional_batch_warning_v1 {

  int present;

  struct thinkthen_batch_warning_v1 value;
} thinkthen_optional_batch_warning_v1;


typedef struct thinkthen_optional_u16_v1 {

  int present;

  uint16_t value;
} thinkthen_optional_u16_v1;


typedef struct thinkthen_attempt_v1 {

  uint64_t ordinal;

  struct thinkthen_string_v1 request_sha256;

  uint64_t wall_ms;

  uint32_t outcome;

  struct thinkthen_string_v1 sdk_request_id;

  struct thinkthen_optional_u16_v1 status;

  struct thinkthen_optional_u64_v1 server_ms;

  struct thinkthen_optional_string_v1 request_id;
} thinkthen_attempt_v1;


typedef struct thinkthen_attempts_v1 {

  const struct thinkthen_attempt_v1 *data;

  size_t len;
} thinkthen_attempts_v1;


typedef struct thinkthen_optional_attempts_v1 {

  int present;

  struct thinkthen_attempts_v1 value;
} thinkthen_optional_attempts_v1;


typedef struct thinkthen_optional_discriminator_v1 {

  int present;

  uint32_t value;
} thinkthen_optional_discriminator_v1;


typedef struct thinkthen_question_source_v1 {

  uint32_t origin;

  struct thinkthen_string_v1 answered_by;
} thinkthen_question_source_v1;


typedef struct thinkthen_question_sources_v1 {

  const struct thinkthen_question_source_v1 *data;

  size_t len;
} thinkthen_question_sources_v1;


typedef union thinkthen_observation_identity_data_v1 {

  struct thinkthen_string_v1 observation_id;

  struct thinkthen_string_v1 failure_id;
} thinkthen_observation_identity_data_v1;


typedef struct thinkthen_observation_identity_v1 {

  uint32_t kind;

  union thinkthen_observation_identity_data_v1 data;
} thinkthen_observation_identity_v1;


typedef struct thinkthen_observation_identities_v1 {

  const struct thinkthen_observation_identity_v1 *data;

  size_t len;
} thinkthen_observation_identities_v1;


typedef struct thinkthen_meta_v1 {

  struct thinkthen_string_v1 tool;

  struct thinkthen_optional_string_v1 question_sha256;

  struct thinkthen_optional_string_v1 questions_sha256;

  struct thinkthen_string_v1 url;

  struct thinkthen_string_v1 model;

  struct thinkthen_optional_usage_v1 usage;

  uint64_t requests_sent;

  int cached;

  struct thinkthen_strings_v1 requests;

  size_t failed_questions;

  struct thinkthen_optional_profile_warning_v1 profile_warning;

  struct thinkthen_optional_batch_v1 batch_setting;

  struct thinkthen_optional_batch_warning_v1 batch_warning;

  struct thinkthen_optional_string_v1 context_sha256;

  struct thinkthen_optional_attempts_v1 attempts;

  struct thinkthen_optional_discriminator_v1 origin;

  struct thinkthen_question_sources_v1 question_sources;

  struct thinkthen_observation_identities_v1 observations;

  struct thinkthen_optional_string_v1 answered_by;
} thinkthen_meta_v1;


typedef struct thinkthen_image_views_v1 {

  const struct thinkthen_image_view_v1 *data;

  size_t len;
} thinkthen_image_views_v1;


typedef struct thinkthen_optional_image_views_v1 {

  int present;

  struct thinkthen_image_views_v1 value;
} thinkthen_optional_image_views_v1;


typedef struct thinkthen_row_v1 {

  struct thinkthen_string_v1 answer_id;

  struct thinkthen_optional_content_v1 input;

  struct thinkthen_optional_question_v1 question;

  struct thinkthen_optional_answer_v1 answer;

  struct thinkthen_optional_rule_v1 threshold;

  struct thinkthen_optional_location_v1 position;

  struct thinkthen_optional_string_v1 input_file;

  struct thinkthen_meta_v1 meta;

  struct thinkthen_optional_image_views_v1 images;
} thinkthen_row_v1;


typedef union thinkthen_decide_value_data_v1 {

  int boolean;

  struct thinkthen_content_v1 authored;
} thinkthen_decide_value_data_v1;


typedef struct thinkthen_decide_value_v1 {

  uint32_t kind;

  union thinkthen_decide_value_data_v1 data;
} thinkthen_decide_value_v1;


typedef union thinkthen_member_value_data_v1 {

  struct thinkthen_decide_value_v1 decide;

  struct thinkthen_optional_string_v1 choose;

  struct thinkthen_strings_v1 tag;

  double score;
} thinkthen_member_value_data_v1;


typedef struct thinkthen_member_value_v1 {

  uint32_t kind;

  union thinkthen_member_value_data_v1 data;
} thinkthen_member_value_v1;


typedef struct thinkthen_member_success_v1 {

  struct thinkthen_string_v1 answer_id;

  struct thinkthen_member_value_v1 value;

  struct thinkthen_answer_v1 answer;

  struct thinkthen_rule_v1 threshold;
} thinkthen_member_success_v1;


typedef struct thinkthen_member_failure_v1 {

  struct thinkthen_string_v1 failure_id;

  uint32_t cause;
} thinkthen_member_failure_v1;


typedef union thinkthen_member_data_v1 {

  struct thinkthen_member_success_v1 success;

  struct thinkthen_member_failure_v1 failure;
} thinkthen_member_data_v1;


typedef struct thinkthen_member_v1 {

  struct thinkthen_string_v1 name;

  struct thinkthen_string_v1 request;

  struct thinkthen_question_view_v1 question;

  uint32_t state;

  union thinkthen_member_data_v1 data;
} thinkthen_member_v1;


typedef struct thinkthen_members_v1 {

  const struct thinkthen_member_v1 *data;

  size_t len;
} thinkthen_members_v1;


typedef struct thinkthen_annotate_view_v1 {

  struct thinkthen_row_v1 common;

  struct thinkthen_members_v1 answers;
} thinkthen_annotate_view_v1;


typedef struct thinkthen_choose_view_v1 {

  struct thinkthen_row_v1 common;

  struct thinkthen_optional_string_v1 value;
} thinkthen_choose_view_v1;


typedef struct thinkthen_decide_view_v1 {

  struct thinkthen_row_v1 common;

  struct thinkthen_decide_value_v1 value;
} thinkthen_decide_view_v1;


typedef struct thinkthen_reported_usage_v1 {

  int present;

  struct thinkthen_optional_u64_v1 input_tokens;

  struct thinkthen_optional_u64_v1 output_tokens;
} thinkthen_reported_usage_v1;


typedef struct thinkthen_source_detail_v1 {

  uint32_t origin;

  struct thinkthen_string_v1 answered_by;

  struct thinkthen_optional_size_v1 batch_size;
} thinkthen_source_detail_v1;


typedef struct thinkthen_source_details_v1 {

  const struct thinkthen_source_detail_v1 *data;

  size_t len;
} thinkthen_source_details_v1;


typedef struct thinkthen_input_view_v1 {

  struct thinkthen_optional_content_v1 original;

  struct thinkthen_optional_location_v1 position;

  struct thinkthen_optional_image_views_v1 images;
} thinkthen_input_view_v1;


typedef struct thinkthen_input_views_v1 {

  const struct thinkthen_input_view_v1 *data;

  size_t len;
} thinkthen_input_views_v1;


typedef struct thinkthen_details_v1 {

  struct thinkthen_optional_question_v1 question;

  struct thinkthen_optional_rule_v1 threshold;

  struct thinkthen_optional_string_v1 raw_pick;

  struct thinkthen_reported_usage_v1 usage;

  struct thinkthen_source_details_v1 question_sources;

  struct thinkthen_observation_identities_v1 observations;

  struct thinkthen_input_views_v1 inputs;
} thinkthen_details_v1;


typedef struct thinkthen_filter_view_v1 {

  struct thinkthen_row_v1 common;

  int value;
} thinkthen_filter_view_v1;


typedef struct thinkthen_find_view_v1 {

  struct thinkthen_row_v1 common;

  struct thinkthen_optional_content_v1 value;

  struct thinkthen_optional_size_v1 index;
} thinkthen_find_view_v1;


typedef union thinkthen_observed_probabilities_data_v1 {

  double yes;

  struct thinkthen_probabilities_v1 named;
} thinkthen_observed_probabilities_data_v1;


typedef struct thinkthen_observed_probabilities_v1 {

  uint32_t kind;

  union thinkthen_observed_probabilities_data_v1 data;
} thinkthen_observed_probabilities_v1;


typedef struct thinkthen_observation_success_v1 {

  struct thinkthen_string_v1 answer_id;

  struct thinkthen_string_v1 observation_id;

  struct thinkthen_member_value_v1 value;

  struct thinkthen_observed_probabilities_v1 probabilities;

  struct thinkthen_optional_double_v1 confidence;
} thinkthen_observation_success_v1;


typedef union thinkthen_question_observation_data_v1 {

  struct thinkthen_observation_success_v1 success;

  struct thinkthen_member_failure_v1 failure;
} thinkthen_question_observation_data_v1;


typedef struct thinkthen_question_observation_v1 {

  size_t index;

  struct thinkthen_optional_string_v1 member;

  struct thinkthen_optional_discriminator_v1 stage;

  size_t position;

  struct thinkthen_string_v1 question_sha256;

  struct thinkthen_string_v1 model;

  struct thinkthen_string_v1 url;

  struct thinkthen_strings_v1 requests;

  uint64_t requests_sent;

  int cached;

  size_t failed_questions;

  struct thinkthen_optional_usage_v1 usage;

  struct thinkthen_question_sources_v1 question_sources;

  uint32_t state;

  union thinkthen_question_observation_data_v1 data;
} thinkthen_question_observation_v1;


typedef struct thinkthen_tag_view_v1 {

  struct thinkthen_row_v1 common;

  struct thinkthen_strings_v1 value;
} thinkthen_tag_view_v1;


typedef struct thinkthen_score_view_v1 {

  struct thinkthen_row_v1 common;

  double value;
} thinkthen_score_view_v1;


typedef struct thinkthen_rank_view_v1 {

  struct thinkthen_row_v1 common;

  struct thinkthen_optional_size_v1 value;

  struct thinkthen_optional_string_v1 question_name;
} thinkthen_rank_view_v1;


typedef struct thinkthen_entity_v1 {

  struct thinkthen_string_v1 text;

  size_t start;

  size_t end;

  size_t length;

  struct thinkthen_string_v1 kind;

  double strength;
} thinkthen_entity_v1;


typedef struct thinkthen_entities_v1 {

  const struct thinkthen_entity_v1 *data;

  size_t len;
} thinkthen_entities_v1;


typedef struct thinkthen_entity_edge_v1 {

  struct thinkthen_string_v1 relation;

  struct thinkthen_entity_v1 source;

  struct thinkthen_entity_v1 target;

  double probability;

  int either;
} thinkthen_entity_edge_v1;


typedef struct thinkthen_entity_edges_v1 {

  const struct thinkthen_entity_edge_v1 *data;

  size_t len;
} thinkthen_entity_edges_v1;


typedef struct thinkthen_optional_entity_edges_v1 {

  int present;

  struct thinkthen_entity_edges_v1 value;
} thinkthen_optional_entity_edges_v1;


typedef struct thinkthen_recognize_value_v1 {

  struct thinkthen_entities_v1 entities;

  struct thinkthen_optional_entity_edges_v1 relations;
} thinkthen_recognize_value_v1;


typedef struct thinkthen_piece_v1 {

  size_t start;

  size_t end;

  struct thinkthen_probabilities_v1 tags;
} thinkthen_piece_v1;


typedef struct thinkthen_pieces_v1 {

  const struct thinkthen_piece_v1 *data;

  size_t len;
} thinkthen_pieces_v1;


typedef struct thinkthen_optional_probabilities_v1 {

  int present;

  struct thinkthen_probabilities_v1 value;
} thinkthen_optional_probabilities_v1;


typedef struct thinkthen_name_v1 {

  size_t start;

  size_t end;

  struct thinkthen_optional_probabilities_v1 kinds;

  struct thinkthen_optional_probabilities_v1 edges;
} thinkthen_name_v1;


typedef struct thinkthen_names_v1 {

  const struct thinkthen_name_v1 *data;

  size_t len;
} thinkthen_names_v1;


typedef struct thinkthen_place_v1 {

  size_t start;

  size_t end;
} thinkthen_place_v1;


typedef struct thinkthen_pair_v1 {

  struct thinkthen_string_v1 relation;

  struct thinkthen_place_v1 source;

  struct thinkthen_place_v1 target;

  double probability;
} thinkthen_pair_v1;


typedef struct thinkthen_pairs_v1 {

  const struct thinkthen_pair_v1 *data;

  size_t len;
} thinkthen_pairs_v1;


typedef struct thinkthen_recognize_answer_v1 {

  struct thinkthen_pieces_v1 pieces;

  struct thinkthen_names_v1 names;

  struct thinkthen_pairs_v1 pairs;
} thinkthen_recognize_answer_v1;


typedef struct thinkthen_recognize_view_v1 {

  struct thinkthen_row_v1 common;

  struct thinkthen_recognize_value_v1 value;

  struct thinkthen_recognize_answer_v1 answer;
} thinkthen_recognize_view_v1;


typedef struct thinkthen_endpoint_v1 {

  struct thinkthen_string_v1 name;

  struct thinkthen_string_v1 kind;
} thinkthen_endpoint_v1;


typedef struct thinkthen_edge_v1 {

  struct thinkthen_string_v1 relation;

  struct thinkthen_endpoint_v1 source;

  struct thinkthen_endpoint_v1 target;

  double probability;

  int either;
} thinkthen_edge_v1;


typedef struct thinkthen_edges_v1 {

  const struct thinkthen_edge_v1 *data;

  size_t len;
} thinkthen_edges_v1;


typedef struct thinkthen_optional_endpoint_v1 {

  int present;

  struct thinkthen_endpoint_v1 value;
} thinkthen_optional_endpoint_v1;


typedef struct thinkthen_relation_success_v1 {

  struct thinkthen_string_v1 answer_id;

  double probability;

  int accepted;
} thinkthen_relation_success_v1;


typedef union thinkthen_relation_answer_data_v1 {

  struct thinkthen_relation_success_v1 success;

  struct thinkthen_member_failure_v1 failure;
} thinkthen_relation_answer_data_v1;


typedef struct thinkthen_relation_answer_v1 {

  struct thinkthen_string_v1 relation;

  struct thinkthen_string_v1 reads;

  uint32_t method;

  uint32_t direction;

  struct thinkthen_endpoint_v1 source;

  struct thinkthen_optional_endpoint_v1 target;

  struct thinkthen_string_v1 request;

  uint32_t state;

  union thinkthen_relation_answer_data_v1 data;
} thinkthen_relation_answer_v1;


typedef struct thinkthen_relation_answers_v1 {

  const struct thinkthen_relation_answer_v1 *data;

  size_t len;
} thinkthen_relation_answers_v1;


typedef struct thinkthen_relate_view_v1 {

  struct thinkthen_row_v1 common;

  struct thinkthen_edges_v1 value;

  struct thinkthen_relation_answers_v1 questions;
} thinkthen_relate_view_v1;


typedef union thinkthen_row_observation_data_v1 {

  struct thinkthen_decide_view_v1 decide;

  struct thinkthen_choose_view_v1 choose;

  struct thinkthen_tag_view_v1 tag;

  struct thinkthen_score_view_v1 score;

  struct thinkthen_filter_view_v1 filter;

  struct thinkthen_rank_view_v1 rank;

  struct thinkthen_find_view_v1 find;

  struct thinkthen_annotate_view_v1 annotate;

  struct thinkthen_recognize_view_v1 recognize;

  struct thinkthen_relate_view_v1 relate;
} thinkthen_row_observation_data_v1;


typedef struct thinkthen_row_observation_v1 {

  size_t index;

  uint32_t function;

  union thinkthen_row_observation_data_v1 data;
} thinkthen_row_observation_v1;


typedef union thinkthen_observation_data_v1 {

  struct thinkthen_question_observation_v1 question;

  struct thinkthen_row_observation_v1 row;
} thinkthen_observation_data_v1;


typedef struct thinkthen_observation_v1 {

  uint32_t kind;

  union thinkthen_observation_data_v1 data;
} thinkthen_observation_v1;


typedef struct thinkthen_source_entity_v1 {

  struct thinkthen_entity_v1 entity;

  struct thinkthen_optional_location_v1 position;
} thinkthen_source_entity_v1;


typedef struct thinkthen_source_entities_v1 {

  const struct thinkthen_source_entity_v1 *data;

  size_t len;
} thinkthen_source_entities_v1;


typedef struct thinkthen_source_entity_edge_v1 {

  struct thinkthen_string_v1 relation;

  struct thinkthen_source_entity_v1 source;

  struct thinkthen_source_entity_v1 target;

  double probability;

  int either;
} thinkthen_source_entity_edge_v1;


typedef struct thinkthen_source_entity_edges_v1 {

  const struct thinkthen_source_entity_edge_v1 *data;

  size_t len;
} thinkthen_source_entity_edges_v1;


typedef struct thinkthen_optional_source_entity_edges_v1 {

  int present;

  struct thinkthen_source_entity_edges_v1 value;
} thinkthen_optional_source_entity_edges_v1;


typedef struct thinkthen_source_recognition_v1 {

  int present;

  struct thinkthen_source_entities_v1 entities;

  struct thinkthen_optional_source_entity_edges_v1 relations;
} thinkthen_source_recognition_v1;


typedef struct thinkthen_source_endpoint_v1 {

  size_t ordinal;

  struct thinkthen_endpoint_v1 endpoint;

  struct thinkthen_content_v1 record;

  struct thinkthen_optional_location_v1 position;
} thinkthen_source_endpoint_v1;


typedef struct thinkthen_source_edge_v1 {

  struct thinkthen_string_v1 relation;

  struct thinkthen_source_endpoint_v1 source;

  struct thinkthen_source_endpoint_v1 target;

  double probability;

  int either;
} thinkthen_source_edge_v1;


typedef struct thinkthen_source_edges_v1 {

  const struct thinkthen_source_edge_v1 *data;

  size_t len;
} thinkthen_source_edges_v1;


typedef struct thinkthen_source_relations_v1 {

  int present;

  struct thinkthen_source_edges_v1 edges;
} thinkthen_source_relations_v1;


typedef struct thinkthen_optional_meta_v1 {

  int present;

  struct thinkthen_meta_v1 value;
} thinkthen_optional_meta_v1;


typedef struct thinkthen_facts_v1 {

  struct thinkthen_string_v1 call_id;

  uint64_t cache_answers;

  struct thinkthen_optional_string_v1 estimated_cost_usd;

  struct thinkthen_optional_u64_v1 input_tokens;

  struct thinkthen_optional_string_v1 model;

  struct thinkthen_optional_u64_v1 output_tokens;

  uint64_t records;

  uint64_t requests_sent;

  double seconds;

  struct thinkthen_optional_u64_v1 command_ms;
} thinkthen_facts_v1;


typedef struct thinkthen_optional_facts_v1 {

  int present;

  struct thinkthen_facts_v1 value;
} thinkthen_optional_facts_v1;


typedef struct thinkthen_stopped_v1 {

  struct thinkthen_optional_size_v1 at;

  uint32_t cause;

  struct thinkthen_optional_u16_v1 status;

  int retryable;
} thinkthen_stopped_v1;


typedef struct thinkthen_optional_stopped_v1 {

  int present;

  struct thinkthen_stopped_v1 value;
} thinkthen_optional_stopped_v1;


typedef struct thinkthen_error_v1 {

  int code;

  struct thinkthen_string_v1 message;

  int retryable;

  struct thinkthen_optional_stopped_v1 stopped;
} thinkthen_error_v1;


typedef struct thinkthen_optional_error_v1 {

  int present;

  struct thinkthen_error_v1 value;
} thinkthen_optional_error_v1;


typedef struct thinkthen_summary_v1 {

  uint32_t state;

  struct thinkthen_string_v1 schema;

  struct thinkthen_optional_string_v1 answer_id;

  struct thinkthen_optional_discriminator_v1 function;

  size_t count;

  size_t observation_count;

  struct thinkthen_optional_meta_v1 meta;

  struct thinkthen_optional_facts_v1 facts;

  struct thinkthen_optional_attempts_v1 attempts;

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


typedef struct thinkthen_source_spec_v1 {

  struct thinkthen_strings_v1 paths;

  uint32_t unit;

  size_t window;
} thinkthen_source_spec_v1;


typedef struct thinkthen_images_v1 {

  const struct thinkthen_image *const *data;

  size_t len;
} thinkthen_images_v1;


typedef struct thinkthen_record_v1 {

  struct thinkthen_optional_content_v1 original;

  struct thinkthen_optional_content_v1 context;

  struct thinkthen_choices_v1 options;

  struct thinkthen_images_v1 images;
} thinkthen_record_v1;





int thinkthen_annotate_batch_start(const struct thinkthen_engine *e,
                                   const struct thinkthen_question *q,
                                   const struct thinkthen_source *s,
                                   const struct thinkthen_controls_v1 *c,
                                   struct thinkthen_batch **out);


int thinkthen_annotate_complete(const struct thinkthen_engine *engine,
                                const struct thinkthen_question *question,
                                const struct thinkthen_source *source,
                                const struct thinkthen_controls_v1 *controls,
                                struct thinkthen_result **out);


int thinkthen_batch_facts(const struct thinkthen_batch *owner, struct thinkthen_result **out);


void thinkthen_batch_free(struct thinkthen_batch *owner);


int thinkthen_batch_next(struct thinkthen_batch *owner, struct thinkthen_result **out);


char *thinkthen_call(const struct thinkthen_engine *engine, const char *request_json);


char *thinkthen_call_opts(const struct thinkthen_engine *engine,
                          const char *request_json,
                          int64_t deadline_ms,
                          thinkthen_cancel_token *cancel);


void thinkthen_cancel(thinkthen_cancel_token *token);


void thinkthen_cancel_token_free(thinkthen_cancel_token *token);


thinkthen_cancel_token *thinkthen_cancel_token_new(void);


int thinkthen_choose_batch_start(const struct thinkthen_engine *e,
                                 const struct thinkthen_question *q,
                                 const struct thinkthen_source *s,
                                 const struct thinkthen_controls_v1 *c,
                                 struct thinkthen_batch **out);


int thinkthen_choose_complete(const struct thinkthen_engine *engine,
                              const struct thinkthen_question *question,
                              const struct thinkthen_source *source,
                              const struct thinkthen_controls_v1 *controls,
                              struct thinkthen_result **out);


int thinkthen_decide(const struct thinkthen_engine *engine,
                     const char *question_json,
                     const char *text,
                     size_t text_len,
                     struct thinkthen_answer *out);


int thinkthen_decide_batch_start(const struct thinkthen_engine *e,
                                 const struct thinkthen_question *q,
                                 const struct thinkthen_source *s,
                                 const struct thinkthen_controls_v1 *c,
                                 struct thinkthen_batch **out);


int thinkthen_decide_complete(const struct thinkthen_engine *engine,
                              const struct thinkthen_question *question,
                              const struct thinkthen_source *source,
                              const struct thinkthen_controls_v1 *controls,
                              struct thinkthen_result **out);


int thinkthen_decide_many(const struct thinkthen_engine *engine,
                          const char *question_json,
                          const char *const *texts,
                          const size_t *lengths,
                          size_t count,
                          struct thinkthen_answer *out);


int thinkthen_decide_many_opts(const struct thinkthen_engine *engine,
                               const char *question_json,
                               const char *const *texts,
                               const size_t *lengths,
                               size_t count,
                               int64_t deadline_ms,
                               thinkthen_cancel_token *cancel,
                               struct thinkthen_answer *out);


int thinkthen_decide_many_with_facts(const struct thinkthen_engine *engine,
                                     const char *question_json,
                                     const char *const *texts,
                                     const size_t *lengths,
                                     size_t count,
                                     struct thinkthen_answer *out,
                                     char **facts_json,
                                     size_t *facts_len);


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


int thinkthen_decide_opts(const struct thinkthen_engine *engine,
                          const char *question_json,
                          const char *text,
                          size_t text_len,
                          int64_t deadline_ms,
                          thinkthen_cancel_token *cancel,
                          struct thinkthen_answer *out);


int thinkthen_decide_with_facts(const struct thinkthen_engine *engine,
                                const char *question_json,
                                const char *evidence,
                                size_t evidence_len,
                                struct thinkthen_answer *out,
                                char **facts_json,
                                size_t *facts_len);


int thinkthen_decide_with_facts_opts(const struct thinkthen_engine *engine,
                                     const char *question_json,
                                     const char *evidence,
                                     size_t evidence_len,
                                     int64_t deadline_ms,
                                     thinkthen_cancel_token *cancel,
                                     struct thinkthen_answer *out,
                                     char **facts_json,
                                     size_t *facts_len);


int thinkthen_engine_finish_usage_status_v1(const struct thinkthen_engine *engine,
                                            struct thinkthen_complete_usage_persistence_v1 *out_state,
                                            struct thinkthen_complete_utf8_v1 *out_advice);


void thinkthen_engine_free(struct thinkthen_engine *engine);


struct thinkthen_engine *thinkthen_engine_new(void);


struct thinkthen_engine *thinkthen_engine_new_with(const char *settings_json);


int thinkthen_engine_usage_persistence_v1(const struct thinkthen_engine *engine,
                                          struct thinkthen_complete_usage_persistence_v1 *out_state,
                                          struct thinkthen_complete_utf8_v1 *out_advice);


int thinkthen_error_code(const struct thinkthen_engine *engine);


int thinkthen_error_complete(const struct thinkthen_engine *engine, struct thinkthen_result **out);


const char *thinkthen_error_facts_json(const struct thinkthen_engine *engine);


const char *thinkthen_error_message(const struct thinkthen_engine *engine);


int thinkthen_error_retryable(const struct thinkthen_engine *engine);


int thinkthen_filter_batch_start(const struct thinkthen_engine *e,
                                 const struct thinkthen_question *q,
                                 const struct thinkthen_source *s,
                                 const struct thinkthen_controls_v1 *c,
                                 struct thinkthen_batch **out);


int thinkthen_filter_complete(const struct thinkthen_engine *engine,
                              const struct thinkthen_question *question,
                              const struct thinkthen_source *source,
                              const struct thinkthen_controls_v1 *controls,
                              struct thinkthen_result **out);


int thinkthen_find_complete(const struct thinkthen_engine *engine,
                            const struct thinkthen_question *question,
                            const struct thinkthen_source *source,
                            const struct thinkthen_controls_v1 *controls,
                            struct thinkthen_result **out);


void thinkthen_free_string(char *text);


int thinkthen_image_clone(const struct thinkthen_engine *engine,
                          const uint8_t *bytes,
                          size_t len,
                          uint32_t media,
                          struct thinkthen_optional_string_v1 filename,
                          struct thinkthen_image **out);


void thinkthen_image_free(struct thinkthen_image *owner);


int thinkthen_image_view(const struct thinkthen_image *owner, struct thinkthen_image_view_v1 *out);


int thinkthen_plan_json(const struct thinkthen_engine *engine,
                        const char *plan_json,
                        char **out,
                        size_t *out_len);


int thinkthen_question_author(const struct thinkthen_question *owner,
                              struct thinkthen_question_author_v1 *out);


int thinkthen_question_file(const struct thinkthen_engine *engine,
                            const char *path,
                            char **out,
                            size_t *out_len);


void thinkthen_question_free(struct thinkthen_question *owner);


int thinkthen_question_load(const struct thinkthen_engine *engine,
                            struct thinkthen_string_v1 path,
                            struct thinkthen_question **out);


int thinkthen_question_load_named(const struct thinkthen_engine *engine,
                                  uint32_t role,
                                  struct thinkthen_string_v1 name,
                                  struct thinkthen_question **out);


int thinkthen_question_load_reference(const struct thinkthen_engine *engine,
                                      uint32_t role,
                                      struct thinkthen_string_v1 reference,
                                      struct thinkthen_question **out);


int thinkthen_question_new(const struct thinkthen_engine *engine,
                           const struct thinkthen_question_spec_v1 *spec,
                           struct thinkthen_question **out);


int thinkthen_question_new_authored(const struct thinkthen_engine *engine,
                                    const struct thinkthen_question_spec_v1 *spec,
                                    const struct thinkthen_question_author_v1 *author,
                                    struct thinkthen_question **out);


int thinkthen_question_new_recognition_v1(const struct thinkthen_engine *engine,
                                          const struct thinkthen_question_spec_v1 *spec,
                                          const struct thinkthen_question_author_v1 *metadata,
                                          const struct thinkthen_recognition_task_v1 *task,
                                          struct thinkthen_question **out);


int thinkthen_question_parse(const struct thinkthen_engine *engine,
                             uint32_t role,
                             struct thinkthen_string_v1 json,
                             struct thinkthen_question **out);


int thinkthen_question_recognition_task_v1(const struct thinkthen_question *owner,
                                           struct thinkthen_recognition_task_v1 *out);


int thinkthen_rank_complete(const struct thinkthen_engine *engine,
                            const struct thinkthen_question *question,
                            const struct thinkthen_source *source,
                            const struct thinkthen_controls_v1 *controls,
                            struct thinkthen_result **out);


int thinkthen_recognize(const struct thinkthen_engine *engine,
                        const char *spec_json,
                        const char *text,
                        size_t text_len,
                        char **out,
                        size_t *out_len);


int thinkthen_recognize_complete(const struct thinkthen_engine *engine,
                                 const struct thinkthen_question *question,
                                 const struct thinkthen_source *source,
                                 const struct thinkthen_controls_v1 *controls,
                                 struct thinkthen_result **out);


int thinkthen_recognize_opts(const struct thinkthen_engine *engine,
                             const char *spec_json,
                             const char *text,
                             size_t text_len,
                             int64_t deadline_ms,
                             thinkthen_cancel_token *cancel,
                             char **out,
                             size_t *out_len);


int thinkthen_recognize_with_facts(const struct thinkthen_engine *engine,
                                   const char *spec_json,
                                   const char *evidence,
                                   size_t evidence_len,
                                   char **out,
                                   size_t *out_len,
                                   char **facts_json,
                                   size_t *facts_len);


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


int thinkthen_relate(const struct thinkthen_engine *engine,
                     const char *spec_json,
                     const char *const *texts,
                     const size_t *lengths,
                     size_t count,
                     char **out,
                     size_t *out_len);


int thinkthen_relate_complete(const struct thinkthen_engine *engine,
                              const struct thinkthen_question *question,
                              const struct thinkthen_source *source,
                              const struct thinkthen_controls_v1 *controls,
                              struct thinkthen_result **out);


int thinkthen_relate_opts(const struct thinkthen_engine *engine,
                          const char *spec_json,
                          const char *const *texts,
                          const size_t *lengths,
                          size_t count,
                          int64_t deadline_ms,
                          thinkthen_cancel_token *cancel,
                          char **out,
                          size_t *out_len);


int thinkthen_relate_with_facts(const struct thinkthen_engine *engine,
                                const char *spec_json,
                                const char *const *texts,
                                const size_t *lengths,
                                size_t count,
                                char **out,
                                size_t *out_len,
                                char **facts_json,
                                size_t *facts_len);


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


int thinkthen_request_plan_json(const struct thinkthen_engine *engine,
                                const char *request_json,
                                size_t request_len,
                                char **out,
                                size_t *out_len);


int thinkthen_result_annotate(const struct thinkthen_result *owner,
                              size_t at,
                              struct thinkthen_annotate_view_v1 *out);


int thinkthen_result_choose(const struct thinkthen_result *owner,
                            size_t at,
                            struct thinkthen_choose_view_v1 *out);


int thinkthen_result_decide(const struct thinkthen_result *owner,
                            size_t at,
                            struct thinkthen_decide_view_v1 *out);


int thinkthen_result_details(const struct thinkthen_result *owner,
                             size_t at,
                             struct thinkthen_details_v1 *out);


int thinkthen_result_filter(const struct thinkthen_result *owner,
                            size_t at,
                            struct thinkthen_filter_view_v1 *out);


int thinkthen_result_find(const struct thinkthen_result *owner,
                          size_t at,
                          struct thinkthen_find_view_v1 *out);


void thinkthen_result_free(struct thinkthen_result *owner);


int thinkthen_result_member_author(const struct thinkthen_result *owner,
                                   size_t at,
                                   size_t member,
                                   struct thinkthen_question_author_v1 *out);


int thinkthen_result_observation(const struct thinkthen_result *owner,
                                 size_t at,
                                 struct thinkthen_observation_v1 *out);


int thinkthen_result_observation_author(const struct thinkthen_result *owner,
                                        size_t at,
                                        struct thinkthen_question_author_v1 *out);


int thinkthen_result_observation_details(const struct thinkthen_result *owner,
                                         size_t at,
                                         struct thinkthen_details_v1 *out);


int thinkthen_result_question_author(const struct thinkthen_result *owner,
                                     size_t at,
                                     struct thinkthen_question_author_v1 *out);


int thinkthen_result_rank(const struct thinkthen_result *owner,
                          size_t at,
                          struct thinkthen_rank_view_v1 *out);


int thinkthen_result_rank_member(const struct thinkthen_result *owner,
                                 size_t row,
                                 size_t member,
                                 struct thinkthen_rank_view_v1 *out);


int thinkthen_result_rank_member_count(const struct thinkthen_result *owner,
                                       size_t row,
                                       size_t *out);


int thinkthen_result_rank_member_details(const struct thinkthen_result *owner,
                                         size_t row,
                                         size_t member,
                                         struct thinkthen_details_v1 *out);


int thinkthen_result_recognition_task_v1(const struct thinkthen_result *owner,
                                         size_t at,
                                         struct thinkthen_recognition_task_v1 *out);


int thinkthen_result_recognize(const struct thinkthen_result *owner,
                               size_t at,
                               struct thinkthen_recognize_view_v1 *out);


int thinkthen_result_relate(const struct thinkthen_result *owner,
                            size_t at,
                            struct thinkthen_relate_view_v1 *out);


int thinkthen_result_row(const struct thinkthen_result *owner,
                         size_t at,
                         struct thinkthen_row_observation_v1 *out);


int thinkthen_result_score(const struct thinkthen_result *owner,
                           size_t at,
                           struct thinkthen_score_view_v1 *out);


int thinkthen_result_source_recognition(const struct thinkthen_result *owner,
                                        size_t row,
                                        struct thinkthen_source_recognition_v1 *out);


int thinkthen_result_source_relations(const struct thinkthen_result *owner,
                                      size_t row,
                                      struct thinkthen_source_relations_v1 *out);


int thinkthen_result_summary(const struct thinkthen_result *owner,
                             struct thinkthen_summary_v1 *out);


int thinkthen_result_tag(const struct thinkthen_result *owner,
                         size_t at,
                         struct thinkthen_tag_view_v1 *out);


int thinkthen_score_batch_start(const struct thinkthen_engine *e,
                                const struct thinkthen_question *q,
                                const struct thinkthen_source *s,
                                const struct thinkthen_controls_v1 *c,
                                struct thinkthen_batch **out);


int thinkthen_score_complete(const struct thinkthen_engine *engine,
                             const struct thinkthen_question *question,
                             const struct thinkthen_source *source,
                             const struct thinkthen_controls_v1 *controls,
                             struct thinkthen_result **out);


void thinkthen_session_cancel(struct thinkthen_session *session);


const char *thinkthen_session_error_message(void);


int thinkthen_session_finish(struct thinkthen_session *session,
                             const char *failure_json,
                             size_t failure_len);


void thinkthen_session_free(struct thinkthen_session *session);


int thinkthen_session_new(const struct thinkthen_engine *engine,
                          const char *request_json,
                          size_t request_len,
                          struct thinkthen_session **out);


int thinkthen_session_new_with_surface(const struct thinkthen_engine *engine,
                                       const char *request_json,
                                       size_t request_len,
                                       const char *surface,
                                       size_t surface_len,
                                       struct thinkthen_session **out);


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


int thinkthen_source_files(const struct thinkthen_engine *engine,
                           const struct thinkthen_source_spec_v1 *spec,
                           struct thinkthen_source **out);


void thinkthen_source_free(struct thinkthen_source *owner);


int thinkthen_source_image_files(const struct thinkthen_engine *engine,
                                 const struct thinkthen_source_spec_v1 *spec,
                                 struct thinkthen_source **out);


int thinkthen_source_records(const struct thinkthen_engine *engine,
                             const struct thinkthen_record_v1 *records,
                             size_t count,
                             struct thinkthen_source **out);


int thinkthen_tag_batch_start(const struct thinkthen_engine *e,
                              const struct thinkthen_question *q,
                              const struct thinkthen_source *s,
                              const struct thinkthen_controls_v1 *c,
                              struct thinkthen_batch **out);


int thinkthen_tag_complete(const struct thinkthen_engine *engine,
                           const struct thinkthen_question *question,
                           const struct thinkthen_source *source,
                           const struct thinkthen_controls_v1 *controls,
                           struct thinkthen_result **out);
