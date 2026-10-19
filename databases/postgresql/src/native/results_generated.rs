// Generated from the canonical Rust result schema; do not edit.
use pgrx::prelude::*;
use serde_json::Value;
pgrx::extension_sql!(
    r#"
CREATE TYPE thinkthen_named_probability AS ("name" text, "value" double precision);
CREATE TYPE thinkthen_answer AS ("confidence" double precision, "kind" text, "level" text, "pick" text, "probabilities" thinkthen_named_probability[], "probability" double precision);
CREATE TYPE thinkthen_image AS ("base64" text, "height" bigint, "media" text, "width" bigint);
CREATE TYPE thinkthen_attempt AS ("ordinal" numeric, "outcome" text, "request_id" text, "request_sha256" text, "sdk_request_id" text, "server_ms" numeric, "status" bigint, "wall_ms" numeric);
CREATE TYPE thinkthen_batch_setting AS ("integer_value" numeric, "string_value" text);
CREATE TYPE thinkthen_batch_warning AS ("running" thinkthen_batch_setting, "tuned_for" thinkthen_batch_setting);
CREATE TYPE thinkthen_observation AS ("failure_id" text, "observation_id" text);
CREATE TYPE thinkthen_profile_warning AS ("running" text, "tuned_for" text);
CREATE TYPE thinkthen_question_source AS ("answered_by" text, "batch_size" bigint, "origin" text);
CREATE TYPE thinkthen_token_usage AS ("input_tokens" numeric, "output_tokens" numeric);
CREATE TYPE thinkthen_metadata AS ("answered_by" text, "attempts" thinkthen_attempt[], "batch_setting" thinkthen_batch_setting, "batch_warning" thinkthen_batch_warning, "cached" boolean, "context_sha256" text, "failed_questions" numeric, "model" text, "observations" thinkthen_observation[], "origin" text, "profile_warning" thinkthen_profile_warning, "question_sha256" text, "question_sources" thinkthen_question_source[], "questions_sha256" text, "requests" text[], "requests_sent" numeric, "tool" text, "url" text, "usage" thinkthen_token_usage);
CREATE TYPE thinkthen_position AS ("file" text, "first" numeric, "images" text[], "last" numeric);
CREATE TYPE thinkthen_string_declaration AS ("type" text);
CREATE TYPE thinkthen_property_declaration AS ("items" thinkthen_string_declaration, "type" text);
CREATE TYPE thinkthen_declared_property AS ("name" text, "value" thinkthen_property_declaration);
CREATE TYPE thinkthen_input_declaration AS ("properties" thinkthen_declared_property[], "required" text[], "type" text);
CREATE TYPE thinkthen_label AS ("description" jsonb, "description_present" boolean, "name" text);
CREATE TYPE thinkthen_question AS ("batch" thinkthen_batch_setting, "context_schema" thinkthen_input_declaration, "false" jsonb, "false_present" boolean, "item_schema" thinkthen_input_declaration, "label_details" thinkthen_label[], "labels" text[], "levels" text[], "model" text, "name" text, "on" text[], "options" text[], "profile" text, "text" jsonb, "text_present" boolean, "true" jsonb, "true_present" boolean, "verb" text, "wording_version" bigint);
CREATE TYPE thinkthen_source_location AS ("file" text, "first_line" numeric, "last_line" numeric);
CREATE TYPE thinkthen_threshold AS ("number_value" double precision, "string_value" text);
CREATE TYPE thinkthen_decision AS ("answer" thinkthen_answer, "answer_id" text, "file" text, "first_line" numeric, "images" thinkthen_image[], "index" numeric, "input" jsonb, "input_present" boolean, "input_file" text, "last_line" numeric, "meta" thinkthen_metadata, "position" thinkthen_position, "question" thinkthen_question, "question_name" text, "schema" text, "source" thinkthen_source_location, "threshold" thinkthen_threshold, "value" jsonb, "value_present" boolean);
CREATE TYPE thinkthen_choice AS ("answer" thinkthen_answer, "answer_id" text, "file" text, "first_line" numeric, "images" thinkthen_image[], "index" numeric, "input" jsonb, "input_present" boolean, "input_file" text, "last_line" numeric, "meta" thinkthen_metadata, "position" thinkthen_position, "question" thinkthen_question, "question_name" text, "schema" text, "source" thinkthen_source_location, "threshold" thinkthen_threshold, "value" text);
CREATE TYPE thinkthen_tags AS ("answer" thinkthen_answer, "answer_id" text, "file" text, "first_line" numeric, "images" thinkthen_image[], "index" numeric, "input" jsonb, "input_present" boolean, "input_file" text, "last_line" numeric, "meta" thinkthen_metadata, "position" thinkthen_position, "question" thinkthen_question, "question_name" text, "schema" text, "source" thinkthen_source_location, "threshold" thinkthen_threshold, "value" text[]);
CREATE TYPE thinkthen_score AS ("answer" thinkthen_answer, "answer_id" text, "file" text, "first_line" numeric, "images" thinkthen_image[], "index" numeric, "input" jsonb, "input_present" boolean, "input_file" text, "last_line" numeric, "meta" thinkthen_metadata, "position" thinkthen_position, "question" thinkthen_question, "question_name" text, "schema" text, "source" thinkthen_source_location, "threshold" jsonb, "threshold_present" boolean, "value" double precision);
CREATE TYPE thinkthen_filtered AS ("answer" thinkthen_answer, "answer_id" text, "file" text, "first_line" numeric, "images" thinkthen_image[], "index" numeric, "input" jsonb, "input_present" boolean, "input_file" text, "last_line" numeric, "meta" thinkthen_metadata, "position" thinkthen_position, "question" thinkthen_question, "question_name" text, "schema" text, "source" thinkthen_source_location, "threshold" thinkthen_threshold, "value" boolean);
CREATE TYPE thinkthen_ranked_member_result AS ("answer" thinkthen_answer, "answer_id" text, "images" thinkthen_image[], "meta" thinkthen_metadata, "question" thinkthen_question, "schema" text, "source" thinkthen_source_location, "threshold" jsonb, "threshold_present" boolean, "value" numeric);
CREATE TYPE thinkthen_named_ranked_member AS ("name" text, "result" thinkthen_ranked_member_result);
CREATE TYPE thinkthen_ranked AS ("answer" thinkthen_answer, "answer_id" text, "file" text, "first_line" numeric, "images" thinkthen_image[], "index" numeric, "input" jsonb, "input_present" boolean, "input_file" text, "last_line" numeric, "members" thinkthen_named_ranked_member[], "meta" thinkthen_metadata, "position" thinkthen_position, "question" thinkthen_question, "question_name" text, "schema" text, "source" thinkthen_source_location, "threshold" jsonb, "threshold_present" boolean, "value" numeric);
CREATE TYPE thinkthen_failure AS ("cause" text, "kind" text);
CREATE TYPE thinkthen_answer_value AS ("array_value" text[], "boolean_value" boolean, "number_value" double precision, "string_value" text);
CREATE TYPE thinkthen_annotation_answer AS ("answer" thinkthen_answer, "answer_id" text, "failure" thinkthen_failure, "failure_id" text, "observations" thinkthen_observation[], "question" thinkthen_question, "question_sources" thinkthen_question_source[], "request" text, "threshold" thinkthen_threshold, "usage" thinkthen_token_usage, "value" thinkthen_answer_value);
CREATE TYPE thinkthen_named_answer AS ("name" text, "value" thinkthen_annotation_answer);
CREATE TYPE thinkthen_failed_value AS ("failed" thinkthen_failure);
CREATE TYPE thinkthen_field_value AS ("array_value" text[], "boolean_value" boolean, "number_value" double precision, "object_value" thinkthen_failed_value, "string_value" text);
CREATE TYPE thinkthen_named_field_value AS ("name" text, "value" thinkthen_field_value);
CREATE TYPE thinkthen_annotation AS ("answer_id" text, "answers" thinkthen_named_answer[], "file" text, "first_line" numeric, "index" numeric, "input" jsonb, "input_present" boolean, "last_line" numeric, "meta" thinkthen_metadata, "position" thinkthen_position, "schema" text, "source" thinkthen_source_location, "value" thinkthen_named_field_value[]);
CREATE TYPE thinkthen_entity_probabilities AS ("edges" thinkthen_named_probability[], "end" numeric, "kinds" thinkthen_named_probability[], "start" numeric);
CREATE TYPE thinkthen_place AS ("end" numeric, "start" numeric);
CREATE TYPE thinkthen_relation_probabilities AS ("probability" double precision, "relation" text, "source" thinkthen_place, "target" thinkthen_place);
CREATE TYPE thinkthen_token_probabilities AS ("end" numeric, "start" numeric, "tags" thinkthen_named_probability[]);
CREATE TYPE thinkthen_selected_span AS ("end" numeric, "start" numeric);
CREATE TYPE thinkthen_boundary_reading AS ("end" numeric, "file" text, "first_line" numeric, "kept" boolean, "kind" text, "last_line" numeric, "length" numeric, "probability" double precision, "selected" thinkthen_selected_span, "span_probability" double precision, "start" numeric, "strength" double precision, "text" text);
CREATE TYPE thinkthen_recognition_probabilities AS ("names" thinkthen_entity_probabilities[], "pairs" thinkthen_relation_probabilities[], "pieces" thinkthen_token_probabilities[], "proposals" thinkthen_boundary_reading[]);
CREATE TYPE thinkthen_kind_description AS ("name" text, "value" jsonb, "value_present" boolean);
CREATE TYPE thinkthen_relation_rule AS ("either" boolean, "name" text, "reads" text, "single" boolean, "source" text, "target" text);
CREATE TYPE thinkthen_recognition_stage_context AS ("boundary" text, "kind_edge" text, "relation" text);
CREATE TYPE thinkthen_recognition_question AS ("batch" thinkthen_batch_setting, "context_schema" thinkthen_input_declaration, "entity_definition" jsonb, "entity_definition_present" boolean, "instructions" jsonb, "instructions_present" boolean, "item_schema" thinkthen_input_declaration, "kinds" thinkthen_kind_description[], "label_details" thinkthen_label[], "mode" text, "model" text, "name" text, "on" text[], "profile" text, "relation_threshold" thinkthen_threshold, "relations" thinkthen_relation_rule[], "snippet_pieces" bigint, "stage_context" thinkthen_recognition_stage_context, "threshold" thinkthen_threshold, "verb" text, "wording_version" bigint);
CREATE TYPE thinkthen_entity AS ("end" numeric, "file" text, "first_line" numeric, "kind" text, "last_line" numeric, "length" numeric, "start" numeric, "strength" double precision, "text" text);
CREATE TYPE thinkthen_boundary_proposal AS ("end" numeric, "file" text, "first_line" numeric, "last_line" numeric, "length" numeric, "probability" double precision, "start" numeric, "text" text);
CREATE TYPE thinkthen_entity_edge AS ("either" boolean, "probability" double precision, "relation" text, "source" thinkthen_entity, "target" thinkthen_entity);
CREATE TYPE thinkthen_recognized_value AS ("entities" thinkthen_entity[], "mode" text, "proposals" thinkthen_boundary_proposal[], "relations" thinkthen_entity_edge[]);
CREATE TYPE thinkthen_recognized AS ("answer" thinkthen_recognition_probabilities, "answer_id" text, "file" text, "first_line" numeric, "index" numeric, "input" jsonb, "input_present" boolean, "last_line" numeric, "meta" thinkthen_metadata, "position" thinkthen_position, "question" thinkthen_recognition_question, "schema" text, "source" thinkthen_source_location, "value" thinkthen_recognized_value);
CREATE TYPE thinkthen_candidate_probability AS ("name" text, "value" double precision);
CREATE TYPE thinkthen_find_answer AS ("confidence" double precision, "kind" text, "pick" text, "probabilities" thinkthen_candidate_probability[]);
CREATE TYPE thinkthen_find_candidate AS ("index" numeric, "input" jsonb, "input_present" boolean, "probability" double precision, "source" thinkthen_source_location);
CREATE TYPE thinkthen_find_question AS ("batch" thinkthen_batch_setting, "context_schema" thinkthen_input_declaration, "item_schema" thinkthen_input_declaration, "label_details" thinkthen_label[], "model" text, "name" text, "none" boolean, "on" text[], "profile" text, "text" jsonb, "text_present" boolean, "verb" text, "wording_version" bigint);
CREATE TYPE thinkthen_found AS ("answer" thinkthen_find_answer, "answer_id" text, "candidates" thinkthen_find_candidate[], "file" text, "first_line" numeric, "index" numeric, "last_line" numeric, "meta" thinkthen_metadata, "position" thinkthen_position, "question" thinkthen_find_question, "schema" text, "threshold" jsonb, "threshold_present" boolean, "value" jsonb, "value_present" boolean);
CREATE TYPE thinkthen_related_entity AS ("kind" text, "name" text);
CREATE TYPE thinkthen_relation_endpoint AS ("kind" text, "name" text);
CREATE TYPE thinkthen_relation_reading AS ("accepted" boolean, "answer" thinkthen_answer, "answer_id" text, "direction" text, "failure" thinkthen_failure, "failure_id" text, "method" text, "observations" thinkthen_observation[], "probability" double precision, "question" thinkthen_question, "question_sources" thinkthen_question_source[], "reads" text, "relation" text, "request" text, "source" thinkthen_related_entity, "target" thinkthen_relation_endpoint, "threshold" thinkthen_threshold, "usage" thinkthen_token_usage);
CREATE TYPE thinkthen_relation_answers AS ("questions" thinkthen_relation_reading[]);
CREATE TYPE thinkthen_indexed_source AS ("index" numeric, "source" thinkthen_source_location);
CREATE TYPE thinkthen_relation_fields AS ("kind" text, "name" text);
CREATE TYPE thinkthen_relation_question AS ("batch" thinkthen_batch_setting, "context_schema" thinkthen_input_declaration, "fields" thinkthen_relation_fields, "item_schema" thinkthen_input_declaration, "label_details" thinkthen_label[], "model" text, "name" text, "on" text[], "profile" text, "relations" thinkthen_relation_rule[], "threshold" thinkthen_threshold, "verb" text, "wording_version" bigint);
CREATE TYPE thinkthen_located_endpoint AS ("file" text, "first_line" numeric, "kind" text, "last_line" numeric, "name" text, "ordinal" numeric, "record" jsonb, "record_present" boolean);
CREATE TYPE thinkthen_located_edge AS ("either" boolean, "probability" double precision, "relation" text, "source" thinkthen_located_endpoint, "target" thinkthen_located_endpoint);
CREATE TYPE thinkthen_related AS ("answer" thinkthen_relation_answers, "answer_id" text, "file" text, "first_line" numeric, "index" numeric, "input" jsonb, "input_present" boolean, "input_sources" thinkthen_indexed_source[], "last_line" numeric, "meta" thinkthen_metadata, "position" thinkthen_position, "question" thinkthen_relation_question, "schema" text, "value" thinkthen_located_edge[]);
CREATE TYPE thinkthen_usage_persistence AS ("advice" text, "observed_at" text, "state" text);
CREATE TYPE thinkthen_facts AS ("attempts" thinkthen_attempt[], "cache_answers" numeric, "call_id" text, "estimated_cost_usd" text, "held_model_mismatch" boolean, "input_tokens" numeric, "largest_request_bytes" numeric, "largest_request_estimated_input_tokens" numeric, "model" text, "output_tokens" numeric, "records" numeric, "requests_sent" numeric, "seconds" double precision, "token_estimate_method" text, "usage_persistence" thinkthen_usage_persistence);
CREATE TYPE thinkthen_token_band AS ("lower" numeric, "upper" numeric);
CREATE TYPE thinkthen_plan AS ("estimated_bytes" numeric, "estimated_input_tokens" thinkthen_token_band, "first_body_utf8" text, "largest_request_bytes" numeric, "largest_request_estimated_input_tokens" numeric, "records" numeric, "requests" numeric, "token_estimate_method" text, "upper_bound" boolean);
CREATE TYPE thinkthen_input_budget_denial AS ("kind" text, "last_status" bigint, "limit" numeric);
CREATE TYPE thinkthen_send_budget_denial AS ("kind" text, "last_status" bigint);
CREATE TYPE thinkthen_stopped AS ("at" numeric, "cause" text, "retryable" boolean, "status" bigint);
CREATE TYPE thinkthen_error AS ("cause" text, "estimated_input_denial" thinkthen_input_budget_denial, "kind" text, "message" text, "retryable" boolean, "send_budget_denial" thinkthen_send_budget_denial, "stopped" thinkthen_stopped);
CREATE TYPE thinkthen_observed_probability AS ("name" text, "probability" double precision);
CREATE TYPE thinkthen_probability_values AS ("array_value" thinkthen_observed_probability[], "number_value" double precision);
CREATE TYPE thinkthen_probability_distribution AS ("kind" text, "value" thinkthen_probability_values);
CREATE TYPE thinkthen_reported_usage AS ("input_tokens" numeric, "output_tokens" numeric);
CREATE TYPE thinkthen_question_detail AS ("answer_id" text, "cached" boolean, "confidence" double precision, "failed_questions" numeric, "failure" thinkthen_failure, "failure_id" text, "input" jsonb, "input_present" boolean, "input_source" thinkthen_source_location, "input_sources" thinkthen_indexed_source[], "inputs" jsonb[], "model" text, "observations" thinkthen_observation[], "probabilities" thinkthen_probability_distribution, "question" thinkthen_question, "question_sha256" text, "question_sources" thinkthen_question_source[], "raw_pick" text, "reported_usage" thinkthen_reported_usage, "requests" text[], "requests_sent" numeric, "threshold" thinkthen_threshold, "url" text, "usage" thinkthen_token_usage, "value" thinkthen_answer_value);
CREATE TYPE thinkthen_observed_input AS ("images" thinkthen_image[], "input" jsonb, "input_present" boolean, "source" thinkthen_source_location);
CREATE TYPE thinkthen_event AS ("detail" thinkthen_question_detail, "index" numeric, "inputs" thinkthen_observed_input[], "kind" text, "member" text, "position" numeric, "stage" text, "value" jsonb, "value_present" boolean);
"#,
    name = "native_generated_types"
);
#[expect(
    clippy::too_many_lines,
    clippy::cognitive_complexity,
    reason = "generated schema projections"
)]
pub(super) fn convert(
    kind: &str,
    value: &Value,
) -> Result<PgHeapTuple<'static, AllocatedByRust>, thinkthen::Error> {
    match kind {
        "thinkthen_named_probability" => {
            let mut tuple = super::output::tuple(kind);
            let held = value.get("name");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("name", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("value");
            let converted: Option<f64> = held.and_then(Value::as_f64);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("value", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_answer" => {
            let mut tuple = super::output::tuple(kind);
            let held = value.get("confidence");
            let converted: Option<f64> = held.and_then(Value::as_f64);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("confidence", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("kind");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("kind", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("level");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("level", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("pick");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("pick", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("probabilities");
            let converted: Option<Array<'static, PgHeapTuple<'static, AllocatedByRust>>> = held
                .and_then(Value::as_object)
                .map(|values| {
                    crate::ffi::composite_array(
                        "thinkthen_named_probability",
                        values
                            .iter()
                            .map(|(name, value)| {
                                let entry = serde_json::json!({"name": name, "value": value});
                                convert("thinkthen_named_probability", &entry).map(Some)
                            })
                            .collect::<Result<Vec<_>, thinkthen::Error>>()?,
                    )
                })
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("probabilities", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("probability");
            let converted: Option<f64> = held.and_then(Value::as_f64);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("probability", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_image" => {
            let mut tuple = super::output::tuple(kind);
            let held = value.get("base64");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("base64", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("height");
            let converted: Option<i64> = held.and_then(Value::as_i64);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("height", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("media");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("media", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("width");
            let converted: Option<i64> = held.and_then(Value::as_i64);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("width", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_attempt" => {
            let mut tuple = super::output::tuple(kind);
            let held = value.get("ordinal");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("ordinal", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("outcome");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("outcome", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("request_id");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("request_id", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("request_sha256");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("request_sha256", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("sdk_request_id");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("sdk_request_id", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("server_ms");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("server_ms", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("status");
            let converted: Option<i64> = held.and_then(Value::as_i64);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("status", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("wall_ms");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("wall_ms", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_batch_setting" => {
            let mut tuple = super::output::tuple(kind);
            let held = Some(value);
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("integer_value", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = Some(value);
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("string_value", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_batch_warning" => {
            let mut tuple = super::output::tuple(kind);
            let held = value.get("running");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_batch_setting", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("running", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("tuned_for");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_batch_setting", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("tuned_for", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_observation" => {
            let mut tuple = super::output::tuple(kind);
            let held = value.get("failure_id");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("failure_id", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("observation_id");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("observation_id", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_profile_warning" => {
            let mut tuple = super::output::tuple(kind);
            let held = value.get("running");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("running", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("tuned_for");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("tuned_for", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_question_source" => {
            let mut tuple = super::output::tuple(kind);
            let held = value.get("answered_by");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("answered_by", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("batch_size");
            let converted: Option<i64> = held.and_then(Value::as_i64);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("batch_size", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("origin");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("origin", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_token_usage" => {
            let mut tuple = super::output::tuple(kind);
            let held = value.get("input_tokens");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("input_tokens", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("output_tokens");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("output_tokens", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_metadata" => {
            let mut tuple = super::output::tuple(kind);
            let held = value.get("answered_by");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("answered_by", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("attempts");
            let converted: Option<Array<'static, PgHeapTuple<'static, AllocatedByRust>>> = held
                .and_then(Value::as_array)
                .map(|values| {
                    crate::ffi::composite_array(
                        "thinkthen_attempt",
                        values
                            .iter()
                            .map(|value| {
                                Some(value)
                                    .filter(|value| !value.is_null())
                                    .map(|value| convert("thinkthen_attempt", value))
                                    .transpose()
                            })
                            .collect::<Result<Vec<_>, thinkthen::Error>>()?,
                    )
                })
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("attempts", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("batch_setting");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_batch_setting", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("batch_setting", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("batch_warning");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_batch_warning", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("batch_warning", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("cached");
            let converted: Option<bool> = held.and_then(Value::as_bool);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("cached", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("context_sha256");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("context_sha256", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("failed_questions");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("failed_questions", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("model");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("model", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("observations");
            let converted: Option<Array<'static, PgHeapTuple<'static, AllocatedByRust>>> = held
                .and_then(Value::as_array)
                .map(|values| {
                    crate::ffi::composite_array(
                        "thinkthen_observation",
                        values
                            .iter()
                            .map(|value| {
                                Some(value)
                                    .filter(|value| !value.is_null())
                                    .map(|value| convert("thinkthen_observation", value))
                                    .transpose()
                            })
                            .collect::<Result<Vec<_>, thinkthen::Error>>()?,
                    )
                })
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("observations", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("origin");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("origin", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("profile_warning");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_profile_warning", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("profile_warning", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("question_sha256");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("question_sha256", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("question_sources");
            let converted: Option<Array<'static, PgHeapTuple<'static, AllocatedByRust>>> = held
                .and_then(Value::as_array)
                .map(|values| {
                    crate::ffi::composite_array(
                        "thinkthen_question_source",
                        values
                            .iter()
                            .map(|value| {
                                Some(value)
                                    .filter(|value| !value.is_null())
                                    .map(|value| convert("thinkthen_question_source", value))
                                    .transpose()
                            })
                            .collect::<Result<Vec<_>, thinkthen::Error>>()?,
                    )
                })
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("question_sources", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("questions_sha256");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("questions_sha256", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("requests");
            let converted: Option<Vec<Option<String>>> = held
                .and_then(Value::as_array)
                .map(|values| {
                    values
                        .iter()
                        .map(|value| Ok(Some(value).and_then(Value::as_str).map(str::to_owned)))
                        .collect::<Result<Vec<_>, thinkthen::Error>>()
                })
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("requests", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("requests_sent");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("requests_sent", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("tool");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("tool", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("url");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("url", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("usage");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_token_usage", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("usage", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_position" => {
            let mut tuple = super::output::tuple(kind);
            let held = value.get("file");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("file", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("first");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("first", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("images");
            let converted: Option<Vec<Option<String>>> = held
                .and_then(Value::as_array)
                .map(|values| {
                    values
                        .iter()
                        .map(|value| Ok(Some(value).and_then(Value::as_str).map(str::to_owned)))
                        .collect::<Result<Vec<_>, thinkthen::Error>>()
                })
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("images", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("last");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("last", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_string_declaration" => {
            let mut tuple = super::output::tuple(kind);
            let held = value.get("type");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("type", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_property_declaration" => {
            let mut tuple = super::output::tuple(kind);
            let held = value.get("items");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_string_declaration", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("items", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("type");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("type", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_declared_property" => {
            let mut tuple = super::output::tuple(kind);
            let held = value.get("name");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("name", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("value");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_property_declaration", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("value", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_input_declaration" => {
            let mut tuple = super::output::tuple(kind);
            let held = value.get("properties");
            let converted: Option<Array<'static, PgHeapTuple<'static, AllocatedByRust>>> = held
                .and_then(Value::as_object)
                .map(|values| {
                    crate::ffi::composite_array(
                        "thinkthen_declared_property",
                        values
                            .iter()
                            .map(|(name, value)| {
                                let entry = serde_json::json!({"name": name, "value": value});
                                convert("thinkthen_declared_property", &entry).map(Some)
                            })
                            .collect::<Result<Vec<_>, thinkthen::Error>>()?,
                    )
                })
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("properties", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("required");
            let converted: Option<Vec<Option<String>>> = held
                .and_then(Value::as_array)
                .map(|values| {
                    values
                        .iter()
                        .map(|value| Ok(Some(value).and_then(Value::as_str).map(str::to_owned)))
                        .collect::<Result<Vec<_>, thinkthen::Error>>()
                })
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("required", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("type");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("type", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_label" => {
            let mut tuple = super::output::tuple(kind);
            let held = value.get("description");
            tuple
                .set_by_name("description_present", held.is_some())
                .map_err(|_| crate::call::defect("generated result presence is unavailable"))?;
            let converted: Option<pgrx::datum::JsonB> =
                held.map(|value| pgrx::datum::JsonB(value.clone()));
            if let Some(converted) = converted {
                tuple
                    .set_by_name("description", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("name");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("name", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_question" => {
            let mut tuple = super::output::tuple(kind);
            let held = value.get("batch");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_batch_setting", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("batch", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("context_schema");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_input_declaration", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("context_schema", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("false");
            tuple
                .set_by_name("false_present", held.is_some())
                .map_err(|_| crate::call::defect("generated result presence is unavailable"))?;
            let converted: Option<pgrx::datum::JsonB> =
                held.map(|value| pgrx::datum::JsonB(value.clone()));
            if let Some(converted) = converted {
                tuple
                    .set_by_name("false", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("item_schema");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_input_declaration", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("item_schema", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("label_details");
            let converted: Option<Array<'static, PgHeapTuple<'static, AllocatedByRust>>> = held
                .and_then(Value::as_array)
                .map(|values| {
                    crate::ffi::composite_array(
                        "thinkthen_label",
                        values
                            .iter()
                            .map(|value| {
                                Some(value)
                                    .filter(|value| !value.is_null())
                                    .map(|value| convert("thinkthen_label", value))
                                    .transpose()
                            })
                            .collect::<Result<Vec<_>, thinkthen::Error>>()?,
                    )
                })
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("label_details", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("labels");
            let converted: Option<Vec<Option<String>>> = held
                .and_then(Value::as_array)
                .map(|values| {
                    values
                        .iter()
                        .map(|value| Ok(Some(value).and_then(Value::as_str).map(str::to_owned)))
                        .collect::<Result<Vec<_>, thinkthen::Error>>()
                })
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("labels", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("levels");
            let converted: Option<Vec<Option<String>>> = held
                .and_then(Value::as_array)
                .map(|values| {
                    values
                        .iter()
                        .map(|value| Ok(Some(value).and_then(Value::as_str).map(str::to_owned)))
                        .collect::<Result<Vec<_>, thinkthen::Error>>()
                })
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("levels", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("model");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("model", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("name");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("name", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("on");
            let converted: Option<Vec<Option<String>>> = held
                .and_then(Value::as_array)
                .map(|values| {
                    values
                        .iter()
                        .map(|value| Ok(Some(value).and_then(Value::as_str).map(str::to_owned)))
                        .collect::<Result<Vec<_>, thinkthen::Error>>()
                })
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("on", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("options");
            let converted: Option<Vec<Option<String>>> = held
                .and_then(Value::as_array)
                .map(|values| {
                    values
                        .iter()
                        .map(|value| Ok(Some(value).and_then(Value::as_str).map(str::to_owned)))
                        .collect::<Result<Vec<_>, thinkthen::Error>>()
                })
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("options", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("profile");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("profile", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("text");
            tuple
                .set_by_name("text_present", held.is_some())
                .map_err(|_| crate::call::defect("generated result presence is unavailable"))?;
            let converted: Option<pgrx::datum::JsonB> =
                held.map(|value| pgrx::datum::JsonB(value.clone()));
            if let Some(converted) = converted {
                tuple
                    .set_by_name("text", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("true");
            tuple
                .set_by_name("true_present", held.is_some())
                .map_err(|_| crate::call::defect("generated result presence is unavailable"))?;
            let converted: Option<pgrx::datum::JsonB> =
                held.map(|value| pgrx::datum::JsonB(value.clone()));
            if let Some(converted) = converted {
                tuple
                    .set_by_name("true", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("verb");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("verb", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("wording_version");
            let converted: Option<i64> = held.and_then(Value::as_i64);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("wording_version", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_source_location" => {
            let mut tuple = super::output::tuple(kind);
            let held = value.get("file");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("file", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("first_line");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("first_line", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("last_line");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("last_line", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_threshold" => {
            let mut tuple = super::output::tuple(kind);
            let held = Some(value);
            let converted: Option<f64> = held.and_then(Value::as_f64);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("number_value", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = Some(value);
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("string_value", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_decision" => {
            let mut tuple = super::output::tuple(kind);
            let held = value.get("answer");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_answer", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("answer", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("answer_id");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("answer_id", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("file");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("file", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("first_line");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("first_line", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("images");
            let converted: Option<Array<'static, PgHeapTuple<'static, AllocatedByRust>>> = held
                .and_then(Value::as_array)
                .map(|values| {
                    crate::ffi::composite_array(
                        "thinkthen_image",
                        values
                            .iter()
                            .map(|value| {
                                Some(value)
                                    .filter(|value| !value.is_null())
                                    .map(|value| convert("thinkthen_image", value))
                                    .transpose()
                            })
                            .collect::<Result<Vec<_>, thinkthen::Error>>()?,
                    )
                })
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("images", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("index");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("index", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("input");
            tuple
                .set_by_name("input_present", held.is_some())
                .map_err(|_| crate::call::defect("generated result presence is unavailable"))?;
            let converted: Option<pgrx::datum::JsonB> =
                held.map(|value| pgrx::datum::JsonB(value.clone()));
            if let Some(converted) = converted {
                tuple
                    .set_by_name("input", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("input_file");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("input_file", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("last_line");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("last_line", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("meta");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_metadata", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("meta", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("position");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_position", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("position", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("question");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_question", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("question", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("question_name");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("question_name", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("schema");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("schema", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("source");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_source_location", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("source", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("threshold");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .map(|value| convert("thinkthen_threshold", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("threshold", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("value");
            tuple
                .set_by_name("value_present", held.is_some())
                .map_err(|_| crate::call::defect("generated result presence is unavailable"))?;
            let converted: Option<pgrx::datum::JsonB> =
                held.map(|value| pgrx::datum::JsonB(value.clone()));
            if let Some(converted) = converted {
                tuple
                    .set_by_name("value", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_choice" => {
            let mut tuple = super::output::tuple(kind);
            let held = value.get("answer");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_answer", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("answer", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("answer_id");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("answer_id", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("file");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("file", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("first_line");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("first_line", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("images");
            let converted: Option<Array<'static, PgHeapTuple<'static, AllocatedByRust>>> = held
                .and_then(Value::as_array)
                .map(|values| {
                    crate::ffi::composite_array(
                        "thinkthen_image",
                        values
                            .iter()
                            .map(|value| {
                                Some(value)
                                    .filter(|value| !value.is_null())
                                    .map(|value| convert("thinkthen_image", value))
                                    .transpose()
                            })
                            .collect::<Result<Vec<_>, thinkthen::Error>>()?,
                    )
                })
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("images", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("index");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("index", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("input");
            tuple
                .set_by_name("input_present", held.is_some())
                .map_err(|_| crate::call::defect("generated result presence is unavailable"))?;
            let converted: Option<pgrx::datum::JsonB> =
                held.map(|value| pgrx::datum::JsonB(value.clone()));
            if let Some(converted) = converted {
                tuple
                    .set_by_name("input", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("input_file");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("input_file", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("last_line");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("last_line", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("meta");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_metadata", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("meta", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("position");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_position", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("position", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("question");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_question", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("question", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("question_name");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("question_name", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("schema");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("schema", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("source");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_source_location", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("source", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("threshold");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .map(|value| convert("thinkthen_threshold", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("threshold", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("value");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("value", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_tags" => {
            let mut tuple = super::output::tuple(kind);
            let held = value.get("answer");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_answer", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("answer", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("answer_id");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("answer_id", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("file");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("file", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("first_line");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("first_line", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("images");
            let converted: Option<Array<'static, PgHeapTuple<'static, AllocatedByRust>>> = held
                .and_then(Value::as_array)
                .map(|values| {
                    crate::ffi::composite_array(
                        "thinkthen_image",
                        values
                            .iter()
                            .map(|value| {
                                Some(value)
                                    .filter(|value| !value.is_null())
                                    .map(|value| convert("thinkthen_image", value))
                                    .transpose()
                            })
                            .collect::<Result<Vec<_>, thinkthen::Error>>()?,
                    )
                })
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("images", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("index");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("index", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("input");
            tuple
                .set_by_name("input_present", held.is_some())
                .map_err(|_| crate::call::defect("generated result presence is unavailable"))?;
            let converted: Option<pgrx::datum::JsonB> =
                held.map(|value| pgrx::datum::JsonB(value.clone()));
            if let Some(converted) = converted {
                tuple
                    .set_by_name("input", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("input_file");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("input_file", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("last_line");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("last_line", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("meta");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_metadata", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("meta", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("position");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_position", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("position", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("question");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_question", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("question", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("question_name");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("question_name", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("schema");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("schema", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("source");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_source_location", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("source", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("threshold");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .map(|value| convert("thinkthen_threshold", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("threshold", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("value");
            let converted: Option<Vec<Option<String>>> = held
                .and_then(Value::as_array)
                .map(|values| {
                    values
                        .iter()
                        .map(|value| Ok(Some(value).and_then(Value::as_str).map(str::to_owned)))
                        .collect::<Result<Vec<_>, thinkthen::Error>>()
                })
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("value", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_score" => {
            let mut tuple = super::output::tuple(kind);
            let held = value.get("answer");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_answer", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("answer", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("answer_id");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("answer_id", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("file");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("file", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("first_line");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("first_line", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("images");
            let converted: Option<Array<'static, PgHeapTuple<'static, AllocatedByRust>>> = held
                .and_then(Value::as_array)
                .map(|values| {
                    crate::ffi::composite_array(
                        "thinkthen_image",
                        values
                            .iter()
                            .map(|value| {
                                Some(value)
                                    .filter(|value| !value.is_null())
                                    .map(|value| convert("thinkthen_image", value))
                                    .transpose()
                            })
                            .collect::<Result<Vec<_>, thinkthen::Error>>()?,
                    )
                })
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("images", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("index");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("index", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("input");
            tuple
                .set_by_name("input_present", held.is_some())
                .map_err(|_| crate::call::defect("generated result presence is unavailable"))?;
            let converted: Option<pgrx::datum::JsonB> =
                held.map(|value| pgrx::datum::JsonB(value.clone()));
            if let Some(converted) = converted {
                tuple
                    .set_by_name("input", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("input_file");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("input_file", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("last_line");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("last_line", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("meta");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_metadata", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("meta", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("position");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_position", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("position", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("question");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_question", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("question", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("question_name");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("question_name", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("schema");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("schema", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("source");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_source_location", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("source", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("threshold");
            tuple
                .set_by_name("threshold_present", held.is_some())
                .map_err(|_| crate::call::defect("generated result presence is unavailable"))?;
            let converted: Option<pgrx::datum::JsonB> =
                held.map(|value| pgrx::datum::JsonB(value.clone()));
            if let Some(converted) = converted {
                tuple
                    .set_by_name("threshold", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("value");
            let converted: Option<f64> = held.and_then(Value::as_f64);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("value", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_filtered" => {
            let mut tuple = super::output::tuple(kind);
            let held = value.get("answer");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_answer", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("answer", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("answer_id");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("answer_id", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("file");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("file", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("first_line");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("first_line", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("images");
            let converted: Option<Array<'static, PgHeapTuple<'static, AllocatedByRust>>> = held
                .and_then(Value::as_array)
                .map(|values| {
                    crate::ffi::composite_array(
                        "thinkthen_image",
                        values
                            .iter()
                            .map(|value| {
                                Some(value)
                                    .filter(|value| !value.is_null())
                                    .map(|value| convert("thinkthen_image", value))
                                    .transpose()
                            })
                            .collect::<Result<Vec<_>, thinkthen::Error>>()?,
                    )
                })
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("images", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("index");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("index", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("input");
            tuple
                .set_by_name("input_present", held.is_some())
                .map_err(|_| crate::call::defect("generated result presence is unavailable"))?;
            let converted: Option<pgrx::datum::JsonB> =
                held.map(|value| pgrx::datum::JsonB(value.clone()));
            if let Some(converted) = converted {
                tuple
                    .set_by_name("input", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("input_file");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("input_file", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("last_line");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("last_line", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("meta");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_metadata", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("meta", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("position");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_position", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("position", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("question");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_question", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("question", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("question_name");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("question_name", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("schema");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("schema", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("source");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_source_location", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("source", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("threshold");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .map(|value| convert("thinkthen_threshold", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("threshold", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("value");
            let converted: Option<bool> = held.and_then(Value::as_bool);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("value", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_ranked_member_result" => {
            let mut tuple = super::output::tuple(kind);
            let held = value.get("answer");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_answer", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("answer", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("answer_id");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("answer_id", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("images");
            let converted: Option<Array<'static, PgHeapTuple<'static, AllocatedByRust>>> = held
                .and_then(Value::as_array)
                .map(|values| {
                    crate::ffi::composite_array(
                        "thinkthen_image",
                        values
                            .iter()
                            .map(|value| {
                                Some(value)
                                    .filter(|value| !value.is_null())
                                    .map(|value| convert("thinkthen_image", value))
                                    .transpose()
                            })
                            .collect::<Result<Vec<_>, thinkthen::Error>>()?,
                    )
                })
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("images", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("meta");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_metadata", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("meta", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("question");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_question", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("question", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("schema");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("schema", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("source");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_source_location", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("source", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("threshold");
            tuple
                .set_by_name("threshold_present", held.is_some())
                .map_err(|_| crate::call::defect("generated result presence is unavailable"))?;
            let converted: Option<pgrx::datum::JsonB> =
                held.map(|value| pgrx::datum::JsonB(value.clone()));
            if let Some(converted) = converted {
                tuple
                    .set_by_name("threshold", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("value");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("value", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_named_ranked_member" => {
            let mut tuple = super::output::tuple(kind);
            let held = value.get("name");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("name", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("result");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_ranked_member_result", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("result", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_ranked" => {
            let mut tuple = super::output::tuple(kind);
            let held = value.get("answer");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_answer", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("answer", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("answer_id");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("answer_id", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("file");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("file", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("first_line");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("first_line", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("images");
            let converted: Option<Array<'static, PgHeapTuple<'static, AllocatedByRust>>> = held
                .and_then(Value::as_array)
                .map(|values| {
                    crate::ffi::composite_array(
                        "thinkthen_image",
                        values
                            .iter()
                            .map(|value| {
                                Some(value)
                                    .filter(|value| !value.is_null())
                                    .map(|value| convert("thinkthen_image", value))
                                    .transpose()
                            })
                            .collect::<Result<Vec<_>, thinkthen::Error>>()?,
                    )
                })
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("images", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("index");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("index", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("input");
            tuple
                .set_by_name("input_present", held.is_some())
                .map_err(|_| crate::call::defect("generated result presence is unavailable"))?;
            let converted: Option<pgrx::datum::JsonB> =
                held.map(|value| pgrx::datum::JsonB(value.clone()));
            if let Some(converted) = converted {
                tuple
                    .set_by_name("input", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("input_file");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("input_file", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("last_line");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("last_line", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("members");
            let converted: Option<Array<'static, PgHeapTuple<'static, AllocatedByRust>>> = held
                .and_then(Value::as_array)
                .map(|values| {
                    crate::ffi::composite_array(
                        "thinkthen_named_ranked_member",
                        values
                            .iter()
                            .map(|value| {
                                Some(value)
                                    .filter(|value| !value.is_null())
                                    .map(|value| convert("thinkthen_named_ranked_member", value))
                                    .transpose()
                            })
                            .collect::<Result<Vec<_>, thinkthen::Error>>()?,
                    )
                })
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("members", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("meta");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_metadata", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("meta", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("position");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_position", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("position", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("question");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_question", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("question", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("question_name");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("question_name", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("schema");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("schema", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("source");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_source_location", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("source", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("threshold");
            tuple
                .set_by_name("threshold_present", held.is_some())
                .map_err(|_| crate::call::defect("generated result presence is unavailable"))?;
            let converted: Option<pgrx::datum::JsonB> =
                held.map(|value| pgrx::datum::JsonB(value.clone()));
            if let Some(converted) = converted {
                tuple
                    .set_by_name("threshold", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("value");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("value", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_failure" => {
            let mut tuple = super::output::tuple(kind);
            let held = value.get("cause");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("cause", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("kind");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("kind", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_answer_value" => {
            let mut tuple = super::output::tuple(kind);
            let held = Some(value);
            let converted: Option<Vec<Option<String>>> = held
                .and_then(Value::as_array)
                .map(|values| {
                    values
                        .iter()
                        .map(|value| Ok(Some(value).and_then(Value::as_str).map(str::to_owned)))
                        .collect::<Result<Vec<_>, thinkthen::Error>>()
                })
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("array_value", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = Some(value);
            let converted: Option<bool> = held.and_then(Value::as_bool);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("boolean_value", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = Some(value);
            let converted: Option<f64> = held.and_then(Value::as_f64);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("number_value", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = Some(value);
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("string_value", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_annotation_answer" => {
            let mut tuple = super::output::tuple(kind);
            let held = value.get("answer");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_answer", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("answer", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("answer_id");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("answer_id", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("failure");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_failure", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("failure", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("failure_id");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("failure_id", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("observations");
            let converted: Option<Array<'static, PgHeapTuple<'static, AllocatedByRust>>> = held
                .and_then(Value::as_array)
                .map(|values| {
                    crate::ffi::composite_array(
                        "thinkthen_observation",
                        values
                            .iter()
                            .map(|value| {
                                Some(value)
                                    .filter(|value| !value.is_null())
                                    .map(|value| convert("thinkthen_observation", value))
                                    .transpose()
                            })
                            .collect::<Result<Vec<_>, thinkthen::Error>>()?,
                    )
                })
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("observations", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("question");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_question", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("question", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("question_sources");
            let converted: Option<Array<'static, PgHeapTuple<'static, AllocatedByRust>>> = held
                .and_then(Value::as_array)
                .map(|values| {
                    crate::ffi::composite_array(
                        "thinkthen_question_source",
                        values
                            .iter()
                            .map(|value| {
                                Some(value)
                                    .filter(|value| !value.is_null())
                                    .map(|value| convert("thinkthen_question_source", value))
                                    .transpose()
                            })
                            .collect::<Result<Vec<_>, thinkthen::Error>>()?,
                    )
                })
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("question_sources", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("request");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("request", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("threshold");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .map(|value| convert("thinkthen_threshold", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("threshold", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("usage");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_token_usage", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("usage", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("value");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .map(|value| convert("thinkthen_answer_value", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("value", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_named_answer" => {
            let mut tuple = super::output::tuple(kind);
            let held = value.get("name");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("name", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("value");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_annotation_answer", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("value", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_failed_value" => {
            let mut tuple = super::output::tuple(kind);
            let held = value.get("failed");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_failure", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("failed", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_field_value" => {
            let mut tuple = super::output::tuple(kind);
            let held = Some(value);
            let converted: Option<Vec<Option<String>>> = held
                .and_then(Value::as_array)
                .map(|values| {
                    values
                        .iter()
                        .map(|value| Ok(Some(value).and_then(Value::as_str).map(str::to_owned)))
                        .collect::<Result<Vec<_>, thinkthen::Error>>()
                })
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("array_value", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = Some(value);
            let converted: Option<bool> = held.and_then(Value::as_bool);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("boolean_value", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = Some(value);
            let converted: Option<f64> = held.and_then(Value::as_f64);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("number_value", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = Some(value);
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_failed_value", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("object_value", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = Some(value);
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("string_value", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_named_field_value" => {
            let mut tuple = super::output::tuple(kind);
            let held = value.get("name");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("name", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("value");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .map(|value| convert("thinkthen_field_value", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("value", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_annotation" => {
            let mut tuple = super::output::tuple(kind);
            let held = value.get("answer_id");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("answer_id", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("answers");
            let converted: Option<Array<'static, PgHeapTuple<'static, AllocatedByRust>>> = held
                .and_then(Value::as_object)
                .map(|values| {
                    crate::ffi::composite_array(
                        "thinkthen_named_answer",
                        values
                            .iter()
                            .map(|(name, value)| {
                                let entry = serde_json::json!({"name": name, "value": value});
                                convert("thinkthen_named_answer", &entry).map(Some)
                            })
                            .collect::<Result<Vec<_>, thinkthen::Error>>()?,
                    )
                })
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("answers", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("file");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("file", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("first_line");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("first_line", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("index");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("index", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("input");
            tuple
                .set_by_name("input_present", held.is_some())
                .map_err(|_| crate::call::defect("generated result presence is unavailable"))?;
            let converted: Option<pgrx::datum::JsonB> =
                held.map(|value| pgrx::datum::JsonB(value.clone()));
            if let Some(converted) = converted {
                tuple
                    .set_by_name("input", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("last_line");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("last_line", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("meta");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_metadata", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("meta", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("position");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_position", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("position", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("schema");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("schema", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("source");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_source_location", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("source", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("value");
            let converted: Option<Array<'static, PgHeapTuple<'static, AllocatedByRust>>> = held
                .and_then(Value::as_object)
                .map(|values| {
                    crate::ffi::composite_array(
                        "thinkthen_named_field_value",
                        values
                            .iter()
                            .map(|(name, value)| {
                                let entry = serde_json::json!({"name": name, "value": value});
                                convert("thinkthen_named_field_value", &entry).map(Some)
                            })
                            .collect::<Result<Vec<_>, thinkthen::Error>>()?,
                    )
                })
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("value", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_entity_probabilities" => {
            let mut tuple = super::output::tuple(kind);
            let held = value.get("edges");
            let converted: Option<Array<'static, PgHeapTuple<'static, AllocatedByRust>>> = held
                .and_then(Value::as_object)
                .map(|values| {
                    crate::ffi::composite_array(
                        "thinkthen_named_probability",
                        values
                            .iter()
                            .map(|(name, value)| {
                                let entry = serde_json::json!({"name": name, "value": value});
                                convert("thinkthen_named_probability", &entry).map(Some)
                            })
                            .collect::<Result<Vec<_>, thinkthen::Error>>()?,
                    )
                })
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("edges", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("end");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("end", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("kinds");
            let converted: Option<Array<'static, PgHeapTuple<'static, AllocatedByRust>>> = held
                .and_then(Value::as_object)
                .map(|values| {
                    crate::ffi::composite_array(
                        "thinkthen_named_probability",
                        values
                            .iter()
                            .map(|(name, value)| {
                                let entry = serde_json::json!({"name": name, "value": value});
                                convert("thinkthen_named_probability", &entry).map(Some)
                            })
                            .collect::<Result<Vec<_>, thinkthen::Error>>()?,
                    )
                })
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("kinds", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("start");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("start", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_place" => {
            let mut tuple = super::output::tuple(kind);
            let held = value.get("end");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("end", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("start");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("start", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_relation_probabilities" => {
            let mut tuple = super::output::tuple(kind);
            let held = value.get("probability");
            let converted: Option<f64> = held.and_then(Value::as_f64);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("probability", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("relation");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("relation", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("source");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_place", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("source", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("target");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_place", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("target", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_token_probabilities" => {
            let mut tuple = super::output::tuple(kind);
            let held = value.get("end");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("end", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("start");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("start", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("tags");
            let converted: Option<Array<'static, PgHeapTuple<'static, AllocatedByRust>>> = held
                .and_then(Value::as_object)
                .map(|values| {
                    crate::ffi::composite_array(
                        "thinkthen_named_probability",
                        values
                            .iter()
                            .map(|(name, value)| {
                                let entry = serde_json::json!({"name": name, "value": value});
                                convert("thinkthen_named_probability", &entry).map(Some)
                            })
                            .collect::<Result<Vec<_>, thinkthen::Error>>()?,
                    )
                })
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("tags", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_selected_span" => {
            let mut tuple = super::output::tuple(kind);
            let held = value.get("end");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("end", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("start");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("start", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_boundary_reading" => {
            let mut tuple = super::output::tuple(kind);
            let held = value.get("end");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("end", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("file");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("file", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("first_line");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("first_line", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("kept");
            let converted: Option<bool> = held.and_then(Value::as_bool);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("kept", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("kind");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("kind", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("last_line");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("last_line", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("length");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("length", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("probability");
            let converted: Option<f64> = held.and_then(Value::as_f64);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("probability", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("selected");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_selected_span", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("selected", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("span_probability");
            let converted: Option<f64> = held.and_then(Value::as_f64);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("span_probability", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("start");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("start", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("strength");
            let converted: Option<f64> = held.and_then(Value::as_f64);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("strength", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("text");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("text", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_recognition_probabilities" => {
            let mut tuple = super::output::tuple(kind);
            let held = value.get("names");
            let converted: Option<Array<'static, PgHeapTuple<'static, AllocatedByRust>>> = held
                .and_then(Value::as_array)
                .map(|values| {
                    crate::ffi::composite_array(
                        "thinkthen_entity_probabilities",
                        values
                            .iter()
                            .map(|value| {
                                Some(value)
                                    .filter(|value| !value.is_null())
                                    .map(|value| convert("thinkthen_entity_probabilities", value))
                                    .transpose()
                            })
                            .collect::<Result<Vec<_>, thinkthen::Error>>()?,
                    )
                })
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("names", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("pairs");
            let converted: Option<Array<'static, PgHeapTuple<'static, AllocatedByRust>>> = held
                .and_then(Value::as_array)
                .map(|values| {
                    crate::ffi::composite_array(
                        "thinkthen_relation_probabilities",
                        values
                            .iter()
                            .map(|value| {
                                Some(value)
                                    .filter(|value| !value.is_null())
                                    .map(|value| convert("thinkthen_relation_probabilities", value))
                                    .transpose()
                            })
                            .collect::<Result<Vec<_>, thinkthen::Error>>()?,
                    )
                })
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("pairs", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("pieces");
            let converted: Option<Array<'static, PgHeapTuple<'static, AllocatedByRust>>> = held
                .and_then(Value::as_array)
                .map(|values| {
                    crate::ffi::composite_array(
                        "thinkthen_token_probabilities",
                        values
                            .iter()
                            .map(|value| {
                                Some(value)
                                    .filter(|value| !value.is_null())
                                    .map(|value| convert("thinkthen_token_probabilities", value))
                                    .transpose()
                            })
                            .collect::<Result<Vec<_>, thinkthen::Error>>()?,
                    )
                })
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("pieces", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("proposals");
            let converted: Option<Array<'static, PgHeapTuple<'static, AllocatedByRust>>> = held
                .and_then(Value::as_array)
                .map(|values| {
                    crate::ffi::composite_array(
                        "thinkthen_boundary_reading",
                        values
                            .iter()
                            .map(|value| {
                                Some(value)
                                    .filter(|value| !value.is_null())
                                    .map(|value| convert("thinkthen_boundary_reading", value))
                                    .transpose()
                            })
                            .collect::<Result<Vec<_>, thinkthen::Error>>()?,
                    )
                })
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("proposals", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_kind_description" => {
            let mut tuple = super::output::tuple(kind);
            let held = value.get("name");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("name", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("value");
            tuple
                .set_by_name("value_present", held.is_some())
                .map_err(|_| crate::call::defect("generated result presence is unavailable"))?;
            let converted: Option<pgrx::datum::JsonB> =
                held.map(|value| pgrx::datum::JsonB(value.clone()));
            if let Some(converted) = converted {
                tuple
                    .set_by_name("value", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_relation_rule" => {
            let mut tuple = super::output::tuple(kind);
            let held = value.get("either");
            let converted: Option<bool> = held.and_then(Value::as_bool);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("either", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("name");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("name", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("reads");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("reads", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("single");
            let converted: Option<bool> = held.and_then(Value::as_bool);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("single", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("source");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("source", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("target");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("target", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_recognition_stage_context" => {
            let mut tuple = super::output::tuple(kind);
            let held = value.get("boundary");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("boundary", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("kind_edge");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("kind_edge", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("relation");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("relation", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_recognition_question" => {
            let mut tuple = super::output::tuple(kind);
            let held = value.get("batch");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_batch_setting", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("batch", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("context_schema");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_input_declaration", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("context_schema", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("entity_definition");
            tuple
                .set_by_name("entity_definition_present", held.is_some())
                .map_err(|_| crate::call::defect("generated result presence is unavailable"))?;
            let converted: Option<pgrx::datum::JsonB> =
                held.map(|value| pgrx::datum::JsonB(value.clone()));
            if let Some(converted) = converted {
                tuple
                    .set_by_name("entity_definition", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("instructions");
            tuple
                .set_by_name("instructions_present", held.is_some())
                .map_err(|_| crate::call::defect("generated result presence is unavailable"))?;
            let converted: Option<pgrx::datum::JsonB> =
                held.map(|value| pgrx::datum::JsonB(value.clone()));
            if let Some(converted) = converted {
                tuple
                    .set_by_name("instructions", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("item_schema");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_input_declaration", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("item_schema", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("kinds");
            let converted: Option<Array<'static, PgHeapTuple<'static, AllocatedByRust>>> = held
                .and_then(Value::as_object)
                .map(|values| {
                    crate::ffi::composite_array(
                        "thinkthen_kind_description",
                        values
                            .iter()
                            .map(|(name, value)| {
                                let entry = serde_json::json!({"name": name, "value": value});
                                convert("thinkthen_kind_description", &entry).map(Some)
                            })
                            .collect::<Result<Vec<_>, thinkthen::Error>>()?,
                    )
                })
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("kinds", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("label_details");
            let converted: Option<Array<'static, PgHeapTuple<'static, AllocatedByRust>>> = held
                .and_then(Value::as_array)
                .map(|values| {
                    crate::ffi::composite_array(
                        "thinkthen_label",
                        values
                            .iter()
                            .map(|value| {
                                Some(value)
                                    .filter(|value| !value.is_null())
                                    .map(|value| convert("thinkthen_label", value))
                                    .transpose()
                            })
                            .collect::<Result<Vec<_>, thinkthen::Error>>()?,
                    )
                })
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("label_details", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("mode");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("mode", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("model");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("model", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("name");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("name", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("on");
            let converted: Option<Vec<Option<String>>> = held
                .and_then(Value::as_array)
                .map(|values| {
                    values
                        .iter()
                        .map(|value| Ok(Some(value).and_then(Value::as_str).map(str::to_owned)))
                        .collect::<Result<Vec<_>, thinkthen::Error>>()
                })
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("on", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("profile");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("profile", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("relation_threshold");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_threshold", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("relation_threshold", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("relations");
            let converted: Option<Array<'static, PgHeapTuple<'static, AllocatedByRust>>> = held
                .and_then(Value::as_array)
                .map(|values| {
                    crate::ffi::composite_array(
                        "thinkthen_relation_rule",
                        values
                            .iter()
                            .map(|value| {
                                Some(value)
                                    .filter(|value| !value.is_null())
                                    .map(|value| convert("thinkthen_relation_rule", value))
                                    .transpose()
                            })
                            .collect::<Result<Vec<_>, thinkthen::Error>>()?,
                    )
                })
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("relations", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("snippet_pieces");
            let converted: Option<i64> = held.and_then(Value::as_i64);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("snippet_pieces", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("stage_context");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_recognition_stage_context", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("stage_context", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("threshold");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_threshold", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("threshold", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("verb");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("verb", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("wording_version");
            let converted: Option<i64> = held.and_then(Value::as_i64);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("wording_version", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_entity" => {
            let mut tuple = super::output::tuple(kind);
            let held = value.get("end");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("end", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("file");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("file", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("first_line");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("first_line", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("kind");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("kind", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("last_line");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("last_line", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("length");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("length", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("start");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("start", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("strength");
            let converted: Option<f64> = held.and_then(Value::as_f64);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("strength", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("text");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("text", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_boundary_proposal" => {
            let mut tuple = super::output::tuple(kind);
            let held = value.get("end");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("end", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("file");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("file", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("first_line");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("first_line", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("last_line");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("last_line", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("length");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("length", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("probability");
            let converted: Option<f64> = held.and_then(Value::as_f64);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("probability", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("start");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("start", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("text");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("text", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_entity_edge" => {
            let mut tuple = super::output::tuple(kind);
            let held = value.get("either");
            let converted: Option<bool> = held.and_then(Value::as_bool);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("either", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("probability");
            let converted: Option<f64> = held.and_then(Value::as_f64);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("probability", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("relation");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("relation", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("source");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_entity", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("source", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("target");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_entity", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("target", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_recognized_value" => {
            let mut tuple = super::output::tuple(kind);
            let held = value.get("entities");
            let converted: Option<Array<'static, PgHeapTuple<'static, AllocatedByRust>>> = held
                .and_then(Value::as_array)
                .map(|values| {
                    crate::ffi::composite_array(
                        "thinkthen_entity",
                        values
                            .iter()
                            .map(|value| {
                                Some(value)
                                    .filter(|value| !value.is_null())
                                    .map(|value| convert("thinkthen_entity", value))
                                    .transpose()
                            })
                            .collect::<Result<Vec<_>, thinkthen::Error>>()?,
                    )
                })
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("entities", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("mode");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("mode", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("proposals");
            let converted: Option<Array<'static, PgHeapTuple<'static, AllocatedByRust>>> = held
                .and_then(Value::as_array)
                .map(|values| {
                    crate::ffi::composite_array(
                        "thinkthen_boundary_proposal",
                        values
                            .iter()
                            .map(|value| {
                                Some(value)
                                    .filter(|value| !value.is_null())
                                    .map(|value| convert("thinkthen_boundary_proposal", value))
                                    .transpose()
                            })
                            .collect::<Result<Vec<_>, thinkthen::Error>>()?,
                    )
                })
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("proposals", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("relations");
            let converted: Option<Array<'static, PgHeapTuple<'static, AllocatedByRust>>> = held
                .and_then(Value::as_array)
                .map(|values| {
                    crate::ffi::composite_array(
                        "thinkthen_entity_edge",
                        values
                            .iter()
                            .map(|value| {
                                Some(value)
                                    .filter(|value| !value.is_null())
                                    .map(|value| convert("thinkthen_entity_edge", value))
                                    .transpose()
                            })
                            .collect::<Result<Vec<_>, thinkthen::Error>>()?,
                    )
                })
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("relations", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_recognized" => {
            let mut tuple = super::output::tuple(kind);
            let held = value.get("answer");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_recognition_probabilities", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("answer", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("answer_id");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("answer_id", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("file");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("file", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("first_line");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("first_line", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("index");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("index", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("input");
            tuple
                .set_by_name("input_present", held.is_some())
                .map_err(|_| crate::call::defect("generated result presence is unavailable"))?;
            let converted: Option<pgrx::datum::JsonB> =
                held.map(|value| pgrx::datum::JsonB(value.clone()));
            if let Some(converted) = converted {
                tuple
                    .set_by_name("input", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("last_line");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("last_line", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("meta");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_metadata", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("meta", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("position");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_position", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("position", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("question");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_recognition_question", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("question", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("schema");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("schema", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("source");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_source_location", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("source", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("value");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_recognized_value", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("value", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_candidate_probability" => {
            let mut tuple = super::output::tuple(kind);
            let held = value.get("name");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("name", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("value");
            let converted: Option<f64> = held.and_then(Value::as_f64);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("value", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_find_answer" => {
            let mut tuple = super::output::tuple(kind);
            let held = value.get("confidence");
            let converted: Option<f64> = held.and_then(Value::as_f64);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("confidence", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("kind");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("kind", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("pick");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("pick", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("probabilities");
            let converted: Option<Array<'static, PgHeapTuple<'static, AllocatedByRust>>> = held
                .and_then(Value::as_object)
                .map(|values| {
                    crate::ffi::composite_array(
                        "thinkthen_candidate_probability",
                        values
                            .iter()
                            .map(|(name, value)| {
                                let entry = serde_json::json!({"name": name, "value": value});
                                convert("thinkthen_candidate_probability", &entry).map(Some)
                            })
                            .collect::<Result<Vec<_>, thinkthen::Error>>()?,
                    )
                })
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("probabilities", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_find_candidate" => {
            let mut tuple = super::output::tuple(kind);
            let held = value.get("index");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("index", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("input");
            tuple
                .set_by_name("input_present", held.is_some())
                .map_err(|_| crate::call::defect("generated result presence is unavailable"))?;
            let converted: Option<pgrx::datum::JsonB> =
                held.map(|value| pgrx::datum::JsonB(value.clone()));
            if let Some(converted) = converted {
                tuple
                    .set_by_name("input", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("probability");
            let converted: Option<f64> = held.and_then(Value::as_f64);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("probability", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("source");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_source_location", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("source", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_find_question" => {
            let mut tuple = super::output::tuple(kind);
            let held = value.get("batch");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_batch_setting", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("batch", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("context_schema");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_input_declaration", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("context_schema", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("item_schema");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_input_declaration", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("item_schema", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("label_details");
            let converted: Option<Array<'static, PgHeapTuple<'static, AllocatedByRust>>> = held
                .and_then(Value::as_array)
                .map(|values| {
                    crate::ffi::composite_array(
                        "thinkthen_label",
                        values
                            .iter()
                            .map(|value| {
                                Some(value)
                                    .filter(|value| !value.is_null())
                                    .map(|value| convert("thinkthen_label", value))
                                    .transpose()
                            })
                            .collect::<Result<Vec<_>, thinkthen::Error>>()?,
                    )
                })
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("label_details", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("model");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("model", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("name");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("name", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("none");
            let converted: Option<bool> = held.and_then(Value::as_bool);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("none", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("on");
            let converted: Option<Vec<Option<String>>> = held
                .and_then(Value::as_array)
                .map(|values| {
                    values
                        .iter()
                        .map(|value| Ok(Some(value).and_then(Value::as_str).map(str::to_owned)))
                        .collect::<Result<Vec<_>, thinkthen::Error>>()
                })
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("on", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("profile");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("profile", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("text");
            tuple
                .set_by_name("text_present", held.is_some())
                .map_err(|_| crate::call::defect("generated result presence is unavailable"))?;
            let converted: Option<pgrx::datum::JsonB> =
                held.map(|value| pgrx::datum::JsonB(value.clone()));
            if let Some(converted) = converted {
                tuple
                    .set_by_name("text", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("verb");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("verb", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("wording_version");
            let converted: Option<i64> = held.and_then(Value::as_i64);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("wording_version", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_found" => {
            let mut tuple = super::output::tuple(kind);
            let held = value.get("answer");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_find_answer", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("answer", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("answer_id");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("answer_id", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("candidates");
            let converted: Option<Array<'static, PgHeapTuple<'static, AllocatedByRust>>> = held
                .and_then(Value::as_array)
                .map(|values| {
                    crate::ffi::composite_array(
                        "thinkthen_find_candidate",
                        values
                            .iter()
                            .map(|value| {
                                Some(value)
                                    .filter(|value| !value.is_null())
                                    .map(|value| convert("thinkthen_find_candidate", value))
                                    .transpose()
                            })
                            .collect::<Result<Vec<_>, thinkthen::Error>>()?,
                    )
                })
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("candidates", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("file");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("file", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("first_line");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("first_line", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("index");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("index", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("last_line");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("last_line", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("meta");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_metadata", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("meta", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("position");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_position", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("position", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("question");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_find_question", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("question", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("schema");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("schema", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("threshold");
            tuple
                .set_by_name("threshold_present", held.is_some())
                .map_err(|_| crate::call::defect("generated result presence is unavailable"))?;
            let converted: Option<pgrx::datum::JsonB> =
                held.map(|value| pgrx::datum::JsonB(value.clone()));
            if let Some(converted) = converted {
                tuple
                    .set_by_name("threshold", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("value");
            tuple
                .set_by_name("value_present", held.is_some())
                .map_err(|_| crate::call::defect("generated result presence is unavailable"))?;
            let converted: Option<pgrx::datum::JsonB> =
                held.map(|value| pgrx::datum::JsonB(value.clone()));
            if let Some(converted) = converted {
                tuple
                    .set_by_name("value", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_related_entity" => {
            let mut tuple = super::output::tuple(kind);
            let held = value.get("kind");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("kind", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("name");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("name", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_relation_endpoint" => {
            let mut tuple = super::output::tuple(kind);
            let held = value.get("kind");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("kind", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("name");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("name", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_relation_reading" => {
            let mut tuple = super::output::tuple(kind);
            let held = value.get("accepted");
            let converted: Option<bool> = held.and_then(Value::as_bool);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("accepted", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("answer");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_answer", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("answer", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("answer_id");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("answer_id", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("direction");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("direction", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("failure");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_failure", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("failure", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("failure_id");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("failure_id", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("method");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("method", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("observations");
            let converted: Option<Array<'static, PgHeapTuple<'static, AllocatedByRust>>> = held
                .and_then(Value::as_array)
                .map(|values| {
                    crate::ffi::composite_array(
                        "thinkthen_observation",
                        values
                            .iter()
                            .map(|value| {
                                Some(value)
                                    .filter(|value| !value.is_null())
                                    .map(|value| convert("thinkthen_observation", value))
                                    .transpose()
                            })
                            .collect::<Result<Vec<_>, thinkthen::Error>>()?,
                    )
                })
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("observations", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("probability");
            let converted: Option<f64> = held.and_then(Value::as_f64);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("probability", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("question");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_question", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("question", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("question_sources");
            let converted: Option<Array<'static, PgHeapTuple<'static, AllocatedByRust>>> = held
                .and_then(Value::as_array)
                .map(|values| {
                    crate::ffi::composite_array(
                        "thinkthen_question_source",
                        values
                            .iter()
                            .map(|value| {
                                Some(value)
                                    .filter(|value| !value.is_null())
                                    .map(|value| convert("thinkthen_question_source", value))
                                    .transpose()
                            })
                            .collect::<Result<Vec<_>, thinkthen::Error>>()?,
                    )
                })
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("question_sources", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("reads");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("reads", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("relation");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("relation", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("request");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("request", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("source");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_related_entity", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("source", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("target");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_relation_endpoint", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("target", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("threshold");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_threshold", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("threshold", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("usage");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_token_usage", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("usage", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_relation_answers" => {
            let mut tuple = super::output::tuple(kind);
            let held = value.get("questions");
            let converted: Option<Array<'static, PgHeapTuple<'static, AllocatedByRust>>> = held
                .and_then(Value::as_array)
                .map(|values| {
                    crate::ffi::composite_array(
                        "thinkthen_relation_reading",
                        values
                            .iter()
                            .map(|value| {
                                Some(value)
                                    .filter(|value| !value.is_null())
                                    .map(|value| convert("thinkthen_relation_reading", value))
                                    .transpose()
                            })
                            .collect::<Result<Vec<_>, thinkthen::Error>>()?,
                    )
                })
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("questions", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_indexed_source" => {
            let mut tuple = super::output::tuple(kind);
            let held = value.get("index");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("index", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("source");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_source_location", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("source", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_relation_fields" => {
            let mut tuple = super::output::tuple(kind);
            let held = value.get("kind");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("kind", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("name");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("name", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_relation_question" => {
            let mut tuple = super::output::tuple(kind);
            let held = value.get("batch");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_batch_setting", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("batch", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("context_schema");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_input_declaration", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("context_schema", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("fields");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_relation_fields", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("fields", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("item_schema");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_input_declaration", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("item_schema", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("label_details");
            let converted: Option<Array<'static, PgHeapTuple<'static, AllocatedByRust>>> = held
                .and_then(Value::as_array)
                .map(|values| {
                    crate::ffi::composite_array(
                        "thinkthen_label",
                        values
                            .iter()
                            .map(|value| {
                                Some(value)
                                    .filter(|value| !value.is_null())
                                    .map(|value| convert("thinkthen_label", value))
                                    .transpose()
                            })
                            .collect::<Result<Vec<_>, thinkthen::Error>>()?,
                    )
                })
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("label_details", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("model");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("model", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("name");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("name", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("on");
            let converted: Option<Vec<Option<String>>> = held
                .and_then(Value::as_array)
                .map(|values| {
                    values
                        .iter()
                        .map(|value| Ok(Some(value).and_then(Value::as_str).map(str::to_owned)))
                        .collect::<Result<Vec<_>, thinkthen::Error>>()
                })
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("on", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("profile");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("profile", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("relations");
            let converted: Option<Array<'static, PgHeapTuple<'static, AllocatedByRust>>> = held
                .and_then(Value::as_array)
                .map(|values| {
                    crate::ffi::composite_array(
                        "thinkthen_relation_rule",
                        values
                            .iter()
                            .map(|value| {
                                Some(value)
                                    .filter(|value| !value.is_null())
                                    .map(|value| convert("thinkthen_relation_rule", value))
                                    .transpose()
                            })
                            .collect::<Result<Vec<_>, thinkthen::Error>>()?,
                    )
                })
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("relations", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("threshold");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_threshold", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("threshold", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("verb");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("verb", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("wording_version");
            let converted: Option<i64> = held.and_then(Value::as_i64);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("wording_version", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_located_endpoint" => {
            let mut tuple = super::output::tuple(kind);
            let held = value.get("file");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("file", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("first_line");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("first_line", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("kind");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("kind", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("last_line");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("last_line", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("name");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("name", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("ordinal");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("ordinal", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("record");
            tuple
                .set_by_name("record_present", held.is_some())
                .map_err(|_| crate::call::defect("generated result presence is unavailable"))?;
            let converted: Option<pgrx::datum::JsonB> =
                held.map(|value| pgrx::datum::JsonB(value.clone()));
            if let Some(converted) = converted {
                tuple
                    .set_by_name("record", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_located_edge" => {
            let mut tuple = super::output::tuple(kind);
            let held = value.get("either");
            let converted: Option<bool> = held.and_then(Value::as_bool);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("either", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("probability");
            let converted: Option<f64> = held.and_then(Value::as_f64);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("probability", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("relation");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("relation", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("source");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_located_endpoint", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("source", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("target");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_located_endpoint", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("target", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_related" => {
            let mut tuple = super::output::tuple(kind);
            let held = value.get("answer");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_relation_answers", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("answer", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("answer_id");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("answer_id", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("file");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("file", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("first_line");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("first_line", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("index");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("index", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("input");
            tuple
                .set_by_name("input_present", held.is_some())
                .map_err(|_| crate::call::defect("generated result presence is unavailable"))?;
            let converted: Option<pgrx::datum::JsonB> =
                held.map(|value| pgrx::datum::JsonB(value.clone()));
            if let Some(converted) = converted {
                tuple
                    .set_by_name("input", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("input_sources");
            let converted: Option<Array<'static, PgHeapTuple<'static, AllocatedByRust>>> = held
                .and_then(Value::as_array)
                .map(|values| {
                    crate::ffi::composite_array(
                        "thinkthen_indexed_source",
                        values
                            .iter()
                            .map(|value| {
                                Some(value)
                                    .filter(|value| !value.is_null())
                                    .map(|value| convert("thinkthen_indexed_source", value))
                                    .transpose()
                            })
                            .collect::<Result<Vec<_>, thinkthen::Error>>()?,
                    )
                })
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("input_sources", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("last_line");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("last_line", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("meta");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_metadata", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("meta", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("position");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_position", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("position", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("question");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_relation_question", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("question", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("schema");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("schema", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("value");
            let converted: Option<Array<'static, PgHeapTuple<'static, AllocatedByRust>>> = held
                .and_then(Value::as_array)
                .map(|values| {
                    crate::ffi::composite_array(
                        "thinkthen_located_edge",
                        values
                            .iter()
                            .map(|value| {
                                Some(value)
                                    .filter(|value| !value.is_null())
                                    .map(|value| convert("thinkthen_located_edge", value))
                                    .transpose()
                            })
                            .collect::<Result<Vec<_>, thinkthen::Error>>()?,
                    )
                })
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("value", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_usage_persistence" => {
            let mut tuple = super::output::tuple(kind);
            let held = value.get("advice");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("advice", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("observed_at");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("observed_at", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("state");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("state", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_facts" => {
            let mut tuple = super::output::tuple(kind);
            let held = value.get("attempts");
            let converted: Option<Array<'static, PgHeapTuple<'static, AllocatedByRust>>> = held
                .and_then(Value::as_array)
                .map(|values| {
                    crate::ffi::composite_array(
                        "thinkthen_attempt",
                        values
                            .iter()
                            .map(|value| {
                                Some(value)
                                    .filter(|value| !value.is_null())
                                    .map(|value| convert("thinkthen_attempt", value))
                                    .transpose()
                            })
                            .collect::<Result<Vec<_>, thinkthen::Error>>()?,
                    )
                })
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("attempts", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("cache_answers");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("cache_answers", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("call_id");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("call_id", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("estimated_cost_usd");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("estimated_cost_usd", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("held_model_mismatch");
            let converted: Option<bool> = held.and_then(Value::as_bool);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("held_model_mismatch", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("input_tokens");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("input_tokens", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("largest_request_bytes");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("largest_request_bytes", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("largest_request_estimated_input_tokens");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("largest_request_estimated_input_tokens", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("model");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("model", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("output_tokens");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("output_tokens", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("records");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("records", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("requests_sent");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("requests_sent", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("seconds");
            let converted: Option<f64> = held.and_then(Value::as_f64);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("seconds", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("token_estimate_method");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("token_estimate_method", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("usage_persistence");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_usage_persistence", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("usage_persistence", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_token_band" => {
            let mut tuple = super::output::tuple(kind);
            let held = value.get("lower");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("lower", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("upper");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("upper", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_plan" => {
            let mut tuple = super::output::tuple(kind);
            let held = value.get("estimated_bytes");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("estimated_bytes", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("estimated_input_tokens");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_token_band", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("estimated_input_tokens", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("first_body_utf8");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("first_body_utf8", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("largest_request_bytes");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("largest_request_bytes", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("largest_request_estimated_input_tokens");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("largest_request_estimated_input_tokens", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("records");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("records", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("requests");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("requests", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("token_estimate_method");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("token_estimate_method", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("upper_bound");
            let converted: Option<bool> = held.and_then(Value::as_bool);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("upper_bound", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_input_budget_denial" => {
            let mut tuple = super::output::tuple(kind);
            let held = value.get("kind");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("kind", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("last_status");
            let converted: Option<i64> = held.and_then(Value::as_i64);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("last_status", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("limit");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("limit", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_send_budget_denial" => {
            let mut tuple = super::output::tuple(kind);
            let held = value.get("kind");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("kind", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("last_status");
            let converted: Option<i64> = held.and_then(Value::as_i64);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("last_status", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_stopped" => {
            let mut tuple = super::output::tuple(kind);
            let held = value.get("at");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("at", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("cause");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("cause", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("retryable");
            let converted: Option<bool> = held.and_then(Value::as_bool);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("retryable", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("status");
            let converted: Option<i64> = held.and_then(Value::as_i64);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("status", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_error" => {
            let mut tuple = super::output::tuple(kind);
            let held = value.get("cause");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("cause", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("estimated_input_denial");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_input_budget_denial", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("estimated_input_denial", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("kind");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("kind", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("message");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("message", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("retryable");
            let converted: Option<bool> = held.and_then(Value::as_bool);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("retryable", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("send_budget_denial");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_send_budget_denial", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("send_budget_denial", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("stopped");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_stopped", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("stopped", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_observed_probability" => {
            let mut tuple = super::output::tuple(kind);
            let held = value.get("name");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("name", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("probability");
            let converted: Option<f64> = held.and_then(Value::as_f64);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("probability", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_probability_values" => {
            let mut tuple = super::output::tuple(kind);
            let held = Some(value);
            let converted: Option<Array<'static, PgHeapTuple<'static, AllocatedByRust>>> = held
                .and_then(Value::as_array)
                .map(|values| {
                    crate::ffi::composite_array(
                        "thinkthen_observed_probability",
                        values
                            .iter()
                            .map(|value| {
                                Some(value)
                                    .filter(|value| !value.is_null())
                                    .map(|value| convert("thinkthen_observed_probability", value))
                                    .transpose()
                            })
                            .collect::<Result<Vec<_>, thinkthen::Error>>()?,
                    )
                })
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("array_value", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = Some(value);
            let converted: Option<f64> = held.and_then(Value::as_f64);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("number_value", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_probability_distribution" => {
            let mut tuple = super::output::tuple(kind);
            let held = value.get("kind");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("kind", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("value");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_probability_values", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("value", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_reported_usage" => {
            let mut tuple = super::output::tuple(kind);
            let held = value.get("input_tokens");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("input_tokens", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("output_tokens");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("output_tokens", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_question_detail" => {
            let mut tuple = super::output::tuple(kind);
            let held = value.get("answer_id");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("answer_id", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("cached");
            let converted: Option<bool> = held.and_then(Value::as_bool);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("cached", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("confidence");
            let converted: Option<f64> = held.and_then(Value::as_f64);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("confidence", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("failed_questions");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("failed_questions", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("failure");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_failure", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("failure", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("failure_id");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("failure_id", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("input");
            tuple
                .set_by_name("input_present", held.is_some())
                .map_err(|_| crate::call::defect("generated result presence is unavailable"))?;
            let converted: Option<pgrx::datum::JsonB> =
                held.map(|value| pgrx::datum::JsonB(value.clone()));
            if let Some(converted) = converted {
                tuple
                    .set_by_name("input", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("input_source");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_source_location", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("input_source", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("input_sources");
            let converted: Option<Array<'static, PgHeapTuple<'static, AllocatedByRust>>> = held
                .and_then(Value::as_array)
                .map(|values| {
                    crate::ffi::composite_array(
                        "thinkthen_indexed_source",
                        values
                            .iter()
                            .map(|value| {
                                Some(value)
                                    .filter(|value| !value.is_null())
                                    .map(|value| convert("thinkthen_indexed_source", value))
                                    .transpose()
                            })
                            .collect::<Result<Vec<_>, thinkthen::Error>>()?,
                    )
                })
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("input_sources", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("inputs");
            let converted: Option<Vec<Option<pgrx::datum::JsonB>>> = held
                .and_then(Value::as_array)
                .map(|values| {
                    values
                        .iter()
                        .map(|value| Ok(Some(value).map(|value| pgrx::datum::JsonB(value.clone()))))
                        .collect::<Result<Vec<_>, thinkthen::Error>>()
                })
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("inputs", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("model");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("model", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("observations");
            let converted: Option<Array<'static, PgHeapTuple<'static, AllocatedByRust>>> = held
                .and_then(Value::as_array)
                .map(|values| {
                    crate::ffi::composite_array(
                        "thinkthen_observation",
                        values
                            .iter()
                            .map(|value| {
                                Some(value)
                                    .filter(|value| !value.is_null())
                                    .map(|value| convert("thinkthen_observation", value))
                                    .transpose()
                            })
                            .collect::<Result<Vec<_>, thinkthen::Error>>()?,
                    )
                })
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("observations", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("probabilities");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_probability_distribution", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("probabilities", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("question");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_question", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("question", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("question_sha256");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("question_sha256", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("question_sources");
            let converted: Option<Array<'static, PgHeapTuple<'static, AllocatedByRust>>> = held
                .and_then(Value::as_array)
                .map(|values| {
                    crate::ffi::composite_array(
                        "thinkthen_question_source",
                        values
                            .iter()
                            .map(|value| {
                                Some(value)
                                    .filter(|value| !value.is_null())
                                    .map(|value| convert("thinkthen_question_source", value))
                                    .transpose()
                            })
                            .collect::<Result<Vec<_>, thinkthen::Error>>()?,
                    )
                })
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("question_sources", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("raw_pick");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("raw_pick", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("reported_usage");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_reported_usage", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("reported_usage", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("requests");
            let converted: Option<Vec<Option<String>>> = held
                .and_then(Value::as_array)
                .map(|values| {
                    values
                        .iter()
                        .map(|value| Ok(Some(value).and_then(Value::as_str).map(str::to_owned)))
                        .collect::<Result<Vec<_>, thinkthen::Error>>()
                })
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("requests", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("requests_sent");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("requests_sent", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("threshold");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_threshold", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("threshold", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("url");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("url", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("usage");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_token_usage", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("usage", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("value");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .map(|value| convert("thinkthen_answer_value", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("value", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_observed_input" => {
            let mut tuple = super::output::tuple(kind);
            let held = value.get("images");
            let converted: Option<Array<'static, PgHeapTuple<'static, AllocatedByRust>>> = held
                .and_then(Value::as_array)
                .map(|values| {
                    crate::ffi::composite_array(
                        "thinkthen_image",
                        values
                            .iter()
                            .map(|value| {
                                Some(value)
                                    .filter(|value| !value.is_null())
                                    .map(|value| convert("thinkthen_image", value))
                                    .transpose()
                            })
                            .collect::<Result<Vec<_>, thinkthen::Error>>()?,
                    )
                })
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("images", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("input");
            tuple
                .set_by_name("input_present", held.is_some())
                .map_err(|_| crate::call::defect("generated result presence is unavailable"))?;
            let converted: Option<pgrx::datum::JsonB> =
                held.map(|value| pgrx::datum::JsonB(value.clone()));
            if let Some(converted) = converted {
                tuple
                    .set_by_name("input", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("source");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_source_location", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("source", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        "thinkthen_event" => {
            let mut tuple = super::output::tuple(kind);
            let held = value.get("detail");
            let converted: Option<PgHeapTuple<'static, AllocatedByRust>> = held
                .filter(|value| !value.is_null())
                .map(|value| convert("thinkthen_question_detail", value))
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("detail", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("index");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("index", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("inputs");
            let converted: Option<Array<'static, PgHeapTuple<'static, AllocatedByRust>>> = held
                .and_then(Value::as_array)
                .map(|values| {
                    crate::ffi::composite_array(
                        "thinkthen_observed_input",
                        values
                            .iter()
                            .map(|value| {
                                Some(value)
                                    .filter(|value| !value.is_null())
                                    .map(|value| convert("thinkthen_observed_input", value))
                                    .transpose()
                            })
                            .collect::<Result<Vec<_>, thinkthen::Error>>()?,
                    )
                })
                .transpose()?;
            if let Some(converted) = converted {
                tuple
                    .set_by_name("inputs", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("kind");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("kind", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("member");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("member", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("position");
            let converted: Option<pgrx::AnyNumeric> =
                held.and_then(Value::as_u64).map(pgrx::AnyNumeric::from);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("position", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("stage");
            let converted: Option<String> = held.and_then(Value::as_str).map(str::to_owned);
            if let Some(converted) = converted {
                tuple
                    .set_by_name("stage", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            let held = value.get("value");
            tuple
                .set_by_name("value_present", held.is_some())
                .map_err(|_| crate::call::defect("generated result presence is unavailable"))?;
            let converted: Option<pgrx::datum::JsonB> =
                held.map(|value| pgrx::datum::JsonB(value.clone()));
            if let Some(converted) = converted {
                tuple
                    .set_by_name("value", converted)
                    .map_err(|_| crate::call::defect("generated result field is unavailable"))?;
            }
            Ok(tuple)
        }
        _ => Err(crate::call::defect("unknown generated result type")),
    }
}
