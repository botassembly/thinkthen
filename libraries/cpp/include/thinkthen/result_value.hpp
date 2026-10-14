#pragma once
#include "value.hpp"
#include <optional>
#include <cstdint>
namespace tt::results {
class Node;
namespace detail { const Data& document(const Node& value); }
enum class PresenceState { absent, null, value };
template<class T> struct Presence {
    PresenceState state = PresenceState::absent;
    std::optional<T> value;
};
class Node {
    friend const Data& detail::document(const Node& value);
protected:
    Node(const Node&) = default;
    Node(Node&&) = default;
    Node& operator=(const Node&) = default;
    Node& operator=(Node&&) = default;
    Data value_;
    void set_member(const char* key, Data value) {
        auto members=value_.object();
        for(auto& member:members) if(member.first==key) {
            member.second=std::move(value); value_=Data(std::move(members)); return;
        }
        members.emplace_back(key,std::move(value)); value_=Data(std::move(members));
    }
    explicit Node(Data value):value_(std::move(value)) {}
    // Preserve unknown extensions and caller-owned original JSON without imposing a grammar.
};
namespace detail {
inline const Data& document(const Node& value) { return value.value_; }
// Unsupported bridge for native decoding and result fixtures; callers author inputs instead.
struct Loader { template<class T> static T input(const Data& value) { return T(value,0); } };
inline bool literal(const Data& value, const char* key, const char* expected) {
    return value.contains(key) && value.at(key).is_string() && value.at(key).get<std::string>() == expected;
}
template<class T> struct Encoder {
    static Data write(const T& value) {
        if constexpr(std::is_base_of_v<Node,T>) return document(value);
        else return Data(value);
    }
};
template<class T> struct Encoder<std::optional<T>> {
    static Data write(const std::optional<T>& value) { return value ? Encoder<T>::write(*value) : Data(nullptr); }
};
template<class T> struct Encoder<std::vector<T>> {
    static Data write(const std::vector<T>& value) { Data::Array array; for(const auto& item:value) array.push_back(Encoder<T>::write(item)); return Data(std::move(array)); }
};
template<class T> struct Encoder<std::vector<std::pair<std::string,T>>> {
    static Data write(const std::vector<std::pair<std::string,T>>& value) { Data::Object object; for(const auto& [key,item]:value) object.emplace_back(key,Encoder<T>::write(item)); return Data(std::move(object)); }
};
template<class... T> struct Encoder<std::variant<T...>> {
    static Data write(const std::variant<T...>& value) { return std::visit([](const auto& item){ return Encoder<std::decay_t<decltype(item)>>::write(item); },value); }
};
template<class T> struct Decoder {
    static T read(const Data& value) {
        if constexpr(std::is_same_v<T,std::nullptr_t>) return nullptr;
        else if constexpr(std::is_same_v<T,Data>) return value;
        else if constexpr(std::is_arithmetic_v<T> || std::is_same_v<T,std::string>) return value.get<T>();
        else return Loader::input<T>(value);
    }
    static bool matches(const Data& value) {
        if constexpr(std::is_same_v<T,std::nullptr_t>) return value.is_null();
        else if constexpr(std::is_same_v<T,bool>) return value.is_boolean();
        else if constexpr(std::is_arithmetic_v<T>) return value.is_number();
        else if constexpr(std::is_same_v<T,std::string>) return value.is_string();
        else return value.is_object();
    }
};
template<class T> struct Decoder<std::vector<T>> {
    static std::vector<T> read(const Data& value) {
        std::vector<T> result; for(const auto& item:value) result.push_back(Decoder<T>::read(item)); return result;
    }
    static bool matches(const Data& value) { return value.is_array(); }
};
template<class T> struct Decoder<std::vector<std::pair<std::string,T>>> {
    static std::vector<std::pair<std::string,T>> read(const Data& value) {
        std::vector<std::pair<std::string,T>> result;
        for(const auto& [key,item]:value.object()) result.emplace_back(key,Decoder<T>::read(item));
        return result;
    }
    static bool matches(const Data& value) { return value.is_object(); }
};
template<class... T> struct Decoder<std::variant<T...>> {
    static std::variant<T...> read(const Data& value) {
        std::optional<std::variant<T...>> result;
        ([&]{ if(!result && Decoder<T>::matches(value)) result.emplace(std::in_place_type<T>,Decoder<T>::read(value)); }(),...);
        if(!result) throw std::invalid_argument("native value has no C++ alternative");
        return std::move(*result);
    }
};
template<class T> Presence<T> member_value(const Data& value, const char* key) {
    if(!value.contains(key)) return {};
    if(value.at(key).is_null()) return {PresenceState::null,{}};
    return {PresenceState::value,Decoder<T>::read(value.at(key))};
}
template<class T> Data encode(const T& value) { return Encoder<T>::write(value); }
template<class T> T decode(const Data& value) { return Decoder<T>::read(value); }
}
}
