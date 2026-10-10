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
public enum AuthoredQuestionKind : uint { Atomic = 1, DynamicChoose = 3, Find = 8, RankSet = 7, Rank = 6, Recognize = 4, Relate = 5, Set = 2 }
public enum FailureKind : int { Backend = 2, Cancelled = 5, Deadline = 3, Defect = 6, Local = 4, Usage = 1 }
public enum UsagePersistenceState : uint { Disabled = 1, Failed = 4, Pending = 2, Written = 3 }
internal static class NativeUsage {
 [DllImport("thinkthen", CallingConvention=CallingConvention.Cdecl)] internal static extern int thinkthen_engine_usage_persistence_v1(EngineHandle engine, out CompleteUsagePersistenceV1 output1, out CompleteUtf8V1 output2);
 [DllImport("thinkthen", CallingConvention=CallingConvention.Cdecl)] internal static extern int thinkthen_engine_finish_usage_status_v1(EngineHandle engine, out CompleteUsagePersistenceV1 output1, out CompleteUtf8V1 output2);
}
