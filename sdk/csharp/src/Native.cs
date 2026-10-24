using System.Runtime.InteropServices;
using Microsoft.Win32.SafeHandles;

namespace ThinkThen;

internal sealed class ClientHandle : SafeHandleZeroOrMinusOneIsInvalid
{
    internal ClientHandle(nint value) : base(true) => SetHandle(value);
    protected override bool ReleaseHandle() { Native.ClientFree(handle); return true; }
}
internal sealed class CallHandle : SafeHandleZeroOrMinusOneIsInvalid
{
    internal CallHandle(nint value) : base(true) => SetHandle(value);
    protected override bool ReleaseHandle() { Native.Free(handle); return true; }
}

[StructLayout(LayoutKind.Sequential)]
internal struct Buffer { internal nint Data; internal nuint Length; }
[StructLayout(LayoutKind.Sequential)]
internal struct NativeRow
{
    internal Buffer Fields;
    internal Buffer Detail;
    internal nuint Position;
    internal byte Kept;
}

internal static class Native
{
    private const string Library = "thinkthen_ffi";
    private static readonly Lazy<nint> Loaded = new(Load);

    static Native() => NativeLibrary.SetDllImportResolver(typeof(Native).Assembly,
        (name, _, _) => name == Library ? Loaded.Value : nint.Zero);

    private static nint Load()
    {
        string rid;
        string filename;
        if (OperatingSystem.IsLinux() && (RuntimeInformation.ProcessArchitecture is Architecture.X64 or Architecture.Arm64)
            && !RuntimeInformation.RuntimeIdentifier.Contains("musl", StringComparison.Ordinal))
        { rid = RuntimeInformation.ProcessArchitecture == Architecture.Arm64 ? "linux-arm64" : "linux-x64"; filename = "libthinkthen_ffi.so"; }
        else if (OperatingSystem.IsMacOS() && RuntimeInformation.ProcessArchitecture == Architecture.Arm64)
        { rid = "osx-arm64"; filename = "libthinkthen_ffi.dylib"; }
        else if (OperatingSystem.IsWindows() && RuntimeInformation.ProcessArchitecture == Architecture.X64)
        { rid = "win-x64"; filename = "thinkthen_ffi.dll"; }
        else throw new DllNotFoundException("The ThinkThen package has no native asset for this runtime.");
        string root = AppContext.BaseDirectory;
        string path = Path.Combine(root, "runtimes", rid, "native", filename);
        if (!File.Exists(path)) path = Path.Combine(root, filename);
        return NativeLibrary.Load(Path.GetFullPath(path));
    }

    internal static byte[] Copy(Buffer value)
    {
        byte[] bytes = new byte[checked((int)value.Length)];
        if (bytes.Length != 0) Marshal.Copy(value.Data, bytes, 0, bytes.Length);
        return bytes;
    }

    [DllImport(Library, EntryPoint = "thinkthen_client_open", CallingConvention = CallingConvention.Cdecl)]
    internal static extern int ClientOpen(byte[] settings, nuint length, out nint client, out Buffer error);
    [DllImport(Library, EntryPoint = "thinkthen_client_free", CallingConvention = CallingConvention.Cdecl)]
    internal static extern void ClientFree(nint client);
    [DllImport(Library, EntryPoint = "thinkthen_call_open", CallingConvention = CallingConvention.Cdecl)]
    internal static extern int Open(ClientHandle client, byte[] request, nuint length, uint offsets, out nint call, out Buffer error);
    [DllImport(Library, EntryPoint = "thinkthen_send_utf16", CallingConvention = CallingConvention.Cdecl, CharSet = CharSet.Unicode)]
    internal static extern int SendText(CallHandle call, [In] char[] text, nuint length, out Buffer error);
    [DllImport(Library, EntryPoint = "thinkthen_send_json", CallingConvention = CallingConvention.Cdecl)]
    internal static extern int SendJson(CallHandle call, byte[] json, nuint length, out Buffer error);
    [DllImport(Library, EntryPoint = "thinkthen_end_input", CallingConvention = CallingConvention.Cdecl)]
    internal static extern void EndInput(CallHandle call);
    [DllImport(Library, EntryPoint = "thinkthen_next", CallingConvention = CallingConvention.Cdecl)]
    internal static extern int Next(CallHandle call, out NativeRow row, out Buffer error);
    [DllImport(Library, EntryPoint = "thinkthen_cancel", CallingConvention = CallingConvention.Cdecl)]
    internal static extern void Cancel(CallHandle call);
    [DllImport(Library, EntryPoint = "thinkthen_free", CallingConvention = CallingConvention.Cdecl)]
    internal static extern void Free(nint call);
    [DllImport(Library, EntryPoint = "thinkthen_buffer_free", CallingConvention = CallingConvention.Cdecl)]
    internal static extern void BufferFree(ref Buffer buffer);
    [DllImport(Library, EntryPoint = "thinkthen_row_free", CallingConvention = CallingConvention.Cdecl)]
    internal static extern void RowFree(ref NativeRow row);
}
