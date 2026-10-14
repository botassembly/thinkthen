"""JVM result projections derived from canonical call metadata and packets."""
import json
from host_answers import call_results, scalar_input_selectors


def signature(spec, definitions, scalar=False):
    from jvm import conversion
    from csharp import shape, nullable
    payload = spec['receipt_payload']
    array = payload.get('type') == 'array'
    receipt = payload['items'] if array else payload
    result_type = conversion(receipt, definitions)[0]
    value_schema = spec['aggregate_value']
    if array:
        value_schema = value_schema['items']
    source = shape(value_schema)
    if '$ref' in source and source['$ref'].endswith('DecideValue'):
        value_type = 'Object'
    else:
        if isinstance(source, dict) and isinstance(source.get('type'), list):
            source = {**source, 'type': next(kind for kind in source['type'] if kind != 'null')}
        value_type = conversion(source, definitions)[0]
    mode = spec['probability_mode']
    odds_type = 'Void' if mode == 'null' else 'List<BigDecimal>' if mode == 'items' and spec['call_shape'] != 'collection' else 'BigDecimal'
    if spec['call_shape'] != 'aggregate' and not scalar:
        value_type = f'List<{value_type}>'
        if mode != 'null': odds_type = f'List<{odds_type}>'
    return value_type, odds_type, result_type


def named_calls(definitions, results):
    from jvm import name
    lines = ['// Generated from the shared Rust call graph. Do not edit.',
             'package thinkthen;', 'import java.util.*;', 'import java.math.*;',
             'import java.util.concurrent.CompletableFuture;', 'import java.util.function.Function;',
             'public abstract class NamedCalls {',
             'protected abstract CompletableFuture<Engine.OwnedCall> named(Inputs.RequestFunction function, Inputs.RequestQuestion question, Inputs.RequestInput input, Inputs.RequestOptions options);']
    lines += ['private static <T> CompletableFuture<T> project(CompletableFuture<Engine.OwnedCall> pending, Function<Engine.OwnedCall,T> convert) {', 'var result = new CompletableFuture<T>();', 'result.whenComplete((value, error) -> { if (result.isCancelled()) pending.cancel(false); });', 'pending.whenComplete((value, error) -> { if (error != null) result.completeExceptionally(error); else { try { result.complete(convert.apply(value)); } catch (RuntimeException failure) { result.completeExceptionally(failure); } } });', 'return result;', '}']
    scalar_types = set(scalar_input_selectors(definitions))
    for function, spec in call_results(results).items():
        selectors = [None] + [child for child, _ in definitions['RequestInput']['variants']]
        for selector in selectors:
            scalar = selector in scalar_types and spec['call_shape'] == 'input'
            v, p, r = signature(spec, results, scalar)
            dynamic = selector is None and spec['call_shape'] == 'input'
            result = f'Results.CallResult<{"?,?" if dynamic else v + "," + p},Results.{r}>'
            # Named generated result types live inside Results, except native leaves.
            if not dynamic:
                for key in results:
                    typ = name(key)
                    if typ in (v, v.removeprefix('List<').removesuffix('>')):
                        result = result.replace(typ + ',', 'Results.' + typ + ',').replace('<' + typ + '>', '<Results.' + typ + '>')
            input_type = 'Inputs.RequestInput' if selector is None else 'Inputs.' + name(selector)
            args = f'Inputs.RequestQuestion question, {input_type} input'
            project = 'Results.project' + function.title() + ('Scalar' if scalar else '') + '(call)'
            if dynamic:
                test = ' || '.join('input instanceof Inputs.' + name(child) for child in sorted(scalar_types))
                project = f'({test}) ? Results.project{function.title()}Scalar(call) : {project}'
            lines += [f'public final CompletableFuture<{result}> {function}({args}) {{ return {function}(question, input, null); }}',
                      f'public final CompletableFuture<{result}> {function}({args}, Inputs.RequestOptions options) {{ return project(named(Inputs.RequestFunction.{function.upper()}, question, input, options), call -> {project}); }}']
    return lines + ['}']


def answers(definitions):
    from jvm import name
    from csharp import nullable
    lines = ['public static final class CallResult<V,P,R> extends Engine.OwnedCall {',
             'private final V value;', 'private final P probability;', 'private final List<R> results;', 'private final Facts facts;',
             'private CallResult(Engine.OwnedCall call, V value, P probability, List<R> results) {',
             'super(call.packets(), call.terminal()); this.value = value; this.probability = probability; this.results = Collections.unmodifiableList(new ArrayList<>(results));',
             'this.facts = call.terminal().facts().value();', '}',
             'public V value() { return value; }', 'public P probability() { return probability; }',
             'public List<R> results() { return results; }', 'public Facts facts() { return facts; }', '}',
             'private static <T> T optional(Presence<T> value) { return value.state() == Presence.State.VALUE ? value.value() : null; }',
             'private static <T> List<T> immutable(List<T> values) { return Collections.unmodifiableList(values); }']
    for function, spec in call_results(definitions).items():
        payload = spec['receipt_payload']
        array = payload.get('type') == 'array'
        v, p, r = signature(spec, definitions)
        carrier = name(spec['carrier'])
        receipt_schema = definitions[(payload['items'] if array else payload)['$ref'].split('/')[-1]]
        field = spec['value_projection']
        value_schema = receipt_schema['properties'][field]
        from jvm import member
        value_expr = 'row.' + member(field) + '()'
        if nullable(value_schema, definitions) or field not in receipt_schema.get('required', []):
            value_expr = f'optional({value_expr})'
        if function == 'decide': value_expr += '.value()'
        mode = spec['probability_mode']
        if mode == 'null': odds_expr = 'null'
        elif mode == 'yes': odds_expr = 'row.answer() instanceof AnswerYesNo odds ? odds.probability() : null'
        elif mode == 'picked':
            odds_expr = 'row.answer() instanceof ' + ('FindAnswer' if function == 'find' else 'AnswerChoice') + ' odds ? odds.probabilities().get(odds.pick()) : null'
        elif function == 'rank': odds_expr = 'row.answer() instanceof AnswerYesNo odds ? odds.probability() : null'
        elif function == 'relate': odds_expr = 'row.value().stream().map(RelatedEntityEdge::probability).toList()'
        else: odds_expr = 'recognitionProbabilities(row)'
        if spec.get('selection_path'):
            selected = member(spec['selection_path'][-1])
            odds_expr = f'optional(row.{selected}()) == null ? null : ({odds_expr})'
        if spec.get('resolution'):
            odds_expr = f'resolved(call, {json.dumps(function)}, optional(row.index())) ? ({odds_expr}) : null'

        lines += [f'static CallResult<{v},{p},{r}> project{function.title()}(Engine.OwnedCall call) {{',
                  f'var results = new ArrayList<{r}>();',
                  'for (SessionPacket packet : call.packets()) {',
                  f'if (packet instanceof {carrier} aggregate) ' + ('results.addAll(aggregate.value());' if array else 'results.add(aggregate.value());')]
        for child, _ in definitions['completesessionPacket']['variants']:
            fields = definitions[child].get('properties', {})
            if array and fields.get('kind', {}).get('const') == 'row' and fields.get('function', {}).get('const') == function and fields.get('value') == payload['items']:
                lines += [f'else if (packet instanceof {name(child)} row) results.add(row.value());']
        lines += ['}']
        if function == 'rank': lines += [f'results.sort(Comparator.comparing({r}::value));']
        if array:
            item_v = v[5:-1]
            lines += [f'var values = new ArrayList<{item_v}>();']
            if mode != 'null': lines += [f'var probabilities = new ArrayList<{p[5:-1]}>();']
            lines += [f'for ({r} row : results) {{ values.add({value_expr});']
            if mode != 'null': lines += [f'probabilities.add({odds_expr});']
            lines += ['}', f'return new CallResult<>(call, immutable(values), {"null" if mode == "null" else "immutable(probabilities)"}, results);']
        else:
            lines += ['if (results.size() != 1) throw new IllegalStateException("Expected one aggregate result");',
                      'var row = results.getFirst();', f'return new CallResult<>(call, {value_expr}, {odds_expr}, results);']
        lines += ['}']
        if spec['call_shape'] == 'input':
            sv, sp, _ = signature(spec, definitions, True)
            lines += [f'static CallResult<{sv},{sp},{r}> project{function.title()}Scalar(Engine.OwnedCall call) {{',
                      f'var result = project{function.title()}(call);',
                      'if (result.results().size() != 1) throw new IllegalStateException("Expected one input result");',
                      f'return new CallResult<>(call, result.value().getFirst(), {"null" if mode == "null" else "result.probability().getFirst()"}, result.results());', '}']
    lines += recognition_probabilities()
    lines += resolution(definitions)
    return '\n'.join(lines)


def recognition_probabilities():
    return ['private static List<BigDecimal> recognitionProbabilities(Recognition row) {',
            'if (row.value() instanceof RecognizeModeProposals boundary) return boundary.proposals().stream().map(BoundaryProposal::probability).toList();',
            'var entities = ((RecognizeEntities)row.value()).entities();',
            'var proposals = row.answer() instanceof RecognitionOddsRecognizeAnswer odds ? odds.proposals() : List.<RecognitionProposal>of();',
            'var probabilities = new ArrayList<BigDecimal>();',
            'for (var entity : entities) { BigDecimal probability = null;',
            'for (var proposal : proposals) { var selected = optional(proposal.selected());',
            'if (proposal.kept() && selected != null && selected.start().equals(entity.start()) && selected.end().equals(entity.end()) && Objects.equals(optional(proposal.kind()), entity.kind())) { probability = proposal.spanProbability(); break; }',
            '}', 'probabilities.add(probability);', '}', 'return immutable(probabilities);', '}']


def resolution(definitions):
    path = lambda parts: 'new String[]{' + ', '.join(json.dumps(part) for part in parts) + '}'
    lines = ['private static Object path(Object value, String[] members) {',
             'for (String member : members) { if (!(value instanceof Map<?,?> object)) return null; value = object.get(member); } return value;', '}',
             'private static boolean resolved(Engine.OwnedCall call, String function, BigInteger index) {',
             'for (var item : call.packets()) { var packet = Values.object(Values.json(item));',
             'if (!function.equals(packet.get("function"))) continue;',
             'switch (function) {']
    for function, spec in call_results(definitions).items():
        resolution = spec.get('resolution')
        if not resolution: continue
        guard = ' && '.join(f'{json.dumps(value)}.equals(path(packet, {path(parts)}))' for parts, value in resolution['guards'])
        lines += [f'case {json.dumps(function)} -> {{', f'if (!({guard})) continue;',
                  f'Object nativeIndex = path(packet, {path(resolution["index_path"])});',
                  'BigInteger actual = nativeIndex instanceof BigDecimal number ? number.toBigIntegerExact() : BigInteger.ZERO;',
                  'if (!actual.equals(index == null ? BigInteger.ZERO : index)) continue;',
                  f'return path(packet, {path(resolution["path"])}) != null;', '}']
    return lines + ['default -> { }', '}', '}', 'return false;', '}']


def facade_calls(definitions, language, results):
    # The language wrappers expose native collections and nullable values.
    from jvm import name
    lines = ['// Generated from the shared Rust call graph. Do not edit.',
             f'package thinkthen.{language}', 'import thinkthen.Engine', 'import thinkthen.Inputs', 'import thinkthen.Results',
             'import java.util.concurrent.CompletableFuture']
    if language == 'scala': lines += ['import scala.jdk.CollectionConverters.*']
    if language == 'kotlin':
        lines += ['abstract class KotlinNamedCalls(protected val namedEngine: Engine) {',
                  'protected abstract suspend fun <T> await(start: () -> CompletableFuture<T>): T']
    else:
        lines += ['abstract class ScalaNamedCalls(protected val namedEngine: Engine) {',
                  'protected def run[A,B <: OwnedCall](native: CompletableFuture[A])(convert: A => B): Call[B]']
    scalar_types = set(scalar_input_selectors(definitions))
    def host_type(typ, nil=False):
        if typ.startswith('List<'):
            child = host_type(typ[5:-1], nil)
            return 'List<' + child + '>' if language == 'kotlin' else 'List[' + child + ']'
        typ = {'Object': 'Any', 'Void': 'Nothing'}.get(typ, typ)
        if typ in ('BigDecimal', 'BigInteger'): typ = 'java.math.' + typ
        elif typ not in ('String', 'Boolean', 'Any', 'Nothing'): typ = 'Results.' + typ
        if nil or typ == 'Nothing': typ = typ + '?' if language == 'kotlin' else 'Option[' + typ + ']'
        return typ
    def convert(typ, expr, nil=False, depth=0):
        if typ.startswith('List<'):
            child = typ[5:-1]; var = 'item' + str(depth)
            inner = convert(child, var, nil, depth + 1)
            return expr + ('.map { ' + var + ' -> ' + inner + ' }' if language == 'kotlin' else '.asScala.toList.map(' + var + ' => ' + inner + ')')
        if typ == 'Void': return 'null' if language == 'kotlin' else 'None'
        if nil and language == 'scala': return 'Option(' + expr + ')'
        return expr
    from csharp import nullable
    for function, spec in call_results(results).items():
        selectors = [child for child, _ in definitions['RequestInput']['variants']]
        if spec['call_shape'] != 'input': selectors = [None]
        for selector in selectors:
            scalar = selector in scalar_types and spec['call_shape'] == 'input'
            v, p, r = signature(spec, results, scalar)
            schema = spec['aggregate_value']
            if spec['receipt_payload'].get('type') == 'array': schema = schema['items']
            value_nil = nullable(schema, results) or v == 'Object' or v == 'List<Object>'
            if language == 'scala' and spec['call_shape'] == 'collection' and schema is True:
                value_nil = False
            odds_nil = spec['probability_mode'] != 'null'
            hv, hp, hr = host_type(v, value_nil), host_type(p, odds_nil), 'Results.' + r
            result = f'CallResult<{hv},{hp},{hr}>' if language == 'kotlin' else f'CallResult[{hv},{hp},{hr}]'
            input_type = 'Inputs.RequestInput' if selector is None else 'Inputs.' + name(selector)
            value = convert(v, 'native.value()', value_nil)
            odds = convert(p, 'native.probability()', odds_nil)
            if language == 'kotlin':
                lines += [f'suspend fun {function}(question: Inputs.RequestQuestion, input: {input_type}, options: Inputs.RequestOptions? = null): {result} {{',
                          f'val native = await {{ namedEngine.{function}(question, input, options) }}',
                          f'return CallResult(native, {value}, {odds}, native.results().toList(), native.facts())', '}']
            else:
                lines += [f'final def {function}(question: Inputs.RequestQuestion, input: {input_type}, options: Option[Inputs.RequestOptions]): Call[{result}] = run(namedEngine.{function}(question, input, options.orNull)) {{ native =>',
                          f'new CallResult(native, {value}, {odds}, native.results().asScala.toList, native.facts())', '}',
                          f'final def {function}(question: Inputs.RequestQuestion, input: {input_type}): Call[{result}] = {function}(question, input, None)',
                          f'final def {function}(question: Inputs.RequestQuestion, input: {input_type}, options: Inputs.RequestOptions): Call[{result}] = {function}(question, input, Some(options))']
    for function, spec in call_results(results).items():
        if spec['call_shape'] != 'input': continue
        r = signature(spec, results)[2]
        selectors = [child for child, _ in definitions['RequestInput']['variants']]
        if language == 'kotlin':
            lines += [f'suspend fun {function}(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions? = null): CallResult<*,*,Results.{r}> = when (input) {{']
            lines += [f'is Inputs.{name(child)} -> {function}(question, input, options)' for child in selectors]
            lines += ['}']
        else:
            result = f'Call[CallResult[?,?,Results.{r}]]'
            lines += [f'final def {function}(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Option[Inputs.RequestOptions]): {result} = input match {{']
            lines += [f'case selected: Inputs.{name(child)} => {function}(question, selected, options)' for child in selectors]
            lines += ['}', f'final def {function}(question: Inputs.RequestQuestion, input: Inputs.RequestInput): {result} = {function}(question, input, None)',
                      f'final def {function}(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions): {result} = {function}(question, input, Some(options))']
    lines += ['}']
    if language == 'kotlin':
        lines += ['open class OwnedCall internal constructor(private val native: Engine.OwnedCall) {',
                  'val packets: List<Results.SessionPacket> get() = native.packets().toList()',
                  'val terminal: Results.SessionPacketTerminal get() = native.terminal()',
                  'fun packets() = packets', 'fun terminal() = terminal', '}',
                  'class CallResult<V,P,R> internal constructor(native: Engine.OwnedCall, val value: V, val probability: P, val results: List<R>, val facts: Results.Facts) : OwnedCall(native)']
    else:
        lines += ['class OwnedCall private[scala](private val native: Engine.OwnedCall) {',
                  'def packets(): List[Results.SessionPacket] = native.packets().asScala.toList',
                  'def terminal(): Results.SessionPacketTerminal = native.terminal()', '}',
                  'final class CallResult[V,P,R] private[scala](native: Engine.OwnedCall, val value: V, val probability: P, val results: List[R], val facts: Results.Facts) extends OwnedCall(native)']
    return lines
