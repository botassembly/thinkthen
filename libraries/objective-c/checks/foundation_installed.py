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
sys.path.insert(0, str(REPO / "libraries/cpp/fixtures"))
from children import child_env
from backend import Backend
from session_cases import native_cases

if sys.platform != "darwin":
    print("Foundation installed consumer requires macOS", file=sys.stderr)
    sys.exit(77)
# The shared carrier fixture checks host conversion only; it is not execution parity.
with tempfile.TemporaryDirectory(prefix="thinkthen-foundation-results-") as temporary:
    work = Path(temporary)
    source = ROOT / "Sources/Foundation"
    subprocess.run(["xcrun", "clang", "-fobjc-arc", "-fblocks", "-Wall", "-Wextra", "-Werror",
                    "-I" + str(source), str(source / "TTResult.m"), str(source / "TTResults.g.m"),
                    str(ROOT / "checks/FoundationResultsConsumer.m"), "-framework", "Foundation",
                    "-o", str(work / "results")], env=child_env(home=work), check=True, capture_output=True, timeout=30)
    subprocess.run([str(work / "results"), str(REPO / "libraries/python/tests/fixtures/complete.json")],
                   env=child_env(home=work), check=True, timeout=3)
with contextlib.ExitStack() as installed:
    package = Path(os.environ.get("THINKTHEN_APPLE_PACKAGE", ROOT))
    if os.environ.get("THINKTHEN_ARTIFACT"):
        package = Path(installed.enter_context(tempfile.TemporaryDirectory(prefix="thinkthen-foundation-package-")))
        with tarfile.open(os.environ["THINKTHEN_ARTIFACT"]) as archive:
            archive.extractall(package, filter="data")
    if not (package / "CThinkThen.xcframework/Info.plist").is_file():
        print("Matching assembled Apple package with CThinkThen.xcframework is required; no native build is started", file=sys.stderr)
        sys.exit(77)
    with tempfile.TemporaryDirectory(prefix="thinkthen-foundation-consumer-") as temporary:
        work = Path(temporary)
        shutil.copytree(package, work / "package", ignore=shutil.ignore_patterns("target", ".build", "checks", "Examples"))
        (work / "Sources/Consumer").mkdir(parents=True)
        (work / "Sources/Cases").mkdir(parents=True)
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
     linkerSettings: [.linkedFramework("Foundation")])])
    ''')
        env = child_env(home=work)
        subprocess.run(["swift", "build", "--disable-sandbox", "-j", "1"], cwd=work, env=env,
                       check=True, capture_output=True, timeout=60)
        barrier = work / "barrier"
        barrier.mkdir()
        server = Backend(barrier)
        try:
            env.update(THINKTHEN_BASE_URL=f"http://127.0.0.1:{server.server_port}/generic/v1",
                       THINKTHEN_API_KEY="tt-canary-295", THINKTHEN_CACHE=str(work / "cache"))
            result = subprocess.run([str(work / ".build/debug/Consumer"), str(barrier)],
                                    cwd=work, env=env, text=True, capture_output=True, timeout=8)
            assert result.returncode == 0 and "FOUNDATION_INSTALLED_PASS" in result.stdout, (result.stdout, result.stderr)
            assert collections.Counter(server.arrivals) == collections.Counter(["consumer-objc", "hold-foundation"]), server.arrivals
            print("Foundation installed consumer passed ownership and held-provider cancellation")
        finally:
            (barrier / "release-hold-foundation").touch()
            server.close()

        # Run the Objective-C inventory against the actual installed named calls.
        # The full profile removes the routine filter; it is candidate-only.
        if not (REPO / "target/debug/conformance-backend").is_file():
            raise RuntimeError("the recorded shared backend must be built before Foundation parity")
        native_cases(work / ".build/debug/Cases", consumer="objective-c")
