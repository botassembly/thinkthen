#!/usr/bin/env python3
"""Smoke the installed Windows command and optionally the packed Rust crate."""

import argparse
import http.server
import os
from pathlib import Path
import subprocess
import tarfile
import tempfile
import threading
import zipfile

import importlib.util


REPO = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location("windows_command", REPO / "sdlc/scripts/release-windows-command.py")
COMMAND = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(COMMAND)
QUESTION = "Does this report say what the person did before the problem appeared?"


def run(args, env, *, text=None, code=0, output=None, timeout=180):
    result = subprocess.run(args, env=env, input=text, capture_output=True, text=True, timeout=timeout)
    if result.returncode != code or (output is not None and result.stdout.strip() != output):
        raise RuntimeError(f"{args[0]} exited {result.returncode}: {result.stdout}{result.stderr}")
    return result


def smoke(binary, sample, env, version):
    """Replay through a counted loopback proxy and prove token refusal sends nothing."""
    requests = []

    class Counter(http.server.BaseHTTPRequestHandler):
        def do_POST(self):
            requests.append(self.path)
            self.send_error(500, "unexpected request in offline release smoke")

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
                 "--no-cache"], env, text=(sample / "report.txt").read_text(), output="true")
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
        env.update(HOME=str(root / "home"), APPDATA=str(root / "Roaming"),
                   LOCALAPPDATA=str(root / "Local"), CARGO_NET_OFFLINE="true")
        for folder in ("home", "Roaming", "Local", "command", "sample"):
            (root / folder).mkdir()
        with zipfile.ZipFile(args.platform / f"thinkthen-{version}-{COMMAND.TARGET}.zip") as archive:
            archive.extractall(root / "command")  # Verified one regular executable above.
        with tarfile.open(args.platform / "thinkthen-first-run.tar.gz") as archive:
            archive.extractall(root / "sample", filter="data")
        sample = root / "sample/thinkthen-first-run"
        smoke(root / "command/thinkthen.exe", sample, env, version)
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
