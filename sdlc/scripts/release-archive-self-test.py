#!/usr/bin/env python3
"""Focused gitless release input proof. Cargo and Docker only stand in for builds."""

import os
from pathlib import Path
import shutil
import subprocess
import io
import tarfile
import tempfile


REPO = Path(__file__).resolve().parents[2]


def run(*args, cwd=REPO, env=None):
    return subprocess.run(args, cwd=cwd, env=env, text=True, capture_output=True)


def expect(result, text, success=False):
    if (result.returncode == 0) != success or (text and text not in result.stderr + result.stdout):
        raise AssertionError((result.args, result.returncode, result.stdout, result.stderr, text))


def resolve_outputs(commit, version):
    # release.yml appends resolve's standard output to GITHUB_OUTPUT, which takes only name=value lines.
    rehearsal = f"v{version}-rehearsal-{commit[:7]}"
    for mode, ref, name in (("rehearse", "refs/heads/main", rehearsal),
                            ("rehearse", "refs/heads/release/0.1", rehearsal),
                            ("release", f"refs/tags/v{version}", f"v{version}")):
        result = run("sh", str(REPO / "sdlc/scripts/release-workflow"), "resolve", mode, ref,
                     env=os.environ | {"GITHUB_SHA": commit})
        wanted = f"sha={commit}\nversion={version}\nname={name}\n"
        if result.returncode or result.stdout != wanted or "versions: " not in result.stderr:
            raise AssertionError(("resolve outputs", mode, result.returncode, result.stdout, result.stderr))
    # ADR 0116 item 7: rehearse runs from main or release/X.Y alone, and release only from a v* tag.
    for mode, ref, wanted in (
            ("rehearse", "refs/heads/feature", "rehearse must run from main or a release/X.Y branch"),
            ("rehearse", "refs/heads/release/next", "rehearse must run from main or a release/X.Y branch"),
            ("rehearse", "refs/heads/release/0.1/fix", "rehearse must run from main or a release/X.Y branch"),
            ("rehearse", "refs/tags/v0.1.0", "rehearse must run from main or a release/X.Y branch"),
            ("release", "refs/heads/release/0.1", "release must run from a v* tag")):
        result = run("sh", str(REPO / "sdlc/scripts/release-workflow"), "resolve", mode, ref,
                     env=os.environ | {"GITHUB_SHA": commit})
        if result.returncode != 1 or result.stdout or result.stderr != f"release-workflow: {wanted}, got {ref}\n":
            raise AssertionError(("resolve refusal", mode, ref, result.returncode, result.stdout, result.stderr))


def main():
    commit = subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=REPO, text=True).strip()
    version = next(line.split('"')[1] for line in (REPO / "crates/thinkthen/Cargo.toml").read_text().splitlines()
                   if line.startswith('version = "'))
    resolve_outputs(commit, version)
    host = subprocess.check_output(["rustc", "-vV"], text=True).split("host: ", 1)[1].splitlines()[0]
    if host != "x86_64-unknown-linux-gnu":
        print(f"release archive self-test: skipped synthetic C fixture on {host}")
        return
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
        # The draft requires every target's first-run archive to match byte for byte, and rehearsal run
        # 36937157758 failed on owner, time, order and gzip header differences. A rebuild after new
        # file times and a looser umask must give the same bytes and the fixed entry metadata.
        for path in (source / "demos/27-test-with-no-network").rglob("*"):
            os.utime(path, (1_700_000_000, 1_700_000_000))
        expect(run("sh", "-c", 'umask 002 && exec sh "$0" "$@"', str(source / "sdlc/scripts/release-pack"),
                   host, str(base / "again"), "first-run", cwd=source), "", success=True)
        first_run = (base / "legacy/thinkthen-first-run.tar.gz").read_bytes()
        with tarfile.open(fileobj=io.BytesIO(first_run), mode="r:gz") as packed:
            entries = [(entry.name, entry.mode, entry.uid, entry.gid, entry.uname, entry.mtime)
                       for entry in packed.getmembers()]
        if (first_run != (base / "again/thinkthen-first-run.tar.gz").read_bytes() or first_run[4:9] != bytes(5)
                or entries != [("thinkthen-first-run", 0o755, 0, 0, "", 0),
                               ("thinkthen-first-run/recording", 0o755, 0, 0, "", 0),
                               ("thinkthen-first-run/recording/thinkthen.jsonl", 0o644, 0, 0, "", 0),
                               ("thinkthen-first-run/report.txt", 0o644, 0, 0, "", 0)]):
            raise AssertionError(("first-run archive is not reproducible", entries))
        expect(run("sh", str(source / "sdlc/scripts/release-pack"), host,
                   str(base / "unselected"), "go", cwd=source), "archived source tar path must be absolute")

        fake_bin = base / "bin"
        fake_bin.mkdir()
        cargo = fake_bin / "cargo"
        cargo.write_text("#!/bin/sh\nprintf 'called\\n' >>\"$THINKTHEN_CARGO_CALLS\"\nexit 0\n")
        cargo.chmod(0o755)
        native = base / "native-target/release"
        native.mkdir(parents=True)
        (native / "libthinkthen_c.so").write_bytes(b"fixture shared")
        # release-pack localizes the static library, so the fixture is a real one-function archive.
        (base / "fixture.c").write_text("int thinkthen_fixture(void) { return 0; }\n")
        subprocess.run(["cc", "-c", "-o", str(base / "fixture.o"), str(base / "fixture.c")], check=True)
        subprocess.run(["ar", "rcs", str(native / "libthinkthen_c.a"), str(base / "fixture.o")], check=True)
        env = os.environ.copy()
        env.update(PATH=str(fake_bin) + os.pathsep + env["PATH"],
                   CARGO_TARGET_DIR=str(native.parent),
                   THINKTHEN_CARGO_CALLS=str(base / "cargo-calls"),
                   THINKTHEN_ARCHIVED_SOURCE_TAR=str(archive),
                   THINKTHEN_ARCHIVED_SOURCE_COMMIT=commit)
        # Keep the held SQL/DataFrame families out of execution; their allowlist is unchanged.
        parts = ("c", "go", "cpp", "swift", "zig", "php", "dart", "ada", "objective-c", "cobol")
        expect(run("sh", str(source / "sdlc/scripts/release-pack"), host,
                   str(base / "paired"), *parts, cwd=source, env=env), "", success=True)
        for kind in parts:
            if len(list((base / "paired").glob(f"thinkthen-{kind}-*.tar.gz"))) != 1:
                raise AssertionError(f"missing {kind} fixture archive")
        # Ticket 0366: with no CARGO_TARGET_DIR, each part builds in its own scratch folder, which
        # goes once the part is packed. This cargo records the folder it was given and fills it.
        own_bin, own_tmp, own_calls = base / "own-bin", base / "own-tmp", base / "own-calls"
        own_bin.mkdir()
        own_tmp.mkdir()
        (own_bin / "cargo").write_text("#!/bin/sh\nprintf '%s\\n' \"$CARGO_TARGET_DIR\" >>\"$THINKTHEN_CARGO_CALLS\"\n"
                                       "mkdir -p \"$CARGO_TARGET_DIR/release\"\n"
                                       f"cp '{native}'/* \"$CARGO_TARGET_DIR/release/\"\n")
        (own_bin / "cargo").chmod(0o755)
        own_env = {key: value for key, value in env.items() if key != "CARGO_TARGET_DIR"}
        own_env.update(PATH=str(own_bin) + os.pathsep + os.environ["PATH"], TMPDIR=str(own_tmp),
                       THINKTHEN_CARGO_CALLS=str(own_calls))
        result = run("sh", str(source / "sdlc/scripts/release-pack"), host, str(base / "own"), "c", "first-run",
                     cwd=source, env=own_env)
        expect(result, "release-pack: c built", success=True)
        built = own_calls.read_text().splitlines()
        if (len(built) != 1 or not built[0].startswith(str(own_tmp)) or Path(built[0]).exists()
                or list(own_tmp.iterdir()) or "release-pack: first-run built" not in result.stderr
                or not list((base / "own").glob(f"thinkthen-c-{version}-{host}.tar.gz"))):
            raise AssertionError(("own build folders", built, list(own_tmp.iterdir()), result.stderr))
        gate = str(REPO / "sdlc/scripts/release-workflow")
        expect(run("sh", gate, "go-cpp-gate", str(base / "paired"), host, commit), "", success=True)
        expect(run("sh", gate, "swift-zig-gate", str(base / "paired"), host, commit), "", success=True)
        expect(run("sh", gate, "php-dart-gate", str(base / "paired"), host, commit), "", success=True)
        expect(run("sh", gate, "ada-objc-cobol-gate", str(base / "paired"), host, commit), "", success=True)
        for family, relative in (("swift", "Sources/ThinkThen/ThinkThen.swift"),
                                 ("zig", "src/thinkthen.zig"),
                                 ("php", "autoload.php"),
                                 ("dart", "lib/src/door.dart"),
                                 ("ada", "src/thinkthen.ads"),
                                 ("objective-c", "Sources/ThinkThen.m"),
                                 ("cobol", "src/tt_call.cob")):
            copied = source / "libraries" / family / relative
            original = copied.read_bytes()
            copied.write_bytes(original + b"\n// altered archived wrapper\n")
            output = base / f"{family}-changed"
            expect(run("sh", str(source / "sdlc/scripts/release-pack"), host,
                       str(output), *parts, cwd=source, env=env),
                   f"source file differs: libraries/{family}/{relative}")
            if output.exists():
                raise AssertionError(f"changed {family} source created package output")
            copied.write_bytes(original)
        # Alter only the staged copy after the full extracted-tree check has passed.
        # This exercises the wrapper's own byte comparison against the selected tar.
        fake_cp = fake_bin / "cp"
        fake_cp.write_text("#!/bin/sh\n"
                           "last=\nfor arg do last=$arg; done\n"
                           "/bin/cp \"$@\" || exit\n"
                           "for arg do if [ \"$arg\" = \"$THINKTHEN_PLANT_SOURCE\" ]; then\n"
                           "  printf 'changed copy\\n' >>\"$last/${arg##*/}\"\n"
                           "fi; if [ \"$arg\" = \"$THINKTHEN_PLANT_TWIN\" ]; then\n"
                           "  : >\"$last/$(printf %s \"${arg##*/}\" | tr A-Z a-z)\"\n"
                           "fi; done\n")
        fake_cp.chmod(0o755)
        for family, relative in (("ada", "src/thinkthen.ads"),
                                 ("objective-c", "Sources/ThinkThen.m"),
                                 ("cobol", "src/tt_call.cob")):
            copied_env = env | {"THINKTHEN_PLANT_SOURCE": f"libraries/{family}/{relative}"}
            output = base / f"{family}-copied-change"
            expect(run("sh", str(source / "sdlc/scripts/release-pack"), host,
                       str(output), *parts, cwd=source, env=copied_env),
                   f"{family} source differs from archived commit: {relative}")
            if list(output.glob(f"thinkthen-{family}-*")):
                raise AssertionError(f"changed {family} copied member created wrapper output")
        # The old Objective-C package held ThinkThen.h beside thinkthen.h.
        output = base / "objective-c-twin"
        expect(run("sh", str(source / "sdlc/scripts/release-pack"), host, str(output), *parts, cwd=source,
                   env=env | {"THINKTHEN_PLANT_TWIN": "libraries/objective-c/Sources/ThinkThen.h"}),
               "objective-c holds names that differ only in case: ./sources/thinkthen.h")
        if list(output.glob("thinkthen-objective-c-*")):
            raise AssertionError("case-only twin created wrapper output")
        fake_cp.unlink()
        missing_pair = base / "missing-pair"
        shutil.copytree(base / "paired", missing_pair)
        for file in missing_pair.glob("thinkthen-cpp-*"):
            file.unlink()
        expect(run("sh", gate, "go-cpp-gate", str(missing_pair), host, commit),
               "missing or linked thinkthen-cpp-")
        missing_swift = base / "missing-swift"
        shutil.copytree(base / "paired", missing_swift)
        for file in missing_swift.glob("thinkthen-swift-*"):
            file.unlink()
        expect(run("sh", gate, "swift-zig-gate", str(missing_swift), host, commit),
               "missing or linked thinkthen-swift-")
        for family in ("php", "dart"):
            missing = base / f"missing-{family}"
            shutil.copytree(base / "paired", missing)
            for file in missing.glob(f"thinkthen-{family}-*"):
                file.unlink()
            expect(run("sh", gate, "php-dart-gate", str(missing), host, commit),
                   f"missing or linked thinkthen-{family}-")
        for family in ("ada", "objective-c", "cobol"):
            missing = base / f"missing-{family}"
            shutil.copytree(base / "paired", missing)
            for file in missing.glob(f"thinkthen-{family}-*"):
                file.unlink()
            expect(run("sh", gate, "ada-objc-cobol-gate", str(missing), host, commit),
                   f"missing or linked thinkthen-{family}-")
        no_sidecar = base / "missing-cobol-sidecar"
        shutil.copytree(base / "paired", no_sidecar)
        (no_sidecar / f"thinkthen-cobol-{version}-{host}.tar.gz.sha256").unlink()
        expect(run("sh", gate, "ada-objc-cobol-gate", str(no_sidecar), host, commit),
               f"missing or linked thinkthen-cobol-{version}-{host}.tar.gz.sha256")
        another_commit = "0" * 40
        expect(run("sh", gate, "go-cpp-gate", str(base / "paired"), host, another_commit),
               "checkout differs from resolved SHA")
        expect(run("sh", gate, "swift-zig-gate", str(base / "paired"), host, another_commit),
               "checkout differs from resolved SHA")
        expect(run("sh", gate, "php-dart-gate", str(base / "paired"), host, another_commit),
               "checkout differs from resolved SHA")
        expect(run("sh", gate, "ada-objc-cobol-gate", str(base / "paired"), host, another_commit),
               "checkout differs from resolved SHA")
        altered_source = base / "altered-source"
        shutil.copytree(base / "paired", altered_source)
        for family in ("go", "cpp"):
            name = f"thinkthen-{family}-{version}-{host}.tar.gz"
            path = altered_source / name
            repacked = altered_source / f"{family}.tmp"
            with tarfile.open(path, "r:gz") as archive, tarfile.open(repacked, "w:gz") as output:
                for member in archive:
                    if member.name == "./THINKTHEN-PACKAGE-INPUTS":
                        data = archive.extractfile(member).read().replace(commit.encode(), another_commit.encode())
                        member.size = len(data)
                        output.addfile(member, io.BytesIO(data))
                    else:
                        output.addfile(member, archive.extractfile(member) if member.isfile() else None)
            repacked.replace(path)
            digest = subprocess.check_output(["sha256sum", str(path)], text=True).split()[0]
            (altered_source / f"{name}.sha256").write_text(f"{digest}  {name}\n")
        expect(run("sh", gate, "go-cpp-gate", str(altered_source), host, commit),
               "go source differs from resolved SHA")
        other_target = "aarch64-unknown-linux-gnu"
        expect(run("sh", gate, "go-cpp-gate", str(base / "paired"), other_target, commit),
               "unsupported target has thinkthen-go-")
        expect(run("sh", gate, "swift-zig-gate", str(base / "paired"), other_target, commit),
               "unsupported target has thinkthen-swift-")
        expect(run("sh", gate, "php-dart-gate", str(base / "paired"), other_target, commit),
               "unsupported target has thinkthen-php-")
        platform = base / "platform"
        platform.mkdir()
        for target in (host, other_target, "aarch64-apple-darwin", "x86_64-apple-darwin"):
            folder = platform / f"platform-{target}"
            if target == host:
                shutil.copytree(base / "paired", folder)
            else:
                folder.mkdir()
                (folder / f"fixture-{target}.bin").write_bytes(b"existing target file")
        npm = base / "npm"
        npm.mkdir()
        (npm / f"thinkthen-{version}.tgz").write_bytes(b"fixture npm")
        (npm / f"thinkthen-{version}.tgz.sha256").write_text("fixture sidecar\n")
        collected = base / "collected"
        expect(run("sh", gate, "collect", str(platform), str(npm), str(collected)), "", success=True)
        expected_files = {f"thinkthen-{kind}-{version}-{host}.tar.gz{suffix}"
                          for kind in parts for suffix in ("", ".sha256")}
        expected_files |= {f"fixture-{target}.bin" for target in
                           (other_target, "aarch64-apple-darwin", "x86_64-apple-darwin")}
        expected_files |= {f"thinkthen-{version}.tgz", f"thinkthen-{version}.tgz.sha256"}
        if {file.name for file in collected.iterdir()} != expected_files:
            raise AssertionError("collect omitted or added a selected fixture file")
        extra_go = platform / f"platform-{host}/thinkthen-go-extra.zip"
        extra_go.write_bytes(b"unselected release asset")
        expect(run("sh", gate, "go-cpp-gate", str(extra_go.parent), host, commit),
               "unexpected Go/C++ family entry thinkthen-go-extra.zip")
        refused_output = base / "refused-go-assets"
        expect(run("sh", gate, "collect", str(platform), str(base / "no-npm"), str(refused_output)),
               "unexpected Go/C++ family entry thinkthen-go-extra.zip")
        if refused_output.exists():
            raise AssertionError("extra Go asset created collected output")
        extra_go.unlink()
        extra_swift = platform / f"platform-{host}/thinkthen-swift-extra.zip"
        extra_swift.write_bytes(b"unselected release asset")
        expect(run("sh", gate, "swift-zig-gate", str(extra_swift.parent), host, commit),
               "unexpected Swift/Zig family entry thinkthen-swift-extra.zip")
        refused_output = base / "refused-swift-assets"
        expect(run("sh", gate, "collect", str(platform), str(base / "no-npm"), str(refused_output)),
               "unexpected Swift/Zig family entry thinkthen-swift-extra.zip")
        if refused_output.exists():
            raise AssertionError("extra Swift asset created collected output")
        extra_swift.unlink()
        extra_php = platform / f"platform-{host}/thinkthen-php-extra.zip"
        extra_php.write_bytes(b"unselected release asset")
        expect(run("sh", gate, "php-dart-gate", str(extra_php.parent), host, commit),
               "unexpected PHP/Dart family entry thinkthen-php-extra.zip")
        refused_output = base / "refused-php-assets"
        expect(run("sh", gate, "collect", str(platform), str(base / "no-npm"), str(refused_output)),
               "unexpected PHP/Dart family entry thinkthen-php-extra.zip")
        if refused_output.exists():
            raise AssertionError("extra PHP asset created collected output")
        extra_php.unlink()
        extra_cobol = platform / f"platform-{host}/thinkthen-cobol-{version}-{other_target}.tar.zip"
        extra_cobol.write_bytes(b"unselected release asset")
        expect(run("sh", gate, "ada-objc-cobol-gate", str(extra_cobol.parent), host, commit),
               f"unexpected Ada/Objective-C/COBOL family entry {extra_cobol.name}")
        refused_output = base / "refused-cobol-assets"
        expect(run("sh", gate, "collect", str(platform), str(base / "no-npm"), str(refused_output)),
               f"unexpected Ada/Objective-C/COBOL family entry {extra_cobol.name}")
        if refused_output.exists():
            raise AssertionError("extra COBOL asset created collected output")
        extra_cobol.unlink()
        for target, family in ((other_target, "ada"),
                               ("aarch64-apple-darwin", "objective-c"),
                               ("x86_64-apple-darwin", "cobol")):
            foreign = platform / f"platform-{target}/thinkthen-{family}-{version}-{target}.tar.gz"
            foreign.write_bytes(b"unsupported language asset")
            expect(run("sh", gate, "ada-objc-cobol-gate", str(foreign.parent), target, commit),
                   f"unsupported target has {foreign.name}")
            refused_output = base / f"refused-{family}-{target}"
            expect(run("sh", gate, "collect", str(platform), str(base / "no-npm"), str(refused_output)),
                   f"unsupported target has {foreign.name}")
            if refused_output.exists():
                raise AssertionError(f"{family} asset on {target} created collected output")
            foreign.unlink()
        extra_platform = platform / "platform-extra"
        extra_platform.mkdir()
        (extra_platform / "unselected.zip").write_bytes(b"unselected release asset")
        refused_output = base / "refused-platform"
        expect(run("sh", gate, "collect", str(platform), str(base / "no-npm"), str(refused_output)),
               "unexpected platform folder platform-extra")
        if refused_output.exists():
            raise AssertionError("extra platform folder created collected output")
        for family in ("ada", "objective-c", "cobol"):
            script = (REPO / "libraries" / family / "check.sh").read_text()
            installed_end = script.index("  exit 0\nfi\n")
            source_start = script.index("unset THINKTHEN_API_KEY", installed_end)
            source_only = script[installed_end:source_start]
            if "command -v node" not in source_only or "import jsonschema" not in source_only:
                raise AssertionError(f"{family} source prerequisites precede installed return")
        fake_curl = fake_bin / "curl"
        fake_curl.write_text("#!/bin/sh\nwhile [ $# -gt 0 ]; do\n"
                             "if [ \"$1\" = -o ]; then shift; printf wrong >\"$1\"; exit 0; fi\n"
                             "shift\ndone\nexit 2\n")
        fake_curl.chmod(0o755)
        github_path = base / "github-path"
        github_path.touch()
        github_env = base / "github-env"
        github_env.touch()
        tool_env = os.environ | {"PATH": str(fake_bin) + os.pathsep + os.environ["PATH"],
                                 "GITHUB_PATH": str(github_path), "GITHUB_ENV": str(github_env)}
        expect(run("sh", gate, "go-cpp-tools", other_target, str(base / "other-tools"), env=tool_env),
               "Go/C++ installed tools require Linux x86-64")
        expect(run("sh", gate, "go-cpp-tools", host, str(base / "bad-go"), env=tool_env),
               "Go 1.27.1 archive differs from official checksum")
        expect(run("sh", gate, "swift-zig-tools", other_target, str(base / "other-zig"), env=tool_env),
               "Swift/Zig installed tools require Linux x86-64")
        expect(run("sh", gate, "swift-zig-tools", host, str(base / "bad-zig"), env=tool_env),
               "Zig 0.15.2 archive differs from official checksum")
        expect(run("sh", gate, "php-dart-tools", other_target, str(base / "other-dart"), env=tool_env),
               "PHP/Dart installed tools require Linux x86-64")
        expect(run("sh", gate, "php-dart-tools", host, str(base / "bad-dart"), env=tool_env),
               "Dart 3.13.4 SDK differs from official checksum")
        if github_path.read_text():
            raise AssertionError("bad tool archive entered the runner path")
        if github_env.read_text():
            raise AssertionError("bad Dart archive entered the runner environment")
        # Let synthetic extraction and pub filling reach the real cache-hash guard.
        # The earlier bad-download plant exercised the real SDK checksum command.
        fake_hash = fake_bin / "sha256sum"
        fake_hash.write_text("#!/bin/sh\n[ \"$1\" = -c ] && exit 0\nexit 2\n")
        fake_hash.chmod(0o755)
        fake_unzip = fake_bin / "unzip"
        fake_unzip.write_text("#!/bin/sh\nwhile [ $# -gt 0 ]; do\n"
                              "if [ \"$1\" = -d ]; then shift; mkdir -p \"$1/dart-sdk/bin\"; "
                              "cat >\"$1/dart-sdk/bin/dart\" <<'SH'\n"
                              "#!/bin/sh\nif [ \"$1\" = --version ]; then "
                              "echo 'Dart SDK version: 3.13.4 (stable)' >&2; exit 0; fi\n"
                              "mkdir -p \"$PUB_CACHE/hosted/pub.dev/ffi-2.2.0\" "
                              "\"$PUB_CACHE/hosted-hashes/pub.dev\"\n"
                              "printf '%s\\n' wrong >\"$PUB_CACHE/hosted-hashes/pub.dev/ffi-2.2.0.sha256\"\n"
                              "SH\nchmod +x \"$1/dart-sdk/bin/dart\"; exit 0; fi\nshift\ndone\nexit 2\n")
        fake_unzip.chmod(0o755)
        expect(run("sh", gate, "php-dart-tools", host, str(base / "bad-ffi"), env=tool_env),
               "cached ffi 2.2.0 differs from official digest")
        if github_path.read_text() or github_env.read_text():
            raise AssertionError("bad ffi cache entered the runner environment")
        fake_hash.unlink()
        fake_unzip.unlink()
        wrong = env | {"THINKTHEN_ARCHIVED_SOURCE_COMMIT": "0" * 40}
        expect(run("sh", str(source / "sdlc/scripts/release-pack"), host,
                   str(base / "wrong"), *parts, cwd=source, env=wrong),
               "archived source commit differs from selected checkout")
        expect(run("sh", str(source / "sdlc/scripts/release-pack"), "--reuse", host,
                   str(base / "reuse"), *parts, cwd=source, env=env),
               "archived source does not permit --reuse")
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
        c_header.unlink()
        expect(run("sh", str(source / "sdlc/scripts/release-pack"), host,
                   str(base / "native-missing"), *parts, cwd=source, env=env),
               "source file differs: libraries/c/include/thinkthen.h")
        c_header.symlink_to("../../../LICENSE")
        expect(run("sh", str(source / "sdlc/scripts/release-pack"), host,
                   str(base / "native-type"), *parts, cwd=source, env=env),
               "source file differs: libraries/c/include/thinkthen.h")
        c_header.unlink()
        c_header.write_bytes(original_header)
        source_link = source / "CLAUDE.md"
        original_link = os.readlink(source_link)
        source_link.unlink()
        source_link.symlink_to("README.md")
        expect(run("sh", str(source / "sdlc/scripts/release-pack"), host,
                   str(base / "source-link"), *parts, cwd=source, env=env),
               "source link differs: CLAUDE.md")
        source_link.unlink()
        source_link.symlink_to(original_link)
        extra_config = source / "libraries/c/.cargo/config.toml"
        extra_config.parent.mkdir()
        extra_config.write_text("[build]\nrustflags = []\n")
        expect(run("sh", str(source / "sdlc/scripts/release-pack"), host,
                   str(base / "extra-config"), *parts, cwd=source, env=env),
               "unexpected source path: libraries/c/.cargo/config.toml")
        extra_config.unlink()
        extra_config.parent.rmdir()
        calls_before_refusal = (base / "cargo-calls").read_text().splitlines()
        (source / "libraries/go/README.md").write_text("changed source\n")
        expect(run("sh", str(source / "sdlc/scripts/release-pack"), host,
                   str(base / "changed"), *parts, cwd=source, env=env),
               "source file differs: libraries/go/README.md")
        if (base / "cargo-calls").read_text().splitlines() != calls_before_refusal:
            raise AssertionError("an archived-source refusal reached Cargo")
        expect(run("sh", str(REPO / "sdlc/scripts/release-pack"), host,
                   str(base / "override"), "go", env=env),
               "archive identity cannot override a repository checkout")

        # Capture only the launcher's transport inputs. Assert their identity below.
        docker = fake_bin / "docker"
        docker.write_text("#!/bin/sh\n"
                          "for arg do case $arg in *:/work) root=${arg%:/work} ;; "
                          "THINKTHEN_ARCHIVED_SOURCE_COMMIT=*) selected=${arg#*=} ;; "
                          "THINKTHEN_ARCHIVED_SOURCE_TAR=*) tar_path=${arg#*=} ;; esac; done\n"
                          "printf '%s\\n' \"$selected\" >\"$root/out/selected-commit\"\n"
                          "printf '%s\\n' \"$tar_path\" >\"$root/out/selected-tar\"\n"
                          "cp \"$root/source.tar\" \"$root/out/source.tar\"\n")
        docker.chmod(0o755)
        launcher_env = os.environ.copy()
        launcher_env["PATH"] = str(fake_bin) + os.pathsep + launcher_env["PATH"]
        launcher_env["THINKTHEN_RELEASE_EXPECTED_SHA"] = "0" * 40
        expect(run("sh", str(REPO / "sdlc/scripts/release-container"), str(base / "container-work"),
                   str(base / "container-out"), env=launcher_env), "checkout differs from resolved release SHA")
        launcher_env["THINKTHEN_RELEASE_EXPECTED_SHA"] = commit
        expect(run("sh", str(REPO / "sdlc/scripts/release-container"), str(base / "container-work"),
                   str(base / "container-out"), env=launcher_env), "", success=True)
        output = base / "container-out"
        if (output / "selected-commit").read_text().strip() != commit:
            raise AssertionError("launcher passed another selected commit")
        if (output / "selected-tar").read_text().strip() != "/work/source.tar":
            raise AssertionError("launcher passed another tar path")
        with (output / "source.tar").open("rb") as packed:
            actual = subprocess.check_output(["git", "get-tar-commit-id"], stdin=packed, text=True).strip()
        if actual != commit:
            raise AssertionError("launcher passed a tar from another source commit")
        fake_go = fake_bin / "fake-go"
        fake_go.write_text("#!/bin/sh\nprintf '%s\\n' \"$FAKE_GO_VERSION\"\n")
        fake_go.chmod(0o755)
        no_python = fake_bin / "no-python"
        no_python.write_text("#!/bin/sh\nexit 1\n")
        no_python.chmod(0o755)
        for version, diagnostic in (
            ("go version go1.21.9 linux/amd64", "Go 1.22 or newer is required"),
            ("go version go1.27rc1 linux/amd64", "a stable Go 1.x release is required"),
            ("go version go2.0.0 linux/amd64", "a stable Go 1.x release is required"),
            ("go version go1.22.12 linux/amd64", "Python jsonschema is unavailable"),
            ("go version go1.27.1 linux/amd64", "Python jsonschema is unavailable"),
        ):
            version_env = os.environ | {"THINKTHEN_GO_BIN": str(fake_go),
                                        "THINKTHEN_PYTHON_BIN": str(no_python),
                                        "FAKE_GO_VERSION": version}
            expect(run("sh", str(REPO / "libraries/go/check.sh"), "0", env=version_env), diagnostic)
        installed_env = os.environ | {"THINKTHEN_GO_BIN": str(fake_go),
                                      "THINKTHEN_PYTHON_BIN": str(no_python),
                                      "FAKE_GO_VERSION": "go version go1.27.1 linux/amd64",
                                      "THINKTHEN_ARTIFACT": str(base / "absent-wrapper"),
                                      "THINKTHEN_HEAVY_LOCK": str(base / "held-lock"),
                                      "THINKTHEN_HEAVY_LOCK_HELD": str(base / "held-lock")}
        for family in ("go", "cpp"):
            expect(run("sh", str(REPO / f"libraries/{family}/check.sh"), "0", env=installed_env),
                   f"{family}: missing C archive")
            source_env = installed_env | {"THINKTHEN_ARTIFACT": ""}
            expect(run("sh", str(REPO / f"libraries/{family}/check.sh"), "0", env=source_env),
                   f"{family}: not run: Python jsonschema is unavailable")
    print("release archive self-test: gitless legacy, Go/C++, Swift/Zig and PHP/Dart inputs pass")


if __name__ == "__main__":
    main()
