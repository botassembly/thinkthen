#!/usr/bin/env python3
"""Check the actual Windows C execution contexts, before artifacts cross jobs."""
import re

TARGET = 'x86_64-pc-windows-msvc'
CONDITION = f"matrix.target == '{TARGET}'"
SETUP = 'python3 sdlc/scripts/release-msvc.py >> "$GITHUB_ENV"'
CLIPPY = 'cargo clippy --locked --offline --manifest-path libraries/c/Cargo.toml --all-targets -- -D warnings'
TEST = 'cargo test --locked --offline --manifest-path libraries/c/Cargo.toml --no-fail-fast'
PACK = 'CARGO_NET_OFFLINE=true sdlc/scripts/release-pack "$TARGET" release-files command c python typescript first-run'
PLATFORM = 'python3 sdlc/scripts/release-windows-command.py platform release-files "$(sed -n \'s/^version = "\\(.*\\)"$/\\1/p\' crates/thinkthen/Cargo.toml | head -n 1)"'
VERSION = 'version=$(sed -n \'s/^version = "\\(.*\\)"$/\\1/p\' crates/thinkthen/Cargo.toml | head -n 1)'
CHECK = 'python3 sdlc/scripts/release-windows-c.py check "platform/thinkthen-c-$version-x86_64-pc-windows-msvc.zip"'
CONSUME = 'python3 sdlc/scripts/release-windows-c-smoke.py "platform/thinkthen-c-$version-x86_64-pc-windows-msvc.zip"'


def enabled(step, condition=None):
    return (step.get('if') == condition and step.get('continue-on-error') is None
            and step.get('shell') is None and step.get('working-directory') is None)


def rules(name, jobs):
    out = []
    for kind in ('build', 'smoke'):
        job = jobs.get(kind, {})
        if not ((job.get('strategy') or {}).get('matrix') or {}).get('include'):
            continue  # Retain the existing minimal graph fixture.
        steps = job.get('steps', [])
        setup = next((i for i, step in enumerate(steps) if step.get('run') == SETUP
                      and enabled(step, CONDITION)), None)
        if job.get('if') is not None or job.get('continue-on-error') is not None:
            out.append(f'{name}: Windows C {kind} job must execute and propagate failures')
        if kind == 'build':
            build = next((i for i, step in enumerate(steps) if 'release-pack' in step.get('run', '')
                          and enabled(step) and (step.get('env') or {}).get('TARGET') == '${{ matrix.target }}'), None)
            upload = next((i for i, step in enumerate(steps)
                           if (step.get('with') or {}).get('name') == 'platform-${{ matrix.target }}'
                           and enabled(step)), None)
            arm = []
            if build is not None:
                script = steps[build]['run']
                match = re.search(r'if \[ "\$TARGET" = x86_64-pc-windows-msvc \]; then\n(.*?)\n\s*elif ', script, re.S)
                if match and script.lstrip().startswith('set -eu\n'):
                    arm = [line.strip() for line in match[1].splitlines()]
            commands = [
                ('root Clippy', 'cargo clippy --locked --offline --workspace --all-targets -- -D warnings'),
                ('root tests', 'cargo test --locked --offline --no-fail-fast --workspace --all-targets'),
                ('C Clippy', CLIPPY), ('C door tests', TEST), ('C packing', PACK), ('platform verification', PLATFORM)]
            for label, command in commands:
                if arm.count(command) != 1:
                    out.append(f'{name}: Windows {label} must execute in the Windows arm and propagate failure')
            if all(command in arm for _, command in commands):
                if arm != [command for _, command in commands]:
                    out.append(f'{name}: Windows C checks, pack and platform verification must run in order without masking')
            if setup is None or build is None or upload is None or not setup < build < upload:
                out.append(f'{name}: Windows build must select x64 MSVC before C checks and verify before upload')
        else:
            download = next((i for i, step in enumerate(steps)
                             if (step.get('with') or {}).get('name') == 'platform-${{ matrix.target }}'
                             and (step.get('with') or {}).get('path') == 'platform' and enabled(step)), None)
            consume = next((i for i, step in enumerate(steps) if CHECK in step.get('run', '') or
                            CONSUME in step.get('run', '')), None)
            lines = steps[consume].get('run', '').splitlines() if consume is not None else []
            for label, command in [('downloaded C validation', CHECK), ('downloaded C consumption', CONSUME)]:
                if lines.count(command) != 1 or consume is None or not enabled(steps[consume], CONDITION):
                    out.append(f'{name}: Windows {label} must execute after download and propagate failure')
            if lines != ['set -eu', VERSION, CHECK, CONSUME]:
                out.append(f'{name}: downloaded Windows C validation and consumption must run in order without masking')
            if setup is None or download is None or consume is None or not (setup < consume and download < consume):
                out.append(f'{name}: Windows smoke must select x64 MSVC and download before archive consumption')
    return out


def standalone(name, jobs):
    job = jobs.get('stage-0', {})
    steps = job.get('steps', [])
    setup = next((i for i, step in enumerate(steps) if step.get('run') == SETUP and enabled(step)), None)
    checks = [i for i, step in enumerate(steps) if step.get('working-directory') == 'libraries/c'
              and step.get('run', '').startswith(('cargo clippy ', 'cargo test '))]
    if setup is None or not checks or setup >= min(checks):
        return [f'{name}: standalone Windows must select x64 MSVC before C checks']
    return []
