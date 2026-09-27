#!/usr/bin/env python3
"""Check the policies that cargo and clippy cannot check for themselves.

Every check reports and the script keeps going, so one run names every
problem. The standard library is the only import.
"""

from __future__ import annotations

import json
import pathlib
import posixpath
import re
import subprocess
import sys
import tempfile
import tomllib

REPO = pathlib.Path(__file__).resolve().parents[2]
CRATES_IO = "registry+https://github.com/rust-lang/crates.io-index"
ALLOWED_LICENSES = {
    "MIT",
    "Apache-2.0",
    "Unicode-3.0",
    "Unlicense",
}
# `arc-swap`, `csv-core`, and `signal-hook` use the already accepted terms. Three more licenses
# arrive with the TLS stack under ureq and with nothing
# else. Each one is tied to the crates that force it, so the allowance cannot
# quietly cover a crate that lands later. All three are permissive and carry no
# copyleft term, and there is no HTTPS in Rust without them. A crate listed here
# that stops needing its exception fails the check, so the list cannot rot.
# The `polars` feature adds four more, as the root deny.toml records (ticket 0130).
LICENSE_EXCEPTIONS = {
    "ring": {"ISC"},
    "rustls-webpki": {"ISC"},
    "untrusted": {"ISC"},
    "subtle": {"BSD-3-Clause"},
    "webpki-roots": {"CDLA-Permissive-2.0"},
    # Ticket 0130: the `polars` feature's tree, as the root deny.toml admits it.
    "foldhash": {"Zlib"},
    "slotmap": {"Zlib"},
    "xxhash-rust": {"BSL-1.0"},
    "ar_archive_writer": {"Apache-2.0 WITH LLVM-exception"},
}
ACCEPTED_DEPENDENCIES = {
    "thinkthen": {
        "arc-swap", "clap", "csv-core", "polars", "serde", "serde_json", "sha2", "signal-hook",
        "thiserror", "ureq",
    },
    "conformance-backend": {"serde", "serde_json"},
}
ACCEPTED_TARGET_DEPENDENCIES = {"thinkthen": {"nix"}, "conformance-backend": set()}
ACCEPTED_DEV_DEPENDENCIES = {
    "thinkthen": {"proptest", "conformance-backend", "signal-hook"},
    "conformance-backend": set(),
}
# Ticket 0078: the host signal proofs deliver a signal to one worker thread.
ACCEPTED_TARGET_DEV_DEPENDENCIES = {"thinkthen": {"nix"}, "conformance-backend": set()}
# Ticket 0092 rules that a test-only, unpublished member may sit beside the
# crate: the loopback backend every surface's tests start. It is held to the
# same lints, license, size, and dependency tables as the crate.
MEMBERS = {"thinkthen": "crates/thinkthen", "conformance-backend": "conformance/backend"}
MAX_FILE_LINES = 500
# Main held three `rustfmt::skip` attributes when ticket 0088 pinned this count.
MAX_FORMAT_SKIPS = 3
INHERITED = {"workspace": True}

# ADR 0010's clarification of 2026-09-19: other backends will come, so the
# vendor's words live behind one adapter. These are the words that name the
# vendor rather than the judgment: its question type, its field for what an
# option means, its module, its host, and the stem of its model names.
VENDOR_WORDS = ("noul", "criteria", "systemone", "typesafe", "jev")
# The adapters folder, where every vendor word belongs. `adapters.rs` names the
# modules it holds and says which one this build uses, and each adapter's own
# module owns its name, its default address, its default model, and its
# endpoint path.
ADAPTERS = "crates/thinkthen/src/core/adapters.rs"
# Ticket 0020 moved the last of them, so no file outside the adapters folder
# holds a vendor word and the allowance is empty. A file added here would need
# a ticket saying why a vendor word cannot live behind the adapter.
SEAM_ALLOWED: dict[str, set[str]] = {}

ACCEPTED_RUST_LINTS = {
    "missing_debug_implementations": "forbid",
    "missing_docs": "warn",
    "unexpected_cfgs": {
        "level": "forbid",
        "check-cfg": ["cfg(thinkthen_internal_doctest)"],
    },
    "unreachable_pub": "forbid",
    "unsafe_code": "forbid",
}
ACCEPTED_CLIPPY_LINTS = {
    "allow_attributes_without_reason": "deny",
    "cognitive_complexity": "warn",
    "dbg_macro": "deny",
    "disallowed_macros": "allow",
    "disallowed_methods": "allow",
    "disallowed_types": "allow",
    "excessive_nesting": "warn",
    "expect_used": "deny",
    "indexing_slicing": "deny",
    "panic": "deny",
    "print_stderr": "deny",
    "print_stdout": "deny",
    "todo": "deny",
    "too_many_lines": "warn",
    "unimplemented": "deny",
    "unwrap_used": "deny",
    "wildcard_imports": "warn",
}
ACCEPTED_SHARED_CLIPPY = {
    "too-many-lines-threshold": 90,
    "too-many-arguments-threshold": 6,
    "type-complexity-threshold": 180,
    "cognitive-complexity-threshold": 20,
    "excessive-nesting-threshold": 4,
    "enum-variant-size-threshold": 200,
    "allow-expect-in-tests": True,
    "allow-indexing-slicing-in-tests": True,
    "allow-unwrap-in-tests": True,
    "allow-panic-in-tests": True,
    "allow-dbg-in-tests": False,
    "allow-print-in-tests": False,
}
# An overflow in release is a wrong number rather than a stop, and a wrong
# number in a judgment is worse than a stop. A panic ends the process, because
# a tool this small has nothing to unwind to.
ACCEPTED_RELEASE_PROFILE = {"overflow-checks": True, "panic": "unwind"}

ACCEPTED_CRATE_ROOT_ATTRIBUTES = {
    "crates/thinkthen/src/core/mod.rs": (
        "#![forbid(unsafe_code)]",
        "#![forbid(clippy::disallowed_methods, clippy::disallowed_types)]",
        "#![forbid(clippy::disallowed_macros, clippy::indexing_slicing)]",
        "#![forbid(clippy::allow_attributes_without_reason)]",
    ),
    "crates/thinkthen/src/main.rs": ("#![forbid(unsafe_code)]",),
}

BYTES = "thinkthen-core must receive bytes from its caller"
CONTEXT = "thinkthen-core must receive process context from its caller"
SOCKET = "thinkthen-core must not open or resolve a network socket"
CLOCK = "thinkthen-core must receive time values from its caller"
PROCESS = "thinkthen judges and never runs a command"
DYNAMIC_JSON = "wire bodies must decode into typed structs and never into dynamic JSON"

FAILURES: list[str] = []


def fail(check: str, message: str) -> None:
    FAILURES.append(f"{check}: {message}")


def read_toml(relative: str) -> dict:
    try:
        return tomllib.loads((REPO / relative).read_text(encoding="utf-8"))
    except (OSError, tomllib.TOMLDecodeError) as error:
        fail("read", f"cannot parse {relative}: {error}")
        return {}


def banned(prefix: str, names: tuple[str, ...], reason: str) -> list[dict[str, str]]:
    return [{"path": f"{prefix}{name}", "reason": reason} for name in names]


def accepted_core_clippy() -> dict:
    methods = (
        banned("std::fs::", (
            "canonicalize", "copy", "create_dir", "create_dir_all", "exists",
            "hard_link", "metadata", "read", "read_dir", "read_link",
            "read_to_string", "remove_dir", "remove_dir_all", "remove_file",
            "rename", "set_permissions", "set_times", "soft_link",
            "symlink_metadata", "write",
        ), BYTES)
        + banned("std::path::", ("absolute",), BYTES)
        + banned("std::os::unix::fs::", (
            "chown", "chroot", "fchown", "lchown", "mkfifo", "symlink",
        ), BYTES)
        + banned("std::path::Path::", (
            "canonicalize", "exists", "is_dir", "is_file", "is_symlink",
            "metadata", "read_dir", "read_link", "symlink_metadata", "try_exists",
        ), BYTES)
        + banned("std::env::", (
            "args", "args_os", "current_dir", "current_exe", "home_dir",
            "remove_var", "set_current_dir", "set_var", "temp_dir", "var",
            "var_os", "vars", "vars_os",
        ), CONTEXT)
        + banned("std::net::ToSocketAddrs::", ("to_socket_addrs",), SOCKET)
        + banned("std::time::Instant::", ("elapsed", "now"), CLOCK)
        + banned("std::time::SystemTime::", ("elapsed", "now"), CLOCK)
        + banned("serde_json::Value::", (
            "get", "get_mut", "pointer", "pointer_mut", "as_object",
            "as_object_mut", "as_array", "as_array_mut",
        ), DYNAMIC_JSON)
        + banned("serde_json::", ("to_value", "from_value"), DYNAMIC_JSON)
    )
    types = (
        banned("std::fs::", ("DirBuilder", "DirEntry", "File", "OpenOptions", "ReadDir"), BYTES)
        + banned("std::net::", ("TcpListener", "TcpStream", "UdpSocket"), SOCKET)
        + banned("std::os::unix::net::", ("UnixDatagram", "UnixListener", "UnixStream"), SOCKET)
        + banned("std::process::", ("Command",), PROCESS)
        + banned("serde_json::", ("Value", "Map"), DYNAMIC_JSON)
    )
    return ACCEPTED_SHARED_CLIPPY | {
        "disallowed-methods": methods,
        "disallowed-types": types,
        "disallowed-macros": banned("serde_json::", ("json",), DYNAMIC_JSON),
    }


def check_toolchain() -> None:
    toolchain = read_toml("rust-toolchain.toml").get("toolchain")
    if not isinstance(toolchain, dict) or set(toolchain) != {"channel", "profile", "components"}:
        fail("toolchain", "rust-toolchain.toml holds only channel, profile, and components")
        return
    if re.fullmatch(r"[0-9]+\.[0-9]+\.[0-9]+", str(toolchain["channel"])) is None:
        fail("toolchain", "channel names one exact three-part release")
    if toolchain["profile"] != "minimal":
        fail("toolchain", "profile is minimal")
    if sorted(toolchain["components"]) != ["clippy", "rustfmt"]:
        fail("toolchain", "components are exactly clippy and rustfmt")
    if read_toml("rustfmt.toml") != {"style_edition": "2024"}:
        fail("rustfmt", "rustfmt.toml selects the 2024 style edition and nothing else")


def check_workspace() -> None:
    manifest = read_toml("Cargo.toml")
    workspace = manifest.get("workspace", {})
    if workspace.get("members") != list(MEMBERS.values()):
        fail("workspace", "thinkthen and the conformance backend are the workspace members")
    if workspace.get("exclude") != ["conformance/consumer", "libraries", "databases"]:
        fail("workspace", "the consumer and every binding folder are their own workspaces (ADR 0047)")
    if workspace.get("default-members") != ["crates/thinkthen"]:
        fail("workspace", "a plain root build compiles thinkthen alone (ADR 0047)")
    if workspace.get("resolver") != "3":
        fail("workspace", "resolver is 3")
    package = workspace.get("package", {})
    if package.get("edition") != "2024":
        fail("workspace", "edition is 2024")
    if re.fullmatch(r"[0-9]+\.[0-9]+\.[0-9]+", str(package.get("rust-version"))) is None:
        fail("workspace", "rust-version is one exact three-part release")
    if package.get("rust-version") != read_toml("rust-toolchain.toml").get("toolchain", {}).get("channel"):
        fail("workspace", "rust-version matches the pinned toolchain channel")
    if manifest.get("profile", {}).get("release") != ACCEPTED_RELEASE_PROFILE:
        fail("profile", "the release profile matches the accepted copy")
    lints = workspace.get("lints", {})
    if lints.get("rust") != ACCEPTED_RUST_LINTS:
        fail("lints", "the workspace Rust lint table matches the accepted copy")
    if lints.get("clippy") != ACCEPTED_CLIPPY_LINTS:
        fail("lints", "the workspace Clippy lint table matches the accepted copy")


def check_member(name: str) -> dict:
    """Hold one member to the shared lints, license, and dependency tables."""
    manifest = read_toml(f"{MEMBERS[name]}/Cargo.toml")
    package = manifest.get("package", {})
    if manifest.get("lints") != INHERITED:
        fail("lints", f"{name} inherits the workspace lint table")
    for field in ("edition", "rust-version"):
        if package.get(field) != INHERITED:
            fail("workspace", f"{name} inherits the workspace {field}")
    if package.get("publish") is not False:
        fail("workspace", f"{name} is not publishable while the repository is private")
    if package.get("license") != "MIT":
        fail("workspace", f'{name} declares license = "MIT", as ADR 0015 rules')
    if set(manifest.get("dependencies", {})) != ACCEPTED_DEPENDENCIES[name]:
        fail("dependencies", f"{name} declares the accepted direct dependency set")
    target = manifest.get("target", {}).get("cfg(unix)", {}).get("dependencies", {})
    if set(target) != ACCEPTED_TARGET_DEPENDENCIES[name]:
        fail("dependencies", f"{name} declares the accepted target dependency set")
    if set(manifest.get("dev-dependencies", {})) != ACCEPTED_DEV_DEPENDENCIES[name]:
        fail("dependencies", f"{name} declares the accepted development dependency set")
    target_dev = manifest.get("target", {}).get("cfg(unix)", {}).get("dev-dependencies", {})
    if set(target_dev) != ACCEPTED_TARGET_DEV_DEPENDENCIES[name]:
        fail("dependencies", f"{name} declares the accepted target development dependency set")
    return manifest


def check_crates() -> None:
    backend = check_member("conformance-backend")
    if backend.get("features") or backend.get("target"):
        fail("dependencies", "the conformance backend declares no feature and no target table")
    manifest = check_member("thinkthen")
    target = manifest.get("target", {}).get("cfg(unix)", {}).get("dependencies", {})
    # Ticket 0078: engine workers mask host signals, so the library needs nix.
    if target.get("nix") != {
        "version": "0.29",
        "default-features": False,
        "features": ["signal"],
    }:
        fail("dependencies", "nix is a Unix library dependency with only its signal feature")
    target_dev = manifest.get("target", {}).get("cfg(unix)", {}).get("dev-dependencies", {})
    if target_dev.get("nix") != {
        "version": "0.29",
        "default-features": False,
        "features": ["pthread", "signal"],
    }:
        fail("dependencies", "tests add only the pthread feature to nix")
    binary = manifest.get("bin", [])
    if len(binary) != 1 or binary[0].get("required-features") != ["cli"]:
        fail("workspace", "the binary requires the cli feature")
    for failure in feature_failures(manifest):
        fail("dependencies", failure)
    for plant in (
        {"default": ["cli", "polars"]},
        {"polars": ["dep:polars", "dep:clap"]},
    ):
        if not feature_failures({**manifest, "features": {**manifest.get("features", {}), **plant}}):
            fail("dependencies", f"the planted features {plant} are refused")
    dependencies = manifest.get("dependencies", {})
    for plant in (
        {**dependencies, "polars": {**dependencies.get("polars", {}), "optional": False}},
        {**dependencies, "polars": {**dependencies.get("polars", {}), "default-features": True}},
    ):
        if not feature_failures({**manifest, "dependencies": plant}):
            fail("dependencies", "a planted Polars that is not optional, or keeps its defaults, is refused")
    if not feature_failures({**manifest, "dev-dependencies": {**manifest.get("dev-dependencies", {}), "polars-core": "0.55.2"}}):
        fail("dependencies", "a planted polars-core dev-dependency is refused")


def feature_failures(manifest: dict) -> list[str]:
    """Ticket 0130: `thinkthen` compiles no Polars by default."""
    held = []
    target = manifest.get("target", {}).get("cfg(unix)", {}).get("dependencies", {})
    optional = {
        dependency for dependency, specification in
        (manifest.get("dependencies", {}) | target).items()
        if isinstance(specification, dict) and specification.get("optional") is True
    }
    if optional != {"clap", "csv-core", "signal-hook", "polars"}:
        held.append("exactly the command dependencies and polars are optional")
    if manifest.get("features") != {
        "default": ["cli"], "cli": ["dep:clap", "dep:csv-core", "dep:signal-hook"], "polars": ["dep:polars"],
    }:
        held.append("the default cli feature selects only command dependencies, and polars only Polars")
    if manifest.get("dependencies", {}).get("polars", {}).get("default-features") is not False:
        held.append("polars has its default features off")
    if set(manifest.get("dev-dependencies", {})) != ACCEPTED_DEV_DEPENDENCIES["thinkthen"]:
        held.append("thinkthen declares the accepted development dependency set")
    return held


def check_clippy_configs() -> None:
    if read_toml("crates/thinkthen/clippy.toml") != accepted_core_clippy():
        fail("clippy-config", "crates/thinkthen/clippy.toml matches the complete accepted copy")


def lock_versions(relative: str) -> dict[str, set[str]]:
    versions: dict[str, set[str]] = {}
    for package in read_toml(relative).get("package", []):
        versions.setdefault(package["name"], set()).add(package["version"])
    return versions


def check_consumer() -> None:
    """Ticket 0086: the external consumer builds against the root's versions and lints."""
    root, consumer = lock_versions("Cargo.lock"), lock_versions("conformance/consumer/Cargo.lock")
    for name in sorted(set(consumer) - {"consumer", "fork-probe"}):
        if not consumer[name] <= root.get(name, set()):
            fail("consumer", f"conformance/consumer/Cargo.lock pins {name} {sorted(consumer[name])}, "
                 f"and the root lock pins {sorted(root.get(name, set()))}")
    workspace = read_toml("conformance/consumer/Cargo.toml").get("workspace", {})
    lints = read_toml("Cargo.toml").get("workspace", {}).get("lints")
    if workspace.get("lints") != lints:
        fail("consumer", "the consumer workspace lint table equals the root table")
    if read_toml("conformance/consumer/consumer/Cargo.toml").get("lints") != INHERITED:
        fail("consumer", "the consumer crate inherits the root lint table")
    probe = read_toml("conformance/consumer/fork-probe/Cargo.toml").get("lints")
    if lints and probe != {**lints, "rust": {**lints["rust"], "unsafe_code": "deny"}}:
        fail("consumer", "fork-probe uses the root lint table with unsafe_code denied, not forbidden")


# ADR 0047: each binding under `libraries` or `databases` is its own workspace
# over the public API. Its plants copy the first binding, the Rust examples.
BINDING_PLANT_BASE = "libraries/rust"
# A binding's deny.toml is the root file plus its named entries, each reason
# aside. A binding holds a deny.toml only where this table names it.
BINDING_DENY = {
    "databases/duckdb": [("licenses", "exceptions", [{"crate": "zlib-rs", "allow": ["Zlib"]}])],
    # pyo3's build helper needs target-lexicon. Ian approved it on 2026-09-25 (ticket 0105).
    "libraries/python": [("licenses", "exceptions", [
        {"crate": "target-lexicon", "allow": ["Apache-2.0 WITH LLVM-exception"]}])],
    "libraries/r": [("advisories", "ignore", [{"id": "RUSTSEC-2024-0436"}])],
    "databases/postgresql": [("advisories", "ignore", ["RUSTSEC-2021-0127"])],
}
# pgrx's generated code needs `unexpected_cfgs` below forbid (ticket 0111).
BINDING_LINTS = {"databases/postgresql": {"unexpected_cfgs": {
    "level": "deny", "check-cfg": ["cfg(thinkthen_internal_doctest)"]}}}
PLANTED_TEST = '#[test]\nfn planted() {\n    eprintln!("skipped");\n    return;\n}\n'
BINDING_PLANTS = (
    ("publish = true", "Cargo.toml", lambda text: text.replace("publish = false", "publish = true")),
    ("default features", "Cargo.toml", lambda text: text.replace(
        "default-features = false", "default-features = true")),
    ("dependency on another binding", "Cargo.toml", lambda text: text.replace(
        "[dependencies]\n", '[dependencies]\nthinkthen-c = { path = "../c" }\n')),
    ("renamed thinkthen with default features", "Cargo.toml", lambda text: text.replace(
        "[dev-dependencies]\n", '[dev-dependencies]\nengine = { package = "thinkthen", path = "../../crates/thinkthen" }\n')),
    ("patch toward another binding", "Cargo.toml", lambda text: text + '\n[patch.crates-io]\nx = { path = "../c" }\n'),
    ("unsafe outside an FFI module", "src/lib.rs", lambda text: text + "unsafe fn planted() {}\n"),
    ("overflow-checks = false", "Cargo.toml", lambda text: text.replace(
        "overflow-checks = true", "overflow-checks = false")),
    ("second ureq version", "Cargo.lock", lambda text: re.sub(
        r'(name = "ureq"\nversion = ")[^"]+', r"\g<1>0.0.1", text, count=1)),
    ("test that prints skipped and returns", "tests/examples.rs", lambda text: text + PLANTED_TEST),
    ("ignored test", "tests/examples.rs", lambda text: text + "#[test]\n#[ignore]\nfn planted() {}\n"),
    ("deny.toml no check reads", "deny.toml", lambda text: text + "[licenses]\n"),
)


def binding_crates() -> dict[str, str]:
    """The crate folder of each binding whose surfaces.txt line names one, such as R's inside its package."""
    lines = (REPO / "sdlc/surfaces.txt").read_text(encoding="utf-8").splitlines()
    return {fields[0]: f"{fields[0]}/{fields[2]}" for fields in map(str.split, lines)
            if len(fields) == 3 and not fields[0].startswith("#")}


def feature_folders() -> set[str]:
    """Ticket 0130: a `feature` line is a Cargo feature of `thinkthen` and holds no crate."""
    lines = (REPO / "sdlc/surfaces.txt").read_text(encoding="utf-8").splitlines()
    return {fields[0] for fields in map(str.split, lines)
            if len(fields) == 2 and fields[1] == "feature" and not fields[0].startswith("#")}


def crate_failures(crates: dict[str, str]) -> list[str]:
    """A crate folder stays inside its binding folder: no leading `/` and no `..` part."""
    return [f"{name}'s crate folder {crate} leaves the binding folder" for name, crate in crates.items()
            if posixpath.normpath(crate) != crate or not crate.startswith(f"{name}/")]


def binding_files(name: str, crate: str) -> dict[str, str]:
    """One binding's manifest, lock, Clippy settings, and Rust sources, without build output, and its deny.toml."""
    folder = REPO / crate
    files = {name: (folder / name).read_text(encoding="utf-8")
             for name in ("Cargo.toml", "Cargo.lock", "clippy.toml") if (folder / name).is_file()}
    # Every Rust file under the binding folder, so a crate folder hides none.
    for source in (REPO / name).rglob("*.rs"):
        if "target" not in source.relative_to(REPO / name).parts:
            files[posixpath.relpath(source.as_posix(), folder.as_posix())] = source.read_text(encoding="utf-8")
    deny = REPO / name / "deny.toml"
    return {**files, "deny.toml": deny.read_text(encoding="utf-8")} if deny.is_file() else files


def lock_tree(lock: dict) -> set[tuple[str, str]]:
    """The name and version of every package in thinkthen's resolved tree.

    The root lock's tree also holds thinkthen's development dependencies, so it
    is a superset of the normal tree a binding resolves.
    """
    packages = lock.get("package", [])

    def named(spec: str) -> list[tuple[str, str]]:
        name, *version = spec.split()
        return [(package["name"], package["version"]) for package in packages
                if package["name"] == name and version[:1] in ([], [package["version"]])]

    reached: set[tuple[str, str]] = set()
    waiting = named("thinkthen")
    while waiting:
        pair = waiting.pop()
        if pair not in reached:
            reached.add(pair)
            package = next(package for package in packages if (package["name"], package["version"]) == pair)
            waiting += [found for spec in package.get("dependencies", []) for found in named(spec)]
    return reached


def binding_test_failures(relative: str, tokens: list[str], allowed_ignore: str | None = None) -> list[str]:
    """R2-28: no binding test is ignored except one named stress test; no early return."""
    held = []
    allowed_seen = False
    for place in range(len(tokens)):
        attribute = tokens[place:place + 3]
        if attribute == ["#", "[", "ignore"]:
            close = tokens.index("]", place + 3) if "]" in tokens[place + 3:] else len(tokens)
            approved = (relative == POLARS_STRESS and allowed_ignore == POLARS_STRESS_FUNCTION
                        and not allowed_seen
                        and tokens[place - 4:place] == ["#", "[", "test", "]"]
                        and tokens[place + 3:close] == ["="]
                        and tokens[close + 1:close + 3] == ["fn", allowed_ignore])
            if approved:
                allowed_seen = True
            else:
                held.append(f"{relative} ignores a test")
        if attribute != ["#", "[", "test"] or "{" not in tokens[place:]:
            continue
        start = tokens.index("{", place)
        depth, end = 0, start
        for end in range(start, len(tokens)):
            depth += {"{": 1, "}": -1}.get(tokens[end], 0)
            if depth == 0:
                break
        body = tokens[start:end]
        asserted = next((at for at, token in enumerate(body[:-1])
                         if token.startswith(("assert", "debug_assert")) and body[at + 1] == "!"), len(body))
        if "return" in body[:asserted]:
            held.append(f"{relative} has a test that returns before its first assertion")
    if allowed_ignore is not None and not allowed_seen:
        held.append(f"{relative} lacks its named ignored stress test {allowed_ignore}")
    return held


def binding_failures(name: str, files: dict[str, str], crate: str = "") -> list[str]:
    crate = crate or name
    try:
        manifest = tomllib.loads(files.get("Cargo.toml", ""))
        lock = tomllib.loads(files.get("Cargo.lock", ""))
        clippy = tomllib.loads(files.get("clippy.toml", ""))
    except tomllib.TOMLDecodeError as error:
        return [f"{name} holds a manifest, lock, and clippy.toml that parse: {error}"]
    root = read_toml("Cargo.toml").get("workspace", {})
    lints = root.get("lints", {})
    package = manifest.get("package", {})
    held = []
    if package.get("publish") is not False:
        held.append(f"{name} sets publish = false")
    if any(package.get(field) != root.get("package", {}).get(field) for field in ("edition", "rust-version")):
        held.append(f"{name} uses the root edition and rust-version")
    # Every table a dependency can hide in: the package, each target, the
    # workspace's shared table, and each patch source.
    tables = [manifest, *manifest.get("target", {}).values(), manifest.get("workspace", {}),
              *({"patch": table} for table in manifest.get("patch", {}).values())]
    dependencies = [(kind, specification.get("package", dependency) if isinstance(specification, dict)
                     else dependency, specification) for table in tables
                    for kind in ("dependencies", "dev-dependencies", "build-dependencies", "patch")
                    for dependency, specification in table.get(kind, {}).items()]
    if [(kind, specification) for kind, dependency, specification in dependencies
            if dependency == "thinkthen"] != [("dependencies", {
                "path": posixpath.relpath("crates/thinkthen", crate), "default-features": False})]:
        held.append(f"{name} depends on thinkthen once, by path, with default features off")
    for _, dependency, specification in dependencies:
        path = specification.get("path") if isinstance(specification, dict) else None
        reached = posixpath.relpath(posixpath.normpath(posixpath.join(REPO.as_posix(), crate, path)),
                                    REPO.as_posix()) if path else ""
        if reached.split("/")[0] in ("libraries", "databases") and not f"{reached}/".startswith(f"{name}/"):
            held.append(f"{name} depends on another binding through {dependency}")
    if lints and manifest.get("lints") != {**lints, "rust": {
            **lints["rust"], "unsafe_code": "deny", **BINDING_LINTS.get(name, {})}}:
        held.append(f"{name} uses the root lint table with unsafe_code denied, not forbidden")
    if clippy != ACCEPTED_SHARED_CLIPPY:
        held.append(f"{name}/clippy.toml matches the shared thresholds and test allowances")
    if manifest.get("profile", {}).get("release") != ACCEPTED_RELEASE_PROFILE:
        held.append(f"{name} copies the root release profile")
    ours, theirs = lock_tree(lock), lock_tree(tomllib.loads((REPO / "Cargo.lock").read_text(encoding="utf-8")))
    drift = ours - theirs
    if not ours or drift:
        held.append(f"{name}/Cargo.lock resolves thinkthen's tree to the root lock's versions: "
                    f"{sorted(drift) or 'thinkthen is absent'}")
    if "deny.toml" in files:
        held += deny_failures(name, tomllib.loads(files["deny.toml"]))
    for relative, text in files.items():
        if relative.endswith(".rs"):
            tokens = rust_tokens(text)
            if "unsafe" in tokens and posixpath.basename(relative) != "ffi.rs":
                held.append(f"{name}/{relative} holds unsafe outside the binding's FFI module")
            if "unexpected_cfgs" in tokens:
                held.append(f"{name}/{relative} names unexpected_cfgs outside the lint table")
            held += binding_test_failures(f"{name}/{relative}", tokens)
    return held


def deny_failures(name: str, deny: dict) -> list[str]:
    if name not in BINDING_DENY:
        return [f"{name}/deny.toml is one no policy check reads"]
    root = read_toml("deny.toml")
    tables = {(section, key): named for section, key, named in BINDING_DENY[name]}
    for section, key in (("licenses", "exceptions"), ("advisories", "ignore")):
        named = tables.get((section, key), [])
        table, held = deny.get(section, {}), root.get(section, {}).get(key, [])
        entries = [{field: value for field, value in entry.items() if field != "reason"}
                   if isinstance(entry, dict) else entry for entry in table.get(key, [])]
        # A binding may leave out a root entry its tree never meets.
        if [entry for entry in entries if entry not in held] != named:
            return [f"{name}/deny.toml is the root file plus its named {section} {key}"]
        deny = {**deny, section: {**table, key: held}}
    if deny != root:
        return [f"{name}/deny.toml is the root file plus its named entries"]
    return []


def check_bindings() -> None:
    """ADR 0047 and ticket 0093: every binding folder, then one planted failure of each kind."""
    crates = binding_crates()
    for failure in crate_failures(crates):
        fail("binding", failure)
    crates = {name: crate for name, crate in crates.items() if not crate_failures({name: crate})}
    if not all(crate_failures({"libraries/r": crate}) for crate in ("libraries/r/../rust", "libraries/r//tmp/rust")):
        fail("binding", "R's crate folders ../rust and /tmp/rust are refused")
    for folder in sorted(path for path in [*REPO.glob("libraries/*"), *REPO.glob("databases/*")] if path.is_dir()):
        name = folder.relative_to(REPO).as_posix()
        crate = crates.get(name, name)
        if name in feature_folders():
            continue
        if not (REPO / crate / "Cargo.toml").is_file():
            fail("binding", f"{name} holds a Cargo.toml, or its surfaces.txt line names its crate folder")
            continue
        for failure in binding_failures(name, binding_files(name, crate), crate):
            fail("binding", failure)
    for name, tables in BINDING_DENY.items():
        if not (REPO / name / "deny.toml").is_file():
            continue
        deny = read_toml(f"{name}/deny.toml")
        other = {**deny, "bans": {**deny.get("bans", {}), "wildcards": "allow"}}
        seconds = [{**deny, section: {**deny.get(section, {}), key: [*deny.get(section, {}).get(key, []), "planted"]}}
                   for section, key, _ in tables]
        if not all(deny_failures(name, planted) for planted in [other, *seconds]):
            fail("binding", f"{name}/deny.toml with a planted extra entry or another difference is refused")
    if "libraries/r" in crates and (REPO / crates["libraries/r"] / "Cargo.toml").is_file():
        crate = crates["libraries/r"]
        files = binding_files("libraries/r", crate)
        moved = files["Cargo.toml"].replace("../../../../../crates/thinkthen", "../../crates/thinkthen")
        if not binding_failures("libraries/r", {**files, "Cargo.toml": moved}, crate):
            fail("binding", "R's manifest with a thinkthen path from the binding folder is refused")
    if "../tests/examples.rs" not in binding_files(BINDING_PLANT_BASE, f"{BINDING_PLANT_BASE}/src"):
        fail("binding", "Rust files outside the crate folder go unread")
    base = binding_files(BINDING_PLANT_BASE, BINDING_PLANT_BASE)
    for label, relative, plant in BINDING_PLANTS:
        planted = {**base, relative: plant(base.get(relative, ""))}
        if planted[relative] == base.get(relative) or not binding_failures(BINDING_PLANT_BASE, planted):
            fail("binding", f"the planted {label} is refused")


# Ticket 0130: the Rust Polars door is the `polars` feature of `thinkthen`. Its
# tests keep R2-28's scan, and no rung builds every feature.
POLARS_TESTS = "crates/thinkthen/tests/polars"
POLARS_STRESS = f"{POLARS_TESTS}/throttle_equality.rs"
POLARS_STRESS_FUNCTION = "two_hundred_series_records_match_the_slice"
RUNGS = ("install", "lint", "test", "spec", "surfaces", "package")


def check_polars_feature() -> None:
    tests = sorted((REPO / POLARS_TESTS).rglob("*.rs"))
    if not tests:
        fail("polars", f"{POLARS_TESTS} holds the Polars door's tests")
    for path in tests:
        relative = path.relative_to(REPO).as_posix()
        allowed = POLARS_STRESS_FUNCTION if relative == POLARS_STRESS else None
        for failure in binding_test_failures(relative, rust_tokens(path.read_text(encoding="utf-8")), allowed):
            fail("polars", failure)
    if not binding_test_failures("planted.rs", rust_tokens("#[test]\n#[ignore]\nfn planted() {}\n")):
        fail("polars", "an ignored Polars test is refused")
    stress = (REPO / POLARS_STRESS).read_text(encoding="utf-8")
    extra = stress + "\n#[test]\n#[ignore]\nfn planted() { assert!(true); }\n"
    if not binding_test_failures(POLARS_STRESS, rust_tokens(extra), POLARS_STRESS_FUNCTION):
        fail("polars", "another ignored test in the approved file is refused")
    sample = '#[test]\n#[ignore = "stress"]\nfn two_hundred_series_records_match_the_slice() { assert!(true); }\n'
    if not binding_test_failures("planted.rs", rust_tokens(sample), POLARS_STRESS_FUNCTION):
        fail("polars", "the approved name in another file is refused")
    if not binding_test_failures(POLARS_STRESS, rust_tokens(sample.replace(POLARS_STRESS_FUNCTION, "planted")), POLARS_STRESS_FUNCTION):
        fail("polars", "another ignored name in the approved file is refused")
    for rung in RUNGS:
        if rung_failures(rung, (REPO / "sdlc/scripts" / rung).read_text(encoding="utf-8")):
            fail("polars", f"sdlc/scripts/{rung} builds with every feature, and Polars belongs to its lane")
    if not rung_failures("test", "cargo test --locked --workspace --all-targets --all-features\n"):
        fail("polars", "a planted --all-features in a rung is refused")


def rung_failures(rung: str, text: str) -> list[str]:
    return [f"{rung} passes --all-features"] if "--all-features" in text else []


# Ticket 0111 item 15: nothing widens the PostgreSQL crate's check-cfg, and no
# script restarts the server, because a restart passes the caller's environment.
POSTGRESQL = "databases/postgresql"


def postgresql_failures(files: dict[str, str], scripts: dict[str, str], configs: list[str]) -> list[str]:
    try:
        build = "build" in tomllib.loads(files.get("Cargo.toml", "")).get("package", {})
    except tomllib.TOMLDecodeError:
        build = True
    held = [f"{POSTGRESQL} has a build script, which can widen check-cfg"] if "build.rs" in files or build else []
    held += [f"{config} can change the lints cargo applies" for config in configs]
    for relative, text in scripts.items():
        # Comments go first, so a comment ending in a backslash hides no real line.
        code = "\n".join(line for line in text.splitlines() if not line.lstrip().startswith("#")).replace("\\\n", " ")
        if re.search(r"\bpg_ctl\b[^\n;&|]*\brestart\b", code):
            held.append(f"{POSTGRESQL}/{relative} runs pg_ctl restart")
    return held


def check_postgresql_binding() -> None:
    folder = REPO / POSTGRESQL
    if not folder.is_dir():
        return
    files = binding_files(POSTGRESQL, POSTGRESQL)
    scripts = {script.relative_to(folder).as_posix(): script.read_text(encoding="utf-8")
               for script in folder.rglob("*.sh") if "target" not in script.relative_to(folder).parts}
    configs = [f"{place}/.cargo/{name}" for place in (POSTGRESQL, "databases", ".")
               for name in ("config", "config.toml") if (REPO / place / ".cargo" / name).exists()]
    for failure in postgresql_failures(files, scripts, configs):
        fail("binding", failure)
    restart = {**scripts, "check.sh": scripts.get("check.sh", "") + '"$BIN/pg_ctl" -D "$DATA" restart\n'}
    split = {**scripts, "check.sh": scripts.get("check.sh", "") + '"$BIN/pg_ctl" -D "$DATA" \\\n  restart\n'}
    hidden = {**scripts, "check.sh": scripts.get("check.sh", "") + '# a comment \\\n"$BIN/pg_ctl" -D "$DATA" restart\n'}
    keyed = files.get("Cargo.toml", "").replace("[package]\n", '[package]\nbuild = "planted.rs"\n', 1)
    warm = files.get("src/warm.rs", "") + '#[cfg_attr(test, allow(unexpected_cfgs, reason = "planted"))]\nfn planted() {}\n'
    forbid = files.get("Cargo.toml", "").replace('unexpected_cfgs = { level = "deny"', 'unexpected_cfgs = { level = "allow"')
    rust = binding_files(BINDING_PLANT_BASE, BINDING_PLANT_BASE)
    denied = rust["Cargo.toml"].replace('unexpected_cfgs = { level = "forbid"', 'unexpected_cfgs = { level = "deny"')
    if not (all(postgresql_failures(*plant) for plant in [
            ({**files, "build.rs": ""}, scripts, []), ({**files, "Cargo.toml": keyed}, scripts, []),
            (files, restart, []), (files, split, []), (files, hidden, []), (files, scripts, [f"{POSTGRESQL}/.cargo/config"])])
            and binding_failures(POSTGRESQL, {**files, "src/warm.rs": warm})
            and binding_failures(POSTGRESQL, {**files, "Cargo.toml": forbid})
            and binding_failures(BINDING_PLANT_BASE, {**rust, "Cargo.toml": denied})):
        fail("binding", "a planted build.rs, build key, cargo config, pg_ctl restart, unexpected_cfgs allow, or lint level is refused")


CORE_ALLOWED_DEPENDENCIES = {"serde", "serde_json", "sha2", "thiserror"}
CORE_PROHIBITED_PATHS = (
    ("std", "fs"),
    ("std", "env"),
    ("std", "net"),
    ("std", "process"),
    ("std", "time", "Instant", "now"),
    ("std", "time", "SystemTime", "now"),
)


def rust_char_end(text: str, quote: int) -> int | None:
    """Return the end of a Rust character literal, or None for a lifetime."""
    place = quote + 1
    if place >= len(text) or text[place] in "\r\n'":
        return None
    if text[place] != "\\":
        place += 1
    elif place + 1 >= len(text):
        return None
    elif text[place + 1] == "x":
        place += 4
    elif text[place + 1:place + 3] == "u{":
        close = text.find("}", place + 3)
        if close < 0:
            return None
        place = close + 1
    else:
        place += 2
    return place + 1 if place < len(text) and text[place] == "'" else None


def rust_tokens(text: str) -> list[str]:
    """Lex the Rust tokens policy needs, skipping comments and every literal."""
    tokens = []
    place = 0
    while place < len(text):
        if text.startswith("//", place):
            end = text.find("\n", place + 2)
            place = len(text) if end < 0 else end + 1
            continue
        if text.startswith("/*", place):
            depth = 1
            place += 2
            while place < len(text) and depth:
                if text.startswith("/*", place):
                    depth += 1
                    place += 2
                elif text.startswith("*/", place):
                    depth -= 1
                    place += 2
                else:
                    place += 1
            continue
        raw = re.match(r"(?:br|cr|r)(#+)?\"", text[place:])
        if raw:
            hashes = raw.group(1) or ""
            place += raw.end()
            end = text.find('"' + hashes, place)
            place = len(text) if end < 0 else end + len(hashes) + 1
            continue
        prefix = 1 if text.startswith(("b\"", "c\"", "b'"), place) else 0
        quote = text[place + prefix] if place + prefix < len(text) else ""
        if quote == '"':
            end = place + prefix + 1
            while end < len(text):
                if text[end] == "\\":
                    end += 2
                elif text[end] == quote:
                    end += 1
                    break
                else:
                    end += 1
            place = end
            continue
        if quote == "'":
            end = rust_char_end(text, place + prefix)
            if end is not None:
                place = end
                continue
        if text.startswith("r#", place) and place + 2 < len(text) and (
                text[place + 2].isalpha() or text[place + 2] == "_"):
            place += 2
        if text[place].isalpha() or text[place] == "_":
            end = place + 1
            while end < len(text) and (text[end].isalnum() or text[end] == "_"):
                end += 1
            tokens.append(text[place:end])
            place = end
            continue
        if text.startswith("::", place):
            tokens.append("::")
            place += 2
            continue
        if not text[place].isspace():
            tokens.append(text[place])
        place += 1
    return tokens


def parse_use_tree(tokens: list[str], place: int = 0,
                   prefix: tuple[str, ...] = ()) -> tuple[list[tuple[tuple[str, ...], str | None]], int]:
    """Parse one tokenized use tree into imported paths and optional aliases."""
    paths = []
    if place < len(tokens) and tokens[place] == "{":
        place += 1
        while place < len(tokens) and tokens[place] != "}":
            branch, place = parse_use_tree(tokens, place, prefix)
            paths.extend(branch)
            if place < len(tokens) and tokens[place] == ",":
                place += 1
        return paths, place + int(place < len(tokens) and tokens[place] == "}")
    if place < len(tokens) and tokens[place] == "::":
        place += 1
    segments = []
    while place < len(tokens) and tokens[place] not in {"{", "}", ",", ";", "as"}:
        if tokens[place] != "::":
            segments.append(tokens[place])
        place += 1
        if place < len(tokens) and tokens[place] == "::":
            place += 1
            if place < len(tokens) and tokens[place] == "{":
                return parse_use_tree(tokens, place, prefix + tuple(segments))
        else:
            break
    alias = None
    if place < len(tokens) and tokens[place] == "as":
        alias = tokens[place + 1] if place + 1 < len(tokens) else ""
        place += 2
    path = prefix + tuple(segments)
    if path and path[-1] == "self" and prefix:
        path = path[:-1]
    return [(path, alias)], place


def rust_use_paths(tokens: list[str]) -> list[tuple[tuple[str, ...], str | None]]:
    """Collect paths from every use declaration."""
    paths = []
    place = 0
    while place < len(tokens):
        if tokens[place] != "use":
            place += 1
            continue
        end = place + 1
        depth = 0
        while end < len(tokens):
            depth += int(tokens[end] == "{") - int(tokens[end] == "}")
            if tokens[end] == ";" and depth == 0:
                break
            end += 1
        tree, _ = parse_use_tree(tokens[place + 1:end])
        paths.extend(tree)
        place = end + 1
    return paths


def extern_crates(tokens: list[str]) -> list[tuple[str, str | None]]:
    """Collect `extern crate` roots and aliases."""
    crates = []
    for place in range(len(tokens) - 2):
        if tokens[place:place + 2] != ["extern", "crate"]:
            continue
        root = tokens[place + 2]
        alias = (
            tokens[place + 4]
            if place + 4 < len(tokens) and tokens[place + 3] == "as"
            else None
        )
        crates.append((root, alias))
    return crates


def token_path_at(tokens: list[str], place: int, path: tuple[str, ...]) -> bool:
    """Match one normalized path at a token boundary."""
    expected = []
    for part in path:
        if expected:
            expected.append("::")
        expected.append(part)
    return tokens[place:place + len(expected)] == expected


def direct_root_references(tokens: list[str]) -> set[str]:
    """Find engine or cli reached directly from crate or an ancestor."""
    held = set()
    for place, token in enumerate(tokens):
        if token == "crate" and place + 2 < len(tokens) and tokens[place + 1] == "::":
            if tokens[place + 2] in {"engine", "cli"}:
                held.add(tokens[place + 2])
        if token != "super":
            continue
        end = place
        while end + 2 < len(tokens) and tokens[end + 1:end + 3] == ["::", "super"]:
            end += 2
        if end + 2 < len(tokens) and tokens[end + 1] == "::" and tokens[end + 2] in {"engine", "cli"}:
            held.add(tokens[end + 2])
    return held


def aliases_outer_root(path: tuple[str, ...], alias: str | None) -> bool:
    """Say whether one use alias can later hide a reverse reference."""
    return alias is not None and (path == ("crate",) or (path and set(path) == {"super"}))


def imports_outer_glob(path: tuple[str, ...]) -> bool:
    """Say whether a glob can import names from the crate root or an ancestor."""
    return len(path) > 1 and path[-1] == "*" and path[0] in {"crate", "super"}


def core_policy_failures(text: str) -> list[str]:
    tokens = rust_tokens(text)
    held = []
    for path in CORE_PROHIBITED_PATHS:
        if any(token_path_at(tokens, place, path) for place in range(len(tokens))):
            held.append("::".join(path))
    for module in direct_root_references(tokens):
        held.append(f"reverse reference to {module}")
    imports = rust_use_paths(tokens)
    for path, alias in imports:
        if aliases_outer_root(path, alias):
            held.append("alias of the crate root or an ancestor")
        if imports_outer_glob(path):
            held.append("glob import from the crate root or an ancestor")
        if path[:1] == ("crate",) and len(path) > 1 and path[1] in {"engine", "cli"}:
            held.append(f"reverse import of {path[1]}")
        if path and set(path) == {"super"}:
            continue
        supers = 0
        while supers < len(path) and path[supers] == "super":
            supers += 1
        if supers and supers < len(path) and path[supers] in {"engine", "cli"}:
            held.append(f"reverse import of {path[supers]}")
    if any(root == "self" and alias is not None for root, alias in extern_crates(tokens)):
        held.append("alias of the crate root or an ancestor")
    return sorted(set(held))


def dependency_roots(dependencies: dict) -> dict[str, str]:
    """Map each Cargo dependency's Rust path to its canonical package name."""
    roots = {}
    for alias, specification in dependencies.items():
        package = specification.get("package", alias) if isinstance(specification, dict) else alias
        roots[alias.replace("-", "_")] = package
    return roots


def core_dependency_failures(text: str, dependencies: dict) -> list[str]:
    """Name package dependencies referenced by core but absent from its allowlist."""
    tokens = rust_tokens(text)
    imported_roots = {path[0] for path, _ in rust_use_paths(tokens) if path}
    imported_roots.update(root for root, _ in extern_crates(tokens))
    held = set()
    for root, package in dependency_roots(dependencies).items():
        direct = any(
            tokens[place:place + 2] == [root, "::"] for place in range(len(tokens) - 1)
        )
        if package not in CORE_ALLOWED_DEPENDENCIES and (root in imported_roots or direct):
            held.add(package)
    return sorted(held)


def check_core_policy() -> None:
    core = REPO / "crates/thinkthen/src/core"
    manifest = read_toml("crates/thinkthen/Cargo.toml")
    dependencies = manifest.get("dependencies", {})
    for source in sorted(core.rglob("*.rs")):
        text = source.read_text(encoding="utf-8")
        held = core_policy_failures(text)
        if held:
            fail("core", f"{source.relative_to(REPO)} uses prohibited paths {held}")
        outer = core_dependency_failures(text, dependencies)
        if outer:
            fail("core", f"{source.relative_to(REPO)} uses outer dependencies {outer}")
    canonical = set(dependency_roots(dependencies).values())
    if not CORE_ALLOWED_DEPENDENCIES <= canonical:
        fail("core", "the package retains every accepted core dependency")
    policy_plants = (
        "std::fs::read(path)",
        "crate::engine::request",
        "crate::r#engine::request",
        "use crate::engine as e;",
        "use crate::{engine as e};",
        "use crate::{core::Answer, engine::{self as e}};",
        "use crate::{\n    cli::{self as command},\n};",
        "use super::super::engine as e;",
        "super::super::cli::entry();",
        "crate::cli::entry",
        "use crate::{cli as command};",
        "use crate::{self as root};",
        "use crate as root;",
        "use {crate as root};",
        "use {crate::{self as root}};",
        "use super::super::{self as root};",
        "use super::super as root;",
        "use {super::super as root};",
        "use {super::super::{self as root}};",
        "use super as parent;",
        "extern crate self as root;",
        "use crate::*; engine::request();",
        "use super::*; engine::request();",
        "use {crate::*}; engine::request();",
        "use crate::{*}; engine::request();",
        "use {crate::{*}}; engine::request();",
        "use super::super::*; cli::entry();",
        "use {super::{*}}; engine::request();",
        "use crate::{core::*, *}; engine::request();",
        "use /* root */ crate /* separator */ :: {\n    *\n}; engine::request();",
        "use /* outer /* nested */ comment */ super::*; engine::request();",
        "/* use crate::cli; /* crate::engine */ */ crate::engine::request();",
    )
    if any(not core_policy_failures(plant) for plant in policy_plants):
        fail("core", "the planted API and reverse-reference violations are refused")
    policy_controls = (
        "// use crate::engine as hidden;",
        "/* use crate::cli; /* crate::engine */ */ use crate::core::Answer;",
        'const EXAMPLE: &str = "use crate::engine as hidden;";',
        'const EXAMPLE: &str = r###"super::super::cli::entry"###;',
        "const MARKER: char = 'e'; use super::Answer;",
        "fn borrow<'a, 'b>(left: &'a str, right: &'b str) {}",
        "use crate::{core::Answer, engine_value as value};",
        "use self::*;",
        "use self::{*};",
        "use serde::*;",
        "use {serde_json::*, sha2::*};",
        'const EXAMPLE: &str = r###"use crate::*; engine::request();"###;',
    )
    if any(core_policy_failures(control) for control in policy_controls):
        fail("core", "comments, literals, lifetimes, and internal imports remain allowed")
    dependency_plants = (
        (dependencies, "use clap::Parser;", ["clap"]),
        (dependencies, "use clap as parser;", ["clap"]),
        (dependencies, "use {clap as parser};", ["clap"]),
        (dependencies, "use {clap::{self as parser, Parser}};", ["clap"]),
        (dependencies, "use {\n    r#clap::{Parser},\n};", ["clap"]),
        (dependencies, "extern crate clap as parser;", ["clap"]),
        (dependencies, "clap::Parser::parse();", ["clap"]),
        (dependencies, "use {csv_core::{Reader as CsvReader}};", ["csv-core"]),
        (dependencies, "use csv_core as records;", ["csv-core"]),
        (dependencies, "use ureq as transport;", ["ureq"]),
        (dependencies, "use {ureq as transport};", ["ureq"]),
        (dependencies, "r#ureq::Agent::new_with_defaults();", ["ureq"]),
        ({"transport": {"package": "ureq"}}, "transport::Agent::new_with_defaults()", ["ureq"]),
        ({"json": {"package": "serde_json"}}, "json::from_str(body)", []),
        (dependencies, "use {serde::{Deserialize as Decode}, serde_json as json};", []),
        (dependencies, "use {serde::{self as data, Deserialize}, sha2 as digest};", []),
        (dependencies, "use {serde::*, r#serde_json::*, sha2::*};", []),
        (dependencies, "extern crate thiserror as errors;", []),
        (dependencies, "// clap::Parser\n/* ureq::Agent /* csv_core */ */ serde::Serialize;", []),
        (dependencies, 'const TEXT: &str = r#"clap::Parser"#;', []),
        (dependencies, "use crate::{core::Answer, engine_value as value};", []),
    )
    if any(core_dependency_failures(text, held) != expected
           for held, text, expected in dependency_plants):
        fail("core", "the outer-dependency and alias plants are refused")


# Ticket 0083: the transform catalog prints embedded bytes and owns nothing
# else. It names no file, environment, network, clock, process, thread, or
# signal API, reaches no other command module, and uses only `clap` outside
# the standard library.
CATALOG = "crates/thinkthen/src/cli/transform.rs"
CATALOG_BANNED_WORDS = {
    "fs", "env", "net", "time", "process", "thread", "os", "stdin", "Stdin",
    "Command", "File", "OpenOptions", "TcpStream", "UdpSocket", "Instant", "SystemTime",
    "engine", "edge", "cache", "interrupt", "usage", "recording", "Environment", "args",
    "include_str", "env_var", "option_env", "nix", "ureq", "signal_hook", "libc", "csv_core",
}
CATALOG_CRATE_PATHS = {("crate", "failure", "Failure")}
CATALOG_SUPER_PATHS = {("super", "CATALOG"), ("super", "lookup")}
CATALOG_ROOTS = {"std", "clap", "crate", "super", "self"}


def catalog_policy_failures(text: str, banned: set[str] = CATALOG_BANNED_WORDS,
                            crate_paths: set[tuple[str, ...]] = CATALOG_CRATE_PATHS,
                            super_paths: set[tuple[str, ...]] = CATALOG_SUPER_PATHS,
                            prefix: bool = False) -> list[str]:
    """Name every capability or owner a read-only command source reaches.

    A path is allowed when it is an allowed path, or with `prefix` when one is its prefix.
    """
    tokens = rust_tokens(text)
    held = {f"names {token}" for token in tokens if token in banned}

    def allowed(path: tuple[str, ...], prefixes: set[tuple[str, ...]]) -> bool:
        if not prefix:
            return path in prefixes
        return any(path[:len(held)] == held for held in prefixes)

    for path, alias in rust_use_paths(tokens):
        if aliases_outer_root(path, alias) or imports_outer_glob(path):
            held.add("aliases or globs an outer module")
        elif path[:1] == ("crate",) and not allowed(path, crate_paths):
            held.add("uses " + "::".join(path))
        elif path[:1] == ("super",) and not allowed(path, super_paths):
            held.add("uses " + "::".join(path))
        elif path and path[0] not in CATALOG_ROOTS:
            held.add("uses " + "::".join(path))
    for place, token in enumerate(tokens[:-1]):
        if token in {"crate", "super"} and tokens[place + 1] == "::" and not any(
                token_path_at(tokens, place, allowed) for allowed in crate_paths):
            if not (token == "super" and tokens[place - 1:place] == ["use"]):
                held.add(f"reaches {token}::{tokens[place + 2] if place + 2 < len(tokens) else ''}")
    if extern_crates(tokens):
        held.add("declares an extern crate")
    return sorted(held)


def check_catalog_policy() -> None:
    held = catalog_policy_failures((REPO / CATALOG).read_text(encoding="utf-8"))
    if held:
        fail("catalog", f"{CATALOG} {held}")
    plants = (
        'std::fs::read("band.jq")',
        "use std::fs::File;",
        'std::env::var("THINKTHEN_API_KEY")',
        "use std::env as e;",
        'std::net::TcpStream::connect("127.0.0.1:1")',
        "std::time::Instant::now()",
        "std::time::SystemTime::now()",
        'std::process::Command::new("jq")',
        'use std::process::Command; Command::new("/usr/bin/jq").status();',
        'std::os::unix::process::CommandExt::exec(&mut command);',
        'nix::unistd::execv(path, &[])',
        "std::io::stdin().lock()",
        "crate::engine::request()",
        "use crate::engine::cache_prune as prune;",
        "crate::cli::edge::Environment::read()",
        "use super::edge::Environment;",
        "super::interrupt::activate(&mut environment)",
        "use super::cache;",
        "crate::cli::find::run(arguments, environment, input, writer)",
        "use crate::*; engine::request();",
        "use super::*;",
        "use crate as root;",
        "extern crate self as root;",
        'include_str!("/etc/passwd")',
        "ureq::get(url)",
        "use crate::failure::Failure::Output;",
        "use super::CATALOG::x;",
        "use super::lookup::inner;",
    )
    for plant in plants:
        if not catalog_policy_failures(plant):
            fail("catalog", f"the planted catalog violation {plant!r} is refused")
    controls = (
        "use std::io::{ErrorKind, Write};",
        "use clap::{Args, Subcommand};",
        "use crate::failure::Failure;",
        "use super::{CATALOG, lookup};",
        'const NOTE: &str = "std::fs::read and crate::engine stay text";',
        "// std::process::Command in a comment",
        'include_bytes!("../../transforms/band.jq")',
    )
    for control in controls:
        if catalog_policy_failures(control):
            fail("catalog", f"the catalog control {control!r} stays allowed")


# Tickets 0113 and 0114: audit and diff read the paths they are handed, or standard input, and
# write standard output, and standard error for a failure. diff also writes its warnings on
# standard error. Each may open a file, and nothing else the catalog refuses. A token check
# cannot prove which paths it opens; review checks that.
MEASURE = (
    "crates/thinkthen/src/cli/measure.rs",
    "crates/thinkthen/src/cli/audit.rs",
    "crates/thinkthen/src/cli/diff.rs",
    "crates/thinkthen/src/cli/audit/write.rs",
)
# Ticket 0125: `audit --write` writes the one file it names, and only here.
MEASURE_WRITER = "crates/thinkthen/src/cli/audit/write.rs"
MEASURE_COMMANDS = ("Audit", "Diff")
MEASURE_BANNED_WORDS = CATALOG_BANNED_WORDS - {"fs", "File", "stdin", "Stdin"} | {"DirBuilder"}
MEASURE_WRITES = {
    "create", "create_new", "options", "create_dir", "create_dir_all", "remove_file",
    "remove_dir", "remove_dir_all", "rename", "copy", "set_permissions", "write", "hard_link",
    "soft_link", "symlink",
}
MEASURE_CRATE_PATHS = {("crate", "core"), ("crate", "cli", "measure"), ("crate", "failure", "Failure")}
ROUTER = "crates/thinkthen/src/cli/mod.rs"


def measure_policy_failures(text: str, writer: bool = False) -> list[str]:
    """Name every capability a measuring command reaches beyond reading its inputs.

    The writer may call `std::fs::write` by its full path, and nothing else that writes.
    """
    held = set(catalog_policy_failures(text, MEASURE_BANNED_WORDS, MEASURE_CRATE_PATHS, set(),
                                       prefix=True))
    tokens = rust_tokens(text)
    for place in range(len(tokens) - 2):
        if tokens[place] in {"fs", "File"} and tokens[place + 1] == "::" \
                and tokens[place + 2] in MEASURE_WRITES:
            if writer and tokens[place - 2:place + 3] == ["std", "::", "fs", "::", "write"]:
                continue
            held.add(f"writes through {tokens[place]}::{tokens[place + 2]}")
    for path, alias in rust_use_paths(tokens):
        if path[:2] == ("std", "fs") and (set(path[2:]) & MEASURE_WRITES or alias or "*" in path):
            held.add("uses " + "::".join(path))
    return sorted(held)


def route_failures(text: str) -> list[str]:
    """Refuse a router that reads the environment before a measuring command returns."""
    tokens = rust_tokens(text)

    def first(path: tuple[str, ...]) -> int | None:
        return next((place for place in range(len(tokens))
                     if token_path_at(tokens, place, path)), None)

    read = first(("Environment", "read"))
    held = []
    for command in MEASURE_COMMANDS:
        found = first(("Command", command))
        if found is None or read is None or found > read:
            held.append(f"{command} returns after Environment::read")
    return held


def check_measure_policy() -> None:
    for relative in MEASURE:
        held = measure_policy_failures((REPO / relative).read_text(encoding="utf-8"),
                                       relative == MEASURE_WRITER)
        if held:
            fail("measure", f"{relative} {held}")
    for held in route_failures((REPO / ROUTER).read_text(encoding="utf-8")):
        fail("measure", f"{ROUTER}: {held}")
    plants = (
        'std::env::var("THINKTHEN_API_KEY")',
        'std::net::TcpStream::connect("127.0.0.1:1")',
        "std::time::Instant::now()",
        "std::thread::spawn(|| ())",
        "signal_hook::flag::register(2, flag)",
        'std::process::Command::new("jq")',
        "crate::engine::request()",
        "crate::cli::edge::Environment::read()",
        "std::fs::write(path, bytes)",
        "use std::fs::remove_file;",
        "use std::fs::{read, rename};",
        "std::fs::File::create(path)",
        "std::fs::OpenOptions::new()",
        "std::fs::File::create_new(path)",
        "std::fs::File::options()",
        "std::fs::DirBuilder::new().create(path)",
        "std::fs::hard_link(a, b)",
        "std::fs::soft_link(a, b)",
        "std::os::unix::fs::symlink(a, b)",
        "use std::fs::File as F; F::open(path);",
        "use std::fs::*;",
    )
    for plant in plants:
        if not measure_policy_failures(plant):
            fail("measure", f"the planted measure violation {plant!r} is refused")
    controls = (
        "std::fs::read(path)",
        "std::io::stdin().lock()",
        "use crate::core::measure::key::Key;",
        "use crate::cli::measure::{Cause, Refusal, read};",
        "use crate::failure::Failure;",
        "use clap::{Args, ValueEnum};",
    )
    for control in controls:
        if measure_policy_failures(control):
            fail("measure", f"the measure control {control!r} stays allowed")
    writer_plants = (
        "std::fs::rename(a, b)",
        "std::fs::OpenOptions::new()",
        "std::fs::File::create(path)",
        "use std::fs::write; write(path, text)",
        "fs::write(path, text)",
    )
    for plant in writer_plants:
        if not measure_policy_failures(plant, writer=True):
            fail("measure", f"the planted writer violation {plant!r} is refused")
    if measure_policy_failures("std::fs::write(path, text)", writer=True):
        fail("measure", "the writer's one std::fs::write stays allowed")
    audit, diff = "if let Some(Command::Audit(a)) = c {}", "if let Some(Command::Diff(a)) = c {}"
    read = "let e = Environment::read();"
    if route_failures(audit + read + diff) != ["Diff returns after Environment::read"] \
            or route_failures(audit + diff + read):
        fail("measure", "a late diff return is refused and early returns allowed")


# Ticket 0077: every live attempt passes the one process width gate in the
# HTTP module, so no other production file reaches the HTTP library, and only
# the one accessor names the process width state.
HTTP_DOOR = "crates/thinkthen/src/engine/http.rs"
WIDTH_DOOR = "crates/thinkthen/src/engine/mod.rs"
WIDTH_STATE = "PROCESS_WIDTH"
# Ticket 0078: an embedding host keeps its signal dispositions.
ENGINE = "crates/thinkthen/src/engine/"


def is_test_source(relative: str) -> bool:
    return ("/tests/" in relative or "_tests/" in relative
            or relative.endswith(("/tests.rs", "_tests.rs")))


def door_failures(sources: dict[str, str]) -> list[str]:
    """Name each second live-send door and each second width-state reference."""
    held = []
    for relative, text in sorted(sources.items()):
        tokens = rust_tokens(text)
        if "ureq" in tokens and relative != HTTP_DOOR and not is_test_source(relative):
            held.append(f"{relative} reaches ureq outside {HTTP_DOOR}")
        if ("signal_hook" in tokens and relative.startswith(ENGINE)
                and not is_test_source(relative)):
            held.append(f"{relative} installs a signal handler inside the engine")
        uses = tokens.count(WIDTH_STATE)
        if uses and (relative != WIDTH_DOOR or uses != 2):
            held.append(f"{relative} names {WIDTH_STATE} {uses} times")
    return held


def check_doors() -> None:
    sources = {
        source.relative_to(REPO).as_posix(): source.read_text(encoding="utf-8")
        for source in sorted((REPO / "crates/thinkthen/src").rglob("*.rs"))
    }
    if WIDTH_STATE not in rust_tokens(sources.get(WIDTH_DOOR, "")):
        fail("doors", f"{WIDTH_DOOR} holds the process width state")
    for failure in door_failures(sources):
        fail("doors", failure)
    plants = (
        ("crates/thinkthen/src/cli/find.rs", "ureq::post(url).send(body)"),
        ("crates/thinkthen/src/engine/request.rs", "use ureq::Agent;"),
        ("crates/thinkthen/src/cli/schedule.rs", "crate::engine::PROCESS_WIDTH.select(None)"),
        ("crates/thinkthen/src/engine/width_tests.rs", "&super::PROCESS_WIDTH"),
        (WIDTH_DOOR, "fn second() -> &'static Widths { &PROCESS_WIDTH }"),
        ("crates/thinkthen/src/engine/recorder.rs", "signal_hook::flag::register(SIGXFSZ, flag)"),
    )
    for relative, text in plants:
        if not door_failures({**sources, relative: sources.get(relative, "") + "\n" + text}):
            fail("doors", f"the planted door {text[-60:]!r} in {relative} is refused")
    controls = (
        ("crates/thinkthen/src/cli/find.rs", "// ureq stays in the HTTP module"),
        ("crates/thinkthen/src/cli/find.rs", 'const NOTE: &str = "PROCESS_WIDTH";'),
        ("crates/thinkthen/src/engine/http/tests.rs", "ureq::Error::HostNotFound"),
        ("crates/thinkthen/src/engine/host_signal_tests.rs", "signal_hook::flag::register"),
        ("crates/thinkthen/src/cli/file_size.rs", "signal_hook::flag::register"),
    )
    for relative, text in controls:
        if door_failures({**sources, relative: sources.get(relative, "") + "\n" + text}):
            fail("doors", f"the door control {text!r} in {relative} stays allowed")


# Ticket 0085: the command reaches the engine through the one facade. Only
# the facade builds the production HTTP pool, and no command file names the
# scheduler, request, recorder, or transport modules underneath it.
FACADE = "crates/thinkthen/src/engine/facade.rs"
CLI = "crates/thinkthen/src/cli/"
LIBRARY_ROOT = "crates/thinkthen/src/lib.rs"
LOW_MODULES = frozenset({
    "annotate_schedule", "http", "prepared_request", "recorder", "request", "schedule", "workers",
})
# `crate::schedule` and `crate::annotate_schedule` name the command's own modules.
LOW_ALIASES = LOW_MODULES - {"annotate_schedule", "schedule"}


def without_test_modules(tokens: list[str]) -> list[str]:
    """Drop each `#[cfg(test)] mod name { ... }` block from one token list."""
    kept = []
    place = 0
    marker = ["#", "[", "cfg", "(", "test", ")", "]", "mod"]
    while place < len(tokens):
        if tokens[place:place + len(marker)] != marker or place + 9 >= len(tokens) \
                or tokens[place + 9] != "{":
            kept.append(tokens[place])
            place += 1
            continue
        place += 9
        depth = 0
        while place < len(tokens):
            depth += int(tokens[place] == "{") - int(tokens[place] == "}")
            place += 1
            if depth == 0:
                break
    return kept


def facade_failures(sources: dict[str, str]) -> list[str]:
    """Name each second pool builder and each command path under the facade."""
    held = []
    for relative, text in sorted(sources.items()):
        if is_test_source(relative):
            continue
        tokens = without_test_modules(rust_tokens(text))
        if relative != FACADE and any(
                token_path_at(tokens, place, ("Client", "new")) for place in range(len(tokens))):
            held.append(f"{relative} builds the HTTP pool outside {FACADE}")
        if relative == LIBRARY_ROOT:
            for path, _ in rust_use_paths(tokens):
                if path[:1] == ("engine",) and len(path) > 1 and path[1] in LOW_MODULES:
                    held.append(f"{relative} re-exports engine::{path[1]}")
        if not relative.startswith(CLI):
            continue
        for place, token in enumerate(tokens):
            if token == "engine" and tokens[place + 1:place + 2] == ["::"] \
                    and tokens[place + 2:place + 3] and tokens[place + 2] in LOW_MODULES:
                held.append(f"{relative} names engine::{tokens[place + 2]}")
            if token == "crate" and tokens[place + 1:place + 2] == ["::"] \
                    and tokens[place + 2:place + 3] and tokens[place + 2] in LOW_ALIASES:
                held.append(f"{relative} names crate::{tokens[place + 2]}")
        for path, alias in rust_use_paths(tokens):
            if path[:2] != ("crate", "engine"):
                continue
            if len(path) > 2 and path[2] in LOW_MODULES:
                held.append(f"{relative} imports engine::{path[2]}")
            if len(path) == 2 and alias is not None or path[2:] == ("*",):
                held.append(f"{relative} aliases or globs the engine")
    return sorted(set(held))


def check_facade() -> None:
    sources = {
        source.relative_to(REPO).as_posix(): source.read_text(encoding="utf-8")
        for source in sorted((REPO / "crates/thinkthen/src").rglob("*.rs"))
    }
    if not any(token_path_at(rust_tokens(sources.get(FACADE, "")), place, ("Client", "new"))
               for place in range(len(rust_tokens(sources.get(FACADE, ""))))):
        fail("facade", f"{FACADE} builds the HTTP pool")
    for failure in facade_failures(sources):
        fail("facade", failure)
    plants = (
        ("crates/thinkthen/src/cli/find.rs", "fn pool() { let _ = Client::new(timeout, true); }"),
        ("crates/thinkthen/src/engine/request.rs",
         "#[cfg(test)] mod t {}\nfn pool() -> Client { Client::new(Duration::ZERO, true) }"),
        ("crates/thinkthen/src/cli/judge.rs", "use crate::engine::schedule::run_cancelled;"),
        ("crates/thinkthen/src/cli/annotate.rs",
         "use crate::engine::{facade, annotate_schedule as grouped};"),
        ("crates/thinkthen/src/cli/asking.rs",
         "fn f() { crate::engine::recorder::Recorder::of(None, None); }"),
        ("crates/thinkthen/src/cli/asking.rs", "use crate::engine::{self as low};"),
        ("crates/thinkthen/src/cli/relate.rs", "use crate::engine::*;"),
        ("crates/thinkthen/src/cli/find.rs", "fn f() { crate::prepared_request::split(); }"),
        (LIBRARY_ROOT, "pub(crate) use engine::{http, recorder};"),
    )
    for relative, text in plants:
        if not facade_failures({**sources, relative: sources.get(relative, "") + "\n" + text}):
            fail("facade", f"the planted facade bypass {text[-60:]!r} in {relative} is refused")
    controls = (
        ("crates/thinkthen/src/cli/find.rs", "use crate::engine::facade::{Engine, Found};"),
        ("crates/thinkthen/src/cli/judge.rs", "use crate::schedule::Output;"),
        ("crates/thinkthen/src/cli/find.rs", "// crate::engine::http::Client::new stays below"),
        ("crates/thinkthen/src/engine/http/tests.rs", "Client::new(Duration::ZERO, false)"),
        ("crates/thinkthen/src/cli/failure/tests.rs",
         "crate::engine::http::Client::new(Duration::ZERO, false)"),
        ("crates/thinkthen/src/engine/http.rs",
         "#[cfg(test)]\nmod more { fn f() { Client::new(Duration::ZERO, false); } }"),
    )
    for relative, text in controls:
        if facade_failures({**sources, relative: sources.get(relative, "") + "\n" + text}):
            fail("facade", f"the facade control {text!r} in {relative} stays allowed")


def check_crate_roots() -> None:
    for relative, attributes in ACCEPTED_CRATE_ROOT_ATTRIBUTES.items():
        try:
            lines = (REPO / relative).read_text(encoding="utf-8").splitlines()
        except OSError as error:
            fail("crate-root", f"cannot read {relative}: {error}")
            continue
        for attribute in attributes:
            if lines.count(attribute) != 1:
                fail("crate-root", f"{relative} holds exactly one {attribute}")


def check_sources() -> None:
    sources = sorted(
        source for folder in ("crates", "conformance") for source in (REPO / folder).rglob("*.rs")
    )
    if not sources:
        fail("size", "the workspace holds at least one Rust source file")
    for source in sources:
        lines = sum(1 for line in source.read_text(encoding="utf-8").splitlines() if line.strip())
        if lines > MAX_FILE_LINES:
            relative = source.relative_to(REPO)
            fail("size", f"{relative} has {lines} non-blank lines and the ceiling is {MAX_FILE_LINES}")
    skips = sum(source.read_text(encoding="utf-8").count("rustfmt::skip") for source in sources)
    if skips > MAX_FORMAT_SKIPS:
        fail("format", f"crates hold {skips} rustfmt::skip attributes and the ceiling is {MAX_FORMAT_SKIPS}")


def adapter_paths() -> tuple[str, ...]:
    """The files a vendor word may live in: `adapters.rs` and each module it declares.

    The folder itself is not the permission. A file dropped beside the adapters
    without a `pub(crate) mod` line naming it is read like any other source, so the
    exemption cannot be taken by moving a file.
    """
    listing = REPO / ADAPTERS
    if not listing.is_file():
        # Every source is then read, because nothing is exempt. The run fails
        # on this line and on each vendor word the adapter's own files hold.
        fail("seam", f"{ADAPTERS} is missing, so the seam has no home")
        return ()
    declared = re.findall(
        r"^pub(?:\(crate\))? mod (\w+);", listing.read_text(encoding="utf-8"), re.M
    )
    if not declared:
        fail("seam", f"{ADAPTERS} declares no adapter module, so the seam has no home")
    folder = ADAPTERS.removesuffix(".rs")
    return (ADAPTERS, *(f"{folder}/{name}.rs" for name in declared),
            *(f"{folder}/{name}/" for name in declared))


def check_seam() -> None:
    """Hold the vendor's words inside the adapter, the fixtures, and the tests."""
    adapter = adapter_paths()
    unused = set(SEAM_ALLOWED)
    for source in sorted((REPO / "crates").rglob("*.rs")):
        relative = source.relative_to(REPO).as_posix()
        # A file named tests.rs is one module's `#[cfg(test)] mod tests`, and
        # a file under tests/ is an integration test. Both are tests.
        if (relative.startswith(adapter) or "/tests/" in relative
                or "/conformance_tests/" in relative
                or relative.endswith(("/tests.rs", "_tests.rs"))):
            continue
        allowed = SEAM_ALLOWED.get(relative, frozenset())
        unused.discard(relative)
        held = set()
        lines = source.read_text(encoding="utf-8").splitlines()
        # The code ends where the module's test module begins. A bare
        # `#[cfg(test)]` on anything else, such as a use line, stops nothing,
        # so the attribute alone is not the end.
        for number, line in enumerate(lines, 1):
            if line.startswith("#[cfg(test)]") and lines[number : number + 1] and (
                lines[number].startswith("mod tests")
            ):
                break
            for word in VENDOR_WORDS:
                if word in line.lower() and word not in allowed:
                    fail("seam", f"{relative}:{number} names {word!r} outside the adapter")
                elif word in line.lower():
                    held.add(word)
        for word in sorted(allowed - held):
            fail("seam", f"{relative} no longer names {word!r}, so the seam allowance goes")
    for relative in sorted(unused):
        fail("seam", f"the seam allowance names {relative}, which the workspace no longer holds")


def license_allowed(expression: str, allowed: set[str] = frozenset()) -> bool:
    """Say whether an SPDX expression offers this repository an allowed license.

    OR takes any allowed operand, AND takes every operand, and OR binds looser
    than AND. The deprecated slash spells OR. A `WITH` exception makes its own
    identifier, so `Apache-2.0 WITH LLVM-exception` is not `Apache-2.0`, and a
    crate that offers it beside MIT still passes through the OR.
    """
    allowed = ALLOWED_LICENSES | set(allowed)
    tokens = re.findall(r"[A-Za-z0-9.+-]+|[()/]", expression)
    if "".join(tokens) != re.sub(r"\s+", "", expression):
        return False
    tokens = ["OR" if token == "/" else token for token in tokens]
    position = 0

    def primary() -> bool | None:
        nonlocal position
        if position == len(tokens):
            return None
        if tokens[position] == "(":
            position += 1
            value = disjunction()
            if value is None or position == len(tokens) or tokens[position] != ")":
                return None
            position += 1
            return value
        name = tokens[position]
        if name in {"AND", "OR", "WITH", ")"}:
            return None
        position += 1
        offered = name in allowed
        if position < len(tokens) and tokens[position] == "WITH":
            position += 1
            if position == len(tokens) or tokens[position] in {"AND", "OR", "WITH", "(", ")"}:
                return None
            # Only an allowance that names the whole `WITH` form admits it.
            offered = f"{name} WITH {tokens[position]}" in allowed
            position += 1
        return offered

    def conjunction() -> bool | None:
        nonlocal position
        value = primary()
        if value is None:
            return None
        while position < len(tokens) and tokens[position] == "AND":
            position += 1
            operand = primary()
            if operand is None:
                return None
            value = value and operand
        return value

    def disjunction() -> bool | None:
        nonlocal position
        value = conjunction()
        if value is None:
            return None
        while position < len(tokens) and tokens[position] == "OR":
            position += 1
            operand = conjunction()
            if operand is None:
                return None
            value = value or operand
        return value

    value = disjunction()
    return value is True and position == len(tokens)


LICENSE_GRAMMAR_CASES = (
    ("MIT", True),
    ("GPL-3.0", False),
    ("MIT OR GPL-3.0", True),
    ("MIT AND GPL-3.0", False),
    ("Apache-2.0 / MIT", True),
    ("(MIT OR Apache-2.0) AND Unicode-3.0", True),
    ("(MIT OR GPL-3.0) AND GPL-2.0", False),
    ("Apache-2.0 WITH LLVM-exception", False),
    ("Apache-2.0 AND ISC", False),
    ("Apache-2.0 AND GPL-3.0", False),
    ("Apache-2.0 WITH LLVM-exception OR MIT", True),
    ("", False),
    ("MIT OR", False),
    ("MIT MIT", False),
    ("(MIT", False),
    ("AND MIT", False),
)


def check_license_grammar() -> None:
    """Hold the SPDX reader to its table, since a widened reader passes quietly."""
    for expression, expected in LICENSE_GRAMMAR_CASES:
        if license_allowed(expression) is not expected:
            fail("dependencies", f"the SPDX reader answers {expected} for {expression!r}")
    for expression, expected in ((("Apache-2.0 AND ISC"), True), ("ISC AND MIT", True)):
        if license_allowed(expression, {"ISC"}) is not expected:
            fail("dependencies", f"the SPDX reader answers {expected} for {expression!r} with ISC")
    llvm = "Apache-2.0 WITH LLVM-exception"
    if not license_allowed(llvm, {llvm}) or license_allowed(llvm, {"LLVM-exception"}):
        fail("dependencies", f"the SPDX reader admits {llvm!r} only by its whole form")


def check_dependencies() -> None:
    result = subprocess.run(
        ["cargo", "metadata", "--locked", "--all-features", "--format-version", "1"],
        cwd=REPO, check=False, capture_output=True, text=True,
    )
    if result.returncode != 0:
        fail("dependencies", f"cargo metadata --locked succeeds: {result.stderr.strip()}")
        return
    metadata = json.loads(result.stdout)
    packages = {package["id"]: package for package in metadata["packages"]}
    members = {packages[identifier]["name"]: identifier for identifier in metadata["workspace_members"]}
    if set(members) != set(ACCEPTED_DEPENDENCIES):
        fail("dependencies", "cargo metadata reports exactly the accepted members")
        return

    nodes = {node["id"]: node for node in metadata["resolve"]["nodes"]}
    resolved_names: set[str] = set()
    for name, identifier in members.items():
        resolved = {packages[dependency]["name"] for dependency in nodes[identifier]["dependencies"]}
        accepted = (
            ACCEPTED_DEPENDENCIES[name]
            | ACCEPTED_TARGET_DEPENDENCIES[name]
            | ACCEPTED_DEV_DEPENDENCIES[name]
        )
        if resolved != accepted:
            fail("dependencies", f"{name} resolves the accepted direct dependency set, found {sorted(resolved)}")

    for identifier in nodes:
        if identifier in members.values():
            continue
        package = packages[identifier]
        if package["source"] != CRATES_IO:
            fail("dependencies", f"{package['name']} resolves from crates.io")
        name = package["name"]
        expression = package.get("license") or ""
        exception = LICENSE_EXCEPTIONS.get(name, set())
        if not license_allowed(expression, exception):
            fail("dependencies", f"{name} has a disallowed license expression {expression!r}")
        elif exception and license_allowed(expression):
            fail("dependencies", f"{name} passes without its license exception, so the exception goes")
        resolved_names.add(name)

    for name in sorted(set(LICENSE_EXCEPTIONS) - resolved_names):
        fail("dependencies", f"the license exception for {name} names a crate the tree no longer holds")

    for locked in read_toml("Cargo.lock").get("package", []):
        if locked["name"] in members:
            continue
        if locked.get("source") != CRATES_IO or not locked.get("checksum"):
            fail("dependencies", f"locked package {locked['name']} carries a crates.io source and a checksum")
    print(f"policy: checked {len(nodes)} resolved packages")

    check_library_graph()


# Ticket 0078: the library alone installs no signal handler, and on Unix it
# masks host signals on its workers through nix. The graph covers every target
# and build edges. Dev edges stay out, because the tests use `signal-hook`.
COMMAND_ONLY = {"clap", "csv-core", "signal-hook"}
GRAPH_PLANTS = (
    '[target."cfg(windows)".dependencies]\nclap = "4.6.7"\n',
    '[build-dependencies]\nclap = "4.6.7"\n',
)
GRAPH_FILES = (
    "Cargo.toml", "Cargo.lock", "crates/thinkthen/Cargo.toml", "conformance/backend/Cargo.toml",
)
GRAPH_STUBS = (
    "crates/thinkthen/src/lib.rs", "crates/thinkthen/src/main.rs",
    "conformance/backend/src/lib.rs", "conformance/backend/src/main.rs",
)


def library_direct(root: pathlib.Path, frozen: str) -> set[str] | str:
    """Name the library's direct normal and build dependencies on every target."""
    result = subprocess.run(
        ["cargo", "tree", frozen, "-p", "thinkthen", "-e", "normal,build", "--target", "all",
         "--no-default-features", "--depth", "1", "--prefix", "none"],
        cwd=root, check=False, capture_output=True, text=True,
    )
    if result.returncode != 0:
        return result.stderr.strip()
    return {line.split()[0] for line in result.stdout.splitlines()[1:] if line.strip()}


def graph_failures(direct: set[str]) -> list[str]:
    held = []
    if direct & COMMAND_ONLY:
        held.append("the default-features-off graph excludes command dependencies")
    if sys.platform != "win32" and "nix" not in direct:
        held.append("the default-features-off graph holds nix on Unix")
    return held


def check_library_graph() -> None:
    direct = library_direct(REPO, "--locked")
    if isinstance(direct, str):
        fail("dependencies", f"default-features-off cargo tree succeeds: {direct}")
        return
    for failure in graph_failures(direct):
        fail("dependencies", failure)
    for plant in GRAPH_PLANTS:
        with tempfile.TemporaryDirectory() as scratch:
            root = pathlib.Path(scratch)
            for relative in GRAPH_FILES + GRAPH_STUBS:
                (root / relative).parent.mkdir(parents=True, exist_ok=True)
                source = REPO / relative
                (root / relative).write_text(
                    source.read_text(encoding="utf-8") if relative in GRAPH_FILES else "",
                    encoding="utf-8",
                )
            manifest = root / "crates/thinkthen/Cargo.toml"
            manifest.write_text(manifest.read_text(encoding="utf-8") + "\n" + plant, encoding="utf-8")
            planted = library_direct(root, "--offline")
            if isinstance(planted, str):
                fail("dependencies", f"the planted graph reads through cargo tree: {planted}")
            elif not graph_failures(planted):
                fail("dependencies", f"the planted graph {plant.splitlines()[0]!r} is refused")

HEADER_WORDS = ("header", "authorization", "api-key", "api_key", "x-api-key", "cookie")


def recording_failures(held: object, place: str = "") -> list[str]:
    """Every header, credential key, or bearer value in one recording entry."""
    found: list[str] = []
    if isinstance(held, dict):
        for key, value in held.items():
            if any(word in key.lower() for word in HEADER_WORDS):
                found.append(f"{place}/{key} is a header or credential key")
            found += recording_failures(value, f"{place}/{key}")
    elif isinstance(held, list):
        for at, value in enumerate(held):
            found += recording_failures(value, f"{place}/{at}")
    elif isinstance(held, str) and held.lower().lstrip().startswith("bearer "):
        found.append(f"{place} holds a bearer value")
    return found


def check_recordings() -> None:
    """Ticket 0147: a recording keeps no header, so it can never hold a key."""
    for plant in ({"request": {"headers": {}}}, {"Authorization": "x"}, {"response": ["Bearer x"]}):
        if not recording_failures(plant):
            fail("recordings", f"the planted entry {plant!r} is refused")
    listed = subprocess.run(["git", "ls-files", "-z", "*.json"], cwd=REPO, capture_output=True, check=True)
    for relative in listed.stdout.decode().split("\0"):
        if "/recording" not in f"/{relative}":
            continue
        try:
            held = json.loads((REPO / relative).read_text(encoding="utf-8"))
        except (OSError, ValueError) as error:
            fail("recordings", f"cannot read {relative}: {error}")
            continue
        for failure in recording_failures(held):
            fail("recordings", f"{relative}: {failure}")


def main() -> int:
    check_toolchain()
    check_workspace()
    check_crates()
    check_clippy_configs()
    check_consumer()
    check_bindings()
    check_polars_feature()
    check_postgresql_binding()
    check_crate_roots()
    check_core_policy()
    check_catalog_policy()
    check_measure_policy()
    check_doors()
    check_facade()
    check_sources()
    check_seam()
    check_license_grammar()
    check_dependencies()
    check_recordings()
    for failure in FAILURES:
        print(failure, file=sys.stderr)
    if FAILURES:
        return 1
    print("policy: every accepted table, ban list, and dependency matches")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
