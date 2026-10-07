


































typedef struct thinkthen_engine thinkthen_engine;


typedef struct thinkthen_cancel_token thinkthen_cancel_token;


typedef struct thinkthen_answer {
    int outcome;
    double probability;
} thinkthen_answer;


thinkthen_engine *thinkthen_engine_new(void);


thinkthen_engine *thinkthen_engine_new_with(const char *settings_json);


void thinkthen_engine_free(thinkthen_engine *engine);


const char *thinkthen_error_message(const thinkthen_engine *engine);


int thinkthen_error_retryable(const thinkthen_engine *engine);


int thinkthen_error_code(const thinkthen_engine *engine);


thinkthen_cancel_token *thinkthen_cancel_token_new(void);


void thinkthen_cancel(thinkthen_cancel_token *token);


void thinkthen_cancel_token_free(thinkthen_cancel_token *token);




int thinkthen_question_file(const thinkthen_engine *engine, const char *path,
                            char **out, size_t *out_len);


int thinkthen_plan_json(const thinkthen_engine *engine, const char *plan_json,
                        char **out, size_t *out_len);


int thinkthen_decide(const thinkthen_engine *engine, const char *question_json,
                     const char *text, size_t text_len,
                     thinkthen_answer *out);


int thinkthen_decide_opts(const thinkthen_engine *engine, const char *question_json,
                          const char *text, size_t text_len,
                          int64_t deadline_ms, thinkthen_cancel_token *cancel,
                          thinkthen_answer *out);


int thinkthen_decide_many(const thinkthen_engine *engine, const char *question_json,
                          const char *const *texts, const size_t *lengths,
                          size_t count, thinkthen_answer *out);


int thinkthen_decide_many_opts(const thinkthen_engine *engine, const char *question_json,
                               const char *const *texts, const size_t *lengths,
                               size_t count, int64_t deadline_ms,
                               thinkthen_cancel_token *cancel, thinkthen_answer *out);


char *thinkthen_call(const thinkthen_engine *engine, const char *request_json);


char *thinkthen_call_opts(const thinkthen_engine *engine, const char *request_json,
                          int64_t deadline_ms, thinkthen_cancel_token *cancel);


const char *thinkthen_error_facts_json(const thinkthen_engine *engine);


int thinkthen_recognize(const thinkthen_engine *engine, const char *spec_json,
                        const char *text, size_t text_len, char **out,
                        size_t *out_len);


int thinkthen_recognize_opts(const thinkthen_engine *engine, const char *spec_json,
                             const char *text, size_t text_len,
                             int64_t deadline_ms, thinkthen_cancel_token *cancel,
                             char **out, size_t *out_len);


int thinkthen_relate(const thinkthen_engine *engine, const char *spec_json,
                     const char *const *texts, const size_t *lengths,
                     size_t count, char **out, size_t *out_len);


int thinkthen_relate_opts(const thinkthen_engine *engine, const char *spec_json,
                          const char *const *texts, const size_t *lengths,
                          size_t count, int64_t deadline_ms,
                          thinkthen_cancel_token *cancel, char **out,
                          size_t *out_len);


int thinkthen_decide_with_facts(const thinkthen_engine *engine,
    const char *question_json, const char *text, size_t text_len,
    thinkthen_answer *out, char **facts_json, size_t *facts_len);
int thinkthen_decide_with_facts_opts(const thinkthen_engine *engine,
    const char *question_json, const char *text, size_t text_len,
    int64_t deadline_ms, thinkthen_cancel_token *cancel,
    thinkthen_answer *out, char **facts_json, size_t *facts_len);
int thinkthen_decide_many_with_facts(const thinkthen_engine *engine,
    const char *question_json, const char *const *texts, const size_t *lengths,
    size_t count, thinkthen_answer *out, char **facts_json, size_t *facts_len);
int thinkthen_decide_many_with_facts_opts(const thinkthen_engine *engine,
    const char *question_json, const char *const *texts, const size_t *lengths,
    size_t count, int64_t deadline_ms, thinkthen_cancel_token *cancel,
    thinkthen_answer *out, char **facts_json, size_t *facts_len);
int thinkthen_recognize_with_facts(const thinkthen_engine *engine,
    const char *spec_json, const char *text, size_t text_len,
    char **out, size_t *out_len, char **facts_json, size_t *facts_len);
int thinkthen_recognize_with_facts_opts(const thinkthen_engine *engine,
    const char *spec_json, const char *text, size_t text_len,
    int64_t deadline_ms, thinkthen_cancel_token *cancel,
    char **out, size_t *out_len, char **facts_json, size_t *facts_len);
int thinkthen_relate_with_facts(const thinkthen_engine *engine,
    const char *spec_json, const char *const *texts, const size_t *lengths,
    size_t count, char **out, size_t *out_len,
    char **facts_json, size_t *facts_len);
int thinkthen_relate_with_facts_opts(const thinkthen_engine *engine,
    const char *spec_json, const char *const *texts, const size_t *lengths,
    size_t count, int64_t deadline_ms, thinkthen_cancel_token *cancel,
    char **out, size_t *out_len, char **facts_json, size_t *facts_len);


void thinkthen_free_string(char *text);

typedef struct thinkthen_question thinkthen_question;
typedef struct thinkthen_source thinkthen_source;
typedef struct thinkthen_image thinkthen_image;
typedef struct thinkthen_result thinkthen_result;
typedef struct thinkthen_batch thinkthen_batch;

typedef struct thinkthen_string_v1 { const char *data; size_t len; } thinkthen_string_v1;
typedef struct thinkthen_strings_v1 { const thinkthen_string_v1 *data; size_t len; } thinkthen_strings_v1;
typedef struct thinkthen_optional_string_v1 { int present; thinkthen_string_v1 value; } thinkthen_optional_string_v1;
typedef struct thinkthen_optional_size_v1 { int present; size_t value; } thinkthen_optional_size_v1;
typedef struct thinkthen_optional_u64_v1 { int present; uint64_t value; } thinkthen_optional_u64_v1;
typedef struct thinkthen_optional_u16_v1 { int present; uint16_t value; } thinkthen_optional_u16_v1;
typedef struct thinkthen_optional_double_v1 { int present; double value; } thinkthen_optional_double_v1;
typedef struct thinkthen_optional_discriminator_v1 { int present; uint32_t value; } thinkthen_optional_discriminator_v1;



typedef struct thinkthen_content_v1 { uint32_t kind; thinkthen_string_v1 data; } thinkthen_content_v1;
typedef struct thinkthen_optional_content_v1 { int present; thinkthen_content_v1 value; } thinkthen_optional_content_v1;






typedef struct thinkthen_rule_v1 { uint32_t kind; double low; double high; } thinkthen_rule_v1;
typedef struct thinkthen_optional_rule_v1 { int present; thinkthen_rule_v1 value; } thinkthen_optional_rule_v1;












typedef struct thinkthen_choice_v1 {
    thinkthen_string_v1 name;
    thinkthen_optional_content_v1 description;
    thinkthen_optional_double_v1 weight;
} thinkthen_choice_v1;
typedef struct thinkthen_choices_v1 { const thinkthen_choice_v1 *data; size_t len; } thinkthen_choices_v1;
typedef struct thinkthen_relation_v1 {
    thinkthen_string_v1 name;
    thinkthen_string_v1 source;
    thinkthen_string_v1 target;
    thinkthen_optional_string_v1 reads;
    int either;
    int single;
} thinkthen_relation_v1;
typedef struct thinkthen_relations_v1 { const thinkthen_relation_v1 *data; size_t len; } thinkthen_relations_v1;
typedef struct thinkthen_member_spec_v1 {
    thinkthen_string_v1 name;
    const thinkthen_question *question;
} thinkthen_member_spec_v1;
typedef struct thinkthen_member_specs_v1 { const thinkthen_member_spec_v1 *data; size_t len; } thinkthen_member_specs_v1;
typedef struct thinkthen_question_spec_v1 {
    uint32_t kind;
    thinkthen_content_v1 text;
    thinkthen_optional_content_v1 yes;
    thinkthen_optional_content_v1 no;
    thinkthen_choices_v1 choices;
    thinkthen_rule_v1 threshold;
    thinkthen_rule_v1 relation_threshold;
    thinkthen_optional_string_v1 model;
    thinkthen_optional_string_v1 profile;
    thinkthen_optional_size_v1 batch;
    int batch_max;
    int none;
    thinkthen_strings_v1 on;
    thinkthen_member_specs_v1 members;
    thinkthen_choices_v1 kinds;
    thinkthen_relations_v1 relations;
    thinkthen_optional_string_v1 name_pointer;
    thinkthen_optional_string_v1 kind_pointer;
} thinkthen_question_spec_v1;


typedef struct thinkthen_question_view_v1 thinkthen_question_view_v1;
typedef struct thinkthen_question_member_v1 {
    thinkthen_string_v1 name;
    const thinkthen_question_view_v1 *question;
} thinkthen_question_member_v1;
typedef struct thinkthen_question_members_v1 { const thinkthen_question_member_v1 *data; size_t len; } thinkthen_question_members_v1;
struct thinkthen_question_view_v1 {
    uint32_t kind;
    thinkthen_content_v1 text;
    thinkthen_optional_content_v1 yes;
    thinkthen_optional_content_v1 no;
    thinkthen_choices_v1 choices;
    thinkthen_rule_v1 threshold;
    thinkthen_rule_v1 relation_threshold;
    thinkthen_optional_string_v1 model;
    thinkthen_optional_string_v1 profile;
    thinkthen_optional_size_v1 batch;
    int batch_max;
    int none;
    thinkthen_strings_v1 on;
    thinkthen_question_members_v1 members;
    thinkthen_choices_v1 kinds;
    thinkthen_relations_v1 relations;
    thinkthen_optional_string_v1 name_pointer;
    thinkthen_optional_string_v1 kind_pointer;
};
typedef struct thinkthen_optional_question_v1 { int present; thinkthen_question_view_v1 value; } thinkthen_optional_question_v1;

typedef struct thinkthen_images_v1 { const thinkthen_image *const *data; size_t len; } thinkthen_images_v1;


typedef struct thinkthen_image_view_v1 {
    uint32_t media;
    const uint8_t *bytes;
    size_t bytes_len;
    uint32_t width;
    uint32_t height;
    thinkthen_optional_string_v1 filename;
} thinkthen_image_view_v1;
typedef struct thinkthen_image_views_v1 { const thinkthen_image_view_v1 *data; size_t len; } thinkthen_image_views_v1;
typedef struct thinkthen_optional_image_views_v1 { int present; thinkthen_image_views_v1 value; } thinkthen_optional_image_views_v1;
typedef struct thinkthen_record_v1 {
    thinkthen_optional_content_v1 original;
    thinkthen_optional_content_v1 context;
    thinkthen_choices_v1 options;
    thinkthen_images_v1 images;
} thinkthen_record_v1;





typedef struct thinkthen_source_spec_v1 {
    thinkthen_strings_v1 paths;
    uint32_t unit;
    size_t window;
} thinkthen_source_spec_v1;
typedef struct thinkthen_controls_v1 {
    int64_t deadline_ms;
    thinkthen_cancel_token *cancel;
    thinkthen_optional_content_v1 context;
    thinkthen_optional_size_v1 batch;
    int batch_max;
    int attempts;
    thinkthen_string_v1 surface;
} thinkthen_controls_v1;





typedef struct thinkthen_decide_value_v1 {
    uint32_t kind;
    union {
        int boolean;
        thinkthen_content_v1 authored;
    } data;
} thinkthen_decide_value_v1;
typedef struct thinkthen_probability_v1 { thinkthen_string_v1 name; double probability; } thinkthen_probability_v1;
typedef struct thinkthen_probabilities_v1 { const thinkthen_probability_v1 *data; size_t len; } thinkthen_probabilities_v1;
typedef struct thinkthen_optional_probabilities_v1 { int present; thinkthen_probabilities_v1 value; } thinkthen_optional_probabilities_v1;





typedef struct thinkthen_named_answer_v1 {
    thinkthen_string_v1 pick;
    thinkthen_probabilities_v1 probabilities;
    thinkthen_optional_double_v1 confidence;
} thinkthen_named_answer_v1;
typedef struct thinkthen_score_answer_v1 {
    thinkthen_string_v1 level;
    thinkthen_probabilities_v1 probabilities;
    thinkthen_optional_double_v1 confidence;
} thinkthen_score_answer_v1;
typedef struct thinkthen_answer_v1 {
    uint32_t kind;
    union {
        double probability;
        thinkthen_named_answer_v1 choice;
        thinkthen_probabilities_v1 tag;
        thinkthen_score_answer_v1 score;
        thinkthen_named_answer_v1 find;
    } data;
} thinkthen_answer_v1;
typedef struct thinkthen_optional_answer_v1 { int present; thinkthen_answer_v1 value; } thinkthen_optional_answer_v1;
typedef struct thinkthen_location_v1 {
    thinkthen_optional_string_v1 file;
    thinkthen_optional_size_v1 first_line;
    thinkthen_optional_size_v1 last_line;
} thinkthen_location_v1;
typedef struct thinkthen_optional_location_v1 { int present; thinkthen_location_v1 value; } thinkthen_optional_location_v1;


typedef struct thinkthen_member_value_v1 {
    uint32_t kind;
    union {
        thinkthen_decide_value_v1 decide;
        thinkthen_optional_string_v1 choose;
        thinkthen_strings_v1 tag;
        double score;
    } data;
} thinkthen_member_value_v1;








typedef struct thinkthen_member_failure_v1 {
    thinkthen_string_v1 failure_id;
    uint32_t cause;
} thinkthen_member_failure_v1;
typedef struct thinkthen_member_success_v1 {
    thinkthen_string_v1 answer_id;
    thinkthen_member_value_v1 value;
    thinkthen_answer_v1 answer;
    thinkthen_rule_v1 threshold;
} thinkthen_member_success_v1;
typedef struct thinkthen_member_v1 {
    thinkthen_string_v1 name;
    thinkthen_string_v1 request;
    thinkthen_question_view_v1 question;
    uint32_t state;
    union {
        thinkthen_member_success_v1 success;
        thinkthen_member_failure_v1 failure;
    } data;
} thinkthen_member_v1;
typedef struct thinkthen_members_v1 { const thinkthen_member_v1 *data; size_t len; } thinkthen_members_v1;

typedef struct thinkthen_entity_v1 {
    thinkthen_string_v1 text;
    size_t start;
    size_t end;
    size_t length;
    thinkthen_string_v1 kind;
    double strength;
} thinkthen_entity_v1;
typedef struct thinkthen_entities_v1 { const thinkthen_entity_v1 *data; size_t len; } thinkthen_entities_v1;
typedef struct thinkthen_entity_edge_v1 {
    thinkthen_string_v1 relation;
    thinkthen_entity_v1 source;
    thinkthen_entity_v1 target;
    double probability;
    int either;
} thinkthen_entity_edge_v1;
typedef struct thinkthen_entity_edges_v1 { const thinkthen_entity_edge_v1 *data; size_t len; } thinkthen_entity_edges_v1;
typedef struct thinkthen_optional_entity_edges_v1 { int present; thinkthen_entity_edges_v1 value; } thinkthen_optional_entity_edges_v1;
typedef struct thinkthen_place_v1 { size_t start; size_t end; } thinkthen_place_v1;
typedef struct thinkthen_piece_v1 {
    size_t start;
    size_t end;
    thinkthen_probabilities_v1 tags;
} thinkthen_piece_v1;
typedef struct thinkthen_pieces_v1 { const thinkthen_piece_v1 *data; size_t len; } thinkthen_pieces_v1;
typedef struct thinkthen_name_v1 {
    size_t start;
    size_t end;
    thinkthen_optional_probabilities_v1 kinds;
    thinkthen_optional_probabilities_v1 edges;
} thinkthen_name_v1;
typedef struct thinkthen_names_v1 { const thinkthen_name_v1 *data; size_t len; } thinkthen_names_v1;
typedef struct thinkthen_pair_v1 {
    thinkthen_string_v1 relation;
    thinkthen_place_v1 source;
    thinkthen_place_v1 target;
    double probability;
} thinkthen_pair_v1;
typedef struct thinkthen_pairs_v1 { const thinkthen_pair_v1 *data; size_t len; } thinkthen_pairs_v1;
typedef struct thinkthen_recognize_value_v1 {
    thinkthen_entities_v1 entities;
    thinkthen_optional_entity_edges_v1 relations;
} thinkthen_recognize_value_v1;
typedef struct thinkthen_recognize_answer_v1 {
    thinkthen_pieces_v1 pieces;
    thinkthen_names_v1 names;
    thinkthen_pairs_v1 pairs;
} thinkthen_recognize_answer_v1;

typedef struct thinkthen_endpoint_v1 { thinkthen_string_v1 name; thinkthen_string_v1 kind; } thinkthen_endpoint_v1;
typedef struct thinkthen_optional_endpoint_v1 { int present; thinkthen_endpoint_v1 value; } thinkthen_optional_endpoint_v1;
typedef struct thinkthen_edge_v1 {
    thinkthen_string_v1 relation;
    thinkthen_endpoint_v1 source;
    thinkthen_endpoint_v1 target;
    double probability;
    int either;
} thinkthen_edge_v1;
typedef struct thinkthen_edges_v1 { const thinkthen_edge_v1 *data; size_t len; } thinkthen_edges_v1;




typedef struct thinkthen_relation_success_v1 {
    thinkthen_string_v1 answer_id;
    double probability;
    int accepted;
} thinkthen_relation_success_v1;
typedef struct thinkthen_relation_answer_v1 {
    thinkthen_string_v1 relation;
    thinkthen_string_v1 reads;
    uint32_t method;
    uint32_t direction;
    thinkthen_endpoint_v1 source;
    thinkthen_optional_endpoint_v1 target;
    thinkthen_string_v1 request;
    uint32_t state;
    union {
        thinkthen_relation_success_v1 success;
        thinkthen_member_failure_v1 failure;
    } data;
} thinkthen_relation_answer_v1;
typedef struct thinkthen_relation_answers_v1 { const thinkthen_relation_answer_v1 *data; size_t len; } thinkthen_relation_answers_v1;

typedef struct thinkthen_usage_v1 { uint64_t input_tokens; uint64_t output_tokens; } thinkthen_usage_v1;
typedef struct thinkthen_optional_usage_v1 { int present; thinkthen_usage_v1 value; } thinkthen_optional_usage_v1;





typedef struct thinkthen_question_source_v1 { uint32_t origin; thinkthen_string_v1 answered_by; } thinkthen_question_source_v1;
typedef struct thinkthen_question_sources_v1 { const thinkthen_question_source_v1 *data; size_t len; } thinkthen_question_sources_v1;


typedef struct thinkthen_observation_identity_v1 {
    uint32_t kind;
    union {
        thinkthen_string_v1 observation_id;
        thinkthen_string_v1 failure_id;
    } data;
} thinkthen_observation_identity_v1;
typedef struct thinkthen_observation_identities_v1 { const thinkthen_observation_identity_v1 *data; size_t len; } thinkthen_observation_identities_v1;
typedef struct thinkthen_profile_warning_v1 { thinkthen_string_v1 tuned_for; thinkthen_string_v1 running; } thinkthen_profile_warning_v1;
typedef struct thinkthen_optional_profile_warning_v1 { int present; thinkthen_profile_warning_v1 value; } thinkthen_optional_profile_warning_v1;


typedef struct thinkthen_batch_v1 { uint32_t kind; size_t records; } thinkthen_batch_v1;
typedef struct thinkthen_optional_batch_v1 { int present; thinkthen_batch_v1 value; } thinkthen_optional_batch_v1;
typedef struct thinkthen_batch_warning_v1 { thinkthen_batch_v1 tuned_for; thinkthen_batch_v1 running; } thinkthen_batch_warning_v1;
typedef struct thinkthen_optional_batch_warning_v1 { int present; thinkthen_batch_warning_v1 value; } thinkthen_optional_batch_warning_v1;



typedef struct thinkthen_attempt_v1 {
    uint64_t ordinal;
    thinkthen_string_v1 request_sha256;
    uint64_t wall_ms;
    uint32_t outcome;
    thinkthen_string_v1 sdk_request_id;
    thinkthen_optional_u16_v1 status;
    thinkthen_optional_u64_v1 server_ms;
    thinkthen_optional_string_v1 request_id;
} thinkthen_attempt_v1;
typedef struct thinkthen_attempts_v1 { const thinkthen_attempt_v1 *data; size_t len; } thinkthen_attempts_v1;
typedef struct thinkthen_optional_attempts_v1 { int present; thinkthen_attempts_v1 value; } thinkthen_optional_attempts_v1;
typedef struct thinkthen_meta_v1 {
    thinkthen_string_v1 tool;
    thinkthen_optional_string_v1 question_sha256;
    thinkthen_optional_string_v1 questions_sha256;
    thinkthen_string_v1 url;
    thinkthen_string_v1 model;
    thinkthen_optional_usage_v1 usage;
    uint64_t requests_sent;
    int cached;
    thinkthen_strings_v1 requests;
    size_t failed_questions;
    thinkthen_optional_profile_warning_v1 profile_warning;
    thinkthen_optional_batch_v1 batch_setting;
    thinkthen_optional_batch_warning_v1 batch_warning;
    thinkthen_optional_string_v1 context_sha256;
    thinkthen_optional_attempts_v1 attempts;
    thinkthen_optional_discriminator_v1 origin;
    thinkthen_question_sources_v1 question_sources;
    thinkthen_observation_identities_v1 observations;
    thinkthen_optional_string_v1 answered_by;
} thinkthen_meta_v1;
typedef struct thinkthen_optional_meta_v1 { int present; thinkthen_meta_v1 value; } thinkthen_optional_meta_v1;


typedef struct thinkthen_facts_v1 {
    thinkthen_string_v1 call_id;
    uint64_t cache_answers;
    thinkthen_optional_string_v1 estimated_cost_usd;
    thinkthen_optional_u64_v1 input_tokens;
    thinkthen_optional_string_v1 model;
    thinkthen_optional_u64_v1 output_tokens;
    uint64_t records;
    uint64_t requests_sent;
    double seconds;
    thinkthen_optional_u64_v1 command_ms;
} thinkthen_facts_v1;
typedef struct thinkthen_optional_facts_v1 { int present; thinkthen_facts_v1 value; } thinkthen_optional_facts_v1;











typedef struct thinkthen_stopped_v1 {
    thinkthen_optional_size_v1 at;
    uint32_t cause;
    thinkthen_optional_u16_v1 status;
    int retryable;
} thinkthen_stopped_v1;
typedef struct thinkthen_optional_stopped_v1 { int present; thinkthen_stopped_v1 value; } thinkthen_optional_stopped_v1;
typedef struct thinkthen_error_v1 {
    int code;
    thinkthen_string_v1 message;
    int retryable;
    thinkthen_optional_stopped_v1 stopped;
} thinkthen_error_v1;
typedef struct thinkthen_optional_error_v1 { int present; thinkthen_error_v1 value; } thinkthen_optional_error_v1;

typedef struct thinkthen_row_v1 {
    thinkthen_string_v1 answer_id;
    thinkthen_optional_content_v1 input;
    thinkthen_optional_question_v1 question;
    thinkthen_optional_answer_v1 answer;
    thinkthen_optional_rule_v1 threshold;
    thinkthen_optional_location_v1 position;
    thinkthen_optional_string_v1 input_file;
    thinkthen_meta_v1 meta;
    thinkthen_optional_image_views_v1 images;
} thinkthen_row_v1;
typedef struct thinkthen_decide_view_v1 { thinkthen_row_v1 common; thinkthen_decide_value_v1 value; } thinkthen_decide_view_v1;
typedef struct thinkthen_choose_view_v1 { thinkthen_row_v1 common; thinkthen_optional_string_v1 value; } thinkthen_choose_view_v1;
typedef struct thinkthen_tag_view_v1 { thinkthen_row_v1 common; thinkthen_strings_v1 value; } thinkthen_tag_view_v1;
typedef struct thinkthen_score_view_v1 { thinkthen_row_v1 common; double value; } thinkthen_score_view_v1;
typedef struct thinkthen_filter_view_v1 { thinkthen_row_v1 common; int value; } thinkthen_filter_view_v1;
typedef struct thinkthen_rank_view_v1 {
    thinkthen_row_v1 common;
    thinkthen_optional_size_v1 value;
    thinkthen_optional_string_v1 question_name;
} thinkthen_rank_view_v1;
typedef struct thinkthen_find_view_v1 {
    thinkthen_row_v1 common;
    thinkthen_optional_content_v1 value;
    thinkthen_optional_size_v1 index;
} thinkthen_find_view_v1;
typedef struct thinkthen_annotate_view_v1 { thinkthen_row_v1 common; thinkthen_members_v1 answers; } thinkthen_annotate_view_v1;
typedef struct thinkthen_recognize_view_v1 {
    thinkthen_row_v1 common;
    thinkthen_recognize_value_v1 value;
    thinkthen_recognize_answer_v1 answer;
} thinkthen_recognize_view_v1;
typedef struct thinkthen_relate_view_v1 {
    thinkthen_row_v1 common;
    thinkthen_edges_v1 value;
    thinkthen_relation_answers_v1 questions;
} thinkthen_relate_view_v1;







typedef struct thinkthen_observed_probabilities_v1 {
    uint32_t kind;
    union { double yes; thinkthen_probabilities_v1 named; } data;
} thinkthen_observed_probabilities_v1;
typedef struct thinkthen_observation_success_v1 {
    thinkthen_string_v1 answer_id;
    thinkthen_string_v1 observation_id;
    thinkthen_member_value_v1 value;
    thinkthen_observed_probabilities_v1 probabilities;
    thinkthen_optional_double_v1 confidence;
} thinkthen_observation_success_v1;
typedef struct thinkthen_question_observation_v1 {
    size_t index;
    thinkthen_optional_string_v1 member;
    thinkthen_optional_discriminator_v1 stage;
    size_t position;
    thinkthen_string_v1 question_sha256;
    thinkthen_string_v1 model;
    thinkthen_string_v1 url;
    thinkthen_strings_v1 requests;
    uint64_t requests_sent;
    int cached;
    size_t failed_questions;
    thinkthen_optional_usage_v1 usage;
    thinkthen_question_sources_v1 question_sources;
    uint32_t state;
    union {
        thinkthen_observation_success_v1 success;
        thinkthen_member_failure_v1 failure;
    } data;
} thinkthen_question_observation_v1;
typedef struct thinkthen_row_observation_v1 {
    size_t index;
    uint32_t function;
    union {
        thinkthen_decide_view_v1 decide;
        thinkthen_choose_view_v1 choose;
        thinkthen_tag_view_v1 tag;
        thinkthen_score_view_v1 score;
        thinkthen_filter_view_v1 filter;
        thinkthen_rank_view_v1 rank;
        thinkthen_find_view_v1 find;
        thinkthen_annotate_view_v1 annotate;
        thinkthen_recognize_view_v1 recognize;
        thinkthen_relate_view_v1 relate;
    } data;
} thinkthen_row_observation_v1;


typedef struct thinkthen_observation_v1 {
    uint32_t kind;
    union {
        thinkthen_question_observation_v1 question;
        thinkthen_row_observation_v1 row;
    } data;
} thinkthen_observation_v1;


typedef struct thinkthen_summary_v1 {
    uint32_t state;
    thinkthen_string_v1 schema;
    thinkthen_optional_string_v1 answer_id;
    thinkthen_optional_discriminator_v1 function;
    size_t count;
    size_t observation_count;
    thinkthen_optional_meta_v1 meta;
    thinkthen_optional_facts_v1 facts;
    thinkthen_optional_attempts_v1 attempts;
    thinkthen_optional_error_v1 error;
} thinkthen_summary_v1;


typedef struct thinkthen_reported_usage_v1 {
    int present;
    thinkthen_optional_u64_v1 input_tokens;
    thinkthen_optional_u64_v1 output_tokens;
} thinkthen_reported_usage_v1;
typedef struct thinkthen_source_detail_v1 {
    uint32_t origin;
    thinkthen_string_v1 answered_by;
    thinkthen_optional_size_v1 batch_size;
} thinkthen_source_detail_v1;
typedef struct thinkthen_source_details_v1 { const thinkthen_source_detail_v1 *data; size_t len; } thinkthen_source_details_v1;
typedef struct thinkthen_input_view_v1 {
    thinkthen_optional_content_v1 original;
    thinkthen_optional_location_v1 position;
    thinkthen_optional_image_views_v1 images;
} thinkthen_input_view_v1;
typedef struct thinkthen_input_views_v1 { const thinkthen_input_view_v1 *data; size_t len; } thinkthen_input_views_v1;
typedef struct thinkthen_details_v1 {
    thinkthen_optional_question_v1 question;
    thinkthen_optional_rule_v1 threshold;
    thinkthen_optional_string_v1 raw_pick;
    thinkthen_reported_usage_v1 usage;
    thinkthen_source_details_v1 question_sources;
    thinkthen_observation_identities_v1 observations;
    thinkthen_input_views_v1 inputs;
} thinkthen_details_v1;



typedef struct thinkthen_source_entity_v1 { thinkthen_entity_v1 entity; thinkthen_optional_location_v1 position; } thinkthen_source_entity_v1;
typedef struct thinkthen_source_entities_v1 { const thinkthen_source_entity_v1 *data; size_t len; } thinkthen_source_entities_v1;
typedef struct thinkthen_source_entity_edge_v1 { thinkthen_string_v1 relation; thinkthen_source_entity_v1 source, target; double probability; int either; } thinkthen_source_entity_edge_v1;
typedef struct thinkthen_source_entity_edges_v1 { const thinkthen_source_entity_edge_v1 *data; size_t len; } thinkthen_source_entity_edges_v1;
typedef struct thinkthen_optional_source_entity_edges_v1 { int present; thinkthen_source_entity_edges_v1 value; } thinkthen_optional_source_entity_edges_v1;
typedef struct thinkthen_source_recognition_v1 { int present; thinkthen_source_entities_v1 entities; thinkthen_optional_source_entity_edges_v1 relations; } thinkthen_source_recognition_v1;
typedef struct thinkthen_source_endpoint_v1 { size_t ordinal; thinkthen_endpoint_v1 endpoint; thinkthen_content_v1 record; thinkthen_optional_location_v1 position; } thinkthen_source_endpoint_v1;
typedef struct thinkthen_source_edge_v1 { thinkthen_string_v1 relation; thinkthen_source_endpoint_v1 source, target; double probability; int either; } thinkthen_source_edge_v1;
typedef struct thinkthen_source_edges_v1 { const thinkthen_source_edge_v1 *data; size_t len; } thinkthen_source_edges_v1;
typedef struct thinkthen_source_relations_v1 { int present; thinkthen_source_edges_v1 edges; } thinkthen_source_relations_v1;
int thinkthen_result_source_recognition(const thinkthen_result *, size_t, thinkthen_source_recognition_v1 *);
int thinkthen_result_source_relations(const thinkthen_result *, size_t, thinkthen_source_relations_v1 *);
int thinkthen_result_details(const thinkthen_result *, size_t, thinkthen_details_v1 *);
int thinkthen_result_observation_details(const thinkthen_result *, size_t, thinkthen_details_v1 *);


int thinkthen_decide_batch_start(const thinkthen_engine *, const thinkthen_question *, const thinkthen_source *, const thinkthen_controls_v1 *, thinkthen_batch **);
int thinkthen_choose_batch_start(const thinkthen_engine *, const thinkthen_question *, const thinkthen_source *, const thinkthen_controls_v1 *, thinkthen_batch **);
int thinkthen_tag_batch_start(const thinkthen_engine *, const thinkthen_question *, const thinkthen_source *, const thinkthen_controls_v1 *, thinkthen_batch **);
int thinkthen_score_batch_start(const thinkthen_engine *, const thinkthen_question *, const thinkthen_source *, const thinkthen_controls_v1 *, thinkthen_batch **);
int thinkthen_filter_batch_start(const thinkthen_engine *, const thinkthen_question *, const thinkthen_source *, const thinkthen_controls_v1 *, thinkthen_batch **);
int thinkthen_annotate_batch_start(const thinkthen_engine *, const thinkthen_question *, const thinkthen_source *, const thinkthen_controls_v1 *, thinkthen_batch **);
int thinkthen_batch_next(thinkthen_batch *, thinkthen_result **);
int thinkthen_batch_facts(const thinkthen_batch *, thinkthen_result **);
void thinkthen_batch_free(thinkthen_batch *);

int thinkthen_decide_complete(const thinkthen_engine *, const thinkthen_question *, const thinkthen_source *, const thinkthen_controls_v1 *, thinkthen_result **);
int thinkthen_choose_complete(const thinkthen_engine *, const thinkthen_question *, const thinkthen_source *, const thinkthen_controls_v1 *, thinkthen_result **);
int thinkthen_tag_complete(const thinkthen_engine *, const thinkthen_question *, const thinkthen_source *, const thinkthen_controls_v1 *, thinkthen_result **);
int thinkthen_score_complete(const thinkthen_engine *, const thinkthen_question *, const thinkthen_source *, const thinkthen_controls_v1 *, thinkthen_result **);
int thinkthen_filter_complete(const thinkthen_engine *, const thinkthen_question *, const thinkthen_source *, const thinkthen_controls_v1 *, thinkthen_result **);
int thinkthen_rank_complete(const thinkthen_engine *, const thinkthen_question *, const thinkthen_source *, const thinkthen_controls_v1 *, thinkthen_result **);
int thinkthen_find_complete(const thinkthen_engine *, const thinkthen_question *, const thinkthen_source *, const thinkthen_controls_v1 *, thinkthen_result **);
int thinkthen_annotate_complete(const thinkthen_engine *, const thinkthen_question *, const thinkthen_source *, const thinkthen_controls_v1 *, thinkthen_result **);
int thinkthen_recognize_complete(const thinkthen_engine *, const thinkthen_question *, const thinkthen_source *, const thinkthen_controls_v1 *, thinkthen_result **);
int thinkthen_relate_complete(const thinkthen_engine *, const thinkthen_question *, const thinkthen_source *, const thinkthen_controls_v1 *, thinkthen_result **);
void thinkthen_result_free(thinkthen_result *);
int thinkthen_result_summary(const thinkthen_result *, thinkthen_summary_v1 *);
int thinkthen_result_observation(const thinkthen_result *, size_t, thinkthen_observation_v1 *);
int thinkthen_result_row(const thinkthen_result *, size_t, thinkthen_row_observation_v1 *);
int thinkthen_result_decide(const thinkthen_result *, size_t, thinkthen_decide_view_v1 *);
int thinkthen_result_choose(const thinkthen_result *, size_t, thinkthen_choose_view_v1 *);
int thinkthen_result_tag(const thinkthen_result *, size_t, thinkthen_tag_view_v1 *);
int thinkthen_result_score(const thinkthen_result *, size_t, thinkthen_score_view_v1 *);
int thinkthen_result_filter(const thinkthen_result *, size_t, thinkthen_filter_view_v1 *);
int thinkthen_result_rank(const thinkthen_result *, size_t, thinkthen_rank_view_v1 *);

int thinkthen_result_rank_member_count(const thinkthen_result *, size_t, size_t *);
int thinkthen_result_rank_member_details(const thinkthen_result *, size_t, size_t, thinkthen_details_v1 *);
int thinkthen_result_rank_member(const thinkthen_result *, size_t, size_t, thinkthen_rank_view_v1 *);
int thinkthen_result_find(const thinkthen_result *, size_t, thinkthen_find_view_v1 *);
int thinkthen_result_annotate(const thinkthen_result *, size_t, thinkthen_annotate_view_v1 *);
int thinkthen_result_recognize(const thinkthen_result *, size_t, thinkthen_recognize_view_v1 *);
int thinkthen_result_relate(const thinkthen_result *, size_t, thinkthen_relate_view_v1 *);

int thinkthen_error_complete(const thinkthen_engine *, thinkthen_result **);


int thinkthen_question_new(const thinkthen_engine *, const thinkthen_question_spec_v1 *, thinkthen_question **);
int thinkthen_question_load(const thinkthen_engine *, thinkthen_string_v1 path, thinkthen_question **);

enum thinkthen_input_declaration_kind_v1 {
    THINKTHEN_DECLARATION_ABSENT_V1=0, THINKTHEN_DECLARATION_STRING_V1=1,
    THINKTHEN_DECLARATION_OBJECT_V1=2
};
enum thinkthen_input_property_kind_v1 {
    THINKTHEN_PROPERTY_STRING_V1=1, THINKTHEN_PROPERTY_NUMBER_V1=2,
    THINKTHEN_PROPERTY_BOOLEAN_V1=3, THINKTHEN_PROPERTY_STRING_LIST_V1=4
};
enum thinkthen_question_loader_role_v1 {
    THINKTHEN_LOAD_ATOMIC_V1=1, THINKTHEN_LOAD_SET_V1=2,
    THINKTHEN_LOAD_DYNAMIC_CHOOSE_V1=3, THINKTHEN_LOAD_RECOGNIZE_V1=4,
    THINKTHEN_LOAD_RELATE_V1=5, THINKTHEN_LOAD_RANK_V1=6,
    THINKTHEN_LOAD_RANK_SET_V1=7, THINKTHEN_LOAD_FIND_V1=8
};
typedef struct thinkthen_input_property_v1 {
    thinkthen_string_v1 name; uint32_t kind;
} thinkthen_input_property_v1;
typedef struct thinkthen_input_properties_v1 {
    const thinkthen_input_property_v1 *data; size_t len;
} thinkthen_input_properties_v1;
typedef struct thinkthen_input_declaration_v1 {
    uint32_t kind;
    thinkthen_input_properties_v1 properties;
    thinkthen_strings_v1 required;
} thinkthen_input_declaration_v1;
typedef struct thinkthen_question_author_v1 {
    thinkthen_optional_string_v1 name;
    thinkthen_optional_u64_v1 wording_version;
    thinkthen_input_declaration_v1 item_schema, context_schema;
} thinkthen_question_author_v1;

int thinkthen_question_new_authored(const thinkthen_engine *, const thinkthen_question_spec_v1 *, const thinkthen_question_author_v1 *, thinkthen_question **);

int thinkthen_question_author(const thinkthen_question *, thinkthen_question_author_v1 *);


int thinkthen_question_parse(const thinkthen_engine *, uint32_t role, thinkthen_string_v1 json, thinkthen_question **);
int thinkthen_question_load_named(const thinkthen_engine *, uint32_t role, thinkthen_string_v1 name, thinkthen_question **);
int thinkthen_question_load_reference(const thinkthen_engine *, uint32_t role, thinkthen_string_v1 reference, thinkthen_question **);

int thinkthen_result_question_author(const thinkthen_result *, size_t row, thinkthen_question_author_v1 *);
int thinkthen_result_member_author(const thinkthen_result *, size_t row, size_t member, thinkthen_question_author_v1 *);
int thinkthen_result_observation_author(const thinkthen_result *, size_t observation, thinkthen_question_author_v1 *);
void thinkthen_question_free(thinkthen_question *);
int thinkthen_image_clone(const thinkthen_engine *, const uint8_t *, size_t, uint32_t media, thinkthen_optional_string_v1 filename, thinkthen_image **);
int thinkthen_image_view(const thinkthen_image *, thinkthen_image_view_v1 *);
void thinkthen_image_free(thinkthen_image *);
int thinkthen_source_records(const thinkthen_engine *, const thinkthen_record_v1 *, size_t, thinkthen_source **);
int thinkthen_source_files(const thinkthen_engine *, const thinkthen_source_spec_v1 *, thinkthen_source **);

int thinkthen_source_image_files(const thinkthen_engine *, const thinkthen_source_spec_v1 *, thinkthen_source **);
void thinkthen_source_free(thinkthen_source *);
