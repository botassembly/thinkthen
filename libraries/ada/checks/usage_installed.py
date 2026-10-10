"""Exercise installed Ada and COBOL persistence with real owned usage locks."""
import argparse
import fcntl
import importlib.util
import json
import os
from pathlib import Path
import selectors
import shutil
import subprocess
import sys
import tempfile

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / 'conformance/children'))
from children import child_env

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--family', choices=('ada', 'cobol'))
args = parser.parse_args()
families = (args.family,) if args.family else ('ada', 'cobol')

NATIVE = Path(os.environ.get('THINKTHEN_C_LIBRARY', ROOT / 'libraries/c/target/debug/libthinkthen_c.so'))


def cobol_source(package):
    source = (package / 'examples/session.cob').read_text()
    source = source.replace('01 engine usage pointer.', '''01 engine usage pointer.
01 mode-text pic x(8).
01 continue-text pic x(8).
01 saved-kind usage binary-long unsigned.
01 settings-text pic x(64) value z'{"cache":false,"max_retries":0}'.
01 diagnostic usage pointer.
01 diagnostic-text pic x(8192).
01 saved-advice pic x(8192).
01 saved-length usage binary-double unsigned.''')
    source = source.replace('procedure division.', '''procedure division.
    accept mode-text from environment "TT_USAGE_MODE"
    allocate tt-n-complete-usage-persistence-v1
    if mode-text = "bounds"
       allocate tt-n-complete-utf8-v1
       move all "x" to evidence-text
       set v-data of tt-n-complete-utf8-v1 to address of evidence-text
       move 8193 to v-len of tt-n-complete-utf8-v1
       move all "z" to output-text move 37 to written
       call "TT_SESSION_TEXT" using by reference tt-n-complete-utf8-v1
          output-text written returning status-code
       if status-code not = tt-n-cobol-overflow-constant or written not = 37
          or output-text not = all "z" move 41 to return-code goback end-if
       move 8192 to v-len of tt-n-complete-utf8-v1
       call "TT_SESSION_TEXT" using by reference tt-n-complete-utf8-v1
          output-text written returning status-code
       if status-code not = 0 or written not = 8192 or output-text not = all "x"
          move 42 to return-code goback end-if
       free tt-n-complete-utf8-v1 tt-n-complete-usage-persistence-v1
       display "BOUNDS_PASS" move 0 to return-code goback
    end-if''')
    source = source.replace('call "thinkthen_engine_new" returning engine',
                            'call "thinkthen_engine_new_with" using by reference settings-text returning engine')
    source = source.replace('    allocate tt-n-cobol-RequestCall-decide', '''    call "TT_ENGINE_USAGE_PERSISTENCE" using by value engine
       by reference tt-n-complete-usage-persistence-v1 saved-advice saved-length
       returning status-code
    if status-code not = 0 move 30 to return-code goback end-if
    if mode-text = "disabled"
       if v-kind of tt-n-complete-usage-persistence-v1 not =
          tt-n-complete-usage-persistence-disabled-v1-constant
          move 31 to return-code goback end-if
       call "TT_ENGINE_FINISH_USAGE_STATUS" using by value engine
          by reference tt-n-complete-usage-persistence-v1 saved-advice saved-length
          returning status-code
       if status-code not = 0 or saved-length not = 0 or
          v-kind of tt-n-complete-usage-persistence-v1 not =
          tt-n-complete-usage-persistence-disabled-v1-constant
          move 32 to return-code goback end-if
       call "thinkthen_engine_free" using by value engine returning omitted
       free tt-n-complete-usage-persistence-v1
       display "DISABLED_PASS" move 0 to return-code goback
    end-if
    if v-kind of tt-n-complete-usage-persistence-v1 not =
       tt-n-complete-usage-persistence-written-v1-constant
       move 33 to return-code goback end-if
    allocate tt-n-cobol-RequestCall-decide''', 1)
    close = '    call "thinkthen_engine_free" using by value engine returning omitted\n    if terminal-owner'
    source = source.replace(close, '''    call "TT_ENGINE_USAGE_PERSISTENCE" using by value engine
       by reference tt-n-complete-usage-persistence-v1 saved-advice saved-length
       returning status-code
    if status-code not = 0 or saved-length not = 0 or
       v-kind of tt-n-complete-usage-persistence-v1 not =
       tt-n-complete-usage-persistence-pending-v1-constant
       move 34 to return-code goback end-if
    display "PENDING"
    accept continue-text
    call "TT_ENGINE_FINISH_USAGE_STATUS" using by value engine
       by reference tt-n-complete-usage-persistence-v1 saved-advice saved-length
       returning status-code
    if status-code not = 0 move 35 to return-code goback end-if
    move v-kind of tt-n-complete-usage-persistence-v1 to saved-kind
    if mode-text = "written"
       if saved-kind not = tt-n-complete-usage-persistence-written-v1-constant
          or saved-length not = 0 move 36 to return-code goback end-if
    else
       if saved-kind not = tt-n-complete-usage-persistence-failed-v1-constant
          or saved-advice not = "check the usage folder permissions and free space"
          move 37 to return-code goback end-if
    end-if
    call "TT_ENGINE_USAGE_PERSISTENCE" using by value engine
       by reference tt-n-complete-usage-persistence-v1 output-text written
       returning status-code
    if status-code not = 0 or saved-kind not =
       v-kind of tt-n-complete-usage-persistence-v1 or
       output-text not = saved-advice or written not = saved-length
       move 38 to return-code goback end-if
    call "thinkthen_engine_free" using by value engine returning omitted
    set engine to null
    call "TT_ENGINE_USAGE_PERSISTENCE" using by value engine
       by reference tt-n-complete-usage-persistence-v1 output-text written
       returning status-code
    if status-code not = tt-n-eusage-constant or saved-kind not =
       v-kind of tt-n-complete-usage-persistence-v1 or output-text not = saved-advice
       or written not = saved-length move 39 to return-code goback end-if
    call "TT_ENGINE_FINISH_USAGE_STATUS" using by value engine
       by reference tt-n-complete-usage-persistence-v1 output-text written
       returning status-code
    if status-code not = tt-n-eusage-constant move 40 to return-code goback end-if
    allocate tt-n-complete-utf8-v1
    call "thinkthen_session_error_message" returning diagnostic
    set v-data of tt-n-complete-utf8-v1 to diagnostic
    call "strlen" using by value diagnostic returning
       v-len of tt-n-complete-utf8-v1
    call "TT_SESSION_TEXT" using by reference tt-n-complete-utf8-v1
       diagnostic-text written returning status-code
    if status-code not = 0 or diagnostic-text not =
       "invalid session arguments or input" move 43 to return-code goback end-if
    free tt-n-complete-utf8-v1
    display "OWNED_ADVICE " function trim(saved-advice trailing)
    free tt-n-complete-usage-persistence-v1
    if terminal-owner''')
    return source


with tempfile.TemporaryDirectory(prefix='usage-installed-') as temporary:
    work = Path(temporary)
    (work / 'OWNER').write_text('0468 Ada and COBOL installed persistence consumer\n')
    for family in families:
        package = Path(os.environ.get('THINKTHEN_USAGE_COBOL_PACKAGE', ROOT / 'libraries/cobol')) if family == 'cobol' else ROOT / 'libraries' / family
        shutil.copytree(package / 'src', work / family / 'src')
    if 'cobol' in families:
        package = Path(os.environ.get('THINKTHEN_USAGE_COBOL_PACKAGE', ROOT / 'libraries/cobol'))
        shutil.copytree(package / 'copybooks', work / 'cobol/copybooks')
        shutil.copytree(package / 'examples', work / 'cobol/examples')
        shutil.copy2(Path(os.environ.get('THINKTHEN_C_HEADER', ROOT / 'libraries/c/include/thinkthen.h')), work / 'cobol/src/thinkthen.h')
    shutil.copy2(NATIVE, work / 'libthinkthen.so.0')
    (work / 'libthinkthen.so').symlink_to('libthinkthen.so.0')
    if 'ada' in families:
        shutil.copy2(ROOT / 'libraries/ada/checks/usage_status.adb', work / 'usage_status.adb')
        (work / 'ada-objects').mkdir()
        build = subprocess.run(['gnatmake', '-q', '-gnat2022', '-I' + str(work / 'ada/src'),
                                str(work / 'usage_status.adb'), '-D', str(work / 'ada-objects'),
                                '-o', str(work / 'ada-consumer'), '-largs', '-L' + str(work),
                                '-lthinkthen', '-Wl,-rpath,$ORIGIN'], cwd=work, env=child_env(), capture_output=True, text=True, timeout=60)
        assert build.returncode == 0, build.stdout + build.stderr
    if 'cobol' in families:
        (work / 'usage_status.cob').write_text(cobol_source(work / 'cobol'))
        subprocess.run(['cobc', '-x', '-free', '-fstatic-call', '-fno-gen-c-decl-static-call',
                        '-I', str(work / 'cobol/copybooks'), '-A', '-include ' + str(work / 'cobol/src/tt_session.h') + ' -Wno-incompatible-pointer-types',
                        '-o', str(work / 'cobol-consumer'), str(work / 'usage_status.cob'),
                        str(work / 'cobol/src/tt_session.c'), str(work / 'cobol/src/tt_requests_generated.c'),
                        '-Q', str(work / 'libthinkthen.so.0') + ' -Wl,-rpath,$ORIGIN'], env=child_env(), check=True, timeout=30)
    spec = importlib.util.spec_from_file_location('usage_backend', ROOT / 'libraries/csharp/tests/backend.py')
    backend = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(backend)
    server = backend.Backend(work)
    expected = []
    try:
        if 'cobol' in families:
            bounds = subprocess.run([str(work / 'cobol-consumer')],
                                    env=child_env(home=work / 'bounds-home', TT_USAGE_MODE='bounds'),
                                    capture_output=True, text=True, timeout=5)
            assert bounds.returncode == 0 and 'BOUNDS_PASS' in bounds.stdout, (bounds.stdout, bounds.stderr)
        question = work / 'question.json'
        question.write_text(json.dumps({'decide': 'Is it?'}))
        for family in families:
            for mode in ('written', 'failed', 'disabled'):
                home = work / (family + '-' + mode)
                usage = home / 'state/thinkthen'
                usage.mkdir(parents=True, mode=0o700)
                with (usage / '.lock').open('w') as lock:
                    os.chmod(lock.name, 0o600)
                    fcntl.flock(lock, fcntl.LOCK_EX)
                    env = child_env(home=home, THINKTHEN_BASE_URL=f'http://127.0.0.1:{server.server_port}/generic/v1',
                                    THINKTHEN_API_KEY='tt-canary-290',
                                    TT_USAGE_MODE=mode, TT_SESSION_QUESTION_FILE=str(question),
                                    TT_SESSION_EVIDENCE='usage-' + family + '-' + mode, TT_SESSION_EXPECT_FAILURE='N')
                    if mode == 'disabled':
                        env.update(HOME='', XDG_STATE_HOME='', APPDATA='', LOCALAPPDATA='')
                    command = [str(work / (family + '-consumer'))] + ([mode] if family == 'ada' else [])
                    child = subprocess.Popen(command, cwd=work, env=env, stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
                    try:
                        prefix = ''
                        if mode != 'disabled':
                            with selectors.DefaultSelector() as ready:
                                ready.register(child.stdout, selectors.EVENT_READ)
                                assert ready.select(15), (family, mode, 'Pending not observed')
                            prefix = child.stdout.readline()
                            assert prefix.strip() == 'PENDING', (family, mode, prefix, child.poll())
                            if mode == 'written':
                                fcntl.flock(lock, fcntl.LOCK_UN)
                            child.stdin.write('continue\n')
                            child.stdin.flush()
                        stdout, stderr = child.communicate(timeout=15)
                        assert child.returncode == 0, (family, mode, child.returncode, prefix + stdout, stderr)
                        assert 'DISABLED_PASS' in stdout if mode == 'disabled' else ('ADA_USAGE_PASS' in stdout if family == 'ada' else 'SESSION_PASS' in stdout)
                        if family == 'cobol' and mode != 'disabled':
                            assert 'VALUE 0000000001' in stdout and 'REQUESTS 00000000000000000001' in stdout, stdout
                        if mode != 'disabled':
                            expected.append('usage-' + family + '-' + mode)
                        assert server.arrivals == expected, server.arrivals
                        print(family, mode, 'PASS')
                    finally:
                        if child.poll() is None:
                            child.kill(); child.communicate()
        assert server.attempts == server.connections == len(expected) == 2 * len(families)
        print('Installed persistence, retained answers/facts/advice and', len(expected), 'exact requests PASS')
    finally:
        server.close()
