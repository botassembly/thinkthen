"""Generate owned JVM status types and carrier measurements from the C compiler."""
import importlib.util


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
    return {'thinkthen/UsagePersistence.java': java, 'thinkthen/NativeUsageLayouts.java': layouts,
            'kotlin/UsagePersistence.kt': kotlin, 'scala/UsagePersistence.scala': scala}
