// Generated from the canonical native result schema; do not edit.
#pragma once
inline LogicalType Receipt_related_entityType() {
return LogicalType::STRUCT({{"kind", LogicalType::VARCHAR}, {"name", LogicalType::VARCHAR}});
}
inline Value Receipt_related_entityValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_related_entityType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_related_entityType()); }
return Value::STRUCT(Receipt_related_entityType(), {ReceiptLeaf(ReceiptMember(value, "kind"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "name"), LogicalType::VARCHAR)});
}
inline LogicalType Receipt_relation_member_targetType() {
return LogicalType::STRUCT({{"kind", LogicalType::VARCHAR}, {"name", LogicalType::VARCHAR}});
}
inline Value Receipt_relation_member_targetValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_relation_member_targetType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_relation_member_targetType()); }
return Value::STRUCT(Receipt_relation_member_targetType(), {ReceiptLeaf(ReceiptMember(value, "kind"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "name"), LogicalType::VARCHAR)});
}
inline LogicalType Receipt_relation_member_usageType() {
return LogicalType::STRUCT({{"input_tokens", LogicalType::UBIGINT}, {"output_tokens", LogicalType::UBIGINT}});
}
inline Value Receipt_relation_member_usageValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_relation_member_usageType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_relation_member_usageType()); }
return Value::STRUCT(Receipt_relation_member_usageType(), {ReceiptLeaf(ReceiptMember(value, "input_tokens"), LogicalType::UBIGINT), ReceiptLeaf(ReceiptMember(value, "output_tokens"), LogicalType::UBIGINT)});
}
inline LogicalType Receipt_relation_memberType() {
return LogicalType::STRUCT({{"accepted", LogicalType::BOOLEAN}, {"answer", Receipt_answerType()}, {"answer_id", LogicalType::VARCHAR}, {"direction", LogicalType::VARCHAR}, {"failure", Receipt_failureType()}, {"failure_id", LogicalType::VARCHAR}, {"method", LogicalType::VARCHAR}, {"observations", LogicalType::LIST(Receipt_observationType())}, {"probability", LogicalType::DOUBLE}, {"question", Receipt_question_documentType()}, {"question_sources", LogicalType::LIST(Receipt_question_sourceType())}, {"reads", LogicalType::VARCHAR}, {"relation", LogicalType::VARCHAR}, {"request", LogicalType::VARCHAR}, {"source", Receipt_related_entityType()}, {"target", Receipt_relation_member_targetType()}, {"threshold", Receipt_thresholdType()}, {"usage", Receipt_relation_member_usageType()}});
}
inline Value Receipt_relation_memberValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_relation_memberType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_relation_memberType()); }
return Value::STRUCT(Receipt_relation_memberType(), {ReceiptLeaf(ReceiptMember(value, "accepted"), LogicalType::BOOLEAN), Receipt_answerValue(ReceiptMember(value, "answer")), ReceiptLeaf(ReceiptMember(value, "answer_id"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "direction"), LogicalType::VARCHAR), Receipt_failureValue(ReceiptMember(value, "failure")), ReceiptLeaf(ReceiptMember(value, "failure_id"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "method"), LogicalType::VARCHAR), ReceiptList(ReceiptMember(value, "observations"), Receipt_observationType(), [](ReceiptJSON *item) { return Receipt_observationValue(item); }), ReceiptLeaf(ReceiptMember(value, "probability"), LogicalType::DOUBLE), Receipt_question_documentValue(ReceiptMember(value, "question")), ReceiptList(ReceiptMember(value, "question_sources"), Receipt_question_sourceType(), [](ReceiptJSON *item) { return Receipt_question_sourceValue(item); }), ReceiptLeaf(ReceiptMember(value, "reads"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "relation"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "request"), LogicalType::VARCHAR), Receipt_related_entityValue(ReceiptMember(value, "source")), Receipt_relation_member_targetValue(ReceiptMember(value, "target")), Receipt_thresholdValue(ReceiptMember(value, "threshold")), Receipt_relation_member_usageValue(ReceiptMember(value, "usage"))});
}
inline LogicalType Receipt_answersType() {
return LogicalType::STRUCT({{"questions", LogicalType::LIST(Receipt_relation_memberType())}});
}
inline Value Receipt_answersValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_answersType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_answersType()); }
return Value::STRUCT(Receipt_answersType(), {ReceiptList(ReceiptMember(value, "questions"), Receipt_relation_memberType(), [](ReceiptJSON *item) { return Receipt_relation_memberValue(item); })});
}
inline LogicalType Receipt_physical_sourceType() {
return LogicalType::STRUCT({{"file", LogicalType::VARCHAR}, {"first_line", LogicalType::UBIGINT}, {"last_line", LogicalType::UBIGINT}});
}
inline Value Receipt_physical_sourceValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_physical_sourceType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_physical_sourceType()); }
return Value::STRUCT(Receipt_physical_sourceType(), {ReceiptLeaf(ReceiptMember(value, "file"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "first_line"), LogicalType::UBIGINT), ReceiptLeaf(ReceiptMember(value, "last_line"), LogicalType::UBIGINT)});
}
inline LogicalType Receipt_session_input_sourceType() {
return LogicalType::STRUCT({{"index", LogicalType::UBIGINT}, {"source", Receipt_physical_sourceType()}});
}
inline Value Receipt_session_input_sourceValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_session_input_sourceType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_session_input_sourceType()); }
return Value::STRUCT(Receipt_session_input_sourceType(), {ReceiptLeaf(ReceiptMember(value, "index"), LogicalType::UBIGINT), Receipt_physical_sourceValue(ReceiptMember(value, "source"))});
}
inline LogicalType Receipt_relate_question_document_batchType() {
return LogicalType::STRUCT({{"integer_value", LogicalType::UBIGINT}, {"string_value", LogicalType::VARCHAR}});
}
inline Value Receipt_relate_question_document_batchValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_relate_question_document_batchType()); }
return Value::STRUCT(Receipt_relate_question_document_batchType(), {ReceiptLeaf(value, LogicalType::UBIGINT), ReceiptLeaf(value, LogicalType::VARCHAR)});
}
inline LogicalType Receipt_relate_question_document_context_schema_properties_entryType() {
return LogicalType::STRUCT({{"name", LogicalType::VARCHAR}, {"value", Receipt_input_property_typeType()}});
}
inline Value Receipt_relate_question_document_context_schema_properties_entryValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_relate_question_document_context_schema_properties_entryType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_relate_question_document_context_schema_properties_entryType()); }
return Value::STRUCT(Receipt_relate_question_document_context_schema_properties_entryType(), {ReceiptLeaf(ReceiptMember(value, "name"), LogicalType::VARCHAR), Receipt_input_property_typeValue(ReceiptMember(value, "value"))});
}
inline LogicalType Receipt_relate_question_document_context_schemaType() {
return LogicalType::STRUCT({{"properties", LogicalType::LIST(Receipt_relate_question_document_context_schema_properties_entryType())}, {"required", LogicalType::LIST(LogicalType::VARCHAR)}, {"type", LogicalType::VARCHAR}});
}
inline Value Receipt_relate_question_document_context_schemaValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_relate_question_document_context_schemaType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_relate_question_document_context_schemaType()); }
return Value::STRUCT(Receipt_relate_question_document_context_schemaType(), {ReceiptMapping(ReceiptMember(value, "properties"), Receipt_relate_question_document_context_schema_properties_entryType(), [](const string &key, ReceiptJSON *item) { return Value::STRUCT(Receipt_relate_question_document_context_schema_properties_entryType(), {Value(key), Receipt_input_property_typeValue(item)}); }), ReceiptList(ReceiptMember(value, "required"), LogicalType::VARCHAR, [](ReceiptJSON *item) { return ReceiptLeaf(item, LogicalType::VARCHAR); }), ReceiptLeaf(ReceiptMember(value, "type"), LogicalType::VARCHAR)});
}
inline LogicalType Receipt_relate_question_document_fieldsType() {
return LogicalType::STRUCT({{"kind", LogicalType::VARCHAR}, {"name", LogicalType::VARCHAR}});
}
inline Value Receipt_relate_question_document_fieldsValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_relate_question_document_fieldsType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_relate_question_document_fieldsType()); }
return Value::STRUCT(Receipt_relate_question_document_fieldsType(), {ReceiptLeaf(ReceiptMember(value, "kind"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "name"), LogicalType::VARCHAR)});
}
inline LogicalType Receipt_relate_question_document_item_schema_properties_entryType() {
return LogicalType::STRUCT({{"name", LogicalType::VARCHAR}, {"value", Receipt_input_property_typeType()}});
}
inline Value Receipt_relate_question_document_item_schema_properties_entryValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_relate_question_document_item_schema_properties_entryType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_relate_question_document_item_schema_properties_entryType()); }
return Value::STRUCT(Receipt_relate_question_document_item_schema_properties_entryType(), {ReceiptLeaf(ReceiptMember(value, "name"), LogicalType::VARCHAR), Receipt_input_property_typeValue(ReceiptMember(value, "value"))});
}
inline LogicalType Receipt_relate_question_document_item_schemaType() {
return LogicalType::STRUCT({{"properties", LogicalType::LIST(Receipt_relate_question_document_item_schema_properties_entryType())}, {"required", LogicalType::LIST(LogicalType::VARCHAR)}, {"type", LogicalType::VARCHAR}});
}
inline Value Receipt_relate_question_document_item_schemaValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_relate_question_document_item_schemaType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_relate_question_document_item_schemaType()); }
return Value::STRUCT(Receipt_relate_question_document_item_schemaType(), {ReceiptMapping(ReceiptMember(value, "properties"), Receipt_relate_question_document_item_schema_properties_entryType(), [](const string &key, ReceiptJSON *item) { return Value::STRUCT(Receipt_relate_question_document_item_schema_properties_entryType(), {Value(key), Receipt_input_property_typeValue(item)}); }), ReceiptList(ReceiptMember(value, "required"), LogicalType::VARCHAR, [](ReceiptJSON *item) { return ReceiptLeaf(item, LogicalType::VARCHAR); }), ReceiptLeaf(ReceiptMember(value, "type"), LogicalType::VARCHAR)});
}
inline LogicalType Receipt_relate_question_documentType() {
return LogicalType::STRUCT({{"batch", Receipt_relate_question_document_batchType()}, {"context_schema", Receipt_relate_question_document_context_schemaType()}, {"fields", Receipt_relate_question_document_fieldsType()}, {"item_schema", Receipt_relate_question_document_item_schemaType()}, {"label_details", LogicalType::LIST(Receipt_labelType())}, {"model", LogicalType::VARCHAR}, {"name", LogicalType::VARCHAR}, {"on", LogicalType::LIST(LogicalType::VARCHAR)}, {"profile", LogicalType::VARCHAR}, {"relations", LogicalType::LIST(Receipt_relation_ruleType())}, {"threshold", Receipt_thresholdType()}, {"verb", LogicalType::VARCHAR}, {"wording_version", LogicalType::BIGINT}});
}
inline Value Receipt_relate_question_documentValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_relate_question_documentType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_relate_question_documentType()); }
return Value::STRUCT(Receipt_relate_question_documentType(), {Receipt_relate_question_document_batchValue(ReceiptMember(value, "batch")), Receipt_relate_question_document_context_schemaValue(ReceiptMember(value, "context_schema")), Receipt_relate_question_document_fieldsValue(ReceiptMember(value, "fields")), Receipt_relate_question_document_item_schemaValue(ReceiptMember(value, "item_schema")), ReceiptList(ReceiptMember(value, "label_details"), Receipt_labelType(), [](ReceiptJSON *item) { return Receipt_labelValue(item); }), ReceiptLeaf(ReceiptMember(value, "model"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "name"), LogicalType::VARCHAR), ReceiptList(ReceiptMember(value, "on"), LogicalType::VARCHAR, [](ReceiptJSON *item) { return ReceiptLeaf(item, LogicalType::VARCHAR); }), ReceiptLeaf(ReceiptMember(value, "profile"), LogicalType::VARCHAR), ReceiptList(ReceiptMember(value, "relations"), Receipt_relation_ruleType(), [](ReceiptJSON *item) { return Receipt_relation_ruleValue(item); }), Receipt_thresholdValue(ReceiptMember(value, "threshold")), ReceiptLeaf(ReceiptMember(value, "verb"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "wording_version"), LogicalType::BIGINT)});
}
inline LogicalType Receipt_related_entity_edge_sourceType() {
return LogicalType::STRUCT({{"file", LogicalType::VARCHAR}, {"first_line", LogicalType::UBIGINT}, {"kind", LogicalType::VARCHAR}, {"last_line", LogicalType::UBIGINT}, {"name", LogicalType::VARCHAR}, {"ordinal", LogicalType::UBIGINT}, {"record", LogicalType::JSON()}, {"record_present", LogicalType::BOOLEAN}});
}
inline Value Receipt_related_entity_edge_sourceValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_related_entity_edge_sourceType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_related_entity_edge_sourceType()); }
return Value::STRUCT(Receipt_related_entity_edge_sourceType(), {ReceiptLeaf(ReceiptMember(value, "file"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "first_line"), LogicalType::UBIGINT), ReceiptLeaf(ReceiptMember(value, "kind"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "last_line"), LogicalType::UBIGINT), ReceiptLeaf(ReceiptMember(value, "name"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "ordinal"), LogicalType::UBIGINT), ReceiptLeaf(ReceiptMember(value, "record"), LogicalType::JSON()), Value::BOOLEAN(ReceiptMember(value, "record") != nullptr)});
}
inline LogicalType Receipt_related_entity_edge_targetType() {
return LogicalType::STRUCT({{"file", LogicalType::VARCHAR}, {"first_line", LogicalType::UBIGINT}, {"kind", LogicalType::VARCHAR}, {"last_line", LogicalType::UBIGINT}, {"name", LogicalType::VARCHAR}, {"ordinal", LogicalType::UBIGINT}, {"record", LogicalType::JSON()}, {"record_present", LogicalType::BOOLEAN}});
}
inline Value Receipt_related_entity_edge_targetValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_related_entity_edge_targetType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_related_entity_edge_targetType()); }
return Value::STRUCT(Receipt_related_entity_edge_targetType(), {ReceiptLeaf(ReceiptMember(value, "file"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "first_line"), LogicalType::UBIGINT), ReceiptLeaf(ReceiptMember(value, "kind"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "last_line"), LogicalType::UBIGINT), ReceiptLeaf(ReceiptMember(value, "name"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "ordinal"), LogicalType::UBIGINT), ReceiptLeaf(ReceiptMember(value, "record"), LogicalType::JSON()), Value::BOOLEAN(ReceiptMember(value, "record") != nullptr)});
}
inline LogicalType Receipt_related_entity_edgeType() {
return LogicalType::STRUCT({{"either", LogicalType::BOOLEAN}, {"probability", LogicalType::DOUBLE}, {"relation", LogicalType::VARCHAR}, {"source", Receipt_related_entity_edge_sourceType()}, {"target", Receipt_related_entity_edge_targetType()}});
}
inline Value Receipt_related_entity_edgeValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_related_entity_edgeType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_related_entity_edgeType()); }
return Value::STRUCT(Receipt_related_entity_edgeType(), {ReceiptLeaf(ReceiptMember(value, "either"), LogicalType::BOOLEAN), ReceiptLeaf(ReceiptMember(value, "probability"), LogicalType::DOUBLE), ReceiptLeaf(ReceiptMember(value, "relation"), LogicalType::VARCHAR), Receipt_related_entity_edge_sourceValue(ReceiptMember(value, "source")), Receipt_related_entity_edge_targetValue(ReceiptMember(value, "target"))});
}
inline LogicalType Receipt_relatedType() {
return LogicalType::STRUCT({{"answer", Receipt_answersType()}, {"answer_id", LogicalType::VARCHAR}, {"file", LogicalType::VARCHAR}, {"first_line", LogicalType::UBIGINT}, {"index", LogicalType::UBIGINT}, {"input", LogicalType::JSON()}, {"input_present", LogicalType::BOOLEAN}, {"input_sources", LogicalType::LIST(Receipt_session_input_sourceType())}, {"last_line", LogicalType::UBIGINT}, {"meta", Receipt_metadataType()}, {"position", Receipt_positionType()}, {"question", Receipt_relate_question_documentType()}, {"schema", LogicalType::VARCHAR}, {"value", LogicalType::LIST(Receipt_related_entity_edgeType())}});
}
inline Value Receipt_relatedValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_relatedType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_relatedType()); }
return Value::STRUCT(Receipt_relatedType(), {Receipt_answersValue(ReceiptMember(value, "answer")), ReceiptLeaf(ReceiptMember(value, "answer_id"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "file"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "first_line"), LogicalType::UBIGINT), ReceiptLeaf(ReceiptMember(value, "index"), LogicalType::UBIGINT), ReceiptLeaf(ReceiptMember(value, "input"), LogicalType::JSON()), Value::BOOLEAN(ReceiptMember(value, "input") != nullptr), ReceiptList(ReceiptMember(value, "input_sources"), Receipt_session_input_sourceType(), [](ReceiptJSON *item) { return Receipt_session_input_sourceValue(item); }), ReceiptLeaf(ReceiptMember(value, "last_line"), LogicalType::UBIGINT), Receipt_metadataValue(ReceiptMember(value, "meta")), Receipt_positionValue(ReceiptMember(value, "position")), Receipt_relate_question_documentValue(ReceiptMember(value, "question")), ReceiptLeaf(ReceiptMember(value, "schema"), LogicalType::VARCHAR), ReceiptList(ReceiptMember(value, "value"), Receipt_related_entity_edgeType(), [](ReceiptJSON *item) { return Receipt_related_entity_edgeValue(item); })});
}
inline LogicalType Receipt_facts_usage_persistenceType() {
return LogicalType::STRUCT({{"advice", LogicalType::VARCHAR}, {"observed_at", LogicalType::VARCHAR}, {"state", LogicalType::VARCHAR}});
}
inline Value Receipt_facts_usage_persistenceValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_facts_usage_persistenceType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_facts_usage_persistenceType()); }
return Value::STRUCT(Receipt_facts_usage_persistenceType(), {ReceiptLeaf(ReceiptMember(value, "advice"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "observed_at"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "state"), LogicalType::VARCHAR)});
}
inline LogicalType Receipt_factsType() {
return LogicalType::STRUCT({{"attempts", LogicalType::LIST(Receipt_attemptType())}, {"cache_answers", LogicalType::UBIGINT}, {"call_id", LogicalType::VARCHAR}, {"estimated_cost_usd", LogicalType::VARCHAR}, {"held_model_mismatch", LogicalType::BOOLEAN}, {"input_tokens", LogicalType::UBIGINT}, {"largest_request_bytes", LogicalType::UBIGINT}, {"largest_request_estimated_input_tokens", LogicalType::UBIGINT}, {"model", LogicalType::VARCHAR}, {"output_tokens", LogicalType::UBIGINT}, {"records", LogicalType::UBIGINT}, {"requests_sent", LogicalType::UBIGINT}, {"seconds", LogicalType::DOUBLE}, {"token_estimate_method", LogicalType::VARCHAR}, {"usage_persistence", Receipt_facts_usage_persistenceType()}});
}
inline Value Receipt_factsValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_factsType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_factsType()); }
return Value::STRUCT(Receipt_factsType(), {ReceiptList(ReceiptMember(value, "attempts"), Receipt_attemptType(), [](ReceiptJSON *item) { return Receipt_attemptValue(item); }), ReceiptLeaf(ReceiptMember(value, "cache_answers"), LogicalType::UBIGINT), ReceiptLeaf(ReceiptMember(value, "call_id"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "estimated_cost_usd"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "held_model_mismatch"), LogicalType::BOOLEAN), ReceiptLeaf(ReceiptMember(value, "input_tokens"), LogicalType::UBIGINT), ReceiptLeaf(ReceiptMember(value, "largest_request_bytes"), LogicalType::UBIGINT), ReceiptLeaf(ReceiptMember(value, "largest_request_estimated_input_tokens"), LogicalType::UBIGINT), ReceiptLeaf(ReceiptMember(value, "model"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "output_tokens"), LogicalType::UBIGINT), ReceiptLeaf(ReceiptMember(value, "records"), LogicalType::UBIGINT), ReceiptLeaf(ReceiptMember(value, "requests_sent"), LogicalType::UBIGINT), ReceiptLeaf(ReceiptMember(value, "seconds"), LogicalType::DOUBLE), ReceiptLeaf(ReceiptMember(value, "token_estimate_method"), LogicalType::VARCHAR), Receipt_facts_usage_persistenceValue(ReceiptMember(value, "usage_persistence"))});
}
inline LogicalType Receipt_token_bandType() {
return LogicalType::STRUCT({{"lower", LogicalType::UBIGINT}, {"upper", LogicalType::UBIGINT}});
}
inline Value Receipt_token_bandValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_token_bandType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_token_bandType()); }
return Value::STRUCT(Receipt_token_bandType(), {ReceiptLeaf(ReceiptMember(value, "lower"), LogicalType::UBIGINT), ReceiptLeaf(ReceiptMember(value, "upper"), LogicalType::UBIGINT)});
}
inline LogicalType Receipt_planType() {
return LogicalType::STRUCT({{"estimated_bytes", LogicalType::UBIGINT}, {"estimated_input_tokens", Receipt_token_bandType()}, {"first_body_utf8", LogicalType::VARCHAR}, {"largest_request_bytes", LogicalType::UBIGINT}, {"largest_request_estimated_input_tokens", LogicalType::UBIGINT}, {"records", LogicalType::UBIGINT}, {"requests", LogicalType::UBIGINT}, {"token_estimate_method", LogicalType::VARCHAR}, {"upper_bound", LogicalType::BOOLEAN}});
}
inline Value Receipt_planValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_planType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_planType()); }
return Value::STRUCT(Receipt_planType(), {ReceiptLeaf(ReceiptMember(value, "estimated_bytes"), LogicalType::UBIGINT), Receipt_token_bandValue(ReceiptMember(value, "estimated_input_tokens")), ReceiptLeaf(ReceiptMember(value, "first_body_utf8"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "largest_request_bytes"), LogicalType::UBIGINT), ReceiptLeaf(ReceiptMember(value, "largest_request_estimated_input_tokens"), LogicalType::UBIGINT), ReceiptLeaf(ReceiptMember(value, "records"), LogicalType::UBIGINT), ReceiptLeaf(ReceiptMember(value, "requests"), LogicalType::UBIGINT), ReceiptLeaf(ReceiptMember(value, "token_estimate_method"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "upper_bound"), LogicalType::BOOLEAN)});
}
inline LogicalType Receipt_error_estimated_input_denialType() {
return LogicalType::STRUCT({{"kind", LogicalType::VARCHAR}, {"last_status", LogicalType::BIGINT}, {"limit", LogicalType::UBIGINT}});
}
inline Value Receipt_error_estimated_input_denialValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_error_estimated_input_denialType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_error_estimated_input_denialType()); }
return Value::STRUCT(Receipt_error_estimated_input_denialType(), {ReceiptLeaf(ReceiptMember(value, "kind"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "last_status"), LogicalType::BIGINT), ReceiptLeaf(ReceiptMember(value, "limit"), LogicalType::UBIGINT)});
}
inline LogicalType Receipt_error_send_budget_denialType() {
return LogicalType::STRUCT({{"kind", LogicalType::VARCHAR}, {"last_status", LogicalType::BIGINT}});
}
inline Value Receipt_error_send_budget_denialValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_error_send_budget_denialType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_error_send_budget_denialType()); }
return Value::STRUCT(Receipt_error_send_budget_denialType(), {ReceiptLeaf(ReceiptMember(value, "kind"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "last_status"), LogicalType::BIGINT)});
}
inline LogicalType Receipt_stoppedType() {
return LogicalType::STRUCT({{"at", LogicalType::UBIGINT}, {"cause", LogicalType::VARCHAR}, {"retryable", LogicalType::BOOLEAN}, {"status", LogicalType::BIGINT}});
}
inline Value Receipt_stoppedValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_stoppedType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_stoppedType()); }
return Value::STRUCT(Receipt_stoppedType(), {ReceiptLeaf(ReceiptMember(value, "at"), LogicalType::UBIGINT), ReceiptLeaf(ReceiptMember(value, "cause"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "retryable"), LogicalType::BOOLEAN), ReceiptLeaf(ReceiptMember(value, "status"), LogicalType::BIGINT)});
}
inline LogicalType Receipt_errorType() {
return LogicalType::STRUCT({{"estimated_input_denial", Receipt_error_estimated_input_denialType()}, {"kind", LogicalType::VARCHAR}, {"message", LogicalType::VARCHAR}, {"retryable", LogicalType::BOOLEAN}, {"send_budget_denial", Receipt_error_send_budget_denialType()}, {"stopped", Receipt_stoppedType()}});
}
inline Value Receipt_errorValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_errorType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_errorType()); }
return Value::STRUCT(Receipt_errorType(), {Receipt_error_estimated_input_denialValue(ReceiptMember(value, "estimated_input_denial")), ReceiptLeaf(ReceiptMember(value, "kind"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "message"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "retryable"), LogicalType::BOOLEAN), Receipt_error_send_budget_denialValue(ReceiptMember(value, "send_budget_denial")), Receipt_stoppedValue(ReceiptMember(value, "stopped"))});
}
inline LogicalType Receipt_session_observation_authorType() {
return LogicalType::STRUCT({{"name", LogicalType::VARCHAR}, {"wording_version", LogicalType::VARCHAR}});
}
inline Value Receipt_session_observation_authorValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_session_observation_authorType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_session_observation_authorType()); }
return Value::STRUCT(Receipt_session_observation_authorType(), {ReceiptLeaf(ReceiptMember(value, "name"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "wording_version"), LogicalType::VARCHAR)});
}
inline LogicalType Receipt_session_question_detail_failureType() {
return LogicalType::STRUCT({{"cause", LogicalType::VARCHAR}, {"kind", LogicalType::VARCHAR}});
}
inline Value Receipt_session_question_detail_failureValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_session_question_detail_failureType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_session_question_detail_failureType()); }
return Value::STRUCT(Receipt_session_question_detail_failureType(), {ReceiptLeaf(ReceiptMember(value, "cause"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "kind"), LogicalType::VARCHAR)});
}
inline LogicalType Receipt_session_question_detail_input_sourceType() {
return LogicalType::STRUCT({{"file", LogicalType::VARCHAR}, {"first_line", LogicalType::UBIGINT}, {"last_line", LogicalType::UBIGINT}});
}
inline Value Receipt_session_question_detail_input_sourceValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_session_question_detail_input_sourceType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_session_question_detail_input_sourceType()); }
return Value::STRUCT(Receipt_session_question_detail_input_sourceType(), {ReceiptLeaf(ReceiptMember(value, "file"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "first_line"), LogicalType::UBIGINT), ReceiptLeaf(ReceiptMember(value, "last_line"), LogicalType::UBIGINT)});
}
inline LogicalType Receipt_session_named_probabilityType() {
return LogicalType::STRUCT({{"name", LogicalType::VARCHAR}, {"probability", LogicalType::DOUBLE}});
}
inline Value Receipt_session_named_probabilityValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_session_named_probabilityType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_session_named_probabilityType()); }
return Value::STRUCT(Receipt_session_named_probabilityType(), {ReceiptLeaf(ReceiptMember(value, "name"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "probability"), LogicalType::DOUBLE)});
}
inline LogicalType Receipt_session_question_detail_probabilities_valueType() {
return LogicalType::STRUCT({{"array_value", LogicalType::LIST(Receipt_session_named_probabilityType())}, {"number_value", LogicalType::DOUBLE}});
}
inline Value Receipt_session_question_detail_probabilities_valueValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_session_question_detail_probabilities_valueType()); }
return Value::STRUCT(Receipt_session_question_detail_probabilities_valueType(), {ReceiptList(value, Receipt_session_named_probabilityType(), [](ReceiptJSON *item) { return Receipt_session_named_probabilityValue(item); }), ReceiptLeaf(value, LogicalType::DOUBLE)});
}
inline LogicalType Receipt_session_question_detail_probabilitiesType() {
return LogicalType::STRUCT({{"kind", LogicalType::VARCHAR}, {"value", Receipt_session_question_detail_probabilities_valueType()}});
}
inline Value Receipt_session_question_detail_probabilitiesValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_session_question_detail_probabilitiesType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_session_question_detail_probabilitiesType()); }
return Value::STRUCT(Receipt_session_question_detail_probabilitiesType(), {ReceiptLeaf(ReceiptMember(value, "kind"), LogicalType::VARCHAR), Receipt_session_question_detail_probabilities_valueValue(ReceiptMember(value, "value"))});
}
inline LogicalType Receipt_session_question_detail_reported_usageType() {
return LogicalType::STRUCT({{"input_tokens", LogicalType::UBIGINT}, {"output_tokens", LogicalType::UBIGINT}});
}
inline Value Receipt_session_question_detail_reported_usageValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_session_question_detail_reported_usageType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_session_question_detail_reported_usageType()); }
return Value::STRUCT(Receipt_session_question_detail_reported_usageType(), {ReceiptLeaf(ReceiptMember(value, "input_tokens"), LogicalType::UBIGINT), ReceiptLeaf(ReceiptMember(value, "output_tokens"), LogicalType::UBIGINT)});
}
inline LogicalType Receipt_session_question_detail_thresholdType() {
return LogicalType::STRUCT({{"number_value", LogicalType::DOUBLE}, {"string_value", LogicalType::VARCHAR}});
}
inline Value Receipt_session_question_detail_thresholdValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_session_question_detail_thresholdType()); }
return Value::STRUCT(Receipt_session_question_detail_thresholdType(), {ReceiptLeaf(value, LogicalType::DOUBLE), ReceiptLeaf(value, LogicalType::VARCHAR)});
}
inline LogicalType Receipt_session_question_detail_usageType() {
return LogicalType::STRUCT({{"input_tokens", LogicalType::UBIGINT}, {"output_tokens", LogicalType::UBIGINT}});
}
inline Value Receipt_session_question_detail_usageValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_session_question_detail_usageType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_session_question_detail_usageType()); }
return Value::STRUCT(Receipt_session_question_detail_usageType(), {ReceiptLeaf(ReceiptMember(value, "input_tokens"), LogicalType::UBIGINT), ReceiptLeaf(ReceiptMember(value, "output_tokens"), LogicalType::UBIGINT)});
}
inline LogicalType Receipt_session_question_detail_valueType() {
return LogicalType::STRUCT({{"array_value", LogicalType::LIST(LogicalType::VARCHAR)}, {"boolean_value", LogicalType::BOOLEAN}, {"number_value", LogicalType::DOUBLE}, {"string_value", LogicalType::VARCHAR}});
}
inline Value Receipt_session_question_detail_valueValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_session_question_detail_valueType()); }
return Value::STRUCT(Receipt_session_question_detail_valueType(), {ReceiptList(value, LogicalType::VARCHAR, [](ReceiptJSON *item) { return ReceiptLeaf(item, LogicalType::VARCHAR); }), ReceiptLeaf(value, LogicalType::BOOLEAN), ReceiptLeaf(value, LogicalType::DOUBLE), ReceiptLeaf(value, LogicalType::VARCHAR)});
}
inline LogicalType Receipt_question_detailType() {
return LogicalType::STRUCT({{"answer_id", LogicalType::VARCHAR}, {"cached", LogicalType::BOOLEAN}, {"confidence", LogicalType::DOUBLE}, {"failed_questions", LogicalType::UBIGINT}, {"failure", Receipt_session_question_detail_failureType()}, {"failure_id", LogicalType::VARCHAR}, {"input", LogicalType::JSON()}, {"input_present", LogicalType::BOOLEAN}, {"input_source", Receipt_session_question_detail_input_sourceType()}, {"input_sources", LogicalType::LIST(Receipt_session_input_sourceType())}, {"inputs", LogicalType::LIST(LogicalType::JSON())}, {"model", LogicalType::VARCHAR}, {"observations", LogicalType::LIST(Receipt_observationType())}, {"probabilities", Receipt_session_question_detail_probabilitiesType()}, {"question", Receipt_question_documentType()}, {"question_sha256", LogicalType::VARCHAR}, {"question_sources", LogicalType::LIST(Receipt_question_sourceType())}, {"raw_pick", LogicalType::VARCHAR}, {"reported_usage", Receipt_session_question_detail_reported_usageType()}, {"requests", LogicalType::LIST(LogicalType::VARCHAR)}, {"requests_sent", LogicalType::UBIGINT}, {"threshold", Receipt_session_question_detail_thresholdType()}, {"url", LogicalType::VARCHAR}, {"usage", Receipt_session_question_detail_usageType()}, {"value", Receipt_session_question_detail_valueType()}});
}
inline Value Receipt_question_detailValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_question_detailType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_question_detailType()); }
return Value::STRUCT(Receipt_question_detailType(), {ReceiptLeaf(ReceiptMember(value, "answer_id"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "cached"), LogicalType::BOOLEAN), ReceiptLeaf(ReceiptMember(value, "confidence"), LogicalType::DOUBLE), ReceiptLeaf(ReceiptMember(value, "failed_questions"), LogicalType::UBIGINT), Receipt_session_question_detail_failureValue(ReceiptMember(value, "failure")), ReceiptLeaf(ReceiptMember(value, "failure_id"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "input"), LogicalType::JSON()), Value::BOOLEAN(ReceiptMember(value, "input") != nullptr), Receipt_session_question_detail_input_sourceValue(ReceiptMember(value, "input_source")), ReceiptList(ReceiptMember(value, "input_sources"), Receipt_session_input_sourceType(), [](ReceiptJSON *item) { return Receipt_session_input_sourceValue(item); }), ReceiptList(ReceiptMember(value, "inputs"), LogicalType::JSON(), [](ReceiptJSON *item) { return ReceiptLeaf(item, LogicalType::JSON()); }), ReceiptLeaf(ReceiptMember(value, "model"), LogicalType::VARCHAR), ReceiptList(ReceiptMember(value, "observations"), Receipt_observationType(), [](ReceiptJSON *item) { return Receipt_observationValue(item); }), Receipt_session_question_detail_probabilitiesValue(ReceiptMember(value, "probabilities")), Receipt_question_documentValue(ReceiptMember(value, "question")), ReceiptLeaf(ReceiptMember(value, "question_sha256"), LogicalType::VARCHAR), ReceiptList(ReceiptMember(value, "question_sources"), Receipt_question_sourceType(), [](ReceiptJSON *item) { return Receipt_question_sourceValue(item); }), ReceiptLeaf(ReceiptMember(value, "raw_pick"), LogicalType::VARCHAR), Receipt_session_question_detail_reported_usageValue(ReceiptMember(value, "reported_usage")), ReceiptList(ReceiptMember(value, "requests"), LogicalType::VARCHAR, [](ReceiptJSON *item) { return ReceiptLeaf(item, LogicalType::VARCHAR); }), ReceiptLeaf(ReceiptMember(value, "requests_sent"), LogicalType::UBIGINT), Receipt_session_question_detail_thresholdValue(ReceiptMember(value, "threshold")), ReceiptLeaf(ReceiptMember(value, "url"), LogicalType::VARCHAR), Receipt_session_question_detail_usageValue(ReceiptMember(value, "usage")), Receipt_session_question_detail_valueValue(ReceiptMember(value, "value"))});
}
inline LogicalType Receipt_observed_inputType() {
return LogicalType::STRUCT({{"images", LogicalType::LIST(Receipt_imageType())}, {"input", LogicalType::JSON()}, {"input_present", LogicalType::BOOLEAN}, {"source", Receipt_physical_sourceType()}});
}
inline Value Receipt_observed_inputValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_observed_inputType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_observed_inputType()); }
return Value::STRUCT(Receipt_observed_inputType(), {ReceiptList(ReceiptMember(value, "images"), Receipt_imageType(), [](ReceiptJSON *item) { return Receipt_imageValue(item); }), ReceiptLeaf(ReceiptMember(value, "input"), LogicalType::JSON()), Value::BOOLEAN(ReceiptMember(value, "input") != nullptr), Receipt_physical_sourceValue(ReceiptMember(value, "source"))});
}
inline LogicalType Receipt_entity_documentType() {
return LogicalType::STRUCT({{"kind", LogicalType::VARCHAR}, {"name", LogicalType::VARCHAR}});
}
inline Value Receipt_entity_documentValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_entity_documentType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_entity_documentType()); }
return Value::STRUCT(Receipt_entity_documentType(), {ReceiptLeaf(ReceiptMember(value, "kind"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "name"), LogicalType::VARCHAR)});
}
inline LogicalType Receipt_annotation_value_value_object_valueType() {
return LogicalType::STRUCT({{"cause", LogicalType::VARCHAR}, {"kind", LogicalType::VARCHAR}});
}
inline Value Receipt_annotation_value_value_object_valueValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_annotation_value_value_object_valueType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_annotation_value_value_object_valueType()); }
return Value::STRUCT(Receipt_annotation_value_value_object_valueType(), {ReceiptLeaf(ReceiptMember(value, "cause"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "kind"), LogicalType::VARCHAR)});
}
inline LogicalType Receipt_annotation_value_valueType() {
return LogicalType::STRUCT({{"array_value", LogicalType::LIST(LogicalType::VARCHAR)}, {"boolean_value", LogicalType::BOOLEAN}, {"number_value", LogicalType::DOUBLE}, {"object_value", Receipt_annotation_value_value_object_valueType()}, {"string_value", LogicalType::VARCHAR}});
}
inline Value Receipt_annotation_value_valueValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_annotation_value_valueType()); }
return Value::STRUCT(Receipt_annotation_value_valueType(), {ReceiptList(value, LogicalType::VARCHAR, [](ReceiptJSON *item) { return ReceiptLeaf(item, LogicalType::VARCHAR); }), ReceiptLeaf(value, LogicalType::BOOLEAN), ReceiptLeaf(value, LogicalType::DOUBLE), Receipt_annotation_value_value_object_valueValue(value), ReceiptLeaf(value, LogicalType::VARCHAR)});
}
inline LogicalType Receipt_annotation_valueType() {
return LogicalType::STRUCT({{"kind", LogicalType::VARCHAR}, {"value", Receipt_annotation_value_valueType()}});
}
inline Value Receipt_annotation_valueValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_annotation_valueType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_annotation_valueType()); }
return Value::STRUCT(Receipt_annotation_valueType(), {ReceiptLeaf(ReceiptMember(value, "kind"), LogicalType::VARCHAR), Receipt_annotation_value_valueValue(ReceiptMember(value, "value"))});
}
inline LogicalType Receipt_session_observed_row_value_array_value_itemType() {
return LogicalType::STRUCT({{"either", LogicalType::BOOLEAN}, {"name", LogicalType::VARCHAR}, {"probability", LogicalType::DOUBLE}, {"relation", LogicalType::VARCHAR}, {"source", Receipt_entity_documentType()}, {"target", Receipt_entity_documentType()}, {"value", Receipt_annotation_valueType()}});
}
inline Value Receipt_session_observed_row_value_array_value_itemValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_session_observed_row_value_array_value_itemType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_session_observed_row_value_array_value_itemType()); }
return Value::STRUCT(Receipt_session_observed_row_value_array_value_itemType(), {ReceiptLeaf(ReceiptMember(value, "either"), LogicalType::BOOLEAN), ReceiptLeaf(ReceiptMember(value, "name"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "probability"), LogicalType::DOUBLE), ReceiptLeaf(ReceiptMember(value, "relation"), LogicalType::VARCHAR), Receipt_entity_documentValue(ReceiptMember(value, "source")), Receipt_entity_documentValue(ReceiptMember(value, "target")), Receipt_annotation_valueValue(ReceiptMember(value, "value"))});
}
inline LogicalType Receipt_recognition_edge_documentType() {
return LogicalType::STRUCT({{"either", LogicalType::BOOLEAN}, {"probability", LogicalType::DOUBLE}, {"relation", LogicalType::VARCHAR}, {"source", Receipt_entityType()}, {"target", Receipt_entityType()}});
}
inline Value Receipt_recognition_edge_documentValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_recognition_edge_documentType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_recognition_edge_documentType()); }
return Value::STRUCT(Receipt_recognition_edge_documentType(), {ReceiptLeaf(ReceiptMember(value, "either"), LogicalType::BOOLEAN), ReceiptLeaf(ReceiptMember(value, "probability"), LogicalType::DOUBLE), ReceiptLeaf(ReceiptMember(value, "relation"), LogicalType::VARCHAR), Receipt_entityValue(ReceiptMember(value, "source")), Receipt_entityValue(ReceiptMember(value, "target"))});
}
inline LogicalType Receipt_session_observed_row_value_object_value_valueType() {
return LogicalType::STRUCT({{"array_value", LogicalType::LIST(LogicalType::VARCHAR)}, {"boolean_value", LogicalType::BOOLEAN}, {"number_value", LogicalType::DOUBLE}, {"string_value", LogicalType::VARCHAR}});
}
inline Value Receipt_session_observed_row_value_object_value_valueValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_session_observed_row_value_object_value_valueType()); }
return Value::STRUCT(Receipt_session_observed_row_value_object_value_valueType(), {ReceiptList(value, LogicalType::VARCHAR, [](ReceiptJSON *item) { return ReceiptLeaf(item, LogicalType::VARCHAR); }), ReceiptLeaf(value, LogicalType::BOOLEAN), ReceiptLeaf(value, LogicalType::DOUBLE), ReceiptLeaf(value, LogicalType::VARCHAR)});
}
