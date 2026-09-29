#!/usr/bin/env python3
"""Focused gitless release input proof. Cargo and Docker only stand in for builds."""

import os
from pathlib import Path
import shutil
import subprocess
import tempfile


REPO = Path(__file__).resolve().parents[2]


def run(*args, cwd=REPO, env=None):
    return subprocess.run(args, cwd=cwd, env=env, text=True, capture_output=True)


def expect(result, text, success=False):
    if (result.returncode == 0) != success or (text and text not in result.stderr + result.stdout):
        raise AssertionError((result.args, result.returncode, result.stdout, result.stderr, text))


def main():
    host = subprocess.check_output(["rustc", "-vV"], text=True).split("host: ", 1)[1].splitlines()[0]
    commit = subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=REPO, text=True).strip()
    version = next(line.split('"')[1] for line in (REPO / "crates/thinkthen/Cargo.toml").read_text().splitlines()
                   if line.startswith('version = "'))
    with tempfile.TemporaryDirectory(prefix="thinkthen-release-archive-") as temporary:
        base = Path(temporary)
        source = base / "source"
        source.mkdir()
        archive = base / "source.tar"
        with archive.open("wb") as output:
            subprocess.run(["git", "archive", "--format=tar", "HEAD"], cwd=REPO, stdout=output, check=True)
        subprocess.run(["tar", "-xf", str(archive), "-C", str(source)], check=True)
        if (source / ".git").exists():
            raise AssertionError("Git archive unexpectedly contains .git")
        # Exercise the working candidate before it is committed; archived product files still
        # come from HEAD and are compared by release-pack to the original tar below.
        shutil.copy2(REPO / "sdlc/scripts/release-pack", source / "sdlc/scripts/release-pack")
        expect(run("sh", str(source / "sdlc/scripts/release-pack"), host,
                   str(base / "legacy"), "first-run", cwd=source), "", success=True)
        if not (base / "legacy/thinkthen-first-run.tar.gz").is_file():
            raise AssertionError("gitless legacy archive missing")
        expect(run("sh", str(source / "sdlc/scripts/release-pack"), host,
                   str(base / "unselected"), "go", cwd=source), "archived source tar path must be absolute")

        fake_bin = base / "bin"
        fake_bin.mkdir()
        cargo = fake_bin / "cargo"
        cargo.write_text("#!/bin/sh\nprintf 'called\\n' >>\"$THINKTHEN_CARGO_CALLS\"\nexit 0\n")
        cargo.chmod(0o755)
        native = source / "libraries/c/target/release"
        native.mkdir(parents=True)
        (native / "libthinkthen_c.so").write_bytes(b"fixture shared")
        (native / "libthinkthen_c.a").write_bytes(b"fixture static")
        env = os.environ.copy()
        env.update(PATH=str(fake_bin) + os.pathsep + env["PATH"],
                   THINKTHEN_CARGO_CALLS=str(base / "cargo-calls"),
                   THINKTHEN_ARCHIVED_SOURCE_TAR=str(archive),
                   THINKTHEN_ARCHIVED_SOURCE_COMMIT=commit)
        parts = ("c", "go", "cpp")
        expect(run("sh", str(source / "sdlc/scripts/release-pack"), host,
                   str(base / "paired"), *parts, cwd=source, env=env), "", success=True)
        for kind in parts:
            if len(list((base / "paired").glob(f"thinkthen-{kind}-*.tar.gz"))) != 1:
                raise AssertionError(f"missing {kind} fixture archive")
        wrong = env | {"THINKTHEN_ARCHIVED_SOURCE_COMMIT": "0" * 40}
        expect(run("sh", str(source / "sdlc/scripts/release-pack"), host,
                   str(base / "wrong"), *parts, cwd=source, env=wrong),
               "archived source commit differs from selected checkout")
        expect(run("sh", str(source / "sdlc/scripts/release-pack"), "--reuse", host,
                   str(base / "reuse"), *parts, cwd=source, env=env),
               "source wrappers need a fresh C build")
        stale = base / "stale"
        stale.mkdir()
        (stale / f"thinkthen-c-{version}-{host}.tar.gz").write_bytes(b"prior C")
        expect(run("sh", str(source / "sdlc/scripts/release-pack"), host,
                   str(stale), *parts, cwd=source, env=env),
               "output without an existing C archive or sidecar")
        c_header = source / "libraries/c/include/thinkthen.h"
        original_header = c_header.read_bytes()
        c_header.write_bytes(b"changed native header\n")
        expect(run("sh", str(source / "sdlc/scripts/release-pack"), host,
                   str(base / "native-changed"), *parts, cwd=source, env=env),
               "source file differs: libraries/c/include/thinkthen.h")
        c_header.write_bytes(original_header)
        (source / "libraries/go/README.md").write_text("changed source\n")
        expect(run("sh", str(source / "sdlc/scripts/release-pack"), host,
                   str(base / "changed"), *parts, cwd=source, env=env),
               "source file differs: libraries/go/README.md")
        if (base / "cargo-calls").read_text().splitlines() != ["called"]:
            raise AssertionError("an archived-source refusal reached Cargo")
        expect(run("sh", str(REPO / "sdlc/scripts/release-pack"), host,
                   str(base / "override"), "go", env=env),
               "archive identity cannot override a repository checkout")

        # The launcher itself checks the real tar identifier before Docker. A fake Docker
        # writes one output marker only so this focused path need not run SQL packaging.
        docker = fake_bin / "docker"
        docker.write_text("#!/bin/sh\n"
                          "for arg do case $arg in *:/work) root=${arg%:/work} ;; "
                          "THINKTHEN_ARCHIVED_SOURCE_COMMIT=*) selected=${arg#*=} ;; "
                          "THINKTHEN_ARCHIVED_SOURCE_TAR=*) tar_path=${arg#*=} ;; esac; done\n"
                          "[ -n \"$root\" ] && [ \"$tar_path\" = /work/source.tar ] || exit 2\n"
                          "actual=$(git get-tar-commit-id <\"$root/source.tar\")\n"
                          "[ \"$actual\" = \"$selected\" ] || exit 3\n"
                          "printf '%s\\n' \"$actual\" >\"$root/out/receipt\"\n")
        docker.chmod(0o755)
        launcher_env = os.environ.copy()
        launcher_env["PATH"] = str(fake_bin) + os.pathsep + launcher_env["PATH"]
        launcher_env["THINKTHEN_RELEASE_EXPECTED_SHA"] = "0" * 40
        expect(run("sh", str(REPO / "sdlc/scripts/release-container"), str(base / "container-work"),
                   str(base / "container-out"), env=launcher_env), "checkout differs from resolved release SHA")
        launcher_env["THINKTHEN_RELEASE_EXPECTED_SHA"] = commit
        expect(run("sh", str(REPO / "sdlc/scripts/release-container"), str(base / "container-work"),
                   str(base / "container-out"), env=launcher_env), "", success=True)
        if (base / "container-out/receipt").read_text().strip() != commit:
            raise AssertionError("launcher passed a tar from another source commit")
    print("release archive self-test: gitless legacy and controlled Go/C++ inputs pass")


if __name__ == "__main__":
    main()
