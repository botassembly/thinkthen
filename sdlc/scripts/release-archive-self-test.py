#!/usr/bin/env python3
"""Focused gitless release input proof. Cargo and Docker only stand in for builds."""

import os
import json
import runpy
import sys
from unittest import mock
from pathlib import Path
import shutil
import subprocess
import io
import hashlib
import tarfile
import tempfile
import hashlib
import struct
import zipfile


REPO = Path(__file__).resolve().parents[2]
C_FIXTURE = __import__('runpy').run_path(str(REPO / 'sdlc/scripts/release-windows-c-fixture.py'))


def run(*args, cwd=REPO, env=None):
    return subprocess.run(args, cwd=cwd, env=env, text=True, capture_output=True)


def expect(result, text, success=False):
    if (result.returncode == 0) != success or (text and text not in result.stderr + result.stdout):
        raise AssertionError((result.args, result.returncode, result.stdout, result.stderr, text))


def draft_notes():
    with tempfile.TemporaryDirectory(prefix="thinkthen-draft-notes-") as temporary:
        root = Path(temporary)
        for name in ("sdlc/scripts/release-workflow", "sdlc/scripts/scratch.sh",
                     "sdlc/scripts/fetch.sh", "crates/thinkthen/Cargo.toml",
                     "release-notes/0.2.0.md", "release-notes/0.2.0-known-gaps.md"):
            target = root / name
            target.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(REPO / name, target)
        binary = root / "bin"
        binary.mkdir()
        receipt = root / "calls.jsonl"
        commit = "1" * 40
        for name, code in {
            "git": f"#!/bin/sh\nprintf '%s\\n' '{commit}'\n",
            "python3": "#!/bin/sh\nexit 0\n",
            "gh": f"#!{sys.executable}\n" + """import json, os, sys
from pathlib import Path
args = sys.argv[1:]
body = Path(args[args.index('--notes-file') + 1]).read_text() if '--notes-file' in args else None
with open(os.environ['GH_TEST_RECEIPT'], 'a') as held:
    held.write(json.dumps([args, body]) + '\\n')
if args[:2] == ['release', 'view']:
    sys.exit(1)
if args[0] == 'api':
    print('HTTP/2 404')
""",
        }.items():
            path = binary / name
            path.write_text(code)
            path.chmod(0o755)
        files = root / "files"
        files.mkdir()
        (files / "fixture.sha256").write_text("checksum\n")
        env = {"PATH": str(binary) + ":/usr/bin:/bin", "HOME": str(root),
               "LC_ALL": "C.UTF-8", "GH_TEST_RECEIPT": str(receipt),
               "GITHUB_REPOSITORY": "botassembly/thinkthen", "GITHUB_REF": "refs/tags/v0.2.0"}
        expected = ((root / "release-notes/0.2.0.md").read_text() + "\n" +
                    (root / "release-notes/0.2.0-known-gaps.md").read_text())
        for mode in ("release", "rehearse"):
            receipt.write_text("")
            result = run("sh", str(root / "sdlc/scripts/release-workflow"), "draft", mode,
                         "v0.2.0", commit, str(files), env=env)
            expect(result, "", success=True)
            calls = [json.loads(line) for line in receipt.read_text().splitlines()]
            created = [call for call in calls if call[0][:2] == ['release', 'create']]
            wanted = expected if mode == "release" else f"Rehearsal from {commit}; do not publish\n\n" + expected
            if len(created) != 1 or created[0][1] != wanted:
                raise AssertionError(("draft notes", mode, created))
            if ('--draft' not in created[0][0] or
                    ('--verify-tag' if mode == 'release' else '--target') not in created[0][0]):
                raise AssertionError(("draft guards", created))
        for mode, selected, overrides, diagnostic in (
            ("unexpected", commit, {}, "draft mode must be release or rehearse"),
            ("rehearse", "2" * 40, {}, "draft checkout differs from resolve SHA"),
            ("release", commit, {"GITHUB_REF": "refs/heads/main"}, "release ref differs from name"),
        ):
            receipt.write_text("")
            expect(run("sh", str(root / "sdlc/scripts/release-workflow"), "draft", mode,
                       "v0.2.0", selected, str(files), env=env | overrides), diagnostic)
            if receipt.read_text():
                raise AssertionError(("invalid draft reached GitHub", mode, receipt.read_text()))
        for document in ("0.2.0.md", "0.2.0-known-gaps.md"):
            page = root / "release-notes" / document
            body = page.read_text()
            page.unlink()
            for mode in ("release", "rehearse"):
                receipt.write_text("")
                expect(run("sh", str(root / "sdlc/scripts/release-workflow"), "draft", mode,
                           "v0.2.0", commit, str(files), env=env), "missing release document release-notes/" + document)
                if receipt.read_text():
                    raise AssertionError("missing notes reached GitHub")
            page.write_text(body)
        manifest = root / "crates/thinkthen/Cargo.toml"
        manifest.write_text(manifest.read_text().replace('version = "0.2.0"', 'version = "9.8.7"'))
        for document in (".md", "-known-gaps.md"):
            (root / "release-notes" / ("0.2.0" + document)).rename(
                root / "release-notes" / ("9.8.7" + document))
        receipt.write_text("")
        expect(run("sh", str(root / "sdlc/scripts/release-workflow"), "draft", "rehearse",
                   "v9.8.7", commit, str(files), env=env), "", success=True)
        created = [json.loads(line) for line in receipt.read_text().splitlines()
                   if json.loads(line)[0][:2] == ['release', 'create']]
        if len(created) != 1 or created[0][1] != f"Rehearsal from {commit}; do not publish\n\n" + expected:
            raise AssertionError(("draft did not load source version documents", created))


def resolve_outputs(commit, version):
    repository = "botassembly/thinkthen"
    candidate = f"rc/{version}-rc.1"
    candidate_branch = f"release/0.2.0-candidate-{commit[:9]}"
    invalid_candidate_branches = tuple("release/0.2.0-candidate-" + suffix for suffix in (
        "", "a" * 8, "a" * 41, "A" * 9, "g" * 9, "a" * 9 + "/fix", "a" * 9 + "\n"))
    good = {"head_sha": commit, "head_branch": candidate, "event": "workflow_dispatch",
            "status": "completed", "conclusion": "success", "path": ".github/workflows/release.yml"}
    page = lambda runs: json.dumps({"total_count": len(runs), "workflow_runs": runs})
    rehearsal = f"v{version}-rehearsal-{commit[:7]}"
    with tempfile.TemporaryDirectory(prefix="thinkthen-resolve-") as temporary:
        root = Path(temporary).resolve()
        receipt = root / "calls.jsonl"
        gh = root / "gh"
        gh.write_text(f"#!{sys.executable}\n" + """import json, os, sys
with open(os.environ['GH_TEST_RECEIPT'], 'a') as log:
    log.write(json.dumps(sys.argv[1:]) + '\\n')
sys.stdout.write(os.environ['GH_TEST_RESPONSE'])
sys.stderr.write('must not expose API diagnostics')
sys.exit(int(os.environ.get('GH_TEST_EXIT', '0')))
""")
        gh.chmod(0o755)
        env = {"PATH": f"{root}:{Path(sys.executable).parent}:/usr/bin:/bin", "HOME": str(root), "LC_ALL": "C",
               "GITHUB_SHA": commit, "GITHUB_REPOSITORY": repository,
               "GH_TEST_RECEIPT": str(receipt), "GH_TEST_RESPONSE": page([good])}
        endpoint = (f"repos/{repository}/actions/workflows/release.yml/runs?head_sha={commit}"
                    "&status=success&event=workflow_dispatch&per_page=100")
        expected_call = ["api", "--method", "GET", "--paginate", "-H",
                         "Accept: application/vnd.github+json", endpoint]

        def check(mode, ref, response=page([good]), exit_code=0, error=None, override=None):
            receipt.write_text("")
            result = run("sh", str(REPO / "sdlc/scripts/release-workflow"), "resolve", mode, ref,
                         env=env | {"GH_TEST_RESPONSE": response, "GH_TEST_EXIT": str(exit_code)}
                         | (override or {}))
            calls = [json.loads(line) for line in receipt.read_text().splitlines()]
            queried = mode == "release" and error not in ("before-query",)
            if calls != ([expected_call] if queried else []):
                raise AssertionError(("resolve query", mode, ref, calls))
            if error is None:
                name = rehearsal if mode == "rehearse" else f"v{version}"
                wanted = f"sha={commit}\nversion={version}\nname={name}\n"
                if result.returncode or result.stdout != wanted or "versions: " not in result.stderr:
                    raise AssertionError(("resolve outputs", mode, result.returncode, result.stdout, result.stderr))
            elif error != "before-query":
                sentence = (f"release-workflow: could not read the rehearsal runs for {commit}\n"
                            if error == "read" else f"release-workflow: no successful rehearsal ran on {commit}; "
                            f"dispatch rehearse mode on an rc/{version}-rc.N tag for that commit first\n")
                if result.returncode != 1 or result.stdout or not result.stderr.endswith(sentence):
                    raise AssertionError(("resolve refusal", result.returncode, result.stdout, result.stderr))
            else:
                if result.returncode != 1 or result.stdout:
                    raise AssertionError(("early resolve refusal", result.returncode, result.stdout, result.stderr))
            return result

        for number in (1, 2, 12):
            check("rehearse", f"refs/tags/rc/{version}-rc.{number}")
        check("rehearse", "refs/heads/main")
        check("rehearse", "refs/heads/ticket/0543-final")
        check("rehearse", f"refs/heads/{candidate_branch}")
        check("release", f"refs/tags/v{version}")
        for branch in ("main", "ticket/0543-final", candidate_branch):
            branch_run = good | {"head_branch": branch,
                                 "path": good["path"] + f"@refs/heads/{branch}"}
            check("release", f"refs/tags/v{version}", page([branch_run]))
            check("release", f"refs/tags/v{version}",
                  page([branch_run | {"head_sha": "0" * 40}]), error="absent")
            check("rehearse", f"refs/heads/{branch}", error="before-query",
                  override={"GITHUB_SHA": "0" * 40})
        for branch in (candidate, f"rc/{version}-rc.12"):
            for prefix in ("", f"{repository}/"):
                for suffix in ("", f"@{branch}", f"@refs/tags/{branch}"):
                    check("release", f"refs/tags/v{version}",
                          page([good | {"head_branch": branch, "path": prefix + good['path'] + suffix}]))
        check("release", f"refs/tags/v{version}", page([good | {"run_attempt": 2}]))
        check("release", f"refs/tags/v{version}", page([]) + "\n" + page([good]))
        for changes in ({"conclusion": "failure"}, {"conclusion": False}, {"conclusion": None},
                        {"head_sha": "0" * 40}, {"head_branch": "v0.1.2"},
                        {"head_branch": "ticket/0543-other"}, {"head_branch": "release/0.1"},
                        {"head_branch": "release/12.34"},
                        {"head_branch": "rc/9.9.9-rc.1"},
                        {"head_branch": f"rc/{version}-rc.0"},
                        {"head_branch": f"rc/{version}-rc.01"},
                        {"head_branch": f"rc/{version}-rc.-1"},
                        {"head_branch": f"rc/{version}-rc.1/fix"},
                        {"head_branch": "refs/tags/v0.2.0"}, {"head_branch": "feature"},
                        {"head_branch": "release/next"}, {"head_branch": None},
                        {"event": "push"}, {"status": "in_progress"},
                        {"path": ".github/workflows/other.yml"}, {"path": None}, {"path": 1},
                        {"path": "other/repository/" + good['path']},
                        {"path": good['path'] + "@refs/tags/v0.2.0"},
                        {"path": good['path'] + f"@refs/heads/{candidate}"},
                        {"path": good['path'] + f"@rc/{version}-rc.2"},
                        {"path": good['path'] + "@release/0.1"},
                        {"path": good['path'] + "@main@main"}):
            check("release", f"refs/tags/v{version}", page([good | changes]), error="absent")
        for branch in invalid_candidate_branches:
            check("release", f"refs/tags/v{version}", page([good | {"head_branch": branch}]), error="absent")
        for missing in good:
            check("release", f"refs/tags/v{version}",
                  page([{key: value for key, value in good.items() if key != missing}]), error="absent")
        check("release", f"refs/tags/v{version}", page([]), error="absent")
        for malformed in ("", "{", "[]", "null", "{}", page([good]) + "{", page([good]) + "garbage",
                          json.dumps({"total_count": True, "workflow_runs": []}),
                          json.dumps({"total_count": -1, "workflow_runs": []}),
                          json.dumps({"total_count": 1, "workflow_runs": [None]})):
            check("release", f"refs/tags/v{version}", malformed, error="read")
        check("release", f"refs/tags/v{version}", page([good]), exit_code=1, error="read")
        candidate_error = f"rehearse must run from an rc/{version}-rc.N tag with positive N"
        for ref in ("refs/heads/ticket/0543-other", "refs/heads/release/0.1", "refs/heads/feature",
                    f"refs/tags/v{version}", "refs/tags/rc/9.9.9-rc.1",
                    f"refs/heads/{candidate}", f"refs/tags/rc/{version}-rc.0",
                    f"refs/tags/rc/{version}-rc.01", f"refs/tags/rc/{version}-rc.-1",
                    f"refs/tags/rc/{version}-rc.", f"refs/tags/rc/{version}-rc.one",
                    f"refs/tags/rc/{version}-rc.1/fix", f"refs/tags/rc/{version}-rc.1\n") + tuple(
                        f"refs/heads/{branch}" for branch in invalid_candidate_branches):
            result = check("rehearse", ref, error="before-query")
            if result.stderr != f"release-workflow: {candidate_error}, got {ref}\n":
                raise AssertionError(("resolve candidate sentence", result.stderr))
        for mode, ref, wanted in (
                ("release", f"refs/tags/{candidate}", "release must run from a v* tag"),
                ("release", "refs/heads/release/0.1", "release must run from a v* tag"),
                ("release", "refs/heads/main", "release must run from a v* tag"),
                ("release", "refs/heads/ticket/0543-final", "release must run from a v* tag"),
                ("release", f"refs/heads/{candidate_branch}", "release must run from a v* tag")):
            result = check(mode, ref, error="before-query")
            if result.stderr != f"release-workflow: {wanted}, got {ref}\n":
                raise AssertionError(("resolve ref sentence", result.stderr))
        result = check("release", f"refs/tags/v{version}", error="before-query",
                       override={"GITHUB_SHA": "0" * 40})
        if result.stderr != "release-workflow: checkout differs from dispatch SHA\n":
            raise AssertionError(("resolve checkout sentence", result.stderr))
        check("rehearse", f"refs/tags/{candidate}", error="before-query",
              override={"GITHUB_SHA": "0" * 40})
        check("release", "refs/tags/v9.9.9", error="before-query")
        check("unknown", "refs/heads/main", error="before-query")

    # Timeout/missing executable proof uses the real helper's public entry point without waiting.
    helper = runpy.run_path(str(REPO / "sdlc/scripts/release-rehearsal.py"))
    historical = good | {"head_sha": "abac3ce61bf1188b40cbc3c2ef0a0589eb247c86",
                         "head_branch": "release/0.1", "id": 37126990511, "workflow_id": 369147892}
    tag_run = good | {"head_sha": "08328c9c04574719b9e93900dd8fa46ad3b645b4",
                      "head_branch": "v0.1.2", "id": 37130570517, "workflow_id": 369147892}
    if (helper['eligible'](historical, historical['head_sha'], repository, version)
            or helper['eligible'](tag_run, tag_run['head_sha'], repository, version)):
        raise AssertionError("historical branch and release-tag runs cannot qualify the candidate")
    for failure in (subprocess.TimeoutExpired("gh", 60), FileNotFoundError("gh")):
        with mock.patch.dict(os.environ, {"GITHUB_SHA": commit, "GITHUB_REPOSITORY": repository}), \
                mock.patch("subprocess.run", side_effect=failure), mock.patch("sys.stderr", new_callable=io.StringIO) as stderr:
            if helper['main'](version) != 1 or stderr.getvalue() != f"release-workflow: could not read the rehearsal runs for {commit}\n":
                raise AssertionError(("helper transport error", stderr.getvalue()))


def tap_formula(version):
    # Pin the former tap's rendered bytes independently of the extracted renderer.
    template = """class Thinkthen < Formula
  desc "Semantic judgments over text"
  homepage "https://thinkthen.dev"
  version "$version"
  license "MIT"

  if OS.mac?
    if Hardware::CPU.arm?
      url "https://github.com/botassembly/thinkthen/releases/download/v$version/thinkthen-$version-aarch64-apple-darwin.tar.gz"
      sha256 "$mac_arm"
    else
      url "https://github.com/botassembly/thinkthen/releases/download/v$version/thinkthen-$version-x86_64-apple-darwin.tar.gz"
      sha256 "$mac_intel"
    end
  elsif Hardware::CPU.arm?
    url "https://github.com/botassembly/thinkthen/releases/download/v$version/thinkthen-$version-aarch64-unknown-linux-musl.tar.gz"
    sha256 "$linux_arm"
  else
    url "https://github.com/botassembly/thinkthen/releases/download/v$version/thinkthen-$version-x86_64-unknown-linux-musl.tar.gz"
    sha256 "$linux_intel"
  end

  def install
    bin.install "thinkthen"
  end

  test do
    assert_match "thinkthen $version", shell_output("#{bin}/thinkthen --version")
  end
end
"""
    with tempfile.TemporaryDirectory(prefix="thinkthen-tap-formula-") as temporary:
        root = Path(temporary).resolve()
        platform = root / "platform"
        hashes = {}
        archives = []
        for target, variable in (("x86_64-unknown-linux-musl", "linux_intel"),
                                 ("aarch64-unknown-linux-musl", "linux_arm"),
                                 ("x86_64-apple-darwin", "mac_intel"),
                                 ("aarch64-apple-darwin", "mac_arm")):
            folder = target.replace("-musl", "-gnu")
            archive = platform / f"platform-{folder}/thinkthen-{version}-{target}.tar.gz"
            archive.parent.mkdir(parents=True)
            archive.write_bytes(f"fixture archive for {target}\n".encode())
            digest = hashlib.sha256(archive.read_bytes()).hexdigest()
            archive.with_name(archive.name + ".sha256").write_text(f"{digest}  {archive.name}\n")
            hashes[variable] = digest
            archives.append(archive)
        output = root / "thinkthen.rb"
        # No key variables, credential files or outbound commands participate.
        env = {"PATH": f"{Path(sys.executable).parent}:/usr/bin:/bin", "HOME": str(root), "LC_ALL": "C"}
        command = ("sh", str(REPO / "sdlc/scripts/release-workflow"), "tap-formula", version, str(platform), str(output))
        result = run(*command, env=env)
        wanted = template.replace("$version", version)
        for variable, digest in hashes.items():
            wanted = wanted.replace(f"${variable}", digest)
        if result.returncode != 0 or result.stdout or result.stderr or output.read_text() != wanted:
            raise AssertionError(("tap formula bytes", result.returncode, result.stdout, result.stderr))
        syntax = run("ruby", "-c", str(output), env=env)
        if syntax.returncode != 0 or syntax.stdout != "Syntax OK\n":
            raise AssertionError(("tap formula syntax", syntax.returncode, syntax.stdout, syntax.stderr))
        archive = archives[0]
        original = archive.read_bytes()
        archive.unlink()
        result = run(*command, env=env)
        sentence = f"release-workflow: tap archive {archive} is missing\n"
        if result.returncode != 1 or result.stdout or result.stderr != sentence:
            raise AssertionError(("missing tap archive", result.returncode, result.stdout, result.stderr))
        archive.write_bytes(b"wrong bytes")
        result = run(*command, env=env)
        sentence = f"release-workflow: tap archive {archive} differs from its checksum\n"
        # The checksum tool may add its own diagnostic before the pinned refusal.
        if result.returncode != 1 or result.stdout or not result.stderr.endswith(sentence):
            raise AssertionError(("bad tap checksum", result.returncode, result.stdout, result.stderr))
        archive.write_bytes(original)
        result = run(*command[:3], "0.0.0", *command[4:], env=env)
        if result.returncode != 1 or result.stdout or result.stderr != "release-workflow: tap version differs from resolved version\n":
            raise AssertionError(("tap version", result.returncode, result.stdout, result.stderr))


def archived_flutter_admission(commit, host):
    # Admission and source identity need no compiler, including on Apple hosts.
    with tempfile.TemporaryDirectory(prefix="thinkthen-flutter-archive-") as temporary:
        base = Path(temporary).resolve()
        source = base / "source"
        source.mkdir()
        archive = base / "source.tar"
        with archive.open("wb") as output:
            subprocess.run(["git", "archive", "--format=tar", "HEAD"], cwd=REPO, stdout=output, check=True)
        subprocess.run(["tar", "-xf", str(archive), "-C", str(source)], check=True)
        env = os.environ | {"THINKTHEN_ARCHIVED_SOURCE_TAR": str(archive),
                            "THINKTHEN_ARCHIVED_SOURCE_COMMIT": commit}
        command = ("sh", str(source / "sdlc/scripts/release-pack"), host, str(base / "out"))
        expect(run(*command, "dart", "flutter", cwd=source, env=env),
               "source wrappers need c in this invocation")
        for part in ("unknown", "csharp", "jvm", "command", "crate", "r", "first-run"):
            expect(run(*command, "c", "dart", "flutter", part, cwd=source, env=env),
                   f"archived source does not permit {part}")
        member = source / "libraries/dart/flutter/lib/thinkthen_flutter.dart"
        original = member.read_bytes()
        for payload in (None, original + b"\n// altered archived Flutter source\n"):
            if payload is None:
                member.unlink()
            else:
                member.write_bytes(payload)
            expect(run(*command, "c", "dart", "flutter", cwd=source, env=env),
                   "source file differs: libraries/dart/flutter/lib/thinkthen_flutter.dart")
        member.write_bytes(original)
        if (base / "out").exists():
            raise AssertionError("archive refusal created package output")
    print("release archive: Flutter source identity and part refusals pass", flush=True)



def c_pack_exit(host, version):
    # Use the existing native archive fixture pattern through the real C pack path.
    with tempfile.TemporaryDirectory(prefix="thinkthen-c-pack-exit-") as temporary:
        base = Path(temporary).resolve()
        native = base / "native/release"
        native.mkdir(parents=True)
        (base / "fixture.c").write_text('int thinkthen_fixture(void) { return 0; }\n')
        subprocess.run(["cc", "-c", "-o", str(base / "fixture.o"), str(base / "fixture.c")], check=True)
        subprocess.run(["ar", "rcs", str(native / "libthinkthen_c.a"), str(base / "fixture.o")], check=True)
        suffix = "dylib" if host.endswith("-apple-darwin") else "so"
        (native / f"libthinkthen_c.{suffix}").write_bytes(b"fixture shared")
        binary = base / "bin"
        binary.mkdir()
        cargo = binary / "cargo"
        cargo.write_text("#!/bin/sh\nexit 0\n")
        cargo.chmod(0o755)
        env = os.environ | {"PATH": str(binary) + os.pathsep + os.environ["PATH"],
                            "CARGO_TARGET_DIR": str(native.parent), "THINKTHEN_FULL_COMPILER": "1"}
        env.pop("CFLAGS", None)
        archive = f"thinkthen-c-{version}-{host}.tar.gz"
        output = base / "success"
        result = run("sh", str(REPO / "sdlc/scripts/release-pack"), host, str(output), "c", env=env)
        expect(result, "", success=True)
        with tarfile.open(output / archive) as packed:
            assert packed.extractfile("./lib/libthinkthen.a").read().startswith(b"!<arch>"), "C archive missing native library"
        cargo.write_text("#!/bin/sh\nexit 42\n")
        output = base / "failure"
        result = run("sh", str(REPO / "sdlc/scripts/release-pack"), host, str(output), "c", env=env)
        diagnostic = f"release-pack: missing archive {output / archive}"
        if result.returncode != 42 or diagnostic not in result.stderr or (output / archive).exists():
            raise AssertionError(("C pack must preserve build exit 42 and name missing archive", result))
    print("release archive: unset CFLAGS C pack and build exit 42 pass", flush=True)


def main():
    commit = subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=REPO, text=True).strip()
    version = next(line.split('"')[1] for line in (REPO / "crates/thinkthen/Cargo.toml").read_text().splitlines()
                   if line.startswith('version = "'))
    resolve_outputs(commit, version)
    tap_formula(version)
    host = subprocess.check_output(["rustc", "-vV"], text=True).split("host: ", 1)[1].splitlines()[0]
    archived_flutter_admission(commit, host)
    if host.endswith("-apple-darwin") or host == "x86_64-unknown-linux-gnu":
        c_pack_exit(host, version)
    if host != "x86_64-unknown-linux-gnu":
        print(f"release archive self-test: skipped synthetic C fixture on {host}")
        return
    with tempfile.TemporaryDirectory(prefix="thinkthen-release-archive-") as temporary:
        base = Path(temporary).resolve()
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
            if path.is_file():
                path.chmod(0o755)  # Model Windows accepting X_OK for every regular data file.
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
        for path in (source / "demos/27-test-with-no-network").rglob("*"):
            if path.is_file():
                path.chmod(0o644)
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
        (base / "fixture.c").write_text(
            'const char *thinkthen_fixture(void) { return "/build/host/.rustup/library/std/src/lib.rs"; }\n')
        subprocess.run(["cc", "-c", "-o", str(base / "fixture.o"), str(base / "fixture.c")], check=True)
        subprocess.run(["ar", "rcs", str(native / "libthinkthen_c.a"), str(base / "fixture.o")], check=True)
        cpp_guard = str(REPO / "libraries/cpp/fixtures/guard.py")
        native_members = base / "native-members.txt"
        native_members.write_text("\n".join(str(native / name) for name in
                                 ("libthinkthen_c.so", "libthinkthen_c.a")) + "\n")
        # Later build output is not part of the captured shipping install.
        (native / "later-output.md").write_bytes(b"/Users/synthetic-private-location")
        expect(run(sys.executable, cpp_guard, str(native), str(native_members)), "PRIVATE_PACKAGE_PASS", success=True)
        private_cpp = base / "private-cpp"
        private_cpp.mkdir()
        for private_bytes in (b"/home/synthetic-private-location", b"/Users/synthetic-private-location",
                              b"/build/home/workspace/source.rs",
                              b"auth.json", b"-----BEGIN PRIVATE KEY-----", b"tt-canary-273",
                              b"/build/home/home/private", b"/build/home/Users/private",
                              b"/build/home/auth.json", b"/build/home/tt-canary-273"):
            (private_cpp / "libthinkthen.a").write_bytes((native / "libthinkthen_c.a").read_bytes() + private_bytes)
            private_members = base / "private-members.txt"
            private_members.write_text(str(private_cpp / "libthinkthen.a") + "\n")
            expect(run(sys.executable, cpp_guard, str(private_cpp), str(private_members)), "private pattern in libthinkthen.a")
        env = os.environ.copy()
        env.update(PATH=str(fake_bin) + os.pathsep + env["PATH"],
                   CARGO_TARGET_DIR=str(native.parent),
                   THINKTHEN_CARGO_CALLS=str(base / "cargo-calls"),
                   THINKTHEN_ARCHIVED_SOURCE_TAR=str(archive),
                   THINKTHEN_ARCHIVED_SOURCE_COMMIT=commit)
        # Keep the held SQL/DataFrame families out of execution; their allowlist is unchanged.
        before = (base / "cargo-calls").read_bytes() if (base / "cargo-calls").exists() else b""
        expect(run("sh", str(source / "sdlc/scripts/release-pack"), host,
                   str(base / "refused-foundation"), "c", "objective-c", cwd=source, env=env),
               "Objective-C Foundation requires an Apple target")
        assert not (base / "refused-foundation").exists()
        assert ((base / "cargo-calls").read_bytes() if (base / "cargo-calls").exists() else b"") == before
        parts = ("c", "go", "cpp", "swift", "zig", "php", "dart", "ada", "cobol")
        expect(run("sh", str(source / "sdlc/scripts/release-pack"), host,
                   str(base / "paired"), *parts, cwd=source, env=env), "", success=True)
        with tarfile.open(next((base / "paired").glob("thinkthen-swift-*.tar.gz"))) as packed:
            assert packed.extractfile(f"./Sources/ThinkThen/Native/{host}/libthinkthen.so").read() == (native / "libthinkthen_c.so").read_bytes()
        with tarfile.open(next((base / "paired").glob("thinkthen-dart-*.tar.gz"))) as packed:
            manifest = json.load(packed.extractfile("./native-assets.json"))
            assert manifest['assets']['linux_x64']['sha256'] == hashlib.sha256((native / 'libthinkthen_c.so').read_bytes()).hexdigest(), 'Dart manifest must identify the packed C bytes'
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
        with tarfile.open(base / "paired" / f"thinkthen-go-{version}-{host}.tar.gz") as go_package, \
                tarfile.open(base / "paired" / f"thinkthen-c-{version}-{host}.tar.gz") as c_package:
            for member in ("include/thinkthen.h", "lib/libthinkthen.a"):
                assert go_package.extractfile(f"./native/{host}/{member}").read() == c_package.extractfile(f"./{member}").read(), "Go bundled native input differs from C package"
        expect(run("sh", str(REPO / "sdlc/scripts/release-go-cpp-pair"), str(base / "paired"),
                   "ada-objective-c-cobol"), "", success=True)
        # Consumer-required examples must survive source package assembly.
        cobol_name = f"thinkthen-cobol-{version}-{host}.tar.gz"
        with tarfile.open(base / "paired" / cobol_name) as packed:
            assert packed.extractfile("./examples/blocking.cob").read() == (source / "libraries/cobol/examples/blocking.cob").read_bytes()
        incomplete = base / "missing-cobol-example"
        shutil.copytree(base / "paired", incomplete)
        damaged = incomplete / cobol_name
        replacement = incomplete / "replacement.tar.gz"
        with tarfile.open(damaged) as packed, tarfile.open(replacement, "w:gz") as output:
            for member in packed:
                if member.name == "./examples/blocking.cob":
                    continue
                payload = packed.extractfile(member) if member.isfile() else None
                output.addfile(member, payload)
        replacement.replace(damaged)
        damaged.with_name(damaged.name + ".sha256").write_text(
            f"{hashlib.sha256(damaged.read_bytes()).hexdigest()}  {damaged.name}\n")
        expect(run("sh", str(REPO / "sdlc/scripts/release-go-cpp-pair"), str(incomplete),
                   "ada-objective-c-cobol"), "source members differ for " + cobol_name)
        print("release-archive: complete COBOL source package passes; missing blocking example refuses", flush=True)
        expect(run("sh", gate, "swift-zig-gate", str(base / "paired"), host, commit), "", success=True)
        for removed in ("CallProjection.swift", "CallResult.swift", None):
            altered = base / ("swift-inventory-" + (removed or "unknown"))
            shutil.copytree(base / "paired", altered)
            swift_name = f"thinkthen-swift-{version}-{host}.tar.gz"
            damaged = altered / swift_name
            replacement = altered / "replacement.tar.gz"
            with tarfile.open(damaged) as packed, tarfile.open(replacement, "w:gz") as output:
                for member in packed:
                    if removed and member.name == "./Sources/ThinkThen/" + removed:
                        continue
                    payload = packed.extractfile(member) if member.isfile() else None
                    output.addfile(member, payload)
                if removed is None:
                    member = tarfile.TarInfo("./Sources/ThinkThen/Unknown.swift")
                    member.size = 1
                    output.addfile(member, io.BytesIO(b"x"))
            replacement.replace(damaged)
            damaged.with_name(damaged.name + ".sha256").write_text(
                f"{hashlib.sha256(damaged.read_bytes()).hexdigest()}  {damaged.name}\n")
            expect(run("sh", str(REPO / "sdlc/scripts/release-go-cpp-pair"), str(altered),
                       "swift-zig"), "source members differ for " + swift_name)
        print("release-archive: complete Swift source package passes; missing or unknown sources refuse", flush=True)
        for private_bytes in (b"/home/synthetic-private-location", b"/Users/synthetic-private-location",
                              b"/build/home/workspace/source.rs",
                              b"tt-canary-273", b"/build/home/home/private",
                              b"/build/home/Users/private", b"/build/home/tt-canary-273"):
            private_zig = base / "private-zig"
            shutil.copytree(base / "paired", private_zig, dirs_exist_ok=True)
            archive_path = next(private_zig.glob("thinkthen-zig-*.tar.gz"))
            replacement = private_zig / "private.tmp"
            with tarfile.open(archive_path, "r:gz") as archive, tarfile.open(replacement, "w:gz") as output:
                for member in archive:
                    payload = archive.extractfile(member).read() if member.isfile() else None
                    if member.name.endswith("/lib/libthinkthen.a"):
                        payload += private_bytes
                        member.size = len(payload)
                    output.addfile(member, io.BytesIO(payload) if payload is not None else None)
            replacement.replace(archive_path)
            digest = hashlib.sha256(archive_path.read_bytes()).hexdigest()
            archive_path.with_name(archive_path.name + ".sha256").write_text(f"{digest}  {archive_path.name}\n")
            private_result = run("sh", gate, "swift-zig-gate", str(private_zig), host, commit)
            if private_result.returncode != 1 or "Zig archive contains private bytes" not in private_result.stderr:
                raise AssertionError(("Zig private native bytes", private_result.returncode, private_result.stderr))
        expect(run("sh", gate, "php-dart-gate", str(base / "paired"), host, commit), "", success=True)
        expect(run("sh", gate, "ada-objc-cobol-gate", str(base / "paired"), host, commit), "", success=True)
        # Exercise the shared Dart/Flutter route with the real builder and fake native outputs.
        flutter_env = env
        flutter_pair = base / "flutter-pair"
        expect(run("sh", str(source / "sdlc/scripts/release-pack"), host, str(flutter_pair),
                   "c", "dart", "flutter", cwd=source, env=flutter_env), "", success=True)
        expect(run("sh", str(REPO / "sdlc/scripts/release-go-cpp-pair"), str(flutter_pair),
                   "dart-flutter"), "", success=True)
        # Repairing membership must still reject missing new inputs and unexpected source.
        for removed in ("./hook/build.dart", "./native-assets.json", "./lib/src/session/client.dart", None):
            planted = base / ("missing-dart-" + (removed.rsplit("/", 1)[-1] if removed else "unexpected"))
            shutil.copytree(base / "paired", planted)
            path = next(planted.glob("thinkthen-dart-*.tar.gz"))
            replacement = planted / "replacement.tar.gz"
            with tarfile.open(path, "r:gz") as archive, tarfile.open(replacement, "w:gz") as output:
                for member in archive:
                    if member.name != removed:
                        output.addfile(member, archive.extractfile(member) if member.isfile() else None)
                if removed is None:
                    member = tarfile.TarInfo("./lib/unexpected.dart")
                    output.addfile(member, io.BytesIO())
            replacement.replace(path)
            path.with_name(path.name + ".sha256").write_text(
                f"{hashlib.sha256(path.read_bytes()).hexdigest()}  {path.name}\n")
            expect(run("sh", gate, "php-dart-gate", str(planted), host, commit),
                   f"source members differ for {path.name}")

        for family, relative in (("swift", "Sources/ThinkThen/OwnedSession.swift"),
                                 ("zig", "src/thinkthen.zig"),
                                 ("php", "autoload.php"),
                                 ("dart", "lib/src/session/client.dart"),
                                 ("ada", "src/thinkthen.ads"),
                                 ("objective-c", "Sources/Foundation/ThinkThen.m"),
                                 ("cobol", "src/tt_session.c")):
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
                           "  destination=$last\n"
                           "  [ ! -d \"$last\" ] || destination=$last/${arg##*/}\n"
                           "  printf 'changed copy\\n' >>\"$destination\"\n"
                           "fi; if [ \"$arg\" = \"$THINKTHEN_PLANT_TWIN\" ]; then\n"
                           "  : >\"$last/$(printf %s \"${arg##*/}\" | tr A-Z a-z)\"\n"
                           "fi; done\n")
        fake_cp.chmod(0o755)
        for family, relative in (("ada", "src/thinkthen.ads"),
                                 ("cobol", "src/tt_session.c"),
                                 ("flutter", "README-PUBLIC.md")):
            source_member = f"libraries/dart/flutter/{relative}" if family == "flutter" else f"libraries/{family}/{relative}"
            copied_env = env | {"THINKTHEN_PLANT_SOURCE": source_member}
            output = base / f"{family}-copied-change"
            selected_parts = ("c", "dart", "flutter") if family == "flutter" else parts
            packed_member = "flutter/README.md" if family == "flutter" else relative
            expect(run("sh", str(source / "sdlc/scripts/release-pack"), host,
                       str(output), *selected_parts, cwd=source, env=copied_env),
                   f"{family} source differs from archived commit: {packed_member}")
            if list(output.glob(f"thinkthen-{family}-*")):
                raise AssertionError(f"changed {family} copied member created wrapper output")
        # The archive writer still refuses names that collide on case-blind Apple volumes.
        output = base / "swift-twin"
        expect(run("sh", str(source / "sdlc/scripts/release-pack"), host, str(output), *parts, cwd=source,
                   env=env | {"THINKTHEN_PLANT_TWIN": "libraries/swift/Sources/ThinkThen/OwnedSession.swift"}),
               "swift holds names that differ only in case: ./sources/thinkthen/ownedsession.swift")
        if list(output.glob("thinkthen-swift-*")):
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
        for family in ("ada", "cobol"):
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
        windows = platform / "platform-x86_64-pc-windows-msvc"
        windows.mkdir()
        binary = bytearray(128)
        binary[:2] = b"MZ"
        struct.pack_into("<I", binary, 60, 64)
        binary[64:68] = b"PE\0\0"
        struct.pack_into("<H", binary, 68, 0x8664)
        struct.pack_into("<H", binary, 86, 2)
        struct.pack_into("<H", binary, 88, 0x20B)
        win_zip = windows / f"thinkthen-{version}-x86_64-pc-windows-msvc.zip"
        with zipfile.ZipFile(win_zip, "w") as output:
            output.writestr("thinkthen.exe", binary)
            output.writestr("README.md", (REPO / "README.md").read_bytes())
        c_zip = C_FIXTURE["create"](windows, version)
        sample = windows / "thinkthen-first-run.tar.gz"
        sample.write_bytes(first_run)
        wheel = windows / f"thinkthen-{version}-cp310-abi3-win_amd64.whl"
        wheel.write_bytes(b"wheel fixture")
        for file in (win_zip, sample, wheel):
            file.with_name(file.name + ".sha256").write_text(
                f"{hashlib.sha256(file.read_bytes()).hexdigest()}  {file.name}\n")
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
        expected_files |= {f"thinkthen-{version}.tgz", f"thinkthen-{version}.tgz.sha256",
                           c_zip.name, c_zip.name + ".sha256", win_zip.name, win_zip.name + ".sha256", sample.name, sample.name + ".sha256",
                           wheel.name, wheel.name + ".sha256"}
        if {file.name for file in collected.iterdir()} != expected_files:
            raise AssertionError("collect omitted or added a selected fixture file")
        original_c = c_zip.read_bytes()
        for plant in ('missing', 'malformed'):
            if plant == 'missing':
                c_zip.unlink()
            else:
                c_zip.write_bytes(b'not ZIP')
                C_FIXTURE['C'].checksum(c_zip)
            refused = base / ('refused-windows-c-' + plant)
            result = run('sh', gate, 'collect', str(platform), str(npm), str(refused))
            expect(result, 'exactly the command ZIP' if plant == 'missing' else 'not a zip file')
            if result.returncode != 1 or refused.exists():
                raise AssertionError('C collection refusal did not precede output creation')
            c_zip.write_bytes(original_c)
            C_FIXTURE['C'].checksum(c_zip)
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
                               ("aarch64-unknown-linux-gnu", "objective-c"),
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
        for family in ("ada", "cobol"):
            script = (REPO / "libraries" / family / "check.sh").read_text()
            if family == "ada":
                # The current Ada gate installs supplied native-bearing archives
                # in native_session.py rather than the obsolete shell dispatcher.
                missing = base / "missing-installed-ada.tar.gz"
                refused = run(sys.executable, str(REPO / "libraries/ada/checks/native_session.py"),
                              env=os.environ | {"THINKTHEN_ARTIFACT": str(missing)})
                expect(refused, "FileNotFoundError")
                if str(missing) not in refused.stderr:
                    raise AssertionError("Ada did not refuse the supplied missing archive")
                continue
            if "import jsonschema" not in script:
                # Generated session consumers no longer use the legacy source
                # grammar. They must still unpack and test the installed package.
                if "installed_unpack" not in script or "THINKTHEN_PARITY_PACKAGE" not in script:
                    raise AssertionError(f"{family} gate does not check the installed package")
                continue
            installed_end = script.index("  exit 0\nfi\n")
            source_start = script.index("unset THINKTHEN_API_KEY", installed_end)
            source_only = script[installed_end:source_start]
            if "command -v node" not in source_only or "import jsonschema" not in source_only:
                raise AssertionError(f"{family} source prerequisites precede installed return")
        # These plants test archive checksums, not installed Swift or sandbox
        # availability. Observe those prerequisite versions through local stubs.
        for name in ("swift", "swiftc", "bwrap"):
            tool = fake_bin / name
            response = ("Swift version 6.4 (swift-6.4-RELEASE)\nTarget: x86_64-unknown-linux-gnu"
                        if name != "bwrap" else "bubblewrap fixture")
            tool.write_text("#!/bin/sh\n[ \"$1\" = --version ] || exit 94\n"
                            + "printf '%s\\n' '" + response + "'\n")
            tool.chmod(0o755)
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

        # Cross the fetch pipeline's real cache-lock child environment before importing.
        docker = fake_bin / "docker"
        docker.write_text("#!/bin/sh\n"
                          "set -eu\n"
                          "for arg do case $arg in *:/work) root=${arg%:/work} ;; "
                          "THINKTHEN_ARCHIVED_SOURCE_COMMIT=*) selected=${arg#*=} ;; "
                          "THINKTHEN_ARCHIVED_SOURCE_TAR=*) tar_path=${arg#*=} ;; "
                          "PYTHONDONTWRITEBYTECODE=*) export \"$arg\" ;; esac; done\n"
                          "python3 \"$root/src/databases/duckdb/tools/cache_lock.py\" \"$root/cache.lock\" "
                          "python3 \"$root/src/databases/duckdb/tools/source_hosts.py\" --help >/dev/null\n"
                          "python3 \"$root/src/sdlc/scripts/release-archive-tree.py\" \"$root/source.tar\" \"$root/src\"\n"
                          "printf '%s\\n' \"$selected\" >\"$root/out/selected-commit\"\n"
                          "printf '%s\\n' \"$tar_path\" >\"$root/out/selected-tar\"\n"
                          "cp \"$root/source.tar\" \"$root/out/source.tar\"\n")
        docker.chmod(0o755)
        launcher_env = os.environ.copy()
        launcher_env.pop("PYTHONDONTWRITEBYTECODE", None)
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
                                      "THINKTHEN_ARTIFACT": str(base / "absent-wrapper")}
        for family in ("go", "cpp"):
            expect(run("sh", str(REPO / f"libraries/{family}/check.sh"), "0", env=installed_env),
                   f"{family}: missing {'matching ' if family == 'cpp' else ''}C archive")
            source_env = installed_env | {"THINKTHEN_ARTIFACT": ""}
            result = run("sh", str(REPO / f"libraries/{family}/check.sh"), "0", env=source_env)
            if family == "cpp":
                # The generated C++ source check invokes Python before building;
                # it propagates the failed generator's exit rather than a legacy
                # jsonschema prerequisite diagnostic.
                if result.returncode != 1:
                    raise AssertionError(("C++ unavailable Python was not refused", result))
            else:
                expect(result, f"{family}: not run: Python jsonschema is unavailable")
    print("release archive self-test: gitless legacy, Go/C++, Swift/Zig and PHP/Dart inputs pass")


if __name__ == "__main__":
    draft_notes()
    main()
