// Generated from the compiler-derived C header ABI; do not edit.
using System.Runtime.InteropServices;
namespace ThinkThen;
[StructLayout(LayoutKind.Sequential)]
internal struct StringV1 {
    public IntPtr data;
    public nuint len;
}
[StructLayout(LayoutKind.Sequential)]
internal struct CompleteUsagePersistenceV1 {
    public uint kind;
}
[StructLayout(LayoutKind.Sequential)]
internal struct CompleteUtf8V1 {
    public IntPtr data;
    public nuint len;
}
[StructLayout(LayoutKind.Sequential)]
internal struct NamedRequestV1 {
    public CompleteUtf8V1 function;
    public CompleteUtf8V1 question_json;
    public CompleteUtf8V1 input_json;
    public CompleteUtf8V1 options_json;
}
public enum AuthoredQuestionKind : uint {
    Atomic = 1, DynamicChoose = 3, Find = 8, RankSet = 7, Rank = 6, Recognize = 4, Relate = 5,
    Set = 2
}
public enum FailureKind : int {
    Backend = 2, Cancelled = 5, Deadline = 3, Defect = 6, Local = 4, Usage = 1
}
public enum UsagePersistenceState : uint {
    Disabled = 1, Failed = 4, Pending = 2, Written = 3
}
internal static class NativeUsage {
    [DllImport("thinkthen", CallingConvention=CallingConvention.Cdecl)] internal static extern int thinkthen_engine_usage_persistence_v1(EngineHandle engine,
    out CompleteUsagePersistenceV1 output1, out CompleteUtf8V1 output2);
    [DllImport("thinkthen", CallingConvention=CallingConvention.Cdecl)] internal static extern int thinkthen_engine_finish_usage_status_v1(EngineHandle engine,
    out CompleteUsagePersistenceV1 output1, out CompleteUtf8V1 output2);
}
internal static class NativeNamed {
    [DllImport("thinkthen", CallingConvention=CallingConvention.Cdecl)] internal static extern int thinkthen_session_new_named_with_surface(EngineHandle engine,
    in NamedRequestV1 request, CompleteUtf8V1 surface, out IntPtr session);
}
internal static class NativeWait {
    internal const uint Input = 1;
    internal const uint Result = 0;
    internal const uint Settlement = 2;
    [DllImport("thinkthen", CallingConvention=CallingConvention.Cdecl)] internal static extern int thinkthen_session_wait(SessionHandle session,
    uint interest, uint timeoutMilliseconds, out uint ready);
    [DllImport("thinkthen", CallingConvention=CallingConvention.Cdecl)] internal static extern int thinkthen_session_next_packet(SessionHandle session,
    uint interest, out uint status, out IntPtr result);
    [DllImport("thinkthen", CallingConvention=CallingConvention.Cdecl)] internal static extern int thinkthen_session_push(SessionHandle session,
    byte[] descriptor, nuint length, out uint status);
    internal const uint ReadResult = 0;
    internal const uint ReadPending = 1;
    internal const uint ReadEnd = 2;
}
