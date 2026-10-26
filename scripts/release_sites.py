"""Stage source-index and extension-site inputs from qualified release packages."""
import hashlib
import json
from pathlib import Path
import shutil
import subprocess
import tarfile
import zipfile

from release_workflow import ROOT, run, version

CRATES = ('thinkthen',)


def sources(staged, scratch):
    """Vendor R once so its final tag is a complete R package source."""
    r = staged / 'r'
    r.mkdir()
    source = scratch / 'tag-source/sdk/r'
    run('scripts/r-package', '--source', source)
    from release_notices import source_bundle
    for name, data in source_bundle(source / 'src/engine/vendor').items():
        path = source / 'inst' / name
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(data)
    with tarfile.open(r / f'thinkthen_{version()}.tar.gz', 'w:gz') as archive:
        archive.add(source, arcname='thinkthen')
    universe = [{'package': 'thinkthen', 'url': 'https://github.com/botassembly/thinkthen',
                 'subdir': 'sdk/r', 'branch': '*release'}]
    (r / 'packages.json').write_text(json.dumps(universe, indent=2) + '\n')


def crates(staged):
    destination = staged / 'crates'
    destination.mkdir()
    import os
    target = Path(os.environ.get('CARGO_TARGET_DIR', ROOT / 'target'))
    run('cargo', 'package', '--offline', '--no-verify', '--target-dir', target,
        *[arg for name in CRATES for arg in ('-p', name)])
    for name in CRATES:
        archive = destination / f'{name}-{version()}.crate'
        shutil.copy2(target / 'package' / archive.name, archive)
        from release_crates import metadata
        archive.with_suffix('.json').write_text(json.dumps(metadata(archive)) + '\n')


def sign_maven(registry):
    """Sign staged bytes, then add Central's required checksums to the bundle."""
    maven = registry / 'maven'
    for path in sorted(maven.rglob('*')):
        if not path.is_file():
            continue
        from release_upload import quiet
        quiet('gpg', '--batch', '--yes', '--pinentry-mode', 'loopback', '--passphrase-fd', '0',
            '--armor', '--detach-sign', path, input=__import__('os').environ['MAVEN_CENTRAL_GPG_PASSPHRASE'], text=True,
            stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    for path in sorted(maven.rglob('*')):
        if path.is_file():
            for name in ('md5', 'sha1'):
                path.with_name(path.name + '.' + name).write_text(hashlib.new(name, path.read_bytes()).hexdigest())
    with zipfile.ZipFile(registry / 'maven-central.zip', 'w', zipfile.ZIP_DEFLATED) as archive:
        for path in sorted(maven.rglob('*')):
            if path.is_file():
                archive.write(path, path.relative_to(maven))


def homebrew(staged):
    destination = staged / 'homebrew'
    destination.mkdir()
    text = ['class Thinkthen < Formula', '  desc "Typed questions about evidence"',
            '  homepage "https://github.com/botassembly/thinkthen"',
            f'  version "{version()}"', '  license "MIT"', '  if OS.mac?',
            '    depends_on arch: :arm64', '    depends_on macos: :sequoia']
    for index, target in enumerate(('macos-arm64', 'linux-arm64', 'linux-x86_64')):
        if index == 1:
            text.append('  elsif Hardware::CPU.arm?')
        elif index == 2:
            text.append('  else')
        name = f'thinkthen-{version()}-{target}.tar.gz'
        digest = hashlib.sha256((staged / 'github' / name).read_bytes()).hexdigest()
        text.extend([f'    url "https://github.com/botassembly/thinkthen/releases/download/v{version()}/{name}"',
                     f'    sha256 "{digest}"'])
    text.extend(['  end', '  def install', '    bin.install "thinkthen"', '    man1.install "thinkthen.1"', '    pkgshare.install "THIRD-PARTY-NOTICES"', '  end',
                 '  test do', f'    assert_match "thinkthen {version()}", shell_output("#{{bin}}/thinkthen --version")',
                 '  end', 'end'])
    (destination / 'thinkthen.rb').write_text('\n'.join(text) + '\n')


def validate(staged):
    import os
    tools = Path.home() / '.local/share/thinkthen-build-tools/registry-validators/python/bin'
    run(tools / 'twine', 'check', '--strict', *sorted((staged / 'pypi').glob('*.whl')))
    for package in sorted((staged / 'npm').glob('*.tgz')):
        run('npm', 'publish', '--dry-run', '--ignore-scripts', '--offline', package)
    for package in sorted((staged / 'rubygems').glob('*.gem')):
        run('ruby', '-rrubygems/package', '-e', 'Gem::Package.new(ARGV.fetch(0)).verify', package)
