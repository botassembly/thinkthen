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
    expected = {path.relative_to(TARGET / "classes" / name).as_posix(): path.read_bytes()
                for path in (TARGET / "classes" / name).rglob("*") if path.is_file()}
    assert members.pop("META-INF/MANIFEST.MF").startswith(b"Manifest-Version: 1.0"), "bad JAR manifest"
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



def descriptor_shape(native, abi, kind):
    """Describe C by-value carriers recursively; FFM erases nominal identity."""
    if kind not in native['records']:
        return {}
    record = native['records'][kind]
    fields = {}
    for name, field in record['fields'].items():
        if field['type'] == 'union':
            raise ValueError('C ABI mismatch: unrepresented by-value anonymous union ' + kind)
        nested = descriptor_shape(native, abi, field['type'])
        fields[name] = {'offset': field['offset'], 'width': field['width'],
                        'type': 'record' if nested else abi.wire_type(field['type'], field['width'], signed=False),
                        'shape': nested}
    return {'size': record['size'], 'alignment': record['alignment'], 'fields': fields}


def abi_check(header, jar, library):
    spec = importlib.util.spec_from_file_location('c_abi', ROOT.parents[1] / 'sdlc/scripts/check-c-exports.py')
    abi = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(abi)
    sys.path.insert(0, str(ROOT.parents[1] / 'conformance/children'))
    from children import child_env
    # ABI checks need only the actual JDK used by this public FFM boundary.
    jdk = Path(os.environ.get('THINKTHEN_JDK_HOME', '/usr/lib/jvm/java-21-openjdk-amd64'))
    native = abi.header_abi(header)
    with tempfile.TemporaryDirectory(prefix='thinkthen-jvm-abi-') as folder:
        scratch = Path(folder)
        unit = scratch / 'AbiProbe.java'
        unit.write_text(JVM_ABI_PROBE)
        env = child_env(HOME=str(scratch / 'home'))
        subprocess.run([str(jdk / 'bin/javac'), '--enable-preview', '--release', '21', '-cp', str(jar), '-d', str(scratch), str(unit)], env=env, check=True, stdout=sys.stderr)
        command = [str(jdk / 'bin/java'), '--enable-preview', '--enable-native-access=ALL-UNNAMED', '-XX:ActiveProcessorCount=1',
                   '-Dthinkthen.library=' + str(library.resolve()), '-cp', str(jar) + os.pathsep + str(scratch), 'thinkthen.AbiProbe']
        actual = json.loads(subprocess.check_output(command, text=True, env=env))
    omitted = {'thinkthen_call', 'thinkthen_decide', 'thinkthen_decide_many', 'thinkthen_decide_many_with_facts',
               'thinkthen_decide_with_facts', 'thinkthen_image_view', 'thinkthen_question_author', 'thinkthen_question_file',
               'thinkthen_recognize', 'thinkthen_recognize_with_facts', 'thinkthen_relate', 'thinkthen_relate_with_facts', 'thinkthen_result_row'}
    constant_names = {n for n in native['constants'] if n.endswith('_V1') and not n.startswith(('THINKTHEN_PROBABILITIES_', 'THINKTHEN_RESULT_'))} | {n for n in native['constants'] if n.startswith('THINKTHEN_E') and not n.endswith('_V1')} | {'THINKTHEN_YES', 'THINKTHEN_NO', 'THINKTHEN_UNSURE'}
    expected = abi.represented_abi(native, native['records'], set(native['functions']) - omitted, constant_names, signed=False)
    # Java FFM layouts have structural carrier identity and no unsigned integers.
    for record in expected['records'].values():
        for field in record['fields'].values():
            if field['type'].startswith('thinkthen_'):
                field['type'] = 'record'
    for name, function in expected['functions'].items():
        declaration = native['functions'][name]
        function['return_shape'] = descriptor_shape(native, abi, declaration['return'])
        function['argument_shapes'] = [descriptor_shape(native, abi, kind) for kind in declaration['arguments']]
        function['return'] = 'record' if function['return_shape'] else function['return']
        function['arguments'] = ['record' if shape else kind for kind, shape in zip(function['arguments'], function['argument_shapes'])]
    abi.compare_abi(expected, actual)
    print(f'JVM C ABI: {len(actual["records"])} actual JAR layouts, {len(actual["constants"])} represented constants, {len(actual["functions"])} linked descriptors match')


def abi_plants(header, library):
    """Compile changed actual Java declarations and reject their JAR facts."""
    sys.path.insert(0, str(ROOT.parents[1] / 'conformance/children'))
    from children import child_env
    jdk = Path(os.environ.get('THINKTHEN_JDK_HOME', '/usr/lib/jvm/java-21-openjdk-amd64'))
    with tempfile.TemporaryDirectory(prefix='thinkthen-jvm-abi-plants-') as folder:
        scratch = Path(folder)
        source = scratch / 'thinkthen'
        source.mkdir()
        for original in (ROOT / 'door/thinkthen').glob('*.java'):
            (source / original.name).write_bytes(original.read_bytes())
        env = child_env(HOME=str(scratch / 'home'))
        plants = [('field order', 'NativeLayouts0.java', 'ADDRESS.withName("data"), JAVA_LONG.withName("len")', 'JAVA_LONG.withName("len"), ADDRESS.withName("data")'),
                  ('enum', 'Complete.java', 'enum Function { DECIDE, CHOOSE,', 'enum Function { CHOOSE, DECIDE,'),
                  ('return', 'NativeCalls.java', '"thinkthen_result_rank_member_details",FunctionDescriptor.of(JAVA_INT,', '"thinkthen_result_rank_member_details",FunctionDescriptor.of(JAVA_LONG,'),
                  ('equal-size aggregate', 'NativeCalls.java', 'for(String n:List.of("thinkthen_question_load"))add(linker,symbols,n,FunctionDescriptor.of(JAVA_INT,ADDRESS,layout("thinkthen_string_v1"),ADDRESS))', 'for(String n:List.of("thinkthen_question_load"))add(linker,symbols,n,FunctionDescriptor.of(JAVA_INT,ADDRESS,layout("thinkthen_optional_double_v1"),ADDRESS))'),
                  ('nested aggregate', 'NativeCalls.java', 'layout("thinkthen_optional_string_v1"),ADDRESS)', 'structure(JAVA_INT.withName("present"),layout("thinkthen_optional_double_v1").withName("value")),ADDRESS)')]
        for name, file, before, after in plants:
            copied = source / file
            text = copied.read_text()
            if before not in text:
                raise ValueError(f'JVM ABI plant has no declaration: {name}')
            copied.write_text(text.replace(before, after, 1))
            classes = scratch / 'classes'
            subprocess.run([str(jdk / 'bin/javac'), '--enable-preview', '--release', '21', '-d', str(classes), *map(str, source.glob('*.java'))], env=env, check=True, stdout=sys.stderr)
            jar = scratch / 'plant.jar'
            subprocess.run([str(jdk / 'bin/jar'), '--create', '--file', str(jar), '-C', str(classes), '.'], env=env, check=True)
            try:
                abi_check(header, jar, library)
            except ValueError as error:
                if not str(error).startswith('C ABI mismatch:'):
                    raise
            else:
                raise ValueError(f'JVM ABI drift accepted: {name}')
            copied.write_text(text)
            print(f'JVM ABI {name} drift refused')


JVM_ABI_PROBE = r'''
package thinkthen;
import java.lang.foreign.*;
import java.lang.reflect.*;
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
 static Map<String,Object> fields(GroupLayout l,String prefix,long base) {
  Map<String,Object> result=new TreeMap<>();
  for(MemoryLayout f:l.memberLayouts())if(f.name().isPresent()) {
   String name=f.name().orElseThrow();long offset=base+l.byteOffset(MemoryLayout.PathElement.groupElement(name));
   result.put(prefix+name,Map.of("type",kind(f),"offset",offset,"width",f.byteSize()));
   if(f instanceof UnionLayout u)result.putAll(fields(u,prefix+name+".",offset));
  }return result;
 }
 static Map<String,Object> shape(MemoryLayout layout) {
  if(!(layout instanceof GroupLayout group))return Map.of();
  Map<String,Object> fields=new TreeMap<>();
  for(MemoryLayout field:group.memberLayouts())if(field.name().isPresent()) {
   String name=field.name().orElseThrow();
   fields.put(name,Map.of("offset",group.byteOffset(MemoryLayout.PathElement.groupElement(name)),"width",field.byteSize(),"type",kind(field),"shape",shape(field)));
  }
  return Map.of("size",group.byteSize(),"alignment",group.byteAlignment(),"fields",fields);
 }
 static Object value(Class<?> c,String field)throws Exception{Field f=c.getDeclaredField(field);f.setAccessible(true);return f.get(null);}
 public static void main(String[] args)throws Exception {
  Map<String,MemoryLayout> layouts=new TreeMap<>(NativeLayouts.L);layouts.put("thinkthen_answer",(MemoryLayout)value(Door.class,"ANSWER"));
  Map<String,Object> records=new TreeMap<>(),functions=new TreeMap<>();
  for(var entry:layouts.entrySet()) {
   if(entry.getKey().endsWith("_v1_data"))continue;
   MemoryLayout l=entry.getValue();records.put(entry.getKey(),Map.of("size",l.byteSize(),"alignment",l.byteAlignment(),"fields",fields((GroupLayout)l,"",0)));
  }
  for(Class<?> owner:List.of(NativeCalls.class,Door.class)) {
   Map<?,?> calls=(Map<?,?>)value(owner,"CALLS");
   for(var entry:calls.entrySet()) {
    Method method=entry.getValue().getClass().getDeclaredMethod("descriptor");method.setAccessible(true);
    FunctionDescriptor d=(FunctionDescriptor)method.invoke(entry.getValue());
    Map<String,Object> fact=Map.of("return",d.returnLayout().map(AbiProbe::kind).orElse("void"),"return_width",d.returnLayout().map(MemoryLayout::byteSize).orElse(0L),"arguments",d.argumentLayouts().stream().map(AbiProbe::kind).toList(),"argument_widths",d.argumentLayouts().stream().map(MemoryLayout::byteSize).toList(),"calling_convention",System.getProperty("os.name").startsWith("Windows")?"cdecl":"C","return_shape",d.returnLayout().map(AbiProbe::shape).orElse(Map.of()),"argument_shapes",d.argumentLayouts().stream().map(AbiProbe::shape).toList());
    if(functions.containsKey(entry.getKey())&&!functions.get(entry.getKey()).equals(fact))throw new IllegalStateException("conflicting import "+entry.getKey());
    functions.put((String)entry.getKey(),fact);
   }
  }
  Map<String,Integer> constants=new TreeMap<>();
  for(String spec:List.of("Function:FUNCTION:1","RuleKind:RULE:0","Media:IMAGE:1","SourceUnit:SOURCE:1","ValueKind:DECIDE:0","AtomicKind:ANSWER:1","MemberState:MEMBER:1","MemberCause:MEMBER:1","Origin:ORIGIN:1","AttemptOutcome:ATTEMPT:1","RelationMethod:RELATION:1","Direction:DIRECTION:1","Stage:STAGE:1","IdentityKind:ID:1","StopCause:STOP:1","BatchKind:BATCH:1","EventKind:EVENT:1")) {
   String[] bits=spec.split(":");Class<?> type=Class.forName("thinkthen.Complete$"+bits[0]);
   for(Object v:type.getEnumConstants()){Enum<?> e=(Enum<?>)v;constants.put("THINKTHEN_"+bits[1]+"_"+e.name()+"_V1",e.ordinal()+Integer.parseInt(bits[2]));}
  }
  for(String spec:List.of("CompleteDetails$DeclarationKind:DECLARATION:0","CompleteDetails$PropertyKind:PROPERTY:1","Requests$QuestionRole:LOAD:1")) {
   String[] bits=spec.split(":");for(Object v:Class.forName("thinkthen."+bits[0]).getEnumConstants()){Enum<?> e=(Enum<?>)v;constants.put("THINKTHEN_"+bits[1]+"_"+e.name()+"_V1",e.ordinal()+Integer.parseInt(bits[2]));}
  }
  for(var e:Complete.ContentKind.values())if(e!=Complete.ContentKind.ABSENT)constants.put("THINKTHEN_CONTENT_"+e.name()+"_V1",e.ordinal()+1);
  for(var e:Door.FailureKind.values())constants.put("THINKTHEN_E"+e.name(),e.ordinal()+1);
  constants.put("THINKTHEN_YES",Door.Outcome.YES.ordinal());constants.put("THINKTHEN_NO",Door.Outcome.NO.ordinal());constants.put("THINKTHEN_UNSURE",Door.Outcome.NOT_SURE.ordinal());
  System.out.println(Json.write(Map.of("records",records,"functions",functions,"constants",constants)));
 }
}
'''


def main():
    if len(sys.argv) == 4:
        abi_check(*map(Path, sys.argv[1:]))
        return
    pom = ET.fromstring((ROOT / "pom.xml").read_text())
    metadata = {child.tag.rsplit("}", 1)[-1]: (child.text or "").strip() for child in pom}
    assert metadata["groupId"] == "io.github.botassembly" and metadata["artifactId"] == "thinkthen-jvm" and metadata["version"] == VERSION
    receipt = {}
    for name in ("door", "kotlin", "scala"):
        jar = TARGET / "jars" / f"thinkthen-{name}.jar"
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
    native = os.environ.get('THINKTHEN_RELEASE_C_DIR')
    header = Path(native) / 'include/thinkthen.h' if native else ROOT.parents[1] / 'libraries/c/include/thinkthen.h'
    library = Path(native) / 'lib/libthinkthen.so' if native else ROOT.parents[1] / 'libraries/c/target/debug/libthinkthen_c.so'
    abi_check(header, TARGET / 'jars/thinkthen-door.jar', library)
    abi_plants(header, library)


if __name__ == '__main__':
    main()
