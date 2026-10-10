"""Resolve the documented JVM SDKs from explicit homes or host executables."""
import os
from pathlib import Path
import shutil
import subprocess
import re


def home(variable, executable, runtime=None):
    chosen = os.environ.get(variable)
    if chosen:
        root = Path(chosen).expanduser().resolve()
    else:
        found = shutil.which(executable)
        if found is None:
            raise RuntimeError(f"{executable} is missing; set {variable}")
        root = Path(found).resolve().parent.parent
    binary = root / "bin" / executable
    if not binary.is_file() or not os.access(binary, os.X_OK):
        raise RuntimeError(f"{variable} must name a home with executable {binary}")
    if runtime is not None and not (root / runtime).is_file():
        raise RuntimeError(f"{variable} must contain {runtime}")
    return root


JDK = home("THINKTHEN_JDK_HOME", "javac")
KOTLIN = home("THINKTHEN_KOTLIN_HOME", "kotlinc", "lib/kotlin-stdlib.jar")
SCALA = home("THINKTHEN_SCALA_HOME", "scalac", "lib/scala.jar")
if not (JDK / "bin/java").is_file() or not (JDK / "bin/jar").is_file():
    raise RuntimeError("THINKTHEN_JDK_HOME must contain java and jar")

def stable():
    for executable in ('javac', 'java'):
        version = subprocess.check_output([str(JDK / 'bin' / executable), '-version'], stderr=subprocess.STDOUT, text=True)
        match = re.search(r'(?:javac |version ")([0-9]+)', version)
        if match is None or int(match[1]) < 22:
            raise RuntimeError('JVM stable package requires JDK 22 or later; preview JDKs are unsupported')


if __name__ == "__main__":
    import sys
    if '--stable' in sys.argv:
        stable()
    import tempfile
    for variable, binary, runtime, original in (
        ("THINKTHEN_JDK_HOME", "javac", None, JDK),
        ("THINKTHEN_KOTLIN_HOME", "kotlinc", "lib/kotlin-stdlib.jar", KOTLIN),
        ("THINKTHEN_SCALA_HOME", "scalac", "lib/scala.jar", SCALA),
    ):
        prior = os.environ.get(variable)
        try:
            with tempfile.TemporaryDirectory(prefix="thinkthen-sdk-path-") as folder:
                alias = Path(folder) / "sdk"
                alias.symlink_to(original, target_is_directory=True)
                os.environ[variable] = str(alias)
                assert home(variable, binary, runtime) == original, f"{variable} override ignored"
                os.environ[variable] = str(Path(folder) / "missing")
                try:
                    home(variable, binary, runtime)
                except RuntimeError:
                    pass
                else:
                    raise AssertionError(f"{variable} missing path accepted")
        finally:
            if prior is None:
                os.environ.pop(variable, None)
            else:
                os.environ[variable] = prior
    print("JVM SDK path overrides and missing-path refusals PASS")
