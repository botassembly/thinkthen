#pragma once
#include "receipt.hpp"
namespace duckdb {
inline LogicalType DetailsRecognitionItemType() {
    auto fields = StructType::GetChildTypes(Receipt_entityType());
    fields.emplace_back("probability", LogicalType(LogicalTypeId::DOUBLE));
    return LogicalType::STRUCT(std::move(fields));
}
inline LogicalType DetailsValueType(const string &verb) {
    if (verb == "filter") { return LogicalType::JSON(); }
    if (verb == "recognize") { return LogicalType::LIST(DetailsRecognitionItemType()); }
    if (verb == "decide" || verb == "choose" || verb == "score" || verb == "tag" || verb == "rank") {
        return ReceiptAnswerType(verb);
    }
    if (verb == "relate") { return Receipt_related_entity_edgeType(); }
    const auto result_type = ReceiptResultType(verb);
    for (const auto &member : StructType::GetChildTypes(result_type)) {
        if (member.first == "value") { return member.second; }
    }
    throw OrdinaryError("thinkthen defect: details result has no value");
}
inline LogicalType DetailsProbabilityType(const string &verb) {
    return verb == "recognize" ? LogicalType::LIST(LogicalType::DOUBLE) : LogicalType::DOUBLE;
}
inline LogicalType DetailsFailureType() {
    auto fields = StructType::GetChildTypes(Receipt_errorType());
    fields.emplace_back("cause", LogicalType(LogicalTypeId::VARCHAR));
    return LogicalType::STRUCT(std::move(fields));
}
inline Value DetailsFailureValue(ReceiptJSON *value) {
    if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(DetailsFailureType()); }
    auto ordinary = Receipt_errorValue(value);
    auto fields = StructValue::GetChildren(ordinary);
    fields.push_back(ReceiptLeaf(ReceiptMember(value, "cause"), LogicalType::VARCHAR));
    return Value::STRUCT(DetailsFailureType(), std::move(fields));
}
inline LogicalType DetailsType(const string &verb) {
    return LogicalType::STRUCT({{"ordinal", LogicalType::UBIGINT}, {"value", DetailsValueType(verb)},
        {"probability", DetailsProbabilityType(verb)}, {"result", ReceiptResultType(verb)},
        {"failure", DetailsFailureType()}, {"facts", Receipt_factsType()},
        {"observations", LogicalType::LIST(Receipt_eventType())}});
}
inline Value DetailsValue(ReceiptJSON *value, const string &verb) {
    if (verb == "filter") { return ReceiptLeaf(value, LogicalType::JSON()); }
    if (verb == "decide" || verb == "choose" || verb == "score" || verb == "tag" || verb == "rank") {
        return ReceiptAnswerValue(verb, value);
    }
    if (verb == "annotate") {
        return ReceiptMapping(value, Receipt_annotated_row_entryType(), [](const string &key, ReceiptJSON *item) {
            return Value::STRUCT(Receipt_annotated_row_entryType(), {Value(key), Receipt_annotated_fieldValue(item)});
        });
    }
    if (verb == "recognize") {
        return ReceiptList(value, DetailsRecognitionItemType(), [](ReceiptJSON *item) {
            auto entity = Receipt_entityValue(item);
            auto fields = StructValue::GetChildren(entity);
            fields.push_back(ReceiptLeaf(ReceiptMember(item, "probability"), LogicalType::DOUBLE));
            return Value::STRUCT(DetailsRecognitionItemType(), std::move(fields));
        });
    }
    if (verb == "relate") { return Receipt_related_entity_edgeValue(value); }
    return ReceiptLeaf(value, DetailsValueType(verb));
}
inline Value DetailsRow(ReceiptJSON *row, const string &verb) {
    auto probability = ReceiptMember(row, "probability");
    auto odds = DetailsProbabilityType(verb).id() == LogicalTypeId::LIST
        ? ReceiptList(probability, LogicalType::DOUBLE, [](ReceiptJSON *item) { return ReceiptLeaf(item, LogicalType::DOUBLE); })
        : ReceiptLeaf(probability, LogicalType::DOUBLE);
    return Value::STRUCT(DetailsType(verb), {
        ReceiptLeaf(ReceiptMember(row, "ordinal"), LogicalType::UBIGINT),
        DetailsValue(ReceiptMember(row, "value"), verb), std::move(odds),
        ReceiptResultValue(verb, ReceiptMember(row, "result")), DetailsFailureValue(ReceiptMember(row, "failure")),
        Receipt_factsValue(ReceiptMember(row, "facts")),
        ReceiptList(ReceiptMember(row, "observations"), Receipt_eventType(), [](ReceiptJSON *item) { return Receipt_eventValue(item); })});
}
inline Value DecodeDetails(const string &text, const string &verb, bool collection) {
    auto doc = duckdb_yyjson::yyjson_read(text.data(), text.size(), 0);
    if (!doc) { throw OrdinaryError("thinkthen defect: details could not be decoded"); }
    struct Held { duckdb_yyjson::yyjson_doc *doc; ~Held() { duckdb_yyjson::yyjson_doc_free(doc); } } held {doc};
    auto root = duckdb_yyjson::yyjson_doc_get_root(doc);
    auto rows = ReceiptMember(root, "rows");
    if (!rows) {
        auto native = ReceiptMember(root, "native");
        auto failure = DetailsFailureValue(ReceiptMember(native, "error"));
        auto fields = StructValue::GetChildren(DetailsRow(nullptr, verb));
        fields[4] = std::move(failure);
        auto row = Value::STRUCT(DetailsType(verb), std::move(fields));
        return collection ? Value::LIST(DetailsType(verb), {row}) : row;
    }
    if (!collection) { return DetailsRow(duckdb_yyjson::yyjson_arr_get(rows, 0), verb); }
    return ReceiptList(rows, DetailsType(verb), [&](ReceiptJSON *row) { return DetailsRow(row, verb); });
}
} // namespace duckdb
