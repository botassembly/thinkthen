#!/usr/bin/env python3
"""Pack, sign, check and upload the registry packages (ticket 0355).

`release-workflow` calls each operation. `pack`, `maven-sign rehearse` and
`dry-run` read no secret and change no remote service, so a rehearsal runs
them. `maven-sign release`, `maven-upload`, `nuget-push` and `pub-publish`
run only in the release jobs, behind environment `release`.

Usage:
  release-registry.py pack PLATFORM_DIR OUT SHA
  release-registry.py maven-sign MAVEN_DIR rehearse
  release-registry.py maven-sign MAVEN_DIR release BUNDLE_ZIP
  release-registry.py maven-upload BUNDLE_ZIP
  release-registry.py nuget-push NUPKG
  release-registry.py dry-run PUB_DIR
  release-registry.py pub-publish PUB_DIR
"""

import base64
import hashlib
import io
import json
import os
import pathlib
import re
import shutil
import subprocess
import sys
import tarfile
import tempfile
import time
import urllib.error
import urllib.request
import uuid
import xml.etree.ElementTree as ET
import zipfile

REPO = pathlib.Path(__file__).resolve().parents[2]
TARGET = "x86_64-unknown-linux-gnu"
GROUP, ARTIFACT = "io.github.botassembly", "thinkthen-jvm"
NUGET_ID, PUB_NAME = "Botassembly.ThinkThen", "thinkthen_dart"
GO_MODULE = "github.com/botassembly/thinkthen/libraries/go"
MAVEN_SECRETS = ("MAVEN_CENTRAL_GPG_PRIVATE_KEY", "MAVEN_CENTRAL_GPG_PASSPHRASE",
                 "MAVEN_CENTRAL_USERNAME", "MAVEN_CENTRAL_PASSWORD")
ZIP_TIME = (1980, 1, 1, 0, 0, 0)


class Refusal(Exception):
    pass


def require(condition, message):
    if not condition:
        raise Refusal(message)


def version():
    for line in (REPO / "crates/thinkthen/Cargo.toml").read_text(encoding="utf-8").splitlines():
        if line.startswith('version = "'):
            return line.split('"')[1]
    raise Refusal("crates/thinkthen/Cargo.toml names no version")


def archive_files(platform, family, v):
    """The members of one checksummed family archive, by file name."""
    name = f"thinkthen-{family}-{v}-{TARGET}.tar.gz"
    path = platform / name
    require(path.is_file() and not path.is_symlink(), f"{name} is missing or linked")
    data = path.read_bytes()
    sidecar = platform / f"{name}.sha256"
    require(sidecar.is_file() and sidecar.read_text() == f"{hashlib.sha256(data).hexdigest()}  {name}\n",
            f"{name} differs from its checksum")
    files = {}
    with tarfile.open(fileobj=io.BytesIO(data), mode="r:gz") as archive:
        for member in archive.getmembers():
            if member.isdir():
                continue
            require(member.isfile(), f"{name} holds a link or device: {member.name}")
            relative = member.name.removeprefix("./")
            require(relative and not relative.startswith("/") and ".." not in relative.split("/"),
                    f"{name} holds an unsafe path: {member.name}")
            files[relative] = archive.extractfile(member).read()
    return files


def field(root, path):
    namespace = {"m": "http://maven.apache.org/POM/4.0.0"}
    node = root.find("/".join(f"m:{part}" for part in path.split("/")), namespace)
    return (node.text or "").strip() if node is not None else ""


def check_pom(data, v):
    root = ET.fromstring(data)
    require((field(root, "groupId"), field(root, "artifactId"), field(root, "version")) == (GROUP, ARTIFACT, v),
            "POM identity differs")
    require(field(root, "packaging") == "jar", "POM packaging must be jar")
    for path in ("name", "description", "url", "licenses/license/name", "licenses/license/url",
                 "developers/developer/name", "scm/url", "scm/connection"):
        require(field(root, path), f"POM lacks {path}, which Maven Central requires")


def check_nupkg(data, v):
    with zipfile.ZipFile(io.BytesIO(data)) as package:
        names = package.namelist()
        require(f"{NUGET_ID}.nuspec" in names, "nupkg lacks its nuspec")
        root = ET.fromstring(package.read(f"{NUGET_ID}.nuspec"))
    values = {node.tag.rsplit("}", 1)[-1]: (node.text or "").strip() for node in root.iter()}
    require(values.get("id") == NUGET_ID and values.get("version") == v, "nupkg identity differs")


def check_pubspec(text, v):
    require(re.search(rf"(?m)^name: {PUB_NAME}$", text), "pub package name differs")
    require(re.search(rf"(?m)^version: {re.escape(v)}$", text), "pub package version differs")
    require(not re.search(r"(?m)^publish_to:", text), "pub package sets publish_to")
    require(not re.search(r"(?m)^\s+path:", text), "pub package has a path dependency")


def check_composer(root=REPO):
    top = json.loads((root / "composer.json").read_text(encoding="utf-8"))
    php = json.loads((root / "libraries/php/composer.json").read_text(encoding="utf-8"))
    for key in ("name", "type", "license", "require"):
        require(top.get(key) == php.get(key), f"root composer.json {key} differs from libraries/php")
    require(top.get("description"), "root composer.json lacks a description")
    require(top.get("autoload") == {"files": ["libraries/php/autoload.php"]},
            "root composer.json must autoload libraries/php/autoload.php")
    require("version" not in top, "root composer.json must not set version; Packagist reads the tag")


def check_go(root=REPO, v=None):
    first = (root / "libraries/go/go.mod").read_text(encoding="utf-8").splitlines()[0]
    require(first == f"module {GO_MODULE}", "Go module path differs from its folder")
    require(int((v or version()).split(".")[0]) < 2, "a Go major version of 2 or more needs a /vN module path")


def deterministic_zip(path, members):
    with zipfile.ZipFile(path, "w", zipfile.ZIP_DEFLATED) as bundle:
        for name, data in members:
            info = zipfile.ZipInfo(name, ZIP_TIME)
            info.compress_type = zipfile.ZIP_DEFLATED
            info.external_attr = 0o644 << 16
            bundle.writestr(info, data)


def pack(platform, out, sha):
    v = version()
    head = subprocess.run(["git", "rev-parse", "HEAD"], cwd=REPO, capture_output=True, text=True).stdout.strip()
    require(head == sha, "checkout differs from the resolved SHA")
    require(not out.exists() or (out.is_dir() and not any(out.iterdir())), "registry output must start empty")
    check_composer()
    check_go()

    csharp = archive_files(platform, "csharp", v)
    nupkg_name = f"{NUGET_ID}.{v}.nupkg"
    require(nupkg_name in csharp, f"C# archive lacks {nupkg_name}")
    check_nupkg(csharp[nupkg_name], v)
    (out / "nuget").mkdir(parents=True)
    (out / "nuget" / nupkg_name).write_bytes(csharp[nupkg_name])

    jvm = archive_files(platform, "jvm", v)
    for name in ("pom.xml", "README.md", "thinkthen-door.jar", "thinkthen-kotlin.jar", "thinkthen-scala.jar"):
        require(name in jvm, f"JVM archive lacks {name}")
    check_pom(jvm["pom.xml"], v)
    base = out / "maven" / GROUP.replace(".", "/") / ARTIFACT / v
    base.mkdir(parents=True)
    stem = f"{ARTIFACT}-{v}"
    (base / f"{stem}.pom").write_bytes(jvm["pom.xml"])
    (base / f"{stem}.jar").write_bytes(jvm["thinkthen-door.jar"])
    (base / f"{stem}-kotlin.jar").write_bytes(jvm["thinkthen-kotlin.jar"])
    (base / f"{stem}-scala.jar").write_bytes(jvm["thinkthen-scala.jar"])
    sources = sorted(p for folder in ("door", "kotlin", "scala")
                     for p in (REPO / "libraries/jvm" / folder).rglob("*")
                     if p.suffix in (".java", ".kt", ".scala") and p.is_file())
    require(sources, "no JVM sources to pack")
    deterministic_zip(base / f"{stem}-sources.jar",
                      [(p.relative_to(REPO / "libraries/jvm").as_posix(), p.read_bytes()) for p in sources])
    # Central accepts a javadoc JAR that holds only a README when no pages can be generated.
    deterministic_zip(base / f"{stem}-javadoc.jar", [("README.md", jvm["README.md"])])
    for path in sorted(base.iterdir()):
        data = path.read_bytes()
        path.with_name(path.name + ".md5").write_text(hashlib.md5(data).hexdigest())
        path.with_name(path.name + ".sha1").write_text(hashlib.sha1(data).hexdigest())

    dart = archive_files(platform, "dart", v)
    require("pubspec.yaml" in dart, "Dart archive lacks pubspec.yaml")
    check_pubspec(dart["pubspec.yaml"].decode("utf-8"), v)
    pub = out / "pub" / PUB_NAME
    for name, data in dart.items():
        if name in ("THINKTHEN-PACKAGE-INPUTS", "pubspec.lock"):
            continue
        (pub / name).parent.mkdir(parents=True, exist_ok=True)
        (pub / name).write_bytes(data)
    print(f"release-registry: packed {nupkg_name}, {stem} and {PUB_NAME} {v}")


def maven_artifacts(maven):
    v = version()
    base = maven / GROUP.replace(".", "/") / ARTIFACT / v
    stem = f"{ARTIFACT}-{v}"
    names = [f"{stem}.pom", f"{stem}.jar", *(f"{stem}-{kind}.jar" for kind in ("kotlin", "scala", "sources", "javadoc"))]
    for name in names:
        data = (base / name).read_bytes() if (base / name).is_file() else None
        require(data is not None, f"Maven bundle lacks {name}")
        require((base / f"{name}.md5").read_text() == hashlib.md5(data).hexdigest(), f"{name}.md5 differs")
        require((base / f"{name}.sha1").read_text() == hashlib.sha1(data).hexdigest(), f"{name}.sha1 differs")
    expected = {f"{name}{suffix}" for name in names for suffix in ("", ".md5", ".sha1")}
    require({p.name for p in base.iterdir()} == expected, "Maven bundle holds unexpected files")
    return base, names


def gpg(home, *args, data=None):
    result = subprocess.run(["gpg", "--homedir", str(home), "--batch", "--no-tty", *args],
                            input=data, capture_output=True)
    require(result.returncode == 0, f"gpg {args[0]} failed: {result.stderr.decode(errors='replace')[-300:]}")
    return result.stdout.decode()


def maven_sign(maven, mode, bundle=None):
    if mode == "rehearse":
        present = [name for name in MAVEN_SECRETS if name in os.environ]
        require(not present, "rehearsal signing refuses a Maven secret in its environment: " + ", ".join(present))
    else:
        require(os.environ.get("MAVEN_CENTRAL_GPG_PRIVATE_KEY") and "MAVEN_CENTRAL_GPG_PASSPHRASE" in os.environ,
                "release signing needs MAVEN_CENTRAL_GPG_PRIVATE_KEY and MAVEN_CENTRAL_GPG_PASSPHRASE")
        require(bundle is not None and not bundle.exists(), "release signing needs a new bundle path")
    maven_artifacts(maven)
    with tempfile.TemporaryDirectory(prefix="thinkthen-maven-") as tmp:
        home = pathlib.Path(tmp) / "gnupg"
        home.mkdir(mode=0o700)
        copy = pathlib.Path(tmp) / "maven"
        shutil.copytree(maven, copy)
        if mode == "rehearse":
            passphrase = ""
            gpg(home, "--pinentry-mode", "loopback", "--passphrase", "", "--quick-gen-key",
                "ThinkThen rehearsal <rehearsal@invalid>", "ed25519", "sign", "1d")
        else:
            passphrase = os.environ["MAVEN_CENTRAL_GPG_PASSPHRASE"]
            gpg(home, "--import", data=os.environ["MAVEN_CENTRAL_GPG_PRIVATE_KEY"].encode())
        keys = [line.split(":")[4] for line in gpg(home, "--with-colons", "--list-secret-keys").splitlines()
                if line.startswith("sec:")]
        require(len(keys) == 1, f"signing needs exactly one secret key, found {len(keys)}")
        base, names = maven_artifacts(copy)
        for name in names:
            gpg(home, "--pinentry-mode", "loopback", "--passphrase-fd", "0", "--local-user", keys[0],
                "--armor", "--detach-sign", "--output", str(base / f"{name}.asc"), str(base / name),
                data=passphrase.encode())
            gpg(home, "--verify", str(base / f"{name}.asc"), str(base / name))
        members = sorted((p.relative_to(copy).as_posix(), p.read_bytes()) for p in copy.rglob("*") if p.is_file())
        require(len(members) == 4 * len(names), "signed Maven bundle has the wrong file count")
        if mode == "rehearse":
            deterministic_zip(pathlib.Path(tmp) / "bundle.zip", members)
            print(f"release-registry: rehearsal signed and verified {len(names)} Maven files with a throwaway key")
        else:
            deterministic_zip(bundle, members)
            print(f"release-registry: signed and verified {len(names)} Maven files into {bundle.name}")
        # Stop the agent this run started for its own key folder.
        subprocess.run(["gpgconf", "--homedir", str(home), "--kill", "gpg-agent"], capture_output=True)


def secret(name):
    value = os.environ.get(name, "")
    require(value, f"{name} is missing")
    return value


def status_of(url):
    try:
        with urllib.request.urlopen(urllib.request.Request(url), timeout=60) as reply:
            return reply.status
    except urllib.error.HTTPError as error:
        return error.code


def multipart(field_name, filename, data):
    boundary = uuid.uuid4().hex
    body = (f"--{boundary}\r\nContent-Disposition: form-data; name=\"{field_name}\"; filename=\"{filename}\"\r\n"
            f"Content-Type: application/octet-stream\r\n\r\n").encode() + data + f"\r\n--{boundary}--\r\n".encode()
    return body, f"multipart/form-data; boundary={boundary}"


def central(request, timeout):
    """One Central Portal call; an HTTP error names its code and reply body, which never holds the token."""
    try:
        with urllib.request.urlopen(request, timeout=timeout) as reply:
            return reply.read().decode()
    except urllib.error.HTTPError as error:
        raise Refusal(f"Central answered {error.code}: {error.read().decode(errors='replace')[:500]}")


def maven_upload(bundle):
    token = base64.b64encode(f"{secret('MAVEN_CENTRAL_USERNAME')}:{secret('MAVEN_CENTRAL_PASSWORD')}".encode()).decode()
    require(bundle.is_file(), "the signed Maven bundle is missing")
    v = version()
    pom = f"https://repo1.maven.org/maven2/{GROUP.replace('.', '/')}/{ARTIFACT}/{v}/{ARTIFACT}-{v}.pom"
    found = status_of(pom)
    require(found == 404, f"Maven Central already holds {ARTIFACT} {v}" if found == 200
            else f"Maven Central answered {found} for the version check")
    body, kind = multipart("bundle", bundle.name, bundle.read_bytes())
    portal = "https://central.sonatype.com/api/v1/publisher"
    request = urllib.request.Request(f"{portal}/upload?name={ARTIFACT}-{v}&publishingType=AUTOMATIC", data=body,
                                     method="POST", headers={"Authorization": f"Bearer {token}", "Content-Type": kind})
    deployment = central(request, 300).strip()
    require(re.fullmatch(r"[0-9a-fA-F-]{36}", deployment), "Central returned no deployment id")
    print(f"::notice::Maven Central deployment {deployment}")
    misses = 0
    for _ in range(180):
        time.sleep(10)
        request = urllib.request.Request(f"{portal}/status?id={deployment}", data=b"", method="POST",
                                         headers={"Authorization": f"Bearer {token}"})
        try:
            state = json.loads(central(request, 60)).get("deploymentState")
        except (Refusal, urllib.error.URLError, json.JSONDecodeError) as error:
            misses += 1
            require(misses < 5, f"Maven deployment {deployment} status failed five times: {error}")
            continue
        misses = 0
        print(f"release-registry: Maven deployment {deployment} is {state}")
        if state in ("PUBLISHING", "PUBLISHED"):
            return
        require(state != "FAILED", f"Maven deployment {deployment} failed validation; see the Central Portal")
        require(state in ("PENDING", "VALIDATING", "VALIDATED"), f"Maven deployment {deployment} has unknown state {state}")
    raise Refusal(f"Maven deployment {deployment} did not publish within 30 minutes")


def nuget_push(nupkg):
    key = secret("NUGET_API_KEY")
    require(nupkg.is_file(), "the nupkg is missing")
    body, kind = multipart("package", nupkg.name, nupkg.read_bytes())
    # nuget.org's default push policy answers 400 to a push without protocol 4.1.0 or newer (ticket 0391).
    request = urllib.request.Request("https://www.nuget.org/api/v2/package", data=body, method="PUT",
                                     headers={"X-NuGet-ApiKey": key, "X-NuGet-Protocol-Version": "4.1.0",
                                              "Content-Type": kind})
    try:
        with urllib.request.urlopen(request, timeout=300) as reply:
            require(reply.status in (200, 201, 202), f"NuGet answered {reply.status}")
    except urllib.error.HTTPError as error:
        # NuGet puts its reason in the status line and the body; neither should hold the key, and both lose it here.
        said = f"{error.reason}: {error.read().decode(errors='replace').replace(key, '[key]')[:500]}".replace(key, "[key]")
        raise Refusal(f"NuGet answered {error.code} {said}")
    print(f"release-registry: pushed {nupkg.name}")


def dart():
    return os.environ.get("TT_DART") or shutil.which("dart") or "dart"


def dry_run(pub):
    pubspec = (pub / "pubspec.yaml").read_text(encoding="utf-8")
    check_pubspec(pubspec, version())
    with tempfile.TemporaryDirectory(prefix="thinkthen-pub-") as tmp:
        copy = pathlib.Path(tmp) / PUB_NAME
        shutil.copytree(pub, copy)
        result = subprocess.run([dart(), "pub", "publish", "--dry-run"], cwd=copy, capture_output=True, text=True)
        sys.stdout.write(result.stdout[-2000:])
        require(result.returncode == 0, f"dart pub publish --dry-run exited {result.returncode}: {result.stderr[-500:]}")
    print(f"release-registry: pub dry run passed for {PUB_NAME}")


def pub_publish(pub):
    v = version()
    check_pubspec((pub / "pubspec.yaml").read_text(encoding="utf-8"), v)
    found = status_of(f"https://pub.dev/api/packages/{PUB_NAME}/versions/{v}")
    require(found == 404, f"pub.dev already holds {PUB_NAME} {v}" if found == 200
            else f"pub.dev answered {found} for the version check")
    # The same temporary copy as the dry run, so no repository ignore file changes what is sent.
    with tempfile.TemporaryDirectory(prefix="thinkthen-pub-") as tmp:
        copy = pathlib.Path(tmp) / PUB_NAME
        shutil.copytree(pub, copy)
        result = subprocess.run([dart(), "pub", "publish", "--force"], cwd=copy)
    require(result.returncode == 0, f"dart pub publish exited {result.returncode}")


def main(argv):
    try:
        match argv:
            case ["pack", platform, out, sha]:
                pack(pathlib.Path(platform), pathlib.Path(out), sha)
            case ["maven-sign", maven, "rehearse"]:
                maven_sign(pathlib.Path(maven), "rehearse")
            case ["maven-sign", maven, "release", bundle]:
                maven_sign(pathlib.Path(maven), "release", pathlib.Path(bundle))
            case ["maven-upload", bundle]:
                maven_upload(pathlib.Path(bundle))
            case ["nuget-push", nupkg]:
                nuget_push(pathlib.Path(nupkg))
            case ["dry-run", pub]:
                dry_run(pathlib.Path(pub))
            case ["pub-publish", pub]:
                pub_publish(pathlib.Path(pub))
            case _:
                print(__doc__, file=sys.stderr)
                return 2
    except Refusal as refusal:
        print(f"release-registry: {refusal}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
