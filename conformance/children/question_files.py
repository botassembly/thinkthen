"""Apply the documented Local category when authored input uses a file selector."""

def file_question_expectation(fixture):
    if fixture.get('raw') is not None and fixture['expect'].get('error') == 'usage':
        return {**fixture, 'expect': {**fixture['expect'], 'error': 'local'}}
    return fixture


def question_selector(value, home, definition, *, inline_raw):
    """Select authored bytes unchanged; generated records use a public file."""
    if value.get('loader'):
        kind = {'file': 'file', 'load': 'file', 'named': 'name', 'load_named': 'name',
                'reference': 'reference', 'load_reference': 'reference'}[value['loader']]
        return {'kind': kind, {'file': 'path', 'name': 'name', 'reference': 'reference'}[kind]: value['reference']}
    if value.get('question_form') == 'file':
        return {'kind': 'file', 'path': str(home / 'fixture-question.json')}
    if value.get('raw') is not None:
        if inline_raw:
            return {'kind': 'definition', 'raw': value['raw']}
        path = home / 'raw-question.json'
        path.write_text(value['raw'], encoding='utf-8')
        return {'kind': 'file', 'path': str(path)}
    return {'kind': 'definition', 'value': definition}
