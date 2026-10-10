#!/usr/bin/env python3
"""Select the pinned Linux x86 language tools for release build or smoke.

Usage: release-language-tools.py managed|smoke JOB_DIR EXPECTED_SHA [--acquire]
The caller supplies GITHUB_ENV and GITHUB_PATH. Acquisition is opt-in.
"""

import hashlib
import http.client
import json
import os
from pathlib import Path
import platform
import re
import shutil
import subprocess
import sys
import tarfile
import time
import urllib.error
import urllib.request
import zipfile
import xml.etree.ElementTree as ET

SNAPSHOT = "20260923T150000Z"
RELEASES = {
    "noble": "cdb2f31d809f589719a53c6ad15f255b27569c4059542ada282aaa21b8e164b0",
    "noble-updates": "bfe315bdaacb92480f6d1d14387e2fc23d3513207a04c4185f7254e468498736",
    "noble-security": "b8b375a06aeada33b0ff9a445647bf790e17ea296b3ce709ca8d9f839bb356d1",
}
PACKAGES = {
    "bubblewrap": "0.9.0-1ubuntu0.3",
    "gprbuild": "2024.1.20231009-5~24.04",
    "gnat-13": "13.3.0-6ubuntu2~24.04.1",
    "gnat-13-x86-64-linux-gnu": "13.3.0-6ubuntu2~24.04.1",
    "gcc": "4:13.2.0-7ubuntu1",
    "gcc-13": "13.3.0-6ubuntu2~24.04.1",
    "gcc-13-base": "13.3.0-6ubuntu2~24.04.1",
    "gcc-13-x86-64-linux-gnu": "13.3.0-6ubuntu2~24.04.1",
    "cpp-13": "13.3.0-6ubuntu2~24.04.1",
    "gobjc-13": "13.3.0-6ubuntu2~24.04.1",
    "gobjc-13-x86-64-linux-gnu": "13.3.0-6ubuntu2~24.04.1",
    "libobjc-13-dev": "13.3.0-6ubuntu2~24.04.1",
    "libobjc4": "14.2.0-4ubuntu2~24.04.1",
    "gnucobol4": "4.0~early~20200606-6.1build1",
    "libcob5-dev": "4.0~early~20200606-6.1build1",
    "libcob5t64": "4.0~early~20200606-6.1build1",
    "libncurses-dev": "6.4+20240113-1ubuntu2.2",
    "libncurses6": "6.4+20240113-1ubuntu2.2",
    "libncursesw6": "6.4+20240113-1ubuntu2.2",
    "libtinfo6": "6.4+20240113-1ubuntu2.2",
}
MANAGED = {}
SMOKE = PACKAGES
ASSETS = {
    "dotnet": ("https://builds.dotnet.microsoft.com/dotnet/Sdk/8.0.131/dotnet-sdk-8.0.131-linux-x64.tar.gz",
               "sha512", "a6182d3f136c524485da248156b263012eba70179a0849131b5331548770fc149a42347f7865fdbd4f81057b28fa68c2395ab0e9549f2c8a268310f48a2304eb"),
    "kotlin": ("https://github.com/JetBrains/kotlin/releases/download/v2.4.20/kotlin-compiler-2.4.20.zip",
                "sha256", "59e9ca74c7904ef2c122b12114937673ccce68de820a663f0ed66ccf8799e0b7"),
    "scala": ("https://github.com/scala/scala3/releases/download/3.9.0/scala3-3.9.0.tar.gz",
               "sha256", "8ecddee33ecda620256e4b744b928d3ad6de64653bd41e4b6a308992b3b6bb31"),
}
OWNERS = {"gprbuild": "gprbuild", "gnatmake": "gnat-13", "gcc": "gcc",
          "cobc": "gnucobol4", "bwrap": "bubblewrap"}


def fail(message):
    raise RuntimeError(message)


def command(args, *, env=None):
    result = subprocess.run(args, env=env, text=True, capture_output=True, check=False)
    if result.returncode:
        fail(f"command failed ({result.returncode}): {args[0]}: {result.stderr.strip()}")
    return (result.stdout + result.stderr).strip()


def digest(path, kind):
    h = hashlib.new(kind)
    with path.open("rb") as source:
        for part in iter(lambda: source.read(1024 * 1024), b""):
            h.update(part)
    return h.hexdigest()


def verify_digest(path, kind, expected):
    if digest(path, kind) != expected:
        fail(f"{path.name} differs from the pinned {kind}")


def fetch(url, path, opener=urllib.request.urlopen, pause=time.sleep):
    if path.exists() or path.is_symlink():
        fail(f"download output already exists: {path}")
    # Retry transient request and body failures within the same three-attempt limit.
    # Remove only this download's partial output before retrying.
    for attempt in range(3):
        try:
            with opener(url, timeout=60) as response:
                headers = getattr(response, "headers", {})
                if getattr(response, "status", None) == 206 or headers.get("Content-Range"):
                    raise urllib.error.URLError("download returned a partial response")
                with path.open("xb") as output:
                    shutil.copyfileobj(response, output)
                    length = headers.get("Content-Length")
                    if length is not None and output.tell() != int(length):
                        raise http.client.IncompleteRead(b"", int(length) - output.tell())
            return
        except urllib.error.HTTPError as error:
            if attempt == 2 or (error.code < 500 and error.code != 429):
                raise
        except (urllib.error.URLError, TimeoutError, ConnectionError, http.client.IncompleteRead):
            path.unlink(missing_ok=True)
            if attempt == 2:
                raise
        pause(10 * (attempt + 1))


def package_version(name, run=command):
    try:
        output = run(["dpkg-query", "-W", "-f=${Version} ${Status}", name])
    except RuntimeError:
        return None
    version, _, status = output.partition(" ")
    return version if status == "install ok installed" else None


def package_set(mode):
    return MANAGED if mode == "managed" else SMOKE


def snapshot_sources(job):
    source = job / "ubuntu.sources"
    if source.exists() or source.is_symlink():
        fail("snapshot source file already exists")
    rows = []
    for uri, suites in (("http://archive.ubuntu.com/ubuntu", "noble noble-updates"),
                        ("http://security.ubuntu.com/ubuntu", "noble-security")):
        rows.append("Types: deb\nURIs: " + uri + "\nSuites: " + suites +
                    "\nComponents: main universe\nSigned-By: /usr/share/keyrings/ubuntu-archive-keyring.gpg\n" +
                    "Snapshot: " + SNAPSHOT + "\n")
    source.write_text("\n".join(rows))
    return source


def selected_plan(plan):
    selected = {}
    for line in plan.splitlines():
        if line.startswith(("Remv ", "Conf ")):
            if line.startswith("Remv "):
                fail("apt plan removes an installed package")
            continue
        if not line.startswith("Inst "):
            continue
        # apt lists every archive that holds the version, joined by ", ", and may
        # end the line with the packages the step leaves broken until later steps.
        archive = r"Ubuntu:24\.04/noble(?:-updates|-security)?"
        match = re.fullmatch(r"Inst ([a-z0-9][a-z0-9+.-]*) (?:\[[^]]+\] )?"
                             rf"\(([^ ()]+) {archive}(?:, {archive})* \[(?:amd64|all)\]\)(?: \[[^]]*\])?",
                             line)
        if not match or match[1] in selected:
            fail(f"apt plan contains an unrecognized or duplicate selection: {line}")
        selected[match[1]] = match[2]
    if not selected:
        fail("apt did not produce a package acquisition plan")
    return selected


def snapshot_package(name, version, apt, run):
    # The empty status file excludes locally installed packages. apt's isolated
    # lists contain only indexes authenticated by the snapshot source above.
    metadata = run(["apt-cache", *apt[1:], "-o", "Dir::State::status=/dev/null",
                    "show", f"{name}={version}"])
    for stanza in metadata.split("\n\n"):
        fields = dict(line.split(": ", 1) for line in stanza.splitlines() if ": " in line)
        if (fields.get("Package") == name and fields.get("Version") == version
                and fields.get("Architecture") in ("amd64", "all")
                and fields.get("Filename", "").startswith("pool/")
                and re.fullmatch(r"[0-9a-f]{64}", fields.get("SHA256", ""))):
            return
    fail(f"apt selected {name}={version} outside signed snapshot metadata")


def ensure_packages(mode, job, acquire, run=command, get=fetch):
    wanted = package_set(mode)
    existing = {name: package_version(name, run) for name in wanted}
    if all(existing[name] == version for name, version in wanted.items()):
        if mode == "smoke":
            provider = run(["dpkg-query", "-W", "-f=${Provides}", "libncurses-dev"])
            if f"libncurses5-dev (= {PACKAGES['libncurses-dev']})" not in provider.split(", "):
                fail("selected libncurses-dev does not provide libncurses5-dev")
        return {"source": "installed", "versions": existing}
    if not acquire:
        fail("exact Ubuntu package set is absent; acquisition was not enabled")
    for suite, expected in RELEASES.items():
        path = job / (suite + ".InRelease")
        get(f"https://snapshot.ubuntu.com/ubuntu/{SNAPSHOT}/dists/{suite}/InRelease", path)
        verify_digest(path, "sha256", expected)
    sources = snapshot_sources(job)
    lists = job / "apt-lists"
    lists.mkdir()
    (lists / "partial").mkdir()
    apt = ["apt-get", "-o", f"Dir::Etc::sourcelist={sources}", "-o", "Dir::Etc::sourceparts=-",
           "-o", f"Dir::State::lists={lists}"]
    run(["sudo", *apt, "update"])
    pins = [f"{name}={version}" for name, version in sorted(wanted.items())]
    # Run without root, apt prints a simulation note first; selected_plan reads only Inst lines.
    plan = run([*apt, "-s", "install", "--no-install-recommends", *pins])
    selected_plan_versions = selected_plan(plan)
    for name, version in selected_plan_versions.items():
        if name in wanted and wanted[name] != version:
            fail(f"apt substituted pinned package {name}")
        snapshot_package(name, version, apt, run)
    (job / "apt-plan.txt").write_text(plan + "\n")
    run(["sudo", *apt, "install", "--yes", "--allow-downgrades", "--no-install-recommends", *pins])
    selected = {name: package_version(name, run) for name in wanted}
    if selected != wanted:
        fail("installed Ubuntu package versions differ from the snapshot pins")
    if mode == "smoke":
        provider = run(["dpkg-query", "-W", "-f=${Provides}", "libncurses-dev"])
        if f"libncurses5-dev (= {PACKAGES['libncurses-dev']})" not in provider.split(", "):
            fail("selected libncurses-dev does not provide libncurses5-dev")
    return {"source": SNAPSHOT, "versions": selected, "closure": selected_plan_versions,
            "plan": "apt-plan.txt"}


JDK_FLOOR = int(ET.parse(Path(__file__).resolve().parents[2] / "libraries/jvm/pom.xml").getroot().find("{*}properties/{*}thinkthen.session.jdk").text)


def selected_executable(path):
    resolved = path.expanduser().resolve()
    return resolved if resolved.is_file() and os.access(resolved, os.X_OK) else None


def sdk_home(variable, executable, runtime=None):
    chosen = os.environ.get(variable)
    if chosen:
        home = Path(chosen).expanduser().resolve()
    else:
        found = shutil.which(executable)
        if not found:
            return None
        home = Path(found).resolve().parent.parent
    binary = selected_executable(home / "bin" / executable)
    if binary is None or (runtime and not (home / runtime).is_file()):
        if chosen:
            fail(f"selected {variable} lacks {executable} or its runtime")
        return None
    return home


def managed_env(jdk):
    env = os.environ.copy()
    env.update(JAVA_HOME=str(jdk), JAVACMD=str(jdk / "bin/java"),
               PATH=str(jdk / "bin") + os.pathsep + env.get("PATH", ""))
    return env


def check_jdk(run=command):
    home = sdk_home("THINKTHEN_JDK_HOME", "javac")
    if home is None:
        fail("stable JDK is absent; select THINKTHEN_JDK_HOME")
    env = managed_env(home)
    for tool, flag in (("java", "-version"), ("javac", "-version"), ("jar", "--version")):
        if selected_executable(home / "bin" / tool) is None:
            fail(f"selected JDK lacks {tool}")
        shown = run([str(home / "bin" / tool), flag], env=env)
        version = re.search(r"(?<![0-9])([0-9]+)\.[0-9]", shown)
        if version is None or int(version[1]) < JDK_FLOOR or "-ea" in shown:
            fail(f"selected JDK {tool} differs from stable JDK {JDK_FLOOR} or later")
    return home


def safe_extract(archive, destination):
    destination.mkdir()
    if zipfile.is_zipfile(archive):
        with zipfile.ZipFile(archive) as data:
            members = data.infolist()
            for item in members:
                path = Path(item.filename)
                if path.is_absolute() or ".." in path.parts or ((item.external_attr >> 16) & 0o170000) == 0o120000:
                    fail("SDK zip contains an unsafe member")
            data.extractall(destination)
    else:
        with tarfile.open(archive, "r:gz") as data:
            members = data.getmembers()
            for item in members:
                path = Path(item.name)
                if path.is_absolute() or ".." in path.parts or not (item.isfile() or item.isdir()):
                    fail("SDK tar contains an unsafe member")
            data.extractall(destination)


def sdk_path(name, job, acquire, run=command, get=fetch, env=None):
    if name == "dotnet":
        explicit = os.environ.get("THINKTHEN_DOTNET")
        chosen = explicit or shutil.which("dotnet")
        binary = selected_executable(Path(chosen)) if chosen else None
        if binary and run([str(binary), "--version"]) == "8.0.131":
            return binary
        if explicit:
            fail("selected THINKTHEN_DOTNET differs from SDK 8.0.131")
    else:
        variable, executable, member, wanted, banner = {
            "kotlin": ("THINKTHEN_KOTLIN_HOME", "kotlinc", "lib/kotlin-stdlib.jar", "2.4.20",
                       r"kotlinc-jvm 2\.4\.20(?![.\w-])"),
            "scala": ("THINKTHEN_SCALA_HOME", "scalac", "lib/scala.jar", "3.9.0",
                      r"Scala compiler version 3\.9\.0(?![.\w-])"),
        }[name]
        home = sdk_home(variable, executable, member)
        if home and re.search(banner, run([str(home / "bin" / executable), "-version"], env=env)):
            return home
        if os.environ.get(variable):
            fail(f"selected {variable} differs from {wanted}")
    if not acquire:
        fail(f"exact {name} SDK is absent; acquisition was not enabled")
    url, kind, expected = ASSETS[name]
    archive = job / url.rsplit("/", 1)[1]
    get(url, archive)
    verify_digest(archive, kind, expected)
    output = job / name
    safe_extract(archive, output)
    candidate = output / ({"dotnet": "dotnet", "kotlin": "kotlinc", "scala": "scala3-3.9.0"}[name])
    if name == "dotnet":
        candidate = selected_executable(candidate)
        if candidate is None or run([str(candidate), "--version"]) != "8.0.131":
            fail("acquired .NET SDK differs from 8.0.131")
        return candidate
    candidate = output / ("kotlinc" if name == "kotlin" else "scala3-3.9.0")
    executable = "kotlinc" if name == "kotlin" else "scalac"
    member = "lib/kotlin-stdlib.jar" if name == "kotlin" else "lib/scala.jar"
    wanted = (r"kotlinc-jvm 2\.4\.20(?![.\w-])" if name == "kotlin"
              else r"Scala compiler version 3\.9\.0(?![.\w-])")
    if selected_executable(candidate / "bin" / executable) is None or not (candidate / member).is_file():
        fail(f"acquired {name} SDK layout differs from pin")
    if not re.search(wanted, run([str(candidate / "bin" / executable), "-version"], env=env)):
        fail(f"acquired {name} SDK version differs from pin")
    return candidate


RESTRICT_USERNS = Path("/proc/sys/kernel/apparmor_restrict_unprivileged_userns")


def bwrap_profile(bwrap):
    # Ubuntu 24.04 ships this form for each program that needs user namespaces.
    return ("abi <abi/4.0>,\ninclude <tunables/global>\n\n"
            f'profile bwrap "{bwrap}" flags=(unconfined) {{\n  userns,\n}}\n')


def probe_bwrap(job, run, bwrap, restrict=RESTRICT_USERNS):
    """Prove the pinned bwrap can make a namespace with no network; return how."""
    probe = [str(bwrap), "--unshare-user", "--unshare-pid", "--unshare-net", "--die-with-parent",
             "--ro-bind", "/", "/", "--", "/bin/true"]
    try:
        run(probe)
        return "not needed"
    except RuntimeError:
        # Rehearsal run 36853575250: ubuntu-24.04 restricts user namespaces to programs with
        # an AppArmor profile. The profile goes into this job's kernel only (ticket 0369).
        if not restrict.is_file() or restrict.read_text().strip() != "1":
            raise
    profile = job / "bwrap.apparmor"
    profile.write_text(bwrap_profile(bwrap))
    run(["sudo", "apparmor_parser", "--replace", str(profile)])
    run(probe)
    return "loaded"


def check_smoke_tools(job, run=command, bwrap=Path("/usr/bin/bwrap"), restrict=RESTRICT_USERNS):
    versions = {"gprbuild": "GPRBUILD Pro 18.0w", "gnatmake": "GNATMAKE 13.3.0",
                "gcc": "13.3.0", "cobc": "GnuCOBOL) 4.0-early-dev.0"}
    selected = {}
    for tool, marker in versions.items():
        found = shutil.which(tool)
        if not found:
            fail(f"installed language tool is missing: {tool}")
        path = Path(found).resolve()
        if marker not in run([str(path), "--version"]):
            fail(f"selected {tool} version differs from pin")
        owner = run(["dpkg-query", "-S", found]).split(":", 1)[0]
        if owner != OWNERS[tool]:
            fail(f"selected {tool} package owner differs from pin")
        selected[tool] = str(path)
    cc = shutil.which("cc")
    cc_path = selected_executable(Path(cc)) if cc else None
    if cc_path is None or str(cc_path) != selected["gcc"]:
        fail("selected cc is not the selected GCC 13")
    if "13.3.0" not in run([str(cc_path), "--version"]):
        fail("selected cc version differs from pin")
    cc_owner = run(["dpkg-query", "-S", str(cc_path)]).split(":", 1)[0]
    if cc_owner != "gcc-13-x86-64-linux-gnu":
        fail("selected cc package owner differs from pin")
    if selected_executable(bwrap) is None or "0.9.0" not in run([str(bwrap), "--version"]):
        fail("/usr/bin/bwrap differs from the pinned bubblewrap")
    if run(["dpkg-query", "-S", str(bwrap)]).split(":", 1)[0] != "bubblewrap":
        fail("/usr/bin/bwrap package owner differs from pin")
    selected["bwrap_apparmor"] = probe_bwrap(job, run, bwrap, restrict)
    selected["cc"] = str(cc_path)
    selected["bwrap"] = str(bwrap)
    return selected


def write_handoff(github_env, github_path, values, jdk):
    for target in (github_env, github_path):
        if not target.is_absolute() or not target.is_file() or target.is_symlink():
            fail("GitHub handoff path must be an existing absolute file")
    entries = {**values, "JAVA_HOME": str(jdk), "JAVACMD": str(jdk / "bin/java")}
    for value in entries.values():
        if "\n" in value or "\r" in value:
            fail("GitHub handoff value contains a newline")
    with github_env.open("a") as output:
        for key, value in entries.items():
            output.write(f"{key}={value}\n")
    with github_path.open("a") as output:
        output.write(str(jdk / "bin") + "\n")


def validate_handoff(github_env, github_path):
    for target in (github_env, github_path):
        if not target.is_absolute() or not target.is_file() or target.is_symlink():
            fail("GitHub handoff path must be an existing absolute file")


def setup(mode, job, expected_sha, acquire, github_env, github_path, run=command, get=fetch,
          bwrap=Path("/usr/bin/bwrap"), restrict=RESTRICT_USERNS):
    if mode not in ("managed", "smoke") or platform.system() != "Linux" or platform.machine() != "x86_64":
        fail("language tools support only Linux x86-64 managed or smoke")
    if not re.fullmatch(r"[0-9a-f]{40}", expected_sha) or run(["git", "rev-parse", "HEAD"]) != expected_sha:
        fail("checkout differs from selected release SHA")
    validate_handoff(github_env, github_path)
    if not job.is_absolute() or job.exists() or job.is_symlink():
        fail("job-owned tool directory must be an absent absolute path")
    job.mkdir(mode=0o700)
    packages = ensure_packages(mode, job, acquire, run, get)
    jdk = check_jdk(run)
    env = managed_env(jdk)
    dotnet = sdk_path("dotnet", job, acquire, run, get, env)
    kotlin = sdk_path("kotlin", job, acquire, run, get, env)
    scala = sdk_path("scala", job, acquire, run, get, env)
    selected = check_smoke_tools(job, run, bwrap, restrict) if mode == "smoke" else {}
    values = {"THINKTHEN_DOTNET": str(dotnet), "THINKTHEN_JDK_HOME": str(jdk),
              "THINKTHEN_KOTLIN_HOME": str(kotlin), "THINKTHEN_SCALA_HOME": str(scala)}
    record = {"mode": mode, "source_commit": expected_sha, "packages": packages,
              "assets": {name: {"url": url, "algorithm": kind, "digest": value}
                         for name, (url, kind, value) in ASSETS.items()},
              "selected": {**values, "java": str(jdk / "bin/java"), **selected}}
    (job / "selection.json").write_text(json.dumps(record, indent=2, sort_keys=True) + "\n")
    write_handoff(github_env, github_path, values, jdk)
    return record


def main(argv):
    if len(argv) not in (3, 4) or (len(argv) == 4 and argv[3] != "--acquire"):
        fail("usage: release-language-tools.py managed|smoke JOB_DIR EXPECTED_SHA [--acquire]")
    github_env = Path(os.environ.get("GITHUB_ENV", ""))
    github_path = Path(os.environ.get("GITHUB_PATH", ""))
    result = setup(argv[0], Path(argv[1]), argv[2], len(argv) == 4, github_env, github_path)
    print(f"language tools: {result['mode']} selected from {result['packages']['source']}")


if __name__ == "__main__":
    try:
        main(sys.argv[1:])
    except (RuntimeError, OSError, ValueError, tarfile.TarError, zipfile.BadZipFile) as error:
        print(f"release-language-tools: {error}", file=sys.stderr)
        sys.exit(1)
