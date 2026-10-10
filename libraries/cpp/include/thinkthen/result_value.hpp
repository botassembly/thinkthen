#pragma once
#include "json.hpp"
#include <optional>
#include <cstdint>
namespace tt::results {
enum class PresenceState { absent, null, value };
template<class T> struct Presence {
    PresenceState state = PresenceState::absent;
    std::optional<T> value;
};
class Node {
protected:
    Json value_;
    void set_member(const char* key, Json value) {
        auto members=value_.object();
        for(auto& member:members) if(member.first==key) {
            member.second=std::move(value); value_=Json(std::move(members)); return;
        }
        members.emplace_back(key,std::move(value)); value_=Json(std::move(members));
    }
public:
    explicit Node(Json value):value_(std::move(value)) {}
    // Preserve unknown extensions and caller-owned original JSON without imposing a grammar.
    const Json& document() const noexcept { return value_; }
};
inline bool literal(const Json& value, const char* key, const char* expected) {
    return value.contains(key) && value.at(key).is_string() && value.at(key).get<std::string>() == expected;
}
template<class T> Json encode(const T& value);
template<class T> struct Encoder {
    static Json write(const T& value) {
        if constexpr(std::is_base_of_v<Node,T>) return value.document();
        else return Json(value);
    }
};
template<class T> struct Encoder<std::vector<T>> {
    static Json write(const std::vector<T>& value) { Json::Array array; for(const auto& item:value) array.push_back(encode(item)); return Json(std::move(array)); }
};
template<class T> struct Encoder<std::vector<std::pair<std::string,T>>> {
    static Json write(const std::vector<std::pair<std::string,T>>& value) { Json::Object object; for(const auto& [key,item]:value) object.emplace_back(key,encode(item)); return Json(std::move(object)); }
};
template<class... T> struct Encoder<std::variant<T...>> {
    static Json write(const std::variant<T...>& value) { return std::visit([](const auto& item){ return encode(item); },value); }
};
template<class T> Json encode(const T& value) { return Encoder<T>::write(value); }
template<class T> struct Decoder {
    static T read(const Json& value) {
        if constexpr(std::is_same_v<T,std::nullptr_t>) return nullptr;
        else if constexpr(std::is_same_v<T,Json>) return value;
        else if constexpr(std::is_arithmetic_v<T> || std::is_same_v<T,std::string>) return value.get<T>();
        else return T(value);
    }
    static bool matches(const Json& value) {
        if constexpr(std::is_same_v<T,bool>) return value.is_boolean();
        else if constexpr(std::is_arithmetic_v<T>) return value.is_number();
        else if constexpr(std::is_same_v<T,std::string>) return value.is_string();
        else return value.is_object();
    }
};
template<class T> T decode(const Json& value) { return Decoder<T>::read(value); }
template<class T> struct Decoder<std::vector<T>> {
    static std::vector<T> read(const Json& value) {
        std::vector<T> result; for(const auto& item:value) result.push_back(decode<T>(item)); return result;
    }
    static bool matches(const Json& value) { return value.is_array(); }
};
template<class T> struct Decoder<std::vector<std::pair<std::string,T>>> {
    static std::vector<std::pair<std::string,T>> read(const Json& value) {
        std::vector<std::pair<std::string,T>> result;
        for(const auto& [key,item]:value.object()) result.emplace_back(key,decode<T>(item));
        return result;
    }
    static bool matches(const Json& value) { return value.is_object(); }
};
template<class... T> struct Decoder<std::variant<T...>> {
    static std::variant<T...> read(const Json& value) {
        std::optional<std::variant<T...>> result;
        ([&]{ if(!result && Decoder<T>::matches(value)) result.emplace(std::in_place_type<T>,decode<T>(value)); }(),...);
        if(!result) throw std::invalid_argument("native value has no C++ alternative");
        return std::move(*result);
    }
};
template<class T> Presence<T> member_value(const Json& value, const char* key) {
    if(!value.contains(key)) return {};
    if(value.at(key).is_null()) return {PresenceState::null,{}};
    return {PresenceState::value,decode<T>(value.at(key))};
}
}
