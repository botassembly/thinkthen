"""Install the bundled native package and exercise ten generated named calls."""
import json
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import tarfile
sys.path.insert(0, str(Path(__file__).resolve().parents[3] / 'conformance/children'))
from children import child_env
from session_values import ROOT, cases, construct, definitions

PACKAGE = ROOT / 'libraries/ada'
subprocess.run([sys.executable, str(ROOT / 'sdlc/generators/results/generate.py'), '--target', 'ada', '--check'], env=child_env(), check=True)
with tempfile.TemporaryDirectory(prefix='thinkthen-ada-session-') as temporary:
    work = Path(temporary)
    installed = work / 'installed package'
    shutil.copytree(PACKAGE / 'src', installed / 'src')
    shutil.copy2(PACKAGE / 'thinkthen.gpr', installed / 'thinkthen.gpr')
    (installed / 'native/include').mkdir(parents=True)
    (installed / 'native/lib').mkdir()
    shutil.copy2(ROOT / 'libraries/c/include/thinkthen.h', installed / 'native/include/thinkthen.h')
    subprocess.run(['sh', str(ROOT / 'libraries/c/localize.sh'), str(ROOT / 'libraries/c/target/debug/libthinkthen_c.a'), str(installed / 'native/lib/libthinkthen.a')], env=child_env(), check=True, capture_output=True, timeout=60)
    for file in ('README.md', 'LICENSE'):
        shutil.copy2(PACKAGE / file, installed / file)
    shutil.copytree(PACKAGE / 'examples', installed / 'examples')
    artifact = PACKAGE / 'checks/target/native-session-package.tar.gz'
    artifact.parent.mkdir(parents=True, exist_ok=True)
    with tarfile.open(artifact, 'w:gz') as archive:
        archive.add(installed, arcname='thinkthen')
    shutil.rmtree(installed)
    with tarfile.open(artifact) as archive:
        archive.extractall(work / 'extracted', filter='data')
    (work / 'extracted/thinkthen').rename(installed)
    declarations, calls = [], []
    for verb, case, expression in cases():
        declarations.append(f'R_{verb} : constant T_RequestCall_{verb} := {expression};')
        calls.append(f'Thinkthen.Sessions.Calls.{verb.title()} (Owner, R_{verb}); Finish (Owner); Drain; Put_Line ("SHARED_CASE {case}");')
    # Admission, cancellation and execution failures add no host-side semantic rules.
    key = 'RequestCall_decide'
    def request(question, original):
        return construct(key, definitions[key], {'function':'decide', 'question':question, 'input':original, 'options':{'details':True}})
    failure = request({'kind':'text','text':'Does it pass?'}, {'kind':'text','text':'failure-one'})
    feed = request({'kind':'text','text':'Does it pass?'}, {'kind':'feed','name':'owned'})
    invalid = request({'kind':'text','text':''}, {'kind':'text','text':'invalid'})
    false_request = request({'kind':'definition','value':{'decide':'Does it pass?', 'threshold':0.95}}, {'kind':'text','text':'False evidence.'})
    null_request = request({'kind':'definition','value':{'decide':'Does it pass?', 'threshold':'0.7:0.95'}}, {'kind':'text','text':'Null evidence.'})
    item = construct('RequestSessionDescriptor', definitions['RequestSessionDescriptor'], {'item':{'original':{'kind':'text','text':'Located evidence.'}}, 'location':{'file':'owned.txt','first_line':1,'last_line':1}})
    declarations += [f'False_Request : constant T_RequestCall_decide := {false_request};', f'Null_Request : constant T_RequestCall_decide := {null_request};', f'Item : T_RequestSessionDescriptor := {item};', 'Pushed : Push_Status;', f'Failed : constant T_RequestCall_decide := {failure};', f'Feed : constant T_RequestCall_decide := {feed};', f'Invalid : constant T_RequestCall_decide := {invalid};']
    calls += ['Thinkthen.Sessions.Calls.Decide (Owner, False_Request); Finish (Owner); Drain (Decision => 0);', 'Thinkthen.Sessions.Calls.Decide (Owner, Null_Request); Finish (Owner); Drain (Decision => 1);', 'Thinkthen.Sessions.Calls.Decide (Owner, Feed); Push (Owner, Item, Pushed); if Pushed /= Accepted then raise Program_Error with "feed refused"; end if; Finish (Owner); Drain (Located => True);', 'Thinkthen.Sessions.Calls.Decide (Owner, Feed); Cancel (Owner); Drain (Cancelled);', '''declare
Refused : Boolean := False;
begin
begin
Thinkthen.Sessions.Calls.Decide (Owner, Invalid);
exception
when Usage_Error => Refused := True;
end;
if not Refused then raise Program_Error with "invalid question admitted"; end if;
end;''']
    source = (PACKAGE / 'checks/native_session.adb').read_text().replace('-- DECLARATIONS', '\n'.join(declarations)).replace('-- CALLS', '\n'.join(calls))
    (work / 'native_session.adb').write_text(source)
    (work / 'consumer.gpr').write_text('with "installed package/thinkthen.gpr";\nproject Consumer is\nfor Main use ("native_session.adb");\nfor Object_Dir use "objects";\npackage Compiler is\nfor Default_Switches ("Ada") use ("-gnat2022");\nend Compiler;\nend Consumer;\n')
    build = subprocess.run(['gprbuild', '-p', '-P', 'consumer.gpr', '-j2', '-cargs', '-O0'], cwd=work, env=child_env(), capture_output=True, text=True, timeout=60)
    assert build.returncode == 0, build.stdout + build.stderr
    backend = subprocess.Popen([str(ROOT / 'target/debug/conformance-backend')], env=child_env(), stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
    try:
        port = int(backend.stdout.readline())
        for mode, route in (('', 'generic/v1'), ('failure', 'arm/status/401/v1')):
            env = child_env(home=work / ('home-' + mode), THINKTHEN_BASE_URL=f'http://127.0.0.1:{port}/{route}', THINKTHEN_API_KEY='sk-conformance-loopback')
            command = [str(work / 'objects/native_session')] + ([mode] if mode else [])
            result = subprocess.run(command, cwd=work, env=env, text=True, capture_output=True, timeout=30)
            print(result.stdout)
            assert result.returncode == 0, result.stderr
            assert 'INSTALLED_ADA_NATIVE_SESSION_PASS' in result.stdout
        backend.stdin.write('count\n'); backend.stdin.flush()
        count = backend.stdout.readline().strip()
        assert count == '19', count
        print('native session loopback requests:', count)
        print('development package artifact:', artifact)
    finally:
        backend.stdin.close()
        backend.wait(timeout=10)
        diagnostics = backend.stderr.read()
        assert not diagnostics, diagnostics
