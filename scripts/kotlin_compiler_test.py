"""Compile the literal Kotlin README against installed JARs despite Dokka cache order."""
import json
import os
from pathlib import Path
import runpy
import shutil
import subprocess
import sys
import tempfile

from kotlin_compiler import MANIFEST, ROOT, classpath, repository, version
from release_registry_check import environment

installed = Path(sys.argv[1]).resolve()
prepared = repository()
closure = json.loads((prepared / MANIFEST).read_text())
dokka = prepared / 'org/jetbrains/dokka/analysis-kotlin-descriptors/2.0.0/analysis-kotlin-descriptors-2.0.0.jar'
assert dokka.is_file(), 'prepare Dokka before this regression check'
readme = runpy.run_path(str(ROOT / 'scripts/qualification-readmes'))['kotlin_readme']
with tempfile.TemporaryDirectory(prefix='kotlin-compiler-check-') as temporary:
    scratch = Path(temporary)
    package = scratch / 'package/kotlin'
    package.mkdir(parents=True)
    shutil.copyfile(ROOT / 'sdk/kotlin/README.md', package / 'README.md')
    jars = list(installed.glob('*.jar'))
    assert any(jar.name.startswith('thinkthen-kotlin-') for jar in jars), 'supply installed JVM lib folder'
    for jar in jars:
        (package / jar.name).symlink_to(jar)
    for reverse in (False, True):
        case = scratch / str(reverse)
        cache = case / 'cache'
        maven = cache / 'm2'
        case.mkdir()
        cache.mkdir(mode=0o700)
        maven.mkdir()
        paths = [*closure['jars'], 'dokka-analysis.jar']
        for name in reversed(paths) if reverse else paths:
            destination = maven / name
            destination.parent.mkdir(parents=True, exist_ok=True)
            destination.symlink_to(dokka if name == 'dokka-analysis.jar' else prepared / name)
        (maven / MANIFEST).write_text(json.dumps(closure))
        os.environ['THINKTHEN_JVM_CACHE'] = str(cache)
        output = subprocess.run(['java', '-cp', classpath(), 'org.jetbrains.kotlin.cli.jvm.K2JVMCompiler', '-version'],
                                capture_output=True, text=True, check=True)
        assert 'kotlinc-jvm ' + version() in output.stdout + output.stderr
        print((output.stdout + output.stderr).strip(), flush=True)
        env = environment(case) | {'THINKTHEN_BACKEND': 'stub'}
        (case / 'home').mkdir(exist_ok=True)
        def run(*args, **options):
            result = subprocess.run(list(map(str, args)), cwd=case, env=env,
                                    capture_output=True, text=True, **options)
            if result.returncode:
                print(result.stderr, file=sys.stderr, flush=True)
                result.check_returncode()
            if args[-1] == 'FirstKt':
                assert result.stdout.strip() == 'true', result.stdout
            print(result.stdout, end='', flush=True)
            return result
        readme(package.parent, case, run)
        print('Dokka cache insertion order reversed=' + str(reverse) + ': PASS', flush=True)
        if not reverse:
            # Reproduce the former whole-cache compiler selection against the same README and SDK.
            selected = readme.__globals__['compiler_classpath']
            readme.__globals__['compiler_classpath'] = lambda: os.pathsep.join(str(p) for p in maven.rglob('*.jar'))
            try:
                readme(package.parent, case, run)
            except subprocess.CalledProcessError as error:
                assert 'expected version is 2.0.0' in error.stderr, error.stderr
                print('Former whole-cache classpath: reproduced metadata mismatch', flush=True)
            else:
                raise AssertionError('former whole-cache classpath unexpectedly passed')
            finally:
                readme.__globals__['compiler_classpath'] = selected
print('Pinned Kotlin compiler closure and literal README: PASS')
