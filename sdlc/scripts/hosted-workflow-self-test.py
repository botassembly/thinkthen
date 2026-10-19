#!/usr/bin/env python3
"""Plant hosted candidate routing and parameter failures at the actual entry points."""
from copy import deepcopy
import hashlib
import os
import io
import json
import runpy
import shutil
import signal
import tarfile
from pathlib import Path
import subprocess
import sys
import tempfile
import time
from types import SimpleNamespace
from unittest.mock import patch

import yaml

REPO = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(REPO))
from conformance.children.children import child_env
from conformance.source_fixtures import stage_checkout


def host_setup_calls(target, smoke, uv_line="uv 0.9.17 (0a1b2c3d4 2025-12-09)", old_bash=False):
    """Run the real setup branch while a temporary PATH records its host calls.

    A stray uv sits on PATH and fails if run, as on a runner image with its own uv.
    The pinned uv lives where pip put it and prints uv_line.
    """
    with tempfile.TemporaryDirectory(prefix="thinkthen-host-setup-") as tmp:
        root = Path(tmp).resolve()
        bin_dir = root / "bin"
        bin_dir.mkdir()
        route = bin_dir / "route"
        route.write_text("""#!/bin/sh
case ${0##*/} in
rustc) case $1 in
  -vV) printf 'host: %s\\n' "$FAKE_TARGET" ;;
  --version) echo 'rustc 1.95.0 (59807616e 2026-04-14)' ;;
  --print) printf '%s/toolchains/1.95.0\\n' "$RUSTUP_HOME" ;;
  esac ;;
mktemp) exec /usr/bin/mktemp -d "$FAKE_ROOT/tmp.XXXXXXXX" ;;
sha256sum) echo 'unexpected sha256sum call' >&2; exit 94 ;;
cargo) [ "$1 $2" != 'deny --version' ] || echo 'cargo-deny 0.19.4' ;;
rustup) [ "$1 $2" != 'target add' ] || echo "rustup $*" >> "$FAKE_LOG" ;;
maturin) echo 'maturin 1.15.0' ;;
npm) case $* in "ci --no-audit --no-fund --silent --prefix $FAKE_ROOT/"*) echo npm-seed >> "$FAKE_LOG" ;;
  "install --no-audit --no-fund --no-package-lock --prefix $FAKE_ROOT/runner/thinkthen-generators prettier@3.6.2") ;;
  *) echo "unexpected npm call: $*" >&2; exit 94 ;; esac ;;
python3) [ "$1" != databases/duckdb/tools/source_hosts.py ] || echo duckdb-source-hosts >> "$FAKE_LOG"
  # Keep the real manifest reader while other Python commands remain host stubs.
  if [ "$1" = - ] && [ "${2:-}" = "$FAKE_MANIFEST" ]; then exec "$FAKE_PYTHON" "$@"; fi
  case " $* " in *' uv==0.9.17 '*) touch "$FAKE_UV_READY"; echo uv-install >> "$FAKE_LOG" ;; esac
  case $1 in -c) case $2 in *find_uv_bin*) echo "$FAKE_ROOT/pinned/uv" ;; esac ;; esac ;;
python3.14) exit 1 ;;
sudo) case "$*" in
  'apt-get update') echo pg-index >> "$FAKE_LOG" ;;
  'apt-get install -y postgresql-server-dev-16') echo pg-host >> "$FAKE_LOG" ;;
  'apt-get install -y r-base-dev libgtk-3-dev') touch "$FAKE_ROOT/r-ready"; echo r-host >> "$FAKE_LOG" ;;
  *) echo "unexpected sudo call: $*" >&2; exit 94 ;; esac ;;
bash) if [ "$1" = libraries/r/tools/setup.sh ]; then
    [ -f "$FAKE_ROOT/r-ready" ] || { echo 'R dependency setup preceded R host readiness' >&2; exit 92; }
    echo r-dependencies >> "$FAKE_LOG"
  else
    echo bash-check >> "$FAKE_LOG"; [ -z "$FAKE_OLD_BASH" ]
  fi ;;
python3.13) [ "$1" != --version ] || {
  touch "$FAKE_PY313_READY"; echo py313-ready >> "$FAKE_LOG"; echo 'Python 3.13.0';
} ;;
uv) case $0 in "$FAKE_ROOT/pinned/uv") ;; *) echo 'a stray uv ran' >&2; exit 93 ;; esac
  [ -f "$FAKE_UV_READY" ] || { echo 'uv was not installed' >&2; exit 91; }
  [ "$1" != --version ] || { echo uv-check >> "$FAKE_LOG"; echo "$FAKE_UV_LINE"; }
  if [ "$1" = venv ]; then
    case $4 in */python3.14)
      echo sqlite-venv >> "$FAKE_LOG"
      mkdir -p "$5/bin"
      ln -s "$FAKE_ROOT/openssl3/bin/python3.14" "$5/bin/python" ;;
    *) echo "seed $4" >> "$FAKE_LOG" ;; esac
  fi
  case $* in "pip install --quiet --python $FAKE_ROOT/.cache/thinkthen-toolchains/sqlite-python-3.14/bin/python jsonschema==4.25.1")
    echo sqlite-jsonschema >> "$FAKE_LOG" ;; esac ;;
uname) case $1 in -s) case $FAKE_TARGET in *apple*) echo Darwin ;; *) echo Linux ;; esac ;;
  -m) case $FAKE_TARGET in aarch64*) echo arm64 ;; *) echo x86_64 ;; esac ;; esac ;;
xcrun) echo "$FAKE_ROOT/sdk" ;;
brew) case "$*" in "install pkgconf jq openssl@$RUBY_OPENSSL_SERIES") touch "$FAKE_ROOT/openssl-ready" ;;
  "--prefix openssl@3") echo "$FAKE_ROOT/openssl-default"; exit 0 ;; esac
  case $1 in --cache) for formula in "$@"; do :; done; echo "$FAKE_ROOT/$formula" ;; --prefix) echo "$FAKE_ROOT/openssl3" ;; esac ;;
shasum) PG_MAJOR=${3##*postgresql@}; . databases/postgresql/runtime-darwin.env
  case $FAKE_TARGET in aarch64*) pin=$PG_BOTTLE_ARM64_SEQUOIA ;;
    *) pin=${PG_BOTTLE_X86_64_SONOMA:-$PG_SOURCE_SHA256} ;; esac
  printf '%s  %s\\n' "$pin" "$3" ;;
sh) case $1 in databases/duckdb/tools/setup.sh)
      [ -f "$FAKE_UV_READY" ] && [ -f "$FAKE_PY313_READY" ] || {
        echo 'DuckDB setup preceded uv or Python 3.13 readiness' >&2; exit 92;
      } ;;
    esac
    printf '%s %s\\n' "$1" "$2${3:+ $3}" >> "$FAKE_LOG" ;;
esac
""")
        route.chmod(0o755)
        openssl = root / "openssl3/bin/openssl"
        openssl.parent.mkdir(parents=True)
        openssl.write_text('#!/bin/sh\n[ -f "$FAKE_ROOT/openssl-ready" ] || exit 91\necho "OpenSSL $FAKE_OPENSSL_VERSION"\n')
        openssl.chmod(0o755)
        for name, line in (("openssl-default/bin/openssl", "OpenSSL 3.5.9"),
                           ("bin/openssl", "OpenSSL 4.0.3")):
            other = root / name
            other.parent.mkdir(parents=True, exist_ok=True)
            other.write_text(f"#!/bin/sh\necho '{line}'\n")
            other.chmod(0o755)
        sqlite_python = root / "openssl3/bin/python3.14"
        sqlite_python.write_text('#!/bin/sh\necho sqlite-python-check >> "$FAKE_LOG"\n')
        sqlite_python.chmod(0o755)
        for name in ("rustc", "rustup", "cargo", "maturin", "npm", "uv", "uname", "sh", "xcrun", "brew", "shasum",
                     "python3", "python3.12", "python3.13", "python3.14", "bash", "sudo", "mktemp", "sha256sum"):
            (bin_dir / name).symlink_to(route.name)
        (root / "pinned").mkdir()
        (root / "pinned/uv").symlink_to(route)
        sdk = root / 'sdk'
        sdk.mkdir()
        # Apple's git launcher reads the selected SDK even though all host work is stubbed.
        (sdk / 'SDKSettings.plist').write_text(
            '<?xml version="1.0"?><plist version="1.0"><dict>'
            '<key>Version</key><string>15.0</string></dict></plist>')
        manifest = REPO / 'sdlc/tools.json'
        openssl_version = json.loads(manifest.read_text())['tools']['ruby']['openssl_version']
        log = root / "calls"
        env = os.environ.copy()
        env.update(PATH=f"{bin_dir}:/usr/bin:/bin", HOME=str(root), RUSTUP_HOME=str(root / "rustup"),
                   TMPDIR=str(root), FAKE_TARGET=target, FAKE_LOG=str(log), FAKE_ROOT=str(root),
                   FAKE_UV_READY=str(root / "uv-ready"), FAKE_PY313_READY=str(root / "py313-ready"),
                   FAKE_UV_LINE=uv_line, FAKE_OLD_BASH="1" if old_bash else "", GITHUB_PATH=str(root / "github-path"),
                   GITHUB_ENV=str(root / "github-env"), RUNNER_TEMP=str(root / "runner"),
                   FAKE_PYTHON=sys.executable, FAKE_MANIFEST=str(manifest),
                   FAKE_OPENSSL_VERSION=openssl_version,
                   # The hosted reader is replaced by 0620; supply its old variable only in this routing fixture.
                   RUBY_OPENSSL_SERIES=openssl_version.rsplit('.', 1)[0],
                   RELEASE_SMOKE="1" if smoke else "0")
        result = subprocess.run(["/bin/sh", str(REPO / "sdlc/scripts/release-workflow"), "host-setup", target],
                                env=env, capture_output=True, text=True, timeout=20, check=False)
        calls = [line.replace(str(root), "$ROOT").replace(str(REPO), "$REPO") for line in log.read_text().splitlines()] if log.exists() else []
        pgrx = root / ".pgrx/config.toml"
        if pgrx.exists():
            calls.append("pgrx " + pgrx.read_text().replace(str(root), "$ROOT").replace("\n", " ").strip())
        handoff = root / "github-path"
        if handoff.exists():
            calls.append(f"path {handoff.read_text().strip().replace(str(root), '$ROOT')}")
        handoff = root / "github-env"
        if handoff.exists():
            calls.append(f"env {handoff.read_text().strip().replace(str(root), '$ROOT')}")
        return result, calls


def shared_host_routing_cases():
    for target in ('x86_64-unknown-linux-gnu', 'aarch64-unknown-linux-gnu',
                   'aarch64-apple-darwin', 'x86_64-apple-darwin'):
        for smoke in (False, True):
            result, calls = host_setup_calls(target, smoke)
            assert result.returncode == 0, (target, smoke, calls, result.stderr)
            duckdb = [call for call in calls if call.startswith('databases/duckdb/tools/setup.sh ')]
            expected = (['databases/duckdb/tools/setup.sh --fetch $REPO/duckdb-hosts'] if smoke else
                        ['databases/duckdb/tools/setup.sh --fetch'] if 'apple' in target else [])
            assert duckdb == expected, (target, smoke, duckdb)
            assert 'duckdb-source-hosts' not in calls, (target, smoke, calls)


def cache_refusal_cases():
    checker = runpy.run_path(str(REPO / 'sdlc/scripts/workflows'))
    folder = REPO / '.github/workflows'
    # Exercise the entire tracked workflow set: approved dependency caches in
    # other workflows must not inherit the release-only cache ban.
    assert checker['check'](folder) == []
    release = yaml.safe_load((folder / 'release.yml').read_text())
    assert checker['release']('release.yml', release) == []
    for job in release['jobs']:
        for cache in ({'uses': 'actions/cache@' + 'a' * 40,
                       'with': {'path': 'duckdb-hosts', 'key': 'duckdb-hosts'}},
                      {'uses': 'actions/setup-node@' + 'a' * 40, 'with': {'cache': 'npm'}}):
            document = deepcopy(release)
            document['jobs'][job].setdefault('steps', []).insert(1, cache)
            assert checker['release']('release.yml', document) == [f'release.yml: job {job} uses a cache']
    for name in ('gate.yml', 'pages.yml', 'windows.yml'):
        document = yaml.safe_load((folder / name).read_text())
        assert checker['every'](name, document, '', folder) == []
        job = next(iter(document['jobs']))
        document['jobs'][job].setdefault('steps', []).append({'uses': 'actions/cache@v5'})
        assert checker['every'](name, document, '', folder) == [
            f'{name}: job {job} uses actions/cache@v5, which is not pinned to a commit']


def linux_work_destination_cases():
    for name, sudo, free, destination in (
            ('roomy', 0, 900, '/mnt/thinkthen-container'),
            ('no sudo', 1, 900, 'FALLBACK'),
            ('small', 0, 50, 'FALLBACK'),
            ('missing', 0, None, 'FALLBACK')):
        with tempfile.TemporaryDirectory(prefix='thinkthen-linux-work-') as temporary:
            root = Path(temporary).resolve()
            log = root / 'calls'
            capacity = 'exit 1' if free is None else f'free={free}'
            df = ('echo "df $*" >> "$FAKE_LOG"; case $2 in /mnt) %s ;; *) free=100 ;; esac; '
                  'printf "Filesystem 1024-blocks Used Available Capacity Mounted\\nx 1 1 %%s 1%%%% %%s\\n" '
                  '"$free" "$2"' % capacity)
            for tool, body in (('df', df), ('sudo', f'echo "sudo $*" >> "$FAKE_LOG"; exit {sudo}')):
                path = root / tool
                path.write_text('#!/bin/sh\n' + body + '\n')
                path.chmod(0o755)
            result = subprocess.run(['/bin/sh', str(REPO / 'sdlc/scripts/release-workflow'),
                                     'linux-work', 'FALLBACK'],
                                    env={**os.environ, 'PATH': f'{root}:/usr/bin:/bin',
                                         'FAKE_LOG': str(log)}, capture_output=True, text=True, timeout=20)
            calls = log.read_text().splitlines() if log.exists() else []
            sudo_calls = [call for call in calls if call.startswith('sudo ')]
            assert result.returncode == 0 and result.stdout.strip() == destination, (
                name, result.returncode, result.stdout, result.stderr)
            assert calls[:1] == ['df -h / /mnt'], (name, calls)
            assert not any(' rm ' in f' {call} ' for call in calls), (name, calls)
            assert bool(sudo_calls) == bool(free and free > 100), (name, calls)


def linux_tool_destination_case():
    gate = yaml.safe_load((REPO / '.github/workflows/gate.yml').read_text())
    step = next(step['run'] for step in gate['jobs']['linux-release']['steps']
                if step.get('name') == 'Install existing pinned Linux host and consumer tools')
    block, checksum = step.split('command -v mustmatch\n', 1)[1].split('printf ', 1)
    assert '. sdlc/scripts/fetch.sh\n' in block and 'fetch_url "$HOME/.local/bin/jq" ' in block
    assert '"$JQ_SHA256" "$HOME/.local/bin/jq" | sha256sum --check --strict' in checksum.splitlines()[0]
    url = 'https://github.com/jqlang/jq/releases/download/$JQ_VERSION/jq-linux-amd64'
    with tempfile.TemporaryDirectory(prefix='thinkthen-linux-tool-') as temporary:
        root = Path(temporary)
        payload = root / 'jq-fixture'
        payload.write_bytes(b'pinned jq fixture\n')
        home = root / 'home'
        env = {'PATH': os.environ['PATH'], 'HOME': str(home),
               'JQ_SHA256': hashlib.sha256(payload.read_bytes()).hexdigest()}
        assert not (home / '.local/bin').exists()
        # Keep missing-parent failures deterministic without waiting for retry pauses.
        command = 'sleep() { :; }\n' + block.replace(url, payload.as_uri())
        result = subprocess.run(['/bin/sh', '-ec', command],
                                cwd=REPO, env=env, capture_output=True, text=True, timeout=5)
        assert result.returncode == 0, (result.returncode, result.stdout, result.stderr)
        assert (home / '.local/bin/jq').read_bytes() == payload.read_bytes()
        assert hashlib.sha256((home / '.local/bin/jq').read_bytes()).hexdigest() == env['JQ_SHA256']


def linux_nested_tools_case():
    gate = yaml.safe_load((REPO / '.github/workflows/gate.yml').read_text())
    steps = gate['jobs']['linux-release']['steps']
    selected = next(step['run'] for step in steps
                    if step.get('name') == 'Select existing Go, Swift, Zig, PHP, Dart and native language tools')
    handoff = next(line for line in selected.splitlines()
                   if '"$THINKTHEN_SCALA_HOME/bin"' in line and '>> "$GITHUB_PATH"' in line)
    workflow = (REPO / 'sdlc/scripts/release-workflow').read_text()
    host_setup = workflow.split('host-setup)\n', 1)[1].split('\nresolve)', 1)[0]
    installed = host_setup.rsplit('if [ "${RELEASE_SMOKE:-0}" = 1 ]; then', 1)[1]
    assert 'uv pip install --python "$generator_tools/python/bin/python" ' in next(
        line for line in installed.splitlines() if "ruff==" in line)
    assert '--prefix "$generator_tools" ' in next(
        line for line in installed.splitlines() if "prettier@" in line)
    release = yaml.safe_load((REPO / '.github/workflows/release.yml').read_text())
    smoke = release['jobs']['smoke']
    setup = next(step for step in smoke['steps'] if 'host-setup' in step.get('run', ''))
    assert 'if' not in setup and setup['env']['RELEASE_SMOKE'] == '1'
    assert 'host-setup "$TARGET"' in next(step['run'] for step in steps
        if step.get('name') == 'Install existing pinned Linux host and consumer tools')
    formatters = '\n'.join(line for line in installed.splitlines()
                           if line.strip().startswith(('mkdir -p "$generator_tools/bin"',
                                                       'ln -s "$generator_tools/python/bin/ruff"',
                                                       'printf ')) and '$generator_tools/' in line)
    with tempfile.TemporaryDirectory(prefix='thinkthen-nested-tools-') as temporary:
        root = Path(temporary).resolve()
        path_file = root / 'github-path'
        path_file.touch()
        env = {'PATH': os.environ['PATH'], 'HOME': str(root), 'GITHUB_PATH': str(path_file)}
        tools = {'THINKTHEN_JDK_HOME': 'javac', 'THINKTHEN_KOTLIN_HOME': 'kotlinc',
                 'THINKTHEN_SCALA_HOME': 'scalac'}
        for variable, binary in tools.items():
            home = root / variable
            (home / 'bin').mkdir(parents=True)
            executable = home / 'bin' / binary
            executable.write_text('#!/bin/sh\nexit 0\n')
            executable.chmod(0o755)
            env[variable] = str(home)
        env['generator_tools'] = str(root / 'work')
        for relative, binary in (('python/bin', 'ruff'), ('node_modules/.bin', 'prettier')):
            folder = root / 'work' / relative
            folder.mkdir(parents=True)
            executable = folder / binary
            executable.write_text('#!/bin/sh\nexit 0\n')
            executable.chmod(0o755)
        subprocess.run(['/bin/sh', '-ec', handoff + '\n' + formatters], env=env, check=True, timeout=5)
        env['PATH'] = os.pathsep.join(path_file.read_text().splitlines() + [env['PATH']])
        # Use the real nested gate boundary: homes are removed, PATH must still select each SDK.
        command = '. sdlc/scripts/allow-list\n'
        command += '\n'.join(f'[ -z "${{{variable}+present}}" ]; [ "$(command -v {binary})" = "{env[variable]}/bin/{binary}" ]'
                             for variable, binary in tools.items())
        for relative, binary in (('bin', 'ruff'), ('node_modules/.bin', 'prettier')):
            command += f'\n[ "$(command -v {binary})" = "{root}/work/{relative}/{binary}" ]\n'
        subprocess.run(['/bin/sh', '-ec', command], cwd=REPO, env=env, check=True, timeout=5)
        # Setup and qualification use different homes; offline npm must read
        # the cache setup actually populated through the real child boundary.
        setup = next(step['run'] for step in steps
                     if step.get('name') == 'Require the frozen dispatch commit and candidate')
        cache_handoff = next(line for line in setup.splitlines() if "printf 'npm_config_cache=" in line)
        env_file = root / 'github-env'
        subprocess.run(['/bin/sh', '-ec', cache_handoff], env={**env, 'work': str(root),
            'STAGE': 'core', 'GITHUB_ENV': str(env_file)}, check=True, timeout=5)
        env.update(line.split('=', 1) for line in env_file.read_text().splitlines())
        env.update(GITHUB_ACTIONS='true', RUNNER_ENVIRONMENT='github-hosted',
                   GITHUB_TOKEN='fake-hosted-filter', PRIVATE_SERVICE_KEY='fake-hosted-filter')
        qualification = next(step['run'] for step in steps if step.get('name') ==
                             'Run the selected existing gates and retain their actual exits')
        selection = qualification[qualification.index("env = child_env("):qualification.index("for key in ('THINKTHEN_DOTNET'")].rstrip() + '\n'
        expected_cache = str(root / 'core/npm-cache')
        hosted = ('assert env["GITHUB_ACTIONS"] == "true"\n'
                  'assert env["RUNNER_ENVIRONMENT"] == "github-hosted"\n'
                  'assert "GITHUB_TOKEN" not in env and "PRIVATE_SERVICE_KEY" not in env\n')
        probe = ('import os,sys\n'
                 f'sys.path.insert(0, {str(REPO / "databases/duckdb/tools")!r})\n'
                 'from source_hosts import tool_environment\n'
                 'env = os.environ\n' + hosted +
                 'os.environ.update(GITHUB_TOKEN="fake-hosted-filter", PRIVATE_SERVICE_KEY="fake-hosted-filter")\n'
                 'env = tool_environment()\n' + hosted)
        script = ('import subprocess,sys\nfrom pathlib import Path\nfrom conformance.children.children import child_env\n'
                  f'work = Path({str(root)!r})\n' + selection +
                  hosted + f'assert env["npm_config_cache"] == {expected_cache!r}\n'
                  'subprocess.run(["sh", "-ec", '
                  '". sdlc/scripts/allow-list; test -d \\\"$npm_config_cache\\\""], '
                  'env=env, check=True)\n'
                  f'subprocess.run([sys.executable, {str(REPO / "databases/duckdb/tools/cache_lock.py")!r}, '
                  f'{str(root / "cache.lock")!r}, sys.executable, "-c", {probe!r}], '
                  'env=dict(env, GITHUB_TOKEN="fake-hosted-filter", PRIVATE_SERVICE_KEY="fake-hosted-filter"), check=True)\n')
        Path(expected_cache).mkdir(parents=True)
        subprocess.run([sys.executable, '-c', script], cwd=REPO, env=env, check=True, timeout=5)


def linux_failed_phase_evidence_case():
    gate = yaml.safe_load((REPO / '.github/workflows/gate.yml').read_text())
    command = next(step['run'] for step in gate['jobs']['linux-release']['steps']
                   if step.get('name') == 'Run the selected existing gates and retain their actual exits')
    # Exercise the actual driver, replacing expensive gate commands with a child
    # that writes the same evidence boundary and exits with a real process status.
    driver = command[command.index('result = ('):].rsplit('\nPY', 1)[0]
    read_text = Path.read_text
    def pressure_text(path, *args, **kwargs):
        return 'some avg10=0.00\n' if str(path) == '/proc/pressure/io' else read_text(path, *args, **kwargs)
    for preparation, code, evidence in ((0, 0, True), (0, 1, True), (0, 23, True),
                                        (0, 23, False), (23, None, False)):
        with tempfile.TemporaryDirectory(prefix='thinkthen-linux-phase-') as temporary:
            root = Path(temporary)
            out = root / 'out'; out.mkdir()
            subprocess.run(['git', 'init', '-q', str(root)], check=True)
            subprocess.run(['git', '-C', str(root), '-c', 'user.name=Fixture',
                            '-c', 'user.email=fixture@example.invalid', 'commit',
                            '--allow-empty', '-qm', 'Initialize fixture'], check=True)
            sha = subprocess.check_output(['git', '-C', str(root), 'rev-parse', 'HEAD'], text=True).strip()
            invoked = []
            def child(command, env, deadline):
                invoked.append(command)
                if command[:2] == ['sh', '-c']:
                    return subprocess.run([sys.executable, '-c', f'raise SystemExit({preparation})'], env=env).returncode
                if command[:2] != ['sh', 'sdlc/scripts/test-full-cases']:
                    return subprocess.run([sys.executable, '-c', 'raise SystemExit(0)'], env=env).returncode
                script = ('from pathlib import Path\n'
                          "p=Path('target/parity'); p.mkdir(parents=True)\n"
                          "(p/'consumer.log').write_text('child diagnostic\\n')\n"
                          "(p/'matrix.json').write_text('[{\"exit\": %d}]\\n')\n" % code) if evidence else ''
                return subprocess.run([sys.executable, '-c', script + f'raise SystemExit({code})'], env=env).returncode
            namespace = dict(root=root, out=out, sha=sha, tag='rc/fixture', stage='functional',
                             env={}, admission=SimpleNamespace(run_owned=child, child_pids=lambda: []),
                             Path=Path, os=os, subprocess=subprocess, signal=signal, time=time,
                             shutil=shutil, json=json, hashlib=hashlib)
            previous = Path.cwd()
            try:
                os.chdir(root)
                with patch.object(Path, 'read_text', new=pressure_text):
                    try:
                        exec(compile(driver, 'gate.yml', 'exec'), namespace)
                    except RuntimeError as error:
                        failed, status = ('polars-prepare', preparation) if preparation else ('fullcases', code)
                        assert status != 0 and str(error) == f'{failed} failed: {status}', error
                    else:
                        assert code == 0
            finally:
                os.chdir(previous)
            result = json.loads((out / 'result.json').read_text())
            assert result['state'] == ('failed' if preparation or code else 'completed'), result
            assert result['phases']['polars-prepare']['command_exit'] == preparation
            assert result['phases']['polars-prepare']['exit'] == preparation
            if not preparation:
                assert result['phases']['fullcases']['command_exit'] == code
                assert result['phases']['fullcases']['exit'] == code
            phases = {'cli-fresh', 'polars-prepare'}
            if not preparation:
                phases.add('fullcases')
                if not code: phases.update(('package', 'stress'))
            assert set(result['phases']) == phases
            assert len(invoked) == len(phases)
            assert invoked[1][:2] == ['sh', '-c']
            assert '. sdlc/scripts/full-compiler; unset CARGO_INCREMENTAL' in invoked[1][2]
            assert 'CARGO_TARGET_DIR="$PWD/target/polars"' in invoked[1][2]
            assert invoked[1][2].index('--test polars --no-run') < invoked[1][2].index('cargo build')
            assert (out / 'fullcases-parity').exists() == evidence
            if evidence:
                assert (out / 'fullcases-parity/consumer.log').read_text() == 'child diagnostic\n'
                assert json.loads((out / 'fullcases-parity/matrix.json').read_text()) == [{'exit': code}]


def linux_source_staging_case():
    gate = yaml.safe_load((REPO / '.github/workflows/gate.yml').read_text())
    command = next(step['run'] for step in gate['jobs']['linux-release']['steps']
                   if step.get('name') == 'Run the selected existing gates and retain their actual exits')
    # Execute the actual inline staging with generated output from earlier checks present.
    start = command.rfind('\n', 0, command.index('# Fresh stage follows')) + 1
    end = command.rfind('\n', 0, command.index("r=root/'libraries/r/target'")) + 1
    driver = command[start:end]
    import textwrap
    driver = textwrap.dedent(driver)
    with tempfile.TemporaryDirectory(prefix='thinkthen-linux-source-') as temporary:
        root = Path(temporary).resolve()
        archived = subprocess.check_output(['git', 'archive', 'HEAD', 'libraries/dart',
                                            'libraries/python/thinkthen', 'specification/fixtures/images/red.png',
                                            'specification/request.schema.json',
                                            'crates/thinkthen/src/public/results/complete.schema.json'], cwd=REPO)
        with tarfile.open(fileobj=io.BytesIO(archived)) as source:
            source.extractall(root, filter='data')
        (root / 'sdlc').mkdir()
        (root / 'sdlc/generators').symlink_to(REPO / 'sdlc/generators', target_is_directory=True)
        flutter = root / 'libraries/dart/flutter'
        (flutter / 'linked_readme.md').symlink_to('README.md')
        (root / 'libraries/dart/lib/linked_dart.dart').symlink_to('thinkthen_dart.dart')
        (root / 'libraries/dart/checks/session_consumer/bin/linked_main.dart').symlink_to('main.dart')
        (root / 'libraries/python/thinkthen/linked_init.py').symlink_to('__init__.py')
        subprocess.run(['git', 'init', '-q', str(root)], check=True)
        subprocess.run(['git', '-C', str(root), 'add', '-f', 'libraries', 'specification'], check=True)
        loop = flutter / 'example/linux/flutter/ephemeral/.plugin_symlinks/thinkthen_flutter'
        loop.parent.mkdir(parents=True, exist_ok=True)
        loop.symlink_to(flutter, target_is_directory=True)
        for relative in ('libraries/dart/checks/session_consumer/bin', 'libraries/python/thinkthen'):
            folder = root / relative
            (folder / 'generated-loop').symlink_to(folder, target_is_directory=True)
            (folder / 'stale.so').write_bytes(b'old native output')
        native = root / 'target/debug/libthinkthen_c.so'
        native.parent.mkdir(parents=True)
        native.write_bytes(b'current native')
        (native.parent / 'lib_thinkthen.so').write_bytes(b'current Python native')
        (native.parent / 'libthinkthen_ruby.so').write_bytes(b'current Ruby native')
        (native.parent / 'libthinkthen_typescript.so').write_bytes(b'current Node native')
        for language in ('ruby', 'typescript'):
            (root / 'libraries' / language / 'target').mkdir(parents=True)
        invoked = []
        def run(name, args, **kwargs):
            invoked.append(name)
            if name == 'dart-stage':
                subprocess.run([sys.executable, *args[1:]], cwd=root,
                               env=dict(os.environ, PYTHONDONTWRITEBYTECODE='1'), check=True)
                assert (root / 'libraries/dart/target/core-wave/bin/linked_main.dart').is_symlink()
        import importlib.util
        namespace = dict(root=root, native=native, run=run, env=dict(TT_DART='dart', TT_FLUTTER='flutter',
                         RUBY='ruby', PYTHON_HOST=sys.executable), Path=Path, shutil=shutil,
                         importlib=importlib)
        previous = Path.cwd()
        try:
            os.chdir(root)
            exec(compile(driver, 'gate.yml', 'exec'), namespace)
        finally:
            os.chdir(previous)
        staged = root / 'libraries/dart/target/core-wave-flutter'
        for source, destination in (
                (flutter, staged),
                (root / 'libraries/dart/checks/session_consumer/bin', root / 'libraries/dart/target/core-wave/bin'),
                (root / 'libraries/python/thinkthen', root / 'libraries/python/target/core-wave/thinkthen')):
            for item in subprocess.check_output(['git', '-C', str(source), 'ls-files', '-z'], text=True).split('\0'):
                if item:
                    original, copied = source / item, destination / item
                    assert copied.is_symlink() == original.is_symlink(), item
                    if original.is_symlink():
                        assert os.readlink(copied) == os.readlink(original), item
                    elif Path(item).name != 'pubspec.yaml':
                        assert copied.read_bytes() == original.read_bytes(), item
        package_link = root / 'libraries/dart/target/core-wave/lib/linked_dart.dart'
        assert package_link.is_symlink() and os.readlink(package_link) == 'thinkthen_dart.dart'
        assert not (staged / 'example/linux/flutter/ephemeral').exists()
        for relative in ('libraries/dart/target/core-wave/bin', 'libraries/python/target/core-wave/thinkthen'):
            assert not (root / relative / 'generated-loop').exists()
            assert not (root / relative / 'stale.so').exists()
        assert (root / 'libraries/dart/target/core-wave/bin/shipped_example.dart').is_file()
        assert (root / 'libraries/dart/target/core-wave/bin/fixture_values_generated.dart').is_file()
        assert (staged / 'example/pubspec_overrides.yaml').is_file()
        assert (root / 'libraries/python/target/core-wave/thinkthen/_thinkthen.so').read_bytes() == b'current Python native'
        assert 'flutter-linux' in invoked and 'python-pandas-polars-import' in invoked


def postgresql_package_case():
    # The package entry point must select its pinned headers without ~/.pgrx.
    with tempfile.TemporaryDirectory(prefix='thinkthen-postgresql-package-') as temporary:
        root = Path(temporary)
        database = root / 'databases/postgresql'
        database.mkdir(parents=True)
        for name in ('package.sh', 'runtime.sh', 'runtime.sha256', 'runtime.url',
                     'pgrx-package-locked.sh', 'Cargo.lock', 'thinkthen.control'):
            shutil.copyfile(REPO / 'databases/postgresql' / name, database / name)
        tools = root / 'bin'
        tools.mkdir()
        pg_config = tools / 'pg_config'
        pg_config.write_text('#!/bin/sh\ncase $1 in --pkglibdir) echo /lib ;; --sharedir) echo /share ;; esac\n')
        pg_config.chmod(0o755)
        cargo = tools / 'cargo'
        cargo.write_text('''#!/bin/sh
set -eu
[ "$1" != metadata ] || exit 0
[ "$1 $2" = 'pgrx package' ]
[ "$PGRX_PG_CONFIG_PATH" = "$4" ]
[ ! -e "$HOME/.pgrx" ]
tree=target/release/thinkthen-pg16
mkdir -p "$tree/lib" "$tree/share/extension"
touch "$tree/lib/thinkthen.so"
cp thinkthen.control "$tree/share/extension/"
version=$(sed -n "s/^default_version = '\\(.*\\)'$/\\1/p" thinkthen.control)
touch "$tree/share/extension/thinkthen--$version.sql"
''')
        cargo.chmod(0o755)
        uname = tools / 'uname'
        uname.write_text('#!/bin/sh\ncase $1 in -s) echo Linux ;; *) echo x86_64 ;; esac\n')
        uname.chmod(0o755)
        (database / 'pgrx-package-locked.sh').chmod(0o755)
        env = {'PATH': str(tools) + os.pathsep + os.environ['PATH'], 'HOME': str(root / 'empty-home'),
               'PG_CONFIG': str(pg_config), 'TMPDIR': str(root)}
        result = subprocess.run(['/bin/sh', str(database / 'package.sh'), '16', str(root / 'out')],
                                env=env, capture_output=True, text=True, timeout=5)
        assert result.returncode == 0, result.stderr
        assert (root / 'out/lib/thinkthen.so').is_file()
        # Installed smoke must pass the existing archive to the runtime check,
        # which otherwise requires source-packaging tools on the host.
        scripts = root / 'sdlc/scripts'
        scripts.mkdir(parents=True)
        for name in ('scratch.sh', 'release-workflow'):
            shutil.copyfile(REPO / 'sdlc/scripts' / name, scripts / name)
        for relative, body in (('libraries/ruby/toolchain.env', 'RUBY_MINORS=\n'),
                               ('libraries/python/host.sh', 'python_host() { command -v python3; }\n'),
                               ('libraries/typescript/node.sha256', ''),
                               ('databases/sqlite/amalgamation.sha256', ''),
                               ('crates/thinkthen/Cargo.toml', '[package]\nversion = "0.2.0"\n')):
            path = root / relative
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text(body)
        (root / 'platform').mkdir()
        for major in (16, 17, 18):
            (root / 'platform' / f'thinkthen-postgresql{major}-0.2.0-aarch64-unknown-linux-gnu.tar.gz').touch()
        (database / 'runtime.sh').write_text('runtime_ready() {\n'
            '  test -f "${THINKTHEN_ARTIFACT:-/nonexistent}"\n'
            '  case "$THINKTHEN_ARTIFACT" in */thinkthen-postgresql"$PG_MAJOR"-0.2.0-aarch64-unknown-linux-gnu.tar.gz) ;; *) exit 95 ;; esac\n}\n')
        module = runpy.run_path(str(REPO / 'sdlc/scripts/hosted-preflight.py'))
        check = module['Preflight']()
        runtime_commands = []
        def command(args, expected=None, cwd=root):
            if args[:2] == ['sh', '-ec'] and 'runtime_ready' in args[2]:
                subprocess.run(args, cwd=cwd, env=env, capture_output=True, check=True, timeout=5)
                runtime_commands.append(args)
        with patch.dict(module['qualify'].__globals__, ROOT=root, rust=lambda *args: None,
                        cargo_cache=lambda *args: None, downloaded_inputs=lambda *args: None), \
                patch.dict(sys.modules, inputs=SimpleNamespace(selected=None, versions=lambda: ()),
                           source_hosts=SimpleNamespace(prepare=None)), \
                patch.object(check, 'command', side_effect=command), patch.object(check, 'tools'):
            module['installed'](check, 'aarch64-unknown-linux-gnu', root / 'cache')
            assert not runtime_commands and any(message.startswith('native prepared inputs:')
                                                for message in check.missing), check.missing
            module['qualify'](check, 'smoke', 'aarch64-unknown-linux-gnu')
        assert len(runtime_commands) == 3, (runtime_commands, check.missing)


def linux_coverage_destination_case():
    gate = yaml.safe_load((REPO / '.github/workflows/gate.yml').read_text())
    job = gate['jobs']['linux-release']
    setup = next(step['run'] for step in job['steps']
                 if step.get('name') == 'Require the frozen dispatch commit and candidate')
    selection = next(line.strip() for line in setup.splitlines() if line.strip().startswith('work=$('))
    command = selection + '\nprintf "%s" "$work/$STAGE/coverage"\n'
    driver = runpy.run_path(str(REPO / 'sdlc/scripts/coverage'))

    class AcceptedOutput(Exception):
        pass

    # Use Linux absolute paths even when macOS redirects /home through its own filesystem.
    # Stop at directory creation: exercise the privacy rule without scanning or building a tree.
    def accepts(output):
        path = Path(output)
        resolved = path if path == Path('/home') or Path('/home') in path.parents else path.resolve()
        with patch.dict(driver['main'].__globals__, tools=lambda: {}, installed_helpers=lambda: []), \
                patch.object(sys, 'platform', 'linux'), \
                patch.object(sys, 'argv', ['coverage', '--platform', 'linux', '--output', output]), \
                patch.object(subprocess, 'check_output', side_effect=['0' * 40, b'']), \
                patch.object(Path, 'resolve', return_value=resolved), \
                patch.object(Path, 'mkdir', side_effect=AcceptedOutput) as create:
            try:
                driver['main']()
            except AcceptedOutput:
                create.assert_called_once_with(parents=True, exist_ok=False)
                return True
            except RuntimeError as error:
                assert str(error) == 'installed coverage output must be outside /home to preserve package privacy'
                create.assert_not_called()
                return False
        raise AssertionError('coverage did not reach its output boundary')

    assert not accepts('/home/runner/work/_temp/thinkthen-linux-release/coverage/coverage')
    assert not accepts('/home')
    with tempfile.TemporaryDirectory(prefix='thinkthen-linux-coverage-') as temporary:
        root = Path(temporary)
        for roomy in (False, True):
            for name, body in (
                    ('df', 'case "$2" in /mnt) free=' + ('900' if roomy else '50') +
                     ' ;; *) free=100 ;; esac\nprintf "Filesystem Blocks Used Available Capacity Mounted\\nx 1 1 %s 1%% %s\\n" "$free" "$2"'),
                    ('sudo', 'exit 0')):
                tool = root / name
                tool.write_text('#!/bin/sh\n' + body + '\n')
                tool.chmod(0o755)
            env = {'PATH': f'{root}:{os.defpath}', 'STAGE': 'coverage',
                   'RUNNER_TEMP': '/home/runner/work/_temp'}
            result = subprocess.run(['/bin/sh', '-ec', command], cwd=REPO, env=env,
                                    capture_output=True, text=True, timeout=5)
            assert result.returncode == 0, result.stderr
            expected = '/mnt/thinkthen-container' if roomy else '/tmp/thinkthen-linux-release'
            assert result.stdout == expected + '/coverage/coverage', result.stdout
            assert accepts(result.stdout), result.stdout
            if not roomy:
                old = command.replace('/tmp/thinkthen-linux-release', '"$RUNNER_TEMP/thinkthen-linux-release"')
                result = subprocess.run(['/bin/sh', '-ec', old], cwd=REPO, env=env,
                                        capture_output=True, text=True, timeout=5)
                assert result.returncode == 0, result.stderr
                assert not accepts(result.stdout), result.stdout


def dart_cache_setup_case():
    bootstrap = (REPO / 'sdlc/scripts/release-workflow').read_text()
    preparation = bootstrap.split('\tpub_cache=$1/pub-cache\n', 1)[1].split('\n}\ncase ', 1)[0]
    with tempfile.TemporaryDirectory(prefix='thinkthen-dart-cache-') as temporary:
        root = Path(temporary)
        dart = root / 'dart'
        dart.write_text(f'#!{sys.executable}\n' +
                        'from pathlib import Path\nimport os, sys, yaml\n' +
                        'cache = Path(os.environ["PUB_CACHE"])\n' +
                        'args = sys.argv[1:]\n' +
                        'if args == ["pub", "cache", "add", "ffi", "--version", "2.2.0"]:\n' +
                        '    packages = {"ffi": yaml.safe_load(Path(os.environ["SOURCE_LOCK"]).read_text())["packages"]["ffi"]}\n' +
                        'else:\n' +
                        '    assert args in (["pub", "get", "--enforce-lockfile"], ["pub", "get", "--offline", "--enforce-lockfile"])\n' +
                        '    assert Path.cwd() != Path(os.environ["SOURCE_LOCK"]).parent\n' +
                        '    assert Path("pubspec.lock").read_bytes() == Path(os.environ["SOURCE_LOCK"]).read_bytes()\n' +
                        '    assert Path("pubspec.yaml").is_file()\n' +
                        '    packages = yaml.safe_load(Path("pubspec.lock").read_text())["packages"]\n' +
                        'for name, package in packages.items():\n' +
                        '    folder = name + "-" + package["version"]\n' +
                        '    manifest = cache / "hosted/pub.dev" / folder / "pubspec.yaml"\n' +
                        '    stamp = cache / "hosted-hashes/pub.dev" / (folder + ".sha256")\n' +
                        '    if "--offline" in args:\n' +
                        '        assert manifest.is_file(), "missing " + name\n' +
                        '        assert stamp.read_text().strip() == package["description"]["sha256"]\n' +
                        '    else:\n' +
                        '        manifest.parent.mkdir(parents=True, exist_ok=True)\n' +
                        '        manifest.write_text(name)\n' +
                        '        stamp.parent.mkdir(parents=True, exist_ok=True)\n' +
                        '        stamp.write_text(package["description"]["sha256"])\n')
        dart.chmod(0o755)
        cache = root / 'pub-cache'
        env = {**os.environ, 'PUB_CACHE': str(cache),
               'SOURCE_LOCK': str(REPO / 'libraries/dart/pubspec.lock')}
        command = '. "$1/sdlc/scripts/scratch.sh"\nfail() { exit 1; }\n'
        command += 'dart_bin=$2\npub_cache=$3\n' + preparation
        # A cache containing only ffi reproduces the old release preparation.
        subprocess.run([str(dart), 'pub', 'cache', 'add', 'ffi', '--version', '2.2.0'], env=env, check=True)
        module = runpy.run_path(str(REPO / 'sdlc/scripts/hosted-preflight.py'))
        with patch.dict(os.environ, env):
            check = module['Preflight']()
            module['dart_cache'](check, str(dart))
            assert check.missing, 'ffi-only cache passed native-assets preflight'
            result = subprocess.run(['/bin/sh', '-ec', command, 'prepare', str(REPO), str(dart), str(cache)],
                                    cwd=REPO, env=env, capture_output=True, text=True, timeout=5)
            assert result.returncode == 0, result.stderr
            check = module['Preflight']()
            module['dart_cache'](check, str(dart))
            assert not check.missing, check.missing
            shutil.rmtree(cache / 'hosted/pub.dev/hooks-2.0.2')
            check = module['Preflight']()
            module['dart_cache'](check, str(dart))
            assert check.missing, 'missing hooks passed native-assets preflight'


def flutter_locked_setup_case():
    workflow = (REPO / 'sdlc/scripts/release-workflow').read_text()
    step = workflow.split('\nphp-dart-tools)\n', 1)[1].split('\nswift-zig-tools)', 1)[0]
    step = step[step.index('mkdir -p "$HOME/.local/opt/flutter"'):]
    acquisition = step.split('flutter=', 1)[0]
    command = '(cd "$HOME' + step.split('(cd "$HOME', 1)[1].split('git diff --exit-code', 1)[0]
    command += 'git diff --exit-code\n'
    with tempfile.TemporaryDirectory(prefix='thinkthen-flutter-locks-') as temporary:
        root = Path(temporary).resolve()
        sdk = root / 'home/.local/opt/flutter'
        # Exercise real Git acquisition through an isolated transport, without downloading an SDK.
        source = root / 'source'
        subprocess.run(['git', 'init', '-q', str(source)], check=True)
        git = ['git', '-C', str(source)]
        commit = git + ['-c', 'user.name=Fixture', '-c', 'user.email=fixture@example.invalid',
                        'commit', '-q', '--allow-empty', '-m', 'SDK fixture']
        subprocess.run(commit, check=True)
        pin = subprocess.check_output(git + ['rev-parse', 'HEAD'], text=True).strip()
        subprocess.run(git + ['tag', '3.47.5'], check=True)
        acquire = acquisition.replace('https://github.com/flutter/flutter.git', source.as_uri())
        acquire = acquire.replace('6a19cca56475dbfba1478ee68d7bd0c2ef891da1', pin)
        env = {'PATH': os.environ['PATH'], 'HOME': str(root / 'home')}
        result = subprocess.run(['/bin/sh', '-ec', acquire], env=env,
                                capture_output=True, text=True, timeout=5)
        assert result.returncode == 0, result.stderr
        tags = subprocess.check_output(['git', '-C', str(sdk), 'tag', '--points-at', 'HEAD'], text=True)
        assert tags.splitlines() == ['3.47.5'], 'Flutter cannot identify its release without its tag'
        assert subprocess.check_output(['git', '-C', str(sdk), 'rev-parse', 'HEAD'], text=True).strip() == pin
        # A moved release ref must fail against the approved commit before any SDK command runs.
        subprocess.run(commit, check=True)
        subprocess.run(git + ['tag', '-f', '3.47.5'], check=True, capture_output=True)
        result = subprocess.run(['/bin/sh', '-ec', acquire],
                                env={**env, 'HOME': str(root / 'moved-tag-home')},
                                capture_output=True, text=True, timeout=5)
        assert result.returncode == 1, (result.returncode, result.stderr)
        tools = sdk / 'packages/flutter_tools'
        tools.mkdir(parents=True)
        (tools / 'pubspec.lock').write_text('SDK lock fixture\n')
        for source in (REPO / 'libraries/dart').rglob('pubspec.*'):
            if source.name not in ('pubspec.yaml', 'pubspec.lock'):
                continue
            target = root / source.relative_to(REPO)
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes(source.read_bytes())
        example = root / 'libraries/dart/flutter/example'
        for source in (REPO / 'libraries/dart/flutter/example').rglob('generated_*'):
            target = root / source.relative_to(REPO)
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes(source.read_bytes())
        manifest = yaml.safe_load((example / 'pubspec.yaml').read_text())
        lock = yaml.safe_load((example / 'pubspec.lock').read_text())
        for package in ('thinkthen_dart', 'thinkthen_flutter'):
            path = manifest['dependencies'][package]['path']
            assert lock['packages'][package]['source'] == 'path'
            assert lock['packages'][package]['description']['path'] == path
            assert (example / path / 'pubspec.yaml').is_file()
        # Checked-in Linux generation must agree with local plugin declarations.
        ffi_plugins = []
        for dependency in manifest['dependencies'].values():
            if not isinstance(dependency, dict) or 'path' not in dependency:
                continue
            package = yaml.safe_load((example / dependency['path'] / 'pubspec.yaml').read_text())
            linux = package.get('flutter', {}).get('plugin', {}).get('platforms', {}).get('linux', {})
            if linux.get('ffiPlugin'):
                ffi_plugins.append(package['name'])
        generated = example / 'linux/flutter/generated_plugins.cmake'
        listed = generated.read_text().split('list(APPEND FLUTTER_FFI_PLUGIN_LIST\n', 1)[1].split(')', 1)[0]
        assert listed.split() == sorted(ffi_plugins), 'Generated Linux FFI plugins differ from manifests'
        subprocess.run(['git', 'init', '-q', str(root)], check=True)
        subprocess.run(['git', 'add', 'libraries'], cwd=root, check=True)
        subprocess.run(['git', '-c', 'user.name=Fixture', '-c', 'user.email=fixture@example.invalid',
                        'commit', '-qm', 'Tracked consumer fixture'], cwd=root, check=True)
        assert not (root / 'libraries/dart/flutter/pubspec.lock').exists()
        required = set(yaml.safe_load((root / 'libraries/dart/pubspec.lock').read_text())['packages'])
        resolver = root / 'resolver'
        resolver.write_text(f'#!{sys.executable}\n' +
                            'from pathlib import Path\nimport sys\nimport yaml\n' +
                            'assert sys.argv[1:] == ["pub", "get", "--enforce-lockfile"]\n' +
                            'assert Path("pubspec.lock").is_file(), "Cannot enforce a missing pubspec.lock"\n' +
                            # Check local sources and closure entries; the real SDK owns version solving.
                            'if Path("pubspec.yaml").is_file():\n' +
                            '    lock = yaml.safe_load(Path("pubspec.lock").read_text())["packages"]\n' +
                            f'    assert set({sorted(required)!r}).issubset(lock), "Incomplete dependency closure"\n' +
                            '    override = Path("pubspec_overrides.yaml")\n' +
                            '    overrides = yaml.safe_load(override.read_text()).get("dependency_overrides", {}) if override.exists() else {}\n' +
                            '    pending, seen = [Path.cwd()], set()\n' +
                            '    while pending:\n' +
                            '        project = pending.pop().resolve()\n' +
                            '        if project in seen: continue\n' +
                            '        seen.add(project)\n' +
                            '        manifest = yaml.safe_load((project / "pubspec.yaml").read_text())\n' +
                            '        deps = dict(manifest.get("dependencies", {}))\n' +
                            '        if project == Path.cwd(): deps.update(manifest.get("dev_dependencies", {}))\n' +
                            '        for name, declared in deps.items():\n' +
                            '            spec = overrides.get(name, declared)\n' +
                            '            source = next((kind for kind in ("path", "sdk", "git") if isinstance(spec, dict) and kind in spec), "hosted")\n' +
                            '            assert lock[name]["source"] == source, "Dependency source conflict: " + name\n' +
                            '            if source == "path":\n' +
                            '                base = Path.cwd() if name in overrides else project\n' +
                            '                target = (base / spec["path"]).resolve()\n' +
                            '                assert target == (Path.cwd() / lock[name]["description"]["path"]).resolve(), "Locked path conflict: " + name\n' +
                            '                assert yaml.safe_load((target / "pubspec.yaml").read_text())["name"] == name\n' +
                            '                pending.append(target)\n' +
                            'with open(sys.argv[0] + ".calls", "a") as calls:\n' +
                            '    calls.write(str(Path.cwd()) + "\\n")\n')
        resolver.chmod(0o755)
        before = {path: path.read_bytes() for path in root.rglob('pubspec.*')}
        env = {'PATH': os.environ['PATH'], 'HOME': str(root / 'home'),
               'dart': str(resolver), 'flutter': str(resolver)}
        result = subprocess.run(['/bin/sh', '-ec', command], cwd=root, env=env,
                                capture_output=True, text=True, timeout=5)
        assert result.returncode == 0, (result.returncode, result.stdout, result.stderr)
        calls = Path(str(resolver) + '.calls').read_text().splitlines()
        expected = [str(tools), str(root / 'libraries/dart'),
                    str(root / 'libraries/dart/checks/session_consumer'), str(example)]
        assert calls == expected, (calls, expected)
        assert {path: path.read_bytes() for path in root.rglob('pubspec.*')} == before
        override = example / 'pubspec_overrides.yaml'
        assert not override.exists()
        original = generated.read_bytes()
        generated.write_text(generated.read_text() + '# Changed generation\n')
        result = subprocess.run(['/bin/sh', '-ec', command], cwd=root, env=env,
                                capture_output=True, text=True, timeout=5)
        assert result.returncode == 1 and 'generated_plugins.cmake' in result.stdout, result
        generated.write_bytes(original)
        for broken in (command.replace('dependency_overrides:', 'ignored_overrides:'),
                       command.replace('path: ../..', 'path: ..')):
            result = subprocess.run(['/bin/sh', '-ec', broken], cwd=root, env=env,
                                    capture_output=True, text=True, timeout=5)
            assert result.returncode != 0, result.stdout
            assert 'conflict: thinkthen_dart' in result.stderr, result.stderr
            assert not override.exists(), 'Failed acquisition retained its owned override'
            assert {path: path.read_bytes() for path in root.rglob('pubspec.*')} == before
        # Every retained consumer lock must carry the local Dart package's full closure.
        for consumer in (example, root / 'libraries/dart/checks/session_consumer'):
            locked = consumer / 'pubspec.lock'
            original = locked.read_bytes()
            incomplete = yaml.safe_load(original)
            incomplete['packages'].pop('record_use')
            locked.write_text(yaml.safe_dump(incomplete))
            result = subprocess.run(['/bin/sh', '-ec', command], cwd=root, env=env,
                                    capture_output=True, text=True, timeout=5)
            assert result.returncode != 0 and 'Incomplete dependency closure' in result.stderr, result.stderr
            assert not override.exists(), 'Failed acquisition retained its owned override'
            locked.write_bytes(original)
            assert {path: path.read_bytes() for path in root.rglob('pubspec.*')} == before
        override.write_text('owned by another caller\n')
        result = subprocess.run(['/bin/sh', '-ec', command], cwd=root, env=env,
                                capture_output=True, text=True, timeout=5)
        assert result.returncode != 0
        assert override.read_text() == 'owned by another caller\n'
        override.unlink()
        caller_target = root / 'caller-owned.yaml'
        override.symlink_to(caller_target)
        result = subprocess.run(['/bin/sh', '-ec', command], cwd=root, env=env,
                                capture_output=True, text=True, timeout=5)
        assert result.returncode != 0, 'Acquisition accepted a caller-owned dangling override'
        assert override.is_symlink(), 'Acquisition removed a caller-owned override'
        assert not caller_target.exists(), 'Acquisition wrote through a caller-owned override'
        override.unlink()



def preflight_cases():
    gate = yaml.safe_load((REPO / '.github/workflows/gate.yml').read_text())
    steps = gate['jobs']['linux-release']['steps']
    preflight = next(i for i, step in enumerate(steps) if step.get('name') == 'Check every prepared hosted input')
    qualification = next(i for i, step in enumerate(steps) if step.get('name') == 'Run the selected existing gates and retain their actual exits')
    assert steps[preflight]['env'] == {'PREFLIGHT_ONLY': '${{ inputs.preflight_only }}'}
    assert preflight < qualification
    assert 'if' not in steps[preflight] and 'continue-on-error' not in steps[preflight]
    assert steps[qualification]['if'] == '${{ !inputs.preflight_only }}'
    assert all('!inputs.preflight_only' in step.get('if', '') for step in steps
               if 'actions/upload-artifact@' in step.get('uses', ''))
    release = yaml.safe_load((REPO / '.github/workflows/release.yml').read_text())
    for job, name in (('build', 'Check every prepared build input'),
                      ('smoke', 'Check every prepared installed smoke input')):
        step = next(step for step in release['jobs'][job]['steps'] if step.get('name') == name)
        assert 'hosted-preflight.py' in step['run'] and 'if' not in step and 'continue-on-error' not in step
    with tempfile.TemporaryDirectory(prefix='thinkthen-preflight-fixture-') as temporary:
        root = Path(temporary)
        scripts = root / 'sdlc/scripts'
        scripts.mkdir(parents=True)
        helper = scripts / 'hosted-preflight.py'
        helper.write_bytes((REPO / 'sdlc/scripts/hosted-preflight.py').read_bytes())
        children = root / 'conformance/children'
        children.mkdir(parents=True)
        (children / 'children.py').write_bytes((REPO / 'conformance/children/children.py').read_bytes())
        (root / 'rust-toolchain.toml').write_bytes((REPO / 'rust-toolchain.toml').read_bytes())
        (root / 'libraries/typescript').mkdir(parents=True)
        (root / 'libraries/typescript/node.sha256').write_bytes((REPO / 'libraries/typescript/node.sha256').read_bytes())
        (scripts / 'release-workflow').write_bytes((REPO / 'sdlc/scripts/release-workflow').read_bytes())
        tools = root / 'bin'
        tools.mkdir()
        probe = tools / 'fixture'
        probe.write_text(f'#!{sys.executable}\n' +
                         'from pathlib import Path\nimport sys\n' +
                         'name = Path(sys.argv[0]).name\n' +
                         'if name == "rustc": print("rustc 1.95.0 (fixture)")\n' +
                         'if name == "node": print("v22.22.3")\n' +
                         'if name == "maturin": print("maturin 1.15.0")\n' +
                         'if name == "rustup": print("clippy-fixture\\nrustfmt-fixture")\n')
        probe.chmod(0o755)
        required = ('cargo', 'rustc', 'rustup', 'git', 'python3', 'node', 'npm', 'tar', 'maturin', 'cl.exe', 'lib.exe', 'link.exe')
        for tool in required:
            (tools / tool).symlink_to(probe)
        # Compiler fixtures stand at the process boundary; no digest check is bypassed.
        command = [sys.executable, str(helper), 'build', 'x86_64-pc-windows-msvc']
        env = {'PATH': str(tools), 'HOME': str(root), 'PYTHONDONTWRITEBYTECODE': '1'}
        ready = subprocess.run(command, cwd=root, env=env, capture_output=True, text=True, timeout=5)
        assert ready.returncode == 0 and ready.stderr == '', ready.stderr
        assert ready.stdout == 'hosted-preflight: all prepared inputs are ready\n'
        # A failed prepared command must retain the error needed to repair its input.
        maturin = tools / 'maturin'
        maturin.unlink()
        maturin.write_text(f'#!{sys.executable}\nimport os, sys\n'
                          'print("prepared command output")\n'
                          'print("prepared command error", file=sys.stderr)\n'
                          'if os.environ.get("PREFLIGHT_NOISY"):\n'
                          '    print("o" * (2040 - len("prepared command output\\n")) + os.environ["PRIVATE_SERVICE_KEY"])\n'
                          '    for name in ("THINKTHEN_API_KEY", "GITHUB_TOKEN", "PRIVATE_SERVICE_KEY", "db_password", "AUTHORIZATION"):\n'
                          '        print(os.environ[name])\n'
                          '        print(os.environ[name], file=sys.stderr)\n'
                          '    print("o" * 20000)\n'
                          '    print("e" * 20000, file=sys.stderr)\n'
                          '    print("prepared command output end")\n'
                          '    print("ModuleNotFoundError: No module named prepared_fixture", file=sys.stderr)\n'
                          'sys.exit(9)\n')
        maturin.chmod(0o755)
        failed = subprocess.run(command, cwd=root, env=env, capture_output=True, text=True, timeout=5)
        assert failed.returncode == 1 and 'exit 9\nprepared command output\nprepared command error' in failed.stderr, failed.stderr
        secrets = {name: 'fake-preflight-private-' + str(index) for index, name in enumerate(
            ('THINKTHEN_API_KEY', 'GITHUB_TOKEN', 'PRIVATE_SERVICE_KEY', 'db_password', 'AUTHORIZATION'))}
        failed = subprocess.run(command, cwd=root, env=dict(env, PREFLIGHT_NOISY='1',
                                THINKTHEN_PROBE_SETTING='prepared_fixture', **secrets),
                                capture_output=True, text=True, timeout=5)
        assert failed.returncode == 1 and failed.stdout == '', failed
        assert len(failed.stderr) < 10000, 'failed child output was not bounded'
        assert all(value not in failed.stderr for value in secrets.values()), 'failed child output exposed an environment secret'
        assert 'fake-pre' not in failed.stderr, 'truncation exposed a fragment of an environment secret'
        for marker in ('prepared command output', 'prepared command error', 'prepared command output end',
                       'ModuleNotFoundError: No module named prepared_fixture', '[truncated]'):
            assert marker in failed.stderr, (marker, failed.stderr)
        maturin.unlink()
        maturin.symlink_to(probe)
        # Rehearsal output is future input during bootstrap-only proof.
        # The normal coverage path must refuse absent and different-SHA reports.
        module = runpy.run_path(str(helper))
        # An exact cache hit cannot excuse missing sources or stale/partial native hosts.
        sys.path.insert(0, str(REPO / 'databases/duckdb/tools'))
        from inputs import selected, versions
        ruby = root / 'libraries/ruby'
        ruby.mkdir(parents=True)
        (ruby / 'toolchain.env').write_text("RUBY_MINORS=''\n")
        cache = root / 'toolchains'
        for version in versions():
            identity = selected(version, 'x86_64-apple-darwin')
            python = cache / 'duckdb/community' / version / 'venv/bin/python'
            cli = root / 'databases/duckdb/build/community' / version / 'build/release/duckdb'
            for binary, value in ((python, identity['source_commit'][:10]), (cli, identity['source_commit'][:8])):
                binary.parent.mkdir(parents=True, exist_ok=True)
                binary.write_text('#!/bin/sh\necho ' + value + '\n')
                binary.chmod(0o755)
        for state in ('matching', 'wrong-pin', 'partial'):
            if state == 'wrong-pin': cli.write_text('#!/bin/sh\necho wrong-source-id\n')
            elif state == 'partial': cli.unlink()
            check = module['Preflight']()
            with patch.dict(module['native_inputs'].__globals__, ROOT=root), \
                    patch.dict(os.environ, {'THINKTHEN_DUCKDB_HOST_CACHE_HIT': 'true'}):
                module['native_inputs'](check, 'x86_64-apple-darwin', cache, source_only=True)
            source_failures = [message for message in check.missing if message.startswith('DuckDB ')]
            assert len(source_failures) == 2 * len(versions()), check.missing
            cli_failures = [message for message in check.missing if str(cli) in message]
            assert bool(cli_failures) == (state != 'matching'), (state, check.missing)
            if state == 'wrong-pin': assert 'differs from pin' in cli_failures[0], cli_failures
        # Missing lock-reading support must not hide an independent coverage input.
        globals_ = module['qualify'].__globals__
        check = module['Preflight']()
        with patch.dict(globals_, ROOT=REPO, rust=lambda *args: None,
                        cargo_cache=lambda *args: None, installed=lambda *args: None), \
                patch.object(check, 'command'), patch.object(check, 'tools'), \
                patch.dict(sys.modules, {'yaml': None}), \
                patch.dict(os.environ, {'RUNNER_TEMP': str(root), 'PREFLIGHT_ONLY': 'false'}):
            check.check('prepared host inputs', lambda: module['qualify'](check, 'coverage', 'x86_64-unknown-linux-gnu'))
        assert any('yaml' in message for message in check.missing), check.missing
        assert any('same-SHA Apple coverage input' in message for message in check.missing), check.missing
        check = module['Preflight']()
        module['dart_locks'](check)
        for project in ('libraries/dart', 'libraries/dart/checks/session_consumer', 'libraries/dart/flutter/example'):
            assert any(message.startswith(project + ' frozen Dart inputs:') for message in check.missing), check.missing
        report = root / 'thinkthen-apple-coverage/totals.json'
        for local_only, source, wanted in ((True, None, 0), (False, None, 1),
                                           (False, 'a' * 40, 0), (False, 'b' * 40, 1)):
            if source:
                report.parent.mkdir(exist_ok=True)
                report.write_text(json.dumps({'source': source, 'platform_collection': 'apple'}))
            check = module['Preflight']()
            with patch.dict(os.environ, {'RUNNER_TEMP': str(root), 'SHA': 'a' * 40}):
                module['apple_input'](check, local_only)
            assert int(bool(check.missing)) == wanted, (local_only, source, check.missing)
        # Exercise real checksum bytes through the installed-smoke entry point.
        (root / 'crates/thinkthen').mkdir(parents=True)
        manifest = REPO / 'crates/thinkthen/Cargo.toml'
        (root / 'crates/thinkthen/Cargo.toml').write_bytes(manifest.read_bytes())
        import tomllib
        version = tomllib.loads(manifest.read_text())['package']['version']
        platform = root / 'platform'
        platform.mkdir()
        names = [f'thinkthen-c-{version}-x86_64-pc-windows-msvc.zip',
                 f'thinkthen-{version}-x86_64-pc-windows-msvc.zip', 'thinkthen-first-run.tar.gz']
        for name in names:
            archive = platform / name
            archive.write_bytes(('fixture payload ' + name).encode())
            digest = hashlib.sha256(archive.read_bytes()).hexdigest()
            (platform / (name + '.sha256')).write_text(digest + '  ' + name + '\n')
        (root / 'crate-dist').mkdir()
        (root / 'crate-dist' / f'thinkthen-{version}.crate').write_bytes(b'fixture crate')
        smoke = [sys.executable, str(helper), 'smoke', 'x86_64-pc-windows-msvc']
        ready = subprocess.run(smoke, cwd=root, env=env, capture_output=True, text=True, timeout=5)
        assert ready.returncode == 0 and ready.stderr == '', ready.stderr
        for name in names[:2]:
            (platform / name).write_bytes(b'corrupted archive')
        corrupt = subprocess.run(smoke, cwd=root, env=env, capture_output=True, text=True, timeout=5)
        assert corrupt.returncode == 1 and corrupt.stderr.count('SHA256 differs from pin') == 2, corrupt.stderr
        for tool in ('node', 'cl.exe', 'lib.exe'):
            (tools / tool).unlink()
        missing = subprocess.run(command, cwd=root, env=env, capture_output=True, text=True, timeout=5)
        assert missing.returncode == 1, missing
        for tool in ('node', 'cl.exe', 'lib.exe'):
            assert 'hosted-preflight: ' + tool + ': missing from PATH' in missing.stderr, missing.stderr
        assert 'all prepared inputs are ready' not in missing.stdout

def smoke_family_cases():
    release = yaml.safe_load((REPO / '.github/workflows/release.yml').read_text())
    smoke = release['jobs']['smoke']
    cells = smoke['strategy']['matrix']['include']
    base = set('command c python typescript ruby sqlite duckdb postgresql16 postgresql17 postgresql18 installers'.split())
    expected = {
        'x86_64-unknown-linux-gnu': base | set('go-cpp swift-zig php-dart ada-objective-c-cobol managed examples'.split()),
        'aarch64-unknown-linux-gnu': base,
        'aarch64-apple-darwin': base | {'swift-zig', 'ada-objective-c-cobol', 'examples'},
        'x86_64-apple-darwin': base | {'swift-zig', 'ada-objective-c-cobol', 'examples'},
        'x86_64-pc-windows-msvc': {'windows-c', 'windows-command'},
    }
    assert len(cells) == len({(cell['target'], cell['family']) for cell in cells})
    assert {target: {cell['family'] for cell in cells if cell['target'] == target}
            for target in expected} == expected
    assert smoke['timeout-minutes'] == '${{ matrix.job_minutes }}'
    assert smoke['strategy']['fail-fast'] is False and 'if' not in smoke
    assert 'smoke' in release['jobs']['draft']['needs']
    selected = next(step for step in smoke['steps'] if 'sdlc/scripts/release-smoke smoke-files' in step.get('run', ''))
    assert selected['env']['THINKTHEN_RELEASE_SMOKE_FAMILY'] == '${{ matrix.family }}'
    assert selected['env']['THINKTHEN_TEST_PROFILE'] == 'full'
    assert selected['timeout-minutes'] == '${{ matrix.consumer_minutes }}'
    logs = next(step for step in smoke['steps'] if step.get('name') == 'Retain available smoke logs')
    assert logs['if'] == 'always()' and '${{ matrix.family }}' in logs['with']['name']
    for cell in cells:
        assert 0 < cell['consumer_minutes'] < cell['minutes'] < cell['job_minutes'] <= 360
    gate = yaml.safe_load((REPO / '.github/workflows/gate.yml').read_text())
    for workflow, names in ((release, ('apple-packages', 'crate', 'windows-sdks', 'draft')),
                            (gate, ('ladder', 'language', 'linux-release'))):
        for name in names:
            job = workflow['jobs'][name]
            for step in job['steps']:
                if 'run' in step:
                    assert 0 < step['timeout-minutes'] < job['timeout-minutes'] <= 360
                    assert 'tee -a "$RUNNER_TEMP/' in step['run']
            retained = next(step for step in job['steps'] if step.get('name') == 'Retain available long-job logs')
            assert retained['if'].startswith('always()') and 'actions/upload-artifact@' in retained['uses']
    # Use real archive/checksum/scan/completeness code with local consumer exits.
    # This keeps the installed execution outside-in while avoiding tool builds.
    with tempfile.TemporaryDirectory(prefix='thinkthen-smoke-family-') as temporary:
        root = Path(temporary).resolve()
        stage_checkout(REPO, root)
        scripts = root / 'sdlc/scripts'
        # Only the isolated consumer leaf and external compiler/backend processes
        # are controlled. All release scripts and their dependencies stay real.
        (root / 'libraries/c/check.sh').write_text('echo selected-c\nexit "${FAKE_EXIT:-0}"\n')
        tools = root / 'fixture-tools'
        tools.mkdir()
        cargo = tools / 'cargo'
        cargo.write_text('#!/bin/sh\n[ "$*" = "build --locked --offline --quiet --package conformance-backend" ]\n')
        cargo.chmod(0o755)
        backend = root / 'fixture-target/debug/conformance-backend'
        backend.parent.mkdir(parents=True)
        backend.write_text(f'#!{sys.executable}\nimport sys\nprint(12345, flush=True)\n'
                           'for line in sys.stdin:\n    pass\n')
        backend.chmod(0o755)
        import tomllib
        version = tomllib.loads((root / 'crates/thinkthen/Cargo.toml').read_text())['package']['version']
        bundle = root / 'bundle'
        bundle.mkdir()
        import zipfile
        archive_names = ['thinkthen-' + family + '-' + version + '-fixture.tar.gz' for family in
                         ('c', 'sqlite', 'duckdb', 'postgresql16', 'postgresql17', 'postgresql18')]
        archive_names += [f'thinkthen-{version}-fixture.tar.gz', 'thinkthen-first-run.tar.gz', f'thinkthen-{version}.tgz']
        for name in archive_names:
            with tarfile.open(bundle / name, 'w:gz') as archive:
                info = tarfile.TarInfo('payload')
                info.size = 7
                archive.addfile(info, io.BytesIO(b'fixture'))
        with zipfile.ZipFile(bundle / f'thinkthen-{version}.whl', 'w') as archive:
            archive.writestr('payload', 'fixture')
        with tarfile.open(bundle / f'thinkthen-{version}.gem', 'w') as archive:
            payload = io.BytesIO()
            with tarfile.open(fileobj=payload, mode='w:gz') as data:
                info = tarfile.TarInfo('payload')
                info.size = 7
                data.addfile(info, io.BytesIO(b'fixture'))
            info = tarfile.TarInfo('data.tar.gz')
            info.size = len(payload.getvalue())
            archive.addfile(info, io.BytesIO(payload.getvalue()))
        for archive in list(bundle.iterdir()):
            (bundle / (archive.name + '.sha256')).write_text(
                hashlib.sha256(archive.read_bytes()).hexdigest() + '  ' + archive.name + '\n')
        home = root / 'home'
        home.mkdir()
        env = child_env(home=home, PATH=f'{tools}:{os.environ["PATH"]}', TMPDIR=str(root),
                        CARGO_TARGET_DIR=str(backend.parents[1]),
                        THINKTHEN_RELEASE_SMOKE_FAMILY='c', LANG='C.UTF-8')
        command = ['/bin/sh', str(scripts / 'release-smoke'), str(bundle)]
        def run(extra=None):
            return subprocess.run(command, cwd=root, env={**env, **(extra or {})},
                                  capture_output=True, text=True, timeout=5)
        good = run()
        assert good.returncode == 0 and good.stdout.count('selected-c') == 1, good
        assert 'selected family c only' in good.stdout and 'surfaces: pass command' not in good.stdout
        for status in ('1', '77'):
            bad = run({'FAKE_EXIT': status})
            assert bad.returncode == 1 and 'surfaces: FAIL libraries/c' in bad.stdout, bad
        unknown = run({'THINKTHEN_RELEASE_SMOKE_FAMILY': 'c; exit 0'})
        assert unknown.returncode == 2 and 'unknown family' in unknown.stderr, unknown
        absent = run({'THINKTHEN_RELEASE_SMOKE_FAMILY': 'managed'})
        assert absent.returncode == 1 and 'selected family managed has no consumer' in absent.stdout, absent
        archive = bundle / f'thinkthen-sqlite-{version}-fixture.tar.gz'
        archive.write_bytes(b'corrupt unselected archive')
        corrupt = run()
        assert corrupt.returncode == 1 and 'its .sha256 is missing or does not match' in corrupt.stdout, corrupt
        assert 'selected-c' not in corrupt.stdout, corrupt
        with tarfile.open(archive, 'w:gz') as packed:
            data = str(home).encode()
            info = tarfile.TarInfo('builder-path')
            info.size = len(data)
            packed.addfile(info, io.BytesIO(data))
        checksum = bundle / (archive.name + '.sha256')
        checksum.write_text(hashlib.sha256(archive.read_bytes()).hexdigest() + '  ' + archive.name + '\n')
        leaked = run()
        assert leaked.returncode == 1 and "builder's home or the test probe" in leaked.stdout, leaked
        assert 'selected-c' not in leaked.stdout, leaked
        archive.unlink()
        (bundle / (archive.name + '.sha256')).unlink()
        missing = run()
        assert missing.returncode == 1 and 'FAIL databases/sqlite (DIR holds no file for it)' in missing.stdout, missing
        assert 'selected-c' not in missing.stdout, missing


def parameter_run(case, command, **kwargs):
    print(f'hosted parameter: {case}', file=sys.stderr, flush=True)
    started = time.monotonic()
    try:
        result = subprocess.run(command, **kwargs)
    except subprocess.TimeoutExpired as error:
        print(f'hosted parameter: {case}: timeout after {error.timeout}s', file=sys.stderr)
        for name, output in (('stdout', error.stdout), ('stderr', error.stderr)):
            if isinstance(output, bytes):
                output = output.decode('utf-8', errors='replace')
            if output:
                print(f'{name} (last 4096 characters):\n{output[-4096:]}', file=sys.stderr)
        raise
    print(f'hosted parameter: {case}: exit {result.returncode} in '
          f'{time.monotonic() - started:.3f}s', file=sys.stderr, flush=True)
    return result


def parameter_cases():
    head = subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=REPO, text=True).strip()
    guard = REPO / 'sdlc/scripts/hosted-candidate.py'
    cases = [(head, head, 0), (head, '0' * 40, 1), ('0' * 40, '0' * 40, 1)]
    cases += [(value, head, 1) for value in ('', '--help', head.upper(), head[:39],
                                           head + '\n', '$(touch /tmp/hosted-plant)', 'x; exit 0')]
    for candidate, dispatch, wanted in cases:
        result = parameter_run(f'candidate={candidate!r} dispatch={dispatch!r}',
                               [sys.executable, str(guard), 'checkout', candidate, dispatch],
                                cwd=REPO, capture_output=True, text=True, timeout=5)
        assert result.returncode == wanted, (candidate, result.stderr)
        if wanted:
            assert result.stderr.startswith('hosted-candidate: '), result.stderr
    with tempfile.TemporaryDirectory(prefix='thinkthen-apple-workflow-') as temporary:
        root = Path(temporary)
        stage_checkout(REPO, root)
        for name in ('bin', 'packages'):
            (root / name).mkdir()
        for tool, body in [('uname', 'case $1 in -s) echo Darwin ;; *) echo arm64 ;; esac'),
                           ('git', f'echo {head}'), ('shasum', 'exit 0'),
                           ('tar', f'echo source_commit={head}')]:
            path = root / 'bin' / tool
            path.write_text('#!/bin/sh\n' + body + '\n')
            path.chmod(0o755)
        # Run the real candidate guard with the prepared interpreter, not the host launcher.
        (root / 'bin/python3').symlink_to(sys.executable)
        env = {'PATH': f'{root}/bin:{os.defpath}', 'HOME': str(root), 'LANG': 'C.UTF-8'}
        # stage_checkout preserves the tracked helper's non-executable mode.
        release = yaml.safe_load((REPO / '.github/workflows/release.yml').read_text())
        apple = release['jobs']['apple-packages']
        assembly = next(step['run'] for step in apple['steps'] if step.get('name') ==
                        'Assemble Swift and Foundation from both captured C packages')
        command = next(line for line in assembly.splitlines() if 'build-xcframework.sh' in line)
        (root / 'bin/uname').write_text('#!/bin/sh\necho Linux\n')
        for cell in apple['strategy']['matrix']['include']:
            result = parameter_run(f'Apple assembly target={cell["target"]}',
                                   ['/bin/sh', '-x', '-c', command], cwd=root,
                                    env={**env, 'TARGET': cell['target']},
                                    capture_output=True, text=True, timeout=5)
            assert result.returncode == 2 and 'Apple native tools are required' in result.stderr, (
                cell['target'], result.returncode, result.stderr)
        (root / 'bin/uname').write_text('#!/bin/sh\ncase $1 in -s) echo Darwin ;; *) echo arm64 ;; esac\n')
        for status in (0, 77, 1):
            (root / 'libraries/swift/check.sh').write_text(f'echo selected-swift\nexit {status}\n')
            (root / 'libraries/objective-c/check.sh').write_text('echo WRONG_LANGUAGE\nexit 1\n')
            result = parameter_run(f'Apple swift exit={status}',
                                   ['/bin/sh', '-x', str(root / 'sdlc/scripts/release-workflow'), 'apple-check',
                                     'aarch64-apple-darwin', str(root / 'packages'), 'swift', head],
                                    cwd=root, env=env, capture_output=True, text=True, timeout=5)
            assert result.returncode == (0 if status == 0 else 1), result.stderr
            assert 'selected-swift' in result.stdout and 'WRONG_LANGUAGE' not in result.stdout
            assert ('surfaces: pass' if status == 0 else 'surfaces: FAIL') in result.stdout
        for language in ('all', 'swift objective-c', '../swift', 'swift; exit 0'):
            result = parameter_run(f'Apple language={language!r}',
                                   ['/bin/sh', '-x', str(root / 'sdlc/scripts/release-workflow'), 'apple-check',
                                     'aarch64-apple-darwin', str(root / 'packages'), language, head],
                                    cwd=root, env=env, capture_output=True, text=True, timeout=5)
            assert result.returncode == 1 and 'select exactly one Apple language' in result.stderr
            assert 'selected-swift' not in result.stdout


smoke_family_cases()
preflight_cases()
print('hosted preflight: ready, aggregate missing, checksum corruption and Apple input cases pass')
shared_host_routing_cases()
cache_refusal_cases()
linux_work_destination_cases()
linux_tool_destination_case()
linux_nested_tools_case()
linux_failed_phase_evidence_case()
linux_source_staging_case()
postgresql_package_case()
linux_coverage_destination_case()
dart_cache_setup_case()
flutter_locked_setup_case()
parameter_cases()
print('hosted workflows: release helper execution cases pass')
