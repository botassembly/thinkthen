// Generated from the canonical native result schema; do not edit.
#pragma once
inline LogicalType Receipt_answer_probabilities_entryType() {
return LogicalType::STRUCT({{"name", LogicalType::VARCHAR}, {"value", LogicalType::DOUBLE}});
}
inline Value Receipt_answer_probabilities_entryValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_answer_probabilities_entryType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_answer_probabilities_entryType()); }
return Value::STRUCT(Receipt_answer_probabilities_entryType(), {ReceiptLeaf(ReceiptMember(value, "name"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "value"), LogicalType::DOUBLE)});
}
inline LogicalType Receipt_answerType() {
return LogicalType::STRUCT({{"confidence", LogicalType::DOUBLE}, {"kind", LogicalType::VARCHAR}, {"level", LogicalType::VARCHAR}, {"pick", LogicalType::VARCHAR}, {"probabilities", LogicalType::LIST(Receipt_answer_probabilities_entryType())}, {"probability", LogicalType::DOUBLE}});
}
inline Value Receipt_answerValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_answerType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_answerType()); }
return Value::STRUCT(Receipt_answerType(), {ReceiptLeaf(ReceiptMember(value, "confidence"), LogicalType::DOUBLE), ReceiptLeaf(ReceiptMember(value, "kind"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "level"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "pick"), LogicalType::VARCHAR), ReceiptMapping(ReceiptMember(value, "probabilities"), Receipt_answer_probabilities_entryType(), [](const string &key, ReceiptJSON *item) { return Value::STRUCT(Receipt_answer_probabilities_entryType(), {Value(key), ReceiptLeaf(item, LogicalType::DOUBLE)}); }), ReceiptLeaf(ReceiptMember(value, "probability"), LogicalType::DOUBLE)});
}
inline LogicalType Receipt_imageType() {
return LogicalType::STRUCT({{"base64", LogicalType::VARCHAR}, {"height", LogicalType::BIGINT}, {"media", LogicalType::VARCHAR}, {"width", LogicalType::BIGINT}});
}
inline Value Receipt_imageValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_imageType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_imageType()); }
return Value::STRUCT(Receipt_imageType(), {ReceiptLeaf(ReceiptMember(value, "base64"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "height"), LogicalType::BIGINT), ReceiptLeaf(ReceiptMember(value, "media"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "width"), LogicalType::BIGINT)});
}
inline LogicalType Receipt_attemptType() {
return LogicalType::STRUCT({{"ordinal", LogicalType::UBIGINT}, {"outcome", LogicalType::VARCHAR}, {"request_id", LogicalType::VARCHAR}, {"request_sha256", LogicalType::VARCHAR}, {"sdk_request_id", LogicalType::VARCHAR}, {"server_ms", LogicalType::UBIGINT}, {"status", LogicalType::BIGINT}, {"wall_ms", LogicalType::UBIGINT}});
}
inline Value Receipt_attemptValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_attemptType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_attemptType()); }
return Value::STRUCT(Receipt_attemptType(), {ReceiptLeaf(ReceiptMember(value, "ordinal"), LogicalType::UBIGINT), ReceiptLeaf(ReceiptMember(value, "outcome"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "request_id"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "request_sha256"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "sdk_request_id"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "server_ms"), LogicalType::UBIGINT), ReceiptLeaf(ReceiptMember(value, "status"), LogicalType::BIGINT), ReceiptLeaf(ReceiptMember(value, "wall_ms"), LogicalType::UBIGINT)});
}
inline LogicalType Receipt_meta_batch_settingType() {
return LogicalType::STRUCT({{"integer_value", LogicalType::UBIGINT}, {"string_value", LogicalType::VARCHAR}});
}
inline Value Receipt_meta_batch_settingValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_meta_batch_settingType()); }
return Value::STRUCT(Receipt_meta_batch_settingType(), {ReceiptLeaf(value, LogicalType::UBIGINT), ReceiptLeaf(value, LogicalType::VARCHAR)});
}
inline LogicalType Receipt_batch_settingType() {
return LogicalType::STRUCT({{"integer_value", LogicalType::UBIGINT}, {"string_value", LogicalType::VARCHAR}});
}
inline Value Receipt_batch_settingValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_batch_settingType()); }
return Value::STRUCT(Receipt_batch_settingType(), {ReceiptLeaf(value, LogicalType::UBIGINT), ReceiptLeaf(value, LogicalType::VARCHAR)});
}
inline LogicalType Receipt_meta_batch_warningType() {
return LogicalType::STRUCT({{"running", Receipt_batch_settingType()}, {"tuned_for", Receipt_batch_settingType()}});
}
inline Value Receipt_meta_batch_warningValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_meta_batch_warningType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_meta_batch_warningType()); }
return Value::STRUCT(Receipt_meta_batch_warningType(), {Receipt_batch_settingValue(ReceiptMember(value, "running")), Receipt_batch_settingValue(ReceiptMember(value, "tuned_for"))});
}
inline LogicalType Receipt_observationType() {
return LogicalType::STRUCT({{"failure_id", LogicalType::VARCHAR}, {"observation_id", LogicalType::VARCHAR}});
}
inline Value Receipt_observationValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_observationType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_observationType()); }
return Value::STRUCT(Receipt_observationType(), {ReceiptLeaf(ReceiptMember(value, "failure_id"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "observation_id"), LogicalType::VARCHAR)});
}
inline LogicalType Receipt_meta_profile_warningType() {
return LogicalType::STRUCT({{"running", LogicalType::VARCHAR}, {"tuned_for", LogicalType::VARCHAR}});
}
inline Value Receipt_meta_profile_warningValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_meta_profile_warningType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_meta_profile_warningType()); }
return Value::STRUCT(Receipt_meta_profile_warningType(), {ReceiptLeaf(ReceiptMember(value, "running"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "tuned_for"), LogicalType::VARCHAR)});
}
inline LogicalType Receipt_question_sourceType() {
return LogicalType::STRUCT({{"answered_by", LogicalType::VARCHAR}, {"batch_size", LogicalType::BIGINT}, {"origin", LogicalType::VARCHAR}});
}
inline Value Receipt_question_sourceValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_question_sourceType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_question_sourceType()); }
return Value::STRUCT(Receipt_question_sourceType(), {ReceiptLeaf(ReceiptMember(value, "answered_by"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "batch_size"), LogicalType::BIGINT), ReceiptLeaf(ReceiptMember(value, "origin"), LogicalType::VARCHAR)});
}
inline LogicalType Receipt_meta_usageType() {
return LogicalType::STRUCT({{"input_tokens", LogicalType::UBIGINT}, {"output_tokens", LogicalType::UBIGINT}});
}
inline Value Receipt_meta_usageValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_meta_usageType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_meta_usageType()); }
return Value::STRUCT(Receipt_meta_usageType(), {ReceiptLeaf(ReceiptMember(value, "input_tokens"), LogicalType::UBIGINT), ReceiptLeaf(ReceiptMember(value, "output_tokens"), LogicalType::UBIGINT)});
}
inline LogicalType Receipt_metadataType() {
return LogicalType::STRUCT({{"answered_by", LogicalType::VARCHAR}, {"attempts", LogicalType::LIST(Receipt_attemptType())}, {"batch_setting", Receipt_meta_batch_settingType()}, {"batch_warning", Receipt_meta_batch_warningType()}, {"cached", LogicalType::BOOLEAN}, {"context_sha256", LogicalType::VARCHAR}, {"failed_questions", LogicalType::UBIGINT}, {"model", LogicalType::VARCHAR}, {"observations", LogicalType::LIST(Receipt_observationType())}, {"origin", LogicalType::VARCHAR}, {"profile_warning", Receipt_meta_profile_warningType()}, {"question_sha256", LogicalType::VARCHAR}, {"question_sources", LogicalType::LIST(Receipt_question_sourceType())}, {"questions_sha256", LogicalType::VARCHAR}, {"requests", LogicalType::LIST(LogicalType::VARCHAR)}, {"requests_sent", LogicalType::UBIGINT}, {"tool", LogicalType::VARCHAR}, {"url", LogicalType::VARCHAR}, {"usage", Receipt_meta_usageType()}});
}
inline Value Receipt_metadataValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_metadataType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_metadataType()); }
return Value::STRUCT(Receipt_metadataType(), {ReceiptLeaf(ReceiptMember(value, "answered_by"), LogicalType::VARCHAR), ReceiptList(ReceiptMember(value, "attempts"), Receipt_attemptType(), [](ReceiptJSON *item) { return Receipt_attemptValue(item); }), Receipt_meta_batch_settingValue(ReceiptMember(value, "batch_setting")), Receipt_meta_batch_warningValue(ReceiptMember(value, "batch_warning")), ReceiptLeaf(ReceiptMember(value, "cached"), LogicalType::BOOLEAN), ReceiptLeaf(ReceiptMember(value, "context_sha256"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "failed_questions"), LogicalType::UBIGINT), ReceiptLeaf(ReceiptMember(value, "model"), LogicalType::VARCHAR), ReceiptList(ReceiptMember(value, "observations"), Receipt_observationType(), [](ReceiptJSON *item) { return Receipt_observationValue(item); }), ReceiptLeaf(ReceiptMember(value, "origin"), LogicalType::VARCHAR), Receipt_meta_profile_warningValue(ReceiptMember(value, "profile_warning")), ReceiptLeaf(ReceiptMember(value, "question_sha256"), LogicalType::VARCHAR), ReceiptList(ReceiptMember(value, "question_sources"), Receipt_question_sourceType(), [](ReceiptJSON *item) { return Receipt_question_sourceValue(item); }), ReceiptLeaf(ReceiptMember(value, "questions_sha256"), LogicalType::VARCHAR), ReceiptList(ReceiptMember(value, "requests"), LogicalType::VARCHAR, [](ReceiptJSON *item) { return ReceiptLeaf(item, LogicalType::VARCHAR); }), ReceiptLeaf(ReceiptMember(value, "requests_sent"), LogicalType::UBIGINT), ReceiptLeaf(ReceiptMember(value, "tool"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "url"), LogicalType::VARCHAR), Receipt_meta_usageValue(ReceiptMember(value, "usage"))});
}
inline LogicalType Receipt_positionType() {
return LogicalType::STRUCT({{"file", LogicalType::VARCHAR}, {"first", LogicalType::UBIGINT}, {"images", LogicalType::LIST(LogicalType::VARCHAR)}, {"last", LogicalType::UBIGINT}});
}
inline Value Receipt_positionValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_positionType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_positionType()); }
return Value::STRUCT(Receipt_positionType(), {ReceiptLeaf(ReceiptMember(value, "file"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "first"), LogicalType::UBIGINT), ReceiptList(ReceiptMember(value, "images"), LogicalType::VARCHAR, [](ReceiptJSON *item) { return ReceiptLeaf(item, LogicalType::VARCHAR); }), ReceiptLeaf(ReceiptMember(value, "last"), LogicalType::UBIGINT)});
}
inline LogicalType Receipt_question_document_batchType() {
return LogicalType::STRUCT({{"integer_value", LogicalType::UBIGINT}, {"string_value", LogicalType::VARCHAR}});
}
inline Value Receipt_question_document_batchValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_question_document_batchType()); }
return Value::STRUCT(Receipt_question_document_batchType(), {ReceiptLeaf(value, LogicalType::UBIGINT), ReceiptLeaf(value, LogicalType::VARCHAR)});
}
inline LogicalType Receipt_string_rootType() {
return LogicalType::STRUCT({{"type", LogicalType::VARCHAR}});
}
inline Value Receipt_string_rootValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_string_rootType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_string_rootType()); }
return Value::STRUCT(Receipt_string_rootType(), {ReceiptLeaf(ReceiptMember(value, "type"), LogicalType::VARCHAR)});
}
inline LogicalType Receipt_input_property_typeType() {
return LogicalType::STRUCT({{"items", Receipt_string_rootType()}, {"type", LogicalType::VARCHAR}});
}
inline Value Receipt_input_property_typeValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_input_property_typeType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_input_property_typeType()); }
return Value::STRUCT(Receipt_input_property_typeType(), {Receipt_string_rootValue(ReceiptMember(value, "items")), ReceiptLeaf(ReceiptMember(value, "type"), LogicalType::VARCHAR)});
}
inline LogicalType Receipt_question_document_context_schema_properties_entryType() {
return LogicalType::STRUCT({{"name", LogicalType::VARCHAR}, {"value", Receipt_input_property_typeType()}});
}
inline Value Receipt_question_document_context_schema_properties_entryValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_question_document_context_schema_properties_entryType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_question_document_context_schema_properties_entryType()); }
return Value::STRUCT(Receipt_question_document_context_schema_properties_entryType(), {ReceiptLeaf(ReceiptMember(value, "name"), LogicalType::VARCHAR), Receipt_input_property_typeValue(ReceiptMember(value, "value"))});
}
inline LogicalType Receipt_question_document_context_schemaType() {
return LogicalType::STRUCT({{"properties", LogicalType::LIST(Receipt_question_document_context_schema_properties_entryType())}, {"required", LogicalType::LIST(LogicalType::VARCHAR)}, {"type", LogicalType::VARCHAR}});
}
inline Value Receipt_question_document_context_schemaValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_question_document_context_schemaType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_question_document_context_schemaType()); }
return Value::STRUCT(Receipt_question_document_context_schemaType(), {ReceiptMapping(ReceiptMember(value, "properties"), Receipt_question_document_context_schema_properties_entryType(), [](const string &key, ReceiptJSON *item) { return Value::STRUCT(Receipt_question_document_context_schema_properties_entryType(), {Value(key), Receipt_input_property_typeValue(item)}); }), ReceiptList(ReceiptMember(value, "required"), LogicalType::VARCHAR, [](ReceiptJSON *item) { return ReceiptLeaf(item, LogicalType::VARCHAR); }), ReceiptLeaf(ReceiptMember(value, "type"), LogicalType::VARCHAR)});
}
inline LogicalType Receipt_question_document_item_schema_properties_entryType() {
return LogicalType::STRUCT({{"name", LogicalType::VARCHAR}, {"value", Receipt_input_property_typeType()}});
}
inline Value Receipt_question_document_item_schema_properties_entryValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_question_document_item_schema_properties_entryType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_question_document_item_schema_properties_entryType()); }
return Value::STRUCT(Receipt_question_document_item_schema_properties_entryType(), {ReceiptLeaf(ReceiptMember(value, "name"), LogicalType::VARCHAR), Receipt_input_property_typeValue(ReceiptMember(value, "value"))});
}
inline LogicalType Receipt_question_document_item_schemaType() {
return LogicalType::STRUCT({{"properties", LogicalType::LIST(Receipt_question_document_item_schema_properties_entryType())}, {"required", LogicalType::LIST(LogicalType::VARCHAR)}, {"type", LogicalType::VARCHAR}});
}
inline Value Receipt_question_document_item_schemaValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_question_document_item_schemaType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_question_document_item_schemaType()); }
return Value::STRUCT(Receipt_question_document_item_schemaType(), {ReceiptMapping(ReceiptMember(value, "properties"), Receipt_question_document_item_schema_properties_entryType(), [](const string &key, ReceiptJSON *item) { return Value::STRUCT(Receipt_question_document_item_schema_properties_entryType(), {Value(key), Receipt_input_property_typeValue(item)}); }), ReceiptList(ReceiptMember(value, "required"), LogicalType::VARCHAR, [](ReceiptJSON *item) { return ReceiptLeaf(item, LogicalType::VARCHAR); }), ReceiptLeaf(ReceiptMember(value, "type"), LogicalType::VARCHAR)});
}
inline LogicalType Receipt_labelType() {
return LogicalType::STRUCT({{"description", LogicalType::JSON()}, {"description_present", LogicalType::BOOLEAN}, {"name", LogicalType::VARCHAR}});
}
inline Value Receipt_labelValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_labelType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_labelType()); }
return Value::STRUCT(Receipt_labelType(), {ReceiptLeaf(ReceiptMember(value, "description"), LogicalType::JSON()), Value::BOOLEAN(ReceiptMember(value, "description") != nullptr), ReceiptLeaf(ReceiptMember(value, "name"), LogicalType::VARCHAR)});
}
inline LogicalType Receipt_question_documentType() {
return LogicalType::STRUCT({{"batch", Receipt_question_document_batchType()}, {"context_schema", Receipt_question_document_context_schemaType()}, {"false", LogicalType::JSON()}, {"false_present", LogicalType::BOOLEAN}, {"item_schema", Receipt_question_document_item_schemaType()}, {"label_details", LogicalType::LIST(Receipt_labelType())}, {"labels", LogicalType::LIST(LogicalType::VARCHAR)}, {"levels", LogicalType::LIST(LogicalType::VARCHAR)}, {"model", LogicalType::VARCHAR}, {"name", LogicalType::VARCHAR}, {"on", LogicalType::LIST(LogicalType::VARCHAR)}, {"options", LogicalType::LIST(LogicalType::VARCHAR)}, {"profile", LogicalType::VARCHAR}, {"text", LogicalType::JSON()}, {"text_present", LogicalType::BOOLEAN}, {"true", LogicalType::JSON()}, {"true_present", LogicalType::BOOLEAN}, {"verb", LogicalType::VARCHAR}, {"wording_version", LogicalType::BIGINT}});
}
inline Value Receipt_question_documentValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_question_documentType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_question_documentType()); }
return Value::STRUCT(Receipt_question_documentType(), {Receipt_question_document_batchValue(ReceiptMember(value, "batch")), Receipt_question_document_context_schemaValue(ReceiptMember(value, "context_schema")), ReceiptLeaf(ReceiptMember(value, "false"), LogicalType::JSON()), Value::BOOLEAN(ReceiptMember(value, "false") != nullptr), Receipt_question_document_item_schemaValue(ReceiptMember(value, "item_schema")), ReceiptList(ReceiptMember(value, "label_details"), Receipt_labelType(), [](ReceiptJSON *item) { return Receipt_labelValue(item); }), ReceiptList(ReceiptMember(value, "labels"), LogicalType::VARCHAR, [](ReceiptJSON *item) { return ReceiptLeaf(item, LogicalType::VARCHAR); }), ReceiptList(ReceiptMember(value, "levels"), LogicalType::VARCHAR, [](ReceiptJSON *item) { return ReceiptLeaf(item, LogicalType::VARCHAR); }), ReceiptLeaf(ReceiptMember(value, "model"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "name"), LogicalType::VARCHAR), ReceiptList(ReceiptMember(value, "on"), LogicalType::VARCHAR, [](ReceiptJSON *item) { return ReceiptLeaf(item, LogicalType::VARCHAR); }), ReceiptList(ReceiptMember(value, "options"), LogicalType::VARCHAR, [](ReceiptJSON *item) { return ReceiptLeaf(item, LogicalType::VARCHAR); }), ReceiptLeaf(ReceiptMember(value, "profile"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "text"), LogicalType::JSON()), Value::BOOLEAN(ReceiptMember(value, "text") != nullptr), ReceiptLeaf(ReceiptMember(value, "true"), LogicalType::JSON()), Value::BOOLEAN(ReceiptMember(value, "true") != nullptr), ReceiptLeaf(ReceiptMember(value, "verb"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "wording_version"), LogicalType::BIGINT)});
}
inline LogicalType Receipt_decide_sourceType() {
return LogicalType::STRUCT({{"file", LogicalType::VARCHAR}, {"first_line", LogicalType::UBIGINT}, {"last_line", LogicalType::UBIGINT}});
}
inline Value Receipt_decide_sourceValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_decide_sourceType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_decide_sourceType()); }
return Value::STRUCT(Receipt_decide_sourceType(), {ReceiptLeaf(ReceiptMember(value, "file"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "first_line"), LogicalType::UBIGINT), ReceiptLeaf(ReceiptMember(value, "last_line"), LogicalType::UBIGINT)});
}
inline LogicalType Receipt_decide_thresholdType() {
return LogicalType::STRUCT({{"number_value", LogicalType::DOUBLE}, {"string_value", LogicalType::VARCHAR}});
}
inline Value Receipt_decide_thresholdValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_decide_thresholdType()); }
return Value::STRUCT(Receipt_decide_thresholdType(), {ReceiptLeaf(value, LogicalType::DOUBLE), ReceiptLeaf(value, LogicalType::VARCHAR)});
}
inline LogicalType Receipt_decisionType() {
return LogicalType::STRUCT({{"answer", Receipt_answerType()}, {"answer_id", LogicalType::VARCHAR}, {"file", LogicalType::VARCHAR}, {"first_line", LogicalType::UBIGINT}, {"images", LogicalType::LIST(Receipt_imageType())}, {"index", LogicalType::UBIGINT}, {"input", LogicalType::JSON()}, {"input_present", LogicalType::BOOLEAN}, {"input_file", LogicalType::VARCHAR}, {"last_line", LogicalType::UBIGINT}, {"meta", Receipt_metadataType()}, {"position", Receipt_positionType()}, {"question", Receipt_question_documentType()}, {"question_name", LogicalType::VARCHAR}, {"schema", LogicalType::VARCHAR}, {"source", Receipt_decide_sourceType()}, {"threshold", Receipt_decide_thresholdType()}, {"value", LogicalType::JSON()}, {"value_present", LogicalType::BOOLEAN}});
}
inline Value Receipt_decisionValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_decisionType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_decisionType()); }
return Value::STRUCT(Receipt_decisionType(), {Receipt_answerValue(ReceiptMember(value, "answer")), ReceiptLeaf(ReceiptMember(value, "answer_id"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "file"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "first_line"), LogicalType::UBIGINT), ReceiptList(ReceiptMember(value, "images"), Receipt_imageType(), [](ReceiptJSON *item) { return Receipt_imageValue(item); }), ReceiptLeaf(ReceiptMember(value, "index"), LogicalType::UBIGINT), ReceiptLeaf(ReceiptMember(value, "input"), LogicalType::JSON()), Value::BOOLEAN(ReceiptMember(value, "input") != nullptr), ReceiptLeaf(ReceiptMember(value, "input_file"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "last_line"), LogicalType::UBIGINT), Receipt_metadataValue(ReceiptMember(value, "meta")), Receipt_positionValue(ReceiptMember(value, "position")), Receipt_question_documentValue(ReceiptMember(value, "question")), ReceiptLeaf(ReceiptMember(value, "question_name"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "schema"), LogicalType::VARCHAR), Receipt_decide_sourceValue(ReceiptMember(value, "source")), Receipt_decide_thresholdValue(ReceiptMember(value, "threshold")), ReceiptLeaf(ReceiptMember(value, "value"), LogicalType::JSON()), Value::BOOLEAN(ReceiptMember(value, "value") != nullptr)});
}
inline LogicalType Receipt_choose_sourceType() {
return LogicalType::STRUCT({{"file", LogicalType::VARCHAR}, {"first_line", LogicalType::UBIGINT}, {"last_line", LogicalType::UBIGINT}});
}
inline Value Receipt_choose_sourceValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_choose_sourceType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_choose_sourceType()); }
return Value::STRUCT(Receipt_choose_sourceType(), {ReceiptLeaf(ReceiptMember(value, "file"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "first_line"), LogicalType::UBIGINT), ReceiptLeaf(ReceiptMember(value, "last_line"), LogicalType::UBIGINT)});
}
inline LogicalType Receipt_choose_thresholdType() {
return LogicalType::STRUCT({{"number_value", LogicalType::DOUBLE}, {"string_value", LogicalType::VARCHAR}});
}
inline Value Receipt_choose_thresholdValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_choose_thresholdType()); }
return Value::STRUCT(Receipt_choose_thresholdType(), {ReceiptLeaf(value, LogicalType::DOUBLE), ReceiptLeaf(value, LogicalType::VARCHAR)});
}
inline LogicalType Receipt_choiceType() {
return LogicalType::STRUCT({{"answer", Receipt_answerType()}, {"answer_id", LogicalType::VARCHAR}, {"file", LogicalType::VARCHAR}, {"first_line", LogicalType::UBIGINT}, {"images", LogicalType::LIST(Receipt_imageType())}, {"index", LogicalType::UBIGINT}, {"input", LogicalType::JSON()}, {"input_present", LogicalType::BOOLEAN}, {"input_file", LogicalType::VARCHAR}, {"last_line", LogicalType::UBIGINT}, {"meta", Receipt_metadataType()}, {"position", Receipt_positionType()}, {"question", Receipt_question_documentType()}, {"question_name", LogicalType::VARCHAR}, {"schema", LogicalType::VARCHAR}, {"source", Receipt_choose_sourceType()}, {"threshold", Receipt_choose_thresholdType()}, {"value", LogicalType::VARCHAR}});
}
inline Value Receipt_choiceValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_choiceType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_choiceType()); }
return Value::STRUCT(Receipt_choiceType(), {Receipt_answerValue(ReceiptMember(value, "answer")), ReceiptLeaf(ReceiptMember(value, "answer_id"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "file"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "first_line"), LogicalType::UBIGINT), ReceiptList(ReceiptMember(value, "images"), Receipt_imageType(), [](ReceiptJSON *item) { return Receipt_imageValue(item); }), ReceiptLeaf(ReceiptMember(value, "index"), LogicalType::UBIGINT), ReceiptLeaf(ReceiptMember(value, "input"), LogicalType::JSON()), Value::BOOLEAN(ReceiptMember(value, "input") != nullptr), ReceiptLeaf(ReceiptMember(value, "input_file"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "last_line"), LogicalType::UBIGINT), Receipt_metadataValue(ReceiptMember(value, "meta")), Receipt_positionValue(ReceiptMember(value, "position")), Receipt_question_documentValue(ReceiptMember(value, "question")), ReceiptLeaf(ReceiptMember(value, "question_name"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "schema"), LogicalType::VARCHAR), Receipt_choose_sourceValue(ReceiptMember(value, "source")), Receipt_choose_thresholdValue(ReceiptMember(value, "threshold")), ReceiptLeaf(ReceiptMember(value, "value"), LogicalType::VARCHAR)});
}
inline LogicalType Receipt_tag_sourceType() {
return LogicalType::STRUCT({{"file", LogicalType::VARCHAR}, {"first_line", LogicalType::UBIGINT}, {"last_line", LogicalType::UBIGINT}});
}
inline Value Receipt_tag_sourceValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_tag_sourceType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_tag_sourceType()); }
return Value::STRUCT(Receipt_tag_sourceType(), {ReceiptLeaf(ReceiptMember(value, "file"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "first_line"), LogicalType::UBIGINT), ReceiptLeaf(ReceiptMember(value, "last_line"), LogicalType::UBIGINT)});
}
inline LogicalType Receipt_tag_thresholdType() {
return LogicalType::STRUCT({{"number_value", LogicalType::DOUBLE}, {"string_value", LogicalType::VARCHAR}});
}
inline Value Receipt_tag_thresholdValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_tag_thresholdType()); }
return Value::STRUCT(Receipt_tag_thresholdType(), {ReceiptLeaf(value, LogicalType::DOUBLE), ReceiptLeaf(value, LogicalType::VARCHAR)});
}
inline LogicalType Receipt_tagsType() {
return LogicalType::STRUCT({{"answer", Receipt_answerType()}, {"answer_id", LogicalType::VARCHAR}, {"file", LogicalType::VARCHAR}, {"first_line", LogicalType::UBIGINT}, {"images", LogicalType::LIST(Receipt_imageType())}, {"index", LogicalType::UBIGINT}, {"input", LogicalType::JSON()}, {"input_present", LogicalType::BOOLEAN}, {"input_file", LogicalType::VARCHAR}, {"last_line", LogicalType::UBIGINT}, {"meta", Receipt_metadataType()}, {"position", Receipt_positionType()}, {"question", Receipt_question_documentType()}, {"question_name", LogicalType::VARCHAR}, {"schema", LogicalType::VARCHAR}, {"source", Receipt_tag_sourceType()}, {"threshold", Receipt_tag_thresholdType()}, {"value", LogicalType::LIST(LogicalType::VARCHAR)}});
}
inline Value Receipt_tagsValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_tagsType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_tagsType()); }
return Value::STRUCT(Receipt_tagsType(), {Receipt_answerValue(ReceiptMember(value, "answer")), ReceiptLeaf(ReceiptMember(value, "answer_id"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "file"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "first_line"), LogicalType::UBIGINT), ReceiptList(ReceiptMember(value, "images"), Receipt_imageType(), [](ReceiptJSON *item) { return Receipt_imageValue(item); }), ReceiptLeaf(ReceiptMember(value, "index"), LogicalType::UBIGINT), ReceiptLeaf(ReceiptMember(value, "input"), LogicalType::JSON()), Value::BOOLEAN(ReceiptMember(value, "input") != nullptr), ReceiptLeaf(ReceiptMember(value, "input_file"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "last_line"), LogicalType::UBIGINT), Receipt_metadataValue(ReceiptMember(value, "meta")), Receipt_positionValue(ReceiptMember(value, "position")), Receipt_question_documentValue(ReceiptMember(value, "question")), ReceiptLeaf(ReceiptMember(value, "question_name"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "schema"), LogicalType::VARCHAR), Receipt_tag_sourceValue(ReceiptMember(value, "source")), Receipt_tag_thresholdValue(ReceiptMember(value, "threshold")), ReceiptList(ReceiptMember(value, "value"), LogicalType::VARCHAR, [](ReceiptJSON *item) { return ReceiptLeaf(item, LogicalType::VARCHAR); })});
}
inline LogicalType Receipt_score_sourceType() {
return LogicalType::STRUCT({{"file", LogicalType::VARCHAR}, {"first_line", LogicalType::UBIGINT}, {"last_line", LogicalType::UBIGINT}});
}
inline Value Receipt_score_sourceValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_score_sourceType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_score_sourceType()); }
return Value::STRUCT(Receipt_score_sourceType(), {ReceiptLeaf(ReceiptMember(value, "file"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "first_line"), LogicalType::UBIGINT), ReceiptLeaf(ReceiptMember(value, "last_line"), LogicalType::UBIGINT)});
}
inline LogicalType Receipt_scoreType() {
return LogicalType::STRUCT({{"answer", Receipt_answerType()}, {"answer_id", LogicalType::VARCHAR}, {"file", LogicalType::VARCHAR}, {"first_line", LogicalType::UBIGINT}, {"images", LogicalType::LIST(Receipt_imageType())}, {"index", LogicalType::UBIGINT}, {"input", LogicalType::JSON()}, {"input_present", LogicalType::BOOLEAN}, {"input_file", LogicalType::VARCHAR}, {"last_line", LogicalType::UBIGINT}, {"meta", Receipt_metadataType()}, {"position", Receipt_positionType()}, {"question", Receipt_question_documentType()}, {"question_name", LogicalType::VARCHAR}, {"schema", LogicalType::VARCHAR}, {"source", Receipt_score_sourceType()}, {"threshold", LogicalType::JSON()}, {"threshold_present", LogicalType::BOOLEAN}, {"value", LogicalType::DOUBLE}});
}
inline Value Receipt_scoreValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_scoreType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_scoreType()); }
return Value::STRUCT(Receipt_scoreType(), {Receipt_answerValue(ReceiptMember(value, "answer")), ReceiptLeaf(ReceiptMember(value, "answer_id"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "file"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "first_line"), LogicalType::UBIGINT), ReceiptList(ReceiptMember(value, "images"), Receipt_imageType(), [](ReceiptJSON *item) { return Receipt_imageValue(item); }), ReceiptLeaf(ReceiptMember(value, "index"), LogicalType::UBIGINT), ReceiptLeaf(ReceiptMember(value, "input"), LogicalType::JSON()), Value::BOOLEAN(ReceiptMember(value, "input") != nullptr), ReceiptLeaf(ReceiptMember(value, "input_file"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "last_line"), LogicalType::UBIGINT), Receipt_metadataValue(ReceiptMember(value, "meta")), Receipt_positionValue(ReceiptMember(value, "position")), Receipt_question_documentValue(ReceiptMember(value, "question")), ReceiptLeaf(ReceiptMember(value, "question_name"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "schema"), LogicalType::VARCHAR), Receipt_score_sourceValue(ReceiptMember(value, "source")), ReceiptLeaf(ReceiptMember(value, "threshold"), LogicalType::JSON()), Value::BOOLEAN(ReceiptMember(value, "threshold") != nullptr), ReceiptLeaf(ReceiptMember(value, "value"), LogicalType::DOUBLE)});
}
inline LogicalType Receipt_filter_sourceType() {
return LogicalType::STRUCT({{"file", LogicalType::VARCHAR}, {"first_line", LogicalType::UBIGINT}, {"last_line", LogicalType::UBIGINT}});
}
inline Value Receipt_filter_sourceValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_filter_sourceType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_filter_sourceType()); }
return Value::STRUCT(Receipt_filter_sourceType(), {ReceiptLeaf(ReceiptMember(value, "file"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "first_line"), LogicalType::UBIGINT), ReceiptLeaf(ReceiptMember(value, "last_line"), LogicalType::UBIGINT)});
}
inline LogicalType Receipt_filter_thresholdType() {
return LogicalType::STRUCT({{"number_value", LogicalType::DOUBLE}, {"string_value", LogicalType::VARCHAR}});
}
inline Value Receipt_filter_thresholdValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_filter_thresholdType()); }
return Value::STRUCT(Receipt_filter_thresholdType(), {ReceiptLeaf(value, LogicalType::DOUBLE), ReceiptLeaf(value, LogicalType::VARCHAR)});
}
inline LogicalType Receipt_filteredType() {
return LogicalType::STRUCT({{"answer", Receipt_answerType()}, {"answer_id", LogicalType::VARCHAR}, {"file", LogicalType::VARCHAR}, {"first_line", LogicalType::UBIGINT}, {"images", LogicalType::LIST(Receipt_imageType())}, {"index", LogicalType::UBIGINT}, {"input", LogicalType::JSON()}, {"input_present", LogicalType::BOOLEAN}, {"input_file", LogicalType::VARCHAR}, {"last_line", LogicalType::UBIGINT}, {"meta", Receipt_metadataType()}, {"position", Receipt_positionType()}, {"question", Receipt_question_documentType()}, {"question_name", LogicalType::VARCHAR}, {"schema", LogicalType::VARCHAR}, {"source", Receipt_filter_sourceType()}, {"threshold", Receipt_filter_thresholdType()}, {"value", LogicalType::BOOLEAN}});
}
inline Value Receipt_filteredValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_filteredType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_filteredType()); }
return Value::STRUCT(Receipt_filteredType(), {Receipt_answerValue(ReceiptMember(value, "answer")), ReceiptLeaf(ReceiptMember(value, "answer_id"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "file"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "first_line"), LogicalType::UBIGINT), ReceiptList(ReceiptMember(value, "images"), Receipt_imageType(), [](ReceiptJSON *item) { return Receipt_imageValue(item); }), ReceiptLeaf(ReceiptMember(value, "index"), LogicalType::UBIGINT), ReceiptLeaf(ReceiptMember(value, "input"), LogicalType::JSON()), Value::BOOLEAN(ReceiptMember(value, "input") != nullptr), ReceiptLeaf(ReceiptMember(value, "input_file"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "last_line"), LogicalType::UBIGINT), Receipt_metadataValue(ReceiptMember(value, "meta")), Receipt_positionValue(ReceiptMember(value, "position")), Receipt_question_documentValue(ReceiptMember(value, "question")), ReceiptLeaf(ReceiptMember(value, "question_name"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "schema"), LogicalType::VARCHAR), Receipt_filter_sourceValue(ReceiptMember(value, "source")), Receipt_filter_thresholdValue(ReceiptMember(value, "threshold")), ReceiptLeaf(ReceiptMember(value, "value"), LogicalType::BOOLEAN)});
}
inline LogicalType Receipt_rank_member_result_sourceType() {
return LogicalType::STRUCT({{"file", LogicalType::VARCHAR}, {"first_line", LogicalType::UBIGINT}, {"last_line", LogicalType::UBIGINT}});
}
inline Value Receipt_rank_member_result_sourceValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_rank_member_result_sourceType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_rank_member_result_sourceType()); }
return Value::STRUCT(Receipt_rank_member_result_sourceType(), {ReceiptLeaf(ReceiptMember(value, "file"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "first_line"), LogicalType::UBIGINT), ReceiptLeaf(ReceiptMember(value, "last_line"), LogicalType::UBIGINT)});
}
inline LogicalType Receipt_rank_member_resultType() {
return LogicalType::STRUCT({{"answer", Receipt_answerType()}, {"answer_id", LogicalType::VARCHAR}, {"images", LogicalType::LIST(Receipt_imageType())}, {"meta", Receipt_metadataType()}, {"question", Receipt_question_documentType()}, {"schema", LogicalType::VARCHAR}, {"source", Receipt_rank_member_result_sourceType()}, {"threshold", LogicalType::JSON()}, {"threshold_present", LogicalType::BOOLEAN}, {"value", LogicalType::UBIGINT}});
}
inline Value Receipt_rank_member_resultValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_rank_member_resultType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_rank_member_resultType()); }
return Value::STRUCT(Receipt_rank_member_resultType(), {Receipt_answerValue(ReceiptMember(value, "answer")), ReceiptLeaf(ReceiptMember(value, "answer_id"), LogicalType::VARCHAR), ReceiptList(ReceiptMember(value, "images"), Receipt_imageType(), [](ReceiptJSON *item) { return Receipt_imageValue(item); }), Receipt_metadataValue(ReceiptMember(value, "meta")), Receipt_question_documentValue(ReceiptMember(value, "question")), ReceiptLeaf(ReceiptMember(value, "schema"), LogicalType::VARCHAR), Receipt_rank_member_result_sourceValue(ReceiptMember(value, "source")), ReceiptLeaf(ReceiptMember(value, "threshold"), LogicalType::JSON()), Value::BOOLEAN(ReceiptMember(value, "threshold") != nullptr), ReceiptLeaf(ReceiptMember(value, "value"), LogicalType::UBIGINT)});
}
inline LogicalType Receipt_rank_memberType() {
return LogicalType::STRUCT({{"name", LogicalType::VARCHAR}, {"result", Receipt_rank_member_resultType()}});
}
inline Value Receipt_rank_memberValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_rank_memberType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_rank_memberType()); }
return Value::STRUCT(Receipt_rank_memberType(), {ReceiptLeaf(ReceiptMember(value, "name"), LogicalType::VARCHAR), Receipt_rank_member_resultValue(ReceiptMember(value, "result"))});
}
inline LogicalType Receipt_rank_sourceType() {
return LogicalType::STRUCT({{"file", LogicalType::VARCHAR}, {"first_line", LogicalType::UBIGINT}, {"last_line", LogicalType::UBIGINT}});
}
inline Value Receipt_rank_sourceValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_rank_sourceType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_rank_sourceType()); }
return Value::STRUCT(Receipt_rank_sourceType(), {ReceiptLeaf(ReceiptMember(value, "file"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "first_line"), LogicalType::UBIGINT), ReceiptLeaf(ReceiptMember(value, "last_line"), LogicalType::UBIGINT)});
}
inline LogicalType Receipt_rankedType() {
return LogicalType::STRUCT({{"answer", Receipt_answerType()}, {"answer_id", LogicalType::VARCHAR}, {"file", LogicalType::VARCHAR}, {"first_line", LogicalType::UBIGINT}, {"images", LogicalType::LIST(Receipt_imageType())}, {"index", LogicalType::UBIGINT}, {"input", LogicalType::JSON()}, {"input_present", LogicalType::BOOLEAN}, {"input_file", LogicalType::VARCHAR}, {"last_line", LogicalType::UBIGINT}, {"members", LogicalType::LIST(Receipt_rank_memberType())}, {"meta", Receipt_metadataType()}, {"position", Receipt_positionType()}, {"question", Receipt_question_documentType()}, {"question_name", LogicalType::VARCHAR}, {"schema", LogicalType::VARCHAR}, {"source", Receipt_rank_sourceType()}, {"threshold", LogicalType::JSON()}, {"threshold_present", LogicalType::BOOLEAN}, {"value", LogicalType::UBIGINT}});
}
inline Value Receipt_rankedValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_rankedType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_rankedType()); }
return Value::STRUCT(Receipt_rankedType(), {Receipt_answerValue(ReceiptMember(value, "answer")), ReceiptLeaf(ReceiptMember(value, "answer_id"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "file"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "first_line"), LogicalType::UBIGINT), ReceiptList(ReceiptMember(value, "images"), Receipt_imageType(), [](ReceiptJSON *item) { return Receipt_imageValue(item); }), ReceiptLeaf(ReceiptMember(value, "index"), LogicalType::UBIGINT), ReceiptLeaf(ReceiptMember(value, "input"), LogicalType::JSON()), Value::BOOLEAN(ReceiptMember(value, "input") != nullptr), ReceiptLeaf(ReceiptMember(value, "input_file"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "last_line"), LogicalType::UBIGINT), ReceiptList(ReceiptMember(value, "members"), Receipt_rank_memberType(), [](ReceiptJSON *item) { return Receipt_rank_memberValue(item); }), Receipt_metadataValue(ReceiptMember(value, "meta")), Receipt_positionValue(ReceiptMember(value, "position")), Receipt_question_documentValue(ReceiptMember(value, "question")), ReceiptLeaf(ReceiptMember(value, "question_name"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "schema"), LogicalType::VARCHAR), Receipt_rank_sourceValue(ReceiptMember(value, "source")), ReceiptLeaf(ReceiptMember(value, "threshold"), LogicalType::JSON()), Value::BOOLEAN(ReceiptMember(value, "threshold") != nullptr), ReceiptLeaf(ReceiptMember(value, "value"), LogicalType::UBIGINT)});
}
inline LogicalType Receipt_failureType() {
return LogicalType::STRUCT({{"cause", LogicalType::VARCHAR}, {"kind", LogicalType::VARCHAR}});
}
inline Value Receipt_failureValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_failureType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_failureType()); }
return Value::STRUCT(Receipt_failureType(), {ReceiptLeaf(ReceiptMember(value, "cause"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "kind"), LogicalType::VARCHAR)});
}
inline LogicalType Receipt_annotation_member_thresholdType() {
return LogicalType::STRUCT({{"number_value", LogicalType::DOUBLE}, {"string_value", LogicalType::VARCHAR}});
}
inline Value Receipt_annotation_member_thresholdValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_annotation_member_thresholdType()); }
return Value::STRUCT(Receipt_annotation_member_thresholdType(), {ReceiptLeaf(value, LogicalType::DOUBLE), ReceiptLeaf(value, LogicalType::VARCHAR)});
}
inline LogicalType Receipt_annotation_member_usageType() {
return LogicalType::STRUCT({{"input_tokens", LogicalType::UBIGINT}, {"output_tokens", LogicalType::UBIGINT}});
}
inline Value Receipt_annotation_member_usageValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_annotation_member_usageType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_annotation_member_usageType()); }
return Value::STRUCT(Receipt_annotation_member_usageType(), {ReceiptLeaf(ReceiptMember(value, "input_tokens"), LogicalType::UBIGINT), ReceiptLeaf(ReceiptMember(value, "output_tokens"), LogicalType::UBIGINT)});
}
inline LogicalType Receipt_valueType() {
return LogicalType::STRUCT({{"array_value", LogicalType::LIST(LogicalType::VARCHAR)}, {"boolean_value", LogicalType::BOOLEAN}, {"number_value", LogicalType::DOUBLE}, {"string_value", LogicalType::VARCHAR}});
}
inline Value Receipt_valueValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_valueType()); }
return Value::STRUCT(Receipt_valueType(), {ReceiptList(value, LogicalType::VARCHAR, [](ReceiptJSON *item) { return ReceiptLeaf(item, LogicalType::VARCHAR); }), ReceiptLeaf(value, LogicalType::BOOLEAN), ReceiptLeaf(value, LogicalType::DOUBLE), ReceiptLeaf(value, LogicalType::VARCHAR)});
}
inline LogicalType Receipt_annotation_memberType() {
return LogicalType::STRUCT({{"answer", Receipt_answerType()}, {"answer_id", LogicalType::VARCHAR}, {"failure", Receipt_failureType()}, {"failure_id", LogicalType::VARCHAR}, {"observations", LogicalType::LIST(Receipt_observationType())}, {"question", Receipt_question_documentType()}, {"question_sources", LogicalType::LIST(Receipt_question_sourceType())}, {"request", LogicalType::VARCHAR}, {"threshold", Receipt_annotation_member_thresholdType()}, {"usage", Receipt_annotation_member_usageType()}, {"value", Receipt_valueType()}});
}
inline Value Receipt_annotation_memberValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_annotation_memberType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_annotation_memberType()); }
return Value::STRUCT(Receipt_annotation_memberType(), {Receipt_answerValue(ReceiptMember(value, "answer")), ReceiptLeaf(ReceiptMember(value, "answer_id"), LogicalType::VARCHAR), Receipt_failureValue(ReceiptMember(value, "failure")), ReceiptLeaf(ReceiptMember(value, "failure_id"), LogicalType::VARCHAR), ReceiptList(ReceiptMember(value, "observations"), Receipt_observationType(), [](ReceiptJSON *item) { return Receipt_observationValue(item); }), Receipt_question_documentValue(ReceiptMember(value, "question")), ReceiptList(ReceiptMember(value, "question_sources"), Receipt_question_sourceType(), [](ReceiptJSON *item) { return Receipt_question_sourceValue(item); }), ReceiptLeaf(ReceiptMember(value, "request"), LogicalType::VARCHAR), Receipt_annotation_member_thresholdValue(ReceiptMember(value, "threshold")), Receipt_annotation_member_usageValue(ReceiptMember(value, "usage")), Receipt_valueValue(ReceiptMember(value, "value"))});
}
inline LogicalType Receipt_annotation_answers_entryType() {
return LogicalType::STRUCT({{"name", LogicalType::VARCHAR}, {"value", Receipt_annotation_memberType()}});
}
inline Value Receipt_annotation_answers_entryValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_annotation_answers_entryType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_annotation_answers_entryType()); }
return Value::STRUCT(Receipt_annotation_answers_entryType(), {ReceiptLeaf(ReceiptMember(value, "name"), LogicalType::VARCHAR), Receipt_annotation_memberValue(ReceiptMember(value, "value"))});
}
inline LogicalType Receipt_annotation_sourceType() {
return LogicalType::STRUCT({{"file", LogicalType::VARCHAR}, {"first_line", LogicalType::UBIGINT}, {"last_line", LogicalType::UBIGINT}});
}
inline Value Receipt_annotation_sourceValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_annotation_sourceType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_annotation_sourceType()); }
return Value::STRUCT(Receipt_annotation_sourceType(), {ReceiptLeaf(ReceiptMember(value, "file"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "first_line"), LogicalType::UBIGINT), ReceiptLeaf(ReceiptMember(value, "last_line"), LogicalType::UBIGINT)});
}
inline LogicalType Receipt_annotated_field_object_valueType() {
return LogicalType::STRUCT({{"failed", Receipt_failureType()}});
}
inline Value Receipt_annotated_field_object_valueValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_annotated_field_object_valueType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_annotated_field_object_valueType()); }
return Value::STRUCT(Receipt_annotated_field_object_valueType(), {Receipt_failureValue(ReceiptMember(value, "failed"))});
}
inline LogicalType Receipt_annotated_fieldType() {
return LogicalType::STRUCT({{"array_value", LogicalType::LIST(LogicalType::VARCHAR)}, {"boolean_value", LogicalType::BOOLEAN}, {"number_value", LogicalType::DOUBLE}, {"object_value", Receipt_annotated_field_object_valueType()}, {"string_value", LogicalType::VARCHAR}});
}
inline Value Receipt_annotated_fieldValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_annotated_fieldType()); }
return Value::STRUCT(Receipt_annotated_fieldType(), {ReceiptList(value, LogicalType::VARCHAR, [](ReceiptJSON *item) { return ReceiptLeaf(item, LogicalType::VARCHAR); }), ReceiptLeaf(value, LogicalType::BOOLEAN), ReceiptLeaf(value, LogicalType::DOUBLE), Receipt_annotated_field_object_valueValue(value), ReceiptLeaf(value, LogicalType::VARCHAR)});
}
inline LogicalType Receipt_annotated_row_entryType() {
return LogicalType::STRUCT({{"name", LogicalType::VARCHAR}, {"value", Receipt_annotated_fieldType()}});
}
inline Value Receipt_annotated_row_entryValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_annotated_row_entryType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_annotated_row_entryType()); }
return Value::STRUCT(Receipt_annotated_row_entryType(), {ReceiptLeaf(ReceiptMember(value, "name"), LogicalType::VARCHAR), Receipt_annotated_fieldValue(ReceiptMember(value, "value"))});
}
inline LogicalType Receipt_annotationType() {
return LogicalType::STRUCT({{"answer_id", LogicalType::VARCHAR}, {"answers", LogicalType::LIST(Receipt_annotation_answers_entryType())}, {"file", LogicalType::VARCHAR}, {"first_line", LogicalType::UBIGINT}, {"index", LogicalType::UBIGINT}, {"input", LogicalType::JSON()}, {"input_present", LogicalType::BOOLEAN}, {"last_line", LogicalType::UBIGINT}, {"meta", Receipt_metadataType()}, {"position", Receipt_positionType()}, {"schema", LogicalType::VARCHAR}, {"source", Receipt_annotation_sourceType()}, {"value", LogicalType::LIST(Receipt_annotated_row_entryType())}});
}
inline Value Receipt_annotationValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_annotationType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_annotationType()); }
return Value::STRUCT(Receipt_annotationType(), {ReceiptLeaf(ReceiptMember(value, "answer_id"), LogicalType::VARCHAR), ReceiptMapping(ReceiptMember(value, "answers"), Receipt_annotation_answers_entryType(), [](const string &key, ReceiptJSON *item) { return Value::STRUCT(Receipt_annotation_answers_entryType(), {Value(key), Receipt_annotation_memberValue(item)}); }), ReceiptLeaf(ReceiptMember(value, "file"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "first_line"), LogicalType::UBIGINT), ReceiptLeaf(ReceiptMember(value, "index"), LogicalType::UBIGINT), ReceiptLeaf(ReceiptMember(value, "input"), LogicalType::JSON()), Value::BOOLEAN(ReceiptMember(value, "input") != nullptr), ReceiptLeaf(ReceiptMember(value, "last_line"), LogicalType::UBIGINT), Receipt_metadataValue(ReceiptMember(value, "meta")), Receipt_positionValue(ReceiptMember(value, "position")), ReceiptLeaf(ReceiptMember(value, "schema"), LogicalType::VARCHAR), Receipt_annotation_sourceValue(ReceiptMember(value, "source")), ReceiptMapping(ReceiptMember(value, "value"), Receipt_annotated_row_entryType(), [](const string &key, ReceiptJSON *item) { return Value::STRUCT(Receipt_annotated_row_entryType(), {Value(key), Receipt_annotated_fieldValue(item)}); })});
}
inline LogicalType Receipt_name_odds_edges_entryType() {
return LogicalType::STRUCT({{"name", LogicalType::VARCHAR}, {"value", LogicalType::DOUBLE}});
}
inline Value Receipt_name_odds_edges_entryValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_name_odds_edges_entryType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_name_odds_edges_entryType()); }
return Value::STRUCT(Receipt_name_odds_edges_entryType(), {ReceiptLeaf(ReceiptMember(value, "name"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "value"), LogicalType::DOUBLE)});
}
inline LogicalType Receipt_name_odds_kinds_entryType() {
return LogicalType::STRUCT({{"name", LogicalType::VARCHAR}, {"value", LogicalType::DOUBLE}});
}
inline Value Receipt_name_odds_kinds_entryValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_name_odds_kinds_entryType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_name_odds_kinds_entryType()); }
return Value::STRUCT(Receipt_name_odds_kinds_entryType(), {ReceiptLeaf(ReceiptMember(value, "name"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "value"), LogicalType::DOUBLE)});
}
inline LogicalType Receipt_name_oddsType() {
return LogicalType::STRUCT({{"edges", LogicalType::LIST(Receipt_name_odds_edges_entryType())}, {"end", LogicalType::UBIGINT}, {"kinds", LogicalType::LIST(Receipt_name_odds_kinds_entryType())}, {"start", LogicalType::UBIGINT}});
}
inline Value Receipt_name_oddsValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_name_oddsType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_name_oddsType()); }
return Value::STRUCT(Receipt_name_oddsType(), {ReceiptMapping(ReceiptMember(value, "edges"), Receipt_name_odds_edges_entryType(), [](const string &key, ReceiptJSON *item) { return Value::STRUCT(Receipt_name_odds_edges_entryType(), {Value(key), ReceiptLeaf(item, LogicalType::DOUBLE)}); }), ReceiptLeaf(ReceiptMember(value, "end"), LogicalType::UBIGINT), ReceiptMapping(ReceiptMember(value, "kinds"), Receipt_name_odds_kinds_entryType(), [](const string &key, ReceiptJSON *item) { return Value::STRUCT(Receipt_name_odds_kinds_entryType(), {Value(key), ReceiptLeaf(item, LogicalType::DOUBLE)}); }), ReceiptLeaf(ReceiptMember(value, "start"), LogicalType::UBIGINT)});
}
inline LogicalType Receipt_placeType() {
return LogicalType::STRUCT({{"end", LogicalType::UBIGINT}, {"start", LogicalType::UBIGINT}});
}
inline Value Receipt_placeValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_placeType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_placeType()); }
return Value::STRUCT(Receipt_placeType(), {ReceiptLeaf(ReceiptMember(value, "end"), LogicalType::UBIGINT), ReceiptLeaf(ReceiptMember(value, "start"), LogicalType::UBIGINT)});
}
inline LogicalType Receipt_pair_oddsType() {
return LogicalType::STRUCT({{"probability", LogicalType::DOUBLE}, {"relation", LogicalType::VARCHAR}, {"source", Receipt_placeType()}, {"target", Receipt_placeType()}});
}
inline Value Receipt_pair_oddsValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_pair_oddsType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_pair_oddsType()); }
return Value::STRUCT(Receipt_pair_oddsType(), {ReceiptLeaf(ReceiptMember(value, "probability"), LogicalType::DOUBLE), ReceiptLeaf(ReceiptMember(value, "relation"), LogicalType::VARCHAR), Receipt_placeValue(ReceiptMember(value, "source")), Receipt_placeValue(ReceiptMember(value, "target"))});
}
inline LogicalType Receipt_piece_odds_tags_entryType() {
return LogicalType::STRUCT({{"name", LogicalType::VARCHAR}, {"value", LogicalType::DOUBLE}});
}
inline Value Receipt_piece_odds_tags_entryValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_piece_odds_tags_entryType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_piece_odds_tags_entryType()); }
return Value::STRUCT(Receipt_piece_odds_tags_entryType(), {ReceiptLeaf(ReceiptMember(value, "name"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "value"), LogicalType::DOUBLE)});
}
inline LogicalType Receipt_piece_oddsType() {
return LogicalType::STRUCT({{"end", LogicalType::UBIGINT}, {"start", LogicalType::UBIGINT}, {"tags", LogicalType::LIST(Receipt_piece_odds_tags_entryType())}});
}
inline Value Receipt_piece_oddsValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_piece_oddsType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_piece_oddsType()); }
return Value::STRUCT(Receipt_piece_oddsType(), {ReceiptLeaf(ReceiptMember(value, "end"), LogicalType::UBIGINT), ReceiptLeaf(ReceiptMember(value, "start"), LogicalType::UBIGINT), ReceiptMapping(ReceiptMember(value, "tags"), Receipt_piece_odds_tags_entryType(), [](const string &key, ReceiptJSON *item) { return Value::STRUCT(Receipt_piece_odds_tags_entryType(), {Value(key), ReceiptLeaf(item, LogicalType::DOUBLE)}); })});
}
inline LogicalType Receipt_recognition_odds_proposals_item_selectedType() {
return LogicalType::STRUCT({{"end", LogicalType::UBIGINT}, {"start", LogicalType::UBIGINT}});
}
inline Value Receipt_recognition_odds_proposals_item_selectedValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_recognition_odds_proposals_item_selectedType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_recognition_odds_proposals_item_selectedType()); }
return Value::STRUCT(Receipt_recognition_odds_proposals_item_selectedType(), {ReceiptLeaf(ReceiptMember(value, "end"), LogicalType::UBIGINT), ReceiptLeaf(ReceiptMember(value, "start"), LogicalType::UBIGINT)});
}
inline LogicalType Receipt_recognition_odds_proposals_itemType() {
return LogicalType::STRUCT({{"end", LogicalType::UBIGINT}, {"kept", LogicalType::BOOLEAN}, {"kind", LogicalType::VARCHAR}, {"length", LogicalType::UBIGINT}, {"probability", LogicalType::DOUBLE}, {"selected", Receipt_recognition_odds_proposals_item_selectedType()}, {"span_probability", LogicalType::DOUBLE}, {"start", LogicalType::UBIGINT}, {"strength", LogicalType::DOUBLE}, {"text", LogicalType::VARCHAR}});
}
inline Value Receipt_recognition_odds_proposals_itemValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_recognition_odds_proposals_itemType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_recognition_odds_proposals_itemType()); }
return Value::STRUCT(Receipt_recognition_odds_proposals_itemType(), {ReceiptLeaf(ReceiptMember(value, "end"), LogicalType::UBIGINT), ReceiptLeaf(ReceiptMember(value, "kept"), LogicalType::BOOLEAN), ReceiptLeaf(ReceiptMember(value, "kind"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "length"), LogicalType::UBIGINT), ReceiptLeaf(ReceiptMember(value, "probability"), LogicalType::DOUBLE), Receipt_recognition_odds_proposals_item_selectedValue(ReceiptMember(value, "selected")), ReceiptLeaf(ReceiptMember(value, "span_probability"), LogicalType::DOUBLE), ReceiptLeaf(ReceiptMember(value, "start"), LogicalType::UBIGINT), ReceiptLeaf(ReceiptMember(value, "strength"), LogicalType::DOUBLE), ReceiptLeaf(ReceiptMember(value, "text"), LogicalType::VARCHAR)});
}
inline LogicalType Receipt_recognition_oddsType() {
return LogicalType::STRUCT({{"names", LogicalType::LIST(Receipt_name_oddsType())}, {"pairs", LogicalType::LIST(Receipt_pair_oddsType())}, {"pieces", LogicalType::LIST(Receipt_piece_oddsType())}, {"proposals", LogicalType::LIST(Receipt_recognition_odds_proposals_itemType())}});
}
inline Value Receipt_recognition_oddsValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_recognition_oddsType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_recognition_oddsType()); }
return Value::STRUCT(Receipt_recognition_oddsType(), {ReceiptList(ReceiptMember(value, "names"), Receipt_name_oddsType(), [](ReceiptJSON *item) { return Receipt_name_oddsValue(item); }), ReceiptList(ReceiptMember(value, "pairs"), Receipt_pair_oddsType(), [](ReceiptJSON *item) { return Receipt_pair_oddsValue(item); }), ReceiptList(ReceiptMember(value, "pieces"), Receipt_piece_oddsType(), [](ReceiptJSON *item) { return Receipt_piece_oddsValue(item); }), ReceiptList(ReceiptMember(value, "proposals"), Receipt_recognition_odds_proposals_itemType(), [](ReceiptJSON *item) { return Receipt_recognition_odds_proposals_itemValue(item); })});
}
inline LogicalType Receipt_recognize_question_document_batchType() {
return LogicalType::STRUCT({{"integer_value", LogicalType::UBIGINT}, {"string_value", LogicalType::VARCHAR}});
}
inline Value Receipt_recognize_question_document_batchValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_recognize_question_document_batchType()); }
return Value::STRUCT(Receipt_recognize_question_document_batchType(), {ReceiptLeaf(value, LogicalType::UBIGINT), ReceiptLeaf(value, LogicalType::VARCHAR)});
}
inline LogicalType Receipt_recognize_question_document_context_schema_properties_entryType() {
return LogicalType::STRUCT({{"name", LogicalType::VARCHAR}, {"value", Receipt_input_property_typeType()}});
}
inline Value Receipt_recognize_question_document_context_schema_properties_entryValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_recognize_question_document_context_schema_properties_entryType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_recognize_question_document_context_schema_properties_entryType()); }
return Value::STRUCT(Receipt_recognize_question_document_context_schema_properties_entryType(), {ReceiptLeaf(ReceiptMember(value, "name"), LogicalType::VARCHAR), Receipt_input_property_typeValue(ReceiptMember(value, "value"))});
}
inline LogicalType Receipt_recognize_question_document_context_schemaType() {
return LogicalType::STRUCT({{"properties", LogicalType::LIST(Receipt_recognize_question_document_context_schema_properties_entryType())}, {"required", LogicalType::LIST(LogicalType::VARCHAR)}, {"type", LogicalType::VARCHAR}});
}
inline Value Receipt_recognize_question_document_context_schemaValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_recognize_question_document_context_schemaType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_recognize_question_document_context_schemaType()); }
return Value::STRUCT(Receipt_recognize_question_document_context_schemaType(), {ReceiptMapping(ReceiptMember(value, "properties"), Receipt_recognize_question_document_context_schema_properties_entryType(), [](const string &key, ReceiptJSON *item) { return Value::STRUCT(Receipt_recognize_question_document_context_schema_properties_entryType(), {Value(key), Receipt_input_property_typeValue(item)}); }), ReceiptList(ReceiptMember(value, "required"), LogicalType::VARCHAR, [](ReceiptJSON *item) { return ReceiptLeaf(item, LogicalType::VARCHAR); }), ReceiptLeaf(ReceiptMember(value, "type"), LogicalType::VARCHAR)});
}
inline LogicalType Receipt_recognize_question_document_item_schema_properties_entryType() {
return LogicalType::STRUCT({{"name", LogicalType::VARCHAR}, {"value", Receipt_input_property_typeType()}});
}
inline Value Receipt_recognize_question_document_item_schema_properties_entryValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_recognize_question_document_item_schema_properties_entryType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_recognize_question_document_item_schema_properties_entryType()); }
return Value::STRUCT(Receipt_recognize_question_document_item_schema_properties_entryType(), {ReceiptLeaf(ReceiptMember(value, "name"), LogicalType::VARCHAR), Receipt_input_property_typeValue(ReceiptMember(value, "value"))});
}
inline LogicalType Receipt_recognize_question_document_item_schemaType() {
return LogicalType::STRUCT({{"properties", LogicalType::LIST(Receipt_recognize_question_document_item_schema_properties_entryType())}, {"required", LogicalType::LIST(LogicalType::VARCHAR)}, {"type", LogicalType::VARCHAR}});
}
inline Value Receipt_recognize_question_document_item_schemaValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_recognize_question_document_item_schemaType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_recognize_question_document_item_schemaType()); }
return Value::STRUCT(Receipt_recognize_question_document_item_schemaType(), {ReceiptMapping(ReceiptMember(value, "properties"), Receipt_recognize_question_document_item_schema_properties_entryType(), [](const string &key, ReceiptJSON *item) { return Value::STRUCT(Receipt_recognize_question_document_item_schema_properties_entryType(), {Value(key), Receipt_input_property_typeValue(item)}); }), ReceiptList(ReceiptMember(value, "required"), LogicalType::VARCHAR, [](ReceiptJSON *item) { return ReceiptLeaf(item, LogicalType::VARCHAR); }), ReceiptLeaf(ReceiptMember(value, "type"), LogicalType::VARCHAR)});
}
inline LogicalType Receipt_recognize_question_document_kinds_entryType() {
return LogicalType::STRUCT({{"name", LogicalType::VARCHAR}, {"value", LogicalType::JSON()}, {"value_present", LogicalType::BOOLEAN}});
}
inline Value Receipt_recognize_question_document_kinds_entryValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_recognize_question_document_kinds_entryType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_recognize_question_document_kinds_entryType()); }
return Value::STRUCT(Receipt_recognize_question_document_kinds_entryType(), {ReceiptLeaf(ReceiptMember(value, "name"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "value"), LogicalType::JSON())});
}
inline LogicalType Receipt_recognize_question_document_relation_thresholdType() {
return LogicalType::STRUCT({{"number_value", LogicalType::DOUBLE}, {"string_value", LogicalType::VARCHAR}});
}
inline Value Receipt_recognize_question_document_relation_thresholdValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_recognize_question_document_relation_thresholdType()); }
return Value::STRUCT(Receipt_recognize_question_document_relation_thresholdType(), {ReceiptLeaf(value, LogicalType::DOUBLE), ReceiptLeaf(value, LogicalType::VARCHAR)});
}
inline LogicalType Receipt_relation_ruleType() {
return LogicalType::STRUCT({{"either", LogicalType::BOOLEAN}, {"name", LogicalType::VARCHAR}, {"reads", LogicalType::VARCHAR}, {"single", LogicalType::BOOLEAN}, {"source", LogicalType::VARCHAR}, {"target", LogicalType::VARCHAR}});
}
inline Value Receipt_relation_ruleValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_relation_ruleType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_relation_ruleType()); }
return Value::STRUCT(Receipt_relation_ruleType(), {ReceiptLeaf(ReceiptMember(value, "either"), LogicalType::BOOLEAN), ReceiptLeaf(ReceiptMember(value, "name"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "reads"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "single"), LogicalType::BOOLEAN), ReceiptLeaf(ReceiptMember(value, "source"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "target"), LogicalType::VARCHAR)});
}
inline LogicalType Receipt_recognition_stage_contextType() {
return LogicalType::STRUCT({{"boundary", LogicalType::VARCHAR}, {"kind_edge", LogicalType::VARCHAR}, {"relation", LogicalType::VARCHAR}});
}
inline Value Receipt_recognition_stage_contextValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_recognition_stage_contextType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_recognition_stage_contextType()); }
return Value::STRUCT(Receipt_recognition_stage_contextType(), {ReceiptLeaf(ReceiptMember(value, "boundary"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "kind_edge"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "relation"), LogicalType::VARCHAR)});
}
inline LogicalType Receipt_thresholdType() {
return LogicalType::STRUCT({{"number_value", LogicalType::DOUBLE}, {"string_value", LogicalType::VARCHAR}});
}
inline Value Receipt_thresholdValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_thresholdType()); }
return Value::STRUCT(Receipt_thresholdType(), {ReceiptLeaf(value, LogicalType::DOUBLE), ReceiptLeaf(value, LogicalType::VARCHAR)});
}
inline LogicalType Receipt_recognize_question_documentType() {
return LogicalType::STRUCT({{"batch", Receipt_recognize_question_document_batchType()}, {"context_schema", Receipt_recognize_question_document_context_schemaType()}, {"entity_definition", LogicalType::JSON()}, {"entity_definition_present", LogicalType::BOOLEAN}, {"instructions", LogicalType::JSON()}, {"instructions_present", LogicalType::BOOLEAN}, {"item_schema", Receipt_recognize_question_document_item_schemaType()}, {"kinds", LogicalType::LIST(Receipt_recognize_question_document_kinds_entryType())}, {"label_details", LogicalType::LIST(Receipt_labelType())}, {"mode", LogicalType::VARCHAR}, {"model", LogicalType::VARCHAR}, {"name", LogicalType::VARCHAR}, {"on", LogicalType::LIST(LogicalType::VARCHAR)}, {"profile", LogicalType::VARCHAR}, {"relation_threshold", Receipt_recognize_question_document_relation_thresholdType()}, {"relations", LogicalType::LIST(Receipt_relation_ruleType())}, {"snippet_pieces", LogicalType::BIGINT}, {"stage_context", Receipt_recognition_stage_contextType()}, {"threshold", Receipt_thresholdType()}, {"verb", LogicalType::VARCHAR}, {"wording_version", LogicalType::BIGINT}});
}
inline Value Receipt_recognize_question_documentValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_recognize_question_documentType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_recognize_question_documentType()); }
return Value::STRUCT(Receipt_recognize_question_documentType(), {Receipt_recognize_question_document_batchValue(ReceiptMember(value, "batch")), Receipt_recognize_question_document_context_schemaValue(ReceiptMember(value, "context_schema")), ReceiptLeaf(ReceiptMember(value, "entity_definition"), LogicalType::JSON()), Value::BOOLEAN(ReceiptMember(value, "entity_definition") != nullptr), ReceiptLeaf(ReceiptMember(value, "instructions"), LogicalType::JSON()), Value::BOOLEAN(ReceiptMember(value, "instructions") != nullptr), Receipt_recognize_question_document_item_schemaValue(ReceiptMember(value, "item_schema")), ReceiptMapping(ReceiptMember(value, "kinds"), Receipt_recognize_question_document_kinds_entryType(), [](const string &key, ReceiptJSON *item) { return Value::STRUCT(Receipt_recognize_question_document_kinds_entryType(), {Value(key), ReceiptLeaf(item, LogicalType::JSON()), Value::BOOLEAN(true)}); }), ReceiptList(ReceiptMember(value, "label_details"), Receipt_labelType(), [](ReceiptJSON *item) { return Receipt_labelValue(item); }), ReceiptLeaf(ReceiptMember(value, "mode"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "model"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "name"), LogicalType::VARCHAR), ReceiptList(ReceiptMember(value, "on"), LogicalType::VARCHAR, [](ReceiptJSON *item) { return ReceiptLeaf(item, LogicalType::VARCHAR); }), ReceiptLeaf(ReceiptMember(value, "profile"), LogicalType::VARCHAR), Receipt_recognize_question_document_relation_thresholdValue(ReceiptMember(value, "relation_threshold")), ReceiptList(ReceiptMember(value, "relations"), Receipt_relation_ruleType(), [](ReceiptJSON *item) { return Receipt_relation_ruleValue(item); }), ReceiptLeaf(ReceiptMember(value, "snippet_pieces"), LogicalType::BIGINT), Receipt_recognition_stage_contextValue(ReceiptMember(value, "stage_context")), Receipt_thresholdValue(ReceiptMember(value, "threshold")), ReceiptLeaf(ReceiptMember(value, "verb"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "wording_version"), LogicalType::BIGINT)});
}
inline LogicalType Receipt_recognition_sourceType() {
return LogicalType::STRUCT({{"file", LogicalType::VARCHAR}, {"first_line", LogicalType::UBIGINT}, {"last_line", LogicalType::UBIGINT}});
}
inline Value Receipt_recognition_sourceValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_recognition_sourceType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_recognition_sourceType()); }
return Value::STRUCT(Receipt_recognition_sourceType(), {ReceiptLeaf(ReceiptMember(value, "file"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "first_line"), LogicalType::UBIGINT), ReceiptLeaf(ReceiptMember(value, "last_line"), LogicalType::UBIGINT)});
}
inline LogicalType Receipt_entityType() {
return LogicalType::STRUCT({{"end", LogicalType::UBIGINT}, {"file", LogicalType::VARCHAR}, {"first_line", LogicalType::UBIGINT}, {"kind", LogicalType::VARCHAR}, {"last_line", LogicalType::UBIGINT}, {"length", LogicalType::UBIGINT}, {"start", LogicalType::UBIGINT}, {"strength", LogicalType::DOUBLE}, {"text", LogicalType::VARCHAR}});
}
inline Value Receipt_entityValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_entityType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_entityType()); }
return Value::STRUCT(Receipt_entityType(), {ReceiptLeaf(ReceiptMember(value, "end"), LogicalType::UBIGINT), ReceiptLeaf(ReceiptMember(value, "file"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "first_line"), LogicalType::UBIGINT), ReceiptLeaf(ReceiptMember(value, "kind"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "last_line"), LogicalType::UBIGINT), ReceiptLeaf(ReceiptMember(value, "length"), LogicalType::UBIGINT), ReceiptLeaf(ReceiptMember(value, "start"), LogicalType::UBIGINT), ReceiptLeaf(ReceiptMember(value, "strength"), LogicalType::DOUBLE), ReceiptLeaf(ReceiptMember(value, "text"), LogicalType::VARCHAR)});
}
inline LogicalType Receipt_boundary_proposalType() {
return LogicalType::STRUCT({{"end", LogicalType::UBIGINT}, {"length", LogicalType::UBIGINT}, {"probability", LogicalType::DOUBLE}, {"start", LogicalType::UBIGINT}, {"text", LogicalType::VARCHAR}});
}
inline Value Receipt_boundary_proposalValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_boundary_proposalType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_boundary_proposalType()); }
return Value::STRUCT(Receipt_boundary_proposalType(), {ReceiptLeaf(ReceiptMember(value, "end"), LogicalType::UBIGINT), ReceiptLeaf(ReceiptMember(value, "length"), LogicalType::UBIGINT), ReceiptLeaf(ReceiptMember(value, "probability"), LogicalType::DOUBLE), ReceiptLeaf(ReceiptMember(value, "start"), LogicalType::UBIGINT), ReceiptLeaf(ReceiptMember(value, "text"), LogicalType::VARCHAR)});
}
inline LogicalType Receipt_entity_edgeType() {
return LogicalType::STRUCT({{"either", LogicalType::BOOLEAN}, {"probability", LogicalType::DOUBLE}, {"relation", LogicalType::VARCHAR}, {"source", Receipt_entityType()}, {"target", Receipt_entityType()}});
}
inline Value Receipt_entity_edgeValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_entity_edgeType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_entity_edgeType()); }
return Value::STRUCT(Receipt_entity_edgeType(), {ReceiptLeaf(ReceiptMember(value, "either"), LogicalType::BOOLEAN), ReceiptLeaf(ReceiptMember(value, "probability"), LogicalType::DOUBLE), ReceiptLeaf(ReceiptMember(value, "relation"), LogicalType::VARCHAR), Receipt_entityValue(ReceiptMember(value, "source")), Receipt_entityValue(ReceiptMember(value, "target"))});
}
inline LogicalType Receipt_recognizeType() {
return LogicalType::STRUCT({{"entities", LogicalType::LIST(Receipt_entityType())}, {"mode", LogicalType::VARCHAR}, {"proposals", LogicalType::LIST(Receipt_boundary_proposalType())}, {"relations", LogicalType::LIST(Receipt_entity_edgeType())}});
}
inline Value Receipt_recognizeValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_recognizeType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_recognizeType()); }
return Value::STRUCT(Receipt_recognizeType(), {ReceiptList(ReceiptMember(value, "entities"), Receipt_entityType(), [](ReceiptJSON *item) { return Receipt_entityValue(item); }), ReceiptLeaf(ReceiptMember(value, "mode"), LogicalType::VARCHAR), ReceiptList(ReceiptMember(value, "proposals"), Receipt_boundary_proposalType(), [](ReceiptJSON *item) { return Receipt_boundary_proposalValue(item); }), ReceiptList(ReceiptMember(value, "relations"), Receipt_entity_edgeType(), [](ReceiptJSON *item) { return Receipt_entity_edgeValue(item); })});
}
inline LogicalType Receipt_recognizedType() {
return LogicalType::STRUCT({{"answer", Receipt_recognition_oddsType()}, {"answer_id", LogicalType::VARCHAR}, {"file", LogicalType::VARCHAR}, {"first_line", LogicalType::UBIGINT}, {"index", LogicalType::UBIGINT}, {"input", LogicalType::JSON()}, {"input_present", LogicalType::BOOLEAN}, {"last_line", LogicalType::UBIGINT}, {"meta", Receipt_metadataType()}, {"position", Receipt_positionType()}, {"question", Receipt_recognize_question_documentType()}, {"schema", LogicalType::VARCHAR}, {"source", Receipt_recognition_sourceType()}, {"value", Receipt_recognizeType()}});
}
inline Value Receipt_recognizedValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_recognizedType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_recognizedType()); }
return Value::STRUCT(Receipt_recognizedType(), {Receipt_recognition_oddsValue(ReceiptMember(value, "answer")), ReceiptLeaf(ReceiptMember(value, "answer_id"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "file"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "first_line"), LogicalType::UBIGINT), ReceiptLeaf(ReceiptMember(value, "index"), LogicalType::UBIGINT), ReceiptLeaf(ReceiptMember(value, "input"), LogicalType::JSON()), Value::BOOLEAN(ReceiptMember(value, "input") != nullptr), ReceiptLeaf(ReceiptMember(value, "last_line"), LogicalType::UBIGINT), Receipt_metadataValue(ReceiptMember(value, "meta")), Receipt_positionValue(ReceiptMember(value, "position")), Receipt_recognize_question_documentValue(ReceiptMember(value, "question")), ReceiptLeaf(ReceiptMember(value, "schema"), LogicalType::VARCHAR), Receipt_recognition_sourceValue(ReceiptMember(value, "source")), Receipt_recognizeValue(ReceiptMember(value, "value"))});
}
inline LogicalType Receipt_find_answer_probabilities_entryType() {
return LogicalType::STRUCT({{"name", LogicalType::VARCHAR}, {"value", LogicalType::DOUBLE}});
}
inline Value Receipt_find_answer_probabilities_entryValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_find_answer_probabilities_entryType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_find_answer_probabilities_entryType()); }
return Value::STRUCT(Receipt_find_answer_probabilities_entryType(), {ReceiptLeaf(ReceiptMember(value, "name"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "value"), LogicalType::DOUBLE)});
}
inline LogicalType Receipt_find_answerType() {
return LogicalType::STRUCT({{"confidence", LogicalType::DOUBLE}, {"kind", LogicalType::VARCHAR}, {"pick", LogicalType::VARCHAR}, {"probabilities", LogicalType::LIST(Receipt_find_answer_probabilities_entryType())}});
}
inline Value Receipt_find_answerValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_find_answerType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_find_answerType()); }
return Value::STRUCT(Receipt_find_answerType(), {ReceiptLeaf(ReceiptMember(value, "confidence"), LogicalType::DOUBLE), ReceiptLeaf(ReceiptMember(value, "kind"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "pick"), LogicalType::VARCHAR), ReceiptMapping(ReceiptMember(value, "probabilities"), Receipt_find_answer_probabilities_entryType(), [](const string &key, ReceiptJSON *item) { return Value::STRUCT(Receipt_find_answer_probabilities_entryType(), {Value(key), ReceiptLeaf(item, LogicalType::DOUBLE)}); })});
}
inline LogicalType Receipt_find_candidate_sourceType() {
return LogicalType::STRUCT({{"file", LogicalType::VARCHAR}, {"first_line", LogicalType::UBIGINT}, {"last_line", LogicalType::UBIGINT}});
}
inline Value Receipt_find_candidate_sourceValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_find_candidate_sourceType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_find_candidate_sourceType()); }
return Value::STRUCT(Receipt_find_candidate_sourceType(), {ReceiptLeaf(ReceiptMember(value, "file"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "first_line"), LogicalType::UBIGINT), ReceiptLeaf(ReceiptMember(value, "last_line"), LogicalType::UBIGINT)});
}
inline LogicalType Receipt_find_candidateType() {
return LogicalType::STRUCT({{"index", LogicalType::UBIGINT}, {"input", LogicalType::JSON()}, {"input_present", LogicalType::BOOLEAN}, {"probability", LogicalType::DOUBLE}, {"source", Receipt_find_candidate_sourceType()}});
}
inline Value Receipt_find_candidateValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_find_candidateType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_find_candidateType()); }
return Value::STRUCT(Receipt_find_candidateType(), {ReceiptLeaf(ReceiptMember(value, "index"), LogicalType::UBIGINT), ReceiptLeaf(ReceiptMember(value, "input"), LogicalType::JSON()), Value::BOOLEAN(ReceiptMember(value, "input") != nullptr), ReceiptLeaf(ReceiptMember(value, "probability"), LogicalType::DOUBLE), Receipt_find_candidate_sourceValue(ReceiptMember(value, "source"))});
}
inline LogicalType Receipt_find_question_document_batchType() {
return LogicalType::STRUCT({{"integer_value", LogicalType::UBIGINT}, {"string_value", LogicalType::VARCHAR}});
}
inline Value Receipt_find_question_document_batchValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_find_question_document_batchType()); }
return Value::STRUCT(Receipt_find_question_document_batchType(), {ReceiptLeaf(value, LogicalType::UBIGINT), ReceiptLeaf(value, LogicalType::VARCHAR)});
}
inline LogicalType Receipt_find_question_document_context_schema_properties_entryType() {
return LogicalType::STRUCT({{"name", LogicalType::VARCHAR}, {"value", Receipt_input_property_typeType()}});
}
inline Value Receipt_find_question_document_context_schema_properties_entryValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_find_question_document_context_schema_properties_entryType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_find_question_document_context_schema_properties_entryType()); }
return Value::STRUCT(Receipt_find_question_document_context_schema_properties_entryType(), {ReceiptLeaf(ReceiptMember(value, "name"), LogicalType::VARCHAR), Receipt_input_property_typeValue(ReceiptMember(value, "value"))});
}
inline LogicalType Receipt_find_question_document_context_schemaType() {
return LogicalType::STRUCT({{"properties", LogicalType::LIST(Receipt_find_question_document_context_schema_properties_entryType())}, {"required", LogicalType::LIST(LogicalType::VARCHAR)}, {"type", LogicalType::VARCHAR}});
}
inline Value Receipt_find_question_document_context_schemaValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_find_question_document_context_schemaType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_find_question_document_context_schemaType()); }
return Value::STRUCT(Receipt_find_question_document_context_schemaType(), {ReceiptMapping(ReceiptMember(value, "properties"), Receipt_find_question_document_context_schema_properties_entryType(), [](const string &key, ReceiptJSON *item) { return Value::STRUCT(Receipt_find_question_document_context_schema_properties_entryType(), {Value(key), Receipt_input_property_typeValue(item)}); }), ReceiptList(ReceiptMember(value, "required"), LogicalType::VARCHAR, [](ReceiptJSON *item) { return ReceiptLeaf(item, LogicalType::VARCHAR); }), ReceiptLeaf(ReceiptMember(value, "type"), LogicalType::VARCHAR)});
}
inline LogicalType Receipt_find_question_document_item_schema_properties_entryType() {
return LogicalType::STRUCT({{"name", LogicalType::VARCHAR}, {"value", Receipt_input_property_typeType()}});
}
inline Value Receipt_find_question_document_item_schema_properties_entryValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_find_question_document_item_schema_properties_entryType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_find_question_document_item_schema_properties_entryType()); }
return Value::STRUCT(Receipt_find_question_document_item_schema_properties_entryType(), {ReceiptLeaf(ReceiptMember(value, "name"), LogicalType::VARCHAR), Receipt_input_property_typeValue(ReceiptMember(value, "value"))});
}
inline LogicalType Receipt_find_question_document_item_schemaType() {
return LogicalType::STRUCT({{"properties", LogicalType::LIST(Receipt_find_question_document_item_schema_properties_entryType())}, {"required", LogicalType::LIST(LogicalType::VARCHAR)}, {"type", LogicalType::VARCHAR}});
}
inline Value Receipt_find_question_document_item_schemaValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_find_question_document_item_schemaType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_find_question_document_item_schemaType()); }
return Value::STRUCT(Receipt_find_question_document_item_schemaType(), {ReceiptMapping(ReceiptMember(value, "properties"), Receipt_find_question_document_item_schema_properties_entryType(), [](const string &key, ReceiptJSON *item) { return Value::STRUCT(Receipt_find_question_document_item_schema_properties_entryType(), {Value(key), Receipt_input_property_typeValue(item)}); }), ReceiptList(ReceiptMember(value, "required"), LogicalType::VARCHAR, [](ReceiptJSON *item) { return ReceiptLeaf(item, LogicalType::VARCHAR); }), ReceiptLeaf(ReceiptMember(value, "type"), LogicalType::VARCHAR)});
}
inline LogicalType Receipt_find_question_documentType() {
return LogicalType::STRUCT({{"batch", Receipt_find_question_document_batchType()}, {"context_schema", Receipt_find_question_document_context_schemaType()}, {"item_schema", Receipt_find_question_document_item_schemaType()}, {"label_details", LogicalType::LIST(Receipt_labelType())}, {"model", LogicalType::VARCHAR}, {"name", LogicalType::VARCHAR}, {"none", LogicalType::BOOLEAN}, {"on", LogicalType::LIST(LogicalType::VARCHAR)}, {"profile", LogicalType::VARCHAR}, {"text", LogicalType::JSON()}, {"text_present", LogicalType::BOOLEAN}, {"verb", LogicalType::VARCHAR}, {"wording_version", LogicalType::BIGINT}});
}
inline Value Receipt_find_question_documentValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_find_question_documentType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_find_question_documentType()); }
return Value::STRUCT(Receipt_find_question_documentType(), {Receipt_find_question_document_batchValue(ReceiptMember(value, "batch")), Receipt_find_question_document_context_schemaValue(ReceiptMember(value, "context_schema")), Receipt_find_question_document_item_schemaValue(ReceiptMember(value, "item_schema")), ReceiptList(ReceiptMember(value, "label_details"), Receipt_labelType(), [](ReceiptJSON *item) { return Receipt_labelValue(item); }), ReceiptLeaf(ReceiptMember(value, "model"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "name"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "none"), LogicalType::BOOLEAN), ReceiptList(ReceiptMember(value, "on"), LogicalType::VARCHAR, [](ReceiptJSON *item) { return ReceiptLeaf(item, LogicalType::VARCHAR); }), ReceiptLeaf(ReceiptMember(value, "profile"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "text"), LogicalType::JSON()), Value::BOOLEAN(ReceiptMember(value, "text") != nullptr), ReceiptLeaf(ReceiptMember(value, "verb"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "wording_version"), LogicalType::BIGINT)});
}
inline LogicalType Receipt_foundType() {
return LogicalType::STRUCT({{"answer", Receipt_find_answerType()}, {"answer_id", LogicalType::VARCHAR}, {"candidates", LogicalType::LIST(Receipt_find_candidateType())}, {"file", LogicalType::VARCHAR}, {"first_line", LogicalType::UBIGINT}, {"index", LogicalType::UBIGINT}, {"last_line", LogicalType::UBIGINT}, {"meta", Receipt_metadataType()}, {"position", Receipt_positionType()}, {"question", Receipt_find_question_documentType()}, {"schema", LogicalType::VARCHAR}, {"threshold", LogicalType::JSON()}, {"threshold_present", LogicalType::BOOLEAN}, {"value", LogicalType::JSON()}, {"value_present", LogicalType::BOOLEAN}});
}
inline Value Receipt_foundValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_foundType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_foundType()); }
return Value::STRUCT(Receipt_foundType(), {Receipt_find_answerValue(ReceiptMember(value, "answer")), ReceiptLeaf(ReceiptMember(value, "answer_id"), LogicalType::VARCHAR), ReceiptList(ReceiptMember(value, "candidates"), Receipt_find_candidateType(), [](ReceiptJSON *item) { return Receipt_find_candidateValue(item); }), ReceiptLeaf(ReceiptMember(value, "file"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "first_line"), LogicalType::UBIGINT), ReceiptLeaf(ReceiptMember(value, "index"), LogicalType::UBIGINT), ReceiptLeaf(ReceiptMember(value, "last_line"), LogicalType::UBIGINT), Receipt_metadataValue(ReceiptMember(value, "meta")), Receipt_positionValue(ReceiptMember(value, "position")), Receipt_find_question_documentValue(ReceiptMember(value, "question")), ReceiptLeaf(ReceiptMember(value, "schema"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "threshold"), LogicalType::JSON()), Value::BOOLEAN(ReceiptMember(value, "threshold") != nullptr), ReceiptLeaf(ReceiptMember(value, "value"), LogicalType::JSON()), Value::BOOLEAN(ReceiptMember(value, "value") != nullptr)});
}
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
inline LogicalType Receipt_session_observed_row_value_object_valueType() {
return LogicalType::STRUCT({{"entities", LogicalType::LIST(Receipt_entityType())}, {"kind", LogicalType::VARCHAR}, {"mode", LogicalType::VARCHAR}, {"proposals", LogicalType::LIST(Receipt_boundary_proposalType())}, {"relations", LogicalType::LIST(Receipt_recognition_edge_documentType())}, {"value", Receipt_session_observed_row_value_object_value_valueType()}});
}
inline Value Receipt_session_observed_row_value_object_valueValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_session_observed_row_value_object_valueType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_session_observed_row_value_object_valueType()); }
return Value::STRUCT(Receipt_session_observed_row_value_object_valueType(), {ReceiptList(ReceiptMember(value, "entities"), Receipt_entityType(), [](ReceiptJSON *item) { return Receipt_entityValue(item); }), ReceiptLeaf(ReceiptMember(value, "kind"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "mode"), LogicalType::VARCHAR), ReceiptList(ReceiptMember(value, "proposals"), Receipt_boundary_proposalType(), [](ReceiptJSON *item) { return Receipt_boundary_proposalValue(item); }), ReceiptList(ReceiptMember(value, "relations"), Receipt_recognition_edge_documentType(), [](ReceiptJSON *item) { return Receipt_recognition_edge_documentValue(item); }), Receipt_session_observed_row_value_object_value_valueValue(ReceiptMember(value, "value"))});
}
inline LogicalType Receipt_session_observed_row_valueType() {
return LogicalType::STRUCT({{"array_value", LogicalType::LIST(Receipt_session_observed_row_value_array_value_itemType())}, {"integer_value", LogicalType::UBIGINT}, {"object_value", Receipt_session_observed_row_value_object_valueType()}});
}
inline Value Receipt_session_observed_row_valueValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_session_observed_row_valueType()); }
return Value::STRUCT(Receipt_session_observed_row_valueType(), {ReceiptList(value, Receipt_session_observed_row_value_array_value_itemType(), [](ReceiptJSON *item) { return Receipt_session_observed_row_value_array_value_itemValue(item); }), ReceiptLeaf(value, LogicalType::UBIGINT), Receipt_session_observed_row_value_object_valueValue(value)});
}
inline LogicalType Receipt_observed_rowType() {
return LogicalType::STRUCT({{"kind", LogicalType::VARCHAR}, {"value", Receipt_session_observed_row_valueType()}});
}
inline Value Receipt_observed_rowValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_observed_rowType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_observed_rowType()); }
return Value::STRUCT(Receipt_observed_rowType(), {ReceiptLeaf(ReceiptMember(value, "kind"), LogicalType::VARCHAR), Receipt_session_observed_row_valueValue(ReceiptMember(value, "value"))});
}
inline LogicalType Receipt_eventType() {
return LogicalType::STRUCT({{"answer_id", LogicalType::VARCHAR}, {"author", Receipt_session_observation_authorType()}, {"detail", Receipt_question_detailType()}, {"failure_id", LogicalType::VARCHAR}, {"index", LogicalType::UBIGINT}, {"inputs", LogicalType::LIST(Receipt_observed_inputType())}, {"kind", LogicalType::VARCHAR}, {"member", LogicalType::VARCHAR}, {"observations", LogicalType::LIST(Receipt_observationType())}, {"position", LogicalType::UBIGINT}, {"question_sources", LogicalType::LIST(Receipt_question_sourceType())}, {"raw_pick", LogicalType::VARCHAR}, {"stage", LogicalType::VARCHAR}, {"value", Receipt_observed_rowType()}});
}
inline Value Receipt_eventValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_eventType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_eventType()); }
return Value::STRUCT(Receipt_eventType(), {ReceiptLeaf(ReceiptMember(value, "answer_id"), LogicalType::VARCHAR), Receipt_session_observation_authorValue(ReceiptMember(value, "author")), Receipt_question_detailValue(ReceiptMember(value, "detail")), ReceiptLeaf(ReceiptMember(value, "failure_id"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "index"), LogicalType::UBIGINT), ReceiptList(ReceiptMember(value, "inputs"), Receipt_observed_inputType(), [](ReceiptJSON *item) { return Receipt_observed_inputValue(item); }), ReceiptLeaf(ReceiptMember(value, "kind"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "member"), LogicalType::VARCHAR), ReceiptList(ReceiptMember(value, "observations"), Receipt_observationType(), [](ReceiptJSON *item) { return Receipt_observationValue(item); }), ReceiptLeaf(ReceiptMember(value, "position"), LogicalType::UBIGINT), ReceiptList(ReceiptMember(value, "question_sources"), Receipt_question_sourceType(), [](ReceiptJSON *item) { return Receipt_question_sourceValue(item); }), ReceiptLeaf(ReceiptMember(value, "raw_pick"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "stage"), LogicalType::VARCHAR), Receipt_observed_rowValue(ReceiptMember(value, "value"))});
}
