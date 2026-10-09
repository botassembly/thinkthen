"""Check local C# and native package bytes and plant stale, tampered and secret variants."""
import hashlib
import importlib.util
import os
import subprocess
import sys
import tempfile
import io
import json
from pathlib import Path
import re
import tarfile
import zipfile

sys.path.insert(0, str(Path(__file__).resolve().parent))
ROOT = Path(__file__).resolve().parents[1]
VERSION = re.search(r"<Version>([^<]+)</Version>", (ROOT / "ThinkThen.csproj").read_text())[1]
MAJOR, MINOR, PATCH = VERSION.split(".")
HEADER = ROOT.parents[1] / "libraries/c/include/thinkthen.h"
NATIVE = ROOT.parents[1] / "libraries/c/target/debug/libthinkthen_c.so"
NUPKG = ROOT / f"target/scratch/managed/Botassembly.ThinkThen.{VERSION}.nupkg"
ARCHIVE = ROOT / f"target/artifacts/thinkthen-c-{VERSION}-x86_64-linux-gnu.tar.gz"
BAD = (b"tt-canary-290", b"/home/", b"/Users/", b"auth.json", b"-----BEGIN PRIVATE KEY-----")


def digest(data):
    return hashlib.sha256(data).hexdigest()


def safe(label, data):
    assert not any(token in data for token in BAD), f"private pattern in {label}"


def inspect(header, package, native, expected=None):
    version = re.findall(r"(?m)^#define THINKTHEN_VERSION_(MAJOR|MINOR|PATCH)\s+(\d+)\s*$", header)
    assert version == [("MAJOR", MAJOR), ("MINOR", MINOR), ("PATCH", PATCH)], "header version mismatch"
    actual = {"header": digest(header.encode()), "package": digest(package), "native": digest(native)}
    if expected is not None:
        assert actual == expected, "artifact SHA-256 mismatch"
    with zipfile.ZipFile(io.BytesIO(package)) as bundle:
        members = set(bundle.namelist())
        assert {"Botassembly.ThinkThen.nuspec", "lib/net8.0/ThinkThen.dll", "README.md", "LICENSE"} <= members
        assert bundle.read('runtimes/linux-x64/native/libthinkthen.so') == NATIVE.read_bytes(), 'tampered packaged native asset'
        assert b"<id>Botassembly.ThinkThen</id>" in bundle.read("Botassembly.ThinkThen.nuspec")
        assert bundle.read("README.md") == (ROOT / "README.md").read_bytes(), "stale package README"
        for name in members:
            safe(name, name.encode() + bundle.read(name))
    with tarfile.open(fileobj=io.BytesIO(native), mode="r:gz") as bundle:
        files = {member.name: member for member in bundle}
        assert "./lib/libthinkthen.so" in files and "./lib/libthinkthen.so.0" in files
        assert files["./lib/libthinkthen.so.0"].issym()
        assert bundle.extractfile(files["./lib/libthinkthen.so"]).read() == NATIVE.read_bytes(), "tampered native member"
    return actual


def fail(label, action):
    try:
        action()
    except AssertionError:
        print(f"planted {label}: rejected")
    else:
        raise AssertionError(f"planted {label}: accepted")


def rewrite_zip(package, name, suffix):
    result = io.BytesIO()
    with zipfile.ZipFile(io.BytesIO(package)) as source, zipfile.ZipFile(result, "w") as altered:
        for member in source.infolist():
            data = source.read(member)
            altered.writestr(member, data + suffix if member.filename == name else data)
    return result.getvalue()


def rewrite_tar(native):
    result = io.BytesIO()
    with tarfile.open(fileobj=io.BytesIO(native), mode="r:gz") as source, tarfile.open(fileobj=result, mode="w:gz") as altered:
        for member in source:
            if member.isfile():
                data = source.extractfile(member).read()
                if member.name == "./lib/libthinkthen.so":
                    data += b"tampered"
                    member.size = len(data)
                altered.addfile(member, io.BytesIO(data))
            else:
                altered.addfile(member)
    return result.getvalue()



def abi_check(header, package):
    spec = importlib.util.spec_from_file_location('c_abi', ROOT.parents[1] / 'sdlc/scripts/check-c-exports.py')
    abi = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(abi)
    sys.path.insert(0, str(ROOT.parents[1] / 'conformance/children'))
    from children import child_env
    from toolchains import dotnet
    native = abi.header_abi(header)
    with tempfile.TemporaryDirectory(prefix='thinkthen-csharp-abi-') as folder:
        scratch = Path(folder)
        with zipfile.ZipFile(package) as bundle:
            (scratch / 'ThinkThen.dll').write_bytes(bundle.read('lib/net8.0/ThinkThen.dll'))
        (scratch / 'Probe.cs').write_text(CSHARP_ABI_PROBE)
        (scratch / 'Probe.csproj').write_text('<Project Sdk="Microsoft.NET.Sdk"><PropertyGroup><OutputType>Exe</OutputType><TargetFramework>net8.0</TargetFramework><ImplicitUsings>enable</ImplicitUsings></PropertyGroup></Project>')
        (scratch / 'NuGet.Config').write_text('<configuration><packageSources><clear /></packageSources></configuration>')
        env = child_env(DOTNET_CLI_HOME=str(scratch / 'home'), NUGET_PACKAGES=str(scratch / 'nuget'),
                        DOTNET_CLI_TELEMETRY_OPTOUT='1', DOTNET_SKIP_FIRST_TIME_EXPERIENCE='1', DOTNET_NOLOGO='1')
        subprocess.run([str(dotnet()), 'build', str(scratch / 'Probe.csproj'), '--configfile', str(scratch / 'NuGet.Config'), '-v', 'quiet', '-m:1'], env=env, check=True, stdout=sys.stderr)
        actual = json.loads(subprocess.check_output([str(dotnet()), str(scratch / 'bin/Debug/net8.0/Probe.dll'), str(scratch / 'ThinkThen.dll')], env=env, text=True))
    # Validate the sole owned SDK's imports against the full frozen C header.
    required_imports = {'thinkthen_engine_new_with', 'thinkthen_engine_free', 'thinkthen_error_code',
                        'thinkthen_error_retryable', 'thinkthen_error_message', 'thinkthen_error_facts_json',
                        'thinkthen_free_string', 'thinkthen_question_parse', 'thinkthen_question_free',
                        'thinkthen_request_plan_json'} | {name for name in native['functions'] if name.startswith('thinkthen_session_')}
    assert required_imports == actual['functions'].keys(), 'owned SDK native imports differ'
    assert set(actual['records']) == {'thinkthen_string_v1'}, 'unused native layouts retained'
    constant_names = {name for name in native['constants'] if name.startswith('THINKTHEN_LOAD_') or name.startswith('THINKTHEN_E') and not name.endswith('_V1')}
    expected = abi.represented_abi(native, {'thinkthen_string_v1'}, required_imports, constant_names)
    for name, prototype in actual['functions'].items():
        pointees = prototype.pop('argument_pointees')
        for parameter, represented in zip(native['functions'][name]['arguments'], pointees):
            if represented and abi.pointee_type(parameter, native) != represented:
                raise ValueError(f'C# typed reference differs from C: {name}')
    abi.compare_abi(expected, actual)
    print(f'C# C ABI: {len(actual["records"])} actual layouts, {len(actual["constants"])} represented constants, {len(actual["functions"])} reflected imports match')


def abi_plants(header):
    """Compile changed real declarations; the same package checker must refuse them."""
    sys.path.insert(0, str(ROOT.parents[1] / 'conformance/children'))
    from children import child_env
    from toolchains import dotnet
    with tempfile.TemporaryDirectory(prefix='thinkthen-csharp-abi-plants-') as folder:
        scratch = Path(folder)
        source = scratch / 'src'
        source.mkdir()
        for original in (ROOT / 'src').glob('*.cs'):
            (source / original.name).write_bytes(original.read_bytes())
        project = scratch / 'ThinkThen.csproj'
        project.write_text('<Project Sdk="Microsoft.NET.Sdk"><PropertyGroup><TargetFramework>net8.0</TargetFramework><ImplicitUsings>enable</ImplicitUsings><Nullable>enable</Nullable></PropertyGroup></Project>')
        generated = ROOT.parents[1] / 'sdlc/generators/results/csharp/CompleteFacts.g.cs'
        (source / generated.name).write_bytes(generated.read_bytes())
        config = scratch / 'NuGet.Config'
        config.write_text('<configuration><packageSources><clear /></packageSources></configuration>')
        env = child_env(DOTNET_CLI_HOME=str(scratch / 'home'), NUGET_PACKAGES=str(scratch / 'nuget'),
                        DOTNET_CLI_TELEMETRY_OPTOUT='1', DOTNET_SKIP_FIRST_TIME_EXPERIENCE='1', DOTNET_NOLOGO='1')
        plants = [('field order', 'NativeBridge.g.cs', 'public IntPtr data;\n public nuint len;', 'public nuint len;\n public IntPtr data;'),
                  ('enum', 'NativeBridge.g.cs', 'Atomic = 1', 'Atomic = 19'),
                  ('return', 'NativeSession.cs', 'extern void thinkthen_session_cancel(', 'extern long thinkthen_session_cancel(')]
        for name, file, before, after in plants:
            copied = source / file
            text = copied.read_text()
            if before not in text:
                raise ValueError(f'C# ABI plant has no declaration: {name}')
            copied.write_text(text.replace(before, after, 1))
            subprocess.run([str(dotnet()), 'build', str(project), '--configfile', str(config), '-v', 'quiet', '-m:1'], env=env, check=True, stdout=sys.stderr)
            package = scratch / 'plant.nupkg'
            with zipfile.ZipFile(package, 'w') as bundle:
                bundle.write(scratch / 'bin/Debug/net8.0/ThinkThen.dll', 'lib/net8.0/ThinkThen.dll')
            try:
                abi_check(header, package)
            except ValueError as error:
                if not str(error).startswith('C ABI mismatch:'):
                    raise
            else:
                raise ValueError(f'C# ABI drift accepted: {name}')
            copied.write_text(text)
            print(f'C# ABI {name} drift refused')


CSHARP_ABI_PROBE = r'''
using System.Reflection;
using System.Reflection.Emit;
using System.Runtime.InteropServices;
using System.Text.Json;
using System.Text.RegularExpressions;
class Probe {
 static string Name(Type t) => "thinkthen_" + Regex.Replace(t.Name,"(?<!^)(?=[A-Z])","_").ToLowerInvariant();
 static int Width(Type t) => t==typeof(void)?0:t.IsByRef||t.IsArray||typeof(SafeHandle).IsAssignableFrom(t)||t==typeof(IntPtr)||t==typeof(UIntPtr)?IntPtr.Size:Marshal.SizeOf(t);
 static string Kind(Type t) {
  if(t.IsByRef||t.IsArray||typeof(SafeHandle).IsAssignableFrom(t)||t==typeof(IntPtr))return "pointer";
  if(t==typeof(UIntPtr))return "unsigned"+(IntPtr.Size*8);
  if(t==typeof(void))return "void";
  if(t.Name.EndsWith("V1Data"))return "union";
  if(t==typeof(float)||t==typeof(double))return "float"+(Width(t)*8);
  if(t.IsPrimitive)return (t==typeof(byte)||t==typeof(ushort)||t==typeof(uint)||t==typeof(ulong)?"unsigned":"signed")+(Width(t)*8);
  return Name(t);
 }
 static Dictionary<string,object> Fields(Type t,string prefix="",long baseOffset=0) {
  var output=new Dictionary<string,object>();
  foreach(var f in t.GetFields(BindingFlags.Instance|BindingFlags.Public|BindingFlags.NonPublic)) {
   string n=prefix+f.Name.ToLowerInvariant();long offset=baseOffset+Marshal.OffsetOf(t,f.Name).ToInt64();
   output.Add(n,new {type=Kind(f.FieldType),offset,width=Width(f.FieldType)});
   if(f.FieldType.Name.EndsWith("V1Data"))foreach(var nested in Fields(f.FieldType,n+".",offset))output.Add(nested.Key,nested.Value);
  }return output;
 }
 static void Main(string[] args) {
  var assembly=Assembly.LoadFrom(args[0]);var records=new Dictionary<string,object>();var functions=new Dictionary<string,object>();
  var engine=assembly.GetType("ThinkThen.Engine",true)!;
  if(engine.GetMethods(BindingFlags.Public|BindingFlags.Instance|BindingFlags.Static).Any(m=>m.Name is "Call" or "CallTyped" or "Decide" or "DecideMany" or "Recognize" or "Relate" || m.Name.EndsWith("Complete") || m.Name.EndsWith("Batch")) || engine.GetMethods().Where(m=>m.Name=="Open").Any(m=>m.GetParameters().Any(p=>p.ParameterType==typeof(string))))throw new Exception("retired public execution API retained");
  foreach(string old in new[]{"Native","CompleteReaders","Requests","Questions","ICompleteEngine","CompleteRequest","Answer","Outcome"})if(assembly.GetType("ThinkThen."+old) is {IsPublic:true})throw new Exception("retired public type retained: "+old);
  var dynamicAssembly=AssemblyBuilder.DefineDynamicAssembly(new AssemblyName("AbiAlign"),AssemblyBuilderAccess.Run);
  dynamicAssembly.SetCustomAttribute(new CustomAttributeBuilder(typeof(System.Runtime.CompilerServices.IgnoresAccessChecksToAttribute).GetConstructor(new[]{typeof(string)})!,new object[]{assembly.GetName().Name!}));
  var module=dynamicAssembly.DefineDynamicModule("align");
  foreach(var t in assembly.GetTypes()) {
   if(t.Namespace=="ThinkThen"&&t.IsValueType&&!t.IsEnum&&(t.Name.EndsWith("V1")||t.Name=="Answer")) {
    var holder=module.DefineType("Align"+t.Name,TypeAttributes.Public|TypeAttributes.SequentialLayout|TypeAttributes.Sealed,typeof(ValueType));
    holder.DefineField("prefix",typeof(byte),FieldAttributes.Public);holder.DefineField("value",t,FieldAttributes.Public);
    var aligned=holder.CreateType()!;
    records.Add(Name(t),new {size=Marshal.SizeOf(t),alignment=Marshal.OffsetOf(aligned,"value").ToInt64(),fields=Fields(t)});
   }
   foreach(var m in t.GetMethods(BindingFlags.Static|BindingFlags.Public|BindingFlags.NonPublic)) {
    var import=m.GetCustomAttribute<DllImportAttribute>();if(import==null)continue;
    string n=import.EntryPoint??m.Name;var parameters=m.GetParameters();
    string convention=import.CallingConvention switch {CallingConvention.Cdecl=>OperatingSystem.IsWindows()?"cdecl":"C",CallingConvention.Winapi=>OperatingSystem.IsWindows()?"stdcall":"C",_=>import.CallingConvention.ToString()};
    var signature=new Dictionary<string,object>{{"return",Kind(m.ReturnType)},{"return_width",Width(m.ReturnType)},{"arguments",parameters.Select(p=>Kind(p.ParameterType)).ToArray()},{"argument_widths",parameters.Select(p=>Width(p.ParameterType)).ToArray()},{"calling_convention",convention},{"argument_pointees",parameters.Select(p=>p.ParameterType.IsByRef?Kind(p.ParameterType.GetElementType()!):"").ToArray()}};
    if(functions.TryGetValue(n,out var prior)&&JsonSerializer.Serialize(prior)!=JsonSerializer.Serialize(signature))throw new Exception("conflicting native import "+n);
    functions[n]=signature;
   }
  }
  var constants=new Dictionary<string,int>();
  var authored=assembly.GetType("ThinkThen.AuthoredQuestionKind",true)!;
  foreach(var value in Enum.GetValues(authored))constants.Add("THINKTHEN_LOAD_"+Regex.Replace(Enum.GetName(authored,value)!,"(?<!^)(?=[A-Z])","_").ToUpperInvariant()+"_V1",Convert.ToInt32(value));
  var failure=assembly.GetType("ThinkThen.FailureKind",true)!;
  foreach(var value in Enum.GetValues(failure))constants.Add("THINKTHEN_E"+Enum.GetName(failure,value)!.ToUpperInvariant(),Convert.ToInt32(value));
  Console.WriteLine(JsonSerializer.Serialize(new {records,functions,constants}));
 }
}

namespace System.Runtime.CompilerServices { [AttributeUsage(AttributeTargets.Assembly)] public sealed class IgnoresAccessChecksToAttribute(string assemblyName):Attribute { public string AssemblyName {get;}=assemblyName; } }
'''



def main():
    if len(sys.argv) == 3:
        abi_check(Path(sys.argv[1]), Path(sys.argv[2]))
        return
    header, package, native = HEADER.read_text(), NUPKG.read_bytes(), ARCHIVE.read_bytes()
    receipt = inspect(header, package, native)
    (ROOT / "target/artifacts/manifest.json").write_text(json.dumps(receipt, indent=2) + "\n")
    fail("header-version", lambda: inspect(header.replace(f"#define THINKTHEN_VERSION_PATCH {PATCH}", f"#define THINKTHEN_VERSION_PATCH {int(PATCH) + 1}"), package, native))
    stale = rewrite_zip(package, "README.md", b"\nstale\n")
    fail("stale-package-member", lambda: inspect(header, stale, native))
    fail("stale-package-hash", lambda: inspect(header, stale, native, receipt))
    tampered = rewrite_tar(native)
    fail("tampered-native-member", lambda: inspect(header, package, tampered))
    fail("tampered-native-hash", lambda: inspect(header, package, tampered, receipt))
    fail("private-byte", lambda: safe("plant", b"/home/private/file"))
    secret_zip = rewrite_zip(package, "README.md", b"tt-canary-290")
    fail("compressed-private-byte", lambda: inspect(header, secret_zip, native))
    print("C# local packages: source members, hash receipt, header version, managed secrecy and planted negatives PASS")

    abi_check(HEADER, NUPKG)

    abi_plants(HEADER)


if __name__ == '__main__':
    main()
