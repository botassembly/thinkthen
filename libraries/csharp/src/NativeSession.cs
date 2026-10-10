using System.Runtime.InteropServices;
namespace ThinkThen;
internal static class NativeSession
{
    private const string Library = "thinkthen";
    [DllImport(Library, CallingConvention=CallingConvention.Cdecl)] internal static extern int thinkthen_request_plan_json(EngineHandle engine, byte[] request, nuint length, out IntPtr output, out nuint outputLength);
    [DllImport(Library, CallingConvention=CallingConvention.Cdecl)] internal static extern int thinkthen_session_new(EngineHandle engine, byte[] request, nuint length, out IntPtr session);
    [DllImport(Library, CallingConvention=CallingConvention.Cdecl)] internal static extern int thinkthen_session_new_with_surface(EngineHandle engine, byte[] request, nuint length, byte[] surface, nuint surfaceLength, out IntPtr session);
    [DllImport(Library, CallingConvention=CallingConvention.Cdecl)] internal static extern int thinkthen_session_try_push(SessionHandle session, byte[] descriptor, nuint length, out uint status);
    [DllImport(Library, CallingConvention=CallingConvention.Cdecl)] internal static extern int thinkthen_session_try_read(SessionHandle session, out uint status, out IntPtr packet);
    [DllImport(Library, CallingConvention=CallingConvention.Cdecl)] internal static extern int thinkthen_session_finish(SessionHandle session, byte[]? failure, nuint length);
    [DllImport(Library, CallingConvention=CallingConvention.Cdecl)] internal static extern void thinkthen_session_cancel(SessionHandle session);
    [DllImport(Library, CallingConvention=CallingConvention.Cdecl)] internal static extern void thinkthen_session_free(IntPtr session);
    [DllImport(Library, CallingConvention=CallingConvention.Cdecl)] internal static extern void thinkthen_session_result_free(IntPtr packet);
    [DllImport(Library, CallingConvention=CallingConvention.Cdecl)] internal static extern int thinkthen_session_result_json(PacketHandle packet, out IntPtr json, out nuint length);
    [DllImport(Library, CallingConvention=CallingConvention.Cdecl)] internal static extern IntPtr thinkthen_session_error_message();
    internal static void Check(int code) { if (code != 0) throw new Failure(code, false, Marshal.PtrToStringUTF8(thinkthen_session_error_message()) ?? "Native session failure.", null); }
}
