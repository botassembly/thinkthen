"""Generate owned JVM status types and carrier measurements from the C compiler."""
import importlib.util
import json
import re
from jvm_host import named_calls, facade_calls


def outputs(root):
    path = root / 'sdlc/scripts/check-c-exports.py'
    spec = importlib.util.spec_from_file_location('jvm_usage_abi', path)
    abi = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(abi)
    native = abi.header_abi(root / 'libraries/c/include/thinkthen.h')
    prefix = 'THINKTHEN_COMPLETE_USAGE_PERSISTENCE_'
    states = [(key.removeprefix(prefix).removesuffix('_V1').title(), value)
              for key, value in native['constants'].items() if key.startswith(prefix) and key.endswith('_V1')]
    state = native['records']['thinkthen_complete_usage_persistence_v1']
    advice = native['records']['thinkthen_complete_utf8_v1']
    for function in ('thinkthen_engine_usage_persistence_v1', 'thinkthen_engine_finish_usage_status_v1'):
        signature = native['functions'][function]
        if signature['return'] != 'int' or signature['arguments'] != [
                'const thinkthen_engine *', 'thinkthen_complete_usage_persistence_v1 *',
                'thinkthen_complete_utf8_v1 *']:
            raise ValueError('JVM usage query signature changed: ' + function)
    if state['fields']['kind']['type'] != 'uint32_t' or state['fields']['kind']['width'] != 4:
        raise ValueError('JVM usage kind requires a uint32_t carrier')
    if advice['fields']['data']['type'] != 'const char *' or advice['fields']['len']['type'] != 'size_t':
        raise ValueError('JVM usage advice requires a counted UTF-8 view')
    java = ['// Generated from the compiler-derived C header ABI. Do not edit.',
            'package thinkthen;', 'import java.util.Objects;',
            '/** An owned observation of this engine\'s current usage deltas. */',
            'public record UsagePersistence(State state, String advice) {',
            'public UsagePersistence { Objects.requireNonNull(state); }',
            'public enum State { ' + ', '.join(name + '(' + str(value) + ')' for name, value in states) + ';',
            'private final int code; State(int code) { this.code = code; }',
            'static State read(int code) { for (State state : values()) if (state.code == code) return state;',
            'throw new IllegalStateException("Unknown native usage persistence state"); }', '}', '}']
    layouts = ['// Generated from compiler-measured C carriers. Do not edit.',
               'package thinkthen;', 'import java.lang.foreign.*;', 'final class NativeUsageLayouts {',
               'private NativeUsageLayouts() {}']
    for label, record in [('STATE', state), ('ADVICE', advice)]:
        layouts.append(f'static final MemoryLayout {label} = MemoryLayout.sequenceLayout({record["size"]}, ValueLayout.JAVA_BYTE).withByteAlignment({record["alignment"]});')
        for name, field in record['fields'].items():
            layouts.append(f'static final long {label}_{name.upper()} = {field["offset"]};')
    layouts += ['}']
    from jvm_native import render as native_results
    result_reader = native_results(root, native)
    kotlin = ['// Generated from the C header constants. Do not edit.', 'package thinkthen.kotlin',
              'data class UsagePersistence(val state: State, val advice: String?) {',
              'enum class State { ' + ', '.join(name for name, _ in states) + ' }',
              'companion object { internal fun read(value: thinkthen.UsagePersistence) =',
              'UsagePersistence(State.valueOf(value.state().name), value.advice()) }', '}']
    scala = ['// Generated from the C header constants. Do not edit.', 'package thinkthen.scala',
             'final case class UsagePersistence(state: UsagePersistence.State, advice: Option[String])',
             'object UsagePersistence {', 'enum State { case ' + ', '.join(name for name, _ in states) + ' }',
             'private[scala] def read(value: thinkthen.UsagePersistence): UsagePersistence =',
             'UsagePersistence(State.valueOf(value.state().name()), Option(value.advice()))', '}']
    waits = ['// Generated from the canonical C header ABI. Do not edit.',
             'package thinkthen;', 'import java.lang.foreign.*;', 'final class NativeWait {', 'private NativeWait() {}']
    for name, value in native['constants'].items():
        if name.startswith('THINKTHEN_SESSION_WAIT_'):
            waits += ['static final int ' + name.removeprefix('THINKTHEN_SESSION_WAIT_').removesuffix('_V1') + ' = ' + str(value) + ';']
        if name.startswith('THINKTHEN_SESSION_READY_'):
            waits += ['static final int ' + name.removeprefix('THINKTHEN_SESSION_').removesuffix('_V1') + ' = ' + str(value) + ';']
    layouts_by_type = {'const thinkthen_session *': 'ValueLayout.ADDRESS', 'uint32_t': 'ValueLayout.JAVA_INT', 'uint32_t *': 'ValueLayout.ADDRESS'}
    prototype = native['functions']['thinkthen_session_wait']
    carriers = ', '.join(layouts_by_type[typ] for typ in prototype['arguments'])
    waits += ['static int waitReady(MemorySegment session, int interest, int timeout, MemorySegment ready) {',
              'return (int)NativeSession.call("thinkthen_session_wait", ValueLayout.JAVA_INT, new MemoryLayout[]{' + carriers + '}, session, interest, timeout, ready);', '}']
    for prefix in ('THINKTHEN_SESSION_RESULT_', 'THINKTHEN_SESSION_PENDING_', 'THINKTHEN_SESSION_END_'):
        for name, value in native['constants'].items():
            if name == prefix + 'V1':
                waits += ['static final int READ_' + name.removeprefix('THINKTHEN_SESSION_').removesuffix('_V1') + ' = ' + str(value) + ';']
    for method, function, arguments in [
        ('nextPacket', 'thinkthen_session_next_packet', 'MemorySegment session, int interest, MemorySegment status, MemorySegment result'),
        ('push', 'thinkthen_session_push', 'MemorySegment session, MemorySegment descriptor, long length, MemorySegment status')]:
        parameters = native['functions'][function]['arguments']
        layouts_by_type.update({'thinkthen_session *': 'ValueLayout.ADDRESS', 'const char *': 'ValueLayout.ADDRESS',
                                'size_t': 'ValueLayout.JAVA_LONG', 'thinkthen_session_result **': 'ValueLayout.ADDRESS'})
        carriers = ', '.join(layouts_by_type[typ] for typ in parameters)
        values = ', '.join(argument.rsplit(' ', 1)[-1] for argument in arguments.split(', '))
        waits += ['static int ' + method + '(' + arguments + ') {',
                  'return (int)NativeSession.call("' + function + '", ValueLayout.JAVA_INT, new MemoryLayout[]{' + carriers + '}, ' + values + ');', '}']
    waits += ['}']
    kinds = [(key.removeprefix('THINKTHEN_E').title(), value)
             for key, value in native['constants'].items() if re.fullmatch(r'THINKTHEN_E[A-Z]+', key)]
    errors = ['// Generated from compiler-derived C error constants. Do not edit.',
              'package thinkthen;', 'public enum ErrorKind {',
              ', '.join(name.upper() + '(' + str(value) + ')' for name, value in kinds) + ';',
              'private final int code; ErrorKind(int code) { this.code = code; }',
              'public int code() { return code; }',
              'public static ErrorKind read(int code) { for (ErrorKind kind : values()) if (kind.code == code) return kind;',
              'throw new IllegalStateException("Unknown native error kind"); }', '}']
    signature = native['functions']['thinkthen_session_new_named_with_surface']
    if signature['return'] != 'int' or signature['arguments'] != [
            'const thinkthen_engine *', 'const thinkthen_named_request_v1 *',
            'thinkthen_complete_utf8_v1', 'thinkthen_session **']:
        raise ValueError('JVM named constructor signature changed')
    request_schema = json.loads((root / 'specification/request.schema.json').read_text())
    from generate import graph, prepare
    from jvm import ROOTS
    request_definitions = prepare(graph(request_schema, ('RequestInput', 'RequestFunction')))
    result_definitions = prepare(graph(json.loads((root / 'crates/thinkthen/src/public/results/complete.schema.json').read_text()), ROOTS))
    named = native['records']['thinkthen_named_request_v1']
    named_lines = ['// Generated from compiler-derived C carriers. Do not edit.',
                   'package thinkthen;', 'import java.lang.foreign.*;',
                   'final class NativeNamed {', 'private NativeNamed() {}',
                   f'static final MemoryLayout REQUEST = MemoryLayout.sequenceLayout({named["size"]}, ValueLayout.JAVA_BYTE).withByteAlignment({named["alignment"]});',
                   'static void text(Arena arena, MemorySegment view, String text) {',
                   'var data = arena.allocateFrom(ValueLayout.JAVA_BYTE, NativeSession.utf8(text));',
                   f'view.set(ValueLayout.ADDRESS, {advice["fields"]["data"]["offset"]}, data);',
                   f'view.set(NativeSession.SIZE, {advice["fields"]["len"]["offset"]}, data.byteSize());', '}',
                   'static MemorySegment create(MemorySegment engine, Engine.Surface surface, Inputs.RequestFunction function, Inputs.RequestQuestion question, Inputs.RequestInput input, Inputs.RequestOptions options) {',
                   'try (Arena arena = Arena.ofConfined()) {', 'var request = arena.allocate(REQUEST);']
    for field in named['fields']:
        value = {'function': '(String)Values.json(function)', 'question_json': 'Json.write(Values.json(question))',
                 'input_json': 'Json.write(Values.json(input))', 'options_json': 'options == null ? "{}" : Json.write(options.json())'}[field]
        named_lines += [f'text(arena, request.asSlice({named["fields"][field]["offset"]}, {advice["size"]}), {value});']
    named_lines += [f'var token = arena.allocate(NativeUsageLayouts.ADVICE);',
                    'text(arena, token, surface.name().toLowerCase(java.util.Locale.ROOT));',
                    'var output = arena.allocate(ValueLayout.ADDRESS);',
                    'NativeSession.check((int)NativeSession.call("thinkthen_session_new_named_with_surface", ValueLayout.JAVA_INT,',
                    'new MemoryLayout[]{ValueLayout.ADDRESS, ValueLayout.ADDRESS, MemoryLayout.structLayout(ValueLayout.ADDRESS, NativeSession.SIZE), ValueLayout.ADDRESS}, engine, request, token, output));',
                    'return output.get(ValueLayout.ADDRESS, 0);', '}', '}', '}']
    return {'thinkthen/NativeResults.java': result_reader, 'kotlin/KotlinNamedCalls.kt': facade_calls(request_definitions, 'kotlin', result_definitions), 'scala/ScalaNamedCalls.scala': facade_calls(request_definitions, 'scala', result_definitions), 'thinkthen/ErrorKind.java': errors, 'thinkthen/NamedCalls.java': named_calls(request_definitions, result_definitions), 'thinkthen/NativeNamed.java': named_lines, 'thinkthen/NativeWait.java': waits, 'thinkthen/UsagePersistence.java': java, 'thinkthen/NativeUsageLayouts.java': layouts,
            'kotlin/UsagePersistence.kt': kotlin, 'scala/UsagePersistence.scala': scala}
