"""Run the literal DuckDB/shared SQL examples through the installed extension."""
import json
import os
from pathlib import Path
import re
import sys

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / 'scripts'))
from guide_examples import IDEAL, blocks, scenario
from typed_api import Host
from migration_cases import command

MESSAGES = ['Please refund my order.', 'Where is my parcel?',
            'I was charged twice for one invoice.',
            'I was charged twice and my parcel is late again.',
            'Morgan works for Example Group.\nPlease refund my order.']
SCENARIOS = ('yes', 'no', 'unsure', 'failed', 'low-score')


def normalized(sql):
    return re.sub(r'\s+', ' ', re.sub(r'^--[^\n]*\n', '', sql, flags=re.M)).strip()


def statements(sql):
    # Split SQL, not lines: JSON strings and the multiline record remain literal.
    current, quoted = [], False
    for char in sql:
        current.append(char)
        if char == "'":
            quoted = not quoted
        if char == ';' and not quoted:
            yield ''.join(current)
            current = []
    assert not ''.join(current).strip(), sql


def equal(label, actual, expected):
    assert actual == expected, (label, actual, expected)


def status(name, band=False):
    return 'failed' if name == 'failed' else 'unsure' if name == 'unsure' and band else 'answered'


def probability(name):
    return None if name == 'failed' else {'yes': '0.95', 'no': '0.05', 'unsure': '0.5'}[name]


def answer(name, band=False):
    return None if status(name, band) != 'answered' else str(name != 'no').lower()


def route(name, band=True):
    return ('try again later' if name == 'failed' else 'a person reads it' if name == 'unsure' and band
            else 'inbox' if name == 'no' else 'refund queue')


def original(rows=3):
    return [{'msg_id': str(i + 1), 'body': body} for i, body in enumerate(MESSAGES[:rows])]


def child_states(name):
    states = dict.fromkeys(('refund', 'topic', 'urgency'), 'answered')
    if name == 'failed':
        return dict.fromkeys(states, 'failed')
    if name == 'children-failed':
        return dict.fromkeys(states, 'failed')
    if name == 'unsure':
        states['refund'] = states['topic'] = 'unsure'
    if '-' in name:
        field, state = name.split('-')
        states[field] = 'failed' if state == 'failed' else 'unsure'
    return states


def forms(name):
    children = child_states(name)
    top = 'answered'  # Valid form evidence; even an outage fails its entries.
    topic = None if children['topic'] != 'answered' else 'shipping' if name == 'no' else 'billing'
    urgency = None if children['urgency'] != 'answered' else '0.5' if name == 'unsure' else '1.0'
    branch = ('try again later' if top == 'failed' or 'failed' in children.values() else
              'a person reads it' if 'unsure' in children.values() else
              'inbox' if name == 'no' else 'refund queue')
    return [{**row, 'status': top, **{field + '_status': state for field, state in children.items()},
             'topic': topic, 'urgency': urgency, 'where_it_goes': branch} for row in original()]


def expected(index, part, name):
    """Expected displayed cells for each literal example, independent of execution."""
    if index in (0, 32):
        return []
    if index == 1:
        return [] if part == 0 else [{'Count': '5'}]
    if index in (5, 12, 18, 20, 23):
        return [{'Count': '1'}]
    if index in (14, 27):
        return [{'Count': '3'}]
    if index == 2:
        return [{**row, 'is_refund': answer(name, True), 'status': status(name, True)} for row in original(5)]
    if index == 3:
        return [{'where_it_goes': 'refund queue'}] if answer(name) == 'true' else []
    if index == 4:
        return [{'where_it_goes': None if name == 'failed' else route(name, False)}]
    if index in (6, 8):
        value = 'a person reads it' if index == 6 and name == 'failed' else route(name)
        return [{'where_it_goes': value}]
    if index == 7:
        return [{'where_it_goes': route(name), 'probability': probability(name),
                 'error_code': 'backend.status' if name == 'failed' else None}]
    if index == 9:
        return [{'note': 'could not judge it: failed: backend.status: the backend answered status 503: {"error":"unavailable"}'}] if name == 'failed' else []
    if index == 10:
        return [{'is_refund': answer(name)}]
    if index == 11:
        return [{'probability': probability(name)}]
    if index == 13:
        return [{'team_for': {'yes': 'billing team', 'no': 'warehouse', 'unsure': 'a person reads it', 'failed': None, 'other': 'front desk'}[name]}]
    if index == 15:
        return [{**row, 'where_it_goes': route(name)} for row in original()]
    if index == 16:
        return original() if name != 'no' else []
    if index == 17:
        if part == 0:
            return original() if name == 'yes' else []
        return [{**row, 'error_code': 'backend.status'} for row in original()] if name == 'failed' else []
    if index == 19:
        if part == 0:
            return [{'team_for': 'billing team'}] if name in ('yes', 'unsure') else []
        labels = ['billing', 'shipping'] if name in ('yes', 'unsure') else [None]
        return [{'position': '1', 'id': None, 'status': status(name), 'topic': label} for label in labels]
    if index == 21:
        score = None if name == 'failed' else '0.5' if name == 'unsure' else '0.0' if name == 'low-score' else '1.0'
        return [{'urgency': score, 'note': 'answer today' if score == '1.0' else None}]
    if index == 22:
        return [{'answer': None if name == 'failed' else str(i + 1), 'body': body} for i, body in enumerate(MESSAGES[:3])]
    if index == 24:
        return [{'line': {'yes': 'Please refund my order.', 'no': 'no line answers',
                          'unsure': 'a person reads it', 'failed': None}[name]}]
    if index == 25:
        if name == 'relations':
            return [{'position': '1', 'id': None, 'status': 'answered', 'kind': kind, 'text': text}
                    for kind, text in [('person', 'Morgan'), ('organization', 'Example Group')]]
        return [{'position': '1', 'id': None, 'status': status(name), 'kind': None, 'text': None}]
    if index == 26:
        if name == 'relations':
            return [{'position': '1', 'id': None, 'status': 'answered', 'source': 'Morgan',
                     'relation': 'works_for', 'target': 'Example Group'}]
        return [{'position': '1', 'id': None, 'status': status(name), 'source': None,
                 'relation': None, 'target': None}]
    if index == 28:
        return forms(name)
    if index == 29:
        return ([{'is_refund': answer(name), 'status': status(name)}] if part == 0 else
                [{'topic': {'yes': 'billing', 'no': 'shipping', 'unsure': None, 'failed': None}[name],
                  'status': status(name, True)}])
    if index == 30:
        # Shared SQL displays the JSON id; DuckDB's guide explicitly unwraps it.
        return [{'id': 'a1', 'is_refund': answer(name), 'status': status(name)}]
    if index == 31:
        return ([{'status': status(name, True), 'records': '3'}] if part == 0 else
                original() if name == 'unsure' else [])
    raise AssertionError(('unaccounted guide example', index))


def guide_examples(package, guide, name, negative=None):
    authored = blocks(IDEAL / (guide + '.md'))
    reference = {normalized(body): i for i, (language, body, _) in enumerate(blocks(IDEAL / 'duckdb.md'))}
    observer = []
    choice_case = {'replies': [{'distribution': {'billing': 0, 'shipping': 0, 'other': 1}}]} if name == 'other' else None
    with scenario(name, case=choice_case, observer=observer) as (_, env):
        env['THINKTHEN_TEST_LOOPBACK_URL'] = env['THINKTHEN_BASE_URL']
        with Host(package, env) as host:
            host.ok('SET thinkthen_max_retries = 0')
            covered, failures = [], []
            for number, (language, body, line) in enumerate(authored):
                assert language == 'sql', (guide, line, language)
                key = normalized(body)
                # The shared guide labels precisely two alternative host spellings.
                if guide == 'sql' and number in (2, 3):
                    assert ('thinkthen.decide' in key if number == 3 else "thinkthen_decide('SELECT" in key)
                    continue
                shared_id = "SELECT id, answer AS is_refund, status" in key
                lookup = key.replace('SELECT id, answer AS is_refund, status', "SELECT id->>'$' AS id, answer AS is_refund, status")
                assert lookup in reference, ('new guide block needs output proof', guide, line, key)
                index = reference[lookup]
                if name == 'other' and index not in (0, 1, 12, 13):
                    continue
                if name == 'low-score' and index not in (0, 1, 20, 21):
                    continue
                if name == 'relations' and index not in (0, 1, 25, 26):
                    continue
                if '-' in name and name != 'low-score' and index not in (0, 1, 27, 28):
                    continue
                # These structural examples need populated, empty, failed cases,
                # rather than arbitrary boundary-tag distributions.
                if index in (25, 26) and name not in ('relations', 'unsure', 'failed'):
                    continue
                if index == 0:
                    body = body.replace('/absolute/path/to/thinkthen.duckdb_extension', str(package / 'thinkthen.duckdb_extension'))
                for part, sql in enumerate(statements(body)):
                    label = f'{guide}.md:{line} statement {part + 1} [{name}]'
                    if name == 'failed' and index in (10, 16):
                        error = host.failure(sql)
                        assert 'backend.status:' in error and ('1 of 1' if index == 10 else '3 of 3') + ' records failed' in error, (label, error)
                    else:
                        rows = host.ok(sql)
                        wanted = expected(index, part, name)
                        if shared_id and wanted:
                            wanted[0]['id'] = '"a1"'
                        if negative == 'output' and index == 3:
                            wanted = [{'where_it_goes': 'wrong queue'}]
                        if negative == 'branch' and index == 6:
                            wanted = [{'where_it_goes': 'wrong branch'}]
                        # SQL joins promise these cells, not their order. The
                        # authored expansion queries deliberately have no ORDER BY.
                        if index in (19, 25, 26):
                            rows = sorted(rows, key=lambda row: json.dumps(row, sort_keys=True))
                            wanted = sorted(wanted, key=lambda row: json.dumps(row, sort_keys=True))
                        try:
                            equal(label, rows, wanted)
                        except AssertionError as error:
                            failures.append(str(error))
                    covered.append((line, part))
            assert covered, (guide, name)
            assert not failures, '\n'.join(failures)
            print(f'PASS {guide} [{name}]: {len(covered)} literal statements, output and branches', flush=True)


def run(package, loopback, negative=None, guides=('duckdb', 'sql')):
    os.environ['THINKTHEN_LOOPBACK'] = str(loopback)
    failures = []
    for guide in guides:
        for name in (*SCENARIOS, 'other', 'relations', 'refund-failed', 'topic-failed', 'urgency-failed', 'refund-unsure', 'topic-unsure'):
            try:
                guide_examples(package, guide, name, negative)
            except AssertionError as error:
                print(str(error), file=sys.stderr, flush=True)
                failures.append(str(error))
    if 'duckdb' in guides:
        refusals_and_forms(package, negative)
        limit_probe(package)
    assert not failures, '\n'.join(failures)
    if negative is None and 'duckdb' in guides:
        for control in ('output', 'branch', 'refusal'):
            try:
                if control == 'refusal':
                    refusals_and_forms(package, control)
                else:
                    guide_examples(package, 'duckdb', 'yes', control)
            except AssertionError:
                print('PASS DuckDB runner rejects wrong ' + control, flush=True)
            else:
                raise AssertionError('runner accepted negative control: ' + control)


def refusals_and_forms(package, negative=None):
    from cases import invocation
    from interface_cases import COMMON
    # All refusals run against an actual answering service; request counts prove
    # admission happened before any remote request, rather than merely failing.
    with scenario('yes', observer=(servers := [])) as (_, env):
        env['THINKTHEN_TEST_LOOPBACK_URL'] = env['THINKTHEN_BASE_URL']
        with Host(package, env) as host:
            statements_and_codes = [
                ("SET GLOBAL thinkthen_batch = 1", 'usage.option'),
                ("SELECT * FROM thinkthen_filter((SELECT 'text' AS record), 'Ready?', low := 0.2, high := 0.8)", 'usage.threshold'),
                ("SELECT * FROM thinkthen_decide((SELECT 42 AS record), 'Ready?')", 'usage.sql-selection'),
                ("SELECT * FROM thinkthen_decide((SELECT 'text' AS body), 'Ready?')", 'usage.sql-selection'),
                ("SELECT * FROM thinkthen_decide((SELECT 'text' AS record), 'Ready?', record := 42)", 'usage.option'),
                ("SELECT * FROM thinkthen_choose((SELECT 'text' AS record), 'Which?', choices := ['billing'], descriptions := MAP {'unknown':'No such choice'})", 'usage.option'),
                ("SELECT * FROM thinkthen_annotate((SELECT 'text' AS record), questions := '{\"bad\":{}}'::JSON)", 'usage.question'),
                ("SELECT * FROM thinkthen_decide((SELECT 'text' AS record), 'Ready?', raise_if_failed := 'yes')", 'usage.option'),
            ]
            for column in COMMON:
                statements_and_codes.append((f"SELECT * FROM thinkthen_decide((SELECT 'text' AS record, 1 AS {column}), 'Ready?')", 'usage.sql-selection'))
            for name in COMMON:
                q = {'function': 'annotate', 'questions': {name: {'function': 'decide', 'question': 'Ready?'}}}
                statements_and_codes.append(('SELECT * FROM ' + invocation({'question': q, 'records': ['text']}), 'usage.column'))
            for function in ('filter', 'rank', 'annotate'):
                child = ({'function': 'annotate', 'questions': {}} if function == 'annotate' else
                         {'function': function, 'question': 'Ready?', **({'levels': ['low','high']} if function == 'rank' else {})})
                q = {'function': 'annotate', 'questions': {'bad': child}}
                statements_and_codes.append(('SELECT * FROM ' + invocation({'question': q, 'records': ['text']}), 'usage.question'))
            for sql, code in statements_and_codes:
                error = host.failure('SELECT 1' if negative == 'refusal' else sql)
                assert code + ':' in error, (sql, error)
                if 'low := 0.2' in sql:
                    assert 'thinkthen_decide' in error, error
            for sql in (
                "SELECT * FROM thinkthen_filter((SELECT 'text' AS record), 'Ready?', failures := 'row')",
                "SELECT * FROM thinkthen_annotate((SELECT 'text' AS record), toolbox := '{}')",
                "SELECT * FROM thinkthen_annotate((SELECT 'text' AS record), questions := (SELECT '{}'))",
                "SET thinkthen_api_key = 'synthetic'", "SET thinkthen_url = 'http://127.0.0.1'", "SET thinkthen_max_requests = 1",
            ):
                host.failure(sql)
            equal('all usage refusals send no requests', command(servers[0], 'count'), '0')
            for sql in (
                "SELECT * FROM thinkthen_recognize((SELECT 'text' AS record), kinds := ['person']) WHERE answer",
                "SELECT * FROM thinkthen_choose((SELECT 'text' AS record), 'Which?', choices := ['billing']) WHERE answer",
            ):
                host.failure(sql)
            # SQL nests entries, so these two names really do not clash.
            pair = {'function': 'annotate', 'questions': {name: {'function': 'decide', 'question': 'Ready?'} for name in ('refund', 'refund_status')}}
            rows = host.ok('SELECT answer.refund,answer.refund_status FROM ' + invocation({'question': pair, 'records': ['text']}))
            equal('nested names do not invent flat columns', rows, [{'refund': 'true', 'refund_status': 'true'}])
            empty = host.ok("SELECT * FROM thinkthen_decide((SELECT 'text' AS record WHERE false), 'Ready?')")
            equal('empty input', empty, [])
            requests = command(servers[0], 'count')
            blanks = host.ok("SELECT status,error_code FROM thinkthen_decide((SELECT record FROM (VALUES (''),('  '),(NULL)) inputs(record)), 'Ready?')")
            equal('blank and missing rows', blanks, [{'status': 'failed', 'error_code': code} for code in ('record.empty', 'record.empty', 'record.missing')])
            equal('blank and empty inputs send no requests', command(servers[0], 'count'), requests)
            for function, options in (
                ('decide', "'Ready?'"), ('choose', "'Which?', choices := ['one','two']"),
                ('tag', "'Which?', labels := ['one','two']"), ('score', "'How?', levels := ['low','high']"),
                ('rank', "'How?', levels := ['low','high'], top := 1"), ('find', "'Where?'"),
                ('recognize', "kinds := ['person']"),
                ('relate', "rules := [{'source':'person','target':'organization','relation':'works_for'}]"),
                ('annotate', "questions := '{\"ready\":{\"function\":\"decide\",\"question\":\"Ready?\"}}'::JSON"),
            ):
                call = f"thinkthen_{function}((SELECT NULL::VARCHAR AS record), " + options
                equal(function + ' keeps failed default row', host.ok('SELECT status,error_code FROM ' + call + ')'),
                      [{'status': 'failed', 'error_code': 'record.missing'}])
                error = host.failure('SELECT * FROM ' + call + ', raise_if_failed := true)')
                assert '1 of 1 records failed' in error and 'position 1' in error, (function, error)
            equal('failed evidence with raise sends no requests', command(servers[0], 'count'), requests)
            line = 'Please refund my order.'
            text = 'é😀 lead\n' + line
            from cases import quote
            sql = ("SELECT answer.text AS line,substr(record,answer.start_offset+1,answer.end_offset-answer.start_offset) AS sliced "
                   "FROM thinkthen_find((SELECT " + quote(text) + " AS record), 'Where?')")
            equal('Unicode span offsets slice original SQL text', host.ok(sql), [{'line': line, 'sliced': line}])

    for name in ('failed', 'children-failed', 'refund-failed', 'topic-failed', 'urgency-failed', 'refund-unsure', 'topic-unsure'):
        with scenario(name, case={'replies': []} if name == 'children-failed' else None) as (_, env):
            env['THINKTHEN_TEST_LOOPBACK_URL'] = env['THINKTHEN_BASE_URL']
            with Host(package, env) as host:
                host.ok('SET thinkthen_max_retries = 0')
                question = {'function': 'annotate', 'questions': {
                    'refund': {'function': 'decide', 'question': 'Ready?', 'threshold': {'low': .2, 'high': .8}},
                    'topic': {'function': 'choose', 'question': 'Which?', 'choices': ['billing', 'shipping', 'other']},
                    'urgency': {'function': 'score', 'question': 'How?', 'levels': ['routine', 'urgent']}}}
                call = invocation({'question': question, 'records': ['text']})
                rows = host.ok('SELECT to_json(detail) AS detail,status FROM ' + call)
                detail = json.loads(rows[0]['detail'])
                assert rows[0]['status'] == 'answered', rows
                for field, state in child_states(name).items():
                    equal(name + ' saved child status', detail[field]['status'], state)
                    if state == 'failed':
                        code = 'backend.status' if name == 'failed' else 'backend.reply'
                        assert detail[field]['error']['code'] == code and detail[field]['answer'] is None, detail
                raising = 'SELECT * FROM ' + call[:-1] + ', raise_if_failed := true)'
                if name.endswith('failed'):
                    error = host.failure(raising)
                    assert '1 of 1 records failed' in error and 'position 1' in error, error
                else:
                    equal(name + ' never raises for uncertainty', host.ok('SELECT count(*) AS n FROM (' + raising + ')'), [{'n': '1'}])
    print('PASS DuckDB pre-request refusals, nested names, saved child errors and opt-in failure', flush=True)


def limit_probe(package):
    for function, extra in (('filter', ''), ('decide', ', raise_if_failed := true')):
        for outside in ('', ' LIMIT 1'):
            with scenario('yes', observer=(servers := [])) as (_, env):
                env['THINKTHEN_TEST_LOOPBACK_URL'] = env['THINKTHEN_BASE_URL']
                with Host(package, env) as host:
                    host.ok('SET thinkthen_max_retries = 0')
                    sql = (f"SELECT position FROM thinkthen_{function}((SELECT CASE WHEN record_read(n)=1 THEN 'text' ELSE NULL END AS record "
                           f"FROM (VALUES (1),(2)) input(n)), 'Ready?'" + extra + ')' + outside)
                    result = host.query(sql)
                    consumed = host.ok('SELECT record_reads() AS n')
                    equal('LIMIT probe consumed both inputs', consumed, [{'n': '2'}])
                    equal('LIMIT probe only valid evidence sent', command(servers[0], 'count'), '1')
                    if 'error' in result:
                        assert '1 of 2 records failed' in result['error'] and 'position 2' in result['error'], result
                        disposition = 'failure reported'
                    else:
                        assert outside, ('fully consumed call hid failure', result)
                        equal('LIMIT output', result['rows'], [{'position': '1'}])
                        disposition = 'already-read failure hidden'
                    print(f'LIMIT DuckDB {function} outside={bool(outside)} consumed=2 requests=1: {disposition}', flush=True)
    print('PASS DuckDB outer LIMIT accounting probe', flush=True)
