"""Upload only the approved draft artifacts. No package compilation happens here."""
import argparse
import base64
import hashlib
import json
import os
from pathlib import Path
import subprocess
import shutil
import time
import uuid
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


def approved(tag, digest, destination, source=ROOT):
    recovery = os.environ.get('GITHUB_REF') == 'refs/tags/v0.2.0-registry-recovery.1'
    if recovery:
        if (tag != 'v0.2.0' or digest != '102b6539e2e90d74a4c4f862a17067851f82c14236059964c8612f8ada903859'
                or output('git', '-C', source, 'rev-parse', 'HEAD') != '24e419497cf89715a88727546e4f3b6661f8303e'):
            raise ValueError('Registry recovery requires the approved final source and checksum')
    elif tag != 'v' + version() or os.environ.get('GITHUB_REF') != 'refs/tags/' + tag:
        raise ValueError('upload must run at the final native package version tag')
    if not recovery and source.resolve() != ROOT.resolve():
        raise ValueError('normal publication uses the version-tag source checkout')
    provenance = json.loads((source / 'release-source.json').read_text())
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


def ruby_existing(package):
    """Only the platform version API establishes absence; downloads must match."""
    platform = package.stem.removeprefix('thinkthen-' + version()).removeprefix('-') or 'ruby'
    url = f'https://rubygems.org/api/v2/rubygems/thinkthen/versions/{version()}.json?platform={platform}'
    try:
        with urllib.request.urlopen(urllib.request.Request(url, headers={'User-Agent': 'ThinkThen release'})) as response:
            response.read()
    except urllib.error.HTTPError as error:
        if error.code == 404:
            return False
        raise
    if not existing('https://rubygems.org/downloads/' + package.name, package.read_bytes()):
        raise ValueError('RubyGems version exists but its package download is unavailable')
    return True


def maven(registry, evidence=None):
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
    if evidence is not None:
        receipt = json.loads((evidence / 'receipt.json').read_text())
        files = {path.relative_to(registry / 'maven').as_posix(): hashlib.sha256(path.read_bytes()).hexdigest()
                 for path in (registry / 'maven').rglob('*') if path.is_file()}
        expected = dict(receipt['original_files'])
        difference = receipt['metadata_difference']
        expected[difference['path']] = difference['after']
        if any(files.get(path) != digest for path, digest in expected.items()):
            raise ValueError('outbound signing changed an original Maven file')
        receipt['outbound_files'] = files
        receipt['bundle_sha256'] = hashlib.sha256(bundle).hexdigest()
        (evidence / 'receipt.json').write_text(json.dumps(receipt, indent=2) + '\n')
        central('deployment/' + receipt['predecessor'], 'DELETE')
        receipt['upload_started'] = True
        (evidence / 'receipt.json').write_text(json.dumps(receipt, indent=2) + '\n')
    with urllib.request.urlopen(request, timeout=60) as response:
        deployment = str(uuid.UUID(response.read().decode().strip()))
    print('Maven Central deployment ID:', deployment, flush=True)
    if evidence is not None:
        (evidence / 'deployment-id.txt').write_text(deployment + '\n')
        receipt = json.loads((evidence / 'receipt.json').read_text())
        receipt['deployment_id'] = deployment
        (evidence / 'receipt.json').write_text(json.dumps(receipt, indent=2) + '\n')
    return deployment


def central(path, method='POST'):
    credential = base64.b64encode((os.environ['MAVEN_CENTRAL_USERNAME'] + ':' +
                                  os.environ['MAVEN_CENTRAL_PASSWORD']).encode()).decode()
    request = urllib.request.Request('https://central.sonatype.com/api/v1/publisher/' + path,
                                     headers={'Authorization': 'Bearer ' + credential}, method=method)
    with urllib.request.urlopen(request, timeout=30) as response:
        return response.read()


def central_status(deployment, evidence, name):
    deployment = str(uuid.UUID(deployment))
    status = json.loads(central('status?id=' + deployment))
    (evidence / name).write_text(json.dumps(status, indent=2) + '\n')
    if status.get('deploymentId') != deployment:
        raise ValueError('Central returned a different deployment identity')
    return status


def wait_central(deployment, evidence):
    deadline = time.monotonic() + 20 * 60
    while True:
        status = central_status(deployment, evidence, 'status.json')
        state = status.get('deploymentState')
        print('Maven Central deployment', deployment, state, flush=True)
        if state == 'PUBLISHED':
            return
        if state == 'FAILED':
            raise ValueError('Central deployment failed; retained status.json includes validation errors')
        if state not in ('PENDING', 'VALIDATING', 'VALIDATED', 'PUBLISHING'):
            raise ValueError('Central returned an unknown deployment state')
        remaining = deadline - time.monotonic()
        if remaining <= 0:
            raise TimeoutError('Central publication timed out; resume status-only with the retained deployment ID')
        time.sleep(min(15, remaining))


def maven_recovery(registry):
    evidence = ROOT / 'maven-recovery'
    evidence.mkdir(exist_ok=True)
    deployment = os.environ.get('MAVEN_DEPLOYMENT_ID', '')
    if deployment:
        deployment = str(uuid.UUID(deployment))
        (evidence / 'deployment-id.txt').write_text(deployment + '\n')
        (evidence / 'receipt.json').write_text(json.dumps({'deployment_id': deployment, 'status_only': True}) + '\n')
        wait_central(deployment, evidence)
        return
    if os.environ.get('GITHUB_RUN_ATTEMPT') != '1' or (evidence / 'receipt.json').exists():
        raise ValueError('recovery reruns require the recorded deployment ID and never upload again')
    predecessor = '67d06e5b-c5e4-43b5-a357-f7818e726d77'
    status = central_status(predecessor, evidence, 'predecessor.json')
    if status.get('deploymentState') != 'FAILED':
        raise ValueError('only the named FAILED predecessor permits this corrected upload')
    outbound = evidence / 'outbound'
    shutil.copytree(registry / 'maven', outbound / 'maven')
    original = {path.relative_to(outbound / 'maven').as_posix(): hashlib.sha256(path.read_bytes()).hexdigest()
                for path in (outbound / 'maven').rglob('*') if path.is_file()}
    name = 'io/github/botassembly/thinkthen-jvm/0.2.0/thinkthen-jvm-0.2.0.pom'
    pom = outbound / 'maven' / name
    before = pom.read_bytes()
    if hashlib.sha256(before).hexdigest() != 'b63131f98b802f41e7e8dab7b5e092ee7f09537ec1d0303371146c8cf5a1de57':
        raise ValueError('the original relocation POM differs from the approved asset')
    insertion = b'<name>ThinkThen JVM</name>'
    corrected = before.replace(b'<packaging>pom</packaging>', b'<packaging>pom</packaging>' + insertion, 1)
    if corrected == before or corrected.replace(insertion, b'', 1) != before:
        raise ValueError('the relocation POM correction is not the approved reversible insertion')
    pom.write_bytes(corrected)
    (evidence / 'corrected-pom.xml').write_bytes(corrected)
    corrected_hash = hashlib.sha256(corrected).hexdigest()
    receipt = {'predecessor': predecessor, 'original_files': original,
               'metadata_difference': {'path': name, 'before': original[name], 'after': corrected_hash,
                                       'insertion': insertion.decode()}, 'upload_started': False}
    (evidence / 'receipt.json').write_text(json.dumps(receipt, indent=2) + '\n')
    deployment = maven(outbound, evidence)
    wait_central(deployment, evidence)


def upload(site, registry):
    if site == 'npm':
        package, = (registry / 'npm').glob('*.tgz')
        quiet('npm', 'publish', package, '--access', 'public', '--ignore-scripts')
    elif site == 'nuget':
        package, = (registry / 'nuget').glob('*.nupkg')
        quiet('dotnet', 'nuget', 'push', package, '--source', 'https://api.nuget.org/v3/index.json',
              '--api-key', os.environ['NUGET_API_KEY'], '--no-symbols')
    elif site == 'rubygems':
        for package in sorted((registry / 'rubygems').glob('*.gem')):
            if not ruby_existing(package):
                try:
                    quiet('gem', 'push', package)
                except ValueError:
                    if not ruby_existing(package):
                        raise
    elif site == 'maven':
        if os.environ.get('GITHUB_REF') == 'refs/tags/v0.2.0-registry-recovery.1':
            maven_recovery(registry)
        else:
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
    parser.add_argument('--source', type=Path, default=ROOT)
    args = parser.parse_args()
    if os.environ.get('GITHUB_REF') == 'refs/tags/v0.2.0-registry-recovery.1' and args.site not in ('rubygems', 'maven'):
        raise ValueError('the recovery tooling tag permits only RubyGems and Maven')
    registry = approved(args.tag, args.checksum, args.destination, args.source)
    if args.command == 'upload':
        upload(args.site, registry)


if __name__ == '__main__':
    try:
        main()
    except Exception as error:
        # HTTP and subprocess objects can carry authorization headers or argv.
        raise SystemExit('release upload failed: ' + type(error).__name__)
