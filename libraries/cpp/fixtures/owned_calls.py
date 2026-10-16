"""Focused installed named C++ calls, deriving questions from shared conformance."""
import json
from pathlib import Path
import subprocess
import sys
import time
import tempfile
sys.path.insert(0, str(Path(__file__).resolve().parents[3] / 'conformance/children'))
from children import child_env
from usage import run_usage
from backend import Backend
ROOT = Path(__file__).resolve().parents[3]
prefix, binary, scratch_root = (path.resolve() for path in map(Path, sys.argv[1:4]))
scratch_root.mkdir(parents=True, exist_ok=True)
owned = tempfile.TemporaryDirectory(prefix='caller-',dir=scratch_root)
scratch = Path(owned.name)
linked = subprocess.check_output(['ldd',binary],env=child_env(),text=True)
if len(sys.argv)>4 and sys.argv[4]=='static':
    assert 'libthinkthen.so.0' not in linked,linked
else:
    assert 'libthinkthen.so.0 => ' + str(prefix / 'lib/libthinkthen.so.0') in linked, linked
# The focused behavior mode reuses a warm native build; distribution privacy
# stays on the ordinary installed-package route with qualified artifacts.
if len(sys.argv) <= 4 or sys.argv[4] != 'usage':
    subprocess.run([sys.executable,str(Path(__file__).with_name("guard.py")),prefix,sys.argv[5]],env=child_env(),check=True)
server = Backend(scratch, key='tt-canary-301', arrivals='named')
cases = json.loads((ROOT / 'conformance/cases.json').read_text())['cases']
try:
    env = child_env(home=str(scratch), XDG_CONFIG_HOME=str(scratch),
                    XDG_CACHE_HOME=str(scratch / 'cache'), XDG_STATE_HOME=str(scratch / 'state'),
                    THINKTHEN_BASE_URL=f'http://127.0.0.1:{server.server_port}/generic/v1',
                    THINKTHEN_API_KEY='tt-canary-301', THINKTHEN_CACHE=str(scratch / 'cache'),
                    THINKTHEN_CPP_RECORD=str(scratch / 'recorded'))
    if len(sys.argv) > 4 and sys.argv[4] == 'usage':
        for mode in ('usage-written', 'usage-failed'):
            state = scratch / mode / 'state/thinkthen'; state.mkdir(mode=0o700, parents=True)
            before = len(server.requests)
            usage_env = child_env(home=scratch / mode, THINKTHEN_BASE_URL=env['THINKTHEN_BASE_URL'],
                                  THINKTHEN_API_KEY='tt-canary-301', THINKTHEN_CACHE=str(scratch / 'cache'))
            result = run_usage([binary, mode, scratch, scratch], env=usage_env, state=state, mode=mode.removeprefix('usage-'), timeout=5)
            assert result.returncode == 0 and result.stdout.strip() == 'usage-status-pass requests=1', (mode, result.returncode, result.stdout, result.stderr)
            assert len(server.requests) == before + 1, server.requests
            print('CPP_USAGE_INSTALLED_PASS', mode, 'requests=1')
        sys.exit(0)
    schema = subprocess.run([binary.parent / "schema_projection"], env=env, capture_output=True, text=True, timeout=20)
    assert schema.returncode == 0, (schema.returncode, schema.stdout, schema.stderr)
    print(schema.stdout.strip())
    before = len(server.requests)
    preview = subprocess.run([binary, 'plan', scratch, scratch], env=env, capture_output=True, text=True, timeout=5)
    assert preview.returncode == 0, (preview.stdout, preview.stderr)
    assert json.loads(preview.stdout) and len(server.requests) == before, preview.stdout
    print('CPP_INSTALLED_PLAN_PASS requests=0')
    for verb in ('decide','choose','tag','score','filter','rank','find','annotate','recognize','relate'):
        source = prefix / 'share/thinkthen-cpp/examples' / (verb + '.cpp')
        assert source.is_file(), 'CPP_EXAMPLE_MISSING ' + verb
        authored = source.read_text()
        assert '.result()' in authored and not any(old in authored for old in ('.collect(', 'as_SessionPacket', 'from_document(')), 'CPP_EXAMPLE_STALE ' + verb
        case = next(case for case in cases if case['verb'] == verb)
        question = dict(case.get('question', case.get('question_set', {})))
        original = case.get('text', case['exchanges'][0]['evidence'])
        if verb == 'find':
            original = question.pop('units')
            question.pop('none', None)
        if verb == 'relate': original = case['entities']
        qpath = scratch / (verb + '-question.json')
        ipath = scratch / (verb + '-input.json')
        qpath.write_text(json.dumps(question))
        ipath.write_text(json.dumps(original))
        before_plan = len(server.requests)
        planned = subprocess.run([binary, 'plan-' + verb, qpath, ipath], env=env, capture_output=True, text=True, timeout=5)
        assert planned.returncode == 0 and len(server.requests) == before_plan, (verb, planned.returncode, planned.stdout, planned.stderr)
        if verb in ('decide', 'choose', 'tag', 'score', 'filter', 'rank'):
            plan = json.loads(planned.stdout)
            required = json.loads((ROOT / 'specification/result.schema.json').read_text())['$defs']['completeplan']['required']
            assert set(plan) == set(required), (verb, plan)
        else:
            assert planned.stdout.strip() == 'plan-refusal-pass', (verb, planned.stdout)
        print('CPP_INSTALLED_PLAN_FUNCTION_PASS', verb, 'requests=0')
        result = subprocess.run([binary, verb, qpath, ipath], env=env, capture_output=True, text=True, timeout=20)
        assert result.returncode == 0, (verb, result.returncode, result.stdout, result.stderr)
        packets = [json.loads(line) for line in result.stdout.splitlines()]
        terminal = packets[-1]
        assert terminal['kind'] == 'terminal' and 'failure' not in terminal, terminal
        assert terminal['facts']['requests_sent'] >= 1, terminal
        print('CPP_INSTALLED_NAMED_PASS', verb, 'packets=' + str(len(packets)))
        example = binary.parent / ('example_' + verb)
        sampled = subprocess.run([example, qpath, ipath, scratch / 'recorded'], env=env, capture_output=True, text=True, timeout=20)
        assert sampled.returncode == 0, (verb, sampled.returncode, sampled.stdout, sampled.stderr)
        assert sampled.stderr == '', (verb, sampled.stderr)
        sampled_result = json.loads(sampled.stdout)
        assert sampled_result.keys() == {'value', 'probability', 'results', 'facts'}, sampled_result
        assert sampled_result['facts']['requests_sent'] >= 0, sampled_result
        assert sampled_result['results'], sampled_result
        sampled_value = sampled_result['value']
        if verb == 'decide':
            assert sampled_value is True and sampled_result['probability'] == 0.9, sampled_result
        elif verb == 'choose':
            assert sampled_result['probability'] == 0.9, sampled_result
        elif verb in ('tag', 'score', 'annotate'):
            assert sampled_result['probability'] is None, sampled_result
        if verb in ('decide', 'choose', 'tag', 'score'):
            judgments = [p['value']['value']['value'] for p in packets
                         if p['kind'] == 'observation' and p['value']['kind'] == 'row'
                         and p['value']['value']['kind'] == 'judgment']
            assert len(judgments) == 1 and sampled_value == judgments[0]['value'], (verb, sampled_value, judgments)
        elif verb == 'filter':
            assert sampled_value == [original], (verb, sampled_value)
        elif verb == 'rank':
            assert sampled_value == [0], (verb, sampled_value)
        elif verb == 'recognize':
            recognition = next(packet['value'][0] for packet in packets if packet['kind'] == 'aggregate')
            assert sampled_value == recognition['value']['entities'], sampled_value
            assert len(sampled_result['probability']) == len(sampled_value), sampled_result
            for entity, probability in zip(sampled_value, sampled_result['probability'], strict=True):
                proposal = next(p for p in recognition['answer']['proposals'] if p['kept'] and p.get('kind') == entity['kind'] and p.get('selected', {}).get('start') == entity['start'] and p.get('selected', {}).get('end') == entity['end'])
                assert probability == proposal['span_probability'], (probability, proposal)
        elif verb == 'annotate':
            assert isinstance(sampled_value, dict), sampled_value
        elif verb == 'relate':
            assert sampled_result['probability'] == [edge['probability'] for edge in sampled_value], sampled_result
        print('CPP_INSTALLED_EXAMPLE_PASS', verb)

    assert len(server.requests) == 11, len(server.requests)
    question = {"version": 1, "questions": {"refund": {"decide": "Does this ask for a refund?", "threshold": "0.2:0.8"}}}
    qpath = scratch / 'annotate-null-question.json'
    ipath = scratch / 'annotate-null-input.json'
    qpath.write_text(json.dumps(question))
    ipath.write_text(json.dumps('unsure'))
    result = subprocess.run([binary,'annotate-null',qpath,ipath],env=env,capture_output=True,text=True,timeout=5)
    assert result.returncode == 0, (result.returncode,result.stdout,result.stderr)
    packets = [json.loads(line) for line in result.stdout.splitlines()]
    rows = [packet['value'] for packet in packets if packet['kind']=='row']
    assert len(rows)==1 and rows[0]['value']=={'refund': None}, rows
    assert packets[-1]['kind']=='terminal' and 'failure' not in packets[-1] and packets[-1]['facts']['requests_sent']==1, packets[-1]
    assert len(server.requests)==12, len(server.requests)
    print('CPP_INSTALLED_NULL_ANNOTATION_PASS present_null retained_document terminal_facts')
    before = len(server.requests)
    result = subprocess.run([binary,'locale',scratch,scratch],env=env,capture_output=True,text=True,timeout=5)
    assert result.returncode == 0, (result.returncode,result.stdout,result.stderr)
    packets = [json.loads(line) for line in result.stdout.splitlines()]
    assert packets[-1]['kind']=='terminal' and 'failure' not in packets[-1] and packets[-1]['facts']['requests_sent']==1, packets[-1]
    assert len(server.requests)==before+1, server.requests
    print('CPP_INSTALLED_LOCALE_PASS nested_input probabilities retained_packets')
    baseline = len(server.requests)
    for mode in ('values','invalid','image','closed','failure'):
        qpath = scratch / ('tag-question.json' if mode == 'image' else 'decide-question.json')
        result = subprocess.run([binary,mode,qpath,qpath],env=env,capture_output=True,text=True,timeout=5)
        assert result.returncode == 0, (mode,result.returncode,result.stdout,result.stderr)
        assert result.stdout.strip() in ('values-pass','admission-pass','failure-pass','closed-push-pass'), result
        assert len(server.requests) == baseline + (mode == 'failure'), (mode,len(server.requests))
        print('CPP_INSTALLED_EDGE_PASS',mode)
    for mode, sends in (('cardinality-zero', 0), ('cardinality-many', 1)):
        before = len(server.requests)
        result = subprocess.run([binary, mode, scratch, scratch], env=env, capture_output=True, text=True, timeout=5)
        assert result.returncode == 0 and result.stdout.strip() == ('cardinality-pass judgments=0' if sends == 0 else 'cardinality-pass judgments=2'), (mode, result.returncode, result.stdout, result.stderr)
        assert len(server.requests) == before + sends, (mode, len(server.requests), before)
        if sends:
            wire = server.requests[before]
            assert wire['state'] == 'Each question quotes the text it asks about.' and len(wire['questions']) == 2, wire
            assert [question['instructions'] for question in wire['questions'].values()] == ['The text is "first". Is it?', 'The text is "second". Is it?'], wire
        print('CPP_INSTALLED_CARDINALITY_PASS', mode)
    for original, yes, no, expected, odds in (
        ('yes', None, 'kept', None, 0.9),
        ('no', 'kept', None, None, 0.1),
        ('unsure', None, False, None, None),
        ('no', True, False, False, 0.1),
        ('yes', {'wide': 18446744073709551615, 'false': False}, None, {'wide': 18446744073709551615, 'false': False}, 0.9),
        ('yes', [None, False, {'x': 7}], None, [None, False, {'x': 7}], 0.9),
        (['yes', 'no', 'unsure'], {'yes': True}, False, [{'yes': True}, False, None], [0.9, 0.1, None]),
        (['yes'], None, False, [None], [0.9]),
    ):
        qpath=scratch / 'result-question.json'; ipath=scratch / 'result-input.json'
        definition={'decide':'Is it?', 'threshold':'0.2:0.8'}
        if not isinstance(yes, bool): definition['true']=yes
        if not isinstance(no, bool): definition['false']=no
        qpath.write_text(json.dumps(definition))
        ipath.write_text(json.dumps(original))
        result=subprocess.run([binary,'result-decide',qpath,ipath],env=env,capture_output=True,text=True,timeout=5)
        assert result.returncode==0, (result.stdout,result.stderr)
        value=json.loads(result.stdout)
        assert value['value']==expected and value['probability']==odds, value
        assert len(value['results'])==(len(original) if isinstance(original,list) else 1), value
        assert value['facts']['requests_sent']>=0, value
    print('CPP_INSTALLED_AUTHORED_RESULT_PASS null false object list wide_integer input_order ownership')
    for definition in ({'version':1,'questions': {'one': {'decide': 'Is it?'}, 'two': {'decide': 'Is it still?'}}},
                       {'score':'How is it?', 'levels':['low','high']}):
        qpath=scratch / 'whole-set-question.json'; ipath=scratch / 'whole-set-input.json'
        qpath.write_text(json.dumps(definition)); ipath.write_text(json.dumps(['yes','no','unsure']))
        result=subprocess.run([binary,'result-rank',qpath,ipath],env=env,capture_output=True,text=True,timeout=5)
        assert result.returncode==0, (result.stdout,result.stderr)
        ranked=json.loads(result.stdout)
        assert sorted(ranked['value'])==[0,1,2] and len(ranked['results'])==3, ranked
        assert ranked['value']==[row['index'] for row in ranked['results']], ranked
        assert ranked['probability']==[row['answer'].get('probability') for row in ranked['results']], ranked
    print('CPP_INSTALLED_WHOLE_SET_RANK_PASS question_set score nullable_probabilities')
    qpath.write_text(json.dumps({'find':'Which one?'})); ipath.write_text(json.dumps([None,'no']))
    result=subprocess.run([binary,'result-find',qpath,ipath],env=env,capture_output=True,text=True,timeout=5)
    assert result.returncode==0, (result.stdout,result.stderr)
    found=json.loads(result.stdout)
    assert found['value'] is None and found['results'][0]['index']==0 and found['probability']==0.9, found
    print('CPP_INSTALLED_NULL_FIND_PASS selected_null_retains_probability')
    qpath.write_text(json.dumps({'version':1,'recognize': {'mode':'boundary_only'}, 'threshold':0.0001})); ipath.write_text(json.dumps('Ada arrived.'))
    result=subprocess.run([binary,'result-recognize',qpath,ipath],env=env,capture_output=True,text=True,timeout=5)
    assert result.returncode==0, (result.stdout,result.stderr)
    boundaries=json.loads(result.stdout)
    assert boundaries['value'] and boundaries['probability']==[item['probability'] for item in boundaries['value']], boundaries
    print('CPP_INSTALLED_BOUNDARY_RESULT_PASS proposal_probability')
    qpath.write_text(json.dumps({'version':1,'recognize': {'kinds':{'person':"A person's name."}}, 'threshold':0.0001}))
    result=subprocess.run([binary,'result-recognize',qpath,ipath],env=env,capture_output=True,text=True,timeout=5)
    assert result.returncode==0, (result.stdout,result.stderr)
    recognized=json.loads(result.stdout)
    assert recognized['value'] and len(recognized['probability'])==len(recognized['value']), recognized
    assert any(probability!=entity['strength'] for probability,entity in zip(recognized['probability'],recognized['value'],strict=True)), recognized
    proposals=recognized['results'][0]['answer']['proposals']
    for entity, probability in zip(recognized['value'],recognized['probability'],strict=True):
        proposal=next(p for p in proposals if p['kept'] and p.get('kind')==entity['kind'] and p.get('selected',{}).get('start')==entity['start'] and p.get('selected',{}).get('end')==entity['end'])
        assert probability==proposal['span_probability'], (probability,proposal)
    print('CPP_INSTALLED_RECOGNITION_ODDS_PASS kept_span_probability differs_from_strength')
    source = scratch / 'physical.txt'
    source.write_text('yes\n\nno\n')
    result = subprocess.run([binary,'files',scratch / 'decide-question.json',source],env=env,capture_output=True,text=True,timeout=5)
    assert result.returncode == 0, (result.stdout,result.stderr)
    rows = [json.loads(line)['value'] for line in result.stdout.splitlines() if json.loads(line)['kind']=='row']
    assert [row['source']['first_line'] for row in rows] == [1,3], rows
    assert [row['source']['file'] for row in rows] == [str(source),str(source)], rows
    print('CPP_INSTALLED_FILES_PASS physical_lines=1,3')
    held = subprocess.Popen([binary,'held',source,source],env=env,stdin=subprocess.PIPE,stdout=subprocess.PIPE,stderr=subprocess.PIPE,text=True)
    try:
        deadline=time.monotonic()+5
        while not (scratch / 'arrived-hold-cpp-owned').exists():
            assert held.poll() is None and time.monotonic()<deadline, 'provider did not become held'
            time.sleep(.005)
        out,err=held.communicate('!',timeout=5)
        assert held.returncode==0 and out.splitlines()==['started','host-progress','cleaned'] and not err, (out,err)
        assert 'hold-cpp-owned' not in server.completions, server.completions
        print('CPP_INSTALLED_HELD_PASS host_progress cancel_cleanup_before_release')
    finally:
        (scratch / 'release-hold-cpp-owned').touch()
        if held.poll() is None: held.kill(); held.communicate()

    print('CPP_INSTALLED_TEN_PASS requests=' + str(len(server.requests)))
finally:
    (scratch / 'release-hold-cpp-owned').touch()
    server.close()
    owned.cleanup()
