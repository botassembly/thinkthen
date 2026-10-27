"""Upload only the approved draft artifacts. No package compilation happens here."""
import argparse
import base64
import hashlib
import json
import os
from pathlib import Path
import subprocess
import tarfile
import tempfile
import urllib.request
import urllib.error

from release_workflow import ROOT, checksums, output, version
from release_sites import CRATES, sign_maven


def quiet(*args, **kwargs):
    # Failed publish commands can contain ephemeral credentials in their argv.
    status = subprocess.run(list(map(str, args)), **kwargs).returncode
    if status:
        raise ValueError('package upload command failed; inspect the preceding site response')


def verify_download(destination, digest):
    """Apply the same approved inventory and asset checks before any publication."""
    sums = destination / 'SHA256SUMS'
    if hashlib.sha256(sums.read_bytes()).hexdigest() != digest:
        raise ValueError('draft differs from the PM-approved checksum inventory')
    checksums(destination)


def approved(tag, digest, destination):
    if tag != 'v' + version() or os.environ.get('GITHUB_REF') != 'refs/tags/' + tag:
        raise ValueError('upload must run at the final native package version tag')
    provenance = json.loads((ROOT / 'release-source.json').read_text())
    review = 'review-' + tag
    query = subprocess.run(['gh', 'release', 'view', review, '--json', 'tagName'], capture_output=True, text=True)
    if query.returncode:
        review = tag
        info = json.loads(output('gh', 'release', 'view', review, '--json', 'tagName'))
    else:
        info = json.loads(query.stdout)
    if info['tagName'] != review:
        raise ValueError('the release tag differs from the reviewed version')
    quiet('gh', 'release', 'download', review, '--dir', destination)
    verify_download(destination, digest)
    registry = destination / f'thinkthen-{version()}-registries.tar.gz'
    with tarfile.open(registry) as archive:
        archive.extractall(destination / 'registries', filter='data')
    staged = destination / 'registries'
    if json.loads((staged / 'source.json').read_text()) != provenance:
        raise ValueError('native tag and draft packages have different source revisions')
    return staged


def existing(url, package):
    """An immutable matching artifact is safe to reuse after a partial upload."""
    try:
        request = urllib.request.Request(url, headers={'User-Agent': 'ThinkThen release'})
        with urllib.request.urlopen(request) as response:
            actual = hashlib.sha256(response.read()).digest()
    except urllib.error.HTTPError as error:
        if error.code == 404:
            return False
        raise
    if actual != hashlib.sha256(package).digest():
        raise ValueError('site already serves different bytes for this package version')
    return True


def maven(registry):
    with tempfile.TemporaryDirectory(prefix='thinkthen-sign-') as temporary:
        environment = dict(os.environ, GNUPGHOME=temporary)
        quiet('gpg', '--batch', '--import', input=os.environ['MAVEN_CENTRAL_GPG_PRIVATE_KEY'],
              text=True, env=environment, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        previous = os.environ.get('GNUPGHOME')
        os.environ['GNUPGHOME'] = temporary
        try:
            sign_maven(registry)
        finally:
            if previous is None:
                del os.environ['GNUPGHOME']
            else:
                os.environ['GNUPGHOME'] = previous
    bundle = (registry / 'maven-central.zip').read_bytes()
    boundary = 'thinkthen-central-bundle'
    body = (f'--{boundary}\r\nContent-Disposition: form-data; name="bundle"; filename="bundle.zip"\r\n'
            'Content-Type: application/octet-stream\r\n\r\n').encode() + bundle + f'\r\n--{boundary}--\r\n'.encode()
    credential = base64.b64encode((os.environ['MAVEN_CENTRAL_USERNAME'] + ':' +
                                   os.environ['MAVEN_CENTRAL_PASSWORD']).encode()).decode()
    request = urllib.request.Request('https://central.sonatype.com/api/v1/publisher/upload?publishingType=AUTOMATIC',
                                     body, {'Authorization': 'Bearer ' + credential,
                                            'Content-Type': 'multipart/form-data; boundary=' + boundary})
    with urllib.request.urlopen(request) as response:
        print('Maven Central: deployment accepted, status', response.status)


def upload(site, registry):
    if site == 'npm':
        package, = (registry / 'npm').glob('*.tgz')
        quiet('npm', 'publish', package, '--access', 'public', '--ignore-scripts')
    elif site == 'nuget':
        package, = (registry / 'nuget').glob('*.nupkg')
        quiet('dotnet', 'nuget', 'push', package, '--source', 'https://api.nuget.org/v3/index.json',
              '--api-key', os.environ['NUGET_API_KEY'])
    elif site == 'rubygems':
        for package in sorted((registry / 'rubygems').glob('*.gem')):
            url = 'https://rubygems.org/downloads/' + package.name
            if not existing(url, package.read_bytes()):
                try:
                    quiet('gem', 'push', package)
                except ValueError:
                    if not existing(url, package.read_bytes()):
                        raise
    elif site == 'maven':
        maven(registry)
    elif site == 'crates':
        from release_crates import upload as crate_upload
        for name in CRATES:
            crate_upload(registry / f'crates/{name}-{version()}.crate')
    else:
        raise ValueError('unsupported upload site')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('command', choices=('download', 'upload'))
    parser.add_argument('site')
    parser.add_argument('tag')
    parser.add_argument('checksum')
    parser.add_argument('destination', type=Path)
    args = parser.parse_args()
    registry = approved(args.tag, args.checksum, args.destination)
    if args.command == 'upload':
        upload(args.site, registry)


if __name__ == '__main__':
    try:
        main()
    except Exception as error:
        # HTTP and subprocess objects can carry authorization headers or argv.
        raise SystemExit('release upload failed: ' + type(error).__name__)
