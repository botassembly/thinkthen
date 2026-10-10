"""Dispatch and permission boundaries for published install checks (0398B)."""
import re

DISPATCH_STEP = {
    'env': {'GH_TOKEN': '${{ secrets.GITHUB_TOKEN }}',
            'NAME': '${{ needs.resolve.outputs.name }}',
            'VERSION': '${{ needs.resolve.outputs.version }}'},
    'run': 'gh workflow run install-check.yml --ref "$NAME" -f version="$VERSION"',
}
PUBLISH_STEP = {
    'env': {'GH_TOKEN': '${{ secrets.GITHUB_TOKEN }}',
            'NAME': '${{ needs.resolve.outputs.name }}',
            'SHA': '${{ needs.resolve.outputs.sha }}'},
    'run': 'sdlc/scripts/release-workflow publish "$NAME" "$SHA"',
}
RUNNERS = ['ubuntu-24.04', 'ubuntu-24.04-arm', 'macos-15', 'macos-15-intel', 'macos-26']
PORTABLE = ['download', 'cargo-install', 'cargo-add', 'pip', 'uv', 'npm', 'rubygems']
WINDOWS = ['download', 'cargo-install', 'cargo-add', 'pip', 'uv', 'npm', 'nuget', 'maven', 'c']
LINUX = ['nuget', 'maven', 'pub', 'packagist', 'go', 'c', 'sqlite', 'duckdb', 'postgresql']
R_IMAGE = 'rocker/r-ver:4.6.1@sha256:268e4c559c905a78519793f1a8c9ec5c8051a6ac98a16cbf489135ccdad330bb'


def publish_dispatch(name, doc):
    jobs = doc.get('jobs') or {}
    publish = jobs.get('publish') or {}
    out = []
    if publish.get('permissions') != {'contents': 'write', 'actions': 'write'}:
        out.append(f'{name}: publish must have only contents: write and actions: write')
    steps = publish.get('steps') or []
    if (len(steps) < 2 or steps[-2:] != [PUBLISH_STEP, DISPATCH_STEP] or publish.get('continue-on-error') is not None
            or 'defaults' in publish or 'defaults' in doc
            or sum(step == DISPATCH_STEP for step in steps) != 1):
        out.append(f'{name}: publish must dispatch install-check once as its last unconditional step from the resolved release name and version')
    for job, spec in jobs.items():
        if job != 'publish' and (spec.get('permissions') or {}).get('actions') == 'write':
            out.append(f'{name}: job {job} holds actions: write; only publish may dispatch install-check')
    return out


def other_dispatches(name, doc):
    out = []
    for job, spec in (doc.get('jobs') or {}).items():
        for step in spec.get('steps') or []:
            run = step.get('run') or ''
            # Direct CLI and REST dispatches share the same sole accepted route.
            if re.search(r'\bgh\s+workflow\s+run\b|/dispatches\b|workflow_dispatch\b', run):
                if name != 'release.yml' or job != 'publish' or step != DISPATCH_STEP:
                    out.append(f'{name}: job {job} dispatches a workflow outside the exact publish install-check step')
    return out


def install_workflow(name, doc):
    out = []
    if doc.get('permissions') != {}:
        out.append(f'{name}: top-level permissions must be {{}}')
    trigger = doc.get('on', doc.get(True)) or {}
    dispatch = trigger.get('workflow_dispatch') if isinstance(trigger, dict) else None
    inputs = (dispatch or {}).get('inputs', {}) if isinstance(dispatch, dict) else {}
    if (set(inputs) != {'version'} or inputs['version'].get('type') != 'string'
            or inputs['version'].get('required') is not True):
        out.append(f'{name}: dispatch must require only a string version')
    jobs = doc.get('jobs') or {}
    if set(jobs) != {'portable', 'homebrew', 'linux', 'windows', 'r-universe'}:
        out.append(f'{name}: jobs must retain all public channels on Unix and Windows')
    for job, spec in jobs.items():
        if spec.get('permissions') != {'contents': 'read'}:
            out.append(f'{name}: job {job} must have only contents: read')
        if 'environment' in spec:
            out.append(f'{name}: job {job} must not name an environment')
        if spec.get('if') is not None or spec.get('continue-on-error') is not None:
            out.append(f'{name}: job {job} must run unconditionally and propagate failures')
        matrix = ((spec.get('strategy') or {}).get('matrix') or {})
        expected = {'portable': {'runner': RUNNERS, 'channel': PORTABLE},
                    'homebrew': {'runner': ['ubuntu-24.04', 'macos-26']},
                    'linux': {'channel': LINUX}, 'windows': {'channel': WINDOWS}, 'r-universe': {}}.get(job)
        if expected is not None and (matrix != expected or spec.get('runs-on') != (
                '${{ matrix.runner }}' if job in ('portable', 'homebrew') else 'windows-2025' if job == 'windows' else 'ubuntu-24.04')):
            out.append(f'{name}: job {job} must retain its accepted channel and runner matrix')
        if job == 'r-universe' and spec.get('container') != R_IMAGE:
            out.append(f'{name}: R-universe must use the pinned Linux R 4.6.1 image')
        channel = '${{ matrix.channel }}' if job in ('portable', 'linux', 'windows') else job
        env = {'VERSION': '${{ inputs.version }}'}
        if job in ('portable', 'linux', 'windows'):
            env['CHANNEL'] = channel
            command = 'python3 sdlc/scripts/install-check "$CHANNEL" "$VERSION"'
        else:
            command = f'python3 sdlc/scripts/install-check {channel} "$VERSION"'
        expected_step = {'name': 'Install and replay', 'env': env, 'run': command}
        if job == 'windows':
            expected_step['shell'] = 'bash'
        steps = spec.get('steps') or []
        if not steps or steps[-1] != expected_step or 'defaults' in spec or 'defaults' in doc:
            out.append(f'{name}: job {job} must end with the exact unconditional install and replay step using version through env')
    def secrets(value):
        if isinstance(value, dict):
            return any(secrets(child) for child in value.values())
        if isinstance(value, list):
            return any(secrets(child) for child in value)
        return isinstance(value, str) and bool(re.search(r'\$\{\{[^}]*\bsecrets\b', value))
    if secrets(doc):
        out.append(f'{name}: must not read a secret')
    return out
