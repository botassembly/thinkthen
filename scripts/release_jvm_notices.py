"""Verify shipped JAR inputs and collect their complete runtime terms."""
import hashlib
import json
from pathlib import Path
import re
import zipfile

from release_notices import ROOT, NOTICE


def dependencies(folder, root=ROOT):
    pins = {Path(url).name: digest.removeprefix('sha256:') for url, digest in
            json.loads((root / 'scripts/release-downloads.json').read_text()).items()
            if url.startswith('https://repo.maven.apache.org/') and url.endswith('.jar')}
    files, entries = {}, {}
    for jar in sorted(folder.glob('*.jar')):
        if jar.name.startswith('thinkthen-'): continue
        digest = hashlib.sha256(jar.read_bytes()).hexdigest()
        if digest != pins.get(jar.name): raise ValueError('unlisted or changed runtime JAR: ' + jar.name)
        with zipfile.ZipFile(jar) as archive:
            texts = {name: archive.read(name) for name in archive.namelist()
                     if not name.endswith('/') and re.search(r'(?i)license|notice|copyright|copying', name)}
        supplement = root / 'licenses/runtime' / jar.stem
        if supplement.is_dir():
            texts.update({str(path.relative_to(supplement)): path.read_bytes()
                          for path in supplement.rglob('*') if path.is_file()})
        if not texts: raise ValueError('missing runtime JAR terms: ' + jar.name)
        entries[jar.name] = {'sha256': digest, 'files': {name: hashlib.sha256(data).hexdigest()
                                                       for name, data in texts.items()}}
        files.update({NOTICE + '/jvm/' + jar.name + '/' + name: data for name, data in texts.items()})
    if entries:
        files[NOTICE + '/jvm/index.json'] = (json.dumps({'components': entries}, sort_keys=True, indent=2) + '\n').encode()
    return files


def distribute(out, root=ROOT):
    from release_notice_packages import apply
    for family in ('java', 'kotlin'):
        contents = dependencies(out / family / 'dependencies', root)
        if not contents: raise ValueError('missing shipped runtime JARs: ' + family)
        artifact, = (out / family).glob('thinkthen-' + family + '-*.jar')
        apply(artifact, contents, 'META-INF/')
        if family == 'java':
            for copy in (out / 'kotlin/dependencies').glob('thinkthen-java-*.jar'):
                apply(copy, contents, 'META-INF/')


def check(out, root=ROOT):
    from release_notice_check import contents
    for family in ('java', 'kotlin'):
        expected = dependencies(out / family / 'dependencies', root)
        artifact, = (out / family).glob('thinkthen-' + family + '-*.jar')
        delivered = contents(artifact)
        for name, data in expected.items():
            if delivered['META-INF/' + name] != data: raise ValueError('runtime JAR terms differ: ' + name)
