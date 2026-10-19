"""Deterministic readable generated source; retain native tokens unchanged."""
import re
import subprocess
# Only whitespace separates native tokens. Keep literals, comments, and preprocessor
# directives intact; wrap at delimiters where continuation is explicit.
def native(text):
    tokens = re.findall(r'^[ \t]*\#[^\n]*|//[^\n]*|/\*[\s\S]*?\*/|"(?:\\.|[^"\\])*"|\'(?:\\.|[^\'\\])*\'|[^"\'/#]+|.', text, re.M)
    lines=[]; current=''; depth=0; parens=0
    def flush():
        nonlocal current
        if current.strip(): lines.append('    '*max(depth,0)+current.strip())
        current=''
    for token in tokens:
        if token.startswith(('"',"'",'//','/*')) or token.lstrip().startswith('#'):
            current+=token
            if token.startswith('//') or token.lstrip().startswith('#'):flush()
            continue
        for c in token:
            if c=='\n':flush()
            elif c=='{':current+=c;flush();depth+=1
            elif c=='}':flush();depth-=1;current+=c
            elif c==';':current+=c;flush()
            elif c==',':
                current+=c
                if len(current)>90:flush()
            else:current+=c
    flush();return '\n'.join(lines)+'\n'


def format_source(path, text, env=None):
    suffix = str(path).rsplit('.', 1)[-1]
    command = None
    if suffix in ('py', 'pyi'):
        command = ['ruff', 'format', '--isolated', '--stdin-filename', 'generated.' + suffix, '-']
    elif suffix in ('js', 'ts'):
        command = ['prettier', '--no-config', '--no-editorconfig', '--parser', 'typescript' if suffix == 'ts' else 'babel']
    elif suffix == 'swift':
        command = ['swift', 'format', 'format', '--configuration', '{"lineLength":100}']
    elif suffix == 'rs':
        command = ['rustfmt', '--edition', '2024', '--emit', 'stdout']
    elif suffix in ('cs', 'java', 'h', 'hpp', 'c', 'm'):
        return native(text)
    if command:
        return subprocess.run(command, input=text, text=True, stdout=subprocess.PIPE,
                              check=True, env=env).stdout
    return text
