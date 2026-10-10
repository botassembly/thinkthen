"""Compile a generated named Ada caller against an installed release package."""
import importlib.util
import json
import os
from pathlib import Path
import subprocess
import sys
from session_values import ROOT, construct, definitions, ada_text
sys.path.insert(0, str(ROOT / 'conformance/children'))
from children import child_env

spec = importlib.util.spec_from_file_location('shared_session_inputs', ROOT / 'libraries/cpp/fixtures/session_cases.py')
inputs = importlib.util.module_from_spec(spec)
spec.loader.exec_module(inputs)
package = Path(sys.argv[1])
fixture = json.loads(Path(sys.argv[2]).read_text())
home = Path.cwd()
descriptor = inputs.descriptor(fixture, home)
verb = descriptor.pop('verb')
cancel = descriptor.pop('cancel')
held = descriptor.pop('held_cancel')
expression = construct('RequestCall_' + verb, definitions['RequestCall_' + verb], {'function': verb, **descriptor})
source = (Path(__file__).with_name('full_case.adb')).read_text()
source = source.replace('-- REQUEST', f'Request : constant T_RequestCall_{verb} := {expression};')
source = source.replace('-- SETTINGS', 'Ada.Strings.Unbounded.To_String (' + ada_text(sys.argv[3]) + '),')
source = source.replace('CALL_NAME', verb.title())
source = source.replace('-- CANCEL', 'Cancel (Owner);' if cancel else '')
source = source.replace('HELD_CANCEL', "Standard.Boolean'(" + str(bool(held)) + ')')
path = home / 'thinkthen-sessions-parity_case.adb'
path.write_text(source)
objects = home / 'ada-objects'
objects.mkdir(exist_ok=True)
binary = home / 'ada-consumer'
build = subprocess.run(['gnatmake', '-q', '-gnat2022', '-I' + str(package / 'src'), str(path),
                        '-D', str(objects), '-o', str(binary), '-largs',
                        str(package / 'native/lib/libthinkthen.a'), '-ldl', '-lpthread', '-lm'],
                       env=child_env(), text=True, capture_output=True, timeout=120)
assert build.returncode == 0, build.stdout + build.stderr
os.execve(str(binary), [str(binary)], env=child_env(home=home, THINKTHEN_API_KEY='sk-conformance-loopback', LIQUIDAI_API_KEY='sk-conformance-loopback', OPENROUTER_API_KEY='sk-conformance-loopback', PERPLEXITY_API_KEY='sk-conformance-loopback'))
