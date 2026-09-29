"""Build and run an installed Zig source package in four isolated consumers."""
from collections import Counter
import json
import os
from pathlib import Path
import shutil
import tarfile
import tempfile

from backend import Backend
from process_group import run

PACKAGE = Path(__file__).resolve().parent.parent
ROOT = PACKAGE / "target"
SOURCE = ROOT / "artifacts/thinkthen-zig-0.0.1-src.tar.gz"
NATIVE = ROOT / "artifacts/thinkthen-c-0.0.1-x86_64-linux-gnu.tar.gz"
ZIG = Path(shutil.which("zig") or "").resolve()
assert ZIG.is_file() and ZIG.name == "zig", "configured Zig executable unavailable"
expected = json.loads((PACKAGE / "Tests/expected_requests.json").read_text())
normalize = lambda rows: Counter(json.dumps(row, sort_keys=True, ensure_ascii=False, separators=(",", ":")) for row in rows)
ROOT.joinpath("logs").mkdir(parents=True, exist_ok=True)

for mode in ("shared", "static"):
    for index in (0, 1):
        with tempfile.TemporaryDirectory(prefix=f"zig-installed-{mode}-{index}-", dir=ROOT / "logs") as folder:
            trial = Path(folder)
            for name in ("package", "native", "project", "home", "cache", "barrier"):
                trial.joinpath(name).mkdir()
            with tarfile.open(SOURCE) as archive:
                archive.extractall(trial / "package", filter="data")
            with tarfile.open(NATIVE) as archive:
                archive.extractall(trial / "native", filter="data")
            for name in ("build.zig", "build.zig.zon", "matrix.zig", "allocation.zig", "concurrent.zig", "type_case.zig", "settings.zig"):
                shutil.copy2(PACKAGE / "Tests" / name, trial / "project" / name)
            metadata = trial / "project/build.zig.zon"
            metadata.write_text(metadata.read_text().replace('.path = "../"', '.path = "../package"'))
            backend = Backend(trial / "barrier")
            receipts = []
            try:
                empty = trial / "empty-tool"
                empty.touch()
                base = ["bwrap", "--unshare-all", "--share-net", "--die-with-parent", "--clearenv",
                        "--ro-bind", "/usr", "/usr", "--symlink", "usr/bin", "/bin",
                        "--ro-bind", "/lib", "/lib", "--ro-bind", "/lib64", "/lib64",
                        "--ro-bind", "/etc", "/etc", "--ro-bind", str(ZIG.parent), "/zig",
                        "--ro-bind", str(empty), "/usr/bin/cargo", "--ro-bind", str(empty), "/usr/bin/rustc",
                        "--bind", str(trial), "/work", "--dir", "/home", "--tmpfs", "/tmp",
                        "--proc", "/proc", "--dev", "/dev", "--chdir", "/work/project",
                        "--setenv", "PATH", "/zig:/usr/bin:/bin", "--setenv", "HOME", "/work/home",
                        "--setenv", "ZIG_GLOBAL_CACHE_DIR", "/work/cache",
                        "--setenv", "LD_LIBRARY_PATH", "/work/native/lib",
                        "--setenv", "THINKTHEN_CACHE", "/work/home/native-cache",
                        "--setenv", "THINKTHEN_API_KEY", "tt-canary-273",
                        "--setenv", "TT_BARRIER_DIR", "/work/barrier",
                        "--setenv", "THINKTHEN_BASE_URL", f"http://127.0.0.1:{backend.server_port}/generic/v1"]
                def command(label, argv, count, marker):
                    result = run(base + ["--", *argv], cwd=trial / "project", env={"PATH": os.environ.get("PATH", "/usr/bin:/bin")}, timeout=100)
                    output = result.stdout + result.stderr
                    receipts.append({"case": label, "exit": result.exit, "signals": result.signals,
                                     "arrivals": len(backend.arrivals)})
                    assert result.exit == 0 and marker.encode() in output, (mode, index, label, result.exit, output[-1500:])
                    assert len(backend.arrivals) == count, (mode, index, label, len(backend.arrivals), count)
                command("namespace", ["/bin/sh", "-c", "test ! -x /usr/bin/cargo && test ! -x /usr/bin/rustc && test ! -e /work/source && echo NAMESPACE_PASS"], 0, "NAMESPACE_PASS")
                command("build", ["/zig/zig", "build", "-j2", f"-Dlink-mode={mode}", "-Dnative=/work/native"], 0, "")
                command("example-build", ["/zig/zig", "build", "-j2", f"-Dlink-mode={mode}", "-Dnative=/work/native", "--build-file", "/work/package/build.zig", "--prefix", "/work/package/zig-out"], 0, "")
                command("example", ["/work/package/zig-out/bin/thinkthen-example"], 1, "yes 0.90")
                command("matrix", ["/work/project/zig-out/bin/matrix"], 32, "matrix: ten verbs")
                command("allocation", ["/work/project/zig-out/bin/allocation"], 34, "allocation: bulk indexes")
                command("concurrent", ["/work/project/zig-out/bin/concurrent", "--callers-only"], 37, "concurrent: three callers PASS")
                command("held", ["/work/project/zig-out/bin/concurrent", "--holds-only"], 42, "fresh-token recovery recovery-scalar PASS")
                assert normalize(backend.arrivals) == normalize(expected), (mode, index, "wire bodies")
                assert backend.bulk_completion == [], (mode, index, backend.bulk_completion)
                print(f"Zig installed {mode}-{index}: 42 exact request bodies PASS", flush=True)
            finally:
                (ROOT / "logs" / f"installed-{mode}-{index}.json").write_text(json.dumps(receipts, indent=2) + "\n")
                backend.close()
