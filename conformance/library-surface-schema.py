"""Derive generated declarations by calling their existing schema renderers."""
import importlib
import importlib.util
import json
from pathlib import Path
import sys
import tempfile


def contracts(root, authority):
    request = json.loads((root / authority['request']).read_text())
    result = json.loads((root / authority['result']).read_text())
    definitions = request['$defs']
    call = definitions['RequestCall']
    if 'RequestFunction' in definitions:
        function = definitions['RequestFunction']
        functions = function.get('enum', [item['const'] for item in function.get('oneOf', [])])
    else:
        functions = [item['properties']['function']['const'] for item in call['oneOf']]
    return request, result, functions


def generated_sources(root, target, request, result):
    generator_folder = root / 'sdlc/generators/results'
    sys.path[:0] = [str(generator_folder), str(generator_folder / 'templates')]
    generator = importlib.import_module('generate')
    module_name = {'objc': 'objc', 'jvm': 'jvm'}.get(target, target)
    template = importlib.import_module(module_name) if target != 'python' else importlib.import_module('templates.python')
    def graph(schema, roots):
        return generator.prepare(generator.graph(schema, roots))
    request = json.loads(json.dumps(request))
    request['$defs']['Request'] = {k: v for k, v in request.items() if k != '$defs'}
    version = request['$defs']['RequestVersion']['oneOf'][0]['const']
    header = (root / 'libraries/c/include/thinkthen.h').read_text()
    outputs = {}
    roots = getattr(template, 'ROOTS', ('completesessionPacket',))
    if target in ('php', 'dart'):
        from native import immediate_schema
        result = immediate_schema(result)
    results = graph(result, roots)
    inputs = graph(request, getattr(template, 'INPUT_ROOTS', ('Request', 'EngineSettings', 'RequestSessionDescriptor')))
    if target == 'c':
        spec = importlib.util.spec_from_file_location('surface_c_header', root / 'sdlc/scripts/generate-c-header.py')
        header_generator = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(header_generator)
        outputs['thinkthen.h'] = header_generator.generate(root).decode()
    elif target == 'python':
        outputs['_native_results.pyi'] = template.types(results, stubs=True)
        if hasattr(template, 'named_calls'):
            outputs['_named_generated.py'] = template.named_calls(request)
    elif target == 'typescript':
        outputs['results_generated.d.ts'] = (template.declarations(results, inputs) if getattr(template, 'INPUT_ROOTS', ())
                                             else template.declarations(results))
    elif target == 'ruby':
        outputs['results_generated.rb'] = (template.render(results) + "module ThinkThen\n  class Client\n    REQUEST_VERSION = "
                                           + repr(version) + ".freeze\n  end\nend\n")
        if hasattr(template, 'rbs'):
            outputs['results_generated.rbs'] = template.rbs(results)
    elif target == 'cpp':
        outputs['results_generated.hpp'] = template.render(results, version) + template.usage(header)
        outputs['inputs_generated.hpp'] = template.input_render(inputs)
        spec = importlib.util.spec_from_file_location('surface_cpp_abi', root / 'sdlc/scripts/check-c-exports.py')
        abi = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(abi)
        from csharp import enum_values
        outputs['calls_generated.hpp'] = template.named_calls(
            enum_values(request['$defs']['RequestFunction']),
            abi.declarations(root / 'libraries/c/include/thinkthen.h')['functions'])
    elif target == 'csharp':
        outputs['CompleteFacts.g.cs'] = generator.generated(result, 'csharp')
        outputs['NativeBridge.g.cs'] = generator.bridge()
        outputs['RequestInputs.g.cs'] = template.render_inputs(request, generator.prepare, generator.graph)
        outputs['NamedCalls.g.cs'] = (template.named_calls(template.enum_values(request['$defs']['RequestFunction']))
                                      + template.answer_helpers(graph(result, ('completesessionPacket', 'completeplan'))))
    elif target == 'swift':
        outputs['ResultsGenerated.swift'] = template.render(results)
        outputs['InputsGenerated.swift'] = template.render(inputs, True) + '\npublic let ownedRequestVersion = ' + json.dumps(version) + '\n'
    elif target == 'dart':
        outputs['results_generated.dart'] = template.render(results)
        outputs['inputs_generated.dart'] = template.render(inputs, True) + '\nconst requestVersion = ' + json.dumps(version) + ';\n'
        outputs['calls_generated.dart'] = template.calls(inputs, results)
        spec = importlib.util.spec_from_file_location('surface_dart_abi', root / 'sdlc/scripts/check-c-exports.py')
        abi = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(abi)
        outputs['abi_generated.dart'] = template.bridge(abi.header_abi(root / 'libraries/c/include/thinkthen.h'), results)
    elif target == 'go':
        outputs['owned_results_generated.go'] = template.render(results, inputs) + '\nconst OwnedRequestVersion = ' + json.dumps(version) + '\n' + template.usage(header)
    elif target == 'php':
        outputs.update(template.outputs(results, header))
        outputs['inputs_generated.php'] = template.inputs(inputs)
    elif target == 'objc':
        outputs.update(template.outputs(results, version))
    elif target == 'jvm':
        outputs['Results.java'] = template.render(results)
        selected, pending = {}, list(template.INPUT_ROOTS)
        while pending:
            key = pending.pop()
            if key not in selected:
                source = selected[key] = inputs[key]
                pending.extend(generator.references(source))
                pending.extend(child for child, _ in source.get('variants', []))
        outputs['Inputs.java'] = template.inputs(selected)
        usage = importlib.import_module('jvm_usage')
        outputs.update({filename: '\n'.join(lines) + '\n'
                        for filename, lines in usage.outputs(root).items()})
    elif target == 'zig':
        outputs['plan_generated.zig'] = template.render(graph(result, ('completeplan',)))
        outputs['request_generated.zig'] = template.render(inputs)
    elif target == 'ada':
        # Reuse the production generator, including its friendly schema projections.
        # Redirect only output paths; neither the source tree nor its templates change.
        original_path = template.output_path
        with tempfile.TemporaryDirectory(prefix='library-surface-ada-') as folder:
            temporary = Path(folder)
            try:
                template.output_path = lambda root, target, name: temporary / name
                template.generate(root, generator.graph, generator.prepare,
                                  generator.child_env(keep=('LANG', 'LC_ALL', 'TMPDIR')), False)
                outputs.update({path.name: path.read_text() for path in temporary.glob('*.ads')})
            finally:
                template.output_path = original_path
    elif target == 'cobol':
        outputs.update(template.outputs(root, graph(request, ('Request', 'RequestSessionDescriptor'))))
    # R's generated Rust metadata has no host declaration syntax. Its NAMESPACE
    # remains the authoritative public export edge; source class labels are not exports.
    return outputs


def references(value):
    if isinstance(value, dict):
        if '$ref' in value:
            yield value['$ref'].split('/')[-1]
        for child in value.values():
            yield from references(child)
    elif isinstance(value, list):
        for child in value:
            yield from references(child)


def properties(schema, name, path=()):
    definitions = schema['$defs']
    pending, seen, fields = [(definitions[name], tuple(path))], set(), set()
    while pending:
        source, remaining = pending.pop()
        if not isinstance(source, dict):
            continue
        if remaining:
            child = source.get('properties', {}).get(remaining[0])
            if child is not None:
                pending.append((child, remaining[1:]))
        else:
            fields.update(source.get('properties', {}))
        for union in ('oneOf', 'anyOf', 'allOf'):
            pending.extend((child, remaining) for child in source.get(union, []))
        if '$ref' in source:
            key = source['$ref'].split('/')[-1]
            if (key, remaining) not in seen:
                seen.add((key, remaining))
                pending.append((definitions[key], remaining))
    return fields


def reachable(schema):
    pending = list(references({k: v for k, v in schema.items() if k != '$defs'}))
    selected = {}
    while pending:
        name = pending.pop()
        if name not in selected:
            selected[name] = schema['$defs'][name]
            pending.extend(references(selected[name]))
    return selected
