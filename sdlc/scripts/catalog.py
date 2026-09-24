#!/usr/bin/env python3
"""Hold the built-in transform catalog to the repository transforms.

Ticket 0083. `transforms/NAME/NAME.jq` is the development source,
`crates/thinkthen/transforms/NAME.jq` is the shipped copy, and the table in
`cli/transform.rs` names the members. Every name and every byte agrees in all
three, and in the source package when `--crate PATH` names one.

Usage: catalog.py, run by `lint`, checks the tree and then proves the archive
check refuses a changed byte, a missing member, and an unlisted member.
catalog.py --crate PATH, run by `package`, checks one `.crate` archive.
"""

import io
import pathlib
import re
import sys
import tarfile

REPO = pathlib.Path(__file__).resolve().parents[2]
PACKAGED = "crates/thinkthen/transforms"
TABLE = "crates/thinkthen/src/cli/transform.rs"
ROW = re.compile(r'\(\s*"([a-z]+)",\s*include_bytes!\("\.\./\.\./transforms/([a-z]+)\.jq"\),?\s*\)')


def repository(root):
    """The development sources, one folder per transform."""
    found = {}
    for path in sorted((root / "transforms").glob("*/*.jq")):
        if path.stem != path.parent.name:
            continue
        found[path.stem] = path.read_bytes()
    return found


def compare(expected, actual, where):
    """Name every missing, unlisted, or changed member."""
    failures = [f"{where}: {name}.jq is missing" for name in sorted(set(expected) - set(actual))]
    failures += [f"{where}: {name} is not a listed member" for name in sorted(set(actual) - set(expected))]
    failures += [f"{where}: {name}.jq differs from the repository copy"
                 for name in sorted(set(expected) & set(actual)) if expected[name] != actual[name]]
    return failures


def stem(file):
    """A member's public name, or the whole file name when it is not `.jq`."""
    return file.removesuffix(".jq") if file.endswith(".jq") else file


def archive(crate, expected):
    """Compare the transforms inside one opened source package archive."""
    actual = {}
    for member in crate.getmembers():
        parts = member.name.split("/")
        if len(parts) == 3 and parts[1] == "transforms":
            actual[stem(parts[2])] = crate.extractfile(member).read() if member.isfile() else b""
    return compare(expected, actual, "package")


def tree(root):
    """Compare the shipped copies and the Rust table with the sources."""
    expected = repository(root)
    packaged = {stem(path.name): path.read_bytes() for path in sorted((root / PACKAGED).iterdir())}
    failures = compare(expected, packaged, PACKAGED)
    rows = ROW.findall((root / TABLE).read_text(encoding="utf-8"))
    names = [name for name, _ in rows]
    if any(name != file for name, file in rows) or names != sorted(expected):
        failures.append(f"{TABLE}: the table names {names}, not {sorted(expected)}")
    return failures


def planted(expected):
    """The archive check refuses each planted fault for its own reason."""
    def crate(members):
        data = io.BytesIO()
        with tarfile.open(fileobj=data, mode="w:gz") as built:
            for name, bytes_ in members.items():
                entry = tarfile.TarInfo(f"thinkthen-0.0.1/transforms/{name}.jq")
                entry.size = len(bytes_)
                built.addfile(entry, io.BytesIO(bytes_))
        data.seek(0)
        return data

    first = sorted(expected)[0]
    changed = dict(expected, **{first: expected[first][:-1] + b" "})
    missing = {name: bytes_ for name, bytes_ in expected.items() if name != first}
    unlisted = dict(expected, extra=b".\n")
    cases = (
        (expected, []),
        (changed, [f"package: {first}.jq differs from the repository copy"]),
        (missing, [f"package: {first}.jq is missing"]),
        (unlisted, ["package: extra is not a listed member"]),
    )
    failures = []
    for members, said in cases:
        with tarfile.open(fileobj=crate(members), mode="r:gz") as opened:
            found = archive(opened, expected)
        if found != said:
            failures.append(f"self-test: expected {said}, found {found}")
    return failures


def main(arguments):
    expected = repository(REPO)
    if len(expected) != 10:
        print(f"catalog: the repository holds {len(expected)} transforms, not 10", file=sys.stderr)
        return 1
    if arguments[:1] == ["--crate"] and len(arguments) == 2:
        with tarfile.open(arguments[1], "r:gz") as opened:
            failures = archive(opened, expected)
        done = "catalog: the source package holds exactly the ten transforms, byte for byte"
    elif not arguments:
        failures = tree(REPO) + planted(expected)
        done = "catalog: the shipped copies and the table match the sources, and every planted fault fails"
    else:
        print(__doc__, file=sys.stderr)
        return 2
    for failure in failures:
        print(f"catalog: {failure}", file=sys.stderr)
    if failures:
        return 1
    print(done)
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
