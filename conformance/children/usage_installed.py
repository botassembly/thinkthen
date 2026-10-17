"""Exercise installed Ada and COBOL persistence with real owned usage locks."""
import argparse
import importlib.util
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / 'conformance/children'))
from children import child_env
from usage import run_usage

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--family', choices=('ada', 'cobol'))
parser.add_argument('--ada-package', type=Path)
parser.add_argument('--ada-objects', type=Path)
args = parser.parse_args()
families = (args.family,) if args.family else ('ada', 'cobol')
if bool(args.ada_package) != bool(args.ada_objects):
    parser.error('--ada-package and --ada-objects must be supplied together')

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
       if status-code not = tt-n-cobol-overflow-k or written not = 37
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
    source = source.replace('    allocate tt-n-cobol-RequestCall', '''    call "TT_ENGINE_USAGE_PERSISTENCE" using by value engine
       by reference tt-n-complete-usage-persistence-v1 saved-advice saved-length
       returning status-code
    if status-code not = 0 move 30 to return-code goback end-if
    if mode-text = "disabled"
       if v-kind of tt-n-complete-usage-persistence-v1 not =
          tt-n-complete-usage-persistence-disabled-v1-k
          move 31 to return-code goback end-if
       call "TT_ENGINE_FINISH_USAGE_STATUS" using by value engine
          by reference tt-n-complete-usage-persistence-v1 saved-advice saved-length
          returning status-code
       if status-code not = 0 or saved-length not = 0 or
          v-kind of tt-n-complete-usage-persistence-v1 not =
          tt-n-complete-usage-persistence-disabled-v1-k
          move 32 to return-code goback end-if
       call "thinkthen_engine_free" using by value engine returning omitted
       free tt-n-complete-usage-persistence-v1
       display "DISABLED_PASS" move 0 to return-code goback
    end-if
    if v-kind of tt-n-complete-usage-persistence-v1 not =
       tt-n-complete-usage-persistence-written-v1-k
       move 33 to return-code goback end-if
    allocate tt-n-cobol-RequestCall''', 1)
    close = '    call "thinkthen_engine_free" using by value engine returning omitted\n    if terminal-owner'
    source = source.replace(close, '''    call "TT_ENGINE_USAGE_PERSISTENCE" using by value engine
       by reference tt-n-complete-usage-persistence-v1 saved-advice saved-length
       returning status-code
    if status-code not = 0 or saved-length not = 0 or
       v-kind of tt-n-complete-usage-persistence-v1 not =
       tt-n-complete-usage-persistence-pending-v1-k
       move 34 to return-code goback end-if
    display "PENDING"
    accept continue-text
    call "TT_ENGINE_FINISH_USAGE_STATUS" using by value engine
       by reference tt-n-complete-usage-persistence-v1 saved-advice saved-length
       returning status-code
    if status-code not = 0 move 35 to return-code goback end-if
    move v-kind of tt-n-complete-usage-persistence-v1 to saved-kind
    if mode-text = "written"
       if saved-kind not = tt-n-complete-usage-persistence-written-v1-k
          or saved-length not = 0 move 36 to return-code goback end-if
    else
       if saved-kind not = tt-n-complete-usage-persistence-failed-v1-k
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
    if status-code not = tt-n-eusage-k or saved-kind not =
       v-kind of tt-n-complete-usage-persistence-v1 or output-text not = saved-advice
       or written not = saved-length move 39 to return-code goback end-if
    call "TT_ENGINE_FINISH_USAGE_STATUS" using by value engine
       by reference tt-n-complete-usage-persistence-v1 output-text written
       returning status-code
    if status-code not = tt-n-eusage-k move 40 to return-code goback end-if
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
        if family != 'ada' or not args.ada_package:
            shutil.copytree(package / 'src', work / family / 'src')
    if 'cobol' in families:
        package = Path(os.environ.get('THINKTHEN_USAGE_COBOL_PACKAGE', ROOT / 'libraries/cobol'))
        shutil.copytree(package / 'copybooks', work / 'cobol/copybooks')
        shutil.copytree(package / 'examples', work / 'cobol/examples')
        shutil.copy2(Path(os.environ.get('THINKTHEN_C_HEADER', ROOT / 'libraries/c/include/thinkthen.h')), work / 'cobol/src/thinkthen.h')
    if 'cobol' in families or not args.ada_package:
        shutil.copy2(NATIVE, work / 'libthinkthen.so.0')
        (work / 'libthinkthen.so').symlink_to('libthinkthen.so.0')
    if 'ada' in families:
        shutil.copy2(ROOT / 'libraries/ada/checks/usage_status.adb', work / 'usage_status.adb')
        (work / 'ada-objects').mkdir()
        package = args.ada_package.resolve(strict=True) if args.ada_package else work / 'ada'
        reuse = ['-aO' + str(args.ada_objects.resolve(strict=True))] if args.ada_objects else []
        linking = [str(package / 'native/lib/libthinkthen.a'), '-ldl', '-lpthread', '-lm'] if args.ada_package else ['-L' + str(work), '-lthinkthen', '-Wl,-rpath,$ORIGIN']
        build = subprocess.run(['gnatmake', '-q', '-gnat2022', '-I' + str(package / 'src'), *reuse,
                                str(work / 'usage_status.adb'), '-D', str(work / 'ada-objects'),
                                '-o', str(work / 'ada-consumer'), '-largs', *linking],
                               cwd=work, env=child_env(), capture_output=True, text=True, timeout=60)
        assert build.returncode == 0, build.stdout + build.stderr
    if 'cobol' in families:
        (work / 'usage_status.cob').write_text(cobol_source(work / 'cobol'))
        subprocess.run(['cobc', '-x', '-free', '-fstatic-call', '-fno-gen-c-decl-static-call',
                        '-I', str(work / 'cobol/copybooks'), '-A', '-include ' + str(work / 'cobol/src/tt_session.h') + ' -Wno-incompatible-pointer-types',
                        '-o', str(work / 'cobol-consumer'), str(work / 'usage_status.cob'),
                        str(work / 'cobol/src/tt_session.c'), str(work / 'cobol/src/tt_requests_generated.c'),
                        '-Q', str(work / 'libthinkthen.so.0') + ' -Wl,-rpath,$ORIGIN'], env=child_env(), check=True, timeout=30)
    if os.environ.get('THINKTHEN_COVERAGE_STAGE'):
        stage = Path(os.environ['THINKTHEN_COVERAGE_STAGE']); stage.mkdir(parents=True, exist_ok=True)
        for family in families:
            shutil.copy2(work / (family + '-consumer'), stage / (family + '-consumer'))
    from backend import Backend
    server = Backend(work)
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
                env = child_env(home=home, THINKTHEN_BASE_URL=f'http://127.0.0.1:{server.server_port}/generic/v1',
                                THINKTHEN_API_KEY='tt-canary-290',
                                TT_USAGE_MODE=mode, TT_SESSION_QUESTION_FILE=str(question),
                                TT_SESSION_EVIDENCE='usage-' + family + '-' + mode, TT_SESSION_EXPECT_FAILURE='N')
                if mode == 'disabled':
                    env.update(HOME='', XDG_STATE_HOME='', APPDATA='', LOCALAPPDATA='')
                command = [str(work / (family + '-consumer'))] + ([mode] if family == 'ada' else [])
                result = run_usage(command, state=home / 'state/thinkthen', mode=mode,
                                   env=env, cwd=work, pending='stdout')
                stdout, stderr = result.stdout, result.stderr
                assert result.returncode == 0, (family, mode, result.returncode, stdout, stderr)
                assert 'DISABLED_PASS' in stdout if mode == 'disabled' else ('ADA_USAGE_PASS' in stdout if family == 'ada' else 'SESSION_PASS' in stdout)
                if family == 'cobol' and mode != 'disabled':
                    assert 'VALUE 0000000001' in stdout and 'REQUESTS 00000000000000000001' in stdout, stdout
                if mode != 'disabled':
                    expected.append('usage-' + family + '-' + mode)
                assert server.arrivals == expected, server.arrivals
                print(family, mode, 'PASS')
        assert server.attempts == server.connections == len(expected) == 2 * len(families)
        print('Installed persistence, retained answers/facts/advice and', len(expected), 'exact requests PASS')
    finally:
        server.close()
