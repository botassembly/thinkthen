#!/usr/bin/env python3
"""Planted-archive proof of the registry operations (ticket 0355). No network.

Each case copies the operations into a small Git tree, plants one fault, and
checks the exit code and the refusal sentence. Signing cases need gpg.
"""

import base64
import contextlib
import hashlib
import json
import io
import os
import pathlib
import shutil
import subprocess
import sys
import tarfile
import tempfile
import zipfile

REPO = pathlib.Path(__file__).resolve().parents[2]
TARGET = "x86_64-unknown-linux-gnu"
SECRETS = ("MAVEN_CENTRAL_GPG_PRIVATE_KEY", "MAVEN_CENTRAL_GPG_PASSPHRASE", "MAVEN_CENTRAL_USERNAME",
           "MAVEN_CENTRAL_PASSWORD", "NUGET_API_KEY")
TREE = ("crates/thinkthen/Cargo.toml", "composer.json", "libraries/php/composer.json", "libraries/go/go.mod",
        "libraries/jvm/pom.xml", "libraries/jvm/README.md", "libraries/jvm/door/thinkthen/Door.java",
        "libraries/jvm/door/thinkthen/Json.java", "libraries/jvm/kotlin/KotlinCaller.kt",
        "libraries/jvm/scala/ScalaCaller.scala", "libraries/dart/pubspec.yaml", "libraries/dart/README.md",
        "libraries/dart/lib/thinkthen_dart.dart")


def version():
    return next(line.split('"')[1] for line in (REPO / "crates/thinkthen/Cargo.toml").read_text().splitlines()
                if line.startswith('version = "'))


def archive(folder, family, files):
    name = f"thinkthen-{family}-{version()}-{TARGET}.tar.gz"
    buffer = io.BytesIO()
    with tarfile.open(fileobj=buffer, mode="w:gz") as tar:
        for member, data in {"THINKTHEN-PACKAGE-INPUTS": b"source_commit=0\n", **files}.items():
            info = tarfile.TarInfo(f"./{member}")
            info.size = len(data)
            tar.addfile(info, io.BytesIO(data))
    (folder / name).write_bytes(buffer.getvalue())
    (folder / f"{name}.sha256").write_text(f"{hashlib.sha256(buffer.getvalue()).hexdigest()}  {name}\n")


def nupkg(identity, v):
    buffer = io.BytesIO()
    with zipfile.ZipFile(buffer, "w") as package:
        package.writestr("Botassembly.ThinkThen.nuspec", f'<?xml version="1.0"?><package xmlns="http://schemas.microsoft.com/'
                         f'packaging/2013/05/nuspec.xsd"><metadata><id>{identity}</id><version>{v}</version>'
                         f'</metadata></package>')
        package.writestr("lib/net8.0/ThinkThen.dll", b"dll")
    return buffer.getvalue()


def jar(name):
    buffer = io.BytesIO()
    with zipfile.ZipFile(buffer, "w") as package:
        package.writestr("META-INF/MANIFEST.MF", "Manifest-Version: 1.0\n")
        package.writestr(f"{name}.class", b"class")
    return buffer.getvalue()


def tree(root, plant):
    for path in TREE:
        (root / path).parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(REPO / path, root / path)
    (root / "sdlc/scripts").mkdir(parents=True)
    shutil.copyfile(REPO / "sdlc/scripts/release-registry.py", root / "sdlc/scripts/release-registry.py")
    v = version()
    platform = root / "platform"
    platform.mkdir()
    files = {
        "csharp": {f"Botassembly.ThinkThen.{v}.nupkg": nupkg("Botassembly.ThinkThen", v)},
        "jvm": {"pom.xml": (root / "libraries/jvm/pom.xml").read_bytes(),
                "README.md": (root / "libraries/jvm/README.md").read_bytes(),
                **{f"thinkthen-{kind}.jar": jar(kind) for kind in ("door", "kotlin", "scala")}},
        "dart": {"pubspec.yaml": (root / "libraries/dart/pubspec.yaml").read_bytes(), "pubspec.lock": b"lock",
                 "README.md": b"readme", "lib/thinkthen_dart.dart": b"library;"},
    }
    plant(root, files)
    for family, members in files.items():
        archive(platform, family, members)
    if plant is sidecar:
        name = f"thinkthen-jvm-{v}-{TARGET}.tar.gz.sha256"
        (platform / name).write_text("0" * 64 + (platform / name).read_text()[64:])
    git = ["git", "-c", "user.name=t", "-c", "user.email=t@invalid", "-c", "commit.gpgsign=false"]
    subprocess.run(["git", "init", "-q", str(root)], check=True)
    subprocess.run([*git, "-C", str(root), "add", "-A", "--", *TREE, "sdlc"], check=True)
    subprocess.run([*git, "-C", str(root), "commit", "-qm", "fixture"], check=True)
    return subprocess.check_output(["git", "-C", str(root), "rev-parse", "HEAD"], text=True).strip()


def keep(root, files):
    pass


def sidecar(root, files):
    pass


def text(path, old, new):
    path.write_text(path.read_text().replace(old, new))


def run(root, *args, env=None):
    clean = {k: v for k, v in os.environ.items() if k not in SECRETS}
    return subprocess.run([sys.executable, str(root / "sdlc/scripts/release-registry.py"), *args],
                          cwd=root, env=clean | (env or {}), capture_output=True, text=True)


GH = """#!/bin/sh
echo "gh $*" >> "$FAKE_LOG"
case "$*" in
  *'--include'*) printf 'HTTP/2.0 %s\\n' "$FAKE_GO_STATUS" ;;
  *'--jq .object.sha'*) echo "$FAKE_GO_SHA" ;;
esac
"""


def go_tag_cases():
    """The publish step tags the Go module once, at the resolved commit, before the release goes public."""
    head = subprocess.check_output(["git", "-C", str(REPO), "rev-parse", "HEAD"], text=True).strip()
    tag = f"libraries/go/v{version()}"
    post = f"gh api --method POST repos/o/r/git/refs -f ref=refs/tags/{tag} -f sha={head}"
    read = f"gh api --include repos/o/r/git/ref/tags/{tag}"
    edit = "gh release edit v0 --draft=false"
    cases = [("go-tag-new", "404", head, 0, [read, post, edit]),
             ("go-tag-same", "200", head, 0, [read, f"gh api repos/o/r/git/ref/tags/{tag} --jq .object.sha", edit]),
             ("go-tag-other", "200", "0" * 40, 1, [read, f"gh api repos/o/r/git/ref/tags/{tag} --jq .object.sha"]),
             ("go-tag-unreadable", "500", head, 1, [read])]
    out = []
    for name, status, sha, code, calls in cases:
        with tempfile.TemporaryDirectory(prefix="thinkthen-go-tag-") as tmp:
            bin_dir = pathlib.Path(tmp)
            (bin_dir / "gh").write_text(GH)
            (bin_dir / "curl").write_text("#!/bin/sh\nexit 22\n")
            for tool in ("gh", "curl"):
                (bin_dir / tool).chmod(0o755)
            env = {k: v for k, v in os.environ.items() if k not in SECRETS}
            env.update(PATH=f"{bin_dir}:{env['PATH']}", FAKE_LOG=str(bin_dir / "log"), FAKE_GO_STATUS=status,
                       FAKE_GO_SHA=sha, GITHUB_REPOSITORY="o/r")
            result = subprocess.run(["sh", str(REPO / "sdlc/scripts/release-workflow"), "publish", "v0", head],
                                    env=env, capture_output=True, text=True)
            log = (bin_dir / "log").read_text().splitlines() if (bin_dir / "log").exists() else []
            out.append((name, (result.returncode, log), (code, calls)))
    return out


class Reply(io.BytesIO):
    status = 201

    def __enter__(self):
        return self

    def __exit__(self, *_):
        return False


def upload_cases(bundle):
    """Drive maven_upload against stubbed replies: version check, upload, then status polls."""
    import importlib.util
    import urllib.error
    spec = importlib.util.spec_from_file_location("registry", REPO / "sdlc/scripts/release-registry.py")
    registry = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(registry)
    registry.time.sleep = lambda _: None
    out = []
    for name, states, wanted in (
            ("upload-published", ["PENDING", "boom", "VALIDATED", "PUBLISHED"], None),
            ("upload-failed", ["VALIDATING", "FAILED"], "failed validation; see the Central Portal"),
            ("upload-unknown", [None], "has unknown state None"),
            ("upload-flaky", ["boom"] * 5, "status failed five times")):
        calls = []

        def urlopen(request, timeout, states=list(states)):
            url = request.full_url
            calls.append(url.split("?")[0].rsplit("/", 1)[-1])
            if url.endswith(".pom"):
                raise urllib.error.HTTPError(url, 404, "missing", {}, None)
            assert request.get_header("Authorization") == "Bearer " + base64.b64encode(b"u:p").decode()
            if "/upload?" in url:
                return Reply(b"0123abcd-0123-0123-0123-0123456789ab")
            state = states.pop(0)
            if state == "boom":
                raise urllib.error.HTTPError(url, 502, "bad gateway", {}, io.BytesIO(b"gateway"))
            return Reply(json.dumps({"deploymentState": state}).encode())
        registry.urllib.request.urlopen = urlopen
        os.environ.update(MAVEN_CENTRAL_USERNAME="u", MAVEN_CENTRAL_PASSWORD="p")
        try:
            with contextlib.redirect_stdout(io.StringIO()):
                registry.maven_upload(bundle)
            got = None
        except registry.Refusal as refusal:
            got = next((w for w in [wanted] if w and w in str(refusal)), str(refusal))
        finally:
            for key in ("MAVEN_CENTRAL_USERNAME", "MAVEN_CENTRAL_PASSWORD"):
                os.environ.pop(key, None)
        out.append((name, got, wanted))
    return out


def nuget_cases(nupkg):
    """Drive nuget_push against stubbed replies: the headers it sends and what a refusal prints."""
    import importlib.util
    import urllib.error
    spec = importlib.util.spec_from_file_location("registry", REPO / "sdlc/scripts/release-registry.py")
    registry = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(registry)
    key = "planted-nuget-key"
    out = []
    for name, code, reason, body, wanted in (
            ("nuget-pushed", 201, None, b"", (f"release-registry: pushed {nupkg.name}", "4.1.0", key)),
            ("nuget-400", 400, "A client version '4.1.0' or higher is required to be able to push packages",
             b"echo " + key.encode(), "NuGet answered 400 A client version '4.1.0' or higher is required to be able to push packages: echo [key]"),
            ("nuget-400-long", 400, "Bad Request", b"x" * 490 + key.encode(),
             "NuGet answered 400 Bad Request: " + "x" * 490 + "[key]"),
            ("nuget-409", 409, "Conflict", b"The package ID is reserved.",
             "NuGet answered 409 Conflict: The package ID is reserved.")):
        sent = []

        def urlopen(request, timeout, code=code, reason=reason, body=body):
            sent.append((request.get_header("X-nuget-protocol-version"), request.get_header("X-nuget-apikey")))
            if code >= 400:
                raise urllib.error.HTTPError(request.full_url, code, reason, {}, io.BytesIO(body))
            reply = Reply(b"")
            reply.status = code
            return reply
        registry.urllib.request.urlopen = urlopen
        os.environ["NUGET_API_KEY"] = key
        printed = io.StringIO()
        try:
            with contextlib.redirect_stdout(printed):
                registry.nuget_push(nupkg)
            got = (printed.getvalue().strip(), *sent[0])
        except registry.Refusal as refusal:
            got = str(refusal)
        finally:
            os.environ.pop("NUGET_API_KEY", None)
        out.append((name, got, wanted))
    return out


def main():
    v = version()
    bad = 0
    stem = f"thinkthen-jvm-{v}"
    pack_cases = [
        ("pack", keep, None),
        ("nupkg-id", lambda r, f: f["csharp"].update({f"Botassembly.ThinkThen.{v}.nupkg": nupkg("Other.ThinkThen", v)}),
         "nupkg identity differs"),
        ("nupkg-version", lambda r, f: f["csharp"].update({f"Botassembly.ThinkThen.{v}.nupkg": nupkg("Botassembly.ThinkThen", "9.9.9")}),
         "nupkg identity differs"),
        ("pom-group", lambda r, f: f["jvm"].update({"pom.xml": f["jvm"]["pom.xml"].replace(b"io.github.botassembly", b"io.github.other")}),
         "POM identity differs"),
        ("pom-scm", lambda r, f: f["jvm"].update({"pom.xml": f["jvm"]["pom.xml"].replace(b"<scm>", b"<x>").replace(b"</scm>", b"</x>")}),
         "POM lacks scm/url, which Maven Central requires"),
        ("pom-packaging", lambda r, f: f["jvm"].update({"pom.xml": f["jvm"]["pom.xml"].replace(b"<packaging>jar", b"<packaging>pom")}),
         "POM packaging must be jar"),
        ("checksum", sidecar, f"thinkthen-jvm-{v}-{TARGET}.tar.gz differs from its checksum"),
        ("pub-path", lambda r, f: f["dart"].update({"pubspec.yaml": f["dart"]["pubspec.yaml"] + b"  other:\n    path: ..\n"}),
         "pub package has a path dependency"),
        ("composer-drift", lambda r, f: text(r / "composer.json", '"php": ">=8.3"', '"php": ">=8.2"'),
         "root composer.json require differs from libraries/php"),
        ("composer-version", lambda r, f: text(r / "composer.json", '"type"', '"version": "0.0.1",\n  "type"'),
         "root composer.json must not set version; Packagist reads the tag"),
        ("go-path", lambda r, f: text(r / "libraries/go/go.mod", "/libraries/go", ""),
         "Go module path differs from its folder"),
    ]
    with tempfile.TemporaryDirectory(prefix="thinkthen-registry-") as tmp:
        for name, plant, refusal in pack_cases:
            root = pathlib.Path(tmp) / name
            sha = tree(root, plant)
            result = run(root, "pack", "platform", "out", sha)
            got = result.returncode, result.stderr.strip()
            wanted = (0, "") if refusal is None else (1, f"release-registry: {refusal}")
            if got != wanted:
                bad += 1
                print(f"registry self-test {name}: wanted {wanted}, got {got}", file=sys.stderr)
        root = pathlib.Path(tmp) / "pack"
        base = root / "out/maven/io/github/botassembly/thinkthen-jvm" / v
        layout = sorted(p.name for p in base.iterdir())
        artifacts = [f"{stem}{suffix}" for suffix in ("-javadoc.jar", "-kotlin.jar", "-scala.jar", "-sources.jar", ".jar", ".pom")]
        pub = sorted(p.relative_to(root / "out/pub").as_posix() for p in (root / "out/pub").rglob("*") if p.is_file())
        expected_pub = ["thinkthen_dart/README.md", "thinkthen_dart/lib/thinkthen_dart.dart", "thinkthen_dart/pubspec.yaml"]
        with zipfile.ZipFile(base / f"{stem}-sources.jar") as sources:
            source_names = sorted(sources.namelist())
        checks = [
            ("maven-layout", layout, sorted(f"{a}{s}" for a in artifacts for s in ("", ".md5", ".sha1"))),
            ("maven-sources", source_names, ["door/thinkthen/Door.java", "door/thinkthen/Json.java",
                                             "kotlin/KotlinCaller.kt", "scala/ScalaCaller.scala"]),
            ("nuget-file", [p.name for p in (root / "out/nuget").iterdir()], [f"Botassembly.ThinkThen.{v}.nupkg"]),
            ("pub-files", pub, expected_pub),
        ]
        again = run(root, "pack", "platform", "out", subprocess.check_output(
            ["git", "-C", str(root), "rev-parse", "HEAD"], text=True).strip())
        checks.append(("pack-twice", (again.returncode, again.stderr.strip()),
                       (1, "release-registry: registry output must start empty")))
        other = run(root, "pack", "platform", "out2", "0" * 40)
        checks.append(("pack-sha", (other.returncode, other.stderr.strip()),
                       (1, "release-registry: checkout differs from the resolved SHA")))
        refusals = [
            ("rehearse-with-key", ("maven-sign", "out/maven", "rehearse"), {"MAVEN_CENTRAL_GPG_PRIVATE_KEY": "x"},
             "rehearsal signing refuses a Maven secret in its environment: MAVEN_CENTRAL_GPG_PRIVATE_KEY"),
            ("release-without-key", ("maven-sign", "out/maven", "release", "bundle.zip"), {},
             "release signing needs MAVEN_CENTRAL_GPG_PRIVATE_KEY and MAVEN_CENTRAL_GPG_PASSPHRASE"),
            ("upload-without-token", ("maven-upload", "bundle.zip"), {"MAVEN_CENTRAL_USERNAME": "u"},
             "MAVEN_CENTRAL_PASSWORD is missing"),
            ("nuget-without-key", ("nuget-push", f"out/nuget/Botassembly.ThinkThen.{v}.nupkg"), {},
             "NUGET_API_KEY is missing"),
        ]
        for name, args, env, refusal in refusals:
            result = run(root, *args, env=env)
            checks.append((name, (result.returncode, result.stderr.strip()), (1, f"release-registry: {refusal}")))
        if shutil.which("gpg"):
            signed = run(root, "maven-sign", "out/maven", "rehearse")
            checks.append(("rehearse-sign", (signed.returncode, signed.stdout.strip(), sorted(p.name for p in base.iterdir())),
                           (0, "release-registry: rehearsal signed and verified 6 Maven files with a throwaway key", layout)))
            (base / f"{stem}.jar.sha1").write_text("0" * 40)
            stale = run(root, "maven-sign", "out/maven", "rehearse")
            checks.append(("rehearse-stale-checksum", (stale.returncode, stale.stderr.strip()),
                           (1, f"release-registry: {stem}.jar.sha1 differs")))
        else:
            checks.append(("rehearse-sign", "gpg missing", "gpg present"))
        checks += go_tag_cases()
        (root / "bundle.zip").write_bytes(b"zip")
        checks += upload_cases(root / "bundle.zip")
        checks += nuget_cases(root / f"out/nuget/Botassembly.ThinkThen.{v}.nupkg")
        for name, got, wanted in checks:
            if got != wanted:
                bad += 1
                print(f"registry self-test {name}: wanted {wanted}, got {got}", file=sys.stderr)
    total = len(pack_cases) + len(checks)
    print(f"registry self-test: {total - bad}/{total} cases hold")
    return 1 if bad else 0


if __name__ == "__main__":
    sys.exit(main())
