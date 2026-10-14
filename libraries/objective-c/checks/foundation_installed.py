"""Install the supplied Apple package into an unrelated Foundation consumer."""
import collections
import contextlib
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import tarfile

ROOT = Path(__file__).resolve().parents[1]
REPO = ROOT.parents[1]
sys.path.insert(0, str(REPO / "conformance/children"))
from children import child_env
from backend import Backend
from session_cases import native_cases

if sys.platform != "darwin":
    print("Foundation installed consumer requires macOS", file=sys.stderr)
    sys.exit(77)
with contextlib.ExitStack() as installed:
    package = Path(os.environ.get("THINKTHEN_APPLE_PACKAGE", ROOT))
    if os.environ.get("THINKTHEN_ARTIFACT"):
        package = Path(installed.enter_context(tempfile.TemporaryDirectory(prefix="thinkthen-foundation-package-")))
        with tarfile.open(os.environ["THINKTHEN_ARTIFACT"]) as archive:
            archive.extractall(package, filter="data")
    if not (package / "CThinkThen.xcframework/Info.plist").is_file():
        print("Matching assembled Apple package with CThinkThen.xcframework is required; no native build is started", file=sys.stderr)
        sys.exit(77)
    consumer_stage = os.environ.get("THINKTHEN_COVERAGE_STAGE")
    if consumer_stage:
        Path(consumer_stage).mkdir(parents=True, exist_ok=True)
    consumer = contextlib.nullcontext(consumer_stage) if consumer_stage else tempfile.TemporaryDirectory(prefix="thinkthen-foundation-consumer-")
    with consumer as temporary:
        work = Path(temporary)
        shutil.copytree(package, work / "package", ignore=shutil.ignore_patterns("target", ".build", "checks", "Examples"))
        (work / "Sources/Consumer").mkdir(parents=True)
        (work / "Sources/Cases").mkdir(parents=True)
        (work / "Sources/Results").mkdir(parents=True)
        shutil.copy2(ROOT / "checks/FoundationResultsConsumer.m", work / "Sources/Results/main.m")
        # Fixture projection uses a local private declaration, never an exported module member.
        for target in ("Cases", "Results"):
            shutil.copy2(ROOT / "Sources/Foundation/TTResultPrivate.h", work / "Sources" / target / "TTResultPrivate.h")
        shutil.copy2(ROOT / "checks/FoundationCases.m", work / "Sources/Cases/main.m")
        shutil.copy2(ROOT / "checks/FoundationConsumer.m", work / "Sources/Consumer/main.m")
        (work / "Package.swift").write_text('''// swift-tools-version: 6.0
    import PackageDescription
    let package = Package(name: "FoundationConsumer", platforms: [.macOS(.v15)],
     dependencies: [.package(path: "package")], targets: [
     .executableTarget(name: "Consumer", dependencies: [
     .product(name: "ThinkThenFoundation", package: "package")],
     cSettings: [.unsafeFlags(["-fobjc-arc", "-fblocks"])],
     linkerSettings: [.linkedFramework("Foundation")]),
     .executableTarget(name: "Cases", dependencies: [
     .product(name: "ThinkThenFoundation", package: "package")],
     cSettings: [.unsafeFlags(["-fobjc-arc", "-fblocks"])],
     linkerSettings: [.linkedFramework("Foundation")]),
     .executableTarget(name: "Results", dependencies: [
     .product(name: "ThinkThenFoundation", package: "package")],
     cSettings: [.unsafeFlags(["-fobjc-arc", "-fblocks"])],
     linkerSettings: [.linkedFramework("Foundation")])])
    ''')
        env = child_env(keep=("LLVM_PROFILE_FILE", "CARGO_TARGET_DIR"), home=work)
        profile_link = []
        if os.environ.get("THINKTHEN_COVERAGE_INITIALIZER"):
            profile_link = ["-Xlinker", os.environ["THINKTHEN_COVERAGE_INITIALIZER"],
                            "-Xlinker", "-u", "-Xlinker", "___llvm_profile_runtime"]
        subprocess.run(["swift", "build", "--disable-sandbox", "-j", "1"] + profile_link, cwd=work, env=env,
                       check=True, timeout=300)
        subprocess.run([str(work / ".build/debug/Results"), str(REPO / "conformance/fixtures/complete.json")],
                       env=env, check=True, timeout=3)
        barrier = work / "barrier"
        barrier.mkdir()
        server = Backend(barrier, key='tt-canary-295', packed_questions={'Is it?'}, canonical_packed=True, hold_first=True)
        try:
            env.update(THINKTHEN_BASE_URL=f"http://127.0.0.1:{server.server_port}/generic/v1",
                       THINKTHEN_API_KEY="tt-canary-295", THINKTHEN_CACHE=str(work / "cache"))
            result = subprocess.run([str(work / ".build/debug/Consumer"), str(barrier)],
                                    cwd=work, env=env, text=True, capture_output=True, timeout=8)
            assert result.returncode == 0 and "FOUNDATION_INSTALLED_PASS" in result.stdout, (result.returncode, result.stdout, result.stderr)
            assert collections.Counter(server.arrivals) == collections.Counter(["consumer-objc", 1e308, 1e-308, "hold-foundation"]), server.arrivals
            print("Foundation installed consumer passed ownership and held-provider cancellation")
        finally:
            (barrier / "release-hold-foundation").touch()
            server.close()

        # Run the Objective-C inventory against the actual installed named calls.
        # The full profile removes the routine filter; it is candidate-only.
        if not (Path(os.environ.get("CARGO_TARGET_DIR", str(REPO / "target"))) / "debug/conformance-backend").is_file():
            raise RuntimeError("the recorded shared backend must be built before Foundation parity")
        # Foundation dictionaries have no insertion-order contract. Preserve the
        # authored fixture order through the public ordered-map builder.
        adapter = work / "ordered-cases"
        adapter.write_text("#!/usr/bin/env python3\n" + r'''import json, os, sys
from pathlib import Path

def ordered(value):
    if isinstance(value, dict):
        return {"kind":"object", "entries":[[key, ordered(item)] for key,item in value.items()]}
    if isinstance(value, list):
        return {"kind":"array", "items":[ordered(item) for item in value]}
    return {"kind":"scalar", "value":value}

source = Path(sys.argv[1])
fixture = json.loads(source.read_text())
fixture["ordered"] = {key:ordered(fixture[key]) for key in ("question", "input", "options", "feed_items") if key in fixture}
source.write_text(json.dumps(fixture, ensure_ascii=False, separators=(",", ":")))
binary = Path(__file__).parent / ".build/debug/Cases"
os.execve(str(binary), [str(binary), *sys.argv[1:]], os.environ)
''')
        adapter.chmod(0o700)
        native_cases(adapter, consumer="objective-c")
