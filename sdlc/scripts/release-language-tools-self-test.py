#!/usr/bin/env python3
"""Static selected-input and refusal checks for release-language-tools.py."""

import importlib.util
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
from unittest.mock import patch
import zipfile

HERE = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location("release_language_tools", HERE / "release-language-tools.py")
tools = importlib.util.module_from_spec(spec)
spec.loader.exec_module(tools)
SHA = "a" * 40


def executable(path):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text("#!/bin/sh\nexit 88\n")
    path.chmod(0o755)


class Observer:
    """Return pinned external observations; never enforce the checked property."""

    def __init__(self, root):
        self.root = root
        self.calls = []
        self.versions = tools.PACKAGES.copy()
        self.provider = f"libncurses5-dev (= {tools.PACKAGES['libncurses-dev']}), ncurses-dev"
        self.java_version = "21.0.12"
        self.cc_owner = "gcc-13-x86-64-linux-gnu"

    def __call__(self, args, *, env=None):
        self.calls.append((args, env))
        if args == ["git", "rev-parse", "HEAD"]:
            return SHA
        if args[:3] == ["dpkg-query", "-W", "-f=${Version} ${Status}"]:
            return self.versions.get(args[3], "absent") + " install ok installed"
        if args == ["dpkg-query", "-W", "-f=${Provides}", "libncurses-dev"]:
            return self.provider
        if args[:2] == ["dpkg-query", "-S"]:
            path = Path(args[2])
            if path.parent == self.root / "jdk/bin":
                owner = "openjdk-21-jre-headless" if path.name == "java" else "openjdk-21-jdk-headless"
            else:
                owner = (self.cc_owner if path.name == "gcc-13"
                         else tools.OWNERS.get(path.name, "wrong-owner"))
            return f"{owner}: {path}"
        path = Path(args[0])
        if path.parent == self.root / "jdk/bin":
            return self.java_version
        if path.name == "dotnet":
            return "8.0.131"
        if path.name in ("kotlinc", "scalac"):
            return ("info: kotlinc-jvm 2.4.20 (JRE 21.0.12)" if path.name == "kotlinc"
                    else "Scala compiler version 3.9.0 -- fixture")
        return {"gprbuild": "GPRBUILD Pro 18.0w", "gnatmake": "GNATMAKE 13.3.0",
                "gcc-13": "gcc 13.3.0", "cobc": "cobc (GnuCOBOL) 4.0-early-dev.0",
                "bwrap": "bubblewrap 0.9.0"}.get(path.name, "")


def fixture(root):
    for name in ("java", "javac", "jar"):
        executable(root / "jdk/bin" / name)
    for name, member in (("kotlin", "kotlin-stdlib.jar"), ("scala", "scala.jar")):
        executable(root / name / "bin" / ("kotlinc" if name == "kotlin" else "scalac"))
        path = root / name / "lib" / member
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(b"fixture runtime")
    executable(root / "dotnet/dotnet")
    for name in ("gprbuild", "gnatmake", "gcc-13", "cobc", "bwrap"):
        executable(root / "bin" / name)
    (root / "bin/gcc").symlink_to("gcc-13")
    (root / "bin/cc").symlink_to("gcc-13")
    github_env, github_path = root / "github-env", root / "github-path"
    github_env.touch()
    github_path.touch()
    return github_env, github_path


def must_fail(action, phrase):
    try:
        action()
    except RuntimeError as error:
        assert phrase in str(error), (phrase, error)
    else:
        raise AssertionError(f"expected refusal: {phrase}")


def main():
    with tempfile.TemporaryDirectory(prefix="thinkthen-language-tools-test-") as folder:
        root = Path(folder)
        github_env, github_path = fixture(root)
        observer = Observer(root)
        old = os.environ.copy()
        os.environ.update(THINKTHEN_DOTNET=str(root / "dotnet/dotnet"),
                          THINKTHEN_JDK_HOME=str(root / "jdk"),
                          THINKTHEN_KOTLIN_HOME=str(root / "kotlin"),
                          THINKTHEN_SCALA_HOME=str(root / "scala"),
                          JAVA_HOME=str(root / "wrong-java"), JAVACMD=str(root / "wrong-java/bin/java"),
                          PATH=str(root / "bin") + os.pathsep + old.get("PATH", ""))
        try:
            with patch.object(tools.platform, "system", return_value="Linux"), \
                 patch.object(tools.platform, "machine", return_value="x86_64"):
                record = tools.setup("managed", root / "managed", SHA, False, github_env, github_path,
                                     observer)
                assert record["packages"]["versions"] == tools.MANAGED
                assert record["selected"]["java"] == str(root / "jdk/bin/java")
                assert json.loads((root / "managed/selection.json").read_text())["source_commit"] == SHA
                for args, env in observer.calls:
                    if args[0].endswith(("/kotlinc", "/scalac")):
                        assert env["JAVA_HOME"] == str(root / "jdk")
                        assert env["JAVACMD"] == str(root / "jdk/bin/java")
                        assert env["PATH"].startswith(str(root / "jdk/bin") + os.pathsep)
                assert "JAVACMD=" + str(root / "jdk/bin/java") in github_env.read_text()
                assert github_path.read_text() == str(root / "jdk/bin") + "\n"

                observer.calls.clear()
                smoke = tools.setup("smoke", root / "smoke", SHA, False, github_env, github_path,
                                    observer, bwrap=root / "bin/bwrap")
                assert smoke["packages"]["versions"] == tools.SMOKE
                assert smoke["selected"]["cc"].endswith("gcc-13")
                assert any(args[0].endswith("bwrap") and "--unshare-user" in args
                           for args, _ in observer.calls)
                assert all("install" not in args and "update" not in args for args, _ in observer.calls)

                rogue = root / "rogue/gcc-13"
                executable(rogue)
                (root / "bin/cc").unlink()
                (root / "bin/cc").symlink_to(rogue)
                must_fail(lambda: tools.check_smoke_tools(observer, root / "bin/bwrap"),
                          "not the selected GCC 13")
                (root / "bin/cc").unlink()
                (root / "bin/cc").symlink_to("gcc-13")
                observer.cc_owner = "unowned"
                must_fail(lambda: tools.check_smoke_tools(observer, root / "bin/bwrap"),
                          "selected cc package owner differs")
                observer.cc_owner = "gcc-13-x86-64-linux-gnu"

                observer.versions["gnucobol4"] = "3.1.2"
                must_fail(lambda: tools.setup("smoke", root / "wrong-package", SHA, False,
                                              github_env, github_path, observer,
                                              bwrap=root / "bin/bwrap"), "package set is absent")
                observer.versions["gnucobol4"] = tools.PACKAGES["gnucobol4"]
                observer.provider = "ncurses-dev"
                must_fail(lambda: tools.ensure_packages("smoke", root / "smoke", False, observer),
                          "does not provide")
                observer.provider = f"libncurses5-dev (= {tools.PACKAGES['libncurses-dev']}), ncurses-dev"
                observer.java_version = "17.0.1"
                observer.calls.clear()
                must_fail(lambda: tools.setup("managed", root / "wrong-java", SHA, False,
                                              github_env, github_path, observer), "selected JDK java differs")
                assert not any(args[0].endswith(("/kotlinc", "/scalac")) for args, _ in observer.calls)
                observer.java_version = "21.0.12"
                (root / "kotlin/lib/kotlin-stdlib.jar").unlink()
                observer.calls.clear()
                must_fail(lambda: tools.setup("managed", root / "missing-runtime", SHA, False,
                                              github_env, github_path, observer), "selected THINKTHEN_KOTLIN_HOME lacks")
                assert not any(args[0].endswith(("/kotlinc", "/scalac")) for args, _ in observer.calls)
                (root / "kotlin/lib/kotlin-stdlib.jar").write_bytes(b"fixture runtime")
                os.environ["THINKTHEN_DOTNET"] = str(root / "wrong-dotnet")
                observer.calls.clear()
                must_fail(lambda: tools.setup("managed", root / "wrong-selector", SHA, True,
                                              github_env, github_path, observer), "selected THINKTHEN_DOTNET differs")
                assert not any(args[0].endswith(("/kotlinc", "/scalac")) for args, _ in observer.calls)
                os.environ["THINKTHEN_DOTNET"] = str(root / "dotnet/dotnet")

                must_fail(lambda: tools.check_smoke_tools(observer, root / "missing-bwrap"),
                          "/usr/bin/bwrap differs")
                archive = root / "wrong-archive.zip"
                archive.write_bytes(b"different SDK bytes")
                must_fail(lambda: tools.verify_digest(archive, "sha256", "0" * 64), "pinned sha256")
                with patch.dict(os.environ, {"THINKTHEN_DOTNET": ""}), \
                     patch.object(tools.shutil, "which", return_value=None):
                    requested = []
                    def wrong_asset(url, path):
                        requested.append(url)
                        path.write_bytes(b"different SDK bytes")
                    must_fail(lambda: tools.sdk_path("dotnet", root / "managed", True, observer,
                                                     wrong_asset), "pinned sha512")
                    assert requested == [tools.ASSETS["dotnet"][0]]
                    assert not (root / "managed/dotnet").exists()
                with zipfile.ZipFile(root / "extra.zip", "w") as zip_file:
                    zip_file.writestr("../escape", b"bad")
                must_fail(lambda: tools.safe_extract(root / "extra.zip", root / "extract"),
                          "unsafe member")
                source_dir = root / "sources"
                source_dir.mkdir()
                sources = tools.snapshot_sources(source_dir).read_text()
                assert sources.count("Snapshot: " + tools.SNAPSHOT) == 2
                assert "archive.ubuntu.com" in sources and "security.ubuntu.com" in sources
                assert "packages.microsoft.com" not in sources and "ppa.launchpad.net" not in sources
                acquisition_cli(root)
        finally:
            os.environ.clear()
            os.environ.update(old)
    print("release language tools self-test: selected inputs and refusals PASS")


def acquisition_cli(root):
    """Run the public entry point with observed apt and download boundaries."""
    process_dir = root / "processes"
    process_dir.mkdir()
    log = root / "process-log"
    package = "openjdk-21-jdk"
    version = tools.PACKAGES[package]
    process = process_dir / "apt-cache"
    process.write_text("#!/usr/bin/env python3\nimport os, sys\n"
                       "from pathlib import Path\n"
                       "Path(os.environ['OBSERVE_LOG']).open('a').write('metadata\\n')\n"
                       "print(os.environ['OBSERVE_METADATA'])\n")
    process.chmod(0o755)
    process = process_dir / "apt-get"
    process.write_text("#!/usr/bin/env python3\nimport os, sys\n"
                       "from pathlib import Path\n"
                       "Path(os.environ['OBSERVE_LOG']).open('a').write('apt ' + ' '.join(sys.argv[1:]) + '\\n')\n"
                       "if 'install' in sys.argv and '-s' not in sys.argv: "
                       "Path(os.environ['OBSERVE_INSTALLED']).touch()\n"
                       "if '-s' in sys.argv: print(os.environ['OBSERVE_PLAN'])\n")
    process.chmod(0o755)
    process = process_dir / "sudo"
    process.write_text("#!/bin/sh\nexec \"$@\"\n")
    process.chmod(0o755)
    # The entry point's fetch boundary receives signed deterministic bytes.
    # The digest is patched only for these InRelease fixtures; no SDK is fetched.
    wrapper = ("import importlib.util, pathlib, sys\n"
               "spec = importlib.util.spec_from_file_location('tools', sys.argv[1])\n"
               "tools = importlib.util.module_from_spec(spec); spec.loader.exec_module(tools)\n"
               "tools.fetch = lambda url, path: path.write_bytes(url.encode())\n"
               "tools.RELEASES = {key: tools.hashlib.sha256((f'https://snapshot.ubuntu.com/ubuntu/'"
               "f'{tools.SNAPSHOT}/dists/{key}/InRelease').encode()).hexdigest() "
               "for key in tools.RELEASES}\n"
               "import os\n"
               "tools.package_version = lambda name, run=tools.command: "
               "tools.MANAGED[name] if pathlib.Path(os.environ['OBSERVE_INSTALLED']).exists() else None\n"
               "def setup(mode, job, sha, acquire, *handoff):\n"
               "    job.mkdir()\n"
               "    return {'mode': mode, 'packages': tools.ensure_packages(mode, job, acquire, get=tools.fetch)}\n"
               "tools.setup = setup\n"
               "try: tools.main(sys.argv[2:])\n"
               "except RuntimeError as error: print(error, file=sys.stderr); sys.exit(1)\n")
    env = os.environ.copy()
    env.update(PATH=str(process_dir) + os.pathsep + env["PATH"], OBSERVE_LOG=str(log),
               OBSERVE_INSTALLED=str(root / "installed"),
               OBSERVE_METADATA="\n\n".join(
                   f"Package: {name}\nVersion: {version}\nArchitecture: amd64\n"
                   "Filename: pool/main/o/openjdk.deb\nSHA256: " + "a" * 64
                   for name in (package, package + "-headless")),
               # apt-get -s run without root prints this note ahead of its plan.
               OBSERVE_PLAN=("NOTE: This is only a simulation!\n"
                             "      apt-get needs root privileges for real execution.\n"
                             "      Keep also in mind that locking is deactivated,\n"
                             "      so don't depend on the relevance to the real current situation!\n"
                             f"Reading package lists...\nInst {package} ({version} Ubuntu:24.04/noble-updates [amd64])\n"
                             # apt names both archives that hold one version, and may list broken packages.
                             f"Inst {package}-headless ({version} Ubuntu:24.04/noble-updates, "
                             f"Ubuntu:24.04/noble-security [amd64]) [{package}:amd64 ]"))
    # Keep this focused on the acquisition boundary: other installed inputs
    # are represented by the already exercised selected-home fixture.
    script = HERE / "release-language-tools.py"
    for label, plan, metadata, should_install in (
        ("good", env["OBSERVE_PLAN"], env["OBSERVE_METADATA"], True),
        ("foreign", "Reading package lists...\nInst foreign-library (99.0 thirdparty:foreign [amd64])",
         env["OBSERVE_METADATA"], False),
        ("foreign-snapshot", "Reading package lists...\nInst foreign-library (99.0 Ubuntu:24.04/noble [amd64])",
         env["OBSERVE_METADATA"], False),
        ("changed", f"Reading package lists...\nInst {package} (99.0 Ubuntu:24.04/noble-updates [amd64])",
         env["OBSERVE_METADATA"], False),
    ):
        Path(env["OBSERVE_INSTALLED"]).unlink(missing_ok=True)
        log.unlink(missing_ok=True)
        env.update(OBSERVE_PLAN=plan, OBSERVE_METADATA=metadata)
        command = [sys.executable, "-c", wrapper, str(script), "managed",
                   str(root / ("acquire-" + label)), SHA, "--acquire"]
        # Use the real CLI argument parser and acquisition path. Intercept
        # setup after package selection to avoid unrelated SDK work.
        result = subprocess.run(command, env=env, text=True, capture_output=True, check=False)
        assert result.returncode == (0 if should_install else 1), result
        if label == "foreign":
            assert "unrecognized or duplicate selection" in result.stderr, result
        if label == "foreign-snapshot":
            assert "outside signed snapshot metadata" in result.stderr, result
        if label == "changed":
            assert "substituted pinned package" in result.stderr, result
        assert Path(env["OBSERVE_INSTALLED"]).exists() == should_install
        assert "-s" in log.read_text(), (label, log.read_text())
        assert ("metadata" in log.read_text()) == (label in ("good", "foreign-snapshot"))


if __name__ == "__main__":
    main()
