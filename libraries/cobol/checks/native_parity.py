"""Run shared fixtures through installed generated COBOL session records."""
import ctypes.util
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
from session_records import ROOT, module, definitions
from templates.csharp import enum_values

sys.path.insert(0, str(ROOT / 'conformance/children'))
from children import child_env, read_process_line
shared = module('shared_session_cases', ROOT / 'conformance/children/session_cases.py')
PACKAGE = Path(os.environ.get('THINKTHEN_PARITY_PACKAGE', ROOT / 'libraries/cobol')).resolve(strict=True)
HEADER = Path(os.environ.get('THINKTHEN_C_HEADER', ROOT / 'libraries/c/include/thinkthen.h')).resolve(strict=True)
LIBRARY = Path(os.environ.get('THINKTHEN_C_LIBRARY', ROOT / 'libraries/c/target/debug/libthinkthen_c.so')).resolve(strict=True)

with tempfile.TemporaryDirectory(prefix='thinkthen-cobol-session-parity-') as temporary:
    work = Path(temporary)
    installed = work / 'installed'
    shutil.copytree(PACKAGE / 'src', installed / 'src')
    shutil.copytree(PACKAGE / 'copybooks', installed / 'copybooks')
    shutil.copy(HEADER, installed / 'src/thinkthen.h')
    native = work / 'libthinkthen.so.0'; shutil.copy(LIBRARY, native)
    programs = []
    for verb in enum_values(definitions['RequestFunction']):
        programs.append(f'''identification division.
program-id. COBOL_{verb.upper()}.
data division.
working-storage section.
copy "tt-native-generated.cpy".
linkage section.
01 engine usage pointer.
01 input-record usage pointer.
01 session-owner usage pointer.
procedure division using engine input-record session-owner.
 set address of tt-n-cobol-RequestCall to input-record
 call "TT_SESSION_{verb.upper()}" using by value engine
    by reference tt-n-cobol-RequestCall session-owner
    returning return-code
 goback.
end program COBOL_{verb.upper()}.
''')
    for verb in enum_values(definitions['RequestFunction']):
        programs.append(f'''identification division.
program-id. COBOL_BLOCK_{verb.upper()}.
data division.
working-storage section.
copy "tt-native-generated.cpy".
linkage section.
01 engine usage pointer.
01 question usage pointer.
01 input-record usage pointer.
01 option-record usage pointer.
01 answer usage pointer.
01 status-record usage pointer.
procedure division using engine question input-record option-record answer status-record.
 set address of tt-n-cobol-Answer{verb.title()}Result to answer
 set address of tt-n-cobol-AnswerStatus to status-record
 call "TT_{verb.upper()}" using by value engine question input-record option-record
    by reference tt-n-cobol-Answer{verb.title()}Result tt-n-cobol-AnswerStatus
    returning return-code
 goback.
end program COBOL_BLOCK_{verb.upper()}.
''')
    programs.append('''identification division.
program-id. COBOL_PUSH.
data division.
working-storage section.
copy "tt-native-generated.cpy".
linkage section.
01 session-owner usage pointer.
01 input-record usage pointer.
01 push-status binary-long unsigned.
procedure division using session-owner input-record push-status.
 set address of tt-n-cobol-RequestSessionDescriptor to input-record
 call "TT_SESSION_PUSH" using by value session-owner
    by reference tt-n-cobol-RequestSessionDescriptor push-status
    returning return-code
 goback.
end program COBOL_PUSH.
''')
    source = work / 'consumer.cob'; source.write_text('\n'.join(programs))
    bridge = work / 'consumer.so'
    subprocess.run(['cobc','-b','-free','-fstatic-call','-fno-gen-c-decl-static-call',
                    '-I',str(installed / 'copybooks'),'-A','-include '+str(installed / 'src/tt_session.h')+' -Wno-incompatible-pointer-types',
                    '-o',str(bridge),str(source),str(installed / 'src/tt_session.c'),
                    str(installed / 'src/tt_requests_generated.c'),'-Q',str(native)+' -Wl,-rpath,'+str(work)],
                   env=child_env(LC_ALL='C.UTF-8'),check=True,timeout=60)
    def invoke(step, settings, env, home, backend):
        fixture = shared.descriptor(step,home)
        for item in fixture['input'].get('items', []):
            if item.get('images'):
                item['images'] = [{'kind':'file', 'path':str(ROOT / path),
                    'media':step.get('media', 'image/png')} for path in step['image_paths']]
        if step.get('incremental') and fixture['input']['kind'] != 'source':
            fixture['feed_items'] = fixture['input']['items']
            fixture['input'] = {'kind':'feed','name':'records'}
        given = home / 'session-request.json'; given.write_text(shared.shared.compact(fixture))
        command = [sys.executable,str(ROOT / 'libraries/cobol/checks/session_consumer.py'),str(given),shared.shared.compact(settings)]
        child = dict(env, TT_SESSION_BRIDGE=str(bridge), TT_SESSION_NATIVE=str(native), TT_SESSION_PACKAGE=str(installed))
        if not step.get('held_cancel'):
            before = int(backend.read('count')) if fixture.get('call_result_contract') else None
            result = subprocess.run(command,cwd=home,env=child,capture_output=True,text=True,timeout=30)
            if before is not None:
                assert int(backend.read('count')) - before == 1, 'canonical COBOL call must send once'
            return result
        process = subprocess.Popen(command,cwd=home,env=child,stdin=subprocess.PIPE,stdout=subprocess.PIPE,stderr=subprocess.PIPE,text=True)
        try:
            assert backend.read('wait 1') == 'wait 1'
            process.stdin.write('!'); process.stdin.flush()
            assert read_process_line(process.stdout) == 'cancel-fired\n'
            backend.process.stdin.write('release\n'); backend.process.stdin.flush()
            stdout,stderr = process.communicate(timeout=30)
            return subprocess.CompletedProcess(command,process.returncode,stdout,stderr)
        finally:
            backend.process.stdin.write('release\n'); backend.process.stdin.flush()
            if process.poll() is None: process.kill(); process.wait()
    shared.native_cases(Path(sys.executable), consumer='cobol', invoke=invoke)
