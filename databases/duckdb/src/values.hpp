// DuckDB owns every projected value before the engine row is released.
LogicalType EnumType(std::initializer_list<const char *> names) {
    Vector values(LogicalTypeId::VARCHAR, names.size());
    idx_t index = 0;
    for (auto name : names) values.SetValue(index++, Value(name));
    return LogicalType::ENUM(values, names.size());
}
#include "contract_types.hpp"
string Quote(const string &value) {
    auto doc = yyjson_mut_doc_new(nullptr);
    yyjson_mut_doc_set_root(doc, yyjson_mut_strncpy(doc, value.data(), value.size()));
    auto text = yyjson_mut_write(doc, 0, nullptr);
    string result(text);
    std::free(text); yyjson_mut_doc_free(doc);
    return result;
}
string Encode(const Value &value) {
    if (value.IsNull()) return "null";
    auto type = value.type();
    if (type.id() == LogicalTypeId::LIST) {
        string result = "[";
        for (auto &child : ListValue::GetChildren(value)) {
            if (result.size() > 1) result += ",";
            result += Encode(child);
        }
        return result + "]";
    }
    if (type.id() == LogicalTypeId::STRUCT) {
        string result = "{";
        auto &children = StructValue::GetChildren(value);
        for (idx_t i = 0; i < children.size(); ++i) {
            if (children[i].IsNull()) continue;
            if (result.size() > 1) result += ",";
            result += Quote(StructType::GetChildName(type, i)) + ":" + Encode(children[i]);
        }
        return result + "}";
    }
    if (type == LogicalType::JSON()) return value.GetValue<string>();
    if (type.id() == LogicalTypeId::VARCHAR || type.id() == LogicalTypeId::ENUM) return Quote(value.ToString());
    return value.ToString();
}
Document Parse(const string &text) {
    Document doc(yyjson_read(text.data(), text.size(), 0), yyjson_doc_free);
    if (!doc) throw InvalidInputException("usage.abi: malformed engine JSON");
    return doc;
}
Value Project(const LogicalType &type, yyjson_val *value) {
    if (!value || yyjson_is_null(value)) return Value(type);
    if (type == LogicalType::JSON()) {
        auto text = yyjson_val_write(value, 0, nullptr);
        string result(text); std::free(text);
        return Value(result).DefaultCastAs(type);
    }
    switch (type.id()) {
    case LogicalTypeId::STRUCT: {
        vector<Value> children;
        for (auto &child : StructType::GetChildTypes(type)) children.push_back(Project(child.second, yyjson_obj_get(value, child.first.c_str())));
        return Value::STRUCT(type, std::move(children));
    }
    case LogicalTypeId::LIST: {
        vector<Value> children;
        size_t index, size; yyjson_val *child;
        yyjson_arr_foreach(value, index, size, child) children.push_back(Project(ListType::GetChildType(type), child));
        return Value::LIST(ListType::GetChildType(type), std::move(children));
    }
    case LogicalTypeId::MAP: {
        vector<Value> keys, values;
        size_t index, size; yyjson_val *key, *child;
        yyjson_obj_foreach(value, index, size, key, child) {
            keys.emplace_back(yyjson_get_str(key)); values.push_back(Project(MapType::ValueType(type), child));
        }
        return Value::MAP(MapType::KeyType(type), MapType::ValueType(type), std::move(keys), std::move(values));
    }
    case LogicalTypeId::ENUM:
        try { return Value(yyjson_get_str(value)).DefaultCastAs(type); }
        catch (const Exception &) { throw InvalidInputException("usage.abi: unknown engine enum value"); }
    case LogicalTypeId::VARCHAR: return Value(string(yyjson_get_str(value), yyjson_get_len(value)));
    case LogicalTypeId::BOOLEAN: return Value::BOOLEAN(yyjson_get_bool(value));
    case LogicalTypeId::DOUBLE: return Value::DOUBLE(yyjson_get_num(value));
    case LogicalTypeId::BIGINT: return Value::BIGINT(yyjson_get_sint(value));
    default: throw InvalidInputException("usage.abi: unexpected engine value type");
    }
}
Value Project(const LogicalType &type, const thinkthen_buffer &buffer) {
    if (!buffer.data) return Value(type);
    auto doc = Parse(Text(buffer));
    return Project(type, yyjson_doc_get_root(doc.get()));
}
LogicalType AnnotationType(yyjson_val *questions) {
    child_list_t<LogicalType> fields;
    size_t index, size; yyjson_val *key, *child;
    yyjson_obj_foreach(questions, index, size, key, child) {
        auto function = yyjson_get_str(yyjson_obj_get(child, "function"));
        if (!function) throw BinderException("usage.question: missing annotation function");
        fields.emplace_back(yyjson_get_str(key), LogicalType::STRUCT({
            {"function", LogicalTypeId::VARCHAR}, {"status", StatusType()},
            {"answer", AnswerType(function)}, {"probability", LogicalTypeId::DOUBLE},
            {"error", LogicalType::STRUCT({{"code", CodeType()}, {"message", LogicalTypeId::VARCHAR}, {"http_status", LogicalTypeId::BIGINT}})},
            {"detail", DetailType(function)}}));
    }
    return LogicalType::STRUCT(std::move(fields));
}
