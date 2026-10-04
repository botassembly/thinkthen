"""Public install checks. Gates exercise local fixtures; installs run only by dispatch."""
import hashlib
import json
import os
import posixpath
from pathlib import Path
import re
import subprocess
import sys
import tarfile
import tempfile

REPO = Path(__file__).resolve().parents[2]
QUESTION = "Does this report say what the person did before the problem appeared?"
CHANNELS = ("download", "homebrew", "cargo-install", "cargo-add", "pip", "uv", "npm", "rubygems",
            "nuget", "maven", "pub", "packagist", "go", "r-universe", "c", "sqlite", "duckdb", "postgresql")
VERSION = re.compile(r"(?:0|[1-9][0-9]*)\.(?:0|[1-9][0-9]*)\.(?:0|[1-9][0-9]*)\Z")


class Failure(Exception):
    pass


def validate_version(version):
    if not isinstance(version, str) or not VERSION.fullmatch(version):
        raise Failure("version must be three whole numbers separated by dots")
    return tuple(map(int, version.split('.')))


def check_result(version, installed, reply):
    validate_version(version)
    if installed != version:
        raise Failure(f"installed thinkthen {installed}, wanted {version}")
    if not isinstance(reply, dict) or reply.get("value") is not True:
        raise Failure("replay did not return true")
    facts = reply.get("facts")
    requests = reply.get("requests_sent", facts.get("requests_sent") if isinstance(facts, dict) else None)
    if type(requests) is not int or requests != 0:
        raise Failure("replay must report requests_sent 0")


def check_index(channel, version, listed, binary=True):
    wanted = validate_version(version)
    if channel == "R-universe" and listed is not None:
        present = validate_version(listed)
        if present > wanted:
            raise Failure(f"R-universe no longer offers thinkthen {version}; its binary index lists {listed}")
        if present == wanted:
            if not binary:
                raise Failure(f"R-universe lists thinkthen {version} only as source; a Linux binary is required")
            return
    elif version in (listed or []):
        return
    raise Failure(f"{channel} does not list thinkthen {version} yet; dispatch install-check again later")


def clean_environment(home):
    env = {"PATH": os.environ.get("PATH", "/usr/bin:/bin"), "HOME": str(home), "LANG": "en_US.UTF-8",
           "LC_ALL": "en_US.UTF-8", "XDG_CONFIG_HOME": str(home / "config"),
           "XDG_CACHE_HOME": str(home / "cache"), "XDG_STATE_HOME": str(home / "state"),
           "CARGO_HOME": str(home / "cargo"), "RUSTUP_HOME": os.environ.get("RUSTUP_HOME", str(Path.home() / ".rustup")),
           "UV_CACHE_DIR": str(home / "uv"), "PUB_CACHE": str(home / "pub-cache"),
           "GEM_HOME": str(home / "gems"), "GEM_PATH": str(home / "gems"),
           "NUGET_PACKAGES": str(home / "nuget"), "DOTNET_CLI_HOME": str(home / "dotnet"),
           "DOTNET_CLI_TELEMETRY_OPTOUT": "1", "GOTOOLCHAIN": "local",
           "HOMEBREW_NO_AUTO_UPDATE": "1", "HOMEBREW_NO_ANALYTICS": "1", "GIT_CONFIG_NOSYSTEM": "1",
           "HOMEBREW_NO_INSTALL_CLEANUP": "1", "HOMEBREW_CACHE": str(home / "brew-cache"),
           "npm_config_cache": str(home / "npm-cache"), "R_LIBS_USER": str(home / "r-library")}
    return env


class Check:
    def __init__(self, root, channel, version):
        self.root, self.channel, self.version = root, channel, version
        self.env = clean_environment(root)
        self.sample = root / "sample/thinkthen-first-run"
        self.project = root / "consumer"
        self.project.mkdir()
        self.native = None

    def run(self, *args, cwd=None, input=None):
        result = subprocess.run([str(arg) for arg in args], cwd=cwd or self.project, env=self.env,
                                input=input, text=True, capture_output=True, timeout=1800)
        if result.returncode:
            # Children have only this run's clean environment, with no provider keys.
            raise Failure(f"{self.channel} command failed ({result.returncode}): {result.stderr.strip() or result.stdout.strip()}")
        return result.stdout.strip()

    def fetch(self, url, output):
        output.parent.mkdir(parents=True, exist_ok=True)
        helper = '. "$1"; shift; fetch_url "$@" --proto =https --tlsv1.2'
        self.run("sh", "-c", helper, "install-check-fetch", REPO / "sdlc/scripts/fetch.sh", output, url)
        return output

    def text(self, url, name):
        return self.fetch(url, self.root / name).read_text()

    def release(self, name):
        base = f"https://github.com/botassembly/thinkthen/releases/download/v{self.version}"
        archive = self.fetch(f"{base}/{name}", self.root / name)
        digest = self.text(f"{base}/{name}.sha256", f"{name}.sha256")
        verify_checksum(archive, digest)
        return archive

    def unpack(self, archive, destination):
        destination.mkdir(parents=True)
        extract_archive(archive, destination)
        return destination

    def prepare_sample(self):
        archive = self.release("thinkthen-first-run.tar.gz")
        self.unpack(archive, self.root / "sample")
        if not (self.sample / "report.txt").is_file() or not (self.sample / "recording").is_dir():
            raise Failure("first-run sample is missing its text or recording")
        (self.sample / "question.txt").write_text(QUESTION)
        settings = {"replay": str(self.sample / "recording"), "cache": False}
        (self.root / "settings.json").write_text(json.dumps(settings))
        request = {"decide": QUESTION, "evidence": (self.sample / "report.txt").read_text()}
        (self.root / "request.json").write_text(json.dumps(request))

    def c_archive(self):
        if self.native is None:
            name = f"thinkthen-c-{self.version}-x86_64-unknown-linux-gnu.tar.gz"
            self.native = self.unpack(self.release(name), self.root / "native")
            self.env["LD_LIBRARY_PATH"] = str(self.native / "lib")
            self.env["PKG_CONFIG_PATH"] = str(self.native / "lib/pkgconfig")
        return self.native

    def response(self, *command):
        try:
            return json.loads(self.run(*command))
        except json.JSONDecodeError as error:
            raise Failure(f"{self.channel} consumer did not return one JSON result") from error

    def command_replay(self, command):
        observed = self.run(command, "--version")
        installed = observed.removeprefix("thinkthen ")
        # CLI facts and the answer use separate streams; retain both from the same call.
        result = subprocess.run([str(command), "decide", QUESTION, "--replay", str(self.sample / "recording"),
                                 "--facts"], input=(self.sample / "report.txt").read_text(),
                                env=self.env, cwd=self.project, text=True, capture_output=True, timeout=60)
        if result.returncode != 0:
            raise Failure(f"{self.channel} replay failed ({result.returncode}): {result.stderr.strip()}")
        try:
            facts = next(json.loads(line) for line in result.stderr.splitlines() if line.startswith('{'))
            reply = {"value": json.loads(result.stdout), "requests_sent": facts["requests_sent"]}
        except (ValueError, KeyError, StopIteration) as error:
            raise Failure("command replay is missing its answer or request facts") from error
        return installed, reply


def verify_checksum(archive, text):
    lines = text.splitlines()
    if len(lines) != 1:
        raise Failure(f"checksum for {archive.name} must name exactly one file")
    fields = lines[0].split()
    if len(fields) != 2 or fields[1].lstrip('*') not in (archive.name, f"./{archive.name}"):
        raise Failure(f"checksum for {archive.name} names another file")
    if not re.fullmatch(r"[0-9a-fA-F]{64}", fields[0]) or hashlib.sha256(archive.read_bytes()).hexdigest() != fields[0].lower():
        raise Failure(f"release asset {archive.name} differs from its checksum")


def extract_archive(archive, destination):
    with tarfile.open(archive) as packed:
        entries = {}
        sentence = f"release asset {archive.name} contains an unsafe entry"
        for entry in packed.getmembers():
            path = Path(entry.name)
            name = str(path)
            if path.is_absolute() or '..' in path.parts or name in entries or not (
                    entry.isfile() or entry.isdir() or entry.issym() or entry.islnk()):
                raise Failure(sentence)
            entries[name] = entry
        for name, entry in entries.items():
            # A link may point only to a regular member of this archive. Directory
            # aliases could redirect later extraction, so refuse links in parents.
            if any(str(parent) in entries and (entries[str(parent)].issym() or entries[str(parent)].islnk())
                   for parent in Path(name).parents):
                raise Failure(sentence)
            seen = set()
            while entry.issym() or entry.islnk():
                if name in seen or Path(entry.linkname).is_absolute():
                    raise Failure(sentence)
                seen.add(name)
                hard_link = entry.islnk()
                parent = str(Path(name).parent) if entry.issym() else '.'
                name = posixpath.normpath(posixpath.join(parent, entry.linkname))
                if name == '..' or name.startswith('../') or name not in entries:
                    raise Failure(sentence)
                entry = entries[name]
                if hard_link and not entry.isfile():
                    raise Failure(sentence)
            if seen and not entry.isfile():
                raise Failure(sentence)
        try:
            packed.extractall(destination, filter="data")
        except (tarfile.TarError, OSError) as error:
            raise Failure(sentence) from error


def main(argv=None):
    args = sys.argv[1:] if argv is None else argv
    try:
        if len(args) != 2 or args[0] not in CHANNELS:
            raise Failure("usage: install-check CHANNEL VERSION")
        channel, version = args
        validate_version(version)
        from install_check_channels import install
        from install_check_tools import prepare
        with tempfile.TemporaryDirectory(prefix="thinkthen-install-check-") as owned:
            check = Check(Path(owned), channel, version)
            prepare(check)
            check.prepare_sample()
            installed, reply, proof = install(check)
            check_result(version, installed, reply)
            print(f"install-check: {channel} thinkthen {version}: true, requests_sent 0; version proof: {proof}")
        return 0
    except (Failure, OSError, subprocess.TimeoutExpired) as error:
        print(f"install-check: {error}", file=sys.stderr)
        return 1
