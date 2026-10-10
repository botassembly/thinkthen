"""Run shared fixtures through installed generated COBOL session records."""
import ctypes.util
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
from session_records import ROOT, module

sys.path.insert(0, str(ROOT / 'conformance/children'))
from children import child_env
shared = module('shared_session_cases', ROOT / 'libraries/cpp/fixtures/session_cases.py')
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
    for verb in shared.shared.FUNCTIONS:
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
 set address of tt-n-cobol-RequestCall-{verb} to input-record
 call "TT_SESSION_{verb.upper()}" using by value engine
    by reference tt-n-cobol-RequestCall-{verb} session-owner
    returning return-code
 goback.
end program COBOL_{verb.upper()}.
''')
    source = work / 'consumer.cob'; source.write_text('\n'.join(programs))
    bridge = work / 'consumer.so'
    subprocess.run(['cobc','-b','-free','-fstatic-call','-fno-gen-c-decl-static-call',
                    '-I',str(installed / 'copybooks'),'-A','-include '+str(installed / 'src/tt_session.h')+' -Wno-incompatible-pointer-types',
                    '-o',str(bridge),str(source),str(installed / 'src/tt_session.c'),
                    str(installed / 'src/tt_requests_generated.c'),'-Q',str(native)+' -Wl,-rpath,'+str(work)],
                   env=child_env(LC_ALL='C.UTF-8'),check=True,timeout=60)
    def invoke(step, settings, env, home, backend):
        given = home / 'session-request.json'; given.write_text(shared.shared.compact(shared.descriptor(step,home)))
        command = [sys.executable,str(ROOT / 'libraries/cobol/checks/session_consumer.py'),str(given),shared.shared.compact(settings)]
        child = dict(env, TT_SESSION_BRIDGE=str(bridge), TT_SESSION_NATIVE=str(native), TT_SESSION_PACKAGE=str(installed))
        if not step.get('held_cancel'):
            return subprocess.run(command,cwd=home,env=child,capture_output=True,text=True,timeout=30)
        process = subprocess.Popen(command,cwd=home,env=child,stdin=subprocess.PIPE,stdout=subprocess.PIPE,stderr=subprocess.PIPE,text=True)
        try:
            assert backend.read('wait 1') == 'wait 1'
            process.stdin.write('!'); process.stdin.flush()
            assert process.stdout.readline() == 'cancel-fired\n'
            backend.process.stdin.write('release\n'); backend.process.stdin.flush()
            stdout,stderr = process.communicate(timeout=30)
            return subprocess.CompletedProcess(command,process.returncode,stdout,stderr)
        finally:
            backend.process.stdin.write('release\n'); backend.process.stdin.flush()
            if process.poll() is None: process.kill(); process.wait()
    shared.native_cases(Path(sys.executable), consumer='cobol', invoke=invoke)
