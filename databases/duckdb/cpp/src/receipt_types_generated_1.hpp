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
