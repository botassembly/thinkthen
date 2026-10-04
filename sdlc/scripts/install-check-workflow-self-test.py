#!/usr/bin/env python3
"""Plant install-check and dispatch boundary faults into real offline workflow copies."""
import copy
from pathlib import Path
import runpy
import sys
import tempfile
import yaml

sys.path.insert(0, str(Path(__file__).resolve().parent))
REPO = Path(__file__).resolve().parents[2]
workflow_guard = runpy.run_path(str(REPO / 'sdlc/scripts/workflows'))['check']
INSTALL = 'install-check.yml'
RELEASE = 'release.yml'
LAST = 'release.yml: publish must dispatch install-check once as its last unconditional step from the resolved release name and version'
OUTSIDE = 'release.yml: job publish dispatches a workflow outside the exact publish install-check step'
CHECK = 'install-check.yml: job portable must end with the exact unconditional install and replay step using version through env'


def mutate_publish(function):
    return lambda doc: function(doc['jobs']['publish'])


def mutate_portable(function):
    return lambda doc: function(doc['jobs']['portable'])


def cases():
    plants = [
        ('push', INSTALL, lambda doc: doc[True].update({'push': {}}),
         ['install-check.yml: starts on `push`; only workflow_dispatch may start a workflow']),
        ('id-token', INSTALL, mutate_portable(lambda job: job['permissions'].update({'id-token': 'write'})),
         ['install-check.yml: job portable must have only contents: read']),
        ('contents-write', INSTALL, mutate_portable(lambda job: job['permissions'].update({'contents': 'write'})),
         ['install-check.yml: job portable must have only contents: read']),
        ('extra-actions', INSTALL, mutate_portable(lambda job: job['permissions'].update({'actions': 'write'})),
         ['install-check.yml: job portable must have only contents: read']),
        ('top-permission', INSTALL, lambda doc: doc['permissions'].update({'id-token': 'write'}),
         ['install-check.yml: top-level permissions must be {}']),
        ('environment', INSTALL, mutate_portable(lambda job: job.update({'environment': 'release'})),
         ['install-check.yml: job portable must not name an environment']),
        ('secret', INSTALL, mutate_portable(lambda job: job['steps'][0].update({'env': {'KEY': '${{ secrets.GITHUB_TOKEN }}'}})),
         ['install-check.yml: must not read a secret']),
        ('bracket-secret', INSTALL, lambda doc: doc.update({'env': {'KEY': "${{ secrets['TOKEN'] }}"}}),
         ['install-check.yml: must not read a secret']),
        ('input-in-run', INSTALL, mutate_portable(lambda job: job['steps'][-1].update({'run': 'python3 sdlc/scripts/install-check "$CHANNEL" ${{ inputs.version }}'})),
         ['install-check.yml: job portable expands an input or event value inside run; pass it through env', CHECK]),
        ('optional-version', INSTALL, lambda doc: doc[True]['workflow_dispatch']['inputs']['version'].update({'required': False}),
         ['install-check.yml: dispatch must require only a string version']),
        ('drop-runner', INSTALL, mutate_portable(lambda job: job['strategy']['matrix']['runner'].pop()),
         ['install-check.yml: job portable must retain its accepted channel and runner matrix']),
        ('drop-channel', INSTALL, mutate_portable(lambda job: job['strategy']['matrix']['channel'].pop()),
         ['install-check.yml: job portable must retain its accepted channel and runner matrix']),
        ('ignored-install-job', INSTALL, mutate_portable(lambda job: job.update({'continue-on-error': True})),
         ['install-check.yml: job portable must run unconditionally and propagate failures']),
        ('skipped-install-job', INSTALL, mutate_portable(lambda job: job.update({'if': False})),
         ['install-check.yml: job portable must run unconditionally and propagate failures']),
        ('missing-dispatch', RELEASE, mutate_publish(lambda job: job['steps'].pop()), [LAST]),
        ('missing-publish', RELEASE, mutate_publish(lambda job: job['steps'].pop(-2)), [LAST]),
        ('dispatch-not-last', RELEASE, mutate_publish(lambda job: job['steps'].append({'run': 'echo later'})), [LAST]),
        ('dispatch-permission', RELEASE, mutate_publish(lambda job: job['permissions'].pop('actions')),
         ['release.yml: publish must have only contents: write and actions: write']),
        ('extra-dispatch-permission', RELEASE, lambda doc: doc['jobs']['resolve']['permissions'].update({'actions': 'write'}),
         ['release.yml: resolve must enforce the exact rehearsal guard with only contents/actions read and step GH_TOKEN',
          'release.yml: job resolve holds actions: write; only publish may dispatch install-check']),
        ('ignored-publish-job', RELEASE, mutate_publish(lambda job: job.update({'continue-on-error': True})), [LAST]),
        ('dispatch-elsewhere', INSTALL, mutate_portable(lambda job: job['steps'].insert(0, {'run': 'gh workflow run another.yml'})),
         ['install-check.yml: job portable dispatches a workflow outside the exact publish install-check step']),
        ('dispatch-api-elsewhere', INSTALL, mutate_portable(lambda job: job['steps'].insert(0, {'run': 'gh api repos/owner/repo/actions/workflows/other.yml/dispatches'})),
         ['install-check.yml: job portable dispatches a workflow outside the exact publish install-check step']),
    ]
    for key, value in [('if', False), ('continue-on-error', True), ('shell', 'bash {0}')]:
        plants.append((f'install-{key}', INSTALL, mutate_portable(lambda job, k=key, v=value: job['steps'][-1].update({k: v})), [CHECK]))
        plants.append((f'dispatch-{key}', RELEASE, mutate_publish(lambda job, k=key, v=value: job['steps'][-1].update({k: v})), [OUTSIDE, LAST]))
    for run in ('if false; then gh workflow run install-check.yml --ref "$NAME" -f version="$VERSION"; fi',
                'gh workflow run install-check.yml --ref "$NAME" -f version="$VERSION" || true',
                'gh workflow run another.yml --ref "$NAME" -f version="$VERSION"'):
        plants.append(('changed-dispatch', RELEASE, mutate_publish(lambda job, r=run: job['steps'][-1].update({'run': r})), [OUTSIDE, LAST]))
    for key, value in [('NAME', 'main'), ('VERSION', '0.0.1'), ('GH_TOKEN', '${{ vars.TOKEN }}')]:
        plants.append((f'dispatch-env-{key}', RELEASE, mutate_publish(lambda job, k=key, v=value: job['steps'][-1]['env'].update({k: v})), [OUTSIDE, LAST]))
    plants.extend([
        ('unchecked-publish', RELEASE, mutate_publish(lambda job: job['steps'][-2].update({'continue-on-error': True})), [LAST]),
        ('install-default-shell', INSTALL, mutate_portable(lambda job: job.update({'defaults': {'run': {'shell': 'bash {0}'}}})), [CHECK]),
        ('dispatch-default-shell', RELEASE, mutate_publish(lambda job: job.update({'defaults': {'run': {'shell': 'bash {0}'}}})), [LAST]),
    ])
    return plants


def main():
    sources = {path.name: yaml.safe_load(path.read_text()) for path in (REPO / '.github/workflows').glob('*.y*ml')}
    failures = 0
    table = [('real', None, None, []), *cases()]
    for name, file, plant, expected in table:
        docs = copy.deepcopy(sources)
        if file: plant(docs[file])
        with tempfile.TemporaryDirectory(prefix='thinkthen-workflow-plant-') as own:
            folder = Path(own)
            for filename, doc in docs.items(): (folder / filename).write_text(yaml.safe_dump(doc))
            got = workflow_guard(folder)
        if got != expected:
            failures += 1
            print(f'install-check workflow {name}: expected {expected}, got {got}', file=sys.stderr)
    print(f'install-check workflow self-test: {len(table)-failures}/{len(table)} cases hold')
    return 1 if failures else 0


if __name__ == '__main__':
    raise SystemExit(main())
