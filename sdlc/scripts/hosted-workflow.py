"""Hosted candidate execution boundaries for the existing platform workflows."""
STANDARD = {'ubuntu-latest', 'ubuntu-24.04', 'ubuntu-24.04-arm',
            'macos-15', 'macos-15-intel', 'macos-26', 'windows-2025'}
GUARD = 'python3 sdlc/scripts/hosted-candidate.py checkout "$CANDIDATE_SHA" "$DISPATCH_SHA"'
APPLE = ['sdlc/scripts/release-workflow apple-check "$TARGET" target/apple-packages swift "$SHA"',
         'sdlc/scripts/release-workflow apple-check "$TARGET" target/apple-packages objective-c "$SHA"']


def rules(name, doc):
    out = []
    jobs = doc.get('jobs') or {}
    for label, job in jobs.items():
        if name == 'release.yml' and label in ('build', 'smoke'):
            continue  # The existing release matrix contract checks these exact runners.
        runner = job.get('runs-on')
        if runner is None:  # Reusable jobs and minimal graph fixtures have no runner.
            continue
        matrix = (job.get('strategy') or {}).get('matrix') or {}
        choices = (matrix.get('runner') or [row.get('runner') for row in matrix.get('include', [])]
                   if runner == '${{ matrix.runner }}' else [runner])
        if not choices or any(not isinstance(value, str) or value not in STANDARD for value in choices):
            out.append(f'{name}: job {label} must use only standard free hosted runners')
    if name == 'windows.yml' and 'stage-0' in jobs:
        job = jobs['stage-0']
        steps = job.get('steps') or []
        guard = next((i for i, step in enumerate(steps) if step.get('run') == GUARD
                      and step.get('env') == {'CANDIDATE_SHA': '${{ inputs.candidate_sha }}',
                                              'DISPATCH_SHA': '${{ github.sha }}'}
                      and step.get('if') is None and step.get('continue-on-error') is None), None)
        build = next((i for i, step in enumerate(steps) if 'cargo ' in step.get('run', '')), None)
        if guard is None or build is None or guard >= build:
            out.append(f'{name}: Windows qualification must guard the candidate SHA before building')
    if name == 'release.yml':
        steps = jobs.get('apple-packages', {}).get('steps') or []
        # Minimal graph fixtures do not assemble Apple packages.
        assemble = next((i for i, step in enumerate(steps)
                         if 'release-pack --apple-wrappers' in step.get('run', '')), None)
        if assemble is not None:
            proof = next((i for i, step in enumerate(steps)
                          if step.get('run', '').splitlines() == ['set -eu', *APPLE]
                          and step.get('env') == {'TARGET': '${{ matrix.target }}',
                                                  'SHA': '${{ needs.resolve.outputs.sha }}',
                                                  'THINKTHEN_TEST_PROFILE': 'full'}
                          and step.get('if') is None and step.get('continue-on-error') is None), None)
            upload = next((i for i, step in enumerate(steps)
                           if (step.get('with') or {}).get('name') == 'apple-packages-${{ matrix.target }}'), None)
            if proof is None or upload is None or not assemble < proof < upload:
                out.append(f'{name}: both installed Apple languages must qualify on the resolved SHA before upload')
    return out
