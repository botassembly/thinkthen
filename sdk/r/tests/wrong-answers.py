"""Run installed R regressions using the same native engine."""
import hashlib
import fcntl
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
import json
import os
from pathlib import Path
import shlex
import shutil
import subprocess
import tempfile
import sys
import threading

ROOT = Path(__file__).resolve().parents[3]
if sys.argv[1:] not in ([], ["--routine"], ["--full"]):
    raise SystemExit("usage: scripts/r-wrong-answer-gate [--routine|--full]")


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


# The release producer records every source input and each immutable output.
# A facade-only change can reuse its archive, but a changed engine cannot.
version = subprocess.check_output(["R", "--version"], text=True).splitlines()[0]
metadata = {"ffi", "target", "R", "packages", "profile"}
archive = None
print("R runtime:", version, flush=True)
for receipt in sorted((Path.home() / ".local/share/thinkthen-build-tools/native").glob("*/release/*/artifacts/identity.json")):
    manifest = json.loads(receipt.read_text())
    inputs = manifest["inputs"]
    changed = [name for name, value in inputs.items() if name not in metadata
               and (not (ROOT / name).is_file() or digest(ROOT / name) != value)]
    if inputs["R"] != version or changed:
        continue
    candidate = receipt.parent / "libthinkthen_r.a"
    if digest(candidate) == manifest["artifacts"][candidate.name]:
        archive = candidate
        break
if archive is not None:
    print("Reuse verified R archive:", archive, digest(archive), flush=True)

requests = []


class Handler(BaseHTTPRequestHandler):
    def do_POST(self):
        self.rfile.read(int(self.headers.get("Content-Length", 0)))
        requests.append(self.path)
        self.send_response(500)
        self.end_headers()

    def log_message(self, *_):
        pass


with tempfile.TemporaryDirectory(prefix="thinkthen-r-wrong-answers-") as temporary:
    scratch = Path(temporary)
    if archive is None:
        # This isolated debug consumer never invokes the release/native producer.
        target = Path.home() / ".local/share/thinkthen-build-tools/r-interface-debug"
        target.mkdir(parents=True, exist_ok=True)
        with (target / "build.lock").open("w") as lock:
            fcntl.flock(lock, fcntl.LOCK_EX)
            build_environment = dict(os.environ, CARGO_NET_OFFLINE="true",
                R_HOME=subprocess.check_output(["R", "RHOME"], text=True).strip())
            subprocess.run(["cargo", "build", "--offline", "--locked", "-p", "thinkthen-r",
                            "--target-dir", str(target)], cwd=ROOT, env=build_environment, check=True)
            archive = scratch / "libthinkthen_r.a"
            shutil.copy2(target / "debug/libthinkthen_r.a", archive)
            (target / "source.json").write_text(json.dumps({
                "revision": subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=ROOT, text=True).strip(),
                "R": version, "archive_sha256": digest(archive),
            }, indent=2) + "\n")
        print("Use isolated debug R archive:", digest(archive), flush=True)
    stage = scratch / "thinkthen"
    stage.mkdir()
    for name in ("DESCRIPTION", "NAMESPACE", "LICENSE"):
        shutil.copy2(ROOT / "sdk/r" / name, stage / name)
    shutil.copytree(ROOT / "sdk/r/R", stage / "R")
    shutil.copytree(ROOT / "sdk/r/man", stage / "man")
    libraries = stage / "inst/libs"
    libraries.mkdir(parents=True)
    shutil.copy2(ROOT / "sdk/r/src/entrypoint.c", scratch / "entrypoint.c")
    frameworks = shlex.split(subprocess.check_output([str(ROOT / "scripts/native-frameworks")], text=True))
    subprocess.run(["R", "CMD", "SHLIB", "entrypoint.c", str(archive), *frameworks,
                    "-o", str(libraries / "thinkthen.so")], cwd=scratch, check=True)
    library = scratch / "library"
    library.mkdir()
    (scratch / "home").mkdir()
    # Prepared dependency packages are explicit inputs, separate from user config.
    prepared = os.environ["R_LIBS_USER"]
    environment = {name: os.environ[name] for name in ("PATH", "R_HOME") if name in os.environ}
    environment.update(HOME=str(scratch / "home"), R_LIBS_USER=str(library) + os.pathsep + prepared,
                       R_LIBS_SITE="", THINKTHEN_API_KEY="synthetic-local-test-key",
                       OPENAI_API_KEY="synthetic-local-test-key")
    subprocess.run(["R", "CMD", "INSTALL", "--library=" + str(library), str(stage)],
                   cwd=scratch, env=environment, check=True)
    failures = []
    server = ThreadingHTTPServer(("127.0.0.1", 0), Handler)
    worker = threading.Thread(target=server.serve_forever)
    worker.start()
    try:
        environment["THINKTHEN_BASE_URL"] = environment["THINKTHEN_TEST_URL"] = f"http://127.0.0.1:{server.server_port}/v1"
        if sys.argv[1:]:
            for script in ("contract.R", "typed-surface.R", "typed-frames.R", "surface.R", "frames.R", "frame-serialization.R"):
                result = subprocess.run(["Rscript", "--vanilla", str(ROOT / "sdk/r/tests" / script)],
                                        cwd=scratch, env=environment)
                if result.returncode:
                    failures.append(script)
        result = subprocess.run(["Rscript", "--vanilla", str(ROOT / "sdk/r/tests/interface.R")], cwd=scratch, env=environment)
        if result.returncode:
            failures.append("interface")
        for case in ("rank", "client"):
            result = subprocess.run(["Rscript", "--vanilla", str(ROOT / "sdk/r/tests/wrong-answers.R"), case],
                                    cwd=scratch, env=environment)
            if result.returncode:
                failures.append(case)
        print("Loopback default-backend requests:", len(requests), flush=True)
        if requests:
            failures.append("unexpected backend requests")
    finally:
        server.shutdown()
        worker.join()
        server.server_close()
    if sys.argv[1:] == ["--full"] and not failures:
        build_environment = dict(os.environ, CARGO_NET_OFFLINE="true")
        target = Path.home() / ".local/share/thinkthen-build-tools/r-interface-debug"
        build_environment["CARGO_TARGET_DIR"] = str(target)
        subprocess.run(["cargo", "build", "--offline", "--locked", "-p", "loopback", "--bin", "loopback"], cwd=ROOT, env=build_environment, check=True)
        environment["THINKTHEN_LOOPBACK"] = str(target / "debug/loopback")
        subprocess.run(["python3", str(ROOT / "sdk/r/tests/driver.py"), "--root", str(ROOT), "--library", str(library), "--scratch", str(scratch)], cwd=ROOT, env=environment, check=True)
    if sys.argv[1:] and not failures:
        # Inspect the genuine source package, using the already tested current
        # module instead of compiling the native engine a second time.
        source_dir = scratch / "source-package"
        subprocess.run(["python3", str(ROOT / "scripts/r-package"), str(source_dir)], cwd=ROOT, check=True)
        source_archive = next(source_dir.glob("thinkthen_*.tar.gz"))
        checked = subprocess.run(["R", "CMD", "check", "--no-install", "--library=" + str(library), "--no-tests", "--no-manual", "--no-vignettes", str(source_archive)], cwd=scratch, env=environment)
        report = (scratch / "thinkthen.Rcheck/00check.log").read_text()
        if checked.returncode or " WARNING" in report or " NOTE" in report:
            failures.append("R CMD check")
    if failures:
        raise SystemExit("R public regressions failed: " + ", ".join(failures))
print("Installed R wrong-answer regressions passed; zero backend requests")
