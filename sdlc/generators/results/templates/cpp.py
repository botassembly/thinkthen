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
    return {'string':'std::string','boolean':'bool','integer':integer_type(s),'number':'double','null':'std::nullptr_t'}.get(kind,'Data') if isinstance(kind,str) else 'Data'

def condition(tag):
    mode,member,value=tag
    if mode=='literal': return f'detail::literal(value_, {json.dumps(member)}, {json.dumps(value)})'
    if mode=='literals': return ' && '.join(f'detail::literal(value_, {json.dumps(k)}, {json.dumps(v)})' for k,v in value.items())
    if mode=='member': return f'value_.contains({json.dumps(member)})'
    return ' && '.join([f'value_.contains({json.dumps(k)})' for k in value['required']]+[f'!value_.contains({json.dumps(k)})' for k in value['excluded']])

def cache_field(value_type, method):
    slot = 'std::shared_ptr<const ' + value_type + '>'
    return f'private: std::shared_ptr<{slot}> cached_{method}_ = std::make_shared<{slot}>();'


def cache_body(owner, value_type, method, expression):
    return f'inline const {value_type}& {owner}::{method}() const {{ auto stored = std::atomic_load(cached_{method}_.get()); if (!stored) {{ auto candidate = std::make_shared<const {value_type}>({expression}); if (std::atomic_compare_exchange_strong(cached_{method}_.get(), &stored, candidate)) stored = std::move(candidate); }} return *stored; }}'


def render(definitions,version):
    from generate import ROOT, SCHEMA, graph, immediate_schema, prepare
    extras = prepare(graph(immediate_schema(json.loads(SCHEMA.read_text())), ('completeImmediateFacts',)))
    definitions = dict(sorted({**extras, **definitions}.items()))
    lines=['// Generated from the shared Rust result graph; do not edit.','#pragma once','#include "result_value.hpp"','#include <memory>','#include <atomic>','namespace tt::results {',f'inline constexpr const char* request_version = {json.dumps(version)};']
    for key in definitions: lines += [f'class {name(key)};']
    bodies=[]
    for key,raw in definitions.items():
        n=name(key); s=shape(raw)
        lines += [f'class {n} final : public Node {{', f'    {n}(const Data& value, int):Node(value) {{}}', 'friend struct detail::Loader;', 'public:']
        if 'properties' in s:
            for member,field in s['properties'].items():
                t=typ(field, presence=True); method=member+'_' if member in ('template','class','operator','namespace','default','true','false') else member
                lines += [cache_field(f'Presence<{t}>', method), f'public: const Presence<{t}>& {method}() const;']
                bodies += [cache_body(n, f'Presence<{t}>', method, f'detail::member_value<{t}>(value_, {json.dumps(member)})')]
        from host_answers import named_object_fields
        for member in named_object_fields(s, definitions):
            lines += [f'    std::string {member}_name() const;']
            bodies += [f'inline std::string {n}::{member}_name() const {{ return value_.at({json.dumps(member)}).at("name").get<std::string>(); }}']
        for child,tag in s.get('variants',[]):
            cn=name(child)
            lines += [f'    std::optional<{cn}> as_{cn}() const;']
            bodies += [f'inline std::optional<{cn}> {n}::as_{cn}() const {{ if ({condition(tag)}) return detail::Loader::input<{cn}>(value_); return std::nullopt; }}']
        if 'properties' not in s and 'variants' not in s:
            t=typ(s)
            lines += [cache_field(t, 'value'), f'public: const {t}& value() const;']
            bodies += [cache_body(n, t, 'value', f'detail::decode<{t}>(value_)')]
        lines += ['};']
    return usage((ROOT / 'libraries/c/include/thinkthen.h').read_text()) + '\n'.join(lines+bodies+['}'])+'\n' + answer_helpers(definitions) + native_values(definitions)

def input_render(definitions):
    lines=['// Generated from request.schema.json; do not edit.','#pragma once','#include "result_value.hpp"','namespace tt::inputs {','using results::Node;']
    for key in definitions: lines += [f'class {name(key)};']
    bodies=[]
    for key,raw in definitions.items():
        n=name(key);s=shape(raw)
        lines += [f'class {n} : public Node {{', f'    {n}(const Data& value, int):Node(value) {{}}', 'friend struct ::tt::results::detail::Loader;', 'public:']
        if 'variants' in s:
            for child,_ in s['variants']:
                cn=name(child); lines += [f'    {n}(const {cn}& value);']
                bodies += [f'inline {n}::{n}(const {cn}& value):Node(::tt::results::detail::document(value)) {{}}']
        elif 'properties' in s:
            constants={k:v['const'] for k,v in s['properties'].items() if isinstance(v,dict) and 'const' in v}
            for member, field in s['properties'].items():
                if isinstance(field, dict) and '$ref' in field and member in s.get('required', []):
                    resolved = definitions[field['$ref'].removeprefix('#/$defs/')]
                    values = enum_values(resolved)
                    if values and len(values) == 1:
                        constants[member] = values[0]
            expr='Data::Object{'+','.join('{'+json.dumps(k)+',Data('+json.dumps(v)+')}' for k,v in constants.items())+'}'
            lines += [f'    {n}():Node(Data({expr})) {{}}']
            for member,field in s['properties'].items():
                if isinstance(field,dict) and 'const' in field: continue
                t=typ(field); lines += [f'    {n}& set_{member}(const {t}& value);']
                bodies += [f'inline {n}& {n}::set_{member}(const {t}& value) {{ set_member({json.dumps(member)},results::detail::encode(value)); return *this; }}']
        else:
            t=typ(s)
            lines += [f'    explicit {n}(const {t}& value);']
            bodies += [f'inline {n}::{n}(const {t}& value):Node(results::detail::encode(value)) {{}}']
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
    lines = ['// Generated from the compiler-derived C header; do not edit.', '#pragma once', '#include <cstdint>', '#include <string>', '#ifndef THINKTHEN_CPP_ENUMS', '#define THINKTHEN_CPP_ENUMS', 'namespace tt {', 'enum class UsagePersistenceState : uint32_t {']
    lines += [word.lower() + ' = ' + value + ',' for _, word, value in members]
    lines += ['};', 'enum class NativeFailureKind {']
    lines += [word.lower() + ' = ' + value + ',' for word, value in re.findall(r'^#define THINKTHEN_E(\w+) (\d+)\s*$', header, re.M)]
    lines += ['};', 'inline NativeFailureKind failure_kind(const std::string& name) {']
    lines += [f'if(name=="{word.lower()}") return NativeFailureKind::{word.lower()};' for word, _ in re.findall(r'^#define THINKTHEN_E(\w+) (\d+)\s*$', header, re.M)]
    return '\n'.join(lines + ['return NativeFailureKind::defect;', '}', '}', '#endif']) + '\n'


def named_calls(functions, native_functions):
    from host_answers import usage_operations
    lines = ['// Generated from RequestFunction and the compiler-derived ABI; do not edit.', '#pragma once',
             'namespace tt {', 'template<class Owner> class NamedCalls {', 'public:']
    for function in functions:
        lines += [f'AnswerCall<results::read_{function}> {function}(const inputs::RequestQuestion& question, const inputs::RequestInput& input, const inputs::RequestOptions& options={{}}) const {{',
                  f'return AnswerCall<results::read_{function}>(static_cast<const Owner*>(this)->start({json.dumps(function)}, question, input, options));', '}']
    if 'thinkthen_request_plan_named_value' in native_functions:
        lines += ['results::Plan plan(const inputs::RequestCall& call) const {',
                  'return static_cast<const Owner*>(this)->plan_impl(call);', '}']
    for _, operation in usage_operations(native_functions):
        lines += [f'UsageStatus {operation}() const {{',
                  f'return static_cast<const Owner*>(this)->usage_impl({str(operation == "finish_usage_status").lower()});', '}']
    return '\n'.join(lines + ['};', '}']) + '\n'


def answer_helpers(definitions):
    from host_answers import aggregate_answers, aggregate_projection, judgment_route, scalar_answers
    scalars = scalar_answers(definitions)
    scalar_names = {function for function, _, _ in scalars}
    path, guards = judgment_route(definitions)
    def access(parts):
        return '::tt::results::detail::document(packet)' + ''.join('.at(' + json.dumps(part) + ')' for part in parts)
    lines = ['namespace tt {',
             'class Failure : public std::runtime_error { public:',
             'NativeFailureKind kind;', 'bool retryable=false;',
             'std::optional<results::ImmediateFacts> facts;', 'std::optional<results::CallError> failure;',
             'std::optional<results::SessionPacketTerminal> terminal;', 'std::vector<results::SessionPacket> packets;',
             'Failure(int code, const std::string& message);',
             'Failure(const char* message, const std::vector<results::SessionPacket>& values);',
             'explicit Failure(results::SessionPacketTerminal value);', '};', '}', 'namespace tt::results {',
             'inline void successful_answer(const std::vector<SessionPacket>& packets) {',
             'size_t terminals=0;',
             'for(const auto& packet:packets) if(literal(::tt::results::detail::document(packet), "kind", "terminal")) {',
             '++terminals;',
             'if(::tt::results::detail::document(packet).contains("failure")) throw ::tt::Failure("terminal contains a failure",packets);', '}',
             'if(terminals!=1 || packets.empty() || !literal(::tt::results::detail::document(packets.back()),"kind","terminal")) throw ::tt::Failure("expected one final successful terminal",packets);', '}']
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
                  'if(answer) throw ::tt::Failure("multiple matching judgments",packets);']
        expression = f'detail::decode<{value_type}>(judgment.at("value"))'
        if nullable:
            expression = f'judgment.at("value").is_null() ? {answer_type}{{}} : {answer_type}({expression})'
        lines += [f'answer.emplace({expression});', '}',
                  'if(!answer) throw ::tt::Failure("no matching judgment",packets);', 'return std::move(*answer);', '}']
    for function, aggregate, payload in aggregate_answers(definitions):
        if function in scalar_names: continue
        projected, member = aggregate_projection(function, payload, definitions)
        if member:
            value_type = typ(projected['items'])
            lines += [f'inline std::vector<{value_type}> read_{function}(const std::vector<SessionPacket>& packets) {{',
                      'successful_answer(packets);', f'std::vector<{value_type}> answer;',
                      f'for(const auto& packet:packets) {{ if(literal(::tt::results::detail::document(packet),"function",{json.dumps(function)})) {{',
                      'const auto& value=::tt::results::detail::document(packet).at("value");',
                      f'if(literal(::tt::results::detail::document(packet),"kind","row")) answer.push_back(detail::decode<{value_type}>(value.at({json.dumps(member)})));',
                      f'else if(literal(::tt::results::detail::document(packet),"kind","aggregate")) {{ if(!answer.empty()) continue; for(const auto& row:value) answer.push_back(detail::decode<{value_type}>(row.at({json.dumps(member)}))); }}',
                      '}}', 'return answer;', '}']
            continue
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
                      'if(answer) throw ::tt::Failure("multiple matching aggregates",packets);',
                      'answer.emplace(*aggregate->value().value);', '}',
                      'if(!answer) throw ::tt::Failure("no matching aggregate",packets);',
                      'return std::move(*answer);', '}']
    return '\n'.join(lines + ['}']).replace('literal(', 'detail::literal(') + '\n'


def native_values(definitions):
    """Copy compiler-derived native views into immutable host values, without parsing."""
    import c
    from generate import SCHEMA, graph, immediate_schema, prepare, references
    all_definitions = prepare(graph(immediate_schema(json.loads(SCHEMA.read_text())),
                                    ('completesessionPacket', 'completeImmediateFacts')))
    definitions, pending = {}, ['completesessionPacket', 'completeImmediateFacts']
    while pending:
        key = pending.pop()
        if key in definitions:
            continue
        schema = definitions[key] = all_definitions[key]
        pending.extend(references(schema))
        pending.extend(child for child, _ in schema.get('variants', []))
    class Layout(c.Target):
        def __init__(self):
            super().__init__(definitions)
            self.members = {}
            self.arms = {}
        def structure(self, key, members):
            self.members[key] = members
            super().structure(key, members)
        def union(self, key, arms):
            self.arms[c.ident(key)] = arms
            super().union(key, arms)
    target = Layout()
    for key, schema in definitions.items():
        target.definition(key, schema)
    lines = ['#ifdef THINKTHEN_CPP_IMPLEMENTATION', '#include <thinkthen/thinkthen.h>', 'namespace tt::results::detail {']
    lines += [f'inline Data native_value(const {key}& value);' for key in target.members]
    lines += ['inline Data native_value(const thinkthen_complete_json_v1& value);',
              'inline Data native_value(const thinkthen_complete_utf8_v1& value) { return Data(std::string(value.data ? value.data : "", value.len)); }',
              'template<class T> Data native_value(const T* value) { if(!value) throw std::invalid_argument("missing native value"); return native_value(*value); }',
              'template<class T, std::enable_if_t<std::is_arithmetic_v<T>, int> = 0> Data native_value(T value) { return Data(value); }',
              'inline Data native_value(const thinkthen_complete_json_v1& value) {',
              'switch(value.kind) {',
              'case THINKTHEN_COMPLETE_JSON_NULL_V1: return Data(nullptr);',
              'case THINKTHEN_COMPLETE_JSON_BOOLEAN_V1: return Data(bool(value.data.boolean));',
              'case THINKTHEN_COMPLETE_JSON_STRING_V1: return native_value(value.data.string);',
              'case THINKTHEN_COMPLETE_JSON_NUMBER_V1: { auto text=native_value(value.data.number).get<std::string>();',
              'if(text.find_first_of(".eE")==std::string::npos) { if(!text.empty() && text[0]==\'-\') { int64_t number{}; auto parsed=std::from_chars(text.data(),text.data()+text.size(),number); if(parsed.ec==std::errc{}) return Data(number); } else { uint64_t number{}; auto parsed=std::from_chars(text.data(),text.data()+text.size(),number); if(parsed.ec==std::errc{}) return Data(number); } throw std::invalid_argument("native integer is outside the C++ range"); }',
              'std::istringstream stream(text); stream.imbue(std::locale::classic()); double number{}; stream >> number; if(stream.fail() || !stream.eof() || !std::isfinite(number)) throw std::invalid_argument("invalid native number"); return Data(number); }',
              'case THINKTHEN_COMPLETE_JSON_ARRAY_V1: { Data::Array items; for(size_t i=0;i<value.data.array.len;++i) items.push_back(native_value(value.data.array.data[i])); return Data(std::move(items)); }',
              'case THINKTHEN_COMPLETE_JSON_OBJECT_V1: { Data::Object items; for(size_t i=0;i<value.data.object.len;++i) { auto& item=value.data.object.data[i]; auto name=native_value(item.name).get<std::string>(); for(const auto& prior:items) if(prior.first==name) throw std::invalid_argument("duplicate original member"); items.emplace_back(std::move(name),native_value(item.value)); } return Data(std::move(items)); }',
              'default: throw std::invalid_argument("invalid native value kind"); } }']
    for key, members in target.members.items():
        schema = target.schemas.get(key, {})
        body = []
        if key in target.arms:
            body += ['switch(value.kind) {']
            for arm, _, _, _ in target.arms[key]:
                symbol = key.removesuffix('_v1').upper() + '_' + arm.upper() + '_V1'
                field = arm + '_' if arm in ('true', 'false') else arm
                expression = 'Data(nullptr)' if arm == 'null' else f'native_value(value.data.{field})'
                if arm == 'boolean': expression = f'Data(bool(value.data.{field}))'
                body += [f'case {symbol}: return {expression};']
            body += ['default: throw std::invalid_argument("invalid native alternative"); }']
        elif 'enum' in schema:
            body += ['switch(value.kind) {']
            for value in schema['enum']:
                symbol = key.removesuffix('_v1').upper() + '_' + re.sub(r'[^A-Za-z0-9]+', '_', value).strip('_').upper() + '_V1'
                body += [f'case {symbol}: return Data({json.dumps(value)});']
            body += ['default: throw std::invalid_argument("invalid native enum"); }']
        elif 'properties' in schema:
            body += ['Data::Object items;']
            for member, field_type in members:
                if member == 'extensions': continue
                expression = f'value.{member + "_" if member in ("true", "false") else member}'
                if field_type.endswith('_presence_v1'):
                    decoded = f'native_value({expression}.value)'
                    if c.normalized(schema['properties'][member]).get('type') == 'boolean':
                        decoded = f'Data(bool({expression}.value))'
                    body += [f'if({expression}.presence==THINKTHEN_COMPLETE_PRESENCE_NULL_V1) items.emplace_back({json.dumps(member)},Data(nullptr));',
                             f'else if({expression}.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) items.emplace_back({json.dumps(member)},{decoded});']
                else:
                    decoded = f'native_value({expression})'
                    if c.normalized(schema['properties'][member]).get('type') == 'boolean': decoded = f'Data(bool({expression}))'
                    body += [f'items.emplace_back({json.dumps(member)},{decoded});']
            body += ['for(size_t i=0;i<value.extensions.len;++i) { auto& item=value.extensions.data[i]; items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value)); }', 'return Data(std::move(items));']
        elif members[0][0] == 'presence':
            body += ['return value.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? native_value(value.value) : Data(nullptr);']
        elif members[0][0] == 'data':
            mapping = key.endswith('_map_v1')
            body += ['Data::Object items;' if mapping else 'Data::Array items;', 'for(size_t i=0;i<value.len;++i) {']
            body += ['auto& item=value.data[i]; items.emplace_back(native_value(item.name).get<std::string>(),native_value(item.value));' if mapping else 'items.push_back(native_value(value.data[i]));']
            body += ['}', 'return Data(std::move(items));']
        elif members[0][0] == 'name':
            body += ['return native_value(value.value);']
        else:
            expression = 'Data(bool(value.value))' if schema.get('type') == 'boolean' else 'native_value(value.value)'
            body += [f'return {expression};']
        lines += [f'inline Data native_value(const {key}& value) {{'] + body + ['}']
    return '\n'.join(lines + ['}', '#endif']) + '\n'
