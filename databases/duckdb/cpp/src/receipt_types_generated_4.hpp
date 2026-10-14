// Generated from the canonical native result schema; do not edit.
#pragma once
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
