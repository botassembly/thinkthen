"""Validate local JAR contents against this build and plant stale and private members."""
import hashlib
import importlib.util
import subprocess
import sys
import tempfile
import io
import json
import os
from pathlib import Path
import re
import zipfile
import xml.etree.ElementTree as ET

ROOT = Path(__file__).resolve().parents[1]
# Every package shares the command crate's version (ticket 0376).
VERSION = re.search(r'(?m)^version = "([^"]+)"$', (ROOT.parents[1] / "crates/thinkthen/Cargo.toml").read_text())[1]
TARGET = Path(os.environ.get("THINKTHEN_JVM_OUT", ROOT / "target"))
BAD = (b"tt-canary-289", b"/home/", b"/Users/", b"auth.json", b"-----BEGIN PRIVATE KEY-----")


def inspect(name, source):
    with zipfile.ZipFile(io.BytesIO(source)) as bundle:
        members = {item.filename: bundle.read(item) for item in bundle.infolist() if not item.is_dir()}
    assert members, f"empty {name} JAR"
    legacy = {"Door", "Complete", "CompleteDetails", "CompleteEngine", "Requests", "Questions", "Ids", "KotlinComplete", "KotlinFacade", "KotlinCallerKt", "KotlinRequests", "ScalaComplete", "ScalaFacade", "ScalaCaller", "ScalaRequests"}
    assert not any(Path(member).name.split("$")[0].removesuffix(".class") in legacy for member in members if member.endswith(".class")), "retired public class escaped product JAR"
    demo_classes = {"KotlinCallerKt.class", "ScalaCaller$package.class",
                    "ScalaCaller$package$.class", "scalaCaller.class",
                    "ScalaCaller$package.tasty", "scalaCaller.tasty"}
    assert not demo_classes.intersection(members), "demo entrypoint escaped product JAR"
    spec = importlib.util.spec_from_file_location("package_inventory", ROOT.parents[1] / "sdlc/scripts/package-inventory.py")
    inventory = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(inventory)
    expected = {member: (TARGET / "classes" / name / member).read_bytes()
                for member in inventory.jvm_inventory(TARGET)['members'][name]}
    manifest = members.pop("META-INF/MANIFEST.MF", None)
    assert manifest is None or manifest.startswith(b"Manifest-Version: 1.0"), "bad JAR manifest"
    if "META-INF/thinkthen/product-inventory.json" in members:
        assert json.loads(members.pop("META-INF/thinkthen/product-inventory.json")) == inventory.jvm_inventory(), "stale embedded JVM inventory"
    assert members == expected, f"stale or extra {name} JAR member"
    assert not any("ProbeDoor" in member or "TypeCase" in member for member in members), "diagnostic class escaped product JAR"
    assert not any(token in data for member, content in members.items() for data in (member.encode(), content) for token in BAD), "private byte in JAR"
    return hashlib.sha256(source).hexdigest()


def reject(label, action):
    try:
        action()
    except AssertionError:
        print(f"planted {label}: rejected")
    else:
        raise AssertionError(f"planted {label}: accepted")



def session_abi_check(header, jar, library):
    """Compile the layout expressions at actual session call sites, without calling them."""
    spec = importlib.util.spec_from_file_location('c_abi', ROOT.parents[1] / 'sdlc/scripts/check-c-exports.py')
    abi = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(abi)
    sys.path.insert(0, str(ROOT.parents[1] / "conformance/children"))
    from children import child_env
    declarations = {}
    sources = list((ROOT / 'session/thinkthen').glob('*.java'))
    text = '\n'.join(p.read_text() for p in sources)
    # The free helper is itself an actual descriptor declaration, shared by its callers.
    size = re.search(r'static final [\w.]+ SIZE = ([^;]+);', text)[1].replace('LINKER', 'linker')
    free = re.search(r'call\(name, (null), new MemoryLayout\[\]\{([^}]*)\}', text)
    for source in sources:
        unit = source.read_text()
        for found in re.finditer(r'(?:NativeSession\.)?call\("(thinkthen_\w+)",\s*([\w.]+),\s*(new MemoryLayout\[\]\{[^}]*\}|\w+)', unit):
            name, returned, arguments = found.groups()
            if not arguments.startswith('new '):
                definitions = list(re.finditer(r'(?:var|MemoryLayout\[\])\s+' + arguments + r'\s*=\s*(new MemoryLayout\[\]\{[^}]*\})', unit[:found.start()]))
                if not definitions:
                    raise ValueError('C ABI mismatch: JVM session unresolved layout ' + name)
                arguments = definitions[-1][1]
            declaration = (returned, arguments)
            if name in declarations and declarations[name] != declaration:
                raise ValueError('C ABI mismatch: JVM session conflicting declaration ' + name)
            declarations[name] = declaration
        for name in re.findall(r'NativeSession\.free\("(thinkthen_\w+)"', unit):
            declarations[name] = (free[1], 'new MemoryLayout[]{' + free[2] + '}')
    usage = re.search(r'call\(function,\s*([\w.]+),\s*(new MemoryLayout\[\]\{[^}]*\})', text)
    for name in re.findall(r'NativeSession\.usage\(pointer, "(thinkthen_\w+)"', text):
        declarations[name] = (usage[1], usage[2])
    required = set(re.findall(r'"(thinkthen_\w+)"', text))
    if required != set(declarations):
        raise ValueError('C ABI mismatch: JVM session unmeasured import')
    lines = [JVM_ABI_PROBE.split(' public static void main(')[0],
             ' public static void main(String[] args)throws Exception {',
             ' var linker=Linker.nativeLinker();',
             ' var SIZE=' + size + ';',
             ' var symbols=SymbolLookup.libraryLookup(args[0],Arena.global());',
             ' Map<String,Object> functions=new TreeMap<>();']
    for name, (returned, arguments) in sorted(declarations.items()):
        returned, arguments = returned.replace('NativeSession.SIZE', 'SIZE'), arguments.replace('NativeSession.SIZE', 'SIZE')
        descriptor = f'FunctionDescriptor.ofVoid({arguments})' if returned == 'null' else f'FunctionDescriptor.of({returned},{arguments})'
        lines += ['{ var d=' + descriptor + ';',
                  ' linker.downcallHandle(symbols.find("' + name + '").orElseThrow(),d);',
                  ' functions.put("' + name + '",Map.of("return",d.returnLayout().map(AbiProbe::kind).orElse("void"),"return_width",d.returnLayout().map(MemoryLayout::byteSize).orElse(0L),"arguments",d.argumentLayouts().stream().map(AbiProbe::kind).toList(),"argument_widths",d.argumentLayouts().stream().map(MemoryLayout::byteSize).toList(),"calling_convention",System.getProperty("os.name").startsWith("Windows")?"cdecl":"C")); }']
    lines += [' System.out.println(Json.write(Map.of("functions",functions)));', ' }', '}']
    jdk = Path(os.environ.get('THINKTHEN_SESSION_JDK_HOME', os.environ.get('THINKTHEN_JDK_HOME', '/usr/lib/jvm/java-22-openjdk-amd64')))
    with tempfile.TemporaryDirectory(prefix='thinkthen-jvm-session-abi-') as folder:
        scratch = Path(folder)
        unit = scratch / 'AbiProbe.java'
        unit.write_text('\n'.join(lines))
        env = child_env(HOME=str(scratch / 'home'))
        subprocess.run([str(jdk / 'bin/javac'), '--release', '22', '-cp', str(jar), '-d', str(scratch), str(unit)], env=env, check=True, stdout=sys.stderr)
        actual = json.loads(subprocess.check_output([str(jdk / 'bin/java'), '--enable-native-access=ALL-UNNAMED', '-XX:ActiveProcessorCount=1', '-cp', str(jar) + os.pathsep + str(scratch), 'thinkthen.AbiProbe', str(library.resolve())], env=env, text=True))
    native = abi.header_abi(header)
    expected = abi.represented_abi(native, [], declarations, [], signed=False)
    abi.compare_abi(expected, actual)
    print(f'JVM session C ABI: {len(declarations)} source-declared, compiled and linked descriptors match')


JVM_ABI_PROBE = r'''
package thinkthen;
import java.lang.foreign.*;
import java.util.*;
public class AbiProbe {
 static String kind(MemoryLayout l) {
  if(l instanceof AddressLayout)return "pointer";
  if(l instanceof UnionLayout)return "union";
  if(l instanceof GroupLayout)return "record";
  if(l instanceof ValueLayout.OfDouble||l instanceof ValueLayout.OfFloat)return "float"+(l.byteSize()*8);
  if(l instanceof ValueLayout)return "integer"+(l.byteSize()*8);
  throw new IllegalStateException("unrepresented ABI layout "+l);
 }
 public static void main(String[] args){}
}
'''


def main():
    if len(sys.argv) == 4:
        session_abi_check(*map(Path, sys.argv[1:]))
        return
    pom = ET.fromstring((ROOT / "pom.xml").read_text())
    metadata = {child.tag.rsplit("}", 1)[-1]: (child.text or "").strip() for child in pom}
    assert metadata["groupId"] == "io.github.botassembly" and metadata["artifactId"] == "thinkthen-jvm" and metadata["version"] == VERSION
    receipt = {}
    spec = importlib.util.spec_from_file_location("package_inventory", ROOT.parents[1] / "sdlc/scripts/package-inventory.py")
    inventory = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(inventory)
    definition = inventory.jvm_inventory(TARGET)
    assert json.loads((TARGET / 'jars/product-inventory.json').read_text()) == definition, 'stale generated JVM inventory'
    for name, filename in definition['jars'].items():
        jar = TARGET / "jars" / filename
        source = jar.read_bytes()
        receipt[name] = inspect(name, source)
        altered = io.BytesIO()
        with zipfile.ZipFile(io.BytesIO(source)) as original, zipfile.ZipFile(altered, "w") as copy:
            for item in original.infolist():
                data = original.read(item)
                copy.writestr(item, data + b"stale" if item.filename.endswith(".class") and item.filename == next(k for k in original.namelist() if k.endswith(".class")) else data)
        reject(f"stale-{name}-class", lambda: inspect(name, altered.getvalue()))
        secret = io.BytesIO()
        with zipfile.ZipFile(io.BytesIO(source)) as original, zipfile.ZipFile(secret, "w") as copy:
            for item in original.infolist():
                copy.writestr(item, original.read(item))
            copy.writestr("private.txt", b"tt-canary-289")
        reject(f"compressed-private-{name}", lambda: inspect(name, secret.getvalue()))
    (TARGET / "jars/manifest.json").write_text(json.dumps(receipt, indent=2) + "\n")
    print("JVM JARs: exact compiled members, metadata, no diagnostic exports and planted negatives PASS")
    if '--session' in sys.argv:
        return
    native = os.environ.get('THINKTHEN_RELEASE_C_DIR')
    header = Path(native) / 'include/thinkthen.h' if native else ROOT.parents[1] / 'libraries/c/include/thinkthen.h'
    library = Path(native) / 'lib/libthinkthen.so' if native else ROOT.parents[1] / 'libraries/c/target/debug/libthinkthen_c.so'
    session_abi_check(header, TARGET / 'jars/thinkthen-door.jar', library)


if __name__ == '__main__':
    main()
