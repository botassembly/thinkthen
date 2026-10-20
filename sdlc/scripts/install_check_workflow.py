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
