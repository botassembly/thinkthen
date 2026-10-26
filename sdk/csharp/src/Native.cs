using System.Runtime.InteropServices;
using Microsoft.Win32.SafeHandles;

[assembly: System.Runtime.CompilerServices.DisableRuntimeMarshalling]
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

internal sealed class QuestionHandle : SafeHandleZeroOrMinusOneIsInvalid
{
    internal QuestionHandle(nint value) : base(true) => SetHandle(value);
    protected override bool ReleaseHandle() { Native.QuestionFree(handle); return true; }
}
internal sealed class ToolboxHandle : SafeHandleZeroOrMinusOneIsInvalid
{
    internal ToolboxHandle(nint value) : base(true) => SetHandle(value);
    protected override bool ReleaseHandle() { Native.ToolboxFree(handle); return true; }
}
[StructLayout(LayoutKind.Sequential)]
internal struct NativeText { internal nint Data; internal nuint Length; }
[StructLayout(LayoutKind.Sequential)]
internal struct Buffer { internal nint Data; internal nuint Length; }
[StructLayout(LayoutKind.Sequential)]
internal struct NativeError
{
    internal uint StructSize;
    internal int Code;
    internal ushort HttpStatus;
    internal Buffer Message;
    internal static NativeError Empty => new() { StructSize = (uint)Marshal.SizeOf<NativeError>() };
}
[StructLayout(LayoutKind.Sequential)]
internal struct NativeRow
{
    internal uint StructSize, Function, Status;
    internal ulong Position;
    internal double Probability;
    internal int ErrorCode;
    internal byte AnswerBool;
    internal ulong Rank;
    internal double Number;
    internal Buffer Id, Answer, Detail, Source, Error;
    internal static NativeRow Empty => new() { StructSize = (uint)Marshal.SizeOf<NativeRow>() };
}

internal static partial class Native
{
    private const string Library = "thinkthen_ffi";
    private static readonly Lazy<nint> Loaded = new(Load);

    static Native() => NativeLibrary.SetDllImportResolver(typeof(Native).Assembly,
        (name, _, _) => name == Library ? Loaded.Value : nint.Zero);

    private static unsafe nint Load()
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
        if (!File.Exists(path) && File.Exists(Path.Combine(root, filename))) path = Path.Combine(root, filename);
        nint library = NativeLibrary.Load(Path.GetFullPath(path));
        try
        {
            var version = (delegate* unmanaged[Cdecl]<uint>)NativeLibrary.GetExport(library, "thinkthen_abi_version");
            var compatible = (delegate* unmanaged[Cdecl]<uint, uint, byte>)NativeLibrary.GetExport(library, "thinkthen_abi_compatible");
            if (compatible(Contract.AbiVersion, version()) == 0) throw new EntryPointNotFoundException();
            foreach (string export in Contract.Exports) NativeLibrary.GetExport(library, export);
            return library;
        }
        catch (EntryPointNotFoundException)
        {
            NativeLibrary.Free(library);
            throw new ThinkThenException(new(ErrorCode.UsageAbi, Contract.AbiMessage));
        }
    }

    internal static byte[] Copy(Buffer value)
    {
        byte[] bytes = new byte[checked((int)value.Length)];
        if (bytes.Length != 0) Marshal.Copy(value.Data, bytes, 0, bytes.Length);
        return bytes;
    }

    [System.Runtime.InteropServices.UnmanagedCallConv(CallConvs = new[] { typeof(System.Runtime.CompilerServices.CallConvCdecl) })]
    [LibraryImport(Library, EntryPoint = "thinkthen_settings_conversion_error")]
    internal static partial int SettingsConversionError(ref NativeError error);
    [System.Runtime.InteropServices.UnmanagedCallConv(CallConvs = new[] { typeof(System.Runtime.CompilerServices.CallConvCdecl) })]
    [LibraryImport(Library, EntryPoint = "thinkthen_question_conversion_error")]
    internal static partial int QuestionConversionError(ref NativeError error);
    [System.Runtime.InteropServices.UnmanagedCallConv(CallConvs = new[] { typeof(System.Runtime.CompilerServices.CallConvCdecl) })]
    [LibraryImport(Library, EntryPoint = "thinkthen_client_open_json")]
    internal static partial int ClientOpen([In, MarshalAs(UnmanagedType.LPArray, SizeParamIndex = 1)] byte[] settings, nuint length, out nint client, ref NativeError error);
    [System.Runtime.InteropServices.UnmanagedCallConv(CallConvs = new[] { typeof(System.Runtime.CompilerServices.CallConvCdecl) })]
    [LibraryImport(Library, EntryPoint = "thinkthen_client_free")]
    internal static partial void ClientFree(nint client);
    [System.Runtime.InteropServices.UnmanagedCallConv(CallConvs = new[] { typeof(System.Runtime.CompilerServices.CallConvCdecl) })]
    [LibraryImport(Library, EntryPoint = "thinkthen_call_open_question")]
    internal static partial int Open(ClientHandle client, QuestionHandle question, nint name, uint offsets, out nint call, ref NativeError error);
    [System.Runtime.InteropServices.UnmanagedCallConv(CallConvs = new[] { typeof(System.Runtime.CompilerServices.CallConvCdecl) })]
    [LibraryImport(Library, EntryPoint = "thinkthen_call_send_utf16")]
    internal static partial int SendText(CallHandle call, [In, MarshalAs(UnmanagedType.LPArray, SizeParamIndex = 2)] char[] text, nuint length, ref NativeError error);
    [System.Runtime.InteropServices.UnmanagedCallConv(CallConvs = new[] { typeof(System.Runtime.CompilerServices.CallConvCdecl) })]
    [LibraryImport(Library, EntryPoint = "thinkthen_call_send_json")]
    internal static partial int SendJson(CallHandle call, [In, MarshalAs(UnmanagedType.LPArray, SizeParamIndex = 2)] byte[] json, nuint length, ref NativeError error);
    [System.Runtime.InteropServices.UnmanagedCallConv(CallConvs = new[] { typeof(System.Runtime.CompilerServices.CallConvCdecl) })]
    [LibraryImport(Library, EntryPoint = "thinkthen_call_end_input")]
    internal static partial void EndInput(CallHandle call);
    [System.Runtime.InteropServices.UnmanagedCallConv(CallConvs = new[] { typeof(System.Runtime.CompilerServices.CallConvCdecl) })]
    [LibraryImport(Library, EntryPoint = "thinkthen_call_next")]
    internal static partial int Next(CallHandle call, ref NativeRow row, ref NativeError error);
    [System.Runtime.InteropServices.UnmanagedCallConv(CallConvs = new[] { typeof(System.Runtime.CompilerServices.CallConvCdecl) })]
    [LibraryImport(Library, EntryPoint = "thinkthen_call_cancel")]
    internal static partial void Cancel(CallHandle call);
    [System.Runtime.InteropServices.UnmanagedCallConv(CallConvs = new[] { typeof(System.Runtime.CompilerServices.CallConvCdecl) })]
    [LibraryImport(Library, EntryPoint = "thinkthen_call_free")]
    internal static partial void Free(nint call);
    [System.Runtime.InteropServices.UnmanagedCallConv(CallConvs = new[] { typeof(System.Runtime.CompilerServices.CallConvCdecl) })]
    [LibraryImport(Library, EntryPoint = "thinkthen_error_free")]
    internal static partial void ErrorFree(ref NativeError error);
    [System.Runtime.InteropServices.UnmanagedCallConv(CallConvs = new[] { typeof(System.Runtime.CompilerServices.CallConvCdecl) })]
    [LibraryImport(Library, EntryPoint = "thinkthen_code_name")]
    internal static partial nint CodeName(int code);
    [System.Runtime.InteropServices.UnmanagedCallConv(CallConvs = new[] { typeof(System.Runtime.CompilerServices.CallConvCdecl) })]
    [LibraryImport(Library, EntryPoint = "thinkthen_call_send_missing")]
    internal static partial int SendMissing(CallHandle call, ref NativeError error);
    [System.Runtime.InteropServices.UnmanagedCallConv(CallConvs = new[] { typeof(System.Runtime.CompilerServices.CallConvCdecl) })]
    [LibraryImport(Library, EntryPoint = "thinkthen_row_free")]
    internal static partial void RowFree(ref NativeRow row);
    [System.Runtime.InteropServices.UnmanagedCallConv(CallConvs = new[] { typeof(System.Runtime.CompilerServices.CallConvCdecl) })]
    [LibraryImport(Library, EntryPoint = "thinkthen_question_from_json")]
    internal static partial int QuestionFromJson([In, MarshalAs(UnmanagedType.LPArray, SizeParamIndex = 1)] byte[] data, nuint length, out nint question, ref NativeError error);
    [System.Runtime.InteropServices.UnmanagedCallConv(CallConvs = new[] { typeof(System.Runtime.CompilerServices.CallConvCdecl) })]
    [LibraryImport(Library, EntryPoint = "thinkthen_question_free")]
    internal static partial void QuestionFree(nint question);
    [System.Runtime.InteropServices.UnmanagedCallConv(CallConvs = new[] { typeof(System.Runtime.CompilerServices.CallConvCdecl) })]
    [LibraryImport(Library, EntryPoint = "thinkthen_toolbox_from_json")]
    internal static partial int ToolboxFromJson([In, MarshalAs(UnmanagedType.LPArray, SizeParamIndex = 1)] byte[] data, nuint length, out nint toolbox, ref NativeError error);
    [System.Runtime.InteropServices.UnmanagedCallConv(CallConvs = new[] { typeof(System.Runtime.CompilerServices.CallConvCdecl) })]
    [LibraryImport(Library, EntryPoint = "thinkthen_toolbox_from_definitions_json")]
    internal static partial int ToolboxFromDefinitionsJson([In, MarshalAs(UnmanagedType.LPArray, SizeParamIndex = 1)] byte[] data, nuint length, out nint toolbox, ref NativeError error);

    [LibraryImport(Library, EntryPoint = "thinkthen_toolbox_to_json")]
    internal static partial int ToolboxToJson(ToolboxHandle toolbox, out Buffer output, ref NativeError error);

    [LibraryImport(Library, EntryPoint = "thinkthen_toolbox_definitions_json")]
    internal static partial int ToolboxDefinitionsJson(ToolboxHandle toolbox, out Buffer output, ref NativeError error);

    [LibraryImport(Library, EntryPoint = "thinkthen_buffer_free")]
    internal static partial void BufferFree(ref Buffer buffer);

    [LibraryImport(Library, EntryPoint = "thinkthen_toolbox_free")]
    internal static partial void ToolboxFree(nint toolbox);
    [System.Runtime.InteropServices.UnmanagedCallConv(CallConvs = new[] { typeof(System.Runtime.CompilerServices.CallConvCdecl) })]
    [LibraryImport(Library, EntryPoint = "thinkthen_call_send_invalid")]
    internal static partial int SendInvalid(CallHandle call, ref NativeError error);
    [UnmanagedCallConv(CallConvs = new[] { typeof(System.Runtime.CompilerServices.CallConvCdecl) })]
    [LibraryImport(Library, EntryPoint = "thinkthen_question_new")]
    internal static partial int QuestionNew(uint function, [In, MarshalAs(UnmanagedType.LPArray, SizeParamIndex = 2)] byte[] text, nuint length, out nint question, ref NativeError error);
    [UnmanagedCallConv(CallConvs = new[] { typeof(System.Runtime.CompilerServices.CallConvCdecl) })]
    [LibraryImport(Library, EntryPoint = "thinkthen_question_set_cut")]
    internal static partial int SetCut(nint question, double value, ref NativeError error);
    [UnmanagedCallConv(CallConvs = new[] { typeof(System.Runtime.CompilerServices.CallConvCdecl) })]
    [LibraryImport(Library, EntryPoint = "thinkthen_question_set_band")]
    internal static partial int SetBand(nint question, double low, double high, ref NativeError error);
    [UnmanagedCallConv(CallConvs = new[] { typeof(System.Runtime.CompilerServices.CallConvCdecl) })]
    [LibraryImport(Library, EntryPoint = "thinkthen_question_set_evidence")]
    internal static partial int SetEvidence(nint question, [In, MarshalAs(UnmanagedType.LPArray, SizeParamIndex = 2)] byte[] data, nuint length, ref NativeError error);
    [UnmanagedCallConv(CallConvs = new[] { typeof(System.Runtime.CompilerServices.CallConvCdecl) })]
    [LibraryImport(Library, EntryPoint = "thinkthen_toolbox_select_as")]
    internal static partial int ToolboxSelectAs(ToolboxHandle toolbox, [In, MarshalAs(UnmanagedType.LPArray, SizeParamIndex = 2)] byte[] data, nuint length, uint function, out nint question, ref NativeError error);
    [UnmanagedCallConv(CallConvs = new[] { typeof(System.Runtime.CompilerServices.CallConvCdecl) })]
    [LibraryImport(Library, EntryPoint = "thinkthen_settings_from_json")]
    internal static partial int SettingsFromJson([In, MarshalAs(UnmanagedType.LPArray, SizeParamIndex = 1)] byte[] data, nuint length, out nint settings, ref NativeError error);
    [UnmanagedCallConv(CallConvs = new[] { typeof(System.Runtime.CompilerServices.CallConvCdecl) })]
    [LibraryImport(Library, EntryPoint = "thinkthen_settings_free")]
    internal static partial void SettingsFree(nint settings);
}
