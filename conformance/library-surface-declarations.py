"""Read public declarations, excluding comments, strings and method bodies."""
import ast
import json
from dataclasses import dataclass
import re
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from conformance.children.children import child_env


@dataclass(frozen=True, order=True)
class Declaration:
    owner: str
    name: str
    kind: str


def python(source):
    tree = ast.parse(source)
    result = set()
    exports = None
    for node in tree.body:
        if isinstance(node, ast.Assign) and any(isinstance(t, ast.Name) and t.id == '__all__' for t in node.targets):
            exports = set(ast.literal_eval(node.value))
    def visit(nodes, owner=''):
        for node in nodes:
            name = getattr(node, 'name', None)
            if isinstance(node, ast.ClassDef):
                result.add(Declaration(owner, name, 'hidden_type' if name.startswith('_') else 'type'))
                for base in node.bases:
                    target = base.value if isinstance(base, ast.Subscript) else base
                    if isinstance(target, ast.Name):
                        result.add(Declaration(name, target.id, 'inherited'))
                visit(node.body, name)
            elif isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)):
                if not name.startswith('_') or name == '__init__':
                    is_property = any(isinstance(d, ast.Name) and d.id == 'property' for d in node.decorator_list)
                    result.add(Declaration(owner, name, 'field' if is_property else 'call'))
                    container = owner + '.' + name if owner else name
                    for arg in [*node.args.posonlyargs, *node.args.args]:
                        if arg.arg not in ('self', 'cls'):
                            result.add(Declaration(container, arg.arg, 'parameter'))
                    for arg in node.args.kwonlyargs:
                        result.add(Declaration(container, arg.arg, 'field'))
                    keyword = node.args.kwarg
                    if keyword and isinstance(keyword.annotation, ast.Subscript):
                        annotation = keyword.annotation
                        if isinstance(annotation.value, ast.Name) and annotation.value.id == 'Unpack' and isinstance(annotation.slice, ast.Name):
                            result.add(Declaration(container, annotation.slice.id, 'unpacked'))
            elif isinstance(node, ast.Assign) and isinstance(node.value, (ast.Name, ast.Subscript, ast.BinOp)):
                for target in node.targets:
                    if isinstance(target, ast.Name) and not target.id.startswith('_'):
                        result.add(Declaration(owner, target.id, 'type'))
            elif isinstance(node, ast.AnnAssign) and isinstance(node.target, ast.Name):
                if not node.target.id.startswith('_'):
                    result.add(Declaration(owner, node.target.id, 'field'))
                    result.add(Declaration(owner + '.' + node.target.id, ast.unparse(node.annotation), 'annotation'))
    visit(tree.body)
    if exports is not None:
        result = {d for d in result if d.owner or d.name in exports}
        result.update(Declaration('', name, 'export') for name in exports)
    return result


QUOTED = r'"(?:\\.|[^"\\])*"|\'(?:\\.|[^\'\\])*\''


def masked(source, comments=r'//[^\n]*|/\*[\s\S]*?\*/'):
    # Preserve positions and newlines for scope matching, but never match words in comments/strings.
    return re.sub(comments + '|' + QUOTED,
                  lambda m: ''.join('\n' if c == '\n' else ' ' for c in m[0]), source)


def ruby_masked(source):
    """Use Ruby's source lexer; never evaluate or load the inspected library."""
    requested = os.environ.get('RUBY', 'ruby')
    executable = shutil.which(requested)
    if executable is None:
        raise OSError('Ruby source lexer unavailable: RUBY or PATH must name Ruby with Ripper')
    lexer = Path(__file__).with_name('library-surface-ruby.rb')
    with tempfile.TemporaryDirectory(prefix='library-surface-lexer-') as home:
        run = subprocess.run([executable, '--disable-gems', str(lexer)], input=source,
                             capture_output=True, text=True, encoding='utf-8',
                             env=child_env(home=home, LANG='C.UTF-8', LC_ALL='C.UTF-8'))
    if run.returncode:
        raise ValueError('Ruby source lexer failed: ' + run.stderr.strip())
    parsed = json.loads(run.stdout)
    text = parsed['masked']
    if len(text) != len(source) or text.count('\n') != source.count('\n'):
        raise ValueError('Ruby source lexer failed: invalid masked source')
    owners = {int(line): owner for line, owner in parsed['owners'].items()}
    visibility = {int(line): event for line, event in parsed['visibility'].items()}
    return text, owners, parsed['private_constants'], visibility


def normalized_annotation(source):
    """Normalize type syntax while preserving quoted literal content."""
    quoted = r'("(?:\\.|[^"\\])*"|\'(?:\\.|[^\'\\])*\'|`(?:\\.|[^`\\])*`)'
    parts = re.split(quoted, source)
    for index in range(0, len(parts), 2):
        parts[index] = re.sub(r'\(([\w.]+)\)', r'\1', re.sub(r'\s+', '', parts[index]))
    return ''.join(parts).lstrip('|')


def braces(source):
    stack, pairs, depths = [], {}, []
    for pos, char in enumerate(source):
        depths.append(len(stack))
        if char == '{':
            stack.append(pos)
        elif char == '}' and stack:
            pairs[stack.pop()] = pos
    return pairs, depths


def include_guard(text):
    """Identify a paired empty outer guard, never an ordinary public macro."""
    lines = [line.strip() for line in text.splitlines() if line.strip()]
    if len(lines) < 3:
        return None
    opening = re.fullmatch(r'#\s*ifndef\s+([A-Za-z_]\w*)', lines[0])
    if not opening or not re.fullmatch(r'#\s*define\s+' + re.escape(opening[1]), lines[1]):
        return None
    if any(re.fullmatch(r'#\s*undef\s+' + re.escape(opening[1]), line) for line in lines):
        return None
    depth = 0
    for index, line in enumerate(lines):
        directive = re.match(r'#\s*(if|ifdef|ifndef|endif)\b', line)
        if not directive:
            continue
        depth += -1 if directive[1] == 'endif' else 1
        if depth < 0 or depth == 0 and index != len(lines) - 1:
            return None
    return opening[1] if depth == 0 and re.fullmatch(r'#\s*endif', lines[-1]) else None


def cstyle(source, language):
    if language == 'typescript':
        source = re.sub(r'(\breadonly\s+)"(\w+)"(?=\??\s*:)', lambda m: m[1] + ' ' + m[2] + ' ', source)
    text = masked(re.sub(r'@"([^"]+)"', lambda m: '  ' + m[1] + ' ', source) if language == 'zig' else source)
    if language == 'cpp':
        # Template parameter classes are not declarations or owners. Preserve offsets.
        text = re.sub(r'\btemplate\s*<[^<>]*(?:<[^<>]*>[^<>]*)*>',
                      lambda m: ''.join('\n' if c == '\n' else ' ' for c in m[0]), text)
    pairs, depths = braces(text)
    result, scopes, hidden = set(), [], []
    if language == 'cpp':
        for match in re.finditer(r'\bnamespace\s+([\w:]+)?\s*\{', text):
            start = text.rfind('{', match.start(), match.end())
            if start in pairs and (not match[1] or 'detail' in match[1].split('::')):
                hidden.append((start, pairs[start]))
    def cpp_public(pos):
        if any(start < pos < end for start, end in hidden):
            return False
        parent = max((s for s in scopes if s[0] < pos < s[1]), default=None)
        if not parent:
            return True
        access = [m for m in re.finditer(r'\b(public|private|protected)\s*:', text[parent[0]:pos])
                  if depths[parent[0] + m.start()] == depths[parent[0]] + 1]
        return access[-1][1] == 'public' if access else struct_scopes[parent[0]]
    struct_scopes = {}
    parentheses, nesting = [], 0
    for char in text:
        parentheses.append(nesting)
        nesting += (char == '(') - (char == ')')
    type_pattern = r'\b(?:class|struct|interface|enum(?:\s+class)?|record|union|trait|object|extension)\s+(\w+)'
    if language == 'dart':
        type_pattern = r'\b(?:class|enum|mixin)\s+(\w+)'
    if language == 'zig':
        type_pattern = r'\bpub\s+const\s+(\w+)\s*=\s*(?:struct|union|enum|opaque)\b'
    elif language == 'objc':
        type_pattern = r'@interface\s+(\w+)'
    for match in re.finditer(type_pattern, text):
        start = text.find('{', match.end())
        end_marker = text.find(';', match.end())
        if start < 0 or start not in pairs or 0 <= end_marker < start:
            continue
        prefix = text[max(text.rfind('\n', 0, match.start()), text.rfind('}', 0, match.start())) + 1:match.start()]
        if language in ('java', 'csharp', 'swift') and not match[0].startswith('extension') and not re.search(r'\bpublic\b', prefix):
            if language == 'java':
                hidden.append((start, pairs[start]))
            continue
        name = match[1]
        if language == 'cpp' and not cpp_public(match.start()):
            hidden.append((start, pairs[start]))
            continue
        if language == 'go' and not name[:1].isupper():
            continue
        end = pairs[start]
        if language in ('dart', 'kotlin', 'scala') and (name.startswith('_') or re.search(r'\b(private|protected)\b', prefix)):
            hidden.append((start, end))
            continue
        if language == 'c':
            alias = re.match(r'\s*(\w+)\s*;', text[end + 1:])
            if alias:
                name = alias[1]
        scopes.append((start, end, name))
        if language == 'cpp':
            struct_scopes[start] = match[0].startswith(('struct', 'union'))
            header = text[match.end():start]
            # Only public bases contribute the caller's inherited surface.
            for base in re.finditer(r'(?:^\s*:|,)\s*(?:(public|protected|private)\s+)?([\w:]+)(?:\s*<[^{}]*?>)?', header):
                if base[1] == 'public' or not base[1] and struct_scopes[start]:
                    result.add(Declaration(name, base[2].split('::')[-1], 'inherited'))
        if language == 'dart':
            header = text[match.end():start]
            bases = re.search(r'\bwith\s+(.+?)(?=\bimplements\b|$)', header, re.S)
            if bases:
                for base in bases[1].split(','):
                    target = re.match(r'\s*(\w+)', base)
                    if target:
                        result.add(Declaration(name, target[1], 'inherited'))
        if language in ('typescript', 'csharp'):
            header = text[match.end():start]
            bases = re.search(r'\bextends\s+(.+)|^\s*:\s*(.+)', header)
            if bases:
                for base in (bases[1] or bases[2]).split(','):
                    target = re.match(r'\s*([\w.]+)', base)
                    if target:
                        result.add(Declaration(name, target[1].split('.')[-1], 'inherited'))
        if language in ('java', 'kotlin', 'scala'):
            header = text[match.end():start]
            base = re.search(r'\bextends\s+(\w+)|^\s*(?:private\s+constructor\s*\([^)]*\)\s*)?:\s*(\w+)', header)
            if base:
                result.add(Declaration(name, next(group for group in base.groups() if group), 'inherited'))
        if language == 'java' and match[0].startswith('record'):
            header = text[match.end():start]
            opening = header.find('(')
            if opening >= 0:
                components, component_start, nesting = [], opening + 1, 0
                for pos in range(opening + 1, len(header)):
                    char = header[pos]
                    if char in '(<[':
                        nesting += 1
                    elif char == ')' and nesting == 0:
                        components.append(header[component_start:pos])
                        break
                    elif char in ')>]':
                        nesting -= 1
                    elif char == ',' and nesting == 0:
                        components.append(header[component_start:pos])
                        component_start = pos + 1
                for component in components:
                    accessor = re.search(r'(\w+)\s*$', component)
                    if accessor:
                        result.add(Declaration(name, accessor[1], 'call'))
        if not match[0].startswith('extension'):
            result.add(Declaration('', name, 'type'))
    if language == 'go':
        for match in re.finditer(r'\btype\s+([A-Z]\w*)\s+(?:struct|interface)\s*\{', text):
            start = text.rfind('{', match.start(), match.end())
            if start in pairs:
                scopes.append((start, pairs[start], match[1]))
                result.add(Declaration('', match[1], 'type'))
    if language == 'objc':
        for match in re.finditer(r'@interface\s+(\w+)[\s\S]*?@end', text):
            scopes.append((match.start(), match.end(), match[1]))
            result.add(Declaration('', match[1], 'type'))
    def owner(pos):
        if any(start < pos < end for start, end in hidden):
            return None
        matches = [s for s in scopes if s[0] < pos < s[1]]
        scope = max(matches, default=None)
        if scope and language != 'objc' and depths[pos] != depths[scope[0]] + 1:
            return None
        if not scope and depths[pos] > (1 if language in ('cpp', 'c') else 0):
            return None
        return scope[2] if scope else ''
    patterns = {
        'typescript': r'(?m)(?:^|[;{}\n])\s*(?:(?:export|declare|readonly|static|async)\s+)*(\w+)\s*(?:\??\s*:|(?:<[^\n]*?>)?\s*\()',
        'java': r'\bpublic\s+(?:(?:static|final|abstract|synchronized)\s+)*(?:[\w<>?,.\[\]]+\s+)(\w+)\s*(?:\(|[=;])',
        'csharp': r'\bpublic\s+(?:(?:static|readonly|override|abstract|required|sealed|async)\s+)*(?:[\w<>?,.\[\]]+\s+)(\w+)\s*(?:\(|\{|[=;])',
        'swift': r'\bpublic\s+(?:(?:static|class|mutating|convenience)\s+)*(?:func|var|let)\s+`?(\w+)',
        'kotlin': r'(?m)(?:^|[;{}\n])\s*(?:(?:public|override|open|final|suspend)\s+)*(?:fun|val|var)\s+`?(\w+)' ,
        'scala': r'(?m)(?:^|[;{}\n])\s*(?:(?:override|final|lazy)\s+)*(?:def|val|var)\s+`?(\w+)' ,
        'zig': r'\bpub\s+(?:fn|const|var)\s+(\w+)|(?:^|[{},\n])\s*(\w+)\s*:',
        'go': r'(?m:^\s*)([A-Z]\w*)\s+[^\n{}]+',
        'php': r'\bpublic\s+(?:static\s+)?function\s+(\w+)\s*\(|\bpublic\s+(?:readonly\s+)?[\w?\\]+\s+\$(\w+)',
        'dart': r'(?m)(?:^|[;{}\n])\s*(?:(?:static|final|late|const|external|factory)\s+)*[\w<>?,.]+\s+(?:get\s+)?(\w+)\s*(?:\(|=>|[;=])|\bget\s+(\w+)\s*(?:=>|\{)',
        'cpp': r'(?m)(?:^|(?<=[;{}\n]))[ \t]*[A-Za-z_][\w:<>,? \t]*?[*& \t]+(\w+)\s*(?:\(|[;=])',
        'c': r'(?m)(?:^|(?<=[;{}\n]))[ \t]*[A-Za-z_][\w \t]*?[* \t]+(\w+)\s*(?:\(|;|\[)',
        'objc': r'[-+]\s*\([^)]*\)\s*(\w+)|@property[^;\n]*?(\w+)\s*;',
    }
    for match in re.finditer(patterns.get(language, patterns['java']), text):
        name = next(group for group in match.groups() if group)
        if re.search(r'\b(class|struct|record|enum|interface)\s+' + re.escape(name) + r'\b', match[0]):
            continue
        pos = next(match.start(i) for i, group in enumerate(match.groups(), 1) if group)
        container = owner(pos)
        if language in ('typescript', 'zig') and parentheses[pos]:
            continue
        if language == 'cpp' and (parentheses[pos] or name == 'operator'):
            continue
        if language == 'go' and not container:
            continue
        if container is None or name.startswith('_') or language == 'dart' and (name == 'Function' or re.match(r'\s*(?:library|part)\b', match[0])):
            continue
        if language == 'cpp' and container:
            if not cpp_public(pos):
                continue
        tail = text[match.end():match.end() + 3]
        kind = 'call' if '(' in match[0] or language in ('swift', 'objc', 'php') and 'func' in match[0] or language in ('kotlin', 'scala', 'zig') and re.search(r'\b(fun|def|fn)\b', match[0]) else 'field'
        if language == 'zig' and re.search(r'\bpub\s+const\b', match[0]):
            kind = 'export'
        if language == 'zig' and kind == 'field' and not container:
            continue
        if kind == 'call' and (name == container or language == 'scala' and name == 'this'):
            continue
        result.add(Declaration(container, name, kind))
        if language == 'typescript' and kind == 'field':
            colon = text.find(':', pos, match.end())
            if colon >= 0:
                nesting, end = 0, colon + 1
                while end < len(text):
                    char = text[end]
                    if char == ';' and nesting == 0:
                        break
                    if char == '}' and nesting == 0:
                        break
                    nesting += (char in '([{<') - (char in ')]}>')
                    end += 1
                annotation = normalized_annotation(source[colon + 1:end])
                result.add(Declaration(container + '.' + name, annotation, 'annotation'))
    # Enum constants form the public contract even without a field declaration.
    for start, end, container in scopes:
        prefix = text[max(0, text.rfind(';', 0, start) + 1):start]
        if not re.search(r'\benum\b[^{}]*$', prefix):
            continue
        body = text[start + 1:end]
        if language == 'java':
            body = body.split(';', 1)[0]
        if language == 'php':
            names = re.findall(r'\bcase\s+(\w+)', body)
        else:
            names = re.findall(r'(?:^|,)\s*([\w/-]+)\s*(?=[=,(]|$)', body)
        result.update(Declaration(container, name, 'field') for name in names)
    if language == 'php':
        for match in re.finditer(r'\b(public\s+)?const\s+(\w+)\s*=', text):
            container = owner(match.start())
            if container == '' or container and match[1]:
                result.add(Declaration(container, match[2], 'field' if container else 'export'))
    if language in ('c', 'cpp'):
        guard = include_guard(text)
        for name in re.findall(r'(?m)^\s*#\s*define\s+([A-Za-z_]\w*)', text):
            if name == guard:
                guard = None
                continue
            result.add(Declaration('', name, 'export'))
    if language == 'swift':
        for match in re.finditer(r'\bcase\s+`?(\w+)`?', text):
            container = owner(match.start())
            if container:
                result.add(Declaration(container, match[1], 'field'))
    if language == 'typescript':
        for match in re.finditer(r'\bexport\s+(?:declare\s+)?function\s+(\w+)\s*\(', text):
            result.add(Declaration('', match[1], 'call'))
        for match in re.finditer(r'\bexport\s*\{([^}]+)\}', text):
            for binding in match[1].split(','):
                names = re.findall(r'\b\w+\b', binding)
                if names:
                    result.add(Declaration('', names[-1], 'export'))
        for match in re.finditer(r'\bexport\s+\*\s+as\s+(\w+)', text):
            result.add(Declaration('', match[1], 'export'))
    if language == 'go':
        for match in re.finditer(r'(?m)^func\s+(?:\(\w+\s+\*?(\w+)(?:\[[^\]]+\])?\)\s+)?([A-Z]\w*)\s*(?:\[[^\]]+\])?\s*\(', text):
            receiver = match[1] or ''
            result.add(Declaration(receiver, match[2], 'private_call' if receiver and not receiver[:1].isupper() else 'call'))
            if not receiver:
                opening, nesting, end = match.end() - 1, 1, match.end()
                while end < len(text) and nesting:
                    nesting += (text[end] == '(') - (text[end] == ')')
                    end += 1
                signature = text[end:text.find('{', end)]
                for returned in re.findall(r'\b[a-z]\w*\b', signature):
                    result.add(Declaration('', returned, 'returned_type'))
        for match in re.finditer(r'(?m)^const\s+([A-Z]\w*)|^[ \t]*([A-Z]\w*)[ \t]+\w*[ \t]*=', text):
            if depths[match.start()] == 0:
                result.add(Declaration('', match[1] or match[2], 'export'))
    if language == 'go':
        for match in re.finditer(r'\btype\s+(\w+)(?:\[[^]]+\])?\s+struct\s*\{', text):
            start = text.rfind('{', match.start(), match.end())
            if start not in pairs:
                continue
            for embedded in re.finditer(r'(?:^|[;\n])\s*\*?(\w+)(?:\[[^]\n]+\])?\s*(?=$|[;\n])', text[start + 1:pairs[start]]):
                result.add(Declaration(match[1], embedded[1], 'embedded'))
    if language == 'go':
        for match in re.finditer(r'\btype\s+(\w+)\s*=\s*(\w+)\b', text):
            result.add(Declaration(match[1], match[2], 'embedded'))
    if language in ('typescript', 'zig', 'swift', 'go'):
        patterns = {'typescript': r'\bexport\s+(?:declare\s+)?(?:class|interface|type|function|const|enum)\s+(\w+)',
                    'zig': r'\bpub\s+(?:const|fn)\s+(\w+)',
                    'swift': r'\bpublic\s+(?:typealias|enum|struct|class)\s+(\w+)',
                    'go': r'\btype\s+([A-Z]\w*)'}
        for match in re.finditer(patterns[language], text):
            if owner(match.start()) == '':
                result.add(Declaration('', match[1], 'export'))
    if language == 'typescript':
        bindings = {}
        for match in re.finditer(r'\bexport\s+(?:(?:declare|default|abstract)\s+)*(?:class|interface|type|function|const|enum)\s+(\w+)', text):
            if depths[match.start()] == 0:
                bindings.setdefault(match[1], set()).add(match[1])
        for match in re.finditer(r'\bexport\s+\*\s+as\s+(\w+)', text):
            bindings.setdefault(match[1], set()).add(match[1])
        for match in re.finditer(r'\bexport\s+(?:type\s+)?\{([^}]+)\}', text):
            if depths[match.start()] != 0:
                continue
            for binding in match[1].split(','):
                local = re.fullmatch(r'\s*(?:type\s+)?(\w+)(?:\s+as\s+(\w+))?\s*', binding)
                if local:
                    bindings.setdefault(local[1], set()).add(local[2] or local[1])
        for match in re.finditer(r'\b(?:declare\s+)?function\s+(\w+)\s*\(', text):
            if depths[match.start()] == 0:
                result.add(Declaration('', match[1], 'call'))
        public = set()
        for item in result:
            local = item.owner.split('.')[0] if item.owner else item.name
            for name in bindings.get(local, ()):
                mapped_owner = name + item.owner[len(local):] if item.owner else ''
                public.add(Declaration(mapped_owner, item.name if item.owner else name, item.kind))
            if not item.owner and item.kind == 'export' and item.name in {n for names in bindings.values() for n in names}:
                public.add(item)
        result = public
    return result


def go_owners(declared, embedded):
    owners, pending = set(), [embedded]
    while pending:
        target = pending.pop()
        for item in declared:
            if item.kind == 'embedded' and item.name == target and item.owner not in owners:
                owners.add(item.owner)
                pending.append(item.owner)
    return {owner for owner in owners if owner[:1].isupper()}


def public_go(declared):
    result = {d for d in declared if d.kind not in ('private_call', 'returned_type', 'embedded')}
    returned = {d.name for d in declared if d.kind == 'returned_type'}
    for item in declared:
        if item.kind != 'private_call':
            continue
        owners = go_owners(declared, item.owner)
        if item.owner in returned:
            owners.add(item.owner)
        result.update(Declaration(owner, item.name, 'call') for owner in owners)
    return result


def other(source, language):
    result = set()
    if language == 'r':
        result.update(Declaration('', name, 'export') for name in re.findall(r'(?m)^export\((\w+)\)', source))
        text = masked(source, r'#[^\n]*')
        for match in re.finditer(r'(?m)^([\w.]+)\s*(?:<-|=)\s*function\s*\(', text):
            result.add(Declaration('', match[1], 'call'))
            start, depth = match.end(), 1
            for pos in range(start, len(text)):
                char = text[pos]
                depth += (char in '([{') - (char in ')]}')
                if depth == 0 or char == ',' and depth == 1:
                    formal = re.match(r'\s*([\w.]+)\s*(?:=|$)', text[start:pos])
                    if formal and formal[1] != '...':
                        result.add(Declaration(match[1], formal[1], 'field'))
                    start = pos + 1
                if depth == 0:
                    break
    elif language == 'ruby':
        private = set()
        constants = {}
        text, owners, directives, visibility = ruby_masked(source)
        # Keep full origins when the inventory combines declarations with short owner names.
        origins, hidden_types = {}, set()
        for number, (raw, line) in enumerate(zip(source.splitlines(), text.splitlines()), 1):
            declarations = result
            result = set()
            lexical = owners.get(number, '')
            match = re.match(r'\s*(?:class|module)\s+(\w+)|\s*(\w+)\s*=\s*Struct\.new', line)
            if match and '<singleton>' not in lexical:
                name = match[1] or match[2]
                result.add(Declaration('', name, 'type'))
            container = lexical.split('::')[-1]
            visible_owner, mode, method_names = visibility.get(number, (lexical, '', ()))
            if mode == 'private':
                private.add(visible_owner)
            elif mode == 'public':
                private.discard(visible_owner)
            match = re.match(r'\s*(\w+)\s*=\s*%w\[([^]]*)\]', raw) if re.match(r'\s*\w+\s*=', line) else None
            if match:
                constants[(lexical, match[1])] = match[2].split()
            loop = re.match(r'\s*(\w+)\.each\s+do\s+\|(\w+)\|', line)
            literal_loop = re.match(r'\s*%w\[([^]]*)\]\.each\s+do\s+\|(\w+)\|', raw) if re.match(r'\s*\.each\s+do\b', line) else None
            if literal_loop:
                singleton_loop = (lexical, literal_loop[2], literal_loop[1].split())
            singleton = re.match(r'\s*define_singleton_method\("(\w+)#\{(\w+)\}"\)', raw) if re.match(r'\s*define_singleton_method\(', line) else None
            if singleton and 'singleton_loop' in locals() and singleton_loop[:2] == (lexical, singleton[2]):
                result.update(Declaration(container, singleton[1] + name, 'call') for name in singleton_loop[2])
            if loop:
                variable = loop[2]
                method_loop = (lexical, variable, constants.get((lexical, loop[1]), ()))
            elif re.match(r'\s*define_method\((\w+)\)', line):
                dynamic = re.match(r'\s*define_method\((\w+)\)', line)[1]
                if 'method_loop' in locals() and method_loop[:2] == (lexical, dynamic):
                    result.update(Declaration(container, name, 'call') for name in method_loop[2])
            match = re.match(r'\s*def\s+(?:self\.)?(\w+)\s*(?:\(([^)]*)\))?', line)
            if match and lexical not in private and not match[1].startswith('_'):
                result.add(Declaration(container, match[1], 'call'))
                for name in re.findall(r'\b(\w+)\s*:', match[2] or ''):
                    result.add(Declaration(container + '.' + match[1], name, 'field'))
            if lexical not in private and re.match(r'\s*attr_reader\b', line):
                for name in re.findall(r':(\w+)', line):
                    result.add(Declaration(container, name, 'field'))
            for name in method_names:
                declaration = Declaration(container, name, 'call')
                origins.get(declaration, set()).discard(visible_owner)
                if not origins.get(declaration):
                    declarations.discard(declaration)
            match = re.match(r'\s*define_method\([\'\"](\w+)[\'\"]\)', raw) if re.match(r'\s*define_method\(', line) else None
            if match:
                result.add(Declaration(container, match[1], 'field'))
            for declaration in result:
                path = '::'.join(filter(None, (lexical, declaration.name))) if declaration.kind == 'type' else lexical
                origins.setdefault(declaration, set()).add(path)
            result.update(declarations)
        for lexical, receiver, name in directives:
            if receiver in (None, 'self'):
                target = lexical
            elif receiver.startswith('::') or receiver.split('::')[0] == 'Object':
                target = receiver.removeprefix('::').removeprefix('Object::')
                target = '' if target == 'Object' else target
            else:
                parts = lexical.split('::')
                candidates = ['::'.join(parts[:end] + [receiver]) for end in range(len(parts), 0, -1)] + [receiver]
                target = next((candidate for candidate in candidates if candidate in owners.values()), receiver)
            hidden_types.add('::'.join(filter(None, (target, name))))
        result = {d for d in result if any(not any(path == hidden or path.startswith(hidden + '::')
                                                  for hidden in hidden_types) for path in origins[d])}
    elif language == 'ada':
        text = re.sub(r'--[^\n]*', '', source)
        if re.search(r'(?im)^\s*private\s+package\b', text):
            return result
        text = re.split(r'(?im)^\s*private\s*$', text)[0]
        def normalized(value):
            return re.sub(r'\s+', ' ', value.strip()).lower()
        for match in re.finditer(r'(?i)\b(?:type|subtype)\s+(\w+)\b([^;]*);', text):
            result.add(Declaration('', match[1], 'type'))
            # Opaque public types retain their declaration; private completions stay hidden.
            result.add(Declaration(match[1], normalized(match[2]), 'annotation'))
        for match in re.finditer(r'(?is)\btype\s+(\w+)\b[^;]*?\bis\s+record\b(.*?)end\s+record', text):
            for field in re.finditer(r'\b(\w+)\s*:\s*([^;]+);', match[2]):
                result.add(Declaration(match[1], field[1], 'field'))
                result.add(Declaration(match[1] + '.' + field[1], normalized(field[2]), 'annotation'))
        for match in re.finditer(r'(?im)^\s*(\w+)\s*:\s*constant\b([^;]*);', text):
            result.add(Declaration('', match[1], 'export'))
            result.add(Declaration(match[1], normalized(match[2]), 'annotation'))
        for match in re.finditer(r'(?is)\b(function|procedure)\s+(\w+)\s*(\([^;]*?(?:;[^;]*?)*?\))?\s*(return\s+[^;]+)?;', text):
            result.add(Declaration('', match[2], 'call'))
            result.add(Declaration(match[2], normalized((match[3] or '') + ' ' + (match[4] or '')), 'annotation'))
            builder = re.match(r'(?i)With_(\w+)$', match[2])
            item = re.match(r'(?i)\(\s*Item\s*:\s*(?:[\w]+\.)*(\w+)\s*;', match[3] or '')
            if builder and item:
                result.add(Declaration(item[1], builder[1], 'field'))
    elif language == 'cobol':
        scopes = []
        for line in source.splitlines():
            match = re.match(r'\s*(\d\d)\s+([\w-]+)', line)
            if match:
                level, name = int(match[1]), match[2]
                scopes = [s for s in scopes if s[0] < level]
                result.add(Declaration(scopes[-1][1] if scopes else '', name, 'field'))
                scopes.append((level, name))
    return result


def php(source):
    """Explicit internal namespaces hide implementation declarations only."""
    text = masked(source)
    pairs, depths = braces(text)
    namespaces = list(re.finditer(r'\bnamespace\s*(?P<name>[\w\\]+)?\s*(?P<edge>[;{])', text))
    erased = list(source)
    def erase(start, end):
        erased[start:end] = ['\n' if char == '\n' else ' ' for char in source[start:end]]
    for index, declaration in enumerate(namespaces):
        opening = declaration.end() - 1
        bracketed = declaration['edge'] == '{'
        end = pairs.get(opening, len(source) - 1) + 1 if bracketed else namespaces[index + 1].start() if index + 1 < len(namespaces) else len(source)
        if 'Internal' in (declaration['name'] or '').split('\\'):
            erase(declaration.start(), end)
        elif bracketed:
            erase(declaration.start(), declaration.end())
            erase(end - 1, end)
    visible = ''.join(erased)
    result = cstyle(visible, 'php')
    text = masked(visible)
    pairs, depths = braces(text)
    for declaration in re.finditer(r'\b(?:class|trait)\s+(\w+)([^{};]*)\{', text):
        owner = declaration[1]
        start = declaration.end() - 1
        if start not in pairs: continue
        end = pairs[start]
        base = re.search(r'\bextends\s+([\w\\]+)', declaration[2])
        if base: result.add(Declaration(owner, base[1].split('\\')[-1], 'inherited'))
        for trait in re.finditer(r'\buse\s+([\w\\,\s]+)\s*;', text[start + 1:end]):
            if depths[start + 1 + trait.start()] != depths[start] + 1: continue
            for name in trait[1].split(','):
                result.add(Declaration(owner, name.strip().split('\\')[-1], 'inherited'))
        for constructor in re.finditer(r'\bpublic\s+function\s+__construct\s*\(', text[start + 1:end]):
            opening = start + 1 + constructor.end() - 1
            closing, depth = opening + 1, 1
            while closing < end and depth:
                depth += (text[closing] == '(') - (text[closing] == ')')
                closing += 1
            signature = text[opening + 1:closing - 1]
            for promoted in re.finditer(r'\bpublic\s+(?:readonly\s+)?[\w?\\|]+\s+\$(\w+)', signature):
                result.add(Declaration(owner, promoted[1], 'field'))
    return result


def declarations(source, language):
    if language == 'php':
        return php(source)
    if language == 'python':
        return python(source)
    if language == 'rbs':
        return rbs(source)
    if language in ('r', 'ruby', 'ada', 'cobol'):
        return other(source, language)
    return cstyle(source, language)


def public_inherited(declared):
    result = {d for d in declared if d.kind != 'inherited'}
    while True:
        promoted = {Declaration(link.owner, member.name, member.kind)
                    for link in declared if link.kind == 'inherited'
                    for member in result if member.owner == link.name and member.kind in ('call', 'field')}
        if promoted <= result:
            return result
        result.update(promoted)


def python_keywords(declared):
    """Resolve keyword dictionaries and inherited fields from their typed source."""
    expanded = set(declared)
    while True:
        inherited = {Declaration(link.owner, member.name, 'field')
                     for link in declared if link.kind == 'inherited'
                     for member in expanded if member.owner == link.name and member.kind == 'field'}
        if inherited <= expanded:
            break
        expanded.update(inherited)
    for link in declared:
        if link.kind == 'unpacked':
            expanded.update(Declaration(link.owner, member.name, 'field')
                            for member in expanded.copy() if member.owner == link.name and member.kind == 'field')
    return {d for d in expanded if d.kind not in ('hidden_type', 'unpacked') and not d.owner.startswith('_')}


def rbs(source):
    """Read RBS member signatures, including nested typed keyword parameters."""
    result, scopes, private = set(), [], set()
    owning_method = None
    for line in source.splitlines():
        indent = len(line) - len(line.lstrip())
        if line.strip() and not line.lstrip().startswith('#'):
            scopes = [s for s in scopes if s[0] < indent]
        match = re.match(r'\s*(?:class|module)\s+(\w+)', line)
        if match:
            scopes.append((indent, match[1]))
            result.add(Declaration('', match[1], 'type'))
        owner = scopes[-1][1] if scopes else ''
        if line.strip() == 'private':
            private.add(owner)
        elif line.strip() == 'public':
            private.discard(owner)
        if owner in private:
            continue
        method = re.match(r'\s*def\s+(?:self\.)?(\w+)[!?]?:\s*\(', line)
        continuation = re.match(r'\s*\|\s*\(', line)
        if method:
            owning_method = (owner, method[1])
        elif continuation and owning_method and owning_method[0] == owner:
            method = continuation
        elif line.strip() and not line.lstrip().startswith('#'):
            owning_method = None
        if method:
            name = owning_method[1]
            result.add(Declaration(owner, name, 'call'))
            if '->' in line:
                returned = re.sub(r'\s+', '', line.rsplit('->', 1)[1])
                result.add(Declaration(owner + '.' + name, returned, 'return'))
            start, depth, pieces = method.end(), 0, []
            for pos in range(start, len(line)):
                char = line[pos]
                if char == ')' and depth == 0:
                    pieces.append(line[start:pos]); break
                if char == ',' and depth == 0:
                    pieces.append(line[start:pos]); start = pos + 1
                else:
                    depth += (char in '([{') - (char in ')]}')
            for piece in pieces:
                keyword = re.match(r'\s*\??(\w+):\s*(.+)', piece)
                if keyword:
                    container = owner + '.' + name
                    result.add(Declaration(container, keyword[1], 'field'))
                    result.add(Declaration(container + '.' + keyword[1], keyword[2].strip(), 'annotation'))
        field = re.match(r'\s*attr_reader\s+(\w+):', line)
        if field:
            result.add(Declaration(owner, field[1], 'field'))
    return result
