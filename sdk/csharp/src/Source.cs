namespace ThinkThen;

// An uncooperative producer may outlive a cancelled call, but never owns it.
internal sealed class Source(IAsyncEnumerable<Record> records, CancellationToken token)
{
    private readonly IAsyncEnumerator<Record> iterator = records.GetAsyncEnumerator(token);
    private Task<bool>? pending;
    internal Record Current => iterator.Current;
    internal async Task<bool> Advance()
    {
        pending = iterator.MoveNextAsync().AsTask();
        bool result = await pending.WaitAsync(token).ConfigureAwait(false);
        pending = null;
        return result;
    }
    internal async Task Finish(bool suppressFailure)
    {
        if (pending is not null)
        {
            _ = FinishPending(pending);
            return;
        }
        Task? disposal = null;
        try
        {
            disposal = iterator.DisposeAsync().AsTask();
            await disposal.WaitAsync(token).ConfigureAwait(false);
        }
        catch
        {
            if (disposal is not null) _ = Observe(disposal);
            if (!suppressFailure) throw;
        }
    }
    private async Task FinishPending(Task<bool> advance)
    {
        try { await advance.ConfigureAwait(false); }
        catch { /* Preserve the call's earlier failure. */ }
        try { await iterator.DisposeAsync().ConfigureAwait(false); }
        catch { /* Preserve the call's earlier failure. */ }
    }
    private static async Task Observe(Task task)
    {
        try { await task.ConfigureAwait(false); }
        catch { /* Preserve the call's earlier failure. */ }
    }
}
