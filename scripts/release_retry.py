"""Admit unchanged qualified products for a Windows-only release rehearsal."""
import hashlib
import io
import json
import os
from pathlib import Path
import re
import subprocess
import zipfile

REPOSITORY = 'botassembly/thinkthen'
ROOT = Path(__file__).resolve().parent.parent
# These files check or orchestrate releases; none is a package or native recipe.
CHECKS = frozenset({
    '.github/workflows/release-prepare.yml', '.github/workflows/native-qualification.yml',
    'scripts/header_contract.py', 'scripts/qualification-recipe-test',
    'scripts/qualification-continuation-test', 'scripts/release-script-test',
    'scripts/release_retry.py', 'scripts/release-retry-test',
})
# Exact reviewed verifier changes in mixed producer modules. Other changes refuse.
VERIFIER_CHANGES = {'.gitattributes': [None, {'mode': '100644', 'kind': 'blob',
                                         'object': 'a1f6ccc6edfebab3210f3ce9cd74c07d9a471a5a'}],
 'scripts/release_workflow.py': [{'mode': '100644',
                                  'kind': 'blob',
                                  'object': '9caae92bbf7183ffaeb89d9aa19f28f6e37beb95'},
                                 {'mode': '100644',
                                  'kind': 'blob',
                                  'object': '4c474b7bf7951ebe15ed6e8aa2c96853db6eda11'}],
 'scripts/release_windows_r.py': [{'mode': '100644',
                                   'kind': 'blob',
                                   'object': '727507ca7902a5b6ae724e29c5ae3b5b199f1be0'},
                                  {'mode': '100644',
                                   'kind': 'blob',
                                   'object': 'ca68a837b374787425a6604819c0fde7de7fda34'}],
 'scripts/gate': [{'mode': '100755',
                   'kind': 'blob',
                   'object': 'ebd131aa7006d8c99b1ca8b5e6496c106da1a517'},
                  {'mode': '100755',
                   'kind': 'blob',
                   'object': 'a274dda8c63a8597dc178d9b8ef1ee9ce9a46b08'}]}
TARGETS = {'linux-x86_64': 'ubuntu-24.04', 'linux-arm64': 'ubuntu-24.04-arm',
           'macos-arm64': 'macos-15'}
ARTIFACTS = {**{'platform-' + target: 'qualification / build (' + target + ', ' + runner + ')'
                for target, runner in TARGETS.items()},
             'native-windows-r': 'qualification / windows-r'}
REQUIRED = {'source', 'qualification / native-gate-inputs', 'qualification / gate',
            *ARTIFACTS.values()}


def git(*args, root=ROOT):
    return subprocess.check_output(['git', *args], cwd=root)


def manifest(revision, root=ROOT):
    """Hash every tracked byte and mode; preserve uninitialized gitlinks."""
    entries = {}
    for record in git('ls-tree', '-rz', '--full-tree', revision, root=root).split(b'\0'):
        if not record:
            continue
        metadata, name = record.split(b'\t', 1)
        mode, kind, oid = metadata.decode().split()
        path = name.decode('utf-8')
        entries[path] = {'mode': mode, 'kind': kind, 'object': oid}
    return {'revision': revision, 'tree': git('rev-parse', revision + '^{tree}', root=root).decode().strip(),
            'files': entries}


def compare(prior, current):
    changed, exceptions = [], []
    for path in sorted(prior['files'].keys() | current['files'].keys()):
        old, new = prior['files'].get(path), current['files'].get(path)
        if old == new:
            continue
        if path not in CHECKS and VERIFIER_CHANGES.get(path) != [old, new]:
            raise ValueError('product source differs: ' + path + '; use a full release run')
        changed.append(path)
        if path not in CHECKS:
            exceptions.append({'path': path, 'prior': old, 'current': new})
    return {'changed_checks': changed, 'reviewed_verifier_changes': exceptions}


def api(path):
    # gh follows artifact redirects without forwarding the API credential.
    return subprocess.check_output(['gh', 'api', '--hostname', 'github.com',
                                    'repos/' + REPOSITORY + '/' + path])


def pages(path, key):
    result = []
    for page in range(1, 101):
        batch = json.loads(api(path + ('&' if '?' in path else '?') + f'per_page=100&page={page}'))[key]
        result.extend(batch)
        if len(batch) < 100:
            return result
    raise ValueError('prior run API response exceeds 100 pages')


def identity(run, jobs, artifacts):
    if (run.get('repository', {}).get('full_name') != REPOSITORY or
            run.get('head_repository', {}).get('full_name') != REPOSITORY or
            run.get('path') != '.github/workflows/release-prepare.yml' or
            run.get('event') != 'workflow_dispatch' or run.get('head_branch') != 'main' or
            run.get('status') != 'completed' or not re.fullmatch('[0-9a-f]{40}', run.get('head_sha', ''))):
        raise ValueError('prior run is not a completed official main release preparation')
    selected = {}
    for name in REQUIRED:
        matches = [job for job in jobs if job['name'] == name]
        if (len(matches) != 1 or matches[0].get('conclusion') != 'success' or
                matches[0].get('run_attempt') != run['run_attempt'] or
                matches[0].get('head_sha') != run['head_sha']):
            raise ValueError('prior required job did not pass: ' + name)
        selected[name] = matches[0]
    admitted = {}
    for name, producer in ARTIFACTS.items():
        matches = [artifact for artifact in artifacts if artifact['name'] == name]
        if len(matches) != 1:
            raise ValueError('expect exactly one prior artifact: ' + name)
        artifact = matches[0]
        source = artifact.get('workflow_run', {})
        # Artifacts have no attempt field. Their creation must fall inside the
        # exact successful attempt's producer, excluding stale rerun artifacts.
        job = selected[producer]
        if (artifact.get('expired') or source.get('id') != run['id'] or
                source.get('head_sha') != run['head_sha'] or
                source.get('head_branch') != 'main' or
                not job['started_at'] <= artifact['created_at'] <= job['completed_at'] or
                not re.fullmatch('sha256:[0-9a-f]{64}', artifact.get('digest', ''))):
            raise ValueError('prior artifact identity differs: ' + name)
        admitted[name] = artifact
    return admitted


def extract(data, destination):
    with zipfile.ZipFile(io.BytesIO(data)) as archive:
        for entry in archive.infolist():
            path = Path(entry.filename)
            if path.is_absolute() or '..' in path.parts or '\\' in entry.filename or (entry.external_attr >> 16) & 0o170000 == 0o120000:
                raise ValueError('unsafe artifact member: ' + entry.filename)
        archive.extractall(destination)


def prepare(run_id, destination):
    if not re.fullmatch('[1-9][0-9]*', run_id):
        raise ValueError('prior run ID must be a positive integer')
    run = json.loads(api('actions/runs/' + run_id))
    if run.get('id') != int(run_id):
        raise ValueError('prior run ID differs from the requested run')
    jobs = pages(f'actions/runs/{run_id}/attempts/{run["run_attempt"]}/jobs', 'jobs')
    artifacts = pages(f'actions/runs/{run_id}/artifacts', 'artifacts')
    admitted = identity(run, jobs, artifacts)
    prior_sha = run['head_sha']
    subprocess.run(['git', 'fetch', '--no-tags', 'https://github.com/' + REPOSITORY + '.git', prior_sha], cwd=ROOT, check=True)
    current_sha = git('rev-parse', 'HEAD').decode().strip()
    prior, current = manifest(prior_sha), manifest(current_sha)
    comparison = compare(prior, current)
    destination.mkdir(parents=True, exist_ok=False)
    receipt = {'repository': REPOSITORY, 'run_id': run['id'], 'run_attempt': run['run_attempt'],
               'prior': prior, 'current': current, **comparison, 'artifacts': {}, 'packages': {},
               'jobs': [{key: job[key] for key in ('id', 'name', 'conclusion', 'run_attempt', 'head_sha', 'started_at', 'completed_at')}
                        for job in jobs if job['name'] in REQUIRED]}
    for name, artifact in admitted.items():
        data = api(f'actions/artifacts/{artifact["id"]}/zip')
        if 'sha256:' + hashlib.sha256(data).hexdigest() != artifact['digest']:
            raise ValueError('downloaded artifact digest differs: ' + name)
        folder = destination / name
        extract(data, folder)
        receipt['artifacts'][name] = {'id': artifact['id'], 'digest': artifact['digest'],
            'producer_job': ARTIFACTS[name], 'files': {p.relative_to(folder).as_posix(): hashlib.sha256(p.read_bytes()).hexdigest()
                                                     for p in folder.rglob('*') if p.is_file()}}
        if name.startswith('platform-'):
            archives = list(folder.glob('*.tar.gz'))
            if len(archives) != 1 or len(list(folder.iterdir())) != 1:
                raise ValueError('expect one platform archive: ' + name)
            target = name.removeprefix('platform-')
            receipt['packages'][target] = {'revision': prior_sha, 'archive': archives[0].name,
                                          'sha256': hashlib.sha256(archives[0].read_bytes()).hexdigest()}
    (destination / 'reuse.json').write_text(json.dumps(receipt, sort_keys=True, indent=2) + '\n')
    print('release retry: verified prior qualification and reused three platforms plus Windows R')


def selected(sha):
    path = os.environ.get('THINKTHEN_RELEASE_REUSE')
    if not path:
        return None
    receipt = json.loads(Path(path).read_text())
    if git('rev-parse', 'HEAD').decode().strip() != sha:
        raise ValueError('release reuse source is not the current checkout')
    current = manifest(sha)
    if receipt['repository'] != REPOSITORY or receipt['current'] != current:
        raise ValueError('release reuse receipt differs from current source')
    compare(receipt['prior'], current)
    return receipt


def archive_revision(target, archive, sha):
    receipt = selected(sha)
    if receipt is None or target == 'windows-x86_64':
        return sha
    package = receipt['packages'][target]
    if archive.name != package['archive'] or hashlib.sha256(archive.read_bytes()).hexdigest() != package['sha256']:
        raise ValueError('reused platform archive differs: ' + target)
    return package['revision']


def r_revision(folder, sha):
    receipt = selected(sha)
    if receipt is None:
        return sha
    expected = receipt['artifacts']['native-windows-r']['files']
    actual = {p.relative_to(folder).as_posix(): hashlib.sha256(p.read_bytes()).hexdigest()
              for p in folder.rglob('*') if p.is_file()}
    if actual != expected:
        raise ValueError('reused Windows R package differs')
    return receipt['prior']['revision']


if __name__ == '__main__':
    import argparse
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('run_id')
    parser.add_argument('destination', type=Path)
    args = parser.parse_args()
    prepare(args.run_id, args.destination)
