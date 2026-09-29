"""Run copied Go modules against independently counted loopback backends."""
import collections
import json
import os
from pathlib import Path
import shutil
import subprocess

from backend import Backend

ROOT = Path(__file__).resolve().parents[3]
PORT = ROOT / "libraries/go"
OUT = ROOT / "target/go"
NATIVE = OUT / "native"
GO = os.environ.get("THINKTHEN_GO_BIN", "go")
EXPECTED = [json.loads(row) for row in (PORT / "fixtures/accepted_requests.jsonl").read_text().splitlines()]
STATES = json.loads((PORT / "fixtures/expected_arrivals.json").read_text())


def multiset(rows):
    return collections.Counter(json.dumps(row, sort_keys=True, ensure_ascii=False, separators=(",", ":")) for row in rows)


def checked(command, *, cwd, env, marker=""):
    result = subprocess.run(command, cwd=cwd, env=env, text=True, capture_output=True, timeout=180)
    if result.returncode or marker and marker not in result.stdout + result.stderr:
        raise AssertionError((command, result.returncode, result.stdout[-1000:], result.stderr[-1000:]))
    return result


def main():
    assert len(EXPECTED) == len(STATES) == 39
    results = []
    for mode in ("shared", "static"):
        for number in (1, 2):
            label = f"{mode}-{number}"
            folder = OUT / "consumers" / label
            if folder.exists():
                shutil.rmtree(folder)
            module = folder / "module"
            shutil.copytree(PORT, module)
            barrier = folder / "barrier"
            barrier.mkdir()
            cache = folder / "cache"
            cache.mkdir()
            server = Backend(barrier)
            try:
                pc = NATIVE / "lib/pkgconfig"
                env = {"PATH": os.environ.get("PATH", "/usr/bin:/bin"), "HOME": str(folder),
                       "GOCACHE": str(OUT / "cache"), "GOMODCACHE": str(OUT / "modcache"),
                       "GOPROXY": "off", "GOSUMDB": "off", "GOTOOLCHAIN": "local",
                       "CGO_ENABLED": "1", "PKG_CONFIG_PATH": str(pc),
                       "LD_LIBRARY_PATH": str(NATIVE / "lib"),
                       "THINKTHEN_API_KEY": "tt-canary-274",
                       "THINKTHEN_BASE_URL": f"http://127.0.0.1:{server.server_port}/generic/v1",
                       "THINKTHEN_CACHE": str(cache), "TT_BARRIER_DIR": str(barrier)}
                if mode == "static":
                    pc = folder / "pkgconfig"
                    pc.mkdir()
                    (pc / "thinkthen.pc").write_text(
                        f"Name: thinkthen\nDescription: installed static C\nVersion: 0.0.1\n"
                        f"Cflags: -I{NATIVE / 'include'}\n"
                        f"Libs: {NATIVE / 'lib/libthinkthen.a'} -lgcc_s -lutil -lrt -lpthread -lm -ldl -lc\n")
                    env["PKG_CONFIG_PATH"] = str(pc)
                    env["CGO_LDFLAGS_ALLOW"] = r"^" + str(NATIVE / "lib/libthinkthen.a").replace(".", r"\.") + r"$"
                first = r"^(TestMatrix|TestHeldContext|TestCancellationGoroutinesSettle|TestConstructorFailureCopy|TestFailuresAndRecovery)$"
                for expression in (first, r"^TestConcurrent$", r"^TestHeldScalarContract$"):
                    case_env = env | ({"TT_CHECK_SCALAR_CONTRACT": "1"} if expression == r"^TestHeldScalarContract$" else {})
                    checked([GO, "test", "-race", "-count=1", "-run", expression, "."], cwd=module, env=case_env)
                (folder / "requests.jsonl").write_text("".join(json.dumps(row, ensure_ascii=False) + "\n" for row in server.requests))
                assert len(server.arrivals) == len(server.requests) == 39, (label, len(server.arrivals),
                    multiset(STATES) - multiset(server.arrivals), multiset(server.arrivals) - multiset(STATES))
                assert multiset(server.arrivals) == multiset(STATES), label
                assert multiset(server.requests) == multiset(EXPECTED), (label,
                    multiset(EXPECTED) - multiset(server.requests), multiset(server.requests) - multiset(EXPECTED))
                assert server.attempts == 39 and server.connections >= 39, label
                assert server.bulk_completion == ["packed:first,second,third"], label
                consumer = folder / "external"
                consumer.mkdir()
                (consumer / "go.mod").write_text(
                    "module example.org/installed-consumer\n\ngo 1.22\n\n"
                    "require github.com/botassembly/thinkthen/libraries/go v0.0.1\n"
                    f"replace github.com/botassembly/thinkthen/libraries/go => {module}\n")
                shutil.copyfile(module / "examples/decide/main.go", consumer / "main.go")
                external = checked([GO, "run", "-p", "2", "."], cwd=consumer,
                                   env=env | {"THINKTHEN_CACHE": str(folder / "external-cache")},
                                   marker="outcome=1 probability=0.9")
                assert external.returncode == 0 and len(server.arrivals) == 40 and server.arrivals[-1] == "café", label
                results.append({"consumer": label, "arrivals": 39, "full_body_match": True,
                                "external_arrivals": 1, "attempts": server.attempts, "connections": server.connections})
            finally:
                server.close()
    (OUT / "matrix.json").write_text(json.dumps(results, indent=2) + "\n")
    # The new constructor gets its own fresh ledger, so the accepted 39-body
    # baseline remains a useful regression check for the retained matrix.
    configured = OUT / "consumers/shared-1"
    barrier = configured / "settings-barrier"
    barrier.mkdir(exist_ok=True)
    server = Backend(barrier)
    try:
        env = {"PATH": os.environ.get("PATH", "/usr/bin:/bin"), "HOME": str(configured),
               "GOCACHE": str(OUT / "cache"), "GOMODCACHE": str(OUT / "modcache"),
               "GOPROXY": "off", "GOSUMDB": "off", "GOTOOLCHAIN": "local", "CGO_ENABLED": "1",
               "PKG_CONFIG_PATH": str(NATIVE / "lib/pkgconfig"), "LD_LIBRARY_PATH": str(NATIVE / "lib"),
               "THINKTHEN_API_KEY": "tt-canary-274",
               "THINKTHEN_BASE_URL": f"http://127.0.0.1:{server.server_port}/generic/v1",
               "THINKTHEN_CACHE": str(configured / "settings-cache"), "TT_BARRIER_DIR": str(barrier)}
        checked([GO, "test", "-race", "-count=1", "-run", "^TestSettingsConstructor$", "."],
                cwd=configured / "module", env=env)
        assert server.arrivals == ["configured-go"], server.arrivals
    finally:
        server.close()
    negative_barrier = configured / "wrong-detail-barrier"
    negative_barrier.mkdir(exist_ok=True)
    negative = Backend(negative_barrier)
    try:
        wrong_env = env | {"THINKTHEN_BASE_URL": f"http://127.0.0.1:{negative.server_port}/generic/v1",
                           "THINKTHEN_CACHE": str(configured / "wrong-detail-cache"),
                           "TT_BARRIER_DIR": str(negative_barrier), "TT_PLANT_WRONG_DETAIL": "1"}
        wrong = subprocess.run([GO, "test", "-race", "-count=1", "-run", "^TestMatrix$", "."],
                               cwd=configured / "module", env=wrong_env, text=True, capture_output=True, timeout=180)
        assert wrong.returncode == 1 and "details changed:" in wrong.stdout and negative.arrivals, wrong.stdout
    finally:
        negative.close()
    print("GO_INSTALLED_MATRIX_PASS 4 consumers x 39 exact full request bodies plus one external module call each")
    print("GO_SETTINGS_PASS one configured request; invalid object sent nothing")
    print("GO_WRONG_DETAIL_PLANT_REJECTED")


if __name__ == "__main__":
    main()
