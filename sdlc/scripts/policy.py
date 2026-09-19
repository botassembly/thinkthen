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
# Three more licenses arrive with the TLS stack under ureq and with nothing
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
    "thinkthen": {"clap", "thinkthen-core", "ureq"},
    "thinkthen-core": {"serde", "serde_json", "sha2", "thiserror"},
}
ACCEPTED_DEV_DEPENDENCIES = {"thinkthen": set(), "thinkthen-core": {"proptest"}}
MAX_FILE_LINES = 500
INHERITED = {"workspace": True}

ACCEPTED_RUST_LINTS = {
    "missing_debug_implementations": "forbid",
    "missing_docs": "warn",
    "unreachable_pub": "forbid",
    "unsafe_code": "forbid",
}
ACCEPTED_CLIPPY_LINTS = {
    "allow_attributes_without_reason": "deny",
    "cognitive_complexity": "warn",
    "dbg_macro": "deny",
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
ACCEPTED_CRATE_ROOT_ATTRIBUTES = {
    "crates/thinkthen-core/src/lib.rs": (
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
    if sorted(workspace.get("members", [])) != ["crates/thinkthen", "crates/thinkthen-core"]:
        fail("workspace", "members are exactly the two crates")
    if workspace.get("resolver") != "3":
        fail("workspace", "resolver is 3")
    package = workspace.get("package", {})
    if package.get("edition") != "2024":
        fail("workspace", "edition is 2024")
    if re.fullmatch(r"[0-9]+\.[0-9]+\.[0-9]+", str(package.get("rust-version"))) is None:
        fail("workspace", "rust-version is one exact three-part release")
    if package.get("rust-version") != read_toml("rust-toolchain.toml").get("toolchain", {}).get("channel"):
        fail("workspace", "rust-version matches the pinned toolchain channel")
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
        if "license" in package:
            fail("workspace", f"{name} declares no license while the repository is private")
        if set(manifest.get("dependencies", {})) != ACCEPTED_DEPENDENCIES[name]:
            fail("dependencies", f"{name} declares the accepted direct dependency set")
        if set(manifest.get("dev-dependencies", {})) != ACCEPTED_DEV_DEPENDENCIES[name]:
            fail("dependencies", f"{name} declares the accepted development dependency set")


def check_clippy_configs() -> None:
    if read_toml("crates/thinkthen-core/clippy.toml") != accepted_core_clippy():
        fail("clippy-config", "crates/thinkthen-core/clippy.toml matches the accepted copy")
    if read_toml("crates/thinkthen/clippy.toml") != ACCEPTED_SHARED_CLIPPY:
        fail("clippy-config", "crates/thinkthen/clippy.toml matches the accepted copy")


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
        fail("dependencies", "cargo metadata reports exactly the two workspace crates")
        return

    nodes = {node["id"]: node for node in metadata["resolve"]["nodes"]}
    resolved_names: set[str] = set()
    for name, identifier in members.items():
        resolved = {packages[dependency]["name"] for dependency in nodes[identifier]["dependencies"]}
        accepted = ACCEPTED_DEPENDENCIES[name] | ACCEPTED_DEV_DEPENDENCIES[name]
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


def main() -> int:
    check_toolchain()
    check_workspace()
    check_crates()
    check_clippy_configs()
    check_crate_roots()
    check_sources()
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
