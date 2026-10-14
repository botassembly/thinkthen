#pragma once

#include "bridge.hpp"
#include "json_result.hpp"
#include "yyjson.hpp"

namespace duckdb {
using ReceiptJSON = duckdb_yyjson::yyjson_val;
inline ReceiptJSON *ReceiptMember(ReceiptJSON *value, const char *name) {
    return value ? duckdb_yyjson::yyjson_obj_get(value, name) : nullptr;
}
inline Value ReceiptLeaf(ReceiptJSON *value, const LogicalType &type) {
    if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(type); }
    if (type.id() == LogicalTypeId::BOOLEAN && duckdb_yyjson::yyjson_is_bool(value)) {
        return Value::BOOLEAN(duckdb_yyjson::yyjson_get_bool(value));
    }
    if (type.id() == LogicalTypeId::BIGINT && duckdb_yyjson::yyjson_is_int(value)) {
        return Value::BIGINT(duckdb_yyjson::yyjson_get_sint(value));
    }
    if (type.id() == LogicalTypeId::UBIGINT && duckdb_yyjson::yyjson_is_int(value)) {
        return Value::UBIGINT(duckdb_yyjson::yyjson_get_uint(value));
    }
    if (type.id() == LogicalTypeId::DOUBLE && duckdb_yyjson::yyjson_is_num(value)) {
        return Value::DOUBLE(duckdb_yyjson::yyjson_get_num(value));
    }
    if (type == LogicalType::JSON()) {
        auto raw = duckdb_yyjson::yyjson_val_write(value, 0, nullptr);
        if (!raw) { throw OrdinaryError("thinkthen defect: authored receipt value could not be projected"); }
        string text(raw);
        free(raw);
        return ThinkThenJSON(text);
    }
    if (type.id() == LogicalTypeId::VARCHAR && duckdb_yyjson::yyjson_is_str(value)) {
        return Value(string(duckdb_yyjson::yyjson_get_str(value), duckdb_yyjson::yyjson_get_len(value)));
    }
    return Value(type); // In a typed union, only the actual native arm is populated.
}
template <class Convert>
Value ReceiptList(ReceiptJSON *value, const LogicalType &type, Convert convert) {
    if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(LogicalType::LIST(type)); }
    if (!duckdb_yyjson::yyjson_is_arr(value)) { return Value(LogicalType::LIST(type)); }
    vector<Value> values;
    size_t index, count;
    ReceiptJSON *item;
    yyjson_arr_foreach(value, index, count, item) { values.push_back(convert(item)); }
    return Value::LIST(type, std::move(values));
}
template <class Convert>
Value ReceiptMapping(ReceiptJSON *value, const LogicalType &type, Convert convert) {
    if (!value || duckdb_yyjson::yyjson_is_null(value)) { return Value(LogicalType::LIST(type)); }
    if (!duckdb_yyjson::yyjson_is_obj(value)) { throw OrdinaryError("thinkthen defect: native receipt map has another type"); }
    vector<Value> values;
    size_t index, count;
    ReceiptJSON *key, *item;
    yyjson_obj_foreach(value, index, count, key, item) {
        values.push_back(convert(string(duckdb_yyjson::yyjson_get_str(key), duckdb_yyjson::yyjson_get_len(key)), item));
    }
    return Value::LIST(type, std::move(values));
}
#include "receipt_types_generated.hpp"
inline LogicalType ReceiptResultType(const string &verb) {
    if (verb == "decide") { return Receipt_decisionType(); }
    if (verb == "choose") { return Receipt_choiceType(); }
    if (verb == "tag") { return Receipt_tagsType(); }
    if (verb == "score") { return Receipt_scoreType(); }
    if (verb == "filter") { return Receipt_filteredType(); }
    if (verb == "rank") { return Receipt_rankedType(); }
    if (verb == "find") { return Receipt_foundType(); }
    if (verb == "annotate") { return Receipt_annotationType(); }
    if (verb == "recognize") { return Receipt_recognizedType(); }
    return Receipt_relatedType();
}
inline Value ReceiptResultValue(const string &verb, ReceiptJSON *value) {
    if (verb == "decide") { return Receipt_decisionValue(value); }
    if (verb == "choose") { return Receipt_choiceValue(value); }
    if (verb == "tag") { return Receipt_tagsValue(value); }
    if (verb == "score") { return Receipt_scoreValue(value); }
    if (verb == "filter") { return Receipt_filteredValue(value); }
    if (verb == "rank") { return Receipt_rankedValue(value); }
    if (verb == "find") { return Receipt_foundValue(value); }
    if (verb == "annotate") { return Receipt_annotationValue(value); }
    if (verb == "recognize") { return Receipt_recognizedValue(value); }
    return Receipt_relatedValue(value);
}
inline LogicalType ReceiptAnswerType(const string &verb) {
    if (verb == "decide" || verb == "filter") { return LogicalType::BOOLEAN; }
    if (verb == "choose") { return LogicalType::VARCHAR; }
    if (verb == "tag") { return LogicalType::LIST(LogicalType::VARCHAR); }
    if (verb == "score") { return LogicalType::DOUBLE; }
    if (verb == "rank") { return LogicalType::UBIGINT; }
    return LogicalType::JSON();
}
inline Value ReceiptAnswerValue(const string &verb, ReceiptJSON *value) {
    if (verb == "tag") {
        return ReceiptList(value, LogicalType::VARCHAR, [](ReceiptJSON *item) { return ReceiptLeaf(item, LogicalType::VARCHAR); });
    }
    return ReceiptLeaf(value, ReceiptAnswerType(verb));
}
inline LogicalType ReceiptType(const string &verb) {
    return LogicalType::STRUCT({{"results", LogicalType::LIST(ReceiptResultType(verb))},
        {"answers", LogicalType::LIST(ReceiptAnswerType(verb))},
        {"facts", Receipt_factsType()}, {"failure", Receipt_errorType()},
        {"observations", LogicalType::LIST(Receipt_eventType())},
        {"ordinals", LogicalType::LIST(LogicalType::UBIGINT)}, {"selection", LogicalType::UBIGINT}});
}
inline Value DecodeReceipt(const string &text, const string &verb) {
    auto doc = duckdb_yyjson::yyjson_read(text.data(), text.size(), 0);
    if (!doc) { throw OrdinaryError("thinkthen defect: native receipt could not be decoded"); }
    struct Held { duckdb_yyjson::yyjson_doc *doc; ~Held() { duckdb_yyjson::yyjson_doc_free(doc); } } held {doc};
    auto root = duckdb_yyjson::yyjson_doc_get_root(doc);
    auto native = ReceiptMember(root, "native");
    auto results = ReceiptMember(native, "value");
    if (!results) { results = ReceiptMember(root, "completed"); }
    vector<Value> values;
    if (results && duckdb_yyjson::yyjson_is_arr(results)) {
        size_t index, count; ReceiptJSON *item;
        yyjson_arr_foreach(results, index, count, item) { values.push_back(ReceiptResultValue(verb, item)); }
    } else if (results) { values.push_back(ReceiptResultValue(verb, results)); }
    return Value::STRUCT(ReceiptType(verb), {
        Value::LIST(ReceiptResultType(verb), std::move(values)),
        ReceiptList(ReceiptMember(root, "answers"), ReceiptAnswerType(verb), [&](ReceiptJSON *value) { return ReceiptAnswerValue(verb, value); }), Receipt_factsValue(ReceiptMember(native, "facts")),
        Receipt_errorValue(ReceiptMember(native, "error")),
        ReceiptList(ReceiptMember(root, "observations"), Receipt_eventType(), [](ReceiptJSON *value) { return Receipt_eventValue(value); }),
        ReceiptList(ReceiptMember(root, "ordinals"), LogicalType::UBIGINT, [](ReceiptJSON *value) { return ReceiptLeaf(value, LogicalType::UBIGINT); }),
        ReceiptLeaf(ReceiptMember(root, "selection"), LogicalType::UBIGINT)});
}
} // namespace duckdb
