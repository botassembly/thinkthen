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
    expected = {'thinkthen', 'thinkthen-sessions', 'thinkthen-sessions-calls', 'thinkthen-persistence', 'thinkthen-requests-native', 'thinkthen_kinds', 'thinkthen-values', 'thinkthen-answers', 'thinkthen-native_values', 'thinkthen-inputs', 'thinkthen-inputs-native', 'thinkthen-inputs-calls', 'thinkthen-native_sessions'}
    assert {p.stem for p in (installed / 'src').glob('*.adb')} == expected
    assert {p.stem for p in (installed / 'src').glob('*.ads')} == expected | {'thinkthen-session_c', 'thinkthen-requests', 'thinkthen-native_sessions'}
    if '--full' in sys.argv:
        subprocess.run([sys.executable, str(PACKAGE / 'checks/complete_parity.py'), '--package', str(installed)], env=child_env(THINKTHEN_BACKEND_BIN=os.environ.get('THINKTHEN_BACKEND_BIN', str(ROOT / 'target/debug/conformance-backend'))), check=True)
        raise SystemExit(0)
    declarations, calls = [], []
    for verb, case, expression in cases():
        declarations.append(f'R_{verb} : constant T_RequestCall := {expression};')
        calls.append(f'Select_Route ("case/{case}/v1"); Thinkthen.Native_Sessions.Start (Client, Owner, R_{verb}, "{verb}"); Finish (Owner); Drain{" (Odds => 0.99)" if verb == "decide" else ""}; Put_Line ("SHARED_CASE {case}");')
    answer_checks = {
        'decide': 'Require (As_Boolean (Item.Value) and then Item.Probability.State = Present_Field and then Item.Probability.Value = 0.99 and then Item.Results\'Length = 1 and then Requests_Sent (Item.Facts) = 1, "paired refund result");',
        'choose': 'Require (Item.Value.Known and then To_String (Item.Value.Value) = "billing" and then Item.Probability.State = Present_Field and then abs (Item.Probability.Value - 0.7) < 0.00001 and then Item.Results\'Length = 1 and then Requests_Sent (Item.Facts) = 1, "paired choose result");',
        'tag': '''Require (Item.Value'Length = 2 and then Item.Probability.State = Null_Field, "safe tags");''',
        'score': 'Require (Item.Value > 0.0 and then Item.Probability.State = Null_Field, "safe score");',
        'filter': 'Require (Item.Value\'Length = 2 and then As_String (Item.Value (1)) = "row 0" and then As_String (Item.Value (2)) = "row 2" and then Item.Probability\'Length = 2, "plain filter originals");',
        'rank': '''Require (Item.Value'Length = 3 and then Item.Value = Rank_List'[1, 2, 0] and then Item.Probability'Length = 3, "plain original rank indexes");''',
        'find': 'Require (Kind (Item.Value) /= Thinkthen.Values.Null_Value and then Item.Probability.State = Present_Field, "safe find");',
        'annotate': 'Require (Item.Value\'Length > 0 and then Item.Probability.State = Null_Field, "safe annotations");',
        'recognize': 'Require (Length (Field_Value (Item.Value, "entities")) > 0 and then Natural (Item.Probability.Length) > 0 and then Item.Probability (1).Value > 0.99 and then abs (Item.Probability (1).Value - 0.9877) > 0.001, "kept selected span probability");',
        'relate': 'Require (Length (Item.Value) > 0 and then Source_Name (Element (Item.Value, 1))\'Length > 0 and then Target_Name (Element (Item.Value, 1))\'Length > 0 and then Natural (Item.Probability.Length) = Length (Item.Value), "direct relation names");',
    }
    for verb, case, _ in cases():
        question = f'Thinkthen.Inputs.Native.Wrap (R_{verb}.T_question)'
        if verb in ('recognize','relate'):
            qi = next(i for i,(key,_) in enumerate(definitions['RequestQuestion']['variants']) if key == 'RequestQuestion_definition')
            di = next(i for i,(key,_) in enumerate(definitions['RequestDefinition']['variants']) if verb in definitions[key].get('properties',{}))
            question = f'Thinkthen.Inputs.{verb.title()}_Question (Thinkthen.Inputs.Native.Wrap (R_{verb}.T_question.V_{qi}.T_value.V_{di}))'
        original = f'Thinkthen.Inputs.Native.Wrap (R_{verb}.T_input)'
        if verb not in ('filter', 'rank', 'find', 'annotate', 'relate'):
            original = f'Thinkthen.Inputs.As_text ({original})'
        target = verb.title() + ('_List_Result' if verb == 'annotate' else '_Result')
        calls.append(f'Select_Route ("case/{case}/v1"); declare use Thinkthen.Answers; use Thinkthen.Values; Item : constant {target} := Thinkthen.Inputs.Calls.{verb.title()} (Client, {question}, {original}, Thinkthen.Inputs.Native.Wrap (R_{verb}.T_options)); begin Require (Item.Results\'Length > 0 and then Requests_Sent (Item.Facts) > 0, "owned call result"); {answer_checks[verb]} end;')
    # Planning crosses the public typed API and preserves the native zero-send refusals.
    for verb, _, _ in cases():
        preview = f'Thinkthen.Inputs.Calls.Plan (Client, Thinkthen.Inputs.Function_Kind_{verb.title()}, Thinkthen.Inputs.Native.Wrap (R_{verb}.T_question), Thinkthen.Inputs.Native.Wrap (R_{verb}.T_input), Thinkthen.Inputs.Native.Wrap (R_{verb}.T_options))'
        if verb in ('decide', 'choose', 'tag', 'score', 'filter', 'rank'):
            calls.append('declare use Thinkthen.Inputs.Calls; Item : constant Plan_Estimate := ' + preview + '; begin Require (Records (Item) > 0 and then Thinkthen.Inputs.Calls.Requests (Item) > 0 and then Estimated_Bytes (Item) > 0 and then Largest_Request_Bytes (Item) > 0 and then Largest_Request_Estimated_Input_Tokens (Item) > 0 and then Token_Estimate_Method (Item)\'Length > 0 and then Estimated_Input_Tokens (Item).Upper >= Estimated_Input_Tokens (Item).Lower and then Upper_Bound (Item) = ' + ('True' if verb == 'tag' else 'False') + ' and then First_Body_Utf8 (Item).Present, "nine typed plan fields"); end;')
        else:
            calls.append('declare Refused : Boolean := False; begin begin declare Item : constant Thinkthen.Inputs.Calls.Plan_Estimate := ' + preview + '; begin if Thinkthen.Inputs.Calls.Records (Item) > 0 then null; end if; end; exception when Usage_Error => Refused := True; end; Require (Refused, "native unsupported plan refusal"); end;')
    # Admission, cancellation and execution failures add no host-side semantic rules.
    key = 'RequestCall'
    def request(question, original):
        return construct(key, definitions[key], {'function':'decide', 'question':question, 'input':original, 'options':{}})
    failure = request({'kind':'text','text':'Does it pass?'}, {'kind':'text','text':'failure-one'})
    feed = request({'kind':'text','text':'Does it pass?'}, {'kind':'feed','name':'owned'})
    invalid = request({'kind':'text','text':''}, {'kind':'text','text':'invalid'})
    false_request = request({'kind':'definition','value':{'decide':'Does it pass?', 'threshold':0.95}}, {'kind':'text','text':'False evidence.'})
    null_request = request({'kind':'definition','value':{'decide':'Does it pass?', 'threshold':'0.7:0.95'}}, {'kind':'text','text':'Null evidence.'})
    authored_null = request({'kind':'definition','value':{'decide':'Does it pass?', 'true':None}}, {'kind':'text','text':'Authored null evidence.'})
    authored_object = request({'kind':'definition','value':{'decide':'Does it pass?', 'threshold':0.95, 'false':{'wide':18446744073709551615, 'list':[False,None], 'null':None}}}, {'kind':'text','text':'Authored object evidence.'})
    declarations += [f'Authored_Null : constant T_RequestCall := {authored_null};', f'Authored_Object : constant T_RequestCall := {authored_object};']
    item = construct('RequestSessionDescriptor', definitions['RequestSessionDescriptor'], {'item':{'original':{'kind':'text','text':'Located evidence.'}}, 'location':{'file':'owned.txt','first_line':1,'last_line':1}})
    declarations += [f'False_Request : constant T_RequestCall := {false_request};', f'Null_Request : constant T_RequestCall := {null_request};', f'Item : T_RequestSessionDescriptor := {item};', 'Pushed : Push_Status;', f'Failed : constant T_RequestCall := {failure};', f'Feed : constant T_RequestCall := {feed};', f'Invalid : constant T_RequestCall := {invalid};']
    calls += ['Select_Route ("generic/v1");', 'declare Preview : constant Thinkthen.Values.Value := Thinkthen.Native_Sessions.Plan (Client, False_Request, "decide"); begin Require (Thinkthen.Values.As_Number (Thinkthen.Values.Field_Value (Preview, "records")) = "1", "typed plan result"); end;', 'Thinkthen.Native_Sessions.Start (Owner, False_Request, "decide"); Finish (Owner); Drain (Decision => 0);', 'Thinkthen.Native_Sessions.Start (Owner, Null_Request, "decide"); Finish (Owner); Drain (Decision => 1);', 'Thinkthen.Native_Sessions.Start (Owner, Feed, "decide"); Push (Owner, Item, Pushed); if Pushed /= Accepted then raise Program_Error with "feed refused"; end if; Finish (Owner); Drain (Located => True);', 'Thinkthen.Native_Sessions.Start (Owner, Feed, "decide"); Cancel (Owner); Drain (Cancelled);', '''declare
Refused : Boolean := False;
begin
begin
declare Preview : constant Thinkthen.Values.Value := Thinkthen.Native_Sessions.Plan (Client, Invalid, "decide"); begin Require (Thinkthen.Values.Length (Preview) = 0, "invalid plan admitted"); end;
exception
when Usage_Error => Refused := True;
end;
if not Refused then raise Program_Error with "invalid plan admitted"; end if;
Refused := False;
begin Thinkthen.Native_Sessions.Start (Owner, Invalid, "decide"); exception when Usage_Error => Refused := True; end;
if not Refused then raise Program_Error with "invalid question admitted"; end if;
end;''']
    calls += [
        'Thinkthen.Native_Sessions.Start (Owner, False_Request, "decide"); Finish (Owner); declare Completed : constant Thinkthen.Answers.Completion := Thinkthen.Answers.Collect (Owner); Answer : constant Thinkthen.Answers.Decide_Result := Thinkthen.Answers.Decide (Completed); begin Require (not Thinkthen.Values.As_Boolean (Answer.Value) and then Answer.Probability.State = Thinkthen.Answers.Present_Field and then Answer.Probability.Value = 0.9, "safe false value"); Require (Thinkthen.Answers.Requests_Sent (Thinkthen.Answers.Call_Facts (Completed)) = 0 and then Thinkthen.Answers.Cache_Answers (Thinkthen.Answers.Call_Facts (Completed)) = 1, "safe cached facts"); declare Refused : Boolean := False; begin begin declare Ignored : constant Thinkthen.Answers.Choose_Result := Thinkthen.Answers.Choose (Completed); begin if Ignored.Results\'Length > 0 then null; end if; end; exception when Thinkthen.Answers.Answer_Cardinality_Error => Refused := True; end; Require (Refused and then Thinkthen.Answers.Has_Facts (Completed) and then Thinkthen.Answers.Packet_Count (Completed) > 0, "cardinality refusal retained completion"); end; end;',
        'Thinkthen.Native_Sessions.Start (Owner, Null_Request, "decide"); Finish (Owner); declare Completed : constant Thinkthen.Answers.Completion := Thinkthen.Answers.Collect (Owner); Answer : constant Thinkthen.Answers.Decide_Result := Thinkthen.Answers.Decide (Completed); begin Require (Thinkthen.Values.Kind (Answer.Value) = Thinkthen.Values.Null_Value and then Answer.Probability.State = Thinkthen.Answers.Null_Field, "safe null value"); end;',
        '''Thinkthen.Native_Sessions.Start (Owner, Feed, "decide"); Cancel (Owner); declare use Thinkthen.Answers; Completed : constant Completion := Collect (Owner); Refused : Boolean := False; begin Require (Status (Completed) = Thinkthen.Answers.Failed and then Kind (Call_Failure (Completed)) = Cancelled, "retained cancellation"); begin declare Ignored : constant Decide_Result := Thinkthen.Answers.Decide (Completed); begin if Ignored.Results'Length > 0 then null; end if; end; exception when Session_Failure => Refused := True; end; Require (Refused and then Message (Call_Failure (Completed))'Length > 0, "typed execution failure retained"); end;''',
    ]
    calls += [
        'Select_Route ("generic/v1");',
        '''declare use Thinkthen.Answers; use Thinkthen.Values;
Answer : constant Decide_Result := Thinkthen.Inputs.Calls.Decide (Client, Thinkthen.Inputs.Native.Wrap (Authored_Null.T_question), Thinkthen.Inputs.As_text (Thinkthen.Inputs.Native.Wrap (Authored_Null.T_input)));
begin Require (Kind (Answer.Value) = Thinkthen.Values.Null_Value and then Answer.Probability.State = Present_Field and then Answer.Probability.Value = 0.9, "resolved authored null retains probability"); end;''',
        '''declare use Thinkthen.Answers; use Thinkthen.Values;
Answer : constant Decide_Result := Thinkthen.Inputs.Calls.Decide (Client, Thinkthen.Inputs.Native.Wrap (Authored_Object.T_question), Thinkthen.Inputs.As_text (Thinkthen.Inputs.Native.Wrap (Authored_Object.T_input)));
begin Require (Kind (Answer.Value) = Object_Value and then Field_Name (Answer.Value, 1) = "wide" and then As_Number (Field_Value (Answer.Value, "wide")) = "18446744073709551615" and then not As_Boolean (Element (Field_Value (Answer.Value, "list"), 1)) and then Kind (Element (Field_Value (Answer.Value, "list"), 2)) = Thinkthen.Values.Null_Value and then Kind (Field_Value (Answer.Value, "null")) = Thinkthen.Values.Null_Value and then Answer.Probability.State = Present_Field and then Answer.Probability.Value = 0.9, "resolved authored object preserves leaf and order"); end;''',
        '''declare use Thinkthen.Answers; use Thinkthen.Values;
Answer : constant Decide_List_Result := Thinkthen.Inputs.Calls.Decide (Client, Thinkthen.Inputs.Text_Question ("Does it pass?"), Thinkthen.Inputs.Make_Input_records ([Thinkthen.Inputs.Text_Item ("A one-record list.")]));
begin Require (Answer.Value'Length = 1 and then As_Boolean (Answer.Value (1)) and then Answer.Probability'Length = 1 and then Answer.Probability (1).State = Present_Field and then Answer.Probability (1).Value = 0.9, "concrete one-record list retains list shape"); end;''',
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
        'Thinkthen.Native_Sessions.Start (Client, Owner, File_Request, "decide"); Finish (Owner); Drain (Located => True, Expected_Sends => 1);',
        'Select_Settings ("generic/v1", ' + 'Thinkthen.Inputs.Native.Wrap (' + construct('EngineSettings', definitions['EngineSettings'], {'cache':str(work / 'answer-cache')}) + ')' + ');',
        'Thinkthen.Native_Sessions.Start (Client, Owner, Cached_Request, "decide"); Finish (Owner); Drain (Expected_Sends => 1);',
        'Thinkthen.Native_Sessions.Start (Client, Owner, Cached_Request, "decide"); Finish (Owner); Drain (Expected_Sends => 0, Cache_Hit => True);',
        'Select_Settings ("generic/v1", ' + 'Thinkthen.Inputs.Native.Wrap (' + construct('EngineSettings', definitions['EngineSettings'], {'cache':False,'record':str(work / 'recording')}) + ')' + ');',
        'Thinkthen.Native_Sessions.Start (Client, Owner, Recorded_Request, "decide"); Finish (Owner); Drain (Expected_Sends => 1);',
        'Select_Settings ("generic/v1", ' + 'Thinkthen.Inputs.Native.Wrap (' + construct('EngineSettings', definitions['EngineSettings'], {'cache':False,'replay':str(work / 'recording')}) + ')' + ');',
        'Thinkthen.Native_Sessions.Start (Client, Owner, Recorded_Request, "decide"); Finish (Owner); Drain (Expected_Sends => 0);',
        'Select_Settings ("arm/images/liquid/decide/v1", ' + 'Thinkthen.Inputs.Native.Wrap (' + construct('EngineSettings', definitions['EngineSettings'], {'cache':False,'backend':'liquid','model':'d1'}) + ')' + ');',
        'Thinkthen.Native_Sessions.Start (Client, Owner, Image_Request, "decide"); Finish (Owner); Drain (Expected_Sends => 1, Images => True);',
    ]
    source = ('with Thinkthen.Native_Sessions; use Thinkthen.Native_Sessions;\nwith Thinkthen.Inputs.Calls;\nwith Thinkthen.Inputs;\nwith Thinkthen.Inputs.Native;\nwith Ada.Strings.Fixed;\nwith Thinkthen.Answers;\nwith Thinkthen.Values;\nuse type Thinkthen.Answers.Field_State;\nuse type Thinkthen.Values.Value_Kind;\n' + (PACKAGE / 'checks/native_session.adb').read_text()).replace('-- DECLARATIONS', '\n'.join(declarations)).replace('-- CALLS', '\n'.join(calls))
    source = __import__('re').sub(r'(?<![.\w])(?:Close|Finish|Cancel|Next_Packet|Try_Read)\s*\(', lambda match: 'Thinkthen.Native_Sessions.' + match.group(0), source)
    source = __import__('re').sub(r'\bText\s*\(', 'Thinkthen.Native_Sessions.Text (', source)
    (work / 'thinkthen-inputs-native_check.ads').write_text('procedure Thinkthen.Inputs.Native_Check;\n')
    source = source.replace('procedure Native_Session is', 'procedure Thinkthen.Inputs.Native_Check is').replace('end Native_Session;', 'end Thinkthen.Inputs.Native_Check;')
    (work / 'thinkthen-inputs-native_check.adb').write_text(source)
    shutil.copy2(installed / 'examples/session_demo.adb', work / 'session_demo.adb')
    subprocess.run([sys.executable, str(PACKAGE / 'checks/session_abi.py'), str(installed / 'native/include/thinkthen.h'), str(installed / 'src/thinkthen-session_c.ads')], env=child_env(), check=True)
    (work / 'consumer.gpr').write_text('with "installed package/thinkthen.gpr";\nproject Consumer is\nfor Main use ("thinkthen-inputs-native_check.adb", "session_demo.adb");\nfor Object_Dir use "objects";\npackage Compiler is\nfor Default_Switches ("Ada") use ("-gnat2022");\nend Compiler;\nend Consumer;\n')
    build = subprocess.run(['gprbuild', '-p', '-P', 'consumer.gpr', '-j2', '-cargs', '-O0'], cwd=work, env=child_env(), capture_output=True, text=True, timeout=60)
    assert build.returncode == 0, build.stdout + build.stderr
    if os.environ.get('THINKTHEN_COVERAGE_STAGE'):
        stage = Path(os.environ['THINKTHEN_COVERAGE_STAGE']); stage.mkdir(parents=True, exist_ok=True)
        shutil.copy2(work / 'objects/thinkthen-inputs-native_check', stage / 'native_session')
    for unit in ('Thinkthen.Requests', 'Thinkthen.Session_C', 'Thinkthen.Native_Sessions'):
        (work / 'privacy_probe.adb').write_text('with ' + unit + ';\nprocedure Privacy_Probe is begin null; end Privacy_Probe;\n')
        denied = subprocess.run(['gnatmake', '-q', '-gnat2022', '-gnatc', '-I' + str(installed / 'src'), 'privacy_probe.adb', '-D', str(work / 'objects')], cwd=work, env=child_env(), capture_output=True, text=True)
        assert denied.returncode != 0 and 'private' in denied.stderr, denied.stdout + denied.stderr
    print('Installed Ada private carrier and C package refusals PASS')
    backend = subprocess.Popen([os.environ.get('THINKTHEN_BACKEND_BIN', str(ROOT / 'target/debug/conformance-backend'))], env=child_env(), stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
    try:
        port = int(backend.stdout.readline())
        for mode, route in (('', 'generic/v1'), ('failure', 'arm/status/401/v1')):
            env = child_env(home=work / ('home-' + mode), THINKTHEN_BASE_URL=f'http://127.0.0.1:{port}/{route}', TT_BACKEND_ROOT=f'http://127.0.0.1:{port}/', THINKTHEN_API_KEY='sk-conformance-loopback')
            command = [str(work / 'objects/thinkthen-inputs-native_check')] + ([mode] if mode else [])
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
        assert count == '42', count
        print('native session loopback requests:', count)
        print('development package artifact:', artifact)
    finally:
        backend.stdin.close()
        backend.wait(timeout=10)
        diagnostics = backend.stderr.read()
        assert not diagnostics, diagnostics
    subprocess.run([sys.executable, str(ROOT / 'conformance/children/usage_installed.py'), '--family', 'ada', '--ada-package', str(installed), '--ada-objects', str(installed / 'obj')], env=child_env(THINKTHEN_COVERAGE_STAGE=os.environ.get('THINKTHEN_COVERAGE_STAGE', '')), check=True)
