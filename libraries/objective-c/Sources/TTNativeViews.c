#include "TTNative.h"
#include <stdlib.h>
#include <string.h>
#include <limits.h>
struct TTAllocation { void *data; struct TTAllocation *next; };
struct TTArena { struct TTAllocation *head; int failed; };
static void *allocate(struct TTArena *a,size_t n,size_t width,const void *p) { if(!n) return NULL; if(!p || n>SIZE_MAX/width) { a->failed=1; return NULL; } void *v=calloc(n,width); struct TTAllocation *node=malloc(sizeof(*node)); if(!v || !node) { free(v); free(node); a->failed=1; return NULL; } node->data=v; node->next=a->head; a->head=node; return v; }
static void arena_free(struct TTArena *a) { while(a->head) { struct TTAllocation *n=a->head; a->head=n->next; free(n->data); free(n); } free(a); }
static thinkthen_annotate_view_v1 clone_annotate_view(struct TTArena *a,thinkthen_annotate_view_v1 v);
static thinkthen_answer_v1 clone_answer(struct TTArena *a,thinkthen_answer_v1 v);
static thinkthen_attempt_v1 clone_attempt(struct TTArena *a,thinkthen_attempt_v1 v);
static thinkthen_attempts_v1 clone_attempts(struct TTArena *a,thinkthen_attempts_v1 v);
static thinkthen_batch_v1 clone_batch(struct TTArena *a,thinkthen_batch_v1 v);
static thinkthen_batch_warning_v1 clone_batch_warning(struct TTArena *a,thinkthen_batch_warning_v1 v);
static thinkthen_choice_v1 clone_choice(struct TTArena *a,thinkthen_choice_v1 v);
static thinkthen_choices_v1 clone_choices(struct TTArena *a,thinkthen_choices_v1 v);
static thinkthen_choose_view_v1 clone_choose_view(struct TTArena *a,thinkthen_choose_view_v1 v);
static thinkthen_content_v1 clone_content(struct TTArena *a,thinkthen_content_v1 v);
static thinkthen_decide_value_v1 clone_decide_value(struct TTArena *a,thinkthen_decide_value_v1 v);
static thinkthen_decide_view_v1 clone_decide_view(struct TTArena *a,thinkthen_decide_view_v1 v);
static thinkthen_details_v1 clone_details(struct TTArena *a,thinkthen_details_v1 v);
static thinkthen_edge_v1 clone_edge(struct TTArena *a,thinkthen_edge_v1 v);
static thinkthen_edges_v1 clone_edges(struct TTArena *a,thinkthen_edges_v1 v);
static thinkthen_endpoint_v1 clone_endpoint(struct TTArena *a,thinkthen_endpoint_v1 v);
static thinkthen_entities_v1 clone_entities(struct TTArena *a,thinkthen_entities_v1 v);
static thinkthen_entity_edge_v1 clone_entity_edge(struct TTArena *a,thinkthen_entity_edge_v1 v);
static thinkthen_entity_edges_v1 clone_entity_edges(struct TTArena *a,thinkthen_entity_edges_v1 v);
static thinkthen_entity_v1 clone_entity(struct TTArena *a,thinkthen_entity_v1 v);
static thinkthen_error_v1 clone_error(struct TTArena *a,thinkthen_error_v1 v);
static thinkthen_facts_v1 clone_facts(struct TTArena *a,thinkthen_facts_v1 v);
static thinkthen_filter_view_v1 clone_filter_view(struct TTArena *a,thinkthen_filter_view_v1 v);
static thinkthen_find_view_v1 clone_find_view(struct TTArena *a,thinkthen_find_view_v1 v);
static thinkthen_image_view_v1 clone_image_view(struct TTArena *a,thinkthen_image_view_v1 v);
static thinkthen_image_views_v1 clone_image_views(struct TTArena *a,thinkthen_image_views_v1 v);
static thinkthen_input_declaration_v1 clone_input_declaration(struct TTArena *a,thinkthen_input_declaration_v1 v);
static thinkthen_input_properties_v1 clone_input_properties(struct TTArena *a,thinkthen_input_properties_v1 v);
static thinkthen_input_property_v1 clone_input_property(struct TTArena *a,thinkthen_input_property_v1 v);
static thinkthen_input_view_v1 clone_input_view(struct TTArena *a,thinkthen_input_view_v1 v);
static thinkthen_input_views_v1 clone_input_views(struct TTArena *a,thinkthen_input_views_v1 v);
static thinkthen_location_v1 clone_location(struct TTArena *a,thinkthen_location_v1 v);
static thinkthen_member_failure_v1 clone_member_failure(struct TTArena *a,thinkthen_member_failure_v1 v);
static thinkthen_member_success_v1 clone_member_success(struct TTArena *a,thinkthen_member_success_v1 v);
static thinkthen_member_v1 clone_member(struct TTArena *a,thinkthen_member_v1 v);
static thinkthen_member_value_v1 clone_member_value(struct TTArena *a,thinkthen_member_value_v1 v);
static thinkthen_members_v1 clone_members(struct TTArena *a,thinkthen_members_v1 v);
static thinkthen_meta_v1 clone_meta(struct TTArena *a,thinkthen_meta_v1 v);
static thinkthen_name_v1 clone_name(struct TTArena *a,thinkthen_name_v1 v);
static thinkthen_named_answer_v1 clone_named_answer(struct TTArena *a,thinkthen_named_answer_v1 v);
static thinkthen_names_v1 clone_names(struct TTArena *a,thinkthen_names_v1 v);
static thinkthen_observation_identities_v1 clone_observation_identities(struct TTArena *a,thinkthen_observation_identities_v1 v);
static thinkthen_observation_identity_v1 clone_observation_identity(struct TTArena *a,thinkthen_observation_identity_v1 v);
static thinkthen_observation_success_v1 clone_observation_success(struct TTArena *a,thinkthen_observation_success_v1 v);
static thinkthen_observation_v1 clone_observation(struct TTArena *a,thinkthen_observation_v1 v);
static thinkthen_observed_probabilities_v1 clone_observed_probabilities(struct TTArena *a,thinkthen_observed_probabilities_v1 v);
static thinkthen_optional_answer_v1 clone_optional_answer(struct TTArena *a,thinkthen_optional_answer_v1 v);
static thinkthen_optional_attempts_v1 clone_optional_attempts(struct TTArena *a,thinkthen_optional_attempts_v1 v);
static thinkthen_optional_batch_v1 clone_optional_batch(struct TTArena *a,thinkthen_optional_batch_v1 v);
static thinkthen_optional_batch_warning_v1 clone_optional_batch_warning(struct TTArena *a,thinkthen_optional_batch_warning_v1 v);
static thinkthen_optional_content_v1 clone_optional_content(struct TTArena *a,thinkthen_optional_content_v1 v);
static thinkthen_optional_discriminator_v1 clone_optional_discriminator(struct TTArena *a,thinkthen_optional_discriminator_v1 v);
static thinkthen_optional_double_v1 clone_optional_double(struct TTArena *a,thinkthen_optional_double_v1 v);
static thinkthen_optional_endpoint_v1 clone_optional_endpoint(struct TTArena *a,thinkthen_optional_endpoint_v1 v);
static thinkthen_optional_entity_edges_v1 clone_optional_entity_edges(struct TTArena *a,thinkthen_optional_entity_edges_v1 v);
static thinkthen_optional_error_v1 clone_optional_error(struct TTArena *a,thinkthen_optional_error_v1 v);
static thinkthen_optional_facts_v1 clone_optional_facts(struct TTArena *a,thinkthen_optional_facts_v1 v);
static thinkthen_optional_image_views_v1 clone_optional_image_views(struct TTArena *a,thinkthen_optional_image_views_v1 v);
static thinkthen_optional_location_v1 clone_optional_location(struct TTArena *a,thinkthen_optional_location_v1 v);
static thinkthen_optional_meta_v1 clone_optional_meta(struct TTArena *a,thinkthen_optional_meta_v1 v);
static thinkthen_optional_probabilities_v1 clone_optional_probabilities(struct TTArena *a,thinkthen_optional_probabilities_v1 v);
static thinkthen_optional_profile_warning_v1 clone_optional_profile_warning(struct TTArena *a,thinkthen_optional_profile_warning_v1 v);
static thinkthen_optional_question_v1 clone_optional_question(struct TTArena *a,thinkthen_optional_question_v1 v);
static thinkthen_optional_rule_v1 clone_optional_rule(struct TTArena *a,thinkthen_optional_rule_v1 v);
static thinkthen_optional_size_v1 clone_optional_size(struct TTArena *a,thinkthen_optional_size_v1 v);
static thinkthen_optional_source_entity_edges_v1 clone_optional_source_entity_edges(struct TTArena *a,thinkthen_optional_source_entity_edges_v1 v);
static thinkthen_optional_stopped_v1 clone_optional_stopped(struct TTArena *a,thinkthen_optional_stopped_v1 v);
static thinkthen_optional_string_v1 clone_optional_string(struct TTArena *a,thinkthen_optional_string_v1 v);
static thinkthen_optional_u16_v1 clone_optional_u16(struct TTArena *a,thinkthen_optional_u16_v1 v);
static thinkthen_optional_u64_v1 clone_optional_u64(struct TTArena *a,thinkthen_optional_u64_v1 v);
static thinkthen_optional_usage_v1 clone_optional_usage(struct TTArena *a,thinkthen_optional_usage_v1 v);
static thinkthen_pair_v1 clone_pair(struct TTArena *a,thinkthen_pair_v1 v);
static thinkthen_pairs_v1 clone_pairs(struct TTArena *a,thinkthen_pairs_v1 v);
static thinkthen_piece_v1 clone_piece(struct TTArena *a,thinkthen_piece_v1 v);
static thinkthen_pieces_v1 clone_pieces(struct TTArena *a,thinkthen_pieces_v1 v);
static thinkthen_place_v1 clone_place(struct TTArena *a,thinkthen_place_v1 v);
static thinkthen_probabilities_v1 clone_probabilities(struct TTArena *a,thinkthen_probabilities_v1 v);
static thinkthen_probability_v1 clone_probability(struct TTArena *a,thinkthen_probability_v1 v);
static thinkthen_profile_warning_v1 clone_profile_warning(struct TTArena *a,thinkthen_profile_warning_v1 v);
static thinkthen_question_author_v1 clone_question_author(struct TTArena *a,thinkthen_question_author_v1 v);
static thinkthen_question_member_v1 clone_question_member(struct TTArena *a,thinkthen_question_member_v1 v);
static thinkthen_question_members_v1 clone_question_members(struct TTArena *a,thinkthen_question_members_v1 v);
static thinkthen_question_observation_v1 clone_question_observation(struct TTArena *a,thinkthen_question_observation_v1 v);
static thinkthen_question_source_v1 clone_question_source(struct TTArena *a,thinkthen_question_source_v1 v);
static thinkthen_question_sources_v1 clone_question_sources(struct TTArena *a,thinkthen_question_sources_v1 v);
static thinkthen_question_view_v1 clone_question_view(struct TTArena *a,thinkthen_question_view_v1 v);
static thinkthen_rank_view_v1 clone_rank_view(struct TTArena *a,thinkthen_rank_view_v1 v);
static thinkthen_recognize_answer_v1 clone_recognize_answer(struct TTArena *a,thinkthen_recognize_answer_v1 v);
static thinkthen_recognize_value_v1 clone_recognize_value(struct TTArena *a,thinkthen_recognize_value_v1 v);
static thinkthen_recognize_view_v1 clone_recognize_view(struct TTArena *a,thinkthen_recognize_view_v1 v);
static thinkthen_relate_view_v1 clone_relate_view(struct TTArena *a,thinkthen_relate_view_v1 v);
static thinkthen_relation_answer_v1 clone_relation_answer(struct TTArena *a,thinkthen_relation_answer_v1 v);
static thinkthen_relation_answers_v1 clone_relation_answers(struct TTArena *a,thinkthen_relation_answers_v1 v);
static thinkthen_relation_success_v1 clone_relation_success(struct TTArena *a,thinkthen_relation_success_v1 v);
static thinkthen_relation_v1 clone_relation(struct TTArena *a,thinkthen_relation_v1 v);
static thinkthen_relations_v1 clone_relations(struct TTArena *a,thinkthen_relations_v1 v);
static thinkthen_reported_usage_v1 clone_reported_usage(struct TTArena *a,thinkthen_reported_usage_v1 v);
static thinkthen_row_observation_v1 clone_row_observation(struct TTArena *a,thinkthen_row_observation_v1 v);
static thinkthen_row_v1 clone_row(struct TTArena *a,thinkthen_row_v1 v);
static thinkthen_rule_v1 clone_rule(struct TTArena *a,thinkthen_rule_v1 v);
static thinkthen_score_answer_v1 clone_score_answer(struct TTArena *a,thinkthen_score_answer_v1 v);
static thinkthen_score_view_v1 clone_score_view(struct TTArena *a,thinkthen_score_view_v1 v);
static thinkthen_source_detail_v1 clone_source_detail(struct TTArena *a,thinkthen_source_detail_v1 v);
static thinkthen_source_details_v1 clone_source_details(struct TTArena *a,thinkthen_source_details_v1 v);
static thinkthen_source_edge_v1 clone_source_edge(struct TTArena *a,thinkthen_source_edge_v1 v);
static thinkthen_source_edges_v1 clone_source_edges(struct TTArena *a,thinkthen_source_edges_v1 v);
static thinkthen_source_endpoint_v1 clone_source_endpoint(struct TTArena *a,thinkthen_source_endpoint_v1 v);
static thinkthen_source_entities_v1 clone_source_entities(struct TTArena *a,thinkthen_source_entities_v1 v);
static thinkthen_source_entity_edge_v1 clone_source_entity_edge(struct TTArena *a,thinkthen_source_entity_edge_v1 v);
static thinkthen_source_entity_edges_v1 clone_source_entity_edges(struct TTArena *a,thinkthen_source_entity_edges_v1 v);
static thinkthen_source_entity_v1 clone_source_entity(struct TTArena *a,thinkthen_source_entity_v1 v);
static thinkthen_source_recognition_v1 clone_source_recognition(struct TTArena *a,thinkthen_source_recognition_v1 v);
static thinkthen_source_relations_v1 clone_source_relations(struct TTArena *a,thinkthen_source_relations_v1 v);
static thinkthen_stopped_v1 clone_stopped(struct TTArena *a,thinkthen_stopped_v1 v);
static thinkthen_string_v1 clone_string(struct TTArena *a,thinkthen_string_v1 v);
static thinkthen_strings_v1 clone_strings(struct TTArena *a,thinkthen_strings_v1 v);
static thinkthen_summary_v1 clone_summary(struct TTArena *a,thinkthen_summary_v1 v);
static thinkthen_tag_view_v1 clone_tag_view(struct TTArena *a,thinkthen_tag_view_v1 v);
static thinkthen_usage_v1 clone_usage(struct TTArena *a,thinkthen_usage_v1 v);
static thinkthen_annotate_view_v1 clone_annotate_view(struct TTArena *a,thinkthen_annotate_view_v1 v) { thinkthen_annotate_view_v1 out={0}; (void)a; out.common=clone_row(a,v.common); out.answers=clone_members(a,v.answers); return out; }
static thinkthen_answer_v1 clone_answer(struct TTArena *a,thinkthen_answer_v1 v) { thinkthen_answer_v1 out={0}; (void)a; out.kind=v.kind; switch(v.kind) { case 1: out.data.probability=v.data.probability; break; case 2: out.data.choice=clone_named_answer(a,v.data.choice); break; case 3: out.data.tag=clone_probabilities(a,v.data.tag); break; case 4: out.data.score=clone_score_answer(a,v.data.score); break; case 5: out.data.find=clone_named_answer(a,v.data.find); break; default:a->failed=1; } return out; }
static thinkthen_attempt_v1 clone_attempt(struct TTArena *a,thinkthen_attempt_v1 v) { thinkthen_attempt_v1 out={0}; (void)a; out.ordinal=v.ordinal; out.request_sha256=clone_string(a,v.request_sha256); out.wall_ms=v.wall_ms; out.outcome=v.outcome; out.sdk_request_id=clone_string(a,v.sdk_request_id); out.status=clone_optional_u16(a,v.status); out.server_ms=clone_optional_u64(a,v.server_ms); out.request_id=clone_optional_string(a,v.request_id); return out; }
static thinkthen_attempts_v1 clone_attempts(struct TTArena *a,thinkthen_attempts_v1 v) { thinkthen_attempts_v1 out={0}; thinkthen_attempt_v1 *p=allocate(a,v.len,sizeof(*p),v.data); out.len=v.len; out.data=p; if(p) for(size_t i=0;i<v.len;++i) p[i]=clone_attempt(a,v.data[i]); return out; }
static thinkthen_batch_v1 clone_batch(struct TTArena *a,thinkthen_batch_v1 v) { thinkthen_batch_v1 out={0}; (void)a; out.kind=v.kind; out.records=v.records; return out; }
static thinkthen_batch_warning_v1 clone_batch_warning(struct TTArena *a,thinkthen_batch_warning_v1 v) { thinkthen_batch_warning_v1 out={0}; (void)a; out.tuned_for=clone_batch(a,v.tuned_for); out.running=clone_batch(a,v.running); return out; }
static thinkthen_choice_v1 clone_choice(struct TTArena *a,thinkthen_choice_v1 v) { thinkthen_choice_v1 out={0}; (void)a; out.name=clone_string(a,v.name); out.description=clone_optional_content(a,v.description); out.weight=clone_optional_double(a,v.weight); return out; }
static thinkthen_choices_v1 clone_choices(struct TTArena *a,thinkthen_choices_v1 v) { thinkthen_choices_v1 out={0}; thinkthen_choice_v1 *p=allocate(a,v.len,sizeof(*p),v.data); out.len=v.len; out.data=p; if(p) for(size_t i=0;i<v.len;++i) p[i]=clone_choice(a,v.data[i]); return out; }
static thinkthen_choose_view_v1 clone_choose_view(struct TTArena *a,thinkthen_choose_view_v1 v) { thinkthen_choose_view_v1 out={0}; (void)a; out.common=clone_row(a,v.common); out.value=clone_optional_string(a,v.value); return out; }
static thinkthen_content_v1 clone_content(struct TTArena *a,thinkthen_content_v1 v) { thinkthen_content_v1 out={0}; (void)a; out.kind=v.kind; out.data=clone_string(a,v.data); return out; }
static thinkthen_decide_value_v1 clone_decide_value(struct TTArena *a,thinkthen_decide_value_v1 v) { thinkthen_decide_value_v1 out={0}; (void)a; out.kind=v.kind; switch(v.kind) { case 0: break; case 1: out.data.boolean=v.data.boolean; break; case 2: out.data.authored=clone_content(a,v.data.authored); break; default:a->failed=1; } return out; }
static thinkthen_decide_view_v1 clone_decide_view(struct TTArena *a,thinkthen_decide_view_v1 v) { thinkthen_decide_view_v1 out={0}; (void)a; out.common=clone_row(a,v.common); out.value=clone_decide_value(a,v.value); return out; }
static thinkthen_details_v1 clone_details(struct TTArena *a,thinkthen_details_v1 v) { thinkthen_details_v1 out={0}; (void)a; out.question=clone_optional_question(a,v.question); out.threshold=clone_optional_rule(a,v.threshold); out.raw_pick=clone_optional_string(a,v.raw_pick); out.usage=clone_reported_usage(a,v.usage); out.question_sources=clone_source_details(a,v.question_sources); out.observations=clone_observation_identities(a,v.observations); out.inputs=clone_input_views(a,v.inputs); return out; }
static thinkthen_edge_v1 clone_edge(struct TTArena *a,thinkthen_edge_v1 v) { thinkthen_edge_v1 out={0}; (void)a; out.relation=clone_string(a,v.relation); out.source=clone_endpoint(a,v.source); out.target=clone_endpoint(a,v.target); out.probability=v.probability; out.either=v.either; return out; }
static thinkthen_edges_v1 clone_edges(struct TTArena *a,thinkthen_edges_v1 v) { thinkthen_edges_v1 out={0}; thinkthen_edge_v1 *p=allocate(a,v.len,sizeof(*p),v.data); out.len=v.len; out.data=p; if(p) for(size_t i=0;i<v.len;++i) p[i]=clone_edge(a,v.data[i]); return out; }
static thinkthen_endpoint_v1 clone_endpoint(struct TTArena *a,thinkthen_endpoint_v1 v) { thinkthen_endpoint_v1 out={0}; (void)a; out.name=clone_string(a,v.name); out.kind=clone_string(a,v.kind); return out; }
static thinkthen_entities_v1 clone_entities(struct TTArena *a,thinkthen_entities_v1 v) { thinkthen_entities_v1 out={0}; thinkthen_entity_v1 *p=allocate(a,v.len,sizeof(*p),v.data); out.len=v.len; out.data=p; if(p) for(size_t i=0;i<v.len;++i) p[i]=clone_entity(a,v.data[i]); return out; }
static thinkthen_entity_edge_v1 clone_entity_edge(struct TTArena *a,thinkthen_entity_edge_v1 v) { thinkthen_entity_edge_v1 out={0}; (void)a; out.relation=clone_string(a,v.relation); out.source=clone_entity(a,v.source); out.target=clone_entity(a,v.target); out.probability=v.probability; out.either=v.either; return out; }
static thinkthen_entity_edges_v1 clone_entity_edges(struct TTArena *a,thinkthen_entity_edges_v1 v) { thinkthen_entity_edges_v1 out={0}; thinkthen_entity_edge_v1 *p=allocate(a,v.len,sizeof(*p),v.data); out.len=v.len; out.data=p; if(p) for(size_t i=0;i<v.len;++i) p[i]=clone_entity_edge(a,v.data[i]); return out; }
static thinkthen_entity_v1 clone_entity(struct TTArena *a,thinkthen_entity_v1 v) { thinkthen_entity_v1 out={0}; (void)a; out.text=clone_string(a,v.text); out.start=v.start; out.end=v.end; out.length=v.length; out.kind=clone_string(a,v.kind); out.strength=v.strength; return out; }
static thinkthen_error_v1 clone_error(struct TTArena *a,thinkthen_error_v1 v) { thinkthen_error_v1 out={0}; (void)a; out.code=v.code; out.message=clone_string(a,v.message); out.retryable=v.retryable; out.stopped=clone_optional_stopped(a,v.stopped); return out; }
static thinkthen_facts_v1 clone_facts(struct TTArena *a,thinkthen_facts_v1 v) { thinkthen_facts_v1 out={0}; (void)a; out.call_id=clone_string(a,v.call_id); out.cache_answers=v.cache_answers; out.estimated_cost_usd=clone_optional_string(a,v.estimated_cost_usd); out.input_tokens=clone_optional_u64(a,v.input_tokens); out.model=clone_optional_string(a,v.model); out.output_tokens=clone_optional_u64(a,v.output_tokens); out.records=v.records; out.requests_sent=v.requests_sent; out.seconds=v.seconds; out.command_ms=clone_optional_u64(a,v.command_ms); return out; }
static thinkthen_filter_view_v1 clone_filter_view(struct TTArena *a,thinkthen_filter_view_v1 v) { thinkthen_filter_view_v1 out={0}; (void)a; out.common=clone_row(a,v.common); out.value=v.value; return out; }
static thinkthen_find_view_v1 clone_find_view(struct TTArena *a,thinkthen_find_view_v1 v) { thinkthen_find_view_v1 out={0}; (void)a; out.common=clone_row(a,v.common); out.value=clone_optional_content(a,v.value); out.index=clone_optional_size(a,v.index); return out; }
static thinkthen_image_view_v1 clone_image_view(struct TTArena *a,thinkthen_image_view_v1 v) { thinkthen_image_view_v1 out={0}; (void)a; out.media=v.media; uint8_t *p=allocate(a,v.bytes_len,1,v.bytes); if(p) memcpy(p,v.bytes,v.bytes_len); out.bytes=p; out.bytes_len=v.bytes_len; out.width=v.width; out.height=v.height; out.filename=clone_optional_string(a,v.filename); return out; }
static thinkthen_image_views_v1 clone_image_views(struct TTArena *a,thinkthen_image_views_v1 v) { thinkthen_image_views_v1 out={0}; thinkthen_image_view_v1 *p=allocate(a,v.len,sizeof(*p),v.data); out.len=v.len; out.data=p; if(p) for(size_t i=0;i<v.len;++i) p[i]=clone_image_view(a,v.data[i]); return out; }
static thinkthen_input_declaration_v1 clone_input_declaration(struct TTArena *a,thinkthen_input_declaration_v1 v) { thinkthen_input_declaration_v1 out={0}; (void)a; out.kind=v.kind; out.properties=clone_input_properties(a,v.properties); out.required=clone_strings(a,v.required); return out; }
static thinkthen_input_properties_v1 clone_input_properties(struct TTArena *a,thinkthen_input_properties_v1 v) { thinkthen_input_properties_v1 out={0}; thinkthen_input_property_v1 *p=allocate(a,v.len,sizeof(*p),v.data); out.len=v.len; out.data=p; if(p) for(size_t i=0;i<v.len;++i) p[i]=clone_input_property(a,v.data[i]); return out; }
static thinkthen_input_property_v1 clone_input_property(struct TTArena *a,thinkthen_input_property_v1 v) { thinkthen_input_property_v1 out={0}; (void)a; out.name=clone_string(a,v.name); out.kind=v.kind; return out; }
static thinkthen_input_view_v1 clone_input_view(struct TTArena *a,thinkthen_input_view_v1 v) { thinkthen_input_view_v1 out={0}; (void)a; out.original=clone_optional_content(a,v.original); out.position=clone_optional_location(a,v.position); out.images=clone_optional_image_views(a,v.images); return out; }
static thinkthen_input_views_v1 clone_input_views(struct TTArena *a,thinkthen_input_views_v1 v) { thinkthen_input_views_v1 out={0}; thinkthen_input_view_v1 *p=allocate(a,v.len,sizeof(*p),v.data); out.len=v.len; out.data=p; if(p) for(size_t i=0;i<v.len;++i) p[i]=clone_input_view(a,v.data[i]); return out; }
static thinkthen_location_v1 clone_location(struct TTArena *a,thinkthen_location_v1 v) { thinkthen_location_v1 out={0}; (void)a; out.file=clone_optional_string(a,v.file); out.first_line=clone_optional_size(a,v.first_line); out.last_line=clone_optional_size(a,v.last_line); return out; }
static thinkthen_member_failure_v1 clone_member_failure(struct TTArena *a,thinkthen_member_failure_v1 v) { thinkthen_member_failure_v1 out={0}; (void)a; out.failure_id=clone_string(a,v.failure_id); out.cause=v.cause; return out; }
static thinkthen_member_success_v1 clone_member_success(struct TTArena *a,thinkthen_member_success_v1 v) { thinkthen_member_success_v1 out={0}; (void)a; out.answer_id=clone_string(a,v.answer_id); out.value=clone_member_value(a,v.value); out.answer=clone_answer(a,v.answer); out.threshold=clone_rule(a,v.threshold); return out; }
static thinkthen_member_v1 clone_member(struct TTArena *a,thinkthen_member_v1 v) { thinkthen_member_v1 out={0}; (void)a; out.name=clone_string(a,v.name); out.request=clone_string(a,v.request); out.question=clone_question_view(a,v.question); out.state=v.state; switch(v.state) { case 1: out.data.success=clone_member_success(a,v.data.success); break; case 2: out.data.failure=clone_member_failure(a,v.data.failure); break; default:a->failed=1; } return out; }
static thinkthen_member_value_v1 clone_member_value(struct TTArena *a,thinkthen_member_value_v1 v) { thinkthen_member_value_v1 out={0}; (void)a; out.kind=v.kind; switch(v.kind) { case 1: out.data.decide=clone_decide_value(a,v.data.decide); break; case 2: out.data.choose=clone_optional_string(a,v.data.choose); break; case 3: out.data.tag=clone_strings(a,v.data.tag); break; case 4: out.data.score=v.data.score; break; default:a->failed=1; } return out; }
static thinkthen_members_v1 clone_members(struct TTArena *a,thinkthen_members_v1 v) { thinkthen_members_v1 out={0}; thinkthen_member_v1 *p=allocate(a,v.len,sizeof(*p),v.data); out.len=v.len; out.data=p; if(p) for(size_t i=0;i<v.len;++i) p[i]=clone_member(a,v.data[i]); return out; }
static thinkthen_meta_v1 clone_meta(struct TTArena *a,thinkthen_meta_v1 v) { thinkthen_meta_v1 out={0}; (void)a; out.tool=clone_string(a,v.tool); out.question_sha256=clone_optional_string(a,v.question_sha256); out.questions_sha256=clone_optional_string(a,v.questions_sha256); out.url=clone_string(a,v.url); out.model=clone_string(a,v.model); out.usage=clone_optional_usage(a,v.usage); out.requests_sent=v.requests_sent; out.cached=v.cached; out.requests=clone_strings(a,v.requests); out.failed_questions=v.failed_questions; out.profile_warning=clone_optional_profile_warning(a,v.profile_warning); out.batch_setting=clone_optional_batch(a,v.batch_setting); out.batch_warning=clone_optional_batch_warning(a,v.batch_warning); out.context_sha256=clone_optional_string(a,v.context_sha256); out.attempts=clone_optional_attempts(a,v.attempts); out.origin=clone_optional_discriminator(a,v.origin); out.question_sources=clone_question_sources(a,v.question_sources); out.observations=clone_observation_identities(a,v.observations); out.answered_by=clone_optional_string(a,v.answered_by); return out; }
static thinkthen_name_v1 clone_name(struct TTArena *a,thinkthen_name_v1 v) { thinkthen_name_v1 out={0}; (void)a; out.start=v.start; out.end=v.end; out.kinds=clone_optional_probabilities(a,v.kinds); out.edges=clone_optional_probabilities(a,v.edges); return out; }
static thinkthen_named_answer_v1 clone_named_answer(struct TTArena *a,thinkthen_named_answer_v1 v) { thinkthen_named_answer_v1 out={0}; (void)a; out.pick=clone_string(a,v.pick); out.probabilities=clone_probabilities(a,v.probabilities); out.confidence=clone_optional_double(a,v.confidence); return out; }
static thinkthen_names_v1 clone_names(struct TTArena *a,thinkthen_names_v1 v) { thinkthen_names_v1 out={0}; thinkthen_name_v1 *p=allocate(a,v.len,sizeof(*p),v.data); out.len=v.len; out.data=p; if(p) for(size_t i=0;i<v.len;++i) p[i]=clone_name(a,v.data[i]); return out; }
static thinkthen_observation_identities_v1 clone_observation_identities(struct TTArena *a,thinkthen_observation_identities_v1 v) { thinkthen_observation_identities_v1 out={0}; thinkthen_observation_identity_v1 *p=allocate(a,v.len,sizeof(*p),v.data); out.len=v.len; out.data=p; if(p) for(size_t i=0;i<v.len;++i) p[i]=clone_observation_identity(a,v.data[i]); return out; }
static thinkthen_observation_identity_v1 clone_observation_identity(struct TTArena *a,thinkthen_observation_identity_v1 v) { thinkthen_observation_identity_v1 out={0}; (void)a; out.kind=v.kind; switch(v.kind) { case 1: out.data.observation_id=clone_string(a,v.data.observation_id); break; case 2: out.data.failure_id=clone_string(a,v.data.failure_id); break; default:a->failed=1; } return out; }
static thinkthen_observation_success_v1 clone_observation_success(struct TTArena *a,thinkthen_observation_success_v1 v) { thinkthen_observation_success_v1 out={0}; (void)a; out.answer_id=clone_string(a,v.answer_id); out.observation_id=clone_string(a,v.observation_id); out.value=clone_member_value(a,v.value); out.probabilities=clone_observed_probabilities(a,v.probabilities); out.confidence=clone_optional_double(a,v.confidence); return out; }
static thinkthen_observation_v1 clone_observation(struct TTArena *a,thinkthen_observation_v1 v) { thinkthen_observation_v1 out={0}; (void)a; out.kind=v.kind; switch(v.kind) { case 1: out.data.question=clone_question_observation(a,v.data.question); break; case 2: out.data.row=clone_row_observation(a,v.data.row); break; default:a->failed=1; } return out; }
static thinkthen_observed_probabilities_v1 clone_observed_probabilities(struct TTArena *a,thinkthen_observed_probabilities_v1 v) { thinkthen_observed_probabilities_v1 out={0}; (void)a; out.kind=v.kind; switch(v.kind) { case 1: out.data.yes=v.data.yes; break; case 2: out.data.named=clone_probabilities(a,v.data.named); break; default:a->failed=1; } return out; }
static thinkthen_optional_answer_v1 clone_optional_answer(struct TTArena *a,thinkthen_optional_answer_v1 v) { thinkthen_optional_answer_v1 out={0}; if(v.present!=0 && v.present!=1) a->failed=1; else if(v.present) { out.present=1; out.value=clone_answer(a,v.value); } return out; }
static thinkthen_optional_attempts_v1 clone_optional_attempts(struct TTArena *a,thinkthen_optional_attempts_v1 v) { thinkthen_optional_attempts_v1 out={0}; if(v.present!=0 && v.present!=1) a->failed=1; else if(v.present) { out.present=1; out.value=clone_attempts(a,v.value); } return out; }
static thinkthen_optional_batch_v1 clone_optional_batch(struct TTArena *a,thinkthen_optional_batch_v1 v) { thinkthen_optional_batch_v1 out={0}; if(v.present!=0 && v.present!=1) a->failed=1; else if(v.present) { out.present=1; out.value=clone_batch(a,v.value); } return out; }
static thinkthen_optional_batch_warning_v1 clone_optional_batch_warning(struct TTArena *a,thinkthen_optional_batch_warning_v1 v) { thinkthen_optional_batch_warning_v1 out={0}; if(v.present!=0 && v.present!=1) a->failed=1; else if(v.present) { out.present=1; out.value=clone_batch_warning(a,v.value); } return out; }
static thinkthen_optional_content_v1 clone_optional_content(struct TTArena *a,thinkthen_optional_content_v1 v) { thinkthen_optional_content_v1 out={0}; if(v.present!=0 && v.present!=1) a->failed=1; else if(v.present) { out.present=1; out.value=clone_content(a,v.value); } return out; }
static thinkthen_optional_discriminator_v1 clone_optional_discriminator(struct TTArena *a,thinkthen_optional_discriminator_v1 v) { thinkthen_optional_discriminator_v1 out={0}; if(v.present!=0 && v.present!=1) a->failed=1; else if(v.present) { out.present=1; out.value=v.value; } return out; }
static thinkthen_optional_double_v1 clone_optional_double(struct TTArena *a,thinkthen_optional_double_v1 v) { thinkthen_optional_double_v1 out={0}; if(v.present!=0 && v.present!=1) a->failed=1; else if(v.present) { out.present=1; out.value=v.value; } return out; }
static thinkthen_optional_endpoint_v1 clone_optional_endpoint(struct TTArena *a,thinkthen_optional_endpoint_v1 v) { thinkthen_optional_endpoint_v1 out={0}; if(v.present!=0 && v.present!=1) a->failed=1; else if(v.present) { out.present=1; out.value=clone_endpoint(a,v.value); } return out; }
static thinkthen_optional_entity_edges_v1 clone_optional_entity_edges(struct TTArena *a,thinkthen_optional_entity_edges_v1 v) { thinkthen_optional_entity_edges_v1 out={0}; if(v.present!=0 && v.present!=1) a->failed=1; else if(v.present) { out.present=1; out.value=clone_entity_edges(a,v.value); } return out; }
static thinkthen_optional_error_v1 clone_optional_error(struct TTArena *a,thinkthen_optional_error_v1 v) { thinkthen_optional_error_v1 out={0}; if(v.present!=0 && v.present!=1) a->failed=1; else if(v.present) { out.present=1; out.value=clone_error(a,v.value); } return out; }
static thinkthen_optional_facts_v1 clone_optional_facts(struct TTArena *a,thinkthen_optional_facts_v1 v) { thinkthen_optional_facts_v1 out={0}; if(v.present!=0 && v.present!=1) a->failed=1; else if(v.present) { out.present=1; out.value=clone_facts(a,v.value); } return out; }
static thinkthen_optional_image_views_v1 clone_optional_image_views(struct TTArena *a,thinkthen_optional_image_views_v1 v) { thinkthen_optional_image_views_v1 out={0}; if(v.present!=0 && v.present!=1) a->failed=1; else if(v.present) { out.present=1; out.value=clone_image_views(a,v.value); } return out; }
static thinkthen_optional_location_v1 clone_optional_location(struct TTArena *a,thinkthen_optional_location_v1 v) { thinkthen_optional_location_v1 out={0}; if(v.present!=0 && v.present!=1) a->failed=1; else if(v.present) { out.present=1; out.value=clone_location(a,v.value); } return out; }
static thinkthen_optional_meta_v1 clone_optional_meta(struct TTArena *a,thinkthen_optional_meta_v1 v) { thinkthen_optional_meta_v1 out={0}; if(v.present!=0 && v.present!=1) a->failed=1; else if(v.present) { out.present=1; out.value=clone_meta(a,v.value); } return out; }
static thinkthen_optional_probabilities_v1 clone_optional_probabilities(struct TTArena *a,thinkthen_optional_probabilities_v1 v) { thinkthen_optional_probabilities_v1 out={0}; if(v.present!=0 && v.present!=1) a->failed=1; else if(v.present) { out.present=1; out.value=clone_probabilities(a,v.value); } return out; }
static thinkthen_optional_profile_warning_v1 clone_optional_profile_warning(struct TTArena *a,thinkthen_optional_profile_warning_v1 v) { thinkthen_optional_profile_warning_v1 out={0}; if(v.present!=0 && v.present!=1) a->failed=1; else if(v.present) { out.present=1; out.value=clone_profile_warning(a,v.value); } return out; }
static thinkthen_optional_question_v1 clone_optional_question(struct TTArena *a,thinkthen_optional_question_v1 v) { thinkthen_optional_question_v1 out={0}; if(v.present!=0 && v.present!=1) a->failed=1; else if(v.present) { out.present=1; out.value=clone_question_view(a,v.value); } return out; }
static thinkthen_optional_rule_v1 clone_optional_rule(struct TTArena *a,thinkthen_optional_rule_v1 v) { thinkthen_optional_rule_v1 out={0}; if(v.present!=0 && v.present!=1) a->failed=1; else if(v.present) { out.present=1; out.value=clone_rule(a,v.value); } return out; }
static thinkthen_optional_size_v1 clone_optional_size(struct TTArena *a,thinkthen_optional_size_v1 v) { thinkthen_optional_size_v1 out={0}; if(v.present!=0 && v.present!=1) a->failed=1; else if(v.present) { out.present=1; out.value=v.value; } return out; }
static thinkthen_optional_source_entity_edges_v1 clone_optional_source_entity_edges(struct TTArena *a,thinkthen_optional_source_entity_edges_v1 v) { thinkthen_optional_source_entity_edges_v1 out={0}; if(v.present!=0 && v.present!=1) a->failed=1; else if(v.present) { out.present=1; out.value=clone_source_entity_edges(a,v.value); } return out; }
static thinkthen_optional_stopped_v1 clone_optional_stopped(struct TTArena *a,thinkthen_optional_stopped_v1 v) { thinkthen_optional_stopped_v1 out={0}; if(v.present!=0 && v.present!=1) a->failed=1; else if(v.present) { out.present=1; out.value=clone_stopped(a,v.value); } return out; }
static thinkthen_optional_string_v1 clone_optional_string(struct TTArena *a,thinkthen_optional_string_v1 v) { thinkthen_optional_string_v1 out={0}; if(v.present!=0 && v.present!=1) a->failed=1; else if(v.present) { out.present=1; out.value=clone_string(a,v.value); } return out; }
static thinkthen_optional_u16_v1 clone_optional_u16(struct TTArena *a,thinkthen_optional_u16_v1 v) { thinkthen_optional_u16_v1 out={0}; if(v.present!=0 && v.present!=1) a->failed=1; else if(v.present) { out.present=1; out.value=v.value; } return out; }
static thinkthen_optional_u64_v1 clone_optional_u64(struct TTArena *a,thinkthen_optional_u64_v1 v) { thinkthen_optional_u64_v1 out={0}; if(v.present!=0 && v.present!=1) a->failed=1; else if(v.present) { out.present=1; out.value=v.value; } return out; }
static thinkthen_optional_usage_v1 clone_optional_usage(struct TTArena *a,thinkthen_optional_usage_v1 v) { thinkthen_optional_usage_v1 out={0}; if(v.present!=0 && v.present!=1) a->failed=1; else if(v.present) { out.present=1; out.value=clone_usage(a,v.value); } return out; }
static thinkthen_pair_v1 clone_pair(struct TTArena *a,thinkthen_pair_v1 v) { thinkthen_pair_v1 out={0}; (void)a; out.relation=clone_string(a,v.relation); out.source=clone_place(a,v.source); out.target=clone_place(a,v.target); out.probability=v.probability; return out; }
static thinkthen_pairs_v1 clone_pairs(struct TTArena *a,thinkthen_pairs_v1 v) { thinkthen_pairs_v1 out={0}; thinkthen_pair_v1 *p=allocate(a,v.len,sizeof(*p),v.data); out.len=v.len; out.data=p; if(p) for(size_t i=0;i<v.len;++i) p[i]=clone_pair(a,v.data[i]); return out; }
static thinkthen_piece_v1 clone_piece(struct TTArena *a,thinkthen_piece_v1 v) { thinkthen_piece_v1 out={0}; (void)a; out.start=v.start; out.end=v.end; out.tags=clone_probabilities(a,v.tags); return out; }
static thinkthen_pieces_v1 clone_pieces(struct TTArena *a,thinkthen_pieces_v1 v) { thinkthen_pieces_v1 out={0}; thinkthen_piece_v1 *p=allocate(a,v.len,sizeof(*p),v.data); out.len=v.len; out.data=p; if(p) for(size_t i=0;i<v.len;++i) p[i]=clone_piece(a,v.data[i]); return out; }
static thinkthen_place_v1 clone_place(struct TTArena *a,thinkthen_place_v1 v) { thinkthen_place_v1 out={0}; (void)a; out.start=v.start; out.end=v.end; return out; }
static thinkthen_probabilities_v1 clone_probabilities(struct TTArena *a,thinkthen_probabilities_v1 v) { thinkthen_probabilities_v1 out={0}; thinkthen_probability_v1 *p=allocate(a,v.len,sizeof(*p),v.data); out.len=v.len; out.data=p; if(p) for(size_t i=0;i<v.len;++i) p[i]=clone_probability(a,v.data[i]); return out; }
static thinkthen_probability_v1 clone_probability(struct TTArena *a,thinkthen_probability_v1 v) { thinkthen_probability_v1 out={0}; (void)a; out.name=clone_string(a,v.name); out.probability=v.probability; return out; }
static thinkthen_profile_warning_v1 clone_profile_warning(struct TTArena *a,thinkthen_profile_warning_v1 v) { thinkthen_profile_warning_v1 out={0}; (void)a; out.tuned_for=clone_string(a,v.tuned_for); out.running=clone_string(a,v.running); return out; }
static thinkthen_question_author_v1 clone_question_author(struct TTArena *a,thinkthen_question_author_v1 v) { thinkthen_question_author_v1 out={0}; (void)a; out.name=clone_optional_string(a,v.name); out.wording_version=clone_optional_u64(a,v.wording_version); out.item_schema=clone_input_declaration(a,v.item_schema); out.context_schema=clone_input_declaration(a,v.context_schema); return out; }
static thinkthen_question_member_v1 clone_question_member(struct TTArena *a,thinkthen_question_member_v1 v) { thinkthen_question_member_v1 out={0}; (void)a; out.name=clone_string(a,v.name); thinkthen_question_view_v1 *p=allocate(a,1,sizeof(*p),v.question); if(p) *p=clone_question_view(a,*v.question); out.question=p; return out; }
static thinkthen_question_members_v1 clone_question_members(struct TTArena *a,thinkthen_question_members_v1 v) { thinkthen_question_members_v1 out={0}; thinkthen_question_member_v1 *p=allocate(a,v.len,sizeof(*p),v.data); out.len=v.len; out.data=p; if(p) for(size_t i=0;i<v.len;++i) p[i]=clone_question_member(a,v.data[i]); return out; }
static thinkthen_question_observation_v1 clone_question_observation(struct TTArena *a,thinkthen_question_observation_v1 v) { thinkthen_question_observation_v1 out={0}; (void)a; out.index=v.index; out.member=clone_optional_string(a,v.member); out.stage=clone_optional_discriminator(a,v.stage); out.position=v.position; out.question_sha256=clone_string(a,v.question_sha256); out.model=clone_string(a,v.model); out.url=clone_string(a,v.url); out.requests=clone_strings(a,v.requests); out.requests_sent=v.requests_sent; out.cached=v.cached; out.failed_questions=v.failed_questions; out.usage=clone_optional_usage(a,v.usage); out.question_sources=clone_question_sources(a,v.question_sources); out.state=v.state; switch(v.state) { case 1: out.data.success=clone_observation_success(a,v.data.success); break; case 2: out.data.failure=clone_member_failure(a,v.data.failure); break; default:a->failed=1; } return out; }
static thinkthen_question_source_v1 clone_question_source(struct TTArena *a,thinkthen_question_source_v1 v) { thinkthen_question_source_v1 out={0}; (void)a; out.origin=v.origin; out.answered_by=clone_string(a,v.answered_by); return out; }
static thinkthen_question_sources_v1 clone_question_sources(struct TTArena *a,thinkthen_question_sources_v1 v) { thinkthen_question_sources_v1 out={0}; thinkthen_question_source_v1 *p=allocate(a,v.len,sizeof(*p),v.data); out.len=v.len; out.data=p; if(p) for(size_t i=0;i<v.len;++i) p[i]=clone_question_source(a,v.data[i]); return out; }
static thinkthen_question_view_v1 clone_question_view(struct TTArena *a,thinkthen_question_view_v1 v) { thinkthen_question_view_v1 out={0}; (void)a; out.kind=v.kind; out.text=clone_content(a,v.text); out.yes=clone_optional_content(a,v.yes); out.no=clone_optional_content(a,v.no); out.choices=clone_choices(a,v.choices); out.threshold=clone_rule(a,v.threshold); out.relation_threshold=clone_rule(a,v.relation_threshold); out.model=clone_optional_string(a,v.model); out.profile=clone_optional_string(a,v.profile); out.batch=clone_optional_size(a,v.batch); out.batch_max=v.batch_max; out.none=v.none; out.on=clone_strings(a,v.on); out.members=clone_question_members(a,v.members); out.kinds=clone_choices(a,v.kinds); out.relations=clone_relations(a,v.relations); out.name_pointer=clone_optional_string(a,v.name_pointer); out.kind_pointer=clone_optional_string(a,v.kind_pointer); return out; }
static thinkthen_rank_view_v1 clone_rank_view(struct TTArena *a,thinkthen_rank_view_v1 v) { thinkthen_rank_view_v1 out={0}; (void)a; out.common=clone_row(a,v.common); out.value=clone_optional_size(a,v.value); out.question_name=clone_optional_string(a,v.question_name); return out; }
static thinkthen_recognize_answer_v1 clone_recognize_answer(struct TTArena *a,thinkthen_recognize_answer_v1 v) { thinkthen_recognize_answer_v1 out={0}; (void)a; out.pieces=clone_pieces(a,v.pieces); out.names=clone_names(a,v.names); out.pairs=clone_pairs(a,v.pairs); return out; }
static thinkthen_recognize_value_v1 clone_recognize_value(struct TTArena *a,thinkthen_recognize_value_v1 v) { thinkthen_recognize_value_v1 out={0}; (void)a; out.entities=clone_entities(a,v.entities); out.relations=clone_optional_entity_edges(a,v.relations); return out; }
static thinkthen_recognize_view_v1 clone_recognize_view(struct TTArena *a,thinkthen_recognize_view_v1 v) { thinkthen_recognize_view_v1 out={0}; (void)a; out.common=clone_row(a,v.common); out.value=clone_recognize_value(a,v.value); out.answer=clone_recognize_answer(a,v.answer); return out; }
static thinkthen_relate_view_v1 clone_relate_view(struct TTArena *a,thinkthen_relate_view_v1 v) { thinkthen_relate_view_v1 out={0}; (void)a; out.common=clone_row(a,v.common); out.value=clone_edges(a,v.value); out.questions=clone_relation_answers(a,v.questions); return out; }
static thinkthen_relation_answer_v1 clone_relation_answer(struct TTArena *a,thinkthen_relation_answer_v1 v) { thinkthen_relation_answer_v1 out={0}; (void)a; out.relation=clone_string(a,v.relation); out.reads=clone_string(a,v.reads); out.method=v.method; out.direction=v.direction; out.source=clone_endpoint(a,v.source); out.target=clone_optional_endpoint(a,v.target); out.request=clone_string(a,v.request); out.state=v.state; switch(v.state) { case 1: out.data.success=clone_relation_success(a,v.data.success); break; case 2: out.data.failure=clone_member_failure(a,v.data.failure); break; default:a->failed=1; } return out; }
static thinkthen_relation_answers_v1 clone_relation_answers(struct TTArena *a,thinkthen_relation_answers_v1 v) { thinkthen_relation_answers_v1 out={0}; thinkthen_relation_answer_v1 *p=allocate(a,v.len,sizeof(*p),v.data); out.len=v.len; out.data=p; if(p) for(size_t i=0;i<v.len;++i) p[i]=clone_relation_answer(a,v.data[i]); return out; }
static thinkthen_relation_success_v1 clone_relation_success(struct TTArena *a,thinkthen_relation_success_v1 v) { thinkthen_relation_success_v1 out={0}; (void)a; out.answer_id=clone_string(a,v.answer_id); out.probability=v.probability; out.accepted=v.accepted; return out; }
static thinkthen_relation_v1 clone_relation(struct TTArena *a,thinkthen_relation_v1 v) { thinkthen_relation_v1 out={0}; (void)a; out.name=clone_string(a,v.name); out.source=clone_string(a,v.source); out.target=clone_string(a,v.target); out.reads=clone_optional_string(a,v.reads); out.either=v.either; out.single=v.single; return out; }
static thinkthen_relations_v1 clone_relations(struct TTArena *a,thinkthen_relations_v1 v) { thinkthen_relations_v1 out={0}; thinkthen_relation_v1 *p=allocate(a,v.len,sizeof(*p),v.data); out.len=v.len; out.data=p; if(p) for(size_t i=0;i<v.len;++i) p[i]=clone_relation(a,v.data[i]); return out; }
static thinkthen_reported_usage_v1 clone_reported_usage(struct TTArena *a,thinkthen_reported_usage_v1 v) { thinkthen_reported_usage_v1 out={0}; (void)a; out.present=v.present; out.input_tokens=clone_optional_u64(a,v.input_tokens); out.output_tokens=clone_optional_u64(a,v.output_tokens); return out; }
static thinkthen_row_observation_v1 clone_row_observation(struct TTArena *a,thinkthen_row_observation_v1 v) { thinkthen_row_observation_v1 out={0}; (void)a; out.index=v.index; out.function=v.function; switch(v.function) { case 1: out.data.decide=clone_decide_view(a,v.data.decide); break; case 2: out.data.choose=clone_choose_view(a,v.data.choose); break; case 3: out.data.tag=clone_tag_view(a,v.data.tag); break; case 4: out.data.score=clone_score_view(a,v.data.score); break; case 5: out.data.filter=clone_filter_view(a,v.data.filter); break; case 6: out.data.rank=clone_rank_view(a,v.data.rank); break; case 7: out.data.find=clone_find_view(a,v.data.find); break; case 8: out.data.annotate=clone_annotate_view(a,v.data.annotate); break; case 9: out.data.recognize=clone_recognize_view(a,v.data.recognize); break; case 10: out.data.relate=clone_relate_view(a,v.data.relate); break; default:a->failed=1; } return out; }
static thinkthen_row_v1 clone_row(struct TTArena *a,thinkthen_row_v1 v) { thinkthen_row_v1 out={0}; (void)a; out.answer_id=clone_string(a,v.answer_id); out.input=clone_optional_content(a,v.input); out.question=clone_optional_question(a,v.question); out.answer=clone_optional_answer(a,v.answer); out.threshold=clone_optional_rule(a,v.threshold); out.position=clone_optional_location(a,v.position); out.input_file=clone_optional_string(a,v.input_file); out.meta=clone_meta(a,v.meta); out.images=clone_optional_image_views(a,v.images); return out; }
static thinkthen_rule_v1 clone_rule(struct TTArena *a,thinkthen_rule_v1 v) { thinkthen_rule_v1 out={0}; (void)a; out.kind=v.kind; out.low=v.low; out.high=v.high; return out; }
static thinkthen_score_answer_v1 clone_score_answer(struct TTArena *a,thinkthen_score_answer_v1 v) { thinkthen_score_answer_v1 out={0}; (void)a; out.level=clone_string(a,v.level); out.probabilities=clone_probabilities(a,v.probabilities); out.confidence=clone_optional_double(a,v.confidence); return out; }
static thinkthen_score_view_v1 clone_score_view(struct TTArena *a,thinkthen_score_view_v1 v) { thinkthen_score_view_v1 out={0}; (void)a; out.common=clone_row(a,v.common); out.value=v.value; return out; }
static thinkthen_source_detail_v1 clone_source_detail(struct TTArena *a,thinkthen_source_detail_v1 v) { thinkthen_source_detail_v1 out={0}; (void)a; out.origin=v.origin; out.answered_by=clone_string(a,v.answered_by); out.batch_size=clone_optional_size(a,v.batch_size); return out; }
static thinkthen_source_details_v1 clone_source_details(struct TTArena *a,thinkthen_source_details_v1 v) { thinkthen_source_details_v1 out={0}; thinkthen_source_detail_v1 *p=allocate(a,v.len,sizeof(*p),v.data); out.len=v.len; out.data=p; if(p) for(size_t i=0;i<v.len;++i) p[i]=clone_source_detail(a,v.data[i]); return out; }
static thinkthen_source_edge_v1 clone_source_edge(struct TTArena *a,thinkthen_source_edge_v1 v) { thinkthen_source_edge_v1 out={0}; (void)a; out.relation=clone_string(a,v.relation); out.source=clone_source_endpoint(a,v.source); out.target=clone_source_endpoint(a,v.target); out.probability=v.probability; out.either=v.either; return out; }
static thinkthen_source_edges_v1 clone_source_edges(struct TTArena *a,thinkthen_source_edges_v1 v) { thinkthen_source_edges_v1 out={0}; thinkthen_source_edge_v1 *p=allocate(a,v.len,sizeof(*p),v.data); out.len=v.len; out.data=p; if(p) for(size_t i=0;i<v.len;++i) p[i]=clone_source_edge(a,v.data[i]); return out; }
static thinkthen_source_endpoint_v1 clone_source_endpoint(struct TTArena *a,thinkthen_source_endpoint_v1 v) { thinkthen_source_endpoint_v1 out={0}; (void)a; out.ordinal=v.ordinal; out.endpoint=clone_endpoint(a,v.endpoint); out.record=clone_content(a,v.record); out.position=clone_optional_location(a,v.position); return out; }
static thinkthen_source_entities_v1 clone_source_entities(struct TTArena *a,thinkthen_source_entities_v1 v) { thinkthen_source_entities_v1 out={0}; thinkthen_source_entity_v1 *p=allocate(a,v.len,sizeof(*p),v.data); out.len=v.len; out.data=p; if(p) for(size_t i=0;i<v.len;++i) p[i]=clone_source_entity(a,v.data[i]); return out; }
static thinkthen_source_entity_edge_v1 clone_source_entity_edge(struct TTArena *a,thinkthen_source_entity_edge_v1 v) { thinkthen_source_entity_edge_v1 out={0}; (void)a; out.relation=clone_string(a,v.relation); out.source=clone_source_entity(a,v.source); out.target=clone_source_entity(a,v.target); out.probability=v.probability; out.either=v.either; return out; }
static thinkthen_source_entity_edges_v1 clone_source_entity_edges(struct TTArena *a,thinkthen_source_entity_edges_v1 v) { thinkthen_source_entity_edges_v1 out={0}; thinkthen_source_entity_edge_v1 *p=allocate(a,v.len,sizeof(*p),v.data); out.len=v.len; out.data=p; if(p) for(size_t i=0;i<v.len;++i) p[i]=clone_source_entity_edge(a,v.data[i]); return out; }
static thinkthen_source_entity_v1 clone_source_entity(struct TTArena *a,thinkthen_source_entity_v1 v) { thinkthen_source_entity_v1 out={0}; (void)a; out.entity=clone_entity(a,v.entity); out.position=clone_optional_location(a,v.position); return out; }
static thinkthen_source_recognition_v1 clone_source_recognition(struct TTArena *a,thinkthen_source_recognition_v1 v) { thinkthen_source_recognition_v1 out={0}; (void)a; out.present=v.present; out.entities=clone_source_entities(a,v.entities); out.relations=clone_optional_source_entity_edges(a,v.relations); return out; }
static thinkthen_source_relations_v1 clone_source_relations(struct TTArena *a,thinkthen_source_relations_v1 v) { thinkthen_source_relations_v1 out={0}; (void)a; out.present=v.present; out.edges=clone_source_edges(a,v.edges); return out; }
static thinkthen_stopped_v1 clone_stopped(struct TTArena *a,thinkthen_stopped_v1 v) { thinkthen_stopped_v1 out={0}; (void)a; out.at=clone_optional_size(a,v.at); out.cause=v.cause; out.status=clone_optional_u16(a,v.status); out.retryable=v.retryable; return out; }
static thinkthen_string_v1 clone_string(struct TTArena *a,thinkthen_string_v1 v) { thinkthen_string_v1 out={0}; out.len=v.len; char *p=allocate(a,v.len,1,v.data); if(p) memcpy(p,v.data,v.len); out.data=p; return out; }
static thinkthen_strings_v1 clone_strings(struct TTArena *a,thinkthen_strings_v1 v) { thinkthen_strings_v1 out={0}; thinkthen_string_v1 *p=allocate(a,v.len,sizeof(*p),v.data); out.len=v.len; out.data=p; if(p) for(size_t i=0;i<v.len;++i) p[i]=clone_string(a,v.data[i]); return out; }
static thinkthen_summary_v1 clone_summary(struct TTArena *a,thinkthen_summary_v1 v) { thinkthen_summary_v1 out={0}; (void)a; out.state=v.state; out.schema=clone_string(a,v.schema); out.answer_id=clone_optional_string(a,v.answer_id); out.function=clone_optional_discriminator(a,v.function); out.count=v.count; out.observation_count=v.observation_count; out.meta=clone_optional_meta(a,v.meta); out.facts=clone_optional_facts(a,v.facts); out.attempts=clone_optional_attempts(a,v.attempts); out.error=clone_optional_error(a,v.error); return out; }
static thinkthen_tag_view_v1 clone_tag_view(struct TTArena *a,thinkthen_tag_view_v1 v) { thinkthen_tag_view_v1 out={0}; (void)a; out.common=clone_row(a,v.common); out.value=clone_strings(a,v.value); return out; }
static thinkthen_usage_v1 clone_usage(struct TTArena *a,thinkthen_usage_v1 v) { thinkthen_usage_v1 out={0}; (void)a; out.input_tokens=v.input_tokens; out.output_tokens=v.output_tokens; return out; }
struct TTNativeResult {
    struct TTArena *arena;
    thinkthen_summary_v1 summary;
    thinkthen_row_observation_v1 *rows;
    thinkthen_observation_v1 *events;
    thinkthen_details_v1 *details, *event_details;
    thinkthen_question_author_v1 *authors, *event_authors, **member_authors;
    size_t *member_counts, *rank_counts;
    thinkthen_rank_view_v1 **rank_members;
    thinkthen_source_recognition_v1 *recognition;
    thinkthen_source_relations_v1 *relations;
};
void tt_native_result_free(TTNativeResult *r) { if(r) { arena_free(r->arena); free(r); } }
int tt_native_snapshot(thinkthen_result *raw,TTNativeResult **out) {
    if(!raw || !out) { thinkthen_result_free(raw); return THINKTHEN_EUSAGE; }
    TTNativeResult *r=calloc(1,sizeof(*r));
    if(!r) { thinkthen_result_free(raw); return THINKTHEN_ELOCAL; }
    r->arena=calloc(1,sizeof(*r->arena));
    if(!r->arena) { free(r); thinkthen_result_free(raw); return THINKTHEN_ELOCAL; }
    struct TTArena *a=r->arena; thinkthen_summary_v1 s={0}; int code=thinkthen_result_summary(raw,&s);
    if(code) goto done;
    r->summary=clone_summary(a,s);
#define ARRAY(field,n) r->field=allocate(a,n,sizeof(*r->field),raw); if((n) && !r->field) goto done;
    ARRAY(rows,s.count) ARRAY(details,s.count) ARRAY(authors,s.count)
    ARRAY(member_authors,s.count) ARRAY(member_counts,s.count) ARRAY(rank_members,s.count) ARRAY(rank_counts,s.count)
    ARRAY(recognition,s.count) ARRAY(relations,s.count)
    ARRAY(events,s.observation_count) ARRAY(event_details,s.observation_count) ARRAY(event_authors,s.observation_count)
#undef ARRAY
    for(size_t i=0;i<s.observation_count;++i) {
        thinkthen_observation_v1 v={0}; code=thinkthen_result_observation(raw,i,&v); if(code) goto done; r->events[i]=clone_observation(a,v);
        thinkthen_details_v1 d={0}; code=thinkthen_result_observation_details(raw,i,&d); if(code) goto done; r->event_details[i]=clone_details(a,d);
        thinkthen_question_author_v1 author={0}; code=thinkthen_result_observation_author(raw,i,&author); if(code) goto done; r->event_authors[i]=clone_question_author(a,author);
    }
    for(size_t i=0;i<s.count;++i) {
        thinkthen_row_observation_v1 v={0}; v.function=s.function.value;
        switch(v.function) {
#define ROW(name,n) case n:code=thinkthen_result_##name(raw,i,&v.data.name);break;
            ROW(decide,1) ROW(choose,2) ROW(tag,3) ROW(score,4) ROW(filter,5)
            ROW(rank,6) ROW(find,7) ROW(annotate,8) ROW(recognize,9) ROW(relate,10)
#undef ROW
            default:code=THINKTHEN_EDEFECT;
        }
        if(code) goto done;
        r->rows[i]=clone_row_observation(a,v);
        thinkthen_details_v1 d={0}; code=thinkthen_result_details(raw,i,&d); if(code) goto done; r->details[i]=clone_details(a,d);
        thinkthen_question_author_v1 author={0}; code=thinkthen_result_question_author(raw,i,&author); if(code) goto done; r->authors[i]=clone_question_author(a,author);
        if(v.function==8) {
            size_t n=v.data.annotate.answers.len; r->member_counts[i]=n;
            r->member_authors[i]=allocate(a,n,sizeof(author),raw); if(n && !r->member_authors[i]) goto done;
            for(size_t j=0;j<n;++j) { code=thinkthen_result_member_author(raw,i,j,&author); if(code) goto done; r->member_authors[i][j]=clone_question_author(a,author); }
        }
        if(v.function==6) {
            size_t n=0; code=thinkthen_result_rank_member_count(raw,i,&n); if(code) goto done; r->rank_counts[i]=n;
            r->rank_members[i]=allocate(a,n,sizeof(thinkthen_rank_view_v1),raw); if(n && !r->rank_members[i]) goto done;
            for(size_t j=0;j<n;++j) { thinkthen_rank_view_v1 rank={0}; code=thinkthen_result_rank_member(raw,i,j,&rank); if(code) goto done; r->rank_members[i][j]=clone_rank_view(a,rank); }
        }
        if(v.function==9) { thinkthen_source_recognition_v1 rec={0}; code=thinkthen_result_source_recognition(raw,i,&rec); if(code) goto done; r->recognition[i]=clone_source_recognition(a,rec); }
        if(v.function==10) { thinkthen_source_relations_v1 rel={0}; code=thinkthen_result_source_relations(raw,i,&rel); if(code) goto done; r->relations[i]=clone_source_relations(a,rel); }
    }
done:
    thinkthen_result_free(raw);
    if(code || a->failed) { tt_native_result_free(r); return code?code:THINKTHEN_ELOCAL; }
    *out=r; return 0;
}
int tt_native_summary(const TTNativeResult *r,thinkthen_summary_v1 *out) { if(!r || !out) return THINKTHEN_EUSAGE; *out=r->summary; return 0; }
#define GET(name,field,type,count) int tt_native_##name(const TTNativeResult *r,size_t i,type *out) { if(!r || !out || i>=r->summary.count) return THINKTHEN_EUSAGE; *out=r->field[i]; return 0; }
GET(details,details,thinkthen_details_v1,count) GET(author,authors,thinkthen_question_author_v1,count)

#undef GET
#define EVENT(name,field,type) int tt_native_##name(const TTNativeResult *r,size_t i,type *out) { if(!r || !out || i>=r->summary.observation_count) return THINKTHEN_EUSAGE; *out=r->field[i]; return 0; }
EVENT(observation,events,thinkthen_observation_v1) EVENT(observation_details,event_details,thinkthen_details_v1) EVENT(observation_author,event_authors,thinkthen_question_author_v1)
#undef EVENT
#define VIEW(name,n) int tt_native_##name(const TTNativeResult *r,size_t i,thinkthen_##name##_view_v1 *out) { if(!r || !out || i>=r->summary.count || r->rows[i].function!=n) return THINKTHEN_EUSAGE; *out=r->rows[i].data.name; return 0; }
VIEW(decide,1) VIEW(choose,2) VIEW(tag,3) VIEW(score,4) VIEW(filter,5) VIEW(rank,6) VIEW(find,7) VIEW(annotate,8) VIEW(recognize,9) VIEW(relate,10)
#undef VIEW
int tt_native_member_author(const TTNativeResult *r,size_t i,size_t j,thinkthen_question_author_v1 *out) { if(!r || !out || i>=r->summary.count || j>=r->member_counts[i]) return THINKTHEN_EUSAGE; *out=r->member_authors[i][j]; return 0; }
int tt_native_rank_member_count(const TTNativeResult *r,size_t i,size_t *out) { if(!r || !out || i>=r->summary.count || r->rows[i].function!=6) return THINKTHEN_EUSAGE; *out=r->rank_counts[i]; return 0; }
int tt_native_rank_member(const TTNativeResult *r,size_t i,size_t j,thinkthen_rank_view_v1 *out) { if(!r || !out || i>=r->summary.count || j>=r->rank_counts[i]) return THINKTHEN_EUSAGE; *out=r->rank_members[i][j]; return 0; }

int tt_native_source_recognition(const TTNativeResult *r,size_t i,thinkthen_source_recognition_v1 *out) { if(!r || !out || i>=r->summary.count || r->rows[i].function!=9) return THINKTHEN_EUSAGE; *out=r->recognition[i]; return 0; }
int tt_native_source_relations(const TTNativeResult *r,size_t i,thinkthen_source_relations_v1 *out) { if(!r || !out || i>=r->summary.count || r->rows[i].function!=10) return THINKTHEN_EUSAGE; *out=r->relations[i]; return 0; }
