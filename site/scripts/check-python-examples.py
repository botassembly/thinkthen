"""Keep ordinary Python examples on the module functions, without clients."""
import ast
from pathlib import Path
import re
import sys


def client_calls(source):
    tree = ast.parse(source)
    modules = {'thinkthen', 'thinkthen.aio', 'tt'}
    constructors = set()
    for node in ast.walk(tree):
        if isinstance(node, ast.Import):
            modules.update(alias.asname or alias.name for alias in node.names
                           if alias.name in {'thinkthen', 'thinkthen.aio'})
        if isinstance(node, ast.ImportFrom) and node.module in {'thinkthen', 'thinkthen.aio'}:
            constructors.update(alias.asname or alias.name for alias in node.names
                                if alias.name in {'Client', 'AsyncClient'})
            if node.module == 'thinkthen':
                modules.update(alias.asname or alias.name for alias in node.names
                               if alias.name == 'aio')
    constructors.update(f'{module}.{name}' for module in modules
                        for name in ('Client', 'AsyncClient'))
    return [node.lineno for node in ast.walk(tree) if isinstance(node, ast.Call)
            and ast.unparse(node.func) in constructors]


def sections(markdown):
    heading = ''
    for part in re.split(r'(?m)^(## .*)$', markdown):
        if part.startswith('## '):
            heading = part[3:].strip()
        else:
            yield heading, part


def check(root):
    page = root / 'site/src/content/docs/sdks/python.mdx'
    settings_ids = set()
    for heading, body in sections(page.read_text()):
        if heading == 'When to make a client':
            settings_ids.update(re.findall(r'<Example\s+id="([^"]+)"\s*/>', body))
    errors = []
    examples = root / 'site/examples'
    for source in sorted(examples.rglob('show.py')):
        example = source.parent.relative_to(examples).as_posix()
        if example in settings_ids:
            continue
        errors.extend(f'{source.relative_to(root)}:{line}: use a module function; '
                      'clients belong only in the client-settings section'
                      for line in client_calls(source.read_text()))
    readme = root / 'sdk/python/README.md'
    for heading, body in sections(readme.read_text()):
        if heading == 'When to make a client':
            continue
        for block in re.findall(r'(?ms)^```python\s*\n(.*?)^```\s*$', body):
            if client_calls(block):
                errors.append('sdk/python/README.md: use module functions outside '
                              'the client-settings section')
    return errors


if __name__ == '__main__':
    root = Path(sys.argv[1]).resolve() if len(sys.argv) == 2 else Path(__file__).resolve().parents[2]
    findings = check(root)
    for finding in findings:
        print(finding, file=sys.stderr)
    if findings:
        raise SystemExit(1)
    print('Python, pandas and Polars examples use module calls outside client settings')
