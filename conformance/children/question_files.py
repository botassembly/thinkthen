"""Apply the documented Local category when authored input uses a file selector."""

def file_question_expectation(fixture):
    if fixture.get('raw') is not None and fixture['expect'].get('error') == 'usage':
        return {**fixture, 'expect': {**fixture['expect'], 'error': 'local'}}
    return fixture
