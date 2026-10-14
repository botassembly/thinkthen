"""JVM host helpers derived from request functions and normalized packets."""
import json
from host_answers import aggregate_answers, aggregate_projection, judgment_route, scalar_answers


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


def valid_shape(source, definitions, expression, depth=0):
    """Validate required answer fields without restating the Rust shapes."""
    from csharp import enum_values, shape
    source = shape(source)
    if '$ref' in source:
        return valid_shape(definitions[source['$ref'].split('/')[-1]], definitions, expression, depth)
    values = enum_values(source)
    if values:
        return '(' + ' || '.join(json.dumps(value) + '.equals(' + expression + ')' for value in values) + ')'
    kind = source.get('type')
    if kind == 'object':
        var = 'object' + str(depth)
        checks = [f'{expression} instanceof Map<?,?> {var}']
        checks += [valid_shape(source['properties'][field], definitions, f'{var}.get({json.dumps(field)})', depth + 1)
                   for field in source.get('required', [])]
        return '(' + ' && '.join(checks) + ')'
    if kind == 'array':
        var = 'list' + str(depth)
        item = 'element' + str(depth)
        child = valid_shape(source['items'], definitions, item, depth + 1)
        return f'({expression} instanceof List<?> {var} && {var}.stream().allMatch({item} -> {child}))'
    carrier = {'boolean': 'Boolean', 'string': 'String', 'number': 'BigDecimal', 'integer': 'BigDecimal'}.get(kind)
    return f'{expression} instanceof {carrier}' if carrier else f'{expression} != null'


def answers(definitions):
    from jvm import conversion
    route, guards = judgment_route(definitions)
    from csharp import shape
    terminal_shape = next(source for source in definitions.values()
                          if source.get('properties', {}).get('kind', {}).get('const') == 'terminal')
    call_error = definitions[shape(terminal_shape['properties']['failure'])['$ref'].split('/')[-1]]
    error_check = valid_shape(call_error['properties']['error'], definitions, 'failure.get("error")')
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
             f'if (!(terminal.get("failure") instanceof Map<?,?> failure) || !{error_check}) throw new CardinalityFailure(call);',
             'throw new Engine.SessionFailure(call);', '}', '}',
             'private static Object scalar(Engine.OwnedCall call, String function, String kind, boolean nullable) {',
             'terminal(call); Object answer = null; int count = 0;',
             'for (SessionPacket item : call.packets()) { Map<String,Object> packet = Values.object(Values.json(item));',
             f'if (!function.equals(packet.get("function")) || !({guard})) continue;',
             f'Object raw = path(packet, {path(route)});',
             'if (!(raw instanceof Map<?,?> judgment) || !kind.equals(judgment.get("kind"))) continue;',
             'if (!judgment.containsKey("value") || (!nullable && judgment.get("value") == null)) throw new CardinalityFailure(call);',
             'answer = judgment.get("value"); count++;', '}',
             'if (count != 1) throw new CardinalityFailure(call); return answer;', '}',
             'private static Object collection(Engine.OwnedCall call, String function, boolean array, boolean rows) {',
             'terminal(call); List<Object> values = new ArrayList<>(); Object answer = null; int count = 0;',
             'for (SessionPacket item : call.packets()) { Map<String,Object> packet = Values.object(Values.json(item));',
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
                  f'if (!({valid_shape(source, definitions, "value")})) throw new CardinalityFailure(call);',
                  f'return {decode};', '}']
    for function, _, payload in aggregate_answers(definitions):
        if function in scalar_functions: continue
        projected, projection = aggregate_projection(function, payload, definitions)
        typ, decode = conversion(projected, definitions)
        array = payload.get('type') == 'array'
        rows = any(source.get('properties', {}).get('function', {}).get('const') == function and
                   source.get('properties', {}).get('kind', {}).get('const') == 'row' for source in definitions.values())
        lines += [f'public static {typ} {function}(Engine.OwnedCall call) {{',
                  f'Object value = collection(call, {json.dumps(function)}, {str(array).lower()}, {str(rows).lower()});',
                  *([f'List<Object> projected = new ArrayList<>(); for (Object item : (List<?>)value) {{ var receipt = Values.object(item); if (!receipt.containsKey({json.dumps(projection)})) throw new CardinalityFailure(call); projected.add(receipt.get({json.dumps(projection)})); }} value = projected;'] if projection else []),
                  f'return {decode};', '}']
    return '\n'.join(line for line in lines + ['}'] if line)


def facade_calls(definitions, language, results):
    from csharp import enum_values
    lines = ['// Generated from the shared Rust request graph. Do not edit.',
             f'package thinkthen.{language}', 'import thinkthen.Engine', 'import thinkthen.Inputs',
             'import java.util.concurrent.CompletableFuture']
    if language == 'scala': lines += ['import scala.jdk.CollectionConverters.*']
    if language == 'kotlin':
        lines += ['abstract class KotlinNamedCalls(protected val namedEngine: Engine) {',
                  'protected abstract suspend fun await(start: () -> CompletableFuture<Engine.OwnedCall>): OwnedCall']
        for function in enum_values(definitions['RequestFunction']):
            lines.append(f'suspend fun {function}(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions? = null): OwnedCall = await {{ namedEngine.{function}(question, input, options) }}')
    else:
        lines += ['abstract class ScalaNamedCalls(protected val namedEngine: Engine) {',
                  'protected def run(native: CompletableFuture[Engine.OwnedCall]): Call']
        for function in enum_values(definitions['RequestFunction']):
            lines += [f'final def {function}(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Option[Inputs.RequestOptions] = None): Call = run(namedEngine.{function}(question, input, options.orNull))',
                      f'final def {function}(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions): Call = run(namedEngine.{function}(question, input, options))']
    lines += ['}']
    from jvm import conversion
    methods = {}
    for function, _, payload in scalar_answers(results):
        source = dict(payload)
        kinds = source.get('type')
        nil = isinstance(kinds, list) and 'null' in kinds
        if isinstance(kinds, list): source['type'] = next(kind for kind in kinds if kind != 'null')
        methods[function] = (*conversion(source, results)[:1], nil)
    for function, _, payload in aggregate_answers(results):
        if function not in methods: methods[function] = (conversion(aggregate_projection(function, payload, results)[0], results)[0], False)
    if language == 'kotlin':
        lines += ['class OwnedCall internal constructor(private val native: Engine.OwnedCall) {',
                  'val packets: List<thinkthen.Results.SessionPacket> get() = native.packets().toList()',
                  'val terminal: thinkthen.Results.SessionPacketTerminal get() = native.terminal()',
                  'fun packets() = packets', 'fun terminal() = terminal']
        for function, (typ, nil) in methods.items():
            collection = typ.startswith('List<')
            host = typ.replace('BigDecimal', 'java.math.BigDecimal').replace('BigInteger', 'java.math.BigInteger')
            if collection:
                child = host[5:-1]
                if child not in ('String', 'Object', 'java.math.BigInteger', 'java.math.BigDecimal', 'Boolean'): child = 'thinkthen.Results.' + child
                host = 'List<' + child.replace('Object', 'Any?') + '>'
            elif typ not in ('String', 'Boolean', 'Object', 'BigDecimal', 'BigInteger'): host = 'thinkthen.Results.' + typ
            lines.append(f'fun {function}(): {host}{"?" if nil else ""} = native.{function}(){".toList()" if collection else ""}')
    else:
        lines += ['final class OwnedCall private[scala](private val native: Engine.OwnedCall) {',
                  'def packets(): List[thinkthen.Results.SessionPacket] = native.packets().asScala.toList',
                  'def terminal(): thinkthen.Results.SessionPacketTerminal = native.terminal()']
        for function, (typ, nil) in methods.items():
            collection = typ.startswith('List<')
            host = typ.replace('BigDecimal', 'java.math.BigDecimal').replace('BigInteger', 'java.math.BigInteger')
            if collection:
                child = host[5:-1]
                if child not in ('String', 'Object', 'java.math.BigInteger', 'java.math.BigDecimal', 'Boolean'): child = 'thinkthen.Results.' + child
                host = 'List[' + child.replace('Object', 'Any') + ']'
            elif typ not in ('String', 'Boolean', 'Object', 'BigDecimal', 'BigInteger'): host = 'thinkthen.Results.' + typ
            expression = f'native.{function}()' + ('.asScala.toList' if collection else '')
            if nil: host, expression = f'Option[{host}]', f'Option({expression})'
            lines.append(f'def {function}(): {host} = {expression}')
    return lines + ['}']
