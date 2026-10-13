"""Select the declarations each native adapter uses, including carrier dependencies."""
import re

COMMON = (
    'thinkthen_engine_free', 'thinkthen_engine_new_with',
    'thinkthen_engine_usage_persistence_v1', 'thinkthen_engine_finish_usage_status_v1',
    'thinkthen_error_code', 'thinkthen_error_message',
    'thinkthen_session_cancel', 'thinkthen_session_error_message',
    'thinkthen_session_finish', 'thinkthen_session_free',
    'thinkthen_session_result_free', 'thinkthen_session_try_push', 'thinkthen_session_push',
    'thinkthen_session_try_read', 'thinkthen_session_wait', 'thinkthen_session_next_packet',
)
USE = {
    'php': (*COMMON, 'thinkthen_session_new_named_with_surface', 'thinkthen_session_result_json'),
    'swift': (*COMMON, 'thinkthen_error_retryable', 'thinkthen_free_string',
              'thinkthen_question_free', 'thinkthen_question_parse', 'thinkthen_request_plan_json',
              'thinkthen_session_new_with_surface', 'thinkthen_session_new_named_with_surface',
              'thinkthen_session_result_json'),
    'ada': (*COMMON, 'thinkthen_engine_new', 'thinkthen_error_retryable',
            'thinkthen_session_new_with_surface', 'thinkthen_session_new_named_with_surface',
            'thinkthen_session_result_view', 'thinkthen_session_new_typed_with_surface',
            'thinkthen_session_try_push_typed', 'thinkthen_session_push_typed', 'thinkthen_session_finish_typed',
            'THINKTHEN_COBOL_OVERFLOW',
            'THINKTHEN_COMPLETE_FAILURE_KIND_BACKEND_V1',
            'THINKTHEN_COMPLETE_FAILURE_KIND_CANCELLED_V1',
            'THINKTHEN_COMPLETE_FAILURE_KIND_DEADLINE_V1',
            'THINKTHEN_COMPLETE_FAILURE_KIND_LOCAL_V1',
            'THINKTHEN_COMPLETE_FAILURE_KIND_USAGE_V1',
            'THINKTHEN_COMPLETE_SESSION_PACKET_TERMINAL_V1'),
}


def selected_functions(functions, language):
    names = {name for name in USE[language] if name.startswith('thinkthen_')}
    missing = names - functions.keys()
    if missing:
        raise ValueError('missing native declarations: ' + ', '.join(sorted(missing)))
    return {name: value for name, value in functions.items() if name in names}


def selected_header(header, language):
    """Keep complete original declarations in their dependency-preserving order."""
    source = re.sub(r'/\*.*?\*/|//[^\n]*', '', header, flags=re.S)
    macros = dict(re.findall(r'^#define\s+(THINKTHEN_\w+)\s+([^\n]+)', source, re.M))
    source = re.sub(r'^\s*#.*$', '', source, flags=re.M)
    source = source.replace('extern "C" {', '')
    # The last brace closes C++ linkage, rather than a declaration.
    source = re.sub(r'\}\s*$', '', source)
    declarations = []
    for text in source.split(';'):
        # Struct members contain semicolons, so reconstruct balanced declarations.
        if declarations and declarations[-1].count('{') > declarations[-1].count('}'):
            declarations[-1] += text + ';'
        elif text.strip():
            declarations.append(text.strip() + ';')
    definitions = {}
    for text in declarations:
        if text.startswith('typedef'):
            match = re.search(r'\b(thinkthen_\w+)\s*;$', text)
        else:
            match = re.search(r'\b(thinkthen_\w+)\s*\(', text)
        if match:
            definitions[match[1]] = text
    wanted = set(USE[language])
    wanted.update(name for name in macros if name.startswith(('THINKTHEN_E', 'THINKTHEN_SESSION_', 'THINKTHEN_COMPLETE_USAGE_PERSISTENCE_')))
    pending = list(wanted)
    while pending:
        name = pending.pop()
        text = definitions.get(name, macros.get(name))
        if text is None:
            raise ValueError('missing native declaration: ' + name)
        dependencies = set(re.findall(r'\b(?:thinkthen_\w+|THINKTHEN_\w+)\b', text))
        if name.startswith('thinkthen_') and name.endswith('_v1') and text.startswith('typedef'):
            prefix = name.removesuffix('_v1').upper() + '_'
            dependencies.update(constant for constant in macros if constant.startswith(prefix))
        for dependency in dependencies:
            if dependency not in wanted:
                wanted.add(dependency)
                pending.append(dependency)
    lines = ['/* Generated from selected canonical C declarations; do not edit. */',
             '#include <stdbool.h>', '#include <stddef.h>', '#include <stdint.h>']
    lines.extend('#define ' + name + ' ' + value for name, value in macros.items() if name in wanted)
    lines.extend(text for name, text in definitions.items() if name in wanted)
    return '\n\n'.join(lines) + '\n'
