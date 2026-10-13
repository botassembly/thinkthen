#!/usr/bin/env python3
"""Check public host declarations against schemas and approved native adapters."""
import argparse
import importlib.util
import json
from pathlib import Path
import re
import sys
import subprocess

ROOT = Path(__file__).resolve().parents[1]


def support(name):
    spec = importlib.util.spec_from_file_location(name.replace('-', '_'), Path(__file__).with_name(name + '.py'))
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


READ = support('library-surface-declarations')
SCHEMA = support('library-surface-schema')


def canonical(name):
    name = name.strip('`').removeprefix('set_').removeprefix('T_')
    name = re.sub(r'([a-z0-9])([A-Z])', r'\1_\2', name)
    return name.replace('-', '_').lower()


def native(name, style):
    parts = name.split('_')
    if style in ('pascal', 'camel'):
        value = ''.join(part[:1].upper() + part[1:] for part in parts)
        return value[:1].lower() + value[1:] if style == 'camel' else value
    return name.upper().replace('_', '-') if style == 'upper' else name


def language(path, default):
    return {'.kt': 'kotlin', '.scala': 'scala', '.h': 'c' if default == 'cobol' else default}.get(path.suffix, default)


def members(declared, containers, prefix=''):
    found = set()
    for item in declared:
        if any(item.owner == container or container.endswith('.*') and item.owner.startswith(container[:-1]) for container in containers):
            found.add(canonical(item.name.removeprefix(prefix)))
    return found


def function_errors(declared, functions, config, containers, facade=''):
    if config['language'] == 'java':
        declared = READ.public_inherited(declared)
    names = {canonical(d.name) for d in declared if d.kind == 'call' and d.owner in containers}
    errors = []
    for function in functions:
        name = config.get('function_prefix', '') + native(function, config['naming']) + config.get('function_suffix', '')
        if canonical(name) not in names:
            errors.append('missing function: ' + function + (' (' + facade + ')' if facade else ''))
    return errors


def facade_errors(declared, functions, manifest, config):
    errors = []
    for facade, container in config.get('facades', {}).items():
        errors.extend(function_errors(declared, functions, config, [container], facade))
        for helper, kind in manifest['helpers'].items():
            if kind != 'call':
                continue
            name = native(helper, config['naming'])
            if not any(d.kind == 'call' and d.owner == container and d.name == name for d in declared):
                errors.append('missing helper: ' + helper + ' (' + facade + ')')
    return errors


def helper_errors(declared, manifest, config):
    errors = []
    helper_names = config.get('helper_names', {})
    call_containers = {'', *config.get('function_containers', [])}
    for helper, kind in manifest['helpers'].items():
        wanted = helper_names.get(helper, config.get('function_prefix', '') + native(helper, config['naming']))
        if '.' in wanted:
            container, member = wanted.rsplit('.', 1)
            kinds = ('call', 'field') if kind == 'builder' else ('call',)
            found = any(d.owner == container and d.name == member and d.kind in kinds for d in declared)
        elif kind == 'builder':
            found = any(d.name == wanted and d.kind in ('type', 'export') for d in declared)
        else:
            found = any(d.owner in call_containers and canonical(d.name) == canonical(wanted)
                        and d.kind == 'call' for d in declared)
        if not found:
            errors.append('missing helper: ' + helper)
    return errors


def dart_exports(root, config):
    entry = root / config['entrypoint']
    pending, filters = [entry], {entry: None}
    while pending:
        path = pending.pop()
        source = path.read_text()
        for match in re.finditer(r"\b(export|part)\s+'([^']+)'([^;]*);", source):
            if ':' in match[2]:
                continue
            child = (path.parent / match[2]).resolve()
            if child not in filters:
                show = re.search(r'\bshow\s+([\w\s,]+)', match[3])
                filters[child] = set(re.findall(r'\w+', show[1])) if show else None
                pending.append(child)
    return filters


def exported(declared, names):
    if names is None:
        return declared
    return {d for d in declared if d.name in names and not d.owner or d.owner in names}


def check(root, manifest, name):
    config = manifest['libraries'][name]
    request, result, functions = SCHEMA.contracts(root, manifest['authority'])
    sys.path.insert(0, str(root / 'sdlc/generators/results'))
    from registry import OUTPUTS
    suffixes = {
        'python': ('.pyi',), 'typescript': ('.d.ts',), 'ruby': ('.rb',),
        'c': ('.h',), 'cpp': ('.hpp',), 'csharp': ('.cs',), 'go': ('.go',),
        'java': ('.java', '.kt', '.scala'), 'swift': ('.swift',), 'zig': ('.zig',),
        'php': ('.php',), 'dart': ('.dart',), 'ada': ('.ads',), 'objc': ('.h',),
        'cobol': ('.h', '.cpy'), 'r': (),
    }
    generated_paths = [output.path for output in OUTPUTS
                       if output.target == name and output.path.endswith(suffixes[config['language']])]

    errors, declared, source_files = [], set(), {}
    exports = dart_exports(root, config) if config.get('entrypoint') else None
    for pattern in dict.fromkeys(config['declarations'] + generated_paths):
        for path in sorted(root.glob(pattern)):
            relative = path.relative_to(root).as_posix()
            if relative in manifest.get('exceptions', {}).get(name, {}).get('paths', []):
                continue
            if exports is not None and path.resolve() not in exports:
                continue
            source_files[relative] = path
            try:
                found = READ.declarations(path.read_text(), language(path, config['language']))
                declared.update(exported(found, exports[path.resolve()]) if exports is not None else found)
            except (SyntaxError, ValueError) as error:
                errors.append(f'{relative}: declaration parse failed: {error}')
    if not source_files:
        errors.append('no public declaration files')
    promoted_owners = {owner: READ.go_owners(declared, owner) for owner in config.get('representation_embedded_members', {})}
    if config['language'] == 'go':
        declared = READ.public_go(declared)
    if config['language'] == 'r':
        exports = {d.name for d in declared if d.kind == 'export'}
        declared = {d for d in declared if d.kind == 'export' or d.kind == 'call' and d.name in exports or d.owner in exports}
    names = {canonical(d.name) for d in declared if d.kind in ('call', 'export')}
    errors.extend(function_errors(declared, functions, config, config['function_containers']))
    errors.extend(facade_errors(declared, functions, manifest, config))
    for kind, schema_name in [('option', 'RequestOptions'), ('setting', 'EngineSettings')]:
        containers = config['option_containers' if kind == 'option' else 'setting_containers']
        found = members(declared, containers, config.get('field_prefix', ''))
        for field in request['$defs'][schema_name]['properties']:
            if canonical(field) not in found:
                errors.append(f'missing {kind}: {field} in {", ".join(containers)}')
    helper_names = config.get('helper_names', {})
    errors.extend(helper_errors(declared, manifest, config))
    expected = set()
    try:
        sources = SCHEMA.generated_sources(root, name, request, result)
        for filename, source in sources.items():
            paths = [path for path in source_files.values() if path.name == Path(filename).name]
            if not paths:
                # Metadata and private bridge outputs are not host exports.
                if filename.endswith(suffixes[config['language']]):
                    errors.append('missing generated declaration file: ' + filename)
                continue
            lang = language(paths[0], config['language'])
            wanted = READ.declarations(source, lang)
            if lang == 'go':
                wanted = READ.public_go(wanted)
            if exports is not None:
                wanted = exported(wanted, exports[paths[0].resolve()])
            wanted = {d for d in wanted if d.kind != 'inherited'}
            expected.update(wanted)
            actual = READ.declarations(paths[0].read_text(), lang)
            if lang == 'go':
                actual = READ.public_go(actual)
            if exports is not None:
                actual = exported(actual, exports[paths[0].resolve()])
            actual = {d for d in actual if d.kind != 'inherited'}
            for item in sorted(wanted - actual):
                errors.append(f'missing generated declaration: {filename}:{item.owner}.{item.name}')
            for item in sorted(actual - wanted):
                errors.append(f'unlisted generated declaration: {filename}:{item.owner}.{item.name}')
    except (KeyError, ValueError, OSError, TypeError, AttributeError, ImportError, RuntimeError, subprocess.SubprocessError) as error:
        errors.append('schema declaration derivation failed: ' + str(error))
    categories = manifest['categories']
    allowed_types = set(categories['lifecycle'] + categories['errors'])
    allowed_calls = set()
    allowed_calls.update(canonical(config.get('function_prefix', '') + native(n, config['naming'])) for n in functions + list(manifest['helpers']))
    allowed_calls.update(canonical(config.get('function_prefix', '') + native(n, config['naming']) + config.get('function_suffix', '')) for n in functions)
    allowed_calls.update(canonical(n.split('.')[-1]) for n in helper_names.values())
    allowed_native_calls = set()
    for category, owners in [('lifecycle_members', config['lifecycle_owners']),
                             ('representation_members', config['representation_owners'])]:
        for owner in owners:
            for member in categories[category] + config.get(category, {}).get(owner, []):
                allowed_native_calls.add((owner, canonical(member)))
                if category == 'lifecycle_members' and config.get('function_suffix'):
                    allowed_native_calls.add((owner, canonical(native(member, config['naming']) + config['function_suffix'])))
    allowed_native_calls.update((owner, canonical(member))
                                for owner in ['', *config['representation_owners']]
                                for member in config.get('representation_calls', []))
    for embedded, owners in promoted_owners.items():
        allowed_native_calls.update((owner, canonical(member)) for owner in owners
                                    for member in config['representation_embedded_members'][embedded])
    schema_types = SCHEMA.reachable(request) | SCHEMA.reachable(result)
    allowed_types.update(config.get('representation_types', []))
    allowed_types.update(d.name for d in expected if d.kind in ('type', 'export'))
    allowed_types.update(schema_types)
    allowed_types.update('Input' + n for n in schema_types)
    exception_names = set(manifest.get('exceptions', {}).get(name, {}).get('names', []))
    for item in sorted(declared - expected):
        if item.kind == 'type' or item.kind == 'export' and item.name[:1].isupper():
            if item.name not in allowed_types and canonical(item.name) not in allowed_calls and (item.owner, canonical(item.name)) not in allowed_native_calls:
                errors.append(f'unlisted public type/export: {item.owner}.{item.name}')
        elif item.kind in ('call', 'export'):
            if canonical(item.name) not in allowed_calls and (item.owner, canonical(item.name)) not in allowed_native_calls and item.name not in config.get('wrapper_members', {}).get(item.owner, []):
                errors.append(f'unlisted public call/export: {item.owner}.{item.name}')
        elif item.kind == 'field':
            if canonical(item.name) in exception_names:
                continue
            if any(item.owner == n or n.endswith('.*') and item.owner.startswith(n[:-1]) for n in config['option_containers']) and canonical(item.name) in request['$defs']['RequestOptions']['properties']:
                continue
            if any(item.owner == n or n.endswith('.*') and item.owner.startswith(n[:-1]) for n in config['setting_containers']) and canonical(item.name) in request['$defs']['EngineSettings']['properties']:
                continue
            owner_type = item.owner.removeprefix('Input')
            if owner_type in schema_types and canonical(item.name) in schema_types[owner_type].get('properties', {}):
                continue
            # Function parameter names are checked as options only in declared option containers.
            if '.' in item.owner and not any(item.owner == n or n.endswith('.*') and item.owner.startswith(n[:-1]) for n in config['option_containers'] + config['setting_containers']):
                continue
            function = item.owner.rsplit('.', 1)[-1]
            authored = 'Authored_' + config.get('question_schema_aliases', {}).get(function, function)
            if function in functions and authored in request['$defs'] and canonical(item.name) in SCHEMA.properties(request, authored):
                continue
            adapter = config.get('field_adapters', {}).get(item.owner)
            if adapter:
                field = adapter.get('aliases', {}).get(item.name, canonical(item.name))
                contract = request if adapter['contract'] == 'request' else result
                types = adapter['type'] if isinstance(adapter['type'], list) else [adapter['type']]
                if any(field in SCHEMA.properties(contract, key) for key in types):
                    continue
            if item.name in config.get('wrapper_members', {}).get(item.owner, []):
                continue
            if canonical(item.name) not in {canonical(n) for n in categories.get('lifecycle_fields', [])}:
                errors.append(f'unlisted public field: {item.owner}.{item.name}')
    return sorted(set(errors))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--root', type=Path, default=ROOT)
    parser.add_argument('--manifest', type=Path)
    parser.add_argument('--library', action='append', help='check only this host; repeatable')
    parser.add_argument('--json', action='store_true')
    args = parser.parse_args()
    try:
        root = args.root.resolve()
        manifest = json.loads((args.manifest or root / 'conformance/library-surface.json').read_text())
        libraries = args.library or list(manifest['libraries'])
        unknown = set(libraries) - manifest['libraries'].keys()
        if unknown:
            raise ValueError('unknown libraries: ' + ', '.join(sorted(unknown)))
        reports = {name: check(root, manifest, name) for name in libraries}
        if args.json:
            print(json.dumps({'libraries': reports}, indent=2, sort_keys=True))
        else:
            for name, errors in reports.items():
                print(f'{name}: {len(errors)} surface discrepancies' if errors else name + ': pass')
                for error in errors:
                    print('  ' + error)
        return int(any(reports.values()))
    except (OSError, ValueError, KeyError) as error:
        print('library-surface: ' + str(error), file=sys.stderr)
        return 2


if __name__ == '__main__':
    raise SystemExit(main())
