using Microsoft.Win32.SafeHandles;
namespace ThinkThen;
internal sealed class EngineHandle : SafeHandleZeroOrMinusOneIsInvalid
{
    internal EngineHandle(IntPtr pointer) : base(true) => SetHandle(pointer);
    protected override bool ReleaseHandle() { Native.thinkthen_engine_free(handle); return true; }
}
internal sealed class SessionHandle : SafeHandleZeroOrMinusOneIsInvalid
{
    internal SessionHandle(IntPtr pointer) : base(true) => SetHandle(pointer);
    protected override bool ReleaseHandle() { NativeSession.thinkthen_session_free(handle); return true; }
}
internal sealed class PacketHandle : SafeHandleZeroOrMinusOneIsInvalid
{
    internal PacketHandle(IntPtr pointer) : base(true) => SetHandle(pointer);
    protected override bool ReleaseHandle() { NativeSession.thinkthen_session_result_free(handle); return true; }
}
internal sealed class QuestionHandle : SafeHandleZeroOrMinusOneIsInvalid
{
    internal QuestionHandle(IntPtr pointer) : base(true) => SetHandle(pointer);
    protected override bool ReleaseHandle() { Native.thinkthen_question_free(handle); return true; }
}
