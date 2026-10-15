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
        choices = (list(matrix.get('runner') or []) +
                   [row['runner'] for row in matrix.get('include', []) if 'runner' in row]
                   if runner == '${{ matrix.runner }}' else [runner])
        if runner == "${{ inputs.platform || 'ubuntu-24.04' }}":
            dispatch = (doc.get('on', doc.get(True)) or {}).get('workflow_dispatch') or {}
            platform = (dispatch.get('inputs') or {}).get('platform') or {}
            choices = platform.get('options', []) if platform.get('type') == 'choice' else []
            if platform.get('default') not in choices:
                choices = []
        if not choices or any(not isinstance(value, str) or value not in STANDARD for value in choices):
            out.append(f'{name}: job {label} must use only standard free hosted runners')
    if name == 'windows.yml' and 'stage-0' in jobs:
        job = jobs['stage-0']
        dispatch = (doc.get('on', doc.get(True)) or {}).get('workflow_dispatch') or {}
        candidate = dispatch.get('inputs', {}).get('candidate_sha') or {}
        if candidate.get('type') != 'string' or candidate.get('required') is not True:
            out.append(f'{name}: Windows qualification must require its candidate SHA input')
        steps = job.get('steps') or []
        guard = next((i for i, step in enumerate(steps) if step.get('run') == GUARD and step.get('id') == 'candidate'
                      and step.get('env') == {'CANDIDATE_SHA': '${{ inputs.candidate_sha }}',
                                              'DISPATCH_SHA': '${{ github.sha }}'}
                      and step.get('if') is None and step.get('continue-on-error') is None), None)
        build = next((i for i, step in enumerate(steps) if 'cargo ' in step.get('run', '')), None)
        if guard is None or build is None or guard >= build:
            out.append(f'{name}: Windows qualification must guard the candidate SHA before building')
        elif job.get('continue-on-error') is not None or any(
                step.get('if') is not None and step['if'] not in (
                    "${{ !cancelled() && steps.candidate.outcome == 'success' }}",
                    "${{ always() && steps.candidate.outcome == 'success' && steps.cache.outputs.cache-hit != 'true' }}")
                for step in steps[guard + 1:]):
            out.append(f'{name}: Windows steps must stop after a failed candidate SHA guard')
    if name == 'windows.yml' and 'recognition-cancel' in jobs:
        job = jobs['recognition-cancel']
        command = ('cargo test --locked --offline -p thinkthen --test request_session '
                   'recognize::cancellation_stops_waiting_duplicate_recognition_without_extra_sends '
                   '-- --exact --nocapture')
        steps = job.get('steps') or []
        guard = next((i for i, step in enumerate(steps)
                      if step.get('run') == GUARD and step.get('id') == 'candidate'
                      and step.get('env') == {'CANDIDATE_SHA': '${{ inputs.candidate_sha }}',
                                              'DISPATCH_SHA': '${{ github.sha }}'}
                      and step.get('if') is None and step.get('continue-on-error') is None), None)
        build = next((i for i, step in enumerate(steps) if 'cargo ' in step.get('run', '')), None)
        lane = {'name': 'Require the standard Windows lane',
                'env': {'LANE': '${{ inputs.lane }}'}, 'run': 'test "$LANE" = windows'}
        standard = jobs.get('stage-0', {}).get('steps', [])
        candidate = next((i for i, step in enumerate(standard) if step.get('id') == 'candidate'), None)
        if lane not in standard or candidate is None or standard.index(lane) >= candidate:
            out.append(f'{name}: the standard Windows lane must reject unknown input before its candidate SHA guard')
        if (job.get('if') != "inputs.lane == 'recognition-cancel'"
                or job.get('runs-on') != 'windows-2025'
                or job.get('environment') is not None
                or job.get('continue-on-error') is not None
                or job.get('strategy') is not None
                or (doc.get('permissions') or {}) != {'contents': 'read'}
                or job.get('env', {}).get('CARGO_BUILD_JOBS') != '4'
                or guard is None or build is None or guard >= build
                or any(step.get('if') is not None or step.get('continue-on-error') is not None for step in steps)
                or [step['run'] for step in steps if 'cargo test' in step.get('run', '')] != [command]):
            out.append(f'{name}: recognition cancellation must run only its fixed Rust scenario on the guarded standard host')
    if name == 'release.yml':
        apple = jobs.get('apple-packages', {})
        steps = apple.get('steps') or []
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
                           if (step.get('with') or {}).get('name') == 'apple-packages-${{ matrix.target }}'
                           and step.get('if') is None and step.get('continue-on-error') is None), None)
            if (proof is None or upload is None or not assemble < proof < upload
                    or apple.get('continue-on-error') is not None):
                out.append(f'{name}: both installed Apple languages must qualify on the resolved SHA before upload')
    return out
