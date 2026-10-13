"""The outputs owned by the schema generators, grouped by producer invocation."""
from dataclasses import dataclass


@dataclass(frozen=True)
class Output:
    target: str
    mode: str
    path: str
    producer: str = 'results'


def group(target, mode, base, *names):
    return tuple(Output(target, mode, base + name) for name in names)


OUTPUTS = (
    *group('c', 'results', 'libraries/c/src/session/', 'views_generated.rs'),
    Output('c', 'header', 'libraries/c/include/thinkthen.h', 'header'),
    *group('csharp', 'results', 'sdlc/generators/results/csharp/', 'CompleteFacts.g.cs'),
    *group('csharp', 'inputs', 'libraries/csharp/src/', 'RequestInputs.g.cs'),
    *group('csharp', 'bridge', 'libraries/csharp/src/', 'NativeBridge.g.cs'),
    *group('r', 'results', 'libraries/r/thinkthen/src/rust/src/', 'results_generated.rs'),
    *group('python', 'results', 'libraries/python/src/', 'results_generated.rs'),
    *group('python', 'results', 'libraries/python/thinkthen/', '_native_results.py', '_native_results.pyi'),
    *group('zig', 'results', 'libraries/zig/src/', 'plan_generated.zig'),
    *group('zig', 'inputs', 'libraries/zig/src/', 'request_generated.zig'),
    *group('ruby', 'results', 'libraries/ruby/lib/thinkthen/', 'results_generated.rb'),
    *group('ruby', 'results', 'libraries/ruby/src/ffi/', 'results_generated.rs'),
    *group('typescript', 'results', 'libraries/typescript/', 'results_generated.js', 'results_generated.d.ts'),
    *group('go', 'results', 'libraries/go/', 'owned_results_generated.go'),
    *group('php', 'results', 'libraries/php/src/session/', 'results_generated.php', 'graph_generated.json', 'ffi_generated.h', 'constants_generated.php', 'inputs_generated.php'),
    *group('cpp', 'results', 'libraries/cpp/include/thinkthen/', 'results_generated.hpp'),
    *group('cpp', 'inputs', 'libraries/cpp/include/thinkthen/', 'inputs_generated.hpp'),
    *group('dart', 'results', 'libraries/dart/lib/src/session/', 'results_generated.dart'),
    *group('dart', 'inputs', 'libraries/dart/lib/src/session/', 'inputs_generated.dart'),
    *group('dart', 'bridge', 'libraries/dart/lib/src/session/', 'abi_generated.dart'),
    *group('ada', 'results', 'libraries/ada/src/', 'thinkthen-requests.ads', 'thinkthen-requests.adb', 'thinkthen-sessions-calls.ads', 'thinkthen-sessions-calls.adb', 'thinkthen_session_c.ads'),
    *group('objc', 'results', 'libraries/objective-c/Sources/Foundation/', 'TTResults.g.h', 'TTResults.g.m'),
    *group('swift', 'results', 'libraries/swift/Sources/ThinkThen/', 'ResultsGenerated.swift'),
    *group('swift', 'inputs', 'libraries/swift/Sources/ThinkThen/', 'InputsGenerated.swift'),
    *group('swift', 'bridge', 'libraries/swift/Sources/ThinkThen/', 'ABIGenerated.swift'),
    *group('swift', 'bridge', 'libraries/swift/Sources/CThinkThen/', 'bridge.c'),
    *group('cobol', 'results', 'libraries/cobol/', 'src/tt_requests_generated.h', 'src/tt_requests_generated.c', 'copybooks/tt-native-generated.cpy'),
    *group('jvm', 'results', 'libraries/jvm/session/thinkthen/', 'Results.java', 'Inputs.java', 'RequestVersion.java', 'NativeWait.java', 'UsagePersistence.java', 'NativeUsageLayouts.java'),
    *group('jvm', 'results', 'libraries/jvm/session/kotlin/', 'Results.kt', 'UsagePersistence.kt'),
    *group('jvm', 'results', 'libraries/jvm/session/scala/', 'Results.scala', 'UsagePersistence.scala'),
)


def generated_paths():
    return {output.path for output in OUTPUTS}


def invocations():
    return tuple(dict.fromkeys((output.producer, output.target, output.mode)
                              for output in OUTPUTS))


def paths(target, mode):
    return tuple(output.path for output in OUTPUTS
                 if output.target == target and output.mode == mode)


def output_path(root, target, name):
    candidates = [output.path for output in OUTPUTS
                  if output.target == target and output.path.endswith('/' + name)]
    if len(candidates) != 1:
        raise ValueError(f'unknown or ambiguous generated output: {target}/{name}')
    return root / candidates[0]
