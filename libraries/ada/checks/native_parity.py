"""Execute the shared cases through compiled Ada public native methods."""
import os
from pathlib import Path
import re
import subprocess
import tempfile
import sys

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / 'conformance'))
import c_parity as shared
import parity

run = subprocess.run
PACKAGE = Path(os.environ.get("THINKTHEN_PARITY_PACKAGE", ROOT / "libraries/ada" )).resolve(strict=True)
HEADER = Path(os.environ.get("THINKTHEN_C_HEADER", ROOT / "libraries/c/include/thinkthen.h")).resolve(strict=True)
if os.environ.get("THINKTHEN_ARTIFACT") and not os.environ.get("THINKTHEN_PARITY_PACKAGE"):
    raise ValueError("installed ada parity requires its extracted package")

def compile_consumer(scratch, env, include, library, source):
    target = scratch / 'ada'; target.mkdir()
    options = ['-gnat2022', '-I' + str(PACKAGE / 'src'), '-I' + str(ROOT / 'libraries/ada/checks')]
    run(['gnatmake', '-c', *options, str(ROOT / 'libraries/ada/checks/ada_parity.adb'), '-D', str(target)], check=True, env=env)
    obj = scratch / 'driver.o'; output = scratch / 'driver'
    run(['cc', '-pthread', '-std=c11', '-Wall', '-Wextra', '-Werror', '-g', '-fsanitize=address', '-fno-omit-frame-pointer', '-I', str(include), '-c', str(source), '-o', str(obj)], check=True, env=env)
    run(['gnatbind', '-n', '-I' + str(target), str(target / 'ada_parity.ali')], cwd=scratch, check=True, env=env)
    run(['gnatlink', str(target / 'ada_parity.ali'), str(obj), '-o', str(output), str(library), '-pthread', '-fsanitize=address', '-Wl,-rpath,' + str(scratch)], cwd=scratch, check=True, env=env)
    return output


def adapter(scratch):
    declarations = (ROOT / 'libraries/ada/checks/ada_parity.ads').read_text()
    names = re.findall('External_Name => "ada_(thinkthen_\\w+)"', declarations)
    header = HEADER.read_text()
    header = re.sub(r'/\*.*?\*/', '', header, flags=re.S)
    prototypes = []
    for name in names:
        match = re.search(r'\b(?:int|void)\s+' + name + r'\s*\([^;{}]+\)\s*;', header)
        prototypes.append(re.sub(r'\b' + name + r'\b', 'ada_' + name, match.group()))
    prefix = '#include "thinkthen.h"\n' + '\n'.join(prototypes) + '\n'
    prefix += '\n'.join('#define ' + name + ' ada_' + name for name in names)
    prefix += '\nvoid adainit(void); void adafinal(void);\n'
    path = scratch / 'adapter.h'; path.write_text(prefix)
    return path


with tempfile.TemporaryDirectory(prefix='thinkthen-ada-adapter-') as folder:
    raise SystemExit(shared.main(consumer='ada', compile_consumer=compile_consumer,
                                adapter_header=adapter(Path(folder)), initialize='adainit(); atexit(adafinal);'))
