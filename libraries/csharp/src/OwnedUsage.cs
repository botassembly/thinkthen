using System.Runtime.InteropServices;
namespace ThinkThen;
/// <summary>An owned live persistence observation, separate from call facts.</summary>
public sealed record UsagePersistenceStatus(UsagePersistenceState State, string? Advice);
public sealed partial class Engine
{
    /// <summary>Observe persistence without waiting for the writer or filesystem.</summary>
    public UsagePersistenceStatus UsagePersistence() => Persistence(false);
    /// <summary>Finish current deltas. Only usage-lock acquisition has a deadline; other filesystem work may take longer.</summary>
    public UsagePersistenceStatus FinishUsageStatus() => Persistence(true);
    private UsagePersistenceStatus Persistence(bool finish) => Live(_ =>
    {
        CompleteUsagePersistenceV1 state;
        CompleteUtf8V1 advice;
        int code = finish
            ? NativeUsage.thinkthen_engine_finish_usage_status_v1(ownedEngine, out state, out advice)
            : NativeUsage.thinkthen_engine_usage_persistence_v1(ownedEngine, out state, out advice);
        NativeSession.Check(code);
        var kind = (UsagePersistenceState)state.kind;
        if (!Enum.IsDefined(kind)) throw new InvalidOperationException("Invalid native persistence state.");
        string? text = advice.data == IntPtr.Zero ? null : Marshal.PtrToStringUTF8(advice.data, checked((int)advice.len));
        return new UsagePersistenceStatus(kind, text);
    });
}
