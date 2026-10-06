#!/usr/bin/env python3
"""Smoke the installed Windows command and optionally the packed Rust crate."""

import argparse
from contextlib import contextmanager
import http.server
import os
import json
import shutil
from pathlib import Path
import subprocess
import sys
import tarfile
import tempfile
import threading
import zipfile

import importlib.util


REPO = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location("windows_command", REPO / "sdlc/scripts/release-windows-command.py")
COMMAND = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(COMMAND)
INSTALLER_SPEC = importlib.util.spec_from_file_location("windows_installer", REPO / "sdlc/scripts/windows-installer-test.py")
INSTALLER = importlib.util.module_from_spec(INSTALLER_SPEC)
INSTALLER_SPEC.loader.exec_module(INSTALLER)
QUESTION = "Does this report say what the person did before the problem appeared?"


def run(args, env, *, text=None, code=0, output=None, timeout=180):
    # A Windows text-mode stdin converts LF to CRLF and changes replay identity.
    payload = text.encode("utf-8") if isinstance(text, str) else text
    raw = subprocess.run(args, env=env, input=payload, capture_output=True, timeout=timeout)
    result = subprocess.CompletedProcess(raw.args, raw.returncode, raw.stdout.decode("utf-8"), raw.stderr.decode("utf-8"))
    if result.returncode != code or (output is not None and result.stdout.strip() != output):
        raise RuntimeError(f"{args[0]} exited {result.returncode}: {result.stdout}{result.stderr}")
    return result


def smoke(binary, sample, env, version):
    """Replay through a counted loopback proxy and prove token refusal sends nothing."""
    requests = []

    class Counter(http.server.BaseHTTPRequestHandler):
        def do_POST(self):
            requests.append(self.path)
            self.send_response(500)
            self.send_header("Connection", "close")
            self.send_header("Content-Length", "0")
            self.end_headers()

        def do_CONNECT(self):
            self.do_POST()

        def log_message(self, *_args):
            pass

    with http.server.ThreadingHTTPServer(("127.0.0.1", 0), Counter) as server:
        thread = threading.Thread(target=server.serve_forever, daemon=True)
        thread.start()
        try:
            url = f"http://127.0.0.1:{server.server_port}"
            env = env | {"HTTP_PROXY": url, "HTTPS_PROXY": url, "ALL_PROXY": url,
                         "http_proxy": url, "https_proxy": url, "all_proxy": url,
                         "NO_PROXY": "", "no_proxy": ""}
            run([str(binary), "--version"], env, output=f"thinkthen {version}")
            run([str(binary), "decide", QUESTION, "--replay", str(sample / "recording"),
                 "--no-cache"], env, text=(sample / "report.txt").read_bytes(), output="true")
            cap = env | {"THINKTHEN_MAX_ESTIMATED_INPUT_TOKENS_TOTAL": "10"}
            refused = run([str(binary), "decide", "Is it?", "--no-cache", "--url", url + "/generic/v1"],
                          cap, text="evidence", code=2, output="")
            if "would be exceeded before this call's first request" not in refused.stderr:
                raise RuntimeError("token cap did not print its pre-send refusal")
            if requests:
                raise RuntimeError(f"offline smoke sent {len(requests)} requests")
        finally:
            server.shutdown()
            thread.join()
    print(f"Windows command smoke: {binary.name} reports {version}, replays true and sends 0 requests")


def interrupt(binary, env):
    """Run native console proof against the exact checked packed executable."""
    binary = binary.absolute()
    if not binary.is_file() or binary.is_symlink():
        raise RuntimeError("release interruption requires one regular exact executable")
    supplied = env | {"THINKTHEN_WINDOWS_RELEASE_BINARY": str(binary)}
    run(["cargo", "test", "--locked", "--offline", "-p", "thinkthen", "--test", "windows",
         "interrupt::release_binary_console_interrupt", "--", "--exact", "--ignored"],
        supplied, timeout=600)
    print("Windows packed command interruption: actual isolated Ctrl-C returned 130 with one request")


def python_smoke(platform, env, version, root):
    """Install the packed wheel away from source and count its real requests."""
    names = ('PATH', 'SystemRoot', 'SystemDrive', 'TEMP', 'TMP', 'HOME', 'APPDATA', 'LOCALAPPDATA')
    env = {name: env[name] for name in names if name in env}
    venv = root / 'python'
    run([sys.executable, '-I', '-m', 'venv', str(venv)], env)
    python = str(venv / 'Scripts/python.exe')
    wheel = platform / f'thinkthen-{version}-cp310-abi3-win_amd64.whl'
    run([python, '-I', '-m', 'pip', 'install', '--no-index', '--no-deps', str(wheel.resolve())], env)
    requests = []

    class Backend(http.server.BaseHTTPRequestHandler):
        def do_POST(self):
            body = json.loads(self.rfile.read(int(self.headers['Content-Length'])))
            requests.append(body)
            data = json.dumps({'model': 'jev-1.13.0', 'answers': {
                name: {'type': 'noul', 'noul': 0.9} for name in body['questions']}}).encode()
            self.send_response(200)
            self.send_header('Connection', 'close')
            self.send_header('Content-Length', str(len(data)))
            self.end_headers()
            self.wfile.write(data)

        def log_message(self, *_args):
            pass

    with http.server.ThreadingHTTPServer(('127.0.0.1', 0), Backend) as server:
        thread = threading.Thread(target=server.serve_forever, daemon=True)
        thread.start()
        try:
            supplied = env | {'THINKTHEN_API_KEY': 'sk-python-wheel-loopback',
                              'THINKTHEN_BASE_URL': f'http://127.0.0.1:{server.server_port}/generic/v1'}
            program = ('import pathlib, sys, thinkthen as tt, thinkthen._thinkthen as native; '
                       'assert all(pathlib.Path(p).resolve().is_relative_to(pathlib.Path(sys.prefix).resolve()) '
                       'for p in (tt.__file__, native.__file__)); ')
            capped = supplied | {'THINKTHEN_MAX_ESTIMATED_INPUT_TOKENS_TOTAL': '10'}
            refusal = run([python, '-I', '-c', program + '\ntry: tt.Engine(cache=False).decide("Is it late?", "one")'
                           '\nexcept tt.UsageError as error: print(error.kind, error)'], capped)
            if 'would be exceeded before this call\'s first request' not in refusal.stdout or requests:
                raise RuntimeError('installed Python spend refusal did not send zero requests')
            answer = run([python, '-I', '-c', program + 'answer = tt.Engine(cache=False).decide("Is it late?", "one"); '
                          'print(answer.value, answer.facts["requests_sent"])'], supplied, output='True 1')
            if len(requests) != 1 or 'sk-python-wheel-loopback' in answer.stdout + answer.stderr + refusal.stdout + refusal.stderr:
                raise RuntimeError('installed Python wheel sent the wrong count or exposed its key')
        finally:
            server.shutdown()
            thread.join(timeout=10)
    print('Windows Python wheel smoke: installed import, spend refusal sent 0 requests, answer sent 1')


@contextmanager
def installer_release(platform, version):
    """Serve only the verified command archive and sidecar through loopback."""
    name = f"thinkthen-{version}-{COMMAND.TARGET}.zip"
    prefix = f"/botassembly/thinkthen/releases/download/v{version}/"
    payloads = {prefix + file: (platform / file).read_bytes() for file in (name, name + ".sha256")}
    payloads["/repos/botassembly/thinkthen/releases"] = json.dumps([
        dict(tag_name="v" + version, draft=False, prerelease=False,
             assets=[dict(name=name), dict(name=name + ".sha256")])]).encode()
    requests = []

    class Release(http.server.BaseHTTPRequestHandler):
        def do_GET(self):
            requests.append(self.path)
            data = payloads.get(self.path)
            if data is None:
                self.send_error(404)
                return
            self.send_response(200)
            self.send_header("Content-Length", str(len(data)))
            self.send_header("Connection", "close")
            self.end_headers()
            self.wfile.write(data)

        def log_message(self, *_args):
            pass

    with http.server.ThreadingHTTPServer(("127.0.0.1", 0), Release) as server:
        thread = threading.Thread(target=server.serve_forever, daemon=True)
        thread.start()
        try:
            yield f"http://127.0.0.1:{server.server_port}", requests, prefix + name
        finally:
            server.shutdown()
            thread.join()


def installed_smoke(platform, sample, env, version, root):
    """Install with both supported hosts, then smoke each resulting command."""
    hosts = [(host, shutil.which(host)) for host in ("powershell", "pwsh")]
    if any(path is None for _, path in hosts):
        raise RuntimeError("installer smoke requires both Windows PowerShell 5.1 and PowerShell 7")
    with installer_release(platform, version) as (base, requests, asset):
        for host, path in hosts:
            target = root / ("installed-" + host)
            install_env = env | {"THINKTHEN_INSTALL_BASE": base, "THINKTHEN_INSTALL_API": base,
                                 "THINKTHEN_INSTALL_DIR": str(target)}
            before = len(requests)
            run([path, "-NoProfile", "-NonInteractive", "-File", str(REPO / "install.ps1"),
                 "-Version", version], install_env)
            if requests[before:] != [asset, asset + ".sha256"]:
                raise RuntimeError("explicit installer version requested unexpected release data")
            receipt = json.loads((target / "thinkthen.install.json").read_text())
            if receipt["state"] != "installed" or receipt["version"] != version:
                raise RuntimeError("installer did not commit its installed receipt")
            if {file.name for file in target.iterdir()} != {"thinkthen.exe", "thinkthen.install.json", ".thinkthen-install.lock"}:
                raise RuntimeError("installer left an unexpected target inventory")
            smoke(target / "thinkthen.exe", sample, env, version)
            print(f"Windows installer smoke: {host} installed the checked archive with 2 loopback downloads")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("platform", type=Path)
    parser.add_argument("--crate-dir", type=Path)
    args = parser.parse_args()
    if os.name != "nt":
        parser.error("Windows command smoke requires Windows")
    version = next(line.split('"')[1] for line in (REPO / "crates/thinkthen/Cargo.toml").read_text().splitlines()
                   if line.startswith('version = "'))
    COMMAND.platform(args.platform, version)
    with tempfile.TemporaryDirectory(prefix="thinkthen-windows-smoke-") as temporary:
        root = Path(temporary)
        env = {key: value for key, value in os.environ.items()
               if not key.upper().startswith(("THINKTHEN_", "XDG_")) and not key.upper().endswith("_API_KEY")}
        env.setdefault("CARGO_HOME", str(Path.home() / ".cargo"))
        env.setdefault("RUSTUP_HOME", str(Path.home() / ".rustup"))
        host = shutil.which("powershell")
        if host is None:
            raise RuntimeError("installer smoke requires Windows PowerShell 5.1")
        INSTALLER.prepare_private_fixture(root, host, env)
        env.update(HOME=str(root / "home"), APPDATA=str(root / "Roaming"),
                   LOCALAPPDATA=str(root / "Local"), CARGO_NET_OFFLINE="true")
        for folder in ("home", "Roaming", "Local", "command", "sample"):
            (root / folder).mkdir()
        with zipfile.ZipFile(args.platform / f"thinkthen-{version}-{COMMAND.TARGET}.zip") as archive:
            archive.extractall(root / "command")  # Verified one regular executable above.
        with tarfile.open(args.platform / "thinkthen-first-run.tar.gz") as archive:
            archive.extractall(root / "sample", filter="data")
        sample = root / "sample/thinkthen-first-run"
        run(["python", str(REPO / "sdlc/scripts/windows-installer-test.py"), "--binary",
             str(root / "command/thinkthen.exe"), "--version", version], env, timeout=600)
        installed_smoke(args.platform, sample, env, version, root)
        interrupt(root / "command/thinkthen.exe", env)
        python_smoke(args.platform, env, version, root)
        if args.crate_dir:
            wanted = args.crate_dir / f"thinkthen-{version}.crate"
            if list(args.crate_dir.glob("*.crate")) != [wanted] or wanted.is_symlink():
                raise RuntimeError("packed Rust crate inventory differs from the resolved version")
            with tarfile.open(wanted) as archive:
                archive.extractall(root / "source", filter="data")
            run(["cargo", "install", "--locked", "--offline", "--path",
                 str(root / f"source/thinkthen-{version}"), "--root", str(root / "crate"),
                 "--target", COMMAND.TARGET], env, timeout=1800)
            smoke(root / "crate/bin/thinkthen.exe", sample, env, version)


if __name__ == "__main__":
    main()
