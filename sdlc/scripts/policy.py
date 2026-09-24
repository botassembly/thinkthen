#!/usr/bin/env python3
"""Check the policies that cargo and clippy cannot check for themselves.

Every check reports and the script keeps going, so one run names every
problem. The standard library is the only import.
"""

from __future__ import annotations

import json
import pathlib
import re
import subprocess
import sys
import tomllib

REPO = pathlib.Path(__file__).resolve().parents[2]
CRATES_IO = "registry+https://github.com/rust-lang/crates.io-index"
ALLOWED_LICENSES = {
    "MIT",
    "Apache-2.0",
    "Unicode-3.0",
    "Unlicense",
}
# `csv-core` and `signal-hook` use the already accepted terms. Three more licenses
# arrive with the TLS stack under ureq and with nothing
# else. Each one is tied to the crates that force it, so the allowance cannot
# quietly cover a crate that lands later. All three are permissive and carry no
# copyleft term, and there is no HTTPS in Rust without them. A crate listed here
# that stops needing its exception fails the check, so the list cannot rot.
LICENSE_EXCEPTIONS = {
    "ring": {"ISC"},
    "rustls-webpki": {"ISC"},
    "untrusted": {"ISC"},
    "subtle": {"BSD-3-Clause"},
    "webpki-roots": {"CDLA-Permissive-2.0"},
}
ACCEPTED_DEPENDENCIES = {
    "thinkthen": {
        "clap", "csv-core", "serde", "serde_json", "sha2", "signal-hook", "thiserror", "ureq"
    },
}
ACCEPTED_TARGET_DEPENDENCIES = {"thinkthen": {"nix"}}
ACCEPTED_DEV_DEPENDENCIES = {"thinkthen": {"proptest"}}
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
    if workspace.get("members") != ["crates/thinkthen"]:
        fail("workspace", "thinkthen is the one workspace member")
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


def check_crates() -> None:
    for name in sorted(ACCEPTED_DEPENDENCIES):
        manifest = read_toml(f"crates/{name}/Cargo.toml")
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
        if target.get("nix") != {
            "version": "0.29",
            "default-features": False,
            "features": ["signal"],
            "optional": True,
        }:
            fail("dependencies", "nix is optional on Unix with only its signal feature")
        if set(manifest.get("dev-dependencies", {})) != ACCEPTED_DEV_DEPENDENCIES[name]:
            fail("dependencies", f"{name} declares the accepted development dependency set")
        optional = {
            dependency for dependency, specification in
            (manifest.get("dependencies", {}) | target).items()
            if isinstance(specification, dict) and specification.get("optional") is True
        }
        if optional != {"clap", "csv-core", "nix"}:
            fail("dependencies", "exactly the command dependencies are optional")
        binary = manifest.get("bin", [])
        if len(binary) != 1 or binary[0].get("required-features") != ["cli"]:
            fail("workspace", "the binary requires the cli feature")
        features = manifest.get("features", {})
        if features.get("default") != ["cli"] or set(features.get("cli", [])) != {
            "dep:clap", "dep:csv-core", "dep:nix",
        }:
            fail("dependencies", "the default cli feature selects only command dependencies")


def check_clippy_configs() -> None:
    if read_toml("crates/thinkthen/clippy.toml") != accepted_core_clippy():
        fail("clippy-config", "crates/thinkthen/clippy.toml matches the complete accepted copy")


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
CATALOG_ROOTS = {"std", "clap", "crate", "super", "self"}


def catalog_policy_failures(text: str) -> list[str]:
    """Name every capability or owner the catalog source reaches."""
    tokens = rust_tokens(text)
    held = {f"names {token}" for token in tokens if token in CATALOG_BANNED_WORDS}
    for path, alias in rust_use_paths(tokens):
        if aliases_outer_root(path, alias) or imports_outer_glob(path):
            held.add("aliases or globs an outer module")
        elif path[:1] == ("crate",) and path not in CATALOG_CRATE_PATHS:
            held.add("uses " + "::".join(path))
        elif path[:1] == ("super",) and path not in {("super", "CATALOG"), ("super", "lookup")}:
            held.add("uses " + "::".join(path))
        elif path and path[0] not in CATALOG_ROOTS:
            held.add("uses " + "::".join(path))
    for place, token in enumerate(tokens[:-1]):
        if token in {"crate", "super"} and tokens[place + 1] == "::" and not any(
                token_path_at(tokens, place, allowed) for allowed in CATALOG_CRATE_PATHS):
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
    sources = sorted((REPO / "crates").rglob("*.rs"))
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
            position += 1
            offered = False
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


def check_dependencies() -> None:
    result = subprocess.run(
        ["cargo", "metadata", "--locked", "--format-version", "1"],
        cwd=REPO, check=False, capture_output=True, text=True,
    )
    if result.returncode != 0:
        fail("dependencies", f"cargo metadata --locked succeeds: {result.stderr.strip()}")
        return
    metadata = json.loads(result.stdout)
    packages = {package["id"]: package for package in metadata["packages"]}
    members = {packages[identifier]["name"]: identifier for identifier in metadata["workspace_members"]}
    if set(members) != set(ACCEPTED_DEPENDENCIES):
        fail("dependencies", "cargo metadata reports exactly the thinkthen package")
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

    minimal = subprocess.run(
        ["cargo", "metadata", "--locked", "--format-version", "1", "--no-default-features"],
        cwd=REPO, check=False, capture_output=True, text=True,
    )
    if minimal.returncode != 0:
        fail("dependencies", f"default-features-off metadata succeeds: {minimal.stderr.strip()}")
        return
    graph = json.loads(minimal.stdout)
    packages = {package["id"]: package for package in graph["packages"]}
    package = next((item for item in graph["packages"] if item["name"] == "thinkthen"), None)
    if package is None:
        fail("dependencies", "the minimal graph contains thinkthen")
        return
    node = next(item for item in graph["resolve"]["nodes"] if item["id"] == package["id"])
    direct = {packages[item["pkg"]]["name"] for item in node["deps"]}
    if direct & {"clap", "csv-core", "nix"}:
        fail("dependencies", "the default-features-off graph excludes command dependencies")


def main() -> int:
    check_toolchain()
    check_workspace()
    check_crates()
    check_clippy_configs()
    check_crate_roots()
    check_core_policy()
    check_catalog_policy()
    check_sources()
    check_seam()
    check_license_grammar()
    check_dependencies()
    for failure in FAILURES:
        print(failure, file=sys.stderr)
    if FAILURES:
        return 1
    print("policy: every accepted table, ban list, and dependency matches")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
