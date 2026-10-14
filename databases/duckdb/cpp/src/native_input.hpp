#pragma once

#include "bridge.hpp"
#include "images.hpp"
#include "duckdb/common/types/blob.hpp"
#include "yyjson.hpp"

namespace duckdb {
inline duckdb_yyjson::yyjson_mut_val *NativeInputValue(duckdb_yyjson::yyjson_mut_doc *doc, const Value &value, bool record = false) {
    using namespace duckdb_yyjson;
    if (value.IsNull()) { return yyjson_mut_null(doc); }
    if (value.type().id() == LogicalTypeId::STRUCT) {
        auto object = yyjson_mut_obj(doc);
        auto &children = StructValue::GetChildren(value);
        for (idx_t index = 0; index < children.size(); ++index) {
            auto name = ThinkThenStructName(value.type(), index);
            if (record && name == "images" && !children[index].IsNull()) {
                if (children[index].type() != LogicalType::LIST(ImageType())) {
                    throw OrdinaryError("thinkthen usage: record images require native image values");
                }
                if (!ListValue::GetChildren(children[index]).empty()) { ImageMembers(children[index]); }
            }
            yyjson_mut_obj_add(object, yyjson_mut_strncpy(doc, name.data(), name.size()), NativeInputValue(doc, children[index]));
            if (record && (name == "json" || name == "context" || name == "options" || name == "examples" || name == "seed_spans")) {
                auto marker = name + "_present";
                yyjson_mut_obj_add(object, yyjson_mut_strncpy(doc, marker.data(), marker.size()), yyjson_mut_bool(doc, !children[index].IsNull()));
            }
        }
        return object;
    }
    if (value.type().id() == LogicalTypeId::MAP) {
        auto object = yyjson_mut_obj(doc);
        for (auto &entry : MapValue::GetChildren(value)) {
            auto &children = StructValue::GetChildren(entry);
            auto name = children[0].GetValue<string>();
            yyjson_mut_obj_add(object, yyjson_mut_strncpy(doc, name.data(), name.size()), NativeInputValue(doc, children[1]));
            if (record && (name == "json" || name == "context" || name == "options" || name == "examples" || name == "seed_spans")) {
                auto marker = name + "_present";
                yyjson_mut_obj_add(object, yyjson_mut_strncpy(doc, marker.data(), marker.size()), yyjson_mut_bool(doc, !children[1].IsNull()));
            }
        }
        return object;
    }
    if (value.type().id() == LogicalTypeId::LIST || value.type().id() == LogicalTypeId::ARRAY) {
        auto list = yyjson_mut_arr(doc);
        auto &children = value.type().id() == LogicalTypeId::LIST ? ListValue::GetChildren(value) : ArrayValue::GetChildren(value);
        for (auto &child : children) { yyjson_mut_arr_append(list, NativeInputValue(doc, child, record)); }
        return list;
    }
    if (value.type().id() == LogicalTypeId::BLOB) {
        auto &bytes = StringValue::Get(value);
        auto encoded = Blob::ToBase64(string_t(bytes.data(), bytes.size()));
        return yyjson_mut_strncpy(doc, encoded.data(), encoded.size());
    }
    if (value.type() != LogicalType::JSON() && value.type().id() == LogicalTypeId::VARCHAR) {
        auto &text = StringValue::Get(value);
        return yyjson_mut_strncpy(doc, text.data(), text.size());
    }
    auto raw = value.type() == LogicalType::JSON() ? StringValue::Get(value) : value.ToString();
    auto parsed = yyjson_read(raw.data(), raw.size(), 0);
    if (!parsed) { throw OrdinaryError("thinkthen usage: native input accepts text, JSON, booleans, numbers and ordered collections"); }
    auto copied = yyjson_val_mut_copy(doc, yyjson_doc_get_root(parsed));
    yyjson_doc_free(parsed);
    return copied;
}
inline string NativeInputJSON(const Value &value, bool records = false) {
    auto doc = duckdb_yyjson::yyjson_mut_doc_new(nullptr);
    if (!doc) { throw OrdinaryError("thinkthen defect: native input could not be copied"); }
    struct Held { duckdb_yyjson::yyjson_mut_doc *doc; ~Held() { duckdb_yyjson::yyjson_mut_doc_free(doc); } } held {doc};
    duckdb_yyjson::yyjson_mut_doc_set_root(doc, NativeInputValue(doc, value, records));
    auto bytes = duckdb_yyjson::yyjson_mut_write(doc, 0, nullptr);
    if (!bytes) { throw OrdinaryError("thinkthen defect: native input could not be copied"); }
    string result(bytes);
    free(bytes);
    return result;
}
} // namespace duckdb
