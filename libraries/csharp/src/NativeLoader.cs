using System.Reflection;
using System.Runtime.InteropServices;
namespace ThinkThen;
internal static class NativeLoader
{
    [System.Diagnostics.CodeAnalysis.SuppressMessage("Usage", "CA2255", Justification = "The binding must resolve native assets before any native operation.")]
    [System.Runtime.CompilerServices.ModuleInitializer]
    internal static void Initialize() => NativeLibrary.SetDllImportResolver(typeof(Engine).Assembly, Resolve);
    private static readonly Lazy<IntPtr> Library = new(() => Load(typeof(Engine).Assembly));
    [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
    private delegate IntPtr Version(out uint number);
    private static IntPtr Resolve(string name, Assembly assembly, DllImportSearchPath? search)
    {
        if (name != "thinkthen" && name != "libthinkthen.so.0")
            throw new DllNotFoundException("Unexpected native import.");
        return Library.Value;
    }
    private static IntPtr Load(Assembly assembly)
    {
        string os = OperatingSystem.IsLinux() ? "linux" : OperatingSystem.IsMacOS() ? "osx" : OperatingSystem.IsWindows() ? "win" : throw new PlatformNotSupportedException();
        string arch = RuntimeInformation.ProcessArchitecture switch { Architecture.X64 => "x64", Architecture.Arm64 => "arm64", _ => throw new PlatformNotSupportedException() };
        if (os == "win" && arch != "x64") throw new PlatformNotSupportedException();
        string filename = os switch { "linux" => "libthinkthen.so", "osx" => "libthinkthen.dylib", _ => "thinkthen.dll" };
        string folder = Path.GetDirectoryName(assembly.Location) ?? throw new DllNotFoundException("Missing assembly location.");
        // NuGet either flattens the selected RID asset or retains its runtime directory.
        foreach (string path in new[] { Path.Combine(folder, filename), Path.Combine(folder, "runtimes", os + "-" + arch, "native", filename) })
            // Windows resolves thinkthen.dll to the managed ThinkThen.dll beside the consumer.
            if ((os != "win" || !string.Equals(path, assembly.Location, StringComparison.OrdinalIgnoreCase)) && File.Exists(path))
            {
                IntPtr library = NativeLibrary.Load(Path.GetFullPath(path));
                try
                {
                    if (!NativeLibrary.TryGetExport(library, "thinkthen_version", out IntPtr symbol))
                        throw new DllNotFoundException("ThinkThen native library is missing thinkthen_version.");
                    using var reader = new StreamReader(assembly.GetManifestResourceStream("ThinkThen.Interface.h")
                        ?? throw new DllNotFoundException("Missing ThinkThen C interface header."));
                    const string prefix = "#define THINKTHEN_INTERFACE_NUMBER ";
                    uint? required = null;
                    while (reader.ReadLine() is string line)
                        if (line.StartsWith(prefix, StringComparison.Ordinal))
                            required = uint.Parse(line[prefix.Length..].TrimEnd('u'), System.Globalization.CultureInfo.InvariantCulture);
                    if (required is null) throw new DllNotFoundException("Missing ThinkThen C interface digest.");
                    string? version = Marshal.PtrToStringUTF8(Marshal.GetDelegateForFunctionPointer<Version>(symbol)(out uint loaded));
                    if (loaded != required)
                        throw new DllNotFoundException($"ThinkThen C interface mismatch: header requires {required}, loaded library {version} provides {loaded}");
                    return library;
                }
                catch { NativeLibrary.Free(library); throw; }
            }
        throw new DllNotFoundException("The installed ThinkThen package has no native asset for " + os + "-" + arch + ".");
    }
}
