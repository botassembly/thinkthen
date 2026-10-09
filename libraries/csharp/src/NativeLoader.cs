using System.Reflection;
using System.Runtime.InteropServices;
namespace ThinkThen;
internal static class NativeLoader
{
    private static readonly object gate = new();
    private static bool initialized;
    internal static void Initialize()
    {
        lock (gate)
        {
            if (initialized) return;
            NativeLibrary.SetDllImportResolver(typeof(Engine).Assembly, Resolve);
            initialized = true;
        }
    }
    private static IntPtr Resolve(string name, Assembly assembly, DllImportSearchPath? search)
    {
        if (name != "thinkthen" && name != "libthinkthen.so.0")
            throw new DllNotFoundException("Unexpected native import.");
        string os = OperatingSystem.IsLinux() ? "linux" : OperatingSystem.IsMacOS() ? "osx" : OperatingSystem.IsWindows() ? "win" : throw new PlatformNotSupportedException();
        string arch = RuntimeInformation.ProcessArchitecture switch { Architecture.X64 => "x64", Architecture.Arm64 => "arm64", _ => throw new PlatformNotSupportedException() };
        if (os == "win" && arch != "x64") throw new PlatformNotSupportedException();
        string filename = os switch { "linux" => "libthinkthen.so", "osx" => "libthinkthen.dylib", _ => "thinkthen.dll" };
        string folder = Path.GetDirectoryName(assembly.Location) ?? throw new DllNotFoundException("Missing assembly location.");
        // NuGet either flattens the selected RID asset or retains its runtime directory.
        foreach (string path in new[] { Path.Combine(folder, filename), Path.Combine(folder, "runtimes", os + "-" + arch, "native", filename) })
            if (File.Exists(path)) return NativeLibrary.Load(Path.GetFullPath(path));
        throw new DllNotFoundException("The installed ThinkThen package has no native asset for " + os + "-" + arch + ".");
    }
}
