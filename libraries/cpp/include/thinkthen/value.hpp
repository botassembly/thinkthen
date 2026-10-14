#pragma once
// Local JSON value, parser and writer for host values. MIT, with this package.
#include <cmath>
#include <cstdint>
#include <charconv>
#include <locale>
#include <iomanip>
#include <istream>
#include <sstream>
#include <stdexcept>
#include <string>
#include <type_traits>
#include <utility>
#include <variant>
#include <vector>

namespace tt {
class Data;
namespace detail { std::string serialize(const Data& value); Data parse_original(const std::string& text); }
class Data {
    friend std::string detail::serialize(const Data& value);
    friend Data detail::parse_original(const std::string& text);
public:
    // Keep insertion order: JSON label maps carry prompt order into the C interface.
    using Object = std::vector<std::pair<std::string, Data>>;
    using Array = std::vector<Data>;
private:
    std::variant<std::nullptr_t, bool, int64_t, uint64_t, double, std::string, Array, Object> value_;
    static std::string quote(const std::string& text) {
        std::string out = "\"";
        const char* hex = "0123456789abcdef";
        for (unsigned char c : text) {
            switch (c) {
            case '"': out += "\\\""; break;
            case '\\': out += "\\\\"; break;
            case '\b': out += "\\b"; break;
            case '\f': out += "\\f"; break;
            case '\n': out += "\\n"; break;
            case '\r': out += "\\r"; break;
            case '\t': out += "\\t"; break;
            default:
                if (c < 0x20) { out += "\\u00"; out += hex[c >> 4]; out += hex[c & 15]; }
                else out += static_cast<char>(c);
            }
        }
        return out + '"';
    }
public:
    Data():value_(nullptr) {} Data(std::nullptr_t):value_(nullptr) {}
    Data(bool b):value_(b) {}
    Data(double n):value_(n) {if (!std::isfinite(n)) throw std::invalid_argument("nonfinite JSON number");}
    template<typename T, std::enable_if_t<std::is_integral_v<T> && !std::is_same_v<T,bool>, int> = 0>
    Data(T n):value_(nullptr) { if constexpr(std::is_signed_v<T>) value_=int64_t(n); else value_=uint64_t(n); }
    Data(const char* s):value_(std::string(s)) {}
    Data(std::string s):value_(std::move(s)) {}
    Data(Array a):value_(std::move(a)) {} Data(Object o):value_(std::move(o)) {}
    Data(std::initializer_list<std::pair<const std::string, Data>> members):value_(Object{}) {
        auto& obj=std::get<Object>(value_);
        for (const auto& [key,val]:members) {
            for (const auto& existing:obj) if (existing.first==key) throw std::invalid_argument("duplicate JSON key");
            obj.emplace_back(key,val);
        }
    }
private:
    static Data parse(const std::string& text);
public:
    static Data read(std::istream& input);
    bool is_null() const {return std::holds_alternative<std::nullptr_t>(value_);}
    bool is_boolean() const {return std::holds_alternative<bool>(value_);}
    bool is_number() const {return std::holds_alternative<double>(value_) || std::holds_alternative<int64_t>(value_) || std::holds_alternative<uint64_t>(value_);}
    bool is_string() const {return std::holds_alternative<std::string>(value_);}
    bool is_array() const {return std::holds_alternative<Array>(value_);}
    bool is_object() const {return std::holds_alternative<Object>(value_);}
    const Object& object() const {return std::get<Object>(value_);}
    bool contains(const std::string& key) const {
        if (!is_object()) return false;
        for (const auto& [name, val] : std::get<Object>(value_)) { (void)val; if (name==key) return true; }
        return false;
    }
    const Data& at(const std::string& key) const {
        for (const auto& [name,val]:std::get<Object>(value_)) if (name==key) return val;
        throw std::out_of_range("missing JSON member: " + key);
    }
    const Data& at(size_t i) const {return std::get<Array>(value_).at(i);}
    auto begin() const {return std::get<Array>(value_).begin();}
    auto end() const {return std::get<Array>(value_).end();}
    template <typename T> T get() const {
        if constexpr (std::is_same_v<T, std::string>) return std::get<std::string>(value_);
        else if constexpr (std::is_same_v<T, bool>) return std::get<bool>(value_);
        else if constexpr (std::is_arithmetic_v<T>) {
            if(auto p=std::get_if<int64_t>(&value_)) return static_cast<T>(*p);
            if(auto p=std::get_if<uint64_t>(&value_)) return static_cast<T>(*p);
            return static_cast<T>(std::get<double>(value_));
        }
    }
private:
    std::string dump() const {
        if (is_null()) return "null";
        if (auto p=std::get_if<bool>(&value_)) return *p ? "true":"false";
        if (auto p=std::get_if<int64_t>(&value_)) return std::to_string(*p);
        if (auto p=std::get_if<uint64_t>(&value_)) return std::to_string(*p);
        if (auto p=std::get_if<double>(&value_)) {std::ostringstream s; s.imbue(std::locale::classic()); s << std::setprecision(17) << *p; return s.str();}
        if (auto p=std::get_if<std::string>(&value_)) return quote(*p);
        if (auto p=std::get_if<Array>(&value_)) {std::string s="["; for (const auto& v:*p) {if (s.size()>1) s+=','; s+=v.dump();} return s+']';}
        std::string s="{"; for (const auto& [k,v]:std::get<Object>(value_)) {if (s.size()>1) s+=','; s+=quote(k)+":"+v.dump();} return s+'}';
    }
public:
    friend bool operator==(const Data& a,const Data& b) {
        if (a.is_number() && b.is_number()) {
            if(auto x=std::get_if<int64_t>(&a.value_)) {
                if(auto y=std::get_if<uint64_t>(&b.value_)) return *x>=0 && static_cast<uint64_t>(*x)==*y;
            }
            if(auto x=std::get_if<uint64_t>(&a.value_)) {
                if(auto y=std::get_if<int64_t>(&b.value_)) return *y>=0 && *x==static_cast<uint64_t>(*y);
            }
            if(a.value_.index()!=b.value_.index()) {
                const Data& integer=std::holds_alternative<double>(a.value_) ? b:a;
                double number=std::holds_alternative<double>(a.value_) ? a.get<double>():b.get<double>();
                if(std::trunc(number)!=number) return false;
                if(auto x=std::get_if<uint64_t>(&integer.value_)) return number>=0 && number<18446744073709551616.0 && *x==static_cast<uint64_t>(number);
                if(auto x=std::get_if<int64_t>(&integer.value_)) return number>=-9223372036854775808.0 && number<9223372036854775808.0 && *x==static_cast<int64_t>(number);
            }
        }
        if (a.value_.index()!=b.value_.index()) return false;
        if (!a.is_object()) return a.value_==b.value_;
        const auto& first=std::get<Object>(a.value_);
        const auto& second=std::get<Object>(b.value_);
        if (first.size()!=second.size()) return false;
        for (const auto& [key,val]:first) if (!b.contains(key) || !(b.at(key)==val)) return false;
        return true;
    }
    friend bool operator!=(const Data& a,const Data& b) {return !(a==b);}
};
namespace detail { inline std::string serialize(const Data& value) { return value.dump(); } }
} // namespace tt
