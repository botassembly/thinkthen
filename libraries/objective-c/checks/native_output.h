#include "TTNativeAPI.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
static void quoted(thinkthen_string_v1 s) { putchar(34); for(size_t i=0;i<s.len;++i) { unsigned char c=s.data[i]; if(c==34 || c==92) { putchar(92);putchar(c); } else if(c<32) printf("\\u%04x",c); else putchar(c); } putchar(34); }
static void emit_annotate_view(thinkthen_annotate_view_v1 v);
static void emit_answer(thinkthen_answer_v1 v);
static void emit_attempt(thinkthen_attempt_v1 v);
static void emit_attempts(thinkthen_attempts_v1 v);
static void emit_batch(thinkthen_batch_v1 v);
static void emit_batch_warning(thinkthen_batch_warning_v1 v);
static void emit_choice(thinkthen_choice_v1 v);
static void emit_choices(thinkthen_choices_v1 v);
static void emit_choose_view(thinkthen_choose_view_v1 v);
static void emit_content(thinkthen_content_v1 v);
static void emit_decide_value(thinkthen_decide_value_v1 v);
static void emit_decide_view(thinkthen_decide_view_v1 v);
static void emit_details(thinkthen_details_v1 v);
static void emit_edge(thinkthen_edge_v1 v);
static void emit_edges(thinkthen_edges_v1 v);
static void emit_endpoint(thinkthen_endpoint_v1 v);
static void emit_entities(thinkthen_entities_v1 v);
static void emit_entity_edge(thinkthen_entity_edge_v1 v);
static void emit_entity_edges(thinkthen_entity_edges_v1 v);
static void emit_entity(thinkthen_entity_v1 v);
static void emit_error(thinkthen_error_v1 v);
static void emit_facts(thinkthen_facts_v1 v);
static void emit_filter_view(thinkthen_filter_view_v1 v);
static void emit_find_view(thinkthen_find_view_v1 v);
static void emit_image_view(thinkthen_image_view_v1 v);
static void emit_image_views(thinkthen_image_views_v1 v);
static void emit_input_declaration(thinkthen_input_declaration_v1 v);
static void emit_input_properties(thinkthen_input_properties_v1 v);
static void emit_input_property(thinkthen_input_property_v1 v);
static void emit_input_view(thinkthen_input_view_v1 v);
static void emit_input_views(thinkthen_input_views_v1 v);
static void emit_location(thinkthen_location_v1 v);
static void emit_member_failure(thinkthen_member_failure_v1 v);
static void emit_member_success(thinkthen_member_success_v1 v);
static void emit_member(thinkthen_member_v1 v);
static void emit_member_value(thinkthen_member_value_v1 v);
static void emit_members(thinkthen_members_v1 v);
static void emit_meta(thinkthen_meta_v1 v);
static void emit_name(thinkthen_name_v1 v);
static void emit_named_answer(thinkthen_named_answer_v1 v);
static void emit_names(thinkthen_names_v1 v);
static void emit_observation_identities(thinkthen_observation_identities_v1 v);
static void emit_observation_identity(thinkthen_observation_identity_v1 v);
static void emit_observation_success(thinkthen_observation_success_v1 v);
static void emit_observation(thinkthen_observation_v1 v);
static void emit_observed_probabilities(thinkthen_observed_probabilities_v1 v);
static void emit_optional_answer(thinkthen_optional_answer_v1 v);
static void emit_optional_attempts(thinkthen_optional_attempts_v1 v);
static void emit_optional_batch(thinkthen_optional_batch_v1 v);
static void emit_optional_batch_warning(thinkthen_optional_batch_warning_v1 v);
static void emit_optional_content(thinkthen_optional_content_v1 v);
static void emit_optional_discriminator(thinkthen_optional_discriminator_v1 v);
static void emit_optional_double(thinkthen_optional_double_v1 v);
static void emit_optional_endpoint(thinkthen_optional_endpoint_v1 v);
static void emit_optional_entity_edges(thinkthen_optional_entity_edges_v1 v);
static void emit_optional_error(thinkthen_optional_error_v1 v);
static void emit_optional_facts(thinkthen_optional_facts_v1 v);
static void emit_optional_image_views(thinkthen_optional_image_views_v1 v);
static void emit_optional_location(thinkthen_optional_location_v1 v);
static void emit_optional_meta(thinkthen_optional_meta_v1 v);
static void emit_optional_probabilities(thinkthen_optional_probabilities_v1 v);
static void emit_optional_profile_warning(thinkthen_optional_profile_warning_v1 v);
static void emit_optional_question(thinkthen_optional_question_v1 v);
static void emit_optional_rule(thinkthen_optional_rule_v1 v);
static void emit_optional_size(thinkthen_optional_size_v1 v);
static void emit_optional_source_entity_edges(thinkthen_optional_source_entity_edges_v1 v);
static void emit_optional_stopped(thinkthen_optional_stopped_v1 v);
static void emit_optional_string(thinkthen_optional_string_v1 v);
static void emit_optional_u16(thinkthen_optional_u16_v1 v);
static void emit_optional_u64(thinkthen_optional_u64_v1 v);
static void emit_optional_usage(thinkthen_optional_usage_v1 v);
static void emit_pair(thinkthen_pair_v1 v);
static void emit_pairs(thinkthen_pairs_v1 v);
static void emit_piece(thinkthen_piece_v1 v);
static void emit_pieces(thinkthen_pieces_v1 v);
static void emit_place(thinkthen_place_v1 v);
static void emit_probabilities(thinkthen_probabilities_v1 v);
static void emit_probability(thinkthen_probability_v1 v);
static void emit_profile_warning(thinkthen_profile_warning_v1 v);
static void emit_question_author(thinkthen_question_author_v1 v);
static void emit_question_member(thinkthen_question_member_v1 v);
static void emit_question_members(thinkthen_question_members_v1 v);
static void emit_question_observation(thinkthen_question_observation_v1 v);
static void emit_question_source(thinkthen_question_source_v1 v);
static void emit_question_sources(thinkthen_question_sources_v1 v);
static void emit_question_view(thinkthen_question_view_v1 v);
static void emit_rank_view(thinkthen_rank_view_v1 v);
static void emit_recognize_answer(thinkthen_recognize_answer_v1 v);
static void emit_recognize_value(thinkthen_recognize_value_v1 v);
static void emit_recognize_view(thinkthen_recognize_view_v1 v);
static void emit_relate_view(thinkthen_relate_view_v1 v);
static void emit_relation_answer(thinkthen_relation_answer_v1 v);
static void emit_relation_answers(thinkthen_relation_answers_v1 v);
static void emit_relation_success(thinkthen_relation_success_v1 v);
static void emit_relation(thinkthen_relation_v1 v);
static void emit_relations(thinkthen_relations_v1 v);
static void emit_reported_usage(thinkthen_reported_usage_v1 v);
static void emit_row_observation(thinkthen_row_observation_v1 v);
static void emit_row(thinkthen_row_v1 v);
static void emit_rule(thinkthen_rule_v1 v);
static void emit_score_answer(thinkthen_score_answer_v1 v);
static void emit_score_view(thinkthen_score_view_v1 v);
static void emit_source_detail(thinkthen_source_detail_v1 v);
static void emit_source_details(thinkthen_source_details_v1 v);
static void emit_source_edge(thinkthen_source_edge_v1 v);
static void emit_source_edges(thinkthen_source_edges_v1 v);
static void emit_source_endpoint(thinkthen_source_endpoint_v1 v);
static void emit_source_entities(thinkthen_source_entities_v1 v);
static void emit_source_entity_edge(thinkthen_source_entity_edge_v1 v);
static void emit_source_entity_edges(thinkthen_source_entity_edges_v1 v);
static void emit_source_entity(thinkthen_source_entity_v1 v);
static void emit_source_recognition(thinkthen_source_recognition_v1 v);
static void emit_source_relations(thinkthen_source_relations_v1 v);
static void emit_stopped(thinkthen_stopped_v1 v);
static void emit_string(thinkthen_string_v1 v);
static void emit_strings(thinkthen_strings_v1 v);
static void emit_summary(thinkthen_summary_v1 v);
static void emit_tag_view(thinkthen_tag_view_v1 v);
static void emit_usage(thinkthen_usage_v1 v);
static void emit_annotate_view(thinkthen_annotate_view_v1 v) { putchar(123); fputs("\"common\":",stdout); emit_row(v.common); putchar(44); fputs("\"answers\":",stdout); emit_members(v.answers); putchar(125); }
static void emit_answer(thinkthen_answer_v1 v) { putchar(123); fputs("\"kind\":",stdout); printf("%llu",(unsigned long long)v.kind); putchar(44); fputs("\"data\":",stdout); putchar(123); fputs("\"probability\":",stdout); if(v.kind==1) { printf("%.17g",v.data.probability); } else fputs("null",stdout); putchar(44); fputs("\"choice\":",stdout); if(v.kind==2) { emit_named_answer(v.data.choice); } else fputs("null",stdout); putchar(44); fputs("\"tag\":",stdout); if(v.kind==3) { emit_probabilities(v.data.tag); } else fputs("null",stdout); putchar(44); fputs("\"score\":",stdout); if(v.kind==4) { emit_score_answer(v.data.score); } else fputs("null",stdout); putchar(44); fputs("\"find\":",stdout); if(v.kind==5) { emit_named_answer(v.data.find); } else fputs("null",stdout); putchar(125); putchar(125); }
static void emit_attempt(thinkthen_attempt_v1 v) { putchar(123); fputs("\"ordinal\":",stdout); printf("%llu",(unsigned long long)v.ordinal); putchar(44); fputs("\"request_sha256\":",stdout); emit_string(v.request_sha256); putchar(44); fputs("\"wall_ms\":",stdout); printf("%llu",(unsigned long long)v.wall_ms); putchar(44); fputs("\"outcome\":",stdout); printf("%llu",(unsigned long long)v.outcome); putchar(44); fputs("\"sdk_request_id\":",stdout); emit_string(v.sdk_request_id); putchar(44); fputs("\"status\":",stdout); emit_optional_u16(v.status); putchar(44); fputs("\"server_ms\":",stdout); emit_optional_u64(v.server_ms); putchar(44); fputs("\"request_id\":",stdout); emit_optional_string(v.request_id); putchar(125); }
static void emit_attempts(thinkthen_attempts_v1 v) { putchar(91); for(size_t i=0;i<v.len;++i) { if(i) putchar(44); emit_attempt(v.data[i]); } putchar(93); }
static void emit_batch(thinkthen_batch_v1 v) { putchar(123); fputs("\"kind\":",stdout); printf("%llu",(unsigned long long)v.kind); putchar(44); fputs("\"records\":",stdout); printf("%llu",(unsigned long long)v.records); putchar(125); }
static void emit_batch_warning(thinkthen_batch_warning_v1 v) { putchar(123); fputs("\"tuned_for\":",stdout); emit_batch(v.tuned_for); putchar(44); fputs("\"running\":",stdout); emit_batch(v.running); putchar(125); }
static void emit_choice(thinkthen_choice_v1 v) { putchar(123); fputs("\"name\":",stdout); emit_string(v.name); putchar(44); fputs("\"description\":",stdout); emit_optional_content(v.description); putchar(44); fputs("\"weight\":",stdout); emit_optional_double(v.weight); putchar(125); }
static void emit_choices(thinkthen_choices_v1 v) { putchar(91); for(size_t i=0;i<v.len;++i) { if(i) putchar(44); emit_choice(v.data[i]); } putchar(93); }
static void emit_choose_view(thinkthen_choose_view_v1 v) { putchar(123); fputs("\"common\":",stdout); emit_row(v.common); putchar(44); fputs("\"value\":",stdout); emit_optional_string(v.value); putchar(125); }
static void emit_content(thinkthen_content_v1 v) { putchar(123); fputs("\"kind\":",stdout); printf("%llu",(unsigned long long)v.kind); putchar(44); fputs("\"data\":",stdout); emit_string(v.data); putchar(125); }
static void emit_decide_value(thinkthen_decide_value_v1 v) { putchar(123); fputs("\"kind\":",stdout); printf("%llu",(unsigned long long)v.kind); putchar(44); fputs("\"data\":",stdout); putchar(123); fputs("\"boolean\":",stdout); if(v.kind==1) { printf("%llu",(unsigned long long)v.data.boolean); } else fputs("null",stdout); putchar(44); fputs("\"authored\":",stdout); if(v.kind==2) { emit_content(v.data.authored); } else fputs("null",stdout); putchar(125); putchar(125); }
static void emit_decide_view(thinkthen_decide_view_v1 v) { putchar(123); fputs("\"common\":",stdout); emit_row(v.common); putchar(44); fputs("\"value\":",stdout); emit_decide_value(v.value); putchar(125); }
static void emit_details(thinkthen_details_v1 v) { putchar(123); fputs("\"question\":",stdout); emit_optional_question(v.question); putchar(44); fputs("\"threshold\":",stdout); emit_optional_rule(v.threshold); putchar(44); fputs("\"raw_pick\":",stdout); emit_optional_string(v.raw_pick); putchar(44); fputs("\"usage\":",stdout); emit_reported_usage(v.usage); putchar(44); fputs("\"question_sources\":",stdout); emit_source_details(v.question_sources); putchar(44); fputs("\"observations\":",stdout); emit_observation_identities(v.observations); putchar(44); fputs("\"inputs\":",stdout); emit_input_views(v.inputs); putchar(125); }
static void emit_edge(thinkthen_edge_v1 v) { putchar(123); fputs("\"relation\":",stdout); emit_string(v.relation); putchar(44); fputs("\"source\":",stdout); emit_endpoint(v.source); putchar(44); fputs("\"target\":",stdout); emit_endpoint(v.target); putchar(44); fputs("\"probability\":",stdout); printf("%.17g",v.probability); putchar(44); fputs("\"either\":",stdout); printf("%llu",(unsigned long long)v.either); putchar(125); }
static void emit_edges(thinkthen_edges_v1 v) { putchar(91); for(size_t i=0;i<v.len;++i) { if(i) putchar(44); emit_edge(v.data[i]); } putchar(93); }
static void emit_endpoint(thinkthen_endpoint_v1 v) { putchar(123); fputs("\"name\":",stdout); emit_string(v.name); putchar(44); fputs("\"kind\":",stdout); emit_string(v.kind); putchar(125); }
static void emit_entities(thinkthen_entities_v1 v) { putchar(91); for(size_t i=0;i<v.len;++i) { if(i) putchar(44); emit_entity(v.data[i]); } putchar(93); }
static void emit_entity_edge(thinkthen_entity_edge_v1 v) { putchar(123); fputs("\"relation\":",stdout); emit_string(v.relation); putchar(44); fputs("\"source\":",stdout); emit_entity(v.source); putchar(44); fputs("\"target\":",stdout); emit_entity(v.target); putchar(44); fputs("\"probability\":",stdout); printf("%.17g",v.probability); putchar(44); fputs("\"either\":",stdout); printf("%llu",(unsigned long long)v.either); putchar(125); }
static void emit_entity_edges(thinkthen_entity_edges_v1 v) { putchar(91); for(size_t i=0;i<v.len;++i) { if(i) putchar(44); emit_entity_edge(v.data[i]); } putchar(93); }
static void emit_entity(thinkthen_entity_v1 v) { putchar(123); fputs("\"text\":",stdout); emit_string(v.text); putchar(44); fputs("\"start\":",stdout); printf("%llu",(unsigned long long)v.start); putchar(44); fputs("\"end\":",stdout); printf("%llu",(unsigned long long)v.end); putchar(44); fputs("\"length\":",stdout); printf("%llu",(unsigned long long)v.length); putchar(44); fputs("\"kind\":",stdout); emit_string(v.kind); putchar(44); fputs("\"strength\":",stdout); printf("%.17g",v.strength); putchar(125); }
static void emit_error(thinkthen_error_v1 v) { putchar(123); fputs("\"code\":",stdout); printf("%llu",(unsigned long long)v.code); putchar(44); fputs("\"message\":",stdout); emit_string(v.message); putchar(44); fputs("\"retryable\":",stdout); printf("%llu",(unsigned long long)v.retryable); putchar(44); fputs("\"stopped\":",stdout); emit_optional_stopped(v.stopped); putchar(125); }
static void emit_facts(thinkthen_facts_v1 v) { putchar(123); fputs("\"call_id\":",stdout); emit_string(v.call_id); putchar(44); fputs("\"cache_answers\":",stdout); printf("%llu",(unsigned long long)v.cache_answers); putchar(44); fputs("\"estimated_cost_usd\":",stdout); emit_optional_string(v.estimated_cost_usd); putchar(44); fputs("\"input_tokens\":",stdout); emit_optional_u64(v.input_tokens); putchar(44); fputs("\"model\":",stdout); emit_optional_string(v.model); putchar(44); fputs("\"output_tokens\":",stdout); emit_optional_u64(v.output_tokens); putchar(44); fputs("\"records\":",stdout); printf("%llu",(unsigned long long)v.records); putchar(44); fputs("\"requests_sent\":",stdout); printf("%llu",(unsigned long long)v.requests_sent); putchar(44); fputs("\"seconds\":",stdout); printf("%.17g",v.seconds); putchar(44); fputs("\"command_ms\":",stdout); emit_optional_u64(v.command_ms); putchar(125); }
static void emit_filter_view(thinkthen_filter_view_v1 v) { putchar(123); fputs("\"common\":",stdout); emit_row(v.common); putchar(44); fputs("\"value\":",stdout); printf("%llu",(unsigned long long)v.value); putchar(125); }
static void emit_find_view(thinkthen_find_view_v1 v) { putchar(123); fputs("\"common\":",stdout); emit_row(v.common); putchar(44); fputs("\"value\":",stdout); emit_optional_content(v.value); putchar(44); fputs("\"index\":",stdout); emit_optional_size(v.index); putchar(125); }
static void emit_image_view(thinkthen_image_view_v1 v) { putchar(123); fputs("\"media\":",stdout); printf("%llu",(unsigned long long)v.media); putchar(44); fputs("\"bytes\":",stdout); putchar(91); for(size_t i=0;i<v.bytes_len;++i) { if(i) putchar(44); printf("%u",v.bytes[i]); } putchar(93); putchar(44); fputs("\"width\":",stdout); printf("%llu",(unsigned long long)v.width); putchar(44); fputs("\"height\":",stdout); printf("%llu",(unsigned long long)v.height); putchar(44); fputs("\"filename\":",stdout); emit_optional_string(v.filename); putchar(125); }
static void emit_image_views(thinkthen_image_views_v1 v) { putchar(91); for(size_t i=0;i<v.len;++i) { if(i) putchar(44); emit_image_view(v.data[i]); } putchar(93); }
static void emit_input_declaration(thinkthen_input_declaration_v1 v) { putchar(123); fputs("\"kind\":",stdout); printf("%llu",(unsigned long long)v.kind); putchar(44); fputs("\"properties\":",stdout); emit_input_properties(v.properties); putchar(44); fputs("\"required\":",stdout); emit_strings(v.required); putchar(125); }
static void emit_input_properties(thinkthen_input_properties_v1 v) { putchar(91); for(size_t i=0;i<v.len;++i) { if(i) putchar(44); emit_input_property(v.data[i]); } putchar(93); }
static void emit_input_property(thinkthen_input_property_v1 v) { putchar(123); fputs("\"name\":",stdout); emit_string(v.name); putchar(44); fputs("\"kind\":",stdout); printf("%llu",(unsigned long long)v.kind); putchar(125); }
static void emit_input_view(thinkthen_input_view_v1 v) { putchar(123); fputs("\"original\":",stdout); emit_optional_content(v.original); putchar(44); fputs("\"position\":",stdout); emit_optional_location(v.position); putchar(44); fputs("\"images\":",stdout); emit_optional_image_views(v.images); putchar(125); }
static void emit_input_views(thinkthen_input_views_v1 v) { putchar(91); for(size_t i=0;i<v.len;++i) { if(i) putchar(44); emit_input_view(v.data[i]); } putchar(93); }
static void emit_location(thinkthen_location_v1 v) { putchar(123); fputs("\"file\":",stdout); emit_optional_string(v.file); putchar(44); fputs("\"first_line\":",stdout); emit_optional_size(v.first_line); putchar(44); fputs("\"last_line\":",stdout); emit_optional_size(v.last_line); putchar(125); }
static void emit_member_failure(thinkthen_member_failure_v1 v) { putchar(123); fputs("\"failure_id\":",stdout); emit_string(v.failure_id); putchar(44); fputs("\"cause\":",stdout); printf("%llu",(unsigned long long)v.cause); putchar(125); }
static void emit_member_success(thinkthen_member_success_v1 v) { putchar(123); fputs("\"answer_id\":",stdout); emit_string(v.answer_id); putchar(44); fputs("\"value\":",stdout); emit_member_value(v.value); putchar(44); fputs("\"answer\":",stdout); emit_answer(v.answer); putchar(44); fputs("\"threshold\":",stdout); emit_rule(v.threshold); putchar(125); }
static void emit_member(thinkthen_member_v1 v) { putchar(123); fputs("\"name\":",stdout); emit_string(v.name); putchar(44); fputs("\"request\":",stdout); emit_string(v.request); putchar(44); fputs("\"question\":",stdout); emit_question_view(v.question); putchar(44); fputs("\"state\":",stdout); printf("%llu",(unsigned long long)v.state); putchar(44); fputs("\"data\":",stdout); putchar(123); fputs("\"success\":",stdout); if(v.state==1) { emit_member_success(v.data.success); } else fputs("null",stdout); putchar(44); fputs("\"failure\":",stdout); if(v.state==2) { emit_member_failure(v.data.failure); } else fputs("null",stdout); putchar(125); putchar(125); }
static void emit_member_value(thinkthen_member_value_v1 v) { putchar(123); fputs("\"kind\":",stdout); printf("%llu",(unsigned long long)v.kind); putchar(44); fputs("\"data\":",stdout); putchar(123); fputs("\"decide\":",stdout); if(v.kind==1) { emit_decide_value(v.data.decide); } else fputs("null",stdout); putchar(44); fputs("\"choose\":",stdout); if(v.kind==2) { emit_optional_string(v.data.choose); } else fputs("null",stdout); putchar(44); fputs("\"tag\":",stdout); if(v.kind==3) { emit_strings(v.data.tag); } else fputs("null",stdout); putchar(44); fputs("\"score\":",stdout); if(v.kind==4) { printf("%.17g",v.data.score); } else fputs("null",stdout); putchar(125); putchar(125); }
static void emit_members(thinkthen_members_v1 v) { putchar(91); for(size_t i=0;i<v.len;++i) { if(i) putchar(44); emit_member(v.data[i]); } putchar(93); }
static void emit_meta(thinkthen_meta_v1 v) { putchar(123); fputs("\"tool\":",stdout); emit_string(v.tool); putchar(44); fputs("\"question_sha256\":",stdout); emit_optional_string(v.question_sha256); putchar(44); fputs("\"questions_sha256\":",stdout); emit_optional_string(v.questions_sha256); putchar(44); fputs("\"url\":",stdout); emit_string(v.url); putchar(44); fputs("\"model\":",stdout); emit_string(v.model); putchar(44); fputs("\"usage\":",stdout); emit_optional_usage(v.usage); putchar(44); fputs("\"requests_sent\":",stdout); printf("%llu",(unsigned long long)v.requests_sent); putchar(44); fputs("\"cached\":",stdout); printf("%llu",(unsigned long long)v.cached); putchar(44); fputs("\"requests\":",stdout); emit_strings(v.requests); putchar(44); fputs("\"failed_questions\":",stdout); printf("%llu",(unsigned long long)v.failed_questions); putchar(44); fputs("\"profile_warning\":",stdout); emit_optional_profile_warning(v.profile_warning); putchar(44); fputs("\"batch_setting\":",stdout); emit_optional_batch(v.batch_setting); putchar(44); fputs("\"batch_warning\":",stdout); emit_optional_batch_warning(v.batch_warning); putchar(44); fputs("\"context_sha256\":",stdout); emit_optional_string(v.context_sha256); putchar(44); fputs("\"attempts\":",stdout); emit_optional_attempts(v.attempts); putchar(44); fputs("\"origin\":",stdout); emit_optional_discriminator(v.origin); putchar(44); fputs("\"question_sources\":",stdout); emit_question_sources(v.question_sources); putchar(44); fputs("\"observations\":",stdout); emit_observation_identities(v.observations); putchar(44); fputs("\"answered_by\":",stdout); emit_optional_string(v.answered_by); putchar(125); }
static void emit_name(thinkthen_name_v1 v) { putchar(123); fputs("\"start\":",stdout); printf("%llu",(unsigned long long)v.start); putchar(44); fputs("\"end\":",stdout); printf("%llu",(unsigned long long)v.end); putchar(44); fputs("\"kinds\":",stdout); emit_optional_probabilities(v.kinds); putchar(44); fputs("\"edges\":",stdout); emit_optional_probabilities(v.edges); putchar(125); }
static void emit_named_answer(thinkthen_named_answer_v1 v) { putchar(123); fputs("\"pick\":",stdout); emit_string(v.pick); putchar(44); fputs("\"probabilities\":",stdout); emit_probabilities(v.probabilities); putchar(44); fputs("\"confidence\":",stdout); emit_optional_double(v.confidence); putchar(125); }
static void emit_names(thinkthen_names_v1 v) { putchar(91); for(size_t i=0;i<v.len;++i) { if(i) putchar(44); emit_name(v.data[i]); } putchar(93); }
static void emit_observation_identities(thinkthen_observation_identities_v1 v) { putchar(91); for(size_t i=0;i<v.len;++i) { if(i) putchar(44); emit_observation_identity(v.data[i]); } putchar(93); }
static void emit_observation_identity(thinkthen_observation_identity_v1 v) { putchar(123); fputs("\"kind\":",stdout); printf("%llu",(unsigned long long)v.kind); putchar(44); fputs("\"data\":",stdout); putchar(123); fputs("\"observation_id\":",stdout); if(v.kind==1) { emit_string(v.data.observation_id); } else fputs("null",stdout); putchar(44); fputs("\"failure_id\":",stdout); if(v.kind==2) { emit_string(v.data.failure_id); } else fputs("null",stdout); putchar(125); putchar(125); }
static void emit_observation_success(thinkthen_observation_success_v1 v) { putchar(123); fputs("\"answer_id\":",stdout); emit_string(v.answer_id); putchar(44); fputs("\"observation_id\":",stdout); emit_string(v.observation_id); putchar(44); fputs("\"value\":",stdout); emit_member_value(v.value); putchar(44); fputs("\"probabilities\":",stdout); emit_observed_probabilities(v.probabilities); putchar(44); fputs("\"confidence\":",stdout); emit_optional_double(v.confidence); putchar(125); }
static void emit_observation(thinkthen_observation_v1 v) { putchar(123); fputs("\"kind\":",stdout); printf("%llu",(unsigned long long)v.kind); putchar(44); fputs("\"data\":",stdout); putchar(123); fputs("\"question\":",stdout); if(v.kind==1) { emit_question_observation(v.data.question); } else fputs("null",stdout); putchar(44); fputs("\"row\":",stdout); if(v.kind==2) { emit_row_observation(v.data.row); } else fputs("null",stdout); putchar(125); putchar(125); }
static void emit_observed_probabilities(thinkthen_observed_probabilities_v1 v) { putchar(123); fputs("\"kind\":",stdout); printf("%llu",(unsigned long long)v.kind); putchar(44); fputs("\"data\":",stdout); putchar(123); fputs("\"yes\":",stdout); if(v.kind==1) { printf("%.17g",v.data.yes); } else fputs("null",stdout); putchar(44); fputs("\"named\":",stdout); if(v.kind==2) { emit_probabilities(v.data.named); } else fputs("null",stdout); putchar(125); putchar(125); }
static void emit_optional_answer(thinkthen_optional_answer_v1 v) { if(v.present) { emit_answer(v.value); } else fputs("null",stdout); }
static void emit_optional_attempts(thinkthen_optional_attempts_v1 v) { if(v.present) { emit_attempts(v.value); } else fputs("null",stdout); }
static void emit_optional_batch(thinkthen_optional_batch_v1 v) { if(v.present) { emit_batch(v.value); } else fputs("null",stdout); }
static void emit_optional_batch_warning(thinkthen_optional_batch_warning_v1 v) { if(v.present) { emit_batch_warning(v.value); } else fputs("null",stdout); }
static void emit_optional_content(thinkthen_optional_content_v1 v) { if(v.present) { emit_content(v.value); } else fputs("null",stdout); }
static void emit_optional_discriminator(thinkthen_optional_discriminator_v1 v) { if(v.present) { printf("%llu",(unsigned long long)v.value); } else fputs("null",stdout); }
static void emit_optional_double(thinkthen_optional_double_v1 v) { if(v.present) { printf("%.17g",v.value); } else fputs("null",stdout); }
static void emit_optional_endpoint(thinkthen_optional_endpoint_v1 v) { if(v.present) { emit_endpoint(v.value); } else fputs("null",stdout); }
static void emit_optional_entity_edges(thinkthen_optional_entity_edges_v1 v) { if(v.present) { emit_entity_edges(v.value); } else fputs("null",stdout); }
static void emit_optional_error(thinkthen_optional_error_v1 v) { if(v.present) { emit_error(v.value); } else fputs("null",stdout); }
static void emit_optional_facts(thinkthen_optional_facts_v1 v) { if(v.present) { emit_facts(v.value); } else fputs("null",stdout); }
static void emit_optional_image_views(thinkthen_optional_image_views_v1 v) { if(v.present) { emit_image_views(v.value); } else fputs("null",stdout); }
static void emit_optional_location(thinkthen_optional_location_v1 v) { if(v.present) { emit_location(v.value); } else fputs("null",stdout); }
static void emit_optional_meta(thinkthen_optional_meta_v1 v) { if(v.present) { emit_meta(v.value); } else fputs("null",stdout); }
static void emit_optional_probabilities(thinkthen_optional_probabilities_v1 v) { if(v.present) { emit_probabilities(v.value); } else fputs("null",stdout); }
static void emit_optional_profile_warning(thinkthen_optional_profile_warning_v1 v) { if(v.present) { emit_profile_warning(v.value); } else fputs("null",stdout); }
static void emit_optional_question(thinkthen_optional_question_v1 v) { if(v.present) { emit_question_view(v.value); } else fputs("null",stdout); }
static void emit_optional_rule(thinkthen_optional_rule_v1 v) { if(v.present) { emit_rule(v.value); } else fputs("null",stdout); }
static void emit_optional_size(thinkthen_optional_size_v1 v) { if(v.present) { printf("%llu",(unsigned long long)v.value); } else fputs("null",stdout); }
static void emit_optional_source_entity_edges(thinkthen_optional_source_entity_edges_v1 v) { if(v.present) { emit_source_entity_edges(v.value); } else fputs("null",stdout); }
static void emit_optional_stopped(thinkthen_optional_stopped_v1 v) { if(v.present) { emit_stopped(v.value); } else fputs("null",stdout); }
static void emit_optional_string(thinkthen_optional_string_v1 v) { if(v.present) { emit_string(v.value); } else fputs("null",stdout); }
static void emit_optional_u16(thinkthen_optional_u16_v1 v) { if(v.present) { printf("%llu",(unsigned long long)v.value); } else fputs("null",stdout); }
static void emit_optional_u64(thinkthen_optional_u64_v1 v) { if(v.present) { printf("%llu",(unsigned long long)v.value); } else fputs("null",stdout); }
static void emit_optional_usage(thinkthen_optional_usage_v1 v) { if(v.present) { emit_usage(v.value); } else fputs("null",stdout); }
static void emit_pair(thinkthen_pair_v1 v) { putchar(123); fputs("\"relation\":",stdout); emit_string(v.relation); putchar(44); fputs("\"source\":",stdout); emit_place(v.source); putchar(44); fputs("\"target\":",stdout); emit_place(v.target); putchar(44); fputs("\"probability\":",stdout); printf("%.17g",v.probability); putchar(125); }
static void emit_pairs(thinkthen_pairs_v1 v) { putchar(91); for(size_t i=0;i<v.len;++i) { if(i) putchar(44); emit_pair(v.data[i]); } putchar(93); }
static void emit_piece(thinkthen_piece_v1 v) { putchar(123); fputs("\"start\":",stdout); printf("%llu",(unsigned long long)v.start); putchar(44); fputs("\"end\":",stdout); printf("%llu",(unsigned long long)v.end); putchar(44); fputs("\"tags\":",stdout); emit_probabilities(v.tags); putchar(125); }
static void emit_pieces(thinkthen_pieces_v1 v) { putchar(91); for(size_t i=0;i<v.len;++i) { if(i) putchar(44); emit_piece(v.data[i]); } putchar(93); }
static void emit_place(thinkthen_place_v1 v) { putchar(123); fputs("\"start\":",stdout); printf("%llu",(unsigned long long)v.start); putchar(44); fputs("\"end\":",stdout); printf("%llu",(unsigned long long)v.end); putchar(125); }
static void emit_probabilities(thinkthen_probabilities_v1 v) { putchar(91); for(size_t i=0;i<v.len;++i) { if(i) putchar(44); emit_probability(v.data[i]); } putchar(93); }
static void emit_probability(thinkthen_probability_v1 v) { putchar(123); fputs("\"name\":",stdout); emit_string(v.name); putchar(44); fputs("\"probability\":",stdout); printf("%.17g",v.probability); putchar(125); }
static void emit_profile_warning(thinkthen_profile_warning_v1 v) { putchar(123); fputs("\"tuned_for\":",stdout); emit_string(v.tuned_for); putchar(44); fputs("\"running\":",stdout); emit_string(v.running); putchar(125); }
static void emit_question_author(thinkthen_question_author_v1 v) { putchar(123); fputs("\"name\":",stdout); emit_optional_string(v.name); putchar(44); fputs("\"wording_version\":",stdout); emit_optional_u64(v.wording_version); putchar(44); fputs("\"item_schema\":",stdout); emit_input_declaration(v.item_schema); putchar(44); fputs("\"context_schema\":",stdout); emit_input_declaration(v.context_schema); putchar(125); }
static void emit_question_member(thinkthen_question_member_v1 v) { putchar(123); fputs("\"name\":",stdout); emit_string(v.name); putchar(44); fputs("\"question\":",stdout); emit_question_view(*v.question); putchar(125); }
static void emit_question_members(thinkthen_question_members_v1 v) { putchar(91); for(size_t i=0;i<v.len;++i) { if(i) putchar(44); emit_question_member(v.data[i]); } putchar(93); }
static void emit_question_observation(thinkthen_question_observation_v1 v) { putchar(123); fputs("\"index\":",stdout); printf("%llu",(unsigned long long)v.index); putchar(44); fputs("\"member\":",stdout); emit_optional_string(v.member); putchar(44); fputs("\"stage\":",stdout); emit_optional_discriminator(v.stage); putchar(44); fputs("\"position\":",stdout); printf("%llu",(unsigned long long)v.position); putchar(44); fputs("\"question_sha256\":",stdout); emit_string(v.question_sha256); putchar(44); fputs("\"model\":",stdout); emit_string(v.model); putchar(44); fputs("\"url\":",stdout); emit_string(v.url); putchar(44); fputs("\"requests\":",stdout); emit_strings(v.requests); putchar(44); fputs("\"requests_sent\":",stdout); printf("%llu",(unsigned long long)v.requests_sent); putchar(44); fputs("\"cached\":",stdout); printf("%llu",(unsigned long long)v.cached); putchar(44); fputs("\"failed_questions\":",stdout); printf("%llu",(unsigned long long)v.failed_questions); putchar(44); fputs("\"usage\":",stdout); emit_optional_usage(v.usage); putchar(44); fputs("\"question_sources\":",stdout); emit_question_sources(v.question_sources); putchar(44); fputs("\"state\":",stdout); printf("%llu",(unsigned long long)v.state); putchar(44); fputs("\"data\":",stdout); putchar(123); fputs("\"success\":",stdout); if(v.state==1) { emit_observation_success(v.data.success); } else fputs("null",stdout); putchar(44); fputs("\"failure\":",stdout); if(v.state==2) { emit_member_failure(v.data.failure); } else fputs("null",stdout); putchar(125); putchar(125); }
static void emit_question_source(thinkthen_question_source_v1 v) { putchar(123); fputs("\"origin\":",stdout); printf("%llu",(unsigned long long)v.origin); putchar(44); fputs("\"answered_by\":",stdout); emit_string(v.answered_by); putchar(125); }
static void emit_question_sources(thinkthen_question_sources_v1 v) { putchar(91); for(size_t i=0;i<v.len;++i) { if(i) putchar(44); emit_question_source(v.data[i]); } putchar(93); }
static void emit_question_view(thinkthen_question_view_v1 v) { putchar(123); fputs("\"kind\":",stdout); printf("%llu",(unsigned long long)v.kind); putchar(44); fputs("\"text\":",stdout); emit_content(v.text); putchar(44); fputs("\"yes\":",stdout); emit_optional_content(v.yes); putchar(44); fputs("\"no\":",stdout); emit_optional_content(v.no); putchar(44); fputs("\"choices\":",stdout); emit_choices(v.choices); putchar(44); fputs("\"threshold\":",stdout); emit_rule(v.threshold); putchar(44); fputs("\"relation_threshold\":",stdout); emit_rule(v.relation_threshold); putchar(44); fputs("\"model\":",stdout); emit_optional_string(v.model); putchar(44); fputs("\"profile\":",stdout); emit_optional_string(v.profile); putchar(44); fputs("\"batch\":",stdout); emit_optional_size(v.batch); putchar(44); fputs("\"batch_max\":",stdout); printf("%llu",(unsigned long long)v.batch_max); putchar(44); fputs("\"none\":",stdout); printf("%llu",(unsigned long long)v.none); putchar(44); fputs("\"on\":",stdout); emit_strings(v.on); putchar(44); fputs("\"members\":",stdout); emit_question_members(v.members); putchar(44); fputs("\"kinds\":",stdout); emit_choices(v.kinds); putchar(44); fputs("\"relations\":",stdout); emit_relations(v.relations); putchar(44); fputs("\"name_pointer\":",stdout); emit_optional_string(v.name_pointer); putchar(44); fputs("\"kind_pointer\":",stdout); emit_optional_string(v.kind_pointer); putchar(125); }
static void emit_rank_view(thinkthen_rank_view_v1 v) { putchar(123); fputs("\"common\":",stdout); emit_row(v.common); putchar(44); fputs("\"value\":",stdout); emit_optional_size(v.value); putchar(44); fputs("\"question_name\":",stdout); emit_optional_string(v.question_name); putchar(125); }
static void emit_recognize_answer(thinkthen_recognize_answer_v1 v) { putchar(123); fputs("\"pieces\":",stdout); emit_pieces(v.pieces); putchar(44); fputs("\"names\":",stdout); emit_names(v.names); putchar(44); fputs("\"pairs\":",stdout); emit_pairs(v.pairs); putchar(125); }
static void emit_recognize_value(thinkthen_recognize_value_v1 v) { putchar(123); fputs("\"entities\":",stdout); emit_entities(v.entities); putchar(44); fputs("\"relations\":",stdout); emit_optional_entity_edges(v.relations); putchar(125); }
static void emit_recognize_view(thinkthen_recognize_view_v1 v) { putchar(123); fputs("\"common\":",stdout); emit_row(v.common); putchar(44); fputs("\"value\":",stdout); emit_recognize_value(v.value); putchar(44); fputs("\"answer\":",stdout); emit_recognize_answer(v.answer); putchar(125); }
static void emit_relate_view(thinkthen_relate_view_v1 v) { putchar(123); fputs("\"common\":",stdout); emit_row(v.common); putchar(44); fputs("\"value\":",stdout); emit_edges(v.value); putchar(44); fputs("\"questions\":",stdout); emit_relation_answers(v.questions); putchar(125); }
static void emit_relation_answer(thinkthen_relation_answer_v1 v) { putchar(123); fputs("\"relation\":",stdout); emit_string(v.relation); putchar(44); fputs("\"reads\":",stdout); emit_string(v.reads); putchar(44); fputs("\"method\":",stdout); printf("%llu",(unsigned long long)v.method); putchar(44); fputs("\"direction\":",stdout); printf("%llu",(unsigned long long)v.direction); putchar(44); fputs("\"source\":",stdout); emit_endpoint(v.source); putchar(44); fputs("\"target\":",stdout); emit_optional_endpoint(v.target); putchar(44); fputs("\"request\":",stdout); emit_string(v.request); putchar(44); fputs("\"state\":",stdout); printf("%llu",(unsigned long long)v.state); putchar(44); fputs("\"data\":",stdout); putchar(123); fputs("\"success\":",stdout); if(v.state==1) { emit_relation_success(v.data.success); } else fputs("null",stdout); putchar(44); fputs("\"failure\":",stdout); if(v.state==2) { emit_member_failure(v.data.failure); } else fputs("null",stdout); putchar(125); putchar(125); }
static void emit_relation_answers(thinkthen_relation_answers_v1 v) { putchar(91); for(size_t i=0;i<v.len;++i) { if(i) putchar(44); emit_relation_answer(v.data[i]); } putchar(93); }
static void emit_relation_success(thinkthen_relation_success_v1 v) { putchar(123); fputs("\"answer_id\":",stdout); emit_string(v.answer_id); putchar(44); fputs("\"probability\":",stdout); printf("%.17g",v.probability); putchar(44); fputs("\"accepted\":",stdout); printf("%llu",(unsigned long long)v.accepted); putchar(125); }
static void emit_relation(thinkthen_relation_v1 v) { putchar(123); fputs("\"name\":",stdout); emit_string(v.name); putchar(44); fputs("\"source\":",stdout); emit_string(v.source); putchar(44); fputs("\"target\":",stdout); emit_string(v.target); putchar(44); fputs("\"reads\":",stdout); emit_optional_string(v.reads); putchar(44); fputs("\"either\":",stdout); printf("%llu",(unsigned long long)v.either); putchar(44); fputs("\"single\":",stdout); printf("%llu",(unsigned long long)v.single); putchar(125); }
static void emit_relations(thinkthen_relations_v1 v) { putchar(91); for(size_t i=0;i<v.len;++i) { if(i) putchar(44); emit_relation(v.data[i]); } putchar(93); }
static void emit_reported_usage(thinkthen_reported_usage_v1 v) { putchar(123); fputs("\"present\":",stdout); printf("%llu",(unsigned long long)v.present); putchar(44); fputs("\"input_tokens\":",stdout); emit_optional_u64(v.input_tokens); putchar(44); fputs("\"output_tokens\":",stdout); emit_optional_u64(v.output_tokens); putchar(125); }
static void emit_row_observation(thinkthen_row_observation_v1 v) { putchar(123); fputs("\"index\":",stdout); printf("%llu",(unsigned long long)v.index); putchar(44); fputs("\"function\":",stdout); printf("%llu",(unsigned long long)v.function); putchar(44); fputs("\"data\":",stdout); putchar(123); fputs("\"decide\":",stdout); if(v.function==1) { emit_decide_view(v.data.decide); } else fputs("null",stdout); putchar(44); fputs("\"choose\":",stdout); if(v.function==2) { emit_choose_view(v.data.choose); } else fputs("null",stdout); putchar(44); fputs("\"tag\":",stdout); if(v.function==3) { emit_tag_view(v.data.tag); } else fputs("null",stdout); putchar(44); fputs("\"score\":",stdout); if(v.function==4) { emit_score_view(v.data.score); } else fputs("null",stdout); putchar(44); fputs("\"filter\":",stdout); if(v.function==5) { emit_filter_view(v.data.filter); } else fputs("null",stdout); putchar(44); fputs("\"rank\":",stdout); if(v.function==6) { emit_rank_view(v.data.rank); } else fputs("null",stdout); putchar(44); fputs("\"find\":",stdout); if(v.function==7) { emit_find_view(v.data.find); } else fputs("null",stdout); putchar(44); fputs("\"annotate\":",stdout); if(v.function==8) { emit_annotate_view(v.data.annotate); } else fputs("null",stdout); putchar(44); fputs("\"recognize\":",stdout); if(v.function==9) { emit_recognize_view(v.data.recognize); } else fputs("null",stdout); putchar(44); fputs("\"relate\":",stdout); if(v.function==10) { emit_relate_view(v.data.relate); } else fputs("null",stdout); putchar(125); putchar(125); }
static void emit_row(thinkthen_row_v1 v) { putchar(123); fputs("\"answer_id\":",stdout); emit_string(v.answer_id); putchar(44); fputs("\"input\":",stdout); emit_optional_content(v.input); putchar(44); fputs("\"question\":",stdout); emit_optional_question(v.question); putchar(44); fputs("\"answer\":",stdout); emit_optional_answer(v.answer); putchar(44); fputs("\"threshold\":",stdout); emit_optional_rule(v.threshold); putchar(44); fputs("\"position\":",stdout); emit_optional_location(v.position); putchar(44); fputs("\"input_file\":",stdout); emit_optional_string(v.input_file); putchar(44); fputs("\"meta\":",stdout); emit_meta(v.meta); putchar(44); fputs("\"images\":",stdout); emit_optional_image_views(v.images); putchar(125); }
static void emit_rule(thinkthen_rule_v1 v) { putchar(123); fputs("\"kind\":",stdout); printf("%llu",(unsigned long long)v.kind); putchar(44); fputs("\"low\":",stdout); printf("%.17g",v.low); putchar(44); fputs("\"high\":",stdout); printf("%.17g",v.high); putchar(125); }
static void emit_score_answer(thinkthen_score_answer_v1 v) { putchar(123); fputs("\"level\":",stdout); emit_string(v.level); putchar(44); fputs("\"probabilities\":",stdout); emit_probabilities(v.probabilities); putchar(44); fputs("\"confidence\":",stdout); emit_optional_double(v.confidence); putchar(125); }
static void emit_score_view(thinkthen_score_view_v1 v) { putchar(123); fputs("\"common\":",stdout); emit_row(v.common); putchar(44); fputs("\"value\":",stdout); printf("%.17g",v.value); putchar(125); }
static void emit_source_detail(thinkthen_source_detail_v1 v) { putchar(123); fputs("\"origin\":",stdout); printf("%llu",(unsigned long long)v.origin); putchar(44); fputs("\"answered_by\":",stdout); emit_string(v.answered_by); putchar(44); fputs("\"batch_size\":",stdout); emit_optional_size(v.batch_size); putchar(125); }
static void emit_source_details(thinkthen_source_details_v1 v) { putchar(91); for(size_t i=0;i<v.len;++i) { if(i) putchar(44); emit_source_detail(v.data[i]); } putchar(93); }
static void emit_source_edge(thinkthen_source_edge_v1 v) { putchar(123); fputs("\"relation\":",stdout); emit_string(v.relation); putchar(44); fputs("\"source\":",stdout); emit_source_endpoint(v.source); putchar(44); fputs("\"target\":",stdout); emit_source_endpoint(v.target); putchar(44); fputs("\"probability\":",stdout); printf("%.17g",v.probability); putchar(44); fputs("\"either\":",stdout); printf("%llu",(unsigned long long)v.either); putchar(125); }
static void emit_source_edges(thinkthen_source_edges_v1 v) { putchar(91); for(size_t i=0;i<v.len;++i) { if(i) putchar(44); emit_source_edge(v.data[i]); } putchar(93); }
static void emit_source_endpoint(thinkthen_source_endpoint_v1 v) { putchar(123); fputs("\"ordinal\":",stdout); printf("%llu",(unsigned long long)v.ordinal); putchar(44); fputs("\"endpoint\":",stdout); emit_endpoint(v.endpoint); putchar(44); fputs("\"record\":",stdout); emit_content(v.record); putchar(44); fputs("\"position\":",stdout); emit_optional_location(v.position); putchar(125); }
static void emit_source_entities(thinkthen_source_entities_v1 v) { putchar(91); for(size_t i=0;i<v.len;++i) { if(i) putchar(44); emit_source_entity(v.data[i]); } putchar(93); }
static void emit_source_entity_edge(thinkthen_source_entity_edge_v1 v) { putchar(123); fputs("\"relation\":",stdout); emit_string(v.relation); putchar(44); fputs("\"source\":",stdout); emit_source_entity(v.source); putchar(44); fputs("\"target\":",stdout); emit_source_entity(v.target); putchar(44); fputs("\"probability\":",stdout); printf("%.17g",v.probability); putchar(44); fputs("\"either\":",stdout); printf("%llu",(unsigned long long)v.either); putchar(125); }
static void emit_source_entity_edges(thinkthen_source_entity_edges_v1 v) { putchar(91); for(size_t i=0;i<v.len;++i) { if(i) putchar(44); emit_source_entity_edge(v.data[i]); } putchar(93); }
static void emit_source_entity(thinkthen_source_entity_v1 v) { putchar(123); fputs("\"entity\":",stdout); emit_entity(v.entity); putchar(44); fputs("\"position\":",stdout); emit_optional_location(v.position); putchar(125); }
static void emit_source_recognition(thinkthen_source_recognition_v1 v) { putchar(123); fputs("\"present\":",stdout); printf("%llu",(unsigned long long)v.present); putchar(44); fputs("\"entities\":",stdout); emit_source_entities(v.entities); putchar(44); fputs("\"relations\":",stdout); emit_optional_source_entity_edges(v.relations); putchar(125); }
static void emit_source_relations(thinkthen_source_relations_v1 v) { putchar(123); fputs("\"present\":",stdout); printf("%llu",(unsigned long long)v.present); putchar(44); fputs("\"edges\":",stdout); emit_source_edges(v.edges); putchar(125); }
static void emit_stopped(thinkthen_stopped_v1 v) { putchar(123); fputs("\"at\":",stdout); emit_optional_size(v.at); putchar(44); fputs("\"cause\":",stdout); printf("%llu",(unsigned long long)v.cause); putchar(44); fputs("\"status\":",stdout); emit_optional_u16(v.status); putchar(44); fputs("\"retryable\":",stdout); printf("%llu",(unsigned long long)v.retryable); putchar(125); }
static void emit_string(thinkthen_string_v1 v) { quoted(v); }
static void emit_strings(thinkthen_strings_v1 v) { putchar(91); for(size_t i=0;i<v.len;++i) { if(i) putchar(44); emit_string(v.data[i]); } putchar(93); }
static void emit_summary(thinkthen_summary_v1 v) { putchar(123); fputs("\"state\":",stdout); printf("%llu",(unsigned long long)v.state); putchar(44); fputs("\"schema\":",stdout); emit_string(v.schema); putchar(44); fputs("\"answer_id\":",stdout); emit_optional_string(v.answer_id); putchar(44); fputs("\"function\":",stdout); emit_optional_discriminator(v.function); putchar(44); fputs("\"count\":",stdout); printf("%llu",(unsigned long long)v.count); putchar(44); fputs("\"observation_count\":",stdout); printf("%llu",(unsigned long long)v.observation_count); putchar(44); fputs("\"meta\":",stdout); emit_optional_meta(v.meta); putchar(44); fputs("\"facts\":",stdout); emit_optional_facts(v.facts); putchar(44); fputs("\"attempts\":",stdout); emit_optional_attempts(v.attempts); putchar(44); fputs("\"error\":",stdout); emit_optional_error(v.error); putchar(125); }
static void emit_tag_view(thinkthen_tag_view_v1 v) { putchar(123); fputs("\"common\":",stdout); emit_row(v.common); putchar(44); fputs("\"value\":",stdout); emit_strings(v.value); putchar(125); }
static void emit_usage(thinkthen_usage_v1 v) { putchar(123); fputs("\"input_tokens\":",stdout); printf("%llu",(unsigned long long)v.input_tokens); putchar(44); fputs("\"output_tokens\":",stdout); printf("%llu",(unsigned long long)v.output_tokens); putchar(125); }
static void output(TTClient *e,unsigned kind,int code,TTNativeResult *r) { (void)e; (void)kind; (void)code; if(!r) abort(); thinkthen_summary_v1 s={0}; if(tt_native_summary(r,&s)) abort(); fputs("{\"summary\":",stdout); emit_summary(s);
fputs(",\"rows\":[",stdout); for(size_t i=0;i<s.count;++i) { if(i) putchar(44); thinkthen_row_observation_v1 v={0}; v.function=s.function.value; switch(v.function) {
case 1: if(tt_native_decide(r,i,&v.data.decide)) abort(); break;
case 2: if(tt_native_choose(r,i,&v.data.choose)) abort(); break;
case 3: if(tt_native_tag(r,i,&v.data.tag)) abort(); break;
case 4: if(tt_native_score(r,i,&v.data.score)) abort(); break;
case 5: if(tt_native_filter(r,i,&v.data.filter)) abort(); break;
case 6: if(tt_native_rank(r,i,&v.data.rank)) abort(); break;
case 7: if(tt_native_find(r,i,&v.data.find)) abort(); break;
case 8: if(tt_native_annotate(r,i,&v.data.annotate)) abort(); break;
case 9: if(tt_native_recognize(r,i,&v.data.recognize)) abort(); break;
case 10: if(tt_native_relate(r,i,&v.data.relate)) abort(); break;
default:abort(); } emit_row_observation(v); } putchar(93);
fputs(",\"observations\":[",stdout); for(size_t i=0;i<s.observation_count;++i) { if(i) putchar(44); thinkthen_observation_v1 v={0}; if(tt_native_observation(r,i,&v)) abort(); emit_observation(v); } putchar(93);
fputs(",\"details\":[",stdout); for(size_t i=0;i<s.count;++i) { if(i) putchar(44); thinkthen_details_v1 v={0}; if(tt_native_details(r,i,&v)) abort(); emit_details(v); } putchar(93);
fputs(",\"observation_details\":[",stdout); for(size_t i=0;i<s.observation_count;++i) { if(i) putchar(44); thinkthen_details_v1 v={0}; if(tt_native_observation_details(r,i,&v)) abort(); emit_details(v); } putchar(93);
fputs(",\"authors\":[",stdout); for(size_t i=0;i<s.count;++i) { if(i) putchar(44); thinkthen_question_author_v1 v={0}; if(tt_native_author(r,i,&v)) abort(); emit_question_author(v); } putchar(93);
fputs(",\"observation_authors\":[",stdout); for(size_t i=0;i<s.observation_count;++i) { if(i) putchar(44); thinkthen_question_author_v1 v={0}; if(tt_native_observation_author(r,i,&v)) abort(); emit_question_author(v); } putchar(93);
fputs(",\"located_recognition\":[",stdout); for(size_t i=0;i<s.count;++i) { if(i) putchar(44); thinkthen_source_recognition_v1 v={0}; if(s.function.value==9 && tt_native_source_recognition(r,i,&v)) abort(); emit_source_recognition(v); } putchar(93);
fputs(",\"located_relations\":[",stdout); for(size_t i=0;i<s.count;++i) { if(i) putchar(44); thinkthen_source_relations_v1 v={0}; if(s.function.value==10 && tt_native_source_relations(r,i,&v)) abort(); emit_source_relations(v); } putchar(93);
fputs(",\"member_authors\":[",stdout); for(size_t i=0;i<s.count;++i) { if(i) putchar(44); putchar(91); if(s.function.value==8) { thinkthen_annotate_view_v1 v={0}; if(tt_native_annotate(r,i,&v)) abort(); for(size_t j=0;j<v.answers.len;++j) { if(j) putchar(44); thinkthen_question_author_v1 author={0}; if(tt_native_member_author(r,i,j,&author)) abort(); emit_question_author(author); } } putchar(93); } putchar(93);
fputs(",\"rank_members\":[",stdout); for(size_t i=0;i<s.count;++i) { if(i) putchar(44); putchar(91); if(s.function.value==6) { size_t count=0; if(tt_native_rank_member_count(r,i,&count)) abort(); for(size_t j=0;j<count;++j) { if(j) putchar(44); thinkthen_rank_view_v1 v={0}; if(tt_native_rank_member(r,i,j,&v)) abort(); emit_rank_view(v); } } putchar(93); } putchar(93); puts("}"); }
