#!/usr/bin/env python3
"""Static selected-input and refusal checks for release-language-tools.py."""

import importlib.util
import json
import os
from pathlib import Path
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
                owner = tools.OWNERS.get(path.name, "wrong-owner")
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
        finally:
            os.environ.clear()
            os.environ.update(old)
    print("release language tools self-test: selected inputs and refusals PASS")


if __name__ == "__main__":
    main()
