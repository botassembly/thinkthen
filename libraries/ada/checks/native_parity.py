"""Execute the shared cases through compiled Ada public native methods."""
import inspect
import os
from pathlib import Path
import re
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / 'conformance'))
import c_parity as shared
import parity

run = subprocess.run

def compile_consumer(command, *args, **kwargs):
    if command[0] != 'cc' or '-fsanitize=address' not in command:
        return run(command, *args, **kwargs)
    source = Path(next(part for part in command if part.endswith('/driver.c')))
    scratch = source.parent
    target = ROOT / 'libraries/ada/checks/target'
    target.mkdir(exist_ok=True)
    options = ['-gnat2022', '-I' + str(ROOT / 'libraries/ada/src'), '-I' + str(ROOT / 'libraries/ada/checks')]
    run(['gnatmake', '-c', *options, str(ROOT / 'libraries/ada/checks/ada_parity.adb'), '-D', str(target)], check=True, env=kwargs.get('env'))
    declarations = (ROOT / 'libraries/ada/checks/ada_parity.ads').read_text()
    names = re.findall('External_Name => "ada_(thinkthen_\\w+)"', declarations)
    header = (ROOT / 'libraries/c/include/thinkthen.h').read_text()
    header = re.sub(r'/\*.*?\*/', '', header, flags=re.S)
    prototypes = []
    for name in names:
        match = re.search(r'\b(?:int|void)\s+' + name + r'\s*\([^;{}]+\)\s*;', header)
        prototypes.append(re.sub(r'\b' + name + r'\b', 'ada_' + name, match.group()))
    prefix = '#include "thinkthen.h"\n' + '\n'.join(prototypes) + '\n'
    prefix += '\n'.join('#define ' + name + ' ada_' + name for name in names)
    prefix += '\nvoid adainit(void); void adafinal(void);\n'
    text = source.read_text().replace('int main(int argc,char **argv) {', 'int main(int argc,char **argv) { adainit(); atexit(adafinal);')
    source.write_text(prefix + text)
    output = Path(command[command.index('-o') + 1])
    obj = scratch / 'driver.o'
    # C and Ada share the matching header's descriptors. C assertions only read
    # snapshots produced through the Ada methods exported by this consumer.
    compile_command = ['cc', '-pthread', '-std=c11', '-Wall', '-Wextra', '-Werror', '-g', '-fsanitize=address', '-fno-omit-frame-pointer', '-I', str(ROOT / 'libraries/c/include'), '-c', str(source), '-o', str(obj)]
    run(compile_command, check=True, env=kwargs.get('env'))
    run(['gnatbind', '-n', '-I' + str(target), str(target / 'ada_parity.ali')], cwd=scratch, check=True, env=kwargs.get('env'))
    return run(['gnatlink', str(target / 'ada_parity.ali'), str(obj), '-o', str(output), '-L' + str(ROOT / 'libraries/c/target/debug'), '-lthinkthen_c', '-pthread', '-fsanitize=address', '-Wl,-rpath,' + str(scratch)], cwd=scratch, check=True, env=kwargs.get('env'))

subprocess.run = compile_consumer
# Reuse the suite's inputs, assertions, loopback counts and storage steps.
source = inspect.getsource(shared.main).replace("required_cases(inventory, 'c')", "required_cases(inventory, 'ada')").replace("'consumer': 'c'", "'consumer': 'ada'").replace('C fixture ', 'Ada fixture ').replace('C shared fixture results:', 'Ada shared fixture results:')
exec(compile(source, __file__, 'exec'), shared.__dict__)
raise SystemExit(shared.main())
