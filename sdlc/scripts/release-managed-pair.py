#!/usr/bin/env python3
"""Assemble or verify the fixed Linux C#/JVM wrappers from captured job receipts.

Receipts are created by the workflow at Git tar generation, C container output,
and managed compilation. This program never creates or refreshes their trust pins.
"""
import argparse
import hashlib
import io
import json
from pathlib import Path
import re
import shutil
import sys
import tarfile
import xml.etree.ElementTree as ET
import zipfile


def require(condition, message):
    if not condition:
        raise ValueError(message)


def regular(path):
    require(path.is_file() and not path.is_symlink(), f"missing or linked file: {path}")
    return path.read_bytes()


def digest(data):
    return hashlib.sha256(data).hexdigest()


def receipt(path, keys):
    data = json.loads(regular(path))
    require(type(data) is dict and set(data) == set(keys), f"invalid receipt: {path}")
    return data


def pinned(data, expected, label):
    require(re.fullmatch(r"[0-9a-f]{64}", expected) is not None, f"invalid {label} digest")
    require(digest(data) == expected, f"{label} differs from captured receipt")


def tar_files(data, label):
    with tarfile.open(fileobj=io.BytesIO(data), mode="r:*") as archive:
        files = {}
        dirs = set()
        seen = set()
        source_links = {"CLAUDE.md": "AGENTS.md", "crates/thinkthen/LICENSE": "../../LICENSE"}
        for member in archive:
            name = member.name.removeprefix("./")
            require(name not in seen and not name.startswith("/") and ".." not in Path(name).parts,
                    f"unsafe or repeated {label} member")
            seen.add(name)
            if label == "source" and member.issym():
                require(source_links.get(name) == member.linkname, f"unsafe source symlink: {name}")
                continue
            require(member.isfile() or member.isdir(), f"unsafe {label} member type: {name}")
            if member.isdir():
                require(name not in dirs, f"repeated {label} directory")
                dirs.add(name)
            else:
                files[name] = archive.extractfile(member).read()
        return files, archive.pax_headers.copy(), dirs


def archive_members(data, expected, label):
    files, _, dirs = tar_files(data, label)
    require(dirs <= {"", "."}, f"{label} directory inventory differs")
    require(set(files) == expected, f"{label} inventory differs")
    return files


def zip_members(data, label):
    with zipfile.ZipFile(io.BytesIO(data)) as archive:
        members = {}
        dirs = set()
        for item in archive.infolist():
            name = item.filename
            require(not name.startswith("/") and ".." not in Path(name).parts,
                    f"unsafe {label} member")
            require(name not in members, f"duplicate {label} member: {name}")
            member_type = (item.external_attr >> 16) & 0o170000
            allowed_type = (0, 0o040000) if item.is_dir() else (0, 0o100000)
            require(member_type in allowed_type, f"unsafe {label} ZIP member type: {name}")
            if item.is_dir():
                require(name not in dirs, f"duplicate {label} directory: {name}")
                dirs.add(name)
                continue
            members[name] = archive.read(item)
    return members, dirs


def xml_field(data, name):
    root = ET.fromstring(data)
    matches = [node.text.strip() for node in root.iter()
               if node.tag.rsplit("}", 1)[-1] == name and node.text]
    require(len(matches) == 1, f"ambiguous XML {name}")
    return matches[0]


def check_nupkg(data, version, source):
    members, dirs = zip_members(data, "nupkg")
    require(not dirs, "unexpected nupkg directory")
    fixed = {"Botassembly.ThinkThen.nuspec", "lib/net8.0/ThinkThen.dll", "README.md", "LICENSE",
             "_rels/.rels", "[Content_Types].xml"}
    other = set(members) - fixed
    require(fixed <= set(members) and len(other) == 1 and all(re.fullmatch(
        r"package/services/metadata/core-properties/[0-9a-fA-F-]+\.psmdcp", name) for name in other),
        "nupkg inventory differs")
    require(members["lib/net8.0/ThinkThen.dll"], "empty net8 DLL")
    nuspec = members["Botassembly.ThinkThen.nuspec"]
    require(xml_field(nuspec, "id") == "Botassembly.ThinkThen" and
            xml_field(nuspec, "version") == version, "nupkg identity differs")
    for name in ("README.md", "LICENSE"):
        require(members[name] == source[f"libraries/csharp/{name}"], f"nupkg {name} differs from source")
    forbidden = (b"/home/", b"/Users/", b"thinkthen_panic_probe", b"tt-canary-290", b"-----BEGIN PRIVATE KEY-----")
    require(not any(token in value or token in name.encode() for name, value in members.items()
                    for token in forbidden), "private nupkg member")


def check_jar(data, kind):
    members, dirs = zip_members(data, f"{kind} JAR")
    require(dirs == ({"META-INF/", "thinkthen/"} if kind == "door" else {"META-INF/"}),
            f"{kind} JAR directory inventory differs")
    manifest = members.pop("META-INF/MANIFEST.MF", None)
    require(manifest is not None and manifest.startswith(b"Manifest-Version: 1.0"), "JAR manifest differs")
    classes = {
        "door": (
            "Complete Complete$AnnotateRow Complete$AnnotationMember Complete$AtomicAnswer Complete$AtomicKind "
            "Complete$Attempt Complete$AttemptOutcome Complete$BatchKind Complete$BatchSetting Complete$BatchWarning "
            "Complete$CallControls Complete$CallFacts Complete$Choice Complete$ChooseRow Complete$CommonRow "
            "Complete$CompleteError Complete$Content Complete$ContentKind Complete$DecideRow Complete$DecideValue "
            "Complete$Direction Complete$Edge Complete$Endpoint Complete$Entity Complete$EntityEdge Complete$EventKind "
            "Complete$FileSource Complete$FilterRow Complete$FindRow Complete$Function Complete$IdentityKind "
            "Complete$ImageInput Complete$ImageView Complete$Location Complete$Media Complete$MemberCause "
            "Complete$MemberFailure Complete$MemberState Complete$MemberSuccess Complete$MemberValue Complete$Meta "
            "Complete$NameSpan Complete$ObservationEvent Complete$ObservationIdentity Complete$ObservationSuccess "
            "Complete$ObservedProbabilities Complete$OptionalValue Complete$Origin Complete$PairSpan Complete$Piece "
            "Complete$Place Complete$Probability Complete$ProfileWarning Complete$Question Complete$QuestionMember "
            "Complete$QuestionObservation Complete$QuestionSource Complete$RankRow Complete$RecognizeAnswer "
            "Complete$RecognizeRow Complete$RecognizeValue Complete$RecordInput Complete$RelateRow Complete$Relation "
            "Complete$RelationAnswer Complete$RelationMethod Complete$RelationSuccess Complete$RowObservation "
            "Complete$RowValue Complete$Rule Complete$RuleKind Complete$ScoreRow Complete$SourceUnit Complete$Stage "
            "Complete$StopCause Complete$Stopped Complete$TagRow Complete$TokenUsage Complete$ValueKind "
            "CompleteDetails CompleteDetails$DeclarationKind CompleteDetails$Details CompleteDetails$InputDeclaration "
            "CompleteDetails$InputProperty CompleteDetails$InputView CompleteDetails$PropertyKind "
            "CompleteDetails$QuestionAuthor CompleteDetails$ReportedUsage CompleteDetails$SourceDetail "
            "CompleteDetails$SourceEdge CompleteDetails$SourceEndpoint CompleteDetails$SourceEntity "
            "CompleteDetails$SourceEntityEdge CompleteDetails$SourceRecognition CompleteDetails$SourceRelations "
            "CompleteEngine CompleteEngine$CompleteCall CompleteReaders Door Door$AnnotatedField "
            "Door$AnnotatedField$Answered Door$AnnotatedField$Failed Door$AnnotatedField$Unresolved Door$Answer "
            "Door$Failure Door$FailureKind Door$NativeCall Door$NativeFailure Door$Outcome Door$Token Door$TypedResult Ids "
            "Ids$AnswerId Ids$CallId Ids$Digest Ids$FailureId Ids$ObservationId Ids$SdkRequestId Json Json$Reader "
            "NativeBatch NativeBatch$Command NativeCalls NativeCalls$NativeCall NativeDetailReaders0 NativeDetails NativeExecution "
            "NativeExecution$RowReader NativeInputs NativeLayouts NativeLayouts0 NativeLayouts1 NativeLayouts2 "
            "NativeLayouts3 NativeLayouts4 NativeLayouts5 NativeLayouts6 NativeLayouts7 NativeRead NativeReaders0 "
            "NativeReaders1 NativeReaders2 Questions Requests Requests$CompleteRequest Requests$InputSource "
            "Requests$QuestionInput Requests$QuestionRole "
        ).split(),
        "kotlin": (
            "KotlinComplete KotlinFacade KotlinFacade$RunningDecision KotlinRequests "
        ).split(),
        "scala": (
            "ScalaComplete ScalaComplete$ ScalaFacade "
            "ScalaFacade$RunningDecision ScalaRequests ScalaRequests$ "
        ).split(),
    }[kind]
    expected = {("thinkthen/" if kind == "door" else "") + name + ".class" for name in classes}
    if kind == "kotlin":
        expected.add("META-INF/main.kotlin_module")
    if kind == "scala":
        expected.update(name + ".tasty" for name in
                        ("ScalaComplete", "ScalaFacade", "ScalaRequests"))
    require(set(members) == expected, f"{kind} JAR classes differ")
    require(not any(token in value for value in members.values() for token in
                    (b"/home/", b"/Users/", b"thinkthen_panic_probe", b"tt-canary-275",
                     b"-----BEGIN PRIVATE KEY-----")), f"private {kind} JAR byte")


def check_source(path, receipt_path, selected):
    source_receipt = receipt(receipt_path, ("commit", "sha256"))
    require(re.fullmatch(r"[0-9a-f]{40}", selected) is not None and
            source_receipt["commit"] == selected, "source receipt commit differs from resolved SHA")
    source_bytes = regular(path)
    pinned(source_bytes, source_receipt["sha256"], "source tar")
    _, headers, _ = tar_files(source_bytes, "source")
    require(headers.get("comment") == selected, "Git tar commit header differs")
    return source_bytes


# Windows command releases do not include the C or managed family at this stage.
# Tickets 0381, 0384 and 0385 add that family with their own target proof.
def check_c(path, receipt_path, target, version):
    captured = receipt(receipt_path, ("name", "sha256"))
    name = f"thinkthen-c-{version}-{target}.tar.gz"
    require(captured["name"] == name and path.name == name, "captured C basename differs")
    pinned(regular(path), captured["sha256"], "C archive")
    require(regular(path.with_name(name + ".sha256")) ==
            f"{captured['sha256']}  {name}\n".encode(), "C sidecar differs")
    return captured


def check(args):
    require(re.fullmatch(r"[A-Za-z0-9_]+-unknown-linux-gnu", args.target) is not None,
            "invalid Linux target")
    require(re.fullmatch(r"[0-9]+\.[0-9]+\.[0-9]+", args.version) is not None,
            "invalid version")
    source_receipt = receipt(args.source_receipt, ("commit", "sha256"))
    managed = receipt(args.managed_receipt, ("nupkg", "door", "kotlin", "scala"))
    require(re.fullmatch(r"[0-9a-f]{40}", source_receipt["commit"]) is not None,
            "invalid captured source commit")
    source_bytes = check_source(args.source_tar, args.source_receipt, source_receipt["commit"])
    source, _, _ = tar_files(source_bytes, "source")
    c_receipt = check_c(args.c_archive, args.c_receipt, args.target, args.version)
    c_name = c_receipt["name"]
    manifest = (f"source_commit={source_receipt['commit']}\ntarget={args.target}\n"
                f"version={args.version}\nc_archive={c_name}\nc_sha256={c_receipt['sha256']}\n").encode()
    nupkg = f"Botassembly.ThinkThen.{args.version}.nupkg"
    inner = {"nupkg": nupkg, **{kind: f"thinkthen-{kind}.jar" for kind in ("door", "kotlin", "scala")}}
    outer = {"csharp": f"thinkthen-csharp-{args.version}-{args.target}.tar.gz",
             "jvm": f"thinkthen-jvm-{args.version}-{args.target}.tar.gz"}
    if args.mode == "assemble":
        require(args.output.is_dir() and not args.output.is_symlink(), "output must be a real directory")
        for name in outer.values():
            require(not (args.output / name).exists() and not (args.output / name).is_symlink() and
                    not (args.output / (name + ".sha256")).exists() and
                    not (args.output / (name + ".sha256")).is_symlink(),
                    f"output already exists: {name}")
        source_paths = {"csharp": ("LICENSE", "README.md"),
                        "jvm": ("LICENSE", "README.md", "pom.xml")}
        for kind, name in inner.items():
            path = args.nupkg if kind == "nupkg" else args.jars / name
            data = regular(path)
            pinned(data, managed[kind], kind)
            if kind == "nupkg":
                check_nupkg(data, args.version, source)
            else:
                check_jar(data, kind)
        require(tuple(xml_field(source["libraries/jvm/pom.xml"], key) for key in
                      ("groupId", "artifactId", "version")) ==
                ("io.github.botassembly", "thinkthen-jvm", args.version), "source POM identity differs")
        import tempfile
        with tempfile.TemporaryDirectory(dir=args.output) as temporary:
            for family in outer:
                folder = Path(temporary) / family
                folder.mkdir()
                for name in source_paths[family]:
                    (folder / name).write_bytes(source[f"libraries/{family}/{name}"])
                (folder / "THINKTHEN-PACKAGE-INPUTS").write_bytes(manifest)
                keys = ("nupkg",) if family == "csharp" else ("door", "kotlin", "scala")
                for kind in keys:
                    path = args.nupkg if kind == "nupkg" else args.jars / inner[kind]
                    shutil.copyfile(path, folder / inner[kind])
                with tarfile.open(args.output / outer[family], "w:gz") as archive:
                    archive.add(folder, arcname=".")
                data = regular(args.output / outer[family])
                (args.output / (outer[family] + ".sha256")).write_text(
                    f"{digest(data)}  {outer[family]}\n")
    for family, name in outer.items():
        data = regular(args.output / name)
        require(regular(args.output / (name + ".sha256")) == f"{digest(data)}  {name}\n".encode(),
                f"{family} outer sidecar differs")
        names = {"THINKTHEN-PACKAGE-INPUTS", "LICENSE", "README.md"}
        names |= {nupkg} if family == "csharp" else {"pom.xml", *(inner[k] for k in ("door", "kotlin", "scala"))}
        files = archive_members(data, names, family)
        require(files["THINKTHEN-PACKAGE-INPUTS"] == manifest, f"{family} C identity differs")
        for member in ("README.md", "LICENSE"):
            require(files[member] == source[f"libraries/{family}/{member}"], f"{family} {member} differs")
        if family == "csharp":
            pinned(files[nupkg], managed["nupkg"], "nupkg")
            check_nupkg(files[nupkg], args.version, source)
        else:
            require(files["pom.xml"] == source["libraries/jvm/pom.xml"], "JVM POM differs from source")
            require(tuple(xml_field(files["pom.xml"], key) for key in ("groupId", "artifactId", "version")) ==
                    ("io.github.botassembly", "thinkthen-jvm", args.version), "JVM POM identity differs")
            for kind in ("door", "kotlin", "scala"):
                pinned(files[inner[kind]], managed[kind], f"{kind} JAR")
                check_jar(files[inner[kind]], kind)
    return True


def main():
    if len(sys.argv) == 5 and sys.argv[1] == "source-check":
        check_source(Path(sys.argv[2]), Path(sys.argv[3]), sys.argv[4])
        print("release-managed-pair: source-check pass")
        return
    if len(sys.argv) == 6 and sys.argv[1] == "c-check":
        check_c(Path(sys.argv[2]), Path(sys.argv[3]), sys.argv[4], sys.argv[5])
        print("release-managed-pair: c-check pass")
        return
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("mode", choices=("assemble", "verify"))
    for name in ("source-tar", "source-receipt", "c-archive", "c-receipt", "managed-receipt", "output"):
        parser.add_argument("--" + name, type=Path, required=True)
    parser.add_argument("--target", required=True)
    parser.add_argument("--version", required=True)
    parser.add_argument("--nupkg", type=Path)
    parser.add_argument("--jars", type=Path)
    args = parser.parse_args()
    if args.mode == "assemble":
        require(args.nupkg is not None and args.jars is not None, "assembly needs fresh managed paths")
    check(args)
    print(f"release-managed-pair: {args.mode} pass")


if __name__ == "__main__":
    try:
        main()
    except (ValueError, OSError, KeyError, tarfile.TarError, zipfile.BadZipFile, ET.ParseError) as error:
        print(f"release-managed-pair: {error}", file=sys.stderr)
        sys.exit(1)
