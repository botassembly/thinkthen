"""JVM host helpers derived from request functions and normalized packets."""
import json
from host_answers import aggregate_answers, judgment_route, scalar_answers


def named_calls(definitions):
    from csharp import enum_values
    lines = ['// Generated from the shared Rust request graph. Do not edit.',
             'package thinkthen;', 'import java.util.concurrent.CompletableFuture;',
             'public abstract class NamedCalls {',
             'protected abstract CompletableFuture<Engine.OwnedCall> named(Inputs.RequestFunction function, Inputs.RequestQuestion question, Inputs.RequestInput input, Inputs.RequestOptions options);']
    for function in enum_values(definitions['RequestFunction']):
        args = 'Inputs.RequestQuestion question, Inputs.RequestInput input'
        lines += [f'public final CompletableFuture<Engine.OwnedCall> {function}({args}) {{ return {function}(question, input, null); }}',
                  f'public final CompletableFuture<Engine.OwnedCall> {function}({args}, Inputs.RequestOptions options) {{ return named(Inputs.RequestFunction.{function.upper()}, question, input, options); }}']
    return lines + ['}']


def answers(definitions):
    from jvm import conversion
    route, guards = judgment_route(definitions)
    path = lambda parts: 'new String[]{' + ', '.join(json.dumps(part) for part in parts) + '}'
    guard = ' && '.join(f'{json.dumps(value)}.equals(path(packet, {path(parts)}))' for parts, value in guards)
    lines = ['public static final class CallAnswers {', 'private CallAnswers() {}',
             'public static final class CardinalityFailure extends IllegalStateException {',
             'private static final long serialVersionUID = 1L;', 'private final transient Engine.OwnedCall call;',
             'CardinalityFailure(Engine.OwnedCall call) { super("Expected exactly one successful matching answer"); this.call = call; }',
             'public Engine.OwnedCall call() { return call; }', '}',
             'private static Object path(Object value, String[] members) {',
             'for (String member : members) { if (!(value instanceof Map<?,?> object)) return null; value = object.get(member); } return value;', '}',
             'private static void terminal(Engine.OwnedCall call) {',
             'Map<String,Object> terminal = Values.object(call.terminal().json());',
             'if (!"terminal".equals(terminal.get("kind"))) throw new CardinalityFailure(call);',
             'if (terminal.containsKey("failure")) {',
             'if (!(terminal.get("failure") instanceof Map<?,?> failure) || !(failure.get("error") instanceof Map<?,?> error) || !(error.get("message") instanceof String)) throw new CardinalityFailure(call);',
             'throw new Engine.SessionFailure(call);', '}', '}',
             'private static Object scalar(Engine.OwnedCall call, String function, String kind, boolean nullable) {',
             'terminal(call); Object answer = null; int count = 0;',
             'for (SessionPacket item : call.packets()) { Map<String,Object> packet = Values.object(item.json());',
             f'if (!function.equals(packet.get("function")) || !({guard})) continue;',
             f'Object raw = path(packet, {path(route)});',
             'if (!(raw instanceof Map<?,?> judgment) || !kind.equals(judgment.get("kind"))) continue;',
             'if (!judgment.containsKey("value") || (!nullable && judgment.get("value") == null)) throw new CardinalityFailure(call);',
             'answer = judgment.get("value"); count++;', '}',
             'if (count != 1) throw new CardinalityFailure(call); return answer;', '}',
             'private static Object collection(Engine.OwnedCall call, String function, boolean array, boolean rows) {',
             'terminal(call); List<Object> values = new ArrayList<>(); Object answer = null; int count = 0;',
             'for (SessionPacket item : call.packets()) { Map<String,Object> packet = Values.object(item.json());',
             'if (!function.equals(packet.get("function"))) continue;',
             'boolean row = rows && "row".equals(packet.get("kind"));',
             'if (!row && !"aggregate".equals(packet.get("kind"))) continue;',
             'if (!packet.containsKey("value") || packet.get("value") == null) throw new CardinalityFailure(call);',
             'answer = packet.get("value"); count++;',
             'if (array) { if (row) values.add(answer); else { if (!(answer instanceof List<?> list)) throw new CardinalityFailure(call); values.addAll(list); } }', '}',
             'if (array) { if (!rows && count != 1) throw new CardinalityFailure(call); return Collections.unmodifiableList(values); }',
             'if (count != 1) throw new CardinalityFailure(call); return answer;', '}']
    scalar_functions = set()
    for function, kind, payload in scalar_answers(definitions):
        scalar_functions.add(function)
        source = dict(payload)
        kinds = source.get('type')
        nil = isinstance(kinds, list) and 'null' in kinds
        if isinstance(kinds, list): source['type'] = next(value for value in kinds if value != 'null')
        typ, decode = conversion(source, definitions)
        lines += [f'public static {typ} {function}(Engine.OwnedCall call) {{',
                  f'Object value = scalar(call, {json.dumps(function)}, {json.dumps(kind)}, {str(nil).lower()});',
                  ('if (value == null) return null;' if nil else ''),
                  f'return {decode};', '}']
    for function, _, payload in aggregate_answers(definitions):
        if function in scalar_functions: continue
        typ, decode = conversion(payload, definitions)
        array = payload.get('type') == 'array'
        rows = any(source.get('properties', {}).get('function', {}).get('const') == function and
                   source.get('properties', {}).get('kind', {}).get('const') == 'row' for source in definitions.values())
        lines += [f'public static {typ} {function}(Engine.OwnedCall call) {{',
                  f'Object value = collection(call, {json.dumps(function)}, {str(array).lower()}, {str(rows).lower()});',
                  f'return {decode};', '}']
    return '\n'.join(line for line in lines + ['}'] if line)
