"""Shared guide extraction, isolated scenario execution and observable assertions."""
from contextlib import contextmanager
from pathlib import Path
import os
import json
import re
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parent.parent
IDEAL = ROOT / 'docs/surfaces'


def blocks(path):
    """Inventory all fences, including setup and output, without dropping blocks."""
    text = path.read_text(encoding='utf-8')
    return [(match[1], match[2], text[:match.start()].count('\n') + 1)
            for match in re.finditer(r'^```([^\n]*)\n(.*?)^```\s*$', text, re.M | re.S)]


def inventory():
    return {path.stem: blocks(path) for path in sorted(IDEAL.glob('*.md'))
            if path.stem not in ('c-cpp',)}


def environment():
    # Keep platform runtime variables; isolate caller configuration and credentials.
    result = {key: value for key, value in os.environ.items()
              if not key.startswith('THINKTHEN_') and not key.endswith(('_API_KEY', '_TOKEN', '_SECRET', '_PASSWORD'))}
    result.pop('PYTHONPATH', None)
    return result


@contextmanager
def scenario(name, *, case=None, observer=None):
    """Select backend replies, not a universal final row state.

    Definitions: crates/thinkthen/test-support/server/guide.rs.
    With canonical guide domains/bars:
      yes/no: predicate .95/.05; tag no is answered with an empty label list.
      unsure: predicate .5 and uniform choices; recognize/relate answer empty.
      relations: populated structural names/relations; low-score: lowest level.
      failed: HTTP 503 for primitives; annotate still answers a form whose
              entries failed. A targeted child scenario leaves siblings answered.
    A one-cut decide cannot be unsure: set two bars when testing that branch.
    case= replaces the named replies with the supplied existing fixture rules;
    do not infer outcomes from name when a case override is supplied.
    """
    executable = os.environ.get('THINKTHEN_LOOPBACK')
    if not executable:
        raise RuntimeError('THINKTHEN_LOOPBACK must name the built private fixture')
    with tempfile.TemporaryDirectory(prefix='guide-scenario-') as directory:
        folder = Path(directory)
        env = environment()
        for key, child in (('HOME', 'home'), ('XDG_CONFIG_HOME', 'config'), ('XDG_CACHE_HOME', 'cache')):
            target = folder / child
            target.mkdir()
            env[key] = str(target)
        arguments = [executable, '--guide-scenario', name]
        if case is not None:
            path = folder / 'case.json'
            path.write_text(json.dumps(case))
            arguments = [executable, str(path)]
        with subprocess.Popen(arguments, stdin=subprocess.PIPE,
                              stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True) as server:
            try:
                port = server.stdout.readline().strip()
                base = 'http://127.0.0.1:' + port + '/v1'
                if not port.isdigit():
                    raise RuntimeError('guide fixture did not supply its loopback address: ' + base)
                env.update(THINKTHEN_BACKEND='systemone', THINKTHEN_API_KEY='synthetic-loopback-key',
                           THINKTHEN_BASE_URL=base)
                if observer is not None:
                    observer.append(server)
                yield folder, env
            finally:
                server.stdin.close()
                try:
                    server.wait(timeout=15)
                except subprocess.TimeoutExpired:
                    server.kill()
                    server.wait()
                    raise RuntimeError('guide fixture did not stop after stdin closed')
                if server.returncode:
                    raise RuntimeError('guide fixture failed: ' + server.stderr.read())


def check(label, result, *, output=None, branch=None, exit_code=0, refusal=None,
          negative=None):
    """Assert observable behavior; an accepted promised refusal always fails."""
    if negative == 'output':
        output = 'deliberately wrong output\n'
    if negative == 'branch':
        branch = 'deliberately wrong branch'
    if negative == 'exit':
        exit_code = 99
    errors = []
    if result.returncode != exit_code:
        errors.append(f'exit: expected {exit_code}, got {result.returncode}')
    if output is not None and result.stdout != output:
        errors.append(f'output: expected {output!r}, got {result.stdout!r}')
    if branch is not None and branch not in result.stdout.splitlines():
        errors.append(f'branch: expected {branch!r}')
    if refusal is not None and (result.returncode == 0 or not re.search(refusal, result.stdout + result.stderr)):
        errors.append('promised refusal was accepted or its diagnostic was missing')
    if errors:
        for stream, value in (('stdout', result.stdout), ('stderr', result.stderr)):
            if value:
                print(f'{label} {stream}:\n{value}', flush=True)
        raise AssertionError(label + ': ' + '; '.join(errors))
    print('PASS ' + label)


def run_python(python, negative=None):
    """Prove the current guide's literal conditional, plus its call refusals."""
    guide = ROOT / 'docs/surfaces/python.md'
    source = next(body for language, body, _ in blocks(guide)
                  if language == 'python' and 'print("refund queue" if is_refund else "inbox")' in body)
    for name, output, branch in (
            ('yes', 'refund queue\n', 'refund queue'),
            ('no', 'inbox\n', 'inbox'),
            ('unsure', 'a person reads it 0.5\n', 'a person reads it 0.5'),
            ('failed', None, None)):
        with scenario(name) as (folder, env):
            path = folder / 'example.py'
            path.write_text(source + '\n')
            result = subprocess.run([python, '-I', str(path)], cwd=folder, env=env,
                                    capture_output=True, text=True, timeout=60)
            kwargs = dict(output=output, branch=branch)
            if name == 'failed':
                kwargs.update(exit_code=1, refusal=r'Failed')
            control = negative if name == 'yes' and negative in ('output', 'branch') else None
            check('python/current/branch-on-the-answer/' + name, result, negative=control, **kwargs)
    with scenario('yes') as (folder, env):
        path = folder / 'refusal.py'
        # Documented public declaration: choices must be a list/domain, not text.
        statement = 'import thinkthen as tt\nclient = tt.Client()\nclient.choose("Which?", "body", choices="not-a-list")\n'
        path.write_text(statement.replace('choices="not-a-list"', 'choices=["one", "two"]') if negative == 'refusal' else statement)
        result = subprocess.run([python, '-I', str(path)], cwd=folder, env=env,
                                capture_output=True, text=True, timeout=60)
        check('python/current/choices/runtime-refusal', result, exit_code=1,
              refusal=r'(TypeError|UsageError)')
        # First prove the checker resolves the installed public package and accepts valid code.
        valid = folder / 'valid.py'
        valid.write_text(statement.replace('choices="not-a-list"', 'choices=["one", "two"]'))
        args = ['mypy', '--enable-incomplete-feature=TypeForm', '--strict', '--no-incremental',
                '--python-executable', python]
        result = subprocess.run([*args, str(valid)], cwd=folder, env=env,
                                capture_output=True, text=True, timeout=60)
        check('python/current/choices/compiler-valid-control', result)
        result = subprocess.run([*args, str(path)], cwd=folder, env=env,
                                capture_output=True, text=True, timeout=60)
        check('python/current/choices/compiler-refusal', result, exit_code=1,
              refusal=r'refusal.py:3: error:')


