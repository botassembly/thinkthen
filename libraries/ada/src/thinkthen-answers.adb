-- Generated owned native answers. Do not edit.
with Thinkthen.Native_Values;
with Ada.Unchecked_Conversion;
with Ada.Strings.Fixed;
with System;
with Interfaces.C;
with Thinkthen.Native_Sessions; use Thinkthen.Native_Sessions;
with Thinkthen.Session_C;
package body Thinkthen.Answers is
   use Ada.Strings.Unbounded;
   use Thinkthen.Values;
   use Thinkthen.Sessions;
   use Thinkthen.Session_C;
   use Interfaces.C;
   use type Interfaces.Unsigned_32;
   package Packet_Lists is new Ada.Containers.Vectors (Positive, Value);
   function Read_thinkthen_complete_session_packet_v1 (P : access constant thinkthen_complete_session_packet_v1) return Value;
function Read_thinkthen_complete_session_packet_decide_row_v1 (P : access constant thinkthen_complete_session_packet_decide_row_v1) return Value;
function Read_thinkthen_complete_atomic_decide_value_v1 (P : access constant thinkthen_complete_atomic_decide_value_v1) return Value;
function Read_thinkthen_complete_answer_v1 (P : access constant thinkthen_complete_answer_v1) return Value;
function Read_thinkthen_complete_answer_yes_no_v1 (P : access constant thinkthen_complete_answer_yes_no_v1) return Value;
function Read_thinkthen_complete_answer_choice_v1 (P : access constant thinkthen_complete_answer_choice_v1) return Value;
function Read_thinkthen_complete_number_map_v1 (P : thinkthen_complete_number_map_v1) return Value;
function Read_thinkthen_complete_answer_tag_v1 (P : access constant thinkthen_complete_answer_tag_v1) return Value;
function Read_thinkthen_complete_answer_score_v1 (P : access constant thinkthen_complete_answer_score_v1) return Value;
function Read_thinkthen_complete_answer_id_v1 (P : access constant thinkthen_complete_answer_id_v1) return Value;
function Read_thinkthen_complete_image_list_v1 (P : thinkthen_complete_image_list_v1) return Value;
function Read_thinkthen_complete_image_v1 (P : access constant thinkthen_complete_image_v1) return Value;
function Read_thinkthen_complete_image_media_v1 (P : access constant thinkthen_complete_image_media_v1) return Value;
function Read_thinkthen_complete_rank_member_list_v1 (P : thinkthen_complete_rank_member_list_v1) return Value;
function Read_thinkthen_complete_rank_member_v1 (P : access constant thinkthen_complete_rank_member_v1) return Value;
function Read_thinkthen_complete_rank_member_result_v1 (P : access constant thinkthen_complete_rank_member_result_v1) return Value;
function Read_thinkthen_complete_meta_v1 (P : access constant thinkthen_complete_meta_v1) return Value;
function Read_thinkthen_complete_attempt_list_v1 (P : thinkthen_complete_attempt_list_v1) return Value;
function Read_thinkthen_complete_attempt_v1 (P : access constant thinkthen_complete_attempt_v1) return Value;
function Read_thinkthen_complete_attempt_outcome_v1 (P : access constant thinkthen_complete_attempt_outcome_v1) return Value;
function Read_thinkthen_complete_sdk_request_id_v1 (P : access constant thinkthen_complete_sdk_request_id_v1) return Value;
function Read_thinkthen_complete_batch_setting_v1 (P : access constant thinkthen_complete_batch_setting_v1) return Value;
function Read_thinkthen_complete_batch_warning_v1 (P : access constant thinkthen_complete_batch_warning_v1) return Value;
function Read_thinkthen_complete_observation_list_v1 (P : thinkthen_complete_observation_list_v1) return Value;
function Read_thinkthen_complete_observation_v1 (P : access constant thinkthen_complete_observation_v1) return Value;
function Read_thinkthen_complete_observation_observation_id_v1 (P : access constant thinkthen_complete_observation_observation_id_v1) return Value;
function Read_thinkthen_complete_observation_id_v1 (P : access constant thinkthen_complete_observation_id_v1) return Value;
function Read_thinkthen_complete_observation_failure_id_v1 (P : access constant thinkthen_complete_observation_failure_id_v1) return Value;
function Read_thinkthen_complete_failure_id_v1 (P : access constant thinkthen_complete_failure_id_v1) return Value;
function Read_thinkthen_complete_origin_v1 (P : access constant thinkthen_complete_origin_v1) return Value;
function Read_thinkthen_complete_profile_warning_v1 (P : access constant thinkthen_complete_profile_warning_v1) return Value;
function Read_thinkthen_complete_question_source_list_v1 (P : thinkthen_complete_question_source_list_v1) return Value;
function Read_thinkthen_complete_question_source_v1 (P : access constant thinkthen_complete_question_source_v1) return Value;
function Read_thinkthen_complete_string_list_v1 (P : thinkthen_complete_string_list_v1) return Value;
function Read_thinkthen_complete_usage_v1 (P : access constant thinkthen_complete_usage_v1) return Value;
function Read_thinkthen_complete_question_document_v1 (P : access constant thinkthen_complete_question_document_v1) return Value;
function Read_thinkthen_complete_question_document_decide_v1 (P : access constant thinkthen_complete_question_document_decide_v1) return Value;
function Read_thinkthen_complete_batch_v1 (P : access constant thinkthen_complete_batch_v1) return Value;
function Read_thinkthen_complete_batch_string_v1 (P : access constant thinkthen_complete_batch_string_v1) return Value;
function Read_thinkthen_complete_input_declaration_v1 (P : access constant thinkthen_complete_input_declaration_v1) return Value;
function Read_thinkthen_complete_input_declaration_string_root_v1 (P : access constant thinkthen_complete_input_declaration_string_root_v1) return Value;
function Read_thinkthen_complete_string_type_v1 (P : access constant thinkthen_complete_string_type_v1) return Value;
function Read_thinkthen_complete_input_declaration_object_root_v1 (P : access constant thinkthen_complete_input_declaration_object_root_v1) return Value;
function Read_thinkthen_complete_input_property_type_map_v1 (P : thinkthen_complete_input_property_type_map_v1) return Value;
function Read_thinkthen_complete_input_property_type_v1 (P : access constant thinkthen_complete_input_property_type_v1) return Value;
function Read_thinkthen_complete_input_property_type_string_v1 (P : access constant thinkthen_complete_input_property_type_string_v1) return Value;
function Read_thinkthen_complete_input_property_type_number_v1 (P : access constant thinkthen_complete_input_property_type_number_v1) return Value;
function Read_thinkthen_complete_input_property_type_boolean_v1 (P : access constant thinkthen_complete_input_property_type_boolean_v1) return Value;
function Read_thinkthen_complete_input_property_type_array_v1 (P : access constant thinkthen_complete_input_property_type_array_v1) return Value;
function Read_thinkthen_complete_string_root_v1 (P : access constant thinkthen_complete_string_root_v1) return Value;
function Read_thinkthen_complete_object_type_v1 (P : access constant thinkthen_complete_object_type_v1) return Value;
function Read_thinkthen_complete_label_list_v1 (P : thinkthen_complete_label_list_v1) return Value;
function Read_thinkthen_complete_label_v1 (P : access constant thinkthen_complete_label_v1) return Value;
function Read_thinkthen_complete_question_name_v1 (P : access constant thinkthen_complete_question_name_v1) return Value;
function Read_thinkthen_complete_wording_version_v1 (P : access constant thinkthen_complete_wording_version_v1) return Value;
function Read_thinkthen_complete_question_document_choose_v1 (P : access constant thinkthen_complete_question_document_choose_v1) return Value;
function Read_thinkthen_complete_question_document_tag_v1 (P : access constant thinkthen_complete_question_document_tag_v1) return Value;
function Read_thinkthen_complete_question_document_score_v1 (P : access constant thinkthen_complete_question_document_score_v1) return Value;
function Read_thinkthen_complete_version_v1 (P : access constant thinkthen_complete_version_v1) return Value;
function Read_thinkthen_complete_physical_source_v1 (P : access constant thinkthen_complete_physical_source_v1) return Value;
function Read_thinkthen_complete_threshold_v1 (P : access constant thinkthen_complete_threshold_v1) return Value;
function Read_thinkthen_complete_decide_value_v1 (P : access constant thinkthen_complete_decide_value_v1) return Value;
function Read_thinkthen_complete_session_packet_choose_row_v1 (P : access constant thinkthen_complete_session_packet_choose_row_v1) return Value;
function Read_thinkthen_complete_atomic_nullable_string_v1 (P : access constant thinkthen_complete_atomic_nullable_string_v1) return Value;
function Read_thinkthen_complete_session_packet_tag_row_v1 (P : access constant thinkthen_complete_session_packet_tag_row_v1) return Value;
function Read_thinkthen_complete_atomic_array_of_string_v1 (P : access constant thinkthen_complete_atomic_array_of_string_v1) return Value;
function Read_thinkthen_complete_session_packet_score_row_v1 (P : access constant thinkthen_complete_session_packet_score_row_v1) return Value;
function Read_thinkthen_complete_atomic_double_v1 (P : access constant thinkthen_complete_atomic_double_v1) return Value;
function Read_thinkthen_complete_session_packet_filter_row_v1 (P : access constant thinkthen_complete_session_packet_filter_row_v1) return Value;
function Read_thinkthen_complete_atomic_boolean_v1 (P : access constant thinkthen_complete_atomic_boolean_v1) return Value;
function Read_thinkthen_complete_session_packet_annotate_row_v1 (P : access constant thinkthen_complete_session_packet_annotate_row_v1) return Value;
function Read_thinkthen_complete_annotation_v1 (P : access constant thinkthen_complete_annotation_v1) return Value;
function Read_thinkthen_complete_annotation_member_map_v1 (P : thinkthen_complete_annotation_member_map_v1) return Value;
function Read_thinkthen_complete_annotation_member_v1 (P : access constant thinkthen_complete_annotation_member_v1) return Value;
function Read_thinkthen_complete_annotation_member_answer_id_v1 (P : access constant thinkthen_complete_annotation_member_answer_id_v1) return Value;
function Read_thinkthen_complete_value_v1 (P : access constant thinkthen_complete_value_v1) return Value;
function Read_thinkthen_complete_annotation_member_failure_id_v1 (P : access constant thinkthen_complete_annotation_member_failure_id_v1) return Value;
function Read_thinkthen_complete_failure_v1 (P : access constant thinkthen_complete_failure_v1) return Value;
function Read_thinkthen_complete_failure_cause_v1 (P : access constant thinkthen_complete_failure_cause_v1) return Value;
function Read_thinkthen_complete_failure_field_kind_v1 (P : access constant thinkthen_complete_failure_field_kind_v1) return Value;
function Read_thinkthen_complete_position_v1 (P : access constant thinkthen_complete_position_v1) return Value;
function Read_thinkthen_complete_annotated_row_v1 (P : access constant thinkthen_complete_annotated_row_v1) return Value;
function Read_thinkthen_complete_annotated_field_map_v1 (P : thinkthen_complete_annotated_field_map_v1) return Value;
function Read_thinkthen_complete_annotated_field_v1 (P : access constant thinkthen_complete_annotated_field_v1) return Value;
function Read_thinkthen_complete_failed_v1 (P : access constant thinkthen_complete_failed_v1) return Value;
function Read_thinkthen_complete_session_packet_decide_aggregate_v1 (P : access constant thinkthen_complete_session_packet_decide_aggregate_v1) return Value;
function Read_thinkthen_complete_atomic_decide_value_list_v1 (P : thinkthen_complete_atomic_decide_value_list_v1) return Value;
function Read_thinkthen_complete_session_packet_choose_aggregate_v1 (P : access constant thinkthen_complete_session_packet_choose_aggregate_v1) return Value;
function Read_thinkthen_complete_atomic_nullable_string_list_v1 (P : thinkthen_complete_atomic_nullable_string_list_v1) return Value;
function Read_thinkthen_complete_session_packet_tag_aggregate_v1 (P : access constant thinkthen_complete_session_packet_tag_aggregate_v1) return Value;
function Read_thinkthen_complete_atomic_array_of_string_list_v1 (P : thinkthen_complete_atomic_array_of_string_list_v1) return Value;
function Read_thinkthen_complete_session_packet_score_aggregate_v1 (P : access constant thinkthen_complete_session_packet_score_aggregate_v1) return Value;
function Read_thinkthen_complete_atomic_double_list_v1 (P : thinkthen_complete_atomic_double_list_v1) return Value;
function Read_thinkthen_complete_session_packet_filter_aggregate_v1 (P : access constant thinkthen_complete_session_packet_filter_aggregate_v1) return Value;
function Read_thinkthen_complete_atomic_boolean_list_v1 (P : thinkthen_complete_atomic_boolean_list_v1) return Value;
function Read_thinkthen_complete_session_packet_rank_aggregate_v1 (P : access constant thinkthen_complete_session_packet_rank_aggregate_v1) return Value;
function Read_thinkthen_complete_atomic_non_zero_usize_list_v1 (P : thinkthen_complete_atomic_non_zero_usize_list_v1) return Value;
function Read_thinkthen_complete_atomic_non_zero_usize_v1 (P : access constant thinkthen_complete_atomic_non_zero_usize_v1) return Value;
function Read_thinkthen_complete_session_packet_find_aggregate_v1 (P : access constant thinkthen_complete_session_packet_find_aggregate_v1) return Value;
function Read_thinkthen_complete_find_v1 (P : access constant thinkthen_complete_find_v1) return Value;
function Read_thinkthen_complete_find_answer_v1 (P : access constant thinkthen_complete_find_answer_v1) return Value;
function Read_thinkthen_complete_find_candidate_list_v1 (P : thinkthen_complete_find_candidate_list_v1) return Value;
function Read_thinkthen_complete_find_candidate_v1 (P : access constant thinkthen_complete_find_candidate_v1) return Value;
function Read_thinkthen_complete_find_question_document_v1 (P : access constant thinkthen_complete_find_question_document_v1) return Value;
function Read_thinkthen_complete_session_packet_annotate_aggregate_v1 (P : access constant thinkthen_complete_session_packet_annotate_aggregate_v1) return Value;
function Read_thinkthen_complete_annotation_list_v1 (P : thinkthen_complete_annotation_list_v1) return Value;
function Read_thinkthen_complete_session_packet_recognize_aggregate_v1 (P : access constant thinkthen_complete_session_packet_recognize_aggregate_v1) return Value;
function Read_thinkthen_complete_recognition_list_v1 (P : thinkthen_complete_recognition_list_v1) return Value;
function Read_thinkthen_complete_recognition_v1 (P : access constant thinkthen_complete_recognition_v1) return Value;
function Read_thinkthen_complete_recognition_odds_v1 (P : access constant thinkthen_complete_recognition_odds_v1) return Value;
function Read_thinkthen_complete_recognition_odds_recognize_answer_v1 (P : access constant thinkthen_complete_recognition_odds_recognize_answer_v1) return Value;
function Read_thinkthen_complete_name_odds_list_v1 (P : thinkthen_complete_name_odds_list_v1) return Value;
function Read_thinkthen_complete_name_odds_v1 (P : access constant thinkthen_complete_name_odds_v1) return Value;
function Read_thinkthen_complete_pair_odds_list_v1 (P : thinkthen_complete_pair_odds_list_v1) return Value;
function Read_thinkthen_complete_pair_odds_v1 (P : access constant thinkthen_complete_pair_odds_v1) return Value;
function Read_thinkthen_complete_place_v1 (P : access constant thinkthen_complete_place_v1) return Value;
function Read_thinkthen_complete_piece_odds_list_v1 (P : thinkthen_complete_piece_odds_list_v1) return Value;
function Read_thinkthen_complete_piece_odds_v1 (P : access constant thinkthen_complete_piece_odds_v1) return Value;
function Read_thinkthen_complete_recognition_proposal_list_v1 (P : thinkthen_complete_recognition_proposal_list_v1) return Value;
function Read_thinkthen_complete_recognition_proposal_v1 (P : access constant thinkthen_complete_recognition_proposal_v1) return Value;
function Read_thinkthen_complete_recognition_odds_boundary_odds_v1 (P : access constant thinkthen_complete_recognition_odds_boundary_odds_v1) return Value;
function Read_thinkthen_complete_boundary_proposal_list_v1 (P : thinkthen_complete_boundary_proposal_list_v1) return Value;
function Read_thinkthen_complete_boundary_proposal_v1 (P : access constant thinkthen_complete_boundary_proposal_v1) return Value;
function Read_thinkthen_complete_recognize_question_document_v1 (P : access constant thinkthen_complete_recognize_question_document_v1) return Value;
function Read_thinkthen_complete_json_map_v1 (P : thinkthen_complete_json_map_v1) return Value;
function Read_thinkthen_complete_recognition_mode_v1 (P : access constant thinkthen_complete_recognition_mode_v1) return Value;
function Read_thinkthen_complete_relation_rule_list_v1 (P : thinkthen_complete_relation_rule_list_v1) return Value;
function Read_thinkthen_complete_relation_rule_v1 (P : access constant thinkthen_complete_relation_rule_v1) return Value;
function Read_thinkthen_complete_recognition_stage_context_v1 (P : access constant thinkthen_complete_recognition_stage_context_v1) return Value;
function Read_thinkthen_complete_verb_v1 (P : access constant thinkthen_complete_verb_v1) return Value;
function Read_thinkthen_complete_recognize_v1 (P : access constant thinkthen_complete_recognize_v1) return Value;
function Read_thinkthen_complete_recognize_entities_v1 (P : access constant thinkthen_complete_recognize_entities_v1) return Value;
function Read_thinkthen_complete_entity_list_v1 (P : thinkthen_complete_entity_list_v1) return Value;
function Read_thinkthen_complete_entity_v1 (P : access constant thinkthen_complete_entity_v1) return Value;
function Read_thinkthen_complete_entity_edge_list_v1 (P : thinkthen_complete_entity_edge_list_v1) return Value;
function Read_thinkthen_complete_entity_edge_v1 (P : access constant thinkthen_complete_entity_edge_v1) return Value;
function Read_thinkthen_complete_recognize_mode_proposals_v1 (P : access constant thinkthen_complete_recognize_mode_proposals_v1) return Value;
function Read_thinkthen_complete_boundary_mode_v1 (P : access constant thinkthen_complete_boundary_mode_v1) return Value;
function Read_thinkthen_complete_session_packet_relate_aggregate_v1 (P : access constant thinkthen_complete_session_packet_relate_aggregate_v1) return Value;
function Read_thinkthen_complete_relation_v1 (P : access constant thinkthen_complete_relation_v1) return Value;
function Read_thinkthen_complete_answers_v1 (P : access constant thinkthen_complete_answers_v1) return Value;
function Read_thinkthen_complete_relation_member_list_v1 (P : thinkthen_complete_relation_member_list_v1) return Value;
function Read_thinkthen_complete_relation_member_v1 (P : access constant thinkthen_complete_relation_member_v1) return Value;
function Read_thinkthen_complete_relation_member_answer_id_v1 (P : access constant thinkthen_complete_relation_member_answer_id_v1) return Value;
function Read_thinkthen_complete_relation_direction_v1 (P : access constant thinkthen_complete_relation_direction_v1) return Value;
function Read_thinkthen_complete_relation_method_v1 (P : access constant thinkthen_complete_relation_method_v1) return Value;
function Read_thinkthen_complete_related_entity_v1 (P : access constant thinkthen_complete_related_entity_v1) return Value;
function Read_thinkthen_complete_relation_member_failure_id_v1 (P : access constant thinkthen_complete_relation_member_failure_id_v1) return Value;
function Read_thinkthen_complete_session_input_source_list_v1 (P : thinkthen_complete_session_input_source_list_v1) return Value;
function Read_thinkthen_complete_session_input_source_v1 (P : access constant thinkthen_complete_session_input_source_v1) return Value;
function Read_thinkthen_complete_relate_question_document_v1 (P : access constant thinkthen_complete_relate_question_document_v1) return Value;
function Read_thinkthen_complete_relate_fields_v1 (P : access constant thinkthen_complete_relate_fields_v1) return Value;
function Read_thinkthen_complete_related_entity_edge_list_v1 (P : thinkthen_complete_related_entity_edge_list_v1) return Value;
function Read_thinkthen_complete_related_entity_edge_v1 (P : access constant thinkthen_complete_related_entity_edge_v1) return Value;
function Read_thinkthen_complete_relation_endpoint_v1 (P : access constant thinkthen_complete_relation_endpoint_v1) return Value;
function Read_thinkthen_complete_relation_endpoint_related_entity_v1 (P : access constant thinkthen_complete_relation_endpoint_related_entity_v1) return Value;
function Read_thinkthen_complete_relation_endpoint_source_v1 (P : access constant thinkthen_complete_relation_endpoint_source_v1) return Value;
function Read_thinkthen_complete_session_packet_observation_v1 (P : access constant thinkthen_complete_session_packet_observation_v1) return Value;
function Read_thinkthen_complete_request_function_v1 (P : access constant thinkthen_complete_request_function_v1) return Value;
function Read_thinkthen_complete_session_observation_v1 (P : access constant thinkthen_complete_session_observation_v1) return Value;
function Read_thinkthen_complete_session_observation_question_v1 (P : access constant thinkthen_complete_session_observation_question_v1) return Value;
function Read_thinkthen_complete_session_question_detail_v1 (P : access constant thinkthen_complete_session_question_detail_v1) return Value;
function Read_thinkthen_complete_json_list_v1 (P : thinkthen_complete_json_list_v1) return Value;
function Read_thinkthen_complete_session_probabilities_v1 (P : access constant thinkthen_complete_session_probabilities_v1) return Value;
function Read_thinkthen_complete_session_probabilities_yes_no_v1 (P : access constant thinkthen_complete_session_probabilities_yes_no_v1) return Value;
function Read_thinkthen_complete_session_probabilities_named_v1 (P : access constant thinkthen_complete_session_probabilities_named_v1) return Value;
function Read_thinkthen_complete_session_named_probability_list_v1 (P : thinkthen_complete_session_named_probability_list_v1) return Value;
function Read_thinkthen_complete_session_named_probability_v1 (P : access constant thinkthen_complete_session_named_probability_v1) return Value;
function Read_thinkthen_complete_token_usage_v1 (P : access constant thinkthen_complete_token_usage_v1) return Value;
function Read_thinkthen_complete_session_observation_row_v1 (P : access constant thinkthen_complete_session_observation_row_v1) return Value;
function Read_thinkthen_complete_session_observed_row_v1 (P : access constant thinkthen_complete_session_observed_row_v1) return Value;
function Read_thinkthen_complete_session_observed_row_judgment_v1 (P : access constant thinkthen_complete_session_observed_row_judgment_v1) return Value;
function Read_thinkthen_complete_session_judgment_v1 (P : access constant thinkthen_complete_session_judgment_v1) return Value;
function Read_thinkthen_complete_session_judgment_decision_v1 (P : access constant thinkthen_complete_session_judgment_decision_v1) return Value;
function Read_thinkthen_complete_session_judgment_choice_v1 (P : access constant thinkthen_complete_session_judgment_choice_v1) return Value;
function Read_thinkthen_complete_session_judgment_score_v1 (P : access constant thinkthen_complete_session_judgment_score_v1) return Value;
function Read_thinkthen_complete_session_judgment_tags_v1 (P : access constant thinkthen_complete_session_judgment_tags_v1) return Value;
function Read_thinkthen_complete_session_observed_row_annotated_v1 (P : access constant thinkthen_complete_session_observed_row_annotated_v1) return Value;
function Read_thinkthen_complete_session_annotation_list_v1 (P : thinkthen_complete_session_annotation_list_v1) return Value;
function Read_thinkthen_complete_session_annotation_v1 (P : access constant thinkthen_complete_session_annotation_v1) return Value;
function Read_thinkthen_complete_annotation_value_v1 (P : access constant thinkthen_complete_annotation_value_v1) return Value;
function Read_thinkthen_complete_annotation_value_decision_v1 (P : access constant thinkthen_complete_annotation_value_decision_v1) return Value;
function Read_thinkthen_complete_annotation_value_choice_v1 (P : access constant thinkthen_complete_annotation_value_choice_v1) return Value;
function Read_thinkthen_complete_annotation_value_score_v1 (P : access constant thinkthen_complete_annotation_value_score_v1) return Value;
function Read_thinkthen_complete_annotation_value_tags_v1 (P : access constant thinkthen_complete_annotation_value_tags_v1) return Value;
function Read_thinkthen_complete_annotation_value_failed_v1 (P : access constant thinkthen_complete_annotation_value_failed_v1) return Value;
function Read_thinkthen_complete_session_observed_row_recognized_v1 (P : access constant thinkthen_complete_session_observed_row_recognized_v1) return Value;
function Read_thinkthen_complete_session_recognition_v1 (P : access constant thinkthen_complete_session_recognition_v1) return Value;
function Read_thinkthen_complete_recognition_edge_list_v1 (P : thinkthen_complete_recognition_edge_list_v1) return Value;
function Read_thinkthen_complete_recognition_edge_v1 (P : access constant thinkthen_complete_recognition_edge_v1) return Value;
function Read_thinkthen_complete_session_observed_row_find_v1 (P : access constant thinkthen_complete_session_observed_row_find_v1) return Value;
function Read_thinkthen_complete_session_observed_row_relations_v1 (P : access constant thinkthen_complete_session_observed_row_relations_v1) return Value;
function Read_thinkthen_complete_session_relation_edge_list_v1 (P : thinkthen_complete_session_relation_edge_list_v1) return Value;
function Read_thinkthen_complete_session_relation_edge_v1 (P : access constant thinkthen_complete_session_relation_edge_v1) return Value;
function Read_thinkthen_complete_entity_document_v1 (P : access constant thinkthen_complete_entity_document_v1) return Value;
function Read_thinkthen_complete_session_packet_terminal_v1 (P : access constant thinkthen_complete_session_packet_terminal_v1) return Value;
function Read_thinkthen_complete_facts_v1 (P : access constant thinkthen_complete_facts_v1) return Value;
function Read_thinkthen_complete_call_id_v1 (P : access constant thinkthen_complete_call_id_v1) return Value;
function Read_thinkthen_complete_persistence_observation_v1 (P : access constant thinkthen_complete_persistence_observation_v1) return Value;
function Read_thinkthen_complete_usage_persistence_v1 (P : access constant thinkthen_complete_usage_persistence_v1) return Value;
function Read_thinkthen_complete_call_error_v1 (P : access constant thinkthen_complete_call_error_v1) return Value;
function Read_thinkthen_complete_error_v1 (P : access constant thinkthen_complete_error_v1) return Value;
function Read_thinkthen_complete_input_denial_v1 (P : access constant thinkthen_complete_input_denial_v1) return Value;
function Read_thinkthen_complete_input_denial_initial_request_v1 (P : access constant thinkthen_complete_input_denial_initial_request_v1) return Value;
function Read_thinkthen_complete_input_denial_additional_request_v1 (P : access constant thinkthen_complete_input_denial_additional_request_v1) return Value;
function Read_thinkthen_complete_input_denial_retry_v1 (P : access constant thinkthen_complete_input_denial_retry_v1) return Value;
function Read_thinkthen_complete_failure_kind_v1 (P : access constant thinkthen_complete_failure_kind_v1) return Value;
function Read_thinkthen_complete_send_denial_v1 (P : access constant thinkthen_complete_send_denial_v1) return Value;
function Read_thinkthen_complete_send_denial_before_first_send_v1 (P : access constant thinkthen_complete_send_denial_before_first_send_v1) return Value;
function Read_thinkthen_complete_send_denial_before_additional_send_v1 (P : access constant thinkthen_complete_send_denial_before_additional_send_v1) return Value;
function Read_thinkthen_complete_send_denial_before_retry_v1 (P : access constant thinkthen_complete_send_denial_before_retry_v1) return Value;
function Read_thinkthen_complete_stopped_v1 (P : access constant thinkthen_complete_stopped_v1) return Value;
function Read_thinkthen_complete_stop_cause_v1 (P : access constant thinkthen_complete_stop_cause_v1) return Value;
function Read_Json (P : access constant thinkthen_complete_json_v1) return Value renames Thinkthen.Native_Values.Read;
function Read_thinkthen_complete_answer_yes_no_v1 (P : access constant thinkthen_complete_answer_yes_no_v1) return Value is
Fields : Field_List (1 .. 2 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("kind", String_Item (Text (P.kind)));
Count := Count + 1; Fields (Count) := Named ("probability", Number_Item (Ada.Strings.Fixed.Trim (double'Image (P.probability), Ada.Strings.Both)));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_answer_yes_no_v1;
function Read_thinkthen_complete_number_map_v1 (P : thinkthen_complete_number_map_v1) return Value is
Count : constant Natural := Index (P.len);
type Items_Type is array (1 .. Count) of thinkthen_complete_number_map_entry_v1 with Convention => C;
type Items_Access is access constant Items_Type;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Items_Access);
Items : Items_Access;
Fields : Field_List (1 .. Count);
begin
if Count > 0 then Items := Pointer (P.data.all'Address); end if;
for I in 1 .. Count loop
Fields (I) := Named (Text (Items (I).name), Number_Item (Ada.Strings.Fixed.Trim (double'Image (Items (I).value), Ada.Strings.Both)));
end loop;
return Object_Item (Fields);
end Read_thinkthen_complete_number_map_v1;
function Read_thinkthen_complete_answer_choice_v1 (P : access constant thinkthen_complete_answer_choice_v1) return Value is
Fields : Field_List (1 .. 4 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
case P.confidence.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("confidence", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("confidence", Number_Item (Ada.Strings.Fixed.Trim (double'Image (P.confidence.value), Ada.Strings.Both)));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("kind", String_Item (Text (P.kind)));
Count := Count + 1; Fields (Count) := Named ("pick", String_Item (Text (P.pick)));
Count := Count + 1; Fields (Count) := Named ("probabilities", Read_thinkthen_complete_number_map_v1(P.probabilities));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_answer_choice_v1;
function Read_thinkthen_complete_answer_tag_v1 (P : access constant thinkthen_complete_answer_tag_v1) return Value is
Fields : Field_List (1 .. 2 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("kind", String_Item (Text (P.kind)));
Count := Count + 1; Fields (Count) := Named ("probabilities", Read_thinkthen_complete_number_map_v1(P.probabilities));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_answer_tag_v1;
function Read_thinkthen_complete_answer_score_v1 (P : access constant thinkthen_complete_answer_score_v1) return Value is
Fields : Field_List (1 .. 4 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
case P.confidence.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("confidence", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("confidence", Number_Item (Ada.Strings.Fixed.Trim (double'Image (P.confidence.value), Ada.Strings.Both)));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("kind", String_Item (Text (P.kind)));
Count := Count + 1; Fields (Count) := Named ("level", String_Item (Text (P.level)));
Count := Count + 1; Fields (Count) := Named ("probabilities", Read_thinkthen_complete_number_map_v1(P.probabilities));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_answer_score_v1;
function Read_thinkthen_complete_answer_v1 (P : access constant thinkthen_complete_answer_v1) return Value is

begin
if P = null then raise Constraint_Error with "missing native value"; end if;
case P.kind is
when 1 => return Read_thinkthen_complete_answer_yes_no_v1 (P.data.yes_no);
when 2 => return Read_thinkthen_complete_answer_choice_v1 (P.data.choice);
when 3 => return Read_thinkthen_complete_answer_tag_v1 (P.data.tag);
when 4 => return Read_thinkthen_complete_answer_score_v1 (P.data.score);
when others => raise Constraint_Error with "invalid native alternative";
end case;
end Read_thinkthen_complete_answer_v1;
function Read_thinkthen_complete_answer_id_v1 (P : access constant thinkthen_complete_answer_id_v1) return Value is

begin
if P = null then raise Constraint_Error with "missing native value"; end if;
return String_Item (Text (P.value));
end Read_thinkthen_complete_answer_id_v1;
function Read_thinkthen_complete_image_media_v1 (P : access constant thinkthen_complete_image_media_v1) return Value is

begin
if P = null then raise Constraint_Error with "missing native value"; end if;
case P.kind is
when 1 => return String_Item ("image/jpeg");
when 2 => return String_Item ("image/png");
when others => raise Constraint_Error with "invalid native enum";
end case;
end Read_thinkthen_complete_image_media_v1;
function Read_thinkthen_complete_image_v1 (P : access constant thinkthen_complete_image_v1) return Value is
Fields : Field_List (1 .. 4 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("base64", String_Item (Text (P.base64)));
Count := Count + 1; Fields (Count) := Named ("height", Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.height), Ada.Strings.Both)));
Count := Count + 1; Fields (Count) := Named ("media", Read_thinkthen_complete_image_media_v1(P.media));
Count := Count + 1; Fields (Count) := Named ("width", Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.width), Ada.Strings.Both)));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_image_v1;
function Read_thinkthen_complete_image_list_v1 (P : thinkthen_complete_image_list_v1) return Value is
Count : constant Natural := Index (P.len);
type Items_Type is array (1 .. Count) of access constant thinkthen_complete_image_v1 with Convention => C;
type Items_Access is access constant Items_Type;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Items_Access);
Items : Items_Access;
Items_Copy : Value_List (1 .. Count);
begin
if Count > 0 then Items := Pointer (P.data); end if;
for I in 1 .. Count loop
Items_Copy (I) := Read_thinkthen_complete_image_v1(Items (I));
end loop;
return Array_Item (Items_Copy);
end Read_thinkthen_complete_image_list_v1;
function Read_thinkthen_complete_attempt_outcome_v1 (P : access constant thinkthen_complete_attempt_outcome_v1) return Value is

begin
if P = null then raise Constraint_Error with "missing native value"; end if;
case P.kind is
when 1 => return String_Item ("ok");
when 2 => return String_Item ("status");
when 3 => return String_Item ("transport");
when others => raise Constraint_Error with "invalid native enum";
end case;
end Read_thinkthen_complete_attempt_outcome_v1;
function Read_thinkthen_complete_sdk_request_id_v1 (P : access constant thinkthen_complete_sdk_request_id_v1) return Value is

begin
if P = null then raise Constraint_Error with "missing native value"; end if;
return String_Item (Text (P.value));
end Read_thinkthen_complete_sdk_request_id_v1;
function Read_thinkthen_complete_attempt_v1 (P : access constant thinkthen_complete_attempt_v1) return Value is
Fields : Field_List (1 .. 8 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("ordinal", Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.ordinal), Ada.Strings.Both)));
Count := Count + 1; Fields (Count) := Named ("outcome", Read_thinkthen_complete_attempt_outcome_v1(P.outcome));
case P.request_id.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("request_id", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("request_id", String_Item (Text (P.request_id.value)));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("request_sha256", String_Item (Text (P.request_sha256)));
Count := Count + 1; Fields (Count) := Named ("sdk_request_id", Read_thinkthen_complete_sdk_request_id_v1(P.sdk_request_id));
case P.server_ms.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("server_ms", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("server_ms", Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.server_ms.value), Ada.Strings.Both)));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.status.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("status", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("status", Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.status.value), Ada.Strings.Both)));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("wall_ms", Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.wall_ms), Ada.Strings.Both)));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_attempt_v1;
function Read_thinkthen_complete_attempt_list_v1 (P : thinkthen_complete_attempt_list_v1) return Value is
Count : constant Natural := Index (P.len);
type Items_Type is array (1 .. Count) of access constant thinkthen_complete_attempt_v1 with Convention => C;
type Items_Access is access constant Items_Type;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Items_Access);
Items : Items_Access;
Items_Copy : Value_List (1 .. Count);
begin
if Count > 0 then Items := Pointer (P.data); end if;
for I in 1 .. Count loop
Items_Copy (I) := Read_thinkthen_complete_attempt_v1(Items (I));
end loop;
return Array_Item (Items_Copy);
end Read_thinkthen_complete_attempt_list_v1;
function Read_thinkthen_complete_batch_setting_v1 (P : access constant thinkthen_complete_batch_setting_v1) return Value is

begin
if P = null then raise Constraint_Error with "missing native value"; end if;
case P.kind is
when 1 => return Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.data.integer), Ada.Strings.Both));
when 2 => return String_Item (Text (P.data.string));
when others => raise Constraint_Error with "invalid native alternative";
end case;
end Read_thinkthen_complete_batch_setting_v1;
function Read_thinkthen_complete_batch_warning_v1 (P : access constant thinkthen_complete_batch_warning_v1) return Value is
Fields : Field_List (1 .. 2 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("running", Read_thinkthen_complete_batch_setting_v1(P.running));
Count := Count + 1; Fields (Count) := Named ("tuned_for", Read_thinkthen_complete_batch_setting_v1(P.tuned_for));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_batch_warning_v1;
function Read_thinkthen_complete_observation_id_v1 (P : access constant thinkthen_complete_observation_id_v1) return Value is

begin
if P = null then raise Constraint_Error with "missing native value"; end if;
return String_Item (Text (P.value));
end Read_thinkthen_complete_observation_id_v1;
function Read_thinkthen_complete_observation_observation_id_v1 (P : access constant thinkthen_complete_observation_observation_id_v1) return Value is
Fields : Field_List (1 .. 1 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("observation_id", Read_thinkthen_complete_observation_id_v1(P.observation_id));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_observation_observation_id_v1;
function Read_thinkthen_complete_failure_id_v1 (P : access constant thinkthen_complete_failure_id_v1) return Value is

begin
if P = null then raise Constraint_Error with "missing native value"; end if;
return String_Item (Text (P.value));
end Read_thinkthen_complete_failure_id_v1;
function Read_thinkthen_complete_observation_failure_id_v1 (P : access constant thinkthen_complete_observation_failure_id_v1) return Value is
Fields : Field_List (1 .. 1 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("failure_id", Read_thinkthen_complete_failure_id_v1(P.failure_id));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_observation_failure_id_v1;
function Read_thinkthen_complete_observation_v1 (P : access constant thinkthen_complete_observation_v1) return Value is

begin
if P = null then raise Constraint_Error with "missing native value"; end if;
case P.kind is
when 1 => return Read_thinkthen_complete_observation_observation_id_v1 (P.data.observation_id);
when 2 => return Read_thinkthen_complete_observation_failure_id_v1 (P.data.failure_id);
when others => raise Constraint_Error with "invalid native alternative";
end case;
end Read_thinkthen_complete_observation_v1;
function Read_thinkthen_complete_observation_list_v1 (P : thinkthen_complete_observation_list_v1) return Value is
Count : constant Natural := Index (P.len);
type Items_Type is array (1 .. Count) of access constant thinkthen_complete_observation_v1 with Convention => C;
type Items_Access is access constant Items_Type;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Items_Access);
Items : Items_Access;
Items_Copy : Value_List (1 .. Count);
begin
if Count > 0 then Items := Pointer (P.data); end if;
for I in 1 .. Count loop
Items_Copy (I) := Read_thinkthen_complete_observation_v1(Items (I));
end loop;
return Array_Item (Items_Copy);
end Read_thinkthen_complete_observation_list_v1;
function Read_thinkthen_complete_origin_v1 (P : access constant thinkthen_complete_origin_v1) return Value is

begin
if P = null then raise Constraint_Error with "missing native value"; end if;
case P.kind is
when 1 => return String_Item ("live");
when 2 => return String_Item ("cache");
when 3 => return String_Item ("replay");
when 4 => return String_Item ("proxy");
when 5 => return String_Item ("memory");
when others => raise Constraint_Error with "invalid native enum";
end case;
end Read_thinkthen_complete_origin_v1;
function Read_thinkthen_complete_profile_warning_v1 (P : access constant thinkthen_complete_profile_warning_v1) return Value is
Fields : Field_List (1 .. 2 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("running", String_Item (Text (P.running)));
Count := Count + 1; Fields (Count) := Named ("tuned_for", String_Item (Text (P.tuned_for)));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_profile_warning_v1;
function Read_thinkthen_complete_question_source_v1 (P : access constant thinkthen_complete_question_source_v1) return Value is
Fields : Field_List (1 .. 3 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("answered_by", String_Item (Text (P.answered_by)));
case P.batch_size.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("batch_size", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("batch_size", Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.batch_size.value), Ada.Strings.Both)));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("origin", Read_thinkthen_complete_origin_v1(P.origin));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_question_source_v1;
function Read_thinkthen_complete_question_source_list_v1 (P : thinkthen_complete_question_source_list_v1) return Value is
Count : constant Natural := Index (P.len);
type Items_Type is array (1 .. Count) of access constant thinkthen_complete_question_source_v1 with Convention => C;
type Items_Access is access constant Items_Type;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Items_Access);
Items : Items_Access;
Items_Copy : Value_List (1 .. Count);
begin
if Count > 0 then Items := Pointer (P.data); end if;
for I in 1 .. Count loop
Items_Copy (I) := Read_thinkthen_complete_question_source_v1(Items (I));
end loop;
return Array_Item (Items_Copy);
end Read_thinkthen_complete_question_source_list_v1;
function Read_thinkthen_complete_string_list_v1 (P : thinkthen_complete_string_list_v1) return Value is
Count : constant Natural := Index (P.len);
type Items_Type is array (1 .. Count) of thinkthen_complete_utf8_v1 with Convention => C;
type Items_Access is access constant Items_Type;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Items_Access);
Items : Items_Access;
Items_Copy : Value_List (1 .. Count);
begin
if Count > 0 then Items := Pointer (P.data.all'Address); end if;
for I in 1 .. Count loop
Items_Copy (I) := String_Item (Text (Items (I)));
end loop;
return Array_Item (Items_Copy);
end Read_thinkthen_complete_string_list_v1;
function Read_thinkthen_complete_usage_v1 (P : access constant thinkthen_complete_usage_v1) return Value is
Fields : Field_List (1 .. 2 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
case P.input_tokens.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("input_tokens", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("input_tokens", Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.input_tokens.value), Ada.Strings.Both)));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.output_tokens.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("output_tokens", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("output_tokens", Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.output_tokens.value), Ada.Strings.Both)));
when others => raise Constraint_Error with "invalid native presence";
end case;
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_usage_v1;
function Read_thinkthen_complete_meta_v1 (P : access constant thinkthen_complete_meta_v1) return Value is
Fields : Field_List (1 .. 19 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
case P.answered_by.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("answered_by", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("answered_by", String_Item (Text (P.answered_by.value)));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.attempts.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("attempts", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("attempts", Read_thinkthen_complete_attempt_list_v1(P.attempts.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.batch_setting.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("batch_setting", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("batch_setting", Read_thinkthen_complete_batch_setting_v1(P.batch_setting.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.batch_warning.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("batch_warning", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("batch_warning", Read_thinkthen_complete_batch_warning_v1(P.batch_warning.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("cached", Boolean_Item (P.cached /= 0));
case P.context_sha256.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("context_sha256", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("context_sha256", String_Item (Text (P.context_sha256.value)));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("failed_questions", Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.failed_questions), Ada.Strings.Both)));
Count := Count + 1; Fields (Count) := Named ("model", String_Item (Text (P.model)));
Count := Count + 1; Fields (Count) := Named ("observations", Read_thinkthen_complete_observation_list_v1(P.observations));
case P.origin.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("origin", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("origin", Read_thinkthen_complete_origin_v1(P.origin.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.profile_warning.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("profile_warning", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("profile_warning", Read_thinkthen_complete_profile_warning_v1(P.profile_warning.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.question_sha256.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("question_sha256", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("question_sha256", String_Item (Text (P.question_sha256.value)));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("question_sources", Read_thinkthen_complete_question_source_list_v1(P.question_sources));
case P.questions_sha256.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("questions_sha256", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("questions_sha256", String_Item (Text (P.questions_sha256.value)));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("requests", Read_thinkthen_complete_string_list_v1(P.requests));
Count := Count + 1; Fields (Count) := Named ("requests_sent", Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.requests_sent), Ada.Strings.Both)));
Count := Count + 1; Fields (Count) := Named ("tool", String_Item (Text (P.tool)));
Count := Count + 1; Fields (Count) := Named ("url", String_Item (Text (P.url)));
case P.usage.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("usage", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("usage", Read_thinkthen_complete_usage_v1(P.usage.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_meta_v1;
function Read_thinkthen_complete_batch_string_v1 (P : access constant thinkthen_complete_batch_string_v1) return Value is

begin
if P = null then raise Constraint_Error with "missing native value"; end if;
case P.kind is
when 1 => return String_Item ("max");
when others => raise Constraint_Error with "invalid native enum";
end case;
end Read_thinkthen_complete_batch_string_v1;
function Read_thinkthen_complete_batch_v1 (P : access constant thinkthen_complete_batch_v1) return Value is

begin
if P = null then raise Constraint_Error with "missing native value"; end if;
case P.kind is
when 1 => return Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.data.integer), Ada.Strings.Both));
when 2 => return Read_thinkthen_complete_batch_string_v1(P.data.string);
when others => raise Constraint_Error with "invalid native alternative";
end case;
end Read_thinkthen_complete_batch_v1;
function Read_thinkthen_complete_string_type_v1 (P : access constant thinkthen_complete_string_type_v1) return Value is

begin
if P = null then raise Constraint_Error with "missing native value"; end if;
case P.kind is
when 1 => return String_Item ("string");
when others => raise Constraint_Error with "invalid native enum";
end case;
end Read_thinkthen_complete_string_type_v1;
function Read_thinkthen_complete_input_declaration_string_root_v1 (P : access constant thinkthen_complete_input_declaration_string_root_v1) return Value is
Fields : Field_List (1 .. 1 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("type", Read_thinkthen_complete_string_type_v1(P.c_type));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_input_declaration_string_root_v1;
function Read_thinkthen_complete_input_property_type_string_v1 (P : access constant thinkthen_complete_input_property_type_string_v1) return Value is
Fields : Field_List (1 .. 1 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("type", String_Item (Text (P.c_type)));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_input_property_type_string_v1;
function Read_thinkthen_complete_input_property_type_number_v1 (P : access constant thinkthen_complete_input_property_type_number_v1) return Value is
Fields : Field_List (1 .. 1 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("type", String_Item (Text (P.c_type)));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_input_property_type_number_v1;
function Read_thinkthen_complete_input_property_type_boolean_v1 (P : access constant thinkthen_complete_input_property_type_boolean_v1) return Value is
Fields : Field_List (1 .. 1 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("type", String_Item (Text (P.c_type)));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_input_property_type_boolean_v1;
function Read_thinkthen_complete_string_root_v1 (P : access constant thinkthen_complete_string_root_v1) return Value is
Fields : Field_List (1 .. 1 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("type", Read_thinkthen_complete_string_type_v1(P.c_type));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_string_root_v1;
function Read_thinkthen_complete_input_property_type_array_v1 (P : access constant thinkthen_complete_input_property_type_array_v1) return Value is
Fields : Field_List (1 .. 2 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("items", Read_thinkthen_complete_string_root_v1(P.items));
Count := Count + 1; Fields (Count) := Named ("type", String_Item (Text (P.c_type)));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_input_property_type_array_v1;
function Read_thinkthen_complete_input_property_type_v1 (P : access constant thinkthen_complete_input_property_type_v1) return Value is

begin
if P = null then raise Constraint_Error with "missing native value"; end if;
case P.kind is
when 1 => return Read_thinkthen_complete_input_property_type_string_v1 (P.data.string);
when 2 => return Read_thinkthen_complete_input_property_type_number_v1 (P.data.number);
when 3 => return Read_thinkthen_complete_input_property_type_boolean_v1 (P.data.boolean);
when 4 => return Read_thinkthen_complete_input_property_type_array_v1 (P.data.c_array);
when others => raise Constraint_Error with "invalid native alternative";
end case;
end Read_thinkthen_complete_input_property_type_v1;
function Read_thinkthen_complete_input_property_type_map_v1 (P : thinkthen_complete_input_property_type_map_v1) return Value is
Count : constant Natural := Index (P.len);
type Items_Type is array (1 .. Count) of thinkthen_complete_input_property_type_map_entry_v1 with Convention => C;
type Items_Access is access constant Items_Type;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Items_Access);
Items : Items_Access;
Fields : Field_List (1 .. Count);
begin
if Count > 0 then Items := Pointer (P.data.all'Address); end if;
for I in 1 .. Count loop
Fields (I) := Named (Text (Items (I).name), Read_thinkthen_complete_input_property_type_v1(Items (I).value));
end loop;
return Object_Item (Fields);
end Read_thinkthen_complete_input_property_type_map_v1;
function Read_thinkthen_complete_object_type_v1 (P : access constant thinkthen_complete_object_type_v1) return Value is

begin
if P = null then raise Constraint_Error with "missing native value"; end if;
case P.kind is
when 1 => return String_Item ("object");
when others => raise Constraint_Error with "invalid native enum";
end case;
end Read_thinkthen_complete_object_type_v1;
function Read_thinkthen_complete_input_declaration_object_root_v1 (P : access constant thinkthen_complete_input_declaration_object_root_v1) return Value is
Fields : Field_List (1 .. 3 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("properties", Read_thinkthen_complete_input_property_type_map_v1(P.properties));
case P.required.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("required", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("required", Read_thinkthen_complete_string_list_v1(P.required.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("type", Read_thinkthen_complete_object_type_v1(P.c_type));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_input_declaration_object_root_v1;
function Read_thinkthen_complete_input_declaration_v1 (P : access constant thinkthen_complete_input_declaration_v1) return Value is

begin
if P = null then raise Constraint_Error with "missing native value"; end if;
case P.kind is
when 1 => return Read_thinkthen_complete_input_declaration_string_root_v1 (P.data.string_root);
when 2 => return Read_thinkthen_complete_input_declaration_object_root_v1 (P.data.object_root);
when others => raise Constraint_Error with "invalid native alternative";
end case;
end Read_thinkthen_complete_input_declaration_v1;
function Read_thinkthen_complete_label_v1 (P : access constant thinkthen_complete_label_v1) return Value is
Fields : Field_List (1 .. 2 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
case P.description.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("description", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("description", Read_Json (P.description.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("name", String_Item (Text (P.name)));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_label_v1;
function Read_thinkthen_complete_label_list_v1 (P : thinkthen_complete_label_list_v1) return Value is
Count : constant Natural := Index (P.len);
type Items_Type is array (1 .. Count) of access constant thinkthen_complete_label_v1 with Convention => C;
type Items_Access is access constant Items_Type;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Items_Access);
Items : Items_Access;
Items_Copy : Value_List (1 .. Count);
begin
if Count > 0 then Items := Pointer (P.data); end if;
for I in 1 .. Count loop
Items_Copy (I) := Read_thinkthen_complete_label_v1(Items (I));
end loop;
return Array_Item (Items_Copy);
end Read_thinkthen_complete_label_list_v1;
function Read_thinkthen_complete_question_name_v1 (P : access constant thinkthen_complete_question_name_v1) return Value is

begin
if P = null then raise Constraint_Error with "missing native value"; end if;
return String_Item (Text (P.value));
end Read_thinkthen_complete_question_name_v1;
function Read_thinkthen_complete_wording_version_v1 (P : access constant thinkthen_complete_wording_version_v1) return Value is

begin
if P = null then raise Constraint_Error with "missing native value"; end if;
return Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.value), Ada.Strings.Both));
end Read_thinkthen_complete_wording_version_v1;
function Read_thinkthen_complete_question_document_decide_v1 (P : access constant thinkthen_complete_question_document_decide_v1) return Value is
Fields : Field_List (1 .. 13 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
case P.batch.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("batch", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("batch", Read_thinkthen_complete_batch_v1(P.batch.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.context_schema.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("context_schema", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("context_schema", Read_thinkthen_complete_input_declaration_v1(P.context_schema.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.item_schema.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("item_schema", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("item_schema", Read_thinkthen_complete_input_declaration_v1(P.item_schema.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.label_details.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("label_details", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("label_details", Read_thinkthen_complete_label_list_v1(P.label_details.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.model.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("model", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("model", String_Item (Text (P.model.value)));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.name.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("name", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("name", Read_thinkthen_complete_question_name_v1(P.name.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.on.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("on", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("on", Read_thinkthen_complete_string_list_v1(P.on.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.profile.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("profile", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("profile", String_Item (Text (P.profile.value)));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.wording_version.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("wording_version", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("wording_version", Read_thinkthen_complete_wording_version_v1(P.wording_version.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.false_u.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("false", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("false", Read_Json (P.false_u.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.text.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("text", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("text", Read_Json (P.text.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.true_u.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("true", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("true", Read_Json (P.true_u.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("verb", String_Item (Text (P.verb)));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_question_document_decide_v1;
function Read_thinkthen_complete_question_document_choose_v1 (P : access constant thinkthen_complete_question_document_choose_v1) return Value is
Fields : Field_List (1 .. 12 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
case P.batch.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("batch", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("batch", Read_thinkthen_complete_batch_v1(P.batch.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.context_schema.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("context_schema", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("context_schema", Read_thinkthen_complete_input_declaration_v1(P.context_schema.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.item_schema.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("item_schema", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("item_schema", Read_thinkthen_complete_input_declaration_v1(P.item_schema.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.label_details.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("label_details", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("label_details", Read_thinkthen_complete_label_list_v1(P.label_details.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.model.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("model", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("model", String_Item (Text (P.model.value)));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.name.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("name", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("name", Read_thinkthen_complete_question_name_v1(P.name.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.on.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("on", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("on", Read_thinkthen_complete_string_list_v1(P.on.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.profile.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("profile", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("profile", String_Item (Text (P.profile.value)));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.wording_version.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("wording_version", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("wording_version", Read_thinkthen_complete_wording_version_v1(P.wording_version.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("options", Read_thinkthen_complete_string_list_v1(P.options));
case P.text.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("text", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("text", Read_Json (P.text.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("verb", String_Item (Text (P.verb)));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_question_document_choose_v1;
function Read_thinkthen_complete_question_document_tag_v1 (P : access constant thinkthen_complete_question_document_tag_v1) return Value is
Fields : Field_List (1 .. 12 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
case P.batch.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("batch", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("batch", Read_thinkthen_complete_batch_v1(P.batch.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.context_schema.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("context_schema", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("context_schema", Read_thinkthen_complete_input_declaration_v1(P.context_schema.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.item_schema.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("item_schema", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("item_schema", Read_thinkthen_complete_input_declaration_v1(P.item_schema.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.label_details.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("label_details", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("label_details", Read_thinkthen_complete_label_list_v1(P.label_details.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.model.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("model", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("model", String_Item (Text (P.model.value)));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.name.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("name", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("name", Read_thinkthen_complete_question_name_v1(P.name.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.on.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("on", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("on", Read_thinkthen_complete_string_list_v1(P.on.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.profile.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("profile", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("profile", String_Item (Text (P.profile.value)));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.wording_version.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("wording_version", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("wording_version", Read_thinkthen_complete_wording_version_v1(P.wording_version.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("labels", Read_thinkthen_complete_string_list_v1(P.labels));
case P.text.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("text", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("text", Read_Json (P.text.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("verb", String_Item (Text (P.verb)));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_question_document_tag_v1;
function Read_thinkthen_complete_question_document_score_v1 (P : access constant thinkthen_complete_question_document_score_v1) return Value is
Fields : Field_List (1 .. 12 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
case P.batch.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("batch", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("batch", Read_thinkthen_complete_batch_v1(P.batch.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.context_schema.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("context_schema", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("context_schema", Read_thinkthen_complete_input_declaration_v1(P.context_schema.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.item_schema.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("item_schema", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("item_schema", Read_thinkthen_complete_input_declaration_v1(P.item_schema.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.label_details.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("label_details", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("label_details", Read_thinkthen_complete_label_list_v1(P.label_details.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.model.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("model", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("model", String_Item (Text (P.model.value)));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.name.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("name", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("name", Read_thinkthen_complete_question_name_v1(P.name.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.on.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("on", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("on", Read_thinkthen_complete_string_list_v1(P.on.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.profile.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("profile", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("profile", String_Item (Text (P.profile.value)));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.wording_version.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("wording_version", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("wording_version", Read_thinkthen_complete_wording_version_v1(P.wording_version.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("levels", Read_thinkthen_complete_string_list_v1(P.levels));
case P.text.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("text", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("text", Read_Json (P.text.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("verb", String_Item (Text (P.verb)));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_question_document_score_v1;
function Read_thinkthen_complete_question_document_v1 (P : access constant thinkthen_complete_question_document_v1) return Value is

begin
if P = null then raise Constraint_Error with "missing native value"; end if;
case P.kind is
when 1 => return Read_thinkthen_complete_question_document_decide_v1 (P.data.decide);
when 2 => return Read_thinkthen_complete_question_document_choose_v1 (P.data.choose);
when 3 => return Read_thinkthen_complete_question_document_tag_v1 (P.data.tag);
when 4 => return Read_thinkthen_complete_question_document_score_v1 (P.data.score);
when others => raise Constraint_Error with "invalid native alternative";
end case;
end Read_thinkthen_complete_question_document_v1;
function Read_thinkthen_complete_version_v1 (P : access constant thinkthen_complete_version_v1) return Value is

begin
if P = null then raise Constraint_Error with "missing native value"; end if;
case P.kind is
when 1 => return String_Item ("thinkthen.result/2");
when others => raise Constraint_Error with "invalid native enum";
end case;
end Read_thinkthen_complete_version_v1;
function Read_thinkthen_complete_physical_source_v1 (P : access constant thinkthen_complete_physical_source_v1) return Value is
Fields : Field_List (1 .. 3 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("file", String_Item (Text (P.file)));
case P.first_line.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("first_line", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("first_line", Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.first_line.value), Ada.Strings.Both)));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.last_line.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("last_line", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("last_line", Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.last_line.value), Ada.Strings.Both)));
when others => raise Constraint_Error with "invalid native presence";
end case;
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_physical_source_v1;
function Read_thinkthen_complete_rank_member_result_v1 (P : access constant thinkthen_complete_rank_member_result_v1) return Value is
Fields : Field_List (1 .. 9 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("answer", Read_thinkthen_complete_answer_v1(P.answer));
Count := Count + 1; Fields (Count) := Named ("answer_id", Read_thinkthen_complete_answer_id_v1(P.answer_id));
case P.images.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("images", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("images", Read_thinkthen_complete_image_list_v1(P.images.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("meta", Read_thinkthen_complete_meta_v1(P.meta));
Count := Count + 1; Fields (Count) := Named ("question", Read_thinkthen_complete_question_document_v1(P.question));
Count := Count + 1; Fields (Count) := Named ("schema", Read_thinkthen_complete_version_v1(P.schema));
case P.source.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("source", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("source", Read_thinkthen_complete_physical_source_v1(P.source.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.threshold.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("threshold", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("threshold", Read_Json (P.threshold.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("value", Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.value), Ada.Strings.Both)));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_rank_member_result_v1;
function Read_thinkthen_complete_rank_member_v1 (P : access constant thinkthen_complete_rank_member_v1) return Value is
Fields : Field_List (1 .. 2 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("name", String_Item (Text (P.name)));
Count := Count + 1; Fields (Count) := Named ("result", Read_thinkthen_complete_rank_member_result_v1(P.result));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_rank_member_v1;
function Read_thinkthen_complete_rank_member_list_v1 (P : thinkthen_complete_rank_member_list_v1) return Value is
Count : constant Natural := Index (P.len);
type Items_Type is array (1 .. Count) of access constant thinkthen_complete_rank_member_v1 with Convention => C;
type Items_Access is access constant Items_Type;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Items_Access);
Items : Items_Access;
Items_Copy : Value_List (1 .. Count);
begin
if Count > 0 then Items := Pointer (P.data); end if;
for I in 1 .. Count loop
Items_Copy (I) := Read_thinkthen_complete_rank_member_v1(Items (I));
end loop;
return Array_Item (Items_Copy);
end Read_thinkthen_complete_rank_member_list_v1;
function Read_thinkthen_complete_threshold_v1 (P : access constant thinkthen_complete_threshold_v1) return Value is

begin
if P = null then raise Constraint_Error with "missing native value"; end if;
case P.kind is
when 1 => return Number_Item (Ada.Strings.Fixed.Trim (double'Image (P.data.number), Ada.Strings.Both));
when 2 => return String_Item (Text (P.data.string));
when others => raise Constraint_Error with "invalid native alternative";
end case;
end Read_thinkthen_complete_threshold_v1;
function Read_thinkthen_complete_decide_value_v1 (P : access constant thinkthen_complete_decide_value_v1) return Value is

begin
if P = null then raise Constraint_Error with "missing native value"; end if;
return Read_Json (P.value);
end Read_thinkthen_complete_decide_value_v1;
function Read_thinkthen_complete_atomic_decide_value_v1 (P : access constant thinkthen_complete_atomic_decide_value_v1) return Value is
Fields : Field_List (1 .. 13 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("answer", Read_thinkthen_complete_answer_v1(P.answer));
Count := Count + 1; Fields (Count) := Named ("answer_id", Read_thinkthen_complete_answer_id_v1(P.answer_id));
case P.images.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("images", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("images", Read_thinkthen_complete_image_list_v1(P.images.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.index.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("index", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("index", Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.index.value), Ada.Strings.Both)));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.input.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("input", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("input", Read_Json (P.input.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.members.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("members", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("members", Read_thinkthen_complete_rank_member_list_v1(P.members.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("meta", Read_thinkthen_complete_meta_v1(P.meta));
Count := Count + 1; Fields (Count) := Named ("question", Read_thinkthen_complete_question_document_v1(P.question));
case P.question_name.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("question_name", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("question_name", String_Item (Text (P.question_name.value)));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("schema", Read_thinkthen_complete_version_v1(P.schema));
case P.source.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("source", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("source", Read_thinkthen_complete_physical_source_v1(P.source.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.threshold.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("threshold", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("threshold", Read_thinkthen_complete_threshold_v1(P.threshold.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.value.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("value", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("value", Read_thinkthen_complete_decide_value_v1(P.value.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_atomic_decide_value_v1;
function Read_thinkthen_complete_session_packet_decide_row_v1 (P : access constant thinkthen_complete_session_packet_decide_row_v1) return Value is
Fields : Field_List (1 .. 3 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("function", String_Item (Text (P.c_function)));
Count := Count + 1; Fields (Count) := Named ("kind", String_Item (Text (P.kind)));
Count := Count + 1; Fields (Count) := Named ("value", Read_thinkthen_complete_atomic_decide_value_v1(P.value));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_session_packet_decide_row_v1;
function Read_thinkthen_complete_atomic_nullable_string_v1 (P : access constant thinkthen_complete_atomic_nullable_string_v1) return Value is
Fields : Field_List (1 .. 13 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("answer", Read_thinkthen_complete_answer_v1(P.answer));
Count := Count + 1; Fields (Count) := Named ("answer_id", Read_thinkthen_complete_answer_id_v1(P.answer_id));
case P.images.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("images", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("images", Read_thinkthen_complete_image_list_v1(P.images.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.index.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("index", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("index", Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.index.value), Ada.Strings.Both)));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.input.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("input", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("input", Read_Json (P.input.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.members.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("members", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("members", Read_thinkthen_complete_rank_member_list_v1(P.members.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("meta", Read_thinkthen_complete_meta_v1(P.meta));
Count := Count + 1; Fields (Count) := Named ("question", Read_thinkthen_complete_question_document_v1(P.question));
case P.question_name.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("question_name", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("question_name", String_Item (Text (P.question_name.value)));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("schema", Read_thinkthen_complete_version_v1(P.schema));
case P.source.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("source", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("source", Read_thinkthen_complete_physical_source_v1(P.source.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.threshold.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("threshold", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("threshold", Read_thinkthen_complete_threshold_v1(P.threshold.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.value.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("value", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("value", String_Item (Text (P.value.value)));
when others => raise Constraint_Error with "invalid native presence";
end case;
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_atomic_nullable_string_v1;
function Read_thinkthen_complete_session_packet_choose_row_v1 (P : access constant thinkthen_complete_session_packet_choose_row_v1) return Value is
Fields : Field_List (1 .. 3 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("function", String_Item (Text (P.c_function)));
Count := Count + 1; Fields (Count) := Named ("kind", String_Item (Text (P.kind)));
Count := Count + 1; Fields (Count) := Named ("value", Read_thinkthen_complete_atomic_nullable_string_v1(P.value));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_session_packet_choose_row_v1;
function Read_thinkthen_complete_atomic_array_of_string_v1 (P : access constant thinkthen_complete_atomic_array_of_string_v1) return Value is
Fields : Field_List (1 .. 13 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("answer", Read_thinkthen_complete_answer_v1(P.answer));
Count := Count + 1; Fields (Count) := Named ("answer_id", Read_thinkthen_complete_answer_id_v1(P.answer_id));
case P.images.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("images", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("images", Read_thinkthen_complete_image_list_v1(P.images.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.index.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("index", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("index", Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.index.value), Ada.Strings.Both)));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.input.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("input", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("input", Read_Json (P.input.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.members.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("members", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("members", Read_thinkthen_complete_rank_member_list_v1(P.members.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("meta", Read_thinkthen_complete_meta_v1(P.meta));
Count := Count + 1; Fields (Count) := Named ("question", Read_thinkthen_complete_question_document_v1(P.question));
case P.question_name.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("question_name", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("question_name", String_Item (Text (P.question_name.value)));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("schema", Read_thinkthen_complete_version_v1(P.schema));
case P.source.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("source", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("source", Read_thinkthen_complete_physical_source_v1(P.source.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.threshold.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("threshold", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("threshold", Read_thinkthen_complete_threshold_v1(P.threshold.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("value", Read_thinkthen_complete_string_list_v1(P.value));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_atomic_array_of_string_v1;
function Read_thinkthen_complete_session_packet_tag_row_v1 (P : access constant thinkthen_complete_session_packet_tag_row_v1) return Value is
Fields : Field_List (1 .. 3 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("function", String_Item (Text (P.c_function)));
Count := Count + 1; Fields (Count) := Named ("kind", String_Item (Text (P.kind)));
Count := Count + 1; Fields (Count) := Named ("value", Read_thinkthen_complete_atomic_array_of_string_v1(P.value));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_session_packet_tag_row_v1;
function Read_thinkthen_complete_atomic_double_v1 (P : access constant thinkthen_complete_atomic_double_v1) return Value is
Fields : Field_List (1 .. 13 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("answer", Read_thinkthen_complete_answer_v1(P.answer));
Count := Count + 1; Fields (Count) := Named ("answer_id", Read_thinkthen_complete_answer_id_v1(P.answer_id));
case P.images.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("images", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("images", Read_thinkthen_complete_image_list_v1(P.images.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.index.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("index", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("index", Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.index.value), Ada.Strings.Both)));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.input.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("input", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("input", Read_Json (P.input.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.members.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("members", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("members", Read_thinkthen_complete_rank_member_list_v1(P.members.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("meta", Read_thinkthen_complete_meta_v1(P.meta));
Count := Count + 1; Fields (Count) := Named ("question", Read_thinkthen_complete_question_document_v1(P.question));
case P.question_name.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("question_name", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("question_name", String_Item (Text (P.question_name.value)));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("schema", Read_thinkthen_complete_version_v1(P.schema));
case P.source.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("source", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("source", Read_thinkthen_complete_physical_source_v1(P.source.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.threshold.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("threshold", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("threshold", Read_thinkthen_complete_threshold_v1(P.threshold.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("value", Number_Item (Ada.Strings.Fixed.Trim (double'Image (P.value), Ada.Strings.Both)));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_atomic_double_v1;
function Read_thinkthen_complete_session_packet_score_row_v1 (P : access constant thinkthen_complete_session_packet_score_row_v1) return Value is
Fields : Field_List (1 .. 3 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("function", String_Item (Text (P.c_function)));
Count := Count + 1; Fields (Count) := Named ("kind", String_Item (Text (P.kind)));
Count := Count + 1; Fields (Count) := Named ("value", Read_thinkthen_complete_atomic_double_v1(P.value));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_session_packet_score_row_v1;
function Read_thinkthen_complete_atomic_boolean_v1 (P : access constant thinkthen_complete_atomic_boolean_v1) return Value is
Fields : Field_List (1 .. 13 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("answer", Read_thinkthen_complete_answer_v1(P.answer));
Count := Count + 1; Fields (Count) := Named ("answer_id", Read_thinkthen_complete_answer_id_v1(P.answer_id));
case P.images.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("images", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("images", Read_thinkthen_complete_image_list_v1(P.images.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.index.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("index", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("index", Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.index.value), Ada.Strings.Both)));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.input.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("input", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("input", Read_Json (P.input.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.members.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("members", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("members", Read_thinkthen_complete_rank_member_list_v1(P.members.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("meta", Read_thinkthen_complete_meta_v1(P.meta));
Count := Count + 1; Fields (Count) := Named ("question", Read_thinkthen_complete_question_document_v1(P.question));
case P.question_name.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("question_name", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("question_name", String_Item (Text (P.question_name.value)));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("schema", Read_thinkthen_complete_version_v1(P.schema));
case P.source.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("source", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("source", Read_thinkthen_complete_physical_source_v1(P.source.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.threshold.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("threshold", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("threshold", Read_thinkthen_complete_threshold_v1(P.threshold.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("value", Boolean_Item (P.value /= 0));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_atomic_boolean_v1;
function Read_thinkthen_complete_session_packet_filter_row_v1 (P : access constant thinkthen_complete_session_packet_filter_row_v1) return Value is
Fields : Field_List (1 .. 3 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("function", String_Item (Text (P.c_function)));
Count := Count + 1; Fields (Count) := Named ("kind", String_Item (Text (P.kind)));
Count := Count + 1; Fields (Count) := Named ("value", Read_thinkthen_complete_atomic_boolean_v1(P.value));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_session_packet_filter_row_v1;
function Read_thinkthen_complete_value_v1 (P : access constant thinkthen_complete_value_v1) return Value is

begin
if P = null then raise Constraint_Error with "missing native value"; end if;
case P.kind is
when 1 => return Boolean_Item (P.data.boolean /= 0);
when 2 => return Null_Item;
when 3 => return String_Item (Text (P.data.string));
when 4 => return Read_thinkthen_complete_string_list_v1(P.data.c_array);
when 5 => return Number_Item (Ada.Strings.Fixed.Trim (double'Image (P.data.number), Ada.Strings.Both));
when others => raise Constraint_Error with "invalid native alternative";
end case;
end Read_thinkthen_complete_value_v1;
function Read_thinkthen_complete_annotation_member_answer_id_v1 (P : access constant thinkthen_complete_annotation_member_answer_id_v1) return Value is
Fields : Field_List (1 .. 9 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("answer", Read_thinkthen_complete_answer_v1(P.answer));
Count := Count + 1; Fields (Count) := Named ("answer_id", Read_thinkthen_complete_answer_id_v1(P.answer_id));
Count := Count + 1; Fields (Count) := Named ("observations", Read_thinkthen_complete_observation_list_v1(P.observations));
Count := Count + 1; Fields (Count) := Named ("question", Read_thinkthen_complete_question_document_v1(P.question));
Count := Count + 1; Fields (Count) := Named ("question_sources", Read_thinkthen_complete_question_source_list_v1(P.question_sources));
Count := Count + 1; Fields (Count) := Named ("request", String_Item (Text (P.request)));
case P.threshold.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("threshold", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("threshold", Read_thinkthen_complete_threshold_v1(P.threshold.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.usage.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("usage", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("usage", Read_thinkthen_complete_usage_v1(P.usage.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.value.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("value", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("value", Read_thinkthen_complete_value_v1(P.value.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_annotation_member_answer_id_v1;
function Read_thinkthen_complete_failure_cause_v1 (P : access constant thinkthen_complete_failure_cause_v1) return Value is

begin
if P = null then raise Constraint_Error with "missing native value"; end if;
case P.kind is
when 1 => return String_Item ("missing_answer");
when 2 => return String_Item ("wrong_kind");
when 3 => return String_Item ("missing_probability");
when 4 => return String_Item ("invalid_probability");
when 5 => return String_Item ("invalid_distribution");
when 6 => return String_Item ("unexpected_probability");
when others => raise Constraint_Error with "invalid native enum";
end case;
end Read_thinkthen_complete_failure_cause_v1;
function Read_thinkthen_complete_failure_field_kind_v1 (P : access constant thinkthen_complete_failure_field_kind_v1) return Value is

begin
if P = null then raise Constraint_Error with "missing native value"; end if;
case P.kind is
when 1 => return String_Item ("backend");
when others => raise Constraint_Error with "invalid native enum";
end case;
end Read_thinkthen_complete_failure_field_kind_v1;
function Read_thinkthen_complete_failure_v1 (P : access constant thinkthen_complete_failure_v1) return Value is
Fields : Field_List (1 .. 2 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("cause", Read_thinkthen_complete_failure_cause_v1(P.cause));
Count := Count + 1; Fields (Count) := Named ("kind", Read_thinkthen_complete_failure_field_kind_v1(P.kind));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_failure_v1;
function Read_thinkthen_complete_annotation_member_failure_id_v1 (P : access constant thinkthen_complete_annotation_member_failure_id_v1) return Value is
Fields : Field_List (1 .. 8 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("failure", Read_thinkthen_complete_failure_v1(P.failure));
Count := Count + 1; Fields (Count) := Named ("failure_id", Read_thinkthen_complete_failure_id_v1(P.failure_id));
Count := Count + 1; Fields (Count) := Named ("observations", Read_thinkthen_complete_observation_list_v1(P.observations));
Count := Count + 1; Fields (Count) := Named ("question", Read_thinkthen_complete_question_document_v1(P.question));
Count := Count + 1; Fields (Count) := Named ("question_sources", Read_thinkthen_complete_question_source_list_v1(P.question_sources));
Count := Count + 1; Fields (Count) := Named ("request", String_Item (Text (P.request)));
case P.threshold.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("threshold", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("threshold", Read_thinkthen_complete_threshold_v1(P.threshold.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.usage.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("usage", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("usage", Read_thinkthen_complete_usage_v1(P.usage.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_annotation_member_failure_id_v1;
function Read_thinkthen_complete_annotation_member_v1 (P : access constant thinkthen_complete_annotation_member_v1) return Value is

begin
if P = null then raise Constraint_Error with "missing native value"; end if;
case P.kind is
when 1 => return Read_thinkthen_complete_annotation_member_answer_id_v1 (P.data.answer_id);
when 2 => return Read_thinkthen_complete_annotation_member_failure_id_v1 (P.data.failure_id);
when others => raise Constraint_Error with "invalid native alternative";
end case;
end Read_thinkthen_complete_annotation_member_v1;
function Read_thinkthen_complete_annotation_member_map_v1 (P : thinkthen_complete_annotation_member_map_v1) return Value is
Count : constant Natural := Index (P.len);
type Items_Type is array (1 .. Count) of thinkthen_complete_annotation_member_map_entry_v1 with Convention => C;
type Items_Access is access constant Items_Type;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Items_Access);
Items : Items_Access;
Fields : Field_List (1 .. Count);
begin
if Count > 0 then Items := Pointer (P.data.all'Address); end if;
for I in 1 .. Count loop
Fields (I) := Named (Text (Items (I).name), Read_thinkthen_complete_annotation_member_v1(Items (I).value));
end loop;
return Object_Item (Fields);
end Read_thinkthen_complete_annotation_member_map_v1;
function Read_thinkthen_complete_position_v1 (P : access constant thinkthen_complete_position_v1) return Value is
Fields : Field_List (1 .. 4 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
case P.file.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("file", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("file", String_Item (Text (P.file.value)));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.first.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("first", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("first", Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.first.value), Ada.Strings.Both)));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.images.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("images", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("images", Read_thinkthen_complete_string_list_v1(P.images.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.last.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("last", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("last", Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.last.value), Ada.Strings.Both)));
when others => raise Constraint_Error with "invalid native presence";
end case;
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_position_v1;
function Read_thinkthen_complete_failed_v1 (P : access constant thinkthen_complete_failed_v1) return Value is
Fields : Field_List (1 .. 1 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("failed", Read_thinkthen_complete_failure_v1(P.failed));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_failed_v1;
function Read_thinkthen_complete_annotated_field_v1 (P : access constant thinkthen_complete_annotated_field_v1) return Value is

begin
if P = null then raise Constraint_Error with "missing native value"; end if;
case P.kind is
when 1 => return Boolean_Item (P.data.boolean /= 0);
when 2 => return Null_Item;
when 3 => return String_Item (Text (P.data.string));
when 4 => return Read_thinkthen_complete_string_list_v1(P.data.c_array);
when 5 => return Number_Item (Ada.Strings.Fixed.Trim (double'Image (P.data.number), Ada.Strings.Both));
when 6 => return Read_thinkthen_complete_failed_v1(P.data.object);
when others => raise Constraint_Error with "invalid native alternative";
end case;
end Read_thinkthen_complete_annotated_field_v1;
function Read_thinkthen_complete_annotated_field_map_v1 (P : thinkthen_complete_annotated_field_map_v1) return Value is
Count : constant Natural := Index (P.len);
type Items_Type is array (1 .. Count) of thinkthen_complete_annotated_field_map_entry_v1 with Convention => C;
type Items_Access is access constant Items_Type;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Items_Access);
Items : Items_Access;
Fields : Field_List (1 .. Count);
begin
if Count > 0 then Items := Pointer (P.data.all'Address); end if;
for I in 1 .. Count loop
Fields (I) := Named (Text (Items (I).name), Read_thinkthen_complete_annotated_field_v1(Items (I).value));
end loop;
return Object_Item (Fields);
end Read_thinkthen_complete_annotated_field_map_v1;
function Read_thinkthen_complete_annotated_row_v1 (P : access constant thinkthen_complete_annotated_row_v1) return Value is

begin
if P = null then raise Constraint_Error with "missing native value"; end if;
return Read_thinkthen_complete_annotated_field_map_v1(P.value);
end Read_thinkthen_complete_annotated_row_v1;
function Read_thinkthen_complete_annotation_v1 (P : access constant thinkthen_complete_annotation_v1) return Value is
Fields : Field_List (1 .. 12 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("answer_id", Read_thinkthen_complete_answer_id_v1(P.answer_id));
Count := Count + 1; Fields (Count) := Named ("answers", Read_thinkthen_complete_annotation_member_map_v1(P.answers));
case P.file.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("file", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("file", String_Item (Text (P.file.value)));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.first_line.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("first_line", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("first_line", Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.first_line.value), Ada.Strings.Both)));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.index.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("index", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("index", Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.index.value), Ada.Strings.Both)));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.input.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("input", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("input", Read_Json (P.input.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.last_line.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("last_line", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("last_line", Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.last_line.value), Ada.Strings.Both)));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("meta", Read_thinkthen_complete_meta_v1(P.meta));
case P.position.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("position", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("position", Read_thinkthen_complete_position_v1(P.position.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("schema", Read_thinkthen_complete_version_v1(P.schema));
case P.source.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("source", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("source", Read_thinkthen_complete_physical_source_v1(P.source.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("value", Read_thinkthen_complete_annotated_row_v1(P.value));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_annotation_v1;
function Read_thinkthen_complete_session_packet_annotate_row_v1 (P : access constant thinkthen_complete_session_packet_annotate_row_v1) return Value is
Fields : Field_List (1 .. 3 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("function", String_Item (Text (P.c_function)));
Count := Count + 1; Fields (Count) := Named ("kind", String_Item (Text (P.kind)));
Count := Count + 1; Fields (Count) := Named ("value", Read_thinkthen_complete_annotation_v1(P.value));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_session_packet_annotate_row_v1;
function Read_thinkthen_complete_atomic_decide_value_list_v1 (P : thinkthen_complete_atomic_decide_value_list_v1) return Value is
Count : constant Natural := Index (P.len);
type Items_Type is array (1 .. Count) of access constant thinkthen_complete_atomic_decide_value_v1 with Convention => C;
type Items_Access is access constant Items_Type;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Items_Access);
Items : Items_Access;
Items_Copy : Value_List (1 .. Count);
begin
if Count > 0 then Items := Pointer (P.data); end if;
for I in 1 .. Count loop
Items_Copy (I) := Read_thinkthen_complete_atomic_decide_value_v1(Items (I));
end loop;
return Array_Item (Items_Copy);
end Read_thinkthen_complete_atomic_decide_value_list_v1;
function Read_thinkthen_complete_session_packet_decide_aggregate_v1 (P : access constant thinkthen_complete_session_packet_decide_aggregate_v1) return Value is
Fields : Field_List (1 .. 3 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("function", String_Item (Text (P.c_function)));
Count := Count + 1; Fields (Count) := Named ("kind", String_Item (Text (P.kind)));
Count := Count + 1; Fields (Count) := Named ("value", Read_thinkthen_complete_atomic_decide_value_list_v1(P.value));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_session_packet_decide_aggregate_v1;
function Read_thinkthen_complete_atomic_nullable_string_list_v1 (P : thinkthen_complete_atomic_nullable_string_list_v1) return Value is
Count : constant Natural := Index (P.len);
type Items_Type is array (1 .. Count) of access constant thinkthen_complete_atomic_nullable_string_v1 with Convention => C;
type Items_Access is access constant Items_Type;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Items_Access);
Items : Items_Access;
Items_Copy : Value_List (1 .. Count);
begin
if Count > 0 then Items := Pointer (P.data); end if;
for I in 1 .. Count loop
Items_Copy (I) := Read_thinkthen_complete_atomic_nullable_string_v1(Items (I));
end loop;
return Array_Item (Items_Copy);
end Read_thinkthen_complete_atomic_nullable_string_list_v1;
function Read_thinkthen_complete_session_packet_choose_aggregate_v1 (P : access constant thinkthen_complete_session_packet_choose_aggregate_v1) return Value is
Fields : Field_List (1 .. 3 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("function", String_Item (Text (P.c_function)));
Count := Count + 1; Fields (Count) := Named ("kind", String_Item (Text (P.kind)));
Count := Count + 1; Fields (Count) := Named ("value", Read_thinkthen_complete_atomic_nullable_string_list_v1(P.value));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_session_packet_choose_aggregate_v1;
function Read_thinkthen_complete_atomic_array_of_string_list_v1 (P : thinkthen_complete_atomic_array_of_string_list_v1) return Value is
Count : constant Natural := Index (P.len);
type Items_Type is array (1 .. Count) of access constant thinkthen_complete_atomic_array_of_string_v1 with Convention => C;
type Items_Access is access constant Items_Type;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Items_Access);
Items : Items_Access;
Items_Copy : Value_List (1 .. Count);
begin
if Count > 0 then Items := Pointer (P.data); end if;
for I in 1 .. Count loop
Items_Copy (I) := Read_thinkthen_complete_atomic_array_of_string_v1(Items (I));
end loop;
return Array_Item (Items_Copy);
end Read_thinkthen_complete_atomic_array_of_string_list_v1;
function Read_thinkthen_complete_session_packet_tag_aggregate_v1 (P : access constant thinkthen_complete_session_packet_tag_aggregate_v1) return Value is
Fields : Field_List (1 .. 3 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("function", String_Item (Text (P.c_function)));
Count := Count + 1; Fields (Count) := Named ("kind", String_Item (Text (P.kind)));
Count := Count + 1; Fields (Count) := Named ("value", Read_thinkthen_complete_atomic_array_of_string_list_v1(P.value));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_session_packet_tag_aggregate_v1;
function Read_thinkthen_complete_atomic_double_list_v1 (P : thinkthen_complete_atomic_double_list_v1) return Value is
Count : constant Natural := Index (P.len);
type Items_Type is array (1 .. Count) of access constant thinkthen_complete_atomic_double_v1 with Convention => C;
type Items_Access is access constant Items_Type;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Items_Access);
Items : Items_Access;
Items_Copy : Value_List (1 .. Count);
begin
if Count > 0 then Items := Pointer (P.data); end if;
for I in 1 .. Count loop
Items_Copy (I) := Read_thinkthen_complete_atomic_double_v1(Items (I));
end loop;
return Array_Item (Items_Copy);
end Read_thinkthen_complete_atomic_double_list_v1;
function Read_thinkthen_complete_session_packet_score_aggregate_v1 (P : access constant thinkthen_complete_session_packet_score_aggregate_v1) return Value is
Fields : Field_List (1 .. 3 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("function", String_Item (Text (P.c_function)));
Count := Count + 1; Fields (Count) := Named ("kind", String_Item (Text (P.kind)));
Count := Count + 1; Fields (Count) := Named ("value", Read_thinkthen_complete_atomic_double_list_v1(P.value));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_session_packet_score_aggregate_v1;
function Read_thinkthen_complete_atomic_boolean_list_v1 (P : thinkthen_complete_atomic_boolean_list_v1) return Value is
Count : constant Natural := Index (P.len);
type Items_Type is array (1 .. Count) of access constant thinkthen_complete_atomic_boolean_v1 with Convention => C;
type Items_Access is access constant Items_Type;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Items_Access);
Items : Items_Access;
Items_Copy : Value_List (1 .. Count);
begin
if Count > 0 then Items := Pointer (P.data); end if;
for I in 1 .. Count loop
Items_Copy (I) := Read_thinkthen_complete_atomic_boolean_v1(Items (I));
end loop;
return Array_Item (Items_Copy);
end Read_thinkthen_complete_atomic_boolean_list_v1;
function Read_thinkthen_complete_session_packet_filter_aggregate_v1 (P : access constant thinkthen_complete_session_packet_filter_aggregate_v1) return Value is
Fields : Field_List (1 .. 3 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("function", String_Item (Text (P.c_function)));
Count := Count + 1; Fields (Count) := Named ("kind", String_Item (Text (P.kind)));
Count := Count + 1; Fields (Count) := Named ("value", Read_thinkthen_complete_atomic_boolean_list_v1(P.value));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_session_packet_filter_aggregate_v1;
function Read_thinkthen_complete_atomic_non_zero_usize_v1 (P : access constant thinkthen_complete_atomic_non_zero_usize_v1) return Value is
Fields : Field_List (1 .. 13 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("answer", Read_thinkthen_complete_answer_v1(P.answer));
Count := Count + 1; Fields (Count) := Named ("answer_id", Read_thinkthen_complete_answer_id_v1(P.answer_id));
case P.images.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("images", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("images", Read_thinkthen_complete_image_list_v1(P.images.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.index.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("index", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("index", Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.index.value), Ada.Strings.Both)));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.input.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("input", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("input", Read_Json (P.input.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.members.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("members", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("members", Read_thinkthen_complete_rank_member_list_v1(P.members.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("meta", Read_thinkthen_complete_meta_v1(P.meta));
Count := Count + 1; Fields (Count) := Named ("question", Read_thinkthen_complete_question_document_v1(P.question));
case P.question_name.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("question_name", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("question_name", String_Item (Text (P.question_name.value)));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("schema", Read_thinkthen_complete_version_v1(P.schema));
case P.source.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("source", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("source", Read_thinkthen_complete_physical_source_v1(P.source.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.threshold.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("threshold", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("threshold", Read_thinkthen_complete_threshold_v1(P.threshold.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("value", Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.value), Ada.Strings.Both)));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_atomic_non_zero_usize_v1;
function Read_thinkthen_complete_atomic_non_zero_usize_list_v1 (P : thinkthen_complete_atomic_non_zero_usize_list_v1) return Value is
Count : constant Natural := Index (P.len);
type Items_Type is array (1 .. Count) of access constant thinkthen_complete_atomic_non_zero_usize_v1 with Convention => C;
type Items_Access is access constant Items_Type;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Items_Access);
Items : Items_Access;
Items_Copy : Value_List (1 .. Count);
begin
if Count > 0 then Items := Pointer (P.data); end if;
for I in 1 .. Count loop
Items_Copy (I) := Read_thinkthen_complete_atomic_non_zero_usize_v1(Items (I));
end loop;
return Array_Item (Items_Copy);
end Read_thinkthen_complete_atomic_non_zero_usize_list_v1;
function Read_thinkthen_complete_session_packet_rank_aggregate_v1 (P : access constant thinkthen_complete_session_packet_rank_aggregate_v1) return Value is
Fields : Field_List (1 .. 3 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("function", String_Item (Text (P.c_function)));
Count := Count + 1; Fields (Count) := Named ("kind", String_Item (Text (P.kind)));
Count := Count + 1; Fields (Count) := Named ("value", Read_thinkthen_complete_atomic_non_zero_usize_list_v1(P.value));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_session_packet_rank_aggregate_v1;
function Read_thinkthen_complete_find_answer_v1 (P : access constant thinkthen_complete_find_answer_v1) return Value is
Fields : Field_List (1 .. 4 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
case P.confidence.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("confidence", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("confidence", Number_Item (Ada.Strings.Fixed.Trim (double'Image (P.confidence.value), Ada.Strings.Both)));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("kind", String_Item (Text (P.kind)));
Count := Count + 1; Fields (Count) := Named ("pick", String_Item (Text (P.pick)));
Count := Count + 1; Fields (Count) := Named ("probabilities", Read_thinkthen_complete_number_map_v1(P.probabilities));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_find_answer_v1;
function Read_thinkthen_complete_find_candidate_v1 (P : access constant thinkthen_complete_find_candidate_v1) return Value is
Fields : Field_List (1 .. 4 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
case P.index.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("index", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("index", Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.index.value), Ada.Strings.Both)));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.input.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("input", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("input", Read_Json (P.input.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("probability", Number_Item (Ada.Strings.Fixed.Trim (double'Image (P.probability), Ada.Strings.Both)));
case P.source.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("source", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("source", Read_thinkthen_complete_physical_source_v1(P.source.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_find_candidate_v1;
function Read_thinkthen_complete_find_candidate_list_v1 (P : thinkthen_complete_find_candidate_list_v1) return Value is
Count : constant Natural := Index (P.len);
type Items_Type is array (1 .. Count) of access constant thinkthen_complete_find_candidate_v1 with Convention => C;
type Items_Access is access constant Items_Type;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Items_Access);
Items : Items_Access;
Items_Copy : Value_List (1 .. Count);
begin
if Count > 0 then Items := Pointer (P.data); end if;
for I in 1 .. Count loop
Items_Copy (I) := Read_thinkthen_complete_find_candidate_v1(Items (I));
end loop;
return Array_Item (Items_Copy);
end Read_thinkthen_complete_find_candidate_list_v1;
function Read_thinkthen_complete_find_question_document_v1 (P : access constant thinkthen_complete_find_question_document_v1) return Value is
Fields : Field_List (1 .. 12 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
case P.batch.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("batch", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("batch", Read_thinkthen_complete_batch_v1(P.batch.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.context_schema.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("context_schema", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("context_schema", Read_thinkthen_complete_input_declaration_v1(P.context_schema.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.item_schema.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("item_schema", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("item_schema", Read_thinkthen_complete_input_declaration_v1(P.item_schema.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.label_details.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("label_details", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("label_details", Read_thinkthen_complete_label_list_v1(P.label_details.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.model.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("model", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("model", String_Item (Text (P.model.value)));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.name.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("name", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("name", Read_thinkthen_complete_question_name_v1(P.name.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("none", Boolean_Item (P.none /= 0));
case P.on.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("on", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("on", Read_thinkthen_complete_string_list_v1(P.on.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.profile.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("profile", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("profile", String_Item (Text (P.profile.value)));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.text.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("text", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("text", Read_Json (P.text.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("verb", String_Item (Text (P.verb)));
case P.wording_version.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("wording_version", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("wording_version", Read_thinkthen_complete_wording_version_v1(P.wording_version.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_find_question_document_v1;
function Read_thinkthen_complete_find_v1 (P : access constant thinkthen_complete_find_v1) return Value is
Fields : Field_List (1 .. 13 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("answer", Read_thinkthen_complete_find_answer_v1(P.answer));
Count := Count + 1; Fields (Count) := Named ("answer_id", Read_thinkthen_complete_answer_id_v1(P.answer_id));
case P.candidates.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("candidates", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("candidates", Read_thinkthen_complete_find_candidate_list_v1(P.candidates.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.file.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("file", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("file", String_Item (Text (P.file.value)));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.first_line.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("first_line", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("first_line", Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.first_line.value), Ada.Strings.Both)));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.index.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("index", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("index", Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.index.value), Ada.Strings.Both)));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.last_line.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("last_line", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("last_line", Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.last_line.value), Ada.Strings.Both)));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("meta", Read_thinkthen_complete_meta_v1(P.meta));
case P.position.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("position", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("position", Read_thinkthen_complete_position_v1(P.position.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("question", Read_thinkthen_complete_find_question_document_v1(P.question));
Count := Count + 1; Fields (Count) := Named ("schema", Read_thinkthen_complete_version_v1(P.schema));
case P.threshold.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("threshold", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("threshold", Read_Json (P.threshold.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.value.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("value", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("value", Read_Json (P.value.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_find_v1;
function Read_thinkthen_complete_session_packet_find_aggregate_v1 (P : access constant thinkthen_complete_session_packet_find_aggregate_v1) return Value is
Fields : Field_List (1 .. 3 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("function", String_Item (Text (P.c_function)));
Count := Count + 1; Fields (Count) := Named ("kind", String_Item (Text (P.kind)));
Count := Count + 1; Fields (Count) := Named ("value", Read_thinkthen_complete_find_v1(P.value));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_session_packet_find_aggregate_v1;
function Read_thinkthen_complete_annotation_list_v1 (P : thinkthen_complete_annotation_list_v1) return Value is
Count : constant Natural := Index (P.len);
type Items_Type is array (1 .. Count) of access constant thinkthen_complete_annotation_v1 with Convention => C;
type Items_Access is access constant Items_Type;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Items_Access);
Items : Items_Access;
Items_Copy : Value_List (1 .. Count);
begin
if Count > 0 then Items := Pointer (P.data); end if;
for I in 1 .. Count loop
Items_Copy (I) := Read_thinkthen_complete_annotation_v1(Items (I));
end loop;
return Array_Item (Items_Copy);
end Read_thinkthen_complete_annotation_list_v1;
function Read_thinkthen_complete_session_packet_annotate_aggregate_v1 (P : access constant thinkthen_complete_session_packet_annotate_aggregate_v1) return Value is
Fields : Field_List (1 .. 3 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("function", String_Item (Text (P.c_function)));
Count := Count + 1; Fields (Count) := Named ("kind", String_Item (Text (P.kind)));
Count := Count + 1; Fields (Count) := Named ("value", Read_thinkthen_complete_annotation_list_v1(P.value));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_session_packet_annotate_aggregate_v1;
function Read_thinkthen_complete_name_odds_v1 (P : access constant thinkthen_complete_name_odds_v1) return Value is
Fields : Field_List (1 .. 4 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
case P.edges.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("edges", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("edges", Read_thinkthen_complete_number_map_v1(P.edges.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("end", Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.c_end), Ada.Strings.Both)));
case P.kinds.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("kinds", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("kinds", Read_thinkthen_complete_number_map_v1(P.kinds.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("start", Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.start), Ada.Strings.Both)));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_name_odds_v1;
function Read_thinkthen_complete_name_odds_list_v1 (P : thinkthen_complete_name_odds_list_v1) return Value is
Count : constant Natural := Index (P.len);
type Items_Type is array (1 .. Count) of access constant thinkthen_complete_name_odds_v1 with Convention => C;
type Items_Access is access constant Items_Type;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Items_Access);
Items : Items_Access;
Items_Copy : Value_List (1 .. Count);
begin
if Count > 0 then Items := Pointer (P.data); end if;
for I in 1 .. Count loop
Items_Copy (I) := Read_thinkthen_complete_name_odds_v1(Items (I));
end loop;
return Array_Item (Items_Copy);
end Read_thinkthen_complete_name_odds_list_v1;
function Read_thinkthen_complete_place_v1 (P : access constant thinkthen_complete_place_v1) return Value is
Fields : Field_List (1 .. 2 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("end", Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.c_end), Ada.Strings.Both)));
Count := Count + 1; Fields (Count) := Named ("start", Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.start), Ada.Strings.Both)));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_place_v1;
function Read_thinkthen_complete_pair_odds_v1 (P : access constant thinkthen_complete_pair_odds_v1) return Value is
Fields : Field_List (1 .. 4 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("probability", Number_Item (Ada.Strings.Fixed.Trim (double'Image (P.probability), Ada.Strings.Both)));
Count := Count + 1; Fields (Count) := Named ("relation", String_Item (Text (P.relation)));
Count := Count + 1; Fields (Count) := Named ("source", Read_thinkthen_complete_place_v1(P.source));
Count := Count + 1; Fields (Count) := Named ("target", Read_thinkthen_complete_place_v1(P.target));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_pair_odds_v1;
function Read_thinkthen_complete_pair_odds_list_v1 (P : thinkthen_complete_pair_odds_list_v1) return Value is
Count : constant Natural := Index (P.len);
type Items_Type is array (1 .. Count) of access constant thinkthen_complete_pair_odds_v1 with Convention => C;
type Items_Access is access constant Items_Type;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Items_Access);
Items : Items_Access;
Items_Copy : Value_List (1 .. Count);
begin
if Count > 0 then Items := Pointer (P.data); end if;
for I in 1 .. Count loop
Items_Copy (I) := Read_thinkthen_complete_pair_odds_v1(Items (I));
end loop;
return Array_Item (Items_Copy);
end Read_thinkthen_complete_pair_odds_list_v1;
function Read_thinkthen_complete_piece_odds_v1 (P : access constant thinkthen_complete_piece_odds_v1) return Value is
Fields : Field_List (1 .. 3 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("end", Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.c_end), Ada.Strings.Both)));
Count := Count + 1; Fields (Count) := Named ("start", Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.start), Ada.Strings.Both)));
Count := Count + 1; Fields (Count) := Named ("tags", Read_thinkthen_complete_number_map_v1(P.tags));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_piece_odds_v1;
function Read_thinkthen_complete_piece_odds_list_v1 (P : thinkthen_complete_piece_odds_list_v1) return Value is
Count : constant Natural := Index (P.len);
type Items_Type is array (1 .. Count) of access constant thinkthen_complete_piece_odds_v1 with Convention => C;
type Items_Access is access constant Items_Type;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Items_Access);
Items : Items_Access;
Items_Copy : Value_List (1 .. Count);
begin
if Count > 0 then Items := Pointer (P.data); end if;
for I in 1 .. Count loop
Items_Copy (I) := Read_thinkthen_complete_piece_odds_v1(Items (I));
end loop;
return Array_Item (Items_Copy);
end Read_thinkthen_complete_piece_odds_list_v1;
function Read_thinkthen_complete_recognition_proposal_v1 (P : access constant thinkthen_complete_recognition_proposal_v1) return Value is
Fields : Field_List (1 .. 7 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("end", Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.c_end), Ada.Strings.Both)));
Count := Count + 1; Fields (Count) := Named ("kept", Boolean_Item (P.kept /= 0));
case P.kind.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("kind", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("kind", String_Item (Text (P.kind.value)));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.selected.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("selected", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("selected", Read_thinkthen_complete_place_v1(P.selected.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("span_probability", Number_Item (Ada.Strings.Fixed.Trim (double'Image (P.span_probability), Ada.Strings.Both)));
Count := Count + 1; Fields (Count) := Named ("start", Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.start), Ada.Strings.Both)));
case P.strength.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("strength", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("strength", Number_Item (Ada.Strings.Fixed.Trim (double'Image (P.strength.value), Ada.Strings.Both)));
when others => raise Constraint_Error with "invalid native presence";
end case;
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_recognition_proposal_v1;
function Read_thinkthen_complete_recognition_proposal_list_v1 (P : thinkthen_complete_recognition_proposal_list_v1) return Value is
Count : constant Natural := Index (P.len);
type Items_Type is array (1 .. Count) of access constant thinkthen_complete_recognition_proposal_v1 with Convention => C;
type Items_Access is access constant Items_Type;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Items_Access);
Items : Items_Access;
Items_Copy : Value_List (1 .. Count);
begin
if Count > 0 then Items := Pointer (P.data); end if;
for I in 1 .. Count loop
Items_Copy (I) := Read_thinkthen_complete_recognition_proposal_v1(Items (I));
end loop;
return Array_Item (Items_Copy);
end Read_thinkthen_complete_recognition_proposal_list_v1;
function Read_thinkthen_complete_recognition_odds_recognize_answer_v1 (P : access constant thinkthen_complete_recognition_odds_recognize_answer_v1) return Value is
Fields : Field_List (1 .. 4 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("names", Read_thinkthen_complete_name_odds_list_v1(P.names));
Count := Count + 1; Fields (Count) := Named ("pairs", Read_thinkthen_complete_pair_odds_list_v1(P.pairs));
Count := Count + 1; Fields (Count) := Named ("pieces", Read_thinkthen_complete_piece_odds_list_v1(P.pieces));
Count := Count + 1; Fields (Count) := Named ("proposals", Read_thinkthen_complete_recognition_proposal_list_v1(P.proposals));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_recognition_odds_recognize_answer_v1;
function Read_thinkthen_complete_boundary_proposal_v1 (P : access constant thinkthen_complete_boundary_proposal_v1) return Value is
Fields : Field_List (1 .. 5 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("end", Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.c_end), Ada.Strings.Both)));
Count := Count + 1; Fields (Count) := Named ("length", Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.length), Ada.Strings.Both)));
Count := Count + 1; Fields (Count) := Named ("probability", Number_Item (Ada.Strings.Fixed.Trim (double'Image (P.probability), Ada.Strings.Both)));
Count := Count + 1; Fields (Count) := Named ("start", Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.start), Ada.Strings.Both)));
Count := Count + 1; Fields (Count) := Named ("text", String_Item (Text (P.text)));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_boundary_proposal_v1;
function Read_thinkthen_complete_boundary_proposal_list_v1 (P : thinkthen_complete_boundary_proposal_list_v1) return Value is
Count : constant Natural := Index (P.len);
type Items_Type is array (1 .. Count) of access constant thinkthen_complete_boundary_proposal_v1 with Convention => C;
type Items_Access is access constant Items_Type;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Items_Access);
Items : Items_Access;
Items_Copy : Value_List (1 .. Count);
begin
if Count > 0 then Items := Pointer (P.data); end if;
for I in 1 .. Count loop
Items_Copy (I) := Read_thinkthen_complete_boundary_proposal_v1(Items (I));
end loop;
return Array_Item (Items_Copy);
end Read_thinkthen_complete_boundary_proposal_list_v1;
function Read_thinkthen_complete_recognition_odds_boundary_odds_v1 (P : access constant thinkthen_complete_recognition_odds_boundary_odds_v1) return Value is
Fields : Field_List (1 .. 2 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("pieces", Read_thinkthen_complete_piece_odds_list_v1(P.pieces));
Count := Count + 1; Fields (Count) := Named ("proposals", Read_thinkthen_complete_boundary_proposal_list_v1(P.proposals));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_recognition_odds_boundary_odds_v1;
function Read_thinkthen_complete_recognition_odds_v1 (P : access constant thinkthen_complete_recognition_odds_v1) return Value is

begin
if P = null then raise Constraint_Error with "missing native value"; end if;
case P.kind is
when 1 => return Read_thinkthen_complete_recognition_odds_recognize_answer_v1 (P.data.recognize_answer);
when 2 => return Read_thinkthen_complete_recognition_odds_boundary_odds_v1 (P.data.boundary_odds);
when others => raise Constraint_Error with "invalid native alternative";
end case;
end Read_thinkthen_complete_recognition_odds_v1;
function Read_thinkthen_complete_json_map_v1 (P : thinkthen_complete_json_map_v1) return Value is
Count : constant Natural := Index (P.len);
type Items_Type is array (1 .. Count) of thinkthen_complete_json_map_entry_v1 with Convention => C;
type Items_Access is access constant Items_Type;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Items_Access);
Items : Items_Access;
Fields : Field_List (1 .. Count);
begin
if Count > 0 then Items := Pointer (P.data.all'Address); end if;
for I in 1 .. Count loop
Fields (I) := Named (Text (Items (I).name), Read_Json (Items (I).value));
end loop;
return Object_Item (Fields);
end Read_thinkthen_complete_json_map_v1;
function Read_thinkthen_complete_recognition_mode_v1 (P : access constant thinkthen_complete_recognition_mode_v1) return Value is

begin
if P = null then raise Constraint_Error with "missing native value"; end if;
case P.kind is
when 1 => return String_Item ("whole");
when 2 => return String_Item ("boundary_only");
when others => raise Constraint_Error with "invalid native enum";
end case;
end Read_thinkthen_complete_recognition_mode_v1;
function Read_thinkthen_complete_relation_rule_v1 (P : access constant thinkthen_complete_relation_rule_v1) return Value is
Fields : Field_List (1 .. 6 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("either", Boolean_Item (P.either /= 0));
Count := Count + 1; Fields (Count) := Named ("name", String_Item (Text (P.name)));
Count := Count + 1; Fields (Count) := Named ("reads", String_Item (Text (P.reads)));
case P.single.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("single", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("single", Boolean_Item (P.single.value /= 0));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("source", String_Item (Text (P.source)));
Count := Count + 1; Fields (Count) := Named ("target", String_Item (Text (P.target)));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_relation_rule_v1;
function Read_thinkthen_complete_relation_rule_list_v1 (P : thinkthen_complete_relation_rule_list_v1) return Value is
Count : constant Natural := Index (P.len);
type Items_Type is array (1 .. Count) of access constant thinkthen_complete_relation_rule_v1 with Convention => C;
type Items_Access is access constant Items_Type;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Items_Access);
Items : Items_Access;
Items_Copy : Value_List (1 .. Count);
begin
if Count > 0 then Items := Pointer (P.data); end if;
for I in 1 .. Count loop
Items_Copy (I) := Read_thinkthen_complete_relation_rule_v1(Items (I));
end loop;
return Array_Item (Items_Copy);
end Read_thinkthen_complete_relation_rule_list_v1;
function Read_thinkthen_complete_recognition_stage_context_v1 (P : access constant thinkthen_complete_recognition_stage_context_v1) return Value is
Fields : Field_List (1 .. 3 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
case P.boundary.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("boundary", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("boundary", String_Item (Text (P.boundary.value)));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.kind_edge.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("kind_edge", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("kind_edge", String_Item (Text (P.kind_edge.value)));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.relation.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("relation", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("relation", String_Item (Text (P.relation.value)));
when others => raise Constraint_Error with "invalid native presence";
end case;
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_recognition_stage_context_v1;
function Read_thinkthen_complete_verb_v1 (P : access constant thinkthen_complete_verb_v1) return Value is

begin
if P = null then raise Constraint_Error with "missing native value"; end if;
case P.kind is
when 1 => return String_Item ("recognize");
when others => raise Constraint_Error with "invalid native enum";
end case;
end Read_thinkthen_complete_verb_v1;
function Read_thinkthen_complete_recognize_question_document_v1 (P : access constant thinkthen_complete_recognize_question_document_v1) return Value is
Fields : Field_List (1 .. 19 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
case P.batch.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("batch", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("batch", Read_thinkthen_complete_batch_v1(P.batch.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.context_schema.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("context_schema", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("context_schema", Read_thinkthen_complete_input_declaration_v1(P.context_schema.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.entity_definition.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("entity_definition", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("entity_definition", Read_Json (P.entity_definition.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.instructions.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("instructions", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("instructions", Read_Json (P.instructions.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.item_schema.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("item_schema", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("item_schema", Read_thinkthen_complete_input_declaration_v1(P.item_schema.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("kinds", Read_thinkthen_complete_json_map_v1(P.kinds));
case P.label_details.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("label_details", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("label_details", Read_thinkthen_complete_label_list_v1(P.label_details.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.mode.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("mode", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("mode", Read_thinkthen_complete_recognition_mode_v1(P.mode.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.model.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("model", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("model", String_Item (Text (P.model.value)));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.name.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("name", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("name", Read_thinkthen_complete_question_name_v1(P.name.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.on.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("on", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("on", Read_thinkthen_complete_string_list_v1(P.on.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.profile.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("profile", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("profile", String_Item (Text (P.profile.value)));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.relation_threshold.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("relation_threshold", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("relation_threshold", Read_thinkthen_complete_threshold_v1(P.relation_threshold.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.relations.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("relations", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("relations", Read_thinkthen_complete_relation_rule_list_v1(P.relations.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.snippet_pieces.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("snippet_pieces", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("snippet_pieces", Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.snippet_pieces.value), Ada.Strings.Both)));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.stage_context.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("stage_context", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("stage_context", Read_thinkthen_complete_recognition_stage_context_v1(P.stage_context.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("threshold", Read_thinkthen_complete_threshold_v1(P.threshold));
Count := Count + 1; Fields (Count) := Named ("verb", Read_thinkthen_complete_verb_v1(P.verb));
case P.wording_version.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("wording_version", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("wording_version", Read_thinkthen_complete_wording_version_v1(P.wording_version.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_recognize_question_document_v1;
function Read_thinkthen_complete_entity_v1 (P : access constant thinkthen_complete_entity_v1) return Value is
Fields : Field_List (1 .. 9 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("end", Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.c_end), Ada.Strings.Both)));
case P.file.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("file", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("file", String_Item (Text (P.file.value)));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.first_line.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("first_line", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("first_line", Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.first_line.value), Ada.Strings.Both)));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("kind", String_Item (Text (P.kind)));
case P.last_line.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("last_line", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("last_line", Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.last_line.value), Ada.Strings.Both)));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("length", Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.length), Ada.Strings.Both)));
Count := Count + 1; Fields (Count) := Named ("start", Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.start), Ada.Strings.Both)));
Count := Count + 1; Fields (Count) := Named ("strength", Number_Item (Ada.Strings.Fixed.Trim (double'Image (P.strength), Ada.Strings.Both)));
Count := Count + 1; Fields (Count) := Named ("text", String_Item (Text (P.text)));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_entity_v1;
function Read_thinkthen_complete_entity_list_v1 (P : thinkthen_complete_entity_list_v1) return Value is
Count : constant Natural := Index (P.len);
type Items_Type is array (1 .. Count) of access constant thinkthen_complete_entity_v1 with Convention => C;
type Items_Access is access constant Items_Type;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Items_Access);
Items : Items_Access;
Items_Copy : Value_List (1 .. Count);
begin
if Count > 0 then Items := Pointer (P.data); end if;
for I in 1 .. Count loop
Items_Copy (I) := Read_thinkthen_complete_entity_v1(Items (I));
end loop;
return Array_Item (Items_Copy);
end Read_thinkthen_complete_entity_list_v1;
function Read_thinkthen_complete_entity_edge_v1 (P : access constant thinkthen_complete_entity_edge_v1) return Value is
Fields : Field_List (1 .. 5 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
case P.either.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("either", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("either", Boolean_Item (P.either.value /= 0));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("probability", Number_Item (Ada.Strings.Fixed.Trim (double'Image (P.probability), Ada.Strings.Both)));
Count := Count + 1; Fields (Count) := Named ("relation", String_Item (Text (P.relation)));
Count := Count + 1; Fields (Count) := Named ("source", Read_thinkthen_complete_entity_v1(P.source));
Count := Count + 1; Fields (Count) := Named ("target", Read_thinkthen_complete_entity_v1(P.target));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_entity_edge_v1;
function Read_thinkthen_complete_entity_edge_list_v1 (P : thinkthen_complete_entity_edge_list_v1) return Value is
Count : constant Natural := Index (P.len);
type Items_Type is array (1 .. Count) of access constant thinkthen_complete_entity_edge_v1 with Convention => C;
type Items_Access is access constant Items_Type;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Items_Access);
Items : Items_Access;
Items_Copy : Value_List (1 .. Count);
begin
if Count > 0 then Items := Pointer (P.data); end if;
for I in 1 .. Count loop
Items_Copy (I) := Read_thinkthen_complete_entity_edge_v1(Items (I));
end loop;
return Array_Item (Items_Copy);
end Read_thinkthen_complete_entity_edge_list_v1;
function Read_thinkthen_complete_recognize_entities_v1 (P : access constant thinkthen_complete_recognize_entities_v1) return Value is
Fields : Field_List (1 .. 2 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("entities", Read_thinkthen_complete_entity_list_v1(P.entities));
case P.relations.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("relations", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("relations", Read_thinkthen_complete_entity_edge_list_v1(P.relations.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_recognize_entities_v1;
function Read_thinkthen_complete_boundary_mode_v1 (P : access constant thinkthen_complete_boundary_mode_v1) return Value is

begin
if P = null then raise Constraint_Error with "missing native value"; end if;
case P.kind is
when 1 => return String_Item ("boundary_only");
when others => raise Constraint_Error with "invalid native enum";
end case;
end Read_thinkthen_complete_boundary_mode_v1;
function Read_thinkthen_complete_recognize_mode_proposals_v1 (P : access constant thinkthen_complete_recognize_mode_proposals_v1) return Value is
Fields : Field_List (1 .. 2 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("mode", Read_thinkthen_complete_boundary_mode_v1(P.mode));
Count := Count + 1; Fields (Count) := Named ("proposals", Read_thinkthen_complete_boundary_proposal_list_v1(P.proposals));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_recognize_mode_proposals_v1;
function Read_thinkthen_complete_recognize_v1 (P : access constant thinkthen_complete_recognize_v1) return Value is

begin
if P = null then raise Constraint_Error with "missing native value"; end if;
case P.kind is
when 1 => return Read_thinkthen_complete_recognize_entities_v1 (P.data.entities);
when 2 => return Read_thinkthen_complete_recognize_mode_proposals_v1 (P.data.mode_proposals);
when others => raise Constraint_Error with "invalid native alternative";
end case;
end Read_thinkthen_complete_recognize_v1;
function Read_thinkthen_complete_recognition_v1 (P : access constant thinkthen_complete_recognition_v1) return Value is
Fields : Field_List (1 .. 13 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("answer", Read_thinkthen_complete_recognition_odds_v1(P.answer));
Count := Count + 1; Fields (Count) := Named ("answer_id", Read_thinkthen_complete_answer_id_v1(P.answer_id));
case P.file.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("file", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("file", String_Item (Text (P.file.value)));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.first_line.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("first_line", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("first_line", Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.first_line.value), Ada.Strings.Both)));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.index.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("index", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("index", Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.index.value), Ada.Strings.Both)));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.input.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("input", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("input", Read_Json (P.input.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.last_line.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("last_line", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("last_line", Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.last_line.value), Ada.Strings.Both)));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("meta", Read_thinkthen_complete_meta_v1(P.meta));
case P.position.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("position", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("position", Read_thinkthen_complete_position_v1(P.position.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("question", Read_thinkthen_complete_recognize_question_document_v1(P.question));
Count := Count + 1; Fields (Count) := Named ("schema", Read_thinkthen_complete_version_v1(P.schema));
case P.source.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("source", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("source", Read_thinkthen_complete_physical_source_v1(P.source.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("value", Read_thinkthen_complete_recognize_v1(P.value));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_recognition_v1;
function Read_thinkthen_complete_recognition_list_v1 (P : thinkthen_complete_recognition_list_v1) return Value is
Count : constant Natural := Index (P.len);
type Items_Type is array (1 .. Count) of access constant thinkthen_complete_recognition_v1 with Convention => C;
type Items_Access is access constant Items_Type;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Items_Access);
Items : Items_Access;
Items_Copy : Value_List (1 .. Count);
begin
if Count > 0 then Items := Pointer (P.data); end if;
for I in 1 .. Count loop
Items_Copy (I) := Read_thinkthen_complete_recognition_v1(Items (I));
end loop;
return Array_Item (Items_Copy);
end Read_thinkthen_complete_recognition_list_v1;
function Read_thinkthen_complete_session_packet_recognize_aggregate_v1 (P : access constant thinkthen_complete_session_packet_recognize_aggregate_v1) return Value is
Fields : Field_List (1 .. 3 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("function", String_Item (Text (P.c_function)));
Count := Count + 1; Fields (Count) := Named ("kind", String_Item (Text (P.kind)));
Count := Count + 1; Fields (Count) := Named ("value", Read_thinkthen_complete_recognition_list_v1(P.value));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_session_packet_recognize_aggregate_v1;
function Read_thinkthen_complete_relation_direction_v1 (P : access constant thinkthen_complete_relation_direction_v1) return Value is

begin
if P = null then raise Constraint_Error with "missing native value"; end if;
case P.kind is
when 1 => return String_Item ("source_to_target");
when 2 => return String_Item ("either");
when others => raise Constraint_Error with "invalid native enum";
end case;
end Read_thinkthen_complete_relation_direction_v1;
function Read_thinkthen_complete_relation_method_v1 (P : access constant thinkthen_complete_relation_method_v1) return Value is

begin
if P = null then raise Constraint_Error with "missing native value"; end if;
case P.kind is
when 1 => return String_Item ("yes_no");
when 2 => return String_Item ("choice");
when others => raise Constraint_Error with "invalid native enum";
end case;
end Read_thinkthen_complete_relation_method_v1;
function Read_thinkthen_complete_related_entity_v1 (P : access constant thinkthen_complete_related_entity_v1) return Value is
Fields : Field_List (1 .. 2 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("kind", String_Item (Text (P.kind)));
Count := Count + 1; Fields (Count) := Named ("name", String_Item (Text (P.name)));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_related_entity_v1;
function Read_thinkthen_complete_relation_member_answer_id_v1 (P : access constant thinkthen_complete_relation_member_answer_id_v1) return Value is
Fields : Field_List (1 .. 16 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("direction", Read_thinkthen_complete_relation_direction_v1(P.direction));
Count := Count + 1; Fields (Count) := Named ("method", Read_thinkthen_complete_relation_method_v1(P.method));
Count := Count + 1; Fields (Count) := Named ("observations", Read_thinkthen_complete_observation_list_v1(P.observations));
Count := Count + 1; Fields (Count) := Named ("question", Read_thinkthen_complete_question_document_v1(P.question));
Count := Count + 1; Fields (Count) := Named ("question_sources", Read_thinkthen_complete_question_source_list_v1(P.question_sources));
Count := Count + 1; Fields (Count) := Named ("reads", String_Item (Text (P.reads)));
Count := Count + 1; Fields (Count) := Named ("relation", String_Item (Text (P.relation)));
Count := Count + 1; Fields (Count) := Named ("request", String_Item (Text (P.request)));
Count := Count + 1; Fields (Count) := Named ("source", Read_thinkthen_complete_related_entity_v1(P.source));
case P.target.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("target", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("target", Read_thinkthen_complete_related_entity_v1(P.target.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("threshold", Read_thinkthen_complete_threshold_v1(P.threshold));
case P.usage.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("usage", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("usage", Read_thinkthen_complete_usage_v1(P.usage.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("accepted", Boolean_Item (P.accepted /= 0));
Count := Count + 1; Fields (Count) := Named ("answer", Read_thinkthen_complete_answer_v1(P.answer));
Count := Count + 1; Fields (Count) := Named ("answer_id", Read_thinkthen_complete_answer_id_v1(P.answer_id));
Count := Count + 1; Fields (Count) := Named ("probability", Number_Item (Ada.Strings.Fixed.Trim (double'Image (P.probability), Ada.Strings.Both)));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_relation_member_answer_id_v1;
function Read_thinkthen_complete_relation_member_failure_id_v1 (P : access constant thinkthen_complete_relation_member_failure_id_v1) return Value is
Fields : Field_List (1 .. 14 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("direction", Read_thinkthen_complete_relation_direction_v1(P.direction));
Count := Count + 1; Fields (Count) := Named ("method", Read_thinkthen_complete_relation_method_v1(P.method));
Count := Count + 1; Fields (Count) := Named ("observations", Read_thinkthen_complete_observation_list_v1(P.observations));
Count := Count + 1; Fields (Count) := Named ("question", Read_thinkthen_complete_question_document_v1(P.question));
Count := Count + 1; Fields (Count) := Named ("question_sources", Read_thinkthen_complete_question_source_list_v1(P.question_sources));
Count := Count + 1; Fields (Count) := Named ("reads", String_Item (Text (P.reads)));
Count := Count + 1; Fields (Count) := Named ("relation", String_Item (Text (P.relation)));
Count := Count + 1; Fields (Count) := Named ("request", String_Item (Text (P.request)));
Count := Count + 1; Fields (Count) := Named ("source", Read_thinkthen_complete_related_entity_v1(P.source));
case P.target.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("target", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("target", Read_thinkthen_complete_related_entity_v1(P.target.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("threshold", Read_thinkthen_complete_threshold_v1(P.threshold));
case P.usage.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("usage", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("usage", Read_thinkthen_complete_usage_v1(P.usage.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("failure", Read_thinkthen_complete_failure_v1(P.failure));
Count := Count + 1; Fields (Count) := Named ("failure_id", Read_thinkthen_complete_failure_id_v1(P.failure_id));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_relation_member_failure_id_v1;
function Read_thinkthen_complete_relation_member_v1 (P : access constant thinkthen_complete_relation_member_v1) return Value is

begin
if P = null then raise Constraint_Error with "missing native value"; end if;
case P.kind is
when 1 => return Read_thinkthen_complete_relation_member_answer_id_v1 (P.data.answer_id);
when 2 => return Read_thinkthen_complete_relation_member_failure_id_v1 (P.data.failure_id);
when others => raise Constraint_Error with "invalid native alternative";
end case;
end Read_thinkthen_complete_relation_member_v1;
function Read_thinkthen_complete_relation_member_list_v1 (P : thinkthen_complete_relation_member_list_v1) return Value is
Count : constant Natural := Index (P.len);
type Items_Type is array (1 .. Count) of access constant thinkthen_complete_relation_member_v1 with Convention => C;
type Items_Access is access constant Items_Type;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Items_Access);
Items : Items_Access;
Items_Copy : Value_List (1 .. Count);
begin
if Count > 0 then Items := Pointer (P.data); end if;
for I in 1 .. Count loop
Items_Copy (I) := Read_thinkthen_complete_relation_member_v1(Items (I));
end loop;
return Array_Item (Items_Copy);
end Read_thinkthen_complete_relation_member_list_v1;
function Read_thinkthen_complete_answers_v1 (P : access constant thinkthen_complete_answers_v1) return Value is
Fields : Field_List (1 .. 1 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("questions", Read_thinkthen_complete_relation_member_list_v1(P.questions));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_answers_v1;
function Read_thinkthen_complete_session_input_source_v1 (P : access constant thinkthen_complete_session_input_source_v1) return Value is
Fields : Field_List (1 .. 2 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("index", Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.index), Ada.Strings.Both)));
Count := Count + 1; Fields (Count) := Named ("source", Read_thinkthen_complete_physical_source_v1(P.source));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_session_input_source_v1;
function Read_thinkthen_complete_session_input_source_list_v1 (P : thinkthen_complete_session_input_source_list_v1) return Value is
Count : constant Natural := Index (P.len);
type Items_Type is array (1 .. Count) of access constant thinkthen_complete_session_input_source_v1 with Convention => C;
type Items_Access is access constant Items_Type;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Items_Access);
Items : Items_Access;
Items_Copy : Value_List (1 .. Count);
begin
if Count > 0 then Items := Pointer (P.data); end if;
for I in 1 .. Count loop
Items_Copy (I) := Read_thinkthen_complete_session_input_source_v1(Items (I));
end loop;
return Array_Item (Items_Copy);
end Read_thinkthen_complete_session_input_source_list_v1;
function Read_thinkthen_complete_relate_fields_v1 (P : access constant thinkthen_complete_relate_fields_v1) return Value is
Fields : Field_List (1 .. 2 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("kind", String_Item (Text (P.kind)));
Count := Count + 1; Fields (Count) := Named ("name", String_Item (Text (P.name)));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_relate_fields_v1;
function Read_thinkthen_complete_relate_question_document_v1 (P : access constant thinkthen_complete_relate_question_document_v1) return Value is
Fields : Field_List (1 .. 13 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
case P.batch.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("batch", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("batch", Read_thinkthen_complete_batch_v1(P.batch.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.context_schema.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("context_schema", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("context_schema", Read_thinkthen_complete_input_declaration_v1(P.context_schema.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.fields.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("fields", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("fields", Read_thinkthen_complete_relate_fields_v1(P.fields.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.item_schema.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("item_schema", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("item_schema", Read_thinkthen_complete_input_declaration_v1(P.item_schema.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.label_details.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("label_details", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("label_details", Read_thinkthen_complete_label_list_v1(P.label_details.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.model.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("model", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("model", String_Item (Text (P.model.value)));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.name.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("name", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("name", Read_thinkthen_complete_question_name_v1(P.name.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.on.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("on", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("on", Read_thinkthen_complete_string_list_v1(P.on.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.profile.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("profile", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("profile", String_Item (Text (P.profile.value)));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("relations", Read_thinkthen_complete_relation_rule_list_v1(P.relations));
Count := Count + 1; Fields (Count) := Named ("threshold", Read_thinkthen_complete_threshold_v1(P.threshold));
Count := Count + 1; Fields (Count) := Named ("verb", String_Item (Text (P.verb)));
case P.wording_version.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("wording_version", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("wording_version", Read_thinkthen_complete_wording_version_v1(P.wording_version.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_relate_question_document_v1;
function Read_thinkthen_complete_relation_endpoint_related_entity_v1 (P : access constant thinkthen_complete_relation_endpoint_related_entity_v1) return Value is
Fields : Field_List (1 .. 2 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("kind", String_Item (Text (P.kind)));
Count := Count + 1; Fields (Count) := Named ("name", String_Item (Text (P.name)));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_relation_endpoint_related_entity_v1;
function Read_thinkthen_complete_relation_endpoint_source_v1 (P : access constant thinkthen_complete_relation_endpoint_source_v1) return Value is
Fields : Field_List (1 .. 7 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
case P.file.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("file", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("file", String_Item (Text (P.file.value)));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.first_line.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("first_line", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("first_line", Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.first_line.value), Ada.Strings.Both)));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("kind", String_Item (Text (P.kind)));
case P.last_line.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("last_line", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("last_line", Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.last_line.value), Ada.Strings.Both)));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("name", String_Item (Text (P.name)));
Count := Count + 1; Fields (Count) := Named ("ordinal", Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.ordinal), Ada.Strings.Both)));
case P.c_record.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("record", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("record", Read_Json (P.c_record.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_relation_endpoint_source_v1;
function Read_thinkthen_complete_relation_endpoint_v1 (P : access constant thinkthen_complete_relation_endpoint_v1) return Value is

begin
if P = null then raise Constraint_Error with "missing native value"; end if;
case P.kind is
when 1 => return Read_thinkthen_complete_relation_endpoint_related_entity_v1 (P.data.related_entity);
when 2 => return Read_thinkthen_complete_relation_endpoint_source_v1 (P.data.source);
when others => raise Constraint_Error with "invalid native alternative";
end case;
end Read_thinkthen_complete_relation_endpoint_v1;
function Read_thinkthen_complete_related_entity_edge_v1 (P : access constant thinkthen_complete_related_entity_edge_v1) return Value is
Fields : Field_List (1 .. 5 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
case P.either.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("either", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("either", Boolean_Item (P.either.value /= 0));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("probability", Number_Item (Ada.Strings.Fixed.Trim (double'Image (P.probability), Ada.Strings.Both)));
Count := Count + 1; Fields (Count) := Named ("relation", String_Item (Text (P.relation)));
Count := Count + 1; Fields (Count) := Named ("source", Read_thinkthen_complete_relation_endpoint_v1(P.source));
Count := Count + 1; Fields (Count) := Named ("target", Read_thinkthen_complete_relation_endpoint_v1(P.target));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_related_entity_edge_v1;
function Read_thinkthen_complete_related_entity_edge_list_v1 (P : thinkthen_complete_related_entity_edge_list_v1) return Value is
Count : constant Natural := Index (P.len);
type Items_Type is array (1 .. Count) of access constant thinkthen_complete_related_entity_edge_v1 with Convention => C;
type Items_Access is access constant Items_Type;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Items_Access);
Items : Items_Access;
Items_Copy : Value_List (1 .. Count);
begin
if Count > 0 then Items := Pointer (P.data); end if;
for I in 1 .. Count loop
Items_Copy (I) := Read_thinkthen_complete_related_entity_edge_v1(Items (I));
end loop;
return Array_Item (Items_Copy);
end Read_thinkthen_complete_related_entity_edge_list_v1;
function Read_thinkthen_complete_relation_v1 (P : access constant thinkthen_complete_relation_v1) return Value is
Fields : Field_List (1 .. 13 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("answer", Read_thinkthen_complete_answers_v1(P.answer));
Count := Count + 1; Fields (Count) := Named ("answer_id", Read_thinkthen_complete_answer_id_v1(P.answer_id));
case P.file.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("file", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("file", String_Item (Text (P.file.value)));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.first_line.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("first_line", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("first_line", Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.first_line.value), Ada.Strings.Both)));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.index.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("index", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("index", Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.index.value), Ada.Strings.Both)));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.input.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("input", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("input", Read_Json (P.input.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.input_sources.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("input_sources", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("input_sources", Read_thinkthen_complete_session_input_source_list_v1(P.input_sources.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.last_line.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("last_line", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("last_line", Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.last_line.value), Ada.Strings.Both)));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("meta", Read_thinkthen_complete_meta_v1(P.meta));
case P.position.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("position", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("position", Read_thinkthen_complete_position_v1(P.position.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("question", Read_thinkthen_complete_relate_question_document_v1(P.question));
Count := Count + 1; Fields (Count) := Named ("schema", Read_thinkthen_complete_version_v1(P.schema));
Count := Count + 1; Fields (Count) := Named ("value", Read_thinkthen_complete_related_entity_edge_list_v1(P.value));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_relation_v1;
function Read_thinkthen_complete_session_packet_relate_aggregate_v1 (P : access constant thinkthen_complete_session_packet_relate_aggregate_v1) return Value is
Fields : Field_List (1 .. 3 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("function", String_Item (Text (P.c_function)));
Count := Count + 1; Fields (Count) := Named ("kind", String_Item (Text (P.kind)));
Count := Count + 1; Fields (Count) := Named ("value", Read_thinkthen_complete_relation_v1(P.value));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_session_packet_relate_aggregate_v1;
function Read_thinkthen_complete_request_function_v1 (P : access constant thinkthen_complete_request_function_v1) return Value is

begin
if P = null then raise Constraint_Error with "missing native value"; end if;
case P.kind is
when 1 => return String_Item ("decide");
when 2 => return String_Item ("choose");
when 3 => return String_Item ("tag");
when 4 => return String_Item ("score");
when 5 => return String_Item ("filter");
when 6 => return String_Item ("rank");
when 7 => return String_Item ("find");
when 8 => return String_Item ("annotate");
when 9 => return String_Item ("recognize");
when 10 => return String_Item ("relate");
when others => raise Constraint_Error with "invalid native enum";
end case;
end Read_thinkthen_complete_request_function_v1;
function Read_thinkthen_complete_json_list_v1 (P : thinkthen_complete_json_list_v1) return Value is
Count : constant Natural := Index (P.len);
type Items_Type is array (1 .. Count) of access constant thinkthen_complete_json_v1 with Convention => C;
type Items_Access is access constant Items_Type;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Items_Access);
Items : Items_Access;
Items_Copy : Value_List (1 .. Count);
begin
if Count > 0 then Items := Pointer (P.data); end if;
for I in 1 .. Count loop
Items_Copy (I) := Read_Json (Items (I));
end loop;
return Array_Item (Items_Copy);
end Read_thinkthen_complete_json_list_v1;
function Read_thinkthen_complete_session_probabilities_yes_no_v1 (P : access constant thinkthen_complete_session_probabilities_yes_no_v1) return Value is
Fields : Field_List (1 .. 2 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("kind", String_Item (Text (P.kind)));
Count := Count + 1; Fields (Count) := Named ("value", Number_Item (Ada.Strings.Fixed.Trim (double'Image (P.value), Ada.Strings.Both)));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_session_probabilities_yes_no_v1;
function Read_thinkthen_complete_session_named_probability_v1 (P : access constant thinkthen_complete_session_named_probability_v1) return Value is
Fields : Field_List (1 .. 2 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("name", String_Item (Text (P.name)));
Count := Count + 1; Fields (Count) := Named ("probability", Number_Item (Ada.Strings.Fixed.Trim (double'Image (P.probability), Ada.Strings.Both)));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_session_named_probability_v1;
function Read_thinkthen_complete_session_named_probability_list_v1 (P : thinkthen_complete_session_named_probability_list_v1) return Value is
Count : constant Natural := Index (P.len);
type Items_Type is array (1 .. Count) of access constant thinkthen_complete_session_named_probability_v1 with Convention => C;
type Items_Access is access constant Items_Type;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Items_Access);
Items : Items_Access;
Items_Copy : Value_List (1 .. Count);
begin
if Count > 0 then Items := Pointer (P.data); end if;
for I in 1 .. Count loop
Items_Copy (I) := Read_thinkthen_complete_session_named_probability_v1(Items (I));
end loop;
return Array_Item (Items_Copy);
end Read_thinkthen_complete_session_named_probability_list_v1;
function Read_thinkthen_complete_session_probabilities_named_v1 (P : access constant thinkthen_complete_session_probabilities_named_v1) return Value is
Fields : Field_List (1 .. 2 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("kind", String_Item (Text (P.kind)));
Count := Count + 1; Fields (Count) := Named ("value", Read_thinkthen_complete_session_named_probability_list_v1(P.value));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_session_probabilities_named_v1;
function Read_thinkthen_complete_session_probabilities_v1 (P : access constant thinkthen_complete_session_probabilities_v1) return Value is

begin
if P = null then raise Constraint_Error with "missing native value"; end if;
case P.kind is
when 1 => return Read_thinkthen_complete_session_probabilities_yes_no_v1 (P.data.yes_no);
when 2 => return Read_thinkthen_complete_session_probabilities_named_v1 (P.data.named);
when others => raise Constraint_Error with "invalid native alternative";
end case;
end Read_thinkthen_complete_session_probabilities_v1;
function Read_thinkthen_complete_token_usage_v1 (P : access constant thinkthen_complete_token_usage_v1) return Value is
Fields : Field_List (1 .. 2 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("input_tokens", Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.input_tokens), Ada.Strings.Both)));
Count := Count + 1; Fields (Count) := Named ("output_tokens", Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.output_tokens), Ada.Strings.Both)));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_token_usage_v1;
function Read_thinkthen_complete_session_question_detail_v1 (P : access constant thinkthen_complete_session_question_detail_v1) return Value is
Fields : Field_List (1 .. 24 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
case P.answer_id.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("answer_id", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("answer_id", Read_thinkthen_complete_answer_id_v1(P.answer_id.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("cached", Boolean_Item (P.cached /= 0));
case P.confidence.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("confidence", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("confidence", Number_Item (Ada.Strings.Fixed.Trim (double'Image (P.confidence.value), Ada.Strings.Both)));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("failed_questions", Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.failed_questions), Ada.Strings.Both)));
case P.failure.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("failure", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("failure", Read_thinkthen_complete_failure_v1(P.failure.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.failure_id.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("failure_id", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("failure_id", Read_thinkthen_complete_failure_id_v1(P.failure_id.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.input.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("input", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("input", Read_Json (P.input.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.input_source.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("input_source", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("input_source", Read_thinkthen_complete_physical_source_v1(P.input_source.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("input_sources", Read_thinkthen_complete_session_input_source_list_v1(P.input_sources));
Count := Count + 1; Fields (Count) := Named ("inputs", Read_thinkthen_complete_json_list_v1(P.inputs));
Count := Count + 1; Fields (Count) := Named ("model", String_Item (Text (P.model)));
Count := Count + 1; Fields (Count) := Named ("observations", Read_thinkthen_complete_observation_list_v1(P.observations));
case P.probabilities.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("probabilities", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("probabilities", Read_thinkthen_complete_session_probabilities_v1(P.probabilities.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("question", Read_thinkthen_complete_question_document_v1(P.question));
Count := Count + 1; Fields (Count) := Named ("question_sha256", String_Item (Text (P.question_sha256)));
Count := Count + 1; Fields (Count) := Named ("question_sources", Read_thinkthen_complete_question_source_list_v1(P.question_sources));
case P.raw_pick.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("raw_pick", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("raw_pick", String_Item (Text (P.raw_pick.value)));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.reported_usage.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("reported_usage", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("reported_usage", Read_thinkthen_complete_usage_v1(P.reported_usage.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("requests", Read_thinkthen_complete_string_list_v1(P.requests));
Count := Count + 1; Fields (Count) := Named ("requests_sent", Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.requests_sent), Ada.Strings.Both)));
case P.threshold.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("threshold", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("threshold", Read_thinkthen_complete_threshold_v1(P.threshold.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("url", String_Item (Text (P.url)));
case P.usage.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("usage", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("usage", Read_thinkthen_complete_token_usage_v1(P.usage.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.value.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("value", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("value", Read_thinkthen_complete_value_v1(P.value.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_session_question_detail_v1;
function Read_thinkthen_complete_session_observation_question_v1 (P : access constant thinkthen_complete_session_observation_question_v1) return Value is
Fields : Field_List (1 .. 6 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("detail", Read_thinkthen_complete_session_question_detail_v1(P.detail));
Count := Count + 1; Fields (Count) := Named ("index", Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.index), Ada.Strings.Both)));
Count := Count + 1; Fields (Count) := Named ("kind", String_Item (Text (P.kind)));
case P.member.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("member", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("member", String_Item (Text (P.member.value)));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("position", Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.position), Ada.Strings.Both)));
case P.stage.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("stage", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("stage", String_Item (Text (P.stage.value)));
when others => raise Constraint_Error with "invalid native presence";
end case;
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_session_observation_question_v1;
function Read_thinkthen_complete_session_judgment_decision_v1 (P : access constant thinkthen_complete_session_judgment_decision_v1) return Value is
Fields : Field_List (1 .. 2 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("kind", String_Item (Text (P.kind)));
case P.value.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("value", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("value", Boolean_Item (P.value.value /= 0));
when others => raise Constraint_Error with "invalid native presence";
end case;
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_session_judgment_decision_v1;
function Read_thinkthen_complete_session_judgment_choice_v1 (P : access constant thinkthen_complete_session_judgment_choice_v1) return Value is
Fields : Field_List (1 .. 2 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("kind", String_Item (Text (P.kind)));
case P.value.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("value", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("value", String_Item (Text (P.value.value)));
when others => raise Constraint_Error with "invalid native presence";
end case;
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_session_judgment_choice_v1;
function Read_thinkthen_complete_session_judgment_score_v1 (P : access constant thinkthen_complete_session_judgment_score_v1) return Value is
Fields : Field_List (1 .. 2 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("kind", String_Item (Text (P.kind)));
Count := Count + 1; Fields (Count) := Named ("value", Number_Item (Ada.Strings.Fixed.Trim (double'Image (P.value), Ada.Strings.Both)));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_session_judgment_score_v1;
function Read_thinkthen_complete_session_judgment_tags_v1 (P : access constant thinkthen_complete_session_judgment_tags_v1) return Value is
Fields : Field_List (1 .. 2 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("kind", String_Item (Text (P.kind)));
Count := Count + 1; Fields (Count) := Named ("value", Read_thinkthen_complete_string_list_v1(P.value));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_session_judgment_tags_v1;
function Read_thinkthen_complete_session_judgment_v1 (P : access constant thinkthen_complete_session_judgment_v1) return Value is

begin
if P = null then raise Constraint_Error with "missing native value"; end if;
case P.kind is
when 1 => return Read_thinkthen_complete_session_judgment_decision_v1 (P.data.decision);
when 2 => return Read_thinkthen_complete_session_judgment_choice_v1 (P.data.choice);
when 3 => return Read_thinkthen_complete_session_judgment_score_v1 (P.data.score);
when 4 => return Read_thinkthen_complete_session_judgment_tags_v1 (P.data.tags);
when others => raise Constraint_Error with "invalid native alternative";
end case;
end Read_thinkthen_complete_session_judgment_v1;
function Read_thinkthen_complete_session_observed_row_judgment_v1 (P : access constant thinkthen_complete_session_observed_row_judgment_v1) return Value is
Fields : Field_List (1 .. 2 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("kind", String_Item (Text (P.kind)));
Count := Count + 1; Fields (Count) := Named ("value", Read_thinkthen_complete_session_judgment_v1(P.value));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_session_observed_row_judgment_v1;
function Read_thinkthen_complete_annotation_value_decision_v1 (P : access constant thinkthen_complete_annotation_value_decision_v1) return Value is
Fields : Field_List (1 .. 2 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("kind", String_Item (Text (P.kind)));
case P.value.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("value", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("value", Boolean_Item (P.value.value /= 0));
when others => raise Constraint_Error with "invalid native presence";
end case;
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_annotation_value_decision_v1;
function Read_thinkthen_complete_annotation_value_choice_v1 (P : access constant thinkthen_complete_annotation_value_choice_v1) return Value is
Fields : Field_List (1 .. 2 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("kind", String_Item (Text (P.kind)));
case P.value.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("value", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("value", String_Item (Text (P.value.value)));
when others => raise Constraint_Error with "invalid native presence";
end case;
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_annotation_value_choice_v1;
function Read_thinkthen_complete_annotation_value_score_v1 (P : access constant thinkthen_complete_annotation_value_score_v1) return Value is
Fields : Field_List (1 .. 2 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("kind", String_Item (Text (P.kind)));
Count := Count + 1; Fields (Count) := Named ("value", Number_Item (Ada.Strings.Fixed.Trim (double'Image (P.value), Ada.Strings.Both)));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_annotation_value_score_v1;
function Read_thinkthen_complete_annotation_value_tags_v1 (P : access constant thinkthen_complete_annotation_value_tags_v1) return Value is
Fields : Field_List (1 .. 2 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("kind", String_Item (Text (P.kind)));
Count := Count + 1; Fields (Count) := Named ("value", Read_thinkthen_complete_string_list_v1(P.value));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_annotation_value_tags_v1;
function Read_thinkthen_complete_annotation_value_failed_v1 (P : access constant thinkthen_complete_annotation_value_failed_v1) return Value is
Fields : Field_List (1 .. 2 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("kind", String_Item (Text (P.kind)));
Count := Count + 1; Fields (Count) := Named ("value", Read_thinkthen_complete_failure_v1(P.value));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_annotation_value_failed_v1;
function Read_thinkthen_complete_annotation_value_v1 (P : access constant thinkthen_complete_annotation_value_v1) return Value is

begin
if P = null then raise Constraint_Error with "missing native value"; end if;
case P.kind is
when 1 => return Read_thinkthen_complete_annotation_value_decision_v1 (P.data.decision);
when 2 => return Read_thinkthen_complete_annotation_value_choice_v1 (P.data.choice);
when 3 => return Read_thinkthen_complete_annotation_value_score_v1 (P.data.score);
when 4 => return Read_thinkthen_complete_annotation_value_tags_v1 (P.data.tags);
when 5 => return Read_thinkthen_complete_annotation_value_failed_v1 (P.data.failed);
when others => raise Constraint_Error with "invalid native alternative";
end case;
end Read_thinkthen_complete_annotation_value_v1;
function Read_thinkthen_complete_session_annotation_v1 (P : access constant thinkthen_complete_session_annotation_v1) return Value is
Fields : Field_List (1 .. 2 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("name", String_Item (Text (P.name)));
Count := Count + 1; Fields (Count) := Named ("value", Read_thinkthen_complete_annotation_value_v1(P.value));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_session_annotation_v1;
function Read_thinkthen_complete_session_annotation_list_v1 (P : thinkthen_complete_session_annotation_list_v1) return Value is
Count : constant Natural := Index (P.len);
type Items_Type is array (1 .. Count) of access constant thinkthen_complete_session_annotation_v1 with Convention => C;
type Items_Access is access constant Items_Type;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Items_Access);
Items : Items_Access;
Items_Copy : Value_List (1 .. Count);
begin
if Count > 0 then Items := Pointer (P.data); end if;
for I in 1 .. Count loop
Items_Copy (I) := Read_thinkthen_complete_session_annotation_v1(Items (I));
end loop;
return Array_Item (Items_Copy);
end Read_thinkthen_complete_session_annotation_list_v1;
function Read_thinkthen_complete_session_observed_row_annotated_v1 (P : access constant thinkthen_complete_session_observed_row_annotated_v1) return Value is
Fields : Field_List (1 .. 2 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("kind", String_Item (Text (P.kind)));
Count := Count + 1; Fields (Count) := Named ("value", Read_thinkthen_complete_session_annotation_list_v1(P.value));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_session_observed_row_annotated_v1;
function Read_thinkthen_complete_recognition_edge_v1 (P : access constant thinkthen_complete_recognition_edge_v1) return Value is
Fields : Field_List (1 .. 5 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("either", Boolean_Item (P.either /= 0));
Count := Count + 1; Fields (Count) := Named ("probability", Number_Item (Ada.Strings.Fixed.Trim (double'Image (P.probability), Ada.Strings.Both)));
Count := Count + 1; Fields (Count) := Named ("relation", String_Item (Text (P.relation)));
Count := Count + 1; Fields (Count) := Named ("source", Read_thinkthen_complete_entity_v1(P.source));
Count := Count + 1; Fields (Count) := Named ("target", Read_thinkthen_complete_entity_v1(P.target));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_recognition_edge_v1;
function Read_thinkthen_complete_recognition_edge_list_v1 (P : thinkthen_complete_recognition_edge_list_v1) return Value is
Count : constant Natural := Index (P.len);
type Items_Type is array (1 .. Count) of access constant thinkthen_complete_recognition_edge_v1 with Convention => C;
type Items_Access is access constant Items_Type;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Items_Access);
Items : Items_Access;
Items_Copy : Value_List (1 .. Count);
begin
if Count > 0 then Items := Pointer (P.data); end if;
for I in 1 .. Count loop
Items_Copy (I) := Read_thinkthen_complete_recognition_edge_v1(Items (I));
end loop;
return Array_Item (Items_Copy);
end Read_thinkthen_complete_recognition_edge_list_v1;
function Read_thinkthen_complete_session_recognition_v1 (P : access constant thinkthen_complete_session_recognition_v1) return Value is
Fields : Field_List (1 .. 4 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("entities", Read_thinkthen_complete_entity_list_v1(P.entities));
Count := Count + 1; Fields (Count) := Named ("mode", Read_thinkthen_complete_recognition_mode_v1(P.mode));
case P.proposals.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("proposals", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("proposals", Read_thinkthen_complete_boundary_proposal_list_v1(P.proposals.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.relations.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("relations", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("relations", Read_thinkthen_complete_recognition_edge_list_v1(P.relations.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_session_recognition_v1;
function Read_thinkthen_complete_session_observed_row_recognized_v1 (P : access constant thinkthen_complete_session_observed_row_recognized_v1) return Value is
Fields : Field_List (1 .. 2 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("kind", String_Item (Text (P.kind)));
Count := Count + 1; Fields (Count) := Named ("value", Read_thinkthen_complete_session_recognition_v1(P.value));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_session_observed_row_recognized_v1;
function Read_thinkthen_complete_session_observed_row_find_v1 (P : access constant thinkthen_complete_session_observed_row_find_v1) return Value is
Fields : Field_List (1 .. 2 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("kind", String_Item (Text (P.kind)));
case P.value.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("value", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("value", Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.value.value), Ada.Strings.Both)));
when others => raise Constraint_Error with "invalid native presence";
end case;
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_session_observed_row_find_v1;
function Read_thinkthen_complete_entity_document_v1 (P : access constant thinkthen_complete_entity_document_v1) return Value is
Fields : Field_List (1 .. 2 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("kind", String_Item (Text (P.kind)));
Count := Count + 1; Fields (Count) := Named ("name", String_Item (Text (P.name)));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_entity_document_v1;
function Read_thinkthen_complete_session_relation_edge_v1 (P : access constant thinkthen_complete_session_relation_edge_v1) return Value is
Fields : Field_List (1 .. 5 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("either", Boolean_Item (P.either /= 0));
Count := Count + 1; Fields (Count) := Named ("probability", Number_Item (Ada.Strings.Fixed.Trim (double'Image (P.probability), Ada.Strings.Both)));
Count := Count + 1; Fields (Count) := Named ("relation", String_Item (Text (P.relation)));
Count := Count + 1; Fields (Count) := Named ("source", Read_thinkthen_complete_entity_document_v1(P.source));
Count := Count + 1; Fields (Count) := Named ("target", Read_thinkthen_complete_entity_document_v1(P.target));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_session_relation_edge_v1;
function Read_thinkthen_complete_session_relation_edge_list_v1 (P : thinkthen_complete_session_relation_edge_list_v1) return Value is
Count : constant Natural := Index (P.len);
type Items_Type is array (1 .. Count) of access constant thinkthen_complete_session_relation_edge_v1 with Convention => C;
type Items_Access is access constant Items_Type;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Items_Access);
Items : Items_Access;
Items_Copy : Value_List (1 .. Count);
begin
if Count > 0 then Items := Pointer (P.data); end if;
for I in 1 .. Count loop
Items_Copy (I) := Read_thinkthen_complete_session_relation_edge_v1(Items (I));
end loop;
return Array_Item (Items_Copy);
end Read_thinkthen_complete_session_relation_edge_list_v1;
function Read_thinkthen_complete_session_observed_row_relations_v1 (P : access constant thinkthen_complete_session_observed_row_relations_v1) return Value is
Fields : Field_List (1 .. 2 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("kind", String_Item (Text (P.kind)));
Count := Count + 1; Fields (Count) := Named ("value", Read_thinkthen_complete_session_relation_edge_list_v1(P.value));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_session_observed_row_relations_v1;
function Read_thinkthen_complete_session_observed_row_v1 (P : access constant thinkthen_complete_session_observed_row_v1) return Value is

begin
if P = null then raise Constraint_Error with "missing native value"; end if;
case P.kind is
when 1 => return Read_thinkthen_complete_session_observed_row_judgment_v1 (P.data.judgment);
when 2 => return Read_thinkthen_complete_session_observed_row_annotated_v1 (P.data.annotated);
when 3 => return Read_thinkthen_complete_session_observed_row_recognized_v1 (P.data.recognized);
when 4 => return Read_thinkthen_complete_session_observed_row_find_v1 (P.data.find);
when 5 => return Read_thinkthen_complete_session_observed_row_relations_v1 (P.data.relations);
when others => raise Constraint_Error with "invalid native alternative";
end case;
end Read_thinkthen_complete_session_observed_row_v1;
function Read_thinkthen_complete_session_observation_row_v1 (P : access constant thinkthen_complete_session_observation_row_v1) return Value is
Fields : Field_List (1 .. 3 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("index", Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.index), Ada.Strings.Both)));
Count := Count + 1; Fields (Count) := Named ("kind", String_Item (Text (P.kind)));
Count := Count + 1; Fields (Count) := Named ("value", Read_thinkthen_complete_session_observed_row_v1(P.value));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_session_observation_row_v1;
function Read_thinkthen_complete_session_observation_v1 (P : access constant thinkthen_complete_session_observation_v1) return Value is

begin
if P = null then raise Constraint_Error with "missing native value"; end if;
case P.kind is
when 1 => return Read_thinkthen_complete_session_observation_question_v1 (P.data.question);
when 2 => return Read_thinkthen_complete_session_observation_row_v1 (P.data.row);
when others => raise Constraint_Error with "invalid native alternative";
end case;
end Read_thinkthen_complete_session_observation_v1;
function Read_thinkthen_complete_session_packet_observation_v1 (P : access constant thinkthen_complete_session_packet_observation_v1) return Value is
Fields : Field_List (1 .. 3 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("function", Read_thinkthen_complete_request_function_v1(P.c_function));
Count := Count + 1; Fields (Count) := Named ("kind", String_Item (Text (P.kind)));
Count := Count + 1; Fields (Count) := Named ("value", Read_thinkthen_complete_session_observation_v1(P.value));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_session_packet_observation_v1;
function Read_thinkthen_complete_call_id_v1 (P : access constant thinkthen_complete_call_id_v1) return Value is

begin
if P = null then raise Constraint_Error with "missing native value"; end if;
return String_Item (Text (P.value));
end Read_thinkthen_complete_call_id_v1;
function Read_thinkthen_complete_usage_persistence_v1 (P : access constant thinkthen_complete_usage_persistence_v1) return Value is

begin
if P = null then raise Constraint_Error with "missing native value"; end if;
case P.kind is
when 1 => return String_Item ("disabled");
when 2 => return String_Item ("pending");
when 3 => return String_Item ("written");
when 4 => return String_Item ("failed");
when others => raise Constraint_Error with "invalid native enum";
end case;
end Read_thinkthen_complete_usage_persistence_v1;
function Read_thinkthen_complete_persistence_observation_v1 (P : access constant thinkthen_complete_persistence_observation_v1) return Value is
Fields : Field_List (1 .. 3 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
case P.advice.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("advice", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("advice", String_Item (Text (P.advice.value)));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("observed_at", String_Item (Text (P.observed_at)));
Count := Count + 1; Fields (Count) := Named ("state", Read_thinkthen_complete_usage_persistence_v1(P.state));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_persistence_observation_v1;
function Read_thinkthen_complete_facts_v1 (P : access constant thinkthen_complete_facts_v1) return Value is
Fields : Field_List (1 .. 15 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
case P.attempts.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("attempts", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("attempts", Read_thinkthen_complete_attempt_list_v1(P.attempts.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("cache_answers", Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.cache_answers), Ada.Strings.Both)));
Count := Count + 1; Fields (Count) := Named ("call_id", Read_thinkthen_complete_call_id_v1(P.call_id));
case P.estimated_cost_usd.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("estimated_cost_usd", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("estimated_cost_usd", String_Item (Text (P.estimated_cost_usd.value)));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.held_model_mismatch.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("held_model_mismatch", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("held_model_mismatch", Boolean_Item (P.held_model_mismatch.value /= 0));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.input_tokens.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("input_tokens", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("input_tokens", Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.input_tokens.value), Ada.Strings.Both)));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("largest_request_bytes", Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.largest_request_bytes), Ada.Strings.Both)));
case P.largest_request_estimated_input_tokens.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("largest_request_estimated_input_tokens", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("largest_request_estimated_input_tokens", Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.largest_request_estimated_input_tokens.value), Ada.Strings.Both)));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.model.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("model", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("model", String_Item (Text (P.model.value)));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.output_tokens.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("output_tokens", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("output_tokens", Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.output_tokens.value), Ada.Strings.Both)));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("records", Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.records), Ada.Strings.Both)));
Count := Count + 1; Fields (Count) := Named ("requests_sent", Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.requests_sent), Ada.Strings.Both)));
Count := Count + 1; Fields (Count) := Named ("seconds", Number_Item (Ada.Strings.Fixed.Trim (double'Image (P.seconds), Ada.Strings.Both)));
Count := Count + 1; Fields (Count) := Named ("token_estimate_method", String_Item (Text (P.token_estimate_method)));
case P.usage_persistence.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("usage_persistence", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("usage_persistence", Read_thinkthen_complete_persistence_observation_v1(P.usage_persistence.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_facts_v1;
function Read_thinkthen_complete_input_denial_initial_request_v1 (P : access constant thinkthen_complete_input_denial_initial_request_v1) return Value is
Fields : Field_List (1 .. 2 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("kind", String_Item (Text (P.kind)));
Count := Count + 1; Fields (Count) := Named ("limit", Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.limit), Ada.Strings.Both)));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_input_denial_initial_request_v1;
function Read_thinkthen_complete_input_denial_additional_request_v1 (P : access constant thinkthen_complete_input_denial_additional_request_v1) return Value is
Fields : Field_List (1 .. 2 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("kind", String_Item (Text (P.kind)));
Count := Count + 1; Fields (Count) := Named ("limit", Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.limit), Ada.Strings.Both)));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_input_denial_additional_request_v1;
function Read_thinkthen_complete_input_denial_retry_v1 (P : access constant thinkthen_complete_input_denial_retry_v1) return Value is
Fields : Field_List (1 .. 3 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("kind", String_Item (Text (P.kind)));
Count := Count + 1; Fields (Count) := Named ("last_status", Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.last_status), Ada.Strings.Both)));
Count := Count + 1; Fields (Count) := Named ("limit", Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.limit), Ada.Strings.Both)));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_input_denial_retry_v1;
function Read_thinkthen_complete_input_denial_v1 (P : access constant thinkthen_complete_input_denial_v1) return Value is

begin
if P = null then raise Constraint_Error with "missing native value"; end if;
case P.kind is
when 1 => return Read_thinkthen_complete_input_denial_initial_request_v1 (P.data.initial_request);
when 2 => return Read_thinkthen_complete_input_denial_additional_request_v1 (P.data.additional_request);
when 3 => return Read_thinkthen_complete_input_denial_retry_v1 (P.data.retry);
when others => raise Constraint_Error with "invalid native alternative";
end case;
end Read_thinkthen_complete_input_denial_v1;
function Read_thinkthen_complete_failure_kind_v1 (P : access constant thinkthen_complete_failure_kind_v1) return Value is

begin
if P = null then raise Constraint_Error with "missing native value"; end if;
case P.kind is
when 1 => return String_Item ("usage");
when 2 => return String_Item ("backend");
when 3 => return String_Item ("local");
when 4 => return String_Item ("cancelled");
when 5 => return String_Item ("deadline");
when 6 => return String_Item ("defect");
when others => raise Constraint_Error with "invalid native enum";
end case;
end Read_thinkthen_complete_failure_kind_v1;
function Read_thinkthen_complete_send_denial_before_first_send_v1 (P : access constant thinkthen_complete_send_denial_before_first_send_v1) return Value is
Fields : Field_List (1 .. 1 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("kind", String_Item (Text (P.kind)));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_send_denial_before_first_send_v1;
function Read_thinkthen_complete_send_denial_before_additional_send_v1 (P : access constant thinkthen_complete_send_denial_before_additional_send_v1) return Value is
Fields : Field_List (1 .. 1 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("kind", String_Item (Text (P.kind)));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_send_denial_before_additional_send_v1;
function Read_thinkthen_complete_send_denial_before_retry_v1 (P : access constant thinkthen_complete_send_denial_before_retry_v1) return Value is
Fields : Field_List (1 .. 2 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("kind", String_Item (Text (P.kind)));
Count := Count + 1; Fields (Count) := Named ("last_status", Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.last_status), Ada.Strings.Both)));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_send_denial_before_retry_v1;
function Read_thinkthen_complete_send_denial_v1 (P : access constant thinkthen_complete_send_denial_v1) return Value is

begin
if P = null then raise Constraint_Error with "missing native value"; end if;
case P.kind is
when 1 => return Read_thinkthen_complete_send_denial_before_first_send_v1 (P.data.before_first_send);
when 2 => return Read_thinkthen_complete_send_denial_before_additional_send_v1 (P.data.before_additional_send);
when 3 => return Read_thinkthen_complete_send_denial_before_retry_v1 (P.data.before_retry);
when others => raise Constraint_Error with "invalid native alternative";
end case;
end Read_thinkthen_complete_send_denial_v1;
function Read_thinkthen_complete_stop_cause_v1 (P : access constant thinkthen_complete_stop_cause_v1) return Value is

begin
if P = null then raise Constraint_Error with "missing native value"; end if;
case P.kind is
when 1 => return String_Item ("usage");
when 2 => return String_Item ("local");
when 3 => return String_Item ("no_key");
when 4 => return String_Item ("transport");
when 5 => return String_Item ("status");
when 6 => return String_Item ("too_large");
when 7 => return String_Item ("reply");
when 8 => return String_Item ("backend");
when 9 => return String_Item ("cancelled");
when 10 => return String_Item ("deadline");
when 11 => return String_Item ("defect");
when others => raise Constraint_Error with "invalid native enum";
end case;
end Read_thinkthen_complete_stop_cause_v1;
function Read_thinkthen_complete_stopped_v1 (P : access constant thinkthen_complete_stopped_v1) return Value is
Fields : Field_List (1 .. 4 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
case P.c_at.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("at", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("at", Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.c_at.value), Ada.Strings.Both)));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("cause", Read_thinkthen_complete_stop_cause_v1(P.cause));
Count := Count + 1; Fields (Count) := Named ("retryable", Boolean_Item (P.retryable /= 0));
case P.status.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("status", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("status", Number_Item (Ada.Strings.Fixed.Trim (Interfaces.Unsigned_64'Image (P.status.value), Ada.Strings.Both)));
when others => raise Constraint_Error with "invalid native presence";
end case;
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_stopped_v1;
function Read_thinkthen_complete_error_v1 (P : access constant thinkthen_complete_error_v1) return Value is
Fields : Field_List (1 .. 6 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
case P.estimated_input_denial.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("estimated_input_denial", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("estimated_input_denial", Read_thinkthen_complete_input_denial_v1(P.estimated_input_denial.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("kind", Read_thinkthen_complete_failure_kind_v1(P.kind));
Count := Count + 1; Fields (Count) := Named ("message", String_Item (Text (P.message)));
Count := Count + 1; Fields (Count) := Named ("retryable", Boolean_Item (P.retryable /= 0));
case P.send_budget_denial.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("send_budget_denial", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("send_budget_denial", Read_thinkthen_complete_send_denial_v1(P.send_budget_denial.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("stopped", Read_thinkthen_complete_stopped_v1(P.stopped));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_error_v1;
function Read_thinkthen_complete_call_error_v1 (P : access constant thinkthen_complete_call_error_v1) return Value is
Fields : Field_List (1 .. 2 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
Count := Count + 1; Fields (Count) := Named ("error", Read_thinkthen_complete_error_v1(P.error));
case P.facts.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("facts", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("facts", Read_thinkthen_complete_facts_v1(P.facts.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_call_error_v1;
function Read_thinkthen_complete_session_packet_terminal_v1 (P : access constant thinkthen_complete_session_packet_terminal_v1) return Value is
Fields : Field_List (1 .. 3 + (if P = null then 0 else Index (P.extensions.len)));
Count : Natural := 0;
begin
if P = null then raise Constraint_Error with "missing native value"; end if;
case P.facts.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("facts", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("facts", Read_thinkthen_complete_facts_v1(P.facts.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
case P.failure.presence is
when 0 => null;
when 1 => Count := Count + 1; Fields (Count) := Named ("failure", Null_Item);
when 2 => Count := Count + 1; Fields (Count) := Named ("failure", Read_thinkthen_complete_call_error_v1(P.failure.value));
when others => raise Constraint_Error with "invalid native presence";
end case;
Count := Count + 1; Fields (Count) := Named ("kind", String_Item (Text (P.kind)));
declare
Extension_Count : constant Natural := Index (P.extensions.len);
type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;
type Extension_Access is access constant Extension_Array;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);
Extensions : Extension_Access;
begin
if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;
for I in 1 .. Extension_Count loop
Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));
end loop;
end;
return Object_Item (Fields (1 .. Count));
end Read_thinkthen_complete_session_packet_terminal_v1;
function Read_thinkthen_complete_session_packet_v1 (P : access constant thinkthen_complete_session_packet_v1) return Value is

begin
if P = null then raise Constraint_Error with "missing native value"; end if;
case P.kind is
when 1 => return Read_thinkthen_complete_session_packet_decide_row_v1 (P.data.decide_row);
when 2 => return Read_thinkthen_complete_session_packet_choose_row_v1 (P.data.choose_row);
when 3 => return Read_thinkthen_complete_session_packet_tag_row_v1 (P.data.tag_row);
when 4 => return Read_thinkthen_complete_session_packet_score_row_v1 (P.data.score_row);
when 5 => return Read_thinkthen_complete_session_packet_filter_row_v1 (P.data.filter_row);
when 6 => return Read_thinkthen_complete_session_packet_annotate_row_v1 (P.data.annotate_row);
when 7 => return Read_thinkthen_complete_session_packet_decide_aggregate_v1 (P.data.decide_aggregate);
when 8 => return Read_thinkthen_complete_session_packet_choose_aggregate_v1 (P.data.choose_aggregate);
when 9 => return Read_thinkthen_complete_session_packet_tag_aggregate_v1 (P.data.tag_aggregate);
when 10 => return Read_thinkthen_complete_session_packet_score_aggregate_v1 (P.data.score_aggregate);
when 11 => return Read_thinkthen_complete_session_packet_filter_aggregate_v1 (P.data.filter_aggregate);
when 12 => return Read_thinkthen_complete_session_packet_rank_aggregate_v1 (P.data.rank_aggregate);
when 13 => return Read_thinkthen_complete_session_packet_find_aggregate_v1 (P.data.find_aggregate);
when 14 => return Read_thinkthen_complete_session_packet_annotate_aggregate_v1 (P.data.annotate_aggregate);
when 15 => return Read_thinkthen_complete_session_packet_recognize_aggregate_v1 (P.data.recognize_aggregate);
when 16 => return Read_thinkthen_complete_session_packet_relate_aggregate_v1 (P.data.relate_aggregate);
when 17 => return Read_thinkthen_complete_session_packet_observation_v1 (P.data.observation);
when 18 => return Read_thinkthen_complete_session_packet_terminal_v1 (P.data.terminal);
when others => raise Constraint_Error with "invalid native alternative";
end case;
end Read_thinkthen_complete_session_packet_v1;
   function Collect (Owner : in out Session) return Completion is
      Owned : Packet_Lists.Vector;
      Packet : Thinkthen.Sessions.Packet;
      Read : Read_Status;
      Result : Completion;
   begin
      loop
         Thinkthen.Sessions.Next_Packet (Owner, Packet, Read);
         exit when Read = Finished;
         if Read /= Thinkthen.Sessions.Result then
            Result.Valid := False;
         else
            declare
               Item : constant Value := Read_thinkthen_complete_session_packet_v1 (View (Packet));
            begin
               if As_String (Field_Value (Item, "kind")) = "terminal" then
                  Result.Terminal_Count := Result.Terminal_Count + 1;
                  Result.Terminal := Item;
               else
                  if Result.Terminal_Count > 0 then Result.Valid := False; end if;
                  Owned.Append (Item);
               end if;
            end;
         end if;
      end loop;
      declare
         Items : Value_List (1 .. Natural (Owned.Length));
      begin
         for I in Items'Range loop Items (I) := Owned (I); end loop;
         Result.Packets := Array_Item (Items);
      end;
      return Result;
   end Collect;
   function Status (Call : Completion) return Completion_Status is
   begin
      if not Call.Valid or else Call.Terminal_Count /= 1 then return Malformed; end if;
      if Has_Field (Call.Terminal, "failure") then
         if Kind (Field_Value (Call.Terminal, "failure")) /= Object_Value then return Malformed; end if;
         return Failed;
      end if;
      return Succeeded;
   end Status;
   function Packet_Count (Call : Completion) return Natural is (Length (Call.Packets));
   function Has_Facts (Call : Completion) return Boolean is
     (Call.Terminal_Count = 1 and then Has_Field (Call.Terminal, "facts") and then
       Kind (Field_Value (Call.Terminal, "facts")) = Object_Value);
   function Call_Facts (Call : Completion) return Facts is ((Data => Field_Value (Call.Terminal, "facts")));
   function Call_Failure (Call : Completion) return Failure is ((Data => Field_Value (Call.Terminal, "failure")));
   function Kind (Error : Failure) return Error_Status is
      Name : constant String := As_String (Field_Value (Field_Value (Error.Data, "error"), "kind"));
   begin
      return Error_Status'Value (Name);
   end Kind;
   function Message (Error : Failure) return String is (As_String (Field_Value (Field_Value (Error.Data, "error"), "message")));
   function Retryable (Error : Failure) return Boolean is (As_Boolean (Field_Value (Field_Value (Error.Data, "error"), "retryable")));
   function Has_Facts (Error : Failure) return Boolean is (Has_Field (Error.Data, "facts") and then Kind (Field_Value (Error.Data, "facts")) = Object_Value);
   function Failure_Facts (Error : Failure) return Facts is ((Data => Field_Value (Error.Data, "facts")));
   function Requests_Sent (Item : Facts) return Interfaces.Unsigned_64 is (Interfaces.Unsigned_64'Value (As_Number (Field_Value (Item.Data, "requests_sent"))));
   function Records (Item : Facts) return Interfaces.Unsigned_64 is (Interfaces.Unsigned_64'Value (As_Number (Field_Value (Item.Data, "records"))));
   function Scalar_Packet (Packet : Value; Judgment : String) return Boolean is
   begin
      if As_String (Field_Value (Packet, "kind")) /= "observation" then return False; end if;
if As_String (Field_Value (Field_Value (Packet, "value"), "kind")) /= "row" then return False; end if;
if As_String (Field_Value (Field_Value (Field_Value (Packet, "value"), "value"), "kind")) /= "judgment" then return False; end if;
      return As_String (Field_Value (Field_Value (Field_Value (Field_Value (Packet, "value"), "value"), "value"), "kind")) = Judgment;
   exception
      when Constraint_Error => return False;
   end Scalar_Packet;
   function Answer (Call : Completion; Function_Name, Judgment : String; Retained : Boolean := False) return Value is
      package Answer_Lists is new Ada.Containers.Vectors (Positive, Value);
      Items : Answer_Lists.Vector;
      Collection : constant Boolean := Function_Name = "filter" or else Function_Name = "rank" or else Function_Name = "annotate" or else Function_Name = "recognize";
   begin
      if not Retained then
      case Status (Call) is
         when Failed => raise Session_Failure with Message (Call_Failure (Call));
         when Malformed => raise Answer_Cardinality_Error with "expected exactly one valid terminal";
         when Succeeded => null;
      end case;
      end if;
      for I in 1 .. Length (Call.Packets) loop
         declare
            Packet : constant Value := Element (Call.Packets, I);
         begin
            if Has_Field (Packet, "function") and then As_String (Field_Value (Packet, "function")) = Function_Name then
               if Judgment /= "" then
                  if Scalar_Packet (Packet, Judgment) then Items.Append (Field_Value (Field_Value (Field_Value (Field_Value (Packet, "value"), "value"), "value"), "value")); end if;
               elsif Collection and then As_String (Field_Value (Packet, "kind")) = "row" then
                  Items.Append (Field_Value (Packet, "value"));
               elsif As_String (Field_Value (Packet, "kind")) = "aggregate" then
                  declare Payload : constant Value := Field_Value (Packet, "value"); begin
                     if Collection then
                        for J in 1 .. Length (Payload) loop Items.Append (Element (Payload, J)); end loop;
                     else Items.Append (Payload); end if;
                  end;
               end if;
            end if;
         end;
      end loop;
      if Collection or else Retained then
         declare
            Copy : Value_List (1 .. Natural (Items.Length));
         begin
            for I in Copy'Range loop Copy (I) := Items (I); end loop;
            return Array_Item (Copy);
         end;
      end if;
      if Natural (Items.Length) /= 1 then raise Answer_Cardinality_Error with "expected exactly one matching answer"; end if;
      return Items (1);
   end Answer;
   function Source_Name (Edge : Value) return String is (As_String (Field_Value (Field_Value (Edge, "source"), "name")));
   function Target_Name (Edge : Value) return String is (As_String (Field_Value (Field_Value (Edge, "target"), "name")));
   function Relation_Name (Edge : Value) return String is (As_String (Field_Value (Edge, "relation")));
   function Cache_Answers (Item : Facts) return Interfaces.Unsigned_64 is (Interfaces.Unsigned_64'Value (As_Number (Field_Value (Item.Data, "cache_answers"))));
function Call_Id (Item : Facts) return String is (As_String (Field_Value (Item.Data, "call_id")));
function Estimated_Cost_Usd (Item : Facts) return Text_Field is
begin
if not Has_Field (Item.Data, "estimated_cost_usd") then return (State => Missing_Field); end if;
if Kind (Field_Value (Item.Data, "estimated_cost_usd")) = Null_Value then return (State => Null_Field); end if;
return (State => Present_Field, Value => To_Unbounded_String (As_String (Field_Value (Item.Data, "estimated_cost_usd"))));
end Estimated_Cost_Usd;
function Held_Model_Mismatch (Item : Facts) return Boolean_Field is
begin
if not Has_Field (Item.Data, "held_model_mismatch") then return (State => Missing_Field); end if;
if Kind (Field_Value (Item.Data, "held_model_mismatch")) = Null_Value then return (State => Null_Field); end if;
return (State => Present_Field, Value => As_Boolean (Field_Value (Item.Data, "held_model_mismatch")));
end Held_Model_Mismatch;
function Input_Tokens (Item : Facts) return Count_Field is
begin
if not Has_Field (Item.Data, "input_tokens") then return (State => Missing_Field); end if;
if Kind (Field_Value (Item.Data, "input_tokens")) = Null_Value then return (State => Null_Field); end if;
return (State => Present_Field, Value => Interfaces.Unsigned_64'Value (As_Number (Field_Value (Item.Data, "input_tokens"))));
end Input_Tokens;
function Largest_Request_Bytes (Item : Facts) return Interfaces.Unsigned_64 is (Interfaces.Unsigned_64'Value (As_Number (Field_Value (Item.Data, "largest_request_bytes"))));
function Largest_Request_Estimated_Input_Tokens (Item : Facts) return Count_Field is
begin
if not Has_Field (Item.Data, "largest_request_estimated_input_tokens") then return (State => Missing_Field); end if;
if Kind (Field_Value (Item.Data, "largest_request_estimated_input_tokens")) = Null_Value then return (State => Null_Field); end if;
return (State => Present_Field, Value => Interfaces.Unsigned_64'Value (As_Number (Field_Value (Item.Data, "largest_request_estimated_input_tokens"))));
end Largest_Request_Estimated_Input_Tokens;
function Model (Item : Facts) return Text_Field is
begin
if not Has_Field (Item.Data, "model") then return (State => Missing_Field); end if;
if Kind (Field_Value (Item.Data, "model")) = Null_Value then return (State => Null_Field); end if;
return (State => Present_Field, Value => To_Unbounded_String (As_String (Field_Value (Item.Data, "model"))));
end Model;
function Output_Tokens (Item : Facts) return Count_Field is
begin
if not Has_Field (Item.Data, "output_tokens") then return (State => Missing_Field); end if;
if Kind (Field_Value (Item.Data, "output_tokens")) = Null_Value then return (State => Null_Field); end if;
return (State => Present_Field, Value => Interfaces.Unsigned_64'Value (As_Number (Field_Value (Item.Data, "output_tokens"))));
end Output_Tokens;
function Seconds (Item : Facts) return Long_Float is (Long_Float'Value (As_Number (Field_Value (Item.Data, "seconds"))));
function Token_Estimate_Method (Item : Facts) return String is (As_String (Field_Value (Item.Data, "token_estimate_method")));
function Retained_Values (Call : Completion; Function_Name : Function_Kind) return Value_List is
Item : Value;
begin
case Function_Name is
when Decide_Function => Item := Answer (Call, "decide", "decision", True);
when Choose_Function => Item := Answer (Call, "choose", "choice", True);
when Tag_Function => Item := Answer (Call, "tag", "tags", True);
when Score_Function => Item := Answer (Call, "score", "score", True);
when Filter_Function => Item := Answer (Call, "filter", "", True);
when Rank_Function => Item := Answer (Call, "rank", "", True);
when Find_Function => Item := Answer (Call, "find", "", True);
when Annotate_Function => Item := Answer (Call, "annotate", "", True);
when Recognize_Function => Item := Answer (Call, "recognize", "", True);
when Relate_Function => Item := Answer (Call, "relate", "", True);
end case;
declare Copy : Value_List (1 .. Length (Item)); begin
for I in Copy'Range loop Copy (I) := Element (Item, I); end loop;
return Copy;
end;
end Retained_Values;
function Resolved_Decide (Call : Completion; Row : Value; Ordinal : Positive) return Boolean is
Row_Index : constant Value := (if Has_Field (Row, "index") and then Kind (Field_Value (Row, "index")) /= Null_Value then Field_Value (Row, "index") else Number_Item (Ada.Strings.Fixed.Trim (Natural'Image (Ordinal - 1), Ada.Strings.Both)));
begin
for J in 1 .. Length (Call.Packets) loop
declare Packet : constant Value := Element (Call.Packets, J); begin
if As_String (Field_Value (Packet, "kind")) = "observation" and then As_String (Field_Value (Field_Value (Packet, "value"), "kind")) = "row" and then As_String (Field_Value (Field_Value (Field_Value (Packet, "value"), "value"), "kind")) = "judgment" and then As_String (Field_Value (Field_Value (Field_Value (Field_Value (Packet, "value"), "value"), "value"), "kind")) = "decision" and then As_String (Field_Value (Packet, "function")) = "decide" and then Field_Value (Field_Value (Packet, "value"), "index") = Row_Index and then Kind (Field_Value (Field_Value (Field_Value (Field_Value (Packet, "value"), "value"), "value"), "value")) /= Null_Value then return True; end if;
exception when Constraint_Error => null; end;
end loop;
return False;
end Resolved_Decide;
function Resolved_Choose (Call : Completion; Row : Value; Ordinal : Positive) return Boolean is
Row_Index : constant Value := (if Has_Field (Row, "index") and then Kind (Field_Value (Row, "index")) /= Null_Value then Field_Value (Row, "index") else Number_Item (Ada.Strings.Fixed.Trim (Natural'Image (Ordinal - 1), Ada.Strings.Both)));
begin
for J in 1 .. Length (Call.Packets) loop
declare Packet : constant Value := Element (Call.Packets, J); begin
if As_String (Field_Value (Packet, "kind")) = "observation" and then As_String (Field_Value (Field_Value (Packet, "value"), "kind")) = "row" and then As_String (Field_Value (Field_Value (Field_Value (Packet, "value"), "value"), "kind")) = "judgment" and then As_String (Field_Value (Field_Value (Field_Value (Field_Value (Packet, "value"), "value"), "value"), "kind")) = "choice" and then As_String (Field_Value (Packet, "function")) = "choose" and then Field_Value (Field_Value (Packet, "value"), "index") = Row_Index and then Kind (Field_Value (Field_Value (Field_Value (Field_Value (Packet, "value"), "value"), "value"), "value")) /= Null_Value then return True; end if;
exception when Constraint_Error => null; end;
end loop;
return False;
end Resolved_Choose;
function Record_Value (Item : Receipt) return Value is (Item.Data);
function Read_Choice (Item : Value) return Choice is
begin
   if Kind (Item) = Null_Value then return (Known => False); end if;
   return (Known => True, Value => To_Unbounded_String (As_String (Item)));
end Read_Choice;
function Texts (Item : Value) return Text_List is
   Result : Text_List (1 .. Length (Item));
begin
   for I in Result'Range loop Result (I) := To_Unbounded_String (As_String (Element (Item, I))); end loop;
   return Result;
end Texts;
function Receipts (Call : Completion; Function_Name : String; Is_Array : Boolean) return Value_List is
   Owned : Packet_Lists.Vector;
begin
   case Status (Call) is
      when Failed => raise Session_Failure with Message (Call_Failure (Call));
      when Malformed => raise Answer_Cardinality_Error with "expected one valid terminal";
      when Succeeded => null;
   end case;
   for I in 1 .. Length (Call.Packets) loop
      declare Packet : constant Value := Element (Call.Packets, I); begin
         if Has_Field (Packet, "function") and then As_String (Field_Value (Packet, "function")) = Function_Name then
            declare
               Packet_Kind : constant String := As_String (Field_Value (Packet, "kind"));
               Payload : constant Value := Field_Value (Packet, "value");
            begin
               if Packet_Kind = "aggregate" and then Is_Array then
                  for J in 1 .. Length (Payload) loop Owned.Append (Element (Payload, J)); end loop;
               elsif Packet_Kind = "row" or else Packet_Kind = "aggregate" then
                  Owned.Append (Payload);
               end if;
            end;
         end if;
      end;
   end loop;
   declare Result : Value_List (1 .. Natural (Owned.Length)); begin
      for I in Result'Range loop Result (I) := Owned (I); end loop;
      return Result;
   end;
end Receipts;
function Number (Item : Value) return Number_Field is
begin
   if Kind (Item) = Null_Value then return (State => Null_Field); end if;
   return (State => Present_Field, Value => Long_Float'Value (As_Number (Item)));
end Number;
function Item_Probabilities (Row : Value; Recognition : Boolean) return Probability_Lists.Vector is
   Result : Probability_Lists.Vector;
   Payload : constant Value := Field_Value (Row, "value");
begin
   if not Recognition then
      for I in 1 .. Length (Payload) loop
         Result.Append (Number (Field_Value (Element (Payload, I), "probability")));
      end loop;
   elsif Has_Field (Payload, "proposals") then
      declare Proposals : constant Value := Field_Value (Payload, "proposals"); begin
         for I in 1 .. Length (Proposals) loop
            Result.Append (Number (Field_Value (Element (Proposals, I), "probability")));
         end loop;
      end;
   else
      declare
         Entities : constant Value := Field_Value (Payload, "entities");
         Proposals : constant Value := Field_Value (Field_Value (Row, "answer"), "proposals");
      begin
         for I in 1 .. Length (Entities) loop
            declare
               Entity : constant Value := Element (Entities, I);
               Matched : Boolean := False;
            begin
               for J in 1 .. Length (Proposals) loop
                  declare
                     Proposal : constant Value := Element (Proposals, J);
                     Selected : constant Value := Field_Value (Proposal, "selected");
                  begin
                     if As_Boolean (Field_Value (Proposal, "kept")) and then Kind (Selected) /= Null_Value
                       and then Field_Value (Selected, "start") = Field_Value (Entity, "start")
                       and then Field_Value (Selected, "end") = Field_Value (Entity, "end")
                       and then Field_Value (Proposal, "kind") = Field_Value (Entity, "kind") then
                        if Matched then raise Answer_Cardinality_Error with "duplicate selected span"; end if;
                        Result.Append (Number (Field_Value (Proposal, "span_probability")));
                        Matched := True;
                     end if;
                  end;
               end loop;
               if not Matched then raise Answer_Cardinality_Error with "missing selected span"; end if;
            end;
         end loop;
      end;
   end if;
   return Result;
end Item_Probabilities;

function Decide (Call : Completion) return Decide_Result is
Rows : constant Value_List := Receipts (Call, "decide", True);
Result : Decide_Result (Rows'Length);
begin
if Rows'Length /= 1 then raise Answer_Cardinality_Error with "expected one result"; end if;
Result.Facts := Call_Facts (Call);
for I in Rows'Range loop
Result.Results (I) := (Data => Rows (I));
Result.Value := Field_Value (Rows (I), "value");
Result.Probability := (if Resolved_Decide (Call, Rows (I), I) then Number (Field_Value (Field_Value (Rows (I), "answer"), "probability")) else (State => Null_Field));
end loop;
return Result;
end Decide;
function Decide (Call : Completion) return Decide_List_Result is
Rows : constant Value_List := Receipts (Call, "decide", True);
Result : Decide_List_Result (Rows'Length, Rows'Length);
begin
Result.Facts := Call_Facts (Call);
for I in Rows'Range loop
Result.Results (I) := (Data => Rows (I));
Result.Value (I) := Field_Value (Rows (I), "value");
Result.Probability (I) := (if Resolved_Decide (Call, Rows (I), I) then Number (Field_Value (Field_Value (Rows (I), "answer"), "probability")) else (State => Null_Field));
end loop;
return Result;
end Decide;
function Choose (Call : Completion) return Choose_Result is
Rows : constant Value_List := Receipts (Call, "choose", True);
Result : Choose_Result (Rows'Length);
begin
if Rows'Length /= 1 then raise Answer_Cardinality_Error with "expected one result"; end if;
Result.Facts := Call_Facts (Call);
for I in Rows'Range loop
Result.Results (I) := (Data => Rows (I));
Result.Value := Read_Choice (Field_Value (Rows (I), "value"));
Result.Probability := (if Resolved_Choose (Call, Rows (I), I) then Number (Field_Value (Field_Value (Field_Value (Rows (I), "answer"), "probabilities"), As_String (Field_Value (Field_Value (Rows (I), "answer"), "pick")))) else (State => Null_Field));
end loop;
return Result;
end Choose;
function Choose (Call : Completion) return Choose_List_Result is
Rows : constant Value_List := Receipts (Call, "choose", True);
Result : Choose_List_Result (Rows'Length, Rows'Length);
begin
Result.Facts := Call_Facts (Call);
for I in Rows'Range loop
Result.Results (I) := (Data => Rows (I));
Result.Value (I) := Read_Choice (Field_Value (Rows (I), "value"));
Result.Probability (I) := (if Resolved_Choose (Call, Rows (I), I) then Number (Field_Value (Field_Value (Field_Value (Rows (I), "answer"), "probabilities"), As_String (Field_Value (Field_Value (Rows (I), "answer"), "pick")))) else (State => Null_Field));
end loop;
return Result;
end Choose;
function Tag (Call : Completion) return Tag_Result is
Rows : constant Value_List := Receipts (Call, "tag", True);
Result : Tag_Result (Value_Count => (if Rows'Length = 1 then Length (Field_Value (Rows (1), "value")) else 0), Record_Count => Rows'Length);
begin
if Rows'Length /= 1 then raise Answer_Cardinality_Error with "expected one result"; end if;
Result.Facts := Call_Facts (Call);
for I in Rows'Range loop
Result.Results (I) := (Data => Rows (I));
Result.Value := Texts (Field_Value (Rows (I), "value"));
Result.Probability := (State => Null_Field);
end loop;
return Result;
end Tag;
function Tag (Call : Completion) return Tag_List_Result is
Rows : constant Value_List := Receipts (Call, "tag", True);
Result : Tag_List_Result (Rows'Length, Rows'Length);
begin
Result.Facts := Call_Facts (Call);
for I in Rows'Range loop
Result.Results (I) := (Data => Rows (I));
Result.Value.Append (Texts (Field_Value (Rows (I), "value")));
Result.Probability := (State => Null_Field);
end loop;
return Result;
end Tag;
function Score (Call : Completion) return Score_Result is
Rows : constant Value_List := Receipts (Call, "score", True);
Result : Score_Result (Rows'Length);
begin
if Rows'Length /= 1 then raise Answer_Cardinality_Error with "expected one result"; end if;
Result.Facts := Call_Facts (Call);
for I in Rows'Range loop
Result.Results (I) := (Data => Rows (I));
Result.Value := Long_Float'Value (As_Number (Field_Value (Rows (I), "value")));
Result.Probability := (State => Null_Field);
end loop;
return Result;
end Score;
function Score (Call : Completion) return Score_List_Result is
Rows : constant Value_List := Receipts (Call, "score", True);
Result : Score_List_Result (Rows'Length, Rows'Length);
begin
Result.Facts := Call_Facts (Call);
for I in Rows'Range loop
Result.Results (I) := (Data => Rows (I));
Result.Value (I) := Long_Float'Value (As_Number (Field_Value (Rows (I), "value")));
Result.Probability := (State => Null_Field);
end loop;
return Result;
end Score;
function Filter (Call : Completion) return Filter_Result is
Rows : constant Value_List := Receipts (Call, "filter", True);
Result : Filter_Result (Rows'Length, Rows'Length);
begin
Result.Facts := Call_Facts (Call);
for I in Rows'Range loop
Result.Results (I) := (Data => Rows (I));
Result.Value (I) := Field_Value (Rows (I), "input");
Result.Probability (I) := Number (Field_Value (Field_Value (Rows (I), "answer"), "probability"));
end loop;
return Result;
end Filter;
function Rank (Call : Completion) return Rank_Result is
Rows : constant Value_List := Receipts (Call, "rank", True);
Result : Rank_Result (Rows'Length, Rows'Length);
begin
Result.Facts := Call_Facts (Call);
for I in Rows'Range loop
Result.Results (I) := (Data => Rows (I));
Result.Value (I) := Interfaces.Unsigned_64'Value (As_Number (Field_Value (Rows (I), "index")));
Result.Probability (I) := Number (Field_Value (Field_Value (Rows (I), "answer"), "probability"));
end loop;
return Result;
end Rank;
function Find (Call : Completion) return Find_Result is
Rows : constant Value_List := Receipts (Call, "find", False);
Result : Find_Result (Rows'Length);
begin
if Rows'Length /= 1 then raise Answer_Cardinality_Error with "expected one result"; end if;
Result.Facts := Call_Facts (Call);
for I in Rows'Range loop
Result.Results (I) := (Data => Rows (I));
Result.Value := Field_Value (Rows (I), "value");
Result.Probability := (if Kind (Field_Value (Rows (I), "index")) /= Null_Value then Number (Field_Value (Field_Value (Field_Value (Rows (I), "answer"), "probabilities"), As_String (Field_Value (Field_Value (Rows (I), "answer"), "pick")))) else (State => Null_Field));
end loop;
return Result;
end Find;
function Annotate (Call : Completion) return Annotate_Result is
Rows : constant Value_List := Receipts (Call, "annotate", True);
Result : Annotate_Result (Rows'Length);
begin
if Rows'Length /= 1 then raise Answer_Cardinality_Error with "expected one result"; end if;
Result.Facts := Call_Facts (Call);
for I in Rows'Range loop
Result.Results (I) := (Data => Rows (I));
Result.Value := Field_Value (Rows (I), "value");
Result.Probability := (State => Null_Field);
end loop;
return Result;
end Annotate;
function Annotate (Call : Completion) return Annotate_List_Result is
Rows : constant Value_List := Receipts (Call, "annotate", True);
Result : Annotate_List_Result (Rows'Length, Rows'Length);
begin
Result.Facts := Call_Facts (Call);
for I in Rows'Range loop
Result.Results (I) := (Data => Rows (I));
Result.Value (I) := Field_Value (Rows (I), "value");
Result.Probability := (State => Null_Field);
end loop;
return Result;
end Annotate;
function Recognize (Call : Completion) return Recognize_Result is
Rows : constant Value_List := Receipts (Call, "recognize", True);
Result : Recognize_Result (Rows'Length);
begin
if Rows'Length /= 1 then raise Answer_Cardinality_Error with "expected one result"; end if;
Result.Facts := Call_Facts (Call);
for I in Rows'Range loop
Result.Results (I) := (Data => Rows (I));
Result.Value := Field_Value (Rows (I), "value");
Result.Probability := Item_Probabilities (Rows (I), True);
end loop;
return Result;
end Recognize;
function Recognize (Call : Completion) return Recognize_List_Result is
Rows : constant Value_List := Receipts (Call, "recognize", True);
Result : Recognize_List_Result (Rows'Length, Rows'Length);
begin
Result.Facts := Call_Facts (Call);
for I in Rows'Range loop
Result.Results (I) := (Data => Rows (I));
Result.Value (I) := Field_Value (Rows (I), "value");
Result.Probability.Append (Item_Probabilities (Rows (I), True));
end loop;
return Result;
end Recognize;
function Relate (Call : Completion) return Relate_Result is
Rows : constant Value_List := Receipts (Call, "relate", False);
Result : Relate_Result (Rows'Length);
begin
if Rows'Length /= 1 then raise Answer_Cardinality_Error with "expected one result"; end if;
Result.Facts := Call_Facts (Call);
for I in Rows'Range loop
Result.Results (I) := (Data => Rows (I));
Result.Value := Field_Value (Rows (I), "value");
Result.Probability := Item_Probabilities (Rows (I), False);
end loop;
return Result;
end Relate;
end Thinkthen.Answers;
