"""Focused installed named C++ calls, deriving questions from shared conformance."""
import json
from pathlib import Path
import subprocess
import sys
import time
import tempfile
sys.path.insert(0, str(Path(__file__).resolve().parents[3] / 'conformance/children'))
from children import child_env
from fixture import Backend
ROOT = Path(__file__).resolve().parents[3]
prefix, binary, scratch_root = (path.resolve() for path in map(Path, sys.argv[1:]))
scratch_root.mkdir(parents=True, exist_ok=True)
owned = tempfile.TemporaryDirectory(prefix='caller-',dir=scratch_root)
scratch = Path(owned.name)
linked = subprocess.check_output(['ldd',binary],env=child_env(),text=True)
assert 'libthinkthen.so.0 => ' + str(prefix / 'lib/libthinkthen.so.0') in linked, linked
subprocess.run([sys.executable,str(Path(__file__).with_name("guard.py")),prefix],env=child_env(),check=True)
server = Backend(scratch)
cases = json.loads((ROOT / 'conformance/cases.json').read_text())['cases']
try:
    env = child_env(HOME=str(scratch), XDG_CONFIG_HOME=str(scratch),
                    XDG_CACHE_HOME=str(scratch / 'cache'), XDG_STATE_HOME=str(scratch / 'state'),
                    THINKTHEN_BASE_URL=f'http://127.0.0.1:{server.server_port}/generic/v1',
                    THINKTHEN_API_KEY='tt-canary-301', THINKTHEN_CACHE=str(scratch / 'cache'))
    for verb in ('decide','choose','tag','score','filter','rank','find','annotate','recognize','relate'):
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
        result = subprocess.run([binary, verb, qpath, ipath], env=env, capture_output=True, text=True, timeout=20)
        assert result.returncode == 0, (verb, result.returncode, result.stdout, result.stderr)
        packets = [json.loads(line) for line in result.stdout.splitlines()]
        terminal = packets[-1]
        assert terminal['kind'] == 'terminal' and 'failure' not in terminal, terminal
        assert terminal['facts']['requests_sent'] >= 1, terminal
        print('CPP_INSTALLED_NAMED_PASS', verb, 'packets=' + str(len(packets)))
    assert len(server.requests) == 11, len(server.requests)
    baseline = len(server.requests)
    for mode in ('values','invalid','image','failure'):
        qpath = scratch / ('tag-question.json' if mode == 'image' else 'decide-question.json')
        result = subprocess.run([binary,mode,qpath,qpath],env=env,capture_output=True,text=True,timeout=5)
        assert result.returncode == 0, (mode,result.returncode,result.stdout,result.stderr)
        assert result.stdout.strip() in ('values-pass','admission-pass','failure-pass'), result
        assert len(server.requests) == baseline + (mode == 'failure'), (mode,len(server.requests))
        print('CPP_INSTALLED_EDGE_PASS',mode)
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
