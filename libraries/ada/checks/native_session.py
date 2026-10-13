"""Install the bundled native package and exercise ten generated named calls."""
import json
import os
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
    supplied = os.environ.get('THINKTHEN_ARTIFACT')
    if supplied:
        artifact = Path(supplied).resolve(strict=True)
    else:
        shutil.copytree(PACKAGE / 'src', installed / 'src')
        shutil.copy2(PACKAGE / 'thinkthen.gpr', installed / 'thinkthen.gpr')
        (installed / 'native/include').mkdir(parents=True)
        (installed / 'native/lib').mkdir()
        shutil.copy2(ROOT / 'libraries/c/include/thinkthen.h', installed / 'native/include/thinkthen.h')
        subprocess.run(['sh', str(ROOT / 'libraries/c/localize.sh'), os.environ.get('THINKTHEN_C_STATIC_LIBRARY', str(ROOT / 'libraries/c/target/debug/libthinkthen_c.a')), str(installed / 'native/lib/libthinkthen.a')], env=child_env(), check=True, capture_output=True, timeout=60)
        for file in ('README.md', 'LICENSE'):
            shutil.copy2(PACKAGE / file, installed / file)
        (installed / 'examples').mkdir()
        shutil.copy2(PACKAGE / 'examples/session_demo.adb', installed / 'examples/session_demo.adb')
        artifact = PACKAGE / 'checks/target/native-session-package.tar.gz'
        artifact.parent.mkdir(parents=True, exist_ok=True)
        with tarfile.open(artifact, 'w:gz') as archive:
            archive.add(installed, arcname='thinkthen')
        shutil.rmtree(installed)
    with tarfile.open(artifact) as archive:
        archive.extractall(work / 'extracted', filter='data')
    unpacked = work / 'extracted'
    if not (unpacked / 'src').is_dir():
        unpacked = unpacked / 'thinkthen'
    unpacked.rename(installed)
    expected = {'thinkthen', 'thinkthen-sessions', 'thinkthen-sessions-calls', 'thinkthen-persistence', 'thinkthen-requests-native', 'thinkthen_kinds', 'thinkthen-values', 'thinkthen-answers'}
    assert {p.stem for p in (installed / 'src').glob('*.adb')} == expected
    assert {p.stem for p in (installed / 'src').glob('*.ads')} == expected | {'thinkthen_session_c', 'thinkthen-requests'}
    if '--full' in sys.argv:
        subprocess.run([sys.executable, str(PACKAGE / 'checks/complete_parity.py'), '--package', str(installed)], env=child_env(THINKTHEN_BACKEND_BIN=os.environ.get('THINKTHEN_BACKEND_BIN', str(ROOT / 'target/debug/conformance-backend'))), check=True)
        raise SystemExit(0)
    declarations, calls = [], []
    for verb, case, expression in cases():
        declarations.append(f'R_{verb} : constant T_RequestCall := {expression};')
        calls.append(f'Select_Route ("case/{case}/v1"); Thinkthen.Sessions.Calls.{verb.title()} (Client, Owner, R_{verb}); Finish (Owner); Drain{" (Odds => 0.99)" if verb == "decide" else ""}; Put_Line ("SHARED_CASE {case}");')
    answer_checks = {
        'decide': '''Require (Decide_Answer (Completed).Known and then Decide_Answer (Completed).Value, "safe decide"); declare Refused : Boolean := False; begin begin declare Ignored : constant Choice := Choose_Answer (Completed); begin if Ignored.Known then null; end if; end; exception when Answer_Cardinality_Error => Refused := True; end; Require (Refused and then Has_Facts (Completed) and then Packet_Count (Completed) > 0, "cardinality refusal retained completion"); end;''',
        'choose': '''Require (Choose_Answer (Completed).Known and then To_String (Choose_Answer (Completed).Value) = "billing", "safe choose");''',
        'tag': '''Require (Tag_Answer (Completed)'Length = 2, "safe tags");''',
        'score': '''Require (Score_Answer (Completed) > 0.0, "safe score");''',
        'filter': '''Require (Filter_Answer (Completed)'Length = 2, "plain filter list");''',
        'rank': '''Require (Rank_Answer (Completed)'Length = 3, "plain rank list");''',
        'find': '''Require (Has_Match (Find_Answer (Completed)), "safe find");''',
        'annotate': '''Require (Annotate_Answer (Completed)'Length > 0, "safe annotations");''',
        'recognize': '''Require (Recognize_Answer (Completed)'Length = 1, "safe recognition aggregate");''',
        'relate': '''declare Item : constant Thinkthen.Values.Value := Edges (Relate_Answer (Completed)); begin Require (Thinkthen.Values.Length (Item) > 0 and then Source_Name (Thinkthen.Values.Element (Item, 1))'Length > 0 and then Target_Name (Thinkthen.Values.Element (Item, 1))'Length > 0, "direct relation names"); end;''',
    }
    for verb, case, _ in cases():
        calls.append(f'Select_Route ("case/{case}/v1"); Thinkthen.Sessions.Calls.{verb.title()} (Client, Owner, R_{verb}); Finish (Owner); declare use Thinkthen.Answers; Completed : constant Completion := Collect (Owner); begin Close (Owner); Require (Status (Completed) = Succeeded and then Has_Facts (Completed) and then Requests_Sent (Call_Facts (Completed)) > 0, "owned safe completion"); {answer_checks[verb]} end;')
    # Admission, cancellation and execution failures add no host-side semantic rules.
    key = 'RequestCall'
    def request(question, original):
        return construct(key, definitions[key], {'function':'decide', 'question':question, 'input':original, 'options':{'details':True}})
    failure = request({'kind':'text','text':'Does it pass?'}, {'kind':'text','text':'failure-one'})
    feed = request({'kind':'text','text':'Does it pass?'}, {'kind':'feed','name':'owned'})
    invalid = request({'kind':'text','text':''}, {'kind':'text','text':'invalid'})
    false_request = request({'kind':'definition','value':{'decide':'Does it pass?', 'threshold':0.95}}, {'kind':'text','text':'False evidence.'})
    null_request = request({'kind':'definition','value':{'decide':'Does it pass?', 'threshold':'0.7:0.95'}}, {'kind':'text','text':'Null evidence.'})
    item = construct('RequestSessionDescriptor', definitions['RequestSessionDescriptor'], {'item':{'original':{'kind':'text','text':'Located evidence.'}}, 'location':{'file':'owned.txt','first_line':1,'last_line':1}})
    declarations += [f'False_Request : constant T_RequestCall := {false_request};', f'Null_Request : constant T_RequestCall := {null_request};', f'Item : T_RequestSessionDescriptor := {item};', 'Pushed : Push_Status;', f'Failed : constant T_RequestCall := {failure};', f'Feed : constant T_RequestCall := {feed};', f'Invalid : constant T_RequestCall := {invalid};']
    calls += ['Select_Route ("generic/v1");', 'declare Preview : constant String := Thinkthen.Sessions.Calls.Plan (Client, False_Request); begin Require (Ada.Strings.Fixed.Index (Preview, """records"":1") > 0, "typed plan result"); end;', 'Thinkthen.Sessions.Calls.Decide (Owner, False_Request); Finish (Owner); Drain (Decision => 0);', 'Thinkthen.Sessions.Calls.Decide (Owner, Null_Request); Finish (Owner); Drain (Decision => 1);', 'Thinkthen.Sessions.Calls.Decide (Owner, Feed); Push (Owner, Item, Pushed); if Pushed /= Accepted then raise Program_Error with "feed refused"; end if; Finish (Owner); Drain (Located => True);', 'Thinkthen.Sessions.Calls.Decide (Owner, Feed); Cancel (Owner); Drain (Cancelled);', '''declare
Refused : Boolean := False;
begin
begin
Thinkthen.Sessions.Calls.Decide (Owner, Invalid);
exception
when Usage_Error => Refused := True;
end;
if not Refused then raise Program_Error with "invalid question admitted"; end if;
end;''']
    calls += [
        'Thinkthen.Sessions.Calls.Decide (Owner, False_Request); Finish (Owner); declare Completed : Thinkthen.Answers.Completion; Answer : constant Thinkthen.Answers.Decision := Thinkthen.Answers.Decide_Answer (Owner, Completed); begin Require (Answer.Known and then not Answer.Value, "safe false value"); Require (Thinkthen.Answers.Requests_Sent (Thinkthen.Answers.Call_Facts (Completed)) = 0 and then Thinkthen.Answers.Cache_Answers (Thinkthen.Answers.Call_Facts (Completed)) = 1, "safe cached facts"); end;',
        'Thinkthen.Sessions.Calls.Decide (Owner, Null_Request); Finish (Owner); declare Completed : Thinkthen.Answers.Completion; Answer : constant Thinkthen.Answers.Decision := Thinkthen.Answers.Decide_Answer (Owner, Completed); begin Require (not Answer.Known, "safe null value"); end;',
        '''Thinkthen.Sessions.Calls.Decide (Owner, Feed); Cancel (Owner); declare use Thinkthen.Answers; Completed : constant Completion := Collect (Owner); Refused : Boolean := False; begin Require (Status (Completed) = Thinkthen.Answers.Failed and then Kind (Call_Failure (Completed)) = Cancelled, "retained cancellation"); begin declare Ignored : constant Decision := Decide_Answer (Completed); begin if Ignored.Known then null; end if; end; exception when Session_Failure => Refused := True; end; Require (Refused and then Message (Call_Failure (Completed))'Length > 0, "typed execution failure retained"); end;''',
    ]
    evidence = work / 'evidence.txt'
    evidence.write_text('File evidence.\n')
    red = work / 'red.png'
    shutil.copy2(ROOT / 'specification/fixtures/images/red.png', red)
    extras = {
        'File_Request': request({'kind':'text','text':'Does it pass?'}, {'kind':'source','source':{'paths':[str(evidence)],'reading':{'unit':'line'}}}),
        'Cached_Request': request({'kind':'text','text':'Does it pass?'}, {'kind':'text','text':'Cache evidence.'}),
        'Recorded_Request': request({'kind':'text','text':'Does it pass?'}, {'kind':'text','text':'Recorded evidence.'}),
        'Image_Request': request({'kind':'text','text':'Is red visible?'}, {'kind':'records','items':[{'images':[{'kind':'file','path':str(red)}, {'kind':'file','path':str(red)}]}]}),
    }
    declarations += [f'{name} : constant T_RequestCall := {expression};' for name, expression in extras.items()]
    calls += [
        'Thinkthen.Sessions.Calls.Decide (Client, Owner, File_Request); Finish (Owner); Drain (Located => True, Expected_Sends => 1);',
        'Select_Settings ("generic/v1", "' + json.dumps({'cache':str(work / 'answer-cache')})[1:-1].replace('"','""') + '");',
        'Thinkthen.Sessions.Calls.Decide (Client, Owner, Cached_Request); Finish (Owner); Drain (Expected_Sends => 1);',
        'Thinkthen.Sessions.Calls.Decide (Client, Owner, Cached_Request); Finish (Owner); Drain (Expected_Sends => 0, Cache_Hit => True);',
        'Select_Settings ("generic/v1", "' + json.dumps({'cache':False,'record':str(work / 'recording')})[1:-1].replace('"','""') + '");',
        'Thinkthen.Sessions.Calls.Decide (Client, Owner, Recorded_Request); Finish (Owner); Drain (Expected_Sends => 1);',
        'Select_Settings ("generic/v1", "' + json.dumps({'cache':False,'replay':str(work / 'recording')})[1:-1].replace('"','""') + '");',
        'Thinkthen.Sessions.Calls.Decide (Client, Owner, Recorded_Request); Finish (Owner); Drain (Expected_Sends => 0);',
        'Select_Settings ("arm/images/liquid/decide/v1", "' + json.dumps({'cache':False,'backend':'liquid','model':'d1'})[1:-1].replace('"','""') + '");',
        'Thinkthen.Sessions.Calls.Decide (Client, Owner, Image_Request); Finish (Owner); Drain (Expected_Sends => 1, Images => True);',
    ]
    source = ('with Ada.Strings.Fixed;\n' + (PACKAGE / 'checks/native_session.adb').read_text()).replace('-- DECLARATIONS', '\n'.join(declarations)).replace('-- CALLS', '\n'.join(calls))
    (work / 'native_session.adb').write_text(source)
    shutil.copy2(installed / 'examples/session_demo.adb', work / 'session_demo.adb')
    subprocess.run([sys.executable, str(PACKAGE / 'checks/session_abi.py'), str(installed / 'native/include/thinkthen.h'), str(installed / 'src/thinkthen_session_c.ads')], env=child_env(), check=True)
    (work / 'consumer.gpr').write_text('with "installed package/thinkthen.gpr";\nproject Consumer is\nfor Main use ("native_session.adb", "session_demo.adb");\nfor Object_Dir use "objects";\npackage Compiler is\nfor Default_Switches ("Ada") use ("-gnat2022");\nend Compiler;\nend Consumer;\n')
    build = subprocess.run(['gprbuild', '-p', '-P', 'consumer.gpr', '-j2', '-cargs', '-O0'], cwd=work, env=child_env(), capture_output=True, text=True, timeout=60)
    assert build.returncode == 0, build.stdout + build.stderr
    if os.environ.get('THINKTHEN_COVERAGE_STAGE'):
        stage = Path(os.environ['THINKTHEN_COVERAGE_STAGE']); stage.mkdir(parents=True, exist_ok=True)
        shutil.copy2(work / 'objects/native_session', stage / 'native_session')
    backend = subprocess.Popen([os.environ.get('THINKTHEN_BACKEND_BIN', str(ROOT / 'target/debug/conformance-backend'))], env=child_env(), stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
    try:
        port = int(backend.stdout.readline())
        for mode, route in (('', 'generic/v1'), ('failure', 'arm/status/401/v1')):
            env = child_env(home=work / ('home-' + mode), THINKTHEN_BASE_URL=f'http://127.0.0.1:{port}/{route}', TT_BACKEND_ROOT=f'http://127.0.0.1:{port}/', THINKTHEN_API_KEY='sk-conformance-loopback')
            command = [str(work / 'objects/native_session')] + ([mode] if mode else [])
            result = subprocess.run(command, cwd=work, env=env, text=True, capture_output=True, timeout=30)
            print(result.stdout)
            assert result.returncode == 0, result.stderr
            assert 'INSTALLED_ADA_NATIVE_SESSION_PASS' in result.stdout
        example_env = child_env(home=work / 'home-example', THINKTHEN_BASE_URL=f'http://127.0.0.1:{port}/generic/v1', THINKTHEN_API_KEY='sk-conformance-loopback')
        example = subprocess.run([str(work / 'objects/session_demo')], cwd=work, env=example_env, text=True, capture_output=True, timeout=30)
        assert example.returncode == 0, example.stdout + example.stderr
        assert example.stdout.strip() == 'TRUE', example.stdout
        print('Installed Ada safe answer example PASS')
        backend.stdin.write('count\n'); backend.stdin.flush()
        count = backend.stdout.readline().strip()
        assert count == '39', count
        print('native session loopback requests:', count)
        print('development package artifact:', artifact)
    finally:
        backend.stdin.close()
        backend.wait(timeout=10)
        diagnostics = backend.stderr.read()
        assert not diagnostics, diagnostics
    subprocess.run([sys.executable, str(ROOT / 'conformance/children/usage_installed.py'), '--family', 'ada', '--ada-package', str(installed), '--ada-objects', str(installed / 'obj')], env=child_env(THINKTHEN_COVERAGE_STAGE=os.environ.get('THINKTHEN_COVERAGE_STAGE', '')), check=True)
