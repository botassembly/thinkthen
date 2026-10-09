// Generated from the compiler-derived C header ABI; do not edit.
using System.Runtime.InteropServices;
namespace ThinkThen;
[StructLayout(LayoutKind.Sequential)]
internal struct StringV1 {
 public IntPtr data;
 public nuint len;
}
public enum AuthoredQuestionKind : uint { Atomic = 1, DynamicChoose = 3, Find = 8, RankSet = 7, Rank = 6, Recognize = 4, Relate = 5, Set = 2 }
public enum FailureKind : int { Backend = 2, Cancelled = 5, Deadline = 3, Defect = 6, Local = 4, Usage = 1 }
