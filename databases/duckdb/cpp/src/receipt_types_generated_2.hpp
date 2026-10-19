// Generated from the canonical native result schema; do not edit.
#pragma once
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
return LogicalType::STRUCT({{"end", LogicalType::UBIGINT}, {"file", LogicalType::VARCHAR}, {"first_line", LogicalType::UBIGINT}, {"kept", LogicalType::BOOLEAN}, {"kind", LogicalType::VARCHAR}, {"last_line", LogicalType::UBIGINT}, {"length", LogicalType::UBIGINT}, {"probability", LogicalType::DOUBLE}, {"selected", Receipt_recognition_odds_proposals_item_selectedType()}, {"span_probability", LogicalType::DOUBLE}, {"start", LogicalType::UBIGINT}, {"strength", LogicalType::DOUBLE}, {"text", LogicalType::VARCHAR}});
}
inline Value Receipt_recognition_odds_proposals_itemValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_recognition_odds_proposals_itemType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_recognition_odds_proposals_itemType()); }
return Value::STRUCT(Receipt_recognition_odds_proposals_itemType(), {ReceiptLeaf(ReceiptMember(value, "end"), LogicalType::UBIGINT), ReceiptLeaf(ReceiptMember(value, "file"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "first_line"), LogicalType::UBIGINT), ReceiptLeaf(ReceiptMember(value, "kept"), LogicalType::BOOLEAN), ReceiptLeaf(ReceiptMember(value, "kind"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "last_line"), LogicalType::UBIGINT), ReceiptLeaf(ReceiptMember(value, "length"), LogicalType::UBIGINT), ReceiptLeaf(ReceiptMember(value, "probability"), LogicalType::DOUBLE), Receipt_recognition_odds_proposals_item_selectedValue(ReceiptMember(value, "selected")), ReceiptLeaf(ReceiptMember(value, "span_probability"), LogicalType::DOUBLE), ReceiptLeaf(ReceiptMember(value, "start"), LogicalType::UBIGINT), ReceiptLeaf(ReceiptMember(value, "strength"), LogicalType::DOUBLE), ReceiptLeaf(ReceiptMember(value, "text"), LogicalType::VARCHAR)});
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
return LogicalType::STRUCT({{"end", LogicalType::UBIGINT}, {"file", LogicalType::VARCHAR}, {"first_line", LogicalType::UBIGINT}, {"last_line", LogicalType::UBIGINT}, {"length", LogicalType::UBIGINT}, {"probability", LogicalType::DOUBLE}, {"start", LogicalType::UBIGINT}, {"text", LogicalType::VARCHAR}});
}
inline Value Receipt_boundary_proposalValue(ReceiptJSON *value) {
if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(Receipt_boundary_proposalType()); }
if (!duckdb_yyjson::yyjson_is_obj(value)) { return Value(Receipt_boundary_proposalType()); }
return Value::STRUCT(Receipt_boundary_proposalType(), {ReceiptLeaf(ReceiptMember(value, "end"), LogicalType::UBIGINT), ReceiptLeaf(ReceiptMember(value, "file"), LogicalType::VARCHAR), ReceiptLeaf(ReceiptMember(value, "first_line"), LogicalType::UBIGINT), ReceiptLeaf(ReceiptMember(value, "last_line"), LogicalType::UBIGINT), ReceiptLeaf(ReceiptMember(value, "length"), LogicalType::UBIGINT), ReceiptLeaf(ReceiptMember(value, "probability"), LogicalType::DOUBLE), ReceiptLeaf(ReceiptMember(value, "start"), LogicalType::UBIGINT), ReceiptLeaf(ReceiptMember(value, "text"), LogicalType::VARCHAR)});
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
