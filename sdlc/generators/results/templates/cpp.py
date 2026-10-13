"""C++ immutable typed accessors from the native semantic graph."""
import json
import re
from csharp import shape as common_shape, enum_values

def shape(raw, presence=False):
    if isinstance(raw, dict) and "const" in raw and "type" not in raw:
        raw = {**raw, "type": "string" if isinstance(raw["const"],str) else "boolean" if isinstance(raw["const"],bool) else "integer"}
    if not presence and isinstance(raw, dict) and not enum_values(raw):
        alternatives = raw.get("anyOf", raw.get("oneOf", []))
        kinds = raw.get("type")
        if isinstance(kinds, list):
            alternatives = [{**raw, "type": kind} for kind in kinds]
        if alternatives and any(child is True or child == {} for child in alternatives):
            return {}
        # Null inside a collection or union is a value, not member presence.
        if alternatives and any(child.get("type") == "null" or
                                isinstance(child.get("type"), list) and "null" in child["type"]
                                for child in alternatives if isinstance(child, dict)):
            return {"cpp_alternatives": alternatives}
    try:
        result = common_shape(raw)
        if isinstance(result,dict) and enum_values(result): return {**result,"type":"string"}
        return result
    except ValueError:
        alternatives=raw.get("anyOf", raw.get("oneOf",[]))
        if not alternatives and isinstance(raw.get("type"),list): alternatives=[{**raw,"type":kind} for kind in raw["type"]]
        if not alternatives: raise
        return {"cpp_alternatives": alternatives}
ROOTS = ('completesessionPacket', 'completeplan')

def name(key):
    return ''.join(p[:1].upper()+p[1:] for p in re.findall(r'[A-Z]?[a-z]+|[A-Z]+(?![a-z])|[0-9]+', key.removeprefix('complete')))

def integer_type(s):
    if s.get("minimum", -1) >= 0 or s.get("format", "").startswith("uint"):
        return "uint8_t" if s.get("maximum")==255 else "uint32_t" if s.get("maximum")==4294967295 else "uint64_t"
    return "int64_t"

def typ(raw, presence=False):
    s=shape(raw, presence)
    if 'cpp_alternatives' in s: return 'std::variant<'+', '.join(dict.fromkeys(typ(child) for child in s['cpp_alternatives']))+'>'
    if '$ref' in s: return name(s['$ref'].removeprefix('#/$defs/'))
    kind=s.get('type')
    if kind=='array': return 'std::vector<'+typ(s.get('items',{}))+'>'
    if kind=='object' and isinstance(s.get('additionalProperties'),dict): return 'std::vector<std::pair<std::string, '+typ(s['additionalProperties'])+'>>'
    if 'primitive_variants' in s:
        schemas=s.get('primitive_schemas',{})
        return 'std::variant<'+', '.join(dict.fromkeys(typ(schemas.get(k,{'type':k})) for k in s['primitive_variants'] if not presence or k != 'null'))+'>'
    return {'string':'std::string','boolean':'bool','integer':integer_type(s),'number':'double','null':'std::nullptr_t'}.get(kind,'Json') if isinstance(kind,str) else 'Json'

def condition(tag):
    mode,member,value=tag
    if mode=='literal': return f'detail::literal(value_, {json.dumps(member)}, {json.dumps(value)})'
    if mode=='literals': return ' && '.join(f'detail::literal(value_, {json.dumps(k)}, {json.dumps(v)})' for k,v in value.items())
    if mode=='member': return f'value_.contains({json.dumps(member)})'
    return ' && '.join([f'value_.contains({json.dumps(k)})' for k in value['required']]+[f'!value_.contains({json.dumps(k)})' for k in value['excluded']])

def render(definitions,version):
    lines=['// Generated from the shared Rust result graph; do not edit.','#pragma once','#include "result_value.hpp"','#include <memory>','namespace tt::results {',f'inline constexpr const char* request_version = {json.dumps(version)};']
    for key in definitions: lines += [f'class {name(key)};']
    bodies=[]
    for key,raw in definitions.items():
        n=name(key); s=shape(raw)
        lines += [f'class {n} : public Node {{ public: using Node::Node;']
        if 'properties' in s:
            for member,field in s['properties'].items():
                t=typ(field, presence=True); method=member+'_' if member in ('template','class','operator','namespace','default','true','false') else member
                lines += [f'private: mutable std::shared_ptr<const Presence<{t}>> cached_{method}_;', f'public: const Presence<{t}>& {method}() const;']
                bodies += [f'inline const Presence<{t}>& {n}::{method}() const {{ if (!cached_{method}_) cached_{method}_ = std::make_shared<const Presence<{t}>>(detail::member_value<{t}>(value_, {json.dumps(member)})); return *cached_{method}_; }}']
        from host_answers import named_object_fields
        for member in named_object_fields(s, definitions):
            lines += [f'    std::string {member}_name() const;']
            bodies += [f'inline std::string {n}::{member}_name() const {{ return value_.at({json.dumps(member)}).at("name").get<std::string>(); }}']
        for child,tag in s.get('variants',[]):
            cn=name(child)
            lines += [f'    std::optional<{cn}> as_{cn}() const;']
            bodies += [f'inline std::optional<{cn}> {n}::as_{cn}() const {{ if ({condition(tag)}) return {cn}(value_); return std::nullopt; }}']
        if 'properties' not in s and 'variants' not in s:
            t=typ(s)
            lines += [f'private: mutable std::shared_ptr<const {t}> cached_value_;', f'public: const {t}& value() const;']
            bodies += [f'inline const {t}& {n}::value() const {{ if (!cached_value_) cached_value_ = std::make_shared<const {t}>(decode<{t}>(value_)); return *cached_value_; }}']
        lines += ['};']
    return '\n'.join(lines+bodies+['}'])+'\n' + answer_helpers(definitions)

def input_render(definitions):
    lines=['// Generated from request.schema.json; do not edit.','#pragma once','#include "result_value.hpp"','namespace tt::inputs {','using results::Node;']
    for key in definitions: lines += [f'class {name(key)};']
    bodies=[]
    for key,raw in definitions.items():
        n=name(key);s=shape(raw)
        lines += [f'class {n} : public Node {{', f'    {n}(const Json& value, int):Node(value) {{}}', f'public: static {n} from_document(const Json& value) {{ return {n}(value,0); }}']
        if 'variants' in s:
            for child,_ in s['variants']:
                cn=name(child); lines += [f'    {n}(const {cn}& value);']
                bodies += [f'inline {n}::{n}(const {cn}& value):Node(value.document()) {{}}']
        elif 'properties' in s:
            constants={k:v['const'] for k,v in s['properties'].items() if isinstance(v,dict) and 'const' in v}
            for member, field in s['properties'].items():
                if isinstance(field, dict) and '$ref' in field and member in s.get('required', []):
                    resolved = definitions[field['$ref'].removeprefix('#/$defs/')]
                    values = enum_values(resolved)
                    if values and len(values) == 1:
                        constants[member] = values[0]
            expr='Json::Object{'+','.join('{'+json.dumps(k)+',Json('+json.dumps(v)+')}' for k,v in constants.items())+'}'
            lines += [f'    {n}():Node(Json({expr})) {{}}']
            for member,field in s['properties'].items():
                if isinstance(field,dict) and 'const' in field: continue
                t=typ(field); lines += [f'    {n}& set_{member}(const {t}& value);']
                bodies += [f'inline {n}& {n}::set_{member}(const {t}& value) {{ set_member({json.dumps(member)},results::encode(value)); return *this; }}']
        else:
            t=typ(s)
            lines += [f'    explicit {n}(const {t}& value);']
            bodies += [f'inline {n}::{n}(const {t}& value):Node(results::encode(value)) {{}}']
        lines += ['};']
    lines = lines + bodies
    lines += [
        'inline RequestQuestion question_file(const std::string& path) { return RequestQuestionFile().set_path(path); }',
        'inline RequestQuestion question_name(const std::string& name) { return RequestQuestionName().set_name(name); }',
        'inline RequestQuestion question_reference(const std::string& reference) { return RequestQuestionReference().set_reference(reference); }',
        'inline RequestItem item(const RequestOriginal& original) { return RequestItem().set_original(original); }',
        'inline RequestInput files(const std::vector<std::string>& paths, const RequestReader& reading={}) { return RequestInputSource().set_source(RequestSource().set_paths(paths).set_reading(reading)); }',
    ]
    return '\n'.join(lines+['}'])+'\n'



def usage(header):
    members = re.findall(r'^#define (THINKTHEN_COMPLETE_USAGE_PERSISTENCE_(\w+)_V1) (\d+)\s*$', header, re.M)
    lines = ['// Generated from the compiler-derived C header; do not edit.', '#pragma once', '#include <thinkthen/thinkthen.h>', 'namespace tt {', 'enum class UsagePersistenceState : uint32_t {']
    lines += [word.lower() + ' = ' + symbol + ',' for symbol, word, _ in members]
    lines += ['};', 'enum class NativeFailureKind {']
    lines += [word.lower() + ' = ' + symbol + ',' for symbol, word in re.findall(r'^#define (THINKTHEN_E(\w+)) \d+\s*$', header, re.M)]
    return '\n'.join(lines + ['};', '}']) + '\n'


def named_calls(functions, native_functions):
    from host_answers import usage_operations
    lines = ['// Generated from RequestFunction and the compiler-derived ABI; do not edit.', '#pragma once',
             'namespace tt {', 'template<class Owner> class NamedCalls {', 'public:']
    for function in functions:
        lines += [f'AnswerCall<results::read_{function}> {function}(const inputs::RequestQuestion& question, const inputs::RequestInput& input, const inputs::RequestOptions& options={{}}) const {{',
                  f'return AnswerCall<results::read_{function}>(static_cast<const Owner*>(this)->start({json.dumps(function)}, question, input, options));', '}']
    if 'thinkthen_request_plan_json' in native_functions:
        lines += ['results::Plan plan(const inputs::RequestCall& call) const {',
                  'auto bytes=inputs::Request().set_call(call).document().dump(); char* raw=nullptr; size_t size=0;',
                  'detail::check_session(thinkthen_request_plan_json(static_cast<const Owner*>(this)->native_engine(),bytes.data(),bytes.size(),&raw,&size));',
                  'std::unique_ptr<char,decltype(&thinkthen_free_string)> owned(raw,&thinkthen_free_string);',
                  'return results::Plan(Json::parse(std::string(raw,size)));', '}']
    for symbol, operation in usage_operations(native_functions):
        lines += [f'UsageStatus {operation}() const {{',
                  'auto engine=static_cast<const Owner*>(this)->native_engine();',
                  'thinkthen_complete_usage_persistence_v1 state{}; thinkthen_complete_utf8_v1 advice{};',
                  f'detail::check_session({symbol}(engine,&state,&advice));',
                  'return {static_cast<UsagePersistenceState>(state.kind),advice.data ? std::optional<std::string>(std::string(advice.data,advice.len)) : std::nullopt};', '}']
    return '\n'.join(lines + ['};', '}']) + '\n'


def answer_helpers(definitions):
    from host_answers import aggregate_answers, judgment_route, scalar_answers
    scalars = scalar_answers(definitions)
    scalar_names = {function for function, _, _ in scalars}
    path, guards = judgment_route(definitions)
    def access(parts):
        return 'packet.document()' + ''.join('.at(' + json.dumps(part) + ')' for part in parts)
    lines = ['namespace tt::results {',
             'class AnswerCardinalityFailure : public std::runtime_error { public:',
             'std::vector<SessionPacket> packets;',
             'AnswerCardinalityFailure(const char* message, const std::vector<SessionPacket>& values):std::runtime_error(message),packets(values) {}', '};',
             'inline void successful_answer(const std::vector<SessionPacket>& packets) {',
             'size_t terminals=0;',
             'for(const auto& packet:packets) if(literal(packet.document(), "kind", "terminal")) {',
             '++terminals;',
             'if(packet.document().contains("failure")) throw AnswerCardinalityFailure("terminal contains a failure",packets);', '}',
             'if(terminals!=1 || packets.empty() || !literal(packets.back().document(),"kind","terminal")) throw AnswerCardinalityFailure("expected one final successful terminal",packets);', '}']
    for function, kind, payload in scalars:
        nullable = isinstance(payload.get('type'), list) and 'null' in payload['type']
        raw = {**payload, 'type': next(k for k in payload['type'] if k != 'null')} if nullable else payload
        value_type = typ(raw)
        answer_type = f'std::optional<{value_type}>' if nullable else value_type
        guard = ' && '.join(f'literal({access(parts[:-1])}, {json.dumps(parts[-1])}, {json.dumps(value)})' for parts, value in guards)
        lines += [f'inline {answer_type} read_{function}(const std::vector<SessionPacket>& packets) {{',
                  'successful_answer(packets);', f'std::optional<{answer_type}> answer;',
                  f'for(const auto& packet:packets) if({guard}) {{',
                  f'const auto& judgment={access(path)};',
                  f'if(!literal(judgment,"kind",{json.dumps(kind)})) continue;',
                  'if(answer) throw AnswerCardinalityFailure("multiple matching judgments",packets);']
        expression = f'decode<{value_type}>(judgment.at("value"))'
        if nullable:
            expression = f'judgment.at("value").is_null() ? {answer_type}{{}} : {answer_type}({expression})'
        lines += [f'answer.emplace({expression});', '}',
                  'if(!answer) throw AnswerCardinalityFailure("no matching judgment",packets);', 'return std::move(*answer);', '}']
    for function, aggregate, payload in aggregate_answers(definitions):
        if function in scalar_names: continue
        rows = [(key, value['properties']['value']) for key, value in definitions.items()
                if value.get('properties', {}).get('function', {}).get('const') == function
                and value.get('properties', {}).get('kind', {}).get('const') == 'row']
        if rows:
            row, payload = rows[0]
            value_type = typ(payload)
            lines += [f'inline std::vector<{value_type}> read_{function}(const std::vector<SessionPacket>& packets) {{',
                      'successful_answer(packets);', f'std::vector<{value_type}> answer;',
                      f'for(const auto& packet:packets) if(auto row=packet.as_{name(row)}()) answer.push_back(*row->value().value);',
                      'return answer;', '}']
        else:
            value_type = typ(payload)
            lines += [f'inline {value_type} read_{function}(const std::vector<SessionPacket>& packets) {{',
                      'successful_answer(packets);', f'std::optional<{value_type}> answer;',
                      f'for(const auto& packet:packets) if(auto aggregate=packet.as_{name(aggregate)}()) {{',
                      'if(answer) throw AnswerCardinalityFailure("multiple matching aggregates",packets);',
                      'answer.emplace(*aggregate->value().value);', '}',
                      'if(!answer) throw AnswerCardinalityFailure("no matching aggregate",packets);',
                      'return std::move(*answer);', '}']
    return '\n'.join(lines + ['}']).replace('literal(', 'detail::literal(') + '\n'
