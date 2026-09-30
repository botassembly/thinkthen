#pragma once
// Local JSON value, parser and writer for host values. MIT, with this package.
#include <cmath>
#include <cstdlib>
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
class Json {
public:
    // Keep insertion order: JSON label maps carry prompt order into the C door.
    using Object = std::vector<std::pair<std::string, Json>>;
    using Array = std::vector<Json>;
private:
    std::variant<std::nullptr_t, bool, double, std::string, Array, Object> value_;
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
    static void utf8(std::string& out, unsigned cp) {
        if (cp <= 0x7f) out += static_cast<char>(cp);
        else if (cp <= 0x7ff) { out += static_cast<char>(0xc0 | (cp >> 6)); out += static_cast<char>(0x80 | (cp & 63)); }
        else if (cp <= 0xffff) {
            out += static_cast<char>(0xe0 | (cp >> 12)); out += static_cast<char>(0x80 | ((cp >> 6) & 63)); out += static_cast<char>(0x80 | (cp & 63));
        } else {
            out += static_cast<char>(0xf0 | (cp >> 18)); out += static_cast<char>(0x80 | ((cp >> 12) & 63));
            out += static_cast<char>(0x80 | ((cp >> 6) & 63)); out += static_cast<char>(0x80 | (cp & 63));
        }
    }
    class Parser {
        const std::string& input; size_t pos = 0; unsigned depth = 0;
        [[noreturn]] void invalid() const { throw std::invalid_argument("invalid JSON at byte " + std::to_string(pos)); }
        void ws() { while (pos < input.size() && (input[pos]==' ' || input[pos]=='\t' || input[pos]=='\n' || input[pos]=='\r')) ++pos; }
        char take() { if (pos == input.size()) invalid(); return input[pos++]; }
        bool consume(char c) { ws(); if (pos < input.size() && input[pos] == c) { ++pos; return true; } return false; }
        unsigned hex4() {
            unsigned n=0;
            for (unsigned i=0;i<4;++i) { unsigned char c=static_cast<unsigned char>(take()); n <<= 4;
                if (c>='0' && c<='9') n += c-'0';
                else if (c>='a' && c<='f') n += c-'a'+10;
                else if (c>='A' && c<='F') n += c-'A'+10;
                else invalid();
            }
            return n;
        }
        std::string str() {
            if (take() != '"') invalid();
            std::string result;
            while (true) {
                unsigned char c=static_cast<unsigned char>(take());
                if (c=='"') return result;
                if (c<0x20) invalid();
                if (c=='\\') {
                    char e=take();
                    switch (e) {
                    case '"': case '\\': case '/': result += e; break;
                    case 'b': result += '\b'; break; case 'f': result += '\f'; break;
                    case 'n': result += '\n'; break; case 'r': result += '\r'; break;
                    case 't': result += '\t'; break;
                    case 'u': {
                        unsigned cp=hex4();
                        if (cp>=0xd800 && cp<=0xdbff) {
                            if (take()!='\\' || take()!='u') invalid();
                            unsigned lo=hex4(); if (lo<0xdc00 || lo>0xdfff) invalid();
                            cp=0x10000+((cp-0xd800)<<10)+(lo-0xdc00);
                        } else if (cp>=0xdc00 && cp<=0xdfff) invalid();
                        utf8(result,cp); break;
                    }
                    default: invalid();
                    }
                } else if (c < 0x80) result += static_cast<char>(c);
                else {
                    // Validate raw UTF-8; reject overlong encodings, surrogates and out-of-range code points.
                    unsigned n=0, cp=0, minimum=0;
                    if (c>=0xc2 && c<=0xdf) {n=1; cp=c&31; minimum=0x80;}
                    else if (c>=0xe0 && c<=0xef) {n=2; cp=c&15; minimum=0x800;}
                    else if (c>=0xf0 && c<=0xf4) {n=3; cp=c&7; minimum=0x10000;}
                    else invalid();
                    for (unsigned i=0;i<n;++i) { unsigned char b=static_cast<unsigned char>(take()); if ((b&0xc0)!=0x80) invalid(); cp=(cp<<6)|(b&63); }
                    if (cp<minimum || cp>0x10ffff || (cp>=0xd800 && cp<=0xdfff)) invalid();
                    utf8(result,cp);
                }
            }
        }
        Json number() {
            size_t start=pos;
            if (input[pos]=='-') ++pos;
            if (pos>=input.size()) invalid();
            if (input[pos]=='0') ++pos;
            else { if (input[pos]<'1' || input[pos]>'9') invalid(); do {++pos;} while (pos<input.size() && input[pos]>='0' && input[pos]<='9'); }
            if (pos<input.size() && input[pos]=='.') { ++pos; if (pos>=input.size() || input[pos]<'0' || input[pos]>'9') invalid(); do {++pos;} while (pos<input.size() && input[pos]>='0' && input[pos]<='9'); }
            if (pos<input.size() && (input[pos]=='e' || input[pos]=='E')) { ++pos; if (pos<input.size() && (input[pos]=='+' || input[pos]=='-')) ++pos;
                if (pos>=input.size() || input[pos]<'0' || input[pos]>'9') invalid();
                do {++pos;} while (pos<input.size() && input[pos]>='0' && input[pos]<='9'); }
            auto part=input.substr(start,pos-start);
            char* end=nullptr;
            double d=std::strtod(part.c_str(), &end);
            if (end!=part.c_str()+part.size() || !std::isfinite(d)) invalid();
            return Json(d);
        }
        Json value() {
            ws(); if (pos==input.size()) invalid();
            char c=input[pos];
            if (c=='"') return Json(str());
            if (c=='-' || (c>='0' && c<='9')) return number();
            for (const auto& literal : {std::pair{"true",Json(true)}, {"false",Json(false)}, {"null",Json(nullptr)}}) {
                std::string name(literal.first); if (input.compare(pos,name.size(),name)==0) {pos+=name.size(); return literal.second;}
            }
            if (c=='[' || c=='{') {
                if (++depth>128) invalid();
                ++pos;
                if (c=='[') {
                    Array rows; if (!consume(']')) { do {rows.push_back(value());} while (consume(',')); if (!consume(']')) invalid(); }
                    --depth; return Json(std::move(rows));
                }
                Object obj;
                if (!consume('}')) { do { ws(); if (pos==input.size() || input[pos]!='"') invalid();
                    std::string key=str(); if (!consume(':')) invalid();
                    auto item=value();
                    for (const auto& entry : obj) if (entry.first==key) invalid();
                    obj.emplace_back(std::move(key),std::move(item));
                } while (consume(',')); if (!consume('}')) invalid(); }
                --depth; return Json(std::move(obj));
            }
            invalid();
        }
    public:
        explicit Parser(const std::string& s):input(s) {}
        Json parse() {Json v=value(); ws(); if (pos!=input.size()) invalid(); return v;}
    };
public:
    Json():value_(nullptr) {} Json(std::nullptr_t):value_(nullptr) {}
    Json(bool b):value_(b) {}
    Json(double n):value_(n) {if (!std::isfinite(n)) throw std::invalid_argument("nonfinite JSON number");}
    Json(int n):value_(static_cast<double>(n)) {}
    Json(const char* s):value_(std::string(s)) {}
    Json(std::string s):value_(std::move(s)) {}
    Json(Array a):value_(std::move(a)) {} Json(Object o):value_(std::move(o)) {}
    Json(std::initializer_list<std::pair<const std::string, Json>> members):value_(Object{}) {
        auto& obj=std::get<Object>(value_);
        for (const auto& [key,val]:members) {
            for (const auto& existing:obj) if (existing.first==key) throw std::invalid_argument("duplicate JSON key");
            obj.emplace_back(key,val);
        }
    }
    static Json parse(const std::string& s) {return Parser(s).parse();}
    static Json parse(std::istream& s) {return parse(std::string(std::istreambuf_iterator<char>(s),{}));}
    bool is_null() const {return std::holds_alternative<std::nullptr_t>(value_);}
    bool is_boolean() const {return std::holds_alternative<bool>(value_);}
    bool is_number() const {return std::holds_alternative<double>(value_);}
    bool is_string() const {return std::holds_alternative<std::string>(value_);}
    bool is_array() const {return std::holds_alternative<Array>(value_);}
    bool is_object() const {return std::holds_alternative<Object>(value_);}
    const Object& object() const {return std::get<Object>(value_);}
    bool contains(const std::string& key) const {
        if (!is_object()) return false;
        for (const auto& [name, val] : std::get<Object>(value_)) { (void)val; if (name==key) return true; }
        return false;
    }
    const Json& at(const std::string& key) const {
        for (const auto& [name,val]:std::get<Object>(value_)) if (name==key) return val;
        throw std::out_of_range("missing JSON member: " + key);
    }
    const Json& at(size_t i) const {return std::get<Array>(value_).at(i);}
    auto begin() const {return std::get<Array>(value_).begin();}
    auto end() const {return std::get<Array>(value_).end();}
    template <typename T> T get() const {
        if constexpr (std::is_same_v<T, std::string>) return std::get<std::string>(value_);
        else if constexpr (std::is_same_v<T, bool>) return std::get<bool>(value_);
        else if constexpr (std::is_same_v<T, double>) return std::get<double>(value_);
    }
    std::string dump() const {
        if (is_null()) return "null";
        if (auto p=std::get_if<bool>(&value_)) return *p ? "true":"false";
        if (auto p=std::get_if<double>(&value_)) {std::ostringstream s; s << std::setprecision(17) << *p; return s.str();}
        if (auto p=std::get_if<std::string>(&value_)) return quote(*p);
        if (auto p=std::get_if<Array>(&value_)) {std::string s="["; for (const auto& v:*p) {if (s.size()>1) s+=','; s+=v.dump();} return s+']';}
        std::string s="{"; for (const auto& [k,v]:std::get<Object>(value_)) {if (s.size()>1) s+=','; s+=quote(k)+":"+v.dump();} return s+'}';
    }
    friend bool operator==(const Json& a,const Json& b) {
        if (a.value_.index()!=b.value_.index()) return false;
        if (!a.is_object()) return a.value_==b.value_;
        const auto& first=std::get<Object>(a.value_);
        const auto& second=std::get<Object>(b.value_);
        if (first.size()!=second.size()) return false;
        for (const auto& [key,val]:first) if (!b.contains(key) || !(b.at(key)==val)) return false;
        return true;
    }
    friend bool operator!=(const Json& a,const Json& b) {return !(a==b);}
};
} // namespace tt
