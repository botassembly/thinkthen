"""Real Git replacement and incomplete-pin refusals before counted tool starts."""
import json
import os
import shutil
import subprocess
from pathlib import Path

from inputs import HERE
from release_pack_cases import REPO


def replacement_controls(case, source, commit, build, env, write_script):
    def git(*args, data=None):
        result = subprocess.run(['git', '-C', str(source), *args], env=case.env,
                                input=data, capture_output=True, text=True, check=True)
        return result.stdout.strip()

    blob = git('rev-parse', 'HEAD:source.cc')
    tree = git('rev-parse', 'HEAD^{tree}')
    changed_blob = git('hash-object', '-w', '--stdin', data='replacement source\n')
    rows = git('ls-tree', 'HEAD').replace(blob, changed_blob)
    changed_tree = git('mktree', data=rows + '\n')
    changed_commit = git('-c', 'user.name=fixture', '-c', 'user.email=fixture@example.invalid',
                         'commit-tree', changed_tree, '-p', commit, data='replacement\n')
    marker = case.root / 'replacement-tool-starts'
    counted = case.root / 'replacement-shim'
    for name in ('cargo', 'cmake', 'rustc', 'readelf', 'duckdb', 'cc', 'c++', 'make'):
        write_script(counted / name, f'echo {name} >> "{marker}"; exit 99')
    refused_env = {**env, 'PATH': f'{counted}:{env["PATH"]}'}
    validator = [os.sys.executable, '-B', str(HERE / 'validate_inputs.py'), '--source', str(source),
                 '--commit', commit, '--static', env['THINKTHEN_DUCKDB_CPP_STATIC_DIR'],
                 '--manifest', str(build.parent / 'archive-sha256.txt')]
    for name, old, new in [('commit', commit, changed_commit), ('tree', tree, changed_tree),
                           ('blob', blob, changed_blob)]:
        git('replace', old, new)
        try:
            # Git still reports the pinned HEAD, but ordinary readers follow the replacement.
            case.assertEqual(git('rev-parse', 'HEAD'), commit)
            case.assertEqual(git('show', 'HEAD:source.cc'), 'replacement source')
            (source / 'source.cc').write_text('replacement source\n')
            for command in (validator, ['/bin/sh', str(build)]):
                result = subprocess.run(command, env=refused_env, text=True, capture_output=True, check=False)
                case.assertEqual(result.returncode, 1, result.stderr)
                case.assertIn('raw Git source file differs: source.cc', result.stderr)
            case.assertFalse(marker.exists(), 'a refused replacement reached a tool')
            # Ordinary raw bytes remain valid with the replacement ref still installed.
            (source / 'source.cc').write_text('original\n')
            result = subprocess.run(validator, env=refused_env, text=True, capture_output=True, check=False)
            case.assertEqual(result.returncode, 0, result.stderr)
        finally:
            # Only refs created inside this run's TemporaryDirectory are removed.
            case.assertTrue(source.is_relative_to(case.root) and case.root != REPO)
            git('replace', '-d', old)
            (source / 'source.cc').write_text('original\n')
        print(f'raw {name} replacement: refused exit 1; tool starts 0; restored positive exit 0')
    foreign = case.root / 'foreign-git'
    shutil.copytree(source, foreign)
    subprocess.run(['git', '-C', str(foreign), 'update-ref', 'refs/foreign/' + commit, changed_commit],
                   env=case.env, check=True, capture_output=True)
    git_overrides = {
        'GIT_DIR': str(foreign / '.git'), 'GIT_WORK_TREE': str(foreign),
        'GIT_COMMON_DIR': str(foreign / '.git'), 'GIT_INDEX_FILE': str(case.root / 'foreign-index'),
        'GIT_OBJECT_DIRECTORY': str(foreign / '.git/objects'),
        'GIT_ALTERNATE_OBJECT_DIRECTORIES': str(foreign / '.git/objects'),
        'GIT_CONFIG_COUNT': '1', 'GIT_CONFIG_KEY_0': 'core.worktree', 'GIT_CONFIG_VALUE_0': str(foreign),
        'GIT_CONFIG_PARAMETERS': "'core.worktree=invalid'", 'GIT_CONFIG': str(case.root / 'foreign-config'),
        'GIT_CONFIG_SYSTEM': str(case.root / 'foreign-config'),
        'GIT_CONFIG_GLOBAL': str(case.root / 'foreign-config'), 'GIT_REPLACE_REF_BASE': 'refs/foreign/',
        'GIT_CEILING_DIRECTORIES': str(source),
    }
    override_env = {**refused_env, **git_overrides}
    (source / 'source.cc').write_text('replacement source\n')
    for command in (validator, ['/bin/sh', str(build)]):
        result = subprocess.run(command, env=override_env, text=True, capture_output=True, check=False)
        case.assertEqual(result.returncode, 1, result.stderr)
        case.assertIn('raw Git source file differs: source.cc', result.stderr)
    case.assertFalse(marker.exists())
    (source / 'source.cc').write_text('original\n')
    result = subprocess.run(validator, env=override_env, text=True, capture_output=True, check=False)
    case.assertEqual(result.returncode, 0, result.stderr)
    print('inherited Git overrides: refused exit 1; tool starts 0; restored positive exit 0')


def node_controls(case, write_script):
    tree = case.root / 'node-tree'
    for name in ('site/scripts', 'databases/duckdb/tools'):
        shutil.copytree(REPO / name, tree / name, ignore=shutil.ignore_patterns('__pycache__'))
    pins = tree / 'databases/duckdb/tools/version.env'
    original = pins.read_text()
    marker = case.root / 'node-child-starts'
    shim = case.root / 'node-shim'
    # Count resolution shells too, before any fake build or host command can run.
    for name in ('sh', 'cargo', 'rustc', 'cmake', 'duckdb', 'git', 'cc', 'c++', 'make', 'psql'):
        write_script(shim / name, f'echo {name} >> "{marker}"; exit 99')
    env = {key: value for key, value in case.env.items() if not key.startswith('THINKTHEN_')}
    env['PATH'] = f'{shim}:{env["PATH"]}'
    module = (tree / 'site/scripts/duckdb-inputs.mjs').as_uri()
    code = f"import {{readDuckDBVersions}} from {json.dumps(module)}; console.log(JSON.stringify(readDuckDBVersions(process.argv[1])));"
    reader = ['node', '--input-type=module', '-e', code, str(pins)]
    smoke = ['node', str(tree / 'site/scripts/smoke-sql.mjs'), 'install/duckdb']
    plants = [('pinless version', original.replace('v1.5.5 v1.5.4', 'v1.5.5 v1.5.4 v9.9.9'), 'source_commit')]
    # Independently remove every required field for both versions and all four targets.
    for line in original.splitlines():
        if line.startswith(('DUCKDB_V1_5_5_', 'DUCKDB_V1_5_4_')):
            field = line.split('=', 1)[0]
            plants.append((field, original.replace(line + '\n', ''), 'missing or malformed DuckDB'))
    plants += [
        ('bad hash', original.replace('DUCKDB_V1_5_4_OSX_AMD64_CLI_SHA256=',
                                     'DUCKDB_V1_5_4_OSX_AMD64_CLI_SHA256=x'), 'cli_sha256'),
        ('unsafe manifest', original.replace('archive-sha256-v1.5.4-osx-amd64.txt', '../outside.txt'), 'manifest'),
        ('unsafe requirements', original.replace('requirements-v1.5.4.txt', '../outside.txt'), 'requirements'),
        ('duplicate assignment', original + 'DUCKDB_VERSIONS="v1.5.5 v1.5.4"\n', 'duplicate'),
        ('malformed assignment', original + 'DUCKDB_TEST=$(touch marker)\n', 'malformed'),
        ('duplicate version', original.replace('v1.5.5 v1.5.4', 'v1.5.5 v1.5.5'), 'duplicate'),
    ]
    for name, text, cause in plants:
        pins.write_text(text)
        for command in (reader, smoke):
            result = subprocess.run(command, env=env, capture_output=True, text=True, check=False, timeout=10)
            case.assertEqual(result.returncode, 1, (name, result.stderr))
            case.assertIn(cause, result.stderr, name)
        case.assertFalse(marker.exists(), f'{name} started a child')
    pins.write_text(original)
    result = subprocess.run(reader, env=env, capture_output=True, text=True, check=False)
    case.assertEqual((result.returncode, json.loads(result.stdout)), (0, ['v1.5.5', 'v1.5.4']))
    print(f'Node incomplete/malformed pin plants: {len(plants)}; reader/smoke refusals {2 * len(plants)}; child starts 0; restored positive exit 0')
